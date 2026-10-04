# emacs/src/emacsgtkfixed.h (60 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACSGTKFIXED_H  (line 22, body da39a3ee5e6b)

## Types and their fields — 2
- [ ] _EmacsFixed: GtkFixed container, EmacsFixedPrivate* priv
- [ ] _EmacsFixedClass: GtkFixedClass parent_class

## File-scope objects — 0

## Decisions

- **Nothing in this header is compiled on this target.**  `HAVE_PGTK` is undefined (the frozen
  table records it `undef`), so C 30-33's type macros (both records' guards) and C 52-54's
  `emacs_fixed_get_type` are dead text; `src/emacsgtkfixed.c` is not in the frozen build's source
  list, so `femacs/src/emacsgtkfixed.rs` is the empty-shape note that names the exclusion; and
  C 24's `<gtk/gtk.h>` has no mirror here - the platform target's own headers own those names
  (PORTING.md 0.1-9).  The mirror carries the header's live text anyway, because the header's
  *text* is what a mirror mirrors (the `src/w32select.h` precedent, status 3.158).
- **Two records are deferred rather than invented, and the ledger's boxes stay unchecked.**
  `struct _EmacsFixed` (C 39-45) and `struct _EmacsFixedClass` (C 47-50) each hold a GTK record
  **by value** (`GtkFixed container`, `GtkFixedClass parent_class`), and no compiler on this
  target has `<gtk/gtk.h>` to measure those records with.  The two options that would make the
  checklist green are both lies: a zero-sized stand-in (no `sizeof`) and a byte array (a size
  nobody measured) - the same reading `include-w32heap.md` records for `struct file_data` and
  `include-haikuterm.md` recorded for `haikufont_info` before `struct font` landed (status 3.154,
  3.164).  The extractor names the deferral on stderr instead, and the box is the signal.
- **`struct frame *` and `GtkWidget *` are P-55/P-72 pointers.**  C 26/35's `struct frame;` is a
  forward declaration this header does not define, and `GtkWidget` belongs to GTK, so the one live
  declaration (`emacs_fixed_new`, C 56) keeps C's arity and both pointer positions untyped with
  C's prototype in a `/// C:` line.
- **`--check` asks for less than fidelity does.**  The extractor's item list is the guard macro
  plus the two deferred records; the prototype at C 56 is a declaration, not a definition
  (`## Functions defined — 0`), so it is carried for fidelity rather than because an item asks
  for it.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/emacsgtkfixed.h femacs/src/include/emacsgtkfixed.rs`
  -> `missing type deferred: src/emacsgtkfixed.h _EmacsFixed/_EmacsFixedClass (the ledger has
  unchecked boxes)` on stderr and `OK emacs/src/emacsgtkfixed.h -> femacs/src/include/emacsgtkfixed.rs`
  (no `- ` problem lines) - i.e. the two deferrals are visible, not silent.
- `python3 tools/check-cond-eval.py src/emacsgtkfixed.h` -> `3 conditions, 0 unknown to the
  evaluator, 0 disagreements`.
- `python3 tools/extract-abi.py emacs/src/emacsgtkfixed.h` -> this ledger: 0 subrs, 0 functions
  defined, 1 macro (the guard), 2 types (both deferred), 0 file-scope objects.
- **No unit includes this mirror** (`rg -n 'include/emacsgtkfixed' femacs` -> no hits); it is
  compiled by `xtask layout-check`'s single crate like every other `src/include` mirror.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures;
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures.
