# emacs/src/haikuterm.c (4898 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `configure.ac` 3097
> clears `HAIKU_OBJ` and 3114 fills it with `haikuterm.o` under `test "${HAVE_BE_APP}" = "yes"` -
> the same test that `AC_DEFINE`s `HAVE_HAIKU` (3106-3110) - and `build/emacs-ref/src/Makefile`
> 378 reads `HAIKU_OBJ = ` (empty), so `docs/baseline/frozen-build-sources.txt` lists no
> `haikuterm.o` for this target and `extract-abi.py`'s checklist for the file is empty (P-86 ②).
> What follows is that clause's substitute: the **C line-range table**, the `## Evidence`
> readings, the `## Deviations` and the per-slice sections at this file's foot.  Port:
> `femacs/src/haikuterm.rs` (**4,445 lines**), which carries **C 1-3138** - P-86 ④'s first
> nineteen slices of this file - with every `#if`'s arms both carried.
>
> **Why 1044 and not 1000 (slice 1).**  C 1000 is inside `haiku_draw_text_decoration`
> (C 887-1044); the nearest whole-item boundaries are this item's `}` at C 1044 (44 lines past
> 1000) and the previous item's `}` at C 885 (115 lines short), so the slice is **C 1-1044**.
>
> **Why 3138 and not 3140 (slice 19).**  C 3140 opens the `redisplay_interface` table's definition
> and C 3138 the closing brace of the item before it, so the slice is **C 3092-3138** - the blank
> line C 3092 and `haiku_default_font_parameter` (C 3093-3138).  The next definition is the table
> itself (C 3140-3170), which is the ABI round Owed 1 names.
>
> **Why 3091 and not 3093 (slice 18).**  C 3093 is `haiku_default_font_parameter`'s `static void`
> and C 3091 the closing brace of the item before it, so the slice is **C 3078-3091** - the blank
> line C 3078 and `haiku_define_frame_cursor` (C 3080-3091).  The next definition is
> `haiku_default_font_parameter` (C 3093); the `redisplay_interface` table follows at C 3140-3170
> and is the ABI round Owed 1 names.
>
> **Why 3077 and not 3080 (slice 17).**  C 3080 is `haiku_define_frame_cursor`'s `static void` and
> C 3077 the closing brace of the item before it, so the slice is **C 2981-3077** - the blank line
> C 2981 plus the C 2982-2987 comment block, `haiku_mouse_position` (C 2988-3061) and
> `haiku_flush` (C 3063-3077).  The next definition is `haiku_define_frame_cursor` (C 3080).
>
> **Why 2980 and not 2982 (slice 16).**  C 2981 is blank and C 2982 opens a comment block, so the
> slice is **C 2893-2980** - the blank line C 2893 and the three items `haiku_define_fringe_bitmap`
> (C 2894-2915), `haiku_destroy_fringe_bitmap` (C 2917-2926) and `haiku_scroll_run` (C 2928-2980).
> The next definition follows the C 2982 comment block.
>
> **Why 2892 and not 2894 (slice 15).**  C 2894 is `haiku_define_fringe_bitmap`'s `static void` and
> C 2892 the closing brace of the item before it, so the slice is **C 2804-2892** - the blank line
> C 2804 and `haiku_draw_fringe_bitmap` (C 2805-2892), which is the first text in this file to need
> the `draw_fringe_bitmap_params` record and `p->cursor_p`/`p->overlay_p`.  The next definition is
> `haiku_define_fringe_bitmap` (C 2894).
>
> **Why 2803 and not 2805 (slice 14).**  C 2805 is `haiku_draw_fringe_bitmap`'s `static void` and
> C 2803 the closing brace of the item before it, so the slice is **C 2746-2803** - the blank line
> C 2746 and `haiku_set_vertical_scroll_bar` (C 2747-2803), the vertical twin of slice 13's
> horizontal function.  The next definition is `haiku_draw_fringe_bitmap` (C 2805).
>
> **Why 2745 and not 2747 (slice 13).**  C 2747 is `haiku_set_vertical_scroll_bar`'s `static void`
> and C 2745 the closing brace of the item before it, so the slice is **C 2646-2745** - the blank
> line C 2646, `haiku_scroll_bar_create` (C 2647-2690) and `haiku_set_horizontal_scroll_bar`
> (C 2692-2745).  The next definition is `haiku_set_vertical_scroll_bar` (C 2747).
>
> **Why 2645 and not 2647 (slice 12).**  C 2647 is `haiku_scroll_bar_create`'s `static struct
> scroll_bar *` and C 2645 the closing brace of the item before it, so the slice is **C 2523-2645**
> - the blank line C 2523 and the three items `haiku_condemn_scroll_bars` (C 2524-2544),
> `haiku_redeem_scroll_bar` (C 2546-2620, C 2564's `goto horizontal` included) and
> `haiku_judge_scroll_bars` (C 2622-2645).  The next definition is `haiku_scroll_bar_create`
> (C 2647).
>
> **Why 2522 and not 2524 (slice 11).**  C 2524 is `haiku_condemn_scroll_bars`'s `static void` and
> C 2522 the closing brace of the item before it, so the slice is **C 2398-2522** - the blank line
> C 2398 and the five items `haiku_compute_glyph_string_overhangs` (C 2399-2428),
> `haiku_draw_vertical_window_border` (C 2430-2446), `haiku_set_scroll_bar_default_width`
> (C 2448-2458), `haiku_set_scroll_bar_default_height` (C 2460-2470) and `haiku_draw_window_divider`
> (C 2472-2522).  The next definition is `haiku_condemn_scroll_bars` (C 2524).
>
> **Why 2397 and not 2399 (slice 10).**  C 2399 is `haiku_compute_glyph_string_overhangs`'s
> `static void` and C 2397 the closing brace of the item before it, so the slice is **C 2220-2397**
> - the blank line C 2220 and the four items `haiku_draw_bar_cursor` (C 2221-2316),
> `haiku_draw_window_cursor` (C 2318-2365), `haiku_show_hourglass` (C 2367-2381) and
> `haiku_hide_hourglass` (C 2383-2397).  The next definition is
> `haiku_compute_glyph_string_overhangs` (C 2399).
>
> **Why 2219 and not 2221 (slice 9).**  C 2221 is `haiku_draw_bar_cursor`'s `static void` and
> C 2219 the closing brace of the item before it, so the slice is **C 2075-2219** - the blank line
> C 2075 and the three items `haiku_after_update_window_line` (C 2076-2129), `haiku_set_window_size`
> (C 2131-2167) and `haiku_draw_hollow_cursor` (C 2169-2219).  The next definition is
> `haiku_draw_bar_cursor` (C 2221).
>
> **Why 2074 and not 2076 (slice 8).**  C 2076 is `haiku_draw_glyph_string_background`'s `static
> void` and C 2074 the closing brace of the item before it, so the slice is **C 1903-2074** - the
> blank line C 1903 and `haiku_draw_glyph_string` (C 1904-2074), the dispatcher every glyph-string
> type goes through.  The next definition is `haiku_draw_glyph_string_background` (C 2076).
>
> **Why 1902 and not 1904 (slice 7).**  C 1904 is `haiku_draw_glyph_string`'s `static void` and
> C 1902 the closing brace of the item before it, so the slice is **C 1733-1902** - the blank line
> C 1733, `haiku_translate_transform` (C 1734-1740) and `haiku_draw_image_glyph_string`
> (C 1742-1902).  The next definition is `haiku_draw_glyph_string` (C 1904).
>
> **Why 1732 and not 1734 (slice 6).**  C 1734 is `haiku_translate_transform`'s `static void` and
> C 1732 the closing brace of the item before it, so the slice is **C 1641-1732** - the blank line
> C 1641 and `haiku_draw_image_relief` (C 1642-1732).  The next definition is
> `haiku_translate_transform` (C 1734).
>
> **Why 1640 and not 1642 (slice 5).**  C 1642 is `haiku_draw_image_relief`'s `static void` and
> C 1640 the closing brace of the item before it, so the slice is **C 1512-1640** - the blank line
> C 1512 and the four items `haiku_clip_to_row` (C 1513-1530), `haiku_update_begin` (C 1532-1538),
> `haiku_update_end` (C 1540-1545) and `haiku_draw_composite_glyph_string_foreground`
> (C 1547-1640).  The next definition is `haiku_draw_image_relief` (C 1642).
>
> **Why 1511 and not 1513 (slice 4).**  C 1513 is `haiku_clip_to_row`'s `static void` and C
> 1511 the closing brace of the item before it, so the slice is **C 1400-1511** - the blank line
> C 1400 and the three items `haiku_draw_stretch_glyph_string` (C 1401-1497), `haiku_start_clip`
> (C 1499-1504) and `haiku_end_clip` (C 1506-1511).  The next definition is `haiku_clip_to_row`
> (C 1513).
>
> **Why 1399 and not 1401 (slice 3).**  C 1401 is `haiku_draw_stretch_glyph_string`'s `static
> void` and C 1399 the closing brace of the item before it, so the slice is **C 1199-1399** - the
> blank line C 1199 and the five items C 1200-1399: `haiku_draw_background_rect`,
> `haiku_maybe_draw_background`, `haiku_mouse_face_colors`,
> `haiku_draw_glyph_string_foreground` and `haiku_draw_glyphless_glyph_string_foreground` (whose
> C 1330 `#ifdef GCC_LINT` pair is this slice's second condition).  The next definition is
> `haiku_draw_stretch_glyph_string` (C 1401).
>
> **Why 1198 and not 1200 (slice 2).**  C 1200 is `haiku_draw_image`'s `static void` and C 1198
> the closing brace of the item before it, so the slice is **C 1045-1198** - the blank line C 1045
> and the five items `haiku_draw_string_box` (C 1046-1106), `haiku_draw_plain_background`
> (C 1108-1126), `haiku_get_bitmap_rec` (C 1128-1132), `haiku_update_bitmap_rec` (C 1134-1161)
> and `haiku_draw_stipple_background` (C 1163-1198).  The next definition is `haiku_draw_image`
> (C 1200).
>
> **The slice's own conditions.**  The file has five `#if`/`#ifdef` lines and
> `tools/check-cond-eval.py src/haikuterm.c` reads it as `5 conditions, 0 unknown, 0
> disagreements [VERIFIED]`: C 40's `#ifdef USE_BE_CAIRO` (in this slice), C 1330's `#ifdef
> GCC_LINT`, C 4148's `#if 0`, C 4772's and C 4895's `#ifdef USE_BE_CAIRO` (all four in the
> remainder).  All five take their false arm on this target.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`HAIKU_OBJ`, i.e. `HAVE_HAIKU` ← `HAVE_BE_APP`, configure.ac 3106-3114) | `feature = "have_haiku"` (on every item *and* on C 21-38's include graph) | a Haiku app-kit configuration |
| C 40 `#ifdef USE_BE_CAIRO` … C 42 | `feature = "be_cairo"` (the name `femacs/xtask/Cargo.toml` registers for `USE_BE_CAIRO`) | a Haiku build with cairo - **carried as a record only**: `<cairo.h>` is a system header with no mirror and no name from it is used in this slice (see Deviations) |
| C 1330 `#ifdef GCC_LINT` / `#else` (slice 3) | `feature = "gcc_lint"` (the name `femacs/xtask/Cargo.toml` 127 registers for `GCC_LINT`) | a `gcc_lint` build.  **Both arms carry**; this target's live arm is the `#else` (C 1333's `PACIFY_GCC_BUG_81401 = 0`), which is what sizes C 1335's `char buf[7 + ...]` - the constant the port names once per arm and reads in the array's length |
| C 4148 `#if 0` … C 4156 | not in this slice (dead text in every configuration) | - |
| C 4772, C 4895 `#ifdef USE_BE_CAIRO` | not in this slice | - |

