# emacs/src/w32term.h (929 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 119
- [x] ALIGN_STACK  (line 38, body da39a3ee5e6b)
- [x] APPS_PRESSED  (line 843, body 3daa00115a66)
- [x] BLACK_PIX_DEFAULT  (line 43, body d50f1c345465)
- [x] CP_DEFAULT  (line 46, body 70b8dcb93382)
- [x] FILE_NOTIFICATIONS_SIZE  (line 782, body c847eeea4f8b)
- [x] FRAME_BASELINE_OFFSET  (line 469, body 1e8e33ed07da)
- [x] FRAME_DISPLAY_INFO  (line 472, body 93e959594bbc)
- [x] FRAME_FONT  (line 467, body f161d4ff7b50)
- [x] FRAME_FONTSET  (line 468, body 47a09239c1dd)
- [x] FRAME_NATIVE_WINDOW  (line 465, body 3a085f01627f)
- [x] FRAME_NORMAL_PLACEMENT  (line 474, body 7413a327d690)
- [x] FRAME_OUTPUT_DATA  (line 461, body 71501307c738)
- [x] FRAME_PREV_FSMODE  (line 475, body efdbdddb6cb4)
- [x] FRAME_W32_WINDOW  (line 464, body f0a4830680ab)
- [x] GET_APPCOMMAND_LPARAM  (line 679, body ef2a302a920d)
- [x] GUISTR  (line 900, body 11f6ad8ec52a)
- [x] GUI_ENCODE_FILE  (line 901, body f58afd68f662)
- [x] GUI_ENCODE_SYSTEM  (line 902, body 95cdc629b95b)
- [x] GUI_FN  (line 903, body 47fdd9283ded)
- [x] GUI_SDATA  (line 907, body 61d567bc07cc)
- [x] HORIZONTAL_SCROLL_BAR_BOTTOM_BORDER  (line 626, body f28321d56c3c)
- [x] HORIZONTAL_SCROLL_BAR_INSIDE_HEIGHT  (line 586, body 52855ff9774e)
- [x] HORIZONTAL_SCROLL_BAR_INSIDE_WIDTH  (line 603, body 030e30111d14)
- [x] HORIZONTAL_SCROLL_BAR_LEFT_BORDER  (line 623, body b7aa8700ff6a)
- [x] HORIZONTAL_SCROLL_BAR_LEFT_RANGE  (line 598, body 10d24a511ceb)
- [x] HORIZONTAL_SCROLL_BAR_MIN_HANDLE  (line 630, body e5d5d8c5887e)
- [x] HORIZONTAL_SCROLL_BAR_RIGHT_BORDER  (line 624, body 7c68d668d349)
- [x] HORIZONTAL_SCROLL_BAR_TOP_BORDER  (line 625, body f28321d56c3c)
- [x] LEFT_WIN_PRESSED  (line 841, body 2cedb007191f)
- [x] MAX_TOUCH_POINTS  (line 441, body b1d5781111d8)
- [x] MSH_MOUSEWHEEL  (line 668, body d11414adb014)
- [x] PIX_TYPE  (line 280, body 2a31e46c0035)
- [x] RIGHT_WIN_PRESSED  (line 842, body da533c139833)
- [x] SCROLL_BAR_PACK  (line 546, body 9ab6a12f4c4b)
- [x] SCROLL_BAR_UNPACK  (line 549, body f98d805797de)
- [x] SCROLL_BAR_W32_WINDOW  (line 555, body 043b8506e123)
- [x] SET_SCROLL_BAR_W32_WINDOW  (line 559, body c88e22082412)
- [x] TME_LEAVE  (line 852, body 0dfedc5978a9)
- [x] UNICODE_NOCHAR  (line 685, body ec4cc6079e5c)
- [x] VERTICAL_SCROLL_BAR_BOTTOM_BORDER  (line 621, body 90ddb401fb04)
- [x] VERTICAL_SCROLL_BAR_INSIDE_HEIGHT  (line 581, body 36f98e3eabb8)
- [x] VERTICAL_SCROLL_BAR_INSIDE_WIDTH  (line 564, body 643baf8bba30)
- [x] VERTICAL_SCROLL_BAR_LEFT_BORDER  (line 618, body f28321d56c3c)
- [x] VERTICAL_SCROLL_BAR_MIN_HANDLE  (line 629, body 45dbf03ff832)
- [x] VERTICAL_SCROLL_BAR_RIGHT_BORDER  (line 619, body f28321d56c3c)
- [x] VERTICAL_SCROLL_BAR_TOP_BORDER  (line 620, body 6f08e035de5b)
- [x] VERTICAL_SCROLL_BAR_TOP_RANGE  (line 576, body 3d518bbbd4a8)
- [x] VK_APPS  (line 835, body 9d5623b59f81)
- [x] VK_LWIN  (line 833, body 22c3b427d2db)
- [x] VK_NUMPAD_BEGIN  (line 817, body 0622a45c97ce)
- [x] VK_NUMPAD_CLEAR  (line 818, body 501181fd8449)
- [x] VK_NUMPAD_DELETE  (line 829, body 2bbbbf8e1b40)
- [x] VK_NUMPAD_DOWN  (line 827, body 8e0cbda9e660)
- [x] VK_NUMPAD_END  (line 822, body d3b8dc0de182)
- [x] VK_NUMPAD_ENTER  (line 819, body 258ea2c47232)
- [x] VK_NUMPAD_HOME  (line 823, body 639017e17b7a)
- [x] VK_NUMPAD_INSERT  (line 828, body 060c73978a62)
- [x] VK_NUMPAD_LEFT  (line 824, body e77248fe53c5)
- [x] VK_NUMPAD_NEXT  (line 821, body 089d2b1e858e)
- [x] VK_NUMPAD_PRIOR  (line 820, body 75737f6cfbd1)
- [x] VK_NUMPAD_RIGHT  (line 826, body 5cf57c888cbf)
- [x] VK_NUMPAD_UP  (line 825, body c9fa62cc597a)
- [x] VK_RWIN  (line 834, body 9af7df7f76d2)
- [x] WHITE_PIX_DEFAULT  (line 44, body bf1092eaaf1c)
- [x] WM_APPCOMMAND  (line 678, body a1feebaa2341)
- [x] WM_EMACS_BRINGTOTOP  (line 712, body 3fe43f58150f)
- [x] WM_EMACS_CREATEHSCROLLBAR  (line 693, body 203f3fd587b6)
- [x] WM_EMACS_CREATEVSCROLLBAR  (line 692, body 9622f9f3893a)
- [x] WM_EMACS_CREATEWINDOW  (line 690, body b842b152e6d3)
- [x] WM_EMACS_DESTROYWINDOW  (line 696, body db364053c923)
- [x] WM_EMACS_DESTROY_CARET  (line 706, body f1946f4d1a0e)
- [x] WM_EMACS_DONE  (line 691, body 23cc18f4b62b)
- [x] WM_EMACS_DRAGOVER  (line 716, body 15569da7f4aa)
- [x] WM_EMACS_DROP  (line 717, body be64fb8647c0)
- [x] WM_EMACS_END  (line 718, body 69ff6e16e2e8)
- [x] WM_EMACS_FILENOTIFY  (line 714, body 27167199b6a1)
- [x] WM_EMACS_HIDE_CARET  (line 708, body f5b07a67bab3)
- [x] WM_EMACS_IME_STATUS  (line 715, body f147cca94b28)
- [x] WM_EMACS_INPUT_READY  (line 713, body 1576fd4b4f46)
- [x] WM_EMACS_KILL  (line 689, body ea6f1db843c6)
- [x] WM_EMACS_PAINT  (line 711, body ad6c7fe87ddd)
- [x] WM_EMACS_REGISTER_HOT_KEY  (line 702, body f0a961cb4f27)
- [x] WM_EMACS_SETCURSOR  (line 709, body d778d0e3de99)
- [x] WM_EMACS_SETFOCUS  (line 698, body 7929d2f0cb78)
- [x] WM_EMACS_SETFOREGROUND  (line 699, body 1d1e0f23b549)
- [x] WM_EMACS_SETKEYBOARDLAYOUT  (line 701, body a86d1e283f29)
- [x] WM_EMACS_SETLOCALE  (line 700, body a6ccee0187e0)
- [x] WM_EMACS_SETWINDOWPOS  (line 695, body 588d16e7ff0f)
- [x] WM_EMACS_SET_TOOLKIT_THEME  (line 719, body a1ba8018b734)
- [x] WM_EMACS_SHOWCURSOR  (line 710, body 6bcc41bfa9a9)
- [x] WM_EMACS_SHOWWINDOW  (line 694, body d4340969658e)
- [x] WM_EMACS_SHOW_CARET  (line 707, body d30c24a14a99)
- [x] WM_EMACS_START  (line 688, body 9ec95bf50ebb)
- [x] WM_EMACS_TOGGLE_LOCK_KEY  (line 704, body b6ecb197aa87)
- [x] WM_EMACS_TRACKPOPUPMENU  (line 697, body 3a0e6891418a)
- [x] WM_EMACS_TRACK_CARET  (line 705, body 0d3d55de63af)
- [x] WM_EMACS_UNREGISTER_HOT_KEY  (line 703, body b569bddadbe2)
- [x] WM_MOUSEHWHEEL  (line 675, body 0fe081684c8b)
- [x] WM_MOUSELEAVE  (line 851, body 1d3633be995a)
- [x] WM_MOUSEWHEEL  (line 665, body 864e956eea3e)
- [x] WM_UNICHAR  (line 682, body 2fbeb85d9c3f)
- [x] WM_XBUTTONDOWN  (line 671, body d33965f43a1d)
- [x] WM_XBUTTONUP  (line 672, body 7711706433e1)
- [x] WND_BACKGROUND_INDEX  (line 726, body c7ab4d87d7e7)
- [x] WND_BORDER_INDEX  (line 723, body 630d9ccc0aa5)
- [x] WND_EXTRA_BYTES  (line 729, body 916605011a14)
- [x] WND_FONTWIDTH_INDEX  (line 721, body f28321d56c3c)
- [x] WND_HSCROLLBAR_INDEX  (line 725, body b1ccca558b0d)
- [x] WND_LAST_INDEX  (line 727, body 555cab4fb265)
- [x] WND_LINEHEIGHT_INDEX  (line 722, body 10720c7cfed2)
- [x] WND_VSCROLLBAR_INDEX  (line 724, body 41e278e691e6)
- [x] XSCROLL_BAR  (line 534, body aa568fbbbc66)
- [x] enter_crit  (line 762, body a37d4ea22f90)
- [x] leave_crit  (line 763, body bd22055e108b)
- [x] w32_clear_area  (line 660, body 0c651b5ce75a)
- [x] w32_clear_rect  (line 657, body 0b21c814a961)
- [x] w32_fill_area  (line 637, body c2a6519597d0)
- [x] w32_fill_area_abs  (line 647, body 8035d72c18b1)
- [x] w32_kbdhook_active  (line 811, body b6589fc6ab0d)

