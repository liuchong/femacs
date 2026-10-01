# A6 试跑 #1：`src/sort.c` → `femacs/src/sort.rs`

- 参照：`emacs/src/sort.c` @ `fd7afced403`（1,178 行）
- 产物：`femacs/src/sort.rs`（1,223 行，漂移 3.8%，闸门上限 5%）
- 台账：`docs/ports/sort.md`（清单全部勾选 + Decisions + D7）

## 证据

| 检查 | 命令 | 结果 |
| --- | --- | --- |
| 编译 | `cd femacs && cargo run -q -p xtask -- build-tu src/sort.c` | `built src/sort.rs -> ../build/femacs/sort.o` |
| 清单 | `python3 tools/extract-abi.py --check emacs/src/sort.c femacs/src/sort.rs` | `OK`（0 差异） |
| 快闸门 | `cd femacs && cargo run -q -p xtask -- gate-fast` | 清单 `no differences`、`cargo check: ok`、`layout-check passed`（退出码 0） |

## 这次试跑改掉的普遍规则（已写进 `PORTING.md`）

1. **`globals.rs` = `<globals.h>` 镜像的第二半**：`make-docfile` 除了 `Q*` 还写
   `[_Noreturn] [NO_INLINE] EXFUN (Ffoo, maxargs)`；Rust 里"同名声明 + 同名定义"
   是 `E0428`，所以原型放进 `pub mod subrs { … }` 再 `use subrs::*;`——同名定义
   会**遮蔽整体导入**，语义等于 C 的"原型 + 定义"（`sort.c` 需要 `Fvaluelt` 就是
   第一个用例；评审 F1 用"TU 内定义 `Fcons` + 调用 `Ffuncall`/`Fvector`"的探针
   复核，符号表证明 `CALLMANY!` 调用的是自己的 `$f`）。**推论**：镜像里不得再手写
   subr 原型，`lisp.rs` 原先手写的 `Fcons`/`Ffuncall`/`Fsignal` 已删除。
2. **C 的前向声明没有 Rust 对应物**（`merge_register_cleanup`）：删除声明行，把定义
   留在 C 的位置。
3. **C 的"先声明、再在分支里逐字段赋值"没有 Rust 对应物**（`E0381`）：
   `sortslice lo;` → `sortslice { keys: null_mut(), values: null_mut() }`。
4. **edition 2024 的 `#[unsafe(no_mangle)]`**：裸 `#[no_mangle]` 是硬错误。
5. **宏里的"实参个数"必须是常量表达式**：镜像里的 `CALLN!`（对应 C 的
   `ARRAYELTS`）用 `{ let _ = &$arg; 1 }` 计数会以 `E0435` 失败，改成
   `{ let _ = stringify!($arg); 1 }`（求值仍由数组初始化做一次）。
6. **`static_assert` 落在 `lisp.rs`**，调用点 `static_assert! (…)`。
7. **`attributes: noreturn` 的 8 个 subr → `-> !`**（`subrs.tsv` 新增 `flags` 列，
   与 `lisp.rs` 手写的 `Fsignal` 一致；`const`/`pure`/`noinline` 记为不表达，P-25）。
8. **gnulib 头的 WIP 镜像**：`lib/include/stdint.in.rs`（`PTRDIFF_*`、`PTRDIFF_WIDTH`）
   与 `lib/include/limits.in.rs`（`_GL_INTEGER_WIDTH`/`_GL_COB*`），只含当前用到的
   一节，其余留给逐文件补全（P-26）。
9. **检查器只跟随一层 `include!`**：被包含镜像的行数与函数体不再算作本文件的，
   所以桩检测与 5% 漂移只针对文件自身（P-27）。
10. **生成物 TSV 必须转义字段**：`n`（interactive spec）里含制表符与换行，原先会
    把一条记录拆成多行（20 行坏记录）；`gen-baseline.py` 现在写 `\t`/`\n`/`\r` 转义。

## 需要评审确认的偏离

见 `docs/ports/sort.md` 的 "Deviations"（11 条）与 "Call-relationship differences"
（无）。其中语义相关的是：`goto` → 单状态机（不跳过尾部、不改提前 `return`）、
`unwrap_unchecked` 调用 `pred_fun`（前置条件与 C 相同：非 NULL）、`memcpy`/`memmove`
→ `ptr::copy_nonoverlapping`/`ptr::copy`（元素计数 = C 的字节数 / `sizeof`）。

## 评审（对抗评审 B：工具链/ABI）与处置

| 编号 | 结论 | 处置 |
| --- | --- | --- |
| F1 | `lisp.rs` 手写的 `Fcons`/`Ffuncall`/`Fsignal` 原型在 crate 根，定义这些 subr 的 TU 会 `E0428` | 已删除三行；`--check lisp.h` 仍 `OK`；`Fcons` 探针编译通过 |
| F2 | subr 的"原型 ↔ 定义"检查在 Rust 侧失效 | 记入 P-28，阻断 `alloc.c`，须先补机械闸门 |
| F3 | `CALLMANY!` 丢掉 `$f`，169 处非 `Ffuncall` 的 `CALLN`/`CALLMANY` 会调错函数 | 已改为 `($f) (…)`；`calln!` 传 `Ffuncall` 路径；探针 `nm` 显示 `_Fvector` 被引用 |
| F4 | 检查器改动打开桩绕过（实现放进被包含文件即可全桩通过） | 桩检测恢复基于"文件 + 一层镜像"的全文，只跳过 C 文件里没有的名字；5% 行数仍只看文件自身；`evil_tu.rs` 复现 → `FAIL` |
| F5 | 生成器"后写覆盖"丢掉了第 8 个 `noreturn`（`Fx_close_connection`） | 按名字合并：签名不同报错退出，`flags` 取并集；现在 `-> !` 共 8 处 |
| F6 | 原型集是"全配置超集"（多 304 个 GUI/平台名字） | 记为偏离 P-29，待按目标源文件清单过滤（P-08） |
| F7 | `static_assert!` 只接受一个实参 | 已支持可选 message（C23/lib/verify.h 的形态） |
| F8 | `_Noreturn` 只有声明侧、定义侧无约束；`ATTRIBUTE_COLD`/`NO_INLINE` 未记录 | §5.1/§5.2 已写明定义侧必须 `-> !`，其余属性记为不表达 |

评审 B 的"复核为正确"清单（原型 arity/`flags` 列逐条重算、遮蔽机制、TSV 转义、
`_GL_INTEGER_WIDTH` 位精确、生成物可复现、闸门）见其报告；§0.1-10/11、§5.1、§5.2、
P-25…P-29 已据此修订。

## 评审 A（算法保真）与处置

评审 A 用**差分对照**（C 与 Rust 各跑同一语料 2,184 例：n=1…5000、6 种键型、
`values` 两态、`reverse` 两态，外加 96 例"排序中途 signal"）逐字节比对：
`XSYMBOL` 修好后 **0 差异**（含 `cleanup_mem` 的重定位结果与 GC 标记轨迹）。
它同时发现了一个**共享镜像的 P0**——`lisp.rs::XSYMBOL` 返回的是编码偏移而不是指针
（少了 C 的 `XBARE_SYMBOL` 里的 `+ lispsym`），三道闸门全绿却让 `(sort SEQ #'<)`
段错误。这与本项目最危险的失效模式一致：**闸门查的是形状，行为要另设闸门**
（P-30，阶段 B 前必须补）。逐条处置见 `docs/ports/sort.md` 的 "Adversarial review"。
