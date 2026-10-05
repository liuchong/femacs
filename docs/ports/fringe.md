# emacs/src/fringe.c (1880 lines)

> **This file is P-86's unit: the frozen configuration compiles no `fringe.o`.**  `configure.ac`
> 7513-7521 defines `HAVE_WINDOW_SYSTEM` and puts `fringe.o` into `WINDOW_SYSTEM_OBJ` exactly when
> `$window_system != none` - `WINDOW_SYSTEM_OBJ="fontset.o fringe.o image.o"` is that `if`'s whole
> body - so the frozen configuration (darwin-arm64, no window system) has `WINDOW_SYSTEM_OBJ=` in
> `build/emacs-ref/src/Makefile` 304, `HAVE_WINDOW_SYSTEM` `undef` in
> `docs/baseline/config/darwin-arm64.macros.tsv` 1678, and `docs/baseline/frozen-build-sources.txt`
> lists no `src/fringe.c` at all.  `extract-abi.py`'s checklist for the file is therefore empty, so
> P-86 ②'s substitute is what follows: the **C line-range table**, the `## Evidence` readings and the
> per-`#if` `## Deviations`.  The old revision's conclusion ("not compiled by the frozen
> configuration, so the port is the empty unit plus a note", status 3.31) is withdrawn as a
> *conclusion* - its measurements still hold and are where the file gate comes from - and the text is
> carried **in full** (P-86 ①).
>
> Port: `femacs/src/fringe.rs`, **slice 1 = C 1-1364**, both arms of every `#if` carried.
> The slice ends on the `}` of `update_window_fringes` (C 1364), the file's last definition before
> the initialization section; **the next definition is `destroy_fringe_bitmap` (C 1370-1371)**.
>
> **Why 1364.**  P-86 ④ says to start at C 1 and stop on a whole-item boundary, and the target is
> roughly 1,000-1,400 C lines: the boundaries near it are C 1364 (`update_window_fringes`' `}`) and,
> one item earlier, C 1359 - the first `DEFUN` of the file is at C 1394, which would be past the
> target.  C 1-1364 also holds the file's five fringe-update entry points (`lookup_fringe_bitmap`,
> `draw_fringe_bitmap`, `draw_row_fringe_bitmaps`, `draw_window_fringes`, `update_window_fringes`)
> plus their two `static` helpers, i.e. the whole live path a fringe update takes, and it is what
> stops `extract-abi.py --check` being vacuous for the file (P-86 ⑩).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 7513-7521's `WINDOW_SYSTEM_OBJ`) | `feature = "have_window_system"` (on every item *and* on C 20-35's include graph) | any window-system build |
| C 33-35 `#ifdef HAVE_PGTK` + `pgtkterm.h` | `feature = "have_pgtk"` (registered) | a PGTK build |
| C 1433, 1448, 1498, 1517, 1865 (`HAVE_X_WINDOWS` / `HAVE_ANDROID` / `HAVE_PGTK` / `HAVE_NTGUI`) | `have_x_windows` / `have_android` / `have_pgtk` / `have_ntgui` (all registered) | slice 2 (C 1433+) |

No new feature name is needed: the file gate's four inner arms are all registered already.

## Decisions

- **The file gate is `have_window_system`, measured from the build list rather than inferred.**  The
  one `configure.ac` branch that puts `fringe.o` in an object list is 7513-7521, and its condition is
  `test "$window_system" != "none"` - the same condition that defines `HAVE_WINDOW_SYSTEM`.
  `HAVE_WINDOW_SYSTEM` is `undef` here, `WINDOW_SYSTEM_OBJ=` is empty in the target's `Makefile`
  (304), and `docs/baseline/frozen-build-sources.txt` has no `src/fringe.c`; `src/Makefile.in` 478
  lists `fringe.o` inside `$(WINDOW_SYSTEM_OBJ)`, and its 303/316 lines are comments about it.
- **The unit cross-checks the file gate against the reference's own `#if`s**: C 1433 says
  `#if defined (HAVE_X_WINDOWS) || defined (HAVE_PGTK) || defined (HAVE_ANDROID)`, C 1448
  `HAVE_X_WINDOWS || HAVE_ANDROID`, C 1498 `!HAVE_X_WINDOWS && HAVE_PGTK`, C 1517/1865 `HAVE_NTGUI` -
  every arm of the file's own body is one of the four window systems, i.e. the file's inner conditions
  are a subset of "a window system is configured".  All four are slice 2's.
- **The arm flip's feature set is `have_window_system` + `have_x_windows` + `dispextern_face_records`**,
  and the third is not this file's business: it is the arm `include/dispextern.rs` puts on the six
  records a unit declares itself (`face`, `bidi_it`, `draw_fringe_bitmap_params`, `composition_it`,
  `it`, `redisplay_interface`) - status 3.220 - and this slice uses two of them plus `struct face`.
  `have_x_windows` is the window-system arm this tree carries furthest (the `xterm.rs` /
  `w32term.rs` / `haikuterm.rs` shape); the C file's own body needs no X11 name in C 1-1364, so the
  choice is a *mirror-chain* choice, and Evidence ② names what each arm still owes.
