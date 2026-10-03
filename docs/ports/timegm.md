# emacs/lib/timegm.c (57 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] __timegm64  (line 30)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- `_LIBC` is undefined, so C 20-22's `<libc-config.h>` include is live and C 36-57's
  `#if defined _LIBC && __TIMESIZE != 64` arm — the 32-bit `timegm` wrapper with
  `libc_hidden_def`, `in_time_t_range` and `__set_errno` — is dead; it is named in the file's
  `// femacs:` note (P-51), which is why the port is shorter than the reference.
- The definition's two renames are mktime-internal.h 53's `# define __timegm64 timegm` and
  the generated `<time.h>`'s `#define timegm rpl_timegm` (REPLACE_TIMEGM), so the symbol is
  the reference object's `_rpl_timegm` (`nm -g build/emacs-ref/lib/timegm.o`).  The port
  keeps C's source name `__timegm64` and emits the real symbol with
  `#[unsafe(export_name = "rpl_timegm")]` (PORTING.md 10.13①, the mempcpy precedent).
- C 32's function-`static` `gmtime_offset` is the port's `static mut` in the same function
  body (C zero-initializes it); `__mktime_internal (tmp, false, &gmtime_offset)` is the
  mktime-internal mirror's rename macro plus `&raw mut` for C's `&`.
- Line-count drift (C 57 vs the port's 43) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
