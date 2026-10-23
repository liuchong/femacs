# src/abi_shim.rs (the C-ABI entry points the macOS Swift units import; no reference file)

This file is **not** the port of a reference file: C has no functions with these names.  It is the
core side's table of honouring the names the macOS Swift units import with `@_silgen_name`
(`tools/check-swift-imports.py`; the rulings are
`docs/reviews/2026-10-04-swift-import-rulings.md`).  Each entry is the C-ABI spelling of a C
construct the Swift lane reads or writes - a `struct frame` member, a `FRAME_*` accessor macro, or
a bitfield of `frames.bits1` - so the core has to provide it as a real symbol before the Swift
units can link.

The first line is the `DECLARED` pattern `check-target-names.py` accepts for a product that is not
a port (`^// femacs: generated mirror of the frozen build's`), which is why the file is under
`src/` yet appears in no row of `docs/port-inventory.md` or `docs/port-map.md`; both were
re-run to confirm it (see `- **The gate audit (this file's 73 gated entries, no export added or changed)**: a `/tmp` script
  parsed every `#[cfg (...)]` of this file and every identifier each gated body names, resolved
  each identifier's home in the mirror tree and compared gates.  Line: `# shim items: 179;
  gated: 73`; verdict **0 mirror-item gate mismatches**; distribution `have_ns` 45,
  `have_ns,ns_impl_cocoa` 1, `dispextern_face_records` 16, `have_window_system` 11 - identical
  before and after, because there was nothing to fix.  The one asymmetry the audit names is a
  *kind* of gate, not a mismatch: 57 of the 73 carry a gate C has, 16 carry the mirror's
  `dispextern_face_records` where C's two records are ungated (stricter than C, by 3.220's
  ruling - `## Gate audit`).  The out-of-scope neighbour is measured too: the ungated entries
  reading `include/frame.rs`'s `FRAME_*_SHIFT`/`FRAME_HAS_VERTICAL_SCROLL_BARS*` reach items that
  mirror declares only under `#[cfg (not (feature = "have_window_system"))]` (frame.rs 278-331,
  608-620), which is the forty-`E0425` wall the `have_window_system` flip hits (batches 11/12).
  `build-tu` was re-run after the audit (no gate changed) and the object is unchanged: 129 `T`
  plus the one `D`, and none of the 73 guarded names in the frozen object.

- **The twenty citation drifts this file had are fixed (the round after batch 13)**:
  `python3 tools/check-citations.py` reported `44 drift` tree-wide with **20 in
  `src/abi_shim.rs`**; all twenty now cite the reference line the quoted text actually
  lives on, and the tool's own reading is `17 drift, 1 range, 344 external, 33 allowed, 0 stale`
  with **no `abi_shim` finding left** (the remaining drift is `platform/macos/*.swift`,
  another lane's).  Three of the twenty were genuinely stale *numbers* - `FRAME_MENU_BAR_LINES`
  1113 -> frame.h 1086, `FRAME_TAB_BAR_LINES` 1126 -> 1090, `FRAME_TAB_BAR_HEIGHT` 1127 -> 1091 -
  and the other seventeen were a *pairing* problem the tool's rule makes visible: it pairs the
  first `path line` on a citation line with the first code-looking backticked snippet after it and
  asks whether that snippet occurs within that line's window, so a citation whose number names one
  construct while the first snippet names another reports drift even though both halves are
  individually right (e.g. `frame.h 635` followed by `FRAME_MENU_BAR_LINES`, or `dispextern.h 969`
  followed by `struct glyph_row`).  Those had the snippet moved to the half that owns the number
  (`(frame.h 635, `menu_bar_lines`)`, `(dispextern.h 969, `visible_height`; inside
  `struct glyph_row` 918)`), and every number written is the one measured in the reference header.
  Comments only: the object is unchanged (`nm -g`: 150 `T` plus batch 12's one `D`; `nm -u`: the
  same 18).

## Evidence`).

## Exports - 206 (all `#[unsafe(no_mangle)] pub unsafe extern "C" fn` but one `pub static mut` - batch 12's `max_used_fringe_bitmap` storage -, `frame` received as `*mut c_void` by the lanes' habit; the ABI is the Swift declaration's)

Two hundred and six: one hundred and twenty-seven are in the frozen object (`nm -g
build/femacs/abi_shim.o`, 150 `T` symbols including the unit's own Rust-internal ones, plus the
one `D` batch 12 defines) and the other seventy-nine appear when the configuration that owns
their member is built - the four batch-5 arm-only writers, the forty-two `struct ns_output`
accessors of batch 6, batch 6's `face_set_background`, batch 11's ten `struct glyph_string`
accessors and batch 12/13's twenty-two (`dispextern_face_records`'s fifteen, batch 12's fringe
export and batch 13's six cursor/fringe ones) - see `## Decisions` and `## Gate audit`.
Thirty-seven of the two hundred and six (the four `z_group_*` enumerators, the two `Lisp_Object`
accessors of batch 7, batch 8's fourteen mirror-named entries, batch 10's `set_hash_value_slot`,
batch 11's two `enum glyph_type` enumerators, batch 12's `WINDOW_FRAME` and
`max_used_fringe_bitmap` storage and batch 13's twelve - the two frame macros and the ten
enumerators) stand in a module, `c_abi_of_mirror_named_items`, because the mirrors already spell
those names in Rust; the module is why the crate root can hold both (measured, `## Evidence`).

| name | Swift import | providing Rust item | C text (line) |
| --- | --- | --- | --- |
| `frame_left_pos` | `nsmenu.swift` 324, `nsfns.swift` 644, `nsterm.swift` 1598 (`(UnsafeMutableRawPointer?) -> Int32`) | `abi_shim.rs` 64 | `int left_pos, top_pos;` frame.h 604 |
| `frame_top_pos` | `nsmenu.swift` 325, `nsfns.swift` 645, `nsterm.swift` 1599 (`-> Int32`) | `abi_shim.rs` 71 | frame.h 604 |
| `frame_title` | `nsfns.swift` 253 (`-> Lisp_Object`) | `abi_shim.rs` 78 | `Lisp_Object title;` frame.h 162 |
| `frame_icon_name` | `nsfns.swift` 254 (`-> Lisp_Object`) | `abi_shim.rs` 85 | `Lisp_Object icon_name;` frame.h 158 |
| `frame_param_alist` | `nsfns.swift` 255 (`-> Lisp_Object`) | `abi_shim.rs` 92 | `Lisp_Object param_alist;` frame.h 209 |
| `frame_selected_window` | `nsfns.swift` 498-499 (`-> Lisp_Object`) | `abi_shim.rs` 99 | `#define FRAME_SELECTED_WINDOW(f) f->selected_window` frame.h 1204 |
| `frame_menu_bar_vector` | `nsmenu.swift` 318-319 (`-> Lisp_Object`) | `abi_shim.rs` 106 | `Lisp_Object menu_bar_vector;` frame.h 234 |
| `frame_tool_bar_items` | `nsmenu.swift` 322-323 (`-> Lisp_Object`) | `abi_shim.rs` 113 | `Lisp_Object tool_bar_items;` frame.h 283 |
| `frame_menu_bar_items_used` | `nsmenu.swift` 314-315 (`-> Int32`) | `abi_shim.rs` 120 | `int menu_bar_items_used;` frame.h 307 |
| `frame_n_tool_bar_items` | `nsmenu.swift` 320-321 (`-> Int32`) | `abi_shim.rs` 127 | `int n_tool_bar_items;` frame.h 573 |
| `frame_set_border_width` | `nsfns.swift` 273-274 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 139 | `int border_width;` frame.h 613 |
| `frame_set_child_frame_border_width` | `nsfns.swift` 262-263 (`(…, Int32)`) | `abi_shim.rs` 146 | `int child_frame_border_width;` frame.h 617 |
| `frame_set_internal_border_width` | `nsfns.swift` 264-265 (`(…, Int32)`) | `abi_shim.rs` 153 | `int internal_border_width;` frame.h 622 |
| `frame_set_menu_bar_lines` | `nsfns.swift` 486-487 (`(…, Int32)`) | `abi_shim.rs` 160 | `int menu_bar_lines;` frame.h 635 |
| `frame_set_tab_bar_lines` | `nsfns.swift` 492-493 (`(…, Int32)`) | `abi_shim.rs` 167 | `int tab_bar_lines;` frame.h 558 |
| `frame_set_tab_bar_height` | `nsfns.swift` 490-491 (`(…, Int32)`) | `abi_shim.rs` 174 | `int tab_bar_height;` frame.h 561 |
| `frame_set_background_pixel` | `nsfns.swift` 269-270 (`(…, UInt)`) | `abi_shim.rs` 181 | `unsigned long background_pixel;` frame.h 732 |
| `frame_set_foreground_pixel` | `nsfns.swift` 267-268 (`(…, UInt)`) | `abi_shim.rs` 188 | `unsigned long foreground_pixel;` frame.h 733 |
| `frame_explicit_name` | `nsfns.swift` 249 (`-> Bool`) | `abi_shim.rs` 202 | `bool_bf explicit_name : 1` frame.h 417 (`FRAME_EXPLICIT_NAME_SHIFT` 15) |
| `frame_set_explicit_name` | `nsfns.swift` 250-251 (`(…, Bool)`) | `abi_shim.rs` 209 | frame.h 417 |
| `frame_set_wants_modeline` | `nsfns.swift` 271-272 (`(…, **Bool**)`) | `abi_shim.rs` 219 | `bool_bf wants_modeline : 1` frame.h 402 (`FRAME_WANTS_MODELINE_SHIFT` 11) |
| `frame_set_output_method` | `nsfns.swift` 266 (`(…, Int32)`) | `abi_shim.rs` 230 | `ENUM_BF (output_method) output_method : 4;` frame.h 446 (shift 21, `FRAME_OUTPUT_METHOD_MASK` 0x0F) |
| `frame_tab_bar_redisplayed` | `nsfns.swift` 260-261 (`-> Bool`) | `abi_shim.rs` 241 | `bool_bf tab_bar_redisplayed : 1` frame.h 503 (shift 30) |
| `frame_tab_bar_resized` | `nsfns.swift` 257 (`-> Bool`) | `abi_shim.rs` 248 | `bool_bf tab_bar_resized : 1` frame.h 504 (shift 31) |
| `frame_set_tab_bar_resized` | `nsfns.swift` 258-259 (`(…, Bool)`) | `abi_shim.rs` 255 | frame.h 504 |
| `frame_tab_bar_window` | `nsfns.swift` 256 (`-> Lisp_Object`) | `abi_shim.rs` 271 | `Lisp_Object tab_bar_window;` frame.h 254 (**inside `#ifdef HAVE_WINDOW_SYSTEM`**) - the frozen answer is `Qnil` |
| `frame_set_external_menu_bar` | `nsfns.swift` 487-488 (`(…, Int32)`) | `abi_shim.rs` 283 | `#define FRAME_EXTERNAL_MENU_BAR(f) false` frame.h 1082 (the `#else` arm) - no-op |
| `frame_set_external_tool_bar` | `nsfns.swift` 493-494 (`(…, Int32)`) | `abi_shim.rs` 291 | `#define FRAME_EXTERNAL_TOOL_BAR(f) false` frame.h 1097 (the `#else` arm) - no-op |
| `frame_name` | `nsfns.swift` 252 (`-> Lisp_Object`) | `abi_shim.rs` 307 | `Lisp_Object name;` frame.h 154 |
| `frame_set_terminal` | `nsfns.swift` 631-632 (`(…, UnsafeMutableRawPointer?)`) | `abi_shim.rs` 315 | `struct terminal *terminal;` frame.h 648 (C's writes: nsfns.m 1267, 2988) |
| `frame_set_no_split` | `nsfns.swift` 635 (`(…, Bool)`) | `abi_shim.rs` 323 | `bool_bf no_split : 1` frame.h 411 (`FRAME_NO_SPLIT_SHIFT` 14; C's writes: nsfns.m 1474, 3139) |
| `frame_set_undecorated` | `nsfns.swift` 573-574 (`(…, Bool)`) | `abi_shim.rs` 334 | `bool_bf undecorated : 1` frame.h 449 (shift 25; C's write: nsfns.m 1404 `FRAME_UNDECORATED (f) = …`) |
| `frame_set_can_set_window_size` | `nsfns.swift` 637-638 (`(…, Bool)`) | `abi_shim.rs` 345 | `bool_bf can_set_window_size : 1` frame.h 491 (shift 28; C's writes: nsfns.m 1545, 3149) |
| `frame_set_was_invisible` | `nsfns.swift` 636 (`(…, Bool)`) | `abi_shim.rs` 358 | `bool_bf was_invisible : 1` frame.h 530 (second flag word, shift 5; C's write: nsfns.m 1569) |
| `window_contents` | `nsfns.swift` 499, `nsmenu.swift` 413-414 (`-> Lisp_Object`) | `abi_shim.rs` 375 | `Lisp_Object contents;` window.h 143 |
| `window_current_matrix` | `nsfns.swift` 500-501 (`-> UnsafeMutableRawPointer?`) | `abi_shim.rs` 382 | `struct glyph_matrix *current_matrix;` window.h 224 |
| `buffer_filename` | `nsfns.swift` 504-505 (`-> Lisp_Object`) | `abi_shim.rs` 390 | `#define BVAR(buf, field) ((buf)->field ## _)` buffer.h 308 (the member is `Lisp_Object filename_;`, buffer.h 330) |
| `terminal_id` | `nsfns.swift` 373 (`-> Int32`) | `abi_shim.rs` 397 | `int id;` termhooks.h 516 |
| `terminal_display_info_ns` | `nsfns.swift` 374-375 (`-> UnsafeMutablePointer<ns_display_info>?`) | `abi_shim.rs` 406 | `struct ns_display_info *ns;` termhooks.h 538 (inside `union display_info`, 533) |
| `frame_alloc_ns_output` | `nsfns.swift` 633 (`(…)`) | `abi_shim.rs` 430, **`#[cfg (feature = "have_ns")]`** | `f->output_data.ns = xzalloc (sizeof *f->output_data.ns);` nsfns.m 1270, 2991 |
| `frame_set_fontset` | `nsfns.swift` 634 (`(…, Int32)`) | `abi_shim.rs` 441, **`have_ns`** | `#define FRAME_FONTSET(f) ((f)->output_data.ns->fontset)` nsterm.h 1096 (C's writes: nsfns.m 1272, 2994) |
| `frame_set_display_info` | `nsfns.swift` 716-717 (`(…, UnsafeMutablePointer<ns_display_info>?)`) | `abi_shim.rs` 453, **`have_ns`** | `#define FRAME_DISPLAY_INFO(f) ((f)->output_data.ns->display_info)` nsterm.h 1032 (C's writes: nsfns.m 1280, 2995) |
| `frame_set_double_buffered` | `nsfns.swift` 495-496 (`(…, Bool)`) | `abi_shim.rs` 474, **`have_ns` + `ns_impl_cocoa`** | `#define FRAME_DOUBLE_BUFFERED(f) ((f)->output_data.ns->double_buffered)` nsterm.h 1052 (inside `#if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MIN_REQUIRED >= 101400`, 1051-1053) |
| `ns_output_view` | `nsfns.swift` 275 (`(UnsafeMutableRawPointer?) -> NSView?`) | `abi_shim.rs` 511 **`have_ns`** | `EmacsView *view;` nsterm.h 938 (frozen half `void *view;` 948); `FRAME_NS_VIEW` 1045 |
| `ns_output_miniimage` | `nsfns.swift` 278 (`-> AnyObject?`), `nsterm.swift` 1850 (`-> NSImage?`) | `abi_shim.rs` 523 **`have_ns`** | `id miniimage;` nsterm.h 939 (frozen `void *miniimage;` 949) |
| `ns_output_cursor_color` | `nsfns.swift` 282 (`-> NSColor?`) | `abi_shim.rs` 536 **`have_ns`** | `NSColor *cursor_color;` nsterm.h 940 (frozen 950); `FRAME_CURSOR_COLOR` 1046 |
| `ns_output_foreground_color` | `nsfns.swift` 286 (`-> NSColor?`) | `abi_shim.rs` 548 **`have_ns`** | `NSColor *foreground_color;` nsterm.h 941 (frozen 951); `FRAME_FOREGROUND_COLOR` 1037 |
| `ns_output_background_color` | `nsfns.swift` 290 (`-> NSColor?`) | `abi_shim.rs` 559 **`have_ns`** | `NSColor *background_color;` nsterm.h 942 (frozen 952); `FRAME_BACKGROUND_COLOR` 1038 |
| `ns_output_icon_top` | `nsfns.swift` 294 (`-> Int32`) | `abi_shim.rs` 569 **`have_ns`** | `int icon_top;` nsterm.h 990 |
| `ns_output_icon_left` | `nsfns.swift` 297 (`-> Int32`) | `abi_shim.rs` 581 **`have_ns`** | `int icon_left;` nsterm.h 991 |
| `ns_output_window_desc` | `nsfns.swift` 300 (`-> UInt`) | `abi_shim.rs` 596 **`have_ns`** | `Window window_desc, parent_desc;` nsterm.h 980; `FRAME_NS_WINDOW` 1034 |
| `ns_output_parent_desc` | `nsfns.swift` 303 (`-> UInt`) | `abi_shim.rs` 608 **`have_ns`** | `Window … parent_desc;` nsterm.h 980 |
| `ns_output_explicit_parent` | `nsfns.swift` 306 (`-> Bool`) | `abi_shim.rs` 621 **`have_ns`** | `char explicit_parent;` nsterm.h 981 |
| `ns_output_text_cursor` | `nsfns.swift` 310 (`-> NSObject?`) | `abi_shim.rs` 635 **`have_ns`** | `Emacs_Cursor text_cursor;` nsterm.h 960 |
| `ns_output_in_animation` | `nsfns.swift` 314, `nsmenu.swift` 326 (`-> Int32`) | `abi_shim.rs` 647 **`have_ns`** | `int in_animation;` nsterm.h 1010 |
| `ns_output_display_info` | `nsfns.swift` 318 (`-> UnsafeMutablePointer<ns_display_info>`) | `abi_shim.rs` 663 **`have_ns`** | `struct ns_display_info *display_info;` nsterm.h 1004; `FRAME_DISPLAY_INFO` 1032 |
| `ns_output_zooming` | `nsterm.swift` 2011 (`-> Bool`) | `abi_shim.rs` 674 **`have_ns`** | `int zooming;` nsterm.h 1007 |
| `ns_output_set_view` | `nsfns.swift` 276 (`(…, NSView?)`) | `abi_shim.rs` 686 **`have_ns`** | `FRAME_NS_VIEW (f) = self;` nsterm.m 8587 |
| `ns_output_set_miniimage` | `nsfns.swift` 280 (`(…, AnyObject?)`) | `abi_shim.rs` 698 **`have_ns`** | `f->output_data.ns->miniimage = image;` nsfns.m 921, 962 |
| `ns_output_set_cursor_color` | `nsfns.swift` 284 (`(…, NSColor?)`) | `abi_shim.rs` 711 **`have_ns`** | `FRAME_CURSOR_COLOR (f) = [col retain];` nsfns.m 358 |
| `ns_output_set_foreground_color` | `nsfns.swift` 288 (`(…, NSColor?)`) | `abi_shim.rs` 724 **`have_ns`** | `f->output_data.ns->foreground_color = col;` nsfns.m 277 |
| `ns_output_set_background_color` | `nsfns.swift` 292 (`(…, NSColor?)`) | `abi_shim.rs` 737 **`have_ns`** | `f->output_data.ns->background_color = col;` nsfns.m 310 |
| `ns_output_set_icon_top` | `nsfns.swift` 295 (`(…, Int32)`) | `abi_shim.rs` 749 **`have_ns`** | `= -1;` nsfns.m 1036, `= XFIXNUM (icon_y);` 1048 |
| `ns_output_set_icon_left` | `nsfns.swift` 298 (`(…, Int32)`) | `abi_shim.rs` 761 **`have_ns`** | `= -1;` nsfns.m 1037, `= XFIXNUM (icon_x);` 1049 |
| `ns_output_set_window_desc` | `nsfns.swift` 301 (`(…, UInt)`) | `abi_shim.rs` 775 **`have_ns`** | `f->output_data.ns->window_desc = desc_ctr++;` nsfns.m 1285 |
| `ns_output_set_parent_desc` | `nsfns.swift` 304 (`(…, UInt)`) | `abi_shim.rs` 789 **`have_ns`** | nsfns.m 1288, 1293, 3076 |
| `ns_output_set_explicit_parent` | `nsfns.swift` 308 (`(…, Bool)`) | `abi_shim.rs` 802 **`have_ns`** | `= 1;` nsfns.m 1289, `= 0;` 1294 |
| `ns_output_set_current_pointer` | `nsfns.swift` 312 (`(…, NSObject?)`) | `abi_shim.rs` 816 **`have_ns`** | nsfns.m 1498; `FRAME_POINTER_TYPE (f) = cursor;` nsterm.m 2634 |
| `ns_output_set_text_cursor` | `nsfns.swift` 651 (`(…, NSObject?)`) | `abi_shim.rs` 829 **`have_ns`** | `Emacs_Cursor text_cursor;` nsterm.h 960; write nsfns.m 1478 |
| `ns_output_set_nontext_cursor` | `nsfns.swift` 653 | `abi_shim.rs` 842 **`have_ns`** | nsterm.h 961; nsfns.m 1479 |
| `ns_output_set_modeline_cursor` | `nsfns.swift` 655 | `abi_shim.rs` 855 **`have_ns`** | nsterm.h 962; nsfns.m 1480 |
| `ns_output_set_hand_cursor` | `nsfns.swift` 657 | `abi_shim.rs` 868 **`have_ns`** | nsterm.h 963; nsfns.m 1481 |
| `ns_output_set_hourglass_cursor` | `nsfns.swift` 659 | `abi_shim.rs` 881 **`have_ns`** | nsterm.h 964; nsfns.m 1482 |
| `ns_output_set_horizontal_drag_cursor` | `nsfns.swift` 661 | `abi_shim.rs` 894 **`have_ns`** | nsterm.h 965; nsfns.m 1483 |
| `ns_output_set_vertical_drag_cursor` | `nsfns.swift` 663 | `abi_shim.rs` 907 **`have_ns`** | nsterm.h 966; nsfns.m 1484 |
| `ns_output_set_left_edge_cursor` | `nsfns.swift` 665 | `abi_shim.rs` 920 **`have_ns`** | nsterm.h 967; nsfns.m 1485 |
| `ns_output_set_top_left_corner_cursor` | `nsfns.swift` 667 | `abi_shim.rs` 933 **`have_ns`** | nsterm.h 968; nsfns.m 1486 |
| `ns_output_set_top_edge_cursor` | `nsfns.swift` 669 | `abi_shim.rs` 946 **`have_ns`** | nsterm.h 969; nsfns.m 1487 |
| `ns_output_set_top_right_corner_cursor` | `nsfns.swift` 671 | `abi_shim.rs` 959 **`have_ns`** | nsterm.h 970; nsfns.m 1488 |
| `ns_output_set_right_edge_cursor` | `nsfns.swift` 673 | `abi_shim.rs` 972 **`have_ns`** | nsterm.h 971; nsfns.m 1489 |
| `ns_output_set_bottom_right_corner_cursor` | `nsfns.swift` 675 | `abi_shim.rs` 985 **`have_ns`** | nsterm.h 972; nsfns.m 1490 |
| `ns_output_set_bottom_edge_cursor` | `nsfns.swift` 677 | `abi_shim.rs` 998 **`have_ns`** | nsterm.h 973; nsfns.m 1491 |
| `ns_output_set_bottom_left_corner_cursor` | `nsfns.swift` 679 | `abi_shim.rs` 1011 **`have_ns`** | nsterm.h 974; nsfns.m 1492 |
| `ns_output_set_in_animation` | `nsfns.swift` 316, `nsmenu.swift` 328 (`(…, Int32)`) | `abi_shim.rs` 1023 **`have_ns`** | nsfns.m 1500, nsmenu.m 1109/1111 |
| `ns_output_set_zooming` | `nsterm.swift` 2012 (`(…, Bool)`) | `abi_shim.rs` 1036 **`have_ns`** | nsterm.m 1907, 1949, 8662 |
| `glyph_row_y` | `nsterm.swift` 834 (`(UnsafeMutableRawPointer?) -> Int32`) | `abi_shim.rs` 1056 | `int x, y;` dispextern.h 950; C reads `row->y` nsterm.m 818 |
| `glyph_row_visible_height` | `nsterm.swift` 835-836 (`-> Int32`) | `abi_shim.rs` 1066 | `int visible_height;` dispextern.h 969; C reads nsterm.m 821 |
| `set_waiting_for_input_flag` | `nsmenu.swift` 407 (`(Bool)`) | `abi_shim.rs` 1085 | `waiting_for_input = 0;` nsmenu.m 202; macro thread.h 185 |
| `face_set_background` | `nsfns.swift` 475-476 (`(…, UInt)`) | `abi_shim.rs` 1107 **`dispextern_face_records`** | `unsigned long background;` dispextern.h 1819; write nsfns.m 328-329 |
| `frame_set_no_accept_focus` | `nsterm.swift` 2162-2163 (`(…, Bool)`) | `abi_shim.rs` 1127 | `FRAME_NO_ACCEPT_FOCUS (f) = !NILP (new_value);` nsterm.m 2074; frame.h 411/1249 |
| `frame_set_menu_bar_items_used` | `nsmenu.swift` 316-317 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1141 | `f->menu_bar_items_used = menu_items_used;` nsmenu.m 303, `= 0;` 359 (member frame.h 307) |
| `frame_alpha` | `nsterm.swift` 2235-2236 (`(…, Int32) -> Double`) | `abi_shim.rs` 1152 | `double alpha[2];` frame.h 707-711; read nsterm.m 2378/2380 |
| `z_group_none` | `nsterm.swift` 2169 (`() -> Int32`) | `abi_shim.rs` 1182 **`mirror-named`** | `z_group_none,` frame.h 63 (`enum z_group` 61-67) = 0 |
| `z_group_above` | `nsterm.swift` 2170 | `abi_shim.rs` 1186 **`mirror-named`** | `z_group_above,` frame.h 64 = 1 |
| `z_group_below` | `nsterm.swift` 2171 | `abi_shim.rs` 1190 **`mirror-named`** | `z_group_below,` frame.h 65 = 2 |
| `z_group_above_suspended` | `nsterm.swift` 2172 | `abi_shim.rs` 1194 **`mirror-named`** | `z_group_above_suspended,` frame.h 66 = 3 |
| `FLOATP` | `nsterm.swift` 2228 (`(Lisp_Object) -> Bool`) | `abi_shim.rs` 1202 **`mirror-named`** | `#define FLOATP(x) lisp_h_FLOATP (x)` lisp.h 448 |
| `XFLOAT_DATA` | `nsterm.swift` 2229 (`(Lisp_Object) -> Double`) | `abi_shim.rs` 1208 **`mirror-named`** | `INLINE double XFLOAT_DATA (Lisp_Object f)` lisp.h 3186-3189 |
| `Qnil` | `nsfns.swift` 323, `nsterm.swift` 157, `nsmenu.swift` 367, `nsselect.swift` 190, `nsimage.swift` 131, `nsfont.swift` 532, `macfont.swift` 868 (`() -> Lisp_Object`) | `abi_shim.rs` 1231 **`mirror-named`** | `#define Qnil builtin_lisp_symbol (0)` globals.h 7363 (`iQnil` 0 at 1242) |
| `Qt` | `nsfns.swift` 324, `nsmenu.swift` 368, `nsselect.swift` 191 (`() -> Lisp_Object`) | `abi_shim.rs` 1238 **`mirror-named`** | `# define Qt builtin_lisp_symbol (1)` globals.h 7365 (`iQt` 1 at 1244) |
| `Qabove` | `nsterm.swift` 2191 (`() -> Lisp_Object`) | `abi_shim.rs` 1246 **`mirror-named`** | `# define Qabove builtin_lisp_symbol (218)` globals.h 7582 (`iQabove` 1678); `DEFSYM (Qabove, "above")` window.c 9251 |
| `Qabove_suspended` | `nsterm.swift` 2192 (`-> Lisp_Object`) | `abi_shim.rs` 1252 **`mirror-named`** | `DEFSYM (Qabove_suspended, "above-suspended")` frame.c 7368 (`iQabove_suspended` 220, globals.h 1682) |
| `Qbelow` | `nsterm.swift` 2193 (`-> Lisp_Object`) | `abi_shim.rs` 1258 **`mirror-named`** | `DEFSYM (Qbelow, "below")` frame.c 7367 (`iQbelow` 278, globals.h 1798) |
| `Qdark` | `nsterm.swift` 2194 (`-> Lisp_Object`) | `abi_shim.rs` 1264 **`mirror-named`** | `DEFSYM (Qdark, "dark")` nsfns.m 4207 (`iQdark` 447, globals.h 2136) |
| `NILP` | `nsfns.swift` 180, `nsterm.swift` 818, `nsmenu.swift` 252, `nsselect.swift` 152, `nsfont.swift` 476, `macfont.swift` 741 (`(Lisp_Object) -> Bool`) | `abi_shim.rs` 1272 **`mirror-named`** | `# define NILP(x) lisp_h_NILP (x)` lisp.h 450 (over 397, `INLINE bool (NILP)` 1447-1452) |
| `EQ` | `nsfns.swift` 181, `macfont.swift` 649, `nsfont.swift` 478, `nsimage.swift` 117, `nsmenu.swift` 251, `nsselect.swift` 151, `nsterm.swift` 137 (`(Lisp_Object, Lisp_Object) -> Bool`) | `abi_shim.rs` 1281 **`mirror-named`** | `INLINE bool EQ (Lisp_Object, Lisp_Object)` lisp.h 1314-1325 |
| `CONSP` | `nsfns.swift` 183, `nsterm.swift` 1142, `nsselect.swift` 153, `nsfont.swift` 480, `macfont.swift` 651 (`(Lisp_Object) -> Bool`) | `abi_shim.rs` 1289 **`mirror-named`** | `# define CONSP(x) lisp_h_CONSP (x)` lisp.h 446 (over 392, `INLINE bool (CONSP)` 1455-1458) |
| `SYMBOLP` | `nsfns.swift` 185, `nsterm.swift` 135, `nsselect.swift` 156, `nsfont.swift` 484, `macfont.swift` 653 (`(Lisp_Object) -> Bool`) | `abi_shim.rs` 1296 **`mirror-named`** | `INLINE bool SYMBOLP (Lisp_Object)` lisp.h 1074-1079 |
| `STRINGP` | `nsfns.swift` 184, `nsterm.swift` 2212, `nsmenu.swift` 253, `nsselect.swift` 155, `nsimage.swift` 118 (`(Lisp_Object) -> Bool`) | `abi_shim.rs` 1303 **`mirror-named`** | `INLINE bool STRINGP (Lisp_Object)` lisp.h 1581-1584 |
| `FIXNUMP` | `nsfns.swift` 190, `nsterm.swift` 2230, `nsselect.swift` 154, `nsimage.swift` 119, `nsfont.swift` 482, `macfont.swift` 655 (`(Lisp_Object) -> Bool`) | `abi_shim.rs` 1311 **`mirror-named`** | `# define FIXNUMP(x) lisp_h_FIXNUMP (x)` lisp.h 449 (over 395, `INLINE bool (FIXNUMP)` 1263-1266) |
| `XCAR` | `nsfns.swift` 195, `nsterm.swift` 1143, `nsselect.swift` 158, `nsfont.swift` 486, `macfont.swift` 784 (`(Lisp_Object) -> Lisp_Object`) | `abi_shim.rs` 1319 **`mirror-named`** | `# define XCAR(c) lisp_h_XCAR (c)` lisp.h 455 (over 405, `INLINE Lisp_Object (XCAR)` 1492-1497) |
| `XCDR` | `nsfns.swift` 196, `nsterm.swift` 1144, `nsselect.swift` 159, `nsimage.swift` 121, `nsfont.swift` 488, `macfont.swift` 785 (`(Lisp_Object) -> Lisp_Object`) | `abi_shim.rs` 1326 **`mirror-named`** | `# define XCDR(c) lisp_h_XCDR (c)` lisp.h 456 (over 406, `INLINE Lisp_Object (XCDR)` 1499-1503) |
| `XSETFRAME` | `nsfns.swift` 711, `nsmenu.swift` 272, `nsselect.swift` 181, `nsterm.swift` 1594 (`(UnsafeMutablePointer<Lisp_Object>, UnsafeMutableRawPointer?)`) | `abi_shim.rs` 1495 (crate root: the mirror spells this one as a macro) | `#define XSETFRAME(a, b) XSETPSEUDOVECTOR (a, b, PVEC_FRAME)` frame.h 926 |
| `frame_set_total_cols` | `nsfns.swift` 868-869 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1516 | `int total_cols, total_lines;` frame.h 596; `FRAME_TOTAL_COLS (tip_f) = w->total_cols;` nsfns.m 3462 |
| `frame_set_pixel_width` | `nsfns.swift` 871-872 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1525 | `int pixel_width, pixel_height;` frame.h 598; `FRAME_PIXEL_WIDTH (f) = new_native_width;` frame.c 1098 |
| `frame_set_pixel_height` | `nsfns.swift` 874-875 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1534 | `int pixel_width, pixel_height;` frame.h 598; `FRAME_PIXEL_HEIGHT (f) = new_native_height;` frame.c 1099 |
| `frame_border_width` | `nsfns.swift` 908 (`(UnsafeMutableRawPointer?) -> Int32`) | `abi_shim.rs` 1545 | `int border_width;` frame.h 613; `int border = fullscreen ? 0 : f->border_width;` nsfns.m 3560 |
| `frame_size_hint_flags` | `nsterm.swift` 1884-1885 (`-> Int32`) | `abi_shim.rs` 1554 | `int size_hint_flags;` frame.h 610; `if (f->size_hint_flags & XNegative)` nsterm.m 1827 |
| `frame_set_size_hint_flags` | `nsterm.swift` 1886-1887 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1563 | `f->size_hint_flags`'s read-modify-write clearing the two negative flags (nsterm.m 1871, 1955; the flags are `XNegative`/`YNegative`, nsterm.swift 1888-1889) |
| `frame_set_config_scroll_bar_width` | `nsfns.swift` 1155-1156 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1573 | `int config_scroll_bar_width;` frame.h 693; `FRAME_CONFIG_SCROLL_BAR_WIDTH (f) = …` nsfns.m 2530 (macro 1286) |
| `frame_set_config_scroll_bar_cols` | `nsfns.swift` 1159-1160 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1583 | `int config_scroll_bar_cols;` frame.h 694; `FRAME_CONFIG_SCROLL_BAR_COLS (f) = …` nsfns.m 2531 (macro 1296) |
| `frame_set_config_scroll_bar_height` | `nsfns.swift` 1163-1164 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1593 | `int config_scroll_bar_height;` frame.h 701; `FRAME_CONFIG_SCROLL_BAR_HEIGHT (f) = …` nsfns.m 2539 (macro 1291) |
| `frame_set_config_scroll_bar_lines` | `nsfns.swift` 1167-1168 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1603 | `int config_scroll_bar_lines;` frame.h 702; `FRAME_CONFIG_SCROLL_BAR_LINES (f) = …` nsfns.m 2540 (macro 1301) |
| `window_set_left_col` | `nsfns.swift` 917-918 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1622 | `int left_col;` window.h 248; `w->left_col = 0;` nsfns.m 3442 |
| `window_set_top_line` | `nsfns.swift` 919-920 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1630 | `int top_line;` window.h 249; `w->top_line = 0;` nsfns.m 3443 |
| `window_set_pixel_left` | `nsfns.swift` 921-922 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1638 | `int pixel_left;` window.h 243; `w->pixel_left = 0;` nsfns.m 3444 |
| `window_set_pixel_top` | `nsfns.swift` 923-924 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1646 | `int pixel_top;` window.h 244; `w->pixel_top = 0;` nsfns.m 3445 |
| `window_set_total_cols` | `nsfns.swift` 926-927 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1655 | `int total_cols;` window.h 264; `w->total_cols = XFIXNAT (XCAR (Vx_max_tooltip_size));` nsfns.m 3451, `= 80;` 3456 |
| `window_set_total_lines` | `nsfns.swift` 929-930 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1664 | `int total_lines;` window.h 265; `w->total_lines = XFIXNAT (XCDR (Vx_max_tooltip_size));` nsfns.m 3452, `= 40;` 3457 |
| `window_set_pixel_width` | `nsfns.swift` 932-933 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1673 | `int pixel_width;` window.h 252; `w->pixel_width = w->total_cols * FRAME_COLUMN_WIDTH (tip_f);` nsfns.m 3460 |
| `window_set_pixel_height` | `nsfns.swift` 935-936 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1682 | `int pixel_height;` window.h 253; `w->pixel_height = w->total_lines * FRAME_LINE_HEIGHT (tip_f);` nsfns.m 3461 |
| `window_total_cols` | `nsfns.swift` 925 (`(UnsafeMutableRawPointer?) -> Int32`) | `abi_shim.rs` 1692 | `int total_cols;` window.h 264; read nsfns.m 3460, 3462 |
| `window_total_lines` | `nsfns.swift` 928 (`-> Int32`) | `abi_shim.rs` 1701 | `int total_lines;` window.h 265; read nsfns.m 3461 |
| `window_pixel_width` | `nsfns.swift` 931 (`-> Int32`; no call site yet) | `abi_shim.rs` 1712 | `int pixel_width;` window.h 252 |
| `window_pixel_height` | `nsfns.swift` 934 (`-> Int32`) | `abi_shim.rs` 1721 | `int pixel_height;` window.h 253; `make_fixnum (w->pixel_height)` nsfns.m 3501 |
| `window_phys_cursor_x` | `nsfns.swift` 770 (`(UnsafeMutableRawPointer?) -> Int`) | `abi_shim.rs` 1734 | `struct cursor_pos phys_cursor;` window.h 332, `int x, y;` 94; `int x = text_area_x + window->phys_cursor.x;` nsfns.m 4024 |
| `window_phys_cursor_y` | `nsfns.swift` 771 (`-> Int`) | `abi_shim.rs` 1743 | `struct cursor_pos phys_cursor;` window.h 332, `int x, y;` 95; `int y = text_area_y + window->phys_cursor.y;` nsfns.m 4025 |
| `window_desired_matrix` | `nsfns.swift` 937-938 (`-> UnsafeMutableRawPointer?`) | `abi_shim.rs` 1754 | `struct glyph_matrix *desired_matrix;` window.h 225; `clear_glyph_matrix (w->desired_matrix);` nsfns.m 3476 |
| `set_hash_value_slot` | `macfont.swift` 834-835 (`(UnsafeMutableRawPointer?, Int, Lisp_Object)`) | `abi_shim.rs` 1338 **`mirror-named`** | `INLINE void set_hash_value_slot (struct Lisp_Hash_Table *, ptrdiff_t, Lisp_Object)` lisp.h 3982-3985 |
| `glyph_string_set_left_overhang` | `nsterm.swift` 3556-3557 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 1785 **`have_window_system`** | `int left_overhang;` dispextern.h 1384; `s->left_overhang = …` nsterm.m 3071, 3077 |
| `glyph_string_set_right_overhang` | `nsterm.swift` 3558-3559 (`(…, Int32)`) | `abi_shim.rs` 1797 **`have_window_system`** | `int right_overhang;` dispextern.h 1389; `s->right_overhang = …` nsterm.m 3070-3071, 3076 |
| `glyph_string_char2b` | `nsterm.swift` 3560-3561 (`(UnsafeMutableRawPointer?) -> UnsafePointer<UInt32>?`) | `abi_shim.rs` 1809 **`have_window_system`** | `unsigned *char2b;` dispextern.h 1407; read nsterm.m 3062 |
| `glyph_string_nchars` | `nsterm.swift` 3562 (`-> Int32`) | `abi_shim.rs` 1820 **`have_window_system`** | `int nchars;` dispextern.h 1408; read nsterm.m 3062 |
| `glyph_string_font` | `nsterm.swift` 3563-3564 (`-> UnsafeMutableRawPointer?`) | `abi_shim.rs` 1832 **`have_window_system`** | `struct font *font;` dispextern.h 1417; read nsterm.m 3062 |
| `glyph_string_cmp` | `nsterm.swift` 3565-3566 (`-> UnsafeMutableRawPointer?`) | `abi_shim.rs` 1843 **`have_window_system`** | `struct composition *cmp;` dispextern.h 1421; read nsterm.m 3053, 3076 |
| `glyph_string_cmp_id` | `nsterm.swift` 3567-3568 (`-> Int`) | `abi_shim.rs` 1855 **`have_window_system`** | `ptrdiff_t cmp_id;` dispextern.h 1424; read nsterm.m 3068 |
| `glyph_string_cmp_from` | `nsterm.swift` 3569-3570 (`-> Int32`) | `abi_shim.rs` 1866 **`have_window_system`** | `int cmp_from, cmp_to;` dispextern.h 1427; read nsterm.m 3069 |
| `glyph_string_cmp_to` | `nsterm.swift` 3571 (`-> Int32`) | `abi_shim.rs` 1875 **`have_window_system`** | `int cmp_from, cmp_to;` dispextern.h 1428; read nsterm.m 3069 |
| `glyph_string_first_glyph` | `nsterm.swift` 3572-3573 (`-> UnsafeMutableRawPointer?`) | `abi_shim.rs` 1887 **`have_window_system`** | `struct glyph *first_glyph;` dispextern.h 1480; read nsterm.m 3054/3055/3059 |
| `glyph_type` | `nsterm.swift` 3577 (`(UnsafeMutableRawPointer?) -> Int32`) | `abi_shim.rs` 1909 | `unsigned type : 3;` dispextern.h 514; read nsterm.m 3054/3055/3059 |
| `CHAR_GLYPH` | `nsterm.swift` 3578 (`() -> Int32`) | `abi_shim.rs` 1357 **`mirror-named`** | `CHAR_GLYPH,` dispextern.h 434 (`enum glyph_type` 431-452) = 0 |
| `COMPOSITE_GLYPH` | `nsterm.swift` 3579 (`() -> Int32`) | `abi_shim.rs` 1363 **`mirror-named`** | `COMPOSITE_GLYPH,` dispextern.h 437 = 1 |
| `glyph_row_mode_line_p` | `nsterm.swift` 3539-3540 (`(UnsafeMutableRawPointer?) -> Bool`) | `abi_shim.rs` 1920 | `bool_bf mode_line_p : 1;` dispextern.h 1086; read nsterm.m 2977 |
| `glyph_row_full_width_p` | `nsterm.swift` 3541-3542 (`-> Bool`) | `abi_shim.rs` 1930 | `bool_bf full_width_p : 1;` dispextern.h 1083; read nsterm.m 2983 |
| `glyph_row_set_redraw_fringe_bitmaps_p` | `nsterm.swift` 3543-3544 (`(…, Bool)`) | `abi_shim.rs` 1941 | `bool_bf redraw_fringe_bitmaps_p : 1;` dispextern.h 1043; write nsterm.m 2978 |
| `window_pseudo_window_p` | `nsterm.swift` 3532-3533 (`(UnsafeMutableRawPointer?) -> Bool`) | `abi_shim.rs` 1957 | `bool_bf pseudo_window_p : 1;` window.h 438 (bit 11 of the offset-500 unit); read nsterm.m 2977 |
| `composition_pixel_width` | `nsterm.swift` 3582-3583 (`-> Int32`) | `abi_shim.rs` 1976 | `short pixel_width, ascent, descent;` composite.h 153; read nsterm.m 3076 |
| `composition_lbearing` | `nsterm.swift` 3584-3585 (`-> Int32`) | `abi_shim.rs` 1986 | `short lbearing, rbearing;` composite.h 155; read nsterm.m 3077 |
| `composition_rbearing` | `nsterm.swift` 3586-3587 (`-> Int32`) | `abi_shim.rs` 1995 | `short lbearing, rbearing;` composite.h 155; read nsterm.m 3076 |
| `font_driver` | `nsterm.swift` 3596 (`(UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?`) | `abi_shim.rs` 2009 | `struct font_driver const *driver;` font.h 400; read nsterm.m 3062 |
| `font_driver_text_extents` | `nsterm.swift` 3597-3598 (`-> font_text_extents_fn?`) | `abi_shim.rs` 2024 | `void (*text_extents) (…)` font.h 672-674 (offset 104); call nsterm.m 3062 |
| `WINDOW_FRAME` | `nsterm.swift` 3747 (`(UnsafeMutableRawPointer?) -> Lisp_Object`) | `abi_shim.rs` 1381 **`mirror-named`** | `#define WINDOW_FRAME(w) ((w)->frame)` frame.h 929; read nsterm.m 3228 |
| `face_foreground` | `nsterm.swift` 3751 (`(UnsafeMutableRawPointer?) -> UInt`) | `abi_shim.rs` 2055 **`dispextern_face_records`** | `unsigned long foreground;` dispextern.h 1816; bare read nsterm.m 3279 |
| `face_background` | `nsterm.swift` 3752 (`-> UInt`) | `abi_shim.rs` 2067 **`dispextern_face_records`** | `unsigned long background;` dispextern.h 1819; bare read nsterm.m 3254/3281 |
| `gui_define_fringe_bitmap` | `nsterm.swift` 3794 (`(UnsafeMutableRawPointer?, Int32)`) | `src/fringe.rs` 2482 **`have_window_system`** - this shim's stand-in was removed in status 3.669 (P-88 (5)) | `void gui_define_fringe_bitmap (struct frame *, int)` fringe.c 1853 (declared dispextern.h 3711); call nsterm.m 3263 |
| `max_used_fringe_bitmap` | `nsterm.swift` 3796 (`var max_used_fringe_bitmap: Int32`) | `abi_shim.rs` 1397 **`mirror-named`** (the definition; `D`) | `int max_used_fringe_bitmap = MAX_STANDARD_FRINGE_BITMAPS;` fringe.c 497 (read nsterm.m 3261) |
| `draw_fringe_bitmap_params_face` | `nsterm.swift` 3757-3758 (`-> UnsafeMutableRawPointer?`) | `abi_shim.rs` 2079 **`dispextern_face_records`** | `struct face *face;` dispextern.h 2232; read nsterm.m 3229 |
| `draw_fringe_bitmap_params_which` | `nsterm.swift` 3759-3760 (`-> Int32`) | `abi_shim.rs` 2089 **`dispextern_face_records`** | `int which;` dispextern.h 2225; read nsterm.m 3235/3258/3261 |
| `draw_fringe_bitmap_params_wd` | `nsterm.swift` 3761-3762 (`-> Int32`) | `abi_shim.rs` 2099 **`dispextern_face_records`** | `int wd, h, dh;` dispextern.h 2227; read nsterm.m 3235/3238 |
| `draw_fringe_bitmap_params_h` | `nsterm.swift` 3763-3764 (`-> Int32`) | `abi_shim.rs` 2108 **`dispextern_face_records`** | `int wd, h, dh;` dispextern.h 2227; read nsterm.m 3235/3238 |
| `draw_fringe_bitmap_params_dh` | `nsterm.swift` 3765-3766 (`-> Int32`) | `abi_shim.rs` 2117 **`dispextern_face_records`** | `int wd, h, dh;` dispextern.h 2227; read nsterm.m 3235 |
| `draw_fringe_bitmap_params_x` | `nsterm.swift` 3767-3768 (`-> Int32`) | `abi_shim.rs` 2127 **`dispextern_face_records`** | `int x, y;` dispextern.h 2228; read nsterm.m 3238 |
| `draw_fringe_bitmap_params_y` | `nsterm.swift` 3769-3770 (`-> Int32`) | `abi_shim.rs` 2136 **`dispextern_face_records`** | `int x, y;` dispextern.h 2228; read nsterm.m 3238 |
| `draw_fringe_bitmap_params_bx` | `nsterm.swift` 3771-3772 (`-> Int32`) | `abi_shim.rs` 2146 **`dispextern_face_records`** | `int bx, nx, by, ny;` dispextern.h 2229; read nsterm.m 3240-3241 |
| `draw_fringe_bitmap_params_by` | `nsterm.swift` 3773-3774 (`-> Int32`) | `abi_shim.rs` 2155 **`dispextern_face_records`** | `int bx, nx, by, ny;` dispextern.h 2229; read nsterm.m 3240-3241 |
| `draw_fringe_bitmap_params_nx` | `nsterm.swift` 3775-3776 (`-> Int32`) | `abi_shim.rs` 2164 **`dispextern_face_records`** | `int bx, nx, by, ny;` dispextern.h 2229; read nsterm.m 3240-3241 |
| `draw_fringe_bitmap_params_ny` | `nsterm.swift` 3777-3778 (`-> Int32`) | `abi_shim.rs` 2173 **`dispextern_face_records`** | `int bx, nx, by, ny;` dispextern.h 2229; read nsterm.m 3240-3241 |
| `draw_fringe_bitmap_params_cursor_p` | `nsterm.swift` 3779-3780 (`-> Bool`) | `abi_shim.rs` 2187 **`dispextern_face_records`** | `bool_bf cursor_p : 1;` dispextern.h 2230 (byte 52 bit 0); read nsterm.m 3235/3278 |
| `draw_fringe_bitmap_params_overlay_p` | `nsterm.swift` 3781-3782 (`-> Bool`) | `abi_shim.rs` 2197 **`dispextern_face_records`** | `bool_bf overlay_p : 1;` dispextern.h 2231 (byte 52 bit 1); read nsterm.m 3235/3250/3280 |
| `WINDOW_XFRAME` | `nsterm.swift` 3993 (`(UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?`) | `abi_shim.rs` 2261 | `#define WINDOW_XFRAME(W) XFRAME (WINDOW_FRAME (W))` window.h 620 |
| `window_set_phys_cursor_type` | `nsterm.swift` 4001 (`(…, Int32)`) | `abi_shim.rs` 2273 **`have_window_system`** | `enum text_cursor_kinds phys_cursor_type;` window.h 344 (block 341-352) |
| `window_set_phys_cursor_on_p` | `nsterm.swift` 4003 (`(…, Bool)`) | `abi_shim.rs` 2287 | `bool_bf phys_cursor_on_p : 1;` window.h 422 (outside the block; same bit in all arms) |
| `window_phys_cursor_width` | `nsterm.swift` 4005 (`-> Int32`) | `abi_shim.rs` 2302 **`have_window_system`** | `int phys_cursor_width;` window.h 347 (block 341-352) |
| `window_set_phys_cursor_width` | `nsterm.swift` 4007 (`(…, Int32)`) | `abi_shim.rs` 2312 **`have_window_system`** | `int phys_cursor_width;` window.h 347 |
| `window_phys_cursor_hpos` | `nsterm.swift` 4009 (`-> Int32`) | `abi_shim.rs` 2325 | `struct cursor_pos phys_cursor;` window.h 332, `int hpos` 97 (before the block) |
| `glyph_row_exact_window_width_line_p` | `nsterm.swift` 4015 (`-> Bool`) | `abi_shim.rs` 2342 | `bool_bf exact_window_width_line_p : 1;` dispextern.h 1115; read nsterm.m 3332 |
| `glyph_row_used` | `nsterm.swift` 4017 (`(…, Int32) -> Int16`) | `abi_shim.rs` 2353 | `short used[1 + LAST_AREA];` dispextern.h 938; read `row->used[TEXT_AREA]` nsterm.m 3333 |
| `glyph_row_set_cursor_in_fringe_p` | `nsterm.swift` 4019 (`(…, Bool)`) | `abi_shim.rs` 2363 | `bool_bf cursor_in_fringe_p : 1;` dispextern.h 1118; write nsterm.m 3335 |
| `glyph_row_height` | `nsterm.swift` 4021 (`-> Int32`) | `abi_shim.rs` 2371 | `int ascent, height;` dispextern.h 959; read nsterm.m 3364-3365 |
| `glyph_resolved_level` | `nsterm.swift` 4024 (`-> Int32`) | `abi_shim.rs` 2381 | `unsigned resolved_level : 7;` dispextern.h 553; read nsterm.m 3357 |
| `glyph_pixel_width` | `nsterm.swift` 4025 (`-> Int16`) | `abi_shim.rs` 2390 | `short pixel_width;` dispextern.h 503; read nsterm.m 3358 |
| `FRAME_CURSOR_WIDTH` | `nsterm.swift` 4029 (`-> Int32`) | `abi_shim.rs` 1411 **`mirror-named`** | `#define FRAME_CURSOR_WIDTH(f) ((f)->cursor_width)` frame.h 1372 |
| `FRAME_FOREGROUND_PIXEL` | `nsterm.swift` 4030 (`-> UInt`) | `abi_shim.rs` 1418 **`mirror-named`** | `#define FRAME_FOREGROUND_PIXEL(f) ((f)->foreground_pixel)` frame.h 1375 |
| `DEFAULT_CURSOR` | `nsterm.swift` 4039 (`() -> Int32`) | `abi_shim.rs` 1425 **`mirror-named`** | `DEFAULT_CURSOR` dispextern.h 216-225 (`enum text_cursor_kinds`) = -2 |
| `NO_CURSOR` | `nsterm.swift` 4040 (`() -> Int32`) | `abi_shim.rs` 1430 **`mirror-named`** | `NO_CURSOR` dispextern.h 216-225 (`enum text_cursor_kinds`) = -1 |
| `FILLED_BOX_CURSOR` | `nsterm.swift` 4041 (`() -> Int32`) | `abi_shim.rs` 1435 **`mirror-named`** | `FILLED_BOX_CURSOR` dispextern.h 216-225 (`enum text_cursor_kinds`) = 0 |
| `HOLLOW_BOX_CURSOR` | `nsterm.swift` 4042 (`() -> Int32`) | `abi_shim.rs` 1440 **`mirror-named`** | `HOLLOW_BOX_CURSOR` dispextern.h 216-225 (`enum text_cursor_kinds`) = 1 |
| `BAR_CURSOR` | `nsterm.swift` 4043 (`() -> Int32`) | `abi_shim.rs` 1445 **`mirror-named`** | `BAR_CURSOR` dispextern.h 216-225 (`enum text_cursor_kinds`) = 2 |
| `HBAR_CURSOR` | `nsterm.swift` 4044 (`() -> Int32`) | `abi_shim.rs` 1450 **`mirror-named`** | `HBAR_CURSOR` dispextern.h 216-225 (`enum text_cursor_kinds`) = 3 |
| `VERTICAL_BORDER_FACE_ID` | `nsterm.swift` 4045 (`() -> Int32`) | `abi_shim.rs` 1456 **`mirror-named`** | `VERTICAL_BORDER_FACE_ID,` dispextern.h 1973 (face-id enum 1962-1976) = 12 |
| `WINDOW_DIVIDER_FACE_ID` | `nsterm.swift` 4046 (`() -> Int32`) | `abi_shim.rs` 1462 **`mirror-named`** | `WINDOW_DIVIDER_FACE_ID,` dispextern.h 1974 (face-id enum 1962-1976) = 13 |
| `WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID` | `nsterm.swift` 4047 (`() -> Int32`) | `abi_shim.rs` 1468 **`mirror-named`** | `WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID,` dispextern.h 1975 (face-id enum 1962-1976) = 14 |
| `WINDOW_DIVIDER_LAST_PIXEL_FACE_ID` | `nsterm.swift` 4049 (`() -> Int32`) | `abi_shim.rs` 1474 **`mirror-named`** | `WINDOW_DIVIDER_LAST_PIXEL_FACE_ID,` dispextern.h 1976 (face-id enum 1962-1976) = 15 |
| `get_phys_cursor_geometry` | `nsterm.swift` 4060-4063 (`(…, UnsafeMutablePointer<Int32>? ×3)`) | `abi_shim.rs` 2431 **`have_window_system`** | src/xdisp.c 2707-2771, inside `#ifdef HAVE_WINDOW_SYSTEM` (2488) |
| `draw_phys_cursor_glyph` | `nsterm.swift` 4065-4066 (`(…, Int32)`) | `abi_shim.rs` 2511 **`have_window_system`** | src/xdisp.c 35099-35145, inside `#ifdef HAVE_WINDOW_SYSTEM` (35086) |
| `draw_fringe_bitmap` | `nsterm.swift` 4068-4069 (`(…, Int32)`) | `src/fringe.rs` 1276 **`have_window_system`** - this shim's stand-in was removed in status 3.669 (P-88 (5)); the unit's is C 838-905's function, slice 1 | src/fringe.c 838-905; no build compiles

**`ns_output_*` / `ns_output_set_*` (batch 6) and the batch-7 additions**: the `have_ns` mark is the gate of `struct ns_output` (nsterm.h 935-1022, mirrored gated in `include/nsterm.rs` 419-489); `dispextern_face_records` is the gate `include/dispextern.rs` 1941 puts on `struct face`; `mirror-named` means the name is also a Rust item in a mirror, so the C-ABI spelling stands in `c_abi_of_mirror_named_items` (see `## Decisions`).  Batch 8's fifteen are the family read below (`Q*` and the accessors); fourteen carry `mirror-named`, and `XSETFRAME` is the one entry of the family whose mirror spelling is a `macro_rules!` rather than an item, so its name is free at the crate root - the split is the criterion, not an accident (`## Decisions`).

- [x] `frame_left_pos`
- [x] `frame_top_pos`
- [x] `frame_title`
- [x] `frame_icon_name`
- [x] `frame_param_alist`
- [x] `frame_selected_window`
- [x] `frame_menu_bar_vector`
- [x] `frame_tool_bar_items`
- [x] `frame_menu_bar_items_used`
- [x] `frame_n_tool_bar_items`
- [x] `frame_set_border_width`
- [x] `frame_set_child_frame_border_width`
- [x] `frame_set_internal_border_width`
- [x] `frame_set_menu_bar_lines`
- [x] `frame_set_tab_bar_lines`
- [x] `frame_set_tab_bar_height`
- [x] `frame_set_background_pixel`
- [x] `frame_set_foreground_pixel`
- [x] `frame_explicit_name`
- [x] `frame_set_explicit_name`
- [x] `frame_set_wants_modeline`
- [x] `frame_set_output_method`
- [x] `frame_tab_bar_redisplayed`
- [x] `frame_tab_bar_resized`
- [x] `frame_set_tab_bar_resized`
- [x] `frame_tab_bar_window`
- [x] `frame_set_external_menu_bar`
- [x] `frame_set_external_tool_bar`
- [x] `frame_name`
- [x] `frame_set_terminal`
- [x] `frame_set_no_split`
- [x] `frame_set_undecorated`
- [x] `frame_set_can_set_window_size`
- [x] `frame_set_was_invisible`
- [x] `window_contents`
- [x] `window_current_matrix`
- [x] `buffer_filename`
- [x] `terminal_id`
- [x] `terminal_display_info_ns`
- [x] `frame_alloc_ns_output`
- [x] `frame_set_fontset`
- [x] `frame_set_display_info`
- [x] `frame_set_double_buffered`
- [x] `ns_output_view`
- [x] `ns_output_miniimage`
- [x] `ns_output_cursor_color`
- [x] `ns_output_foreground_color`
- [x] `ns_output_background_color`
- [x] `ns_output_icon_top`
- [x] `ns_output_icon_left`
- [x] `ns_output_window_desc`
- [x] `ns_output_parent_desc`
- [x] `ns_output_explicit_parent`
- [x] `ns_output_text_cursor`
- [x] `ns_output_in_animation`
- [x] `ns_output_display_info`
- [x] `ns_output_zooming`
- [x] `ns_output_set_view`
- [x] `ns_output_set_miniimage`
- [x] `ns_output_set_cursor_color`
- [x] `ns_output_set_foreground_color`
- [x] `ns_output_set_background_color`
- [x] `ns_output_set_icon_top`
- [x] `ns_output_set_icon_left`
- [x] `ns_output_set_window_desc`
- [x] `ns_output_set_parent_desc`
- [x] `ns_output_set_explicit_parent`
- [x] `ns_output_set_current_pointer`
- [x] `ns_output_set_text_cursor`
- [x] `ns_output_set_nontext_cursor`
- [x] `ns_output_set_modeline_cursor`
- [x] `ns_output_set_hand_cursor`
- [x] `ns_output_set_hourglass_cursor`
- [x] `ns_output_set_horizontal_drag_cursor`
- [x] `ns_output_set_vertical_drag_cursor`
- [x] `ns_output_set_left_edge_cursor`
- [x] `ns_output_set_top_left_corner_cursor`
- [x] `ns_output_set_top_edge_cursor`
- [x] `ns_output_set_top_right_corner_cursor`
- [x] `ns_output_set_right_edge_cursor`
- [x] `ns_output_set_bottom_right_corner_cursor`
- [x] `ns_output_set_bottom_edge_cursor`
- [x] `ns_output_set_bottom_left_corner_cursor`
- [x] `ns_output_set_in_animation`
- [x] `ns_output_set_zooming`
- [x] `glyph_row_y`
- [x] `glyph_row_visible_height`
- [x] `set_waiting_for_input_flag`
- [x] `face_set_background`
- [x] `frame_set_no_accept_focus`
- [x] `frame_alpha`
- [x] `frame_set_menu_bar_items_used`
- [x] `z_group_none`
- [x] `z_group_above`
- [x] `z_group_below`
- [x] `z_group_above_suspended`
- [x] `FLOATP`
- [x] `XFLOAT_DATA`
- [x] `Qnil`
- [x] `Qt`
- [x] `Qabove`
- [x] `Qabove_suspended`
- [x] `Qbelow`
- [x] `Qdark`
- [x] `NILP`
- [x] `EQ`
- [x] `CONSP`
- [x] `SYMBOLP`
- [x] `STRINGP`
- [x] `FIXNUMP`
- [x] `XCAR`
- [x] `XCDR`
- [x] `XSETFRAME`
- [x] `frame_set_total_cols`
- [x] `frame_set_pixel_width`
- [x] `frame_set_pixel_height`
- [x] `frame_border_width`
- [x] `frame_size_hint_flags`
- [x] `frame_set_size_hint_flags`
- [x] `frame_set_config_scroll_bar_width`
- [x] `frame_set_config_scroll_bar_cols`
- [x] `frame_set_config_scroll_bar_height`
- [x] `frame_set_config_scroll_bar_lines`
- [x] `window_set_left_col`
- [x] `window_set_top_line`
- [x] `window_set_pixel_left`
- [x] `window_set_pixel_top`
- [x] `window_set_total_cols`
- [x] `window_set_total_lines`
- [x] `window_set_pixel_width`
- [x] `window_set_pixel_height`
- [x] `window_total_cols`
- [x] `window_total_lines`
- [x] `window_pixel_width`
- [x] `window_pixel_height`
- [x] `window_phys_cursor_x`
- [x] `window_phys_cursor_y`
- [x] `window_desired_matrix`
- [x] `set_hash_value_slot`
- [x] `glyph_string_set_left_overhang`
- [x] `glyph_string_set_right_overhang`
- [x] `glyph_string_char2b`
- [x] `glyph_string_nchars`
- [x] `glyph_string_font`
- [x] `glyph_string_cmp`
- [x] `glyph_string_cmp_id`
- [x] `glyph_string_cmp_from`
- [x] `glyph_string_cmp_to`
- [x] `glyph_string_first_glyph`
- [x] `glyph_type`
- [x] `CHAR_GLYPH`
- [x] `COMPOSITE_GLYPH`
- [x] `glyph_row_mode_line_p`
- [x] `glyph_row_full_width_p`
- [x] `glyph_row_set_redraw_fringe_bitmaps_p`
- [x] `window_pseudo_window_p`
- [x] `composition_pixel_width`
- [x] `composition_lbearing`
- [x] `composition_rbearing`
- [x] `font_driver`
- [x] `font_driver_text_extents`
- [x] `WINDOW_FRAME`
- [x] `face_foreground`
- [x] `face_background`
- [x] `gui_define_fringe_bitmap`
- [x] `max_used_fringe_bitmap`
- [x] `draw_fringe_bitmap_params_face`
- [x] `draw_fringe_bitmap_params_which`
- [x] `draw_fringe_bitmap_params_wd`
- [x] `draw_fringe_bitmap_params_h`
- [x] `draw_fringe_bitmap_params_dh`
- [x] `draw_fringe_bitmap_params_x`
- [x] `draw_fringe_bitmap_params_y`
- [x] `draw_fringe_bitmap_params_bx`
- [x] `draw_fringe_bitmap_params_by`
- [x] `draw_fringe_bitmap_params_nx`
- [x] `draw_fringe_bitmap_params_ny`
- [x] `draw_fringe_bitmap_params_cursor_p`
- [x] `draw_fringe_bitmap_params_overlay_p`
- [x] `WINDOW_XFRAME`
- [x] `window_set_phys_cursor_type`
- [x] `window_set_phys_cursor_on_p`
- [x] `window_phys_cursor_width`
- [x] `window_set_phys_cursor_width`
- [x] `window_phys_cursor_hpos`
- [x] `glyph_row_exact_window_width_line_p`
- [x] `glyph_row_used`
- [x] `glyph_row_set_cursor_in_fringe_p`
- [x] `glyph_row_height`
- [x] `glyph_resolved_level`
- [x] `glyph_pixel_width`
- [x] `FRAME_CURSOR_WIDTH`
- [x] `FRAME_FOREGROUND_PIXEL`
- [x] `DEFAULT_CURSOR`
- [x] `NO_CURSOR`
- [x] `FILLED_BOX_CURSOR`
- [x] `HOLLOW_BOX_CURSOR`
- [x] `BAR_CURSOR`
- [x] `HBAR_CURSOR`
- [x] `VERTICAL_BORDER_FACE_ID`
- [x] `WINDOW_DIVIDER_FACE_ID`
- [x] `WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID`
- [x] `WINDOW_DIVIDER_LAST_PIXEL_FACE_ID`
- [x] `get_phys_cursor_geometry`
- [x] `draw_phys_cursor_glyph`
- [x] `draw_fringe_bitmap`

**No unchecked box appears below on purpose**: a ledger that still carries one is skipped by
`xtask`'s `finished_units()`, so the shim would never be built by `gate-fast`/`dead-arms` and the
exports it already provides would read as missing.  The live tracking of what is left is the
ratchet (`docs/baseline/swift-import-pending.txt`, 668 rows at this batch; batch 13's
twenty-seven rows are among them - twenty-one go at the integrator's next `--write-pending` and
six change class, from the ruling buckets to the `rust-side-gated` record) plus the rulings
doc.
Of the ruling's 65 class-1 names, **62 stand in this file now** (`c_abi_of_mirror_named_items`
included) and 3 do not, each for a measured reason: `image_load_failed_p` and `image_pixmap` name
members of `struct image`, which `include/dispextern.rs` 1360 says is not mirrored yet (a
dispextern-lane dependency, not this lane's), and `frame_set_menu_bar_items_used` landed with
batch 7 below (it is in the table, i.e. this sentence's own 62 counted it - the three that are
left are the two `image` names and nothing else).  The names status 3.259's slice handed this lane
that are *not* landed are listed in `## Decisions` with their measurements.

## Gate audit (the 73 gated entries, against the gates of the mirror items they read)

**Why this section exists**: a gate is a load-bearing part of this file - the wrong arm exports a
name in a configuration where its mirror item does not exist, or hides one where it does, and
neither `rustc` nor any gate of this tree reports it (the frozen build compiles only the frozen
arm; the flip arms are compiled one at a time and several of them are known not to compile yet).

**Method** (one script, kept out of `tools/` on purpose - the lane's write set is this file and
this document; it ran from `/tmp` and was deleted): parse every `#[cfg (...)]` of this file's
items; extract each body's identifiers (types, called items, macros, constants - the field names
are checked by hand below); resolve each identifier's home in `src/include/*.rs` or
`lib/include/*.rs` and take *that* item's own nearest `#[cfg (...)]`; then require the
entry's gate to imply the mirror item's.  Re-run after batch 13 added six
gated entries: `# shim items: 206; gated: 79`, verdict still **0 mismatches**, distribution
`have_ns` 45, `have_ns,ns_impl_cocoa` 1, `dispextern_face_records` 16, `have_window_system`
17 - the six new rows are at the table's end.  A name declared in several mirrors
(`FRAME_OUTPUT_DATA`, `FRAME_DISPLAY_INFO`, `glyph_string`, `frame`) is a family: each copy is
another arm, so the test is "the entry's gate implies **one** copy's gate", and the other copies
are listed for the human to check (they are the x11/w32/android/haiku arms of the same macro).
The run's line is `# shim items: 179; gated: 73` and its verdict is **0 mirror-item gate
mismatches**.

**Reading, before and after this audit** (nothing in this file changed - there was nothing to
fix - so the two readings are identical):

| reading | batch-12 audit | after batch 13 | what changed |
| --- | --- | --- | --- |
| gated entries | 73 | 79 | +6 (this batch's cursor/fringe entries) |
| `have_ns` | 45 | 45 | - |
| `have_ns` + `ns_impl_cocoa` | 1 | 1 | - |
| `dispextern_face_records` | 16 | 16 | - |
| `have_window_system` | 11 | 17 | +6 |
| mismatches | 0 | 0 | - |

**Two gate *kinds* are in play, and the table says which one each row is** (this is the audit's
one substantive finding): 63 of the 79 carry a gate **C itself has** (`#ifdef HAVE_NS` over
nsterm.h's whole block, `#if defined (NS_IMPL_COCOA) && …` over the one `double_buffered`
member, `#ifdef HAVE_WINDOW_SYSTEM` over `dispextern.h` 1358-1509, and the Makefile's
`WINDOW_SYSTEM_OBJ` for fringe.c); the other 16 carry a gate the **mirror** decided on
(`dispextern_face_records`, status 3.220's arm for the records every unit keeps its own copy of)
while C has those two records ungated.  Those 16 are therefore *stricter than C*: they are hidden
in the frozen configuration although C's text has the structs there.  That is not a defect to fix
here - the export cannot exist ungated without a rival definition of the record (`E0428`), which
is exactly the shape batch 6's `face_set_background` established and status 3.220 ruled on - but
it is a real asymmetry and it is recorded here rather than left implicit.

**The audit's out-of-scope neighbour, measured** (named for the frame lane, not fixed here): the
*ungated* entries that read `include/frame.rs`'s `FRAME_*_SHIFT` constants and its
`FRAME_HAS_VERTICAL_SCROLL_BARS*` helpers reach items the mirror declares under
`#[cfg (not (feature = "have_window_system"))]` (frame.rs 278-331 and 608-620), i.e. frozen-arm-
only items.  That is why the `have_window_system` flip answers forty `E0425`s in that mirror
(batches 11 and 12 measured it).  The fix is *not* to gate those entries negatively - the C
macros exist in both arms, so hiding the names in the window-system arm would be the error this
audit is about - but to have the mirror carry that arm's shift list (status 3.264 item 6 (i)).

| 条目 | 端口的门 | 镜像项（文件:行，门） | 判据（C 的哪一行/哪个 `#if`） | 相符 |
| --- | --- | --- | --- | --- |
| `frame_alloc_ns_output` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`  (+ ungated: src/include/frame.rs:107, src/include/frame.rs:24, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `frame_set_fontset` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`  (+ ungated: src/include/frame.rs:107, src/include/frame.rs:24, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `frame_set_display_info` | `have_ns` | `src/include/nsterm.rs:318` `have_ns`, `src/include/nsterm.rs:420` `have_ns`  (+ ungated: src/include/frame.rs:107, src/include/frame.rs:24, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `frame_set_double_buffered` | `have_ns,ns_impl_cocoa` | `src/include/nsterm.rs:420` `have_ns`  (+ ungated: src/include/frame.rs:107, src/include/frame.rs:24, …) | nsterm.h 1012-1015 `#if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MIN_REQUIRED >= 101400` (member 1014) | 相符 |
| `ns_output_view` | `have_ns` | `src/include/nsterm.rs:560` `have_ns`  (+ ungated: src/include/frame.rs:107, src/include/widgetprv.rs:62) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_miniimage` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_cursor_color` | `have_ns` | `src/include/nsterm.rs:565` `have_ns`  (+ ungated: src/include/frame.rs:107, src/include/haikuterm.rs:438, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_foreground_color` | `have_ns` | `src/include/nsterm.rs:535` `have_ns`  (+ ungated: src/include/frame.rs:107, src/include/widgetprv.rs:62) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_background_color` | `have_ns` | `src/include/nsterm.rs:540` `have_ns`  (+ ungated: src/include/frame.rs:107, src/include/widgetprv.rs:62) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_icon_top` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_icon_left` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_window_desc` | `have_ns` | `src/include/nsterm.rs:525` `have_ns`  (+ ungated: src/include/frame.rs:107, src/include/widgetprv.rs:62) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_parent_desc` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_explicit_parent` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_text_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_in_animation` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_display_info` | `have_ns` | `src/include/nsterm.rs:318` `have_ns`, `src/include/nsterm.rs:515` `have_ns`  (+ ungated: src/include/androidterm.rs:472, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_zooming` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_view` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_miniimage` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_cursor_color` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_foreground_color` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_background_color` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_icon_top` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_icon_left` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_window_desc` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_parent_desc` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_explicit_parent` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_current_pointer` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_text_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_nontext_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_modeline_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_hand_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_hourglass_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_horizontal_drag_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_vertical_drag_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_left_edge_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_top_left_corner_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_top_edge_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_top_right_corner_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_right_edge_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_bottom_right_corner_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_bottom_edge_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_bottom_left_corner_cursor` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_in_animation` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `ns_output_set_zooming` | `have_ns` | `src/include/nsterm.rs:420` `have_ns`, `src/include/nsterm.rs:520` `have_ns`  (+ ungated: src/include/androidterm.rs:409, src/include/frame.rs:107, …) | nsterm.h 29 `#ifdef HAVE_NS` | 相符 |
| `face_set_background` | `dispextern_face_records` | `src/include/dispextern.rs:1985` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `glyph_string_set_left_overhang` | `have_window_system` | `src/include/dispextern.rs:1415` `have_window_system`  (+ ungated: src/include/composite.rs:227) | dispextern.h 1358-1509 `#ifdef HAVE_WINDOW_SYSTEM` | 相符 |
| `glyph_string_set_right_overhang` | `have_window_system` | `src/include/dispextern.rs:1415` `have_window_system`  (+ ungated: src/include/composite.rs:227) | dispextern.h 1358-1509 `#ifdef HAVE_WINDOW_SYSTEM` | 相符 |
| `glyph_string_char2b` | `have_window_system` | `src/include/dispextern.rs:1415` `have_window_system`  (+ ungated: src/include/composite.rs:227) | dispextern.h 1358-1509 `#ifdef HAVE_WINDOW_SYSTEM` | 相符 |
| `glyph_string_nchars` | `have_window_system` | `src/include/dispextern.rs:1415` `have_window_system`  (+ ungated: src/include/composite.rs:227) | dispextern.h 1358-1509 `#ifdef HAVE_WINDOW_SYSTEM` | 相符 |
| `glyph_string_font` | `have_window_system` | `src/include/dispextern.rs:1415` `have_window_system`  (+ ungated: src/include/composite.rs:227, src/include/font.rs:81) | dispextern.h 1358-1509 `#ifdef HAVE_WINDOW_SYSTEM` | 相符 |
| `glyph_string_cmp` | `have_window_system` | `src/include/dispextern.rs:1415` `have_window_system`  (+ ungated: src/include/composite.rs:227) | dispextern.h 1358-1509 `#ifdef HAVE_WINDOW_SYSTEM` | 相符 |
| `glyph_string_cmp_id` | `have_window_system` | `src/include/dispextern.rs:1415` `have_window_system`  (+ ungated: src/include/composite.rs:227, src/include/lisp.rs:50) | dispextern.h 1358-1509 `#ifdef HAVE_WINDOW_SYSTEM` | 相符 |
| `glyph_string_cmp_from` | `have_window_system` | `src/include/dispextern.rs:1415` `have_window_system`  (+ ungated: src/include/composite.rs:227) | dispextern.h 1358-1509 `#ifdef HAVE_WINDOW_SYSTEM` | 相符 |
| `glyph_string_cmp_to` | `have_window_system` | `src/include/dispextern.rs:1415` `have_window_system`  (+ ungated: src/include/composite.rs:227) | dispextern.h 1358-1509 `#ifdef HAVE_WINDOW_SYSTEM` | 相符 |
| `glyph_string_first_glyph` | `have_window_system` | `src/include/dispextern.rs:1415` `have_window_system`  (+ ungated: src/include/composite.rs:227) | dispextern.h 1358-1509 `#ifdef HAVE_WINDOW_SYSTEM` | 相符 |
| `face_foreground` | `dispextern_face_records` | `src/include/dispextern.rs:1985` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `face_background` | `dispextern_face_records` | `src/include/dispextern.rs:1985` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_face` | `dispextern_face_records` | `src/include/dispextern.rs:1985` `dispextern_face_records`, `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_which` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_wd` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_h` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_dh` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_x` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_y` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_bx` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_by` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_nx` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_ny` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_cursor_p` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `draw_fringe_bitmap_params_overlay_p` | `dispextern_face_records` | `src/include/dispextern.rs:2786` `dispextern_face_records` | C ungated (dispextern.h 1787-1937 / 2223-2233); the gate is the mirror's (3.220) | 相符 |
| `gui_define_fringe_bitmap` | `have_window_system` | the unit (`src/fringe.rs` 2482) since 3.669; the shim's stand-in is gone  (+ ungated: src/include/dispextern.rs:2510, src/include/frame.rs:107, …) | frozen Makefile 303/478 (`WINDOW_SYSTEM_OBJ` empty; fringe.c is in no source list) | 相符 |
| `window_set_phys_cursor_type` | `have_window_system` | `src/include/window.rs:121` `phys_cursor_type` `have_window_system` (field; cfg at 120) | window.h 344, inside the block 341-352 | 相符 |
| `window_phys_cursor_width` | `have_window_system` | `src/include/window.rs:123` `phys_cursor_width` `have_window_system` (field; cfg at 122) | window.h 347, inside the block 341-352 | 相符 |
| `window_set_phys_cursor_width` | `have_window_system` | `src/include/window.rs:123` `phys_cursor_width` `have_window_system` (field; cfg at 122) | window.h 347, inside the block 341-352 | 相符 |
| `get_phys_cursor_geometry` | `have_window_system` | `src/include/window.rs:123` `phys_cursor_width` `have_window_system` (field, read and written)  (+ ungated: src/include/termhooks.rs:425, src/include/frame.rs:107, lib/include/min-max.rs:17) | src/xdisp.c 2707-2771 inside `#ifdef HAVE_WINDOW_SYSTEM` (2488) | 相符 |
| `draw_phys_cursor_glyph` | `have_window_system` | (no gated mirror item)  (+ ungated: src/include/termhooks.rs:425, src/include/frame.rs:107) | src/xdisp.c 35099-35145 inside `#ifdef HAVE_WINDOW_SYSTEM` (35086) | 相符 |
| `draw_fringe_bitmap` | `have_window_system` | the unit (`src/fringe.rs` 1276) since 3.669; the shim's stand-in is gone  (+ ungated: src/include/termhooks.rs:425, src/include/frame.rs:107) | fringe.c

## Decisions
- **Batch 5 lands the `frame_*` family's second half plus the head of the read family: 15 exports,
  11 of which clear their ratchet row.**  The six ungated `frame_*` names (`frame_name`,
  `frame_set_terminal`, `frame_set_no_split`, `frame_set_undecorated`,
  `frame_set_can_set_window_size`, `frame_set_was_invisible`) write members the frozen record has
  (`include/frame.rs`), and the five reads (`window_contents`, `window_current_matrix`,
  `buffer_filename`, `terminal_id`, `terminal_display_info_ns`) read `struct window`/`struct
  buffer`/`struct terminal` members no Swift file declares a layout for - the lane's own headers say
  so in as many words (`nsfns.swift` 503-505 and 370-375, `nsterm.swift` 818-825).  Each flag writer
  uses the mirror's own shift constant (`FRAME_NO_SPLIT_SHIFT` 14, `FRAME_UNDECORATED_SHIFT` 25,
  `FRAME_CAN_SET_WINDOW_SIZE_SHIFT` 28, `FRAME_WAS_INVISIBLE_SHIFT` 5 in the second word), i.e. the
  same bit `frame.rs` 220-250's `no_split!`/`undecorated!`/`can_set_window_size!`/`was_invisible!`
  macros read, so writer and reader cannot drift apart unnoticed.  **The integrator deletes exactly
  eleven rows** from `docs/baseline/swift-import-pending.txt` - the gate's `STALE (satisfied)` list
  for this batch: `frame_name`, `frame_set_terminal`, `frame_set_no_split`, `frame_set_undecorated`,
  `frame_set_can_set_window_size`, `frame_set_was_invisible`, `window_contents`,
  `window_current_matrix`, `buffer_filename`, `terminal_id`, `terminal_display_info_ns` - and leaves
  the four arm-only rows below.
- **Four of the fifteen are arm-only: they carry the arm's gate and their ratchet rows stay.**  The
  three writers of `struct ns_output` members - `frame_alloc_ns_output` (nsfns.m 1270/2991:
  `f->output_data.ns = xzalloc (sizeof *f->output_data.ns);`), `frame_set_fontset` (nsfns.m
  1272/2994: `FRAME_FONTSET (f) = -1;`) and `frame_set_display_info` (nsfns.m 1280/2995:
  `FRAME_DISPLAY_INFO (f) = dpyinfo;`) - are exported under **`#[cfg (feature = "have_ns")]`**,
  because the record they write lives under that gate in the mirror (`include/nsterm.rs` 419-489;
  `ns_display_info` at 317-392).  `HAVE_NS` is undef in the frozen configuration
  (`docs/baseline/config/darwin-arm64.macros.tsv` 1481; `NS_IMPL_COCOA` 2996, `NS_IMPL_GNUSTEP`
  2997).  `frame_set_double_buffered` - the name status 3.255 item 3 ruled on - carries the mirror's
  own pair (nsterm.rs 582-584 on the macro, 487-488 on the member): `have_ns` **and**
  `ns_impl_cocoa`, because the member lives on `ns_output`, itself `have_ns`-gated; the ruling's
  shorthand `ns_impl_cocoa` alone would name a type the mirror does not have in that configuration.
  All four rows stay, with the reason re-worded to "the export exists behind the member's own gate;
  only a HAVE_NS build can clear this row" - the `frame_set_double_buffered` shape and the
  `macfont`/`nsfont` permanent-dead-arm class.  Measured: the frozen object answers none of the four
  (`nm -g build/femacs/abi_shim.o`).
- **One name of the batch list is held back outright: `frame_set_ns_transparent_titlebar`.**  Its
  member is `bool_bf ns_transparent_titlebar;` inside `#ifdef NS_IMPL_COCOA` (frame.h 735-739, the
  macro at 1259; C's write is nsfns.m 1416), and the port's `struct frame` mirror carries **no** such
  field (`include/frame.rs` 27-142's member run ends at `z_order`, 141).  With no host in the record
  there is no faithful body: an ungated export would invent a host for a member the frozen record
  does not have (P-08), and a gated one would write nothing.  The fix is the frame lane's one field
  under the same gate - the shape `include/nsterm.rs` 487-488 already uses for `double_buffered` -
  after which this export is the same one-liner as its seven siblings.  The ratchet row stays as it
  is until that field lands.
- **B class, not shim work: `FRAME_NS_VIEW`, `FRAME_DISPLAY_INFO`, `MOUSE_HL_INFO`,
  `WINDOW_TO_FRAME_PIXEL_Y`.**  Status 3.255's test applies first, and each of these bodies is a
  member read (or arithmetic over one) the Swift lane can spell at the use site, so a same-name entry
  here would put a symbol in the frozen record that C does not have (P-08).  The datum behind the
  test is in the tree: `nsfns.swift` 39-58, `nsfont.swift` 427 and `macfont.swift` 248 already
  declare `struct ns_display_info`'s prefix on the Swift side, which is what makes a record's member
  readable there; `nsterm.swift` has no `struct ns_output` declaration yet, so the two
  `ns_output`-shaped names need that decision from the Swift lane first.  The nsterm lane's own
  expansions (`nsterm.swift` 2009-2031, `frame_ns_titlebar_height`/`frame_toolbar_height`) call
  `FRAME_NS_VIEW (f)` today, so a B ruling for that name reaches those two call sites as well.  The
  rest of the `ns_output` member-macro family (`FRAME_NS_WINDOW`, `FRAME_CURSOR_COLOR`,
  `FRAME_POINTER_TYPE`, `FRAME_FONT`) is the same shape and wants the same ruling before a batch
  touches it.
- **The ratchet's reason column does not recognise this family, and one row is a false positive.**
  Fourteen of this batch's fifteen rows carry the machine reason "not found in the reference objects,
  the SDK or the reference's C and Rust text - check the spelling", and only `frame_name` carries the
  A wording - and that one is an accident of `print.c` 1915-1930's **local variable** `frame_name`,
  which the classifier's word-boundary search reads as "the reference's own C text declares or defines
  it".  The cause is structural: `tools/check-swift-imports.py`'s provider universe is the reference
  objects, the SDK, the reference's C text and the Rust tree's *item* names, while these names are the
  lanes' own C-ABI spelling of a record member (the rulings doc's class 1, "the core must export this
  C-ABI entry"), so all of them fall through to the spelling bucket.  The integrator may want to
  re-word those rows rather than leave "check the spelling" on names that are spelled exactly as the
  lanes import them.

- **One name of the batch list is held back: `frame_set_double_buffered`.**  Its C construct is
  `#define FRAME_DOUBLE_BUFFERED(f) ((f)->output_data.ns->double_buffered)` (nsterm.h 1052) inside
  `#if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MIN_REQUIRED >= 101400`, and the frozen
  configuration has `HAVE_NS`, `NS_IMPL_COCOA` and `NS_IMPL_GNUSTEP` all **undef**
  (`docs/baseline/config/darwin-arm64.macros.tsv` 1481/2996/2997; the reference's own
  `build/emacs-ref/src/config.h` 1205/2181 agree), so C never sees that member and this mirror
  gates it `#[cfg (feature = "ns_impl_cocoa")]` (`include/nsterm.rs` 487-488).  A write body can
  therefore only exist *inside* that arm; exporting it ungated would invent a host for a member the
  frozen record does not have (P-08/P-51), and gating the export on `ns_impl_cocoa` would leave the
  ratchet row unpayable in a normal build.  Ruling requested; the other twenty-seven names of the
  batches are landed.
- **Two names of the round's batch list have no importer and were not landed**: the Swift tree has
  no `@_silgen_name("frame_set_left_pos")` / `@_silgen_name("frame_set_top_pos")` anywhere
  (`rg` over `platform/macos/*.swift` finds only the two *readers*, `frame_left_pos` /
  `frame_top_pos`) and `docs/baseline/swift-import-pending.txt` has no row for either, so exporting
  them would add C-ABI surface nobody imports.  They wait for a real import (or a ruling).

- **A unit of its own, not the unit that owns the record.**  A header mirror is `include!`d into
  every consumer, so a definition written in a mirror is one definition per unit (`E0428`); and a
  `#[unsafe(no_mangle)]` name that stands beside a C file is rejected by `extract-abi.py` as an
  `invented exported symbol` (measured on a copy of `frame.rs` with one export appended: `FAIL …
  fn frame_left_pos carries #[unsafe(no_mangle)] but the reference does not define a non-static
  function of that name`).  A unit with no C counterpart is outside every `--check` pairing
  (`--check emacs/src/abi_shim.c …` answers `FileNotFoundError`), and `xtask` is the precedent for
  a product with no C source (status 3.241).
- **`build-tu` reaches it without a C file** through the `UNIT_DIRS` stem fallback:
  `build-tu src/abi_shim.c` builds `src/abi_shim.rs`, and the object lands in
  `build/femacs/abi_shim.o`, which is the tree `check-swift-imports.py` reads with `nm -g`.
- **The gate's wording is misleading, the signal is not**: once a name is provided, the tool
  prints `STALE <name>: nothing imports it any more` for its ratchet row (the row satisfied and the
  row orphaned land in the same set, `set(rows) - set(missing)`).  In this batch the ten names are
  still imported (three of them from three files); the rows simply have to go.  The integrator
  deletes them.
- **ABI = the Swift declaration.**  Where the lane's declaration and a C prototype differ, the
  Swift side wins: `UnsafeMutableRawPointer?` is received as `*mut c_void` (the lane's habit, and
  the same pointer C passes), `Int32` as `c_int`, `Bool` as `bool`, `UInt` as `c_ulong`, and the
  lanes' `Lisp_Object` typealias (`UnsafeMutableRawPointer?`, `nsfns.swift` 26) is the port's
  `Lisp_Object` (`Lisp_Word`, 8 bytes).

- **Batch 6 lands the whole `struct ns_output` family: 46 entries, 43 of them guarded, three
  clearing their rows.**  The family is **42** names (14 reads + 28 writes), not the "~20" the
  handoff estimated - the ratchet carried every one of them (`swift-import-pending.txt` 454-495 at
  batch 6's measurement; the file was re-generated at the next HEAD and they sit at 467-508 now).
  Each is an accessor of a member of `struct ns_output` (nsterm.h 935-1022), so each carries
  **`#[cfg (feature = "have_ns")]`**, C's own gate for that record (`include/nsterm.rs` 29/419-489;
  `HAVE_NS`/`NS_IMPL_COCOA`/`NS_IMPL_GNUSTEP` are undef in the frozen configuration,
  `darwin-arm64.macros.tsv` 1481/2996/2997): an ungated body cannot even name the type, and a
  no-op body would lie about what it wrote (P-08).  They are A-class by ruling 3.255's own test -
  the importing units declare no `ns_output` prefix (`nsfns.swift` 246-248 says the names stand
  for members of a record the core owns), so the expansion needs a core-internal fact.  The bodies
  spell **C's own accessor** where C has one (`FRAME_NS_VIEW`, `FRAME_NS_WINDOW`,
  `FRAME_FOREGROUND_COLOR`, `FRAME_BACKGROUND_COLOR`, `FRAME_CURSOR_COLOR`, `FRAME_DISPLAY_INFO` -
  the mirror's `macro_rules!`, nsterm.rs 513-577) and C's own `f->output_data.ns->member` chain
  (through the mirror's `FRAME_OUTPUT_DATA!`) otherwise.  The three ungated names of the batch are
  the ones the frozen configuration really owes: `glyph_row_y`/`glyph_row_visible_height`
  (`struct glyph_row`'s `y`/`visible_height`, dispextern.h 950/969 - the pair 3.257 item 5 (3)
  ruled A) and `set_waiting_for_input_flag` (thread.h 185's macro, written `= 0;` at nsmenu.m 202 -
  the body is the mirror's own `macro_rules! waiting_for_input`, so writer and reader share one
  accessor).  The 43 guarded bodies are this lane's "recorded text": measured, **not one of them is
  compiled by any configuration** (3.262's flip, below).
- **The guarded include of the record's mirror is a measured necessity, not decoration.**  The
  batch-5 guarded bodies name `ns_output`, and a unit that names a mirrored type has to reach the
  mirror (P-72); this file did not include it, so the arm could not compile *even if* the platform
  vocabulary existed.  Measured with `xtask probe src/abi_shim.rs … --cfg feature="have_ns"` on the
  same file with and without the line: **without** it the arm answers 39 x `cannot find type
  ns_output`, 36 x `cannot find macro FRAME_OUTPUT_DATA` and one error per `FRAME_*` macro;
  **with** it those are gone and the eight survivors are `include/nsterm.rs`'s own platform names
  (`NSArray`, `NSString`, `EmacsMenu`, `Window`, `NSRect`, `CGFloat`) - which is exactly what
  3.262's `dead-arms` flip measured and why the flip was reverted.  No error names a batch-6 body.
- **`face_set_background` carries `dispextern_face_records`, and its body is compiled.**  C has
  `struct face` unconditionally (dispextern.h 1787-1937), but the port's copy is
  `#[cfg (feature = "dispextern_face_records")]` (`include/dispextern.rs` 1941-1943) because the
  units that keep typed copies of it (`xfaces.rs` 288-312, `term.rs`, `alloc.rs`) are not closed
  under the flip yet (`include/frame.rs` 698-718), so the frozen configuration has no `face` for
  the body to name.  Unlike the `have_ns` block this one **compiles in its own arm**: `xtask probe
  src/abi_shim.rs … --cfg feature="dispextern_face_records"` -> exit 0, and `nm -g` on that object
  answers `T _face_set_background` (plus the ungated entries).  The row stays with the reason
  "only a `dispextern_face_records` build can clear it".
- **Batch 7 lands what status 3.259's tenth slice left, and introduces one shape: a module for
  entries whose names the mirrors already spell in Rust.**  Landed: `frame_set_no_accept_focus`
  (nsterm.m 2074, the mirror's `FRAME_NO_ACCEPT_FOCUS_SHIFT` 26 - the same bit `frame.rs` 244-247's
  macro uses), `frame_set_menu_bar_items_used` (nsmenu.m 303/359 - the ruling's last landable
  class-1 name, whose reader landed in batch 1), `frame_alpha` (`double alpha[2]`, frame.h 707-711,
  read with C's subscript at nsterm.m 2378/2380), the four `enum z_group` enumerators and the two
  `Lisp_Object` accessors `FLOATP`/`XFLOAT_DATA`.  The last six have a **name collision**: the
  mirror already declares `pub const z_group_none` (`include/frame.rs` 752-756) and
  `pub fn FLOATP`/`XFLOAT_DATA` (`include/lisp.rs` 916/1695), and a crate-root definition of the
  same name beside them is `E0428` (measured: "the name `z_group_none` is defined multiple
  times").  The C-ABI spelling therefore stands in a **module** - `c_abi_of_mirror_named_items`,
  the shape `include/thread.rs` 281-288's `thread_objs` and `include/frame.rs` 720's
  `frame_face_accessors` already use for the same reason - and each body calls the mirror's item
  through `super::`, so the value keeps one source of truth (`super::` is deliberate: a glob import
  is shadowed by the module's own item).  This shape is new to this file and is flagged for the
  integrator: it is what 3.255's ruling "A" needs when the name is already taken, and the whole
  `Lisp_Object` accessor family (`NILP`, `CONSP`, `EQ`, `STRINGP`, …) is in the same position.
- **Four names of status 3.259's list are held back, each for a measured missing host.**  C puts
  `no_focus_on_map` and `z_group` inside `#ifdef HAVE_WINDOW_SYSTEM` (frame.h 455-483) and
  `ns_appearance` (plus `enum ns_appearance_type`) inside `#ifdef NS_IMPL_COCOA` (frame.h 69-76,
  735-739); the frozen configuration has neither macro, and the mirror's shift list has the gap
  those members leave (`include/frame.rs` 146-183: no `FRAME_NO_FOCUS_ON_MAP_SHIFT`, no
  `FRAME_Z_GROUP_SHIFT`) and carries no `ns_appearance*` name at all.  So `frame_set_no_focus_on_map`,
  `frame_set_z_group`, `frame_set_ns_appearance` and the three `ns_appearance_*` enumerators wait
  for the frame lane to host those members under their own gates - the same fix
  `frame_set_ns_transparent_titlebar` (3.257 item 2) is waiting for.  Their ratchet rows stay.
- **Four names of that list are not the core's, and the handoff's classification needs one
  correction.**  Measured: `store_frame_param`, `parse_color_spec` and `error` **are** exported
  already (`nm -g build/femacs/*.o` has all three), so they need nothing.  `setAppearance` is
  `-[EmacsWindow setAppearance]`, defined by the *Swift* unit's own class at src/nsterm.m 10398
  (the call site `[window setAppearance];` is C 2144 in the same file), and
  `EmacsWindow_initWithEmacsFrame` is that class's constructor (C 1983) - both are ruling class 2
  (the unit exports its own `@_cdecl`), not C-ABI entries.  `Vframe_alpha_lower_limit` (frame.c
  7421) is a `DEFVAR_LISP` global whose storage the port gives to the unit that owns it (status
  3.29/3.252, the `Vcomp_eln_to_el_h` shape in `src/comp.rs` 40-41): the shim must **not** define a
  second copy of that symbol - the owner is `src/frame.rs`, which does not carry it yet.
  `Qabove`, `Qabove_suspended`, `Qbelow` and `Qdark` are the one real discrepancy: the handoff
  reads them as "the core already provides them", but the port exports **no `_Q*` symbol at all**
  (`nm -g build/femacs/*.o`), the mirror's `pub fn Qabove` (`include/globals.rs` 989-997) is a
  mangled Rust item, and `DEFSYM!` is a no-op macro in this port (`include/lisp.rs` 3493).  The
  Swift shape is a *call* (`nsterm.swift` 2190-2195 `func Qabove() -> Lisp_Object`), and the same
  holds for the whole `Q*` family - `Qnil` and `Qt` carry ratchet rows too - so this wants **one
  family ruling** (the shim exports the call-shaped name for all of them, or the Swift lane takes
  the data shape and a unit exports the storage), not a per-name entry.
- **Where the Swift declaration's type and C's member differ, the declaration wins and the
  conversion is explicit.**  `Window` is `typedef int` (nsgui.h 67) while the lane declares
  `ns_output_window_desc`/`ns_output_parent_desc` as `-> UInt` and their setters as `…, UInt`: the
  reads widen with `as c_ulong` and the writes narrow with `as c_int`, exact for the small counters
  C stores there (`desc_ctr++` at nsfns.m 1285, `XFIXNAT (parent)` at 1288).  `char explicit_parent`
  and `int zooming` become `bool` in the lane's declarations, so the reads are `!= 0` and the
  writes put C's own values back (`v as c_char`, `v as c_int` - C writes 1/0 at nsfns.m 1289/1294
  and nsterm.m 1907/1949/8662).  `ns_output_display_info` returns the mirror's `*mut ns_display_info`
  rather than a degraded `*mut c_void`, because that member *is* typed in the mirror (batch 5's
  `frame_set_display_info` took the degraded form because the Swift side hands in an untyped
  pointer).

- **Batch 8 lands the head of the `Lisp_Object` accessor family: 15 exports, all A class by name,
  all of which clear their ratchet row.**  The family ruling is status 3.264 item 3 (`Q*` and the
  accessors are A class with the *call* shape); this batch re-tests it name by name against status
  3.255's B-first rule and the B answer is the same for all fifteen, for a datum the lane had not
  written down before: the Swift side's `Lisp_Object` **is an opaque pointer** (`typealias
  Lisp_Object = UnsafeMutableRawPointer?`, `nsfns.swift` 29 and `macfont.swift` 38) - no tag, no
  `x` word, no layout.  So `NILP`/`CONSP`/`SYMBOLP`/`STRINGP`/`FIXNUMP`'s tag tests, `EQ`'s `XLI`
  comparison, `XCAR`/`XCDR`'s `XCONS (c)->u.s.…` reads and `XSETFRAME`'s tag-and-pointer write all
  name core-internal facts and have no Swift-side spelling.  **This batch's B list is empty**; the
  family's real B candidates are member reads whose record layout a Swift file does declare
  (`FRAME_NS_VIEW`-shaped names, ruled on in batch 5) and they are not in this family.
- **Six of the fifteen have a *data* symbol in the reference objects, and are still this lane's.**
  `_Qnil`, `_Qt`, `_Qabove`, `_Qabove_suspended`, `_Qbelow` and `_Qdark` are `S` in
  `nm -g build/emacs-ref/src/*.o` - the frozen record does define these names, so the handoff's
  `nm -g` sieve ("a real symbol -> not yours") fires and has to be read together with the family
  ruling.  Three measured facts make them the core's: the port exports **no** `_Q*` symbol at all
  (`nm -g build/femacs/*.o` at HEAD; `DEFSYM!` is a no-op macro, `include/lisp.rs` 3493), the
  mirror spells each symbol as a Rust `pub fn Q*` (`include/globals.rs`, PORTING.md P-17's call
  shape), and every Swift unit imports it as a **call** (`func Qnil() -> Lisp_Object`, seven
  units).  P-08 is not violated - the datum exists; what the port cannot hand the seam is a
  *data* symbol (the symbol-table address is not a Rust constant), so the C-ABI form of this
  family is the function, exactly as 3.264 item 3 ruled.
- **The module/crate-root split is the `mirror-named` criterion, and batch 8 measures both
  sides.**  Fourteen of the fifteen names are already Rust items in mirrors included into this
  unit, so they stand in `c_abi_of_mirror_named_items` and each body calls the mirror's own item
  through `super::` (one source of truth; the crate root would be `E0428`, measured at batch 7).
  `XSETFRAME` is the counter-example that shows the criterion is about *items*: the mirror spells
  it as `macro_rules! XSETFRAME` (`include/frame.rs` 202-204), macros live in their own
  namespace, so the name is free and the entry is an ordinary `#[unsafe(no_mangle)]` crate-root
  export whose body invokes the mirror's macro (the batch-6 `set_waiting_for_input_flag` shape).
  A name only has to move into the module when the mirror has bound it as a value/item.  Batch 10's twenty-six rows are the two records' non-bitfield members the nsfns/nsterm units
write or read (`struct frame`'s ten, `struct window`'s fifteen) plus `set_hash_value_slot`; only
the last one is `mirror-named`, because the other twenty-five are plain field accesses the
mirrors carry as *fields*, not as items (`## Decisions`).  Batch 11's twenty-two are the `glyph_string`/`glyph`/`glyph_row`/
`window`/`composition`/`font` members nsterm's fourteenth round reaches: twelve in the frozen
object, ten behind `have_window_system` (the record's own gate) and two `mirror-named`
(`## Decisions`).  Batch 12's eighteen are `WINDOW_FRAME`'s macro spelling, the two
`struct face` members, the thirteen `struct draw_fringe_bitmap_params` members, the fringe
function and the one data symbol: two in the frozen object and sixteen behind a gate
(`## Decisions`).  Batch 13's twenty-seven are the cursor/border/divider
slice: twenty-one in the frozen object (nine crate-root members, the two frame macros and the
ten enumerators) and six behind `have_window_system` (`## Decisions`, and `## Gate audit` for
which of the five window names really are gated).

- **Batch 10 lands the two records' non-bitfield member accessors the NS Swift units import: 26
  exports, all of which clear their ratchet row.**  Two of the twenty-eight names the integrator
  listed are answered without a new export, each for a measured reason rather than a preference:
  **`frame_set_fontset` is already landed** (batch 5, under `#[cfg (feature = "have_ns")]`,
  because its host is `struct ns_output`'s `fontset`, nsterm.h 1096 - its ratchet row is already
  the `rust-side-gated` record, so this batch leaves it alone); and **`set_frame_cursor_types` is
  not a member accessor at all**: it is a *function*, declared `void set_frame_cursor_types
  (struct frame *, Lisp_Object);` (src/lisp.h 4409) and defined in **src/xdisp.c 34779**
  (`T _set_frame_cursor_types` in `build/emacs-ref/src/xdisp.o`), so it is the xdisp unit's
  port-owed export and the shim must not invent a rival definition (P-08).  The Swift lane's own
  comment for that second one cites `src/frame.h 1517`, where neither the declaration nor the
  definition is - worth a fix on that side (`## Evidence` has the measurement).  The eighteen
  names of the handoff's separate skip list are untouched for the reasons it gives, each checked:
  nine are bitfields of `struct frame`/`struct window` (every one of them carries a row in
  `docs/baseline/frame-bits.tsv` / `window-bits.tsv`, i.e. status 3.272/3.273's numbering is what
  they wait for), seven are `have_ns`/`ns_impl_cocoa`/AppKit-shaped members whose host the frozen
  record does not carry, and two are `current_buffer`/`current_buffer_begv`'s P-19 rewrite.
- **`struct frame`'s ten and `struct window`'s fifteen are plain field accesses; the gate check
  is what makes them the frozen arm's.**  frame.h's `struct frame` carries no gate over the ten
  members (596-702; the file's only `#if` in that range is 668-672, over `wait_event_type`), and
  window.h's `struct window` none over the fifteen (the file's only `#if` in 60-340 is 290-303,
  `HAVE_TEXT_CONVERSION`); the mirrors carry each field ungated in the same run
  (`include/frame.rs` 185-230, `include/window.rs` 75-115, the `have_window_system` gate starting
  at 120 on `phys_cursor_type`).  So no entry carries a gate: a gate here would name a
  configuration that cannot compile it, and an ungated export of a *gated* member would invent a
  host the frozen record does not have (P-08) - the two shapes 3.255/3.257 ruled on.
- **Where the lane's declared type is wider than the member, the declaration wins with an
  explicit conversion** (the batch-6 `ns_output_window_desc` rule): `window_phys_cursor_x`/`_y`
  are declared `-> Int` (`nsfns.swift` 770-771, and the call site narrows the result with
  `Int32(window_phys_cursor_x(window))` at 4837-4838) while `struct cursor_pos`'s `x`/`y` are
  `int` (window.h 94-95), so those two bodies read the field and `as c_long` it - `c_long` being
  Darwin's 64-bit `long`, the width Swift's `Int` has.  `set_hash_value_slot`'s `idx` is the same situation in the other direction: the lane
  declares `Int` (`macfont.swift` 834), the mirror's item takes `hash_idx_t` (`include/lisp.rs`
  2262, C's `ptrdiff_t` folded to i32), so the body casts `idx as hash_idx_t`.  Every other entry
  is a straight `c_int` field against the lane's `Int32`.

- **Batch 11 lands nsterm's fourteenth slice: 22 exports, 12 of which clear their ratchet row and
  10 of which carry the record's own gate.**  The two halves are the arm question, measured in
  the C source first: `struct glyph_string` (src/dispextern.h 1358-1509, `#ifdef
  HAVE_WINDOW_SYSTEM`) does **not exist** in the frozen configuration, and the mirror carries the
  record under the same feature (`include/dispextern.rs` 1369) - so its ten accesses are exported
  behind `#[cfg (feature = "have_window_system")]`, the batch-5 `have_ns` shape and status 3.255's
  ruling (an ungated export would name a record the frozen arm does not have, P-08).  Everything
  else this batch touches is ungated in both the header and the mirror: `struct glyph`'s `type`
  bitfield (dispextern.h 514) and `enum glyph_type` (431-452) on a record the frozen arm has,
  `struct glyph_row`'s three flags (1043/1083/1086, the record ungated since batch 6),
  `struct window`'s `pseudo_window_p` (window.h 438), `struct composition`'s three shorts
  (composite.h 153/155) and `struct font`'s `driver` with `struct font_driver`'s `text_extents`
  slot (font.h 400 and 672-674; the `#ifdef HAVE_WINDOW_SYSTEM` at font.h 676 opens *after*
  `text_extents`).
- **The ten gated bodies are carried, not compiled - and the wall that stops them is
  pre-existing, in another lane's record.**  The `have_window_system` probe (`## Evidence`) exits
  1 with **48 errors, none of them in this batch**: 40 `E0425` in `include/frame.rs`'s
  window-system arm - sixteen `FRAME_*_SHIFT` names the mirror carries in the frozen layout only
  (`FRAME_NO_SPLIT_SHIFT`, `FRAME_WANTS_MODELINE_SHIFT`, `FRAME_ICONIFIED_SHIFT`,
  `FRAME_UNDECORATED_SHIFT`, `FRAME_TAB_BAR_RESIZED_SHIFT`, `FRAME_OUTPUT_METHOD_SHIFT`,
  `FRAME_EXPLICIT_NAME_SHIFT`, … - each named again in the unit's own `_root.rs` 204-362/1131,
  the batch-1/2/5/7 `frame_*` bitfield entries) plus the three
  `FRAME_HAS_VERTICAL_SCROLL_BARS*` helpers, of which rustc says "found an item that was
  configured out" - and 8 `E0080` layout-assert failures inside `include/font.rs`
  (`size_of::<font>`, `offset_of!(font, driver) == 176`, `size_of::<font_driver>`,
  `offset_of!(font_driver, otf_capability) == 120` …).  That is status 3.264 item 6 (i)'s wall seen from this side: the frame mirror carries the
  frozen shift list only, so no configuration this tree builds compiles the `have_window_system`
  arm; the ten rows therefore stay, re-classified from the ruling bucket to the `rust-side-gated`
  record ("the work is done; only a HAVE_WINDOW_SYSTEM build can clear this row").  Named for the
  frame lane, not for this one.
- **Three names of the handoff are *not* this batch's, each measured.**  `windows_or_buffers_changed`
  (`S` in `build/femacs/*.o`), `composition_gstring_from_id` and `composition_gstring_width`
  (both `T`, `src/composite.rs`) are already provided by the core, so nothing was added for
  them; the Swift lane's own comment says the same (`nsterm.swift` 3600-3603).
- **Two shapes of this batch are new spellings of rules already in this file.**  The two
  `enum glyph_type` enumerators are Rust `pub const`s at the mirror's crate root
  (`include/dispextern.rs` 240-241), so their C-ABI spelling stands in
  `c_abi_of_mirror_named_items` (status 3.264 item 2).  `window_pseudo_window_p` is a *bitfield*
  read that goes through the mirror's own packed-flag reader and bit
  (`window_bit` + `WINDOW_PSEUDO_WINDOW_P_SHIFT`, `include/window.rs` 392/165) rather than the
  `window_bits!` macro, because that macro lives in the unit `src/window.rs` 88-93 and is not
  reachable from this unit (measured: `cannot find macro \`window_bits\``) - the batch-7
  `FRAME_NO_ACCEPT_FOCUS_SHIFT` shape with the reader named explicitly.  `font_driver_text_extents`
  returns the mirror's own slot type (`include/font.rs` 511), which is ABI-identical to the lane's
  `font_text_extents_fn`; and where a declaration is wider than the member the body converts
  (`composition_*`'s `short` -> `c_int`, `glyph_string_cmp_id`'s `ptrdiff_t` -> `Int`).

- **Batch 12 lands nsterm's fifteenth slice: 18 exports, 2 of which clear their ratchet row and
  16 of which carry a gate.**  The split is the arm question, measured in the C source first.
  Ungated: `WINDOW_FRAME` (frame.h 929, the macro's own name beside round 13's `window_frame`
  member entry - two spellings by PORTING.md 0.2's names-verbatim rule) and
  `max_used_fringe_bitmap` (fringe.c 497) - **a data symbol, not a function**: the Swift lane
  imports it as a `var` (`nsterm.swift` 3796), the mirror only *declares* it
  (`include/dispextern.rs` 2510, C 3710's `extern int`), so this batch is the *definition* the
  seam needs, in the module because the name is taken (measured: `D _max_used_fringe_bitmap`,
  value 25 = C's `countof (standard_bitmaps)` over fringe.c 460-489).  Gated: fifteen members of
  `struct face` (1787-1937) and `struct draw_fringe_bitmap_params` (2223-2233), both ungated in
  C but carried by the mirror behind `dispextern_face_records` (status 3.220's arm for the
  records every unit declares its own copy of), plus `gui_define_fringe_bitmap` behind
  `have_window_system`.
- **The mirror's two copies of those records live at different paths, and the bodies say so.**
  `struct face` is a crate-root item (`include/dispextern.rs` 1985), while the five records of C
  2159-3200 sit inside the mirror's `pub mod dispextern_unit_names` (2542-3684, "because a unit
  that includes this mirror declares each of them too") - so the thirteen
  `draw_fringe_bitmap_params` bodies name the type as
  `dispextern_unit_names::draw_fringe_bitmap_params`.  Measured, and it is the one real defect
  this round caught: with the unqualified name the flip arm answers thirteen `E0425`s
  ("cannot find type `draw_fringe_bitmap_params` in this scope"), one per body, and the same
  probe is **exit 0 with all fifteen `T` names** once the path is spelled - see `## Evidence`.
- **`gui_define_fringe_bitmap` is a gated debt whose body stops at C's second guard, and that
  boundary is written down rather than hidden.**  C's definition is src/fringe.c 1853, a file the
  frozen configuration does not compile at all (Makefile's `WINDOW_SYSTEM_OBJ` empty,
  `build/emacs-ref/src/Makefile` 303/478 and `docs/baseline/frozen-build-sources.txt`; the port's
  unit is the empty `src/fringe.rs`, status 3.31), so the name is owed to a
  `HAVE_WINDOW_SYSTEM` build - status 3.255 item 3's class, the shape batch 5's `have_ns`
  exports took.  C's tail (`fb = fringe_bitmaps[n]; if (fb) rif->define_fringe_bitmap (n,
  fb->bits, fb->height, fb->width);`) needs `fringe_bitmaps` (fringe.c 496's file-scope `static
  struct fringe_bitmap **`) and `struct fringe_bitmap` (declared in fringe.c itself) - neither is
  in any mirror - and the slot read needs `struct redisplay_interface`
  (`dispextern_unit_names` 3509, gate `dispextern_face_records`).  The export therefore carries
  C's guard sequence and no call: **its purpose today is the symbol the Swift seam imports**, and
  the two statements land with the unit.  The row stays a record; this is the batch's one
  knowingly incomplete body.
- **Two bitfield members of `draw_fringe_bitmap_params` are read by measurement, not by a mirror
  accessor.**  The mirror stores C's two-bit run as `flags` (a `u8` at offset 52,
  `include/dispextern.rs` 2802) and names no accessor for it, so the bits were taken with the
  port's own probe: `spikes/gated-layout-probe.py 'struct draw_fringe_bitmap_params'
  dispextern.h c --bits` answers `52:0-0 bool_bf cursor_p` and `52:1-1 bool_bf overlay_p` in the
  frozen arm (and in the x11/ns_cocoa arms too), which is what the two bodies test.

- **Batch 13 lands nsterm's sixteenth slice - the cursor, border and divider names: 27 exports, 21
  of which clear their ratchet row and 6 of which carry `have_window_system`.**  Twenty-one are
  ungated: nine crate-root members (`WINDOW_XFRAME`'s macro, the two window flag/member reads and
  `phys_cursor.hpos`, the four `glyph_row` members, the two `glyph` members) and twelve
  `mirror-named` (the two frame macros and the ten enumerators - six `enum text_cursor_kinds`, four
  face ids - whose names the mirrors already bind as `pub fn`/`pub const`).
- **The five window names the handoff grouped under window.h's window-system block are three gated
  and two *not*, and the C text is the reason.**  window.h's `#ifdef HAVE_WINDOW_SYSTEM` block is
  **341-352** and holds exactly `phys_cursor_type` (344) and `phys_cursor_width` (347), so only
  `window_set_phys_cursor_type`, `window_phys_cursor_width` and `window_set_phys_cursor_width`
  carry the gate (the mirror's fields agree: `include/window.rs` 121/123, cfgs at 120/122).  The
  other two are ungated in every arm: `phys_cursor_on_p` is a `bool_bf … : 1` of the window's flag
  word (window.h 422, outside the block, and `docs/baseline/window-bits.tsv` line 14 gives it the
  same unit and shift in all four arms), and `phys_cursor` is `struct cursor_pos` at window.h 332,
  *before* the `#ifdef` opens (the mirror's `include/window.rs` 115 carries it ungated).  Gating
  those two would hide names C has in the frozen configuration - the error mode the gate audit
  above warns about - so they are exported ungated and their rows clear.
- **Of the three gated functions, one carries C's body whole and two stop at a named boundary.**
  `get_phys_cursor_geometry` (xdisp.c 2707-2771) is **complete**: every identifier resolves in the
  mirrors (`XFRAME`/`WINDOW_FRAME!`, `glyph_bits!`/`glyph_row_bits!`, `min`/`max` over `isize` -
  hence the explicit casts, and the first draft's two missing ones were caught by the flip arm,
  `## Evidence` - `FRAME_COLUMN_WIDTH`/`FRAME_LINE_HEIGHT`, the `WINDOW_*_LINE_HEIGHT` and
  `WINDOW_*_TO_FRAME_PIXEL_*` macros and the extern `window_text_bottom_y`, which is ported but
  declared only in the unit `src/window.rs`, so this file declares the symbol it calls).  Its one
  **link-time dependency is named**: it reads `x_stretch_cursor_p` (xdisp.c's `DEFVAR_BOOL`, whose
  storage is the xdisp unit's; the mirror declares it at `include/globals.rs` 7200 and no unit
  defines it yet), so a `have_window_system` build of this object would carry that one undefined
  reference - the frozen arm does not compile the body, so the frozen object's undefined list is
  unchanged (`## Evidence`).  `draw_phys_cursor_glyph` and `draw_fringe_bitmap` are the two
  **boundary** cases: the first carries C's `hpos` bounds guard and clamp and stops at
  `x1 = draw_glyphs (…)` (`draw_glyphs` is xdisp.c 35158's own function, with no mirror, unit or
  declaration in this tree), the second carries none of C's statements (its first switch needs
  `STANDARD_BITMAP_HEIGHT`/`standard_bitmaps`, `get_logical_cursor_bitmap` and
  `draw_fringe_bitmap_1`, all `fringe.c`'s own).  Both keep the **symbol** the Swift seam imports
  and the row as a record; the code comments say in as many words what is missing.

## Evidence
- **Batch 13 (27 entries: 21 in the frozen object, 6 behind `have_window_system`)**: `cd femacs
  && CARGO_TARGET_DIR=/tmp/femacs-abi13 cargo run -q -p xtask -- build-tu src/abi_shim.c` ->
  `built src/abi_shim.rs -> ../build/femacs/abi_shim.o` (exit status **0**, no `error:` line).
  `nm -g build/femacs/abi_shim.o` answers the twenty-one as `T` (verbatim, by family:
  `_BAR_CURSOR` 0x0000, `_DEFAULT_CURSOR` 0x0028, `_FILLED_BOX_CURSOR` 0x00a0,
  `_FRAME_CURSOR_WIDTH` 0x01a0, `_FRAME_FOREGROUND_PIXEL` 0x0204, `_HBAR_CURSOR` 0x0268,
  `_HOLLOW_BOX_CURSOR` 0x0270, `_NO_CURSOR` 0x0284, `_VERTICAL_BORDER_FACE_ID` 0x03f8,
  `_WINDOW_DIVIDER_FACE_ID` 0x0400, `_WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID` 0x0408,
  `_WINDOW_DIVIDER_LAST_PIXEL_FACE_ID` 0x0410, `_WINDOW_XFRAME` 0x047c, `_glyph_pixel_width`
  0x3160, `_glyph_resolved_level` 0x31cc, `_glyph_row_exact_window_width_line_p` 0x3290,
  `_glyph_row_height` 0x3378, `_glyph_row_set_cursor_in_fringe_p` 0x3458, `_glyph_row_used`
  0x3540, `_window_phys_cursor_hpos` 0x3b64, `_window_set_phys_cursor_on_p` 0x3e64) and
  **none** of the six gated ones.  The object now holds **127** C-ABI names out of **150** `T`
  symbols plus batch 12's one `D` (106 / 129 before this batch), and `nm -u` is the same 18
  entries as at batches 10-12 - the gated bodies are not compiled in the frozen arm, so they add
  no undefined reference there.
- **The `have_window_system` flip, and the two defects this batch's own code had in its first
  run**: `xtask probe src/abi_shim.rs /tmp/abi_shim.ws13.o --crate-type lib --emit obj --cfg
  'feature="have_window_system"' -C panic=abort -C codegen-units=1 -C opt-level=0` -> **exit 1,
  51 errors**: `43` `E0425` (every one a `FRAME_*` name of `include/frame.rs`'s frozen-only arm -
  the batch-11/12 wall, three occurrences more than batch 12's forty because the compile now
  reaches further) and `8` `E0080` (`include/font.rs`'s layout asserts) - **no error names this
  batch**.  The first run of the batch was not clean and is recorded as measured: it answered
  `53` errors including **two `E0308` in `get_phys_cursor_geometry`** (`_root.rs` 2478: `max`
  takes `isize`, and two arguments had no cast) plus one `E0758` "unterminated block comment"
  from a `/*` inside a block comment (Rust nests them; the `.o` path in a comment tripped it).
  Both are fixed, and rustc's report of them is the proof that these gated bodies **are**
  type-checked in the flip arm even though the arm's own errors stop the build - the strongest
  reading available for a body whose configuration cannot compile yet.
- **The gate audit, re-run over the batch's six new gated entries**: `# shim items: 206; gated:
  79`, verdict **0 mirror-item gate mismatches**, distribution `have_ns` 45,
  `have_ns,ns_impl_cocoa` 1, `dispextern_face_records` 16, `have_window_system` 17 - the six new
  rows are at `## Gate audit`'s table end, and the three window ones name the *fields* their C
  block holds (`include/window.rs` 121/123), the two remaining ones name no gated mirror item
  because their bodies carry no gated member.
- **`check-swift-imports.py`**: twenty-one `STALE (satisfied)` lines, exactly this batch's
  ungated twenty-one and no other row of this shim's.  The six gated names move to the
  `rust-side-gated` class, whose count reads **73** in this run (70 at the audit's reading; the
  window also carries other lanes' churn, so the delta is not exactly six).  Tail:
  `977 import(s) in 7 Swift file(s), 5726 core export(s), 642 unprovided, 663 ratchet row(s), 21
  failure(s)` (the ledger file itself is at 668 rows after the integrator's regeneration for this
  batch's 27 names).
- **The other four gates, re-run in this tree**: `check-globals` -> `284 units, 0 DEFVAR_*
  variables without a definition in their own unit`; `check-mirror-dupes` -> `1 duplicate
  declaration(s), 1 ratchet row(s), 0 failure(s)`; `check-declarations` -> `13 declaration sites
  (0 forbidden, 13 allowed literals), 0 failures`; `check-target-names` -> `628 .rs files, 591 map
  targets, 0 finding(s)`; `check-undef.py` -> 0 references out of `abi_shim.o` (its 14 `NEW` rows
  are xdisp.o's).
- **Batch 12 (18 entries: 2 in the frozen object, 16 behind a gate)**: `cd femacs &&
  CARGO_TARGET_DIR=/tmp/femacs-abi12 cargo run -q -p xtask -- build-tu src/abi_shim.c` ->
  `built src/abi_shim.rs -> ../build/femacs/abi_shim.o` (exit status **0**, no `error:` line).
  `nm -g build/femacs/abi_shim.o` answers the two, verbatim: `0000000000000300 T _WINDOW_FRAME`
  and `0000000000004ee8 D _max_used_fringe_bitmap` - note the second is **`D`**, not `T`: it is
  storage, and the tool's provider set counts it (its row is one of the two `STALE (satisfied)`
  lines below).  The object now holds **106** C-ABI names out of **129** `T` symbols plus that
  one `D` (104 / 128 before this batch), and answers none of the sixteen gated ones.
- **The fifteen `dispextern_face_records` entries are compiled, measured**:
  `xtask probe src/abi_shim.rs /tmp/abi_shim.face12.o --crate-type lib --emit obj --cfg
  'feature="dispextern_face_records"' -C panic=abort -C codegen-units=1 -C opt-level=0` ->
  **exit 0, zero errors**, and `nm -g` on that object answers all fifteen `T` names
  (`_face_foreground`, `_face_background` and the thirteen `_draw_fringe_bitmap_params_*`).
  The counterfactual that found this batch's defect is in `## Decisions`: with the type spelled
  without its module path the same probe answers thirteen `E0425`s at `_root.rs` 1998-2116.
- **The one `have_window_system` entry, and its wall**: the same probe with
  `--cfg 'feature="have_window_system"'` -> **exit 1, 48 errors** - byte-for-byte the reading
  batch 11 recorded for the ten `glyph_string_*` bodies (forty `E0425`s in `include/frame.rs`'s
  window-system arm plus the eight `E0080` layout asserts in `include/font.rs`) - and **no error
  names `gui_define_fringe_bitmap`** (`grep -c` = 0).  So its body is carried, not compiled: the
  wall is the frame mirror's window-system arm, not this batch.
- **The two bit positions, measured with the port's own probe**: `python3
  spikes/gated-layout-probe.py 'struct draw_fringe_bitmap_params' dispextern.h c --bits` ->
  `52:0-0 | bool_bf cursor_p` and `52:1-1 | bool_bf overlay_p` (frozen arm; the x11 and ns_cocoa
  arms agree), over `sizeof=64, align=8` - the numbers the two bodies' masks use.
- **`check-swift-imports.py`**: two `STALE (satisfied)` lines for this batch - `WINDOW_FRAME` and
  `max_used_fringe_bitmap` - and no other new row of this shim's (the other twelve lines in the
  run are batch 11's, whose rows the integrator's `--write-pending` has not dropped yet in this
  worktree).  The sixteen gated names move to the `rust-side-gated` class, whose count reads
  **70** in this run (57 at batch 11's reading; the window also carries other lanes' churn).
  Tail: `949 import(s) in 7 Swift file(s), 5705 core export(s), 636 unprovided, 650 ratchet
  row(s), 14 failure(s)`.
- **The object's own undefined list did not change**: `nm -u build/femacs/abi_shim.o` is the same
  18 entries as at batches 10 and 11, so `tools/check-undef.py` reports **0** new references out
  of `abi_shim.o` (its 14 `NEW` rows are xdisp.o's, its 3 stale rows that lane's).
- **The other four gates, re-run in this tree**: `check-globals` -> `284 units, 0 DEFVAR_*
  variables without a definition in their own unit`; `check-mirror-dupes` -> `1 duplicate
  declaration(s), 1 ratchet row(s), 0 failure(s)` (the `extern` declaration in the mirror and
  this batch's definition do not collide); `check-declarations` -> `13 declaration sites
  (0 forbidden, 13 allowed literals), 0 failures`; `check-target-names` -> `628 .rs files, 591
  map targets, 0 finding(s)` (the file count moved with another lane's new `include/xlib.rs`, not
  with this batch).
- **Batch 11 (22 entries: 12 in the frozen object, 10 behind `have_window_system`)**: `cd femacs
  && CARGO_TARGET_DIR=/tmp/femacs-abi11 cargo run -q -p xtask -- build-tu src/abi_shim.c` ->
  `built src/abi_shim.rs -> ../build/femacs/abi_shim.o` (exit status **0**, no `error:` line).
  `nm -g build/femacs/abi_shim.o` answers the twelve as `T`, verbatim: `T _font_driver` 0x1a38,
  `T _font_driver_text_extents` 0x1aa4, `T _glyph_type` 0x3188, `T _CHAR_GLYPH` 0x0000,
  `T _COMPOSITE_GLYPH` 0x0008, `T _window_pseudo_window_p` 0x37dc, `T _glyph_row_mode_line_p`
  0x2fc8, `T _glyph_row_full_width_p` 0x2f54, `T _glyph_row_set_redraw_fringe_bitmaps_p` 0x303c,
  `T _composition_pixel_width` 0x1960, `T _composition_lbearing` 0x18f4,
  `T _composition_rbearing` 0x19cc - and **none** of the ten `_glyph_string_*` names (the gate,
  as expected).  The object now holds **104** C-ABI names out of **128** `T` symbols (92 / 116
  before this batch).
- **The ten behind the gate, and the wall in front of them**:
  `xtask probe src/abi_shim.rs /tmp/abi_shim.winsys.o --crate-type lib --emit obj --cfg
  'feature="have_window_system"' -C panic=abort -C codegen-units=1 -C opt-level=0` -> **exit 1**,
  `48` errors: `34 + 6` `E0425` (the sixteen `FRAME_*_SHIFT` names - seven of them three times,
  three twice, six once: `FRAME_WANTS_MODELINE_SHIFT`, `FRAME_UNDECORATED_SHIFT`,
  `FRAME_TAB_BAR_RESIZED_SHIFT`, `FRAME_OUTPUT_METHOD_SHIFT`, `FRAME_NO_SPLIT_SHIFT`,
  `FRAME_ICONIFIED_SHIFT`, `FRAME_EXPLICIT_NAME_SHIFT`, `FRAME_WAS_INVISIBLE_SHIFT`,
  `FRAME_NO_ACCEPT_FOCUS_SHIFT`, `FRAME_CAN_SET_WINDOW_SIZE_SHIFT`, `FRAME_WINDOW_CHANGE_SHIFT`,
  `FRAME_WINDOW_STATE_CHANGE_SHIFT`, `FRAME_VISIBLE_SHIFT`, `FRAME_TAB_BAR_REDISPLAYED_SHIFT`,
  `FRAME_GARBAGED_SHIFT`, `FRAME_FROZEN_WINDOW_STARTS_SHIFT`, `FRAME_NO_SPECIAL_GLYPHS_SHIFT` -
  plus `FRAME_HAS_VERTICAL_SCROLL_BARS{,_ON_LEFT,_ON_RIGHT}`, where rustc's note is "found an
  item that was configured out") and `8` `E0080` (`include/font.rs`'s layout asserts:
  `size_of::<font>`, `offset_of!(font, driver) == 176`, `size_of::<font_driver>`,
  `offset_of!(font_driver, otf_capability) == 120` …).  Every
  error's location is either `_root.rs`'s pre-batch-11 region (204-1131, this batch's entries
  start at 1357 in the module and 1673 at the crate root) or a mirror; **no error names a
  batch-11 entry**.  So the ten bodies are carried and not compiled - the 3.262 shape - and the
  wall is the frame mirror's window-system arm, which status 3.264 item 6 (i) already assigned to
  the frame lane.
- **`check-swift-imports.py`**: twelve `STALE (satisfied)` lines, exactly this batch's ungated
  twelve (`CHAR_GLYPH`, `COMPOSITE_GLYPH`, `composition_lbearing`, `composition_pixel_width`,
  `composition_rbearing`, `font_driver`, `font_driver_text_extents`, `glyph_row_full_width_p`,
  `glyph_row_mode_line_p`, `glyph_row_set_redraw_fringe_bitmaps_p`, `glyph_type`,
  `window_pseudo_window_p`) and no other row of this shim's; the ten gated names moved out of the
  ruling bucket - the tail's class counts go from `rust-side-gated=47` to `rust-side-gated=**57**`
  (the tool reads the `#[cfg (feature = "have_window_system")]` above each declaration), which is
  the batch's own signal on the row side.  Tail: `949 import(s) in 7 Swift file(s), 5699 core
  export(s), 638 unprovided, 632 ratchet row(s), 30 failure(s)` (the failures are other lanes'
  in-flight Swift).
- **The object's own undefined list did not change**: `nm -u build/femacs/abi_shim.o` is the same
  18 entries as at batch 10 (the eight non-runtime names `_setjmp`, `_cons_to_signed`,
  `_cons_to_unsigned`, `_current_thread`, `_slow_eq`, `_symbols_with_pos_enabled`,
  `_wrong_type_argument`, `_xfree` plus the ten Rust-runtime panic symbols), so
  `tools/check-undef.py` reports **0** new references out of `abi_shim.o` (its 14 `NEW` rows are
  xdisp.o's, its 3 stale rows that lane's).
- **The three names the handoff excluded are provided, measured**: `nm -g build/femacs/*.o`
  answers `S _windows_or_buffers_changed` and `T _composition_gstring_from_id` /
  `T _composition_gstring_width`, so no row was added for them.
- **The other four gates, re-run in this tree**: `check-globals` -> `284 units, 0 DEFVAR_*
  variables without a definition in their own unit`; `check-mirror-dupes` -> `1 duplicate
  declaration(s), 1 ratchet row(s), 0 failure(s)`; `check-declarations` -> `13 declaration sites
  (0 forbidden, 13 allowed literals), 0 failures`; `check-target-names` -> `627 .rs files, 591 map
  targets, 0 finding(s)`.
- **Batch 10 (26 entries, 26 ratchet rows cleared)**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-abi10
  cargo run -q -p xtask -- build-tu src/abi_shim.c` -> `built src/abi_shim.rs ->
  ../build/femacs/abi_shim.o` (exit status **0**, no `error:`; 2789 warnings, the tree's own).
  `nm -g build/femacs/abi_shim.o` answers all twenty-six as `T`, verbatim:
  `T _frame_set_pixel_width` 0x25cc, `T _frame_set_pixel_height` 0x2558,
  `T _frame_set_total_cols` 0x288c, `T _frame_border_width` 0x198c,
  `T _frame_size_hint_flags` 0x2a80, `T _frame_set_size_hint_flags` 0x2640,
  `T _frame_set_config_scroll_bar_cols` 0x1fac, `T _frame_set_config_scroll_bar_height` 0x2020,
  `T _frame_set_config_scroll_bar_lines` 0x2094, `T _frame_set_config_scroll_bar_width` 0x2108,
  `T _window_set_pixel_width` 0x3560, `T _window_set_pixel_height` 0x3404,
  `T _window_set_pixel_left` 0x3478, `T _window_set_pixel_top` 0x34ec,
  `T _window_set_total_cols` 0x3648, `T _window_set_total_lines` 0x36bc,
  `T _window_set_left_col` 0x3390, `T _window_set_top_line` 0x35d4,
  `T _window_phys_cursor_x` 0x31e0, `T _window_phys_cursor_y` 0x324c,
  `T _window_pixel_width` 0x3324, `T _window_pixel_height` 0x32b8,
  `T _window_total_cols` 0x3730, `T _window_total_lines` 0x379c,
  `T _window_desired_matrix` 0x3174, `T _set_hash_value_slot` 0x2e00.  The object now holds
  **92** C-ABI names out of **116** `T` symbols (66 / 90 before this batch) and still answers none
  of the 47 guarded ones.
- **The sieve that answered the two list entries this batch does *not* export**:
  `nm -g build/emacs-ref/src/*.o` answers `T _set_frame_cursor_types` (in `xdisp.o`; the
  definition is src/xdisp.c 34779, the declaration src/lisp.h 4409 - `grep -n
  set_frame_cursor_types emacs/src/frame.h` is empty, so the Swift comment's `frame.h 1517`
  citation is a defect of that lane's, not a disagreement about the symbol), and `frame_set_fontset`
  is already this file's (batch 5) with its ratchet row in the `rust-side-gated` class.  Neither is
  a new export, and both are named in `## Decisions`.
- **`check-swift-imports.py`**: twenty-seven `STALE (satisfied)` lines, twenty-six of them this
  batch's names and the twenty-seventh another lane's (`ns_clear_frame`); the tail is
  `907 import(s) in 7 Swift file(s), 5687 core export(s), 610 unprovided, 621 ratchet row(s), 43
  failure(s)` - the export count includes this batch's 26, and the 43 failures (and the 55 rows
  the tool asks a ruling for) are the in-flight Swift batches of other lanes in this worktree.
- **The object's own undefined list grew by exactly one name**: 17 entries at HEAD, 18 now.  The
  addition is the tenth Rust-runtime panic symbol, `_panic_const_mul_overflow`, which is the only
  arithmetic this batch's bodies inline - the mirror's `2 * idx as usize` inside
  `set_hash_value_slot` (`include/lisp.rs` 2506); the other eight non-panic names are unchanged
  (`_setjmp`, `_cons_to_signed`, `_cons_to_unsigned`, `_current_thread`, `_slow_eq`,
  `_symbols_with_pos_enabled`, `_wrong_type_argument`, `_xfree`, all satisfied by other unit
  objects).  `tools/check-undef.py` therefore reports **0** new references out of `abi_shim.o`
  (its 14 `NEW` rows are xdisp.o's).
- **The twenty-six bodies compile in both configurations**: `xtask probe src/abi_shim.rs
  /tmp/abi_shim.checking10.o --crate-type lib --emit obj --cfg enable_checking -C panic=abort -C
  codegen-units=1 -C opt-level=0` -> exit 0, no `error:`; `nm -g` on that object answers the same
  twenty-six `T` names and its undefined list contains `_die`, i.e. the `eassert!` bodies (the
  mirror's `set_hash_value_slot` among them) are compiled there.
- **The other four gates, re-run in this tree**: `check-globals` -> `284 units, 0 DEFVAR_*
  variables without a definition in their own unit`; `check-mirror-dupes` -> `1 duplicate
  declaration(s), 1 ratchet row(s), 0 failure(s)`; `check-declarations` -> `13 declaration sites
  (0 forbidden, 13 allowed literals), 0 failures`; `check-target-names` -> `627 .rs files, 591 map
  targets, 0 finding(s)`.
- **Batch 8 (15 entries, 15 ratchet rows cleared)**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-abi8
  cargo run -q -p xtask -- build-tu src/abi_shim.c` -> `built src/abi_shim.rs ->
  ../build/femacs/abi_shim.o` (exit status **0**, no `error:` line; the warning stream is the
  tree's - 2789 warnings, none naming a batch-8 entry - and rustc's trailing `rustc --explain
  E0133` hint belongs to that pre-existing `warning[E0133]` lint stream, not to a failure).
  `nm -g build/femacs/abi_shim.o` answers all fifteen as
  `T`, verbatim: `0000000000000000 T _CONSP`, `0000000000000010 T _EQ`,
  `0000000000000080 T _FIXNUMP`, `0000000000000178 T _NILP`, `0000000000000184 T _Qabove`,
  `0000000000000190 T _Qabove_suspended`, `000000000000019c T _Qbelow`,
  `00000000000001a8 T _Qdark`, `00000000000001b4 T _Qnil`, `00000000000001bc T _Qt`,
  `00000000000001c8 T _STRINGP`, `00000000000001d8 T _SYMBOLP`, `00000000000002f0 T _XCAR`,
  `0000000000000378 T _XCDR`, `0000000000000488 T _XSETFRAME`.  The object now holds **66** C-ABI
  names out of **90** `T` symbols (51 / 75 before this batch) and still answers none of the 47
  guarded ones.
- **The handoff's `nm -g` sieve, run name by name against the reference objects**: the nine
  accessors have **no** symbol in `build/emacs-ref/{src,lib}/*.o` at all (they are lisp.h's
  macros and INLINEs - 3.251's A-class test), and the six `Q*` names have **data** (`S`) symbols
  only: `_Qnil` 0x3168, `_Qt` 0x3170, `_Qabove` 0x3838, `_Qabove_suspended` 0x3848,
  `_Qbelow` 0x3a18, `_Qdark` 0x3f60.  None of the fifteen is a *function* symbol in the reference.
  The port side of the same test: at HEAD `nm -g build/femacs/*.o` answered **no** `_Q*` symbol,
  and `DEFSYM!` is `{}` (`include/lisp.rs` 3493) - which is why the call shape is the only shape
  the seam can get (see `## Decisions`).
- **`check-swift-imports.py`**: exactly fifteen `STALE (satisfied)` lines, and they are this
  batch's fifteen and no other row of this shim's - `CONSP`, `EQ`, `FIXNUMP`, `NILP`, `Qabove`,
  `Qabove_suspended`, `Qbelow`, `Qdark`, `Qnil`, `Qt`, `STRINGP`, `SYMBOLP`, `XCAR`, `XCDR`,
  `XSETFRAME` ("provided now, so the row goes").  Tail:
  `877 import(s) in 7 Swift file(s), 5656 core export(s), 616 unprovided, 537 ratchet row(s), 109
  failure(s)` - the failures and the `unclassified` rows are the in-flight Swift batches of other
  lanes in this worktree, unchanged by this batch.
- **The object's own undefined list grew by exactly two names, and both are satisfied**: at HEAD
  `nm -u build/femacs/abi_shim.o` was 15 entries (nine Rust-runtime panic symbols, `_setjmp`,
  `_cons_to_signed`, `_cons_to_unsigned`, `_current_thread`, `_wrong_type_argument`, `_xfree`);
  with batch 8 it is 17, the two new ones being `_slow_eq` and `_symbols_with_pos_enabled`, which
  **only** this batch's `EQ` body reaches (its non-inline arm; every other body of the file
  inlines `BASE_EQ`/`TAGGEDP`).  Both are defined by other unit objects (`nm -g build/femacs/*.o`
  answers `T _slow_eq` / `_symbols_with_pos_enabled`), so `tools/check-undef.py` reports **0** new
  references out of `abi_shim.o`: its `NEW` list is 14 entries, every one of them `referenced by
  xdisp.o` (another lane's unit), and its three stale ratchet rows (`_Vcomp_eln_to_el_h`,
  `_redisplay_internal`, `_try_window`) are that lane's too.
- **The batch-8 bodies are compiled in *both* configurations - the one reading this file's
  history has never had**: `xtask probe src/abi_shim.rs /tmp/abi_shim.checking.o --crate-type lib
  --emit obj --cfg enable_checking -C panic=abort -C codegen-units=1 -C opt-level=0` -> exit 0,
  and `nm -g` on that object answers the same fifteen `T` names, with **`_die`** now in its
  undefined list (absent from the frozen object's 17) - i.e. the `eassert!` arm of
  `XSETFRAME`'s expansion (`XSETTYPED_PSEUDOVECTOR!`, `include/lisp.rs` 3085-3089) is live there.
  All fifteen entries are ungated, so unlike the 47 guarded bodies of batches 5-7 these bodies are
  compiled by the frozen build itself, not merely carried.
- **The other four gates, re-run in this tree**: `check-globals` -> `284 units, 0 DEFVAR_*
  variables without a definition in their own unit`; `check-mirror-dupes` -> `1 duplicate
  declaration(s), 1 ratchet row(s), 0 failure(s)`; `check-declarations` -> `13 declaration sites
  (0 forbidden, 13 allowed literals), 0 failures`; `check-target-names` -> `627 .rs files, 591 map
  targets, 0 finding(s)` (the `DECLARED` first line still keeps this file out of rule R1).
- **Batches 6 and 7 (46 + 9 entries, 12 ratchet rows cleared)**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-abi cargo run -q -p xtask -- build-tu src/abi_shim.c` -> `built src/abi_shim.rs -> ../build/femacs/abi_shim.o` (exit 0, no `error:`; the warning stream is the tree's, 2793 of them, none naming a batch-6/7 entry).  `nm -g build/femacs/abi_shim.o` answers **51** C-ABI names out of 75 `T` symbols, and answers **none** of the 47 guarded ones (the expected reading for the frozen configuration): the new ungated twelve are `_glyph_row_y`, `_glyph_row_visible_height`, `_set_waiting_for_input_flag`, `_frame_set_no_accept_focus`, `_frame_alpha`, `_frame_set_menu_bar_items_used`, `_z_group_none`, `_z_group_above`, `_z_group_below`, `_z_group_above_suspended`, `_FLOATP`, `_XFLOAT_DATA`; `_face_set_background` and the 42 `_ns_output_*` names are absent.  `tools/check-swift-imports.py` prints exactly twelve `STALE (satisfied)` lines - those twelve names and no other row of this shim's - and its tail is
  `877 import(s) in 7 Swift file(s), 5641 core export(s), 631 unprovided, 549 ratchet row(s), 106 failure(s)`, the failures being the in-flight Swift batches of other lanes.
- **The two gates of the guarded blocks, measured**: `xtask probe src/abi_shim.rs … --cfg feature="dispextern_face_records"` -> **exit 0**, and `nm -g` on that object answers `T _face_set_background` (so that body is compiled, not merely carried); the same probe with `--cfg feature="have_ns"` -> exit 1 with **eight** errors, all inside `include/nsterm.rs` (`NSArray` x2, `NSString`, `EmacsMenu`, `Window` x3, `NSRect`), **none of them naming a batch-6/7 body** - the reading status 3.262 recorded when the `dead-arms` flip was tried and reverted.  The counterfactual for the guarded include is in `## Decisions` (39 + 36 errors without it).
- **The `E0428` the mirror-named group avoids, and the module that compiles**: a file with
  `pub const z_group_none: i32 = 0;` beside `#[unsafe(no_mangle)] pub unsafe extern "C" fn z_group_none ()` answers `error[E0428]: the name z_group_none is defined multiple times`; the same pair in a module compiles and exports both symbols (measured on a two-name probe).  In this file the module is `c_abi_of_mirror_named_items` and `nm -g build/femacs/abi_shim.o` answers its six names.
- **No reference symbol was invented, and the object's own undefined list grew by exactly one
  name**: none of the 55 new names is a symbol in `build/emacs-ref/{src,lib}/*.o` (checked name by
  name against the object symbol list, which is the A-class test status 3.251 states), and
  `nm -u build/femacs/abi_shim.o` is the previous twelve entries plus **`_current_thread`** - the
  flag writer reads `current_thread`, which the port defines in `src/thread.rs` 171 (`D` in
  `build/femacs/thread.o`), so nothing new is unsatisfiable.  `tools/check-undef.py` reports
  `14 NEW undefined reference(s)` and every one of them is `xdisp.o`'s, another lane's unit, the
  same fourteen as at batch 5.
- **Other gates at this point**: `check-globals` `284 units, 0 DEFVAR_* variables without a
  definition in their own unit`; `check-mirror-dupes` `1 duplicate declaration(s), 1 ratchet
  row(s), 0 failure(s)` (the module's items are definitions, so the tool never reads them as
  copies); `check-declarations` `13 declaration sites (0 forbidden, 13 allowed literals), 0
  failures` plus `read 630 files (623 .rs 7 .swift)`; `check-target-names` `627 .rs files, 591 map
  targets, 0 finding(s)` (the `DECLARED` first line is untouched, so the shim stays out of R1).
- **What the ruling's class 1 looks like now**: of its 65 names, **62 stand in this file**
  (module included); the only two that do not are `image_load_failed_p` and `image_pixmap`, whose
  `struct image` `include/dispextern.rs` 1360 records as not mirrored yet.

- **Batch 5 (the `frame_*` second half and the head of the read family, 15 exports)**:
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-abi cargo run -q -p xtask -- build-tu src/abi_shim.c`
  -> `built src/abi_shim.rs -> ../build/femacs/abi_shim.o`.  `nm -g build/femacs/abi_shim.o` answers
  the eleven ungated names - `T _frame_name`, `T _frame_set_terminal`, `T _frame_set_no_split`,
  `T _frame_set_undecorated`, `T _frame_set_can_set_window_size`, `T _frame_set_was_invisible`,
  `T _window_contents`, `T _window_current_matrix`, `T _buffer_filename`, `T _terminal_id`,
  `T _terminal_display_info_ns` (39 named exports in all, 34 of them `_frame_*`) - and answers **none**
  of the four arm-only ones (`_frame_alloc_ns_output`, `_frame_set_fontset`,
  `_frame_set_display_info`, `_frame_set_double_buffered`), which is the expected reading for the
  frozen configuration.  `tools/check-swift-imports.py` prints
  `STALE (satisfied) <name>: provided now, so the row goes` for exactly those eleven and for no other
  row of this shim's; its tail is `838 import(s) in 7 Swift file(s), 5629 core export(s), 616
  unprovided, 549 ratchet row(s), 89 failure(s)` - the 89 failures are the in-flight `nsfns.swift`
  batch (`window_set_*`, `frame_set_pixel_*`, `current_buffer_begv_byte`, …), another lane's imports,
  so the only rows this batch owns are the eleven `STALE (satisfied)` lines just named.
- **No host and no reference symbol was invented**: `nm -g build/emacs-ref/{src,lib}/*.o` has no
  symbol for any of the fifteen (checked name by name against the object symbol list; C's nearest
  text is the macro or member each `/// C:` line names), and `check-undef.py`'s reading for this
  object is unchanged - `nm -u build/femacs/abi_shim.o` is the same twelve entries as before the
  batch (`__RNv…` panic shims, `_setjmp`, `_cons_to_signed`, `_cons_to_unsigned`,
  `_wrong_type_argument`, `_xfree`); the tool's current `14 NEW undefined reference(s)` are all
  `xdisp.o`'s, another lane's unit.  The four gated bodies contribute no reference in the frozen
  configuration (they are compiled out), so `frame_alloc_ns_output`'s `xzalloc` call is invisible to
  this reading - and **no gate compiles those bodies**: `xtask dead-arms`' `FLIPS` list holds six
  configurations (`no_rint`, `no_trunc`, `no_signbit`, `have_haiku`, `have_native_comp`,
  `HAVE_LOG2`), none of them `HAVE_NS`/`NS_IMPL_COCOA`, so the arm's text is recorded rather than
  compiled - the same status the `ns_impl_cocoa` field of `struct ns_output` already has in this tree
  (P-51's "carrying the text is not the same as compiling it", recorded here rather than claimed).
- **Other gates at this point**: `check-globals` `284 units, 0 DEFVAR_* variables without a
  definition in their own unit`; `check-mirror-dupes` `1 duplicate declaration(s), 1 ratchet row(s),
  0 failure(s)`; `check-declarations` `13 declaration sites (0 forbidden, 13 allowed literals), 0
  failures` plus `read 630 files (623 .rs 7 .swift)`; `check-target-names` `627 .rs files, 591 map
  targets, 0 finding(s)` (the `DECLARED` first line is untouched, so the shim stays out of R1).
- **Batches 3 and 4 (the `frames.bits1` accessors and the three names status 3.242 fixed)**:
  `build-tu src/abi_shim.c` -> `built src/abi_shim.rs -> ../build/femacs/abi_shim.o`;
  `nm -g build/femacs/abi_shim.o` answers `T _frame_explicit_name`, `T _frame_set_explicit_name`,
  `T _frame_set_wants_modeline`, `T _frame_set_output_method`, `T _frame_tab_bar_redisplayed`,
  `T _frame_tab_bar_resized`, `T _frame_set_tab_bar_resized`, `T _frame_tab_bar_window`,
  `T _frame_set_external_menu_bar`, `T _frame_set_external_tool_bar` (28 `_frame_*` in all);
  `check-swift-imports.py` then prints `STALE (satisfied) <name>: provided now, so the row goes` for
  each of them (the ten new rows; the eight from batch 2 are still in the file, so that run lists
  eighteen of this shim's rows - the other two `STALE (satisfied)` lines, `ns_free_frame_resources`
  and `ns_set_offset`, are the nsterm lane's).
- **No reference symbol was invented** (status 3.251's rule): `nm -g build/emacs-ref/src/*.o`
  contains none of the shim's names (they are the Swift lanes' C-ABI spellings of C macros and
  members, which is what the ruling says), and `check-undef.py` reports `0` new undefined
  references from `abi_shim.o` (its current failure list is nineteen references out of `xdisp.o`,
  another lane's unit).
- **Batch 2 (the eight `frame_set_*` writers)**: `build-tu src/abi_shim.c` -> `built
  src/abi_shim.rs -> ../build/femacs/abi_shim.o`; `nm -g build/femacs/abi_shim.o` answers
  `T _frame_set_background_pixel`, `T _frame_set_border_width`,
  `T _frame_set_child_frame_border_width`, `T _frame_set_foreground_pixel`,
  `T _frame_set_internal_border_width`, `T _frame_set_menu_bar_lines`,
  `T _frame_set_tab_bar_height`, `T _frame_set_tab_bar_lines` (18 `_frame_*` in all), each against
  its `@_silgen_name` line in the table above; `check-swift-imports.py` then prints
  `STALE (satisfied) frame_set_background_pixel: provided now, so the row goes` and the same line
  for the other seven (its tail: `761 import(s) in 7 Swift file(s), 5599 core export(s), 573
  unprovided, 557 ratchet row(s), 36 failure(s)` - the Swift lanes are adding imports in the same
  window, so the eight `STALE (satisfied)` lines are this batch's whole signal).

- **`build-tu`**: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-abi cargo run -q -p xtask -- build-tu
  src/abi_shim.c` -> `built src/abi_shim.rs -> ../build/femacs/abi_shim.o`.
- **`nm -g` (the ten symbols, `nm` spelling against the Swift names)**: `T _frame_left_pos`,
  `T _frame_top_pos`, `T _frame_title`, `T _frame_icon_name`, `T _frame_param_alist`,
  `T _frame_selected_window`, `T _frame_menu_bar_vector`, `T _frame_tool_bar_items`,
  `T _frame_menu_bar_items_used`, `T _frame_n_tool_bar_items` - each with its `@_silgen_name` line
  in the table above.
- **`check-swift-imports.py`**: these ten names left the `unprovided` set, so the tool now lists
  `STALE frame_left_pos` … `STALE frame_n_tool_bar_items` (ten of the eleven `STALE` lines; the
  eleventh, `visible_bell`, is another lane's).  The round's tail line was
  `710 import(s) in 7 Swift file(s), 5566 core export(s), 554 unprovided, 535 ratchet row(s), 41
  failure(s)` - the Swift lanes are adding imports at the same time, so the only rows this batch
  owns are the ten `STALE` lines just named.
- **`check-target-names.py`**: `627 .rs files, 591 map targets, 0 finding(s)` - the `DECLARED`
  first line is what keeps `abi_shim.rs` out of rule R1.
- **`gen-port-inventory.py` / `gen-port-map.py`**: re-run with the file in the tree ->
  `wrote …: 590 files, done=583 wip=7 todo=0` and the map's six mode tables unchanged; neither
  grows a row for the shim (they iterate the reference tree's C sources).
- **`check-symbols.py` / `check-undef.py`**: `86 units compared, 1 mismatched` (the mismatch is
  `lib/regex`, another lane) - the shim is not compared, having no reference object; `696 unit
  object(s) …, 0 new` for the undefined-symbol gate.

- **Ruling (3.255, integrator): `frame_set_double_buffered` is exported under the same gate the
  member lives under.**  `#[cfg (feature = "ns_impl_cocoa")]` on the export, and the ratchet row
  stays with the reason "only a Cocoa build can clear this row" - the `macfont`/`nsfont` permanent
  dead-arm shape.  Exporting it ungated would invent a host for a member the frozen record does not
  have (P-08); deferring it to the NS round would push an already-measured row into the future.
  The general ruling for macro-name entries in this file is in `docs/status.md` 3.255 (B first:
  expand in Swift when the body is expressible there; A only when the expansion needs core-internal
  facts, which is when this shim exports the same-name C-ABI entry).

- **Integrator note (3.262): the four guarded exports are recorded text whose compilation is
  measured, not assumed.**  Adding the `have_ns` flip to `xtask dead-arms` compiles this file's
  `#[cfg (feature = "have_ns")]` bodies for the first time: they fail with the platform types the
  NS region depends on (`CGFloat`, `NSRect`, `NSString`, `NSArray`, `EmacsMenu`, `Window`), which
  `include/nsterm.rs` 50-52 deliberately leaves to "the platform target".  The one defect the flip
  caught that was *not* platform-shaped - eleven bare `c_float` spellings in that mirror - is fixed
  (`core::ffi::c_float`, status 3.91 ②).  The flip is reverted until an AppKit/GNUstep mirror
  exists; re-adding it is two entries plus the FLIPS array length, and this file's guarded bodies
  are then compiled rather than merely carried.

## 2026-10-06: two fringe exports moved back to their unit (status 3.669)

`src/abi_shim.rs` no longer defines the two names this ledger's rows 231 and 272 used to
point at, and it carries a note where the first of them was.  Both were stand-ins recorded as
"the unit does not exist yet" (`src/fringe.rs` was the empty P-86 unit at status 3.31):

* `gui_define_fringe_bitmap` - the unit now carries C 1852-1863 (slice 2, `src/fringe.rs`
  2482), body and all;
* `draw_fringe_bitmap` - the unit has carried C 838-905 since slice 1 (`src/fringe.rs`
  1276).

Both are `#[unsafe(no_mangle)] pub unsafe extern "C" fn` under `have_window_system`, so the two
Swift imports (`nsterm.swift` 3794, 4068-4069) are served by the unit and the shim's copies were
duplicate definitions in the configuration that compiles both (P-88 (5)'s move-back; the ledger
`docs/ports/fringe.md` Owed 4 is the other half of this record).  Readings that round: `build-tu
src/abi_shim.rs` -> `built`; `check-link.py --executable` -> `linked build/femacs/emacs - the
program has no undefined reference`; `check-swift-imports.py` `136 failure(s)` before this round's
builds and `9 failure(s)` after them - the gate's providers are the **built objects** (`nm -g
build/femacs/**/*.o`, its line 15), so that swing is the object set (this round built `abi_shim` and
`fringe`), not the move-back; the deleted stand-ins were gated `have_window_system` and never were
providers here.  The gate stays red for its own reasons (159 ratcheted names await a ruling).

**The paragraphs above that describe the stand-ins' bodies (this file's "gated debt whose body
stops at C's second guard" and "no statement of C's body is carried") describe code that is no
longer in the tree.**  They are kept because they are the record of why the stand-ins existed and
what the unit's own text had to supply when it landed.
