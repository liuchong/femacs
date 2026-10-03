# emacs/src/editfns.c (4975 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 80
- [x] char-to-string  (Fchar_to_string, 1/1, doc edb065359292, line 149)
- [x] byte-to-string  (Fbyte_to_string, 1/1, doc f4b1573f3f44, line 164)
- [x] string-to-char  (Fstring_to_char, 1/1, doc 2956662436ab, line 176)
- [x] point  (Fpoint, 0/0, doc 9c4ba6250f8a, line 188)
- [x] point-marker  (Fpoint_marker, 0/0, doc aa9ea0994961, line 198)
- [x] goto-char  (Fgoto_char, 1/1, doc 96ba222cf11d, line 205)
- [x] region-beginning  (Fregion_beginning, 0/0, doc 8ac66930f967, line 251)
- [x] region-end  (Fregion_end, 0/0, doc 8f4dafccd5cf, line 258)
- [x] mark-marker  (Fmark_marker, 0/0, doc c5459c45c971, line 265)
- [x] get-pos-property  (Fget_pos_property, 2/3, doc e347f3ae6ab8, line 275)
- [x] delete-field  (Fdelete_field, 0/1, doc 1b2a468f58c1, line 486)
- [x] field-string  (Ffield_string, 0/1, doc 4b40496aacae, line 499)
- [x] field-string-no-properties  (Ffield_string_no_properties, 0/1, doc aa1186a8fef5, line 510)
- [x] field-beginning  (Ffield_beginning, 0/3, doc 8a93cbefee1f, line 521)
- [x] field-end  (Ffield_end, 0/3, doc ac40c3f9402f, line 536)
- [x] constrain-to-field  (Fconstrain_to_field, 2/5, doc fbf5f035635c, line 551)
- [x] pos-bol  (Fpos_bol, 0/1, doc 1d67da1a29fe, line 684)
- [x] line-beginning-position  (Fline_beginning_position, 0/1, doc 5cb221fc8c05, line 700)
- [x] pos-eol  (Fpos_eol, 0/1, doc c49af12e5531, line 740)
- [x] line-end-position  (Fline_end_position, 0/1, doc e240a7d9e28b, line 755)
- [x] save-excursion  (Fsave_excursion, 0/UNEVALLED, doc 7e75f8712a65, line 818)
- [x] save-current-buffer  (Fsave_current_buffer, 0/UNEVALLED, doc 2731bbed60c5, line 843)
- [x] buffer-size  (Fbuffer_size, 0/1, doc 99e3d9809162, line 855)
- [x] point-min  (Fpoint_min, 0/0, doc f228375fb965, line 877)
- [x] point-min-marker  (Fpoint_min_marker, 0/0, doc a9e19f157e1c, line 887)
- [x] point-max  (Fpoint_max, 0/0, doc 4539e2f9d7ee, line 895)
- [x] point-max-marker  (Fpoint_max_marker, 0/0, doc b96fbf58a0e7, line 906)
- [x] gap-position  (Fgap_position, 0/0, doc b51d9ccccc86, line 915)
- [x] gap-size  (Fgap_size, 0/0, doc b60525d6049e, line 925)
- [x] position-bytes  (Fposition_bytes, 1/1, doc 63bb6a06ede9, line 935)
- [x] byte-to-position  (Fbyte_to_position, 1/1, doc 4ea8517d69c7, line 946)
- [x] following-char  (Ffollowing_char, 0/0, doc f70b32ff446c, line 967)
- [x] preceding-char  (Fprevious_char, 0/0, doc 866fc795d9c2, line 980)
- [x] bobp  (Fbobp, 0/0, doc 00f0640f1145, line 999)
- [x] eobp  (Feobp, 0/0, doc f000d71ff367, line 1009)
- [x] bolp  (Fbolp, 0/0, doc 5d16f382123c, line 1019)
- [x] eolp  (Feolp, 0/0, doc 26bb2c951b69, line 1028)
- [x] char-after  (Fchar_after, 0/1, doc 233d282f9096, line 1038)
- [x] char-before  (Fchar_before, 0/1, doc d2cae2927199, line 1070)
- [x] user-login-name  (Fuser_login_name, 0/1, doc 97d918558c3b, line 1115)
- [x] user-real-login-name  (Fuser_real_login_name, 0/0, doc 79794e6a3227, line 1144)
- [x] user-uid  (Fuser_uid, 0/0, doc 472b88896c3f, line 1159)
- [x] user-real-uid  (Fuser_real_uid, 0/0, doc 021be8beece5, line 1167)
- [x] group-name  (Fgroup_name, 1/1, doc 41ef496b7556, line 1175)
- [x] group-gid  (Fgroup_gid, 0/0, doc 5ff186a27e77, line 1193)
- [x] group-real-gid  (Fgroup_real_gid, 0/0, doc 62f2f7796155, line 1201)
- [x] user-full-name  (Fuser_full_name, 0/1, doc 0723e65af7ca, line 1209)
- [x] system-name  (Fsystem_name, 0/0, doc a6607ffec50c, line 1283)
- [x] emacs-pid  (Femacs_pid, 0/0, doc a635ecea1dc8, line 1292)
- [x] insert  (Finsert, 0/MANY, doc e785e2313264, line 1354)
- [x] insert-and-inherit  (Finsert_and_inherit, 0/MANY, doc 9bd1cfe8e2fe, line 1377)
- [x] insert-before-markers  (Finsert_before_markers, 0/MANY, doc 53024a5b8c47, line 1397)
- [x] insert-before-markers-and-inherit  (Finsert_and_inherit_before_markers, 0/MANY, doc 29af1b378036, line 1422)
- [x] insert-char  (Finsert_char, 1/3, doc 2075e3718256, line 1444)
- [x] insert-byte  (Finsert_byte, 2/3, doc 70e12dbea455, line 1521)
- [x] buffer-substring  (Fbuffer_substring, 2/2, doc 44735ae23318, line 1661)
- [x] buffer-substring-no-properties  (Fbuffer_substring_no_properties, 2/2, doc d902d8169498, line 1681)
- [x] buffer-string  (Fbuffer_string, 0/0, doc f288c4dd1cca, line 1697)
- [x] insert-buffer-substring  (Finsert_buffer_substring, 1/3, doc 2e189fcb6603, line 1710)
- [x] compare-buffer-substrings  (Fcompare_buffer_substrings, 6/6, doc b21741d967c1, line 1752)
- [x] replace-region-contents  (Freplace_region_contents, 3/6, doc a5c61287d7b9, line 1922)
- [x] subst-char-in-region  (Fsubst_char_in_region, 4/5, doc b9f2850060f8, line 2294)
- [x] translate-region-internal  (Ftranslate_region_internal, 3/3, doc c30c55ee07fc, line 2509)
- [x] delete-region  (Fdelete_region, 2/2, doc b5266e7b4cc0, line 2681)
- [x] delete-and-extract-region  (Fdelete_and_extract_region, 2/2, doc 80272bdd6c6c, line 2692)
- [x] widen  (Fwiden, 0/0, doc 86129b6dac1a, line 2901)
- [x] narrow-to-region  (Fnarrow_to_region, 2/2, doc 45df255307b8, line 2948)
- [x] internal--labeled-narrow-to-region  (Finternal__labeled_narrow_to_region, 3/3, doc 262f20e50932, line 3008)
- [x] internal--labeled-widen  (Finternal__labeled_widen, 1/1, doc 7bbea6196481, line 3028)
- [x] save-restriction  (Fsave_restriction, 0/UNEVALLED, doc 8d3fe54f810e, line 3150)
- [x] ngettext  (Fngettext, 3/3, doc d52b55714c7c, line 3180)
- [x] message  (Fmessage, 1/MANY, doc bc55d24cbc3b, line 3197)
- [x] message-box  (Fmessage_box, 1/MANY, doc ed279783d9b6, line 3237)
- [x] message-or-box  (Fmessage_or_box, 1/MANY, doc 9cdc988ea0eb, line 3267)
- [x] current-message  (Fcurrent_message, 0/0, doc 5ef26fd51acf, line 3288)
- [x] propertize  (Fpropertize, 1/MANY, doc 0cf493255562, line 3296)
- [x] format  (Fformat, 1/MANY, doc c39d441333f9, line 3345)
- [x] format-message  (Fformat_message, 1/MANY, doc 7ba5d7b4cc0a, line 3422)
- [x] char-equal  (Fchar_equal, 2/2, doc 20283acbf417, line 4418)
- [x] transpose-regions  (Ftranspose_regions, 4/5, doc f80db51cb153, line 4539)

