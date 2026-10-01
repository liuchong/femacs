# PORTING.md — femacs 的 C → Rust 映射指南（v2）

适用基线：`emacs/` 检出 `fd7afced403`（GNU Emacs 32.0.50）。
本文是逐文件移植时的裁决依据。v2 依据 2026-10-02 两名对抗评审的结论重写：
`docs/reviews/2026-10-02-adversarial-review.md` 记录了每条缺陷、证据与处置，
`docs/reviews/2026-10-02-artifacts.sha256` 是评审时的工件哈希。

配套文件：`docs/port-plan.md`（执行手册）、`LIFETIMES.tsv`（生命周期台账，生成物）、
`docs/port-map.md`（逐文件目标与进度）、`docs/baseline/*.tsv`（参照静态基线）、
`tools/cinventory.py`（唯一的 C/Rust 抽取器）、`tools/extract-abi.py`（清单与判定）。

**未知项一律不得自行发挥**：遇到本文未覆盖的构造，先记入 `docs/ports/NAME.md`，
再在本文追加条目（§11），评审确认后统一；同一构造不允许两种写法。

---

## 0. 总则

### 0.1 编译模型（v2 重写：v1 会产生重复符号）

C 与 Rust 的编译模型必须同构，但不能照抄 `#include` 的物理行为：

1. **一个 `.c` = 一个独立编译单元** = `femacs/<同路径>/NAME.rs`，由 `xtask` 逐个调用
   `rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort -C opt-level=…
   --cfg 'feature="…"' -o build/NAME.o femacs/<dir>/NAME.rs` 编译。
2. **头文件 = 声明镜像**：`emacs/<dir>/NAME.h` → `femacs/<dir>/include/NAME.rs`（`src/`
   为 `femacs/src/include/NAME.rs`）。镜像里**只允许声明**：
   - 类型（`#[repr(C)]` struct/union/enum）、常量、`macro_rules!` 定义；
   - 函数与变量一律写成 `unsafe extern "C" { pub fn NAME(…); pub static mut NAME: T; }`；
   - 不允许出现 `#[no_mangle]` 的定义、`pub static` 的定义、`impl` 块或任何会产出
     符号的项——否则每个包含它的编译单元都会重复定义（评审已实测 `ld: duplicate symbols`）。
3. **单一定义点**：每个函数/全局对象在**唯一一个** `.rs`（其 `.c` 的镜像）里定义，
   并 `#[no_mangle] pub unsafe extern "C"` 导出；其余单元只通过头镜像声明引用。
4. **包含去重**：`xtask` 先展开包含图（`include!` 链）并按文件去重，等价于 C 的
   `#ifndef NAME_H` 包含卫；同一编译单元内同一镜像只包含一次，避免重定义错误。
5. **宏的单一来源**：所有 `macro_rules!` 定义集中在一个 prelude 单元（`#[macro_export]`），
   其它单元 `use`；不得把宏定义逐字复制进多个镜像。
6. **函数指针身份**：凡在 C 里取地址或比较函数指针的位置（例如 `specpdl` 的
   `unwind_ptr.func == xfree`），该函数必须**只有一个定义**（不得依赖 `static inline`
   的每单元副本）；`INLINE` / `EXTERN_INLINE`（见 §10.6）逐个判定。
7. **`cfg` 的来源**：`--cfg` 由 `xtask` 从同一份配置生成（§9.1/§9.2），不得手写。
8. **`Lisp_Object` 与 `Lisp_Word` 是 C 的指针型别名**（见 §2.2），不做 newtype 包装。
9. **头镜像的额外规则（A5 落地时实测）**：镜像被 `include!` 进编译单元，因此
   - 不允许内部属性（`#![…]`）与 crate 相对路径（`use crate::…`），allow 由包含它的
     单元给出；
   - 访问器（`XLI`/`XIL`/`XTYPE`…）不能写成 `const fn`——指针与整数互转在常量求值里非法，
     与 C 一样按普通内联函数处理；
   - 作为 union 成员的 `#[repr(C)]` 结构体必须 `Copy`（C 结构体天然可平凡复制），
     这是 union 成员的硬性要求，不影响布局。（`spikes/layout.rs` 与 `lisp.rs` 均已实测）
