# emacs/lib/regex_internal.h (850 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 12
- [x] bitset_set  (line 722, static)
- [x] bitset_clear  (line 728, static)
- [x] bitset_contain  (line 734, static)
- [x] bitset_empty  (line 740, static)
- [x] bitset_set_all  (line 746, static)
- [x] bitset_copy  (line 755, static)
- [x] bitset_not  (line 761, static)
- [x] bitset_merge  (line 773, static)
- [x] bitset_mask  (line 781, static)
- [x] re_string_char_size_at  (line 791)
- [x] re_string_wchar_at  (line 804)
- [x] re_string_elem_size_at  (line 817)

## Macros defined — 90
- [x] ASCII_CHARS  (line 111, body 70070b762760)
- [x] BIN_TREE_STORAGE_SIZE  (line 490, body 23dccaa5565c)
- [x] BITSET_WORDS  (line 219, body ea314ef16656)
- [x] BITSET_WORD_BITS  (line 216, body 786eaef2a1da)
- [x] BITSET_WORD_MAX  (line 214, body 7531eda0d2d7)
- [x] COLL_ELEM_LEN_MAX  (line 116, body fe5dbbcea5ce)
- [x] CONTEXT_BEGBUF  (line 502, body ecbba6a71427)
- [x] CONTEXT_ENDBUF  (line 503, body 884afc219602)
- [x] CONTEXT_NEWLINE  (line 501, body 12c15aebf7ea)
- [x] CONTEXT_WORD  (line 500, body 356a192b7913)
- [x] DEBUG_ASSERT  (line 45, body d925f8edceaf)
- [x] EPSILON_BIT  (line 272, body fe5dbbcea5ce)
- [x] FALLTHROUGH  (line 844, body a29646b15239)
- [x] IDX_MAX  (line 204, body 58dc0e80ae67)
- [x] IS_BEGBUF_CONTEXT  (line 507, body dc31faca742d)
- [x] IS_ENDBUF_CONTEXT  (line 508, body 2e13c6ea7d36)
- [x] IS_EPSILON_NODE  (line 377, body ff81b1bbfa5c)
- [x] IS_IN  (line 439, body 7cb6efb98ba5)
- [x] IS_NEWLINE  (line 512, body 49f273e9b115)
- [x] IS_NEWLINE_CONTEXT  (line 506, body 10e3a829a4e2)
- [x] IS_ORDINARY_CONTEXT  (line 509, body 746734e3e5fe)
- [x] IS_WIDE_NEWLINE  (line 514, body 29b309e9a043)
- [x] IS_WIDE_WORD_CHAR  (line 513, body 2b99e919346a)
- [x] IS_WORD_CHAR  (line 511, body 3e1b6f31c032)
- [x] IS_WORD_CONTEXT  (line 505, body 76f5eb09f626)
- [x] MAX  (line 460, body 42e190a85efa)
- [x] MIN  (line 463, body 7c03c63086ce)
- [x] NEWLINE_CHAR  (line 119, body 20b0345de3d0)
- [x] NEXT_ENDBUF_CONSTRAINT  (line 232, body 3be01b55ecdc)
- [x] NEXT_NEWLINE_CONSTRAINT  (line 230, body 19fbaf699b05)
- [x] NEXT_NOTWORD_CONSTRAINT  (line 228, body 92df71781c5c)
- [x] NEXT_WORD_CONSTRAINT  (line 227, body eeed498aee81)
- [x] NOT_SATISFY_NEXT_CONSTRAINT  (line 522, body fa6c0020a679)
- [x] NOT_SATISFY_PREV_CONSTRAINT  (line 516, body af48ab61c991)
- [x] NOT_WORD_DELIM_CONSTRAINT  (line 234, body d228a45608c3)
- [x] PREV_BEGBUF_CONSTRAINT  (line 231, body 7232895c5fbc)
- [x] PREV_NEWLINE_CONSTRAINT  (line 229, body d73afeb43f0b)
- [x] PREV_NOTWORD_CONSTRAINT  (line 226, body 5f399b2ba212)
- [x] PREV_WORD_CONSTRAINT  (line 225, body f2b1649be8f5)
- [x] REGEX_COB128  (line 178, body 74989d6aaff7)
- [x] REGEX_COB16  (line 181, body 9b4d13052104)
- [x] REGEX_COB32  (line 180, body 69fa8bc25086)
- [x] REGEX_COB4  (line 183, body 5383497b4935)
- [x] REGEX_COB64  (line 179, body 7b86d72a4351)
- [x] REGEX_COB8  (line 182, body 3e6f546f81d0)
- [x] REGEX_UINTEGER_WIDTH  (line 177, body 18b025c7328b)
- [x] SBC_MAX  (line 114, body f7c0435cad1b)
- [x] ULONG_WIDTH  (line 171, body 3bd7f75b4567)
- [x] WIDE_NEWLINE_CHAR  (line 120, body a2938778fa5e)
- [x] WORD_DELIM_CONSTRAINT  (line 233, body ecdf501256ce)
- [x] _REGEX_INTERNAL_H  (line 21, body 356a192b7913)
- [x] _RE_DEFINE_LOCALE_FUNCTIONS  (line 93, body 356a192b7913)
- [x] __btowc  (line 159, body 579257467ba3)
- [x] __iswalnum  (line 155, body 4b3cdd8ba391)
- [x] __iswctype  (line 156, body 1a64808f1184)
- [x] __mbrtowc  (line 160, body 35b4b1d73ef3)
- [x] __regfree  (line 143, body 0a0978dcc939)
- [x] __towlower  (line 157, body bff4fecb51f4)
- [x] __towupper  (line 158, body 02bdfa87e259)
- [x] __wcrtomb  (line 161, body c86109e360f1)
- [x] __wctype  (line 154, body 27d6321ecfc6)
- [x] gettext  (line 103, body dab3eb70474a)
- [x] gettext_noop  (line 105, body 3df63b7acb05)
- [x] isascii  (line 88, body 5d0a6e13a236)
- [x] isblank  (line 80, body e028b5081ee0)
- [x] lock_define  (line 70, body da39a3ee5e6b)
- [x] lock_fini  (line 72, body a29646b15239)
- [x] lock_init  (line 71, body b6589fc6ab0d)
- [x] lock_lock  (line 74, body 79b9a7a6337e)
- [x] lock_unlock  (line 75, body a29646b15239)
- [x] re_free  (line 472, body ed3eb07fd6ed)
- [x] re_malloc  (line 469, body 630a64fa9a9e)
- [x] re_node_set_empty  (line 694, body c7bc74d24985)
- [x] re_node_set_free  (line 695, body b60e96ca38b2)
- [x] re_node_set_init_empty  (line 691, body 85c2460e0416)
- [x] re_node_set_remove  (line 692, body beb2ed045688)
- [x] re_realloc  (line 471, body 36508933ea89)
- [x] re_string_byte_at  (line 455, body ab2498e3e300)
- [x] re_string_cur_idx  (line 452, body 719673a94621)
- [x] re_string_eoi  (line 451, body 4b37a58c59dc)
- [x] re_string_fetch_byte  (line 444, body 168816d88f5e)
- [x] re_string_first_byte  (line 446, body 3041cca1cb8d)
- [x] re_string_get_buffer  (line 453, body 6709edbcc400)
- [x] re_string_is_single_byte_char  (line 448, body 989b6125805d)
- [x] re_string_length  (line 454, body 955b06315952)
- [x] re_string_peek_byte  (line 442, body ea5469293c45)
- [x] re_string_set_index  (line 457, body 28856c378990)
- [x] re_string_skip_bytes  (line 456, body bb440342337a)
- [x] wchar_t  (line 129, body 5a100eb0c44e)
- [x] wctype_t  (line 130, body fd6cfc7c12d9)

