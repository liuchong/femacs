# emacs/src/haikuterm.h (375 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 17
- [x] FRAME_BASELINE_OFFSET  (line 293, body b0b369e4efe6)
- [x] FRAME_COMPLETE_P  (line 282, body ff97bddcf804)
- [x] FRAME_CURSOR_COLOR  (line 294, body 1508afda5824)
- [x] FRAME_DIRTY_P  (line 281, body 6d0b95dab920)
- [x] FRAME_DISPLAY_INFO  (line 289, body 9fb76a17606d)
- [x] FRAME_FONT  (line 290, body 8628e27a5888)
- [x] FRAME_FONTSET  (line 291, body 3c9697ce3da8)
- [x] FRAME_HAIKU_DRAWABLE  (line 287, body 489e08f58d05)
- [x] FRAME_HAIKU_MENU_BAR  (line 288, body f202cd3b4928)
- [x] FRAME_HAIKU_VIEW  (line 286, body 5f243ee739c1)
- [x] FRAME_HAIKU_WINDOW  (line 285, body 28a7fb702f45)
- [x] FRAME_NATIVE_WINDOW  (line 292, body 28a7fb702f45)
- [x] FRAME_OUTPUT_DATA  (line 284, body 94a12c3490fa)
- [x] HAVE_CHAR_CACHE_MAX  (line 35, body 0cca08dd76e2)
- [x] MAKE_FRAME_DIRTY  (line 283, body aad54d34cac2)
- [x] XSCROLL_BAR  (line 279, body aa568fbbbc66)
- [x] _HAIKU_TERM_H_  (line 20, body da39a3ee5e6b)

## Types and their fields — 5
- [x] haiku_bitmap_record: haiku img, char* file, int refcount, int height, int width, int depth, uint32_t stipple_foreground, uint32_t stipple_background, void* stipple_bits
- [x] haiku_display_info: struct haiku_display_info* next, struct terminal* terminal, Lisp_Object name_list_element, Lisp_Object color_map, int n_fonts, int smallest_char_width, int smallest_font_height, struct frame* focused_frame, struct frame* focus_event_frame, struct frame* last_mouse_glyph_frame, struct haiku_bitmap_record* bitmaps, ptrdiff_t bitmaps_size, ptrdiff_t bitmaps_last, int grabbed, int n_planes, int color_p, Lisp_Object rdb, Lisp_Object default_name, Emacs_Cursor vertical_scroll_bar_cursor, Emacs_Cursor horizontal_scroll_bar_cursor, Mouse_HLInfo mouse_highlight, struct frame* highlight_frame, struct frame* last_mouse_frame, struct frame* last_mouse_motion_frame, int last_mouse_motion_x, int last_mouse_motion_y, struct haiku_rect last_mouse_glyph, haiku display, double resx, double resy, Time last_mouse_movement_time, Window root_window, Emacs_Cursor text_cursor, Emacs_Cursor nontext_cursor, Emacs_Cursor modeline_cursor, Emacs_Cursor hand_cursor, Emacs_Cursor hourglass_cursor, Emacs_Cursor horizontal_drag_cursor, Emacs_Cursor vertical_drag_cursor, Emacs_Cursor left_edge_cursor, Emacs_Cursor top_left_corner_cursor, Emacs_Cursor top_edge_cursor, Emacs_Cursor top_right_corner_cursor, Emacs_Cursor right_edge_cursor, Emacs_Cursor bottom_right_corner_cursor, Emacs_Cursor bottom_edge_cursor, Emacs_Cursor bottom_left_corner_cursor, Emacs_Cursor no_cursor
- [x] haiku_output: struct haiku_display_info* display_info, Emacs_Cursor text_cursor, Emacs_Cursor nontext_cursor, Emacs_Cursor modeline_cursor, Emacs_Cursor hand_cursor, Emacs_Cursor hourglass_cursor, Emacs_Cursor horizontal_drag_cursor, Emacs_Cursor vertical_drag_cursor, Emacs_Cursor left_edge_cursor, Emacs_Cursor top_left_corner_cursor, Emacs_Cursor top_edge_cursor, Emacs_Cursor top_right_corner_cursor, Emacs_Cursor right_edge_cursor, Emacs_Cursor bottom_right_corner_cursor, Emacs_Cursor bottom_edge_cursor, Emacs_Cursor bottom_left_corner_cursor, Emacs_Cursor no_cursor, Emacs_Cursor current_cursor, Emacs_Color cursor_color, Window parent_desc, haiku window, haiku view, haiku menubar, int fontset, int baseline_offset, bool_bf hourglass_p : 1, bool_bf menu_bar_open_p : 1, bool_bf dirty_p : 1, bool_bf complete_p : 1, struct font* font, int pending_top, int pending_left, bool configury_done, uint32_t cursor_fg, struct haiku_menu_bar_click_event* saved_menu_event, int wait_for_event_type, uint32_t black_relief_pixel, uint32_t white_relief_pixel, long relief_background, int frame_x, int frame_y, int fullscreen_mode
- [ ] haikufont_info: struct font font, haiku be_font, struct font_metrics** metrics, short metrics_nrows, unsigned short** glyphs
- [x] scroll_bar: union vectorlike_header header, Lisp_Object window, Lisp_Object next, Lisp_Object prev, int top, int left, int width, int height, void* scroll_bar, int dragging, int update, int position, int total, bool horizontal, int page_size

