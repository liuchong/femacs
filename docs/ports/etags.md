# emacs/lib-src/etags.c (8280 lines)

# STATUS (read this first).  **The C body is fully carried over: C 143-8280 (the whole file)
# is ported and the unit compiles.**  `cd femacs && CARGO_TARGET_DIR=/tmp/… cargo run -q -p
# xtask -- build-tu lib-src/etags.c` ends `built lib-src/etags.rs ->
# ../build/femacs/etags.o`, and the manual
# `rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name et2_check -C
# panic=abort -C codegen-units=1 -C opt-level=0 build/femacs/units/etags/_root.rs`
# exits 0.  38 warnings: 24 `unnecessary parentheses` in the earlier window's `print_help`
# and in the `regex.rs` mirror, 12 C-faithful `value assigned … is never read`, and 2
# `suspicious definition of the runtime read/write symbol` from the mirrors; none in this
# window's own new code.
#
# The window order is C's own.  The previous revision of this block said the 31 language
# parsers whose addresses `lang_names` (C 851) takes had to be *moved* before that table;
# that was wrong.  Rust items are order-independent (PORTING.md 0.1-12), so the port drops C
# 334-417's forward declarations and leaves **every definition at its C position** — the unit
# was uncompilable before this window only because those 31 definitions were still missing,
# not because of where they sit.  The last window is C 1660-8280: `process_file_name`,
# `process_file`, `reset_input`, `find_entries`, the whole tag/tree machinery (`make_tag` …
# `in_word_set`), `C_symtype`, `consider_token`, `make_C_tag`, `C_entries` and its six
# wrappers, the 31 parsers, the regex machinery (`scan_separators` … `out`), `readline`, the
# `savestr`/`skip_*`/`fatal`/`error`/`concat` helpers, the path helpers and the allocators.
#
# Boxes ticked: 126 of 126 functions, 29 of 29 macros, 7 of 12 file-scope objects.  The five
# remaining rows (`constant code`, `create`, `defer`, `value`, `variable`, all "line 653")
# are **not objects**: they are the words of `Forth_help`'s one C string literal, whose
# backslash-newline continuations `cinventory`'s literal blanker does not see, so the
# comma-separated words parse as declarators.  They are integrator note rows, not port work
# (see "Owed note rows" at the end).

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 126
- [x] memcpyz  (line 145, static)
- [x] strcaseeq  (line 152, static)
- [x] strneq  (line 158, static)
- [x] strncaseeq  (line 164, static)
- [x] notinname  (line 171, static)
- [x] begtoken  (line 183, static)
- [x] intoken  (line 203, static)
- [x] endtoken  (line 224, static)
- [x] print_language_names  (line 896, static)
- [x] print_version  (line 929, static)
- [x] print_help  (line 943, static)
- [x] main  (line 1143)
- [x] cleanup_tags_file  (line 1494, static)
- [x] get_compressor_from_suffix  (line 1544, static)
- [x] get_language_from_langname  (line 1580, static)
- [x] get_language_from_interpreter  (line 1602, static)
- [x] get_language_from_filename  (line 1624, static)
- [x] process_file_name  (line 1666, static)
- [x] process_file  (line 1819, static)
- [x] reset_input  (line 1889, static)
- [x] find_entries  (line 1900, static)
- [x] make_tag  (line 2073, static)
- [x] pfnote  (line 2119, static)
- [x] push_node  (line 2187, static)
- [x] pop_node  (line 2200, static)
- [x] free_tree  (line 2220, static)
- [x] free_fdesc  (line 2259, static)
- [x] add_node  (line 2279, static)
- [x] invalidate_nodes  (line 2385, static)
- [x] number_len  (line 2457, static)
- [x] total_size_of_entries  (line 2472, static)
- [x] put_entry  (line 2492, static)
- [x] put_entries  (line 2565, static)
- [x] hash  (line 2705, static)
- [x] in_word_set  (line 2751, static)
- [x] C_symtype  (line 2820, static)
- [x] pushclass_above  (line 2969, static)
- [x] popclass_above  (line 2988, static)
- [x] write_classname  (line 3000, static)
- [x] consider_token  (line 3052, static)
- [x] make_C_tag  (line 3424, static)
- [x] perhaps_more_input  (line 3443, static)
- [x] C_entries  (line 3456, static)
- [x] default_C_entries  (line 4393, static)
- [x] plain_C_entries  (line 4400, static)
- [x] Cplusplus_entries  (line 4407, static)
- [x] Cjava_entries  (line 4414, static)
- [x] Cstar_entries  (line 4421, static)
- [x] Yacc_entries  (line 4428, static)
- [x] just_read_file  (line 4458, static)
- [x] F_takeprec  (line 4471, static)
- [x] F_getit  (line 4494, static)
- [x] Fortran_functions  (line 4518, static)
- [x] Go_functions  (line 4609, static)
- [x] Ada_getit  (line 4680, static)
- [x] Ada_funcs  (line 4742, static)
- [x] Asm_labels  (line 4855, static)
- [x] Perl_functions  (line 4888, static)
- [x] Python_functions  (line 4998, static)
- [x] Ruby_functions  (line 5021, static)
- [x] Rust_entries  (line 5200, static)
- [x] PHP_functions  (line 5245, static)
- [x] Cobol_paragraphs  (line 5324, static)
- [x] Makefile_targets  (line 5352, static)
- [x] Pascal_functions  (line 5387, static)
- [x] L_getit  (line 5567, static)
- [x] Lisp_functions  (line 5583, static)
- [x] Lua_functions  (line 5643, static)
- [x] PS_functions  (line 5685, static)
- [x] Forth_words  (line 5715, static)
- [x] Scheme_functions  (line 5772, static)
- [x] TeX_commands  (line 5829, static)
- [x] TEX_decode_env  (line 6043, static)
- [x] Texinfo_nodes  (line 6093, static)
- [x] HTML_labels  (line 6116, static)
- [x] Prolog_functions  (line 6240, static)
- [x] prolog_skip_comment  (line 6275, static)
- [x] prolog_pr  (line 6301, static)
- [x] prolog_atom  (line 6331, static)
- [x] test_objc_is_mercury  (line 6430, static)
- [x] Mercury_functions  (line 6554, static)
- [x] mercury_skip_comment  (line 6596, static)
- [x] memstreq  (line 6661, static)
- [x] mercury_decl  (line 6667, static)
- [x] mercury_pr  (line 6828, static)
- [x] Erlang_functions  (line 6897, static)
- [x] erlang_func  (line 6955, static)
- [x] erlang_attribute  (line 6991, static)
- [x] erlang_atom  (line 7021, static)
- [x] scan_separators  (line 7059, static)
- [x] analyze_regex  (line 7111, static)
- [x] add_regex  (line 7180, static)
- [x] substitute  (line 7292, static)
- [x] free_regexps  (line 7338, static)
- [x] regex_tag_multiline  (line 7360, static)
- [x] nocase_tail  (line 7439, static)
- [x] get_tag  (line 7454, static)
- [x] get_lispy_tag  (line 7473, static)
- [x] readline_internal  (line 7503, static)
- [x] readline  (line 7574, static)
- [x] savestr  (line 7767, static)
- [x] savenstr  (line 7777, static)
- [x] skip_spaces  (line 7786, static)
- [x] skip_non_spaces  (line 7795, static)
- [x] skip_name  (line 7804, static)
- [x] fatal  (line 7814, static)
- [x] pfatal  (line 7824, static)
- [x] suggest_asking_for_help  (line 7831, static)
- [x] error  (line 7840, static)
- [x] verror  (line 7849, static)
- [x] concat  (line 7859, static)
- [x] etags_getcwd  (line 7871, static)
- [x] etags_mktmp  (line 7891, static)
- [x] escape_shell_arg_string  (line 7945, static)
- [x] do_move_file  (line 7987, static)
- [x] relative_filename  (line 8029, static)
- [x] absolute_filename  (line 8068, static)
- [x] absolute_dirname  (line 8132, static)
- [x] filename_is_absolute  (line 8151, static)
- [x] canonicalize_filename  (line 8163, static)
- [x] linebuffer_init  (line 8205, static)
- [x] linebuffer_setlen  (line 8215, static)
- [x] memory_full  (line 8229, static)
- [x] xmalloc  (line 8235, static)
- [x] xnmalloc  (line 8246, static)
- [x] xnrealloc  (line 8257, static)