## Types and their fields — 11
- [x] <anon@354>: unsigned char c, re_bitset_ptr_t sbcset, re_charset_t* mbcset, Idx idx, re_context_type ctx_type
- [x] <anon@710>: unsigned char ch, unsigned char* name, wchar_t wch
- [x] bin_tree_storage_t: struct bin_tree_storage_t* next, bin_tree_t data
- [x] bin_tree_t: struct bin_tree_t* parent, struct bin_tree_t* left, struct bin_tree_t* right, struct bin_tree_t* first, struct bin_tree_t* next, re_token_t token, Idx node_idx
- [x] re_backref_cache_entry: Idx node, Idx str_idx, Idx subexp_from, Idx subexp_to, bitset_word_t eps_reachable_subexps_map, char more
- [x] re_dfa_t: re_token_t* nodes, size_t nodes_alloc, size_t nodes_len, Idx* nexts, Idx* org_indices, re_node_set* edests, re_node_set* eclosures, re_node_set* inveclosures, struct re_state_table_entry* state_table, re_dfastate_t* init_state, re_dfastate_t* init_state_word, re_dfastate_t* init_state_nl, re_dfastate_t* init_state_begbuf, bin_tree_t* str_tree, bin_tree_storage_t* str_tree_storage, re_bitset_ptr_t sb_char, int str_tree_storage_idx, re_hashval_t state_hash_mask, Idx init_node, Idx nbackref, bitset_word_t used_bkref_map, bitset_word_t completed_bkref_map, unsigned int has_plural_match : 1, unsigned int has_mb_node : 1, unsigned int is_utf8 : 1, unsigned int map_notascii : 1, unsigned int word_ops_used : 1, int mb_cur_max, bitset_t word_char, reg_syntax_t syntax, Idx* subexp_map, char* re_str
- [x] re_dfastate_t: re_hashval_t hash, re_node_set nodes, re_node_set non_eps_nodes, re_node_set inveclosure, re_node_set* entrance_nodes, struct re_dfastate_t** trtable, struct re_dfastate_t** word_trtable, unsigned int context : 4, unsigned int halt : 1, unsigned int accept_mb : 1, unsigned int has_backref : 1, unsigned int has_constraint : 1
- [x] re_fail_stack_ent_t: Idx idx, Idx node, regmatch_t* regs, re_node_set eps_via_nodes
- [x] re_fail_stack_t: Idx num, Idx alloc, struct re_fail_stack_ent_t* stack
- [x] re_state_table_entry: Idx num, Idx alloc, re_dfastate_t** array
- [x] re_string_t: const unsigned char* raw_mbs, unsigned char* mbs, wint_t* wcs, Idx* offsets, mbstate_t cur_state, Idx raw_mbs_idx, Idx valid_len, Idx valid_raw_len, Idx bufs_len, Idx cur_idx, Idx raw_len, Idx len, Idx raw_stop, Idx stop, unsigned int tip_context, RE_TRANSLATE_TYPE trans, re_const_bitset_ptr_t word_char, unsigned char icase, unsigned char is_utf8, unsigned char map_notascii, unsigned char mbs_allocated, unsigned char offsets_needed, unsigned char newline_anchor, unsigned char word_ops_used, int mb_cur_max

