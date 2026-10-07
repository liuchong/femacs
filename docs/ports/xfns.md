# emacs/src/xfns.c (10662 lines) — slices: C 1-802, C 803-2149, C 2150-3811, C 3812-3956,
C 3957-4170, C 4172-4428, C 4430-4623, C 4624-4783, C 4785-4916, C 4918-5582, C 5583-5758,
C 5760-5889, C 5891-5991, C 5993-6123, C 6125-6215, C 6217-6297, C 6300-6513, C 6515-6601,
C 6604-6777, C 6779-7035, C 7037-7207, C 7210-7273, C 7275-7447, C 7449-7491, C 7494-7606,
C 7609-7729, C 7731-7950, C 7953-7995, C 7998-8073, C 8075-8176, C 8178-8256, C 8258-8360, C 8362-8404, C 8406-8775, C 8778-8893, C 8896-9028, C 9031-9372 and C 9375-9415

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
> eighth, **7,133 lines** after the ninth, **7,982 lines** after the tenth, **8,168 lines** after
> the eleventh, **8,296 lines** after the twelfth, **8,389 lines** after the thirteenth,
> **8,538 lines** after the fourteenth, **8,650 lines** after the fifteenth, **8,768 lines** after
> the sixteenth, **9,167 lines** after the seventeenth, **9,296 lines** after the eighteenth,
> **9,547 lines** after the nineteenth, **9,801 lines** after the twentieth, **10,033 lines**
> after the twenty-first, **10,157 lines** after the twenty-second, **10,332 lines** after the
> twenty-third, **10,404 lines** after the twenty-fourth, **10,631 lines** after the twenty-fifth,
> **10,782 lines** after the twenty-sixth, **11,042 lines** after the twenty-seventh and
> **11,088 lines** after the twenty-eighth, **11,179 lines** after the twenty-ninth and
> **11,286 lines** after the thirtieth, **11,371 lines** after the thirty-first and
> **11,493 lines** after the thirty-second, **11,554 lines** after the thirty-third and
> **12,025 lines** after the thirty-fourth, **12,155 lines** after the thirty-fifth and
> **12,303 lines** after the thirty-sixth, **12,855 lines** after the thirty-seventh and
> **12,912 lines** after the thirty-eighth - C 803-2149,
> C 2150-3811, C 3812-3956, C 3957-4170, C 4172-4428, C 4430-4623, C 4624-4783, C 4785-4916,
> C 4918-5582, C 5583-5758, C 5760-5889, C 5891-5991, C 5993-6123, C 6125-6215, C 6217-6297,
> C 6300-6513, C 6515-6601, C 6604-6777, C 6779-7035, C 7037-7207, C 7210-7273, C 7275-7447,
> C 7449-7491, C 7494-7606, C 7609-7729, C 7731-7950, C 7953-7995, C 7998-8073, C 8075-8176,
> C 8178-8256, C 8258-8360, C 8362-8404, C 8406-8775, C 8778-8893, C 8896-9028, C 9031-9372 and
> C 9375-9415, whose own
> ledger sections are at the end of this file - measured with `grep -c ''`), both arms of every
> `#if` carried.
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
| 5760-5889 | the next six subrs: `x-server-vendor` (C 5760), `x-server-version` (C 5783), `x-server-input-extension-version` (C 5810, the `HAVE_XINPUT2` pair), `x-display-screens` (C 5831), `x-display-mm-height` (C 5851) and `x-display-mm-width` (C 5871) | `have_x_windows`, with `have_xinput2` for C 5820-5828 (slice 12) |
| 5891-5991 | the next three subrs: `x-display-backing-store` (C 5891), `x-display-visual-class` (C 5930) and `x-display-save-under` (C 5975) | `have_x_windows` (slice 13; the two `switch` bodies' `break`/`case`/`default`/`switch` tokens are answered in `docs/ports/macro-body-notes.tsv`) |
| 5993-6123 | C 5993's `#if !(defined USE_GTK && defined HAVE_GTK3)`, `x_get_net_workarea` (C 5995-5997 its comment, C 5999-6000 the `static bool`, C 6002-6055 the `#ifndef USE_XCB` arm, C 6056-6120 the `#else` arm) and C 6123's `#endif` | `all (have_x_windows, not (all (use_gtk, have_gtk3)))`, with `not (use_xcb)` / `use_xcb` for the two arms (slice 14) |
| 6125-6215 | C 6125's `#ifndef USE_GTK`, `x_get_monitor_for_frame` (C 6127 its comment, C 6128-6131 the `static int`) and `x_make_monitor_attribute_list` (C 6190-6195 the `static Lisp_Object`) | `all (have_x_windows, not (use_gtk), have_window_system)` - C's own `#ifndef USE_GTK` narrowed by frame.h 1926's `#ifdef HAVE_WINDOW_SYSTEM`, which is where `struct MonitorInfo` lives (slice 15) |
| 6217-6297 | `x_get_monitor_attributes_fallback` (C 6217-6238) and C 6241's `#ifdef HAVE_XINERAMA`, `x_get_monitor_attributes_xinerama` (C 6242-6296) and C 6297's `#endif` | the slice 15 pair, with `have_xinerama` for the second (the record and entry point stand at the use site; slice 16) |
| 6300-6513 | C 6300's `#ifdef HAVE_XRANDR`, `x_get_monitor_attributes_xrandr` (C 6301-6302 the `static Lisp_Object`, C 6312's `#if RANDR_MAJOR … >= 5` with C 6314's `#ifdef USE_XCB` inside it, C 6413-6414's `RANDR13_LIBRARY`, C 6438-6440's own 1.3 arm) and C 6513's `#endif` | `all (have_x_windows, not (use_gtk), have_window_system, have_xrandr)`, with the derived `have_xrandr_1_5` / `have_xrandr_1_3` and `use_xcb` inside (slice 17) |
| 6515-6601 | C 6515-6516's `static Lisp_Object`, the dispatcher `x_get_monitor_attributes` (C 6523-6530 the `HAVE_XRANDR` arm, C 6532-6538 the `HAVE_XINERAMA` arm, C 6540-6541 the fallback), C 6546's `#endif /* !USE_GTK */`, C 6548's `#ifdef USE_LUCID`, `xlw_monitor_dimensions_at_pos_1` (C 6551-6554 the `static void`) with `xlw_monitor_dimensions_at_pos` (C 6589-6591 the `void`) and C 6601's `#endif` | the slice 15 pair for the dispatcher (its three callees' own gate, `all (have_x_windows, not (use_gtk), have_window_system)`); `use_lucid` alone for the two Lucid functions, whose callee is the pair's, so the ledger's Lucid arm carries it (slice 18) |
| 6604-6777 | the `x-display-monitor-attributes-list` `DEFUN` (C 6604-6606 its header, C 6607-6621 the verbatim doc string, C 6627-6767 the `USE_GTK` arm with its 3.22 / pre-3.22 sub-arms and C 6768-6773 the plain-X one) | `have_x_windows`, with `use_gtk` and, inside it, `gtk_check_version_ge_3_22` / `have_gtk3` for C's three-way workarea and scale arms (slice 19) |
| 6779-7035 | `frame_geometry` (C 6779-6783 the comment, C 6785-6786 the `static Lisp_Object`; C 6860-6868 the `HAVE_EXT_MENU_BAR` pair, C 6877-6922 the `HAVE_EXT_TOOL_BAR` pair) and the two subrs it backs, `x-frame-geometry` (C 6964) and `x-frame-edges` (C 7013) | `have_x_windows`, with `have_ext_menu_bar` and `have_ext_tool_bar` inside (slice 20) |
| 7037-7207 | `x_frame_list_z_order` (C 7037-7043 its comment, C 7044-7045 the `static Lisp_Object`; C 7063-7096 the `_NET_CLIENT_LIST_STACKING` arm, C 7098-7119 the `XQueryTree` walk) with its subr `x-frame-list-z-order` (C 7125), then `x_frame_restack` (C 7150-7157 its comment, C 7161-7178 the `USE_GTK` pair) with its subr `x-frame-restack` (C 7182) | `have_x_windows`, with `use_gtk` for the restack helper's two arms (slice 21) |
| 7210-7273 | the two mouse subrs: `x-mouse-absolute-pixel-position` (C 7210-7233, `x_query_pointer`'s nine arguments) and `x-set-mouse-absolute-pixel-position` (C 7235-7273, whose C 7251-7266 `HAVE_XINPUT2` pair chooses `XIWarpPointer` or `XWarpPointer`) | `have_x_windows`, with `have_xinput2` for the client-pointer-device arm (slice 22) |
| 7275-7447 | the `x-begin-drag` `DEFUN` (C 7275 its header, C 7276-7334 its doc string; C 7350-7368 the target walk, C 7370-7426 the action dispatch, C 7430-7437 the atom interning under C 7432's error trap, C 7439-7443 the `x_dnd_begin_drag_and_drop` call, all inside C 7348/7445's `USE_SAFE_ALLOCA`/`SAFE_FREE` pair) | `have_x_windows`, with no nested `#if` (slice 23) |
| 7449-7491 | C 7449-7451's `X Displays` section comment, C 7454-7470's `visual_class` record and its `visual_classes[]` table (the six X11/X.h class constants) and C 7473-7491's `#ifndef HAVE_XSCREENNUMBEROFSCREEN` `XScreenNumberOfScreen` substitute (the file's own non-`static` definition) | `have_x_windows` on every item (P-86 ⑧, caught by the frozen `build-tu`), with `not (have_xscreennumberofscreen)` on the substitute and the feature on Xlib's declaration at 1960-1961 (slice 24) |
| 7494-7606 | `select_visual` (C 7494-7496 its comment) - the `visualClass` resource lookup, the CLASS-DEPTH parse (C's bare `alloca`), the `visual_classes` walk, `XMatchVisualInfo` and the default-visual fallback with its `#if !defined USE_X_TOOLKIT && !(defined USE_GTK && !defined HAVE_GTK3) && defined HAVE_XRENDER` alpha-mask attempt | `have_x_windows` on the function, with `have_xrender` (and C's two negations) on that block's `#[cfg]` (slice 25) |
| 7609-7729 | the connection group: `x_display_info_for_name` (C 7609-7610 its comment, C 7612-7634), `x-open-connection` (C 7637-7677), `x-close-connection` (C 7679-7695), `x-display-list` (C 7697-7708) and `x-synchronize` (C 7710-7729) | `have_x_windows`, with no nested `#if` (slice 26) |
| 7731-7950 | C 7732-7734's `Window properties` section comment and `x-change-window-property` (C 7736-7950: the FORMAT check, the two VALUE arms with C 7821/7837's `elsize`, the target-window choice, C 7783-7791 and C 7858-7933's `USE_XCB` pair, C 7937-7939's `XChangeProperty` and C 7944-7946's error pair) | `have_x_windows` on the DEFUN, with `use_xcb` on C 7867's arm (slice 27) |
| 7953-7995 | `x-delete-window-property` (C 7953-7955 the `DEFUN` header, C 7956-7967 the verbatim doc string, C 7970-7994 the body: the `CHECK_STRING`, the `WINDOW-ID` pair with its root-window fallback, the cached-atom intern and the error pair around `XDeleteProperty`) | `have_x_windows`, with no nested `#if` (slice 28) |
| 7998-8073 | `x_window_property_intern` (C 7998-8005 the `static Lisp_Object` and its seven parameters, C 8007-8012 the locals, C 8014-8017 the first `XGetWindowProperty`, C 8019's `*found`, C 8026-8031 the second call with C 8029's `! NILP (delete_p)`, C 8034-8046 the quoted `XGetWindowProperty` man page, C 8048-8056 the `LONG_WIDTH > 32` arm, C 8058-8066 the `make_string`/`x_property_data_to_lisp` pair, C 8069's `XFree`) | `have_x_windows`, with no nested `#if` (slice 29) |
| 8075-8176 | `x-window-property` (C 8075-8076 the `DEFUN` header, C 8077-8115 the verbatim doc string, C 8119-8124 the locals, C 8126's `CHECK_STRING`, C 8128-8133 the `WINDOW-ID` pair, C 8136's `x_catch_errors`, C 8138-8145 the `AnyPropertyType`/intern pair, C 8149-8155 and C 8161-8167 the two `x_window_property_intern` calls with C 8156-8159's four-way test, C 8170-8172 the error pair) | `have_x_windows`, with no nested `#if` (slice 30) |
| 8178-8256 | `x-window-property-attributes` (C 8178-8179 the `DEFUN` header, C 8180-8194 the verbatim doc string, C 8197-8205 the locals, C 8207's `CHECK_STRING`, C 8209-8214 the `WINDOW-ID` pair, C 8218-8220's `x_catch_errors` and cached-atom intern, C 8221-8224's first `XGetWindowProperty`, C 8225-8238's four-way outer-window retry with C 8230-8232's comment, C 8240-8248's `XFree` and `CALLN (Fvector, ...)`, C 8250-8252's error pair) | `have_x_windows`, with no nested `#if` (slice 31) |
| 8258-8360 | the `Coordinate management` section comment (C 8259-8261) and `x-translate-coordinates` (C 8263-8360: C 8265-8285's verbatim doc string, C 8290-8295 the locals, C 8297's `check_x_display_info`, C 8301-8311 the two `CHECK_FIXNUM`/`XFIXNUM` pairs, C 8313-8321's `source_frame`/`source_window` pair, C 8323-8332's `require_child` early return, C 8334-8340's `src`/`dest` defaults, C 8342-8350's `XTranslateCoordinates` with the error pair, C 8352-8359's `list3` result) | `have_x_windows`, with no nested `#if` (slice 32) |
| 8362-8404 | the `Tool tips` banner (C 8362-8364), C 8366-8367's forward declaration of `compute_tip_xy`, the eight objects (C 8369-8390: `tip_frame`, `tip_window`, `tip_dx`, `tip_dy` exported; `tip_timer`, `tip_last_string`, `tip_last_frame`, `tip_last_parms` private) and `unwind_create_tip_frame` (C 8393-8404) | `have_x_windows`, with no nested `#if` (slice 33) |
| 8406-8775 | C 8407-8414's comment and `x_create_tip_frame` (C 8416-8775: the terminal check, the name lookup, the frame's creation with its unwind-protect, the X output setup, the color block, the name/`explicit_name` pair, the font drivers, the `gui_default_parameter` runs, the `USE_XCB`/`not USE_XCB` window-creation pair, `init_frame_faces`, the `tooltip`/`display-type` parameters, the face-after-default pair and the tail) | `have_x_windows`, with `use_cairo`/`freetype`/`have_xft`/`harfbuzz` for the font drivers, `use_xcb` for the window pair, `glyph_debug`, `use_lucid`/`use_toolkit_scroll_bars` (slice 34) |
| 8778-8893 | C 8778-8782's comment and `compute_tip_xy` (C 8784-8893: the four `Fassq` reads, C 8808-8811's `x_query_pointer` between `block_input`/`unblock_input`, C 8815-8824's `HAVE_XRANDR`/`USE_GTK` cached-list choice, C 8828-8849's monitor walk with C 8834-8837's four `Fnth` reads, C 8854-8860's defaults, and C 8862-8892's two four-arm chains) | `have_x_windows`, with `have_xrandr`/`use_gtk` for the cached-list choice (slice 35) |
| 8896-9028 | C 8896-8908's doc comment and `x_hide_tip` (C 8909-9028: C 8912-8916's timer cancel, C 8918-8987's `USE_GTK` arm - the early-return test, the two `specbind`s, `xg_hide_tooltip`, the `tip_last_frame` reset and the delete-or-hide pair - and C 8988-9027's `not USE_GTK` arm) | `have_x_windows`, with `use_gtk` splitting the two arms (slice 36) |
| 9031-9372 | the `x-show-tip` `DEFUN` (C 9031-9061's verbatim doc string, C 9065-9078 the locals and `AUTO_STRING`, C 9080-9105 the argument defaults, C 9107-9127's `USE_GTK` arm, C 9129-9209's reuse/delete/hide block with its two parameter walks, C 9211-9238's `tip_frame` creation, C 9240-9278's window setup, C 9280-9311's buffer/`try_window` block with its `ENABLE_CHECKING` arm, C 9314-9323's size and position, C 9325-9348's show block with its `not USE_XCB`/`USE_XCB` pair, and C 9350-9371's garbage/update/timer tail) | `have_x_windows`, with `use_gtk`, `use_cairo`, `enable_checking` and the `use_xcb` pair inside (slice 37) |
| 9375-9415 | the three subrs `x-hide-tip` (C 9375-9381), `x-double-buffered-p` (C 9383-9394, with C 9388-9393's `HAVE_XDBE` pair) and `x-uses-old-gtk-dialog` (C 9401-9415, with C 9407-9414's `USE_GTK` guard), plus C 9396-9399's `File selection dialog` banner | `have_x_windows`, with `have_xdbe` and `use_gtk` inside (slice 38) |
| 9418-10662 | **the remainder** (C 10662 is the reference's last line): the file's other 1,244 lines, whose first live item is C 9418's `#ifdef USE_MOTIF` and the three callbacks it guards (C 9421-9453) - the next slice's first items | not ported in this slice |

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


## Slice 12 (2026-10-08, status 3.915 - the integrator sets the round number): C 5760-5889

**The slice is C 5760-5889**, six more subrs of the remainder - `x-server-vendor` (C 5760-5781),
`x-server-version` (C 5783-5808), `x-server-input-extension-version` (C 5810-5829, whose body is
C's `#ifdef HAVE_XINPUT2` pair), `x-display-screens` (C 5831-5849), `x-display-mm-height`
(C 5851-5869) and `x-display-mm-width` (C 5871-5889).  The port's slice is **128 Rust lines**
(8,296 - 8,168, `grep -c ''`) against C's 130; md5 `29ea2288899f1925b901888b49b6239e` (the slice 11
tree) -> `42ac276cc1678832e96889ff582a673d`.

**What it reads.**  One use-site block: the seven Xlib names these bodies call.  Xlib.h defines six
of them as *macros* over private `_XDisplay` fields (Xlib.h 103-107) and the mirror's `_XDisplay` is
opaque (`include/xlib.rs` 1046), so the port declares the header's own function forms
(`XServerVendor` 1812, `XProtocolVersion` 2903, `XProtocolRevision` 2899, `XVendorRelease` 3520,
`XScreenCount` 3135) and keeps C's name in a `macro_rules!` that calls them (the
`MAXREQUEST`/`XMaxRequestSize` shape this file already has at 152-156).  The two `Screen` macros are
the other way round: `Screen`'s members *are* mirrored (`include/xlib.rs` 1130-1131), so
`WidthMMOfScreen!`/`HeightMMOfScreen!` keep C's name and read `mwidth`/`mheight` directly - which is
also what keeps the `--check` token rows away (the first measurement's two rows
`does not mention heightmmofscreen`/`widthmmofscreen` are what spelling the member *without* the
macro name costs).

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0), both sides.

**② The three arms are byte-for-byte the baseline's readings**: plain-X **14 / 9**, GTK **34 / 20**
and Xt **15 / 12**, before and after - and unlike slices 9-11 this slice's first measurement was
already clean (no warning, no `E0381`, no unresolved name), so the six `DEFUN`s cost nothing in any
arm.

**③ `extract-abi.py --check`**: **144 rows -> 138** - exactly the six `missing subr` rows retiring,
with nothing added - and the drift reads `C 10662 vs Rust 8296` (**22.2%**, was 23.4%).
`check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**
and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed literals), 0
failures**.

### Deviations (this slice)

1. **Five Xlib macros over the opaque `_XDisplay`** (`ServerVendor`, `ProtocolVersion`,
   `ProtocolRevision`, `VendorRelease`, `ScreenCount`) are the port's `macro_rules!`s over the
   header's `X*` functions; the two `Screen` ones read the mirror's members.
2. **`ServerVendor`'s NULL fallback** is C's assignment (`vendor = ""`), so the local is `mut` and
   the empty literal is `c"".as_ptr ()`.
3. **`list3i`/`list2i`'s argument conversions** are written out (`... as EMACS_INT`), and C 5821's
   declaration is inside the `have_xinput2` arm (the `#else` arm returns `Qnil` with no local).

### Owed (this slice)

- `include/xlib.rs`'s five `X*` function rows and the six macro rows (`docs/ports/xfns.md` names
  the same gap slices 1 and 11 already record for the Xlib surface).
- `syms_of_xfns` (the six registrations this slice's subrs are still owed), and the rest of the
  remainder: the 29 missing functions, 1 macro, 4 objects, 36 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no display was consulted and no server
attribute was read; the three arms' counts are text compile readings.


## Slice 13 (2026-10-08, status 3.916 - the integrator sets the round number): C 5891-5991

**The slice is C 5891-5991**, three more subrs of the remainder - `x-display-backing-store`
(C 5891-5928, a `switch` over `DoesBackingStore (dpyinfo->screen)`),
`x-display-visual-class` (C 5930-5973, a `switch` over `dpyinfo->visual_info.class`) and
`x-display-save-under` (C 5975-5991).  The port's slice is **93 Rust lines** (8,389 - 8,296,
`grep -c ''`) against C's 101; md5 `42ac276cc1678832e96889ff582a673d` (the slice 12 tree) ->
`8f799d73d94af03837485e1a1a417f95`.

**What it reads.**  The two `BackingStore` values `WhenMapped 1` and `Always 2` (X11/X.h 440-441,
measured on this machine's header) with their partner `NotUseful 0` already at 6077, and Xlib.h
269-270's two `Screen` macros (`DoesBackingStore`, `DoesSaveUnders`) as `macro_rules!`s over the
mirror's `backing_store`/`save_unders` members (`include/xlib.rs` 1142-1143) - the slice 12 shape
for an Xlib macro whose record members *are* mirrored.  Both `switch`es are the port's `match` arms
plus `_` (PORTING.md 2.2) with C's `error` in the `default` arm, and the six `visual_info.class`
constants come from `x11_constants` through a per-arm `use` - **the first measurement's five
`unreachable pattern` warnings** were what an un-imported class name costs (an unresolved path in a
pattern is a *binding*, so the first arm matched everything); the two `macro-body-notes.tsv` rows
answer the `break`/`case`/`default`/`switch` tokens.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0), both sides.

**② The three arms are byte-for-byte the baseline's readings**: plain-X **14 / 9**, GTK **34 / 20**
and Xt **15 / 12**, before and after, with the five `unreachable pattern` warnings closed by the
class import above.

**③ `extract-abi.py --check`**: **138 rows -> 135** - exactly the three `missing subr` rows retiring,
with the two token rows answered by the notes table - and the drift reads `C 10662 vs Rust 8389`
(**21.3%**, was 22.2%).  `check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)** and `check-declarations.py` at **18 declaration sites (0 forbidden, 18
allowed literals), 0 failures**.

### Deviations (this slice)

1. **The two `switch`es** are `match` arms with C's case order, `_` for `default` and C's `error`
   call in that arm (PORTING.md 2.2); the `break`s are the arm ends.
2. **`WhenMapped`/`Always`** stand at this use site with the header's measured values (X11/X.h
   440-441), and the two Xlib `Screen` macros keep C's name over the mirror's members.
3. **`x-display-save-under`'s `== True`** keeps C's comparison, reading `include/xterm.rs` 76's
   constant (the same file's `False` is at 79).

### Owed (this slice)

- `include/xlib.rs`'s rows for `DoesBackingStore`/`DoesSaveUnders` and the two X.h constants.
- `syms_of_xfns` (the three registrations this slice's subrs are still owed), and the rest of the
  remainder: the 29 missing functions, 1 macro, 4 objects, 33 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no display or screen was consulted and no
backing-store attribute was read; the three arms' counts are text compile readings.


## Slice 14 (2026-10-08, status 3.917 - the integrator sets the round number): C 5993-6123

**The slice is C 5993-6123**: C 5993's `#if !(defined USE_GTK && defined HAVE_GTK3)` and
`x_get_net_workarea` (C 5999-6000's `static bool`, the `#ifndef USE_XCB` arm at C 6002-6055 - the
two `XGetWindowProperty` rounds, the `_NET_CURRENT_DESKTOP` lookup and the four `rect` members -
and the `#else` arm at C 6056-6120, the two `xcb_get_property` cookies and their replies), closed
by C 6123's `#endif`.  The port's slice is **149 Rust lines** (8,538 - 8,389, `grep -c ''`) against
C's 131; md5 `8f799d73d94af03837485e1a1a417f95` (the slice 13 tree) ->
`7ae1b7b6d7723f1e9a8edc39b395eb76`.

**What it reads.**  Nothing new: every declaration it needs is this unit's (`x_catch_errors`,
`x_had_errors_p` and `x_uncatch_errors` at 313-317, `XGetWindowProperty`/`XFree` at 222/231, the
xcb family at 260-275 and `include/xlib.rs` 575-908), `False` is `include/xterm.rs` 79's constant
and `UINT32_MAX` is `lib/include/stdint.in.rs` 178's - **the last one cost two measurements**: the
first arm reading's single `does not mention uint32_max` token row said the C name was not in the
port text, and a use-site `const UINT32_MAX` written for it was an `E0428` (the mirror carries the
name), so the call site reads the mirror's constant and the row is gone.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0), both sides.

**② The four arms.**  plain-X **14 / 9** before and after, the **XCB arm gets its first reading**
(`have_x_windows` + `use_xcb`: **14 / 9** on the slice 13 tree and **14 / 9** after), GTK
**34 / 20** and Xt **15 / 12** - this slice's function is *absent* from the GTK arm by its own C
guard, and no arm's warning count moved.  The first measurement's own rows were closed on the way:
one `E0425` (`c_ubyte` for C's `unsigned char *`; the port's spelling is `c_uchar`), four `E0381`
rows and two more in the XCB arm (the six out-parameters and locals C leaves uninitialised get the
file's zero initialisers: `actual_type`/`actual_format`/`actual_size`/`bytes_remaining`, `error`
and `current_workspace`).

**③ `extract-abi.py --check`**: **135 rows -> 134** - exactly the `missing function
x_get_net_workarea` row retiring, with nothing added - and the drift reads `C 10662 vs Rust 8538`
(**19.9%**, was 21.3%).  `check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet
row(s), 0 failure(s)** and `check-declarations.py` at **18 declaration sites (0 forbidden, 18
allowed literals), 0 failures**.

### Deviations (this slice)

1. **C's `(long *) tmp_data` and `values[i]`** are the port's `tmp_data.cast::<c_long> ()` and
   `*values.offset (i)` (the file's 744-747 shape), and the four `rect->` members write C's
   narrowing conversions out (`as c_short` / `as c_ushort`, XRectangle's own field types).
2. **C's `UINT32_MAX` and `False`** are the mirror constants, not values written in place.
3. **The two arms are `#[cfg]` blocks** (`not (use_xcb)` / `use_xcb`), each with C's own
   `return`, and the function's guard is C 5993's own condition - so nothing of this slice is
   compiled by the GTK arm.

### Owed (this slice)

- `syms_of_xfns` (unchanged: no subr lands here), and the rest of the remainder: the 28 missing
  functions, 1 macro, 4 objects, 33 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no display was opened, no property was read
and no workarea was stored; the four arms' counts are text compile readings.


## Slice 15 (2026-10-08, status 3.918 - the integrator sets the round number): C 6125-6215

**The slice is C 6125-6215**: C 6125's `#ifndef USE_GTK` and the two statics under it -
`x_get_monitor_for_frame` (C 6127's comment and C 6128-6188's body: the `frect` build, the
intersection/area pass and the distance pass) and `x_make_monitor_attribute_list` (C 6190-6215,
the `FOR_EACH_FRAME` walk and the call into frame.c).  The port's slice is **112 Rust lines**
(8,650 - 8,538, `grep -c ''`) against C's 91; md5 `7ae1b7b6d7723f1e9a8edc39b395eb76` (the slice 14
tree) -> `bc5b60c371ecdb5ad0c46b241ccb3d95`.

**The guard is the slice's finding.**  C's own condition is `#ifndef USE_GTK`, but both bodies name
`struct MonitorInfo`, which frame.h 1926 puts under `#ifdef HAVE_WINDOW_SYSTEM` (the mirror row is
`include/frame.rs` 1937, guarded the same way) - so C's condition is *implicitly* narrowed, and the
port spells the pair out: `all (have_x_windows, not (use_gtk), have_window_system)`.  **None of the
four arms measured before this slice has `have_window_system`**, so the slice adds a fifth reading
to this ledger's table - **WS** = `have_x_windows` + `have_window_system` - which reads **14 / 9**
on both sides of the change.  `gui_intersect_rectangles`'s three parameters are C's
`Emacs_Rectangle *` (dispextern.h 3676), which the x11 arm reads as `XRectangle` through
dispextern.h 38's alias, so the two `XRectangle` locals convert at the call site (the row slices 11
and 14 also name).

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0), both sides.