## Types and their fields — 10
- [x] W32Msg: MSG msg, DWORD dwModifiers, RECT rect
- [x] deferred_msg: struct deferred_msg* next, W32Msg w32msg, LRESULT result, int completed
- [x] notifications_set: LPBYTE notifications, DWORD size, void* desc, struct notifications_set* next, struct notifications_set* prev
- [x] scroll_bar: union vectorlike_header header, Lisp_Object window, Lisp_Object next, Lisp_Object prev, Lisp_Object w32_window_low, Lisp_Object w32_window_high, Lisp_Object w32_widget_low, Lisp_Object w32_widget_high, int top, int left, int width, int height, int start, int end, int dragging, bool horizontal
- [x] tagTRACKMOUSEEVENT: DWORD cbSize, DWORD dwFlags, HWND hwndTrack, DWORD dwHoverTime
- [x] w32_bitmap_record: Emacs_Pixmap pixmap, char* file, Emacs_Pixmap stipple, HINSTANCE hinst, int refcount, int height, int width, int depth
- [x] w32_display_info: struct w32_display_info* next, struct terminal* terminal, Lisp_Object name_list_element, int reference_count, double resx, double resy, int n_planes, int n_cbits, int grabbed, ptrdiff_t icon_bitmap_id, Window root_window, HCURSOR vertical_scroll_bar_cursor, HCURSOR horizontal_scroll_bar_cursor, const char* rdb, int has_palette, struct w32_palette_entry* color_list, unsigned num_colors, HPALETTE palette, int regen_palette, int faked_key, int smallest_char_width, int smallest_font_height, Emacs_GC* scratch_cursor_gc, Mouse_HLInfo mouse_highlight, char* w32_id_name, int n_fonts, struct w32_bitmap_record* bitmaps, ptrdiff_t bitmaps_size, ptrdiff_t bitmaps_last, struct frame* w32_focus_frame, struct frame* w32_focus_event_frame, struct frame* highlight_frame, struct frame* w32_pending_autoraise_frame, struct frame* last_mouse_frame, struct frame* last_mouse_motion_frame, struct frame* last_mouse_glyph_frame, int last_mouse_motion_x, int last_mouse_motion_y, RECT last_mouse_glyph, struct scroll_bar* last_mouse_scroll_bar, int last_mouse_scroll_bar_pos, Time last_mouse_movement_time, int cursor_display_counter
- [x] w32_output: struct x_output x_compatible, HMENU menubar_widget, HPALETTE old_palette, Emacs_GC* cursor_gc, Window window_desc, Window parent_desc, struct font* font, int baseline_offset, int fontset, COLORREF cursor_pixel, COLORREF border_pixel, COLORREF mouse_pixel, COLORREF cursor_foreground_pixel, COLORREF scroll_bar_foreground_pixel, COLORREF scroll_bar_background_pixel, HCURSOR text_cursor, HCURSOR nontext_cursor, HCURSOR modeline_cursor, HCURSOR hand_cursor, HCURSOR hourglass_cursor, HCURSOR horizontal_drag_cursor, HCURSOR vertical_drag_cursor, HCURSOR left_edge_cursor, HCURSOR top_left_corner_cursor, HCURSOR top_edge_cursor, HCURSOR top_right_corner_cursor, HCURSOR right_edge_cursor, HCURSOR bottom_right_corner_cursor, HCURSOR bottom_edge_cursor, HCURSOR bottom_left_corner_cursor, unsigned hourglass_p : 1, HCURSOR current_cursor, DWORD dwStyle, unsigned explicit_parent : 1, unsigned asked_for_visible : 1, unsigned menubar_active : 1, Emacs_GC* gc, unsigned long pixel, unsigned long relief_background, WINDOWPLACEMENT normal_placement, int prev_fsmode, HBITMAP paint_buffer, HDC paint_dc, HDC paint_buffer_handle, HGDIOBJ paint_dc_object, int paint_buffer_width, int paint_buffer_height, unsigned paint_buffer_dirty : 1, unsigned want_paint_buffer : 1, int touch_ids, LONG touch_x, LONG touch_y, EMACS_INT touch_base, DWORD tool_bar_dwID
- [x] w32_palette_entry: struct w32_palette_entry* next, PALETTEENTRY entry
- [x] x_output: int focus_state