10. **草稿生成器只做机械部分，且从不写入镜像**：`tools/emit-mirror.py` 从 C 头文件机械地
   产出结构体/联合/typedef/字面常量/`extern` 声明（函数指针字段、位域结构体、无法映射的
   类型会被明确跳过并列入"人工工作"清单）。它的输出是**草稿**（`docs/ports/*-draft.rs`），
   必须逐类型补全并通过布局闸门后才允许合并进镜像——实测把未补全的草稿整块贴进镜像时，
   `xtask layout-check` 立刻以 `sizeof handler` 断言失败拦下，这正是期望行为。
11. **布局断言由工具生成，不手写**：`xtask layout-asserts` 读
   `docs/baseline/layout/c-layout-*.tsv`（C 编译器输出的冻结基线），生成 `const _: () =
   assert!(size_of/offset_of …);`；镜像里没有声明过的类型记为 pending，数量随移植递减，
   但不允许把 pending 当成通过。

### 0.2 命名与可见性

- 函数、宏、结构体、联合、枚举、枚举成员、全局变量、静态变量、subr：**名字逐字相同**
  （`Fcons`、`Scons`、`Qnil`、`Vload_path`、`GC_MARK_STACK`、`DEFUN`、`AUTO_STRING`…）。
- 与 Rust 关键字冲突时用原生标识符：`r#const`、`r#type`、`r#match`、`r#where`、`r#fn`、
  `r#use`、`r#mod`、`r#final`。参照树中实际冲突（抽样）：`const` 96、`type` 31、
  `match` 9、`where` 5、`fn` 5、`use` 2、`mod` 2、`final` 2。
- **声明 ≠ 定义**：C 的 `extern` 声明 → `unsafe extern "C" { … }`（头镜像内）；C 的定义
  → `#[no_mangle] pub`（唯一单元内）。`static`（内部链接）→ 非 `pub`、不导出符号。
- C 的全局变量（尤其 `Lisp_Object` 全局）在 Rust 侧写成 `#[no_mangle] pub static mut NAME: T`
  ——`staticpro` 注册的是它的地址，地址必须稳定（不入 `Box`、不搬移）。

### 0.3 语句与顺序

- 一条 C 语句 = 一条 Rust 语句；定义顺序、`#if` 分支顺序不变。
- 允许并预期 `unsafe`；不得借 `unsafe` 改变语义（例如把 `goto` 清理路径改成提前返回）。
- 禁止 `todo!()`/`unimplemented!()`/空实现/默认返回值/注释掉的代码（硬约束 3）。
- 注释逐字保留，但 C 的注释体里若含 `/*`（如 `w32fns.c:4127`）必须先转义/改写，
  因为 Rust 块注释可嵌套——否则会吞掉后续代码。

### 0.4 参照规模（由 `python3 tools/gen-baseline.py` 生成，可复算）

覆盖 8 个参与链接的 C 目录：`src/`、`lib/`、`lib-src/`、`lwlib/`、`oldXMenu/`、`exec/`、
`nt/`、`msdos/`。

| 度量 | 数值 | 来源 |
| --- | ---: | --- |
| 文件数 / 行数 | 590 / 678,320 | `docs/baseline/inventory.tsv` |
| 函数定义 | 10,012 | 同上（注释剥离、`#if 0` 跳过） |
| 活跃 `DEFUN`（subr） | **1,841** | `docs/baseline/subrs.tsv`（1,849 处文本出现 − 8 处在 `#if 0` 死区） |
| 唯一 subr C 名 | 1,736 | `docs/baseline/totals.tsv` |
| 宏定义 | 7,156 | `inventory.tsv` |
| 具值常量（对象宏 + 枚举成员） | 1,834 | 同上 |
| 结构体/联合字段 | 5,441 | `LIFETIMES.tsv` |
| 文件级对象 | 1,934 | 同上 |

构造计数（评审复算 + 本轮复核，命令见括号）：

