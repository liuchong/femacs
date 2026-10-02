# emacs/lib/filemode.c (166 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] ftypelet  (line 43, static)
- [x] strmode  (line 85)
- [x] filemodestring  (line 154)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# emacs/lib/filemode.c (166 lines) — port `femacs/lib/filemode.rs`

## Decisions

- **HAVE_DECL_STRMODE is 1** (macOS's <string.h> declares `strmode`), so C 23-107's
  `#if ! HAVE_DECL_STRMODE` arm is dead text kept under the feature that names the frozen side
  (P-51, the getgroups.c shape): `ftypelet` and gnulib's own `strmode` live there, and C 107's
  `#endif` leaves `filemodestring` (C 154-166) as the one live function.
- `filemodestring` calls the *system* `strmode` through the port's `system` module, which is
  what the C compiler does on this target (the declaration comes from <string.h>, not from
  gnulib's header); the filemode.h mirror records C's own `#if !HAVE_DECL_STRMODE` declaration
  as a cfg'd extern.
- The type predicates `ftypelet` uses (`S_ISCTG`, `S_ISDOOR`, `S_ISMPB/C/X`, `S_ISNWK`,
  `S_ISPORT`, `S_ISWHT`) and the four `S_TYPEIS*` tests are gnulib's fallbacks and are **0** on
  this target, which is why every "nonstandard" test in the dead arm is a constant condition;
  they are macro_rules in `sys_stat.in.rs` with C's bodies.
- No call-relationship differences (D7).
