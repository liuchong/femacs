# emacs/lwlib/xlwmenu.c (2920 lines)

`emacs/lwlib/xlwmenu.c` @ `fd7afced403`; port `lwlib/xlwmenu.rs` (**4432 lines**, md5
`9a657c6dfddcc73ad132681d6cf78522`, sha1/sha256 `e7a2011227247a1024e43eaa9c2cafdef24aa7ec`);
the reference file is 2920 lines, md5 `07aac74641dfddc7f313cfeb9cc798e4`.

**The file gate is `feature = "use_lucid"`** (P-86 clause 8, the build's condition rather than one
the C text spells): `lwlib/Makefile.in` 52 (`LUCID_OBJS = lwlib-Xlw.o xlwmenu.o lwlib-Xaw.o`),
56 (`TOOLKIT_OBJS = $(@X_TOOLKIT_TYPE@_OBJS)`) and 58 (`OBJS = lwlib.o $(TOOLKIT_OBJS)
lwlib-utils.o`) are what put this object in the build, with `configure.ac`
7231's `AC_DEFINE ([USE_LUCID], [1], …)` as the configure condition.  `USE_LUCID`,
`USE_X_TOOLKIT` and `HAVE_X11` are `undef` in docs/baseline/config/darwin-arm64.macros.tsv, so the
frozen configuration compiles none of this file and the frozen object is the empty one.

`feature = "use_x_toolkit"` is the second condition the text itself needs (it is what makes
`widget_value.toolkit_data`/`free_toolkit_data` live, lwlib-widget.h 87-101, which C 421, 446 and
447 read).  The build's own `-Demacs` is the third: P-86 clause 1's both-arms rule carries it as
`feature = "emacs"`, and its frozen side is the standalone-lwlib build C 44-48, 1474-1487,
1861-1872, 1882-1892, 2203-2210, 2821-2830 and 2877-2919 spell.

