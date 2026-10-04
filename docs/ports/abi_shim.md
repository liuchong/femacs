# femacs/src/abi_shim.rs (the C-ABI entry points the macOS Swift units import; no reference file)

This file is **not** the port of a reference file: C has no functions with these names.  It is the
core side's table of honouring the names the macOS Swift units import with `@_silgen_name`
(`tools/check-swift-imports.py`; the rulings are
`docs/reviews/2026-10-04-swift-import-rulings.md`).  Each entry is the C-ABI spelling of a C
construct the Swift lane reads or writes - a `struct frame` member, a `FRAME_*` accessor macro, or
a bitfield of `frames.bits1` - so the core has to provide it as a real symbol before the Swift
units can link.

The first line is the `DECLARED` pattern `check-target-names.py` accepts for a product that is not
a port (`^// femacs: generated mirror of the frozen build's`), which is why the file is under
`femacs/src/` yet appears in no row of `docs/port-inventory.md` or `docs/port-map.md`; both were
re-run to confirm it (see `## Evidence`).

## Exports - 97 (all `#[unsafe(no_mangle)] pub unsafe extern "C" fn`, `frame` received as `*mut c_void` by the lanes' habit; the ABI is the Swift declaration's)

Ninety-seven: fifty are in the frozen object (`nm -g build/femacs/abi_shim.o`, 74 `T` symbols
including the unit's own Rust-internal ones) and the other forty-seven appear when the
configuration that owns their member is built - the four batch-5 arm-only writers, the
forty-two `struct ns_output` accessors of batch 6 and batch 6's `face_set_background` (see
`## Decisions`).  Six of the ninety-seven (the `z_group_*` enumerators and the two `Lisp_Object`
accessors) stand in a module, `c_abi_of_mirror_named_items`, because the mirrors already spell
those names in Rust; the module is why the crate root can hold both (measured, `## Evidence`).

