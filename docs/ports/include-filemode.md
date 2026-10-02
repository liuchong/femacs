# emacs/lib/filemode.h (50 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [ ] FILEMODE_H_  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/filemode.h` @ `fd7afced403`; port `femacs/lib/include/filemode.rs`.  HAVE_DECL_STRMODE
is 1 on this target, so C's `#if !HAVE_DECL_STRMODE` declaration of `strmode` is dead and is
kept as a cfg'd extern (the same feature `femacs/lib/filemode.rs`'s dead arm uses); the live
declaration is the system <string.h>'s.  `filemodestring` is declared in-unit, as C does.
