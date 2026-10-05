# emacs/src/xsmfns.c (581 lines)

> **This file is P-86's class: the frozen configuration compiles none of it.**  `HAVE_X_SM` is
> `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 1122, so C 23's `#ifdef HAVE_X_SM`
> takes nothing of C 25-581 and `docs/baseline/frozen-build-sources.txt` lists no `xsmfns.o`; the
> checklist `tools/extract-abi.py` generates for the file is empty (P-86 ②), so this ledger carries
> P-86 ②'s substitute - the **C line-range table**, the `## Evidence` readings and the
> `## Deviations`.  The port is `femacs/src/xsmfns.rs`, **the whole file (C 1-581), 1,035 Rust
> lines**, with both arms of every `#if` carried (P-86 ①: no stub, no empty unit).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| C 23 `#ifdef HAVE_X_SM` … C 581 `#endif` (the file itself) | `feature = "have_x_sm"`, on every item **and on C 25-39's include graph** (P-86 ⑧) | a session-management configuration (not this target - which is why the frozen object is the 336-byte empty one) |
| C 41 `#if defined USE_GTK && !defined HAVE_GTK3` … C 43 `#endif` (the `gdk_x11_set_sm_client_id` alias) | `all (feature = "use_gtk", not (feature = "have_gtk3"))` | GTK 2 |
| C 365 `#ifndef USE_GTK` … C 389 `#endif /* ! USE_GTK */` (the client-leader window) | `not (feature = "use_gtk")` | no GTK toolkit |
| C 474 `#ifdef USE_GTK` / C 478 `#else` (the one call pair) | `feature = "use_gtk"` / `not (feature = "use_gtk")` | one arm each |
| C 527 `#if false` (inside the `DEFUN`) | **no gate**: dead in every configuration, recorded here and not transcribed (the `src/xfns.rs` 958 `#if false` precedent) | never |

