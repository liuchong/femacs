# emacs/nt/inc/grp.h (36 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _GRP_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 1
- [x] group: char* gr_name, gid_t gr_gid

## File-scope objects — 0
## Decisions

Mirror of nt/inc/grp.h, complete for this target: the include guard, `struct group` with C's
two fields, and the three prototypes (`getgid`, `getegid`, `getgrgid`).

**Which arm**: the file has no `#if`, and it is not on
`docs/baseline/config/cond-eval-verified.txt`, so the extractor resolves nothing in it - the
whole file is live text.

**Includes**: C 23's `#include <pwd.h>` resolves to this tree's own `nt/inc/pwd.h` in a MinGW
build, so the mirror pulls in `include/pwd.rs` - which is where `gid_t` comes from, exactly as
C's comment says.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/grp.h
femacs/nt/inc/include/grp.rs` -> `OK`.
