# emacs/src/haikuterm.c (4898 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `configure.ac` 3097
> clears `HAIKU_OBJ` and 3114 fills it with `haikuterm.o` under `test "${HAVE_BE_APP}" = "yes"` -
> the same test that `AC_DEFINE`s `HAVE_HAIKU` (3106-3110) - and `build/emacs-ref/src/Makefile`
> 378 reads `HAIKU_OBJ = ` (empty), so `docs/baseline/frozen-build-sources.txt` lists no
> `haikuterm.o` for this target and `extract-abi.py`'s checklist for the file is empty (P-86 ②).
> What follows is that clause's substitute: the **C line-range table**, the `## Evidence`
> readings, the `## Deviations` and the per-slice sections at this file's foot.  Port:
> `femacs/src/haikuterm.rs` (**1,815 lines**), which carries **C 1-1198** - P-86 ④'s first two
> slices of this file - with every `#if`'s arms both carried.
>
> **Why 1044 and not 1000 (slice 1).**  C 1000 is inside `haiku_draw_text_decoration`
> (C 887-1044); the nearest whole-item boundaries are this item's `}` at C 1044 (44 lines past
> 1000) and the previous item's `}` at C 885 (115 lines short), so the slice is **C 1-1044**.
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
| C 1330 `#ifdef GCC_LINT` / `#else` | not in this slice (C 1045 onward) | - |
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
| 1199-4898 | **the remainder of the file** (C 1200 `haiku_draw_image` onward; C 1330 the `GCC_LINT` arm, C 4148 the `#if 0` block, C 4512 `ASSIGN_CURSOR`, C 4772/4895 the two `USE_BE_CAIRO` blocks) | not in this slice: recorded so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |

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