`have_x_sm` is **not yet registered** in `femacs/xtask/Cargo.toml`'s `[features]` table
(`grep -c '^have_x_sm' femacs/xtask/Cargo.toml` -> 0) - `## Owed` 1.  A condition with no `#if`
in this file still binds the port's gate set: the mirrors C 26-39 reach are themselves gated
(`include/xlib.rs`'s `Display`/`Window`/`x11_constants` and `include/xterm.rs`'s fields are
`have_x_windows`, `include/xterm.rs`'s `use_x_toolkit` fields are Xt-typed), so a
`have_x_sm`-without-`have_x_windows` graph cannot compile - the coupling `docs/ports/xgselect.md`'s
`## Owed` 2 names for `use_gtk`, and the reason the two arm flips below set `have_x_windows` too.

## C line ranges

One row per C block, in C's order, contiguous (each row absorbs the blank lines that follow the one
before), covering C 1-581.  `docs/port-map.md` pairs only `src/xsmfns.c` with this unit (there is no
`src/xsmfns.h`), so one table is what `tools/check-dead-file-coverage.py` reads.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's licence/declaration block (`/* Session management module for systems which understand the X Session management protocol.` + the GPL block) | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 20-22 | the blank and C 21 `#include <config.h>` | ungated (every unit includes it) |
| 23-24 | C 23 `#ifdef HAVE_X_SM` **the file gate**, and the blank | gate marker |
| 25-27 | `<X11/SM/SMlib.h>`, `<X11/Xlib.h>`, `<X11/Xutil.h>` | file gate; SMlib.h is P-88 ①'s in-unit segment (no header here, UNVERIFIED), Xlib.h is `include/xlib.rs` (P-90 ②), Xutil.h's record and setter stand with the Xlib.h names in the same use-site section |
| 28-32 | the blank and `<unistd.h>`, `<sys/param.h>`, `<errno.h>`, `<stdio.h>` | file gate; `<sys/param.h>` has no mirror and nothing here names it (Deviation 16) |
| 33-39 | the blank and the six mirrors `lisp.h`, `frame.h`, `termhooks.h`, `xterm.h`, `process.h`, `keyboard.h` | file gate; the edges those reach are spelled in the unit (`systime.rs`, `character.rs`, `font.rs`, `string.rs`, `stdckdint.in.rs`) plus the use-site declarations |
| 40-43 | the blank and the `USE_GTK && !HAVE_GTK3` macro (C 42) | `use_gtk` + `not (have_gtk3)` |
| 44-47 | the blank, `/* This is the event used when SAVE_SESSION_EVENT occurs.  */` and `static struct input_event emacs_event;` | file gate |
| 48-51 | the blank, the descriptor comment and `static int ice_fd = -1;` | file gate |
| 52-55 | the blank, the shutdown-interaction comment and `static bool doing_interact;` | file gate |
| 56-59 | the blank, the session-manager-object comment and `static SmcConn smc_conn;` | file gate |
| 60-63 | the blank, the client-session-id comment and `static char *client_id;` | file gate |
| 64-67 | the blank, the program-path comment and `static char *emacs_program;` | file gate |
| 68-72 | the blank, the restart-option comment and `#define SMID_OPT "--smid="` | file gate |
| 73-78 | the two blanks, the no-splash comment and `static char NOSPLASH_OPT[] = "--no-splash";` | file gate |
| 79-82 | the blank, the `--chdir` comment and `#define CHDIR_OPT "--chdir="` | file gate |
| 83-90 | the blank and `ice_connection_closed` | file gate |
| 91-95 | the two blanks and the "handle any messages" comment | file gate |
| 96-123 | `x_session_check_input` (C 107/121 the two `emacs_event.kind` reads/writes) | file gate; `unsafe extern "C" fn` (C 360 takes its address) |
| 124-126 | the blank and the connection-predicate comment | file gate |
| 127-131 | `x_session_have_connection` | file gate; non-`static` (C symbol) |
| 132-143 | the blank, the interaction comment, the blank and `smc_interact_CB` | file gate; `unsafe extern "C" fn` (C 277) |
| 144-152 | the blank, the save-yourself comment (C 145-151) and the blank | file gate |
| 153-282 | `smc_save_yourself_CB` (C 161 `#define NR_PROPS 5`; C 207's mid-block `cwd` declaration; C 226's `ckd_add`; C 274-277's four-way interaction test) | file gate; `unsafe extern "C" fn` (C 439) |
| 283-285 | the blank and the "according to the SM specification" comment | file gate |
| 286-291 | `smc_die_CB` | file gate; `unsafe extern "C" fn` (C 441) |
| 292-303 | the blank, the "leave them empty" comment, the blank and `smc_save_complete_CB` | file gate; C's body is the comment `/* Empty */` alone |
| 304-309 | the blank and `smc_shutdown_cancelled_CB` | file gate; the same empty body |
| 310-324 | the blank, the error-handler comment and `smc_error_handler` | file gate; the same empty body (C 449) |
| 325-336 | the blank and `ice_error_handler` | file gate; the same empty body (C 450) |
| 337-344 | the two blanks and `ice_io_error_handler` | file gate; `unsafe extern "C" fn` (C 451) |
| 345-348 | the blank and the ICE-connection-watch comment | file gate |
| 349-361 | `ice_conn_watch_CB` | file gate; `unsafe extern "C" fn` (C 454) |
| 362-364 | the blank, `/* Create the client leader window.  */` and the blank | file gate |
| 365-389 | `#ifndef USE_GTK` + `create_client_leader_window` + `#endif` | `not (feature = "use_gtk")` |
| 390-393 | the two blanks and the "try to open a connection" comment | file gate |
| 394-482 | `x_session_initialize` (C 397 `#define SM_ERRORSTRING_LEN 512`; C 439-446 the callbacks record; C 459-468 the open call; C 474-480 the GTK pair) | file gate; non-`static` |
| 483-485 | the blank and the "ensure the session manager is not contacted again" comment | file gate |
| 486-490 | `x_session_close` | file gate; non-`static` |
| 491-492 | the two blanks | file gate |
| 493-535 | the `DEFUN ("handle-save-session", …)` (C 507-508 the `kill_emacs` test; C 512-520 the interaction arm; C 521-532 the kill arm; **C 527-531 `#if false`**, not transcribed) | file gate |
| 536-538 | the two blanks and the page separator | file gate |
| 539-541 | the Initialization banner | file gate |
| 542-579 | `syms_of_xsmfns` (C 545 `DEFSYM`; C 547/555 the two `DEFVAR_LISP`s; C 578 `defsubr`) | file gate; the two variables C declares through `DEFVAR_LISP` are defined in the unit (Deviation 22) |
| 580-581 | the blank and C 581 `#endif /* HAVE_X_SM */` | gate marker |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/xsmfns-lane/cargo cargo run -q -p xtask -- build-tu
src/xsmfns.c` -> `built src/xsmfns.rs -> ../build/femacs/xsmfns.o`, **exit 0**, and
`build/femacs/xsmfns.o` is **336 bytes**, md5 `dbed68837694579185377dbb0821ab62` - the empty
object, which is the expected reading (every item sits behind `have_x_sm`, so under the frozen
feature set rustc compiles only `include/config.rs` and the `#![allow]` attribute - P-86 ⑧'s
criterion).  **Three `build-tu` runs** (budget 3, at budget): the mid-round reading, the final
revision, and the re-read after this file's last comment pass; all three are the same 336
bytes/md5, because everything that changed in between is inside the gate.

