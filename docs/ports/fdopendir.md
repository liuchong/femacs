# emacs/lib/fdopendir.c (238 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] rpl_fdopendir  (line 225)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- HAVE_FDOPENDIR is 1 on this target, so C 26-214's `#if !HAVE_FDOPENDIR` arm - the whole
  `dirent-private.h` fallback with its own `struct dirent`, the `openat` machinery and the
  `save-cwd` dance - is dead by measurement and is not carried (the tolerance row records that
  the port is 77% shorter than C).  The live text is the twelve-line `rpl_fdopendir` at
  C 225-237.
- C 221's `# undef fdopendir` needs no `system` module here: this unit defines `rpl_fdopendir`
  (not `fdopendir`), so the dirent mirror's own `fdopendir` declaration is the system entry the
  replacement calls.
- The `<dirent.h>` mirror is new (`DIR` as an opaque enum, the directory calls); `STAT`/
  `fstat`/`S_ISDIR` come from the sys_stat mirror and `errno`/`ENOTDIR` from the errno mirror.
- No call-relationship differences (D7): one `fstat`, one `fdopendir`.