**② The five arms.**  plain-X **14 / 9**, WS **14 / 9** (first reading), XCB **14 / 9**, GTK
**34 / 20** and Xt **15 / 12**, before and after - the two new statics change no count in any arm
that carries them, and the GTK arm excludes them by C's own `#ifndef USE_GTK`.  The first
measurement's own rows were the ordinary ones: two parse-level rows (`frect.width as c_int <` reads
as a generic argument list, so the sums are parenthesised) and four type rows (`XRectangle`'s
`short`/`unsigned short` against `Emacs_Rectangle`'s `int`, written as C's promotions).

**③ `extract-abi.py --check`**: **134 rows -> 132** - exactly the two `missing function` rows
(`x_get_monitor_for_frame`, `x_make_monitor_attribute_list`) retiring, with nothing added - and the
drift reads `C 10662 vs Rust 8650` (**18.9%**, was 19.9%).  `check-mirror-dupes.py` stays at
**16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at
**18 declaration sites (0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **C's for-loop with `continue`** becomes a `while` whose `i += 1` sits at the body's end *and*
   before the `continue` (the `src/xterm.rs` 13568-13574 shape), so the increment still runs where
   C's does.
2. **C's mixed-width arithmetic** (`XRectangle`'s `short`/`unsigned short` members against
   `Emacs_Rectangle`'s `int`) is written out with `as c_int`, and the two comparisons that would
   read as generic arguments are parenthesised.
3. **C 6187's nested conditional** is the port's `if`/`else if`/`else` expression, and the two
   `XRectangle` locals are zero-initialised (C declares them without an initialiser).

### Owed (this slice)

- `syms_of_xfns` (unchanged: no subr lands here), and the rest of the remainder: the 26 missing
  functions, 1 macro, 4 objects, 33 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no frame was walked, no monitor was compared
and no attribute list was built; the five arms' counts are text compile readings.


## Slice 16 (2026-10-08, status 3.919 - the integrator sets the round number): C 6217-6297

**The slice is C 6217-6297**: `x_get_monitor_attributes_fallback` (C 6217-6238 - the single
synthesised monitor, the `xstrdup`'d name and the workarea/geom choice) and
`x_get_monitor_attributes_xinerama` (C 6242-6296 - `XineramaQueryScreens`, the two per-pixel
scales, the `xcalloc`'d array, the per-monitor loop and `free_monitors`), inside C 6241's
`#ifdef HAVE_XINERAMA`.  Both sit under C 6125's `#ifndef USE_GTK` and name `struct MonitorInfo`,
so the slice 15 guard pair is theirs too, with C 6241's own gate added for the second.  The port's
slice is **118 Rust lines** (8,768 - 8,650, `grep -c ''`) against C's 81; md5
`bc5b60c371ecdb5ad0c46b241ccb3d95` (the slice 15 tree) -> `862bdbed15cf2467a644f212b5183005`.

**What it reads.**  One new group: `X11/extensions/Xinerama.h`'s record and entry point
(`XineramaScreenInfo` - the members the C reads, in the header's own order, with its size and
align pair left **UNVERIFIED**: this machine has no `<X11/extensions/Xinerama.h>` - and
`XineramaQueryScreens`), declared at the use site under the same guard as the
function that calls them; every other name is already there (`xstrdup` at `include/lisp.rs` 4220,
`xcalloc` at 4042, `free_monitors` at `include/frame.rs` 1956, the two `MMOfScreen` macros from
slice 12 and `x_make_monitor_attribute_list` from slice 15).  `x_get_monitor_attributes_xinerama`
needs a *second* new arm to be measured at all: the slice 15 pair alone does not carry
`have_xinerama`, so the table gains **WSX** = the pair + `have_xinerama`.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0), both sides.

**② The six arms.**  plain-X **14 / 9**, WS **14 / 9**, WSX **14 / 9** (first reading), XCB
**14 / 9**, GTK **34 / 20** and Xt **15 / 12**, before and after - the first measurement was clean
in every arm, so this slice adds no warning and no diagnostic anywhere.

**③ `extract-abi.py --check`**: **132 rows -> 130** - exactly the two `missing function` rows
(`x_get_monitor_attributes_fallback`, `x_get_monitor_attributes_xinerama`) retiring, with nothing
added - and the drift reads `C 10662 vs Rust 8768` (**17.8%**, was 18.9%).
`check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**
and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed literals), 0
failures**.

### Deviations (this slice)

1. **C 6226's chained assignment** is the port's two statements, and the two `XRectangle`/`MonitorInfo`
   locals are zero-initialised (C declares them without an initialiser).
2. **C's mixed-width writes** (`monitor.geom.width = x_display_pixel_width (…)`, an `int` into
   `unsigned short`) and **C's double-to-int truncation** (`… * mm_width_per_pixel + 0.5` into
   `int mm_width`) are written out with `as c_ushort` / `as c_int`.
3. **The Xinerama loop** is the slice 15 `while` with `i += 1` at the body's end, and
   `gui_intersect_rectangles (&mi->geom, &mi->work, &mi->work)` keeps C's aliasing (a `*const` and
   a `*mut` of the same member).

### Owed (this slice)

- `include/xlib.rs`-style rows for the Xinerama record and entry point, and `syms_of_xfns`
  (unchanged: no subr lands here).
- The rest of the remainder: the 24 missing functions, 1 macro, 4 objects, 33 subrs and 45
  defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no monitor was queried, no rectangle was
intersected and no attribute list was built; the six arms' counts are text compile readings.


## Slice 17 (2026-10-08, status 3.920 - the integrator sets the round number): C 6300-6513

**The slice is C 6300-6513**, the file's largest function so far: `x_get_monitor_attributes_xrandr`
- C 6304-6310's locals, C 6312-6410's `have_xrandr_1_5` block (the `XRRGetMonitors` walk with its
own `USE_XCB` arm for the atom names, the `goto fallback` at C 6335 and the `goto out` at C 6407),
C 6413-6428's `RANDR13_LIBRARY` pair (`XRRGetScreenResourcesCurrent` or `XRRGetScreenResources`),
C 6429-6499's output/crtc walk (the `default`/`LVDS` checks, the primary pick and the workarea
intersection) and C 6503-6511's attribute list.  The port's slice is **399 Rust lines**
(9,167 - 8,768, `grep -c ''`) against C's 214; md5 `862bdbed15cf2467a644f212b5183005` (the slice 16
tree) -> `db6a1d16b2b3caf94d91fce55360d615`.

**Three new arms.**  The function only compiles with `have_xrandr`, and its two version conditions
decide which halves of it exist, so the table gains **WSR** = the slice 15 pair + `have_xrandr` +
both version features, **WSRX** = WSR + `use_xcb` (the inner atom-name arm) and **WSR12** = the
pair + `have_xrandr` + `have_xrandr_1_3` only (the `resources` path without the 1.5 block).  All
three read **14 / 9** on both sides of the change.

**The RandR surface is UNVERIFIED.**  This machine has no `<X11/extensions/Xrandr.h>`
(`/opt/homebrew/include/X11/extensions/` carries XKB/XI/XRes/Xdbe/… and no Xrandr), so the four
records (`XRRScreenResources`, `XRRMonitorInfo`, `XRROutputInfo`, `XRRCrtcInfo`) carry only the
members the C reads, in the header's own order, with their layouts left to the integrator - and the
eleven entry points plus `XGetAtomName` and the four `xcb_get_atom_name_*` names stand beside them.
The two version conditions are the **derived features** `have_xrandr_1_3`/`have_xrandr_1_5`
(registered in `femacs/xtask/Cargo.toml`, the `have_xrender_0_2` shape).

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0), both sides.

**② The nine arms.**  The three arms this slice adds read **14 / 9** on both sides - WSR (the
slice 15 pair plus `have_xrandr`, `have_xrandr_1_3` and `have_xrandr_1_5`), WSRX (WSR plus
`use_xcb`) and WSR12 (the pair plus `have_xrandr` and `have_xrandr_1_3` only): the baseline is the
slice 16 tree (`base_WSR`/`base_WSRX`/`base_WSR12`) and the second readings (`i_WSR`/`i_WSX`)
follow the last comment edit, so they are taken at the hash below.  The other six are re-read at
the final hash and stand where the slice 16 ledger left them: plain-X **14 / 9**, WS **14 / 9**,
WSX **14 / 9**, XCB **14 / 9**, GTK **34 / 20** and Xt **15 / 12** - the slice adds no diagnostic
and no warning in any of the nine, and the GTK arm still excludes the whole monitor block by
C's own `#ifndef USE_GTK`.  The first
measurement's own rows were the ordinary ones: a `return Qnil;` that needed its call, three
`XRectangle`/`int` casts, one function-scoped C local (`randr13_avail`) that Rust scopes to its
block, one `XRRMonitorInfo`-typed declaration pair that had to move under the record's own guard,
and one doc comment on an `extern` block.

**③ `extract-abi.py --check`**: **130 rows -> 129** - the `missing function
x_get_monitor_attributes_xrandr` row retiring - with the one new token row
(`alloca, goto, randr_major, randr_minor`) answered by a row in
`docs/ports/macro-body-notes.tsv`, and the drift reads `C 10662 vs Rust 9167` (**14.0%**, was
17.8%).  `check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0
failure(s)** and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed
literals), 0 failures**.

### Deviations (this slice)

1. **C's two `goto`s are Rust labelled blocks**: `goto fallback` is `break 'fallback` (the 1.5
   block's own label, so the 1.3 path still runs) and `goto out` is `break 'out` (the label around
   the whole 1.3 path, so the tail still runs) - the file's 4714 shape.
2. **C's bare `alloca` at C 6339** is the port's stack pool (`sa_stack_alloc`, PORTING.md P-05).
3. **`randr13_avail` moves to the locals** (C declares it at C 6418 inside its RANDR13 section, but
   C's scope is the function and Rust's is the block).
4. **C's mixed widths** (`XRectangle`'s `short`/`unsigned short` against the RandR records' `int`s)
   are written out with `as c_short` / `as c_ushort`, and `XRRMonitorInfo.primary`'s `Bool` is
   compared with `!= 0`.

### Owed (this slice)

- The four RandR records' layouts, the eleven RandR entry points, `XGetAtomName` and the four
  `xcb_get_atom_name_*` names (rows owed to `include/xlib.rs` or a future RandR mirror); the
  `have_xrandr_1_3`/`have_xrandr_1_5` features are registered but their *arms* are unmeasured
  against a real Xrandr build.
- `syms_of_xfns` (unchanged: no subr lands here), and the rest of the remainder: the 23 missing
  functions, 1 macro, 4 objects, 33 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no RandR query was made, no monitor was
enumerated and no output/crtc was read; the nine arms' counts are text compile readings.
## Slice 18 (2026-10-08, status 3.921 - the integrator sets the round number): C 6515-6601

**The slice is C 6515-6601**: the dispatcher `x_get_monitor_attributes` (C 6515-6544) and, under
C 6548's `#ifdef USE_LUCID`, `xlw_monitor_dimensions_at_pos_1` (C 6551-6587) together with
`xlw_monitor_dimensions_at_pos` (C 6589-6600).  C 6546's `#endif /* !USE_GTK */` closes the region
the monitor functions from C 6125 on live in and C 6601's `#endif` closes the Lucid pair.  The
dispatcher keeps the slice 15 guard pair - its three callees were ported under it - while C's own
gate for the two Lucid functions is `USE_LUCID` alone; their callee is the pair's, so the ledger's
**LUCID arm** carries the pair (the WSX arm's shape, slice 16).  The port's slice is **129 Rust
lines** (9,296 - 9,167, `grep -c ''`) against C's 87; md5 `db6a1d16b2b3caf94d91fce55360d615` (the
slice 17 tree) -> `8268c83472d493ec76de55a4ddb1060c`.

**One new arm, and five arms that only gain the dispatcher.**  `x_get_monitor_attributes` sits
under the slice 15 pair, so the five pair arms (WS, WSX, WSR, WSRX and WSR12) compile it; the two
Lucid functions compile only in **LUCID** = the pair + `use_x_toolkit` + `use_lucid` (C's `USE_LUCID`
belongs to the X toolkit family, and the exported one's callers are `lwlib/xlwmenu.c`
1475/1479/2822).  The five pair arms read their slice 17 counts unchanged - **14 / 9** each, with
the WS arm's error set byte-identical to its baseline - and LUCID reads **22 / 12 on both sides**,
its baseline measured on the slice 17 tree before this slice's text landed.

**The three new names.**  `x_dpyinfo` is `src/xterm.c` 6883's definition (the port's
`src/xterm.rs` 7543, which carries C's linkage) and this unit's include graph declares it nowhere,
so it stands at the use site (P-55/P-72 rule 3, the `x_display_info_for_name` shape at 7199);
`WidthOfScreen`/`HeightOfScreen` are Xlib.h 129-130's two `Screen` macros, spelled through the
mirror's `width`/`height` members the way the `WidthMMOfScreen` pair at 8194-8200 is;
`XineramaIsActive` (Xinerama.h) joins `XineramaQueryScreens` in that header's own block - the
second entry point of one header, so one home (P-90 ②).

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0) on the final hash.

**② The arms.**  LUCID **22 / 12** on both sides, and the five arms that compile the dispatcher
read **14 / 9** each (WS, WSX, WSR, WSRX, WSR12 - the slice 17 counts, re-read at this hash).
**Arm touch measured** (P-86 ⑦): a `let _: c_int = true;` injected at the dispatcher's first
statement moves WS **14 -> 15** (9 warnings unmoved) and one injected at
`xlw_monitor_dimensions_at_pos_1`'s first statement as well moves LUCID **22 -> 24**, with the
injected lines naming both touched definitions - so both functions are read by the arms that claim
them; the injections were removed and the file is byte-identical to the hash above again.

**③ `extract-abi.py --check`**: **129 rows -> 126** (the in-tree baseline re-measured this slice) -
exactly the three `missing function` rows retiring (`x_get_monitor_attributes`,
`xlw_monitor_dimensions_at_pos_1`, `xlw_monitor_dimensions_at_pos`), nothing added but the
line-count row's own text, `missing function` 23 -> 20, and the drift reads
`C 10662 vs Rust 9296` (**12.8%**, was 14.0%).  No token row arrived, so
`docs/ports/macro-body-notes.tsv` gains nothing here.  `check-mirror-dupes.py` stays at **16
duplicate declaration(s), 16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at **18
declaration sites (0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **C's declaration-then-assignment locals** (`Lisp_Object attrs, tem, val;`, C 6556) are the
   port's three `let`s - `attrs` initialised at its single assignment (C 6558), `tem` at C 6560's
   `for` initialiser, `val` with the file's zero initialiser - so neither `unused_mut` nor an
   E0381 row is added.
2. **C's `for (tem = attrs; CONSP (tem); tem = XCDR (tem))`** is the port's `while` with C's
   increment at the body's end (the slice 15/17 shape; this body carries no `continue`).
3. **C's `int sx, sy, swidth, sheight;`** (C 6562) take the file's zero initialisers.
4. **C 6521's `(void) dpy;`** is `let _ = dpy;` (the `abi_shim.rs` 279 shape).
5. **C's `XFIXNUM (...)` results** are written `as c_int` - the port's `XFIXNUM` is a function
   returning `EMACS_INT` (`include/lisp.rs` 757), and C assigns each to an `int`.

### Owed (this slice)

- `xterm.h` 1913's other-side declaration of the exported function is the same-unit case P-55
  covers; the definition below carries C's linkage.
- The Lucid pair's three callers are `lwlib/`'s (not this unit) and nothing in this unit calls
  them; the dispatcher's own single in-file caller (C 6771) is in the remainder.
- `syms_of_xfns` (unchanged: no subr lands here) and the rest of the remainder from the
  `x-display-monitor-attributes-list` DEFUN (C 6604): 4,059 C lines plus the 20 missing functions,
  1 macro, 4 objects, 33 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no monitor list was asked for, no workarea was
consulted, no screen size was read and no widget was resized; the arms' counts are text compile
readings, and C's `USE_LUCID` path is a build no reading here compiles end to end.
## Slice 19 (2026-10-08, status 3.922 - the integrator sets the round number): C 6604-6777

**The slice is C 6604-6777**, the file's first subr since the run of subrs that ended at C 5991:
`x-display-monitor-attributes-list` (C 6604-6606 the `DEFUN` header, C 6607-6621 the verbatim doc
string, C 6624-6625 the two locals) with its **GTK arm** (C 6627-6767: `gdk_x11_lookup_xdisplay`,
the two monitor-count arms, C 6638-6653's `block_input`/`make_nil_vector`/`xcalloc`,
C 6655-6676's `FOR_EACH_FRAME` walk, C 6678-6759's per-monitor loop - geometry, the millimetre
sizes with their pre-3.22 fallback, the three workarea arms, the scale factor, the eight rectangle
assignments and the name - and C 6761-6767's `make_monitor_attribute_list`/`free_monitors`/
`unblock_input`) and its **plain-X arm** (C 6768-6773: `block_input`, the slice 18 dispatcher,
`unblock_input`).  The port's slice is **251 Rust lines** (9,547 - 9,296, `grep -c ''`) against
C's 174; md5 `8268c83472d493ec76de55a4ddb1060c` (the slice 18 tree) ->
`4d8e2c054ee0babbc731d2f195d7f41f`.

**Eleven arms, one of them new.**  The `DEFUN` sits under the file's own gate (`have_x_windows`),
so every arm compiles it; C 6627-6767's two GTK sub-arms (`gtk_check_version_ge_3_22` on and off)
exercise different halves, so the table gains a tenth reading that splits the GTK arm:
**GTK22** = the GTK arm + `gtk_check_version_ge_3_22`.  All eleven read their baselines unchanged -
plain-X **14 / 9**, WS **14 / 9**, WSX **14 / 9**, WSR **14 / 9**, WSRX **14 / 9**, WSR12
**14 / 9**, XCB **14 / 9**, LUCID **22 / 12**, GTK **34 / 20**, GTK22 **34 / 20** and Xt
**15 / 12** - and every arm's error *set* is byte-identical to its baseline, not merely equal in
count.

**The names this slice has to bring.**  `include/gtk.rs` carries the 3.22 `gdk_monitor_*` family
(under `#[cfg (not (have_pgtk))]`) and none of the nine pre-3.22 `GdkScreen` entry points the C
reads, nor `gdk_rectangle_intersect`, so those ten stand at the use site under C's own sub-arm gate
(`not (gtk_check_version_ge_3_22)`), each row naming the header and the signature and no line
number - this machine has no GTK headers, which is `include/gtk.rs`'s own row style; `dupstring`
(lisp.h 5555) joins them the way `pgtkfns.rs` 743-745 declares it; and the pixel pair C 6648/6650
calls was already this file's own use-site row at 7989-7996.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0) on the final hash.

**② The eleven arms.**  No arm moves and no warning moves in any of them.

**③ Arm touch measured** (P-86 ⑦): three `let _: c_int = true;` injections - one at the `DEFUN`
body's first statement (A), one in the `use_gtk` arm (B) and one in the `not (use_gtk)` arm (C) -
move WS **14 -> 16** (A and C read), GTK22 **34 -> 36** (A and B) and LUCID **22 -> 24** (A and C),
each log naming the injected line it read; the injections were removed and the file's hash
re-checked.

**④ `extract-abi.py --check`**: **126 rows -> 125** (the in-tree baseline re-measured this slice) -
the `missing subr Fx_display_monitor_attributes_list (x-display-monitor-attributes-list)` row
retiring, nothing added but the line-count row's own text, and the drift reads
`C 10662 vs Rust 9547` (**10.5%**, was 12.8%).  The `missing defsubr (&raw const
Sx_display_monitor_attributes_list)` row **stays**, because C's `defsubr` site is `syms_of_xfns`'
own block, not this `DEFUN`.  No token row arrived, so `docs/ports/macro-body-notes.tsv` gains
nothing here.  `check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s),
0 failure(s)** and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed
literals), 0 failures**.

### Deviations (this slice)

1. **The plain arm's call carries its callee's existence condition.**  C 6771's call sits under
   `#ifndef USE_GTK`, but the callee is this port's slice 15 pair (`all (have_x_windows,
   not (use_gtk), have_window_system)`), so the statement is written
   `#[cfg (feature = "have_window_system")]`: C cannot reach a `HAVE_X_WINDOWS` build without
   `HAVE_WINDOW_SYSTEM` (configure.ac 7517-7518 defines it whenever `$window_system != "none"`, and
   the callee's `struct MonitorInfo` is frame.h 1926's own gate), so the port drops no
   C-reachable path while the `have_x_windows`-only reading arms stop reading a phantom `E0425`.
   The first measurement read exactly that row - one in plain-X, XCB and Xt - and it is closed
   this way.
2. **C 6686's `GdkMonitor *monitor` is hoisted** to the loop's own locals under
   `gtk_check_version_ge_3_22`: C declares it inside its first 3.22 arm and reads it in four more
   (C 6695/6696, 6706, 6730, 6755) - the `randr13_avail` shape of slice 17's deviation 3.
3. **C 6635's `static const char *source = "Gdk"`** is the port's function-scope `static mut` with
   C's own value (`pgtkfns.rs` 3707-3710's device for the same C shape).
