# emacs/lib/stdio-consolesafe.c (202 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] workaround_fwrite0  (line 31, static)
- [x] _gl_consolesafe_fwrite  (line 59)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- Only C 31-68 is live: the `#if defined __MINGW32__ && __USE_MINGW_ANSI_STDIO` arm from C 70 on
  (with its `fseterr.h` and `vasprintf` fallbacks) is dead on this target.  The tolerance row
  records that the port is 59% shorter than C for that reason.
- C's `ckd_mul (&nbytes, size, nmemb) || nbytes == 0` is the `overflowing_mul` pair plus the
  zero test (PORTING.md 2.6/P-15, the token-aliases family).
- `fputs`/`fputc` and `EOF` were added to the stdio mirror for the first function; `malloc`/
  `free` come from the stdlib mirror and `strlen`/`memcpy` from the string mirror.
- C's `tmp[nbytes] = '\0';` and the `ptr += l + 1` walk keep C's shape.
- No call-relationship differences (D7).
