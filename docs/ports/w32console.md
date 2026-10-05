# emacs/src/w32console.c (1262 lines) — whole file, C 1-1262

> **One of P-86's class: the frozen configuration compiles nothing here.**  `w32console.o` is added
> to `W32_OBJ` by `configure.ac` 3051 — the `else` arm of the Cygwin split inside the `HAVE_W32`
> test (3019-3030) whose 3024 `AC_DEFINE`s `HAVE_NTGUI` — and `docs/baseline/frozen-build-sources.txt`
> lists no `w32console.o` for this target, so `extract-abi.py`'s checklist for the file is empty
> (P-86 ②) and this ledger is that clause's substitute: the **C line-range table**, the `## Evidence`
> readings and the `## Deviations`.  Port: `femacs/src/w32console.rs` (**1995 lines**, md5
> `af0c3a190a1ead655989af0cb0bfb445`), carrying **C 1-1262 with both arms of every `#if`**.  The
> reference `emacs/src/w32console.c` is md5 `7505293a1e206bbd66090144986fb7a6` (unchanged; `emacs/`
> is read-only).
>
> **This ledger replaces the reading it inherits.**  The previous `docs/ports/w32console.md` (and the
> 19-line comment unit it described) read "the frozen configuration does not compile this file ⇒ the
> port is the empty unit plus the note that names the gate" (the P-08 reading, `docs/status.md`
> 3.31).  P-86 ① replaces that conclusion with *transcribe the text, both arms, no stub*; what the
> earlier note measured (not in the frozen source list, empty checklist, 336-byte `build-tu` object)
> is unchanged and still true — it is the *conclusion* that is withdrawn, exactly as
> `docs/ports/w32menu.md`, `docs/ports/w32term.md` and `docs/ports/w32heap.md` record for their own
> pairs.
>
> **The file gate is `windowsnt`, and the arm set is `windowsnt` + `have_ntgui` +
> `have_window_system`.**  `HAVE_NTGUI` (3024) is on for *both* the Cygwin and the native Windows
> GUI build, while `w32console.o` joins `W32_OBJ` only in the non-Cygwin arm (3051) — the build whose
> `nt/inc/ms-w32.h` 23-25 `#define`s `WINDOWSNT`.  The three units of that same arm already landed
> with `windowsnt` as their file gate (`src/w32.rs`, `src/w32heap.rs`, `src/w32proc.rs`), so this
> one does too; `have_ntgui` and `have_window_system` are set in the arm reading because the mirrors
> this unit reaches (`termhooks.rs`, `dispextern.rs`, `w32term.rs`) spell their platform arms with
> them.
>
> **Acquisition: this piece is `text-only` (P-86 ③⑵).**  The machine has no Windows and no mingw
> toolchain (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no
> Homebrew `mingw-w64`, and `find` finds no `windows.h`/`winnt.h`/`wincon.h`), so there is **no
> `<windows.h>` to compile or measure against**, no arm flip on a Windows target and no layout
> measurement: every record and every value of the `<windows.h>` stand-in section is `UNVERIFIED`
> (the documented SDK text, not a measurement) and the layout assertions are owed.  What the round
> does carry is the frozen `build-tu` reading, the machine-checked coverage table, the `#if`
> verdicts, a per-item C citation, the `probe` reading of the arm, and a **mutation test** over this
> unit's own lines.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`W32_OBJ`, `configure.ac` 3022-3051; the native-Windows `else` of the Cygwin split) | `feature = "windowsnt"` (on every item *and* on C 28-44's include graph) | the native MS Windows GUI build (`nt/inc/ms-w32.h` 23-25 defines `WINDOWSNT`); not the Cygwin GUI build |
| C 42-44 `#ifdef WINDOWSNT` / `#include "w32.h"` / `#endif` | the file gate itself (`windowsnt`); the `#else` arm is C's absent one, and the **edge is dropped** (see Deviations 2) | as above |
| C 73-75 `#ifndef USE_SEPARATE_SCREEN` / `static CONSOLE_CURSOR_INFO prev_console_cursor;` / `#endif` | `not (feature = "use_separate_screen")`; the `#else` arm is C's absent one | `USE_SEPARATE_SCREEN` is not in `docs/baseline/config/darwin-arm64.macros.tsv` (undefined), so this is the frozen arm |
| C 682-688 `#ifdef USE_SEPARATE_SCREEN` / C 1030-1046 the same pair | `feature = "use_separate_screen"` for the first arm, `not (feature = "use_separate_screen")` for the second; both arms carried | the `#else` arm is the frozen one |

