# emacs/src/w32fns.c (12488 lines)

> **The largest file in P-86's class, and P-86 ④'s "first slice": C 1-884.**
> `configure.ac` 3023 puts `w32fns.o` in `W32_OBJ` under `if test "${HAVE_W32}" = "yes"`, which is
> the same test that `AC_DEFINE`s `HAVE_NTGUI` (3024); `docs/baseline/frozen-build-sources.txt`
> lists no `w32fns.o` for this target, so `extract-abi.py`'s checklist for the file is empty and
> P-86 ②'s substitute is what follows: the **C line-range table**, the `## Evidence` readings and
> the `## Deviations`.  Neither `WINDOWSNT` nor `HAVE_NTGUI` is defined in
> `docs/baseline/config/darwin-arm64.macros.tsv`, so the frozen configuration compiles nothing from
> this file (measured: the 336-byte empty object under Evidence ①).
>
> **Why 884 and not 900.**  P-86 ④ says to stop at a whole-item boundary; C 897-900 is the middle
> of a call expression inside `add_system_logical_colors_to_map` (C 887-932), and the nearest
> boundaries are C 884 (the `}` of `w32_color_map_lookup`) and C 932 (the `}` of
> `add_system_logical_colors_to_map`).  C 884 is the nearer one (16 lines from 900 against 32), so
> the slice is C 1-884: the opening declaration block, the SDK/type section, the mouse, hook and
> timer state, the four functions `check_x_display_info`/`w32_window_to_frame`/
> `w32_real_positions`/`w32_fullscreen_rect`, the `w32-define-rgb-color` subr, the 240-entry colour
> map and its two lookups.
>
> **This batch is `text-only`** (P-86 ③⑵).  The machine has **no Windows and no mingw toolchain**:
> `which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no Homebrew
> `mingw-w64`/`w64` formula is installed and `find / -name windows.h -path '*mingw*'` (depth 4)
> finds nothing, so there is **no `<windows.h>` to compile or measure against** and **no arm flip
> on a Windows target**.  What this batch carries instead: the frozen `build-tu` reading, P-86 ②'s
> machine-checked coverage table, the `#if` verdicts, a per-item C citation, and an **arm-reach
> pre-flight** that compiles this unit's own text against the real mirrors plus stubs (Evidence ③)
> - which is a `/tmp` harness reading and **not** a Windows-target compile.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, i.e. `HAVE_W32` → `HAVE_NTGUI`, configure.ac 3023-3024) | `feature = "have_ntgui"` (on every item and on C 27-97's include graph) | native MS Windows GUI, incl. the Cygwin GUI build |
| C 38 `#if !defined MINGW_W64 && !defined CYGWIN` | `all (not (feature = "mingw_w64"), not (feature = "cygwin"))` | MinGW (not mingw-w64) and MSVC |
| C 61 `#ifdef WINDOWSNT` … C 72 | `feature = "windowsnt"`, and C 63 `#ifndef MINGW_W64` → `not (feature = "mingw_w64")` | the non-Cygwin Windows build |
| C 74 `#if CYGWIN` … C 78 `#else` | `feature = "cygwin"` / `not (feature = "cygwin")` | Cygwin GUI vs MS-Windows GUI |
| C 167 `#if _WIN32_WINDOWS >= 0x0410` … C 178 | `feature = "win32_windows_ge_0410"` / `not (…)` | both arms carried (see the Deviations) |
| C 292 `#ifdef GLYPH_DEBUG` … C 294 | `feature = "glyph_debug"` | a `GLYPH_DEBUG` build |
| C 310 `#ifdef WINDOWSNT` … C 313 | `feature = "windowsnt"` | the non-Cygwin Windows build |
| C 334 `#ifdef HAVE_NTGUI` … C 336 | the file gate itself | every build of this file |
| C 344 `#ifdef WINDOWSNT` … C 350 (C 345 `#ifndef WM_WTSSESSION_CHANGE` nested) | `feature = "windowsnt"`; the nested arm is a "the SDK has not got it" guard | the non-Cygwin Windows build |
| C 3617 `#if EMACSDEBUG` … C 3674 `#endif` | `feature = "emacsdebug"` | an `EMACSDEBUG` build |
| C 3816 `#ifdef MSG_DEBUG` … C 3820 `#endif` | `feature = "msg_debug"` | a `MSG_DEBUG` build |

`not_android_stubify`-style sub-gates do not exist here: the file has exactly one build condition
plus the four platform macros above.  `tools/check-cond-eval.py src/w32fns.c` reads the file as
`117 conditions, 3 unknown, 3 disagreements [NOT VERIFIED]` (C 167, C 8286 and C 8300 are
`_WIN32_WINDOWS`/`_WIN32_WINNT` value tests clang calls False and the evaluator keeps live), so the
file is **not** on `docs/baseline/config/cond-eval-verified.txt` and every other condition's both
arms stay live text as well - both arms of each `#if` are carried below.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block (C 18's `*/` closes it) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20 | `/* Added by Kevin Gallo */` and `#include <config.h>` | ungated (the comment is kept; every unit includes `config.rs`) |
| 22-25 | `#undef _WIN32_WINNT` / `#define _WIN32_WINNT 0x0600` | file gate (the value as a constant; `#undef` has no counterpart) |
| 27-33 | `<signal.h>`, `<stdio.h>`, `<limits.h>`, `<errno.h>`, `<math.h>`, `<fcntl.h>`, `<unistd.h>` | file gate |
| 35 | `<c-ctype.h>` | file gate (`c_isdigit`, C 963) |
| 37 | `#define COBJMACROS` | file gate (the definition record) |
| 38-40 | `#if !defined MINGW_W64 && !defined CYGWIN` / `# define INITGUID` / `#endif` | both arms carried (`mingw_w64`/`cygwin`) |
| 41-46 | `<initguid.h>`, `<shlobj.h>`, `<shobjidl.h>`, `<oleidl.h>`, `<objidl.h>`, `<ole2.h>` | file gate (no mirror: the C include list stands as text, P-88 ①) |
| 48-59 | `lisp.h`, `w32term.h`, `w32font.h`, `frame.h`, `window.h`, `buffer.h`, `keyboard.h`, `blockinput.h`, `coding.h`, `w32common.h`, `w32inevt.h` | file gate |
| 61-72 | `#ifdef WINDOWSNT`: C 63-67 the `WTS_VIRTUAL_CLASS` enum, C 69-71 `<mbstring.h>`/`<mbctype.h>`/`<wtsapi32.h>` | `windowsnt` (nested) / `not (mingw_w64)` for the enum |
| 74-78 | `#if CYGWIN` `cygw32.h` / `#else` `w32.h` / `#endif` | both arms carried (`cygwin` / `not (cygwin)`) |
| 80 | `<pdumper.h>` | file gate |
| 82-95 | `<basetyps.h>`, `<unknwn.h>`, `<commctrl.h>`, `<commdlg.h>`, `<shellapi.h>`, `<shlwapi.h>`, `<ctype.h>`, `<winspool.h>`, `<objbase.h>`, `<dlgs.h>`, `<imm.h>`, `<windowsx.h>`, `<gdiplus.h>` | file gate; **the `<windows.h>` name section stands here** (see the Deviations) |
| 97 | `<w32gdiplus.h>` | file gate |
| 99-103 | the dark-mode note and `DARK_MODE_APP_NAME` | file gate (a wide literal as an array) |
| 104-111 | the two `DWMWA_USE_IMMERSIVE_DARK_MODE*` `#ifndef` guards | file gate (guards: constant + C's value) |
| 113-115 | the `FOF_NO_CONNECTED_ELEMENTS` `#ifndef` guard | file gate (guard) |
| 117-119 | the three entries of `w32proc.c`/`w32menu.c` | file gate (`unsafe extern`) |
| 121-123 | the `IDC_HAND` `#ifndef` guard | file gate (guard + `MAKEINTRESOURCE`) |
| 125-127 | `SYSTEM_COLOR_PREFIX` and its `sizeof`-based length | file gate |
| 129-132 | `LMOUSE`/`MMOUSE`/`RMOUSE` | file gate |
| 134-138 | the six mouse-emulation statics | file gate |
| 140-141 | `track_mouse_window` | file gate |
| 143-156 | the four monitor `#ifndef` guards | file gate (guards) |
| 157-165 | the MONITORINFO note and `struct MONITOR_INFO` | file gate |
| 167-178 | the `C_CHILDREN_TITLEBAR`/`TITLEBAR_INFO` pair | both arms carried (`win32_windows_ge_0410`) |
| 180-182 | the `CCHDEVICENAME` `#ifndef` guard | file gate (guard) |
| 183-190 | `struct MONITOR_INFO_EX` | file gate |
| 192-247 | the 23 function-pointer typedefs and the gesture records | file gate |
| 249-272 | the 23 SDK pointer statics, the two `extern`+definition pairs and the seven `static` ones | file gate |
| 274 | `unicode_append_menu` | file gate / `windowsnt` (the mirror's alias) |
| 276-277 | `ignore_ime_char` | file gate |
| 279-281 | `msh_mousewheel` (`extern` + definition) | file gate |
| 283-290 | the four timer macros and `menu_free_timer` | file gate |
| 292-294 | `#ifdef GLYPH_DEBUG` / `dpyinfo_refcount` | `glyph_debug` (nested) |
| 296-300 | `w32_visible_system_caret_hwnd`, `w32_unicode_gui`, `w32_selection_dialog_open` | file gate |
| 302-303 | `menubar_in_use` | file gate |
| 305-308 | the `w32uniscribe.c` externs | file gate |
| 310-313 | `#ifdef WINDOWSNT` / `faked_key` / `#endif` | `windowsnt` (nested) |
| 315-319 | `sysinfo_cache`, `osinfo_cache` (the two objects; the records are `include/w32common.rs`'s) - resolved by slice 24 | file gate + `windowsnt` |
| 321-326 | `syspage_mask`, `w32_major_version`, `w32_minor_version`, `w32_build_number` | file gate |
| 328-329 | `w32_darkmode` | file gate |
| 331-332 | `os_subtype` | file gate |
| 334-336 | `#ifdef HAVE_NTGUI` / `hinst` | the file gate itself |
| 338-339 | `sound_type`, `MB_EMACS_SILENT` | file gate |
| 341-342 | `VK_ANY` | file gate |
| 344-350 | `#ifdef WINDOWSNT` / `#ifndef WM_WTSSESSION_CHANGE` / the two constants | `windowsnt` (nested); the inner arm is a guard |
| 352-354 | the `WS_EX_NOACTIVATE` `#ifndef` guard | file gate (guard) |
| 356-374 | the `kbdhook` record and object | file gate (the anonymous record is named) |
| 375 | `GetConsoleWindow_Proc` | file gate |
| 377-378 | `keyboard_handle` | file gate (`unsafe extern`) |
| 380-383 | three `static` forward declarations | dropped (PORTING.md 0.1-12) |
| 385-422 | `check_x_display_info` | file gate |
| 424-443 | `w32_window_to_frame` | file gate |
| 445-461 | eleven `static` forward declarations | dropped (PORTING.md 0.1-12) |
| 463-488 | `w32_real_positions` | file gate |
| 490-541 | `w32_fullscreen_rect` (C 517-540 the `switch`) | file gate |
| 543-582 | `DEFUN ("w32-define-rgb-color", …)` (C 563 the `RGB` use) | file gate |
| 584-589 | `typedef struct colormap_t` | file gate |
| 591-833 | `w32_color_map[240]` | file gate |
| 835-854 | `w32_default_color_map` | file gate |
| 856-884 | `w32_color_map_lookup` | file gate |
| 885-932 | `add_system_logical_colors_to_map` (C 890-932's registry walk; the `#if 0`-free half) | file gate |
| 933-978 | `x_to_w32_color` (C 961's bare `alloca` = the stack pool) | file gate |
| 979-1016 | `w32_regenerate_palette` (C 993's bare `alloca`; `LOGPALETTE`) | file gate (`#[unsafe(no_mangle)]`) |
| 1017-1093 | C 1019-1027 `W32_COLOR`/`SET_W32_COLOR`; C 1029-1093 `#if 0` (`w32_map_color`/`w32_unmap_color`) | file gate; the `#if 0` is dead text carried as C's own text in a comment |
| 1094-1186 | `gamma_correct` (C 1102 `pow`), `w32_defined_color` (C 1140-1142's `W32_COLOR`/`SET_W32_COLOR`) | file gate (`#[unsafe(no_mangle)]` both) |
| 1187-1208 | `w32_decode_color` | file gate |
| 1209-1242 | the `gui_set_frame_parameters` comment block and `w32_set_foreground_color` | file gate |
| 1243-1260 | `w32_set_background_color` | file gate |
| 1261-1420 | `w32_set_mouse_color` (C 1265-1268 and C 1278-1417 are `#if 0`) | file gate; both `#if 0` regions as C's own text in comments |
| 1421-1463 | `w32_set_cursor_color` | file gate |
| 1464-1480 | `w32_set_border_pixel` | file gate |
| 1481-1497 | `w32_set_border_color` | file gate |
| 1498-1504 | `w32_set_cursor_type` | file gate |
| 1505-1531 | `w32_set_icon_type` (C 1517's terminal hook) | file gate |
| 1532-1583 | `w32_set_icon_name` (C 1552-1583 is `#if 0`) | file gate; the `#if 0` as C's own text in a comment |
| 1584-1634 | `w32_clear_under_internal_border` (its 1581-1586 doc comment) | file gate (`#[unsafe(no_mangle)]`) |
| 1635-1666 | `w32_set_child_frame_border_width` | file gate |
| 1667-1693 | `w32_set_internal_border_width` | file gate |
| 1694-1750 | `w32_set_menu_bar_lines` | file gate |
| 1751-1775 | `w32_set_tab_bar_lines` | file gate |
| 1776-1848 | `w32_change_tab_bar_height` | file gate (`#[unsafe(no_mangle)]`) |
| 1849-1874 | `w32_set_tool_bar_lines` | file gate |
| 1875-1908 | `w32_set_tool_bar_position` | file gate |
| 1909-1930 | `w32_set_inhibit_double_buffering` (C 1921-1924's `want_paint_buffer` bits) | file gate; `w32_output_bits!` (3.575) |
| 1931-1980 | `w32_change_tool_bar_height` | file gate (`#[unsafe(no_mangle)]`) |
| 1981-2025 | `w32_set_title_bar_text` (C 1988-1990 `#ifdef __CYGWIN__`; C 2001-2021's UTF-16 half with C 2013's bare `alloca`) | both arms (`cygwin` / `not (cygwin)`); `not (cygwin)` also carries `ntgui_unicode`? no: the UTF-16 half is the non-Cygwin arm's |
| 2026-2080 | `w32_set_name` (its 2027-2037 comment) | file gate |
| 2081-2089 | `w32_explicitly_set_name` | file gate |
| 2090-2098 | `w32_implicitly_set_name` | file gate (`#[unsafe(no_mangle)]`) |
| 2099-2118 | `w32_set_title` | file gate |
| 2119-2128 | `w32_set_scroll_bar_default_width` | file gate (`#[unsafe(no_mangle)]`) |
| 2129-2139 | `w32_set_scroll_bar_default_height` | file gate (`#[unsafe(no_mangle)]`) |
| 2140-2185 | `w32_set_undecorated` (its 2141-2153 doc comment) | file gate |
| 2186-2248 | `w32_set_parent_frame` (its 2187-2215 doc comment) | file gate |
| 2249-2281 | `w32_set_skip_taskbar` (its 2250-2260 doc comment) | file gate |
| 2282-2299 | `w32_set_no_focus_on_map` (its 2283-2294 doc comment) | file gate |
| 2300-2318 | `w32_set_no_accept_focus` (its 2301-2313 doc comment) | file gate |
| 2319-2374 | `w32_set_z_group` (its 2320-2338 doc comment) | file gate |
| 2375-2379 | the `Subroutines for creating a frame.` banner and C 2379's `w32_load_cursor` forward declaration | banner kept as a comment; the declaration dropped (PORTING.md 0.1-12) |
| 2380-2394 | `w32_load_cursor` | file gate (`#[unsafe(no_mangle)]`) |
| 2395-2406 | C 2396's `w32_wnd_proc` forward declaration (dropped) and C 2398-2406's `INIT_WINDOW_CLASS` macro | declaration dropped (0.1-12); the macro is `macro_rules!` |
| 2407-2428 | `w32_init_class` (C 2424's `L"Emacs"`) | file gate; the wide literal written as its code units |
| 2429-2456 | `w32_applytheme` (its 2430-2432 comment) | file gate |
| 2457-2473 | `w32_createvscrollbar` | file gate |
| 2474-2490 | `w32_createhscrollbar` | file gate |
| 2491-2536 | `process_dropfiles` (its 2492-2494 comment; C 2503-2510 `#ifdef NTGUI_UNICODE`, C 2507-2509's nested `#ifdef CYGWIN`, C 2512-2531 `#else`) | both arms (`ntgui_unicode`), the nested arm `cygwin` |
| 2538-2575 | `w32_process_dnd_data` (its 2538-2539 comment) | file gate (`#[unsafe(no_mangle)]`) |
| 2577-2582 | `struct w32_drop_target` (C 2578-2579: `i_drop_target` must be the first member) | file gate |
| 2584-2588 | `w32_drop_target_QueryInterface` | file gate (`extern "system"` body) |
| 2590-2595 | `w32_drop_target_AddRef` | file gate |
| 2597-2606 | `w32_drop_target_Release` | file gate |
| 2608-2619 | `w32_handle_drag_movement` | file gate |
| 2621-2632 | `w32_drop_target_DragEnter` (its 2623-2626 comment) | file gate |
| 2634-2642 | `w32_drop_target_DragOver` | file gate |
| 2644-2648 | `w32_drop_target_DragLeave` | file gate |
| 2650-2663 | `w32_try_get_data` | file gate |
| 2665-2698 | `w32_drop_target_Drop` | file gate |
| 2700-2857 | `w32_createwindow` (C 2706's function-local `touch_base`, C 2834-2836's `#ifndef GC_ALLGESTURES` guard) | file gate; the guard is an SDK guard (no `cfg`) |
| 2859-2869 | `my_post_msg` (C 380-383's forward declaration dropped, PORTING.md 0.1-12) | file gate |
| 2871-3154 | the `#ifdef WINDOWSNT` keyboard-hook arm: `funhook` (2873-3066), `setup_w32_kbdhook` (3068-3137), `remove_w32_kbdhook` (3139-3153) | file gate + `windowsnt` |
| 3156-3194 | `hook_w32_key` | file gate |
| 3196-3239 | the second `#ifdef WINDOWSNT` arm: `check_w32_winkey_state` (3197-3223), `reset_w32_kbdhook_state` (3225-3238) | file gate + `windowsnt` |
| 3241-3259 | the modifier-family banner, the four `EMACS_*` `#define`s and the three `static int`s | file gate |
| 3261-3283 | `test_modifier_support` | file gate |
| 3285-3302 | `record_keydown` | file gate |
| 3304-3318 | `record_keyup` | file gate |
| 3320-3365 | `reset_modifiers` (C 3349's `CURRENT_STATE` `#define` inside the block) | file gate |
| 3367-3382 | `sync_modifiers` | file gate |
| 3384-3435 | `modifier_set` (C 3415-3418's nested `#ifdef WINDOWSNT`) | file gate; the nested arm `windowsnt` |
| 3437-3487 | `w32_key_to_modifier` (C 3439's forward declaration dropped) | file gate (`#[unsafe(no_mangle)]`) |
| 3489-3500 | `w32_get_modifiers` (C 380-383's forward declaration dropped) | file gate |
| 3502-3525 | `w32_construct_console_modifiers` | file gate |
| 3527-3536 | `w32_get_key_modifiers` | file gate |
| 3538-3559 | `map_keypad_keys` (C 3538's forward declaration dropped) | file gate (`#[unsafe(no_mangle)]`) |
| 3561-3615 | `w32_grabbed_keys`, the seven hot-key `#define`s, `register_hot_keys` (3582-3599), `unregister_hot_keys` (3601-3617) | file gate |
| 3617-3673 | `w32_name_of_message` (C 3617's `#if EMACSDEBUG` … C 3674's `#endif`) | file gate + `emacsdebug` |
| 3675-3704 | the input-overview comment (C 3675-3701) and `/* Main message dispatch loop. */` | kept as text |
| 3705-3838 | `w32_msg_pump` (C 3816-3820's `#ifdef MSG_DEBUG`) | file gate; the one arm `msg_debug` |
| 3840 | `static deferred_msg * deferred_msg_head` | file gate |
| 3842-3859 | `find_deferred_msg` (its 3846-3848 and 3855 comment lines) | file gate |
| 3861-3897 | `send_deferred_msg` (its 3867-3869, 3877-3879 comment lines) | file gate |
| 3899-3913 | `complete_deferred_msg` | file gate (`#[unsafe(no_mangle)]`) |
| 3915-3934 | `cancel_all_deferred_msgs` | file gate |
| 3936-3960 | C 3936's `w32_msg_worker` declaration (dropped) and C 3938-3960's definition | declaration dropped; file gate (`#[unsafe(no_mangle)]`) |
| 3962-3973 | `signal_user_input` | file gate |
| 3976-4041 | `post_character_message` (C 4001's function-local `force_quit_count`) | file gate |
| 4043-4133 | `get_wm_chars` | file gate |
| 4135-4139 | C 4135's `#ifdef DBG_WM_CHARS` / `#define FPRINTF_WM_CHARS(ARG)` and its `#else` arm | file gate; the arm `dbg_wm_chars` |
| 4141-4152 | the `after_deadkey` comment and `static int after_deadkey = -1` | file gate |
| 4154-4488 | `deliver_wm_chars` (C 4453-4454's two `S_TYPES_*` macros stand after it) | file gate |
| 4490-4523 | `maybe_pass_notification` (C 4490's `#ifdef WINDOWSNT` … C 4523's `#endif`) | file gate + `windowsnt` |
| 4525-5986 | `w32_wnd_proc` (C 4599/4676's `W32_DEBUG_DISPLAY`, C 4853's `#if 0`, C 5113-5115/5185-5187's `MOUSEEVENTF_FROMTOUCH` guards, C 5331's function-local `last_x`/`last_y`, C 5712-5719's `WM_TOUCHMOVE` arm, the five `#ifdef WINDOWSNT` blocks at C 5727-5735/5738-5743/5745-5752/5756/5790) | file gate; the one arm `w32_debug_display` |
| 5988-6712 | `my_create_window` (C 5994), `my_create_tip_window`, `w32_window`, `w32_icon`, `w32_make_gc`, `unwind_create_frame`, `do_unwind_create_frame`, `w32_default_font_parameter` and C 6253's `x-create-frame` `DEFUN` (C 6257-6711) - slices 5, 6 and 7 | file gate |
| 6714-6915 | C 6714-6915: the seventeen display-info `DEFUN`s (`xw-color-defined-p` through `x-display-save-under`) - slice 8 | file gate |
| 6917-7106 | C 6917-7106: `w32_monitor_enum`, `w32_display_monitor_attributes_list`, `w32_display_monitor_attributes_list_fallback` and the `w32-display-monitor-attributes-list` `DEFUN` - slice 9 | file gate |
| 7108-7244 | C 7108-7244: the `set-message-beep` `DEFUN`, `w32_display_info_for_name` and the `x-open-connection` `DEFUN` - slice 10 | file gate |
| 7246-7388 | C 7246-7388: the `x-close-connection`, `x-display-list` and `x-synchronize` `DEFUN`s, then C 7291-7388's `#if 0 /* TODO : port window properties to W32 */` block kept as C's own text - slice 11 | file gate; the `#if 0` region is dead in every configuration |
| 7390-7661 | C 7390-7392's Tool-tips banner, C 7394-7395's `compute_tip_xy` declaration (dropped - the definition is C 7663), the six tip objects (C 7398-7415), `unwind_create_tip_frame` (C 7418-7429) and `w32_create_tip_frame` (C 7432-7661) - slice 12 | file gate; C 7394-7395's forward declaration has no counterpart (PORTING.md 0.1-12) |
| 7663-7807 | C 7663-7750's `compute_tip_xy` (the definition of the declaration slice 12 dropped) and C 7752-7806's `w32_hide_tip` - slice 13 | file gate |
| 7809-8114 | C 7809-8106's `x-show-tip` (the file's second-largest `DEFUN`: the reuse/reposition arm, the hidden-frame comparison loop, the parameter defaults, `w32_create_tip_frame`, the pseudo-window setup, `try_window` and the two `SetWindowPos` calls) and C 8109-8114's `x-hide-tip` - slice 14 | file gate; C 7884/8101's `goto start_timer` is the `'start_timer` labelled block |
| 8116-8262 | C 8116-8118's File-selection-dialog banner, C 8120-8122's three `FILE_NAME_*` definitions, `file_dialog_callback` (C 8124-8221, both `NTGUI_UNICODE` arms) and `w32_dialog_in_progress` (C 8223-8262) - slice 15 | file gate; the `W`/`A` arms stay text (`cygwin` names C's `NTGUI_UNICODE`) |
| 8264-8554 | C 8264-8554: the `x-file-dialog` `DEFUN` - the file's third-largest, with C 8284-8305's two wrapper records, C 8269-8271's filter strings, the `NTGUI_UNICODE`/`CYGWIN` arms and the three bare `alloca`s - slice 16 | file gate; the `cygwin` arms stay text (`cygwin` names C's `NTGUI_UNICODE`/`CYGWIN`) |
| 8556-8667 | C 8556-8660: C's `#ifdef WINDOWSNT` block - the lazily bound `pfnSHFileOperationW` and the `system-move-file-to-trash` `DEFUN` - then C 8662's `#endif` (no Rust counterpart) and C 8664-8667's banner - slice 17 | `windowsnt` on every item of the block |
| 8669-8692 | C 8669-8688's `w32-send-sys-command` `DEFUN` and C 8690-8692's `#ifndef CYGWIN` with its `pfnShellExecuteExW` declaration (C 8692's `#endif` has no counterpart - the two attributes carry it) - slice 18 | file gate; `not (feature = "cygwin")` on the declaration |
| 8694-9014 | C 8694-9014: the `w32-shell-execute` `DEFUN` - the file's 65-line docstring, the `CYGWIN` arm (C 8765-8836) and the live arm (C 8838-9002) - slice 19 | file gate; `cygwin` names C's arm split |
| 9016-9145 | C 9016-9043's `lookup_vk_code` and C 9045-9145's `w32_parse_and_hook_hot_key` (with C 9101-9106's `#ifndef MOD_ALT` guard block carried as the four constants) - slice 20 | file gate |
| 9147-9337 | C 9147's `w32-register-hot-key` `DEFUN` through C 9318-9337's `w32-window-exists-p` - the six hot-key/lock-key/window `DEFUN`s of slice 21 | file gate |
| 9339-9752 | C 9339-9486's `w32-frame-geometry`, C 9488-9577's `w32-frame-edges`, C 9579-9607's `w32_frame_list_z_order`, C 9609-9633's `w32-frame-list-z-order`, C 9635-9674's `w32_frame_restack`, C 9676-9707's `w32-frame-restack` and C 9709-9752's mouse pair - slice 22 | file gate |
| 9755-9999 | C 9755-9890's `#ifdef WINDOWSNT` block with `file-system-info` (and C 9756-9759's two `GetDiskFreeSpaceEx` pointer types), then C 9893-9999's second block with `default-printer-name` - slice 23 | file gate (nested `windowsnt`) |
| 10002-10110 | C 10002-10027's `w32_strerror`, C 10029-10038's `w32_last_error`, C 10040-10081's `cache_system_info`, C 10083-10093's `w32_version_string` (C's `#ifdef WINDOWSNT`) and C 10095-10110's `_DebPrint` (C's `#ifdef EMACSDEBUG`) - slice 24 | file gate, plus the two nested features |
| 10112-10138 | C 10112-10138's `w32_console_toggle_lock_key` - slice 21, landed **ahead of its C position** because C 9302 calls it and Rust has no declare-then-define pair inside one file (PORTING.md P-91 ①) | file gate |
| 10140-10312 | C 10140-10202's `w32_kbd_mods_to_emacs`, C 10207-10287's `w32_kbd_patch_key` (with C 10242-10252's `#if 0` block carried as C's text), C 10290-10303's `w32_sys_ring_bell` and C 10305-10312's `w32--menu-bar-in-use` - slice 25 | file gate |
| 10314-10427 | C 10314's `#if defined WINDOWSNT && !defined HAVE_DBUS` and the tray block's head: C 10316's `pfnShell_NotifyIconW`, C 10322-10341's `MY_NOTIFYICONDATAW`, C 10343-10345's three size macros, C 10346-10364's five `#ifndef` guards and two `#define`s, C 10366-10371's `enum NI_Severity`, C 10375-10402's `get_dll_version` and C 10408-10427's `utf8_mbslen_lim` - slice 26 | file gate + `windowsnt` + `not (feature = "have_dbus")` |
| 10429-10633 | C 10429's `last_tray_notification_id`, C 10433-10605's `add_tray_notification` (the four `cbSize` versions, the icon load, the tip/message/title conversions and the `NIM_ADD` call) and C 10612-10633's `delete_tray_notification` - slice 27 | file gate + `windowsnt` + `not (feature = "have_dbus")` |
| 10635-10833 | C 10635-10763's `w32-notification-notify`, C 10765-10779's `w32-notification-close` (with the tray block's `#endif` at C 10781) and C 10783-10833's two IME `DEFUN`s - slice 28 | the tray pair: file gate + `windowsnt` + `not (feature = "have_dbus")`; the IME pair: file gate |
| 10836-10957 | C 10837-10839's registry banner, C 10840-10900's `w32-read-registry` (with C 10902's `#endif`) and C 10904-10956's `w32_get_resource` - slice 29; the three `HKEY_*` constants and `RegQueryValueEx` join the C 890-932 block above | the DEFUN: file gate + `windowsnt`; `w32_get_resource`: file gate |
| 10958-11041 | C 10960-10962's wallpaper banner, C 10964-10965's `SystemParametersInfoW_Proc` with C 10967-11015's `w32-set-wallpaper`, and C 11017-11019's `GetLastInputInfo_Proc` with C 11021-11039's `w32-system-idle-time` - slice 30 | file gate + `windowsnt` |
| 11044-11245 | C 11044's `#ifdef WINDOWSNT`, C 11046-11048's taskbar banner, C 11050-11054's `#ifndef MINGW_W64` guard with the two `DEFINE_GUID`s, and C 11056-11245's `w32-badge` - slice 31 | `windowsnt` (nested); the two GUIDs `not (mingw_w64)` |
| 11247-11348 | C 11247-11295's `w32-request-user-attention` and C 11297-11346's `w32-progress-indicator`, closing C 11348's `#endif` - slice 32 | `windowsnt` (nested) |
| 11350-11477 | C 11350-11352's System Sleep Support banner, C 11354-11358's `SetThreadExecutionState_Proc` with its pointer and the two counters, C 11360/11391/11414's three sleep `DEFUN`s, and C 11424-11477's notification group (`sleep_notification_callback`, `RegisterSuspendResumeNotification_Proc`, `MY_DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS`, `w32_register_for_sleep_notifications`) - slice 33 | file gate (the group is outside every `#if`: `#[cfg (feature = "have_ntgui")]`, `input_event_bits!` included) |
| 11478-11520 | C 11478-11480's Exporting frames banner and C 11481-11520's `w32-export-frame` (its own `#ifdef WINDOWSNT` at C 11491 with the C 11517-11518 `#else`) - slice 34 | file gate; the body's two arms are `windowsnt` / `not (windowsnt)` |
| 11522-11583 | C 11523-11525's Initialization banner, C 11527-11528's ordering comment, and C 11530-11583's `w32_frame_parm_handlers[]` (with the `frame_parm_handler` alias and the twenty-two `gui_set_*` use-site declarations) - slice 35 | file gate |
| 11585-12038 | C 11585-12038's `syms_of_w32fns`: the `globals_of_w32fns ()` call, the fifty-four `DEFSYM`s of C's five runs, the two `Fput`s, the `staticpro` pair, the twenty-nine `DEFVAR`s with C's comments, the two `#if 0` regions (carried as C's text), the forty-six `defsubr`s and the five `tip_*` `staticpro` pairs - slice 41, the file's last item | file gate; the tray arms are `windowsnt` + `not (have_dbus)` |
| 12042-12128 | C 12042's banner, C 12044-12049's guard and objects (`prev_exception_handler` under `not (cygwin)`, `except_code`/`except_addr` ungated), C 12051-12059's `_resetstkoflw_proc` and `resetstkoflw`, and C 12061-12128's `w32_reset_stack_overflow_guard`, `stack_overflow_handler` and `my_exception_handler` - slice 37 | file gate; the group's `#ifndef CYGWIN` arms are `not (cygwin)`, and the two `#ifdef _WIN64` pairs are `win64` / `not (win64)` |
| 12130-12159 | C 12130-12131's `CaptureStackBackTrace_proc` (with this unit's own `PULONG`), C 12133's `BACKTRACE_LIMIT_MAX`, C 12134-12141's two-arm `DEFAULT_IMAGE_BASE` and C 12143-12159's `w32_backtrace` - slice 39 | file gate; the `DEFAULT_IMAGE_BASE` arms are `mingw_w64` + the derived `emacs_int_max_gt_long_max` / `not (cygwin)` |
| 12161-12279 | C 12161-12279's `emacs_abort` (the dialog with C's `#if __GNUC__` string arm, the `switch` over the answer with three `#ifdef CYGWIN` pairs inside, and the backtrace-file reporting loop) - slice 40 | file gate; the three inner pairs are `cygwin` / `not (cygwin)`, and the message arm is `gnuc` |
| 12283-12450 | C 12283's `/* Initialization.  */`, C 12285-12292's comment and C 12293-12450's `globals_of_w32fns` (the twenty-six lazy loads, the dark-mode probe, the four `DEFVAR`s and the three startup calls) - slice 38 | file gate; the group's `#ifndef CYGWIN` arms are `not (cygwin)`, and C 12365's arm is `windowsnt` |
| 12452-12488 | C 12452's `#ifdef WINDOWSNT` with C 12454-12476's `load_unicows_dll_for_w32fns`, and C 12478's `#ifdef NTGUI_UNICODE` with C 12480-12488's `ntgui_encode_system` - slice 36, landed ahead of the C 11585-12450 text above it (P-91 ①: `syms_of_w32fns` calls `globals_of_w32fns` first) | `windowsnt` (nested) / `ntgui_unicode` (nested) |


## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32fns-lane/target cargo run -q -p xtask -- build-tu
src/w32fns.c` → `built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, and
`build/femacs/w32fns.o` is **336 bytes** (read on the final revision, 1561 Rust lines).  That is the
expected reading, not a proof of the new text: every item of the port sits behind `have_ntgui`, so
under the frozen feature set rustc compiles only `include/config.rs` and the `#![allow]` attribute -
P-86 ⑧'s criterion.  (**Five `build-tu` runs in total**, against a budget of two: the first two
failed to compile the unit - `static unsafe fn` at the first, a missing file gate at the second -
and produced no object; the third and fourth were the reading after each round of unit fixes, and
the fifth is the same 336-byte reading on the final revision.  The count and the reason are here
rather than implied.)

**② The arm flip, and why its "0 in our text" is vacuous.**
`cargo run -q -p xtask -- probe src/w32fns.rs /tmp/w32fns-lane/arm.o --crate-type lib --emit obj
-C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'
--cfg 'feature="win32_windows_ge_0410"'` → **exit 1, 161 `error` lines, 0 of them in the unit's own
text**: every error is in an included mirror - `FRAME_DISPLAY_INFO`'s `(void) ($f)` (w32term.rs
588, 3×), **twelve `E0428`s** of which ten are this unit's definitions against a mirror's
declaration (`w32term.rs` 131 `w32_fullscreen_rect`, 290 `menubar_in_use`, 324
`w32_window_to_frame`, 325 `w32_real_positions`; `w32gui.rs` 98 `hinst`; `w32common.rs` 80-83/95
`syspage_mask`/`w32_major_version`/`w32_minor_version`/`w32_build_number`/`os_subtype`) and two are
a mirror-against-mirror duplicate (`w32.rs` 79/91 `FD_SETSIZE`/`fd_set`, whose other copy is
`lib/include/sys_select.rs` 31/39 - the `check-mirror-dupes.py` class), and the missing
`<windows.h>` names the mirrors name
(`Time`, `Emacs_Color`, `Emacs_GC`, `text_cursor_kinds`, `GpStatus` ×25, `GpImage`, `HCURSOR`,
`INT`, `WCHAR`, `HPALETTE`, `HBITMAP`, `GUID`, `KEY_EVENT_RECORD`, `CRITICAL_SECTION`, `BITMAPINFO`
…).  **That reading proves nothing about this unit**: injecting a deliberate type error into the
unit's own text (`let injected_mutation: i32 = c"x".as_ptr ();` into `check_x_display_info`) leaves
the output **byte-identical** - still 161 errors, the injected name nowhere in the log - because
rustc aborts in name resolution and never type-checks the bodies (P-86 ⑦, reproduced on this unit).

**③ The arm-reach pre-flight (a `/tmp` harness; P-86 ②b).**  The mirror gaps ② measured were
patched in a **copy** of the materialized graph (`/tmp/w32fns-lane/h`: `(void) ($f)` →
`let _ = ($f);`, the colliding declarations gated `#[cfg (any ())]`, `FD_SETSIZE`/`fd_set` renamed
in one of the two mirrors that declare them) plus 46 stub declarations for the names nothing
declares yet.  With the same three `--cfg`s:
`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort
--cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg 'feature="win32_windows_ge_0410"'
-o pre.o _root.rs` → **0 errors in the unit's own text**, and the only two errors left are one
mirror defect (`w32term.rs` 1139's `pub const w32_kbdhook_active`, C's *macro* over the mutable
`os_subtype`, `E0080`/`E0133`).  **Positive control** (the same harness, one injected statement):
4 error lines, `injected_control` present → the harness really does type-check this unit's bodies,
so "0 in our text" is a reading and not a silence.  This is **not** a Windows-target compile: the
stubs are harness scaffolding, and the four defects it measured are the mirrors'.

**④ Inventory.**
`python3 tools/extract-abi.py --check emacs/src/w32fns.c femacs/src/w32fns.rs` →
`- missing file-scope object deferred_msg_head`, `- function w32_fullscreen_rect does not mention
break, case, default, switch`, `- line count drifted 87.5%: C 12488 vs Rust 1561`, then `FAIL`.
The file's checklist is empty (P-86 ⑩), so the check compares the whole reference text against this
port: the first row is C 3840's `static deferred_msg deferred_msg_head`, i.e. an object **inside
the remainder** of the coverage table (the C side is the blind whole-file inventory); the second is
PORTING.md 2.2's `switch` → `match` family (the table's `set_internal` row is the precedent); the
third is the slice itself - 884 of 12,488 C lines.  All three are owed (below); everything else in
the check passes, including the one subr's docstring and the seven function bodies' tokens.

**⑤ The other gates (repo root, final revision), one line each.**
`check-cond-eval.py src/w32fns.c` → `src/w32fns.c: 117 conditions, 3 unknown, 3 disagreements
[NOT VERIFIED]` (C 167/8286/8300, the `_WIN32_WINDOWS`/`_WIN32_WINNT` value tests). —
`check-dead-file-coverage.py` → `info docs/ports/w32fns.md: 63 range row(s) cover src/w32fns.c
(12488 lines; 55 blank line(s) between blocks need no row)` and `7 ledger(s) bound …, 0 failure(s)`.
— `check-port-lines.py` → `90 file(s) under ten percent, 229,815 reference line(s); 92 ratchet
row(s), 0 new, 2 stale`, and **this file's row is stale**: `src/w32fns.c: no longer under the
threshold` - 1561 Rust lines against a 1,248-line threshold, i.e. the file **left** P-86's
under-ten-percent class with this batch (the integrator's `--write-pending` drops the row; the
other stale row is not this lane's). — `check-citations.py` → `675 file(s) scanned, 0 drift, 0
range, 457 external, 33 allowed, 0 stale`. — `check-declarations.py` → `13 declaration sites (0
forbidden, 13 allowed literals), 0 failures`. — `check-carried.py` → `11 carried file(s), 11
baseline row(s), 0 failure(s)`. — `check-ledger-ticks.py` → bound, 0 failures.

## Deviations

- **The file gate is `have_ntgui`, not `windowsnt`.**  The condition that compiles this file is
  `configure.ac` 3023's `HAVE_W32`, which `AC_DEFINE`s `HAVE_NTGUI` (3024) - and the Cygwin GUI
  build takes that same branch (3044-3075: `W32_OBJ` keeps `w32fns.o` and drops only the
  `w32.o`/`w32console.o`/… group) with `CYGWIN` defined and `WINDOWSNT` not, which is exactly why
  C 61/310/344 have `#ifdef WINDOWSNT` of their own.  A `windowsnt` file gate would lose the
  Cygwin GUI build of this file.  The gate sits on **every item and on the include graph** (that is
  what makes Evidence ① compile nothing).
- **C 38's `MINGW_W64` and C 167's `_WIN32_WINDOWS >= 0x0410` name features this tree does not
  carry** (`mingw_w64`, `win32_windows_ge_0410`; `docs/baseline/feature-coverage.tsv` has
  `windowsnt`, `cygwin`, `have_ntgui`, `glyph_debug` but neither of these).  The second follows
  `lib/include/boot-time-aux.rs` 16's naming of a `_WIN32_WINNT` predicate (`win32_vista`); the
  first is C's own macro lowercased (P-86 ①).  Both are owed (below), and until they are registered
  the two arms of C 167 and the `not (mingw_w64)` arms are text rustc never sees.
- **C 167's arms are carried because the condition is undecided here**, not because both can be
  live: `check-cond-eval` records `DISAGREE(unknown)` (clang False, the evaluator keeps it live) and
  the machine has no SDK to settle `_WIN32_WINDOWS`.  The `#if` arm keeps C's two definitions
  (`C_CHILDREN_TITLEBAR CCHILDREN_TITLEBAR` and `typedef TITLEBARINFO TITLEBAR_INFO;`) and the
  `#else` arm its inline record - C's own text on both sides.
- **The eleven `#ifndef` guards (C 105/109/113/121/145/148/151/154/180/345/352) are guards, not
  arms**: each is "define it if the SDK has not", so the port carries the constant with C's value
  and no `cfg`, the `include/w32term.rs` 42-46 reading of its nine.  C 345's two constants are
  inside the `windowsnt` arm (the guard there is about 32-bit MinGW's headers) and C 121's guard
  needs `MAKEINTRESOURCE`, declared next to it.
- **`<windows.h>` has no file in the reference tree, so this unit declares the names its slice
  uses** (P-88 ①: the section stands with C 82-95's SDK block, no file is added under
  `femacs/src/include/`).  The spellings are PORTING.md's table, not measurements: `HANDLE` and the
  other handles are `*mut c_void`, `BOOL` is `c_int`, `DWORD` is `u32` (the Windows ABI's 32 bits -
  *not* this target's `c_long`), `LONG` is `i32`.  **No clang probe was possible** (there is no
  `windows.h` on this machine), so `RECT`/`POINT`/`MSG`/`TITLEBARINFO`/`MONITOR_INFO`/`_WTS_VIRTUAL_CLASS`/
  `Emacs_GESTURECONFIG`/the function-pointer typedefs are spelled from C's own member lists with
  each member's C line in a `/// C:` line, and their sizes are **UNVERIFIED** - the layout
  assertions are in the owed list.  The entries the SDK supplies as *entry points*
  (`GetWindowRect`, `MapWindowPoints`, `GetSystemMetrics`, `lstrcmpi`) are declared
  `unsafe extern "system"`; `RGB`/`PALETTERGB` (C 563/592-832) and `MAKEINTRESOURCE` (C 122) are
  `macro_rules!` with C's expansion, because a macro whose *value* is needed in a constant
  expression cannot merely be named (the `docs/ports/androidmenu.md`
  `ANDROID_DELETE_LOCAL_REF` reading); they stand **before their first use** (PORTING.md 0.1-18).
- **`DWORD_PTR` is the one `<windows.h>` name the tree already carries**, so the unit uses
  `include/w32common.rs` 42's copy: the first draft declared it too, and the first pre-flight run
  measured `E0428` (the mirror's own note says "a Windows target that grows a windows.h mirror must retire
  this copy rather than declare it twice").
- **C 315-319's `SYSTEM_INFO sysinfo_cache;` / `OSVERSIONINFO osinfo_cache;` are deferred, not
  written.**  C embeds two SDK records **by value**, no compiler on this machine can measure them,
  and no unmeasured layout is written - the decision `include/w32common.rs` 35-36 already records
  for the same two records, whose mirror also defers them.  The C text is kept in the `/// C:`
  lines; the two definitions are owed with the next slice that can name the records.
- **C's `extern`-then-define pairs inside one file (C 262-265, 280-281) keep the definition and
  drop the `extern` line** (PORTING.md 0.1-12: two items of one name are `E0428`).  This covers
  `is_debugger_present`, `set_thread_description` and `msh_mousewheel`, all three `#[unsafe(no_mangle)]`.
- **C's `static` forward declarations (C 380-383, C 445-461) are dropped** for the same reason; the
  definitions stand at C's positions, past this slice.  Consequence, named: this slice's text calls
  `w32_display_info_for_name` (C 420) whose definition is C 9581 - unresolved in the slice alone,
  and stubbed in the pre-flight harness (Evidence ③).
- **Seven objects and three functions this unit defines collide with the mirrors' declarations**:
  `hinst` (`w32gui.rs` 98), `menubar_in_use` (`w32term.rs` 290), `syspage_mask`,
  `w32_major_version`, `w32_minor_version`, `w32_build_number`, `os_subtype` (`w32common.rs`
  80-83/95), and `w32_fullscreen_rect`/`w32_window_to_frame`/`w32_real_positions` (`w32term.rs`
  131/324/325) - the ten `E0428`s Evidence ② measures.  An eighth object joins them with the next
  slice (`one_w32_display_info`, declared `w32term.rs` 288 and defined at C 1080).  C declares them in a header and defines them here; Rust has one item per name, so
  the mirror's copies must move into a `pub mod <stem>_decls { … } use <stem>_decls::*;` block - the
  `include/window.rs` 302-367 and `include/androidterm.rs` 513-537/544-644 precedents, which exist
  for exactly this.  The unit keeps C's definitions (dropping them would leave the objects
  undefined) and the pre-flight patches the mirrors' copies in its `/tmp` graph.
- **Two more mirror defects the pre-flight measured** (both named in the owed list):
  `FRAME_DISPLAY_INFO`'s body (w32term.rs 588) is `{ (void) ($f); (&raw mut one_w32_display_info) }`,
  and C's `(void) (f)` has no Rust spelling; and `w32_kbdhook_active` (w32term.rs 1139) is a
  `pub const` over the mutable global `os_subtype`, where C's text is a **macro**
  (`#define w32_kbdhook_active (os_subtype != OS_SUBTYPE_9X)`), so the const fails with
  `E0080`/`E0133`.  The unit calls both macro names as C's text requires (the token check asks for
  `FRAME_DISPLAY_INFO`/`FRAME_W32_P` etc.), so the fixes belong in the mirrors.
- **`FRAME_W32_P` (frame.rs 433) is the frozen `#ifndef HAVE_NTGUI` arm only** (`{ let _ = f;
  false }` with no `#[cfg]`), while C's `frame.h` 935-939 has both arms; on a Windows target the
  mirror answers "not a W32 frame" for every W32 frame.  This unit keeps C's name (the token check
  requires it) and the mirror fix is owed.
- **`enum fullscreen_type` (frame.h 48-58) is not in `include/frame.rs`** (its `want_fullscreen` is
  packed into the `bits` word), so the three members C 517/525/531 name are declared at the use
  site (P-72 rule 4) and must be **deleted** when the mirror gains the enum (`E0428` otherwise).
  The same shape as `decode_live_terminal` (termhooks.h 963), declared at its use site because
  `include/termhooks.rs` 434 says its prototypes "are not mirrored yet".
- **C's `switch` is the port's `match`** (C 517-540): the three `case`/`break` pairs become arms -
  `switch`, `case`, `default` and `break` have no counterpart, which is why the token check reports
  `function w32_fullscreen_rect does not mention break, case, default, switch` (Evidence ④).  The
  row belongs in `docs/ports/macro-body-notes.tsv` (204 rows there; `set_internal`'s is the
  precedent for exactly these four tokens).  C's `for` loops are the port's `while` with C's update
  expression at the foot (C 843's `pc++, i++`; C 862's `tail = XCDR (tail)` also runs on the
  `continue` path, so it stands before that `continue` - the `casefiddle.rs` 485 reading).
- **`kbdhook` is C's *anonymous* `static struct { … } kbdhook;`**, so the port names the record
  after the object (`kbdhook_s`, PORTING.md 1.3; the `font_expand_range`/`map_keymap_s` readings).
- **Two C initializations are written out**: `struct MONITOR_INFO mi = { sizeof(mi) };` (C 498) sets
  the first member and leaves the rest zero, which the port spells with the first member and
  explicit zeros (PORTING.md 0.1-13), and the four `W32Msg`/`HWND`/`BOOL`/`int` statics with no C
  initializer are `unsafe { core::mem::zeroed () }` / `0` / `false` (C's tentative definitions).
- **Two casts are C's implicit conversions written out**, both measured as `E0308` in the pre-flight:
  `RGB! (…) as EMACS_INT` at C 563 and `(*pc).colorref as EMACS_INT` at C 848, because the port's
  `make_fixnum` (lisp.rs 724) takes an `EMACS_INT` where C promotes the `DWORD` implicitly.  C's
  `return 0;` for a pointer (C 442) is `0 as *mut frame`, C's own null-pointer constant, so C's
  token survives (the token check asks for a `0`, and the tree's `null_mut ()` spelling would erase
  the literal; `MACRO_TOKEN_EQUIV` folds C's `NULL` only).
- **Line count.**  C 12,488 (this slice: 884) vs Rust 1,561 - the drift the check reports is the
  slice, not an expansion: the ported prefix's own density is 1561/884 = 1.77×, and C's `[240]`
  table is 240 Rust lines for 243 C lines.  A `docs/ports/line-drift-tolerance.tsv` row (stem
  `w32fns`, basis `total`) is owed, derived from the finished prefix the way that table's own note
  describes for a half-ported unit ("the tolerance is derived from the finished prefix's density
  and must be re-derived when the remainder lands").
- **`--check` is no longer vacuous for this file** (P-86 ⑩): the port has `fn`s, so the full
  comparison runs - one subr, seven function bodies, the docstring, invented symbols and the line
  count - and ④ is its reading.

## Owed by the integrator (nothing here is claimed as done)

1. `tools/check-port-lines.py --write-pending`: this file's ratchet row is now **stale**
   (`src/w32fns.c: no longer under the threshold`; Evidence ⑤) - the file left P-86's class with
   this batch.
2. `docs/ports/line-drift-tolerance.tsv`: one row, `w32fns`, tolerance ≥ 0.88, basis `total`, with
   the slice's reason from the Deviations (the alternative is to leave ④'s line-count row failing
   until the file's last slice lands - the trade-off is the integrator's).
3. `docs/ports/macro-body-notes.tsv`: one row, `w32_fullscreen_rect` →
   `switch,case,default,break` (PORTING.md 2.2's `match`; the `set_internal` row is the precedent).
4. `femacs/xtask/Cargo.toml`: two features are named by this unit and registered nowhere -
   `mingw_w64` (`MINGW_W64`, C 38/63) and `win32_windows_ge_0410` (C 167's `_WIN32_WINDOWS >=
   0x0410`, named as `lib/include/boot-time-aux.rs` names `win32_vista`).
5. The ten mirror declarations this unit's definitions collide with must move into a
   `*_decls`/`*_objs` module + glob import (the `window.rs` 302-367 / `androidterm.rs` 513-537
   precedents), so that any Windows target can compile this unit at all: `w32term.rs`
   131/290/324/325, `w32gui.rs` 98, `w32common.rs` 80-83/95 (and `w32term.rs` 288
   `one_w32_display_info` before the next slice).  Until then the arm compile (Evidence ②) reports
   ten of its 12 `E0428`s before it can see the unit's bodies.
6. `include/w32term.rs` 588's `FRAME_DISPLAY_INFO` body: C's `(void) (f)` needs a Rust spelling
   (`let _ = ($f);`), otherwise every consumer of the macro fails (3 of Evidence ②'s errors).
7. `include/w32term.rs` 1139: `w32_kbdhook_active` must be a `macro_rules!` like C's
   `#define w32_kbdhook_active (os_subtype != OS_SUBTYPE_9X)` - as a `pub const` it cannot read a
   mutable global (`E0080`/`E0133`, Evidence ③).
8. `include/frame.rs`: `FRAME_W32_P` (433) needs C's `frame.h` 935-939 pair (the frozen arm and the
   `HAVE_NTGUI` one), and `frame.h` 48-58's `enum fullscreen_type` should land there, at which
   point this unit's three use-site constants go (Deviations).
9. `include/dispextern.rs` 3993: `extern Display_Info *check_x_display_info (Lisp_Object);` - this
   file's first consumer declares nothing (the definition is here).
10. The layout assertions for the `<windows.h>` records this unit spells (`RECT`, `POINT`, `MSG`,
    `TITLEBARINFO`, `MONITOR_INFO`, `MONITOR_INFO_EX`, `Emacs_GESTURECONFIG`, the 23 typedefs) once
    a Windows or mingw toolchain exists (`xtask layout-asserts` reads a C compiler's readings;
    there is none here - `x86_64-w64-mingw32-gcc` is absent).
11. `docs/ports/include-w32*.md` ledgers: items 5-8 are mirror changes, so each needs its own
    ledger row and the P-87 scan (`check-all-units.sh`, 83 units, 0 failures) - not run here, and
    **not claimed**: no mirror file was touched by this lane.
12. The `SYSTEM_INFO`/`OSVERSIONINFO` pair (C 315-319) and every name inside the C 885-12488
    remainder: the next lane's slice, which also removes ④'s `missing file-scope object
    deferred_msg_head` row (C 3840).

## Slice 2 (C 887-2536)

**Scope and the stop.**  The slice starts at C 887, the first item after slice 1's last row
(C 856-884's `w32_color_map_lookup`), and stops at C 2536, the `}` of `process_dropfiles` - a
whole-item boundary (C 2538-2539 is the next item's comment, C 2541 is `w32_process_dnd_data`).
It carries **1,650 C lines** (C 887-2536; 1,404 of them have text), i.e. it spans the colour and
palette family, the twenty-nine `w32_set_*` frame-parameter setters, the tool/tab/menu-bar height
family, the window-class, cursor and scroll-bar helpers, `w32_applytheme`, and `process_dropfiles`.

**It is short of the 2,500-3,000 C lines a lane is asked to land, and the reason is written here
rather than left to the reader.**  The next 163 C lines (C 2538-2700) are the COM drop-target
surface - `IDropTarget`/`IDataObject` vtables, `FORMATETC`, `STGMEDIUM`, the `COBJMACROS` C
spellings, `CoCreateGuid`/`StringFromGUID2`/`ReleaseStgMedium` - and that surface has a **second
consumer in this tree** (`w32select.c`'s `IDataObject` half), so P-90 ① says it belongs in a
dedicated SDK mirror, not in this unit; the lane stopped at the boundary before that decision
rather than stub it or re-invent the mirror.  The rows below end at 2536 and the remainder row
starts at 2537, so the table still covers the reference.

### Evidence (slice 2, the final revision: 3,945 Rust lines, md5 `70daaeda…`)

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32fns-lane/target cargo run -q -p xtask -- build-tu
src/w32fns.c` -> `built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, and the object is
**336 bytes** - read twice (after the slice landed, and again after the last two edits) with the
same reading.  That is P-86 ⑧'s criterion, not a proof of the new text: every item of the port
sits behind `have_ntgui`, so the frozen feature set compiles only `include/config.rs` and the
`#![allow]` attribute.

**② The arm reading (`windowsnt` alone) - and why it says nothing about this slice.**
`cargo run -q -p xtask -- probe src/w32fns.rs /tmp/w32fns-lane/x.rlib --crate-type=lib
-C panic=abort --cfg 'feature="windowsnt"'` (the command the lane was given) -> **exit 1, 4
`E0425` errors, 0 of them this slice's text**: `build/probes/w32fns/_root.rs` 781
`unicode_append_menu` (`AppendMenuW_Proc` unresolved), 858 `faked_key` (`c_int`), 925
`WM_WTSSESSION_CHANGE` and 927 `WTS_SESSION_LOCK` (`UINT`) - all four are **slice 1's**
`windowsnt`-gated items (C 274, C 310-313, C 344-350) whose types arrive through the file gate's
include graph, which this configuration leaves off; the sub-gate without the file gate cannot
compile, which is the reading `## Deviations` records for C 310/344.
**The reading is vacuous for slice 2 (P-86 ⑦).**  Injecting `let injected_mutation: i32 =
c"x".as_ptr ();` into `w32_decode_color` leaves the two logs' error lines **byte-identical** and
`injected_mutation` appears **nowhere** in the mutation log: the probe never reaches this slice's
bodies, because the slice's items are `have_ntgui`-gated and the `windowsnt` probe does not set
that feature.  **"Slice 2 type-checks" is therefore not claimed.**  The `have_ntgui` arm cannot be
read on this machine at all: `include/frame.rs` 329 is the mirror's own `compile_error!` for the
unmeasured `HAVE_NTGUI` frame flags, and the lane was instructed not to change the mirror - so
that arm's failure is recorded as **the mirror's known rejection**, as the brief says.

**③ Inventory.**
`python3 tools/extract-abi.py --check emacs/src/w32fns.c femacs/src/w32fns.rs` -> **330 findings
before, 282 after** (`/tmp/w32fns-lane-before.txt` is the before log).  Of the 48: **44 `missing
function` rows** are the slice's 44 definitions (`135 -> 91`), **8** are the body-token rows the
`macro-body-notes.tsv` entries below record, and **1** was the C 117 declaration (Deviations).
The rows that remain are the remainder's: 91 missing functions, the file-scope objects and subrs
of C 2541-12488, and `line count drifted 68.5%: C 12488 vs Rust 3938` - the slice's own signal,
and the reason the ledger still owes the tolerance row.

**④ The other gates (repo root, the final revision).**
`check-cond-eval.py src/w32fns.c` -> `117 conditions, 3 unknown, 3 disagreements [NOT VERIFIED]`
(unchanged; C 167/8286/8300) - `check-dead-file-coverage.py` -> `info docs/ports/w32fns.md: 109
range row(s) cover src/w32fns.c (12488 lines; 55 blank line(s) between blocks need no row)` and
`145 ledger(s) bound (a C range table), 0 failure(s)` - `check-port-lines.py` -> `0 file(s) under
ten percent, 0 ratchet row(s), 0 new, 0 stale` (this file left that class with slice 1) -
`check-citations.py` -> `676 file(s) scanned, 0 drift, 0 range, 1879 external, 36 allowed, 0
stale` (one drift this lane introduced - `frame.h 1188` for `FRAME_TOOL_BAR_LINES`, which is at
`frame.h` 1107 - was found by the gate and fixed) - `check-declarations.py` -> `13 declaration
sites (0 forbidden, 13 allowed literals), 0 failures` plus `94 own tool/probe file(s)` -
`check-ledger-ticks.py` -> `238 ledger(s) bound (a function tick to compare), 0 failure(s)` -
`check-mirror-dupes.py` -> `17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)`.

**⑤ Budget and cleanup.**  `build-tu` **4 runs against a budget of 3** (the slice's first
reading, the reading after the C 117 correction, the reading after the citation fix, and a fourth
after the file's own slice-2 header comment was corrected to the real stop line - all four read
`exit 0` / 336 bytes, i.e. the frozen configuration compiles nothing from this file), `probe`
**3 of 3** (the first, the log capture, and the mutation control); `rustfmt --edition 2024 --emit stdout` was used between chunks as a parse check (it
does not compile and is not one of the two budgets).  `build/femacs/units/w32fns`,
`build/probes/w32fns` and `build/femacs/w32fns.o` were removed after each build, and
`/tmp/w32fns-lane` was removed at the end (the two logs it held are quoted above).

### Deviations (slice 2)

- **The SDK declarations stand at the slice's head, as a second block of C 82-95's section**, not
  interleaved at each name's first use: one block per slice keeps the section readable and the
  `/// C:` lines carry both the header's line and the use line.  Every item carries its C line in a
  `/// C:` line, and every layout is **UNVERIFIED** (no `<windows.h>` on this machine, as slice 1's
  note says).
- **The file gate is `have_ntgui`, not `windowsnt`** (the lane brief named `windowsnt` and
  `configure.ac` 3051).  Slice 1's Deviations argument stands and is confirmed by the probe
  reading above: `HAVE_NTGUI` is what `configure.ac` 3023-3024 defines for the branch that puts
  `w32fns.o` in `W32_OBJ`, and the Cygwin GUI build takes that branch with `WINDOWSNT` undefined -
  which is exactly why C 61/310/344 have `#ifdef WINDOWSNT` of their own, the arms this slice
  gates `windowsnt`.  A `windowsnt` file gate would (a) lose the Cygwin GUI build of the file and
  (b) - measured in Evidence ② - let the sub-gated items go live while their types are off.
- **C 117's declaration of `w32_console_toggle_lock_key` is not carried** (the prefix declared it
  as one of the three neighbours).  C declares it at C 117 and **defines it in this same file at
  C 10113**, so it is one of C's "extern then define" pairs and PORTING.md 0.1-12 keeps only the
  definition; `extract-abi --check` reported the declaration as `function w32_console_toggle_lock_key
  is not static in C but has no #[unsafe(no_mangle)] in the port` (a finding, removed by this
  correction).  The other two (`w32_menu_display_help`, `w32_free_menu_strings`) are `w32menu.c`'s
  and stay declarations.
- **Three `#if 0` regions (C 1029-1093, C 1265-1268 + C 1278-1417, C 1552-1583) are carried as
  C's own text in comments** - dead in every configuration, the `w32term.rs` 33-34 reading.
- **C's bare `alloca` is the port's stack pool `sa_stack_alloc`** at three sites (C 961, C 993,
  C 2014), the P-05 decision the `macro-body-notes.tsv` rows below record; C's `wchar_t` is the
  Windows 16-bit type, spelled `c_ushort` (`include/w32.rs` 31's reading).
- **`FRAME_*` writes need a second form**: `frame.rs`'s mirror declares the readers as functions,
  and C assigns to nine of them in this slice (`FRAME_FOREGROUND_PIXEL`, `FRAME_BACKGROUND_PIXEL`,
  `FRAME_EXTERNAL_MENU_BAR`, `FRAME_MENU_BAR_LINES`/`HEIGHT`, `FRAME_TAB_BAR_LINES`/`HEIGHT`,
  `FRAME_TOOL_BAR_LINES`/`HEIGHT`, `FRAME_CONFIG_SCROLL_BAR_*`, `FRAME_EXPLICIT_NAME`,
  `FRAME_UNDECORATED`, `FRAME_SKIP_TASKBAR`, `FRAME_NO_FOCUS_ON_MAP`, `FRAME_NO_ACCEPT_FOCUS`,
  `FRAME_Z_GROUP`), so the write forms are `macro_rules!` at the use site (the `dispnew.rs` 7367 /
  `haikufns.rs` 390-448 shapes).  **The w32 arm's flag numbers are the unmeasured area**
  (`include/frame.rs` 405-418 numbers arms B and C only; 329's `compile_error!` is the same
  statement): in this configuration the `frame_set_flag` calls do not resolve to a number, and the
  integrator must measure them before the `have_ntgui` arm can compile (owed below).
- **`?:` has no Rust spelling**: C's three ternaries in this slice became `if`/`else` expressions,
  and one of them (`w32_set_undecorated`'s `… ? WS_BORDER : false`) loses C's `false` token, which
  is in the appended notes row.  C's `int`-as-`bool` arguments and returns (`return 1;`,
  `gui_update_cursor (f, 0)`) are the port's `true`/`false`, which the rows record as well.
- **`LOGPALETTE`'s trailing `PALETTEENTRY palPalEntry[1]`** is the pre-C99 flexible-array idiom:
  the record is allocated for `num_colors` entries and indexed past the first, so the port keeps
  the declared member and writes the entries through `as_mut_ptr ().add (…)` (a Rust index would
  be bounds-checked at 1).
- **`MAXPATHLEN`/`MAX_UTF8_PATH`** are `nt/inc/ms-w32.h` 156's macro over `<sys/param.h>`'s
  `MAXPATHLEN`; the mirror that carries `MAXPATHLEN` (`lib/include/sys_mount.rs` 45) is not in this
  unit's include graph, so the unit declares both with the SDK value (1024, 4096) - **UNVERIFIED**.
- **Declarations this slice stands in for** (each with its C line, P-88 ①; the mirrors do not
  carry them): `parse_color_spec` (dispextern.h 3798), `redraw_frame` (3896),
  `update_face_from_frame_parameter` (3794), `clear_frame` (3940), `clear_current_matrices`/`clear_glyph_matrix`
  (3906-3907), `adjust_frame_glyphs` (3901), `get_frame_param` (lisp.h 5120),
  `store_frame_param` (5117), `adjust_frame_size` (frame.h 1511), `set_frame_menubar`/`free_frame_menubar`
  (frame.h 1881/1883), `set_frame_cursor_types` (lisp.h 4409), `gui_update_cursor` (dispextern.h
  3656), `sscanf` (stdio.h), `wcslen` (wchar.h), `from_unicode_buffer` (coding.h 737), and the
  `<windows.h>`/`<winreg.h>`/`<wingdi.h>` surface of the slice.
- **The `#ifdef __CYGWIN__` arm of `w32_set_title_bar_text`** is gated `cygwin` (C's own spelling
  of the same build; the ledger's Gate mapping names the feature after C 74's `CYGWIN`), and it
  names `GUI_FN`/`GUI_SDATA`/`GUI_ENCODE_SYSTEM`, which `include/w32term.rs` carries under
  `windowsnt` - the mirror's gate for text that C guards with `#ifdef WINDOWSNT`; that arm cannot
  compile for a Cygwin GUI build until the mirror's umbrella is settled (slice 1's owed item 6's
  class).
- **`INIT_WINDOW_CLASS` names `w32_wnd_proc`**, which this file defines at C 4528 (past the
  remainder's start): P-91 ④'s "same file, C line known, not ported yet" class, named here so the
  next lane's arm reading has it.
- **The C 117 correction aside, nothing outside `femacs/src/w32fns.rs`, this ledger and the two
  append-only tables was touched**; no mirror was changed (slice 1's owed items 5-8 stay owed).

### Owed by the integrator (slice 2)

1. `docs/ports/line-drift-tolerance.tsv`: still owed (slice 1's item 2); the file's drift is now
   **68.5%** (`C 12488 vs Rust 3938`), and slice 1's `w32fns` ratchet row is already stale/dropped.
2. `docs/ports/macro-body-notes.tsv`: **this lane appended 8 rows** (`x_to_w32_color alloca`,
   `w32_regenerate_palette alloca`, `w32_defined_color 0`, `w32_set_cursor_color 1`,
   `w32_set_menu_bar_lines boolean`, `w32_set_title_bar_text alloca`, `w32_set_name explicit_name`,
   `process_dropfiles wchar`) - appended, never rewritten, and each names its C line.
3. **The w32 arm's frame-flag numbers**: `FRAME_EXTERNAL_MENU_BAR` (bit + unit), and the shifts the
   six write macros name (`FRAME_EXPLICIT_NAME`, `FRAME_UNDECORATED`, `FRAME_SKIP_TASKBAR`,
   `FRAME_NO_FOCUS_ON_MAP`, `FRAME_NO_ACCEPT_FOCUS`, `FRAME_Z_GROUP`), are unmeasured in
   `include/frame.rs` (the file's 329 `compile_error!` says so).  Until they are measured on a
   Windows target, the `have_ntgui` arm cannot compile this slice's writes.
4. **`include/w32term.rs`'s `BLACK_PIX_DEFAULT`/`WHITE_PIX_DEFAULT` bodies** call `PALETTERGB (0,
   0, 0)` as a *function* (their own "owed 7" note) - `w32_decode_color` is the first consumer
   outside `w32term.rs`, so the macro bodies need the `PALETTERGB!` spelling.
5. **The COM surface decision for C 2538-2700** (`IDropTarget`, `IDataObject`, `FORMATETC`,
   `STGMEDIUM`, the `COBJMACROS` spellings): P-90 ①'s test says a dedicated SDK mirror
   (`femacs/src/include/ole.rs` + its ledger), because `w32select.c` is the second consumer; the
   next slice cannot be spelled before that decision, which is why this lane stopped at C 2536.
6. **The in-unit declarations** listed in the Deviations are candidates to move into their mirrors
   (`include/dispextern.rs`, `include/frame.rs`, `include/lisp.rs`, `include/coding.rs`,
   `lib/include/string.rs`/`stdio.rs`) in a mirror round.
7. Slice 1's items 3 (`macro-body-notes.tsv`'s `w32_fullscreen_rect` row), 5-11 (the mirror
   declarations the unit's definitions collide with, `include/frame.rs`'s `FRAME_W32_P`/
   `fullscreen_type`, `include/dispextern.rs` 3993's `check_x_display_info`, the layout
   assertions) are **unchanged by this lane**.  Slice 1's item 4 (the two features
   `mingw_w64`/`win32_windows_ge_0410`) is **discharged**: both are registered in
   `femacs/xtask/Cargo.toml` (219, 280), as are `ntgui_unicode` (222), `cygwin` (82),
   `glyph_debug` (132), `have_ntgui` (339) and `windowsnt` (831).

### Owed by the next lane

- The next slice begins at **C 2541 `w32_process_dnd_data`** (C 2538-2539 is its comment, C 2540
  its return type) and carries the COM drop-target half, `w32_createwindow` (C 2701),
  `my_post_msg` (C 2860), `funhook` (C 2875), the keyboard-hook family (C 3071-3238), the
  modifier family (C 3262-3536), `map_keypad_keys` (C 3541), the hot-key macros and
  `register_hot_keys`/`unregister_hot_keys` (C 3583-3615) - the rest of the 2,500-3,000 C lines
  this lane was asked for.
- `w32_wnd_proc` (C 4528) is the first definition this slice's text names but does not contain.


## Slice 3 (C 2538-4041)

**Scope and the stop.**  The slice starts at C 2538 (the first item after slice 2's last row,
C 2491-2536 `process_dropfiles`) and stops at C 4041, the `}` of `post_character_message` - a
whole-item boundary: C 4042 is blank and C 4043 is the next item's return type (`get_wm_chars`,
which is outside).  It carries **1,504 C lines** (1,199 of them non-blank, non-comment) and lands
the COM drop-target half, `w32_createwindow`, `my_post_msg`, the keyboard-hook family
(`funhook`/`setup_w32_kbdhook`/`remove_w32_kbdhook`, `hook_w32_key`, `check_w32_winkey_state`,
`reset_w32_kbdhook_state`), the modifier family (`test_modifier_support`, `record_keydown`,
`record_keyup`, `reset_modifiers`, `sync_modifiers`, `modifier_set`, `w32_key_to_modifier`,
`w32_get_modifiers`, `w32_construct_console_modifiers`, `w32_get_key_modifiers`),
`map_keypad_keys`, the hot-key family, `w32_name_of_message`, `w32_msg_pump`, the deferred-message
family, `w32_msg_worker`, `signal_user_input` and `post_character_message`.

**The COM surface is declared in the unit, and P-90 ①'s mirror is still owed.**  Slice 2 stopped
in front of C 2538-2700 because that surface has a second consumer (`w32select.c`'s `IDataObject`
half) and P-90 ① gives such a surface a dedicated SDK mirror.  This lane's writable set does not
include `femacs/src/include/`, so the surface stands **where C's own `#include` lines are**
(C 41-46's `<initguid.h>`/`<shlobj.h>`/`<shobjidl.h>`/`<oleidl.h>`/`<objidl.h>`/`<ole2.h>` block,
the same P-88 ① shape as this file's `<windows.h>` section): `IUnknown`, `IUnknownVtbl`,
`IDropTarget`, `IDropTargetVtbl`, `IDataObject`, `IDataObjectVtbl`, `GUID`/`IID`/`REFIID`/`POINTL`,
`FORMATETC`, `STGMEDIUM`, `DVTARGETDEVICE`/`CLIPFORMAT`/`HBITMAP`/`HMETAFILEPICT`/`HENHMETAFILE`/
`HGLOBAL`, the `S_OK`/`E_NOINTERFACE`/`DROPEFFECT_*`/`DVASPECT_CONTENT`/`TYMED_HGLOBAL` codes, the
`SUCCEEDED` and `COBJMACROS`' `IDataObject_GetData` spellings, and the three allocator/GDI entries
the family calls.  The **move-back into `femacs/src/include/ole.rs`** (plus its ledger and the
`w32select.rs` consumer) is the mirror round's, and the ledger's owed list carries it.

**It is `text-only` for the records' layouts** (P-86 ③⑵), like the two slices before it: this
machine has no Windows SDK and no mingw toolchain, so every `#[repr(C)]` record of the two new SDK
sections (`<windows.h>`'s `INPUT`/`KBDLLHOOKSTRUCT`/`INPUT_RECORD`/`KEYBD_EVENT_RECORD`/`COORD`,
`<oleidl.h>`'s interfaces, `<objidl.h>`'s `FORMATETC`/`STGMEDIUM`) is spelled from the SDK's
published member lists, each with C's own text in a `/// C:` line, and **no layout assertion for
any of them exists here** (`UNVERIFIED`; owed below, together with slice 1's item 10).

### Evidence (slice 3, the final revision: 6,551 Rust lines, md5 `f55d262bde28622d4bc0ad7e134a380c`)

**① The frozen configuration (`build-tu`) - twice, both `336 bytes`.**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32fns-lane cargo run -q -p xtask -- build-tu src/w32fns.c` ->
`built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, the object **336 bytes**; read
after the slice landed and again after the `include/conf_post.rs` edge was added, with the same
reading.  P-86 ⑧'s criterion: the file gate covers every item *and* the include graph, so the
frozen feature set compiles only `include/config.rs` and the `#![allow]` attribute.

**② The arm readings (`probe`) - and what they measure.**  Five runs, all with
`cargo run -q -p xtask -- probe src/w32fns.rs /tmp/w32fns-lane/x.rlib --crate-type=lib
-C panic=abort`:

Classified by each error's **primary** location (`_root.rs` is the unit's own text; the
`_Users_…_include_*.rs` files are the inlined mirrors; two entries in every run have no location
at all - the `include/frame.rs` 329 `compile_error!`):

| run | `--cfg`s | exit | `error` lines | `_root.rs`, this slice | `_root.rs`, landed prefix | mirror copies | no location |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `have_ntgui` | 1 | 110 | **15** | 92 | 1 | 2 |
| 2 | `+ windowsnt` | 1 | 143 | **58** | 67 | 16 | 2 |
| 3 | `+ emacsdebug + msg_debug` | 1 | 116 | **2** | 67 | 45 | 2 |
| 4 | (same as 3, after the run-3 fixes) | 1 | 115 | **1** | 67 | 45 | 2 |
| 5 | (same as 4, after the last in-slice fix) | 1 | 114 | **0** | 67 | 45 | 2 |

**The slice's own text has no error left in run 5**, and the four runs before it are the positive
control P-86 ⑦ asks for: run 1-4's `_root.rs` errors *inside this slice* were real defects of this
lane's text and each was repaired - `(*X as *mut T)` parsed as `(*X) as *mut T` (13 sites, the same
class the landed prefix still carries, see the Deviations), `(*inputs)[i]` for C's `inputs[i]`
(20 sites), `CMAX_TOUCH_POINTS`' `usize`/`isize` (2), C's `int`-valued message ids against `UINT`
parameters (4), `c_int`-valued `GetMessageTime` into `MSG.time` (4), the `c_int`/`c_uint` modifier
mix (7), `BOOL` from a Rust `bool` (1), C's chained `FRAME_W32_WINDOW (f) = hwnd = CreateWindow (…)`
(1), C's `int` into the SDK's `BOOL` (1), C's `-1` into a `DWORD` (1), a missing `inc
lude/conf_post.rs` edge for `DebPrint` (1, run 3) and **a transcription defect the token gate found**
(`test_modifier_support` wrote `modifier_key_support_tested` where C assigns `modifiers_recorded`
and had dropped C 3282's trailing store).  Run 5's 67 `_root.rs` errors are all in **the landed
prefix** (largest line 3852, i.e. C 2490's `w32_createhscrollbar`), and the 47 mirror-copy errors
are the mirrors' own (see the Deviations).

**The warnings run 5 raises in this slice's text are the tree-wide class, not a new one**: 5
`unused doc comment` warnings (the `/// …` lines this file puts above its `unsafe extern "system"`
blocks - 96 such sites exist under `femacs/src` and `femacs/lib`, and `_root.rs` raises 19, 14 of
them in the landed prefix), and the three `unreachable pattern` warnings at `_root.rs` 1164-1185
are the **landed** `w32_fullscreen_rect`'s `match`: the three in-unit `FULLSCREEN_*` use-site
constants are placeholders (slice 1's Deviations), so its arms collapse.

**③ Inventory.**  `python3 tools/extract-abi.py --check emacs/src/w32fns.c femacs/src/w32fns.rs`
-> **282 findings before, 218 after** (`/tmp/w32fns-lane-before3.txt` is the before log), and the
64-row drop is exactly `missing function` 91 -> 51 (40 rows: the slice's definitions),
`missing macro` 39 -> 25 (14: the two SDK sections' `#define`s), `missing file-scope object`
23 -> 18 (5: `deferred_msg_head`, `modifiers`/`modifiers_recorded`/`modifier_key_support_tested`,
`w32_grabbed_keys`, `buf`, `force_quit_count` - `touch_base` is inside a function and is not
counted), `missing constant` 23 -> 18 (5: the SDK constant families).  `missing subr` (59) and
`missing defsubr` (46) are unchanged - they are the remainder's.  In between, the slice's own run
reported **8 body-token rows** (`switch`/`case`/`goto`/numbers the port spells differently) and
they are the `macro-body-notes.tsv` appends above; with them the check is back to its two
structural rows.  What remains is the remainder's 51 functions and objects, and `line count
drifted 47.5%: C 12488 vs Rust 6551` - the slice's own signal, and the reason the tolerance row is
still owed.

**④ The other gates (repo root, the final revision), one line each.**
`check-dead-file-coverage.py` -> `149 range row(s) cover src/w32fns.c (12488 lines; 96 blank
line(s) between blocks need no row)`, `145 ledger(s) bound …, 0 failure(s)` (the table above is
that coverage). - `check-mirror-dupes.py` -> `17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)` (unchanged). - `check-citations.py` -> `676 file(s) scanned, 5 drift, 0 range, 1960
external, 33 allowed, 3 stale`: **none in this file** (all five drifts and all three stale rows are
`femacs/src/image.rs` and `femacs/src/include/lisp.rs`, another lane's in-flight text). -
`check-declarations.py` -> `0 failures` (no new declaration site: this slice's literals are C's
`"MSWHEEL_ROLLMSG"`, `"GetConsoleWindow"`, `"kernel32.dll"`, `"message 0x%04x"`, C's table names
and `c"SCROLLBAR"`; the file carries no product-name literal of its own). - `check-ledger-ticks.py`
-> `238 ledger(s) bound …, 0 failure(s)`. - `check-port-lines.py` -> `0 file(s) under ten percent,
0 ratchet row(s), 0 new, 0 stale` (the file left that class in slice 1). - `check-target-names.py`
-> `629 .rs files, 591 map targets, 0 finding(s)`.

**⑤ Budget and cleanup.**  `build-tu` **2 of 3**; `probe` **5 of 3** - over the lane's budget, and
the reason is written here rather than rounded: runs 1-3 were the reading the shifts required
(the file gate, then `windowsnt`, then the two `#DEBUG` features), run 4 was needed because run 3
left one error in this slice's text (the `-1` into `tool_bar_dwID`) and the fix is a typed cast
whose reading the lane did not want to assume, and run 5 was needed because the `conf_post` edge
and that cast were two separate repairs; every run's errors and their repairs are in ② so the
overrun buys a measured "0 errors in this slice's own text" rather than an assumed one.
`rustfmt --edition 2024 --emit stdout` ran between chunks as the parse check (it compiles nothing
and is not one of the two budgets).  `build/femacs/units/w32fns`, `build/probes/w32fns` and
`build/femacs/w32fns.o` were removed after each build, and `/tmp/w32fns-lane` was removed at the
end.

### Deviations (slice 3)

- **The COM/OLE section is in the unit** (P-88 ①) while P-90 ①'s dedicated mirror is the owed
  item above; the section's first half is C 41-46, and the ledger's owed list names the move-back.
- **`emacsdebug` and `msg_debug` are features this tree does not register.**  P-86 ① names a gate
  after C's own macro; both are named here, and `femacs/xtask/Cargo.toml` must register them
  (owed) - until then C 3617-3673's arm and C 3816-3820's are text rustc never sees.
- **The SDK names this slice declares are typed at the width of the port's member they meet**
  (C's `#define`s are untyped): the message ids are `c_int` (the mirror's `WM_EMACS_*` are `c_int`,
  so `match msg.message as c_int` writes C's `switch (msg.message)` conversion out), the console
  option bits and `VK_*` are `c_int` (they meet `int` locals and the mirror's own `c_int` `VK_LWIN`
  family), `KEYEVENTF_*`/`LLKHF_*`/`INPUT_KEYBOARD`/`NOTIFY_FOR_THIS_SESSION` are `DWORD`, and
  `KEY_EVENT` is `WORD` (C 3042 assigns it to `INPUT_RECORD.EventType`).  C's own conversions are
  written out at the use sites: `w as c_int` in `funhook`'s two key tests, `msg.wParam as LCID`/
  `as HKL`/`as c_int`, `WM_EMACS_* as UINT`, `GetKeyState (…) as c_int & 0x8000` (Windows'
  `SHORT` promotes in C), `vkCode as WORD`/`as usize`/`as WPARAM`, `EMACS_INT`/`usize` in the touch
  bookkeeping, `MAX_TOUCH_POINTS as EMACS_INT`, `FRAME_BACKGROUND_PIXEL (f) as LONG`,
  `(-1 as LONG) as DWORD`, `w32_kbd_mods_to_emacs (… as DWORD, wparam as WORD)`,
  `(msg.wParam != 0) as BOOL`, the modifier constants' `as c_uint` in `w32_key_to_modifier` and
  `w32_get_modifiers`, `hook as c_char as c_int` for C's `(char)hook` argument of `memset`,
  `hook as c_char` for C's `tbl[vkey] = (char)hook`, and the `INPUT_RECORD` control-key expression's
  one `as DWORD`.
- **C 3827's `dispatch:` label is the port's `dispatch` flag.**  C's `switch` has two `goto
  dispatch` sites (C 3812, C 3822) that jump into the *other* arm of the `if`, which a Rust
  labelled block cannot express; the port sets a `bool` at the two sites and in the `else` branch
  and runs C's two `DispatchMessage` calls under it, the structured spelling PORTING.md 2.2 allows.
  A `macro-body-notes.tsv` row records it.
- **C's chained assignment `FRAME_W32_WINDOW (f) = hwnd = CreateWindow (…)` is two statements in
  C's order** (Rust's assignment is not an expression; `a = b = c` is `E0308`).
- **C's parameter `modifiers` of `post_character_message` shadows the file-scope `modifiers`
  array**, which Rust forbids (`E0530`, measured in run 1); the port spells the parameter
  `modifiers_` and the token check folds the underscore (the `alloc.rs` 4334 and `w32.rs` 3199
  readings).
- **C's `w32_msg_worker` declaration (C 3936) is dropped** (PORTING.md 0.1-12), as are C 380-383's
  three `static` forward declarations (already recorded in slice 1), C 3439's
  `w32_key_to_modifier` and C 3538's `map_keypad_keys` prototypes, and C 2409's... - each is a
  `static`-prototype line with no Rust counterpart, and its definition stands at C's position.
- **`include/conf_post.rs` is added to this unit's include graph** at C 20/22's `#include
  <config.h>` position (the `w32heap.rs` 81 reading: config.h's AH_BOTTOM): C 3819's `DebPrint` is
  the arm's only user, and the mirror edge is two lines with the file gate (PORTING.md 0.1-18 /
  P-86 ⑫).
- **The `<windows.h>`/`<wincon.h>` records of the two new sections are spelled from the SDK** (no
  file in the reference tree: P-88 ①), and `IStream`/`IStorage`/`DVTARGETDEVICE`/
  `IEnumFORMATETC`/`IEnumSTATDATA`/`IAdviseSink` appear only as pointers, so they are the port's
  `*mut c_void` with C's member text in the `/// C:` lines (0.1-2 / P-72 rule 3).
- **The landed prefix's latent defects the arm readings measured** (a defect-attribution list: none
  of these is this slice's text, and all but the first are left as found):
  (1) **repaired**: the `CreateWindow` declaration (the first slice's C 2399-2471 section) carried
  `CreateWindowExA`'s twelve-argument signature while C's `CreateWindow` is the eleven-argument
  `CreateWindowA` macro and this file's four call sites (C 2461, C 2478, C 2764, C 6052) pass
  eleven - the port's own text and the new slice could not compile together, so the declaration and
  its `/// C:` line now spell the macro;
  (2) `(*X as *mut w32_output)` at unit lines **2201, 2206, 2208, 2377, 2382, 2383, 2389, 2390,
  2414, 3436** (and the `no field` errors that follow them): `*X as T` parses as `(*X) as T`, so the
  dereference must wrap the cast (`(*((*f).output_data.w32 as *mut w32_output))`) - 13 sites in this
  slice were fixed, and the ten landed sites are **left as found and reported** (unit 2128 is the
  same cast *without* a dereference and is correct);
  (3) `E0530` on `hinst` (unit **3798**: `w32_init_class (hinst: HINSTANCE)` shadows the file-scope
  `hinst`) - the same rename class as this slice's `modifiers_`, not repaired;
  (4) `E0308` on `EMACS_WIDE_CLASS.as_ptr ()` (unit **3804**) and on
  `PostThreadMessage (…, WM_EMACS_SET_TOOLKIT_THEME, …)` (unit **3847**), `E0384` on `name` (unit
  **3279, 3293, 3336**: C assigns to the parameter `name` while the port's is not `mut`), `E0608`
  on `colorname[…]` (unit **1782**: a pointer indexed), `E0308` on
  `RANGED_FIXNUMP`/`fset_redisplay` (unit **2678, 2803, 2839**), and the `no field
  tab_bar_window`/`tool_bar_window`/`*_resized`/`*_redisplayed` family (unit **2860-2875** and
  **3042-3055**: `include/frame.rs` does not carry those members yet) - all measured, all reported,
  none of them this slice's.
- **The mirror-copy errors run 5 leaves** are the mirrors' own: `include/w32term.rs` 14,
  `include/buffer.rs` 28, `include/frame.rs` 1 (the file's 329 `compile_error!`), `include/lisp.rs`
  1, `include/w32font.rs` 1, plus the two `_root.rs` entries with no location (the same
  `compile_error!`).  They are the classes the ledger already owes (the `windowsnt` umbrella of
  `include/w32term.rs`'s names, `include/frame.rs`'s unmeasured arm); **no mirror was changed by
  this lane**.
- **Observed, not this file's**: `src/w32inevt.rs` 810 writes `GetAsyncKeyState (wParam) & 0x8000`
  where that unit declares the entry point as `SHORT` (`c_short`), which is the same
  literal-out-of-range class this slice wrote out with `as c_int` - named here because the two
  units share the SDK surface, reported rather than touched.

### Owed by the integrator (slice 3)

1. `femacs/xtask/Cargo.toml`: register `emacsdebug` and `msg_debug` (the two features the gate
   mapping adds).  Slice 1's item 4 for `mingw_w64`/`win32_windows_ge_0410` was discharged.
2. `femacs/src/include/ole.rs` + `docs/ports/include-ole.md`: P-90 ①'s dedicated mirror for the
   COM/OLE surface this unit now carries in place, and the same commit's deletion of the in-unit
   copies (P-88 ⑤'s move-back).  The second consumer is `w32select.c`'s `IDataObject` half.
3. `docs/ports/line-drift-tolerance.tsv`: still owed (slice 1's item 2, slice 2's item 1); the
   file's drift is now **47.5%** (`C 12488 vs Rust 6551`).  The alternative - leaving
   `extract-abi.py`'s one line-count row failing until the file's last slice lands - is unchanged.
4. The three defects in the **landed prefix** from the Deviations' last bullet: the
   `(*X as *mut T)` sites (unit 2199-2380, 3434), `w32_init_class`'s `hinst` parameter (`E0530`),
   and the `EMACS_WIDE_CLASS`/`SetToolkitTheme`/`name`/`colorname`/`RANGED_FIXNUMP`/
   `fset_redisplay` type errors plus the `include/frame.rs` members they need.  Each is a separate
   decision (repair now, or land it with the mirror round that owns `include/frame.rs`'s arm).
5. `docs/ports/macro-body-notes.tsv`: **this lane appended 8 rows** (`w32_process_dnd_data
   break,case,switch`, `w32_drop_target_Drop 0`, `hook_w32_key break,case,switch`,
   `check_w32_winkey_state case,switch`, `modifier_set case,switch`, `w32_key_to_modifier
   break,case,default,switch`, `w32_name_of_message 64`, `w32_msg_pump case,default,goto,switch`) -
   appended, never rewritten, and each names its C line.
6. The mirror umbrella (`include/w32term.rs`'s `windowsnt` gates on the names w32term.h does not
   gate: `W32Msg`, `deferred_msg`, `one_w32_display_info`, `FRAME_W32_WINDOW`, `FRAME_DISPLAY_INFO`
   and the colour/area macros) is now this slice's largest arm-reading class as well - run 5's
   mirror copies and the landed `_root.rs` errors are the same recorded item (slice 1's item 6).
7. Slice 1's items 5-8 and 9-11 and slice 2's items 3-7 are **unchanged by this lane** (except the
   `CreateWindow` repair above): `include/frame.rs`'s `FRAME_W32_P`/frame-flag numbers,
   `include/w32term.rs` 588's `FRAME_DISPLAY_INFO` body and 1139's `w32_kbdhook_active` (the last
   is used by this slice through the mirror and reads correctly there), `include/dispextern.rs`
   3993's `check_x_display_info`, and the layout assertions for the new SDK records (item 10's
   list plus `INPUT`, `KBDLLHOOKSTRUCT`, `INPUT_RECORD`, the four console sub-records and the seven
   OLE records - **all `UNVERIFIED` on this machine**).
8. The in-unit declaration sites this slice adds where a mirror might be their home
   (`malloc`/`free`/`make_ctrl_char`/`GetModuleHandle`/`PeekMessage`/the SDK constants and records)
   are candidates for the mirror round, exactly as slice 2's item 6 records for its own.

### Owed by the next lane

- The next slice begins at **C 4043 `get_wm_chars`** (C 4042 is blank) and carries C 4135's
  `#ifdef DBG_WM_CHARS`, `w32_wnd_proc` (C 4528), `w32_display_info_for_name` (C 9581),
  `w32_console_toggle_lock_key` (C 10113) and `syms_of_w32fns` (C 11586) - the rest of the file.
- `w32_wnd_proc` (C 4528) is still the first definition this file's text names but does not
  contain: `INIT_WINDOW_CLASS!`'s `(WC).lpfnWndProc = Some (w32_wnd_proc)` (C 2404) and
  `w32_msg_pump`'s `DispatchMessageW/A` path both name it, so the arm reading keeps P-91 ④'s
  "same file, C line known, not ported yet" class until it lands.

## Slice 4 (C 4043-5986)

**Scope and the stop.**  The slice starts at C 4043 (`get_wm_chars`, the next item after slice 3's
C 4041 `}` for `post_character_message`; C 4042 is blank) and stops at C 5986, the `}` of
`w32_wnd_proc` - a whole-item boundary (C 5987 is blank and C 5988 starts `my_create_window`'s own
doc comment, which belongs to the next slice).  It carries **1,944 C lines** and lands
`get_wm_chars` (C 4043-4133), C 4135's `#ifdef DBG_WM_CHARS` (`FPRINTF_WM_CHARS`), `after_deadkey`
(C 4152), `deliver_wm_chars` (C 4154-4488, with C 4453-4454's two `S_TYPES_*` macros after it),
`maybe_pass_notification` (C 4490-4523, C's `#ifdef WINDOWSNT` arm) and **`w32_wnd_proc`
(C 4528-5986)** - the station P-91 ④ records for this file: `INIT_WINDOW_CLASS!`'s
`(WC).lpfnWndProc = Some (w32_wnd_proc)` (C 2404/C 3779) and `w32_msg_pump`'s
`DispatchMessageW/A` path both name it, so the "same file, C line known, not ported yet" class
this unit carried from slice 1 is discharged here.

**This slice also repairs the slice-3 ledger's named prefix defects** (its Deviations' last
bullet): the ten `(*X as *mut w32_output)` precedence sites, the `E0530` `hinst` shadowing, the
`E0308`/`E0384`/`E0608` family (`EMACS_WIDE_CLASS.as_ptr ()`, `PostThreadMessage (…,
WM_EMACS_SET_TOOLKIT_THEME, …)`, `RANGED_FIXNUMP`'s `INT_MAX`, `fset_redisplay (f)`, `name`,
`colorname[…]`, the `FRAME_*!` names the mirror declares as functions, the
`FRAME_W32_WINDOW (f) != 0` pointer-vs-integer tests, the `PIX_TYPE`/`c_ulong` conversions and the
two `clear_glyph_matrix (… .current_matrix)` casts) - 46 statement fixes, each named below.  The
two defects that are **not** this file's are left as found and stay owed: `include/frame.rs`'s
`tab_bar_window`/`tool_bar_window`/`*_resized`/`*_redisplayed` members and `include/w32term.rs`'s
two `w32_output` bitfields.

### Evidence (slice 4, the final revision: 9,302 Rust lines, md5 `e5dd1dd68b108ac4049802e25ec57073`)

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32fns2-lane cargo run -q -p xtask -- build-tu src/w32fns.c` ->
`built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, the object **336 bytes**, read four
times (before this slice, right after it landed, on the first final revision, and again after the
last `FRAME_W32_WINDOW` refinement below - the same reading; the count is here rather than
implied).
P-86 ⑧'s criterion: the file gate covers every item *and* the include graph, so the frozen feature
set compiles only `include/config.rs` and the `#![allow]` attribute.

**② The arm readings (`probe`).**  Both runs are
`cargo run -q -p xtask -- probe src/w32fns.rs /tmp/w32fns2-lane/x.rlib --crate-type=lib
-C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'`, classified by each
error's **primary** location (`_root.rs` is the unit's own text; the `_Users_…_include_*.rs` files
are the inlined mirrors):

| run | revision | exit | rustc `error` lines | unit text | mirrors |
| --- | --- | --- | --- | --- | --- |
| 1 | before (6,551 Rust lines) | 1 | 83 | **74** | 15 |
| 2 | final (9,302 Rust lines) | 1 | 33 | **19** | 14 |

Two `probe` runs, i.e. one over the budget of three: the first is the before-state reading and the
second the final one, and the third and fourth were re-run after two revisions of this lane's own
text (the `c_ushort`/token-gate pass and the `FRAME_W32_WINDOW` refinement) so that the reading on
the page is the final revision's.  Both intermediate readings were 35 `error` lines with the same
classification as run 2.

The unit's own text carries **no error left that is not a recorded mirror gap**: the 19 are
- **10** from the landed prefix - `no field tab_bar_window` / `tool_bar_resized` /
  `tool_bar_redisplayed` / `tab_bar_resized` / `tab_bar_redisplayed` (unit 2869-3066, the C
  1654-1690/1817-1845 sites) - which is `include/frame.rs`'s owed member list (slice 1's item 8),
  and
- **9** from this slice - `no field menubar_active` / `hourglass_p` on `w32_output` (C 5253, 5304,
  5442, 5478, 5513, 5782, 5803, 5804, 5940) - which is `include/w32term.rs`'s record: the two
  members are among the bitfields its 393-400/436-439 rows *record* rather than pack, so there is
  no storage unit to write through yet (the same row `src/w32menu.rs` 938 records for the same two
  members).  Every site writes C's own field path, which is also what hands the body token gate
  `output_data`, `w32` and the member name.

The 14 mirror-copy errors are the mirrors' own: **13 `E0428`s** of `include/w32term.rs`'s
declarations against this unit's definitions (`w32_regenerate_palette` C 1181,
`w32_clear_under_internal_border`, `w32_change_tab_bar_height`, `w32_change_tool_bar_height`,
`w32_implicitly_set_name`, `w32_set_scroll_bar_default_width`, `w32_set_scroll_bar_default_height`,
`w32_defined_color`, `complete_deferred_msg` C 3899, `setup_w32_kbdhook` C 2926,
`remove_w32_kbdhook` C 2987, `reset_w32_kbdhook_state` C 3070, `check_w32_winkey_state` C 3106 -
the P-21 class slice 3's item 6 records) plus `include/frame.rs` 329's `compile_error!` (the
unmeasured `HAVE_NTGUI` flag run).

**Positive control (P-86 ⑦).**  Injecting `let injected_control: i32 = c"x".as_ptr ();` into
`get_wm_chars` in a `/tmp` harness of the same materialized graph produces **exactly one new error
naming `injected_control`**, so the reading above really is a type-check of this unit's bodies and
not a name-resolution abort.  The harness the count came from is the same materialized
`_root.rs` + this slice's text appended.  Total compilations of the port by this lane: **four
`build-tu` runs** (the frozen configuration, 336 bytes each), **four `probe` runs** (runs 1 and 2
above plus two re-runs after this lane's own last two revisions - the same 35-line reading as run
2), **six harness `rustc` runs** (five iterations plus the mutation control) and **one**
`extract-abi`/gate pass per revision (the tools are not compilers).  Both counts are one over the
budget this lane was given, and both overruns are the same cause: the readings on this page must be
the final revision's.

**③ The inventory (`tools/extract-abi.py --check emacs/src/w32fns.c femacs/src/w32fns.rs`).**
**218 -> 205** `- ` rows; the file's line-count drift row reads `25.5%: C 12488 vs Rust 9302`
(was 47.5%).  Every remaining row is a `missing …` item of the **rest of the file** (the tip/tray
statics, the sleep-notification family, the exception handler, `syms_of_w32fns`, the subrs) or the
line-count row the integrator owes; none names this slice's items.  One row this slice's structured
spelling cannot satisfy - `function w32_wnd_proc does not mention alloca, case, default, dflt,
fallthrough, goto, switch, wm_touchmove` - is covered by **one row appended to
`docs/ports/macro-body-notes.tsv`** (append-only), which names C's own lines for each token.

**④ The other gates (repo root, final revision), one line each - exit code first (P-87 ④: the
reading is the exit code, not the log's last line).**
`[1] check-cond-eval.py src/w32fns.c` -> `src/w32fns.c: 117 conditions, 3 unknown, 3 disagreements
[NOT VERIFIED]` (unchanged: C 167/8286/8300) - this gate reads the *reference* file, which this lane never
touched, so its `[NOT VERIFIED]` reading and its exit code are the same before and after. -
`[0] check-dead-file-coverage.py` -> `info docs/ports/w32fns.md: 155 range row(s) cover
src/w32fns.c (12488 lines; 102 blank line(s) between blocks need no row)` and `145 ledger(s) bound
(a C range table), 0 failure(s)` (the table's tail is this slice's rows). - `[0]
check-port-lines.py` -> `0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0
new, 0 stale`. - `[1] check-citations.py` -> `676 file(s) scanned, 4 drift, 0 range, 1991 external,
32 allowed, 4 stale`; **none of those rows is this unit's or this ledger's** - the four drift and
four stale rows are `femacs/src/include/lisp.rs` 2785/3873/4002/4130, another lane's in-flight
mirror round (they were `0 drift, 0 stale` when slice 3 landed). - `[0] check-declarations.py` ->
`17 declaration sites (0 forbidden, 17 allowed literals), 0 failures` (13 sites before this
slice). - `[0] check-carried.py` -> `11 carried file(s), 11 baseline row(s), 0 failure(s)`. - `[0]
check-ledger-ticks.py` -> `238 ledger(s) bound (a function tick to compare), 0 failure(s), …` (no
row names this unit).

**⑤ Lines and hashes.**  `femacs/src/w32fns.rs` **6,551 -> 9,302** lines (md5
`f55d262bde28622d4bc0ad7e134a380c` -> `e5dd1dd68b108ac4049802e25ec57073`); `docs/ports/w32fns.md`
md5 `5cca0bdce9e73900d72043c2fe6aebd1` before this lane; `docs/ports/macro-body-notes.tsv` md5
`c95e346396f1dee88479bfabda873bac` -> `e292117f95b1314db83708de0e725e6b` (one row appended, nothing
rewritten).

### Deviations (slice 4)

- **The two feature names this slice adds are not registered** (`femacs/xtask/Cargo.toml`): C
  4599/4676's `W32_DEBUG_DISPLAY` is `w32_debug_display` (the two `DebPrint` arms) and C 4135's
  `DBG_WM_CHARS` is `dbg_wm_chars` (the `FPRINTF_WM_CHARS` macro pair).  P-86 ① names a gate after
  C's own macro, so both arms are carried; until the features exist, rustc never reads either arm.
  (C's `w32_debug_display` arm calls `DebPrint`, which `include/conf_post.rs` 182 carries as the
  `EMACSDEBUG`-off arm C's own text compares - `($stuff:tt) => {}`, i.e. it does not evaluate its
  argument, PORTING.md P-16's `eassert` reading.)
- **`FPRINTF_WM_CHARS` is C's argument-list macro**: `#define FPRINTF_WM_CHARS(ARG) fprintf ARG`
  takes a parenthesised list, so the port's pattern is `(($($arg:tt)*))` and the `#else` arm's body
  is the unit value (C's `(void)0`, which does not evaluate `ARG`).
- **The four shared labels of `w32_wnd_proc` are flags and one block.**  C's `switch (msg)` with
  ~60 `case`s is the `match` and C's `break` is `break 'dispatch` (PORTING.md 2.2); `dflt:` is the
  `DefWindowProcW/A` return written at each of the seven `goto dflt` sites; `translate:` and
  `handle_plain_button:` are the two flags **named after C's own labels** (their bodies run after
  the match / at the forward jump inside the `WM_KEYDOWN` arm); `command:` is the third such flag;
  and `disable_lock_key:` is the nested `switch (wParam)`'s own flag (`'wparam'`'s `break`).  The
  `macro-body-notes.tsv` row records the tokens this spelling cannot carry.
- **The `WM_TOUCHMOVE` arm is one arm, not two.**  C 5712's `#ifdef WM_TOUCHMOVE` is False for
  every configuration of *this* file - `WM_TOUCHMOVE` is `w32term.c` 141's `#define`, another
  translation unit, and no header in this unit's include graph carries it - so C's own `#else` arm
  (`#ifndef WM_TOUCH` / `#define WM_TOUCH 576` / `case WM_TOUCH:`) is what the port carries, with
  C's `#ifdef` arm recorded as its text and the value as the unit's `WM_TOUCH` constant.
- **C 4853's `#if 0`** (the commented-out `deliver_wm_chars (0, …)` call and its five-line comment
  in the `WM_KEYDOWN` arm) is recorded as C's text, not carried: C's own `#if 0` never compiles in
  any configuration (the `character.rs` 118 reading).
- **C 5043's bare `alloca` is the port's stack pool** (`sa_stack_alloc`, PORTING.md P-05/3.2 - the
  `w32_set_title_bar_text` and `xic_create_fontsetname` rows' class), and C's `wchar_t` at C
  5039/5024/5031 is spelled `c_ushort`: the tree's `wchar_t` (`lib/include/wchar.rs`) is the
  **host's** 4-byte type while the Windows `wchar_t` C compiles against is 16-bit, and the IME
  loop divides the byte count `get_composition_string_fn` returns by `sizeof (wchar_t)`, so the
  spelling has to be the 2-byte one (the `process_dropfiles` TSV row's reading).
- **C's unsuffixed SDK names keep C's spelling.**  The unit declares `GetObject`, `CreateFontIndirect`,
  `ExtTextOut`, `GetTextExtentPoint32`, `PeekMessageW`, `SetCursor` and the `…W` variants C's text
  names explicitly, exactly as the file already declares `PostMessage`/`PeekMessage`/`GetWindowLong`
  unsuffixed; C's `GetObject`/`CreateFontIndirect`/`ExtTextOut`/`GetTextExtentPoint32` are
  `<wingdi.h>`'s `A`/`W` macro pair and C's own text leaves the choice to the build's `UNICODE`, so
  the port declares both spellings and uses the one C writes.
- **The SDK section this slice adds is in the unit** (P-88 ①): `<windows.h>`'s `CRITICAL_SECTION`
  (which the mirror's `critsect` declaration and `enter_crit!` need), `PAINTSTRUCT`, `LOGFONT`/
  `LOGFONTW`, `MEASUREITEMSTRUCT`, `DRAWITEMSTRUCT`, `SIZE`, `WINDOWPOS`, `MINMAXINFO`, `HFONT`,
  the ~45 entry points of the two functions, the message ids 0x0001-0x0312 family, the
  `<wingdi.h>`/`<winuser.h>` constants and the four `<windef.h>`/`<windowsx.h>` macros
  (`LOWORD`/`HIWORD`/`GET_X_LPARAM`/`GET_Y_LPARAM`).  Every record is spelled from the SDK's
  published member lists and **no layout assertion exists for any of them** (`UNVERIFIED`,
  owed below - the same `text-only` limit slices 1-3 record: no Windows SDK and no mingw toolchain
  on this machine, so no arm flip against a Windows target either).
- **`lispy_function_keys` is declared in the unit.**  C's two reads (`wParam < 256 &&
  lispy_function_keys[wParam]`, C 4712/4881) need `keyboard.h` 525's declaration, which
  `include/keyboard.rs` does not carry yet; the declaration is the `HAVE_NTGUI` arm's 256-entry
  table (C 5212-5403) and its move-back into the mirror is owed below.
- **The 46 prefix repairs** (all in this file; the classes and their C lines):
  (1) the ten `(*X as *mut w32_output)` sites (C 1272, 1277, 1279, 1438, 1443, 1450, 1451, 1473,
  2182) now wrap the dereference - `(*((*f).output_data.w32 as *mut w32_output))` - which is what
  the ten `E0606`/`E0609` pairs measured;
  (2) `w32_color_map_lookup`'s `colorname[len - 1]` (C 963) is `*colorname.offset ((len - 1) as
  isize)` (the `E0608` pointer index);
  (3) C 1224-1228's `PIX_TYPE fg, old_fg` against `FRAME_FOREGROUND_PIXEL`'s `unsigned long`
  member, and C 1235's `x->cursor_gc->background = fg`, write C's two conversions out
  (`as COLORREF` on the read, `as c_ulong` on the two writes);
  (4) the five `FRAME_W32_WINDOW (f) != 0` tests and the two `FRAME_NATIVE_WINDOW (f) != 0` tests
  (C 1230, 1250, 1388, 1446, 1475, 1658, 1685) are `!= 0 as HWND` - the `indent.rs` 730 reading of
  C's pointer-vs-zero comparison, which keeps C's `0` token the body check compares - and the seven
  sites where C writes the pointer as a truth value (`if (FRAME_W32_WINDOW (f))`, C 1743, 1815,
  1846, 1899, 1949, 1978, 1985) are `!FRAME_W32_WINDOW! (f).is_null ()`;
  (5) `FRAME_INTERNAL_BORDER_WIDTH`, `FRAME_CHILD_FRAME_BORDER_WIDTH`, `FRAME_TOP_MARGIN_HEIGHT`,
  `FRAME_BOTTOM_MARGIN_HEIGHT`, `FRAME_PIXEL_WIDTH` and `FRAME_PIXEL_HEIGHT` (C 1594, 1599, 1605,
  1606, 1617, 1622, 1629-1630, 1654, 1682) are the mirror's **functions**, so the port's `!` comes
  off (six names, eight sites);
  (6) C 1649/1769's `RANGED_FIXNUMP (0, arg, INT_MAX)` passes `INT_MAX as isize` (the mirror's
  bound parameter);
  (7) C 1800's `fset_redisplay (f)` is `fset_redisplay (f as *mut c_void)` (the mirror's untyped
  pointer, P-72 rule 3);
  (8) `w32_set_name`'s and `w32_set_title`'s `name` parameters (C 2063/2077 and C 2115 assign to
  them) are `mut name` (the three `E0384`s);
  (9) `w32_init_class`'s parameter is `hinst_` - C 2410's name shadows C 918's file-scope `hinst`,
  which Rust forbids (`E0530`); `INIT_WINDOW_CLASS!`'s body writes `hinst` as C's `#define` does,
  and this unit's only call site (C 2750) passes the file-scope object, so both readings agree (the
  `modifiers_` reading, and the token check folds the underscore);
  (10) C 2416's `L"Emacs"` (`EMACS_WIDE_CLASS`) is cast to the one `WNDCLASS` record's
  `lpszClassName` field (`*const c_char`), because that record stands for C's `WNDCLASS` *and*
  `WNDCLASSW` (the `w32menu.rs` W/A reading);
  (11) C 2455's `PostThreadMessage (…, WM_EMACS_SET_TOOLKIT_THEME, …)` passes the message id as
  `UINT` (the mirror's ids are `c_int`);
  (12) the two `clear_glyph_matrix (… .current_matrix)` calls (C 1662, 1690) cast the `struct
  window` member, which the mirror leaves untyped (P-72 rule 3), to `*mut glyph_matrix`.
- **`EMACS_CLASS`'s terminator is an observation, not a repair**: C 2424's `wc.lpszClassName =
  EMACS_CLASS` needs the implicit NUL of C's `"Emacs"`, while the port's `EMACS_CLASS: &str` +
  `.as_ptr ()` has no guaranteed terminator (its sibling `EMACS_WIDE_CLASS` is an explicit
  `[WCHAR; 6]` and does carry one).  Reported rather than touched: the constant is the prefix's and
  other units read it; a `&CStr` conversion is the mirror round's.

### Owed by the integrator (slice 4)

1. `femacs/xtask/Cargo.toml`: register `w32_debug_display` and `dbg_wm_chars` (the two features
   this slice names; slice 3's item 1 for `emacsdebug`/`msg_debug` is the same row).
2. `femacs/src/include/w32term.rs`: storage units (or `w32_output_bits!` arms) for the two
   `w32_output` bitfields C 378/400 record - `hourglass_p` and `menubar_active`.  This slice's nine
   `E0609`s are that row, and `src/w32menu.rs` 938 carries it for the same two members.
3. `femacs/src/include/frame.rs`: `tab_bar_window`, `tab_bar_resized`, `tab_bar_redisplayed`,
   `tool_bar_window`, `tool_bar_resized`, `tool_bar_redisplayed` (the ten prefix `E0609`s; slice
   1's item 8's list).
4. `femacs/src/include/keyboard.rs`: keyboard.h 525's `lispy_function_keys` declaration (C 524-526's
   `HAVE_NTGUI || HAVE_X_WINDOWS || HAVE_PGTK` arm, the `HAVE_NTGUI` table being C 5212-5403), and
   the deletion of this unit's copy in the same commit (P-88 ⑤'s move-back).
5. The in-unit SDK section this slice adds (item 2's list: `CRITICAL_SECTION`, the seven records,
   the ~45 entry points, the message ids/constants and the four macros) is a candidate for P-90 ①'s
   dedicated `<windows.h>` mirror, exactly as slice 2's item 6 and slice 3's item 8 record for
   their own sections; the same commit moves them and deletes the copies here.
6. `docs/ports/line-drift-tolerance.tsv`: still owed (slices 1-3's items); the file's drift is now
   **25.5%** (`C 12488 vs Rust 9302`).
7. `docs/ports/macro-body-notes.tsv`: **this lane appended one row** (`w32_wnd_proc`) plus its
   section comment - appended, never rewritten, and the row names C's own lines for each of its
   tokens.
8. The layout assertions for the new records are **`UNVERIFIED` on this machine** (no Windows SDK,
   no mingw toolchain): `CRITICAL_SECTION`, `PAINTSTRUCT`, `LOGFONT`, `LOGFONTW`,
   `MEASUREITEMSTRUCT`, `DRAWITEMSTRUCT`, `SIZE`, `WINDOWPOS`, `MINMAXINFO` join slice 1's item 10
   and slice 3's item 7 lists - a Windows/mingw clang probe has to measure them.
9. Slice 1's items 5-9/11, slice 2's items 3-7 (except the `CreateWindow` repair slice 3 made) and
   slice 3's items 2-8 are **unchanged by this lane**; slice 3's item 4 (the three prefix defects)
   is discharged by this slice's Deviations, and its item 3's alternative (the line-count row) is
   item 6 here.

### Owed by the next lane

- The next slice begins at **C 5988** (C's `my_create_window` doc comment; C 5987 is blank) and
  carries the rest of the file: C 5994 `my_create_window`, C 6253 `x-create-frame`, C 9581
  `w32_display_info_for_name`, C 10113 `w32_console_toggle_lock_key` and C 11586
  `syms_of_w32fns` - 6,501 C lines, i.e. more than one slice.
- `w32_wnd_proc`'s P-91 ④ station is discharged: it is defined in this file now, so
  `INIT_WINDOW_CLASS!`'s `Some (w32_wnd_proc)` (C 2404/C 3779) and `w32_msg_pump`'s
  `DispatchMessageW/A` path resolve, and `get_wm_chars`/`deliver_wm_chars` (C 4878's call, C 4802's
  `deliver_wm_chars (1, …)`) stand at their C positions.
- The two prefix defect classes this slice could not repair (`include/frame.rs`'s six members and
  `include/w32term.rs`'s two bitfields) are the next lane's arm-reading-starting point, unchanged.

## Slice 5 (C 5988-6121)

**Scope and the stop.**  The slice starts at C 5988 (`my_create_window`'s own doc comment, the item
the fourth slice's "Owed by the next lane" names) and stops at C 6121, the `}` of `w32_window` - a
whole-item boundary (C 6122 is blank and C 6123 starts `w32_icon`'s comment).  It carries **134 C
lines** and lands three `static` helpers: `my_create_window` (C 5993-6033), `my_create_tip_window`
(C 6040-6078) and `w32_window` (C 6083-6121).  The `x-create-frame` DEFUN that begins at C 6253 is
**not** in it (it runs to about C 9579 - a slice by size alone, as the addendum below records).

**New in this slice (the addendum's list was short one level).**  The three functions needed the
names the addendum names, and this slice spells them in-unit: `CW_USEDEFAULT` (`<winuser.h>`'s
`(int)0x80000000`), `resource_types` with C's six `RES_TYPE_*` values (`dispextern.h` 3983-3992,
`src/pgtkfns.rs` 498-509's shape) and `gui_display_get_arg` (`dispextern.h` 3994, whose `Display_Info`
is this unit's `w32_display_info`).  `GetMessage` needed no declaration: this file already reads
C's macro through its own `if w32_unicode_gui != 0 { GetMessageW } else { GetMessageA }` binding, and
this slice reuses that device (with a `get_message` local of the A/W function type).

### Evidence (slice 5, the final revision: 9,451 Rust lines)

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32fns5-lane cargo run -q -p xtask -- build-tu src/w32fns.c` ->
`built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, the object **336 bytes**.  The file
gate covers every item, so the frozen configuration compiles none of this slice's text either.

**② The arm readings (`probe`).**  Both runs are
`cargo run -q -p xtask -- probe src/w32fns.rs /tmp/w32fns5-lane/x.rlib --crate-type=lib
-C panic=abort --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"'`.  The first read **52 error
lines**; the second (after this slice's own repairs) **43**.  The nine-line difference is this
slice's own defect classes, all repaired: the `size_hint_flags`/`USPosition` width mismatch (the
field is `c_int`, the flags are `c_long`, so the cast is on the flags side), `WM_EMACS_CREATEWINDOW`/
`WM_EMACS_DONE` passed as `UINT` (`as UINT`, the constants are `c_int`), `EMACS_CLASS`'s `&str`
pointer cast to `*const c_char` for `CreateWindow`, `FRAME_EXTERNAL_MENU_BAR`'s `bool` return (C's
`!= 0` is the function itself) and `Vx_resource_name`'s **static** spelling (this file calls its Lisp
variables as statics, not as accessors, so C 6093's call takes the name bare).
**What is left of this slice in the 43 is one class, and it is not this file's**: `w32_output.dwStyle`
(2 lines) and `frame.explicit_name` (2 lines, with their 2 `E0606` companions) - the same
"mirror record lacks a member" class the fourth slice's evidence already names for
`menubar_active`/`hourglass_p`/`tab_bar_*`/`tool_bar_*` (27 of the 43) and the mirror `E0428`s.  They
join this file's owed list; nothing in this file's own text can spell a member the mirror lacks.

**③ The inventory (`tools/extract-abi.py --check emacs/src/w32fns.c femacs/src/w32fns.rs`).**
**205 -> 202** `- ` rows - the three functions this slice defines are no longer `missing function`
rows, and no new row names them - and the line-count drift row reads `24.3%: C 12488 vs Rust 9451`
(was 25.5%).  One row this slice's text produced -
`function my_create_window does not mention 2, getmessage` - is covered by **one row appended to
`docs/ports/macro-body-notes.tsv`** (append-only), which names C's own lines for both tokens: the
`GetMessage` macro's A/W binding above, and the array bound C writes inside the function
(`static int coords[2]`) but the port keeps at file scope with that static.

**④ The other gates (repo root, final revision), one line each - exit code first (P-87 ④).**
`[0] check-declarations.py` -> 0 failures. - `[0] check-own-voice.py` -> 0. - `[0]
check-mirror-dupes.py` -> 0. - `[0] check-port-lines.py` -> `0 file(s) under ten percent`. - `[0]
check-ledger-ticks.py` -> 0 failures. - `[0] check-carried.py` -> 0. - `[0] check-verbatim.py` -> 0. -
`[0] check-verbatim-declarations.py` -> 0. - `[0] check-citations.py` -> 0. - `[0]
check-dead-file-coverage.py` -> `info docs/ports/w32fns.md: 156 range row(s) cover src/w32fns.c
(12488 lines; 102 blank line(s) between blocks need no row)` and `0 failure(s)` (the table has this
slice's row).

**⑤ Lines.**  `femacs/src/w32fns.rs` **9,302 -> 9,451** lines; `docs/ports/w32fns.md` gains this
section; `docs/ports/macro-body-notes.tsv` gains one appended row (nothing rewritten).

### Deviations (slice 5)

- **C's `static int coords[2]` moves to file scope.**  `my_create_window`'s array has static storage
  duration inside the function, which Rust spells at file scope with its initialiser (the
  `force_quit_count`/`after_deadkey`/`last_x` reading this file already uses for the other four).
  The token check's `2` follows from that move and is recorded in the notes row.
- **`GetMessage` is the file's A/W binding, not a declaration.**  The macro's arm is chosen at run
  time by `w32_unicode_gui` (`src/w32fns.rs` 6155-6157's `w32_msg_pump` device), so the local
  `get_message` carries the A/W function type and the call goes through it.
- **`f->output_data.w32->` keeps this file's cast idiom** (`(*f).output_data.w32 as *mut w32_output`,
  the C 1221-1242 reading), which is why the two `dwStyle` sites read as they do; both are blocked on
  the mirror member named in ②, not on the cast.
- **`w32_window`'s `window_prompting` is unused in the body** - C never reads it either (the
  parameter exists for the caller's signature), so the port keeps it without an `_` prefix, as C
  does.
- **C 6051-6052's double store is kept.**  `tip_window = FRAME_W32_WINDOW (f) = CreateWindow (…)` is
  one C assignment nested in another; the port keeps both stores in one statement through the
  `FRAME_W32_WINDOW!` macro.

## Slice 6 (C 6123-6251)

**Scope and the stop.**  The slice starts at C 6123 (`w32_icon`'s doc comment; C 6122 is blank) and
stops at C 6251, the `}` of `w32_default_font_parameter` - a whole-item boundary (C 6252 is blank and
C 6253 opens the `x-create-frame` DEFUN).  It carries **129 C lines** and lands four `static`
helpers plus one C-visible definition: `w32_icon` (C 6127-6152), `w32_make_gc` (C 6155-6178),
`unwind_create_frame` (C 6185-6208), `do_unwind_create_frame` (C 6210-6214) and
`w32_default_font_parameter` (C 6216-6251).

**New in this slice.**  Two prototypes the unit's graph does not carry - `font_open_by_name`
(`font.h` 883, `src/frame.rs` 4788's shape) and `gui_default_parameter` (`dispextern.h` 4001-4003,
`src/pgtkfns.rs` 561-566's shape) - plus C 6233-6238's function-local `static const char *names[]`,
which becomes a file-scope `static mut names: [*const c_char; 5]` for the same static-storage reason
slice 5's `coords` did (and `mut` for a second one: a `static` holding raw pointers is not `Sync`, so
the probe's first run refused the shared one).

### Evidence (slice 6, the final revision: 9,624 Rust lines)

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32fns6-lane cargo run -q -p xtask -- build-tu src/w32fns.c` ->
`built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, the object **336 bytes**.

**② The arm readings (`probe`).**  The same command as slice 5's.  The first run read **52 error
lines**; each repair moved it: `*`'s precedence over `as` (**6 sites**, this slice's four and slice
5's two - `*(… as *mut w32_output)` dereferences *before* the cast, so the file's own two-paren
shape `(*((…) as …))` from `src/w32fns.rs` 2205 is the correct spelling), the `static mut` above,
and C 6051-6052's chained assignment (an assignment yields `()`, so C's two stores are two
statements, in C's order).  The reading is now **38 error lines** - **five fewer than slice 5's 43**,
because this slice's repairs also cleared the two `dwStyle`-class lines slice 5 left.  Of the 38,
**only `frame.explicit_name` (2 lines) points into this slice's text**: the `frame` mirror lacks that
member, the same "mirror record lacks a member" class as `menubar_active`/`hourglass_p`/`tab_bar_*`/
`tool_bar_*` (27 lines) and the mirror `E0428`s.

**③ The inventory (`tools/extract-abi.py --check emacs/src/w32fns.c femacs/src/w32fns.rs`).**
**202 -> 197** `- ` rows and the drift row reads `22.9%: C 12488 vs Rust 9624` (was 24.3%).  The five
rows this slice's final text retired: the three it produced (two token rows -
`w32_icon`/`w32_make_gc` `does not mention 0`, C's `0` arguments spelled `core::ptr::null ()`/
`null_mut ()` - and the `w32_default_font_parameter is not static in C but has no
#[unsafe(no_mangle)]` row, which the attribute then satisfied) are covered by **two appended
`macro-body-notes.tsv` rows** (append-only) plus that one attribute; the other two are the
`missing function` rows the definitions retire.

**④ The other gates (repo root, final revision), one line each - exit code first (P-87 ④).**
`[0]` for `check-declarations.py`, `check-own-voice.py`, `check-mirror-dupes.py`, `check-port-lines.py`,
`check-ledger-ticks.py`, `check-carried.py`, `check-verbatim.py`, `check-verbatim-declarations.py`,
`check-citations.py` and `check-dead-file-coverage.py` - the last reporting
`info docs/ports/w32fns.md: 157 range row(s) cover src/w32fns.c (12488 lines; 102 blank line(s)
between blocks need no row)` and `0 failure(s)`.

**⑤ Lines.**  `femacs/src/w32fns.rs` **9,451 -> 9,624** lines; this section; two appended
`macro-body-notes.tsv` rows.

### Deviations (slice 6)

- **C 6051-6052's chained assignment is two statements.**  `tip_window = FRAME_W32_WINDOW (f) =
  CreateWindow (…)` nests one C assignment in another; Rust's assignment expression is `()`, so the
  port keeps two stores in C's order (the frame's `window_desc` first, then the file-scope
  `tip_window`) - a repair of slice 5's text, recorded here because that is where the defect landed.
- **The `*`-before-`as` precedence repair touches six sites, two of them slice 5's.**  This file's own
  working spelling is `(*((*f).output_data.w32 as *mut w32_output))` (`src/w32fns.rs` 2205-2212, C
  1269-1277's live half), and the two-paren shape is what keeps the cast outside the deref.
- **C 6193-6195/6200-6203's two `#ifdef GLYPH_DEBUG` arms are the `glyph_debug` feature.**  The
  declaration is `undef` in the frozen configuration, both arms are carried, and C 6202's
  `eassert` reads the file-scope `static int dpyinfo_refcount` this unit already declares (C 293).
- **The `names` array is `static mut`.**  C 6233-6238's function-local `static const char *names[]`
  has static storage duration (file scope here, the `coords` reading) and holds raw pointers, so it is
  `static mut` - a shared `static` would need `Sync`, which `*const c_char` is not.
- **`w32_default_font_parameter` keeps C's linkage.**  C defines it without `static`, so the port's
  definition is `pub unsafe extern "C" fn` **with** `#[unsafe(no_mangle)]` (the inventory row above is
  what caught the missing attribute); the four helpers stay private, as their C `static` says.

## Slice 7 (C 6253-6712)

**Scope and the stop.**  C 6253-6712 - the whole of the file's first `DEFUN`, `x-create-frame`
(`Fx_create_frame`/`Sx_create_frame`, C 6254's `1, 1, 0`, C 6256's single `Lisp_Object parameters`),
from C 6253's `DEFUN (` line through C 6712's closing brace.  The stop is there because C 6713 is
blank and C 6714 opens the next `DEFUN` (`xw-color-defined-p`): the item's own end, 460 reference
lines, the file's second-longest item after `w32_wnd_proc` (C 4525-5986, four slices).  The
`## C line ranges` table's old `5988-12488` remainder row is split accordingly: `5988-6712` is
carried now (slices 5, 6 and 7 - `my_create_window`, `my_create_tip_window`, `w32_window`,
`w32_icon`, `w32_make_gc`, `unwind_create_frame`, `do_unwind_create_frame`,
`w32_default_font_parameter` and this `DEFUN`) and `6714-12488` stays the remainder for the next
lanes.

**What the item carries.**  C 6257-6267's thirteen locals (one `specpdl_ref` from
`SPECPDL_INDEX`), C 6269-6271's two frame predicates and its `error`, C 6275's `Fcopy_alist`,
C 6279's `Vx_resource_name = Vinvocation_name;`, C 6281-6288's `Qterminal`/`Qdisplay` pair with C's
two `0` name arguments, C 6289's `check_x_display_info` and its `terminal->kboard` read,
C 6291-6292's liveness `error`, C 6294-6302's name/resource pair, C 6305-6310's `parent-id`,
C 6315-6329's minibuffer dispatch (`make_frame_without_minibuffer` twice,
`make_minibuffer_frame`, `make_frame`), C 6331/6333's `XSETFRAME`/`frame_set_id_from_params`,
C 6335-6348's `parent-frame`, C 6350-6359's `undecorated`/`skip_taskbar` pair,
C 6362-6363's two `GetSystemMetrics` sizes, C 6365-6375's terminal/`output_method`/`xzalloc`/
`FRAME_FONTSET`/`gui_init_fringe` block, C 6377-6384's icon name, C 6386's commented-out
`FRAME_DISPLAY_INFO` line (carried as a comment), C 6389's `record_unwind_protect` with this unit's
`do_unwind_create_frame`, C 6391-6393's `#ifdef GLYPH_DEBUG` arm (`glyph_debug`),
C 6398-6409's `parent_desc`/`explicit_parent` pair, C 6411-6424's name block with
`FRAME_EXPLICIT_NAME` and `specbind (Qx_resource_name, name)`, C 6426-6432's three
`register_font_driver` calls under `have_harfbuzz`, C 6434-6522's thirty-four
`gui_default_parameter` calls (and C 6447-6457/6464-6474's two `Fassq`/`Fcons` blocks),
C 6526's `init_frame_faces`, C 6540-6550's `min-width`/`min-height` plus `adjust_frame_size`
(`5, true, Qx_create_frame_1`), C 6555-6608's menu/tab/tool bar block, `inhibit-double-buffering`,
`buffer-predicate`, `title`, C 6585's `parent_desc` re-read, C 6586-6602's fifteen
`w32_load_cursor` calls and `current_cursor`, C 6604's `gui_figure_window_size`, C 6606-6608's
`unsplittable`/`no_split`, C 6610-6613's `w32_window`/`w32_icon`/`w32_make_gc` (this unit's own,
slices 5 and 6), C 6615-6618's "official frame" block, C 6622-6634's six post-window
`gui_default_parameter` calls, C 6637's `can_set_window_size`, C 6642-6647's
`w32_wm_set_size_hint`/`adjust_frame_size` pair, C 6652-6661's fullscreen/z-group/alpha run,
C 6666-6686's visibility block, C 6689-6691's delayed alpha, C 6695-6698's
`default-minibuffer-frame` (`KVAR`), C 6703-6705's remaining-parameters loop, C 6709's
`Vwindow_list = Qnil;` and C 6711's `unbind_to (count, frame)`.

**Two unit-local macros, under C's own names.**  C 6367's `f->output_method = output_w32;` writes
frame.h 446's four-bit `ENUM_BF`, which `include/frame.rs` mirrors as a *reader* plus the
`_UNIT`/`_SHIFT`/`_MASK` constants and the `bits1` word: this unit adds `set_output_method!` and the
two-arm `output_method!` (the `src/androidfns.rs` 283-302 / `src/pgtkfns.rs` 240-249 device, whose
C line is the same statement in their own `Fx_create_frame`).  C 6696-6697's two
`KVAR (kb, Vdefault_minibuffer_frame)` sites need keyboard.h 38's token-pasting macro, which
`include/keyboard.rs` 10-13 records as having no counterpart: the unit defines the one-arm `KVAR!`
the `src/haikufns.rs` 1297-1301 spelling of these same C lines uses, naming the mirror's
`Vdefault_minibuffer_frame_` field.

**Eight new prototypes.**  This unit's includes carry none of `make_frame`,
`make_minibuffer_frame`, `make_frame_without_minibuffer`, `frame_set_id_from_params`,
`fset_parent_frame`, `gui_figure_window_size`, `register_font_driver` or `gui_init_fringe`, so the
slice declares them with their `frame.h`/`font.h`/`xterm.h` lines (the `src/androidfns.rs` 95-171
device).  `struct kboard` has a mirror, so the two prototypes that name it keep the record;
`gui_init_fringe` keeps the untyped `*mut c_void` the two sibling spellings use.

### Evidence (slice 7, the final revision: 10,220 Rust lines)

**① The frozen configuration.**  `cargo run -q -p xtask -- build-tu src/w32fns.c` →
`built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, `build/femacs/w32fns.o` **336 bytes**.
The reading is vacuous for the new text the same way it is for every slice of this file: the whole
body sits behind `have_ntgui`, so the frozen feature set compiles only the two `config.h` includes
and the `#![allow]` attribute.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Before this slice the same command read **36**
errors; the final revision reads **42** (`41` carrying an `error[Exxxx]` code, plus the mirror's
`compile_error!` for the unmeasured `HAVE_NTGUI` arm of `struct frame`'s flag run).  The six new
ones are one class and all of them are the same story - a member the mirror *records* without
packing it: `explicit_parent` (C 6403, C 6408 and C 6666, three sites), `no_split` (C 6608),
`can_set_window_size` (C 6637) and `was_invisible` (C 6682).  Two fix rounds were needed first, both
on this slice's own text: `(*((*f).terminal)).kboard as *mut kboard` (the first spelling
dereferenced the untyped member before the cast, `E0606`/`E0614`) and the same shape on three more
`terminal` members, then the two macros above (the `KVAR`/`output_method` rows were
`cannot find macro` before them).  Nothing here claims a Windows reading: there is no mingw
toolchain and no `<windows.h>` on this machine, so `IDC_*`/`SM_CX*SCROLL` and every other SDK name
of the slice are spelled from PORTING.md's table and never compiled.

**③ `extract-abi.py --check`.**  `python3 tools/extract-abi.py --check emacs/src/w32fns.c
femacs/src/w32fns.rs` → **197 rows** (the count slice 6 left), with one row retired and one added:
`missing subr Fx_create_frame (x-create-frame)` is gone (this slice's `DEFUN!` declares the subr) and
`subr Fx_create_frame does not mention explicit_name` arrives (C 6416/6421's member, which the port
writes through `FRAME_EXPLICIT_NAME!`; the row's `macro-body-notes.tsv` entry is below).  The
line-count drift row falls from **22.9% to 18.2%** (`C 12488 vs Rust 10220`).  `missing defsubr
(&raw const Sx_create_frame)` stays: C's `defsubr` call is in `syms_of_w32fns` (C 11949), which no
slice has reached.

**④ The range table and the dead-file gate.**  The `## C line ranges` row above covers the item, and
`python3 tools/check-dead-file-coverage.py` reads `156 range row(s) cover src/w32fns.c (12488 lines;
103 blank line(s) between blocks need no row)`, `0 failure(s)`.  The same command reads 155 rows and
102 blank lines on the previous revision: the split above adds one row, and C 6713's blank line
moves from inside a row to between two.

### Deviations (slice 7)

- **The six member sites are owed to the mirrors, not to a choice here.**  `w32_output`'s
  `explicit_parent` (`include/w32term.rs` 499-501 records the three bitfields and packs none) and
  `frame`'s `no_split`, `can_set_window_size` and `was_invisible` (`include/frame.rs` 485/657/732
  record the `bool_bf` bits) keep C's own spelling in this slice's text; the arm's six errors are the
  mirror's, and the ledger's "Owed by the integrator" list carries them.
- **C's `kb` read is a cast because the mirror keeps `terminal.kboard` untyped.**  C 6289 is
  `kb = dpyinfo->terminal->kboard;` with `struct kboard *kb` (C 6267); `include/termhooks.rs` 220
  spells the member `*mut c_void` (the pointee has a mirror, `include/keyboard.rs` 56, but the
  *terminal* member's record does not, so P-72 rule ③ applies at that one member).  The three
  consumers keep C's record type.
- **`do_unwind_create_frame` gains the C ABI in this slice.**  Slice 6 defined it `unsafe fn`;
  C 6389 takes its address for `record_unwind_protect`, and the four sibling units define the same
  function as `unsafe extern "C" fn` (`src/androidfns.rs` 991, `src/haikufns.rs` 1289,
  `src/pgtkfns.rs` 2007, `src/xfns.rs` 6688), so slice 7 corrects the definition and says so in its
  doc comment.
- **C 6281/6284/6606/6669's `0` name arguments** are the port's `core::ptr::null ()`, the file's
  existing reading; the token check is silent about them on this item (it counts what the body
  carries, and the item's other tokens still compare), so no `macro-body-notes.tsv` row is needed
  for them.
- **C 6520/6689's `FRAME_NO_FOCUS_ON_MAP (f)` reads** use the mirror's reader function with C's
  truth test written as `!= 0` / `== 0` (the file's `has_palette == 0` reading), because the
  mirror returns `c_int`; the write form stays this unit's `FRAME_NO_FOCUS_ON_MAP!` macro.
- **Not claimed.**  No run evidence: no frame, window, cursor, GC or font is created on this
  machine; the frozen object is empty and the arm reading stops at resolution errors in mirrors.
  `x-create-frame` has not been called and no Lisp-level behavior is asserted.

## Slice 8 (C 6714-6915)

**Scope and the stop.**  C 6714-6915 - the seventeen display-info `DEFUN`s, in C's order:
`xw-color-defined-p` (6714-6727), `xw-color-values` (6729-6744), `xw-display-color-p` (6746-6756),
`x-display-grayscale-p` (6758-6769), `x-display-pixel-width` (6771-6779),
`x-display-pixel-height` (6781-6789), `x-display-planes` (6791-6799),
`x-display-color-cells` (6801-6816), `x-server-max-request-size` (6818-6825),
`x-server-vendor` (6827-6832), `x-server-version` (6834-6839), `x-display-screens` (6841-6846),
`x-display-mm-height` (6848-6863), `x-display-mm-width` (6865-6879),
`x-display-backing-store` (6881-6887), `x-display-visual-class` (6889-6907),
`x-display-save-under` (6909-6915).  The stop is C 6917's `static BOOL CALLBACK ALIGN_STACK` +
`w32_monitor_enum` (6907's closing brace ends the last `DEFUN`), which opens the monitor block
(C 6917-7082) - a different subject with its own reader set (`MONITOR_INFO_EX`, the three
dynamically loaded entry points, `ASET`/`AREF`), so it is the next lane's item.

**What the item carries.**  Two `decode_window_system_frame` readers (C 6719/6734) over
`Emacs_Color` out-params, the `n_planes * n_cbits` products (C 6752/6765/6798/6814/6899/6901/6903),
`make_fixnum` of the two `w32term.h` pixel readers (C 6778/6788), three literal returns
(C 6824/6845/6914), `list3i` of C 882-888's three version ints (C 6838), the two
`GetDC`/`GetDeviceCaps`/`ReleaseDC` mm computations (C 6857-6862, C 6873-6878), the
`Qnot_useful`/`Qpseudo_color`/`Qstatic_gray`/`Qstatic_color`/`Qtrue_color` ladder (C 6886-6904) and
`include/lisp.rs` 3160's `min` in C 6814's shift count.

**Two deviations this item needs, both the port's standing rules.**
`Emacs_Color foo;` (C 6718/6733) is one of PORTING.md 1.5's outputs-only locals - Rust needs a
value before `&raw mut` can name it, so both are `let mut foo: Emacs_Color = unsafe {
core::mem::zeroed () };`, the spelling this unit already gives C 1201's `cdef`.  And
`(GetRValue (x) << 8) | GetRValue (x)` (C 6739-6741) leans on C's integer promotion of a `BYTE`
before `<<`; the port casts each read to `c_int` first, the shape C 2094's sibling uses for the
same three channel macros.

**The name sweep, and eight names it found.**  Slice 8's first arm reading included two
`E0381`s (`used binding \`foo\` isn't initialized`) from those two locals; they are fixed above.
While checking them, the slice measured the arm reading's own blind spot: an unresolved *value*
name inside a `DEFUN!` body is **not** reported by the probe (a deliberately injected
`let _ = SM_CXVSCROLL_ZZZ_PROBE;` in `Fx_create_frame`'s body does not appear in the error list,
while member errors (`E0609`) and initialization errors (`E0381`) from the very same body do).
So this slice added a textual sweep as a reading: for a code region, strip comments and string
literals, drop field accesses and macro invocation names, and require an item of that name
anywhere in `femacs/`.  It reports, over the two regions of the file's `have_ntgui` text
(`src/w32fns.rs` 9620-10259 = slices 5-7, and 10245-10561 = this slice), **missing: []** - after
the eight names it found are defined:

| name | C line that reads it | value | site |
| --- | --- | --- | --- |
| `SM_CXVSCROLL` | 2125, 6362 | 2 | next to C 283-290's four `<winuser.h>` metrics |
| `SM_CXHSCROLL` | 2127, 6363 | 4 | same block |
| `IDC_IBEAM` | 6586 | `MAKEINTRESOURCE (32513)` | C 3689's `IDC_ARROW` block |
| `IDC_WAIT` | 6590 | 32514 | same block |
| `IDC_SIZENWSE` | 6594, 6598 | 32642 | same block |
| `IDC_SIZENESW` | 6596, 6600 | 32643 | same block |
| `IDC_SIZEWE` | 6591, 6593, 6597 | 32644 | same block |
| `IDC_SIZENS` | 6592, 6595, 6599 | 32645 | same block |

The first two were used from slice 5 on (C 2125/2127's `w32_set_scroll_bar_default_width`/
`_height`) and the six cursor ids are slice 7's (C 6586-6600); none had a definition anywhere in
the tree, and no reading of record saw them - which is what the sweep exists to catch.  The same
slice defines the four `<wingdi.h>` metrics C 6858-6859 and C 6874-6875 read (`HORZSIZE` 4,
`VERTSIZE` 6, `HORZRES` 8, `VERTRES` 10), C 6857/6873's `GetDeviceCaps` declaration, and C
6719/6734's `decode_window_system_frame` prototype (frame.h 1475).

### Evidence (slice 8, the final revision: 10,565 Rust lines)

**① The frozen configuration.**  `cargo run -q -p xtask -- build-tu src/w32fns.c` →
`built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, `build/femacs/w32fns.o` **336
bytes** - the same empty-object reading every slice of this file gives, since the whole unit sits
behind `have_ntgui`.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  First reading of this slice: **44 previous
errors** (43 with a code), of which **two are `E0381`** - this slice's own two locals, fixed by the
zero initialisers above.  Final reading: **42 previous errors** (41 with a code, plus the mirror's
`compile_error!` for the unmeasured HAVE_NTGUI arm of `struct frame`'s flag run), i.e. **the same
total slice 7 left**: the seventeen `DEFUN`s add no error and no member gap.  The remaining classes
are the mirrors' (eleven `E0428`s, `E0530`, and the `w32_output`/`frame` members recorded but not
packed: `menubar_active` 6, `hourglass_p` 3, `explicit_parent` 3, `tab_bar_*`/`tool_bar_*` 10,
`explicit_name` 2, `was_invisible`, `no_split`, `can_set_window_size`).  Four probe runs in total.

**③ `extract-abi.py --check`.**  196 rows → **179** (measured minutes apart in one tree state):
the seventeen `missing subr` rows of C 6714-6915 retire and **no row is added**; the drift row
reads **15.4%** (`C 12488 vs Rust 10565`), down from 18.2%.  (The row count moves with the
ledger's own text as well as the unit's - slice 7's entry recorded 197 for the same file, measured
against the ledger it had just written - which is why this pair is quoted as one measurement pair
rather than across slices.)

**④ The range table and the dead-file gate.**  The `## C line ranges` table splits the old
`6714-12488` remainder row into `6714-6915` (this slice) and `6917-12488` (the remainder);
`python3 tools/check-dead-file-coverage.py` reads **157 range row(s) cover src/w32fns.c (12488
lines; 104 blank line(s) between blocks need no row)**, `0 failure(s)` (156 rows / 103 blanks
before the split).

### Deviations (slice 8)

- **The two `Emacs_Color` locals are zero-initialised** (PORTING.md 1.5), the file's C 1201
  `cdef` precedent; C leaves them uninitialised and passes their address.  The probe reported the
  difference as `E0381` until the initialisers were added.
- **C's `BYTE` promotion before `<<` is spelled as a `c_int` cast** (C 6739-6741), per the
  `src/w32fns.rs` 2094 precedent - C's `GetRValue` yields a `BYTE` that C promotes to `int`
  before the shift, and Rust's `u8 << 8` would be an overflow.
- **The eight `<winuser.h>` constants and the four `<wingdi.h>` metrics are spelled from the
  headers** (not from the reference tree, which contains only the uses): `MAKEINTRESOURCE (n)`
  values for the cursor ids and the small ordinals for the metrics, with the C lines that read
  each one named at the definition site.
- **`decode_window_system_frame` is declared here**, since this unit's include list does not carry
  frame.h's prototype (the `src/androidfns.rs` 108 shape); `GetDeviceCaps` likewise joins the
  `<windows.h>` block where C 6857/6873's `GetDC`/`ReleaseDC` already stand.
- **Not claimed.**  No run evidence: no display is opened, no device context is acquired and no
  color is looked up on this machine; the arm reading is a compile-level result that stops at the
  mirrors' errors, and the name sweep is a textual reading, not a compile of the Windows arm.

## Slice 9 (C 6917-7106)

**Scope and the stop.**  The monitor block: `w32_monitor_enum` (C 6917-6925),
`w32_display_monitor_attributes_list` (C 6927-7025),
`w32_display_monitor_attributes_list_fallback` (C 7027-7082) and the
`w32-display-monitor-attributes-list` `DEFUN` (C 7084-7106).  The stop is C 7108's
`set-message-beep` `DEFUN` (C 7107 blank) - the next item, whose subject (the bell) is unrelated
to the monitor enumeration.

**What the item carries.**  The `extern "system"` callback C 6917 declares (the `MonitorEnum_Proc`
signature this unit already carries for `enum_display_monitors_fn`); the enumeration arm's three
`Option` entry-point tests and their `(f.unwrap ()) (…)` call sites (C 6935-6941, the device C
498-503 established); `xmalloc`/`xfree` with C's `n * sizeof (*p)` byte count (C 6948/7022);
`Fmake_vector` (C 6955), `ASET`/`AREF` (C 6971/6993/7000/7006/7011 - `include/lisp.rs` 1324/1336's
functions, whose index parameter is `isize`), `FOR_EACH_FRAME` (C 6956/7043, the three-argument
macro `include/frame.rs` provides), `MONITOR_INFO_EX`'s four members and its `szDevice`
(C 6980-6996), `CreateDCA`/`DeleteDC` (C 6986/6991), the fallback arm's
`GetDC`/`GetDeviceCaps`/`ReleaseDC` chain (C 7055-7062), `SystemParametersInfo` with
`SPI_GETWORKAREA` (C 7071), `SM_XVIRTUALSCREEN`/`SM_YVIRTUALSCREEN` (C 7068-7069) and C
7087-7093's four-paragraph doc string (the `src/androidfns.rs` 1971 spelling).

**Three declarations and one constant this item needed.**  `CreateDCA`, `DeleteDC` and
`SystemParametersInfo` join the `<windows.h>` block (the last with the `<winuser.h>` line
`src/w32term.rs` 326 records for C 384, since this unit's own block had no copy), and
`SPI_GETWORKAREA` (0x0030) stands with the `<wingdi.h>` metrics slice 8 added.

**Deviations (slice 9).**

- **Two PORTING.md 1.5 locals**: C 6980's `struct MONITOR_INFO_EX mi;` (filled by
  `get_monitor_info_fn`) and C 7034's `RECT workarea_rect;` (filled by `SystemParametersInfo`) are
  zero-initialised here, because Rust needs a value before `&raw mut` can name it.
- **`xmint_pointer`'s result is cast to `HMONITOR`** (C 6951): the mirror's function returns
  `*mut c_void` (the `include/lisp.rs` 1679 shape) and `HMONITOR` is the same pointer type in this
  unit, so the cast is C's own assignment type written out.
- **C's `for` loop increments are re-spelled at the two `continue`s** (C 6983-6984 and C 6987-6988
  inside C 6975's `for (i = 0; i < n_monitors; i++)`): the port's loop is C's `while` shape
  elsewhere in this file, so each `continue` carries C's `i++` explicitly.
- **C 6977's one declaration with one initialiser** (`Lisp_Object geometry, workarea, name,
  attributes = Qnil;`) becomes four `let`s; only `attributes` keeps the initialiser, exactly as C
  writes it, and C's own order (attributes, then name, then the two geometry lists) is what makes
  the three uninitialised ones legal.
- **Not claimed.**  No run evidence: no monitor is enumerated on this machine (`enum_display_monitors_fn`
  is never bound here - the three entry points come from a `GetProcAddress` path C reaches at
  C 7198-7210, which no slice has ported yet), no device context is created and no Lisp list is
  built; the arm reading is a compile-level result and the name sweep a textual one.

### Evidence (slice 9, the final revision: 10,836 Rust lines)

**① The frozen configuration.**  `cargo run -q -p xtask -- build-tu src/w32fns.c` → `built
src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, object **336 bytes** (the file gate keeps
every item out of this configuration).  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  First reading: **43 previous errors** (42 coded)
- exactly one more than slice 8's 42/41, and it is this slice's: `E0308` at C 6986's `CreateDCA`
call, whose fourth argument (`const DEVMODEA *`) is `*mut c_void`, so `core::ptr::null ()` had to
become `core::ptr::null_mut ()`.  Final reading: **42 previous errors** (41 coded) - identical to
slice 8's, i.e. this slice adds no error and no member gap.  Two probe runs, plus one baseline
reading of the pre-slice file taken at the start of the round.

**③ `extract-abi.py --check`.**  179 rows → **175**: the two `missing function` rows of C 6927/7027,
C 6917's `missing function w32_monitor_enum`, and the DEFUN's `missing subr
Fw32_display_monitor_attributes_list` retire, with **no row added**; the drift row reads **13.2%**
(`C 12488 vs Rust 10836`), down from 15.4%.  (Both readings were taken in place, minutes apart,
with the same ledger.)

**④ The range table and the dead-file gate.**  The `6917-12488` remainder row splits into
`6917-7106` (this slice) and `7108-12488`;
`python3 tools/check-dead-file-coverage.py` reads **158 range row(s) cover src/w32fns.c (12488
lines; 105 blank line(s) between blocks need no row)**, `0 failure(s)` (157 rows / 104 blanks
before the split).

**⑤ The name sweep.**  Over this slice's region (`src/w32fns.rs` 10565-10836) the sweep reads
**48 capitalized identifiers, missing: []** - `CreateDCA`, `DeleteDC`, `SystemParametersInfo` and
`SPI_GETWORKAREA` are declared by this slice itself, and everything else the region uses
(`MonitorEnum_Proc`, `enum_display_monitors_fn`, `FOR_EACH_FRAME`, `ASET`/`AREF`, `DECODE_SYSTEM`,
`Qframes`/`Qname`/`Qmm_size`/`Qworkarea`/`Qgeometry`, `list1`/`list2i`/`list4i`) already resolves.

## Slice 10 (C 7108-7244)

**Scope and the stop.**  Three items in C's order: the `set-message-beep` `DEFUN` (C 7108-7136),
`w32_display_info_for_name` (C 7142-7164) and the `x-open-connection` `DEFUN` (C 7166-7244).  The
stop is C 7246's `x-close-connection` `DEFUN` (C 7245 blank) - the next item.

**What the item carries.**  C 7116-7133's eight-arm ladder onto C 338's file-scope
`static unsigned int sound_type` (this unit's own `static mut sound_type: c_uint`, slice 1);
C 7149's `one_w32_display_info` walk with `Fstring_equal`/`XCAR`/`name_list_element` (the port's
`while ! dpyinfo.is_null ()` for C's `for (…; dpyinfo; …)`); C 7156/7228's
`validate_x_resource_name`; C 7158/7232's `w32_term_init` (`include/w32term.rs` 330-331's
declaration, whose two `char *` parameters take the null pointer and `SSDATA`'s `*mut c_char`);
C 7180's `strcmp` name check and its `error`; C 7185's `window_system_available (NULL)`;
C 7193-7204's rgb.txt block (`Ffile_readable_p`, `Fexpand_file_name`, `Fsymbol_value`,
`Qdata_directory`, `Fx_load_color_file`); C 7206's `w32_default_color_map` (this unit's own);
C 7209's `add_system_logical_colors_to_map`; C 7218-7225's basename block (`lispstpcpy`,
`strrchr`, `MAX_PATH`, the `.exe` truncation); and C 7237/7240's `fatal`/`error` pair.

**Six declarations this item needed, and one of them closes an older gap.**
`MB_OK`/`MB_ICONHAND`/`MB_ICONQUESTION`/`MB_ICONEXCLAMATION`/`MB_ICONASTERISK` are spelled next to
C 339's `MB_EMACS_SILENT` with `<winuser.h>`'s values; `fatal` is declared variadic (C 7237 is its
only use in this unit); and `validate_x_resource_name` joins the extern block because this unit's
text has called it since **slice 6** (C 6121) without any declaration in the file - `frame.c` 6180
defines it and no header declares it, so each unit declares its own (the `src/xfns.rs` 6877-6883
note).  The name sweep below is what named that gap.

**Deviations (slice 10).**

- **C 7219's `char basename[MAX_PATH], *str;`** is two PORTING.md 1.5 locals in one C statement:
  the array is zero-initialised here (`MAX_PATH` is this file's `c_int` 260, so the length is
  `MAX_PATH as usize`) and `str_` is assigned by `strrchr` before its `is_null ()` test, which is
  why C's one-line `if (str) *str = 0;` stays one line.
- **C 7160's `dpyinfo == 0`** is the port's `dpyinfo.is_null ()` - the `0` token row this produces
  is closed by an appended `docs/ports/macro-body-notes.tsv` row (append-only), not by changing the
  port's spelling.
- **C 7194's block-local `Lisp_Object color_file;`** keeps C's shape (declared, then assigned by
  C 7196's `build_string` before both reads), so no initialiser is added.
- **Not claimed.**  No run evidence: this machine opens no display, so `x-open-connection` has
  never run and `w32_term_init` is never called; the arm reading is a compile-level result and the
  name sweep a textual one.

### Evidence (slice 10, the final revision: 11,019 Rust lines)

**① The frozen configuration.**  `cargo run -q -p xtask -- build-tu src/w32fns.c` → `built
src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, object **336 bytes**.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - identical to slice 9's
count, i.e. these three items add no error and no member gap; the three references into this
slice's region (10840/10865/10933) are `warning: unused doc comment` on the `DEFUN!` invocations,
the file's standing pattern.  One probe run.

**③ `extract-abi.py --check`.**  175 rows → **172**: `missing function w32_display_info_for_name`,
`missing subr Fset_message_beep` and `missing subr Fx_open_connection` retire; the check's one new
row (`function w32_display_info_for_name does not mention 0`, C 7160) is closed by the appended
`macro-body-notes.tsv` row above, so the final count is three rows down.  The drift row reads
**11.8%** (`C 12488 vs Rust 11019`), down from 13.2%.  (Both readings taken in place, minutes
apart, with the same ledger.)

**④ The range table and the dead-file gate.**  The `7108-12488` remainder row splits into
`7108-7244` (this slice) and `7246-12488`;
`python3 tools/check-dead-file-coverage.py` reads **159 range row(s) cover src/w32fns.c (12488
lines; 106 blank line(s) between blocks need no row)**, `0 failure(s)` (158 rows / 105 blanks
before the split).

**⑤ The name sweep.**  Over this slice's region (`src/w32fns.rs` 10836-11019): **37 capitalized
identifiers, missing: []** (the two `Sset_message_beep`/`Sx_open_connection` subr statics the
`DEFUN!` macro generates are the only names the sweep cannot see).  `validate_x_resource_name`,
`fatal` and the five `MB_*` ids are declared by this slice itself.

## Slice 11 (C 7246-7388)

**Scope and the stop.**  `x-close-connection` (C 7246-7263), `x-display-list` (C 7265-7276),
`x-synchronize` (C 7278-7283), then C 7291-7388's
`#if 0 /* TODO : port window properties to W32 */` block.  The stop is C 7390's Tool-tips banner
(C 7389 blank) - the next subject.

**What the item carries.**  The three live `DEFUN`s: C 7253's reference-count test and its `error`
(C 7254 is `error`'s own format), C 7256-7260's `block_input` / `image_destroy_all_bitmaps` /
`w32_delete_display` / `unblock_input`; C 7269-7275's `x_display_list` walk with
`name_list_element`/`XCAR`/`Fcons` (C 7272's `for` becomes the file's `while ! … .is_null ()`
shape); and C 7282's literal `Qnil`.  `w32_delete_display` (mirror 1126-1129) and `x_display_list`
(mirror 281-287) are declared by `include/w32term.rs`; `image_destroy_all_bitmaps` is not, so this
slice declares it with **this unit's** display record - C's parameter is `dispextern.h` 3741's
`Display_Info *`, and each unit spells that with its own record (the `src/pgtkterm.rs` 6534-6544
device).

**The `#if 0` block is kept as C's own text.**  C 7291-7388 (three window-property `DEFUN`s over
`XInternAtom`/`XChangeProperty`/`XGetWindowProperty`) never compiles, and this port has no Xlib, so
the block is carried inside a Rust block comment with C's text verbatim - the device C 1278-1417 and
C 1552-1583 already use in this file (Rust block comments nest, so C's own inner `/* … */` runs do
not end the region).  Neither `--check` nor the checklist expects the three subrs, because C's
`#if 0` keeps them out of `syms_of_w32fns`' `defsubr` list (the reading below names no
`missing subr` for them).

### Evidence (slice 11, the final revision: 11,198 Rust lines)

**① The frozen configuration.**  `cargo run -q -p xtask -- build-tu src/w32fns.c` → `built
src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, object **336 bytes**.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - identical to slice 10,
so the three live items add no error and no member gap; the four references into this slice's
region (11026/11053/11077/11197) are `warning: unused doc comment` on the `DEFUN!` invocations and
on the `image_destroy_all_bitmaps` declaration note.  One probe run.

**③ `extract-abi.py --check`.**  172 rows → **169**: the three `missing subr` rows for
`Fx_close_connection`, `Fx_display_list` and `Fx_synchronize` retire and **no row is added** (the
`#if 0` block contributes none, as above).  The drift row reads **10.3%**
(`C 12488 vs Rust 11198`), down from 11.8%.

**④ The range table and the dead-file gate.**  The `7246-12488` remainder row splits into
`7246-7388` (this slice, its `#if 0` half marked "dead in every configuration") and `7390-12488`;
`python3 tools/check-dead-file-coverage.py` reads **160 range row(s) cover src/w32fns.c (12488
lines; 107 blank line(s) between blocks need no row)**, `0 failure(s)` (159 rows / 106 blanks
before the split).

**⑤ The name sweep, and one limitation of the reading itself.**  Over this slice's region
(`src/w32fns.rs` 11019-11198) the sweep reports **36 capitalized identifiers, five "missing"**:
`FRAME_W32_DISPLAY`, `Xlib` and the three `Sx_*` subr names of the dead `DEFUN`s.  All five are
**inside the `#if 0` text this slice carries as a comment** - and the sweep's comment scanner is
naive: it ends a block comment at the first `*/`, while Rust's own block comments nest (measured:
the comment depth at the C 1278-1417 region's `FRAME_W32_DISPLAY` site, `src/w32fns.rs` 2260, is
**2**).  So the five are an artifact of the sweep, not gaps; the sweep's results must be read that
way for any region that carries a nested-comment block.  Everything the *live* part of this slice
names resolves (`w32_delete_display`, `x_display_list`, `image_destroy_all_bitmaps` - declared by
this slice - `block_input`/`unblock_input`, `check_x_display_info`, `Fcons`, `XCAR`).

### Deviations (slice 11)

- **The `#if 0` block is text, not code** (C 7291-7388), on the C 1278-1417 precedent; the file's
  `feature = "have_ntgui"` arm therefore never compiles the three window-property `DEFUN`s, exactly
  as C never does.
- **`image_destroy_all_bitmaps` is declared here** rather than in `include/dispextern.rs` (whose
  3741 prototype the mirror does not yet carry, the ledger's owed item): the declaration names
  `*mut w32_display_info`, the unit's own record for C's `Display_Info *`.
- **The name sweep's nested-comment limitation** is recorded above and in this ledger rather than
  fixed: it is a /tmp reading, not a gate, so its false positives are triaged by hand.
- **Not claimed.**  No run evidence: no connection is closed, no display list is built and no
  property is set on this machine; the arm reading is compile-level and the sweep textual.

## Slice 12 (C 7390-7661)

**Scope and the stop.**  C 7390-7392's Tool-tips banner, C 7394-7395's `compute_tip_xy` forward
declaration, the six tool-tip objects (C 7398-7415), `unwind_create_tip_frame` (C 7418-7429) and
`w32_create_tip_frame` (C 7432-7661).  The stop is C 7663, which opens `compute_tip_xy`'s
definition (C 7663-7761) - the next item.

**What the item carries.**  The six objects (C 7402's `Window tip_window;` is the one non-`static`
of them, so it carries `#[unsafe(no_mangle)]`; the five `static Lisp_Object`s are zero in C, i.e.
the port's `lisp_h_Qnil ()`); `unwind_create_tip_frame` (C's `static void` whose address C 7476
takes, so the definition carries the C ABI); and `w32_create_tip_frame`'s whole body -
C 7459's parameter copy, C 7462's name read, C 7472's `make_frame (false)`, C 7476's
`record_unwind_protect`, C 7482-7488's terminal/`output_method!`/`xzalloc`/`FRAME_FONTSET!`/
`fset_icon_name` run, C 7490-7492's `glyph_debug` arm, C 7493's `FRAME_KBOARD (f) = kb;`,
C 7495-7508's name block, C 7510-7516's three `register_font_driver` calls (`have_harfbuzz`),
C 7518-7557's twelve `gui_default_parameter` calls and the `internalBorder` block, C 7562's
`init_frame_faces`, C 7564-7566's `dwStyle`/`parent_desc`/`explicit_parent` writes,
C 7568's `gui_figure_window_size`, C 7570-7576's five fringe/divider zeros, C 7578-7582's
`my_create_tip_window`/`w32_make_gc` pair, C 7584-7600's seven more `gui_default_parameter` calls,
C 7602-7604's `tooltip` parameter default, C 7606-7628's bg/fg block (`Fframe_parameter`,
`calln` with `Qface_set_after_frame_default`), C 7634-7643's reference counts,
`Vframe_list`, `adjust_frame_size (…, Qtip_frame)`, C 7650's `face_change` restore and C 7653's
`unbind_to`.  New constant: `WS_DISABLED` (0x08000000, C 7564).

**The flag-table correction (slices 6 and 7 revisited).**  C writes five `frame.h` `bool_bf`
members directly in this file's text; the mirror carries each bit's `_UNIT`/`_SHIFT` pair and the
`frame_set_flag`/`frame_flag` device the file already used for `FRAME_UNDECORATED!`, so this slice
moves all five onto it: C 6105/6107's `explicit_name` reads and writes in `w32_window` (slice 6),
C 6608's `no_split`, C 6637's `can_set_window_size` and C 6682's `was_invisible` in
`Fx_create_frame` (slice 7), plus this slice's own C 7473 `wants_modeline`, C 7488 `tooltip`,
C 7630 `no_split` and C 7641 `can_set_window_size`.  The arm reading drops from **42 to 40**
previous errors: the five `frame`-bit sites retire and this slice's text adds one
`explicit_parent` site (the `w32_output` member the mirror still records without packing - the
same owed class as `menubar_active`/`hourglass_p`).  Three appended
`docs/ports/macro-body-notes.tsv` rows close the token rows the correction creates.

### Evidence (slice 12, the final revision: 11,509 Rust lines)

**① The frozen configuration.**  `build-tu src/w32fns.c` → exit 0, `build/femacs/w32fns.o`
**336 bytes**.  One run (plus one earlier run that read the two parse errors below).

**② The arm probe (`have_ntgui` + `windowsnt`).**  **40 previous errors** (from slice 11's 42; the
five retired sites and the one new `explicit_parent` site above).  Classes: `menubar_active` 6,
`explicit_parent` 4, `hourglass_p` 3, `tab_bar_*`/`tool_bar_*` 9, the eleven `E0428`s and
`E0530`.  One probe run after two unit-local fixes: C's banner text closed the port's own comment
at its first `*/` (`error: unknown start of token`, `expected item, found \`,\``), and the note
that quoted that delimiter did it a second time - the banner is now carried inside one wrapper
comment and the note names the delimiter in prose.

**③ `extract-abi.py --check`.**  169 rows → **162**: seven retire (`missing function
unwind_create_tip_frame`, `missing function w32_create_tip_frame`, and the five `missing
file-scope object` rows `tip_frame`, `tip_timer`, `tip_last_string`, `tip_last_frame`,
`tip_last_parms`); the correction adds four token rows
(`w32_window does not mention explicit_name`, `Fx_create_frame`/`w32_create_tip_frame`'s member
lists), all closed by the three appended notes rows - so the final count is seven rows down.  The
drift row reads **7.8%** (`C 12488 vs Rust 11509`), down from 10.3%.

**④ The range table and the dead-file gate.**  The `7390-12488` remainder row splits into
`7390-7661` (this slice; C 7394-7395 marked as the dropped forward declaration) and `7663-12488`;
`check-dead-file-coverage.py` reads **161 range row(s) cover src/w32fns.c (12488 lines; 108 blank
line(s) between blocks need no row)**, `0 failure(s)` (160 rows / 107 blanks before the split).

**⑤ The name sweep, now a repository tool.**  The reading slice 8 introduced and slice 10 used
(where it found `validate_x_resource_name`) is committed as **`tools/check-unit-names.py`** - a
triage reading, not one of the ten gates, with its two known false-positive classes (nested block
comments; macro-generated names and globals reached through a mirrored record) written into its
docstring.  Over this slice's region (`src/w32fns.rs` 11252-11509) it reads **57 capitalized
identifiers, missing: []**.

### Deviations (slice 12)

- **The five bitfield writes are the mirror's flag device**, not C's member syntax: the port's
  `frame_set_flag (f, FRAME_X_UNIT, FRAME_X_SHIFT, v)` writes the same bit in the mirror's packed
  word (PORTING.md 1.3 / P-80), and the notes rows above record the token loss.
- **C 7493's `FRAME_KBOARD (f) = kb;`** writes through frame.h 919's `((f)->terminal->kboard)`;
  the mirror's `terminal.kboard` member is the untyped `*mut c_void` (P-72 rule ③), so the write
  carries C's own type as a cast - the C 6289 reading of slice 7.
- **`explicit_parent` keeps C's spelling** (C 7566): it is a recorded-not-packed `w32_output`
  member, so the write is the 4th owed site of that name (slice 7 has three).
- **C 7394-7395's forward declaration is dropped** (Rust needs none); the range table row records it.
- **Not claimed.**  No run evidence: no tooltip frame is created here (no display, no
  `my_create_tip_window` call), the arm reading is compile-level and the name sweep textual.

## Slice 13 (C 7663-7807)

**Scope and the stop.**  `compute_tip_xy`'s definition (C 7663-7750, the item C 7394-7395's
declaration of slice 12 announced) and `w32_hide_tip` (C 7752-7806).  The stop is C 7809's
`x-show-tip` `DEFUN` (C 7808 blank).

**What the item carries.**  C 7672-7675's four `Fcdr (Fassq (…))` parameter reads; C 7682's
`POINT pt;` and C 7704's `struct MONITOR_INFO info;` (both PORTING.md 1.5 locals, zero-initialised
here); C 7687-7688's two pixel readings off `FRAME_DISPLAY_INFO`; C 7690-7694's
`block_input`/`GetCursorPos`/`unblock_input`; C 7702-7716's monitor clamp over
`monitor_from_point_fn`/`get_monitor_info_fn` (the `Option` tests of C 498-503); C 7719-7749's two
ladders (C's `*root_y = XFIXNUM (top)` narrows an `EMACS_INT` into the `int` the caller owns - the
port's `as c_int`); and `w32_hide_tip`'s timer cancel (`calln!`), its guard and its
`specbind`/`delete_frame`/`w32_make_frame_invisible`/`unbind_to` body.  Two new declarations:
`GetCursorPos` (`<winuser.h>`, C 7691) and `delete_frame` (frame.h 1509, C 7790 - the
`src/androidfns.rs` 117 spelling).

### Evidence (slice 13, the final revision: 11,681 Rust lines)

**① The frozen configuration.**  `build-tu src/w32fns.c` → exit 0, object **336 bytes**.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **40 previous errors**, unchanged from slice 12 -
these two functions add no error and no member gap (no flagged line falls in the new region).  One
run.

**③ `extract-abi.py --check`.**  162 rows → **160**: `missing function compute_tip_xy` and
`missing function w32_hide_tip` retire, no row is added by this slice, and the drift row reads
**6.5%** (`C 12488 vs Rust 11681`), down from 7.8%.

**④ The range table and the dead-file gate.**  The `7663-12488` remainder row splits into
`7663-7807` (this slice) and `7809-12488`; `check-dead-file-coverage.py` reads **162 range row(s)
cover src/w32fns.c (12488 lines; 109 blank line(s) between blocks need no row)**, `0 failure(s)`
(161 rows / 108 blanks before the split).  The ten gates read 0 failures.

**⑤ The name sweep.**  `python3 tools/check-unit-names.py femacs/src/w32fns.rs 11509 11681` reads
`missing: []` (`GetCursorPos` and `delete_frame` are declared by this slice itself).

### Deviations (slice 13)

- **The two locals are PORTING.md 1.5 outputs-only locals** (`POINT pt;` C 7682,
  `struct MONITOR_INFO info;` C 7704): zero-initialised here, as the ledger names.
- **`XFIXNUM`'s `EMACS_INT` narrows to `c_int` explicitly** where C relies on the implicit
  conversion into `*root_x`/`*root_y` (C 7720 etc.).
- **Not claimed.**  No run evidence: no cursor position is read and no tooltip is hidden on this
  machine; the arm reading is compile-level and the name sweep textual.

## Slice 14 (C 7809-8114)

**Scope and the stop.**  The file's second-largest `DEFUN`, `x-show-tip` (C 7809-8106), and
`x-hide-tip` (C 8109-8114).  The stop is C 8116's File-selection-dialog banner (C 8115 blank).

**What the item carries.**  C's six-argument shape with the six parameters re-bound `let mut` (the
`DEFUN!` macro's `$arg:ident` cannot say `mut`; the `src/androidfns.rs` 3028-3037 note and that
file's identical C lines are the sibling); C 7884's `goto start_timer` and C 8101's label as the
`'start_timer` labelled block whose `break 'start_timer` is the jump and which *stops before* the
timer line exactly as C's label does; C 7847-7885's reuse arm with `Fequal_including_properties`,
`compute_tip_xy` and the two `SetWindowPos` calls (`HWND_TOPMOST`/`HWND_TOP` and the four `SWP_*`
flags); C 7886-7937's hidden-frame comparison (two `CONSP` walks with `calln! (Qassq_delete_all,
…)`) and its three `w32_hide_tip` exits; C 7942-7980's parameter defaults and
`w32_create_tip_frame` call; C 7982-8020's pseudo-window setup (`bset_left_margin_cols`,
`bset_right_margin_cols`, `set_window_buffer`, `Fset_window_parameter (Qno_other_window)`, the
`Vx_max_tooltip_size` clamp with `RANGED_FIXNUMP`, `FRAME_TOTAL_COLS!`, `adjust_frame_glyphs`);
C 8024-8099's buffer insertion (`set_buffer_internal_1`, `bset_truncate_lines`, three `specbind`s,
`Ferase_buffer`, `Finsert (1, &string)`, `SET_TEXT_POS!`, `try_window` with
dispextern.h 3700's `TRY_WINDOW_IGNORE_FONTS_CHANGE`, C 8039-8051's `enable_checking` arm,
`Fwindow_text_pixel_size`, `compute_tip_xy`, `AdjustWindowRect` with `FRAME_EXTERNAL_MENU_BAR`
read as the mirror's `bool` function, `SET_FRAME_VISIBLE!`, `ShowWindow`, `update_single_window`,
`windows_or_buffers_changed`) and C 8101-8105's timer + `unbind_to`.  Two new declarations
(`try_window`, `update_single_window` - dispextern.h 3573/3918) and two unit-local macros
(`FRAME_TOTAL_COLS!` for the frame.h 1035 writer, `SET_TEXT_POS!` for dispextern.h 291-292's
two-assignment expansion; the `src/androidfns.rs` 2963-2974 / `src/haikufns.rs` 449-454 device).

### Evidence (slice 14, the final revision: 12,064 Rust lines)

**① The frozen configuration.**  `build-tu src/w32fns.c` → exit 0.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **41 previous errors**, one more than slice 13's
40: this slice's two new members of the `window` mirror - C 7991's `pseudo_window_p` and C 8094's
`must_be_updated_p`, neither recorded by `include/window.rs` (the `w32_output`/`frame` class, owed
to the mirror).  The other classes are unchanged (eleven `E0428`s including the new
`tip_window`/`windowsnt`-mirror collision this slice's C 7402 definition creates,
`menubar_active` 6, `explicit_parent` 4, `hourglass_p` 3, `tab_bar_*`/`tool_bar_*` 9, `E0530`).
Six unit-local fixes were needed first, all found by this reading: C 8032's `Finsert` argument
shape, the `'start_timer` block's position (C's label is *before* the timer line, so the block must
not be the body's tail expression - `E0308 expected *mut Lisp_X, found ()`), C 7886's
`tooltip_reuse_hidden_frame` is a `bool` in the mirror (so `&&`, not `!= 0`; C 8113's `!` likewise),
C 8033-8034's two matrices are `*mut c_void` in the window mirror (`as *mut glyph_matrix`), and
C 6104-6107's `int explicit` needs `explicit != 0` at `w32_set_name` (a type error slice 12's
flag-table correction introduced and this slice's reading caught).

**③ `extract-abi.py --check`.**  160 rows → **157**: the two `missing subr` rows
(`Fx_show_tip`, `Fx_hide_tip`) retire, and the **line-count drift row disappears** - the target is
now 12,064 lines against the reference's 12,488, inside the checker's 5% window (the last reading
was 6.5%).  This is the first time this file has no drift row.

**④ The range table and the dead-file gate.**  The `7809-12488` remainder row splits into
`7809-8114` (this slice; C 7884/8101's `goto` recorded) and `8116-12488`;
`check-dead-file-coverage.py` reads **163 range row(s) cover src/w32fns.c (12488 lines; 110 blank
line(s) between blocks need no row)**, `0 failure(s)` (162 rows / 109 blanks before the split).

**⑤ The name sweep, and `check-citations`.**  `tools/check-unit-names.py` over the new region:
`missing: []`.  `check-citations.py` first read **1 drift** - this slice's own doc comment cited
`TRY_WINDOW_IGNORE_FONTS_CHANGE` as dispextern.h 2761 (the *mirror's* line) instead of the
reference's dispextern.h 3700 - and reads 0 drift after the fix.  The ten gates read 0 failures.

### Deviations (slice 14)

- **Two `window` mirror members are owed** (`pseudo_window_p`, `must_be_updated_p`): C's writes are
  kept verbatim and the arm reading names them, rather than inventing a unit-local bit device
  (`src/androidfns.rs` 2955-2962 has one for the same two bits, but the mirror carries neither the
  members nor the shift constants this file would need).
- **`tip_window`'s definition collides with the mirror's declaration** (`include/w32term.rs` 1315):
  the port keeps C's non-static definition (C 7402 does define it here) and the `E0428` is the
  integrator's - drop the mirror's declaration when this unit lands (the class slice 7's ledger
  names for the eleven other names).
- **`FRAME_TOTAL_COLS!`/`SET_TEXT_POS!` are unit-local**, because the mirror carries the frame.h
  1035 reader and a function-shaped `SET_TEXT_POS` while C writes both (the two sibling units'
  device, cited above).
- **Not claimed.**  No run evidence: no tooltip is shown on this machine (no display,
  `SetWindowPos` is never called), the arm reading is compile-level and the name sweep textual.

## Slice 15 (C 8116-8262)

**Scope and the stop.**  C 8116-8118's File-selection-dialog banner, C 8120-8122's three
`#define FILE_NAME_*`, `file_dialog_callback` (C 8124-8221) and `w32_dialog_in_progress`
(C 8223-8262).  The stop is C 8264's `x-file-dialog` `DEFUN` (C 8263 blank) - the next item and
the file's third-largest.

**What the item carries.**  The callback's two `OFNOTIFY` views of C's `LPARAM` (C 8134-8135),
C 8138-8142's `#ifdef NTGUI_UNICODE` pair (the `cygwin` feature names C's condition: `conf_post.h`
169-170 defines it only for `CYGWIN && HAVE_NTGUI`, so the `#else` arm is live and both stand),
the two `dropdown_changed` expressions (C's `int` from a `||`, the port's `as c_int`), the
Windows-7 fallback walk (C 8164-8173's `GetDlgItem`/`GetWindow`/`GW_CHILD` chain), the two
`SendMessageW`/`SendMessageA` pairs with C's two **wide** literals (`L"Current Directory"`,
`L""` - carried as `[u16]` arrays, the `DARK_MODE_APP_NAME` device at the top of this file) and
the narrow twins (`c"…"`), `EnableWindow`/`SetFocus`, and `w32_dialog_in_progress`'s critical
section (`enter_crit`/`leave_crit`, C 8248's `bool` global) and `FOR_EACH_FRAME` z-group walk
(`FRAME_Z_GROUP_ABOVE`/`_SUSPENDED` as the mirror's `bool` readers, `w32_set_z_group` as this
unit's own).

**New SDK surface.**  The three `<dlgs.h>` ids C's `#define`s rename (`edt1` 0x0480, `cmb13`
0x047C, `lst1` 0x0460 - kept under C's `FILE_NAME_*` names); `WM_NOTIFY`, `GW_CHILD`; the five
`<commdlg.h>` ids (`CDN_FIRST` = 0xFFFFFDA7, `CDN_INITDONE`, `CDN_TYPECHANGE`, `CDM_FIRST`,
`CDM_SETCONTROLTEXT`); the `NMHDR`, `OPENFILENAMEA`, `OPENFILENAMEW`, `OFNOTIFYA`, `OFNOTIFYW`
records and the `LPOFNHOOKPROC` pointer typedef (the full records - C 8264's `x-file-dialog` needs
their members next, and `lpOFN->nFilterIndex` is the one member this slice reads); and six
functions (`GetParent`, `GetDlgItem`, `GetWindow`, `SendMessageW`, `SendMessageA`,
`EnableWindow`).

### Evidence (slice 15, the final revision: 12,394 Rust lines)

**① The frozen configuration.**  `build-tu src/w32fns.c` → exit 0.  One run (its first reading
caught the banner's nested comment and the `lpOFN` deref below).

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors**, one more than slice
14's 41: this slice's `w32_dialog_in_progress` definition collides with the mirror's declaration
(`include/w32term.rs`, the same `E0428` class as `tip_window` - an integrator item, not a defect of
the text).  Outside the `E0609`/`E0428` classes only the pre-existing `E0530` remains, so this
slice adds no member gap and no type error after two unit-local fixes: C's banner text closed the
port's own comment at its first `*/` (the slice-12 lesson, applied here as a wrapper comment), and
`(*notify_w).lpOFN.nFilterIndex` needed C's own deref-of-a-pointer-member spelling
(`(*(*notify_w).lpOFN).nFilterIndex`).  Two probe runs.

**③ `extract-abi.py --check`.**  157 rows → **155**: `missing function file_dialog_callback` and
`missing function w32_dialog_in_progress` retire, no row is added, and the file still has no
line-count drift row (12,394 lines against 12,488).

**④ The range table and the dead-file gate.**  The `8116-12488` remainder row splits into
`8116-8262` (this slice; its `W`/`A` arms marked) and `8264-12488`;
`check-dead-file-coverage.py` reads **164 range row(s) cover src/w32fns.c (12488 lines; 111 blank
line(s) between blocks need no row)**, `0 failure(s)`; the ten gates read 0 failures.

**⑤ The name sweep.**  `tools/check-unit-names.py` reads 50 capitalized identifiers with five
"missing" - `Clone`, `Copy` (a `#[derive]`'s arguments), `Flags`, `FlagsEx` (a struct definition's
field names) and `Msg` (C's own parameter name in a declaration).  All five are the tool's known
false-positive classes; its docstring now names them (the two earlier classes were nested block
comments and macro-generated names).

### Deviations (slice 15)

- **The `OFNOTIFY`/`OPENFILENAME` records are declared in full**, not as the four members this
  slice reads: C 8264's `x-file-dialog` reads `lpstrFile`, `nMaxFile`, `Flags` and the rest, so the
  records are the file's own (the `--check` reading has no row for them).
- **C's two wide literals are `[u16]` arrays** (`CURRENT_DIRECTORY_L`, `EMPTY_L`), because Rust has
  no `L"…"` syntax; C's narrow twins stay `c"…"`.
- **`w32_dialog_in_progress` collides with the mirror's declaration** (`E0428`, above): the port
  keeps C's non-static definition.
- **Not claimed.**  No run evidence: no dialog is opened on this machine and the callback is never
  entered; the arm reading is compile-level and the name sweep textual.

## Slice 16 (C 8264-8554)

**Scope and the stop.**  The whole `x-file-dialog` `DEFUN` (C 8264-8554) - the file's third-largest
item.  The stop is C 8556's `#ifdef WINDOWSNT` (C 8555 blank) and `system-move-file-to-trash`
(C 8562), the next item.

**What the item carries.**  C 8269-8271's two filter strings (the wide one as a `[c_ushort]` array,
the narrow as `[c_char]`); C 8274-8278's five locals; C 8284-8305's two **anonymous wrapper
structs** (`new_file_details_w_s` / `new_file_details_a_s`, C's `#if _WIN32_WINNT < 0x500` arm
under the feature `win32_nt_lt_500` that names C's condition - the file head's
`win32_windows_ge_0410` convention, both arms text because `check-cond-eval.py` cannot decide the
value); C 8293-8296's `NTGUI_UNICODE` declaration arm and C 8297-8315's `#else` arm, both carried
(`cygwin` names C's condition); C 8317-8411's conversion block, including C 8333-8337's `CYGWIN`
arm (`Fcygwin_convert_file_name_to_windows`), C 8350-8357's `NTGUI_UNICODE` arm (`to_unicode`,
`memcpy`) and the live arm's `ENCODE_FILE`/`Fcopy_sequence`/`unixtodos_filename` chain,
`filename_to_utf16`/`filename_to_ansi`, and C 8384/8402/8408's three **bare `alloca`s** - the
port's `sa_stack_alloc` (PORTING.md P-05; the `src/w32menu.rs` 1005 reading) - feeding the
`.unwrap ()`-called `pMultiByteToWideChar`/`pWideCharToMultiByte`; C 8413-8478's two record-filling
arms (`memset`, C 8422/8456's `w32_major_version` size choice, `countof!`, the six `OFN_*` flags);
C 8480-8505's dialog call (`w32_dialog_in_progress` as both the pre-call and the unwind function,
two `specbind`s, `GetOpenFileNameW`/`A` bound to `file_dialog_callback`); and C 8507-8553's result
handling (C 8510-8511's `NTGUI_UNICODE` arm with `from_unicode_buffer`, C 8512-8519's live arm with
`filename_from_utf16`/`filename_from_ansi` + `dostounix_filename` + `DECODE_FILE`, C 8521-8523's
`CYGWIN` arm, the C 8527-8532 directory strip, C 8535's `CommDlgExtendedError` and C 8539-8546's
eight-argument `Fcompleting_read` fallback).

**New surface.**  `CP_ACP` (winnls.h), the six `OFN_*` flags (commdlg.h), `GetOpenFileNameW`,
`GetOpenFileNameA`, `CommDlgExtendedError`, `to_unicode`/`from_unicode_buffer` (coding.h 728/737 -
the `include/coding.rs` mirror does not carry them yet, so the slice declares them), the two
wrapper records and the two filter arrays.  Slice 15's `FILE_NAME_*` constants gained the
`edt1`/`cmb13`/`lst1` ids they alias, because C's macro bodies *are* those tokens and
`extract-abi.py` compares a macro's body.

### Evidence (slice 16, the final revision: 12,835 Rust lines)

**① The frozen configuration.**  `build-tu src/w32fns.c` → exit 0.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - the same count slice 15
left, i.e. this DEFUN adds no error and no member gap once four rounds of fixes are in (all of them
found by this reading): `countof (x)` had to be the mirror's macro `countof! (x)`; C's `errno` is
the port's `*errno ()`; the five lazily bound code-page calls need `.unwrap ()` (the mirror's
`Option` typedefs); `SBYTES` returns `isize`, so `MAX_UTF8_PATH as isize`; `multiByteToWideCharFlags`
is the mirror's `c_ulong` and must be passed bare (the proc's second parameter is `c_ulong`, not
`DWORD`); `pWideCharToMultiByte`'s two trailing arguments are `*mut BOOL`, not `null`; and
`use_unicode` is a `c_int`, so C's `use_unicode &&` is `use_unicode != 0 &&`.  Five probe runs.

**③ `extract-abi.py --check`.**  155 rows → **151** and **no token or family row** for this file:
the `missing subr Fx_file_dialog` row retires; the 3 macro rows slice 15 had created
(`FILE_NAME_*` vs `edt1`/`cmb13`/`lst1`) close when the three header ids become the aliased
constants; the `memcpy` and `wchar_t` family rows close when C 8350-8357's arm and the `c_ushort`
spellings are carried; and the one remaining token row (six tokens: `alloca`, `PVOID`,
`pvReserved`, `dwReserved`, `FlagsEx`, `uninit`) is closed by one appended
`macro-body-notes.tsv` row that names why each cannot reappear.  The file still has no
line-count drift row.

**④ The range table and the dead-file gate.**  The `8264-12488` remainder row splits into
`8264-8554` (this slice) and `8556-12488`; `check-dead-file-coverage.py` reads **165 range row(s)
cover src/w32fns.c (12488 lines; 112 blank line(s) between blocks need no row)**, `0 failure(s)`.

**⑤ `check-mirror-dupes.py` fired once - and that is the reading working.**  The slice's first
draft declared `Fcygwin_convert_file_name_to_windows`/`_from_windows` itself; the gate reported
both as *already declared by `include/globals.rs`*, so the two lines came out and the gate reads 0
failures (the C 8334/8336 text now points at the mirror's declarations).  The name sweep over the
new region reads `missing: []` after triage (its five candidates are the tool's known
false-positive classes: `FlagsEx`, `Some`, `Sx_file_dialog`).

### Deviations (slice 16)

- **The three bare `alloca`s are `sa_stack_alloc`** (PORTING.md P-05; the `src/w32menu.rs` 1005
  reading), which is why the `alloca` token is recorded in the notes row rather than spelled.
- **C's `wchar_t` is the mirror's `c_ushort`** everywhere this slice names it (the token family
  `tools/extract-abi.py` keeps); C 8435's `guichar_t` cast stays `guichar_t`.
- **The two anonymous wrapper structs live at file scope**, named after the variables they
  initialise (`new_file_details_w_s`/`new_file_details_a_s`); their members are therefore outside
  the body the token check reads, which the notes row records.
- **C 8312's `UNINIT` attribute has no Rust counterpart** (the attribute is GCC's); the declaration
  carries a comment naming it.
- **Not claimed.**  No run evidence: no file dialog is opened on this machine (no Windows, no
  `GetOpenFileName*`), the callback is never entered, and the arm reading is compile-level.

## Slice 17 (C 8556-8667)

**Scope and the stop.**  C's `#ifdef WINDOWSNT` block: the lazily bound `pfnSHFileOperationW`
(C 8558) and the `system-move-file-to-trash` `DEFUN` (C 8562-8660), then C 8662's `#endif` and
C 8664-8667's "w32 specialized functions" banner.  The stop is C 8669's `w32-send-sys-command`
`DEFUN` - the next item.

**What the item carries.**  C 8572's `maybe_load_unicows_dll` (`w32.h` 176, declared by
`include/w32.rs`); C 8574-8579's directory test (`Ffile_directory_p`, `Ffile_symlink_p`,
`Fdirectory_file_name`, the two `Qdelete_*` symbols); C 8584's `Fexpand_file_name` and C 8586-8588's
`Ffind_file_name_handler` + `calln!`; C 8594-8596's `ENCODE_FILE` + `map_w32_filename`; and
C 8600-8655's two arms - the `W` one over `SHFILEOPSTRUCTW` (`filename_to_utf16`, `_wchmod` with
C 8612's `0666`, the five-flag OR, `eassert!` on the `Option`, and the `pfnSHFileOperationW`
call) and the `A` one over `SHFILEOPSTRUCTA` (`_mbspbrk`'s `?*` guard, `_chmod`,
`SHFileOperationA`) - closing with C 8656's `report_file_error`.  New surface: the
`PFNSHFILEOPERATIONW` type and its `static mut` storage, `FILEOP_FLAGS`, the two `SHFILEOPSTRUCT`
records, `FO_DELETE`, four `FOF_*` flags, `ERROR_FILE_NOT_FOUND`, and the three CRT entries
(`_wchmod`, `_chmod`, `_mbspbrk`) plus `SHFileOperationA`.

### Evidence (slice 17, the final revision: 13,042 Rust lines)

**① The frozen configuration.**  `build-tu src/w32fns.c` → exit 0.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - the count slice 16
left, so this block adds no error and no member gap once two fixes are in, both found by this
reading: `FOF_NO_CONNECTED_ELEMENTS` was **already declared** at the top of this file (C 154's
`#ifndef` guard) as a `DWORD`, so the slice's own declaration came out and the two OR sites cast
that one flag down to `FILEOP_FLAGS` (`u16`) - the module-level duplicate was visible as both an
`E0428` and the `E0277`/`E0308` pair the header's narrower type produced.  One probe run for each
of the two readings.

**③ `extract-abi.py --check`.**  151 rows → **150**: `missing subr Fsystem_move_file_to_trash`
retires and no row is added; no token or family row appears for this block, and the file still has
no line-count drift row.

**④ The range table and the dead-file gate.**  The `8556-12488` remainder row splits into
`8556-8667` (this slice, with C 8662's `#endif` explicitly recorded as having no Rust counterpart -
the items carry the `windowsnt` feature themselves) and `8669-12488`;
`check-dead-file-coverage.py` reads **166 range row(s) cover src/w32fns.c (12488 lines; 113 blank
line(s) between blocks need no row)**, `0 failure(s)`.  Ten gates read 0 failures; the name sweep
over the new region reads `missing: []` after triage (its one candidate is `DEFUN!`'s own
`Ssystem_move_file_to_trash` static).

### Deviations (slice 17)

- **`FOF_NO_CONNECTED_ELEMENTS` keeps the file's earlier `DWORD` spelling** (C 154's guard block
  landed long before this slice); only the two OR sites narrow it, which is where C's `int`-valued
  macro meets the `WORD` member.
- **The `0666` arguments are `0o666`**, the port's spelling for C's octal literals.
- **`pfnSHFileOperationW` is an `Option<PFNSHFILEOPERATIONW>` static**, so C 8624's
  `eassert (pfnSHFileOperationW)` is `is_some ()`; C 12466 (`globals_of_w32fns`) is what assigns it.
- **C 8662's `#endif` and C 8664-8667's banner are carried in C's order** (the banner inside one
  wrapper comment, the `#endif` recorded in the range table rather than spelled).
- **Not claimed.**  No run evidence: nothing is moved to the recycle bin on this machine
  (`SHFileOperationA/W` are never called), the arm reading is compile-level.

## Slice 18 (C 8669-8692)

**Scope and the stop.**  C 8669-8688's `w32-send-sys-command` - the first `DEFUN` of the "w32
specialized functions" block - and C 8690-8692's `#ifndef CYGWIN` with C 8691's
`pfnShellExecuteExW` declaration.  The stop is C 8694's `w32-shell-execute` `DEFUN` (C 8693 blank).

**What the item carries.**  C 8680's `decode_window_system_frame`, C 8682's `CHECK_FIXNUM`,
C 8684-8685's `FRAME_W32_P` test and `PostMessage` (already declared above as `<winuser.h>`'s)
with C's `XFIXNUM (command)` narrowed into the `WPARAM` parameter; and C 8691's pointer, whose
storage is the `Option<PFNSHELLEXECUTEEXW>` static `globals_of_w32fns` assigns.  New surface:
`WM_SYSCOMMAND` (0x0112), `PFNSHELLEXECUTEEXW`, and the `<shellapi.h>` `SHELLEXECUTEINFOW` record -
declared in full, with C's `DUMMYUNIONNAME` as the Rust union `SHELLEXECUTEINFOW_union`, because
C 8694's `w32-shell-execute` fills every member next.

### Evidence (slice 18, the final revision: 13,120 Rust lines)

**① The frozen configuration.**  `build-tu src/w32fns.c` → exit 0.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - the same count slices
16 and 17 left, with **no unit-local fix needed at all** this time: the declaration, the record and
the DEFUN all resolved on the first reading.  One probe run.

**③ `extract-abi.py --check`.**  150 rows → **149**, with one swap inside the count:
`missing subr Fw32_send_sys_command` retires, and the file's **first `line count drifted` row**
appears (5.1%: C 12,488 vs Rust 13,120 - the port is now longer than the C file).  That row is
closed the documented way, by a new `docs/ports/line-drift-tolerance.tsv` row for `w32fns`
(tolerance **0.35**, basis **total**) whose reason names what the extra lines are: both arms of
every `#if` (the file gate plus the six nested features), the per-item `/// C:` doc comments, the
per-slice declaration blocks with one citation per prototype, the `[repr(C)]` records and the
`[u16]`/`[c_char]` arrays C spells as literals.  The check uses `max (0.05, tolerance)`, so the row
also covers the growth the file's remaining 3,800 C lines will bring.

**④ The range table and the dead-file gate.**  The `8694-12488` remainder row's predecessor
splits into `8669-8692` (this slice, with C 8692's `#endif` recorded as having no counterpart - the
two attributes carry it) and `8694-12488`; `check-dead-file-coverage.py` reads **167 range row(s)
cover src/w32fns.c (12488 lines; 114 blank line(s) between blocks need no row)**, `0 failure(s)`.
Ten gates read 0 failures, and the name sweep reads `missing: []` after triage (`Clone`/`Copy` are
the derive arguments, `Sw32_send_sys_command` is `DEFUN!`'s own static).

### Deviations (slice 18)

- **`SHELLEXECUTEINFOW` is declared ahead of its only user** (C 8694's `w32-shell-execute`, the next
  slice): C declares the record in `<shellapi.h>`, and this file's text needs the *type* here for
  C 8691's pointer, so the record lands with that declaration rather than with its first write.
- **C's anonymous union is a Rust `union`** (`SHELLEXECUTEINFOW_union`, both names kept).
- **The file's line-count tolerance row is new** (`docs/ports/line-drift-tolerance.tsv`, 0.35/total):
  the port's target crossed the checker's 5% window at this slice, and the row's reason is what the
  extra lines are made of.
- **Not claimed.**  No run evidence: no `WM_SYSCOMMAND` message is posted on this machine, and
  `pfnShellExecuteExW` is never called; the arm reading is compile-level.

## Slice 19 (C 8694-9014)

**Scope and the stop.**  The whole `w32-shell-execute` `DEFUN`: C 8695-8759's 65-line docstring,
C 8765-8836's `CYGWIN` arm and C 8838-9002's live arm, closing with C 9005-9013's shared tail.  The
stop is C 9016's `lookup_vk_code` comment (C 9019 the function).

**What the item carries.**  The docstring is **generated from C's own text** through
`cinventory.doc_comment_value` (make-docfile's rule: the comment's leading/trailing whitespace
drops, escapes resolve), because C's doc has trailing spaces *inside* the text that a hand-written
literal lost - the checker's `subr … docstring differs` row is what caught that, and the second
generation matched.  C 8763's `BVAR! (current_buffer! (), directory_)`; C 8776-8836's `CYGWIN` arm
(`GUI_ENCODE_FILE`/`GUI_ENCODE_SYSTEM`/`GUI_SDATA` from `include/w32term.rs`, `ShellExecuteW`, the
ten `SE_ERR_*` cases as a `match`, `w32_strerror`); and C 8838-9002's live arm - C 8840's
`c"file:///"` with C 8841's length constant, C 8863's `IS_DIRECTORY_SEP` (a unit-local macro) with
`c_isalpha`, C 8874-8896's absolute-name probe (`Fexpand_file_name`,
`Ffind_file_name_handler (…, Qfile_exists_p)`, `Fcopy_sequence (ENCODE_FILE (…))`, `faccessat` with
`AT_FDCWD`/`F_OK`/`AT_EACCESS`, `unixtodos_filename`), C 8908-8915's `pMultiByteToWideChar` +
`xmalloc` pass, C 8916-8960's `SHELLEXECUTEINFOW` fill (`filename_to_utf16`, the two
`sa_stack_alloc` buffers where C has `alloca`, C 8932-8943's ASCII-only `ops_w` loop,
`pfnShellExecuteExW`), and C 8961-8996's `SHELLEXECUTEINFOA` twin (`codepage_for_filenames`,
`pWideCharToMultiByte`, `filename_to_ansi`, `ShellExecuteExA`).  New surface: `SHELLEXECUTEINFOA`
(+ its union), the three `SEE_MASK_*`, `SW_SHOWDEFAULT`, the ten `SE_ERR_*`, the eight
`ERROR_*` codes, `ShellExecuteExA`, `ShellExecuteW` (the `cygwin` arm), `faccessat` and the
`IS_DIRECTORY_SEP!` macro.

### Evidence (slice 19, the final revision: 13,516 Rust lines)

**① The frozen configuration.**  `build-tu src/w32fns.c` → exit 0.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - the count slices 16-18
left, so this DEFUN adds no error and no member gap once four fixes are in, every one of them found
by this reading: `buffer.directory` is the mirror's **`directory_`** (the C-name-with-underscore
convention `xdisp.rs` 28808 already spells); `c_isalpha` returns `bool` in the port, so C's
`!= 0` test comes out; `code_convert_string_norecord`'s third parameter is `bool` (C's `0`); and
C 8937's `*s++` needs `let mut s`.  Five probe runs.

**③ `extract-abi.py --check`.**  149 rows → **148**, and the three changes are all this item's:
`missing subr Fw32_shell_execute` retires; the `docstring differs` row arrives and leaves when the
literal is regenerated from C's text through make-docfile's rule; and the six-token row
(`alloca`, `break`, `case`, `default`, `file_url_str`, `switch`) is closed by one appended
`macro-body-notes.tsv` row that names each substitution (the two `alloca`s are `sa_stack_alloc`,
C's `switch` is a `match`, and the function-local `file_url_str[]` is the `c"file:///"` spelling).

**④ The range table and the dead-file gate.**  The `8694-12488` remainder row splits into
`8694-9014` (this slice) and `9016-12488`; `check-dead-file-coverage.py` reads **168 range row(s)
cover src/w32fns.c (12488 lines; 115 blank line(s) between blocks need no row)**, `0 failure(s)`.
Ten gates read 0 failures; the name sweep reads `missing: []` after triage (`Clone`/`Copy` are the
derive arguments, `Sw32_shell_execute` is `DEFUN!`'s static).

### Deviations (slice 19)

- **The docstring is machine-generated from C's comment** (make-docfile's rule), not transcribed:
  C's doc carries trailing spaces inside the text (`…the default.  `), which the digest checks.
- **The two `alloca`s are `sa_stack_alloc`** (P-05) and C's `switch` is a `match` - both named in
  the notes row rather than spelled.
- **`IS_DIRECTORY_SEP!` is unit-local** (`include/buffer.rs` does not carry buffer.h's macro).
- **`faccessat` is declared here** (`lib/faccessat.rs` is the gnulib replacement's home).
- **Not claimed.**  No run evidence: nothing is shell-executed on this machine
  (`ShellExecuteExA`/`pfnShellExecuteExW` are never called), the arm reading is compile-level.

## Slice 20 (C 9016-9145)

**Scope and the stop.**  The hot-key prelude: `lookup_vk_code` (C 9016-9043) and
`w32_parse_and_hook_hot_key` (C 9045-9145).  The stop is C 9147's `w32-register-hot-key` `DEFUN`
(C 9146 blank).

**What the item carries.**  `lookup_vk_code`'s `for` over `lispy_function_keys` (the mirror's
`[*const c_char; 256]`) as the file's `while` shape, then C 9029-9040's `w32_kbdhook_active! ()`
block with C's own `'A'`/`'Z'`/`'0'`/`'9'`/`'a'`/`'z'` comparisons and `c_toupper`
(`lib/include/c-ctype.rs`); and `w32_parse_and_hook_hot_key`'s six locals (C's `register` keyword
has no counterpart), the `CHECK_VECTOR`/`ASIZE`/`AREF` chain with `lucid_event_type_list_p`
(keyboard.h 497) and `Fevent_convert_list`, the symbol-versus-fixnum arms (`parse_modifiers`,
keyboard.h 461; `SYMBOL_NAME`; `VK_ANY`; `CHARACTERBITS` - C's `& ~CHARACTERBITS` is `& ! …`
because Rust has no `~`), C 9096-9098's `alt_modifier` rule, and C 9108-9144's two
`w32_kbdhook_active! ()` arms - the first over `hook_w32_key` with `VK_MENU`/`VK_LWIN`/`VK_RWIN`
and the four modifier symbols, the second building C's `w32_modifiers` and returning
`HOTKEY! (…)`.  New surface: `parse_modifiers` and `lucid_event_type_list_p` (both declared here
because `include/keyboard.rs` does not carry them yet) and C 9101-9106's four `MOD_*` bits.

### Evidence (slice 20, the final revision: 13,682 Rust lines)

**① The frozen configuration.**  `build-tu src/w32fns.c` → exit 0.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - the count slices 16-19
left, so these two functions add no error and no member gap once one fix is in: `HOTKEY!`'s
expansion calls `make_fixnum`, whose parameter is `EMACS_INT`, so the two `c_int` arguments are
widened at the call site (`as EMACS_INT`) - the macro is C's `((vk) & 255) | ((mods) << 8)` and the
mirror's expansion cannot do that itself.  Two probe runs.

**③ `extract-abi.py --check`.**  148 rows → **138**: ten rows retire and none is added - the two
`missing function` rows (`lookup_vk_code`, `w32_parse_and_hook_hot_key`) and, because C 9101-9106
is the file's `#ifndef` guard block, the **four `missing macro` and the four `missing constant`
rows** for `MOD_ALT`/`MOD_CONTROL`/`MOD_SHIFT`/`MOD_WIN` (the checker looks for both spellings, and
one constant satisfies each pair).

**④ The range table and the dead-file gate.**  The `9016-12488` remainder row splits into
`9016-9145` (this slice) and `9147-12488`; `check-dead-file-coverage.py` reads **169 range row(s)
cover src/w32fns.c (12488 lines; 116 blank line(s) between blocks need no row)**, `0 failure(s)`.
Ten gates read 0 failures; the name sweep leaves one candidate, the character literal `b'A'`.

### Deviations (slice 20)

- **The four `MOD_*` bits are constants with C's values and no `cfg`** (C's `#ifndef` guard block):
  the file's standing reading of an SDK "define it if the header has not" guard.
- **C's `register` keyword has no counterpart** (the locals are plain `let mut`).
- **`HOTKEY!`'s arguments are widened at the call site** (`make_fixnum`'s `EMACS_INT`), because the
  macro's text is C's.
- **`parse_modifiers`/`lucid_event_type_list_p` are declared here** (keyboard.h 461/497, which
  `include/keyboard.rs` does not carry yet).
- **Not claimed.**  No run evidence: no hot key is registered on this machine
  (`hook_w32_key` is never called), the arm reading is compile-level and the name sweep textual.

## Slice 21 (C 9147-9337 + C 10112-10138)

**Scope and the stop.**  The hot-key family: `w32-register-hot-key` (C 9147-9198),
`w32-unregister-hot-key` (C 9200-9235), `w32-registered-hot-keys` (C 9237-9243),
`w32-reconstruct-hot-key` (C 9245-9275), `w32-toggle-lock-key` (C 9277-9316) and
`w32-window-exists-p` (C 9318-9337).  The stop is C 9339's `w32-frame-geometry` `DEFUN`
(C 9338 blank).  The slice also lands **C 10112-10138's `w32_console_toggle_lock_key`**, 1,775 C
lines ahead of the region it belongs to, because C 9302 calls it and Rust has no
declare-then-define pair inside one file (PORTING.md 0.1-12 / P-91 ①: land whichever the slice
needs); the `## C line ranges` table therefore carries two landed rows around the remainder, and
later slices of this lane insert their text **above** that block, so the file's item order still
equals C's (P-91 ② - Rust items are order-independent).

**What the item carries.**  The six `DEFUN`s in C's order, each with `make-docfile`'s rule turning
C's `doc: /* … */` into the `doc:` literal (`cinventory.doc_comment_value`, the same call the
`Fw32_shell_execute` docstring used), and C's own argument list.  `w32-register-hot-key` reads
C 9179-9180's macro/`NILP`/`Fmemq` test through the file's `w32_kbdhook_active! ()` mirror, C 9183's
`Fmemq (Qnil, w32_grabbed_keys)` slot search over the file's own slot list (C 3567), C 9186-9189's
`Fcons`-or-`XSETCAR` branch (`include/lisp.rs` 892) and C 9193-9194's
`PostThreadMessage (dwWindowsThreadId, WM_EMACS_REGISTER_HOT_KEY, …)`.  `w32-unregister-hot-key` is
C 9207-9208's parse, C 9219's `eassert (CONSP (item))` and C 9222's
`XUNTAG (item, Lisp_Cons, struct Lisp_Cons)` into the `LPARAM` the input thread reads as the list
tail (the `XUNTAG!` macro at `include/lisp.rs` 508, the `src/alloc.rs` 3074 precedent).  The other
four are C 9242's one `Fdelq`/`Fcopy_sequence` expression, C 9254-9272's
`HOTKEY_VK_CODE`/`HOTKEY_MODIFIERS` unpacking plus C's four `Fcons` layers (shift, control,
alt-or-meta by `Vw32_alt_is_meta`, hyper for `MOD_WIN`), C 9292-9313's `EQ` chain over
`Qcapslock`/`Qkp_numlock`/`Qscroll` with `VK_CAPITAL`/`VK_NUMLOCK`/`VK_SCROLL` and the
console-thread handoff, and C 9327-9336's two guards and handle test.  New surface: `FindWindowA`,
declared next to the `FindWindowW` C 3115 already uses (the file's A/W device, see Deviations).

### Evidence (slice 21, the final revision: 13,959 Rust lines)

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w21lane cargo run -q -p xtask -- build-tu src/w32fns.c` →
`built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, the object **336 bytes**.  One run.
The file gate covers every item, so the frozen configuration compiles none of this slice's text.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - the count slices 16-20
left, so the slice's 277 lines add no error and no member gap: no diagnostic sits at or beyond the
slice's first line (`src/w32fns.rs` 13,683), and the three error kinds are the standing mirror set
(15 `E0428` name collisions with the mirrors, 1 `E0530` shadowing, 25 `E0609` missing members, plus the file's one deliberately-failing `HAVE_NTGUI` frame-flag probe).
Two probe runs (the pre-write baseline and the slice's own).

**③ `extract-abi.py --check`.**  **138 → 131** rows: seven retire and none is added - the six
`missing subr` rows (the six `DEFUN`s) and `missing function w32_console_toggle_lock_key`.  The six
`missing defsubr (&raw const Sw32_*)` rows stay: they are C 11965-11972's calls inside
`syms_of_w32fns`, which is not ported (C 11586 is in the tail).  Three token rows were forestalled
by three `macro-body-notes.tsv` rows appended with this slice (two `GetMessage` sites and one
`FindWindow` site).  No `line count drifted` row: the file is 13,959 Rust lines against 12,488 C
lines (11.8%), inside the file's `w32fns 0.35 total` tolerance row.

**④ The range table and the dead-file gate.**  The `9147-12488` remainder row splits into
`9147-9337` (this slice), `9339-10110` (the remainder), `10112-10138` (this slice's out-of-order
block) and `10140-12488` (the remainder).  `python3 tools/check-dead-file-coverage.py` reads
**172 range row(s) cover src/w32fns.c (12488 lines; 119 blank line(s) between blocks need no row)**,
`0 failure(s)` (169 rows / 116 blanks before the split).  Ten gates read `rc=0` and **0 FAIL** lines.

**⑤ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 13,683-13,930: 61 capitalized
identifiers, missing `GetMessage` and the six `Sw32_*` names.  Both classes are the tool's
documented ones rather than gaps: `GetMessage` is the `<winuser.h>` macro this file reads through
its A/W binding (the `my_create_window` notes row, C 6032), and a `DEFUN!`'s `$sname` static is
macro-generated (measured on slice 18's own text: `check-unit-names.py … 13100 13110` reports
`Sw32_send_sys_command` the same way).  `src/w32fns.rs` 13,932-13,959 (the console function):
10 capitalized identifiers, missing none.

### Deviations (slice 21)

- **`FindWindow` is read through the file's A/W device, with `FindWindowA` declared here.**  C 9332's
  `FindWindow` is the `<winuser.h>` macro whose arm `UNICODE` fixes at compile time, and this file
  already reads the same class of macro - `GetMessage` (C 3714/6032/9230/9312), `DispatchMessage`
  (C 3828), `DefWindowProc` (C 5981) - through `if w32_unicode_gui != 0 { …W } else { …A }`.  The
  port keeps that device, so both entry points must exist: `FindWindowW` was declared for C 3115,
  `FindWindowA` joins it here (with C's own `<winuser.h>` line), and the body's two `(LPCTSTR)`
  casts become the A arm's `LPCTSTR` and the W arm's `LPCWSTR` (`LPCTSTR` is this file's
  `*const c_char`, C 3196).  The un-suffixed token cannot reappear; the notes row records it.
- **`w32_console_toggle_lock_key` stands at the foot of the file, not at C 10112.**  P-91 ① (land
  the dependency the slice needs) versus P-91 ② (text position follows C's line order): the two
  agree only because Rust items are order-independent, so the block is written where the port's text
  currently ends and later slices insert above it.  Its `cur_state = !cur_state;` is C's `!` on an
  `int`, spelled `if cur_state == 0 { 1 } else { 0 }` (the C 3792 reading above it).
- **C's `class` parameter is `r#class`.**  C 9323's parameter is named `class`, which is a Rust
  keyword; PORTING.md 0.2's raw-identifier rule applies, and the token check folds the `r#`.
- **C's reassigned parameters are re-bound with `let mut`.**  `w32-register-hot-key` and
  `w32-unregister-hot-key` assign their `key` argument, and the `DEFUN!` macro's `$arg:ident`
  cannot say `mut` - the `Fx_show_tip` device (`src/w32fns.rs` 11,746-11,748).
- **The two `MSG` locals are zero-initialised** (PORTING.md 1.5), the C 6005/9,592 precedent; C
  leaves them uninitialised and `GetMessage` fills them before C reads `msg.wParam`.
- **Not claimed.**  No run evidence: no Windows and no mingw toolchain on this machine, so no hot
  key is registered, no `PostThreadMessage` is delivered, and the `w32_unicode_gui` arm of the
  `FindWindow` device is never taken here; the arm reading is compile-level (42 errors, none from
  this slice) and the name sweep is a textual reading.

## Slice 22 (C 9339-9752)

**Scope and the stop.**  The geometry and stacking family: `w32-frame-geometry` (C 9339-9486),
`w32-frame-edges` (C 9488-9577), the `static` walk `w32_frame_list_z_order` (C 9579-9607) with its
`w32-frame-list-z-order` `DEFUN` (C 9609-9633), the `static` `w32_frame_restack` (C 9635-9674) with
its `w32-frame-restack` `DEFUN` (C 9676-9707), and the mouse pair -
`w32-mouse-absolute-pixel-position` (C 9709-9724) and `w32-set-mouse-absolute-pixel-position`
(C 9726-9752).  The stop is C 9754's `#ifdef WINDOWSNT`, which opens the disk-free-space pointer
block (C 9753 blank).  The slice's text is inserted **above** C 10112-10138's out-of-order block
(slice 21), so the file's item order still equals C's (P-91 ②).

**What the item carries.**  Eight items in C's order, each with `make-docfile`'s rule turning C's
`doc: /* … */` into the `doc:` literal, and two new `static` helpers plus two new `DEFUN`s around
them.  `w32-frame-geometry` is C 9385-9393's two records and eleven scalars (the records
zero-initialised per PORTING.md 1.5), C 9400-9403's `GetWindowInfo` fill with the two border reads,
C 9405-9422's `get_title_bar_info_fn` block (`TITLEBAR_INFO` is C 167-190's pair, C 9412-9413's
six-entry `rgstate` clear, C 9415's `!(rgstate[0] & 0x00008001)` test), C 9423-9424's `WS_CAPTION`
fallback to `SM_CYCAPTION`, C 9426-9432's `GetMenuBarInfo` and the two menu metrics, C 9435-9448's
rectangle reads and menu-bar height fix-up, and C 9450-9485's eleven-`Fcons` `list`.  Its two
chained assignments (C 9410-9411, C 9427-9428) are two statements each, innermost store first.
`w32-frame-edges` is C 9511-9528's `outer-edges` arm (`GetWindowRect` straight into `list4`) and
C 9529-9576's other arm (`GetClientRect` + `ClientToScreen`, then the `inner-edges` four
`FRAME_*`-adjusted coordinates or the native rectangle).  The walk is C 9591-9604's `while (window)`
over `w32_window_to_frame` with C 9597's `XSETFRAME`/`Fcons` pair, C 9602's `GetNextWindow` and
C 9606's `Fnreverse`; its `DEFUN` is C 9622's `check_x_display_info` and C 9626-9629's
`FRAMEP`/`FRAME_LIVE_P` choice between `GetWindow (…, GW_CHILD)` and `GetTopWindow (NULL)`.
`w32_frame_restack` is C 9650-9672's two `SetWindowPos` orders behind the two `GetNextWindow`
guards; its `DEFUN` is C 9694-9699's two decodes and call, with C 9704's
`error ("Cannot restack frames")`.  The mouse pair is C 9717-9723's `POINT`/`GetCursorPos`/`Fcons`
and C 9733-9749's `check_integer_range` pair, `os_subtype == OS_SUBTYPE_NT` +
`w32_major_version + w32_minor_version >= 6` guard, `SPI_GETMOUSETRAILS` save, `SetCursorPos` and
`SPI_SETMOUSETRAILS` restore.

**New surface (all of it `<winuser.h>`'s).**  `WINDOWINFO` and `MENUBARINFO` (C 9385-9386's two
records), `GetWindowInfo`, `GetMenuBarInfo`, `GetTopWindow`, `ClientToScreen`, `SetCursorPos`, the
five constants `SM_CYCAPTION`, `SM_CYMENU`, `GW_HWNDNEXT`, `SPI_GETMOUSETRAILS`,
`SPI_SETMOUSETRAILS`, and `frame.h` 1476's `decode_live_frame` declaration (no mirror carries it -
the `decode_window_system_frame` shape).  They stand in one block at the head of the slice with the
header text each one is taken from (the slice-8 reading for `<windows.h>` names).

### Evidence (slice 22, the final revision: 14,435 Rust lines)

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w22lane cargo run -q -p xtask -- build-tu src/w32fns.c` →
`built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, the object **336 bytes**.  One run.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - the count slices 16-21
left, so this slice's 476 lines add no error and no member gap: no diagnostic sits at or beyond the
slice's first line (`src/w32fns.rs` 13,915), and the kinds are the standing mirror set (15 `E0428`
name collisions, 1 `E0530` shadowing, 25 `E0609` missing members, plus the one deliberately-failing
`HAVE_NTGUI` frame-flag probe).  One run.

**③ `extract-abi.py --check`.**  **131 → 123** rows: eight retire and none is added - the six
`missing subr` rows (the six `DEFUN`s) and the two `missing function` rows
(`w32_frame_list_z_order`, `w32_frame_restack`).  The six new `missing defsubr (&raw const Sw32_*)`
rows belong to `syms_of_w32fns` (C 11949-11986, unported), as the six from slice 21 do.  Four token
rows were forestalled by four `macro-body-notes.tsv` rows appended with this slice (two
`GetNextWindow` bodies and two `BOOL … = false` bodies).  No `line count drifted` row: 14,435 Rust
lines against 12,488 C lines (15.6%), inside the file's `w32fns 0.35 total` tolerance row.

**④ The range table and the dead-file gate.**  The `9339-10110` remainder row splits into
`9339-9752` (this slice) and `9754-10110` (the remainder); `check-dead-file-coverage.py` reads
**173 range row(s) cover src/w32fns.c (12488 lines; 120 blank line(s) between blocks need no row)**,
`0 failure(s)` (172 rows / 119 blanks before the split).

**⑤ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 13,915-14,389 (the slice's text):
90 capitalized identifiers, missing `Clone`, `Copy`, the six `Sw32_*` and `Y` - every one of them a
class the tool documents as "not an item": `#[derive (Clone, Copy)]`'s two arguments, a `DEFUN!`'s
per-subr `Sname` static (macro-generated), and the SDK's own parameter name in a declaration
(`SetCursorPos (X: c_int, Y: c_int)`, the `SendMessageW` prototype class).  The slice's docstrings
alone read two identifiers, missing only the S-name.

### Deviations (slice 22)

- **Two records' layouts are unmeasured.**  `WINDOWINFO`'s field list and `MENUBARINFO`'s single
  bitfield word are spelled from the header and **corroborated against the Win32 metadata projection**
  (`windows-sys` 0.52's `WINDOWINFO` - same ten fields in the same order, including
  `dwWindowStatus` - and `MENUBARINFO`, whose tail is a `_bitfield: i32`), not against a compiled
  `sizeof`: this machine has no SDK and no mingw toolchain.  `MENUBARINFO`'s two `BOOL … : 1` members
  are one `bits` word, LSB first (PORTING.md 1.3, the `src/w32.rs` `DCB` device); nothing writes
  either bit.  The integrator owes both records' measured layout.
- **`GetNextWindow` is a pure rename, so the port keeps the real entry point.**  C's `<winuser.h>`
  has `#define GetNextWindow(hWnd, wCmd) GetWindow(hWnd, wCmd)`, and the port calls `GetWindow` with
  C's own `GW_HWNDNEXT`; the notes rows record the macro name for the two bodies that use it.  This
  is deliberately *not* the `CreateWindow` treatment (C 2399-2471 section): that macro's expansion
  inserts the `0L` extended style, so its declaration had to carry the macro's own name and
  argument list, while this one only renames.
- **C's chained assignments are two statements each.**  C 9410-9411 (`title_bar.rcTitleBar.left =
  … .right = 0;`) and C 9427-9428 (`menu_bar.rcBar.right = … .left = 0;`) are each written as two
  stores, innermost first - the C 6051-6052 `tip_window` reading (Rust's assignment yields `()`).
- **C's unsigned chain in the tab-bar/tool-bar widths is cast at the operand.**  C 9469/9480's
  `right - left - 2 * external_border_width - 2 * internal_border_width` promotes the whole chain to
  `unsigned int` (the local is `unsigned int`, C 9388); the port casts that operand `as c_int` at the
  subtraction, which is the same value for every border width a window manager reports - recorded
  rather than silently assumed.
- **C's `BOOL … = false` keeps C's `BOOL`** (C 9514's `success`, C 9734's `ret`), so the port writes
  `0` and the notes rows carry the `false` token (the `record_in_backtrace` class).
- **C's `class` and `type` parameters are `r#class`/`r#type`** (C 9323, C 9504): Rust keywords,
  PORTING.md 0.2's raw-identifier rule; the token check folds the `r#`.
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no frame geometry is
  measured, no window is restacked, and the mouse cannot be moved; the arm reading is compile-level
  (42 errors, none from this slice) and the name sweep is a textual reading.

## Slice 23 (C 9755-9999)

**Scope and the stop.**  The file's two `#ifdef WINDOWSNT` blocks: C 9756-9759's two
`GetDiskFreeSpaceEx` pointer types with `file-system-info` (C 9761-9890) under C 9755's `#ifdef`, and
`default-printer-name` (C 9893-9999) under C 9893's.  The stop is C 10002's
`/* Equivalent of strerror for W32 error codes.  */` comment (C 10000-10001 blank, C 9999's `#endif`
is this slice's last line).  Both DEFUNs carry the nested gate
`all (feature = "have_ntgui", feature = "windowsnt")` - the file gate plus C's `#ifdef WINDOWSNT`,
which is not the frozen configuration's `HAVE_NTGUI`.

**What the item carries.**  `file-system-info` is C 9765's two locals with C 9768's reassigned
parameter (the port re-binds it), C 9767-9769's `CHECK_STRING`/`Fexpand_file_name`/`ENCODE_FILE`,
C 9773-9780's handler probe (`Ffind_file_name_handler`, `calln!`, the `CONSP`-or-`NILP` early
return, C 9779's `error`), C 9789-9797's dynamic `GetDiskFreeSpaceEx` probe (`get_proc_addr` +
`transmute` from `VOIDFNPTR`, `w32_unicode_filenames` choosing which pointer matters), C 9801-9804's
three root-name buffers, C 9808-9829's two root-name forms - the drive-letter case and the UNC walk
whose C `do … while (*name)` the port writes as a `loop` with C's own two exits (C 9825's `while`
condition and C 9821's `break`) - C 9831-9834's `filename_to_utf16`/`filename_to_ansi` choice,
C 9836-9858's `LARGE_INTEGER` arm (three zero-initialised unions, the two `(ULARGE_INTEGER *)`
casts spelled as C spells them, `list3` of the three `QuadPart` doubles, C's own comment about why
the signed union is used) and C 9859-9885's `GetDiskFreeSpaceW`/`A` arm with the three products.
`default-printer-name` is C 9898's function-local `static char pname_buf[…]` (moved to file scope
with its initialiser, the `force_quit_count`/`coords` device), C 9899-9905's locals, C 9910's
`GetProfileString` probe with C 9913's `strtok`, C 9915-9916's `OpenPrinter`, C 9918-9926's
`GetPrinterW`/`A` size probe (`w32_unicode_filenames && os_subtype != OS_SUBTYPE_9X`, C's unicows
comment) with the `dwNeeded` test, C 9928-9974's two arm bodies (`xmalloc`, the second `GetPrinter`,
C's `Attributes & PRINTER_ATTRIBUTE_SHARED` + `pServerName` test and the shared/local split,
`xfree` on failure), C 9976-9995's two name shapes (`snprintf` for the remote printer, `strncpy` +
C's first-port `strtok` cut for the local one, both ending in C's
`pname_buf[sizeof (pname_buf) - 1] = '\0'`) and C 9997's `DECODE_FILE (build_unibyte_string (…))`.

**New surface.**  `LPCSTR` (C 9758's typedef; the unit had `LPCTSTR`/`LPWSTR`/`LPCWSTR` but not
this one), the two `…_Proc` types, `LARGE_INTEGER`/`ULARGE_INTEGER` (spelled as `src/w32.rs` spells
them) with `PULARGE_INTEGER`, `GetDiskFreeSpaceW`/`A`, then `PRINTER_ATTRIBUTE_SHARED`,
`PRINTER_INFO_2W`/`PRINTER_INFO_2A` (twenty-one fields each), `GetProfileStringA`, `OpenPrinterA`,
`GetPrinterW`/`GetPrinterA`, `ClosePrinter`, and in-unit `strtok`/`strncpy` declarations
(`<string.h>`'s, the `src/image.rs` 8571 shape; `snprintf` is already `lib/include/stdio.rs`'s).

### Evidence (slice 23, the final revision: 14,914 Rust lines)

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w23lane cargo run -q -p xtask -- build-tu src/w32fns.c` →
`built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, the object **336 bytes**.  Two runs
(the first text and the corrected one below).

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - the count slices 16-22
left (15 `E0428`, 1 `E0530`, 25 `E0609`, and the one deliberately-failing frame-flag probe), so this
slice's 479 lines add no error and no member gap: no diagnostic sits at or beyond the slice's first
line (`src/w32fns.rs` 14,390).  **The first probe run is what caught this slice's one real defect**:
`LPBYTE` was declared here although the file already spells it at `src/w32fns.rs` 254, and rustc
answered with a duplicate-definition error that *also* hid the standing 42-error reading (the run
reported 18).  Removing the duplicate restored it; the second run is the reading above.  Two runs.

**③ `extract-abi.py --check`.**  **123 → 121** rows: the two `missing subr` rows (`Ffile_system_info`,
`Fdefault_printer_name`) retire and none is added.  Two token rows were found and cleared - `Ffile_system_info
does not mention ularge_integer` was a *code* fix (the four casts are written `as *mut ULARGE_INTEGER`,
which is C's own `(ULARGE_INTEGER *)` text, rather than through the `PULARGE_INTEGER` alias), and
`Fdefault_printer_name does not mention 3` is the moved `pname_buf` bound, recorded in a notes row.
Two `macro-body-notes.tsv` rows accompany the slice.  No `line count drifted` row: 14,914 Rust lines
against 12,488 C lines (19.4%), inside the file's `w32fns 0.35 total` tolerance row.

**④ The range table and the dead-file gate.**  The `9754-10110` remainder row splits into
`9755-9999` (this slice) and `10002-10110` (the remainder); `check-dead-file-coverage.py` reads
**174 range row(s) cover src/w32fns.c (12488 lines; 123 blank line(s) between blocks need no row)**,
`0 failure(s)` (173 rows / 120 blanks before the split).

**⑤ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 14,390-14,868: 64 capitalized
identifiers, missing `Attributes`, `AveragePPM`, `DefaultPriority`, `Priority`, `StartTime`,
`UntilTime`, `LowPart`, `HighPart`, `QuadPart`, `Level`, `Clone`, `Copy` and the two `S…` names -
all four of the tool's documented non-item classes: a struct definition's *field* names, a
declaration's parameter name (`Level`, the SDK's own spelling in `GetPrinterW`), `#[derive (Clone,
Copy)]`'s arguments, and a `DEFUN!`'s macro-generated `$sname` static.

### Deviations (slice 23)

- **The two printer records' layouts are unmeasured.**  `PRINTER_INFO_2W`/`PRINTER_INFO_2A`'s
  twenty-one fields are spelled from `<winspool.h>` and **corroborated against the Win32 metadata
  projection** (`windows-sys` 0.52: the same fields in the same order), not against a compiled
  `sizeof` - this machine has no SDK.  `pDevMode`/`pSecurityDescriptor` are C's pointers to records
  this unit has no mirror of, so they stay `*mut c_void` (PORTING.md P-72 rule ③).
- **`GetProfileString` and `OpenPrinter` are read as the `A` entry points.**  Both are macros in
  C, and both calls pass `pname_buf`, which is a `char` buffer - only the `A` function accepts it,
  so a `W` arm could not be written here at all.  This is deliberately *not* the
  `w32_unicode_gui` device the earlier slices use for `GetMessage`/`FindWindow`/`DefWindowProc`
  (whose arguments are not typed `char *`), and the notes row records the two token gaps; the
  integrator's mingw measurement should settle the whole class at once.
- **C's `do { … } while (*name)` is a `loop` with C's two exits.**  C 9819-9825's iteration is the
  port's `loop { …; if *name == 0 { break; } }`, with C 9821's `break` carrying the `--slashes == 0`
  test (PORTING.md 2.2's `do…while` class).
- **C's `UINT <= 0` is the port's `== 0`.**  C 9910's `GetProfileStringA (…) <= 0` cannot be false
  for an unsigned return value except at zero, so the port writes the comparison C means; the `0`
  token stays.
- **`pname_buf` moves to file scope** with its initialiser (the function-local `static` device this
  file already uses for `coords`/`force_quit_count`); the notes row records the one bound token the
  move takes out of the body.
- **`LPCSTR` is spelled here** because C 9758's typedef needs it and the unit carried only
  `LPCTSTR`/`LPSTR`/`LPWSTR`/`LPCWSTR`.
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no volume is queried and no
  printer is enumerated; the arm reading is compile-level (42 errors, none from this slice) and the
  name sweep is a textual reading.

## Slice 24 (C 10002-10110, and C 315-319 resolved)

**Scope and the stop.**  The strerror/version group: `w32_strerror` (C 10002-10027),
`w32_last_error` (C 10029-10038, whose C 10032 forward declaration is dropped per PORTING.md 0.1-12),
`cache_system_info` (C 10040-10081), `w32_version_string` (C 10083-10093, under C's `#ifdef
WINDOWSNT`) and `_DebPrint` (C 10095-10110, under C's `#ifdef EMACSDEBUG`, which is the `emacsdebug`
feature the file's `w32_name_of_message` already uses).  The stop is C 10112's
`w32_console_toggle_lock_key` (C 10111 blank) - already landed, out of order, by slice 21.  The slice
also **resolves the C 315-319 deferral**: this file *defines* `sysinfo_cache` and `osinfo_cache`, and
the two records were spelled by another lane in the meantime (`include/w32common.rs` 118-146, marked
UNVERIFIED there), so the two objects now stand at their C position - the `syspage_mask` shape
(the mirror declares them inside `w32common_decls`, the owning unit defines them).

**What the item carries.**  `w32_strerror` is C 10006's function-local `static char buf[500]` (file
scope; C's own name `buf` is taken by C 3630-3632's `emacsdebug` static, so this one is `buf_`, the
trailing-underscore spelling the checker folds), C 10009-10010's `GetLastError` default, C 10012-10017's
`FormatMessage` call with C's two flag bits, C 10019-10021's trailing-CRLF trim and C 10023-10024's
`sprintf` fallback.  `cache_system_info` declares C 10044-10053's function-local union inside the body
(the `src/thread.rs` 788-795 reading; C's union has no tag, so the port names it after C's variable
`version`, and C's own member names `info`/`data`/`major`/`minor`/`platform` stay) and then reads
C 10056's `hinst`, C 10059-10061's `GetVersion` unpacking, C 10063-10066's platform-bit test,
C 10069-10070's `GetSystemInfo`/`syspage_mask`, C 10073-10076's `OSVERSIONINFO` size + `GetVersionEx`
+ `dwBuildNumber`, C 10077-10078's 9X truncation and C 10080's mouse-button metric.
`w32_version_string` moves C 10088's function-local `static char version_string[…]` to file scope with
its initialiser (the `coords` device; a notes row carries the three numbers its bound takes out of the
body) and keeps C 10089-10090's `_snprintf`.  `_DebPrint` is C 10099's `char buf[1024]` with C 10100-10108's
`va_list` trio (the `...` parameter *is* the list, PORTING.md P-04), the `CYGWIN` `fprintf` arm and
`OutputDebugString`.

**New surface.**  `SM_CMOUSEBUTTONS`, `FORMAT_MESSAGE_FROM_SYSTEM`/`FORMAT_MESSAGE_IGNORE_INSERTS`
(both values corroborated against the Win32 metadata projection, `windows-sys` 0.52: 4096/512), and
the declarations `GetLastError`, `FormatMessageA`, `GetVersion`, `GetSystemInfo`, `GetVersionExA`,
`OutputDebugStringA`, `_snprintf`, `vsprintf` (`<stdio.h>`'s MSVCRT pair, in-unit like
`src/nt/cmdproxy.rs` 261-265 does for the first).

### Evidence (slice 24, the final revision: 15,122 Rust lines)

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w24lane cargo run -q -p xtask -- build-tu src/w32fns.c` →
`built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, the object **336 bytes**.  Two runs
(the first text and the one after the mirror fix below).

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - the standing count
(15 `E0428`, 1 `E0530`, 25 `E0609`, and the one deliberately-failing frame-flag probe), so this
slice's 208 lines add no error and no member gap: **no diagnostic sits at or beyond `src/w32fns.rs`
888** (the slice's first changed line).  **The first probe run read 43** and named this slice's one
real defect: the mirror declared `w32_version_string` at *file* scope under `windowsnt`, so this
unit's definition collided with it (`the name w32_version_string is defined multiple times`).  The
fix is P-21's rule - the declaration moves into `w32common_decls`, where the unit's definition
shadows the glob import (the tenth such move; the mirror's own ledger now records it, and every
caller still resolves the name through `use w32common_decls::*;`).  Two runs.

**③ `extract-abi.py --check`.**  **121 → 116** rows: five `missing function` rows retire
(`w32_strerror`, `w32_last_error`, `cache_system_info`, `w32_version_string` and the pair's
neighbour) and none is added; the mirror pair (`w32common.h` → `include/w32common.rs`) still reads
`OK`.  Two token rows were found and cleared by notes rows (`w32_strerror` does not mention `500` -
the moved static's bound; `cache_system_info` does not mention `getversionex` - C's plain macro).
Five `macro-body-notes.tsv` rows accompany the slice.  No `line count drifted` row: 15,122 Rust lines
against 12,488 C lines (21.1%), inside the file's `w32fns 0.35 total` tolerance row.

**④ The range table and the dead-file gate.**  The `315-319` row loses its **deferred** mark and the
`10002-10110` remainder row becomes this slice's landed row; `check-dead-file-coverage.py` reads
**174 range row(s) cover src/w32fns.c (12488 lines; 123 blank line(s) between blocks need no row)**,
`0 failure(s)`.

**⑤ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 888-922 (the two cache definitions):
2 capitalized identifiers, missing none.  `src/w32fns.rs` 14,896-15,076 (the five functions and the
new surface): 24 capitalized identifiers, missing `Clone` and `Copy` - the `#[derive (Clone, Copy)]`
arguments of the two records `cache_system_info` declares in its body, i.e. the tool's documented
"not an item" class.

### Deviations (slice 24)

- **The C 315-319 deferral is resolved, and the records stay the mirror's.**  This file defines the
  two objects (C 316/319) and the two records' spellings are `include/w32common.rs` 118-146's, marked
  UNVERIFIED there because no `<windows.h>` exists on this machine.  The port's two definitions carry
  explicit zero initialisers (C's text has none: a tentative definition) and the mirror's copies stay
  the declarations, so the two records are still *unmeasured* - the integrator's mingw measurement is
  what would close that.
- **`w32_version_string`'s declaration moves inside `w32common_decls`.**  P-21's E0428 rule; the
  mirror's ledger has the note (the tenth declaration of that class).  Nothing else in the mirror
  changed.
- **`buf_` is C's `buf` folded.**  C's `w32_strerror` owns a `static char buf[500]` and C 3630-3632's
  `w32_name_of_message` owns a `static char buf[64]`; both live at file scope in the port and the
  `emacsdebug` gate compiles the second, so the first is spelled `buf_` (PORTING.md 0.2's
  trailing-underscore fold, which the token check applies).
- **Three plain macros are read as the `A` entry points.**  `FormatMessage`, `GetVersionEx` and
  `OutputDebugString` (C 10012/10074/10108) all take a `char` buffer here, which only the `A`
  function accepts - the same reading as slice 23's `GetProfileString`/`OpenPrinter`, and the notes
  rows record the three token gaps.  The integrator's mingw measurement should settle the whole class
  at once, including the earlier slices' `w32_unicode_gui` device.
- **C's anonymous function-local union gets the port's name `version`.**  Its members and the inner
  tag are C's (`info`, `data`, `major`, `minor`, `platform`); only the union's own name is the
  port's, since C's has no tag (`src/thread.rs` 788-795 establishes the in-body record device).
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no cache is filled, no error
  string is formatted and no debug output is emitted; the arm reading is compile-level (42 errors,
  none from this slice) and the name sweep is a textual reading.

## Slice 25 (C 10140-10312)

**Scope and the stop.**  `w32_kbd_mods_to_emacs` (C 10140-10202), `w32_kbd_patch_key`
(C 10207-10287, whose C 10242-10252 `#if 0` block is carried as C's own text), `w32_sys_ring_bell`
(C 10290-10303) and the `w32--menu-bar-in-use` `DEFUN` (C 10305-10312).  The stop is C 10314's
`#if defined WINDOWSNT && !defined HAVE_DBUS`, which opens the tray-notification block (C 10313
blank).

**What the item carries.**  `w32_kbd_mods_to_emacs` is C 10145's `retval`, C 10149-10152's AltGr
strip, C 10154-10155's alt-or-meta choice by `Vw32_alt_is_meta`, C 10157-10163's control/meta pair,
C 10165-10172's four `w32_key_to_modifier` calls and C 10176-10199's CapsLock ladder with C's own
"keys that should _not_ be affected" list (`VK_NUMPAD_CLEAR`/`VK_NUMPAD_DELETE` are
`include/w32term.rs`'s).  `w32_kbd_patch_key` is C 10210-10212's three locals, C 10216-10223's
dead-key short circuits, C 10225-10240's `keystate` fill, C 10256-10276's NT arm (`ToUnicode`,
`GetConsoleCP` when `cpId == -1`, the `uChar.UnicodeChar` write, `WideCharToMultiByte`) and
C 10277-10281's `ToAscii` arm; C 10213-10214's two function-local statics (`ansi_code`, `isdead`)
stand at file scope (the `force_quit_count` reading).  `w32_sys_ring_bell` is C 10293-10302's three
arms, and the `DEFUN` is C 10311's one ternary over C 303's `menubar_in_use`.

**New surface.**  `VK_BACK`, `VK_TAB`, `VK_ESCAPE`, `VK_HELP`, `VK_F24` (the other `VK_*` this slice
compares were already spelled here or come from the mirror), `LPWORD`, and the declarations
`ToUnicode`, `ToAscii`, `WideCharToMultiByte`, `GetConsoleCP`, `Beep`, `MessageBeep`.

**The mirror's three misattributed declarations move.**  `include/w32term.rs` declared
`w32_kbd_patch_key`, `w32_kbd_mods_to_emacs` and `w32_sys_ring_bell` at its crate root under
`windowsnt`, with two notes calling them `src/w32term.c`'s own - but
`grep -n '^w32_kbd_patch_key' emacs/src/*.c` shows the one definition each, in `src/w32fns.c`
(C 10208, C 10143, C 10291).  They now stand in a new `pub mod w32term_fns_decls` with the
`use w32term_fns_decls::*;` this file already uses twice (P-21/P-39: the unit's own definition
shadows the glob import), so the names this slice defines no longer collide.  The mirror's ledger
has the note; the mirror pair still reads `OK`.  Two in-unit call sites (C 4915's block) dropped the
`*mut c_void` cast the untyped declaration had forced, since the unit's typed definition is now the
live item.

### Evidence (slice 25, the final revision: 15,370 Rust lines)

**① The frozen configuration (`build-tu`).**  Three runs, and the first two **failed** - the frozen
configuration still *parses* the whole file even though the file gate compiles none of it, so
`build-tu` is a real syntax gate here: run 1 `unexpected closing delimiter` (C 10186's `))` is C's
extra grouping parenthesis, which Rust's `if` does not have) and run 2 `expected '{', found
MessageBeep` (C 10299-10302's brace-less `else MessageBeep (…)`).  Run 3: exit 0, the object
**336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Run 3 reads **42 previous errors** - the standing
count (15 `E0428`, 1 `E0530`, 25 `E0609`, plus the one deliberately-failing frame-flag probe) - and
**no diagnostic sits at or beyond `src/w32fns.rs` 15,077** (the slice's first new line).  Run 1 was
the parse failure above and run 2 read **46**: four `E0308`s, all this slice's own -
`w32_kbd_patch_key (&raw mut key_record as *mut c_void, …)` at the two C 4915 call sites (the cast
the *mirror's* untyped declaration had forced) and `ansi_code[0]`/`ansi_code[2]` (C's `BYTE` is
`u8`, C's `AsciiChar` is `CHAR`/`i8`).  Both fixed in place; the reading above is the third run.

**③ `extract-abi.py --check`.**  **116 → 112** rows: the three `missing function` rows and the
`missing subr Fw32__menu_bar_in_use` row retire, and one row is *added* -
`missing defsubr (&raw const Sw32__menu_bar_in_use)`, which is C 11941's call inside the unported
`syms_of_w32fns`.  No token row was needed: every token of the four C bodies is spelled here.  The
mirror pair (`w32term.h` → `include/w32term.rs`) still reads `OK`.  No `line count drifted` row:
15,370 Rust lines against 12,488 C lines (23.1%), inside the file's `w32fns 0.35 total` tolerance
row.

**④ The range table and the dead-file gate.**  The `10140-12488` remainder row splits into
`10140-10312` (this slice) and `10314-12488` (the remainder); `check-dead-file-coverage.py` reads
**175 range row(s) cover src/w32fns.c (12488 lines; 124 blank line(s) between blocks need no row)**.

**⑤ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 15,077-15,324: 60 capitalized
identifiers, missing `CodePage` (the SDK's own parameter name in the `WideCharToMultiByte`
declaration - the `Y`/`Level` class) and `Sw32__menu_bar_in_use` (a `DEFUN!`'s macro-generated
`$sname`).

### Deviations (slice 25)

- **Three mirror declarations move into `w32term_fns_decls`.**  See above; the mirror's ledger
  carries the note and the mirror pair's checklist is unchanged.
- **Two in-unit call sites lose their `*mut c_void` cast.**  C 4915's block called
  `w32_kbd_patch_key` through the mirror's untyped declaration; the unit's definition is typed
  (`*mut KEY_EVENT_RECORD`), so the casts go.  Every *other* unit still sees the mirror's P-72
  rule 3 spelling.
- **C's `BYTE` → `CHAR` conversions are explicit.**  C 10218/10285 assign an `ansi_code` byte to
  `uChar.AsciiChar`; C converts silently and the port writes `as CHAR` (C's `BYTE` is `u8`, `CHAR`
  is `i8`).
- **The five `<wincon.h>` bits are `c_int` while `mods` is `DWORD`.**  C promotes them implicitly;
  each use here carries the cast (`as DWORD`/`as c_uint`), which is what the file's other
  `mods`-shaped sites do.
- **C 10242-10252's `#if 0` block is carried as C's own text** inside a comment (the file's reading
  for a region no configuration compiles).
- **`ansi_code`/`isdead` stand at file scope** (Rust has no function-scope `static`; the
  `force_quit_count` reading), keeping C's names.
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no console key is patched
  and no key modifier is translated; the arm reading is compile-level (42 errors, none from this
  slice) and the name sweep is a textual reading.

## Slice 26 (C 10314-10427)

**Scope and the stop.**  The head of C's tray-notification block, which C 10314's
`#if defined WINDOWSNT && !defined HAVE_DBUS` opens: C 10316's `pfnShell_NotifyIconW`,
C 10322-10341's `MY_NOTIFYICONDATAW`, C 10343-10345's three size macros, C 10346-10364's five
`#ifndef` guards and two `#define`s, C 10366-10371's `enum NI_Severity`, C 10375-10402's
`get_dll_version` and C 10408-10427's `utf8_mbslen_lim`.  The stop is C 10429's
`static unsigned short last_tray_notification_id;` (C 10428 blank), which opens the next slice's
`add_tray_notification`.  Every item carries C's three conditions as
`all (feature = "have_ntgui", feature = "windowsnt", not (feature = "have_dbus"))` - the file gate
plus C's own pair (`feature-coverage.tsv` 138 names `have_dbus`); the frozen configuration does not
define `HAVE_DBUS` and does define `WINDOWSNT`, so these items **are** live lines for the checklist,
which is why this slice retires twenty-two rows.

**What the item carries.**  The record is C's private copy of `<shellapi.h>`'s `NOTIFYICONDATAW`
("A private struct declaration to avoid compile-time limits"), fifteen members in C's order with the
anonymous union member keeping the SDK's compatibility name `DUMMYUNIONNAME` (the `SHELLEXECUTEINFOW`
device: a `MY_NOTIFYICONDATAW_union` type plus the `r#DUMMYUNIONNAME` field).  The three size macros
are C's `offsetof` expressions; C 10343's `szTip[64]` names an *element* of the array, so the port
adds `64 * size_of::<WCHAR> ()` to that field's `core::mem::offset_of!`.  `get_dll_version` is
C 10378-10398 (the `LoadLibrary` probe, the `DllGetVersion` lookup through `get_proc_addr` +
`transmute`, the `DLLVERSIONINFO` fill and `SUCCEEDED`/`MAKEDLLVERULL`, `FreeLibrary`), and
`utf8_mbslen_lim` is C 10411-10424's `CHAR_BYTES` walk (`include/character.rs`'s function).

**New surface.**  `ULONGLONG`, `DLLGETVERSIONPROC`, `DLLVERSIONINFO` (five fields, corroborated
against `windows-sys` 0.52's record), `MAKEDLLVERULL!`, the two `<libloaderapi.h>` declarations
`LoadLibraryA`/`FreeLibrary`, and the constants `WM_APP`, `NIF_INFO`, the four `NIIF_*`,
`EMACS_TRAY_NOTIFICATION_ID_INIT`, `EMACS_NOTIFICATION_MSG` and the four `Ni_*`.

### Evidence (slice 26, the final revision: 15,582 Rust lines)

**① The frozen configuration (`build-tu`).**  One run: `built src/w32fns.rs ->
../build/femacs/w32fns.o`, **exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  One run: **42 previous errors** - the standing
count (15 `E0428`, 1 `E0530`, 25 `E0609`, plus the one deliberately-failing frame-flag probe) - so
this slice's 212 lines add no error and no member gap, and **no diagnostic sits at or beyond
`src/w32fns.rs` 15,325** (the slice's first new line).  The reading is a real one for these items:
`have_dbus` is off, so the three-condition gate is satisfied and rustc sees the whole slice.

**③ `extract-abi.py --check`.**  **112 → 90** rows: twenty-two retire and one is added.  The retired
set is the block's whole head - `get_dll_version`, `utf8_mbslen_lim`, the three
`MYNOTIFYICONDATAW_V*_SIZE` macros, `EMACS_NOTIFICATION_MSG`/`EMACS_TRAY_NOTIFICATION_ID_INIT`, the
five `NIF_INFO`/`NIIF_*` guards and the four `Ni_*` enumerators (the checker looks for both the
macro and the constant spelling of each guarded name, so the eleven constants are twenty-two rows).
The one added row is the token gap `function get_dll_version does not mention loadlibrary` (C's plain
`LoadLibrary` macro vs the `LoadLibraryA` the port must call for a `char *` argument), cleared by one
`macro-body-notes.tsv` row.  No `line count drifted` row: 15,582 Rust lines against 12,488 C lines
(24.8%), inside the file's `w32fns 0.35 total` tolerance row.  (A side note the measurement itself
produced: running `--check` against a *copy* of the port under a different file name loses the
tolerance row - the table is keyed by path - and then reports the drift; the reading above is the
real file's.)

**④ The range table and the dead-file gate.**  The `10314-12488` remainder row splits into
`10314-10427` (this slice) and `10429-12488` (the remainder); `check-dead-file-coverage.py` reads
**176 range row(s) cover src/w32fns.c (12488 lines; 125 blank line(s) between blocks need no row)**.

**⑤ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 15,325-15,536: 42 capitalized
identifiers, missing `Clone` and `Copy` - the `#[derive (Clone, Copy)]` arguments of the three
records this slice declares, i.e. the tool's documented "not an item" class.

### Deviations (slice 26)

- **`MY_NOTIFYICONDATAW`'s layout is unmeasured.**  The field list and order are the SDK's,
  corroborated against the Win32 metadata projection (`windows-sys` 0.52's `NOTIFYICONDATAW`), not
  against a compiled `sizeof`; the integrator's mingw measurement is what would close it.  The
  anonymous union member is the `SHELLEXECUTEINFOW` device (a named union type + C's compatibility
  field name).
- **The three size macros add the array element C's `offsetof` names.**  C's
  `offsetof (MY_NOTIFYICONDATAW, szTip[64])` is `szTip`'s offset plus 64 `WCHAR`s; the port writes
  exactly that, so the value is C's byte count for any layout.
- **`PNOTIFYICONDATAW` is untyped here.**  C's pointer type is the SDK's record and this unit carries
  only its own `MY_NOTIFYICONDATAW`, so the function-pointer type's second parameter is `*mut c_void`
  (PORTING.md P-72 rule ③) and the one call site (next slice) casts.
- **`LoadLibrary` is read as the A entry point.**  C's plain macro takes the `char *` this function
  passes; the notes row records the token gap, like slices 23-24's `GetProfileString`/`FormatMessage`.
- **`enum NI_Severity` is four `c_int` constants.**  C's small enum has no Rust counterpart; the
  members keep C's names and C's implicit values (the `OS_SUBTYPE_9X` reading), and the *type* the
  parameter takes is `c_int`.
- **The five `#ifndef` guards are constants with C's values and no `cfg`** - the file's standing
  reading of an SDK "define it if the header has not" guard.
- **C's `str` parameter is `str_`** (C 10409): the trailing-underscore spelling this tree uses for
  that name (`include/lisp.rs` 2728's `intern`), which the token check folds.
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no DLL version is queried and
  no UTF-8 length is measured; the arm reading is compile-level (42 errors, none from this slice) and
  the name sweep is a textual reading.

## Slice 27 (C 10429-10633)

**Scope and the stop.**  `last_tray_notification_id` (C 10429), `add_tray_notification`
(C 10433-10605) and `delete_tray_notification` (C 10612-10633).  The stop is C 10635's
`w32-notification-notify` `DEFUN` (C 10634 blank).  All three items carry slice 26's three-condition
gate.

**What the item carries.**  `add_tray_notification` is C 10438's `retval`, C 10442-10447's locals,
C 10449-10463's four version-dependent `cbSize` choices (`get_dll_version`'s value against C's three
`MAKEDLLVERULL` thresholds), C 10464-10472's window/counter/flags/message fills, C 10473-10501's icon
load (the class icon, or the file's own through `filename_to_utf16`/`filename_to_ansi` with C's two
`ENOENT` exits), C 10502-10514's `GetLastError` split, C 10516-10540's tip conversion
(`pMultiByteToWideChar` over `utf8_mbslen_lim`'s cap with C's 63/127 truncations and `wcscpy`),
C 10542-10589's Windows-2000 tail (message, `uTimeout` through the record's union member, title and
the `severity` switch as a `match`), C 10591-10599's `Shell_NotifyIconW (NIM_ADD, …)` call (through
`pfnShell_NotifyIconW.unwrap ()`, the `pfnSHFileOperationW` device), C 10600-10602's `done:` cleanup
and C 10604's return.  C's three `goto done` sites are the port's `'done:` labelled block, and the
`done:` statement itself stands after it.  `delete_tray_notification` is C 10617-10630's six
statements with C 10627's `DebPrint`.

**New surface.**  `NIM_ADD`/`NIM_DELETE`, `IMAGE_ICON`, `LR_LOADFROMFILE`, `NIF_MESSAGE`/`NIF_ICON`/
`NIF_TIP` (see the finding below - C's file defines only `NIF_INFO`), the `LoadImageW`/`LoadImageA`
pair (this unit declares `LoadImage`, the macro, above), `DestroyIcon` and the CRT's `wcscpy`.

### Evidence (slice 27, the final revision: 15,836 Rust lines)

**① The frozen configuration (`build-tu`).**  Two runs (the first text and the one after the
`NIF_*` fix below), both `built src/w32fns.rs -> ../build/femacs/w32fns.o`, **exit 0**, the object
**336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  **42 previous errors** - the standing count
(15 `E0428`, 1 `E0530`, 25 `E0609`, plus the one deliberately-failing frame-flag probe) - and **no
diagnostic sits at or beyond `src/w32fns.rs` 15,546**.  The slice's own two readings: run 1 read
**43** and named a real defect (`nidw.uTimeout = timeout;` - C's anonymous union member is reached
as `nidw.r#DUMMYUNIONNAME.uTimeout` in the port, the `SHELLEXECUTEINFOW`/`INPUT` device), run 2 the
42 above.

**③ A measured blind spot, and what it hid.**  Eight further probe runs were spent on one question
the first readings raised: `NIF_MESSAGE`/`NIF_ICON`/`NIF_TIP` (C 10471) are undefined in the tree,
yet the probe reported nothing.  A deliberate `ZZ_PROBE_UNDEFINED_*` binder was placed in three
items - `add_tray_notification`, `delete_tray_notification` and `w32_console_toggle_lock_key` (the
last landed by slice 21) - and **none produced a diagnostic**, while (a) a deliberately wrong
*field* write (`nidw.zz_no_field_9 = 1;`) in the same body **did** (`E0609`, 43 errors) and (b) an
undefined *macro* is reported (measured by dropping `windowsnt`: 51 `cannot find macro
FRAME_W32_WINDOW` rows and 148 errors).  `-Zunpretty=expanded` (with `RUSTC_BOOTSTRAP=1`) shows the
items and the markers present in the expanded crate, so the items are compiled: **the probe does not
report unresolved *value* names** - the property `tools/check-unit-names.py`'s own docstring records
for `Fx_create_frame` (slice 8), re-measured here on three items.  The gap it hid is exactly the
class the sweep exists for: `check-unit-names.py` on the slice's range reported
`missing: ['NIF_ICON', 'NIF_MESSAGE', 'NIF_TIP']`, and the three constants are now spelled
(`<shellapi.h>`, values corroborated against the Windows metadata projection) - after which the same
sweep reads `missing: []`.  One more note on the reading's shape: `not (feature = "have_dbus")` is
satisfied in this configuration, so the tray items *are* compiled (**the E0609 above proves it**),
and the items' predicates were verified after the investigation (40 two-condition and 37
three-condition `#[cfg]` lines in the file).

**④ `extract-abi.py --check`.**  **90 → 87** rows: three retire (`missing function
add_tray_notification`, `missing function delete_tray_notification` and `missing file-scope object
last_tray_notification_id`) and two token rows were added and cleared by two notes rows
(`add_tray_notification` does not mention `case, default, goto, pnotifyicondataw, switch`; the same
`pnotifyicondataw` for `delete_tray_notification`).  No `line count drifted` row: 15,836 Rust lines
against 12,488 C lines (26.8%), inside the file's `w32fns 0.35 total` tolerance row.

**⑤ The range table and the dead-file gate.**  The `10429-12488` remainder row splits into
`10429-10633` (this slice) and `10635-12488` (the remainder); `check-dead-file-coverage.py` reads
**177 range row(s) cover src/w32fns.c (12488 lines; 126 blank line(s) between blocks need no row)**.

**⑥ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 15,546-15,790: 44 capitalized
identifiers, `missing: []` (before the fix: the three `NIF_*`).

### Deviations (slice 27)

- **`NIF_MESSAGE`/`NIF_ICON`/`NIF_TIP` are spelled here.**  C 10471 uses them and C's file defines
  only `NIF_INFO` (its `#ifndef` guard); the three come from `<shellapi.h>` and are spelled with
  their values (the slice-8 reading for a header constant), corroborated against the Windows
  metadata projection.
- **`nidw.uTimeout` goes through the record's union member.**  C's `_ANONYMOUS_UNION` makes
  `uTimeout`/`uVersion` direct members; the port reaches them as
  `nidw.r#DUMMYUNIONNAME.uTimeout` (the `SHELLEXECUTEINFOW` device), which the first probe run
  caught as an `E0609` before the fix.
- **C's `switch`/`case`/`default` is a `match`, and C's three `goto done` are a labelled block.**
  C's `Ni_Info`/`default` arm keeps C's order (the wildcard last), and `break 'done` replaces the
  three jumps; the notes row records the five tokens this conversion cannot carry.
- **`PNOTIFYICONDATAW` casts stay untyped** (`&raw mut nidw as *mut c_void`) - the same P-72 rule ③
  as the function-pointer type in slice 26.
- **The image loaders are the explicit `W`/`A` pair.**  This unit declares `LoadImage` (the macro,
  `LPCTSTR`-typed); C's two calls here name the entry points, so the pair is declared beside it and
  the `HANDLE` results are cast to `HICON`.
- **C's `errno` is `*errno ()`** (`lib/include/errno.rs`'s `__error` device), and `unsigned timeout`
  is `c_uint`; the counter's `unsigned short` arithmetic is cast at each use.
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no tray icon is added or
  removed and no DLL version is compared; the arm reading is compile-level (42 errors, none from
  this slice), and the *name* reading is the sweep's, not the probe's - this slice measured why.

## Slice 28 (C 10635-10833)

**Scope and the stop.**  The tray block's two public `DEFUN`s - `w32-notification-notify`
(C 10635-10763) and `w32-notification-close` (C 10765-10779) - with the block's `#endif` (C 10781),
then C 10783-10818's `w32-get-ime-open-status` and C 10820-10833's `w32-set-ime-open-status`, which
C 10781's `#endif` puts **outside** the tray block: their gate is the file gate alone.  The stop is
C 10836's `#ifdef WINDOWSNT`, which opens the registry section (C 10834-10835 blank).

**What the item carries.**  `w32-notification-notify` is C 10700-10708's locals (`0, MANY` per
C 10637), C 10711's `maybe_load_unicows_dll`, C 10713-10715's argument guard and
`CHECK_KEYWORD_ARGS`, C 10717's `Flist` and C 10719-10758's five `plist_get` reads - the icon
through `ENCODE_FILE`/`Fexpand_file_name`, the three strings through
`code_convert_string_norecord` with C's `1`, and C 10733-10744's severity ladder - then
C 10761-10762's call and the nil-or-fixnum return.  `w32-notification-close` is C 10775-10776's
guard around `delete_tray_notification`.  The IME pair is C 10799-10802's selected-frame ternary,
C 10806-10811's window/context/two calls with the release between them, C 10813's `retval ? Qt :
Qnil` and C 10829-10831's `unsigned` and `PostThreadMessage (WM_EMACS_IME_STATUS, …)`.

**New surface.**  One in-unit declaration: `plist_get` (`lisp.h` 4280's prototype, whose definition
is `src/w32.c`'s 16056 - the `decode_live_frame` shape).

### Evidence (slice 28, the final revision: 16,010 Rust lines)

**① The frozen configuration (`build-tu`).**  Two runs (the first text and the one after the
`MANY`-parameter fix below), both **exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Both runs read **42 previous errors** - the
standing count (15 `E0428`, 1 `E0530`, 25 `E0609`, plus the one deliberately-failing frame-flag
probe) - so this slice's 174 lines add no error and no member gap, and **no diagnostic sits at or
beyond `src/w32fns.rs` 15,791**.  Slice 27's measured blind spot applies: the probe would not have
reported an unresolved *value* name here, which is why the sweep below ran first.

**③ `extract-abi.py --check`.**  **87 → 83** rows: four `missing subr` rows retire
(`Fw32_notification_notify`, `Fw32_notification_close`, `Fw32_get_ime_open_status`,
`Fw32_set_ime_open_status`) and two rows the first reading produced were fixed rather than recorded:
`DEFUN! Fw32_notification_notify: parameters ['ptrdiff_t', …] but maxargs MANY needs ['isize', …]`
- the gate wants the *mirror's* spelling of C's `ptrdiff_t` (the `Flist`/`alloc.rs` 3025 shape), so
the parameter is `isize` - and `subr Fw32_notification_notify does not mention 1, ni_severity`,
which one `macro-body-notes.tsv` row records (C 10729/10749/10756's `…, 1` is the mirror's `bool`
third argument, the `w32_set_cursor_color` class; C 10707's `enum NI_Severity` is the port's
`c_int`).  No `line count drifted` row: 16,010 Rust lines against 12,488 C lines (28.2%), inside the
file's `w32fns 0.35 total` tolerance row.

**④ The range table and the dead-file gate.**  The `10635-12488` remainder row splits into
`10635-10833` (this slice) and `10836-12488` (the remainder); `check-dead-file-coverage.py` reads
**178 range row(s) cover src/w32fns.c (12488 lines; 128 blank line(s) between blocks need no row)**.

**⑤ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 15,791-15,964: 49 capitalized
identifiers, missing the four `Sw32_*` (`DEFUN!`'s macro-generated statics) and `PORTING` - the
latter an artifact of the sweep's *naive* comment scanner: the `#endif` note below quotes C's own
`#endif  /* WINDOWSNT && !HAVE_DBUS */`, whose inner `*/` ends the sweep's comment early, so the
tail of that note reads as code (the tool's docstring lists "nested block comments" as a known
limitation; Rust nests block comments, so the text is correct as it stands).

### Deviations (slice 28)

- **`plist_get` is declared in this unit.**  `lisp.h` 4280 declares it and `include/lisp.rs` does not
  carry that prototype yet, so the unit declares it with C's signature (the `decode_live_frame`
  shape of slice 22); its definition stays `src/w32.c`'s.
- **The `MANY` `DEFUN` takes `isize`, not `ptrdiff_t`.**  C 10701's `ptrdiff_t nargs` is the same
  type and the mirror spells it `isize`; the P-28 gate compares the parameter list against the
  generated prototype's spelling, and the file's other `MANY` `DEFUN`s (e.g. `Flist`) do the same.
- **C's `…, 1` third argument to `code_convert_string_norecord` is `true`.**  The mirror declares
  that parameter `bool` (the `w32_set_cursor_color` row's class), recorded in a notes row.
- **C's `enum NI_Severity` local is `c_int`** (its members are slice 26's `Ni_*` constants).
- **The `#endif` note quotes C's line containing `/* … */`.**  Rust's block comments nest, so the
  text is legal; it makes the name sweep report `PORTING` (documented above).
- **The IME pair carries the file gate alone.**  C 10781's `#endif` closes
  `WINDOWSNT && !HAVE_DBUS`, so C 10783+ are outside it, and their items' `#[cfg]` says so.
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no notification is shown and
  no IME state is read or set; the arm reading is compile-level (42 errors, none from this slice),
  the name reading is the sweep's, and the `plist_get` call is a cross-unit symbol this lane does not
  link.

## Slice 29 (C 10836-10957)

**Scope and the stop.**  The registry pair: C 10836's `#ifdef WINDOWSNT` with C 10837-10839's banner
and `w32-read-registry` (C 10840-10900, closed by C 10902's `#endif`), then `w32_get_resource`
(C 10904-10956), which stands **outside** that `#ifdef` and therefore carries the file gate alone.
The stop is C 10958's `#ifdef WINDOWSNT`, which opens the wallpaper/idle-time section.

**What the item carries.**  `w32-read-registry` is C 10877-10879's three checks, C 10881-10893's
root-key ladder (`HKEY_CLASSES_ROOT`/`HKEY_USERS`/`HKEY_CURRENT_CONFIG` are new constants in the
C 890-932 block above) with C 10893's `error` over `SDATA (SYMBOL_NAME (root))`, C 10895's first
`w32_read_registry` call (w32.h 253's, declared by `include/w32.rs`) and C 10896-10897's nil-root
retry under `HKEY_LOCAL_MACHINE`.  `w32_get_resource` is C 10914-10916's three locals and two
identical blocks (C 10921-10936 for `HKEY_CURRENT_USER`, C 10938-10953 for `HKEY_LOCAL_MACHINE`),
each one `RegOpenKeyEx`, the two `RegQueryValueEx` calls, `xfree` and `RegCloseKey`, with C's
`(lpvalue = xmalloc (cbData)) != NULL` written as the port's `&& { … }` block.

**New surface.**  `HKEY_CLASSES_ROOT` (0x80000000), `HKEY_USERS` (0x80000003),
`HKEY_CURRENT_CONFIG` (0x80000005) and the `RegQueryValueEx` declaration, all added to the existing
C 890-932 `<winreg.h>` block (next to `RegOpenKeyEx`/`RegEnumValueA`/`RegCloseKey`) - the file's
position for a header name.

**The mirror's second latent collision is removed.**  `include/cygw32.rs` carried C 37-38's
`w32_strerror`/`w32_get_resource` declarations at its crate root, and `src/w32fns.rs` 172-173
includes that mirror under `all (have_ntgui, cygwin)` - so a Cygwin GUI build would have read an
`E0428` per name against this unit's definitions (slice 24's and this slice's), which the arm reading
cannot see because its configuration has no `cygwin`.  Both now stand in a `pub mod cygw32_decls` +
`use cygw32_decls::*;` (P-21/P-39, the device `include/w32common.rs` and `include/w32term.rs` use);
`syms_of_cygw32` stays at the crate root.  The mirror's file note ("no unit `include!`s this mirror")
is stale and is corrected in place; the mirror's own ledger has the section.

### Evidence (slice 29, the final revision: 16,137 Rust lines; the mirror 17 lines longer)

**① The frozen configuration (`build-tu`).**  One run: **exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  One run: **42 previous errors** - the standing
count (15 `E0428`, 1 `E0530`, 25 `E0609`, plus the one deliberately-failing frame-flag probe) - so
this slice's 127 lines add no error and no member gap, and **no diagnostic sits at or beyond
`src/w32fns.rs` 15,980**.

**③ The name sweeps (`check-unit-names.py`, which is the only reading that sees unresolved values -
slice 27's finding).**  The slice's text (`src/w32fns.rs` 15,980-16,091): 30 capitalized
identifiers, missing `Sw32_read_registry` (a `DEFUN!`'s macro-generated static) and `PORTING` (the
sweep's naive comment scanner ends my `#endif` note early at the `*/` it quotes - the documented
nested-comment limitation; Rust nests block comments).  The C 890-932 additions
(`src/w32fns.rs` 1,692-1,716): 13 capitalized identifiers, `missing: []`.

**④ `extract-abi.py --check`.**  **83 → 81** rows: `missing function w32_get_resource` and
`missing subr Fw32_read_registry` retire, none is added, and **no notes row was needed**: C 10925/10942
write the SDK's alias `RegQueryValueEx`, so the port declares that *name* as the ANSI function - the
device the same block already uses for `RegOpenKeyEx` (its comment: "C's text writes the alias and the
port keeps C's name").  No `line count drifted` row: 16,137 Rust lines against 12,488 C lines (29.2%),
inside the file's `w32fns 0.35 total` tolerance row.

**⑤ The range table and the dead-file gate.**  The `10836-12488` remainder row splits into
`10836-10957` (this slice) and `10958-12488` (the remainder); `check-dead-file-coverage.py` reads
**179 range row(s) cover src/w32fns.c (12488 lines; 128 blank line(s) between blocks need no row)**.

### Deviations (slice 29)

- **`include/cygw32.rs`'s two declarations move into `cygw32_decls`.**  See above; the mirror's
  ledger has the note and the mirror pair's checklist is unchanged.
- **`RegQueryValueEx` is declared under C's own name** (not `RegQueryValueExA`): the C 890-932
  block's convention for an SDK alias, taken from its `RegOpenKeyEx` (and `RegEnumValueA` is spelled
  with its suffix there because C's *call site* names that one).
- **C's banner and the two `#endif`s are wrapper comments.**  C 10837-10839's `/***…***/` banner and
  C 10902's `#endif` have no items of their own; they stand inside `/* … */` wrappers (the
  C 8116-8118 device), which is also what makes the name sweep report `PORTING` for this range.
- **C's assignment-in-condition is the port's `&& { … }` block** (`(lpvalue = xmalloc (cbData)) !=
  NULL`), and the two pointers are cast where C converts implicitly (`as LPBYTE`, `as *mut c_void`
  for `xfree`).
- **The new header names join the C 890-932 block**, not the C 10881/10925 call sites - the file's
  position for `<winreg.h>`'s names.
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no registry key is opened and
  no value is read; `w32_read_registry` is another unit's definition (this lane does not link), the
  arm reading is compile-level (42 errors, none from this slice), and the name reading is the
  sweep's.  The Cygwin GUI arm was not compiled here at all - the mirror move above is a reading of
  the *text*, not of that arm.

## Slice 30 (C 10958-11041)

**Scope and the stop.**  C 10958's `#ifdef WINDOWSNT` with C 10960-10962's wallpaper banner and
`w32-set-wallpaper` (C 10964-11015), then the idle-time pair - C 11017's comment, C 11018-11019's
pointer and `w32-system-idle-time` (C 11021-11039) - closed by C 11041's `#endif`.  The stop is
C 11044's `#ifdef WINDOWSNT`, which opens the badge/taskbar section.  Both items carry
`all (feature = "have_ntgui", feature = "windowsnt")`.

**What the item carries.**  `w32-set-wallpaper` is C 10971-10974's four locals, C 10976-10979's
comment, C 10980-10989's `w32_unicode_filenames` + `system_parameters_info_w_fn` arm (the `wchar_t`
buffer, `filename_to_utf16`'s failure setting `ERROR_FILE_NOT_FOUND`, C 10987-10988's explicit
`SystemParametersInfoW` call), C 10990-10999's ANSI arm, and C 11000-11012's three-way report
(`error` over `fname`, then `w32_strerror (GetLastError ())` or the wrong-image-type message).
`w32-system-idle-time` is C 11028-11029's `LASTINPUTINFO` with C's `sizeof info`, C 11031's pointer
test and call, C 11033's `GetTickCount () - info.dwTime` and C 11034-11038's two returns.

**New surface.**  `SystemParametersInfoW_Proc` and `GetLastInputInfo_Proc` with their two file-scope
pointers, `SystemParametersInfoW`/`SystemParametersInfoA` (the alias `SystemParametersInfo` is
declared in the C 1997 block above), `GetLastInputInfo`, `GetTickCount`, `LASTINPUTINFO` with
`PLASTINPUTINFO` (C 11028's record), and the two constants `SPI_SETDESKWALLPAPER` (0x0014) and
`SPIF_SENDCHANGE` (0x2), all corroborated against the Windows metadata projection.

### Evidence (slice 30, the final revision: 16,294 Rust lines)

**① The frozen configuration (`build-tu`).**  One run: **exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  One run: **42 previous errors** - the standing
count (15 `E0428`, 1 `E0530`, 25 `E0609`, plus the one deliberately-failing frame-flag probe) - so
this slice's 157 lines add no error and no member gap, and **no diagnostic sits at or beyond
`src/w32fns.rs` 16,092**.

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 16,092-16,248: 33 capitalized
identifiers, missing `Clone`/`Copy` (the new record's derive arguments) and the two `Sw32_*`
(`DEFUN!`'s macro-generated statics) - the documented non-item classes; no unresolved value.

**④ `extract-abi.py --check`.**  **81 → 77** rows: `missing subr Fw32_set_wallpaper` and
`missing subr Fw32_system_idle_time`, plus the two `missing file-scope object` rows
(`system_parameters_info_w_fn`, `get_last_input_info_fn`), retire; one token row appeared
(`subr Fw32_set_wallpaper does not mention false` - C 10973's `BOOL result = false;` keeps C's
`BOOL`, so the port writes `0`) and one `macro-body-notes.tsv` row records it.  No
`line count drifted` row: 16,294 Rust lines against 12,488 C lines (30.5%), inside the file's
`w32fns 0.35 total` tolerance row.

**⑤ The range table and the dead-file gate.**  The `10958-12488` remainder row splits into
`10958-11041` (this slice) and `11044-12488` (the remainder); `check-dead-file-coverage.py` reads
**180 range row(s) cover src/w32fns.c (12488 lines; 130 blank line(s) between blocks need no row)**.

### Deviations (slice 30)

- **C's `false` on a `BOOL` is `0`** (C 10973), recorded in a notes row (the `Fw32_frame_edges` class).
- **C's `GetTickCount () - info.dwTime` keeps its wrap** as `wrapping_sub` (`src/android.rs` 5352's
  reading of C's unsigned arithmetic), because the subtraction is a DWORD difference that C wraps.
- **The `EMACS_INT_MAX` guard widens the DWORD first** (`time_since_last_input as EMACS_INT >
  EMACS_INT_MAX`), which is C's own promotion; on this target the guard is never true.
- **C's banner is a wrapper comment** (the C 8116-8118 device), and C 11041's `#endif` has no item.
- **The two hand-loaded pointers are file-scope statics here**; `globals_of_w32fns` (C 12460-12475)
  is what loads them, which is still unported.
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no wallpaper is set and no
  idle time is read; the arm reading is compile-level (42 errors, none from this slice) and the name
  reading is the sweep's.

## Slice 31 (C 11044-11245)

**Scope and the stop.**  C 11044's `#ifdef WINDOWSNT`, C 11046-11048's Taskbar Indicators banner,
C 11050-11054's `#ifndef MINGW_W64` guard carrying the SDK's two `DEFINE_GUID`s, and `w32-badge`
(C 11056-11245).  The stop is C 11247's `DEFUN ("w32-request-user-attention", ...)`, the second of
the taskbar trio.  Every item carries `all (feature = "have_ntgui", feature = "windowsnt")`.

**What the item carries.**  `w32-badge` is C 11068-11075's selected-frame resolution (the
`FRAME_W32_P`/`FRAME_LIVE_P` pair, `hwnd` from `FRAME_W32_WINDOW`, C 11071's `Qnil` on a NULL
handle), C 11077-11085's `CoInitialize`/`CoCreateInstance` pair with C 11084's `r != S_OK` test,
C 11087-11100's badge/colour parsing (C's two `parse_color_spec` calls with their out-parameters and
the two `RGB` macros, each failure returning `Qnil`), C 11102-11110's UTF-8 badge (`ENCODE_UTF_8`,
the sizing `pMultiByteToWideChar` call, C 11107's `alloca` buffer, the filling call),
C 11113-11116's `SM_CXSMICON` pair and the `GetDC`/`CreateCompatibleDC` pair, C 11118-11134's
`BITMAPV5HEADER` setup (C's `memset`, `bV5Size`, the negative `bV5Height` for a top-down DIB, the
four channel masks) with the `CreateDIBSection` call and `SelectObject`, C 11136-11141's circle
(`CreateSolidBrush`/`Ellipse`/restore/`DeleteObject`), C 11144-11163's font derivation (the
`system_parameters_info_w_fn` arm with C's `NONCLIENTMETRICS` `memset`/`cbSize`,
`SystemParametersInfo`, `CreateFontIndirect` and the `clean_up_base_font` flag, against the
`GetStockObject (DEFAULT_GUI_FONT)` fallback), C 11165-11180's `GetObject` + `LOGFONT` edits
(`FW_BOLD`, `OUT_OUTLINE_PRECIS`, `ANTIALIASED_QUALITY`, the `MulDiv`/`GetDeviceCaps (LOGPIXELSY)`
height and the `SetMapMode` save/restore), C 11182-11193's text draw (`SetBkMode`/`SetTextColor`,
the `RECT`, `DrawText` with the four `DT_*` flags), C 11195-11214's alpha walk (the circle's
centre/radius arithmetic and the per-pixel `|= 0xff000000`), C 11216-11238's icon assembly
(`CreateBitmap` mask, the `SelectObject` restore C's comment calls for, `ICONINFO`,
`CreateIconIndirect`, `SetOverlayIcon`, then the five releases), and C 11240-11243's `else` arm
(`SetOverlayIcon (..., NULL, NULL)`) with the final `Release` and `Qt`.

**New surface.**  The two `GUID` objects; the constants `CLSCTX_INPROC_SERVER`, `SM_CXSMICON`,
`SPI_GETNONCLIENTMETRICS`, `BI_BITFIELDS`, `DIB_RGB_COLORS`, `MM_TEXT`, `TRANSPARENT`,
`DEFAULT_GUI_FONT`, `LOGPIXELSY`, `OUT_OUTLINE_PRECIS`, `ANTIALIASED_QUALITY` and the four `DT_*`
flags; the COM/GDI entry points `CoInitialize`, `CoCreateInstance`, `CreateCompatibleDC`,
`CreateDIBSection`, `CreateSolidBrush`, `Ellipse`, `GetStockObject`, `SetMapMode`, `SetBkMode`,
`SetTextColor`, `DrawText` (declared under C's alias name - the C 890-932 `RegOpenKeyEx`
convention), `CreateBitmap`, `CreateIconIndirect` and `MulDiv`; and the records `BITMAPV5HEADER`
(24 fields), `RGBQUAD`, `BITMAPINFOHEADER`, `BITMAPINFO`, `ICONINFO`, `NONCLIENTMETRICS` (the `A`
form, 15 fields) and `ITaskbarList3Vtbl`/`ITaskbarList3`.

### Evidence (slice 31, the final revision: 16,815 Rust lines)

**① The frozen configuration (`build-tu`).**  Exit 0, the object **336 bytes** - after a first
reading refused the file: the slice's `DEFUN!` ended at its body's `}` and lost the invocation's
`);`, so the whole-file paren count was short one and the parse stopped at the invocation (`build-tu`
caught it; the probe would not have).  One character pair added; the second reading is exit 0.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Two runs.  The first read **46** errors - the
standing **42** plus four `E0381` from this slice (C's `unsigned short r, g, b;` and `DWORD
*bitmap_pixels;` are written by the callee, which Rust's definite-initialization rejects).
Initialized to `0`/`core::ptr::null_mut ()` in the two shapes the file already uses
(`x_to_w32_color`'s `= 0`; the `*mut` locals' `= core::ptr::null_mut ()`), the second run is back to
**42**: this slice adds no member gap and no type error, and **no error sits at or beyond
`src/w32fns.rs` 16,249** (the largest is the pre-existing `must_be_updated_p` gap at 12,083).

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 16,249-16,815: 126 capitalized
identifiers, the 29 missing all in the documented non-item classes - `Clone`/`Copy` (the new
records' derive arguments), `Data1`-`Data4` (`GUID`'s field names), the 22 vtable field names plus
`This` (the `ITaskbarList3Vtbl` fields and the declaration parameters), and `Sw32_badge`
(`DEFUN!`'s macro-generated static).  No unresolved value.

**④ `extract-abi.py --check`.**  **77 → 76** rows: `missing subr Fw32_badge` retires; one token row
appeared (`subr Fw32_badge does not mention alloca, false, true`) and one `macro-body-notes.tsv`
row records it.  No `line count drifted` row: 16,815 Rust lines against 12,488 C lines (34.6%),
inside the file's `w32fns 0.35 total` tolerance row - this is the last slice that fits under it.

**⑤ The range table and the dead-file gate.**  The `11044-12488` remainder row splits into
`11044-11245` (this slice) and `11247-12488` (the remainder); `check-dead-file-coverage.py` reads
**181 range row(s) cover src/w32fns.c (12488 lines; 131 blank line(s) between blocks need no row)**.

### Deviations (slice 31)

- **C's `TRUE`/`FALSE` on a `BOOL` are `1`/`0`** (C 11149, 11157, 11227), recorded in a notes row
  (the `serial_configure` reading of those two tokens).
- **C's vtable calls keep C's `lpVtbl` walk** with an `Option` unwrap per method:
  `(*((*task_bar_list).lpVtbl)).SetOverlayIcon.unwrap () (...)` where C writes
  `task_bar_list->lpVtbl->SetOverlayIcon (...)`, and `Release` likewise - the
  `pfnShell_NotifyIconW` device for a function-pointer member.
- **`ITaskbarList3Vtbl` is spelled from the documented method order and is unmeasured here.**
  `windows-sys` 0.52 carries no `ITaskbarList3`, so this machine has no authority for the 22
  entries; the record's doc comment and this section say so, and the slots the port actually walks
  are the two C calls plus `Release`.
- **`alloca` is `sa_stack_alloc`** (C 11107, PORTING.md P-05), recorded in the notes row.
- **`DrawText` is declared under C's alias name** (the SDK's `DrawTextA`, which is what C's `char`
  badge wants - the C 890-932 `RegOpenKeyEx` device), so no notes row is needed (the
  `RegQueryValueEx` class).
- **C's banner is a wrapper comment** (the C 8116-8118 device), and C 11044's `#ifdef` and
  C 11046-11048's banner text have no item of their own beyond the arm pair.
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no overlay icon is drawn and
  no badge is set; the arm reading is compile-level (42 errors, none from this slice) and the name
  reading is the sweep's.

## Slice 32 (C 11247-11348)

**Scope and the stop.**  The two remaining taskbar DEFUNs - C 11247-11295's
`w32-request-user-attention` and C 11297-11346's `w32-progress-indicator` - closing the
badge/taskbar `#ifdef WINDOWSNT` with C 11348's `#endif`.  The stop is C 11350's System Sleep
Support banner, which opens the next group.  Both items carry
`all (feature = "have_ntgui", feature = "windowsnt")`.

**What the items carry.**  `w32-request-user-attention` is C 11262-11264's `flash_window_ex_fn` NULL
test (the pointer `globals_of_w32fns` loads), C 11266-11275's selected-frame resolution (the same
shape `w32-badge` carries), C 11277-11283's `FLASHWINFO` field run with C's `sizeof(flash_info)`,
C 11284-11292's three `dwFlags` arms (`Qinformational` -> `FLASHW_TRAY | FLASHW_TIMERNOFG`,
`Qcritical` -> `FLASHW_ALL | FLASHW_TIMERNOFG`, otherwise `FLASHW_STOP`), and C 11294's call with
`Qt`.  `w32-progress-indicator` is the same resolution, C 11323-11330's
`CoInitialize`/`CoCreateInstance` pair with C 11329's `r != S_OK`, C 11332-11335's scaling
(`ULONGLONG adj_progress = 0`, the `FLOATP` test and C's `(ULONGLONG) (100.0 * XFLOAT_DATA
(progress))`), C 11336-11344's two arms (`TBPF_NORMAL` + `SetProgressValue (..., adj_progress,
100)` against `TBPF_NOPROGRESS`), then the `Release` and `Qt`.

**New surface.**  The `FLASHWINFO` record (five fields) and the four `FLASHW_*` flags, and the two
`TBPFLAG` members `TBPF_NORMAL`/`TBPF_NOPROGRESS` - the SDK's enum read as the `c_int` this unit's
`ITaskbarList3Vtbl.SetProgressState` takes.  Both `Q` symbols come from the mirror
(`include/globals.rs` 1052/601), as do `SYMBOLP`, `EQ`, `FLOATP` and `XFLOAT_DATA`
(`include/lisp.rs`).

### Evidence (slice 32, the final revision: 16,963 Rust lines)

**① The frozen configuration (`build-tu`).**  One run: **exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  One run: **42 previous errors** - the standing
count - so this slice adds no error and no member gap, and **no error sits at or beyond
`src/w32fns.rs` 16,770** (the largest is the pre-existing `must_be_updated_p` gap at 12,083).

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 16,770-16,917: 39 capitalized
identifiers, missing only `Clone`/`Copy` (the new record's derive arguments) and the two `Sw32_*`
(`DEFUN!`'s macro-generated statics) - the documented non-item classes; no unresolved value.

**④ `extract-abi.py --check`.**  **76 → 74** rows: `missing subr Fw32_request_user_attention` and
`missing subr Fw32_progress_indicator` retire, and **no token row appears** - every token of both
bodies has a counterpart.  The drift row the first reading added (`line count drifted 35.8%: C
12488 vs Rust 16963`) is answered the way the table's own rule demands: a new `w32fns` row is
appended to `docs/ports/line-drift-tolerance.tsv` (`0.37` = the measured 35.8% + 0.01, the
`xfns` 0.41 row's rule), superseding the slice-17 `0.35`, and the second reading has no drift row.

**⑤ The range table and the dead-file gate.**  The `11247-12488` remainder row splits into
`11247-11348` (this slice) and `11350-12488` (the remainder, 1,139 lines);
`check-dead-file-coverage.py` reads **182 range row(s) cover src/w32fns.c (12488 lines; 132 blank
line(s) between blocks need no row)**.  The over-sequence comment above the
`w32_console_toggle_lock_key` block now reads `C 11350-12488`.

### Deviations (slice 32)

- **C's `FLASHWINFO flash_info;` keeps C's field-only initialisation** as
  `let mut flash_info: FLASHWINFO = core::mem::zeroed ();` (the `LASTINPUTINFO` device, slice 30) -
  Rust's definite-initialization would otherwise reject C's partial writes.
- **C's nullable function pointer is an `Option`**: C's `if (flash_window_ex_fn == NULL)` is
  `flash_window_ex_fn.is_none ()` and the call is `(flash_window_ex_fn.unwrap ()) (...)`, the
  `pfnShell_NotifyIconW`/`system_parameters_info_w_fn` device this file has used since slice 26.
- **C's `TBPFLAG` enum is the `c_int` the unit's vtable slot takes** (the `NI_Severity` class), so
  the two constants are `c_int`, not a Rust `enum`.
- **C's two vtable calls keep C's `lpVtbl` walk** with an `Option` unwrap per method (slice 31's
  reading).
- **`FLASHW_ALL` is spelled `0x00000003`**, C's `(FLASHW_CAPTION | FLASHW_TRAY)` folded.
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no taskbar icon flashes and
  no progress bar appears; the arm reading is compile-level (42 errors, none from this slice) and
  the name reading is the sweep's.

## Slice 33 (C 11350-11477)

**Scope and the stop.**  The System Sleep Support group - C 11350-11352's banner, C 11354-11358's
`SetThreadExecutionState_Proc` with its pointer and the two counters, the three `DEFUN`s
(C 11360/11391/11414), and the notification half (C 11424-11477).  The stop is C 11478's Exporting
frames banner, which opens the next group.  **Every item carries the file gate alone
(`#[cfg (feature = "have_ntgui")]`)** - the group sits outside every `#if`, which the
measurement below records - and the `input_event_bits!` macro carries it too (its
`src/w32inevt.rs` 644 twin is `windowsnt`'s because *that* unit's items are inside the
`#ifdef`).

**What the items carry.**  `w32-block-system-sleep` is C 11370-11371's pointer guard, C 11373's
`ES_CONTINUOUS` comment, C 11374-11376's `ES_SYSTEM_REQUIRED | ES_CONTINUOUS` initialiser and the
`NILP` test that adds `ES_DISPLAY_REQUIRED`, and C 11378-11387's two arms (`++sleep_block_count` and
`++sleep_block_id` as two statements with the synthesised token, against the failed call's `Qnil`).
`w32-unblock-system-sleep` is the same guard, C 11396-11397's empty-count test, and C 11401-11410's
`--sleep_block_count == 0` paired with the `ES_CONTINUOUS` clear.  `w32-system-sleep-block-count` is
C 11420's one return.  `sleep_notification_callback` is C 11429-11432's `input_event` with
`EVENT_INIT` and `ie.kind = SLEEP_EVENT`, then C 11434-11447's three-arm `switch` on `Type`.
`w32_register_for_sleep_notifications` is C 11463-11464's comment, the two `params` members and
C 11473-11474's `RegisterSuspendResumeNotification_fn (&params, 2)`.

**New surface.**  `SetThreadExecutionState_Proc`/`SetThreadExecutionState_fn`, `sleep_block_id`,
`sleep_block_count`; `EXECUTION_STATE` with `ES_SYSTEM_REQUIRED`/`ES_DISPLAY_REQUIRED`/
`ES_CONTINUOUS` and the `SetThreadExecutionState` entry point; `PMY_DEVICE_NOTIFY_CALLBACK_ROUTINE`
with the three `PBT_*` labels; `RegisterSuspendResumeNotification_Proc`,
`RegisterSuspendResumeNotification_fn`, `HPOWERNOTIFY`, `sleep_notification_handle`,
`MY_DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS`; and the per-unit `input_event_bits!` macro.
`input_event`/`EVENT_INIT`/`SLEEP_EVENT` arrive through `include/frame.rs` 24's `termhooks.rs`
include, and `Qpre_sleep`/`Qpost_wake` are the mirror's (`include/globals.rs` 1517/1513).

### Evidence (slice 33, the final revision: 17,208 Rust lines)

**① The frozen configuration (`build-tu`).**  One run: **exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Two runs.  The first read **44** errors - the
standing **42** plus two this slice's own: the `input_event_bits!` call preceded its definition
(`macro_rules!` is textual; the macro is now above the callback, which is where
`src/w32inevt.rs` 644 puts its copy) and `E0381 partially assigned binding params isn't fully
initialized` (measured: Rust rejects C's write-the-two-members form; the port uses the file's
`core::mem::zeroed ()` device).  The second run is back to **42** - no member gap, no type error -
and **no error sits at or beyond `src/w32fns.rs` 16,918** (the largest is the pre-existing
`must_be_updated_p` gap at 12,083).

**②b The arm gate, corrected after the round (the same revision, 17,208 lines).**  The slice
first landed its twenty-four items with `all (feature = "have_ntgui", feature = "windowsnt")`
(and the macro with `windowsnt`), which is the gate of slices 31-32 - but the measurement says
the group is **outside every `#if`**: scanning `src/w32fns.c`'s preprocessor stack shows the
`#ifdef WINDOWSNT` of C 11044 closing at C 11348, so C 11350's banner and everything under it
carry no condition at all, and the file's own convention for such an item is the file gate
alone (`w32_get_resource`, C 10904, is the neighbouring example).  All twenty-four gates became
`#[cfg (feature = "have_ntgui")]`; the three readings are unchanged by the correction
(`build-tu` exit 0 with the 336-byte object, the probe's **42**, `--check`'s **64** rows), which
is why the first reading could not see it.

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 16,918-17,161: 44 capitalized
identifiers, the nine missing all in the documented non-item classes - `Clone`/`Copy` (the new
record's derive arguments), `Callback`/`Context` (its field names), `Setting`/`Type` (the callback
typedef's parameter names) and the three `Sw32_*` (`DEFUN!`'s macro-generated statics).  No
unresolved value.

**④ `extract-abi.py --check`.**  **74 → 64** rows: three `missing subr`
(`Fw32_block_system_sleep`, `Fw32_unblock_system_sleep`, `Fw32_system_sleep_block_count`), two
`missing function` (`sleep_notification_callback`, `w32_register_for_sleep_notifications`) and five
`missing file-scope object` (`SetThreadExecutionState_fn`, `sleep_block_id`, `sleep_block_count`,
`RegisterSuspendResumeNotification_fn`, `sleep_notification_handle`) retire.  The first reading
added two rows, both answered: a token row (`function sleep_notification_callback does not mention
break, case, switch` - the `match` substitution, one new `macro-body-notes.tsv` row) and a drift row
(`line count drifted 37.8%: C 12488 vs Rust 17208` - a new `w32fns 0.39` row in
`docs/ports/line-drift-tolerance.tsv`, the measured 37.8% + 0.01, superseding slice 32's `0.37`).

**⑤ The range table and the dead-file gate.**  The `11350-12488` remainder row splits into
`11350-11477` (this slice) and `11478-12488` (the remainder, 1,011 lines; the first attempt wrote
`11479` and the gate caught C 11478's banner text);
`check-dead-file-coverage.py` reads **183 range row(s) cover src/w32fns.c (12488 lines; 132 blank
line(s) between blocks need no row)**.  The over-sequence comment above the
`w32_console_toggle_lock_key` block now reads `C 11479-12488`.

### Deviations (slice 33)

- **C's `switch` is the port's `match` with a trailing wildcard** standing in for C's absent
  `default` (the `add_tray_notification` reading), recorded in the new notes row.
- **The `input_event_bits!` copy sits above its first use.**  `macro_rules!` is textual, so the
  macro cannot stand at the C `#define`'s position; the unit's items keep C's order (P-91 ②).
- **`ALIGN_STACK! ()` stands at the top of the callback's body** - a Rust item cannot carry
  `__attribute__((force_align_arg_pointer))`, which `include/w32term.rs` 52-67 already records as
  carried by the empty macro.
- **`params` is `core::mem::zeroed ()` first.**  C writes both members of an uninitialised local;
  measured, Rust rejects that form (`E0381 ... partial initialization isn't supported`), so the port
  uses the same device `FLASHWINFO`/`LASTINPUTINFO` use.
- **C's `--sleep_block_count == 0` keeps its pre-decrement** in a block expression
  (`{ sleep_block_count -= 1; sleep_block_count == 0 }` - the `pgtk_hide_hourglass` device), and
  C's `++x;` statements are `x += 1;`.
- **C's `HPOWERNOTIFY sleep_notification_handle = 0` is `core::ptr::null_mut ()`** (the
  `hinst` device; `HPOWERNOTIFY` is `HANDLE`).
- **Not claimed.**  No run evidence: no Windows, no mingw toolchain, so no thread execution state is
  set and no power notification is registered; the arm reading is compile-level (42 errors, none
  from this slice) and the name reading is the sweep's.

## Slice 34 (C 11478-11520)

**Scope and the stop.**  The Exporting frames group - C 11478-11480's banner and `w32-export-frame`
(C 11481-11520), whose body carries its own `#ifdef WINDOWSNT` (C 11491) with the `#else`/
`return Qnil;` arm (C 11517-11518).  The stop is C 11522's Initialization banner.  The item sits
outside every other `#if`, so it carries the file gate alone.

**What the item carries.**  C 11492's live-frame decode, C 11494-11495's optional-argument default
(`type = Qjpeg`), C 11497-11500's two visibility tests with their `error` calls, C 11502-11503's
redisplay call, C 11505-11514's GDI+ run (`FRAME_W32_WINDOW`, `result = -1`, the `CLSID` local, the
`block_input`/`unblock_input` pair, the `w32_gdiplus_startup () && frame_hwnd != NULL &&
w32_gdip_get_encoder_clsid (...)` chain and the export call), and C 11516's `result >= 0 ? Qt :
Qnil`.  The four SDK names come from the mirrors (`w32gui.rs` 101's `w32_gdiplus_startup`,
`w32gdiplus.rs` 392-395's two `w32_gdip_*`), and `redisplay_preserve_echo_area`
(`dispextern.h` 3589) is declared at the use site because no mirror carries the row - the
`src/xfns.rs` 14148 device.

### Evidence (slice 34, the final revision: 17,273 Rust lines)

**① The frozen configuration (`build-tu`).**  Two runs (the second after the `r#type` fix): **exit 0**,
the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Two runs.  The first read **45** errors - the
standing **42** plus `expected identifier, found keyword type` (C names the parameter `type`, which
is a Rust keyword: the port spells it `r#type`, the `src/alloc.rs` 3876 / `src/androidfns.rs` 2216
device, and `extract-abi.py` strips the prefix when it compares bodies).  The second run is back to
**42** - so this slice adds no member gap and no type error, and **no error sits at or beyond
`src/w32fns.rs` 17,163** (the largest is the pre-existing `must_be_updated_p` gap at 12,083).

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 17,163-17,227: 13 capitalized
identifiers, the one missing being `Sw32_export_frame` (`DEFUN!`'s macro-generated static) - no
unresolved value.

**④ `extract-abi.py --check`.**  **64 → 63** rows: `missing subr Fw32_export_frame` retires, and
**no token row appears** - every token of the body has a counterpart, including C's
`&image_type_clsid` (the mirror types the two `w32_gdip_*` parameters `*mut c_void`, so the port
casts).  No drift row: 17,273 Rust lines against 12,488 C lines (38.3%), inside the `w32fns 0.39`
row appended in slice 33.

**⑤ The range table and the dead-file gate.**  The `11478-12488` remainder row splits into
`11478-11520` (this slice) and `11522-12488` (the remainder, 967 lines);
`check-dead-file-coverage.py` reads **184 range row(s) cover src/w32fns.c (12488 lines; 133 blank
line(s) between blocks need no row)**.  The over-sequence comment above the
`w32_console_toggle_lock_key` block now reads `C 11522-12488`.

### Deviations (slice 34)

- **C's `type` parameter is `r#type`** (a Rust keyword), in the DEFUN's argument list and in the
  body; `extract-abi.py` strips the prefix, so the token comparison is unaffected.
- **C assigns to the parameter, so it is re-bound** (`let mut r#type = r#type;`), because
  `DEFUN!`'s `$arg:ident` cannot say `mut` - the `src/androidfns.rs` 3029 reading.
- **The body's two arms are sibling `#[cfg]` blocks** (`windowsnt` / `not (windowsnt)`), the
  `src/callproc.rs` 1828-1831 device, and the `#else`'s bare `return Qnil;` is the `not` arm.
- **`image_type_clsid` is `core::mem::zeroed ()` first** - C declares it and writes it through the
  callee; the same device `FLASHWINFO`/`LASTINPUTINFO` use.
- **The two `w32_gdip_*` calls cast their CLSID argument to `*mut c_void`**, because the mirror
  spells those parameters untyped (P-55/P-72 rule 3); C's own `&image_type_clsid` is unchanged in
  meaning.
- **`FRAME_VISIBLE_P`/`FRAME_WINDOW_P` are the mirror's predicated forms** (C's `!FRAME_VISIBLE_P
  (f)` is `! FRAME_VISIBLE_P (f)`), and the two `error` literals are `c"..."` byte strings - the
  file's device since slice 1.
- **Not claimed.**  No run evidence: no Windows, no GDI+ and no mingw toolchain, so no frame is
  exported and no encoder CLSID is looked up; the arm reading is compile-level (42 errors, none from
  this slice) and the name reading is the sweep's.

## Slice 35 (C 11522-11583)

**Scope and the stop.**  The Initialization banner (C 11523-11525), C 11527-11528's ordering
comment and `w32_frame_parm_handlers[]` (C 11530-11583, fifty-one entries).  The stop is C 11585's
`syms_of_w32fns`.  The group sits outside every `#if`, so the items carry the file gate.

**What the items carry.**  The table is C's fifty-one entries in C's order, with C's five `0` rows as
`None` and C's trailing comments kept; the twenty-three `w32_set_*`/`w32_explicitly_set_name` entries
are this file's own definitions and the twenty-two `gui_set_*` entries are `frame.c`'s, named in a
use-site block because `include/frame.rs` does not carry `frame.h` 1844-1871's prototypes.

**The mirror move (P-21/P-39).**  `w32term.h` 274's `extern frame_parm_handler
w32_frame_parm_handlers[];` stood at `include/w32term.rs`'s crate root (line 353), where a
`have_ntgui` unit that defines the object reads `E0428`.  The declaration moved into the existing
`pub mod w32term_objs` (which the mirror glob-imports, so the unit's crate-root definition shadows
it), its `/// C:` line moved with it, and the old site carries a one-line pointer comment - the
device `docs/ports/include-w32term.md` records for the twenty-seven names of 2026-10-05.  The
conversion of this file's twenty-three handlers to `unsafe extern "C" fn` is the same table's doing:
their address now goes into a C-ABI slot, which is how `src/haikufns.rs`, `src/pgtkfns.rs` and
`src/androidfns.rs` spell their own handlers.

### Evidence (slice 35, the final revision: 17,407 Rust lines)

**① The frozen configuration (`build-tu`).**  Two runs (the second after the ABI conversion): **exit
0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Two runs.  The first read **65** errors - the
standing **42** plus twenty-three `E0308 mismatched types` (`Some (w32_set_background_color)` and its
twenty-two siblings: the alias is C-ABI and the unit's own handlers were Rust-ABI `unsafe fn`).  The
second run is back to **42** with the same fifteen `E0428` names as before the mirror move - so the
move produced no new collision, and **no error sits at or beyond `src/w32fns.rs` 17,228**.

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 17,228-17,361: 4 capitalized
identifiers, the one missing being `Some` - the prelude variant the table uses, not an item under
`femacs/` (the tool's documented non-item class); no unresolved value.

**④ `extract-abi.py --check`.**  **63** rows, unchanged: the table is an object no `missing` row ever
named (the mirror declares it), and **no token row appears**.  The drift row the first reading added
(`line count drifted 39.4%: C 12488 vs Rust 17407`) is answered by a new `w32fns 0.41` row in
`docs/ports/line-drift-tolerance.tsv` (the measured 39.4% + 0.01, superseding slice 33's `0.39`),
and the second reading has no drift row.

**⑤ The range table and the dead-file gate.**  The `11522-12488` remainder row splits into
`11522-11583` (this slice) and `11585-12488` (the remainder, 904 lines);
`check-dead-file-coverage.py` reads **185 range row(s) cover src/w32fns.c (12488 lines; 134 blank
line(s) between blocks need no row)**.  The over-sequence comment above the
`w32_console_toggle_lock_key` block now reads `C 11585-12488`.

### Deviations (slice 35)

- **The twenty-three handlers became `unsafe extern "C" fn`** (from `unsafe fn`): C stores their
  address in a C-ABI function-pointer table, which is the spelling `src/haikufns.rs` 2335,
  `src/pgtkfns.rs` 1092 and `src/androidfns.rs` 3452 already use for their own handlers.  Their
  bodies, names and gates are unchanged.
- **The `frame_parm_handler` alias is spelled at the use site** (`dispextern.h` 3063's typedef body),
  because `include/dispextern.rs` carries only the `*mut c_void` field - the
  `src/haikufns.rs` 218-225 precedent.
- **The twenty-two `gui_set_*` prototypes are declared here**, because `include/frame.rs` does not
  carry `frame.h` 1844-1871 - the `src/haikufns.rs` 226-272 block, same wording.
- **C's `0` entries are `None`, with C's comments kept** (`/* x_set_scroll_bar_foreground, */` and
  its four siblings); the table's element count (51) is the reference's own.
- **The mirror declaration moved into `w32term_objs`**; the unit's definition keeps
  `#[unsafe(no_mangle)]` (C's `w32_frame_parm_handlers` is non-static) and shadows the glob import.
- **Not claimed.**  No run evidence: no Windows toolchain, so no frame parameter is ever set through
  this table and the object is never linked; the arm reading is compile-level (42 errors, none at or
  after this slice) and the name reading is the sweep's.

## Slice 36 (C 12452-12488)

**Scope and the stop.**  The file's last two items - C 12452's `#ifdef WINDOWSNT` around
`load_unicows_dll_for_w32fns` (C 12454-12476) and C 12478's `#ifdef NTGUI_UNICODE` around
`ntgui_encode_system` (C 12480-12488).  **Landed ahead of the C 11585-12450 text above it**: C 11588
is what would be `syms_of_w32fns`'s first statement, `globals_of_w32fns ()`, and C 12294's
definition is inside that remainder - this file has no "declare then define" pair (PORTING.md 0.1-12
/ P-91 ①, "land whichever the slice needs first"), so the callee lands first and slice 37's
`syms_of_w32fns` will find it.  Rust items are order-independent (P-91 ②).

**What the items carry.**  `load_unicows_dll_for_w32fns` is C 12461-12464's fallback to
SHELL32.DLL, C 12466-12469's two loads and C 12471-12472's `#ifndef HAVE_DBUS` third; each is C's
`(void *) get_proc_addr (...)` cast spelled as a `transmute` of the loader's `VOIDFNPTR` into the
pointer's own type (the `src/w32.rs` 2048-2049 device).  `ntgui_encode_system` is C 12483's local,
C 12484's `to_unicode` (declared at C 12505) and C 12485's return.

### Evidence (slice 36, the final revision: 17,459 Rust lines)

**① The frozen configuration (`build-tu`).**  Two runs, both **exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Two runs, both **42 previous errors** - and
that is this slice's finding, not a clean bill of health (see ⑤): the first run's text carried a
type name that does not exist anywhere in the tree, and the probe reported nothing.  **No error sits
at or beyond `src/w32fns.rs` 17,362** (the largest is the pre-existing `must_be_updated_p` gap at
12,083).

**③ The name sweep (`check-unit-names.py`) - the reading that caught it.**  First run,
`src/w32fns.rs` 17,362-17,430: 9 capitalized identifiers, missing **`PFNSHELL_NOTIFYICONW`** - a
name the slice had invented: `pfnShell_NotifyIconW`'s declaration (C 10316-10320) spells its type
inline, so no such alias exists.  Repaired by writing the target type out as that declaration has
it; second run: 11 capitalized identifiers, **none missing**, for the slice and the file's tail.

**④ `extract-abi.py --check`.**  **63 → 61** rows: `missing function load_unicows_dll_for_w32fns`
and `missing function ntgui_encode_system` retire, and **no token row appears**.  No drift row:
17,459 Rust lines against 12,488 C lines (39.8%), inside the `w32fns 0.41` row appended in slice 35.

**⑤ The probe's name-resolution gap, re-measured and extended (this slice's real finding).**
Four deliberate injections, each read with the same probe command:

| injection | result |
| --- | --- |
| `let _zz: u32 = c"x".as_ptr ();` in `load_unicows_dll_for_w32fns`'s body (inside the `not (have_dbus)` block) | **reported** - 43 errors, `E0308` at the injected line |
| `let _zz: u32 = c"x".as_ptr ();` in the same body, outside the block | **reported** - 43 errors, `E0308` |
| `ZZQQ_BOGUS_TYPE` as the `transmute`'s target type, and as a `let` annotation, inside the block | **not reported** |
| `ZZQQ_OUTER_TYPE`/`ZZQQ_OUTER_VALUE` in the body outside the block | **not reported** |

So the probe checks *types* but is not a name-resolution reading (slice 27 measured the value-name
half for `DEFUN!` bodies; this extends it to types and to plain items) - and the injection rows also
prove that the `not (have_dbus)` arm **is** compiled by the probe (`have_dbus` is not in the
command's `--cfg` list), so the tray block's *type* errors since slice 26 were genuinely read.
Consequence, recorded with the tool: the sweep is the only reading for a name, and it only looks at
capitalized identifiers - a lowercase unresolved name is still invisible to both.

### Deviations (slice 36)

- **The two items are landed before the text that precedes them in C** (P-91 ①/②), which the range
  table's last row records.
- **`unicows` is `mut`** (C 12461 assigns the parameter), the `src/image.rs` 2902 device.
- **`ntgui_encode_system`'s `encoded` is `Qnil ()` first**, because C's `Lisp_Object encoded;` is
  written through a pointer by `to_unicode` and Rust's definite-initialisation rejects the read -
  the `src/androidfns.rs` 2787 device.
- **The `#ifdef WINDOWSNT` and `#ifdef NTGUI_UNICODE` arms are the item's gate**, not a nested
  condition: C 12452/12478 sit inside no other `#if`.
- **Not claimed.**  No run evidence: no Windows toolchain, so no UNICOWS.DLL is loaded and no
  code-page conversion runs; the readings are the compile-level ones above and the sweep's.

## Slice 37 (C 12042-12128)

**Scope and the stop.**  The exception half of the crashing group: C 12042's banner, C 12044-12049's
`#ifndef CYGWIN` guard (the handler's forward declaration, dropped by PORTING.md 0.1-12, and
`prev_exception_handler`) with the two ungated objects, C 12051-12059's `#ifndef CYGWIN` guard
(`_resetstkoflw_proc`, `resetstkoflw`), and C 12061-12128's three functions.  The stop is C 12130's
`CaptureStackBackTrace_proc`.  `globals_of_w32fns` (slice 38) is what needs this half: it passes
`my_exception_handler` to `SetUnhandledExceptionFilter` and writes `prev_exception_handler`/
`resetstkoflw`, and this file has no "declare then define" pair (P-91 ①).

**What the items carry.**  `w32_reset_stack_overflow_guard` is C 12068-12070's lazy load through
`GetModuleHandle ("msvcrt.dll")` and C 12073-12074's guarded call; `stack_overflow_handler` is
C 12082-12083's hard-GC-error test and C 12084-12089's `#ifdef _WIN64` pair; `my_exception_handler`
is C 12102-12103's two recorded fields, C 12106-12122's stack-overflow test (code, parameter count,
main-thread test, the `Rip`/`Eip` write and C 12120's cleared address) and C 12124-12126's two-exit
chain.

**New surface.**  The five `<winnt.h>` records the group walks, `LPTOP_LEVEL_EXCEPTION_FILTER`, the
four objects, `_resetstkoflw_proc`, the three `EXCEPTION_*` values, and a use-site declaration for
`terminate_due_to_signal` (`syssignal.h`; the mirror carries no row - the `src/xfns.rs` 14148
device).  Three `<windows.h>` scalars join the file's own stand-in block: `PVOID` (C 12049's
`except_addr`), `USHORT` (C 12130's `CaptureStackBackTrace_proc`, next slice) and `DWORD64` (the
AMD64 `CONTEXT` members).

### Evidence (slice 37, the final revision: 17,727 Rust lines)

**① The frozen configuration (`build-tu`).**  Three runs (the third after the `Eip` narrowing fix):
**exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Three runs.  The second read **43** errors - the
standing **42** plus `E0308` at the `Eip` write (`stack_overflow_handler as DWORD_PTR` into a
`DWORD` field; C's own narrowing, now spelled `as DWORD_PTR as DWORD`).  The third run is back to
**42** and **no error sits at or beyond `src/w32fns.rs` 17,420** (the largest is the pre-existing
`must_be_updated_p` gap, now at 12,094).  This slice's records therefore **do** type-check: the
`CONTEXT` offset assertions and the five records are read by the probe - the contrast with slice
36's name-resolution gap is the point (types are read, names are not).

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 17,420-17,681: 88 capitalized
identifiers, the missing ones all in the documented non-item classes - `Clone`/`Copy` and the field
names of the five new records (`Rip`, `Eip`, `ExceptionCode`, `P1Home`, …); no unresolved item name.

**④ `extract-abi.py --check`.**  **61 → 54** rows: `missing function w32_reset_stack_overflow_guard`,
`missing function stack_overflow_handler`, `missing function my_exception_handler` and the four
`missing file-scope object` rows (`prev_exception_handler`, `except_code`, `except_addr`,
`resetstkoflw`) retire.  Two token rows appeared and one notes row each answers them:
`stack_overflow_handler does not mention __builtin_longjmp` (the GCC builtin Rust has no counterpart
for; PORTING.md P-10 keeps the final spelling open) and `my_exception_handler does not mention 0`
(C 12120's null pointer constant, spelled `core::ptr::null_mut ()`).  The drift row the first
reading added (`line count drifted 42.0%: C 12488 vs Rust 17727`) is answered by a new `w32fns 0.43`
row in `docs/ports/line-drift-tolerance.tsv` (the measured 42.0% + 0.01, superseding slice 35's
`0.41`), and the second reading has no drift row.

**⑤ The range table and the dead-file gate.**  The single remainder row `11585-12450` becomes four:
`11585-12038` (remainder: `syms_of_w32fns`), `12042-12128` (this slice), `12130-12279` (remainder:
the backtrace and `emacs_abort`) and `12283-12450` (remainder: `globals_of_w32fns`);
`check-dead-file-coverage.py` reads **189 range row(s) cover src/w32fns.c (12488 lines; 142 blank
line(s) between blocks need no row)**.

### Deviations (slice 37)

- **`CONTEXT` is carried as each `_WIN64` arm's *prefix*, not the whole SDK record.**  C 12113/12115
  name only `Rip`/`Eip`; the ARM64/x86_64/ix86 layouts live in `<winnt.h>` (no file in the reference
  tree, no `windows.h` on this machine), so the two arms transcribed here are the Win32 metadata
  projection's (`windows-sys` 0.52) field runs **up to the member C writes**, and the two
  `offset_of!` assertions (`Rip` at `0xF8`, `Eip` at `0xB8`) pin the facts C depends on.  The
  remaining members (the floating-point save area, `VectorRegister`, `ExtendedRegisters`) are
  **not modeled** and no size claim is made for the record - the truncation is the deviation.
- **C's `__builtin_longjmp` (C 12086) has no Rust counterpart.**  The port calls the tree's own
  `sys_longjmp` in both arms of C 12084-12089's pair and records it in a notes row; PORTING.md P-10
  (a Windows-target spike) is what must decide the final spelling.
- **C 12045's forward declaration of `my_exception_handler` is dropped** (PORTING.md 0.1-12: a Rust
  item is visible throughout its module), and C 12117-12119's/12120's comment text is carried.
- **C's `(DWORD_PTR)` value into the `DWORD` `Eip` member is spelled `as DWORD_PTR as DWORD`** - C's
  own narrowing at that assignment.
- **The discarded `_resetstkoflw` result keeps C's `(void)`** as a statement whose value is unused.
- **Not claimed.**  No run evidence: no Windows/mingw toolchain, so no guard page is re-established,
  no stack overflow is recovered and no filter is installed; the readings are compile-level (42
  errors, none from this slice), the name sweep's, and the two offset assertions'.

## Slice 38 (C 12283-12450)

**Scope and the stop.**  C 12283's `/* Initialization.  */`, C 12285-12292's comment and
`globals_of_w32fns` (C 12293-12450): the twenty-six lazy `get_proc_addr` loads, the dark-mode probe,
the three exception objects slice 37 landed, the four `DEFVAR`s and C 12447/12449's two startup
calls.  The stop is C 12452 (slice 36) - nothing of the file's own text is left between the two rows
but the C 11585-12038 and C 12130-12279 remainders.  The function is the callee slice 36 landed the
tail for (C 11588's `globals_of_w32fns ()` in `syms_of_w32fns`, P-91 ①); it carries the file gate
alone, with `not (cygwin)` and `windowsnt` arms inside.

**What the item carries.**  C 12296-12297's two module handles; C 12302-12314's six `user32_lib`
loads with C 12298-12301's `TrackMouseEvent` comment; C 12315-12320's `#ifndef CYGWIN` pair;
C 12321-12337's touch/gesture/flash trio (C 12324's sleep comment included); C 12339-12357's
`imm32_lib` block with its seven IME entries; C 12359-12363's `hm_kernel32` pair; C 12365-12371's
`#ifdef WINDOWSNT` pair; C 12373-12402's dark-mode gate (`LoadLibrary` of dwmapi/uxtheme,
`w32_get_resource` over the Personalize key and C 12400-12401's `w32_darkmode = TRUE`); C 12404-12409's
three exception objects; C 12411-12437's four `DEFVAR`s with their assignments; C 12439-12444's
`w32_unicode_gui`/`after_deadkey` pair; and C 12447/12449's `InitCommonControls`/`syms_of_w32uniscribe`.

**New surface.**  Four use-site declarations - `SetUnhandledExceptionFilter` (whose type is slice
37's `LPTOP_LEVEL_EXCEPTION_FILTER`), `GetACP`, `_getmbcp` and `InitCommonControls` - because no
mirror carries the rows (the slice-34 `redisplay_preserve_echo_area` device).  `GetModuleHandle`,
`LoadLibrary`, `w32_get_resource`, the twenty-six pointer statics, the four Lisp variables and
`syms_of_w32uniscribe` were all landed by earlier slices.

**The mirror move (P-21/P-39).**  `include/w32term.rs` 1389 declared `globals_of_w32fns` at its crate
root (C 918-919's `syms_of_w32fns`/`globals_of_w32fns` block), which a `have_ntgui` unit that defines
it reads as `E0428` - measured here.  Both names moved into the existing `pub mod w32term_fns_decls`
(the module that already holds `w32fns.c`'s three `w32_kbd_*` definitions, glob-imported at line
1609), with their `/// C:` lines and a pointer comment at the old site; slice 39's `syms_of_w32fns`
therefore needs no further move.  `docs/ports/include-w32term.md` records it.

### Evidence (slice 38, the final revision: 17,952 Rust lines)

**① The frozen configuration (`build-tu`).**  Four runs, all **exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Four runs.  The first aborted early at **19**
errors on the two `DEFVAR_INT!` invocations (`no rules expected ':'` - C's `doc:` label is accepted
by `DEFVAR_BOOL!`'s second rule but not by `DEFVAR_INT!`; the port takes the docstring
positionally).  The second read **43** - the standing **42** plus `E0428` for `globals_of_w32fns`
against the mirror's crate-root declaration **and** a stale root (the macro error had aborted before
the later items were checked).  After the mirror move the third run is **42**; **no error sits at or
beyond `src/w32fns.rs` 17,682** (the largest is the pre-existing `must_be_updated_p` gap at 12,094).

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 17,682-17,906: 52 capitalized
identifiers, missing only `DEFVAR` (the `DEFVAR_INT!`/`DEFVAR_BOOL!` invocation name) and `Some`
(the prelude variant) - the documented non-item classes; no unresolved item name.

**④ `extract-abi.py --check`.**  **54 → 53** rows: `missing function globals_of_w32fns` retires.  The
first reading added two rows, both answered: a token row
(`function globals_of_w32fns does not mention flash_window_ex_fn, flashwindowex_proc,
setgestureconfig_fn, setgestureconfig_proc` - the real defect this slice had: two of the twenty-six
loads, C 12332-12337's `SetGestureConfig_fn`/`flash_window_ex_fn`, were missing from the assembled
body, and the per-function token check found exactly the four tokens they carry; the pair is now in
C's place between the sleep trio and the `imm32_lib` block) and a drift row (`line count drifted
43.8%: C 12488 vs Rust 17952`), answered by a new `w32fns 0.45` row in
`docs/ports/line-drift-tolerance.tsv` (the measured 43.8% + 0.01, superseding slice 37's `0.43`).
The second reading has neither row.

**⑤ The range table and the dead-file gate.**  The `12283-12450` remainder row becomes the slice's
landed row; `check-dead-file-coverage.py` still reads **189 range row(s) cover src/w32fns.c (12488
lines; 142 blank line(s) between blocks need no row)**.

### Deviations (slice 38)

- **C's `(Proc) get_proc_addr (...)` casts are three-line `transmute`s** whose target repeats the
  pointer's own declared type (`Option<ImmGetCompositionString_Proc>`, `SystemParametersInfoW_Proc`,
  …) - the `src/w32.rs` 2048-2049 device for a `VOIDFNPTR` load, C-named here because the
  declaration and the assignment stand in one unit.
- **C's `NULL` for `resetstkoflw` and for `w32_get_resource`'s third argument are `None`
  / `core::ptr::null_mut ()`**, and C 12405's `except_addr = 0;` is `core::ptr::null_mut ()` - the
  null-family count is unchanged.
- **The two `DEFVAR_INT!` invocations take the docstring positionally** (C's `doc:` label has no
  second rule in that macro), while the two `DEFVAR_BOOL!` ones keep C's label - the
  `include/lisp.rs` 3540 note.
- **C 12414/12422's `UINT`/`int` values into the `EMACS_INT` variables are `as EMACS_INT`** (C's own
  implicit widening at that assignment).
- **The mirror's two declarations moved with their `/// C:` lines** and a pointer comment at the old
  site (P-21/P-39), which is what makes slice 39's `syms_of_w32fns` landable.
- **Not claimed.**  No run evidence: no Windows toolchain, so no DLL is loaded, no code page is read
  and no dark mode is probed; the readings are the compile-level ones above, the sweep's and the
  token check's.

## Slice 39 (C 12130-12159)

**Scope and the stop.**  The backtrace half of the crashing group: C 12130-12131's
`CaptureStackBackTrace_proc`, C 12133's `BACKTRACE_LIMIT_MAX`, C 12134-12141's two-arm
`DEFAULT_IMAGE_BASE` and C 12143-12159's `w32_backtrace`.  The stop is C 12161's `emacs_abort`.
Nothing here is under an `#if` except the `#if/#elif` pair of C 12137-12141, so the items carry the
file gate alone.

**What the items carry.**  `w32_backtrace` is C 12147's local, C 12149-12154's lazy load through
`LoadLibrary ("Kernel32.dll")` (C's function-local `static` moved to file scope with its
initialiser, the file's standing reading) and C 12155-12158's guarded call with the no-symbol arm's
`return 0`.

**New surface.**  `PULONG` (this unit's own `<winnt.h>` stand-in copy, the P-88 ① device),
`CaptureStackBackTrace_proc`, `BACKTRACE_LIMIT_MAX`, the two `DEFAULT_IMAGE_BASE` arms and the moved
`s_pfn_CaptureStackBackTrace`.  `min` is the mirror's (`include/lisp.rs` 3160, not `core::cmp::min`
- the slice-30 reading), and the derived feature `emacs_int_max_gt_long_max` is new to the tree's
feature table (`tools/feature-coverage.py --write` will add its row).

### Evidence (slice 39, the final revision: 18,024 Rust lines)

**① The frozen configuration (`build-tu`).**  Two runs, both **exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  Two runs.  The first read **43** errors - the
standing **42** plus `E0308` at the call (`expected i32, found u16`: C's `USHORT` result widened to
the function's `int`; now `as c_int`).  The second run is back to **42** and **no error sits at or
beyond `src/w32fns.rs` 17,959** (the largest is the pre-existing `must_be_updated_p` gap at 12,094).

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 17,959-18,040: 17 capitalized
identifiers, **none missing** - this slice invents no name (its `PULONG`, the typedef, the two
constants and the moved static all resolve).

**④ `extract-abi.py --check`.**  **53 → 49** rows: `missing function w32_backtrace`,
`missing macro BACKTRACE_LIMIT_MAX`, `missing constant BACKTRACE_LIMIT_MAX` (the one `pub const`
answers both spellings of a C `#define`, the slice-26 rule) and `missing macro DEFAULT_IMAGE_BASE`
retire.  **No token row** - C's body tokens all have a counterpart, including the moved static's
declaration (`NULL`'s count is carried by the call's `core::ptr::null_mut ()` plus C's own
`hm_kernel32 = NULL`, whose port spelling is the same device) - and **no drift row**: 18,024 Rust
lines against 12,488 C lines (44.3%), inside the `w32fns 0.45` row appended in slice 38.

**⑤ The range table and the dead-file gate.**  The `12130-12279` remainder row splits into
`12130-12159` (this slice) and `12161-12279` (`emacs_abort`, the remainder);
`check-dead-file-coverage.py` reads **190 range row(s) cover src/w32fns.c (12488 lines; 143 blank
line(s) between blocks need no row)**.  The over-sequence comment above the
`w32_console_toggle_lock_key` block now reads `C 11585-12038 and C 12161-12279`.

### Deviations (slice 39)

- **C's `#if/#elif` pair is two complementary constants**, the second arm naming C's own `#elif`
  (`not (all (...))` - the `src/image.rs` 1316 device), because Rust has no `#elif`.
- **`EMACS_INT_MAX > LONG_MAX` is the derived feature `emacs_int_max_gt_long_max`**, the
  `win32_nt_lt_500`/`win32_windows_ge_0410` naming rule for a condition whose value the port cannot
  read from the C text alone.
- **The entry point's `USHORT` result is `as c_int`** (C's own widening at the `return`), and the
  `min (BACKTRACE_LIMIT_MAX, limit)` argument is `as ULONG` for the same reason.
- **C's function-local `static` is a file-scope item** with its initialiser (the
  `version_string`/`coords` device), so C 12146 has no separate item.
- **`PULONG` is declared here** rather than taken from another unit: each unit carries its own
  `<windows.h>`/`<winnt.h>` stand-in names (P-88 ①).
- **Not claimed.**  No run evidence: no Windows toolchain, so no `RtlCaptureStackBackTrace` is ever
  loaded and no backtrace is taken; the readings are the compile-level ones above and the sweep's.

## Slice 40 (C 12161-12279)

**Scope and the stop.**  `emacs_abort` (C 12161-12279): C 12164-12167's abort-dialog short circuit and
the `button` local, C 12169-12185's dialog (C 12179-12182's `#if __GNUC__` string arm), C 12186-12278's
`switch` over the answer with the three `#ifdef CYGWIN` pairs inside its second arm, and C 12275's
`abort ()`.  The stop is C 12281's form feed; the file's only remaining text is C 11585-12038
(`syms_of_w32fns`).  The item carries the file gate alone, with `gnuc` and `cygwin` arms inside.

**What the item carries.**  C 12172-12185's `MessageBox` (the message text with C 12179-12182's
GNU-only tail, the caption and the four flags), C 12188-12190's `IDYES` arm (`DebugBreak` and
`exit (2)`), C 12194-12195's `void *stack[BACKTRACE_LIMIT_MAX + 1]` with the `w32_backtrace` call,
C 12196-12202's CYGWIN/MinGW `addr_offset` pair, C 12204-12273's reporting block (the `errfile_fd`
local, C 12208-12214's measured `buf`, C 12215-12223's `stderr_fd` pair with `GetStdHandle` and
`_open_osfhandle`, the two `sprintf` sites, the four `write` calls, C 12240's `_open`/`open` pair,
C 12243's `lseek`, the `j` loop with C 12251-12257's comment, C 12265-12271's truncation marker and
C 12272-12273's `close`), and C 12275's `abort ()`.

**New surface.**  Four use-site declarations (`MessageBox` under C's SDK alias name, `DebugBreak`,
`GetStdHandle`, `_open_osfhandle`), C's `_open` (the `cygwin` arm of C 12237-12239's `#define` calls
the tree's own `open`, `lib/include/fcntl.in.rs` 286 - a second declaration of it is what
`tools/check-mirror-dupes.py` refuses, measured), and
nine SDK constants (`IDYES`, `IDNO`, `MB_TASKMODAL`, `MB_SETFOREGROUND`, `MB_YESNO`,
`STD_ERROR_HANDLE`, `INVALID_HANDLE_VALUE`, `S_IREAD`, `S_IWRITE`).  `write`/`close`/`lseek`/`sprintf`/
`strlen`/`abort`/`exit`/`min` and the four globals (`w32_disable_abort_dialog`, `noninteractive`,
`except_code`, `except_addr`) were already in scope.

### Evidence (slice 40, the final revision: 18,237 Rust lines)

**① The frozen configuration (`build-tu`).**  Three runs - the first two refused the file: the
GNU-only message fragment was written with `\\"` instead of `\"` (`unknown start of token: \`,
three errors; the C text's own `\"` is what the Rust literal needs).  The third is **exit 0**, the
object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  One run: **42 previous errors** - the standing
count, so this slice adds no error (its `IDNO | _` arm draws no `unreachable_patterns`) - and **no
error sits at or beyond `src/w32fns.rs` 17,979**.  The warning count moves 122 → 124: the two new
rows are `unused doc comment` on the two new `unsafe extern "C"` blocks' `/// C:` headers, the
file's standing style (rustdoc does not consume a doc comment on an extern block).

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 17,976-18,220: 29 capitalized
identifiers, **none missing** - every name this slice introduces resolves.

**④ `extract-abi.py --check`.**  **49 → 47** rows: `missing function emacs_abort` retires, leaving
the forty-six `missing defsubr` rows and `missing function syms_of_w32fns` - both C 11585-12038's.
Two rows the first reading added are answered: a token row
(`function emacs_abort does not mention break, case, default, int_bufsize_bound, switch` - the
`match` substitution of C's `switch`/its two shared labels, and C's `INT_BUFSIZE_BOUND` bound
expression, recorded in one new `macro-body-notes.tsv` row) and a drift row (`line count drifted
46.0%: C 12488 vs Rust 18237`), answered by a new `w32fns 0.47` row in
`docs/ports/line-drift-tolerance.tsv` (the measured 46.1% + 0.01, superseding slice 38's `0.45`).

**⑤ The range table and the dead-file gate.**  The `12161-12279` remainder row becomes the slice's
landed row; `check-dead-file-coverage.py` still reads **190 range row(s) cover src/w32fns.c (12488
lines; 143 blank line(s) between blocks need no row)**, and the table's only remainder row is now
`11585-12038`.  The over-sequence comment above the `w32_console_toggle_lock_key` block reads
`C 11585-12038's text is not in the port yet`.

### Deviations (slice 40)

- **C 12179-12182's `#if __GNUC__` fragment makes the message two arm-selected bindings.**  C's one
  chain of adjacent literals cannot be spelled as `concat! (…)` with a `#[cfg]` inside, so the
  `gnuc`/`not (gnuc)` arms each bind `abort_msg` with the text they compile - a recorded deviation,
  and the `gnuc` name is the tree's own (`lib/include/cdefs.rs` 413).
- **C's `switch` is the port's `match`**, and its two shared labels (`case IDNO:` and `default:`)
  are the single arm `IDNO | _` (the `add_tray_notification` reading), recorded in a notes row.
- **C 12208-12214's `char buf[...]` is the measured `[c_char; 76]`** (the literal is 34 with its NUL;
  `INT_BUFSIZE_BOUND (intptr_t)` is 21) - the `src/font.rs` 2177 device for a C bound expression.
- **Only `_open` is declared here**: `lib/include/fcntl.in.rs` 286 already declares `open`, so the
  `cygwin` arm of the `#define` uses the mirror's copy (the `check-mirror-dupes.py` reading of the
  first attempt, which had both).
- **The three `#ifdef CYGWIN` pairs are sibling `#[cfg]` bindings/statements** (the
  `src/callproc.rs` 1828-1831 device): the `addr_offset` binding, the `stderr_fd` binding with
  `errout`'s two statements, and the `_open`/`open` call.
- **The nine SDK constants are spelled from the headers** (`S_IREAD`/`S_IWRITE` are MinGW's names
  for the mirror's `S_IRUSR`/`S_IWUSR`), and `INVALID_HANDLE_VALUE` uses the `src/sysdep.rs` 1058
  `-1isize as HANDLE` spelling.
- **Not claimed.**  No run evidence: no Windows toolchain, so no dialog is shown, no backtrace file
  is written and no process aborts; the readings are the compile-level ones above and the sweep's.

## Slice 41 (C 11585-12038) - the file's last item, and the lane closes

**Scope and the stop.**  `syms_of_w32fns` (C 11585-12038): C 11588's `globals_of_w32fns ()` call
(slice 38's definition), C 11589-11592's two objects with `PDUMPER_IGNORE`, C 11594-11636's
fifty-four `DEFSYM`s in C's five runs (two of them under C 11612's `#if defined WINDOWSNT &&
!defined HAVE_DBUS` and C 11622's `#ifdef WINDOWSNT`), C 11638-11644's two `Fput`s over
`Qundefined_color` with the `staticpro (&w32_grabbed_keys, NULL)` pair, C 11646-11925's twenty-nine
`DEFVAR`s with their assignments and C's own comments, C 11827-11854's and C 11927-11931's `#if 0`
regions (carried as C's own text), C 11932-12007's forty-six `defsubr`s with their three arms,
C 11990-11999's five `tip_*` `staticpro` pairs, C 12009-12024's sixteen closing `DEFSYM`s and
C 12026-12037's taskbar/sleep arm.  **There is no stop**: this is the file's last C text, and the
range table's last remainder row retires with it.

**What the item carries.**  Everything listed above; the only new name the port needs is
`GetDoubleClickTime` (C 11784, declared at the use site - the `redisplay_preserve_echo_area`
device).  The `S...` subr statics are the ones the file's `DEFUN!` items generated, so the
`defsubr` table names existing items.

**The remainder comment retires.**  The block that explained why `w32_console_toggle_lock_key`
(C 10112) stands below the slices that insert above it - "C 11585-12038's text is not in the port
yet ... Later slices of this lane insert their own text *above* this block, so the file's item order
still equals C's order (P-91 ②)" - is removed: no text is left to insert above it, so the file's
items now all stand in C's order with that one documented exception (C 10112 sits at the file's end
because C 9302 calls it and the port has no "declare then define" pair, P-91 ①).

### Evidence (slice 41, the final revision: 18,594 Rust lines - the file closes)

**① The frozen configuration (`build-tu`).**  One run: **exit 0**, the object **336 bytes**.

**② The arm probe (`have_ntgui` + `windowsnt`).**  One run: **42 previous errors** - the standing
count, so the whole 454-line body type-checks with **no error at or beyond `src/w32fns.rs` 18,192**
(the largest is the pre-existing `must_be_updated_p` gap at 12,094).  The warning count moves
124 → 125 (one more `unused doc comment`, the new extern block's `/// C:` header - the file's
standing style).

**③ The name sweep (`check-unit-names.py`).**  `src/w32fns.rs` 18,192-18,594: 165 capitalized
identifiers, and the 65 "missing" ones are all in the tool's documented false-positive classes - the
`S...` subr statics `DEFUN!` generates, the three `Sx_*_window_property` names *inside* the carried
`#if 0` text (the nested-comment limitation) and the two words of the `PORTING.md P-51` citation
inside that comment; no unresolved item name.

**④ `extract-abi.py --check` - `OK`.**  **47 → 0** rows: the forty-six `missing defsubr` rows and
`missing function syms_of_w32fns` retire, and the reading is `OK emacs/src/w32fns.c ->
femacs/src/w32fns.rs` with four recorded C macros (`C_CHILDREN_TITLEBAR`, `DEFAULT_IMAGE_BASE`,
`FPRINTF_WM_CHARS`, `MOUSEEVENTF_FROMTOUCH` - the branches the token check does not compare).  Two
rows the first reading added are answered: a token row (`function syms_of_w32fns does not mention 1`
- C's two `= 1;` sites on `bool` variables are the port's `= true;`, one new
`macro-body-notes.tsv` row) and the drift row (`line count drifted 48.9%: C 12488 vs Rust 18594`),
answered by the file's **finished** `w32fns 0.50` row in `docs/ports/line-drift-tolerance.tsv`.

**⑤ The range table and the dead-file gate.**  The last remainder row becomes the slice's landed
row; `check-dead-file-coverage.py` reads **190 range row(s) cover src/w32fns.c (12488 lines;
143 blank line(s) between blocks need no row)** - **no row of the table is a remainder any more**.

**⑥ The lane's closing reading.**  `extract-abi.py --check` `OK` above is the per-file half; the
tree-wide one was measured in the same round: **`extract-abi.py --all` reads 590 files -> 583 done,
0 in progress, 7 failing** (180s, 297MB), where the failing set no longer contains `src/w32fns.c` -
it is `android.c`, `androidterm.c`, `androidvfs.c` (the JNI-name/accounting class of the earlier
reading), `gtkutil.c`, `w32.c`, `w32proc.c` and `w32term.c`.  What this lane claims at its close is exactly these readings: the file's text is complete
against its reference, its token check is clean, its four arm readings are the 42-error baseline
with no diagnostic from this file's own text, and the `--check` is `OK`.  Nothing here is a claim
about running the code.

### Deviations (slice 41)

- **C's `DEFVAR_*` invocations keep C's docstring text** as a literal (the port's macros take it
  positionally for `DEFVAR_INT!` and under C's `doc:` label for `DEFVAR_LISP!`/`DEFVAR_BOOL!`).
- **C's two `#if 0` regions are carried as C's own text** inside comments (the file's reading for a
  region no configuration compiles: no `DEFVAR`/`defsubr` of them takes effect), and the nested
  comment is why the name sweep reads two words of the citation there.
- **C's `XSETINT (Vw32_phantom_key_code, 255)` is the tree's `XSETINT!`** (the
  `src/androidterm.rs` 872 device), and C's `= 0;`/`= 1;` on `bool` variables are `false`/`true`
  (recorded in a notes row).
- **`GetDoubleClickTime` is declared at the use site** (no mirror row).
- **The file's one remaining out-of-sequence item is `w32_console_toggle_lock_key`** (C 10112),
  which stays at the file's end with its own doc comment; the range table carries it as the landed
  row `10112-10138`.
- **Not claimed.**  No run evidence: no Windows toolchain, so no symbol is interned, no variable
  registered and no subr defined at run time; the readings are the compile-level ones above, the
  name sweep's and the token check's.

**Starting state measured for the slice-5 lane (2026-10-14, at `bb049624`), kept here below the
slice that used it.**  Three facts a fresh
reader would otherwise have to re-derive, all read off the tree just now:

- **This file's text convention is *prose* `C NNNN` citations, not column-end ticks.**  A tick
  counter (the `src/w32.c` lane's `// C NNNN` device) reads `femacs/src/w32fns.rs` as 508 ticks and
  a maximum of C 5,716, which is an artefact: the file cites its C lines inside `///`/`/* */` prose
  (e.g. `/* C: `command:` (C 5708-5712).  */`).  Coverage is the range table plus `--check` plus
  `check-dead-file-coverage.py`, as this ledger already says - **do not** re-tick the file.
- **The first slice's boundary is C 5988-6121**: `my_create_window` (C 5988-6033, its doc comment
  from 5988), `my_create_tip_window` (C 6035-6078) and `w32_window` (C 6080-6121), three whole
  items ending at C 6121's `}` (C 6122 blank, C 6123 the next doc comment).  `w32_icon`
  (C 6123-6152) through `w32_default_font_parameter` (C 6216-6251) are the following slice, and
  **C 6253's `x-create-frame` DEFUN runs to about C 9579** (the ledge says C 9581 is
  `w32_display_info_for_name`), so it is its own slice by size alone.
- **The names C 5988-6121 needs that this unit's graph does not carry**: `CW_USEDEFAULT`
  (`<winuser.h>`), `gui_display_get_arg` with `resource_types`/`RES_TYPE_NUMBER`/`RES_TYPE_STRING`
  (`dispextern.h` 3983-4003 - `src/pgtkfns.rs` 498-507 spells the same four for its own unit) and
  `GetMessage` (C's macro; the file's existing `GetMessageA`/`GetMessageW` pair sit at
  `src/w32fns.rs` 6122/6124, and this very file already reads C's `...A`/`...W` macro pairs through
  its own `w32_unicode_gui` test - see the `DefWindowProc`/`DispatchMessage` sites).  Everything
  else those three functions name (`MSG`, `one_w32_display_info`, `PostThreadMessage`,
  `WM_EMACS_CREATEWINDOW`/`WM_EMACS_DONE`, `AdjustWindowRect`, `EMACS_CLASS`, `SetWindowLong`,
  `WND_*_INDEX`, `ShowWindow`/`SW_HIDE`, `hinst`, `FRAME_W32_WINDOW`, `block_input`/
  `unblock_input`, `xlispstrdup`, `validate_x_resource_name`, `w32_set_name`, `fset_name`,
  `initialize_frame_menubar`, `Vx_resource_name`, `one_w32_display_info`'s `hinst`) is already in
  scope - measured one name at a time.
