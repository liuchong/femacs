# emacs/src/xfns.c (10662 lines) — slices: C 1-802, C 803-2149, C 2150-3811, C 3812-3956,
C 3957-4170, C 4172-4428, C 4430-4623, C 4624-4783, C 4785-4916, C 4918-5582 and C 5583-5758

> **P-86's first X11 unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 7521 fills `WINDOW_SYSTEM_OBJ="fontset.o fringe.o image.o"` only under
> `HAVE_WINDOW_SYSTEM`, that macro is `undef` in
> `docs/baseline/config/darwin-arm64.macros.tsv`, and `docs/baseline/frozen-build-sources.txt`
> lists no `xfns.o`, so `extract-abi.py`'s checklist for the file is empty and its checkbox
> sections cannot be coverage evidence.  What follows is P-86 ②'s substitute: the **C line-range
> table** for this slice, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/xfns.rs` (**1,120 lines** after slice 1, **3,051 lines** after the second slice,
> **5,455 lines** after the third, **5,662 lines** after the fourth, **5,991 lines** after the fifth,
> **6,325 lines** after the sixth, **6,543 lines** after the seventh, **6,727 lines** after the
> eighth, **7,133 lines** after the ninth, **7,982 lines** after the tenth and **8,168 lines**
> after the eleventh - C 803-2149, C 2150-3811, C 3812-3956, C 3957-4170, C 4172-4428,
> C 4430-4623, C 4624-4783, C 4785-4916, C 4918-5582 and C 5583-5758, whose own ledger sections
> are at the end of this file - measured with `grep -c ''`), both arms of every `#if` carried.
>
> **This ledger replaces the previous revision's conclusion** ("the file is not compiled by the
> frozen configuration, so the port is the empty unit plus a note"), exactly as
> `docs/ports/android.md` did with the same finding: P-86 ① rules that a three-line header comment
> is *not started*, not *done*.  The old note's measurements are still true and are still where the
> *file gate* comes from; what changed is the conclusion drawn from them.
>
> **The slice is C 1-802**, ending at the `}` of `x_set_alpha_background` (C 802).  It holds the
> include graph (C 20-126), the Motif hint block (C 134-161), `check_x_display_info` (C 171-205),
> `x_real_pos_and_offsets` (C 216-588), `x_real_positions` (C 594-599),
> `x_relative_mouse_position` (C 604-646), `gamma_correct` (C 650-659), `x_defined_color`
> (C 667-685), `x_decode_color` (C 692-718), `x_set_wait_for_wm` (C 726-730) and
> `x_set_alpha_background` (C 732-802).  **The next live definition is `x_set_tool_bar_position`
> (C 805)** - C 804 is its `static void` line - and the table's last row is "the remainder".

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`WINDOW_SYSTEM_OBJ`, i.e. `HAVE_WINDOW_SYSTEM`; `configure.ac` 7521) | `feature = "have_x_windows"` (on every item and on C 20-126's include graph) | an X11 configuration |
| C 40 `#ifdef HAVE_X_I18N` … C 42 (`textconv.h`) | `all (feature = "have_x_windows", feature = "have_x_i18n")` | an X11 configuration with X11 I18N |
| C 47 `#ifdef USE_XCB` … C 50 (the `<xcb/*>` headers) | `all (feature = "have_x_windows", feature = "use_xcb")` | X11, XCB transport |
| C 55 `#ifdef HAVE_XRANDR` … C 57 | `feature = "have_xrandr"` (**not registered** in `femacs/xtask/Cargo.toml`; owed, below) | an X11 configuration with Xrandr |
| C 58 `#ifdef HAVE_XINERAMA` … C 60 | `feature = "have_xinerama"` (**not registered**; owed) | an X11 configuration with Xinerama |
| C 62 `#ifdef USE_GTK` … C 64 (`gtkutil.h`) | `all (feature = "have_x_windows", feature = "use_gtk")` | the GTK configuration |
| C 66 `#ifdef HAVE_XDBE` … C 68 | `all (feature = "have_x_windows", feature = "have_xdbe")` | an X11 configuration with Xdbe |
| C 70 `#ifdef HAVE_XINPUT2` … C 72 | `all (feature = "have_x_windows", feature = "have_xinput2")` | an X11 configuration with XInput2 |
| C 74 `#ifdef USE_X_TOOLKIT` … C 122 | `all (feature = "have_x_windows", feature = "use_x_toolkit")` | an X toolkit configuration |
| C 78 `#ifdef HAVE_XAW3D` … C 84 (C 81 the `#else`) | `feature = "have_xaw3d"` (**not registered**; owed) | a Lucid toolkit configuration with Xaw3d |
| C 87 `#ifdef USG` … C 95 (C 93-95 the `#else`) | `feature = "usg"` for C's arm; `<X11/Xos.h>` is the toolchain's header and has no mirror, so both arms stand as the port's comment and there is no Rust item to gate (P-88 ①) | a USG-derived X11 configuration |
| C 101 `#ifdef USE_MOTIF` … C 108 | `feature = "use_motif"` (**not registered**; owed) | the Motif configuration |
| C 110 `#ifdef USE_LUCID` … C 112 (`xlwmenu.h`) | `all (feature = "have_x_windows", feature = "use_lucid")` | the Lucid configuration |
| C 124 `#ifdef USE_GTK` … C 126 (empty) | `feature = "use_gtk"` | the GTK configuration |
| C 130 `#ifdef GLYPH_DEBUG` … C 132 (`dpyinfo_refcount`) | `all (feature = "have_x_windows", feature = "glyph_debug")` | a GLYPH_DEBUG configuration |
| C 134-135 `#ifndef USE_MOTIF` / `#ifndef USE_GTK` … C 160-161 | `all (feature = "have_x_windows", not (feature = "use_motif"), not (feature = "use_gtk"))` | neither Motif nor GTK |
| C 251/279/292/331/343/383/416/450/482/490/539/557/567 (`USE_XCB` and its `#else` partners, inside `x_real_pos_and_offsets`) | `use_xcb` / `not (use_xcb)` | both arms carried |
| C 673 `#ifdef USE_GTK` (inside `x_defined_color`) | `use_gtk`, nested in the file gate | the GTK configuration |
| C 699 `#if false` … C 712 (inside `x_decode_color`) | no gate: C's own condition is the literal `false` | never - recorded, not transcribed |
| C 742 `#ifdef HAVE_XRENDER` … C 756 (C 752-753 the `#else`) | `feature = "have_xrender"` / `not (feature = "have_xrender")` | both arms carried |
| C 764 `#ifdef USE_GTK` … C 771 (inside `x_set_alpha_background`) | `all (feature = "have_x_windows", feature = "use_gtk")` | the GTK configuration |
| C 737-740 / 794-798 (the `HAVE_GTK3` declarations and the class walk) | `all (feature = "have_x_windows", feature = "use_gtk", feature = "have_gtk3")` | the GTK3 configuration |
| C 776 `#ifndef HAVE_GTK3` … C 801 (C 777-782 the `else` arm, C 783-800 the `#else` arm) | `not (feature = "have_gtk3")` / `feature = "have_gtk3"` | both arms carried |

`use_xcb`, `have_x_i18n`, `glyph_debug`, `use_gtk`, `have_gtk3`, `have_xrender`, `use_x_toolkit`,
`use_lucid`, `have_xdbe`, `have_xinput2` and `usg` are names
`femacs/xtask/Cargo.toml` already registers.  `use_motif`, `have_xaw3d`, `have_xrandr` and
`have_xinerama` are the C macros this slice's heads test that the manifest does **not** register
yet: they are used in the port's `#[cfg]`s as C's own names lower-cased and are owed to the
manifest, not added here (the task's rule; the license to add them is the integrator's).

`HAVE_WINDOW_SYSTEM` has no `#if` of its own in this file, but it is the *build* gate
(`configure.ac` 7521), which is why the port's file gate is `have_x_windows`: the `frame.h` items
this slice names carry their own gates in the mirror (`FRAME_TOOLTIP_P` and `FRAME_Z_GROUP*` are
`have_window_system`'s, `FRAME_X_P` and `FRAME_LIVE_P` are ungated), and a real X11 configuration
defines both features.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-20 | the blank and `#include <config.h>` | ungated (every unit includes it) |
| 21-24 | `<stdio.h>`, `<stdlib.h>`, `<math.h>`, `<unistd.h>` | file gate |
| 25-38 | the blank at 25 and `lisp.h`, `character.h`, `xterm.h`, `frame.h`, `window.h`, `buffer.h`, `dispextern.h`, `keyboard.h`, `blockinput.h`, `charset.h`, `coding.h`, `termhooks.h`, `font.h` | file gate |
| 39-42 | `#ifdef HAVE_X_I18N` / `#include "textconv.h"` / `#endif` | `have_x_i18n`, nested in the file gate |
| 43-45 | `<sys/types.h>`, `<sys/stat.h>` | file gate |
| 46-50 | `#ifdef USE_XCB` / `<xcb/xcb.h>` / `<xcb/xproto.h>` / `#endif` | `use_xcb` (both headers are the toolchain's: no mirror, named below) |
| 51-53 | `bitmaps/gray.xbm`, `xsettings.h` | file gate (`gray.xbm` is a data file with no Rust counterpart; the C line is recorded) |
| 54-60 | `#ifdef HAVE_XRANDR`'s `<X11/extensions/Xrandr.h>` and `#ifdef HAVE_XINERAMA`'s `<X11/extensions/Xinerama.h>` | `have_xrandr` / `have_xinerama` (unregistered; both headers have no mirror) |
| 61-64 | `#ifdef USE_GTK` / `#include "gtkutil.h"` / `#endif` | `use_gtk`, nested in the file gate |
| 65-72 | `#ifdef HAVE_XDBE`'s `<X11/extensions/Xdbe.h>` and `#ifdef HAVE_XINPUT2`'s `<X11/extensions/XInput2.h>` | `have_xdbe` / `have_xinput2` (no mirrors) |
| 73-122 | the `USE_X_TOOLKIT` block: `<X11/Shell.h>`, the Xaw3d/Xaw pair (HAVE_XAW3D), `<X11/Xos.h>` with the `USG` dance (C 87-95), `widget.h` (C 97), `../lwlib/lwlib.h` (C 99), the six `<Xm/…>` headers (USE_MOTIF, C 101-108), `../lwlib/xlwmenu.h` (USE_LUCID, C 110-112), `extern LWLIB_ID widget_id_tick;` (C 114-116) and C 118-120's empty `USE_MOTIF` block | `use_x_toolkit`, `have_xaw3d`/`use_motif` (unregistered), `use_lucid`, `usg`; the two Emacs-side headers take mirrors, the X11/Xaw/Xm ones have none |
| 123-126 | the empty `#ifdef USE_GTK` block | `use_gtk` |
| 127-128 | `#define MAXREQUEST(dpy) (XMaxRequestSize (dpy))` | file gate |
| 129-132 | `#ifdef GLYPH_DEBUG` / `static int dpyinfo_refcount;` / `#endif` | `glyph_debug`, nested in the file gate |
| 133-161 | the Motif hints: `MWM_HINTS_DECORATIONS`, C's four commented-out members, `MWM_DECOR_ALL`, the commented-out `_XA_MOTIF_WM_HINTS`, `PropMotifWmHints` (C 149-157) and `PROP_MOTIF_WM_HINTS_ELEMENTS`, all inside C 134-135's `#ifndef USE_MOTIF` / `#ifndef USE_GTK` | `all (have_x_windows, not (use_motif), not (use_gtk))` |
| 162-164 | the two `static` forward declarations (`x_display_info_for_name`, `set_up_x_back_buffer`) | dropped (PORTING.md 0.1-12; both definitions are in the remainder - C 3972 and C 7613) |
| 165-205 | `check_x_display_info` (C 166-169 its comment) | file gate |
| 206-215 | the comment of `x_real_pos_and_offsets` | file gate |
| 216-588 | `x_real_pos_and_offsets` (C 251-262 the `USE_XCB`/`#else` declarations, C 279-282 `x_catch_errors`, C 292-321 the query-tree pair, C 331-484 the geometry/translate pair, C 487-535 `_NET_FRAME_EXTENTS`, C 537-565 the outer geometry, C 567-569 `x_uncatch_errors`) | file gate; `use_xcb` for every `#ifdef USE_XCB` arm and its `#else` partner |
| 589-599 | the comment of `x_real_positions` and the function | file gate |
| 600-646 | the comment of `x_relative_mouse_position` and the function | file gate |
| 647-659 | the comment of `gamma_correct` and the function | file gate |
| 660-685 | the comment of `x_defined_color` and the function (C 673 `#ifdef USE_GTK`) | file gate; `use_gtk` for `xg_check_special_colors` |
| 686-718 | the comment of `x_decode_color` and the function (C 699-706 the `#if false` region) | file gate; the `#if false` region is dead text, recorded not transcribed |
| 719-730 | the comment of `x_set_wait_for_wm` and the function | file gate (the one statement's member is the mirror gap named below) |
| 731-802 | `x_set_alpha_background` (C 742-756 the `HAVE_XRENDER` pair, C 764-771 the `USE_GTK` block, C 794-798 the `HAVE_GTK3` class walk, C 776-801 C's `#ifndef HAVE_GTK3` pair) | file gate; `have_xrender`, `use_gtk`, `have_gtk3` |
| 803-849 | the blank, `x_set_tool_bar_position` (C 804 its `static void`, C 809-847 the `USE_GTK` pair) | file gate; `use_gtk` for the GTK arm's `choice`, `Fmemq`, `xg_change_toolbar_position` and `wrong_choice` |
| 850-880 | `#ifdef HAVE_XDBE` (C 850), `x_set_inhibit_double_buffering` (C 852-879) and the `#endif` | `have_xdbe`, nested in the file gate |
| 881-923 | the doc comment of `x_set_undecorated` (C 882-894) and the function (C 895-923, C 900-921 the `USE_GTK` pair) | file gate; `use_gtk` for `xg_set_undecorated` |
| 924-1005 | the doc comment of `x_set_parent_frame` (C 924-953) and the function (C 954-1005, C 977-990 the GTK/GTK3/XSYNC arms) | file gate; `use_gtk`, `have_gtk3`, `all (have_xsync, not (use_gtk), have_clock_gettime)` |
| 1006-1036 | the doc comment of `x_set_no_focus_on_map` (C 1006-1017) and the function (C 1019-1036, C 1023-1033 the `USE_GTK` pair) | file gate; `use_gtk` |
| 1037-1072 | the doc comment of `x_set_no_accept_focus` (C 1037-1044) and the function (C 1051-1072, C 1055-1069 the `USE_GTK`/`USE_X_TOOLKIT`/`#else` triple) | file gate; `use_gtk`, `use_x_toolkit` |
| 1073-1104 | the doc comment of `x_set_override_redirect` (C 1073-1079) and the function (C 1084-1104, C 1092-1100 the `USE_GTK` pair) | file gate; `use_gtk` |
| 1105-1177 | `#ifdef USE_GTK` (C 1107), `xg_set_icon` (C 1113-1144), `xg_set_icon_from_xpm_data` (C 1147-1174) and the `#endif` | `use_gtk`, nested in the file gate |
| 1178-1220 | C's comment for the family (C 1179-1184) and `x_set_foreground_color` (C 1186-1220) | file gate |
| 1221-1252 | `x_set_background_color` (C 1222-1252, C 1242-1244 the `USE_GTK` pair) | file gate; `use_gtk` |
| 1253-1331 | `enum mouse_cursor` (C 1254-1272), `struct mouse_cursor_types` (C 1274-1286), the `mouse_cursor_types` table (C 1288-1306) and `struct mouse_cursor_data` (C 1308-1331) | file gate (the table's `XC_*` values are `<X11/cursorfont.h>`'s, reached through `xterm.h` 40) |
| 1332-1350 | `x_set_mouse_color_handler` (C 1333-1350) | file gate |
| 1351-1478 | `x_set_mouse_color` (C 1352-1478, C 1450-1470 C's `INSTALL_CURSOR` family) | file gate |
| 1479-1545 | `x_set_cursor_color` (C 1480-1545) | file gate |
| 1546-1581 | the comment of `x_set_border_pixel` (C 1547-1550) and the function (C 1551-1581, C 1557-1570 the `USE_X_TOOLKIT` arm) | file gate; `use_x_toolkit` |
| 1582-1603 | the comment of `x_set_border_color` (C 1583-1592) and the function (C 1594-1603) | file gate |
| 1604-1642 | `x_set_cursor_type` (C 1606-1610) and `x_set_icon_type` (C 1612-1642) | file gate |
| 1643-1679 | `x_set_icon_name` (C 1644-1679) | file gate |
| 1680-1766 | `x_set_menu_bar_lines` (C 1682-1766, C 1686-1688 the `! USE_X_TOOLKIT && ! USE_GTK` declaration, C 1705-1764 the toolkit/GTK pair) | file gate; `use_x_toolkit`, `use_gtk` |
| 1767-1794 | the comment of `x_set_tab_bar_lines` (C 1769-1774) and the function (C 1776-1794) | file gate |
| 1795-1859 | the comment of `x_change_tab_bar_height` (C 1797) and the function (C 1798-1859) | file gate |
| 1860-1891 | the comment of `x_set_tool_bar_lines` (C 1862-1866) and the function (C 1868-1891, C 1876-1880 the `USE_GTK` pair) | file gate; `use_gtk` |
| 1892-1964 | the comment of `x_change_tool_bar_height` (C 1894) and the function (C 1895-1964, C 1898-1963 the `USE_GTK` pair) | file gate; `use_gtk` |
| 1965-1994 | `x_set_child_frame_border_width` (C 1966-1994, C 1982-1985 the `USE_X_TOOLKIT` pair) | file gate; `use_x_toolkit` |
| 1995-2017 | `x_set_internal_border_width` (C 1996-2017, C 2005-2008 the `USE_X_TOOLKIT` pair) | file gate; `use_x_toolkit` |
| 2018-2076 | the comment of `x_set_scroll_bar_foreground` (C 2020-2023) and the function (C 2025-2076, C 2030-2032 the `HAVE_GTK3` declarations, C 2055-2075 the `HAVE_GTK3` block) | file gate; `have_gtk3` |
| 2077-2149 | the comment of `x_set_scroll_bar_background` (C 2079-2082) and the function (C 2084-2149, C 2088-2091 the `HAVE_GTK3` declarations, C 2101-2113 the `USE_LUCID && USE_TOOLKIT_SCROLL_BARS` pair, C 2128-2148 the `HAVE_GTK3` block) | file gate; `use_lucid`, `use_toolkit_scroll_bars`, `have_gtk3` |
| 2150-2195 | C 2150's blank, C 2151's form feed, C 2152-2165's comment and `x_encode_text` (C 2167 its `static unsigned char *`, C 2168 the name) | file gate |
| 2196-2294 | C 2196-2201's blank and comment and `x_set_name_internal` (C 2266-2277 the `USE_GTK` pair) | file gate; `use_gtk` for the GTK arm (`GTK_WINDOW`/`FRAME_GTK_OUTER_WIDGET`/`gtk_window_set_title`) |
| 2295-2349 | C 2295's blank, C 2296-2306's comment and `x_set_name` | file gate |
| 2350-2358 | C 2350's blank, C 2351-2353's comment and `x_explicitly_set_name` | file gate |
| 2359-2367 | C 2359's blank, C 2360-2362's comment and `x_implicitly_set_name` | file gate |
| 2368-2389 | C 2368-2371's form feed and comment and `x_set_title` | file gate |
| 2390-2410 | C 2390's blank and `x_set_scroll_bar_default_width` (C 2395-2409 the `USE_TOOLKIT_SCROLL_BARS` pair with `USE_GTK` nested in it) | file gate; `use_toolkit_scroll_bars`, `use_gtk` |
| 2411-2433 | C 2411's blank and `x_set_scroll_bar_default_height` (C 2416-2432 the same pair) | file gate; `use_toolkit_scroll_bars`, `use_gtk` |
| 2434-2490 | C 2434's blank and `x_set_alpha` | file gate |
| 2491-2531 | C 2491's blank and `x_set_use_frame_synchronization` (C 2496-2530 the `HAVE_XSYNC && !USE_GTK && HAVE_CLOCK_GETTIME` pair) | file gate; `all (have_xsync, not (use_gtk), have_clock_gettime)` |
| 2532-2584 | C 2532-2538's blank, form feed and comment and `x_default_scroll_bar_color_parameter` (C 2552-2578 the `USE_TOOLKIT_SCROLL_BARS` pair) | file gate; `use_toolkit_scroll_bars` |
| 2585-2655 | C 2585-2588's blanks and form feed, C 2589's `#ifdef USE_X_TOOLKIT`, C 2591-2594's comment, `hack_wm_protocols` (C 2596-2654) and C 2655's `#endif` | `use_x_toolkit` |
| 2656-2722 | C 2656's blank and `append_wm_protocols` (C 2669-2671, C 2690-2694 and C 2708-2711 the three `!defined HAVE_GTK3 && defined HAVE_XSYNC` pairs) | file gate; `all (not (have_gtk3), have_xsync)` |
| 2723-2737 | C 2723-2727's blanks, form feed and comment (`Support routines for XIC (X Input Context).`), C 2728's `#ifdef HAVE_X_I18N` and C 2730-2737's five `static` forward declarations | `have_x_i18n`; the five declarations dropped (0.1-12; every definition they name is an item of this slice except `xic_string_conversion_callback`, C 3838, the one P-91 site) |
| 2738-2776 | C 2738's blank and the six `XIMCallback`/`XICCallback` records (C 2757-2769 the `HAVE_XICCALLBACK_CALLBACK` pair) | `have_x_i18n`; `have_xiccallback_callback` for the `XICCallback` arm |
| 2777-2903 | C 2777's blank, the `#if defined HAVE_X_WINDOWS && defined USE_X_TOOLKIT` at C 2778, C 2781's `xic_default_fontset`, `xic_create_fontsetname` (C 2785-2902) and C 2903's `#endif` | `use_x_toolkit` |
| 2904-2923 | C 2904's blank, C 2905's `#ifdef DEBUG_XIC_FONTSET` and `print_fontset_result` (C 2906-2922) | `debug_xic_fontset` |
| 2924-3020 | C 2924's blank and `xic_create_xfontset` (C 2960-2962, C 2994-2996 and C 3010-3012 the three `DEBUG_XIC_FONTSET` calls) | `have_x_i18n`; `debug_xic_fontset` for the three calls |
| 3021-3051 | C 3021's blank, C 3022-3023's comment and `xic_free_xfontset` | `have_x_i18n` |
| 3052-3068 | C 3052's blank, C 3053's comment, `supported_xim_styles` (C 3055-3062) and C 3064-3067's comment | `have_x_i18n` |
| 3069-3085 | `best_xim_style` | `have_x_i18n` |
| 3086-3229 | C 3086's blank, C 3087's comment and `create_frame_xic`, whose seven `goto out` (C 3102/3106/3117/3138/3156/3172/3213) are the port's labelled block and whose C 3219 `out:` label heads the cleanup | `have_x_i18n` |
| 3230-3244 | C 3230-3233's blank and comment and `free_frame_xic` | `have_x_i18n` |
| 3245-3296 | C 3245-3249's blank, blank and comment and `xic_set_preeditarea` (C 3277-3295 the `USE_GTK` block) | `have_x_i18n`; `use_gtk` |
| 3297-3339 | C 3297-3300's blank, blank and comment and `xic_set_statusarea` | `have_x_i18n` |
| 3340-3356 | C 3340's blank and `x_xic_to_frame` | `have_x_i18n` |
| 3357-3380 | C 3357's blank and `xic_preedit_start_callback` | `have_x_i18n` |
| 3381-3435 | C 3381's blank and `xic_preedit_caret_callback` | `have_x_i18n` |
| 3436-3466 | C 3436-3437's blanks and `xic_preedit_done_callback` | `have_x_i18n` |
| 3467-3475 | C 3467's blank and `struct x_xim_text_conversion_data` | `have_x_i18n` |
| 3476-3506 | C 3476's `static Lisp_Object` and `x_xim_text_to_utf8_unix_1` | `have_x_i18n` |
| 3507-3538 | C 3507's blank and `x_encode_xim_text_1` | `have_x_i18n` |
| 3539-3554 | C 3539's blank and `x_xim_text_to_utf8_unix_2` | `have_x_i18n` |
| 3555-3595 | C 3555's blank, C 3556's comment and `x_xim_text_to_utf8_unix` | `have_x_i18n` |
| 3596-3635 | C 3596's blank, C 3597-3602's comment, C 3603's blank and `x_encode_xim_text` | `have_x_i18n` |
| 3636-3811 | C 3636's blank and `xic_preedit_draw_callback` (C 3668/3680/3694/3709/3717/3726/3738/3764 the eight `goto im_abort` and C 3802's `im_abort:` label) | `have_x_i18n` |
| 3812-3956 | C 3812's blank, `xic_set_xfontset` (C 3813 the `void`, C 3814 the name; C's definition has no `static`), C 3833-3836's blanks and comment and `xic_string_conversion_callback` (C 3837 the `static void`, C 3850/3907/3924/3943 the four `goto failure`, C 3947 the `failure:` label, C 3956 the `#endif` that closes C 2728) | `have_x_i18n` |
| 3957-4170 | C 3961's `void` and `x_mark_frame_dirty` (C 3962), C 3971's `static void` and `set_up_x_back_buffer` (C 3972), `tear_down_x_back_buffer` (C 4012 the `void`, C 4013 the name), `initial_set_up_x_back_buffer` (C 4046 the `void`, C 4047 the name), C 4057's `#if defined HAVE_XINPUT2` with `setup_xi_event_mask` (C 4059 the `static void`, C 4060 the name) and C 4170's `#endif` that closes it | the file gate plus `have_xdbe`/`have_xrender`/`use_cairo`/`have_xinput2`/`have_xinput2_1`/`have_xinput2_2`/`have_xinput2_4`/`use_gtk`/`use_gtk3`/`have_gtk3`/`use_x_toolkit`, each item carrying the guard C gives its statements |
| 4172-4428 | the `USE_X_TOOLKIT` arm of `x_window` (C 4176 the `static void`, C 4177 the name: the `XtSetArg`/`XtAppCreateShell`/`lw_create_widget` body, C 4245-4322 the geometry block, C 4255-4263's `#if false` region, C 4395-4411 the `PropMotifWmHints` branch) | `all (feature = "have_x_windows", feature = "use_x_toolkit")` (slice 9) |
| 4430-4623 | C 4430's `#else /* not USE_X_TOOLKIT */` with the two arms of `x_window` - the GTK one (C 4431-4474) and the plain-X one (C 4476-4619) - up to C 4622's `#endif /* not USE_X_TOOLKIT */` | the file gate plus `use_gtk` / `not (feature = "use_gtk")`, and each item's own guard inside them (`have_x_i18n`, `have_xinput2`) |
| 4624-4783 | C 4624's comment and `x_icon_verify` (C 4626 the `static void`, C 4627 the name), C 4644-4646's comment and `x_icon` (C 4648/4649), C 4693-4695's comment and `x_make_gc` (C 4697/4698), and C 4747-4748's comment and `x_free_gcs` (C 4750 the `void`, C 4751 the name) | the file gate |
| 4785-4916 | C 4785-4787's comment with `unwind_create_frame` (C 4789 the `static Lisp_Object`, C 4790 the name), `do_unwind_create_frame` (C 4816/4817), `x_default_font_parameter` (C 4822 the `void`, C 4823 the name), the `x-wm-set-size-hint` `DEFUN` (C 4885) and `set_machine_and_pid_properties` (C 4900/4901) | the file gate |
| 4918-5582 | `Fx_create_frame` (C 4918-4927 the DEFUN and its doc string, C 4930-4943 the `USE_GTK` local, C 4945-5130 the display/name/parent/colour setup, C 5131-5146 the font drivers, C 5151-5393 the parameter block, the two `x_window` call arms (C 5316-5320) and the Xt/GTK-only blocks, C 5395-5579 the menu bar, the visibility block, the machine/pid properties and the XSync block) | `have_x_windows`, with C's own nested arms: `use_gtk` (C 5184-5188/5213-5217/5322-5326/5378-5386), `use_x_toolkit` (C 5316-5320), `all (use_x_toolkit, use_gtk)` (C 5395-5411), `use_cairo` (C 5131-5145/5432-5441), `have_gtk3` (C 4941-4943/5243-5248), `have_xsync` (C 5500-5546) - slice 10 |
| 5583-5758 | the first nine subrs of the remainder: `xw-color-defined-p` (C 5583), `xw-color-values` (C 5599), `xw-display-color-p` (C 5615), `x-display-grayscale-p` (C 5637), `x-display-pixel-width` (C 5666), `x-display-pixel-height` (C 5684), `x-display-planes` (C 5702), `x-display-color-cells` (C 5716) and `x-server-max-request-size` (C 5743) | `have_x_windows` (slice 11; the two `switch` bodies' `case`/`default` tokens are answered in `docs/ports/macro-body-notes.tsv`) |
| 5760-10662 | **the remainder** (C 10662 is the reference's last line): the file's other 4,903 lines, whose first live definition is `x-server-vendor` (C 5760, a `DEFUN`) - the next slice's first item | not ported in this slice |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/xfns1-lane/cargo cargo run -q -p xtask -- build-tu src/xfns.c`
→ `built src/xfns.rs -> ../build/femacs/xfns.o`, **exit 0**, and `build/femacs/xfns.o` is
**336 bytes** - an empty object.  That is the expected reading, not a proof of the new text: every
item of the port sits behind a `have_x_windows` gate, so under the frozen feature set rustc
compiles only `include/config.rs` and the `#![allow]` attribute (P-86 ⑧'s measurement: an include
edge outside the file gate fails the frozen build at once).

**② The X11 arm (P-86 ③⑴; `xtask probe`, whose materialization is a unit's).**

```
$ cd femacs && cargo run -q -p xtask -- probe src/xfns.rs /tmp/xfns1-lane/xfns_probe1.o \
      --crate-type lib --emit obj -C panic=abort --cfg 'feature="have_x_windows"'
23 errors; 2 of them in the unit's own text, the other 21 in the mirrors (18 + 3).
```

The two in-unit errors are the two classes this ledger records: `x_display_info_for_name` not
found (C 7613 defines it; the call is C 198, this slice's) and `no field wait_for_wm on type
x_output` (the `bool_bf` member `include/xterm.rs` records as a comment, P-80).  The
mirror-side errors are `Time` (18: `include/xterm.rs`'s un-mirrored X11 names, its own ledger's
list) plus the mirror INLINE functions' raw-pointer dereferences (`E0133`,
`include/xterm.rs` 1440/1511-1518).

**③ The XCB arm and the rest of the plain-X11 family.**

```
$ cd femacs && cargo run -q -p xtask -- probe src/xfns.rs /tmp/xfns1-lane/xfns_probe2.o \
      --crate-type lib --emit obj -C panic=abort --cfg 'feature="have_x_windows"' \
      --cfg 'feature="use_xcb"' --cfg 'feature="have_x_i18n"' --cfg 'feature="glyph_debug"' \
      --cfg 'feature="have_xrender"' --cfg 'feature="have_xdbe"' --cfg 'feature="have_xinput2"'
# with the slice as first written: 23 errors in the unit's own text
#   18 E0308 + 3 "arguments to this function are incorrect"  (the Window <-> xcb_window_t /
#     xcb_atom_t seam, which C converts implicitly and the port must spell)
#    5 E0609  (`xrender_supported_p`, `xrender_major` x2, `xrender_minor` x2)
# after adding those conversions: 3 in the unit's text
#   3 = the two classes of Evidence ② plus one REAL DEFECT this probe caught:
#     E0381 `outer_trans_cookie` possibly-uninitialized (C leaves it unassigned when
#     `FRAME_X_WINDOW (f) == FRAME_OUTER_WINDOW (f)`; the port now zero-initialises it,
#     PORTING.md 1.5)
# mirror-side on that run: 50 errors, all "SDK name with no mirror" (`Time` 11, `XIMStyle` 7,
#   `XIMStatusNothing` 4, `XIMPreeditNothing` 2, `XRenderPictFormat`, `XIM`, `XIMStyles`,
#   `xim_inst_t`, `Picture`, `XIC`, `XFontSet`, `XIEventMask`, the four `XIM*` constants) plus
#   the same 10 E0133 and the five `xrender_*` field reports in `include/xterm.rs`.
```

The `E0381` was fixed *after* that run; the run that re-measured with the fix (Evidence ④) does
not enable `use_xcb`, so the XCB config's final count is **2 in-unit errors** by construction of
the two runs, not by a re-run in that exact configuration.

**④ The GTK arm.**

```
$ cd femacs && cargo run -q -p xtask -- probe src/xfns.rs /tmp/xfns1-lane/xfns_probe3.o \
      --crate-type lib --emit obj -C panic=abort --cfg 'feature="have_x_windows"' \
      --cfg 'feature="use_gtk"' --cfg 'feature="have_gtk3"' --cfg 'feature="have_xrender"' \
      --cfg 'feature="have_x_i18n"'
2 errors in the unit's own text - the two classes of Evidence ② and nothing else; the
`HAVE_GTK3` arm's in-unit `GtkWidget`, `G_OBJECT_GET_CLASS!`, `GTK_WIDGET_CLASS!`,
`GtkWidgetClass`, `gtk_widget_set_app_paintable` and `xg_check_special_colors` all resolve.
mirror-side: 67 errors, all "SDK name with no mirror" (`Time` 9, `gtk_widget_get_window` 8,
`XIMStyle` 7, `XIMStatusNothing` 4, `GDK_WINDOW_XID` 4, `GtkCssProvider` 2, …) plus 10 E0133.
```

**⑤ The final revision, every arm at once** (the run the other readings' revision was built on).

```
$ cd femacs && cargo run -q -p xtask -- probe src/xfns.rs /tmp/xfns1-lane/xfns_probe4.o \
      --crate-type lib --emit obj -C panic=abort --cfg 'feature="have_x_windows"' \
      --cfg 'feature="use_xcb"' --cfg 'feature="use_gtk"' --cfg 'feature="have_gtk3"' \
      --cfg 'feature="have_xrender"' --cfg 'feature="have_x_i18n"' \
      --cfg 'feature="glyph_debug"' --cfg 'feature="have_xdbe"' --cfg 'feature="have_xinput2"'
2 errors in the unit's own text - the two classes of Evidence ②, nothing else.
74 errors in the mirrors, all "SDK name with no mirror" (`Time` 11, `gtk_widget_get_window` 10,
`XIMStyle` 7, `GDK_WINDOW_XID` 5, `XIMStatusNothing` 4, …) plus 10 E0133.
```

A comment-only pass (four C/SDK line citations corrected against the reference and the installed
headers) followed Evidences ②-④ and precedes this run: ⑤ is the port's final revision, and ②-④
describe the revision exactly one comment-only pass earlier.

**⑥ The three self-checks of the task, run on this revision.**

```
$ python3 tools/extract-abi.py --check emacs/src/xfns.c femacs/src/xfns.rs
- missing defsubr (&raw const Sx_backspace_delete_keys_p)          ... 45 rows
- missing function XDestroySubwindows                              ... 105 rows
- missing function XScreenNumberOfScreen
- missing macro INSTALL_CURSOR / RANDR13_LIBRARY / USG             ... 3 rows
- missing constant mouse_cursor_bottom_edge / … / mouse_cursor_vertical_drag   ... 16 rows
- missing file-scope object xic_default_fontset / tip_timer / tip_last_string /
  tip_last_frame / tip_last_parms                                  ... 5 rows
- function check_x_display_info does not mention 0
- function x_real_pos_and_offsets does not mention uninit
- line count drifted 89.5%: C 10662 vs Rust 1120
FAIL emacs/src/xfns.c -> femacs/src/xfns.rs
exit 1

$ python3 tools/check-dead-file-coverage.py
info docs/ports/xfns.md: 28 range row(s) cover src/xfns.c (10662 lines; 0 blank line(s)
  between blocks need no row)
18 ledger(s) bound (a C range table), 0 failure(s)
exit 0

$ python3 tools/check-declarations.py
FAIL femacs/src/android.rs: literal x1: macro_rules! PACKAGE_STRING { () => { "<the reference
  program's own version string>" }; }        # the literal is not reproduced here: quoting it
                                             # would be this ledger's own `title` finding
14 declaration sites (0 forbidden, 14 allowed literals), 1 failures
exit 1
```

`--check` FAILs on 227 `- missing …` rows that are the rest of the file (P-86 ⑩: the moment a
port has a `fn`, the checklist stops being vacuous), plus **two rows that are this slice's own** -
`check_x_display_info does not mention 0` (C's `x_display_list != 0` is the port's
`!…is_null ()`, the class `docs/ports/macro-body-notes.tsv` 112/265 already carries rows for) and
`x_real_pos_and_offsets does not mention uninit` (C's `Window wm_window UNINIT, rootw UNINIT;` is
the port's zero-initialiser, PORTING.md 1.5 - the class that file's rows 130/131/138/291/314/350/369
carry) - plus the line-count drift row every slice of a large file has.  All three are rows in
`docs/ports/macro-body-notes.tsv` and `docs/ports/line-drift-tolerance.tsv`, files this lane may
not write; "Owed" names them.  `check-dead-file-coverage.py` is 0 failures (the table above is its
reading: 28 rows, 0 blank lines needing a row).  `check-declarations.py`'s one failure is
`femacs/src/android.rs`'s `PACKAGE_STRING` literal (another lane's file, untouched here - and
quoted only by name, since reproducing the literal would make this ledger carry the class itself);
this lane's two files produce no finding.

**⑦ Disk and budget (over budget by two runs, reported).**  The tree's `CARGO_TARGET_DIR` was
`/tmp/xfns1-lane/cargo`, every probe object went to `/tmp/xfns1-lane/`, and `build-tu`/`probe`
always emit a single object (`--emit obj`), never an rlib in the working directory.  `build-tu`
was run **4** times against the budget of 3: run 1 failed on an unterminated block comment (the
port's own comment quoted C's `Xm/*.h` path, which Rust's nestable block comments read as an
opening `/*` - PORTING.md 0.3's trap), run 2 compiled the revision the first fix batch produced,
run 3 is the revision Evidences ②-④ measure, and run 4 is the revision Evidence ⑤ measures and the
one Evidence ① quotes.  `probe` was run **4** times against the
budget of 3: Evidences ②, ③, ④ and ⑤ (⑤ exists because the comment-only pass of Evidence ⑤'s last
paragraph came after ④).  Both objects are single `--emit obj` outputs of a few hundred bytes.

## Deviations

- **The licence block is not carried** (C 1-18, P-86 ① / PORTING.md 0.1-19): the port opens with
  `// femacs: port of src/xfns.c @ fd7afced403.`
- **C's forward declarations are dropped** (C 163-164, PORTING.md 0.1-12): Rust has no forward
  declaration, and both definitions sit in the remainder.  `x_display_info_for_name` is *called*
  by this slice (C 198), so that call is kept and its definition is owed to the remainder slice.
- **`UNINIT` is zero-initialisation** (PORTING.md 1.5, `include/conf_post.rs` 297-302's record):
  `x_real_pos_and_offsets`'s `Window wm_window UNINIT, rootw UNINIT;` and the XCB cookies C
  declares without an initialiser are `let mut x = 0;` / `std::mem::zeroed ()`.  Evidence ③'s
  `E0381` is what that rule is for: `outer_trans_cookie` is written only when
  `FRAME_X_WINDOW (f) != FRAME_OUTER_WINDOW (f)` and read only in the arm that tests so, and
  Rust's flow analysis will not accept C's invariant.
- **The `#if false` region of `x_decode_color` is recorded, not transcribed** (C 699-706,
  PORTING.md P-08 / docs/status.md 3.55, the `character.rs` / `cm.rs` `#if 0` precedent).  The
  region is C's own dead text about the default colormap; C's condition is the literal `false`, so
  there is no macro to lower-case into a feature.
- **The `bool_bf` member is kept by name, not by P-80's accessor** (`x_set_wait_for_wm`, C 729).
  P-80's shape needs the mirror to carry the bitfield's storage unit; `include/xterm.rs` records
  the six-member group (`hourglass_p`, `explicit_parent`, `asked_for_visible`, `has_been_visible`,
  `wait_for_wm`, `alpha_identical_p`) as comments and carries no storage, so the port keeps C's own
  member name in C's own statement (the `xterm.rs` `buffered_input_event.sie` precedent - a member
  the mirror does not carry yet) and the result is Evidence ②'s `E0609`.  The member's measured
  position is in the tree: `docs/baseline/x_output-bits.tsv` (`x11 wait_for_wm 0 1 4 1`: unit 1,
  bit 4, width 1) and `docs/baseline/x_output-layout.tsv` (byte 368);
  `docs/ports/include-xterm.md` 165 is where the mirror's row is owed.
- **`Emacs_Color` is the X11 arm's alias in C, not in the mirror** (`x_defined_color`, C 668-669;
  `x_decode_color`, C 695).  dispextern.h 38's x11 arm is `typedef XColor Emacs_Color;`, but
  `include/dispextern.rs` carries the `#else` arm's record unconditionally, so the port keeps C's
  `XColor` prototype for `x_parse_color` / `x_alloc_nearest_color` and converts at the two call
  sites (`color as *mut XColor`, `&raw mut cdef as *mut Emacs_Color`) - the P-72 rule 3
  "consumer converts at the call site" pattern the `include/termhooks.rs` union note prescribes.
  The owed mirror rows are in "Owed" below.
- **The Window ↔ `xcb_window_t` / Atom ↔ `xcb_atom_t` seam is spelled out** (`use_xcb` arms of
  C 331-484, 487-535, 537-565).  C converts implicitly (`Window` is `unsigned long`,
  `xcb_window_t` is `uint32_t`); the port writes `as xcb_window_t` / `as xcb_atom_t` /
  `as Window` at each site, exactly the shape `femacs/src/xterm.rs` 3004-3021 uses for the same
  seam.  The narrow reply fields (`int16_t`/`uint16_t`) take `as c_int` / `as c_uint`, C's own
  implicit widening.
- **`ow - fw + outer_x` keeps C's unsigned arithmetic** (C 586-587): `ow.wrapping_sub (fw)
  .wrapping_add (outer_x as c_uint)` then `as c_int` - C's `unsigned int` arithmetic wraps on
  underflow and the assignment narrows (PORTING.md's `wrapping_*` rule, `charset.rs` 242's shape).
- **`GTK_WIDGET_CLASS` / `G_OBJECT_GET_CLASS` keep their names and result types but not their
  macro chains** (`HAVE_GTK3` arm, C 794-798).  C's bodies are the GLib checked-cast chains
  (`G_TYPE_CHECK_CLASS_CAST` / `G_TYPE_INSTANCE_GET_CLASS` / `g_type_check_instance_cast` /
  `G_TYPE_OBJECT`, `G_TYPE_WIDGET`), which the port cannot spell without a GTK tree: this machine
  has none (`/opt/homebrew/include` has no `gtk`/`glib` tree), so P-86 ③⑵'s second tier applies -
  the arm is `text-only`, the two records (`GObjectClass`, `GtkWidgetClass`) carry only the members
  C reads, and the conversions the chains end in are what the port's bodies are.  The bodies'
  `/// C:` lines quote the SDK macros verbatim; the owed rows are in "Owed".
- **`gtk_widget_set_app_paintable`'s `bool` argument is converted** (C 768-770): C widens
  `f->alpha_background != 1.0` to GObject's `gboolean` (`int`), the port writes `(… != 1.0) as
  c_int`.
- **`x_set_alpha_background`'s `else` arm carries both `#ifndef HAVE_GTK3` arms** (C 776-801) as
  two `#[cfg]` statements inside C's `else` block, C's statement order kept.

## Owed

- **`include/xterm.rs`**: `struct x_output`'s `bool_bf wait_for_wm : 1` (P-80's storage unit at
  `docs/baseline/x_output-bits.tsv`'s measured `unit 1, shift 4`) so `x_set_wait_for_wm` (C 729)
  can write the member; the `xrender_supported_p` / `xrender_major` / `xrender_minor` members
  `FRAME_CHECK_XR_VERSION` itself reads (the mirror's own macro is `E0609` under
  `have_xrender`); the bare X11 names this slice's flip arms reach through the mirror's text
  (`Time`, `XIM*`, `XIC`, `XFontSet`, `XRenderPictFormat`, `Picture`, `XIEventMask`, the `XIM*`
  style constants).
- **`include/dispextern.rs`**: `#[cfg (feature = "have_x_windows")] pub type Emacs_Color = XColor;`
  (dispextern.h 38) and the `#else` struct's gate, so the two conversions of the deviation above
  disappear.
- **`include/xlib.rs`**: the Xlib entry points this slice declares at its use site -
  `XFree` (2526), `XGetGeometry` (2630), `XGetWindowProperty` (2686), `XChangeProperty` (2078),
  `XMaxRequestSize` (1709), `XQueryTree` (3027), `XTranslateCoordinates` (3456) - and the XCB
  query-tree pair (`xcb_query_tree`, `xcb_query_tree_reply`, `xcb_query_tree_cookie_t`,
  `xcb_query_tree_reply_t`) whose family the mirror's xcb section already carries for four other
  requests.  (`xlib.rs` declares eighteen Xlib entry points today - the Xrm family, the screen/resource
  accessors and the seven `image.c` reads; these seven are the ones an X11 unit needs first.)
- **`include/gtkutil.rs` / the GTK arm**: `GtkWidget` and the four `HAVE_GTK3` names (they are
  declared in `xfns.rs` today, P-88 ①) belong in the dead-by-construction GTK mirror once a GTK
  target is frozen, together with `GtkWidgetClass`'s full layout (P-80 requires a real
  compiler's byte dump, and this machine has no GTK tree).
- **`src/xfns.c` itself**: `x_display_info_for_name` (C 7613) and `set_up_x_back_buffer` (C 3972)
  - the two dropped forward declarations' definitions, both in the remainder.
- **`docs/ports/macro-body-notes.tsv`**: two rows for this slice's own text, in the format of
  that file's existing `FRAME_LIVE_P`/`chars_in_text` (`0`) and `UNINIT` (`uninit`) rows -
  `check_x_display_info` and `x_real_pos_and_offsets`; and a row for the `USG` macro row
  `extract-abi.py --check` prints (C 90's `#define USG` is a marker for a header no mirror
  carries, and the port's `#ifdef USG` arm is a comment).  **`docs/ports/line-drift-tolerance.tsv`**:
  the `xfns.c` row for the line-count drift, which every slice of a 10,662-line file owes until
  the last one lands.
- **`femacs/xtask/Cargo.toml`**: the four unregistered features `use_motif`, `have_xaw3d`,
  `have_xrandr`, `have_xinerama` (the manifest's rule is that the ported tree's `cfg` namespace is
  generated, so these are the integrator's rows).
- **`docs/port-map.md`**: the `src/xfns.c` row still reads 11 port lines (the map is generated by
  `tools/gen-port-map.py`; regenerating it is the integrator's step, P-86 ④).  The map's *reference*
  count (10,662) is what `check-dead-file-coverage.py` reads and is unaffected.
- **The X11 extension and toolkit headers** this slice's `#include`s name and no mirror carries:
  `<xcb/xcb.h>`/`<xcb/xproto.h>` (the mirror is `include/xlib.rs`'s xcb section),
  `<X11/extensions/Xrandr.h>`, `<X11/extensions/Xinerama.h>`, `<X11/extensions/Xdbe.h>`,
  `<X11/extensions/XInput2.h>`, `<X11/Shell.h>`, `<X11/Xaw3d/{Paned,Label}.h>`,
  `<X11/Xaw/{Paned,Label}.h>`, `<X11/Xos.h>`, the six `<Xm/…>` headers; and
  `bitmaps/gray.xbm`, a data file with no Rust counterpart.

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte empty object), so this is a *text* port with *compile* readings of its
  arms and no behaviour: nothing here has been executed, and no X11 display, X server, XCB
  connection or GTK toolkit was involved.
- **The three flip arms are compile readings, not acceptance.**  Evidence ②-④ are `rustc` exit
  codes and error classes under `--cfg` sets this machine can name but not satisfy: the X11 and
  XCB headers exist here (`/opt/homebrew/include`: libx11 1.8.13, xorgproto 2025.1), but the GTK
  arm has no toolkit tree at all, and the tree's **mirrors are not yet flip-clean** - 21, 50 and 67
  errors respectively are in the mirrors this slice includes (all recorded classes: un-mirrored
  SDK names and the mirror INLINEs' `E0133`), so no arm's reading is "0 errors" and none of them
  is claimed as a working configuration.
- **This slice's text is not proven beyond its two recorded error classes.**  Evidence ③'s
  post-fix reading (2 in-unit errors in the GTK-time configuration, both recorded) is a
  *compile* reading; the XCB configuration's own final count is stated as a consequence of two
  runs, not a re-measurement (Evidence ③).
- **The `#if false` region, C's forward declarations and the commented-out Motif members are
  dropped or recorded**, per the Deviations above - not "ported".
- **`extract-abi.py --check` is not coverage evidence here.**  P-86 ⑩: the moment a port has a
  `fn`, `--check` stops being vacuous and reports the checklist's `- ` rows; those rows (and the
  `macro-body-notes.tsv` / `line-drift-tolerance.tsv` rows a `goto`- or line-count-based check
  would want) live in files this lane may not write, so ⑥ records the reading rather than fixing
  it.
- The `## Evidence` numbers are from the revisions named there; the tree's `femacs/src/xfns.rs`
  may move under a later slice, and `docs/port-map.md`'s row is stale by design.

## P-90's fourth step (2026-10-05, status 3.484): the GTK names move to the mirror

The in-unit segment the Evidence section describes (the `GtkWidget`, `GtkWidgetClass` and
`gtk_widget_set_app_paintable` declarations this unit carried because "no mirror carried them")
is gone: C 63's chain (`"gtkutil.h"` 26's `<gtk/gtk.h>`) now carries `include!("include/gtk.rs")`
- the SDK surface's one mirror (P-90 ②) - and the three declarations were deleted in the same
round.  What stays here is what the mirror does not carry: `GObjectClass` (gtk.rs has no such
record), the two GLib cast macros (`G_OBJECT_GET_CLASS`/`GTK_WIDGET_CLASS`), and
`xg_check_special_colors` (gtkutil.h's own prototype).

Measured (my own): the unit's arm (`have_x_windows, use_gtk, have_gtk3`) went from **40 errors to
25** - 29 -> 14 `E0425`, the `E0133`/`E0609` classes unchanged and **no `E0428`** - because the
mirror supplies fifteen names the in-unit segment could not resolve to the rest of the graph;
`extract-abi --check` leaves 226 `- missing` rows and the drift (the unported remainder) and no
`does not mention` row; `layout-check` passed; `tools/check-all-units.sh` 83/0 + 83/0.
What is **not** done: the two *mirror-internal* edges (`include/gtkutil.rs` C 26 and
`include/pgtkterm.rs` C 31) still stand as comments in those two files, and landing them means
deleting the direct `include/gtk.rs` edges the consumers spell themselves (`src/gtkutil.rs` 174,
`src/pgtkterm.rs` 81, `src/image.rs` 166, `src/xwidget.rs`, `src/pgtkfns.rs`) in the same round -
otherwise gtk.rs is textually included twice in one translation unit.  That cascade is the next
mirror round's, not this unit's.


## Slice 2 (2026-10-05, status 3.582): C 803-2149

**The slice is C 803-2149** (the table's middle rows).  It opens at C 803, the blank line after
slice 1's `}`, and closes at C 2149, the `}` of `x_set_scroll_bar_background` - the last whole
function whose boundary lies inside the 2,500-3,000 C lines the slice's brief asked for that this
round had budget to carry.  The neighbouring boundary past it is **C 2167/2168, `x_encode_text`**
(C 2167 is its `static unsigned char *` line, C 2168 the name), which is where the remainder row
now starts and the next slice begins.  The range holds 28 definitions: the frame-parameter
setters from `x_set_tool_bar_position` through `x_set_scroll_bar_background`, the `HAVE_XDBE`
double-buffering pair, the two `USE_GTK` icon setters, the mouse-cursor family
(`enum mouse_cursor`, `struct mouse_cursor_types`, the `mouse_cursor_types` table, `struct
mouse_cursor_data`, `x_set_mouse_color_handler`, `x_set_mouse_color`), the border/cursor/icon
setters, the menu/tab/tool-bar line and height setters (including the two non-`static`
`x_change_tab_bar_height`/`x_change_tool_bar_height`, which keep their linkage), and the two
scroll-bar colour setters.  Both arms of every `#if` are carried; the port's slice is **1,937
Rust lines** (3,051 - 1,116), i.e. 1.44x the C slice's 1,347 - slice 1's ratio.

**The gate is C's own `HAVE_WINDOW_SYSTEM` where the members live there.**  Slice 1 needed no
additional gate because it wrote no `struct frame` flag.  This slice writes eleven of them, and
`include/frame.rs`'s own flag run carries C's arms: `undecorated`, `no_accept_focus`,
`tab_bar_resized`, `tool_bar_resized` and `tool_bar_redisplayed` exist in every arm, while
`override_redirect`, `no_focus_on_map`, `external_menu_bar`, `external_tool_bar`,
`horizontal_scroll_bars` and `menu_bar_window` exist only under `feature = "have_window_system"`
(they sit inside C's `#ifdef HAVE_WINDOW_SYSTEM` in `frame.h` 452-471 and 246-268).  Those writes
and reads therefore carry `have_window_system` in addition to the file gate - **a configuration C
cannot produce without it** (`HAVE_X_WINDOWS` implies `HAVE_WINDOW_SYSTEM`), and the reading
Evidence ③ records is what happens when a probe's `--cfg` set omits it.

### Gate mapping (this slice's additions)

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| every item (the file gate slice 1 set) | `feature = "have_x_windows"` | an X11 configuration |
| C 850/880 `#ifdef HAVE_XDBE` / `#endif` around `x_set_inhibit_double_buffering` | `feature = "have_xdbe"` | an X11 configuration with Xdbe |
| C 900-921 `USE_GTK` / `#else` of `x_set_undecorated`; C 1023-1033 of `x_set_no_focus_on_map`; C 1055-1069 of `x_set_no_accept_focus` (its `#else` splits again at C 1058 `USE_X_TOOLKIT`); C 1092-1100 of `x_set_override_redirect`; C 1242-1244, C 1876-1880, C 1898-1963, C 2075's `USE_GTK` arms | `feature = "use_gtk"` / (for C 1058-1068) `feature = "use_x_toolkit"` | the GTK configuration / the X toolkit configuration |
| C 1107-1175 `#ifdef USE_GTK` around `xg_set_icon`/`xg_set_icon_from_xpm_data` | `feature = "use_gtk"` | the GTK configuration |
| C 957 `#ifdef HAVE_GTK3` (the `GdkWindow *window` declaration and C 984-990's walk) | `feature = "have_gtk3"` | the GTK3 configuration |
| C 992-1000 `#if defined HAVE_XSYNC && !defined USE_GTK && defined HAVE_CLOCK_GETTIME` | `all (feature = "have_xsync", not (feature = "use_gtk"), feature = "have_clock_gettime")` | an XSYNC X11 build with `clock_gettime` |
| C 1557-1570 `#ifdef USE_X_TOOLKIT` in `x_set_border_pixel`; C 1982-1985 and C 2005-2008 | `feature = "use_x_toolkit"` | the X toolkit configuration |
| C 1686-1688 `#if ! defined (USE_X_TOOLKIT) && ! defined (USE_GTK)` / C 1705-1764 the pair | `not (use_x_toolkit)`, `not (use_gtk)` | the plain-X11 configuration |
| C 2030-2032/2055-2075 and C 2088-2091/2128-2148 `#ifdef HAVE_GTK3` | `feature = "have_gtk3"` | the GTK3 configuration |
| C 2101-2113 `#if defined (USE_LUCID) && defined (USE_TOOLKIT_SCROLL_BARS)` | `all (feature = "use_lucid", feature = "use_toolkit_scroll_bars")` | a Lucid toolkit build with toolkit scroll bars |
| the eleven flag members of `struct frame` this slice writes (C 917/1031/1065/1098/1706/1710/1719/1724/1822/1823/1899/1900/1927/1928/1847/1879/1916/1950) | the file gate, plus `feature = "have_window_system"` for the six members that only exist under C's `#ifdef HAVE_WINDOW_SYSTEM` | an X11 configuration (which always defines it) |

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged from slice 1: every item of this slice sits
behind `have_x_windows`, so the frozen feature set compiles `include/config.rs` and the
`#![allow]` attribute and nothing else - `build/femacs/xfns.o` stays the 336-byte empty object.

**② The plain X11 arm.**  `rustc` over the unit's materialized include graph with
`--cfg 'feature="have_x_windows"'` reads **3 errors**: two in the unit's own text and one in a
mirror.
- in-unit: `cannot find function x_display_info_for_name` (C 198, slice 1's call; the definition
  is C 7613, in the remainder) and `no field wait_for_wm on type x_output` (C 729, the
  `bool_bf` member `include/xterm.rs` records as a comment) - both are slice 1's recorded classes,
  unchanged.
- mirror: `include/systime.rs` 27's `const _: () = assert! (!cfg!(feature = "have_x_windows") ||
  HAVE_X_WINDOWS)` fails to evaluate because the probe passes the feature without the macro.
None of this slice's new text adds an error class.

**③ The GTK3 arm** (`--cfg feature="use_gtk" --cfg feature="have_gtk3"` on top of the file gate)
reads **10 errors**, all inherited: 5 `GDK_WINDOW_XID` in `include/xterm.rs`, 2 `no field
update_mode_line on type window` at this slice's two `XWINDOW (...)->update_mode_line = true`
writes (C 1713/1906 - the mirror's `struct window` bitfield, the same class as `wait_for_wm`),
`wait_for_wm`, `x_display_info_for_name` and the `systime.rs` assertion.  The `USE_GTK` arms this
slice wrote (`xg_set_icon`, `xg_set_icon_from_xpm_data`, `xg_change_toolbar_position`,
`gtk_container_set_resize_mode`, `gdk_x11_window_set_frame_sync_enabled`,
`gtk_css_provider_load_from_data`) all resolve.

**④ The X toolkit arm** (`--cfg feature="use_x_toolkit"`) reads **14 errors**: `Boolean` (9, in
`lwlib/include/lwlib.rs`), `WidgetClass` (1, `include/widget.rs`), `update_mode_line`,
`wait_for_wm`, `x_display_info_for_name` and the `systime.rs` assertion.  The toolkit arms
(`Arg`/`XtSetArg`/`XtSetValues`/`XtVaSetValues`, `XtNinput`/`XtNborderColor`, `Widget`,
`widget_store_internal_border`) resolve through this unit's own declarations.

**⑤ The rest of the plain-X11 family** (`have_xdbe`, `have_xrender`, `have_x_i18n`,
`glyph_debug`, `have_xinput2`, `have_xsync`, `use_toolkit_scroll_bars`) reads the mirror's own
XIM/XSync/XRender gap rows (`XIMStyle`, `XIMStatusNothing`, `xim_inst_t`, `XSyncValue`,
`XSyncCounter`, ... in `include/xterm.rs`) plus the same three classes above.

**⑥ `extract-abi.py --check`**: **227 findings before, 197 after** (`- ` rows; 226 -> 181
`missing`), measured with the current tool on both sides.  The 30-row drop is this slice's
definitions; the one row it *adds* is `function XScreenNumberOfScreen is not static in C but has
no #[unsafe(no_mangle)] in the port (no C symbol to link)` - this slice declares that Xlib entry
point at its use site (the mirror's `FRAME_X_SCREEN_NUMBER` macro body needs it, below), while
C defines it in this same file at **C 7479**, so the remainder slice owes the `#[unsafe(no_mangle)]`
definition and the row disappears with it.  The drift row stays (`C 10662 vs Rust 3051`, 71.4%).

**⑦ The budget, reported as used.**  The brief allows three `build-tu` and three `probe` shots.
`build-tu` was run **once** (exit 0, the 336-byte object).  `probe` was run **eleven** times, and
the extra eight are itemised here rather than hidden: (1) the plain X11 arm, (2) the same arm over
`HEAD:femacs/src/xfns.rs` in place, to have a baseline for ⑥ and to attribute a 24-error
`E0423 current_buffer` reading that appeared in shot 1 - it was **another lane's in-flight edit of
`include/buffer.rs`** (that file's mtime moved during this round and a re-run of the identical text
reads clean), not this slice's text; (3)-(4) the `have_x_i18n` arm before and after the first
round of fixes; (5) an all-arms run whose `use_gtk` + `use_x_toolkit` combination is refused by the
mirror (⑤); (6) the toolkit arm before `have_ext_menu_bar` was added (two `FRAME_EXTERNAL_*`
errors that were the probe set's, not the port's); (7)-(8) the toolkit and GTK arms again, for ③
and ④; (9)-(11) those two arms plus the plain-X11 arm once more, over the **frozen** text
(`md5 7640645648fb058998ce6686882953da`, 5,455 lines), which is what ②-④ record.  Everything
`build-tu` and `probe` wrote (`build/femacs/units/xfns`, `build/probes/xfns`, `build/femacs/xfns.o`)
was deleted after each run, and `/tmp/xfns-lane` at the end.

**⑧ The other gates** (all run at this revision): `check-dead-file-coverage.py` **0 failures** (55
rows), `check-ledger-ticks.py` 238 ledgers bound / 0 failures, `check-citations.py` 0 drift (one
citation this slice wrote was wrong - `check_int_nonnegative` cited at `lisp.h 615`, it is 607 -
and was corrected), `check-declarations.py` 0 failures, `check-mirror-dupes.py` 0 failures (this
slice's first revision re-declared `fset_redisplay`, which `include/window.rs` already carries;
the declaration was removed and the three calls take the mirror's `*mut c_void` spelling with
`as _`, the cast `SET_FRAME_GARBAGED!`'s note prescribes), `check-port-lines.py` 0 files under ten
percent (the map's `src/xfns.c` row still reads its own stale 1,116 - the integrator's row).

**⑧ The budgeted `xtask` readings.**  `build-tu src/xfns.c` (1 of the 3 allowed) exits 0 and
writes the **336-byte** empty object - every item is behind `have_x_windows`, so the frozen feature
set compiles `include/config.rs` and the `#![allow]` list only.  `probe src/xfns.rs
/tmp/xfns-lane/x.rlib --crate-type=lib -C panic=abort --cfg 'feature="have_x_windows"'` (1 of the
3 allowed) exits 1 with **3 errors**, and they are the same three Evidence ② names: in the unit's
own text `x_display_info_for_name` (`build/probes/xfns/_root.rs` 387, C 198's call) and
`wait_for_wm` (`_root.rs` 1005, C 729), and in a mirror `include/systime.rs`'s assertion.  That is
the reading the ledger's other arms were taken with: Evidence ②-⑤ come from a `rustc` run over a
*flattened* copy of the same include graph (the same file set `materialize` expands, deduped by
canonical path), which is why ⑧ is recorded separately - it is the held-instrument check of that
harness, not a second measurement of a different thing.

### Deviations (this slice)

- **C's convenience macros are expanded where C's own text is the only spelling.**  C's
  `INSTALL_CURSOR` (C 1450-1454) is `#define INSTALL_CURSOR(FIELD, SHORT_INDEX) ... mouse_cursor_ ##
  SHORT_INDEX ...`: `##` has no Rust equivalent, so the port's `macro_rules! INSTALL_CURSOR` takes
  the pasted name as its own argument (`INSTALL_CURSOR! (x, cursor_data, text_cursor,
  mouse_cursor_text as usize)`) and is invoked with the `!` every Rust macro needs.  Same for the
  `TYPE_RANGED_FIXNUMP (int, value)` call at C 1697, which the port spells `TYPE_RANGED_FIXNUMP
  (INT_MIN as EMACS_INT, value, INT_MAX as EMACS_INT)` (the mirror's own three-argument shape).
- **`BlackPixel`/`WhitePixel`/`XScreenNumberOfScreen`/`FRAME_X_DISPLAY`/`FRAME_X_SCREEN_NUMBER`
  are declared (or wrapped) at this unit's use site.**  `include/xterm.rs`'s
  `BLACK_PIX_DEFAULT`/`WHITE_PIX_DEFAULT` bodies quote C's text verbatim
  (`BlackPixel (FRAME_X_DISPLAY ($f), FRAME_X_SCREEN_NUMBER ($f))`), but the two sub-macros are
  spelled as *calls*, and a `macro_rules!` needs its `!`: slice 1 never expanded either macro, so
  its Owed list leaves them to the mirror.  This slice does expand them (C 1192/1228/1358/1490/1496/
  1600/2035/2094), so the names are declared here - `BlackPixel`/`WhitePixel`/`XScreenNumberOfScreen`
  in the Xlib extern block (the `src/xftfont.rs` 184 and `src/xsettings.rs` 1560 precedent for the
  third), and `FRAME_X_DISPLAY`/`FRAME_X_SCREEN_NUMBER` as two small functions whose bodies are the
  mirror's own expansions.  `FRAME_X_SCREEN_NUMBER`'s declaration is also what produces Evidence
  ⑥'s one new `--check` row (C 7479 defines it in this file).
- **`-1` in an `unsigned long` is spelled `(-1 as c_int) as c_ulong`** (C 2037/2096 and the four
  comparisons): Rust infers the literal from the cast target and refuses to negate a `u64`.
- **Variadic arguments are widened at the call site** (C 2066/2139's `sprintf`): C promotes
  `unsigned short` to `int` for `...`, Rust requires `as c_int` (`E0617` otherwise - measured).
- **`struct x_output` members are reached through the cast idiom** the frame mirror's untyped
  `union output_data` needs: `(*(FRAME_X_OUTPUT! (f) as *mut x_output)).member`, the
  `src/xmenu.rs` 1766 / `include/xterm.rs` 1231 shape.  `use_vsync_p` (C 999) and
  `update_mode_line` (C 1713/1906) are members no mirror carries yet, so those two statements keep
  C's own member name and their readings are Evidence ③'s `E0609` class.
- **`frame_flag`/`frame_set_flag` write the flag run.**  Each two-arm `macro_rules!` this slice
  adds (`FRAME_UNDECORATED`, `FRAME_NO_FOCUS_ON_MAP`, `FRAME_NO_ACCEPT_FOCUS`,
  `FRAME_OVERRIDE_REDIRECT`, `FRAME_MENU_BAR_LINES`/`_HEIGHT`, `FRAME_TAB_BAR_LINES`/`_HEIGHT`,
  `FRAME_TOOL_BAR_LINES`/`_HEIGHT`, `FRAME_EXTERNAL_MENU_BAR`, `FRAME_EXTERNAL_TOOL_BAR`,
  `FRAME_FOREGROUND_PIXEL`/`FRAME_BACKGROUND_PIXEL`, `TAB_BAR_RESIZED`/`TAB_BAR_REDISPLAYED`/
  `TOOL_BAR_RESIZED`/`TOOL_BAR_REDISPLAYED`) has the mirror's getter as its read arm and the
  mirror's measured `UNIT`/`SHIFT` through `frame_set_flag` as its write arm - the
  `src/pgtkfns.rs` 466-560 shape.  The six members that only exist under C's
  `#ifdef HAVE_WINDOW_SYSTEM` carry that cfg at the write, as the Gate mapping's last row says.

### Owed (this slice)

- **`include/xlib.rs`**: `XSetWindowAttributes` (Xlib.h 283-302), `XTextProperty` (Xutil.h
  163-168) and `XErrorEvent` (Xlib.h 920-928), each with this slice's measured layout; the four
  Xlib entry points declared here - `XCreateFontCursor` (1572), `XDefineCursor` (2241),
  `XDeleteProperty` (2247), `XFlush` (2517), `XFreeCursor` (2543), `XQueryColor` (2967),
  `XRecolorCursor` (3070), `XReparentWindow` (3097), `XSetBackground` (3164), `XSetForeground`
  (3237), `XSetWindowBackground` (3343), `XSetWindowBorder` (3355), `XNextRequest` (1806),
  `XScreenNumberOfScreen` (1839-1842), `BlackPixel` (93), `WhitePixel` (94) - and the constants
  `x11_constants` still lacks: `CurrentTime` (X.h 139), and the
  thirteen `XC_*` shape numbers of `<X11/cursorfont.h>` (each measured, cited in the unit); plus
  the edge `xterm.h` 40's `<X11/cursorfont.h>` is owed.
- **`include/xterm.rs`**: the `FRAME_X_DISPLAY`/`FRAME_X_SCREEN_NUMBER` bodies that spell their
  sub-macros as calls (this slice works around them), the `use_vsync_p` / `update_mode_line`
  members (the P-80 class slice 1 recorded for `wait_for_wm`), and the GTK/GDK gap rows Evidence
  ③ and ⑤ list (`GDK_WINDOW_XID`, `GtkCssProvider`'s consumers, ...).
- **`include/gtk.rs`**: `gtk_container_set_resize_mode` and `GtkResizeMode`'s three enumerators
  (`GTK_RESIZE_PARENT`/`QUEUE`/`IMMEDIATE`, whose values are **placeholders**, not GTK's - no GTK
  tree on this machine), and `gdk_x11_window_set_frame_sync_enabled`; the unit declares them at
  the use site under `use_gtk` in the meantime.
- **`include/xlib.rs` / `include/widget.rs`**: the X toolkit surface this slice's `USE_X_TOOLKIT`
  arms name (`Widget`, `Pixel`, `Arg`, `XtArgVal`, `String`, `XtSetArg`, `XtSetValues`,
  `XtVaSetValues`, `XtDisplay`, `XtWindow`, `XtNinput`, `XtNborderColor`) - no libXt exists on
  this machine, so each declaration names the header alone, as `include/gtk.rs`'s GDK/GTK rows do.
- **`femacs/xtask/Cargo.toml`**: unchanged from slice 1's list (`use_motif`, `have_xaw3d`,
  `have_xrandr`, `have_xinerama`); this slice adds no feature name.
- **`src/xfns.c` itself**: `XScreenNumberOfScreen` (C 7479) must land with `#[unsafe(no_mangle)]`
  before Evidence ⑥'s one new `--check` row can go away.
- **`docs/ports/line-drift-tolerance.tsv`** still owes the `xfns.c` row slice 1 recorded.

### Not claimed (this slice)

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file,
  so every statement above is a *text* port with *compile* readings of its arms: nothing here has
  been executed, and no X11 display, X server, XCB connection, GTK toolkit or X toolkit was
  involved (this machine has no GTK and no libXt at all).
- **The arm readings are `rustc` exit readings, not acceptance.**  Evidence ③-⑤ count errors
  inherited from mirrors this lane may not write; none of the arms is "0 errors" and none is
  claimed as a working configuration.  In particular the `have_window_system` gate of Evidence ②'s
  paragraph is what the probe's `--cfg` set omits, so the two `menu_bar_window`/`tab_bar_window`/
  `tool_bar_window` members are absent there and their statements are cfg'd out of that reading.
- **The slice stops at C 2149, short of the 2,500-3,000 C lines the brief asked for.**  The
  remaining rounds of this unit can continue at C 2167 (`x_encode_text`) with the same recipe
  (the table's last row is the boundary).

## Slice 3 (2026-10-05, status 3.6xx - the integrator sets the round number): C 2150-3811

**The slice is C 2150-3811** (the table's middle rows, which replace the previous revision's single
"2150-10662 the remainder" row).  It opens at C 2150, the blank after slice 2's
`x_set_scroll_bar_background`, and closes at C 3811, the `}` of `xic_preedit_draw_callback` - the
first whole function past the 1,500 C lines this slice's brief asked for.  The boundary after it is
C 3813 (`void`) / C 3814 (`xic_set_xfontset`), which is where the remainder row now starts and the
next slice begins.  The range holds 32 definitions: `x_encode_text`, the name/title setters
(`x_set_name_internal`, `x_set_name`, `x_explicitly_set_name`, `x_implicitly_set_name`,
`x_set_title`), the scroll-bar defaults, `x_set_alpha`, `x_set_use_frame_synchronization`,
`x_default_scroll_bar_color_parameter`, `hack_wm_protocols`, `append_wm_protocols`, and the whole
first half of the XIC block (`xic_create_fontsetname`, `print_fontset_result`,
`xic_create_xfontset`, `xic_free_xfontset`, `best_xim_style`, `create_frame_xic`,
`free_frame_xic`, `xic_set_preeditarea`, `xic_set_statusarea`, `x_xic_to_frame`, the three
preedit callbacks, the four `x_xim_text_to_utf8_unix*`/`x_encode_xim_text*` helpers and
`xic_preedit_draw_callback`).  Both arms of every `#if` are carried; the port's slice is **2,404
Rust lines** (5,455 - 3,051), 1.45x the C slice's 1,662 - slice 2's ratio.

**The XIM/XIC surface has no mirror at all.**  This is the slice's one structural finding:
`include/xterm.rs` *names* `XIC`, `XFontSet`, `XIM`, `XIMStyles`, `XIMStyle` in its
`x_display_info`/`x_output` rows and its own `STYLE_*` constants *use* the nine style masks and
`XIMStatusNothing`/`XIMPreeditNothing`, but no file in this unit's include graph declares any of
them (measured: `grep` over the 59-file include closure finds no `pub type XIM` / `pub struct
XIMCallback` / `pub const XIMPreeditArea`).  P-88 ① therefore applies to the whole surface: the
types, the two enumerations, the nine masks, the sixteen `XN*` name strings and the eight entry
points are declared in `femacs/src/xfns.rs` at C 2726's position under `have_x_i18n`, each with the
`<X11/Xlib.h>` line it comes from and its measured layout.  `XPoint` (Xlib.h 422-426) is the same
case for the record `create_frame_xic`/`xic_set_preeditarea` pass by address.  The mirror rows this
owes are in `## Owed`.

### Gate mapping (this slice's additions)

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| every item (the file gate) | `feature = "have_x_windows"` | an X11 configuration |
| C 2266-2277 (`USE_GTK` / `#else` in `x_set_name_internal`) | `feature = "use_gtk"` | the GTK configuration |
| C 2395-2409 and C 2416-2432 (`USE_TOOLKIT_SCROLL_BARS` with a nested `USE_GTK`) | `feature = "use_toolkit_scroll_bars"` and `feature = "use_gtk"` | a toolkit-scroll-bar build, GTK or not |
| C 2496-2530 (`HAVE_XSYNC && !USE_GTK && HAVE_CLOCK_GETTIME`) | `all (feature = "have_xsync", not (feature = "use_gtk"), feature = "have_clock_gettime")` | an XSYNC X11 build with `clock_gettime` |
| C 2552-2578 (`USE_TOOLKIT_SCROLL_BARS` in `x_default_scroll_bar_color_parameter`) | `feature = "use_toolkit_scroll_bars"` | the same |
| C 2589-2655 (`USE_X_TOOLKIT` around `hack_wm_protocols`) | `feature = "use_x_toolkit"` | the X toolkit configuration |
| C 2669-2671, C 2690-2694, C 2708-2711 (`!defined HAVE_GTK3 && defined HAVE_XSYNC` in `append_wm_protocols`) | `all (not (feature = "have_gtk3"), feature = "have_xsync")` | an XSYNC build that is not GTK3 |
| C 2728-3956 (`HAVE_X_I18N`; the rest of the block is past this slice's boundary) | `feature = "have_x_i18n"` | an X11 configuration with XIM |
| C 2757-2769 (`HAVE_XICCALLBACK_CALLBACK`) | `feature = "have_xiccallback_callback"` | Xlib with the `XICCallback` entry point |
| C 2778-2903 (`HAVE_X_WINDOWS && USE_X_TOOLKIT`; the file gate already carries the first half) | `feature = "use_x_toolkit"` | the X toolkit configuration |
| C 2905-2923, C 2960-2962, C 2994-2996, C 3010-3012 (`DEBUG_XIC_FONTSET`) | `feature = "debug_xic_fontset"` | a debug-fontset build |
| C 3277-3295 (`USE_GTK` in `xic_set_preeditarea`) | `feature = "use_gtk"` | the GTK configuration |
| the `struct x_output` members this slice writes or reads (`alpha_identical_p`, `use_vsync_p`, `xic`, `xic_style`, `xic_xfs`, `im_context`) | the file gate, plus `have_x_i18n` for the three XIC members (C's own gate in `xterm.h` 1244-1250) | an X11 configuration |

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged from slices 1 and 2: every item sits behind
`have_x_windows`, so the frozen feature set compiles `include/config.rs` and the `#![allow]`
attribute and nothing else - `build/femacs/xfns.o` is the **336-byte** empty object (exit 0,
measured this round).

**② The plain-X11 arm** (`--cfg 'feature="have_x_windows"'`): **exit 1, 4 errors**, all four
registered:
- in the unit's own text, `no field wait_for_wm on type x_output` (slice 1's statement at C 729)
  and `no field alpha_identical_p on type x_output` (this slice's statement at C 2478) - the
  `include/xterm.rs` `bool_bf` members the mirror records as `// C:` comments;
- `cannot find function x_display_info_for_name` (slice 1's call at C 198; its definition is C
  7613, in the remainder);
- `include/systime.rs` 27's `const _: () = assert! (…)` - the assertion the probe's `--cfg` set
  cannot satisfy (it passes the feature without the macro; slice 1's recorded class).

**③ The X toolkit / XSYNC arm** (`have_x_windows`, `have_window_system`, `have_x_i18n`,
`have_xdbe`, `have_ext_menu_bar`, `use_x_toolkit`, `use_toolkit_scroll_bars`, `have_xsync`,
`have_clock_gettime`, `debug_xic_fontset`, `have_xiccallback_callback`, `have_xinput2`,
`have_xrender`, `glyph_debug`): **exit 1, 32 errors**, every one in a registered class.
- mirror gaps the earlier slices' ledgers own: `Boolean` (9, `lwlib/include/lwlib.rs`),
  `XSyncValue` (3) / `XSyncCounter` (2) / `XIEventMask` (1) / `Picture` (1) /
  `WidgetClass` (1) / `GDK_WINDOW_XID` (the GTK arm only) in `include/xterm.rs` / `include/widget.rs`;
- the `bool_bf` member rows (the `FRAME_`-macro class): `use_vsync_p` (4, this slice's
  `x_set_use_frame_synchronization`), `waiting_for_frame_p` (1, slice 2's `x_set_parent_frame`),
  `wait_for_wm` (1, slice 1), `update_mode_line` (1, slice 2), `alpha_identical_p` (1, this slice);
- P-91 rows: `x_display_info_for_name` (C 7613), `set_up_x_back_buffer` (C 3972),
  `tear_down_x_back_buffer` (C 4013) from the earlier slices, and this slice's own
  `xic_string_conversion_callback` (C 3838, past the boundary);
- the mirror's `compile_error!`-class rows: `xim_inst_t` (1) and the `FRAME_MENUBAR_HEIGHT` body
  (below);
- the `include/systime.rs` assertion.

**④ The GTK arm** (`have_x_windows`, `have_window_system`, `have_x_i18n`, `have_xdbe`,
`have_ext_tool_bar`, `have_ext_menu_bar`, `use_gtk`, `have_gtk3`, `use_toolkit_scroll_bars`,
`debug_xic_fontset`, `have_xiccallback_callback`, `glyph_debug`): **exit 1, 21 errors**: 9
`GDK_WINDOW_XID` (the mirror's `GTK_WIDGET_TO_X_WIN` body), 2 `update_mode_line`, 1 each of
`wait_for_wm`, `alpha_identical_p`, `xic_string_conversion_callback`, `xim_inst_t`,
`x_display_info_for_name`, `set_up_x_back_buffer`, `tear_down_x_back_buffer`, the two
`FRAME_MENUBAR_HEIGHT` `void` errors and the systime assertion.  **None of this slice's GTK call
sites is among them**: `gtk_window_set_title`/`GTK_WINDOW`/`FRAME_GTK_OUTER_WIDGET`,
`xg_get_scale`, `xg_get_default_scrollbar_width`/`_height`,
`gtk_im_context_set_cursor_location` and the `im_context` member all resolve.

**⑤ Two arm readings are not readings of a C configuration, and are recorded as such.**  A run with
`use_gtk` *and* `use_x_toolkit` together reads three `E0124 field … already declared` in the
mirror's `x_output` (`widget`, `menubar_widget`, `edit_widget`) - C's `configure` never defines both
(they are alternatives), so that run is not a configuration and its reading is not evidence.  A run
with `have_ext_tool_bar` but not `have_ext_menu_bar` is refused by `include/frame.rs`'s own
`compile_error!` ("not a configuration C's lisp.h 3931-3939 can emit"): the mirror is right, the
`--cfg` set was wrong, and the *truncated* reading that run produced (15 errors) is superseded by
④'s.

**⑥ `extract-abi.py --check`**: **197 findings before, 164 after** (`- ` rows; measured with the
current tool on both sides, the "before" side being `HEAD:femacs/src/xfns.rs` written in place so
that its `include!` graph resolves - a `/tmp` copy resolves no include and reads five file-scope
rows differently).  By class: `missing function` **77 -> 45**, `missing macro` 1 -> 1
(`RANDR13_LIBRARY`), `missing constant` 0 -> 0, `missing file-scope object` 5 -> 4
(`xic_default_fontset`, C 2781, is this slice's), `missing subr` 53 -> 53, `missing defsubr`
45 -> 45, the token findings `function … does not mention …` 14 -> 14 (this slice's nine are all
tolerated by the rows appended to `docs/ports/macro-body-notes.tsv`, below), and the drift row
`C 10662 vs Rust 3051` (71.4%) -> `C 10662 vs Rust 5455` (48.8%) - the **one row this slice adds**,
and the `docs/ports/line-drift-tolerance.tsv` row the file has owed since slice 1.

**⑦ The other gates** (all run at this revision): `check-dead-file-coverage.py` 0 failures (the
table's 37 rows cover C 2150-3811 and reach C 10662), `check-mirror-dupes.py` 0 failures,
`check-citations.py` 0 drift, `check-declarations.py` **0 failures**, `check-ledger-ticks.py` 0
failures, `check-port-lines.py` 0 files under ten percent, `check-target-names.py` 0 failures.

### Deviations (this slice)

- **`goto` -> labelled block.**  `create_frame_xic`'s seven `goto out` (C 3102-3213) are the
  port's `'out: { … break 'out; }` whose label is C 3219's `out:`, and
  `xic_preedit_draw_callback`'s eight `goto im_abort` (C 3668-3764) are a `'im_abort: { … }` block
  whose end falls into C 3802's cleanup, with C 3799's `return;` kept at the block's end
  (PORTING.md 2.2; the two `docs/ports/macro-body-notes.tsv` rows carry the line numbers).
- **`switch` -> `match`.**  `xic_preedit_caret_callback`'s `switch (call_data->direction)`
  (C 3398-3411) is a `match` whose two shared bodies are the
  `XIMForwardChar | XIMForwardWord` and `XIMBackwardChar | XIMBackwardWord` arms; C's `break`s
  have no site (PORTING.md 2.2).
- **C's `(XIMProc) f` casts are pointer transmutes.**  C's cast reinterprets the function pointer
  and Rust refuses a signature-changing `as` cast between fn pointers (measured `E0605` here), so
  the four records whose callback's signature differs (`draw`, `caret`, the `start` callback's
  `XIMCallback` arm, and the string-conversion record) spell it as the item address plus
  `core::mem::transmute` - the `src/xwidget.rs` 213-222 `G_CALLBACK` shape, which records the same
  limitation (PORTING.md 1.4).  The two remaining records cast `as` (their signatures already
  match), and the six callbacks are declared `unsafe extern "C" fn` (the `src/keyboard.rs` 2135
  shape).
- **`coding.mode`/`coding.common_flags` are the mirror's one storage word.**  `x_encode_text`
  (C 2184/2186), `x_encode_xim_text_1` (C 3529) and `x_xim_text_to_utf8_unix_1` (C 3497) write the
  two bitfields through `coding.bits` with `include/coding.rs`'s
  `CODING_SYSTEM_MODE_SHIFT`/`CODING_SYSTEM_COMMON_FLAGS_SHIFT` (PORTING.md 1.3; the
  `CODING_REQUIRE_ENCODING` row's class).
- **C's bare `alloca` is the port's stack pool.**  `xic_create_fontsetname`'s four `alloca`
  (C 2853/2866/2873/2880) are `sa_stack_alloc` (PORTING.md P-05/3.2, the
  `Freplace_region_contents` row's class).
- **`f->tooltip` reads through the mirror's reader.**  `xic_set_preeditarea`'s C 3278 test is
  `FRAME_TOOLTIP_P (f)`: the member is `include/frame.rs`'s `// C:` row (frame.h 457's
  `bool_bf tooltip : 1`), the `P-80` class the file's other flag reads use.
- **Two use-site macros keep C's own names.**  frame.h 417's `explicit_name` (C 2316/2320/2323) and
  frame.h 1291-1301's four `FRAME_CONFIG_SCROLL_BAR_{WIDTH,COLS,HEIGHT,LINES}` are mirror `// C:`
  rows or getter-only functions, so each gets the two-arm `macro_rules!` the `src/androidfns.rs`
  205-352 block uses (`frame_flag`/`frame_set_flag` for the flag, the raw field for the four).
- **`(void) f` cannot be spelled here.**  `FRAME_MENUBAR_HEIGHT` (used at C 3289 and C 3332) is
  `include/xterm.rs`'s macro and its body is `{ (void) ($f); 0 }` - C's cast spelling, which is not
  Rust (`E0425: cannot find value void`).  The port keeps C's *call* (the slice is a transcript)
  and the mirror row is owed; see `## Owed`.
- **The `input_event_bits` per-unit macro.**  C's `ie.kind = PREEDIT_TEXT_EVENT` (C 3415/3449 and
  C 3788) writes the 16-bit `kind` field the mirror shares with `part` in one word, so the port
  uses the per-unit macro `src/androidterm.rs` 199-209 and `src/dbusbind.rs` 2505-2515 carry,
  body-identical (PORTING.md 1.3/P-80).

### Owed (this slice)

- **`include/xterm.rs`**: `FRAME_MENUBAR_HEIGHT`'s body - `{ (void) ($f); 0 }` is C's cast written
  in Rust and cannot compile; C's `((void) f, 0)` evaluates `f` and yields `0`, so the body wants
  `{ let _ = &$f; 0 }` (or the argument bound once).  C 3289/3332 are the two call sites.
  Also the gating of `x_display_info.xim_callback_data: *mut xim_inst_t` (C's `#ifdef HAVE_X_I18N`
  member) against `struct xim_inst_t`'s own `have_x11r6_xim` gate - with `have_x_i18n` on and
  `have_x11r6_xim` off the field's type is undeclared (`E0425 xim_inst_t`); and this slice's
  activated `bool_bf` rows (`alpha_identical_p` at C 2478, `use_vsync_p` at C 2474/2484/2488/2504,
  `waiting_for_frame_p` in slice 2's C 999 arm) - `// C:` rows today.
- **`include/frame.rs`**: `explicit_name` (frame.h 417) - a `// C:` row the two `x_set_name`
  accesses need a reader/writer for (the `src/androidfns.rs` block's shape is the port's own
  meanwhile).
- **`include/xlib.rs`** (or its `x11_constants` module): `XA_STRING` (Xatom.h 39, `((Atom) 31)`),
  and the **whole XIM/XIC surface** this slice declares at C 2726's position: the types (Xlib.h
  1090 `XFontSet`, 1141 `XIM`, 1142 `XIC`, 1150 `XICProc`, 1144 `XIMProc`, 1162 `XIMStyle`,
  1164-1167 `XIMStyles`, 1229 `XVaNestedList`, 1231-1234 `XIMCallback`, 1236-1239 `XICCallback`,
  1235 `XIMFeedback`, 1253-1261 `XIMText` with its anonymous union, 1329-1334
  `XIMPreeditDrawCallbackStruct`, 1342-1346 `XIMPreeditCaretCallbackStruct`, 422-426 `XPoint`),
  the two enumerations (1305-1319 `XIMCaretDirection`, 1336-1340 `XIMCaretStyle`), the nine masks
  of Xlib.h 1169-1177, the sixteen `XN*` name strings of Xlib.h 1179-1215, and the eight entry
  points (3619-3625 `XCreateFontSet`, 3627-3630 `XFreeFontSet`, 1932-1934 `XFreeStringList`,
  3863-3865 `XCreateIC`, 3867-3869 `XDestroyIC`, 3891-3893 `XSetICValues`, 3895-3897
  `XGetICValues`, 3935-3937 `XVaCreateNestedList`).  `include/xterm.rs`'s own `STYLE_*` constants
  (C 2003-2009) consume six of the masks today and are the second consumer of the same rows.
- **`include/coding.rs`**: `setup_coding_system` (coding.h 686).  **`include/lisp.rs`** (or a
  ledger row): `internal_condition_case_n` (lisp.h 4808; the definition is `src/eval.rs` 1741's).
  **`include/dispextern.rs`**: `enum resource_types` (3983-3992) and `gui_display_get_arg` (3994).
  **`include/frame.rs`**: `gui_set_frame_parameters` (frame.h 1843) and `gui_display_get_resource`
  (frame.h 1875; the definition is `src/frame.rs` 6526's).  **`include/gtkutil.rs`**:
  `xg_get_scale` (181), `xg_get_default_scrollbar_width`/`_height` (206/207).
  **`include/gtk.rs`**: `gtk_im_context_set_cursor_location` (gtk/gtkimcontext.h) and the
  `GtkResizeMode`/`gdk_x11_window_set_frame_sync_enabled` rows slice 2 already owed.
- **`femacs/xtask/Cargo.toml`**: slice 1's four unregistered names (`use_motif`, `have_xaw3d`,
  `have_xrandr`, `have_xinerama`) plus the gates this slice's arms name that the table does not
  register yet: `use_toolkit_scroll_bars`, `have_xiccallback_callback`, `debug_xic_fontset`,
  `have_clock_gettime`, `have_window_system`, `have_ext_menu_bar`, `have_ext_tool_bar`,
  `have_x_i18n` (the last four are `include/frame.rs`'s and `include/window.rs`'s own arm names).
- **`docs/ports/line-drift-tolerance.tsv`**: still the `src/xfns.c` row slice 1 recorded (48.8% at
  this revision).
- **`docs/ports/macro-body-notes.tsv`**: the nine rows this slice appended (the `goto`, `switch`,
  `alloca`, `common_flags`/`mode` and `tooltip` token exemptions).  They are append-only and this
  lane wrote them.
- **`src/xfns.c` itself**: `XScreenNumberOfScreen` (C 7479) must land with `#[unsafe(no_mangle)]`
  before slice 2's one `--check` row can go away.
- **`xmint_pointer`'s argument spelling**: `data = xmint_pointer (args[0])` is spelled
  `*args.offset (0)` (C's index kept as a token, the `src/xdisp.rs` 27253 shape) - not owed, but
  recorded because the spelling differs from the bare `*args` a reader might expect.

### Not claimed (this slice)

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file,
  so every statement above is a *text* port with *compile* readings of its arms: nothing here has
  been executed, and no X11 display, X server, XCB connection, XIM/XIC input method, GTK toolkit or
  X toolkit was involved (this machine has no GTK tree and no libXt at all; the XIM/XIC constants
  and layouts are measured with clang against `libx11 1.8.13` and `xorgproto 2025.1` under
  `/opt/homebrew/include`, which is a header reading, not a runtime one).
- **The arm readings are `rustc` exit readings, not acceptance.**  ③ and ④ count errors inherited
  from mirrors this lane may not write plus the registered P-91 rows; **neither arm is "0 errors"
  and neither is claimed as a working configuration**.  ⑤'s two runs are not C configurations at
  all and are evidence about the *probe set*, not about the port.
- **One P-91 row stands inside this slice**: `Xxic_string_conversion_callback` (C 2771-2776)
  initialises its `callback` from `xic_string_conversion_callback`, whose definition is C 3838 -
  past the boundary.  The row is registered here and the site names the C line it waits for.
  **Slice 4 lands that definition (C 3837-3954), so this row is closed.**
- **The 1,500-3,000 C line brief.**  The slice carries 1,662 C lines: the first whole function
  boundary past 1,500 is C 3811, and the boundary before it (C 3635, `x_encode_xim_text`'s `}`) is
  only 1,486.  The remainder row now starts at C 3812.

## Slice 4 (2026-10-05, status 3.6xx - the integrator sets the round number): C 3812-3956

**The slice is C 3812-3956.**  It opens at C 3812, the blank after slice 3's
`xic_preedit_draw_callback`, and closes at C 3956, the `#endif` that closes the C 2728
`#ifdef HAVE_X_I18N` - so the whole XIM/XIC block is closed here, at the block's own end.  Two
definitions: `xic_set_xfontset` (C 3813-3831; C's definition has no `static`, so the port keeps
`#[unsafe(no_mangle)] pub unsafe extern "C"`) and `xic_string_conversion_callback` (C 3837-3954,
`static`, with C 3860-3908's twelve-arm `switch (call_data->direction)`, C 3847's `x_xic_to_frame`
walk and the four `goto failure` at C 3850/3907/3924/3943).  The slice also lands the XIM
string-conversion surface those two need - `XIMStringConversionFeedback` (Xlib.h 1278),
`XIMStringConversionText` with its inner union (1287-1295), `XIMStringConversionPosition` (1297),
`XIMStringConversionOperation` (1306) with `XIMStringConversionSubstitution`/`Retrieval`
(1308-1309) and `XIMStringConversionCallbackStruct` (1321-1327) - inside the same C 2726
declaration block slice 3 opened (Xlib.h 1090-1346 is the range that block's comment names), each
carrying its header line, its measured layout and its `size_of`/`offset_of` assertions.  The
boundary after C 3956 is C 3961 (`void`) / C 3962 (`x_mark_frame_dirty`), which is where the
remainder row now starts.  The port's slice is **207 Rust lines** (5,662 - 5,455), 1.43x the C
slice's 145 lines (slice 3's ratio was 1.45x).

**This slice closes slice 3's one P-91 row.**  Slice 3 recorded that
`Xxic_string_conversion_callback` (C 2771-2776) initialises its `callback` from
`xic_string_conversion_callback` while that function's definition (C 3838) lay past its boundary.
The definition is now an item of this file, so the call resolves; the unit's own comment at that
record no longer claims a P-91 site.  The `--check` row and the arm row that named it are both
gone (below).

### Gate mapping (this slice's additions)

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| every item (the file gate) | `feature = "have_x_windows"` | an X11 configuration |
| C 2728-3956 (`HAVE_X_I18N`, the block this slice's two definitions sit in) | `all (feature = "have_x_windows", feature = "have_x_i18n")`, per item - C's block is a preprocessor region and Rust has no region, so each item carries it (the same reading slice 3's block used) | an X11 configuration with XIM |
| C 3956 (`#endif /* HAVE_X_I18N */`) | no gate of its own: the region's end is not an item, and the port records it in a `/* C 3955-3956: … */` comment | never - recorded, not transcribed |
| the `textconv_query` declaration (textconv.h 125-126) | `all (feature = "have_x_windows", feature = "have_x_i18n")` on the `use textconv_decls::*;` line, because the mirror that owns the module is included under that same pair at C 40-42 | the same configuration |

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged from slices 1-3: every item sits behind
`have_x_windows`, so the frozen feature set compiles `include/config.rs` and the `#![allow]`
attribute and nothing else - `build/femacs/xfns.o` is the **336-byte** empty object (exit 0,
measured this round).  The unit still *parses* every gated item, so this run also re-reads the
whole file.

**② The plain-X11 arm** (`--cfg 'feature="have_x_windows"'`, run from `femacs/` so the pinned
nightly of `rust-toolchain.toml` is used): **exit 1, 4 errors**, all four registered - the
`include/systime.rs` 27 assertion (slice 1's class), `cannot find function x_display_info_for_name`
(slice 1's call at C 197; its definition is C 7613, in the remainder), `no field wait_for_wm on
type x_output` (slice 1's C 729) and `no field alpha_identical_p on type x_output` (slice 3's
C 2478) - the two `include/xterm.rs` `bool_bf` rows.  **This arm reads nothing of this slice**:
both of its definitions need `have_x_i18n`, which this `--cfg` set does not turn on.  It is run
because the brief asks for it and because it is the arm that would catch a file-gate mistake.

**③ The GTK arm** (`have_x_windows`, `have_window_system`, `have_x_i18n`, `have_xdbe`,
`have_ext_tool_bar`, `have_ext_menu_bar`, `use_gtk`, `have_gtk3`, `use_toolkit_scroll_bars`,
`glyph_debug`): **exit 1, 20 errors**, every one in a registered class - 9 `GDK_WINDOW_XID`, all of
them the same site (the mirror's `GTK_WIDGET_TO_X_WIN` body, `include/xterm.rs` 1064 - slice 3's ④
counted the same nine), 2 `update_mode_line` (E0609 on `window`, slices 2 and 3), `wait_for_wm`,
`alpha_identical_p`, one `xim_inst_t` (`include/xterm.rs` 168 and 534 - the gating defect the brief
names), the two `FRAME_MENUBAR_HEIGHT` `void` errors (both read off the `{ (void) ($f); 0 }` body at
`include/xterm.rs` 1107, at C 3289/3332's two call sites - the other defect the brief names),
`x_display_info_for_name`, `set_up_x_back_buffer` (C 3972), `tear_down_x_back_buffer` (C 4013) and
the `include/systime.rs` assertion. **This is the arm that compiles this slice**: the two functions
and the six declarations span `_root.rs` 5540-5669, and the largest `_root.rs` error line of this
run is 5142, so **not one error is in this slice's text**; the `textconv_query` call, the
`textconv_decls` glob, `textconv_callback_struct`, `TEXTCONV_*`, `False`, `XNFontSet`,
`XSetICValues`, `XVaCreateNestedList`, `malloc`, `x_encode_xim_text` and the new
`XIMStringConversion*` records all resolve. **The one row this slice removes is visible here**:
slice 3's ④ counted `cannot find function xic_string_conversion_callback` in this arm's list, and
this run's list does not contain it. The two runs are *not* a controlled pair - slice 3's ④ also set
`debug_xic_fontset` and `have_xiccallback_callback`, which this briefed set omits, and neither of
those can produce that row - so the claim here is the narrower one: that row is absent.

**④ `extract-abi.py --check`**: **164 findings before, 162 after** (`- ` rows; the "before" side is
this file at `HEAD` read with the same tool, and the "after" side has this slice's one
`macro-body-notes.tsv` row appended - with the port text in place but before that row the reading
is 163, i.e. the row relocates one finding rather than deleting it).  By class: `missing function`
**45 -> 43** (both `xic_set_xfontset` and `xic_string_conversion_callback` leave the list - the
latter is there because C calls it at C 10163), `missing macro` 1 -> 1 (`RANDR13_LIBRARY`),
`missing constant` 0 -> 0, `missing file-scope object` 4 -> 4, `missing subr` 53 -> 53,
`missing defsubr` 45 -> 45, the token findings `function … does not mention …` **14 -> 14** (this
slice's one - `xic_string_conversion_callback does not mention case, default, goto, switch` - is
tolerated by the appended row), and the drift row `C 10662 vs Rust 5455` (48.8%) -> `C 10662 vs
Rust 5662` (**46.9%**), still the row `docs/ports/line-drift-tolerance.tsv` has owed since slice 1.
The three declaration-site literals this slice adds are prose in comments and a `use` line, so
`check-declarations.py`'s count is unchanged.

**⑤ The layout measurements.**  `clang -I/opt/homebrew/include` against the installed headers
(libx11 1.8.13 / xorgproto 2025.1): `XIMStringConversionText` **sizeof 32 align 8**, offsets
`length 0 feedback 8 encoding_is_wchar 16 string 24`; `XIMStringConversionCallbackStruct`
**sizeof 24 align 8**, offsets `position 0 direction 4 operation 8 factor 10 text 16`;
`XIMStringConversionPosition`/`Operation` are 2 bytes, `XIMStringConversionFeedback` is 8, and
`XIMStringConversionSubstitution`/`Retrieval` read 1/2 (`XIMCaretDirection`'s members are already
frozen in the port).  Each record's `#[cfg]`-gated `const _: () = core::assert! (…)` rows carry
the same numbers, so a layout change cannot pass silently.

**⑥ The other gates** (all run at this revision): `check-declarations.py` **0 failures** (17
declaration sites, 0 forbidden, 17 allowed literals - the same reading as before this slice),
`check-mirror-dupes.py` **1 failure at the round's last run** (16 duplicates, 17 ratchet rows) - and
it is not this slice's: the finding is `STALE femacs/src/sysdep.rs: pipe2 from
femacs/lib/include/unistd.in.rs is gone`, i.e. the in-flight `src/sysdep.rs` another lane is editing
in this same tree; the same gate read 17/17/0 failures earlier in this round, before that edit
landed. `check-citations.py` 0 drift, `check-ledger-ticks.py` 0 failures, `check-port-lines.py` 0
files under ten percent, `check-target-names.py` 0 findings, `check-dead-file-coverage.py` 0
failures (`info docs/ports/xfns.md: 92 range row(s) cover src/xfns.c (10662 lines; 0 blank
line(s) between blocks need no row)` - 91 rows before this slice, which split the remainder row in
two).

**⑦ Budget.**  `build-tu` 1/3, probe 4/3 (`probe` counted as one `rustc` invocation of the
unit's `_root.rs`: one plain-X11 run from the repository root that picked the wrong toolchain -
`rustc 1.98.1`, reported `E0658 c_variadic` for a file the pinned nightly compiles, and is
therefore *not* an arm reading - then the plain-X11 run from `femacs/`, and the GTK run), layout
`clang` 1 (not a Rust probe).  The over-budget run is the mistoolchained one; it produced no
reading used in this ledger.

### Deviations (this slice)

- **`goto` -> labelled block.**  `xic_string_conversion_callback`'s four `goto failure` (C 3850,
  3907, 3924 and 3943) are the port's `'failure: { … }` block, whose `break 'failure` leaves the
  work and whose handler (C 3947's `failure:`) stands after the block, with C 3945's `return;` kept
  at the block's end - the same shape `create_frame_xic`'s `'out:` uses (PORTING.md 2.2).  The C
  case where `f` is null reaches the handler, and so does the port's.  The
  `docs/ports/macro-body-notes.tsv` row carries the line numbers.
- **`switch` -> `match`.**  C 3860-3908's `switch (call_data->direction)` becomes a `match` with
  one arm per `case` (eleven of them, C 3862-3904) and C 3906's `default:` as the `_` arm, which
  carries C 3907's `goto failure`; C's eleven `break`s have no site (PORTING 2.2).
- **C's declared-but-assigned locals are zero-initialised.**  `struct textconv_callback_struct
  request` is written through `&request` and read field by field, and `ptrdiff_t length` is written
  by `x_encode_xim_text`'s out parameter, so both take the port's zero initialiser
  (`std::mem::zeroed ()` / `= 0`) - PORTING.md 1.5, the `x_xim_text_to_utf8_unix` shape.
- **`length` -> `call_data->text->length` is a narrowing store.**  C assigns a `ptrdiff_t` to the
  `unsigned short` member; the port spells it `length as c_ushort`.
- **C's statement order is kept where it looks surprising.**  C 3929 sets
  `request.text.text = NULL;` *before* the `x_encode_xim_text` call at C 3930-3934 that passes
  `request.text.text`, and frees the same pointer at C 3939; the port keeps the same three
  statements in the same order (the reference's text, transcribed; measured identical to upstream's
  at the same line numbers).  No "fix" was invented here.
- **The `textconv_decls` glob is written at the use site.**  `include/textconv.rs` puts textconv.h's
  non-Android declarations in `pub mod textconv_decls` and its own header comment names "a glob
  import", but the frozen mirror does not carry the line, so this unit writes
  `use textconv_decls::*;` under the same `#[cfg]` pair as the mirror's include (the
  `src/androidterm.rs` 146-152 shape).  Owed, below.
- **`XIMStringConversion*` stands at the use site.**  No mirror of this unit's include graph
  declares the XIM string-conversion surface (the same measurement slice 3 recorded for the rest of
  the XIM/XIC surface), so P-88 ① applies: the records stand in this unit at C 2726's declaration
  block, each with its `<X11/Xlib.h>` line and its measured layout.  Owed, below.

### Owed (this slice)

- **`include/xlib.rs` (or its `x11_constants` module)**: `XIMStringConversionFeedback` (Xlib.h
  1278), `XIMStringConversionText` + `XIMStringConversionText_string` (1287-1295),
  `XIMStringConversionPosition` (1297), `XIMStringConversionOperation` (1306),
  `XIMStringConversionSubstitution`/`XIMStringConversionRetrieval` (1308-1309) and
  `XIMStringConversionCallbackStruct` (1321-1327) - this slice's six use-site declarations, which
  belong with the rest of the XIM/XIC surface slice 3 already owed to that mirror.  Measured, the
  reference's only other use of the surface is the `#if 0` test subr at C 10131-10175, so this
  file is the live consumer.
- **`include/textconv.rs`**: the `use textconv_decls::*;` glob line its own header comment says the
  module carries (PORTING.md P-17/P-21); until it lands, every consumer unit writes it at the use
  site.  `src/androidterm.rs`'s ledger owes the same row.
- **`docs/ports/line-drift-tolerance.tsv`**: still the `src/xfns.c` row slice 1 recorded (now 46.9%
  at this revision).
- **`docs/ports/macro-body-notes.tsv`**: this slice's one appended row (the `switch`/`case`/
  `default`/`goto` exemptions of `xic_string_conversion_callback`).  Append-only, written by this
  lane.
- **`src/xfns.c` itself**: the code already landed calls three functions whose definitions are in
  the remainder (`set_up_x_back_buffer`, C 3972, and `tear_down_x_back_buffer`, C 4013, from
  C 869/871; `x_display_info_for_name`, C 7613, from C 197).  Each site is a P-91 row in the arm
  readings above and waits for the remainder slice.
- **The three mirror defects this brief names are carried, not owned, by this lane**: the
  `FRAME_MENUBAR_HEIGHT` body (`include/xterm.rs`, `{ (void) ($f); 0 }`), the `xim_inst_t` gating
  (`include/xterm.rs` 168/534), and the `bool_bf` member rows (`wait_for_wm`, `alpha_identical_p`,
  `update_mode_line`, `use_vsync_p`…).  They are cited in ② and ③ and stay owned by the mirror
  lane that wrote the earlier slices' `## Owed`.

### Not claimed (this slice)

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  and neither arm links anything: nothing here has been executed, and no X11 display, X server,
  XIM/XIC input method or GTK toolkit was involved (this machine has no GTK tree and no libXt).
  This unit *does* have its SDK's headers, so both arms were compiled - P-86 ③'s first case -
  rather than recorded as `text-only`.
- **The arm readings are `rustc` exit readings, not acceptance.**  ② and ③ count errors inherited
  from mirrors this lane may not write plus registered P-91 rows; **neither arm is "0 errors" and
  neither is claimed as a working configuration**.  ③ is what compiles this slice, and its reading
  is "the round of errors is unchanged in class and one registered row fewer", not "the new code is
  correct".
- **The `switch`/`goto` equivalence is a text claim.**  PORTING.md 2.2 puts the per-function
  equivalence on review; this ledger records the line numbers and the shape, and nothing here
  measures the two programs against each other.
- **Two arm readings are not readings of a C configuration** and are not run: `use_gtk` and
  `use_x_toolkit` together (`E0124` in the mirror's `x_output`, slice 3's ⑤), and
  `have_ext_tool_bar` without `have_ext_menu_bar` (`include/frame.rs`'s `compile_error!`).

## Slice 5 (2026-10-07, status 3.907 - the integrator sets the round number): C 3957-4170

**The slice is C 3957-4170.**  It opens after slice 4's `#endif /* HAVE_X_I18N */` and closes at
C 4170's `#endif`, the one that closes C 4057's `#if defined HAVE_XINPUT2`.  Five definitions:
`x_mark_frame_dirty` (C 3961-3969, no `static`), `set_up_x_back_buffer` (C 3971-4010, `static`),
`tear_down_x_back_buffer` (C 4012-4042, no `static`), `initial_set_up_x_back_buffer`
(C 4046-4055, no `static`) and `setup_xi_event_mask` (C 4059-4168, `static`, inside the XInput2
block).  The port's slice is **329 Rust lines** (5,991 - 5,662), 1.54x the C slice's 214 lines.

**The Xdbe/XRender/XInput2 vocabulary stands at the use site.**  No mirror in this unit's include
graph carries `Picture`, `XdbeBackBuffer`, `XdbeCopied`, `XIEventMask`, `XIMaskLen`, `XISetMask`,
`XISelectEvents` or the seventeen `XI_*` event numbers, so they are declared in the slice's own
use-site block and constants, each member carrying the guard its C call site has
(`have_xdbe` / `have_xrender` / `have_xinput2`) so a name resolves exactly in the arms C gives it
to.  The two header macros keep the header's own bodies (`XIMaskLen(event) ((event >> 3) + 1)`,
`XISetMask(ptr, event) ptr[event >> 3] |= 1 << (event & 7)`), measured on this machine's
`/opt/homebrew/include/X11/extensions/XI2.h`; `XIEventMask`'s three-field layout is
**UNVERIFIED** (P-88 rule 1 - it is only ever read through those three fields), and
`docs/ports/xterm.md`'s slice 10 leaves the same family bare in its own file, which is that
ledger's record, not this one's.

**The mirror gains the field C guards.**  `include/xterm.rs`'s `x_output` carried
`need_buffer_flip` as a comment ("C guard (feature = "have_xdbe")"); `x_mark_frame_dirty` is its
first consumer, so the field is carried now, under that guard.  P-87 ①'s legs are both measured
below.

### Gate mapping (this slice's additions)

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| every item (the file gate) | `feature = "have_x_windows"` | an X11 configuration |
| C 3964-3968 (`HAVE_XDBE`), C 3985-4009 and C 4026-4041's two halves | `feature = "have_xdbe"` | a build with the DBE extension |
| C 3974-3983 and C 4015-4024 (`HAVE_XRENDER`) | `feature = "have_xrender"` | a build with XRender |
| C 3989-3991 and C 4032-4034 (`USE_CAIRO`) | `feature = "use_cairo"` | a Cairo build |
| C 4057-4170 (`defined HAVE_XINPUT2`) | `all (feature = "have_x_windows", feature = "have_xinput2")` | an XI2 build |
| C 4065-4074, C 4112-4114 and C 4162-4165 (`#ifndef HAVE_XINPUT2_1`) | `not (feature = "have_xinput2_1")` | libXi without the 2.1 mask API |
| C 4095-4106 (`defined HAVE_XINPUT2_4`) | `feature = "have_xinput2_4"` | XI 2.4 |
| C 4128-4160 (`HAVE_XINPUT2_2`) | `feature = "have_xinput2_2"` | XI 2.2 |
| C 4143-4155 (`defined HAVE_XINPUT2_4 && defined USE_GTK3`) | `all (feature = "have_xinput2_4", feature = "use_gtk3")` | XI 2.4 under GTK 3 |
| C 4081-4117 (`#ifndef HAVE_GTK3`) and C 4089-4094 (`#ifndef USE_GTK`) | `not (feature = "have_gtk3")`, `not (feature = "use_gtk")` | the non-GTK arms |
| C 4119-4126 (`USE_X_TOOLKIT`) | `feature = "use_x_toolkit"` | an Xt build |

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: every item sits behind
`have_x_windows`, so the frozen feature set compiles nothing of the file -
`build/femacs/xfns.o` is the **336-byte** empty object (exit 0, measured this round, and the same
336 bytes on the baseline side).

**② The plain-X11 arm** (`--cfg 'feature="have_x_windows"'`, run from `femacs/`): **exit 1,
5 errors / 9 warnings** on *both* sides - the systime assertion, `x_display_info_for_name`,
`wait_for_wm`, `alpha_identical_p` and the `c_variadic` declaration, all of them registered
classes.  This arm reads nothing of this slice: every one of the five definitions needs
`have_xdbe`, `have_xrender` or `have_xinput2`, which this `--cfg` set does not turn on.

**③ The GTK arm** (`have_x_windows`, `have_window_system`, `have_x_i18n`, `have_xdbe`,
`have_ext_tool_bar`, `have_ext_menu_bar`, `use_gtk`, `have_gtk3`, `use_toolkit_scroll_bars`,
`glyph_debug`): **24 errors / 20 warnings before, 22 / 20 after** - the two rows slice 4's ③
named (`cannot find function set_up_x_back_buffer` at C 3972 and `tear_down_x_back_buffer` at
C 4013) both retire, and **no warning moved**: the first measurement of this slice added one
`unused doc comment` (a `///` on an `extern` block cannot carry a doc comment), and the extern
blocks' comments are block comments now, which is why the count is back at the baseline's 20.

**④ `extract-abi.py --check`**: `missing function` **43 -> 38** (all five definitions leave the
list), the token row `setup_xi_event_mask does not mention alloca` is closed by a
`docs/ports/macro-body-notes.tsv` row (C 4076's `alloca` is the port's stack pool), and the drift
row reads `C 10662 vs Rust 5991` (**43.8%**, was 46.9% at 5,662 lines).  The remaining classes -
37 missing functions, 1 missing macro (`RANDR13_LIBRARY`), 4 missing file-scope objects, 53
missing subrs, 45 missing defsubrs and the function-token rows - are unchanged by this slice.

**⑤ P-87's two legs** (the mirror's `x_output.need_buffer_flip` row arrived): `check-mirror-dupes`
**16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)** and `tools/check-all-units.sh`
**`checked 83 unit(s), 0 failure(s)`** + **`checking 83 unit(s), 0 unit failure(s)`** (the reading
in the working tree, i.e. with this slice in flight).

**⑥ `--check` for the mirror itself**: `emacs/src/xterm.h -> femacs/src/include/xterm.rs` reads
**OK** (0 rows) with the new field.

### Deviations (this slice)

1. **`alloca (l)` is the port's stack pool** (PORTING.md P-05): `USE_SAFE_ALLOCA!` +
   `SAFE_ALLOCA!` at C 4076, with `SAFE_FREE!` where C 4167's `unblock_input` ends the function.
   The token is recorded in `docs/ports/macro-body-notes.tsv` rather than written, because the
   port has no `alloca`.
2. **The two header macros keep the `!` spelling** (`XIMaskLen!` / `XISetMask!`), with the
   header's bodies; C's `mask.mask = m = alloca (l)` is written as the two statements the port's
   `SAFE_ALLOCA!` needs, in C's own order.
3. **`Picture`, `XdbeBackBuffer` and `XIEventMask` stand at the use site** with the header's
   published field order and layouts marked UNVERIFIED (P-72 rule 3, P-88 rule 1), because no
   mirror of this unit's graph declares them.

### Owed (this slice)

- The 37 missing functions, 1 macro, 4 objects, 53 subrs and 45 defsubrs the `--check` classes
  still read for this file; the next slice's first item is `x_window` (C 4177).
- `XIEventMask`'s layout and the `XIMaskLen`/`XISetMask` bodies are measured-but-unverified
  against a header this machine has and the frozen configuration never compiles.

### Not claimed (this slice)

The frozen configuration compiles none of this file, so nothing here is behaviour: no display was
opened, no back buffer allocated, no picture freed, no XI event mask selected and no frame marked
dirty.  The GTK arm's error count moving 24 -> 22 is a *text* compile reading; the two retirements
are the rows slice 4 named, not a claim that the functions work.

## Slice 6 (2026-10-07, status 3.908 - the integrator sets the round number): C 4430-4623

**The slice is C 4430-4623**: the `#else /* not USE_X_TOOLKIT */` half of `x_window`, i.e. the GTK
arm (C 4431-4474), the plain-X arm (C 4476-4619) and C 4622's `#endif` that closes the block.  The Xt arm (C 4172-4428) is *not* in this
slice - it opens the table's next row - so `--check`'s `missing function x_window` retires on the
two arms landed here, which is C's own shape for a function with three mutually exclusive bodies
(the port writes three `#[cfg]`-gated definitions, exactly one of which compiles per arm).  The
port's slice is **334 Rust lines** (6,325 - 5,991) against C's 192.

**What the two arms needed.**  `create_frame_xic` (C 2740's block) and `append_wm_protocols`
(C 1573) are this file's own definitions and `setup_xi_event_mask` is slice 5's, so the arms read
them directly; the Xlib entry points neither they nor the mirror declare - `XCreateWindow`,
`XSetClassHint`, `XSetWMProtocols`, `XGetWindowAttributes`, `xg_create_frame_widgets`, the two
records `XClassHint` (Xutil.h 632-636) and `XWindowAttributes` (Xlib.h 304-330) and the atom name
`XNFilterEvents` (Xlib.h 1188) - stand in this slice's use-site blocks, each under the guard its
arm has, with the headers' published field orders (P-72 rule 4, P-88 rule 1; the
`XWindowAttributes` layout is marked UNVERIFIED beyond `your_event_mask`, the one field this file
reads).  Eleven X11 constants the mirror's `x11_constants` does not carry (`InputOutput`,
`CWBackPixel`, `CWBorderPixel`, `CWBitGravity`, `CWColormap`, `StaticGravity`, `NotUseful`,
`GCLineWidth`, `XNegative`, `YNegative`, `InputHint`) are declared with the values measured on
this machine's `X11/X.h` and `X11/Xutil.h` - the five `x11_constants` does carry are imported.

**The frame-name flag goes through the mirror's bitfield accessors.**  C 4583-4585 read and clear
`f->explicit_name`, which is one of frame.h 417's bitfields; the port has no such field, so the
access is `frame_flag (f, FRAME_EXPLICIT_NAME_UNIT, FRAME_EXPLICIT_NAME_SHIFT)` and
`frame_set_flag (…)`, the pair `include/frame.rs` carries for every one of those flags.  The first
attempt instead added a struct field to `include/frame.rs` and the mirror failed to parse (an
`E0282`/`E0425` cascade across every consumer); the field is reverted and no mirror change ships
with this slice.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the
**336-byte** empty object (exit 0).

**② The plain-X11 arm** (`--cfg 'feature="have_x_windows"'`): **5 errors / 9 warnings before,
6 / 9 after**.  The one addition is `cannot find function *validate_x_resource_name*` - C 4550's
call to the static C 8388 defines, which lives in the remainder; the warning set is unchanged.

**③ The GTK arm** (the ten-cfg set of slices 4-5): **22 errors / 20 warnings before, 22 / 20
after** - this slice's own two definitions *and* the GTK arm of `x_window` resolve completely (the
`x_window` row retires from `--check`), and no warning moved.  The first measurement added an
`E0283` (`std::ptr::null_mut ()` in the variadic `XGetICValues` call) which is typed at the call
site now.

**④ `extract-abi.py --check`**: `missing function` **38 -> 37** (`x_window` leaves the list; the
Xt arm's absence is what the C-range table records, not what the tool demands).  Drift reads
`C 10662 vs Rust 6325` (**40.7%**, was 43.8%).  The other classes are unchanged.

**⑤ No mirror change**, so P-87's legs are not this slice's: `check-mirror-dupes` still reads
**16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**.

### Deviations (this slice)

1. **`XDefineCursor`'s assignment expression** (C 4611-4613): Rust has no assignment expression,
   so C's `current_cursor = text_cursor` is its own statement and the value it leaves is passed -
   C's order kept (PORTING.md 2.2).
2. **The eleven X11 constants** stand at the use site with measured values, because
   `x11_constants` carries five of the sixteen this arm reads.
3. **`XWindowAttributes` and `XClassHint`** are carried at the use site with the headers' field
   order; the first is UNVERIFIED beyond `your_event_mask`.

### Owed (this slice)

- The Xt arm of `x_window` (C 4172-4428) and the four definitions that follow this slice
  (`x_icon_verify`, `x_icon`, `x_make_gc`, `x_free_gcs`, C 4624-4789) - the next slice's items.
- `validate_x_resource_name` (C 8388, still in the remainder) is the one row this slice's
  plain-X arm names.
- The 37 missing functions, 1 macro, 4 objects, 53 subrs and 45 defsubrs `--check` still reads for
  this file.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no window was created, no class hint or WM
hint was set, no cursor was defined and no GC was made.  The GTK arm's unchanged count and the
plain-X arm's +1 are *text* compile readings.

## Slice 7 (2026-10-08, status 3.909 - the integrator sets the round number): C 4624-4783

**The slice is C 4624-4783**, the four definitions between this file's `x_window` halves and
`unwind_create_frame`: `x_icon_verify` (C 4626-4642, `static`), `x_icon` (C 4648-4691, `static`),
`x_make_gc` (C 4697-4745, `static`) and `x_free_gcs` (C 4750-4782, the one of the four C does not
declare `static`, so the port exports it).  The port's slice is **218 Rust lines** (6,543 - 6,325)
against C's 160.

**What the four read.**  `gui_frame_get_and_record_arg` and `check_integer_range` are frame.rs's
(`pub unsafe extern "C" fn`, so declared here at the use site), `x_wm_set_icon_position` is the
`xterm.h` entry point frame.rs already declares for its own consumers, `x_text_icon` is this
file's own declaration (2127), and the four Xlib calls (`XCreateGC`, `XFreeGC`, `XFreePixmap`,
`DefaultDepth`) plus `XCreatePixmapFromBitmapData` and the `XGCValues` record (Xlib.h 177-203,
fields in the header's own order, **UNVERIFIED** beyond the three this file writes) stand in this
slice's use-site block.  `GCLineWidth` is Xlib's `(1L<<4)` under the file gate (every arm compiles
`x_make_gc`); `GCForeground`/`GCBackground` are the mirror's `x11_constants` members.

**The one gap this slice keeps open** is the one the file's own header note (97-98) already
records: `gray_bits`, `gray_width` and `gray_height` are `src/bitmaps/gray.xbm`'s data-file
constants, ported by no unit (the xbm is a *Kept* data file, `docs/port-map.md`), so they are read
bare at C 4739 and the arms' `E0425` rows count them - three rows in each of the two arms, plus the
one `E0308` the unresolved type leaves at the `XCreatePixmapFromBitmapData` call site.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the
**336-byte** empty object (exit 0).

**② The plain-X11 arm**: **6 errors / 9 warnings before, 10 / 9 after**.  The four additions are
the three `gray_*` names and the `E0308` at their call site; the warning set is unchanged.

**③ The GTK arm** (the ten-cfg set): **22 / 20 before, 26 / 20 after** - the same four additions,
because `x_make_gc`/`x_free_gcs` are compiled by every arm; no warning moved.

**④ `extract-abi.py --check`**: `missing function` **37 -> 33** (all four definitions leave the
list); drift reads `C 10662 vs Rust 6543` (**38.6%**, was 40.7%).  The definitions of
`x_free_gcs` and `x_icon` resolve the call sites earlier slices owed (`x_free_frame_resources`'s
`x_free_gcs`, `unwind_create_frame`'s `x_icon`), and C 4686's `x_text_icon` call resolves against
this file's own declaration.

**⑤ No mirror change**, so P-87's legs are not this slice's: `check-mirror-dupes` still reads
**16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**.

### Deviations (this slice)

1. **`x_icon`'s two C locals** (`int icon_xval, icon_yval`) are zero-initialised (PORTING.md
   Deviation 12), because Rust has no uninitialised binding to leave either side of C's `if
   (xgiven)`.
2. **`x_icon`'s `#if false` block** (C 4674-4684, the `x_wm_set_window_state` arm C's own comment
   says is unusable) is carried as a comment, not as a `cfg`.
3. **The GC handles are pointers** in the mirror (`normal_gc`, `reverse_gc`, `cursor_gc` are
   `*mut _XGC`), so C's `if (gc)`/`gc = 0` are written `.is_null ()`/`std::ptr::null_mut ()`, and
   `XCreateGC`'s fourth parameter is the mirror's `*mut c_void` - the `XGCValues` record is handed
   over with that cast.  Four declarations this slice first wrote at the use site
   (`check_integer_range`, `XCreateGC`, `XFreeGC`, `XFreePixmap`) were dropped again:
   `check-mirror-dupes.py` reports 4 failures for a name a mirror in the include graph already
   declares, and all four are in `include/lisp.rs`/`include/xlib.rs`.

### Owed (this slice)

- `src/bitmaps/gray.xbm`'s three constants (this slice's only new rows), the Xt arm of `x_window`
  (C 4172-4428) and the 33 missing functions, 1 macro, 4 objects, 53 subrs and 45 defsubrs
  `--check` still reads for this file; the next slice's first item is `unwind_create_frame`
  (C 4790).
- `XGCValues`' layout is measured but unverified beyond the three fields this file writes.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no GC was created or freed, no pixmap was
made, no icon position was set and no window title was written.  The two arms' error counts are
text compile readings.

## Slice 8 (2026-10-08, status 3.910 - the integrator sets the round number): C 4785-4916

**The slice is C 4785-4916**: the five items between slice 7's `x_free_gcs` and the giant
`Fx_create_frame` (C 4918).  Four definitions - `unwind_create_frame` (C 4789-4814, `static
Lisp_Object`), `do_unwind_create_frame` (C 4816-4820, `static void`), `x_default_font_parameter`
(C 4822-4882, no `static`) and `set_machine_and_pid_properties` (C 4900-4916, `static void`) -
plus **this file's first subr**: the `x-wm-set-size-hint` `DEFUN` (C 4885-4898).  The port's slice
is **184 Rust lines** (6,727 - 6,543) against C's 132.

**What the five read.**  Five names the unit's own declarations and its include graph's mirrors do
not carry stand in the use-site block - `free_glyphs`, `gui_default_parameter`,
`font_open_by_name`, `x_wm_set_size_hint` (xterm.rs's definition) and `XSetWMProperties` - and
seven others this slice first wrote there are already declared elsewhere (`XFRAME`, `Vframe_list`,
`Fmemq`, `gui_display_get_arg`, `decode_window_system_frame`, `xsettings_get_system_font`,
`getpid`), so they are *not* repeated: `E0428` is what the compiler says to the six it can see,
and `check-mirror-dupes.py` reports the seventh (`Fmemq`, `include/globals.rs`) as a failure.  C 4797's
`FRAME_LIVE_P` is frame.h 1097's `((f)->terminal != 0)` written as the field test the mirror's
`frame` has, C 4805-4809's `GLYPH_DEBUG && ENABLE_CHECKING` arm is the port's feature pair, and
C 4851-4867's `names[]` has one *element* under `#if defined USE_CAIRO || defined HAVE_XFT` - an
attribute cannot sit on an array element - so its rows live in a `macro_rules!` the two arms
instantiate (the `def_x_atom_refs!` shape, `src/xterm.rs`'s slice 108).

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the
**336-byte** empty object (exit 0).

**② The plain-X11 arm**: **10 errors / 9 warnings before, 10 / 9 after** - *unchanged*.  Every
name this slice reads resolves in that arm.

**③ The GTK arm** (the ten-cfg set): **26 / 20 before, 28 / 20 after** - the two additions are
`GDK_WINDOW_XID` rows (the mirror's `GTK_WIDGET_TO_X_WIN` placeholder, which this slice's
`FRAME_X_WINDOW!`/`FRAME_OUTER_WINDOW!` reads reach), a class slices 1-5 already count.  **No
warning moved**: the first measurement added one `unused doc comment` (a doc comment cannot sit on
a `macro_rules!` or on the `DEFUN!` invocation), and both notes are block comments now, which is
why the count is back at the baseline's 20.

**④ `extract-abi.py --check`**: `missing function` **33 -> 29** (all four definitions leave the
list) and `missing subr` **53 -> 52** (the `x-wm-set-size-hint` DEFUN - this file's first subr;
its *registration* is still `syms_of_xfns`'s, which is in the remainder).  Drift reads
`C 10662 vs Rust 6727` (**36.9%**, was 38.6%).  The function-token rows stay at 20.

**⑤ No mirror change**, so P-87's legs are not this slice's: `check-mirror-dupes` still reads
**16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**.

### Deviations (this slice)

1. **`unwind_create_frame`'s `FRAME_LIVE_P`** is the field test `(*f).terminal.is_null ()` (the
   mirror carries no such macro), and `XChangeProperty`'s `(unsigned char *) &xpid` is the port's
   `&raw mut xpid as *mut c_uchar` with `xpid` made `mut` for the address.
2. **C's `0xffffffffu` suffix** is not Rust's: the comparison is written
   `pid as c_uint <= 0xffff_ffff`.
3. **The `names[]` table** is instantiated by a macro (the pointer casts `.as_ptr () as *mut c_char`
   are the array's element type, and the two arms take the seven- and eight-row forms), and the
   `default_font_names!`/`DEFUN!` notes are block comments.

### Owed (this slice)

- `syms_of_xfns` (the file's subr registrations, `--check`'s `missing defsubr` class), the Xt arm
  of `x_window` (C 4172-4428) and `Fx_create_frame` (C 4918-5582, the next slice's first item).
- The 29 missing functions, 1 macro, 4 objects, 52 subrs and 45 defsubrs `--check` still reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no frame was unwound, no font was opened, no
size hint was sent and no WM property was written.  The two arms' counts are text compile readings.


## Slice 9 (2026-10-08, status 3.912 - the integrator sets the round number): C 4172-4428

**The slice is C 4172-4428**, the `USE_X_TOOLKIT` arm of `x_window` - the body slice 6 recorded as
this table's next-item row.  It is C 4176's `static void` and C 4177's name, C 4179-4186's locals,
C 4188's `block_input`, C 4195's `xlispstrdup`, C 4197-4207 the shell widget's `XtSetArg` block and
`XtAppCreateShell`, C 4209/4212-4240 the pane and frame widgets, C 4245-4322 the geometry block
(C 4255-4263 C's `#if false` region, C 4283-4299 the `window_prompting` flags), C 4324-4348
`XtManageChild`/`XtRealizeWidget`/`XSetClassHint`/`XSetWMHints`/`hack_wm_protocols`, C 4360-4392 the
widget sizes and the `explicit_name` block, C 4395-4411 the `PropMotifWmHints` branch, C 4414-4416
the cursor pair, C 4418's `unblock_input`, C 4422's `lw_set_main_areas` and C 4424-4427's XInput2
pair.  The port's slice is **407 Rust lines** (7,133 - 6,726, `grep -c ''`) against C's 257.

**What it reads.**  This is the only arm of any unit here that builds widgets through libXt, so the
slice is mostly *declarations*: `Boolean`/`XtPointer`/`WidgetClass`/`Dimension` (X11/Intrinsic.h and
X.h - the `Widget` row's twins, at 1386-1392), a use-site `WidgetRec`/`CorePart` pair for the one
record member C 4251-4252 reads, `xlispstrdup` (lisp.h 5553), `validate_x_resource_name` (frame.c
6180; no header declares it), `USPosition` (Xutil.h 102) and `PropModeReplace` (read from
`x11_constants`, `include/xlib.rs` 164).  Three names are **not** declared here: the six `src/widget.h`
resource names (`XtNshowGrip`, `XtNallowResize`, `XtNresizeToPreferred`, `XtNemacsFrame`,
`XtNinternalBorderWidth`, `XtNgeometry`) are read from that header's mirror, which is what
`check-mirror-dupes.py` requires, and `emacsFrameClass` is read from `include/widget.rs` 165-177's
`widget_decls` glob - a *definition* in the including unit shadows a glob import, so the compiler
stayed silent while the gate read the duplicate.  Three guards were widened because C's other arm
reads the same header names: the eleven X11 constants (`USPosition` added), `XClassHint` (Xutil.h
189-192) and `XSetClassHint` (Xutil.h 642, now its own block - the plain-X block's other three entry
points name plain-X-only records).

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0), both sides.

**② The plain-X11 arm**: **10 errors / 9 warnings before, 9 / 9 after**.  One row retires -
`validate_x_resource_name`'s `E0425`, since both arms call it now - and the three `E0381` rows the
missing callee had been *suppressing* surface: `class_hints` (C 4482), `attributes` (C 4483) and
`protocols` (C 4569) are C locals whose fields the branch writes, so the port gives them the file's
zero initialiser (PORTING.md 1.5, the rule slice 1's `outer_trans_cookie` note already carries) and
they close.  Net: 10 -> 9 errors, warnings unchanged.

**③ The GTK arm** (the ten-cfg set): **28 / 20 before, 28 / 20 after**, the same error *set* shifted
by this slice's five lines.  **No warning moved in any arm**: the three arms' warning kinds are
identical to their baselines (12 for the Xt arm, 9 / 20 for the other two), and none of them is in
this slice's own text.

**④ The Xt arm's first reading** (`have_x_windows` + `use_x_toolkit` + `use_toolkit_scroll_bars`):
**21 errors / 12 warnings on the baseline tree -> 10 / 12**.  The eleven that retire are nine
`Boolean` rows in `../lwlib/include/lwlib.rs`'s nine `lw_*` prototypes, one `WidgetClass` row in
`include/widget.rs`'s `emacsFrameClass` and this file's own `XClassHint` `E0425`; the ten that
remain are the file's pre-existing readings - `gray_bits`/`gray_width`/`gray_height` (and the
`XCreatePixmapFromBitmapData` `E0308` they cascade into) from `src/bitmaps/gray.xbm`, the
`x_display_info_for_name` `E0425` at C 3972's forward declaration, the three `E0609` member rows
(`wait_for_wm`, `alpha_identical_p`, `update_mode_line`), the `verror` `c_variadic` row and the
`HAVE_X_WINDOWS` static assert.

**⑤ `extract-abi.py --check`**: **156 rows -> 153**, and the delta is entirely the `x_window`
comparison: the three giant `function x_window does not mention <~60 tokens>` rows (one per C body,
since C defines the function once per arm) shrink to three one-token rows
(`frame_menubar_height`), and their three `family null: the C body uses it 9 time(s), the port 3`
rows retire.  Every other class is unchanged: `missing function` **29**, `missing subr` **52**,
`missing defsubr` **45**, `missing file-scope object` **4**, `missing macro` **1**.  Drift reads
`C 10662 vs Rust 7133` (**33.1%**, was 36.9%).  `check-mirror-dupes.py` is back at **16 duplicate
declaration(s), 16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at **18 declaration
sites (0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **C 4265's `FRAME_MENUBAR_HEIGHT (f) = menubar_size;`** is spelled through the member C's macro
   names, `(*((*f).output_data.x as *mut x_output)).menubar_height = menubar_size;`: the mirror
   carries only xterm.h 1463-1464's `#else` variant (`include/xterm.rs` 1177-1181, `((void) f, 0)`),
   which is not an lvalue.  The P-72 rule 3 form is the `FRAME_LIVE_P` precedent at C 4797; the owed
   mirror row is below, and `--check` still reads the one-token row that this spelling leaves.
2. **C 4387/4389's `explicit_name`** is the file's own `explicit_name!` macro (the `x_set_name`
   sites' spelling at 3246-3248) rather than an open-coded `frame_flag`/`frame_set_flag` pair, so
   C's own name stays in the body (`include/frame.rs` records the flag as a comment only).
3. **`window_prompting` is `mut`**: C 4296 assigns it (`&= ~(XNegative | YNegative)`), and C's
   `bool explicit = f->explicit_name;` becomes a `let` with the type C's initialiser gives it.
4. **The 2,048-byte `al`/`gal` `Arg` arrays** keep the file's `std::array::from_fn (|_| std::mem::zeroed ())`
   shape, and `class_hints`/`attributes` are zero-initialised where C declares them without one.
5. **The `WidgetRec`/`CorePart` record carries only the two members C 4251-4252 reads, and its
   offsets are UNVERIFIED** (the `lwlib/lwlib-utils.rs` 89-108 shape); `core.height as c_int +
   core.border_width as c_int` writes C's usual arithmetic conversions out because both members are
   `Dimension`s (X.h's `unsigned short`).
6. **C 4283-4299's `window_prompting` tests and C 4342's `InputHint`** read the file's own constant
   rows, whose guards this slice widened to the whole `have_x_windows` family.

### Owed (this slice)

- `include/xlib.rs`, the `Widget` row's twins: `Boolean`, `XtPointer`, `WidgetClass`, `Dimension`
  (X11/Intrinsic.h, X.h) and `USPosition` (Xutil.h 102) stand at this unit's use sites.
- `include/lisp.rs`: `xlispstrdup` (lisp.h 5553); `src/frame.rs`: `validate_x_resource_name`
  (frame.c 6180 - *no* header declares it, which is why both `x_window` arms declare it here).
- `include/xterm.rs` 1177-1181: the missing `USE_X_TOOLKIT || USE_GTK` variant of
  `FRAME_MENUBAR_HEIGHT` (xterm.h 1461-1462).  Its other face is a *semantic* gap this slice found:
  the two reads at 4929/4976 get the non-toolkit body's `0` in the GTK arm.
- `src/bitmaps/gray.xbm`'s three constants (slice 7's row) and the three mirror bitfield rows
  (`wait_for_wm`, `alpha_identical_p`, `update_mode_line`) are the other residual arm readings.
- `syms_of_xfns` (the subr registrations), `Fx_create_frame` (C 4918-5582) and the rest of the
  remainder: the 29 missing functions, 1 macro, 4 objects, 52 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no shell/pane/frame widget was created, no
geometry was negotiated, no WM protocol was installed and no Motif hint was written.  The three
arms' counts are text compile readings.


## Slice 10 (2026-10-08, status 3.913 - the integrator sets the round number): C 4918-5582

**The slice is C 4918-5582**: `Fx_create_frame`, the file's largest function - the item slice 8
recorded as the next one - and one body of 665 C lines that this round lands whole: C 4918-4927 the
`DEFUN` head and its doc string (kept verbatim), C 4930-4943's locals (C 4941-4943 the `USE_GTK`
`GdkWindow *gwin`), C 4945-5130 the display / name / parent / colour setup, C 5131-5146 the
font-driver registrations, C 5151-5393 the parameter block down to `x_icon`, `x_make_gc` and the
frame's promotion to official, and C 5395-5579 the menu bar, the size hints, the two
`adjust_frame_size`s, the visibility block, the machine/pid properties, the WM leader property,
the XSync block and the tail that hands the unused parameters to `fset_param_alist` and returns
`unbind_to (count, frame)`.  The port's slice is **849 Rust lines** (7,982 - 7,133, `grep -c ''`)
against C's 665; md5 `88e3afa84a6334163895ce985db2897a` (the slice 9 tree) ->
172efa6331f5678600ff70c6480ec80a.

**What it reads.**  Three families of use-site work.  ⑴ **Declarations no mirror carries**: the five
font-driver objects (`ftcrfont_driver`, `ftcrhbfont_driver`, `xftfont_driver`, `xfthbfont_driver`,
`xfont_driver`), `register_font_driver` (font.h 896), `init_frame_faces`, `initialize_frame_menubar`,
`x_embed_frame`, `delete_frame`, `gui_figure_window_size`, `x_iconify_frame`, `x_make_frame_visible`,
`x_cr_update_surface_desired_size`, `x_sync_init_fences`, `XIGetSelectedEvents`, `XSyncIntToValue`,
`XSyncCreateCounter` and `sysdep.h`'s two XSync types stand in one block (P-72 rule 4; `x_icon`,
`x_make_gc`, `x_icon_verify`, `x_wm_set_size_hint`, `set_machine_and_pid_properties` and the
`x_window` pair are this file's own already-landed definitions).  `x_make_frame_visible` and
`gui_display_get_resource` are declared earlier in this unit, so they are *not* repeated - the first
measurement's two `E0428` rows are what that rule costs.  ⑵ **The frame flags the mirror records as
`// C:` lines**: `no_split!` (C 5311), `can_set_window_size!` (C 5414) and `was_invisible!`
(C 5460-5478) are writers over `bits1`/`bits2` this unit defines, exactly as `src/androidfns.rs`
186-250 defines its own (`set_frame_flag1!`/`set_frame_flag2!`), and `FRAME_OVERRIDE_REDIRECT!`
(1306-1312, on the file's own side) is expanded for the first time - `include/frame.rs` 624-627
emits the unit/shift pair it names only under `have_window_system`, so the two arms without that
feature read the two constants from a use-site pair here.  ⑶ **The per-arm arguments**: C 5184-5188
and C 5213-5217 choose a call argument by arm, which no attribute can gate, so each pair is a
`macro_rules!` the two arms instantiate (`internal_border_default!`, `vertical_scroll_bars_default!`
- the slice 8 `default_font_names!` shape).

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0), both sides.

**② The plain-X11 arm**: **9 errors / 9 warnings before, 14 / 9 after**.  All five additions are
`E0609 no field explicit_parent on type x_output` - `x_output`'s bitfield that `include/xterm.rs`
922 records as a comment (P-80's member-dispatch note at the file's own C 726-730 site) - and they
are this slice's three sites (C 5108, C 5113, C 5446) plus the two expansions of the mirror's
`FRAME_X_EMBEDDED_P!` (C 5164, C 5323).

**③ The GTK arm** (the ten-cfg set): **28 / 20 before, 34 / 20 after** - four of the added rows are
the same `explicit_parent` family and two are `GDK_WINDOW_XID` rows of the class slices 1-5 count
(the `FRAME_X_WINDOW!`/`FRAME_OUTER_WINDOW!` reads this body adds).  **No warning moved in any
arm**: 9 / 20 / 12 before and after, and the one the first measurement added (a doc comment on the
`DEFUN!` invocation) is a block comment now.

**④ The Xt arm**: **10 / 12 before, 15 / 12 after** - the same five `explicit_parent` rows.

**⑤ `extract-abi.py --check`**: **153 rows before and after**, but the set moves: `missing subr`
**52 -> 51** (the `x-create-frame` `DEFUN` retires) and one token row appears, `subr Fx_create_frame
does not mention frame_background_pixel, frame_foreground_pixel, frame_output_data, frame_x_output,
have_clock_gettime, qleft, qright` - three classes: the two colour macros and the two output-data
macros are written as the members the mirror names, `have_clock_gettime` is a `#[cfg]` (the reader
blanks attributes, P-78), and `Qleft`/`Qright` live inside the two per-arm macros rather than in the
body.  Drift reads `C 10662 vs Rust 7982` (**25.1%**, was 33.1%).  `check-mirror-dupes.py` stays at
**16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at
**18 declaration sites (0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **`FRAME_FOREGROUND_PIXEL (f) = -1` and its five siblings at C 5082-5083 are `struct frame`'s
   members** (`frame.rs` 1345/1347 read them), so the writes name `(*f).foreground_pixel` /
   `(*f).background_pixel`; the six `output_data.x` slots C 5084-5087 names are the mirror's fields
   and are written as such.
2. **`f->was_invisible`, `f->no_split` and `f->can_set_window_size`** are `bool_bf`s recorded as
   comments, so C's assignments are the three use-site macros above (`explicit_name!`'s shape).
3. **`KVAR (kb, Vdefault_minibuffer_frame)`** is keyboard.h 38's token-pasting macro; the port
   writes keyboard.h 160's member `(*kb).Vdefault_minibuffer_frame_` (the `src/androidfns.rs`
   1550-1554 shape).
4. **`FRAME_FONT (f)`'s C truth test** is `FRAME_FONT! (f).is_null ()` (the mirror's macro yields
   the font pointer), and `GTK_CONTAINER (...)`/`KVAR (...)` are the port's `!`-suffixed macros.
5. **`record_unwind_protect (do_unwind_create_frame, frame)`** made slice 8's `unsafe fn` an
   `unsafe extern "C" fn`: its address crosses into C's `record_unwind_protect`, so the port gives
   it the C ABI (the slice 8 reading of `E0308` at that call is what this fixes).
6. **C 5184-5188/5213-5217's per-arm arguments** are the two macros above, and C 5334-5339's
   `XIGetSelectedEvents` writes the mirror's `xi_masks`/`num_xi_masks` through a raw pointer (the
   port's shape for an out-parameter).

### Owed (this slice)

- The `explicit_parent` bitfield (`include/xterm.rs` 922, a comment): every X arm that touches it
  reads `E0609`, and this slice makes three of those sites its own.
- The mirror rows for `have_window_system`'s guard on `FRAME_OVERRIDE_REDIRECT_FLAG_UNIT/SHIFT`
  (`include/frame.rs` 624-627) and for the frame flags' writers (`no_split`, `can_set_window_size`,
  `was_invisible`).
- `include/frame.rs`'s missing rows for `make_frame`, `make_minibuffer_frame`,
  `make_frame_without_minibuffer` and `frame_set_id_from_params`; `include/lisp.rs`'s for
  `xlispstrdup` (slice 9) and the XSync/driver entry points this slice declares.
- `have_xrandr`, `have_xinerama` and the three XSync names stay unregistered/owed as before;
  `harfbuzz` and `freetype` are the registered spellings of C's `HAVE_HARFBUZZ`/`HAVE_FREETYPE`.
- `syms_of_xfns` (the subr registrations) and the rest of the remainder: the 29 missing functions,
  1 macro, 4 objects, 51 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no frame object was allocated, no display was
consulted, no font driver was registered, no X window or widget was created, no size hint was sent
and no WM property was written.  The three arms' counts are text compile readings.


## Slice 11 (2026-10-08, status 3.914 - the integrator sets the round number): C 5583-5758

**The slice is C 5583-5758**, the first nine subrs of the remainder - `xw-color-defined-p`
(C 5583-5597), `xw-color-values` (C 5599-5613), `xw-display-color-p` (C 5615-5635),
`x-display-grayscale-p` (C 5637-5664), `x-display-pixel-width` (C 5666-5682),
`x-display-pixel-height` (C 5684-5700), `x-display-planes` (C 5702-5714),
`x-display-color-cells` (C 5716-5741) and `x-server-max-request-size` (C 5743-5758).  Nine small
`DEFUN`s whose doc strings stay verbatim: the port's slice is **186 Rust lines** (8,168 - 7,982,
`grep -c ''`) against C's 176; md5 `172efa6331f5678600ff70c6480ec80a` (the slice 10 tree) ->
`29ea2288899f1925b901888b49b6239e`.

**What it reads.**  Two things only.  ⑴ `x_display_pixel_width`/`x_display_pixel_height` are
xterm.c 18491/18482's definitions, so the two prototypes stand in a use-site block (P-72 rule 4;
rows owed to `include/xterm.rs`).  ⑵ The two `switch` bodies (C 5624-5634, C 5651-5663) are the
port's `match` arms plus `_` (PORTING.md 2.2), whose `case`/`default`/`switch` tokens cannot appear
in the port text - the two rows `docs/ports/macro-body-notes.tsv` now carries.  `list3i`'s three
arguments write C's conversions out (`foo.red as EMACS_INT` …), the two colour locals are C's
`XColor` with the file's zeroed initialiser (the C 695 `cdef` shape: the callee fills them through
`Emacs_Color *`, which dispextern.h 38's alias makes the same type in C and the mirror's record is
not), and the five `\(` doc strings are decoded per `lib-src/make-docfile.c`'s rule
(PORTING.md's P-38/P-24 note: `\X` reads as X, so the port's literal carries `(`, not `\(`).

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0), both sides.

**② The three arms are byte-for-byte the baseline's readings**: plain-X **14 / 9**, GTK **34 / 20**
and Xt **15 / 12**, before and after - the nine subrs add no name, no warning and no diagnostic in
any arm.  The first measurement did add nine `unused doc comment` warnings (a doc comment on a
`DEFUN!` invocation) and two `E0381`/one `E0425` rows; all three are closed (block comments, the
zeroed locals, and the `x11_constants` import the C 5727-5728 comparisons need).

**③ `extract-abi.py --check`**: **153 rows -> 144** - exactly the nine `missing subr` rows retiring,
with nothing added - and the drift reads `C 10662 vs Rust 8168` (**23.4%**, was 25.1%).
`check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**
and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed literals), 0
failures**.

### Deviations (this slice)

1. **`Emacs_Color *` at the two `x_defined_color` calls** is C's `XColor` through dispextern.h 38's
   alias; the mirror carries the `#else` arm's record, so the local is C's `XColor` (zeroed, the C
   695 shape) and the conversion happens at the call site (`&raw mut foo as *mut Emacs_Color`).
2. **C's two `switch`es** are `match` arms with the case order kept and `_` for `default`
   (PORTING.md 2.2); their `case`/`default`/`switch` tokens are recorded, not spelled.
3. **C's `\(` doc-string escape** is decoded (`(On MS Windows…`), and the C 5674/5692 `"` pair is a
   plain `"` in the port's literal - the `make-docfile` value both sides are compared as.
4. **`make_fixnum`'s argument conversions** are written out (`… as EMACS_INT`), as the file's other
   call sites do.

### Owed (this slice)

- `include/xterm.rs`'s two rows for `x_display_pixel_width`/`x_display_pixel_height` (xterm.h
  2011/2004) and the `Emacs_Color = XColor` alias for `have_x_windows` (the row slices 1 and 11 both
  name).
- `syms_of_xfns` (the nine registrations this slice's subrs are still owed), and the rest of the
  remainder: the 29 missing functions, 1 macro, 4 objects, 42 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no display was consulted, no colour was
defined or allocated and no server attribute was read; the three arms' counts are text compile
readings.