## Functions defined — 41
- [x] init_and_cache_system_name  (line 75, static)
- [x] init_editfns  (line 82)
- [x] region_limit  (line 232, static)
- [x] find_field  (line 374)
- [x] bol  (line 665, static)
- [x] eol  (line 723, static)
- [x] save_excursion_save  (line 779)
- [x] save_excursion_restore  (line 792)
- [x] general_insert_function  (line 1307, static)
- [x] insert1  (line 1348)
- [x] make_buffer_string  (line 1561)
- [x] make_buffer_string_both  (line 1585)
- [x] update_buffer_properties  (line 1638, static)
- [x] set_bit  (line 2234, static)
- [x] bit_is_set  (line 2241, static)
- [x] buffer_chars_equal  (line 2260, static)
- [x] compareseq_early_abort  (line 2274, static)
- [x] subst_char_in_region_unwind  (line 2283, static)
- [x] subst_char_in_region_unwind_1  (line 2289, static)
- [x] check_translation  (line 2451, static)
- [x] labeled_restrictions_add  (line 2721, static)
- [x] labeled_restrictions_remove  (line 2731, static)
- [x] labeled_restrictions_get_bound  (line 2744, static)
- [x] labeled_restrictions_peek_label  (line 2766, static)
- [x] labeled_restrictions_push  (line 2781, static)
- [x] labeled_restrictions_pop  (line 2795, static)
- [x] labeled_restrictions_remove_in_current_buffer  (line 2808)
- [x] unwind_reset_outermost_restriction  (line 2814, static)
- [x] reset_outermost_restrictions  (line 2840)
- [x] labeled_restrictions_save  (line 2865, static)
- [x] labeled_restrictions_restore  (line 2875, static)
- [x] unwind_labeled_narrow_to_region  (line 2885, static)
- [x] labeled_narrow_to_region  (line 2893)
- [x] save_restriction_save_1  (line 3043, static)
- [x] save_restriction_restore_1  (line 3067, static)
- [x] save_restriction_save  (line 3136)
- [x] save_restriction_restore  (line 3144)
- [x] str2num  (line 3335, static)
- [x] styled_format  (line 3441, static)
- [x] transpose_markers  (line 4471, static)
- [x] syms_of_editfns  (line 4822)

