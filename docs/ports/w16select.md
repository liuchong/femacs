# emacs/src/w16select.c (692 lines) — whole file, C 1-692

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w16select.o` joins the
> MS-DOS build's own object list rather than this target's: `emacs/src/Makefile.in` 306 leaves
> `MSDOS_OBJ` empty for every configure, and `emacs/msdos/sed1v2.inp` 136 rewrites that line to
> `= dosfns.o msdos.o w16select.o` when the MS-DOS make files edit the generated Makefile;
> `docs/baseline/frozen-build-sources.txt` lists no `w16select.o`.  So `tools/extract-abi.py`'s
> checklist for the file is empty (P-86 ②) and this ledger is that clause's substitute: the **C
> line-range table**, the `## Evidence` readings and the `## Deviations`.  Port:
> `femacs/src/w16select.rs` (**988 lines**, 50,738 bytes, md5
> `3f97833556e72cc11c88a7fa427bde9e`), carrying **C 1-692 with the whole body under its single
> `#ifdef`**.  The reference `emacs/src/w16select.c` is md5 `e543fa63ae75d0078e82b9ca43cad533`
> (unchanged; `emacs/` is read-only), and its `docs/port-map.md` 562 row pairs `src/w16select.c`
> 692 with `femacs/src/w16select.rs` (the port's own count in that table is the integrator's
> regeneration — Owed 4).
>
> **This ledger replaces the reading it inherits.**  The previous `docs/ports/w16select.md` (and
> the 19-line comment unit it described, which read the file as "@ 694 lines" and concluded "the
> frozen configuration does not compile this file ⇒ the port is the empty unit plus the note that
> names the gate") is withdrawn: P-86 ① replaces that conclusion with *transcribe the text, both
> arms, no stub*, exactly as `docs/ports/dosfns.md`, `w32select.md`, `w32inevt.md`,
> `w32console.md`, `w32menu.md` and `w32term.md` record for their own pairs.  What the earlier note
> measured (not in the frozen source list, empty checklist, a 336-byte `build-tu` object) is
> unchanged and still true; it is the *conclusion*, and the reference's own line count (692, not
> 694), that are withdrawn.
>
> **The file gate is `msdos`, on every item *and* on the include graph** (P-86 ⑧: an ungated
> `include!` would break the frozen compile at once, which is why Evidence ① is an empty object).
> C 31's `#include <config.h>` is the one edge outside it, as C has it.  C 29 and C 692 are the
> file's *only* two preprocessor conditionals — `w16select.c` has no `WINDOWSNT`, `HAVE_X_WINDOWS`
> or `SYSTEM_MALLOC` sub-gate anywhere, unlike its `src/dosfns.c` sibling — so the `## Gate
> mapping` table has one row for the file itself and one for the ungated `config.h` edge.
> Acquisition: **this piece is `text-only` (P-86 ③⑵)** — there is no DJGPP toolchain on this
> machine (`which -a i586-pc-msdosdjgpp-gcc i686-pc-msdosdjgpp-gcc djgpp-gcc` is empty; no
> `djgpp`/`go32` include directory under `/opt/homebrew`, `/usr/local` or `/Library`), and the
> reference tree carries none of `<dpmi.h>`, `<go32.h>`, `<sys/farptr.h>`.  So there is **no arm
> flip on a DJGPP target** and **no measurement of any record, member, value or prototype of
> theirs**: every one is `UNVERIFIED` and named in `## Owed`.
>
> **What the arm flip does show is this unit's own text.**  Unlike the sibling MS-DOS-family
> ledgers, *no* error of the `msdos` arm is anchored in a mirror copy: `src/w16select.c` includes
> neither `msdos.h` nor `dosfns.h`, so its include graph does not reach `include/msdos.rs` (the
> `EINPROGRESS` row) or `include/dosfns.rs` (the eight top-level declarations) — the two known
> debts are the *other* MS-DOS units', and this unit's arm is green (Evidence ②).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`MSDOS_OBJ`, `emacs/src/Makefile.in` 306 + `emacs/msdos/sed1v2.inp` 136; the macro is `MSDOS`, `configure.ac` 6953's `AH_TEMPLATE`) | `feature = "msdos"` (registered, `femacs/xtask/Cargo.toml` 400), on every item *and* on C 32-42's include graph | the DJGPP (MS-DOS) build; `MSDOS` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 2849, so no configuration this machine can build sets it |
| C 31 `#include <config.h>` | **no gate** (C has it outside `#ifdef MSDOS`; `include/config.rs`) | every build — the file's only ungated edge (P-86 ⑧) |

