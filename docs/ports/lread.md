# emacs/src/lread.c (5901 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 17
- [x] get-load-suffixes  (Fget_load_suffixes, 0/0, doc f7a01a9f2154, line 961)
- [x] load  (Fload, 1/5, doc 4db613402bca, line 1068)
- [x] locate-file-internal  (Flocate_file_internal, 2/4, doc 6e9a69974960, line 1590)
- [x] eval-buffer  (Feval_buffer, 0/5, doc 73223f55a206, line 2358)
- [x] eval-region  (Feval_region, 2/4, doc 74908b668a52, line 2420)
- [x] read  (Fread, 0/1, doc d6994cc637f6, line 2457)
- [x] read-positioning-symbols  (Fread_positioning_symbols, 0/1, doc 8a887a2064d5, line 2484)
- [x] read-from-string  (Fread_from_string, 1/3, doc 94d5662da4ef, line 2511)
- [x] lread--substitute-object-in-subtree  (Flread__substitute_object_in_subtree, 3/3, doc fff44325031e, line 4380)
- [x] intern  (Fintern, 1/2, doc 1a053a055d51, line 4771)
- [x] intern-soft  (Fintern_soft, 1/2, doc 665767d60851, line 4790)
- [x] unintern  (Funintern, 2/2, doc 3da8123ea5db, line 4828)
- [x] obarray-make  (Fobarray_make, 0/1, doc 4d01219033e1, line 5052)
- [x] obarrayp  (Fobarrayp, 1/1, doc a0f38e9fd366, line 5072)
- [x] obarray-clear  (Fobarray_clear, 1/1, doc 0495736f0713, line 5079)
- [x] mapatoms  (Fmapatoms, 1/2, doc d825e8c16401, line 5119)
- [x] internal--obarray-buckets  (Finternal__obarray_buckets, 1/1, doc 6e6e7942f3a4, line 5131)

