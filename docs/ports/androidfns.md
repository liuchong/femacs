# emacs/src/androidfns.c (3797 lines)

> **This file is P-86's unit: the frozen configuration compiles no `androidfns.o`.** `configure.ac`
> 2597 puts it in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])` (both Android branches, like
> `androidmenu.o`/`androidterm.o`), so `docs/baseline/frozen-build-sources.txt` lists no
> `androidfns.o` for this target and `extract-abi.py`'s checklist for the file is empty - its
> checkbox sections cannot be coverage evidence (P-86 ②).  What follows is P-86 ②'s substitute: the
> **C line-range table**, the `## Evidence` readings and the `## Deviations`.
> Port: `femacs/src/androidfns.rs`, **slice 1 = C 1-1214** (1,573 Rust lines), **slice 2 = C
> 1216-1489** (1,994 Rust lines), **slice 3 = C 1490-1704** (2,225 Rust lines), **slice 4 = C
> 1705-1800** (2,332 Rust lines), **slice 5 = C 1801-1910** (2,447 Rust lines), **slice 6 = C
> 1911-2170** (2,757 Rust lines), **slice 7 = C 2171-2296** (2,898 Rust lines), **slice 8 = C
> 2295-2583** (3,313 Rust lines), **slice 9 = C 2584-2662** (3,416 Rust lines), **slice 10 = C
> 2663-2832** (3,615 Rust lines), **slice 11 = C 2833-2972** (3,826 Rust lines), **slice 12 = C
> 2973-3063** (3,951 Rust lines), **slice 13 = C 3064-3195** (4,183 Rust lines) and **slice 14 = C
> 3196-3357** (4,311 Rust lines), both arms of every `#if` carried.  The old revision's conclusion ("not in the frozen build, so the port is the empty
> unit plus a note") was P-86 ①'s "not started" case; its measurements are still true and are still
> where the file gate comes from.

**Why 1214.**  P-86 ④ says to start at C 1 and stop at a whole-item boundary.  C 733-1214 is the
file's first `DEFUN`, `Fx_create_frame`; its closing brace at C 1214 is the whole-item boundary
nearest the 1,150-line target (C 727's `android_create_frame_window` is the last boundary *before*
1,000, and C 836-1892-style long functions put the next one 1,000 lines further on).
**The next live definition is C 1216's `DEFUN ("xw-color-defined-p", …)`**, whose stub/real arm pair
begins at C 1220/1223.  Landing this `DEFUN` is also what stops `extract-abi.py --check` being
vacuous for the file (P-86 ⑩): before this slice the port had no `fn` at all.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2597's `ANDROID_OBJ`, under 2594's `AC_DEFINE([HAVE_ANDROID])`) | `feature = "have_android"` (on every item and on C 20-31's include graph) | an Android configuration |
| C 33 `#ifndef ANDROID_STUBIFY` … C 58 `#endif` (the seven tooltip objects) | `all (feature = "have_android", feature = "not_android_stubify")` | Android, API calls real |
| C 124 `#ifndef ANDROID_STUBIFY` … C 729 `#endif /* ANDROID_STUBIFY */` (the frame hooks) | the same pair | Android, API calls real |
| C 738 `#ifdef ANDROID_STUBIFY` (inside `Fx_create_frame`), C 741 `#else` | `feature = "have_android"` on the `DEFUN!`; the stub arm is `not (feature = "not_android_stubify")`, the real arm `feature = "not_android_stubify"` | an Android configuration, one arm of the pair |
| C 1015-1022 `#if 0` (the two `android_default_scroll_bar_color_parameter` calls) | **no gate**: dead in every configuration, recorded here and not transcribed (the `src/xfns.rs` 958 `#if false` precedent) | never |

`not_android_stubify` is the feature the tree registered for C's
`HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv`, first used by
`alloc.rs`).  A third condition has **no `#if` in this file and therefore no port gate**:
`HAVE_WINDOW_SYSTEM` (`configure.ac`'s `window_system=android` branch is what puts the file in the
build list, so an Android configuration always has it).  Six places in this slice's text need it -
C 341/349's two frame flags, C 365-366's `f->tool_bar_window`, C 466-467's `f->tab_bar_window` and
C 856's override-redirect constant - so the arm reading is a `have_android`-plus-`not_android_stubify`
reading of the mirror's *frozen* side where the mirror gates an item on `have_window_system`
(Evidence ②/③).

## C line ranges

