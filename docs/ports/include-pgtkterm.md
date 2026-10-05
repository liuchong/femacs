# emacs/src/pgtkterm.h (681 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] _PGTKTERM_H_  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The body is transcribed, both arms** (PORTING.md P-86 ① - the P-08 "dead text, guard only"
  decision this ledger carried until the P-90 round is withdrawn).  The header's body is one
  `#ifdef HAVE_PGTK` (C 29-680), so every item below carries `#[cfg (feature = "have_pgtk")]` and
  the empty `#else` arm is C's own: the frozen configuration compiles nothing of it, and
  `src/pgtkterm.c`'s unit - which is not in the frozen build either - is what would.
  `tools/check-cond-eval.py src/pgtkterm.h` reads `8 conditions, 0 unknown, 0 disagreements
  [VERIFIED]`, so every condition the transcription crosses is *decided*.
- **What the mirror now declares** (all of it gated `have_pgtk`): `pgtk_bitmap_record` (C 43-49),
  `pgtk_device_t` (51-64), `pgtk_touch_point` (66-83), the six colour macros (85-91),
  **`struct scroll_bar`** (93-144, this header's own tag - `xterm.h`'s same-named record lives in
  `include/xterm.rs` and the two never share a TU, because `gtkutil.c` takes `xterm.h` under
  `#ifndef HAVE_PGTK` and this header under `#else`), **`pgtk_display_info`** (146-281) with its
  two anonymous structs named `pgtk_display_info_im`/`pgtk_display_info_scroll` (P-67),
  `x_display_list` (284), **`pgtk_output`** (286-455) with the nested `relief` record (427-432),
  the pgtk `struct x_output` (456-459), the anonymous `FOCUS_*` enumeration's three constants
  (461-471), the ~33 `FRAME_*`/`DEFAULT_GDK_DISPLAY`/`XSCROLL_BAR`/`BLACK_PIX_DEFAULT` accessors
  (474-529), `selection_input_event` (539-547), `SELECTION_EVENT_DISPLAY` (a body, because C's is
  `INLINE`) and the six `SELECTION_EVENT_*` macros (550-567), and the whole prototype block
  (569-679).
