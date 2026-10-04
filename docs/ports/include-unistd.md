# emacs/nt/inc/unistd.h (41 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 8
- [x] AT_EACCESS  (line 30, body 1b6453892473)
- [x] AT_EMPTY_PATH  (line 32, body b48f6f6773aa)
- [x] AT_FDCWD  (line 29, body c999ee6dae0f)
- [x] AT_SYMLINK_NOFOLLOW  (line 31, body 6124cb94c33d)
- [x] O_IGNORE_CTTY  (line 34, body b6589fc6ab0d)
- [x] O_NOCTTY  (line 35, body b6589fc6ab0d)
- [x] O_NOFOLLOW  (line 36, body b6589fc6ab0d)
- [x] _UNISTD_H  (line 4, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0
## Decisions

Mirror of nt/inc/unistd.h, complete for this target: the include guard, the four `AT_*` and three
`O_*` constants, and the nine prototypes in `unistd_decls`.

**Which arm**: the file has no `#if`, and it is not on
`docs/baseline/config/cond-eval-verified.txt`, so the extractor resolves nothing in it - the whole
file is live text.

**Includes**: C 6's `#include <sys/types.h>` is folded into the port's `<sys/types.h>` mirror
(`lib/include/sys_types.in.rs`, which is where `pid_t` comes from - the same choice
`nt/inc/sys/include/acl.rs` records), and C 7's `#include <pwd.h>` resolves to this tree's own
`nt/inc/pwd.h`, so the mirror pulls in `include/pwd.rs`.  The frozen target's `<unistd.h>` mirror
is deliberately *not* pulled in: it is darwin's header rather than MinGW's, and it defines the same
`AT_*`/`O_*` names this file does (the `nt/inc/sys/include/time.rs` decision for `<sys/time.h>`,
the defect status 3.103 records).

**Names with no mirror**: `ssize_t` and `size_t` (the `readlink`/`readlinkat` prototypes) are not in
the `<sys/types.h>` mirror, so they stay spelled the way C spells them and are named from the
including unit (`acl.rs`'s `ssize_t` shape); `pid_t` comes from the included `sys_types.in.rs`.

**Ledger name**: `include-unistd.md`, not `unistd.md` - that name belongs to `emacs/lib/unistd.c`
(this round found `docs/ports/unistd.md` already owning it, and did not touch it).  The stem's first
candidate is free, so the reference-file-owning rule of status 3.102/3.103 is satisfied without
falling back to the third candidate.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/unistd.h
femacs/nt/inc/include/unistd.rs` -> `OK`.