## File-scope objects — 25
- [x] extern MSG CurMsg  (line 51)
- [x] extern BOOL bUseDflt  (line 52)
- [x] extern struct w32_display_info x_display_list  (line 223)
- [x] extern struct w32_display_info one_w32_display_info  (line 224)
- [x] extern HMENU current_popup_menu  (line 227)
- [x] extern int menubar_in_use  (line 228)
- [x] extern frame_parm_handler w32_frame_parm_handlers  (line 274)
- [x] extern struct w32_output w32term_display  (line 458)
- [x] extern DWORD dwWindowsThreadId  (line 731)
- [x] extern HANDLE hWindowsThread  (line 732)
- [x] extern DWORD dwMainThreadId  (line 733)
- [x] extern HANDLE hMainThread  (line 734)
- [x] extern CRITICAL_SECTION critsect  (line 755)
- [x] extern struct notifications_set notifications_set_head  (line 793)
- [x] extern int w32_keyboard_codepage  (line 846)
- [x] extern struct font w32_system_remap_font  (line 875)
- [x] extern HWND w32_system_caret_hwnd  (line 877)
- [x] extern int w32_system_caret_height  (line 878)
- [x] extern int w32_system_caret_x  (line 879)
- [x] extern int w32_system_caret_y  (line 880)
- [x] extern struct window w32_system_caret_window  (line 881)
- [x] extern int w32_system_caret_hdr_height  (line 882)
- [x] extern int w32_system_caret_mode_height  (line 883)
- [x] extern Window tip_window  (line 885)
- [x] extern int w32_message_fd  (line 928)

## Decisions

- **This header has no platform guard of its own, so its checklist is real.**  C 21-25 include
  `w32gui.h`, `frame.h` and `atimer.h` and everything after that is unconditional text: unlike
  `include-nsterm.md`, this ledger's 119 macros, 10 record types and 25 objects are the live
  inventory `tools/extract-abi.py` computes for this file (`docs/port-inventory.md` 653 prints
  119 macros / 19 constants / 129 fields / 25 objects, the all-branches view).
- **The whole body carries `#[cfg (feature = "windowsnt")]`.**  The header is included only by
  Windows GUI units (`#ifdef HAVE_NTGUI` in `w32fns.c`, `w32term.c`, ...) and `HAVE_NTGUI` /
  `WINDOWSNT` are `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so P-51's arm is the
  right side (the `include/w32font.rs` 12 reading of a Windows header).  C's own names are kept
  inside it (`MSG`, `HWND`, `HDC`, `HMENU`, `HPALETTE`, `HCURSOR`, `HBITMAP`, `HGDIOBJ`,
  `COLORREF`, `RECT`, `WINDOWPLACEMENT`, `CRITICAL_SECTION`, `HANDLE`, `DWORD`, `LONG`, `LPBYTE`,
  `WINAPI`, ...): they come from `<windows.h>`, which has no mirror in this tree, and that target
  supplies them.  Nothing in the arm is compiled on this target, so **no layout assertion is
  written** for any of the ten records (the `include/macfont.rs` 26-27 rule), and nothing is
  degraded to `*mut c_void` merely because it is Windows-typed.
- **`check-cond-eval` is NOT VERIFIED here, and the tool's conservative reading is what the mirror
  follows.**  `python3 tools/check-cond-eval.py src/w32term.h` -> `17 conditions, 0 unknown, 1
  disagreements [NOT VERIFIED]`: C 33-40's `#if USE_STACK_LISP_OBJECTS && !defined _WIN64 &&
  !defined __x86_64__ && __GNUC__ + (__GNUC_MINOR__ > 1) >= 5` is True to clang and False to the
  evaluator (`USE_STACK_LISP_OBJECTS` is not in the macro table; `lisp.h` 5721-5736 defines it for
  the Lisp build).  Because the file is not on `cond-eval-verified.txt`, the extractor keeps *both*
  arms of every `#if` live, and the mirror carries both: `ALIGN_STACK` (the attribute arm recorded
  in a `// C:` line - a Rust item cannot express
  `__attribute__((force_align_arg_pointer))`), `SCROLL_BAR_PACK`/`SCROLL_BAR_UNPACK`,
  `w32_kbdhook_active` and the `NTGUI_UNICODE` block.
