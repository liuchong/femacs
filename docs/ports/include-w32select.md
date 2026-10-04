# emacs/src/w32select.h (28 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] W32SELECT_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header itself has no platform guard, so its declarations are live text.**  C 22's
  `#include <windows.h>` is the only platform dependency, and the header never mentions a name from
  it; the mirror therefore names none either (PORTING.md 0.1-9: a unit that needs those names on the
  Windows target takes them from that target's headers).  C 24-26's three entry points are carried
  as `unsafe extern "C"` declarations with a `/// C:` line each, because they are the file's live
  content even though nothing on this target links them.
- **The unit is an empty shape and the mirror is not, and both are right.**  The frozen
  configuration does not compile `src/w32select.c` (docs/baseline/frozen-build-sources.txt: this
  target's platform object lists are empty), so `femacs/src/w32select.rs` is the recorded note that
  names the exclusion; the *header's* text is unconditional, so the mirror carries its declarations
  and their definitions are deferred to the Windows target.  Two different files, two different
  answers - which is why the extractor's per-pair reading is per-pair.
- **`--check` asks for less than fidelity does, and the mirror keeps the difference visible.**  The
  extractor's item list for this header is the guard alone (`## Functions defined — 0` counts
  definitions, not declarations), so a mirror holding only `W32SELECT_H` would already check OK;
  the three declarations are here because C's live text has them, and this paragraph is what keeps
  that from looking like a required item later.
- No unit includes this mirror on this target: `emacs.c` reaches the header inside `#ifdef
  WINDOWSNT` (dead text, recorded in its own ledger), and the two other includers (`w32.c`,
  `w32select.c`) are not compiled here at all.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/w32select.h femacs/src/include/w32select.rs`
  -> `OK emacs/src/w32select.h -> femacs/src/include/w32select.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/w32select.h`
  -> `1 conditions, 0 unknown to the evaluator, 0 disagreements` (that condition is the guard).
- `python3 tools/extract-abi.py emacs/src/w32select.h` -> this ledger: 0 subrs, 0 functions,
  1 macro (the guard), 0 types, 0 file-scope objects.
- `rg -n "syms_of_w32select|globals_of_w32select|term_w32select" femacs/src` -> only this mirror,
  so there is no use-site copy to retire (unlike the 3.143/3.153 cases).
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 0 failures; `check-globals` 283 units / 0; `check-verbatim` 6 trees
  / 4607 files / 0; `check-target-names` 0 findings.
