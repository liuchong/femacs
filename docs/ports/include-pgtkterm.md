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
   ✅ **Stale, closed by the integrator in 3.474**: all four are registered on this revision -
   `femacs/xtask/Cargo.toml` 41 (`cairo_has_pdf_surface`), 42 (`cairo_has_png_functions`),
   43 (`cairo_has_ps_surface`) and 44 (`cairo_has_svg_surface`), each with its C site in the comment;
   `tools/feature-coverage.py` lists them with their sites.  Nothing to register.
3. **The GDK/GTK layouts and the records' `bool_bf` dispatch** (P-80) are owed to a GTK3 toolchain
   (Owed 2 of `docs/ports/include-gtk.md`).

4. **C 31's `<gtk/gtk.h>` edge is owed** (its comment in `femacs/src/include/pgtkterm.rs` carries
   the same text).  The three landed consumers that reach this mirror are `src/pgtkterm.rs` (C 48's
   chain - it has its own edge), `src/image.rs` (C 64's `TERM_HEADER` pgtk arm - it has its own now)
   and `src/xwidget.rs` (gtkutil.h 29's arm, gated `have_pgtk`).  Landing the edge breaks the last
   one: `include/gtk.rs`'s `GdkEvent` union lacks the four records + the `key` member this unit's
   copy carries (measured: `E0428` + `E0119` + `E0740` + `E0609` x2, arm A 53 -> 54).  The edge
   lands in the round that can write `include/gtk.rs` - `docs/ports/xwidget.md` and
   `docs/ports/include-gtkutil.md` carry the member list.

### status 3.496 (the mirror round this file took part in)

- The edge/declaration changes are recorded in `docs/status.md` 3.496: `include/dispextern.rs`'s
  `include!("xlib.rs")` edge lost its `#[cfg]` (the `xtask` dedupe gives the first edge the slot),
  `include/xlib.rs` widened `XRectangle`/`Window`/`XrmDatabase`/`XID` to
  `any (have_x_windows, have_pgtk)`, `include/pgtkterm.rs` fixed a dangling `///` (a syntax error
  that made every `have_pgtk` TU unparsable), its `pgtk_im_set_cursor_location` prototype and its
  two bare `atimer` tags, `include/font.rs` gained the `harfbuzz` arm of the `font_driver` size
  assertion (clang-measured 208/224), `include/xterm.rs`'s three INLINE bodies became `unsafe fn`,
  `include/gtk.rs` gained `GtkWidgetClass.get_preferred_height`, and `include/emacsgtkfixed.rs`
  moved `emacs_fixed_new` into a `_decls` module.  The two arm flips that were blocked now read
  **exit 0**: `src/pgtkim.rs` (`have_pgtk` + `have_gtk3`, 14,083,208-byte rlib) and
  `src/hbfont.rs` (`harfbuzz`, 14,580,952-byte rlib); `layout-check` passes and
  `tools/check-all-units.sh` is **83/0 + 83/0**.

## 2026-10-06: the first consumer of the three `bool_bf` members - the storage unit lands

`src/pgtkterm.rs`'s eighth slice (`pgtk_setup_relief_colors`, C 1843-1847) reads and writes C 440's
`relief_background_valid_p`, and a record's member cannot be declared at a use site, so the record
can no longer only *record* it: `pgtk_output` now stores P-80 ①'s storage unit - `pub bits: c_uint`
- with the member-name-dispatched accessor `pgtk_output_bits!` (read and write forms) and three
masks (`PGTK_OUTPUT_TOOLBAR_IN_HBOX` 1<<0, `PGTK_OUTPUT_TOOLBAR_IS_PACKED` 1<<1,
`PGTK_OUTPUT_RELIEF_BACKGROUND_VALID_P` 1<<2), the `glyph_bits!`/`glyph_row_bits!` shape.  C 394/395
(the two `toolbar_*` members) live in the same unit, so the record no longer carries a
"recorded, not stored" line.  **The unit's offset and the three bit positions are the port's storage
assignment, not a layout claim**: no GTK toolchain exists here (P-88 ②), which is why the record as a
whole stays `UNVERIFIED` and no `size_of`/`offset_of` assertion is written for it.  Because a mirror
changed, P-87's two legs were run: `python3 tools/check-mirror-dupes.py` -> **16 duplicate
declaration(s), 16 ratchet row(s), 0 failure(s)** and
`CARGO_TARGET_DIR=/tmp/sweep8 bash tools/check-all-units.sh` -> **`checked 83 unit(s), 0
failure(s)`** with **`checking 83 unit(s), 0 unit failure(s)`**.  The consumer's reading is in
`docs/ports/pgtkterm.md` (the eighth slice's section, Deviation 28): three arms, delta = the one
registered `cannot find function pgtk_fill_rectangle` class, 0 new warnings.