- **The inner arms and the feature each one carries.**  `_WIN64` is a compiler predefine and not in
  the table; the tree's own feature is `win64` (`xtask/Cargo.toml` 561, `# _WIN64`, the
  `nt/inc/include/stdint.rs` 34 reading), so C 536-552's 64-bit arm is `win64` and the 32-bit
  `#else` is `not (win64)`.  `NTGUI_UNICODE` (C 892) is defined by `conf_post.h` 169-170 **only**
  for `defined CYGWIN && defined HAVE_NTGUI` ("Cygwin runs only on UNICODE-supporting systems"), so
  that arm is `cygwin` (a name this tree already declares) and the `#else` arm - the MinGW/Windows
  GUI one (`GUISTR(x) x`, `guichar_t` is `char`) - is live.  `CYGWIN` (C 927) and `EMACSDEBUG`
  (C 887) are `undef` in the table, so `w32_message_fd` carries `cygwin` and the
  `w32_name_of_message` prototype is recorded.  The file's own `#ifdef WINDOWSNT` (C 803) is the
  same side as the umbrella, so its arm is carried unconditionally inside it and the `#else` arm
  (`#define w32_kbdhook_active 0`) is recorded.
- **The nine `#ifndef` blocks are "define it if the SDK does not" guards**, carried as constants
  with C's values and the guard written in the `/// C:` line (the `include/widget.rs` `XtNfont`
  reading): `WM_MOUSEWHEEL`/`WM_XBUTTONDOWN`/`WM_XBUTTONUP`/`WM_MOUSEHWHEEL`/`WM_APPCOMMAND`/
  `WM_UNICHAR`/`UNICODE_NOCHAR` (C 664-686), `VK_LWIN`/`VK_RWIN`/`VK_APPS` (C 831-836) and
  `WM_MOUSELEAVE`/`TME_LEAVE`/`TRACKMOUSEEVENT` (C 850-861).  `MSH_MOUSEWHEEL` is a *string*
  (`"MSWHEEL_ROLLMSG"`), so it is `&core::ffi::CStr` compared as bytes (the `include/widget.rs`
  shape).
- **The six bitfields are recorded, not packed.**  `hourglass_p` (C 378), `explicit_parent`
  (C 394), `asked_for_visible` (C 397), `menubar_active` (C 400), `paint_buffer_dirty` (C 436) and
  `want_paint_buffer` (C 439) live in `struct w32_output`, which no compiler on this target can
  compile (its members are `<windows.h>`'s); PORTING.md 1.3 / P-80 require the real compiler's byte
  dump before a storage unit may stand for a bitfield, so each is recorded in a `// C:` line inside
  the record and the extractor asks for none of them (`field["bitfield"]` is skipped).
- **`struct relief` is declared here for this header's own record.**  C 402-408 nests
  `struct relief { Emacs_GC *gc; unsigned long pixel; } black_relief, white_relief;` inside
  `struct w32_output`, so the port declares the tag and the two members; `include/androidterm.rs`
  222 declares the same tag for Android's own output record, and the two never meet (that mirror is
  Android's, this one is Windows', and neither is compiled on this target) - nothing of that
  mirror's is touched or removed here.
