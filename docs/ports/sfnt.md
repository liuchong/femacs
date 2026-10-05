# emacs/src/sfnt.c (21,215 lines) — first slice: C 1-1112

> **P-86's largest unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 2645 adds `sfnt.o` to `ANDROID_OBJ` only inside the `XCONFIGURE = android`
> branch (the `REALLY_ANDROID` one, 2620-2654) — `android.o` is added one branch earlier, at
> 2598, which is why `src/android.c`'s file gate is `have_android` alone and this one is the
> pair — and `docs/baseline/frozen-build-sources.txt` lists no `sfnt.o`.  What follows is P-86
> ②'s **C line-range table** for this slice, the `## Evidence` readings and the
> `## Deviations`.  Port: `femacs/src/sfnt.rs` (1,298 lines, `grep -c ''`), both arms of every
> `#if` carried.
>
> **This file's ledger replaces the previous revision's conclusion** ("the file is not in the
> frozen build, so the port is the empty unit plus a note"), exactly as `docs/ports/android.md`
> and `docs/ports/androidselect.md` did: P-86 ① rules that a three-line header comment is *not
> started*, not *done*.
>
> **One measurement differs from `src/android.c`'s, and it changes what the tool can say.**
> `src/sfnt.c` **is** in `docs/baseline/config/cond-eval-verified.txt` (353, together with
> `src/sfnt.h`/`src/sfntfont.c`/`src/sfntfont.h`), so `tools/extract-abi.py` evaluates its `#if`s
> and builds a real inventory — 421 declarations.  The tool prints its "not compiled by the
> frozen configuration … nothing in this file is required here" note and *clears* that inventory
> only when the port has no `fn` at all (`tools/extract-abi.py` 2225-2227); the 11-line
> placeholder had none, which is why `--check` read `OK` on it.  This slice does, so `--check` is
> **not vacuous here**: its reading (Evidence ③) is a live coverage statement about the 21,215
> C lines and counts out what the remaining slices owe.  P-86 ②'s table below is therefore not a
> substitute for a checklist that the tool cannot produce (the `android.c`/`image.c` situation) —
> it is this ledger's account of what the slice *does* carry, next to the tool's account of what
> the file still owes.
>
> **The slice is C 1-1112** (through the `}` of `sfnt_read_cmap_table` at C 1112): the include
> graph and the TEST harness (C 20-139), `xaddmalloc`/`xaddrealloc`/`MIN`/`MAX` (C 141-160),
> the file's own note (C 162-191), `sfnt_table_names` (C 195-219), the two swap helpers and
> their macros (C 221-238), `sfnt_read_table_directory` (C 248-323), `sfnt_find_table`
> (C 328-341), the six cmap subtable readers `sfnt_read_cmap_format_0`/`_2`/`_4`/`_6`/`_8`/`_12`
> (C 350-841), `sfnt_read_24` (C 845-849), `sfnt_read_cmap_format_14` (C 858-932),
> `sfnt_read_cmap_table_1` (C 939-1011) and `sfnt_read_cmap_table` (C 1020-1112).  **The next
> live definition is C 1117's `sfnt_lookup_glyph_0`**, and the table's last row is
> "the remainder".

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2645, the `XCONFIGURE = android` branch; `ANDROID_STUBIFY` is defined in the other one, 2605-2606) | `all (feature = "have_android", feature = "not_android_stubify")` (on every item and on C 22-39's include graph) | a *real* Android configuration |
| C 41 `#ifdef HAVE_MMAP` … C 43 (`#include <sys/mman.h>`) | **no gate**: `HAVE_MMAP` is 1 on this target (`docs/baseline/config/darwin-arm64.macros.tsv` 1469), so the include is live in every configuration and the port records it as text (no mirror; see Owed 5) | always, on this target |
| C 45 `#if defined __GNUC__ && !defined __clang__` … C 47 (`#pragma GCC diagnostic`) | **dropped**: a diagnostic pragma has no Rust counterpart (the `-Wstringop-overflow` class) | never |
| C 49 `#ifdef TEST` … C 134 (the harness: `memory_full_up`, the six allocators, `TEST_STATIC`, `eassert`, `AVOID`) | `all (file gate, feature = "test")` — `femacs/xtask/Cargo.toml` 564 registers C's `TEST` as `test` | a test build |
| C 136 `#else` … C 139 (`#define TEST_STATIC` + `#include "lisp.h"`) | `all (file gate, not (feature = "test"))` | the frozen-shaped configuration |
| C 128 / C 137 (`#define TEST_STATIC static` / `#define TEST_STATIC`) | `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]` on the two readers C exports (`sfnt_read_table_directory`, `sfnt_read_cmap_table`) — a construct PORTING.md does not yet cover, see Deviations 1 and Owed 1 | linkage only; the item is on the file gate either way |
| C 224 / C 232 `#ifndef WORDS_BIGENDIAN` (the two swap helpers' bodies) | `not (feature = "words_bigendian")` (`femacs/xtask/Cargo.toml` 614); the other arm is C's empty body — the big-endian byte order needs no swap, so it is not a branch the port invents | little-endian |

`not_android_stubify` is the feature the tree registered for C's `HAVE_ANDROID && !defined
ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` 640, first used by `alloc.rs` 6428);
`androidselect.rs`'s gate is the same pair, and `docs/ports/androidselect.md` derives it from
`configure.ac` 2647-2648 the same way this ledger derives it from 2645.  **No file-scope C `#if`
in this slice needs a feature this tree does not already have** — a first for this class: every
gate here is one of the five names above, all long registered.

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-20 | the blank and `#include <config.h>` | ungated (every unit includes it) |
| 21-22 | the blank and `#include "sfnt.h"` | file gate |
| 23-39 | the file's own include list (C 24 `<attribute.h>`, C 25 `<byteswap.h>`, C 26 `<fcntl.h>`, C 27 `<intprops.h>`, C 28 `<inttypes.h>`, C 29 `<stdbit.h>`, C 30 `<stdckdint.h>`, C 31 `<stdcountof.h>`, C 32 `<stdint.h>`, C 33 `<stdio.h>`, C 34 `<stdlib.h>`, C 35 `<string.h>`, C 36 `<unistd.h>`, C 37 `<setjmp.h>`, C 38 `<errno.h>`, C 39 `<alloca.h>`) | file gate |
| 40-47 | C 41-43's `#ifdef HAVE_MMAP`/`<sys/mman.h>`/`#endif` and C 45-47's `__GNUC__`-not-`__clang__` `#pragma` block | live unconditionally / dropped |
| 48-139 | the `#ifdef TEST` harness (C 51-56's own includes, C 58-64 `memory_full_up`, C 66-77 `xmalloc`, C 79-90 `xcalloc`, C 92-96 `xzalloc`, C 98-105 `xnmalloc`, C 107-118 `xrealloc`, C 120-124 `xfree`, C 126-134 `TEST_STATIC`/`eassert`/`AVOID`, C 136-139 the `#else` arm) | `test` / `not test` (both carried) |
| 140-157 | `xaddmalloc` (C 141-148), `xaddrealloc` (C 150-157) | file gate |
| 158-160 | the blank and `MIN`/`MAX` (C 159-160, whose bodies are minmax.h's) | file gate |
| 161-191 | the file's own note: what the scaler covers, the TrueType Reference Manual, and why Microsoft's extended scaler differs | file gate |
| 192-219 | `sfnt_table_names` (C 197-219): all 18 slots designated, `SFNT_TABLE_CMAP` … `SFNT_TABLE_POST` | file gate |
| 220-234 | `sfnt_swap16_1` (C 221-226) and `sfnt_swap32_1` (C 229-234) | file gate / `not words_bigendian` |
| 235-238 | `sfnt_swap16` and `sfnt_swap32` (C 237-238) | file gate |
| 239-323 | `sfnt_read_table_directory` (C 248-323; C 253-255 `SFNT_ENDOF`'s use, C 262-271 the TTC probe that returns `(struct sfnt_offset_subtable *) -1`, C 284-288 the second `-1` return, C 292-320 the table-directory realloc and the 4-swap loop) | file gate / `TEST_STATIC` |
| 324-341 | `sfnt_find_table` (C 328-341) | file gate |
| 342-345 | the form feed and the "Character mapping routines." banner | file gate |
| 346-379 | `sfnt_read_cmap_format_0` (C 350-379) | file gate |
| 380-483 | `sfnt_read_cmap_format_2` (C 384-483; C 390-398 the `SFNT_ENDOF` minimum, C 411-437 the 256-entry `sub_header_keys` scan that computes `nsub`, C 442-470 the variable-length read and the `subheaders`/`glyph_index_array` split) | file gate |
| 484-589 | `sfnt_read_cmap_format_4` (C 488-589; C 498-499 the `SFNT_ENDOF` minimum, C 533-548 the `variable_size` arithmetic and the short-table bail, C 558-565 the five data pointers, C 571 the one `reserved_pad` swap, C 573-585 the two swap loops) | file gate |
| 590-659 | `sfnt_read_cmap_format_6` (C 594-659; the `uint16_t i` loop at C 654 is C's own counter type) | file gate |
| 660-750 | `sfnt_read_cmap_format_8` (C 664-750; C 674 reads the 32-bit length before the header, C 711/717 the `ckd_mul`/`ckd_add` pair) | file gate |
| 751-841 | `sfnt_read_cmap_format_12` (C 755-841; C 779 `xaddmalloc`, C 802/808 the `ckd_mul`/`ckd_add` pair) | file gate |
| 842-849 | `sfnt_read_24` (C 845-849; `bytes[0] << 16u` and its `8u` twin) | file gate |
| 850-932 | `sfnt_read_cmap_format_14` (C 858-932; C 887-888 the `ckd_mul (…) || ckd_add (…)`, C 918-923 the two `memcpy`s of the unaligned offsets) | file gate |
| 933-1011 | `sfnt_read_cmap_table_1` (C 939-1011; C 946 the `ckd_add` of the two offsets, C 952-968 the format-14 special case that reads no `length`, C 973-1010 the seven-way `switch`) | file gate |
| 1012-1112 | `sfnt_read_cmap_table` (C 1020-1112; C 1060-1061 the `subtables` array, C 1089 the `*data` array whose `sizeof *data` is a *pointer's* size, C 1096 the `(void *) -1` IO-error test, C 1101-1106 the partial-free loop) | file gate / `TEST_STATIC` |
| 1113-21215 | **the remainder** — the blank at C 1113 and from C 1114's comment through C 21215: `sfnt_lookup_glyph_0` (**the next live definition**, C 1117), the other five cmap lookups (C 1130-1452), `sfnt_read_head_table` … `sfnt_read_meta_table` (C 1477-1790), the `glyf`/`loca`/`hmtx` readers and the mmap pair (C 1793-1906), the simple/compound glyph readers (C 1913-2529), the compound-glyph decomposition (C 2551-3595), the outline builders and the rasterizer (C 3596-6426), and the whole TrueType interpreter, `sfnt_interpret_*` (C 6427-21215, including the `#ifdef TEST` programs at C 6437-11000) | not this slice |

The thirty rows above tile C 1-1112 **contiguously** (blank lines included) rather than skipping
the blanks between blocks: `tools/check-dead-file-coverage.py`'s ledger lookup resolves
`femacs/src/include/sfnt.rs` (this file's header's mirror) to this same `sfnt.md` through its
bare-stem fallback, so a table with gaps at blanks would leave `src/sfnt.h`'s non-blank lines
uncovered and fail (the `android.md`/`w32term.md` shape).  The last row reaches C 21215, which is
what the gate requires of a slice ledger.

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/sfnt1-lane/cargo cargo run -q -p xtask -- build-tu src/sfnt.c`
→ `built src/sfnt.rs -> ../build/femacs/sfnt.o`, **exit 0**, and `build/femacs/sfnt.o` is
**336 bytes** — an empty object, measured on the final revision.  That is the expected reading and
**not** a proof of the new text: every item sits behind the file gate, so under the frozen feature
set rustc compiles only `include/config.rs` and the `#![allow]` attribute.  Two further `build-tu`
runs are part of the same evidence: the first (same command) was the *only* compile that saw the
port's syntax errors, and it reported them (`unclosed delimiter` ×2, `expected {, found header`
×1); the third is the reading quoted here.

**② The arm flip (P-86 ③⑴ — this machine has the NDK, so `text-only` is not allowed).**
`cargo run -q -p xtask -- probe src/sfnt.rs /tmp/sfnt1-lane/sfnt_arm.o --crate-type lib --emit obj
--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`
→ **exit 1, 8 errors, and 0 of them in the unit's own text**:
1. **2 × `E0428`**: `include/sfnt.rs` 2200 and 2206 declare `sfnt_read_table_directory` and
   `sfnt_read_cmap_table` at the *top level* of the mirror, and the unit defines them — the
   declaration block needs the `pub mod sfnt_decls { … } use sfnt_decls::*;` shape
   (`include/android.rs`'s, and P-86's own androidmenu.rs precedent), which is another lane's file
   and is owed below.
2. **1 × `expected item after doc comment`**: `lib/include/alloca.in.rs` 49 — the file ends with
   a `///` comment (`/// C: void *alloca (size_t);`) and no item after it.  This unit is the
   tree's **first** user of that mirror (measured: `rg -n 'alloca\.in\.rs' femacs` finds only
   `femacs/src/sfnt.rs:86`), so the defect was latent.
3. **5 × `E0425`** for `time_t`/`timespec`, all reached through `lisp.rs`'s own include graph
   rather than this file's: `lib/include/sys_time.rs` 44 and 81 (`time_t`), `lib/include/sys_select.rs`
   55 and `src/include/thread.rs` 300/318 (`timespec`).  The Android arm's `lisp.rs` chain never
   reaches `lib/include/time.in.rs`.
The earlier shortened probe (`--cfg` pair but no `--crate-type lib`) read **exit 1, 71 errors**,
including `E0601: main function not found` — the harness's own parameter, corrected in the
command above.

**②a The control — ②'s "0 in our text" is not vacuous here, measured.**  On the two revisions
before the fixes the *same* harness reported **this unit's** errors and named them at
`_root.rs` lines: 36 × `E0608` (raw-pointer indexing — Rust has no `Index` for `*mut T`), 21 ×
`E0600` (`-1 as *mut T`: the literal is inferred as `usize`, so the port must write
`usize::MAX as *mut T`, pdumper.rs 186's spelling), 4 × `E0308` (`scaler_type` `u32` against the
mirror's `sfnt_ttc_tag` `i32`; `uint32_t.overflowing_mul (size_t)`), 3 × `E0381` (two `length`
locals and `header` that C leaves uninitialised; the port zeroes them, PORTING.md 1.5) and
1 × `E0530` (a `let` cannot shadow the `static mut sfnt_table_names` the initialiser defines —
see Deviations 3).  rustc therefore *does* type-check this unit's bodies in this harness — unlike
`docs/ports/androidmenu.md`'s ②, where name-resolution errors in the mirrors aborted before
type-checking and the reading had to come from a `/tmp` stub harness.  The remaining 8 errors and
the 2 warnings (`lib/include/stdbit.in.rs` 644, `src/include/lisp.rs` 3934, both "unused doc
comment") are all in included mirrors.
**The arm reading is from the 1,293-line revision; the final revision is 1,298 lines and the only
differences are the header comment and three `// C …` trailing ranges** (comment-only; the
probe budget is spent, below).

**③ Inventory (the live checklist).**
`python3 tools/extract-abi.py --check emacs/src/sfnt.c femacs/src/sfnt.rs` → **FAIL**, with
**364 `- missing …` lines — 205 functions, 153 macros, 5 constants (`SFNT_BLOCK_STEPS`,
`SFNT_POLY_ALIGNMENT`, the three `SFNT_RUN_CONTEXT_*`), 1 file-scope object
(`build_outline_context`)** — and 4 named rows:
`- function sfnt_read_cmap_table_1 does not mention case, default, switch` (C's seven-way `switch`
is the port's `match`, PORTING.md 2.2 — the `goto`-class token row, owed to
`docs/ports/macro-body-notes.tsv`), the two `- function … is not static in C but has no
#[unsafe(no_mangle)]` rows for the `cfg_attr`-gated pair (Owed 1), and
`- line count drifted 93.9%: C 21215 vs Rust 1298` (the whole file against one slice).
Every one of this slice's own names matches — the 18 functions above, the two `sfnt_swap*_1`
helpers, `sfnt_table_names` and the two `MIN`/`MAX` macros — the only name of the slice's own text
in the missing list is **`TEST_STATIC`** (C 128/137; Owed 2).

**④ Other gates (repo root, final revision).**
`python3 tools/check-dead-file-coverage.py` → **20 ledger(s) bound (a C range table), 0
failure(s)**, and this ledger is one of them: `info docs/ports/sfnt.md: 26 range row(s) cover
src/sfnt.c (21215 lines; 0 blank line(s) between blocks need no row)` plus the same 26 rows over
`src/sfnt.h` (2151 lines, the bare-stem fallback above).  Before this ledger carried the section
the tool did not name `src/sfnt.c` at all — a file with no `## C line ranges` is simply not bound,
and the placeholder's three-line note could not bind it. —
`python3 tools/check-declarations.py` → **14 declaration sites (0 forbidden, 14 allowed
literals), 1 failures**, and the one failure is **`femacs/src/android.rs`'s `PACKAGE_STRING`
literal** — another lane's in-flight file (`git status` lists `femacs/src/android.rs` as
modified), not this one; `femacs/src/sfnt.rs` and this ledger contribute 0.  (The first reading
of this gate in this lane named 16 sites and 3 failures: the two extra were *this ledger's own
prose* — a `decl` row for the C 1-18 block and a `title` row for quoting another unit's product
literal — which is the gate doing its job on a ledger that had just copied C's opening block and
has since been rewritten.)

## Deviations

1. **`TEST_STATIC` is a construct PORTING.md does not cover yet.**  C's `#define TEST_STATIC
   static` (C 128, the TEST arm) / `#define TEST_STATIC` (C 137) decides the linkage of the two
   readers this slice exports.  The port's expression is
   `#[cfg_attr (not (feature = "test"), unsafe (no_mangle))]`, which is the same bit and needs no
   second copy of either body.  This is the tree's **only** `cfg_attr` (measured:
   `rg -c cfg_attr femacs/src/*.rs` → `femacs/src/sfnt.rs:4`) and the tool's textual test for the
   attribute (`tools/extract-abi.py` 1984-1989) does not see through it, which is what the two
   `- function … has no #[unsafe(no_mangle)]` rows in Evidence ③ are.  Owed 1 asks PORTING.md to
   rule on the spelling.
2. **C's designated initialiser for `sfnt_table_names`.**  C 197-219 is `= { [SFNT_TABLE_CMAP] = …,
   … }` with all 18 slots designated; Rust has no designated initialiser, and the port's first
   spelling (a `let mut` local inside the `static`'s initialiser, keyboard.rs 5607's shape) is a
   measured **`E0530: let bindings cannot shadow statics`**.  The port therefore writes the same
   18 values positionally in C's order, each with `/* [SFNT_TABLE_*] = … */` — which is the same
   array (no slot is implicit) and keeps C's order and values on the page.
3. **Locals C leaves uninitialised and the port zeroes** (PORTING.md 1.5, `android.md`'s
   Deviations): `length` in `sfnt_read_cmap_format_8`/`_12` (C 671/762, read into by C 674/765 —
   measured `E0381` when left uninitialised), `header` in `sfnt_read_cmap_table_1` (C 944, whose
   *fields* C fills at C 952/964 and whose uninitialised whole is what `&raw mut header.format`
   trips on), `buffer1`/`buffer` in `sfnt_read_cmap_format_14` (C 866/868).
4. **`(T *) -1` is `usize::MAX as *mut T`** (21 sites: the TTC returns of
   `sfnt_read_table_directory`, the six format readers' IO-error returns,
   `sfnt_read_cmap_table_1`'s five, and C 1096's `(void *) -1` test).  Measured `E0600`: `-1 as
   *mut T` infers the literal as `usize` and cannot be negated.  This is pdumper.rs 186's
   `MAP_FAILED` spelling and belongs in `docs/ports/token-aliases.tsv` (Owed 3).
5. **C's `(uint16_t *) (what)` cast in `sfnt_swap16`/`sfnt_swap32` is two casts in Rust.**  The
   argument is already a `uint16_t *` at most sites but is `&format4->id_delta[i]` (an `int16_t
   *`) at C 577/578, and Rust cannot cast a reference straight to a differently-typed raw pointer;
   the macro keeps C's single cast as `as *mut _ as *mut X` (measured: both call shapes compile).
6. **`SFNT_ENDOF (struct X, field, uint16_t[256])`** (C 392/438) is
   `SFNT_ENDOF! (X, field, [uint16_t; 256])` — Rust's array type syntax; the mirror's macro
   takes the type as `$type1:ty`, so only the spelling of the array type moves.
7. **`ckd_mul`/`ckd_add` are written out at their four sites** (C 711/717, C 802/808, C 887-888
   and C 946) as `overflowing_*` plus a write-back — P-15's spelling (`buffer.rs` 3602, the
   `androidselect.rs` 619-620 shape).  C 887-888's `ckd_mul (…) || ckd_add (…)` is two `if`s
   because the first writes `size` as well as returning the flag; the early return is the same
   control flow and the unobserved difference (C leaves `size` alone when the multiply overflows,
   the port returns before reading it) is recorded here.
8. **C's `switch` (C 973) is the port's `match`** with `_` for `default` (PORTING.md 2.2); the
   seven arms and their C line numbers are on the arms.  This is the row Evidence ③ reports as
   "does not mention case, default, switch".
9. **C's implicit conversions are written as casts at their sites**, which is what Rust
   requires: `header->length` and each `uint16_t`/`uint32_t` field against a `size_t`
   (`(*header).length as size_t`) or an `off_t` (`(*directory).offset as off_t`), the
   `uint16_t i` loop counters against `int` bounds (`as c_int`), `format14`'s
   `num_records` (a `uint32_t`) into the `uint16_t` field (`as uint16_t`, C's own truncation),
   and `sfnt_table_names[table]` indexed by the `enum sfnt_table` (`as usize`).
10. **Line count**: C 1,112 (this slice) vs the port's 1,298 (+16.7%) — the per-item gates, the
   C-line comments, `std::mem::size_of::<T> ()` for C's `sizeof *p`, the `.offset (i as isize)`
   deref for C's `p[i]`, and Rust's brace-only lines.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. ✅ **Done in 3.427** — **`femacs/src/include/sfnt.rs`: the declaration run now sits in
   `pub mod sfnt_decls { use super::*; … } use sfnt_decls::*;`** (the `include/android.rs` shape, P-86's own androidmenu.rs precedent).  Until
   then the arm flip can never get past `E0428` for the two readers this slice defines, and this
   file's own text is never fully compiled (Evidence ②, item 1).  This mirror is another lane's
   in-flight file (`docs/ports/include-sfnt.md` is its ledger).
2. **`TEST_STATIC`'s Rust form** — either a `docs/ports/excluded-macros.txt` row (`kind=syntax`,
   "the port expresses the linkage with `#[cfg_attr (not (feature = "test"), unsafe
   (no_mangle))]`", the `AVOID` row's shape) or a PORTING.md §11 entry; today `--check` counts it
   as a missing macro (Evidence ③).
3. **Two `docs/ports/token-aliases.tsv` rows**: `(T *) -1` → `usize::MAX as *mut T`, and
   `p[i]` on a raw pointer → `*(p.offset (i as isize))` (Deviation 4/Deviations 9's note).  Both
   tokens are now used by a real unit for the first time.
4. **A `docs/ports/macro-body-notes.tsv` row**: `sfnt_read_cmap_table_1` →
   `case,default,switch` (C 973-1010's `switch` is the port's `match`), the row Evidence ③ names.
5. **Done in 3.427 + 3.429** (`alloca.in.rs`'s 26 trailing `///` citations are plain `//` notes now, and the
   five `time_t`/`timespec` errors were **not a mirror gap at all**: the materializer let the unit's
   `#ifdef TEST` `<time.h>` edge claim the one expansion of `time.in.rs`, so the always-live reach through
   `buffer.rs` became a no-op and the names vanished - P-89's root, fixed in `xtask` at 3.429; the arm now
   reads **exit 0 with a 4,784-byte object**) - the remaining names below:
   `time_t`/`timespec` for
   `lib/include/sys_time.rs`/`sys_select.rs`/`src/include/thread.rs` through `lisp.rs`'s chain.
   `<sys/mman.h>`, `<setjmp.h>` and the TEST arm's `<sys/wait.h>`/`<X11/*>` have no mirror at all;
   this slice's own text uses no name from any of them.
