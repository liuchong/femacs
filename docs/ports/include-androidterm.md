# emacs/src/androidterm.h (496 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 19
- [x] BLACK_PIX_DEFAULT  (line 325, body b6589fc6ab0d)
- [x] BLUE_FROM_ULONG  (line 492, body b6010f68ba0f)
- [x] FRAME_ANDROID_COMPLETE_P  (line 311, body 50d3fe67910d)
- [x] FRAME_ANDROID_DRAWABLE  (line 305, body ab16118517ab)
- [x] FRAME_ANDROID_NEED_BUFFER_FLIP  (line 298, body 9e07fafd81e2)
- [x] FRAME_ANDROID_OUTPUT  (line 290, body cac7664d9d95)
- [x] FRAME_ANDROID_WINDOW  (line 294, body 6b5f7a625246)
- [x] FRAME_BASELINE_OFFSET  (line 317, body e2bc110deb3b)
- [x] FRAME_DISPLAY_INFO  (line 322, body c563ddd1775e)
- [x] FRAME_FONT  (line 314, body 9586da1ba2df)
- [x] FRAME_FONTSET  (line 315, body a9a1e0f5b3f0)
- [x] FRAME_NATIVE_WINDOW  (line 295, body 6b5f7a625246)
- [x] FRAME_OUTPUT_DATA  (line 291, body cac7664d9d95)
- [x] GREEN_FROM_ULONG  (line 491, body ac0db4eb4e7e)
- [x] RED_FROM_ULONG  (line 490, body 361e2af92a08)
- [x] RGB_TO_ULONG  (line 489, body ee4a37d6155a)
- [x] WHITE_PIX_DEFAULT  (line 326, body d524558178b7)
- [x] XSCROLL_BAR  (line 382, body aa568fbbbc66)
- [x] _ANDROID_TERM_H_  (line 21, body da39a3ee5e6b)

## Types and their fields — 5
- [x] android_bitmap_record: android_pixmap pixmap, android_pixmap mask, char* file, int refcount, int height, int width, int depth, bool have_mask
- [x] android_display_info: struct android_display_info* next, struct terminal* terminal, Emacs_Window root_window, Lisp_Object name_list_element, Lisp_Object color_map, double resx, double resy, double font_resolution, struct android_gc* scratch_cursor_gc, Mouse_HLInfo mouse_highlight, int n_planes, int n_image_planes, int grabbed, int smallest_char_width, int smallest_font_height, char* x_id_name, int n_fonts, struct android_bitmap_record* bitmaps, ptrdiff_t bitmaps_size, ptrdiff_t bitmaps_last, struct frame* focus_frame, struct frame* x_focus_event_frame, struct frame* highlight_frame, struct frame* pending_autoraise_frame, struct frame* last_mouse_frame, struct frame* last_mouse_glyph_frame, struct frame* last_mouse_motion_frame, int last_mouse_motion_x, int last_mouse_motion_y, Emacs_Rectangle last_mouse_glyph, Time last_mouse_movement_time, int menu_event_id, android_cursor invisible_cursor
- [x] android_output: struct android_gc* normal_gc, struct android_gc* reverse_gc, struct android_gc* cursor_gc, Emacs_Window window, Emacs_Window parent_desc, struct font* font, int baseline_offset, int fontset, unsigned long cursor_pixel, unsigned long mouse_pixel, unsigned long cursor_foreground_pixel, unsigned long scroll_bar_foreground_pixel, unsigned long scroll_bar_background_pixel, Emacs_Cursor text_cursor, Emacs_Cursor nontext_cursor, Emacs_Cursor modeline_cursor, Emacs_Cursor hand_cursor, Emacs_Cursor hourglass_cursor, Emacs_Cursor horizontal_drag_cursor, Emacs_Cursor vertical_drag_cursor, Emacs_Cursor current_cursor, Emacs_Cursor left_edge_cursor, Emacs_Cursor top_left_corner_cursor, Emacs_Cursor top_edge_cursor, Emacs_Cursor top_right_corner_cursor, Emacs_Cursor right_edge_cursor, Emacs_Cursor bottom_right_corner_cursor, Emacs_Cursor bottom_edge_cursor, Emacs_Cursor bottom_left_corner_cursor, bool hourglass, struct android_display_info* display_info, bool_bf has_been_visible : 1, bool_bf alpha_identical_p : 1, bool_bf complete : 1, bool_bf need_buffer_flip : 1, bool_bf need_cursor_updates : 1, struct android_gc* gc, unsigned long pixel, unsigned long relief_background, int focus_state, struct android_touch_point* touch_points, int extracted_text_flags, int extracted_text_token, int extracted_text_hint
- [x] android_touch_point: struct android_touch_point* next, int tool_id, int x, int y, bool tool_bar_p
- [x] scroll_bar: union vectorlike_header header, Lisp_Object window, Lisp_Object next, Lisp_Object prev, Emacs_Window x_window, int top, int left, int width, int height, int start, int end, int dragging, bool horizontal