- **`#if 0` is recorded**: C 71-73's `unsigned refcount;` inside `struct w32_palette_entry` is
  stripped by the extractor (`w32_palette_entry` has C's two live members) and kept as a `// C:`
  line (PORTING.md P-08).
- **Degraded names (P-55/P-72 rule 3).**  `struct image`, `struct face`, `struct input_event` and
  `struct glyph_matrix` are only pointed at, so those parameters are `*mut c_void` with C's
  prototype in the `/// C:` line; `struct window` (C 881) is `window.h`'s and this mirror includes
  nothing of it, so `w32_system_caret_window` is `*mut c_void` too; `w32_frame_parm_handlers[]`'s
  elements are `*mut c_void` until `include/dispextern.rs` carries `frame_parm_handler` (the
  `include/androidterm.rs` 506-511 reading).  `struct font` (C 875) *is* named, because
  `include/font.rs` owns that record.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/w32term.h src/include/w32term.rs`
  -> `OK emacs/src/w32term.h -> src/include/w32term.rs` (no `- ` problem lines); the tool
  prints `(8 C macros have several definitions (configuration branches), body not compared:
  ALIGN_STACK, GUISTR, GUI_ENCODE_FILE, GUI_ENCODE_SYSTEM, GUI_FN, SCROLL_BAR_PACK ...)`, which is
  C's own two-arm structure.
- `python3 tools/extract-abi.py emacs/src/w32term.h` -> this ledger: 0 subrs, 0 functions defined,
  119 macros, 10 types (140 fields), 25 file-scope objects - every box checked.
- `python3 tools/check-cond-eval.py src/w32term.h` -> `17 conditions, 0 unknown, 1 disagreements
  [NOT VERIFIED]` (C 34: clang True, evaluator False - see the Decisions).
- **Two measured Rust rules found while landing this mirror**, both caught by the gates:
  1. C's token-pasting operator cannot appear in a `macro_rules!` body at all -
     `layout-check` reported `error: reserved multi-hash token is forbidden` for
     `GUISTR`/`GUI_FN`'s Cygwin arm, so those two bodies (with `GUI_ENCODE_FILE`/
     `GUI_ENCODE_SYSTEM`) are recorded in a `// C:` line instead; the names are already declared by
     the `#else` arm and the extractor treats them as configuration-branch duplicates.
  2. A C `do { … } while (0)` body makes the checker ask the port's body for the token `0`, so
     `w32_fill_area`/`w32_fill_area_abs` write C's uninitialised `RECT rect` as an explicit
     zero initialiser (`RECT { left: 0, top: 0, right: 0, bottom: 0 }`) - better Rust than C's
     indeterminate value, and the token the gate looks for.
- **No unit includes this mirror** (`rg -n 'include/w32term' femacs` -> no hits); it is compiled by
  `xtask layout-check`'s single crate (the `windowsnt` arm is inert there).
- `CARGO_TARGET_DIR=/tmp/femacs-target-s188 cargo run -q -p xtask -- layout-check` ->
  `layout-check: 91 of 101 mirror(s) compiled, 10 platform-exclusive skipped (...)`,
  `layout probe ok`, `layout-check passed` (this mirror is among the 91).
- `build-tu src/w32term.h` -> `error: cannot find macro 'eassert' in this scope` (then `XUNTAG`):
  the headers C 21-23 includes reach `lisp.h`'s macros through the *unit's* own includes
  (PORTING.md 0.1-9) - the class every header mirror shares (`src/charset.h`, `src/nsterm.h`,
  `src/textconv.h`, `src/xwidget.h`, `src/w32gdiplus.h`, `src/pgtkgui.h`, `src/widget.h`,
  `src/w32font.h`, `src/w32gui.h`, `src/nsxwidget.h` all measured the same way).  The compile
  evidence for this mirror is `layout-check`.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures.

# ----------------------------------------------------------------------
## Two spellings that could not compile at all (2026-10-05, `docs/ports/w32fns.md` owed 6 and 7)

**1. `FRAME_DISPLAY_INFO`'s body (C 472).**  C's `((void) (f), (&one_w32_display_info))` had been
transcribed as `{ (void) ($f); (&raw mut one_w32_display_info) }`, and `(void) ($f)` is not Rust:
the macro was a *syntax* error at every consumer, so three of `docs/ports/w32fns.md`'s arm-compile
errors (`x = FRAME_DISPLAY_INFO (f)->…`) were this line rather than the unit's text.  It is now
`{ let _ = ($f); (&raw mut one_w32_display_info) }` - the `let _ =` shape the frozen
`include/frame.rs` accessors use for C's `((void) (f), …)` arms, which still evaluates the
argument.

**2. `w32_kbdhook_active` (C 809).**  C: `#define w32_kbdhook_active (os_subtype !=
OS_SUBTYPE_9X)` - a macro over a **mutable global** (`w32common.h` 48's `extern int os_subtype`),
read as a value at its call sites (`w32inevt.c` 153/157/194/211/278, `w32fns.c` 3080/3088).  The
mirror had a `pub const` whose value read that global: `E0080`/`E0133` at the arm compile (the
`w32fns.md` reading), because a `const` may not evaluate a mutable static.  It is now a nullary
`macro_rules!` like C's `#define` (body `(unsafe { os_subtype } != OS_SUBTYPE_9X)`), so the name
answers per call site, exactly as C's does; `os_subtype` is the same shield's crate-root object
(`include/w32common.rs` 95) that the unit reaches through C 26's include.  C 811-812's `#else` arm
(`#define w32_kbdhook_active 0`) stays recorded, as before.

**Evidence.**

- `python3 tools/extract-abi.py --check emacs/src/w32term.h src/include/w32term.rs` -> `OK
  emacs/src/w32term.h -> src/include/w32term.rs`.
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`.
- No arm compile is possible here (no Windows toolchain); the two fixes are text-level and their
  reading is `docs/ports/w32fns.md`'s next arm run.

# ----------------------------------------------------------------------
## The declaration homes: 27 names move into modules (2026-10-05, `docs/ports/w32term.md` owed 4 and `docs/ports/w32fns.md` owed 5)

**C.**  Twenty-seven of this header's prototypes and objects are names a `HAVE_NTGUI`
translation unit *defines*: `src/w32term.c` (the unit that includes this mirror through C 22's
`#include "w32term.h"`) defines seven functions and sixteen objects, and `src/w32fns.c` - which
reaches the same header through its own `TERM_HEADER` - defines four more of them.  A mirror
declaration at the crate root collides with the unit's own item as `E0428` the moment such a unit
is compiled, which is why **no `have_ntgui` translation unit could compile this file**.

  * `src/w32term.c`'s seven: `XCreateGC` is `w32fns.c`'s, the rest are this unit's -
    `w32_fill_rect`/`w32_clear_window` (C 636-637), `w32_display_pixel_height`/
    `w32_display_pixel_width` (C 247-248), `w32_release_paint_buffer`/`w32_flip_buffers_if_dirty`
    (C 911-912); its sixteen objects: `x_display_list`/`one_w32_display_info` (C 223-224),
    `dwWindowsThreadId`/`hWindowsThread`/`dwMainThreadId`/`hMainThread` (C 731-734),
    `w32_keyboard_codepage` (C 846), `w32_system_remap_font` (C 875), the seven
    `w32_system_caret_*` (C 877-883) and `w32_message_fd` (C 928);
  * `src/w32fns.c`'s four: `w32_fullscreen_rect` (C 77), `w32_window_to_frame` (C 230),
    `w32_real_positions` (C 232) and the object `menubar_in_use` (C 228).

**The change.**  The declarations now sit in `pub mod w32term_fn_decls` and
`pub mod w32term_objs`, each `use super::*;`, followed by `use w32term_fn_decls::*;` /
`use w32term_objs::*;` - the `window.rs` 302-367 `window_fn_decls` / `androidterm.rs` 513-537
`androidterm_objs` shape (PORTING.md 0.1-10 / P-21).  A crate-root definition *shadows* a glob
import, so the unit's own item stays the live one, and `tools/check-mirror-dupes.py` counts a
declaration with a body as a definition and never as a copy.  Every item keeps the `/// C:` line
of the site it moved from; those sites now carry a one-line pointer comment.

**The names left at the crate root, and why.**  The rest of this header's prototype block
(`w32_term_init`, `w32_define_color`, the `syms_of_*` family, `tip_window`, …) stays where it is.
It is not *measured* against any unit yet - the two Windows units this tree has (`src/w32term.rs`,
`src/w32fns.rs`) are slices, and each new slice will move the names it defines with the same
treatment.  Moving the whole block now would be a claim no reading supports.

**Evidence.**

- The flip `docs/ports/w32term.md` owed 4 exists for, re-run on the revision of this batch in an
  isolated worktree (`git worktree add /tmp/… HEAD`, this file's copies over HEAD's):
  `xtask probe src/w32term.rs … --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'` ->
  **13 error lines / 26 `error[E0428]` before, 12 error lines / 0 `E0428` after**.
  `xtask probe src/w32fns.rs`, same cfgs -> **15 error lines / 12 `E0428` before, 7 / 0 after**
  (the 26 at HEAD are the 22 of this file's names a `windowsnt` crate reaches - `w32_message_fd`
  needs `cygwin` too - plus the four `include/w32.rs` collisions this batch fixes separately; the
  12 are these 10 plus `one_w32_display_info` and `x_display_list`).
- The residual 12 in `src/w32term.rs` are the *next* layer, all of them unit text or another
  mirror's gap: `include/frame.rs`'s `HAVE_NTGUI` flag-run `compile_error!` (1), the
  `<windows.h>` typedefs `SYSTEM_INFO`/`OSVERSIONINFO` the unit declares none of (2), the
  `w32_output` bit-field members `want_paint_buffer`/`paint_buffer_dirty` (4, `docs/ports/w32term.md`
  owed 6 - the accessor pair is the unit's call sites' change), `cursor_pixel` off a mistyped
  `*mut w32_output` (2) - all of them `E0609`/`E0606`/`E0425` in the *unit's* text.
  `src/w32fns.rs`'s residual 5 are the same two typedefs and two `Emacs_GC` sites (the
  `have_ntgui` arm of `dispextern.h` 45-77, a `dispextern.rs` gap this lane did not close).
- `python3 tools/extract-abi.py --check emacs/src/w32term.h src/include/w32term.rs` -> `OK`.
- `cd femacs && cargo run -q -p xtask -- layout-check` -> `layout probe ok`, `layout-check passed`.
- `python3 tools/check-citations.py` -> `0 drift`; `python3 tools/check-declarations.py` ->
  `13 declaration sites (0 forbidden, 13 allowed literals), 0 failures`;
  `python3 tools/check-mirror-dupes.py` -> `18 ratchet row(s), 0 failure(s)` for this file;
  `tools/check-all-units.sh` -> `checked 83 unit(s), 0 failure(s)` / `checking 83 unit(s),
  0 unit failure(s)`.

## `w32term_fns_decls`: the three `src/w32fns.c` definitions move into a module (2026-10-14, w32fns slice 25)

Three names this mirror declared at its crate root under `#[cfg (feature = "windowsnt")]` are
**`src/w32fns.c`'s definitions**, not `src/w32term.c`'s: `w32_kbd_patch_key` (C 10208),
`w32_kbd_mods_to_emacs` (C 10143) and `w32_sys_ring_bell` (C 10291) - `grep -n '^w32_kbd_patch_key'
emacs/src/*.c` shows the one definition each, and the two comments that called them this header's own
unit were wrong.  When w32fns slice 25 wrote the three definitions, the arm probe read three
`E0428`s; the fix is P-21/P-39's device, which this file already uses twice
(`w32term_fn_decls`, `w32term_xfns_decls`): a `pub mod w32term_fns_decls` plus
`use w32term_fns_decls::*;`, so the unit's own item is the live one and every caller still resolves
the name through the glob import.  Nothing else in the mirror changed.

## `w32_frame_parm_handlers` moves into `w32term_objs` (2026-10-14, w32fns slice 35)

C 274's `extern frame_parm_handler w32_frame_parm_handlers[];` was the one declaration left at this
mirror's crate root that a `HAVE_NTGUI` unit defines: `src/w32fns.c` C 11530-11583 **is** the object,
so the declaration and w32fns slice 35's `#[unsafe(no_mangle)] pub static mut
w32_frame_parm_handlers: [frame_parm_handler; 51]` agree in the value namespace and the arm probe
would read `E0428` - the same collision the 2026-10-05 section above describes for twenty-seven
names.  The declaration therefore moved into the *existing* `pub mod w32term_objs`, which this
mirror already glob-imports (`use w32term_objs::*;`, line 1534), and its `/// C:` line went with it;
the crate-root site now carries a one-line pointer comment, the convention that section records.
Nothing else in the mirror changed, and the declaration keeps the `[*mut c_void; 0]` spelling its
own note explains (`dispextern.h` 3063's `frame_parm_handler` is not in `include/dispextern.rs`, and a
nested module's `use super::*` cannot see a name the crate root glob-imported - the `HMENU`
distinction in `w32term_objs`' note).

**Evidence.**  `cd femacs && CARGO_TARGET_DIR=/tmp/w35lane cargo run -q -p xtask -- probe
src/w32fns.rs /tmp/w35lane/x.rlib --crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"'
--cfg 'feature="windowsnt"'` -> **42 errors**, the same fifteen `E0428` names as before the move and
none for `w32_frame_parm_handlers`; `build-tu src/w32fns.c` -> exit 0, the object 336 bytes;
`python3 tools/extract-abi.py --check emacs/src/w32term.h src/include/w32term.rs` -> `OK`;
`python3 tools/check-mirror-dupes.py` -> 0 failure(s).  The unit-side readings are
`docs/ports/w32fns.md`'s `## Slice 35 (C 11522-11583)`.

## `globals_of_w32fns`/`syms_of_w32fns` move into `w32term_fns_decls` (2026-10-14, w32fns slices 37-38)

C 918-919's `extern void globals_of_w32fns (void); extern void syms_of_w32fns (void);` stood at this
mirror's crate root (the `#[cfg (feature = "windowsnt")] unsafe extern "C"` block at line 1381) with
seven other `syms_of_*`/`globals_of_*` prototypes.  `src/w32fns.c` **is** their definition, so
w32fns slice 38's `#[unsafe(no_mangle)] pub unsafe extern "C" fn globals_of_w32fns` read
`E0428 the name globals_of_w32fns is defined multiple times` on the arm probe - the same collision
the 2026-10-05 section above describes for twenty-seven names and the 2026-10-14 section for
`w32_frame_parm_handlers`.  Both names moved into the *existing* `pub mod w32term_fns_decls` (the
module that already holds `src/w32fns.c`'s three `w32_kbd_*` definitions and is glob-imported at
line 1609), each keeping its `/// C:` line, with a one-line pointer comment at the old site.  Moving
`syms_of_w32fns` at the same time is deliberate: it is the same file's definition and the next
slice lands it, so the mirror needs no second edit.

**Evidence.**  `cd femacs && CARGO_TARGET_DIR=/tmp/w38lane cargo run -q -p xtask -- probe
src/w32fns.rs /tmp/w38lane/x.rlib --crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"'
--cfg 'feature="windowsnt"'` -> **43 errors / one `E0428` for `globals_of_w32fns` before, 42 after**;
`build-tu src/w32fns.c` -> exit 0, the object 336 bytes;
`python3 tools/extract-abi.py --check emacs/src/w32term.h src/include/w32term.rs` -> `OK`;
`python3 tools/check-mirror-dupes.py` -> 0 failure(s).  The unit-side readings are
`docs/ports/w32fns.md`'s `## Slice 38 (C 12283-12450)`.

## `parse_button` moves into `w32term_fn_decls` (2026-10-14, w32term slice 7)

**The measurement.**  `src/w32term.h` 776 declares `extern BOOL parse_button (int, int, int *, int
*);` and `src/w32term.c` 3439 defines it, so the mirror declared it - at its crate root, in the
block that says the four names there "are `src/w32term.c`'s own".  That is exact as long as no unit
defines the name; the w32term slice-7 round did, and its arm reading went from **22 to 23** with
`E0428: the name \`parse_button\` is defined multiple times` (the other two `E0428`s of that
reading, `w32_query_frame_background_color` and `w32_image_rotations_p`, are the mirror's own
standing rows - Owed 4's list).

**The move.**  The declaration now stands in `w32term_fn_decls`, the glob-imported module that
already holds the file's other own definitions (`w32_window_to_frame`, `w32_clear_window`,
`w32_real_positions`, ...), with the reading in its comment.  A crate-root definition shadows a glob
import, so the unit's `parse_button` wins and the `E0428` disappears - the P-21/P-39 device this
ledger's "The declaration homes" section records for 27 earlier names.

**Evidence.**  `cd femacs && cargo run -q -p xtask -- probe src/w32term.rs /tmp/w46/x.rlib
--crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg
'feature="have_window_system"' --cfg 'feature="dispextern_face_records"'` -> **23 errors before,
22 after** (and 0 anchored in the unit's text either way);
`python3 tools/extract-abi.py --check emacs/src/w32term.h src/include/w32term.rs` -> `OK`;
`python3 tools/check-mirror-dupes.py` -> 0 failure(s).  The unit-side readings are
`docs/ports/w32term.md`'s `### Slice 7 (C 3433-3629)`.

## `w32_lispy_file_action` follows `parse_button` into `w32term_fn_decls` (2026-10-14, w32term slice 8)

**The measurement.**  `src/w32term.h` 795 declares `extern Lisp_Object w32_lispy_file_action
(DWORD);` and `src/w32term.c` 3636 defines it, so the slice-8 round's arm reading (with the base
configuration plus `--cfg feature="have_w32notify"`) went from **22 to 23** with `E0428: the name
\`w32_lispy_file_action\` is defined multiple times`.

**The move.**  The declaration now stands beside `parse_button` in `w32term_fn_decls` (the
glob-imported module for the file's own definitions), and the reading returned to **22**.  This is
the second name of the `HAVE_W32NOTIFY`/mouse block the module absorbs; the two standing `E0428`s of
the mirror (`w32_query_frame_background_color`, `w32_image_rotations_p`) are unchanged and remain
Owed 4's.

**Evidence.**  `cd femacs && cargo run -q -p xtask -- probe src/w32term.rs /tmp/w47/x.rlib
--crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg
'feature="have_window_system"' --cfg 'feature="dispextern_face_records"' --cfg
'feature="have_w32notify"'` -> **23 before, 22 after**, 0 anchored in the unit's text either way;
`python3 tools/extract-abi.py --check emacs/src/w32term.h src/include/w32term.rs` -> `OK`;
`python3 tools/check-mirror-dupes.py` -> 0 failure(s).  The unit-side readings are
`docs/ports/w32term.md`'s `### Slice 8 (C 3631-3763)`.

## `w32_output`'s `asked_for_visible` joins the packed word (2026-10-15, w32term slice 16)

**The measurement.**  `src/w32term.c` 5190 reads `f->output_data.w32->asked_for_visible`, and the
mirror recorded C 394/397/400's three bitfields (`explicit_parent`, `asked_for_visible`,
`menubar_active`) as **text only**, so the reading gave `E0609: no field \`asked_for_visible\` on
type \`w32_output\`` - the same missing-member class the two `paint_buffer` bits had before they
were packed.

**The change.**  C 397's member is the *third* bit of the same `bits` word the record already
carries for C 436/439 (`W32_OUTPUT_ASKED_FOR_VISIBLE_BIT`, `1 << 2`, with the two
`w32_output_bits! (asked_for_visible, …)` arms).  Adding a bit to a word that already stands
changes no offset; C 378/394/400's other three stay recorded-only.  C 7517/7538 read the same
member in later slices.

**Evidence.**  `cd femacs && cargo run -q -p xtask -- probe src/w32term.rs /tmp/w50/x.rlib
--crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg
'feature="have_window_system"' --cfg 'feature="dispextern_face_records"'` -> **22 errors, 0 anchored
in the unit's text** (the row above is gone, the standing count unchanged);
`python3 tools/check-mirror-dupes.py` -> 0 failure(s); `python3 tools/check-declarations.py` -> 0
failure(s).  The unit-side readings are `docs/ports/w32term.md`'s `### Slice 16 (C 5130-6458)`.

## Three of this mirror's frame accessors gain their write arms (2026-10-15, w32term slice 18)

**The measurement.**  `src/w32term.c` 6854/6860/6861 assign to `FRAME_FONTSET`, `FRAME_FONT` and
`FRAME_BASELINE_OFFSET`, and this mirror carried each as a *reader* alone (`($f:expr) => { … }`),
so the reading gave three `E0061`-class rows (the macro takes one argument and was given two).

**The change.**  Each of the three macros gained the two-argument arm over C's own member text
(`(*(( *$f).output_data.w32 as *mut w32_output)).font/fontset/baseline_offset = $v`), which is the
shape `src/androidterm.rs` 2739-2757 carries for `frame.h`'s accessors and `src/w32term.rs`'s own
`w32_output_bits!`/`window_bits!` pairs already use.  No storage changes: the arms write the
members the readers read.

**Evidence.**  `cd femacs && cargo run -q -p xtask -- probe src/w32term.rs /tmp/w51/x.rlib
--crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg
'feature="have_window_system"' --cfg 'feature="dispextern_face_records"'` -> **22 errors, 0
anchored in the unit's text**; `python3 tools/check-mirror-dupes.py` and `tools/check-declarations.py`
-> 0 failure(s) each.  The unit-side readings are `docs/ports/w32term.md`'s
`### Slice 18 (C 6754-6906)`.

## `FRAME_PREV_FSMODE` gains its write arm (2026-10-15, w32term slice 19)

**The measurement.**  `src/w32term.c` 7093 stores into `FRAME_PREV_FSMODE (f)`, and this mirror
carried the accessor as a reader alone, so the reading gave the `E0061`-class row the slice-18
round recorded for its three siblings.

**The change.**  The two-argument arm writes C's own member
(`(*(( *$f).output_data.w32 as *mut w32_output)).prev_fsmode = $v`).  Together with slice 18's
three, this mirror's four `w32_output` frame accessors now carry both arms; no storage changes.

**Evidence.**  `cd femacs && cargo run -q -p xtask -- probe src/w32term.rs /tmp/w52/x.rlib
--crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg
'feature="have_window_system"' --cfg 'feature="dispextern_face_records"'` -> **22 errors, 0
anchored in the unit's text**; `python3 tools/check-mirror-dupes.py` and
`tools/check-declarations.py` -> 0 failure(s) each.  The unit-side readings are
`docs/ports/w32term.md`'s `### Slice 19 (C 6908-7146)`.

## Nine unit-defined prototypes move into `w32term_fn_decls` (2026-10-15, w32term slice 21)

**The measurement.**  `src/w32term.c` 7503/7605/7638/7665/7720 define `w32_make_frame_visible`,
`w32_make_frame_invisible`, `w32_iconify_frame`, `w32_free_frame_resources` and
`w32_wm_set_size_hint`, and this mirror declared all five in the crate-root `unsafe extern "C"`
block C 251-258 comes from - so the reading went **22 -> 27** with five `E0428`s.  The same block
held the lane's two standing `Owed 4` rows (`w32_query_frame_background_color` at 338,
`w32_image_rotations_p` in the C 802-805 block), and two more names in that second block
(`w32_initialize_display_info`, `initialize_w32_display`) are `src/w32term.c`'s definitions too, so
they would have collided when the file's tail lands.

**The move.**  All nine now stand in `w32term_fn_decls` - the glob-imported module whose imported
names a local definition shadows (the device `parse_button` and `w32_lispy_file_action` already
use, P-21/P-39).  The two crate-root sites keep C's own text and a note naming the module.

**Evidence.**  `cd femacs && cargo run -q -p xtask -- probe src/w32term.rs /tmp/w53/x.rlib
--crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg
'feature="have_window_system"' --cfg 'feature="dispextern_face_records"'` -> **27 before, 20
after**, 0 anchored in the unit's text either way (the lane's baseline was 22, so the move also
closed its two standing `Owed 4` rows); `python3 tools/check-mirror-dupes.py` and
`tools/check-declarations.py` -> 0 failure(s) each.  The unit-side readings are
`docs/ports/w32term.md`'s `### Slice 21 (C 7503-7734)`.

