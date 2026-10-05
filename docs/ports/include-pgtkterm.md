# emacs/src/pgtkterm.h (681 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _PGTKTERM_H_  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The whole body is dead text and is not transcribed** (PORTING.md P-08).  `HAVE_PGTK` is
  `undef` in the frozen table, so the single `#ifdef HAVE_PGTK` at C 29 excludes C 29-680; the
  mirror carries the guard only.  The condition reading is the evidence that the arm was decided
  rather than assumed: `src/pgtkterm.h: 8 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- **The five live `#include` lines (C 23-27) are not mirrored**: they are unconditional C text,
  but they introduce no name the header's live text uses (everything else is inside the guard),
  and every one of those mirrors is either already present in the port or, for `sysselect.rs`,
  unrelated to this header's declarations.  A PGTK target adds them with the region.
- **The dead region's contents are listed by name and C line range** in the mirror and here, so
  the eventual PGTK port starts from an inventory rather than from re-reading 681 lines: four
  small records, the two large display/output records, the PGTK `struct x_output`, the selection
  record, the colour and `FRAME_*`/`SELECTION_EVENT_*` macro families, `x_display_list` and the
  prototype block.  No exclusion row is needed - the gate sees none of them, because the
  inventory is the guard alone.
- **No `include!` lines**, so the mirror names no GTK type and compiles as a crate root on its
  own; that is what the two dead-by-construction mirrors (gtkutil.rs, gnutls.rs) do as well.
- **`src/pgtkterm.c` is not in the frozen build** (docs/port-map.md 511-512), so the compile smoke
  is the note-only placeholder unit `femacs/src/pgtkterm.rs` (docs/status.md 3.31).

## Evidence

- `python3 tools/check-cond-eval.py src/pgtkterm.h` ->
  `src/pgtkterm.h: 8 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- `python3 tools/extract-abi.py --check emacs/src/pgtkterm.h femacs/src/include/pgtkterm.rs`
  -> `OK emacs/src/pgtkterm.h -> femacs/src/include/pgtkterm.rs` (no `- ` problem lines).
- `CARGO_TARGET_DIR=/tmp/femacs-target-mir-pgtkterm cargo run -q -p xtask -- build-tu
  src/pgtkterm.c` -> `built src/pgtkterm.rs -> ../build/femacs/pgtkterm.o`.

# ----------------------------------------------------------------------
## Why the pgtk records are still not here (2026-10-05, the `docs/ports/image.md` pgtk row)

`docs/ports/image.md`'s configuration sweep reads eight error sites in `src/image.rs` under
`+ have_pgtk + have_imagemagick + use_cairo` and names them "the `include/pgtkterm.rs`
placeholder gap": five `FRAME_DISPLAY_INFO` bodies and `pgtk_display_info` / `pgtk_bitmap_record`
(plus the two `E0428`s from the typedefs this batch landed in `dispextern.rs`).  They are **not**
closed here, and the reason is a measurement, not a decision to defer:

```
$ ls -d /opt/homebrew/include/gtk-3.0 /opt/homebrew/include/gtk-4.0
ls: /opt/homebrew/include/gtk-3.0: No such file or directory
ls: /opt/homebrew/include/gtk-4.0: No such file or directory
$ pkg-config --exists gtk+-3.0 && echo present || echo absent
absent
```

`pgtkterm.h`'s body is one `#ifdef HAVE_PGTK` and it includes `<gtk/gtk.h>` (`cairo` *is* present -
`/opt/homebrew/include/cairo` - GTK is not), so no clang probe can read any of the five records and
no `(unit, shift)`/offset could be written as a *reading*; that is the same machine limit
`docs/ports/include-dispextern.md`'s 3.278 4 already records for `pgtk_bitmap_record`,
`pgtk_device_t`, `pgtk_touch_point`, `pgtk_display_info` and `pgtk_output`.  Their pointees are
GTK's own (`GdkSeat`, `GdkDevice`, `GdkEventSequence`, `GtkWidget`, …), which no mirror in the tree
declares, so the records cannot even be typed from what the port has.

What *is* owed and named for the PGTK round: the five records and their measurements, the
`FRAME_DISPLAY_INFO` macro this header defines (pgtkterm.h 477, `FRAME_X_OUTPUT (f)->display_info`)
and the `Display_Info` arm in `include/dispextern.rs` that waits on the record - the arm is
*absent* there (not spelled with a stand-in), so a pgtk configuration fails on the missing record
rather than on a wrong alias.

## Owed (2026-10-05, PORTING.md P-90)

- **The SDK surface moves out of the unit.**  `femacs/src/pgtkterm.rs` carries the cairo, GLib,
  GDK and GTK declarations as in-unit segments because it was the surface's first (and, at that
  moment, only) consumer.  With `gtkutil.c` as the second consumer, P-90 requires one dedicated
  mirror - `femacs/src/include/gtk.rs` with its own ledger `docs/ports/include-gtk.md`, the
  `include-xlib.rs` shape - and the in-unit segments moved into it (P-88 ⑤'s move-back).  The
  mirror is not created yet: this file's own slice was landed without it, and the move-back is
  owed to the round that creates it.  Until then the arm's readings carry the declarations twice
  (once per unit), which is harmless across crates and wrong only inside one TU.