## Macros defined — 29
- [x] CNL  (line 3411, body 299542b6568f)
- [x] CNL_SAVE_DEFINEDEF  (line 3402, body 1d6c391c7dc4)
- [x] C_AUTO  (line 2624, body a9ff286998ab)
- [x] C_EXT  (line 2619, body 3477bbd790ee)
- [x] C_JAVA  (line 2623, body 4afa4141942f)
- [x] C_LANG_NAMES_INDEX  (line 855, body da4b9237bacc)
- [x] C_PLAIN  (line 2620, body 9fd84d69cf9e)
- [x] C_PLPL  (line 2621, body 2b5af27eeb6f)
- [x] C_STAR  (line 2622, body c7ecf997c2c5)
- [x] DEBUG  (line 85, body 7cb6efb98ba5)
- [x] LOOKING_AT  (line 4441, body a0e99c026cf0)
- [x] LOOKING_AT_NOCASE  (line 4448, body 96662d9ac4cd)
- [x] LOOP_ON_INPUT_LINES  (line 4435, body 10962a7e84a6)
- [x] MERCURY_HEURISTICS_RATIO  (line 134, body 1b390cd54a0c)
- [x] MSDOS  (line 95, body 7cb6efb98ba5)
- [x] PRINT_UNDOCUMENTED_OPTIONS_HELP  (line 939, body 7cb6efb98ba5)
- [x] YACC  (line 2625, body 67e6c786c9b7)
- [x] cjava  (line 3400, body 978083aa2de9)
- [x] cplpl  (line 3399, body 00a1095d0579)
- [x] curlb  (line 3393, body 2b7fa7719869)
- [x] curlinepos  (line 3395, body 102415822bf6)
- [x] current_lb_is_new  (line 3390, body 229705100e41)
- [x] instruct  (line 2965, body fa250464ff73)
- [x] nestlev  (line 2963, body 7e98f7c57c36)
- [x] newlb  (line 3394, body b6bb52757b9d)
- [x] newlinepos  (line 3396, body 9d2475374dd4)
- [x] plainc  (line 3398, body 52e35cd6eec8)
- [x] switch_line_buffers  (line 3391, body bf51cfa425d1)
- [x] xrnew  (line 241, body de4e3b1cfbca)