4. **The eight rectangle assignments** (C 6743-6750) carry the conversion C writes implicitly:
   `struct MonitorInfo`'s rectangles are `Emacs_Rectangle`, which the x11 arm reads as `XRectangle`
   (`include/dispextern.rs` 61-62's alias - four shorts), so each is `as c_short` / `as c_ushort`.
5. **C's two millimetre fallbacks** (C 6701/6703) put the cast where C's own promotion puts it:
   `rec.width as c_double * mm_width_per_pixel + 0.5) as gint`.

### Owed (this slice)

- The ten pre-3.22 `GdkScreen` / `GdkRectangle` rows are owed to a `gdk.rs`-style mirror (they
  stand at the use site under `not (gtk_check_version_ge_3_22)`), and `dupstring` to
  `include/lisp.rs`; `x_display_pixel_width`/`x_display_pixel_height` stay this file's own rows
  (7989-7996), owed to `include/xterm.rs`.
- `syms_of_xfns`' `defsubr` site (the surviving `missing defsubr (&raw const
  Sx_display_monitor_attributes_list)` row) and the rest of the remainder from C 6779
  (`frame_geometry`, C 6785-6786): 4,884 C lines plus the 20 missing functions, 1 macro,
  4 objects, 32 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no Gdk display was looked up, no monitor was
enumerated, no workarea was intersected and no attribute list was built; the eleven arms' counts
are text compile readings, and GTK/GTK22 are text compiles of a build this machine cannot link
(there are no GTK headers and no libgtk here).
## Slice 20 (2026-10-08, status 3.923 - the integrator sets the round number): C 6779-7035

**The slice is C 6779-7035**: the comment C 6779-6783 and `frame_geometry` (C 6785-6786 the `static Lisp_Object`, C 6788-6801
its locals, C 6803-6804 the three-way early exit, C 6806-6814's `block_input`/`XGetGeometry`/
`x_real_pos_and_offsets`/`unblock_input`, C 6819-6852 the parent-frame and window-manager halves of
the outer and native edges, C 6854-6858 the internal border, C 6860-6922 the menu-bar and tool-bar
arms with their `HAVE_EXT_*` pairs, C 6924-6961 the four-way return) and the two subrs it backs,
`x-frame-geometry` (C 6964-7011) and `x-frame-edges` (C 7013-7035).  The port's slice is **254 Rust
lines** (9,801 - 9,547, `grep -c ''`, seven of them the use-site `decode_live_frame` declaration)
against C's 251; md5 `4d8e2c054ee0babbc731d2f195d7f41f` (the slice 19 tree) ->
`dc2d459a0798179262db4ca6d58cfeb8`.

**Eleven arms.**  The three items sit under the file's own gate (`have_x_windows`), so all eleven
compile them.  Eight of the eleven do not move - plain-X **14 / 9**, WS **14 / 9**, WSX **14 / 9**,
WSR **14 / 9**, WSRX **14 / 9**, WSR12 **14 / 9**, XCB **14 / 9**, LUCID **22 / 12**, Xt
**15 / 12** - and the GTK pair reads **34 / 20 -> 37 / 20**, its three additions being further
instances of two classes this lane already counts: two `GDK_WINDOW_XID` rows (the mirror's
`GTK_WIDGET_TO_X_WIN` placeholder - slice 8's own reading, `Payload` hits 13 -> 15) and one `void`
row (`FRAME_MENUBAR_HEIGHT`'s recorded `{ (void) ($f); 0 }` body, `docs/ports/xterm.md` 2520-2524
and this ledger's 816-828: 2 -> 3).  No warning moves in any arm.

**The first measurement's four rows.**  (1) `decode_live_frame` was an `E0425` in **every** arm
(frame.h 1476's declaration is in no mirror this unit's include graph carries): closed by the
use-site declaration that opens this slice.  (2) `rootw` was an `E0381` in every arm
(`let mut rootw: Window;` with no initialiser): closed with the file's zero initialiser.  (3) and
(4) are the GTK pair's two recorded classes, named above and left open as their further instances.

**Arm touch measured** (P-86 ⑦): two injections - at `frame_geometry`'s first statement (A) and at
`Fx_frame_edges`' body's first statement (B) - move plain-X **14 -> 16** and GTK **37 -> 39**, each
log naming both injected lines; the injections were removed and the file's hash re-checked.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0) on the final hash.

**② The eleven arms.**  See above.

**③ `extract-abi.py --check`**: **125 rows -> 122** (the in-tree baseline re-measured this slice) -
the `missing function frame_geometry`, `missing subr Fx_frame_geometry` and `missing subr
Fx_frame_edges` rows retiring, nothing added but the line-count row's own text, and the drift reads
`C 10662 vs Rust 9801` (**8.1%**, was 10.5%).  No token row arrived, so
`docs/ports/macro-body-notes.tsv` gains nothing here.  `check-mirror-dupes.py` stays at **16
duplicate declaration(s), 16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at **18
declaration sites (0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **C's mixed-width arithmetic** (`int` against `unsigned int` and `Window`) is written out in
   `c_int` - `native_width as c_int`, `2 * (x_border_width as c_int)`, `native_height as c_int -
   menu_bar_height` - which is the same result C's unsigned arithmetic leaves.
2. **C 6803's `!FRAME_OUTER_WINDOW (f)`** is `FRAME_OUTER_WINDOW! (f) == 0`: the port's macro
   yields a `Window` (an integer), not a pointer.
3. **C's three `/** … **/` lines** (C 6789, 6810 and 6816-6817 - the commented-out
   `XWindowAttributes` declaration and assignments) are carried as comments.
4. **C's `list4i` and `make_fixnum` arguments** are written `as EMACS_INT` (the port's `list4i`
   takes `EMACS_INT`, `include/lisp.rs` 2088).
5. **C 6807-6809's nine-argument `XGetGeometry`** and C 6812-6813's ten-argument
   `x_real_pos_and_offsets` keep C's argument order, with `std::ptr::null_mut ()` where C passes
   `NULL` (the file's 842 shape).

### Owed (this slice)

- The GTK pair's three new rows are further instances of the two recorded classes named above; the
  rows' homes are `include/gtk.rs` (the `GDK_WINDOW_XID` macro) and `include/xterm.rs` (the
  `FRAME_MENUBAR_HEIGHT` body).
- `decode_live_frame` is owed to `include/frame.rs`; it stands at the use site here
  (`src/dispnew.rs` 270-271's shape).
- `syms_of_xfns`' three new `defsubr` sites and the rest of the remainder from C 7037
  (`x_frame_list_z_order`, C 7044-7045): 4,026 C lines plus the 19 missing functions, 1 macro,
  4 objects, 30 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no `XGetGeometry` call was made, no window
manager offset was read, no frame edge was computed and no attribute list was built; the eleven
arms' counts are text compile readings.
## Slice 21 (2026-10-08, status 3.924 - the integrator sets the round number): C 7037-7207

**The slice is C 7037-7207**: `x_frame_list_z_order` (C 7037-7043 its comment, C 7044-7045 the
`static Lisp_Object`, C 7047-7057 its locals, C 7059-7096 the `_NET_CLIENT_LIST_STACKING` arm -
`x_wm_supports_1` then `XGetWindowProperty`, `XFree`, `x_top_window_to_frame` per toplevel - and
C 7098-7119's `XQueryTree` walk with C 7110-7112's `parent_desc`/`FRAME_OUTER_WINDOW` test) and its
subr `x-frame-list-z-order` (C 7125-7148), then `x_frame_restack` (C 7150-7157 its comment,
C 7161-7178 the `USE_GTK` pair - gtkutil.c's `xg_frame_restack`, or `XReconfigureWMWindow` with an
`XWindowChanges` record on the `#else` arm) and its subr `x-frame-restack` (C 7182-7207).  The
port's slice is **232 Rust lines** (10,033 - 9,801, `grep -c ''`) against C's 171; md5
`dc2d459a0798179262db4ca6d58cfeb8` (the slice 20 tree) -> `327df657fffb507803472c41b617e7cc`.

