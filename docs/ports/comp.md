# emacs/src/comp.c (5857 lines)

> **This ledger overturns the entry it replaces.**  The previous revision of this file recorded
> `comp.c` as a checked-off unit under P-08 ("the whole body is inside a disabled `#if`, so 45 of
> 5857 lines are the whole of this target's text"), with the checklist sections at zero and the
> dead text "recorded, not transcribed".  **P-86 rules the other way**: a file whose body sits
> entirely under a condition the frozen configuration leaves false is *ported by transcription*,
> both arms of every `#if` carried, and its ledger records a **C line-range table** instead of
> ticked boxes, because `tools/extract-abi.py`'s checklist for it is empty (P-73's live-line
> filter; the file *is* in `docs/baseline/config/cond-eval-verified.txt` 253) and the checkbox
> sections are therefore not coverage evidence.  The port is `femacs/src/comp.rs` (1194 lines),
> the file gate is `feature = "have_native_comp"`, and the frozen `build-tu` reading is still the
> same object it was - the transcription must not leak a line into this configuration.
>
> The file is 5857 lines and is ported in **slices**.  **Slice 1 (this revision) is C 25-822** -
> the dynamic loading of libgccjit, this unit's records and its first functions up to
> `hash_native_abi`'s closing brace - and stops there because C 824's `freloc_check_fill` is the
> first place the file needs `FOR_EACH_TAIL`, one of the macros `docs/ports/excluded-macros.txt`
> keeps `kind=deferred` (P-22: "使用这些宏的文件暂不开工").  C 824-5857 is **arriving in tranches** (only C 824-1010 is transcribed so far)
> and the line-range table below says so per range rather than leaving it to prose.

`HAVE_NATIVE_COMP` is `undef` (`docs/baseline/config/darwin-arm64.macros.tsv` 1473); `WINDOWSNT`
is `undef` (4010), `HAVE_ZLIB` is `undef` (1721), `HAVE__SETJMP` is `1` (1722).