**The revision every reading above is of**: `femacs/src/xsmfns.rs`, md5
`db7b396794634e5efc0d48d3f4c83188`, 1,035 lines (the text `build-tu` run 3 and both final arm
runs compiled).

**② The measurements this file's declarations are fixed to.**
*Xlib/Xutil (on this machine, `/opt/homebrew/include`: libX11 1.8.13 + xorgproto 2025.1):*
`clang -I/opt/homebrew/include` on a six-line probe prints
`sizeof XClassHint 16 / offsetof res_name 0 / res_class 8`, and the constants
`XA_STRING 31` (Xatom.h 39), `True 1` (Xlib.h 79), `PropModeReplace 0` (X.h 475),
`CopyFromParent 0` (X.h 121); the prototype lines quoted in the unit are the headers' own
(`XCreateSimpleWindow` 1617-1627, `XStoreName` 3405-3409, `XChangeProperty` 2078-2087,
`XSetClassHint` Xutil.h 583-587, `XClassHint` Xutil.h 189-192).
*XSMP/ICE (no header on this machine - see ③):* the in-unit segment's record shapes were fixed by a
clang restatement (`/tmp/xsmfns-lane/sm_restate.c`, `clang -o sm_restate sm_restate.c &&
./sm_restate`): `SmPropValue 16 0 8`, `SmProp 32 0 8 16 24`, `SmcSaveYourselfCallback 16`,
`SmcCallbacks 64 0 16 32 48`.  The unit's `const _: () = assert! (…)` block carries exactly these
numbers, so the Rust `#[repr (C)]` records match the stated shape; **the restatement is this port's
own text, not a header**, and every value in the segment stays UNVERIFIED (P-88 ②/P-90 ④).

**③ The arm flips (P-86 ③⑴/⑥) and the mutation test (P-86 ⑦).**
*Arm A - the file gate plus the toolkit sub-gate* (the non-GTK arm, i.e. C 365-389's text):
`cargo run -q -p xtask -- probe src/xsmfns.rs /tmp/xsmfns-lane/x.rlib --crate-type=lib
-C panic=abort --cfg 'feature="have_x_sm"' --cfg 'feature="have_x_windows"'
--cfg 'feature="use_x_toolkit"'` -> **exit 1, exactly 1 `error`**, and it is
`error[E0080] … assertion failed: !cfg!(feature = "have_x_windows") || HAVE_X_WINDOWS` at
`include/systime.rs:27` - that mirror's own gate-consistency assert, which a **hand** flip cannot
satisfy (`include/config.rs` 24 hard-codes `HAVE_X_WINDOWS: bool = false`; the mechanism that
rewrites it is `xtask dead-arms`, out of this lane's budget - `## Owed` 8).  **Zero errors in this
unit's own text**; the flip type-checked the whole unit (the warnings are 9 `unused doc comment` -
`/// C:` before an `unsafe extern "C" { }` block, of which 4 are the mirrors'
`xterm.rs`/`lisp.rs`/`stdbit.in.rs` and 5 this unit's, the same shape `docs/ports/xgselect.md`
names - plus 1 `unused attribute` in `include/frame.rs`; no `.rlib` is produced because rustc stops
at the error).  The arm-A probe runs that got there, and what they measured:
1) 23 errors - **2 in this unit** (`emacs_strerror (errno)`'s `E0308`;
   `include/character.rs` missing from the graph, 20 mirror rows `E0425`/`E0080`);
