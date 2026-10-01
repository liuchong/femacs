# A6 前置：`DEFUN` 机制（1,736 个 subr 的定义形式）与 P-28 闸门

参照：`emacs/src/lisp.h` @ `fd7afced403` 的 `DEFUN`（3406-3440）、`struct Lisp_Subr`
（2174-2212）、`enum maxargs`（3447-3450）、`SUBR_SECTION_ATTRIBUTE`（3399-3403）。
产物：`femacs/src/include/lisp.rs` 的 `DEFUN!` / `intspec_string!` 宏（PDF 见
`PORTING.md` 5.1），以及 `tools/extract-abi.py` 的 P-28 检查。

## 为什么需要它

`casetab.c` 有 5 个 `DEFUN`，而 590 个文件里有 1,736 个 subr。C 的 `DEFUN` 一次性给出
两半：`static union Aligned_Lisp_Subr sname = {{{PVEC_SUBR << PSEUDOVECTOR_AREA_BITS},
{.a ## maxargs = fnname}, minargs, maxargs, lname, {intspec}, lisp_h_Qnil}};` 与函数
`Lisp_Object fnname`。Rust 侧必须同构：同名静态记录（内部链接）+ `#[unsafe(no_mangle)]`
的函数定义。

## 证据：C 与 Rust 的记录逐字段一致

C 侧探针 `/tmp/defun/cprobe.c` 逐字转录参照的 `struct Lisp_Subr`、`union
Aligned_Lisp_Subr`、`DEFUN`、`SUBR_SECTION_ATTRIBUTE`，常量单独从参照里数出来
（`PVEC_SUBR` = 18，枚举 `pvec_type` 逐项计数；`PSEUDOVECTOR_AREA_BITS` =
12 + 12，`lisp.h` 1029-1039）。Rust 侧探针用同一组调用（1 参 / `MANY` / `UNEVALLED` /
0 参 / `noreturn`）。两侧打印 16 个字段：

| 字段 | 值 | 备注 |
| --- | --- | --- |
| `sizeof` / `alignof` | 56 / 8 | 与 C 编译器冻结基线 `c-layout-darwin-arm64.tsv` 一致 |
| `header.size` | `0x12000000` | `18 << 24` |
| `function` 成员 | `a1` / `aMANY` / `aUNEVALLED` / `a0` | C 的 `{.a ## maxargs}` |
| `min_args`/`max_args` | `1/1`、`0/-2`、`1/-1` | `MANY = -2`、`UNEVALLED = -1` |
| `symbol_name` | `"probe-1"` | NUL 结尾，字节一致 |
| `intspec` | `NULL` / `"p"` / `""`（空串）/ `NULL` | 三种拼法都对 |
| `command_modes` | 0 | C 的 `lisp_h_Qnil` |
| `doc` | 0 | 与 C 一样留待 snarf |

`diff` 结果：16/16 一致（唯一差异是 C 打印 `1`、Rust 打印 `true`）。

附带发现：C 探针**必须**先有 `EXFUN` 形式的前向声明（真实构建里来自生成的
`globals.h`），否则 `DEFUN` 宏在静态初值里引用自己的函数名会报 undeclared ——
这正是本项目"原型一律来自生成的 `subrs` 模块、镜像里不许再手写"（PORTING 0.1-10）
的实证。

## 修掉的镜像缺陷

1. **`MANY`/`UNEVALLED` 写反了**：参照是 `MANY = -2, UNEVALLED = -1`（`lisp.h`
   3448-3449），旧镜像写成 `UNEVALLED = -2, MANY = -1`。这会让
   `eassert (nargs >= UNEVALLED)` 之类的判定变松，也会让 `DEFUN` 记录的
   `max_args` 错。
2. **`defsubr` 的参数类型**：C 是 `union Aligned_Lisp_Subr *`（且 `defsubr` 会写记录的
   `header.size`），旧镜像写 `*const Lisp_Subr`；现在镜像写 `*mut Aligned_Lisp_Subr`，
   调用点写 `defsubr (&raw mut Sfoo);`。
3. **`lisp_h_Qnil` 必须是常量表达式**（C 的 `lisp_h_Qnil` 是宏），否则静态初值里
   用不了；改成 `const fn`（`null_mut()`，与 `NIL_IS_ZERO` 一致）。
4. **宏名必须是 `DEFUN`**（C 的宏名逐字，PORTING 0.2），旧镜像里那个 `DEFUN!`
   是未展开过的草稿（`{.a0 = None}`、`#[no_mangle]`），已按上述替换；
   调用形状 = C 的字段顺序 + **函数体作为最后一个实参**（Rust 的宏调用不能被
   紧跟的项续写），`attributes: noreturn` 写在 `doc:` 之后、决定定义返回 `!`。

## P-28 闸门（`tools/extract-abi.py`）

Rust 里"定义遮蔽整体导入"（PORTING 0.1-10）意味着 C 的"原型 ↔ 定义"类型检查消失，
所以闸门替它做三件事：

1. `DEFUN!` 定义的**参数个数**必须等于参照 `maxargs`（`MANY`→2、`UNEVALLED`→1）；
2. `attributes: noreturn` **双向**必须与参照的 `flags` 一致（定义必须是 `-> !`）；
3. crate 根**不许**出现 subr 名的 `pub fn …;` 声明（它们的原型只来自生成的
   `pub mod subrs`，否则定义该 subr 的 TU 会 `E0428`）。