## Types and their fields — 0

## File-scope objects — 12
- [x] constant code  (line 653)
- [x] constant create  (line 653)
- [x] constant defer  (line 653)
- [x] constant value  (line 653)
- [x] constant variable  (line 653)
- [x] static const char Go_suffixes  (line 661, static)
- [x] static bool with_mercury_definitions  (line 703, static)
- [x] static float mercury_heuristics_ratio  (line 704, static)
- [x] static const char objtag  (line 2891, static)
- [x] static bool is_mercury_type  (line 6393, static)
- [x] static bool is_mercury_quantifier  (line 6394, static)
- [x] static bool is_mercury_declaration  (line 6395, static)


The five `constant code`/`create`/`defer`/`value`/`variable` rows (all reported at C 653) are
the words of `Forth_help`'s one multi-line C string literal, which `cinventory`'s literal
blanker does not see; the port carries that message as one Rust literal, so they cannot
reappear.  They are owed note rows (see the end of this file), not port work.

## Decisions

- **Shape.** One unit, `femacs/lib-src/etags.rs`, in the `lib-src/emacsclient.rs` /
  `lib-src/make-docfile.rs` shape: `#![allow (…)]`, the `use std::ffi` list, the `include!`
  list in C's own order, then C's file-scope objects and functions in C's order, with `main`
  as `#[unsafe(no_mangle)] pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char)
  -> c_int`.  The `static` helpers stay plain `unsafe fn`; the 31 language parsers (whose
  addresses the `lang_names` table takes) and the three `_Noreturn` printers
  (`print_version`, `print_help`, `suggest_asking_for_help`) are `unsafe extern "C" fn`, the
  latter with `-> !` (PORTING.md 1.4/5.1).
- **Dead text (P-08), recorded not transcribed.** `python3 tools/check-cond-eval.py
  lib-src/etags.c` reports `18 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the live
  set is the frozen configuration's: `DEBUG` (C 81-86) is its `#else` arm (`false`), `MSDOS`
  (C 90-96) likewise, `WINDOWSNT` (C 98-105) is undefined (its `<direct.h>`/`#undef`/
  `O_CLOEXEC` block is dead), and `GNUC_PREREQ (14, 0, 0)` (C 139-141) is a preprocessor-only
  arm.  The same applies to the later `#if N` blocks: C 1465-1484's `#if defined WINDOWSNT ||
  MSDOS` arm is dead (the `#else`, the `sort -u -o '…'` command builder, is live), and so are
  C 1634-1637, C 1764-1783, C 7896-7911, C 7920-7930, C 8042-8045 (all on `DOS_NT`/`MSDOS`).
- **Symbols.** `main` is the only export; every other definition in the file is `static` in C,
  so the port keeps them private (no `#[unsafe(no_mangle)]`).
- **Use-site declarations (P-26/P-72 (3)).** `<stddef.h>`/`<stdint.h>`'s `ptrdiff_t`, `size_t`,
  `intmax_t`, `uintmax_t`; `mempcpy`, which the generated `<string.h>` renames
  to `rpl_mempcpy` (PORTING.md 10.13); `c_strcasecmp`/`c_strncasecmp` (`<c-strcase.h>`);
  `<stdlib.h>`'s `exit` and `system`; `<stdio.h>`'s `snprintf`; `<unistd.h>`'s `STDOUT_FILENO`
  (measured 1); `<config.h>`'s `PACKAGE_NAME`/`PACKAGE_VERSION`/`COPYRIGHT` (the frozen
  configuration's values, the way `src/emacs.rs` 435 carries them; the three literal rows are
  in `docs/baseline/declarations-allowlist.tsv`).  `assert` is Rust's `assert!` (the
  `lib/mini-gmp.rs` 8 precedent).  `<limits.h>`'s `CHAR_MAX` was a use-site declaration in the
  first revision of this window; the `lib/include/limits.in.rs` mirror now carries it (the
  mirror lane closed that gap on 2026-10-04), so the declaration was removed and
  `CTAGS_OPTION`/`STDIN_OPTION` read the mirror's constant.