- **The five `#include` lines at C 23-27 are mirrored as edges** (`dispextern.rs`, `frame.rs`,
  `character.rs`, `font.rs`, `sysselect.rs`), together with the three mirrors the body's own names
  need (`lisp.rs` for `Lisp_Object`/`vectorlike_header`/`ptrdiff_t`, `systime.rs` for `Time`,
  `termhooks.rs` for `event_kind`/`EVENT_KIND_WIDTH`) and `xlib.rs` for `<X11/Xresource.h>`'s
  `XrmDatabase`.  C 31's `<gtk/gtk.h>` is the edge to the surface's mirror, `include/gtk.rs`
  (P-90 ②: at the referencing header's own `#include` line).
- **The prototypes and `x_display_list` are in a `pub mod` with a glob import** (`declarations`,
  `prototypes`, then `use …::*;`): C's "prototype in the header + definition in the .c" is
  `E0428` in Rust when both land in one TU, and PORTING.md 0.1-10's measured remedy is the glob -
  a definition in the unit shadows the glob-imported declaration (`src/pgtkterm.rs` defines
  `x_display_list` and twelve `pgtk_*` entry points).  Measured: without the module wrap the
  unit's arm read 13 `E0428`s; with it, 0.
- **`ENUM_BF (event_kind) kind : EVENT_KIND_WIDTH`** (`selection_input_event`'s first member) is
  carried as the storage unit `include/termhooks.rs`'s `input_event` uses for the same bit-field
  (P-80's member shape, `bits: c_uint`), because a Rust struct cannot carry a bit-field.
- **The pgtk union member is spelled once**: C 154-161's *anonymous*
  `union { GdkDisplay *gdpy; GdkDisplay *display; };` has two names for one pointer at offset 0,
  and Rust cannot overlap two named fields, so `pgtk_display_info` keeps C's **first** member's
  name (`gdpy`) and records the alias; the mirror's own `FRAME_X_DISPLAY` uses `.gdpy`, and a
  consumer writing C's `->display` spells `.gdpy` (a recorded deviation).
- **Every GDK/GTK record's size and offset is UNVERIFIED** (P-88 ②): no GTK tree exists here, so
  no layout assertion is written for `pgtk_bitmap_record`, `pgtk_device_t`,
  `pgtk_touch_point`, `scroll_bar`, `pgtk_display_info`, `pgtk_output`, `relief`,
  `x_output` or `selection_input_event`, and each cites the header alone (no line).  The four
  `bool_bf` members (`toolbar_in_hbox`, `toolbar_is_packed`, `relief_background_valid_p`) are
  *recorded* rather than stored, the `include/xterm.rs` 752-754 shape (P-80's member dispatch).
- **Both arms of the three cairo-surface probes (C 33-41) are carried** as C's text and comments;
  the four `CAIRO_HAS_*` features are unregistered in `femacs/xtask/Cargo.toml` and no mirror
  exists for `<cairo-pdf.h>`/`<cairo-ps.h>`/`<cairo-svg.h>` (Owed 2).

## Evidence

- `python3 tools/check-cond-eval.py src/pgtkterm.h` ->
  `src/pgtkterm.h: 8 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- `cd femacs && CARGO_TARGET_DIR=/tmp/gtk3lane/cargo cargo run -q -p xtask -- build-tu
  src/pgtkterm.c` -> `built src/pgtkterm.rs -> ../build/femacs/pgtkterm.o` (336 bytes, the frozen
  empty object), exit 0.
- **The unit's arm reading (P-86 ③⑵'s `text-only` shape; no GTK tree, so no arm can be
  *compiled*)**, on the materialised `build/femacs/units/pgtkterm/_root.rs` with
  `use_gtk + have_pgtk + have_window_system + have_gtk3 + use_cairo + have_xinput2`:
  **before this round 158 errors (147 in the unit + 11 in mirrors); after it 18 (2 + 16)** - the
  143 `pgtk_output`/`pgtk_display_info`/`pgtk_device_t` "cannot find type" errors are gone, and
  the two that remain (`pgtk_new_focus_frame`, `pgtk_unlink_touch_points`) are this unit's own
  functions the file defines at C 900+/1400+, which its slice has not carried yet.
  `src/pgtkfns.rs`'s reading is unchanged at 68 (57 + 11): its segment and the records' edge are
  its own lane's Owed 1/3.
- **The layout round (2026-10-05)** touched only the GTK mirror (`include/gtk.rs`: the P-55
  stand-ins `c_ulong`/`*mut c_void` and the `have_gtk3` gate on
  `gtk_style_context_remove_provider`) plus two inline casts in `src/gtkutil.rs`
  (`docs/ports/include-gtk.md` `## Evidence`); nothing in *this* header changed and its arm reading
  re-measured the same: **19 -> 19** (2 in the unit + 16 in mirrors + rustc's `aborting` row - the
  ledger's `18` counts that row out, the ④ round's table).

## Owed

1. **`src/pgtkterm.rs` still declares its `FRAME_*` copies of `frame.h`'s accessors**
   (`FRAME_COLUMN_WIDTH`, `FRAME_LINE_HEIGHT`, `FRAME_TAB_BAR_HEIGHT`,
   `FRAME_CONFIG_SCROLL_BAR_COLS/LINES`, `FRAME_NO_FOCUS_ON_MAP`, `FRAME_NO_ACCEPT_FOCUS`) - C
   *writes* them here, and `include/frame.rs` carries only the getter arm, so the two-arm spelling
   has no home there yet.  The nine `FRAME_*` macros this header owns *did* move (this round).
2. **The four `cairo_has_*` feature names** (C 33-41) need `docs/baseline/feature-coverage.tsv`
   rows, and the three cairo `<cairo-*.h>` headers have no mirror (nothing in the port's graph
   takes a name from them).
3. **The GDK/GTK layouts and the records' `bool_bf` dispatch** (P-80) are owed to a GTK3 toolchain
   (Owed 2 of `docs/ports/include-gtk.md`).