| 构造 | 数值 | 命令 |
| --- | ---: | --- |
| `#if/#ifdef/#ifndef` 行 | 6,147 | `grep -rhoE '^[[:space:]]*#[[:space:]]*(if\|ifdef\|ifndef)\b' *.c *.h \| wc -l` |
| `__atomic_*` 内建 | **24** | `grep -rhoE '__atomic_[a-z_]+ *\(' *.c *.h \| wc -l` |
| `ckd_add` / `ckd_sub` / `ckd_mul` | 154 / 19 / 114 | 同法 |
| 位域成员行（`.c`+`.h`） | 557 | `grep -rhoE ':[[:space:]]*[0-9]+[[:space:]]*;' *.c *.h \| wc -l` |
| `staticpro (&…)` 注册调用 | 273 | `grep -rho 'staticpro *(&' *.c \| wc -l` |
| `mark_*` 调用族 | 281 | `grep -rhoE '\bmark_[a-z_]+ *\(' *.c *.h \| wc -l` |
| `eassert` / `clobbered_eassert` | 1,806 / 6 | 同法 |
| `DEFSYM`（含定义） | 2,311 | 同法（调用 2,310） |
| `INLINE` 标注 | 638 | `grep -rho '\bINLINE\b' *.c *.h \| wc -l` |
| 裸 `alloca (` | 160 | 同法 |
| `SAFE_ALLOCA_LISP` / `SAFE_ALLOCA_STRING` | 30 / 17 | 同法 |
| `setjmp` / `longjmp` 词频 | 34 / 41 | 其中 `sys_setjmp` 18、`sys_longjmp` 11 |
| `va_list` | 49 | 同法 |
| computed goto | 1 处（`bytecode.c:581`） | `grep -rn 'goto \*' *.c` |
| 线程局部 / C11 原子 | 0 / 见 `__atomic_*` 24 | 两者不同：见 §8 |

> 注：v1 中“8,991 函数”“1,850 subr”“位域 30”“原子 0”等数字已废弃；以本表为准。

---

## 1. 类型映射

### 1.1 标量

| C | Rust | 说明 |
| --- | --- | --- |
| `EMACS_INT` / `EMACS_UINT` | `isize` / `usize` | 与 C 同宽 |
| `Lisp_Word` / `Lisp_Object` | `*mut Lisp_X`（见 §1.2） | C 默认构建下是指针型 |
| `ptrdiff_t` / `size_t` | `isize` / `usize` | 用 `libc` 别名 |
| `bool` | `bool` | 1 字节，与 C 一致 |
| `bool_bf` | 位域（§1.3） | 不可用 `bool` |
| `printmax_t` / `intmax_t` / `uintmax_t` | `i64` / `i64` / `u64` | 打印语义 |
| `char`（字节） | `c_char` | `unsigned char *` → `*mut u8` |
| `long double`（9 处） | `c_longdouble` | 不得降级为 `f64` |
| `float` / `double` | `f32` / `f64` | IEEE 一致 |

隐式整型提升与回绕：凡 C 里依赖二进制补码回绕的表达式，必须显式写
`wrapping_add`/`wrapping_mul`/`wrapping_shl` 等，保留原表达式结构。
C23 的 `ckd_add/ckd_sub/ckd_mul`（287 处，`lib/stdckdint` 语义）→ 用
`overflowing_*` + 目标类型转换判定，逐处核对“溢出时不写入目标”的语义。

### 1.2 `Lisp_Object`（v2 更正）

C（`lisp.h:311-323`、`lisp.h:580-600`）：

```c
typedef struct Lisp_X *Lisp_Word;          /* LISP_WORDS_ARE_POINTERS 默认成立 */
typedef Lisp_Word Lisp_Object;             /* 非 CHECK_LISP_OBJECT_TYPE 时 */
#define lisp_h_XLI(o) ((EMACS_INT) (o))    /* 指针/整数互转 */
#define lisp_h_XIL(i) ((Lisp_Object) (i))
```

Rust：

```rust
#[repr(C)] pub struct Lisp_X { _private: [u8; 0] }   // 不完整类型，仅用于指针
pub type Lisp_Word = *mut Lisp_X;
pub type Lisp_Object = Lisp_Word;
#[inline(always)] pub const fn lisp_h_XLI(o: Lisp_Object) -> EMACS_INT { o as isize }
#[inline(always)] pub const fn lisp_h_XIL(i: EMACS_INT) -> Lisp_Object { i as *mut Lisp_X }
```

- **P-01**：`Lisp_Object` 保持 C 默认构建的**指针型别名**；`CHECK_LISP_OBJECT_TYPE`
  与 `LISP_WORDS_ARE_POINTERS=false`（wide-int）**不支持**，在 `xtask` 中直接拒绝
  （记偏离项）。
- **P-02（`USE_LSB_TAG`）**：它不是常量而是由 `ALIGNOF_EMACS_INT`、`HAVE_C_ALIGNASOF`、
  `VAL_MAX/2 < INTPTR_MAX` 推导（`lisp.h:250-258`），并决定 `VALMASK`、`Lisp_Int1`、
  `lisp_h_XTYPE` 是否存在、`make_fixnum` 形态与 dump 格式。移植策略：把
  `USE_LSB_TAG` 提升为**构建期 `cfg` 决策**，只支持 `= true` 的 64 位目标；其余目标
  拒绝构建。`VALMASK`/`GCTYPEBITS`/`INTTYPEBITS`/`Lisp_Type`/`pvec_type` 等逐值一致，
  并由编译期断言锁定。