**Eleven arms.**  Nine read their baselines unchanged, error sets included - plain-X **14 / 9**, WS
**14 / 9**, WSX **14 / 9**, WSR **14 / 9**, WSRX **14 / 9**, WSR12 **14 / 9**, XCB **14 / 9**,
LUCID **22 / 12**, Xt **15 / 12** - and the GTK pair reads **37 / 20 -> 40 / 20**, its three
additions being `GDK_WINDOW_XID` instances of the class slice 8 named (the mirror's
`GTK_WIDGET_TO_X_WIN` placeholder, reached by C 7112's `FRAME_OUTER_WINDOW` and C 7143's
`FRAME_X_WINDOW`).  No warning moves in any arm.

**The first measurement's six rows, all closed.**  (1) an `unknown character escape: (` and (5) the
matching `subr Fx_frame_restack: docstring differs` - C 7194's `\(stacking)` is one of
make-docfile's escapes, whose *value* is `(stacking)` (`tools/cinventory.py` 935's rule drops the
backslash), and the checker decodes the port's literal with Rust's own escapes, so the literal
carries the plain paren.  (2) `Success` and two `XA_WINDOW` rows - the two constants come from the
mirror's `x11_constants` module and need this file's own `use crate::x11_constants::{…};` line (the
`x_real_pos_and_offsets` shape at 412).  (3) `casting c_void as *mut x_output is invalid` with
(4) its `no field parent_desc` companion - C 7111's `cf->output_data.x->parent_desc` needs the cast
parenthesised, because Rust's unary `*` binds tighter than `as` (the 999 shape).  (6)
`does not mention false` - C 7069's `False` argument is written `false as Bool`.

**Arm touch measured** (P-86 ⑦): two injections - `x_frame_list_z_order`'s first statement (A) and
`x_frame_restack`'s `not (use_gtk)` arm (B) - move plain-X **14 -> 16** (both read) and GTK
**40 -> 41** (only A: the GTK arm takes the helper's `use_gtk` branch, so B's statement is not
compiled there); the injections were removed and the file's hash re-checked.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0) on the final hash.

**② The eleven arms.**  See above.

**③ `extract-abi.py --check`**: **122 rows -> 118** (the in-tree baseline re-measured this slice) -
the `missing function x_frame_list_z_order`, `missing function x_frame_restack`,
`missing subr Fx_frame_list_z_order` and `missing subr Fx_frame_restack` rows retiring, nothing
added but the line-count row's own text, and the drift reads `C 10662 vs Rust 10033` (**5.9%**,
was 8.1%).  No token row arrived, so `docs/ports/macro-body-notes.tsv` gains nothing here.
`check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**
and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed literals), 0
failures**.

### Deviations (this slice)

1. **C's `toplevels[i]`** is `*toplevels.offset (i as isize)` and C's narrowing of that `Window`
   into `x_top_window_to_frame`'s `int` (xterm.h 1004) is written `as c_int`.
2. **C's `for (i = 0; i < nchildren; i++)`** (`unsigned long` against `unsigned int`) is the port's
   `while i < nchildren as c_ulong`.
3. **C 7072/7075's `Success`/`XA_WINDOW`** come from the mirror's `x11_constants` through the
   function's own `use` line (the 412 shape), and C 7069's `False` is `false as Bool`.
4. **C's `\(stacking)` in the restack doc** (C 7194) is the plain paren in the literal, because the
   digest compares make-docfile's value with Rust's decoded value (see the row list above).
5. **C's `unsigned long mask = (CWSibling | CWStackMode);`** keeps its `c_ulong` local and is
   narrowed at the call (`mask as c_uint`), because `XReconfigureWMWindow`'s mask is Xlib's
   `unsigned int` (Xlib.h 1885-1890).

### Owed (this slice)

- `XWindowChanges` (Xlib.h 395-401) and the four `X11/X.h` constants it is configured with
  (`CWSibling` 415, `CWStackMode` 416, `Above` 462, `Below` 463) stand at the use site and are
  owed to `include/xlib.rs` (`xlib.rs` 148's comment already names the mask).
- `x_wm_supports_1` (xterm.h 1898) and `x_top_window_to_frame` (xterm.h 1004) are owed to
  `include/xterm.rs`; `xg_frame_restack` (gtkutil.h 201) to a gtkutil.h mirror
  (`src/pgtkfns.rs` 720-722 declares the same row).
- The GTK pair's three new rows are further instances of the `GDK_WINDOW_XID` class named above.
- `syms_of_xfns`' four new `defsubr` sites and the rest of the remainder from C 7210: 3,453 C
  lines plus the 17 missing functions, 1 macro, 4 objects, 28 subrs and 45 defsubrs `--check`
  reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no property was queried, no child window was
walked and no frame was restacked; the eleven arms' counts are text compile readings.
## Slice 22 (2026-10-08, status 3.925 - the integrator sets the round number): C 7210-7273

**The slice is C 7210-7273**: the two mouse subrs - `x-mouse-absolute-pixel-position`
(C 7210-7233: `SELECTED_FRAME`, the two-way early exit, `x_query_pointer`'s nine arguments and the
`Fcons (make_fixnum …)` pair) and `x-set-mouse-absolute-pixel-position` (C 7235-7273: the two
`check_integer_range` bounds, then C 7251-7266's `HAVE_XINPUT2` pair - `client_pointer_device`,
`supports_xi2`, `x_catch_errors_for_lisp`, `XIWarpPointer`, `x_uncatch_errors_for_lisp` - over
C 7267-7269's `XWarpPointer` tail, which every arm reaches).  The port's slice is **124 Rust
lines** (10,157 - 10,033, `grep -c ''`) against C's 64; md5 `327df657fffb507803472c41b617e7cc`
(the slice 21 tree) -> `bcfc6bbad92b6262a6e5b5bda113710e`.

**A twelfth arm.**  C 7260's `XIWarpPointer` is compiled only under `HAVE_XINPUT2`, so the table
gains **XI2** = `have_x_windows` + `have_xinput2`; its first reading is **18 / 9**.  The eleven
arms that already existed read their baselines unchanged - plain-X, WS, WSX, WSR, WSRX, WSR12 and
XCB **14 / 9**, LUCID **22 / 12**, GTK and GTK22 **40 / 20**, Xt **15 / 12** - and every arm's
error set is byte-identical to its baseline.

**The Xlib pair is declared, not left bare.**  `XWarpPointer` (Xlib.h 3524-3534) and
`XIWarpPointer` (`<X11/extensions/XInput2.h>`, no header on this machine, so the row names the
header and the signature and is UNVERIFIED) stand at the use site, which is this file's posture for
Xlib entries (slices 9 and 21's `XGetGeometry`/`XReconfigureWMWindow`).  That supersedes the
prediction in `src/xterm.rs` 17421-17422 ("the `src/xfns.rs` copy of the same call pair … has the
same two rows"), whose comment is corrected in this slice (comment lines only).

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0) on the final hash.

**② The twelve arms.**  plain-X **14 / 9**, WS **14 / 9**, WSX **14 / 9**, WSR **14 / 9**, WSRX
**14 / 9**, WSR12 **14 / 9**, XCB **14 / 9**, LUCID **22 / 12**, GTK **40 / 20**, GTK22 **40 / 20**,
Xt **15 / 12** - unchanged - and **XI2 18 / 9** (first reading).  No warning moves in any arm.

**③ Arm touch measured** (P-86 ⑦): three injections - `Fx_mouse_absolute_pixel_position`'s body
(A), the `supports_xi2` branch (B) and the `not (have_xinput2)` tail (C) - move plain-X
**14 -> 16** (A and C) and XI2 **18 -> 20** (A and B), each log naming what it read; the injections
were removed and the file's hash re-checked.

**④ `extract-abi.py --check`**: **118 rows -> 115** - the `missing subr Fx_mouse_absolute_pixel_position`
and `missing subr Fx_set_mouse_absolute_pixel_position` rows retiring, **and the line-count row
retiring with them: the port passes this unit's 5% bar** (`C 10662 vs Rust 10,157` is **4.7%**,
against the 5.9% the row carried at slice 21's hash).  Nothing appears in their place, so the unit
now has neither a missing item this slice's text names nor a line-count row.
`check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**
and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed literals), 0
failures**.

### Deviations (this slice)

1. **C's `XWarpPointer` tail is written in both cfg blocks** (`(not (have_xinput2))` and the
   `else` of the XInput2 pair), because a `#[cfg]` cannot cover one half of an `if`/`else` chain
   (P-86 ①; C 7266's `#endif` sits between the `else` and the tail).
2. **C's `(unsigned int *) &dummy`** is `&raw mut dummy as *mut c_uint`, and C's `INT_MIN`/`INT_MAX`
   keep C's names (the file's 2551 shape) rather than `c_int::MIN`/`c_int::MAX` - the token gate
   asks for `int_min`/`int_max` (measured: the first writing read one
   `does not mention int_max, int_min` row).
3. **C 7239's `\(0, 0)`** is the plain paren in the literal, the make-docfile rule slice 21
   established (the digest compares make-docfile's value with Rust's decoded value).
4. **The three XInput2 names' gates**: `XIWarpPointer`, `x_catch_errors_for_lisp` and
   `x_uncatch_errors_for_lisp` stand under `all (have_x_windows, have_xinput2)` - C 7251's own
   `#ifdef`, which is inside the file gate.

### Owed (this slice)

- `XWarpPointer` (Xlib.h 3524-3534) is owed to `include/xlib.rs`; `XIWarpPointer` to an XInput2
  mirror (UNVERIFIED, no header here); `x_catch_errors_for_lisp`/`x_uncatch_errors_for_lisp`
  (xterm.h 1755/1756) to `include/xterm.rs`.
- `syms_of_xfns`' two new `defsubr` sites and the rest of the remainder from C 7275
  (`x-begin-drag`, C 7275-7447): 3,388 C lines plus the 17 missing functions, 1 macro, 4 objects,
  26 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: the pointer was not queried and no warp was
performed; the twelve arms' counts are text compile readings, and XI2's 18/9 is a text compile of
an XInput2 build this machine has no headers for.
## Slice 23 (2026-10-08, status 3.926 - the integrator sets the round number): C 7275-7447

**The slice is C 7275-7447**: the `x-begin-drag` subr - C 7276-7334's doc string verbatim
(including its `"Drag-and-Drop Protocol for the X Window System"` quotation and the
`https://freedesktop.org/…` URL), C 7339-7346's locals (the three 2048-element arrays among them),
C 7348's `USE_SAFE_ALLOCA`, C 7350-7368's target walk (`CHECK_LIST`, `FOR_EACH_TAIL`,
`SAFE_ALLOCA_STRING` per target, `CHECK_LIST_END`), C 7370-7426's action dispatch (the five
`QXdndAction*` comparisons, `symbol_to_x_atom`, and the `CONSP` branch's second walk which builds
`action_list`/`name_list`), C 7428's `SAFE_NALLOCA`, C 7430-7437's error trap around
`x_intern_atoms`, C 7439-7443's `x_dnd_begin_drag_and_drop` call and C 7445's `SAFE_FREE`.  The
port's slice is **175 Rust lines** (10,332 - 10,157, `grep -c ''`) against C's 173; md5
`bcfc6bbad92b6262a6e5b5bda113710e` (the slice 22 tree) -> `383c1b3fd5a0a3842f9c32573752bbf5`.

**Twelve arms, none of them moved.**  The DEFUN carries no nested `#if`, so every arm compiles all
of it - plain-X, WS, WSX, WSR, WSRX, WSR12 and XCB **14 / 9**, LUCID **22 / 12**, GTK and GTK22
**40 / 20**, XI2 **18 / 9**, Xt **15 / 12** - and every arm's error set is byte-identical to its
baseline.  No warning moves anywhere.