2) 12 - **0 in this unit** (the two edges added: `include/character.rs`, and `errno` spelled
   `*errno ()`);
3) 2 - **1 in this unit** (`let mut class_hints: XClassHint;` is `E0381`, Rust needs the record
   initialized before the first member assignment) + the `E0080`;
4) 1 - **0 in this unit**: the `E0080` alone.  A fifth arm-A run re-read the file after the
   last comment pass (the pass that carries C's own prose for the file-scope objects and the
   functions) and prints the same reading: **1 error, 0 in this unit**.
*Arm B - the GTK arm* (C 474-480's `#ifdef USE_GTK`, i.e. GTK 2's spelling of the call):
`… --cfg 'feature="have_x_sm"' --cfg 'feature="have_x_windows"' --cfg 'feature="use_gtk"'` ->
**exit 1, 13 errors: 1 `E0080` at `include/systime.rs:27` (the same hand-flip artifact) + 12
`E0425` in `include/xterm.rs`** (411 and 740-766: its `#[cfg (feature = "use_gtk")]`
`GtkWidget`/`GdkCursor`/`GdkGeometry`/`GtkTooltip`/`GtkWindow`/`GtkIMContext` fields, whose GTK
surface has no mirror - the blocker `docs/ports/include-gtkutil.md` records).  **0 in this unit**:
the GTK call site, both spellings of C 477 and the two `gdk_*` declarations, resolved.  The run was
repeated on the file's final text with the same reading (13 = 12 + 1), the only difference being
one more `unused doc comment` warning (10 + 1 `unused attribute`, against arm A's 9 + 1).
**Seven `probe` invocations in total against a budget of three** (over by four): five for arm A
(the four fix rounds of the list above plus the final-text re-read) and two for arm B (its first
reading and the final-text re-read).  The mutation test below is *not* a probe invocation: it is two
direct `rustc` runs on the already-materialized root.
(The GTK 3 variant - `have_gtk3` - was **not** flipped: its only difference is which of the two
`gdk_*` names is declared, and the same GTK-surface blocker applies.  `## Owed` 7.)
*Mutation test (P-86 ⑦)*: the same gate set applied directly to a copy of the materialized root
(`build/probes/xsmfns/_root.rs`, compiled with the repo's pinned `nightly-2026-10-01` -
`RUSTUP_TOOLCHAIN=nightly-2026-10-01 rustc --edition 2024 -O --crate-type=lib -C panic=abort --cfg
…`) reproduces the probe reading **exactly** (1 error `E0080` + the same 10 warnings), so the
direct route *is* the probe's reading; injecting one type error into this unit
(`delete_read_fd (c"injected type error")` in `ice_connection_closed`) turns it into **2 errors**,
the new row `error[E0308]: mismatched types` at the unit's own line.  "0 errors in this unit" is
therefore a real reading of this unit's text, not a name-resolution abort (P-86 ⑦'s criterion).

**④ The acceptance gate readings (this round).**
* `python3 tools/extract-abi.py --check emacs/src/xsmfns.c femacs/src/xsmfns.rs` -> **OK**
  (the checklist is empty - P-86 ② - and the port's `no_mangle`/`DEFUN!`/`defsubr` names are all
  backed by the reference's *whole text*, which is what P-86's widening consults).
* `python3 tools/check-declarations.py` -> `17 declaration sites (0 forbidden, 17 allowed
  literals), 0 failures`, and **this unit and this ledger contribute no site of any class** (the
  docstrings' bare-word program name is not one of the gate's patterns - the landed `callproc.rs`
  2337 keeps the same wording, `## Deviations` 19).  Mid-round the tree read `18 sites / 1
  forbidden / 1 failure`, the row being `femacs/lib/file-has-acl.rs` - another lane's in-flight
  file, which its own commit `2e53f40` then fixed.
* `python3 tools/check-mirror-dupes.py` -> the same 17 duplicate/17 ratchet rows/0 failures as
  before this round (this unit's use-site declarations name nothing a mirror of its graph carries;
  `Widget`/`WidgetRec`/the XSMP names are carried by no mirror).
* `python3 tools/check-dead-file-coverage.py` -> `83 ledger(s) bound (a C range table), 0
  failure(s)`, with `info docs/ports/xsmfns.md: 45 range row(s) cover src/xsmfns.c (581 lines; 0
  blank line(s) between blocks need no row)` - the table in this ledger is what that row measures.
