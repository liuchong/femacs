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
   - 参照里**只以指针出现的不完整类型**（`struct buffer *`）在“不含 buffer.h 的编译单元也要
     能编译”的镜像里拼不出来：Rust 侧写 ABI 等价的 `*mut c_void`，并在该声明上方留一行
     `/// C: <参照原型>`，`spikes/lisp-abi-probe.sh` 按这一行核对参照头（P-55）。
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
10. **`globals.rs` 是 `<globals.h>` 的镜像**：它同时给出两半——`Q*` 内联函数
    （P-17）与**全部 DEFUN 的 `EXFUN` 原型**（`make-docfile` 写的
    `_Noreturn` / `NO_INLINE` / `EXFUN (Ffoo, maxargs)` 前后缀，见
    `lib-src/make-docfile.c` 740-780）。原型放在 `pub mod subrs { … }` 里再
    `use subrs::*;`：Rust 里"同名声明 + 同名定义"是 `E0428`，而**同名定义会遮蔽整体
    导入**（整体导入优先级最低），语义等于 C 的"原型 + 定义"（A6 实测：TU 内定义
    `Fcons` + 调用 `Ffuncall`/`Fvector` 均编译通过，符号表证明 `CALLMANY!` 调用的是
    自己的 `$f`）。
    **推论（评审 F1）**：镜像里**不许**再手写任何 subr 的原型——C 的 subr 原型只来自
    `<globals.h>`，手写一份就等于在 crate 根放了同名项，定义该 subr 的 TU 立刻 `E0428`
    （`lisp.rs` 原先手写的 `Fcons`/`Ffuncall`/`Fsignal` 已按此删除）。
11. **C 的"原型 ↔ 定义"类型检查在 subr 上失效（评审 F2，P-28）**：因为定义遮蔽整体
    导入，生成的 `EXFUN` 原型不会与 `DEFUN!` 定义比对；C 会在编译期拒绝的
    arity/返回类型不一致（例如 noreturn subr 的 `-> !`）在 Rust 里**静默通过**。
    第一个定义 subr 的 TU（`alloc.c`）开工前必须在 `tools/extract-abi.py` 里补上
    机械替代：把 `DEFUN!` 形式的参数个数与返回类型同 `docs/baseline/subrs.tsv`
    对齐，并禁止 `pub mod subrs` 之外的 crate 根 `F*` 声明。
12. **C 的前向声明没有 Rust 对应物**：`static void f (T *);` 之后再定义，在 Rust 是
    `E0428`，而 Rust 的项与顺序无关。删除声明行、把定义留在 C 的位置，并在
    `docs/ports/NAME.md` 的偏离项里记录。
13. **C 的"先声明变量、再在分支里逐字段赋值"没有 Rust 对应物**（`E0381`：Rust 不支持
    部分初始化）。按 C 自己的零值习惯先整体初始化（`T v = T { 空值 };`），后续字段赋值
    逐条保留，并记录偏离。
14. **包含顺序：`config.rs` → `lisp.rs` → 其他镜像**。`lisp.rs` 里定义 `eassert!` /
    `eassume!` / `CHECK_TYPE` 等宏，其余镜像（`character.rs`、`itree.rs`、`buffer.rs`…）
    在函数体里**文本展开**它们，所以 `include!` 顺序与 C 的"先 `lisp.h`"一致；
    `xtask` 的布局探针也按这个顺序拼装（A6 实测：顺序错了会 `cannot find macro eassert`）。
15. **草稿生成器只做机械部分，且从不写入镜像**：`tools/emit-mirror.py` 从 C 头文件机械地
   产出结构体/联合/typedef/字面常量/`extern` 声明（函数指针字段、位域结构体、无法映射的
   类型会被明确跳过并列入"人工工作"清单）。它的输出是**草稿**（`docs/ports/*-draft.rs`），
   必须逐类型补全并通过布局闸门后才允许合并进镜像——实测把未补全的草稿整块贴进镜像时，
   `xtask layout-check` 立刻以 `sizeof handler` 断言失败拦下，这正是期望行为。
16. **镜像自检（每份镜像提交前）**：类型名扫描必须先去注释/字符串（注释里的 `struct foo` 不是声明）；`counts.tsv` 的下限只许上调；位域要写明存储单元宽度（§1.3）；字符串宏按用途选形（`&CStr` 常量 / `macro_rules!`，§2.1）；头内联函数用 `bash spikes/inline-probe.sh` 与 C 对拍（A6 实测抓到 `multibyte_length` 的 `<<` 优先级使 5 字节形态全错，这是 P-30 的第一块，三道静态闸门都看不见）。
17. **布局断言由工具生成，不手写**：`xtask layout-asserts` 读
   `docs/baseline/layout/c-layout-*.tsv`（C 编译器输出的冻结基线），生成 `const _: () =
   assert!(size_of/offset_of …);`；镜像里没有声明过的类型记为 pending，数量随移植递减，
   但不允许把 pending 当成通过。
18. **镜像里的宏必须先定义后使用**：`macro_rules!` 只在文本顺序之后可见，而三个 spike
    探针把镜像 `include!` 进一个 `mod`（§0.1-9 要求镜像位置无关），镜像内的函数体
    因此不能调用排在它后面的宏——`call0` 按 lisp.h 3462-3471 的顺序挪到了 `calln!`
    之后（P-57）。

### 0.2 命名与可见性

- 函数、宏、结构体、联合、枚举、枚举成员、全局变量、静态变量、subr：**名字逐字相同**
  （`Fcons`、`Scons`、`Qnil`、`Vload_path`、`GC_MARK_STACK`、`DEFUN`、`AUTO_STRING`…）。
- 与 Rust 关键字冲突时用原生标识符：`r#const`、`r#type`、`r#match`、`r#where`、`r#fn`、
  `r#use`、`r#mod`、`r#final`。参照树中实际冲突（抽样）：`const` 96、`type` 31、
  `match` 9、`where` 5、`fn` 5、`use` 2、`mod` 2、`final` 2。
- **声明 ≠ 定义**：C 的 `extern` 声明 → `unsafe extern "C" { … }`（头镜像内）；C 的定义
  → `#[unsafe(no_mangle)] pub`（唯一单元内；edition 2024 要求 unsafe 属性写法，裸
  `#[no_mangle]` 直接报 "unsafe attribute used without unsafe"）。`static`（内部链接）→
  非 `pub`、不导出符号。
- **`DEFUN` 的文件级记录 `Sfoo`**：C 是 `static union Aligned_Lisp_Subr sname`（内部链接）。
  Rust 侧是同名 `static mut Sfoo`，但 Rust 的静态符号会被 mangle（`_ZN…Sfoo`），
  C 里叫 `Sfoo`（局部符号）。记录为偏离：源码里的名字、`defsubr (&raw mut Sfoo)` 的
  引用与 C 逐字相同，符号表里的名字不同（参照里没有按名字查找 `S*` 的代码）。
- C 的全局变量（尤其 `Lisp_Object` 全局）在 Rust 侧写成 `#[unsafe(no_mangle)] pub static mut NAME: T`
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
| 具值常量（对象宏 + 枚举成员） | 3,898 | 同上 |
| 结构体/联合字段 | 5,764 | `LIFETIMES.tsv` |
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
  **落地约定**（`Lisp_Marker`、`itree_node` 已用）：同一存储单元里连续的位域折成一个
  整数字段，名字用 `bits`（与后一字段之间需要凑偏移时才加 `_pad`），字段上方用
  `///` 逐条列出 C 的位域声明；**打包是否正确由 C 编译器的偏移/`sizeof` 断言证明**
  （§0.1-16）。
- 变长结构体（`FLEXIBLE_ARRAY_MEMBER` 11 处）→ `data: [u8; 0]` + `offset_of!`。

### 1.4 函数指针与类型双关

- 带 context 的函数指针逐字对应 `unsafe extern "C" fn(*mut c_void) -> …`，字段名与签名
  相同；**不得改成闭包**（会改变布局与调用约定）。
- `*(T *) p`、union 换成员读、`memcpy` 到别类型 → `ptr::read_unaligned`/`write_unaligned`
  或 union 读，保持未对齐访问语义。
- **C 的函数指针参数只有一种 ABI，移植后也只有一种**：库函数（libm 等）在 Rust 侧是
  `unsafe extern "C" fn (…)`，同一参数位置上传的本单元 `static` 助手
  （`floatfns.c` 的 `ceiling2`/`floor2`/`round2`/`truncate2`/`identity`/`rounddiv_q`）
  也必须声明成 `unsafe extern "C" fn`，否则类型不同、`rounding_driver` 无法同时接受两者
  （`port(floatfns.c)` 实测：混用 `unsafe fn` 报 `E0308 expected "C" fn, found "Rust" fn`）。

### 1.5 C 的未初始化局部变量

- C 里"声明后立刻由被调函数写入"的局部量（`double f1, f2;` 配 `frexp` 的 out 参数、
  `int exponent;`、`EMACS_INT value;` 这类）在 Rust 必须有初值才能取得地址：统一写成
  零初始化（`let mut value: EMACS_INT = 0;`）并在 ledger 里点名；代价是
  "value assigned is never read" 一类 warning（`gate-fast` 不看 warning）。
- `void *p; f (&p);` 同理；`let p: *const T;` 只在**每个分支都赋值**时才合法
  （`floatfns.c` 的 `rescale_for_division` 就是这种形状）。

---

## 2. 控制流与表达式

### 2.1 宏 → `macro_rules!`

- 带代码的 `#define` → 同名 `macro_rules!`；`do { … } while (0)` → 块表达式。
- **实参求值次数必须与 C 相同**：多数宏可先 `let` 绑定一次，但 C 里存在**多次求值**的宏
  ——`AVAIL_ALLOCA(size) ((sa_avail -= (size)), alloca (size))`、`SAFE_ALLOCA`（对外层
  `size` 两次）、以及 release 下 `eassert`（**完全不求值**）。这些是“单次求值”规则的
  例外，必须逐条保留原求值次数（P-16）。
- 变参宏（`__VA_ARGS__`）→ `($($args:tt)*)`。
- **C 的字符串宏** → 同名 `macro_rules!` 或 `pub const …: &str`，两者都保持**内容字节**
  （比较时忽略 C 的隐式 NUL：`"t"`、`b"t\\0"`、`c"t"` 视为同一串）。用在 C 的相邻字面量
  拼接处（`"%"pI"d"`）时必须是宏：`concat! ("%", pI! (), "d")` 实测可行（`macro_rules!`
  的调用会在 `concat!` 里展开，而 `const` 不行）；`sizeof uLSQM - 1` 这类用法在使用点
  写成 `.len ()`。
- **宏里需要"实参个数"时不能用求值表达式**：数组长度是常量表达式，C 的 `ARRAYELTS`
  换成 `{ let _ = &$arg; 1 }` 会以 `E0435`（非常量值用于常量）失败，必须用
  `{ let _ = stringify!($arg); 1 }` 计数、由数组初始化真正求值一次（A6 实测）。

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

- **P-04（2026-10-02 定案）**：本机 stable 1.98.1 定义 C 变参函数报
  `E0658: C-variadic functions are unstable`，而两个 dated nightly
  （`nightly-2026-09-14` = 1.100.0-nightly、`nightly-2026-10-01` = 1.101.0-nightly）
  都能在**没有** `#![feature]` 的情况下定义它（`c_variadic` 自 1.99.0 起 stable，两者
  都在其之后；本段初稿所写“09-14 仍报 unstable”是错的，评审 A 的 F1 与评审 D 的 F6
  各自实测推翻了它）。结论：
  1. `femacs/rust-toolchain.toml` 锁**具体** dated nightly，当前 `nightly-2026-10-01`
     ——选它是因为本轮全部闸门证据都在它上面测，不是因为它比 09-14 更“能编”；
  2. 变参 API 名（`VaList::next_arg`/`clone`）与平台模型在升级时用
     `spikes/va-probe.sh` 复测；
  3. stable ≥ 1.99 发布后切回 stable 并删除该偏离记录；
  4. `#![feature]` 无论如何不能用：镜像单元是被 `include!` 的，内部属性非法。
  映射与证据：`docs/spikes/2026-10-02-variadics.md`；pin 下的 Miri 证据：
  `spikes/miri-probe.sh`（需要 `rustup component add miri rust-src --toolchain <pin>`，
  已对本 pin 安装，`miri-probe: clean (nightly-2026-10-01)`）。
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
- `static_assert`（C23 关键字）→ `lisp.rs` 里的同名 `macro_rules!`，展开成
  `const _: () = assert!(cond);`，调用点写成 `static_assert! (cond);`（A6 实测）。

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

### 5.1 `DEFUN` → `DEFUN!`（1,841 个活跃 subr，v2 更正；宏名与 C 逐字相同，PORTING 0.2）

C：

```c
DEFUN ("cons", Fcons, Scons, 2, 2, 0,
       doc: /* Create a new cons, give it CAR and CDR as components, and return it.  */)
```

Rust（**docstring 必须是字符串字面量**，v1 用注释的写法没有机械保障——评审实测
`stringify!` 得到 `doc : ,`）：

```rust
DEFUN!("cons", Fcons, Scons, 2, 2, 0,
       doc: "Create a new cons, give it CAR and CDR as components, and return it.",
       (car: Lisp_Object, cdr: Lisp_Object),
       {
           return CALLN! (Fcons, car, cdr);
       });
```