- **The unit glob-imports `dispextern_unit_names::*`** (the `src/msdos.rs` 530 shape).  The mirror's
  records live inside that module, and this unit declares none of them, so the glob is what brings
  `draw_fringe_bitmap_params`, `redisplay_interface` and the two `bool_bf` accessors into scope.
- **`struct fringe_bitmap`'s five bitfields are one `u32` named `flags`.**  C's member name `bits` is
  the pointer's, so the run cannot reuse it (PORTING.md 1.3 says to name the run `bits`); the layout
  and the five members' positions are clang measurements (Evidence ⑤), and the read/write pair is the
  mirror-wide `*_bits!` shape.
- **`FRBITS` takes C's four remaining members.**  C's macro expands to two *members* of a struct
  initialiser and Rust has no macro in that position (measured in Evidence ⑧), so the port's macro
  answers the whole record; `bits` and the height it implies keep C's order and meaning.
- **The three `INLINE`/macro spellings the mirrors do not hand a unit** are this unit's own copies
  (`FACE_FROM_ID_OR_NULL`, `FRAME_FACE_CACHE!` and the `window_bits!` writer), which is the tree's
  rule for `INLINE` bodies with no external symbol (the `xdisp.rs` 984/1027 shape).  Each is named in
  Owed 3 with the mirror it belongs in.
