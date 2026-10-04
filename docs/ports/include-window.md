# emacs/src/window.h (1267 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 16
- [x] WINDOWP  (line 478)
- [x] CHECK_WINDOW  (line 484)
- [x] XWINDOW  (line 490)
- [x] wset_frame  (line 499)
- [x] wset_next  (line 505)
- [x] wset_prev  (line 511)
- [x] wset_mode_line_help_echo  (line 517)
- [x] wset_new_pixel  (line 523)
- [x] wset_vertical_scroll_bar  (line 529)
- [x] wset_horizontal_scroll_bar  (line 535)
- [x] wset_horizontal_scroll_bar_type  (line 541)
- [x] wset_cursor_type  (line 547)
- [x] wset_prev_buffers  (line 553)
- [x] wset_next_buffers  (line 559)
- [x] output_cursor_to  (line 1246)
- [x] overlay_matches_window  (line 1258)

## Macros defined — 95
- [x] CHECK_LIVE_WINDOW  (line 1202, body 3d59eda9471c)
- [x] CHECK_VALID_WINDOW  (line 1194, body 8aa266c63470)
- [x] FRAME_TO_WINDOW_PIXEL_X  (line 1082, body 949df5dcd855)
- [x] FRAME_TO_WINDOW_PIXEL_Y  (line 1086, body a6d269e18911)
- [x] MINI_NON_ONLY_WINDOW_P  (line 569, body 436227128f31)
- [x] MINI_ONLY_WINDOW_P  (line 573, body a77bfe1009ed)
- [x] MINI_WINDOW_P  (line 565, body 4d20a2e0f158)
- [x] MIN_SAFE_WINDOW_HEIGHT  (line 673, body ed6616dfd462)
- [x] MIN_SAFE_WINDOW_PIXEL_HEIGHT  (line 675, body 3333b6349eea)
- [x] MIN_SAFE_WINDOW_PIXEL_WIDTH  (line 670, body 59494adc63a8)
- [x] MIN_SAFE_WINDOW_WIDTH  (line 668, body 0902ac4af971)
- [x] USE_HORIZONTAL_SCROLL_BARS  (line 903, body 7cb6efb98ba5)
- [x] WINDOW_BOTTOMMOST_P  (line 695, body e2f327ff5153)
- [x] WINDOW_BOTTOM_DIVIDER_WIDTH  (line 712, body 2c3424de6e17)
- [x] WINDOW_BOTTOM_EDGE_LINE  (line 735, body a2a5163495fc)
- [x] WINDOW_BOTTOM_EDGE_Y  (line 803, body 55cc7576cb7e)
- [x] WINDOW_BOTTOM_PIXEL_EDGE  (line 753, body 5a55f219d044)
- [x] WINDOW_BOX_HEIGHT_NO_MODE_LINE  (line 1050, body feb4529ae4c7)
- [x] WINDOW_BOX_LEFT_EDGE_X  (line 826, body b92d1572de84)
- [x] WINDOW_BOX_LEFT_PIXEL_EDGE  (line 810, body 83c576ec19fa)
- [x] WINDOW_BOX_RIGHT_EDGE_X  (line 833, body 77e6bd6ad75a)
- [x] WINDOW_BOX_RIGHT_PIXEL_EDGE  (line 818, body 569261df29e2)
- [x] WINDOW_BOX_TEXT_HEIGHT  (line 1058, body 8bcb807cd99e)
- [x] WINDOW_BUFFER  (line 626, body 3abd021363ba)
- [x] WINDOW_BUFFER_LOCAL_VALUE  (line 633, body 8db9917c4d4a)
- [x] WINDOW_CONFIG_SCROLL_BAR_COLS  (line 934, body 9fa5f799ab36)
- [x] WINDOW_CONFIG_SCROLL_BAR_HEIGHT  (line 991, body 4be4d67f0e98)
- [x] WINDOW_CONFIG_SCROLL_BAR_LINES  (line 999, body e28657943520)
- [x] WINDOW_CONFIG_SCROLL_BAR_WIDTH  (line 926, body 42b16705058b)
- [x] WINDOW_FRAME_COLUMN_WIDTH  (line 639, body 3dc65864b776)
- [x] WINDOW_FRAME_LINE_HEIGHT  (line 643, body 5074fbaa060b)
- [x] WINDOW_FRINGES_WIDTH  (line 868, body cfba80eded2a)
- [x] WINDOW_FULL_WIDTH_P  (line 701, body cfb3e12486dc)
- [x] WINDOW_HAS_FRINGES_OUTSIDE_MARGINS  (line 872, body 6910b3430fea)
- [x] WINDOW_HAS_HORIZONTAL_SCROLL_BAR  (line 920, body 7cb6efb98ba5)
- [x] WINDOW_HAS_VERTICAL_SCROLL_BAR  (line 894, body 1e4773162384)
- [x] WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_LEFT  (line 888, body f484cec39b49)
- [x] WINDOW_HAS_VERTICAL_SCROLL_BAR_ON_RIGHT  (line 891, body 87c98d0f070a)
- [x] WINDOW_HEADER_LINE_HEIGHT  (line 1030, body d3f179a5b3cf)
- [x] WINDOW_HEADER_LINE_LINES  (line 1035, body 4466d6defa22)
- [x] WINDOW_HORIZONTAL_COMBINATION_P  (line 612, body 78854a572d84)
- [x] WINDOW_H_INCLUDED  (line 21, body da39a3ee5e6b)
- [x] WINDOW_INTERNAL_P  (line 608, body 4c2daf4a5a4c)
- [x] WINDOW_LEAF_P  (line 604, body 6188ab0e23d6)
- [x] WINDOW_LEFTMOST_P  (line 679, body 1a80056b1ad9)
- [x] WINDOW_LEFT_EDGE_COL  (line 722, body 599b70fc4b18)
- [x] WINDOW_LEFT_EDGE_X  (line 758, body a0672c1a3636)
- [x] WINDOW_LEFT_FRINGE_WIDTH  (line 858, body 353ccec5ae06)
- [x] WINDOW_LEFT_MARGIN_COLS  (line 838, body e726b233f5ca)
- [x] WINDOW_LEFT_MARGIN_WIDTH  (line 847, body 8eaaf712bd10)
- [x] WINDOW_LEFT_PIXEL_EDGE  (line 740, body 372d4f6575fe)
- [x] WINDOW_LEFT_SCROLL_BAR_AREA_WIDTH  (line 964, body 53443b038763)
- [x] WINDOW_LEFT_SCROLL_BAR_COLS  (line 944, body 0136da948d62)
- [x] WINDOW_LIVE_P  (line 1198, body 7e7de82c5ea8)
- [x] WINDOW_MARGINS_COLS  (line 842, body 12ed68bb4350)
- [x] WINDOW_MARGINS_WIDTH  (line 853, body fc5d44a45a3c)
- [x] WINDOW_MENU_BAR_P  (line 775, body 7cb6efb98ba5)
- [x] WINDOW_MODE_LINE_HEIGHT  (line 1020, body 8337a0b0cf2d)
- [x] WINDOW_MODE_LINE_LINES  (line 1025, body 57cd45d37e8d)
- [x] WINDOW_PIXEL_HEIGHT  (line 652, body 11bcdb78c737)
- [x] WINDOW_PIXEL_WIDTH  (line 648, body b450621792f9)
- [x] WINDOW_PSEUDO_P  (line 623, body dec0553b7a7e)
- [x] WINDOW_RIGHTMOST_P  (line 687, body 9df8de8438ff)
- [x] WINDOW_RIGHT_DIVIDER_WIDTH  (line 707, body 7388a8dda961)
- [x] WINDOW_RIGHT_EDGE_COL  (line 726, body 4589fc058c0e)
- [x] WINDOW_RIGHT_EDGE_X  (line 764, body f12e4abb61b8)
- [x] WINDOW_RIGHT_FRINGE_WIDTH  (line 863, body c7e009ebdcef)
- [x] WINDOW_RIGHT_MARGIN_COLS  (line 840, body 15c15f97c7d0)
- [x] WINDOW_RIGHT_MARGIN_WIDTH  (line 850, body 6c26da73d44b)
- [x] WINDOW_RIGHT_PIXEL_EDGE  (line 744, body e0c9be883cb6)
- [x] WINDOW_RIGHT_SCROLL_BAR_AREA_WIDTH  (line 970, body 09b5741ee8ae)
- [x] WINDOW_RIGHT_SCROLL_BAR_COLS  (line 952, body 58e470608739)
- [x] WINDOW_SCROLL_BAR_AREA_HEIGHT  (line 1013, body 1e78df86662e)
- [x] WINDOW_SCROLL_BAR_AREA_WIDTH  (line 976, body 3665bba3e9d5)
- [x] WINDOW_SCROLL_BAR_AREA_X  (line 983, body ad5ee41a5c19)
- [x] WINDOW_SCROLL_BAR_AREA_Y  (line 1068, body ca7b1f243660)
- [x] WINDOW_SCROLL_BAR_COLS  (line 958, body 096eed80b2de)
- [x] WINDOW_SCROLL_BAR_LINES  (line 1007, body 6cae35b93221)
- [x] WINDOW_TAB_BAR_P  (line 784, body 7cb6efb98ba5)
- [x] WINDOW_TAB_LINE_HEIGHT  (line 1040, body bf52a5eddba0)
- [x] WINDOW_TAB_LINE_LINES  (line 1045, body 6cc33651f2c2)
- [x] WINDOW_TEXT_TO_FRAME_PIXEL_X  (line 1091, body 9d32f5f43f17)
- [x] WINDOW_TOOL_BAR_P  (line 793, body 7cb6efb98ba5)
- [x] WINDOW_TOPMOST_P  (line 683, body 54ac997cee97)
- [x] WINDOW_TOP_EDGE_LINE  (line 731, body a0268806a182)
- [x] WINDOW_TOP_EDGE_Y  (line 797, body e8434abd3f09)
- [x] WINDOW_TOP_PIXEL_EDGE  (line 749, body 42b7e7dc3648)
- [x] WINDOW_TOTAL_COLS  (line 658, body eb19a365ec89)
- [x] WINDOW_TOTAL_LINES  (line 664, body 3ea6b2d399b0)
- [x] WINDOW_TO_FRAME_PIXEL_X  (line 1074, body 5368d797ade2)
- [x] WINDOW_TO_FRAME_PIXEL_Y  (line 1078, body 36e6b1ede76b)
- [x] WINDOW_VALID_P  (line 1189, body a76de69bf3cf)
- [x] WINDOW_VERTICAL_COMBINATION_P  (line 616, body 3eb2b651736d)
- [x] WINDOW_VERTICAL_SCROLL_BAR_TYPE  (line 877, body 45ccab4a23f0)
- [x] WINDOW_XFRAME  (line 620, body a364b8ee7b0b)