## File-scope objects — 4
- [x] extern struct android_display_info x_display_list  (line 389)
- [x] extern frame_parm_handler android_frame_parm_handlers  (line 398)
- [x] extern struct font_driver androidfont_driver  (line 439)
- [x] extern const struct font_driver android_sfntfont_driver  (line 464)

## Decisions

- **Liveness: nothing in this header is dead text.** `python3 tools/check-cond-eval.py src/androidterm.h`
  reports `3 conditions, 0 unknown, 0 disagreements [VERIFIED]` (the file is in
  `docs/baseline/config/cond-eval-verified.txt`), and all three conditions are taken in the frozen
  configuration: the include guard's `#ifndef _ANDROID_TERM_H_` (C 20), and both
  `#ifndef ANDROID_STUBIFY` regions (C 448-452 and C 472-485) because `ANDROID_STUBIFY` is `undef`
  in `docs/baseline/config/darwin-arm64.macros.tsv` (line 78).  So P-08 has no dead region to
  account for and every declaration is carried.  `current_menu_serial` (C 450), the two
  `android_notification_*` hooks and `init_androidselect`/`syms_of_androidselect`/
  `syms_of_androidvfs` (C 474-483) are inside those live regions and are mirrored.
- **`_ANDROID_TERM_H_` keeps C's name verbatim** (PORTING.md 0.2) rather than the `EMACS_X_H`
  spelling most mirrors use, because the guard's own name is what the extractor keys: C spells it
  `_ANDROID_TERM_H_` (C 20-21).  It is a `pub const ... : bool = true`, the house shape for a
  guard that has no Rust counterpart (0.1-4/-9).
- **The anonymous `enum` at C 277-289 (FOCUS_NONE/IMPLICIT/EXPLICIT) is three `c_int`
  constants**, keyed by `cinventory` under `constants`, not under `macros`; the generated checklist
  above therefore does not list them, but the constant gate compares their values (0/1/2).
- **`struct android_output`'s five `bool_bf : 1` members fold into one `bits: u8` after
  `display_info`** (PORTING.md 1.3): `bool_bf` is `bool` (`conf_post.h` 61 takes that arm for this
  compiler), so the allocation unit is one byte, and the measured offset (240) is asserted.  The
  five C members are named in the comment at that field.
- **The nested anonymous `struct relief` (C 249-253) gets C's own tag as its Rust name** (1.3);
  `black_relief`/`white_relief` are two fields of that type, which is also how
  `cinventory` flattens them (`gc`, `pixel`).
- **Foreign types come from the include graph** (P-69/P-72): `androidgui.rs` (`android_pixmap`,
  `Emacs_Window`, `android_cursor`, `android_gc`), `frame.rs` (`frame`), `dispextern.rs`
  (`Mouse_HLInfo`, `Emacs_Rectangle`, `Emacs_Cursor`, `Emacs_Color`), `termhooks.rs` (`terminal`,
  `input_event`), `font.rs` (font.h), `systime.rs` (`Time`, which termhooks.h 28 pulls in for C).
  `Lisp_Object`/`union vectorlike_header`/`XVECTOR` come from `lisp.rs`, which no `src/` mirror
  includes (0.1-9) - every unit that carries this mirror has it.