`python3 tools/check-cond-eval.py src/comp.c` → `src/comp.c: 60 conditions, 0 unknown, 0
disagreements [VERIFIED]` / `60 conditions, 0 unknown to the evaluator, 0 disagreements`.  The
frozen macro table therefore decides every branch of this file, which is what the *frozen* side of
each row below means; the `LIBGCCJIT_HAVE_*` rows are decided as *undefined* there for the
mechanical reason that the table has no entry for them (the header that would define them is
included by no configuration - see the gap note below), and the arm-flip reading in `## Evidence`
③ is what measures what that costs.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`#ifdef HAVE_NATIVE_COMP`, C 25) | `feature = "have_native_comp"` (on every item and on C 27-41's include graph) | a native-compilation configuration |
| C 48 `#ifdef WINDOWSNT` … C 442 (the DLL loading, the `#undef`s, the sixty-four `DEF_DLL_FN` uses, `init_gccjit_functions`, the `#define` renames) | `all (feature = "have_native_comp", feature = "windowsnt")` | a Windows configuration |
| C 156, 273, 354, 357, 417, 436 `LIBGCCJIT_HAVE_gcc_jit_global_set_initializer` / `_gcc_jit_version` | `... feature = "libgccjit_have_gcc_jit_global_set_initializer"` / `"libgccjit_have_gcc_jit_version"` (nested) | a header that defines them |
| C 186, 305, 386, 533, 609 `LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast` (four `#ifdef`, one `#ifndef`) | `"libgccjit_have_gcc_jit_context_new_bitcast"` (the `#ifndef` side is the *frozen* side: no gate) | a header that defines it |
| C 233, 349, 431 `LIBGCCJIT_HAVE_REFLECTION` | `"libgccjit_have_reflection"` (nested) | a header that defines it |
| C 196, 321, 402 `#if LISP_WORDS_ARE_POINTERS` | none: the condition is *true* on this target (`include/lisp.rs` 3259's `LISP_WORDS_ARE_POINTERS == true`), so the text it guards is the live side and there is no other arm | - |
| C 447/464/466 `#ifdef WINDOWSNT` inside `load_gccjit_if_necessary` | `feature = "windowsnt"` / `not (feature = "windowsnt")` | both arms carried |
| C 511-520 the four-way `SETJMP` choice | `feature = "windowsnt"` × `feature = "have__setjmp"`; no Rust counterpart (see Deviations) | - |
| C 569 `#ifdef LISP_OBJECT_IS_STRUCT` | `"lisp_object_is_struct"` | an `LISP_OBJECT_IS_STRUCT` configuration |
| C 738, 752/756 `HAVE_ZLIB` (the two blocks in `comp_hash_source_file`) | `not (feature = "have_zlib")` / `feature = "have_zlib"` | both arms carried |
| C 797 `#ifdef NS_SELF_CONTAINED` (in `hash_native_abi`) | `"ns_self_contained"` | a self-contained app-bundle configuration |
| C 5585 `#ifdef HAVE_NATIVE_COMP` in `Fnative_comp_available_p` | `feature = "have_native_comp"` / `not (...)` | both arms carried |
| C 5596-5854 `#ifdef HAVE_NATIVE_COMP` in `syms_of_comp` | `feature = "have_native_comp"` (slice 2: tranche 2, not transcribed yet) | - |

**The missing gates are named here, not added.**  Eight of the features the table names -
`libgccjit_have_gcc_jit_global_set_initializer`, `libgccjit_have_gcc_jit_context_new_bitcast`,
`libgccjit_have_reflection`, `libgccjit_have_gcc_jit_version`, `have_zlib`, `ns_self_contained`,
`lisp_object_is_struct`, `have__setjmp` - are **not** in `femacs/xtask/Cargo.toml`'s `[features]`
table, so no build selects their arms and `xtask dead-arms` cannot flip them either (that command
knows seven `FLIPS`, and `have_native_comp` is one of them).  The one consequence that is
*measured* rather than predicted is in `## Evidence` ③: the arm-flip compile reaches all of these
arms' text as text.  Registering the features, and adding the four `LIBGCCJIT_HAVE_*` rows to
`docs/baseline/feature-coverage.tsv`, is the integrator's (`## Owed by the integrator` 1).

## C line ranges

The coverage gate (`python3 tools/check-dead-file-coverage.py`) binds this ledger to **both**
reference files of the pair: `emacs/src/comp.c` (the unit `femacs/src/comp.rs`) and
`emacs/src/comp.h` (the mirror `femacs/src/include/comp.rs`, which resolves to this ledger by the
rule `check-ledger-ticks.py` shares).  The ranges below are therefore contiguous - chosen so that
no non-blank line of *either* file falls outside them - and a row names the constructs of whichever
file its range covers (comp.h's rows are the mirror's, and `docs/ports/include-comp.md` is where
that mirror's own notes live).

| C | block | arm |
| --- | --- | --- |
| 1-19 | both files' licence/declaration blocks | dropped (P-86 ①; the port opens with the `// femacs: port of …` line, PORTING.md 0.1-19) |
| 20-24 | C 21 `#include <config.h>` and C 23 `#include "lisp.h"`; comp.h 20-21's `#ifndef COMP_H`/`#define COMP_H` and comp.h 23's `#include <dynlib.h>` | C 21 ungated (every unit includes it), C 23 ungated (C opens the file gate after it); comp.h's are the mirror's |
| 25-26 | C 25 `#ifdef HAVE_NATIVE_COMP` (the file gate); comp.h 25-26, the head of `struct Lisp_Native_Comp_Unit` | the file gate / the mirror |
| 27-46 | C 27-41 the fourteen includes and C 44-46 the banner comment over the libgccjit loader; comp.h 26-46, the record's fields | file gate (see the includes row of `## Deviations` 6 and the gap note); the mirror's record is live text on every configuration |
| 47-49 | C 48 `#ifdef WINDOWSNT` and C 49 `# include "w32common.h"`; comp.h 47-49's last three fields | `windowsnt` / the mirror |
| 50-114 | C 51-114 the sixty-four `#undef`s of the libgccjit entry points; comp.h 50's `} GCALIGNED_STRUCT;`, 52's `#ifdef HAVE_NATIVE_COMP` and 54's `INLINE_HEADER_BEGIN` | `windowsnt`; the `#undef`s have no counterpart (there is nothing to undef - the port never writes the header's macros), `## Deviations` 8; comp.h 56-104's inlines are the mirror's |
| 115-277 | C 116-276 the sixty-four `DEF_DLL_FN` uses, C 277's `#endif` | `windowsnt`, four of them nested under the `LIBGCCJIT_HAVE_*` gates |
| 278-364 | C 279-364 `init_gccjit_functions` (the sixty-four `LOAD_DLL_FN`/`LOAD_DLL_FN_OPT` uses) | `windowsnt` |
| 365-442 | C 366-441 the sixty-four `#define gcc_jit_x fn_gcc_jit_x` renames, C 442's `#endif` | `windowsnt`; an object-like rename has no Rust counterpart, so the two arms' call spellings are the port's (`## Deviations` 2) |
| 443-467 | C 444-467 `load_gccjit_if_necessary` | file gate / `windowsnt` (C 447-466) |
| 468-474 | C 470-471 `ABI_VERSION`, C 473-474 `HASH_LENGTH`; comp.h 89-104's `#else` branch (`maybe_defer_native_compilation`, `unload_comp_unit`, `syms_of_comp`, `INLINE_HEADER_END`) | file gate; comp.h's branch is live text (the mirror's, `docs/ports/include-comp.md`) |
| 475-491 | C 476-491 the twelve relocation-symbol strings | file gate |
| 492-526 | C 493-495 `STR_VALUE`/`STR`, C 496-501 `FIRST`/`SECOND`/`THIRD`, C 503-505 `CALLNI`, C 507-509 `DECL_BLOCK`, C 511-521 the `SETJMP`/`SETJMP_NAME` choice, C 523-524 `F_RELOC_MAX_SIZE`, C 526 the head of `f_reloc_t` | file gate; `DECL_BLOCK` has the two cfg-exclusive definitions (`windowsnt` / not), `SETJMP` is the `windowsnt` × `have__setjmp` choice and has no Rust counterpart (`## Deviations` 2, 3) |
| 527-531 | C 527-529 `f_reloc_t`'s fields and its closing brace, C 531 `static f_reloc_t freloc;` | file gate |
| 532-535 | C 533-535 `#ifndef LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast` / `# define NUM_CAST_TYPES 15` / `#endif` | `not (feature = "libgccjit_have_gcc_jit_context_new_bitcast")` |
| 536-540 | C 537-540 `reloc_array_t` | file gate |
| 541-658 | C 542-543 the comment over the record, C 544-658 `comp_t` | file gate; the two nested conditions give C 569-572's two fields (`lisp_object_is_struct`) and C 610-617's five (`not …new_bitcast`) |
| 659-662 | C 660 `static comp_t comp;`, C 662 `static FILE *logfile;` | file gate |
| 663-674 | C 664-668 `static_obj_t`, C 670-674 `imm_reloc_t` | file gate |
| 675-687 | C 676-678 the banner comment over the helpers, C 680-687 the six forward declarations of this unit's `static` helpers | file gate (a module; `## Deviations` 5) |
| 688-707 | C 689-707 `helper_link_table` | file gate |
| 708-720 | C 709-720 `format_string` | file gate |
| 721-730 | C 722-730 `comp_hash_string` | file gate |
| 731-767 | C 732-767 `comp_hash_source_file` | file gate / `have_zlib` (C 738-743, 752-758) |
| 768-777 | C 769-777 `DEFUN ("comp--subr-signature", …)` | file gate |
| 778-822 | C 779-822 `hash_native_abi` | file gate / `ns_self_contained` (C 797-818) |
| 824-1010 | C 824-1010: `freloc_check_fill` (`FOR_EACH_TAIL`), `bcall0`, `retrieve_block`, `declare_block`, `emit_mvar_lval`, `register_emitter`, `obj_to_reloc`, `emit_comment`, `declare_imported_func`, plus C 504-505's `CALLNI` macro and the seven `<libgccjit.h>` prototypes C 511-517's `#ifndef WINDOWSNT` arm calls directly | **landed (4.686)** - `dead-arms` compiles it with `have_native_comp` on: 340/340 units, 0 failed |
| 1012-1118 | C 1012-1106: `emit_call`, `emit_call_ref`, `emit_cond_jump`; C 1108-1118: `type_to_cast_index` (the live side of `#ifndef LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast`), plus the eight `<libgccjit.h>` prototypes they call and `enum gcc_jit_unary_op`'s alias with `GCC_JIT_UNARY_OP_LOGICAL_NEGATE` | **landed (4.687)** - `dead-arms` 340/340 units, 0 failed |
| 1120-1210 | C 1120-1197: `emit_coerce` (both `#ifdef` arms carried as gates: `lisp_object_is_struct` and `libgccjit_have_gcc_jit_context_new_bitcast`, the second one's live side being the cast-matrix call); C 1199-1210: `emit_binary_op`; plus four prototypes, `enum gcc_jit_binary_op`'s alias and `gcc_jit_object` (named by `gcc_jit_type_is_pointer`'s prototype) | **landed (4.688)** - `dead-arms` 340/340 units, 0 failed |
| 1212-1518 | C 1212-1254 `emit_rvalue_from_long_long`; 1256-1266 `emit_rvalue_from_emacs_uint`; 1268-1276 `emit_rvalue_from_emacs_int`; 1278-1288 `emit_rvalue_from_lisp_word_tag`; 1290-1305 `emit_rvalue_from_lisp_word`; 1307-1316 `emit_rvalue_from_lisp_obj`; 1318-1346 `emit_ptr_arithmetic`; 1348-1353 `emit_XLI`; 1355-1361 `emit_XLP`; 1363-1377 `emit_XUNTAG`; 1379-1387 `emit_XCONS`; 1389-1400 `emit_BASE_EQ`; 1402-1412 `emit_AND`; 1414-1424 `emit_OR`; 1426-1469 `emit_TAGGEDP`; 1471-1477 `emit_VECTORLIKEP`; 1479-1485 `emit_CONSP`; 1487-1504 `emit_SYMBOL_WITH_POS_P`; 1506-1518 `emit_slow_eq`; plus five prototypes and the `gcc_jit_binary_op`/`gcc_jit_comparison` enumerators | **landed (4.689)** - `dead-arms` 340/340 units, 0 failed |
| 1520-1696 | C 1520-1552 `emit_EQ`; 1554-1560 `emit_FLOATP`; 1562-1579 `emit_BIGNUMP`; 1581-1625 `emit_FIXNUMP`; 1627-1653 `emit_XFIXNUM`; 1655-1664 `emit_INTEGERP`; 1666-1675 `emit_NUMBERP`; 1677-1696 `emit_make_fixnum_LSB_TAG`; plus `gcc_jit_rvalue_dereference` and `gcc_jit_context_get_builtin_function`'s prototypes **and C 4932-4951's `comp-libgccjit-version` DEFUN**, transcribed here because `emit_EQ` calls it (it moves to its C position when C 4932 lands) | **landed (4.690)** - `dead-arms` 340/340 units, 0 failed |
| 1698-1953 | C 1698-1723 `emit_make_fixnum_MSB_TAG`; 1726-1733 `emit_make_fixnum`; 1735-1746 `emit_lisp_obj_reloc_lval`; 1748-1762 `emit_lisp_obj_rval`; 1764-1769 `emit_NILP`; 1771-1791 `emit_XCAR`; 1793-1812 `emit_lval_XCAR`; 1814-1837 `emit_XCDR`; 1839-1862 `emit_lval_XCDR`; 1864-1882 `emit_CHECK_CONS`; 1884-1905 `emit_CHECK_SYMBOL_WITH_POS`; 1907-1913 `emit_car_addr`; 1915-1921 `emit_cdr_addr`; 1923-1935 `emit_XSETCAR`; 1937-1949 `emit_XSETCDR`; two prototypes (`gcc_jit_context_new_array_access`, `gcc_jit_block_add_eval`); C 1951-1953's LIMPLE banner | **landed (4.691)** - `dead-arms` 340/340 units, 0 failed |
| 1955-2084 | C 1955-1996 `emit_mvar_rval`; 1998-2007 `emit_frame_assignment`; 2009-2033 `emit_set_internal`; 2037-2052 `emit_simple_limple_call`; 2054-2061 `emit_simple_limple_call_lisp_ret`; 2063-2067 `emit_simple_limple_call_void_ret`; 2071-2084 `emit_limple_call` (with the `Llisp_emitter` alias for C 2079's function-pointer type); one prototype | **landed (4.692)** - `dead-arms` 340/340 units, 0 failed |
| 2086-2222 | C 2086-2143 `emit_limple_call_ref`; 2145-2192 `emit_setjmp` (both arms: the live `#ifndef WINDOWSNT` one and the `windowsnt` feature's); 2196-2222 `emit_limple_push_handler`; plus `gcc_jit_context_new_param`/`gcc_jit_context_new_function`'s prototypes and `gcc_jit_function_kind`'s enumerators | **landed (4.693)** - `dead-arms` 340/340 units, 0 failed |
| 2224-2526 | C 2224-2521 `emit_limple_insn` - the whole LIMPLE statement dispatcher (jump, cond-jump, cond-jump-narg-leq, phi/assume, push/pop/fetch handler, call/callref/direct-call/direct-callref, set, set-par-to-local, set-args-to-local, set-rest-args-to-local, inc-args, setimm, comment, return, unreachable, and the `native-ice` fallback); five prototypes; `GCC_JIT_COMPARISON_LE`; C 2524-2526's Inliners banner | **landed (4.694)** - `dead-arms` 340/340 units, 0 failed |
| 2528-2655 | C 2528-2542 `emit_call_with_type_hint`; 2545-2560 `emit_call2_with_type_hint`; 2563-2567 `emit_add1`; 2569-2573 `emit_sub1`; 2575-2579 `emit_negate`; 2581-2591 `emit_consp`; 2593-2605 `emit_eq`; 2608-2612 `emit_car`; 2614-2618 `emit_cdr`; 2620-2624 `emit_setcar`; 2626-2630 `emit_setcdr`; 2632-2639 `emit_numperp`; 2641-2648 `emit_integerp`; 2650-2655 `emit_maybe_gc_or_quit` | **landed (4.695)** - `dead-arms` 340/340 units, 0 failed |
| 2657-2892 | C 2661-2851 `emit_static_object` (both arms: the `gcc_jit_global_set_initializer` one and the chunked-`memcpy` one the frozen macro leaves live); C 2854-2892 `declare_imported_data_relocs`; four prototypes and `gcc_jit_global_kind`'s enumerators; `strnlen`/`mempcpy` declared where the mirror has no copy | **landed (4.696)** - `dead-arms` 340/340 units, 0 failed |
| 2894-3076 | C 2894-2908 `declare_imported_data`; 2915-2979 `declare_runtime_imported_funcs` with C 2920-2927's `ADD_IMPORTED` macro; 2984-3068 `emit_ctxt_code` (the optim-quality and doc statics, the current-thread/symbols-with-pos/comp-unit globals, the `freloc` field table over the runtime helpers and the subr list, and the exported link table) | **landed (4.697)** - `dead-arms` 340/340 units, 0 failed |
| 3077-3360 | C 3077-3184 `define_lisp_cons`; 3186-3217 `define_lisp_symbol_with_position`; 3221-3238 `define_jmp_buf`; 3240-3253 `define_memcpy`; 3257-3315 `define_handler_struct`; 3317-3360 `define_thread_state_struct`; three prototypes (`context_new_opaque_struct`, `struct_set_fields`, `context_new_union_type`); C 3071-3073's banner | **landed (4.698)** - `dead-arms` 340/340 units, 0 failed |
| 3362-3506 | C 3363-3399 `define_type_punning`; 3401-3406 `struct cast_type`; 3408-3446 `define_cast_from_to`; 3448-3505 `define_cast_functions` (the fifteen-entry type table, the punning union and the 15x15 matrix) | **landed and verified (4.702)** |
| 3508-4776 | C 3508-3558 `define_CHECK_TYPE`; 3562-3635 `define_CAR_CDR`; 3637-3690 `define_setcar_setcdr`; 3697-3776 `define_add1_sub1`; 3778-3847 `define_negate`; 3851-3902 `define_PSEUDOVECTORP`; 3904-3936 `define_GET_SYMBOL_WITH_POSITION`; 3938-3978 `define_SYMBOL_WITH_POS_SYM`; 3980-4068 `define_maybe_gc_or_quit`; 4072-4107 `define_bool_to_lisp_obj`; 4109-4163 `declare_lex_function`; 4167-4182 `declare_function`; 4184-4283 `compile_function`; 4290-4298 `make_directory_wrapper`(+`_1`); 4306-4470 the two eln-filename DEFUNs; 4472-4702 the four ctxt DEFUNs; 4704-4776 the two option passers | **landed and verified (4.702)** |
| 4778-5531 | C 4778-4928 `comp--compile-ctxt-to-file0`; 4961-5010 the `helper_*` family; 5036-5053 `eln_load_path_final_clean_up`; 5057-5062 `register_native_comp_unit`; 5076-5128 `maybe_defer_native_compilation`; 5137-5230 `fixup_eln_load_path`/`load_static_obj`/`find_relocs`/`check_comp_unit_relocs`/`unset_cu_load_ongoing`; 5232-5367 `load_comp_unit`; 5369-5398 `unload_comp_unit`/`native_function_doc`; 5400-5449 `make_subr`; 5451-5531 the three registration DEFUNs and `file_in_eln_sys_dir` | **landed and verified (4.702)** |
| 5533-5595 | C 5533-5590 `native-elisp-load` and `native-comp-available-p`; C 5591-5595 the blank run and `syms_of_comp`'s head; C 5596-5854 `syms_of_comp`'s `#ifdef HAVE_NATIVE_COMP` region (the `DEFVAR_BOOL`, the `DEFVAR_LISP`s, the ~50 `DEFSYM`s, the five error-type `Fput` pairs, the fourteen `defsubr`s, the `staticpro`s and `Fprovide`) | **landed and verified (4.702) - `comp.c` is transcribed end to end** |
| 5596-5857 | C 5596-5854 `syms_of_comp`'s body; C 5856 `defsubr (&Snative_comp_available_p);` and C 5857's closing brace | live text |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/comp1-lane/cargo cargo run -q -p xtask -- build-tu src/comp.c`
→ `built src/comp.rs -> ../build/femacs/comp.o`, **exit 0**, 2 warnings (both pre-existing, both
`unused doc comment`, one in `lib/include/stdbit.in.rs` 652 and one in `include/lisp.rs` 3934 -
the same two the pre-transcription revision reported).  `build/femacs/comp.o` is **2784 bytes,
sha256 `39e24211dd6f0c760e4db8b73a57e57746fec8d0a79316068aefcfb8e393445c`**, byte-identical to the
object the 48-line revision built: the transcription leaks nothing into this configuration, which
is the reading P-86 ③ asks for by name.

**② The condition evaluator.**  `python3 tools/check-cond-eval.py src/comp.c` →
`src/comp.c: 60 conditions, 0 unknown, 0 disagreements [VERIFIED]`.

**③ The arm flip (P-86 ③⑴), and what it measures.**
`cargo run -q -p xtask -- probe src/comp.rs /tmp/comp1-lane/comp_arm.o --crate-type lib --emit obj
-C panic=abort --cfg 'feature="have_native_comp"'`
→ **exit 1**, `aborting due to 84 previous errors`, and the classes are exactly two:

  * **83 × `E0425 cannot find type`**, all in the unit: `gcc_jit_type` (30),
    `gcc_jit_function` (19), `gcc_jit_field` (12), `gcc_jit_rvalue` (8), `gcc_jit_struct` (5),
    `gcc_jit_lvalue` (5), `gcc_jit_context` (1), `gcc_jit_block` (1), `VaList` (2).  The first
    eight are the header gap below; `VaList` was an unqualified spelling and is now
    `core::ffi::VaList<'_>` (the last edit of this revision, `## Owed` 4).
  * **1 × `E0080`** in the `include/lisp.rs` copy, not in this unit: `const _: () = core::assert!
    (!cfg! (feature = "have_native_comp") || HAVE_NATIVE_COMP);` (mirror line 2853).  The single
    `--cfg` P-86 ③⑴ prescribes therefore trips the mirror's own feature/configuration consistency
    binding - the arm that *is* faithful needs the configuration constant flipped as well, which
    is what `xtask dead-arms` does (it copies `femacs/` to `build/dead-arms/` and flips both).
    That command compiles every finished unit and is outside this lane's budget, so this reading
    is the single-`cfg` one, with this artifact named.

  Nothing else fails: the parse, the `DEF_DLL_FN!`/`LOAD_DLL_FN!`/`STR!`/`DECL_BLOCK!` expansions,
  the item and include-graph resolution and the frozen-mirror copies are clean, and the 9 errors
  the first revision of this slice produced in the *mirror* copies (`UNIBYTE_TO_CHAR`,
  `STRING_CHAR`, `string_char_and_length`, `BYTES_BY_CHAR_HEAD`, `raw_prev_char_len` in the
  `buffer.rs` copy) are gone once `include/character.rs` is in the graph - which is the include
  rule PORTING.md 0.1-4 states and `decompress.rs` records.  P-86 ⑦'s warning applies in reverse
  and is measured: the *unit*'s own errors are separable from the mirror copies' by path
  (`_root.rs` vs `_Users_…_include_buffer_rs.rs`), and the count above is the unit's.
  **The defects this reading found and this revision fixed** are worth naming one by one: the
  `comp_helpers` module's twelve prototypes needed an `unsafe extern "C"` block (12 × "free
  function without a body"), `format_string`'s C variadic needed `unsafe extern "C" fn`
  ("`...` is not supported for non-extern functions"), `emacs_fclose` is `include/lisp.rs` 3148's
  macro (`emacs_fclose!`, not a call), `comp_hash_source_file`'s `int res` is declared inside
  C 752's `#ifdef` but *scoped* to the enclosing block (a Rust `let` in the arm is not), and
  `set_internal`/`vsnprintf` are mirror gaps that have to be declared at the use site (P-72
  rule 4).  Two more were found by the tools: `load_gccjit_if_necessary` is C's `static`, so
  `#[unsafe(no_mangle)]` on it was an invented exported symbol (`extract-abi` ④), and the
  include graph needed `character.rs`.

**④ The inventory and line readings (`extract-abi`).**
`python3 tools/extract-abi.py --check emacs/src/comp.c femacs/src/comp.rs` →
`- line count drifted 2553.3%: C 45 live (of 5857) vs Rust 1194` / `FAIL emacs/src/comp.c ->
femacs/src/comp.rs`.  The one finding is the P-86 class's own: the live-line filter leaves this
file 45 lines, so a transcribed port cannot read as anything but drift; P-86 ⑩'s remedy is a
baseline row in `docs/ports/line-drift-tolerance.tsv`, and **that row is written now** (status
3.678: `comp 0.80 total`, whose number is the measured shortfall and whose reason names C 823-5577
and C 5596-5854 as the owed text - the row above was the `live` reading this note describes).  The reading the previous revision's `OK` came from
is the empty checklist; what changed is the finding the checklist could not produce.  (The first
revision of this slice also read `- fn load_gccjit_if_necessary carries #[unsafe(no_mangle)] but
the reference does not define a non-static function of that name (invented exported symbol)`;
that was a real defect and is fixed - the function is C's `static`.)

**⑤ The layouts (clang, against this machine's header, not memory).**
`clang -I/opt/homebrew/include …` on a probe that takes C 533-658 verbatim and prints
`sizeof`/`offsetof`: `sizeof (comp_t)` **2680**, `_Alignof` **8**, with `cast_union_type` at 432,
`cast_ptr_to_int` 2240, `cast_int_to_ptr` 2248, `cast_types` 2256, `func_speed` 2392, `block`
2408, `data_relocs` 2600, `d_default_idx` 2664, `d_ephemeral_idx` 2672; and **736**/8 with C 610's
fields removed (`func_speed` 448, `block` 464, `data_relocs` 656, `d_default_idx` 720,
`d_ephemeral_idx` 728).  The two sides are C 610's `#ifndef`: the 2680 shape is the one whose
condition the frozen macro table reads as *undefined*, the 736 shape the one this machine's header
selects (it defines `LIBGCCJIT_HAVE_gcc_jit_context_new_bitcast`), and the port carries both, each
with its own `const _` asserts.  The second probe reads `sizeof (f_reloc_t)` **12808** (`size` at
12800), `sizeof (reloc_array_t)` **16** (`r_val` at 8), `sizeof (static_obj_t)` **8** (`data` at
8, the flexible member), `sizeof (imm_reloc_t)` **24** (`idx` at 16).

**⑥ This machine *has* libgccjit (measured; the task's premise says it does not).**
`ls -la /opt/homebrew/include/libgccjit.h` → a symlink into `Cellar/libgccjit/16.2.0/include/`
(70,527 bytes); `/opt/homebrew/lib/gcc/16/libgccjit.dylib` exists; `brew list --versions libgccjit`
→ `libgccjit 16.2.0`.  A bare `printf '#include <libgccjit.h>' | clang -E -x c -` fails only
because clang's default search path does not carry `/opt/homebrew/include`; with `-I` the header
compiles and is what ⑤ measures.  So the toolchain *is* present, and what is missing is not the
SDK but the **declaration section** this unit would have to carry for the arm to compile at all
(P-88's `<zlib.h>`/`<lcms2.h>` path).  That is the gap the next paragraph names.

**The declaration gap, named (`## Evidence` ③'s eight type names).**  C 31 includes
`<libgccjit.h>`, a third-party header with no file anywhere in the reference tree.  The two other
units in that class (`decompress.rs`, `lcms.rs`) carry the declarations they use in the unit
(P-88); **this slice does not**, on this lane's instruction to name the missing pieces rather than
add them, so every `gcc_jit_*` name below is C's own name left undeclared, and the arm's reading
is the 83 name-resolution errors of ③.  What the slice therefore does **not** yet carry, item by
item: the eight opaque types (`gcc_jit_context`, `gcc_jit_block`, `gcc_jit_type`,
`gcc_jit_function`, `gcc_jit_lvalue`, `gcc_jit_rvalue`, `gcc_jit_field`, `gcc_jit_struct`) - all
of them pointer-only in C, so PORTING.md 0.2/P-55's spelling is available for them the moment the
integrator decides the shape; the ten enums C names by value in the `DEF_DLL_FN` signatures
(`gcc_jit_function_kind`, `gcc_jit_global_kind`, `gcc_jit_binary_op`, `gcc_jit_comparison`,
`gcc_jit_unary_op`, `gcc_jit_output_kind`, `gcc_jit_bool_option`, `gcc_jit_int_option`,
`gcc_jit_str_option`, `gcc_jit_types`); and the header's own `LIBGCCJIT_HAVE_*` macros, which the
gates of the table above name.  The `const _` layout asserts of ⑤ are inside that gap too: their
numbers are clang's, and the arm cannot check them until the declarations exist - so they are
**text**, and this ledger does not claim them as a passing check.

## Deviations

1. **`DEF_DLL_FN`/`LOAD_DLL_FN`/`LOAD_DLL_FN_OPT` are written in long form** (C 116-364), the shape
   `decompress.rs` (first consumer) and `lcms.rs` (second) settled: the two generated
   `W32_PFN_gcc_jit_…`/`fn_gcc_jit_…` names are `macro_rules!` parameters, since C pastes them
   from one identifier and Rust cannot; C's `__VA_ARGS__` (the parameter list) is the `$args`
   fragment.  `emacs/src/w32common.h` 77-97 is a `kind=deferred` row in
   `docs/ports/excluded-macros.txt` and stays so (`femacs/src/include/w32common.rs` 175-183 keeps
   C's text).  C's `CDECL` is empty on this ABI.
2. **An object-like `#define` that renames an identifier has no Rust counterpart.**  C 366-441's
   sixty-four `#define gcc_jit_x fn_gcc_jit_x`, C 57-63-class `#undef`s and C 515-526's
   `SETJMP`/`SETJMP_NAME` are the three instances here.  The renames are realized as the arms' two
   call spellings (`(fn_gcc_jit_x.unwrap ()) (…)` under `windowsnt`, `gcc_jit_x (…)` outside it)
   and `SETJMP` as `_setjmp`/`setjmp` at the call sites; `DECL_BLOCK` needs the two spellings
   inside *two cfg-exclusive macro definitions*, because a `macro_rules!` body resolves its names
   where it is defined (`decompress.rs`'s `inflateInit2` row).  **No line of this slice is a call
   site of any of the three**, so no unix-side spelling is carried yet; the slices that add call
   sites carry both, and this row is the record.
3. **`CALLNI`'s call site passes C's stringified name as a literal.**  C's `STR (fun)` stringifies
   a token sequence, and the file's only call site (C 884) is `CALLNI (comp-mvar-slot, mvar)`,
   whose name lexes as five tokens; Rust's `stringify!` inserts a space between them
   (`"comp - mvar - slot"`), so the call site passes `"comp-mvar-slot"` and the macro takes
   `$fun:literal`.  `STR`/`STR_VALUE` themselves are carried (C 493-495), and the single-token
   uses (`DECL_BLOCK`'s `STR (name)`) go through them unchanged.
4. **`format_string` (C 709-720) is `unsafe extern "C" fn`, not a plain `unsafe fn`:** C's `static`
   helper is C-variadic, and Rust allows `...` only on `extern` functions (`decompress.rs`'s
   `md5_gz_stream`-class helpers stay plain `unsafe fn` because they are not variadic).
   `va_start` is `let ap: core::ffi::VaList<'_> = args;` and `va_end` has no counterpart
   (PORTING.md P-04).
5. **C 680-687's forward declarations stand as declarations, in a module.**  A C forward
   declaration has no Rust counterpart when the definition follows in the unit (`alloc.rs`
   471-474), but these definitions are slice 2's and C 689-707 takes their addresses, so they are
   declared in `pub mod comp_helpers` and imported with `use comp_helpers::*;` - PORTING.md 0.10's
   mechanism, which lets slice 2's definitions shadow the glob import instead of colliding
   (`E0428`).  They are `extern "C"` because C takes their address and the native-compiled code
   calls them through that pointer; **slice 2's definitions must keep the C ABI.**
6. **Mirror gaps declared at the use site (P-72 rule 4).**  `push_handler` (lisp.h 4812),
   `record_unwind_protect_excursion` (lisp.h 4826), `set_internal` (lisp.h 4206), `suffix_p`
   (lisp.h 4740), `emacs_fopen` (lisp.h 5322), `md5_gz_stream` (lisp.h 5455), `hexbuf_digest`
   (lisp.h 4247) and `vsnprintf` (`<stdio.h>`; `lib/include/stdio.in.rs` 301 carries it, but no
   unit reaches that template mirror - `femacs/src/android.rs` 1347-1348's note) are declared in
   this unit with C's prototype in a `/// C: …` line.  `emacs_fclose` is *not* one of them: it is
   `include/lisp.rs` 3148's macro and is called as such.
7. **Ungated locals C declares inside an `#if`** are declared once before the arms
   (`comp_hash_source_file`'s `int res`, C 752-758): a C declaration's scope is the enclosing
   block, not the preprocessor arm, and Rust's is the arm's.
8. **The `#undef`s of C 51-114 have no counterpart** and are recorded as the comment at C 366's
   row: the port never writes the header's macros, so there is nothing to cancel.
9. **`set_internal`'s fourth argument** is `lisp.h` 4201's `enum set_internal_bind`, which
   `include/lisp.rs` 1811-1813 mirrors as `Set_Internal_Bind` (bytecode.rs 109-110's spelling);
   C's `enum ` keyword drops, as everywhere in the port.

## Owed by the integrator (nothing here is claimed as done)

1. `femacs/xtask/Cargo.toml` `[features]`: the eight gate names of the Gate-mapping note, and
   `docs/baseline/feature-coverage.tsv` (+ the generated `docs/port-map.md` line counts).
2. `docs/ports/line-drift-tolerance.tsv`: the `src/comp.c` row is stale - it was written for the
   45-live-line revision (tolerance 0.35) and the transcription reads 2553.3% drift.  Same for
   `tools/check-port-lines.py --write-pending`'s row.
3. `docs/ports/excluded-macros.txt`: `FOR_EACH_TAIL` (C 824) is what stops slice 1's boundary; if
   its shape is decided before slice 2, say so there rather than in this ledger.
4. The `VaList` qualification in `format_string`/the `vsnprintf` declaration landed **after** the
   arm reading of `## Evidence` ③, so that reading reports 2 `VaList` errors the file no longer
   has; the spelling is `femacs/src/eval.rs` 105/2251's and was checked on its own under the
   pinned `nightly-2026-10-01`, not as part of an arm run.
5. The `libgccjit.h` declaration shape (P-88's in-unit section vs `*mut c_void` per P-55) is the
   decision that turns `## Evidence` ③ into a real compile reading; it is not this lane's, and the
   header **is** on this machine (⑥) so it can be clang-measured, not remembered.
6. `docs/baseline/config/cond-eval-verified.txt` and `cond-eval` cover this file (253); nothing is
   owed there.  `docs/status.md` (not in this lane's whitelist) is where the batch's own record of
   this overturn belongs.