## Types and their fields — 2
- [x] cursor_pos: int x, int y, int hpos, int vpos
- [x] window: union vectorlike_header header, Lisp_Object frame, Lisp_Object next, Lisp_Object prev, Lisp_Object parent, Lisp_Object normal_lines, Lisp_Object normal_cols, Lisp_Object new_total, Lisp_Object new_normal, Lisp_Object new_pixel, Lisp_Object contents, Lisp_Object prev_buffers, Lisp_Object next_buffers, Lisp_Object old_buffer, Lisp_Object start, Lisp_Object pointm, Lisp_Object old_pointm, Lisp_Object temslot, Lisp_Object vertical_scroll_bar, Lisp_Object vertical_scroll_bar_type, Lisp_Object horizontal_scroll_bar, Lisp_Object horizontal_scroll_bar_type, Lisp_Object display_table, Lisp_Object dedicated, Lisp_Object combination_limit, Lisp_Object window_parameters, Lisp_Object cursor_type, Lisp_Object mode_line_help_echo, struct glyph_matrix* current_matrix, struct glyph_matrix* desired_matrix, EMACS_INT use_time, EMACS_INT sequence_number, int change_stamp, int pixel_left, int pixel_top, int left_col, int top_line, int pixel_width, int pixel_height, int old_pixel_width, int old_pixel_height, int old_body_pixel_width, int old_body_pixel_height, int total_cols, int total_lines, ptrdiff_t hscroll, ptrdiff_t min_hscroll, ptrdiff_t hscroll_whole, modiff_count last_modified, modiff_count last_overlay_modified, ptrdiff_t last_point, ptrdiff_t last_mark, ptrdiff_t base_line_number, ptrdiff_t base_line_pos, ptrdiff_t column_number_displayed, int nrows_scale_factor, int ncols_scale_factor, struct cursor_pos cursor, struct cursor_pos phys_cursor, struct cursor_pos output_cursor, int last_cursor_vpos, int left_fringe_width, int right_fringe_width, int left_margin_cols, int right_margin_cols, int scroll_bar_width, int scroll_bar_height, int mode_line_height, int header_line_height, int tab_line_height, ptrdiff_t window_end_pos, int window_end_vpos, bool_bf mini : 1, bool_bf horizontal : 1, bool_bf update_mode_line : 1, bool_bf last_had_star : 1, bool_bf start_at_line_beg : 1, bool_bf force_start : 1, bool_bf optional_new_start : 1, bool_bf phys_cursor_on_p : 1, bool_bf cursor_off_p : 1, bool_bf last_cursor_off_p : 1, bool_bf must_be_updated_p : 1, bool_bf pseudo_window_p : 1, bool_bf fringes_outside_margins : 1, bool_bf fringes_persistent : 1, bool_bf scroll_bars_persistent : 1, bool_bf window_end_valid : 1, bool_bf redisplay : 1, bool_bf suspend_auto_hscroll : 1, bool_bf preserve_vscroll_p : 1, int vscroll, ptrdiff_t window_end_bytepos