## `w32_output`'s `hourglass_p` joins the packed word (2026-10-15, w32term slice 22)

**The measurement.**  `src/w32term.c` 7763/7765 read and write `f->output_data.w32->hourglass_p`
(`w32_show_hourglass`/`w32_hide_hourglass`), and C 378's member was recorded as text only, so the
accessor pair had no member to name.

**The change.**  C 378's bit becomes `W32_OUTPUT_HOURGLASS_P_BIT: c_uint = 1 << 3` with the two
`w32_output_bits! (hourglass_p, …)` arms - the same shape, and the same `1 << n` numbering, the
slice-16 `asked_for_visible` bit uses.  Adding a bit to a word that already stands changes no
offset; C 394/397/400's other three members stay recorded-only.

**Evidence.**  `cd femacs && cargo run -q -p xtask -- probe src/w32term.rs /tmp/w22/x.rlib
--crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg
'feature="have_window_system"' --cfg 'feature="dispextern_face_records"'` -> **20 errors, 0 anchored
in the unit's text** (the lane's baseline); `python3 tools/check-mirror-dupes.py` and
`tools/check-declarations.py` -> 0 failure(s) each.  The unit-side readings are
`docs/ports/w32term.md`'s `### Slice 22 (C 7736-7864)`.

## `w32_arrow_cursor`'s prototype moves into `w32term_fn_decls` (2026-10-15, w32term slice 22)

