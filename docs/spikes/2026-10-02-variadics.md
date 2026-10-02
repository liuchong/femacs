# P-04: C variadics (`va_list`) in Rust

参照：`emacs/src/doprnt.c` @ `fd7afced403`（607 行，6 个函数，49 处 `va_list` 用法：
`va_arg` 12 次、`va_start`/`va_end` 各 2 次、`va_copy` 1 次），以及 `lisp.h` 里
`TYPE_MAXIMUM`/`STRING_BYTES_BOUND` 等调用点。
结论与证据脚本：`spikes/va-probe.sh`（纯 C ↔ 纯 Rust，6 行逐字节一致）、
`femacs/rust-toolchain.toml`（P-04 的工具链决定）。

## 工具链：stable 不够，dated nightly 才可复现

| 工具链 | `unsafe extern "C" fn f (…, mut ap: ...)`（无 `#![feature]`） |
| --- | --- |
| stable 1.98.1 | `error[E0554]`（feature 不能在 stable 上启用） |
| `nightly-2026-09-14`（4b6d04e70，1.100.0-nightly） | `error[E0658]: C-variadic functions are unstable` |
| `nightly-2026-10-01`（21b707e3f，1.101.0-nightly） | **编译通过**（`c_variadic` 自 1.99.0 起 stable） |

`nightly-2026-09-14` 早于稳定化提交，rolling `nightly`（574ff7d98）虽然可以但会漂移，
所以 pin 到 **`nightly-2026-10-01`**；stable ≥ 1.99 发布后应改回 stable（届时连
`#![feature]` 都不需要）。内部属性（`#![feature]`）无论如何都不能用：镜像单元是被
`include!` 的，落进模块体会报 "an inner attribute is not permitted in this context"。

## API 映射（逐条对应，`spikes/va_probe.rs` 实测）

| C | Rust（edition 2024） |
| --- | --- |
| `int f (char *fmt, ...)` 定义 | `unsafe extern "C" fn f (fmt: *const c_char, mut ap: ...) -> c_int`（`...` 参数**就是** `VaList<'_>`，无需 `va_start`/`va_end`） |
| `va_list ap` 形参（如 `doprnt`） | `ap: VaList<'_>`（ABI 与平台的 `va_list` 一致，可跨 FFI） |
| `va_arg (ap, T)` | `ap.next_arg::<T> ()`（`unsafe`；`T` 必须实现 `VaArgSafe`） |
| `va_copy (dst, src)` | `let mut dst = src.clone ()`（`Clone` 即 `va_copy`） |
| `va_end (ap)` | 不写（`VaList` 的 `Drop` 即 `va_end`） |
| 把 `va_list` **按值**传给被调函数（C 的语义：本目标上 `va_list` 是结构体，按值传递即复制游标） | 调用点写 `callee (ap.clone ())`；直接 `callee (ap)` 是**移动**，之后调用者不能再读（`E0382`） |

`va-probe.sh` 的两侧输出（6 行完全一致）证明：① 混合类型（`int/double/long/unsigned long/char*`）
的读取一致；② 被调函数拿到的是**独立游标**（它读满 5 个参数后，调用者仍从原位置开始）；
③ `va_copy` 的副本可以独立推进而不动原游标。注意这正是 C 在 Apple arm64 上的语义
（`va_list` 是 `struct __darwin_va_list`，按值传递即结构体复制）——**x86-64 上 `va_list`
是数组类型，按值传递会共享状态**，那时同样的 Rust 写法就不等价；这一点登记在
PORTING.md P-04 的“目标相关”注里，等 32 位/x86-64 目标冻结时再逐目标核对。

## 对 `doprnt.c` 的具体后果

- `doprnt_non_null_end (…, va_list ap)` 把 `ap` 转交给 `doprnt (…, ap)` → Rust 直接
  `doprnt (…, ap)`（移动，之后不再读）✓；`doprnt` 自己不再转发，所以链条上无需 clone。
- `evxprintf` 的 `for (;;) { va_copy (ap_copy, ap); … va_end (ap_copy); }` → Rust
  `loop { let mut ap_copy = ap.clone (); … }`（每次迭代一个副本，外层 `ap` 存活）。
- `esprintf`/`exprintf` 的 `va_start`/`va_end` 消失（`...` 参数即列表）；`exprintf` 在冻结
  配置里是死代码（`HAVE_X_WINDOWS && USE_X_TOOLKIT` 都为假），按 P-08 的决定用
  `#[cfg(feature = "x_windows")]` 包起来并加静态断言。
- `va_arg` 的读取类型必须与调用点写入的类型一致（C 的默认参数提升同样适用）；`doprnt.c`
  读 `int/long/ptrdiff_t/EMACS_INT/intmax_t/unsigned/unsigned long/EMACS_UINT/uintmax_t/double/char*`，
  Rust 侧对应 `c_int/c_long/isize/EMACS_INT/i64/c_uint/c_ulong/EMACS_UINT/u64/c_double/*const c_char`。

## 回归

`bash spikes/va-probe.sh` → `va-probe: 6 lines identical`（`spikes/va_probe.{c,rs}`）；
`xtask build-tu src/doprnt.c` 在 pin 的工具链下编译（见 `docs/ports/doprnt.md`）。