- **Five names have no owner mirror yet and keep a recorded stand-in** (P-72 rule 3, the
  dispextern.rs `struct frame *` precedent), each with its C text in a `/// C:` line:
  `struct font *font` (C 181), `struct font_entity *` (C 444) and the two `struct font_driver`
  objects (C 439/464) - `include/font.rs` is a WIP slice without the driver interface - plus
  `struct window *` (C 401, window.h is not in this header's include list) and
  `struct android_notification_event *` (C 474/476, no record anywhere in the ported tree).
  The objects use `*mut c_void` as the pointer-width stand-in; they become `font_driver` in the
  commit that lands the driver record in `font.rs`.
- **`android_frame_parm_handlers` is an unbounded extern array**: C writes
  `extern frame_parm_handler android_frame_parm_handlers[];` (C 398) and `frame_parm_handler` is
  dispextern.h 3063's typedef, which `include/dispextern.rs` does not carry yet.  The port uses the
  `lisp.rs` `pub static mut my_edata: [c_char; 0]` shape for `extern char my_edata[];`, with the
  pointer-sized element stand-in.
- **Three macro bodies are the mirror's known uncompilable site, and the shared notes table is
  where the fix goes.**  `FRAME_ANDROID_NEED_BUFFER_FLIP` (C 298-299), `FRAME_ANDROID_COMPLETE_P`
  (C 311-312) and `FRAME_ANDROID_DRAWABLE` (C 305-307) read the `bool_bf` members
  `need_buffer_flip` and `complete`, which the port folds into `android_output.bits` (PORTING.md
  1.3).  The mirror keeps C's member names in those bodies so the token gate compares C's own
  tokens, but the folded record has no such fields, so those three bodies cannot compile as
  written.  The P-80/cm.rs mask shape (`bits & MASK != 0`, mask constants named the way cm.rs
  names `CM_AUTOWRAP`, plus `FRAME_ANDROID_DRAWABLE`'s write becoming an OR/AND-NOT pair) is the
  resolution, and it needs three rows in the shared `docs/ports/macro-body-notes.tsv`:
  `FRAME_ANDROID_NEED_BUFFER_FLIP\tneed_buffer_flip\t...`,
  `FRAME_ANDROID_COMPLETE_P\tcomplete\t...` and
  `FRAME_ANDROID_DRAWABLE\tneed_buffer_flip\t...`.  That table is the integrator's, so the rows
  are requested in the lane report rather than written here (the `CODING_FOR_UNIBYTE`,
  `MINI_WINDOW_P`, `SET_FRAME_ICONIFIED` and `FrameCols`-class rows are the precedent).
- **`FRAME_ANDROID_OUTPUT`/`FRAME_OUTPUT_DATA` are *place* expressions in C** (P-37): both expand
  to `f->output_data.android`, an lvalue, and the port keeps them `macro_rules!` for that reason -
  `FRAME_ANDROID_DRAWABLE` assigns through the chain.  The union read is `unsafe` at the use site,
  as C's union member access is unchecked.
- **The four colour macros are duplicated in `pgtkterm.rs` on purpose**: pgtkterm.h 71-76 defines
  its own `RGB_TO_ULONG`/`RED_/GREEN_/BLUE_FROM_ULONG`, and no unit includes both headers, so each
  mirror carries the header's own set (C's text, PORTING.md P-26 is a per-TU rule).
- **Known interaction for a future Android target (recorded, not fixed here)**: androidgui.h 42 and
  dispextern.h 175 both `typedef ... Emacs_Cursor`, and C keeps them apart by
  `#ifndef HAVE_WINDOW_SYSTEM`.  `include/dispextern.rs` carries that branch unconditionally
  because the frozen configuration has no window system, so a translation unit that includes both
  mirrors would see two `Emacs_Cursor` items.  The Android target will need the same `cfg` on
  dispextern.rs's alias.  The same class covers `Display_Info`/`Emacs_Pix_Container`
  (dispextern.h 165-166) and androidgui.h's `android_time`, none of which is reachable on this
  target.
- **`androidterm.c` is not in the frozen build**, so `docs/port-map.md` 372-373 keeps
  `femacs/src/androidterm.rs` as the note-only placeholder (docs/status.md 3.31): the mirror is not
  reachable from any built translation unit's include graph and is compiled only when a future
  Android configuration freezes.  This is the same state `include/gtkutil.rs` and
  `include/pgtkterm.rs` are in.

## Evidence

- `python3 tools/check-cond-eval.py src/androidterm.h` ->
  `src/androidterm.h: 3 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
- `python3 tools/extract-abi.py --check emacs/src/androidterm.h femacs/src/include/androidterm.rs`
  -> `OK emacs/src/androidterm.h -> femacs/src/include/androidterm.rs` (no `- ` problem lines).
- Layout measured with the frozen target's clang (the header cannot be compiled on this target as
  it stands: dispextern.h 175's `Emacs_Cursor` collides with androidgui.h 42's, so the probe adds
  `-include` shims for `HAVE_WINDOW_SYSTEM`, `Display_Info`, `Emacs_Pix_Container` and
  `RGB_PIXEL_COLOR` - the four names the Android build gets from its own headers).  Command and
  readings:
  `clang -std=gnu23 -include build/emacs-ref/src/config.h -include /tmp/shim1.h
   -Ibuild/emacs-ref/lib -Idirtraspikes/shim -Ibuild/emacs-ref/src -Iemacs/src -Iemacs/lib
   /tmp/probe2.c` ->
  `android_bitmap_record 48/8`, `android_display_info 312/8`, `android_output 320/8`,
  `android_touch_point 24/8`, `scroll_bar 72/8`; `android_output` offsets 224 hourglass,
  232 display_info, 240 (the bitfield byte), 248 black_relief, 264 white_relief, 280
  relief_background, 288 focus_state, 296 touch_points, 304/308/312 the three extracted-text
  members; `bool_bf 1`.
- The Rust side of those numbers was proved by compiling the mirror's type section against stub
  definitions of the foreign types (`rustc --edition 2024 --crate-type=lib --emit=obj`): clean,
  and the negative control (`== 320` -> `== 999`) fails with
  `error[E0080]: evaluation panicked: assertion failed`.
- `CARGO_TARGET_DIR=/tmp/femacs-target-mir-androidterm cargo run -q -p xtask -- build-tu
  src/androidterm.c` -> `built src/androidterm.rs -> ../build/femacs/androidterm.o` (the unit is the
  note-only placeholder; the mirror itself is not in that translation unit's include graph, see the
  last Decision).