合成用例（`/tmp/p28/`，一条 C 文件定义 3 个 subr）：

| 用例 | 结果 |
| --- | --- |
| 完全一致 | `OK` |
| `probe-one` 定义成 0 参（参照 1 参） | `FAIL`：`0 parameters, maxargs 1 needs 1` |
| 去掉 `attributes: noreturn` | `FAIL`：`attributes noreturn missing` |
| 额外加一行 crate 根 `pub fn Fprobe_one (…) -> …;` | `FAIL`：`declared at crate root … (E0428)` |

## 回归

`--check emacs/src/sort.c femacs/src/sort.rs` → `OK`；
`--check emacs/src/lisp.h femacs/src/include/lisp.rs` → `OK`（`DEFUN`/`MANY`/
`UNEVALLED`/`defsubr` 都在清单里）；
`xtask gate-fast` → 退出码 0（清单 0 差异 / cargo check / layout-check）；
`--all` → 2 done, 0 failing。

## 评审与处置（两轮，2026-10-02）

### 评审 A（ABI/差分）——两个阻断项，均已修

1. **noreturn 记录根本编不过（P0）**。`attributes: noreturn` 让定义变成 `-> !`，
   而 `fn(…) -> !` **不能**存进 `fn(…) -> Lisp_Object` 的联合成员（`E0308`）——
   8 个 noreturn subr（`Fsignal`/`Fthrow`/`Fkill_emacs`/…）一个都定义不了。
   评审同时纠正了对 C 的理解：`_Noreturn` 只写在**声明**（`globals.h` 的 EXFUN）上，
   不是类型的一部分，所以 C 里没有这个冲突。
   **处置**：新增 `subr_function_pointer!`，只在**记录**里把指针打成 C 的成员类型
   （等价于 C 的联合初值 `{.a ## maxargs = fnname}`），定义保持 `-> !` 以便与生成的
   原型同型、可被 P-28 校验。实测：noreturn 记录在 bin/lib 两种 crate 下都编译，
   且 `address_preserved=true`（记录里的地址 == 函数地址）。
   代价：宏内部 `$ret` 必须作为 `tt` 传递（`:ty` 片段无法被嵌套宏的字面量分支再匹配）。
2. **闸门不编译它认证的东西（P0）**：`--check` 是纯文本规则，所以一个 rustc 拒绝的
   文件也能 `OK`。**处置**：`xtask gate-fast` 增加"编译每个**已完成** TU"
   （清单无未勾选项即完成，见 `docs/ports/README.md`），文本规则退化为预过滤。
   现在 `gate-fast` 会 `built src/sort.rs`。

### 评审 B（闸门可绕过性）——六个洞，均已修

| # | 洞 | 处置 |
| --- | --- | --- |
| 1 | `DEFUN!` 调用里插一行注释即可让该 subr **整个**从 P-28 表里消失（arity 与 noreturn 都不再检查） | 匹配前先做"去注释 + 折叠空白（字符串字面量逐字保留）"，并对"参照有 DEFUN 但没有对应定义 / 定义了两次 / 参照没有的 DEFUN"逐条报错 |
| 2 | `DEFUN!` 的**函数体**不在桩检测范围内（`unimplemented!()` 能过） | `subr_defs` 用花括号配对（跳过字面量）取出函数体并纳入桩检测 |
| 3 | `maxargs` 不可解析时闸门**崩溃**而不是报错 | 返回问题条目 |
| 4 | crate 根声明的 lint 用被包含文本（含 `globals.rs` 的 1,736 条原型）→ 若 C 文件有 DEFUN 就误报 | 只看文件自身的折叠文本，并放宽到 `pub(...)`/`unsafe`/`extern "C"`/跨行 |
| 5 | 参数只比个数、不比类型（`MANY` 写成两个 `Lisp_Object` 也过） | 按 C 的 `DEFUN_ARGS_*` 逐参数比类型 |
| 6 | 交互规格被"折叠空白"比较 → prompt 里的多个空格（有语义）可被吃掉；docstring 的 `\xNN`/`\u{…}` 逃逸解码错误会误判 | 两侧都按**值**比较（C 的行拼接只删换行、Rust 的行延续连缩进一起吃掉——所以多行规格必须写成一行）；逃逸按 Rust 的完整语法解码；`normalise` 只在字面量**之外**折叠空白 |
| 7（追加） | 没有任何规则要求 `defsubr` 注册 | 按**参照文件自己的** `defsubr` 集合双向对齐（这自动覆盖 P-31 的两个例外） |

### 攻击用例复核

评审 B 留下的 57 个用例（`/tmp/p28b`，一条 C 文件 + 多种伪造 Rust）全部重跑：
对照组 `ok.rs` → `OK`；其余 16 个攻击（参数类型、重复定义、注释隐藏、`unimplemented!`、
`todo!()`、prompt 空格、`\x41` docstring、符号 `maxargs`、crate 根声明四种拼法、
`MANY` 参数类型、手写定义……）→ 全部 `FAIL`。
另外用**忠实拼写**的多行规格（把 C 拼接后的值写成一行 + `\t`）验证 `OK`，
证明规则没有"过严到无法通过"。