- **The four edges C's chain reaches through headers the mirrors do not carry are gated and stand
  *before* C 33-35's.**  Their position is the fix for P-89's first-edge trap: `pgtkterm.rs` reaches
  the same four files and is itself gated, so an edge of this unit's placed after it (or left
  ungated) loses them (Evidence ①'s second and third readings).

## C line ranges

Rows are **contiguous and ascending** (each absorbs the blank lines that follow the one before) and
cover C 1-1880; `docs/port-map.md` 441 pairs only `src/fringe.c` with this unit, so one table is what
`check-dead-file-coverage.py` reads.  `transcribed` = in `femacs/src/fringe.rs`; `owed` = a later
slice.  The `arm` cell gives the port gate, which `## Gate mapping` derives.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the licence/declaration block (C 1-18) and its blank line | dropped (P-86 ①/0.1-19; the port opens with the `// femacs: port of …` line) |
| 20-35 | C 20's `#include <config.h>`, C 22's `<byteswap.h>`, C 24-31's nine headers (`lisp.h` … `pdumper.h`) and C 33-35's `#ifdef HAVE_PGTK` + `pgtkterm.h` | file gate (`have_window_system`); `have_pgtk` (C 33-35) - **transcribed** (plus the four edges C's chain reaches through headers the mirrors do not carry: character.h, systime.h, font.h, composite.h) |
| 36-69 | C 37-69's commentary on the three ways a fringe bitmap is represented | file gate - **transcribed** (verbatim) |
| 70-86 | `enum fringe_bitmap_align` (C 71-76) and `struct fringe_bitmap` (C 78-85) | file gate - **transcribed** (the bit run is `flags` and the accessors follow the record; Deviation 2) |
| 87-107 | C 89-91's banner, C 93-105's picture and `question_mark_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 108-123 | the picture and `exclamation_mark_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 124-137 | the picture and `left_arrow_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 138-152 | the picture and `right_arrow_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 153-167 | the picture and `up_arrow_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 168-182 | the picture and `down_arrow_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 183-196 | the picture and `left_curly_arrow_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 197-210 | the picture and `right_curly_arrow_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 211-224 | the picture and `large_circle_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 225-238 | the picture and `left_triangle_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 239-252 | the picture and `right_triangle_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 253-266 | the picture and `top_left_angle_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 267-280 | the picture and `top_right_angle_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 281-294 | the picture and `bottom_left_angle_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 295-308 | the picture and `bottom_right_angle_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 309-324 | the picture and `left_bracket_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 325-340 | the picture and `right_bracket_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 341-359 | the picture and `filled_rectangle_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 360-378 | the picture and `hollow_rectangle_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 379-390 | the picture and `hollow_square_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 391-402 | the picture and `filled_square_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 403-421 | the picture and `vertical_bar_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 422-429 | the picture and `horizontal_bar_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 430-450 | the picture and `empty_line_bits` (`static unsigned short []`) | file gate - **transcribed** (2-byte elements; `static`, so no symbol is exported) |
| 451-459 | C 453-455's `BYTES_PER_BITMAP_ROW` / `STANDARD_BITMAP_HEIGHT` / `FRBITS` and C 457-458's NOTE | file gate - **transcribed** (FRBITS takes C's four remaining members; Deviation 1) |
| 460-488 | `standard_bitmaps[]` (C 460-487) | file gate - **transcribed** (25 records = `NO_FRINGE_BITMAP` + 24) |
| 489-497 | C 489-491's `NO_FRINGE_BITMAP`/`UNDEF_FRINGE_BITMAP`/`MAX_STANDARD_FRINGE_BITMAPS` and C 493-497's four objects | file gate - **transcribed** (`max_used_fringe_bitmap` is non-`static`, so exported; the macro-body note carries the `countof` reading) |
| 498-520 | `lookup_fringe_bitmap` (C 503-520) with C 500-501's comment | file gate - **transcribed** (non-`static`) |
| 521-551 | `get_fringe_bitmap_name` (C 529-551) with C 522-527's comment | file gate - **transcribed** (`static`) |
| 552-566 | `get_fringe_bitmap_data` (C 555-566) with C 553's comment | file gate - **transcribed** (`static`) |
| 567-712 | `draw_fringe_bitmap_1` (C 575-712) with C 568-573's comment | file gate - **transcribed** (`static`; Deviations 3, 4, 7 and 10) |
| 713-735 | `get_logical_cursor_bitmap` (C 714-735) | file gate - **transcribed** (`static`; C 719/732's comma expressions, Deviation 5) |
| 736-834 | `get_logical_fringe_bitmap` (C 737-834), C 772/781/786/807/819/826's `goto found;` and C 832's `found:` | file gate - **transcribed** (`static`; the labelled block, Deviation 5) |
| 835-884 | `draw_fringe_bitmap` (C 837-884) | file gate - **transcribed** (non-`static`; Deviations 4 and 7) |
| 885-905 | `draw_row_fringe_bitmaps` (C 890-905) with C 887-888's comment | file gate - **transcribed** (non-`static`) |
| 906-963 | `draw_window_fringes` (C 919-963) with C 907-917's comment | file gate - **transcribed** (non-`static`; Deviation 6) |
| 964-1364 | `update_window_fringes` (C 965-1364), C 977's `#define MAX_BITMAP_CACHE (8*4)` and C 1068-1078's `LEFT_FRINGE`/`RIGHT_FRINGE` - **the slice's last row** | file gate - **transcribed** (non-`static`; Deviations 3, 7, 9 and 13) |
| 1365-1392 | C 1368's comment and `destroy_fringe_bitmap` (C 1370-1392) | file gate - **owed** (**the next slice starts at C 1370**) |
| 1393-1432 | `DEFUN ("destroy-fringe-bitmap", …)` (C 1394-1432) | file gate - **owed** (subr `Sdestroy_fringe_bitmap`) |
| 1433-1442 | C 1433's `#if defined (HAVE_X_WINDOWS) || defined (HAVE_PGTK) || defined (HAVE_ANDROID)` and `swap_nibble` (C 1434-1440) | `any (have_x_windows, have_pgtk, have_android)` - **owed** |
| 1443-1548 | `init_fringe_bitmap` (C 1443-1546), with C 1448-1496's `HAVE_X_WINDOWS || HAVE_ANDROID` arm (C 1452's `USE_CAIRO`, C 1456/1489's `WORDS_BIGENDIAN`), C 1498-1515's `!HAVE_X_WINDOWS && HAVE_PGTK` arm and C 1517-1529's `HAVE_NTGUI` arm | file gate; `any (have_x_windows, have_android)`; `use_cairo`; `windowsnt`; `have_ntgui` - **owed** |
| 1549-1684 | `DEFUN ("define-fringe-bitmap", …)` (C 1549-1683) | file gate - **owed** (subr `Sdefine_fringe_bitmap`; the dynamic-bitmap allocator) |
| 1685-1707 | `DEFUN ("set-fringe-bitmap-face", …)` (C 1685-1706) | file gate - **owed** (subr `Sset_fringe_bitmap_face`) |
| 1708-1750 | `DEFUN ("fringe-bitmaps-at-pos", …)` (C 1708-1749) | file gate - **owed** (subr `Sfringe_bitmaps_at_pos`; calls this slice's `get_fringe_bitmap_name`) |
| 1751-1783 | C 1751-1753's Initialization banner and `syms_of_fringe` (C 1755-1782) | file gate - **owed** (owns `Qtruncation`…`Qhollow_small`, `Vfringe_bitmaps`, `Voverflow_newline_into_fringe` and the four `defsubr`s) |
| 1784-1791 | C 1784's comment and `mark_fringe_data` (C 1786-1790) | file gate - **owed** |
| 1792-1801 | C 1792's comment, C 1794's `static` prototype and `init_fringe_once` (C 1796-1800) | file gate - **owed** (`pdumper_do_now_and_after_load`) |
| 1802-1808 | `init_fringe_once_for_pdumper` (C 1802-1807) | file gate - **owed** (`static`; waits on C 1443's `init_fringe_bitmap`) |
| 1809-1819 | `init_fringe` (C 1809-1818) | file gate - **owed** (`xzalloc` of the two tables) |
| 1820-1851 | `gui_init_fringe` (C 1820-1847) with C 1848-1851's comment | file gate - **owed** (the first user of the `redisplay_interface` hooks) |
| 1852-1864 | `gui_define_fringe_bitmap` (C 1852-1863) | file gate - **owed** (its ABI shim exists in `src/abi_shim.rs`, which must drop that copy in the same commit - Owed 3) |
| 1865-1880 | C 1865's `#ifdef HAVE_NTGUI`, `w32_reset_fringes` (C 1866-1878) and C 1880's `#endif` | `have_ntgui` - **owed** (the file's last row) |

## Findings

1. **P-89's first-edge trap bites this file, and its remedy is *position*, not an ungated edge.**
   `pgtkterm.rs` (C 33-35, gated) reaches `character.rs`, `systime.rs`, `font.rs` and `composite.rs`
   first, so the materializer wrote this unit's own edges to those four as
   `[xtask dedupe] ... already included` and expanded them *inside* `pgtkterm.rs` - which the arm's
   `have_pgtk` is off for.  Measured: the flip then reports `MAX_CHAR` (dispextern.rs 264),
   `Time` (termhooks.rs 110) and `font` (dispextern.rs 1616) missing.  Ungating the four edges was
   tried next and is **wrong**: the frozen build then stops being the empty object
   (`character.rs`'s own items need `lisp.rs`'s `eassert!`/`eassume!`, which the file gate keeps
   out).  Moving them *before* C 33-35's edge - gated like every other edge here - is what works
   (Evidence ①'s three readings).
2. **`max_used_fringe_bitmap` is an `E0428` in every window-system arm.**  C 497 defines it and
   C 3710 declares it `extern`, so the port exports it (PORTING.md 0.2) - and
   `include/dispextern.rs` 2661-2669 declares it at its *top level*, next to the other two globals
   of C 3617-3618.  A top-level `extern` declaration plus the unit's definition is
   `E0428: the name ... is defined multiple times` (the P-39 shape applied to a *data* symbol; the
   mirror's own note 3.220 puts the 102 names of this class in a module, and these three are not in
   one).  This is the same kind of blocker P-86's Android note records, and it is the integrator's
   to move (Owed 3).
3. **`glyph_row::right_fringe_offset` is a plain field in the mirror, not a `glyph_row_bits!` arm.**
   The mirror measures the eight-bit signed bitfield as a whole byte and carries it as `i8`
   (`include/dispextern.rs` 1002-1004), while `left_fringe_offset` shares `bits [5]` and does have an
   arm.  The unit's first flip found it (5 errors, four of them `no rules expected
   right_fringe_offset`), and the port's four uses are now field reads/writes with C's line at the
   site.
4. **Rust has no macro in struct-literal field position** (measured, Evidence ⑧): C's
   `FRBITS (bits)` expands to two members, so the port's `FRBITS` takes the four members C writes
   after it (Deviation 1).
5. **`FRAME_FACE_CACHE` and `FACE_FROM_ID_OR_NULL` are C macros/`INLINE`s with no external symbol**,
   and `include/frame.rs` carries them as functions inside `pub mod frame_face_accessors`, which no
   glob import reaches; other units already keep their own copies (`xdisp.rs` 984/1027, `window.rs`
   913, `font.rs` 274, `xfaces.rs` 445), so this unit keeps its two as well.
6. **The frozen object stays the 336-byte empty one with the four extra edges in place.**  They carry
   the file gate, so the frozen arm expands nothing but `include/config.rs` (P-86 ⑧'s judgement,
   Evidence ①a).
7. **`extract-abi.py --check` is not vacuous for the file any more** (P-86 ⑩): before this slice the
   port had no `fn` and the check printed `OK` on an empty list; this slice's reading is 17 `- ` rows
   and `FAIL`, and every one of them is the unported remainder (9 functions, 4 subrs, 4 `defsubr`s) -
   the named, mechanical statement of C 1365-1880.  No tolerance row is written for a slice (P-86 ⑩
   and this round's instruction).
8. **The four arms C 1433+ selects on would be *new* feature names if the file gate were wrong.**
   They are not needed: `have_x_windows`, `have_android`, `have_pgtk` and `have_ntgui` are registered
   (`docs/baseline/feature-coverage.tsv`, `femacs/xtask/Cargo.toml` 236-259), so this lane registers
   nothing - the integrator's list is the mirror one in Owed 3.

## Evidence

**① The frozen configuration (`build-tu`), three readings.**

    cd femacs && CARGO_TARGET_DIR=/tmp/fringe-lane cargo run -q -p xtask -- build-tu src/fringe.c

a) the last reading: `built src/fringe.rs -> ../build/femacs/fringe.o`, **exit 0**, and
`build/femacs/fringe.o` is **336 bytes**, md5 `dbed68837694579185377dbb0821ab62` - an empty object,
the same size and md5 the P-86 class's other units read (`docs/ports/androidfns.md` Evidence ①,
`docs/ports/ftfont.md` Evidence ①).  That is the expected reading and not a proof of the new text:
every item sits behind `have_window_system`.
b) the other two readings, in order: run 1 was a **syntax error** (a `? 1 : 0` left from C 672-685's
ternary - the port writes `if ... { 1 } else { 0 }`), and the run after the un-gated-edge experiment
was **exit 1** with `character.rs` 141/220/244/386/399's `eassert!`/`eassume!` unresolvable -
Finding 1's counterfactual, and the reason the four edges are gated.  **Budget, reported rather than
kept: five `build-tu` runs (budget: three)** - the parse error, the first success, the run that
materialised the fixed text, the un-gated counterfactual, and the final reading.

**② The arm flip.**

    cd femacs && CARGO_TARGET_DIR=/tmp/fringe-lane cargo run -q -p xtask -- probe \
        src/fringe.rs /tmp/fringe-lane/fringe_arm.rlib --crate-type=lib -C panic=abort \
        --cfg 'feature="have_window_system"' --cfg 'feature="have_x_windows"' \
        --cfg 'feature="dispextern_face_records"'

-> **exit 1, 38 errors, 3 warnings**, and the error classification (P-86 ⑦) is
**0 rows in this unit's own text** (`build/probes/fringe/_root.rs`), **38 rows in the include graph**:

| file | rows | classes |
| --- | --- | --- |
| `_..._include_dispextern_rs.rs` | 29 | 1 `E0428` (`max_used_fringe_bitmap`, Finding 2), 3 `E0425` (`x_display_info` 685, `Emacs_Pix_Container` 760, `Emacs_Cursor` 3739), 13 `E0080` (`face`'s own `sizeof`/`offset_of` assertions, 2188-2216), 3 `E0425` (`estimate_mode_line_height`, 1265/1295/1325 - it is `xdisp.rs`'s export) |
| `_..._include_font_rs.rs` | 8 | 8 `E0080` (`font`'s two and `font_driver`'s six assertions, 137/149/584-598) |
| `_..._include_systime_rs.rs` | 1 | 1 `E0080` (`!cfg!(feature = "have_x_windows") || HAVE_X_WINDOWS`, i.e. the feature and `config.rs`'s constant must move together) |

The 3 warnings are mirrors' too (`lib/include/stdbit.in.rs` 644, `include/frame.rs` 1360,
`include/lisp.rs` 3934 - all pre-existing; the `gnutls`/`ftfont` ledgers name the first two).  So the
flip's own-text reading is real, and the three files above are what stands between it and exit 0; each
is named in Owed 3.

**③ The direct `rustc` iterations behind ② (the pinned nightly, run against the materialised root).**
The repository's own toolchain is `femacs/rust-toolchain.toml`'s `rustc +nightly-2026-10-01`; the
materialiser's `_root.rs` can be compiled directly, and that is where the per-class counts above come
from.  Nine such runs in four groups: (a) two on the first arm set (`have_window_system` +
`dispextern_face_records`, **86 errors**, all mirror-side except the two this unit then fixed);
(b) two on the X11 arm before the four edges were repositioned (**49 then 38 mirror rows**, plus this
unit's two `FRAME_FACE_CACHE` rows); (c) one with `--cfg 'feature="harfbuzz"'` added (**38 as well** -
that feature changes none of the three files' rows, so the arm set stays minimal); (d) two for the
mutation test below.  The two `probe` runs before the final one are counted separately: run 1 was
**5 errors**, four of them `no rules expected right_fringe_offset` (Finding 3), and run 2 was the
**86-error** reading of (a).  **Budget, reported rather than kept: three `probe` runs (budget: three,
the last of them the final reading)** plus these nine direct `rustc` runs and the two scratch probes
of ⑧.

**④ The mutation test (P-86 ⑦).**  With
`let _mutation: c_int = c"injected type error".as_ptr ();` injected as `draw_fringe_bitmap`'s first
statement after `let mut overlay: c_int = 0;`, the same arm compiles to **39 errors** (38 + 1) and the
new row is `E0308: mismatched types` **at the injected line in this unit's own text** - so the compile
reads this unit's text and ②'s "0 rows in its own text" is a measurement, not an empty reading.

**⑤ The clang probe the `struct fringe_bitmap` layout rests on** (P-88 ②: measured, not remembered).
The program is `/tmp/fringe-probe/layout.c` (C 78-85's record copied verbatim, `bool_bf` = `bool` as
`conf_post.h` 58-62 spells it on this target); command `clang -std=gnu23 -o layout layout.c &&
./layout`; output, verbatim: `sizeof fringe_bitmap 16 align 8`; `offsetof bits 0`; `height=0xAB:
ab 00 00 00`; `width=0xCD: 00 cd 00 00`; `period=0xEF: 00 00 ef 00`; `align=3: 00 00 00 03`;
`dynamic=1: 00 00 00 04`.  So the run is the four-byte unit at offset 8 with `height` bits 0-7,
`width` 8-15, `period` 16-23, `align` 24-25 and `dynamic` bit 26, and the record has four bytes of
trailing padding - the constants and the three assertions in the unit are that reading.

**⑥ Inventory (`extract-abi.py --check`).**
`python3 tools/extract-abi.py --check emacs/src/fringe.c femacs/src/fringe.rs` -> **FAIL, 17 `- `
rows**: **9 `missing function`** (`destroy_fringe_bitmap`, `gui_define_fringe_bitmap`,
`gui_init_fringe`, `init_fringe`, `init_fringe_bitmap`, `init_fringe_once`,
`init_fringe_once_for_pdumper`, `mark_fringe_data`, `syms_of_fringe` - the unported C 1365-1880),
**4 `missing subr`** (`Fdestroy_fringe_bitmap`, `Fdefine_fringe_bitmap`, `Fset_fringe_bitmap_face`,
`Ffringe_bitmaps_at_pos`) and **4 `missing defsubr`** (`Sdefine_fringe_bitmap`,
`Sdestroy_fringe_bitmap`, `Sfringe_bitmaps_at_pos`, `Sset_fringe_bitmap_face`).  The reading before
this lane's six `macro-body-notes.tsv` rows was **23 rows**: the same 17 plus 6 token rows
(`MAX_STANDARD_FRINGE_BITMAPS` `countof`/`standard_bitmaps`; `draw_fringe_bitmap_1`
`break`/`case`/`cursor_p`/`overlay_p`/`switch`; `get_logical_fringe_bitmap` `goto`;
`draw_fringe_bitmap` `break`/`case`/`default`/`switch`; `draw_window_fringes` `pseudo_window_p`;
`update_window_fringes` `pseudo_window_p`/`rows`/`uninit`), all six now recorded with their reason.

**⑦ The gates this lane ran.**  `python3 tools/check-dead-file-coverage.py` -> `68 ledger(s) bound (a C
range table), 0 failure(s)` (69 of them with this ledger bound), and its own line for this ledger is
`info docs/ports/fringe.md: 56 range row(s) cover src/fringe.c (1880 lines; 0 blank line(s) between
blocks need no row)`.  - `python3 tools/check-mirror-dupes.py` -> `17
duplicate declaration(s), 17 ratchet row(s), 0 failure(s)`, no row naming `femacs/src/fringe.rs`. -
`python3 tools/check-declarations.py` -> `10 declaration sites (0 forbidden, 10 allowed literals), 0
failures` (exit 0).  - `python3 tools/check-citations.py` -> `676 file(s) scanned, 0 drift, 0 range,
1044 external, 35 allowed, 0 stale` (the `external` tally moved by two while this lane worked - other
lanes' concurrent mirror edits; every one of the four readings 1042/1044 has 0 drift, 0 range and 0
stale, and neither names this unit), and no row names this unit: its `/// C:` lines cite the reference's
own headers (`dispextern.h` 3577/3579/3581/3810/3812/3816, `frame.h` 1379/1656, `termhooks.h` 110,
`character.h` 29, `conf_post.h` 58), which live in the reference tree, and the checker classifies a
path it cannot resolve as `external`, never a failure.  - `python3 tools/check-port-lines.py` -> `51
file(s) under ten percent, 67,241 reference line(s); 51 ratchet row(s), 0 new, 0 stale`, with **no
`src/fringe.c` row**: the file has left P-86's under-ten-percent class (1,895 port lines against the
188-line threshold `docs/baseline/port-lines-pending.tsv` used to carry).  That row's disappearance is
not this lane's write - `docs/baseline/**` is outside this lane's scope (the file's own header says
`--write-pending` writes it, and this lane never passed that flag).

**⑧ The two scratch probes behind Findings 1 and 4** (in `/tmp/fringe-lane`, removed with the lane's
build directory).  (a) `S { mac!() }` with a macro expanding to two struct members -> `expected one of
`,`, `:`, or `}`` - Rust has no macro in that position, so C's `FRBITS` must change arity.  (b) A
`static mut` array of records initialised from `core::mem::size_of_val_raw (core::ptr::addr_of!
(bits)) / size_of::<c_ushort> ()` plus `core::ptr::addr_of_mut! (bits) as *mut c_ushort` -> **exit
0**, and the measured record for the 10-element array is `2058` = `0x080A`, i.e. height 10 (bits 0-7)
and width 8 (bits 8-15) - C's `{ FRBITS (question_mark_bits), 8, 0, ALIGN_BITMAP_CENTER, 0 }` exactly,
and the reason `STANDARD_BITMAP_HEIGHT` keeps C's division while its `sizeof` becomes
`size_of_val_raw` (a static initialiser cannot take `&` of a `static mut`).

## Deviations

1. **`FRBITS` takes C's four remaining members** (`($bits, $width, $period, $align, $dynamic)`) and
   answers the whole record, because Rust has no macro in struct-field position (Evidence ⑧a, Finding
   4).  `bits` is `addr_of_mut!` plus the element-pointer cast - C's array-to-pointer decay - and the
   height is C's own `STANDARD_BITMAP_HEIGHT`.
2. **`struct fringe_bitmap`'s bitfield run is named `flags`**, not `bits`: C's first member owns that
   name (PORTING.md 1.3 names the run `bits`, which is taken here).  The five members keep their own
   names through `fringe_bitmap_bits!` and the shift/mask constants, and the layout is asserted.
3. **C's two `#define`d macros inside `update_window_fringes` are `macro_rules!` whose `else` arm
   assigns and then reads the slot back.**  C's `(bitmap_cache[...] = f (...))` yields the assigned
   value and a Rust assignment yields `()`; the spellings keep C's three evaluations of
   `cache`/`partial_p` and the same answer.  `MAX_BITMAP_CACHE` is the local `const` that sizes the
   array.
4. **C's `switch`es are `match`es.**  `draw_fringe_bitmap_1` C 630-640's switch on `fb->align` needs
   the `_ => {}` arm a `c_int` match requires (C has no `default`); `draw_fringe_bitmap` C 846-868's
   `case NO_CURSOR: default:` shared body is written twice (Rust has no fall-through into a following
   label), and neither leaves a `break`/`default` token - both rows are in
   `docs/ports/macro-body-notes.tsv`.
5. **C's comma expressions that assign and test are two statements.**  `(cmap = ..., !NILP (cmap))`
   (C 719), `((bm = XCDR (bm)), NILP (bm))` (C 724/732), the same in `get_logical_fringe_bitmap`
   (C 756/761/785/796/829) and `(ind = ..., !NILP (ind))` (C 1000); the `ind` site is the one whose
   assignment moved *out* of the condition.  C's `found:` label is a labelled block with six
   `break 'found` (PORTING.md 2.2).
6. **C's `for` loops that increment on `continue` are `while` loops with the increment at both
   sites** (C 1026-1028, 1202-1204; the `ftfont.rs` Deviation 15 shape), and C 1127-1129/1184-1186's
   reverse and second loops the same way.  C 1028/1204/1129/1186's `y += row->height` uses the row the
   body left behind, which is why the increment is written after the body rather than in a `for`
   clause.
7. **Every bitfield access goes through the tree's accessor spellings** (`glyph_row_bits!`,
   `window_bit`, the unit's `window_bits!` writer, `set_overlay_p`/`set_cursor_p`), with the measured
   shift/mask and the `bool`-typed members answering `bool`.
8. **`row->right_fringe_offset` is a plain `i8` field read/write**, not a `glyph_row_bits!` arm: the
   mirror measures the eight-bit signed bitfield as a whole byte and carries it that way (Finding 3).
   `left_fringe_offset` does have an arm and is used.
9. **C 986's `UNINIT` pair is zero-initialised** (`top_row_ends_at_zv_p`, `bot_row_ends_at_zv_p`),
   because Rust's definite-assignment analysis cannot see the property C 982-985's comment states
   (PORTING.md 0.1-13/1.5); C's comment is carried verbatim above them.
10. **C's `0` for a `bool` parameter is `false`**: `lookup_derived_face (w, f, face, FRINGE_FACE_ID,
    false)` (C 609).  Everything else C spells `0`/`1` as a boolean is `false`/`true`.
11. **The comparisons against `unsigned` locals take casts** (`(*cur).right_fringe_offset as c_int`,
    `left_face_id as c_int` on the write side) and `lookup_fringe_bitmap` returns `bn as c_int` where
    C converts the `EMACS_INT` implicitly; `max_used_fringe_bitmap`/`MAX_STANDARD_FRINGE_BITMAPS`
    likewise compare as `EMACS_INT`.
12. **`FRAME_RIF (f)->draw_fringe_bitmap (...)` is the mirror's `Option<...>` hook**: the port casts
    `FRAME_RIF! (f)` to `*mut redisplay_interface`, unwraps the field and passes `w as *mut c_void`
    for the hook's first parameter (the mirror spells the hooks' window parameters `*mut c_void`).
13. **Two locals C reassigns are `mut`, and the three `w->current_matrix as *mut glyph_matrix` reads
    keep the `xdisp.rs` parentheses** (`(*( (*w).current_matrix as *mut glyph_matrix ))`) - `*A as T`
    parses as `(*A) as T`, so the cast must be parenthesised before the deref.
14. **The four edges of C's chain that the mirrors do not carry** (`character.rs`, `systime.rs`,
    `font.rs`, `composite.rs`) stand in the unit, gated, before C 33-35's edge (P-72 rule 4's
    use-site shape and P-89's position rule; Finding 1).
15. **The unit keeps three copies the mirrors spell differently**: `FRAME_FACE_CACHE!` (the mirror has
    a function of the same name in a module), `FACE_FROM_ID_OR_NULL` (idem) and one `window_bits!`
    writer arm (the mirror has `window_bit` for reads only).  Each is the tree's per-unit copy shape
    for an `INLINE`/macro with no external symbol.

## Owed by the integrator (nothing here is claimed as done)

1. **`docs/ports/line-drift-tolerance.tsv`: nothing to write.**  The unit is **1,895 lines** against C's
   1,880, i.e. *over*, and `extract-abi.py --check` asks for no tolerance row; its 17 `- ` rows are the
   unported remainder's, which is a different statement and stays visible.
2. **`docs/baseline/port-lines-pending.tsv`: nothing to retire.**  The file has left P-86's
   under-ten-percent class (1,895 lines against a 188-line threshold) and the ratchet already reads
   `0 new, 0 stale` (Evidence ⑦).  Named because P-86 ④'s closing step asks for the reading, not
   because a row moved.
3. **The mirror items the flip names**, each in the file that owns it:
   (a) `femacs/src/include/dispextern.rs` 2661-2669: the three globals of C 3617-3618/3710
   (`last_tab_bar_item`, `last_tool_bar_item`, `max_used_fringe_bitmap`) sit at the mirror's top
   level, so this unit's C-faithful definition is an `E0428` (Finding 2).  They belong in a module the
   including units do not glob-import - the shape the mirror's own note (3.220) uses for its other 102
   names;
   (b) the same mirror's arm: `x_display_info` (685), `Emacs_Pix_Container` (760) and `Emacs_Cursor`
   (3739) have no definition in the `have_window_system` + `have_x_windows` set, and `struct face`'s
   thirteen `sizeof`/`offset_of` assertions (2188-2216) fail in it - i.e. the record's arms need the
   arms' own typedefs (the mirror's 700-702 note records the two deferred ones);
   (c) `femacs/src/include/font.rs`: `font`'s two and `font_driver`'s six assertions (137/149/584-598)
   fail in the same set (3.496/3.497 were the last to touch that record);
   (d) `femacs/src/include/systime.rs` 27: `!cfg!(feature = "have_x_windows") || HAVE_X_WINDOWS` - a
   flip of `have_x_windows` must also move `config.rs`'s constant, which is the integrator's
   (`docs/baseline/feature-coverage.tsv`'s "config constant" column is what says so);
   (e) `femacs/src/include/dispextern.rs`'s X11 arm has no `estimate_mode_line_height` edge either -
   its definition is `src/xdisp.rs`'s export (three rows of the flip's 29).
4. **The two ABI-shim copies that must go with C 1852's row** (slice 2): `src/abi_shim.rs` 2214-2260
   defines `gui_define_fringe_bitmap` and 2533-2552 defines `draw_fringe_bitmap` as empty
   `#[unsafe(no_mangle)]` bodies *because* this unit did not exist, and 1390-1403 defines
   `max_used_fringe_bitmap = 25` for the same reason.  This slice now defines the third of those
   (C 497) and, in slice 2, the other two; the shim's copies are duplicates and belong to whoever lands
   the slice that collides with them (P-88 ⑤'s move-back).
5. **The six `docs/ports/macro-body-notes.tsv` rows this slice needs** - **written** (that file is in
   this lane's scope), one per `goto`/`switch`/bitfield/`countof` deviation, each naming the C line and
   the reason the token cannot reappear.
6. **The next slice**: **C 1370's `destroy_fringe_bitmap`**, then C 1394 `Fdestroy_fringe_bitmap`
   (`DEFUN`), C 1443 `init_fringe_bitmap` (with the four window-system arms the table lists), C 1549
   `Fdefine_fringe_bitmap`, C 1685 `Fset_fringe_bitmap_face`, C 1708 `Ffringe_bitmaps_at_pos` (the
   only caller of this slice's `get_fringe_bitmap_name`), C 1755 `syms_of_fringe` (which owns
   `Vfringe_bitmaps` and `Voverflow_newline_into_fringe`, the two `DEFVAR_LISP` variables
   `check-globals.py` will then require definitions for) and C 1786-1878's five initializers.
7. **No feature name to register** (Finding 8): the file gate and its four inner arms are registered
   already, so the integrator's list is Owed 3's mirror items, not `femacs/xtask/Cargo.toml`.

## Not claimed

- **No behaviour.**  The frozen configuration compiles nothing from this file (the 336-byte object is
  empty), and the flip reading is a *compile* that stops with 38 mirror-side errors: no bitmap was
  looked up, drawn, aligned, extended or culled, and no face, window or row was read.
- The arm flip is `have_window_system` + `have_x_windows` + `dispextern_face_records` (Evidence ②).
  It is not a link, not a load, and not a P-86 ③⑴ "the class can be verified" claim: three mirror
  files stop it, and the four arms C 1433+ selects on are **not compiled at all** (they are slice 2's).
- The layout and constants in Evidence ⑤ are this machine's clang readings of C 78-85's own text; the
  frozen configuration compiles no such record, so nothing here is a claim about another target.
- C 1365-1880 is unported: nothing in this ledger speaks for those 516 lines, which are exactly the
  9 `missing function`, 4 `missing subr` and 4 `missing defsubr` rows of Evidence ⑥.
- Only `femacs/src/fringe.rs`, `docs/ports/fringe.md` and six rows of
  `docs/ports/macro-body-notes.tsv` were written by this lane.

## 2026-10-05: the arm's `estimate_mode_line_height` was the calling unit's decl (3.505)

`estimate_mode_line_height` (C 3582) is declared by the *units that call it*, not by the mirror:
the mirror cannot reach `struct frame` (P-72 rule 3), so a copy there would take an untyped
pointer and every call site would need a cast - the reading `src/dispnew.rs` 256,
`src/keyboard.rs` 603, `src/menu.rs` 99 and `src/window.rs` 179 already follow.  This unit's own
`unsafe extern "C"` block (the one that declares `window_text_bottom_y`/`lookup_named_face`) now
carries it with C's line, so the flip no longer reports it.  Same arm, in order (all three
readings on the command in this ledger's `## Evidence`): `38` errors when this unit landed,
`24` after the two `include/dispextern.rs` fixes of status 3.505, `13` after this decl - and the
13 are all in other files (`include/font.rs` 8 x `E0080`, `include/systime.rs` 1 x `E0080`, and
the four deferred X11 typedefs of `include/dispextern.rs`).
