# emacs/lib/streq.c  →  femacs/lib/streq.rs (19 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _GL_STREQ_INLINE  (line 17, body a9f37094e6e8)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **PORTING.md P-81**: this is one of gnulib's extern-inline carriers - its C text is
  the `#define <qualifier> _GL_EXTERN_INLINE` line and its includes, with the bodies
  in the header - so the reference object exports the *header's* functions and the unit
  re-exports them under C's own names (`#[unsafe(export_name = "…")]` inside
  `mod <stem>_unit`, each calling `super::…` so the module's own item cannot shadow the
  mirror's and recurse).  tools/check-symbols.py compares the result with the reference
  object symbol by symbol; this entry was on docs/baseline/symbol-pending.txt until it
  matched.
  The body lives in the <string.h> mirror, which now defines `streq` as the *function*
  string.in.h 813-828 declares (`return !strcmp (…)` as `== 0`); it had been a
  `macro_rules!` in the port, and the reference object's `T _streq` is the evidence for
  the function shape.  The three call sites that used the macro form (time_rz.rs 191, 264
  and sig2str.rs 152) now call the function, which is also how C spells them.
