// femacs: port of src/pgtkterm.h @ fd7afced403.
//
// Header mirror of the dead-by-construction class (the include/gnutls.rs and
// include/arg-nonnull.rs shape): the header's body is one `#ifdef HAVE_PGTK` (C 29 at the top,
// `#endif /* HAVE_PGTK */` at C 680), and `HAVE_PGTK` is `undef` in the frozen configuration
// (docs/baseline/config/darwin-arm64.macros.tsv), so C 29-680 carries no compilation on this
// target and P-08 records it instead of transcribing it.  `tools/check-cond-eval.py
// src/pgtkterm.h` reports `8 conditions, 0 unknown, 0 disagreements [VERIFIED]`: the guard
// (C 20), the body's `#ifdef HAVE_PGTK`, the three `CAIRO_HAS_*_SURFACE` probes (C 33/36/39),
// the two toolkit-scroll-bar tests (C 132/137) and `USE_CAIRO` (C 415) are all decided, and the
// outer one is false.  The live inventory is the guard alone - `tools/extract-abi.py` reports
// 1 macro / 0 types / 0 objects - which is what this mirror carries.
//
// The five `#include` lines at C 23-27 (`dispextern.h`, `frame.h`, `character.h`, `font.h`,
// `sysselect.h`) are live, but they declare no name the live text of this header uses, and no C
// declaration outside the `HAVE_PGTK` guard names a PGTK type; a PGTK target adds them together
// with the region below.
//
// What the dead region holds, named so the ledger can be re-derived from the reference: the four
// records `pgtk_bitmap_record` (C 43-49), `pgtk_device_t` (C 51-64), `pgtk_touch_point`
// (C 66-83) and `scroll_bar` (C 93-144, this header's own tag), the two big records
// `pgtk_display_info` (C 146-283) and `pgtk_output` (C 286-455), the PGTK-side `struct x_output`
// (C 456-473), `struct selection_input_event` (C 539-554), the six colour macros (C 85-91), the
// roughly fifty `FRAME_*` accessors (C 474-530, including `FRAME_X_OUTPUT`, `FRAME_FONTSET`,
// `BLACK_PIX_DEFAULT`, `WHITE_PIX_DEFAULT` and `XSCROLL_BAR`), the seven `SELECTION_EVENT_*`
// macros (C 555-566), `x_display_list` (C 284) and the prototype block (C 569-679).  Their
// pointees are GTK's (`GdkSeat`, `GdkDevice`, `GdkEventSequence`, `cairo_pattern_t`,
// `GdkEventKey`, `GdkEventProperty`, `GdkEventSelection`, `gchar`), which no mirror in the tree
// declares, so the region cannot be typed from what the port has.
//
// src/pgtkterm.c is not in the frozen build (docs/port-map.md 511-512), so no translation unit
// reaches this mirror until a PGTK target is frozen; the record of what the port owes there lives
// in docs/ports/include-pgtkterm.md.

// pgtkterm.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _PGTKTERM_H_: bool = true;
