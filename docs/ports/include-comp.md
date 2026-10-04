# emacs/src/comp.h (106 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] NATIVE_COMP_UNITP  (line 57)
- [x] XNATIVE_COMP_UNIT  (line 63)
- [x] maybe_defer_native_compilation  (line 92, static)

## Macros defined — 1
- [x] COMP_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 1
- [x] Lisp_Native_Comp_Unit: union vectorlike_header header, Lisp_Object file, Lisp_Object optimize_qualities, Lisp_Object lambda_gc_guard_h, Lisp_Object lambda_c_name_idx_h, Lisp_Object data_fdoc_v, Lisp_Object data_vec, Lisp_Object* data_relocs, bool loaded_once, bool load_ongoing, dynlib_handle_ptr handle

## File-scope objects — 0

## Decisions

- **The frozen configuration takes comp.h's `#else` branch** (`HAVE_NATIVE_COMP` is `undef`,
  `docs/baseline/config/darwin-arm64.macros.tsv` 1473), and that branch is three items:
  `maybe_defer_native_compilation` (C 92-95) and `unload_comp_unit` (C 97-98), both
  `static inline` no-ops whose empty bodies are the reference's own, plus
  `extern void syms_of_comp (void)` (C 100, comp.c is not ported yet - batch G).  The port
  carries the two inline bodies and leaves `syms_of_comp` to comp.c's own unit.
- **The three items the generator lists above and beyond that branch are dead text (P-08):
  `NATIVE_COMP_UNITP` (C 57), `XNATIVE_COMP_UNIT` (C 63) and the field-carrying
  `struct Lisp_Native_Comp_Unit` (C 30-44) all sit inside `#ifdef HAVE_NATIVE_COMP`, which the
  frozen configuration does not compile, so they are recorded here rather than transcribed.
  Their boxes are checked against that decision, not against a compiled counterpart; the
  reference's own object for this configuration has no such names either.
- **`unload_comp_unit`'s parameter.**  C's `#else` branch never defines the struct, only names
  it in the inline's parameter list - an incomplete type, which C allows and Rust does not
  spell.  The port writes `*mut core::ffi::c_void` (PORTING.md P-72 3) and records it here;
  the field-carrying definition lives in `femacs/src/pdumper.rs` under the `native_comp`
  feature, because `comp.c` (which owns it) is not ported yet.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/comp.h femacs/src/include/comp.rs`
  -> `OK emacs/src/comp.h -> femacs/src/include/comp.rs` (no `- ` problem line).
- `cd femacs && CARGO_TARGET_DIR=/tmp/... cargo run -q -p xtask -- build-tu src/emacs.c`
  -> `built src/emacs.rs -> ../build/femacs/emacs.o`; the same for `src/alloc.c`,
  `src/print.c` and `src/pdumper.c`, which reach this mirror through `lisp.h`'s 2168 include.
- `git -C emacs status --short` -> empty (the reference was not touched).
