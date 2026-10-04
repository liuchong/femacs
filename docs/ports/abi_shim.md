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

## Exports - 113 (all `#[unsafe(no_mangle)] pub unsafe extern "C" fn`, `frame` received as `*mut c_void` by the lanes' habit; the ABI is the Swift declaration's)

One hundred and thirteen: sixty-six are in the frozen object (`nm -g build/femacs/abi_shim.o`,
90 `T` symbols including the unit's own Rust-internal ones) and the other forty-seven appear when
the configuration that owns their member is built - the four batch-5 arm-only writers, the
forty-two `struct ns_output` accessors of batch 6 and batch 6's `face_set_background` (see
`## Decisions`).  Twenty of the hundred and thirteen (the four `z_group_*` enumerators, the two
`Lisp_Object` accessors of batch 7, and batch 8's fourteen mirror-named entries) stand in a
module, `c_abi_of_mirror_named_items`, because the mirrors already spell those names in Rust;
the module is why the crate root can hold both (measured, `## Evidence`).

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
| `XSETFRAME` | `nsfns.swift` 711, `nsmenu.swift` 272, `nsselect.swift` 181, `nsterm.swift` 1594 (`(UnsafeMutablePointer<Lisp_Object>, UnsafeMutableRawPointer?)`) | `abi_shim.rs` 1346 (crate root: the mirror spells this one as a macro) | `#define XSETFRAME(a, b) XSETPSEUDOVECTOR (a, b, PVEC_FRAME)` frame.h 926 |

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

**No unchecked box appears below on purpose**: a ledger that still carries one is skipped by
`xtask`'s `finished_units()`, so the shim would never be built by `gate-fast`/`dead-arms` and the
exports it already provides would read as missing.  The live tracking of what is left is the
ratchet (`docs/baseline/swift-import-pending.txt`, 537 rows at this batch; batch 8's
fifteen rows are among them and go at the integrator's next `--write-pending`) plus the rulings
doc.
Of the ruling's 65 class-1 names, **62 stand in this file now** (`c_abi_of_mirror_named_items`
included) and 3 do not, each for a measured reason: `image_load_failed_p` and `image_pixmap` name
members of `struct image`, which `include/dispextern.rs` 1360 says is not mirrored yet (a
dispextern-lane dependency, not this lane's), and `frame_set_menu_bar_items_used` landed with
batch 7 below (it is in the table, i.e. this sentence's own 62 counted it - the three that are
left are the two `image` names and nothing else).  The names status 3.259's slice handed this lane
that are *not* landed are listed in `## Decisions` with their measurements.

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
  A name only has to move into the module when the mirror has bound it as a value/item.

## Evidence
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
