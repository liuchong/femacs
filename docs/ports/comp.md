# emacs/src/comp.c (5857 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 1
- [x] native-comp-available-p  (Fnative_comp_available_p, 0/0, doc 3c4e6686da50, line 5580)

## Functions defined — 1
- [x] syms_of_comp  (line 5594)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/comp.c` @ `fd7afced403`; port `femacs/src/comp.rs` (59 lines); the
gates are `build-tu src/comp.c` -> `built` and
`python3 tools/extract-abi.py --check emacs/src/comp.c femacs/src/comp.rs` -> `OK`.

- **Which text is live.**  `HAVE_NATIVE_COMP` is `undef`
  (docs/baseline/config/darwin-arm64.macros.tsv 10), so 45 of the file's 5,857 lines are the
  whole of this target's text: C 21-23 (`config.h`, `lisp.h`), `Fnative_comp_available_p` with
  its `#ifdef` arm dead (C 5588's `return Qnil;` is what compiles) and `syms_of_comp`'s one
  `defsubr`.
- **Dead text (recorded here, not transcribed).**  C 25-5578: the `<dynlib.h>` include, the
  `Lisp_Comp_Unit`/`Lisp_Comp_Func`/`Lisp_Comp_Vec` records with `XCOMP`/`COMP_UNIT`/`XCOMP_UNIT`
  and the `comp` allocation helpers, the Limple opcode enum and the cast/alloc/set/comment
  tables, the GIMPLE passes (`emit_*`, `compile_*`, `limple_to_gimple`, `emit_lap_*`), the
  libgccjit entry points and `load_comp_unit`.  C 5596-5854: `syms_of_comp`'s
  `native-comp-jit-compilation` DEFVAR_BOOL, the `comp--#\$` DEFVAR_LISP and the fifty-odd
  `DEFSYM`s for the Limple opcodes and the compiler options.  C 5600's
  `Fprovide (intern_c_string ("native-compile"), Qnil)` is the last of them.
- **Line-count basis.**  `docs/ports/line-drift-tolerance.tsv`'s `comp` row is the first with
  `live` in its fourth column: against C's 5,857 total lines a faithful 59-line port reads as
  99% drift and could only pass by declaring a tolerance that silences the check, so the row
  asks for the baseline `tools/extract-abi.py` now computes from the same branch evaluator the
  declarations use (45 live lines, 31% drift, tolerance 0.35).

## 2026-10-04: the storage of a member whose `DEFVAR` is dead text (3.252)

`Vcomp_eln_to_el_h` is the one member of C's generated `globals` that this unit's dead text
owns and that *live* text elsewhere reads: C 5766's `DEFVAR_LISP` sits inside `#ifdef
HAVE_NATIVE_COMP` (undef here), while `src/lread.c` 1019's `compute_found_effective` reads the
variable with no enclosing `#if` at all.  C's storage for it is `globals.f_Vcomp_eln_to_el_h` -
a field of the generated struct, which exists in every configuration - so the port cannot let the
registration's liveness decide whether the storage exists.  `femacs/src/comp.rs` therefore
carries one `#[unsafe(no_mangle)] pub static mut Vcomp_eln_to_el_h: Lisp_Object = lisp_h_Qnil ();`
even though the `DEFVAR_LISP` line stays recorded as dead text above.  The first real link found
the name referenced by `lread.rs` and defined nowhere (`docs/ports/link.md`, reading three); with
this line the executable link's undefined list is 84 → 83 and `check-globals.py` still reports
284 units / 0 problems.