## Functions defined — 105
- [x] file_get_char  (line 132, static)
- [x] init_source  (line 287, static)
- [x] source_buffer_get  (line 332, static)
- [x] source_buffer_unget  (line 362, static)
- [x] source_marker_get  (line 372, static)
- [x] source_marker_unget  (line 401, static)
- [x] source_string_get  (line 412, static)
- [x] source_string_unget  (line 433, static)
- [x] source_file_get  (line 446, static)
- [x] source_file_unget  (line 480, static)
- [x] source_function_get  (line 486, static)
- [x] source_function_unget  (line 493, static)
- [x] readchar  (line 500, static)
- [x] unreadchar  (line 508, static)
- [x] from_file_p  (line 519, static)
- [x] from_buffer_p  (line 525, static)
- [x] skip_dyn_bytes  (line 531, static)
- [x] skip_dyn_eof  (line 555, static)
- [x] readbyte_from_file  (line 570, static)
- [x] unreadbyte_from_file  (line 626, static)
- [x] invalid_syntax_lisp  (line 636, static)
- [x] invalid_syntax  (line 657, static)
- [x] read_emacs_mule_char  (line 668, static)
- [x] lisp_file_lexical_cookie  (line 759, static)
- [x] safe_to_load_version  (line 891, static)
- [x] record_load_unwind  (line 931, static)
- [x] load_error_handler  (line 939, static)
- [x] load_warn_unescaped_character_literals  (line 945, static)
- [x] suffix_p  (line 994)
- [x] close_infile_unwind  (line 1004, static)
- [x] compute_found_effective  (line 1015, static)
- [x] loadhist_initialize  (line 1031, static)
- [x] close_file_unwind_android_fd  (line 1043, static)
- [x] get_lexical_binding  (line 1054, static)
- [x] save_match_data_load  (line 1571)
- [x] complete_filename_p  (line 1582, static)
- [x] maybe_swap_for_eln1  (line 1612, static)
- [x] maybe_swap_for_eln  (line 1648, static)
- [x] openp  (line 1756)
- [x] build_load_history  (line 2054, static)
- [x] end_of_file_error  (line 2120, static)
- [x] readevalloop_eager_expand_eval  (line 2133, static)
- [x] readevalloop  (line 2159, static)
- [x] read_internal_start  (line 2533, static)
- [x] character_name_to_code  (line 2576, static)
- [x] read_char_escape  (line 2606, static)
- [x] digit_to_number  (line 2868, static)
- [x] invalid_radix_integer  (line 2885, static)
- [x] readbuf_grow  (line 2905, static)
- [x] add_char_to_buffer  (line 2925, static)
- [x] read_integer  (line 2942, static)
- [x] read_char_literal  (line 2994, static)
- [x] read_string_literal  (line 3039, static)
- [x] hash_table_from_plist  (line 3153, static)
- [x] record_from_list  (line 3198, static)
- [x] vector_from_rev_list  (line 3215, static)
- [x] bytecode_from_rev_list  (line 3233, static)
- [x] char_table_from_rev_list  (line 3302, static)
- [x] sub_char_table_from_rev_list  (line 3313, static)
- [x] string_props_from_rev_list  (line 3345, static)
- [x] read_bool_vector  (line 3370, static)
- [x] skip_lazy_string  (line 3412, static)
- [x] get_lazy_string  (line 3493, static)
- [x] symbol_char_span  (line 3543, static)
- [x] skip_space_and_comments  (line 3553, static)
- [x] mark_lread  (line 3632)
- [x] read_stack_top  (line 3667, static)
- [x] read_stack_pop  (line 3674, static)
- [x] read_stack_empty_p  (line 3681, static)
- [x] grow_read_stack  (line 3687, static)
- [x] read_stack_push  (line 3696, static)
- [x] read_stack_reset  (line 3704, static)
- [x] invalid_syntax_with_buffer  (line 3711, static)
- [x] read_and_buffer  (line 3718, static)
- [x] read0  (line 3730, static)
- [x] substitute_object_recurse  (line 4399, static)
- [x] substitute_in_interval  (line 4478, static)
- [x] string_to_number  (line 4500)
- [x] check_obarray_slow  (line 4642)
- [x] intern_sym  (line 4672, static)
- [x] intern_driver  (line 4703)
- [x] intern_1  (line 4713)
- [x] intern_c_string_1  (line 4726)
- [x] intern_c_multibyte  (line 4744)
- [x] define_symbol  (line 4755, static)
- [x] obarray_index  (line 4822, static)
- [x] oblookup  (line 4895)
- [x] oblookup_considering_shorthand  (line 4929)
- [x] allocate_obarray  (line 4984, static)
- [x] make_obarray  (line 4990, static)
- [x] grow_obarray  (line 5010, static)
- [x] map_obarray  (line 5105)
- [x] mapatoms_1  (line 5114, static)
- [x] init_obarray_once  (line 5159)
- [x] defsubr  (line 5188)
- [x] defvar_int  (line 5206)
- [x] defvar_bool  (line 5217)
- [x] defvar_lisp_nopro  (line 5233)
- [x] defvar_lisp  (line 5243)
- [x] defvar_kboard  (line 5254)
- [x] load_path_check  (line 5266, static)
- [x] load_path_default  (line 5317, static)
- [x] init_lread  (line 5421)
- [x] dir_warning  (line 5500)
- [x] syms_of_lread  (line 5529)

## Macros defined — 20
- [x] ADDPARAM  (line 3159, body 8a5807d189a5)
- [x] DEFINE_SYMBOLS  (line 22, body da39a3ee5e6b)
- [x] INFINITY  (line 166, body a11f7aa56df4)
- [x] UPDATE_BEG_END_STATE  (line 797, body 3fe7dbe4cd6f)
- [x] USE_ANDROID_ASSETS  (line 156, body da39a3ee5e6b)
- [x] file_get_char  (line 76, body 9133b4724467)
- [x] file_offset  (line 121, body b4a2df006682)
- [x] file_seek  (line 123, body e4d15015202f)
- [x] file_stream  (line 120, body edb0e23a72a9)
- [x] file_stream_close  (line 125, body 1faea5f0e770)
- [x] file_stream_invalid  (line 126, body cf14fcf50765)
- [x] file_stream_valid_p  (line 124, body 55da589a29b1)
- [x] file_tell  (line 122, body 582d3081959a)
- [x] lread_close  (line 107, body 1faea5f0e770)
- [x] lread_fd  (line 104, body edb0e23a72a9)
- [x] lread_fd_cmp  (line 105, body c567d2df299f)
- [x] lread_fd_p  (line 106, body d0644a538888)
- [x] lread_fstat  (line 108, body 74f079739a67)
- [x] lread_lseek  (line 110, body e4d15015202f)
- [x] lread_read_quit  (line 109, body 079430e799f2)

