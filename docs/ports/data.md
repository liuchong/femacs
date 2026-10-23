# emacs/src/data.c (4400 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 120
- [x] eq  (Feq, 2/2, doc 1028c63ff39b, line 168)
- [x] null  (Fnull, 1/1, doc 6217c247eda3, line 177)
- [x] type-of  (Ftype_of, 1/1, doc 7e5375018e2b, line 187)
- [x] cl-type-of  (Fcl_type_of, 1/1, doc fa1c9c6a2b31, line 202)
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
- [x] user-ptrp  (Fuser_ptrp, 1/1, doc f13a66271ac6, line 500)  — dead in the frozen configuration: `#ifdef HAVE_MODULES` is untaken
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
- [x] threadp  (Fthreadp, 1/1, doc 32876f33665c, line 629)
- [x] mutexp  (Fmutexp, 1/1, doc 9ced90777cd7, line 638)
- [x] condition-variable-p  (Fcondition_variable_p, 1/1, doc 82ad2cc1ee4b, line 647)
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
- [x] defalias  (Fdefalias, 2/3, doc 0ddc4dab1693, line 1002)
- [x] setplist  (Fsetplist, 2/2, doc 3f68e5e7c348, line 1029)
- [x] subr-arity  (Fsubr_arity, 1/1, doc 540c5ca896e3, line 1038)
- [x] subr-name  (Fsubr_name, 1/1, doc 044f0f555e5a, line 1056)
- [x] native-comp-function-p  (Fnative_comp_function_p, 1/1, doc 00558423036a, line 1067)
- [x] subr-native-lambda-list  (Fsubr_native_lambda_list, 1/1, doc 6124590e68f8, line 1074)
- [x] subr-type  (Fsubr_type, 1/1, doc 1a5b0e2f78cc, line 1089)
- [x] subr-native-comp-unit  (Fsubr_native_comp_unit, 1/1, doc c05c24047732, line 1104)  — dead in the frozen configuration: `#ifdef HAVE_NATIVE_COMP` is untaken
- [x] native-comp-unit-file  (Fnative_comp_unit_file, 1/1, doc 2842f3191124, line 1113)  — dead like the above
- [x] native-comp-unit-set-file  (Fnative_comp_unit_set_file, 2/2, doc 2842f3191124, line 1122)  — dead like the above
- [x] interactive-form  (Finteractive_form, 1/1, doc 10258d736327, line 1134)
- [x] command-modes  (Fcommand_modes, 1/1, doc 1fde4b17ab80, line 1218)
- [x] indirect-variable  (Findirect_variable, 1/1, doc f3690ca988af, line 1291)
- [x] symbol-value  (Fsymbol_value, 1/1, doc fb8e10325afe, line 1645)
- [x] set  (Fset, 2/2, doc 67b1a9246f28, line 1660)
- [x] add-variable-watcher  (Fadd_variable_watcher, 2/2, doc 6686d609c646, line 1857)
- [x] remove-variable-watcher  (Fremove_variable_watcher, 2/2, doc dc4012da0625, line 1885)
- [x] get-variable-watchers  (Fget_variable_watchers, 1/1, doc 7282bb591c7c, line 1904)
- [x] default-boundp  (Fdefault_boundp, 1/1, doc 38e91477c206, line 2006)
- [x] default-value  (Fdefault_value, 1/1, doc f123fcac4bbc, line 2019)
- [x] set-default  (Fset_default, 2/2, doc df2bb74cee8d, line 2130)
- [x] make-variable-buffer-local  (Fmake_variable_buffer_local, 1/1, doc 72fb43518d24, line 2178)
- [x] make-local-variable  (Fmake_local_variable, 1/1, doc c078681c6a7d, line 2245)
- [x] kill-local-variable  (Fkill_local_variable, 1/1, doc 9c6b3b90cf56, line 2350)
- [x] local-variable-p  (Flocal_variable_p, 1/2, doc 218d919232f7, line 2416)
- [x] local-variable-if-set-p  (Flocal_variable_if_set_p, 1/2, doc 941c0a96d181, line 2465)
- [x] variable-binding-locus  (Fvariable_binding_locus, 1/1, doc 06a9101ba8cc, line 2500)
- [x] indirect-function  (Findirect_function, 1/2, doc 4039ab149d9c, line 2557)
- [x] aref  (Faref, 2/2, doc 234ce1704f66, line 2568)
- [x] aset  (Faset, 3/3, doc 46c3c9de50cc, line 2619)
- [x] =  (Feqlsign, 1/MANY, doc 25b3b0f7463b, line 2864)
- [x] <  (Flss, 1/MANY, doc cb7e7022c510, line 2872)
- [x] >  (Fgtr, 1/MANY, doc a1fe723ff1b9, line 2883)
- [x] <=  (Fleq, 1/MANY, doc bde064d281ce, line 2894)
- [x] >=  (Fgeq, 1/MANY, doc 1702fecd3cf5, line 2905)
- [x] /=  (Fneq, 2/2, doc 03ece8056e79, line 2916)
- [x] number-to-string  (Fnumber_to_string, 1/1, doc 2bdc2f8fb8f1, line 3067)
- [x] string-to-number  (Fstring_to_number, 1/2, doc daf9337fc5f1, line 3092)
- [x] +  (Fplus, 0/MANY, doc c061e22d5133, line 3307)
- [x] -  (Fminus, 0/MANY, doc ca2e2e6b4f09, line 3318)
- [x] *  (Ftimes, 0/MANY, doc fbafef6431df, line 3340)
- [x] /  (Fquo, 1/MANY, doc b394d7b907ce, line 3351)
- [x] %  (Frem, 2/2, doc 01aadb640853, line 3438)
- [x] mod  (Fmod, 2/2, doc a4d8f6946ac7, line 3448)
- [x] max  (Fmax, 1/MANY, doc d12cd985bfb1, line 3476)
- [x] min  (Fmin, 1/MANY, doc fa12411aa20f, line 3485)
- [x] logand  (Flogand, 0/MANY, doc 788507f831dc, line 3494)
- [x] logior  (Flogior, 0/MANY, doc 439d28cdda2c, line 3506)
- [x] logxor  (Flogxor, 0/MANY, doc aad160b1108d, line 3518)
- [x] logcount  (Flogcount, 1/1, doc eb5f3068a906, line 3530)
- [x] ash  (Fash, 2/2, doc 38d49e6c895c, line 3555)
- [x] 1+  (Fadd1, 1/1, doc 5a45c489a3cb, line 3654)
- [x] 1-  (Fsub1, 1/1, doc 94711ababf89, line 3669)
- [x] lognot  (Flognot, 1/1, doc fee96eb8ce5a, line 3684)
- [x] byteorder  (Fbyteorder, 0/0, doc e53e2910e90c, line 3695)
- [x] bool-vector-exclusive-or  (Fbool_vector_exclusive_or, 2/3, doc b7b59afea1db, line 3824)
- [x] bool-vector-union  (Fbool_vector_union, 2/3, doc 431ab63dee0f, line 3835)
- [x] bool-vector-intersection  (Fbool_vector_intersection, 2/3, doc 8af85dc276b4, line 3846)
- [x] bool-vector-set-difference  (Fbool_vector_set_difference, 2/3, doc 489e3f3c5559, line 3857)
- [x] bool-vector-subsetp  (Fbool_vector_subsetp, 2/2, doc 8aa108fe0f33, line 3868)
- [x] bool-vector-not  (Fbool_vector_not, 1/2, doc 2fb132fb32c5, line 3877)
- [x] bool-vector-count-population  (Fbool_vector_count_population, 1/1, doc 2038373dd2fa, line 3918)
- [x] bool-vector-count-consecutive  (Fbool_vector_count_consecutive, 3/3, doc 18367b3ab62b, line 3943)

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
- [x] add_to_function_history  (line 934, static)
- [x] defalias  (line 971)
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
- [x] set_internal  (line 1678)
- [x] set_symbol_trapped_write  (line 1834, static)
- [x] restore_symbol_trapped_write  (line 1843, static)
- [x] harmonize_variable_watchers  (line 1849, static)
- [x] notify_variable_watchers  (line 1915)
- [x] default_value  (line 1962)
- [x] set_default_internal  (line 2034)
- [x] make_blv  (line 2149, static)
- [x] indirect_function  (line 2550)
- [x] check_integer_coerce_marker  (line 2688, static)
- [x] check_number_coerce_marker  (line 2697, static)
- [x] coerce_marker  (line 2706, static)
- [x] not_number_or_marker  (line 2712, static)
- [x] arithcompare  (line 2718)
- [x] arithcompare_driver  (line 2856, static)
- [x] cons_to_unsigned  (line 2932)
- [x] cons_to_signed  (line 2993)
- [x] fixnum_to_string  (line 3048)
- [x] floating_point_op  (line 3136, static)
- [x] floatop_arith_driver  (line 3148, static)
- [x] float_arith_driver  (line 3184, static)
- [x] bignum_arith_driver  (line 3200, static)
- [x] arith_driver  (line 3251, static)
- [x] integer_remainder  (line 3388, static)
- [x] minmax_driver  (line 3462, static)
- [x] expt_integer  (line 3624)
- [x] bool_vector_spare_mask  (line 3713, static)
- [x] bool_vector_binop_driver  (line 3725, static)
- [x] syms_of_data  (line 4021)