（宏调用**不能被紧跟的项**续写，所以函数体是宏的最后一个实参；字段顺序与 C 相同，
`attributes: noreturn` 写在 `doc:` 之后，`maxargs` 决定联合成员 `.a<maxargs>`，
`defsubr (&raw mut Scons)` 在 `syms_of_*` 里注册——C 的 `defsubr` 会写记录的
`header.size`，所以指针是 `*mut`。）

**`attributes: noreturn` 的记录**：C 把 `_Noreturn` 只写在**声明**上（`globals.h` 的
`_Noreturn EXFUN (Ffoo, N)`），定义仍是 `Lisp_Object fnname (…)`，而 `_Noreturn` 不是
类型的一部分，所以 `{.a2 = Fthrow}` 在 C 里没有类型冲突。Rust 的 `-> !` **是**类型的一部分，
`fn(…) -> !` 不能存进 `fn(…) -> Lisp_Object` 的联合成员（`E0308`）。因此：
- 生成的 `subrs` 原型按 C 的 `_Noreturn` 写 `-> !`；
- `DEFUN!` 的定义也写 `-> !`（与原型同型，P-28 才可校验）；
- **记录**里的指针用 `subr_function_pointer!` 打一次形（等价于 C 的联合初值
  `{.a ## maxargs = fnname}`；两侧都是同宽函数指针且函数从不返回，SAFETY 注在宏上）。
  A6 实测：noreturn 记录在 bin/lib 两种 crate 里都能编译，且记录里存的地址与函数地址
  相同（`address_preserved=true`）。

**多行 interactive 规格的坑**：C 的 `\`+换行是**行拼接**，只删掉换行、保留续行的
缩进；Rust 字符串里的 `\`+换行是**行延续**，会把续行的前导空白一起吃掉。所以把 C 的
多行 `intspec` 原样抄进 Rust 会**改变编译后的字符串值**。规则：多行 `intspec` 在 Rust
里写成一行，用 `\t` 与显式空格把 C 拼接后的值原样表达；`--check` 现在按**解码后的
值**逐字节比较，抄布局会直接 FAIL。

**docstring 校验的粒度**：`--check` 比对的是**词级**摘要（C 的注释排版与 Rust 的字面量
排版不同，空白串会被折叠）；`doc:` 字面量与 C 注释**逐字节一致**的保证由 §7.3 的
`etc/DOC` 逐字节比对在阶段 G 兑现。

- 参数上下限、interactive 规格、Lisp 名逐字一致；`--check` 比对 docstring 摘要（
  `docs/baseline/subrs.tsv` 的 `ldoc_digest`）。
- 若某文件的 `make-docfile` 需要从注释抽取，则必须同时满足“注释与字符串字面量逐字一致”，
  并由 `--check` 机械校验（双轨契约）。
- `struct Lisp_Subr` 布局与 `lisp.h` 一致（含函数指针 union、参数上下限、docstring 指针）。

### 5.2 注册与符号

- `syms_of_*`/`keys_of_*` 同名、`#[unsafe(no_mangle)]`，内部调用顺序与 C 完全一致（顺序影响
  `Qnil`/`Qt` 初始化与 dump 内容）；`defsubr` 1,848 处、`DEFSYM` 2,310 调用逐处保留。
- **`defsubr` 的调用形状**：C 的 `defsubr (union Aligned_Lisp_Subr *)` 会写记录的
  `header.size`（`XSETPVECTYPE`），镜像因此声明为 `*mut`，调用点写
  `defsubr (&raw mut Sfoo);`（`foo` 的定义文件里，顺序与 C 的 `syms_of_*` 相同）。
- **`EXFUN` 的前后缀**：`attributes: noreturn`（8 个 subr，`docs/baseline/subrs.tsv` 的
  `flags` 列）→ 声明为 `-> !`（C 写 `_Noreturn Lisp_Object Ffoo (…)` 并加后缀
  `ATTRIBUTE_COLD`；Rust 无法同时表达"返回 Lisp_Object"与"从不返回"，取
  `-> !`，`ATTRIBUTE_COLD` 只影响优化）。同一个 subr 名可以出现在多个配置分支里
  （1,841 条记录 → 1,736 个名字），生成器按名字合并：签名不同即**报错退出**（照
  `make-docfile` 的 error），`flags` 取并集。
  `attributes: const`（15）/`pure`（6）/`noinline`/`ATTRIBUTE_COLD` 在 Rust 的 extern
  声明上无对应物，只影响优化，逐个记录为不表达（P-25）。