`HAIKU_OBJ`/`HAVE_HAIKU` are the *only* build condition: this file has no
`ANDROID_STUBIFY`-style sub-gate, and `HAVE_WINDOW_SYSTEM` follows from it for the code's names
(an app-kit configuration always has a window system - the `androidfont.rs` reading), which is why
the pre-flight below sets it as well.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-20 | `#include <config.h>` | ungated (every unit includes it) |
| 21-36 | the fifteen headers `dispextern.h`, `frame.h`, `lisp.h`, `haikugui.h`, `keyboard.h`, `haikuterm.h`, `blockinput.h`, `termchar.h`, `termhooks.h`, `menu.h`, `buffer.h`, `haiku_support.h`, `thread.h`, `window.h`, `haikuselect.h` | file gate |
| 37-39 | `#include <math.h>`, `#include <stdlib.h>` | file gate |
| 40-43 | `#ifdef USE_BE_CAIRO` / `#include <cairo.h>` / `#endif` | recorded, not transcribed (`<cairo.h>` has no mirror; no name it declares is used in this slice) |
| 44-46 | `BE_SB_MAX` | file gate |
| 47-49 | `x_display_list` | file gate |
| 50-53 | `up_to_date_count` | file gate |
| 54-56 | `fringe_bmps` | file gate |
| 57-59 | `max_fringe_bmp` | file gate |
| 60-62 | `rdb` | file gate |
| 63-66 | `any_help_event_p` | file gate |
| 67-74 | `get_keysym_name` (non-`static`) | file gate |
| 75-95 | `haiku_window_to_frame` | file gate |
| 96-104 | `haiku_coords_from_parent` | file gate |
| 105-113 | `haiku_toolkit_position` | file gate |
| 114-119 | `haiku_delete_terminal` | file gate |
| 120-139 | `haiku_get_string_resource` | file gate |
| 140-154 | `haiku_update_size_hints` | file gate |
| 155-197 | `haiku_clip_to_string` | file gate |
| 198-206 | `haiku_clip_to_string_exactly` | file gate |
| 207-220 | `haiku_flip_buffers` | file gate |
| 221-240 | `haiku_frame_up_to_date` | file gate |
| 241-247 | `haiku_buffer_flipping_unblocked_hook` | file gate |
| 248-263 | `haiku_clear_frame_area` | file gate |
| 264-286 | `haiku_clear_frame` | file gate |
| 287-330 | `haiku_new_font` | file gate |
| 331-337 | `haiku_valid_modifier_p` | file gate |
| 338-339 | `#define MODIFIER_OR` | file gate |
| 340-349 | `haiku_add_modifier` | file gate |
| 350-364 | `haiku_modifiers_to_emacs` | file gate |
| 365-366 | `#undef MODIFIER_OR` | file gate (Rust macro scope; nothing after C 365 uses it) |
| 367-398 | `haiku_rehighlight` | file gate |
| 399-417 | `haiku_frame_raise_lower` | file gate |
| 418-441 | `haiku_mouse_or_wdesc_frame` | file gate |
| 442-496 | `haiku_set_scroll_bar_thumb` (C 453-468 the `scroll_bar_adjust_thumb_portion_p` arm, C 470-476 the `whole <= 0` arm) | file gate |
| 497-516 | `haiku_set_horizontal_scroll_bar_thumb` | file gate |
| 517-541 | `haiku_scroll_bar_from_widget` (C 531-536 the `for` over `FRAME_SCROLL_BARS`) | file gate |
| 542-549 | `haiku_focus_frame` | file gate |
| 550-571 | `haiku_new_focus_frame` | file gate |
| 572-577 | `haiku_implicitly_set_name` | file gate |
| 578-583 | `haiku_query_frame_background_color` | file gate |
| 584-597 | `haiku_defined_color` | file gate |
| 598-621 | `haiku_draw_box_rect` | file gate |
| 622-658 | `haiku_calculate_relief_colors` (C 630-637 the three-arm colour choice, C 644-653 the relief cache) | file gate |
| 659-747 | `haiku_draw_relief_rect` | file gate |
| 748-758 | `haiku_get_scale_factor` | file gate |
| 759-806 | `haiku_draw_underwave` | file gate |
| 807-845 | `haiku_draw_dash` | file gate |
| 846-886 | `haiku_fill_underline` (C 861-884 the `switch`, C 872-877 the DOTS→DASHES fallthrough pair) | file gate |
| 887-1044 | `haiku_draw_text_decoration` (C 901-1006 the underline arm, C 916-982 the previous-underline/`font` choice, C 985-988 the sanity checks, C 998-1004 the double underline, C 1008-1019 the overline arm, C 1021-1043 the strike-through arm) | file gate |
| 1045-1198 | `haiku_draw_string_box` (C 1046-1106; C 1054-1057 the `full_width_p`/`pseudo_window_p` choice, C 1058-1077 the last-glyph choice and C 1068-1074 the composition walk), `haiku_draw_plain_background` (C 1108-1126), `haiku_get_bitmap_rec` (C 1128-1132), `haiku_update_bitmap_rec` (C 1134-1161; C 1153 the stipple bit) and `haiku_draw_stipple_background` (C 1163-1198; C 1175 the `stipple` read) | file gate (slice 2) |
| 1199-1399 | `haiku_draw_background_rect` (C 1200-1209, non-`static`), `haiku_maybe_draw_background` (C 1211-1231; C 1228 the `background_filled_p` write), `haiku_mouse_face_colors` (C 1233-1258; C 1242-1251 the four inline accessors), `haiku_draw_glyph_string_foreground` (C 1260-1307) and `haiku_draw_glyphless_glyph_string_foreground` (C 1309-1399; C 1330 the `GCC_LINT` pair, C 1335 `buf`, C 1339-1363 the two method arms, C 1371 `encode_char`, C 1383-1395 the box) | file gate (slice 3) |
| 1400-1511 | `haiku_draw_stretch_glyph_string` (C 1401-1497; C 1407 the cursor/stretch test, C 1412-1434 the R2L and column-width choices, C 1443-1476 the stipple pair, C 1478-1495 the background arm, C 1496 the `background_filled_p` write), `haiku_start_clip` (C 1499-1504) and `haiku_end_clip` (C 1506-1511) | file gate (slice 4) |
| 1512-1640 | `haiku_clip_to_row` (C 1513-1530; C 1521 `window_box`'s three outputs, C 1524 `WINDOW_TO_FRAME_PIXEL_Y`), `haiku_update_begin` (C 1532-1538; C 1537 the `FRAME_COMPLETE_P` write), `haiku_update_end` (C 1540-1545; C 1543 the `mouse_face_defer` write) and `haiku_draw_composite_glyph_string_foreground` (C 1547-1640; C 1581 the `u.cmp.automatic` test, C 1588 `COMPOSITION_GLYPH`, C 1600-1637 the `LGSTRING`/`LGLYPH_*` arm) | file gate (slice 5) |
| 1641-1732 | `haiku_draw_image_relief` (C 1642-1732; C 1650 the `image_ascent` call, C 1656-1659 the left box line, C 1663-1666 the two margins, C 1668-1685 the relief/fork, C 1692-1716 the tab-bar/tool-bar `V*_button_margin` chains, C 1720-1727 the four edges, C 1729-1731 the clip rectangle and `haiku_draw_relief_rect`) | file gate (slice 6) |
| 1733-1902 | `haiku_translate_transform` (C 1734-1740) and `haiku_draw_image_glyph_string` (C 1742-1902; C 1747-1748 the two box widths, C 1755-1772 the height/width/x/y folds, C 1777 the `stippled_p` write, C 1780/1782 the cursor background, C 1787-1896 the bitmap arm with `CONVERT_TO_EMACS_RECT`/`gui_intersect_rectangles`/`memcpy`/`be_apply_affine_transform`/`BView_DrawBitmap`/`be_draw_image_mask`/`be_draw_bitmap_with_mask` and the cursor rectangle, C 1898-1901 the relief tail) | file gate (slice 7) |
| 1903-2074 | `haiku_draw_glyph_string` (C 1904-2074; C 1914 and C 1927-1928 the two `stippled_p` writes, C 1916-1938 the right-overhang loop, C 1942-1962 the box/clip decision, C 1967-1999 the five-arm dispatch, C 2001-2059 the prev/next overhang loops, C 2068-2071 the row's `stipple_p` write) | file gate (slice 8) |
| 2075-2219 | `haiku_after_update_window_line` (C 2076-2129; C 2087-2093 the two comma-expression conditions, C 2100-2126 the two face arms), `haiku_set_window_size` (C 2131-2167; C 2140 `XSETFRAME`, C 2144 `want_fullscreen`, C 2148 `configury_done`, C 2162 `haiku_wait_for_event`) and `haiku_draw_hollow_cursor` (C 2169-2219; C 2183-2188 the phys-cursor pair, C 2198-2200 the R2L shift, C 2205-2218 the clip/pen/rectangle) | file gate (slice 9) |
| 2220-2397 | `haiku_draw_bar_cursor` (C 2221-2316; C 2238-2240 the out-of-bounds return, C 2245-2249 the image arm, C 2260-2263 the colour choice, C 2270-2288 the BAR arm, C 2289-2311 the HBAR arm), `haiku_draw_window_cursor` (C 2318-2365; C 2326 the `phys_cursor_on_p` write, C 2328-2334 the fringe arm, C 2338-2362 the five-arm dispatch), `haiku_show_hourglass` (C 2367-2381) and `haiku_hide_hourglass` (C 2383-2397) | file gate (slice 10) |
| 2398-2522 | `haiku_compute_glyph_string_overhangs` (C 2399-2428; C 2402-2404 the two arms, C 2411 the `text_extents` slot, C 2415-2417 the `composition_gstring_width` call, C 2419-2426 the four overhang folds), `haiku_draw_vertical_window_border` (C 2430-2446), `haiku_set_scroll_bar_default_width` (C 2448-2458) and `haiku_set_scroll_bar_default_height` (C 2460-2470) with their three `FRAME_CONFIG_SCROLL_BAR_*` writes, and `haiku_draw_window_divider` (C 2472-2522; C 2476-2487 the three face lookups and colours, C 2493-2519 the three arms) | file gate (slice 11) |
| 2523-2645 | `haiku_condemn_scroll_bars` (C 2524-2544; C 2534-2535 the walk, C 2537-2538 the prepend), `haiku_redeem_scroll_bar` (C 2546-2620; C 2553-2585 the vertical half with C 2564's `goto horizontal`, C 2586-2619 the horizontal half) and `haiku_judge_scroll_bars` (C 2622-2645; C 2633's `for` head, C 2640's chained assignment) | file gate (slice 12) |
| 2646-2745 | `haiku_scroll_bar_create` (C 2647-2690; C 2661 `ALLOCATE_PSEUDOVECTOR`, C 2663 `XSETWINDOW`, C 2674-2677 the two `be_*`/`BView_*` calls, C 2682-2686 the two `XSETVECTOR`s) and `haiku_set_horizontal_scroll_bar` (C 2692-2745; C 2701 the `eassert`, C 2703 `window_box`, C 2706-2708 the four geometry readers, C 2712-2737 the create/move fork, C 2739-2743 the thumb call and the two setters) | file gate (slice 13) |
| 2746-2803 | `haiku_set_vertical_scroll_bar` (C 2747-2803; C 2756 the `eassert`, C 2759 `window_box`, C 2764-2765 the two area readers, C 2770-2794 the create/move fork, C 2796-2801 the thumb call and the two setters) | file gate (slice 14) |
| 2804-2892 | `haiku_draw_fringe_bitmap` (C 2805-2892; C 2823-2824 the invalidate pair, C 2828-2853 the stipple arm with `haiku_get_bitmap_rec`/`haiku_update_bitmap_rec`, C 2855-2888 the fringe bitmap arm with `gui_define_fringe_bitmap` and the two colour choices) | file gate (slice 15) |
| 2893-2980 | `haiku_define_fringe_bitmap` (C 2894-2915; C 2898-2907 the grow, C 2910-2913 the `BBitmap_*` pair), `haiku_destroy_fringe_bitmap` (C 2917-2926) and `haiku_scroll_run` (C 2928-2980; C 2934 `window_box`, C 2936-2938 the three y values, C 2940-2967 the up/down folds, C 2969-2977 the copy) | file gate (slice 16) |
| 2981-3077 | the C 2982-2987 comment block, `haiku_mouse_position` (C 2988-3061; C 3004-3008 the first `FOR_EACH_FRAME`, C 3017-3024 the selected/any-frame fallbacks, C 3031-3056 the view arm with `BView_get_mouse`/`remember_mouse_glyph`/the two `XSETINT`s) and `haiku_flush` (C 3063-3077; C 3068 the dirty/flip test, C 3073 the `FRAME_COMPLETE_P` write, C 3075-3076 the flush) | file gate (slice 17) |
| 3078-3091 | `haiku_define_frame_cursor` (C 3080-3091; C 3082 the tooltip guard, C 3086-3088 the three-part test and the cursor set, C 3090 the `current_cursor` store) | file gate (slice 18) |
| 3092-3138 | `haiku_default_font_parameter` (C 3093-3138; C 3096 the display info, C 3097-3101 the two-argument read and the `Qunbound` fold, C 3103-3115 the system-font and resource fallbacks, C 3117-3134 the four-name loop, C 3136-3137 the recorder) | file gate (slice 19) |
| 3139-4898 | **the remainder of the file** (the C 3140-3170 `redisplay_interface` table onward; C 4148 the `#if 0` block, C 4512 `ASSIGN_CURSOR`, C 4772/4895 the two `USE_BE_CAIRO` blocks) | not in this slice: recorded so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |

## Evidence

**① The frozen configuration (`build-tu`), twice, plus a `/tmp` reproduction at the final revision.**
`cd femacs && CARGO_TARGET_DIR=/tmp/haikuterm-lane/target cargo run -q -p xtask -- build-tu
src/haikuterm.c` → `built src/haikuterm.rs -> ../build/femacs/haikuterm.o`, **exit 0**, and
`build/femacs/haikuterm.o` is **336 bytes** - an empty object.  That is the expected reading, not a
proof of the new text: every item of the port sits behind `have_haiku`, so under the frozen feature
set rustc compiles only `include/config.rs` and the `#![allow]` attribute (P-86 ⑧'s criterion).
**Two runs of the budget were spent**: the first on the slice as it stood, and the second *after*
the one mirror line this batch changed (Deviations 4) - both `exit 0`, both 336 bytes.  Since the
budget was then exhausted and the unit's text was edited again (the fixes Evidence ③ measured),
the final revision's frozen reading is the same compile reproduced in `/tmp` from the same
materialized root with xtask's own rustc arguments:
`rustc --edition 2024 --crate-type lib --emit obj --crate-name haikuterm -C panic=abort
-C codegen-units=1 -C opt-level=0` → **exit 0**, **336 bytes**
(`/tmp/haikuterm-lane/frozen-check.o`).  It is the same reading, not a second `build-tu` run, and
it is named as such.

**② No Haiku-target arm flip exists on this machine (P-86 ③⑵, `text-only`).**  There is **no
Haiku SDK and no Haiku toolchain**: `which -a x86_64-unknown-haiku-gcc i686-unknown-haiku-gcc
haiku-gcc x86_64-pc-haiku-gcc` is empty (exit 1); `rustup target list | grep -i haiku` is empty
and `rustup target list --installed` lists no haiku target (rustc's own `--print target-list`
names `i686-unknown-haiku`/`x86_64-unknown-haiku`, but no `std` for either is installed);
`brew list | grep -i -E 'haiku|beos'` is empty; and `find / -maxdepth 6 -name 'be_interface.h'
-o -maxdepth 6 -name 'Application.h' -path '*haiku*'` finds nothing.  So the port is **not
compiled or linked against a Haiku system** and the C++ half (`femacs/platform/haiku/*.cc`,
`libbe`) is out of reach here.  The reading that replaces the flip is ③ - and it is a `/tmp`
harness reading, not a Haiku-target compile.

**③ The arm-reach pre-flight (a `/tmp` harness).**  `rustup run nightly-2026-10-01 rustc --edition
2024 --crate-type lib --emit obj -C panic=abort --cfg 'feature="have_haiku"'
--cfg 'feature="have_window_system"' -o /tmp/haikuterm-lane/arm.o
build/femacs/units/haikuterm/_root.rs` → **exit 1, 78 error lines**, of which **6 point into
this unit's own text** and **72 are in the included mirrors**.  The harness is the materialized
root (`build/femacs/units/haikuterm/_root.rs` as the second `build-tu` wrote it, whose mirror
copies already carry this batch's one mirror change) with the batch's own text applied to it as
text patches, so the unit under test is the current revision.  Classification:

*In this unit's text (6), every one a named mirror gap rather than a port defect:* `no field
tooltip on type frame` (C 143's `f->tooltip` - `include/frame.rs` carries neither the member nor
its shift: its numbering is the frozen arm's and is gated to it, so the port keeps C's own text
and the mirror round owns the bits); `FRAME_AUTO_RAISE_SHIFT`/`FRAME_AUTO_LOWER_SHIFT` not found
(C 559/564's two flag reads - the same gated numbering, the gap `androidterm.rs` 170-195 names in
its own comment); `FRAME_TOOLTIP_P` not found twice (C 323, C 407 - the mirror defines that
accessor only under `not (have_window_system)`); and `fontset_from_font` not found, which is a
**harness artifact**: C reaches it through `dispextern.h` 171's `#include "fontset.h"`, the unit
names `include/fontset.rs` for it, and the harness drops that include because the frozen graph
never materialized that file (Deviations 3).

*In the mirrors (72), each class an existing or newly measured mirror gap:* 22 × `E0308`
mismatched types, 15 × `cannot find macro eassert` and 4 × `cannot find macro XUNTAG` (the mirror
include-order: C's order for this file reaches `frame.rs`/`window.rs`/`buffer.rs` before
`lisp.h`, and those mirrors invoke lisp.h macros in their own bodies - P-57's rule, which the
mirrors currently satisfy only for the include orders of the units already ported), 12 ×
`FRAME_*_SHIFT` / `FRAME_HAS_VERTICAL_SCROLL_BARS*` not found (the flag-numbering round above),
9 × `E0080` layout assertions for the window-system arms (`sizeof font`/`font_driver`, their
offsets, and `include/systime.rs`'s `!cfg!(feature = "have_haiku") || HAIKU`), and
`note_mouse_highlight`/`XSETPVECTYPESIZE` once each.

**③a Positive control (P-86 ⑦: the reading is not empty).**  Injecting one deliberate type error
into the unit's own text (`let injected_control: i32 = c"x".as_ptr ();` into `haiku_flip_buffers`)
changes the output to **79 error lines** with `injected_control` present once.  The harness does
type-check this unit's bodies, so "6 in our text" is a reading and not a silence.  Six earlier
defects of the port were found and fixed by this same harness on the way: two missing
`uint32_t` conversions (`BView_SetHighColor`), the fifteen `c_float` spellings (C's `float` is
`f32`, PORTING.md 1.1), four missing P-72 ③ casts on `FRAME_OUTPUT_DATA`, the two `IMAGE_*`
macros reading `struct image`'s bit members as if they were fields, and two dropped C literals
(`return 0;` and `s->face`) that the `--check` token comparison reported.

**④ Inventory.**  `python3 tools/extract-abi.py --check emacs/src/haikuterm.c
femacs/src/haikuterm.rs` → **76 `- ` rows plus `FAIL`**: 69 × `missing function` and 1 ×
`missing macro ASSIGN_CURSOR` (both classes entirely inside the C 1045-4898 remainder of the
coverage table - C's `ASSIGN_CURSOR` is defined at C 4512), 5 spelling rows in this slice's own
text (`haiku_focus_frame`/`haiku_new_focus_frame` → `1`, `haiku_implicitly_set_name` → `0`,
`haiku_draw_underwave` → `float`, `haiku_fill_underline` → `break, case, default, fallthrough,
switch`), and `line count drifted 68.3%: C 4898 vs Rust 1555` (the slice: 1044 of 4898 C lines).
Every one is named below; nothing else in the check fails, including the 37 function bodies and
the one non-`static` definition (`get_keysym_name`) whose tokens are compared.

**⑤ The other gates (repo root, final revision), one line each.**
`check-cond-eval.py src/haikuterm.c` → `src/haikuterm.c: 5 conditions, 0 unknown, 0
disagreements [VERIFIED]` / `5 conditions, 0 unknown to the evaluator, 0 disagreements`. -
`check-dead-file-coverage.py` → `10 ledger(s) bound (a C range table), 0 failure(s)` (this file's
table is one of them). - `check-port-lines.py` → `87 file(s) under ten percent, 203,995 reference
line(s); 90 ratchet row(s), 0 new, 3 stale`, one stale row being `src/haikuterm.c: no longer under
the threshold` (this unit's row retires; `--write-pending` is the integrator's step). -
`check-citations.py` → `675 file(s) scanned, 1 drift, 0 range, 491 external, 33 allowed, 0 stale`
- the drift row is `femacs/src/image.rs:515` (another lane's file); this unit's citations are
clean. - `check-declarations.py` → `13 declaration sites (0 forbidden, 13 allowed literals), 0
failures`. - `check-carried.py` → `11 carried file(s), 11 baseline row(s), 0 failure(s)` (the
`Kept`/C++ classes this gate owns; this batch adds none).  **Not run here** (outside this lane's
budget or its whitelist): `gate-fast`, `check-all-units.sh` (owes the P-87 scan this batch's
mirror lines need), `cargo test`, `extract-abi --all`.

## Deviations

1. **The `#if`s.**  C 40-42's `USE_BE_CAIRO` region is carried as a record and not transcribed:
   `<cairo.h>` is a system header of the Haiku toolchain with no mirror (P-88's class), and no
   name it declares is used by C 1-1044 (the two blocks that use it are C 4772-4793 and C
   4895-4897, both outside this slice).  The other four conditions are outside the slice.  C 365's
   `#undef MODIFIER_OR` has no Rust counterpart (a `macro_rules!` cannot be undefined); nothing
   after C 365 uses it, so the difference is not observable in this file.
   **Why every row is contiguous.**  The table's rows run from C 1 to C 4898 without leaving a
   line out, so the blank lines inside an item's tail are covered too.  That is not a style
   choice: `tools/check-dead-file-coverage.py`'s `ledger_for` resolves
   `femacs/src/include/haikuterm.rs` (port-map row 463) to `docs/ports/haikuterm.md` as well -
   `include-haikuterm.md` is not one of the two names it tries - so this one table is read
   against **both** `src/haikuterm.c` and `src/haikuterm.h`, whose blank lines fall in different
   places.  Covering the blanks is what the gate allows (it requires only that no line *with
   text* be left out) and it is what makes both readings pass; the alternative is renaming one
   of the two ledgers so the gate's rule can tell them apart, which this lane may not do (its
   writable set is two files).
2. **Fifteen declarations the frozen mirrors do not carry** (P-72 rule 4), each with its C header
   and line in a `/// C:` note: `assoc_no_quit` (lisp.h 4274), `adjust_frame_size` (frame.h
   1511), `window_box_right` (dispextern.h 3581), `get_font_ascent_descent` (3631),
   `font_for_underline_metrics` (3638), `gui_update_cursor` (3656), `get_glyph_string_clip_rects`
   (3661), `buffer_flipping_blocked_p` (3719), `image_background` (3769), the
   `image_background_transparent` (3771), `gamma_correct` (3845), `gui_mouse_grabbed` (4006),
   `lrint`/`fmin`/`fmax` (math.h).  Two of them are spelled with types the mirror has no name
   for: `Display_Info *` is dispextern.h 159's `typedef struct haiku_display_info Display_Info`
   for this configuration, and `RGB_PIXEL_COLOR` is dispextern.h 3762's macro - both are declared
   here at the use site, with the C text in the `/// C:` line (P-55).
3. **The include graph.**  C reaches `fontset_from_font` (C 301/327) through C 21's
   `dispextern.h` 171 `#include "fontset.h"`, which `include/dispextern.rs` does not carry yet, so
   the unit includes `include/fontset.rs` (P-72 rule 4) - placed after `include/lisp.rs` because
   the mirror invokes lisp.h macros in its own bodies (P-57), a mirror gap the integrator owns.
   Evidence ③'s `fontset_from_font` row is the harness artifact of that include being outside the
   frozen graph.
4. **Two mirror lines this batch changed** (both named, both minimal, P-87's full re-verification
   owed to the integrator because this lane may not run `check-all-units.sh`):
   (a) `femacs/src/include/haikuterm.rs`, `FRAME_COMPLETE_P` gained C's write arm
   (`($f:expr, $v:expr)`) - the mirror's own `haiku_output_bits!` table already carries
   `complete_p`'s two-argument arm, and C 237/271 writes the member, so the read-only macro was a
   gap; the shape is `FRAME_DIRTY_P`'s, three lines below it.  (b) the same file's
   `haiku_output_bit_p`/`haiku_output_bit_set` now take the pointer `FRAME_OUTPUT_DATA` yields
   (`*mut c_void`) and cast once inside; with `*const haiku_output`/`*mut haiku_output` every
   `MAKE_FRAME_DIRTY` expansion in this unit's text was `E0308 expected *mut haiku_output, found
   *mut c_void` (measured in Evidence ③'s first run).  No other mirror file was touched.
5. **`struct face` is a use-site copy** (P-72 ③/P-80, the `xfaces.rs` 279-330 / `font.rs` 174-225
   precedent): `include/dispextern.rs`'s record is behind `dispextern_face_records`, so this unit
   carries the record, the 21 `FACE_*` masks and the `face_bits!` member macro for the fifteen
   members its text reads.  `s->face` itself is the mirror's untyped P-72 ③ member, so each read
   casts once - inline where C's own `face` token must survive (C 630-637), through the unit's
   `GS_FACE!` elsewhere.
6. **The eight `FRAME_*` accessors this slice writes** (`FRAME_FONTSET`, `FRAME_FONT`,
   `FRAME_BASELINE_OFFSET`, `FRAME_COLUMN_WIDTH`, `FRAME_LINE_HEIGHT`, `FRAME_TAB_BAR_HEIGHT`,
   `FRAME_CONFIG_SCROLL_BAR_COLS`, and `FRAME_DIRTY_P`/`FRAME_COMPLETE_P`, which the mirror
   already had) are read-only in the mirrors, and a Rust macro is not an assignment target, so the
   unit carries C's two-argument write arm under C's own name (the `xfaces.rs` 438-447
   `FRAME_FACE_CACHE` precedent).  `frame.h` 300-341's two `bool_bf` members this slice reads
   (`auto_raise`, `auto_lower`) get the per-unit member macros `androidterm.rs` 170-195 defines
   for the same fields.  C 143's `f->tooltip` keeps C's literal text and is a *named* mirror gap
   (the frozen flag numbering) rather than a spelling change.
7. **`WINDOW_BUFFER_LOCAL_VALUE`'s argument order.**  C's macro is `(V, W)` (window.h 633) and
   `include/window.rs` 399 declares it `(w, v)`, so this unit's three use sites (C 934-951) swap
   the two arguments and say so at the first; the mirror gap is owed.
8. **C's `0`/`1` passed to `bool` parameters are `true`/`false`** (C 547, 560, 565, 575 and the
   `popup_activated_p == 0` test at C 564) - the recorded spelling class of
   `docs/ports/frame.md` 710 / `keyboard.md` 1195, whose rows are owed below.
9. **`float` is `f32`** (PORTING.md 1.1; `haiku_draw_underwave`'s four locals), which loses C's
   `float` token - one owed row.  C's `unsigned long`/`int`/`double` mixtures are C's implicit
   conversions written out as casts (`FRAME_BACKGROUND_PIXEL (f) as uint32_t`,
   `(*face).box_color as uint32_t`, `lrint (size) as c_int`, `ceil (value) as c_int`,
   `position as c_double / whole as c_double`, `as face_underline_type`, ...), and the four
   expressions that mix C's `int` members with `unsigned long` locals (C 958-961, 985-988, 1001,
   1033) keep C's conversions *and* its wrap (`wrapping_sub`/`wrapping_add`, PORTING.md 1.1).
   That the local is `unsigned long` and the parameter is `c_int` is also why
   `haiku_fill_underline`'s two calls cast at the boundary.
10. **Two comma expressions and three chained assignments** are written as the statements C's
    effects require, in C's order, each noted at its site: C 471's `top = 0, shown = 1;`, C 798's
    `ax = bx, ay = by;` (with C 834's two-expression `for` increment at the foot of the port's
    `while`), and C 786's `ay = by = y + 0.5;`.  C 916's `for` is a `while` with its update
    expression at the foot (`haiku_scroll_bar_from_widget`) - the `w32fns.rs` 1524 reading of the
    same shape; neither loop has a `continue`.
11. **`switch` → `match`** in `haiku_fill_underline` (C 861-884) with the `FALLTHROUGH` from the
    DOTS arm into the DASHES one as the entry stage the two arms share (PORTING.md 2.2).  C's
    `min`/`max` are `std::cmp::min`/`f64::min`/`std::cmp::max` per the operand types.
12. **Uninitialised C locals** are zeroed and named (PORTING.md 1.5): `haiku_clip_to_string`'s
    `struct haiku_rect r[2]` (with the `core::mem::zeroed ()` spelling `pdumper.rs` 2272 uses),
    `haiku_set_scroll_bar_thumb`'s four `double`s, `haiku_set_horizontal_scroll_bar_thumb`'s four,
    `haiku_draw_underwave`'s nine `int`s and four floats, `haiku_draw_dash`'s five `int`s,
    `haiku_draw_text_decoration`'s `cursor_color`, `thickness`, `position`, and
    `haiku_draw_relief_rect`'s two `uint32_t`s; `haiku_new_font`'s `fontset` parameter is
    reassigned at C 301, so the port rebinds it (`let mut fontset = fontset;`, the `androidmenu.rs`
    shape).
13. **Line count.**  C 4,898 (this slice: 1,044) vs Rust 1,555; the slice's own density is
    1555/1044 = 1.49×, and the check's 68.3% drift is the slice, not an expansion.  A
    `docs/ports/line-drift-tolerance.tsv` row (stem `haikuterm`, basis `total`) is owed, derived
    from the finished prefix the way that table's own note describes for a half-ported unit.
14. **`--check` is no longer vacuous for this file** (P-86 ⑩): the port has `fn`s, so
    `empty_unbuilt` (extract-abi 1362) does not exempt it and the full comparison - bodies,
    invented symbols, the line count - is what ④ reports.

## Owed by the integrator (nothing here is claimed as done)

1. `tools/check-port-lines.py --write-pending`: **discharged by the tree** - the ratchet list is
   empty (`0 file(s) under ten percent, 0 reference line(s); 0 ratchet row(s), 0 new, 0 stale`,
   measured 2026-10-08), the rewrite having landed at status 3.574; this file's row (slice 1's
   Evidence ⑤ read it as one of `90 ratchet row(s) ... 3 stale`) retired with it.  Nothing is
   owed here.
2. `docs/ports/line-drift-tolerance.tsv`: **discharged at slice 2** - the `haikuterm` row is
   appended (`0.69`, basis `total`, with slice 2's measured 62.9% and the note that a half-ported
   file's row is the placeholder the table's header describes, to be re-derived when C 4898 is
   reached).
3. `docs/ports/macro-body-notes.tsv`: **discharged** - the four rows of Deviations 8/9/11 landed
   with slice 1's round (`haiku_fill_underline` → `switch,case,default,break,fallthrough`,
   `haiku_focus_frame`/`haiku_new_focus_frame` → `1`, `haiku_implicitly_set_name` → `0`,
   `haiku_draw_underwave` → `float`), and slice 2 added the two this file still owed:
   `haiku_update_size_hints` → `tooltip` (C 143's member, spelled through the mirror's
   `FRAME_TOOLTIP_P`) and `haiku_draw_stipple_background` → `face` (C 1175/1186/1187's member
   path, spelled through the unit's `GS_FACE!`).
4. `docs/ports/include-frame.md`'s own owed round is what this unit's flip measured:
   `include/frame.rs` carries neither `tooltip` nor the other window-system flag members'
   numbering (all its `FRAME_*_SHIFT`s are `#[cfg (not (feature = "have_window_system"))]`), nor
   the `HAVE_WINDOW_SYSTEM` arms of `FRAME_TOOLTIP_P`, `FRAME_HAS_VERTICAL_SCROLL_BARS*`; until
   that lands, **no Haiku configuration can compile any translation unit** and every
   `have_haiku` flip stays partly vacuous (Evidence ③).
5. The mirror set's include-order self-sufficiency (Evidence ③'s 19 `eassert`/`XUNTAG` errors):
   `frame.rs`/`window.rs`/`buffer.rs` invoke lisp.h macros in their own bodies while C's order for
   this file reaches them before `lisp.h`.  Either each mirror includes what it invokes (P-72 rule
   4's direction) or this unit's include list moves `lisp.h` first (a deviation from C's order);
   the choice is the integrator's.
6. `include/dispextern.rs`: dispextern.h 171's `#include "fontset.h"` (Deviations 3), and the
   nine `E0080` layout assertions the window-system arms still fail (`font`, `font_driver`,
   `include/systime.rs`'s `HAIKU` constant) - measured here, owned there.
7. The P-87 scan for the two mirror lines Deviations 4 changed: `tools/check-all-units.sh`
   (expected `83 unit(s), 0 failure(s)`) - **not run by this lane** and not claimed.
8. This ledger's remainder: C 1045-4898, whose first definition is `haiku_draw_string_box`
   (C 1046-1106); its conditions are C 1330's
   `#ifdef GCC_LINT`, C 4148's `#if 0`, C 4512's `ASSIGN_CURSOR` and C 4772/4895's `USE_BE_CAIRO`.


## Slice 2 (2026-10-08, status 4.078 - the integrator sets the round number): C 1045-1198

**The slice's text.**  The blank line C 1045, then the five items that follow
`haiku_draw_text_decoration`: `haiku_draw_string_box` (C 1046-1106), `haiku_draw_plain_background`
(C 1108-1126), `haiku_get_bitmap_rec` (C 1128-1132), `haiku_update_bitmap_rec` (C 1134-1161) and
`haiku_draw_stipple_background` (C 1163-1198).  Port line count **1,555 -> 1,815** (+260; +226 of
text as first written, +30 for the `face` layout and the two casts of Deviations 1/2, +4 for the
zero-initialised pair of Deviations 1).  Five new unit-local names this slice declares, each with
its precedent: the `glyph_u_cmp_bits!` accessor for C 1063-1074's `u.cmp` run (`pgtkterm.rs`
2102-2111), the `window_bits! (pseudo_window_p, …)` arm for C 1055 (`xterm.rs` 8634-8643), and the
`face_bits!` arms `box` (C 1081/1099, the `pgtkterm.rs` 1960 shape) and `background_defaulted_p`
(C 1121).

**Evidence ① - the arm flip's substitute (`xtask probe`: `/tmp` pre-flight, P-86 ③).**  The arm
set is the one slice 1 recorded and the only one this file has - `have_haiku` +
`have_window_system`:

    cd femacs && CARGO_TARGET_DIR=/tmp/haikuterm-lane cargo run -q -p xtask -- probe \
      src/haikuterm.rs /tmp/haikuterm-lane/o.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="have_haiku"' --cfg 'feature="have_window_system"'

Three runs, and the difference between them is this slice's work:

- **Run 1 (the slice's text as handed over): `exit 1`, `aborting due to 38 previous errors`.**
  Five of the error-prefixed lines point into the unit's own text at three sites: `_root.rs`
  1092:15/41 and 1098:15/41 (the `E0606`/`E0609` pair from `(*(*s).face as *mut face).box_color`
  - `haiku_calculate_relief_colors`' C 631/637, slice 1's text) and 1764:57 (`no field stipple on
  type face`, this slice's C 1175).  64 mirror rows are the two classes slice 1 recorded
  (`eassert`/`XUNTAG` include-order, the window-system `E0080` layout assertions) plus the three
  `FRAME_MOUSE_UPDATE`/`FRAME_COMPLETE_P` rows below.
- **Run 2 (after the `face` layout repair, Deviation 1, and the two casts, Deviation 2):**
  `exit 1`, the `E0606`/`E0609` rows gone, two `E0381` rows take their place - `used binding
  foreground/background isn't initialized on any path` at C 1171's two locals (Deviation 1's third
  part).
- **Run 3 (final revision): `exit 1`, 4 primary errors among 35 error-prefixed lines, and every
  one of the four is in a mirror - `include/haikuterm.rs` twice (`haiku_output_bit_set`'s `($v)
  != 0` under `FRAME_COMPLETE_P`, a `bool`/integer mismatch slice 1's arm set also read),
  `include/systime.rs` once (`E0080`: `!cfg! (feature = "have_haiku") || HAIKU`, the dead-arm
  constant) and `include/frame.rs` once (`note_mouse_highlight` not in scope - the `FRAME_MOUSE_UPDATE`
  body).  **The unit's own text has no primary error left, and none of the four belongs to this
  slice.**  4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 68 rows -> 65.**  The three rows
this slice closed are the two spelling rows (`haiku_update_size_hints` -> `tooltip`, slice 1's,
and the new `haiku_draw_stipple_background` -> `face`) and the line-drift row; the 65 that remain
are 64 `missing function` rows plus `missing macro ASSIGN_CURSOR`, every one of them inside the
C 1199-4898 remainder.  The two closed rows are discharged in `docs/ports/macro-body-notes.tsv`
with their reasons (Owed 3); the drift row is discharged in
`docs/ports/line-drift-tolerance.tsv` with slice 2's measured 62.9% (Owed 2).

**Evidence ③ - what the harness found in slice 1's text (all three repaired here).**

- **The `face` record was the frozen arm's layout, not this configuration's** - `have_haiku`
  implies `HAVE_WINDOW_SYSTEM`, and C 1175's own `s->face->stipple` is the proof (the member
  exists only inside dispextern.h 1797-1811's `#ifdef HAVE_WINDOW_SYSTEM`).  The copy now carries
  the `gc`/`stipple` pair under `feature = "have_window_system"` and every offset above `id` is
  asserted as `<frozen> + FACE_WS_SHIFT` (`include/dispextern.rs` 2280-2283's spelling), with
  `docs/baseline/face-layout.tsv`'s measured 296/8 for the window-system arms.  Without this,
  C 1175 had no member to read.
- **C 631/637's two member reads did not compile.**  `(*(*s).face as *mut face).box_color` parses
  as `((*(*s).face) as *mut face).box_color` - unary `*` binds tighter than `as` - so the cast was
  applied to a `c_void` place and the field access then landed on a raw pointer (the two
  `E0606`/`E0609` rows).  The repair adds the inner parentheses
  (`(*((*s).face as *mut face)).box_color`) and keeps the inline cast Deviation 5 requires, so C's
  own `face` token survives the `--check` body comparison - no new `macro-body-notes` row is
  needed for that function.
- **C 1171's two colours needed PORTING.md 1.5's zero.**  C 1183 hands the callee their addresses,
  so the port cannot prove they are written on that path (`E0381`); they are now `0`-initialised
  and named, the same reading this file's C 1113 `cursor_color` took.

**Evidence ④ - the conditions and the other gates.**  `tools/check-cond-eval.py src/haikuterm.c`
is unchanged by this slice (`5 conditions, 0 unknown, 0 disagreements [VERIFIED]`; C 1045-1198
spells no condition of its own).  The frozen configuration is **not** re-measured this slice:
every item this slice adds carries the same `have_haiku` gate as slice 1's, so compiling it with
the frozen feature set still produces the 336-byte empty object slice 1 measured twice, and the
frozen object is named as such rather than claimed.

### Deviations (this slice)

1. **The `face` use-site copy gains the `HAVE_WINDOW_SYSTEM` pair** (this slice's only structural
   change to text that was already committed, and the reason C 1175 can be written at all):
   `gc` (`Emacs_GC *`'s Haiku arm, untyped - P-72 rule 3) and `stipple` (`ptrdiff_t`) after `id`,
   with `FACE_WS_SHIFT` (`0`/`16`) making both arms' offsets assertions instead of one arm's
   constants.  `docs/baseline/face-layout.tsv` measures 296/8 with `gc`@168, `stipple`@176,
   `foreground`@184 for both window-system arms; the `ns_cocoa` row is the same sixteen bytes with
   this record's Haiku spelling of `gc` (`ftcrfont.rs` 769-784's reading), which is why the pair's
   *type* differs per arm while its *size* does not.  The three `E0381`-adjacent changes (the two
   casts of C 631/637 and the two zeroed colours of C 1171) are recorded in Evidence ③.
2. **C 1070-1074's `for (last_glyph = g++; COND; last_glyph = g++) ;`** is the init assignment
   then a `while` whose body holds C's increment, both statements carrying C's line numbers; the
   loop has no `continue`, so the two spellings agree on every path (the `w32fns.rs` 1524 shape
   slice 1 used for C 916).
3. **C 1153's stipple bit** is `((*bits.offset ((x / 8) as isize)) as c_uchar >> (x % 8)) & 1 != 0`,
   C's own precedence written out (the `>>` before the `&`, the comparison last) and the `c_char`
   load widened by C's `char` promotion rather than the port's choice.
4. **The `haiku_bitmap_record` this slice walks is the mirror's** (`include/haikuterm.rs`:
   `stipple_bits: *mut c_void`, `stipple_foreground`/`stipple_background: uint32_t`), so
   `haiku_update_bitmap_rec` spells C 1146's `char *bits` as a `*mut c_void as *mut c_char` and
   C 1152's `haiku_put_pixel (…, uint32_t)` as a `c_ulong` argument (the mirror's declaration takes
   `c_ulong`; C's prototype is `uint32_t`, and the widening is the call's own).
5. **`haiku_get_bitmap_rec`'s C `&FRAME_DISPLAY_INFO (f)->bitmaps[id - 1]`** is
   `(*FRAME_DISPLAY_INFO! (f)).bitmaps.offset ((id - 1) as isize)`: the mirror types `bitmaps` as a
   raw pointer, so C's subscript is an offset (the `alloc.rs` 464 shape).

### Owed (this slice's additions)

- **`tools/check-port-lines.py`**: discharged by the tree, not by this slice - Owed 1 above; the
  reading at this round is `0 file(s) under ten percent ... 0 ratchet row(s), 0 new, 0 stale`.
- **Nothing else for this slice**: both rows `--check` reported for it are discharged in the tables
  named in Evidence ②, and the C-range table now carries the slice.

### Not claimed (this slice)

- **No runtime evidence, and none is possible here.**  There is no Haiku SDK and no Haiku toolchain
  on this machine (Evidence ② of the file's own section), the frozen configuration compiles none of
  this file, and this lane does not link: **no glyph string was drawn, no box or relief rectangle
  was filled, no stipple bitmap was tiled, and no pixel was written.**  What is measured is the
  arm's type-checking and the text/token gates.
- The four mirror errors of Evidence ③(3) are **not this slice's** - two are slice 1's recorded
  `FRAME_*` gaps and two are the mirror set's own (`systime.rs`'s `HAIKU`, `frame.rs`'s
  `note_mouse_highlight`), all owned by the integrator's mirror round.
- The file is **not closed**: C 1199-4898 remains (65 `--check` rows), and the line-drift row is a
  half-ported file's placeholder, not a measurement of the finished density.

## Slice 3 (2026-10-08, status 4.079 - the integrator sets the round number): C 1199-1399

**The slice's text.**  The blank line C 1199 and the five items after it:
`haiku_draw_background_rect` (C 1200-1209 - non-`static`, so it keeps C's ABI),
`haiku_maybe_draw_background` (C 1211-1231), `haiku_mouse_face_colors` (C 1233-1258),
`haiku_draw_glyph_string_foreground` (C 1260-1307) and
`haiku_draw_glyphless_glyph_string_foreground` (C 1309-1399, with C 1330's `#ifdef GCC_LINT`
pair).  Port line count **1,815 -> 2,169** (+354).  Two names this slice declares for the whole
unit: `glyphless_bits!` (the `glyph.u.glyphless` run C 1337-1383 names, the `haikufont.rs`
1447-1458 shape) and a `face_bits!` arm for `overstrike` (C 1304, `FACE_OVERSTRIKE`'s bit).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Four runs of the same
command as slice 2's Evidence ①, arm set `have_haiku` + `have_window_system`:

- **Run 1: `no rules expected 'overstrike'`** (a macro-level error, not an `error[...]` row) - the
  `face_bits!` table had no arm for C 1304's `face->overstrike`; the arm is now declared.
- **Run 2: `exit 1`, 10 primary errors, 6 of them in the unit's own text** - the three
  `GLYPHLESS_DISPLAY_*` names (they are `dispextern_unit_names`'s, the `haikufont.rs` 1510-1511
  path), `(*FRAME_OUTPUT_DATA! (... ) as *mut haiku_output).cursor_fg` (the same unary-`*`-before-
  `as` parse as slice 2's Deviation 2, fixed the same way: `(*(FRAME_OUTPUT_DATA! (...)
  as *mut haiku_output)).cursor_fg`), and `face_for_char`'s unsafe call inside a safe
  `FACE_FOR_CHAR`.
- **Run 3: `exit 1`, 4 primary errors, and one of the four in the unit's own text** - `this
  operation will panic at runtime` for C 1350's `extras[0]` (the mirror carries the member as
  `[Lisp_Object; 0]`, the `FLEXIBLE_ARRAY_MEMBER` idiom, and a constant index of 0 into a
  zero-length array is `deny(unconditional_panic)`: *index out of bounds: the length is 0 but the
  index is 0*).  The repair is the tree's own spelling, `FAM_ELEM!` (Deviation 3), which is what
  `w32term.rs` 2478 writes for the same C text.
- **Run 4 (final revision): `exit 1`, 4 primary errors among 34 error-prefixed lines, every one of
  the four in a mirror** - the same four slice 2's final revision read (`include/haikuterm.rs`
  twice, `include/systime.rs` once, `include/frame.rs` once).  **The unit's own text has no
  primary error.**  4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 65 rows -> 60.**  The five closed
are `missing function` rows for the five items this slice lands; the 60 that remain are 59
`missing function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 1400-4898 remainder.
One new row appeared and is discharged: **`function haiku_maybe_draw_background does not mention
1`** (C 1228's write, which the port spells `true` on the `bool_bf` member) - a
`docs/ports/macro-body-notes.tsv` row with its reason.

**Evidence ③ - the line-drift row now covers this slice.**  C 4898 vs Rust 2,169 is 55.7%, inside
the `haikuterm` row's 0.69 (slice 2's placeholder, still to be re-derived when the file closes).

### Deviations (this slice)

1. **The four inline accessors and the one record they need are unit copies**
   (`FRAME_FACE_CACHE` C 1379, `FACE_FROM_ID` C 1646, `FACE_FROM_ID_OR_NULL` C 1656,
   `FACE_FOR_CHAR` dispextern.h 2030-2038, plus `struct face_cache` C 1991-2009 and
   `prepare_face_for_display`'s dispextern.h 3810 declaration): C pastes the inline definitions
   into every unit that includes the header, and `include/frame.rs` 1534-1552 keeps its own copy
   behind `dispextern_face_records`, which **this unit's arm set does not carry** - the
   `pgtkterm.rs` 4155-4197 / `term.rs` 307-320 / `xdisp.rs` 1095-1115 shapes.  The alternative is
   the one that lane took (`pgtkterm.md` Deviation 17: put `dispextern_face_records` in the arm
   set and let the mirror's `face`/`face_cache` be the live ones); it is *not* taken here because
   this unit carries its own `face` copy since slice 1, and the mirror's own note
   (`include/dispextern.rs` 2216-2232) says retiring the units' copies must be done in one
   deliberate move across all of them, not per lane.  `FACE_FOR_CHAR` carries **both** arms under
   `have_window_system` (the window-system call to `face_for_char`, the other C's `face->id`).
2. **`include/fontset.rs`'s `face_for_char` takes the untyped P-72 ③ pointers**, so
   `FACE_FOR_CHAR`'s window-system arm casts `f`/`face` once at the call (the declaration is the
   mirror's, not this unit's).
3. **`FAM_ELEM!` for C 1350's `extras[0]`** - Evidence ①'s run 3; the member is a
   `FLEXIBLE_ARRAY_MEMBER` in the mirror and `FAM_ELEM!` is `include/lisp.rs` 1308-1311's base
   accessor (the `w32term.rs` 2478 spelling of the same C text, and `include/lisp.rs` 1296-1308's
   note on why `as_mut_ptr ()` on the `[T; 0]` field would be UB).
4. **C 1313's `static unsigned char2b[8]` keeps C's storage duration**: `static mut char2b:
   [c_uint; 8] = [0; 8];` and `(*s).char2b = &raw mut char2b as *mut c_uint;` (C 1326).  The
   `w32term.rs` 2435 copy of this same function writes it the same way; `xterm.rs` 10229 and
   `pgtkterm.rs` 2274 write a *local* instead, and that difference is theirs to record.
5. **C 1316's `unsigned long color`** is zero-initialised and named (PORTING.md 1.5), because
   C 1386 hands the callee its address - the reading this file's C 1113 and C 1171 already took.
6. **`font->driver->draw`/`encode_char`** are `include/font.rs` 553-590's `font_driver` slots
   reached by the untyped `(*font).driver` (P-72 rule 3), so each call site casts once and
   unwraps the `Option` (`pgtkterm.rs` 2076-2081's spelling of the same two calls).
7. **`haiku_draw_background_rect`'s definition shadows the mirror's declaration.**
   `include/haikuterm.rs` 496-500 declares the prototype with the degraded `*mut c_void`
   arguments (P-55) and the unit glob-imports it; a local definition wins over a glob import
   (measured), and this one keeps **C's typed pointers**, because this unit is where `face` and
   `glyph_string` are in scope.  Callers inside the unit pass `GS_FACE! (s)` for C's `s->face`.
8. **C 1220-1222's `||` chain** is written with the mirror's readers converted to `bool` where C's
   operands are `bool_bf` (`font_not_found_p`, `extends_to_end_of_line_p`) and compared to `0`
   where C's is the `int` parameter (`force_p`), so every operand keeps its C type; C's `max` on
   two `int`s is `std::cmp::max` (this file's own reading of C 938/981), and C 1280/1328's `for`
   heads are the init/`while`/step trios with the increment at the foot.

### Owed (this slice's additions)

- **Nothing new**: the one `--check` row this slice produced is discharged in
  `docs/ports/macro-body-notes.tsv` (Evidence ②), and the line-drift row and the ratchet list are
  unchanged from slice 2's readings.  The C-range table now covers C 1199-1399 item by item.

### Not claimed (this slice)

- **No runtime evidence.**  There is no Haiku SDK and no Haiku toolchain here, the frozen
  configuration compiles none of this file, and this lane does not link: **no glyph was drawn as
  a rectangle, no font driver's `draw` or `encode_char` was called, no glyphless acronym or hex
  code was built, and no pixel or pen was set.**  What is measured is the arm's type-checking and
  the text/token gates.
- The four mirror errors of Evidence ①'s run 4 are **not this slice's** (two are slice 1's
  recorded `FRAME_*` gaps, two are the mirror set's own), and the `--check` rows that remain are
  the C 1400-4898 text this file has not reached.

## Slice 4 (2026-10-08, status 4.080 - the integrator sets the round number): C 1400-1511

**The slice's text.**  The blank line C 1400 and the three items after it:
`haiku_draw_stretch_glyph_string` (C 1401-1497), `haiku_start_clip` (C 1499-1504) and
`haiku_end_clip` (C 1506-1511).  Port line count **2,169 -> 2,310** (+141).  Three prototypes
this slice's calls need are declared in the file's own extern block (the `window_box_right`
entry's rule, since no mirror carries them): `window_box_left` (dispextern.h 3579),
`window_box_left_offset` (3580) and `cursor_in_mouse_face_p` (3692).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  One run of slice 2's command,
arm set `have_haiku` + `have_window_system`: **`exit 1`, 4 primary errors among 34 error-prefixed
lines, and every one of the four is in a mirror** - the same four slices 2 and 3 read
(`include/haikuterm.rs` twice, `include/systime.rs` once, `include/frame.rs` once).  **The unit's
own text has no primary error on the first run**, 4 warnings.  The shapes this slice reuses were
all measured in earlier slices: `FRAME_HAIKU_DRAWABLE!`, `GS_FACE!`,
`glyph_string_bits!`/`glyph_row_bits!` (`reversed_p`, `mouse_face_p`, `mode_line_p`),
`glyph_row_bits!`'s `mouse_face_p` pair C 1214-1231 already used, and the two zeroed colour
locals (Deviation 1).  The four arm readings of this file so far (slices 2, 3, 3, 4) all end at
the same four mirror rows.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 60 rows -> 57.**  The three closed
are the `missing function` rows for the three items this slice lands; the 57 that remain are 56
`missing function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 1512-4898 remainder.
One new row appeared and is discharged: **`function haiku_draw_stretch_glyph_string does not
mention 1`** (C 1496's write of the same `bool_bf` member C 1228 writes) - a
`docs/ports/macro-body-notes.tsv` row with its reason.

### Deviations (this slice)

1. **C 1405's `uint32_t bkg` and C 1437's `unsigned long cursor_color`** are zero-initialised and
   named (PORTING.md 1.5): C 1459/1469 and C 1439 hand the callee their addresses, so the port
   cannot prove they are written on the path that reads them - the reading this file's C 1113,
   C 1171 and C 1316 already took.
2. **C's `min` on two `int`s is `std::cmp::min`** (C 1432), the spelling this file's C 938/981
   readings established; the `isize`-taking shim `pgtkterm.rs` 2395 uses for its own copy of this
   same C line is that lane's choice, not this one's.
3. **`window_box_left_offset`/`window_box_left`/`cursor_in_mouse_face_p` are use-site
   declarations** (the `window_box_right` entry above them): dispextern.h declares them and no
   mirror carries them, so the unit names them itself (P-72 rule 4), with C's own lines in the
   `/// C:` notes.
4. **`!x_stretch_cursor_p`** is a direct read of the `globals.rs` `static mut bool` - the
   `pgtkterm.rs` 2368 spelling of the same C line, and the reason the unit's arm set needs no
   extra feature for C's `extern bool` from xdisp.c.

### Owed (this slice's additions)

- **Nothing new**: the one `--check` row this slice produced is discharged in
  `docs/ports/macro-body-notes.tsv` (Evidence ②).  The line-drift row is unchanged (C 4898 vs
  Rust 2,310 is 52.8%, inside the `haikuterm` row's 0.69) and the ratchet list is empty.

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no stretch glyph was drawn, no cursor
  colour was merged, no stipple bitmap was tiled and no rectangle was filled.**  What is measured
  is the arm's type-checking and the text/token gates.
- The four mirror errors are not this slice's, and the `--check` rows that remain are the
  C 1512-4898 text this file has not reached.

## Slice 5 (2026-10-08, status 4.081 - the integrator sets the round number): C 1512-1640

**The slice's text.**  The blank line C 1512 and the four items after it: `haiku_clip_to_row`
(C 1513-1530), `haiku_update_begin` (C 1532-1538), `haiku_update_end` (C 1540-1545) and
`haiku_draw_composite_glyph_string_foreground` (C 1547-1640 - the composition walk, with both the
`u.cmp.automatic` arm and the `LGSTRING`/`LGLYPH_*` arm).  Port line count **2,310 -> 2,485**
(+175).  `window_box` (dispextern.h 3574-3575) joins this file's use-site extern block (the
`window_box_right` entry's rule).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Two runs of slice 2's command,
arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 8 primary errors, 3 of them in the unit's own text** - `E0381` for
  `window_x`/`window_y`/`window_width` at C 1518 (C 1521's `window_box` writes them through
  pointers the port cannot follow; Deviation 1's zeroing is the repair).
- **Run 2 (final revision): `exit 1`, 5 primary errors among 35 error-prefixed lines, every one in
  a mirror** - `include/systime.rs` once and `include/frame.rs` once (both slice 1's recorded
  classes), and **`include/haikuterm.rs` three times, not twice**: C 1537's
  `FRAME_COMPLETE_P (f) = false` is the *second* instance of the mirror row slice 1 recorded
  (`haiku_output_bit_set`'s `($v) != 0` against a `bool` parameter), so the class is unchanged and
  the count is one higher.  **The unit's own text has no primary error**, 4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 57 rows -> 53.**  The four closed
are `missing function` rows for the four items this slice lands; the 53 that remain are 52
`missing function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 1641-4898 remainder.
One new row appeared and is discharged: **`function haiku_update_end does not mention
mouse_face_defer`** (C 1543's write, which the port makes through the mirror's
`set_mouse_face_defer`) - a `docs/ports/macro-body-notes.tsv` row with its reason.

### Deviations (this slice)

1. **C 1518's three `int` outputs of C 1521's `window_box`** are zero-initialised and named
   (PORTING.md 1.5): C hands the callee their addresses, and C 1523-1526 read them back - the
   reading this file's C 1113, C 1171, C 1316 and C 1405 already took.  C 1521's last argument is
   C's `0`, so the port passes the null pointer (the `window_box` declaration's own note).
2. **C 1537's `FRAME_COMPLETE_P (f) = false`** is the mirror's two-argument write arm
   (`frame_bits`-style l-value macro), the spelling slice 1 used at C 237/271.
3. **C 1543's `MOUSE_HL_INFO (f)->mouse_face_defer = false`** goes through
   `include/dispextern.rs` 2149's setter (the `androidterm.rs` 545 / `pgtkterm.rs` 4894 spelling of
   the same C line) - the `mouse_face_defer` token is registered in
   `docs/ports/macro-body-notes.tsv` rather than planted in the body.
4. **C 1547-1640's composition walk reuses the shapes the `pgtkterm.rs` 2115-2225 copy of the same
   C function measured**: `glyph_u_cmp_bits!` for C 1581, `COMPOSITION_GLYPH` on a
   `*mut composition` cast (`glyph_string.cmp` is the untyped P-72 rule 3 member), the two
   `offsets[j * 2]`/`[j * 2 + 1]` reads as pointer offsets, `composition_gstring_from_id` and the
   `LGSTRING_GLYPH`/`LGLYPH_*` accessors from `include/composite.rs` (which C reaches through
   `dispextern.h`'s own include graph, so the unit needs no new `include!`), and the `draw` slot
   unwrapped per call (slice 3's Deviation 6).  `haiku`'s own C 1570-1579 differs from pgtk's: the
   rectangle is the `BView_` pair here, and `cursor_fg` comes from `FRAME_OUTPUT_DATA`.

### Owed (this slice's additions)

- **Nothing new**: the one `--check` row this slice produced is discharged in
  `docs/ports/macro-body-notes.tsv` (Evidence ②); the line-drift row is unchanged (C 4898 vs
  Rust 2,485 is 49.3%, inside the `haikuterm` row's 0.69) and the ratchet list is still empty.
- **One mirror row's count moved** (Evidence ① run 2: `include/haikuterm.rs` 2 -> 3, the
  `FRAME_COMPLETE_P` write arm) - the mirror round that owns that class
  (`include-haikuterm.rs`'s `haiku_output_bit_set`) is Owed 4-6's, not this file's.

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no clip rectangle was set, no frame
  was marked complete or incomplete, no mouse-face flag was cleared, and no composition glyph
  string was drawn** (no `font_driver.draw` was called).  What is measured is the arm's
  type-checking and the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 1641-4898 text this file has not reached.

## Slice 6 (2026-10-08, status 4.082 - the integrator sets the round number): C 1641-1732

**The slice's text.**  The blank line C 1641 and `haiku_draw_image_relief` (C 1642-1732): the six
`int`/`bool` locals, `x`/`y` (with C 1650's `image_ascent` on `s->img`/`s->face`/`&s->slice`), the
left box line, the two margins, the `DRAW_IMAGE_SUNKEN`/`RAISED` fork with the tab-bar/tool-bar
relief globals, the `x1`/`y1` corners, the two `V*_button_margin` chains, the four edge tests and
the closing `get_glyph_string_clip_rect` + `haiku_draw_relief_rect` pair.  Port line count
**2,485 -> 2,610** (+125).  `image_ascent` (dispextern.h 3774) is declared at the use site, since
no mirror carries the prototype and `src/image.rs` 2846-2849 defines it.

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Two runs of slice 2's command,
arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 6 primary errors, 1 of them in the unit's own text** - `get_glyph_string_clip_rect`
  takes the mirror's `*mut c_void` (include/dispextern.rs 4044), so C 1729's `&r` needs the cast
  this file's C 544 already writes for the plural sibling (Deviation 3).
- **Run 2 (final revision): `exit 1`, 5 primary errors among 35 error-prefixed lines, every one in
  a mirror** - `include/systime.rs` once, `include/frame.rs` once, and **`include/haikuterm.rs`
  three times** (slice 5's reading; the extra one there is slice 1's recorded `FRAME_COMPLETE_P`
  write-arm class, and this slice adds none).  **The unit's own text has no primary error**, 4
  warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 53 rows -> 52.**  The one closed is
`haiku_draw_image_relief`'s `missing function` row; the 52 that remain are 51 `missing function`
rows plus `missing macro ASSIGN_CURSOR`, all inside the C 1733-4898 remainder.  **No new spelling
row**: the two `relief` symbols, the four `V*_button_margin` chains and the two `min` folds all
compare token for token.

### Deviations (this slice)

1. **C 1648's `struct haiku_rect r`** is zero-initialised with `core::mem::zeroed ()` (PORTING.md
   1.5; this file's C 544 uses the same spelling for the same record) - C 1729 hands the callee its
   address before C 1730-1731 read it back.
2. **C's two `min (…, 1000000)` folds over `EMACS_INT` globals** are `std::cmp::min
   (tab_bar_button_relief, 1000000) as c_int` / the tool-bar pair (the `isize` fold the
   `pgtkterm.rs` 2130-2136 copy of the same C lines spells with a shim `min`; this file's own
   C 938/981/1432 readings are `std::cmp::*`), and C's `max (face->box_vertical_line_width, 0)` is
   `std::cmp::max` on two `int`s.
3. **`get_glyph_string_clip_rect` takes the mirror's untyped pointer** (include/dispextern.rs
   4044), so C 1729's `&r` is written `&raw mut r as *mut c_void` - the exact spelling this file's
   C 544 uses for the plural sibling (Evidence ① run 1).
4. **C 1720-1727's four comma expressions** (`x -= thick + extra_x, left_p = true;`) are one
   statement per assignment, in C's order, with the two `s->slice.x + s->slice.width ==
   s->img->width` tests written through `glyph_slice_bits!` and C's `int` comparisons kept
   (`as c_int` on each `glyph_slice_bits!` read, whose members are packed fields - the
   `pgtkterm.rs` 3050-3170 copy of the same function is the shape).
5. **`image_ascent` is a use-site declaration** (P-72 rule 4), with C's own prototype line in the
   `/// C:` note - the `pgtkterm.rs` 3041-3048 declaration of the same one.

### Owed (this slice's additions)

- **Nothing new**: `--check` produced no new row for this slice (Evidence ②), the line-drift row is
  unchanged (C 4898 vs Rust 2,610 is 46.7%, inside the `haikuterm` row's 0.69), and the ratchet
  list is still empty.  `image_ascent`'s definition is `src/image.rs`'s and already landed.

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no image ascent was measured, no
  relief rectangle was drawn, and no clip rectangle was set.**  What is measured is the arm's
  type-checking and the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 1733-4898 text this file has not reached.

## Slice 7 (2026-10-08, status 4.083 - the integrator sets the round number): C 1733-1902

**The slice's text.**  The blank line C 1733, `haiku_translate_transform` (C 1734-1740) and
`haiku_draw_image_glyph_string` (C 1742-1902) - the whole image path: the two box widths, the
`height`/`width`/`x`/`y` folds, the `stippled_p` write, the cursor background, the bitmap arm
(clip rectangle, `gui_intersect_rectangles`, the `memcpy` of `s->img->transform`, the two
`BView_StartClip`/`EndClip` guards, `be_apply_affine_transform` and the three draw calls) and the
relief tail.  Port line count **2,610 -> 2,810** (+200).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Three runs of slice 2's
command, arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 7 primary errors, and 2 of them are in `include/haikugui.rs`** -
  `CONVERT_TO_EMACS_RECT!`'s `width`/`height` assignments are `c_int` -> `c_uint` here, and the
  macro had no conversion written out (Deviation 1).
- **Run 2 (after the mirror repair): `exit 1`, 5 primary errors among 34 error-prefixed lines,
  every one in a mirror** - the same five slices 5 and 6 read (`include/haikuterm.rs` three times,
  `include/systime.rs` once, `include/frame.rs` once).  **The unit's own text has no primary
  error** in either run; 4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 52 rows -> 51.**  The one closed is
`haiku_draw_image_glyph_string`'s `missing function` row (`haiku_translate_transform`'s row had
already closed with slice 5's `haiku_clip_to_row` note - it is the same `missing function` batch,
so the arithmetic is 52 - 1 = 51 with no new row at all).  The 51 that remain are 50
`missing function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 1903-4898 remainder.
**No new spelling row**: `CONVERT_TO_EMACS_RECT`, `gui_intersect_rectangles`, `memcpy`, the six
`be_*`/`BView_*` calls and the `size_of` fold all compare token for token (`sizeof` is on the
checker's skip list, `tools/extract-abi.py` 165).

### Deviations (this slice)

1. **A mirror line this slice changes**: `include/haikugui.rs` 72-96's `CONVERT_TO_EMACS_RECT` /
   `CONVERT_FROM_EMACS_RECT` gain C's own `int`/`unsigned` conversions (`as c_uint` on the two
   `width`/`height` assignments of the `TO` macro, `as c_int` on the twin's), because C relies on
   the implicit conversion of its comma-expression body: the non-X `Emacs_Rectangle` is
   `int x, y; unsigned width, height;` (dispextern.h 66-70) while this arm's `NativeRectangle` is
   `struct haiku_rect` with `int` members (haikugui.h 41-47).  Measured by this file's first use of
   either macro (C 1790: two `E0308` rows, *expected `u32`, found `i32`*), and the shape is
   `include/w32gui.rs` 188-204's pair, which writes the same two casts for its own `LONG`
   rectangle.  The `FROM` twin is the same class in the other direction and is repaired with it;
   **no unit's text exercises `FROM` yet**, so that half is text-only.  P-87's re-scan
   (`tools/check-all-units.sh`, expected `83 unit(s), 0 failure(s)`) is owed to the integrator, as
   this file's Owed 7 already records for slice 1's mirror line.
2. **C 1752's `unsigned long background` and C 1753's `double image_transform[3][3]`** are
   zero-initialised and named (PORTING.md 1.5): C 1780 hands the callee the first's address and C
   1814's `memcpy` fills the second, both of which the port cannot follow - the reading this
   file's C 1113, C 1171, C 1316, C 1405 and C 1648 already took.
3. **`CONVERT_TO_EMACS_RECT! (cr, nr)` is a statement here**, as the mirror's own comment says
   (C's comma expression's value is read by no caller).
4. **C 1815's `sizeof image_transform`** is `core::mem::size_of::<[[c_double; 3]; 3]> ()`
   (72 bytes); `sizeof` is a skipped token in the body comparison.
5. **C 1826-1827's two `x - s->slice.x` / `y - s->slice.y` differences** are `c_int` arithmetic
   (`glyph_slice_bits!`'s reads cast to `c_int`) widened to `c_double` at the call, which is what
   C's prototype for `haiku_translate_transform` takes.
6. **C 1790's `CONVERT_TO_EMACS_RECT` and C 1789's clip rectangle** take the untyped pointers the
   mirrors give them (`haiku_rect` -> `*mut c_void`, the C 544/1729 spellings).

### Owed (this slice's additions)

- **The P-87 re-scan of the mirror line above** (`tools/check-all-units.sh`), which is Owed 7's
  and now covers two mirror changes (slice 1's `include/haikuterm.rs` pair and this
  `include/haikugui.rs` pair).
- Nothing else: `--check` produced no new row (Evidence ②) and the line-drift row is unchanged
  (C 4898 vs Rust 2,810 is 42.6%, inside the `haikuterm` row's 0.69).

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no image was drawn (no
  `BView_DrawBitmap`, no mask compositing), no affine transform was applied, no clip rectangle was
  set and no rectangle/bitmap was blitted.**  The `CONVERT_TO_EMACS_RECT` repair is a *compile*
  reading (the two `E0308` rows before it), not a byte-level one: `c_int` -> `c_uint` is the
  conversion C performs implicitly, and the frozen clang's own layout for both records is asserted
  where each is declared.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 1903-4898 text this file has not reached.

## Slice 8 (2026-10-08, status 4.084 - the integrator sets the round number): C 1903-2074

**The slice's text.**  The blank line C 1903 and `haiku_draw_glyph_string` (C 1904-2074): the draw
lock, the two `stippled_p` writes, the right-overhang loop, the box/clip decision, the five-arm
dispatch, the prev/next overhang loops with the two `save` restores, and the row's `stipple_p`
write.  Port line count **2,810 -> 3,009** (+199).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Two runs of slice 2's command,
arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 12 primary errors, 3 in the unit's own text** - `glyph_string_bits!
  (for_overlaps, …)` returns the packed **integer**, not a `bool` (include/dispextern.rs 1859-1863),
  so C's `!s->for_overlaps` / `if (s->for_overlaps)` have to be spelled `== 0` / `!= 0`;
  4 mirror rows were new too (see run 2).
- **Run 2 (final revision): `exit 1`, 5 primary errors among 34 error-prefixed lines, every one in
  a mirror** - the same five slices 5-7 read (`include/haikuterm.rs` three times, `include/systime.rs`
  once, `include/frame.rs` once).  **The unit's own text has no primary error**, 4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 51 rows -> 50.**  The one closed is
`haiku_draw_glyph_string`'s `missing function` row, and one new row appeared and is discharged:
**`function haiku_draw_glyph_string does not mention break, case, cmp, default, switch`** (C 1967's
`switch` and its five `case` labels are the port's `match`, and C 1971's `u.cmp.automatic` read goes
through this unit's `glyph_u_cmp_bits!`) - a `docs/ports/macro-body-notes.tsv` row with its reason.
The 50 that remain are 49 `missing function` rows plus `missing macro ASSIGN_CURSOR`, all inside the
C 2075-4898 remainder.

### Deviations (this slice)

1. **`for_overlaps` is compared against `0`** (`!= 0` / `== 0`) at C 1916/1944/1964/1970/1978/1991/
   2001: the mirror's arm returns the three-bit field's value as an integer
   (include/dispextern.rs 1859-1863), the spelling slice 3's C 1299 and `pgtkterm.rs`'s own copy of
   this C function already use.  The two `bool_bf` members that *are* booleans (`stippled_p`,
   `background_filled_p`) keep their `bool` arms, and C 1914/1927-1928's `&& face->stipple` writes
   the `!= 0` test the `bool_bf` member needs.
2. **`s->right_overhang`/`s->left_overhang` are plain `int` members of the mirror's
   `glyph_string`** (include/dispextern.rs 1631/1636), so C's four truth tests are `!= 0`
   comparisons rather than bit accessors.
3. **C 1967-1999's `switch`** is the port's `match` on `glyph_bits! (type, …)` with the same five
   arms in C's order and `default:` as `_` (the `emacs_abort ()` arm), the `xterm.rs` 12816 shape.
4. **C 1921-1923's and C 2012/2037's `for` heads** are the init/`while`/step trios with the
   increments at the foot, C's order (`width += next->width, next = next->next` becomes two
   statements; neither loop has a `continue`), and C 1942's mid-block `int box_filled_p = 0;` is a
   `let` at the same point in the body.
5. **C 2018/2043's `enum draw_glyphs_face save = prev->hl;`** keeps C's `draw_glyphs_face` type
   (include/dispextern.rs) and is restored at C 2029/2055 exactly where C restores it.

### Owed (this slice's additions)

- **Nothing new**: the one `--check` row is discharged in `docs/ports/macro-body-notes.tsv`
  (Evidence ②), and the line-drift row is unchanged (C 4898 vs Rust 3,009 is 38.6%, inside the
  `haikuterm` row's 0.69).  The mirror-change re-scan (Owed 7) is unchanged by this slice: it
  touches no mirror file.

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no glyph string was drawn, no clip was
  started or ended, no box was filled and no overhang was redrawn.**  What is measured is the arm's
  type-checking and the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 2075-4898 text this file has not reached.

## Slice 9 (2026-10-08, status 4.085 - the integrator sets the round number): C 2075-2219

**The slice's text.**  The blank line C 2075 and the three items `haiku_after_update_window_line`
(C 2076-2129), `haiku_set_window_size` (C 2131-2167) and `haiku_draw_hollow_cursor` (C 2169-2219).
Port line count **3,009 -> 3,208** (+199).  Three prototypes this slice calls are declared at the
use site (`get_phys_cursor_glyph`, window.h 1186; `get_phys_cursor_geometry`, dispextern.h
3652-3653; `do_pending_window_change`, dispextern.h 3923; plus `lookup_basic_face`,
dispextern.h 3813, which `include/dispextern.rs` 4051 records as owed rather than declared, and
`XSETFRAME`, the `src/abi_shim.rs` 1501 body of frame.h 926's macro), and `f->want_fullscreen`
gets a unit-local reader over the mirror's measured unit/shift/mask constants (the
`haikufns.rs` 1313-1321 shape).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Three runs of slice 2's
command, arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 8 primary errors, 5 in the unit's own text** - four spellings the mirrors
  decide (`FRAME_VISIBLE_P` is a **fn**, not a `!`-macro, frame.rs 1120; `resolved_level` is a
  **bit accessor**, dispextern.rs 587; `lookup_basic_face` is declared **nowhere**; `XSETFRAME` is
  the mirror's **macro**, frame.rs 776-780, so it is `XSETFRAME! (frame, f)`) plus `E0381` for the
  three `get_phys_cursor_geometry` outputs.
- **Run 2: `exit 1`, 9 primary errors, 4 in the unit's own text** - the three `E0381`s moved to
  C 2135's `frame` and C 2188's `x`/`y`/`h` (Deviation 2).
- **Run 3 (final revision): `exit 1`, 5 primary errors among 34 error-prefixed lines, every one in
  a mirror** - the same five slices 5-8 read (`include/haikuterm.rs` three times,
  `include/systime.rs` once, `include/frame.rs` once).  **The unit's own text has no primary
  error**, 4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 50 rows -> 47.**  The three closed
are the `missing function` rows for the three items this slice lands; the 47 that remain are 46
`missing function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 2220-4898 remainder.
**No new spelling row**: the two comma-expression conditions, the `want_fullscreen` read, the
`XSETFRAME` call and the whole phys-cursor path compare token for token.

### Deviations (this slice)

1. **C 2087-2093's two comma-expression conditions** are written as `&& { … ; test }` blocks (the
   `xterm.rs` 8676-8686 shape): C's `(f = XFRAME (w->frame), width = …, width != 0)` becomes the
   block's two assignments then the test, in C's order, and the same for `height`.  `eassert (w)`
   is `eassert! (! w.is_null ())` (the `xterm.rs` 8658 spelling).
2. **C 2135's `Lisp_Object frame` and C 2173's `x`/`y`/`h`** are zero-initialised and named
   (PORTING.md 1.5): C 2140 and C 2188 hand the callee their addresses - `Qnil ()` for the
   `Lisp_Object` (the `haikufns.rs` 2090 reading) and `0` for the three `int`s.
3. **`f->want_fullscreen`** (frame.h 460, the four-bit `ENUM_BF`) is read through a unit-local
   `want_fullscreen!` built on the mirror's `FRAME_WANT_FULLSCREEN_UNIT`/`_SHIFT`/`_MASK`
   (frame.rs 607-757): the mirror carries the constants and **no** accessor, which is the
   `haikufns.rs` 1313-1321 reading of the same member (P-80's masked accessor rule).
4. **`FRAME_VISIBLE_P (f)` is the mirror's function** (frame.rs 1120), called without the `!`
   suffix C's macro name suggests - the `androidfns.rs` 3473 spelling of the same member.
5. **`cursor_glyph->resolved_level`** is `glyph_bits! (resolved_level, (*cursor_glyph))`
   (dispextern.rs 587-590; the field is a seven-bit run, C 553), and C 2199-2200's `pixel_width`
   (a `short`) is cast to `c_int` on both sides of its comparison and subtraction.
6. **Four use-site declarations** (P-72 rule 4, each with C's own prototype in its `/// C:` note):
   `get_phys_cursor_glyph` (window.h 1186), `get_phys_cursor_geometry` (dispextern.h 3652-3653),
   `do_pending_window_change` (3923) and `lookup_basic_face` (3813).  **`XSETFRAME` is not one of
   them**: `include/frame.rs` 776-780 already carries C's macro, so C 2140's `XSETFRAME (frame, f)`
   is the port's `XSETFRAME! (frame, f)` (the `androidfns.rs` 1214 spelling).  A first draft
   declared it as an extern and `tools/check-mirror-dupes.py` failed it in one line
   (`XSETFRAME is already declared by femacs/src/include/frame.rs (in the include graph)`) - the
   gate caught the duplicate before the commit, which is why the note is here.
7. **`haiku_set_window_size`'s `change_gravity` parameter is unused in C's body** (C 2132), so the
   port names it and drops it with `let _ = change_gravity;` (the unit's `unused_variables`
   allow would cover it; naming it keeps the intent visible).

### Owed (this slice's additions)

- **Nothing new**: `--check` produced no new row (Evidence ②), the line-drift row is unchanged
  (C 4898 vs Rust 3,208 is 34.5%, inside the `haikuterm` row's 0.69), and the mirror-change
  re-scan of Owed 7 is unchanged (this slice touches no mirror file).
- **`lookup_basic_face`'s mirror gap is now exercised**: `include/dispextern.rs` 4051 records it as
  owed rather than declared, and this slice is the first unit to call it - the declaration here is
  the per-unit form that gap's own note prescribes, so the mirror round can land the prototype with
  this use as its evidence.

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no window line was redrawn, no frame
  was resized, no fullscreen parameter was read at runtime and no hollow cursor was drawn.**  What
  is measured is the arm's type-checking and the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 2220-4898 text this file has not reached.

## Slice 10 (2026-10-08, status 4.086 - the integrator sets the round number): C 2220-2397

**The slice's text.**  The blank line C 2220 and the four items `haiku_draw_bar_cursor`
(C 2221-2316), `haiku_draw_window_cursor` (C 2318-2365), `haiku_show_hourglass` (C 2367-2381) and
`haiku_hide_hourglass` (C 2383-2397).  Port line count **3,208 -> 3,430** (+222).  Two prototypes
are declared at the use site (`draw_phys_cursor_glyph`, dispextern.h 3649-3651;
`draw_fringe_bitmap`, dispextern.h 3703), `w->phys_cursor_on_p` gains a setter arm on this unit's
own `window_bits!` (the `pgtkterm.rs` 2875-2882 shape), and `MATRIX_ROW!`/`FRAME_CURSOR_COLOR!` are
the mirrors'.

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Three runs of slice 2's
command, arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 7 primary errors, 2 in the unit's own text** - `MATRIX_ROW!` against the
  mirror's untyped `current_matrix` (window.rs 104) and `glyph->face_id`, which is a **bit field**
  (dispextern.rs 594), so it is `glyph_bits! (face_id, …)`.
- **Run 2 (after both repairs): `exit 1`, 5 primary errors, none in the unit's own text** - the
  same five slices 5-9 read.
- **Run 3 (final revision, after the `phys_cursor_on_p` arm moved onto `window_bits!`): `exit 1`,
  5 primary errors among 34 error-prefixed lines, every one in a mirror.**  **The unit's own text
  has no primary error**, 4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 47 rows -> 43.**  The four closed
are the `missing function` rows for the four items this slice lands; one new row appeared and is
discharged: **`function haiku_draw_window_cursor does not mention break, case, default, switch`**
(C 2338's `switch` and its five `case` labels are the port's `match`) - a
`docs/ports/macro-body-notes.tsv` row with its reason.  The 43 that remain are 42 `missing
function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 2398-4898 remainder.

### Deviations (this slice)

1. **`w->phys_cursor_on_p` (window.h 438) is written through this unit's `window_bits!`**, which
   gains the `phys_cursor_on_p` two-argument arm (the masked store over
   `WINDOW_PHYS_CURSOR_ON_P_SHIFT`, `include/window.rs` 191; the `pgtkterm.rs` 2875-2882 copy of the
   same arm).  A first draft named the macro `window_bits_phys_cursor_on_p_set!`, which put the
   member's name inside the macro *name* and left `phys_cursor_on_p` missing from the body
   comparison - the `--check` row is what caught it, and the arm form keeps C's member token at the
   call site (that lane's own reading of the same member).
2. **C 2231's `dummy_x`/`dummy_y`/`dummy_h`** are zero-initialised and named (PORTING.md 1.5): C
   2300 hands the callee their addresses - this file's C 1752/C 2135 reading.
3. **`MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix, …)`**: the mirror types
   `window.current_matrix` as the untyped `*mut c_void` (window.rs 104), so the pointer is cast for
   the mirror's macro - the `dispnew.rs` 2807 spelling of the same call.
4. **`glyph->face_id`** is `glyph_bits! (face_id, (*cursor_glyph))` (dispextern.rs 594; C 553's
   22-bit run), and `pixel_width` (a `short`) is cast to `c_int` on every one of this slice's five
   uses.
5. **`FRAME_CURSOR_COLOR! (f).pixel` is `c_ulong`** (include/haikuterm.rs 445, `Emacs_Color`'s
   member, dispextern.rs 27), so C 2260's comparison needs no cast while C 2261/2263's two
   assignments to the `uint32_t foreground` keep C's narrowing as `as uint32_t`.
6. **`haiku_draw_window_cursor`'s unused `x`/`y`/`active_p`** (C 2319-2321; C's body reads only
   `on_p`, `glyph_row`, `cursor_type` and `cursor_width`) are named and dropped with
   `let _ = (x, y, active_p);` (Deviation 7 of slice 9's `change_gravity`).

### Owed (this slice's additions)

- **Nothing new**: the one `--check` row is discharged in `docs/ports/macro-body-notes.tsv`
  (Evidence ②), the line-drift row is unchanged (C 4898 vs Rust 3,430 is 30.0%, inside the
  `haikuterm` row's 0.69), and no mirror file is touched (Owed 7 unchanged).

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no bar cursor was drawn, no window
  cursor was dispatched, and neither hourglass was shown or hidden.**  What is measured is the
  arm's type-checking and the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 2398-4898 text this file has not reached.

## Slice 11 (2026-10-08, status 4.087 - the integrator sets the round number): C 2398-2522

**The slice's text.**  The blank line C 2398 and the five items `haiku_compute_glyph_string_overhangs`
(C 2399-2428), `haiku_draw_vertical_window_border` (C 2430-2446),
`haiku_set_scroll_bar_default_width` (C 2448-2458), `haiku_set_scroll_bar_default_height`
(C 2460-2470) and `haiku_draw_window_divider` (C 2472-2522).  Port line count **3,430 -> 3,583**
(+153).  Three `FRAME_CONFIG_SCROLL_BAR_*` setters are added to the file's own macro block
(Deviation 1).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Two runs of slice 2's command,
arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 6 primary errors, 1 in the unit's own text** - C 2466's `BScrollBar_default_size
  (true)`: C's prototype is `extern int BScrollBar_default_size (int);` (haiku_support.h 626), so
  the mirror's `c_int` is right and the `bool` argument is C's own conversion (Deviation 4).
- **Run 2 (final revision): `exit 1`, 5 primary errors among 34 error-prefixed lines, every one in
  a mirror** - the same five slices 5-10 read (`include/haikuterm.rs` three times,
  `include/systime.rs` once, `include/frame.rs` once).  **The unit's own text has no primary
  error**, 4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 43 rows -> 38.**  The five closed are
the `missing function` rows for the five items this slice lands, and **no new spelling row
appeared**: the two `text_extents`/`composition_gstring_width` calls, the six `BView_*` calls, the
`FRAME_FOREGROUND_PIXEL` reads and the three setters all compare token for token.  The 38 that
remain are 37 `missing function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 2523-4898
remainder.

### Deviations (this slice)

1. **Three `FRAME_CONFIG_SCROLL_BAR_{WIDTH,HEIGHT,LINES}` setters join this file's macro block**
   (C 2456-2457 and C 2468-2469 are C's l-value macro writes; `include/frame.rs` carries the
   readers as functions at 1294-1303 and no writers): the port adds
   `FRAME_CONFIG_SCROLL_BAR_WIDTH! (f, v)` / `_HEIGHT` / `_LINES` over the mirror's
   `config_scroll_bar_*` members (frame.rs 257-260), the shape
   `FRAME_CONFIG_SCROLL_BAR_COLS!` (this file, C 711) already had.  Macros and functions live in
   different Rust namespaces, so the readers keep their names.
2. **C 2399-2428's two arms** reuse the `pgtkterm.rs` 1797-1840 readings of the same C function:
   `font_metrics` zero-initialised (PORTING.md 1.5 - C 2411 hands its address to the font driver's
   slot), the `text_extents` slot unwrapped from `font_driver`, the two `metrics.rbearing >
   metrics.width` / `metrics.lbearing < 0` folds cast once each (the `c_short` member arithmetic
   widened to `c_int`, which is what the assignment to `s->right_overhang` needs), and
   `composition_gstring_width`'s two `ptrdiff_t` arguments and `*mut c_void` metrics pointer
   written out.  **haiku's C differs from pgtk's at C 2411**: it passes `s->char2b` and `s->nchars`
   straight to `text_extents` (pgtk builds a local `code` array first), so this port does not.
3. **`s->cmp->rbearing`/`pixel_width`/`lbearing`** are read through the `*mut composition` cast the
   mirror's untyped `glyph_string.cmp` needs (slice 8's `COMPOSITION_GLYPH` call is the same
   cast), and each of the three differences is widened with `as c_int`.
4. **C 2466's `BScrollBar_default_size (true)`** is `BScrollBar_default_size (true as c_int)`:
   C's own prototype takes an `int` (haiku_support.h 626) and C's `true` is the conversion, which
   PORTING.md 1.1's written-out-cast rule spells out.  (C 2454's `0` needs no cast.)
5. **The `redisplay_interface` table at C 3149-3170 takes this slice's first, second and fifth
   functions' addresses** (`haiku_compute_glyph_string_overhangs` C 3158,
   `haiku_draw_vertical_window_border` C 3164, `haiku_draw_window_divider` C 3165), and it also
   takes four functions earlier slices landed as plain `unsafe fn` (`haiku_draw_glyph_string`
   C 3159, `haiku_draw_window_cursor` C 3163, `haiku_show_hourglass` C 3168,
   `haiku_hide_hourglass` C 3169).  **Their C function-pointer ABI (`unsafe extern "C" fn`) is owed
   to the slice that lands the table**, which is the `androidfns.rs` slice-13 precedent exactly
   (that slice changed seventeen setters to `unsafe extern "C" fn` when its table landed and
   measured `E0308` before doing so).  This slice therefore keeps `unsafe fn` and records the debt
   rather than splitting the change across two rounds.

### Owed (this slice's additions)

- **The `redisplay_interface` table round** (Deviation 5): eight functions change to
  `unsafe extern "C" fn` when C 3149-3170 lands, four of them from slices 8 and 10.
- **Nothing else**: `--check` produced no new row (Evidence ②) and the line-drift row is unchanged
  (C 4898 vs Rust 3,583 is 26.8%, inside the `haikuterm` row's 0.69).

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no font metrics were measured, no
  overhang was folded, no vertical border or divider was drawn and no scroll-bar default size was
  asked for.**  What is measured is the arm's type-checking and the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 2523-4898 text this file has not reached.

## Slice 12 (2026-10-08, status 4.088 - the integrator sets the round number): C 2523-2645

**The slice's text.**  The blank line C 2523 and the three items `haiku_condemn_scroll_bars`
(C 2524-2544), `haiku_redeem_scroll_bar` (C 2546-2620) and `haiku_judge_scroll_bars` (C 2622-2645).
Port line count **3,583 -> 3,734** (+151).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Two runs of slice 2's command,
arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 5 primary errors, 1 in the unit's own text** - `WINDOW_HAS_HORIZONTAL_SCROLL_BAR`
  is the mirror's **function** (include/window.rs 472, whose body is `false` on this target), not a
  `!`-macro like its vertical sibling (window.rs 691) - so the two spellings differ in one word and
  the probe is what separates them (the `pgtkterm.rs` 6374 reading of the same member).
- **Run 2 (final revision): `exit 1`, 5 primary errors among 34 error-prefixed lines, every one in
  a mirror** - the same five slices 5-11 read.  **The unit's own text has no primary error**,
  4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 38 rows -> 35.**  The three closed
are the `missing function` rows for the three items this slice lands; one new row appeared and is
discharged: **`function haiku_redeem_scroll_bar does not mention goto`** (C 2564's `goto horizontal`
is the port's `break 'horizontal` out of a labelled block, PORTING.md 2.2) - a
`docs/ports/macro-body-notes.tsv` row with its reason.  The 35 that remain are 34 `missing
function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 2646-4898 remainder.

### Deviations (this slice)

1. **C 2564's `goto horizontal` is `break 'horizontal`**, and the block C 2553-2585 opens carries the
   label: the jump leaves exactly the statements C's label is after, so the port wraps the vertical
   half in `'horizontal: { … }` and the horizontal half follows it (the `androidfns.rs`
   `android_toggle_visible_pointer` reading of the same shape; the `goto` token is registered in
   `docs/ports/macro-body-notes.tsv` rather than planted in the body).
2. **C 2640's chained assignment `b->next = b->prev = Qnil;`** is the two stores C performs, in
   C's order (slice 1's Deviation 10 rule for the three earlier chained assignments).
3. **C 2633's `for (; ! NILP (bar); bar = next)`** is the `while` with C's step at the foot, and
   `next`/`b` keep C's scopes (the loop's `b` is declared inside the body, as C declares it).
4. **The four `XSCROLL_BAR! (…)->next`/`->prev` walks** go through the mirror's macro
   (include/haikuterm.rs 390) on C's `Lisp_Object` values, and the four `XSETVECTOR!` calls are the
   mirror's (include/lisp.rs) - no cast is needed anywhere in this slice.

### Owed (this slice's additions)

- **Nothing new**: the one `--check` row is discharged in `docs/ports/macro-body-notes.tsv`
  (Evidence ②), the line-drift row is unchanged (C 4898 vs Rust 3,734 is 23.8%, inside the
  `haikuterm` row's 0.69), no mirror file is touched, and the `redisplay_interface` table's
  function-pointer ABI (Owed 1 of slice 11) is unchanged.

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no scroll bar was condemned, redeemed
  or judged, and no `haiku_scroll_bar_remove` was called.**  What is measured is the arm's
  type-checking and the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 2646-4898 text this file has not reached.

## Slice 13 (2026-10-08, status 4.089 - the integrator sets the round number): C 2646-2745

**The slice's text.**  The blank line C 2646, `haiku_scroll_bar_create` (C 2647-2690) and
`haiku_set_horizontal_scroll_bar` (C 2692-2745).  Port line count **3,734 -> 3,908** (+174).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Four runs of slice 2's
command, arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 7 primary errors, 2 in the unit's own text** - C 2674's `horizontal_p` goes to
  `be_make_scroll_bar_for_view`'s `int` parameter (haiku_support.h 624), and
  `WINDOW_SCROLL_BAR_AREA_Y` is a **macro** (`!`), the third name in this file whose mirror form is
  the opposite of what its sibling suggests (Deviation 3).
- **Run 2: `exit 1`, 6 primary errors** - the `!` fix exposed a **mirror gap**:
  `WINDOW_BOX_HEIGHT_NO_MODE_LINE!`'s body (include/dispextern.rs 1308 and its two siblings) calls
  `estimate_mode_line_height` (dispextern.h 3582), which **no mirror declares**, so the unit
  declares it at the use site (Deviation 4) - the `lookup_basic_face` class again.
- **Run 3: `exit 1`, 7 primary errors, 2 in the unit's own text** - `E0381` for C 2698's
  `window_x`/`window_width` (Deviation 1).
- **Run 4 (final revision): `exit 1`, 5 primary errors among 34 error-prefixed lines, every one in
  a mirror** - the same five slices 5-12 read.  **The unit's own text has no primary error**,
  4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 35 rows -> 33.**  The two closed are
the `missing function` rows for the two items this slice lands; the 33 that remain are 32 `missing
function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 2746-4898 remainder.  The
`function haiku_set_horizontal_scroll_bar does not mention 0` row run 2 read **disappeared with
Deviation 1's zeroing** (the port's own `= 0` supplies the token C 2703's two `0` arguments asked
for), so no `macro-body-notes.tsv` row was needed this slice.

### Deviations (this slice)

1. **C 2698's `window_x`/`window_width`** are zero-initialised and named (PORTING.md 1.5): C 2703
   hands the callee their addresses - this file's C 1518/C 1752 reading.
2. **C 2661's `ALLOCATE_PSEUDOVECTOR (struct scroll_bar, prev, PVEC_OTHER)`** is
   `ALLOCATE_PSEUDOVECTOR! (scroll_bar, prev, pvec_type::PVEC_OTHER)`: the mirror's macro takes the
   type and a `pvec_type` path (the `alloc.rs` 4139 spelling of the same macro).
3. **`WINDOW_SCROLL_BAR_AREA_Y! (w)` is a macro** (include/window.rs 827) while
   `WINDOW_HAS_HORIZONTAL_SCROLL_BAR (w)` is a **function** (472) - the mirror's two spellings for
   two names one line apart in C's own text, which the probe separates (this is the third pair of
   that kind in this lane, after slice 9's `FRAME_VISIBLE_P` and slice 12's scroll-bar pair).
4. **`estimate_mode_line_height` (dispextern.h 3582) is declared at the use site** because the
   mirror's `WINDOW_BOX_HEIGHT_NO_MODE_LINE!` family calls it without any mirror declaring it
   (P-72 rule 4; `include/dispextern.rs` 1308/1338/1368 are the three call sites).  The declaration
   lives in the unit's extern block, which is in scope for the mirror's macro expansion because
   `include!` is textual - the `lookup_basic_face` reading of slice 9.
5. **C 2674's `bool` flag to `be_make_scroll_bar_for_view`** is `horizontal_p as c_int`: C's
   prototype takes an `int` (haiku_support.h 624) and C's `bool` is the conversion, so the port
   writes it out (PORTING.md 1.1; the `BScrollBar_default_size` reading of C 2466).
6. **`bar->dragging`/`bar->update`/`bar->horizontal`** keep C's own constants (`0`, `-1`,
   `horizontal_p`) on the mirror's `scroll_bar` record (include/haikuterm.rs 342-380), and
   `XSETWINDOW!`/`XSETVECTOR!` are the mirrors' macros.

### Owed (this slice's additions)

- **Nothing new**: no `--check` row for this slice (Evidence ②), the line-drift row is unchanged
  (C 4898 vs Rust 3,908 is 20.2%, inside the `haikuterm` row's 0.69), no mirror file is touched,
  and the `redisplay_interface` table's ABI debt (Owed 1 of slice 11) is unchanged.

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no scroll-bar pseudovector was
  allocated, no `be_make_scroll_bar_for_view` or `BView_*` call was made and no thumb was set.**
  What is measured is the arm's type-checking and the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 2746-4898 text this file has not reached.

## Slice 14 (2026-10-08, status 4.090 - the integrator sets the round number): C 2746-2803

**The slice's text.**  The blank line C 2746 and `haiku_set_vertical_scroll_bar` (C 2747-2803), the
vertical twin of slice 13's horizontal function.  Port line count **3,908 -> 3,926** (+18 in the
two blank/comment lines the port adds; the C body is 57 lines).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  **One run**: `exit 1`, 5
primary errors among 34 error-prefixed lines, **every one in a mirror** (the same five slices 5-13
read: `include/haikuterm.rs` three times, `include/systime.rs` once, `include/frame.rs` once).
**The unit's own text has no primary error on the first run**, 4 warnings - the slice reuses only
shapes the previous thirteen slices measured (`WINDOW_SCROLL_BAR_AREA_X!`/`_WIDTH!` are macros like
C 2706's `_Y!`, `WINDOW_HAS_VERTICAL_SCROLL_BAR!` is a macro while its horizontal sibling is a
function, `window_box`'s null arguments, `haiku_scroll_bar_create`'s `false` flag for C 2772's
`false`, and this file's own C 442-495 `haiku_set_scroll_bar_thumb`).

**Evidence ② - the file's own gate (`extract-abi.py --check`): 33 rows -> 32.**  The one closed is
`haiku_set_vertical_scroll_bar`'s `missing function` row, and **no new row appeared**: the four
`WINDOW_*` readers, the three `BView_*`/`be_*` calls and the two setters all compare token for
token.  The 32 that remain are 31 `missing function` rows plus `missing macro ASSIGN_CURSOR`, all
inside the C 2804-4898 remainder.

### Deviations (this slice)

1. **C 2753's `window_y`/`window_height`** are zero-initialised and named (PORTING.md 1.5): C 2759
   hands the callee their addresses - slice 13's C 2698 reading, one function earlier.
2. **The two area readers are macros** (`WINDOW_SCROLL_BAR_AREA_X!`/`_WIDTH!`,
   include/window.rs) and `WINDOW_HAS_VERTICAL_SCROLL_BAR!` is a macro as its C name suggests
   (window.rs 691), unlike the *horizontal* accessor slice 12 measured as the mirror's function -
   the two siblings differ in the mirror and this slice is the first text to use the vertical one's
   `eassert`.
3. **C 2772 passes C's `false`** to `horizontal_p` (the port's `bool`), which
   `be_make_scroll_bar_for_view`'s `int` parameter then takes as slice 13's `as c_int` cast
   (`haiku_scroll_bar_create`'s own conversion) - no new cast site here.
4. **C 2796's `haiku_set_scroll_bar_thumb`** is this file's own C 442-495 definition (landed at
   slice 1, line 910), the vertical twin of the horizontal one slice 13 called.

### Owed (this slice's additions)

- **Nothing new**: no `--check` row for this slice (Evidence ②), the line-drift row is unchanged
  (C 4898 vs Rust 3,926 is 19.8%, inside the `haikuterm` row's 0.69), no mirror file is touched,
  and the `redisplay_interface` table's ABI debt (Owed 1 of slice 11) is unchanged.

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no vertical scroll bar was created,
  moved or given a thumb.**  What is measured is the arm's type-checking and the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 2804-4898 text this file has not reached.

## Slice 15 (2026-10-08, status 4.091 - the integrator sets the round number): C 2804-2892

**The slice's text.**  The blank line C 2804 and `haiku_draw_fringe_bitmap` (C 2805-2892).  Port
line count **3,926 -> 4,082** (+156, of which ~62 lines are the use-site record block Deviation 1
describes).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Two runs of slice 2's command,
arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 12 primary errors, 4 in the unit's own text** - `draw_fringe_bitmap_params`,
  `get_overlay_p`, `gui_define_fringe_bitmap` and `max_used_fringe_bitmap` are all behind
  `dispextern_face_records` in the mirror, so the unit carries the record and names the other three
  itself (Deviation 1).
- **Run 2 (final revision): `exit 1`, 5 primary errors among 34 error-prefixed lines, every one in
  a mirror** - the same five slices 5-14 read (`include/haikuterm.rs` three times,
  `include/systime.rs` once, `include/frame.rs` once).  **The unit's own text has no primary
  error**, 4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 32 rows -> 31.**  The one closed is
`haiku_draw_fringe_bitmap`'s `missing function` row, and one new row appeared and is discharged:
**`function haiku_draw_fringe_bitmap does not mention cursor_p, overlay_p, uint32`** (the two
`bool_bf` members are read through the unit's accessors, and C 2813's `uint32` is the port's
`uint32_t`) - a `docs/ports/macro-body-notes.tsv` row with its reason.  The 31 that remain are 30
`missing function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 2893-4898 remainder.

### Deviations (this slice)

1. **`struct draw_fringe_bitmap_params` is a use-site copy** (P-72 rule 3/P-80): the mirror's record
   *and* its two readers are behind `dispextern_face_records`, which this unit's arm set does not
   carry, so the copy stands in the unit with the same four assertions the `xterm.rs` 9295-9345
   copy of the same C record carries (`sizeof` 64, align 8, `flags`@52, `face`@56) - and the two bit
   constants (`DRAW_FRINGE_BITMAP_CURSOR_P`/`_OVERLAY_P`, C 2230-2231's bits 0 and 1 of that byte)
   with the two readers.  The alternative - putting `dispextern_face_records` in this unit's arm set
   - is the one slice 3's Deviation 1 already declined for the `face` record, for the same reason
   (retiring the units' copies must be one deliberate move across all of them).
2. **`max_used_fringe_bitmap` (dispextern.h 3710) and `gui_define_fringe_bitmap` (3711) are
   declared at the use site** because the mirror's own declarations are inside the same gated block
   (P-72 rule 4; the `pgtkterm.rs` 5220 spelling of the second one).
3. **`p->cursor_p`/`p->overlay_p` are read through `get_cursor_p`/`get_overlay_p`** - the two
   accessors C's own `bool_bf` members need (PORTING.md 1.3/P-80), so neither member name appears in
   the body (registered in `docs/ports/macro-body-notes.tsv`, the `mouse_face_defer` class).
4. **`fringe_bmps`'s two subscripts** are `*fringe_bmps.offset ((*p).which as isize)` (this file's
   own C 55 static is a raw pointer, slice 1's declaration), and `max_fringe_bmp` is this file's C 58
   static; C 2855's `p->which && …` is the `!= 0` test the `int` wants.
5. **The five `BView_*` calls and the two colour casts** keep C's order and C's narrowing
   (`face->background`/`foreground` are `c_ulong` in this unit's own `face` copy, the mirror's
   setters take `uint32_t`).

### Owed (this slice's additions)

- **Nothing new beyond the record copy itself**: the one `--check` row is discharged (Evidence ②),
  the line-drift row is unchanged (C 4898 vs Rust 4,082 is 16.7%, inside the `haikuterm` row's
  0.69), no *mirror* file is touched, and the `redisplay_interface` table's ABI debt (Owed 1 of
  slice 11) is unchanged.
- **`xterm.rs`'s Owed round is where the mirror's gated record group gets its own copy**, when the
  `dispextern_face_records` flip is taken; this unit's copy is the same temporary that lane's
  (documented in the same words).

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no fringe bitmap was drawn, no stipple
  was tiled and no bitmap was created by `gui_define_fringe_bitmap`.**  What is measured is the
  arm's type-checking and the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 2893-4898 text this file has not reached.

## Slice 16 (2026-10-08, status 4.092 - the integrator sets the round number): C 2893-2980

**The slice's text.**  The blank line C 2893 and the three items `haiku_define_fringe_bitmap`
(C 2894-2915), `haiku_destroy_fringe_bitmap` (C 2917-2926) and `haiku_scroll_run` (C 2928-2980).
Port line count **4,082 -> 4,205** (+123).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  **One run**: `exit 1`, 5
primary errors among 34 error-prefixed lines, **every one in a mirror** (the same five slices 5-15
read).  **The unit's own text has no primary error on the first run**, 4 warnings - the slice
reuses this file's own `fringe_bmps`/`max_fringe_bmp` statics (C 55/58), the mirror's `BBitmap_*`
surface and `BView_CopyBits`, lisp.rs's `xmalloc`/`xrealloc`/`memory_full_up`, and one new use-site
declaration (`gui_clear_cursor`, dispextern.h 3657 - Deviation 1).

**Evidence ② - the file's own gate (`extract-abi.py --check`): 31 rows -> 28.**  The three closed
are the `missing function` rows for the three items this slice lands, and **no new row appeared**:
the two `x*alloc` calls, the three `BBitmap_*` calls, the four `BView_*` calls and the two
`WINDOW_TO_FRAME_PIXEL_Y!` reads all compare token for token.  The 28 that remain are 27 `missing
function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 2981-4898 remainder.

### Deviations (this slice)

1. **`gui_clear_cursor` (dispextern.h 3657) is declared at the use site** (P-72 rule 4): no mirror
   carries it - the `pgtkterm.rs` 4448 declaration of the same prototype; `src/xdisp.rs` defines it.
2. **C 2902-2903's `!i ? xmalloc (…) : xrealloc (…)`** is the port's `if i == 0 { … } else { … }`
   over the same two functions, with C's `sizeof (void *)` written as
   `std::mem::size_of::<*mut c_void> ()` and the two results cast to the `*mut *mut c_void` the
   static's type is (slice 1's C 55 declaration).
3. **C 2906's `fringe_bmps[i++] = NULL;`** is C's store followed by its increment, as two
   statements in C's order (slice 12's `b->next = b->prev = Qnil` rule for C's compound expressions).
4. **C 2933's four `window_box` outputs** are zero-initialised and named (PORTING.md 1.5): C 2934
   hands the callee their addresses - this file's C 1518/C 2698/C 2753 reading.
5. **C 2928-2980's `run` argument** is `*mut run` (include/dispextern.rs's `struct run`, whose
   `height`/`current_y`/`desired_y` this slice reads), and the two `WINDOW_TO_FRAME_PIXEL_Y!` calls
   are the mirror's macro (slice 9's C 1524 reading).

### Owed (this slice's additions)

- **Nothing new**: no `--check` row for this slice (Evidence ②), the line-drift row is unchanged
  (C 4898 vs Rust 4,205 is 14.2%, inside the `haikuterm` row's 0.69), no mirror file is touched,
  and the `redisplay_interface` table's ABI debt (Owed 1 of slice 11) is unchanged.

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no fringe bitmap was created, imported
  or freed, and no scroll run copied any bits.**  What is measured is the arm's type-checking and
  the text/token gates.
- The five mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 2981-4898 text this file has not reached.

## Slice 17 (2026-10-08, status 4.093 - the integrator sets the round number): C 2981-3077

**The slice's text.**  The blank line C 2981 and the C 2982-2987 comment block, `haiku_mouse_position`
(C 2988-3061) and `haiku_flush` (C 3063-3077).  Port line count **4,205 -> 4,331** (+126).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Three runs of slice 2's
command, arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 11 primary errors, 5 in the unit's own text** - C 3007's `f->mouse_moved` is a
  **bit** (frame.h 434) with no mirror accessor, `gui_mouse_grabbed` returns `bool` in this unit's
  own declaration, `remember_mouse_glyph` (dispextern.h 3585) is declared **nowhere**, and the two
  `XSETINT`s hand an `i32` to `make_fixnum`'s `isize`.
- **Run 2 (after `mouse_moved!`/`gui_mouse_grabbed`/`remember_mouse_glyph`): `exit 1`, 8 primary
  errors, 2 in the unit's own text** - the two `XSETINT` conversions.
- **Run 3 (final revision): `exit 1`, 6 primary errors among 34 error-prefixed lines, and every one
  is in a mirror** - the usual `systime`/`frame` pair plus **four** rows in `include/haikuterm.rs`
  (three from the earlier slices and one more instance of slice 1's recorded `FRAME_COMPLETE_P`
  write-arm class, which C 3073's `= true` adds).  **The unit's own text has no primary error**,
  4 warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 28 rows -> 26.**  The two closed are
the `missing function` rows for the two items this slice lands, and **no new row appeared**: the two
`FOR_EACH_FRAME!` calls, the five `x_display_list` member reads/writes and the four `BView_*`/frame
calls all compare token for token.  The 26 that remain are 25 `missing function` rows plus `missing
macro ASSIGN_CURSOR`, all inside the C 3078-4898 remainder.

### Deviations (this slice)

1. **`mouse_moved!` is a unit-local macro** (the `androidterm.rs` 234-238 copy of the same member):
   C 3007's `f->mouse_moved = false` is a one-bit write inside `frame`'s flag run, the mirror names
   the shift (`include/frame.rs` 529) and carries no accessor, so the unit reads/writes the bit
   itself through the file's own `set_frame_flag1!` (slice 1).
2. **`gui_mouse_grabbed` is the unit's own declaration and returns `bool`** (its mirror is absent;
   the unit declares it at the use site), so C 3010's truth test needs no `!= 0` - the opposite of
   the `int`-returning readers C's other files use.
3. **`remember_mouse_glyph` (dispextern.h 3585-3587) is declared at the use site** (P-72 rule 4): no
   mirror carries it, and its fourth parameter is C's `&x_display_list->last_mouse_glyph`, which the
   unit's own `haiku_rect` type spells (`NativeRectangle`'s arm, haikugui.h 41-47).
4. **Both functions carry an ABI debt**: `haiku_flush` is the `redisplay_interface` table's second
   entry (C 3151) and `haiku_mouse_position` is `terminal->mouse_position_hook` (C 4431), so each
   must become `unsafe extern "C" fn` when its referrer lands - the `androidfns.rs` slice-13
   precedent, recorded here so the two referrer slices convert them together with the eight slice
   11 already owes.
5. **C 3055-3056's `XSETINT (*x, screen_x)`** is `XSETINT! (*x, screen_x as isize)`: the mirror's
   macro hands its second argument to `make_fixnum`, so C's `int` -> `EMACS_INT` conversion is
   written out (PORTING.md 1.1; the `BScrollBar_default_size` reading of C 2466).
6. **C 2995's `screen_x`/`screen_y`** are zero-initialised and named (PORTING.md 1.5): C 3035 hands
   the callee their addresses - this file's reading since C 1113.

### Owed (this slice's additions)

- **Two more functions join the ABI debt** (Deviation 4): `haiku_flush` for the table slice,
  `haiku_mouse_position` for the terminal slice - so that round converts ten functions, not eight.
- Nothing else: no `--check` row (Evidence ②) and the line-drift row is unchanged (C 4898 vs Rust
  4,331 is 11.6%, inside the `haikuterm` row's 0.69).

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no mouse position was read, no
  `BView_get_mouse` was called and no frame was flushed.**  What is measured is the arm's
  type-checking and the text/token gates.
- The six mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 3078-4898 text this file has not reached.

## Slice 18 (2026-10-08, status 4.094 - the integrator sets the round number): C 3078-3091

**The slice's text.**  The blank line C 3078 and `haiku_define_frame_cursor` (C 3080-3091).  Port
line count **4,331 -> 4,361** (+30).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  **One run**: `exit 1`, 6 primary
errors among 34 error-prefixed lines, **every one in a mirror** (the same six slice 17 read:
`include/haikuterm.rs` four times, `include/systime.rs` once, `include/frame.rs` once).  **The unit's
own text has no primary error on the first run**, 4 warnings - the slice's only new local name is the
`pointer_invisible!` macro (Deviation 1).

**Evidence ② - the file's own gate (`extract-abi.py --check`): 26 rows -> 25.**  The one closed is
`haiku_define_frame_cursor`'s `missing function` row, and **no new row appeared**: the tooltip guard,
the three-part test and the `current_cursor` store all compare token for token.  The 25 that remain
are 24 `missing function` rows plus `missing macro ASSIGN_CURSOR`, all inside the C 3092-4898
remainder.

### Deviations (this slice)

1. **`pointer_invisible!` is a unit-local macro** (the `androidterm.rs` 240-243 copy of the same
   member): C 3086's `!f->pointer_invisible` is a bit of `frame`'s flag run, the mirror names only
   the shift (`include/frame.rs` 542-546, whose own comment records that the number is
   **per-configuration** - 19 frozen, 22 for x11/pgtk/android/w32, 23 for haiku/X-toolkit), so the
   unit reads and writes it through its own `set_frame_flag1!` (slice 1) - the same shape slice 17's
   `mouse_moved!` took.
2. **This function is the `redisplay_interface` table's cursor entry (C 3160)**, so it joins that
   round's ABI conversion.  **The conversion's size is now known exactly**: the table at C 3140-3170
   hands the address of **seventeen** of this file's functions (`haiku_scroll_run`,
   `haiku_after_update_window_line`, `haiku_flush`, `haiku_draw_fringe_bitmap`,
   `haiku_define_fringe_bitmap`, `haiku_destroy_fringe_bitmap`,
   `haiku_compute_glyph_string_overhangs`, `haiku_draw_glyph_string`, `haiku_define_frame_cursor`,
   `haiku_clear_frame_area`, `haiku_clear_under_internal_border`, `haiku_draw_window_cursor`,
   `haiku_draw_vertical_window_border`, `haiku_draw_window_divider`, `haiku_show_hourglass`,
   `haiku_hide_hourglass`, `haiku_default_font_parameter`), and `haiku_mouse_position` joins them
   for `terminal->mouse_position_hook` (C 4431) - so the table round converts **eighteen** `unsafe fn`
   definitions to `unsafe extern "C" fn`, in the one move the `androidfns.rs` slice-13 precedent
   prescribes.  (Slices 11 and 17 recorded eight and ten; this is the full count, now that the
   table's own text has been read item by item.)

### Owed (this slice's additions)

- **The ABI round's size is now exact** (Deviation 2): eighteen functions, all named above.
- Nothing else: no `--check` row (Evidence ②) and the line-drift row is unchanged (C 4898 vs Rust
  4,361 is 11.0%, inside the `haikuterm` row's 0.69).

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no frame cursor was defined, no
  `BView_set_view_cursor` was called and no `current_cursor` was stored.**  What is measured is the
  arm's type-checking and the text/token gates.
- The six mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 3092-4898 text this file has not reached.

## Slice 19 (2026-10-08, status 4.095 - the integrator sets the round number): C 3092-3138

**The slice's text.**  The blank line C 3092 and `haiku_default_font_parameter` (C 3093-3138).  Port
line count **4,361 -> 4,445** (+84, of which ~45 lines are the four use-site declarations and the
`resource_types` copy Deviation 1 describes).

**Evidence ① - the arm flip's substitute (`xtask probe`, P-86 ③).**  Two runs of slice 2's command,
arm set `have_haiku` + `have_window_system`:

- **Run 1: `exit 1`, 6 primary errors, every one in a mirror** - the unit's own text was clean on
  the first run (the slice's only new local names are the four declarations and the enum copy).
  One `--check` row appeared: **`function haiku_default_font_parameter does not mention
  haiku_display_info`**, because C 3096's `struct haiku_display_info *dpyinfo` was written with the
  mirror's `Display_Info` alias (Deviation 2).
- **Run 2 (final revision): `exit 1`, 6 primary errors among 34 error-prefixed lines, every one in a
  mirror** - the same six slices 17/18 read.  **The unit's own text has no primary error**, 4
  warnings.

**Evidence ② - the file's own gate (`extract-abi.py --check`): 25 rows -> 24.**  The one closed is
`haiku_default_font_parameter`'s `missing function` row, and **the row run 1 produced was discharged
by Deviation 2 rather than by a `macro-body-notes.tsv` entry** - the port can keep C's own tag there.
The 24 that remain are 23 `missing function` rows plus `missing macro ASSIGN_CURSOR`, all inside the
C 3139-4898 remainder.

### Deviations (this slice)

1. **Five names are declared at the use site** (P-72 rule 4, the `androidfns.rs` 129-183 copies of
   the same five): `gui_display_get_arg` (dispextern.h 3994), `gui_default_parameter` (4001),
   `font_open_by_spec` (font.h 897), `font_open_by_name` (font.h 883) and the `resource_types` enum
   with its `RES_TYPE_STRING` member (dispextern.h 3983-3992), because no mirror carries any of
   them.
2. **C 3096's `struct haiku_display_info *dpyinfo` keeps C's own tag** (`*mut haiku_display_info`,
   the record this unit's own `x_display_list` static is typed with) rather than the mirror's
   `Display_Info` alias the first draft used: that spelling makes `haiku_display_info` a token of
   the body (so the `--check` row run 1 reported is discharged instead of registered) and it is the
   same record either way (`Disply_Info` is `dispextern_unit_names::x_display_info`, whose arm is
   this tag).
3. **C 3119-3122's compound literal `(const char *[]) { … , NULL }`** is C's four-entry array as a
   Rust array of `*const c_char` whose fourth element is C's `NULL`, and C 3125's
   `for (i = 0; names[i]; i++)` is the `while ! names[i].is_null ()` with the increment at the foot
   and the `break` C 3130 writes.
4. **C 3112's `!NILP (font_param) ? font_param : …`** is the port's `if`/`else` expression (Rust has
   no ternary), in C's order; C 3097-3098's `NULL, NULL` arguments are `std::ptr::null ()` (the
   prototype's `*const c_char`).

### Owed (this slice's additions)

- **The ABI round is the very next item** (Owed 1 of slice 11, sized at eighteen functions by slice
  18's reading): C 3140-3170's table is `haiku_frame_parm_handlers` plus seventeen of this file's
  functions.
- Nothing else: the line-drift row is unchanged (C 4898 vs Rust 4,445 is 9.3%, inside the
  `haikuterm` row's 0.69) and no mirror file is touched.

### Not claimed (this slice)

- **No runtime evidence.**  No Haiku SDK and no Haiku toolchain here, the frozen configuration
  compiles none of this file, and this lane does not link: **no font parameter was read or
  recorded, no system font was opened and no candidate name was tried.**  What is measured is the
  arm's type-checking and the text/token gates.
- The six mirror errors are not this slice's text, and the `--check` rows that remain are the
  C 3139-4898 text this file has not reached.