## Macros defined — 1
- [x] PUT_ERROR  (line 4120, body c7f6139325fa)

## Types and their fields — 1
- [x] Lisp_Val_Fwd: Lisp_Object value, lispfwd fwd

## File-scope objects — 0

- **The two `goto next_arg`s are the loop's label and its back edge (this segment's
  decision).**  `floatop_arith_driver` and `bignum_arith_driver` jump from *above* their
  `while` into it, skipping the switch on the first pass, and C's `next_arg:` label marks
  where the tail statements start.  The port names the loop `'next_arg` and spells the
  loop's back edge as `continue 'next_arg`, so the `va_arg/next_arg` family in
  `docs/ports/token-aliases.tsv` (whose occurrence check has no exemption mechanism) still
  sees both of C's spellings; the "skip the switch once" jump itself is the `skip_switch`
  flag, whose first pass is the jump's target.
- **`set_dest` is a labelled loop, not a flag.**  `bool_vector_binop_driver`'s
  `goto set_dest` leaves the `if` branch's switch and lands on the tail switch; the port
  makes the whole body `'set_dest: loop` and jumps with `break 'set_dest`, which keeps C's
  label and its four destinations, so only the keyword `goto` needs a `macro-body-notes`
  row.
- **`<stdbit.h>` now has a mirror (`lib/include/stdbit.rs`).**  `lisp.h` 26 includes
  the C23 header, so `lisp.rs` includes the mirror in the same position; it carries the
  five width variants of `stdc_count_ones`, `stdc_leading_zeros` and
  `stdc_trailing_zeros` plus C's `sizeof`-dispatcher as three `macro_rules!` macros.  The
  `_GL_STDBIT_HAS_BUILTIN_*` branches are the live ones, so the helpers are Rust's
  `count_ones`/`leading_zeros`/`trailing_zeros` (P-26, P-79).
- **`lisp.rs` also gained the two bool-vector constructors** (`make_clear_bool_vector`,
  `make_uninit_bool_vector`) as `extern "C"` declarations for `alloc.c`'s definitions
  (lisp.h 4537-4538), which is what `bool_vector_binop_driver` calls.
- **`PUT_ERROR` is a `macro_rules!` inside `syms_of_data`.**  C defines the macro in the
  middle of the body, so the port does too: the parameter names stay `sym`, `tail`, `msg`
  and the two `Fput` calls stay in the expansion, which is what keeps the C body's tokens
  visible to the comparison.  Its `msg` is a `c"..."` literal because `build_string` wants
  a NUL-terminated `char *`.
- **`Vmost_positive_fixnum`, `Vmost_negative_fixnum` and `symbols_with_pos_enabled` have
  no definition in data.c.**  The reference's generated globals.h turns them into fields of
  `struct emacs_globals` (`globals.h` 476-477: `#define Vmost_positive_fixnum
  globals.f_Vmost_positive_fixnum`), which is why data.c can assign them; the port's
  generated `globals.rs` declares the same names, so the `DEFVAR_LISP!`/`DEFVAR_BOOL!`
  invocations here address the generated objects (the same shape `casetab.rs` uses for its
  own globals).
- **`TYPE_MAXIMUM (mp_bitcnt_t)` is `mp_bitcnt_t::MAX`.**  gnulib's intprops.h has no mirror
  (P-26); `mp_bitcnt_t` is `unsigned long` (mini-gmp.h 68), so the port writes the type's
  maximum and the `macro-body-notes` row for `Fash` records it (the `esprintf` precedent).
- **C's `attributes: const` on `byteorder` has no counterpart.**  `DEFUN!` accepts only
  `attributes: noreturn` (the marker is make-docfile's, and the port's `Lisp_Subr` record
  has no field for it), so the port's invocation stops at the docstring.
- **Six live subrs and two functions are still gaps** (187-311 `type-of`/`cl-type-of`,
  629-658 `threadp`/`mutexp`/`condition-variable-p`, 934-1020 `add_to_function_history`/
  `defalias`/`defalias`), which is why the checklist above keeps their boxes unchecked and
  why `syms_of_data` keeps their six `defsubr` calls as comments.  Porting them is what
  removes the last line-drift and the six-name exemption in `macro-body-notes.tsv`.

- **The `defalias` block needed three mirrors, and two of them are no-ops.**  `defalias`
  itself calls `maybe_defer_native_compilation`, whose live definition is comp.h's
  `#else /* HAVE_NATIVE_COMP */` empty `static inline` (comp.h 90-95) - the file
  `src/include/comp.rs` is that branch, and its empty body is the reference's, not a
  port stub.  `add_to_function_history` iterates `Vcurrent_load_list` with
  `FOR_EACH_TAIL_SAFE` (`FOR_EACH_TAIL_INTERNAL (tail, (void) ((tail) = Qnil), false)`),
  which the port mirrors once in `lisp.rs` as `FOR_EACH_TAIL_SAFE!` with C's Brent
  tortoise-hare stepper spelled out (`for_each_tail_internal`, `FOR_EACH_TAIL_THRESHOLD`,
  `POWER_OF_2`); the C macro's body and each of its callers then differ only in the loop
  form, which is why the notes row for it is empty of exemptions.  The macro re-binds the
  caller's loop variable (C's `for` mutates the caller's object) and takes the body as a
  second argument, because a Rust macro cannot own the statement that follows it.

