# emacs/lib/lstat.c (97 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] orig_lstat  (line 39, static)
- [x] rpl_lstat  (line 66)

## Macros defined — 1
- [x] __need_system_sys_stat_h  (line 23, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

# emacs/lib/lstat.c (97 lines) — port `lib/lstat.rs`

## Decisions

- **Includes**: C 23's `#define __need_system_sys_stat_h` before <config.h> selects the system
  <sys/stat.h>; the mirror declares the system entries itself, so `orig_lstat` calls the real
  `lstat` and no replacement macro is involved.  C 34-50's <sys/types.h>, <sys/stat.h>,
  stat-time.h, <string.h> and <errno.h> are `sys_types.in.rs`, the new `sys_stat.in.rs`, the new
  `stat-time.rs`, an in-unit `strlen` declaration (string.in.h has no mirror yet) and `errno.rs`.
- **HAVE_LSTAT is 1**, so C 26-30's `#if !HAVE_LSTAT` arm (`typedef int dummy;`) is dead text
  kept under `#[cfg (feature = "no_lstat")]` (P-51); the header mirror and the live body below
  are what the frozen configuration compiles.
- **`file[strlen (file) - 1] != '/'`** is C's unchecked load at an index the comment justifies;
  the port spells it `*file.add (strlen (file) - 1)`.
- No call-relationship differences (D7): `orig_lstat`, `stat_time_normalize`, `stat` and the
  three `S_IS*` tests appear once each, in C's order.
