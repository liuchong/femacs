# emacs/lib/regex_internal.c (1718 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 34
- [x] re_string_allocate  (line 47)
- [x] re_string_construct  (line 75)
- [x] re_string_realloc_buffers  (line 134)
- [x] re_string_construct_common  (line 172, static)
- [x] build_wcs_buffer  (line 202, static)
- [x] build_wcs_upper_buffer  (line 272)
- [x] re_string_skip_chars  (line 489, static)
- [x] build_upper_buffer  (line 529, static)
- [x] re_string_translate_buffer  (line 548, static)
- [x] re_string_reconstruct  (line 569)
- [x] re_string_peek_byte_case  (line 818)
- [x] re_string_fetch_byte_case  (line 848, static)
- [x] re_string_destruct  (line 883, static)
- [x] re_string_context_at  (line 894, static)
- [x] re_node_set_alloc  (line 935)
- [x] re_node_set_init_1  (line 947)
- [x] re_node_set_init_2  (line 963)
- [x] re_node_set_init_copy  (line 993)
- [x] re_node_set_add_intersect  (line 1018)
- [x] re_node_set_init_union  (line 1109)
- [x] re_node_set_merge  (line 1162)
- [x] re_node_set_insert  (line 1249)
- [x] re_node_set_insert_last  (line 1312)
- [x] re_node_set_compare  (line 1335)
- [x] re_node_set_contains  (line 1350)
- [x] re_node_set_remove_at  (line 1371, static)
- [x] re_dfa_add_node  (line 1385, static)
- [x] calc_state_hash  (line 1435, static)
- [x] re_acquire_state  (line 1455)
- [x] re_acquire_state_context  (line 1503)
- [x] register_state  (line 1544)
- [x] free_state  (line 1579, static)
- [x] create_ci_newstate  (line 1599)
- [x] create_cd_newstate  (line 1647)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- 闸门实测：`python3 tools/extract-abi.py --check emacs/lib/regex_internal.c lib/regex_internal.rs` → **OK**；
  `cargo run -q -p xtask -- build-tu lib/regex_internal.c` → **0 error**（23 warning，均为 allow 范围内的
  non_snake_case/dead_code 之类）。
- **闸门/工具修复（本轮附带，非本文件内容）**：`tools/cinventory.py` 的 `function_spans` 取"定义行上方
  第一个非空行"判 `static`；本文件 40-46 行的形状
  `static reg_errcode_t` / `__attribute_warn_unused_result__` / `name (…)` 让它把**属性行**当成声明，
  于是 22 个 C 的 `static` 函数被判成非 static，检查器要求它们带 `#[unsafe(no_mangle)]`（与 PORTING 0.2
  的"static 不出符号"冲突，且会真的导出符号）。已修：回看时跳过纯属性行（P-77 的同类）。
  没有这个修复，本文件不可能既合规又过闸门。
- **`_LIBC` 臂处置（P-51；本文件不在 cond-eval-verified.txt，闸门把两条臂都当活）**：
  - `build_wcs_buffer`（C 204-212）`unsigned char buf[MB_LEN_MAX]` + `DEBUG_ASSERT` → `#[cfg (feature = "libc")]` 死文本；
    活体是 `[u8; 64]`。
  - `build_wcs_upper_buffer`（C 277-283）`char buf[MB_LEN_MAX]` + `DEBUG_ASSERT` → 同上，活体 `[c_char; 64]`。
  - `re_string_reconstruct`（C 706-720）UTF-8 的 `_LIBC` 快路（`isascii (*p)`+`__glibc_likely`）→ `#[cfg (feature = "libc")]`；
    注意 C 把它与活体的 `for (…; p >= end; --p)` 用 `else` 粘在一起，Rust 无法跨 `#[cfg]` 写 `else`，
    故死臂写成完整的 `if`、活体 `while` 紧随（`else`/`for` 都在 MACRO_TOKEN_SKIP，不需要拼）。
  - `re_acquire_state` / `re_acquire_state_context`（C 1424-1428、1490-1494）`#if defined GCC_LINT || defined lint`
    的 `*err = REG_NOERROR;` → `#[cfg (feature = "gcc_lint")]` 死文本。
  - 其余 `_LIBC` 出现处只影响 `#include`，端口用 `include!` 各自镜像表达，不产生死文本。
- **notes 条目**（`docs/ports/macro-body-notes.tsv`）：`build_wcs_upper_buffer` 的 `goto`（C 的
  `goto offsets_needed` 跳进另一条臂的循环体中部，端口用 `offsets_path` 模式标志合成一个循环，PORTING 2.2，
  先例 `mpz_get_str`）；`create_ci_newstate`/`create_cd_newstate` 的 `re_token_type_t`（C 的局部声明类型名，
  端口把 8 位字段当整数读 `type_ (node)`，本版 Rust 不允许整数 `as` 枚举）。
- **位域（1.3）**：`re_token_t`（`type:8`/`constraint:10`/`duplicated:1`/`opt_subexp:1`/`accept_mb:1`/
  `mb_partial:1`/`word_char:1`）、`re_dfastate_t`（`context:4`/`halt:1`/`accept_mb:1`/`has_backref:1`/
  `has_constraint:1`）、`re_dfa_t`（`has_plural_match`/`has_mb_node`/`is_utf8`/`map_notascii`/`word_ops_used`）
  各折成一个 `bits` 整数字段；镜像给出 `type_`/`set_constraint_`/`state_accept_mb_` … 访问器与 `*_SHIFT`/`*_MASK`
  常量，`docs/ports/token-aliases.tsv` 为 `type`/`constraint`/`accept_mb`/`halt`/`has_backref`/`has_constraint`/`context`
  登记家族（带出现次数下界）。**未验证项**：位域在存储单元内的**位序**不在布局断言覆盖内（P-35），
  本端口按小端 LSB-first 写，需 `spikes/*-probe.sh` 的 C 对拍来钉死。
- **行数漂移**：C 1,718 vs 端口 1,843（+7.3%），已记 `docs/ports/line-drift-tolerance.tsv`（`regex_internal` 0.10）；
  原因：位域访问器调用点（C 的 `node->type` 是一处字段读，端口是一处函数调用）、两个 `#[cfg]` 死文本块、
  以及单元头部的 `use`/`include!`。
- **C 的 `static` 前向声明**（C 21-38）按 0.1-12 删除（Rust 无对应物），定义保留在 C 的位置。
- **未初始化局部量（1.5）**：`wchar_t wc`/`wc2`、`mbstate_t cur_state` 由被调函数写出，端口给 `0` 初值。
- **`re_realloc`/`re_malloc` 的计数**：C 的 `n * sizeof (T)` 在 Rust 里必须 `n as usize * size_of::<T> ()`
  （宏内完成，调用点形状不变）。
