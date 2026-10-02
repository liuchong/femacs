# emacs/src/data.c (4400 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 120
- [x] eq  (Feq, 2/2, doc 1028c63ff39b, line 168)
- [x] null  (Fnull, 1/1, doc 6217c247eda3, line 177)
- [ ] type-of  (Ftype_of, 1/1, doc 7e5375018e2b, line 187)
- [ ] cl-type-of  (Fcl_type_of, 1/1, doc fa1c9c6a2b31, line 202)
- [x] consp  (Fconsp, 1/1, doc 65aeaef031cc, line 311)
- [x] atom  (Fatom, 1/1, doc 0c4a192148db, line 321)
- [x] listp  (Flistp, 1/1, doc 1e19cedfe9eb, line 331)
- [x] nlistp  (Fnlistp, 1/1, doc d5dd463566d0, line 342)
- [x] bare-symbol-p  (Fbare_symbol_p, 1/1, doc cf8bec1c29b2, line 352)
- [x] symbol-with-pos-p  (Fsymbol_with_pos_p, 1/1, doc 3113a232324d, line 362)
- [x] symbolp  (Fsymbolp, 1/1, doc d4d50803b828, line 373)
- [x] keywordp  (Fkeywordp, 1/1, doc 500ffb43a138, line 383)
- [x] vectorp  (Fvectorp, 1/1, doc 52efb53d437f, line 396)
- [x] recordp  (Frecordp, 1/1, doc db8659627f9d, line 405)
- [x] stringp  (Fstringp, 1/1, doc ac824718b58d, line 414)
- [x] multibyte-string-p  (Fmultibyte_string_p, 1/1, doc 9c06010e2057, line 424)
- [x] char-table-p  (Fchar_table_p, 1/1, doc cee607bc129c, line 435)
- [x] vector-or-char-table-p  (Fvector_or_char_table_p, 1/1, doc f493aceac19b, line 444)
- [x] bool-vector-p  (Fbool_vector_p, 1/1, doc 99600a32b8e2, line 454)
- [x] arrayp  (Farrayp, 1/1, doc 1caa9fbb4716, line 463)
- [x] sequencep  (Fsequencep, 1/1, doc ce198ddb21d0, line 472)
- [x] bufferp  (Fbufferp, 1/1, doc fb2b8014f592, line 481)
- [x] markerp  (Fmarkerp, 1/1, doc 6dd1f0f02285, line 490)
- [ ] user-ptrp  (Fuser_ptrp, 1/1, doc f13a66271ac6, line 500)
- [x] subrp  (Fsubrp, 1/1, doc ca4b4c3a8f44, line 510)
- [x] closurep  (Fclosurep, 1/1, doc 18e39b08748d, line 521)
- [x] byte-code-function-p  (Fbyte_code_function_p, 1/1, doc 8417d08f8119, line 531)
- [x] interpreted-function-p  (Finterpreted_function_p, 1/1, doc 43c10e133ed1, line 541)
- [x] module-function-p  (Fmodule_function_p, 1/1, doc f485043f2913, line 551)
- [x] char-or-string-p  (Fchar_or_string_p, 1/1, doc 3688b1ae87dd, line 559)
- [x] integerp  (Fintegerp, 1/1, doc e72d1bace87f, line 569)
- [x] integer-or-marker-p  (Finteger_or_marker_p, 1/1, doc 69aab34a0f02, line 579)
- [x] natnump  (Fnatnump, 1/1, doc fa4993e77bec, line 588)
- [x] numberp  (Fnumberp, 1/1, doc d385acb002ad, line 598)
- [x] number-or-marker-p  (Fnumber_or_marker_p, 1/1, doc 9fe7352af47b, line 609)
- [x] floatp  (Ffloatp, 1/1, doc ccc5b08fdb98, line 619)
- [ ] threadp  (Fthreadp, 1/1, doc 32876f33665c, line 629)
- [ ] mutexp  (Fmutexp, 1/1, doc 9ced90777cd7, line 638)
- [ ] condition-variable-p  (Fcondition_variable_p, 1/1, doc 82ad2cc1ee4b, line 647)
- [x] car  (Fcar, 1/1, doc 94ed3b711d04, line 659)
- [x] car-safe  (Fcar_safe, 1/1, doc 682e970140af, line 670)
- [x] cdr  (Fcdr, 1/1, doc 69231fd30d7f, line 677)
- [x] cdr-safe  (Fcdr_safe, 1/1, doc e5dd456dd9a0, line 688)
- [x] setcar  (Fsetcar, 2/2, doc 1e4de21757f9, line 695)
- [x] setcdr  (Fsetcdr, 2/2, doc f85a472355bb, line 704)
- [x] boundp  (Fboundp, 1/1, doc 210c7be33e73, line 715)
- [x] fboundp  (Ffboundp, 1/1, doc 98372094c5aa, line 759)
- [x] makunbound  (Fmakunbound, 1/1, doc bf5e22734b1f, line 767)
- [x] fmakunbound  (Ffmakunbound, 1/1, doc 125cc62266a5, line 792)
- [x] symbol-function  (Fsymbol_function, 1/1, doc b6f4ce4b1691, line 810)
- [x] symbol-plist  (Fsymbol_plist, 1/1, doc 2c704bc14ab6, line 818)
- [x] symbol-name  (Fsymbol_name, 1/1, doc a9aa2c51c00c, line 826)
- [x] bare-symbol  (Fbare_symbol, 1/1, doc 3ea99e1587d6, line 840)
- [x] symbol-with-pos-pos  (Fsymbol_with_pos_pos, 1/1, doc 2c9a9ccea1bf, line 854)
- [x] remove-pos-from-symbol  (Fremove_pos_from_symbol, 1/1, doc f481f1408e2f, line 863)
- [x] position-symbol  (Fposition_symbol, 2/2, doc 9d0e2dc057aa, line 876)
- [x] fset  (Ffset, 2/2, doc 7af3608b8fe3, line 897)
- [ ] defalias  (Fdefalias, 2/3, doc 0ddc4dab1693, line 1002)
- [x] setplist  (Fsetplist, 2/2, doc 3f68e5e7c348, line 1029)
- [x] subr-arity  (Fsubr_arity, 1/1, doc 540c5ca896e3, line 1038)
- [x] subr-name  (Fsubr_name, 1/1, doc 044f0f555e5a, line 1056)
- [x] native-comp-function-p  (Fnative_comp_function_p, 1/1, doc 00558423036a, line 1067)
- [x] subr-native-lambda-list  (Fsubr_native_lambda_list, 1/1, doc 6124590e68f8, line 1074)
- [x] subr-type  (Fsubr_type, 1/1, doc 1a5b0e2f78cc, line 1089)
- [ ] subr-native-comp-unit  (Fsubr_native_comp_unit, 1/1, doc c05c24047732, line 1104)
- [ ] native-comp-unit-file  (Fnative_comp_unit_file, 1/1, doc 2842f3191124, line 1113)
- [ ] native-comp-unit-set-file  (Fnative_comp_unit_set_file, 2/2, doc 2842f3191124, line 1122)
- [x] interactive-form  (Finteractive_form, 1/1, doc 10258d736327, line 1134)
- [x] command-modes  (Fcommand_modes, 1/1, doc 1fde4b17ab80, line 1218)
- [x] indirect-variable  (Findirect_variable, 1/1, doc f3690ca988af, line 1291)
- [x] symbol-value  (Fsymbol_value, 1/1, doc fb8e10325afe, line 1645)
- [ ] set  (Fset, 2/2, doc 67b1a9246f28, line 1660)
- [ ] add-variable-watcher  (Fadd_variable_watcher, 2/2, doc 6686d609c646, line 1857)
- [ ] remove-variable-watcher  (Fremove_variable_watcher, 2/2, doc dc4012da0625, line 1885)
- [ ] get-variable-watchers  (Fget_variable_watchers, 1/1, doc 7282bb591c7c, line 1904)
- [ ] default-boundp  (Fdefault_boundp, 1/1, doc 38e91477c206, line 2006)
- [ ] default-value  (Fdefault_value, 1/1, doc f123fcac4bbc, line 2019)
- [ ] set-default  (Fset_default, 2/2, doc df2bb74cee8d, line 2130)
- [ ] make-variable-buffer-local  (Fmake_variable_buffer_local, 1/1, doc 72fb43518d24, line 2178)
- [ ] make-local-variable  (Fmake_local_variable, 1/1, doc c078681c6a7d, line 2245)
- [ ] kill-local-variable  (Fkill_local_variable, 1/1, doc 9c6b3b90cf56, line 2350)
- [ ] local-variable-p  (Flocal_variable_p, 1/2, doc 218d919232f7, line 2416)
- [ ] local-variable-if-set-p  (Flocal_variable_if_set_p, 1/2, doc 941c0a96d181, line 2465)
- [ ] variable-binding-locus  (Fvariable_binding_locus, 1/1, doc 06a9101ba8cc, line 2500)
- [ ] indirect-function  (Findirect_function, 1/2, doc 4039ab149d9c, line 2557)
- [ ] aref  (Faref, 2/2, doc 234ce1704f66, line 2568)
- [ ] aset  (Faset, 3/3, doc 46c3c9de50cc, line 2619)
- [ ] =  (Feqlsign, 1/MANY, doc 25b3b0f7463b, line 2864)
- [ ] <  (Flss, 1/MANY, doc cb7e7022c510, line 2872)
- [ ] >  (Fgtr, 1/MANY, doc a1fe723ff1b9, line 2883)
- [ ] <=  (Fleq, 1/MANY, doc bde064d281ce, line 2894)
- [ ] >=  (Fgeq, 1/MANY, doc 1702fecd3cf5, line 2905)
- [ ] /=  (Fneq, 2/2, doc 03ece8056e79, line 2916)
- [ ] number-to-string  (Fnumber_to_string, 1/1, doc 2bdc2f8fb8f1, line 3067)
- [ ] string-to-number  (Fstring_to_number, 1/2, doc daf9337fc5f1, line 3092)
- [ ] +  (Fplus, 0/MANY, doc c061e22d5133, line 3307)
- [ ] -  (Fminus, 0/MANY, doc ca2e2e6b4f09, line 3318)
- [ ] *  (Ftimes, 0/MANY, doc fbafef6431df, line 3340)
- [ ] /  (Fquo, 1/MANY, doc b394d7b907ce, line 3351)
- [ ] %  (Frem, 2/2, doc 01aadb640853, line 3438)
- [ ] mod  (Fmod, 2/2, doc a4d8f6946ac7, line 3448)
- [ ] max  (Fmax, 1/MANY, doc d12cd985bfb1, line 3476)
- [ ] min  (Fmin, 1/MANY, doc fa12411aa20f, line 3485)
- [ ] logand  (Flogand, 0/MANY, doc 788507f831dc, line 3494)
- [ ] logior  (Flogior, 0/MANY, doc 439d28cdda2c, line 3506)
- [ ] logxor  (Flogxor, 0/MANY, doc aad160b1108d, line 3518)
- [ ] logcount  (Flogcount, 1/1, doc eb5f3068a906, line 3530)
- [ ] ash  (Fash, 2/2, doc 38d49e6c895c, line 3555)
- [ ] 1+  (Fadd1, 1/1, doc 5a45c489a3cb, line 3654)
- [ ] 1-  (Fsub1, 1/1, doc 94711ababf89, line 3669)
- [ ] lognot  (Flognot, 1/1, doc fee96eb8ce5a, line 3684)
- [ ] byteorder  (Fbyteorder, 0/0, doc e53e2910e90c, line 3695)
- [ ] bool-vector-exclusive-or  (Fbool_vector_exclusive_or, 2/3, doc b7b59afea1db, line 3824)
- [ ] bool-vector-union  (Fbool_vector_union, 2/3, doc 431ab63dee0f, line 3835)
- [ ] bool-vector-intersection  (Fbool_vector_intersection, 2/3, doc 8af85dc276b4, line 3846)
- [ ] bool-vector-set-difference  (Fbool_vector_set_difference, 2/3, doc 489e3f3c5559, line 3857)
- [ ] bool-vector-subsetp  (Fbool_vector_subsetp, 2/2, doc 8aa108fe0f33, line 3868)
- [ ] bool-vector-not  (Fbool_vector_not, 1/2, doc 2fb132fb32c5, line 3877)
- [ ] bool-vector-count-population  (Fbool_vector_count_population, 1/1, doc 2038373dd2fa, line 3918)
- [ ] bool-vector-count-consecutive  (Fbool_vector_count_consecutive, 3/3, doc 18367b3ab62b, line 3943)