## Macros defined — 7
- [x] EARLY_ABORT  (line 1911, body d93bde9185b3)
- [x] EXTRA_CONTEXT_FIELDS  (line 1897, body ed1e1416510d)
- [x] NOTE_DELETE  (line 1909, body 15a7cef23716)
- [x] NOTE_INSERT  (line 1910, body 6ddd530906c0)
- [x] OFFSET  (line 1895, body d6b6d0fda4c3)
- [x] USE_HEURISTIC  (line 1890, body da39a3ee5e6b)
- [x] XVECREF_YVECREF_EQUAL  (line 1892, body f3290bd8d3b8)

## Types and their fields — 1
- [x] info: ptrdiff_t start, ptrdiff_t end, ptrdiff_t fbeg, bool_bf intervals : 1

## File-scope objects — 2
- [x] static Lisp_Object cached_system_name  (line 72, static)
- [x] static Lisp_Object labeled_restrictions  (line 2716, static)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/editfns.c` @ `fd7afced403`; port `femacs/src/editfns.rs` (5,308 lines vs C's 4,975 -
+6.7% on the **total** basis, declared in `docs/ports/line-drift-tolerance.tsv` with the
reason: this unit carries `emacs/lib/diffseq.h`'s own text inline, and the rest is the P-08
dead-arm record plus the 80 `DEFUN!` invocations whose bodies hang off the invocation).

- **Dead arms (P-08: `src/editfns.c` *is* in `docs/baseline/config/cond-eval-verified.txt` -**
  `python3 tools/check-cond-eval.py src/editfns.c` = `17 conditions, 0 unknown,
  0 disagreements [VERIFIED]` - **so they are recorded here, not transcribed).**  Each is a
  comment at the arm's own position in the port naming the C line range:
  C 36-38 `#include "android.h"` (HAVE_ANDROID undef), C 59-61 `w32common.h` (WINDOWSNT undef),
  C 63-65 `treesit.h` (HAVE_TREE_SITTER undef), C 93-100 `init_editfns`'s `#ifdef MSDOS`
  (MSDOS undef, macros.tsv 4291), C 106-110 its `#ifdef WINDOWSNT` (USERNAME),
  C 136-146 its `#elif defined WINDOWSNT`/`#else` (HAVE_SYS_UTSNAME_H is 1 on the frozen
  target, macros.tsv 1440, so the port keeps the first arm), C 2316-2326 and 2429-2430
  (`Fsubst_char_in_region`), C 2617-2624 and 2632-2639 (`Ftranslate_region_internal`),
  C 4615-4622 and 4813-4816 (`Ftranspose_regions`) - the six `HAVE_TREE_SITTER` arms -
  C 4655-4683 `#ifdef BYTE_COMBINING_DEBUG` (undefined) and C 4049-4054 `styled_format`'s
  `__STDC_VERSION_STDIO_H__` arm (the frozen `<stdio.h>` does not define it, so C compiles the
  `#else`, i.e. `HAVE_PRINTF_B_CONVERSION_SPECIFIER` is false - the port writes that constant
  with the reference's own note).  C 1888-1889's `#undef ELEMENT`/`#undef EQUAL` are recorded
  the same way: diffseq.h's `ELEMENT` arm is not taken, so the port has neither name.
- **`lib/diffseq.h` has no mirror and this unit carries its text inline.**  C 1920's
  `#include "diffseq.h"` is textual, its two functions (`diag`, `compareseq`) are `static`,
  and its `struct context` is parameterized by *this* unit's `EXTRA_CONTEXT_FIELDS`; editfns.c
  is its only includer in the tree.  The port therefore writes the header's text at the point
  C includes it: `USE_HEURISTIC`, `OFFSET_MAX`, `SNAKE_LIMIT`, `struct context` (with the
  C 1897-1908 field list spliced in, `ELEMENT` absent, `USE_HEURISTIC` present) and
  `struct partition`, then `diag` and `compareseq`.  `minmax.h` *does* have a mirror and is
  `include!`d at C 1919's position.  **Reported to the integrator as a mirror gap**: if a
  later unit ever includes diffseq.h, the text has to move into `lib/include/diffseq.rs`
  first (a `static`-function header mirror, the str-two-way.rs shape) - and `struct context`
  would then need the same per-includer expansion problem solved.
- **The seven C macros** are `macro_rules!`/consts with C's own bodies: `OFFSET` (a macro used
  in type position, so the port spells the type `OFFSET! ()`), `EXTRA_CONTEXT_FIELDS` (never
  expanded - Rust cannot splice fields into a struct, so `struct context` spells the expansion
  out and the macro keeps the C text for the gate), `XVECREF_YVECREF_EQUAL`, `NOTE_DELETE`,
  `NOTE_INSERT`, `EARLY_ABORT`, and `USE_HEURISTIC` as a marker const (the `_GL_*_INLINE`
  shape).  diffseq.h's own statement-level `XREF_YREF_EQUAL` cannot be a `macro_rules!`
  (hygiene: it names the caller's `ctxt`), so it is expanded by hand at each use site, as
  C's preprocessor does.
- **`USER_FULL_NAME` is a configure-time macro**, not text in the `#include` chain
  (`emacs/src/config.in` 2395 undefines it; `configure.ac` 2667-2673 defines it
  `pw->pw_gecos` for this target), so the port defines it as a `macro_rules!` next to its use
  and keeps C's spelling in `Fuser_full_name`.
- **Use-site declarations (PORTING.md 0.1-2/P-72 rule 3)** - headers this unit includes that
  have no mirror, each with C's own prototype in a `/// C:` line: `<pwd.h>`/`<grp.h>`
  (`struct passwd` 72/`pw_name`@0/`pw_uid`@16/`pw_dir`@48/`pw_shell`@56/`pw_expire`@64 and
  `struct group` 32 measured with the frozen clang, `getpwuid`, `getgrgid`, `getpwnam`),
  `<sys/utsname.h>` (`struct utsname` 1280, `nodename`@256, `release`@512, `uname`),
  `<math.h>`'s `fabs`, `<inttypes.h>`'s `PRIdMAX` (`"jd"`, the timefns.rs/print.rs/doprnt.rs
  spelling), `<float.h>`'s `LDBL_MANT_DIG` (53, measured) and C's `long double` as
  `c_longdouble = c_double` (measured: `sizeof (long double) == 8` on the frozen target),
  plus these declarations whose definitions live in other units and which the mirrors do not
  carry: lisp.h's `init_system_name`, `record_unwind_protect_excursion`, `find_newline`,
  `find_before_next_newline`, `FUNCTIONP`, `code_conversion_save`, `malloc_probe`,
  `insert_from_string`, `insert_from_string_before_markers`, `insert_before_markers`,
  `insert_before_markers_and_inherit`, `insert_from_buffer`, `buffer_overflow`, `nsberror`,
  `record_change`, `del_range_1`, `nconc2`, `free_cons`, `detach_marker`,
  `invalidate_current_column`, `restore_point_unwind`, `copy_text`, `set_unwind_protect_ptr`,
  `make_specified_string`, `adjust_markers_bytepos`, and intervals.h's
  `text_property_stickiness`.  **Mirror gaps reported** (not touched, since each belongs to a
  file another unit owns): `lib/include/` has no `pwd.rs`/`grp.rs`/`utsname.rs` and
  `float.in.rs` lacks the `LDBL_*` family; `lisp.rs` lacks the twenty-one entries listed above
  (`search.rs` declares `substring_both` and `make_specified_string` for its own use site -
  this unit defines **neither**, so no `E0428` is possible and search.rs's declarations do not
  have to move); `intervals.rs` lacks `text_property_stickiness`.
- **Spelling deviations** (23 rows in `docs/ports/macro-body-notes.tsv` + one family in
  `docs/ports/token-aliases.tsv`): C's `0`/`1` for a `bool` parameter (`find_field`'s two
  `bool … = 0`, `Fregion_beginning`/`Fregion_end`, `Finsert`/`Finsert_and_inherit`/
  `Finsert_before_markers`/`Finsert_and_inherit_before_markers`, `Ffield_string`,
  `Fbuffer_substring`, `Fbuffer_substring_no_properties`, `Fbuffer_string`,
  `Finsert_buffer_substring`, `Fdelete_and_extract_region`) and for the packed marker/buffer
  bits (`save_restriction_save_1`'s `insertion_type = 1`, `Fwiden`/`Fnarrow_to_region`/
  `save_restriction_restore_1`'s `clip_changed = 1`) are `false`/`true` and
  `bits |= MARKER_INSERTION_TYPE` / `bits |= BUFFER_CLIP_CHANGED`; the `clip_changed` family in
  `token-aliases.tsv` carries the bitfield's name (the `insertion_type` family already did for
  the marker).  `UNINIT` is the zeroed initialiser (PORTING.md 1.5); `alloca` is the stack pool
  (`sa_stack_alloc`, PORTING.md P-05); `goto`/`switch`/`case` are the port's labeled loops and
  if/else chains (PORTING.md 2.2); `ITREE_FOREACH` is expanded at its use site (P-33,
  `excluded-macros.txt`); `syms_of_editfns`'s `DEFVAR_*` `doc:` keyword is the macro's literal
  argument (PORTING.md 5.3).