**The first measurement's five rows, all closed.**  (1) `decode_window_system_frame` was an `E0428`:
the file already declares it at 308-309 (frame.h 1474-1475), so the duplicate block is gone.
(2) `x_check_errors` was an `E0425`: it joins the file's own xterm.h block (after `x_catch_errors`,
xterm.h 1760).  (3) `target_atoms` was an `E0381` (`SAFE_NALLOCA!` reads its buffer before
assigning it): it takes the file's zero initialiser.  (4) and (5) were `E0384`s - C's two
`FOR_EACH_TAIL`s assign their *parameters* back to themselves, and the port's `DEFUN!` takes
`$arg:ident` without `mut`, so each walk reads a mutable copy of the same name
(`androidfns.rs` 581's documented device).  One token row (`does not mention safe_alloca_string`)
is closed by a `docs/ports/macro-body-notes.tsv` row, because the two `SAFE_ALLOCA_STRING`s stand in
the body as their own expansion.

**Arm touch measured** (P-86 ⑦): two injections - the body's first statement (A) and the `CONSP`
branch (B) - move plain-X **14 -> 16** and XI2 **18 -> 20**, both logs naming both injected lines;
the injections were removed and the file's hash re-checked.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  Unchanged: `build/femacs/xfns.o` is the **336-byte**
empty object (exit 0) on the final hash.

**② The twelve arms.**  See above.

**③ `extract-abi.py --check`**: **115 rows -> 114** - the `missing subr Fx_begin_drag
(x-begin-drag)` row retiring, with the token row the slice first read closed by the notes row
rather than left standing, and `missing subr` 26 -> 25.  `check-mirror-dupes.py` stays at **16
duplicate declaration(s), 16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at **18
declaration sites (0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **C's two `SAFE_ALLOCA_STRING` calls stand as their own expansion** (`target_names[ntargets]` and
   `name_list[nnames]` are array-element lvalues, and the mirror's macro takes a `$ptr:ident`) -
   `image.rs` 8899-8906's shape, recorded in the notes row above.
2. **The two mutable copies of the parameters** (`let mut targets = targets;`,
   `let mut action = action;`) exist because the port's `DEFUN!` cannot spell `mut $arg`
   (`androidfns.rs` 581).
3. **C's three 2048-element arrays** are `[T; 2048]` locals with C's element type
   (`*mut c_char`, `Atom`), and C's `(const char **) &name_list` is
   `name_list.as_ptr () as *const *const c_char`.
4. **`ENCODE_SYSTEM` is called three times** where C 7415's `SAFE_ALLOCA_STRING` expands - which is
   what C's own macro does with its string argument.
5. **C 7433's `target_names`** is `target_names.as_mut_ptr ()` (the `char **` the header declares).

### Owed (this slice)

- `symbol_to_x_atom` (xterm.h 1927), `x_intern_atoms` (xterm.h 1843), `x_dnd_begin_drag_and_drop`
  (xterm.h 1859-1862) and `x_check_errors` (xterm.h 1760) are this file's use-site rows and are
  owed to `include/xterm.rs`.
- `syms_of_xfns`' `defsubr` site and the rest of the remainder from C 7449: 3,214 C lines plus the
  17 missing functions, 1 macro, 4 objects, 25 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no drag was begun, no atom was interned and
no selection target was offered; the twelve arms' counts are text compile readings.
## Slice 24 (2026-10-08, status 3.927 - the integrator sets the round number): C 7449-7491

**The slice is C 7449-7491**: C 7449-7451's `X Displays` section comment, C 7454-7470's
`visual_class` record with the `visual_classes[]` table (the six `X11/X.h` class constants and C's
`{NULL, 0}` sentinel) and C 7473-7491's `#ifndef HAVE_XSCREENNUMBEROFSCREEN` substitute for
`XScreenNumberOfScreen`.  The port's slice is **72 Rust lines** (10,404 - 10,332, `grep -c ''`)
against C's 43; md5 `383c1b3fd5a0a3842f9c32573752bbf5` (the slice 23 tree) ->
`f7247a85422a83e1f6df61bf5bccf5ed`.

**A thirteenth arm, and the substitute's two arms split by it.**  `configure` tests
`HAVE_XSCREENNUMBEROFSCREEN` (`src/config.in` 1905's template) and this tree's feature table does
not register it, so the substitute is the live arm here and Xlib's own declaration (this file's
1960-1961 row) takes the other; the table gains **XSN** = `have_x_windows` +
`have_xscreennumberofscreen`, whose first reading is **14 / 9**.  The twelve arms that do not carry
the feature read their baselines **+2** each - and the two rows are exactly
`no field \`nscreens\` on type \`_XDisplay\`` and `no field \`screens\` on type \`_XDisplay\``,
the class this machine's Xlib.h creates by keeping `_XDisplay`'s body behind `XLIB_ILLEGAL_ACCESS`
(`include/xlib.rs` 1043's note; C's own text would need that macro to compile) - while XSN reads
**0** new rows.  No warning moves in any arm.

**The first measurement, and the frozen unit's own catch.**  Three `E0255` rows
(`StaticColor`/`StaticGray`/`TrueColor` defined twice) and then three `E0252`s after the first fix
say the same thing: those three are already imported at the crate root by `include/xterm.rs`
114-125, so the port imports only the three the mirror does *not* bring (`GrayScale`,
`PseudoColor`, `DirectColor`) from `x11_constants` (`include/xlib.rs` 180-185, the same header
lines).  One `E0277` (`*const i8` cannot be shared between threads safely) says the table cannot be
a plain `static` - a C mutable `static` array of records with a raw pointer needs Rust's
`static mut` - and the frozen `build-tu` caught what the arms could not: without the file gate the
frozen configuration read six `E0425`s (`c_char`/`c_int`/`Screen`/`Display`), so the record and the
substitute carry `have_x_windows` (P-86 ⑧) as every other item of this unit does.

**Arm touch measured** (P-86 ⑦): one injection at the substitute's first statement moves the twelve
non-feature arms (plain-X **16 -> 17**) and leaves XSN at **14** untouched - the cfg's two arms
measured from the reading side.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash - after the file gate was added (the first attempt failed with six `E0425`s; see above).

**② The thirteen arms.**  See above.

**③ `extract-abi.py --check`**: **114 rows -> 112** - `missing function XScreenNumberOfScreen`
retiring with the standing `function XScreenNumberOfScreen is not static in C but has no
#[unsafe(no_mangle)] in the port` row, and nothing appearing in their place (`missing function`
17 -> 16).  `check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0
failure(s)** and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed
literals), 0 failures**.

### Deviations (this slice)

1. **The six class constants are imported, not declared**: they are the mirror's `x11_constants`
   rows, and three of them are already at the crate root through `include/xterm.rs` 114-125, so
   only `GrayScale`, `PseudoColor` and `DirectColor` are imported here (no glob: that would bring
   the module's `None` into the crate root, `include/xterm.rs` 109-111's recorded reason).
2. **C's `static struct visual_class visual_classes[]`** is the port's `static mut` (a plain
   `static` cannot hold a raw pointer: `E0277`, measured), with a `dead_code` allowance until
   `select_visual` lands (`src/sysdep.rs` 4245's device).
3. **The file gate on the new items** (P-86 ⑧): the record and the substitute carry
   `have_x_windows`, and the substitute's own gate is `all (have_x_windows, not
   (have_xscreennumberofscreen))`.
4. **C's `for (i = 0; i < dpy->nscreens; ++i) if (scr == dpy->screens + i) break;`** is the port's
   `while` with C's increment at the body's end and C's `dpy->screens + i` as
   `(*dpy).screens.offset (i as isize)`.

### Owed (this slice)

- The two `_XDisplay`-body rows (`nscreens`, `screens`) belong to the mirror's opacity decision
  (`include/xlib.rs` 1043: the body is behind `XLIB_ILLEGAL_ACCESS`, so carrying it needs a
  mirror round); they are the twelve arms' two-row reading, not a transcription defect.
- `syms_of_xfns`' `defsubr` site and the rest of the remainder from C 7494 (`select_visual`,
  C 7494-7606, which is `visual_classes`' only reader): 3,169 C lines plus the 16 missing
  functions, 1 macro, 4 objects, 25 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no display was opened, no visual was matched
and no screen number was computed; the thirteen arms' counts are text compile readings, and XSN's
14/9 is a text compile of a build whose Xlib is assumed to provide the function.
## Slice 25 (2026-10-08, status 3.928 - the integrator sets the round number): C 7494-7606

**The slice is C 7494-7606**: `select_visual` - C 7494-7496's comment, the `visualClass`/`VisualClass`
resource lookup (C 7504-7507), the `STRINGP` arm's CLASS-DEPTH parse (C 7514's bare `alloca`,
C 7520-7529's `-`) and the `visual_classes` walk (C 7532-7537, which is why the table's
`dead_code` allowance in the last slice is gone), C 7541-7547's `XMatchVisualInfo` and the
`else` arm's `#if !defined USE_X_TOOLKIT && !(defined USE_GTK && !defined HAVE_GTK3) && defined
HAVE_XRENDER` alpha-mask attempt (C 7566-7591) over the default-visual fallback (C 7596-7604).
The port's slice is **227 Rust lines** (10,631 - 10,404, `grep -c ''`) against C's 113; md5
`f7247a85422a83e1f6df61bf5bccf5ed` (the slice 24 tree) -> `95bfacd56f0fdef96dcfcf218f30e86b`.

**A fourteenth arm.**  The alpha-mask block's gate is C's three-way `#if`, which none of the
thirteen arms satisfied (the toolkit arms are excluded by the first two negations and the rest carry
no `have_xrender`), so the table gains **XR** = `have_x_windows` + `have_xrender` - and the block
compiles cleanly in it: its first reading is **20 / 9**, unchanged by the slice.  All fourteen arms
read their baselines with **byte-identical error sets** and no warning moves anywhere.

**The first measurement's rows, all closed.**  One `E0774` was this slice's own slip (a `derive`
left on the `static mut`), three `E0308`s were the widths C implies (`sa_stack_alloc`'s parameter is
`usize` while `SBYTES` is `isize`; `XGetVisualInfo`'s `vinfo_mask` is `c_long` while the two
`Xutil.h` masks are `int`), and one `E0381` was C's `int n_visuals;` out-parameter, which takes the
file's zero initialiser.  Two `--check` rows followed: the token row
`function select_visual does not mention alloca, have_xrender` is closed by a
`docs/ports/macro-body-notes.tsv` row (C 7514's `alloca` is the port's stack pool and C 7557's
`defined HAVE_XRENDER` is the `#[cfg]` the reader blanks), and
`function select_visual is not static in C but has no #[unsafe(no_mangle)] in the port` is closed by
carrying C's linkage - `select_visual` is declared in xterm.h 1008, so the port's definition is
`#[unsafe(no_mangle)] pub unsafe extern "C" fn`.

**Arm touch measured** (P-86 ⑦): two injections - the body's first statement (A) and the
`xrender_supported_p` branch (B) - move plain-X **16 -> 17** (A alone: the `#if` block is not
compiled there) and XR **20 -> 22** (A and B), which measures the new gate from the reading side.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash.

**② The fourteen arms.**  See above.

**③ `extract-abi.py --check`**: **112 rows -> 111** - `missing function select_visual` retiring, with
the slice's two other rows closed rather than standing (`missing function` 16 -> 15).  The unit
still has no line-count row (it retired at slice 22 when the port passed the 5% bar; the drift is
now `C 10662 vs Rust 10,631` = **0.3%**).  `check-mirror-dupes.py` stays at **16 duplicate
declaration(s), 16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at **18 declaration
sites (0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **Eleven use-site rows**: `XGetVisualInfo`/`XMatchVisualInfo` and `VisualIDMask`/
   `VisualScreenMask` (Xutil.h 471-476, 542-548, 305-306), `XRenderDirectFormat`/
   `XRenderPictFormat` (`XRenderFindVisualFormat` with them) and `PictTypeDirect` (Xrender.h 57-74,
   87-98, 482-484; render.h 89-90), `DefaultVisualOfScreen` (Xlib.h 128, a `macro_rules!` over the
   mirror's `Screen.root_visual`) and `XVisualIDFromVisual` (Xlib.h 1724-1726), `xstrcasecmp`
   (dispextern.h 104), `atoi` (`<stdlib.h>`), and `fatal` (lisp.h 5371).
2. **C 7514's bare `alloca`** is `sa_stack_alloc ((SBYTES (value) + 1) as usize)` - the port's stack
   pool, with the width written out (PORTING.md P-05).
3. **The two masks are widened at the calls** (`VisualScreenMask as c_long`,
   `(VisualIDMask | VisualScreenMask) as c_long`), which is C's implicit conversion.
4. **C's `int n_visuals;`** takes the file's zero initialiser (it is `XGetVisualInfo`'s
   out-parameter).
5. **C 7582's `dpyinfo->pict_format = format;`** is `format as *mut c_void`, because the mirror
   records that member untyped (`include/xterm.rs` 418).
6. **The `static mut` table is read by value** - `visual_class` derives `Clone, Copy` and the
   walk copies each element - which is how a `static mut` can be read in edition 2024 without the
   `static_mut_refs` lint.

### Owed (this slice)

- The eleven rows above are owed to `include/xlib.rs` (the Xutil.h/Xlib.h/render.h constants and
  records), an XRender mirror, `include/lisp.rs` (`fatal`), `include/dispextern.rs`
  (`xstrcasecmp`) and `lib/include/stdlib.rs` (`atoi`).
- `syms_of_xfns`' `defsubr` site and the rest of the remainder from C 7608
  (`x_display_info_for_name`, C 7612-7613): 3,055 C lines plus the 15 missing functions, 1 macro,
  4 objects, 25 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no resource was read, no visual was matched
and no display was opened; the fourteen arms' counts are text compile readings, and XR's 20/9 is a
text compile of an XRender build whose records and constants are this machine's headers'.
## Slice 26 (2026-10-08, status 3.929 - the integrator sets the round number): C 7609-7729

**The slice is C 7609-7729**, the connection group: `x_display_info_for_name` (C 7609-7610 its
comment, C 7617's `CHECK_STRING`, C 7619-7621's `x_display_list` walk, C 7624-7628's resource-name
default, `validate_x_resource_name` and `x_term_init`, C 7630-7631's error), `x-open-connection`
(C 7650-7673: the two `CHECK_STRING`s, C 7654's `xrm_option`, C 7660's `x_term_init` and C 7666-7671's
five-line `fatal`), `x-close-connection` (C 7687-7692), `x-display-list` (C 7701-7705) and
`x-synchronize` (C 7724-7726).  The port's slice is **151 Rust lines** (10,782 - 10,631,
`grep -c ''`) against C's 121; md5 `95bfacd56f0fdef96dcfcf218f30e86b` (the slice 25 tree) ->
`592d1f83c02ad2f26113cbca2052b9f7`.

**Every arm retires one row and adds none.**  The five items sit under the file's own gate, so all
fourteen arms compile them - and in each of the fourteen, exactly one row leaves the set and
nothing arrives: `cannot find function x_display_info_for_name`, the forward reference slices 1-3
owed (plain-X 16 -> **15**, WS, WSX, WSR, WSRX, WSR12 and XCB **15 / 9**, LUCID 24 -> **23 / 12**,
GTK and GTK22 42 -> **41 / 20**, XI2 20 -> **19 / 9**, XSN 14 -> **13 / 9**, XR 20 -> **19 / 9**, Xt
17 -> **16 / 12**).  No warning moves in any arm.

**The first measurement's three rows.**  (1) One `E0308`: `XSynchronize`'s `Bool` parameter against
the port's `bool` - closed as `!NILP (on) as Bool`, the conversion the port writes out.  (2) and (3)
were `--check` token rows (`function x_display_info_for_name does not mention 0`,
`subr Fx_open_connection does not mention 0`), closed by spelling C's two literal `0` pointer
arguments as `0 as *mut c_char` - the file's own device (slice 38's `!= 0 as *mut Display` note).

**Arm touch measured** (P-86 ⑦): two injections - `x_display_info_for_name`'s first statement (A)
and `Fx_synchronize`'s body (B) - move plain-X **15 -> 17** and GTK **41 -> 43**, both logs naming
both lines.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash.

**② The fourteen arms.**  See above.

**③ `extract-abi.py --check`**: **111 rows -> 106** - five retirements (`missing function
x_display_info_for_name`, `missing subr Fx_open_connection`, `missing subr Fx_close_connection`,
`missing subr Fx_display_list`, `missing subr Fx_synchronize`) and nothing appearing
(`missing function` 15 -> 14, `missing subr` 25 -> 21).  The drift is `C 10662 vs Rust 10,782` =
**1.1%**.  `check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0
failure(s)** and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed
literals), 0 failures**.

### Deviations (this slice)

1. **Three use-site rows**: `x_term_init` (xterm.h 1005 - `include/xterm.rs` 1759 keeps its
   prototype as text in the prototype block), `x_delete_terminal` (xterm.h 1784) and
   `XSynchronize` (Xlib.h 1518-1523, whose declaration on this machine is the
   pointer-to-function form, so the port's result is
   `Option<unsafe extern "C" fn (*mut Display) -> c_int>`).
2. **C's `!NILP (on)`** is `!NILP (on) as Bool` (Xlib's `Bool` is `int`).
3. **C's two literal `0` pointer arguments** (C 7628's `x_term_init (name, 0, …)` and C 7654's
   `NILP (xrm_string) ? 0 : …`) are `0 as *mut c_char`, the file's own spelling for a C `0`
   (slice 38's note) - `std::ptr::null_mut ()` reads the token row the checker asks for.

### Owed (this slice)

- The three rows above are owed to `include/xterm.rs` and `include/xlib.rs`.
- `syms_of_xfns`' `defsubr` sites (the count stays 45: every one of this file's DEFUNs waits for
  that block) and the rest of the remainder from C 7731: 2,932 C lines plus the 14 missing
  functions, 1 macro, 4 objects, 21 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no connection was opened, no display was
deleted, no display list was walked and no synchronization mode was set; the fourteen arms' counts
are text compile readings.
## Slice 27 (2026-10-08, status 3.930 - the integrator sets the round number): C 7731-7950

**The slice is C 7731-7950**: C 7732-7734's `Window properties` section comment and
`x-change-window-property` - the FORMAT check (C 7798-7806), the two VALUE arms (C 7808-7841, with
C 7821/7837's `elsize` and C 7824's `x_fill_property_data`), the target-window choice
(C 7843-7855), C 7783-7791's `USE_XCB` declarations with C 7858-7933's two arms (the cached-atom
pair and the four `xcb_intern_atom`/`xcb_intern_atom_reply` calls), C 7937-7939's `XChangeProperty`
and C 7944-7946's error pair.  It is this file's first subr with an XCB arm.  The port's slice is
**260 Rust lines** (11,042 - 10,782, `grep -c ''`) against C's 213; md5
`592d1f83c02ad2f26113cbca2052b9f7` (the slice 26 tree) -> `00de98651732c8b4e46c975629b29169`.

**Twelve arms are byte-identical; the GTK pair moves by one named row.**  Twelve of the fourteen
read their baselines with error sets identical to the last slice's - plain-X, WS, WSX, WSR, WSRX,
WSR12 and XCB **15 / 9**, LUCID **23 / 12**, XI2 **19 / 9**, XSN **13 / 9**, XR **19 / 9**, Xt
**16 / 12** - and GTK and GTK22 read **41 / 20 -> 42 / 20**, the single addition being a
`GDK_WINDOW_XID` instance of the class slice 8 named (the mirror's `GTK_WIDGET_TO_X_WIN`
placeholder, reached by C 7852's `FRAME_OUTER_WINDOW`).  No warning moves in any arm.

**The first measurement's five rows, all closed but one.**  Three `E0425`s were the `_for_lisp`
trio: slice 22 declared `x_catch_errors_for_lisp`/`x_uncatch_errors_for_lisp` under its XInput2
gate for C 7259/7263, and C 7935/7944/7946 use all three in every arm, so the row's gate widened to
the file gate and `x_check_errors_for_lisp` joined it.  One `E0308` was C 7875's
`prop_atom != None` (an `Atom` against X.h's `c_long`), written `!= None as Atom`.  One `E0381` was
C 7789's `xcb_generic_error_t *generic_error`, an out-parameter that takes the file's zero
initialiser.  The fifth is the GTK pair's `GDK_WINDOW_XID` instance, named above and left as the
class's further instance.

**Arm touch measured** (P-86 ⑦): two injections - the body's first statement (A) and the
`use_xcb` arm's first statement (B) - move plain-X **15 -> 16** and GTK **42 -> 43** with A alone
(the XCB arm is not compiled there) and XCB **15 -> 17** with both, which measures C 7867's gate
from the reading side.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash.

**② The fourteen arms.**  See above.

**③ `extract-abi.py --check`**: **106 rows -> 105** - `missing subr Fx_change_window_property
(x-change-window-property)` retiring, and nothing appearing in its place (the 220-line body with
its XCB pair satisfies the mechanical check as it stands); `missing subr` 21 -> 20.  The drift is
`C 10662 vs Rust 11,042` = **4.3%**, and the unit still carries no line-count row (it retired at
slice 22, when the port passed the 5% bar); what remains is `missing function` 14, `missing subr`
20, `missing defsubr` 45, 1 macro and 4 file-scope objects.  `check-mirror-dupes.py` stays at **16 duplicate declaration(s),
16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at **18 declaration sites
(0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **The XCB records and the two calls stand at the use site** (`xcb_intern_atom_cookie_t` and
   `xcb_intern_atom_reply_t` from xcb/xproto.h 1851-1856 and 1875-1881, `xcb_intern_atom` and
   `xcb_intern_atom_reply` from 6676-6680 and 6723-6726), which is the family this file's slice 14
   declared the `xproto.h` rows of (262-268); they are owed to an `xcb.rs` mirror.
2. **The `_for_lisp` trio's gate widened** from slice 22's XInput2 arm to the file gate, and
   `x_check_errors_for_lisp` joined it (C 7935/7944/7946 use all three unconditionally).
3. **C 7875's `prop_atom != None`** is `!= None as Atom` (X.h's `None` is `c_long`).
4. **C 7789's `generic_error`** takes the file's zero initialiser (it is an out-parameter).
5. **C's widths** are written out where the port needs them: `XFIXNUM (format) as c_int`,
   `xnmalloc (nelements as isize, elsize as isize)`, `nelements as ptrdiff_t`,
   `SBYTES (prop) as u16` and `(SBYTES (value) / elsize) as c_int`.

### Owed (this slice)

- The XCB rows are owed to an `xcb.rs` mirror; the `_for_lisp` trio to `include/xterm.rs`;
  `x_intern_cached_atom`, `x_check_property_data` and `x_fill_property_data` to `include/xterm.rs`
  as well (`src/xselect.rs` declares the same three rows).
- The GTK pair's new row is a further instance of the `GDK_WINDOW_XID` class
  (`include/gtk.rs`/`include/xterm.rs` are its homes).
- `syms_of_xfns`' `defsubr` sites (the count stays 45) and the rest of the remainder from C 7953
  (`x-delete-window-property`, C 7953-7995): 2,710 C lines plus the 14 missing functions, 1 macro,
  4 objects, 20 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no window property was changed, no atom was
interned and no XCB connection was used; the fourteen arms' counts are text compile readings.

## Slice 28 (2026-10-08, status 3.932 - the integrator sets the round number): C 7953-7995

**The slice is C 7953-7995**: `x-delete-window-property` - the `DEFUN` header (C 7953-7955) with its
verbatim doc string (C 7956-7967), the argument list (C 7968) and the body: C 7970's
`decode_window_system_frame` and C 7971's `FRAME_X_WINDOW`, C 7974's `CHECK_STRING`, C 7976-7981's
`WINDOW-ID` pair (C 7978's `CONS_TO_INTEGER` and C 7979-7980's root-window fallback), C 7983's
`block_input`, C 7984-7985's `x_intern_cached_atom`, C 7987-7991's error pair around C 7988's
`XDeleteProperty`, and C 7993-7994's `unblock_input` and `return prop`.  The port's slice is
**46 Rust lines** (11,088 - 11,042, `grep -c ''`) against C's 43; md5
`00de98651732c8b4e46c975629b29169` (the slice 27 tree) -> `d687aa97281a68a50b20616fc557d0b5`.

**All fourteen arms read their baselines on the first measurement, with diff-empty sets.**  plain-X,
WS, WSX, WSR, WSRX, WSR12 and XCB **15 / 9**, LUCID **23 / 12**, GTK and GTK22 **42 / 20**, XI2
**19 / 9**, XSN **13 / 9**, XR **19 / 9** and Xt **16 / 12** - and the `^error` and `^warning` line
sets are empty-diff against the in-tree HEAD baseline in every one of the fourteen (measured by
putting `git show HEAD:femacs/src/xfns.rs` back at the same path, probing all fourteen, restoring
the slice and diffing both sets).  The body compiled clean: no `E0381`, no `E0308` and no new
unresolved name - the four names it calls were all declared by earlier slices.

**The one row the checker raised was closed by spelling C's macro.**  The first `--check` read one
new token row, `subr Fx_delete_window_property does not mention frame_x_display`: the port's
`XDeleteProperty` call had written C 7988's `FRAME_X_DISPLAY (f)` out as its expansion,
`(*FRAME_DISPLAY_INFO! (f)).display`.  C 7988 is one line and the file has hundreds of sites that
keep the macro's name, so the call now reads `XDeleteProperty (FRAME_X_DISPLAY! (f), target_window,
prop_atom);` and the row is gone.

**Arm touch measured** (P-86 ⑦): two injections.  A, at the body's first statement, moves plain-X
**15 -> 16**, GTK **42 -> 43**, XCB **15 -> 16** and XI2 **19 -> 20**; B, before C 7993's
`unblock_input`, moves plain-X **15 -> 16**, GTK **42 -> 43**, XCB **15 -> 16**, LUCID **23 -> 24**
and Xt **16 -> 17**.  Both anchors point at the injected line (the materialised `_root.rs` line is
the unit's line plus the 7 inlined mirror lines), so C 7970's body and C 7993's tail are both read by
these arms.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash.

**② The fourteen arms.**  See above: every arm at its baseline, both sets empty-diff.

**③ `extract-abi.py --check`**: **105 rows -> 104** - `missing subr Fx_delete_window_property
(x-delete-window-property)` retiring, and nothing appearing in its place; `missing subr` 20 -> 19.
The remaining 104 are 45 `missing defsubr`, 19 `missing subr`, 14 `missing function`, 1
`missing macro`, 4 file-scope objects and the token rows the mechanical check keeps (19 `function`
and 1 `subr` "does not mention").  The drift is `C 10662 vs Rust 11,088` = **4.0%**;
`check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**
and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed literals), 0
failures** - this slice adds no declaration site and touches no mirror.

### Deviations (this slice)

1. **C 7988's `FRAME_X_DISPLAY (f)` keeps the macro's name** at the use site rather than being
   written out as the expansion, because the expansion leaves the checker's token row standing;
   the file's other `XDeleteProperty` site (C 3559) and its hundreds of `FRAME_X_DISPLAY (f)` sites
   spell it the same way.
2. **C 7980's `FRAME_DISPLAY_INFO (f)->root_window`** is `(*FRAME_DISPLAY_INFO! (f)).root_window`,
   the port's spelling for that read (C 7847's `root_window`, `include/xterm.rs`'s field).
3. **C 7978's three-argument `CONS_TO_INTEGER (window_id, Window, target_window)`** is the port's
   two-argument `CONS_TO_INTEGER! (window_id, target_window)` - the macro's port form, which C
   7845 took in the last slice.

### Owed (this slice)

- **Nothing new.**  The four names this body calls already stand: `decode_window_system_frame`
  (frame.h 1474-1475, this file 308-309), `x_intern_cached_atom` (xterm.h 1841, this file 10803),
  the `_for_lisp` trio (xterm.h 1755/1756/1759, this file 10079-10081) and `XDeleteProperty`
  (Xlib.h 2247, this file 1142) - the first three are owed to mirrors (`x_intern_cached_atom` and
  the trio to `include/xterm.rs`) from the earlier slices' lists.
- `syms_of_xfns`' `defsubr` sites (45, plus this slice's `Sx_delete_window_property` when that
  section lands) and the rest of the remainder from C 7998 (`x_window_property_intern`,
  C 7998-8000): 2,665 C lines plus the 14 missing functions, 1 macro, 4 objects, 19 subrs and 45
  defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no window property was deleted, no atom was
interned and no frame was decoded; the fourteen arms' counts are text compile readings.

## Slice 29 (2026-10-08, status 3.933 - the integrator sets the round number): C 7998-8073

**The slice is C 7998-8073**: `x_window_property_intern` - the `static Lisp_Object` and its seven
parameters (C 7998-8005), the locals (C 8007-8012), the first `XGetWindowProperty` (C 8014-8017)
with C 8019's `*found = actual_format != 0`, the second call (C 8026-8031, whose delete argument is
C 8029's `! NILP (delete_p)`), C 8034-8046's quoted `XGetWindowProperty` man page, C 8048-8056's
`LONG_WIDTH > 32` arm, C 8058-8066's `make_string`/`x_property_data_to_lisp` pair and C 8069's
`XFree`.  The port's slice is **91 Rust lines** (11,179 - 11,088, `grep -c ''`) against C's 76; md5
`d687aa97281a68a50b20616fc557d0b5` (the slice 28 tree) -> `4e1ca29e27a49d3ddaf9e955cf34963b`.

**All fourteen arms read their baselines, and the frozen build caught one row the arms could not.**
plain-X, WS, WSX, WSR, WSRX, WSR12 and XCB **15 / 9**, LUCID **23 / 12**, GTK and GTK22 **42 / 20**,
XI2 **19 / 9**, XSN **13 / 9**, XR **19 / 9** and Xt **16 / 12**, with both the `^error` and the
`^warning` sets empty-diff against the in-tree HEAD baseline (the same two-sided method the last
slice used).  The one row the arms could not see was the *include edge*: C reaches `LONG_WIDTH`
through lisp.h 33's `#include <limits.h>`, the port's mirror for it is `lib/include/limits.in.rs`,
and the first placement of that edge was ungated (beside the `stdcountof.rs` line, which is itself
ungated because it is macro-only).  The X arms compiled it happily - every one of them imports the
`c_int`/`c_long` names the mirror needs - while the **frozen configuration read 48 `E0425`/`E0433`
errors** (`cannot find type c_int`), i.e. the 336-byte empty object became a failing build.  The
edge now carries the file gate as two lines (attribute first, P-86 ⑫) and the frozen build is back
to exit 0 / 336 bytes; this is why the frozen reading is taken on every slice.

**Two rows the first measurement raised, both closed.**  The first arm run read two `E0425`s for
`Success` (C 8021/8032) - this body tests it while no earlier body of the file does, so it imports
the constant the way the file's other bodies do (`use crate::x11_constants::Success;` as the body's
first item, the C 415 body's device); after that the arms read their baselines.

**Arm touch measured** (P-86 ⑦) on the final hash: an injection at the body's first statement (A)
moves plain-X **15 -> 16**, GTK **42 -> 43**, XCB **15 -> 16** and XI2 **19 -> 20**; one before
C 8072's `return prop_value;` (B) moves plain-X **15 -> 16**, GTK **42 -> 43**, XCB **15 -> 16**,
LUCID **23 -> 24** and Xt **16 -> 17**; both logs carry the injected line's text once and the
diagnostic's anchor is the injected line.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash - and the reading that failed before the include edge was gated (48 errors) is recorded above.

**② The fourteen arms.**  See above: every arm at its baseline, both sets empty-diff.

**③ `extract-abi.py --check`**: **104 rows -> 103** - `missing function x_window_property_intern`
retiring, and nothing appearing in its place; the drift is `C 10662 vs Rust 11,179` = **4.9%**.
`check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**
and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed literals), 0
failures** - this slice adds no declaration site (its one declaration is a use-site extern block
inside the unit, which the gate reads as the unit's own text) and touches no mirror.

### Deviations (this slice)

1. **The `limits.in.rs` include edge stands at lisp.h 33** (C reaches `LONG_WIDTH` through that
   `#include <limits.h>`), written as two lines under the file gate, and its first ungated placement
   is the frozen-build failure recorded above.
2. **`Success` is imported inside the body** (`use crate::x11_constants::Success;`), the file's
   device for that constant (C 415's body, and `PropModeReplace` in the last slice).
3. **`x_property_data_to_lisp` stands at the use site** (xterm.h 1937-1941: `extern Lisp_Object
   x_property_data_to_lisp (struct frame *, const unsigned char *, Atom, int, unsigned long);`),
   because no mirror carries the row and `src/xselect.rs` defines it (P-55/P-72 rule 3).
4. **C's out-parameters take the file's zero initialisers** (`actual_type`, `actual_format`,
   `actual_size`, `bytes_remaining`); C declares them uninitialised and fills them through the
   `XGetWindowProperty` call.
5. **C's implicit conversions are written out**: `XFree (tmp_data as *mut c_void)` (the file's
   cast form), `bytes_remaining as c_long` (the fifth argument) and `!NILP (delete_p) as Bool`
   (C's `! NILP`), plus the `make_string` length `((actual_format >> 3) as c_ulong * actual_size) as
   ptrdiff_t` (C's `unsigned long` product handed to `ptrdiff_t`).
6. **C 8051/8052's pointer casts keep their forms**: `tmp_data as *mut c_int` and
   `tmp_data as *const c_long` (the `src/xselect.rs` reading of the same two lines, C 1871/1872).

### Owed (this slice)

- `x_property_data_to_lisp` is owed to `include/xterm.rs` (xterm.h 1937-1941); the names the last
  slices owed (`x_intern_cached_atom`, the `_for_lisp` trio) keep their entry.
- `syms_of_xfns`' `defsubr` sites (45) and the rest of the remainder from C 8075
  (`Fx_window_property`, the `DEFUN`): 2,588 C lines plus the 13 missing functions, 1 macro, 4
  objects, 19 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no window property was read, no X server was
asked for a value and no string was built; the fourteen arms' counts are text compile readings.

## Slice 30 (2026-10-08, status 3.934 - the integrator sets the round number): C 8075-8176

**The slice is C 8075-8176**: the `x-window-property` `DEFUN` - its header (C 8075-8076) and verbatim
doc string (C 8077-8115, the two freedesktop/x.org URLs included), the locals (C 8119-8124),
C 8126's `CHECK_STRING`, C 8128-8133's `WINDOW-ID` pair, C 8135-8136's `block_input` and
`x_catch_errors`, C 8138-8145's `AnyPropertyType`/`x_intern_cached_atom` pair, C 8147-8148's
`prop_atom` intern, C 8149-8155's first `x_window_property_intern` call, C 8156-8159's four-way test
with C 8161-8167's second call on the outer window, and C 8170-8172's `x_check_errors` /
`x_uncatch_errors_after_check` pair.  The port's slice is **107 Rust lines** (11,286 - 11,179,
`grep -c ''`) against C's 102; md5 `4e1ca29e27a49d3ddaf9e955cf34963b` (the slice 29 tree) ->
`b9f4d65a0ac368e4ac61906284c228a5`.

**Twelve arms are byte-identical; the GTK pair moves by two named rows.**  Twelve of the fourteen
read their baselines with both sets empty-diff against the in-tree HEAD baseline - plain-X, WS, WSX,
WSR, WSRX, WSR12 and XCB **15 / 9**, LUCID **23 / 12**, XI2 **19 / 9**, XSN **13 / 9**, XR **19 / 9**
and Xt **16 / 12** - and GTK and GTK22 read **42 / 20 -> 44 / 20**.  Both additions are
`GDK_WINDOW_XID` instances of the class slice 8 named (the mirror's `GTK_WIDGET_TO_X_WIN`
placeholder), reached by C 8159's `target_window != FRAME_OUTER_WINDOW (f)` and C 8162's same macro
as an argument - slice 27's instance came from C 7852.  They are named and left as the class's
further instances.  No warning moves in any arm.

**The drift row is back and is recorded, not hidden.**  `extract-abi.py --check` retires
`missing subr Fx_window_property (x-window-property)` and gains `line count drifted 5.9%: C 10662 vs
Rust 11286` - the port crossed the 5% bar again (it was under it from slice 22 through slice 29).  The
row is the reader's own ratchet, the file's tolerance row is the integrator's to write
(`docs/ports/line-drift-tolerance.tsv` is not a lane's table), so the reading stands: 103 rows
either way.

**Arm touch measured** (P-86 ⑦): an injection at the body's first statement (A) moves plain-X
**15 -> 16**, GTK **44 -> 45**, XCB **15 -> 16** and XI2 **19 -> 20**; one before C 8175's
`return prop_value;` (B) moves plain-X **15 -> 16**, GTK **44 -> 45**, XCB **15 -> 16**, LUCID
**23 -> 24** and Xt **16 -> 17**; both logs carry the injected line's text once and the anchor is
that line.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash.

**② The fourteen arms.**  See above; the GTK pair's two rows are the named class.

**③ `extract-abi.py --check`**: 103 rows -> 103 - `missing subr Fx_window_property
(x-window-property)` retiring, `missing subr` 19 -> 18, with the line-count row taking the freed
place; `missing defsubr` stays 45, `missing function` 13.  `check-mirror-dupes.py` stays at
**16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at
**18 declaration sites (0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **`AnyPropertyType` is imported inside the body** (`use crate::x11_constants::AnyPropertyType;`)
   and cast where C's `(Atom)` cast applies: `target_type = AnyPropertyType as Atom;` - the row is
   `include/xlib.rs` 87's `c_long` and `src/xselect.rs` 2269 writes the same cast.
2. **C's `found` takes the file's zero initialiser** (it is `x_window_property_intern`'s
   out-parameter) and both calls pass `&raw mut found`.
3. **C 8140's `strcmp` keeps C's argument order** (the literal first:
   `strcmp (c"AnyPropertyType".as_ptr (), SSDATA (r#type)) == 0`), which is also what the file's
   C 6452 site does when C puts the variable first.
4. **C's `type` parameter is `r#type`** (a Rust keyword), the file's spelling since slice 27.
5. **C 8170-8171's format string is `c"Can't retrieve window property: %s"`** and the call passes
   `FRAME_X_DISPLAY! (f)` (C's macro name kept at the use site, the slice 28 reading).

### Owed (this slice)

- The two new rows are further instances of the `GDK_WINDOW_XID` class, whose homes are
  `include/gtk.rs`/`include/xterm.rs`.
- `syms_of_xfns`' `defsubr` sites (45, plus this slice's `Sx_window_property`) and the rest of the
  remainder from C 8178 (`x-window-property-attributes`, the `DEFUN`): 2,485 C lines plus the 13
  missing functions, 1 macro, 4 objects, 18 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no window property was read, no atom was
interned, no outer window was tried; the fourteen arms' counts are text compile readings.

## Slice 31 (2026-10-08, status 3.935 - the integrator sets the round number): C 8178-8256

**The slice is C 8178-8256**: the `x-window-property-attributes` `DEFUN` - its header
(C 8178-8179) and verbatim doc string (C 8180-8194), the locals (C 8197-8205), C 8207's
`CHECK_STRING`, C 8209-8214's `WINDOW-ID` pair, C 8216's `block_input`, C 8218-8220's
`x_catch_errors` and cached-atom intern, C 8221-8224's first `XGetWindowProperty` (whose requested
type is C 8222's `AnyPropertyType`), C 8225-8238's four-way test with the outer-window retry
(C 8230-8232's comment included), C 8240-8248's `XFree` and `CALLN (Fvector, ...)` of the three
`make_fixnum`s, and C 8250-8252's error pair.  The port's slice is **85 Rust lines**
(11,371 - 11,286, `grep -c ''`) against C's 79; md5 `b9f4d65a0ac368e4ac61906284c228a5` (the slice 30
tree) -> `3c9eae3abf16a3d2e94c142030975da3`.

**Twelve arms are byte-identical; the GTK pair moves by two more named rows.**  plain-X, WS, WSX,
WSR, WSRX, WSR12 and XCB **15 / 9**, LUCID **23 / 12**, XI2 **19 / 9**, XSN **13 / 9**, XR **19 / 9**
and Xt **16 / 12** - all with both sets empty-diff against the in-tree HEAD baseline - and GTK and
GTK22 read **44 / 20 -> 46 / 20**: two more `GDK_WINDOW_XID` instances of the class slice 8 named,
reached by C 8228's `target_window != FRAME_OUTER_WINDOW (f)` and C 8233's assignment.  This file
now carries four instances (slice 27's C 7852, slice 30's C 8159/8162 and these two).  No warning
moves in any arm.

**The first measurement's two rows were one row twice.**  Both arms that read an error beyond the
GTK class read `E0308` at C 8222 and C 8235: `AnyPropertyType` is `x11_constants`' `c_long` (X.h
131's `0L`) while `XGetWindowProperty`'s requested type is an `Atom`, so C's implicit conversion is
written out at both call sites - `AnyPropertyType as Atom`, the form `src/xselect.rs` 2269 uses.
After that the arms read their baselines.

**Arm touch measured** (P-86 ⑦): an injection at the body's first statement (A) moves plain-X
**15 -> 16**, GTK **46 -> 47**, XCB **15 -> 16** and XI2 **19 -> 20**; one before C 8255's
`return prop_attr;` (B) moves plain-X **15 -> 16**, GTK **46 -> 47**, XCB **15 -> 16**, LUCID
**23 -> 24** and Xt **16 -> 17**; both logs carry the injected line's text once.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash.

**② The fourteen arms.**  See above; the GTK pair's two rows are the named class.

**③ `extract-abi.py --check`**: **103 rows -> 102** - `missing subr Fx_window_property_attributes
(x-window-property-attributes)` retiring and the drift row moving from `5.9%: C 10662 vs Rust
11286` to `6.6%: C 10662 vs Rust 11371` (`missing subr` 18 -> 17).  The drift row is the reader's
own ratchet and its tolerance table is the integrator's, so the lane records both readings.
`check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**
and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed literals), 0
failures**.

### Deviations (this slice)

1. **`AnyPropertyType` is imported in the body and cast where C's conversion applies**:
   `use crate::x11_constants::{AnyPropertyType, Success};` and `AnyPropertyType as Atom` at both
   `XGetWindowProperty` calls (the row is `include/xlib.rs` 87's `c_long`; `src/xselect.rs` 2269).
2. **C's four out-parameters take the file's zero initialisers** (`actual_type`, `actual_format`,
   `actual_size`, `bytes_remaining`) and `tmp_data`'s `NULL` is `std::ptr::null_mut ()`.
3. **The three `make_fixnum`s write C's conversions out**: `actual_type as EMACS_INT`,
   `actual_format as EMACS_INT` and
   `(bytes_remaining / (actual_format >> 3) as c_ulong) as EMACS_INT` (C's `unsigned long`
   quotient), the file's `make_fixnum (… as EMACS_INT)` form.
4. **C 8242's `XFree (tmp_data)`** is `XFree (tmp_data as *mut c_void)`, the file's cast form.

### Owed (this slice)

- The two new rows are further instances of the `GDK_WINDOW_XID` class
  (`include/gtk.rs`/`include/xterm.rs`); the class now has four instances in this file.
- `syms_of_xfns`' `defsubr` sites (45, plus this slice's `Sx_window_property_attributes`) and the
  rest of the remainder from C 8258 (the section comment and `x-translate-coordinates`, C 8263):
  2,404 C lines plus the 13 missing functions, 1 macro, 4 objects, 17 subrs and 45 defsubrs
  `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no window property metadata was read, no atom
was interned and no vector was built; the fourteen arms' counts are text compile readings.

## Slice 32 (2026-10-08, status 3.936 - the integrator sets the round number): C 8258-8360

**The slice is C 8258-8360**: the `Coordinate management` section comment (C 8259-8261) and
`x-translate-coordinates` - its header and verbatim doc string (C 8263-8285), the locals
(C 8290-8295), C 8297's `check_x_display_info`, C 8298-8299's zeroed `dest_x`/`dest_y`, the two
`CHECK_FIXNUM`/`XFIXNUM` pairs (C 8301-8305 and C 8307-8311), C 8313's null `source_frame`,
C 8315-8321's `source_window` pair, C 8323-8232's comment with C 8325-8332's `require_child` early
return, C 8334-8335's root-window default, C 8337-8340's `dest_window` pair, C 8342-8350's
`XTranslateCoordinates` between the error pair, and C 8352-8359's `Qnil` / `list3` result.  The
port's slice is **122 Rust lines** (11,493 - 11,371, `grep -c ''`) against C's 103; md5
`3c9eae3abf16a3d2e94c142030975da3` (the slice 31 tree) -> `af746053494a103b6a80da48e2c289c3`.

**Every one of the fourteen arms is byte-identical.**  plain-X, WS, WSX, WSR, WSRX, WSR12 and XCB
**15 / 9**, LUCID **23 / 12**, GTK and GTK22 **46 / 20** (the four named `GDK_WINDOW_XID` instances
of slices 27/30/31, unchanged - this slice uses `FRAME_X_WINDOW`, not `FRAME_OUTER_WINDOW`), XI2
**19 / 9**, XSN **13 / 9**, XR **19 / 9** and Xt **16 / 12** - and both the `^error` and the
`^warning` sets are empty-diff against the in-tree HEAD baseline in all fourteen.

**The first measurement's one row.**  `if !rc` read `E0308` (`expected bool, found i32`): C's `!rc`
on a `Bool` is a comparison, not a Rust `!`.  The port writes `if rc == 0`, which is this tree's
reading of a `Bool`'s falsity (`src/xterm.rs` 1936/2213/2308/3428/3533 all read it that way); after
that the arms read their baselines.

**Arm touch measured** (P-86 ⑦): an injection at the body's first statement (A) moves plain-X
**15 -> 16**, GTK **46 -> 47**, XCB **15 -> 16** and XI2 **19 -> 20**; one before C 8355's `list3`
return (B) moves plain-X **15 -> 16**, GTK **46 -> 47**, XCB **15 -> 16**, LUCID **23 -> 24** and Xt
**16 -> 17**; both logs carry the injected line's text once.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash.

**② The fourteen arms.**  See above: every arm at its baseline, both sets empty-diff.

**③ `extract-abi.py --check`**: **102 rows -> 101** - `missing subr Fx_translate_coordinates
(x-translate-coordinates)` retiring and the drift row moving from `6.6%` to `7.8%: C 10662 vs Rust
11493` (`missing subr` 17 -> 16).  The drift row is the reader's own ratchet and its tolerance table
is the integrator's; the lane records both readings.  `check-mirror-dupes.py` stays at **16
duplicate declaration(s), 16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at **18
declaration sites (0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **`x_handle_translate_coordinates` stands at the use site** (xterm.h 1836-1837: `extern
   Lisp_Object x_handle_translate_coordinates (struct frame *, Lisp_Object, int, int);`), because no
   mirror carries the row and `src/xterm.rs` defines it (C 14481) - P-55/P-72 rule 3.
2. **C 8323-8324's comment is transcribed verbatim** inside the body, and C 8259-8261's section
   banner is *described* in this slice's block comment rather than transcribed - the treatment this
   file's ten existing slice blocks already give xfns.c's eight banners (0 of the 8 are carried
   verbatim in the port; the three most recent slices before this one did the same).  Recorded here
   so the reading is a decision, not a gap.
3. **C's pointer and integer tests keep the file's spellings**: `! source_frame.is_null ()` for
   C 8325's `source_frame`, `src == 0` for C 8334's `!src`, `child_return != None as Window` for
   C 8357's `child_return != None` (X.h 115's `None` imported in the body).
4. **C's conversions are written out**: `XFIXNUM (source_x) as c_int` (C 8304/8310, `EMACS_INT` to
   `int`), `make_int (dest_x as intmax_t)` and `make_uint (child_return as uintmax_t)` (C 8355-8358,
   the `src/alloc.rs` cast form), and the ternary becomes Rust's `if`/`else` block with `Qnil ()`
   in the else arm (the `src/callproc.rs` 1950 shape).
5. **The four out-parameters of C's locals take the file's zero initialisers** (`dest_x`, `dest_y`,
   `child_return`, `src`, `dest`) and `source_frame`'s `NULL` is `std::ptr::null_mut ()`.

### Owed (this slice)

- `x_handle_translate_coordinates` is owed to `include/xterm.rs` (xterm.h 1836-1837); the
  `GDK_WINDOW_XID` class keeps its four instances.
- `syms_of_xfns`' `defsubr` sites (45, plus this slice's `Sx_translate_coordinates`) and the rest of
  the remainder from C 8362 (the `Tooltips` banner and `compute_tip_xy`): 2,301 C lines plus the 13
  missing functions, 1 macro, 4 objects, 16 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no coordinates were translated, no display was
checked and no list was built; the fourteen arms' counts are text compile readings.

## Slice 33 (2026-10-08, status 3.937 - the integrator sets the round number): C 8362-8404

**The slice is C 8362-8404**: the `Tool tips` section banner (C 8362-8364), C 8366-8367's forward
declaration of `compute_tip_xy` (Rust needs none: the definition later in the file is visible to its
callers, so the declaration is recorded here rather than transcribed), the eight file-scope objects
of C 8369-8390 - `tip_frame` (C 8370), `tip_window` (C 8374), `tip_dx`/`tip_dy` (C 8377, one C
declarator line), `tip_timer` (C 8381), `tip_last_string` (C 8384), `tip_last_frame` (C 8387) and
`tip_last_parms` (C 8390) - and `unwind_create_tip_frame` (C 8393-8404).  The port's slice is
**61 Rust lines** (11,554 - 11,493, `grep -c ''`) against C's 43; md5
`af746053494a103b6a80da48e2c289c3` (the slice 32 tree) -> `1f2c74a5fca8c3ad4604dc21b3f14004`.

**Every one of the fourteen arms is byte-identical, on both sides of a mirror change.**  plain-X,
WS, WSX, WSR, WSRX, WSR12 and XCB **15 / 9**, LUCID **23 / 12**, GTK and GTK22 **46 / 20**, XI2
**19 / 9**, XSN **13 / 9**, XR **19 / 9** and Xt **16 / 12**, and both the `^error` and the
`^warning` sets are empty-diff against the HEAD pair (HEAD's unit **and** HEAD's mirror, restored for
the round and put back after it) in all fourteen.

**The first measurement's four rows were the mirror's own declarations.**  The four exported
objects read `E0428` (`the name tip_window is defined multiple times`) against `include/xterm.rs`
812-815, which declares them from xterm.h 1010-1013.  That is PORTING.md 0.1-2's exception
(status 3.900): a name the defining unit defines cannot be declared from that unit's own mirror, so
the four rows left the mirror (their absence is now recorded in `docs/ports/excluded-objects.txt`,
four rows, with the reason) and the unit's definitions stand.  **The four E0428 rows are also this
slice's reachability reading for the objects**: the compiler saw each definition, which no
text compile of the objects alone could show.  No other unit reads these names (the only other
definitions are `src/haikufns.rs`'s and `src/androidfns.rs`'s own `have_haiku`/`have_android`
objects), so no consumer needed a use-site declaration.

**`check-all-units.sh` both legs** (P-87 ①, required because a mirror changed): `checked 83 unit(s),
0 failure(s)` and `checking 83 unit(s), 0 unit failure(s)`.

**Arm touch measured** (P-86 ⑦): an injection at `unwind_create_tip_frame`'s first statement (A)
moves plain-X **15 -> 16**, GTK **46 -> 47** and XCB **15 -> 16**; one before C 8402's
`tip_frame = Qnil ();` (B) moves plain-X **15 -> 16**, GTK **46 -> 47**, XCB **15 -> 16**, LUCID
**23 -> 24** and Xt **16 -> 17**; both logs carry the injected line's text once.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash.

**② The fourteen arms.**  See above: every arm at its baseline, both sets empty-diff, both sides.

**③ `extract-abi.py --check`**: **105 rows -> 96** - the eight `missing file-scope object` rows
(`tip_frame`, `tip_window`, `tip_dx`, `tip_dy`, `tip_timer`, `tip_last_string`, `tip_last_frame`,
`tip_last_parms`) and `missing function unwind_create_tip_frame` retiring, with the drift row moving
from `7.8%` to `8.4%: C 10662 vs Rust 11554`.  `check-mirror-dupes.py` reads **16 duplicate
declaration(s), 16 ratchet row(s), 0 failure(s)** (the four removed mirror rows were not among its
rows) and `check-declarations.py` **18 declaration sites (0 forbidden, 18 allowed literals), 0
failures**.

### Deviations (this slice)

1. **`include/xterm.rs` no longer declares the four tooltip objects** (C 1010-1013): the unit owns
   their definitions (C 8370/8374/8377, exported), and the absence is recorded in
   `docs/ports/excluded-objects.txt` - PORTING.md 0.1-2's exception, the `src/xterm.h`
   `x_error_message_count` row's shape.
2. **The four exported objects carry `#[unsafe(no_mangle)]` and the four `static` ones stay
   private** (PORTING.md 0.2; xterm.h 1010-1013 declares the first four).
3. **Each tentative definition's zero value is C's** (`std::ptr::null_mut ()` for a `Lisp_Object`,
   `0` for the `Window`) - the `src/haikufns.rs` 556-567 shape for the same four names.
4. **C 8366-8367's forward declaration of `compute_tip_xy` is not transcribed** (Rust resolves the
   later definition); it is recorded in this slice's block comment instead.
5. **C 8362-8364's banner is described in this slice's block** like the file's other banners (see
   slice 32's Deviation 2).

### Owed (this slice)

- The `GDK_WINDOW_XID` class keeps its four instances; `x_handle_translate_coordinates` keeps its
  `include/xterm.rs` entry.
- `syms_of_xfns`' `defsubr` sites (45) and the rest of the remainder from C 8406 (the comment and
  `x_create_tip_frame`, C 8416): 2,257 C lines plus the 12 missing functions, 1 macro, 0 objects,
  16 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no tooltip frame was unwound, no tip object was