## File-scope objects — 9
- [x] extern Lisp_Object selected_window  (line 1100)
- [x] extern EMACS_INT window_select_count  (line 1106)
- [x] extern Lisp_Object minibuf_window  (line 1112)
- [x] extern Lisp_Object minibuf_selected_window  (line 1117)
- [x] extern Lisp_Object last_mouse_window  (line 1121)
- [x] extern Lisp_Object echo_area_window  (line 1146)
- [x] extern int update_mode_lines  (line 1154)
- [x] extern int windows_or_buffers_changed  (line 1161)
- [x] extern Lisp_Object Vwindow_list  (line 1206)

# ----------------------------------------------------------------------
## Decisions

- **Two types, 27 of the 122 items.**  This slice is `cursor_pos`, `struct window` and the
  16 inlines plus the nine file-scope objects; the 94 `WINDOW_*` macros are the next slice.
- **The nineteen `bool_bf : 1` flags share one 4-byte unit at offset 500.**  Measured with all
  nineteen set: `ff ff 07 00` (= 0x0007FFFF, nineteen bits).  `bool_bf` is one byte here
  (conf_post.h), so clang packs the 1-bit members into the unit the way `lisp.h`'s comment
  describes; the port stores `bits: u32` and names each member's shift.
- **Two pointer degradations (P-72 rule ③).**  `current_matrix` and `desired_matrix` are
  `struct glyph_matrix *`; `window.h` includes `dispextern.h`, so C sees the type complete,
  but the port's matrix segment is still ahead in the display chain — they carry `*mut c_void`
  with a `/// C:` note until it lands.
