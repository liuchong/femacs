# emacs/lib/cloexec.c (83 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] set_cloexec_flag  (line 39)
- [x] dup_cloexec  (line 80)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

# emacs/lib/cloexec.c (83 lines) — port `lib/cloexec.rs`

## Decisions

- **F_SETFD is defined** by the frozen target's <fcntl.h>, so C 39-62's arm (the
  `fcntl (desc, F_GETFD, 0)` dance) is the live text and C 64-79's `#else` arm (the dup2 punt)
  is dead.  Both arms live *inside* one `set_cloexec_flag` as `#[cfg (…)]` blocks (the malloc.c
  shape): the ledger checks a function's tokens as one body, so a separate cfg'd function would
  have left `dup2`/`EBADF`/`errno` unaccounted for.
- `cloexec.h` has a mirror (`lib/include/cloexec.rs`) carrying the two prototypes and the
  `#include "cloexec.h"` they satisfy.  `dup2` comes from the unistd mirror, the `F_*` values
  from the fcntl mirror.
- No call-relationship differences (D7): `fcntl` is called twice in the live arm and once in
  `dup_cloexec`, which is C's count.