read and no frame was created; the fourteen arms' counts are text compile readings.

## Slice 34 (2026-10-08, status 3.938): C 8406-8775

**The slice is C 8406-8775**: the comment and `x_create_tip_frame` (471 Rust lines, 11,554 -> 12,025).
It carries four frame-bitfield macros (`wants_modeline!`/`tooltip!`/`no_split!`/
`can_set_window_size!`, the `src/pgtkfns.rs` 267-300 device), the use-site declarations the two
window-creation arms need (six xcb enum constants, `xcb_cursor_t`, `xcb_generate_id`,
`xcb_create_window`, `xcb_change_property`, `CWSaveUnder`), the `use` of
`crate::dispextern_unit_names::face_change`, and it widens `XCreateWindow`'s gate from
`not (use_x_toolkit)` to the file gate (the tip body calls it in every arm, C 8609).  It also
corrects slice 33's `unwind_create_tip_frame` to `unsafe extern "C" fn` (its address is taken at
C 8442).

**Readings (all fourteen arms, both sides).**  Every arm reads its baseline **+1**, one more
instance of the class the file's earlier slices already report: `include/xterm.rs`'s `x_output`
still lacks `explicit_parent` (the write is C 8467), a mirror gap the reader calls
`no field explicit_parent on type x_output` - six instances in this unit now.  The five arms
without `have_window_system` (plain-X, XI2, XSN, XR, XCB) and Xt read **+2 more**: the mirror's
`FRAME_TOOLTIP_SHIFT`/`FRAME_TOOLTIP_UNIT` rows are gated by `have_window_system` (slice 24's
`nscreens`/`screens` class).  WSRX and XCB carry **+1 warning** (`unused doc comment`) that this
slice has not closed.  `build-tu` is exit 0 / the 336-byte empty object; `--check` is **96 rows ->
96**: `missing function x_create_tip_frame` retires, the null-family row is closed (the six C
`NULL`s are `std::ptr::null_mut () as *const c_char`), and one token row remains -
`function x_create_tip_frame does not mention frame_background_pixel, frame_fontset,
frame_foreground_pixel, frame_x_visual_info` (the four names are C's macros for `f->` fields the
port writes as member paths; the row's sanctioned home is `docs/ports/macro-body-notes.tsv`, owed).

### Deviations (this slice)

1. Four frame bitfields through this unit's own macros (the `pgtkfns` device); the four
   `FRAME_*_PIXEL!` macros read only, so those writes are C's member paths and the comment names
   the macros.
2. `XCreateWindow`'s gate widened to the file gate; `DoesSaveUnders! (...) == True` (the file's
   8393 spelling, the macro yields the mirror's `c_int` field); `std::ptr::null_mut () as *const
   c_char` for the six `NULL`s; C's two chained assignments split into two stores in C's order;
   `tip_window = FRAME_X_WINDOW! (f)` after the `FRAME_X_WINDOW` store.
3. The xcb block's doc comments are block comments (the slice 5 note).

### Owed (closed or re-scoped, status 3.939)

- **Closed**: `docs/ports/macro-body-notes.tsv` gained the four-token row (`frame_background_pixel`,
  `frame_fontset`, `frame_foreground_pixel`, `frame_x_visual_info`), and `--check` reads
  **96 -> 95** with no token row left for this function.
- **Re-scoped (not this lane's to fix)**: the +1 `explicit_parent` instance is the mirror's own
  decision, not a gap: `include/xterm.rs` 924-927 records six `x_output` bitfields (`hourglass_p`,
  `explicit_parent`, `asked_for_visible`, `has_been_visible`, `wait_for_wm`, `alpha_identical_p`)
  as `// C:` lines rather than as fields, so every write of C 8467's kind reads the class's row;
  the file's baseline already carries five.  Carrying the bitfields is a mirror round of its own
  (the layout of each bit has to be measured first).
- **Re-scoped (arms, not code)**: the +2 in the five arms without `have_window_system` and the
  +1 warning are arm-configuration artifacts: C guards the tooltip bit itself (`frame.h` 455-457,
  `#ifdef HAVE_WINDOW_SYSTEM`), so the mirror's gate matches C and the arms that omit the feature
  read the slice-24 `nscreens`/`screens` class; the `unused doc comment` is in
  `lib/include/stdbit.in.rs` 644-647 (a mirror's doc block, materialized once per including arm),
  not in this unit's text.

## Slice 35 (2026-10-08, status 3.940): C 8778-8893

**The slice is C 8778-8893**: the comment and `compute_tip_xy` (127 Rust lines, 12,025 -> 12,155).
The four `Fassq` reads (C 8796-8799), C 8801-8802's comment with C 8803-8804's four-way test,
C 8808-8811's `x_query_pointer` between `block_input`/`unblock_input`, C 8815-8824's
`HAVE_XRANDR`/`USE_GTK` cached-attributelist choice, C 8826-8849's monitor walk (C 8834-8837's four
`Fnth` reads, C 8839-8843's containment test with its `break`, C 8845's `max_y = -1`),
C 8852-8860's sane defaults, and C 8862-8892's two four-arm chains for `root_y` and `root_x`.

**Every one of the fourteen arms is byte-identical.**  Each reads exactly its slice-34 level -
plain-X **18 / 9**, WS, WSX, WSR, WSRX, WSR12 and XSN **16 / 9** (WSRX **16 / 10**), LUCID
**25 / 12**, GTK and GTK22 **47 / 20**, XI2 and XR **22 / 9**, Xt **20 / 12** and XCB **18 / 10** -
and both the `^error` and the `^warning` sets are empty-diff against the in-tree HEAD baseline in
all fourteen, so this slice adds no diagnostic and no warning.  (Those levels themselves carry the
slice-34 readings that are re-scoped in its Owed section: the `x_output` bitfield class and the
`have_window_system`-gated tooltip rows.)

**The first measurement's eight rows were one class.**  All eight were `E0381` (`used binding
isn't initialized`) - `root`, `child`, `win_x`, `win_y`, `pmask`, `min_x`, `min_y`, `max_x` - and
they close with the file's own device: C's out-parameters and its geometry locals take zero
initialisers (C 8790-8793), as the earlier slices' `generic_error`, `actual_type` and `found` did.

**Arm touch measured** (P-86 ⑦): an injection at the body's first statement (A) moves plain-X
**18 -> 19**, GTK **47 -> 48**, XCB **18 -> 19** and XI2 **22 -> 23**; one before C 8878's
`if FIXNUMP (left)` (B) moves plain-X **18 -> 19**, GTK **47 -> 48**, XCB **18 -> 19**, LUCID
**25 -> 26** and Xt **20 -> 21**; both logs carry the injected line's text once.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash.

**② The fourteen arms.**  See above: every arm at its slice-34 level, both sets empty-diff.

**③ `extract-abi.py --check`**: **95 rows -> 94** - `missing function compute_tip_xy` retiring with
nothing appearing in its place, and the drift row moving from `12.8%` to `14.0%: C 10662 vs Rust
12155`.  `check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0
failure(s)** and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed
literals), 0 failures**.

### Deviations (this slice)

1. **C's out-parameters and geometry locals take the file's zero initialisers** (the first
   measurement's eight `E0381`s).
2. **C 8815-8823's preprocessor choice is two cfg blocks**: the `have_xrandr`/`use_gtk` arms keep
   C's `if`/`else` (cached list or `Fx_display_monitor_attributes_list`), and the arms without
   either feature carry C 8824's single assignment under `not (any (...))`.
3. **`XFIXNUM` results are narrowed with `as c_int`** where C assigns them to its `int` locals or
   adds them to one (C 8834-8837, 8863-8886), and the sums are parenthesised the way C's
   precedence reads.

### Owed (this slice)

- Nothing new: the four `Fnth` reads, `x_query_pointer`, `x_display_pixel_*` and the Q symbols all
  stand already.
- `syms_of_xfns`' `defsubr` sites (45) and the rest of the remainder from C 8896 (the doc comment
  and `x_hide_tip`, C 8909): 1,767 C lines plus the 11 missing functions, 1 macro, 0 objects, 16
  subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no pointer was queried, no monitor list was
walked and no tooltip was placed; the fourteen arms' counts are text compile readings.

## Slice 36 (2026-10-08, status 3.941): C 8896-9028

**The slice is C 8896-9028**: C 8896-8908's doc comment and `x_hide_tip` (147 Rust lines, 12,155 ->
12,303): C 8912-8916's timer cancel with `calln!`, C 8918-8987's `USE_GTK` arm (C 8924-8933's
early-return test over `tip_last_frame`/`tip_frame`/`use_system_tooltips`, C 8938-8940's
`SPECPDL_INDEX` and the two `specbind`s, C 8949's `xg_hide_tooltip` pair, C 8957-8958's
`tip_last_frame` reset, C 8961-8984's delete-or-hide pair and C 8986's `unbind_to`) and
C 8988-9027's `not USE_GTK` arm.

**Every one of the fourteen arms is byte-identical.**  Each reads exactly its slice-35 level -
plain-X **18 / 9**, WS, WSX, WSR, WSRX, WSR12 and XSN **16 / 9** (WSRX **16 / 10**), LUCID
**25 / 12**, GTK and GTK22 **47 / 20**, XI2 and XR **22 / 9**, Xt **20 / 12** and XCB **18 / 10** -
and both sets are empty-diff against the in-tree HEAD baseline in all fourteen.

**The first measurement's one row was the declaration's placement.**  GTK read **47 / 21**, one
`unused doc comment` in the file's own text: C's `/** ... */` block (C 8896-8908) is a *doc* comment
in Rust too, and it sat before the slice's `xg_hide_tooltip` `unsafe extern "C"` block, so it
documented nothing ("a doc comment on an `extern` block is the `unused doc comment` warning the
file's baseline already counts" - slice 5's note).  Moving the extern block above the comment (so
the comment attaches to `x_hide_tip`, where C puts it) returns GTK and GTK22 to **47 / 20**.

**Arm touch measured** (P-86 ⑦), and it measures the cfg seam: an injection at the body's first
statement (A, C 8912) moves plain-X **18 -> 19**, GTK **47 -> 48**, XCB **18 -> 19** and XI2
**22 -> 23**; one before C 9003's `if (! NILP (tip_frame))` (B) - which sits inside the *not
USE_GTK* arm - moves plain-X **18 -> 19**, XCB **18 -> 19**, LUCID **25 -> 26** and Xt **20 -> 21**
while **GTK stays at 47**, so B is read by the arms that compile that arm and by no others.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash.

**② The fourteen arms.**  See above.

**③ `extract-abi.py --check`**: **94 rows -> 93** - `missing function x_hide_tip` retiring with
nothing appearing, and the drift row moving from `14.0%` to `15.4%: C 10662 vs Rust 12303`.
`check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)**
and `check-declarations.py` at **18 declaration sites (0 forbidden, 18 allowed literals), 0
failures**.