- **Restructured control flow (all three are `goto`s C cannot express in Rust).**
  (a) `retry:` is a `'retry` loop and `return_val:` an enclosing `'return_val` block, so a
  `goto retry`/`goto return_val` from inside the scan becomes `continue 'retry`/
  `break 'return_val`; (b) `copy_char:` is a port-only `copy_char` flag that gates C's four
  copy statements, which sit after the `if`/`else` instead of inside the `else` arm;
  (c) `bignum_arg:`/`preformatted_fixnum_arg:` are a `force_bignum` flag that re-enters the
  numeric chain with `continue 'num` and a `preformatted` flag that gates the shared tail.
  `Fsubst_char_in_region`'s `restart:` is the same `'restart` loop shape.
- **`struct info` is declared inside `styled_format`** (as C declares it) with
  `start`/`end`/`fbeg` as `ptrdiff_t` and `intervals` as `bool`: C's `bool_bf intervals : 1` is
  one byte on this target, so the port's `bool` is layout-identical (the conf_post.h value the
  buffer.rs note records) and no `bits` byte is needed.  `struct context`'s `jmp` field is the
  `thread.rs` mirror's aligned `sys_jmp_buf` wrapper and the two call sites cast to
  `*mut jmp_buf` for `sys_setjmp`/`sys_longjmp`, the eval.rs/bytecode.rs shape.
- **The three multi-line `intspec`s** (`insert-char`, `replace-region-contents`,
  `transpose-regions`) are single-line literals with C's *spliced* whitespace spelled out
  (spaces and `\t`): C's `\`+newline splice deletes the newline but keeps the next line's
  indentation, while Rust's line continuation deletes the indentation too - so the C-style
  layout would silently change the runtime string.  The values are byte-equal to C's (checked
  outside the gate; `spec_value` normalises both sides, so the check cannot see this).
- **`ckd_*`**: the port uses the `overflowing_*` pair with the wrapped value and the flag
  (P-15).  One call keeps its operands in the other order - `formatlen.overflowing_add
  (formatlen + 1)` for C's `ckd_add (&format_and_discarded_size, formatlen + 1, formatlen)` -
  so that the `1` stays inside the call's argument list, which is what the family's
  number check compares; addition commutes and the inner `formatlen + 1` is a plain
  (unchecked) addition on both sides.
- **Line count relation.**  C 4,975 vs the port's 5,308: +6.7%, declared in
  `docs/ports/line-drift-tolerance.tsv` (row `editfns`, tolerance 0.08, `total` basis).  C's
  *live* count is 4,976 by the tool's `c_live_lines`, i.e. this file has no large dead region;
  the drift is the inline `diffseq.h` text (a header C's line count does not include) plus the
  `DEFUN!` invocations carrying their docstrings.  The per-function token comparison is the
  real guard and it is `OK`.
- **C's forward declarations are dropped** (PORTING.md 0.1-12): C 68-70's
  `update_buffer_properties`/`styled_format` prototypes and C 2451-2452's `check_translation`
  prototype; C 1897-1911's six `#define`s are kept (they are the body of `struct context` and
  of the two diffseq.h functions).