- **`DEFUN!` 定义侧同样要遵守 `noreturn`（评审 F8，§5.1）**：`Fsignal`/`Fthrow` 等 8 个
  subr 的定义必须是 `-> !`，否则定义与 `EXFUN` 原型不一致（Rust 不再检查，见 §0.1-11）。

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
| P-04 | 变参：`c_variadic`（stable ≥1.99）+ 锁具体 dated nightly | **已定**（2026-10-02，pin `nightly-2026-10-01`） | `doprnt.c` 试跑 + `spikes/va-probe.sh` 6 行 + `docs/ports/va-arg-types.tsv` 的多重集检查（P-45）+ `gate-probes` |
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
| P-18 | `flush_stack_call_func` 的寄存器冲刷用 `_setjmp` 屏障替代 `__builtin_unwind_init` | 已定 | `lisp.rs::flush_stack_barrier`（待 `A7-③` 复核） |
| P-19 | `specpdl`/`specpdl_ptr`/`specpdl_end` 在 C 是线程局部宏；镜像先声明为 `extern` 存储 | 待办 | thread.rs 落地后改为 `current_thread` 访问 |
| P-20 | `get_act_rec`/`set_act_rec` 需要 `current_thread`（thread.h） | 待办 | thread.rs 落地后补齐；其余函数已全部落地 |
| P-21 | `DEFVAR_*` 的变量在 C 里是 `globals.f_V*`（为调试器聚合）；镜像把它们放进 `pub mod variables`，写成 `unsafe extern "C" { pub static mut V…: T; }` 并 `use variables::*;`，由**属主单元**用 `#[unsafe(no_mangle)] pub static mut V…` 定义（定义遮蔽整体导入，与 `DEFUN` 对原型的关系相同，§0.1-10） | 已定（评审 A 的 F4） | `xtask gen-globals` + `docs/baseline/globals/variables.tsv`；实测：定义方不再 `E0428`，符号是真正的 `_Vprintable_chars`。**不许**在 globals.rs 里就地定义：那会给每个 TU 一份 NULL 副本，第一个内联读者（如 `CHAR_PRINTABLE_P` 读 `Vprintable_chars`）就会解引用空指针 |
| P-22 | `FOR_EACH_TAIL*`/`DOHASH*`/`DOOBARRAY` 是循环宏，Rust 形态（迭代器/闭包）尚待决策 | 待办 | `docs/ports/excluded-macros.txt`（kind=deferred），使用这些宏的文件暂不开工 |
| P-23 | GDB 符号技巧（一个名字既是宏又是常量符号）在 Rust 无对应物 | 已定 | `docs/ports/excluded-macros.txt`（kind=debugger）；`xtask` 后续另行产出调试符号 |
| P-24 | 检查器对头镜像的判定规则 | 已定 | 头文件不做行数漂移判定；结构体字段按**成员集合**比对（顺序由布局断言负责）；C 位域跳过；条件分支中的常量与宏别名不比对；**字符串常量按解码后的字节比对**（C 的 `"\xE2\x80\x98"` 与 Rust 的 `"\u{2018}"` 是同一串）；排除/挂账项见 `docs/ports/excluded-macros.txt` |
| P-25 | `EXFUN` 的 `const`/`pure`/`noinline` 前缀无 Rust 对应物 | 已定（偏离已记录） | 只影响优化；`noreturn` 已映射为 `-> !`（§5.2） |
| P-26 | gnulib/系统头（`lib/stdint.in.h`、`lib/limits.in.h`、`lib/intprops.h`、`lib/verify.h`…）的镜像 | **部分完成**：`stdint.in.rs`/`limits.in.rs` 只含宽度一节（WIP） | 逐头文件补全并让 WIP 计数下降（§7.3） |
| P-27 | 检查器只跟随一层 `include!` | 已定 | 名字（函数/宏/静态量/常量/subr）仍按"文件 + 一层镜像"收集；**桩检测看全部被包含文本**（否则可把实现放进被包含文件绕过），5% 行数预算只看文件自身；桩检测跳过 C 文件里没有的名字（镜像内联函数） |
| P-28 | subr 的"原型 ↔ 定义"检查缺失 | **已完成**（`extract-abi.py`：`DEFUN!` 参数个数 vs `maxargs`、`attributes: noreturn` ⟺ C 的 flags、crate 根不许声明 subr 原型） | 合成用例 + `--check` 回归 |
| P-29 | `globals.rs` 的原型集是"全配置超集"（含 304 个仅 GUI/平台文件才有的名字），与 C 按 `GLOBAL_SOURCES` 生成的 `globals.h` 不同 | 已记录（偏离） | C 里调用未配置的 subr 是编译错误，Rust 里会到链接期才暴露；待 `xtask` 冻结每目标的源文件清单后按配置过滤（P-08 完成后） |
| P-28 | subr 的"原型 ↔ 定义"检查缺失 | **已完成**（`extract-abi.py`：注释无关的匹配、`DEFUN!` 参数**类型** vs `DEFUN_ARGS_*`、`attributes: noreturn` ⟺ C 的 flags、crate 根不许声明 subr 原型、`defsubr` 集合逐条对齐、`DEFUN!` 体也做桩检测、`maxargs` 不可解析时报错；`gate-fast` 现在还编译每个**已完成**的 TU） | 评审 A/B 的 40+ 个攻击用例全部按预期通过/拒绝；残余：doc 摘要为词级（字节级靠阶段 G 的 `etc/DOC`） |
| P-31 | 有 5 个 `S…` 记录（`Sfont_drive_otf`、`Sfont_otf_alternates`、`Sdraw_string`、`Sframe_configuration`、`Srestore_frame_configuration`）由 `font.c`/`frame.c` 注册，但定义在 `.m` 一侧；另有 1 个 DEFUN（`minibuf.c` 的 `Fread_function`）参照从不注册 | 已记录 | `.m` 一侧的 Swift 实现要导出这 5 个记录（或由 Swift 侧注册）；`Fread_function` 照参照不注册；`--check` 按参照文件自己的 `defsubr` 集合对齐，两条都自然覆盖 |
| P-33 | `ITREE_FOREACH` / `ITREE_FOREACH_NARROW`：C 在**展开点**声明迭代器，`_NARROW` 再引用它 | **待定（阻断 `buffer.c` 的 17 处；另 4 个文件各 1–2 处）** | 评审 A 已实测两种可行拼法（body 作为最后一个实参 + `while let Some (n) = it.next ()`，`_NARROW` 变成迭代器方法/宏；或适配器 + `for`）。真正待决的是两处语义：(a) `ITREE_##order` 的 token 粘贴在 `macro_rules!` 无对应物（需 4 臂枚举）；(b) C 每个循环**求值 `t` 两次**（头文件自己的 BEWARE），两种 Rust 拼法都只求值一次（当前调用点都是纯表达式，记为偏离）。代价：22 处调用（`buffer.c` 15+2、`xdisp.c` 2、`editfns.c`/`itree.c`/`textprop.c` 各 1）；`casetab.c` 不受阻 |
| P-34 | 布局基线只覆盖手挑的键（现 207 条，A6 起由 `tools/gen-layout-fields.py` 按"镜像结构体的每个非位域字段"生成） | 进行中（有下限闸门） | 每加一个镜像就在 `struct` 列表里加一项并 `bash spikes/run.sh` 重生成（数字永远来自 C 编译器）；`docs/baseline/layout/counts.tsv` 冻结 `asserted/pending` 下限，类型被改名/删除会让断言数下降并**直接失败**（评审 B 的 F1：类型扫描改为去注释/字符串后按 `\s+` 匹配，`inline-probe` 与 `git` 都能复现） |
| P-35 | 位域的**存储单元宽度**不在闸门覆盖内（`offsetof` 不能指名位域；`u8`/`u32`/`u64` 都让邻居偏移与 `sizeof` 保持 80） | 已记录 | 每个镜像在字段注释里写明单元宽度（`itree_node.bits: u8`，clang 记录布局 72:0-72:2）；`lisp.rs` 现存的 `bits: u32 (+_pad)` 在对应 `.c` 落地时逐个核对（评审 A 的 F2） |
| P-36 | 字符串常量：`cinventory` 记录前就折叠了空白，`"a  b"` 与 `"a b"` 会判等；`enum char_bits`（`lisp.h` 3218-3235）还没进 `lisp.rs` | 待办 | 给 `inventory` 增加"原始值"列并在比较时使用；`CHAR_MODIFIER_MASK` 等在 `character.c` 落地前补齐（评审 B 的 F6 / 评审 A 的 F6） |
| P-32 | arity 表在 `lisp.rs` 里出现两次（`DEFUN_ARGS_*` 镜像宏、`DEFUN!` 的 11 个 arity 臂），`xtask` 生成原型时是第三次 | 已记录（待合并） | 三处可归并成一个来源；当前由 P-28 的类型检查与 `DEFUN_ARGS_*` 清单兜底 |
| P-30 | 现有三道闸门**看不见行为错误**（评审 A 的 P0：`XSYMBOL` 少加 `lispsym` 基址，`--check`/`build-tu`/`layout-check` 全绿却会段错误） | **待办（阶段 B 前）** | 用差分对照把核心算法 TU 与参照对拍（评审 A 的 `/tmp/dt` 手法：C 侧 `cc -std=gnu2x` + mock 运行时，Rust 侧 `rustc --crate-type bin`，同一语料逐字节比对 stdout/标记轨迹；语料覆盖 n=1…5000、6 种键型、`values` 两态、`reverse` 两态、中途 signal/longjmp 与 GC 标记） |
| P-37 | C 的宏/函数**种类**必须保持：对象式宏（`BEGV`/`GPT`/`Z`/`MODIFF`/`FETCH_BYTE (n)`…）在 C 里是**左值**（参照有 159 处写入、11 处 `&MODIFF`、1 处 `FETCH_BYTE (pos) =`），写成 `fn` 就再也表达不出这些语句 | 已定（形状见 `docs/ports/include-buffer.md`） | **C 宏 → 同名 `macro_rules!`**（宏体就是 C 的宏体，因此是 place 表达式），**C `INLINE` → `#[inline(always)] fn`**；代价是调用点写 `NAME! ()` 而 C 写 `NAME`，`PT`/`PT_BYTE` 保留 `+ 0` 以保持"不是左值"。`macro_rules!` 无 token 粘贴：`BVAR`/`PER_BUFFER_VAR_OFFSET` 的调用点写完整成员名（C 名 + `_`）；`GET_OVERLAYS_AT` 把宿主函数的 `sa_avail` 作为第一个实参；`FOR_EACH_PER_BUFFER_OBJECT_AT` 把循环体作为最后一个实参（同 `DEFUN!` 的形状）。 证据：`spikes/write-probe.sh`（C/Rust 两侧同一串写入/取址语句，4 行逐字节一致） |
| P-38 | 头/单元镜像的闸门盲区（2026-10-02 三轮对抗评审共实测 16+13+23 个变异体：冻结闸门对 `casetab.rs` 的语句级语义**零覆盖**） | **主要部分已完成（2026-10-02 第三轮）** | `tools/extract-abi.py` 现在比对：① 常量（含一行式 `enum { … };`，并报 `missing constant`）——当场在 `lisp.h` 找出 8 个从未镜像的常量；② **宏体与函数体的 token 包含**（标识符+数字，折叠大小写/整型后缀/字符串字面量/尾随 `_`/`NULL` 惯用法/本文件类型名，排除 C 形参名与短局部名），刻意不同者记 `docs/ports/macro-body-notes.tsv`（支持 glob 与 `*`）；③ 桩检测改成花括号配对（`{ { unimplemented!() } }` 现在报错）；④ docstring 按 **make-docfile 的值**比对（见 5.1）。实测：`--check` 抓得到评审 C 的 MA/MD/MJ（函数体）与 M6/M9/M10（宏体）、M11b/M14（常量）、M7（嵌套块桩）、docstring 词级改动；`gate-fast` 现在还编译 `--cfg enable_checking` 的一份（`eassert!` 不再被静默丢弃）并跑 `fam/shadow/macro` 三个探针。**仍未覆盖**：同一 token 出现多处、只改一处（存在性而非多重性）、字符字面量（`'z'`→`'y'`）、纯顺序调换（两次 `Faset` 对调、`Qt`/`Qnil` 对调）——需要计数/顺序规则，与 P-30 的行为差分互补 |
| P-39 | `lisp.rs` crate 根的 `unsafe extern "C" { pub fn … }`（74 条，约 62 条是 Emacs C 函数：`make_string`/`intern_1`/`defsubr`/`marker_position`…）在**定义它们的单元**（`alloc.c`、`marker.c`…）落地时会 `E0428`——与历史 `Fcons`/`Ffuncall`/`Fsignal` 同型；`--check` 的 P-28 规则只覆盖 subr | 已记录（落地前必须做） | 把这些声明移进一个 `pub mod`（如 `pub mod c_functions { … }`）再 `use c_functions::*;`，让单元里的定义遮蔽整体导入（P-21/P-28 已验证的形状）；同一文件里再加一条闸门规则，禁止 crate 根出现"将由别的单元定义"的 `pub fn` 原型 |
| P-40 | C 的**柔性数组成员**（`Lisp_Vector.contents`、`Lisp_Char_Table.contents`/`extras`、`Lisp_Sub_Char_Table.contents`、`Lisp_Bool_Vector.data`）在镜像里声明为 `[T; 0]`（这样 `sizeof` 与后续字段偏移与 C 一致），但 Rust 的 0 长数组**不能索引**：`arr[i]` 是边界检查，debug/release 都 panic（"the len is 0 but the index is 0"） | 已定（缺陷已修） | `FAM_ELEM! (place, idx)`：按地址取元素，且展开为 **place**，所以 C 的 `x->contents[i]`、`&x->contents[i]`、`x->contents[i] = v` 三种形状都能一一对应；证据 `spikes/fam-probe.sh`（四个 C 名访问器 + 两个 FAM，4 行；负对照：换回旧写法立刻 panic）。**这七个访问器（`AREF`/`ASET`/`aref_addr`/`gc_aset`/`CHAR_TABLE_REF_ASCII`/`set_char_table_extras`/`set_char_table_contents`）此前一直带着这个 P0**——`--check`/`layout-check` 只看文本与布局，`gate-full` 未实现，所以直到 `casetab.c` 成为第一个真正读 `extras` 的单元才暴露 |
| P-41 | `Q*` 的**索引顺序**与 C 不同（P-17 已记录），但没人记录它的后果：`lread.c` 用 `defsym_name[i]` （**make-docfile 生成进 `globals.h` 的表**）给第 i 个内建符号命名。冻结配置里 C 有 1,560 个符号（`iQcase_table` = 339），port 有 2,118 个（`iQcase_table` = 491），集合包含但顺序不同（评审 A 实测：C 的 1,560 全在 port 的集合里，索引逐条一致） | **已记录（`lread.rs` 落地前必须做）** | `lread.rs` 必须按 **port 自己的顺序**重新生成 `defsym_name` 表，否则每个内建符号会被装上别的名字（静默的全局性故障）；`countof (lispsym)` 的变化也会让 `alloc.c` 的内存计数与参照不同；当前 `defsym_name` 在仓库里 0 命中 |
| P-42 | **构建条件文件的离场边界**：`gmalloc.c`/`ralloc.c`/`vm-limit.c` 在冻结配置下不参与链接（`SYSTEM_MALLOC=1`、`REL_ALLOC` undef → `src/Makefile.in` 489-495 的三个 `*_OBJ` 皆空），而 `atimer.c` **在** `base_obj` 里（`src/Makefile.in:461`；此前“清单里没有”的说法是错的） | **范围已定** | 1:1 以**参照树的文件**为准，不以冻结配置的链接清单为准（`docs/port-plan.md` 第 8 行“每一个 `*.c`/`*.h`”）：这三个文件照样移植，验证只到 `--check` + 自身探针；运行时证据要等一个非 `SYSTEM_MALLOC` 的配置被冻结（P-08） |
| P-43 | 同一仓库三种 lint 写法：`doprnt.rs` 允许 `unsafe_op_in_unsafe_fn`，`sort.rs`/`casetab.rs` 不允许 → 同一状态下 438 条 vs 6 条 warning | **已定**：每个 `femacs/src/**/*.rs` 顶部同一组 `#![allow(…)]`，含 `unsafe_op_in_unsafe_fn` | C 的语句本来就“处处 unsafe”，逐句加 `unsafe {}` 只造噪声；`gate-fast` 不看 warning，噪声正是“新 lint 类无人发现”的来源（评审 D F12）。复核方式：三处 allow 列表 `grep` 一致 |
| P-44 | 同义 token 家族此前用“豁免”处理（`macro-body-notes.tsv` 允许 `memcpy`/`mempcpy` 缺席），而 token 检查是**单向包含**：把 `doprnt` 的三处输出拷贝**全删掉**，`--check` 与 `build-tu` 都绿。第二轮评审（B2）又证明只看**出现次数**不够：`overflowing_add (0)` 改成 `(1)` 仍绿 | **已定并实现**：`docs/ports/token-aliases.tsv` + `extract-abi.py` 的 `alias_count_problems`——家族在两侧折叠为同一拼写，并要求 **Rust 侧出现次数 ≥ C 侧** | 评审 D F2。实测：M2（删 1 处）/M3（删 2 处）/M8（删 3 处）/M1（`rpl_mempcpy`→`strcpy`）全部 `FAIL`，阴性对照（原样）`OK` |
| P-45 | `va_arg (ap, T)` 的**读取类型**此前无任何守卫：`%u` 的 `c_uint` 改成 `c_int` 能编译、所有闸门绿（值流进变参 `sprintf`，编译期看不见） | **已定并实现**：`docs/ports/va-arg-types.tsv` + `extract-abi.py` 的 `va_arg_problems`——按**类型多重集**比对，未记录的类型直接报错 | 评审 D F4。实测变异 MV1（`c_uint`→`c_int`）现在 `FAIL`；`doprnt` 两侧各 12 处读取逐项对应 |
| P-46 | `spikes/shim/stdckdint.h` 的 no-op 宏**影子掉**了系统真实实现（本机 `HAVE_STDCKDINT_H=1`），任何按探针命令构建的 C 参照都会静默失去溢出检测（`%99999999999999999999d` 回绕而不是 `error`） | **已定并实现**：shim 只留 `HAVE_*_H` 为 undefined 的头（现仅 `stdbit.h`），且探针一律 `-idirafter spikes/shim` | 评审 A F3；`spikes/shim/README.md` 写明规则与代价 |
| P-47 | 闸门缺“探针可跑性”这一层：`ec20c48` 让三条探针在仓库根跑裸 `rustc`（解析到 stable）而全部闸门仍然绿 | **已定并实现**：`gate-fast` 增加“探针程序的 Rust 侧编译”；新增 `xtask gate-probes` 跑四个 C 差分脚本，`build/emacs-ref` 缺失时**报错**而不是跳过 | 评审 D F1。实测 `gate-probes`：`inline 6081` / `accessor 11` / `write 4` / `va 6` 行逐字节一致 |
| P-48 | **残余**：端口**自造**的文件级常量只被“记录的偏离”覆盖——`doprnt.rs` 的 `TBUF_LEN = SIZE_BOUND_EXTRA + 50` 改成 `+ 500` 仍然全绿（`50` 被 `macro-body-notes.tsv` 豁免，而 `TBUF_LEN` 不是 C 名，没有东西能比它） | **已记录（批次 B 之前做）** | 建“自造常量 ↔ C 表达式”表 + token 包含检查（与 P-44/P-45 同型）；批次 B 起这类常量会变多（`floatfns.c` 的 `signbit`/`emacs_rint`、`bignum` 的 `GMP_NUMB_BITS`），落地时一并做；在那之前 ledger 必须逐条点名自造常量（评审 A F6） |
| P-49 | `<gmp.h>`（= gnulib `lib/gmp.h` → `lib/mini-gmp.h`）没有镜像，而 `bignum.h`、`floatfns.c` 都要 `mpz_t`：`lib/` 属批次 F（`mini-gmp.c` 4,699 行） | **已定（临时位置已记录）** | 该头现已整体镜像为 `femacs/lib/include/mini-gmp.rs`（292 行：类型 + 4 个宏 + 159 条声明，声明放在 `pub mod gmp` 里以便 `bignum.c` 的定义遮蔽它们），由 `spikes/gmp-abi-probe.sh` 从镜像自身反向生成 C harness 做 ABI 类型核对（159 条全过，改错参数宽度立刻失败），每条上面引 C 行；（批次 F 再落 `lib/mini-gmp.c` 的 4,699 行定义。）`mpz_t` 保持 C 的数组 typedef：`pub type mpz_t = [__mpz_struct; 1]`，因此 `sizeof (mpz_t)`=16、`mpz[5]`=80、`Lisp_Bignum`=24、`value` 偏移 8 全部由布局闸门（211 条断言，`spikes/run.sh` 报 `46 shared, 0 mismatch`）钉住——`sizeof mpz` 这条键的根是**对象**不是类型，`xtask layout-asserts` 因此学会从镜像自己的声明取类型（`static mut mpz: [mpz_t; 5]`），评审 B 的 B1 用它证明过 `[mpz_t; 6]` 在此之前全绿；`mpz_t` 形参在 C 里退化为指针，故声明用 `*mut/*const __mpz_struct`，实参写 `…as_mut_ptr ()/…as_ptr ()` |
| P-50 | `<math.h>`/`<limits.h>` 也没有镜像，`floatfns.c` 需要 22 个 libm 函数、`isnan/isfinite/isinf/signbit` 四个宏、`INT_MIN/INT_MAX`、`HUGE_VAL/FP_ILOGB0`，以及 `DBL_MANT_DIG/DBL_MIN_EXP` | **已定（系统镜像已建）** | 新增 `femacs/lib/include/math.rs`（libm 声明 + 四个宏 + 两个常量），`limits.in.rs` 补 `INT_MIN/INT_MAX`，`float.in.rs` 补 `DBL_MANT_DIG=53/DBL_MIN_EXP=-1021`；常量全部取自冻结目标 clang 实测（命令写在文件里）。`signbit` 取**系统语义**（`is_sign_negative`，含 NaN 的符号位），`floatfns.c` 自己的 `#ifndef signbit` 回到 `feature = "no_signbit"` 的死文本——两者对 NaN 结论相反，取系统语义才是参照二进制的行为 |
| P-51 | 死分支的**文本**也要在镜像里（P-08 只决定哪支是活的）：`emacs_rint`/`trunc` 的 `#else`、`double_integer_scale` 的 `#ifdef HAIKU`、`signbit` 的 `#ifndef` | **已定** | 死文本用 `feature = "no_rint"`/`"no_trunc"`/`"haiku"`/`"no_signbit"` 包起来（特性名命名**非冻结**的一侧，默认即冻结配置），每条配 `const _: () = assert! (!cfg! (…) || …)` 绑住冻结常量；`--check` 因此能看到死分支的 token，评审也读得到 C 的原文。**第二轮评审（B3/F2）证明“保留文本”不等于“能编译”**（`no_signbit` 那支把 C 的 `!(x)` 写成 f64 的按位 `!`，根本编不过，且没有断言）：新增 `xtask dead-arms`——把 `femacs/` 复制到 `build/dead-arms/`、翻转对应的冻结常量、开对应 feature，逐个编译已完结的单元（现为 4 个），并由 `gate-probes` 调用 |
| P-52 | C 的**非 `static` 定义就是导出符号**（PORTING.md 0.2），而 `floatfns.c` 的 `extract_float`/`fmod_float`/`double_integer_scale` 被移植成了普通 `pub fn`：`--check`/`build-tu`/`layout-check` 全绿，但目标文件里没有 `_extract_float` 等符号，下一个调用它们的单元（`data.c`/`timefns.c`/`indent.c`/`dispnew.c`/`dbusbind.c`）一链接就失败 | **已定并已修** | 三处（以及后来由新规则抓出的 `syms_of_floatfns`、死分支的 `trunc`）改成 `#[unsafe(no_mangle)] pub unsafe extern "C" fn`，并在 `lisp.rs` 按 lisp.h 补了三条声明（P-39 形状）。`extract-abi.py` 新增一条闸门规则：C 里非 `static`、非 `INLINE`、非 subr 的定义，Rust 侧必须在 `fn NAME` 前有 `#[unsafe(no_mangle)]`（头文件与 `DEFUN!` 生成的除外）；评审 A 的 F1 用 `nm` + `clang -r` 部分链接证明，修后 `nm` 能看到全部五个符号 |
| P-53 | 布局闸门看不见**lib 镜像里的类型**：`known_types()`/`declared_object_types()` 扫的是 `../lib/include`，而 port 的 lib 镜像在 `femacs/lib/include`（从 `femacs/` 看是 `lib/include`）。`mini-gmp.rs` 一落地，`__mpz_struct`/`mpz_t` 的 6 条键立刻从 asserted 掉进 pending（205/13，被 ratchet 挡下） | **已修** | 扫描根同时包含 `lib/include` 与 `../lib/include`；另外 `layout-asserts` 现在**先写 pending 列表再查 ratchet**，失败时也能看到是哪几条键回归了。实测：修复后回到 211 asserted / 7 pending |
| P-54 | `lisp.rs` 的 `other_units` 声明**只比名字不比类型**：`extract-abi.py` 看不见类型，`gen-gmp-abi-probe.py` 只看 mini-gmp，于是 `make_string`/`make_unibyte_string`/`intern_1`/`intern_c_string_1`/`fast_c_string_match_internal`/`egetenv_internal` 六条把 `ptrdiff_t` 写成 `usize`，另有 15 条与参照头类型不一致（评审 B 的 B-2） | **已清空基线（68 条受检）** | 新增 `tools/gen-lisp-abi-probe.py` + `spikes/lisp-abi-probe.sh`：把 `other_units` 的声明反向生成 C，用 `-Werror=incompatible-pointer-types` 对着参照 `lisp.h` 编译。第七轮把 15 条存量全部修掉，基线 `docs/ports/lisp-decl-abi-baseline.txt` **清零**：7 条是“Rust 类型名 = C tag”，生成器现在读 `lisp.h` 的 tag/typedef 表并补关键字（`union Aligned_Lisp_Subr *`、`struct Lisp_Fwd const *`、`const union vectorlike_header *`；typedef 优先——lisp.h 589 的 `struct Lisp_Object` 在未启用的 `CHECK_LISP_OBJECT_TYPE` 分支里）；`allocate_pseudovector` 的 `isize` 改成 `int`、`xnmalloc` 的 `usize` 改成 `ptrdiff_t`；`listn` 补回固定的 `Lisp_Object` 形参（`list!` 随之按 C 的两次求值展开）；`calln` 是宏不是函数（删掉发明的声明，`call0` 改用 `calln!`）；`fclose` 移到 `<stdio.h>` 镜像；两条 `struct buffer *` 用 `/// C:`。**覆盖也随之上升**：`allocate_nil_vector`/`allocate_vector`/`staticpro`/`mark_objects`/`record_unwind_protect_array` 原先被过宽的“含 `Lisp_…` 即不可映射”判据当成未检查，现在都在检查（68 受检 / 5 未映射=全是函数指针形参 / 2 跳过，跳过项现在连理由一起打印） |
| P-55 | 参照里**只以指针出现的不完整类型**（`struct buffer *`）在镜像里没有 Rust 对应物：镜像要能出现在不含 `buffer.h` 的编译单元里，而 Rust 的类型定义全局唯一（写 `pub enum buffer {}` 会与 `buffer.rs` 的 `struct buffer` 撞 `E0428`）。用 `*mut c_void` 又丢掉了类型信息 | **已定并实现** | 该声明上方写一行 `/// C: <参照原型>`（如 `/// C: ptrdiff_t buf_charpos_to_bytepos (struct buffer *, ptrdiff_t)`），`gen-lisp-abi-probe.py` 改按这一行生成 C 侧指针并核对 `lisp.h`：注释写错、参照改签名都会编译失败。Rust 侧 `*mut c_void` 与 C 的指针 ABI 相同，调用点（`buffer.rs`）照旧显式转换 |
| P-56 | `lisp.h` 38 行 `#include <stdio.h>`，5316-5318 在非 Android 配置下取 `# define emacs_fclose fclose`；`fclose` 与 `FILE` 此前被塞在 `lisp.rs` 和 `mini-gmp.rs` 里，而 `mini-gmp.rs` 并不拥有 `FILE` | **已定并实现** | 新增 `femacs/lib/include/stdio.rs`（`FILE` + 系统 `fclose`），由各编译单元按 C 的包含顺序 `config.rs → stdio.rs → lisp.rs` 包含（5 个单元 + 6 个 spike 探针 + `xtask` 生成的布局探针）；`mini-gmp.rs` 删掉重复声明（它本来就要求包含者先有 `FILE`）。实测（第八轮评审 B 的 F6 订正）：`REPLACE_FCLOSE` 不在生成的 `lib/stdio.h` 里，而在 `lib/gnulib.mk` 的 `= 0` 处，它让生成的 `lib/stdio.h` 落在字面量 `#if 0`（953 行）——`rpl_fclose` 与 `_GL_CXXALIAS_SYS` 两支都死，系统 `fclose` 来自 50 行的 `#include_next <stdio.h>`；结论不变（镜像的是系统 `fclose`），`lisp.h` 的 `#include <stdio.h>` 在 34 行不是 38 行（`emacs_fclose` 那两行 5315-5319 是对的） |
| P-57 | 镜像内的函数体调用**排在它后面**的宏，在 crate 根能编译、被 `include!` 进 `mod` 时不能：`call0` 用了 `calln!` 而 `macro_rules! calln` 在其后 570 行 | **已修** | `call0` 按 lisp.h 3462-3471 的顺序挪到 `calln!` 定义之后（两处都是 4 行，行数不变）。§0.1 新增第 18 条把“宏必须先于使用”写成规则；`inline`/`write`/`accessor` 三个探针本来就把镜像 放进 `mod`，因此是这条规则的守门人 |
| P-59 | `tools/cinventory.py` 的 `enum_constants` **只认 `enum [NAME] {` 在同一行**的形式，于是 lisp.h 的 `enum Lisp_Closure`（3206）与 `enum char_bits`（3219）——`{` 在下一行——连成员一起完全没进常量清单；切分成员时又只看逗号不看括号，`CHAR_TABLE_STANDARD_SLOTS` 的值被截断，`NLIMBS_LIMIT` 的嵌套 `min (…)` 里还凭空生成假成员 `lim`；函数内的枚举（doprnt.c 218-225 的 `pDlen`/`pIlen`/`pMlen`/`maxmlen`）也被当成文件级常量要清单。触发者是 `port(keymap.h)` 的 `CHARACTERBITS`（编译不过才发现常量根本不在镜子里） | **已修 + 25 条已补齐** | 扫描改成"注释/字符串剥离后的扁平文本 + 括号感知切分 + 只取 `{` 深度 0 的枚举"（函数内枚举的成员由所在函数体的 token 比对覆盖）。lisp.h 随即暴露 25 条：从未镜像的 14 条枚举成员（`CLOSURE_*` 6、`CHAR_ALT/SUPER/HYPER/SHIFT/CTL/META`、`CHAR_MODIFIER_MASK`、`CHARACTERBITS`）、`Cmp_Bit_EQ/LT/GT` 与 `Cmp_EQ/LT/GT`（C 的隐式值 0/1/2）+ `cmp_bits_t`、`header_size`/`bool_header_size`/`word_size` （C 是 enum 常量，port 是 `const fn`；已改成常量并把 8 处调用点的 `()` 去掉，写法与 C 一致）、`USE_STACK_CONS`/`USE_STACK_STRING`（硬编码 `true`，改成 C 的表达式）。**其中两条值原本是错的**：`SUB_CHAR_TABLE_OFFSET` = 2 而 C 是 1（`PSEUDOVECSIZE (struct Lisp_Sub_Char_Table, contents) - 1`，clang 实测 16 = 8 + 1×8 印证），`FIXNUM_BITS` = `INTTYPEBITS + 1` = **3** 而 C 是 `VALBITS + 1` = **62**：前者让 `set_sub_char_table_contents` 写错槽位，后者让 `bignum.c` 的 `bits <= FIXNUM_BITS` 几乎恒假（能装进 fixnum 的值走成 bignum）并让 `SXHASH_REDUCE` 的归约位宽错到 61。三个常量现在都由 C 的表达式推导（`PSEUDOVECSIZE!`），不再手写数字。根因见 P-60。`PSEUDOVECSIZE` 的宏定义按 P-57 提前到 `static_assert!` 旁边（C 在 lisp.h 3155，晚于用它的常量） |
| P-60 | 文件级 `static_assert` 不在任何清单里：lisp.h 有 **15** 条（121/143/829/1446/1579/2007/2213/2254-2258/2789/3171/4149/4294/5888），`lisp.rs` 只有 1 条（4294 的 `FLT_RADIX`）——正是"没有断言"让 P-59 的两条错值活了下来（C 的编译器当场就拒绝） | **部分已补，闸门规则待补** | 已补 4 条：char-table 的三条（lisp.h 2252-2258，把 `header_size`/`CHAR_TABLE_STANDARD_SLOTS`/`SUB_CHAR_TABLE_OFFSET` 钉在布局上）+ `PTRDIFF_MAX <= ULLONG_MAX`（4149）。其余 10 条：7 条是 `GCALIGNED (…)`/`NIL_IS_ZERO`（`GCALIGNED*` 已在 `excluded-macros.txt`，`NIL_IS_ZERO` 待补）、`static_assert (BITS_WORD_MAX >> (BITS_PER_BITS_WORD - 1) == 1)`（143）与 `POWER_OF_2 (FOR_EACH_TAIL_THRESHOLD)`（5888）应在下一轮补；2789 那条 `static_assert (sizeof x <= 2 * sizeof (hash_hash_t))`（在 `reduce_emacs_uint_to_hash_hash` 内）值等价但没镜像。评审 A 的 F5 还指出：原先把这一行框定成「文件级」断言，而 2007/2789/4149 都是函数内的；另外 `alignof Lisp_Float`/`alignof Aligned_Lisp_Subr` 根本不在 `docs/baseline/layout/c-layout-darwin-arm64.tsv` 里（只有 `sizeof` 键），那两条 `GCALIGNED` 断言没有对应的断言键。**待办**：`tools/extract-abi.py` 增加一条"文件级 `static_assert` 计数/条件比对"规则，否则下一次仍会静默漏掉 |
| P-61 | 镜像之间的 `include!` 构成的是**树**还是 DAG：`buffer.rs` 已经嵌套 `include!("../../lib/include/time.in.rs")`，而 `timespec.h`/`systime.h` 的镜像同样要 `struct timespec`——两个镜像拉同一个 lib 镜像会在同一编译单元里重复定义（`include!` 没有 include guard，0.1-4 说的"xtask 展开包含图并按文件去重"还没实现） | **已定规则 + 本轮按规则落地** | 规则：**每个 lib 镜像只能被"拥有者"镜像或 TU（crate root）引入一次**。本轮 `timespec.rs` 拥有 `gnulib_config.rs`，`systime.rs` 拥有 `timespec.rs` 与 `sys_time.rs`，`time.in.rs` 仍只由 `buffer.rs` 拥有，`sys_time.rs` **不**自己引 `time.in.rs`（靠 TU/上游镜像，与 `src/` 镜像不引 `lisp.rs` 同理）。实测：`gate-fast` 的布局探针把全部 `src/include/*.rs` 编进一个 crate 后仍无重复。**已实现（第八轮）**：`xtask` 加了 `include_graph_ordered`（后序 DFS，依赖在先——镜像的嵌套 `include!` 通常在文件头，它带来的宏必须先于该文件正文，0.1-18）与 `materialize`（把根文件的每个 `include!` 行就地展开成「每个可达文件一份副本」，副本内部的 `include!` 行换成同长度注释以保留行号，另生成 `_root.rs`）；`build-tu`、`build_finished_tus`、`dead-arms` 与布局探针都走它，并显式用 `--crate-name <stem>`。旧规则（「每个 lib 镜像只能被拥有者或 TU 引入一次」）随之**废除**：镜像现在可以自由嵌套。实测（负向）：把 `materialize` 短路回原根文件后，`layout-check` 立刻报 `E0428: the name time_t is defined multiple times`（`buffer.rs` 与 `timespec.rs` 都拉 `time.in.rs`，正是 termhooks/window 都要 `dispextern.h` 的形状）；开着去重时 237 条断言全绿。 |
| P-62 | gnulib 的配置宏不在任何参照头文件里：`_GL_CMP`、`_GL_CONFIG_H_INCLUDED`、`_GL_INLINE*`、`_GL_ATTRIBUTE_CONST`、`_GL_ARG_NONNULL` 都是 configure 从 `m4/gnulib-common.m4` 的 AH_VERBATIM 生成进 config 头的，`lib/timespec.h` 等头文件直接使用它们；而 `femacs/src/include/config.rs` 是 `xtask gen-config` 从**手工挑选的值常量 JSON** 生成的，放不下宏形状的东西 | **已定并落地** | 值常量：加进 `docs/baseline/config/darwin-arm64.json`（`_GL_CONFIG_H_INCLUDED = 1`，src/config.h 38），由 `xtask gen-config` 写进 `config.rs`；**宏形状**的：新文件 `femacs/lib/include/gnulib_config.rs`（`_GL_CMP`，C 的 `(((n1) > (n2)) - ((n1) < (n2)))`，PORTING.md 0.1-5 每宏一家）；纯拼写的：`_GL_TIMESPEC_INLINE`/`_GL_INLINE(_HEADER_BEGIN/END)`/`_GL_ATTRIBUTE_CONST`/`_GL_ARG_NONNULL` 记入 `docs/ports/excluded-macros.txt`（kind=syntax）。`lib/timespec.h` 的 `#error "Please include config.h first."` 用 `const _: () = assert! (_GL_CONFIG_H_INCLUDED, …)` 镜像（不是 `static_assert!`——lib 镜像要能被不含 `lisp.rs` 的探针单独编译） |
| P-63 | 第八轮两名对抗评审（Hooke/Volta）在**已提交**的 `d500727`/`e675757`/`133d837` 上找到：`hash_idx_t` 写成 `isize`（C 是 `int_least32_t`）→ `Lisp_Hash_Table` 88 字节而 C 是 72（P0，更早引入、零闸门覆盖）；`make_uninit_sub_char_table` 少一个 `min_char` 形参并把它钉成 0（P1）；新 `enum_constants` 的 `brace_depth` 会被 `extern "C" {`、多行 `#define` 续行、字符字面量里的花括号带偏（8 个参照文件的枚举成员**静默丢失**，其中包括本轮新镜像的 `lib/timespec.h`）；`/// C:` 注解使被注解声明的 Rust 侧**完全**不受检（改宽度/返回类型/加形参都能过）；61 条 `pub static` 与 3 条 `#[link_name]` 声明对探针不可见；`emacs_fclose` 写成包装 `fn`（C 是对象式宏别名，地址语义不同）；`cmp_bits_t` 应是 `unsigned int`；P-56/P-60 的证据与行号有错；`KEYMAPP`/`list!` 的说明与实现细节有误或缺记录 | **已修 + 已记录** | 逐条：`hash_idx_t = i32`；`make_uninit_sub_char_table (depth, min_char)` 并写回 `min_char`；`flat_code()`（去预处理行含续行 + 去字符字面量）+ 认 `extern "C" {` 的 `brace_depth`——实测全树 557 个参照文件**零丢失**、6 个文件**恢复**成员（`xterm.h` 的 FOCUS_*/`X_ERROR_MESSAGE_SIZE`、`macfont.h`、`image.c` 等）；`annotation_disagreement()`（比 arity 与每个形参/返回类型的宽度）——实测把 `charpos: ptrdiff_t → c_int` 的变异打红；`Option<unsafe extern "C" fn …>` 映射成 `void (*)(…)`，**5 条「未映射」变成受检**（68→**73**条、0 未映射、0 基线）；对象声明改为**计数并打印**（61 条）；`emacs_fclose` 改成 `macro_rules!`；`cmp_bits_t = c_uint`；`keymap.rs` 的 typedef 挪到 C 的位置；P-56/P-60 已订正。`SXHASH_REDUCE` 的签名（C 是 `EMACS_UINT → EMACS_UINT`，port 是 `EMACS_INT → EMACS_INT`，值等价）与 `list!` 把 C 的**未定序**两次求值钉成「先数组后实参」都记进 `docs/ports/lisp.md` |
| P-64 | `lisp-abi-probe` 的剩余盲区（评审 B 的 F2/F3 + 评审 A 的 F1） | **已记录，待做** | ①61 条对象声明只被计数打印，未比类型（C 侧可写 `static MAPPED *p = &NAME;` 配 `-Werror=discarded-qualifiers`——`chartab_size` 的 `const` 就漏在这里）；②`/// C:` 的 pointee 拼写只在**宽度**层面比对（`struct buffer *` ↔ `void *` 是同一 8 字节，可命名 pointee 的拼写差异看不出来）；③Rust 的 `pub type` **定义**（把 `Lisp_Word` 改成 `i32` 探针照样 OK）不受任何闸门检查，只靠布局断言兜底；④`Lisp_Hash_Table`/`Lisp_Obarray` 等类型没进 `tools/gen-layout-fields.py` 的 `STRUCTS`，所以 `hash_idx_t` 的 88→72 只能靠 C 探针实测发现（下一步把它们加进去并重生成 `docs/baseline/layout/*.tsv`，同时更新 `counts.tsv` 的 ratchet） |
| P-65 | `tools/gen-layout-fields.py` 的字段过滤用的是"名字出现在结构体正文里"，所以嵌套 `union`/`struct` 的成员会漏到外层类型上：把 `Lisp_Fwd` 加进 `STRUCTS` 时生成了 `offsetof (struct Lisp_Fwd, intvar)`（`intvar` 在 `union u` 里，不是外层成员），C 探针直接编不过 | **已记录，绕过** | 本轮先把 `Lisp_Fwd`/`Lisp_Buffer_Local_Value` 从 `STRUCTS` 拿掉（`Lisp_Hash_Table`/`hash_table_test`/`Lisp_Obarray` 已加入，211→237 条断言），并在 `tools/gen-layout-fields.py` 里写明原因；**待做**：按 `cinventory` 的嵌套路径（`struct_fields` 的 container/嵌套层）只取第 0 层字段，再把这两个类型加回去 | **已修（第十轮）**：`tools/gen-layout-fields.py` 新增 `direct_field()`——按字段名在结构体正文里的**花括号层数**判断（第 1 层才是该结构体自己的成员，嵌套 union/struct 的成员在第 2 层往后），于是 `Lisp_Fwd`/`Lisp_Buffer_Local_Value` 都能进 `STRUCTS`：断言 237 → **245**，pending 仍 7。顺带修掉一个只有它们才会触发的生成器缺陷：断言是**生成的 Rust**，字段名恰好是 Rust 关键字时要写成 `r#`（`Lisp_Buffer_Local_Value.where` → `r#where`），否则布局探针直接编不过——`rust_ident()` 按 PORTING.md 0.2 的关键字表转义。`glyph`/`glyph_slice` 等 dispextern.h 的类型**随镜像一起**加入（缺类型的键算 pending，而 ratchet 只允许 pending 下降） |
| P-66 | `data.c` 链上的头文件（`dispextern.h` 4026 行、`font.h` 1035、`termhooks.h` 994、`window.h` 1268、`frame.h` 1957、`keyboard.h` 536、`coding.h` 791、`composite.h` 363）**大部分正文在平台/图形分支里**（`HAVE_WINDOW_SYSTEM`/`HAVE_X_WINDOWS`/`HAVE_NTGUI`/`HAVE_NS`/`HAVE_PGTK`/`HAVE_HAIKU`/`HAVE_ANDROID`/`MSDOS`），冻结配置（P-08：仅 darwin-arm64 minimal）只启用很小的一部分。可 `tools/extract-abi.py` 的 `dead_branch_map` **只认字面 `#if 0`**：整段 GUI 声明的"missing" 都会被报出来，等于要求把 batch E/F（显示与平台后端，约 44 万行）在 batch B 之前搬完 | **已记录，方案已定** | ①（**部分已落地**：同一问题的窄版本——C 在**不同分支里重复定义**同名宏时，`extract-abi.py` 取的是**最后一个**定义（Android 分支的 `No_Cursor 0`、`ANDROID_GC_FOREGROUND`），与冻结分支相反；现在这类宏（`macro_duplicates > 1`，`dispextern.h` 里 9 个）的**宏体不比对**、只要求名字在场，并在报告里打印出来）`dead_branch_map` 改成**按冻结配置求值**：读 `docs/baseline/config/*.json` 的宏表（`#ifdef`/`#ifndef`/`#if defined (X)`/`!`/`&&`/`||`/`== 0` 这些形式求值，其余保持"两支都活"的保守行为），落在关闭分支里的 C 声明**不再要求镜像**，而是汇总成一行 "N declarations deferred to the platform target"（可见，不静默）；镜像里若仍写了它们（P-51 的做法）则照常比对。为让求值对未见过的 `HAVE_*` 也正确，需把 JSON 的宏表从"挑选的 30 条"扩成 `build/emacs-ref/src/config.h` 的**完整** `#define`/`#undef` 集（`tools/gen-config-baseline` 重跑，`docs/baseline/config/*.json` 随之更新，由评审复核）。②镜像策略：这些头文件先镜像**冻结配置的活文本**（其余分支以 `// C: #ifdef … (not mirrored on this target — PORTING.md P-66)` 行标出边界，checklist 里逐段列出未镜像部分，WIP 状态照常可见）；batch E/F 各自冻结目标时再补齐。③顺序：`dispextern.h` → `font.h` → `composite.h` → `coding.h` → `termhooks.h` → `window.h` → `frame.h` → `keyboard.h` → `process.h` → `data.c`（`composite.h` 无条件包含 `font.h` + `dispextern.h`，所以它排在这两个之后）。④包含图已就绪：P-61 的去重已实现并负向验证，`termhooks.h`/`window.h` 都要 `dispextern.h` 的菱形不会再撞 `E0428`；⑤**这套抽取器改动不是开工前的必需**：`--all` 本来就把「ledger 里还有未勾选方框」的文件报成 **WIP**（不是 FAIL，四个 lib 镜像就是这么挂着的），所以镜像只覆盖冻结配置的活文本、把平台段留成未勾选项即可开工，代价只是 WIP 计数上升——①仍是把 WIP 变小的精确化，可与 `dispextern.h` 并行做；**第十轮进展**：`tools/gen-cond-macros.py` 落盘了冻结配置的宏表（`clang -dM -E` 全量 dump + config.h 的 `#undef`，**3527 条**，`docs/baseline/config/darwin-arm64.macros.tsv`）；`tools/cinventory.py` 的 `dead_branch_map` 现在会在装载宏表时按 C 预处理器语义求值（`defined`/`!`/`&&`/`||`/比较/算术/`?:`（只求被选中的一侧）/文件内 `#define` 生效且只在活分支生效/**蓝漆规则**：展开中的宏不再展开，`VAL_MAX`↔`USE_LSB_TAG` 的互相引用因此得到 C 的答案）；`tools/check-cond-eval.py` 是它的**差分测试**（把每个分支首行插进 `enum { EMACS_COND_k = 1 };` 标记再预处理，与求值器逐条比对）。**第十二轮进展：求值器已转正，但只对「被差分证明过」的文件生效**。①求值器修到零不一致：宏表扩成 4956 条（`clang -dM -E` 全量 dump，含 `<sys/time.h>` 等系统头，`docs/baseline/config/darwin-arm64.macros.tsv`），求值器补齐 C 条件求值语义（`?:` → `||` → `&&` → 比较 → 移位 → 加减 → 乘除模 → 一元 → primary）、函数式宏展开（如 `GNUC_PREREQ (4,3,2)`）、`true`/`false`（C23 → 1/0）、蓝漆规则 `_COND_EXPANDING`；`tools/check-cond-eval.py` 逐条与 clang 比**分支活性**，实测 **9 个已验证文件、191 条条件、0 不一致**（1 条 unknown）。②`extract-abi.py` 的 `--cond-macros` **默认打开**，但**只对 `docs/baseline/config/cond-eval-verified.txt` 列出的文件生效**（`--cond-macro-paths`）；该表由 `check-cond-eval.py --write-verified` 写出，`xtask gate-fast` 每轮用 `--verified` 重跑差分，**只升不降**（不在表里的文件保持旧的盲态：除字面 `#if 0` 外两支都活）。**第十三轮修正（评审 A 的 F1/F3/F5）：这道「只对表内文件生效」的范围限定第一版是死代码** —— `_BlindConditions` 只把 `CONDITION_MACROS` 置空，而 `dead_branch_map` 读的是 `CONDITION_MACROS_BASE` 并会用 `set_condition_macros` 把它重新装回去，于是宏表实际作用于**每一个**文件；白名单缺失时还是 fail-open（`None` 表示「每个文件都用」），匹配用的是 `path.endswith` 后缀（`test/manual/etags/c-src/emacs/src/lisp.h` 会命中 `src/lisp.h`）。三处都已修：`BASE` 一并保存/清空、白名单缺失直接 `fail`、按 `reference` 根做**绝对路径恒等**匹配；`c_body_text`/`body_is_empty` 也移进同一个盲态作用域（评审 A 的 F2：它们在 `inventory()` 退出后才跑 `dead_branch_map`，`doprnt.c` 的 `exprintf` 体因此被当成「不存在」而不比对）。修好后的真实数字（同一轮实测，全部仍是 WIP、无 FAIL）：`limits.in.h` 1→**24**、`stdint.in.h` 95→**138**、`sys_types.in.h` 18→**17**、`time.in.h` 24→**32**、`dispextern.h` 159→**239**。`docs/ports/include-{limits.in,stdint.in,sys_types.in,time.in}.md` 是**盲态时代**生成的账本，与现在的闸门不是同一套要求，下次动这些文件时用新抽取器重生成。③为什么必须有这道闸：gnulib 的 `*.in.h` 是**模板**，条件里带 `@WINDOWS_STAT_INODES@` 这类 m4 占位符、引用的是**生成后**头文件的状态，冻结宏表根本无法判定——实测 `stdint.in.h` 40/74、`time.in.h` 72/126、`sys_types.in.h` 14/19、`limits.in.h` 5/20、`mini-gmp.h` 2/5 条不一致，故全部留在表外。这些文件的正确判据是 `build/emacs-ref/lib/` 下**生成后**的头文件（未决）。④打开后立刻抓到 `lisp.h` 里 7 处一直在用**错配置文本**的地方并已修正：`NATIVE_COMP_FUNCTIONP`/`NATIVE_COMP_FUNCTION_DYNP` 极性**反了**（把 `#ifdef HAVE_NATIVE_COMP` 的死文本当活的、把 `#else` 的 `return false` 当死的）、`CHILD_SETUP_ERROR_DESC` 取了 `DOS_NT` 分支（应为 5208 行的 `#else`：`"Doing vfork"`）、`EMACS_INT_WIDTH`/`EMACS_UINT_WIDTH`/`BITS_PER_BITS_WORD` 硬编码 64（应为 `LONG_WIDTH`/`ULONG_WIDTH`/`SIZE_WIDTH`；宽度宏按 C 的归属分放——`INT_WIDTH`/`UINT_WIDTH`/`LONG_WIDTH`/`ULONG_WIDTH` 在 `limits.in.h` 112-122，`SIZE_WIDTH`/`PTRDIFF_WIDTH`/`INTMAX_WIDTH`/`UINTMAX_WIDTH` 在 `stdint.in.h` 727-731）、`lisp_h_Qnil` 记入 `docs/ports/excluded-macros.txt`（P-17）。⑤显示链头文件的 checklist 待用新抽取器重生成（`dispextern.h` 已完成）。⑥**枚举的隐式成员过去完全不比对**（评审 B 的 F1/F2）：`tools/cinventory.py` 的 `enum_constants()` 只收带 `=` 的成员，于是 `enum draw_glyphs_face { DRAW_NORMAL_TEXT, DRAW_INVERSE_VIDEO, … }` 的六个成员与 `glyph_row_area` 的 `ANY_AREA = -1` 都不在 C 侧清单里，`dispextern.rs` 把 4 个值写错、漏掉 `DRAW_INVERSE_VIDEO` 与 `ANY_AREA`，而 `--all`/`layout-check` 全绿。现在按 C 的规则展开隐式成员（第一个是 0，之后逐一 +1；前驱不是字面量就不猜、保持沉默），枚举值进了常量闸门；这一改正立刻抓到 `lisp.rs` 缺的 26 个枚举成员（`SYMBOL_TRAPPED_WRITE`、`Test_eql/Test_eq/Test_equal`、`Set_Internal_Bind` 4 个、`handlertype` 6 个、`nonlocal_exit` 2 个、`gc_root_type` 4 个、`hash_table_weakness_t` 5 个），已按 C 的行号与取值补进镜像 |
| P-67 | 显示类头文件里有一类 C 类型是**匿名 struct 的 typedef**（`typedef struct { … } GLYPH;`、`Emacs_Color`、`Emacs_Rectangle`、`Emacs_GC`、`display_pos`、`it_slice`…）：Rust 的 struct 必须有名字，而 `tools/gen-layout-fields.py` 生成键时只认 `struct NAME` 标签（无标签就只打印一行 warning，C 侧其实可以写 `offsetof (Emacs_Color, red)`）；`--check` 的"类型字段集合"比对也按 typedef 名走 | **已定 + 部分落地** | 命名规则：镜像用 **typedef 的名字**声明 `#[repr(C)] pub struct NAME { … }`（ABI 只看布局），并在 checklist 的偏离项里记这一条。布局键的缺口**待做**：让 `gen-layout-fields.py` 支持无标签类型（用 typedef 名生成 `offsetof (NAME, field)`）。本轮实测的第二个教训：位域存储单元要按 **C 的 alignof** 建模——`glyph_slice` 是 8 字节但**只 4 对齐**，写成 `bits: u64` 加 `#[repr(C, align(4))]` 不成立（Rust 的 `align` 只能升不能降），断言当场 `error[E0080]: evaluation panicked: alignof glyph_slice`；改成 `bits: [u32; 2]` 才同时满足 8/4 | |
| P-68 | 第十二～十三轮这两名对抗评审（`791a8a4`/`acbeb3a`/`9f523e4`/`37f1aed`/`feb9955`）留下的**未修项** | **已全部修掉（round 13）** | ①**闸门健壮性（评审 B 的 F3/F4，P1/P2）**：`Lisp_Fwd` 在布局基线里只有 `sizeof`/`alignof`、**零条 offset 键**（它的 `u` 是具名 union 成员，`cinventory` 把 union 成员扁平化到容器后又被 `direct_field()` 按深度拒掉），于是把 `type`/`u` 对调、甚至删掉 `#[repr(C)]`，`layout-check` 与 `--check` 都仍然绿 —— 修法：给生成器加「为聚合成员发键」的能力（`OFF (struct Lisp_Fwd, u)`），并补一条不变量「非位域成员 ≥2 的类型至少要有一条 offset 键」；`counts.tsv` 的 ratchet 只锁计数不锁键集，删一条 `Lisp_Buffer_Local_Value.where` 再补一条可满足的键就能腾挪覆盖（修法：冻一份排序键列表或摘要，要求键集只增）。②**`native_comp` 的 const 断言是装饰（评审 A 的 F6）**：`cfg!(feature = "native_comp")` 在任何真实构建里恒为 `false`（workspace 没有 `[features]`，`xtask dead-arms` 的 `FLIPS` 也没有它），所以那一对死文本从未被编译过；修法：把 `("native_comp", "HAVE_NATIVE_COMP", "true")` 加进 `FLIPS`。③**当轮已修**：`rust_ident()` 换成完整关键字集（含 `box`/`gen`/`try`，并对 `self`/`Self`/`crate`/`super` 直接报错）；基线的「权威文件」改为只认唯一的 `c-layout-*.tsv` 并把文件名打进日志；行号引用已校正。④**评审确认无害、不必重查的部分**：`feb9955` 的 7 处修正与 clang 活文本逐条一致；`glyph_slice` 的 `[u32; 2]` 与 C 的 8/4 及字节模式一致；基线取值可用 `build/emacs-ref` 重放成**逐字节相同**的产物；`GCForeground`/`GCBackground`/`No_Cursor` 的冻结活值被正确选中；`Emacs_Color`/`Emacs_Rectangle`/`Emacs_GC`/`GLYPH` 的布局与 C 逐项相同（但**都还没有布局键**，属覆盖缺口）。 **round 13 的修法（三条都已落地并负向验证）**：①`tools/gen-layout-fields.py` 增 `EXTRA`（`Lisp_Fwd` 补 `offsetof (struct Lisp_Fwd, u)`；C 的 `type` 是位域，`offsetof` 对它非法，所以只能键 `u`，而 `u` 落在 8 就钉住了位域存储单元在前），并加不变量：`STRUCTS` 里每个类型至少一条 offset 键，唯一例外 `glyph_slice`（四个位域）显式记在 `BITFIELD_ONLY` 里带理由 —— 没有这条，一个"本该有键"的类型可以静默变成零键。基线由 `spikes/run.sh` 从 C 编译器重放（255 键，新增 `offsetof Lisp_Fwd.u = 8`），把 `type`/`u` 对调现在报 `error[E0080]: offsetof Lisp_Fwd.u`（已实测，restore 后绿）。②`counts.tsv` 之外新增 `docs/baseline/layout/asserted-keys.txt` 键集棘轮：`xtask layout-check` 要求每一次仍被断言的键都在表里，键集只增；把 `offsetof Lisp_Buffer_Local_Value.where` 从基线删掉（并把计数下限临时放低以隔离两条棘轮）会报 `layout keys disappeared … update asserted-keys.txt only with a reason`（已实测）。③`dead-arms` 的 `FLIPS` 加了 `("native_comp", "HAVE_NATIVE_COMP", "true")`：这是 `lisp.rs` 里 `#[cfg(feature = "native_comp")]` 那段**第一次**过 rustc，立刻抓到两处拼写错误 —— `NATIVE_COMP_FUNCTIONP` 把 C 的 `!NILP (XSUBR (a)->native_comp_u)` 写成 `… != 0`（类型不符），`SUBR_TYPE` 里字段名是 `r#type` 而写成了 `type_`；`DEFUN!` 的 `@record` 分支还要补 C 用聚合规则零填充的四个 native 字段（C 的初始化器停在 `lisp_h_Qnil`）。修完 5 个 TU 在非冻结分支下全部编译通过。⑤**广域差分的边界（评审 A 的实测）**：`emacs/src/*.h` + `emacs/lib/*.h` 共 202 个头里，求值器对 clang 的真错答有 **471 处**，其中 **20 处是「clang 说活、求值器说死」**这种危险方向（`src/conf_post.h:68`、`lib/cdefs.h:44/49/567/669/796` 等）——它们目前都不在已镜像的文件上，白名单修好后不会被用到，但任何新镜像**必须先进 `cond-eval-verified.txt`**（先跑 `--write-verified`），否则闸门会拿错分支当要求。 |
| P-69 | 显示链上的头文件互相引用**不完整类型**（`dispextern.h` 的 `struct glyph`/`glyph_matrix`/`face` 用 `struct frame *`/`struct window *`/`struct font *`，而 C 只靠"首次使用时隐式声明 tag"；`font.h` 的 `struct font_driver` 同样用 `struct frame *`，`composite.h` 用 `void *font` 绕过） | **已定** | C 里 `struct X *` 不需要 `X` 完整，甚至不需要提前声明；Rust 没有不完整类型，所以镜像按**包含图**解决：`X` 的**定义**放在 `X.rs`（`frame.h`→`frame.rs`、`window.h`→`window.rs`、`font.h`→`font.rs`），只以指针用到它的镜像（`dispextern.rs`、`composite.rs`）**不重复声明**，靠 `xtask` 的 TU 包含图把 `X.rs` 排进同一次编译（与 `lisp.rs` 依赖 `buffer.rs`、`buffer.rs` 依赖 `itree.rs` 完全同一条规则；P-61 的去重保证同一类型只有一个声明处）。**顺序推论**：`dispextern.h` 的 `glyph`/`glyph_matrix`/`glyph_row`/`face`/`face_cache`/`it` 与 `font.h` 的 `font_driver` 都排在 `window.h`/`frame.h`/`font.h` 的对应结构**之后**落地（Rust 声明与顺序无关，但类型必须存在于同一 TU）；`composite.h` 因为 C 自己写成 `void *font` 所以不受这条约束，已先落地。 |
| P-70 | `coding.h` 的 `struct coding_system.spec` 里 `struct ccl_spec *ccl` 指向 **ccl.h**（本头不包含它）：C 里这是"未声明 tag 的不完整类型"，Rust 没有不完整类型 | **已定 + 已落地** | 只作指针用途、且自己的头尚未移植的类型，由**首次命名它的镜像**给一个显式替身（本处 `pub struct ccl_spec;`，零大小单元结构；`*mut ccl_spec` 是薄指针，替身自身尺寸永不进入布局），在 ledger 的偏离项里记一行；等 `ccl.rs` 落地时**同一个提交里删掉替身**（否则一个 TU 里两个同名声明会 E0428）。同样适用于将来的 `struct frame`/`struct window`/`struct font`（P-69 的完整定义优先，替身只在还没轮到它们时使用）。 |
| P-71 | `tools/cinventory.py` 的字段扫描器在**嵌套聚合**处出错（本轮实作 `coding.h` 时发现） | **已修 + 已负向验证** | 两个缺陷：①`STRUCT_OPEN_NAME` 写成 `^(struct|union)\s+(NAME)?\s*$`，`\s+` 是必需的，所以裸的 `union`（无名、行上只有关键字）**开不了帧**；②`}` 开头的行只弹栈、**不发字段**，而 `} spec;` 的 `}` 把**外层 struct 也弹掉了**，于是 `coding_system` 的 `spec` 及其后 **17 个成员**（`safe_charsets` 到 `encoder`）对扫描器完全不存在 —— 616 字节的布局只被 `id`/`max_charset_id` 两个键覆盖。修法：分隔符改可选、`} NAME;` 作为**外层容器的字段**发射（`offsetof (struct coding_system, spec)` 合法，C 允许对聚合成员取偏移）。修完 `coding.h` 的键从 37 条增到 66 条，367 条断言全部通过；`gen-layout-fields.py` 的 `EXTRA` 相应去重（扫描器现在自己能报出 `Lisp_Fwd.u`）。**同类风险**：`window.h`/`frame.h` 里大量 `union { … } u;` 形状，落地时必须确认这些聚合成员都进了 `STRUCTS` 的键。 |
| P-72 | 镜像**只能命名它自己声明的类型**（或经它自己的 `include!` 拿到的类型）：Rust 的类型解析不像 C 的"不完整 tag"，一个只在别处定义、而本镜像引用的名字会让**不包含那个头的单元**编译失败 | **已定 + 已落地** | 规则：①类型的**定义**放在定义它的头对应的镜像里；②`typedef struct X Y;` 这类只有 tag 的 typedef 跟着**定义**走（`KBOARD` 从 `lisp.rs` 移到 `keyboard.rs`，虽然 C 写在 lisp.h 3570）；③如果**声明**所在的头的 ledger 要求该声明留在原地（`lisp.h 5081` 的 `extern struct kboard *echo_kboard;`），则指针退化成 `*mut c_void` 并加 `/// C: …` 注释（该约定已有先例：`Lisp_Subr.kbd`、`struct buffer *` 的两个注解），由使用点显式转换；④需要完整类型的单元必须把对应镜像加进自己的 `include!` 列表（`xtask` 的包含图跟随 `include!`）。**触发实例**：落地 `keyboard.rs` 时删掉 `lisp.rs` 的 `kboard` 替身，`src/bignum.rs`（C 的 bignum.c 不需要 kboard）立刻 E0425 —— 这正是这条规则的来源。 |
| P-73 | `tools/cinventory.py` 的字段扫描器**按行**读声明：跨行的成员声明（`void (*set_window_size_hook) (struct frame *f,\n int width, int height);`）会从参数里造出**幽灵成员**（`struct terminal` 因此多出 `width`/`yoff`/`title`）并**丢掉真成员**（那 51 个 hook 里有 4 个没进任何清单/键） | **已修 + 已实测** | 声明现在**拼到分号为止**再解析（与预处理器的规则一致），幽灵成员消失、真成员回来（`terminal` 从 63 个"成员"变成 73 个真实成员），并顺带找回 `composition_status.carryover`（多行数组声明）。**影响面实测**：修复后 `--all` 仍是 17 done / 7 WIP / 0 failing，布局键 325→326（只有那一个），说明此前没有已完成的文件因此被误判；但**任何多行声明的类型**在没有这条修复前都不会被正确键控。 |
| P-74 | `tools/cinventory.py` 的字段扫描器在**收尾声明符换行**时丢成员：`frame.h 653-663` 写成 `union output_data { … }` + 换行 + `output_data;`，P-71 的 `} NAME;` 规则只认同行形式，于是 `struct frame` 少了 `output_data` 这个成员、那 8 字节的联合体在布局基线里**没有任何键** | **已修 + 已实测** | 收尾规则扩展成：若某行只有 `}`（没有声明符），则把**下一行**当作声明符读取并消费掉（`output_data` 因此进入 `struct frame` 的字段表与键表，688 条断言覆盖 568 字节的整个 frame）。同类写法（`}` 与成员名分行）在 `window.h`/`termhooks.h` 里也存在，但那些是匿名聚合、本就无成员名可键。 |
| P-75 | `tools/extract-abi.py` 的**函数体比较**不区分死活分支：`SET_FRAME_ICONIFIED` 在冻结配置下的活文本只有 `eassert`+`f->iconified = i` 两行，但 `c_body_text` 把 `#ifdef HAVE_WINDOW_SYSTEM` 里的 `XSETFRAME`/`FRAME_WINDOW_P`/`gui_consider_frame_title` 也拿去比 token，等于要求端口实现**本目标从不编译**的代码 | **已修 + 已实测** | `c_body_text` 与 `body_is_empty` 现在按该文件自己的宏状态**丢弃死行**（与 P-66 的求值器同一套判定），比较的才是冻结配置真正编译的函数体。**影响面实测**：`--all` 仍是 17 done / 9 WIP / 0 failing，`frame.h` 的差异从 64 降到 48；此前所有含 `#ifdef` 分支的函数体都在要求死文本（例如 lisp.h 的 `NATIVE_COMP_*` 一族），这条修复把这类"幽灵 token"一次性去掉。 |
| P-76 | `extract-abi.py` 的 `defsubr` 抽取不查条件表（`CONDITION_MACROS_PATHS`）：`data.c` 4360-4362 的 `defsubr (&Suser_ptrp)` 在冻结配置（`HAVE_MODULES` undef）下**不该被期望**，但工具仍报 `missing defsubr`；同一文件的 `DEFUN ("user-ptrp", …)` 在把 `src/data.c` 加进 `cond-eval-verified.txt` 之后**正确地**被丢弃了（于是把死文本移植成 `#[cfg(feature = "modules")]` 反而变成 `the reference file has no such DEFUN`）。 | **已修**（2026-10-02，`port(data.c) XIII`） | 两轮评审（A 的 P1、B 的 F1）独立发现：`SUBRS`/`DEFUN` 走条件表，`defsubr` 走原始文本。修法已按本节记录的方式落地：`extract-abi.py` 的 `defsubr` 抽取先在 `ci.blank_comments` 的文本上算 `ci.dead_branch_map`，落在未选中分支里的调用不再被要求，于是 `data.c` 的 `Suser_ptrp`（`HAVE_MODULES` undef）与三条 `HAVE_NATIVE_COMP` 注册都不再报 `missing defsubr`，而 `data.c` 本身第一次通过 `--check`（OK）。ledger 偏离项的临时办法作废。 |
| P-77 | `no_mangle` 规则是**单向**的、且用固定 400 字符回看定位属性：给 C 的 `static` 函数凭空加 `#[unsafe(no_mangle)]`（多一个导出符号）不报错；属性与定义之间插一段长注释就**误报** | **已修**（2026-10-02） | 两轮评审用 `/tmp` 变异体实测（第二轮评审 B 的 M2）。修法：`extract-abi.py` 改为双向规则——C 的非 static 函数必须有 `#[unsafe(no_mangle)]`，Rust 的 `#[unsafe(no_mangle)]` 也必须对应 C 的非 static 定义（否则报 `invented exported symbol`），两者的属性→`fn` 解析共用一个跳过属性/注释/空白的解析器，不再做字符回看。证据：变异体被报错、`--all` 仍 17 done / 10 WIP / 0 failing。 |
| P-78 | **`DEFUN`（subr）的函数体从未参与 token 比对**：`cinventory.function_spans` 有意跳过 `DEFUN (...)`（C 在下标名位置写的是 `Sname`），于是 `c_body_text` 对任何 subr 都返回 None，而比对循环只遍历 `c["functions"]`。实测（2026-10-02）：把 `Feq` 的 `EQ` 改成 `BASE_EQ`、删掉 `Fsetcar` 的一条 `XSETCAR` 语句，**所有闸门全绿**。 | **已修**（2026-10-02） | 新增 `cinventory.defun_spans`（在注释已抹白的文本上做括号/花括号配对，取 `DEFUN` 第二个实参作函数名），`c_body_text`/`body_is_empty` 与比对循环同时遍历 plain function 与 subr；并修掉“以 `#` 开头的整行被丢弃”导致 Rust `#[cfg]` 语句整块丢失（`floatfns.rs` 的 `Fround` 曾比成空集）。覆盖实测：`defun_spans` 找到 150 个 subr 体（data.c 120 / floatfns.c 25 / casetab.c 5），其中 **82 个**真正参与 token 比对（52+25+5；其余是尚未移植的 subr，在 `rust_body is None` 处跳过；data.c 另有 4 个 DEFUN 被条件表判死，本就不期望）；变更后 `--all` 仍 17 done / 10 WIP / 0 failing，首轮扫出的 3 条里 1 条是工具 bug（已修）、2 条是 C 把 `0`/`1` 传给 `bool` 形参（记入 `docs/ports/macro-body-notes.tsv`）。 **残余盲区（评审 B 实测）**：① token 集合看不见"成对常量互换"（`SYMBOL_VARALIAS`↔`SYMBOL_PLAINVAL`、`>`↔`>=`、`idx <= 0`↔`idx < 0`）与 ≤2 字符标识符（`skip_short`，`EQ`↔`BASE_EQ` 因此不可见）——已移植的 52 个 subr 中有 40 个含这类 token，这些点目前只靠人工复核，等 ERT/差分层的"行为探针"（见 docs/port-master-plan.md §7.2）；② `function_spans`（旧路径）与 `cinventory.defuns`（台账路径）的花括号/引号计数仍是字面量盲的，`defun_spans` 已用 `blank_literals` 修好，另外两处按同一理由待修（全树 1,849 个 DEFUN 实测 0 例触发）；③ 给 `bool` 形参传的 `0`/`1`（casetab 两处）走豁免后，`false`↔`true` 互换不可见，见 notes 行的说明；④ **“token 全在、语义变了”这一类完全看不见**——第四轮评审 B 抓到的实例是 `Findirect_variable` 把 `let mut object = object;` 写在 `if` 块内，`XSETSYMBOL!` 写到被遮蔽的绑定而 `return object` 读的是形参（别名链因此静默失效）；第五轮又实测三例：把 `bits | BLV_FOUND` 换成 `| BLV_LOCAL_IF_SET`、把 `& BLV_LOCAL_IF_SET` 换成 `& BLV_FOUND`、整句删掉位写——根因是 notes 机制只能**减**要求，无法要求 Rust 侧的替代拼写（`BLV_FOUND`/`SYMBOL_*_MASK` 在 C 体里根本不存在）。集合式比对无从发现，只能靠人工复核与后面的行为探针。 **第六轮（2026-10-02）部分收口**：④ 类里“整句删掉位写”的一个实例（`make_blv` 的 `blv->local_if_set = 0`）已由 `docs/ports/token-aliases.tsv` 新增的 `local_if_set`/`blv_local_if_set` 家族钉住——家族自带**出现次数**检查，端口必须至少与 C 体同次数地拼写替代 token，删句即报；`set_internal` 的豁免里随之删掉 `local_if_set`，`make_blv` 的豁免行整行删除。`redirect`/`trapped_write`/`found` 仍在逐函数豁免表里，同一类盲区对它们依旧存在；把家族推广到这三个名字是下一项闸门改进。**第七轮（2026-10-02）实测补充（评审 B 的 30+ 变异体，逐条 CLI 确认）**：本轮三条新豁免行本身恰为差集（不多不少），但集合式检查管不住「C 的 token 位置上端口写了什么值」：① `uninit` 豁免后，`UNINIT` 对应的初始化器（`Lisp_Val_Fwd { value: Qnil () }` / `false`）的值、乃至「它是否还存在」都不在任何闸门视野内（改成 `Qunbound ()`/`Qt ()`、整句删掉都全绿；`qnil` 还被 PLAINVAL 臂里的 `valcontents.value = Qnil ()` 顶替）；② `forwarded` 四处赋值的真值（声明处与臂内）；③ 整段删掉 `redirect` 的 packed-bit 写（写里的 `SYMBOL_LOCALIZED` 被 `match` 臂头顶替、`redirect` 被 dispatch 读顶替 —— 所以「家族化 `redirect`」对这类**删除**变体也无效，家族只提供出现次数**下界**，不提供极性/运算符检查）；④ 把 `SYMBOL_REDIRECT_MASK/SHIFT` 换成 `SYMBOL_TRAPPED_WRITE_MASK/SHIFT`（这类 Rust 自造名在 C 里没有 token）；⑤ `| BLV_LOCAL_IF_SET` → `& ! BLV_LOCAL_IF_SET`；⑥ `continue` → `break`（C 是 `goto start`，`goto`/`start` 已豁免；换出的 `break` 反而满足 C 的 switch-arm `break`）；⑦ `CALLN!` → `calln!`、`Fdelq (tem, …)` → `Fdelq (variable, …)`、`if idx > 0` → `idx < 0`、`if ! (*blv).fwd.is_null ()` 丢 `!`、`&&` → `||`、`SET_PER_BUFFER_VALUE_P` 第三实参 `true`/`false`。两个可落地的机制（本段不擅自引入，记为下一项）：`docs/ports/init-values.tsv`（逐声明点记录初始化值，`va-arg-types.tsv` 同形）与 `docs/ports/bitfields.tsv`（C 位域名 → 镜像 MASK/SHIFT 常量对）；此外评审 A 要求把 `Lisp_Symbol`/`Lisp_Buffer_Local_Value` 的位域布局并入可复跑的 `spikes/write_probe.*`（目前只靠第三轮的 clang+gcc 探针与 ABI 推理），这是三者中最廉价的一项。**第八轮（2026-10-02）新增实测**：`variable-binding-locus` 的 `FALLTHROUGH` 形状（空 `SYMBOL_LOCALIZED` 臂 + `break` 落到共享 tail）有 6 个变异体全绿——空臂改成 `return Qnil ()`/`return Qt ()`、删掉/改成 `continue` 那个 `break`、把 tail 搬进臂里再删掉循环后的 tail、给 `SYMBOL_FORWARDED` 臂加 `continue;`；位域 dispatch 与 tail 判定 6 个（掩码/移位互换、丢掉 `>> SHIFT`、tail 的 `== SYMBOL_LOCALIZED` 反转、两个臂标签互换、`KBOARD_OBJFWDP`↔`BUFFER_OBJFWDP`）；别名臂 `continue`→`break`/删除 2 个；`buf`→`current_buffer` 4 处；`BLV_LOCAL_IF_SET` 的 `!= 0`→`== 0`；`Qt`/`Qnil` 互换。两个新机制记为下一步：`docs/ports/required-tokens.tsv`（**正向**要求：C 有 `FALLTHROUGH` ⇒ 端口必须有 `break`；C 有 `goto start` ⇒ 端口必须有 `continue`）与逐 token 计数下界（`rust_count >= c_count`，可抓全部 4 处 `buf` 站点）；`&&` 不是 token、`buffer` 被 Rust 的 `*mut buffer` 类型名顶替，这两条只有探针层能覆盖。另：评审 B 独立复现了**仓库级既有** CI 阻塞项（非本轮引入），已在后续提交里修完：① `cinventory.struct_fields` 的第二个 `<aggregate>` 分支用了 `type` 而不是兄弟分支的 `c_type`（`gen-lifetimes.py` 读的头部域 54 条、`.c`+`.h` 全域 81 条），`gen-lifetimes.py` 与 `extract-abi.py` 的**逐文件 doc 模式**（生成 `docs/ports/*.md` 的那条路径）都会在第一个这样的头文件上 `KeyError: 'c_type'`；`emit-mirror.py` 实测**不受影响**（只在 `struct_body()` 为 None 的回退分支读该键，这些容器到不了那里）—— 键名已统一；② `gen-lifetimes.py` 重写整表时**不搬运决策列**，第一位填写 `owner`/`freed_by` 的移植者会被任何一次再生抹掉（当前表里决策数为 0，所以尚未造成损失）—— 现在按「行的非决策列」为键搬运，行消失的决策会打印并让运行失败；③ `docs/baseline/*.tsv` 相对当前抽取器过期（最后一次写入是 `7ad447f`）：常量 1,834→**3,898**（+2112/−48，其中约 1,546 条是枚举隐式成员，其余是此前被 `flat_code` 挡住的显式枚举成员）、结构体字段 5,441→**5,764**（+324/−1，其中 84 条是 `<aggregate>` 行，另含 P-73 的多行声明修复与旧扫描器从函数指针形参里造出的 33 个伪字段的移除）、subr doc digest 148 行（只动最后一列、行集不变）—— 已再生，并把 master plan 的逐目录表/合计、`docs/goal.md` 与 PORTING.md 的计数表同步更新；两位评审各自复算了这份分解。剩下唯一的 CI 阻塞项是「CI 无 `emacs/` 检出」（见 master plan 403）。**第九轮（2026-10-02）第二个 P-78 类实例（评审 B 的 HIGH）**：`cinventory._defun_heads` 在 Lisp 名之后只跳过 `", \t"`，于是 `DEFUN ("name",\n       Fname, …)` 这种**换行写 C 名**的（Emacs 自己的排版风格：treesit.c 47/58、xwidget.c 25/36、process.c 9/63、w32fns.c 11/60、xfaces.c 8/33…）全被 `re.match` 落空跳过——全树只有 **1,708/1,849** 个 DEFUN 体能进入比对。已把 `\r\n` 计入跳过集合：treesit.c 11→58、data.c 恒为 120、全树 1,849 = 文本出现数；`--all` 不变（17 done / 10 WIP / 0 failing），因为新可达的体都在尚无端口的文件里——不修的话，那些文件移植时会**静默**失去这一层检查。**第十轮（2026-10-02）实测补充（评审 B，`aref`/`aset` + 四个 helper 的 27 个变异体）**：15 个仍全绿，全部属已知的集合式类别——阈值常量（`0x7f`→`0xff`/`0x7e`，且 `0x7f` 被切成 `0`+`x7f` 两个 token）与第二个 `0x7f` 同形、`idxval < 0`→`<= 0`、`! NILP` 极性、`BOOL_VECTOR_P`↔`CHAR_TABLE_P` 与 `VECTORP`↔`CLOSUREP` 臂互换、两处同名 `ASIZE`/`PVSIZE` 只改一处、三条 `error`（字符串字面量不参与比对，删掉其中两条也不报）、`SREF`/`SSET` 的 cast 宽度、`SREF (array, p)` 改成错的下标（`p`/`c` 只有 1 字符，被 `skip_short` 丢掉）。最便宜的收口仍是这里记过的两条：逐 token 计数下界（可关掉 8 个）与 `error` 的字符串字面量比对；同一段里四个 helper 是反例——它们的必需 token 没有一个 C 计数 > 1，逐个都是唯一承重。 |
| P-79 | **能用 safe 就用 safe**（用户 2026-10-02 明确要求）：允许 `unsafe` 不等于应该到处 `unsafe`——只有 C 的指针/位域/联合体/变参/裸内存这些**没有 Rust 安全替身**的地方才用，局部值、算术、切片、迭代、`Option`/`Result`、`match` 一律写 safe Rust | **规则，立即生效** | 口径：①`unsafe` 只能用来镜像 C 的构造（解引用 `*mut`、读写联合体字段、调用 `unsafe fn`、位掩码访问），**不得**用来跳过 Rust 本可静态保证的东西；②能用一个表达式大小的 `unsafe { … }` 就不要包一整块；③新代码里每一处 `unsafe` 都要能指回它所镜像的 C 构造——C 里没有指针运算/联合体/bitfield 的行就不该出现 `unsafe`；④既有代码不做大规模改写（评审基线是按现有形状验过的），但在**下一次触碰**该文件时把无谓的 `unsafe` 收窄；⑤`femacs/xtask/` 与 `tools/`（与 C 无关的宿主侧代码）一律 safe。当前形状的背景：各单元头部的 `#![allow(…, unsafe_op_in_unsafe_fn)]` 是**刻意**的半径决定（否则每个 `*mut` 解引用都要多一层大括号），因此评审按上面五条检查**新写入**的 `unsafe`，把无谓的记为 LOW；本规则不影响已落地代码的既有形状。 |
| P-80 | **位域成员在 Rust 没有字段**，而同一个 C 成员名（`padding_p`、`tab_line_p`、`width`）同时属于多个已镜像结构体：`token-aliases.tsv` 的家族是全局的，会互相串味，所以位域不能走那条路 | **规则，立即生效** | 存储单元化 + **在使用处点名成员**：①每个位域段在镜像里是一个整型存储单元（`glyph.bits: u64`、`glyph_row.bits: [u32;6]` + 有符号的 `right_fringe_offset: i8` + `bools: [u8;4]`、`glyph_matrix.bits: u8`），其位置由真实编译器的 `-fdump-record-layouts` 与逐字节对拍证明（`offsetof`/`sizeof` 看不见位域）；②读取/写入写成**按成员名分派的访问器宏**，成员名作为宏的第一个参数留在移植文本里，于是 `--check` 的宏体 token 比对照样通过：`glyph_bits! (padding_p, g)` / `glyph_bits! (padding_p, g, v)`、`glyph_row_bits! (enabled_p, row[, v])`、`glyph_matrix_bits! (tab_line_p, m[, v])`、`glyph_slice_bits! (width, s[, v])`；③这套形状**按类型分派**，所以同名成员在不同结构体里互不影响，也不存在宏重定义遮蔽；④证明位域位置是本条的硬要求——两个位置都由真 C 编译器的字节串与 Rust 侧的字节串比对（第 55 轮 `glyph`/`glyph_row` 实测逐字节相同）。 |

**P-38/P-24 说明（docstring 的值，2026-10-02 第三轮）**：C 的 `doc: /* … */` 注释不是它的值——
`lib-src/make-docfile.c` 的 `read_c_string_or_comment` 会**丢掉注释首尾空白**，并把 `\X` 读成 X（`\n`/`\t` 等控制字符除外），
所以 C 里的 `\"extra\"` 在 `etc/DOC` 里是 `"extra"`。port 的 `doc:` 字面量必须**解码到同一个值**（评审 A 用本机真实
`etc/DOC` 逐字节验过 casetab.c 的 5 条），而闸门也必须按这个值比对：`cinventory.doc_comment_value` 是唯一实现。
树里有 1,838/1,849 个 DEFUN 注释在 `*/` 前带尾随空白、139 个含 `\`+非 `n/t` 转义，按旧约定写会让几乎每个 docstring 与参照不同。
空白差异仍被 `digest` 的空白折叠吞掉（P-36），词级差异会报 `subr …: docstring differs`。

**探针作者注意（2026-10-02）**：把镜像包进 `mod m { include!(…) }` 时，**带 `#![allow]` 的单元文件（如 `casetab.rs`）会报
"an inner attribute is not permitted in this context"**；探针要么把镜像当 crate root（此时 `lispsym` 之类的外部符号改用嵌套模块定义），
要么去掉那行内部属性。`macro_rules!` 的路径在**调用点**解析，所以探针的调用处必须把 `Fcons`/`c_void`/`list1`… 放进作用域。

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
10. 删除 C 的前向声明（§0.1-12）、把"分支内逐字段赋值"改成先整体零值初始化（§0.1-13），
    并在 `docs/ports/NAME.md` 的偏离项里写明；定义一律 `#[unsafe(no_mangle)]`（§0.2）。
