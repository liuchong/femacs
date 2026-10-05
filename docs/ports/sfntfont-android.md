# emacs/src/sfntfont-android.c (821 lines) — whole file, C 1-821

> **P-86's unit `src/sfntfont-android.c`: the frozen configuration compiles nothing here.**
> `configure.ac` 2645 adds `sfntfont-android.o` to `ANDROID_OBJ` only inside the `XCONFIGURE =
> android` branch (the `REALLY_ANDROID` one, 2620-2654), and 2605-2606 defines `ANDROID_STUBIFY` in
> the other one, so `docs/baseline/frozen-build-sources.txt` lists no `sfntfont-android.o` and
> `tools/extract-abi.py`'s checklist for the file is empty (P-86 ②).  This ledger is that clause's
> substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.
> Port: `femacs/src/sfntfont-android.rs` (**1005 lines**, md5 `5110a27299d16dfec9d5bff292653630`),
> carrying **C 1-821 with both arms of every `#if`**.  The reference `emacs/src/sfntfont-android.c`
> is md5 `bd161722c271c80bff8483c670b0dfab` (unchanged; `emacs/` is read-only) and
> `docs/port-map.md` 526 pairs the two (that row's port-side count is the integrator's
> regeneration — Owed 2).
>
> **This ledger replaces the reading it inherits.**  The previous revision (and the 11-line comment
> unit it described) read "the frozen configuration does not compile this file, so the port is the
> empty unit plus the note that names the gate" (P-08, `docs/status.md` 3.31).  P-86 ① replaces that
> conclusion with *transcribe the text, both arms, no stub*; what the earlier note measured (not in
> the frozen source list, empty checklist, 336-byte `build-tu` object) is unchanged and still true —
> it is the *conclusion* that is withdrawn, as `docs/ports/w32inevt.md`, `docs/ports/sfntfont.md`
> and `docs/ports/androidselect.md` record for their own pairs.
>
> **The file gate is `have_android` + `not_android_stubify`, and the arm set is those two plus
> `aarch64` and `have_window_system`.**  `HAVE_ANDROID` is defined in both Android branches
> (2594-2602) and `ANDROID_STUBIFY` only in the branch 2645 is *not* in, so this file's build
> condition is C's `HAVE_ANDROID && !defined ANDROID_STUBIFY` (the `androidselect.rs`/`androidvfs.rs`
> reading).  `__aarch64__` is 1 in the frozen macro table (`darwin-arm64.macros.tsv` 7020), so the
> six `__aarch64__` pairs are `feature = "aarch64"` / `not (feature = "aarch64")` and the *live* arm
> of each is the NEON one; `have_window_system` is in the arm reading because the `font_driver`
> record the port writes holds `draw`/`get_bitmap`/`free_bitmap` only under that condition
> (font.h 676-698, `docs/status.md` 3.516's correction).
>
> **Acquisition: this piece is not `text-only` — the machine has the NDK.**  The task premise "no
> Android NDK on this machine" was checked and is **wrong** here: `~/Library/Android/sdk/ndk/`
> holds six NDKs (25.1.8937393 … 29.0.14206865), with the
> `aarch64-linux-android<api>-clang` drivers and the full `sysroot/usr/include/android/`.  So the
> arm flip (P-86 ③⑴) was run and reached a real reading (Evidence ②), and every record and constant
> this file declares in place was **measured** with the NDK's own clang (Evidence ③) rather than
> recorded as `UNVERIFIED`.  What is still missing is a *device*: nothing here was linked against
> `liblog`/`libjnigraphics`, installed on Android or run (see `### Not claimed`).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2645, the `XCONFIGURE = android` branch; `ANDROID_STUBIFY` is defined in the other one, 2605-2606) | `all (feature = "have_android", feature = "not_android_stubify")` (on every item *and* on C 20-37's include graph) | a *real* Android configuration |
| C 26 `#ifdef __aarch64__` … C 28 `#endif` (`#include <arm_neon.h>`) | carried as a comment: the intrinsics come from `core::arch::aarch64` on the C 195-309 items (the file note) | never as text; the arm it serves is C 195-309's |
| C 74 `#ifndef __aarch64__` … C 107 `#else` … C 138 `#endif` (`GET_SCANLINE_BUFFER`, two definitions) | `not (feature = "aarch64")` / `feature = "aarch64"`; **both arms carried** (P-86 ①) | one arm per configuration |
| C 195 `#ifdef __aarch64__` … C 309 `#endif` (the three NEON functions) | `feature = "aarch64"` | an aarch64 configuration (the frozen one) |
| C 250 `#if 0` … C 256 `#else` … C 260 `#endif` (inside `sfntfont_android_over_8888_1`) | **no gate**: dead in every configuration; C's arm is kept as a comment (the `w32inevt.md` C 120 reading) | never |
| C 326-328 `#ifdef __aarch64__` (the `lim_x` local), C 361-375 (`#ifdef __aarch64__` inside the pixel loop) | `feature = "aarch64"` / `not (feature = "aarch64")` | one arm per configuration |
| C 490 `#ifndef __aarch64__` … C 492 `#else` … C 494 `#endif` (the two `stride` expressions) | `not (feature = "aarch64")` / `feature = "aarch64"`; both carried | one arm per configuration |
| C 666 `#ifdef HAVE_HARFBUZZ` … C 673 `#endif` (the four driver members) | `feature = "harfbuzz"` (`femacs/xtask/Cargo.toml` 141; `HAVE_HARFBUZZ` is `undef` in the frozen table) | a HarfBuzz build |
| C 609 `#undef MAX_ALLOCA` | no counterpart: `MAX_ALLOCA` is lisp.h 5616's *enum* constant in both languages, so C's `#undef` is a no-op (Deviations 10) | n/a |

`tools/check-cond-eval.py src/sfntfont-android.c` reads **`8 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence ④): the eight are C 26, 74, 195, 250, 326, 361, 490 and 666 — the whole
conditional text of the file.  `ANDROID_STUBIFY` has **no `#if` in this file** (the gate is
`configure.ac`'s) and the tree has not registered it as a feature; `not_android_stubify`
(`femacs/xtask/Cargo.toml` 643) is C's `HAVE_ANDROID && !defined ANDROID_STUBIFY` as the tree spells
it, and the arm flip therefore *sets* it (P-86 ⑥: a single `have_android` cfg would have selected
nothing at all).

## C line ranges

One row per C block, in C's order, contiguous (each row absorbs the blank lines that follow the one
before), covering C 1-821.  `docs/port-map.md` 526 pairs only `src/sfntfont-android.c` with this unit
(there is no `src/sfntfont-android.h` binding), so one table is what
`tools/check-dead-file-coverage.py` reads.  "The port" column names the Rust item, and the port's
own line numbers are in the file's `// C <line>` markers.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-31 | blank, `<config.h>`, `<dirent.h>`, `<string.h>`, `<stdio.h>`, `<stdlib.h>`, then C 26-28's `#ifdef __aarch64__` / `<arm_neon.h>` / `#endif` and C 30-31's `<android/api-level.h>` / `<android/log.h>` | file gate; the two NDK headers are the in-place section (P-88 ①) and `<arm_neon.h>` is a comment (the `core::arch::aarch64` route) |
| 32-38 | blank, `androidterm.h`, `sfntfont.h`, `pdumper.h`, `blockinput.h`, `android.h` | file gate |
| 39-57 | `struct sfntfont_android_scanline_buffer` (C 41-48), the two comments, `system_font_directories` (C 51-57, with C 56's `(char[PATH_MAX]) { }` compound literal) | file gate |
| 58-67 | blank, `font_cache` (C 60), `scanline_buffer` (C 63), `max_scanline_buffer_size` (C 67) | file gate |
| 68-73 | the blank/`^L` separators and the comment above `GET_SCANLINE_BUFFER` | - |
| 74-138 | `#ifndef __aarch64__` (C 74) + the `alloca`/`scanline_buffer` definition (C 76-105) + C 107's `#else` + the `posix_memalign` definition (C 109-136) + C 138's `#endif` | `not (feature = "aarch64")` / `feature = "aarch64"` |
| 139-146 | blank/`^L`, the comment above `sfntfont_android_scale32` | - |
| 147-160 | `sfntfont_android_scale32` | file gate |
| 161-171 | blank and `sfntfont_android_mul8x2` | file gate |
| 172-173 | blank and `U255TO256` (`#define`) | file gate |
| 174-193 | the comment above `sfntfont_android_blend` and the function | file gate |
| 194-203 | blank, C 195's `#ifdef __aarch64__`, the comment, `sfntfont_android_u255to256` (C 199-203) | `feature = "aarch64"` |
| 204-268 | the comment and `sfntfont_android_over_8888_1` (C 208-268; C 250-260's `#if 0`/`#else` pair inside) | `feature = "aarch64"`; C 250-260 dead text as a comment |
| 269-307 | the comment and `sfntfont_android_over_8888` (C 278-307) | `feature = "aarch64"` |
| 308-309 | blank and C 309's `#endif` | - |
| 310-382 | the comment and `sfntfont_android_composite_bitmap` (C 315-382; C 326-328's `lim_x` and C 361-375's vector path) | file gate; `feature = "aarch64"` on the two `#ifdef` regions |
| 383-395 | the comment and `sfntfont_android_union_boxes` (C 387-395) | file gate |
| 396-610 | the comment and `sfntfont_android_put_glyphs` (C 402-610; C 490-494's `stride` pair, C 566-576's bitmap lock and format assertion, C 578-599's clip-rect loop, C 601-607's unlock/damage pair, C 609's `#undef`) | file gate; `not (feature = "aarch64")` / `feature = "aarch64"` on C 490-494 |
| 611-618 | the blank/`^L` separators and the comment above `sfntfont_android_shrink_scanline_buffer` | - |
| 619-636 | `sfntfont_android_shrink_scanline_buffer` | file gate |
| 637-643 | the blank/`^L` separators, `/* Font driver definition.  */`, the comment above the cache accessor | - |
| 644-648 | `sfntfont_android_get_cache` | file gate |
| 649-674 | the comment and `android_sfntfont_driver` (C 651-674; C 666-673's `#ifdef HAVE_HARFBUZZ` members) | file gate; `feature = "harfbuzz"` on C 669-672's members |
| 675-680 | the blank/`^L` separators and the comment above the subr | - |
| 681-738 | `DEFUN ("android-enumerate-fonts", Fandroid_enumerate_fonts, …)` (C 681-738) | file gate |
| 739-741 | the blank/`^L` separators after the subr | - |
| 742-748 | `syms_of_sfntfont_android_for_pdumper` | file gate |
| 749-806 | blank and `init_sfntfont_android` (C 751-806; C 763-800's API-level ladder, C 802-805's `snprintf`) | file gate |
| 807-821 | blank and `syms_of_sfntfont_android` (C 809-821) | file gate |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/sfnta-lane/cargo cargo run -q -p xtask -- build-tu
src/sfntfont-android.c` → `built src/sfntfont-android.rs -> ../build/femacs/sfntfont-android.o`,
**exit 0**, and the object is **336 bytes** — the empty object P-86 ⑧'s criterion names (every item
of the port sits behind the file gate, so rustc compiles only `include/config.rs` and the
`#![allow]`).  Re-measured on the final revision by running the *same* rustc invocation
(`--edition 2024 --crate-type lib --emit obj --crate-name sfntfont_android -C panic=abort
-C codegen-units=1 -C opt-level=0`) on the materialized `build/probes/sfntfont-android/_root.rs`
without any `--cfg`: **exit 0, 336 bytes** (and the same for `build/femacs/units/…/_root.rs`).  The
**three `build-tu` runs are the whole budget**: run 1 read 43 errors (the same-line `#[cfg] include!`
spelling, Deviations 14), run 2 read the `#if 0` preprocessor line as Rust (Deviations 10), run 3 is
the 336-byte reading.

**② The arm flip (P-86 ③⑴; the file gate plus every sub-gate).**
`cargo run -q -p xtask -- probe src/sfntfont-android.rs /tmp/sfnta-lane/arm-final.o --crate-type lib
--emit obj -C panic=abort --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'
--cfg 'feature="have_window_system"' --cfg 'feature="aarch64"'` → **exit 0, 0 errors, 19 warnings,
a 12,624-byte object**.  This is the first P-86 arm reading *this* file has ever had, and it is a
type-check/codegen reading of the unit's own text end to end (no font enumerated, no raster
composited — see `### Not claimed`).  The warning census, by file:

- **6 in `_root.rs`, i.e. this unit's text**: `sfntfont_list`, `sfntfont_match`, `sfntfont_draw`,
  `sfntfont_open`, `sfntfont_text_extents`, `sfntfont_list_family` "redeclared with a different
  signature" — the `clashing_extern_declarations` class this tree's unit builds already print
  (`docs/ports/sfntfont.md` Evidence ④ names the same lint), created deliberately by the module
  copies of Deviations 5.
- **12 in mirrors**: `include/android.rs` 7 (4 × `android_vdir` and 2 × `_jmethodID` "not FFI-safe",
  1 × `AndroidBitmapInfo` "not FFI-safe" from the mirror's own opaque tag in its own `extern` block),
  `lib/include/unistd.in.rs` 2 (the `read`/`write` "suspicious definition" class),
  `lib/include/stdbit.in.rs` 1 and `include/lisp.rs` 1 (unused doc comments),
  `include/frame.rs` 1 (a doubled `#[inline(always)]`, with the "phased out" note).

The per-file classification P-86 ⑦ asks for is therefore **6 unit-side / 12 mirror-side, and no
name-resolution abort**.  Four `probe` invocations were spent (**one over the three the task
allowed**): run 1 read **47 errors** (41 `cannot find macro …` for `eassert`/`eassume`/`XUNTAG`/
`XSETPVECTYPESIZE` in *mirrors* — the `lisp.h` ordering of Deviations 12 — plus 1 `__ANDROID_API__`
in a mirror, and 5 in this unit: the two `PATH_MAX` array lengths, `font_cache`'s zero word, C's
chained assignment and the missing `has_char`), run 2 read **17** (the six `transmute`s of
Deviations 5 in their first spelling — "can't transmute zero-sized type"/"requires unsafe" — and the
two partially-initialised `gui_box` locals), run 3 read **exit 0** on the pre-trim revision, run 4 is
the reading above, on the final revision.  Iteration between the runs was done by re-running rustc
directly on the materialized `_root.rs` (the `docs/ports/sfntfont.md` Evidence ④ method) and with
the clang probes of Evidence ③.

**③ The C-side measurements (the machine's NDK 29.0.14206865, `--target=aarch64-linux-android24`).**
`probe_dirent.c` (the configuration's clang, `-I build/emacs-ref/lib -idirafter spikes/shim …`):
`sizeof(struct dirent)=1048 d_ino@0 d_seekoff@8 d_reclen@16 d_namlen@18 d_type@20 d_name@21`, which is
the record Deviations 4 declares and the two `core::assert!`s pin.  `ndk_abi.c` (the NDK's
`aarch64-linux-android24-clang`): `_Static_assert`s that `ANDROID_LOG_VERBOSE == 2` (UNKNOWN 0,
DEFAULT 1, VERBOSE 2, DEBUG 3, INFO 4, WARN 5, ERROR 6, FATAL 7 — `log.h` 74-90),
`ANDROID_BITMAP_FORMAT_RGBA_8888 == 1` (NONE 0 … RGBA_1010102 10 — `bitmap.h` 59-72),
`sizeof(AndroidBitmapInfo) == 20` with `width`@0, `height`@4, `stride`@8, `format`@12, `flags`@16
(`bitmap.h` 99-118, the `flags` member since API 30), `sizeof(jobject) == 8`, and that
`__android_log_print`, `AndroidBitmap_unlockPixels`, `posix_memalign`, `strstr` and `strcmp` are
assignable to the exact prototypes the port declares; `-S` shows the same numbers as immediates
(`mov w1, #2`, `mov w2, #1`, `mov x3, #20`, `mov x4, #8`).  `nm -u` on the frozen-configuration
probe prints `_strstr`/`_strcmp` (no `rpl_` redirect), which is what lets Deviations 5 declare
`strstr` plainly.

**④ The other gates (repository root, final revision), one line each.**
`python3 tools/check-cond-eval.py src/sfntfont-android.c` → `8 conditions, 0 unknown, 0
disagreements [VERIFIED]`. — `python3 tools/extract-abi.py --check emacs/src/sfntfont-android.c
femacs/src/sfntfont-android.rs` → **one finding**: `line count drifted 22.4%: C 821 vs Rust 1005`
(Owed 1).  All fourteen functions, both `GET_SCANLINE_BUFFER` definitions, `U255TO256`, the subr
`Fandroid_enumerate_fonts` with its `defsubr`, and the three file-scope objects
(`font_cache`, `scanline_buffer`, `max_scanline_buffer_size`) are present, and every function
body's token multiset is satisfied — the two tokens that cannot be spelled (`val`, `restrict`)
carry `docs/ports/macro-body-notes.tsv` rows this lane appended. — `python3
tools/check-declarations.py` → `17 declaration sites (0 forbidden, 17 allowed literals), 0
failures`. — `python3 tools/check-mirror-dupes.py` → `17 duplicate declaration(s), 17 ratchet
row(s), 0 failure(s)` (the six hooks of Deviations 5 sit in a module of this unit, which the tool
reads as a different scope, so the ratchet is untouched). — `python3 tools/check-citations.py` →
`676 file(s) scanned, 0 drift, 0 range, 1779 external, 36 allowed, 0 stale` (the integrator flagged
one drift in this file mid-round: the `font.h 682-698` citation next to a `harfbuzz` mention was
stale, and is now `font.h 676-698`/`font.h 807-820`). — `python3 tools/check-dead-file-coverage.py`
→ `1 range row(s)…`, reading below. — `python3 tools/check-ledger-ticks.py` → `0 failure(s)`.

**⑤ The line-count drift and what it is made of (Owed 1).**
Rust 1005 vs C 821 = **+22.4%**, and the classes are measurable: 72 lines are the per-item
`#[cfg (…)]` attributes P-86 ①/⑧ requires (C has no equivalent inside the file), ~130 lines are the
in-place declarations of Deviations 3-5 and 12 (`<android/bitmap.h>`'s record and its 2 constants,
`<android/log.h>`'s level and `__android_log_print`, `__ANDROID_API__`, `struct dirent` and its two
assertions, the nine use-site prototypes and the six-hook module — the NDK and mirror surface C
gets from headers that are not in the reference tree), and the rest is Rust's structural expansion
(casts at the P-72 ③ conversion sites, the zero-initialised locals of Deviations 8, the record's
explicit `None` members, the two `for`-loop spellings of C's `while`+increment).  The port's own
code lines are *not* inflated: C's 567 code lines + 97 comment/`#if` lines are carried as 633 code
and 122 comment lines.

## Deviations

1. **Both arms of every `#if` are carried** (P-86 ①), each on its own `#[cfg]`: the file gate on
   every item and on the whole include graph (C 20-37), the six `__aarch64__` pairs, the two
   `GET_SCANLINE_BUFFER` definitions, and the HarfBuzz member set — see the Gate mapping table.
2. **`<arm_neon.h>` has no mirror**: the intrinsics are `core::arch::aarch64`'s, whose names are the
   ACLE ones C uses, so every call keeps C's name.  The two immediate-operand forms are Rust's
   const-generic spelling (`vshr_n_u8::<7>`, `vshrn_n_u16::<8>` — C passes the immediate as a
   second argument), and `uint8x8x4_t`'s four lanes are the tuple fields `.0`..`.3` where C writes
   `.val[i]` (the `macro-body-notes.tsv` row for `sfntfont_android_over_8888_1`/`val`).  The gate is
   `feature = "aarch64"`, not `target_arch`: C's own header choice ties the two.
3. **The NDK surface is declared in place** (P-88 ①/P-90 ①, no new file under
   `femacs/src/include/`): `ANDROID_LOG_VERBOSE` and `__android_log_print` (C 31),
   `ANDROID_BITMAP_FORMAT_RGBA_8888` and `AndroidBitmap_unlockPixels` (C 37's `android.h` chain),
   and `__ANDROID_API__` (24, the toolchain's own `-D`, the `androidmenu.rs` 68-73 shape — the
   mirror's `android_get_current_api_level` macro names it and leaves it to the consumer).  All are
   measured (Evidence ③).
4. **`AndroidBitmapInfo` is declared in a module of this unit** (`android_bitmap`), because
   `include/android.rs` 44-46 declares the tag **opaque** (android.h names it only through a
   pointer) while C 430-576 reads `stride`, `height`, `width` and `format`: the module leaves the
   mirror its declaration and gives this unit C's name and members, and the one call that takes the
   mirror's tag (`android_lock_bitmap`, C 567) converts the pointer explicitly (P-72 rule 3; the
   mirror round that carries the record is Owed 4).
5. **The six `sfntfont_*` driver hooks are declared a second time in a module of this unit**
   (`font_driver_hooks`), with the *record's* pointer types: `font.h`'s members take
   `*mut c_void`/`*mut glyph_string`/`*mut font_metrics` where `include/sfntfont.rs`'s prototypes
   spell `*mut frame`/`*mut c_void`, and Rust has no `as` cast between function pointers of
   different signatures (a crate-root redeclaration would instead be a new
   `docs/baseline/mirror-dupes.tsv` row).  The cost is the six `clashing_extern_declarations`
   warnings of Evidence ②; `transmute` (the `sysdep.rs` 2873 shape) was tried first and rejected:
   in a static initialiser the *function item* is zero-sized (`E0591`/`E0080`) and the call needs an
   `unsafe` block.  The mirror round that aligns those stand-ins is Owed 4.
6. **`struct dirent`'s record and its two layout assertions** (C 21's header reaches it through the
   frozen `build/emacs-ref/lib/dirent.h`; `lib/include/dirent.in.rs`/`dirent.rs` carry the calls and
   the opaque `DIR` but not the record) — the `asset-directory-tool.rs` 44-60 shape, measured in
   Evidence ③.  C 720-726 reads `d_name`.
7. **The `alloca` arm is the stack pool** (P-05): C 85's `(buffer) = alloca (_size)` becomes
   `sa_stack_alloc (_size)`, the `msdos.rs` 1747-1749 shape whose `sa_avail` accounting C does not
   do here either.  The `posix_memalign` arm (C 107-136) is C's own call.
8. **Locals C leaves to `memset` or to callees are zero-initialised** (PORTING.md 1.5): the three
   `android_rectangle`s, both `gui_box`es, `bitmap_info`, `bitmap`, `stride`, and the subr's
   `name[PATH_MAX * 2]`; `bitmap_data`/`buffer`/`row`/`raster_row`/`left`/`top`/`temp_y`/`prod`/
   `raster_y`/`foreground`/`back_pixel`/`rb`/`bitmap_inf`o are assigned before every read, as C's
   are.
9. **`font_cache`'s tentative definition** (`static Lisp_Object font_cache;`, C 60) is written
   `= lisp_h_Qnil ()` — C's all-zero word *is* `Qnil` in this encoding (the `androidfont.rs` 324
   shape), and Rust requires the initialiser.
10. **Two C constructs have no Rust counterpart**: C 250-260's `#if 0`/`#else` pair (dead in every
    configuration — C's text is carried as a comment, the `w32inevt.md` C 120 reading) and C 609's
    `#undef MAX_ALLOCA` (that name is lisp.h 5616's *enum* constant here too, so the `#undef` is a
    no-op; the port records it in a comment at the same position).  The `#if 0` line is *also* why
    the frozen `build-tu` failed once (Evidence ①: raw preprocessor text in a Rust file).
11. **`restrict`** (C 411's `unsigned char *restrict raster_row`) has no Rust spelling and cannot be
    named in the port's *code* (the body comparison strips comments), so it carries a
    `macro-body-notes.tsv` row and the declaration is a trailing comment.
12. **The unit adds one include edge**: `include/lisp.rs` at C 33's position.  C's `androidterm.h`
    25 → `character.h` 26 reaches `lisp.h` before `font.h`'s inline bodies (`FONT_OBJECT` and
    friends use `XUNTAG`/`eassert`) are read, but `include/character.rs` carries no such edge yet —
    without it 41 mirror-side `cannot find macro` errors are the arm reading (Evidence ② run 1).
    The mirror round that adds the edge is Owed 4.
13. **The P-72 ③ conversions are explicit at their use sites**: `glyph_string.gc` is the mirror's
    `*mut c_void` (so `*((*s).gc as *mut android_gc)`), `buffer` is `*mut c_void` where C's
    `unsigned int *` receives a `void *` (with C's own casts carried at C 348, 522 and 551 — note
    that C 348 scales in `unsigned int` units and C 522/551 in bytes, and the port keeps each), and
    the `android_rectangle *` → `Emacs_Rectangle *` conversions of C 485 and C 582 are spelled
    (the two records are the same four members).
14. **`#[cfg] … include!` must be two lines, not one.**  The first frozen `build-tu` read 43 errors
    because `xtask`'s `gate_states` records a line as gated only when an attribute *run reached* it
    (`femacs/xtask/src/main.rs`), so `#[cfg (…)] include!("…");` on one line is read as **ungated**
    and the whole mirror graph was compiled in the frozen configuration.  Every include edge here is
    therefore attribute-then-edge, the tree's own spelling.  Worth a `PORTING.md` P-86 note; the
    *items* can merge their attribute onto their own line (the `vm-limit.rs` 114 shape, used for the
    record's `harfbuzz` members).
15. **The line count is +22.4% and needs a tolerance row** (Owed 1); Evidence ⑤ breaks the excess
    into its classes.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **`docs/ports/line-drift-tolerance.tsv`: a row for `sfntfont-android`** (outside this lane's
   write scope).  Measured: C 821 vs the port's **1005** = **+22.4%**, basis `total`; the reasons are
   Evidence ⑤'s three classes (72 gate attributes, ~130 in-place NDK/mirror declaration lines, the
   rest Rust's structural expansion).  Until the row exists, `extract-abi.py --check` prints exactly
   this one finding and nothing else.
2. **`docs/port-map.md` 526**: the port-side count is still the placeholder's 11; the row is the
   integrator's regeneration (`docs/port-map.md` is outside this lane's write scope).
3. **`tools/check-port-lines.py --write-pending`**: this file's ratchet row is stale for the same
   reason (1005 Rust lines against a 821-line threshold, so the file leaves P-86's
   under-ten-percent class with this batch).
4. **Two mirror rounds** (both outside this lane's write scope; each removes a Deviation):
   (a) `femacs/src/include/character.rs` (or another mirror on the `androidterm.h` chain) needs its
   `include!("lisp.rs")` edge for C's character.h 26 → Deviations 12's extra edge goes;
   (b) `femacs/src/include/sfntfont.rs`'s six driver-hook prototypes should spell the *record's*
   pointer stand-ins (`*mut c_void`/`*mut glyph_string`/`*mut font_metrics`, P-72 ③), and
   `femacs/src/include/android.rs` should carry `<android/bitmap.h>`'s record (or the P-90 ①
   SDK-mirror decision) → Deviations 4 and 5's module copies and their six
   `clashing_extern_declarations` warnings go.  The same round could give
   `android_get_current_api_level`'s `__ANDROID_API__` a home (Deviations 3) and clear
   `include/android.rs`'s seven "not FFI-safe" warnings.
5. **A `PORTING.md` P-86 note for Deviations 14** (`xtask gate_states` reads a same-line
   `#[cfg] include!` as ungated) — a tool/rule change, not a port change; the symptom is 43
   mirror-side errors under the *frozen* configuration, which is a bad failure mode to meet twice.

## Not claimed

**There is no runtime evidence of any kind.**  The frozen object (Evidence ①) is 336 bytes of
nothing, and the arm reading (Evidence ②) is a type-check/codegen pass: `rustc` produced a
12,624-byte object for the gate set, but nothing linked it against `liblog`/`libjnigraphics`, no APK
was built, no Android device or emulator ran it, no font directory was scanned, no raster was
composited into a bitmap, and no JNI call was made.  `sfntfont_android_shrink_scanline_buffer`
(the only entry point the tree's ported code already calls — `include/androidterm.rs` 637 declares
it) has a definition here, but its caller `androidterm.c` is a slice and does not call it yet.  The
NDK measurements of Evidence ③ are the *headers'* ABI and constants of NDK 29.0.14206865 for
`aarch64-linux-android24`, not a device's runtime values; the API level the code branches on
(`android_get_current_api_level`) is a run-time value this round never observed.  C 250-260 is dead
in every configuration (Deviations 10).  The line-count row of Owed 1 is not a fidelity claim: the
per-function token comparison (Evidence ④) is.