- `lisp_h_make_fixnum` 的 C 形态依赖 `HAVE_STATEMENT_EXPRESSIONS` + `typeof`（`lisp.h:408-417`）：
  Rust 宏必须先做 `let n_: EMACS_INT = ($n) as EMACS_INT;` 再做溢出断言，且断言按
  `cfg(enable_checking)` 开关——**不得在 C 无断言的构建里新增断言**。

### 1.3 `struct` / `union` / `enum` / 位域

- `struct` → `#[repr(C)] pub struct NAME { … }`，字段顺序、类型、命名一致；嵌套匿名
  `struct`/`union` 必须显式命名嵌入，**不得改变布局**。
- `union` → `#[repr(C)] pub union NAME { … }`；读写一律 `unsafe`。
- `enum` → `#[repr(C)]`（C 的 enum ABI 是 `int`）；`ENUM_BF(T)`（位域里的枚举）见下。
- **位域**（`.c`+`.h` 共 557 行，含 282 处 `bool_bf`）在 Rust 无对应物 → 用等宽整数 +
  显式 `get_*`/`set_*` 辅助，位宽、位序、整体宽度必须逐位一致；与 dump/`memcpy`/`union`
  混用处逐个核对（§6.4）。`ENUM_BF (T) name : 2` 的宽度是 2 位，不是 `T` 的宽度。
- 变长结构体（`FLEXIBLE_ARRAY_MEMBER` 11 处）→ `data: [u8; 0]` + `offset_of!`。

### 1.4 函数指针与类型双关

- 带 context 的函数指针逐字对应 `unsafe extern "C" fn(*mut c_void) -> …`，字段名与签名
  相同；**不得改成闭包**（会改变布局与调用约定）。
- `*(T *) p`、union 换成员读、`memcpy` 到别类型 → `ptr::read_unaligned`/`write_unaligned`
  或 union 读，保持未对齐访问语义。

---

## 2. 控制流与表达式

### 2.1 宏 → `macro_rules!`

- 带代码的 `#define` → 同名 `macro_rules!`；`do { … } while (0)` → 块表达式。
- **实参求值次数必须与 C 相同**：多数宏可先 `let` 绑定一次，但 C 里存在**多次求值**的宏
  ——`AVAIL_ALLOCA(size) ((sa_avail -= (size)), alloca (size))`、`SAFE_ALLOCA`（对外层
  `size` 两次）、以及 release 下 `eassert`（**完全不求值**）。这些是“单次求值”规则的
  例外，必须逐条保留原求值次数（P-16）。
- 变参宏（`__VA_ARGS__`）→ `($($args:tt)*)`。

### 2.2 `switch` / `goto` / 标签

- `switch` → `match`；`case` 顺序、共享体（fallthrough）、`goto` 落入 case 中部
  （`bytecode.c` 大量使用）必须逐处等价。Rust 无 fallthrough：用显式结构化写法
  （共享体抽成同一分支、或用 `loop { … }` 状态机），语义等价由评审逐函数核对。
- 前向 `goto` 清理路径 → `'block: loop { … break 'block; }` 或状态机，**不得改成提前
  `return`** 而跳过后面的清理。

### 2.3 computed goto（`bytecode.c`，v2 更正）

C 里已有两套实现（`bytecode.c:577-598`）：

```c
#define NEXT goto *(targets[op = FETCH])   /* HAVE_GCC_LABELS */
...
#define NEXT break                          /* 否则走 switch */
```

Rust **没有 label-as-value（`&&label`）**，无法构造 `void *targets[]` 跳转表。决策
**P-03**：移植 C 的 `switch` 实现（即 `HAVE_GCC_LABELS = false` 分支，`CASE`/`NEXT`/
`FIRST` 宏名保留，`insn_*` 标签保留），把“threaded 版本不移植”记为偏离项；
`HAVE_GCC_LABELS` 属于 §9.2 的**结构条件**。

### 2.4 变参函数（`va_list` 49 处，v2 更正）

`va_list` 在 `doprnt.c` 里是**普通参数**传递，且用到 `va_copy`（`doprnt.c:595`）。

