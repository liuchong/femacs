# emacs/src/w32term.c (8414 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32term.o` sits in
> `W32_OBJ` (`emacs/src/Makefile.in` 319, `W32_OBJ=@W32_OBJ@`, filled by `configure.ac` under the
> `HAVE_W32` test that also `AC_DEFINE`s `HAVE_NTGUI`), and `docs/baseline/frozen-build-sources.txt`
> lists no `w32term.o` for this target - so `extract-abi.py`'s checklist for the file is empty
> (P-86 ②) and this is that clause's substitute: the **C line-range table**, the `## Evidence`
> readings and the `## Deviations`.  Port: `femacs/src/w32term.rs` (**1592 lines**), which carries
> **C 1-1002** - P-86 ④'s first slice of this file - with **both arms of every `#if` carried**.
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
| 1004-8414 | **the remainder of the file** (C 1005 `w32_destroy_fringe_bitmap` onward: the Display Iterator section at C 1015, Line Dance at 3119, Exposure Events at 3239, Mouse Face at 3822, Tab-bars 3973, Tool-bars 3996, Scroll bars 4019/4160, `w32_read_socket` at 5131, Text Cursor 6462, X errors 6819, Input Methods 6894, Fonts 7737, Image Hooks 7809, `w32_initialize` 8155, `syms_of_w32term` 8273) | not in this slice: recorded here so the table covers the reference (`check-dead-file-coverage.py` requires it), to be replaced by the next lane's rows |

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
6. `include/w32term.rs`: `w32_output` needs the two bitfields' accessors (`paint_buffer_dirty`
   C 436, `want_paint_buffer` C 439), read at C 329/346/814/826 - the record comment says
   "recorded, not packed", and this unit is the first consumer that reads them.
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
14. The next lane's slice: **C 1003-8414**, whose first definition is `w32_destroy_fringe_bitmap`
    (C 1005-1013) and which also removes Evidence 4's `missing file-scope object` rows
    (`dbcs_lead`, `utf16_high`, `utf16_high_modifiers`, `temp_index`, `temp_buffer`,
    `w32_initialized`) and the two remainder macros (`LOAD_PROC`,
    `HIGHLIGHT_COLOR_DARK_BOOST_LIMIT`).
