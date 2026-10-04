# emacs/lib/alloca.in.h (72 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

- [x] _GL_ALLOCA_H  (the guard, carried by the mirror; the extractor lists no item here)

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The extractor lists zero items, and the mirror owes none.**  `tools/extract-abi.py
  emacs/lib/alloca.in.h` reports 0 subrs, 0 functions, 0 macros, 0 types, 0 objects: the whole
  file is a conditional chain that defines the `alloca` *macro*, and a template's `@PLACEHOLDER@`
  conditions cannot be decided by the frozen table (P-66), so the item list is empty rather than
  wrong.  `--check` is therefore OK by construction, and the mirror's job is the decision, not
  items - the `lib/assert.in.h` disposition-mirror shape (status 3.157).
- **The judge is the generated header.**  `build/emacs-ref/lib/alloca.h` exists and its line 47 is
  `#  define alloca __builtin_alloca`: the frozen configuration takes the `__GNUC__`/clang arm, so
  every other arm (including the trailing `extern void *alloca (size_t)` fallback and the
  `#include_next <alloca.h>` workaround, whose `_WIN32` is undefined) is dead text here.  The
  sibling templates judge the same way (`errno.in`/`dirent.in`, status 3.159).
- **The port has no `alloca`, deliberately.**  PORTING.md P-05 and section 3.2 replace it with the
  stack pool (`sa_avail`/`sa_alloc`), and the use-site spelling is recorded in
  `docs/ports/excluded-macros.txt` (`AVAIL_ALLOCA alloca ...`).  A Rust `alloca!` in this mirror
  would be a second mechanism for the same job, which the port forbids; the mirror therefore
  carries C's arms as `/// C:` lines and says why.

## Evidence

- `python3 tools/extract-abi.py --check emacs/lib/alloca.in.h femacs/lib/include/alloca.in.rs`
  -> `OK emacs/lib/alloca.in.h -> femacs/lib/include/alloca.in.rs` (no `- ` problem lines).
- `python3 tools/extract-abi.py emacs/lib/alloca.in.h` -> this ledger: 0 items of every class.
- `grep -n 'define alloca' build/emacs-ref/lib/alloca.h` -> `47:#  define alloca
  __builtin_alloca` (and the dead arms at 49/52/54/61), i.e. the clang arm is the frozen one.
- `rg -n 'alloca' docs/ports/excluded-macros.txt` -> the `AVAIL_ALLOCA alloca ...` row records the
  use-site spelling the port uses instead.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures;
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures.