11. **镜像里的每个内联函数都要与 C 的"函数体"逐字比对，不能只看名字**：评审 A 在
    `sort.rs` 上发现 `XSYMBOL` 少了 `XBARE_SYMBOL` 的 `+ lispsym` 基址（返回的是编码
    偏移而不是指针），`--check`/`build-tu`/`layout-check` 三道闸门全绿却会段错误。
    镜像改动用可运行探针验证；行为级差分见 P-30。
12. **gnulib 的 `_GL_*_INLINE` 包装 `.c`（`lib/stdbit.c`、`lib/streq.c`、`lib/strnul.c`
    …）的判据**：这类文件的内容只有 LGPL 头 + `#define _GL_X_INLINE _GL_EXTERN_INLINE`
    + 两个 include，作用是 C99 inline 模型里"由这一个单元发出外部定义"（冻结配置的
    `_GL_EXTERN_INLINE` 就是两个字面关键字 `extern inline`，宏表 2104）。端口的函数是
    各只有一份定义的普通 Rust 项，所以 **端口保留的是宏名和它与 `_GL_EXTERN_INLINE` 的
    关系**：`lib/include/gnulib_config.rs` 里 `_GL_EXTERN_INLINE` 是 marker 常量，
    包装文件里 `pub const _GL_X_INLINE: () = _GL_EXTERN_INLINE;`，函数体本身落在对应的
    `lib/include/<header>.rs` 或 `src/` 镜像里。这类文件**不写 `// femacs:` 文件头**：
    C 自己只有 20 行左右，漂移闸的 ±5% 容不下额外的头注释——出处写在 `docs/ports/<stem>.md`。

