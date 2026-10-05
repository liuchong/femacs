# emacs/src/w32xfns.c (447 lines) — whole file, C 1-447

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32xfns.o` joins
> `W32_OBJ` by `configure.ac` 3051 — the non-Cygwin arm of the split inside the `HAVE_W32` test
> (3019-3030) whose 3024 `AC_DEFINE`s `HAVE_NTGUI` — and `docs/baseline/frozen-build-sources.txt`
> lists no `w32xfns.o` for this target, so `tools/extract-abi.py`'s checklist for the file is empty
> (P-86 ②) and this ledger is that clause's substitute: the **C line-range table**, the
> `## Evidence` readings and the `## Deviations`.  Port: `femacs/src/w32xfns.rs` (**911 lines**,
> 36,785 bytes, md5 `384a7fafb1147b22062bfcce7809c463`), carrying **C 1-447 with both arms of every
> `#if`**.  The reference `emacs/src/w32xfns.c` is 447 lines, md5
> `3598e9e5a49a389007d104fa0636831c` (unchanged; `emacs/` is read-only), and its `docs/port-map.md`
> 588 row pairs `src/w32xfns.c` 447 with `femacs/src/w32xfns.rs` — that row's port count (11) is the
> integrator's regeneration (Owed 6).
>
> **This ledger replaces the reading it inherits.**  The previous `docs/ports/w32xfns.md` (and the
> 11-line comment unit it described) read "the frozen configuration does not compile this file ⇒ the
> port is the empty unit plus the note that names the gate" (the P-08 reading, `docs/status.md`
> 3.31).  P-86 ① replaces that conclusion with *transcribe the text, both arms, no stub*; what the
> earlier note measured (not in the frozen source list, empty checklist, 336-byte `build-tu` object)
> is unchanged and still true — it is the *conclusion* that is withdrawn, exactly as
> `docs/ports/w32inevt.md`, `w32select.md`, `w32image.md` and `dosfns.md` record for their own pairs.
>
> **The file gate is `windowsnt`, and the arm reading is `windowsnt` alone.**  `HAVE_NTGUI` (3024) is
> on for *both* the Cygwin and the native Windows GUI build, while `w32xfns.o` joins `W32_OBJ` only
> in the non-Cygwin arm (3051) — the build whose `nt/inc/ms-w32.h` 23-25 `#define`s `WINDOWSNT`.  The
> units of that same arm already landed with `windowsnt` as their file gate (`src/w32.rs`,
> `src/w32heap.rs`, `src/w32proc.rs`, `src/w32console.rs`), so this one does too.  C 26-38, C 49-51,
> C 81-83 and C 433-438's `#ifdef WINDOWSNT` is the *same* condition as the file gate, so those four
> blocks carry `windowsnt` and nothing more; the other three conditions are C 31's
> `#ifndef MINGW_W64`, C 66/110's `#if HAVE_W32NOTIFY` and C 364's `#ifdef CYGWIN`.
>
> **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
> Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`/`wingdi.h`/`wtsapi32.h`), so there
> is **no `<windows.h>` to compile or measure against**, no arm flip on a Windows target and no
> layout measurement: every record, every value and every prototype of the `<windows.h>` /
> `<windowsx.h>` / `<wtsapi32.h>` stand-in sections is `UNVERIFIED` (the documented SDK text, not a
> measurement) and the layout assertions are owed (Owed 5).  What the round does carry is the frozen
> `build-tu` reading, the machine-checked range table, the eight `#if` verdicts, a per-item C
> citation, the per-file error census of the `windowsnt` arm, the same unit under three further arm
> variants, and a name mutation that proves the reading is not vacuous.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, `configure.ac` 3019-3051; the native-Windows `else` of the Cygwin split) | `feature = "windowsnt"` (registered, `femacs/xtask/Cargo.toml` 826), on every item **and on the include graph** (P-86 ⑧) | the native MS Windows GUI build (`nt/inc/ms-w32.h` 23-25 defines `WINDOWSNT`); not the Cygwin GUI build |
| C 20 `#include <config.h>` | **no gate** (C has it outside every `#if`; `include/config.rs`) | every build — the file's only ungated edge (P-86 ⑧) |
| C 21-24 `#include <signal.h>`, `<stdio.h>`, `<windows.h>`, `<windowsx.h>` | the file gate (C itself has them before C 26's `#ifdef`, but no configuration the frozen build reaches compiles this file — the `src/w32fns.rs` 27-35 reading of the same position) | the Windows build; C 23/24 have no file in the reference tree, so their names stand in the port's own sections (P-88 ①) |
| C 26-38 `#ifdef WINDOWSNT` | the file gate itself | the Windows build; C has no `#else` |
| C 31-36 `#ifndef MINGW_W64` | `not (feature = "mingw_w64")` (registered, `Cargo.toml` 216) | `MINGW_W64` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`, so C's `#ifndef` arm is the live side of its condition |
| C 49-51, C 81-83, C 433-438 `#ifdef WINDOWSNT` | the file gate | the Windows build; none has an `#else` in C |
| C 66-79, C 110-125 `#if HAVE_W32NOTIFY` | `feature = "have_w32notify"` (registered, `Cargo.toml` 355 — its comment names this file's two blocks) | `HAVE_W32NOTIFY` is `undef` in the frozen macro table, so both arms are dead there; C has no `#else` for either, and the text is carried as the arm (P-51) |
| C 364-368 `#ifdef CYGWIN` | `feature = "cygwin"` (registered, `Cargo.toml` 81) | `CYGWIN` is `undef` in the same table ⇒ dead; C has no `#else` |

`tools/check-cond-eval.py src/w32xfns.c` reads **`8 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence ⑤): C 26, 31, 49, 66, 81, 110, 364 and 433 — those eight rows are the whole
conditional text of the file (the `#endifs` at 36/38/51/79/83/125/368/438 are not conditions).  C 26's
condition is the file gate itself, C 31's and C 364's are decided by macros the frozen table records
as `undef`, and C 66/110's by `HAVE_W32NOTIFY` (`undef` too) — so every one of the eight has a
verdict, and **no arm of this file is live in the frozen configuration**, which is why the frozen
object is empty.

## C line ranges

`docs/port-map.md` 588 pairs one reference with this ledger (`src/w32xfns.c`, 447 lines); the rows
below are ascending, non-overlapping and cover every non-blank line of C 1-447 (the lines between two
rows — C 25, 39, 43, 46, 48, 52, … — are the reference's blank lines, which the gate does not require
a row for), ending at the reference's last line.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening ownership/title block | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 19-24 | C 20 `config.h`; C 21 `signal.h`; C 22 `stdio.h`; C 23 `<windows.h>`; C 24 `<windowsx.h>` | C 20 ungated; C 21-24 the file gate, C 23/24 the in-place stand-ins (P-88 ①, UNVERIFIED) |
| 26-38 | `#ifdef WINDOWSNT`: the `_WIN32_WINNT` override (C 27-30), the `#ifndef MINGW_W64` enum (C 31-36), `#include <wtsapi32.h>` (C 37) | file gate; C 31-36 `not (feature = "mingw_w64")`; C 37's header is the in-place stand-in for `WM_WTSSESSION_CHANGE`/`WTS_SESSION_LOCK` |
| 40-42 | `lisp.h`, `frame.h`, `w32term.h` | file gate; `../lib/include/stdlib.rs` is carried under C 40 (lisp.h's `<stdlib.h>`, C 70/120's `malloc`/`free`) |
| 44-45 | `myalloc` / `myfree` | file gate |
| 47 | `CRITICAL_SECTION critsect;` | file gate |
| 49-51 | `#ifdef WINDOWSNT` / `extern HANDLE keyboard_handle;` / `#endif` | file gate; the declaration is kept (`src/w32inevt.rs` 467 shape) |
| 53-55 | `input_available` (C 53), `interrupt_handle`'s prototype + definition (C 54-55) | file gate |
| 57-92 | `init_crit` (C 62-63's comment inside it) | file gate; C 66-79 `feature = "have_w32notify"` |
| 94-126 | `delete_crit` | file gate; C 110-125 `feature = "have_w32notify"` |
| 128-134 | `signal_quit` (C 131-132's comment inside it) | file gate |
| 136-161 | `select_palette` | file gate |
| 163-168 | `deselect_palette` | file gate |
| 170-241 | `get_frame_dc` (C 170-171's comment above it, C 201-202's inside it) | file gate |
| 243-261 | `release_frame_dc` (C 248-249's comment inside it) | file gate |
| 263-267 | `int_msg` | file gate |
| 269-271 | `lpHead`, `lpTail`, `nQueue` | file gate |
| 273-353 | `get_next_msg` (C 280's comment inside it) | file gate |
| 355 | `extern char * w32_strerror (int error_no);` | file gate; kept as the unit's own declaration — the two mirrors that carry it (`include/w32.rs` 149, `include/cygw32.rs`) are off this graph |
| 357-358 | the `notify_msg_ready` banner | carried as C's own comment |
| 359-369 | `notify_msg_ready` | file gate; C 364-368 `feature = "cygwin"` |
| 371-398 | `post_msg` | file gate |
| 400-419 | `prepend_msg` | file gate |
| 421-422 | the `drain_message_queue` banner | carried as C's own comment |
| 423-447 | `drain_message_queue` | file gate; C 433-438 `feature = "windowsnt"` (the file gate's own condition) |

## Evidence

① **Frozen `build-tu`** (`cd femacs && CARGO_TARGET_DIR=/tmp/w32xfns-lane/target cargo run -q -p
xtask -- build-tu src/w32xfns.c`): `built src/w32xfns.rs -> ../build/femacs/w32xfns.o`, **exit 0,
336 bytes** — the P-08 empty object, which is what every item being `#[cfg (feature = "windowsnt")]`
produces.  The frozen compile still *parses* the whole unit.  Budget: `build-tu` spent **2** (≤3; the
second reading is the one the final text gets, after the three token-check fixes of ⑥).

② **The arm reading** (`cargo run -q -p xtask -- probe src/w32xfns.rs /tmp/w32xfns-lane/x.rlib
--crate-type=lib -C panic=abort --cfg 'feature="windowsnt"'`): **exit 1, 17 errors + 21 warnings**,
and the per-file census (P-86 ⑦) is:

| file (the copy rustc names) | errors | what |
| --- | --- | --- |
| `src/include/w32term.rs` | 12 × `E0428` | the mirror's own **top-level** declarations of the 12 names this unit defines: `prepend_msg` (mirror 1007), `critsect` (1032), `init_crit` (1033), `delete_crit` (1034), `signal_quit` (1035), `select_palette` (1059), `deselect_palette` (1060), `get_frame_dc` (1061), `release_frame_dc` (1062), `drain_message_queue` (1063), `get_next_msg` (1064), `post_msg` (1065) — the `src/w32select.rs` Findings-2 class; the fix is the P-21/P-39 module shape that mirror already uses for `w32term_fn_decls`/`w32term_objs` (Owed 1) |
| `src/include/w32term.rs` | 2 × `E0425` | `KEY_EVENT_RECORD` (mirror 344, C's `w32term.h` 261) and `font` (mirror 447, C's `w32term.h` 335 `struct font *font;`) — names the mirror leaves to its includers; this unit's own C text names neither (Owed 2) |
| `_root.rs` (this unit) | **3 × `E0609`** | C 194 and C 230's `output->paint_buffer_dirty = 1` and C 207's `FRAME_OUTPUT_DATA (f)->want_paint_buffer`: `include/w32term.rs` 523 **records** those two bitfields as a comment instead of a storage unit (P-80's member-dispatch accessor is the fix), so the mirror's `w32_output` has no such fields — the registered mirror-gap class of `src/xwidget.rs` 1018-1021 / `docs/ports/xwidget.md` Findings A/1 (Owed 3) |

**No other file contributes an error**: rustc reports 17 real errors, and the "aborting due to 17
previous errors" line closes them.  The reading is on the **pinned `nightly-2026-10-01`**
(`femacs/rust-toolchain.toml`, which `xtask` honours because it runs rustc with `cwd = femacs/`);
a run under the machine's default **stable 1.98.1** adds one `E0658` in `lib/include/lisp.rs` 3761
(`verror`'s `core::ffi::VaList` — the pin's own subject, recorded in Owed 4 so the two readings are
not confused).

The 21 warnings are 13 `unused doc comment` in the `w32term.rs` copy, 5 in this unit (the `/// C:`
line above each `<…> extern "C" {` block), and 1 each in `stdbit.in.rs`, `lisp.rs` and `frame.rs`.
Budget: `probe` spent **2** (≤3; the first reading was taken before the three token-check fixes of
⑥, which do not move this census).

③ **Three further arm variants** (this unit's own text, so they are arm readings and not a Windows
compile), on the same pinned nightly and through the `/tmp` replica of ④:
`windowsnt + have_ntgui` → 12 `E0428` + 3 `E0609` + `include/frame.rs`'s deliberate HAVE_NTGUI
`compile_error!` (its copy 329), and the two `E0425`s of ② are gone;
`windowsnt + have_window_system` → the same 12 + 3 with 5 `E0425`s; and
`windowsnt + have_ntgui + have_window_system (+ have_w32notify)` → the same 12 + 3 + the same
`compile_error!`, i.e. **the two `HAVE_W32NOTIFY` arms (C 66-79 and C 110-125, `init_crit`'s list
construction and `delete_crit`'s free loop) add no error of their own** — they type-check with
`have_ntgui` on.  The 1-cfg reading the task fixes is the official `probe` of ②; these three are the
replica's, which ④ shows is `xtask`'s own materialization.

④ **The replica is the tool.**  A `/tmp` materializer (the `docs/ports/w32select.md` Evidence ⑫
shape: `include!` expanded once per file, the same `gate_states`/dedupe rules) produces a `_root.rs`
that is **line-for-line identical to `xtask`'s own materialization** for both the `build-tu` root and
the `probe` root (911 lines each; the only difference is the copy directory's path).  **Mutation
test**: renaming `critsect` to `critsect_ZZ` at its one use in `init_crit` adds exactly one `E0425`
(18 real errors against 17) — the reading resolves this unit's own text, so "the 3 findings of ② are
all it has" is not a vacuous statement.

⑤ **`python3 tools/check-cond-eval.py src/w32xfns.c`**: `8 conditions, 0 unknown, 0 disagreements
[VERIFIED]` — the eight rows of `## Gate mapping`, nothing else.

⑥ **`python3 tools/extract-abi.py --check emacs/src/w32xfns.c femacs/src/w32xfns.rs`**: one finding
left, `line count drifted 103.8%: C 447 vs Rust 911` (`FAIL`), i.e. the tolerance row of Owed 7.
Three earlier findings were fixed in this round rather than excused: `select_palette`,
`deselect_palette` and `release_frame_dc` "does not mention output_data, w32" — the port had spelled
C's `f->output_data.w32->…` through the mirror's `FRAME_OUTPUT_DATA!` macro, so the C body's own two
tokens were absent from the Rust body; spelling the chain the way C writes it (with the one cast the
union member needs) makes the token check green.  `drain_message_queue`'s `break, case, switch` is
the one excused row, and it is appended to `docs/ports/macro-body-notes.tsv` (see ⑪).

⑦ **`python3 tools/check-declarations.py`**: `17 declaration sites (0 forbidden, 17 allowed
literals), 0 failures`, `read 683 files` — the unit's first line is the `// femacs: port of …` form
and no comment of its own names this repository (the carried C comments say "Emacs"/"MS Windows",
which are the reference's own text and are not gate patterns).

⑧ **`python3 tools/check-mirror-dupes.py`**: `17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)` — unchanged, and no row names this unit: its 12 colliding definitions are *definitions*
(which that gate does not report) and its `unsafe extern "C"` declarations (`w32_strerror`,
`keyboard_handle`, the `<windows.h>` entry points) are at a crate root no mirror shares.

⑨ **`python3 tools/check-citations.py`**: `676 file(s) scanned, 0 drift, 0 range, 1812 external, 36
allowed, 0 stale` — **no finding names this unit** (0 occurrences of `w32xfns` in the report), so its
~90 `/// C:`/`// C:` citation lines are all in range and current.

⑩ **`python3 tools/check-dead-file-coverage.py`**: `33 ledger(s) bound (a C range table), 0
failure(s)` — this ledger's row is the `## C line ranges` table above.

⑪ **`python3 tools/check-ledger-ticks.py`**: 0 failure(s) — no tick of this ledger names a function
another port carries, and this ledger has no ticked box at all (P-86 ②'s point).

⑫ **The unit**: `femacs/src/w32xfns.rs` 911 lines, 36,785 bytes, md5
`384a7fafb1147b22062bfcce7809c463`; the reference `emacs/src/w32xfns.c` 447 lines, md5
`3598e9e5a49a389007d104fa0636831c`.

## Findings

1. **Every error the `windowsnt` arm reports is in a mirror copy or in a registered mirror gap; the
   unit's own text has exactly three findings, and all three are the mirror's missing bitfield
   storage** (② row 3).  The attribution of each row is in ②'s table; the fix for every one of them
   is a mirror or baseline change outside this lane's write set, and they are Owed 1-4 and 7.
2. **`include/w32term.rs`'s head note is the design this unit follows**: the `<windows.h>` names the
   mirror uses "because that target supplies them from `<windows.h>`" are supplied here — and ②'s two
   `E0425`s are the two names this unit's own C text does not need (`KEY_EVENT_RECORD` is a
   `<wincon.h>` record the *mirror's* `w32_kbd_patch_key` declaration names; `font` is `font.h`'s
   record, which the mirror spells as a pointer to a type this graph cannot have — its own note
   already degrades the same pointer to `*mut c_void` for `w32_system_remap_font` at 1428).  This
   unit declares the names its own transcription takes (plus `UINT_PTR`/`LPCWSTR`, the two scalars
   `AppendMenuW_Proc`'s alias needs) and records the other two as owed rather than inventing surface
   C's text of *this* file never names.
3. **The two `HAVE_W32NOTIFY` arms are the ones C's own build cannot reach here.**  `HAVE_W32NOTIFY`
   (`undef` in the frozen table) implies `HAVE_NTGUI` in C's world, and `DebPrint` is defined by
   `conf_post.h` 160-167 only under `HAVE_NTGUI`; the port carries C 78's call as `DebPrint!` (the
   `include/conf_post.rs` 182 macro, itself `#[cfg (feature = "have_ntgui")]`), so the arm is
   measured with `have_ntgui` on (③) and adds no error.

## Deviations

1. **The include graph carries the file gate, not C's own placement.**  C 21-24 sit *before* C 26's
   `#ifdef WINDOWSNT`, so in C they are ungated; the port puts them under `#[cfg (feature =
   "windowsnt")]` because `include/config.rs` is the only edge whose target is safe to expand on the
   frozen target (P-86 ⑧; the `src/w32fns.rs` 27-35 reading of the same position).  C 20 stays
   ungated, as C has it.
2. **The `<windows.h>` / `<windowsx.h>` / `<wtsapi32.h>` stand-in.**  `#ifdef WINDOWSNT` at C 26 is
   the file gate's own condition, so its body carries `windowsnt` alone; C 27-30's `#undef`/`#define`
   pair has no Rust counterpart and its value is the `_WIN32_WINNT` constant.  C 31-36's
   `#ifndef MINGW_W64` names the *non*-frozen side (`mingw_w64`), and C's enum keeps its implicit
   values as `WTSVirtualClientData = 0`/`WTSVirtualFileHandle = 1`.  The three SDK headers have no
   file in the reference tree, so their names stand in this unit (P-88 ①) — and because the mirror
   this unit includes (`include/w32term.rs`) leaves the same names to its includer, they include the
   mirror's own needs (`MSG`, `RECT`, `HCURSOR`, `HPALETTE`, `PALETTEENTRY`, `WINDOWPLACEMENT`,
   `WM_USER`, `WM_MOUSELAST`).  `WM_USER`/`WM_MOUSELAST` are `c_int` because that mirror's constants
   add to them in that type; `WM_PAINT` and `GDI_ERROR` take their comparison site's type (`UINT`)
   the way `src/w32menu.rs` 245 and `src/w32fns.rs` 226-262 read theirs.  Every record and value in
   the section is `UNVERIFIED` (no SDK here); the layout assertions are Owed 5.
3. **`WTS_SESSION_LOCK` is `WPARAM`, `WM_WTSSESSION_CHANGE` is `c_int`.**  C's `#define`s have no
   type until their use site: C 435 compares the constant with `msg.wParam` (a `WPARAM`), so the port
   declares it in that type and C's own comparison needs no cast; C 434's case label meets
   `msg.message` (a `UINT`) through 4's single scrutinee cast.  `src/w32fns.rs` 920-922 types both
   `UINT` for an unported use site of its own; the two units are separate crates (P-90's seventh
   half), so the two spellings do not meet.
4. **`drain_message_queue`'s `switch (msg.message)` is a `match` with one cast at the scrutinee.**
   `msg.message` is a `UINT` and the mirror declares `WM_EMACS_FILENOTIFY` as a `c_int`, so the
   scrutinee is cast once (`msg.message as c_int`) — the `src/w32menu.rs` 708-717 reading of a `UINT`
   message and a `c_int` constant.  `switch`/`case`/`break` have no Rust token, so C's four arms are
   `match` patterns and the `default` is the `_ => {}` arm; the row is in
   `docs/ports/macro-body-notes.tsv`.
5. **`if (nQueue++)` is bound then stepped.**  Rust has no post-increment operator, so C 384's
   `if (nQueue++)` reads the pre-step value into a local and steps the counter; `nQueue--`/`nQueue++`
   become `nQueue -= 1`/`+= 1` (the `src/w32term.rs` 1573 reading of its own post-increment loop).
6. **C's implicit `void *` conversions are casts.**  C 374/403's `(int_msg *) myalloc (…)` keeps C's
   own cast; `myfree (lpCur)` (C 298/330) and `free (ns)`/`free (notifications_set_head)` (C 120/121/
   124) pass a typed pointer to a `void *` parameter, so the port writes `as *mut c_void` — the
   `src/w32inevt.rs` 1337-1338 shape.  `malloc`'s and `GlobalAllocPtr`'s size argument is `usize`
   (`SIZE_T`), which is why `myalloc (sizeof (int_msg))` needs no second cast.
7. **`w32_strerror`'s declaration is kept.**  C 355 is a declaration of `w32.c`'s entry point at its
   own C position; the two mirrors that carry it are not on this file's include graph, so the unit
   spells it (the `src/w32fns.rs` 117-119 reading).  It is not a forward declaration of a definition
   in this file, so PORTING.md 0.1-12 does not apply.
8. **`CRITICAL_SECTION critsect` is zero-initialized explicitly.**  C's tentative definition
   (`CRITICAL_SECTION critsect;`) has no initializer; Rust's statics need one, so the port writes
   C's zero value field by field (PORTING.md 1.5; `src/w32console.rs` 411-413's `keyboard_handle` is
   the same class).  C 53's `static HANDLE input_available = NULL` and C 269-271's three `static`
   objects keep their own `NULL`/`0` initializers.
9. **`FOR_EACH_FRAME`'s body is the macro's last argument and the two loop variables are C's own
   block locals** (`include/frame.rs` 793's documented deviation); `SET_FRAME_GARBAGED` is the
   mirror's `macro_rules!` of the same name.
10. **The `FRAME_OUTPUT_DATA` / `FRAME_W32_P` / `FRAME_PIXEL_*` / `FRAME_DISPLAY_INFO` accessors are
    the mirrors'**, used where C uses the macro; where C writes the `->` chain itself
    (`f->output_data.w32->old_palette` at C 148/151/167, `->window_desc` at C 253) the port writes
    the same chain with the one cast the still-untyped `output_data.w32` member needs (P-72 rule 3,
    `frame.rs`'s own note).  That spelling is also what keeps ⑥'s token check green.
11. **The two `w32_output` bitfields are read and written as C's fields** (C 194/207/230), even
    though `include/w32term.rs` has no storage for them — see ② row 3 and Owed 3.  The alternative
    (a stub or a dropped statement) is what P-86 ① forbids.

## Owed (nothing here is claimed as done)

1. **`include/w32term.rs`: 12 top-level declarations move into a glob-imported module.**  The mirror
   declares `prepend_msg` (1007), `critsect` (1032-1035's four) and the eight entry points at
   1059-1065 at its crate root; this unit *defines* all twelve, so the arm reads 12 `E0428`s.
   `include/w32term.rs` already has the fix shape (`w32term_fn_decls` 1400, `w32term_objs` 1450,
   `use …::*;` at 1436/1488) for other names — the same move retires all twelve (P-39/P-21).
   **`femacs/src/include/**` is not this lane's writable set.**
2. **`include/w32term.rs`: two unresolved types.**  `KEY_EVENT_RECORD` (mirror 344; C's `w32term.h`
   261) is a `<wincon.h>` record the *includer* supplies — this unit does not declare it because its
   own C text never names it, and `src/w32inevt.rs` 155-200 already declares the record in its own
   stand-in for the unit that needs it.  `font` (mirror 447; C's `w32term.h` 335) is `font.h`'s
   record, which this graph cannot reach: the fix is the P-72 rule-3 degradation that the same mirror
   already applies to `w32_system_remap_font` (1428); `src/w32term.rs` includes this mirror too,
   so its own arm reads the same `E0425`.  The mirror's owner decides.
3. **`include/w32term.rs` 523: `w32_output`'s two bitfields have no storage unit.**  P-80 requires a
   storage unit plus a member-dispatch accessor (the `glyph_bits!`/`process_bits!` shape,
   `src/process.rs` 766); with the accessor, this unit's C 194/207/230 write
   `w32_output_bits! (paint_buffer_dirty, output[, 1])` / `!w32_output_bits! (want_paint_buffer, …)`
   and the three `E0609`s of ② disappear.  The same gap costs `src/w32term.rs` five sites
   (776/793/1375/1383/1396).
4. **A *stable* toolchain adds one `E0658` in `lib/include/lisp.rs`.**  Its 3761 `verror`
   declaration names `core::ffi::VaList`; the pinned `nightly-2026-10-01`
   (`femacs/rust-toolchain.toml`) compiles it, while a stable 1.98.1 run — what a caller whose cwd
   is outside `femacs/` gets — reports the unstable `c_variadic` feature.  That is the pin's own
   subject (the manifest's comment records the resolution), not a row of this unit, and it is
   recorded here only so the two readings of ② are not confused.  With the pinned toolchain the
   arm's 17 errors are the 12 `E0428`s and 2 `E0425`s of Owed 1/2 plus this unit's three Owed-3
   sites.
5. **Every `<windows.h>`/`<windowsx.h>`/`<wtsapi32.h>` name this unit stands in for is
   `UNVERIFIED`**: the six records (`POINT`, `RECT`, `MSG`, `PALETTEENTRY`, `WINDOWPLACEMENT`,
   `CRITICAL_SECTION`) plus C 32-35's enum, the 28 type aliases, the twelve constants and the 26
   entry points.  Their layout
   assertions (`xtask layout-asserts` reads a C compiler's readings) and their values need a
   Windows/mingw toolchain, which this machine has not got.
6. **`docs/port-map.md` 588's port count** still reads 11, and `docs/baseline/port-lines-pending.tsv`
   8 still carries the stale `src/w32xfns.c / 447 / 11 / 44` ratchet row: the integrator's
   regeneration (`--write-pending`).  Neither file is this lane's writable set.
7. **`docs/ports/line-drift-tolerance.tsv` needs one row.**  The reading is `line count drifted
   103.8%: C 447 vs Rust 911` (911/447 = 2.038), so the row is
   `w32xfns\t1.04\t<reason>\ttotal` — reason: the `windowsnt` file gate on every item and the whole
   include graph, both arms of the three inner conditions kept as `#[cfg]` text, and the
   `<windows.h>` surface declared in place with a `/// C:` line per name (P-88 ①; no SDK on this
   machine, so every record and value there is UNVERIFIED).  That file is not this lane's writable
   set, so `extract-abi.py --check` stays `FAIL` with exactly the one finding ⑥ records.
8. **`docs/ports/macro-body-notes.tsv` was appended, not rewritten** (one `#` header + one row for
   `drain_message_queue`'s `break,case,switch`), because the working tree already carried another
   lane's three added lines.

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte object; `build-tu` spent 2 of its 3), so this is a *text* port: no
  Windows target, no critical section, no event, no DC, no palette, no message queue and no
  `PeekMessage` loop was involved, and no ERT/differential/device evidence exists.
- **The arm reading is not a Windows compile.**  This machine has no `<windows.h>`, so Evidence ②-③
  are `rustc` readings of the *materialized mirror graph* under four `--cfg` sets.  This unit's own
  text contributes exactly three findings and they are the mirror's missing bitfield storage (②,
  Owed 3); "3 findings" means the other 14 errors' primary spans are in mirror copies — **not** that
  the arm links, and not that the arm is a working Windows configuration.
- **The `<windows.h>` records and values stay unmeasured** (Owed 5): every one is `UNVERIFIED`,
  with the documented SDK text as its only source.
- **`extract-abi --check` still FAILs for the file**, by construction: the line-count row is the one
  finding left, and it is the integrator's tolerance row (Owed 7).  Nothing in this ledger claims the
  file's checklist is non-empty (P-86 ②) or that the port is complete.