## Functions defined — 60
- [x] BOOLFWDP  (line 41, static)
- [x] INTFWDP  (line 46, static)
- [x] OBJFWDP  (line 51, static)
- [x] XBOOLVAR  (line 57, static)
- [x] XKBOARD_OFFSET  (line 63, static)
- [x] XINTVAR  (line 69, static)
- [x] XOBJVAR  (line 75, static)
- [x] set_blv_found  (line 82, static)
- [x] blv_value  (line 89, static)
- [x] set_blv_value  (line 95, static)
- [x] set_blv_where  (line 101, static)
- [x] set_blv_defcell  (line 107, static)
- [x] set_blv_valcell  (line 113, static)
- [x] wrong_length_argument  (line 119, static)
- [x] wrong_type_argument  (line 131)
- [x] args_out_of_range  (line 138)
- [x] args_out_of_range_3  (line 144)
- [x] circular_list  (line 150)
- [x] slow_eq  (line 160)
- [ ] add_to_function_history  (line 934, static)
- [ ] defalias  (line 971)
- [x] kboard_for_bindings  (line 1314)
- [x] do_symval_forwarding  (line 1337)
- [x] wrong_choice  (line 1365)
- [x] wrong_range  (line 1398, static)
- [x] check_choice  (line 1409, static)
- [x] check_fwd_predicate  (line 1417, static)
- [x] store_symval_forwarding  (line 1470, static)
- [x] swap_in_global_binding  (line 1548)
- [x] swap_in_symval_forwarding  (line 1574, static)
- [x] find_symbol_value  (line 1619)
- [ ] set_internal  (line 1678)
- [x] set_symbol_trapped_write  (line 1834, static)
- [x] restore_symbol_trapped_write  (line 1843, static)
- [ ] harmonize_variable_watchers  (line 1849, static)
- [ ] notify_variable_watchers  (line 1915)
- [ ] default_value  (line 1962)
- [ ] set_default_internal  (line 2034)
- [ ] make_blv  (line 2149, static)
- [ ] indirect_function  (line 2550)
- [ ] check_integer_coerce_marker  (line 2688, static)
- [ ] check_number_coerce_marker  (line 2697, static)
- [ ] coerce_marker  (line 2706, static)
- [ ] not_number_or_marker  (line 2712, static)
- [ ] arithcompare  (line 2718)
- [ ] arithcompare_driver  (line 2856, static)
- [ ] cons_to_unsigned  (line 2932)
- [ ] cons_to_signed  (line 2993)
- [ ] fixnum_to_string  (line 3048)
- [ ] floating_point_op  (line 3136, static)
- [ ] floatop_arith_driver  (line 3148, static)
- [ ] float_arith_driver  (line 3184, static)
- [ ] bignum_arith_driver  (line 3200, static)
- [ ] arith_driver  (line 3251, static)
- [ ] integer_remainder  (line 3388, static)
- [ ] minmax_driver  (line 3462, static)
- [ ] expt_integer  (line 3624)
- [ ] bool_vector_spare_mask  (line 3713, static)
- [ ] bool_vector_binop_driver  (line 3725, static)
- [ ] syms_of_data  (line 4021)