`tools/check-cond-eval.py src/w16select.c` reads **`1 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence ③): C 29 is the whole conditional text of the file, and `MSDOS` alone
decides it.  The probe door set is therefore that one live value
(`--cfg 'feature="msdos"'`), and no second `--cfg` is needed.

## C line ranges

`docs/port-map.md` pairs **one** reference with this ledger (562 `src/w16select.c`; the stem has no
header row, unlike `src/dosfns.c`/`src/dosfns.h` and `src/w32select.c`/`src/w32select.h`), so the
table below is **contiguous**: each row spans its block *and* the blank (or form-feed) separator
that follows it, as `docs/ports/dosfns.md`'s, `msdos.md`'s and `w32inevt.md`'s rows do for the same
reason.  The "C" column is `src/w16select.c`'s numbering.

| C | block | arm |
| --- | --- | --- |
| 1-20 | the file's opening ownership/title block | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 22-25 | the WinOldAp note (what WINOLDAP.MOD is) | carried as C's own text |
| 27 | "Adapted to DJGPP by Eli Zaretskii" | carried as C's own text |
| 29 | `#ifdef MSDOS` | the file gate; every item carries the feature (the line itself has no counterpart) |
| 31 | `#include <config.h>` | **ungated** (`include/config.rs`) |
| 32-34 | `#include <dpmi.h>`, `<go32.h>`, `<sys/farptr.h>` | file gate; three in-unit stand-in sections (P-88 ①, **UNVERIFIED**) |
| 35-42 | `lisp.h`, `dispextern.h` (C 36's "frame.h seems to want this"), `frame.h` (C 37's "Need this to get the X window of selected_frame"), `blockinput.h`, `character.h`, `buffer.h`, `coding.h`, `composite.h` | file gate; C 36/37's trailing notes carried as C's text |
| 44-46 | the "these prototypes belong on a header file" note | carried as C's own text |
| 48-58 | the eleven `CF_*` clipboard-format constants | file gate; `pub const`, only `CF_OEMTEXT` is read |
| 60-67 | C's eight forward declarations | dropped (PORTING.md 0.1-12; the C lines are recorded as `C:` text) |
| 69-74 | `clipboard_xfer_buf_info` (C's anonymous `static struct`) | file gate; the type is named after the variable (P-67) |
| 76-82 | the `last_clipboard_text` note and object | carried as C's text; file gate for the object |
| 84-85 | `clipboard_storage_size` | file gate |
| 87-98 | the Microsoft Knowledge Base / Ralf Brown's Interrupt List notes | carried as C's own text |
| 100-113 | `identify_winoldap_version` | file gate |
| 115-136 | `open_clipboard` | file gate |
| 138-150 | `empty_clipboard` | file gate |
| 152-185 | `alloc_xfer_buf` (C's `static`) | file gate |
| 187-206 | `free_xfer_buf` (C's `static`) | file gate |
| 208-305 | `set_clipboard_data`, including C 225-233's and C 255-269's two arms | file gate; both arms of the internal `if (Raw)` (P-51) |
| 307-323 | `get_clipboard_data_size` | file gate |
| 325-404 | `get_clipboard_data`, including C 355-399's guarded body | file gate |
| 406-418 | `close_clipboard` | file gate |
| 420-435 | `clipboard_compact` | file gate |
| 437-442 | `no_mem_msg`, `binary_msg`, `system_error_msg` | file gate; `&CStr` statics (Deviations 4) |
| 444-545 | `DEFUN w16-set-clipboard-data` (`Fw16_set_clipboard_data`), including C 512's `error:` and C 516's `unblock:` labels | file gate; labelled blocks (Deviations 6) |
| 547-625 | `DEFUN w16-get-clipboard-data` (`Fw16_get_clipboard_data`), including C 616's `closeclip:` and C 619's `unblock:` labels | file gate; labelled blocks (Deviations 6) |
| 627-672 | `DEFUN w16-selection-exists-p` (`Fw16_selection_exists_p`) | file gate |
| 674-690 | `syms_of_win16select` | file gate |
| 692 | `#endif /* MSDOS */` | no counterpart (every item carries the feature) |

The table is contiguous and ascending (30 rows, 1-692): the 112 blank/form-feed lines of
`src/w16select.c` sit inside the rows above (each row spans its block *and* the separator that
follows it), and `tools/check-dead-file-coverage.py`'s own reading is **30 range row(s) cover
`src/w16select.c` (692 lines; 27 blank line(s) between blocks need no row), 0 failure(s)**.

## Subrs (DEFUN) — 3

- [x] w16-set-clipboard-data (C 444, min 1, max 2, intspec `0`)
- [x] w16-get-clipboard-data (C 547, min 0, max 1, intspec `0`)
- [x] w16-selection-exists-p (C 629, min 0, max 2, intspec `0`)

`defsubr` order in `syms_of_win16select` (C 677-679) is C's.

## Functions defined — 14

- [x] identify_winoldap_version (C 101)
- [x] open_clipboard (C 116)
- [x] empty_clipboard (C 139)
- [x] alloc_xfer_buf (C 154)
- [x] free_xfer_buf (C 191)
- [x] set_clipboard_data (C 209)
- [x] get_clipboard_data_size (C 308)
- [x] get_clipboard_data (C 328)
- [x] close_clipboard (C 407)
- [x] clipboard_compact (C 421)
- [x] Fw16_set_clipboard_data (C 444)
- [x] Fw16_get_clipboard_data (C 547)
- [x] Fw16_selection_exists_p (C 629)
- [x] syms_of_win16select (C 674)

The first ten are C's non-`static` definitions (six of them `static`-less helpers C also exposes
through its forward-declaration block) and carry `#[unsafe(no_mangle)]`; `alloc_xfer_buf` and
`free_xfer_buf` are C's two `static` helpers and stay plain `unsafe fn`.  The three `F…` names are
the `DEFUN!`-generated bodies.

## Macros defined — 11

- [x] `CF_TEXT 0x01` (C 48) — `pub const CF_TEXT: c_int`
- [x] `CF_BITMAP 0x02` (C 49)
- [x] `CF_METAFILE 0x03` (C 50)
- [x] `CF_SYLK 0x04` (C 51)
- [x] `CF_DIF 0x05` (C 52)
- [x] `CF_TIFF 0x06` (C 53)
- [x] `CF_OEMTEXT 0x07` (C 54) — the only one read (C 217, 235, 564, 571, 594, 665)
- [x] `CF_DIBBITMAP 0x08` (C 55)
- [x] `CF_WINWRITE 0x80` (C 56)
- [x] `CF_DSPTEXT 0x81` (C 57)
- [x] `CF_DSPBITMAP 0x82` (C 58)

These eleven are the file's whole preprocessor macro list; nothing else in it is a `#define`.

## Types and their fields — 0

The file defines no type of its own.  The four records it *uses* are declared where they enter this
translation unit and are **UNVERIFIED** (Owed 1/2): `<dpmi.h>`'s `__dpmi_regs` (C 32) and
`<go32.h>`'s `_go32_info_block` (C 33) stand in the unit's two header sections, and C 69-74's
anonymous `static struct { unsigned long size; unsigned short rm_segment; }` is named
`clipboard_xfer_buf_info` after the variable (P-67).  `<sys/farptr.h>`'s accessors (C 34) are
prototypes, not records.

## File-scope objects — 6

- [x] `clipboard_xfer_buf_info` (C 71) — `static mut`
- [x] `last_clipboard_text` (C 82) — `static mut *mut c_uchar`
- [x] `clipboard_storage_size` (C 85) — `static mut size_t`
- [x] `no_mem_msg` (C 437) — `static &core::ffi::CStr`
- [x] `binary_msg` (C 439) — `static &core::ffi::CStr`
- [x] `system_error_msg` (C 441) — `static &core::ffi::CStr`

## Evidence

**① The frozen configuration (`build-tu`), before and after.**
`cd femacs && CARGO_TARGET_DIR=/tmp/w16sel-lane/target cargo run -q -p xtask -- build-tu
src/w16select.c` -> `built src/w16select.rs -> ../build/femacs/w16select.o`, **exit 0**, and
`build/femacs/w16select.o` is **336 bytes, md5 `dbed68837694579185377dbb0821ab62`** — the *same*
reading the 19-line note unit took before the port was written, and the same md5 the
`src/dosfns.c`, `src/w32inevt.c` and `src/w32select.c` lanes read for their own empty objects.
P-86 ⑧ predicts exactly this: every item and every `include!` but C 31's sits behind the file gate,
and an ungated `include!` would break this compile at once.

**② The arm flip, as a per-file error census (P-86 ③⑴/⑦).**  Three `probe` runs against the same
`--cfg 'feature="msdos"'` door, in order:

| run | command | exit | errors | warnings | errors anchored in this unit's own lines (`_root.rs`) | errors in mirror copies |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1 | `probe src/w16select.rs /tmp/w16sel-lane/arm1.rlib --crate-type=lib -C panic=abort --cfg 'feature="msdos"'` | 1 | 3 | 10 | **3** (`E0308`) | 0 |
| 2 | same, `arm2.rlib`, after the two fixes | 0 | **0** | 10 | 0 | 0 |
| 3 | same, `arm3.rlib`, after the two fidelity touch-ups (Deviations 9/10's spellings) | 0 | **0** | 10 | 0 | 0 |

The three run-1 errors are all `E0308` and all in the unit's own text: `want_size <=
_go32_info_block.size_of_transfer_buffer` (`u32` vs `u64`, fixed by writing C's widening as
`as c_ulong`) and the one `ok = empty_clipboard () && (…)` expression (`bool` into a `c_uint`
binding, fixed by the `as c_uint` the C `&&` means).  Each edit removed exactly the error its line
carried and nothing else, and **no error of any run is anchored in a mirror copy** — this unit's
include graph reaches neither `include/msdos.rs` nor `include/dosfns.rs`, so the two known
MS-DOS-family mirror debts (Owed 3) do not appear at this door.  The 10 warnings are
`unused doc comment` on the extern blocks (7 in this unit's text, the same pattern
`src/dosfns.rs`'s 23 warnings carry) and three pre-existing `#[inline(always)]`/`(part of
warn(unused))` rows inside `include/frame.rs` and `lib/include/stdbit.in.rs`.

**③ The `#if` verdict.**  `python3 tools/check-cond-eval.py src/w16select.c` -> **`1 conditions, 0
unknown, 0 disagreements [VERIFIED]`**, and `src/w16select.c` is on the evaluator's verified list
(`docs/baseline/config/cond-eval-verified.txt`), so the table above is a measurement and not a
claim.  The single condition is `MSDOS`, `undef` in
`docs/baseline/config/darwin-arm64.macros.tsv` 2849.

**④ The text gates.**
`python3 tools/extract-abi.py --check emacs/src/w16select.c femacs/src/w16select.rs` -> **`OK …`**,
zero problem lines *and* the P-86 ② vacuous pass: `MSDOS` is undef, so `tools/cinventory.py`'s
live-line filter gives the C side an **empty checklist** and the range table above is that clause's
substitute (P-86 ②, ⑩).  `python3 tools/check-declarations.py` -> **`17 declaration sites (0
forbidden, 17 allowed literals), 0 failures`**.  `python3 tools/check-citations.py` -> **`676
file(s) scanned, 1 drift, 0 range, 1812 external, 36 allowed, 0 stale`**, and the one `drift` row is
**not this unit's**: `femacs/src/w16select.rs` contributes **0 findings of any class** (measured
with the gate's own `findings_for`), because its line citations are the `// C <n>:` form and its
header citations the `C: <header>:` form, neither of which the gate's `path.h <n>` pattern matches.
The drift is `femacs/src/w32image.rs:76` (`lisp.h 30`, "`memset` is at lisp.h:2008") — another
lane's in-flight file, present in the shared working tree, and the external count rose from 1779 to
1812 with the other lanes' files over the same window, not with this one.
`python3 tools/check-mirror-dupes.py` -> **`17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)`** — this unit added no duplicate (the in-unit stand-in declarations shadow no mirror
in its graph).  `python3 tools/check-dead-file-coverage.py` binds the table above and
`python3 tools/check-ledger-ticks.py` binds the function ticks; `python3 tools/check-port-lines.py`
reads **`0 new, 2 stale`** — the stale `src/w16select.c` row is this port's (Owed 4).

**⑤ What is **not** a reading.**  No DJGPP compile, no DOS, no WinOldAp, no `Int 2Fh`/`Int 21h`
call, no clipboard, no low-memory buffer, no far-memory store, no DOS-memory transfer, no coding
system.  The three `probe` readings are `rustc` on this machine with one `--cfg`; the frozen
reading is an *empty* object, so it measures the gates and not the code.  Nothing of this file was
executed anywhere.

## Deviations

1. **The opening block is dropped, not carried.**  C 1-20 (title, author and the ownership prose)
   is removed whole per PORTING.md 0.1-19; the port's first line is the
   `// femacs: port of src/w16select.c @ fd7afced403.` declaration.  C 5's author line dies with
   it, and C 27's DJGPP note (which sits *below* the block in C) is carried separately.
2. **C 60-67's eight forward declarations are dropped** (PORTING.md 0.1-12): Rust has no forward
   declarations, a declaration plus a definition is `E0428`, and items are order-independent, so
   each definition stands at its C position.  The block is recorded as `C:` text in the unit.
3. **Zero-initialization where C declares without an initializer (PORTING.md 0.1-13).**  The nine
   `__dpmi_regs regs;` blocks (C 104, 119, 142, 157, 197, 212, 331, 410, 424) and the two
   `struct coding_system coding;` (C 483, 591) take `std::mem::zeroed ()` first, because Rust has
   no partial initialization and `&raw mut` of an uninitialised binding is not allowed.
4. **C's three `static char[]` message arrays are `&core::ffi::CStr` statics.**  The port's spelling
   of a C string literal is `c"..."` (the `src/w32select.rs` 1439 `stdfmt_name` reading), so
   `sizeof (X) - 1` at C 530/533/536 becomes `X.to_bytes ().len ()`.  C's arrays are non-`const`
   but never written; the statics are therefore not `mut` (a mutability the C text never uses).
5. **C 69-74's anonymous struct is named after the variable (P-67).**  C's
   `static struct { … } clipboard_xfer_buf_info;` has no tag, so the port declares the type
   `clipboard_xfer_buf_info` and the object of it.
6. **C's `goto`s are labelled blocks (PORTING.md 2.2).**  `w16-set-clipboard-data`'s
   `done`/`error`/`unblock` are the outer `'flow`, the shared `ok = 0` and the inner `'unblock`
   (the `error:` arm falls into it exactly as C's does); `w16-get-clipboard-data`'s
   `done`/`unblock`/`closeclip` are `'flow`/`'unblock`/`'closeclip`, with the body falling out of
   `'closeclip` into `close_clipboard ()` as C's does (the `src/w32select.rs` 1369 shape).
7. **Two `while (Size--)` loops keep the post-decrement (C 255, C 368).**  Each becomes
   `loop { let old: c_uint = Size; Size -= 1; if old == 0 { break; } … }`, which reproduces C's
   test-before-decrement exactly (the `src/xdisp.rs` 15211 spelling); C's own value after the loop
   (a wrapped `UINT_MAX`) differs from the port's `0`, and C never reads it.
8. **The two coding-bitfield writes are masked writes on the mirror's single `bits: u32`.**
   `coding->mode |= CODING_MODE_LAST_BLOCK` (C 599) and
   `coding->common_flags &= ~CODING_ANNOTATION_MASK` (C 602) address fields `include/coding.rs`
   stores in one unit (`coding.rs` 239-252 spells the shifts), so the port writes
   `coding.bits |= (CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT` and
   `coding.bits &= !(CODING_ANNOTATION_MASK << CODING_SYSTEM_COMMON_FLAGS_SHIFT)`.  The
   common-flags write is deliberately *not* the `src/w32select.rs` 1302 two-part form
   (`bits & COMMON_FLAGS_MASK & !…`), which would also clear the mode bit C 599 sets immediately
   before it — C's `&= ~mask` touches only the common-flags field's bits.
9. **C's int-vs-unsigned comparison is spelled out.**  C 580's `i < truelen` is an `int` against an
   `unsigned`, so C promotes `i`; the port writes `(i as c_uint) < truelen` (the same reading the
   `src/msdos.rs` 1237 notes for its `int`-vs-`size_t` rows).
10. **C's implicit widening at C 160 is written explicitly.**  `want_size <=
    _go32_info_block.size_of_transfer_buffer` becomes `want_size as c_ulong <= …`, because the
    member's type is UNVERIFIED (Owed 2) and `c_ulong` is the header's documented one; C's own
    conversion is the same.
11. **The `CF_*` comparisons are `Format != CF_OEMTEXT as c_uint`.**  C mixes `unsigned` and `int`
    here; the port spells the one conversion.
12. **The three `regs.x.…` stores of 16-bit fields cast explicitly** (`as c_ushort`) and every
    `regs.x.ax` read that feeds a `c_uint` return is `as c_uint`; C does all of it implicitly.
13. **C's `switch (put_status)` is a `match`** with the three `case`s and `_ => {}` for C's missing
    `default` (the `src/w32select.rs` 1038 reading of the same construct).
14. **`ok = empty_clipboard () && ((put_status = …) == 0);`** (C 502-505) is written as the same
    single expression with the boolean result converted by `as c_uint`, so the short-circuit is
    preserved: when `empty_clipboard ()` is 0, `put_status` keeps its C value 0 and the `if
    (put_status)` block is skipped, exactly as in C.
15. **`regs.x.ax > 0 ? 0 : 3` (C 304), `if (last_clipboard_text)` (C 280, C 396) and
    `(dp = memchr (…)) != 0` (C 228) are the `if`/`else`, `.is_null ()` and `loop`/`break` forms.**
    `unsigned char *htext = NULL` is `core::ptr::null_mut ()`.
16. **`coding.id as hash_idx_t`** at C 498/605 matches the tree's convention for
    `CODING_ID_NAME` (`src/callproc.rs` 1121, `src/coding.rs` 10783).
17. **C 640's unused `terminal` parameter has no discard statement.**  C never names it after the
    signature and has no `(void) terminal;`, so the port adds none either (the file-level
    `unused_variables` allow covers it).

## Owed (nothing here is claimed as done)

1. **No DJGPP toolchain, so every stand-in item is UNVERIFIED**: `__dpmi_regs` (both views, the
   member order and the record's size), `_go32_info_block` (the three members' order and types),
   `__tb`, `_dos_ds`, `__dpmi_int`, `_farpokeb`, `_farnspokeb`, `_farnspeekb`, `_farsetsel`,
   `dosmemput`, `dosmemget`, and the five Lisp-side prototypes this unit declares for names no
   mirror carries (`decode_live_frame`, `setup_coding_system`, `message3`, `sit_for`).  The layout
   assertions the other SDK stand-ins carry are owed with them.  *A future DJGPP arm flip is the
   only thing that can close this.*
2. **`_go32_info_block.size_of_transfer_buffer`'s type.**  The port chose `c_ulong` (the header's
   documented `unsigned long`) and wrote C's widening explicitly (Deviations 10); the header is not
   on this machine, so this is a choice and not a reading.  The `regs.x.flags & 1` test (C 177)
   carries the same class of debt: the carry flag's position is the runtime's, not a reading.
3. **The two known MS-DOS-family mirror debts are *not* closed by this lane, and this unit's arm
   cannot expose them.**  `include/msdos.rs` 234's `EINPROGRESS` against `lib/include/errno.rs` 92
   (`docs/ports/dosfns.md` Owed 2) and `include/dosfns.rs` 40/44/45/46/47/56/60/63's eight top-level
   declarations (`docs/ports/dosfns.md` Owed 1) belong to the units that `include!` those mirrors;
   `src/w16select.c` includes neither `msdos.h` nor `dosfns.h`, so Evidence ②'s green arm says
   nothing about them.  They are recorded here only so the integrator does not read this lane's
   `0 errors` as a fix.
4. **`docs/port-map.md` 562's port count.**  The row still reads `femacs/src/w16select.rs | 19`;
   this port is 988 lines.  The regeneration is the integrator's (`--write-pending`), and
   `docs/port-map.md` is not this lane's writable set.  The companion row is
   `docs/baseline/port-lines-pending.tsv`'s `src/w16select.c 692 19 69` line, which
   `tools/check-port-lines.py` now reports **stale** (informational; the same non-writable file).
5. **`dosmemput`/`dosmemget`'s return types, and the four `<sys/farptr.h>` prototypes.**  This unit
   declares both transfers `void` (C reads neither result); `src/msdos.rs` 603 spells `dosmemput`
   `-> c_int` and `src/dosfns.rs` 288 spells the same two `void`.  Only the DJGPP headers settle
   it, and the same is true of `_farpokeb`/`_farnspokeb`/`_farnspeekb`/`_farsetsel`'s widths.
6. **The in-unit stand-in sections duplicate no mirror, and that is the whole reason this arm is
   green** — if a future batch adds a `<dpmi.h>`/`<go32.h>`/`<sys/farptr.h>` mirror and this unit
   `include!`s it, every name in Owed 1 becomes an `E0428` candidate.  Whoever lands such a mirror
   owns reconciling these 11 declarations with it.
7. **The `C:` header citations in the unit carry no line numbers** (the `C: frame.h: …` form), so
   the sibling units' `frame.h 1476`, `coding.h 686`, `lisp.h 4373`/`4392` line numbers were not
   re-verified here; the reader can take them from `src/fontset.rs` 105, `src/w32select.rs` 660 and
   `src/keyboard.rs` 453/`src/editfns.rs` 179, where they already stand.

## Not claimed

Every line of this unit sits behind a gate the frozen configuration leaves false: Evidence ① is an
empty object, and ②-⑤ are compile-level or textual.  **Nothing here was executed** — no
`__dpmi_int`, no `Int 2Fh`/`Int 21h` clipboard or allocation call, no `_farpokeb`/`_farnspokeb`/
`_farnspeekb` store or load, no `dosmemput`/`dosmemget`, no clipboard open/empty/set/get/close/
compact, no coding-system encode or decode, no kill-ring lookup, no message.  There is **no
DJGPP-target compile and no layout measurement**: the arm flip is `rustc` on this machine with one
`--cfg`, and its final reading is **0 errors, all warnings** (Evidence ②).  The file is
**transcribed (C 1-692)**, and "transcribed" is not "compiled", "measured" or "working": the seven
owed items above and the absence of a DJGPP toolchain are what stand between this text and a
measured `msdos` arm.  The arm type-checks today; no behaviour of it has been observed by anyone.
