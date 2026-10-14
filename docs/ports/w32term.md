# emacs/src/w32term.c (8414 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32term.o` sits in
> `W32_OBJ` (`emacs/src/Makefile.in` 319, `W32_OBJ=@W32_OBJ@`, filled by `configure.ac` under the
> `HAVE_W32` test that also `AC_DEFINE`s `HAVE_NTGUI`), and `docs/baseline/frozen-build-sources.txt`
> lists no `w32term.o` for this target - so `extract-abi.py`'s checklist for the file is empty
> (P-86 ②) and this is that clause's substitute: the **C line-range table**, the `## Evidence`
> readings and the `## Deviations`.  Port: `femacs/src/w32term.rs` (**4123 lines**), which carries
> **C 1-2715** in three slices - P-86 ④'s **slice 1** (C 1-1002, 1592 lines), **slice 2**
> (C 1003-1949) and this batch's **slice 3** (C 1950-2715, the image page and the underline
> drawing) - with **both arms of every `#if` carried**.
>
> **This ledger overturns its own earlier conclusion.**  Until this batch it read "the frozen
> configuration does not compile this file ⇒ the port is the empty unit plus the note that names
> the gate" (the P-08 reading, `docs/status.md` 3.31).  P-86 ① replaces that with *transcribe the
> text, both arms, no stub*; what the earlier note measured (the file is not in the frozen source
> list, its checklist is empty, its `build-tu` object is 336 bytes) is unchanged and still true -
> it is the *conclusion* that is withdrawn, exactly as `docs/ports/decompress.md` records for its
> own pair.
>
> **Why 1002 and not 1000.**  C 1000 is inside `w32_define_fringe_bitmap` (C 989-1002); the
> neighbouring whole-item boundaries are this item's `}` at **C 1002** (2 lines past 1000),
> `w32_destroy_fringe_bitmap`'s `}` at C 1013 (13 past) and `w32_draw_fringe_bitmap`'s `}` at
> C 887 (113 short, and 987 is the row's last line).  C 1002 is the nearest, so the slice is
> **C 1-1002** and **the next definition is `w32_destroy_fringe_bitmap` (C 1005-1013)**, the first
> item of the next lane's slice.
>
> **Slice 2: C 1004-1949 (this batch).**  The range opens at C 1004 - the blank line after
> slice 1's `}` - and closes at **C 1949**, `w32_draw_glyph_string_box`'s `}`: the
> `Display Iterator` page (its banner is C 1015-1017, its prototype block C 1019-1046) and
> the relief/box drawing C 1709-1949 that follows it.  The neighbouring whole-item
> boundaries are **C 1952** (`w32_image_rotations_p`, the image page's first definition, 3
> lines past) and C 1881 (`w32_draw_box_rect`'s `}`, 68 short), so C 1949 is the nearest and
> **the next definition is `w32_image_rotations_p` (C 1952-1955)**.  The slice is 946 C
> lines; the port's slice is 1349 Rust lines (1.43x, against slice 1's 1.59x).  It adds no
> condition of its own - it is inside the file gate - and the only `#if` it contains are
> C 1274-1287's and C 1719-1721/1731-1734's `#if 0` regions, carried as C's own text.
>
> **The slice's own conditions.**  `tools/check-cond-eval.py src/w32term.c` reads the file as
> **`41 conditions, 2 unknown, 0 disagreements [VERIFIED]`**, and the two the evaluator leaves
> unknown are **both in this slice**: C 98's `#if _WIN32_WINNT < 0x0500 && !defined(MINGW_W64)` and
> C 129's `#if _WIN32_WINNT < 0x0601` (value tests of a macro the frozen table does not carry).
> The other conditions this slice contains are C 34's and C 214's `#ifdef CYGWIN`, C 51's
> `#ifdef WINDOWSNT`, C 55's `#ifndef WINDOWSNT`, C 255's `#ifdef GLYPH_DEBUG`, five `#ifndef`
> guards on SDK names (C 79, 165, 170, 176, 179) and five `#if 0` regions (C 64-66, 134-139,
> 248-250, 264-289, 312-319).
>
> **Acquisition: this batch is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain - `which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
> Homebrew `mingw-w64`/`w64` formula is installed, and no `windows.h` exists under the toolchain
> roots - so there is **no `<windows.h>` to compile or measure against**, **no arm flip on a
> Windows target** and **no layout measurement**.  What this batch carries instead: the frozen
> `build-tu` reading, the P-86 ② machine-checked coverage table, the `#if` verdicts, a per-item C
> citation, a **multi-configuration read-only sweep** of this unit's own text, and an **arm-reach
> pre-flight** over patched `/tmp` copies of the mirrors plus two mutation tests - which are
> `/tmp` harness readings, **not** a Windows-target compile.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, i.e. `HAVE_W32` → `HAVE_NTGUI`) | `feature = "have_ntgui"` (on every item *and* on C 20-66's include graph) | native MS Windows GUI, incl. the Cygwin GUI build |
| C 34-36 `#ifdef CYGWIN` / `#include <fcntl.h>` | `feature = "cygwin"`; the `#else` arm is empty | the Cygwin GUI build |
| C 51-53 `#ifdef WINDOWSNT` / `#include "w32.h"` | `feature = "windowsnt"`; the `#else` arm is empty | the non-Cygwin Windows build |
| C 55-57 `#ifndef WINDOWSNT` / `#include <io.h>` | `not (feature = "windowsnt")` (recorded: `io.h` has no mirror and this slice names nothing from it) | the Cygwin GUI build |
| C 98-118 `#if _WIN32_WINNT < 0x0500 && !defined(MINGW_W64)` | `feature = "win32_winnt_lt_0500"` + `not (feature = "mingw_w64")`; both arms carried, the `#else` arm empty | a pre-Win2k SDK, and MinGW that is not mingw-w64 |
| C 129-156 `#if _WIN32_WINNT < 0x0601` | the **"define it if the SDK has not got it" guard**: C's values and text carried with no `cfg`, the guard recorded (see Deviations) | an SDK without `TOUCHINPUT` |
| C 165/170/176/179 `#ifndef LWA_ALPHA`/`WS_EX_LAYERED`/`SM_CX*` | the same guard reading: constant with C's value, no `cfg` | an SDK without those names |
| C 214-216 `#ifdef CYGWIN` / `int w32_message_fd = -1;` | `all (feature = "have_ntgui", feature = "cygwin")`; the `#else` arm is empty | the Cygwin GUI build |
| C 248-250, 255-257 `#if 0` / `#ifdef GLYPH_DEBUG` (in the prototype block) | the prototype block is dropped (0.1-12); `glyph_debug` is the feature C's arm names | - |
| C 79-81 `#ifndef GET_WHEEL_DELTA_WPARAM` | the guard reading: the `macro_rules!` is unconditional, C's tokens kept | an SDK without that macro |
| C 1274-1287, 1719-1721, 1731-1734 `#if 0` (slice 2), C 2455-2494 `#if 0` (slice 3) | C's own text inside a comment (the `cm.rs` 86 reading) | - |
| C 2114-2148 `#ifdef WINDOWSNT` (slice 3, a *statement* inside `w32_draw_image_foreground`) | `all (feature = "have_ntgui", feature = "windowsnt")` on the GDI+ arm and `not (feature = "windowsnt")` on C 2149-2160; C's own text in a comment | the non-Cygwin Windows build (the GDI+ arm); every build (C's `#else`) |

`HAVE_NTGUI` and `WINDOWSNT` are `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`; the
file has exactly one build condition of its own (the file gate) plus the platform macros above.

## C line ranges