13. **gnulib 的"标识符重命名"与"包含式 shim"**：①gnulib 的头会把被替换的函数名重命名
    （`<string.h>` 在 `HAVE_MEMPCPY` 未定义时把 `mempcpy` 变成 `rpl_mempcpy`），所以
    `lib/mempcpy.c` 的定义在 C 目标文件里发出的符号是 `rpl_mempcpy`。端口保留 C 的函数名
    （台账按名字比对）并用 `#[unsafe(export_name = "rpl_mempcpy")]` 发出同一个符号；
    `extract-abi.py` 的"这个函数是否导出"判据同时接受 `no_mangle` 与 `export_name`
    （Rust 禁止两者并用），反向判据（导出 C 没有的符号）不变。②`lib/dtoastr.c`、
    `lib/nstrftime.c`、`lib/strtoll.c` 是**包含式 shim**：`#define LENGTH 2` /
    `#define my_strftime nstrftime` / `#define QUAD 1` 之后 `#include "ftoastr.c"` /
    `"strftime.c"` / `<strtol.c>`，即同一份源码在不同宏绑定下产出不同符号。Rust 没有
    解析前的文本宏替换，所以共享源码要先落成 prelude 里的 `macro_rules!`（`DEFUN!` 的
    同型机制）或按绑定参数化，再由每个 shim 实例化一次；这三个文件因此排在共享源码
    （`ftoastr`/`strftime`/`strtol`）之后落地，台账要逐实例记名。
