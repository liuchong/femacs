# 整体 threaded/asm 派发可行性裁定（2026-10-21）

## 问题与边界

用户授权评估“整体 threaded/asm 派发重写”，要求保持算法、Elisp 语义、求值顺序、错误与 GC
行为完全不变；若不可行，则按功能性口径收尾，性能未达如实保留。本文件基于当前源码判断，
不以“再试一次”为目标。

## 参照与端口的事实

- 参照 `emacs/src/bytecode.c` 41-44 行无条件 `#define BYTE_CODE_THREADED`；573-607 行的
  `CASE`/`NEXT`/`FIRST` 定义为：
  - `#define CASE(OP) insn_ ## OP`
  - `#define NEXT goto *(targets[op = FETCH])`
  - `#define FIRST NEXT;`
  - 256 项 `static const void *const targets[256]`，每项是 `&&insn_*` 标签地址。
- C 的 opcode 体不是独立函数：167 个 `CASE (...)` 共享同一个函数栈帧里的
  `top`、`pc`、`arg`、`vectorp`、`bc->fp`、`bytestr_length` 等局部；另有 8 个共享尾标签
  （`varref`、`varset`、`varbind`、`docall`、`dounbind`、`exit`、`op_branch`、
  `pushhandler`），由多个 opcode `goto` 进入。
- 端口 `femacs/src/bytecode.rs` 648-651 行按 P-03 明确选择 switch 臂：
  `macro_rules! NEXT { () => { continue } }`、`FIRST { match ... }`；每个 opcode 体仍是
  同一 Rust 函数内的语句，与 C 的 switch 臂逐句对应。

## 为什么不能只替换派发

Rust 没有“取标签地址”的稳定或可用机制。`asm_goto` 的 label 操作数只能作为编译期跳转目标，
不能组成一张按 opcode 索引的地址表；`asm!` 可以 `br` 到寄存器，但无法取得 Rust 标签的地址。
因此，真正的 computed goto 只有三种实现：

1. **把 167 个 opcode 体全部写成 asm 标签体**：这不是“只改派发”，而是重写整个
   `exec_byte_code`；每个体的 GC 根、`sys_setjmp`/`longjmp`、`Fsignal`、`record_in_backtrace`
   调用都必须重新证明，超出“保持逻辑不变”的可验证边界。
2. **每个 opcode 一个 naked 函数，函数尾 `br` 到下一个函数地址**：真实执行变成
   “间接调用 + 返回 + 间接跳转”，比 switch 多一次控制转移；GC/栈回溯还要跨 naked 帧。
3. **asm 只做跳板、opcode 体仍调用 Rust 助手**：每个 opcode 至少一次 `bl`/`ret`，调用边界
   会把派发节省全部吃掉；S2 计划中的“Rust 助手重装状态”正是这一形态。

## 状态与 ABI 边界

任何 asm 版本都必须持有 `top`、`pc`、`frame_base`、`fp`、`bc`、`vectorp`、`max_stack`、
`arg`、`goto` 这些活值。它们中既有 Lisp_Object 根，也有 `bc_frame` 指针；在调用任何可能
GC 的 Rust 助手前必须写回可被保守栈扫描看到的内存槽。`sys_setjmp`/`sys_longjmp` 的退出路径
会跨过 asm 帧，macOS arm64 与 Android arm64 的栈展开/信号实现不同，必须分别验证。仅这一条
就把“纯派发替换”变成“跨平台 asm 解释器 + 异常/GC 证明”。

## 现有失败是否足以否定整体方案

计划（`docs/bytecode-threaded-plan.md`）的 S1 要求 asm/switch ≤ 0.85×；实测一个真实的
aarch64 256 项 asm 偏移表（inc/halt 两臂）为 **3.738 ms vs switch 3.086 ms = 1.21×**。
S1 只测派发核心，不测复杂体，因此它是**上限**而不是下限：

- 若连只有两条极简臂的 asm 表都比 switch 慢 21%，加入 167 个真实体、状态写回和 Rust 助手
  调用只会增加开销；
- `become` + 函数指针表已实测 **5.46× 慢**，最小 asm 链 **1.25× 慢**；
- 参照自己从 threaded 改成 switch 后慢约 2×，说明 switch 的额外开销主要来自“每指令一次
  额外分支/循环”，而不是 opcode 体；但改写成 asm 解释器无法只去掉这一分支而不重写体。

所以，toy 不需要覆盖全部 opcode 才能否定**当前可落地的设计**：它已经测掉了唯一可行的
派发核心，并显示目标阈值（≤0.85×）不可能达到。剩下的“全 asm 解释器”不是本轮授权的
“派发重写”，而是另一个高风险工程。

## 平台差异

- macOS arm64 与 Android arm64 都使用 aarch64 指令集，但 ABI/栈展开/信号与 `longjmp`
  实现不同；asm 需要两套验证，不能以 macOS 的读数代替 Android。
- 端口的 Android 目标只验收模拟器；asm 解释器若在 bionic 上崩溃，修复成本远高于 switch。

## 裁定

**整体 threaded/asm 派发重写在“逻辑/语义/错误/GC 完全不变”的约束下不可行**：
Rust 无法取得标签地址；唯一真正的 computed goto 必须把 167 个 opcode 体与 8 个共享尾全部
搬进 asm，已超出派发形状，且 S1 已证明派发核心本身更慢。按用户指示，**性能路线到此收尾**：
保留 O2 默认与 PGO 候选，DoD-7 记未达，不再列新的性能菜单或继续试跑。功能性验收以
`verify-dod --full`、L0/L1/L5、GC 压测的现有证据为准。