One row per C block, in C's order.  This ledger is the map's ledger for **both** rows of the
`w32term` stem (`src/w32term.c` -> `femacs/src/w32term.rs` and `src/w32term.h` ->
`femacs/src/include/w32term.rs`), so `check-dead-file-coverage.py` reads the same table
against both references and each row's range therefore runs to the line before the next
row's start - the `docs/ports/frame.md` table does the same for `frame.c`/`frame.h`, and it
is what keeps a blank line in one reference from becoming a gap in the other's reading.
The `arm` cell says what the port does with the block.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's licence/declaration block (C 18's `*/` closes it) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20-20 | `#include <config.h>` | ungated (every unit includes `config.rs`) |
| 21-26 | `<signal.h>`, `<stdio.h>`, `"lisp.h"`, `"blockinput.h"`, `"w32term.h"`, `"w32common.h"` | file gate |
| 27-30 | `<wtypes.h>`, `<gdiplus.h>` (recorded: no file in the reference tree), `"w32gdiplus.h"` | file gate |
| 31-33 | `<ctype.h>`, `<errno.h>`, `<sys/stat.h>` | file gate |
| 34-36 | `#ifdef CYGWIN` / `<fcntl.h>` | both arms (`cygwin`) |
| 37-39 | `<imm.h>` (recorded, nothing named in this slice), `<math.h>` (`floor`) | file gate |
| 40-50 | `"coding.h"`, `"frame.h"`, `"fontset.h"`, `"termhooks.h"`, `"termopts.h"`, `"termchar.h"`, `"buffer.h"`, `"window.h"`, `"keyboard.h"`, `"menu.h"` | file gate |
| 51-54 | `#ifdef WINDOWSNT` / `"w32.h"` / `#endif` | both arms (`windowsnt`) |
| 55-58 | `#ifndef WINDOWSNT` / `<io.h>` / `#endif` | both arms (`not (windowsnt)`, recorded) |
| 59-60 | `<shellapi.h>` (recorded: no file in the reference tree) | file gate |
| 61-63 | `"font.h"`, `"w32font.h"` | file gate |
| 64-67 | `#if 0` (`bitmaps/gray.xbm`) | dead text in every configuration, carried as C's own text in a comment |
| 68-78 | `max_fringe_bmp`, `fringe_bmp`, `last_mousemove_x/y` | file gate |
| 79-82 | `#ifndef GET_WHEEL_DELTA_WPARAM` / `#define` / `#endif` | file gate (guard recorded; `macro_rules!` + `HIWORD`) |
| 83-87 | `any_help_event_p` | file gate |
| 88-93 | `extern msh_mousewheel`, `w32_codepage_for_font`, `w32_load_cursor` | file gate (declarations) |
| 94-97 | `one_w32_display_info`, `x_display_list` | file gate |
| 98-119 | `#if _WIN32_WINNT < 0x0500 && !defined(MINGW_W64)` / `WCRANGE` / `GLYPHSET` / `#endif` | both arms (`win32_winnt_lt_0500`, `mingw_w64`) |
| 120-125 | `pfnSetLayeredWindowAttributes`, `pfnPlgBlt` | file gate |
| 126-157 | the touch records: `TOUCHEVENTF_DOWN/UP`, the C 134-139 `#if 0`, `WM_TOUCHMOVE`, `TOUCHINPUT` | guard reading; C 134-139 dead text (comment) |
| 158-164 | the two proc typedefs and their two statics | file gate |
| 165-182 | the five `#ifndef` guards (`LWA_ALPHA`, `WS_EX_LAYERED`, `SM_CXVIRTUALSCREEN`, `SM_CYVIRTUALSCREEN`) | file gate (guards recorded) |
| 183-184 | `w32_system_remap_font` | file gate |
| 185-217 | the caret/thread/scroll-bar/codepage/wheel objects, and C 214-216's `#ifdef CYGWIN` `w32_message_fd` | file gate; C 214-216 both arms (`cygwin`) |
| 218-259 | the forward-declaration block (C 248-250 and 255-257 the two nested conditions) | dropped (PORTING.md 0.1-12), each line recorded in the port |
| 260-263 | the section banner (`Debugging`) | carried as a comment |
| 264-291 | `#if 0`: the `record_event` recorder | dead text in every configuration (comment) |
| 292-301 | `XChangeGC` | file gate |
| 302-311 | `XCreateGC` | file gate |
| 312-320 | `#if 0`: `XGetGCValues` | dead text in every configuration (comment) |
| 321-351 | `w32_show_back_buffer` | file gate |
| 352-376 | `w32_release_paint_buffer` | file gate |
| 377-389 | `w32_get_mouse_wheel_vertical_delta` | file gate |
| 390-402 | `w32_set_clip_rectangle` | file gate |
| 403-422 | `w32_restore_glyph_string_clip` | file gate |
| 423-438 | `w32_get_scale_factor` | file gate |
| 439-449 | the ASCII wave picture | carried as a comment |
| 450-516 | `w32_draw_underwave` | file gate |
| 517-543 | `w32_draw_rectangle` | file gate |
| 544-554 | `w32_fill_rect` | file gate |
| 555-572 | `w32_clear_window` | file gate |
| 573-575 | `#define OPAQUE_FRAME 255` | file gate |
| 576-620 | `w32_set_frame_alpha` | file gate |
| 621-645 | `w32_display_pixel_height`, `w32_display_pixel_width` | file gate |
| 646-654 | the section banner (`Starting and ending an update`) and C 650-653's comment | carried as a comment |
| 655-672 | `w32_update_begin` | file gate |
| 673-686 | `w32_update_window_begin` | file gate |
| 687-712 | `w32_draw_vertical_window_border` | file gate |
| 713-761 | `w32_draw_window_divider` | file gate |
| 762-785 | `w32_update_window_end` | file gate |
| 786-799 | `w32_update_end` | file gate |
| 800-812 | `w32_frame_up_to_date` | file gate |
| 813-819 | `w32_buffer_flipping_unblocked_hook` | file gate |
| 820-833 | `w32_flip_buffers_if_dirty` | file gate |
| 834-896 | `w32_after_update_window_line` (C 858-869's two comma expressions) | file gate |
| 897-988 | `w32_draw_fringe_bitmap` | file gate |
| 989-1003 | `w32_define_fringe_bitmap` | file gate |
| 1004-1013 | `w32_destroy_fringe_bitmap` | file gate |
| 1015-1017 | the `Display Iterator` page banner | carried as a comment |
| 1019-1046 | the "Function prototypes of this page." block (C 1019's comment + C 1021-1045) | dropped (PORTING.md 0.1-12), each line recorded in the port |
| 1049-1101 | `w32_set_cursor_gc` | file gate |
| 1104-1135 | `w32_set_mouse_face_gc` | file gate |
| 1138-1146 | `w32_set_mode_line_face_gc` | file gate |
| 1149-1189 | `w32_set_glyph_string_gc` | file gate |
| 1192-1213 | `w32_set_glyph_string_clipping` | file gate |
| 1215-1232 | `w32_set_glyph_string_clipping_exactly` | file gate |
| 1234-1266 | `w32_compute_glyph_string_overhangs` | file gate |
| 1268-1292 | `w32_clear_glyph_string_rect` (C 1274-1287's `#if 0`) | file gate; the `#if 0` dead text as a comment |
| 1294-1321 | `w32_fill_stipple_pattern` | file gate |
| 1323-1363 | `w32_draw_glyph_string_background` | file gate |
| 1366-1421 | `w32_draw_glyph_string_foreground` | file gate |
| 1423-1515 | `w32_draw_composite_glyph_string_foreground` | file gate |
| 1518-1601 | `w32_draw_glyphless_glyph_string_foreground` | file gate |
| 1603-1615 | `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` (C 1604-1614's comment + the `#define`) | file gate |
| 1618-1680 | `w32_alloc_lighter_color` | file gate |
| 1682-1698 | `w32_query_colors` | file gate |
| 1700-1707 | `w32_query_frame_background_color` | file gate |
| 1709-1747 | `w32_setup_relief_color` (C 1719-1721 and 1731-1734's two `#if 0` regions) | file gate; both `#if 0` as comments |
| 1750-1776 | `w32_setup_relief_colors` | file gate |
| 1779-1841 | `w32_draw_relief_rect` | file gate |
| 1844-1881 | `w32_draw_box_rect` | file gate |
| 1884-1949 | `w32_draw_glyph_string_box` (slice 2's last item) | file gate |
| 1950-1955 | `w32_image_rotations_p` (C 1951-1955; C 1950 is the blank line under slice 2's `}`) | file gate (non-`static`, so it carries `#[unsafe(no_mangle)]`) |
| 1956-1969 | `transform` (C 1962's documentation URL) | file gate |
| 1970-2190 | `w32_draw_image_foreground` (C 1972-1973's comment; C 2114-2148's `#ifdef WINDOWSNT`) | file gate; the `#ifdef` both arms, C's own text in a comment |
| 2191-2201 | `w32_image_size` (C 2192-2201) | file gate (non-`static`) |
| 2202-2293 | `w32_draw_image_relief` (C 2204-2206's comment) | file gate |
| 2294-2383 | `w32_draw_image_foreground_1` (C 2296-2298's comment) | file gate |
| 2384-2399 | `w32_draw_glyph_string_bg_rect` (C 2386-2389's comment) | file gate |
| 2400-2532 | `w32_draw_image_glyph_string` (C 2402-2414's comment and picture; C 2455-2494's `#if 0`) | file gate; the `#if 0` dead text as C's own text in a comment |
| 2533-2631 | `w32_draw_stretch_glyph_string` (C 2535's comment) | file gate |
| 2632-2674 | `w32_draw_dash` (C 2633-2638's comment) | file gate |
| 2675-2715 | `w32_fill_underline` (C 2676-2680's comment; **slice 3's last item**) | file gate (C's `switch` is the port's `match`; `docs/ports/macro-body-notes.tsv`) |
| 2716-3038 | `w32_draw_glyph_string` (C 2719-3038) - the file's glyph painter: the successor-first background pass (C 2724-2744), C 2747's GC/clipping setup, C 2749-2772's relief-or-clip choice, C 2774-2811's `switch (s->first_glyph->type)`, and C 2813-3037's `if (!s->for_overlaps)` block with the relief, underline (C 2831-2936, including C 2856-2873's two `WINDOW_BUFFER_LOCAL_VALUE` reads), overline, strike-through and the two overhang-repair loops | file gate (`have_ntgui`); slice 4 below |
| 3039-3235 | the five small frame entries and the scroller: `w32_shift_glyphs_for_insert` (C 3041-3054), `w32_delete_glyphs` (C 3057-3067), `w32_clear_frame` (C 3070-3091), `w32_ring_bell` (C 3094-3117), `w32_ins_del_lines` (C 3119-3133, with C 3119-3121's "Line Dance" banner) and `w32_scroll_run` (C 3136-3235) | file gate (`have_ntgui`); slice 5 below |
| 3237-3431 | the Exposure Events section's focus machinery and the two keyboard entries: `w32_frame_highlight` (C 3243-3248), `w32_frame_unhighlight` (C 3250-3255), `w32_new_focus_frame` (C 3257-3284), `w32_focus_changed` (C 3287-3324), `w32_detect_focus_change` (C 3327-3347), C 3350-3358's `#if 0` (`w32_mouse_leave`, carried as C's text), `w32_frame_rehighlight` (C 3360-3374), `w32_frame_rehighlight_1` (C 3376-3403), `get_keysym_name` (C 3405-3420) and `codepage_for_locale` (C 3422-3431) | file gate (`have_ntgui`); slice 6 below |
| 3433-3629 | the mouse section's head: the section comment (C 3433-3434), `parse_button` (C 3439-3499, the mouse-message `switch`), `w32_construct_mouse_click` (C 3501-3532) and `w32_construct_mouse_wheel` (C 3534-3629, with the high-precision/coalescing arms) | file gate (`have_ntgui`); slice 7 below |
| 3631-3763 | the `#if HAVE_W32NOTIFY` block: `w32_lispy_file_action` (C 3636-3670) and the `#ifdef WINDOWSNT` `w32_queue_notifications` (C 3676-3761) | `all (have_ntgui, have_w32notify)`; slice 8 below |
| 3765-3970 | `w32_note_mouse_movement` (C 3774-3819), the "Mouse Face" banner with C 3826-3834's three dropped forward declarations, `w32_define_cursor` (C 3835-3839) and `w32_mouse_position` (C 3861-3970) | file gate (`have_ntgui`); slice 9 below |
| 3973-4157 | the Tab-bars and Tool-bars click handlers (`w32_handle_tab_bar_click` C 3977-3992, `w32_handle_tool_bar_click` C 4000-4015), the "Scroll bars" banner with C 4023-4027's two comments, `w32_window_to_scroll_bar` (C 4029-4056), `w32_set_scroll_bar_thumb` (C 4059-4132) and `w32_set_horizontal_scroll_bar_thumb` (C 4134-4157) | file gate (`have_ntgui`); slice 10 below |
| 4160-4328 | the "Scroll bars, general" banner and the eight `my_*` wrappers (`my_create_vscrollbar` C 4164-4170, `my_create_hscrollbar` C 4172-4178, `my_show_window` C 4182-4191, `my_set_window_pos` C 4193-4210, C 4212-4219's `#if 0`, `my_set_foreground_window` C 4221-4226, `my_destroy_window` C 4229-4234, `my_bring_window_to_top` C 4236-4241), plus `w32_scroll_bar_create` (C 4243-4305) and `w32_scroll_bar_remove` (C 4308-4328) | file gate (`have_ntgui`); slice 11 below |
| 4330-4536 | the two scroll-bar setters: `w32_set_vertical_scroll_bar` (C 4330-4432) and `w32_set_horizontal_scroll_bar` (C 4434-4536), each with its create branch and its move branch | file gate (`have_ntgui`); slice 12 below |
| 4538-4684 | the three scroll-bar hooks: C 4539-4545's comment, `w32_condemn_scroll_bars` (C 4551-4571), `w32_redeem_scroll_bar` (C 4577-4656, with C 4599's `goto horizontal`) and `w32_judge_scroll_bars` (C 4661-4684) | file gate (`have_ntgui`); slice 13 below |
| 4686-4915 | the two scroll-bar click handlers: `w32_scroll_bar_handle_click` (C 4686-4800) and `w32_horizontal_scroll_bar_handle_click` (C 4802-4915), each with its `SCROLLINFO` probe and its `SB_*` switch | file gate (`have_ntgui`); slice 14 below |
| 4917-5128 | the two scroll-bar motion reports and the clearer: `w32_scroll_bar_report_motion` (C 4917-4965, the vertical report with its `SB_THUMBTRACK`/`SB_LINEDOWN` arms), `w32_horizontal_scroll_bar_report_motion` (C 4967-5016, the horizontal twin with `SB_LINERIGHT`) and `w32_scroll_bar_clear` (C 5019-5061), then the socket reader's five `static` objects (C 5063-5076), `mouse_or_wdesc_frame` (C 5078-5107) and C 5128's `menubar_selection_callback` declaration | file gate (`have_ntgui`); slice 15 below |
| 5130-6458 | `w32_read_socket` (C 5130-6458): the socket reader's locals, the `while get_next_msg` body with its twenty-five-arm `switch (msg.msg.message)` (C 5169 `WM_EMACS_PAINT` ... C 6344's `default:`), the `#if HAVE_W32NOTIFY` arm, the `WM_TOUCHMOVE`/`WM_TOUCH` pair with its `goto touch_located` and touch-point loop, the autoraise block and the `FOR_EACH_FRAME` visibility sweep | file gate (`have_ntgui`), with the one `have_w32notify` arm; slice 16 below |
| 6460-6752 | the Text Cursor group: `w32_clip_to_row` (C 6473-6489), `w32_draw_hollow_cursor` (C 6494-6533), `w32_draw_bar_cursor` (C 6543-6626), `w32_define_frame_cursor` (C 6631-6635), `w32_clear_frame_area` (C 6640-6648) and `w32_draw_window_cursor` (C 6650-6752) | file gate (`have_ntgui`); slice 17 below |
| 6754-6906 | `w32_bitmap_icon` (C 6758-6816) and `w32_new_font` (C 6846-6891), with C 6818-6841's "Handling X errors" and C 6894-6906's "TODO: W32 Input Methods" banner-plus-list blocks carried as C's own text | file gate (`have_ntgui`); slice 18 below |
| 6908-7146 | `w32_calc_absolute_position` (C 6912-7021), `w32_set_offset` (C 7029-7065) and `w32fullscreen_hook` (C 7067-7146) | file gate (`have_ntgui`); slice 19 below |
| 7148-7502 | `w32_set_window_size` (C 7152-7256), `w32_set_window_size_and_position` (C 7261-7335), `frame_set_mouse_pixel_position` (C 7339-7365), `w32_get_focus_frame` (C 7367-7379), `w32_focus_frame` (C 7388-7405), `w32_raise_frame` (C 7408-7467), `w32_lower_frame` (C 7470-7479) and `w32_frame_raise_lower` (C 7481-7491) | file gate (`have_ntgui`); slice 20 below |
| 7503-7734 | `w32_make_frame_visible` (C 7503-7599), `w32_make_frame_invisible` (C 7605-7625), `w32_make_frame_visible_invisible` (C 7627-7634), `w32_iconify_frame` (C 7638-7660), `w32_free_frame_resources` (C 7665-7699), `w32_destroy_window` (C 7702-7710) and `w32_wm_set_size_hint` (C 7720-7734) | file gate (`have_ntgui`); slice 21 below |
| 7736-8414 | **the remainder of the file** (C 2720 `w32_draw_glyph_string` onward: C 3044 `w32_shift_glyphs_for_insert`, C 3061 `w32_delete_glyphs`, C 3073 `w32_clear_frame`, C 3097 `w32_ring_bell`, C 3127 `w32_ins_del_lines`, C 3139 `w32_scroll_run`; Line Dance at 3119, Exposure Events at 3239, Mouse Face at 3822, Tab-bars 3973, Tool-bars 3996, Scroll bars 4019/4160, `w32_read_socket` at 5131, Text Cursor 6462, X errors 6819, Input Methods 6894, Fonts 7737, Image Hooks 7809, `w32_initialize` 8155, `syms_of_w32term` 8273) | not in this slice: recorded here so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |

## Evidence

**1. The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32termport/cargo cargo run -q -p xtask -- build-tu src/w32term.c`
-> `built src/w32term.rs -> ../build/femacs/w32term.o`, **exit 0**, and `build/femacs/w32term.o` is
**336 bytes** - an empty object.  That is the expected reading, not a proof of the new text: every
item sits behind `have_ntgui`, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute.  (The command was run five times over the
batch: twice while the text was being written, and three times after the three rounds of fixes
that changed it - a chained-assignment pass, a cast pass and the body-token pass - each of them
`exit 0` and 336 bytes.  Only the last one is the final revision's reading; the direct-rustc
`frozen` row of the sweep below is the same compile on the final revision, also exit 0.)

**2. The multi-configuration sweep (a *free* reading: direct `rustc` on the materialized root).**
`xtask probe` materializes the unit (its own text plus every mirror it reaches, flattened into
`build/probes/w32term/*.rs`); a copy of that directory in `/tmp` is compiled once per
configuration with `rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj
-C panic=abort`, and the errors are counted by primary `-->` anchor.

| configuration | exit | `error` lines | anchor sites | anchored in the unit |
| --- | --- | --- | --- | --- |
| frozen (no `--cfg`) | 0 | 0 | 0 | 0 (the 336-byte object) |
| `have_ntgui` | 1 | 49 | 61 | 42 |
| + `windowsnt` | 1 | 28 | 27 | 0 |
| + `have_window_system` + `dispextern_face_records` ("base") | 1 | 28 | 27 | 0 |
| base + `glyph_debug` | 1 | 28 | 27 | 0 |
| base + `cygwin` | 1 | 29 | 28 | 0 |
| base + `mingw_w64` | 1 | 28 | 27 | 0 |
| base + `win32_winnt_lt_0500` | 1 | 28 | 27 | 0 |
| base + `win32_winnt_lt_0601` | 1 | 28 | 27 | 0 |
| base + both `_WIN32_WINNT` features | 1 | 28 | 27 | 0 |
| base + `cygwin` + `mingw_w64` | 1 | 29 | 28 | 0 |

`have_ntgui` alone is the arm where the *mirror type graph* is missing (the `w32term.h` mirror's
whole body is behind `windowsnt`): 42 of the 61 anchors are in the unit's own text and every one
of them is a name that arm cannot reach (`w32_display_info`, `w32_output`, `w32_output`'s
`paint_buffer_dirty`, `face`, `glyph_string`, `enter_crit`/`leave_crit`, `FRAME_*` …) - which is
the expected shape, not the unit's defect.  The `cygwin` rows carry exactly one more error than
the others: C 214-216's `w32_message_fd` is this unit's sixteenth colliding object.

**The sweep's own limit, measured.**  From `+ windowsnt` on, **`anchored in the unit` is 0 because
rustc aborts at name resolution**: the mirrors' E0428/E0425 rows (below) stop the crate before
type checking, so "0 errors in the unit" would be an *empty* reading here (P-86 ⑦; the
`androidmenu.c` lesson, `docs/status.md` 3.370).  Evidence 3 is what turns that into a reading.

**3. The arm-reach pre-flight and the mutation tests (P-86 ⑦) - a *different* reading.**
The materialization's own copies (the unit's text line for line; the only differences are xtask's
`include!` path rewrites) with nine patch classes and nothing else, each conditional because
`femacs/src/include/**` is owned by another lane and moved during this batch:

| # | patch | applied |
| --- | --- | --- |
| 1 | `include/w32term.rs`: the 23 declarations this unit defines, commented out | yes (23) |
| 2 | `include/w32term.rs`: `FRAME_DISPLAY_INFO`'s `(void) ($f)` body | **no - the mirror lane fixed it while this batch ran** |
| 3 | `include/w32term.rs`: stand-ins for `PALETTERGB` and `EnterCriticalSection`/`LeaveCriticalSection` | yes |
| 4 | `include/w32term.rs`: `w32_output`'s two recorded bitfields (`paint_buffer_dirty`, `want_paint_buffer`) | yes |
| 5 | `include/dispextern.rs`: `draw_fringe_bitmap_params`' two bitfields (`cursor_p`, `overlay_p`) | yes |
| 6 | `include/frame.rs`: `FRAME_W32_P`'s frozen-only arm | yes |
| 7 | `include/frame.rs`: `FRAME_MOUSE_UPDATE`'s `note_mouse_highlight` stand-in | yes |
| 8 | `include/frame.rs`: the `have_ntgui` flag-run `compile_error!` and a `FRAME_GARBAGED_SHIFT` stand-in | yes |
| 9 | `include/w32.rs`: its collisions with the lib mirrors (`FD_SETSIZE`, `fd_set`) | yes (2) |

```
rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg 'feature="have_window_system"' \
  --cfg 'feature="dispextern_face_records"' /tmp/w32termport/arm/_root.rs -o /tmp/w32termport/arm.o
-> exit 1, 142 error blocks, 141 of them anchored in a mirror, **1 anchored in the unit**:
   `root.rs:1494 cannot find function w32_clip_to_row` - whose definition is C 1739's, in the
   *next* slice, and whose C 244 prototype this slice drops with the rest of the block
   (PORTING.md 0.1-12).  That is the whole unit-text reading, and it is named rather than papered
   over with a stub.
```

**The two mutation tests (the same harness).**
* a **type** mutant - `let _mutant: c_int = c"mutant".as_ptr ();` inserted into
  `w32_define_fringe_bitmap`'s body - leaves the output **byte-identical** (exit 1, 142 blocks,
  15 unit anchors, the same set): the unit's own *bodies* are never type-checked, because the
  mirrors' resolution errors stop the crate first.  So "the unit has no type errors" is **not
  claimed**;
* a **resolution** mutant - `let _mutant = w32term_arm_probe_no_such_name;` at the same place -
  adds exactly one error, `root.rs:1591 cannot find value w32term_arm_probe_no_such_name`, so the
  harness *does* reach the unit's own text at name resolution, which is what Evidence 3's "1
  unit-anchored error" measures.  The harness and both mutants were reverted (the `/tmp` copies
  are back to the pristine materialization plus the nine patches above).

**4. The inventory gates (repo root, final revision).**
```
python3 tools/check-dead-file-coverage.py
  -> info docs/ports/w32term.md: 59 range row(s) cover src/w32term.c (8414 lines; 0 blank
     line(s) between blocks need no row)
     info docs/ports/w32term.md: 59 range row(s) cover src/w32term.h (929 lines; 0 blank line(s)
     between blocks need no row)
     15 ledger(s) bound (a C range table), 0 failure(s)
python3 tools/check-port-lines.py
  -> stale src/w32term.c: no longer under the threshold
     ... 86 file(s) under ten percent, 195,581 reference line(s); 87 ratchet row(s), 0 new, 1 stale
python3 tools/check-citations.py
  -> 675 file(s) scanned, 0 drift, 0 range, 494 external, 33 allowed, 0 stale
python3 tools/check-declarations.py
  -> 13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
python3 tools/check-carried.py
  -> 11 carried file(s), 11 baseline row(s), 0 failure(s)
python3 tools/check-cond-eval.py src/w32term.c
  -> src/w32term.c: 41 conditions, 2 unknown, 0 disagreements [VERIFIED]
python3 tools/extract-abi.py --check emacs/src/w32term.c femacs/src/w32term.rs | tail -3
  -> - line count drifted 81.1%: C 8414 vs Rust 1592
     FAIL emacs/src/w32term.c -> femacs/src/w32term.rs
```
`--check` is **not vacuous** for this file (P-86 ⑩): its reading is **134 `- ` rows**, and
**133 of them are the C 1003-8414 remainder** (124 missing functions, the remainder's two macros
`LOAD_PROC`/`HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` and its one constant, six file-scope objects),
with the line-count row as the 134th.  **No item this slice defines is missing, invented or
token-short** - the four "does not mention TOKEN" rows the first draft had were fixed inside the
batch rather than recorded: `double` in `w32_set_frame_alpha` is now `c_double`, the two bitfield
reads are the member-named macros (`garbaged!`, `window_bits! (pseudo_window_p, …)`), and C 970's
`f->output_data.w32->cursor_pixel` keeps C's own field path (the `// C:` lines above name each).
The line-count row is the slice, not an expansion: the ported prefix's own density is
1592/1002 = 1.59×.

**5. Slice 2's own readings (C 1004-1949) - the same three levers, plus one arm-reach reading.**
```
cd femacs && CARGO_TARGET_DIR=/tmp/w32term2-lane/cargo cargo run -q -p xtask -- build-tu src/w32term.c
  -> `built src/w32term.rs -> ../build/femacs/w32term.o`, **exit 0**, `build/femacs/w32term.o` is
     **336 bytes** - the same empty object as Evidence 1, because every item of this slice sits
     behind `have_ntgui` too.  Run three times over the batch (twice while the text was being
     written and once after the fixes below); only the last is the final revision's reading.
python3 tools/extract-abi.py --check emacs/src/w32term.c femacs/src/w32term.rs
  -> **110 `- ` rows, 0 of them a "does not mention" row**: 102 missing functions, one missing
     macro (`LOAD_PROC`, C 8244), six missing file-scope objects and the line-count row
     (`line count drifted 65.0%: C 8414 vs Rust 2941`).  Slice 1's reading was **134** rows, so
     this slice's 22 functions and `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT` account for the 24 that are
     gone, and **no item this slice defines is missing, invented or token-short** - the one
     problem row the first draft had (`w32_set_glyph_string_clipping_exactly does not mention 0`,
     C 1226's `dst->clip[0]`) was fixed inside the batch by writing that subscript as the port's
     `.add (0)` rather than recorded.
python3 tools/check-dead-file-coverage.py
  -> `info docs/ports/w32term.md: 84 range row(s) cover src/w32term.c (8414 lines; 36 blank
     line(s) between blocks need no row)` / the same for `src/w32term.h` (929 lines) /
     `20 ledger(s) bound (a C range table), 0 failure(s)`
python3 tools/check-citations.py
  -> 675 file(s) scanned, **0 drift, 0 range, 0 stale** (the new `/// C:` lines of this slice's
     declarations and its 29 item comments resolve)
python3 tools/check-declarations.py
  -> `14 declaration sites (0 forbidden, 14 allowed literals), 0 failures` on the final
     revision.  The mid-batch run read `1 failures`, and it was **not this file**:
     `femacs/src/android.rs`'s `PACKAGE_STRING` literal, in flight in another lane while this
     batch ran (this unit adds no declaration site of its own)
```

**The arm-reach reading (P-86 ⑦) - what a pristine `have_ntgui` + `windowsnt` compile of this
unit's text says, and what it found.**  `xtask probe src/w32term.rs <out>` materializes the unit
(its own text plus every mirror it reaches, flattened into `build/probes/w32term/`, 56 files); a
copy of that directory in `/tmp` is compiled with
`rustup run nightly-2026-10-01 rustc --edition 2024 --crate-type lib --emit obj -C panic=abort`
and the arm set `have_ntgui`, `windowsnt`, `have_window_system`, `dispextern_face_records` (slice
1's "base" label).  **No patch:** the nine stand-ins of Evidence 3 are *not* applied, so slice 1's
own recorded gaps show up as such, which is the point of this run.

| run | exit | `error` lines | anchored in the unit | of them in slice 2's lines |
| --- | --- | --- | --- | --- |
| frozen (no `--cfg`) | 0 | 0 | 0 | 0 |
| base set, **first** run | 1 | 70 | 44 | **26** |
| base set, after this batch's fixes | 1 | 42 | 16 | **0** |

The **first** run's 26 slice-2-anchored errors were all one class the frozen lever cannot reach -
**Rust does not auto-deref a raw pointer in field access**, so C's
`s->f->output_data.w32->cursor_gc->foreground` chain needs one `(*…)` per hop and the cast needs
its own parentheses (`(*( (*(*s).f).output_data.w32 as *mut w32_output )).cursor_pixel`, measured
by a two-line `rustc` probe): nine sites in `w32_set_cursor_gc`'s four reads,
`w32_draw_relief_rect`'s four, `w32_set_mode_line_face_gc`, `w32_fill_stipple_pattern` and
`w32_clear_glyph_string_rect` (`E0606`/`E0609`); the four `glyph_u_cmp_bits!` expansions passed a
`glyph` value where the macro dereferences (`E0614`); `RGB!`'s body mixed `u8 | u16` where C
promotes both to `int` (`E0277`, four expansions); and `composition_gstring_width`'s two
`ptrdiff_t` parameters were being handed `c_int`s (`E0308`).  All were fixed inside the batch, and
the same defect in **slice 1's** C 970 line (C's `f->output_data.w32->cursor_pixel`) was fixed
with them - its two errors (`E0606` on the cast, `E0609` on the field) are the only lines of that
slice this batch touched, and `## Deviations` records the change.

The **after** run's 16 unit-anchored errors are **all slice 1's**, and every one is a gap this
ledger already carries: four `w32_output.paint_buffer_dirty` and one `want_paint_buffer` (owed 6),
one `cursor_p` and two `overlay_p` on `draw_fringe_bitmap_params` (owed 5), the `<windows.h>`
`OSVERSIONINFO` typedef (the file note's acquisition gap) and seven `E0308`s in
`w32_fill_area_abs`'s `c_ulong`-into-`COLORREF` calls (owed 8).  The other 26 belong to the
mirrors: 24 frozen-only `E0080` layout assertions (`face`/`font`/`font_driver` sizes and offsets,
`redisplay_interface` - they are asserted for the frozen arm and this arm moves the guarded
members), `include/frame.rs` 329's `have_ntgui` `compile_error!` (owed 9),
`include/w32common.rs` 111's `SYSTEM_INFO` (the same `<windows.h>` gap) and **one new mirror row
this slice measures**: `include/w32term.rs` 340 declares `w32_query_frame_background_color`, and
the unit's definition of it is `E0428` until that mirror gets owed 4's `*_decls` module treatment
(now named in owed 4 and owed 15).

**The reading's own limits.**  This is not a Windows compile: there is no `windows.h`, no mingw
toolchain and no layout measurement (the file note above), the `have_ntgui`-alone arm still stops
at name resolution in the mirror graph (110 unit-anchored errors, exactly Evidence 2's class), and
"0 in slice 2's lines" is a *name-resolution and type-check* reading taken in an arm where 42
errors remain elsewhere - it is not "the unit compiles", and no behaviour was executed.

**6. Slice 3's own readings (C 1950-2715) - the same levers, plus the free sweep.**

`cd femacs && CARGO_TARGET_DIR=/tmp/w32term-lane/target cargo run -q -p xtask -- build-tu
src/w32term.c` -> `built src/w32term.rs -> ../build/femacs/w32term.o`, **exit 0**, and
`build/femacs/w32term.o` is **336 bytes** - Evidence 1's empty object.  Every item this slice adds
carries the file gate, so the frozen configuration still compiles only `include/config.rs` and the
`#![allow]` attribute.  Three runs over the batch: the baseline before the text was written, one
mid-batch run that stopped on the first draft's four errors (below), and the final one - the last
two are the reading; only the last is the final revision's.

**The free sweep** (a *free* reading: direct `rustc` on a copy of `build/femacs/units/w32term` in
`/tmp`, the same materialized root `build-tu` compiles, under four `--cfg` sets):

| configuration | errors | where |
| --- | --- | --- |
| none (the frozen arm) | 0 | - |
| `windowsnt` | 0 | - |
| `have_ntgui` + `windowsnt` | the mirror's own refusal | `include/frame.rs` 329's `compile_error!`, the two `E0428`s of `include/w32term.rs` 338/1173 and `glyph_string_bits!`'s `E0425` (`have_window_system` is off in this set) |
| `have_ntgui` + `windowsnt` + `have_window_system` + `dispextern_face_records` | 19 | 9 mirror-anchored, 10 in **slice 1's** lines, **0 in this slice's** |

The 19: the two `E0428`s of `include/w32term.rs` 338 (`w32_query_frame_background_color`, owed 15)
and 1173 (`w32_image_rotations_p`, **new - this slice's definition**, owed 21), `include/frame.rs`
329's `compile_error!`, `CRITICAL_SECTION` (`include/w32term.rs` 1585), `LPVOID`
(`include/w32common.rs` 126/127), `LOGFONT` (`include/w32font.rs` 177), `c_variadic`
(`include/lisp.rs` 3763) and one `E0308` (`include/w32term.rs` 632) - the mirror rows owed 12
already names; then the seven `w32_fill_area_abs` `COLORREF` conversions of owed 8 and the three
`draw_fringe_bitmap_params` reads of owed 5, both in slice 1's lines.  **"0 in this slice's lines"
is a name-resolution and type-check reading**: the arm is not a working Windows configuration, no
behaviour was executed, and the two arms above it are refused, so nothing here says the unit
compiles end to end.

**Two corrections inside earlier slices' lines, measured the same way.**  Before them this sweep
read 39 errors, five of them **slice 2's**: `w32_compute_glyph_string_overhangs`' two
`(*s).cmp.rbearing`/`.pixel_width`/`.lbearing` reads (C 1241-1242) and
`w32_draw_composite_glyph_string_foreground`'s two `(*s).cmp.offsets` reads and its
`COMPOSITION_GLYPH ((*s).cmp, …)` argument (C 1462-1464) were `E0609`/`E0308`, because
`glyph_string::cmp` is the mirror's untyped `*mut c_void` and `include/composite.rs` carries the
record.  All five now cast `as *mut composition` (the `w32_draw_composite_glyph_string_foreground`
shape) and read 0.  The first draft's four errors were this slice's own and were fixed inside the
batch: `E0585` (a `/// C:` line after a struct member), `E0244`-class `box` (a Rust keyword, so
`face_bits! (box, …)`), `E0308` on `GetObject`'s `LPVOID` argument, `E0308` on C's `corner` array
decay and `E0618` on the four GDI+ macro calls (they yield the loaded `Option`).

**The official probes.**  `... probe src/w32term.rs /tmp/w32term-lane/x-windowsnt.rlib --crate-type=lib
-C panic=abort --cfg 'feature="windowsnt"'` -> **exit 0, no output, a 22448-byte rlib**: the
mirror graph and this unit's `windowsnt`-gated declarations compile in the single-gate arm (3.575's
claim, re-read here).  The same command with `--cfg 'feature="have_ntgui"'` added -> **exit 1, 33
errors**, the mirror's refusal this batch does not touch: `include/frame.rs` 329's `compile_error!`
(`HAVE_NTGUI` is not the arm the flag run was measured on), the two `E0428`s above, the four
`E0425`s of owed 22 (`CRITICAL_SECTION`, `LPVOID` twice, `LOGFONT`) and **24 `E0425`s for
`glyph_string_bits!`** - `include/dispextern.rs` 1835 gates that macro on `have_window_system`,
which C's `HAVE_NTGUI` implies (config.h) but this hand-set `--cfg` pair does not set.  None of the
33 is this slice's text and none is fixable in this unit; the substantive reading of the GDI+ and
drawing code is the free sweep's fourth row above.

**The `extract-abi --check` readings (before -> after): 110 -> 99 findings.**  The eleven retired
rows are exactly this slice's eleven definitions (`transform`, `w32_draw_dash`,
`w32_draw_glyph_string_bg_rect`, `w32_draw_image_foreground`, `w32_draw_image_foreground_1`,
`w32_draw_image_glyph_string`, `w32_draw_image_relief`, `w32_draw_stretch_glyph_string`,
`w32_fill_underline`, `w32_image_rotations_p`, `w32_image_size`); the line-count row moved from
`65.0%: C 8414 vs Rust 2941` to `51.0%: C 8414 vs Rust 4123`; and one **new** row this slice
measures, `function w32_fill_underline does not mention break, case, default, fallthrough,
switch`, is answered by the `docs/ports/macro-body-notes.tsv` row this batch appends (the
`tparam1`/`Fvariable_binding_locus` precedents).

## Deviations

- **The `<windows.h>` section is spelled, not measured.**  C reaches `<windows.h>` through C 25's
  chain (`w32term.h` 22 → `w32gui.h` 21), which has no file in the reference tree, so the port
  declares the names this slice uses at that section (the `w32fns.rs` reading of the same header:
  P-88 ①'s declaration site is the chain's include line).  Every scalar is PORTING.md 1.1's width
  (`DWORD`/`LONG` are 32-bit on Windows whatever this target's `c_long` is), every `#[repr(C)]`
  record (`POINT`, `RECT`, `_TOUCHINPUT`, `W32Msg`'s two) is **UNVERIFIED** here, and the layout
  assertions are the integrator's (owed 9).
- **Six casts are C's implicit conversions written out**, each measured as `E0308`/`E0277` in the
  pre-flight: the two `Emacs_GC` members (`c_ulong` → `COLORREF`) at C 525-526 and 1041-1042, the
  two `Emacs_Rectangle` members (`c_int` → `c_uint`) at C 473-476, `floor(…)` (`f64` → `c_int`) at
  C 433/435, `XFIXNUM (…) as c_double` at C 609, `alpha * OPAQUE_FRAME as c_double` at C 607, and
  the three `face->foreground`/`FRAME_FOREGROUND_PIXEL` sites C 706-708 (`c_ulong` → `COLORREF`).
  C's own `(short)` in `GET_WHEEL_DELTA_WPARAM` is `as c_short`.
- **C's chained assignments are one statement per assignment** in C's right-to-left order
  (PORTING.md 0.3): C 428's `*scale_x = *scale_y = 1;` and C 494's `y1 = y2 = y0;`.
- **C's comma expressions are the statements C's effects require**, in C's order and behind C's own
  short-circuit tests: C 506's `x1 = x2, y1 = y2` and C 507's `x2 += dx, y2 = y0 + odd*dy` (inside
  the `while`), and C 858-859/868-869's two in `w32_after_update_window_line`'s `if` chain (the
  `haikuterm.rs` 888 reading).
- **C's uninitialized locals are zeroed and named** (PORTING.md 1.5): the twelve `int`s and two
  records of `w32_draw_underwave` (C 459-462), `rect` in `w32_clear_window` (C 558) and
  `w32_draw_vertical_window_border` (C 694), `scroll_lines` (C 383), `opac`/`ex_style` (C 578-579).
  C's tentative definitions are zeroed too (`one_w32_display_info`, the seven carets, the four
  thread objects, the six scroll-bar ints, `any_help_event_p`, `w32_wheel_scroll_lines`).
- **`odd = !odd` on an `int`** (C 508) is `odd = (odd == 0) as c_int;` - C's `!` yields 0/1 and
  the port writes the comparison out.
- **`#if 0` text is carried as C's own text inside a comment**, in t
  `cm.rs` 86 reading (C 64-66, 134-139, 248-250, 264-289, 312-319).
- **C 129-156's `_WIN32_WINNT < 0x0601` is treated as an SDK guard, not as a `cfg` arm.**  The
  block's own comment says "Define required types and constants on systems with older headers lest
  they be absent", i.e. its other arm is `<winuser.h>`'s declarations, which this target has not
  got - the same shape as the five `#ifndef` guards in this slice and the nine in
  `include/w32term.rs`.  Carrying the text under `win32_winnt_lt_0601` instead would have left
  C 159-160's unguarded `PTOUCHINPUT` parameter with no declaration at all.  Both arms are
  recorded; the choice is this ledger's, and it is the one place where the "value test ⇒ `cfg`"
  reading (`lib/include/boot-time-aux.rs` 395's `win32_vista`) is deliberately not followed.
- **Four bitfield members are read through the mirrors' own accessors, three through names that
  have none yet**: `w->pseudo_window_p`, `desired_row->mode_line_p`/`full_width_p`/
  `redraw_fringe_bitmaps_p` and `hlinfo->mouse_face_defer` are `glyph_row_bits!`
  (`include/dispextern.rs`), `window_bits!`/`garbaged!` (declared in this unit at their use sites,
  the `dispnew.rs` 396/457 precedent, because those two macros live in units this file does not
  include) and the record's own `bits`/`MOUSE_FACE_DEFER` mask (`include/dispextern.rs` 1945);
  C's `output->paint_buffer_dirty`/`want_paint_buffer` (C 329/346/814/826) and
  `p->overlay_p`/`p->cursor_p` (C 915/945/970) are written **C's way**, because the two records'
  mirrors carry them only as a recorded storage unit with no accessor - their fix is owed (owed 4
  and 5), and the pre-flight's patches are the stand-ins.
- **`FACE_FROM_ID_OR_NULL` and `FRAME_FACE_CACHE` are declared in this unit** (C 868's call), the
  `xdisp.rs` 963-1010 / `alloc.rs` 3456-3463 reading; the record's own `face_cache` is *not*
  repeated - the mirror's top-level copy is the one this unit reads.
- **`draw_fringe_bitmap_params` is named by the module path**, `dispextern_unit_names::…`: the
  mirror carries the five records of C 2159-3200 inside `pub mod dispextern_unit_names`, and
  `abi_shim.rs` 2047-2050 measures that the unqualified name is `E0425` in the flip arm.  The unit
  is no C record's owner, so it does not repeat the record.
- **The frozen forward-declaration block (C 218-257) is dropped**, each line recorded where its
  definition lands (PORTING.md 0.1-12); the one boundary it creates is named: C 938 calls
  `w32_clip_to_row`, whose definition is C 1739's - the next slice's.
- **The C 20-66 include graph is gated as C's own build has it** (every header behind
  `have_ntgui`), with the two arms of C 34/51/55 carrying their own features; the headers with no
  file in the reference tree (`<wtypes.h>`, `<gdiplus.h>`, `<imm.h>`, `<io.h>`, `<shellapi.h>`)
  are recorded as `// C …` lines, since this slice names nothing from them.
- **Line count.**  C 8,414 (this slice: 1,002) vs Rust 1,592 - a `docs/ports/line-drift-tolerance.tsv`
  row (stem `w32term`, basis `total`) is owed, derived from the finished prefix the way that
  table's own note describes for a half-ported unit.

### Slice 2 (C 1004-1949)

- **The `<windows.h>` / `<wingdi.h>` names this slice adds are spelled, not measured** - the same
  reading, the same site (C 25's chain) as slice 1's section, and the same **UNVERIFIED** status:
  `pub type WORD`, `pub type HFONT = HGDIOBJ`, the `RGB`/`PALETTERGB`/`GetRValue`/`GetGValue`/
  `GetBValue` macros (`<wingdi.h>`'s own bodies, with C's integer promotions written out:
  `((r)as BYTE | (g)as BYTE as WORD << 8)` promotes to `int` in C, so the port's arms are all
  `DWORD`), `CreatePatternBrush`, `SetTextAlign` and `TA_LEFT`/`TA_BASELINE`.  `FillRect` is
  **not** re-declared: slice 1's section already carries it (its C 551 site) and a second
  declaration measured `E0428` in the arm-reach run.
- **The C-side names this slice declares itself** (P-72 rule 3 / P-88 clause 1, each at its
  use site, each recorded in `## Owed`): `min`/`max` (lisp.h 71-73, which `include/lisp.rs` does
  not carry; the mirror that does, `lib/include/min-max.rs`, is not on this unit's graph),
  `window_box_right`, `get_glyph_string_clip_rects` (with `nr` typed `*mut RECT`, `w32gui.h` 94's
  `#define NativeRectangle RECT`), `prepare_face_for_display`, `image_background`/
  `image_background_transparent`, the `Emacs_Pix_Context`/`RGB_PIXEL_COLOR` pair for the
  `HAVE_NTGUI` arm, the two `IMAGE_BACKGROUND`/`IMAGE_BACKGROUND_TRANSPARENT` macros, `struct
  face`'s three bitfield accessors (`box`, `use_box_color_for_shadows_p`, `overstrike` - the
  `face_bits!`/`FACE_*` names `src/xfaces.rs` 351-435 measures, repeated here) and the glyph
  union's two (`glyph_u_cmp_bits!`, `glyphless_bits!`).  `w32_check_font` (C 7747's definition) is
  **named but not declared**: its C 256 prototype is dropped with the C 218-257 block, so the two
  `IF_DEBUG (w32_check_font (…))` sites reach a name no slice has defined yet - the second of
  `w32_clip_to_row`'s class.
- **Rust does not auto-deref a raw pointer in field access.**  Every hop of C's
  `s->f->output_data.w32->cursor_gc->foreground` is written `(*…)` and the `*mut w32_output` cast
  gets its own parentheses: `(*( (*(*s).f).output_data.w32 as *mut w32_output )).cursor_pixel`,
  `(*(*( … )).cursor_gc).foreground`.  The arm-reach run measured 26 errors of this class in the
  first draft (nine sites plus the four `glyph_u_cmp_bits!` expansions, the `RGB!` promotions and
  `composition_gstring_width`'s arguments) - the same defect existed in **slice 1's** C 970 line
  and was corrected with them (`(*( (*f).output_data.w32 as *mut w32_output )).cursor_pixel`,
  `femacs/src/w32term.rs` 1553), which is the only slice-1 text this batch changed.
- **C's `for (i = 0; …; ++i)` is the header's assignment plus a `while` whose last statement is
  C's increment** (slice 1's C 997-1001 reading), and `w32_draw_glyph_string_box`'s empty-bodied
  `for (last_glyph = g++; …; last_glyph = g++)` is that walk with C's two assignments in C's
  order (`last_glyph = g` then `g = g.add (1)`, which is what `g++` does).
- **C's `'	'`** in `w32_draw_composite_glyph_string_foreground` (C 1474) is the port's `9`;
  **C's `(…) == 0`** on the `int` its `||` builds (C 1551) is the port's `== false` on the two
  bitfield reads; **C's `eassert (s->gc != 0)`** (C 1134/1188) is `(*s).gc != 0 as *mut Emacs_GC`;
  and **C's chained `egc.foreground = relief->pixel = pixel;`** (C 1735) is one statement per
  assignment in C's right-to-left order, slice 1's recorded reading.
- **C's implicit conversions are written out where Rust needs them**: the `min`/`max` nests that
  mix an `int` literal with a `double` (`0xff as c_double`, the channel operands `as c_double` on
  the double side and `as c_int` on the int side), `bright`'s `int` sum widened after C's `/ 6`,
  the glyph members' `short`→`int` wideings (`(*g).pixel_width as c_int`, `ascent`/`descent`),
  `LGLYPH_*`'s `EMACS_INT`→`int` narrowings, the composition `offsets` table's `short` reads,
  `RGB`'s channel casts and the `BYTE`s `* 257` into the record's `unsigned short`, and
  `composition_gstring_width`'s `c_int`→`ptrdiff_t` arguments.  `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT`
  is a `c_long` because C's `187` widens in the `long bright` comparison; C's `char buf[7]` and
  `Lisp_Object glyph` are the port's zeroed locals (PORTING.md 1.5).
- **The two function-local statics keep their lifetimes**: `static unsigned char2b[8]` (C 1522) is
  a `static mut char2b: [c_uint; 8]` inside the function, and C's `s->char2b = char2b;` array decay
  is `&raw mut char2b as *mut c_uint`; the `glyph_u_cmp_bits!`/`glyphless_bits!` macros take a
  **pointer** (the `src/term.rs` 840-851 convention), so their call sites pass one.
- **`IMAGE_BACKGROUND!`'s no-heuristic arm casts to `c_ulong`.**  C's `?:` between the record's
  `unsigned long background` and `image_background`'s `RGB_PIXEL_COLOR` (a `COLORREF` for this
  configuration, dispextern.h 3766) promotes the shorter arm; the port's macro does the same, and
  its call site narrows back with C's own assignment conversion (`as COLORREF`).

### Slice 3 (C 1950-2715)

- **The `<windows.h>` / `<wingdi.h>` / `<gdiplus.h>` names this slice adds are spelled, not
  measured** - the same reading and the same site (C 25's chain) as slices 1 and 2, and the same
  **UNVERIFIED** status.  The plain Windows names: `FLOAT`, `XFORM` (six `FLOAT`s),
  `BITMAP`, `BITMAPINFOHEADER`, `DIBSECTION`, `GetObject`, `StretchBlt`, `SetStretchBltMode`,
  `SetBrushOrgEx`, `CopyImage`, `GetCurrentObject`, `SRCINVERT`, `SRCAND`, `HALFTONE`,
  `OBJ_BITMAP`, `IMAGE_BITMAP`.  The GDI+ names the C text of this slice *itself* names stand here
  too, gated `feature = "windowsnt"` the way `include/w32gdiplus.rs`'s own items are: `GpStatus`
  (the SDK's `Status`, with `Ok`), `GpImage`, `GpGraphics`, `GpBitmap`, `GpUnit` (with
  `UnitPixel`), `InterpolationMode` (with `InterpolationModeHighQualityBilinear`) and
  `DrawImageAbort` - the `src/w32image.rs` 244-293 reading of the same header, and the reason
  `include/w32gdiplus.rs`'s head note gives for leaving the SDK's GDI+ types to its includer.
  **The rest of that header's type set stays owed to the mirror** (owed 12): `GpImageAttributes`,
  `ARGB`, `PROPID`, `PixelFormat`, `RotateFlipType`, `GUID`/`CLSID`/`IID`, `IStream`,
  `PropertyItem`, `ImageCodecInfo`, `EncoderParameters`, `GdiplusStartupInput`/
  `GdiplusStartupOutput` and the three callback typedefs, `ULONG` and `GetThumbnailImageAbort`; the
  sweep reads **no error** for any of them, because rustc resolves a type alias only when it is
  used (measured: the arm reports `CRITICAL_SECTION`/`LPVOID`/`LOGFONT` from the same class and
  nothing for these).  Every record is `#[repr(C)]` and no size is measured (owed 24).
- **The GDI+ entry points are macros that yield the loaded `Option`.**  C's `GdipCreateFromHDC (…)`
  / `GdipDisposeImage (…)` / `GdipDeleteGraphics (…)` / `GdipSetInterpolationMode (…)` /
  `GdipDrawImageRectRectI (…)` / `GdipCreateBitmapFromHBITMAP (…)` are the port's
  `GdipCreateFromHDC! ().unwrap () (…)` and so on (the `pfnSetLayeredWindowAttributes.unwrap ()`
  reading above), and C's `… == Ok` compares the returned `GpStatus` with the value this unit
  declares.  C's `NULL` pointer arguments are `core::ptr::null`/`null_mut` and its null
  `DrawImageAbort` is `None`.
- **`struct image`'s `xform` member is by value in C and a `*mut c_void` in the mirror** (that
  mirror's own recorded deviation, `include/dispextern.rs` 2412-2416).  Every read is therefore
  the use-site cast C's `s->img->xform.eM12` needs - `(*( (*(*s).img).xform as *mut XFORM )).eM12`
  with C's `!= 0` written `!= 0 as FLOAT` (the member is a `FLOAT` and C's `0` converts to it) -
  and C's `&s->img->xform` is `(*(*s).img).xform as *mut XFORM`.  C's `&s->slice` is
  `&raw mut (*s).slice` (`image_ascent`'s third parameter).
- **C 2114-2148's `#ifdef WINDOWSNT` is the first *statement*-level guard in this file.**  Both arms
  are carried, C's own text stands in a comment above them, and because Rust cannot attach a `cfg`
  to an `else` clause, C 2148-2160's HALFTONE/`StretchBlt` block appears **twice**: as the GDI+
  test's `else` under `feature = "windowsnt"` and as the whole arm under
  `not (feature = "windowsnt")`.  C's `(pmode = SetStretchBltMode (…)) != 0` in both copies is the
  block operand the `terminal.rs` 479 reading gives an assignment inside a condition.
- **The bitfield runs read through the mirrors' accessors.**  `s->slice.x`/`y`/`width`/`height` are
  `glyph_slice_bits!` (`include/dispextern.rs` 599-608; each read lands in an `int` parameter or
  variable, so C's `unsigned` → `int` conversion is written out), `s->face->box` is
  `face_bits! (box, …)` - `box` is a Rust keyword, which is why the write-out is a macro - and
  `s->first_glyph->left_box_line_p`/`right_box_line_p`, `s->row`'s `reversed_p`/`mouse_face_p`/
  `mode_line_p` and `s->stippled_p`/`background_filled_p` are `glyph_bits!`/`glyph_row_bits!`/
  `glyph_string_bits!` (the two write sites are the macro's three-argument arm).
- **C's `!` on a `BOOL` is `== 0`.**  `!pfnPlgBlt (…)` is `pfnPlgBlt.unwrap () (…) == 0` and the
  bare `pfnPlgBlt` truth test is `is_some ()`; C's `corner` array decay at that call is
  `&raw mut corner as *mut POINT` (the slice 1 `w32_restore_glyph_string_clip` reading of the same
  decay), and C's `sizeof (dib)` / `sizeof (BITMAP)` are `core::mem::size_of::<DIBSECTION> ()` /
  `<BITMAP> ()` with the `LPVOID` argument spelled `as *mut c_void`.
- **C's `char segment`** (C 2640) is a `c_char`, so each arithmetic use writes C's own promotion to
  `int` out (`segment as c_int`) and `length = segment` is `length = segment as c_int`;
  `which = !which` is the recorded `(which == 0) as c_int`; and `w32_draw_dash`'s
  `for (x = s->x; …; x += length, length = segment)` is the header's assignment plus a `while`
  whose last two statements are C's increment in C's order.
- **`w32_fill_underline`'s `switch` is the port's `match`** over C's own labels, so
  `switch`/`case`/`default`/`break` cannot reappear and the `FACE_UNDERLINE_DOTS` arm's
  `FALLTHROUGH;` is carried as that arm's second statement (C's `w32_draw_dash` call written there
  as well, PORTING.md 2.2).  `emacs_abort` stands in two arms - `FACE_NO_UNDERLINE |
  FACE_UNDERLINE_WAVE` and `_` - where C has `case FACE_NO_UNDERLINE: case FACE_UNDERLINE_WAVE:
  default:`.  The five tokens are recorded in `docs/ports/macro-body-notes.tsv`.
- **Three more `dispextern.h` entry points are declared at their use sites** - `window_box_left`
  (3579), `window_box_left_offset` (3580) and `cursor_in_mouse_face_p` (3692) - the class slice 2's
  five name (owed 17).  `image_ascent` (dispextern.h 3774) is declared the same way but is **not**
  a gap: `src/image.rs` 2808 defines it as a non-`static` `#[unsafe(no_mangle)]` function, so the
  declaration keeps C's prototype and no stand-in (the `gui_define_fringe_bitmap` reading).
- **The unit includes `include/conf_post.rs`** because C's `config.h` AH_BOTTOM does (`DebPrint`,
  C 1807/2145/2353/2366 - the `w32dwrite.rs` 415-417 reading of the same line).  This is the one
  line this batch added to slice 1's include section, next to the `include/config.rs` line C 20
  names.
- **Two corrections inside earlier slices' lines** (measured by this batch's sweep, not new text):
  five `(*s).cmp` sites in slice 2 now cast `as *mut composition` (Evidence 6), and the slice 1
  `w32_draw_glyph_string_box` region is untouched.  The switch-token gap of `w32_fill_underline` is
  the only other earlier-slice-adjacent change; it is a table row, not port text.
- **Line count.**  C 8,414 (this slice 766) vs Rust 4,123 - the `docs/ports/line-drift-tolerance.tsv`
  row owed 2 below now covers a three-slice prefix.

## Owed by the integrator (nothing here is claimed as done)

1. `tools/check-port-lines.py --write-pending`: this file's ratchet row is now **stale**
   (`src/w32term.c: no longer under the threshold`; Evidence 4) - the file has left P-86's
   "under ten percent" set with this batch.
2. `docs/ports/line-drift-tolerance.tsv`: one row, `w32term`, tolerance ≥ 0.63, basis `total`,
   with the slice's reason from the Deviations.
3. `femacs/xtask/Cargo.toml`: two features are named by this unit and registered nowhere -
   `win32_winnt_lt_0500` (C 98's `_WIN32_WINNT < 0x0500`) and `win32_winnt_lt_0601` (C 129's).
   (`win32_windows_ge_0410` and `win32_vista` are the in-tree precedents for the naming.)
4. `docs/ports/include-w32term.md`: the mirror file this unit collides with needs the
   `*_decls`/`*_objs` module + glob-import treatment (`window.rs` 302-367 / `androidterm.rs`
   513-537 precedents) for **23 declarations** - the seven functions `XCreateGC`,
   `w32_release_paint_buffer`, `w32_fill_rect`, `w32_clear_window`, `w32_display_pixel_height`,
   `w32_display_pixel_width`, `w32_flip_buffers_if_dirty` and the sixteen objects
   `x_display_list`, `one_w32_display_info`, `dwWindowsThreadId`, `hWindowsThread`,
   `dwMainThreadId`, `hMainThread`, `w32_keyboard_codepage`, `w32_system_remap_font`, the seven
   `w32_system_caret_*` and `w32_message_fd`.  Until then **no `have_ntgui` translation unit can
   compile this file** - the pre-flight's patch 1 is exactly that list.
5. `include/dispextern.rs`: `draw_fringe_bitmap_params` needs `get_cursor_p`/`get_overlay_p`
   (C 2230-2231; the measured bits are byte 52 bit 0 and bit 1, `abi_shim.rs` 2185-2205's probe),
   and `Mouse_HLInfo` needs the `get_mouse_face_defer`/`set_…` pair (C 3020) - the `flags`/`bits`
   storage units the mirror records are only half the rule (PORTING.md 1.3).
6. ~~`include/w32term.rs`: `w32_output` needs the two bitfields' accessors (`paint_buffer_dirty`
   C 436, `want_paint_buffer` C 439)~~ - **DONE in 3.575**: the record now keeps C's storage unit as
   `pub bits: c_uint` (so every offset after it is unchanged, P-80) with `w32_output_bit_get` /
   `w32_output_bit_set` and the `w32_output_bits!` accessor; the five sites in this unit
   (C 776/793/1375/1383/1396) and `src/w32xfns.rs`'s three use it.  The member name is the macro's
   first argument, so the C token survives `--check`, and the read arm returns C's 0/1 value, so the
   `!= 0` / `== 0` comparisons keep C's spelling.
7. `include/w32term.rs`: `BLACK_PIX_DEFAULT`/`WHITE_PIX_DEFAULT` (C 43-44) expand to `PALETTERGB`,
   and `enter_crit!`/`leave_crit!` expand to `EnterCriticalSection`/`LeaveCriticalSection`: three
   `<windows.h>`/`<wingdi.h>` names no mirror declares.  A `macro_rules!` body is not resolved
   until it is expanded, so the two mirrors compile as text - but any unit that *expands* them
   fails (measured).
8. `include/w32term.rs`'s `w32_clear_rect!`/`w32_clear_area!`/`w32_fill_area!` pass
   `FRAME_BACKGROUND_PIXEL`'s `c_ulong` into `w32_fill_rect`'s `COLORREF` (`u32`); `include/w32gui.rs`'s
   `CONVERT_TO_EMACS_RECT`/`CONVERT_FROM_EMACS_RECT` mix `RECT`'s `LONG` with `Emacs_Rectangle`'s
   `c_int`/`c_uint`.  Both are `E0308` from the pre-flight and both need C's conversions written
   out in the mirror (measured).
9. `include/frame.rs`: `FRAME_W32_P` (the frozen arm only; the `HAVE_NTGUI` arm is the measured
   one), `FRAME_MOUSE_UPDATE` (its body compares `*mut frame` with `mouse_face_mouse_frame:
   *mut c_void` and calls `note_mouse_highlight`, declared nowhere), and the `have_ntgui` flag-run
   `compile_error!` (frame.h 471's `# ifndef HAVE_NTGUI` arm is unmeasured on this machine - the
   same missing toolchain this ledger records).
10. `include/dispextern.rs`: five names this slice calls are declared nowhere in the port -
    `get_glyph_string_clip_rect` (dispextern.h 3663), `gui_intersect_rectangles` (3676),
    `gui_define_fringe_bitmap` (3711; `abi_shim.rs` 2240 already defines it), `buffer_flipping_blocked_p`
    (3719), `lookup_basic_face` (3813) - so the unit declares them at their use sites
    (`androidterm.rs` 160-175 / `dispnew.rs` 257-259 precedent) and the mirrors stay owed.
11. `include/w32.rs`: `FD_SETSIZE`, `fd_set`, `fchmod` and `fchmodat` collide with
    `lib/include/sys_select.rs` and `lib/include/sys_stat.in.rs` (measured: two `E0428`s in the
    pre-flight after the `pub const`/`pub struct` pair; the `pub fn` pair is in the same class).
12. **The Windows arm of the mirror graph has never been compiled.**  Beyond items 1-11, the
    pre-flight's 141 mirror-anchored errors name what a `have_ntgui` + `windowsnt` translation
    unit cannot resolve today: 24 names in `include/w32gdiplus.rs` (`GpStatus`, `GpImage`,
    `GpBitmap`, `GpGraphics`, `GpImageAttributes`, `GpUnit`, `INT`, `ARGB`, `CLSID`, `GUID`,
    `IStream`, `VOID`, `HPALETTE`, `DrawImageAbort`, `GetThumbnailImageAbort`, `EncoderParameters`,
    `ImageCodecInfo`, `InterpolationMode`, `PixelFormat`, `PROPID`, `PropertyItem`,
    `RotateFlipType`, `GdiplusStartupInput`, `GdiplusStartupOutput`), 15 in `include/w32term.rs`
    (`MSG`, `HMENU`, `HPALETTE`, `HINSTANCE`, `CRITICAL_SECTION`, `WINDOWPLACEMENT`,
    `PALETTEENTRY`, `KEY_EVENT_RECORD`, `LPBYTE`, `LPCWSTR`, `WORD`, `UINT_PTR`, `Time`,
    `WM_USER`, `WM_MOUSELAST`), 11 `FRAME_*_SHIFT` constants in `include/frame.rs`, and the
    `buffer`/`dispextern`/`w32font`/`termhooks`/`w32common` names of the same class.  None of
    them is this unit's, and none of them is fixed here.
13. The layout assertions for the four `<windows.h>` records this unit's section spells (`POINT`,
    `RECT`, `_TOUCHINPUT`, the `MSG`/`RECT` pair in `W32Msg`) once a Windows or mingw toolchain
    exists (`xtask layout-asserts` reads a C compiler's readings; `x86_64-w64-mingw32-gcc` is
    absent here).
14. ~~The next lane's slice: **C 1950-8414**~~ - **DONE to C 2715 in this batch (slice 3)**, see
    the C line-range table and `## Deviations`' `### Slice 3`.  The next lane's slice is now
    **C 2716-8414**, whose first definition is **`w32_draw_glyph_string` (C 2720-3042)** - the
    glyph-string dispatcher, followed by `w32_shift_glyphs_for_insert` (C 3044-3060),
    `w32_delete_glyphs` (C 3061-3072), `w32_clear_frame` (C 3073-3096), `w32_ring_bell`
    (C 3097-3126), `w32_ins_del_lines` (C 3127-3138) and `w32_scroll_run` (C 3139-3243).  That
    slice also removes the remaining
    `missing file-scope object` rows (`dbcs_lead`, `utf16_high`, `utf16_high_modifiers`,
    `temp_index`, `temp_buffer`, `w32_initialized`) and the last remainder macro (`LOAD_PROC`,
    C 8244).
15. `include/w32term.rs` 340: `w32_query_frame_background_color` is declared there without owed
    4's `*_decls` module + glob-import treatment, so this unit's definition of it is `E0428`
    (measured in Evidence 5's base-set run: the primary anchor is the mirror line, the unit's
    definition the second).  It joins owed 4's list, which makes that list 8 functions and 16
    objects.
16. `include/lisp.rs`: `min`/`max` (lisp.h 71-73) are missing from it, and the only mirror that
    carries them (`lib/include/min-max.rs`, `lib/min-max.h`'s pair) is not on every unit's include
    graph - `src/w32term.rs` declares its own pair at the use site, the `haikuterm.rs` 1432
    `std::cmp::max` workaround being the other unit's answer to the same gap.
17. `include/dispextern.rs`: `struct face`'s bitfield run needs the three accessors this slice
    reads (`box`, `use_box_color_for_shadows_p`, `overstrike`), the `glyph` union's two units need
    `automatic`/`id` and `method`/`for_no_font`/`len`/`ch`, the `HAVE_NTGUI` arm needs
    `Emacs_Pix_Context`/`RGB_PIXEL_COLOR`, the `IMAGE_BACKGROUND`/`IMAGE_BACKGROUND_TRANSPARENT`
    macro pair is missing, and five entry points this slice calls are declared nowhere in the port
    (`window_box_right` 3581, `get_glyph_string_clip_rects` 3661, `prepare_face_for_display`
    3810, `image_background` 3769, `image_background_transparent` 3771 - all declared at this
    unit's use sites meanwhile).
18. **`<windows.h>` / `<wingdi.h>` has no mirror at all.**  `WORD`, `HFONT`, `RGB`, `PALETTERGB`,
    `GetRValue`/`GetGValue`/`GetBValue`, `CreatePatternBrush`, `SetTextAlign` and `TA_LEFT`/
    `TA_BASELINE` are this slice's additions to that per-unit section (P-88 clause 1: the
    declaration site is C 25's include chain, no file is added under `femacs/src/include/`).  If a
    Windows/mingw header mirror is ever added, the section's rows move into it (P-26/P-70).
19. `w32_check_font` (C 7747): named by this slice's two `IF_DEBUG (w32_check_font (…))` sites
    and declared nowhere, because C's prototype (C 256, inside `#ifdef GLYPH_DEBUG`) is dropped
    with the C 218-257 block.  `w32_clip_to_row`'s case (C 244/1739) is the other one: both need
    either a declaration at their use site or the definition's slice to land.
20. The mirrors' **frozen-only layout assertions** fire under the `have_ntgui` + `windowsnt` arm
    (24 `E0080`s: `face`/`font`/`font_driver` sizes and member offsets, `redisplay_interface`), so
    the arm cannot be type-checked end to end until those assertions are put behind the arm they
    were measured on - the same missing toolchain owed 13 records.

21. `include/w32term.rs` 1170-1173: the `extern "C"` block there declares
    `w32_image_rotations_p`, and this slice **defines** it, so the arm reads **one new `E0428`**
    (the primary anchor is the mirror's line, the unit's definition the second) until that
    declaration gets owed 4's `*_decls` module + glob-import treatment.  It joins owed 4's list,
    which becomes **9 functions** and 16 objects.  It is the only new collision this slice
    creates: `w32_image_size` is declared in `include/w32gui.rs`'s `w32gui_fn_decls` module (a
    crate-root definition shadows the glob-imported one), and the other nine definitions are C
    `static` functions, which the mirror does not declare.
22. **The three new mirror-anchored name gaps this slice's sweep reads**: `LPVOID`
    (`include/w32common.rs` 126-127), `CRITICAL_SECTION` (`include/w32term.rs` 1585) and `LOGFONT`
    (`include/w32font.rs` 177) - the class owed 12 names, and the same `<windows.h>` missing on
    this machine.  They are name-resolution errors in the **mirror**; none is this unit's text,
    and no mirror adds them here (P-72/P-88 leave the SDK's names to the target).
23. `include/dispextern.rs`: **three more entry points** are declared at this unit's use sites
    because no mirror carries them - `window_box_left` (dispextern.h 3579),
    `window_box_left_offset` (3580) and `cursor_in_mouse_face_p` (3692) - the `androidterm.rs`
    160-175 precedent owed 17 records for its five.  (`image_ascent`, dispextern.h 3774, is
    declared the same way but is **not** a gap: `src/image.rs` 2808 defines it.)
24. The layout assertions for this slice's four `#[repr(C)]` records (`XFORM`'s six `FLOAT`s, the
    `BITMAP`/`BITMAPINFOHEADER` pair and the `DIBSECTION` that embeds both, whose `sizeof` C 2008
    passes to `GetObject`) once a Windows or mingw toolchain exists - owed 13's list, and the GDI+
    records' assertions with them.
25. `docs/ports/macro-body-notes.tsv`: this batch **appended one row** (`w32_fill_underline`, the
    five `switch` tokens, `## Deviations`' `### Slice 3` records why); `docs/ports/
    excluded-macros.txt` was **not** touched (this slice excludes no macro).

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence 1/5: a 336-byte empty object), so this is a *text* port: nothing here has been
  executed, and no Windows target, GUI frame, `HDC`, font driver or redisplay loop was involved.
  There is no behavioural test, no differential run, no ERT and no device evidence.
- **The arm-reach reading is not a Windows compile.**  This machine has no `windows.h` and no
  mingw toolchain, so Evidence 5's base-set run is a `rustc` reading of the *mirror graph* under
  four `--cfg`s: 42 errors remain in it (16 in slice 1's own lines, 26 in the mirrors), it was
  taken **without** Evidence 3's nine stand-in patches, and "0 in slice 2's lines" means *no
  name-resolution or type error was anchored in this slice's text in that arm* - not that the
  unit compiles, and not that the arm is a working Windows configuration.
- **The `<windows.h>` records of this slice stay unmeasured.**  No `#[repr(C)]` record is added by
  slice 2, but the section it extends (`POINT`, `RECT`, `W32Msg`) is still **UNVERIFIED**, and the
  new typedefs/macros are C's text at PORTING.md 1.1's widths, with no layout reading (owed 13).
- **Slice 3's 19-error sweep is not a Windows compile either.**  Evidence 6's "0 in this slice's
  lines" is a name-resolution and type-check reading taken under a hand-set `--cfg` combination
  (`have_ntgui`, `windowsnt`, `have_window_system`, `dispextern_face_records`) in which 19 errors
  remain elsewhere; the official `windowsnt`-alone probe compiles the include graph and this unit's
  `windowsnt`-gated names only (clean), and the official `have_ntgui` + `windowsnt` probe is
  **refused by the mirror** (`include/frame.rs` 329's `compile_error!`), so *neither* official arm
  reads the GDI+ block, the image drawing or the underline drawing.  No behaviour was executed;
  there is no `HDC`, bitmap, image or redisplay loop anywhere in this batch's readings.
- **This slice's four records stay unmeasured.**  `XFORM`, `BITMAP`, `BITMAPINFOHEADER` and
  `DIBSECTION` are spelled from `<windows.h>`'s declaration order and PORTING.md 1.1's widths, and
  `sizeof (DIBSECTION)` - the size C 2008 hands `GetObject` - is `size_of`'s answer for that
  spelling, not a measured SDK reading (owed 24).
- **`extract-abi --check` still FAILs for the file**, by construction: 99 rows remain - 91
  `missing function`, 6 `missing file-scope object`, the `LOAD_PROC` macro of C 8244 and the
  line-count row.  Nothing in this ledger claims the file is complete, and the checklist is empty
  for it (P-86 ②), so the C line-range table and these readings are the substitute, not a pass.

### Slice 4 (C 2716-3038)

- **What it is.**  `w32_draw_glyph_string` (C 2719-3038) - the file's glyph painter - and nothing
  else: the successor-first background pass (C 2724-2744), the GC/clipping setup (C 2747), the
  relief-or-clip choice (C 2749-2772), the `switch (s->first_glyph->type)` (C 2774-2811) and the
  `if (!s->for_overlaps)` block (C 2813-3037).  Port: 4,123 -> **4545 lines** (+422).  The C
  line-range table's last row is split at C 3038; the next live definition is C 3044's
  `w32_shift_glyphs_for_insert` (C 3040-3054).
- **The names this slice adds stand at the use site** (P-88 (1)); the *bit* values are the
  measured ones, the entry point is `UNVERIFIED`:
  * `font_for_underline_metrics` (C 2852's `font.h` entry) - no mirror in this unit's graph carries
    it, so the declaration stands with the function (`unsafe extern "C" { pub fn
    font_for_underline_metrics (s: *mut glyph_string) -> *mut font; }`).
  * the eight `struct face` bit constants and the seven `face_bits!` arms C 2820-2971's text reads
    (`FACE_UNDERLINE_SHIFT` 2, `FACE_UNDERLINE_MASK` 0x7, `FACE_OVERLINE_P` 1<<6,
    `FACE_STRIKE_THROUGH_P` 1<<7, `FACE_UNDERLINE_DEFAULTED_P` 1<<10,
    `FACE_OVERLINE_COLOR_DEFAULTED_P` 1<<11, `FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P` 1<<12,
    `FACE_UNDERLINE_AT_DESCENT_LINE_P` 1<<14 - the same values `src/xterm.rs` 9850-9870 measured
    from dispextern.h 1893-1935, repeated here because `include/dispextern.rs`'s own copy is behind
    `dispextern_face_records`, the mirror row that is still owed).
- **C's `struct glyph_string` member types this unit's record gives**: `hl` is the
  `draw_glyphs_face` *value* (a `c_int`), not a pointer, so C's `s->hl` reads and writes are
  `(*s).hl` with no dereference (the first spelling of this slice had `(*(*s).hl)` - the
  `src/xterm.rs` shape of *its* record - and the arm read the eight `E0614`s); `gc->foreground` and
  the three face colours are `unsigned long` members, so C's implicit conversion to `COLORREF` is
  written `as COLORREF` (three `E0308`s); and `face->underline_pixels_above_descent_line` is an
  `int` member, so C's truth test is `!= 0` (one `E0308`).
- **The successor pass is C's `for` with its increment clauses written last** (C 2731-2733:
  `width += next->width, next = next->next` after the body), the two overhang-repair loops keep
  C's `for (prev = s->prev; prev; prev = prev->prev)` shape the same way, and the underline block
  writes C's unsigned arithmetic with `wrapping_add`/`wrapping_sub` (C 2911-2914, 2932); C's
  `max (…)` is this file's `max! (…)`.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes** (the file gate compiles none of
     this text).
  2. The arm sweep (direct `rustc` on the materialized root, the base configuration
     `have_ntgui + windowsnt + have_window_system + dispextern_face_records`) -> **22 errors, 0
     anchored in this slice's text**: the ten primary unit anchors are the *pre-existing* rows at
     `_root.rs` 1322-1561 (the seven `E0308`s of the `draw_fringe_bitmap_params` records and the
     `overlay_p`/`cursor_p` field gaps - Owed 11's class), and the rest are the mirrors'
     (`include/w32term.rs` 4, `font.rs` 4, `w32common.rs` 2, `frame.rs` 1, `w32font.rs` 1,
     including the two `E0428` pairs the P-39 rule still owes).  The first spelling read **35**
     errors with ten of them *in* this slice's text - the four fixes above - and the intermediate
     reading after the `face_bits!` extension was 21 (the same pre-existing rows).
  3. `extract-abi.py --check` -> **99 -> 98 rows** (91 -> **90** `missing function`: the round's
     own `w32_draw_glyph_string` landed), with one token row **added** to
     `docs/ports/macro-body-notes.tsv` (`w32_draw_glyph_string`, `break,case,cmp,default,switch`:
     C's `switch` is the port's `match` and C 2794's `u.cmp.automatic` is read through this unit's
     `glyph_u_cmp_bits!`), and the line-count row still reports the port *behind* C (46.0%;
     4,545 vs 8,414) - the file needs a `line-drift-tolerance.tsv` row only when it completes.
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 5 (C 3039-3235)

- **What it is.**  Six functions: the five small frame entries (`w32_shift_glyphs_for_insert`,
  `w32_delete_glyphs`, `w32_clear_frame`, `w32_ring_bell`, `w32_ins_del_lines`) and the scroller
  `w32_scroll_run` (C 3136-3235, with its two bounding-box branches, the dirty-region check and
  the double-buffering arm).  Port: 4,545 -> **4812 lines** (+267).  The table's last row splits at
  C 3235; the next live definition is C 3239's `w32_frame_up_to_date` (the "Exposure Events"
  section).
- **The names this slice adds stand at the use site** (P-88 (1)): `CreateRectRgn`, `EqualRgn`
  (the two `<wingdi.h>` entries), `FlashWindow`, `ScrollWindowEx` and `SW_INVALIDATE`
  (`<winuser.h>`), `Sleep` (`<winbase.h>`), and the two reference-side prototypes no mirror in this
  unit's graph carries - `gui_clear_cursor` (dispextern.h 3657) and `window_box` (dispextern.h
  3570-3571, whose `int *` out-parameters the port passes as `&raw mut`).  The SDK records are
  `UNVERIFIED`; the two prototypes are read from the reference's own headers.
- **The slice's three spellings the reader should know.**  (1) C's chained assignment
  `from.left = to.left = x;` (C 3208-3209) is two statements in C's own order (the inner target
  first), because Rust has no assignment expression.  (2) C's `RECT from; RECT to;` locals are
  `core::mem::zeroed ()`-started (PORTING.md 1.5, the `w32_clear_window` reading) since every
  member is written below.  (3) `DeleteObject`'s declared parameter is `HGDIOBJ`, so C's three
  HRGN arguments are passed `as HGDIOBJ` (the conversion C performs implicitly).
- **`w32_scroll_bar_clear` (C 3088) is a forward reference to this file's own C 5025** - the
  remainder's definition, the class the arm table already records for `update_theme_scrollbar_*`;
  C's forward declaration (C 224) is dropped by PORTING.md 0.1-12 as usual.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (same base configuration as slice 4) -> **22 errors, 0 anchored in this
     slice's text** - the reading is *unchanged* from slice 4's, i.e. the six functions added no
     own error and no mirror row moved: the ten unit anchors are still the pre-existing
     `_root.rs` 1322-1561 rows (Owed 11's fringe/`draw_fringe_bitmap_params` class) and the twelve
     mirror anchors are the same files.  (`w32_scroll_bar_clear`'s forward reference is not one of
     them: rustc resolves a plain `fn` defined later in the module.)
  3. `extract-abi.py --check` -> **98 -> 92 rows** (90 -> **84** `missing function`: the round's six
     landed), no token rows, and the line-count row improves from 46.0% to **42.8%** (4,812 vs
     8,414 - the port is still behind C by construction).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 6 (C 3237-3431)

- **What it is.**  The "Exposure Events" section's focus machinery and the two keyboard entries this
  file opens with: `w32_frame_highlight`/`w32_frame_unhighlight`, `w32_new_focus_frame`,
  `w32_focus_changed`, `w32_detect_focus_change`, C 3350-3358's `#if 0` block, `w32_frame_rehighlight`
  and `w32_frame_rehighlight_1`, plus `get_keysym_name` (C 3405-3420, the file's only non-`static`
  entry in the block) and `codepage_for_locale`.  Port: 4,812 -> **5110 lines** (+298).  The table's
  last row splits at C 3431; the next live definition is C 3439's `parse_button` (the mouse-message
  `switch`, C 3445-3560).
- **The names this slice adds stand at the use site** (P-88 (1)): `WM_SETFOCUS`/`WM_KILLFOCUS`
  (`<winuser.h>`), `GetKeyNameText` (the `…A` export), `<winnls.h>`'s `LCID`/`LCTYPE` pair with
  `LOCALE_IDEFAULTANSICODEPAGE` and `CP_ACP` (the `src/w32proc.rs` slice-23/24 readings of the same
  header), `GetLocaleInfo` (the `…A` export), the CRT's `atoi`, and `gui_update_cursor`
  (dispextern.h 3656, `bool` parameter).  All UNVERIFIED except the two Emacs-side prototypes.
- **Two bitfield accessors stand at the use site** (the mirrors declare the units and the shifts,
  the accessors live in the units that read them):
  * `frame_bits! (auto_lower|auto_raise, f)` - frame.h 405/408's two bits, whose shifts
    `include/frame.rs` 463-479 gives per configuration (`src/xterm.rs` 22696-22700,
    `src/haikuterm.rs` 722-728 are the sibling copies).
  * `input_event_bits! (kind, bufp, v)` - termhooks.h 377-378's 16+16-bit `kind`/`part` unit, which
    `include/termhooks.rs` 116 carries as `bits: u32`; only the write arm this slice needs is
    declared (`src/xterm.rs` 22985-22999 is the full copy).
- **The four token rows this slice's `--check` first produced are the two spellings C's types force**,
  and they are recorded in `docs/ports/macro-body-notes.tsv` rather than spelled to satisfy the
  counter: C's `gui_update_cursor (f, 1)` passes `1` for a `bool` parameter (the port writes `true`),
  and C's two null-pointer spellings (`w32_new_focus_frame (dpyinfo, 0)` at C 3316 and
  `dpyinfo->highlight_frame = 0` at C 3394) are `core::ptr::null_mut ()` in the port.  Both readings
  are the sibling ports' own (`src/xterm.rs` 22748/23031/18995).
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration, as slices 4-5) -> **22 errors, 0 anchored in this slice's
     text** - the reading is unchanged again: the ten unit anchors remain the pre-existing
     `_root.rs` 1322-1561 rows and the twelve mirror anchors the same files.  The round's first
     spelling read **24**, the two extra being `no field kind on type input_event` (the `bits: u32`
     deviation above, fixed by the `input_event_bits!` write arm).
  3. `extract-abi.py --check` -> **92 -> 83 rows** (84 -> **75** `missing function`: the round's nine
     landed), four token rows added to the notes file (above), and the line-count row improves from
     42.8% to **39.3%** (5,110 vs 8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 7 (C 3433-3629)

- **What it is.**  The mouse section's head: the "Mouse clicks and mouse movement.  Rah." comment,
  `parse_button` (C 3439-3499, the file's mouse-message `switch`), `w32_construct_mouse_click`
  (C 3501-3532) and `w32_construct_mouse_wheel` (C 3534-3629, with the high-precision wheel arm and
  the coalescing accumulator).  Port: 5,110 -> **5401 lines** (+291).  The table's last row splits
  at C 3629; the next live definition is C 3636's `w32_lispy_file_action` (the
  `#if HAVE_W32NOTIFY` block).
- **The names this slice adds stand at the use site** (P-88 (1)): the six `WM_*BUTTON*` messages,
  `WHEEL_DELTA` and `ScreenToClient` (`<winuser.h>`), the `MSG` record (`W32Msg`'s member type,
  which no mirror in this unit's graph declares - the `src/w32fns.rs` 379-388 / `src/w32proc.rs`
  slice-25 spelling) and `<windef.h>`'s `LOWORD` (the companion of the unit's own `HIWORD`).  All
  UNVERIFIED.
- **The `parse_button` switch is the port's `match`** over C's eight message labels (each keeping
  C's `NILP (Vw32_swap_mouse_buttons)` branch) with C's `default:` returning `FALSE`; the token row
  the `--check` first produced (`break,case,default,switch`) is recorded in
  `docs/ports/macro-body-notes.tsv`, the `w32_fill_underline`/`pgtk_draw_window_cursor` class.
- **`parse_button` is one of the names `w32term.h` has to declare but `src/w32term.c` defines, and
  the mirror declared it at its crate root**: the arm reading's `E0428: the name parse_button is
  defined multiple times` (23 errors) is what the round measured, and the declaration moved into
  `include/w32term.rs`'s `w32term_fn_decls` (P-21/P-39's device, the shape that module already uses
  for `w32_window_to_frame`/`w32_clear_window`), the reading returning to **22**.  The move and its
  evidence are `docs/ports/include-w32term.md`'s.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text**.  The
     round's first spelling read **34**; the eleven own-text rows were the four
     `Vw32_swap_mouse_buttons ()` calls (the `globals.rs` name is a `static`, not a `Q…`-style
     function), the three `FRAME_LINE_HEIGHT!` spellings (a function, not a macro) and the four
     `XSETINT!` arguments (`make_fixnum` takes `EMACS_INT`), and the intermediate reading after
     those fixes was 23 - the `parse_button` `E0428` above.
  3. `extract-abi.py --check` -> **83 -> 80 rows** (75 -> **72** `missing function`: the round's
     three landed), one token row added to the notes file (above), and the line-count row improves
     from 39.3% to **35.8%** (5,401 vs 8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 8 (C 3631-3763)

- **What it is.**  The `#if HAVE_W32NOTIFY` block: `w32_lispy_file_action` (C 3636-3670, the
  five-`FILE_ACTION_*` switch with C 3659-3666's `sprintf`/`intern` default) and the `#ifdef
  WINDOWSNT` `w32_queue_notifications` (C 3676-3761, the critical-section drain of the
  notifications-set list, the per-entry event build and the two `xfree`s).  Port: 5,401 ->
  **5610 lines** (+209).  The table's last row splits at C 3763; the next live definition is C
  3774's `w32_note_mouse_movement`.
- **The gate** is the *registered* `have_w32notify` feature (xtask's `Cargo.toml` 366, whose comment
  names the `src/w32xfns.rs` 66-79 reading of the same macro) plus the file gate; `HAVE_W32NOTIFY`
  itself is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1675, so the arm is read with
  `--cfg feature="have_w32notify"` added to the base set (below).
- **The names this slice adds stand at the use site** (P-88 (1)): `FILE_NOTIFY_INFORMATION` (with
  its `FileName[1]` tail, `PFILE_NOTIFY_INFORMATION`), the five `FILE_ACTION_*` values,
  `FILE_NOTIFY_EVENT` (termhooks.h's enumerator, value 28 - the `src/inotify.rs` 284 reading) and
  gnulib's `INT_STRLEN_BOUND` (`src/xterm.rs` 19085's body, the one C 3661's bound needs).  The SDK
  records are UNVERIFIED; the enumerator and the macro are read from the reference and the sibling
  port.
- **C's `static char unknown_fmt[]` is the tree's element-list device**, not `*b"…"`: `c_char` is
  `i8` here, so the byte-string form is an `E0308` (measured in this round's arm reading, and the
  same latent defect slice 24 left in `src/w32proc.rs`'s `hex` - fixed in this round too, with the
  reason in that unit's comment and this round's status bullet).
- **`w32_lispy_file_action` is the second name this mirror had to hand to `w32term_fn_decls`**:
  `src/w32term.h` 795 declares it and `src/w32term.c` 3636 defines it, so the crate-root
  declaration was an `E0428` as soon as the unit defined it (the same case slice 7 measured for
  `parse_button`); the arm reading went **23 -> 22** when it moved, and the move is
  `docs/ports/include-w32term.md`'s.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep with the base configuration **plus `have_w32notify`** -> **22 errors, 0 anchored
     in this slice's text** (the base configuration without that feature also reads 22 on this
     revision).  The round's first spelling read **26**: the three own-text rows were the
     byte-string `unknown_fmt` (an `E0308`, fixed above), its array length (19, not 22 - C's
     `sizeof (unknown_fmt) - 1` is 18) and `kbd_buffer_store_event`'s `*mut c_void` parameter (P-55's
     shape, the argument cast at the call site); the fourth was the mirror `E0428` above.
  3. `extract-abi.py --check` -> **80 -> 79 rows** (72 -> **70** `missing function`: the round's two
     landed), one token row added to the notes file (the `switch` again), and the line-count row
     improves from 35.8% to **33.3%** (5,610 vs 8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 9 (C 3765-3970)

- **What it is.**  The mouse-movement reporter and the pointer-motion reader:
  `w32_note_mouse_movement` (C 3774-3819), the "Mouse Face" banner with C 3826-3834's three own
  forward declarations (dropped, 0.1-12), `w32_define_cursor` (C 3835-3839) and `w32_mouse_position`
  (C 3861-3970, with C 3900-3943's innermost-window search and C 3945-3966's store block).  Port:
  5,610 -> **5875 lines** (+265).  The table's last row splits at C 3970; the next live definition is
  C 3981's `w32_handle_tab_bar_click` (the "Tab-bars" banner at C 3973-3975).
- **The names this slice adds stand at the use site** (P-88 (1)): `<winuser.h>`'s `PostMessage` (the
  `…A` export), `WindowFromPoint`, `ChildWindowFromPoint` and `GetCursorPos`, plus the two
  reference-side prototypes no mirror in this unit's graph declares - `remember_mouse_glyph`
  (dispextern.h 3586, whose `NativeRectangle *` is this unit's `RECT *`) and `gui_mouse_grabbed`
  (dispextern.h 4006, `Display_Info *` being `w32_display_info *` for this term, the
  `src/xterm.rs` 15317-15323 reading of the same pair).  The SDK records are UNVERIFIED.
- **`frame->mouse_moved` is the frame bit run's `bool_bf mouse_moved : 1`** (frame.h 434): the
  slice's three reads/writes go through the unit's own `frame_bits!`, whose `mouse_moved` read and
  write arms (and the `set_frame_flag1!` helper the write arm needs) are declared with it - the
  `src/xterm.rs` 22681/22696-22700 copy of the same pair.
- **C 3886-3887's `FOR_EACH_FRAME (tail, frame) XFRAME (frame)->mouse_moved = false;`** is the
  mirror's `FOR_EACH_FRAME! (tail, frame, { … });` (the three-argument shape frame.rs 800-806
  declares and `src/androidfns.rs` 1948 uses), and C 3926-3930's `? :` between the tooltip check's
  two frames is the port's `if`/`else` expression.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text** - the
     reading is unchanged from slices 4-8, i.e. the round's three functions and its two accessor
     arms added no own error.  (The first `build-tu` run of the round failed on the `FOR_EACH_FRAME!`
     arity above; that is a *text* fix in the same revision, and the frozen `build-tu` is the
     reading that caught it - the arm reading compiled clean.)
  3. `extract-abi.py --check` -> **79 -> 75 rows** (70 -> **67** `missing function`: the round's
     three landed), no token rows, and the line-count row improves from 33.3% to **30.2%**
     (5,875 vs 8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 10 (C 3973-4157)

- **What it is.**  The two click handlers (tab-bar and tool-bar), the "Scroll bars" banner with its
  two comments, `w32_window_to_scroll_bar` (the two-list walk C 4041-4047 writes as one loop) and
  the two thumb setters (`w32_set_scroll_bar_thumb` with C 4085-4096's drag-time `GetScrollInfo`
  probe, and `w32_set_horizontal_scroll_bar_thumb`).  Port: 5,875 -> **6138 lines** (+263).  The
  table's last row splits at C 4157; the next live definition is C 4164's `my_create_vscrollbar`
  (the "Scroll bars, general" banner at C 4160-4162).
- **The names this slice adds stand at the use site** (P-88 (1)): `handle_tab_bar_click` and
  `handle_tool_bar_click` (dispextern.h 3668-3671, both taking a `bool down_p`), the `SCROLLINFO`
  record with `SIF_RANGE`/`SIF_PAGE`/`SIF_POS`, `SB_CTL`, `GetScrollInfo` and `SetScrollInfo`
  (`<winuser.h>`).  The SDK records are UNVERIFIED; the two Emacs-side prototypes are read from the
  reference header.
- **C 4071's `f` argument is dead text in the reference itself**: `w32term.h` 581-582's
  `VERTICAL_SCROLL_BAR_INSIDE_HEIGHT(f,height)` never expands `f`, and `w32_set_scroll_bar_thumb`
  has no `f` in scope (C compiles because a macro argument is unexpanded text).  The port passes a
  typed null in that slot (`VERTICAL_SCROLL_BAR_TOP_RANGE! (core::ptr::null_mut (), …)`) with the
  reason in the comment; C's name is one character, so the token comparison never saw it either way.
- **The token rows of this slice are the `bool` parameter again**: C's `handle_tab_bar_click (f, x,
  y, 1, 0)` passes `1` for dispextern.h 3668's `bool down_p`, and the port writes `true` - the
  `w32_frame_highlight` row's class, recorded in `docs/ports/macro-body-notes.tsv` for both handlers.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text** - the
     reading is unchanged from slices 4-9, i.e. the round's five functions and its `SCROLLINFO`
     record added no own error.
  3. `extract-abi.py --check` -> **75 -> 70 rows** (67 -> **62** `missing function`: the round's
     five landed), two token rows added to the notes file (above), and the line-count row improves
     from 30.2% to **27.1%** (6,138 vs 8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 11 (C 4160-4328)

- **What it is.**  The "Scroll bars, general" section's head: the eight `my_*` wrappers (two
  creators, the shover, the position setter, the foreground raiser, the destroyer and the
  bring-to-top, plus C 4212-4219's `#if 0` `my_set_focus`), `w32_scroll_bar_create` (the
  `ALLOCATE_PSEUDOVECTOR` bar, the nine field stores, the `SCROLLINFO` fill and the frame's list
  insert) and `w32_scroll_bar_remove`.  Port: 6,138 -> **6390 lines** (+252).  The table's last row
  splits at C 4328; the next live definition is C 4338's `w32_set_scroll_bar_end`.
- **The names this slice adds stand at the use site** (P-88 (1)): `SIF_ALL` (`<winuser.h>`), the
  `WINDOWPOS` record and the `SendMessage` entry the two creators call.  The SDK records are
  UNVERIFIED.
- **C 4180's `/*#define ATTACH_THREADS*/` keeps both `#ifndef` arms in C, but only the first is
  compiled**: `my_show_window` and `my_set_window_pos` therefore carry C's `#else` arms as C's own
  text in their doc comments (P-51) and the two tokens (`ShowWindow`, `SetWindowPos`) cannot
  reappear in the port's body - the pair of rows this round added to
  `docs/ports/macro-body-notes.tsv`.  C 4212-4219's `#if 0` block is carried the same way.
- **`ALLOCATE_PSEUDOVECTOR! (scroll_bar, w32_widget_high, pvec_type::PVEC_OTHER)`** is the mirror's
  three-argument macro (lisp.rs 2970-2974, whose `$field` is the member before the pseudovector
  tail); `src/xterm.rs` 28266 is the sibling call of the same C text with its own field name.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text**.  The
     round's first spelling read **23**: the one own-text row was `my_show_window`'s
     `SendMessageTimeout` result, which C converts to the function's `BOOL` return implicitly and
     the port now casts (`as BOOL`).
  3. `extract-abi.py --check` -> **70 -> 61 rows** (62 -> **53** `missing function`: the round's nine
     landed), two token rows added to the notes file (the dead `#else` arms above), and the
     line-count row improves from 27.1% to **24.1%** (6,390 vs 8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 12 (C 4330-4536)

- **What it is.**  The two scroll-bar setters and nothing else: `w32_set_vertical_scroll_bar`
  (C 4330-4432) and `w32_set_horizontal_scroll_bar` (C 4434-4536), each with its create branch
  (the `w32_clear_area` pass and `w32_scroll_bar_create`) and its move branch (the "already
  positioned" redraw through `InvalidateRect`, the clear-`SetWindowPos`-`SetScrollInfo`-reshow run
  and the four remembered settings).  Port: 6,390 -> **6651 lines** (+261).  The table's last row
  splits at C 4536; the next live definition is C 4544's `w32_scroll_bar_clear`.
- **The names this slice adds stand at the use site** (P-88 (1)): `SW_HIDE`, `SW_NORMAL`,
  `HWND_BOTTOM` (`((HWND) 1)`, written `1 as HWND`), `SWP_FRAMECHANGED`, `InvalidateRect` and
  `SetWindowPos` (`<winuser.h>`).  The SDK records are UNVERIFIED; `w32_clear_area` (w32term.h 660)
  and `w32_clear_under_internal_border` (w32fns.c 1587) are the mirror's - both already in this
  unit's graph, so neither is re-declared.
- **The horizontal twin's two extra locals** are C 4447-4448's `clear_left`/`clear_width`
  (`WINDOW_LEFT_EDGE_X` and `WINDOW_PIXEL_WIDTH - WINDOW_RIGHT_DIVIDER_WIDTH`), which the create
  branch uses instead of the bar's own `left`/`width`; everything else is the vertical text's mirror
  with `wset_horizontal_scroll_bar` and the horizontal thumb setter at the tail.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text** - the
     reading is unchanged from slices 4-11, i.e. the round's two functions and its six SDK names
     added no own error.
  3. `extract-abi.py --check` -> **61 -> 59 rows** (53 -> **51** `missing function`: the round's two
     landed), no token rows, and the line-count row improves from 24.1% to **21.0%** (6,651 vs
     8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 13 (C 4538-4684)

- **What it is.**  The three scroll-bar hooks redisplay drives: `w32_condemn_scroll_bars` (the
  prepend of this frame's list onto the condemned one), `w32_redeem_scroll_bar` (the vertical and
  horizontal unlink-and-reinsert pairs) and `w32_judge_scroll_bars` (the removal walk).  Port:
  6,651 -> **6823 lines** (+172).  The table's last row splits at C 4684; the next live definition
  is C 4694's `w32_scroll_bar_handle_click`.
- **C 4599's `goto horizontal` is the port's `break 'horizontal`**: the label sits after the
  vertical arm (C 4622), so that arm is wrapped in a labelled block whose exit is C's jump - the
  `w32proc` `new_child`/`create_child` device - and the token row the `--check` first produced
  (`goto`) is recorded in `docs/ports/macro-body-notes.tsv` with that reason (the
  `w32_executable_type` row's class in the same file).
- **C 4672-4673's `for (; ! NILP (bar); bar = next)`** is the port's `while` with C's increment
  clause at the loop's end, `next` being the guard C 4678 reads before C 4679 clears the two
  pointers.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text** - the
     reading is unchanged from slices 4-12.
  3. `extract-abi.py --check` -> **59 -> 56 rows** (51 -> **48** `missing function`: the round's
     three landed), one token row added to the notes file (above), and the line-count row improves
     from 21.0% to **18.9%** (6,823 vs 8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 14 (C 4686-4915)

- **What it is.**  The two scroll-bar click handlers: the vertical one (C 4686-4800) and the
  horizontal one (C 4802-4915), each of them the event fill, the `GetScrollInfo` probe with its
  `SIF_TRACKPOS`/`SIF_POS` choice, the `SB_*` dispatch (fourteen and thirteen labels) and the two
  `XSETINT`s.  Port: 6,823 -> **7162 lines** (+339).  The table's last row splits at C 4915; the
  next live definition is C 4926's `w32_scroll_bar_report_motion`.
- **The names this slice adds stand at the use site** (P-88 (1)): the `SB_*` command values (the
  vertical and horizontal sets that share numbers) and `SIF_TRACKPOS`, all `UNVERIFIED`.
- **C's dead `f` appears twice per handler and the port keeps it as C writes it**:
  `VERTICAL_SCROLL_BAR_TOP_RANGE (f, …)`/`HORIZONTAL_SCROLL_BAR_LEFT_RANGE (f, …)` at C 4709/4825
  and `FRAME_DISPLAY_INFO (f)` at C 4729/4846, with C 4728/4845's own `struct frame *f; /* Value is
  not used.  */`.  None of the three macros expands its argument, and a Rust `macro_rules!`
  argument that no expansion uses is dropped the same way - **measured** (the round's arm reading
  shows no `cannot find value \`f\`` row with the unbound name in place), which also retires slice
  10's stand-in: that site now passes C's own `f` too, with the reason in its comment.
- **`input_event_bits!` gained the `part` write arm** (the same 16+16-bit unit as `kind`'s, the
  `src/xterm.rs` 1650-1658 copy) - the two handlers set `emacs_event->part` seventeen times between
  them.
- **`FALLTHROUGH` is C's, written in the arm it falls from**: both handlers' `SB_ENDSCROLL` arms end
  in the `default:` arm's two statements (PORTING.md 2.2), and the two `switch` token rows are in
  `docs/ports/macro-body-notes.tsv` with that reading.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text**.
  3. `extract-abi.py --check` -> **57 -> 54 rows** (48 -> **46** `missing function`: the round's two
     landed), two token rows added to the notes file (above), and the line-count row improves from
     18.9% to **14.9%** (7,162 vs 8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 15 (C 4917-5128)

- **What it is.**  The two scroll-bar motion reports - `w32_scroll_bar_report_motion`
  (C 4917-4965) and its horizontal twin `w32_horizontal_scroll_bar_report_motion` (C 4967-5016),
  each the four display-info reads, the `GetScrollInfo` probe with its `SB_THUMBTRACK`/`SIF_*`
  choice, the `scroll_bar_handle` store and the two `XSETINT`s - then `w32_scroll_bar_clear`
  (C 5019-5061, the double-buffering guard and the `for` sweep over `FRAME_SCROLL_BARS`), the
  socket reader's five `static` objects (C 5063-5076), `mouse_or_wdesc_frame` (C 5078-5107) and
  C 5128's `menubar_selection_callback` declaration.  Port: 7162 -> **7456 lines** (+294).  The
  table's last row splits at C 5128; the next live definition is C 5130's `w32_read_socket`.
- **The two reports are already named by this unit's own text**: `w32_mouse_position` (C 3861-3970)
  calls both at C 3875/3877, and Rust's item order resolves them, so this slice adds no forward
  declaration (C's three `static` prototypes are dropped, PORTING.md 0.1-12, recorded in the
  comment above C 3879's block).
- **C's own integer conversion is stated where C writes it**: C 4950/5000's `si.nMax - si.nPage + 1`
  mixes `int` (`nMax`) with `UINT` (`nPage`), and the port writes the narrow the C 4842 row above
  uses - `si.nMax - si.nPage as c_int + 1`.
- **`mouse_or_wdesc_frame` is the same function the X side has** (this unit's copy is C 5078-5107;
  `src/xterm.rs` 29989 is its sibling): C's `?:` on `gui_mouse_grabbed` is an `if` expression, the
  two `!EQ`-style tests keep C's order, and `FRAME_TOOLTIP_P`/`Qdropping` are the mirror's.  This
  unit's copy has **one** test at C 5093 where the X one has two (no `Qdrag_source` clause).
- **The five socket objects are C `static`**, so the port writes them without `pub` (the
  `last_mousemove_x/y` shape); `temp_buffer` is C's `short[100]`, written `[c_short; 100]`.
- **C 5128's declaration is kept as a declaration**: the socket reader calls
  `menubar_selection_callback`, whose definition is `w32menu.c` 386's (ported at
  `src/w32menu.rs` 1216), so the name stands at this use site in an `extern "C"` block (P-88 (1)).
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text** (the ten
     unit anchors stay at `_root.rs` 1322-1561, owed 11; the other twelve are mirrors).
  3. `extract-abi.py --check` -> **54 -> 46 rows**: `missing function` 46 -> **42** (the round's four
     definitions), `missing file-scope object` 6 -> **1** (the five above; `w32_initialized`
     remains), no new token row, and the line-count row improves from 14.9% to **11.4%**
     (7,456 vs 8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 16 (C 5130-6458)

- **What it is.**  The whole socket reader: `static int w32_read_socket (struct terminal *terminal,
  struct input_event *hold_quit)` - C 5134-5139's locals, C 5141's `block_input`, C 5144's
  `drain_message_queue`, C 5147-6399's `while (get_next_msg (&msg, FALSE))` body with the
  `switch (msg.msg.message)` over **twenty-five arms** (C 5169 `WM_EMACS_PAINT` through C 6122
  `WM_TOUCHMOVE`, C 6344's `default:`), C 6403-6407's autoraise, C 6409-6454's `FOR_EACH_FRAME`
  visibility sweep and C 6456-6457's answer.  Port: 7,456 -> **9,057 lines** (+1,597).  The table's
  last row splits at C 6458; the next live definition is C 6473's `w32_clip_to_row`.
- **C's `switch` is the labelled `'dispatch` block and C's mid-arm `break`s are `break
  'dispatch`** (PORTING.md 2.2): C 5335/5366/5374/5384/5395 (the `WM_CHAR`/DBCS exits), C 5432
  (`WM_MOUSEMOVE`'s non-movement test), C 5669/5687 (the DND guards), C 6089 (the duplicate
  monitor event) and C 6131 (no touch entry point).  `WM_SETFOCUS`/`WM_KILLFOCUS` are the mirror's
  `UINT` constants, so those two arms are guards over C's own tokens with the scrutinee's cast.
- **C's two `goto touch_located` are a forward jump into the body's own station**, so the text
  between the arm's head and C 6191 is a labelled block (`break 'touch_located`) - and because a
  Rust `continue` inside a labelled block may not target a loop outside it (`E0695`, **measured**:
  three rows), C's `for` loop carries the label C's `for` does not need and its three
  `continue`s are `continue 'touch_points`.  C's `for` performs its update on `continue`, so the
  port's `while` writes C's own `i += 1`/`++i` at both sites (PORTING.md 2.2's `fringe.rs` shape) -
  including the two inner `for (x = 0; ...)` scans.  C 6136's `int x UNINIT` is `= {0,}` in
  conf_post.h 369, so the port zeroes it, and C 6139's bare `alloca` is the stack pool (P-05).
- **Carried as C's own text**: C 5157-5159's commented-out `DebPrint` preamble, C 5490-5499's
  `#if 0`/`#else` pair (the `#else` is what compiles), C 5809-5817's `#if 0` block and C 6113-6125's
  dead `#if 0` with the `#ifdef WM_TOUCHMOVE`/`#else` pair whose two labels are the same number
  (this file's `WM_TOUCHMOVE` is 576 and `<winuser.h>`'s `WM_TOUCH` is 0x0240 = 576), so the live
  label is the file's own name.  C 6105-6111's `#if HAVE_W32NOTIFY` arm carries `have_w32notify`.
- **The names this slice adds stand at the use site** (P-88 (1)), the SDK records UNVERIFIED: the
  twenty-two `WM_*` values the file does not already carry, `SPI_SETWHEELSCROLLLINES`, the three
  `SIZE_*`, `GetWindowDC`/`GetClipBox`/`MultiByteToWideChar`/`IsDBCSLeadByteEx`, and the six
  reference functions no mirror in this unit's graph declares (`store_frame_param`,
  `frame_ancestor_p`, `cancel_mouse_face`, `gui_redo_mouse_highlight`, `gen_help_event`,
  `w32_process_dnd_data`, `expose_frame`, `clear_mouse_face`, `change_frame_size`,
  `get_frame_param`, `gui_set_fullscreen`, `note_mouse_highlight`).  `countof` is reached the way C
  23's `lisp.h` reaches it (`include!("../lib/include/stdcountof.rs")`, the `src/cm.rs` 33 shape).
- **Three event kinds the mirror does not carry** are declared here with the value their own
  configuration gives (`LANGUAGE_CHANGE_EVENT` 8, `END_SESSION_EVENT` 14, `MULTIMEDIA_KEY_EVENT` 28)
  - the `dbusbind.rs` 572 `DBUS_EVENT` shape, owed below.
- **Two calls name functions whose definitions are later C ranges**: `w32fullscreen_hook`
  (C 5774/6064; defined at C 7067-7146) and `w32_raise_frame` (C 6405; defined at C 7407-7470), both
  still uncarried.  Rust resolves items file-wide, so they bind once those slices land - the
  slice-9 -> slice-15 pair (`w32_mouse_position`'s two report calls) is the same shape.  **They are
  the only names in this slice with no declaration** (the mechanical sweep below).
- **The arm reading cannot see this class, and that is now measured.**  In the base configuration
  the crate's *early* resolution is already failed (two `E0428`s, the frame flag-run
  `compile_error!` and four `E0425`s for SDK types), so value-name resolution is skipped: three
  injections into a `/tmp` copy of the materialized graph - a call to a name defined nowhere, a
  four-argument call to `note_mouse_highlight`, and deleting a statement's `;` - each produced **no
  row**, while a *type* error in the same block did (the `src/w32fns.rs` lane's clean harness, where
  one injected name gave exactly one row, is what an error-free graph gives).  The lane's "0
  anchored" reading therefore means **0 type errors**, and the slice's name surface is verified
  mechanically instead: every function/macro name called in C 5130-6458 has a declaration in the
  unit or its include graph, except the two above (the C 3445-3624 sweep's method, re-run over this
  slice's text).
- **Two mirror fixes this slice forced** (both recorded in their own ledgers): `include/w32term.rs`
  gains C 397's `asked_for_visible` as the third packed bit of `w32_output`'s `bits` word (with its
  two `w32_output_bits!` arms - `docs/ports/include-w32term.md`), and `include/frame.rs`'s
  `tool_bar_window`/`desired_tool_bar_string`/`current_tool_bar_string`/`last_tool_bar_item` lose
  the `not (have_ntgui)` clause their C 261/301 guards never had (`docs/ports/include-frame.md`).
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text** (the same
     twenty-two as slices 1-15; the seven `E0609` rows the first draft had were the frame mirror's
     guard, above).
  3. `extract-abi.py --check` -> **46 -> 45 rows** (`missing function` 42 -> **41**: this slice's one
     definition; `missing file-scope object` stays 1, `LOAD_PROC` stays) and the line-count row
     improves from 11.4% to **7.6%** (9,057 vs 8,414).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 17 (C 6460-6752)

- **What it is.**  The Cursor group: `w32_clip_to_row` (C 6473-6489), `w32_draw_hollow_cursor`
  (C 6494-6533), `w32_draw_bar_cursor` (C 6543-6626, with its image arm and the two
  `BAR_CURSOR`/`HBAR_CURSOR` branches), `w32_define_frame_cursor` (C 6631-6635),
  `w32_clear_frame_area` (C 6640-6648) and `w32_draw_window_cursor` (C 6650-6752, the RIF entry
  with its system-caret block and the five-label `switch (cursor_type)`).  Port: 9,057 ->
  **9,457 lines** (+400).  The table's last row splits at C 6752; the next live definition is
  C 6758's `w32_bitmap_icon` (C 6755-6756's "Icons" banner).
- **The six definitions retire six `missing function` rows** (41 -> 35).
- **The five prototypes no mirror carries stand at the use site** (P-88 (1), the `src/xterm.rs`
  38828-38833 shape): `get_phys_cursor_glyph`, `get_phys_cursor_geometry`,
  `draw_phys_cursor_glyph`, `erase_phys_cursor` (dispextern.h 3649/3652/3654 and the block
  around 3688) and `draw_fringe_bitmap` (dispextern.h 3703), plus `<wingdi.h>`'s `FrameRect` and
  `<imm.h>`'s `WM_IME_STARTCOMPOSITION`.  The SDK records are UNVERIFIED.
- **`FACE_FROM_ID` is dispextern.h's second `INLINE` face accessor**, and the port's live form is
  one copy per including unit (the `src/xterm.rs` 38843 block): this slice adds the unit's copy
  beside its own `FACE_FROM_ID_OR_NULL` (C 868), with C's `eassert` kept.
- **C's uninitialised out-parameters are zeroed** (PORTING.md 1.5): C 6478's three (`window_box`),
  C 6500's three and C 6605's three (`get_phys_cursor_geometry`) - the first draft's nine `E0381`
  rows were exactly this, and C 6719's `used[TEXT_AREA]` needed its `i16` narrowed to `c_int`.
- **C's chained store** `cursor_type = w->phys_cursor_type = NO_CURSOR;` (C 6669) is C's own two
  statements, the inner one first.
- **`window_bits!` gained C 418's `phys_cursor_on_p`** (read and write arms, the mirror's
  `WINDOW_PHYS_CURSOR_ON_P_SHIFT`): C 6677 writes it and C's other reads of the member are in
  later slices.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text**.
  3. `extract-abi.py --check` -> **45 -> 44 rows** (`missing function` 41 -> **35**).  The
     line-count row now reads **12.4%** (`9,457` vs `8,414`): the Rust file has passed the C file,
     so this row stops shrinking and its ratio grows from here (7.6% at slice 16, when the port
     was still the shorter text).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 18 (C 6754-6906)

- **What it is.**  The Icons block and the font entry: `w32_bitmap_icon` (C 6758-6816, the four-way
  icon choice with its two `LoadImage` calls and six `IDI_*` symbols) and `w32_new_font`
  (C 6846-6891, the fontset/font stores, `get_font_ascent_descent`, the scroll-bar column
  arithmetic and `adjust_frame_size`), with C 6818-6841's "Handling X errors" and C 6894-6906's
  "TODO: W32 Input Methods" banner-plus-list blocks carried as C's own text.  Port: 9,457 ->
  **9,740 lines** (+283).  The table's last row splits at C 6906; the next live definition is
  C 6912's `w32_calc_absolute_position`.
- **The two definitions retire two `missing function` rows** (35 -> 33).
- **The SDK surface stands at the use site** (P-88 (1)), the records UNVERIFIED: `<winuser.h>`'s
  `LoadIcon`/`LoadImage` with `IMAGE_ICON`/`LR_DEFAULTSIZE`/`LR_LOADFROMFILE`/`SM_CXSMICON`/
  `SM_CYSMICON`/`WM_SETICON`/`ICON_SMALL`/`ICON_BIG` and the six `IDI_*` values (C's
  `MAKEINTRESOURCE` is the unit's own macro copy, the `src/w32fns.rs` 492 shape), `frame.h` 1840's
  `EMACS_CLASS` (the port's `&CStr` string-constant shape), `dispextern.h` 3631's
  `get_font_ascent_descent` and `frame.h` 1511-1512's `adjust_frame_size` (defined at
  `src/frame.rs` 1221).
- **Four `frame.h` accessors this slice writes get the two-arm macro form** (the
  `src/androidterm.rs` 2739-2757 shape, since `include/frame.rs` carries the reader alone):
  `FRAME_TAB_BAR_HEIGHT`, `FRAME_COLUMN_WIDTH`, `FRAME_CONFIG_SCROLL_BAR_WIDTH` and
  `FRAME_CONFIG_SCROLL_BAR_COLS`.
- **Three of `include/w32term.rs`'s own accessors gained their write arms** (`FRAME_FONT` C 6860,
  `FRAME_FONTSET` C 6854, `FRAME_BASELINE_OFFSET` C 6861) - the same shape, recorded in
  `docs/ports/include-w32term.md`.
- **C's two chained assignments are C's own two stores**: C 6862's
  `FRAME_COLUMN_WIDTH (f) = unit = font->average_width;` (the inner store first, then the outer
  with `unit`) and, in the previous slice, C 6669's cursor-type pair.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text**.
  3. `extract-abi.py --check` -> **44 -> 43 rows** (`missing function` 35 -> **33**), the
     line-count row at **15.8%** (`9,740` vs `8,414`; it grows from here, as slice 17's entry
     records).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 19 (C 6908-7146)

- **What it is.**  The position trio: `w32_calc_absolute_position` (C 6912-7021, the placement
  probe, the multi-monitor left/top scan and the two negative-coordinate branches),
  `w32_set_offset` (C 7029-7065, the gravity block, `w32_wm_set_size_hint` and the two
  `my_set_window_pos` calls) and **`w32fullscreen_hook`** (C 7067-7146, the five fullscreen arms
  with their `ShowWindow`/`SetWindowLong`/`SetWindowPos`/`change_frame_size` quartet and the
  `FULLSCREEN_WAIT` pair).  Port: 9,740 -> **10,070 lines** (+330).  The table's last row splits at
  C 7146; the next live definition is C 7152's `w32_set_window_size`.
- **`w32fullscreen_hook` is the first of slice 16's two owed calls discharged**: C 5774/6064 called
  it and this slice defines it, so the name now resolves inside the file (the `w32_raise_frame`
  call at C 6405 keeps its owed note until C 7407-7470 lands).
- **The three definitions retire three `missing function` rows** (33 -> 30).
- **The SDK surface stands at the use site** (P-88 (1)), UNVERIFIED: `GetWindowPlacement`/
  `SetWindowPlacement`/`ShowWindow`, `GWL_STYLE` (-16), `HWND_TOP`, `WS_OVERLAPPEDWINDOW`
  (0x00cf0000), `SW_SHOWNORMAL`/`SW_MAXIMIZE`, the four `SWP_*` values this slice needs (the
  `SWP_FRAMECHANGED` it also uses was already here from slice 8) and dispnew.c's
  `do_pending_window_change` (defined at `src/dispnew.rs` 7434).  `XNegative`/`YNegative`
  (`include/w32gui.rs` 146-147) and `FRAME_UNDECORATED` (`include/frame.rs`) were already in the
  graph.
- **`FRAME_PREV_FSMODE` gained its write arm** in `include/w32term.rs` (C 7093 - the fourth
  accessor of that family, the slice-18 shape; recorded in `docs/ports/include-w32term.md`).
- **`w32_wm_set_size_hint` (C 7049) is the second call of the "defined later in this file" class**
  (defined at C 7477, still uncarried) - Rust resolves it once that slice lands, exactly as
  `w32_raise_frame`/`w32fullscreen_hook` did before this slice.
- **C's two `{ 0 }` initialisers are explicit zero fills** (C 6926-6927, PORTING.md 1.5), and the
  free-standing `rect` of C 7074 is zeroed for the same reason.  C's `register` keywords (C 7030)
  have no Rust counterpart; the parameter names are kept (PORTING.md 0.1).
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text** (the
     first draft's two `E0308` rows were `FRAME_UNDECORATED`'s `c_int` in a `!` test).
  3. `extract-abi.py --check` -> **43 -> 42 rows** (`missing function` 33 -> **30**), the
     line-count row at **19.7%** (`10,070` vs `8,414`).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 20 (C 7148-7502)

- **What it is.**  The window-size, mouse-warp and Z-order group: `w32_set_window_size`
  (C 7152-7256), `w32_set_window_size_and_position` (C 7261-7335, the same menu-bar probe with the
  position fill), `frame_set_mouse_pixel_position` (C 7339-7365, C's one non-`static` definition
  here), `w32_get_focus_frame` (C 7367-7379), `w32_focus_frame` (C 7388-7405, both `#if 0` blocks
  carried as C's text), **`w32_raise_frame`** (C 7408-7467), `w32_lower_frame` (C 7470-7479) and
  `w32_frame_raise_lower` (C 7481-7491).  Port: 10,070 -> **10,565 lines** (+495).  The table's
  last row splits at C 7502; the next live definition is C 7504's `w32_make_frame_visible`.
- **Slice 16's second and last owed call is discharged**: C 6405 called `w32_raise_frame` and this
  slice defines it - the two calls that reading could not see (the arm sweep is blind to value
  names while the crate has early errors, `docs/status.md` 4.320 ④) both resolve inside the file
  now.
- **The eight definitions retire eight `missing function` rows** (30 -> 22), the largest single
  drop of this lane.
- **`frame_set_mouse_pixel_position` is non-`static`**, so the port's definition carries
  `#[unsafe(no_mangle)] pub unsafe extern "C" fn` (P-52's rule, the `w32_query_frame_background_color`
  shape).
- **The SDK surface stands at the use site** (P-88 (1)), UNVERIFIED, with `MENUBARINFO` declared
  the way `src/w32fns.rs` 14014 declares it (C's two `BOOL : 1` members as one `bits: DWORD`):
  `GetMenuBarInfo`, `AdjustWindowRect`, `GetWindowRect`, `ClientToScreen`, `SetCursorPos`,
  `GetForegroundWindow`, the three `DeferWindowPos` entries with `HDWP`, plus `SWP_NOMOVE`,
  `SM_CYMENUSIZE` (51) and the two `SPI_*MOUSETRAILS` values - the last three following the records
  `src/w32fns.rs` 6986/13990/13992 already carry.
- **`frame_bits!` gained C 494's `after_make_frame`** (C's `bool_bf : 1`; arm A puts it in unit 2
  at bit 14, so the read goes through the mirror's `frame_field` - the `want_fullscreen!` shape).
- **C's chained stores are C's own two statements** (C 7167 and C 7274's
  `info.rcBar.top = info.rcBar.bottom = 0;`), and C's uninitialised locals are zeroed
  (PORTING.md 1.5: C 7157-7158, 7202, 7264-7265, 7344-7345).
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **22 errors, 0 anchored in this slice's text** (the
     first draft's `E0618` was `Vw32_grab_focus_on_raise`, a global rather than a getter).
  3. `extract-abi.py --check` -> **42 -> 34 rows** (`missing function` 30 -> **22**), the
     line-count row at **25.6%** (`10,565` vs `8,414`).
  4. The ten gates: all `rc=0`, 0 FAIL.

### Slice 21 (C 7503-7734)

- **What it is.**  The visibility and resource group: `w32_make_frame_visible` (C 7503-7599, the
  work-area adjustment, the three-way `ShowWindow` flag choice and the wait loop),
  `w32_make_frame_invisible` (C 7605-7625), `w32_make_frame_visible_invisible` (C 7627-7634),
  `w32_iconify_frame` (C 7638-7660), `w32_free_frame_resources` (C 7665-7699),
  `w32_destroy_window` (C 7702-7710) and **`w32_wm_set_size_hint`** (C 7720-7734).  Port:
  10,565 -> **10,916 lines** (+348).  The table's last row splits at C 7734; the next live
  definition is C 7746's `w32_check_font` (C 7736-7741's Fonts banner and its `ifdef GLYPH_DEBUG`).
- **Slice 19's owed call is discharged**: C 7049/7190/7297 called `w32_wm_set_size_hint` and this
  slice defines it.  **No owed call remains in this lane.**
- **The seven definitions retire seven `missing function` rows** (22 -> 15).
- **Six of C's seven items here are non-`static`**, so their definitions carry
  `#[unsafe(no_mangle)] pub unsafe extern "C" fn` (P-52); `w32_make_frame_visible_invisible` and
  `w32_destroy_window` are C's own `static` and stay plain.
- **frame.h 1895-1905's `INLINE` `gui_set_bitmap_icon` is carried as one copy per including unit**
  (C 7508/7650 call it; `src/xterm.rs` 18186 records the bare call as its owed mirror row, this
  unit carries the body over `assq_no_quit`/`Qicon_type`/`FRAME_TERMINAL`'s
  `set_bitmap_icon_hook`).
- **The SDK surface stands at the use site** (P-88 (1)), UNVERIFIED: `SPI_GETWORKAREA`,
  `SW_RESTORE`, `SW_SHOWNOACTIVATE`, `WM_SYSCOMMAND`, `SC_MINIMIZE`, plus `free_frame_faces`
  (dispextern.h 3822), `free_frame_menubar` (frame.h 1883) and keyboard.h 489/510's two polling
  entries.
- **`asked_for_visible` gets its write** (C 7538) - the bit slice 16 packed into `w32_output`'s
  `bits` word, read at C 7517 and C 5190.
- **Nine prototypes left the mirror's crate-root blocks for `w32term_fn_decls`** (a glob import is
  shadowed by a local definition, P-21/P-39): C 251-258's six - `w32_query_frame_background_color`
  and slice 21's five new definitions - and C 802-805's three (`w32_image_rotations_p`,
  `w32_initialize_display_info`, `initialize_w32_display`).  **This closes the lane's two standing
  `E0428` rows** (`Owed 4` since slice 3) and pre-empts the two `initialize_*` collisions the
  file's tail would have hit; the arm reading drops from **22 to 20** - its lowest this lane has
  recorded.  The mirror ledger records the move.
- **Readings.**
  1. `build-tu src/w32term.c` (frozen) -> exit 0, **336 bytes**.
  2. The arm sweep (base configuration) -> **20 errors, 0 anchored in this slice's text** (the
     first draft's `E0308` was `mouse_face_mouse_frame`'s `*mut c_void` in the C 7695 comparison,
     the slice-16 cast).
  3. `extract-abi.py --check` -> **34 -> 27 rows** (`missing function` 22 -> **15**), the
     line-count row at **29.7%** (`10,916` vs `8,414`).
  4. The ten gates: all `rc=0`, 0 FAIL.