6. **`tools/check-port-lines.py --write-pending`**: this file's ratchet row is stale (no longer
   under ten percent) — deliberately left to a batch close, so that the rewrite does not encode
   the parallel lanes' in-flight units.

### Not claimed

Every line of this slice is behind gates the frozen configuration leaves false: **① is an empty
object and the arm reading (②) never produced one**, so there is **no runtime evidence of any
kind** for this port — no font was opened, no table directory was read, no cmap subtable was
parsed, no `read`/`lseek` returned.  C 1113-21215 is **not ported here**; the 364 `- missing …`
lines of Evidence ③ and the 93.9% drift are that boundary.  The `.offset (i as isize)` /
`usize::MAX` / `overflowing_*` rewrites are compile-level and were measured by the compiler, not
by a running program.

**The integrator's 3.427 follow-up, measured.**  After item 1's module and item 5's `alloca.in.rs`
fix: `build-tu src/sfnt.c` exit 0 (336-byte object, unchanged), the Android arm probe
(`--cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'`) reads **5 errors, all
`E0425 time_t`/`timespec`** where the lane's reading was 8 (two `E0428` and the doc-comment error
retired), and `tools/check-all-units.sh` gives `checked 83 unit(s), 0 failure(s)` /
`checking 83 unit(s), 0 unit failure(s)` for the two mirror edits.
