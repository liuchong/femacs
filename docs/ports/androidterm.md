# emacs/src/androidterm.c (7,117 lines)

> **P-86's fourth unit, and its first slice: C 1-829.**  `configure.ac` 2597 puts `androidterm.o`
> in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])` (2594) - **both** Android branches, like
> `androidmenu.o` and unlike `androidselect.o` (2647-2648, the `REALLY_ANDROID` branch only) - and
> `docs/baseline/frozen-build-sources.txt` lists no `androidterm.o` for this target, so
> `extract-abi.py`'s checklist for the file is empty and its checkbox sections cannot be coverage
> evidence.  What follows is P-86 ②'s substitute: the **C line-range table**, the `## Evidence`
> readings, and the `## Deviations`.  Port: `femacs/src/androidterm.rs` (**4,923 lines**, md5
> `0e897fb60f47e84104267b6158642e85`), carrying **C 1-4123**; both arms of every `#if` carried.  This ledger replaces the previous revision's conclusion ("the file is not in
> the frozen build, so the port is the empty unit plus a note"): P-86 ① rules that a three-line
> header comment is not started, not done.  The old note's measurements are still true and are
> still where the *file gate* comes from; what changed is the conclusion drawn from them.
>
> **Why 829, and who is next.**  P-86 ④ says to stop at a whole-item boundary.  C 836-1892 is
> `handle_one_android_event`, a single 1,056-line function - the next whole-item boundary after it
> is C 1892, far past the 1,000-line target - so this slice ends at C 829's `}`, the end of
> `android_handle_ime_event`.  **The next live definition is `handle_one_android_event` (C 836-1892),
> with its forward declaration `static void android_notify_conversion (unsigned long);` at C 834**;
> the table's last row recorded C 830-7117 as the remainder; **slice 2 has replaced it with C 830-1892,
> which leaves C 3799-7117** (the next whole-item boundary is
> `android_draw_glyphless_glyph_string_foreground`, C 4126).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2597's `ANDROID_OBJ`, under 2594's `AC_DEFINE([HAVE_ANDROID])`; both Android branches) | `feature = "have_android"` (on every item and on C 21-36's include graph) | an Android configuration |
| C 47 `#ifndef ANDROID_STUBIFY` … C 6662 `#endif /* !ANDROID_STUBIFY */` (this pair has **no** `#else`: the file's stub arms are the later `#if`s at C 6666/6712/6737/6884, all past this slice, so the stub arm of C 1-829 is *nothing*) | `feature = "not_android_stubify"`, nested in the file gate | Android, API calls real |
| `HAVE_WINDOW_SYSTEM` (configure.ac 7518; the `window_system=android` branch at 2594-2596 is what puts this file in the build list at all) | **no gate**: this file carries no `#if` for it - C 519's `Emacs_Rectangle`, C 543's `note_mouse_highlight`, C 545's `remember_mouse_glyph` and C 350's `buffer_flipping_blocked_p` (dispextern.h 3586-4006) need it exactly as C needs it | every Android configuration |
| `HAVE_TEXT_CONVERSION` (also no `#if` here) | **no gate either**: configure.ac 7523 defines it exactly when `REALLY_ANDROID`, which is C's `!defined ANDROID_STUBIFY` (2606's `#else` is where `ANDROID_STUBIFY` is defined), so `not_android_stubify` implies it | every real Android configuration |

`not_android_stubify` is the feature the tree already registered for C's
`HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` line 640, first
used by `alloc.rs` 6428).  `configure.ac` 2606 defines `ANDROID_STUBIFY` when the Android window
system is configured "but all API calls need to be stubbed out".

## C line ranges