## Macros defined — 1
- [ ] PUT_ERROR  (line 4120, body c7f6139325fa)

## Types and their fields — 1
- [ ] Lisp_Val_Fwd: Lisp_Object value, lispfwd fwd

## File-scope objects — 0

## Decisions

- **Ported so far.**  data.c 41-165 (the 19 accessors and error helpers), 168-705 (40
  subrs), 716-897 (the twelve symbol accessors and `fset`), 1029-1291 (`setplist`,
  `subr-arity`, `subr-name`, `native-comp-function-p`, `subr-native-lambda-list`,
  `subr-type`, `interactive-form`, `command-modes`, `indirect-variable`),
  1314-1646 (the forwarding machinery) and 1832-1846
  (`set_symbol_trapped_write`/`restore_symbol_trapped_write`).  `cargo build-tu src/data.c`
  builds, and `extract-abi.py --check` reports no incorrect item - only the 55 remaining
  subrs, 29 functions, 1 macro (`PUT_ERROR`), 12 constants (7 `A*` plus the five
  `bool_vector_*`), the 120 `defsubr` calls and the line-count drift of a partial file
  (C 4400 vs Rust 1362, 69.0%).
- **Five subrs wait for two headers.**  `type-of` (data.c 187-200) needs this file's own
  `cl-type-of` (data.c 202-310); `cl-type-of` calls `THREADP`, `MUTEXP`, `CONDVARP`
  (`thread.h` 222-320) and `FONT_SPEC_P`/`FONT_ENTITY_P`/`FONT_OBJECT_P` (`font.h`
  441-470); `threadp`, `mutexp` and `condition-variable-p` (data.c 628-658) call the three
  `thread.h` predicates.  Neither header is mirrored yet (`thread.h` pulls `regex-emacs.h`,
  `android.h`, `sysselect.h`, `systhread.h`; `font.h` is 1034 lines of batch E), and
  PORTING.md P-69/P-70 let a mirror name only what it defines, so an inline predicate of an
  unmirrored header has no stand-in.  Those five boxes stay unchecked until `thread.rs` and
  `font.rs` land; everything around them is ported, so the file keeps C's order.
