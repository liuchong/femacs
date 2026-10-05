# emacs/src/xfont.c (1166 lines)

> **P-86's X11 font driver, ported whole.**  The frozen configuration compiles nothing here:
> `configure.ac` 7195-7197 adds `xfont.o` to `FONT_OBJ` only under `HAVE_X_WINDOWS`, that macro is
> `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1720, `FONT_OBJ=` is empty in
> `build/emacs-ref/src/Makefile` 342, and `docs/baseline/frozen-build-sources.txt` lists no
> `xfont.o`.  The file's own body sits under no `#if` of its own (the gate is the *build's* - the
> `lwlib-Xlw.c` shape), so `extract-abi.py`'s checklist is **not** empty and this ledger carries
> both P-86 ②'s substitute (the C line-range table) and the readings.  Port:
> `femacs/src/xfont.rs` (**1731 lines**, `grep -c ''`), both arms of every `#if` carried.
>
> This ledger replaces the previous revision's conclusion ("not compiled by the frozen
> configuration, so the port is the empty unit plus a note"): P-86 ① rules that a header comment is
> *not started*, not *done*.  The old measurements are still true and are still where the file gate
> comes from; what changed is the conclusion drawn from them.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`FONT_OBJ`, i.e. `HAVE_X_WINDOWS`; `configure.ac` 7195-7197) | `feature = "have_x_windows"` (on every item **and on the include graph**) | an X11 configuration |
| C 600-602 `#if defined GCC_LINT \|\| defined lint` | `feature = "gcc_lint"` (**live** here: `GCC_LINT` is 1, macros.tsv 1162) | a linting configuration |
| C 1006 `#if defined HAVE_XRENDER && (RENDER_MAJOR > 0 \|\| (RENDER_MINOR >= 2))` … C 1030 | `all (feature = "have_xrender", feature = "render_version_ge_0_2")` (**`render_version_ge_0_2` is not registered**; Owed 1) | an X11 configuration with Xrender >= 0.2 |
| C 1007-1029's `<X11/extensions/Xrender.h>` names (`XRenderColor`, `XRenderFillRectangle`, `render.h`'s `Picture`/`PictOpSrc`) | the same pair, on the unit-local X11 segment | the same |
| C 107-160's `body` (the file's own body: no `#if` of its own) | `feature = "have_x_windows"` (plus `feature = "have_window_system"`, which the file's own gate implies: `glyph_string` and `struct font`'s eight window-system members are behind it) | an X11 configuration |

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-20 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 21-24 | the blank and `<config.h>`, `<stdlib.h>`, `<X11/Xlib.h>` | file gate (`<X11/Xlib.h>` is the X11 SDK's: `include/xlib.rs`, plus the unit-local segment below for the font surface it lacks) |
| 25-33 | the blank and `"lisp.h"`, `"xterm.h"`, `"frame.h"`, `"blockinput.h"`, `"character.h"`, `"charset.h"`, `"font.h"`, `"pdumper.h"` | file gate |
| 34-44 | the blank at 34, the form feed at 35, "X core font driver." (36) and `struct xfont_info` (38-44) | file gate |
| 45-48 | the blank at 45, "Prototypes of support functions." (46) and C 48's forward declaration of `xfont_get_pcm` | file gate; the declaration is **dropped** (PORTING.md 0.1-12) |
| 49-51 | the blank at 49 and `xfont_get_pcm`'s comment | file gate |
| 52-116 | the blank at 52 and `xfont_get_pcm` (53-115) | file gate |
| 117-124 | the blank at 117 and `xfont_get_cache` (118-124) | file gate |
| 125-132 | the blank at 125 and `compare_font_names` (126-132) | file gate; the port's `unsafe extern "C" fn` (`qsort` takes its address, P-12) |
| 133-154 | the blank at 133, the comment (134-137) and `xfont_decode_coding_xlfd` (139-154) | file gate |
| 155-178 | the blank at 155, the comment (156-158) and `xfont_encode_coding_xlfd` (160-178) | file gate |
| 179-230 | the blank at 179, the comment (180-183) and `xfont_chars_supported` (185-230) | file gate |
| 231-239 | the blank at 231, the hash-table comment (232-237) and `xfont_scripts_cache` (239) | file gate |
| 240-242 | the blank at 240, the comment at 241 and `xfont_scratch_props` (242) | file gate |
| 243-246 | the blank at 243 and the comment (244-246) | file gate |
| 247-290 | the blank at 247 and `xfont_supported_scripts` (248-290) | file gate |
| 291-457 | the blank at 291 and `xfont_list_pattern` (292-457; C 344's bare `alloca` is the port's stack pool) | file gate; the `alloca` row is in `macro-body-notes.tsv` |
| 458-536 | the blank at 458 and `xfont_list` (459-536) | file gate |
| 537-590 | the blank at 537 and `xfont_match` (538-590) | file gate |
| 591-650 | the blank at 591 and `xfont_list_family` (592-650; C 600-602's `GCC_LINT` pair) | file gate; `gcc_lint` for C 601 |
| 651-862 | `xfont_open` (651-862) | file gate |
| 863-887 | the blank at 863 and `xfont_close` (864-887) | file gate |
| 888-896 | the blank at 888 and `xfont_prepare_face` (889-896) | file gate |
| 897-925 | the blank at 897 and `xfont_has_char` (898-925) | file gate |
| 926-945 | the blank at 926 and `xfont_encode_char` (927-945) | file gate |
| 946-987 | the blank at 946 and `xfont_text_extents` (947-987) | file gate |
| 988-1118 | the blank at 988 and `xfont_draw` (989-1118; C 1006-1030's Xrender arm) | file gate; `have_xrender` + `render_version_ge_0_2` for C 1007-1029 |
| 1119-1126 | the blank at 1119 and `xfont_check` (1120-1126) | file gate |
| 1127-1129 | the blank at 1127, the form feed at 1128 and C 1129's forward declaration of `syms_of_xfont_for_pdumper` | file gate; the declaration is **dropped** (0.1-12) |
| 1130-1147 | the blank at 1130 and `struct font_driver const xfont_driver` (1131-1147) | file gate |
| 1148-1161 | the blank at 1148 and `syms_of_xfont` (1149-1160) | file gate |
| 1162-1166 | `syms_of_xfont_for_pdumper` (1162-1166) | file gate |

## Evidence

**① The frozen configuration (`build-tu`), and the reading that put the gate on every item.**
The first run failed with 32 errors - `cannot find type c_uint`, `Lisp_Object`, `xfont_info`,
`cannot find macro eassert`, `cannot find function lisp_h_Qnil` - because the unit's includes are
gated while its `static` caches, its `xfont_info` layout asserts and `xfont_get_pcm` were not: the
names they need come from the gated graph.  That is P-86 ⑧'s measurement ("an include outside the
gate fails the frozen build at once"), read from the other side.  With every item gated:

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xfont-lane cargo run -q -p xtask -- build-tu src/xfont.c
built src/xfont.rs -> ../build/femacs/xfont.o          # exit 0
$ ls -l build/femacs/xfont.o                           # 336 bytes
```

**② The X11 arm (`xtask probe`, the arm the file gate names).**

```
$ cd femacs && CARGO_TARGET_DIR=/tmp/xfont-lane cargo run -q -p xtask -- probe src/xfont.rs \
      /tmp/xfont-lane/x.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="have_x_windows"' --cfg 'feature="have_window_system"' \
      --cfg 'feature="gcc_lint"'
9 errors; 0 in the unit's own text, 9 in the mirrors.
```

All nine are the same un-mirrored X11 name: `include/termhooks.rs` 110 (twice) and 259, and
`include/xterm.rs` 497, 501, 544, 669, 677, 1430 - `cannot find type Time in this scope` (`Time` is
`<X11/Xlib.h>`'s `unsigned long`; the xfns ledger's Owed 1 lists it as its own arm's largest class).

Re-read after the last three source edits (the `CDR` fix the token check found and two
comment-only citation fixes) at probe #4 - the lane's over-budget one (`## Not claimed`): **the same
9 errors at the same nine locations**, no in-unit error.

The three `--cfg` names are P-86 ⑥'s "the file gate *and* every sub-gate": `have_window_system` is
what the file's own gate implies (`WINDOW_SYSTEM_OBJ`; `glyph_string` and `struct font`'s eight
window-system members sit behind it) and `gcc_lint` is live in the frozen configuration (macros.tsv
1162), so C 601's arm is compiled.  `dispextern_face_records` is deliberately **not** set: with it
on, the mirror's `struct face` collides with this unit's local copy (E0428, `include/dispextern.rs`
2162 - status 3.220's recorded state, and the same reading the frozen configuration gives).

**③ The arm's iteration readings (both spent before the final one).**

```
probe #1  (before the fixes)                      37 errors: 9 in-unit, 28 in the mirrors
probe #2  (after the ABI/repr fixes)              26 errors: 17 in-unit,  9 in the mirrors
probe #3  (final)                                  9 errors:  0 in-unit,  9 in the mirrors
```

The in-unit classes probe #1/#2 found, all fixed in the unit (never in a mirror): `E0080` on
`offset_of! (xfont_info, font)` - the record was missing `#[repr(C)]`; `E0308`/`E0605` on the twelve
`font_driver` entries - a C-`static` helper is not an `extern "C" fn`, and a function *item* cannot
be cast to a fn-pointer type of another signature, which is why the seven entries whose C parameter
is `struct frame *`/`struct face *` take the mirror's untyped `*mut c_void` (P-72 ③, the
`androidfont.rs` precedent); `E0381` on `encoding` and on the two `SAFE_NALLOCA` buffers (C leaves
them uninitialised; the port's zero is PORTING.md 1.5); and the missing `XFree`, `x_mark_frame_dirty`,
`font_add_log`, `memcmp`/`memcpy`/`strcpy`, `xstrcasecmp` and `composition` declarations/includes,
which the unit now carries (Deviations 9-11).

**④ The XRender arm (a *local* reading: the same materialisation, plain `rustc`).**

```
$ cd femacs && rustc --edition 2024 --crate-type=lib --emit=metadata -C panic=abort \
      --cfg 'feature="have_x_windows"' --cfg 'feature="have_window_system"' \
      --cfg 'feature="gcc_lint"' --cfg 'feature="have_xrender"' \
      --cfg 'feature="render_version_ge_0_2"' build/probes/xfont/_root.rs
14 errors; 0 in the unit's own text, 14 in the mirrors.
```

The nine `Time` errors of Evidence ② plus five of `include/xterm.rs`'s own text:
`FRAME_CHECK_XR_VERSION!`'s body reads `xdisplay_supported_p`/`xrender_major`/`xrender_minor`, which
`struct x_display_info` does not carry (three errors, `xrender_major` twice), and `pict_format`'s
`XRenderPictFormat` (one).  `Picture` does **not** error there: this unit's own
`pub type Picture = XID;` supplies the mirror's field type too.  This is a diagnostic run, not an
`xtask probe` (budget, `## Not claimed`); the command and its cfg set are exactly the ones the
probe would use.

**⑤ The mutation test (P-86 ⑦: "0 errors in this unit" must be a real reading).**
Injecting one type error into the unit's own text in the materialised root -

```rust
  let _mutation: c_int = c"mutation";
```

inside `xfont_get_cache` - turns the arm's 9 errors into 10, with a new `E0308` reported at the
injected line.  So rustc does reach and check this unit's body in this arm, and the "0 in the unit's
own text" readings above are not an artifact of an include graph that stops before the unit.

**⑥ Layout measurements (clang, this machine's headers).**
`XCharStruct` 12/2 (`lbearing`@0, `rbearing`@2, `width`@4, `ascent`@6, `descent`@8,
`attributes`@10); `XFontProp` 16/8; `XFontStruct` 96/8 (`ext_data`@0, `fid`@8, `min_char_or_byte2`
@20, `min_byte1`@28, `max_byte1`@32, `properties`@48, `min_bounds`@56, `max_bounds`@68, `per_char`
@80, `ascent`@88, `descent`@92); `XChar2b` 2/1; `XRenderColor` 8/2; `XA_FONT` 18,
`XA_UNDERLINE_POSITION` 51, `XA_UNDERLINE_THICKNESS` 52, `PictOpSrc` 1 - all from
`clang -I/opt/homebrew/include` (`xorgproto` 2025.1, `libx11`) on a probe carrying exactly these
declarations, and asserted in the unit.

`struct face`, from `spikes/gated-layout-probe.py 'struct face' dispextern.h c --prelude lisp.h`:
frozen **280**/8 with `font`@216; the `HAVE_WINDOW_SYSTEM` + `HAVE_X_WINDOWS` arm **296**/8 with
`gc`@168, `stipple`@176, `font`@232 - the two numbers the unit's partial record pins.

`struct xfont_info`, from a clang probe over the reference's own headers
(`-DHAVE_WINDOW_SYSTEM=1 -DHAVE_X_WINDOWS=1 -I build/emacs-ref/src -I emacs/src`): **240**/8 with
`display`@216, `xfont`@224, `x_display_id`@232; the frozen arm is 208/8 with `display`@184.  The
unit asserts both through the same 32-byte shift `include/font.rs`'s `FONT_WS_SHIFT` records.

**⑦ The acceptance gates.**

```
$ python3 tools/extract-abi.py --check emacs/src/xfont.c femacs/src/xfont.rs
- line count drifted 48.5%: C 1166 vs Rust 1731        (one row, the tolerance row - Owed 3)
$ python3 tools/check-declarations.py                  # 0 findings
$ python3 tools/check-mirror-dupes.py                  # 17 duplicate(s), 17 ratchet row(s), 0 failure(s)
$ python3 tools/check-citations.py                     # 676 files, 0 drift, 0 range, 0 stale
$ python3 tools/check-dead-file-coverage.py            # the table above: 32 rows, gaps 0
```

The token check (`--check`'s other half) compares every C function's body tokens with the Rust
body's; it reported four rows, all of which the `macro-body-notes.tsv` additions cover (Owed 7).

## Deviations

1. **The licence block is not carried** (C 1-20, P-86 ① / PORTING.md 0.1-19): the port opens with
   `// femacs: port of src/xfont.c @ fd7afced403.` and carries no reference product name.
2. **C's two forward declarations are dropped** (C 48, C 1129; PORTING.md 0.1-12): Rust has no
   forward declaration and both definitions follow in the same unit.
3. **The file gate is the build's, not C's `#if`** (`configure.ac` 7195-7197): `feature =
   "have_x_windows"` is on every item *and* on the include graph (P-86 ⑧), which Evidence ①
   measures from both sides.  `have_window_system` is implied by it (the file's object lives in
   `WINDOW_SYSTEM_OBJ`) and is what `glyph_string`/`struct font`'s window-system members need.
4. **C 601's `GCC_LINT` arm is live** in this configuration (`GCC_LINT` is 1, macros.tsv 1162), so
   the `last_family = "";` line is transcribed under `feature = "gcc_lint"`; the file's other
   `#if` arms are noted at their rows.
5. **C 1006's condition is split**: `HAVE_XRENDER` is the registered feature `have_xrender`, and the
   version half (`RENDER_MAJOR > 0 || RENDER_MINOR >= 2`) is the derived
   `render_version_ge_0_2` - no `#[cfg]` can read `RENDER_MAJOR` out of `<X11/extensions/Xrender.h>`
   (the `xft_version_lt_2_3_6` shape, `femacs/xtask/Cargo.toml` 755).  It is **not registered**
   (Owed 1).
6. **The X11 surface stands in the unit** (P-88 ①): `<X11/Xlib.h>`'s four font records, twelve
   entry points and three atoms, and - under the pair above - `<X11/extensions/Xrender.h>`'s
   `XRenderColor`/`XRenderFillRectangle` and `<X11/extensions/render.h>`'s `Picture`/`PictOpSrc`.
   `include/xlib.rs` is the mirror for that header and this unit includes it at C 24; the names it
   does not carry are the segment.  Every layout and constant in it is a clang reading (Evidence ⑥).
   Owed 2 moves the segment into the mirror (P-90 ①/③).
7. **`struct face` is a unit-local partial record** (P-72 rule 3, the `dispnew.rs` shape): the
   mirror's copy is behind `dispextern_face_records` (status 3.220), and this unit reads exactly
   `face->gc` (C 892-895) and `face->font` (C 990), so only those two offsets are pinned - with the
   two clang readings of Evidence ⑥, not a full re-declaration.
8. **The seven `font_driver` entries whose C parameter is `struct frame *` or `struct face *` take
   the mirror's untyped `*mut c_void`** (`include/font.rs` types those members so, because
   `struct frame` need not be in an including unit's graph - its own note), and each use converts
   at the use point (P-72 ③, the `androidfont.rs` precedent: `FRAME_DISPLAY_INFO! ((f as *mut
   frame))`).  The other five entries keep C's parameter types exactly.  A function *item* cannot
   be cast to a fn pointer of another signature, which is why this shape, not a cast at the table,
   is the one that compiles.
9. **The twelve C-`static` driver members are `unsafe extern "C" fn`** (the mirror's fn-pointer
   types demand C's ABI), and being C-`static` they carry no `#[unsafe(no_mangle)]`: no exported
   symbol is invented.  `compare_font_names` is the same shape for `qsort`'s sake (P-12).
10. **Use-site declarations the mirrors do not carry**, each with its header's line: `XFree`
    (Xlib.h 2526), `qsort` (`<stdlib.h>` 489, the `buffer.rs` shape), `x_mark_frame_dirty`
    (xterm.h 1405 - `include/xterm.rs`'s own `FRAME_X_DRAWABLE!` body names it and no mirror
    declares it), `font_add_log` (font.h 1001 - `include/font.rs`'s `FONT_ADD_LOG!` body),
    `x_catch_errors`/`x_had_errors_p`/`x_clear_errors`/`x_uncatch_errors`/
    `x_display_info_for_display` (xterm.h), the five `font.c` entry points (`copy_font_spec`,
    `font_make_entity`, `font_make_object`, `font_registry_charsets`, `font_intern_prop`,
    `font_parse_xlfd`, `font_unparse_xlfd`, `register_font_driver`) and the four `HAVE_XRENDER`
    helpers (xterm.h 1826-1830, gated where C gates them).  Owed 4-6 name each mirror they belong
    in.
11. **Two includes the port's graph needs that C's `#include` list does not name**: `string.rs`
    (C's `xfont.c` reaches `<string.h>` through `lisp.h` 30 - `memcmp`, `memcpy`, `strcpy`) and
    `composite.rs` (`dispextern.h`'s `struct glyph_string.cmp` is typed `*mut composition` in the
    mirror, so the record's mirror has to be in this unit's graph - P-72 rule 4; C needs only the
    tag).  `dispextern.h`'s `INLINE int xstrcasecmp` lives inside `pub mod dispextern_externs`,
    which no glob import reaches, so the two calls are qualified
    (`dispextern_externs::xstrcasecmp`, the `ftfont.rs` shape).
12. **C's bare `alloca` (C 344) is the port's stack pool** (`sa_stack_alloc`, PORTING.md P-05), the
    `Freplace_region_contents`/`internal_lisp_condition_case` shape; the row that records it for the
    token check is in `docs/ports/macro-body-notes.tsv` (Owed 7).
13. **`xfont_driver` is `pub static mut`**: C's `struct font_driver const` object cannot be a Rust
    `static` (a record holding raw pointers is not `Sync`), so the const-ness is C's and this text's
    differs; nothing in the tree writes it.  `xfont_open` takes its address with
    `std::ptr::addr_of!` (the `ftfont.rs` 3070 note: `&raw const` hides the object from
    `extract-abi.py`'s static scan).
14. **C's integer promotions are spelled `as`** where Rust needs them (`(*pcm).width as c_int`,
    `(*xfont).min_bounds.width as c_int`, `XFIXNUM (val) / 10` narrowed to the `c_int` field, the
    `(long) value` folds of the five `XGetFontProperty` assignments), and C's mixed
    `sizeof "iso8859-1" - 1` comparison keeps its unsigned shape
    (`c"iso8859-1".to_bytes ().len ()`, PORTING.md 2.1).
15. **C's uninitialised locals take the port's zero** (PORTING.md 1.5): `encoding` in
    `xfont_has_char` (C passes its address to `font_registry_charsets`), `last_family` in the
    non-`GCC_LINT` arm (`xfont_list_family`), `xdi`, `num_fonts`, `names`, and the two
    `SAFE_NALLOCA` buffers (`char2b`).  C's `static XCharStruct *pcm;` in `xfont_text_extents`
    (C 953) is a plain local: a `static` local has no Rust counterpart and the value is written
    before every read.
16. **Three tokens have no Rust counterpart** and are recorded in `docs/ports/macro-body-notes.tsv`
    (Owed 7): `alloca` in `xfont_list_pattern`, and the `Display_Info` typedef token in
    `xfont_get_cache`/`xfont_list_family`/`xfont_open` (dispextern.h 115's
    `typedef struct x_display_info Display_Info;` - the mirror spells the tag, P-72 rule 3).
17. **The `face->gc`/`s->gc` comparison goes through `*mut c_void`**: the mirror types
    `glyph_string.gc` as the untyped pointer (its own note - Xlib's `GC` was not in a mirror when it
    landed), while this unit's `struct face.gc` is `include/xlib.rs` 892's `GC`; the comparison
    casts one side.

## Owed

1. **`femacs/xtask/Cargo.toml`**: register `render_version_ge_0_2` (the derived
   `RENDER_MAJOR > 0 || RENDER_MINOR >= 2`, `<X11/extensions/Xrender.h>`), the name C 1006's second
   operand needs and nothing else in the tree carries.  (The integrator registered the sibling
   derived names `xft_version_lt_2_3_6` 755 and the `sqlite_version_number_*`/`webkit_check_*`
   family; this one is new.)
2. **`femacs/src/include/xlib.rs`**: the X11 surface this unit declares at its use site (P-90 ①;
   that mirror already carries `XID` 837, `Atom` 844 and `GC` 892) - the four records
   (`XCharStruct`, `XFontProp`, `XFontStruct`, `XChar2b`), the twelve entry points
   (`XLoadQueryFont`, `XGetAtomName`, `XListFonts`, `XDrawImageString`, `XDrawImageString16`,
   `XDrawString`, `XDrawString16`, `XFree`, `XFreeFont`, `XFreeFontNames`, `XGetFontProperty`,
   `XSetFont`), the three atoms (`XA_FONT` 18, `XA_UNDERLINE_POSITION` 51,
   `XA_UNDERLINE_THICKNESS` 52 - `x11_constants` carries the other `XA_*` names) and, under
   `have_xrender`, `Picture`, `XRenderColor`, `PictOpSrc` and `XRenderFillRectangle`.  When they
   land, this unit's segment disappears in the same commit (P-90 ③ / P-88 ⑤).
3. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `xfont`
   (`xfont\t0.49\t<reason>\ttotal`), tolerance **>= 0.49**, basis `total`: the port carries 1731
   physical lines against C's 1166 (48.5%), because of the `// C <line>` citations P-55 asks for,
   the per-item `#[cfg (feature = "have_x_windows")]` of P-86 ①, the layout assertions, the
   unit-local X11 declaration segment and the seven-arm `xfont_draw`.  `extract-abi.py --check` reports exactly this row today
   (Evidence ⑦).  Not this lane's file to write.
4. **`femacs/src/include/xterm.rs`**: `Time` (nine errors of the X11 arm; `<X11/Xlib.h>`'s
   `unsigned long`, named by `struct input_event` 110, `struct terminal` 259, `x_display_info`
   497/501/544/669/677 and one more at 1430), and - for the XRender arm - `x_display_info`'s
   `xrender_supported_p`/`xrender_major`/`xrender_minor` (the mirror's own
   `FRAME_CHECK_XR_VERSION!` 1178 is `E0609` without them) and `XRenderPictFormat` (its
   `pict_format` field 398).  The xfns ledger's Owed 1 lists the same class.
5. **`femacs/src/include/xterm.rs` / `femacs/src/include/font.rs`**: `x_mark_frame_dirty`
   (xterm.h 1405) and `font_add_log` (font.h 1001) - the two names those mirrors' own macro bodies
   (`FRAME_X_DRAWABLE!` 974, `FONT_ADD_LOG!` 290) call and no mirror declares - together with
   `x_catch_errors`/`x_had_errors_p`/`x_clear_errors`/`x_uncatch_errors`/
   `x_display_info_for_display` (xterm.h) and the eight `font.c` entry points this unit declares at
   its use site (Deviations 10).
6. **`femacs/src/include/dispextern.rs`**: `struct face` and the unit-local copies (status 3.220).
   This unit follows the `dispnew.rs` shape (a partial record with the two offsets it reads); when
   the mirror's record becomes available to every unit, this copy needs the
   `#[cfg (not (feature = "dispextern_face_records"))]` the status note prescribes, together with
   the eight other units that carry a copy.
7. **`docs/ports/macro-body-notes.tsv`**: **written by this lane** (the one notes file the lane's
   brief allows, appended only): four rows - `xfont_list_pattern`/`alloca`, and
   `xfont_get_cache`/`xfont_list_family`/`xfont_open` for the `Display_Info` typedef token
   (Deviations 16).  The integrator should keep them as part of this unit's baseline rows; the
   `line-drift` row (Owed 3) is the other half of what `--check` needs.

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte empty object), so this is a *text* port with *compile* readings of its
  arms and no behaviour: nothing here has been executed, and no X11 display, X server or Xrender
  extension was involved.
- **The two arm readings are compile readings, not acceptance.**  Neither arm is clean: the X11 arm
  has 9 errors and the XRender arm 14, all in the mirrors this unit includes (the classes named in
  Owed 4/5), so no configuration on this machine compiles this unit end to end and none of the
  readings is claimed as a working configuration.
- **The XRender arm is a local reading, not an `xtask probe`.**  This lane's probe budget was spent
  on the file gate's own arm (Evidence ②/③, plus the reported overspend); the XRender arm and the
  mutation test ran the *same* materialised root with the *same* cfg set through plain `rustc` (a re-implementation of `xtask`'s `materialize`, verified
  byte-identical to `build/probes/xfont/_root.rs` outside the output-directory paths).  It is
  labelled as a diagnostic throughout.
- **`extract-abi.py --check` is not coverage evidence by itself.**  It reports the line-drift row
  (Owed 3) and nothing else; its token check is a token *set* comparison, so a reordered statement
  inside a body, a swapped operand of the same tokens, or a dropped `i += 1` that leaves the tokens
  intact would still pass.  The GPL header the file carried is not carried here (P-86 ①).
- **The layout asserts are this machine's readings.**  `xorgproto` 2025.1 and `libx11` under
  `/opt/homebrew/include`; the X11 SDK of another system may add members to `XFontStruct` (it is a
  public extension point - `ext_data` exists for that), and the port's asserts pin the X11R6-era
  record those headers declare.
- **Budgets, as run, with one overspend reported.**  `build-tu` **3 of 3** (Evidence ①'s failing
  reading, then the clean one, then a final-state re-run after the last edits: the same exit 0 and
  336-byte object, md5 `dbed68837694579185377dbb0821ab62`), `probe` **4 of 3** - one over, because
  probe #3 was taken three source edits before the final text (the `CDR` fix `extract-abi.py`'s
  token check found and two comment-only citation fixes) and probe #4 re-read the *final* text:
  the same 9 errors at the same locations.  The XRender arm,
  the mutation test and the local arm re-runs are outside both budgets and are reported as the
  diagnostics they are.  No `cargo test`, `--all`, `gate-fast` or `check-all-units.sh` was run; no
  git write was performed by this lane (the 4 `macro-body-notes.tsv` rows this lane appended were
  swept into another lane's commit `1445fbc` while this unit was in flight - reported, not a write
  by this lane).
- **`femacs/src/xfont.rs` may move under a later round** (the mirror gaps of Owed 2/4/5), and
  `docs/port-map.md`'s row and `docs/status.md`'s ratchet line are the integrator's to refresh.