- **Dead members are out.**  `ephemeral_last_point` (`HAVE_TEXT_CONVERSION`) and the
  `phys_cursor_type`/`phys_cursor_width`/`phys_cursor_ascent`/`phys_cursor_height` block
  (`HAVE_WINDOW_SYSTEM`) are `#undef` in the frozen configuration, so they are not part of this
  target's 93 live members (P-73's live-line filter is what keeps them out).
- **`overlay_matches_window` compares pointers**, so the port uses `std::ptr::eq` on the two
  `*mut window` values rather than C's `==` on pointers (same comparison, Rust spelling).

# ----------------------------------------------------------------------
## Decisions (second slice)

- **39 of the 95 macros land here.**  They are the ones that read only `struct window` (or a
  Lisp_Object) — the geometry, edge, margin and predicate family.  The other 55 need types that
  are not mirrored yet and are listed below.
- **The packed flags are read through `window.bits`.**  `WINDOW_PSEUDO_P`,
  `WINDOW_HORIZONTAL_COMBINATION_P`, `WINDOW_VERTICAL_COMBINATION_P`,
  `WINDOW_HAS_FRINGES_OUTSIDE_MARGINS` and `MINI_WINDOW_P` mask the unit; the C field names
  (`pseudo_window_p`, `horizontal`, `fringes_outside_margins`, `mini`) cannot occur in Rust, so
  they are recorded in `docs/ports/macro-body-notes.tsv` with the reason.  A shared
  `window_bit (w, shift)` reader keeps the shift constants in one place.
