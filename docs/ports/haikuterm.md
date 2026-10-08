# emacs/src/haikuterm.c (4898 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `configure.ac` 3097
> clears `HAIKU_OBJ` and 3114 fills it with `haikuterm.o` under `test "${HAVE_BE_APP}" = "yes"` -
> the same test that `AC_DEFINE`s `HAVE_HAIKU` (3106-3110) - and `build/emacs-ref/src/Makefile`
> 378 reads `HAIKU_OBJ = ` (empty), so `docs/baseline/frozen-build-sources.txt` lists no
> `haikuterm.o` for this target and `extract-abi.py`'s checklist for the file is empty (P-86 ②).
> What follows is that clause's substitute: the **C line-range table**, the `## Evidence`
> readings, the `## Deviations` and the per-slice sections at this file's foot.  Port:
> `femacs/src/haikuterm.rs` (**2,485 lines**), which carries **C 1-1640** - P-86 ④'s first five
> slices of this file - with every `#if`'s arms both carried.
>
> **Why 1044 and not 1000 (slice 1).**  C 1000 is inside `haiku_draw_text_decoration`
> (C 887-1044); the nearest whole-item boundaries are this item's `}` at C 1044 (44 lines past
> 1000) and the previous item's `}` at C 885 (115 lines short), so the slice is **C 1-1044**.
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
| 1641-4898 | **the remainder of the file** (C 1642 `haiku_draw_image_relief` onward; C 4148 the `#if 0` block, C 4512 `ASSIGN_CURSOR`, C 4772/4895 the two `USE_BE_CAIRO` blocks) | not in this slice: recorded so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |

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