## File-scope objects — 0

## Decisions

- **配置分支（P-08）**：`_LIBC` 未定义、`_REGEX_AVOID_UCHAR_H`=1、`_REGEX_LARGE_OFFSETS`=1、
  `HAVE_ISBLANK`/`HAVE_DECL_ISBLANK`=1、无 `GNULIB_LOCK`/`GNULIB_PTHREAD`/
  `GNULIB_REGEX_SINGLE_THREAD`、`DEBUG` 未定义。于是 `lock_*` 取最后一支（空/`dfa` 占位）、
  `Idx`=`regoff_t`=`ssize_t`（`isize`）、`IDX_MAX`=`SSIZE_MAX`、宽字符是 C 的
  `wchar_t`/`wint_t`（`__mbrtowc`→`mbrtowc` 一族），`re_charset_t` 的三个 glibc 成员
  （`coll_syms`/`equiv_classes`/`uint32_t` ranges）不在冻结构建里。
- **死分支文本（P-51）**：`#ifndef ULONG_WIDTH` 的 `REGEX_COB*` 链、`re_string_elem_size_at`
  的 `_LIBC` 块、`#ifdef DEBUG char *re_str`、`wchar_t`/`wctype_t` 的 `#else` 臂都保留文本；
  会改布局或另起定义的两处（`re_str` 字段、glibc 块）用 `#[cfg]` 关掉编译，其余按原样列出。
- **位域（1.3）**：`re_token_t` 的 7 个位域（`type:8` + 6 个 flag，共 23 位）折成 `bits: c_uint`；
  `re_dfastate_t` 的 5 个（8 位）与 `re_dfa_t` 的 5 个各折成一个 `bits: c_uint`；
  `re_charset_t.non_match:1` 折成 `bits: c_uint`。C 声明逐条列在字段上方的 `///` 里。
- **枚举**：`re_context_type`/`re_token_type_t`/`bracket_elem_type` 用 `#[repr(C)]` + 显式判别值；
  判别值里的 `PREV_*`/`EPSILON_BIT` 常量是 `c_int`，Rust 的判别值类型是 `isize`，故逐项
  `as isize`（值不变）。unions（`re_token_t.opr`、`bracket_elem_t.opr`）用 `#[repr(C)] union`，
  其枚举成员加 `Copy`（0.1-9）。
- **类型宽度**：`SBC_MAX`/`BITSET_WORD_BITS` 用 `Idx`（C 里的 `int` 会在 `i / BITSET_WORD_BITS`
  处做整型提升，Rust 不提升，故把常量声明成 `Idx` 以免每次使用处加 cast）；`bitset_*` 的循环
  变量 `bitset_i`、`re_string_char_size_at` 的 `byte_idx` 同理用 `Idx`。
- **指针索引**：C 的下标写法 `p[i]` 在 Rust 的裸指针上是 `E0608`，一律写成 `*p.add (i)`
  （`re_string_peek_byte`/`fetch_byte`/`byte_at`/`wchar_at`）——同一条语句、同一个下标表达式。
- **宏体例外**：`ULONG_WIDTH` 的 C 体重定义是死的（冻结构建的 `<limits.h>` 已定义它），
  port 的值在 `limits.in.rs`；记入 `docs/ports/macro-body-notes.tsv`（唯一的例外行）。
- **`static inline` 定义**：12 个（`bitset_*` 9 + `re_string_*_at` 3）在镜像里有定义体——它们
  不产生 C 符号（P-12/P-37），`__attribute__ ((pure, unused))` 记为无操作（8）。
- **新增支撑镜像**：`wchar.rs`/`wctype.rs`/`ctype.rs`/`string.rs`/`langinfo.rs`（系统头）、
  `attribute.rs`/`verify.rs`/`gettext.rs`（gnulib 头，WIP 镜像，各自有台账）。`gettext` 的
  `_(Msgid)` 因 `_` 不是 Rust 标识符改名 `gettext!`（0.2）。
