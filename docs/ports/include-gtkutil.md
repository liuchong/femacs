# emacs/src/gtkutil.h (245 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] GTKUTIL_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The whole body is dead text and is not transcribed** (PORTING.md P-08).  `USE_GTK` is `undef`
  in `docs/baseline/config/darwin-arm64.macros.tsv`, so the `#ifdef USE_GTK` at C 24 excludes
  C 25-243 from the frozen configuration; the mirror therefore carries only the guard.  The
  decidable-condition count is the evidence that this is a *decision* and not an unexamined arm:
  `src/gtkutil.h: 10 conditions, 0 unknown, 0 disagreements [VERIFIED]`.  The nested conditions
  inside the dead region (`HAVE_PGTK` at C 28/171/187/194/211/228, `USE_CAIRO` at C 218 and
  `HAVE_GTK3 && HAVE_XINPUT2` at C 224) are decided as well, so nothing here is "unknown".
- **The dead region is named in the mirror's comment and in this ledger** so a GTK target knows
  exactly what it inherits: six scroll-bar bound macros (C 38-43), two data keys (C 46/49), the
  `EVENT` alias (C 30/33), three callback records (C 52-84), the prototype block (C 86-230) and
  two flags (C 240/242).  No exclusion row is needed: nothing here is a C-syntax macro body that
  the gate compares, because the gate sees nothing (the inventory is the guard alone).
- **No `include!` lines**: the `<gtk/gtk.h>` and `../lwlib/lwlib-widget.h` includes (C 26-27) are
  inside the dead arm, so the mirror names no GTK type at all and needs no foreign declaration.
  That also makes it one of the few mirrors that compiles as a crate root on its own.
- **`src/gtkutil.c` is not in the frozen build** (docs/port-map.md 450-451, docs/status.md 3.31's
  note-only placeholder in `femacs/src/gtkutil.rs`), so no translation unit reaches this mirror
  today; the compile smoke is that placeholder.

## Evidence

- `python3 tools/check-cond-eval.py src/gtkutil.h` ->
  `src/gtkutil.h: 10 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- `python3 tools/extract-abi.py --check emacs/src/gtkutil.h femacs/src/include/gtkutil.rs`
  -> `OK emacs/src/gtkutil.h -> femacs/src/include/gtkutil.rs` (no `- ` problem lines).
- `CARGO_TARGET_DIR=/tmp/femacs-target-mir-gtkutil cargo run -q -p xtask -- build-tu src/gtkutil.c`
  -> `built src/gtkutil.rs -> ../build/femacs/gtkutil.o`.