- **`user-ptrp` (data.c 499-508) is not ported: it is dead in this configuration.**
  `HAVE_MODULES` is `undef` in the frozen table (`config.rs`'s `HAVE_MODULES = false`), so C
  compiles neither the `DEFUN` nor its `defsubr` (data.c 4360-4362).  Keeping the text as
  `#[cfg(feature = "modules")]` was tried in this round and reverted: with `src/data.c` now
  in `cond-eval-verified.txt` the gate reads the frozen text, so the arm C never compiles is
  reported as `the reference file has no such DEFUN` (an invented item, D3).  The subr is
  therefore absent from the port, and PORTING.md P-76 records the remaining half of the
  story: the tool's `defsubr` extraction does not consult the condition table yet, so it
  still expects `defsubr (&Suser_ptrp)`; `syms_of_data` must record that as a ledger
  deviation rather than write the call.
- **`MODULE_FUNCTIONP` / `XMODULE_FUNCTION` are unconditional (review B's F2).**  The port
  had gated the pair in `lisp.rs` on a `modules` feature that no build ever passes, leaving
  the frozen build with a `false` stub for a predicate C compiles as
  `PSEUDOVECTORP (o, PVEC_MODULE_FUNCTION)` (lisp.h 4872-4883; the `#ifdef HAVE_MODULES` in
  that region opens *after* them, at 4885).  The gates are gone and this file's
  `module-function-p` (data.c 552-558) now calls the predicate C calls.
- **`NO_INLINE` and `AVOID` were attached to the wrong functions (review B's F4).**  C marks
  `slow_eq` `NO_INLINE` (data.c 159) and `wrong_length_argument` `static AVOID` (data.c
  118), which is `_Noreturn ATTRIBUTE_COLD void` (lisp.h 153), not `NO_INLINE`.  The port
  now has `#[inline(never)]` on `slow_eq` only, and the `AVOID` helper keeps its `-> !` with
  `ATTRIBUTE_COLD` dropped as PORTING.md 490-491 says.
- **`attributes: const` / `attributes: pure` are not expressed.**  Fifteen subrs in this
  range carry them (9 `const`, 6 `pure`; the whole file has 16).  C's `attributes:` list
  rides on the `doc:` argument of `DEFUN` (lisp.h 3434), which the macro never uses, so the
  compiler never sees it; only `make-docfile` records it, and `tools/extract-abi.py`
  compares `attributes: noreturn` alone.  Writing the clause would invent a `DEFUN!` arm the
  frozen reference does not need (PORTING.md P-25).
- **Docstrings are make-docfile's value.**  Each `doc: /* ... */` comment becomes a Rust
  string literal with the first line's leading whitespace and the final trailing whitespace
  stripped and the internal newlines kept (`\n`).  Both reviewers re-derived that
  transformation from `make-docfile.c` 283-473 and matched all 40 literals byte for byte,
  independently of `--check`'s own digest comparison.
- **C's forward declaration of `swap_in_symval_forwarding` (data.c 36-38) is deleted**, per
  PORTING.md 0.1-12; Rust needs no declaration, so `boundp`'s call (data.c 745) will resolve
  against the definition at data.c 1574 when that function lands.  The stand-in static the
  previous round left in `data.rs` is gone.
- **`src/data.c` joined the condition table** (`docs/baseline/config/cond-eval-verified.txt`,
  18 files now): `tools/check-cond-eval.py src/data.c` finds 9 conditions, 0 unknown and 0
  disagreements with clang, so the gate now reads this file's frozen live text instead of
  being blind to its `#ifdef`s - which is how the `user-ptrp` question above became visible
  at all (review A's P1, review B's F1).
- **Next segment (planning note).**  `boundp` (data.c 745) calls
  `swap_in_symval_forwarding` (data.c 1574), whose closure is
  `do_symval_forwarding` (1337), `store_symval_forwarding` (1470),
  `check_fwd_predicate` (1417), `kboard_for_bindings` (1314) and the three
  `wrong_*`/`check_choice` helpers (1365-1469) - so the file is ported in C order in
  segments, and the segment *after* this one is the forwarding machinery
  (data.c 1314-1646, ending at `Fsymbol_value`), which leaves 931-1313 (the
  `defalias` family) as the one hole.  That hole needs `comp.h`: `Fdefalias` (data.c
  1019) calls `maybe_defer_native_compilation`, which comp.h declares as an `extern`
  under `HAVE_NATIVE_COMP` and as a **`static inline` no-op** in the `#else` branch
  (comp.h 82-96) that this configuration compiles - i.e. the call is live but
  resolves to an empty body, so it needs `comp.rs` (106 lines) rather than a
  substitute.
- **Segment order inside the file.**  The second segment is 1314-1646 rather than the
  C-adjacent 716-930 because `boundp` (data.c 745) calls `swap_in_symval_forwarding`
  (1574), whose own closure is `do_symval_forwarding` (1337), `store_symval_forwarding`
  (1470), `check_fwd_predicate` (1417) and `kboard_for_bindings` (1314): Rust has no
  forward declarations (PORTING.md 0.1-12), so the machinery has to exist before the
  accessors that call it can be part of a compiling translation unit.  716-897 landed next; the only hole left in the first half is
  931-1028 (the `defalias` family), which waits for `comp.rs` (comp.h's `static inline` no-op, PORTING.md P-51's text-preservation rule).
- **Four lexical notes in `docs/ports/macro-body-notes.tsv`.**  C's `switch`/`case`/
  `default`, `start:`/`goto start` and `FOR_EACH_LIVE_BUFFER` have no Rust spelling, so
  those four function names carry an explicit exemption list (the `doprnt` precedent on the
  same file); every other token of those bodies is still compared.
- **The EXFUN prototypes are imported, not re-declared.**  C sees every `EXFUN (Ffoo, …)`
  because lisp.h includes the generated globals.h; the mirror keeps them in
  `globals.rs`'s `pub mod subrs`, so `lisp.rs` (which includes globals.rs) now carries
  `use subrs::*;` - the same visibility through Rust's own mechanism, which is what lets
  data.c's `Fconcat`/`Fmemq`/`Fleq`/`Fnumber_to_string`/`Fget`/`Fcurrent_buffer` calls
  read exactly like the C (PORTING.md 0.1-14).
- **Second review round (2026-10-02).**  `docs/reviews/2026-10-02-data-review-2.md`
  records it.  Two real defects came out of it that are *not* in this file's text: the
  mirror's `integer_to_intmax`/`integer_to_uintmax` had invented `|| INTEGERP (num)`
  (lisp.h 4109/4122 is `i != 0`), which made this file's `store_symval_forwarding`
  `Lisp_Fwd_Int` arm write 0 instead of signalling `overflow-error`; and
  `swap_in_symval_forwarding` must write the `found` bit directly, because the asserted
  `set_blv_found` does not hold mid-swap.  Both are fixed, together with `wrong_choice`'s
  missing `-> !`.  The gate gained the reverse `no_mangle` rule (PORTING.md P-77).
- **Evidence for this segment beyond `--check`.**  `rustc --cfg enable_checking` on the
  materialized unit (the checking pass only covers finished units) compiles the four new
  `eassert!` bodies, and `nm` on the object shows exactly C's linkage split: the five
  exported helpers as `T _kboard_for_bindings`/`_do_symval_forwarding`/`_wrong_choice`/
  `_swap_in_global_binding`/`_find_symbol_value` (plus `_Fsymbol_value`), the five `static`
  ones as private mangled names.  Both reviewers also recorded what the token gate cannot
  see here: arm order, `>` vs `>=` in the `buffer_defaults` range test, and `idx <= 0`
  vs `idx < 0` are all silent, so those three points rest on the two recorded human
  readings until the ERT/differential layer exists.
- **This file is where the subr-body hole surfaced (P-78).**  While porting 716-930 the
  gate was probed with a mutation (`Feq`'s `EQ` -> `BASE_EQ`) and the mutation passed
  every check, which led to `cinventory.defun_spans` and the loop that now compares subr
  bodies too: 150 bodies across the ported units, 120 of them here.  The sweep's first
  results for this file are three syntax-level exemptions - `Fboundp`'s
  `switch`/`goto`/`redirect`, `Fmakunbound`'s and `Findirect_variable`'s `redirect`, and
  `set_symbol_trapped_write`'s `trapped_write` (the mirror packs those bitfields into
  `u.s.bits`) - in `docs/ports/macro-body-notes.tsv`.
- **Three subrs are dead in this configuration and are not ported.**
  `subr-native-comp-unit`, `native-comp-unit-file` and `native-comp-unit-set-file`
  (data.c 1106-1134) are inside `#ifdef HAVE_NATIVE_COMP`, which the frozen table leaves
  undefined, so the reference builds neither them nor their `defsubr` calls; the same rule
  `user-ptrp` follows applies, and their three boxes stay unchecked.  `syms_of_data`
  (data.c 4021) has to treat those three `defsubr` lines as a ledger deviation: P-76
  records that the tool's defsubr extraction does not consult the condition table yet, so
  writing them would be reported as missing rather than as dead.
- **This segment's three dead-text decisions (recorded, not implied by the code).**
  (a) `subr-native-lambda-list` and `subr-type` keep C's `#ifdef HAVE_NATIVE_COMP` text
  under `#[cfg(feature = "native_comp")]` (the `floatfns.rs` `no_rint` precedent), and
  *that text does compile*: a run of `xtask dead-arms` with this ledger presented as
  complete compiled `src_data.o` under the flipped configuration (the `native_comp` entry
  in `FLIPS` flips the constant and the feature together; the `const _` assertion in
  `lisp.rs` is what makes the pair meaningful).  Until the ledger is complete,
  `finished_units()` skips this unit, so that text is compiled at batch boundaries rather
  than on every gate run - recorded here as the residual it is.
  (b) The three `#ifdef HAVE_NATIVE_COMP` DEFUNs are not ported (bullet above).
  (c) `interactive-form` and `command-modes` drop C's `#ifdef HAVE_MODULES` arms: the
  frozen text has no such branch, so the `if`/`else if` chain is the same shape without
  it.  Unlike (b) this is a *choice* (the tokens would not be compared either way,
  because `c_body_text` drops dead lines) - it is recorded because the port could have
  kept it under `#[cfg(feature = "modules")]`, and chose not to while that feature has no
  mirror for the `module_function_*` helpers it would need.
- **A real defect the reviewers caught in this segment (review B's P1).**
  `Findirect_variable` must rebind the parameter (`XSETSYMBOL (object, sym)`) and return
  it; the port first put the mutable binding *inside* the `if`, so the write landed on a
  shadow and the returned value stayed the parameter - the function's only purpose,
  following an alias chain, silently did nothing.  The token gate cannot see this class
  ("every token survives"): it was found by the human reading, and the fix hoists the
  binding to the function's scope (the `floatfns.rs`/`buffer.rs` precedent).  This is
  recorded in P-78's residuals as the third blind-spot class.
