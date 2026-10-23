# emacs/lib/fstatat.c (150 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 4
- [x] orig_fstatat  (line 33, static)
- [x] normal_fstatat  (line 60, static)
- [x] rpl_fstatat  (line 75)
- [x] stat_func  (line 116, static)

## Macros defined — 10
- [x] AT_EMPTY_PATH  (line 50, body b6589fc6ab0d)
- [x] AT_FUNC_F1  (line 137, body f2b316c5eafa)
- [x] AT_FUNC_F2  (line 138, body 3efb3246d1a8)
- [x] AT_FUNC_NAME  (line 136, body cc8486628caa)
- [x] AT_FUNC_POST_FILE_ARGS  (line 141, body fcd18921b6e9)
- [x] AT_FUNC_POST_FILE_PARAM_DECLS  (line 140, body 16a231b9d999)
- [x] AT_FUNC_USE_F1_COND  (line 139, body f1d7444f129d)
- [x] LSTAT_FOLLOWS_SLASHED_SYMLINK  (line 56, body b6589fc6ab0d)
- [x] __need_system_sys_stat_h  (line 23, body da39a3ee5e6b)
- [x] lstat  (line 125, body 3efb3246d1a8)

## Types and their fields — 0

## File-scope objects — 0

# emacs/lib/fstatat.c (150 lines) — port `lib/fstatat.rs`

## Decisions

- **Includes**: C 23's `#define __need_system_sys_stat_h` is the lstat.c shape; C 38's
  <sys/stat.h> is the new `sys_stat.in.rs`, C 42's stat-time.h the new `stat-time.rs`, C 44-47's
  <errno.h>, <fcntl.h>, <stdlib.h> and <string.h> are `errno.rs`, the new `fcntl.in.rs` and
  in-unit `malloc`/`free`/`strlen` declarations (stdlib.in.h and string.in.h have no mirror yet).
- **HAVE_FSTATAT and HAVE_WORKING_FSTATAT_ZERO_FLAG are both 1**, so C 53-106 is the live text
  and C 108-150's `#else` arm is dead: its `stat_func` and the six `AT_FUNC_*` macros are kept
  under `#[cfg (feature = "no_working_fstatat_zero_flag")]` (P-51), and the `#include "at-func.c"`
  expansion (the /proc/self/fd simulation over save-cwd.h/openat.h/openat-priv.h, headers this
  target has not got) is recorded in `docs/ports/macro-body-notes.tsv` - the gettimeofday.c
  Windows-arm treatment.
- **`AT_EMPTY_PATH`** is not defined by this target's <fcntl.h>, so C 49-51's fallback is live
  and the port keeps it as a constant; the tests that use it (`flags & AT_EMPTY_PATH && !file`)
  therefore never fire, exactly as in C.
- **`file = ""`** for a null pointer is a C string literal; the port uses Rust's `c""`.
- No call-relationship differences (D7): `orig_fstatat`, `normal_fstatat` (C 97's recursive
  call included), `strlen`, `S_ISDIR`/`S_ISLNK` and `errno` appear in C's order and count.