- **Layout.** The seven types C defines in this file (`compressor`, `language`, `fdesc`,
  `node`, `linebuffer`, `argument`, `regexp`) are `#[repr(C)]` and carry the 29 `const _`
  assertions the port measured from clang against the frozen configuration (sizes 16/56/64/64/
  24/24/72; `offset_of` for their live fields) - the shape the other lib-src programs use,
  since these types are file-local and `tools/gen-layout-fields.py`'s list carries only the
  mirrored headers' types (P-34).
- **Deviations.**
  1. C 334-417's forward declarations are dropped (PORTING.md 0.1-12); the definitions stay at
     their C positions.  Rust resolves items regardless of order, so the port needs no
     declaration lines.  (The unit is uncompilable while any of those definitions is still
     missing, but *not* because of where they sit - see STATUS's correction of the earlier
     "the parsers must be moved before `lang_names`" note.)
  2. C's `static language *lang;` inside `main`'s loop is a function-scope `static` (it
     survives the iterations); the port keeps it as `static mut lang` in the same block.  The
     `static char const optstring[]` is the port's `static optstring: &CStr`.
  3. C's chained assignments (`typedefs = typedefs_or_cplusplus = constantypedefs = true;`,
     `globals = members = true;`) are written as the same three assignments in C's
     right-to-left order (Rust has no assignment expression value).
  4. `cleanup_tags_file`'s C parameter `tagfile` shadows the file-scope object of the same
     name; Rust refuses a parameter that shadows a `static` (`E0530`), so the port spells it
     `tagfile_` (the trailing-underscore convention of docs/ports/emacsclient.md 213 /
     docs/ports/keyboard.md 503; the token comparison strips the underscore).
  5. `sizeof *(op)` in C's `xrnew` macro becomes the port's `__sizeof_deref` generic helper;
     `sinzeof ("		 ")` is written as the literal `4` (measured) with the C spelling in a
     comment.
  6. C's `switch` is the port's `match`; character case labels are written as the literal with
     the C spelling in a comment (`97 /* 'a' */`), the `lib-src/emacsclient.rs` 630 convention.
     C's three `FALLTHROUGH;` sites (C 1255, C 1290, C 2742) become arms that carry the
     following arm's statements with a comment naming the C keyword (PORTING.md 2.2).
  7. The `#if defined WINDOWSNT || MSDOS` arm of the `sort` command builder is dead; the
     `#else` arm's `mempcpy` is the port's `rpl_mempcpy`, and `z - escaped_tagfile` is
     `offset_from`.


### Decisions (this window: C 1660-8280, the whole remainder)

- **Shape, continued.**  The definitions keep C's positions and C's names; the `static`
  helpers stay plain `unsafe fn`, and every function whose address a table takes
  (`lang_names`) or whose `_Noreturn` status matters (`fatal`, `suggest_asking_for_help`) is
  `unsafe extern "C" fn` with `-> !` where C has `_Noreturn` (`fatal`, `pfatal` via `exit`,
  `suggest_asking_for_help`).  `error`/`verror`/`fatal` are C-variadic: C's `va_list ap;
  va_start (ap, format); va_end (ap);` have no port site because the `...` parameter *is* the
  list (P-04, the make-docfile.rs precedent), and `verror` takes the `...` parameter as
  `VaList<'_>`.
- **8. `switch` → `match`, with C's fallthroughs written out** (PORTING.md 2.2).  The sites
  this window adds: `hash`'s `default:` falls through into `case 2:` (so the port's `match`
  puts the `2` arm *first* - an `_` arm first would make the literal arm an unreachable
  pattern); `add_regex`'s `case 's'` falls through into `case 'm'`; `C_entries`' `'/'`+`'*'`
  arm leaves `switch (c)` on C's `break` and reaches the token consideration below, which the
  port writes as an arm that ends instead of `continue`; `Ada_funcs`' `case 'u':` falls
  through into `case 't':` with no `FALLTHROUGH` marker in C (the port carries both in one
  arm); `test_objc_is_mercury`'s `case '/'` and `case '@'`/`case '#'` arms fall through into
  the arms below them; `Forth_words`' `(bp[0] == ':' && c_isspace (bp[1]) && bp++)` inside a
  `||` chain is a block expression that increments `bp` only when the first two hold.