| name | Swift import | providing Rust item | C text (line) |
| --- | --- | --- | --- |
| `frame_left_pos` | `nsmenu.swift` 324, `nsfns.swift` 644, `nsterm.swift` 1598 (`(UnsafeMutableRawPointer?) -> Int32`) | `femacs/src/abi_shim.rs` 56 | `int left_pos, top_pos;` frame.h 604 |
| `frame_top_pos` | `nsmenu.swift` 325, `nsfns.swift` 645, `nsterm.swift` 1599 (`-> Int32`) | `abi_shim.rs` 63 | frame.h 604 |
| `frame_title` | `nsfns.swift` 253 (`-> Lisp_Object`) | `abi_shim.rs` 70 | `Lisp_Object title;` frame.h 162 |
| `frame_icon_name` | `nsfns.swift` 254 (`-> Lisp_Object`) | `abi_shim.rs` 77 | `Lisp_Object icon_name;` frame.h 158 |
| `frame_param_alist` | `nsfns.swift` 255 (`-> Lisp_Object`) | `abi_shim.rs` 84 | `Lisp_Object param_alist;` frame.h 209 |
| `frame_selected_window` | `nsfns.swift` 498-499 (`-> Lisp_Object`) | `abi_shim.rs` 91 | `#define FRAME_SELECTED_WINDOW(f) f->selected_window` frame.h 1204 |
| `frame_menu_bar_vector` | `nsmenu.swift` 318-319 (`-> Lisp_Object`) | `abi_shim.rs` 98 | `Lisp_Object menu_bar_vector;` frame.h 234 |
| `frame_tool_bar_items` | `nsmenu.swift` 322-323 (`-> Lisp_Object`) | `abi_shim.rs` 105 | `Lisp_Object tool_bar_items;` frame.h 283 |
| `frame_menu_bar_items_used` | `nsmenu.swift` 314-315 (`-> Int32`) | `abi_shim.rs` 112 | `int menu_bar_items_used;` frame.h 307 |
| `frame_n_tool_bar_items` | `nsmenu.swift` 320-321 (`-> Int32`) | `abi_shim.rs` 119 | `int n_tool_bar_items;` frame.h 573 |
| `frame_set_border_width` | `nsfns.swift` 273-274 (`(UnsafeMutableRawPointer?, Int32)`) | `abi_shim.rs` 127 | `int border_width;` frame.h 613 |
| `frame_set_child_frame_border_width` | `nsfns.swift` 262-263 (`(…, Int32)`) | `abi_shim.rs` 134 | `int child_frame_border_width;` frame.h 617 |
| `frame_set_internal_border_width` | `nsfns.swift` 264-265 (`(…, Int32)`) | `abi_shim.rs` 141 | `int internal_border_width;` frame.h 622 |
| `frame_set_menu_bar_lines` | `nsfns.swift` 486-487 (`(…, Int32)`) | `abi_shim.rs` 148 | `int menu_bar_lines;` frame.h 635 |
| `frame_set_tab_bar_lines` | `nsfns.swift` 492-493 (`(…, Int32)`) | `abi_shim.rs` 155 | `int tab_bar_lines;` frame.h 558 |
| `frame_set_tab_bar_height` | `nsfns.swift` 490-491 (`(…, Int32)`) | `abi_shim.rs` 162 | `int tab_bar_height;` frame.h 561 |
| `frame_set_background_pixel` | `nsfns.swift` 269-270 (`(…, UInt)`) | `abi_shim.rs` 169 | `unsigned long background_pixel;` frame.h 732 |
| `frame_set_foreground_pixel` | `nsfns.swift` 267-268 (`(…, UInt)`) | `abi_shim.rs` 176 | `unsigned long foreground_pixel;` frame.h 733 |
| `frame_explicit_name` | `nsfns.swift` 249 (`-> Bool`) | `abi_shim.rs` 194 | `bool_bf explicit_name : 1` frame.h 417 (`FRAME_EXPLICIT_NAME_SHIFT` 15) |
| `frame_set_explicit_name` | `nsfns.swift` 250-251 (`(…, Bool)`) | `abi_shim.rs` 201 | frame.h 417 |
| `frame_set_wants_modeline` | `nsfns.swift` 271-272 (`(…, **Bool**)`) | `abi_shim.rs` 211 | `bool_bf wants_modeline : 1` frame.h 402 (`FRAME_WANTS_MODELINE_SHIFT` 11) |
| `frame_set_output_method` | `nsfns.swift` 266 (`(…, Int32)`) | `abi_shim.rs` 222 | `ENUM_BF (output_method) output_method : 4;` frame.h 446 (shift 21, `FRAME_OUTPUT_METHOD_MASK` 0x0F) |
| `frame_tab_bar_redisplayed` | `nsfns.swift` 260-261 (`-> Bool`) | `abi_shim.rs` 233 | `bool_bf tab_bar_redisplayed : 1` frame.h 503 (shift 30) |
| `frame_tab_bar_resized` | `nsfns.swift` 257 (`-> Bool`) | `abi_shim.rs` 240 | `bool_bf tab_bar_resized : 1` frame.h 504 (shift 31) |
| `frame_set_tab_bar_resized` | `nsfns.swift` 258-259 (`(…, Bool)`) | `abi_shim.rs` 247 | frame.h 504 |
| `frame_tab_bar_window` | `nsfns.swift` 256 (`-> Lisp_Object`) | `abi_shim.rs` 263 | `Lisp_Object tab_bar_window;` frame.h 254 (**inside `#ifdef HAVE_WINDOW_SYSTEM`**) - the frozen answer is `Qnil` |
| `frame_set_external_menu_bar` | `nsfns.swift` 487-488 (`(…, Int32)`) | `abi_shim.rs` 275 | `#define FRAME_EXTERNAL_MENU_BAR(f) false` frame.h 1082 (the `#else` arm) - no-op |
| `frame_set_external_tool_bar` | `nsfns.swift` 493-494 (`(…, Int32)`) | `abi_shim.rs` 283 | `#define FRAME_EXTERNAL_TOOL_BAR(f) false` frame.h 1097 (the `#else` arm) - no-op |
| `frame_name` | `nsfns.swift` 252 (`-> Lisp_Object`) | `abi_shim.rs` 299 | `Lisp_Object name;` frame.h 154 |
| `frame_set_terminal` | `nsfns.swift` 631-632 (`(…, UnsafeMutableRawPointer?)`) | `abi_shim.rs` 307 | `struct terminal *terminal;` frame.h 648 (C's writes: nsfns.m 1267, 2988) |
| `frame_set_no_split` | `nsfns.swift` 635 (`(…, Bool)`) | `abi_shim.rs` 315 | `bool_bf no_split : 1` frame.h 411 (`FRAME_NO_SPLIT_SHIFT` 14; C's writes: nsfns.m 1474, 3139) |
| `frame_set_undecorated` | `nsfns.swift` 573-574 (`(…, Bool)`) | `abi_shim.rs` 326 | `bool_bf undecorated : 1` frame.h 449 (shift 25; C's write: nsfns.m 1404 `FRAME_UNDECORATED (f) = …`) |
| `frame_set_can_set_window_size` | `nsfns.swift` 637-638 (`(…, Bool)`) | `abi_shim.rs` 337 | `bool_bf can_set_window_size : 1` frame.h 491 (shift 28; C's writes: nsfns.m 1545, 3149) |
| `frame_set_was_invisible` | `nsfns.swift` 636 (`(…, Bool)`) | `abi_shim.rs` 350 | `bool_bf was_invisible : 1` frame.h 530 (second flag word, shift 5; C's write: nsfns.m 1569) |
| `window_contents` | `nsfns.swift` 499, `nsmenu.swift` 413-414 (`-> Lisp_Object`) | `abi_shim.rs` 367 | `Lisp_Object contents;` window.h 143 |
| `window_current_matrix` | `nsfns.swift` 500-501 (`-> UnsafeMutableRawPointer?`) | `abi_shim.rs` 374 | `struct glyph_matrix *current_matrix;` window.h 224 |
| `buffer_filename` | `nsfns.swift` 504-505 (`-> Lisp_Object`) | `abi_shim.rs` 382 | `#define BVAR(buf, field) ((buf)->field ## _)` buffer.h 308 (the member is `Lisp_Object filename_;`, buffer.h 330) |
| `terminal_id` | `nsfns.swift` 373 (`-> Int32`) | `abi_shim.rs` 389 | `int id;` termhooks.h 516 |
| `terminal_display_info_ns` | `nsfns.swift` 374-375 (`-> UnsafeMutablePointer<ns_display_info>?`) | `abi_shim.rs` 398 | `struct ns_display_info *ns;` termhooks.h 538 (inside `union display_info`, 533) |
| `frame_alloc_ns_output` | `nsfns.swift` 633 (`(…)`) | `abi_shim.rs` 422, **`#[cfg (feature = "have_ns")]`** | `f->output_data.ns = xzalloc (sizeof *f->output_data.ns);` nsfns.m 1270, 2991 |
| `frame_set_fontset` | `nsfns.swift` 634 (`(…, Int32)`) | `abi_shim.rs` 433, **`have_ns`** | `#define FRAME_FONTSET(f) ((f)->output_data.ns->fontset)` nsterm.h 1096 (C's writes: nsfns.m 1272, 2994) |
| `frame_set_display_info` | `nsfns.swift` 716-717 (`(…, UnsafeMutablePointer<ns_display_info>?)`) | `abi_shim.rs` 445, **`have_ns`** | `#define FRAME_DISPLAY_INFO(f) ((f)->output_data.ns->display_info)` nsterm.h 1032 (C's writes: nsfns.m 1280, 2995) |
| `frame_set_double_buffered` | `nsfns.swift` 495-496 (`(…, Bool)`) | `abi_shim.rs` 465, **`have_ns` + `ns_impl_cocoa`** | `#define FRAME_DOUBLE_BUFFERED(f) ((f)->output_data.ns->double_buffered)` nsterm.h 1052 (inside `#if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MIN_REQUIRED >= 101400`, 1051-1053) |
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
| `frame_alpha` | `nsterm.swift` 2235-2236 (`(…, Int32) -> Double`) | `abi_shim.rs` 1141 | `double alpha[2];` frame.h 707-711; read nsterm.m 2378/2380 |
| `z_group_none` | `nsterm.swift` 2169 (`() -> Int32`) | `abi_shim.rs` 1171 **`mirror-named`** | `z_group_none,` frame.h 63 (`enum z_group` 61-67) = 0 |
| `z_group_above` | `nsterm.swift` 2170 | `abi_shim.rs` 1175 **`mirror-named`** | `z_group_above,` frame.h 64 = 1 |
| `z_group_below` | `nsterm.swift` 2171 | `abi_shim.rs` 1179 **`mirror-named`** | `z_group_below,` frame.h 65 = 2 |
| `z_group_above_suspended` | `nsterm.swift` 2172 | `abi_shim.rs` 1183 **`mirror-named`** | `z_group_above_suspended,` frame.h 66 = 3 |
| `FLOATP` | `nsterm.swift` 2228 (`(Lisp_Object) -> Bool`) | `abi_shim.rs` 1191 **`mirror-named`** | `#define FLOATP(x) lisp_h_FLOATP (x)` lisp.h 448 |
| `XFLOAT_DATA` | `nsterm.swift` 2229 (`(Lisp_Object) -> Double`) | `abi_shim.rs` 1197 **`mirror-named`** | `INLINE double XFLOAT_DATA (Lisp_Object f)` lisp.h 3186-3189 |

**`ns_output_*` / `ns_output_set_*` (batch 6) and the batch-7 additions**: the `have_ns` mark is the gate of `struct ns_output` (nsterm.h 935-1022, mirrored gated in `include/nsterm.rs` 419-489); `dispextern_face_records` is the gate `include/dispextern.rs` 1941 puts on `struct face`; `mirror-named` means the name is also a Rust item in a mirror, so the C-ABI spelling stands in `c_abi_of_mirror_named_items` (see `## Decisions`).

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

**The remaining names of the ruling's class 1 (37) are not listed as unchecked boxes here on
purpose**: a ledger that still carries an unchecked box is skipped by `xtask`'s
`finished_units()`, so the shim would never be built by `gate-fast`/`dead-arms` and the exports it
already provides would read as missing.  The live tracking of what is left is the ratchet
(`docs/baseline/swift-import-pending.txt`) plus the rulings doc; batches land in the order that
file gives.

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
  (`rg` over `femacs/platform/macos/*.swift` finds only the two *readers*, `frame_left_pos` /
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

## Evidence
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