- **Object-like macros stay constants.**  `USE_HORIZONTAL_SCROLL_BARS` has no parameter in C, so
  the port declares a `const` rather than a function (P-17's rule for value-producing macros).
- **`WINDOW_MENU_BAR_P`/`WINDOW_TAB_BAR_P`/`WINDOW_TOOL_BAR_P`/`WINDOW_HAS_HORIZONTAL_SCROLL_BAR`
  are `false`** on this target (no window system), exactly as C's macros expand.
- **Still blocked (55).**  `FRAME_TO_WINDOW_PIXEL_X/Y`, `MIN_SAFE_WINDOW_PIXEL_WIDTH/HEIGHT` and
  the whole `WINDOW_FRAME_COLUMN_WIDTH`/`_LINE_HEIGHT`/`WINDOW_*_EDGE_X`/fringe/scroll-bar-geometry
  family dereference `WINDOW_XFRAME`, i.e. `struct frame`; `WINDOW_MODE_LINE_HEIGHT`/`_LINES` and
  the header/tab-line pair need dispextern.h's `window_wants_*` and `CURRENT_*_LINE_HEIGHT`;
  `WINDOW_VERTICAL_SCROLL_BAR_TYPE` needs frame.h's `vertical_scroll_bar_none`/`_left`/`_right`.

## Integrator addition (2026-10-03, with src/insdel.c)

`window.h` 1167-1169's three redisplay entry points (`wset_redisplay`, `fset_redisplay`,
`bset_redisplay`) and 1217-1222's `enum window_body_unit` are mirrored now, in a
`window_redisplay_decls` module + glob import and a `#[repr(C)]` enum with C's implicit values.
`src/insdel.c` needed `bset_redisplay` and had declared it at its use site; the mirror is the
right owner, and the unit's own declaration still shadows the import (PORTING.md 0.1-10), which
`build-tu src/insdel.c` proves.  The mirror's `--check` still reports the next uncovered item
(`MARGIN_IN_PIXELS`), i.e. this header stays WIP with the same shape as before - the remaining
list is the tail this file documents.


# ----------------------------------------------------------------------
## Decisions (third slice, 2026-10-04: the geometry family)

