# emacs/nt/inc/pwd.h (29 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _PWD_H_  (line 2, body da39a3ee5e6b)

## Types and their fields — 1
- [x] passwd: char* pw_name, char* pw_passwd, unsigned pw_uid, unsigned pw_gid, int pw_quota, char* pw_gecos, char* pw_dir, char* pw_shell

## File-scope objects — 0
## Decisions

Mirror of nt/inc/pwd.h, complete for this target: the include guard, `struct passwd` with C's
eight members in order, the `uid_t`/`gid_t` typedefs and the four lookups.

**Which arm**: the file has no `#if`, and it is not on
`docs/baseline/config/cond-eval-verified.txt`, so the extractor resolves nothing in it - the
whole file is live text.

**Consumers**: this tree's own `nt/inc/grp.h` includes it for `gid_t` (C's comment on that
include says so), so `include/grp.rs` pulls in this mirror.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead, and `src/fileio.c`, `src/dired.c` and `src/w32.c` are
among the units whose `#include <pwd.h>` resolves here.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/pwd.h
nt/inc/include/pwd.rs` -> `OK`.