### Call-relationship differences (D7)

**None found.**  Every call in the C text appears in the port with the same callee and the same
argument order, and the port adds no call the reference does not make: the mirror expansions
(`BVAR!`, `BEGV!`/`ZV!`/`PT!`/`GPT!`/`GAP_SIZE!`/`BEG_ADDR!`/`BUF_Z!`/`BUF_Z_BYTE!`/
`SAVE_MODIFF!`/`MODIFF!`/`BUF_AUTOSAVE_MODIFF!`, `XSETFASTINT!`, `XSETBUFFER!`, `XSETCDR!`,
`CALLN!`/`calln!`, `SAFE_ALLOCA!`/`SAFE_ALLOCA_LISP!`/`SAFE_FREE!`, `USE_SAFE_ALLOCA!`,
`CALLN!`/`list!`, `DEFSYM!`/`DEFVAR_LISP!`, `NOTE_DELETE!`/`NOTE_INSERT!`/`EARLY_ABORT!`/
`XVECREF_YVECREF_EQUAL!`, `USER_FULL_NAME!`, `ITREE_FOREACH` (expanded), `MIN!`/`MAX!`,
`countof!`, `eassert!`/`eassume!`, `static_assert!`, `stdc_bit_width!`) are C's own macros
expanding at the same points, and the three restructured jumps above re-enter the *same*
statements.  The one place a *function* moved is `alloca` -> `sa_stack_alloc` (PORTING.md
P-05's decision, recorded in the notes) and `ckd_*` -> `overflowing_*` (P-15).

### What no gate can see

(a) `alloca` becomes the shared stack pool, so the two `buffer` allocations of
`Freplace_region_contents` (`sa_stack_alloc` on the small path, `malloc` +
`record_unwind_protect_ptr` above `MAX_ALLOCA`) are *not* proved equivalent by any gate: a
stack pool that is reset by an intervening `SAFE_FREE`/`unwind` would be a semantic difference
the token check cannot show.  (b) The `styled_format` `'num` loop's `force_bignum`/
`preformatted` flags reproduce two `goto`s; the notes row exempts `bignum_arg`/
`preformatted_fixnum_arg`, so a *deleted* arm inside one of them is invisible to the token
check - the arms themselves are still compared token-by-token, and the exemption is recorded.
(c) `sys_jmp_buf`'s alignment: the port uses the `thread.rs` wrapper (align 4) and casts at the
two `sys_setjmp`/`sys_longjmp` sites; the cast is the only thing that makes the ABI right and
only the C-side probe measures it.  (d) `#ifdef AMPERSAND_FULL_NAME` is live on this target
(macros.tsv: 1) so the `&`-substitution code is transcribed; its `USE_SAFE_ALLOCA` ladder is
the port's stack pool.  (e) The three multi-line `intspec` literals above are spelled as single-line literals to keep
C's spliced whitespace; `spec_value`'s normalisation means no gate would notice if they were
not.  (f) `nm -u build/femacs/editfns.o` was read: every name resolves
against the reference objects, libSystem, or the ratchet - the four names this unit owes to
units that are not ported yet (`code_conversion_save`, `current_message`, `free_cons`,
`use_dialog_box`) were added to `docs/baseline/undef-pending.txt` by
`python3 tools/check-undef.py --write-pending`, and no `#[link_name]` was needed for any name
this unit declares.