- **`ternary`-free: this is the first file whose checklist can be closed.**  All 116 live
  subrs, all 60 functions, the single file macro (`PUT_ERROR`) and the one struct
  (`Lisp_Val_Fwd`) are ported, and `--check` says `OK`.  Two tooling facts made that
  possible this round: `extract-abi.py`'s `defsubr` extraction now evaluates the condition
  table (PORTING.md P-76, whose row this round flips from 待修 to 已修), so `syms_of_data`
  does not have to spell four registrations the reference never compiles; and the two
  header mirrors those four blocks need (`font.rs` for `cl-type-of`'s `FONT_SPEC_P` family,
  `thread.rs` for `THREADP`/`MUTEXP`/`CONDVARP`) are now in the include chain, at the
  positions C includes them (`lisp.h` 2289 for thread.h, `composite.h` 28 for font.h).
- **`PSEUDOVECTOR_TYPE`'s mirror signature was wrong** and this file is the first caller:
  lisp.h 1784 takes `const struct Lisp_Vector *` and reads `v->header.size`, while the
  mirror said `*mut vectorlike_header` and read `(*v).size`.  Both are now C's.
- **`cl-type-of`'s two switches are `match`es with a trailing `_ =>` arm** ("Impossible"
  cases and any enumerator C does not name), which is the arm that reaches the `emacs_abort
  ()` after the inner switch; the `macro-body-notes.tsv` row records the three keywords.

- **Ported so far.**  data.c 41-165 (the 19 accessors and error helpers), 168-705 (40
  subrs), 716-897 (the twelve symbol accessors and `fset`), 1029-1291 (the subr
  introspection subrs), 1314-1646 (the forwarding machinery), 1660-1676 (`set`, in C's
  order ahead of its callee), 1678-1960 (`set_internal`, the watcher machinery and its
  three subrs), 1832-1846 (`set_symbol_trapped_write`/`restore_symbol_trapped_write`),
  1962-2176 (`default_value`, the three `default-*` subrs, `set_default_internal`,
  `union Lisp_Val_Fwd` and `make_blv`), 2178-2412 (`make-variable-buffer-local`,
  `make-local-variable`, `kill-local-variable`), 2414-2564 (`local-variable-p`,
  `local-variable-if-set-p`, `variable-binding-locus`, `indirect_function` and
  `indirect-function`), 2566-2717 (`aref`, `aset` and the four marker-coercion helpers of
  the arithmetic section), 2718-2930 (`arithcompare`, `arithcompare_driver` and the six
  comparison subrs `=`/`<`/`>`/`<=`/`>=`/`/=`), 2931-3126 (the two cons converters,
  `fixnum_to_string` and the `number-to-string`/`string-to-number` pair), and 3126-4400
  (`enum arithop`, `floating_point_op`, the three arithmetic drivers, the 17 arithmetic
  subrs, `enum bool_vector_op`, `bool_vector_spare_mask`, `bool_vector_binop_driver` and
  the eight bool-vector subrs, then `syms_of_data` with its 138 `DEFSYM`s, the `PUT_ERROR`
  macro, two `DEFVAR_LISP`s, one `DEFVAR_BOOL` and 116 `defsubr` calls).  The file has 120
  `DEFUN`s, of which four are dead in this configuration (`user-ptrp` plus the three
  `HAVE_NATIVE_COMP` ones): all 116 live subrs and all 60 functions are
  ported.  **The file is complete**: `cargo build-tu src/data.c` builds and
  `extract-abi.py --check emacs/src/data.c src/data.rs` reports `OK` - no missing
  function, subr, macro, constant, field or `defsubr`, and no line-count drift (C 4400 vs
  Rust 4393, 0.2%).  The four remaining subrs of the 120 are the ones the frozen
  configuration never compiles (`user-ptrp` under `#ifdef HAVE_MODULES`, the three
  `HAVE_NATIVE_COMP` registrations); the checker stops requiring them once its `defsubr`
  extraction evaluates the condition table (PORTING.md P-76, fixed this round), and none of
  them is ported, because there is no compiled C to mirror.

