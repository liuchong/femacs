# emacs/lib/c-ctype.c  →  lib/c-ctype.rs (20 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] C_CTYPE_INLINE  (line 18, body a9f37094e6e8)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **PORTING.md 10.12 applies**: this file is a gnulib `_GL_*_INLINE` wrapper - its C content
  is `#define <qualifier> _GL_EXTERN_INLINE` and its includes, i.e. the
  C99 inline model's "this unit emits the external definition" file.  The port's functions
  are single Rust items in the header's mirror, so what survives is the macro and its
  relationship to `_GL_EXTERN_INLINE` (the marker in lib/include/gnulib_config.rs).
- **No `// femacs:` header**: C's own file is 20-23 lines, and the drift rule's +/-5% leaves
  no room for one; this ledger is where the provenance is recorded.

- **PORTING.md P-81**: this is one of gnulib's extern-inline carriers - its C text is
  the `#define <qualifier> _GL_EXTERN_INLINE` line and its includes, with the bodies
  in the header - so the reference object exports the *header's* functions and the unit
  re-exports them under C's own names (`#[unsafe(export_name = "…")]` inside
  `mod <stem>_unit`, each calling `super::…` so the module's own item cannot shadow the
  mirror's and recurse).  tools/check-symbols.py compares the result with the reference
  object symbol by symbol; this entry was on docs/baseline/symbol-pending.txt until it
  matched.
  The mirror, `lib/include/c-ctype.rs`, now carries all fifteen predicates of
  c-ctype.h 168-358 and the whole `_C_CTYPE_*` case-list family (the `_N`-parameterised
  two are in docs/ports/excluded-macros.txt: a Rust pattern cannot offset a literal).
  Every function is exported here, matching the reference object's fifteen `T _c_*`
  symbols.