- **9. `goto` → labelled blocks, flags or inline label bodies** (PORTING.md 2.2).  C's
  `cleanup:` in `process_file_name` and `out:` in `test_objc_is_mercury` are labelled blocks
  (`'cleanup: { … }`, `'out: { loop { … break 'out; } … }`); `tex_next_line:` in
  `TeX_commands` is `'tex_next_line: loop { … break 'tex_next_line; }`; C_entries'
  `still_in_token:` label is the two statements of its block (`toklen += 1; continue;`)
  written at the `goto` site; `resetfvdef:` is written at each `goto resetfvdef` site (the
  `<`, `>`, `+`/`-` arms); and `Perl_functions`' *backward* `goto subr` (into the `sub` arm)
  is a flag: the arm's body is a plain block that both the `sub` match and the `use
  constant` match run.
- **10. Macros that read a function's locals are `macro_rules!` inside that function**
  (P-57 plus `macro_rules!` hygiene: an identifier in a macro body resolves where the macro
  is *defined*).  `current_lb_is_new`, `switch_line_buffers`, `curlb`, `newlb`, `curlinepos`,
  `newlinepos`, `plainc`, `cplpl`, `cjava`, `CNL_SAVE_DEFINEDEF`, `CNL` and `instruct` are
  defined inside `C_entries` after the locals they read (and `instruct` again inside
  `consider_token`, which has its own `bracelev`); `c_ext` is a local of `C_entries` (C's
  parameter is written through `&c_ext`), so it is declared before them.  `nestlev` only
  reads `cstack`, so it stays at file scope.  C's `switch_line_buffers () (curndx = 1 -
  curndx)` is the port's `!curndx`.
- **11. `LOOP_ON_INPUT_LINES`, `LOOKING_AT`, `LOOKING_AT_NOCASE` stay at file scope but take
  the caller's body as a fourth argument** (`$body:block`): a Rust macro cannot expand into
  an unfinished `while`/`for` header, so C's `LOOP_ON_INPUT_LINES (inf, lb, cp) { … }`
  becomes `LOOP_ON_INPUT_LINES! (inf, lb, cp, { … })` and its comma-expression condition a
  block expression.  `LOOKING_AT`'s `assert ("" kw)` compile-time check is dropped and
  `sizeof (kw) - 1` becomes the `&CStr`'s byte length (`.to_bytes ().len ()`), so the macro
  takes `c"…"` at its call sites.
- **12. Names and types Rust needs.**  C's seven file types are joined by `struct
  C_stab_entry`, `cstack_record` (C's anonymous `cstack` struct), `lbs_record` (C's anonymous
  `lbs[2]` element), `mercury_pos_t`, `stkentry`, `tok` and `sym_type`.  `sym_type` is a
  `#[repr(C)]` enum with `use sym_type::*;` so C's enumerators stay unqualified the way C
  spells them; the five *anonymous* state enums (`fvdef`, `typdef`, `structdef`, `definedef`,
  `objdef`) are `c_int` constants with `static mut` variables, the file's own `at_language`
  precedent (C's enum ABI is `int`).  `fdesc` and `tok` derive `Copy` because C copies those
  records as wholes (`*fdhead = *curfdp` in `readline`, `savetoken = token` in `CNL`); the
  `#[repr(C)]` layout is unchanged.
- **13. `static mut` where a plain `static` would need `Sync`**, and `&raw const` in
  initializers: `wordlist` (`in_word_set`), `Mercury_decl_tags` and `lc_trans` (`add_regex`)
  are `static mut` because their element type carries a raw pointer.  The `lang_names`
  initializer cannot take `&static_mut` at all (`E0133`), so the 37 suffix/filename/
  interpreter pointers are `&raw const X as *const *const c_char` (the address C's array
  decay yields).
- **14. C's read-only function-scope `static` objects are `const` items**: `emptyfdesc`
  (`process_file`) and `zeropattern` (`add_regex`).  As `static mut` they would need `Copy`
  on a *mirrored* type for C's `*fdp = emptyfdesc` / `*patbuf = zeropattern`; a `const` item
  has C's bytes and no symbol.
- **15. `xmalloc (sizeof *p)` with an uninitialised `p`** is `xmalloc (std::mem::size_of::<T>
  () as ptrdiff_t)` (the ebrowse.rs 755 precedent): `let mut p: *mut T;` cannot be read in
  its own initialiser (`E0381`).  C's `UNINIT` locals are zero-initialised (PORTING.md 1.5):
  `real_name`, `tmp_name` (`process_file_name`), `dir_last_slash` (`relative_filename`),
  `TEX_opgrp`/`TEX_clgrp` (`TeX_commands`), `lno_end` (`readline`).
- **16. Use-site declarations (P-26/P-72 (3))** this window adds, each with its `/// C:`
  line: `<stdio.h>`'s `fseek`, `remove`, `fgetc`, `rename`, `vfprintf`; `<string.h>`'s
  `strnlen`, `strstr`; `<inttypes.h>`'s `strtoimax` (`lib/strtoimax.rs` owns the definition);
  `<stdlib.h>`/`<fcntl.h>`'s `mkostemp` (`lib/mkostemp.rs`).  `regex.h`'s prototypes live in
  the `regex.rs` mirror's `re_protos` module, which prescribes a wholesale import
  (`use re_protos::*;`, PORTING.md 0.1-10) - this unit is the first to need it.
- **17. Format strings**: `PRIdMAX`/`PRIdPTR` are spliced into one Rust literal as
  `%jd`/`%ld` with the C spelling in a comment (the editfns.rs/filelock.rs precedent, C
  2085/2356/2506/2517/2519/2531/2534/2556/5153/7428/7752).
- **18. Dead arms recorded, not transcribed**: `#if MSDOS || defined (DOS_NT)` at C
  1764-1783 and 7896-7911; `#if defined (DOS_NT)` at C 7920-7930, 8042-8045, 8067-8071,
  8099-8105, 8155-8157, 8166-8186.  C's *runtime* `if (MSDOS)` / `if (!MSDOS)` arms are
  transcribed as written (the `MSDOS` constant is `false`), matching the earlier window.
- **19. Numeric details**: `test_objc_is_mercury`'s `(float) rule_signs + …` is `f32`
  arithmetic (`ratio > mercury_heuristics_ratio` compares two `f32`s); `xnmalloc`/`xnrealloc`
  spell `ckd_mul` as `overflowing_mul` with the store-under-no-overflow shape (P-15, the
  ccl.rs shape) plus `SIZE_MAX` from the stdint mirror; `in_word_set`'s `len <=
  MAX_WORD_LENGTH` is `len <= MAX_WORD_LENGTH as ptrdiff_t` (C widens the int), and the
  `hash` switch's `(unsigned char) str[2]` is `*str.offset (2) as u8 as usize`; C's
  `char`→`int` promotions are written `as c_int` at every `c_isspace`/`c_is*`/`endtoken`
  call, and `debug`/`globals`/`members`/`declarations`/`class_qualify`/`packages_only`/
  `fallback_lang` (C `int`) are compared `!= 0`.

### Gates (this lane's run, 2026-10-04, C 1660-8280)

- `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-et2 cargo run -q -p xtask -- build-tu
  lib-src/etags.c` -> `built lib-src/etags.rs -> ../build/femacs/etags.o` (green; the last
  run is the file's current state)
- `rustc --edition 2024 --crate-type=lib --emit=metadata --crate-name et2_check -C
  panic=abort -C codegen-units=1 -C opt-level=0 build/femacs/units/etags/_root.rs -o
  /tmp/et2.rmeta` -> exit 0 (38 warnings, listed in STATUS)
- `python3 tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s), 0
  failure(s)` (no etags row)
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`
- `python3 tools/check-verbatim.py` -> `6 trees, 4607 files, 0 failures`
- `python3 tools/check-globals.py` -> `283 units, 0 DEFVAR_* variables without a definition
  in their own unit`