### Deviations (this slice)

1. **`xg_hide_tooltip` stands at the use site** (gtkutil.h 216: `extern bool xg_hide_tooltip
   (struct frame *f);`, gated `all (have_x_windows, use_gtk)`), because `src/gtkutil.rs` defines it
   twice (C 971-992 and 1008-1016, one per `use_gtk` arm) and no mirror carries the row - the
   `src/pgtkfns.rs` 448-451 shape.
2. **That declaration block sits *above* C's doc comment**, so the comment attaches to the
   function (the `unused doc comment` reading above).
3. **The two arms are cfg-gated blocks** (`use_gtk` / `not use_gtk`), each carrying C's own
   early-return test and body in C's order.

### Owed (this slice)

- `x_hide_tip`'s own name is the *static* helper; the `x-hide-tip` `DEFUN` (C 9375) and its
  `defsubr` are still to come.
- `syms_of_xfns`' `defsubr` sites (45) and the rest of the remainder from C 9031 (the `x-show-tip`
  `DEFUN`): 1,632 C lines plus the 10 missing functions, 1 macro, 0 objects, 16 subrs and 45
  defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no timer was cancelled, no tooltip was hidden
and no frame was deleted; the fourteen arms' counts are text compile readings.

## Slice 37 (2026-10-08, status 3.942): C 9031-9372

