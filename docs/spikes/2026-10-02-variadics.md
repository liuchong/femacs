# P-04: C variadics (`va_list`) in Rust

参照：`emacs/src/doprnt.c` @ `fd7afced403`（607 行，6 个函数）。 计数按**文件**口径：
`va_list` 出现 **7** 次，`va_arg` **12**、`va_start` **2**、`va_end` **3**、`va_copy` **1**
（`src/` 整树才是 49 处 `va_list`——第一版把树级计数写成了单文件属性，评审 A 的 F2 更正）。
另外 `lisp.h` 里有 `TYPE_MAXIMUM`/`STRING_BYTES_BOUND` 等调用点。
结论与证据脚本：`spikes/va-probe.sh`（纯 C ↔ 纯 Rust，6 行逐字节一致）、
`femacs/rust-toolchain.toml`（P-04 的工具链决定）。

## 工具链：stable 不够，dated nightly 才可复现

| 工具链 | `unsafe extern "C" fn f (…, mut ap: ...)`（无 `#![feature]`） |
| --- | --- |
| stable 1.98.1 | `error[E0658]: C-variadic functions are unstable` |
| `nightly-2026-09-14`（4b6d04e70，1.100.0-nightly） | **编译通过**，产物与 C 侧 6 行逐字节相同 |
| `nightly-2026-10-01`（21b707e3f，1.101.0-nightly） | **编译通过**（`c_variadic` 自 1.99.0 起 stable） |

三行都是实测值（评审 A 的 F1、评审 D 的 F6 各自用 `rustup run <toolchain> rustc` 复现）。
第一版的表格把 stable 写成 `E0554`（那是**带** `#![feature]` 时才会出现的错，与表头
「无 `#![feature]`」自相矛盾）、把 `nightly-2026-09-14` 写成失败，两处都错，已更正。

pin 仍然选 `nightly-2026-10-01`，但理由是**本轮全部闸门证据都在它上面测**，不是因为
09-14 不行；rolling `nightly`（574ff7d98）虽然也能编，但会漂移。stable ≥ 1.99 发布后
应改回 stable（届时连 `#![feature]` 都不需要）。内部属性（`#![feature]`）无论如何都
不能用：镜像单元是被 `include!` 的，落进模块体会报 an inner attribute is not permitted
in this context。

Miri 也必须来自 **pin**：`spikes/miri-probe.sh` 此前用 rolling `+nightly`（`574ff7d98`，
1.100.0-nightly，比 pin 旧），而且 pin 没装 miri 时脚本打印 skipped 并 **以 0 退出**。
现在脚本从 `femacs/rust-toolchain.toml` 读 pin、缺组件就 **exit 1** 并给出安装命令；
本 pin 上已安装，实测 `miri-probe: clean (nightly-2026-10-01)`。

## API 映射（逐条对应，`spikes/va_probe.rs` 实测）

| C | Rust（edition 2024） |
| --- | --- |
| `int f (char *fmt, ...)` 定义 | `unsafe extern "C" fn f (fmt: *const c_char, mut ap: ...) -> c_int`（`...` 参数**就是** `VaList<'_>`，无需 `va_start`/`va_end`） |
| `va_list ap` 形参（如 `doprnt`） | `ap: VaList<'_>`（ABI 与平台的 `va_list` 一致，可跨 FFI） |
| `va_arg (ap, T)` | `ap.next_arg::<T> ()`（`unsafe`；`T` 必须实现 `VaArgSafe`） |
| `va_copy (dst, src)` | `let mut dst = src.clone ()`（`Clone` 即 `va_copy`） |
| `va_end (ap)` | 不写（`VaList` 的 `Drop` 即 `va_end`） |
| 把 `va_list` **按值**传给被调函数（C 的语义：本目标上 `va_list` 是 `char *`，按值传递即复制那个指针 → 被调函数的游标独立） | 调用点写 `callee (ap.clone ())`；直接 `callee (ap)` 是**移动**，之后调用者不能再读（`E0382`） |

平台模型（实测，评审 D 的 F5）：本目标 `va_list` 就是 `char *`
（`sizeof (va_list) == 8`，`__builtin_types_compatible_p (va_list, char *) == 1`；Rust 侧
`size_of::<VaList>() == 8` 且 `VaList` 不是 `Copy`）。第一版文档写的「结构体复制」是错的；
**x86-64 上 `va_list` 是数组类型、按值传递会共享状态**这一条才真正要守，故保留在
PORTING.md P-04 的目标相关注里，等 32 位/x86-64 目标冻结时逐目标核对。

`va-probe.sh` 的两侧输出（6 行完全一致）证明：① 混合类型（`int/double/long/unsigned long/char*`）
的读取一致；② 被调函数拿到的是**独立游标**（它读满 5 个参数后，调用者仍从原位置开始）；
③ `va_copy` 的副本可以独立推进而不动原游标。 第一版探针有两行读的是「下一实参是
`double` 时的 `int`」（C 里就是 UB），两侧读到的是同一段确定性垃圾，因此那两行**不具备
判别力**；现在每一处读取都读一个真实传入的实参（评审 D 的 F7）：

```
A take 1 2.5 3 4 str
A copy 1 orig 1 more 2.5
A after 1 2.5
B take -7 -0.5 -9 10 str
B copy -7 orig -7 more -0.5
B after -7 -0.5
```

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
  Rust 侧对应 `c_int/c_long/ptrdiff_t/EMACS_INT/i64/c_uint/c_ulong/EMACS_UINT/u64/c_double/*mut c_char`。
  这张对应表现在是**机器检查的**：`docs/ports/va-arg-types.tsv` + `extract-abi.py` 的
  `va_arg_problems` 比对两侧的**类型多重集**，未记录的类型直接报错（PORTING.md P-45，
  评审 D 的 F4——`%u` 读成 `c_int` 曾经能编译且全绿）。

## 回归

`bash spikes/va-probe.sh` → `va-probe: 6 lines identical`（`spikes/va_probe.{c,rs}`）；
`bash spikes/miri-probe.sh` → `miri-probe: clean (nightly-2026-10-01)`；
`xtask gate-probes` 一并跑这四个 C 差分脚本（`inline 6081` / `accessor 11` / `write 4` /
`va 6` 行），并在 `build/emacs-ref` 缺失时**报错**而不是跳过（PORTING.md P-47）；
`xtask build-tu src/doprnt.c` 在 pin 的工具链下编译（见 `docs/ports/doprnt.md`）。