- `python3 tools/extract-abi.py --check emacs/lib-src/etags.c femacs/lib-src/etags.rs` ->
  `FAIL`: 0 `missing function`, 0 `missing macro`, 5 `missing file-scope object` (the
  `Forth_help` artifacts of the Owed-rows block), 39 `does not mention` rows (the 4
  string-literal/keyword rows already owed plus the 35 this window's translation implies),
  1 `family found` row (the same `print_help` literal), `line count drifted 9.5%: C 8280 vs
  Rust 9066`.

# Owed note rows (the integrator's `docs/ports/*.tsv`).  `python3 tools/extract-abi.py --check
# emacs/lib-src/etags.c femacs/lib-src/etags.rs` now reports **39** `does not mention` rows
# (4 of them were already owed before this window), 5 `missing file-scope object` rows that
# are not objects, and one `family found` row.  Each `does not mention` line is a row for
# `docs/ports/macro-body-notes.tsv`, verbatim:
#
#   CNL	0	etags: C's `do { … } while (0)` is the port's block expression inside `C_entries`, so the condition's `0` cannot reappear
#   CNL_SAVE_DEFINEDEF	0	etags: C's `do { … } while (0)` is the port's block expression inside `C_entries`, so the condition's `0` cannot reappear
#   LOOKING_AT	1, assert	etags: C's `assert ("" kw)` compile-time check is dropped (it has no runtime effect) and `sizeof (kw) - 1` is the `&CStr`'s byte length at the call site, so neither token can reappear
#   LOOKING_AT_NOCASE	1, assert	etags: C's `assert ("" kw)` compile-time check is dropped (it has no runtime effect) and `sizeof (kw) - 1` is the `&CStr`'s byte length at the call site, so neither token can reappear
#   switch_line_buffers	1	etags: C's `curndx = 1 - curndx` is the port's `!curndx` inside `C_entries`, so the `1` cannot reappear
#   print_language_names	ada, along, and, any, are, bang, based, bzip2, class, compressed, currently, default, detailed, dot, etags, example, file, files, first, followed, fortran, found, given, gzip, help, interpreter, keyword, languages, line, matching, means, names, next, nthese, only, parsing, processing, read, sequence, sharp, specified, such, suffix, supported, switches, tags, template, the, tried, use, using, when, where, with, zstd	etags: C 896-926's message is one C string literal written with backslash-newline continuations, which `cinventory`'s literal blanker does not see, so every word of the message turns into a body token; the port carries the message as one Rust literal whose bytes the `puts` call keeps
#   print_version	copyright, package_name, package_version	etags: C 931-933's `fputs` folds `<config.h>`'s `PACKAGE_NAME`, `PACKAGE_VERSION` and `COPYRIGHT` into one C string literal; the port carries the same bytes as one Rust literal (the frozen values are in docs/ports/etags.md), so the three macro names cannot reappear
#   print_help	abbreviations, about, absolute, accepted, ada, after, allow, also, anchored, and, any, append, are, arguments, assume, back, backward, bar, baz, behavior, belonging, beyond, both, brace, bug, bugs, but, can, case, category, causes, character, checking, class, clauses, closing, column, command, commands, compatibility, considered, constants, consult, consumption, containing, cosmetic, could, create, created, current, currently, cxref, declarations, define, defines, definition, deleting, derived, detailed, determined, differences, directive, directives, directory, don, dot, duplicate, duplicates, each, emacs, empty, end, entire, entries, example, existing, expression, fall, fallback, faster, file, files, final, first, followed, following, foo, force, form, fortran, forward, from, function, functions, generate, generation, given, gives, global, globals, gnu, human, ignore, implement, implemented, implies, include, including, indentation, index, indicating, input, instead, intended, item, items, java, languages, leaving, letter, like, line, make, makes, match, matches, matching, may, means, member, members, mercury, message, messages, methods, modifiers, mods, more, multi, name, named, names, new, newline, next, not, note, number, numbers, objc, objective, often, one, ones, only, option, optional, options, org, other, output, packages, page, parse, pattern, per, perl, place, preceded, predicates, preprocessor, present, print, prints, proc, produce, produces, program, qualified, qualify, rather, read, rebuild, record, regex, regexfile, regexps, regular, relative, report, rewriting, search, searching, see, should, similar, simply, smaller, some, sorted, specified, standard, starting, stdin, stored, structure, structures, style, suppress, tag, tagname, tagregexp, tags, takes, tand, tcl, than, that, the, their, then, these, they, this, tin, too, traditional, try, typedefs, unambiguous, unless, update, usage, use, used, uses, using, variables, version, vgrind, warn, warning, when, where, which, with, write, written, you	etags: C 943-1139's message is one C string literal written with backslash-newline continuations, which `cinventory`'s literal blanker does not see, so every word of the message turns into a body token; the port carries the message as one Rust literal whose bytes the `puts` call keeps
#   main	fallthrough	etags: C 1255/1290's `FALLTHROUGH;` has no `match` site - the arms carry the next arm's statements with a comment naming the C keyword (PORTING.md 2.2), so the token cannot reappear
#   process_file_name	fopen_binary, goto, uninit	etags: C's `"r" FOPEN_BINARY` is one Rust literal (`FOPEN_BINARY` is `&CStr` = `""` on this target), so the macro name cannot reappear etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site etags: `UNINIT` (`<conf_post.h>`) is the zero marker the port spells with an explicit `= 0`/`std::ptr::null_mut ()` initialiser (PORTING.md 1.5)
#   make_tag	pridmax	etags: C's `"%"PRIdMAX`/`PRIdPTR` is spliced into one Rust literal as `%jd`/`%ld` (this ledger's Decisions), so the macro name cannot reappear
#   add_node	pridmax	etags: C's `"%"PRIdMAX`/`PRIdPTR` is spliced into one Rust literal as `%jd`/`%ld` (this ledger's Decisions), so the macro name cannot reappear
#   put_entry	pridmax, pridptr	etags: C's `"%"PRIdMAX`/`PRIdPTR` is spliced into one Rust literal as `%jd`/`%ld` (this ledger's Decisions), so the macro name cannot reappear
#   hash	break, case, default, fallthrough, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   write_classname	continue	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   consider_token	case, default, fallthrough, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   C_entries	case, default, fallthrough, goto, resetfvdef, still_in_token, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site etags: the C label has no Rust counterpart: the port writes the label's block at the `goto` site, or selects the body with a flag (PORTING.md 2.2)
#   F_getit	continue	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   Fortran_functions	case, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   Ada_getit	break, case, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   Ada_funcs	case, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   Perl_functions	goto, subr	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site etags: the C label has no Rust counterpart: the port writes the label's block at the `goto` site, or selects the body with a flag (PORTING.md 2.2)
#   Ruby_functions	pridmax	etags: C's `"%"PRIdMAX`/`PRIdPTR` is spliced into one Rust literal as `%jd`/`%ld` (this ledger's Decisions), so the macro name cannot reappear
#   Cobol_paragraphs	0	etags: C's `p[0]` subscript is the port's `*p` / `*p.offset (…)` spelling, so the subscript's literal cannot reappear
#   Pascal_functions	break, case, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   Lua_functions	0	etags: C's `p[0]` subscript is the port's `*p` / `*p.offset (…)` spelling, so the subscript's literal cannot reappear
#   PS_functions	continue	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   TeX_commands	case, default, goto, switch, uninit	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site etags: `UNINIT` (`<conf_post.h>`) is the zero marker the port spells with an explicit `= 0`/`std::ptr::null_mut ()` initialiser (PORTING.md 1.5)
#   test_objc_is_mercury	case, default, fallthrough, float, goto, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site etags: C's `(float)` cast is the port's `f32`
#   scan_separators	case, default, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   analyze_regex	break, case, default, fopen_binary, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site etags: C's `"r" FOPEN_BINARY` is one Rust literal (`FOPEN_BINARY` is `&CStr` = `""` on this target), so the macro name cannot reappear
#   add_regex	break, case, default, fallthrough, switch, uchar_max	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site etags: C's `UCHAR_MAX + 1` is the port's measured array length 256
#   regex_tag_multiline	break, case, default, pridmax, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site etags: C's `"%"PRIdMAX`/`PRIdPTR` is spliced into one Rust literal as `%jd`/`%ld` (this ledger's Decisions), so the macro name cannot reappear
#   get_tag	continue	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   get_lispy_tag	continue	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site
#   readline	case, default, pridmax, switch	etags: C's `switch`/`case`/`default`/`break`/`continue`/`goto`/`FALLTHROUGH;` are the port's `match`/`loop`/labelled blocks (PORTING.md 2.2); arms that share a body carry the next arm's statements with a comment naming the C keyword, so the keyword token has no Rust site etags: C's `"%"PRIdMAX`/`PRIdPTR` is spliced into one Rust literal as `%jd`/`%ld` (this ledger's Decisions), so the macro name cannot reappear
#   relative_filename	uninit	etags: `UNINIT` (`<conf_post.h>`) is the zero marker the port spells with an explicit `= 0`/`std::ptr::null_mut ()` initialiser (PORTING.md 1.5)
#   filename_is_absolute	0	etags: C's `p[0]` subscript is the port's `*p` / `*p.offset (…)` spelling, so the subscript's literal cannot reappear

# The five `missing file-scope object` rows are the same *literal* artifact as
# `print_help`'s: C 652-653 is `Forth_help`'s one string literal, written with a
# backslash-newline continuation, so `cinventory`'s literal blanker turns its
# comma-separated words into declarators.  The port carries that message as one Rust literal,
# so the names cannot reappear:
#
#   code	code	etags: a word of C 652-653's `Forth_help` string literal, which the port carries as one literal
#   create	create	etags: a word of C 652-653's `Forth_help` string literal, which the port carries as one literal
#   defer	defer	etags: a word of C 652-653's `Forth_help` string literal, which the port carries as one literal
#   value	value	etags: a word of C 652-653's `Forth_help` string literal, which the port carries as one literal
#   variable	variable	etags: a word of C 652-653's `Forth_help` string literal, which the port carries as one literal
#
# The `family found` row is the same class again: `found` is a token-family alias
# (`docs/ports/token-aliases.tsv` 85), and the three C occurrences of the word are the three
# `found` values inside `print_help`'s one literal.  It needs a family-level row in the same
# notes file:
#
#   found	found, blv_found	etags: the word `found` occurs 3 times in C 943-1139's `print_help` string literal (one per backslash-newline continuation); the port carries the message as one Rust literal, so the family count is 0
#
# With those rows in place, the only remaining `extract-abi.py --check` lines are the
# `line count drifted 9.5%` one and the `missing file-scope object` rows' *wording* (the
# generator names them objects; they are literal words).

## Integration (the integrator's pass)

- **The shared rows landed** (the four message-literal functions as `*` rows, the thirty-five token
  rows, the `found` family note and the line-drift row 0.11 against the measured +9.5%), so the
  check runs against the shared files: `python3 tools/extract-abi.py --check emacs/lib-src/etags.c
  femacs/lib-src/etags.rs` prints `OK`, and the 5 boxes above are checked against it.
- **The five "file-scope objects" were not objects, and were fixed at the root**: they are words of
  the `Forth_help` literal, which C writes with a backslash-continued line; `cinventory.globals_`
  read each line through the per-line `strip_strings`, which cannot see a literal that *starts* on
  an earlier line, so the words looked like a declaration.  `globals_` now reads `Lines.scan_rows`,
  the cross-line-aware view `Lines` already builds (status 3.130); C declares none of those names
  and the scan no longer invents them.
- **Behavioural evidence: none.**  The lane translated the whole file and compiled it green, then
  ran the text/structure gates; the control flow of `C_entries`/`consider_token`/TeX/HTML/Mercury
  was checked by reading, and that reading found one real defect (`hash`'s `default:`/`case 2:`
  fallthrough order, which made the literal arm unreachable).  Everything else stays text- and
  object-level; the unit has no differential run like ebrowse's.