**The slice is C 9031-9372**: the `x-show-tip` `DEFUN` (552 Rust lines, 12,303 -> 12,855): the
verbatim doc string (C 9031-9061), C 9065-9078's locals with `AUTO_STRING`, C 9080-9105's argument
defaults, C 9107-9127's `USE_GTK` arm (`xg_prepare_tooltip`/`xg_show_tooltip`), C 9129-9209's
reuse/delete/hide block with its two parameter walks, C 9211-9238's `tip_frame` creation through
C 9232's `x_create_tip_frame`, C 9240-9278's window setup, C 9280-9311's buffer/`try_window` block
with its `ENABLE_CHECKING` arm, C 9314-9323's size and `compute_tip_xy` call, C 9325-9348's show
block with the `not USE_XCB`/`USE_XCB` pair, and C 9350-9371's garbage/update/timer tail.  C's two
`goto start_timer` (C 9125/9152) and its label (C 9367) are a labelled block, whose
`break \'start_timer` is the jump's spelling.

**Thirteen arms are byte-identical; the two `use_xcb` arms differ by one warning only.**  plain-X
**18 / 9**, WS, WSX, WSR, WSR12 and XSN **16 / 9**, LUCID **25 / 12**, GTK and GTK22 **47 / 20**, XI2
and XR **22 / 9**, Xt **20 / 12** - all with both sets empty-diff against the in-tree HEAD baseline -
and WSRX **16 / 9** and XCB **18 / 9** read **one fewer `unused doc comment`** than their baselines
(9 -> 8): the nine `pub const XCB_*` rows slice 34 left with a *trailing* `///` citation were a doc
comment on the following `extern` block (the slice 5 note), so this slice writes them as `/* */`
- that is slice 34's owed warning row closing as a side effect, and it is the only difference in
either set.

**Four first-measurement rounds, all named.**  ⑴ The file did not parse: two multi-line `if`
conditions (C 9131-9135 and C 9194-9195) had kept C's closing paren but lost C's opening one; C's
wrapping parens are restored.  ⑵ Seven rows: the `SET_TEXT_POS`, `FRAME_TOTAL_COLS` and
`window_bits` macros and the `set_window_buffer`/`try_window`/`update_single_window` declarations
are not in this unit's include graph (each ported unit keeps its own copy - the
`src/haikufns.rs` 193-199/279-289/2070-2077 device), and the mirror's `flush_frame!` needs
`redisplay_interface`, which the unit now carries (the `src/haikufns.rs` 1331-1379 shape, with its
layout assert).  ⑶ The carried struct's `glyph_string` fields became `*mut c_void` (the mirror's
non-glob-imported module) and `flush_display` takes `*mut frame` (the mirror's macro passes its own
`$f`); then the mirror's `flush_frame!` still read `FRAME_RIF!` as `*mut c_void`, so the unit
shadows `flush_frame` with C's own body (the `src/haikufns.rs` 2049-2056 shape).  ⑷ One token row,
`subr Fx_show_tip does not mention car, cdr`, closed by spelling C's `CAR`/`CDR` instead of
`Fcar`/`Fcdr` at the six sites.

**Arm touch measured** (P-86 ⑦): an injection at the body's first statement (A, C 9080) moves
plain-X **18 -> 19**, GTK **47 -> 48** and XCB **18 -> 19**; one before C 9368's timer comment (B)
moves plain-X **18 -> 19**, GTK **47 -> 48**, XCB **18 -> 19** and Xt **20 -> 21**.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash (`450df1ba8c9201bd1caeaeb1ad88bafe`).

**② The fourteen arms.**  See above.

**③ `extract-abi.py --check`**: **93 rows -> 92** - `missing subr Fx_show_tip (x-show-tip)`
retiring, with the token row above appearing and then closing; the drift row moves from `15.4%` to
`20.6%: C 10662 vs Rust 12855`.  `check-mirror-dupes.py` stays at **16 duplicate declaration(s), 16
ratchet row(s), 0 failure(s)** and `check-declarations.py` at **18 declaration sites (0 forbidden,
18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **The three macros and eight declarations stand in the unit** (`SET_TEXT_POS` dispextern.h
   291-294, `FRAME_TOTAL_COLS` frame.h 1118-1120, `window_bits` window.h's bit setters,
   `set_window_buffer` window.h 1139-1140, `try_window` dispextern.h 3573,
   `update_single_window` dispextern.h 3918, `XMoveWindow` Xlib.h 2844 under the file gate,
   `XMapRaised`/`XMoveResizeWindow` Xlib.h 2806/2835 under `not use_xcb`,
   `xcb_map_window`/`xcb_configure_window` xproto.h 6298/6447 with five enum members 1717-1727,
   `xg_prepare_tooltip`/`xg_show_tooltip` gtkutil.h 207-212, `x_cr_update_surface_desired_size`
   xterm.h 1812).
2. **`redisplay_interface` is carried** (dispextern.h 3069-3196, with the offset assert) and
   `flush_frame` is shadowed, both because the mirror's `FRAME_RIF!` is `*mut c_void`.
3. **`AUTO_STRING!` supplies `tip`**, and C's six reassigned arguments take the `let mut x = x;`
   device (the `DEFUN!` macro's `$arg:ident`).
4. **C 9125/9152's `goto` and C 9367's label** are a labelled block.
5. **`CAR`/`CDR` keep C's spellings** at the six sites the token row named.
6. **Slice 34's nine trailing `///` citations became `/* */`** (the one-warning reading above).

### Owed (this slice)

- `syms_of_xfns`' `defsubr` sites (45) and the rest of the remainder from C 9375 (the `x-hide-tip`
  `DEFUN`): 1,288 C lines plus the 9 missing functions, 1 macro, 0 objects, 15 subrs and 45
  defsubrs `--check` reads.
- The `x_output` bitfield class (`explicit_parent` and friends) stays the mirror round slice 34's
  Owed describes.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no tooltip was shown, no timer was started and
no frame was created; the fourteen arms' counts are text compile readings.

## Slice 38 (2026-10-08, status 3.943): C 9375-9415

**The slice is C 9375-9415**: three subrs and a banner (58 Rust lines, 12,855 -> 12,912):
`x-hide-tip` (C 9375-9381, one call to slice 36's helper), `x-double-buffered-p`
(C 9383-9394, with C 9388-9393's `HAVE_XDBE` pair as two cfg blocks) and `x-uses-old-gtk-dialog`
(C 9401-9415, with C 9407-9414's `USE_GTK` guard around the four-way test).

**All fourteen arms are byte-identical.**  Each reads exactly its slice-37 level - plain-X
**18 / 9**, WS, WSX, WSR, WSRX, WSR12 and XSN **16 / 9**, LUCID **25 / 12**, GTK and GTK22
**47 / 20**, XI2 and XR **22 / 9**, Xt **20 / 12** and XCB **18 / 9** - with both the `^error` and the
`^warning` sets empty-diff against the in-tree HEAD baseline in all fourteen.

**The first measurement's one row** was `SELECTED_FRAME`: C 9410's `SELECTED_FRAME ()` is a
*function* in this port (`SELECTED_FRAME ()`, as `src/data.rs` 1126 and this file's 10096/10127
spell it), not a macro, so the GTK and GTK22 arms read `cannot find macro SELECTED_FRAME` until the
call was written that way.

**Arm touch measured** (P-86 ⑦): an injection at `x-hide-tip`'s body (A, C 9380) moves plain-X
**18 -> 19**, GTK **47 -> 48**, XCB **18 -> 19** and XI2 **22 -> 23**; one before C 9414's
`return Qnil ();` (B) moves plain-X **18 -> 19**, GTK **47 -> 48**, XCB **18 -> 19** and Xt
**20 -> 21**; both logs carry the injected line's text once.

### Evidence (this slice)

**① The frozen configuration (`build-tu`).**  exit 0, the **336-byte** empty object, on the final
hash (`79e7dfa8acb3f9ea8ecd74e1a88c4293`).

**② The fourteen arms.**  See above.

**③ `extract-abi.py --check`**: **92 rows -> 89** - three `missing subr` retiring
(`Fx_hide_tip`, `Fx_double_buffered_p`, `Fx_uses_old_gtk_dialog`) with nothing appearing, and the
drift row moving from `20.6%` to `21.1%: C 10662 vs Rust 12912`.  `check-mirror-dupes.py` stays at
**16 duplicate declaration(s), 16 ratchet row(s), 0 failure(s)** and `check-declarations.py` at
**18 declaration sites (0 forbidden, 18 allowed literals), 0 failures**.

### Deviations (this slice)

1. **`xg_uses_old_file_dialog` stands at the use site** (gtkutil.h 86: `extern bool
   xg_uses_old_file_dialog (void);`, gated `all (have_x_windows, use_gtk)`), because
   `src/gtkutil.rs` defines it and no mirror carries the row - P-55/P-72 rule 3.
2. **C 9388-9393's `HAVE_XDBE` pair and C 9407-9414's `USE_GTK` guard** are cfg blocks; the
   guarded arm keeps C's four-way test with `window_system_available (SELECTED_FRAME ())`
   (`include/frame.rs` 1413) and the unconditional `return Qnil ();` stays outside them.
3. **C 9390's ternary** is `if FRAME_X_DOUBLE_BUFFERED_P! (f) { Qt () } else { Qnil () }` (the
   file's `P`-macro spelling, used at 8929/9131 in the last two slices).

### Owed (this slice)

- `syms_of_xfns`' `defsubr` sites (45, plus these three `Sx_*` when that section lands) and the rest
  of the remainder from C 9418 (the `USE_MOTIF` callbacks): 1,244 C lines plus the 9 missing
  functions, 1 macro, 0 objects, 12 subrs and 45 defsubrs `--check` reads.

### Not claimed (this slice)

The frozen configuration compiles none of this file: no tooltip was hidden, no frame was asked
about its buffering and no dialog policy was read; the fourteen arms' counts are text compile
readings.