`tools/check-cond-eval.py src/w32console.c` reads **`4 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence ④): C 42, C 73, C 682 and C 1030 — those four rows are the whole conditional
text of the file.  `HAVE_NTGUI`, `WINDOWSNT` and `HAVE_WINDOW_SYSTEM` are all `undef` in the frozen
macro table, and the C file's own three macros (`DEFAULTP`, `SEQMAX`, `SSPRINTF`) are carried as
`macro_rules!` of the same names (C 99-111).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-17 | the file's licence/declaration block | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 19-25 | the authors and the Virtual Terminal note | carried as C's own comment |
| 28 | `#include <config.h>` (plus config.h's own AH_BOTTOM `conf_post.h`) | file gate |
| 30 | `#include <stdio.h>` | file gate |
| 31 | `#include <windows.h>` | file gate; in-place stand-in section (P-88 ①, UNVERIFIED) |
| 33-40 | `lisp.h`, `coding.h`, `termchar.h`, `dispextern.h`, `menu.h`, `w32term.h`, `w32common.h`, `w32inevt.h` | file gate |
| 42-44 | `#ifdef WINDOWSNT` / `#include "w32.h"` / `#endif` | file gate; **edge dropped** (Deviations 2) |
| 46-62 | the fifteen `static` prototypes | dropped (PORTING.md 0.1-12) |
| 64-75 | the seven file-scope statics incl. C 73-75's `#ifndef USE_SEPARATE_SCREEN` | file gate / `not (use_separate_screen)` |
| 77-83 | `keyboard_handle`, `w32_console_unicode_input`, `w32_use_virtual_terminal`, `current_tty` (four `extern` + definition pairs) | file gate; the `extern`s dropped (0.1-12) |
| 85-97 | `ctrl_c_handler` (C 89's prototype is C's own repeat of the definition) | file gate |
| 99-102 | `#define DEFAULTP(p)` | file gate (`macro_rules!`, C's three-way `==` chain) |
| 104 | `#define SEQMAX 256` | file gate |
| 106-111 | `#define SSPRINTF(buf, i, sz, fmt, ...)` | file gate (`macro_rules!`; see Deviations 10) |
| 113-122 | `w32con_write_vt_seq` | file gate |
| 124-130 | `w32con_get_cursor_coords` | file gate |
| 132-142 | `w32con_move_cursor` | file gate |
| 144-150 | `w32con_hide_cursor` | file gate |
| 152-158 | `w32con_show_cursor` | file gate |
| 160-183 | `w32con_set_cursor_size` | file gate |
| 186-192 | `w32con_clear_to_end` | file gate |
| 194-213 | `w32con_clear_frame` | file gate |
| 216-219 | `glyph_base` / `glyphs` / `glyphs_len` / `ceol_initialized` | file gate (C 218's `countof`, Deviations 7) |
| 221-247 | `w32con_clear_end_of_line` | file gate |
| 249-316 | `w32con_ins_del_lines` | file gate |
| 318-321 | `#undef LEFT` / `#undef RIGHT` / `#define LEFT 1` / `#define RIGHT 0` | file gate (Deviations 6) |
| 323-354 | `scroll_line` | file gate |
| 357-376 | `w32con_insert_glyphs` | file gate |
| 378-467 | `w32con_write_glyphs` | file gate |
| 469-534 | `w32con_write_glyphs_with_face` | file gate |
| 536-620 | `tty_draw_row_with_mouse_face` (C 591-616 the `switch`, Deviations 9) | file gate |
| 622-630 | `w32con_delete_glyphs` | file gate |
| 632-658 | `w32con_setup_virtual_terminal` | file gate |
| 660-691 | `w32con_reset_terminal_modes` (C 682-688 the `#ifdef USE_SEPARATE_SCREEN` pair) | file gate / both `use_separate_screen` arms |
| 693-714 | `w32con_set_terminal_modes` | file gate |
| 716-734 | the update comment, `w32con_update_begin`, `w32con_update_end` | file gate |
| 736-753 | the `stubs from termcap.c` banner, `sys_tputs`, `sys_tgetstr` | file gate |
| 756-805 | the `stubs from cm.c` banner, `cost`, `evalcost`, `cmputc`, `cmcheckmagic`, `cmcostinit`, `cmgoto`, `Wcm_clear` | file gate |
| 808-821 | the cursor-position comment, `cursorX`, `cursorY` | file gate |
| 823-825 | the `Faces` banner | file gate |
| 828-861 | `w32_face_attributes` (C 840/854/857 the `face` reads) | file gate |
| 863-950 | `turn_on_face` (C 874's VLA, C 877-947 the fifteen `SSPRINTF`s) | file gate |
| 952-957 | `turn_off_face` | file gate |
| 959-975 | the IME comment, `find_ime_window` | file gate |
| 977-1156 | `initialize_w32_display` (C 1030-1046 the `#ifdef USE_SEPARATE_SCREEN` pair) | file gate / both `use_separate_screen` arms |
| 1159-1185 | `DEFUN ("set-screen-color", …)` | file gate |
| 1187-1204 | `DEFUN ("get-screen-color", …)` | file gate |
| 1206-1221 | `DEFUN ("w32-use-virtual-terminal", …)` | file gate |
| 1223-1228 | `DEFUN ("w32-use-virtual-terminal-p", …)` | file gate |
| 1230-1237 | `DEFUN ("set-cursor-size", …)` | file gate |
| 1239-1262 | `syms_of_ntterm` (the two `DEFVAR_BOOL`s and the five `defsubr`s) | file gate |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32c-lane/target cargo run -q -p xtask -- build-tu
src/w32console.c` → `built src/w32console.rs -> ../build/femacs/w32console.o`, **exit 0**, and
`build/femacs/w32console.o` is **336 bytes** — an empty object, the expected reading (every item
sits behind the file gate, so under the frozen feature set rustc compiles only the header comment
and the `#![allow]`).  The first run of this same command **failed**: 5 parse errors in the unit's
own text (`as c_int < 20` parses as generic arguments for `c_int`), which is the one thing that
door measures — cfg-stripping happens *after* parsing, so a syntax error inside a `#[cfg]`-ed-out
item still fails it.  Fixed by parenthesising the cast (`(x as c_int) < 20`); the reading above is
the re-run.

**② The arm flip (P-86 ③⑴; the file gate plus the two mirrors' platform arms).**
`cd femacs && CARGO_TARGET_DIR=/tmp/w32c-lane/target cargo run -q -p xtask -- probe
src/w32console.rs /tmp/w32c-lane/w32c.o --crate-type=lib -C panic=abort --cfg
'feature="have_ntgui"' --cfg 'feature="windowsnt"' --cfg 'feature="have_window_system"'`
→ **exit 1, 10 errors anchored in three mirror copies and 0 in `_root.rs`**:

| anchor | count | what |
| --- | --- | --- |
| `include/w32term.rs` copy | 8 | 3 × `E0428` (`w32con_hide_cursor` 346, `w32con_show_cursor` 347, `initialize_w32_display` 1123 — the mirror declares at top level three names **this unit defines**) + 5 × `E0425` (`RECT` ×3, `HDC`, `COLORREF` at 1421/1422/1428/1436/1445 — the documented Windows-SDK-name residue) |
| `include/w32inevt.rs` copy | 1 | `E0428` `w32_console_unicode_input` (its C 23 top-level declaration vs this unit's definition, C 79) |
| `include/frame.rs` copy | 1 | the file's own deliberate `compile_error!` (its note asks for a Windows/mingw clang reading of frame.h 471's flag run) |
| `_root.rs` (this unit) | 0 | — |

25 warnings accompany it: 24 × `unused doc comment` (5 of them on this unit's own
`///`-documented `unsafe extern "C"` blocks — the `src/w32term.rs` 1664-1670 style, which has the
same warnings) and 1 × `unused attribute` (`include/frame.rs` 1432).

**③ The mutation test: the "0 in the unit" reading is not vacuous.**  `include/w32term.rs` 1123 and
`include/w32inevt.rs` 19 are name-resolution/type-collection errors, and P-86 ⑦'s trap is exactly
this: if an earlier phase aborts before type-checking, "0 errors in the unit" would be an empty
reading.  A copy of the materialized root in `/tmp` with one injected type error inside
`w32_face_attributes` (`let _mutant: c_int = "w32console_arm_probe_no_such_type";`) compiled under
the same three `--cfg`s reports **one more error, anchored at `_root.rs` and naming the mutant**
(11 errors / 13 `^error` lines vs. the base reading's 10 / 12).  So type-checking does reach this
unit's bodies and Evidence ②'s classification is a real reading of the unit.

**④ The `#if` verdicts.**  `python3 tools/check-cond-eval.py src/w32console.c` →
`4 conditions, 0 unknown, 0 disagreements [VERIFIED]` (C 42, 73, 682, 1030), which is why this file
*is* on `docs/baseline/config/cond-eval-verified.txt` and the frozen macro table decides its arms.

**⑤ `extract-abi --check`.**  `python3 tools/extract-abi.py --check emacs/src/w32console.c
femacs/src/w32console.rs` → one finding, no others: `line count drifted 58.1%: C 1262 vs Rust
1995`.  The three findings the first run carried are closed: the `SSPRINTF` macro's two tokens and
syms_of_ntterm's two are the two `docs/ports/macro-body-notes.tsv` rows this round appended
(Deviations 10/11 record why), and `Fget_screen_color`'s docstring was a **one-character** copy
error — the reference's own C 1191 ends `` `tty-color-alist`. `` with a backtick where C 1162's
sibling sentence ends it with an apostrophe, and the port had copied the apostrophe.

## Deviations

1. **The fifteen `static` prototypes (C 46-62) are dropped** (PORTING.md 0.1-12: Rust has no static
   function prototypes; every one of the fifteen is defined in this file, in C's order).
2. **C 42-44's `w32.h` include edge is dropped, and it is recorded here rather than hidden.**  The
   C include exists "for syms_of_ntterm" (C 43) — the function this unit *defines* at C 1240 — and
   `include/w32.rs` declares that name in its **top-level** `unsafe extern "C"` block (its C
   275-490), not in one of the module + glob-import blocks it uses for the names units define.  A
   unit that both includes that mirror and defines the function is `E0428`; C's own declaration is
   redundant here, and the `#else` arm is C's empty one.  `## Owed` 1 names the mirror row that lets
   the edge return.
3. **C 31's `<windows.h>` is an in-place stand-in section** (P-88 ①/P-90 ④): the scalar typedefs,
   the six console records (`COORD`, `SMALL_RECT`, `CHAR_INFO` with its named `CHAR_INFO_Char`
   union, `CONSOLE_SCREEN_BUFFER_INFO`, `CONSOLE_CURSOR_INFO`), the twenty entry points and the
   eighteen `#define` values this file spells.  **Every one is UNVERIFIED** (no SDK on this machine)
   and `## Owed` 4 lists the layout assertions.  `WriteConsoleOutputCharacter` /
   `FillConsoleOutputCharacter` are the names C's text writes; the `<wincon.h>` macros select the
   `A` entry point when `UNICODE` is not defined, which is the MinGW build's case, so the port
   declares that arm's `LPCSTR`/`CHAR` signature.
4. **Seven names the include graph does not carry are declared at their use sites** (P-66/P-72 rule
   4): `tty_defined_color` (dispextern.h 3796), `root_xy` (dispextern.h 4020), `root_frame`
   (frame.h 1508), `write_glyphs` (dispextern.h 3942), `encode_terminal_code` (termhooks.h 973),
   `exit` and `atoi` (`<stdlib.h>`'s, which `lib/include/stdlib.rs` does not carry).  Each keeps C's
   prototype in a `/// C:` line; `## Owed` 6 names the mirror rows.
5. **`struct face` / `struct face_cache` / `FACE_FROM_ID` are declared in this unit**, because
   `include/dispextern.rs` keeps both records behind `dispextern_face_records` (a layout arm this
   unit does not set) and `include/frame.rs` keeps the accessors in
   `pub mod frame_face_accessors`, which it does not glob-import.  The frozen 280-byte record and
   the `bit` masks are `src/term.rs` 302-403's copy, repeated with the measured offsets (P-72: a
   unit may only name what it declares or includes); `## Owed` 7 records the shared row.
6. **C 318-321's `#undef LEFT` / `#undef RIGHT` have no Rust counterpart**; no mirror on this graph
   declares either name, so the redefinition C ends with is carried as `const LEFT: c_int = 1;` and
   `const RIGHT: c_int = 0;` and the two `#undef`s are recorded in the block's comment.
7. **C 218's `countof (glyph_base)` is the literal `80`.**  Rust forbids a shared reference to a
   `static mut` in a `static` initializer (`E0133`, measured on a two-line probe), so C's
   `sizeof`/`sizeof[0]` fold cannot be written there; the constant it evaluates to is written out
   with the C text in the block's comment.  C's `atoi`/`exit` (C 1065-1066/1041) come from `<stdlib.h>`
   by way of lisp.h, and `min` (C 1074-1075) from lisp.h 71-73's own `#undef`/`#define` pair — both
   are the use-site declarations of Deviation 4 / the `lib/include/min-max.rs` mirror.
8. **C's chained assignments are one statement each** (`dest.X = dest.Y = 0` at C 207,
   `clip.Top = clip.Left = scroll.Left = 0` at C 271, the four-way `clip.Top = scroll.Top = …` at
   C 332, `clip.Right = scroll.Right = …` at C 272): Rust has no assignment expression, so each
   target gets C's own order and the block's comments say which C line the pair came from (the
   `src/w32term.rs` 880-888 reading of the same class).
9. **C 591-616's `switch (draw)` is the port's `match`**, and because C's `switch` needs no
   `default` while Rust's `match` does, the six `DRAW_*` arms are followed by `_ => {}` (the
   `src/term.rs` 3240-3242 shape).  C's `emacs_abort ()` arm keeps its four enumerators.
10. **`SSPRINTF` is a variadic `macro_rules!`, not a copy of C's preprocessor arguments.**  The
    `do { … } while (0)` becomes the one-shot block (PORTING.md 2.2), so `do`/`while`/`0` have no
    counterpart token, and `__VA_ARGS__` becomes `$($args:expr),*`; C's `if (fmt && …)` truth test is
    the pointer's `! ($fmt).is_null ()`, and the `*i < sz` / `*i += snprintf (…)` mix of `DWORD`
    (`u32`) with `size_t` (`usize`) is written out (`as DWORD`/`as usize` and the `wrapping_add`
    that is C's unsigned `+=` of a possibly-negative `int`).  C 874's `char seq[sz]` is C99's VLA
    whose size at the declaration is `SEQMAX` (`sz--` follows it), so the port is a
    `[c_char; SEQMAX]`.  Both tokens are the `docs/ports/macro-body-notes.tsv` row from Evidence ⑤.
11. **The `w32.h` include edge's three names are not stubbed**: `w32con_hide_cursor`,
    `w32con_show_cursor` and `initialize_w32_display` keep C's external linkage and are the
    single definitions the `include/w32term.rs` declarations collide with (Evidence ②), and
    `w32__terminal_is_conhost` / `w32_use_full_screen_buffer` are defined here because this unit's
    two `DEFVAR_BOOL`s (C 1242/1251) are what registers them (the `src/coding.rs` 237-238
    precedent: the explicit item shadows `globals.rs`'s glob-imported `extern`).  `## Owed` 1 covers
    the collisions.

## Owed

1. **Four mirror rows, one per `E0428` in Evidence ②.**  `include/w32term.rs` declares
   `w32con_hide_cursor` (346), `w32con_show_cursor` (347) and `initialize_w32_display` (1123) in a
   top-level `unsafe extern "C"` block, and `include/w32inevt.rs` declares
   `w32_console_unicode_input` at 19 the same way; each of the four is a name some *unit* defines
   (this one defines all four), and the established fix is the one `include/w32.rs` already uses for
   `w32_get_internal_run_time` and `term_timers`/`init_timers`: move the declaration into a module
   the mirror glob-imports, so the unit's definition shadows it.  Not this lane's write set.
2. **The five `E0425`s in `include/w32term.rs`** (`RECT` at 1421/1422/1428, `HDC` at 1436,
   `COLORREF` at 1445) are that mirror's own `<windows.h>` names, undeclared because no
   `<windows.h>` mirror exists; they belong with `docs/ports/w32term.md`'s existing owed list, not
   to this file.
3. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `w32console`, tolerance ≥
   **0.581**, basis `total`, reason from Deviations 3-11.  That file is not in this lane's write
   set, so `extract-abi.py --check` stays `FAIL` with exactly the one row Evidence ⑤ records.
4. **The layout assertions and the measured values for the `<windows.h>` records** (`COORD`,
   `SMALL_RECT`, `CHAR_INFO`/`CHAR_INFO_Char`, `CONSOLE_SCREEN_BUFFER_INFO`,
   `CONSOLE_CURSOR_INFO`) and the eighteen mode/handle constants, once a Windows or mingw toolchain
   exists (`xtask layout-asserts` reads a C compiler's readings; `x86_64-w64-mingw32-gcc` is absent
   here).
5. **`include/frame.rs`'s deliberate `have_ntgui` `compile_error!`** (its own note asks for a
   Windows/mingw clang reading of frame.h 471's flag run) is one of the 10 errors Evidence ②
   reports; it is not this lane's to fix.
6. **The six mirror rows for Deviations 4's use-site declarations**: `include/dispextern.rs`
   (`tty_defined_color`, `root_xy`, `write_glyphs`), `include/frame.rs` (`root_frame`),
   `include/termhooks.rs` (`encode_terminal_code`) and `lib/include/stdlib.rs` (`atoi`, `exit`).
7. **The shared face-record copy (`## Deviations` 5)**: `src/term.rs` 302-403 and this unit carry
   the same frozen `face`/`face_cache`/`FACE_FROM_ID`.  Retiring the copies means either moving them
   into `include/dispextern.rs`'s ungated half or flipping `dispextern_face_records` for both units;
   both are mirror/other-lane work.
8. **`docs/baseline/port-lines-pending.tsv`**: regenerate with `tools/check-port-lines.py
   --write-pending` — the `src/w32console.c` ratchet row is stale now that the stem is far above the
   ten-percent threshold.  Not this lane's write set.
9. **`docs/baseline/feature-coverage.tsv` and `docs/status.md`**: the integrator's regeneration and
   running-log entry for this revision (the `docs/ports/w32menu.md` Owed 8/9 rows).

## Not claimed

- **No runtime evidence of any kind.**  The frozen configuration compiles nothing from this file
  (Evidence ①: a 336-byte empty object), so this is a *text* port: nothing here has been executed,
  and no Windows target, console handle, `COORD`, screen buffer or redisplay loop was involved.
  There is no behavioural test, no differential run, no ERT and no device evidence.  In particular
  C's stubs (`sys_tputs`, `sys_tgetstr`, `evalcost` returning its own argument, the four empty cm
  entry points) are transcribed as text, not exercised.
- **The arm reading is not a Windows compile.**  This machine has no `<windows.h>`, so Evidence ②
  is a `rustc` reading of the *materialized mirror graph* under three `--cfg`s; ten of its errors
  are in mirror copies and "0 in the unit" means only that no error's primary span is one of this
  unit's lines (Evidence ③ shows the reading is not vacuous) — **not** that the unit links, and not
  that the arm is a working Windows configuration.
- **The `<windows.h>` records and values stay unmeasured** (Owed 4): every one is `UNVERIFIED`,
  with the documented-sibling-file precedent as its only source.
- **`extract-abi --check` still FAILs for the file**, by construction: the line-count row is the one
  finding left, and it is the integrator's tolerance row (Owed 3).  Nothing in this ledger claims
  the file's checklist is non-empty (P-86 ②) or that the port is complete.