- **All 95 macros land; this header's checklist is complete.**  The 55 that were open are
  mirrored now, in C's order and with C's names: `WINDOW_XFRAME` (620), the frame metrics
  (639/643), `MIN_SAFE_WINDOW_PIXEL_WIDTH`/`_HEIGHT` (670/675), the outermost/full-width
  predicates (687-703), the divider widths (707/712), the four `WINDOW_*_EDGE_X`/`_Y`
  (758-803), the box edges (810-835), the margin/fringe widths (847-869), the vertical
  scroll-bar type and predicates (877-901), the scroll-bar widths/heights/areas (926-1016),
  the mode/header/tab-line heights and their `LINES` predicates (1020-1046), the box heights
  and the horizontal-scroll-bar start (1050-1072) and the pixel conversions (1074-1092).
  `enum margin_unit` (1225) lands with them; the checker counts its two members as the
  constants `MARGIN_IN_LINES`/`MARGIN_IN_PIXELS` it was still reporting.  The 9 file-scope
  objects and `WINDOW_H_INCLUDED` were already mirrored by the first slice - their boxes were
  stale and are ticked here.
  `python3 tools/extract-abi.py --check emacs/src/window.h femacs/src/include/window.rs` = `OK`.

- **They keep C's `macro_rules!` shape, unlike the first 39 accessors.**  A macro body is
  resolved where it is *expanded*, so it may name frame.h's `struct frame`/`FRAME_*` and
  dispextern.h's `CURRENT_*_LINE_HEIGHT`/`window_box_left` exactly as C's text does, and only
  in the units that reached those headers; a function body would have to compile in every unit
  that includes this mirror, and 7 of the 44 units that include it never reach frame.h
  (bytecode.rs, callint.rs, editfns.rs, insdel.rs, json.rs, marker.rs, textprop.rs) - the
  mirror's first attempt as functions failed with `E0425: cannot find type frame` in exactly
  those units (PORTING.md P-72 rules ①/④, the hazard `include/dispextern.rs`'s
  `MATRIX_BOTTOM_TEXT_ROW` records).  Keeping C's shape also keeps C's evaluation count, which
  a function would fold to one mention (P-16).  The cost is that a call site must be `unsafe`
  (the `window_wants_*` declarations are `unsafe extern "C"`), exactly as C's call sites are.

- **`WINDOW_FRAME` (frame.h 929) is named at this use point.**  frame.h's mirror has not
  reached it (`docs/ports/include-frame.md` lists it open), so the port keeps C's construct
  where `WINDOW_XFRAME` needs it, under C's own name and body (`((w)->frame)`, a *place*, so a
  macro).  When frame.h's lane lands it in `include/frame.rs`, this copy moves there (P-26).

- **The dispextern.h family the line heights need is that header's mirror's now.**  The five
  `WINDOW_*_LINE_HEIGHT`/`WINDOW_BOX_*` macros call `CURRENT_{MODE,HEADER,TAB}_LINE_HEIGHT`
  (dispextern.h 1575/1616/1650), which call the `MATRIX_*_LINE_HEIGHT` readers (1513/1529/1538)
  and `estimate_mode_line_height` (3582).  Those twelve macros were duplicated at four use
  sites (window.rs, dispnew.rs, menu.rs and the in-flight xdisp.rs); they are
  `include/dispextern.rs`'s now (its ledger's boxes are ticked with this change) and the three
  committed copies are deleted - xdisp.rs's is its own lane's to drop.  The externs they call
  (`window_wants_*` = window.h 1213-1215, `estimate_mode_line_height` = dispextern.h 3582) stay
  the units' use-site declarations; `window_wants_*` is the mirror's now, in a module + glob
  import because window.rs *defines* them (PORTING.md 0.1-10, the `window_redisplay_decls`
  shape).

