# `exec_byte_code` threaded-dispatch spike plan (2026-10-21, status 4.1054)

## 背景与判据

参照在 macOS 的 live 配置里走 `BYTE_CODE_THREADED`（`bytecode.c` 41-44、581 起的
`goto *targets[op = FETCH]`）；端口按 P-03 移植的是同一文件的 switch 臂。P-193 已经量出：

- 参照 threaded 的 `bc-loop` 3.20–3.34 ms；把参照自己编成 switch 后 6.80–7.08 ms；
- 端口 switch 4.95–5.45 ms ⇒ **端口比参照 switch 快（0.70–0.79×）**，但比参照 threaded
  仍慢 1.25–1.5×。

4.1051–4.1053 又排除了三条小路径：`bytecode`/`sort` 单独 O3 无收益；Rust `become` +
函数指针表比 switch 慢 5.46×；最小 `asm_goto` 链比 switch 慢 1.25×。因此剩余方案只有一个：
**在 aarch64 上用一张真正的 256 项 asm 跳转表复刻 C 的 computed-goto 派发**，语义不变，
switch 版本保留为 fallback。

## 约束

1. **只改派发形状，不改 `exec_byte_code` 的栈机、错误模型或求值顺序**。每个 opcode 的
   Rust 语句照旧，asm 只负责 `op -> handler` 的间接跳转。
2. `panic = "abort"`；不跨 asm 帧引入 `Drop`；`sys_setjmp`/`sys_longjmp` 的退出路径必须
   与 switch 版本一致。
3. GC 保守栈扫描仍要看到 `top`/`pc`/`fp` 等活跃槽位；asm 寄存器里的 Lisp_Object
   必须在调用任何可能 GC 的 Rust 助手前落回可扫描的栈槽。
4. macOS arm64 与 Android arm64 都只走 aarch64；其他目标继续 switch。
5. 构建可逆：新增独立 `bytecode_threaded` 配置/开关；失败时默认构建不受影响。

## 设计草案

- **状态指针**：进入 asm 前把 `top`、`pc`、`frame_base`、`bc`、`vectorp` 等放进一个
  `#[repr(C)]` 的 `bc_asm_state`，用 callee-saved 寄存器（x19–x28）持有状态指针和跳转表基址。
- **跳转表**：`global_asm!` 里定义 256 个本地标签和一张 `adr`/`.quad` 表；入口
  `br table[op]`。opcode 值来自现有 `FETCH`，表索引与 switch 的 `case` 常量一一对应。
- **助手边界**：简单 opcode 在 asm 内完成（push/pop/算术/分支）；复杂 opcode 通过
  `extern "C"` 调用现有 Rust 函数（`Fsymbol_value`、`Ffuncall`、`set_internal` 等），
  调用前把状态写回 `bc_asm_state`，调用后重新装载；所有 Rust 助手沿用现有签名。
- **共享尾**：`varref`/`varset`/`varbind`/`docall`/`dounbind`/`exit`/`op_branch`/
  `pushhandler` 仍复用现有 Rust 代码；asm 只跳到相应入口。
- **异常/退出**：`sys_longjmp` 或 `Fsignal` 路径不在 asm 内展开；asm 直接 `bl` 到现有
  Rust 包装，由它走原来的退出逻辑。
- **fallback**：`FEMACS_BYTECODE_THREADED=1` 时才链接 asm 版本；默认仍是已验证的 switch。

## 分阶段 spike 与放弃条件

1. **S1（4 opcode）**：`Bstack_ref`、`Bconstant`、`Bgoto`、`Breturn` 四个 opcode +
   256 项 asm 跳转表；用 8,000,000 次简单程序对比 switch。要求 **asm/switch ≤ 0.85×**，
   否则放弃。
2. **S2（16 opcode）**：加入 push/pop/算术/条件分支和一次 Rust 助手调用；比较
   调用边界开销，要求仍 ≤ 0.90×，且 `smoke` 11/0。
3. **S3（全 256 臂）**：只在 S1/S2 达标后开始；以现有 `bytecode.rs` 的 switch 臂为
   逐条对照，保持语句顺序和错误路径。
4. **验收**：`gate-fast`、`gate-load`、`dump`、`smoke`、`tui-smoke`、`xdisp/faces/xfaces`
   定向 ERT、`process-tests`、60 程序 fuzz；`bc-loop/bc-sum/bc-car` 必须进入参照
   threaded 的噪声区间，否则不宣称 DoD-7 达成。
5. **回退**：任一阶段失败，删除 asm 配置与表，保留 switch；按 P-118 回退后四门复绿。

## 风险

- asm 标签表与 Rust 助手之间的 ABI/寄存器保存错误会导致难定位的崩溃；
- GC 扫描看不到只存在于 asm 寄存器中的对象；
- `longjmp` 跨 asm 帧、调试信息、Android/macOS 栈展开差异；
- 256 臂维护成本高，且每次 opcode 改动都要同步表与 switch 臂；
- 如果 S1 的 4-opcode 结果不能明显优于 switch，整条路径应放弃，而不是继续堆工程量。