- **`UNINIT` is `= {0,}` here, and that is a measured fact (this segment's decision).**
  `conf_post.h` 366-371 defines `UNINIT` as `= {0,}` when `GCC_LINT` or `lint` is defined
  and as nothing otherwise; `configure.ac` 1894 defines `GCC_LINT` unconditionally inside
  the warnings block, and the fresh `configure` of this round put `#define GCC_LINT 1` on
  line 209 of the generated `build/emacs-ref/src/config.h`.  So C's `union Lisp_Val_Fwd valcontents UNINIT;` and
  `bool forwarded UNINIT;` (data.c 2202-2203, 2268-2269) are *zero initializers*: the port
  writes `Lisp_Val_Fwd { value: Qnil () }` and `false` (`NIL_IS_ZERO` makes the union's
  zero the same word).  The gate compares the *unexpanded* text (its tokenizer blanks
  comments and drops dead lines, nothing else), so `UNINIT` itself is in those two bodies'
  compared token sets - which is why their notes rows carry `uninit` with this reason.  The same decision covers data.c
  2935/2996 (`uintmax_t val UNINIT`, `intmax_t val UNINIT`).
- **The three new bodies' `break` is C's switch-arm exit (P-78's set-vs-meaning class).**
  Each ported body keeps C's `switch` as a `match` and exits its `loop` with a single
  `break`; that token satisfies C's per-arm `break` in the set comparison although the two
  are different control transfers.  The mirror image of the same blind spot is the alias
  arm: C's `goto start` becomes the Rust `continue`, a token the C body never has, so
  reviewer B's mutation `continue` -> `break` in any of the three bodies is invisible to
  every gate (measured; the `break` it leaves behind even satisfies C's switch-arm token).
  Nothing else in those bodies depends on this, and the `match` arms cannot fall through,
  so the port's control flow *is* C's - recorded because the gate cannot see it.
- **`SET_PER_BUFFER_VALUE_P`'s third argument is invisible in `kill-local-variable`.**
  C passes `0` there and `true` in `make-local-variable`; the port passes
  `false`/`true`.  `kill-local-variable`'s `0` token is satisfied by `if (idx > 0)`, so
  swapping that `false` for `true` would not be reported - a P-78-class residual, listed
  with the casetab `0`/`1` rows.
- **Probe P1 is now fully ported on both halves.**  `make-variable-buffer-local` sets the
  `local_if_set` bit and `local-variable-if-set-p` (data.c 2465) reads it back through
  `BLV_LOCAL_IF_SET`; only an Emacs build to run the probe against is still missing.
- **C's one `FALLTHROUGH` (data.c 2528) is the port's shared tail.**  In
  `variable-binding-locus`, the `SYMBOL_FORWARDED` arm either returns (kboard or
  non-buffer forwarding) or falls into the `SYMBOL_LOCALIZED` arm's statements.  Rust has
  no fallthrough, so the port keeps C's shape by giving the `match` a loop whose `break`
  lands on exactly those statements: the `SYMBOL_LOCALIZED` arm is empty and the two arms
  that reach the `break` are the two that reach C's tail.  The tail's own
  `sym->u.s.redirect == SYMBOL_LOCALIZED` test is kept verbatim - it is what distinguishes
  the fallthrough case - and `FALLTHROUGH` (`lib/attribute.h` 210) therefore appears in
  that body's notes row.
- **Two of this segment's switches have no `break` at all.**  `local-variable-p` and
  `local-variable-if-set-p` return, `continue` or abort from every arm, so their ported
  loops have no `break` either; a `loop` without one has type `!`, which is what the
  `DEFUN!` body has to produce (`variable-binding-locus` needs the `break`, above).
- **The `unused_parens` warnings this TU emits come from the mirror's bitfield spelling,
  not from C (review A corrected an earlier version of this bullet).**  `build-tu src/data.c`
  reports four "unnecessary parentheses around an `if` condition", at `data.rs` 1749, 1993 and
  2097 and once inside an included mirror; all of them are the mirror's re-derivation
  `if ((((*sym).u.s.bits & MASK) >> SHIFT) ...)`, whereas C writes `if (sym->u.redirect ==
  SYMBOL_LOCALIZED)` with no inner pair, and this segment adds none (its own condition at
  `data.rs` 2260 puts the comparison outside the parens).

- **What this segment's notes rows still cannot see (reviewer B's round-7 mutation run).**
  The three rows are *exactly* the difference set (B measured `C - Rust - allowance = {}` and
  `allowance - C = {}` for all three), so no exemption is wider than the C text it explains -
  but a token-set check cannot police the *values* the port puts where the C token stood.  B
  built 30+ mutants against `--check` and the gate missed these, each verified by command:
  the `UNINIT` initializers (`Lisp_Val_Fwd { value: Qnil () }` -> `Qunbound ()`/`Qt ()`, or
  deleting the initializer entirely - the `uninit` exemption leaves the initializer's own
  value unguarded, and `qnil` is satisfied by the `valcontents.value = Qnil ()` in the
  PLAINVAL arm), `forwarded`'s polarity in all four places, deleting the whole packed-bit
  `redirect` write (the `SYMBOL_LOCALIZED` of the write is satisfied by the `match` arm head
  and `redirect` by the dispatch read), swapping `SYMBOL_REDIRECT_MASK/SHIFT` for
  `SYMBOL_TRAPPED_WRITE_MASK/SHIFT` anywhere (those names have no C counterpart at all),
  `| BLV_LOCAL_IF_SET` -> `& ! BLV_LOCAL_IF_SET` (the family is an occurrence *lower bound*,
  not a polarity check), `continue` -> `break` (above), `Fdelq (tem, ...)` -> `Fdelq
  (variable, ...)`, `if idx > 0` -> `idx < 0`, `SET_PER_BUFFER_VALUE_P`'s third argument
  (`true` in `make-local-variable` is satisfied by `forwarded = true`, so only the
  `kill-local-variable` half was recorded before this bullet), `if ! (*blv).fwd.is_null ()`
  losing its `!`, `forwarded && ...` -> `forwarded || ...`, and `CALLN!` -> `calln!`.  The
  matching P-78 residual text records the list.  Two mechanisms would close most of it and
  are deliberately *not* being invented mid-segment: a per-declaration binding table
  (`docs/ports/init-values.tsv`, the `va-arg-types.tsv` shape) for initializer values, and a
  per-C-bitfield table (`redirect`/`trapped_write`/`found` -> their mirror constant pairs) for
  the mask/shift choices.  Reviewer A also asks for the two bitfield layouts to join the
  rerunnable `spikes/write_probe.*` artifacts (they currently rest on the round-3 clang+gcc
  probe and ABI reasoning), which is the cheapest of the three.
- **The ledger's generated header vs the generator (review B's F9).**  `extract-abi.py
  emacs/src/data.c` and `--check` now emit `## Subrs (DEFUN) - 116` (the condition table drops
  the four dead `DEFUN`s), while this file deliberately keeps C's 120 boxes (P-76).  The
  arithmetic stays self-consistent - 75 ticked + 45 unticked = 120, the four dead ones stay
  unticked - but a future regeneration of this checklist would emit 116 boxes and lose the
  four recorded dead lines, so this paragraph is the record that the 120 is deliberate.
- **The new bodies repeat the round-7 blind spots, and here the fallthrough itself is outside
  the gate (reviewer B's round-8 mutation run).**  B's mutants against `--check`, each verified
  by command, stay green for: replacing the empty `SYMBOL_LOCALIZED` arm with `return Qnil ()` /
  `return Qt ()`, deleting or `continue`-ing the `break` that reaches the shared tail, moving the
  tail into the arm and deleting the post-loop one, appending `continue;` to the
  `SYMBOL_FORWARDED` arm (all six leave the tail's control flow wrong); swapping
  `SYMBOL_REDIRECT_MASK/_SHIFT` for the trapped-write pair, dropping the `>> SHIFT`, flipping the
  tail's `== SYMBOL_LOCALIZED`, swapping the two arm labels, and swapping
  `KBOARD_OBJFWDP`/`BUFFER_OBJFWDP`; turning the alias arms' `continue` into `break` or deleting
  it; pointing `decode_buffer (buffer)`, `XSETBUFFER! (tmp, buf)`, `BVAR! (buf, ...)` or
  `PER_BUFFER_VALUE_P (buf, ...)` at `current_buffer`; `!= 0` -> `== 0` on `BLV_LOCAL_IF_SET`;
  and the `Qt`/`Qnil` ternaries.  Two sharp ends worth naming: `buffer` in `local-variable-p` is
  satisfied by the Rust *type* `*mut buffer`, and `&&` is not a token at all, so polarity is
  free.  B's two proposed mechanisms - a positive requirement table
  (`docs/ports/required-tokens.tsv`: C's `FALLTHROUGH` implies the port has a `break`, C's
  `goto start` implies a `continue`) and a per-token count floor (`rust_count >= c_count`, which
  would catch all four `buf` sites) - are recorded as the next gate work rather than invented
  mid-segment.
- **Repo-level, pre-existing, found by reviewer B, diagnosed and fixed in the follow-up commit:
  the CI "Generated artifacts are fresh" step was already red at HEAD.**  Three separate
  defects, none of them data.c's:
  (a) `tools/cinventory.py`'s second `<aggregate>` branch built its record with the key `type`
  where its sibling branch (seven lines above) uses `c_type` - 54 records in the headers
  `gen-lifetimes.py` reads, 81 tree-wide - so `tools/gen-lifetimes.py` and `extract-abi.py`'s
  per-file doc mode (the path that generates `docs/ports/*.md`) both died with
  `KeyError: 'c_type'` on the first such header.  `tools/emit-mirror.py` was **not** affected:
  it reads `c_type` only in a fallback branch these containers never reach (reviewer A measured
  byte-identical output on all 12 affected headers).
  (b) `gen-lifetimes.py` rewrote the whole table without carrying the five decision columns, so
  the first `owner`/`freed_by` a porter fills would have been destroyed by any regeneration
  (the committed file held 0 decisions, so nothing was lost yet - the loss was latent).  It now
  keys the decisions by (the row's own non-decision columns, its occurrence among rows with
  that key) - rows are duplicated on purpose, e.g. one anonymous struct under three `#if`
  variants of `lib/ieee754.in.h`, and reviewer A's mutation showed a first-match pairing moving
  a decision to the wrong copy - and reports **and fails on** any decision whose row is gone;
  (c) `docs/baseline/{inventory,subrs,totals}.tsv` were stale relative to the current extractor
  (last written at `7ad447f`): constants 1,834 -> 3,898 (+2112/-48, about 1,546 of them implicit
  enum members and the rest explicit members a previous guard hid), struct fields
  5,441 -> 5,764 (+324/-1, of which 84 are `<aggregate>` rows; the rest is P-73's multi-line
  declarator fix and the removal of 33 pseudo-fields the old scanner invented from
  function-pointer parameter lists, e.g. `redisplay_interface.width`), and the subr doc digests
  (148 rows, one column, row set unchanged).  Both reviewers recomputed that decomposition, and
  reviewer A confirmed HEAD's own extractor already yields 5,764 - i.e. nothing here is caused
  by the key rename.  Regenerated, and the numbers quoted in `docs/port-master-plan.md` (the
  per-directory table and its total), `docs/goal.md` and `PORTING.md`'s counting table were
  updated to match.
- **A second P-78-class hole, found by reviewer B and fixed here: `cinventory._defun_heads`
  skipped only `", \t"` after the Lisp name**, so every `DEFUN ("name",\n       Fname, ...)` -
  Emacs's own style, e.g. 47 of treesit.c's 58, 25 of xwidget.c's 36, 9 of process.c's 63 - fell
  through `re.match` and never entered the body comparison: 1,708 of the tree's 1,849 DEFUN
  bodies were reachable.  `\r\n` are skipped now; `treesit.c` goes 11 -> 58, `data.c` stays
  120, the tree-wide count is 1,849 = the textual occurrence count P-78 recorded, and `--all`
  is unchanged (the newly reachable bodies belong to files with no port yet - the hole would
  have opened silently the day they are ported).
- **This segment is the first written under P-79 (safe-first).**  `aref`/`aset` keep
  `unsafe` for exactly two things: `aref`'s `SDATA (array).offset (idxval_byte)` handed to
  `STRING_CHAR`, and `aset`'s three variadic `error` calls.  Everything else goes through
  the mirror's helpers (`SREF`/`SSET`/`AREF`/`ASET`/`bool_vector_*`/`CHAR_TABLE_*`/
  `ASIZE`/`PVSIZE`).  Reviewer A's F2 caught the first version, whose multibyte branch
  dereferenced a raw `unsigned char *p` although `SREF`/`SSET` are its byte-for-byte safe
  equivalent and the same body's unibyte branch already used `SSET`; the port now keeps
  C's name `p` as the byte *index*, and the `SDATA` C's pointer expression spelled is
  exempted for `Faset` in `macro-body-notes.tsv` with that reason.  The four static helpers
  are plain safe `fn`s with narrow `unsafe` blocks around their three `marker_position` FFI
  calls and the `wrong_type_argument` in `not_number_or_marker`, rather than the
  `unsafe extern "C" fn` shape the earlier segments gave statics: they are file-local in C
  and need no raw access, so the safe signature costs nothing (older statics keep their
  shape until touched - P-79 clause 4).
- **`string_char_to_byte` joins the mirror.**  lisp.h 4276 declares it, fns.c defines it,
  and `aref`/`aset` are its first callers in the port: it is declared in `lisp.rs`'s
  `other_units` beside `assq_no_quit` (same header, same defining file) with the lisp.h line
  in the comment.  No `/// C:` line is needed - every parameter type is nameable.
- **This segment needs exactly one `macro-body-notes.tsv` row.**  The bodies are `if`/`else
  if` chains around named mirror helpers, so `aref` and the four helpers report no missing
  token at all, and `aset` needs only `sdata` - the token C spells in the pointer
  expression that P-79 replaced (see above).  No `switch`/`goto`-class exemption was
  required for any of the six bodies.
- **What the gate still cannot see in these six bodies (reviewer B's round-10 mutation
  run).**  15 of 27 mutants stay green, all of them the known token-set classes: the
  threshold constants (either `0x7f` may become `0xff`/`0x7e` because the tokenizer splits
  `0x7f` into `0`+`x7f` and there are two sites), `idxval < 0` -> `idxval <= 0`, the
  `! NILP (newelt)` polarity, swapping the `BOOL_VECTOR_P`/`CHAR_TABLE_P` arms (Faref and
  Faset), swapping `VECTORP`/`CLOSUREP`, `ASIZE` <-> `PVSIZE` where both spellings survive,
  the three `error` messages (string literals are dropped, and two of the three calls can be
  deleted without a report), the `SREF`/`SSET` cast widths, and `SREF (array, p)` -> a wrong
  index, since `p` and `c` are one character and `skip_short` drops them.  The cheapest
  closures, both already named in P-78: the per-token count floor (`rust_count >= c_count`,
  which closes 8 of the 15) and comparing string literals for `error`.  The four helpers are
  the load-bearing counterexample inside this segment: none of their required tokens has C
  count > 1, so every one of them is singly guarded.
- **The new declaration's *type* is outside the fast gates (reviewer B's L1).**  A mutant
  `string_char_to_byte (…, c_int) -> c_int` leaves `--check` byte-identical; `tools/gen-lisp-abi-probe.py`
  does emit the check (93 -> 94 entries) but `spikes/lisp-abi-probe.sh` needs the heavy-tier
  `build/emacs-ref`, so nothing in `gate-fast`/`layout-check` would catch a wrong signature.
  Recorded rather than fixed: the prototype is lisp.h 4276's, verbatim.
- **This segment's measured blind spots (reviewer B's round-11 mutants; 25 of 42 stay
  green).**  Recorded here as the concrete list, since P-38 only names the classes:
  every comparison *direction* in all nine `arithcompare` combinations (`lt = f1 > f2`,
  `lt = cmp < 0` <-> `> 0` for either bignum operand, the two tie-break blocks), both
  `isnan` inversions and the early-out/normal-path swap, the `Cmp_Bit_LT`/`_GT`/`_EQ`
  swaps in the packed return, `& Cmp_EQ != 0` -> `== 0` in `Fneq`, deleting either
  `eassume! (sgn != 0)`, `*args.offset (i - 1)` -> `*args.offset (i)` and `i = 1` -> `0`
  in the driver, `& cmpmask` -> `| cmpmask`, the driver's `return Qt ()` -> `Qnil ()`,
  the four fast paths' `*args` <-> `*args.offset (1)`, a dropped `&& FIXNUMP`, `<` -> `>`
  in `Flss`, `<=` -> `<` in `Fleq`, `.offset (1)` -> `.offset (0)` in `Fgtr`,
  `num1 = coerce_marker (num2)`, and `not_number_or_marker (num1)` -> `(num2)`.  Root
  causes: comparison operators are not tokens at all (12 of the 25), one-character names
  are dropped by `skip_short` (`i`), and any token whose C count exceeds one can lose a
  site unnoticed (`0` x15, `num2` x23, `cmp` x12, `sgn` x8, `xbignum_val` x6 in
  `arithcompare`).  Two bodies are fully load-bearing - `Feqlsign` and `Fneq` have no
  token with C count > 1 - and the gate *does* catch every body-level deletion (§3 of the
  review: a caught control for each of the eight names).  The mask caveat is recorded too:
  a mask change that keeps all three `Cmp_*` names present while re-OR-ing them is
  invisible (harmless in these five cases, but the class is real).
