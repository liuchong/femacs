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
| 315-319 | `sysinfo_cache`, `osinfo_cache` | file gate (**deferred**, see the Deviations) |
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
| 4043-12488 | **the remainder of the file** (C 4043 `get_wm_chars` onward: C 4135's `#ifdef DBG_WM_CHARS`, C 4528 `w32_wnd_proc`, C 9581 `w32_display_info_for_name`, C 10113 `w32_console_toggle_lock_key`, C 11586 `syms_of_w32fns`) | not in this slice: recorded here so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |


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
