# emacs/lib/utimens.h (82 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] lutimensat  (line 71)

## Macros defined — 3
- [x] _GL_UTIMENS_INLINE  (line 59, body a960df3ab827)
- [x] lutimens  (line 43, body a926805ba082)
- [x] utimens  (line 38, body 2a4a9a803998)

## Types and their fields — 0

## File-scope objects — 0


## Decisions

`emacs/lib/utimens.h` @ `fd7afced403`; port `femacs/lib/include/utimens.rs`.  All checklist
items ticked; `--check` → `OK`.

- The declarations live in `pub mod decls` + glob import, because `lib/utimens.c` defines
  `utimens`/`lutimens`/`fdutimens` (the P-19/P-21/P-49 shape, as in include/mini-gmp.rs).
- `HAVE_UTIMENS`/`HAVE_LUTIMENS` are undefined, so C 36-38/41-43's renames never fire; the
  port keeps them as cfg'd `macro_rules!` twins naming `rpl_utimens`/`rpl_lutimens`.
- The `GNULIB_FDUTIMENSAT` section is dead (`GNULIB_FDUTIMENSAT` is undefined in the frozen
  configuration) but the header is not in the verified list, so the gate compares both arms:
  the port carries the section's one inline definition, `lutimensat` (P-12), plus the
  `fdutimensat` prototype.
- `_GL_UTIMENS_INLINE` is recorded in `docs/ports/excluded-macros.txt` (the
  `_GL_TIMESPEC_INLINE` precedent).