**The measurement.**  C 7787 defines `w32_arrow_cursor` and this mirror declared it in the
crate-root `unsafe extern "C"` block, so the definition shadowed nothing and the reading would
have carried an `E0428` - the class the slice-21 move closed nine of.

**The change.**  The prototype now stands in `w32term_fn_decls`, the glob-imported module a local
definition shadows (P-21/P-39); the crate-root site keeps C's own text and a note naming the
module.  This is the same move slice 21 made for the nine names that were about to be defined.

**Evidence.**  The probe row above - **20 errors, 0 anchored in the unit's text**, the lane's
baseline, so the move added no collision; `python3 tools/check-mirror-dupes.py` ->
0 failure(s).  The unit-side readings are `docs/ports/w32term.md`'s `### Slice 22 (C 7736-7864)`.

## `w32_term_init` and `w32_delete_display` move into `w32term_fn_decls` (2026-10-15, w32term slice 25)

**The measurement.**  `src/w32term.c` 8044 and 8115 define `w32_term_init` and
`w32_delete_display`, and this mirror declared both at the crate root (C 243-244's
`extern struct w32_display_info *w32_term_init (Lisp_Object, char *, char *);` and C 783's
`extern void w32_delete_display (struct w32_display_info *dpyinfo);`), so the reading would have
carried two `E0428`s - the class slice 21 closed nine of and slice 22 one of.

