# emacs/lib/readlinkat.c (123 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] rpl_readlinkat  (line 35)

## Macros defined — 5
- [x] AT_FUNC_F1  (line 112, body cd11917efc07)
- [x] AT_FUNC_NAME  (line 111, body 78d2e82dc9c4)
- [x] AT_FUNC_POST_FILE_ARGS  (line 114, body 22829af8a565)
- [x] AT_FUNC_POST_FILE_PARAM_DECLS  (line 113, body a94482e8d7bb)
- [x] AT_FUNC_RESULT  (line 115, body 1e6518329e54)

## Types and their fields — 0

## File-scope objects — 0

# emacs/lib/readlinkat.c (123 lines) — port `lib/readlinkat.rs`

## Decisions

- **HAVE_READLINKAT is 1**: C 25-91's arm (`rpl_readlinkat`) is the live text and C 93-123's
  `#else` arm - the `#include "at-func.c"` instantiation over readlink plus
  save_cwd/fchdir/restore_cwd - is dead; its five `AT_FUNC_*` macros are kept as cfg'd
  `macro_rules!` (P-51) and the template body itself is recorded in
  `docs/ports/macro-body-notes.tsv` (the gettimeofday.c Windows-arm treatment), because
  save-cwd.h/openat.h/openat-priv.h are not available on this target.
- The three inner arms (READLINK_TRAILING_SLASH_BUG, READLINK_TRUNCATE_BUG, `__CYGWIN__`) are
  all dead here and kept under their named features, exactly as in lib/readlink.c; the file's
  only function is `rpl_readlinkat`, so no `system`/definition-name split was needed beyond
  `# undef fstatat`/`# undef readlinkat`, which the port expresses with a `system` module.
- No call-relationship differences (D7): `fstatat`, `readlinkat`, `strlen`, `memset` and
  `memcpy` appear exactly once per arm, in C's order.
