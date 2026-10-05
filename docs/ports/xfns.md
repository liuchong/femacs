# emacs/src/xfns.c (10662 lines) — slices: C 1-802 and C 803-2149

> **P-86's first X11 unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 7521 fills `WINDOW_SYSTEM_OBJ="fontset.o fringe.o image.o"` only under
> `HAVE_WINDOW_SYSTEM`, that macro is `undef` in
> `docs/baseline/config/darwin-arm64.macros.tsv`, and `docs/baseline/frozen-build-sources.txt`
> lists no `xfns.o`, so `extract-abi.py`'s checklist for the file is empty and its checkbox
> sections cannot be coverage evidence.  What follows is P-86 ②'s substitute: the **C line-range
> table** for this slice, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/xfns.rs` (**1,120 lines** after slice 1, **3,051 lines** after the second slice -
> C 803-2149, whose own ledger section is at the end of this file - measured with `grep -c ''`),
> both arms of every `#if` carried.
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
| 2150-10662 | **the remainder** (C 10662 is the reference's last line): the file's other 8,513 lines, whose first live definition is `x_encode_text` (C 2167 is its `static unsigned char *` line, C 2168 the name) - the next slice's first item | not ported in this slice |

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

**⑦ The other gates** (all run at this revision): `check-dead-file-coverage.py` **0 failures** (55
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