**This file is `text-only`** (P-86 clause 3(2)): this machine has no Xt, Motif, Xft or cairo
header (`/opt/homebrew/include/X11` is `xorgproto` 2025.1's Xlib half only).  The Xlib records and
scalars the file addresses are **measured with clang** against those headers (`## Evidence` ④);
every Xt, Lucid, Xft and cairo name is C's spelling with the value **UNVERIFIED**.

## Gate mapping

| C condition | feature | why |
| --- | --- | --- |
| (the build's own condition, not in the C text) | `use_lucid` | `lwlib/Makefile.in` 52/56/58 + `configure.ac` 7231 |
| (the C macro that makes the read fields live) | `use_x_toolkit` | lwlib-widget.h 87-101 gates `toolkit_data`/`free_toolkit_data` on `USE_X_TOOLKIT` |
| C 39, 41, 44, 46, 1474, 1482, 1832, 1861, 1865, 1882, 1886, 2203, 2207, 2821, 2825, 2877, 2890, 2907 | `emacs` / `not (feature = "emacs")` | the build's own `-Demacs`; both arms are carried |
| C 107, 343, 371, 1167, 1215, 1733, 2140, 2360, 2422 | `have_x_i18n` | `HAVE_X_I18N` |
| C 111, 333, 357, 1075, 1118, 1201, 1440, 1552, 2025, 2122, 2173, 2221, 2301, 2315, 2325, 2357, 2393, 2399 | `use_cairo_or_have_xft` | `#if defined USE_CAIRO \|\| defined HAVE_XFT` (the derived name `lwlib/include/xlwmenuP.rs` 53 registered) |
| C 112, 114 | `use_cairo_or_have_xft` / `not (…)` | the `DEFAULT_FONTNAME` pair |
| C 1156, 1240 | `use_cairo` | `#ifdef USE_CAIRO` (nested inside the cairo/Xft arm) |

## C line ranges

The rows ascend without overlapping and cover every non-blank line of C 1-2920 (C 1-20 is the
licence head, PORTING.md 0.1-19; `python3 tools/check-dead-file-coverage.py` reads this table).

| C | block | arm |
| --- | --- | --- |
| 1-20 | the licence head (Lucid, Inc. 1992 / FSF) | file gate |
| 21-49 | `config.h`, `<setjmp.h>`, `<lisp.h>`, `<stdio.h>`, `<sys/types.h>`, the five `<X11/*>` headers, `xlwmenuP.h`, and C 39-48’s `#ifdef emacs` pair (`bitmaps/gray.xbm` / `<X11/bitmaps/gray>`) | file gate |
| 50-52 | `pointer_grabbed`, `keyboard_grabbed`, `menu_post_event` (statics) | file gate |
| 53-94 | `xlwMenuTranslations` (static) | file gate |
| 95-102 | three FIXME comments | file gate |
| 103-103 | `#define offset(field) offsetof (XlwMenuRec, field)` | file gate |
| 104-169 | `xlwMenuResources` (static); C 107-110 the `HAVE_X_I18N` row, C 111-115 the `DEFAULT_FONTNAME` `#if`/`#else` pair | file gate |
| 170-190 | `#undef offset` (C 169) and C 171-189’s fourteen static forward declarations (deleted, PORTING.md 0.1-12) | file gate |
| 191-204 | `xlwMenuActionsList` (static) | file gate |
| 205-207 | `#define SuperClass ((CoreWidgetClass)&coreClassRec)` | file gate |
| 208-247 | `xlwMenuClassRec` (non-static) | file gate |
| 248-249 | `WidgetClass xlwMenuWidgetClass = (WidgetClass) &xlwMenuClassRec;` (non-static) | file gate |
| 250-253 | `submenu_destroyed`, `next_release_must_exit` (statics) | file gate |
| 254-258 | the `/* Utilities */` marker | file gate |
| 259-265 | `ungrab_all` (static) | file gate |
| 267-276 | `abort_gracefully` (static) | file gate |
| 278-296 | `push_new_stack` (static) | file gate |
| 298-306 | `pop_new_stack_if_no_contents` (static) | file gate |
| 308-325 | `make_old_stack_space` (static) | file gate |
| 327-355 | `string_width` (static) | file gate / `use_cairo_or_have_xft` (C 333-342) / `have_x_i18n` (C 343-350) |
| 357-387 | the `MENU_FONT_HEIGHT`/`MENU_FONT_ASCENT` pair: C 357-369 the cairo/Xft arm, C 370-379 the `HAVE_X_I18N` arm, C 380-385 the plain arm, C 385 the `#endif` | file gate |
| 388-391 | `arrow_width` (static) | file gate |
| 393-399 | `toggle_button_width` (static) | file gate |
| 402-408 | `radio_button_width` (static) | file gate |
| 411-450 | `resource_widget_value` (static) | file gate |
| 452-499 | `size_menu_item` (static) | file gate |
| 501-562 | `size_menu` (static) | file gate |
| 565-628 | `draw_arrow` (static) | file gate (the `double` token is the ledger’s note row) |
| 630-718 | `draw_shadow_rectangle` (static) | file gate |
| 721-803 | `draw_shadow_rhombus` (static) | file gate |
| 806-834 | `draw_toggle` (static) | file gate |
| 837-865 | `draw_radio` (static) | file gate |
| 868-985 | `draw_separator` (static) | file gate |
| 988-1021 | `separator_height` (static) | file gate |
| 1023-1047 | `draw_highlight` (static) | file gate |
| 1049-1248 | `display_menu_item` (static) | file gate / `use_cairo_or_have_xft` (C 1075-1077, 1118-1166, 1201-1214) / `use_cairo` (C 1156-1158, 1240-1243) / `have_x_i18n` (C 1167-1174, 1215-1224) |
| 1250-1343 | `display_menu` (static) | file gate |
| 1345-1355 | `set_new_state` (static) | file gate |
| 1357-1371 | `expose_cb` (static) | file gate |
| 1373-1387 | `set_window_type` (static) | file gate |
| 1390-1446 | `make_windows_if_needed` (static) | file gate / `use_cairo_or_have_xft` (C 1440-1442) |
| 1448-1461 | `xlwmenu_window_p` (non-static) | file gate |
| 1463-1539 | `fit_to_screen` (static) | file gate / `emacs` (C 1474-1487; the `#else` arm is the standalone lwlib build) |
| 1541-1566 | `create_pixmap_for_menu` (static) | file gate / `use_cairo_or_have_xft` (C 1552-1565) |
| 1568-1666 | `remap_menubar` (static) | file gate; C 1635-1638 is an `#if 0` block (dead text) |
| 1668-1682 | `motion_event_is_in_menu` (static) | file gate |
| 1684-1723 | `map_event_to_widget_value` (static) | file gate |
| 1726-1810 | `make_drawing_gcs` (static) | file gate / `have_x_i18n` (C 1733-1745); `BRIGHTNESS` is defined at C 1755 |
| 1812-1830 | `release_drawing_gcs` (static) | file gate |
| 1832-1837 | `#ifndef emacs` / `#define MINL(x,y)` / `#endif` | file gate |
| 1838-1945 | `compute_shadow_colors` (static) | file gate / `emacs` (C 1861-1872, 1882-1892); `MINL` is used by the `#else` arms |
| 1947-1997 | `make_shadow_gcs` (static) | file gate |
| 1999-2023 | `release_shadow_gcs` (static) | file gate |
| 2025-2026 | `#if defined USE_CAIRO || defined HAVE_XFT` (opens the three functions below) | file gate |
| 2027-2031 | `getDefaultXftFont` (static) | file gate / `use_cairo_or_have_xft` |
| 2033-2063 | `openXftFont` (static) | file gate / `use_cairo_or_have_xft` |
| 2065-2103 | `update_xft_colors` (static) | file gate / `use_cairo_or_have_xft` |
| 2104-2106 | the `#endif` that closes C 2025 | file gate |
| 2107-2180 | `XlwMenuInitialize` (static) | file gate / `use_cairo_or_have_xft` (C 2122-2139, 2173-2175) / `have_x_i18n` (C 2140-2143) |
| 2182-2185 | `XlwMenuClassInitialize` (static) | file gate |
| 2187-2225 | `XlwMenuRealize` (static) | file gate / `emacs` (C 2203-2209) / `use_cairo_or_have_xft` (C 2221-2224) |
| 2227-2244 | `XlwMenuRedisplay` (static) | file gate |
| 2247-2254 | `xlwmenu_redisplay` (non-static) | file gate |
| 2256-2323 | `XlwMenuDestroy` (static) | file gate / `use_cairo_or_have_xft` (C 2301-2318) |
| 2325-2326 | `#if defined USE_CAIRO || defined HAVE_XFT` (opens `fontname_changed`) | file gate |
| 2327-2334 | `fontname_changed` (static) | file gate / `use_cairo_or_have_xft` |
| 2335-2337 | the `#endif` that closes C 2325 | file gate |
| 2338-2431 | `XlwMenuSetValues` (static) | file gate / `use_cairo_or_have_xft` (C 2357-2359, 2393-2396, 2399-2421) / `have_x_i18n` (C 2360-2365, 2422-2428) |
| 2433-2452 | `XlwMenuResize` (static) | file gate |
| 2454-2469 | `handle_single_motion_event` (static) | file gate |
| 2471-2491 | `handle_motion_event` (static) | file gate |
| 2493-2524 | `Start` (static) | file gate |
| 2526-2532 | `Drag` (static) | file gate |
| 2534-2539 | `Nothing` (static) | file gate |
| 2541-2555 | `find_first_selectable` (static) | file gate |
| 2557-2589 | `find_next_selectable` (static) | file gate |
| 2591-2606 | `find_prev_selectable` (static) | file gate |
| 2608-2631 | `Down` (static) | file gate |
| 2633-2667 | `Up` (static) | file gate |
| 2669-2703 | `Left` (non-static) | file gate |
| 2705-2737 | `Right` (non-static) | file gate |
| 2739-2765 | `Key` (static) | file gate |
| 2767-2802 | `Select` (static) | file gate |
| 2805-2920 | `pop_up_menu` (static) | file gate / `emacs` (C 2821-2830, 2877-2919) |

## Findings

1. **The reference's own coupling between the cairo/Xft arm and `HAVE_X_I18N`.**  C 357-369's
   `MENU_FONT_HEIGHT`/`MENU_FONT_ASCENT` arm reads `mw->menu.fontSet` and `mw->menu.font_extents`,
   and `xlwmenuP.h` 59-62 declares those two members only `#ifdef HAVE_X_I18N`.  A configuration
   with `USE_CAIRO`/`HAVE_XFT` and without `HAVE_X_I18N` therefore cannot compile this file in C
   either; the port carries C's text as it stands.  Measured (`## Evidence` ③): the arm flip at
   that combination is 15 errors, all in this unit, all the same two fields.
2. **C 1635-1638 is an `#if 0` block** (`if (!mw->menu.horizontal || i > 1) ws->x +=
   mw->menu.shadow_thickness;`) - dead in every configuration, so the port records the region
   instead of transcribing it (the `src/character.rs` 91-97 / `lwlib-Xlw.md` Findings 1 class).
3. **`xlwMenuResources`'s `(XtPointer) -1` sentinels and `(XtPointer) 1/2/3/10` immediates** are
   C's own initialisers; the port spells them `(-1isize) as XtPointer` and `1isize as XtPointer`
   (the `XtArgVal`/`XtPointer` narrowing C does silently).
4. **`lucid__menu_grab_keyboard`** (C 2891, inside `#ifdef emacs`) is not declared by any header:
   it is `make-docfile`'s `globals.h` variable for keyboard.c's `DEFVAR_BOOL`
   (`src/keyboard.c` 14103-14107).  The port reads the same global from
   `src/include/globals.rs` 7057, which `lisp.rs` includes.
5. **`xlwMenuWidgetClass`** (C 249) is the non-static object `lwlib-Xlw.c`'s `lw_lucid_widget_p`
   and `lwlib-Xaw.c` compare against; the port exports it, and `xlwMenuClassRec` beside it (C 208,
   also non-static).
6. **`Left` and `Right` (C 2670, 2706) are written without `static`**, unlike their neighbours, so
   they have external linkage in C and the port exports them.

## Evidence

**① The frozen configuration (`build-tu`, 1 run of 3).**
`cd femacs && CARGO_TARGET_DIR=/tmp/xlwm-lane/cargo cargo run -q -p xtask -- build-tu
lwlib/xlwmenu.c` -> `built lwlib/xlwmenu.rs -> ../build/femacs/xlwmenu.o`, **exit 0**,
`build/femacs/xlwmenu.o` **336 bytes**, md5 `dbed68837694579185377dbb0821ab62`.  That is the
expected reading and **not a proof of the new text**: every item sits behind the `use_lucid` gate,
so the frozen configuration compiles only `include/config.rs` and the crate's `#![allow]` (P-86
clause 8's "the gate must go down to the include graph").

**② The arm flips (`probe`, 4 runs - one over the lane's budget of 3, reported honestly).**
Run 1, the file gate + the toolkit edge on an earlier text of this unit ->
**exit 1, 675 errors, all in `_root.rs`**, every one `cannot find type c_void`/`c_char`/`c_int`…
The reading is a *materialization* effect, not a defect of the body: `xtask`'s ungated-reach rule
(`xtask/src/main.rs` `ungated_reachable`, P-89) found `src/include/lisp.rs` reachable
ungated because `lwlib/include/lwlib-widget.rs` 17 carries the string `include!("…/lisp.rs")`
**inside a comment**, so this unit's `#[cfg (feature = "use_lucid")]` edge to `lisp.rs` was
suppressed and nothing brought `c_void` into the crate root.  Repair: the module's own `include!`
edges are now each preceded by their `#[cfg]` (so the walk sees them as gated), and a faithful
python replica of `xtask`'s `Expand`/`ungated_reachable` (`/tmp/xlwm-lane/xtask_sim.py`, a scratch
harness, not the tree's probe) reproduces the green readings below before they are spent.
Run 2, the file gate + the toolkit edge (the display sub-gates frozen):
`… -- probe lwlib/xlwmenu.rs /tmp/xlwm-lane/probeA.rlib --crate-type=lib -C panic=abort
--cfg 'feature="use_lucid"' --cfg 'feature="use_x_toolkit"' --cfg 'feature="emacs"'`
-> **exit 0, 0 errors, 2 warnings, an 8,278,352-byte rlib**; both warnings are the mirrors'
(`lib/include/stdbit.in.rs` 644 and `src/include/lisp.rs` 3939 - the same two `lwlib-Xlw`/
`lwlib-Xm` reported), none is this unit's.
Run 3, the file gate, the toolkit edge and **every** display sub-gate:
`… --cfg 'feature="use_lucid"' --cfg 'feature="use_x_toolkit"' --cfg 'feature="emacs"'
--cfg 'feature="use_cairo_or_have_xft"' --cfg 'feature="use_cairo"' --cfg 'feature="have_xft"'
--cfg 'feature="have_x_i18n"'` -> **exit 0, 0 errors, 6 warnings, an 8,316,368-byte rlib**.  Three
of the six are `lwlib-int.rs` 91/94/96's "`widget_xft_data` is not FFI-safe" - this unit's
in-unit stand-in for the tag was a bare `pub struct`; run 4 below is the same command after the
one-line `#[repr (C)]` repair, on the tree's final text.
Run 4 (the budgeted third run spent on the repaired text): **exit 0, 0 errors, 2 warnings, an
8,317,240-byte rlib** - the same two mirror warnings as run 2.
`## Evidence` ③'s third combination is the reference's own dead end (Findings 1).

**③ The `#if` arms, both readings.**  The `#if` conditions and both of their arms are in
`## Gate mapping`; `## Gate mapping`'s rows 3-8 are the ones the two probe runs compile in
opposite states (frozen in run 2, all on in runs 3-4).  The one combination the two runs do not
cover is `use_cairo_or_have_xft` + `have_xft` **without** `have_x_i18n` and without `use_cairo`:
`… --cfg 'feature="use_lucid"' --cfg 'feature="use_x_toolkit"' --cfg 'feature="emacs"'
--cfg 'feature="use_cairo_or_have_xft"' --cfg 'feature="have_xft"'` -> **exit 1, 14 errors, all
in this unit** (`7 x no field fontSet on type _XlwMenu_part` + `7 x no field font_extents on type
_XlwMenu_part`, at the `MENU_FONT_HEIGHT`/`MENU_FONT_ASCENT`/`string_width` lines that spell
C 357-369 and C 1733-1745).  That is the reference's own text (Findings 1), not a port defect: the two
arms can only be compiled in the reference when `HAVE_X_I18N` is defined beside them.

**④ The mutation test (P-86 clause 7).**  A `/tmp` copy of run 4's materialized root
(`/tmp/xlwm-lane/mut`) with one type error injected into **this unit's own text** -
`let _mutant: c_int = c"boom".as_ptr ();` as the first statement of `ungrab_all` - compiled with
run 4's `rustc` flags: control **exit 0, 0 errors** (`0` error lines); mutant **exit 1, exactly one
new error**, `E0308` at `mut/_root.rs:1403` naming that line.  The output changes, so runs 2-4's
green readings are not vacuous.  The copy was discarded; `lwlib/xlwmenu.rs`'s
`shasum` is `e7a2011227247a1024e43eaa9c2cafdef24aa7ec` before and after (the injected text was
never in the tree).

**⑤ The Xlib half, measured with clang** (`xorgproto` 2025.1 + `libX11`, `clang
-I/opt/homebrew/include /tmp/xlwm-lane/xlib_probe.c -lX11`):

```
sizeof(XEvent) 192   sizeof(XButtonEvent) 96   sizeof(XMotionEvent) 96   sizeof(XKeyEvent) 96
sizeof(XSetWindowAttributes) 112   sizeof(XWindowAttributes) 136   sizeof(XFontStruct) 96
sizeof(XGCValues) 128   sizeof(XPoint) 4   sizeof(XRectangle) 8   sizeof(XCharStruct) 12
sizeof(XColor) 16   sizeof(Display*) 8 (a pointer; `struct _XDisplay` is incomplete)
XButtonEvent offsets: type 0 serial 8 send_event 16 display 24 window 32 root 40 subwindow 48
                      time 56 x 64 y 68 x_root 72 y_root 76 state 80 button 84 same_screen 88
XMotionEvent: type 0 window 32 time 56 x 64 y 68 x_root 72 y_root 76 state 80 is_hint 84
XKeyEvent: type 0 window 32 state 80 keycode 84
XSetWindowAttributes: background_pixel 8 border_pixel 24 bit_gravity 32 event_mask 72
                      override_redirect 88 colormap 96 cursor 104
XWindowAttributes: x 0 y 4 width 8 height 12 border_width 16 class 40 bit_gravity 44
                   colormap 80 map_installed 88 override_redirect 120
XFontStruct: min_bounds 56 max_bounds 68 ascent 88 descent 92
XGCValues: foreground 16 background 24 line_style 36 cap_style 40 join_style 44 fill_style 48
           font 88 graphics_exposures 100 clip_mask 112
XCharStruct: lbearing 0 rbearing 2 width 4 ascent 6 descent 8 attributes 10
ButtonPress 4 ButtonRelease 5 MotionNotify 6 KeyPress 2 KeyRelease 3 EnterNotify 7
LeaveNotify 8 Expose 12 CurrentTime 0 None 0 LineSolid 0 LineOnOffDash 1 LineDoubleDash 2
Convex 2 CoordModeOrigin 0 GCForeground 4 GCBackground 8 GCLineStyle 32 GCFillStyle 256
GCStipple 2048 GCFont 16384 GCGraphicsExposures 65536 FillStippled 2 GrabModeAsync 1
Success 0 ButtonPressMask 4 ButtonReleaseMask 8 PointerMotionMask 64 PointerMotionHintMask 128
XA_ATOM 4 PropModeReplace 0 XC_right_ptr 94
```

`Position` and `Dimension` are **not** in this machine's `Xlib.h`/`X.h`, so their two widths are
C's spelling and stay **UNVERIFIED** (`## Deviations` 1).

**⑥ The text gates.**
`python3 tools/extract-abi.py --check emacs/lwlib/xlwmenu.c lwlib/xlwmenu.rs` -> the lone
row `line count drifted 51.8%: C 2920 vs Rust 4432`, whose tolerance row is the integrator's
(`## Owed` 2).  Before the two `docs/ports/macro-body-notes.tsv` rows below, the same command also
reported four body rows, all repaired or recorded: `draw_arrow does not mention double` and
`make_drawing_gcs does not mention float` (the two type keywords of C locals, recorded),
`draw_separator`/`separator_height` `does not mention case/default/switch/break` (C's `switch`
becomes a `match`, recorded), and the `DEFAULT_FONTNAME` constant row (the two-armed `#define` is
now a two-armed `macro_rules!` plus the frozen arm's `const`, both carried).
`python3 tools/check-declarations.py` -> `17 declaration sites (0 forbidden, 17 allowed literals),
0 failures` (whole-tree last line).
`python3 tools/check-mirror-dupes.py` -> `17 duplicate declaration(s), 17 ratchet row(s),
0 failure(s)`; this unit's two files are not among the 17.
`python3 tools/check-dead-file-coverage.py` -> `104 ledger(s) bound (a C range table),
0 failure(s)` - this ledger's 84 rows are bound for `lwlib/xlwmenu.c` and, by the `<stem>.md`
fallback, for `lwlib/xlwmenu.h` too.
`python3 tools/check-ledger-ticks.py` -> `233 ledger(s) bound (a function tick to compare),
0 failure(s), 0 carried by another file, 357 row(s) with no function tick, 0 row(s) without a
ledger`.
`python3 tools/check-citations.py` -> `676 file(s) scanned, 0 drift, 0 range, 1512 external,
36 allowed, 0 stale`.  This unit's citations are all in the **external** bucket (they name
`<X11/*>`, Xft, cairo and the in-tree-but-foreign `src/xterm.h` / `lwlib-utils.h`), so neither the
drift nor the range rule applies to them; an earlier reading in this same round carried one `drift`
and one `stale` row in `src/include/dispextern.rs` 609/617 (another lane's), and the current
reading has neither.

**⑦ Budget as run** (the lane's limit): `build-tu` **1 of 3**, `probe` **4 of 3** (run 1 is the
materialization reading of ②, runs 2-4 the two green arms and the repaired re-run: one over the
budget, reported here), plus the `/tmp` simulator, one control/mutant pair and one `#if`-arm
reading that are `rustc` invocations and not `probe` runs; no `cargo test`, no `xtask --all`, no
`gate-fast`, no `xtask dead-arms`, no `check-all-units.sh`, and no git write of any kind.
`df -h /` after the readings: `/dev/disk3s1s1 1.8Ti 12Gi 3.9Gi 76% /`; `/tmp/xlwm-lane` (48 MB
before the run's own cleanup) is removed at the end.

## Deviations

1. **The Xt/Lucid/Xft/cairo surface is C's spelling, not a measurement** (P-86 clause 3(2)): this
   machine has no `<X11/Intrinsic.h>`, `<X11/IntrinsicP.h>`, `<X11/ObjectP.h>`, `<X11/CoreP.h>`,
   `<X11/CompositeP.h>`, `<X11/StringDefs.h>`, `<X11/Shell.h>`, `<X11/Xft/Xft.h>` or `<Xm/*.h>`.
   `WidgetRec`, `CorePart` and `CoreClassPart` carry the members this file reads in C's relative
   order with the members between them omitted, so their offsets are knowingly not the real ones
   (`CoreClassPart`'s 32 members are complete and in C's own initializer order, C 210-242);
   `XtResource`, `XtActionsRec`, `Arg`, `XtWidgetGeometry`, `XftFont`, `XftColor`, `XftDraw`,
   `XGlyphInfo`, `XRenderColor`, `XFontSetExtents`, the 33 `Xt*`/`Xm*` resource names this unit
   adds to `lwlib/include/xlwmenu.rs`'s 43, the 10 Xt scalars and the 39 Xlib constants are the
   same class.  `Position` and `Dimension` are **UNVERIFIED** (`## Evidence` ⑤): this machine's
   `Xlib.h`/`X.h` do not carry them.
2. **The Xlib half is measured** (`## Evidence` ⑤) and every declaration carries a `/// C:` line
   naming the header and the declaration it stands for; the three Xlib records the file addresses
   member by member (`XEvent` 192, `XButtonEvent` 96, `XMotionEvent`/`XKeyEvent` 96,
   `XSetWindowAttributes`, `XWindowAttributes`, `XFontStruct`, `XGCValues`, `XPoint`,
   `XRectangle`, `XCharStruct`, `XColor`) carry those measured shapes, and `XButtonPressedEvent`
   is Xlib's own alias spelled `pub type`.
3. **C's fourteen static forward declarations (C 171-189) are deleted** (PORTING.md 0.1-12 /
   checklist 10); Rust items are order-independent, and `## C line ranges` names the region.
4. **C 1635-1638 is an `#if 0` block** and is not transcribed (Findings 2).
5. **The `#if`-split `if`/`else` chains are carried with their arms behind their own gates.**  In
   C, `display_menu_item` (C 1118-1174, C 1201-1224), `XlwMenuInitialize` (C 2122-2139),
   `XlwMenuSetValues` (C 2357-2365, 2393-2428) and `compute_shadow_colors` (C 1861-1872,
   1882-1892) each gate the *middle* of an `if`/`else-if`/`else` chain, so one arm's condition is
   another arm's `else`.  Rust cannot split an `else` with an attribute, so the chains are spelled
   with the state the `else` carries - `painted_p`/`key_painted_p` in `display_menu_item`,
   `xft_opened_p` in `XlwMenuInitialize`, `fontname_changed_p`/`font_changed_p` in
   `XlwMenuSetValues`, and a cfg'd `let frobbed_ok` pair in `compute_shadow_colors` - which
   reproduces C's control flow exactly (an arm that answered suppresses the later ones).
6. **Locals C leaves uninitialised start at zero** (PORTING.md 0.1-13): `xcs`, `drop`, `gi`,
   `ink`/`logical`, `xgcv`, `topc`/`botc`, `points`/`pt`, `av`, `r#where`, `relative_pos`,
   `selection_position`, `colors`, `xswa`, `label_width`/`rest_width`/`button_width`/`height`/
   `width`, `val`/`level`, `screen_x`/`screen_y`/`screen_w`/`screen_h`,
   `prev_screen_x`/`prev_screen_y`/`screen_width`/`screen_height`, `mask`, `ws`, `following_item`,
   `no_return`.  Named here because a reviewer cannot see the zeroing in C.
7. **C's truth tests are spelled**: `if (!p)` -> `if p.is_null ()`, `if (x)` -> `if x != 0`,
   `if (!val->toolkit_data)` -> `if (*val).toolkit_data.is_null ()`, `if (*top_pixmap == None)`
   keeps C's `!*top_pixmap` meaning (the value, not the pointer), `XtIsShell (…) == 0` for C's
   `!XtIsShell (…)`, and `val->enabled`/`val->selected` stay Rust `bool` (the mirror's reading)
   with `as c_int` where C passes them to an `int` parameter.
8. **C's `(GC) -1`, `(Pixmap) -1`, `(XtPointer) -1`** are `(-1isize) as T`, and C's `(unsigned
   long) 0`/`1` arguments are typed literals.
9. **`sizeof` and `XtNumber`**: C's `sizeof (T)` is `::core::mem::size_of::<T> ()` (the
   `token-aliases.tsv` family), and C's `XtNumber (arr)` is `XtNumber! (T, LEN)` - Rust's const
   evaluator may not read another static's value, so the count comes from the array's declared
   length (`XLWMENU_RESOURCES_LEN` 25 or 24 by the `have_x_i18n` arm, `XLWMENU_ACTIONS_LEN` 10);
   the compiler still ties the two together, because the array's type uses the same name.
10. **`offset(field)`** keeps C's macro name and takes C's own two-token argument shape
    (`offset! (menu.margin)`), because `offset_of!` needs the field path spelled out; the result is
    narrowed to the `Cardinal` field it initialises, as C's implicit conversion does.
11. **`windows[i]`, `old_stack[i]`, `new_stack[i]`** are `(*ptr.offset (i as isize))` with the
    dereference C's `[]` implies, and the two-deep reads (`old_stack[i]->contents`) carry C's two
    dereferences.
12. **The three tables are `static mut` arrays** (C's own `static XtResource …[]`), so they must be
    reached by `&raw mut`/`.offset`; the class record and `xlwMenuActionsList` are `static mut` for
    the same reason (a plain `static` would need `Sync`, which the raw-pointer members do not
    have).
13. **The Xft/cairo surface is declared once, in this unit.**  `xlwmenuP.h` 28-32's `USE_CAIRO`
    edge (`#include "lwlib-utils.h"`) is **not** an `include!("include/lwlib-utils.rs")`: that
    mirror declares a second, different `XftFont`/`XftColor`/`XftDraw`/`XGlyphInfo` beside this
    unit's (they landed in the same header under a different reading) and the two do not unify, so
    the edge is answered by this unit's own declaration set; the tag `widget_xft_data` that
    `lwlib-int.h`'s `xft_data` field names is declared in the unit instead (`## Owed` 4 names the
    mirror lane).
14. **`DEFAULT_FONTNAME` is a two-armed `macro_rules!`** (C's own `#define` form, C 112/114) plus
    the frozen arm's `const` for the `constants` rule: the two-armed `const` version made that rule
    compare C's `"XtDefaultFont"` against the other arm's `c"Sans-10"`.
15. **Chained assignments are split**: C 2072-2075's `colors[0].pixel = mw->menu.xft_fg.pixel =
    mw->menu.foreground;` is two statements in C's order, and `openXftFont`'s
    `fname = fname_minus = xstrdup (…)` is spelled with the two locals (`fname_in`/`fname`) the
    later reads need.
16. **Rust keywords**: C's `type` parameter is `r#type` (C 878, 1378), C's `where` parameter is
    `r#where` (C 1056), and `CorePart`'s `self` is `self_`.
17. **The mirrors are included inside a module plus a glob import** (`pub mod xlwmenu_decls` +
    `use xlwmenu_decls::*`), this unit's reading of PORTING.md 0.1-10 / P-39: `include/xlwmenu.rs`
    and `include/xlwmenuP.rs` declare `xlwMenuWidgetClass`, `xlwMenuClassRec`,
    `xlwmenu_window_p` and `xlwmenu_redisplay` in a plain `unsafe extern "C"` block, and a Rust
    definition cannot share a scope with the foreign declaration of the same name (E0428).  Each
    `include!` inside that module carries its own `#[cfg]` so `xtask`'s ungated-reach rule
    (`## Evidence` ②) sees the edges as gated.
18. **C's Xt macros are entry points**: `XtDisplay`, `XtScreen`, `XtParent`, `XtWindow`,
    `XtIsShell`, `XtIsRealized`, `XtNumber` and `XtSetArg` keep C's names and call shapes, so the
    call sites carry C's implicit `XlwMenuWidget` -> `Widget` conversion as `as Widget` (the
    `lwlib-Xlw.rs`/`lwlib-Xm.rs` reading of the same shape).
19. **Two `docs/ports/macro-body-notes.tsv` rows were appended once** (the C type keywords
    `double`/`float` of two locals, and the two `switch` functions the port spells with `match`),
    plus the two rows' reasons; the file is shared and append-only.

## Owed

1. **The Xt/Lucid/Xft/cairo surface's real shapes and values** (P-88 clause 2): with a toolchain
   that has `<X11/Intrinsic.h>`, `<X11/Xft/Xft.h>`, `<Xm/*.h>` and cairo, measure `WidgetRec`/
   `CorePart`/`CoreClassPart`/`XtResource`/`XtActionsRec`/`Arg`, the four Xft records, `Position`,
   `Dimension`, every resource name in `xlwMenuResources` and the Xt enum values, and replace the
   placeholders.  **Owner: the batch-F integrator** (this machine cannot supply any of it).
2. **`docs/ports/line-drift-tolerance.tsv` needs this file's row**: measured drift **51.8%**
   (C 2920 vs Rust 4432), which is the port's in-unit surface plus the transported dead arms.  The
   file is not in this lane's write whitelist.  **Owner: the integrator.**
3. **`xtask/Cargo.toml`'s generated `[features]` table** gains the name `emacs` from this
   unit (P-86 clause 1's arm for the build's own `-Demacs`); `use_lucid`, `use_x_toolkit`,
   `have_x_i18n`, `use_cairo`, `have_xft` and `use_cairo_or_have_xft` are already registered.
   **Owner: the integrator** (the table is generated from the tree).
4. **`lwlib/include/xlwmenu.rs` 110-124 and `lwlib/include/xlwmenuP.rs`** still declare their entry
   points in a plain `unsafe extern "C"` block instead of the `<stem>_decls` module + glob import
   the newer mirrors use (e.g. `lwlib-Xlw.rs` 25-49).  This unit wraps them at the inclusion site
   instead, which is what makes it compile; the mirrors' own shape is theirs to change.
   **Owner: the lwlib mirror lane.**
5. **The reference's `USE_CAIRO`/`HAVE_XFT`-without-`HAVE_X_I18N` combination** (Findings 1) stays
   uncompilable in the port exactly as it is in C.  If the reference ever guards those two field
   reads, the port follows.  **Owner: whoever owns the reference reading (not a port defect).**

## Not claimed

- **Zero run evidence**: nothing was linked with Xt, Lucid, cairo or Xft and no menu was created,
  displayed, selected or popped up.  This target cannot link any of it.
- **The Xt/Lucid/Xft/cairo surface is not measured** (`## Deviations` 1): every width, offset,
  enum value and resource name outside the Xlib half is C's spelling, and the records' omitted
  members mean their offsets are knowingly not the real ones.
- **The frozen object is the 336-byte empty one**, so it says nothing about the body; the
  8,278,352 / 8,316,368 / 8,317,240-byte rlibs are `--crate-type=lib` type-check readings of the
  unit and its mirrors, not a build of the widget.
- **The two warnings in runs 2 and 4 are the mirrors'** (`lib/include/stdbit.in.rs` 644,
  `src/include/lisp.rs` 3939); this unit compiles with none of its own.
- **No ERT, no display, no `.elc`/dump evidence**: this file is reachable only through the Lucid
  build this target does not have, so `## Evidence` ⑥ is the whole acceptance here.