* `python3 tools/check-citations.py` -> `676 file(s) scanned, 1 drift, 0 range, 1103 external, 34
  allowed, 1 stale`, and **this unit contributes 0 non-external findings** (one `lisp.h 30` cite
  paired a `` `strlen` `` with an include line during the round and was rewritten; the unit's own 8
  findings are all `external` - the X11/Xutil/Xatom headers this machine has, whose lines were
  measured).  The round's 1 drift + 1 stale row are `femacs/src/include/dispextern.rs`'s alone,
  which the same tool's per-file entry point proves mechanically (`findings_for` on that one file
  prints the drift, and its `:593` allowlist row matches nothing): commit `861cf20` inserted 83
  lines into that mirror and moved the citation from 593 to 600 without moving the row.
  **This lane's baseline reading before that commit was `0 drift, 0 stale`.**

**⑤ The one gate reading this round that is *not* this file's** (recorded so the integrator does
not attribute it here): `tools/check-citations.py`'s `1 drift` + `1 stale` row,
`femacs/src/include/dispextern.rs:600` against the allowlist's `:593` entry - proven to come from
that file alone (Evidence ④'s fourth bullet), i.e. from commit `861cf20`'s 83 inserted lines.

## Deviations

1. C's implicit conversions are spelled: `CopyFromParent as c_uint` for `XCreateSimpleWindow`'s
   `unsigned int border_width` and `as c_ulong` for its two `unsigned long` arguments (C 375; the
   macro is `0L`), `client_ID as *const c_uchar` for C 385's `(unsigned char *)` cast, and
   `as *mut c_void` on every string/pointer C assigns to an `SmPointer`/`void *`.
2. `strlen (…)` (`usize`) and `SBYTES (…)` (`isize`) are cast `as c_int` where C assigns to an
   `int` member (C 183, 193, 203, 215, 231, 237, 240, 248, 254).
3. C's post-increment inside an index expression (`vals[val_idx++]`, C 182 and its five siblings at
   192, 202, 214, 232, 238, 241, 249, 255) is the index then the step, as two statements - the
   `src/pgtkterm.rs` 602 recorded shape.
4. `props[props_idx]`, `vals[vp_idx]` and `initial_argv[i]` are raw-pointer derefs
   (`.offset (… as isize)`), because Rust has no `[]` on a raw pointer (C 178-256, 269-270).