**The move.**  Both prototypes now stand in `w32term_fn_decls`, the glob-imported module a local
definition shadows (P-21/P-39); the two crate-root sites keep C's own text and a note naming the
module.

**Evidence.**  `cd femacs && cargo run -q -p xtask -- probe src/w32term.rs /tmp/w25/y.rlib
--crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg
'feature="have_window_system"' --cfg 'feature="dispextern_face_records"'` -> **20 errors, 0 anchored
in the unit's text** (the lane's baseline), and the `have_ntgui` + `windowsnt` arm stays at **78**;
`python3 tools/check-mirror-dupes.py` and `tools/check-declarations.py` -> 0 failure(s) each.  The
unit-side readings are `docs/ports/w32term.md`'s `### Slice 25 (C 8044-8138)`.

## `syms_of_w32term` and `w32_init_main_thread` move into `w32term_fn_decls` (2026-10-15, w32term slice 26)

**The measurement.**  `src/w32term.c` 8141 and 8272 define `w32_init_main_thread` and
`syms_of_w32term`, and this mirror declared both at the crate root (C 925's
`extern void w32_init_main_thread (void);` and C 914's `extern void syms_of_w32term (void);` in
the C 909-925 block), so the pair would have carried two `E0428`s - the class slice 21 closed
nine of, slice 22 one and slice 25 two.

**The move.**  Both prototypes now stand in `w32term_fn_decls`, the glob-imported module a local
definition shadows (P-21/P-39); the crate-root site keeps C's own text and a note naming the
module.  The two names are the last ones `src/w32term.c` defines that this header declares.

**Evidence.**  `cd femacs && cargo run -q -p xtask -- probe src/w32term.rs /tmp/w26/x.rlib
--crate-type=lib -C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg
'feature="have_window_system"' --cfg 'feature="dispextern_face_records"'` -> **20 errors, 0 anchored
in the unit's text** (the lane's baseline), and the `have_ntgui` + `windowsnt` arm stays at **78**;
`python3 tools/check-mirror-dupes.py` and `tools/check-declarations.py` -> 0 failure(s) each.  The
unit-side readings are `docs/ports/w32term.md`'s `### Slice 26 (C 8139-8271)`.