One row per C block, in C's order, contiguous (each row absorbs the blank lines that follow the one
before), covering C 1-3797.  `docs/port-map.md` 367 pairs only `src/androidfns.c` with this unit
(there is no `src/androidfns.h` binding), so one table is what `check-dead-file-coverage.py` reads.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19 | blank | - |
| 20 | `#include <config.h>` | ungated (every unit includes it) |
| 21-22 | `<math.h>` (`pow`), `<stdlib.h>` | file gate |
| 23 | `lisp.h` 30/33's `<string.h>`/`<limits.h>`, which the port `include!`s at this line (`strcmp`, `INT_MAX`) | file gate |
| 24-31 | the eight mirror includes: `lisp.h` `android.h` `androidterm.h` `blockinput.h` `keyboard.h` `buffer.h` `androidgui.h` `pdumper.h` | file gate |
| 32 | blank | - |
| 33-58 | `#ifndef ANDROID_STUBIFY` + `tip_frame`, `tip_window`, `tip_dx`, `tip_dy`, `tip_timer`, `tip_last_string`, `tip_last_frame`, `tip_last_parms` | real arm |
| 59 | blank | - |
| 60-76 | `android_display_info_for_name` (the walk and its `error`) | file gate |
| 77 | blank | - |
| 78-114 | `check_android_display_info` | file gate |
| 115 | blank | - |
| 116-120 | `check_x_display_info` (the file's one non-`static` accessor of the three) | file gate |
| 121-123 | blank / `^L` separators | - |
| 124-135 | C 124 `#ifndef ANDROID_STUBIFY` + `gamma_correct` | real arm |
| 136-159 | `android_defined_color` | real arm |
| 160-177 | `android_decode_color` (the `static unsigned long` one) | real arm |
| 178-210 | `android_set_parent_frame` (C 187 the `p = XFRAME (…)` inside the test) | real arm |
| 211-227 | `android_set_name_internal` | real arm |
| 228-282 | `android_set_name` (C 266/273 reassign the parameter) | real arm |
| 283-289 | `android_implicitly_set_name` | real arm |
| 290-296 | `android_explicitly_set_name` | real arm |
| 297-321 | `android_set_tool_bar_lines` | real arm |
| 322-348 | `android_set_tool_bar_position` | real arm |
| 349-394 | `android_change_tool_bar_height` (C 365-366 the tool-bar window's matrix) | real arm |
| 395-424 | `android_set_tab_bar_lines` | real arm |
| 425-488 | `android_change_tab_bar_height` (C 466-467 the tab-bar window's matrix) | real arm |
| 489-503 | `android_set_scroll_bar_default_height` | real arm |
| 504-515 | `android_set_scroll_bar_default_width` | real arm |
| 516-539 | `android_icon_verify` | real arm |
| 540-570 | `android_icon` | real arm |
| 571-605 | `android_make_gc` (C 583-599 the three GCs) | real arm |
| 606-634 | `android_free_gcs` | real arm |
| 635-660 | `unwind_create_frame` | real arm |
| 661-666 | `do_unwind_create_frame` (the one helper whose address C takes) | real arm |
| 667-707 | `android_default_font_parameter` (C 679-690 the font-name list) | real arm |
| 708-727 | `android_create_frame_window` (C 714-720 the window creation) | real arm |
| 728-729 | `#endif /* ANDROID_STUBIFY */` and the blank after it | gate marker |
| 730-1214 | C 733-1214 `DEFUN ("x-create-frame")`: C 738-740 the stub arm, C 741-1213 the real arm, C 1015-1022 the `#if 0` | file gate, both sub-arms |
| 1215-1236 | `xw-color-defined-p` (C 1220-1222 the stub arm, C 1223-1235 the real one) | file gate, both sub-arms |
| 1237-1258 | `xw-color-values` (C 1242-1244 / C 1245-1257 the pair) | file gate, both sub-arms |
| 1259-1269 | `xw-display-color-p` | file gate |
| 1270-1280 | `x-display-grayscale-p` | file gate |
| 1281-1293 | `x-display-pixel-width` (C 1287-1289 / C 1290-1292 the pair) | file gate, both sub-arms |
| 1294-1306 | `x-display-pixel-height` (C 1300-1302 / C 1303-1305 the pair) | file gate, both sub-arms |
| 1307-1318 | `x-display-planes` | file gate |
| 1319-1337 | `x-display-color-cells` (C 1333-1334 the truncation) | file gate |
| 1338-1350 | `x-server-vendor` (C 1343-1345 / C 1346-1349 the pair; C 1348 reads a DEFVAR global) | file gate, both sub-arms |
| 1351-1363 | `x-server-version` (C 1356-1358 / C 1359-1362 the pair) | file gate, both sub-arms |
| 1364-1371 | `x-display-screens` | file gate |
| 1372-1384 | `x-display-mm-width` (C 1377-1379 / C 1380-1383 the pair) | file gate, both sub-arms |
| 1385-1397 | `x-display-mm-height` (C 1390-1392 / C 1393-1396 the pair) | file gate, both sub-arms |
| 1398-1409 | `x-display-backing-store` (C 1406-1407 the comment C carries) | file gate |
| 1410-1424 | `x-display-visual-class` | file gate |
| 1425-1457 | C 1426's `#ifndef ANDROID_STUBIFY` plus `android_make_monitor_attribute_list` (C 1439-1450 the frame walk) | real arm |
| 1458-1489 | C 1459's `DEFUN ("android-display-monitor-attributes-list")`: C 1472-1474 the stub arm, C 1475-1488 the real one (C 1479 the `memset`) | file gate, both sub-arms |
| 1490-1624 | C 1491's `#ifndef ANDROID_STUBIFY` plus `frame_geometry` (C 1510-1512 the early return, C 1520-1553 the parent / no-parent split, C 1585-1621 the attribute list) | real arm |
| 1625-1675 | C 1626's `DEFUN ("android-frame-geometry")`: C 1669-1671 the stub arm, C 1672-1674 the real one | file gate, both sub-arms |
| 1676-1704 | C 1677's `DEFUN ("android-frame-edges")`: C 1696-1700 the real arm **first** (`#ifndef`), C 1701-1703 the stub one | file gate, both sub-arms |
| 1705-1743 | C 1706's `#ifndef ANDROID_STUBIFY` plus `android_frame_list_z_order` (C 1722-1734 the child walk, C 1736-1737 the `xfree`) | real arm |
| 1744-1778 | C 1745's `DEFUN ("android-frame-list-z-order")`: C 1762-1764 the stub arm, C 1765-1777 the real one | file gate, both sub-arms |
| 1779-1800 | C 1780's `#ifndef ANDROID_STUBIFY` plus `android_frame_restack` (C 1790-1793 the four writes, C 1796 the reconfigure call) and C's own `#endif` at 1800 | real arm |
| 1801-1837 | C 1802's `DEFUN ("android-frame-restack")`: C 1825-1827 the stub arm, C 1828-1836 the real one (C 1832's `!(A && B)` over two integer handles) | file gate, both sub-arms |
| 1838-1850 | C 1839's `DEFUN ("android-mouse-absolute-pixel-position")` (C's `(void)` parameter list, C 1848's comment) | file gate |
| 1851-1862 | C 1852's `DEFUN ("android-set-mouse-absolute-pixel-position")` | file gate |
| 1863-1895 | C 1864's `DEFUN ("android-get-connection")`: C 1873-1875 the stub arm, C 1876-1894 the real one (C 1881 the display list test, C 1887-1890 the three-way bit-depth read) | file gate, both sub-arms |
| 1896-1910 | C 1897's `DEFUN ("x-display-list")` (C 1905-1907 the `name_list_element` walk) | file gate |
| 1911-1925 | C 1912's `#ifndef ANDROID_STUBIFY` and `unwind_create_tip_frame` (the second helper whose address C takes) | real arm |
| 1926-2169 | `android_create_tip_frame` (C 1927-2169: C 1942-1948 the name lookup, C 1950-1971 the frame setup, C 1973-1998 the colour initialisation, C 2000-2107 the parameter defaults, C 2109-2144 the `tooltip` and `display-type` parameters, C 2146-2168 the frame's officialisation) | real arm |
| 2170-2218 | C 2171's `android_hide_tip` (C 2174-2178 the timer cancel, C 2180-2185 the early return, C 2188-2216 the delete/invisible body) | real arm |
| 2219-2294 | C 2220's `compute_tip_xy` (C 2242-2256 the mouse-frame read and the coordinate translation, C 2258-2261 the screen bounds, C 2263-2293 the two clamps) | real arm |
| 2295-2583 | C's `#endif` at 2296 and C 2298's `DEFUN ("x-show-tip")`: C 2303-2305 the stub arm, C 2306-2582 the real one (C 2307-2320 the locals and `AUTO_STRING`, C 2322-2347 the parameter defaults, C 2349-2428 the reuse/hide logic, C 2430-2460 the tip frame creation, C 2462-2500 the root window, C 2502-2516 the text insertion, C 2518-2534 the `ENABLE_CHECKING` row walk, C 2536-2556 the size and placement, C 2558-2575 the update, C 2577-2581 the label and the timer) | file gate, both sub-arms |
| 2584-2600 | C 2585's `DEFUN ("x-hide-tip")`: C 2589-2596 the stub arm (C 2593-2595's `#if 0` recorded, not transcribed) and C 2597-2598 the real one | file gate, both sub-arms |
| 2601-2619 | C 2602's `DEFUN ("android-detect-mouse")`: C 2609-2615 the real arm, C 2616-2617 the stub | file gate, both sub-arms |
| 2620-2638 | C 2621's `DEFUN ("android-detect-keyboard")`: C 2628-2634 the real arm, C 2635-2636 the stub | file gate, both sub-arms |
| 2639-2662 | C 2640's `DEFUN ("android-toggle-on-screen-keyboard")`: C 2650-2659 the real arm and C 2661's `return Qnil;` outside its `#endif` | file gate |
| 2663-2693 | C 2666's `#ifndef ANDROID_STUBIFY` and `android_set_background_color` (C 2679-2692 the window arm) | real arm |
| 2694-2703 | `android_set_border_color` (C 2699's "no window borders" comment) | real arm |
| 2704-2748 | `android_set_cursor_color` (C 2714-2718 the fore-pixel choice, C 2722-2728 the collision guard, C 2733-2745 the window arm) | real arm |
| 2749-2755 | `android_set_cursor_type` | real arm |
| 2756-2789 | `android_set_foreground_color` (C 2776-2780 the cursor-pixel guard) | real arm |
| 2790-2814 | `android_set_child_frame_border_width` (C 2797-2802 the three-way border) | real arm |
| 2815-2832 | `android_set_internal_border_width` | real arm |
| 2833-2900 | `android_set_menu_bar_lines` (C 2845-2846 the minibuf-only/parent guard, C 2848-2851 the ranged test, C 2856-2859 the two sizes, C 2865-2897 the redisplay block, C 2899 the glyph update) | real arm |
| 2901-2925 | C 2904-2905's "must stay in sync" comment and `enum mouse_cursor` (the fifteen members and `mouse_cursor_max`) | real arm |
| 2926-2940 | `struct mouse_cursor_types` (C 2933-2935's FIXME comment) | real arm |
| 2941-2963 | C 2942's "must stay in sync" comment and the `mouse_cursor_types[]` table (fifteen rows) | real arm |
| 2964-2972 | `struct mouse_cursor_data` (the two `mouse_cursor_max`-long arrays) | real arm |
| 2973-3043 | `android_set_mouse_color` (C 2981 the aggregate initialiser, C 2992-2999 the shape loop, C 3003-3005 the cursor loop, C 3007-3013 the window arm, C 3015-3020 the `INSTALL_CURSOR` macro with C 3022-3036's fifteen calls, C 3040/3042 the tail) | real arm |
| 3044-3063 | `android_set_title` (C 3050-3051 the early return, C 3053's `update_mode_lines`, C 3055 the title, C 3057-3060 the name) | real arm |
| 3064-3116 | `android_set_alpha` (C 3073-3074's "unsupported on Android" comment, C 3076-3105 the two-element parse, C 3107-3108 the writes, C 3110-3115 the terminal hook) | real arm |
| 3117-3128 | `android_set_no_focus_on_map` (C 3122-3127) | real arm |
| 3129-3140 | `android_set_no_accept_focus` (C 3134-3139) | real arm |
| 3141-3195 | C 3142's `frame_parm_handler android_frame_parm_handlers[]` (C 3144-3194's fifty-one entries) | real arm |
| 3196-3239 | C 3199's "Battery information support" comment and C 3201's `DEFUN ("android-query-battery")` (C 3223-3224 the init test, C 3228-3229 the query, C 3231-3238 the eight-element list) | file gate |
| 3240-3265 | C 3243's "SAF directory access management" comment and C 3245's `DEFUN ("android-request-directory-access")` (C 3256-3258 the API-level test with C's concatenated literal) | file gate |
| 3266-3291 | C 3269's "storage permissions" comment and C 3271's `DEFUN ("android-external-storage-available-p")` (C 3285-3288 the no-GUI fallback, C 3290 the test) | file gate |
| 3292-3309 | C 3293's `DEFUN ("android-request-storage-access")` | file gate |
| 3310-3337 | C 3313-3316's comments and `android_set_preeditarea` (C 3323 the frame, C 3325-3330 the coordinate conversion, C 3332-3336 the anchor call) | real arm |
| 3338-3357 | C 3341's "Debugging" comment, C 3343's `DEFUN ("android-recreate-activity")` and C 3357's `#endif /* !ANDROID_STUBIFY */` | file gate / gate marker |
| 3358-3797 | not ported: the file's remaining 440 lines (C 3361's `#ifndef ANDROID_STUBIFY` with `syms_of_androidfns_for_pdumper` at C 3363-3577, then C 3579's `syms_of_androidfns` with its registrations) | not ported |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/androidfns-lane/cargo cargo run -q -p xtask -- build-tu
src/androidfns.c` → `built src/androidfns.rs -> ../build/femacs/androidfns.o`, **exit 0**, and
`build/femacs/androidfns.o` is **336 bytes**, md5 `dbed68837694579185377dbb0821ab62` - an empty
object.  That is the expected reading, not a proof of the new text: every item sits behind a
`have_android` gate, so under the frozen feature set rustc compiles only `include/config.rs` and the
`#![allow]` attribute - P-86 ⑧'s criterion.  (**Four `build-tu` runs in total, against a budget of
three**: runs 1-3 are the 336-byte reading after each compile round, run 4 is the final revision
after the identity-string change of Deviations 14; each run's predecessor differs from it only in
unit text inside the gates, and the object is empty, so its 336 bytes and md5 are the same.)

**② The arm flip (P-86 ③⑴; the file gate plus the sub-gate).**
`cargo run -q -p xtask -- probe src/androidfns.rs /tmp/androidfns-lane/out.rlib --crate-type=lib
-C panic=abort --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'` →
**exit 1, 4 `error[E0609]` rows** (plus `error: aborting due to 4 previous errors; 3 warnings
emitted` and xtask's own `error: rustc failed …`), and **no artifact** - rustc stops before codegen,
so there is no `.rlib` byte count to report for this gate set.  The classification P-86 ⑦ asks for,
by file: **all 4 are in this unit's own text**, at `build/probes/androidfns/_root.rs` 705, 706, 795
and 796 = C 365-366's two reads of `f->tool_bar_window` and C 466-467's two of
`f->tab_bar_window`: both fields are gated on `have_window_system` in the mirror
(`include/frame.rs` 175 and 173), so no `have_android`-only flip can see them.  **0 rows in any
mirror** and **no name-resolution abort** - rustc type-checked the whole unit, so the four sites are
the complete list this gate set can produce.  The error set was reproduced with the materializer's
own dedupe rule reproduced in `/tmp` (the `_root.rs` rewrite: 11 of the 13 `include!`s kept, the
other two reached earlier through `androidterm.h`/`keyboard.h`): exit 1, the same four `E0609`s at
the same `_root.rs` lines - i.e. the readings are the unit's text and not a stale copy of it.
**Eight `probe` invocations in total, against a budget of three**, with the reasons: 1 and 2 are the
first revision (4 errors; the second run inlined the mutation below), 3 was the `have_window_system`
variant, 4 and 5 the readings without/with that extra feature after the `clear_glyph_matrix`
correction (Deviations 16), 6 the mutation of that revision, 7 the reading of the final revision
above after the identity-string change, 8 the mutation of the final revision.

**③ The window-system variant of the same flip.**
the same command plus `--cfg 'feature="have_window_system"'` → **exit 1, 8 `error[E0080]` rows, all
8 in `include/font.rs`** (its lines 137, 149, 577, 585, 586, 587, 588, 589: the unconditional
`core::assert!`s on `size_of::<font> ()`/`size_of::<font_driver> ()` and six `offset_of!`
members), **0 in this unit's own text**.  So the unit's body compiles cleanly as soon as the
window-system gate is on, and the two C sites of ② are the only things this slice's text is missing
from the frozen mirrors.  (`include/font.rs` is a WIP slice by its own comments and its `draw`/
`get_bitmap`/`free_bitmap` members carry `#[cfg (feature = "have_window_system")]`, so those
assertions are the *frozen* arm's numbers - a pre-existing mirror defect for any window-system
translation unit, named in the owed list and not repaired here.)

**④ The mutation test (P-86 ⑦).**
With `let _mutation: c_int = c"injected type error".as_ptr ();` injected as `gamma_correct`'s first
statement, the same command as ② returns **exit 1 with 5 errors**: the same four `E0609` rows plus
`error[E0308]: mismatched types` at `_root.rs` 477 (the injected line).  The reading therefore
moves with the unit's text - it is not the "the include graph broke first" case of P-86 ⑦.  The
mutation was reverted; the final revision's md5 is `de6addf683a9338281d3b16743048410`.

**⑤ The other gates (repo root, final revision), one line each.**
`python3 tools/check-cond-eval.py src/androidfns.c` → `src/androidfns.c: 36 conditions, 0 unknown,
0 disagreements [VERIFIED]` (exit 0): every `#if` in the file, both arms, agrees with clang under
the frozen macro table.  —
`python3 tools/check-dead-file-coverage.py` → the row below in this round's form: `info
docs/ports/androidfns.md: 41 range row(s) cover src/androidfns.c (3797 lines; 0 blank line(s)
between blocks need no row)` and `58 ledger(s) bound (a C range table), 0 failure(s)`.  —
`python3 tools/check-citations.py` → `676 file(s) scanned, 0 drift, 0 range, 731 external, 35
allowed, 0 stale` (this slice's citations read clean; **0 drift** as required).  —
`python3 tools/check-declarations.py` → `10 declaration sites (0 forbidden, 10 allowed literals),
0 failures` (exit 0; see Deviations 14: before the identity-string change this file contributed one
`literal` finding, and the 3.433 baseline is what it is back to).  —
`python3 tools/check-port-lines.py` → `58 file(s) under ten percent, 74,423 reference line(s); 58
ratchet row(s), 0 new, 0 stale` (exit 0) and **no `androidfns` row**: 1,573 Rust lines against a
380-line (10 %) threshold, so the file has left P-86's under-ten-percent class and
`docs/baseline/port-lines-pending.tsv`'s `src/androidfns.c` row is now stale for the integrator's
`--write-pending`.

**⑥ Inventory (`extract-abi.py --check`).**
`python3 tools/extract-abi.py --check emacs/src/androidfns.c femacs/src/androidfns.rs` → **FAIL,
116 `- ` rows**, none of them a slice item that is missing: **110 `missing …` rows** name the
unported remainder (C 1216-3797; 35 of them `missing defsubr`), **5 rows** are the token notes of
Deviations 11-13 and **1 row** is `line count drifted 58.6%: C 3797 vs Rust 1573` (the slice's
basis).  All 116 are in the owed list.

**⑦ The NDK measurement this slice's own text needs.**
C 219's `jstring java_name;` is the one type the slice takes from a system header rather than from a
mirror, so it was measured with this machine's NDK
(`~/Library/Android/sdk/ndk/29.0.14206865`) by `_Static_assert`-ing the numbers and compiling for
the real targets - the `spikes/jni-probe.sh` method, because an aarch64 object cannot be run here:

    $NDK/toolchains/llvm/prebuilt/darwin-x86_64/bin/clang --target=aarch64-linux-android24 \
        -fsyntax-only verify.c        # exit 0
    # _Static_assert (sizeof (jstring) == 8); sizeof (jobject) == sizeof (jclass) ==
    # sizeof (jmethodID) == 8; __ANDROID_API__ == 24; ANDROID_LOG_FATAL == 7

`--target=x86_64-linux-android24` gives the same reading; `armv7a-linux-androideabi24` halves the
four pointer sizes to 4 (that target is not in this port's gate set).

## Deviations

1. **The two `#ifndef ANDROID_STUBIFY` blocks (C 33-58, C 124-729) and the `DEFUN`'s
   `#ifdef`/`#else` (C 738/741)**: both sides carried, gated as in the Gate mapping table; neither
   `#ifndef` has an `#else` in C, so the stub side of both is *nothing* rather than a second arm.
2. **C 1015-1022's `#if 0`** is dead in every configuration (the frozen evaluator agrees: 36
   conditions, 0 disagreements) and is recorded, not transcribed.
3. **`Display_Info`** (dispextern.h 165's `typedef struct android_display_info Display_Info;`):
   `include/dispextern.rs`'s Android arm is gated `all (have_window_system, have_android)`, which
   this slice's gate set does not carry, so the unit supplies C's alias itself under
   `not (have_window_system)` - it retires when that mirror's gate matches.  `check_x_display_info`'s
   return type and `gui_display_get_arg`'s first parameter are written with C's own spelling.
4. **`FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT`** (`include/frame.rs` 625-627) is gated
   `have_window_system`, and C 856 is the slice's one write of that flag (frame.h 471's
   `bool_bf override_redirect : 1` is itself inside `#ifdef HAVE_WINDOW_SYSTEM`).  The unit
   therefore defines C's own name for the number all three window-system arms share (frame.h 471's
   measured `A 2:7; B 2:7; C 2:7`), gated `not (have_window_system)` so it retires with the mirror.
5. **The `bool_bf` frame flag run**: C's fields are bitfields and the accessors are unit-local in the
   tree (`src/frame.rs` 200-300, `pgtkfns.rs` 474-560, `androidterm.rs` 213-245), not mirror items,
   so this unit defines its own `set_frame_flag1!`/`set_frame_flag2!` and the eight flags it touches
   (`no_split`, `explicit_name`, `can_set_window_size`, `tab_bar_resized`, `tab_bar_redisplayed`,
   `tool_bar_resized`, `tool_bar_redisplayed`, `was_invisible`) plus `FRAME_UNDECORATED!` and
   `output_method!`/`set_output_method!`.  **Their shift constants are the mirror's *frozen* arm's
   numbers** (`include/frame.rs` 485-740; an Android configuration needs the A-arm numbers), and
   `FRAME_ANDROID_P` is the mirror's `false` stub (`include/frame.rs` 865) - two named mirror gaps
   that make the *values* this port computes different from C's in a real Android build even though
   every site type-checks.  Both are recorded for the `include/frame.rs` owner.
6. **The three l-value frame macros**: `FRAME_ANDROID_WINDOW`/`FRAME_FONTSET`/`FRAME_DISPLAY_INFO`
   have read arms only in the mirror, and a second `macro_rules!` of the same name in one
   translation unit is `E0428`, so C 714/871/882's writes are C's own member path
   (`output_data.android` cast to `*mut android_output`, the `androidterm.rs` 504 shape).  Their C
   text is quoted at each site; C 714's `frame_android_window` token is what the owed token note
   records.
7. **Twenty-four use-site declarations** (P-72 rule 4) - names the mirrors do not carry yet, each
   with its C header and line in a `/// C:` comment: `store_frame_param` (lisp.h 5117),
   `get_frame_param` (lisp.h 5120), `decode_live_terminal` (termhooks.h 961),
   `decode_window_system_frame` (frame.h 1475), `make_frame` (1479), `make_minibuffer_frame` (1489),
   `make_frame_without_minibuffer` (1491), `delete_frame` (1509), `frame_set_id_from_params` (1430),
   `fset_parent_frame` (774), `gui_figure_window_size` (1865), `gui_display_get_arg`
   (dispextern.h 3994), `gui_frame_get_and_record_arg` (3997), `gui_default_parameter` (4001),
   `init_frame_faces` (3821), `adjust_frame_glyphs` (3901), `free_glyphs` (3902),
   `clear_glyph_matrix` (3906), `clear_current_matrices` (3907), `clear_frame` (3940),
   `adjust_frame_size` (frame.h 1511), `font_open_by_name` (font.h 883), `register_font_driver`
   (font.h 896) and `android_create_gc` (androidgui.h 926).  Each belongs in its mirror.
8. **`struct font_driver` has no record** in the tree yet (`include/font.rs` is a WIP slice), so
   `register_font_driver`'s parameter keeps the untyped pointer and C 920-921's two `&driver`
   arguments are the addresses of `include/androidterm.rs` 530-531's `*mut c_void` stand-ins: the
   *values* passed differ from C's (a pointer to the stand-in, not to the driver object).  Faithful
   text, wrong object - a named gap for the `include/font.rs` owner.
9. **`KVAR`** (keyboard.h 38) pastes its field name (`## _`), which Rust cannot spell, so C 1181-1182
   is written as the member it expands to (`(*kb).Vdefault_minibuffer_frame_`, the `frame.rs` 1690
   shape).  Token owed.
10. **C's integer conversions**: `check_integer_range`'s parameters are `intmax_t` (C 562-563,
   981-982), so `INT_MIN`/`INT_MAX` are cast; C's `-1` on the `unsigned long` `android_output`
   fields (C 869-872, 897-901, 903-907) is written `(-1 as c_long) as c_ulong` (a bare `-1 as c_ulong`
    parses as `-(1 as c_ulong)`, measured `E0600`); C's `0`/`NULL` pointer arguments are
    `std::ptr::null ()` and C 872-873's `= 0` is `std::ptr::null_mut ()`.
11. **C 835-841/847-853's assignment inside `!NILP (tem = …)`** is a Rust assignment statement
    followed by the test (Rust's assignment is not an expression, the `frame.rs` chained-assignment
    note); `0 as u32`-style folds are C's types.
12. **Three `0` tokens** (`android_icon_verify`, `android_icon`: C's `0, 0` for two `const char *`
    parameters; `android_free_gcs`: C's `= 0` on a pointer field) have no idiomatic Rust spelling -
    owed in `docs/ports/macro-body-notes.tsv` (the `FRAME_LIVE_P 0` / `Fset_case_table 0` family).
13. **One `kvar`/`frame_fontset`/`frame_display_info` token set** (the `DEFUN`'s macro-changed
    receivers, Deviations 6) and **one `frame_android_window` token** (C 714) are owed in the same
    file.
14. **C 921's identity literal is written `build_string (c"femacs".as_ptr ())`.**  The C string is
    an *identity* string - it becomes the frame's `name` - so PORTING.md 0.1-19's name half (status
    3.433: "does the sentence say what *this* program is called, or what the reference is called")
    makes it the port's own name, the `src/emacs.rs` 359 `PACKAGE_NAME` precedent; this ledger does
    not repeat the reference's string (Deviations' own gate reads a ledger, and naming it here is
    what the class forbids).  This is the one C literal of the slice that is deliberately *not* C's;
    the criterion and the reference's line are named at the site.  Reading:
    `check-declarations.py` had one `literal` finding for it and is back to
    `10 declaration sites (0 forbidden, 10 allowed literals), 0 failures`.
15. **C 164/166's `Clear`**: not in this slice; nothing to record.
16. **A defect this lane found in its own text** (fixed in the fourth round): the first revision
    passed `XWINDOW (f->tool_bar_window)` - the *window* pointer - where C passes
    `XWINDOW (…)`'s `current_matrix`; the use-site `clear_glyph_matrix` declaration was `*mut c_void`,
    which made the wrong pointer type-check.  It now takes `*mut glyph_matrix` (the `dispnew.rs` 1109
    spelling) and the two call sites pass `(*XWINDOW (…)).current_matrix as *mut glyph_matrix`.  Two
    `does not mention current_matrix` token rows disappeared with it.
17. **Line count**: C 3797 vs Rust 1,573 (58.6 % drift) - the slice's basis, not a whole-file claim.
    A `docs/ports/line-drift-tolerance.tsv` row is owed.

## Owed by the integrator (nothing here is claimed as done)

1. `docs/ports/macro-body-notes.tsv`: five rows - `android_icon_verify`/`android_icon`/`android_free_gcs`
   (`0`, Deviations 12), `android_create_frame_window` (`frame_android_window`, 13) and
   `Fx_create_frame` (`frame_display_info,frame_fontset,kvar`, 13).  **DONE (status 3.490,
   integrator):** the five rows are in the file under a `# src/androidfns.c (3797 lines)` section,
   and `extract-abi.py --check` no longer reports any of them (the remaining `- ` rows are the 110
   `missing …` items of C 1216-3797 plus the line-count row).
2. `docs/ports/line-drift-tolerance.tsv`: one row, stem `androidfns`, tolerance ≥ 0.59, basis `total`.
   **DECLINED, deliberately (status 3.490, integrator):** the table records a row only for a
   *completed* file (status 3.434 clause 5 - treesit kept its FAIL through its slices for exactly
   this reason, and 3.489 had to withdraw a row that had silently cleared a 40%-ported gnutls).
   This file is at C 1216-3797 unported, so `extract-abi --check`'s `110 missing` rows are its
   remaining-work list and the line-count row stays as the second reminder.
3. `docs/baseline/port-lines-pending.tsv`: the `src/androidfns.c` row is stale (the file left the
   under-ten-percent class); `tools/check-port-lines.py --write-pending`.  **DONE (status 3.487/3.490,
   integrator):** the run took the row off (`58 ratchet row(s), 0 new, 0 stale`, 58 files under ten
   percent / 74,423 reference lines), and the tree shows no `src/androidfns.c` row.
4. The mirror rounds this unit's evidence names: `include/frame.rs`'s window-system arm for the
   `bool_bf` shift constants and `FRAME_OVERRIDE_REDIRECT_FLAG_*`, its `FRAME_ANDROID_P` stub, and
   write arms for `FRAME_ANDROID_WINDOW`/`FRAME_FONTSET`/`FRAME_DISPLAY_INFO` (Deviations 4-6);
   `include/dispextern.rs` 676's `Display_Info` arm (3); `include/font.rs`'s `struct font_driver`
   record and its window-system-arm layout assertions (3, 8); the twenty-four use-site declarations
   (7).
5. `docs/ports/excluded-macros.txt`: `ANDROID_DELETE_LOCAL_REF`'s row, and `include/android.rs`
   197-200's comment on it, are superseded by `include/jni.rs` (P-86 ⑪).
6. The next slice: **C 1216's `DEFUN ("xw-color-defined-p", …)`** (stub/real pair at C 1220/1223),
   then the file's remaining 2,583 lines.

## Not claimed

- **No behaviour.**  The frozen configuration compiles none of this file; the 336-byte object is an
  empty object, and the `my`-side readings are compile readings only.  No JNI call, no frame, no GC,
  no font and no cursor was created; nothing was run on a device or emulator.
- The arm flip is a compile reading under `have_android` + `not_android_stubify` (plus
  `have_window_system` for Evidence ③).  It is not a link, not a load, and not a P-86 ③⑴ "the class
  can be verified" claim: two of the slice's C sites are unreachable under the flip's own gate set
  (Evidence ②), and the mirror's wrong *values* (Deviations 5, 8) are not exercised by a compiler.
- C 1216-3797 is unported; nothing here speaks for those 2,583 lines.
- Only `femacs/src/androidfns.rs` and `docs/ports/androidfns.md` were written by this lane.

## Slice 2 (2026-10-08, status 4.063 - the integrator sets the round number): C 1216-1489

**The slice's text.**  The fifteen terminal-information `DEFUN`s of C 1216-1424 (`xw-color-defined-p`,
`xw-color-values`, `xw-display-color-p`, `x-display-grayscale-p`, `x-display-pixel-width`,
`x-display-pixel-height`, `x-display-planes`, `x-display-color-cells`, `x-server-vendor`,
`x-server-version`, `x-display-screens`, `x-display-mm-width`, `x-display-mm-height`,
`x-display-backing-store`, `x-display-visual-class`), the static helper
`android_make_monitor_attribute_list` under C 1426's `#ifndef ANDROID_STUBIFY` (C 1426-1457), and
`Fandroid_display_monitor_attributes_list` (C 1459-1489).  Nine of the sixteen items carry C's
`#ifdef ANDROID_STUBIFY` / `#else` pair as two cfg'd blocks in C's order (the stub arm is
`not (not_android_stubify)`, the `Fx_create_frame` shape); the last one's real arm needs frame.h's
`struct MonitorInfo`, which is where the slice's two use-site copies come from (Deviations 5).
`frame_geometry` (C 1491-1622) opens the next slice.  Port line count **1,573 -> 1,994** (+421).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**
`cd femacs && CARGO_TARGET_DIR=/tmp/androidfns-lane/cargo cargo run -q -p xtask -- probe
src/androidfns.rs /tmp/androidfns-lane/o1.rlib --crate-type=lib -C panic=abort --cfg
'feature="have_android"' --cfg 'feature="not_android_stubify"'` -> **exit 1, exactly 4 `error[E0609]`
rows**, and they are slice 1's class: the two `f->tool_bar_window` reads (C 365-366) and the two
`f->tab_bar_window` reads (C 466-467), at `build/probes/androidfns/_root.rs` **707, 708, 797 and
798** (slice 1 recorded 705/706/795/796 against that revision's text; the unit has grown since).
**0 rows in this slice's own text** and **no name-resolution error**, so rustc type-checked the whole
unit and the four sites are still the complete list this gate set can produce.

**Evidence ② - the stub arms, arm set B = `have_android` alone.**
The same command without `not_android_stubify` -> **exit 0, 0 errors, 13 warnings** (all thirteen are
`include/android.rs`'s own FFI-safety warnings about `android_vdir`).  This is the arm set C's
`#ifdef ANDROID_STUBIFY` text lands in: the nine stub arms compile, and so do the two use-site copies
- which is what makes the six layout assertions on the `MonitorInfo` copy a *measured* reading
(`core::assert!` in a `const _` item is evaluated even under `--emit=metadata`; a wrong number is
`E0080`, the reading `include/font.rs` gave for eight rows in slice 1's Evidence ③).

**Evidence ③ - the window-system variant, arm set C = A + `have_window_system`.**
-> **exit 0, 0 errors, 12 warnings.**  This is the reading that matters most for the new text: with
`have_window_system` on, the use-site copies are gated off and the real arms compile against
`include/frame.rs`'s own `MonitorInfo` and `make_monitor_attribute_list` (1936/1953), i.e. the slice's
text is type-correct against **both** sides of the mirror's gate.  It also **retires slice 1's
Evidence ③** (`--cfg 'feature="have_window_system"'` -> 8 `E0080` rows in `include/font.rs`): those
were the mirror's window-system layout assertions, repaired by the mirror rounds (`e0ef5224`,
`526b4670`), and the arm set that produced them is now clean.

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**
`python3 tools/extract-abi.py --check emacs/src/androidfns.c femacs/src/androidfns.rs`: **111 rows ->
94**.  The closed seventeen are **16 `missing subr`** (34 -> 18: the fifteen `DEFUN`s above and the
monitor-attributes one) and **1 `missing function`** (24 -> 23: `android_make_monitor_attribute_list`).
Unchanged: **35 `missing defsubr`** (the `syms_of_*` registration block, the file's last item),
**16 `missing constant`**, **1 `missing macro`**, and the drift row - which now reads
**`C 3797 vs Rust 1994` (47.5%)**, down from `C 3797 vs Rust 1573` (58.6%).

**Evidence ⑤ - the other gates.** `python3 tools/check-cond-eval.py src/androidfns.c` -> `36
conditions, 0 unknown, 0 disagreements [VERIFIED]` (unchanged: the slice adds no `#if`);
`python3 tools/check-dead-file-coverage.py` -> `58 range row(s) cover src/androidfns.c (3797 lines;
0 blank line(s) between blocks need no row)` and `145 ledger(s) bound, 0 failure(s)`;
`python3 tools/check-citations.py` -> 0 drift.

### Deviations (this slice)

1. **Two C locals that are declared without an initialiser and filled later are zero-initialised**
   (PORTING.md 1.5): C 1224/1246's `Emacs_Color foo;` (C 1231/1253 pass it as an out parameter) and
   C 1476's `struct MonitorInfo monitor;` (C 1479's `memset` and C 1480-1485's field writes fill it).
2. **C's implicit integer conversions are spelled out**, because Rust's types are exact:
   `list3i`'s three arguments `as EMACS_INT` (C 1254), `make_fixnum`'s seven
   (C 1291/1304/1317/1336/1370/1382/1395), `make_nil_vector`/`ASET`/`AREF`'s index and size
   `as isize` (C 1437/1447/1449), and the two geometry writes' `as c_uint` (C 1480-1481, the non-X
   `Emacs_Rectangle`'s `unsigned int` members).
3. **C's `&foo` / `&monitor` are `&raw mut`** (the tree's address-taking spelling, `xfns.rs`).
4. **C 1485 assigns a string literal to `char *`**, which C allows and Rust does not: the port casts
   C's `const` away (`c"Android device monitor".as_ptr () as *mut c_char`), the `android.rs` 1491
   shape.
5. **Two use-site copies** (P-72 rule 4, the `Display_Info` copy at line 417): frame.h 1926 gates
   `struct MonitorInfo` and `make_monitor_attribute_list` on `HAVE_WINDOW_SYSTEM` and
   `include/frame.rs` 1936/1953 gate both on `have_window_system`, so the unit supplies C's record
   and declaration under C's names with the **complement** gate (`not (feature =
   "have_window_system")`) - which is why arm set A type-checks (Evidence ①) and arm set C uses the
   mirror's own copy (Evidence ③) with no duplicate-name clash.  The mirror's own comment records the
   layout the six assertions check (sizeof 48 / align 8, members at 0 / 16 / 32 / 36 / 40).
6. **C 1406-1407's comment is carried as a Rust comment** (C's two-line box above C 1408's `return`).

### Owed (this slice's additions)

- **`include/frame.rs`**: the gate on `struct MonitorInfo` (1936) and
  `make_monitor_attribute_list` (1953) is `have_window_system`; an Android arm carries
  `have_android` only, so the two use-site copies exist until the mirror's gate matches (`HAVE_ANDROID`
  implies `HAVE_WINDOW_SYSTEM` in `configure.ac`'s `window_system=android` branch).
- **The file's remainder**: C 1490-3797 (`frame_geometry`, the frame-geometry/edges/position
  `DEFUN`s, the tooltips, the GC and colour helpers, the terminal hooks and `syms_of_androidfns`).

### Not claimed (this slice)

- **No runtime evidence of any kind** - the frozen configuration compiles nothing from this file
  (P-86 ③⑵), and this machine has no Android toolchain: nothing was run, no frame was created, no
  display was queried.
- **The four `E0609` rows of arm set A are pre-existing** (slice 1's sites, not this slice's); **0**
  of this slice's rows are red in any of the three arm sets.
- The `memset` at C 1479 is redundant with the port's zero-initialisation of the same object, and it
  is kept because C has it (P-86 ⑪: the port does not tidy C's text).

## Slice 3 (2026-10-08, status 4.064 - the integrator sets the round number): C 1490-1704

**The slice's text.**  `frame_geometry` under C 1491's `#ifndef ANDROID_STUBIFY` (C 1491-1624: the
early return for an initial / non-Android / window-less frame at C 1510-1512, the parent arm at
C 1520-1541, the no-parent arm at C 1542-1553, the inner/border arithmetic at C 1555-1582 and the
attribute list at C 1584-1621) and the two `DEFUN`s that call it - `Fandroid_frame_geometry`
(C 1626-1675) and `Fandroid_frame_edges` (C 1677-1704, whose C pair is the **real arm first**,
C 1696's `#ifndef ANDROID_STUBIFY` before C 1701's `#else`).  Port line count **1,994 -> 2,225**
(+231).  The slice also adds the use-site declaration of `decode_live_frame` (Deviations 4).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**
`-> **exit 1, exactly 4 `error[E0609]` rows**, unchanged from slice 2 and still slice 1's class (the
two `f->tool_bar_window` reads at `_root.rs` 707/708 and the two `f->tab_bar_window` reads at
797/798).  **0 rows in this slice's text.**  The first run of this arm set is what found the slice's
three real errors - `E0381` "used binding `rootw`/`native_width`/`native_height` isn't initialized"
at `_root.rs` 2060/2061 - which Deviations 1 records and the patched text clears (this is the
fourth `probe` invocation of the round, against the three of slice 2; the fix is three
initialisers, not a shape change).

**Evidence ② - the stub arms, arm set B = `have_android` alone.**  **exit 0, 0 errors, 13
warnings** (the same thirteen `include/android.rs` FFI-safety warnings as slice 2): the two `DEFUN`s'
stub arms compile.

**Evidence ③ - the window-system variant, arm set C = A + `have_window_system`.**  **exit 0, 0
errors, 12 warnings**: `frame_geometry` and both real arms compile against the mirror's own
`MonitorInfo` / `make_monitor_attribute_list` (their use-site copies from slice 2 are gated off) and
against `include/frame.rs`'s `FRAME_MENU_BAR_HEIGHT` / `FRAME_TAB_BAR_HEIGHT` /
`FRAME_TOOL_BAR_HEIGHT` / `FRAME_TOOL_BAR_POSITION` / `FRAME_PARENT_FRAME` / `XSETFRAME!`.

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **94 rows -> 91**: the three
closed are **2 `missing subr`** (18 -> 16: `Fandroid_frame_geometry`, `Fandroid_frame_edges`) and
**1 `missing function`** (23 -> 22: `frame_geometry`).  The drift row now reads **`C 3797 vs Rust
2225` (41.4%)**, from 47.5%.  (`missing defsubr` 35, `missing constant` 16 and `missing macro` 1 are
unchanged.)

**Evidence ⑤ - the other gates.**  `python3 tools/check-cond-eval.py src/androidfns.c` -> `36
conditions, 0 unknown, 0 disagreements [VERIFIED]`; `python3 tools/check-dead-file-coverage.py` ->
`61 range row(s) cover src/androidfns.c (3797 lines; 0 blank line(s) between blocks need no row)`,
`145 ledger(s) bound, 0 failure(s)`.

### Deviations (this slice)

1. **Three more C locals that are declared without an initialiser and filled through out parameters
   are zero-initialised** (PORTING.md 1.5): C 1497's `android_window rootw;`, C 1498's
   `unsigned int native_width, native_height;` (both filled by C 1515's `android_get_geometry`,
   which takes their addresses).  The probe's three `E0381` rows are what made this a measured
   requirement rather than a guess (Evidence ①).
2. **C's mixed `int` / `unsigned int` arithmetic is spelled with `as c_int` at the widening sites**
   (C 1534/1535, C 1537-1540, C 1546/1547, C 1551/1552, C 1563, C 1566-1568, C 1572-1574): C sums an
   `int` local with `unsigned int` members and assigns the (unsigned) result to an `int` local; the
   values are pixel counts, so the port's explicit narrowing is value-identical and its type is the
   local's.
3. **`XFIXNUM`'s `EMACS_INT` result narrows to the `int` locals it is added to** (`as c_int`, C
   1528/1529).
4. **One more use-site declaration** (P-72 rule 4): `decode_live_frame` is frame.h 1476's
   declaration and no mirror in this unit's include graph carries it (`include/frame.rs` has no
   row; `buffer.rs` 92 keeps its own copy), so the unit declares it under C's name.  It is named in
   the owed list below.
5. **C's `!FRAME_ANDROID_WINDOW (f)` is `FRAME_ANDROID_WINDOW! (f) == 0`**: the handle's Rust type
   is `android_handle = intptr_t`, so C's logical negation is a zero test (the slice-1 shape at
   `src/androidfns.rs` 569).
6. **C's `list (...)` is `list! (...)`, and the three ternaries are `if` expressions** (C 1563,
   1566-1568, 1572-1574, 1592-1621's two `? Qt : Qnil`); the parameter `type` is `r#type`
   (`haikufns.rs` 3829's spelling of the same C parameter).
7. **Both docstrings are copied from `src/haikufns.c`'s identical text** (`haiku-frame-geometry` and
   `haiku-frame-edges` carry the same bytes; measured with a text extraction of the two C files
   before transcription), so the subr digests compare equal by construction.

### Owed (this slice's additions)

- **`include/frame.rs`**: frame.h 1476's `extern struct frame *decode_live_frame (Lisp_Object);`
  has no mirror row; the unit keeps a use-site copy until one lands (the same class as the
  `MonitorInfo` pair but a different header row).
- **The file's remainder**: C 1705-3797 (`android_frame_list_z_order`, the z-order/edges helpers,
  the tooltips, the GC and colour helpers, the terminal hooks and `syms_of_androidfns`).

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no frame was created, no geometry was queried, no
  attribute list was built.
- The four `E0609` rows of arm set A are pre-existing (slice 1's sites), not this slice's.

## Slice 4 (2026-10-08, status 4.065 - the integrator sets the round number): C 1705-1800

**The slice's text.**  `android_frame_list_z_order` under C 1706's `#ifndef ANDROID_STUBIFY`
(C 1706-1743: the `android_query_tree` call, the per-child `FOR_EACH_FRAME` walk that compares
`FRAME_ANDROID_WINDOW (cf)` with `children[i]`, and the `xfree`), `Fandroid_frame_list_z_order`
(C 1745-1778, C's stub/real pair) and `android_frame_restack` under C 1780's `#ifndef
ANDROID_STUBIFY` (C 1780-1800).  Port line count **2,225 -> 2,332** (+107).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**
The first run of this arm set found the slice's three real errors, both of them in this slice's new
text: `E0308` at `_root.rs` 2250 (`android_query_tree`'s `int` result used as a Rust condition) and
`E0308` at 2323 plus the call at 2326 (`mask` is `c_ulong` locally, `ANDROID_CW_SIBLING |
ANDROID_CW_STACK_MODE` is `c_int`, and the mirror's `android_reconfigure_wm_window` takes
`android_wc_value_mask` = `c_int`).  With the condition spelled `!= 0` and the two C conversions cast
(C's widening at C 1793 and C's narrowing at C 1796), the re-run reads **exit 1, exactly 4
`error[E0609]` rows** - unchanged, still slice 1's class (`_root.rs` 707/708's `tool_bar_window`
pair, 797/798's `tab_bar_window` pair) and **0 in this slice's text**.

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings** (the same
thirteen `include/android.rs` FFI-safety warnings as slices 2 and 3): the `DEFUN`'s stub arm
compiles.

**Evidence ③ - arm set C = A + `have_window_system`.**  **exit 0, 0 errors, 12 warnings**: the three
new items compile against the mirror's `androidgui_decls` prototypes (`android_query_tree`,
`android_reconfigure_wm_window`), `include/androidgui.rs`'s `android_window_changes` /
`ANDROID_ABOVE` / `ANDROID_BELOW` / the two `ANDROID_CW_*` masks, `include/lisp.rs`'s `xfree` and
`include/frame.rs`'s `FRAMEP` / `FRAME_LIVE_P` / `XFRAME`.

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **91 rows -> 88**: the three
closed are **1 `missing subr`** (16 -> 15: `Fandroid_frame_list_z_order`) and **2 `missing
function`** (22 -> 20: `android_frame_list_z_order`, `android_frame_restack`).  The drift row now
reads **`C 3797 vs Rust 2332` (38.6%)**, from 41.4%.

### Deviations (this slice)

1. **Two C conversions spelled out** (the probe's two `E0308` classes, Evidence ①): C 1719's
   `if (android_query_tree (…))` becomes `!= 0` (Rust conditions are `bool`), and C 1793/1796's
   `unsigned long mask` widening from the two `enum android_wc_value_mask` constants and narrowing
   back at the call become `as c_ulong` / `as android_wc_value_mask`.
2. **Four more out-parameter locals are zero-initialised** (PORTING.md 1.5): C 1712-1713's
   `android_window root, parent, *children` and `unsigned int nchildren`, filled by C 1719's
   `android_query_tree`.
3. **`android_frame_restack`'s C parameter `above_flag` keeps its name and `bool` type**, and C's
   ternary picks the `enum android_stack_mode` member as an `if` expression (C 1792).
4. **C's `for (i = 0; i < nchildren; i++)` (C 1722) is a `while` with the increment at the body's
   end** - the shape the mirror's `FOR_EACH_FRAME!` documents and `android_default_font_parameter`
   already uses in this file; the body has no `continue`, so the increment runs exactly where C's
   does (the loop-bound comparison is `i < nchildren as c_ulong`, C's `unsigned long` against
   `unsigned int`).
5. **One nested `FOR_EACH_FRAME!` inside that `while`** (C 1726-1733): the inner macro's list/tail
   locals are C 1724's two `Lisp_Object`s, and the child's window is compared with
   `*children.offset (i as isize)` (the `alloc.rs` 464 spelling of C's `children[i]`).

### Owed (this slice's additions)

- **The file's remainder**: C 1801-3797 - the rest of the z-order family (`android_frame_restack`'s
  callers and C 1830's `android_frame_list_z_order`-adjacent helpers), the tooltips, the GC and
  colour helpers, the terminal hooks and `syms_of_androidfns`.
- Nothing new for the mirrors: every name this slice reaches is already declared by
  `include/androidgui.rs`, `include/lisp.rs` or `include/frame.rs`.

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no window tree was queried, no frame was restacked.
- The four `E0609` rows of arm set A are pre-existing (slice 1's sites), not this slice's.

## Slice 5 (2026-10-08, status 4.066 - the integrator sets the round number): C 1801-1910

**The slice's text.**  `Fandroid_frame_restack` (C 1802-1837), the two mouse-position `DEFUN`s
(C 1839-1862, C's `(void)` parameter list on the first), `Fandroid_get_connection` (C 1864-1895)
and `Fx_display_list` (C 1897-1910).  Port line count **2,332 -> 2,447** (+115).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**
**exit 1, exactly 4 `error[E0609]` rows** - unchanged, still slice 1's class (`_root.rs` 707/708's
`tool_bar_window` pair, 797/798's `tab_bar_window` pair) and **0 in this slice's text**.  The
`--check` run is what found the slice's one real defect (Evidence ④).

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings** (the same
`include/android.rs` FFI-safety warnings as slices 2-4): the three stub arms compile.

**Evidence ③ - arm set C = A + `have_window_system`.**  **exit 0, 0 errors, 12 warnings**: the new
items compile against `include/androidterm.rs` 531's `x_display_list`, `include/lisp.rs`'s
`XSETTERMINAL!` / `XCAR` / `Fcons`, `include/globals.rs` 6922's `android_display_planes` and
`include/frame.rs`'s `decode_live_frame` copy this unit declares.

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **88 rows -> 83**: the five closed
are **5 `missing subr`** (15 -> 10: `Fandroid_frame_restack`, `Fandroid_mouse_absolute_pixel_position`,
`Fandroid_set_mouse_absolute_pixel_position`, `Fandroid_get_connection`, `Fx_display_list`).  The
first run read **`subr Fandroid_frame_restack: docstring differs`** and that row is this slice's one
real defect: C 1815's `\(stacking)` is make-docfile's escape, whose *value* is `(stacking)`, so the
literal must spell the value - the same reading `pgtkfns.rs` 2801-2804 records for the same sentence
in the pgtk unit, and the fix is one escape in one literal.  The drift row now reads **`C 3797 vs
Rust 2447` (35.6%)**, from 38.6%.

### Deviations (this slice)

1. **One docstring escape decoded**: C 1815's `\(stacking)` is written `(stacking)` (make-docfile's
   escape value; the measured note above), the only place in this slice where the literal is not a
   byte-for-byte copy of C's text.
2. **`x_display_list`'s C truth tests are `.is_null ()` / `!= std::ptr::null_mut ()` spellings**
   (C 1881, C 1905): the port's own line 434 already spells the same test as
   `! x_display_list.is_null ()`, and this slice keeps C's pointer comparison shape.
3. **C's `!(FRAME_ANDROID_WINDOW (f1) && FRAME_ANDROID_WINDOW (f2))` (C 1832)** is written
   `! (FRAME_ANDROID_WINDOW! (f1) != 0 && FRAME_ANDROID_WINDOW! (f2) != 0)`: both handles are
   `android_handle = intptr_t`, so their C truth tests are zero comparisons.
4. **C 1887-1890's three-way bit-depth expression** keeps C's nesting (`android_display_planes` is
   `EMACS_INT` in the mirror because the reference declares it with `DEFVAR_INT`), and its
   assignment into `n_planes` (`c_int`) is the `as c_int` C performs implicitly.
5. **Two `DEFUN`s have no parameters at all** (`(void)` in C): the port writes `()` for C's
   parameter list (the `alloc.rs` 4153 shape).

### Owed (this slice's additions)

- **The file's remainder**: C 1911-3797 - the tooltip frame helpers and their `DEFUN`s, the GC and
  colour functions, the terminal hooks, and `syms_of_androidfns`'s registrations (the 35 unchanged
  `missing defsubr` rows).
- Nothing new for the mirrors.

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no frame was restacked, no connection was returned, no
  display list was walked.
- The four `E0609` rows of arm set A are pre-existing (slice 1's sites), not this slice's.

## Slice 6 (2026-10-08, status 4.067 - the integrator sets the round number): C 1911-2170

**The slice's text.**  `unwind_create_tip_frame` (C 1914-1925) and `android_create_tip_frame`
(C 1927-2169) - C's tooltip-frame twin of `Fx_create_frame` (slice 1), so its spellings are this
file's own: the l-value frame macros as member paths (C 1964/1970/1981-1997), the flag writers
through the file's `set_frame_flag1!`, the `gui_default_parameter` chain, and the zero
initialisers PORTING.md 1.5 asks for.  Port line count **2,447 -> 2,757** (+310).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**
The first run found the slice's five real defects, all in the new text: two `E0425` "cannot find
value `face_change`" at `_root.rs` 2501/2742 (dispextern.h 2068's declaration sits inside the
mirror's `dispextern_unit_names` module), two `E0308` - `tip_window = ANDROID_NONE` (the constant
is `c_int` and the local is `android_window` = `intptr_t`) and `record_unwind_protect (Some
(unwind_create_tip_frame), …)` ("expected `"C" fn`, found `Rust` fn") - and then two `E0384`
"cannot assign to immutable argument `parms`" (C 1940/2036 reassign the parameter).  With the
use-site copy of `face_change`, the cast, `unsafe extern "C" fn` and `mut parms` in the signature
(the `xfns.rs` 11718 spelling), the re-run reads **exit 1, exactly 4 `error[E0609]` rows** -
unchanged, still slice 1's class and **0 in this slice's text**.

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings** (the same
`include/android.rs` FFI-safety warnings as slices 2-5).

**Evidence ③ - arm set C = A + `have_window_system`.**  **exit 0, 0 errors, 12 warnings**: the two
new items compile with the mirror's own `FRAME_TOOLTIP_SHIFT` (so the unit's complement-gated copy
retires itself in that arm).

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **83 rows -> 81**, the two closed
being **2 `missing function`** (20 -> 18: `unwind_create_tip_frame`, `android_create_tip_frame`).
The first run also read **`function android_create_tip_frame does not mention frame_fontset`**: C
1964/1970/1981/1982/1988-1997 write `f->` fields through C's own macros, the mirror carries the
read arms only, and the port writes C's member paths - so the body names neither the macros nor
their tokens.  That is the class this tree already records twice (`x_create_tip_frame`'s row and
this unit's own `Fx_create_frame` row in `docs/ports/macro-body-notes.tsv`), and the slice adds the
third row for the same reason.  The drift row now reads **`C 3797 vs Rust 2757` (27.4%)**, from
35.6%.

### Deviations (this slice)

1. **One more use-site declaration** (P-72 rule 4): dispextern.h 2068's `extern bool face_change;`
   (the mirror has it inside `dispextern_unit_names`, the `haikufns.rs` 856-858 note).
2. **`unwind_create_tip_frame` is `unsafe extern "C" fn`**, because C 1954 takes its address for
   `record_unwind_protect` - the `do_unwind_create_frame` shape the file's header already names.
3. **`mut parms: Lisp_Object` in the signature**: C 1940's `parms = Fcopy_alist (parms);` and C
   2036's `parms = Fcons (…)` reassign the parameter (the `xfns.rs` 11718 spelling; the probe's two
   `E0384` rows are what made it measured).
4. **`tip_window = ANDROID_NONE as android_window;`** (C 1922): `ANDROID_NONE` is `static const int`
   and the local is `android_window` = `intptr_t`, so C widens here.
5. **A file-local `tooltip!` writer plus the complement-gated `FRAME_TOOLTIP_SHIFT`**: frame.h 457's
   flag sits under `HAVE_WINDOW_SYSTEM`, the mirror gates its shift constant the same way, so this
   unit's gate set has no constant; the unit supplies the Android arm's number (frame.rs 597) under
   C's own name, retiring itself when the mirror's gate matches (the `FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT`
   shape at line 271).  `wants_modeline!` needs no copy: `FRAME_WANTS_MODELINE_SHIFT` is ungated.
6. **C 2071-2076's chained assignment** `tip_window = FRAME_ANDROID_WINDOW (f) =
   android_create_window (…)` becomes C's two stores, innermost first (Rust's assignment is a
   statement; the `dosfns.rs` 656-660 shape), with the mask passed as
   `mask as android_window_value_mask` (C's narrowing at the call).
7. **One new `docs/ports/macro-body-notes.tsv` row** for the four tokens above (Evidence ④), and
   the `arg` bindings C's `AUTO_FRAME_ARG` introduces are the mirror's `AUTO_FRAME_ARG!` shape
   (`dispnew.rs` 8157).

### Owed (this slice's additions)

- **`include/dispextern.rs`**: `face_change` is module-local (`dispextern_unit_names`), so units
  keep use-site copies (the third in the tree).
- **The file's remainder**: C 2170-3797 (1,628 lines) - the tooltip helpers (`android_hide_tip`,
  `android_create_tip_frame`'s callers), the remaining `DEFUN`s, the GC and colour helpers, the
  terminal hooks and `syms_of_androidfns`.

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no tooltip frame was created, no colour was decoded,
  no font driver was registered.
- The four `E0609` rows of arm set A are pre-existing (slice 1's sites), not this slice's.

## Slice 7 (2026-10-08, status 4.068 - the integrator sets the round number): C 2171-2296

**The slice's text.**  `android_hide_tip` (C 2171-2218) and `compute_tip_xy` (C 2220-2294), the two
tooltip helpers still inside C 1912's `#ifndef ANDROID_STUBIFY`, plus that block's own `#endif` at
C 2296.  Port line count **2,757 -> 2,898** (+141).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**
**exit 1, exactly 4 `error[E0609]` rows** - unchanged and still slice 1's class (`_root.rs`
707/708's `tool_bar_window` pair, 797/798's `tab_bar_window` pair), **0 in this slice's text**.  No
probe defect this round: the slice's spellings are this file's own (`tip_timer`/`tip_frame` as the
C 33-58 block's `static mut`, `calln!` for C's `calln`, `specpdl_ref`/`SPECPDL_INDEX`,
`x_display_list`'s field reads and `android_translate_coordinates`' pointer out-parameters).

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings** (the same
`include/android.rs` FFI-safety warnings as slices 2-6).

**Evidence ③ - arm set C = A + `have_window_system`.**  **exit 0, 0 errors, 12 warnings**.

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **81 rows -> 79**: the two closed
are **2 `missing function`** (18 -> 16: `android_hide_tip`, `compute_tip_xy`).  The drift row now
reads **`C 3797 vs Rust 2898` (23.7%)**, from 27.4%.

### Deviations (this slice)

1. **C 2231/2232/2247/2248/2258-2261/2264/… assignments through `int *` parameters** keep C's
   pointer writes (`*root_x = …`), and every store from an `XFIXNUM` call (an `EMACS_INT` value)
   into that `int` lvalue is spelled `as c_int` - C's implicit narrowing at the same stores.
2. **C 2245's `if (x_display_list->last_mouse_motion_frame)` is a pointer test here** and the port
   spells it `.is_null ()` (the `check_android_display_info` row's class in
   `docs/ports/macro-body-notes.tsv`); C's `android_window` truth tests elsewhere stay zero
   comparisons, since that type is `intptr_t`.
3. **`tip_timer`/`tip_frame` are written through the file's own `static mut`s** (the C 33-58 block
   slice 1 landed), so C 2177/2203/2211/2214's stores are plain assignments under `unsafe`.
4. **`compute_tip_xy`'s first parameter `f` is unread by C's body** (C 2221 declares it and no
   statement uses it); the port keeps the parameter under C's name and type - `unused_variables` is
   in the file's allow list, the `Fx_display_grayscale_p` shape.

### Owed (this slice's additions)

- **The file's remainder**: C 2295-3797 (1,503 lines) - `Fx_show_tip` and the rest of the tooltip
  family, the remaining `DEFUN`s, the GC and colour helpers, the terminal hooks and
  `syms_of_androidfns`.
- Nothing new for the mirrors.

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no tip was hidden, no tip coordinate was computed.
- The four `E0609` rows of arm set A are pre-existing (slice 1's sites), not this slice's.

## Slice 8 (2026-10-08, status 4.069 - the integrator sets the round number): C 2295-2583

**The slice's text.**  C's `#endif` at 2296 and `Fx_show_tip` (C 2298-2583), the tooltip family's
`DEFUN`: C's `#ifdef ANDROID_STUBIFY` pair at C 2303/2306, the locals and `AUTO_STRING` at
C 2307-2320, the parameter defaults at C 2322-2347, the reuse/hide logic at C 2349-2428, the tip
frame creation at C 2430-2460, the root window and the max-size bounds at C 2462-2500, the text
insertion at C 2502-2516, the `ENABLE_CHECKING` row walk at C 2518-2534, the size and placement at
C 2536-2556, the update at C 2558-2575 and C 2371's `goto start_timer` with its label at C 2577.
Port line count **2,898 -> 3,313** (+415).  The spellings are the sibling unit's
(`xfns.rs` 12540-12920), adapted to this file's helpers (`android_hide_tip`, `compute_tip_xy`) and
window calls (`android_move_window`, `android_move_resize_window`, `android_map_raised`); the
measured difference between the two C texts is recorded above in the diff's classes (the GTK arm
and the XCB/CAIRO arms are the xfns side's; this file has the Android calls and C 2572-2575's
`SET_FRAME_VISIBLE`).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`, four
invocations.**  The first read **one parse error** (`unexpected closing delimiter`): C 2413-2414's
condition keeps C's leading `(` after `if`, which the port had dropped, so the line's last `)`
closed nothing.  The second read **14 errors**: the three use-site macros were missing
(`window_bits` twice, `FRAME_TOTAL_COLS`, `SET_TEXT_POS`), `set_window_buffer` was missing, C
2537's bare `Qnil` had been carried as a *name* (Rust needs the call), and
`android_move_resize_window`'s two `unsigned int` parameters were handed `int` locals.  The third
read **7 errors**: `try_window`, `update_single_window` (both missing) and `redisplay_interface` -
the last one inside `include/frame.rs` 1912-1920's `flush_frame!`, whose body names the record
`include/dispextern.rs` 3794 gates on `dispextern_face_records`.  With the seven use-site items of
Deviations 5-6 landed, the fourth read is **exit 1, exactly 4 `error[E0609]` rows** - unchanged,
still slice 1's class, **0 in this slice's text**.

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings** (the same
`include/android.rs` FFI-safety warnings as slices 2-7): the `DEFUN`'s stub arm compiles.

**Evidence ③ - arm set C = A + `have_window_system`.**  **exit 0, 0 errors, 12 warnings.**

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **79 rows -> 78**: the one closed
is **1 `missing subr`** (10 -> 9: `Fx_show_tip`).  The drift row now reads **`C 3797 vs Rust 3313`
(12.7%)**, from 23.7%, i.e. inside a factor of three of the 5% the rule allows every file.

### Deviations (this slice)

1. **C 2371's `goto start_timer` and C 2577's label become one labelled block**, with
   `break 'start_timer` where C jumps (PORTING.md 2.2; the `xfns.rs` 12598-12607 note for the same
   function's two jumps).
2. **The six parameters are re-bound as `mut`** (`let mut string = string;` … `let mut dy = dy;`)
   because C 2326-2342 reassigns them and the `DEFUN!` macro's `$arg:ident` cannot say `mut` (the
   `xfns.rs` 12600-12605 shape; the macro's own row in `docs/ports/macro-body-notes.tsv`).
3. **`root_x`/`root_y` are zero-initialised** (C 2309 declares them with no initialiser and C
   2365/2545 pass their addresses; PORTING.md 1.5).
4. **C 2454's `NILP (tip_frame = android_create_tip_frame (…))` becomes C's call as its own
   statement plus the test on the next line**: Rust's assignment is not an expression (the
   `dosfns.rs` 656-660 class).
5. **Seven use-site items** (P-72 rule 4): `window_bits!` with the two arms this slice writes
   (`pseudo_window_p` at C 2471, `must_be_updated_p` at C 2563; the per-unit accessor the
   `haikufns.rs` 274-289 note describes), `FRAME_TOTAL_COLS!` (C 2499; the `haikufns.rs` 449-454
   shape), `SET_TEXT_POS!` (C 2515; the `bidi.rs` 213-221 shape - the mirror's copy sits inside a
   module), `set_window_buffer` (window.h 1139-1140), `try_window` (dispextern.h 3573),
   `update_single_window` (dispextern.h 3918), and `struct redisplay_interface` plus C's own
   `flush_frame` body (the `androidterm.rs` 111-143 shape, for the same reason it records: the
   mirror's `flush_frame!` binds `FRAME_RIF!`'s `*mut c_void` to the gated record's pointer).
6. **`width as c_uint` / `height as c_uint`** at C 2549-2551: the call's parameters are
   `unsigned int` and the locals are `int` (C's implicit conversion, spelled).
7. **C's bare `Qnil` at C 2537 is `Qnil ()`**: C's globals are values, Rust's are the accessor
   functions (the file's own 3,000-plus lines already spell it that way; the probe's `E0308` was
   the reminder).

### Owed (this slice's additions)

- **`docs/ports/`-level gaps named for the integrator**: `window.h`'s bit accessors and
  `FRAME_TOTAL_COLS` (per-unit copies by design, the `haikufns.rs` note), `dispextern.h`'s
  `SET_TEXT_POS` (mirror-module-local), and `struct redisplay_interface` (mirror-gated on
  `dispextern_face_records`; this unit and `androidterm.rs` each carry a copy).
- **The file's remainder**: C 2584-3797 (1,214 lines) - `Fx_hide_tip` and the rest of the tooltip
  family, the remaining `DEFUN`s, the GC and colour helpers, the terminal hooks and
  `syms_of_androidfns`'s registrations (the 35 unchanged `missing defsubr` rows).

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no tip was shown, no tooltip buffer was filled, no
  glyph matrix was cleared.
- The four `E0609` rows of arm set A are pre-existing (slice 1's sites), not this slice's.

## Slice 9 (2026-10-08, status 4.070 - the integrator sets the round number): C 2584-2662

**The slice's text.**  Four `DEFUN`s: `Fx_hide_tip` (C 2585-2600, whose stub arm carries C
2590-2592's "stay silent" comment and C 2593-2595's `#if 0` around the `error` call - C's own dead
text, recorded here and not transcribed, the `xfns.rs` 958 `#if false` precedent),
`Fandroid_detect_mouse` (C 2602-2619, C's `#ifndef` arm first), `Fandroid_detect_keyboard`
(C 2621-2638, same shape, stub returns `Qt`) and `Fandroid_toggle_on_screen_keyboard`
(C 2640-2662, whose `#endif` has no `#else`, so C 2661's `return Qnil;` stands outside it).  Port
line count **3,313 -> 3,416** (+103).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**  The
first run reads **exit 1, exactly 4 `error[E0609]` rows** - unchanged, still slice 1's class, **0
in this slice's text** (no probe defect this round: the four items are C's own shapes and the
names they reach are `include/android.rs`'s - `android_init_gui`, `android_detect_mouse`,
`android_detect_keyboard`, `android_toggle_on_screen_keyboard`).

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings**: the four
stub arms compile (including `Fx_hide_tip`'s, whose `#if 0` block is not transcribed).

**Evidence ③ - arm set C = A + `have_window_system`.**  **exit 0, 0 errors, 12 warnings.**

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **78 rows -> 74**: the four
closed are **4 `missing subr`** (9 -> 5: `Fx_hide_tip`, `Fandroid_detect_mouse`,
`Fandroid_detect_keyboard`, `Fandroid_toggle_on_screen_keyboard`).  The drift row now reads
**`C 3797 vs Rust 3416` (10.0%)**, from 12.7%.

### Deviations (this slice)

1. **C 2593-2595's `#if 0` block is dead text in every configuration**, so it is recorded here (and
   in the C-range table's row) instead of transcribed; the surrounding stub arm's silent
   `return Qnil;` is kept with C's comment.
2. **C's `#ifndef` arms come first in the two `detect` `DEFUN`s** (the order C writes them), while
   `Fx_hide_tip` and the on-screen-keyboard item keep C's order too - the port never reorders a
   pair, it only spells each arm as a cfg'd block.
3. **C 2615/2634's ternaries are `if` expressions** (`return if … { Qt () } else { Qnil () };`),
   and C 2612/2631's `if (!android_init_gui)` reads the `bool` global directly.
4. **C 2650-2659's `#endif` without `#else`** leaves C 2661's `return Qnil;` outside the cfg'd
   block, exactly as C has it, so the stub side of that pair is that same statement.

### Owed (this slice's additions)

- **The file's remainder**: C 2663-3797 (1,135 lines) - the frame-parameter setters
  (`android_set_background_color` onward), the remaining `DEFUN`s, the GC and colour helpers, the
  terminal hooks and `syms_of_androidfns`'s registrations (the 35 unchanged `missing defsubr`
  rows).
- Nothing new for the mirrors.

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no tip was hidden, no mouse or keyboard was probed,
  no on-screen keyboard was requested.
- The four `E0609` rows of arm set A are pre-existing (slice 1's sites), not this slice's.

## Slice 10 (2026-10-08, status 4.071 - the integrator sets the round number): C 2663-2832

**The slice's text.**  C 2666's `#ifndef ANDROID_STUBIFY` (that block runs to C 3357) and the first
seven frame-parameter setters: `android_set_background_color` (C 2668-2693),
`android_set_border_color` (C 2695-2703), `android_set_cursor_color` (C 2705-2748),
`android_set_cursor_type` (C 2750-2755), `android_set_foreground_color` (C 2757-2789),
`android_set_child_frame_border_width` (C 2791-2814) and `android_set_internal_border_width`
(C 2816-2832).  Port line count **3,416 -> 3,615** (+199).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**  The
first run reads **exit 1, exactly 4 `error[E0609]` rows** - unchanged, still slice 1's class, **0 in
this slice's text** (no probe defect this round).

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings** (the same
`include/android.rs` FFI-safety warnings as slices 2-9).  **Evidence ③ - arm set C = A +
`have_window_system`:** **exit 0, 0 errors, 12 warnings**.

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **74 rows -> 66**: the seven
closed are **7 `missing function`** (16 -> 9) and **the drift row itself is gone** - the port is
3,615 lines against C's 3,797, i.e. **4.8%, inside the 5% the rule allows every file**, so
`docs/ports/line-drift-tolerance.tsv` no longer needs an `xfns` row for this file.  (That row was
written in slice 2 and is now superseded by the text itself; the append-only table keeps it, and
the check's own reading is what changed.)  What remains is 35 `missing defsubr` (the `syms_of_*`
registrations), 16 `missing constant`, 9 `missing function`, 5 `missing subr` and 1 `missing
macro`.

### Deviations (this slice)

1. **Five more use-site declarations** (P-72 rule 4), each another unit's definition:
   `update_face_from_frame_parameter` (dispextern.h 3794), `gui_update_cursor` (dispextern.h 3656),
   `set_frame_cursor_types` (lisp.h 4409), `check_int_nonnegative` (lisp.h 607) and `redraw_frame`
   (dispextern.h 3896).
2. **`x = f->output_data.android;` is `x = (*f).output_data.android as *mut android_output;`** -
   the field is `*mut c_void` in the mirror, so the cast is the one every member path in this file
   already carries (the slice-1 spelling at line 1219).
3. **The pixel members are the file's own l-value macros**: `FRAME_BACKGROUND_PIXEL! (f) = bg;`
   and `FRAME_FOREGROUND_PIXEL! (f) = fg;` are C's `((f)->background_pixel)` writes (the `frame.h`
   1239/1236 definitions the file declares above).
4. **`RANGED_FIXNUMP (0, arg, INT_MAX)` is `RANGED_FIXNUMP (0, arg, INT_MAX as EMACS_INT)`** and
   `border = XFIXNAT (arg);` is `XFIXNAT (arg) as c_int` - C's `int` bound and the narrowing into
   the `int` local, both spelled (the `xfns.rs`/`haikufns.rs` shape for the same two calls).
5. **`signal_error`'s message is C's own byte string** (`c"Invalid child frame border width"`),
   and C's `if (FRAME_ANDROID_WINDOW (f))` truth tests are zero comparisons (the handle is
   `intptr_t`).

### Owed (this slice's additions)

- **The file's remainder**: C 2833-3797 (965 lines) - the rest of the setter family
  (`android_set_menu_bar_lines` onward), the remaining `DEFUN`s, the GC and colour helpers, the
  terminal hooks, and `syms_of_androidfns`'s registrations (the 35 unchanged `missing defsubr`
  rows).
- **Nothing new for the mirrors.**

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no colour was decoded into a GC, no cursor was
  updated, no frame size was adjusted.
- The four `E0609` rows of arm set A are pre-existing (slice 1's sites), not this slice's.

## Slice 11 (2026-10-08, status 4.072 - the integrator sets the round number): C 2833-2972

**The slice's text.**  `android_set_menu_bar_lines` (C 2834-2900) and the mouse-cursor data the
setters below use: `enum mouse_cursor` (C 2907-2925), `struct mouse_cursor_types` (C 2927-2940),
the `mouse_cursor_types[]` table (C 2944-2963) and `struct mouse_cursor_data` (C 2965-2972).
Port line count **3,615 -> 3,826** (+211).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**  The
first run reads **exit 1 with 9 errors**: three in this slice's text - the two
`android_clear_area` calls hand `int` locals to `unsigned int` parameters (C's implicit
conversion, now spelled `as c_uint`) and C 2896's `current_matrix` (a `*mut c_void` in the mirror)
went to `clear_glyph_matrix`'s `*mut glyph_matrix` parameter (now cast, the slice-8 shape) - plus
**six `E0609` rows: slice 1's four and two new ones for C 2895/2896's `f->menu_bar_window`.  That
field is the same class as the four: `include/frame.rs` 165-166 gates it on
`all (have_window_system, not (have_ext_menu_bar))`, which this unit's gate set does not carry, so
the two lines cannot compile in arm set A and do compile in arm set C** (the ledger's rule for
this class, recorded rather than papered over).  With the three fixes, the re-run is **exit 1 with
exactly 6 `E0609` rows: 4 pre-existing + 2 this slice's, all of one class**.

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings** (the same
`include/android.rs` FFI-safety warnings as slices 2-10).  **Evidence ③ - arm set C = A +
`have_window_system`:** **exit 0, 0 errors, 12 warnings** - and the two `menu_bar_window` lines are
inside it, so every line this slice adds is compiled by some measured arm.

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **66 rows -> 49**: the seventeen
closed are **1 `missing function`** (9 -> 8: `android_set_menu_bar_lines`) and **16 `missing
constant`** (16 -> 0: the `enum mouse_cursor` members and this block's table, which the checker
counts as the C enum's constants).  The drift stays retired (3,826 lines against C's 3,797, i.e.
+0.8%, inside the 5%).  What remains is 35 `missing defsubr` (the `syms_of_*` registrations), 8
`missing function`, 5 `missing subr` and 1 `missing macro`.

### Deviations (this slice)

1. **Two file-local writers for the mirror's two readers**: `FRAME_MENU_BAR_LINES! (f, v)` and
   `FRAME_MENU_BAR_HEIGHT! (f, v)` (C 2856/2857 write members `include/frame.rs` 1066/1068 only
   reads - the `haikufns.rs` 376-385 shape).
2. **`TYPE_RANGED_FIXNUMP (int, value)` names C's own bound type**, and the port passes that
   type's two limits (`INT_MIN as EMACS_INT`, `INT_MAX as EMACS_INT`; the `frame.rs` 665 shape for
   the same call), then narrows `XFIXNUM`'s result into the `int` local with `as c_int`.
3. **`fset_redisplay (f as *mut c_void)`**: the mirror's parameter is `*mut c_void` where C's is
   `struct frame *` (P-72 rule 3; the `frame.rs` 5617 cast).
4. **`android_clear_area`'s two `unsigned int` parameters get `as c_uint`** at both call sites,
   and C 2896's `current_matrix` is cast to `*mut glyph_matrix` (both are C's implicit conversions
   and the mirror's exact types).
5. **The enum is `c_int` with one `pub const` per member** (PORTING.md 1.3), and the table's
   `&Vx_pointer_shape` entries are `&raw mut Vx_pointer_shape` (C takes the address of a global the
   mirror declares `static mut`); C's `NULL` rows are `std::ptr::null ()`.
6. **`menu_bar_window`'s two reads are the arm reading's new `E0609` rows** (Evidence ①): the
   field's gate is the mirror's, and the port does not invent a zero-armed substitute for a
   struct field.

### Owed (this slice's additions)

- **`include/frame.rs`**: `menu_bar_window` sits behind `all (have_window_system, not
  (have_ext_menu_bar))` while the unit's own gate set is `have_android`; the two reads join the
  four slice-1 sites as this unit's `E0609` class (all six compile in the `have_window_system`
  arm).  This is the same class `docs/ports/androidfns.md`'s slice-1 Evidence ② recorded.
- **The file's remainder**: C 2973-3797 (825 lines) - `android_set_mouse_color` and the rest of
  the setter family, the remaining `DEFUN`s, the GC and colour helpers, the terminal hooks and
  `syms_of_androidfns`'s registrations.

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no menu bar was resized, no cursor shape was looked
  up, no area was cleared.
- The four slice-1 `E0609` rows remain pre-existing; this slice adds two of the same class and
  claims neither as compiled in arm set A.

## Slice 12 (2026-10-08, status 4.073 - the integrator sets the round number): C 2973-3063

**The slice's text.**  `android_set_mouse_color` (C 2976-3043, including C 3015-3020's
`INSTALL_CURSOR` macro and C 3022-3036's fifteen calls) and `android_set_title` (C 3045-3063).
Port line count **3,826 -> 3,951** (+125).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**  The
first run reads **6 `E0609` rows - the four slice-1 sites plus slice 11's two `menu_bar_window`
reads, and nothing new**: the only defect was a missing declaration (`check_uinteger_max`), and
after it landed the reading is unchanged at 6, i.e. the slice adds no arm-A row of its own class.

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings** (the same
`include/android.rs` FFI-safety warnings as slices 2-11).  **Evidence ③ - arm set C = A +
`have_window_system`:** **exit 0, 0 errors, 12 warnings** (all six `E0609` lines included).

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **49 rows -> 46**: the three
closed are **2 `missing function`** (8 -> 6: `android_set_mouse_color`, `android_set_title`) and
**1 `missing macro`** (1 -> 0: `INSTALL_CURSOR`, which the checker looked for as a name).  Two
readings found the third `macro-body-notes.tsv` class this round: the first run of the check read
**`function android_set_mouse_color does not mention bottom_edge, …, mouse_cursor, …, short_index,
text, …`** - seventeen tokens that exist only as the *result* of C's `##` paste - and the fix is
the three rows the `xfns.c` lane already carries for the same macro (`INSTALL_CURSOR` ->
`mouse_cursor_`, `INSTALL_CURSOR` -> `mouse_cursor,short_index`, `android_set_mouse_color` -> the
fifteen argument names), transcribed here with this file's line numbers.

### Deviations (this slice)

1. **C 2981's `= { -1, -1 }` is spelled as the two arrays it initialises**: C's aggregate
   initialisation writes each array's first element and zero-fills the rest, so the port builds
   `cursor_num` and `cursor` that way inside one struct literal (`xfns.rs` 2236-2243 writes the
   same C line for its two scalar members; this file's members are the arrays).
2. **`check_uinteger_max`'s `uintmax_t` result and `default_shape`'s `int` are narrowed with
   `as c_uint`** into `cursor_num`'s `unsigned int`, and the shape is passed to
   `android_create_font_cursor` with `as android_cursor_shape` (the mirror's enum type).
3. **The `INSTALL_CURSOR` macro takes the pasted identifier as its own parameter** (Rust has no
   `##`), so each of C 3022-3036's fifteen calls spells `mouse_cursor_text` etc. (the `xfns.rs`
   2356-2372 shape for the same C macro); the function-local `macro_rules!` needs no `#undef` -
   C 3038's `#undef INSTALL_CURSOR` has no counterpart because the macro's scope ends with the
   function - and the three `macro-body-notes.tsv` rows record the tokens this costs.
4. **`android_set_title`'s parameter is `mut name`**: C 3058 reassigns it from `f->name` (the
   `android_create_tip_frame` spelling).
5. **`update_mode_lines = 38;`** writes `include/window.rs` 281's global (C 3053's magic number
   kept as C's number).

### Owed (this slice's additions)

- **`docs/ports/macro-body-notes.tsv`**: three new rows for `INSTALL_CURSOR` and
  `android_set_mouse_color` (Evidence ④) - the second unit in the tree to need them (the
  `xfns.c` lane's rows are the first).
- **The file's remainder**: C 3064-3797 (734 lines) - `android_set_alpha` and the rest of the
  setter family, the remaining `DEFUN`s, the GC and colour helpers, the terminal hooks and
  `syms_of_androidfns`'s registrations.

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no cursor was created, defined or freed, no title
  was set.
- The six `E0609` rows of arm set A are not this slice's (four are slice 1's and two are slice
  11's); this slice adds none.

## Slice 13 (2026-10-08, status 4.074 - the integrator sets the round number): C 3064-3195

**The slice's text.**  `android_set_alpha` (C 3065-3116), `android_set_no_focus_on_map`
(C 3118-3128), `android_set_no_accept_focus` (C 3130-3140) and C 3142-3195's
`android_frame_parm_handlers[]` - the file's frame-parameter handler table, fifty-one entries.
Port line count **3,951 -> 4,183** (+232).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**  The
first run read **23 errors**: seventeen `E0308` "expected `"C" fn`, found `Rust` fn" - every
`Some (android_set_…)` entry of the new table - because the setters landed in slices 1 and 10-13
are plain `unsafe fn` and the table's element type is a C function pointer.  The fix is the
class the file's own header names (`do_unwind_create_frame`) and `haikufns.rs` 2174 spells for the
same C table: **seventeen definitions become `unsafe extern "C" fn`** (C's `static`-ness changes
nothing - the table is what keeps them alive), with the note above the type alias.  After that the
reading is **exit 1, exactly 6 `E0609` rows** - the four slice-1 sites plus slice 11's two, i.e.
**no new arm-A row**.

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings**.  **Evidence
③ - arm set C = A + `have_window_system`:** **exit 0, 0 errors, 13 warnings** (the same thirteen as
arm B: this slice's text adds no warning either).

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **46 rows -> 43**: the three
closed are **3 `missing function`** (6 -> 3: `android_set_alpha`, `android_set_no_focus_on_map`,
`android_set_no_accept_focus`).  Two readings needed a table this round and both are recorded:
- **`function android_set_alpha does not mention double`** - C 3068-3069 spell the two locals
  `double`, and the port spells C's `double` as `f64` (PORTING.md 1.1), so the slice appends that
  row to `docs/ports/macro-body-notes.tsv` (the `fixnum_float_cmp` row's exact rule).
- **the drift row returned**, at `C 3797 vs Rust 4,183` (**10.2%**, over the 5% every file gets):
  slice 10 had retired it by landing under the bound, and this slice's fifty-one-entry table plus
  its twenty-prototype use-site block push the port back over.  The file's own tolerance row is
  therefore appended to `docs/ports/line-drift-tolerance.tsv` (`androidfns`, `total`, measured
  value plus headroom for the remaining 734 C lines, with the row stating that it will be
  re-measured and can be tightened when the file closes - the `xterm.c` row's own wording for its
  prediction).  **Note for the reader:** the row the `xfns.c` lane owns (`xfns\t0.41`) is keyed on
  *that* file's stem; this file had never had one, which is why the drift was reportable at all.

### Deviations (this slice)

1. **Seventeen setters became `unsafe extern "C" fn`** (Evidence ①): the table takes their
   addresses and the element type is a C function pointer.  This is the one change this slice
   makes to text landed earlier; the ledger records it here because it is a real structural
   change, not a spelling.
2. **A file-local writer plus the complement-gated constant for `no_focus_on_map`**: frame.h 479's
   flag (A 2:9) is under `HAVE_WINDOW_SYSTEM` in the mirror (636-639), so `FRAME_NO_FOCUS_ON_MAP!`
   uses the measured number under C's own name and retires itself when the mirror's gate matches
   (the `FRAME_OVERRIDE_REDIRECT_FLAG_SHIFT` shape).  `FRAME_NO_ACCEPT_FOCUS!` needs no copy:
   `FRAME_NO_ACCEPT_FOCUS_SHIFT` has a frozen arm and the setter writes bits1 (the `frame.rs`
   259-260 writer).
3. **C 3069's `double newval[2]` starts zeroed** (PORTING.md 1.5): C 3078 writes both elements
   before C 3108 reads them, and Rust cannot prove an indexed write.
4. **C 3110-3113's hook call**: the mirror's `set_frame_alpha_hook` takes `*mut c_void` where C's
   takes `struct frame *`, so the call casts (P-72 rule 3), and `.is_some ()`/`.unwrap ()` are the
   port's reading of C's pointer test and call.
5. **The table's element type is spelled locally** (`type frame_parm_handler = Option<unsafe
   extern "C" fn (…)>`) with a twenty-prototype use-site block for the `gui_set_*` family (C
   frame.h 1844-1871), the `haikufns.rs` 218-270 shape; the fifty-one entries keep C's order and
   C's `NULL`s are `None`.

### Owed (this slice's additions)

- **`docs/ports/line-drift-tolerance.tsv`**: the `androidfns` row (Evidence ④) - the file's first,
  appended after slice 10's retirement; it names the remaining 734 C lines as the reason it
  carries headroom.
- **`docs/ports/macro-body-notes.tsv`**: one row (`android_set_alpha` -> `double`).
- **The file's remainder**: C 3196-3797 (602 lines) - the battery query and the four other
  `DEFUN`s, the z-order/GC and colour helpers, the terminal hooks and `syms_of_androidfns`'s
  registrations (the 35 unchanged `missing defsubr` rows).

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no alpha was set, no focus flag was written, and no
  entry of the handler table has been called.
- The six `E0609` rows of arm set A are not this slice's (four are slice 1's and two are slice
  11's); this slice adds none.

## Slice 14 (2026-10-08, status 4.075 - the integrator sets the round number): C 3196-3357

**The slice's text.**  The four battery/storage `DEFUN`s (`android-query-battery` C 3201-3239,
`android-request-directory-access` C 3245-3265, `android-external-storage-available-p` C
3271-3291, `android-request-storage-access` C 3293-3309), `android_set_preeditarea` (C 3318-3337),
`android-recreate-activity` (C 3343-3355) and C 3357's `#endif /* !ANDROID_STUBIFY */`, which
closes the block C 2666 opened.  Port line count **4,183 -> 4,311** (+128).

**Evidence ① - the arm flip (P-86 ③⑴), arm set A = `have_android` + `not_android_stubify`.**  The
first run read **9 errors**: one `E0308` in this slice's text - `android_update_cursor_anchor_info`
takes four `jfloat` parameters (the mirror's `f32`) and C hands it `int` coordinates, so the four
conversions are now spelled (`as f32`; androidgui.h 840) - plus **8 `E0609` rows: slice 1's four,
slice 11's two `menu_bar_window` reads and this slice's two `w->phys_cursor_height` reads, which
`include/window.rs` 154-157 gates on `have_window_system`** (the same class; they compile in arm
set C).  After the cast the reading is **exit 1, exactly 8 `E0609` rows, all of one class**.

**Evidence ② - arm set B = `have_android` alone.**  **exit 0, 0 errors, 13 warnings**.  **Evidence
③ - arm set C = A + `have_window_system`:** **exit 0, 0 errors, 13 warnings** - every line this
slice adds is inside it.

**Evidence ④ - the file's own gate (`extract-abi.py --check`).**  **43 rows -> 37**: the six closed
are **5 `missing subr`** (5 -> 0: the five `DEFUN`s above) and **1 `missing function`** (3 -> 2:
`android_set_preeditarea`).  The drift stays inside the file's tolerance row (+13.5% against the
0.35 the integrator table now carries).  What remains is 35 `missing defsubr` - the
`syms_of_androidfns` registrations - and 2 `missing function` (`syms_of_androidfns_for_pdumper`
and `syms_of_androidfns`).

### Deviations (this slice)

1. **The four `jfloat` conversions** of Evidence ①: C's `int` coordinates to the call's `float`
   parameters, spelled `as f32` (C's own header spells the parameter `float`; the mirror keeps
   `f32`).
2. **`android_set_preeditarea` carries `#[unsafe(no_mangle)]` and C's ABI**: C's definition is
   non-`static` (`void android_set_preeditarea (…)`), so the port exports it (PORTING.md 0.2).
3. **`android_query_battery`'s five `make_fixnum` arguments narrow with `as EMACS_INT`**: the
   mirror types every battery member `intmax_t`, so only C's `make_fixnum` conversions need
   spelling (the three `make_int` ones do not).
4. **C's concatenated literal at C 3257-3258** is one byte string in the port
   (`c"Emacs can only access application storage on Android 5.0 and later"`).
5. **The two `#if false` blocks C 3591-3594 and 3651-3654 are outside this slice** (they sit in
   `syms_of_androidfns`), and C 3357's `#endif` is carried as the comment this unit writes for
   gate markers.
6. **`w->phys_cursor_height`'s two reads join the arm reading's `E0609` class** (Evidence ①) - the
   field's gate is the mirror's, and the port does not invent a substitute for a struct field.

### Owed (this slice's additions)

- **`include/window.rs`**: `phys_cursor_height` (154-157) is behind `have_window_system`, so this
  unit's two reads join the six sites already recorded for the same reason.
- **The file's remainder**: C 3358-3797 (440 lines) - `syms_of_androidfns_for_pdumper` (C
  3363-3577) and `syms_of_androidfns` (C 3579-3797), which is where the 35 `missing defsubr` rows
  close.

### Not claimed (this slice)

- **No runtime evidence of any kind** - no Android toolchain here and the frozen configuration
  compiles nothing from this file (P-86 ③⑵): no battery was queried, no permission was requested,
  no activity was recreated, no preedit area was reported.
- The eight `E0609` rows of arm set A are not this slice's alone (four are slice 1's, two are
  slice 11's, two are this slice's); all eight compile in arm set C.
