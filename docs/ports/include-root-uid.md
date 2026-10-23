# emacs/lib/root-uid.h (30 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 2
- [x] ROOT_UID  (line 27, body b6589fc6ab0d)
- [x] ROOT_UID_H_  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **Both arms are carried, C 27's first.**  `__TANDEM` is the NonStop compiler's own macro: the
  frozen table cannot see it, and `tools/check-cond-eval.py lib/root-uid.h` says so honestly
  (`2 conditions, 0 unknown, 1 disagreement`).  C 27's `0` is the arm the frozen configuration
  selects, and it is also the definition the extractor records (C's `#else` is last), so it stands
  first; C 25's `65535` follows behind `#[cfg (feature = "tandem")]` (P-51).  The extractor compares
  the macro by presence rather than body for such a pair ("1 C macros have several definitions
  (configuration branches), body not compared: ROOT_UID", its branch-macro rule), which is what
  makes both spellings tolerable at once.
- **The feature name drops the leading underscores**, the way `HAVE_ANDROID` becomes `have_android`
  (96 sites) and `WINDOWSNT` becomes `windowsnt` (79).
- **`ROOT_UID` is spelled `uid_t`.**  C's macro has no type of its own (PORTING.md 1.1); the type
  comes from the header's only user, `lib/euidaccess.c`, which compares it against `geteuid ()`, and
  that unit provides `uid_t` through its own includes (PORTING.md 0.1-9).
- **What the use-site copy said was wrong, and it is corrected with the copy's retirement.**  Before
  this mirror existed, `lib/euidaccess.rs` carried `pub const ROOT_UID: uid_t = 0;` with the
  note that "root-uid.h is two lines: `#include <sys/types.h>` and that define".  The reference's
  header has no `#include` at all, and it is 30 lines: the guard, a comment, and the `__TANDEM`
  pair.  The unit now includes the mirror at C 32-34's position and the sentence is gone.

## Evidence

- `python3 tools/extract-abi.py --check emacs/lib/root-uid.h lib/include/root-uid.rs`
  -> `OK emacs/lib/root-uid.h -> lib/include/root-uid.rs` (no `- ` problem lines), with the
  branch-macro note quoted above.
- `python3 tools/check-cond-eval.py lib/root-uid.h`
  -> `2 conditions, 0 unknown to the evaluator, 1 disagreements`.
- Consumer: `build-tu lib/euidaccess.c` -> `built lib/euidaccess.rs -> ../build/femacs/euidaccess.o`.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 0 failures; `check-globals` 283 units / 0; `check-verbatim` 6 trees
  / 4607 files / 0; `check-target-names` 0 findings.
