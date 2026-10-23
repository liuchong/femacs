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
  note-only placeholder in `src/gtkutil.rs`), so no translation unit reaches this mirror
  today; the compile smoke is that placeholder.

## Evidence

- `python3 tools/check-cond-eval.py src/gtkutil.h` ->
  `src/gtkutil.h: 10 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- `python3 tools/extract-abi.py --check emacs/src/gtkutil.h src/include/gtkutil.rs`
  -> `OK emacs/src/gtkutil.h -> src/include/gtkutil.rs` (no `- ` problem lines).
- `CARGO_TARGET_DIR=/tmp/femacs-target-mir-gtkutil cargo run -q -p xtask -- build-tu src/gtkutil.c`
  -> `built src/gtkutil.rs -> ../build/femacs/gtkutil.o`.

## Owed (2026-10-05, PORTING.md P-90)

- **The SDK surface moves out of the unit.**  `src/pgtkterm.rs` carries the cairo, GLib,
  GDK and GTK declarations as in-unit segments because it was the surface's first (and, at that
  moment, only) consumer.  With `gtkutil.c` as the second consumer, P-90 requires one dedicated
  mirror - `src/include/gtk.rs` with its own ledger `docs/ports/include-gtk.md`, the
  `include-xlib.rs` shape - and the in-unit segments moved into it (P-88 ⑤'s move-back).  The
  mirror is not created yet: this file's own slice was landed without it, and the move-back is
  owed to the round that creates it.  Until then the arm's readings carry the declarations twice
  (once per unit), which is harmless across crates and wrong only inside one TU.

## The C 26 edge's measured blocker set (2026-10-05)

The P-90 round left C 26's `<gtk/gtk.h>` edge owed, naming `src/pgtkfns.rs`'s segment as the
blocker.  Measured with the edge in place (`#[cfg (feature = "use_gtk")] include!("gtk.rs");` after
the guard), `python3 tools/check-mirror-dupes.py` reads **35 findings with no ratchet row** - the
fn/const/macro names this edge would duplicate - plus the `E0428`s its item-namespace names add:

| consumer | tracked hits | what it carries | what it needs |
| --- | --- | --- | --- |
| `src/pgtkfns.rs` | 13 | the C 28 third-party segment (32 names) | **moved in this round** through its own stand-in edge (P-88 ⑤); the unit's edge drops when C 26's lands |
| `src/xwidget.rs` | 21 | the C 30 segment (59 names) | blocked by `include/gtk.rs`'s `GdkEvent` union (4 records + the `key` member) - `docs/ports/xwidget.md` |
| `src/xfns.rs` | 1 (+2 structs) | `GtkWidget`, `gtk_widget_set_app_paintable`, and `GtkWidgetClass` with `style_updated` | the mirror's `GtkWidgetClass` must carry `style_updated` (it carries `get_preferred_width`, gtkutil.c's member) - a change **inside** `include/gtk.rs` |

So the edge cannot land from a lane whose scope is `include/gtkutil.rs` + the three units: two of
its consumers need declarations **inside `include/gtk.rs`** (the mirror's own in-flight change -
`src/xgselect.rs`'s four `gdk_x11_*` gates, status 3.477 - landed at `bbb378d` while this round
ran, so that file is free for the next one).  The three-step fix, for the round that
can write that mirror: (1) `GtkWidgetClass` grows `style_updated`; (2) its `GdkEvent` union grows
`button`/`scroll`/`motion`/`crossing`/`key` (with those four records moved in from
`src/xwidget.rs`); (3) then `src/xfns.rs`, `src/xwidget.rs` and `src/pgtkfns.rs` drop their copies
and both stand-in edges come back here.  C 27's `lwlib-widget.h` mirror and C 28-34's two
terminal-header edges are separate and still owed.