## Types and their fields — 4
- [x] read_stack: struct read_stack_entry* stack, ptrdiff_t size, ptrdiff_t sp
- [x] read_stack_entry: enum read_entry_type type, Lisp_Object head, Lisp_Object tail, <aggregate> list, Lisp_Object elems, bool old_locate_syms, <aggregate> vector, Lisp_Object symbol, <aggregate> special, Lisp_Object number, Lisp_Object placeholder, <aggregate> numbered, <aggregate> u
- [x] saved_string: char* string, ptrdiff_t size, ptrdiff_t length, file_offset position
- [x] subst: Lisp_Object object, Lisp_Object placeholder, Lisp_Object completed, Lisp_Object seen

## File-scope objects — 12
- [x] static Lisp_Object read_objects_map  (line 182, static)
- [x] static Lisp_Object read_objects_completed  (line 199, static)
- [x] static ptrdiff_t read_from_string_index  (line 217, static)
- [x] static ptrdiff_t read_from_string_index_byte  (line 218, static)
- [x] static ptrdiff_t read_from_string_limit  (line 219, static)
- [x] static EMACS_INT readchar_offset  (line 222, static)
- [x] static struct saved_string saved_strings  (line 232, static)
- [x] static Lisp_Object Vloads_in_progress  (line 237, static)
- [x] static int unread_char  (line 255, static)
- [x] static struct read_stack rdstack  (line 3629, static)
- [x] static Lisp_Object not_a_number  (line 4487, static)
- [x] static Lisp_Object initial_obarray  (line 4636, static)

# ----------------------------------------------------------------------
## Decisions

The gates:

    build-tu src/lread.c            -> built src/lread.rs -> ../build/femacs/lread.o
    extract-abi.py --check          -> OK emacs/src/lread.c -> femacs/src/lread.rs
                                       (15 C macros have several definitions, body not compared;
                                       3 C macros have a recorded Rust counterpart)
    check-symbols.py --dir src      -> 29 units compared, 0 mismatched, 0 still pending
    check-globals.py                -> 260 units, 0 DEFVAR_* variables without a definition in their own unit
    check-undef.py                  -> 172 pending, 0 new
    check-linkage.py                -> 9 unit object(s) compared, 0 with 0 missing symbol(s)
    ledger                          -> 158 rows, 0 unchecked
    (extra) rustc --cfg enable_checking on the materialized unit -> exit 0

### Carried arms: the frozen configuration's dead text is written, not recorded (status 3.49 / P-51)

`src/lread.c` is *not* in `docs/baseline/config/cond-eval-verified.txt`, so `tools/extract-abi.py`'s
blind evaluator keeps *both* arms of every `#if` live and the per-function token check demands the
tokens of C's text on both sides.  Per the integrator's 2026-10-03 decision the port therefore
*carries* the arms the frozen build does not compile, each under a `#[cfg (feature = "…")]` naming
the C condition's non-frozen side (the `timefns.rs`/`callproc.rs`/`print.rs` shape) - the ledger's
line tolerance is on the **total** basis for the same reason.  The arms, in C order:

- C 60-157: the Android-asset family - `lread_fd`/`lread_fd_cmp`/`lread_fd_p`/`lread_close`/
  `lread_fstat`/`lread_read_quit`/`lread_lseek` (C 104-110) and `file_stream`/`file_offset`/
  `file_tell`/`file_seek`/`file_stream_valid_p`/`file_stream_close`/`file_stream_invalid`
  (C 120-126), `invalid_file_stream` (C 112-116), `file_get_char` the function (C 128-152) and
  `USE_ANDROID_ASSETS` (C 156) - features `have_android`, `use_android_assets`.  The live arm
  (C 63-84) is plain text; `#include "android.h"` has no mirror, so `struct android_fd_or_asset`
  (16/8, clang-measured) and the five `android_*` entries are use-site declarations.
- C 159-168: `#if IEEE_FLOATING_POINT` is **live** (the `union ieee754_double` NaN machinery, see
  below), so the `#else`'s `#define INFINITY HUGE_VAL` is the carried arm
  (`no_ieee_floating_point`; both `#ifndef INFINITY` arms are dead because `<math.h>` defines it).