## File-scope objects — 9
- [x] extern int popup_activated_p  (line 40)
- [x] extern struct haiku_display_info x_display_list  (line 225)
- [x] extern struct font_driver const haikufont_driver  (line 226)
- [x] extern Lisp_Object tip_frame  (line 228)
- [x] extern Lisp_Object tip_dx  (line 229)
- [x] extern Lisp_Object tip_dy  (line 230)
- [x] extern struct frame haiku_dnd_frame  (line 232)
- [x] extern bool haiku_dnd_follow_tooltip  (line 233)
- [x] extern frame_parm_handler haiku_frame_parm_handlers  (line 235)

## Decisions

- **The live text is carried whole; the four dead arms are recorded** (PORTING.md P-08,
  `tools/check-cond-eval.py src/haikuterm.h` -> `5 conditions, 0 unknown, 0 disagreements
  [VERIFIED]`, all five false): `USE_BE_CAIRO` at C 24-26 (`#include <cairo.h>`), C 296-301
  (`FRAME_CR_CONTEXT`) and C 362-366 (two cairo prototypes), and `HAVE_NATIVE_IMAGE_API` at
  C 352-357 (`haiku_can_use_native_image_api`, `haiku_load_image`, `syms_of_haikuimage`).
- **`struct haikufont_info` is not mirrored, and its box above stays unchecked.**  Its first
  member is a `struct font` **by value**, and no mirror declares that record: `include/font.rs`
  is a WIP slice (50 boxes) whose `struct font` still lives in the `font.c` unit mirror
  (`femacs/src/font.rs` 256, with a measurement that is the *frozen* configuration's: 184 bytes
  with the thirteen `HAVE_WINDOW_SYSTEM`-guarded members absent).  On a Haiku build
  `HAVE_WINDOW_SYSTEM` is defined, so the record has those thirteen members and a different size;
  a zero-sized stand-in would be a silent layout lie and a byte-array one would encode the wrong
  size for exactly the platform this header belongs to.  The right fix is upstream: `struct font`
  (and `font_metrics`, `font_driver`) move from the unit mirror into `include/font.rs` under
  P-69, and `haikufont_info` lands in the same commit.  Recorded for the integrator.
- **`struct font *font` / `struct font_metrics **metrics` inside `struct haiku_output`** keep the
  untyped pointer with a `/// C:` line (PORTING.md P-55/P-72 rule 3), the shape
  `include/androidterm.rs` 250 uses for the very same missing record; `haikufont_driver`
  (C 226) and `haiku_frame_parm_handlers` (C 235, element type `frame_parm_handler`, another
  dispextern.h typedef `include/dispextern.rs` does not carry) keep the pointer-sized stand-ins
  in `haikuterm_objs`, also following that mirror.
- **`glyph_string` / `face`** appear only as pointers in two prototypes and no mirror declares
  them, so those parameters are `*mut c_void` with a `/// C:` line (P-55).
- **The four `bool_bf … : 1` members of `struct haiku_output`** (C 190-205: `hourglass_p`,
  `menu_bar_open_p`, `dirty_p`, `complete_p`) are one storage byte in C (`bool_bf` is `bool` on
  this target - conf_post.h 59-61 - and the port measured the same 1-byte unit for `itree_node`),
  so the mirror folds them into `pub bits: u8` (PORTING.md 1.3/P-80) and dispatches on the member
  name through `haiku_output_bits!`; the four masks are named after the members.