- **P-04（2026-10-02 实测更新）**：本机 stable 1.98.1 定义 C 变参函数报
  `E0658: C-variadic functions are unstable`；nightly 1.100.0（2026-09-14）可定义，但
  `VaList` 的方法名已与旧文档不同（实测 `args.arg()` 不存在）。结论：
  1. 变参文件（`va_list` 49 处）在 stable ≥ 1.99 发布前必须用 nightly；
  2. `rust-toolchain.toml` 锁定具体版本（当前 `nightly-2026-09-14`），并在每次升级时
     用 `doprnt.c` 的 `va_copy`/`vsnprintf` 用例重新确认 API 名；
  3. 若届时 stable 已支持，则切回 stable 并删除该偏离记录。
- 变参**调用**（`printf` 家族）不受此限。

### 2.5 断言（v2 更正，评审判定为阻断级）

`eassert`/`eassume` 的 C 语义（`lisp.h:180-196`、`lib/verify.h`）：

| 形态 | C 行为 |
| --- | --- |
| `ENABLE_CHECKING` 且 `suppress_checking == false` | 求值并在失败时 `die` |
| `ENABLE_CHECKING` 且 `suppress_checking == true` | 求值与否按实现，但**不 die**（运行期全局短路） |
| release（`!ENABLE_CHECKING`） | `((void) (false && (cond)))`——**完全不求值** |

Rust 侧三种形态逐一对应，`suppress_checking` 作为同名全局保留；**release 分支不得求值**。
`eassume` → `assume` 语义（`lib/verify.h:389` 的四种平台形态）。另需移植
`clobbered_eassert`（`eval.c:1373` + 5 处）、`verify` 两态、`static_assert`（91 处）、
`eassume`（184 处）。

### 2.6 数值与内建

| C | Rust |
| --- | --- |
| `ckd_add/sub/mul`（287 处） | `overflowing_*` + 目标类型判定（溢出时不写目标） |
| `__builtin_expect` | 保留为分支提示辅助；**不等价**于块布局提示，记偏离项 |
| `__builtin_bswap*` / `__builtin_inff` | `swap_bytes` / `f32::INFINITY` |
| `__builtin_unreachable` | `unsafe { core::hint::unreachable_unchecked() }` + SAFETY |
| `__builtin_unwind_init` / `frame_address` | 见 §4.3（不是 `black_box`） |

---

## 3. 内存

### 3.1 分配器

- `xmalloc`/`xrealloc`/`xfree`/`xzalloc`/`lisp_malloc`/`lisp_free`/`lisp_align_malloc`/
  `pure_alloc`/`mmap_*` 同名移植；Lisp 对象只经这些路径分配。
- **禁止** `#[global_allocator]`；Rust 侧不得用 `Box`/`Vec`/`String` 分配 Lisp 对象。

### 3.2 `alloca` / `SAFE_ALLOCA`（v2 更正）

C 的预算是**每函数共享 16 KB**（`lisp.h:5616-5631`）：`sa_avail` 动态递减，超出转
`record_xmalloc` + `unwind` 释放。

- 计数：裸 `alloca (` 160 处；`SAFE_ALLOCA_LISP` 30、`SAFE_ALLOCA_STRING` 17、
  `REGEX_USE_SAFE_ALLOCA` 2。
- **P-05**：保留 `sa_avail` 记账语义 + **单一栈缓冲池**（不是"每调用点固定缓冲"），
  超限走 `record_xmalloc` 兜底；`SAFE_ALLOCA_LISP` 等必须与 C 一样通过 specpdl 的
  unwind-array 注册，才能被 GC 标记（`eval.c:4345`、`lisp.h:5716`）。
- **循环内 alloca**（`xmenu.c:1021`、`pgtkmenu.c:303`、`w32menu.c:547` 等）依赖每次迭代
  地址不同：这些点必须逐点特判（栈池按偏移分配），否则会互相覆盖。
- 三种候选（栈指针内联汇编 / 栈池 + 记账 / 全走堆）在 spike 中用内存压力对拍选定；
  选定前不得批量移植含 `alloca` 的文件。

### 3.3 拷贝与未对齐访问

`memcpy`/`memmove`/`memset` → 绑定 libc 同名函数，不用 Rust 的
`copy_nonoverlapping` 替代（C 侧可能有重叠/别名假设），逐处核对。

### 3.4 生命周期

每个结构体字段/全局对象的决策写入 `LIFETIMES.tsv`（`owner`、`freed_by` 非空即为已决）。
**Drop 规则**：可能被 `setjmp`/`longjmp` 跳过的路径上不得有带 `Drop` 的局部值。

---

## 4. 错误处理、GC 与栈

### 4.1 非本地跳转（v2 更正）