- **Use-site copies deleted (P-26, "one construct, one landing point").**  window.rs (the whole
  geometry block, the dispextern.h family and `margin_unit` + its two members), dispnew.rs (the
  same two families and a function-shaped `WINDOW_RIGHTMOST_P`), menu.rs (four geometry macros,
  the line-height family, two `window_wants_*` declarations), xfaces.rs and frame.rs
  (`WINDOW_FRAME`/`WINDOW_XFRAME`), indent.rs and minibuf.rs (`WINDOW_FRAME`), term.rs (six
  geometry functions).  Call sites keep C's text; the ones that had been spelled as function
  calls (term.rs, frame.rs and dispnew.rs's `WINDOW_RIGHTMOST_P`) now carry the macro's `!`.
  `term.rs`'s `WINDOW_LEFT_EDGE_X`/`WINDOW_TOP_EDGE_Y` copies did **not** match C's bodies (they
  divided the pixel edge by the frame's column width/line height, i.e. returned canonical
  columns); deleting them restores C's frame-pixel values at the two call sites in
  `tty_draw_row_with_mouse_face` - a behaviour change the integrator should know about.

- **Still open (10 boxes, all satisfied in the mirror but not in this generated list before
  now):** none - see the first bullet.  The header's remaining work is outside its own item
  list: `editfns.rs`, `insdel.rs` and `textprop.rs` include this mirror without
  `include/dispextern.rs`, so the five `WINDOW_*_LINE_HEIGHT` macros cannot expand there even
  though C's `window.h` includes `dispextern.h` and would have given them the text.

### Evidence

- `python3 tools/extract-abi.py --check emacs/src/window.h femacs/src/include/window.rs` -> `OK`
  (was 55 missing macros + 2 missing constants).
- `python3 tools/extract-abi.py --check emacs/src/window.c femacs/src/window.rs` -> `OK` after
  the deletions (the unit's own ledger row for the 493 removed lines is in
  docs/ports/line-drift-tolerance.tsv).
- `python3 tools/extract-abi.py --check emacs/src/dispextern.h femacs/src/include/dispextern.rs`
  -> still `FAIL` for its own open items, 12 fewer than before (153 -> 141), none of them the
  twelve this change moved.
- `CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask -- build-tu <unit>.c`: all 44
  units that include this mirror build, `src/window.c`, `src/dispnew.c` and `src/indent.c`
  included; `src/process.c` fails on a pre-existing `FD_SET`/`FD_CLR`/`FD_ZERO` duplicate
  (`lib/include/sys_select.rs` vs `process.rs`), unrelated to this change.
- `python3 tools/extract-abi.py --check emacs/src/window.h ...` and the three repo-wide checks
  are recorded in the lane's report.

## The window-system four (status 3.275)

`struct window`'s window-system members were filtered out of the first pass by the field scanner's
live-line filter (P-73, which reads the frozen `config.h`): C declares `phys_cursor_type`,
`phys_cursor_width`, `phys_cursor_ascent` and `phys_cursor_height` under
`#ifdef HAVE_WINDOW_SYSTEM` (window.h 339-351).  They are carried now, gated with the feature.

The reading (`spikes/gated-layout-probe.py 'struct window' window.h`, P-82;
`docs/baseline/window-layout.tsv`) is what makes this a one-line-per-member addition rather than a
layout question: frozen 520/8, x11 536/8, ns_cocoa 536/8 - the two window-system arms add exactly
those four bytes and **none of the flag run moves** (`docs/baseline/window-bits.tsv`: 19 flag
members, identical `(unit, shift)` in all three arms), so no numbering is gated here.  The four sit
at 452/456/460/464, i.e. right after `last_cursor_vpos`.

Evidence: `build-tu src/window.c` exit 0; `xtask layout-check` passed; the mirror now answers the
probe with no missing member (`struct window`).

## Decisions (the 29 function prototypes, status 3.325 item 55)

- **window.h's function prototypes are the mirror's now.**  The header declares 38 `extern`
  functions; 9 were already carried and **29 were not**, so thirteen units declared the ones they
  called at their own use sites (34 declarations).  The 29 land in `window_fn_decls`, a module
  with a glob import, in the same shape `window_redisplay_decls` (C 1167-1169) already uses:
  nine of them are **defined** in this tree (`src/window.rs` defines `make_window`,
  `window_from_coordinates`, `decode_live_window`, `window_point`, `window_list`,
  `mru_rooted_frame`, `init_window_once`, `init_window` and `syms_of_window`), and a definition at
  the unit's crate root would collide with a crate-root declaration (E0428, PORTING.md 0.1-10);
  as a glob-imported item the unit's definition simply shadows it.
- **Parameter types degrade per P-72 rule 3 where the mirror cannot name the pointee.**  `struct
  frame *` and `struct buffer *` are `*mut c_void` - the spelling `fset_redisplay`/`bset_redisplay`
  in `window_redisplay_decls` already uses, because this mirror includes neither `frame.rs` nor a
  `buffer` mirror.  Four declarations take a degraded pointer (`window_from_coordinates`,
  `resize_frame_windows`, `mru_rooted_frame`, `bset_update_mode_line`); the nine callers in the
  units cast (`f as *mut c_void`) - C's own pointer, in the port's spelling.  Everything else is
  spelled with types the mirror has: `*mut window`, `Lisp_Object`, `c_int`, `bool`, `ptrdiff_t`,
  `window_part`, `window_body_unit`.
- **26 use-site declarations removed** (the units that only *declare* the name; P-26):
  `buffer.rs` 4, `frame.rs` 4, `emacs.rs` 3, `indent.rs` 3, `bytecode.rs` 2, `keyboard.rs` 2,
  `minibuf.rs` 2, `window.rs` 2, and one each in `composite.rs`, `dispnew.rs`, `font.rs`,
  `process.rs`.  Each unit's comment now says the name is the mirror's.
- **8 declarations remain, in `src/xdisp.rs`** (`decode_live_window`, `grow_mini_window`,
  `shrink_mini_window`, `window_list`, `window_outdated`, `window_parameter`, `window_point`,
  `wset_buffer`): that unit is another lane's in-flight file, so the sweep stopped at its border.
  `check-mirror-dupes` reports exactly those eight as `FAIL` until its lane drops them; the rows
  are listed in the batch report for routing.
- **Two prototypes have no caller anywhere in the tree** (`window_relative_x_coord`,
  `mark_window_cursors_off`) - they are the header's text, and the mirror carries them so the
  header's list is complete rather than "the names some unit happened to need".

## Evidence (the 29 function prototypes)

- **The count**: a script over `emacs/src/window.h`'s `extern … (…);` lines reads **38 function
  prototypes**, of which **29** did not occur in `femacs/src/include/window.rs`; the other 9
  (`wset_redisplay`, `fset_redisplay`, `bset_redisplay`, `window_wants_mode_line`,
  `window_wants_header_line`, `window_wants_tab_line`, `window_body_width`, `window_body_height`,
  `window_scroll_margin`) were already there.
- **`python3 tools/extract-abi.py --check emacs/src/window.h femacs/src/include/window.rs`** ->
  `OK emacs/src/window.h -> femacs/src/include/window.rs`.
- **`build-tu`**: `src/window.c`, `src/frame.rs`, `src/keyboard.rs`, `src/buffer.rs`,
  `src/process.rs`, `src/bytecode.rs`, `src/composite.rs`, `src/dispnew.rs`, `src/emacs.rs`,
  `src/font.rs`, `src/indent.rs`, `src/minibuf.rs` - all `built …`.
- **`xtask layout-check`** -> `layout-check passed (see build/femacs/layout_asserts.rs)`.
- **`python3 tools/check-citations.py`** -> `0 drift, 0 range, 0 stale` (the new `/// C: window.h
  <line>:` lines are checked one by one; every line number is the header's own).
- **`python3 tools/check-mirror-dupes.py`** -> `26 duplicate declaration(s), 18 ratchet row(s),
  8 failure(s)`: the 18 rows are the previous batch's legal two-spelling cases, and the 8 failures
  are the `xdisp.rs` declarations above (this batch's only new hits).