- **`MAKE_FRAME_DIRTY` cannot be C's `(FRAME_DIRTY_P (f) = 1)`**: Rust has no macro that yields an
  assignable place, so `FRAME_DIRTY_P` gets a second, two-argument arm (the write form) and
  `MAKE_FRAME_DIRTY` spells `FRAME_DIRTY_P! ($f, 1)` - which also keeps C's three tokens
  (`FRAME_DIRTY_P`, `f`, `1`) in the body the gate compares.  `FRAME_HAIKU_DRAWABLE`'s comma
  expression becomes a block whose value is `FRAME_HAIKU_VIEW! ($f)`, C's own value.
- **The field-reading `FRAME_*` macros cast the union member once**
  (`*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)`): `frame.rs`'s `output_data.haiku` is still
  P-72 rule 3's `*mut c_void` stand-in from before this record existed, and the cast is what that
  rule says the use site writes - it stays valid when `frame.rs`'s member becomes
  `*mut haiku_output`.  (The sibling `include/androidterm.rs` instead keeps C's text verbatim and
  records the bodies as known-uncompilable; this mirror's version compiles today, which is what
  makes the platform probe evidence rather than a parse check.  Both are recorded.)
- **`XSCROLL_BAR (vec)`** is `(XVECTOR ($vec) as *mut scroll_bar)`, `XVECTOR` being lisp.rs's
  function (the `androidterm.rs` 486 shape).
- **Includes**: C's six (`haikugui.h`, `frame.h`, `character.h`, `dispextern.h`, `font.h`,
  `systime.h`) plus the two the *unit* proves are needed - `termhooks.rs` for `struct terminal`/
  `struct input_event` (C reaches termhooks.h through frame.h) and `haiku_support.rs` for
  `struct haiku_menu_bar_click_event`/`struct haiku_font_change_event` (haikuterm.c 32 includes
  haiku_support.h).  `character.rs` declares no name the live text uses; it is listed because C
  lists it (the `androidterm.rs` include-list style).  `Lisp_Object`/`union vectorlike_header`
  come from the unit's lisp.rs (PORTING.md 0.1-9).
- **`haiku_get_pixel`/`haiku_put_pixel` are live here** (C 340-341), which closes the note in
  `include-haiku_support.md`: that header declares the pair only inside its dead `__cplusplus`
  arm, and this one is the live declaration a Haiku unit gets.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/haikuterm.h femacs/src/include/haikuterm.rs`
  -> `FAIL` with exactly one problem, `- missing type haikufont_info`.  The field loop alone did
  **not** report it (it walks the Rust containers and skips one that is absent), so the ledger box
  above was the only record until the missing-type check (status 3.149/3.151/3.152) was added: its
  own comment names this header's `haikufont_info` as the live example.  That problem line is
  therefore the blocker, not a spelling deviation - the fix is the upstream one in the decision
  bullet above (move `struct font`/`font_metrics`/`font_driver` into `include/font.rs` under
  P-69 and land `haikufont_info` in the same commit), and this is the one mirror in the batch
  whose `--check` is not `OK`.
- Compile proof: the Haiku platform probe (`spikes/haiku_inc_probe.rs`), which compiles this
  mirror with `haikugui.rs`, `haiku_support.rs`, `lisp.rs`/`config.rs` and the rest of a Haiku
  translation unit's closure, with `dispextern.rs` read through a module so that the platform's own
  `Emacs_Cursor` (haikugui.h's) is the one in scope:
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-probe cargo run -q -p xtask -- probe
  ../spikes/haiku_inc_probe.rs ../build/probes/haiku_inc_probe.rmeta --crate-type=lib
  --emit=metadata` -> exit 0; and on the materialised root,
  `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type=lib --emit=metadata -o
  /tmp/haiku_inc_probe.rmeta build/probes/haiku_inc_probe/_root.rs` -> exit 0.
- Consumer smoke: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-haikuterm cargo run -q
  -p xtask -- build-tu src/haikuterm.c` -> `built src/haikuterm.rs -> ../build/femacs/haikuterm.o`
  (the platform unit is the empty unit plus its note, docs/status.md 3.31).
- Gate readings (repo root): `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0;
  `check-verbatim` 6 trees / 4607 files / 0; `check-mirror-dupes` 1 duplicate declaration / 1
  ratchet row / 0 failures; `check-globals` 283 units / 0; `check-target-names` 0 findings.