C 有三种选择（`lisp.h:2271-2287`），必须按条件编译等价移植：

```c
#ifdef HAVE__SETJMP   typedef jmp_buf sys_jmp_buf;   # define sys_setjmp(j) _setjmp (j)
#elif defined HAVE_SIGSETJMP  typedef sigjmp_buf sys_jmp_buf; # define sys_setjmp(j) sigsetjmp (j, 0)
#else                 typedef jmp_buf sys_jmp_buf;   # define sys_setjmp(j) setjmp (j)
```

- 注意 `sigsetjmp` 的第二个参数是 **0（savemask = 0）**，v1 误写为 1。
- macOS/BSD 走 `_setjmp`/`_longjmp` 分支；Windows 的 `__builtin_longjmp`（`w32fns.c`）
  必须与其调用者使用的 `_setjmp` 变体配对。
- `struct handler` 字段与链表语义、`push_handler`/`unwind_to_catch`/`Fsignal` 控制流逐字保留。
- `panic = "abort"`；不用 `Result` 传 C 返回码；不用 `catch_unwind` 模拟 `catch`。

### 4.2 GC 根与标记

- `staticpro (&NAME)`（273 处）逐处保留；`static mut` 地址必须稳定。
- `mark_*` 调用族（281 处）同名移植；specpdl 的 unwind-array 必须被标记与释放。

### 4.3 保守栈扫描（v2 新增，评审判定为阻断级）

C 依赖的机制必须逐个给出 Rust 等价物，`black_box`/空 `asm!` **不能**替代
`__builtin_unwind_init`：

| C 机制 | 位置 | Rust 决策 |
| --- | --- | --- |
| `SET_STACK_TOP_ADDRESS`（两种实现：`__builtin_unwind_init` 或 `test_setjmp()+sys_setjmp`） | `alloc.c:5190-5203` | 采用 `sys_setjmp` 的 jmp_buf 路径（ABI 保证寄存器刷栈） |
| `test_setjmp()` 启动自检（失败 `exit(1)`） | `alloc.c:5087-5170` | 一比一移植，同一失败行为 |
| `GC_SETJMP_WORKS` / `GC_MARK_SECONDARY_STACK` | `configure.ac:6841`、`alloc.c:5257` | 作为构建期常量，取值与 configure 结果一致 |
| `NEAR_STACK_TOP`、栈增长方向判定 | `thread.h:42-46` | 逐行移植 |
| 防尾调用（`__asm__("")` 屏障） | `lisp.h:4466-4471`（bug#65727） | 在同样位置保留屏障；并规定 GC 可见函数的编译单元禁用尾调用消除 |
| `__builtin_frame_address`（`comp.c`） | `comp.c:2164` | `asm!` 取帧地址，逐处 SAFETY |

### 4.4 dump（pdumper）

- **P-09**：偏移由 `src/dmpstruct.awk` 解析 C 源文本生成 `dmpstruct.h`。Rust 侧必须在
  `xtask` 中重新实现等价的 offset/fingerprint 生成器（解析 Rust 结构体镜像），
  否则 dump 校验无输入。可 dump 对象的布局冻结，不得重排。

---

## 5. Lisp 接口

### 5.1 `DEFUN` → `defun!`（1,841 个活跃 subr，v2 更正）

C：

```c
DEFUN ("cons", Fcons, Scons, 2, 2, 0,
       doc: /* Create a new cons, give it CAR and CDR as components, and return it.  */)
```

Rust（**docstring 必须是字符串字面量**，v1 用注释的写法没有机械保障——评审实测
`stringify!` 得到 `doc : ,`）：

```rust
defun!("cons", Fcons, Scons, 2, 2, 0,
       doc: "Create a new cons, give it CAR and CDR as components, and return it.",
       (car: Lisp_Object, cdr: Lisp_Object));
```

- 参数上下限、interactive 规格、Lisp 名逐字一致；`--check` 比对 docstring 摘要（
  `docs/baseline/subrs.tsv` 的 `ldoc_digest`）。
- 若某文件的 `make-docfile` 需要从注释抽取，则必须同时满足“注释与字符串字面量逐字一致”，
  并由 `--check` 机械校验（双轨契约）。
- `struct Lisp_Subr` 布局与 `lisp.h` 一致（含函数指针 union、参数上下限、docstring 指针）。

### 5.2 注册与符号

- `syms_of_*`/`keys_of_*` 同名、`#[no_mangle]`，内部调用顺序与 C 完全一致（顺序影响
  `Qnil`/`Qt` 初始化与 dump 内容）；`defsubr` 1,848 处、`DEFSYM` 2,310 调用逐处保留。