5. C's array decay is spelled: `props.as_mut_ptr ()` for `SmcSetProperties`' `SmProp **` (C 260),
   `errorstring.as_mut_ptr ()` for `SmcOpenConnection`'s `char *` (C 468),
   `NOSPLASH_OPT.as_mut_ptr ()`/`…as_ptr ()` for C 240/241 (C's `char []` static).
6. `xfree` takes `*mut c_void`, so its five call sites cast (C 262-265, 269-270).
7. Values C leaves indeterminate get an initializer (PORTING.md 1.5): `i = 0` at C 169 (C writes it
   through `ckd_add` at C 226), `errorstring`, `prop_ptr`, `values`, `callbacks` zero-filled, and
   `class_hints` zero-filled at C 370 (the `E0381` the third probe run measured).
8. `ckd_add (&i, initial_argc, 3)` (C 226) is the `overflowing_add` pair with the write-back - the
   frozen `<stdckdint.h>` arm of the family (P-15, `docs/ports/token-aliases.tsv`'s `ckd_add` row).
9. C's `!x` on `int`-typed flags is `x == 0`/`"".is_null ()`: C 174-175 (`STRINGP`), 208/243
   (`if (cwd)`), 274-277 (`! shutdown`, `! SmcInteractRequest (…)`), 353 (`if (! opening)`), 470
   (`if (smc_conn != 0)`).
10. `cancel_shutdown` (C's `bool`) is passed `as Bool` to `SmcInteractDone` (C 516); `True` for the
    two `SmcSaveYourselfDone` calls (C 280, 517).
11. C 41-43's `#define gdk_x11_set_sm_client_id(w) gdk_set_sm_client_id (w)` is a `macro_rules!`
    under `use_gtk && !have_gtk3`, and C 477's one call site carries both spellings
    (`gdk_x11_set_sm_client_id! (client_id)` on the GTK 2 side, the plain call on GTK 3), because a
    Rust macro invocation needs the `!`.
12. `SMID_OPT`/`CHDIR_OPT` (C 72/82) are `&CStr` consts, so C's `strlen (MACRO)` is
    `.to_bytes ().len ()` and the `strcpy (stpcpy (dest, PREFIX), value)` nesting is kept (C 234-235,
    245-246) - §2.1's string-macro shape.
13. `NOSPLASH_OPT` (C 78) is the twelve-element array C's `char []` is, with the terminator.
14. `emacs_strerror (errno)` (C 408) is `emacs_strerror (*errno ())`: C's `errno` is the macro
    `(*__error ())` and `lib/include/errno.rs` keeps the accessor (`alloc.rs`'s same-looking call
    sits in a dead arm and never compiled).
15. `#define NR_PROPS 5` (C 161) and `#define SM_ERRORSTRING_LEN 512` (C 397) are `const`s at C's own
    positions, inside the two functions as C's `#define`s are; `SM_ERRORSTRING_LEN` is `usize` for
    the array bound and `as c_int` for the `error_length` argument.
16. C 30's `<sys/param.h>` is a comment: no mirror in this tree carries it and this file names
    nothing from it (the same gap `lib/getloadavg.rs` 65-68 records for its two macros).
17. The XSMP/ICE section (C 25's `<X11/SM/SMlib.h>` + the ICE names) is P-88 ①'s in-unit stand-in:
    **the header is not on this machine**, so its types, values and layouts are written from the
    protocol's published interface and are **UNVERIFIED** (the Rust side of the record shapes is the
    clang restatement of Evidence ②).
18. The in-unit declarations P-72 ③ allows for names the mirrors do not carry: four Xlib entry
    points + `XA_STRING` + `<X11/Xutil.h>`'s record/setter, `validate_x_resource_name`,
    `emacs_get_current_dir_name`, `emacs_strerror`, `Widget`/`WidgetRec`, and the two `gdk_*` names
    (`## Owed` 4 lists each one's home).
19. 0.1-19's seventh half (status 3.503) has **no site in this file**: the two `DEFVAR_LISP`
    docstrings name the program with the bare word (the landed `callproc.rs` 2337 / `editfns.rs` 5117
    / `buffer.rs` 5092 docstrings keep exactly that), the `DEFUN` docstring is 3.503 ②'s kept class
    (`extract-abi.py` compares it verbatim), and the only literals carrying a program identity are
    Lisp symbol names (`emacs-session-save`, `x-session-id`, `x-session-previous-id`), which 3.503
    leaves alone.  **If the integrator reads the rule more widely, this is the file's one place it
    would bite** (`## Owed` 9).
20. `pid`-free: nothing here needed a `libc` name beyond `<unistd.h>`/`<errno.h>`/`<stdio.h>`'s.
21. The five `/// C:` comments before `unsafe extern "C" { }` blocks produce rustc's
    `unused doc comment` warning (the mirrors emit the same four); they are kept because the
    citation gates read that form.
22. The two variables C declares through `DEFVAR_LISP` (`Vx_session_id`, `Vx_session_previous_id`)
    are defined in this unit as `#[unsafe(no_mangle)] pub static mut … = lisp_h_Qnil ()` at C 542's
    position: C's storage is `struct emacs_globals`' field and `include/globals.rs` only declares
    it, which `tools/check-globals.py` requires the owning unit to define (the `alloc.rs` 8006
    shape).
23. `when`-less: `smc_save_complete_CB`/`smc_shutdown_cancelled_CB`/`smc_error_handler`/
    `ice_error_handler` have **empty bodies** in the port because C's own bodies are the comment
    `/* Empty */` alone (C 302/308/323/335) - nothing is stubbed away.

## Owed by the integrator (nothing here is claimed as done)

1. `femacs/xtask/Cargo.toml`'s `[features]` table must register `have_x_sm` (the name this unit's
   whole text hangs on; absent today), so P-51's `dead-arms` and `tools/feature-coverage.py` can
   see it.
2. `docs/ports/line-drift-tolerance.tsv`'s `xsmfns` row (`0.99`, basis `total`, "C 582 vs the
   port's 0 code lines (an empty unit)") is now false: the port carries 1,013 lines against C 581
   (174%).  `extract-abi.py --check` passes with the row as it stands (74% < 99%), but the row's
   *reason* and the tolerance must be re-derived from the landed text - tolerance rows are the
   integrator's file.
3. `docs/baseline/port-lines-pending.tsv`'s `src/xsmfns.c` row (581/12, "known-unported target") and
   `docs/port-map.md`'s `| `src/xsmfns.c` | 581 | `femacs/src/xsmfns.rs` | 12 |` row belong to
   `tools/check-port-lines.py --write-pending` / `tools/gen-port-map.py`.
4. The mirror gaps declared at the use site (P-55/P-72 ③; moving each is P-88 ⑤'s same-commit move
   and this lane may not write mirrors):
   * `include/xlib.rs`: `XCreateSimpleWindow`, `XStoreName`, `XChangeProperty` (Xlib.h 1617/3405/
     2078), `XA_STRING` (Xatom.h 39), and `<X11/Xutil.h>`'s `XClassHint`/`XSetClassHint`
     (Xutil.h 189-192/583-587); also `True`/`False`'s home (today `include/xterm.rs` 65/68).
   * `include/xterm.rs`: `x_session_initialize`/`x_session_have_connection`/`x_session_close`
     (xterm.h 1998-2000; if they land in the mirror they need a `pub mod` + glob import - P-39 -
     or this unit's definitions are `E0428`), its `use_gtk`-typed `GtkWidget`/`Gdk*` fields
     (arm B's 12 `E0425`) and its `use_x_toolkit`-typed `Widget` fields (`Widget` has no
     declaration in any mirror or header here; this unit declares it, P-88 ①).
   * `include/lisp.rs`: `emacs_get_current_dir_name` (lisp.h 5284), `emacs_strerror` (lisp.h 5144).
   * `include/frame.rs`: `validate_x_resource_name` (frame.h 1873) - declared here ungated; the
     mirror's copy would have to square with `frame.rs`'s `have_window_system`-gated definition.
   * `include/gtk.rs`: `gdk_set_sm_client_id` (GTK 2) / `gdk_x11_set_sm_client_id` (GTK 3) - GTK is
     not on this machine, so both are text-only and UNVERIFIED (`docs/ports/include-gtk.md`).
5. The XSMP/ICE surface (item 17) has neither mirror nor header: a machine with libSM/libICE owes
   the measured layouts, and a **second** consumer would force it into a mirror (P-90 ①).
6. `docs/ports/macro-body-notes.tsv`: nothing was needed this round (see ④); if a later round needs
   a row there it is the integrator's file - this lane appended nothing.
7. The GTK 3 arm (Evidence ③) was not flipped; a machine with GTK owes it together with item 4's
   `include/xterm.rs` GTK fields.
8. An `xtask dead-arms` FLIPS entry (`("have_x_sm", "HAVE_X_SM", "true")`, `("use_x_toolkit",
   "USE_X_TOOLKIT", "true")`) is what would make the arm readings config-consistent; the hand flips
   here cannot satisfy `include/systime.rs` 27 and cannot produce an `.rlib`.
9. The status 3.503 reading of the two `DEFVAR_LISP` docstrings (Deviation 19) is this lane's, and
   it is the file's only 0.1-19 exposure - a ruling either way is one text edit.

## Not claimed

- **Zero run evidence.**  Nothing was linked, started or exercised: no session manager was contacted,
  no `SmcOpenConnection` ran, and the port has no `.rlib` (the arm flips stop at rustc's error).
  There is no `libSM`/`libICE` on this machine, so the XSMP half of this file cannot be exercised
  here at all.
- **The frozen object says nothing about the body.**  It is the 336-byte empty one (Evidence ①); the
  "0 errors in this unit" reading is a *name-and-type* reading of the mirror set under hand-set
  `--cfg`s, weaker than a build against the real X11/XSMP toolchains (P-86 ⑥/⑦).
- **No differential or behavioural evidence** exists for this unit, and no C reference object exists
  to differ against on this target (the file is dead in the frozen configuration).
- The mutation test (Evidence ③) proves the *reading*, not the port's fidelity: fidelity's own
  evidence is the range table (coverage), the transcription in `femacs/src/xsmfns.rs` and review.
- The 12 `E0425` rows of arm B and the `E0080` row of both arms are **mirror-side / flip-side** and
  are named here, not fixed (P-39/P-88: this lane does not write mirrors).