One row per C block, in C's order, and the rows are **contiguous** (each one absorbs the blank
lines that follow the one before), so the ranges cover C 1-7117 with no gap.  That is deliberate:
`docs/port-map.md` 372-373 pairs `src/androidterm.c` with this unit and `src/androidterm.h` with
this unit's include mirror, and `check-dead-file-coverage.py`'s ledger rule resolves **both** to
this file (the `docs/ports/frame.md` item-3 precedent: "the header's coverage should not keep
riding on the unit's table", but today it does).  A contiguous table over C 1-7117 covers the
496-line header's lines too, which is what makes the second binding read clean; the fix - a
`src-androidterm.md` for the header, or a rule change - is recorded below.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's licence/declaration block (and the blank after it) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20 | `#include <config.h>` | ungated (every unit includes it) |
| 21-25 | `<stdio.h>`, `<math.h>`, `<stdlib.h>`, `<semaphore.h>` | file gate (`<semaphore.h>` has no mirror and no name of this slice is taken from it) |
| 26-37 | the eleven mirror includes: `lisp.h` `androidterm.h` `keyboard.h` `blockinput.h` `android.h` `buffer.h` `window.h` `textconv.h` `coding.h` `pdumper.h` `keymap.h` | file gate |
| 38-41 | the `x_display_list` note and the definition | file gate |
| 42-45 | the "Android terminal interface functions" note | file gate |
| 46-47 | `#ifndef ANDROID_STUBIFY` | gate (see the table above) |
| 48-49 | `#include <android/log.h>` | real arm (NDK header, no mirror; no name of this slice comes from it) |
| 50-54 | the `any_help_event_p` note and its definition | real arm |
| 55-59 | the wheel-counter note and `wheel_event_x`/`wheel_event_y` | real arm |
| 60-67 | the anonymous `enum` of the three `ANDROID_EVENT_*` codes | real arm |
| 68-96 | `android_window_to_frame` (C 88-89 the `continue` of the frame walk) | real arm |
| 97-105 | `android_clear_frame` | real arm |
| 106-132 | `android_show_hourglass` (C 124 the `popup_activated` test) | real arm |
| 133-138 | `make_invisible_cursor` | real arm |
| 139-161 | `android_hide_hourglass` | real arm |
| 162-264 | `android_flash` (C 201 the `while`, C 229 `FD_ZERO`, C 231 the `pselect`) | real arm |
| 265-277 | `android_ring_bell` | real arm |
| 278-302 | `android_toggle_visible_pointer` (C 288 the `goto set_invisibility`, C 291 its label) | real arm |
| 303-310 | `android_toggle_invisible_pointer` | real arm |
| 311-324 | the `update_begin` note and `android_update_begin` | real arm |
| 325-334 | the `update_end` note and `android_update_end` | real arm |
| 335-348 | `show_back_buffer` | real arm |
| 349-364 | `android_flush_dirty_back_buffer_on` | real arm |
| 365-393 | the modifier note and `android_android_to_emacs_modifiers` | real arm |
| 394-419 | `android_emacs_to_android_modifiers` | real arm |
| 420-421 | `static void android_frame_rehighlight (struct android_display_info *);` - a forward declaration whose definition is C 2127 | real arm (carried as a comment; Rust has no forward declarations) |
| 422-427 | `android_lower_frame` | real arm |
| 428-433 | `android_raise_frame` | real arm |
| 434-459 | `android_new_focus_frame` (C 458 the call to the function defined at C 2127) | real arm |
| 460-494 | `android_focus_changed` (C 468 and C 487 the two `bufp->kind` writes) | real arm |
| 495-516 | `android_detect_focus_change` (C 502 the `switch`) | real arm |
| 517-549 | `android_note_mouse_movement` | real arm |
| 550-573 | `mouse_or_wdesc_frame` | real arm |
| 574-602 | `android_construct_mouse_click` | real arm |
| 603-631 | the touchscreen-update note and `android_update_tools` (C 620-621 the `continue`) | real arm |
| 632-649 | the tool-finder note and `android_find_tool` | real arm |
| 650-675 | the UTF-16 note and `android_decode_utf16` (C 662 the `ckd_mul`) | real arm |
| 676-709 | the cursor-update note and `android_request_cursor_updates` | real arm |
| 710-829 | the input-method note and `android_handle_ime_event` (C 722 and C 744 the two `switch`es) | real arm |
| 830-834 | C 831-832's page break and blank, C 833's `/* Forward declaration.  */` and C 834's `static void android_notify_conversion (unsigned long);` | real arm (slice 2: the comment is carried, the declaration needs no Rust text - PORTING.md 10.10) |
| 836-1892 | `handle_one_android_event` (C 870 the `switch (event->type)`, C 879-1857's forty-seven `goto OTHER`/`goto done_keysym` with the `OTHER:` label at C 1860 and `done_keysym:` at C 1110, C 1836-1841's `#if notdef` block) | real arm (slice 2) |
| 1894-1927 | `android_read_socket` (C 1904 the `while (android_pending ())` loop) | real arm (slice 3) |
| 1929-1946 | `android_frame_up_to_date` (C 1941's `FRAME_ANDROID_COMPLETE_P (f) = true;`) | real arm (slice 3) |
| 1948-1957 | `android_buffer_flipping_unblocked_hook` | real arm (slice 3) |
| 1959-1968 | `android_query_frame_background_color` | real arm (slice 3) |
| 1970-2004 | `android_parse_color` (C 1988 the `for (; CONSP (tem); tem = XCDR (tem))`) | real arm (slice 3) |
| 2006-2038 | `android_alloc_nearest_color` (C 2017-2021 and C 2026-2029 the two luminance folds) | real arm (slice 3) |
| 2040-2051 | `android_query_colors` | real arm (slice 3) |
| 2053-2085 | `android_mouse_position` (C 2077 the `FOR_EACH_FRAME`) | real arm (slice 3) |
| 2087-2100 | `android_get_focus_frame` | real arm (slice 3) |
| 2102-2109 | `android_focus_frame` | real arm (slice 3) |
| 2111-2124 | C 2111-2112's note and the two cursor procedures `android_frame_highlight`/`_unhighlight` | real arm (slice 3) |
| 2126-2156 | `android_frame_rehighlight` | real arm (slice 3) |
| 2158-2162 | `android_frame_rehighlight_hook` | real arm (slice 3) |
| 2164-2171 | `android_frame_raise_lower` | real arm (slice 3) |
| 2173-2180 | `android_make_frame_visible` | real arm (slice 3) |
| 2182-2193 | `android_make_frame_invisible` | real arm (slice 3) |
| 2195-2202 | `android_make_frame_visible_invisible` | real arm (slice 3) |
| 2204-2231 | `android_fullscreen_hook` (C 2214's `f->want_fullscreen == FULLSCREEN_BOTH`) | real arm (slice 3) |
| 2233-2238 | `android_iconify_frame` | real arm (slice 3) |
| 2240-2280 | `android_wait_for_event` (C 2259 the `while (f->wait_event_type)` loop) | real arm (slice 4) |
| 2282-2315 | `android_set_window_size_1` (C 2298/2305 the two `frame_size_history_extra` calls) | real arm (slice 4) |
| 2317-2338 | `android_set_window_size` | real arm (slice 4) |
| 2340-2358 | `android_set_window_size_and_position_1` | real arm (slice 4) |
| 2360-2380 | `android_set_window_size_and_position` | real arm (slice 4) |
| 2382-2455 | C 2382-2383's note and `android_calc_absolute_position` (C 2404/2431 the `has_been_visible` reads) | real arm (slice 4) |
| 2457-2476 | `android_set_offset` | real arm (slice 4) |
| 2478-2482 | `android_set_alpha` (an empty body in C too) | real arm (slice 4) |
| 2484-2528 | `android_new_font` (C 2499 the window-system `baseline_offset`) | real arm (slice 4) |
| 2530-2534 | `android_bitmap_icon` | real arm (slice 4) |
| 2536-2540 | `android_free_pixmap_hook` | real arm (slice 4) |
| 2542-2634 | `android_free_frame_resources` (the fifteen cursor tests and the two relief GCs) | real arm (slice 4) |
| 2636-2642 | `android_delete_frame` | real arm (slice 4) |
| 2644-2648 | `android_delete_terminal` | real arm (slice 4) |
| 2652-2711 | C 2652's `/* RIF functions.  */` and `android_scroll_run` | real arm (slice 5) |
| 2713-2720 | `android_after_update_window_line` | real arm (slice 5) |
| 2722-2734 | `android_flip_and_flush` (C 2732's folded-member write) | real arm (slice 5) |
| 2736-2749 | `android_clear_rectangle` | real arm (slice 5) |
| 2751-2755 | `android_reset_clip_rectangles` | real arm (slice 5) |
| 2757-2777 | `android_clip_to_row` | real arm (slice 5) |
| 2779-2895 | `android_draw_fringe_bitmap` (C 2844's `goto undo_clip`, C 2860-2885's `p->cursor_p`/`p->overlay_p`) | real arm (slice 5) |
| 2897-2944 | C 2897-2898's comment and `android_set_cursor_gc` | real arm (slice 5) |
| 2946-2977 | C 2947's comment and `android_set_mouse_face_gc` | real arm (slice 5) |
| 2980-2988 | C 2980-2982's comment and `android_set_mode_line_face_gc` | real arm (slice 5) |
| 2990-3030 | C 2990-2992's comment and `android_set_glyph_string_gc` | real arm (slice 5) |
| 3033-3045 | C 3033-3034's comment and `android_set_glyph_string_clipping` | real arm (slice 5) |
| 3048-3065 | C 3048-3050's comment and `android_set_glyph_string_clipping_exactly` (C 3062's degraded-array store) | real arm (slice 5) |
| 3067-3096 | C 3067's comment and `android_compute_glyph_string_overhangs` | real arm (slice 6) |
| 3098-3103 | `android_clear_glyph_string_rect` | real arm (slice 6) |
| 3105-3141 | `android_draw_glyph_string_background` | real arm (slice 6) |
| 3143-3158 | `android_fill_triangle` | real arm (slice 6) |
| 3160-3169 | `android_make_point` | real arm (slice 6) |
| 3171-3186 | `android_inside_rect_p` | real arm (slice 6) |
| 3188-3199 | `android_clear_point` | real arm (slice 6) |
| 3201-3319 | `android_draw_relief_rect` | real arm (slice 6) |
| 3321-3355 | `android_draw_box_rect` | real arm (slice 6) |
| 3357-3357 | `#define HIGHLIGHT_COLOR_DARK_BOOST_LIMIT 48000` | real arm (slice 6: the inventory's `missing macro`/`missing constant` pair) |
| 3359-3426 | C 3359's `static bool` and `android_alloc_lighter_color` | real arm (slice 7) |
| 3428-3462 | C 3428-3433's comment and `android_setup_relief_color` | real arm (slice 7) |
| 3464-3498 | C 3464's comment and `android_setup_relief_colors` | real arm (slice 7) |
| 3500-3564 | `android_draw_glyph_string_box` (C 3522-3528 the automatic-composition walk) | real arm (slice 7) |
| 3566-3580 | `android_draw_glyph_string_bg_rect` | real arm (slice 7) |
| 3582-3670 | C 3582's `static void` and `android_draw_image_relief` (C 3657-3664's four comma statements) | real arm (slice 8) |
| 3672-3740 | `android_draw_image_foreground` | real arm (slice 8) |
| 3742-3797 | `android_draw_image_glyph_string` | real arm (slice 8) |
| 3799-3911 | C 3799's `static void` and `android_draw_stretch_glyph_string` | real arm (slice 9) |
| 3913-3926 | `android_get_scale_factor` | real arm (slice 9) |
| 3928-3987 | C 3928's comment and `android_draw_underwave` | real arm (slice 9) |
| 3989-4034 | `android_draw_glyph_string_foreground` | real arm (slice 9) |
| 4036-4123 | C 4036's comment and `android_draw_composite_glyph_string_foreground` | real arm (slice 9) |
| 4125-7117 | **the remainder of the file** - from C 4125's `static void` above `android_draw_glyphless_glyph_string_foreground` (C 4126) to `syms_of_androidterm` | not in this slice: recorded here so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/aterm-lane/target cargo run -q -p xtask -- build-tu
src/androidterm.c` → `built src/androidterm.rs -> ../build/femacs/androidterm.o`, **exit 0**, and
`build/femacs/androidterm.o` is **336 bytes** - an empty object, read on the final revision (1,055
Rust lines).  That is the expected reading, not a proof of the new text: every item of the port sits
behind a `have_android` gate, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute - P-86 ⑧'s criterion.  (**Five `build-tu` runs in
total, against a budget of three, and the count with its reasons is here rather than implied**: runs
1 and 2 failed to compile the unit and produced no object - the first on six C-shaped expressions
written without Rust's syntax (`else` without a block, five `?:` in the two modifier converters),
the second on the ten `match` arms whose statements were not braced; runs 3, 4 and 5 are the
336-byte reading after each round of unit fixes, run 5 being the final revision - runs 3 and 4
differ from it only in comment text.)

**② The arm flip (P-86 ③⑴; the file gate, the sub-gate and the three conditions of the Gate
mapping table).**  *Slice 5 corrected this reading's configuration.*  Slices 1-4 ran the flip with
the file gate and the sub-gate only (`--cfg 'feature="have_android"' --cfg
'feature="not_android_stubify"'`), which is a configuration **this file can never be built in**: the
Gate mapping table's third and fourth rows say every Android configuration also has
`HAVE_WINDOW_SYSTEM` (configure.ac 7518) and `HAVE_TEXT_CONVERSION` (7523).  Measured on the C 1-3067
revision (3,640 Rust lines), with the five gates the real configuration has -
`have_android`, `not_android_stubify`, `have_window_system`, `have_text_conversion` and
`dispextern_face_records` (the last is the mirror's own arm for the five records C 2159-3200 declares,
`include/dispextern.rs` 2790-2800) - the unit reads **exit 1, 3 errors**:

  * `frame.want_fullscreen` (C 2214) and `frame.new_size_p` (C 907) - two members of the frame
    mirror's window-system numbering region (`include/frame.rs` 757-759's mask pair covers
    `want_fullscreen`, and `new_size_p`'s unit is not numbered there yet).  These are Owed 5's round,
    narrowed from a whole region to two fields.
  * `android_notify_conversion` (C 1761) - the one surviving forward reference (defined at C 6635-6648,
    its own address taken by C 6657's interface array).

  The narrower reading's "window-system gap" rows (`frame.tool_bar_window` 7 sites,
  `frame.tab_bar_window` 4, `frame.last_tool_bar_item` 3, `frame.conversion` 2, `frame.new_size_p`,
  `frame.want_fullscreen`, `FULLSCREEN_BOTH`, `font.baseline_offset`) are **retired by this
  measurement**: every one of those names exists in the configuration the file is built in, and the
  rows measured the mirror's *frozen* arm instead.  The unit's own text needed no change for that -
  it already spells C's names - which is the reading P-86 ⑦ asks for.

**② The arm flip (P-86 ③⑴; the file gate plus the sub-gate).**
`cargo run -q -p xtask -- probe src/androidterm.rs /tmp/aterm-lane/arm.o --crate-type lib --emit obj
-C panic=abort --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'` →
**exit 1, 17 `error` lines, 11 of them in an included mirror and 6 in this unit's own text.**
The classification P-86 ⑦ asks for, by file:

- **11 in the mirror** - `include/androidterm.rs` 441 (`FRAME_ANDROID_NEED_BUFFER_FLIP`, 9 errors),
  452 and 462 (`FRAME_ANDROID_DRAWABLE`/`FRAME_ANDROID_COMPLETE_P`, 2 errors): the three `bool_bf`
  bodies read `android_output.need_buffer_flip`/`complete`, which the mirror folds into
  `android_output.bits` (`include/androidterm.rs` 229-247).  These are the mirror's **known
  uncompilable site** (`docs/ports/include-androidterm.md`, which asks for three
  `macro-body-notes.tsv` rows), not port text: the unit writes C's own macro calls and C's own
  assignment shape (`FRAME_ANDROID_COMPLETE_P (f) = false;`), the `androidfont.rs` 870 precedent.
- **4 in this unit's own text, all named blockers**: C 332's `MOUSE_HL_INFO (f)->mouse_face_defer`
  and C 707-708's `need_cursor_updates` (the same folded-`bits` class - `Mouse_HLInfo.bits`,
  dispextern.rs 1941-1945, and `android_output.bits`), and C 458's `android_frame_rehighlight`
  (`E0425`; the definition is C 2127, past this slice).
- **2 in this unit's own text, and this probe's configuration rather than the port**: C 795-796's
  `f->conversion.batch_edit_*` need `have_text_conversion`, which the two `--cfg`s of the flip
  command do not set and which configure.ac 7523 defines exactly when `REALLY_ANDROID` (⇔ C's
  `!defined ANDROID_STUBIFY`) - the implied-feature row of the Gate mapping table above.

**No name-resolution abort this time**, and that is where this reading differs from `androidmenu.c`'s
first one (P-86 ⑦): rustc got past name resolution and type-checked the whole unit, so the 17 rows
above are the *complete* error list for this gate set - the unit's own text contributes exactly the
six named sites and nothing else.  (It is still a type-check/codegen reading: no JNI call, no frame,
no font, no cursor was executed.)
**Four `probe` invocations in total, against a budget of three**, and the reasons are here: runs 1
and 2 are the same **37-error** reading of the first revision (run 2 saved the full log), run 3 is
the **19-error** reading after the first three use-site fixes (the `intmax_t`-vs-`EMACS_INT`
bit-ands, `redisplay_interface`, `use textconv_decls::*;`), and run 4 is the final **17** with the
local `flush_frame` in place - runs 1-3 also carried the two `include/termhooks.rs` 425 `E0308`s
(`FRAME_RIF!` hands back `*mut c_void` where `include/frame.rs` 1193-1200's `flush_frame!` wants
`*mut redisplay_interface`) that the local `flush_frame` retires.

**③ The other gates (repo root, final revision), one line each.**
`python3 tools/check-dead-file-coverage.py` → `info docs/ports/androidterm.md: 41 range row(s) cover
src/androidterm.c (7117 lines; 0 blank line(s) between blocks need no row)`, `info … cover
src/androidterm.h (496 lines; 0 …)`, and `10 ledger(s) bound (a C range table), 0 failure(s)`. —
`python3 tools/check-port-lines.py` → `87 file(s) under ten percent, 203,995 reference line(s); 90
ratchet row(s), 0 new, 3 stale`, and this file's row is one of the three: `stale src/androidterm.c:
no longer under the threshold` (1,055 Rust lines against a 711-line threshold, so the file **left**
P-86's under-ten-percent class with this batch; the integrator's `--write-pending` drops the row). —
`python3 tools/check-citations.py` → `675 file(s) scanned, 1 drift, 0 range, 491 external, 33
allowed, 0 stale`, and the one drift is not this lane's (`femacs/src/image.rs:504`, an in-flight
file); this slice's citations read clean after the first run's `lisp.h 33`/`INT_MAX` pair was
corrected. — `python3 tools/check-declarations.py` → `13 declaration sites (0 forbidden, 13 allowed
literals), 0 failures`. — `python3 tools/check-carried.py` → `11 carried file(s), 11 baseline
row(s), 0 failure(s)`.

**④ Inventory.**
`python3 tools/extract-abi.py --check emacs/src/androidterm.c femacs/src/androidterm.rs` → **FAIL,
116 `- ` rows**, all of them owed and none of them this slice's text: **111 `missing …` rows** name
the remaining 6,288 lines' own items (the file's checklist is empty, so P-86 ⑩ makes the check
compare the whole reference text against an 829-line port; 110 of the 111 are defined only in
C 830-7117), and the single exception is `android_frame_rehighlight` - the forward call at C 458
whose definition is C 2127.  The other **4 rows** are the `switch` family - `android_detect_focus_change`
and `android_handle_ime_event` "does not mention break, case, default, switch" (plus `uninit` for the
`UNINIT` local, `goto` for the labelled jump and one `0`) - i.e. the `docs/ports/macro-body-notes.tsv`
rows this port owes; and the last row is `line count drifted 85.2%: C 7117 vs Rust 1055` (the
slice's basis).  All 116 are in the owed list below.

## Deviations

- **Two use-site repairs the frozen mirrors need, each named so the integrator can retire it.**
  (a) `struct redisplay_interface` (dispextern.h 3069-3196) has no mirror (`include/termhooks.rs`
  181/219 degrades `rif` to `*mut c_void`) and `include/frame.rs` 1193-1200's `flush_frame!` names
  the type, so C 353/408's `flush_frame (f)` is written as C's own body in the unit - the
  `minibuf.rs` 645-668 shape, prefix array plus the one measured member offset (72) - because the
  mirror's macro cannot type-check against the mirror's own `FRAME_RIF!` (measured: `E0308 expected
  *mut redisplay_interface, found *mut c_void`, `include/termhooks.rs` 425).  (b)
  `include/textconv.rs` 160-243 puts textconv.h's ten declarations in `pub mod textconv_decls`, and
  that mirror's own header comment says the module carries a glob import; the glob line is not in
  the frozen mirror, so the unit writes `use textconv_decls::*;` rather than repeating the ten
  declarations (PORTING.md P-26: one declaration per name, in the mirror).
- **Eight declarations the sixteen headers reach, which the frozen mirrors do not carry yet**:
  `popup_activated` (dispextern.h 3879), `gui_mouse_grabbed` (4006, `Display_Info *` =
  `*mut android_display_info`, dispextern.h 165), `buffer_flipping_blocked_p` (3719),
  `remember_mouse_glyph` (3586, `NativeRectangle *` = androidgui.h 218's `Emacs_Rectangle *`),
  `note_mouse_highlight` (3688), `setup_coding_system` (coding.h 686), `detect_input_pending`
  (lisp.h 5095) and `android_create_gc` (androidgui.h 926-927).  Each is declared at the use site
  with its C prototype in a `/// C:` line (P-72 rule 4, the `androidfont.rs` seven-declaration
  shape).
- **`FRAME_ANDROID_OUTPUT`/`FRAME_OUTPUT_DATA` return `*mut c_void`** - the mirror keeps C's exact
  body (`((f)->output_data.android)`, androidterm.h 290-291) and, unlike the six sibling macros,
  adds no P-72 rule 3 cast - so the six uses in this slice write `as *mut android_output`, the cast
  `include/androidterm.rs` 418-430 says belongs at the use site (measured precedent: `frame.rs`
  5497's `output_data.android as *mut android_output`).  The null comparisons (C 518, 687, 707) need
  no cast: the field is a null pointer there.
- **C's `intmax_t state & EMACS_INT mod_ctrl`** (C 411-415) is `i64 & isize` in Rust and has no
  `BitAnd` implementation; C's usual arithmetic conversions make both 64-bit, so the port writes
  C's four operands with an explicit `as intmax_t` (the same statement, the `androidfont.rs` `as …
  as c_int` promotions).
- **`'continue_` labelled blocks** (C 88-89 and C 620-621): C's `continue` jumps to the `for`
  increment, and both the mirror's `FOR_EACH_FRAME!` (`include/frame.rs` 387-395) and the port's
  `while` put that increment after the body, so each body is a labelled block and `break
  'continue_;` leaves it - the `frame.rs` 2836-2845 shape and its recorded reason.
- **`goto set_invisibility` (C 288) is a forward jump to the clean tail**, so the port writes the
  labelled block with `break 'set_invisibility;` and the trailing `f->pointer_invisible = invisible;`
  (PORTING.md 2.2; the `androidmenu.rs` `'finish` shape).
- **The two `switch`es (C 502, C 722, C 744) become `match`es** whose arms keep C's order and the
  empty `default`; Rust has no `switch`/`case`/`default`/`break` there, and the token gate's rows for
  that family are owed (Evidence ④, the `dired.rs` 572-590 precedent).
- **The `bufp->kind`/`result->kind`/`ie->kind` writes (C 468, 487, 594, 612)** go through a per-unit
  `input_event_bits!`, because the mirror folds `kind : 16`/`part : 16` into `input_event.bits`
  (termhooks.rs 95-118); the body is the `androidselect.rs` 135-143 macro.
- **The frame flag fields** `auto_lower`/`auto_raise` (C 447, 455) and `mouse_moved`/
  `pointer_invisible` (C 543, 306, 492) are `bool_bf` members the mirror packs into
  `frame.bits1`/`bits2`, so the unit defines the four read/write `macro_rules!` its slice needs plus
  `set_frame_flag1!` (frame.h declares them as fields, so no header mirror carries an accessor; the
  `frame.rs`/`keyboard.rs`/`term.rs` per-unit precedent).  Their shift constants are
  `include/frame.rs`'s and are gated to the frozen (non-window-system) arm: a window-system
  configuration has no numbering there yet, which is the owed mirror round below.
- **`MOUSE_HL_INFO!` (`include/frame.rs` 450-461) is the frozen configuration's arm.**  Its body is
  C 1037-1039's TTY expansion (`tty_output`), which the unit can only expand by bringing
  `include/termchar.rs` in - the *including* unit's job, as that mirror's comment says - and C's
  Android arm (`&FRAME_ANDROID_OUTPUT (f)->mouse_highlight`, frame.h 1023-1039) is text the mirror
  does not carry yet.  Until that round lands, C 332's expansion reads the wrong arm: a named
  blocker, not a port choice.
- **`UNINIT` and C's other uninitialised locals are zero-initialised** and named (PORTING.md 1.5):
  C 166's `struct android_gc_values values`, C 168's `fd_set fds`, C 338's
  `struct android_swap_info swap_info`, C 660's `struct coding_system coding`, C 661's
  `ptrdiff_t size` and C 718's `Lisp_Object text UNINIT` (`UNINIT` is `= {0,}` in the frozen
  configuration, conf_post.h 369, so C's own `text` starts nil).  `let mut timeout: timespec;`
  (C 211) is the one assignment-before-use local and needs no initialiser.
- **`ckd_mul` (C 662) is the P-15 pair**: `docs/ports/token-aliases.tsv` 13 folds
  `ckd_mul`/`overflowing_mul`, and the port writes
  `(n as ptrdiff_t).overflowing_mul (size_of::<c_ushort> () as ptrdiff_t)` with C's early
  `return Qnil ()` (the `casefiddle.rs` 318 shape: both operands in the destination's type,
  ptrdiff_t, which is what `__builtin_mul_overflow` computes in).
- **`<semaphore.h>` (C 24) and `<android/log.h>` (C 49)** are external headers with no file in the
  reference tree, so the unit carries them as comments at C's lines (P-88 ①); no name of this slice
  comes from either (the file's `sem_t edit_sem` and `__android_log_print` uses are C 5032+ and
  C 1837, both in the remainder).  The ledger names them rather than adding a file under
  `femacs/src/include/`.
- **`x_display_list` (C 41) is a non-`static` definition**, so it carries `#[unsafe(no_mangle)]`
  (PORTING.md 0.2) and shadows the glob-imported declaration in `include/androidterm.rs` 514-517's
  `androidterm_objs` module (PORTING.md 0.1-10/P-21).
- **`ANDROID_EVENT_NORMAL`/`_GOTO_OUT`/`_DROP` (C 61-67)** are C's anonymous `enum` with no assigned
  members, so the port keeps three `c_int` constants (the unnamed-enum shape
  `include/androidterm.rs`'s `FOCUS_*` uses).
- **Seven of this slice's helpers are `unsafe extern "C" fn`**, not plain `unsafe fn`: C takes their
  address - `android_clear_frame` (C 6739), `android_ring_bell` (6740),
  `android_toggle_invisible_pointer` (6742), `android_update_begin` (6743), `android_update_end`
  (6744) in `syms_of_androidterm`, and `android_show_hourglass`/`android_hide_hourglass` (6692-6693)
  in the redisplay interface's array.  The rest stay plain `unsafe fn` (PORTING.md 1.4).
- **Line count.**  C 829 vs the port's 1,055 (+27.3%): the per-item file-gate `#[cfg]`s, the
  use-site declarations and their comments, Rust's brace-only lines for C's one-line `if`/`while`
  bodies, the `as …` promotions and the C-line comments.  A
  `docs/ports/line-drift-tolerance.tsv` row (stem `androidterm`, basis `total`) is owed.

## Owed by the integrator (nothing here is claimed as done)

1. **The remainder of the file: C 4125-7117.**  Slice 9 wrote C 3799-4123 (the stretch, the scale
   factor, the underwave and the two glyph-string foregrounds), so the next slice's rows must replace
   this table's last row; its first definition is `android_draw_glyphless_glyph_string_foreground`
   (C 4126).  C 1761's `android_notify_conversion` (C 6635-6648)
   is the only forward reference left.
2. `docs/ports/line-drift-tolerance.tsv`: one row, `androidterm` `0.30` `total`, with the reason
   from the Deviations above (C 829 vs the port's 1,055).
3. `tools/check-port-lines.py --write-pending`: the `src/androidterm.c` ratchet row (one of the
   three `stale` rows) and `docs/port-map.md`'s 11-line figure for this target are both stale now.
4. `docs/ports/macro-body-notes.tsv`, the rows this slice's token sets need:
   `FRAME_ANDROID_NEED_BUFFER_FLIP`/`FRAME_ANDROID_COMPLETE_P`/`FRAME_ANDROID_DRAWABLE`
   (`need_buffer_flip`, `complete` - the three rows `include-androidterm.md` names), the
   `mouse_face_defer` and `need_cursor_updates` rows for the two direct `bool_bf` accesses (C 332,
   C 707-708), the `switch`/`case`/`default`/`break` rows for C 502, 722 and 744, and the
   `android_frame_rehighlight` row (the call is C's, the definition is past the slice).
5. **The mirror round that still blocks the Android arm reading (②, corrected by slice 5).**  With
   the five gates the real configuration has, the only members missing are `frame.want_fullscreen`
   (C 2214) and `frame.new_size_p` (C 907): `include/frame.rs` carries `FRAME_WANT_FULLSCREEN_MASK`
   (its 757-759) behind `have_window_system` and no accessor pair, and `new_size_p`'s `bool_bf` unit is
   not numbered in that region yet.  The rest of what this item used to name (the `tool_bar_window`/
   `tab_bar_window`/`last_tool_bar_item` fields, `conversion`, `FULLSCREEN_BOTH`,
   `font.baseline_offset`) is **retired**: those names exist in the Android configuration and the
   rows that reported them came from the narrow gate set (see ②, Owed 19).  The change is inside
   `include/frame.rs` and needs `tools/check-all-units.sh` (P-87), so it is not touched here.
6. The eight use-site declarations (Deviations, second bullet) belong in
   `lisp.rs`/`coding.rs`/`dispextern.rs`/`androidgui.rs` (P-88 ⑤); `struct redisplay_interface`
   belongs in `dispextern.rs`/`termhooks.rs` with its offset re-measured; the
   `use textconv_decls::*;` glob line belongs at the end of `include/textconv.rs`; and
   `include/frame.rs` 1193-1200's `flush_frame!` needs the cast its own `FRAME_RIF!` cannot supply.
7. **This ledger is a two-target ledger** (`docs/ports/frame.md` item 3's open case, now the second
   instance): `docs/port-map.md` 372 pairs `src/androidterm.c` with this unit and 373 pairs
   `src/androidterm.h` with this unit's include mirror, and `check-dead-file-coverage.py`'s ledger
   rule resolves both here (the header's own ledger `docs/ports/include-androidterm.md` is not
   reachable by that rule, which looks for `src-androidterm.md` then `androidterm.md`).  The table
   above is contiguous over C 1-7117 precisely because of that, which is what makes the 496-line
   header's binding read clean; the fix is a `src-androidterm.md` for the header, or a rule change -
   the header's coverage should not keep riding on this unit's table.
8. If a configuration ever enables a second Android window system or `harfbuzz`, the two `--cfg`s of
   Evidence ② are the last unmeasured corner of this record; nothing else in the slice names them.

9. `docs/ports/macro-body-notes.tsv` (this lane writes it, and slice 2 added the one row it needs):
   `handle_one_android_event` → `case,default,double,goto,has_been_visible,need_cursor_updates,switch`,
   with the C bodies named.  The `double` token is C 849's `double scroll_unit;`, which the port
   spells `f64` (PORTING.md 1.2); the two `bool_bf` tokens are the `android_output.bits` mask reads/
   write (Deviations, this slice).
10. **Eleven use-site declarations** the frozen mirrors do not carry (slice 2): `frame_size_history_extra`
   (frame.h 1515), `frame_ancestor_p` (frame.h 1911), `expose_frame` (dispextern.h 3675),
   `handle_tab_bar_click` (dispextern.h 3668), `handle_tool_bar_click` (dispextern.h 3670),
   `cancel_mouse_face` (dispextern.h 3690), `clear_mouse_face` (dispextern.h 3691),
   `change_frame_size` (dispextern.h 3924), `gen_help_event` (keyboard.h 513),
   `from_unicode_buffer` (coding.h 737) and `fabs` (`<math.h>`).  Each belongs in the mirror its
   header names (P-88 ⑤); `fabs` also stands in `lib/include/math.rs`'s gap (that mirror carries
   `pow` and the `signbit` macro and no `fabs`).
11. **The five `help_echo_*` objects** (dispextern.h 3614-3616) belong in `include/dispextern.rs`'s
   glob-imported region - the same row `src/xterm.rs` 31179 files for the same five names.
12. **`mouse_face_hidden!`, the per-unit macro** (slice 2): the mirror carries the
   `MOUSE_FACE_HIDDEN` mask (dispextern.rs 2139) and no accessor, so this unit names the pair - the
   `src/xterm.rs` 31036 / `src/haikuterm.rs` 4884 copies.  A mirror-level accessor pair would let
   all three units drop it.

13. **Four more use-site declarations** (slice 3): `gui_update_cursor` (dispextern.h 3656),
   `parse_color_spec` (dispextern.h 3798-3799), `gamma_correct` (dispextern.h 3839 - the definition
   is `src/androidfns.rs` 481's, and this unit's is the declaration side) and `store_frame_param`
   (lisp.h 5117).  Each belongs in the mirror its header names (P-88 ⑤).
14. **`xstrcasecmp` reaches this unit by path** (`dispextern_externs::xstrcasecmp`, the
   `src/dosfns.rs` 791 device), because the mirror keeps the header's `INLINE` definition in
   `dispextern.rs` 2642-2652's module rather than in the glob-imported region; the alternative
   (`src/fontset.rs` 108's fourth copy of the declaration, or `src/font.rs` 298's third definition)
   is what P-26's one-declaration rule is for, and this unit takes neither.
15. **The `have_window_system` gap this slice widens**: `android_fullscreen_hook` (C 2214) is the
   second place in this unit that reads `f->want_fullscreen` and the first that names
   `FULLSCREEN_BOTH` - `include/frame.rs` carries both behind `feature = "have_window_system"`
   (its 757-759's mask pair and its 63's constant), so the frozen reading has three more named
   blockers.  `docs/ports/include-frame.md`'s round is where the numbering for the Android
   window-system configuration belongs.
16. **Five more use-site declarations** (slice 4): `adjust_frame_size` (frame.h 1511-1512),
   `do_pending_window_change` (dispextern.h 3923), `get_font_ascent_descent` (dispextern.h 3631),
   `free_frame_faces` (dispextern.h 3822) and `fontset_from_font` (fontset.h 39).  The last one is
   also a *module* case: the mirror keeps it in `fontset_decls` (`include/fontset.rs` 53-69) and this
   unit's include graph does not reach that file, so the declaration stands here instead of the
   path (Owed 14's alternative).
17. **Four frame.h accessors C writes** (slice 4) need a write arm the mirror does not carry
   (`include/frame.rs` has the getters): `FRAME_COLUMN_WIDTH`, `FRAME_LINE_HEIGHT`,
   `FRAME_TAB_BAR_HEIGHT` and `FRAME_CONFIG_SCROLL_BAR_COLS`.  Slice 4 carries the four two-arm
   per-unit macros (`src/xterm.rs` 15127-15145's shape); the mirror round that gives every unit one
   spelling is `docs/ports/include-frame.md`'s.
18. **The window-system gap widens again**: `font.baseline_offset` (C 2499) is behind
   `feature = "have_window_system"` in `include/font.rs` 108-121, so this slice adds a twentieth
   named blocker of that class (Owed 15 is the frame-side half).
19. **The reading's gate set** (slice 5, and the reason it is an owed item rather than a footnote):
   every `probe`/`build-tu` reading of this unit from now on carries the five gates of ② -
   `have_android`, `not_android_stubify`, `have_window_system`, `have_text_conversion`,
   `dispextern_face_records`.  Slices 1-4's rows in this ledger that name a `have_window_system`
   member or a `HAVE_TEXT_CONVERSION` field are superseded by that measurement; the Deviations that
   describe *port text* (the folded-bit masks, the labelled blocks, the use-site declarations) stand
   unchanged.
20. `docs/ports/include-androidterm.md`: the mirror's own line 36-40 note ("a TU that needs them
   must have the mirror in its own list ... the port will need the same `cfg` on dispextern.rs's
   alias when that target is frozen") is the mirror-side half of Owed 19; that file's ledger is where
   the five-feature set should be recorded for the other mirror readers.
21. **Two records reach this unit by module path** (slice 5): `draw_fringe_bitmap_params` and the
   `get_cursor_p`/`get_overlay_p` pair live in `include/dispextern.rs` 2800-3684's
   `pub mod dispextern_unit_names`, so the unit writes `use dispextern_unit_names::{...}` rather than
   carrying the copies `src/xterm.rs` 9292-9450 keeps (that unit is compiled with the file gate only).
   If the module's records are ever moved to the top level, this `use` line goes with them.

22. **Three per-unit macro families** (slice 7): `face_bits!` (with C's two bit positions as
   constants), `IMAGE_BACKGROUND`/`IMAGE_BACKGROUND_TRANSPARENT` and `glyph_u_cmp_bits!`.  The mirror
   carries the storage units and the masks and no readers, so `src/xterm.rs` 9844-9897,
   `src/haikuterm.rs` 227-244, `src/pgtkterm.rs`, `src/term.rs` and now this unit each spell their
   own; a mirror-level accessor set (or the five records' move to the top level, Owed 21) is what
   would let them go.

23. **Three use-site declarations** (slice 9): `window_box_left` (dispextern.h 3579),
   `window_box_left_offset` (3580) and `cursor_in_mouse_face_p` (3692, defined in `src/xdisp.c`).
   This unit's include graph reaches none of them, so they stand at their use sites (P-72 rule 4) -
   the same finding the third slice recorded for `note_mouse_highlight`/`remember_mouse_glyph`; a
   `src/include/dispextern.rs` round that adds the three, or a `windows.rs`/`xdisp.rs` edge, is what
   would let them go.

### Not claimed

Every line of this file is behind gates the frozen configuration leaves false: ① is an empty object,
② is a compile-level reading of **C 1-829 only**, and nothing here was executed - no JNI call, no
frame, no cursor, no bell, no IME event.  The arm reading's 17 rows are the complete error list for
that gate set, but they are still type-check/codegen readings, and the six rows in this unit's own
text are the named mirror blockers, not measurements of the code's behaviour.  **C 830-1892 is now
ported** (slice 2) and its own reading is the second half of ② below; **C 1894-2238 is now ported**
(slice 3) and its own reading is the third half of ② below; **C 2240-4123 is now ported** (slices 4-9,
each with its own Evidence section below); C 4125-7117 is recorded, not ported.

## 2026-10-05: the four folded-bit writer sites (3.516)

C writes three of `android_output`'s folded bits through macros that are *places*
(`FRAME_ANDROID_COMPLETE_P (f) = false;` at C 316/469 and `FRAME_ANDROID_NEED_BUFFER_FLIP (f) =
false;` at C 347) - a Rust macro cannot be one when the bit lives in a byte.  Each site spells the
mask write instead (PORTING.md P-80, the `cm.rs` convention) and the four functions carry
`macro-body-notes.tsv` rows for C's member names; `bit`-value writes use C's own two arms (C 707-708
assigns an `int` to a one-bit field, which lands as 0/1).  With the mirror's masks in place this
unit's arm goes **19 -> 2 errors**, and both of those are named in
`docs/ports/include-androidterm.md`.

## 2026-10-05: the last real error in this unit's arm was a folded flag (3.522)

`androidterm.c` 3039's `hlinfo->mouse_face_defer = false;` is one of `dispextern.h` 3019-3023's
three `bool_bf : 1` members, which the mirror folds and names through accessors
(`get_mouse_face_defer`/`set_mouse_face_defer`, `include/dispextern.rs` 2114/2117, PORTING.md
P-80) - so the unit's writer site now calls the accessor and carries a `macro-body-notes` row for
C's member name.  With that, this unit's Android arm reads **19 -> 1**, and **the one left is the
slice boundary**: `android_frame_rehighlight` is defined after C 1055, which is where this unit's
`C line ranges` table ends.  The Android family therefore stands at
`androidfns 0, androidvfs 0, androidmenu 0, androidselect 0, androidfont 0, androidterm 1 (slice
boundary)`, and the frozen arm still parses (`build-tu src/androidterm.c` -> exit 0).

## 2026-10-12: slice 2 (C 830-1892) - `handle_one_android_event`

The file's largest function (1,056 C lines) and C 833-834's forward declaration above it: the whole
Android event dispatch - the `ANDROID_CONFIGURE_NOTIFY`/geometry branch, the key-press branch with
its IME pre-edit handling and `done_keysym` label, focus, window-action plus the ENTER_NOTIFY it falls
through into, motion, leave, expose, button press/release with the tab-bar/tool-bar hit tests, the
four touch events, the wheel (with `mwheel_coalesce_scroll_events` coalescing), iconify/deiconify,
the context menu, the input-method dispatch, drag-and-drop (drag, URI and text), the two notification
events, the configuration-changed pair, and the shared `OTHER:` tail.

### Evidence (this slice's own readings)

1. **The arm flip** (`cargo run -q -p xtask -- probe src/androidterm.rs /tmp/aterm-lane/arm.o
   --crate-type lib --emit obj -C panic=abort --cfg 'feature="have_android"'
   --cfg 'feature="not_android_stubify"'`): **exit 1, 19 errors**, and every one is in a class this
   ledger already names.  By file:
   - **17 in the unit's own text, all `have_window_system` members the frozen mirror does not carry**:
     `frame.tool_bar_window` (7 sites), `frame.tab_bar_window` (4), `frame.last_tool_bar_item` (3),
     `frame.new_size_p` (1) and `frame.conversion` (2, slice 1's existing pair, which is also a
     *configuration* reading: `have_text_conversion` makes those two disappear).  C's own text for
     each is carried (`(*f).tool_bar_window`, `(*f).new_size_p`, `f->conversion.batch_edit_*`), so
     these are Owed 5's mirror round, not port choices.
   - **2 forward references**: C 458's `android_frame_rehighlight` (C 2127) and C 1761's
     `android_notify_conversion` (C 6636) - the named-blocker class slice 1 recorded, now with the
     second call site added.
   No other error: the 140-error first reading of the fresh text (below) is fully discharged.
2. **The 11 mirror-vs-unit spellings this slice's text needed** (each named in Deviations/Owed 9-12):
   `CONSP`/`FIXNUMP`/`SINGLE_BYTE_CHAR_P` are *functions* in the mirrors (not macros, so no `!`);
   `SET_FRAME_GARBAGED!`/`SET_FRAME_VISIBLE!`/`FRAME_ANDROID_COMPLETE_P!` are macros (so `!`);
   `popup_activated () == 0` for C's `!popup_activated ()`; `android_window`/`android_xaction.window`
   are `isize` (so `== 0`, not `is_null ()`); `u32::MAX` for C's `(uint32_t) -1`; the `as …`
   promotions C's usual arithmetic conversions supply (event `state`/`keycode`/`time`/`pointer_id`
   fields are `c_uint`/`u32`, the mirrors' `android_find_tool`/`android_*_modifiers`/
   `XSETINT!`/`XSETFRAME!` take or want `c_int`/`isize`/`u64`); `window_from_coordinates`,
   `android_set_preeditarea`, `android_notification_*` take `*mut c_void` where C passes the typed
   pointer; `mouse_face_mouse_frame` is `*mut c_void` in the mirror (two casts); and `signbit!` is
   the mirror's macro (fine) while `fabs` is the use-site declaration above.
3. **Six `probe` invocations for this batch, against a budget of three** - recorded here rather than
   left implicit, with the reading each produced: (1) the text as first written, **140 errors**
   (missing declarations, the folded-bit members, and the promotions above - the compiler as a work
   queue, the PORTING.md process); (2) after the first mechanical batch and the declaration block,
   **44**; (3) after the second batch, **25**; (4) after the third, **21**; (5) after the fourth,
   **19** - and one invocation before those re-read a revision whose own file had been mangled by
   the editing tool (a stray narration fragment and tool markup had landed in the file); that was
   repaired before anything was committed, and it is why the first reading is 140 rather than the
   ~60 a clean first draft would show.  Each run is a single-file `rustc` invocation of 0.6-2.6 s
   under `CARGO_TARGET_DIR=/tmp/aterm-lane`; `build/probes/androidterm` is removed at the end of the
   round.
4. **Inventory** (`python3 tools/extract-abi.py --check emacs/src/androidterm.c femacs/src/androidterm.rs`):
   the 116 rows of slice 1's reading are now **41** - the thirty-six `missing function` rows for
   C 1894-7117's own items, one `missing macro`/`missing constant` pair
   (`HIGHLIGHT_COLOR_DARK_BOOST_LIMIT`), the five `missing file-scope object` rows
   (`edit_counter`, `last_edit_counter`, `edit_sem`, `request_class`, `text_class`) and the
   whole-file line-count row, which reads **68.2%** (C 7,117 against the port's 2,265).  The one new
   `does not mention` row (`handle_one_android_event` → `case,default,double,goto,has_been_visible,
   need_cursor_updates,switch`) was discharged in the same commit by the
   `docs/ports/macro-body-notes.tsv` row of Owed 9.

### Deviations this slice adds

- **The folded `bool_bf` members this slice reads and writes**: C 943-947/1115-1119's
  `FRAME_OUTPUT_DATA (f)->need_cursor_updates` (a read) and C 1287's
  `has_been_visible = true;` (a write) are two members the mirror folds into
  `android_output.bits` (`include/androidterm.rs` 444/448), so the port writes
  `… .bits & ANDROID_OUTPUT_NEED_CURSOR_UPDATES != 0` and `… .bits |= ANDROID_OUTPUT_HAS_BEEN_VISIBLE`
  - the slice-1 device for the same record (its C 707-708).  C 1490's
  `any->pointer_invisible = false;` uses the unit's own `pointer_invisible!` macro, and C
  959/967/1170/1172's four `Mouse_HLInfo.mouse_face_hidden` accesses the new unit-local
  `mouse_face_hidden!` pair (Owed 12).
- **C's `do_help = count = 0;` (C 859)** is two Rust statements (`count = 0; do_help = count;`)
  because an assignment is not an expression there; the comment keeps C's line.
- **C's `switch (event->type)` (C 870) is a `match`**, and its `OTHER:` label (C 1860) is the
  labelled block `'other`, which is also where C's `break` (C 1055) lands - after the switch.  The
  forty-seven `goto OTHER`s and the nine `goto done_keysym`s become `break 'other;`/`break
  'done_keysym;`, and `done_keysym:` (C 1110) is a nested labelled block inside the KEY_PRESS arm
  (the label sits after C 1108's jump, and every jump starts inside the C 1006-1050 block).
- **`case ANDROID_WINDOW_ACTION` falls through into `case ANDROID_ENTER_NOTIFY`** (C 1128-1163):
  C's block has no `break`, so the two share one `match` arm and both bodies stand in C's order.
- **C 853's function-local `static struct android_compose_status compose_status;`** is a
  function-local `static mut` (PORTING.md 1.6), zero-initialised field by field.
- **C 1836-1841's `#if notdef` block** (the `__android_log_print` trace inside
  `ANDROID_CONFIGURATION_CHANGED`) is dead text in every configuration: recorded here, not
  transcribed (the slice-1 `#if false` rule).
- **`double` is `f64`** (C 849's `double scroll_unit;`), which is why the token row of Owed 9 lists
  it.

## 2026-10-12: slice 3 (C 1894-2238) - the frame, terminal and redisplay-interface callbacks

The run of callbacks after the event dispatcher: `android_read_socket` (the event loop that calls
`handle_one_android_event` and the auto-raise tail), `android_frame_up_to_date` and
`android_buffer_flipping_unblocked_hook` (the buffer-flip pair), `android_query_frame_background_color`,
the three colour helpers (`android_parse_color`, `android_alloc_nearest_color`, `android_query_colors`),
`android_mouse_position`, the focus group (`android_get_focus_frame`, `android_focus_frame`), the
cursor pair (`android_frame_highlight`/`_unhighlight`) and `android_frame_rehighlight` with its hook,
`android_frame_raise_lower`, the three visibility hooks, `android_fullscreen_hook` and
`android_iconify_frame`.

### Evidence (this slice's own readings)

1. **The arm flip** (`probe src/androidterm.rs /tmp/aterm-lane/arm.o --crate-type lib --emit obj
   -C panic=abort --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`): **exit 1,
   20 errors**, every one in a class this ledger names - **19 are the `have_window_system` members
   and constants the frozen mirror does not carry** (`frame.tool_bar_window` 7 sites,
   `frame.tab_bar_window` 4, `frame.last_tool_bar_item` 3, `frame.conversion` 2, `frame.new_size_p`
   1, `frame.want_fullscreen` 1 and `FULLSCREEN_BOTH` 1), and **1 is the surviving forward
   reference** (C 1761's `android_notify_conversion`, C 6636).  **Slice 1's and slice 2's
   `android_frame_rehighlight` `E0425` is gone**: this slice defines it (C 2127), which is the first
   of the two forward-reference rows this ledger has carried since slice 1.
2. **Five reads the first draft needed to converge, each a mirror-vs-C spelling** (all recorded in
   Deviations): `FOR_EACH_FRAME!` takes the body as its third argument (`FOR_EACH_FRAME! (tail,
   frame, { … })`, the `src/androidfns.rs` 1948 form) rather than C's following block; `min` is
   lisp.h 71-72's macro, which the mirror spells as the generic function `min` (`include/lisp.rs`
   3158), not gnulib's `MIN!`; `RGB_TO_ULONG!` needs `c_ulong` operands (the `src/haikufns.rs` 897
   device); `android_set_fullscreen` returns `c_int`, so C's `if (…)` is `if (…) != 0`; and
   `xstrcasecmp` is reached by path (Owed 14).
3. **Three `does not mention` rows appeared and were discharged in this commit** by three
   `docs/ports/macro-body-notes.tsv` rows: `android_frame_up_to_date` → `frame_android_complete_p,
   true` (the folded-member write), `android_frame_rehighlight` → `0` (C's null constant), and
   `android_make_frame_visible` → `false` (the mirror's `SET_FRAME_ICONIFIED` takes `c_int`).
4. **Inventory** (`extract-abi.py --check`): 41 rows -> **34** - the `missing function` rows for
   C 2240-7117's items fall by the nineteen this slice lands, the `missing macro`/`missing constant`
   pair and the five `missing file-scope object` rows stand - the macro/constant at C 3357
   (`HIGHLIGHT_COLOR_DARK_BOOST_LIMIT`) and the objects at C 5026/5029/5032 (`edit_counter`,
   `last_edit_counter`, `edit_sem`) and C 5890/5893 (`request_class`, `text_class`) - and the
   line-count row reads **62.4%** (C 7,117 against the port's 2,673).

### Deviations this slice adds

- **C's linkage plus the addresses C takes decide each item's shape**: `#[unsafe(no_mangle)] pub
  unsafe extern "C" fn` for the four non-`static` definitions (`android_parse_color`,
  `android_alloc_nearest_color`, `android_query_colors`, `android_make_frame_visible`,
  `android_make_frame_invisible`, `android_iconify_frame` - the mirror declares them in its
  `androidterm_decls` module, `include/androidterm.rs` 578-587); `unsafe extern "C" fn` for the
  fourteen `static` ones whose address C 6745-6761's hook assignments take; and a plain Rust-ABI
  `unsafe fn` for `android_frame_highlight`/`_unhighlight`/`android_frame_rehighlight`, which C only
  calls (C 2152/2154/2161).
- **C 1941's `FRAME_ANDROID_COMPLETE_P (f) = true;`** is an l-value use of a folded member, so the
  port writes the mask write the mirror's own comment prescribes (`bits |= ANDROID_OUTPUT_COMPLETE`,
  the `cm.rs` convention) with C's line kept.
- **`FOR_EACH_FRAME` (C 2077)** is the mirror's three-argument macro, so C's trailing block is its
  third argument.
- **C 1988's `for (; CONSP (tem); tem = XCDR (tem))`** is the `while` + trailing step the port's
  iterator rule uses, and C 2045's `for (i = 0; i < ncolors; ++i)` the same shape.
- **C 2026-2029's luminance fold** keeps C's four `double` operands (`color->red * 0.299` …) with the
  `as f64` promotions Rust requires, and C 2017-2021's 30/59/11 fold keeps C's integer arithmetic
  with `c_ulong` operands for the `>>`/compare.

## 2026-10-12: slice 4 (C 2240-2648) - the wait, size, position and teardown groups

`android_wait_for_event` (the timeout loop over `android_select`), the two `_1` sizing helpers with
their public wrappers, `android_calc_absolute_position` and `android_set_offset`, the empty
`android_set_alpha`, `android_new_font`, `android_bitmap_icon`, `android_free_pixmap_hook`,
`android_free_frame_resources`, `android_delete_frame` and `android_delete_terminal`.

### Evidence (this slice's own readings)

1. **The arm flip**: **exit 1, 21 errors**, every one in a class this ledger names - **20 are the
   `have_window_system` gap** (`frame.tool_bar_window` 7, `frame.tab_bar_window` 4,
   `frame.last_tool_bar_item` 3, `frame.conversion` 2, `frame.want_fullscreen` 1,
   `frame.new_size_p` 1, `FULLSCREEN_BOTH` 1 and now `font.baseline_offset` 1) and **1 is the
   surviving forward reference** (C 1761's `android_notify_conversion`, C 6636).  The first draft's
   other reads were the unit's own and are all discharged: fifteen `is_null ()` tests on
   `android_cursor` members (`android_cursor` is an integer handle, `include/androidgui.rs` 59, so C's
   truth test is `!= 0`), the three `c_uint` width/height arguments of `android_resize_window`/
   `android_move_resize_window` (`include/androidgui.rs` 960/933), and `fontset_from_font` (Owed 16).
2. **Four `does not mention` rows appeared and were discharged in this commit**: `android_wait_for_event`
   → `double` (the port's `f64`), the two sizing helpers → `0` (C's `0` for the prototype's `bool`),
   and `android_calc_absolute_position` → `has_been_visible` (the folded-mask write).
3. **Inventory**: 34 rows -> **25** - the `missing function` rows for C 2652-7117's items fall by the
   fourteen this slice lands, the `missing macro`/`missing constant` pair and the five
   `missing file-scope object` rows stand, and the line-count row reads **55.8%** (C 7,117 against
   the port's 3,149).
4. **The first draft's five readings** are recorded in Deviations (the cursor test, the two
   width/height type pairs, the nested `FRAME_*` accessor writes and the two module-path
   alternatives), with the two-arm macro block the write sites need.

### Deviations this slice adds

- **`android_cursor` is an integer handle** (`android_handle`, `include/androidgui.rs` 59): C's
  fifteen `f->output_data.android->text_cursor` … truth tests are `!= 0` in the port, the same shape
  the file's `FRAME_ANDROID_WINDOW (f) != 0` tests use.
- **The four frame.h accessors C writes** are the unit's two-arm macros (Owed 17); the three
  `FRAME_FONT`/`FRAME_FONTSET`/`FRAME_BASELINE_OFFSET` writes need no macro of their own, because
  `include/androidterm.rs` 483-493's bodies are place expressions.
- **`android_resize_window`/`android_move_resize_window`** take `c_uint` sizes while their x/y
  arguments are `c_int` (`include/androidgui.rs` 933/960), so the port casts exactly the two width
  and height arguments C's own prototype promotes.
- **`fontset_from_font`** is declared at its use site rather than reached by the mirror's module path
  (Owed 16), because this unit's include graph does not carry `include/fontset.rs`.
- **C 2254-2256's `make_timespec` arithmetic** keeps C's three `1e9` factors and the `(long int)`
  cast (`c_long`), and C 2275's `android_select (0, NULL, NULL, NULL, &tmo)` the three null pointers
  and the `&tmo` out-parameter.

## 2026-10-12: slice 5 (C 2652-3067) - the redisplay interface's drawing helpers

`android_scroll_run` and its clipping/GC companions: `android_after_update_window_line`,
`android_flip_and_flush`, `android_clear_rectangle`, `android_reset_clip_rectangles`,
`android_clip_to_row`, `android_draw_fringe_bitmap`, the five glyph-string GC setters and the two
clipping setters.

### Evidence (this slice's own readings)

1. **The corrected arm flip** (the paragraph inserted at ② above): with the five gates the real
   Android configuration has, the unit reads **exit 1, 3 errors** - `frame.want_fullscreen`,
   `frame.new_size_p` (Owed 5) and `android_notify_conversion` (the forward reference).  The first
   draft's own reads (19 of them, all discharged in this commit) were: the frame mirror's frozen-arm
   fields the narrow gate set made visible, `android_cursor`/`android_pixmap`'s integer handles
   (`ANDROID_NONE`'s casts), the `c_uint` dimensions of `android_create_pixmap_from_bitmap_data` and
   `android_copy_area`, `android_gc_value_mask`'s type, the `Emacs_Rectangle` pointers
   `gui_intersect_rectangles` wants, and the degraded-array `clip` cast.
2. **Three `does not mention` rows appeared and were discharged in this commit**: `android_flip_and_flush`
   → `frame_android_complete_p,true` (the folded-member write), `android_draw_fringe_bitmap` →
   `cursor_p,goto,overlay_p` (the accessors and C 2844's label) and `android_set_mouse_face_gc` → `0`
   (C's null pointer constant).
3. **Inventory**: 25 rows -> **11** - the `missing function` rows for C 3067-7117's items fall by the
   thirteen this slice lands, the `missing macro`/`missing constant` pair and the five
   `missing file-scope object` rows stand, and the line-count row reads **48.9%** (C 7,117 against
   the port's 3,640).

### Deviations this slice adds

- **`window_bits!`** (the unit's, for `pseudo_window_p`, read and write arms) - the four sibling
  window-system units' macro, which `src/androidfns.rs` 2955-2962 carries with the write arm only.
- **`glyph_row_bits!`** comes from the mirror (`include/dispextern.rs` 1114-1215) for
  `mode_line_p`/`redraw_fringe_bitmaps_p`/`stipple_p`, and **`glyph_string_bits!`** for `stippled_p`;
  `draw_fringe_bitmap_params`'s two bits are the mirror's `get_cursor_p`/`get_overlay_p` (Owed 21).
- **C 3062's `dst->clip[0] = r;`** is the `NativeRectangle` degraded-array store: `android_rectangle`
  through the mirror's `[c_int; 2]` field, written as the `src/xterm.rs` 9695-9696 cast with C's line
  kept (the mirror's own note at 1733-1741 records why the type is not its Android answer).
- **C 2732's and C 1941's `FRAME_ANDROID_COMPLETE_P (f) = true;`** are the mask writes the mirror's
  comment prescribes, and C 2907/2942/2953/2973/2987/3001/3022's `s->gc` writes take the mirror's
  `*mut c_void` GC field (P-72 rule 3).

## 2026-10-12: slice 6 (C 3068-3357) - the glyph-string drawing helpers

`android_compute_glyph_string_overhangs`, `android_clear_glyph_string_rect`,
`android_draw_glyph_string_background`, the two geometry helpers (`android_fill_triangle`,
`android_make_point`) with `android_inside_rect_p` and `android_clear_point`,
`android_draw_relief_rect` and `android_draw_box_rect`, and C 3357's
`HIGHLIGHT_COLOR_DARK_BOOST_LIMIT`.

### Evidence (this slice's own readings)

1. **The arm flip** (the five gates of ②): **exit 1, 3 errors** - the same two frame-mirror fields
   (`want_fullscreen`, `new_size_p`) and the same forward reference (`android_notify_conversion`).
   The first draft's 16 own readings were the widened-`c_short` overhang arithmetic
   (`font_metrics`/`composition` members are `c_short` in the mirrors and C's `int` members here),
   the `c_uint` width/height arguments of `android_fill_rectangle`/`android_draw_rectangle`
   (`include/androidgui.rs` 938/946) and the two `font->driver->text_extents`/`composition_gstring_*`
   spellings (`src/xterm.rs` 9704-9736's device).
2. **Inventory**: 11 rows -> **7** - the `missing macro`/`missing constant` pair is **discharged** by
   C 3357's constant, the `missing function` rows fall by the eight this slice lands, and what stands
   is C 6635-6648's `android_notify_conversion` (still the one forward reference), C 5044-7117's
   remaining items and the five file-scope objects.  The line-count row reads **44.2%** (C 7,117
   against the port's 3,966).
3. No new `does not mention` row appeared for this slice: the two `FONT_*` macros, the four
   `glyph_string_bits!` members, `glyph_bits!`'s `type`, and `android_draw_relief_rect`'s four
   `if`/`else` pairs all keep C's token set.

### Deviations this slice adds

- **`composition_gstring_from_id`/`composition_gstring_width`** are the mirror's declarations (the
  `composite_decls` glob import at `include/composite.rs` 411-471, reached through C 34's `coding.h`
  edge): the first draft repeated them at the use site and `tools/check-mirror-dupes.py` named both
  as duplicates, which is the gate that keeps one declaration per name (P-26).  The width call
  passes `&raw mut metrics as *mut c_void` because the mirror's declaration takes the metrics untyped
  (`include/composite.rs` 433-436, `src/xterm.rs` 9729-9731).
- **C's `s->first_glyph->type`** is `glyph_bits! (type, (*(*s).first_glyph))`, and
  `s->background_filled_p`/`stippled_p`/`font_not_found_p`/`extends_to_end_of_line_p` the mirror's
  `glyph_string_bits!` arms.
- **The `?:` in C 3087-3094** are the port's `if`/`else` expressions with the `as c_int` promotions
  the record's `c_short` members need, and C 3177/3179-3181's rectangle subscripting is the
  `.offset (i as isize)` form with the same promotions.

## 2026-10-12: slice 7 (C 3360-3580) - the relief-colour group

`android_alloc_lighter_color`, `android_setup_relief_color`, `android_setup_relief_colors`,
`android_draw_glyph_string_box` and `android_draw_glyph_string_bg_rect`.

### Evidence (this slice's own readings)

1. **The arm flip** (the five gates of ②): **exit 1, 3 errors** - the same two frame-mirror fields and
   the same forward reference.  The first draft's own readings were the three per-unit macro families
   this slice needed (`face_bits!` for `box`/`use_box_color_for_shadows_p` with their two constants,
   `IMAGE_BACKGROUND`/`IMAGE_BACKGROUND_TRANSPARENT`, `glyph_u_cmp_bits!` for `u.cmp.automatic`/`id` -
   the `src/xterm.rs` 9844-9897 and `src/haikuterm.rs` 227-244 copies), the pointer-vs-value split of
   the mirror's `glyph_bits!`/`glyph_bool_p` arms, the `c_ulong` `-1` sentinel of
   `relief->pixel`, and `get_glyph_string_clip_rect`'s `*mut c_void` second parameter.
2. **Two `does not mention` rows appeared and were discharged in this commit**:
   `android_alloc_lighter_color` → `double` (the port's `f64`) and `android_setup_relief_color` → `0`
   (C's null pointer constant for `relief->gc`).
3. **Inventory**: 7 rows -> **5** - the `missing function` rows fall by the five this slice lands,
   the five file-scope objects stand, and the line-count row reads **39.8%** (C 7,117 against the
   port's 4,283).

### Deviations this slice adds

- **The three per-unit macro families** (Owed 22): the mirror keeps `struct face`'s bit unit, the
  `IMAGE_BACKGROUND_*` masks and `glyph`'s `u.cmp` unit, and not their readers, so each unit that
  reads them names its own - this unit's copies cite the `src/xterm.rs`/`src/haikuterm.rs` precedents.
- **`image_background`/`image_background_transparent`** are declared at the use site (dispextern.h
  3769-3772) and take `Emacs_Pixmap`, which is an integer handle here, so C 3478/3479's `0` stays `0`.
- **C 3447/3448/3456's `-1` sentinel** for `relief->pixel` (a `c_ulong`) is the
  `(-1 as c_long) as c_ulong` cast the `src/xterm.rs` 11220-11223 pair uses.
- **C 3508-3531's ternary and the `for` walk over automatic compositions** keep C's order: the
  `last_glyph = g++` step is the two-statement form, and `g->u.cmp.id == s->cmp_id` is the
  `glyph_u_cmp_bits!` read widened to `ptrdiff_t`.

## 2026-10-12: slice 8 (C 3582-3797) - the image-drawing helpers

`android_draw_image_relief`, `android_draw_image_foreground` and `android_draw_image_glyph_string`.

### Evidence (this slice's own readings)

1. **The arm flip** (the five gates of ②): **exit 1, 3 errors** - the same two frame-mirror fields and
   the same forward reference.  The first draft's own readings were the mirror's `glyph_slice`
   accessor (C's four `unsigned : 16` members are `bits [0]`/`bits [1]` with `glyph_slice_bits!`'s
   arms, `include/dispextern.rs` 338-349/605-611 - thirty reads in this slice), the two
   `EMACS_INT` Lisp variables `tab_bar_button_relief`/`tool_bar_button_relief` (`min`'s two arguments
   need one type), and the `*mut c_void`/`android_pixmap`/`android_drawable` casts of the image
   record's `mask`/`pixmap`.
2. **No new `does not mention` row**: the two `TAB_BAR_FACE_ID`/`TOOL_BAR_FACE_ID` branches, the eight
   `XFIXNUM (XCAR …)` reads and the four comma statements of C 3657-3664 all keep C's token set.
3. **Inventory**: 5 rows -> **5** (this slice's three `missing function` rows retire and
   `image_ascent`'s declaration is at the use site); the five file-scope objects stand, and the
   line-count row reads **36.3%** (C 7,117 against the port's 4,531).

### Deviations this slice adds

- **`glyph_slice`'s four members** are the mirror's `glyph_slice_bits! (x|y|width|height, slice)` reads
  with `as c_int` (`include/dispextern.rs` 605-611; C's members are the mirror's own comment's
  "four `unsigned : 16`", so no sign extension is involved), and the `glyph_slice` pointer
  `image_ascent` takes is C's own `&s->slice`.
- **C 3657-3664's four comma statements** (`x -= thick + extra_x, left_p = true;`) are two statements
  each in C's order, and C 3628/3655's chained assignments are the same split.
- **`image_ascent`** is declared at the use site (dispextern.h 3774) - the one name of this slice no
  mirror reaches.

## 2026-10-12: slice 9 (C 3799-4123) - the stretch, the scale factor, the underwave and the two foregrounds

`android_draw_stretch_glyph_string`, `android_get_scale_factor`, `android_draw_underwave`,
`android_draw_glyph_string_foreground` and `android_draw_composite_glyph_string_foreground`.

### Evidence (this slice's own readings)

1. **The arm flip** (the five gates of ②): **exit 1, 3 errors** - the same two frame-mirror fields
   (`want_fullscreen`, `new_size_p`) and the same forward reference (`android_notify_conversion`).
   The first draft's own readings were the `c_short` type of the mirror's `glyph.pixel_width` at the
   three advance sites (`x += (*g).pixel_width as c_int`), `glyph_string_bits! (for_overlaps, …)`'s
   bit-run `int` return against C's `if` (`!= 0`, the `src/haikuterm.rs` 1438 / `src/pgtkterm.rs`
   2089 form - the `bool` arms of that macro are the ones C 3002-3023 read), `COMPOSITION_GLYPH`'s
   `EMACS_INT` return against C's `'\t'` character constant (`as isize`, the `src/pgtkterm.rs` 2159
   and `src/xterm.rs` 10130 form), and the two `scale_x`/`scale_y` places `&raw mut` needs
   initialised.
2. **No new `does not mention` row**: the stretch branch's `x_stretch_cursor_p`/`FRAME_COLUMN_WIDTH`
   tokens, the underwave loop's ten-point arithmetic and the composite walk's
   `LGSTRING_GLYPH`/`LGLYPH_FROM`/`LGLYPH_TO`/`LGLYPH_WIDTH` reads all keep C's token set.
3. **Inventory**: 45 rows -> **40** - the five `missing function` rows this slice lands retire, and
   what stands is the file's 34 remaining definitions (32 live ones past this slice, plus
   `android_notify_conversion` and `syms_of_androidterm`), the five file-scope objects, and the
   line-count row, which reads **30.8%** (C 7,117 against the port's 4,923).

### Deviations this slice adds

- **The three use-site declarations** (Owed 23): `window_box_left`, `window_box_left_offset`
  (dispextern.h 3579-3580) and `cursor_in_mouse_face_p` (3692, `src/xdisp.c`'s) stand at their use
  sites because this unit's include graph reaches none of them.
- **`face_bits!` grows one arm**: C 4119's `s->face->overstrike` is the run's twenty-second bit, so
  the unit adds `FACE_OVERSTRIKE: u32 = 1 << 21` (C's own position, the `src/xterm.rs` 9850 value)
  and the reader arm beside the two the seventh slice added - the same Owed 22 family.
- **C 3931's `int scale_x, scale_y;`** are `let mut … = 0` here: C leaves the two places unset until
  `android_get_scale_factor` writes them, and `&raw mut` needs an initialised place (the
  `src/haikuterm.rs` 1287-1288 and `src/w32term.rs` 1007-1008 shape).
- **The three promotions of this slice**: `(*g).pixel_width as c_int`, the `for_overlaps` reader's
  `!= 0`, and `b'\t' as isize` for C's `'\t'` against `COMPOSITION_GLYPH`'s `EMACS_INT`.