### 5.3 动态模块 ABI

- `src/emacs-module.in.h`（生成出 `emacs-module.h`）与 `src/module-env-{25..32}.h`
  逐字保留：字段顺序、函数指针签名、版本常量，全部由编译期断言锁死大小与偏移。

---

## 6. 线程、信号、原子（v2 更正）

- Emacs 用自己的 `systhread.c` 原语 + 全局锁；Rust 侧**不引入 `std::thread`/`std::sync`
  语义**，移植到 libc pthread，保持信号掩码、栈大小、取消语义一致。
- **`__atomic_*` 有 24 处**（`android.c` 19、`androidterm.c` 3、`keyboard.c:8452`、
  `xgselect.c:43,55`），且与同一对象的**非原子访问混用**（`androidterm.c:6640` vs `6649`、
  `xgselect.c:46` vs `43`）。移植规则：用 `core::sync::atomic` 的 `load`/`store`/`swap`
  配相同的内存序（`__ATOMIC_ACQUIRE/RELEASE/ACQ_REL/SEQ_CST`），保持同一对象；混用处
  必须逐点核对并记录（这类代码的正确性依赖同一变量上的偏序）。
- 线程局部存储：参照树中为 0 处（`_Thread_local`/`__thread` 均无）。

---

## 7. 构建与配置

### 7.1 `config.h` 是输入（v2 更正）

`emacs/src/config.h` 在检出里**不存在**：必须先对每个目标平台运行 `configure` 并把
`config.status` 冻结（记录到 `docs/baseline/config/<target>.json`），再由 `xtask` 生成
`femacs/src/include/config.rs`（同名常量）与 `--cfg` 集合。没有它，§7.2 的判定没有输入。

### 7.2 `#if` 三分法（v2 更正）

| 类别 | 例子 | 翻译 |
| --- | --- | --- |
| 常量条件（两侧都能编译） | `#if HAVE_PDUMPER` 包住语句 | `if HAVE_PDUMPER { … }`（常量折叠） |
| 结构条件（影响类型/字段/函数存在） | `#ifdef HAVE_X_WINDOWS`、`HAVE_GCC_LABELS` | `#[cfg(feature = "…")]` |
| 包含选择 | `# include <sys/…>` 分支 | `xtask` 决定包含哪个镜像 |

每个 `#if` 都要在 `docs/ports/NAME.md` 标记类别；`#elif`/`#else` 的分支同样逐条处理。

### 7.3 DOC 与生成物

- docstring 抽取由移植后的 `make-docfile` 完成，`etc/DOC` 必须与参照逐字节一致。
- `admin/unidata`、`admin/charsets`、`src/lisp.mk`、`ldefs-boot.el` 由 `xtask` 驱动原
  Lisp 脚本生成，产物逐字节比对。

---

## 8. 属性与内建（v2 更正：宏族才是主体）

| C 宏族 / 属性 | 数量 | Rust |
| --- | ---: | --- |
| `GCALIGNED_STRUCT` / `GCALIGNED_UNION_MEMBER` | 31 / 13 | `#[repr(C, align(…))]` / 显式对齐填充 |
| `EXTERNALLY_VISIBLE` | 39 | `#[no_mangle] pub` |
| `ATTRIBUTE_*`（section/align/pure/const/no_sanitize…） | 112 | `#[link_section]`、`#[repr(align)]`、其余记无操作 |
| `NO_INLINE` / `INLINE` / `EXTERN_INLINE` | 30 / 638 | `#[inline(never)]` / `#[inline]`；`INLINE` 的链接语义见 §0.1-6 |
| `cleanup(…)`（`nsterm.h:281`） | 少量 | 作用域守卫（显式调用），逐个核对 |
| `weak_import`（`macfont.h:88`）、`force_align_arg_pointer`（`w32term.h:36`） | 少量 | 目标平台属性/放弃并记偏离 |
| 变量对齐（`android.h:360`） | 少量 | 显式对齐分配 |
| `DEFINE_GDB_SYMBOL*` | 6 个符号 | `#[no_mangle] pub static`，值语义一致 |
| `DEFINE_LISP_SYMBOL` 生成的 `Qxxx` | 依 `DEFSYM` | 同名 `static mut` + 注册 |

`.gdbinit` 还依赖 30+ 类型/枚举名与 `gdb.lookup_symbol("USE_LSB_TAG")`：类型名保持
一致（§0.2），符号用 `#[no_mangle] pub static` 暴露。