- **The ledger's `## Subrs (DEFUN) - 120` vs the generator's 116 (reviewer B's F3, already
  recorded in the round-9 bullet above).**  Nothing new this round; B re-measured it and
  asked for the decision to be explicit, which the round-9 paragraph does: the 120 boxes are
  deliberate (C's `DEFUN`s), the tool's 116 is the condition-aware count, and a regeneration
  would drop the four dead lines.
- **The first `usage:` docstrings in the port (reviewer A's observation).**  `make-docfile`'s
  `scan_keyword_or_put_char` rewrites a `usage:` keyword's surroundings (two newlines) and
  substitutes `fn` for the function's own name when it writes `etc/DOC`, so the stored string
  differs from the `doc: /* … */` comment the port mirrors byte for byte.  The mirror keeps
  the comment text (PORTING.md 5.1: the literal is the comment, and a ported make-docfile
  normalises it in phase G), and `tools/extract-abi.py`'s digest does not model that
  normalisation, so no gate covers it.  These six subrs are the first with a `usage:` line -
  phase G has to apply the normalisation, and until then this note is the record.
- **`arithcompare` is the file's first `MANY` subr and its first non-static export since
  round 8**, so three mirror decisions came with it: `arithcompare` is declared in
  `lisp.rs`'s `other_units` (lisp.h 4183, the declaration its definition shadows - P-39) so
  the units that call it in C (`bytecode.c`, …) can call it when they land; the six
  comparison subrs use the `DEFUN!` macro's `MANY` form (`nargs: isize, args: *mut
  Lisp_Object`, the parameter list the gate's `param_types` derives from C's
  `DEFUN_ARGS_MANY`); and the five call sites that pass a `Cmp_*` enumerator to a
  `cmp_bits_t` parameter spell the conversion C makes implicitly as `as cmp_bits_t` - the
  enumerators themselves stay `c_int`, because in C an enumerator has type `int` (ISO C
  6.7.2.2p3; C++ is the language where it gets the enum's type), which is what
  `lisp.rs`'s own comment beside them has recorded since the mirror was written.  A first
  version of this segment widened the three constants to `cmp_bits_t` to avoid those casts
  and justified it as C's typing; reviewer A's F2 corrected both the code and the comment.
- **The four fast-path subrs exempt one literal.**  C writes `args[0]`/`args[1]`; the mirror
  spells `args[0]` as `*args` (the `sort.rs` `binarysort,count_run` precedent), so the `0`
  cannot reappear and `macro-body-notes.tsv` carries one row for all four with that reason.
  Nothing else in the seven bodies needed an exemption: `arithcompare` is an `if`/`else if`
  chain, so not even `switch`/`case` came back.
- **P-79 in this segment.**  The only `unsafe` introduced is pointer work C itself does:
  the `args[i]` derefs in `arithcompare_driver` and the four fast paths, and the `xbignum_val`
  + `mpz_*` FFI calls in `arithcompare` (all narrow, single-expression blocks).
  `isnan` is the mirror's safe macro (`isnan!`), so those two tests are safe; the
  `arithcompare_driver` is an `unsafe fn`: its body dereferences `args`, and only the caller
  knows the pointer holds `nargs` valid objects, so it cannot be a sound safe `fn`
  (reviewer A's F3 - the six callers are `unsafe extern "C" fn`s already, so nothing else
  changed).  `arithcompare` keeps the `unsafe extern "C" fn` shape every C-visible
  definition in this file has.
- **The cons converters and the number<->string pair (this segment's decisions).**  The two
  `UNINIT` locals are `= {0,}` again (the round-7 reading of conf_post.h 369), so
  `cons_to_unsigned`/`cons_to_signed` share one notes row for `uninit` with that reason.
  `Fnumber_to_string`'s buffer size is C's `max (FLOAT_TO_STRING_BUFSIZE,
  INT_BUFSIZE_BOUND (EMACS_INT))`: `INT_BUFSIZE_BOUND` is a gnulib `intprops.h` macro with no
  generic mirror (P-26/P-62, it needs C's `_GL_SIGNED_TYPE_OR_EXPR`), so the port carries the
  one instantiation as a const - 21 for the signed 64-bit `EMACS_INT` - and spells C's `max`
  as an if/else, which is why `max` is in that body's notes row.  The buffer itself stays
  uninitialised as C leaves it (`MaybeUninit`, the lisp.rs `candidate_string` precedent)
  rather than zeroing 350 bytes nothing reads.
- **`bignum_to_string`, `float_to_string` and `string_to_number` join the mirror.**  They are
  declared in lisp.h 4090/4710/4747 and defined in bignum.c/print.c/lread.c, so their
  declarations live in `lisp.rs`'s `other_units`; the first attempt also re-declared
  `cons_to_unsigned`/`cons_to_signed`, which were already there with lisp.h 4187's prototypes
  (E0428 - removed).
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
  `#[cfg(feature = "have_modules")]` was tried in this round and reverted: with `src/data.c` now
  in `cond-eval-verified.txt` the gate reads the frozen text, so the arm C never compiles is
  reported as `the reference file has no such DEFUN` (an invented item, D3).  The subr is
  therefore absent from the port, and PORTING.md P-76 records the remaining half of the
  story: the tool's `defsubr` extraction does not consult the condition table yet, so it
  still expects `defsubr (&Suser_ptrp)`; `syms_of_data` must record that as a ledger
  deviation rather than write the call.
- **`MODULE_FUNCTIONP` / `XMODULE_FUNCTION` are unconditional (review B's F2).**  The port
  had gated the pair in `lisp.rs` on a `have_modules` feature that no build ever passes, leaving
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
- **Twelve lexical notes in `docs/ports/macro-body-notes.tsv` name a data.c body.**  C's
  `switch`/`case`/`default`, `start:`/`goto start` and `FOR_EACH_LIVE_BUFFER` have no Rust
  spelling, so those function names carry an explicit exemption list (the `doprnt`
  precedent on the same file); every other token of those bodies is still compared.  The
  packed bitfields (`redirect`, `trapped_write`, `local_if_set`) are the one class where an
  exemption can hide a whole statement, so `local_if_set` left this file for
  `token-aliases.tsv` (the next bullet).

- **`union Lisp_Val_Fwd` (data.c 2142-2146) is this file's first union.**  C defines it in
  data.c rather than in a header, so P-72 keeps it in `data.rs` and no header mirror can
  name it - and `xtask layout-check` compiles only `src/include/*.rs`, so a
  `c-layout` key for it is structurally impossible.  The port asserts the same two facts
  where the type lives (`const _: () = assert! (size_of == 8 && align_of == 8)`, the
  `bignum.rs`/`doprnt.rs` precedent) and `--check` compares both member names and types.
- **`local_if_set` is pinned by a token family, not by a note (review B's D1).**
  `blv->local_if_set = 0` (data.c 2170) contributes only the field name and a `0` literal,
  and the port spells the field `BLV_LOCAL_IF_SET`; exempting both tokens left the whole
  statement with no mechanical check at all, so deleting it, inverting it or retargeting it
  to `BLV_FOUND` passed every gate.  `docs/ports/token-aliases.tsv` now carries the family
  `local_if_set`/`blv_local_if_set`, whose *occurrence* check requires the port to spell it
  at least as often as the C body does - the mechanism that closed review D's F2 for
  `memcpy` - so the deletion is reported.  The exemption is gone from `set_internal`'s row
  and `make_blv`'s row is deleted.  `redirect`, `trapped_write` and `found` stay exempted
  per body: a *deleted* read of those is still invisible, and generalising the family to
  them is recorded in PORTING.md P-78's residuals as the next gate improvement.
- **C's bitfield store reads the whole storage unit (review A's D3).**  `blv->local_if_set
  = 0` in C and `(*blv).bits = (*blv).bits & !BLV_LOCAL_IF_SET` in the port both
  read-modify-write a word `xmalloc` left uninitialized, and `set_blv_found` reads that
  word too.  The port keeps C's statement order (the clear comes *before* `set_blv_found`,
  which `blv_found`'s `eassert` depends on) instead of adding an initialization C does not
  have; the observable state is identical and those two bits are the only ones any code
  reads.  Behavioural pin: probe P1 (a reused dirty `blv` must still read `local_if_set` =
  nil), to run in the segment that brings `make-variable-buffer-local` (2210) and
  `local-variable-if-set-p` (2465).
- **lisp.h's remaining data.c prototypes are not in the mirror yet (review A's D5).**
  `set_internal` (lisp.h 4206), `notify_variable_watchers` (4165) and `default_value` (621)
  are defined here but not declared in `lisp.rs`'s `other_units`.  This unit does not need
  them - a definition is visible in its own module - and no other unit is ported yet, so the
  mirror follows the pattern its own comments describe (the block is annotated with the
  first unit that needs each declaration).  The first unit that calls one of them must add
  it, with the `/// C:` prototype when a parameter cannot be named (P-72), as
  `set_default_internal` already does (lisp.rs 3672).
- **Review B's D7 (`store_symval_forwarding`'s row allows a `case` its C body lacks) does
  not hold.**  Measured with the gate's own tokenizer: the row's allowance is exactly the C
  body's token set (`allowed - C = {}`), and `case` is in it - the body is a `switch` over
  `XFWDTYPE` with six `case` labels (data.c 1472-1540).  No change.
- **`--check` used to report every unported subr twice (review B's D6).**  The
  "missing subr" loop and the "`DEFUN!` has no definition" loop are the same fact; the
  second now skips names the first already reported, so this file's remaining count is a
  one-to-one list of what is left (48 subrs, not 96 lines).
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
  under `#[cfg(feature = "have_native_comp")]` (the `floatfns.rs` `no_rint` precedent), and
  *that text does compile*: a run of `xtask dead-arms` with this ledger presented as
  complete compiled `src_data.o` under the flipped configuration (the `have_native_comp` entry
  in `FLIPS` flips the constant and the feature together; the `const _` assertion in
  `lisp.rs` is what makes the pair meaningful).  Until the ledger is complete,
  `finished_units()` skips this unit, so that text is compiled at batch boundaries rather
  than on every gate run - recorded here as the residual it is.
  (b) The three `#ifdef HAVE_NATIVE_COMP` DEFUNs are not ported (bullet above).
  (c) `interactive-form` and `command-modes` drop C's `#ifdef HAVE_MODULES` arms: the
  frozen text has no such branch, so the `if`/`else if` chain is the same shape without
  it.  Unlike (b) this is a *choice* (the tokens would not be compared either way,
  because `c_body_text` drops dead lines) - it is recorded because the port could have
  kept it under `#[cfg(feature = "have_modules")]`, and chose not to while that feature has no
  mirror for the `module_function_*` helpers it would need.
- **A real defect the reviewers caught in this segment (review B's P1).**
  `Findirect_variable` must rebind the parameter (`XSETSYMBOL (object, sym)`) and return
  it; the port first put the mutable binding *inside* the `if`, so the write landed on a
  shadow and the returned value stayed the parameter - the function's only purpose,
  following an alias chain, silently did nothing.  The token gate cannot see this class
  ("every token survives"): it was found by the human reading, and the fix hoists the
  binding to the function's scope (the `floatfns.rs`/`buffer.rs` precedent).  This is
  recorded in P-78's residuals as the third blind-spot class.
- **data.c 1678-1960 has no preprocessor conditions.**  Both a line scan and the frozen
  evaluator agree (the file has 74 dead lines in total - `user-ptrp`, the `HAVE_MODULES`
  arms, the `HAVE_NATIVE_COMP` block and the three dead `defsubr` calls - and none of them
  is in this range), so this segment has no dead-text decision to record; P-75's discard
  rule has nothing to do here.
- **A gate weakening the second reviewer found while checking this segment (its D1).**
  `extract-abi.py`'s `local_types` regex used `\s*` around a `}` + `NAME;` pattern, and
  `\s` crosses newlines: a `}` at the end of one line followed by `return;` or `break;`
  on the next registered those *words* as "a type this file declares", exempting `break`
  (and any such word) from every body comparison in the file.  With `[ \t]*` the fix
  surfaced exactly one hidden token, `set_internal`'s `break` (now in
  `macro-body-notes.tsv` with its reason), and `--all` stayed 17 done / 10 WIP / 0 failing.