- C 975-985 (`Fget_load_suffixes`), C 1180-1188 and C 1282-1297 (`Fload`'s suffix tests), C 1498-1516
  (`Fload`'s `Fmodule_load`/`Fnative_elisp_load` bodies), C 5588-5601 (`syms_of_lread`'s suffix
  lists), C 5196-5199 (`defsubr`'s `Vcomp_subr_list` update): `HAVE_MODULES` (`have_modules`),
  `MODULES_SECONDARY_SUFFIX` (`modules_secondary_suffix`) and `HAVE_NATIVE_COMP`
  (`have_native_comp`).
- C 1610-1718: `maybe_swap_for_eln1` and `maybe_swap_for_eln` are whole functions inside
  `#ifdef HAVE_NATIVE_COMP`; both are carried under the same feature.
- C 1248-1263 (`DOS_NT`), C 1430-1445 (`WINDOWSNT`), C 5606-5614 (`MSDOS`): the platform arms
  (features `dos_nt`, `windowsnt`, `msdos`); the `#include "msdos.h"`/`nsterm.h` pair (C 49-55)
  contributes no name.
- C 4485-4488, C 4572-4581, C 5844-5850: the `IEEE_FLOATING_POINT` else-arms (`not_a_number`, its
  `syms_of_lread` initialisation) under `no_ieee_floating_point`.

**The one source-header condition (step 5 of the per-file loop).**  `IEEE_FLOATING_POINT` is
computed by `lisp.h` 3201-3202 from `<float.h>`, so the frozen `config.h` dump cannot see it and the
evaluator answered False where clang answers True (the four disagreements the ledger's generator
reported).  A row `lisp.h	IEEE_FLOATING_POINT` was added to
`docs/baseline/config/source-header-macros.tsv`, `python3 tools/gen-cond-macros.py` regenerated the
7281-macro table (one line changed), and `python3 tools/check-cond-eval.py --verified
docs/baseline/config/cond-eval-verified.txt` ends `0 disagreements` (see below).  The file still
stays out of the verified list on purpose: carrying both arms is the *stricter* comparison (status
3.51's callproc.c note).  **Side effect the gates cannot see**: the row also makes
`check-cond-eval.py src/print.c` report `0 disagreements` where `docs/ports/print.md` and print.rs's
header record `2 disagreements [NOT VERIFIED]`, so those two recordings are now stale; they are the
print lane's file and were deliberately not touched.

### Layout (`#[repr(C)]` + clang-measured `const _` assertions)

Every type this unit declares has its `sizeof`/`alignof`/`offsetof` pinned to clang's measurement
against the frozen target (the probe is `/tmp/lread_types.c`, whose struct texts are copied from
`src/lread.c`): `source` 32/8 (`object`@16, `multibyte`@24, `emacs_mule_encoding`@25), `infile` 16/8
(`buf`@9), `saved_string` 32/8, `subst` 32/8, `read_entry_type` 4/4, `read_stack_entry` 24/8
(`type`@0, the `u` union@8 with `list`/`vector`/`special`/`numbered` and their members at 8/16),
`read_stack` 24/8, `readbuf_t` 40/8 (`count`@32), `android_fd_or_asset` 16/8 (the Android arm), and
`ieee754_double` 8/8 with its `ieee_nan` member 8/4.  C's anonymous union/struct members inside
`struct read_stack_entry` become the named `read_stack_entry_list`/`_vector`/`_special`/`_numbered`
records plus the `read_stack_entry_u` union (PORTING.md 1.3), so the ledger's field list for that
type is reproduced exactly and `offsetof` can key every member.

### Use-site declarations (no mirror was asked to grow)

Each carries C's own prototype in a `/// C:` line (PORTING.md 0.1-2/P-72).  `<stdio.h>`:
`getc`, `fseek`, `ftello`, `clearerr`, `fprintf`, `stderr` (Darwin's `__stderrp`); `<string.h>`:
the two the `string.rs` mirror already has were dropped in favour of it; `<fcntl.h>`: `AT_FDCWD`
(the `fcntl.in.rs` constant), `<sys/stat.h>`: `emacs_fstatat`, `sys_faccessat`, `emacs_open`,
`emacs_close`, `sys_fstat`, `emacs_fdopen`, `emacs_fopen`, `emacs_read_quit`; `lisp.h`'s
`build_string`/`intern`/`make_specified_string`/`pin_string`/`get_doc_string`/`eval_sub`/
`mark_object`/`init_symbol`/`free_cons`/`plist_get`/`count_lines`/`current_column`/
`find_symbol_value`/`signal_error`/`report_file_error`/`internal_condition_case_1`/
`message_with_string`/`message_dolog`/`emacs_strerror`/`file_accessible_directory_p`/
`decode_env_path`/`nconc2`/`esprintf`/`save_match_save`/`save_restriction_save`/
`save_restriction_restore`/`labeled_restrictions_remove_in_current_buffer`/
`record_unwind_protect_excursion`/`record_unwind_protect_int`/`_ptr`/`_intmax`/
`set_unwind_protect_ptr`/`clear_unwind_protect`/`record_unwind_save_match_data`/`Fnative_elisp_load`
(the subrs come from globals.rs's generated `subrs` module)/`hashtest_eq` (lisp.rs carries the
eql/equal pair only)/`hash_char_array`/`hash_table_alloc_bytes`/`hash_table_free_bytes`/
`make_neg_biguint`/`make_bignum_str`/`atof`.   `<intprops.h>` has no mirror (P-26): `TYPE_MINIMUM`/
`TYPE_MAXIMUM` are `macro_rules!` (the search.rs shape), and lisp.h's `countof` is a macro that
folds with `size_of_val` (it also serves C 3488's `static_assert`).  `<epaths.h>` is generated: the
three strings `load_path_default`/`init_lread` read are the frozen
`build/emacs-ref/src/epaths.h` values.  `DYNAMIC_LIB_SUFFIX`/`DYNAMIC_LIB_SECONDARY_SUFFIX` are the
table's `.dylib`/`.so`.

### `DEFVAR_*` storages (status 3.29)

C's storage for every `DEFVAR_*` of this file is `globals.f_<name>` (lisp.h 3495-3530) inside the
one `struct emacs_globals globals` that alloc.c defines, so `build/emacs-ref/src/lread.o` exports
`_globals` and *no* `_V*` symbol; the port gives each variable its own `#[unsafe(no_mangle)] pub
static mut` at the crate root (28 `Lisp_Object`s and 6 `bool`s) because `include/globals.rs`
mirrors them as external declarations and other units read them.  `check-symbols.py --dir src`
counts this class as the port's documented deviation (173 carriers) instead of failing on it, and
`check-globals.py` reports `0 DEFVAR_* variables without a definition in their own unit`.

### Spelling deviations, one by one

1. **`switch` and `goto`.**  C's dispatch machinery is Rust's `match` and labelled loops: `read0`'s
   `read_obj:`/`read_symbol:` (a `'read_obj` loop plus a `symbol_path` flag; the `case` values are
   `b'('`… so a multibyte character falls to C's `default` arm exactly as in C), `read_char_escape`'s
   `again:`/`mod_key:`/`unicode_hex:` (an outer `'again` loop and the `MOD_KEY`/`UNICODE_HEX`
   sentinels - a `goto` into a switch's middle, the ccl.rs/print.rs shape), `read_string_literal`'s
   inner escape switch, `mark_lread`, `substitute_object_recurse` (over `XTYPE`), `Funintern`'s
   `removed:` (a `break` out of the bucket walk), `readevalloop`'s `read_next:` (a `'read_next` loop
   inside the `'readeval` loop, whose `break 'readeval` is C's `break` on EOF).  Every one of those
   is a `docs/ports/macro-body-notes.tsv` row.
2. **Statement macros that write the caller's locals.**  `UPDATE_BEG_END_STATE (ch)` (C 797-807) is
   the helper function `UPDATE_BEG_END_STATE` with the two locals by address (the marker.rs
   `CONSIDER` precedent); C's function-local `enum { NOMINAL, AFTER_FIRST_DASH, AFTER_ASTERIX }` is
   three file-scope constants plus an in-function `const _: (c_int, c_int, c_int) = (…)` that keeps
   C's declaration where C has it.  `ADDPARAM (name)` (C 3159-3168) is a file-scope `macro_rules!`
   with one rule per name C passes - it cannot stand inside `hash_table_from_plist` because a
   `macro_rules!` item's closing brace ends this gate's body extraction, and the call sites keep
   C's argument names (`test`, `weakness`, `purecopy`).
3. **`bool`/int spellings.**  C's `= 0`/`= 1` on a `bool` and its `true`/`false` are the port's
   `false`/`true` (rows for `build_load_history`, `load_path_default`, `init_lread`, `dir_warning`,
   `Fload`); the `bool` parameters of `readeval`-side calls keep C's `0` where it is a *pointer*
   (`0 as *mut infile`, `0 as *const c_char`).
4. **`UNINIT` and C's uninitialised locals (P-14).**  `file_stream stream UNINIT`,
   `specpdl_ref fd_index UNINIT`, `lread_fd fd`, `readbuf_t rb`'s `cur`/`count`,
   `read_integer`/`read_string_literal`'s `rb`, `read0`'s `uninterned_symbol`/`skip_shorthand`,
   and `Fload`'s `found`/`efound`/`hist_file_name`/`version`/`handler` are zero-initialised; `UNINIT`
   is `= {0,}` in the frozen `conf_post.h`, so the macro name cannot reappear (the `Fload` row).
5. **`#@` doc-string machinery.**  `saved_strings` is a two-element array of the measured record and
   its `countof` uses are the port's `countof!`; `skip_lazy_string`'s `static_assert` becomes a
   `static_assert!` over the array's *type* (a `static mut`'s address is not a const expression in
   Rust) plus the `eassert!` that carries the `countof` token.
6. **`oblookup`/`intern`.**  C's `q`/`qc` symbols are the port's `Q*`/`QC*` accessor functions, so
   every use carries `Qname ()`; `SET_SYMBOL_FWD` takes `*mut c_void` in the mirror while C's
   `defvar_*` take `struct Lisp_Fwd const *`, so the four call sites cast once.
7. **The symbol bitfields (P-80).**  `s->u.s.interned`/`trapped_write`/`redirect`/`declared_special`
   are the mirror's packed `u.s.bits`; the port's `set_symbol_bits! (member, sym, value)` keeps
   C's member name as the macro's first argument, the `hash_table_bits!`/`it_bits!` convention.
8. **`defsym_name`.**  C's `static char const *const defsym_name[]` is generated into the
   reference's `globals.h` (5799; 1561 entries in the frozen build) and is *not* in the port's
   generated `globals.rs`, so the 2118 names of `docs/baseline/globals/symbols.tsv` (the port's own
   order, P-29's superset) are written here, four per line.  This is the single largest block of
   text with no C counterpart in this file and the main reason the line tolerance is 0.16.
9. **`lispsym`.**  Only `src/lread.c` defines `DEFINE_SYMBOLS`, so only its object defines
   `_lispsym` (measured: `nm -g build/emacs-ref/src/lread.o` shows a common `_lispsym`).  The port
   defines the 2118-entry array here too, in a nested `pub mod lispsym_def` with a glob import,
   because `globals.rs` declares `lispsym` as a bare crate-level `extern` and a second crate-level
   definition is `E0428` (PORTING.md 0.1-10's module form).
10. **`getc` is not `__srget`.**  C's `file_get_char` is `#define file_get_char getc`, and on the
    frozen target `<stdio.h>` expands `getc (p)` to `(--(p)->_r < 0 ? __srget (p) : (int)
    *(p)->_p++)`, so the reference object's only `getc` reference is `___srget`.  Calling `__srget`
    directly is *not* `getc`: measured with a three-byte probe
    (`getc`, then `__srget`, then `getc` on a buffered stream), it returns `A` and then garbage and
    leaves the stream unusable, because `__srget` refills unconditionally.  The port therefore calls
    the libc entry point the macro stands for (the `emacs_fclose`/`_putc_unlocked` class) and its
    object references `_getc` where C references `___srget` - the one call-relationship difference
    in D7 that is not a mirror-shape artefact.
11. **`IEEE_FLOATING_POINT`.**  The live arm is C 4572-4576's `union ieee754_double u = { .ieee_nan
    = { .exponent = 0x7ff, .quiet_nan = 1, .mantissa0 = n >> 31 >> 1, .mantissa1 = n } }`.  gnulib's
    `ieee754.in.h` template has no mirror (P-26/P-62), so the union is declared here with clang's
    measured packing and the four member masks (the print.rs shape), and the initialiser is written
    as the two `u32` units that packing produces.
12. **Miscellaneous.**  `XSETCDR`/`XSETCAR` are *functions* in the mirror (no bang) while
    `XSETPVECTYPE!`/`XSETSUBR!`/`XSETFASTINT!` are macros; `SSET`'s index is a `usize` and its
    character a `u8`; `pvec_type::PVEC_CLOSURE`/`PVEC_CHAR_TABLE` are enum members of the mirror's
    `pvec_type`; `dir_warning`'s parameter is C's `use`, spelled `r#use` (a Rust keyword);
    `hash_table_from_plist`'s `params[4 * 2]` stays `4 * 2`; `read_stack_reset` takes C's `intmax_t`
    (`i64`) because it is `record_unwind_protect_intmax`'s callback; `make_fixnum`/`XFIXNUM`/
    `make_fixed_natnum` take `EMACS_INT`, so the arithmetic keeps C's own types instead of narrowing;
    `SREF` returns a `u8` (C's `unsigned char`), so `openp`'s
    two prefix tests spell the bytes `b'/' as u8`/`b':' as u8`, and `complete_filename_p` writes
    `*s.offset (0)` so C's `s[0]` token survives.

### Line counts

C 5,901 lines by `wc -l` (the ledger's own count) vs the port's **6,820 / 6,821** by the same
counts: **15.6 %**, over the 5 % default.  The row
`lread	0.16	C 5901 vs the port's 6820 (15.6%) …	total` was appended to
`docs/ports/line-drift-tolerance.tsv` (the `total` basis, because the carried arms are text by
design; the reasons are in the row).  The excess is the generated `defsym_name` table (~530 lines),
the fixed costs (the `// femacs:` note, `#![allow]`, the include block, ~60 use-site declarations
with their `// C:` lines), the 60 `#[cfg (feature = …)]` attributes of the carried arms, the eight
file-local layout assertion blocks, and C's switch/goto machinery spelled as matches and labelled
loops; the 17 `DEFUN!` docstrings are the one place the port is *shorter* than C's comment blocks.

### Call-relationship differences (D7)

Measured with `nm -u` on both objects (`clang -c` of `emacs/src/lread.c` against the frozen
configuration vs `build/femacs/lread.o`): **176 undefined references in C, 205 in the port, 169
shared**.  Every callee C's *source text* names is in the port's set.  The differences:

* **C-only (3):** `_FUNCTIONP` (C's out-of-line copy of lisp.h's inline), `_current_thread` (C
  reaches the current buffer through `current_thread`; the port uses `_current_buffer`), and
  `_globals` (the §3.29 storage model, seen from C's side).
* **Port-only, mirror shape (§3.29/globals.rs):** `_Vcomp_eln_to_el_h`, `_Vinstallation_directory`,
  `_Vlocale_coding_system`, `_Vpurify_flag`, `_Vquit_flag`, `_Vstandard_output` - C reaches each
  through `globals.f_<name>`, the port declares one symbol per variable.
* **Port-only, codegen/other units' inlines:** `_getc` (see deviation 10), `_clearerr`, `_ferror`,
  `_memset` (Rust's zeroing), `_setjmp`/`_bzero`/`_cons_to_signed`/`_cons_to_unsigned`/`_specpdl`/
  `_specpdl_ptr`/`_sa_stack_alloc`/`_symbols_with_pos_enabled` (lisp.rs's inline families, present in
  other units' objects too), `_current_buffer`, and Rust's core panic symbols.
* **Port-only, kept where C's optimizer removed them:** `_Fgethash` and `_Vcomp_eln_to_el_h` come
  from `compute_found_effective`, which the frozen build's clang drops (unused `static`, because
  `is_native_elisp` folds to `false` when `HAVE_NATIVE_COMP` is undefined).  The port keeps the
  function and therefore references both names; they are pending entries in
  `docs/baseline/undef-pending.txt` (a not-yet-ported comp.c owns `Vcomp_eln_to_el_h`), which the
  integrator rewrites.

### Residual (reported, not fixed)

1. **`oblookup_considering_shorthand`'s linkage.**  C 245 declares it `static` and C 4929 defines it
   without the keyword, so C 6.2.2p4 gives the *definition* internal linkage and the reference object
   exports no symbol for it (`nm -g build/emacs-ref/src/lread.o`).  A blind file cannot see the prior
   declaration - `tools/cinventory.py` disables its `static_prior` rule when the condition table is
   dropped (status 3.67) - so the text gate would demand a `#[unsafe(no_mangle)]` C does not have.
   The port keeps it private (the object decides) and the name is recorded in
   `docs/ports/excluded-macros.txt` with that reason; `check-symbols.py` agrees (`0 mismatched`,
   `invented=[]`).
2. **`docs/ports/print.md` is now stale** about `IEEE_FLOATING_POINT` (see the source-header note
   above); the print lane's files were not touched.
3. **The token gate cannot see these**, all of them recorded in the two notes files rather than
   fixed: C's `goto`/`case`/`switch`/`FALLTHROUGH` keywords, the `0`/`1` of C's `bool`s, `while (1)`,
   `FOPEN_TEXT`/`FOPEN_BINARY` (Rust has no literal concatenation), `INT_BUFSIZE_BOUND`,
   `INT_ADD_OVERFLOW`/`INT_MULTIPLY_OVERFLOW`, `countof` in the two `#@` functions, and the
   `ADDPARAM`/`syms_of_lread` `doc:` labels.
4. **Carried arms are not compiled**: `xtask dead-arms` can only flip the keys `config.rs` carries
   (status 3.9), and the ten features above are platform/compiler predicates, so nothing in this
   tree compiles the Android, module, native-comp, Windows or DOS arms - they are text plus a
   syntax check, exactly as in `print.rs`/`callproc.rs`.

## 2026-10-05: the obarray iterator's empty-bucket marker is the fixnum zero (status 3.614)

`obarray_iter_at_end` is a lisp.h INLINE, so it lives in the mirror; the port wrote its test as
`!NILP (obj)`.  C's test is `!BASE_EQ (obj, make_fixnum (0))`, and the two are not the same
predicate: an empty bucket holds the *fixnum* zero (C writes `make_fixnum (0)` in `make_obarray`,
`grow_obarray`, `obarray-clear` and reads it back in `oblookup`), while `Qnil` is the symbol at
offset 0 of `lispsym`.  The port therefore walked into every empty bucket as if it held a symbol
and skipped the real `nil` symbol; the first `map_obarray` (from `harmonize_variable_watchers`)
died in `XSYMBOL`'s `eassume`.  The mirror's ledger records the fix; this file's own sites
(`oblookup` 5273, the unintern path 5231, the value check 5038) were already C's.

## 2026-10-05: `defsym_name` carries the Lisp names, not the C identifiers (status 3.615)

C's generated `defsym_name[]` (build/emacs-ref/src/globals.h 5799) holds the **Lisp** names -
`"nil"`, `"t"`, `"load"`, `"scroll-up"` - and `init_obarray_once` names every builtin symbol
with it.  The port's table held the *C identifiers* instead (`"Qnil"`, `"Qt"`, `"Qload"`, ...):
1423 of the 2118 entries differ between the two spellings.  Nothing dispatches through a symbol's
name, so the defect was invisible until elisp ran: `defsubr` interns the subr's Lisp name
(`"load"`), which - with the builtin symbol misnamed `"Qload"` - created a *second* symbol, so
`(load "loadup.el")` called a void function and the batch error path (which is where the first
*print* happens) had nothing to print.  The table is now generated from
`docs/baseline/globals/symbols.tsv`'s `lisp_name` column, in that file's index order (the port's
own 2118-symbol superset, P-29), with the formatting of the previous table kept (four entries a
line, `c"...".as_ptr ()`).

## 2026-10-05: the nested load handed the outer reader the inner, closed stream (status 3.616)

`Fload` registers `close_infile_unwind` to put the *previous* `infile` back when the load unwinds,
and C's argument is `infile` - the file the reader returns to (`set_unwind_protect_ptr (fd_index,
close_infile_unwind, infile)`, lread.c 1466; `infile = &input` comes *after*).  The port passed
`&input` - the load's *own* stream, which the unwinder then installs as the current one.  So after
the first nested `(load ...)` of `loadup.el`, the outer reader was reading a closed stream: it saw
EOF, the outer load returned, and elisp stopped after two files (`loadup.el`,
`emacs-lisp/debug-early`) while the command loop took over.  With C's argument the chain runs
`debug-early -> byte-run -> backquote -> subr` and keeps going.