内联汇编 1 处 → `core::arch::asm!`，约束与副作用逐条对应并加 SAFETY。

---

## 9. 未决项（每条都有归属与验证方式）

| 编号 | 未决项 | 现状 | 验证 |
| --- | --- | --- | --- |
| P-01 | `Lisp_Object` 用指针型别名；拒绝 wide-int / CHECK_LISP_OBJECT_TYPE | 已定 | 构建期拒绝逻辑 + 断言 |
| P-02 | `USE_LSB_TAG` 作为 `cfg` 决策，仅支持 true 的 64 位目标 | 已定 | 配置生成器 + 断言 |
| P-03 | computed goto 只移植 switch 版 | 已定（偏离已记录） | 基准对比 + ERT |
| P-04 | 变参：稳定版 ≥1.99 的 `c_variadic`，锁具体版本 | 待 spike | `doprnt.c` 试跑 + `va_copy` 用例 |
| P-05 | `alloca`：栈池 + `sa_avail` 记账 + 循环点特判 | 待 spike | 内存压力对拍 + 循环点用例 |
| P-06 | `static mut` 地址稳定 | 已定 | `staticpro` 断言 |
| P-07 | docstring 用字符串字面量，`--check` 校验摘要 | 已定 | `docs/baseline/subrs.tsv` |
| P-08 | `config.h` 需按目标冻结 | **部分完成**：darwin-arm64（minimal）已冻结；GUI/其他目标待冻结 | `docs/baseline/config/*.json` + `xtask gen-config` |
| P-09 | `dmpstruct.awk` 需在 Rust 侧重实现 | 待办 | dump 双向互换 |
| P-10 | setjmp 三分支 + Windows `__builtin_longjmp` 配对 | 待办 | 目标平台 spike |
| P-11 | 断言族（`clobbered_eassert`/`assume`/`verify`/`static_assert`） | 待办 | 逐条映射表 |
| P-12 | `INLINE` 链接语义与函数指针身份 | 待办 | 地址相等断言 |
| P-13 | 属性宏族与 `cleanup`/`weak_import` | 待办 | 逐条映射表 |
| P-14 | `.gdbinit` 的类型/枚举依赖 | 待办 | GDB 冒烟 |
| P-15 | `ckd_*` 语义逐处核对 | 待办 | 溢出用例 |
| P-16 | 多次求值宏与 release 断言 | 待办 | 宏展开对照 |
| P-17 | `Q*` 符号（由生成的 `globals.h` 提供）在 Rust 侧实现为**同名内联函数** | 已定 | `xtask gen-globals` + `docs/baseline/globals/symbols.tsv` |

**P-17 说明（实测决定）**：参照用 `make-docfile` 生成 `globals.h`（`#define iQfoo N` +
`DEFINE_LISP_SYMBOL (Qfoo)`），使 `Qfoo` 成为**编译期常量表达式**。Rust 无法把符号数组地址
折进常量，因此只有两种选择：`static mut Qfoo`（每个使用点都要 `unsafe`，在 590 个文件的
规模上不可接受）或**同名内联函数** `Qfoo()`（无 unsafe、值相同、名字相同，仅调用形状多一对
括号）。采用后者；`lisp.rs` 像 C 的 `#include <globals.h>` 一样 `include!("globals.rs")`。
符号集合与 Lisp 名冻结在 `docs/baseline/globals/symbols.tsv`（2,118 个，`Qnil` = 0、`Qt` = 1，
其余按确定性顺序；顺序本身是我们的生成器内部细节，dump 保存的是对象而不是索引）。

## 10. 逐文件自检清单

1. `python3 tools/extract-abi.py emacs/<dir>/NAME.c > docs/ports/NAME.md`（生成清单）。
2. 文件头、命名、§1 类型映射、§0.2 声明/定义规则全部遵守。
3. `#if` 逐个标记三类（§7.2）。
4. 位域/union/函数指针/变长结构体/alloca 点按 §1.3/§3.2 处理并记录。
5. 错误路径（`setjmp`/`longjmp`/`goto`）按 §2.2/§4.1 保留。
6. GC 根与分配路径按 §3/§4.2 检查（`staticpro` 是否齐全）。
7. `DEFUN`/docstring/注册顺序按 §5 检查。
8. `python3 tools/extract-abi.py --check emacs/<dir>/NAME.c femacs/<dir>/NAME.rs` 必须 OK。
9. 提交前冻结工件并记录哈希；评审看的是提交后的固定版本。
