# emacs/src/kqueue.c (550 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 3

## Functions defined — 6

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 2

(source: `docs/baseline/inventory.tsv` 119 — `src/kqueue.c 550 6 3 0 0 0 2`)

## Decisions

- **The file gate is the build-level `feature = "have_kqueue"` (P-86 ⑧), on every item *and on
  the include graph***: the frozen configuration leaves `HAVE_KQUEUE` undefined
  (`docs/baseline/config/darwin-arm64.macros.tsv` 1404), so the reference builds no object from
  this file either and this target's `build-tu` object is the 336-byte empty one - while the text
  is carried **in full**, because a kqueue configuration compiles it.  The earlier "empty unit
  plus a deferred-declaration note" reading (status 3.31) is withdrawn as a *conclusion*; its two
  measurements (the 336-byte object, the empty checklist) still hold, and the coverage record is
  therefore the C range table below rather than ticked boxes (P-86 ②).
- **The kqueue API is on this machine** (macOS `<sys/event.h>`, `<sys/resource.h>`,
  `<fcntl.h>`), so the arm flip in `## Evidence` is a *real compile*, not a name-resolution
  reading: every constant, the two record layouts and `RLIMIT_NOFILE` are clang measurements
  (`## Evidence` ⑦), and the five `<sys/event.h>`/`<fcntl.h>` conditions C decides by value
  (`O_EVTONLY`, `O_SYMLINK`) or by `#ifdef` (`HAVE_SYS_RESOURCE_H`, `HAVE_GETRLIMIT`, and the
  `#ifdef EV_ENABLE` of C 448) are decided by that measurement.
- **The external-header surface stands in-unit** (P-88 ①): `<sys/types.h>`, `<sys/event.h>`,
  `<sys/time.h>`, `<fcntl.h>` and `<sys/resource.h>` have no mirror in this tree and this file is
  their only consumer, so the 22 declarations stand at C 22-25/30-32's position, each with a
  `/// C:` line quoting the header; the two records are `#[repr (C)]` with `size_of`/`offset_of`
  assertions (`## Evidence` ⑦).  If a second consumer appears they move into a mirror of their
  own in the same commit (P-88 ⑤ / P-90 ②).
- **Both arms of the frozen macros are carried, not dropped** (P-51, P-86 ①): `HAVE_SYS_RESOURCE_H`
  and `HAVE_GETRLIMIT` are defined/1 here (macros.tsv 1627/1371), so the live side is the port's
  default and the `#else` side (`maxfd = 256;`, no `struct rlimit`) is carried under
  `no_getrlimit`; C 423-432's `#if O_EVTONLY` / `#if O_SYMLINK` and C 447-452's `EV_SET` with its C 448-450 `#ifdef EV_ENABLE`
  follow the same shape with `no_o_evtonly` / `no_o_symlink` / `no_ev_enable`.  All five were
  compiled this round (`## Evidence` ④).
- **The include graph is gated item by item** (P-86 ⑧), and the edges C 22-28's chain reaches
  through other headers are spelled here: `include/globals.rs` (C 26's chain - the *generated*
  `globals.h`, see `## Findings` 1), `include/lisp.rs` (C 26), `include/keyboard.rs` (C 27, whose
  own 32 reaches `include/buffer.rs`), `include/process.rs` (C 28: `add_read_fd`,
  `delete_read_fd`, `FNread_fd`), `include/systime.rs` (`struct timespec` of C 283/287),
  `include/dispextern.rs` (`Emacs_Color`, which `termhooks.h` names) and `include/character.rs`
  (the five character.h helpers `buffer.rs` names).  The last two are what the first arm reading
  was missing (`## Findings` 2).

## C line ranges

Every row is **transcribed this round** and none is owed: the whole file (C 1-550) is carried,
the two arms of each frozen macro are both present, and what remains owed is listed in
`## Owed`.  The rows are contiguous, ascend without overlapping, and the last one ends at the
reference's final line; `docs/port-map.md` 479 binds this ledger to `src/kqueue.c` alone.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of src/kqueue.c @ fd7afced403.` line) |
| 19-20 | blank 19 + C 20 `#include <config.h>` | transcribed; ungated - `include!("include/config.rs")`, the only ungated item |
| 21-25 | blank 21 + C 22-24 `<sys/types.h>`, `<sys/event.h>`, `<sys/time.h>` + C 25 `<fcntl.h>` | transcribed; file gate; the in-unit external-header section (P-88 ①) |
| 26-29 | C 26 `lisp.h`, C 27 `keyboard.h`, C 28 `process.h` + blank 29 | transcribed; file gate; five mirror edges + `include/globals.rs` (C 26's generated-header chain) |
| 30-34 | C 30-32 `#ifdef HAVE_SYS_RESOURCE_H` / `<sys/resource.h>` / `#endif` + blanks 33-34 | transcribed; file gate; live arm is the default (macros.tsv 1627), the `#else` side is `no_getrlimit` |
| 35-40 | C 36 `kqueuefd = -1`, C 39 the tentative `watch_list` + blank 40 | transcribed; file gate; C's own initialisers (`-1`, and zero = `Qnil`) |
| 41-69 | `kqueue_directory_listing` (C 41-68) + blank 69 | transcribed; file gate; C 48's `for` head becomes a `while` with the increment before `continue` |
| 70-106 | `kqueue_generate_event` (C 70-105) + blank 106 | transcribed; file gate; C 81-90's `do { … } while (1)` becomes `loop`, the `Fdelq` advance-before-delete order is C's |
| 107-275 | `kqueue_compare_dir_list` (C 107-274) + blank 275 | transcribed; file gate; C 147/176/187/201's `goto the_end` and C 216's label become one labelled block (`'the_end:`), the two following tails stay in C order |
| 276-337 | `kqueue_callback` (C 276-336) + blank 337 | transcribed; file gate; C 283's function-local `static const struct timespec nullts` is a plain local (PORTING.md 1.5) |
| 338-474 | `DEFUN ("kqueue-add-watch", …)` (C 338-473) + blank 474 | transcribed; file gate (+ the `DEFUN!`'s own gate attribute); C 392-397 the `getrlimit` arm, C 423-432 the two `O_*` arms, C 447-452 `EV_SET` |
| 475-504 | `DEFUN ("kqueue-rm-watch", …)` (C 475-503) + blank 504 | transcribed; file gate (+ the same attribute); see `## Findings` 3 |
| 505-519 | `DEFUN ("kqueue-valid-p", …)` (C 505-517) + blanks 518-519 | transcribed; file gate (+ the same attribute); the docstring is carried byte-for-byte, see `## Findings` 2 |
| 520-525 | `globals_of_kqueue` (C 520-523) + blanks 524-525 | transcribed; file gate; `#[unsafe(no_mangle)] pub unsafe extern "C"` keeps C's symbol |
| 526-547 | `syms_of_kqueue` (C 526-546) + blank 547 | transcribed; file gate; three `defsubr`s, eight `DEFSYM!`s, `staticpro`, `Fprovide` |
| 548-550 | the closing `PROBLEMS` note (C 548-550) | transcribed as the port's trailing comment (C's text carries no code) |

## Findings

1. **A generated header supplies names this file uses but does not include.**  `Qfile_notify_error`
   (C 403 and C 484) is the `DEFSYM` of `src/fileio.c` 6780 and `report_file_notify_error` is
   `src/lisp.h` 4983; the symbol's *name* is visible to C only through the build-generated
   `globals.h` - `emacs/src/globals.h` does not exist in the tree (the built copy is
   `build/emacs-ref/src/globals.h`), so `src/kqueue.c` cannot be preprocessed from the shipped
   tree alone.  The port's edge is `include/globals.rs` (P-86 ⑧); `report_file_notify_error`
   itself comes from `include/lisp.rs`.
2. **The reference's `kqueue-valid-p` docstring describes a watcher thread this file does not
   have** (C 510-513: "A watch can become invalid if the file or directory it watches is
   deleted, or if the watcher thread exits abnormally for any other reason").  No thread is
   created anywhere in the file: the callback runs on the main loop's read-fd handler that C 415
   registers with `add_read_fd`.  The sentence is inherited boilerplate from the sibling
   inotify/filenotify backends, and a port cannot fix it in place, because
   `extract-abi.py --check` compares the docstring byte-for-byte against `docs/baseline/subrs.tsv`
   (row 1041) - the port carries it verbatim.
3. **`kqueue-valid-p` (C 516) is a pure membership test on `watch_list`.**  It performs no
   liveness check at all, unlike the GLib backend's `gfile-valid-p` (which asks the monitor for
   `g_file_monitor_is_cancelled`): a descriptor stays "valid" after the watched file is gone
   until an event is delivered and C 333's `Fkqueue_rm_watch` removes it.  So finding 2's
   promises hold only as a side effect of event delivery, not by this function.
4. **C 400-401's guard is an `int` subtraction compared against a `ptrdiff_t`**
   (`maxfd - 50 < watch_list_len`): if `getrlimit` ever reports a soft `rlim_cur` below 50 the
   difference is negative and the *first* `kqueue-add-watch` signals `file-notify-error` ("no file
   descriptor left") with `watch_list_len` = 0, before any watch exists.  It cannot arise on this
   machine's defaults (the soft limit is not measured here; only the `RLIMIT_NOFILE` id, see
   `## Evidence` 7), and the port must keep the signed arithmetic exactly - the explicit cast is where
   the first arm reading stopped (`## Deviations` 2), so it is recorded rather than "tidied".

## Evidence

1. **frozen arm**: `cd femacs && CARGO_TARGET_DIR=/tmp/kq-lane cargo run -q -p xtask -- build-tu
   src/kqueue.c` -> exit 0, `built src/kqueue.rs -> ../build/femacs/kqueue.o`,
   `../build/femacs/kqueue.o` **336 bytes** (the empty object the file gate leaves).  Run twice
   (before and after the fixes below) with the same reading - P-86 ⑨.
2. **arm flip** (a real compile, the kqueue API is on this machine; the exact command):
   `CARGO_TARGET_DIR=/tmp/kq-lane cargo run -q -p xtask -- probe src/kqueue.rs
   /tmp/kq-lane/kq.rlib --crate-type=lib -C panic=abort --cfg 'feature="have_kqueue"'`
   -> **exit 0, 0 errors, 14 warnings, a 13,154,752-byte rlib**.  All 14 warning spans are in
   mirror copies (composite.rs 11, lisp.rs 1, coding.rs 1, lib/include/stdbit.in.rs 1) and
   **0 are in the unit's own text**.
3. **the first arm reading (before the two fixes) was 12 errors**, and that is what named the
   missing graph: **2 in the unit's own text** (`_root.rs` 564:5 `expected one of … found {` -
   the cast/comparison precedence; `_root.rs` 549:17 `E0614 type rlimit cannot be dereferenced`)
   and **10 in mirror copies** (`include/termhooks.rs` 242/245 `Emacs_Color`;
   `include/buffer.rs` 1498/1512 `STRING_CHAR`, 1524/1593 `UNIBYTE_TO_CHAR`, 1921/1947
   `string_char_and_length`, 1965 `BYTES_BY_CHAR_HEAD`, 1985 `raw_prev_char_len`).  The two edges
   spelled in `## Decisions` (`include/dispextern.rs`, `include/character.rs`) are what the
   mirror half needed, and both of the unit's own errors are fixed in the text carried now.
4. **all five arms compiled** (P-86 ⑥): `rustc --edition 2024 --crate-type=lib -O --cfg
   'feature="have_kqueue"' --cfg 'feature="no_getrlimit"' --cfg 'feature="no_o_evtonly"'
   --cfg 'feature="no_o_symlink"' --cfg 'feature="no_ev_enable"' build/probes/kqueue/_root.rs`
   (run from `femacs/`, so the repository's pinned toolchain applies) -> **exit 0, 0 errors,
   14 warnings, a 13,153,304-byte rlib**.
5. **arm-reach preflight (P-86 ⑦, the reading is not vacuous)**: the same materialisation, same
   flags, with one injected type error in the unit's *own* text
   (`list_length (watch_list) + 1.0`) -> 1 error, `E0277 cannot add {float} to isize` at
   `_root.rs:571:56`; the unmutated run of the same command is 0 errors.  So the compile does read
   this unit's text and "0 errors of its own" is a measurement, not an empty reading.
6. **toolchain**: from `femacs/` the probe runs under the repository's pin
   (`femacs/rust-toolchain.toml`, `nightly-2026-10-01` = rustc 1.101.0-nightly (21b707e3f
   2026-09-30)).  A bare `rustc` from the repository root picks stable 1.98.1 and reports
   `include/lisp.rs` 3756's `verror` `c_variadic` (`E0658`) - a pre-existing mirror property, not
   this unit's (the `docs/ports/minibuf.md` reading; PORTING.md P-04).
7. **clang measurements** (Apple clang 21.0.0, `arm64-apple-darwin25.6.0`, `<sys/event.h>` /
   `<sys/resource.h>` / `<fcntl.h>` of this SDK): `NOTE_DELETE/WRITE/EXTEND/ATTRIB/LINK/RENAME/
   REVOKE` = **1/2/4/8/16/32/64**; `EVFILT_VNODE` = **-4**, `EV_ADD`/`EV_ENABLE`/`EV_CLEAR` =
   **1/4/32**; `O_NONBLOCK`/`O_RDONLY`/`O_EVTONLY`/`O_SYMLINK`/`O_NOFOLLOW` = **4/0/32768/
   2097152/256**; `RLIMIT_NOFILE` = **8**; `sizeof (struct kevent)` = **32** with ident@**0**,
   filter@**8**, flags@**10**, fflags@**12**, data@**16**, udata@**24**; `sizeof (struct
   rlimit)` = **16** with rlim_cur@**0**, rlim_max@**8** and `sizeof (rlim_t)` = **8**;
   `sizeof (struct timespec)` = **16** with tv_sec@**0**, tv_nsec@**8**.
8. **`extract-abi.py --check emacs/src/kqueue.c femacs/src/kqueue.rs`** -> `7306 macros … applied
   to the 424 files`, then **one** `- ` row left: `line count drifted 30.5%: C 550 vs Rust 718`
   (the tolerance row is the integrator's, `## Owed` 1).  The three token rows this round reported
   (`kqueue_generate_event does not mention 1`, `kqueue_compare_dir_list does not mention 1, goto`,
   `Fkqueue_add_watch does not mention ev_set, u_short`) are recorded in
   `docs/ports/macro-body-notes.tsv` (three rows appended; the file now has 1378 lines).
9. `python3 tools/check-declarations.py` -> `10 declaration sites (0 forbidden, 10 allowed
   literals), 0 failures`; `python3 tools/check-citations.py` -> `676 file(s) scanned, 0 drift, 0
   range, 731 external, 35 allowed, 0 stale`; both exit 0.  Neither reports a finding in
   `femacs/src/kqueue.rs` or `docs/ports/kqueue.md`.

## Deviations

1. **The three `DEFUN!` items carry the file gate as well.**  C's gate is a *build-system* fact
   (the file is in no source list), and the port's transcription of it is a `#[cfg]` on every
   item; but `DEFUN!` itself lives in the gated `include/lisp.rs`, so an ungated invocation cannot
   even resolve in the frozen arm - the first `build-tu` of this round failed with `cannot find
   macro DEFUN in this scope` at the three C 338/C 475/C 505 items, and the gate attribute is what
   makes the frozen object the 336-byte empty one (P-86 ⑧).
2. **C 562's comparison is written with explicit parentheses**: C's `maxfd - 50 < watch_list_len`
   becomes `((maxfd - 50) as isize) < watch_list_len`, because in Rust `<` after a cast parses as
   the start of type parameters (the C text has no such ambiguity); the first arm reading stopped
   at exactly this line.
3. **C 394's `maxfd = rlim.rlim_cur;` is a value read**: the port's `rlim` is a `rlimit` value, so
   the field read is `rlim.rlim_cur` (C's `->`/`.` distinction has no Rust counterpart here).
4. **The five feature names are the port's own** (P-51), not C macros: `no_getrlimit`,
   `no_o_evtonly`, `no_o_symlink`, `no_ev_enable` name the `#else` sides of C 376-378 (`struct
   rlimit rlim`), C 392-397, C 423-427, C 428-432 and C 448-450; the live side is the port's
   default because this configuration takes it.
5. **C 95's `event.kind = FILE_NOTIFY_EVENT;` is a bitfield write**: `include/termhooks.rs` packs
   `kind : 16` into `input_event.bits`, so the port writes it with the per-unit `input_event_bits!`
   macro (the `src/gtkutil.rs` 1757-1770 shape, body-identical).
6. **`FILE_NOTIFY_EVENT`'s value (28) is declared in-unit** (C 95): `keyboard.h` declares it inside
   `#ifdef USE_FILE_NOTIFY`, and the frozen `include/termhooks.rs` carries the frozen
   configuration's members, so this file's gate implies it and the value is the `src/gfilenotify.rs`
   reading - owed to the mirror round (`## Owed` 4).
7. **C 283's `static const struct timespec nullts = { 0, 0 };` is a function-local static**, and
   the port spells it as a plain `let nullts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };`
   (PORTING.md 1.5's rule for a constant the callee reads); C's static storage is not observable
   here.
8. **C 80-90's `do { … } while (1)` and C 136's `while (1)` become `loop`**, and C's four
   `goto the_end` plus C 216's label become one labelled block (`'the_end:`) - PORTING.md 2.2's
   shape.  Together with the gate attributes and the moved declarations this is where the 30.5%
   line drift comes from, not from dropped text.
9. **`FUNCTIONP` (C 388) is spelled here as `lisp.h` spells it** (`! NILP (Ffunctionp (a))`), the
   `src/gfilenotify.rs` reading; the mirror does not carry it.

## Owed by the integrator (nothing here is claimed as done)

1. **`docs/ports/line-drift-tolerance.tsv`: a `kqueue` row** for the `30.5%` `--check` reading
   (`C 550 vs Rust 718`, basis `total` - the port carries more text, not less).  Only the
   integrator writes that file (P-86 ⑩), so `extract-abi --check` for this pair stays FAIL until
   the row lands.  **DONE (status 3.489, integrator):** the `total`/`0.32` row is appended
   (C 550 total vs the port's 718 lines, 30.5% drift, the measured value + 0.01), and
   `extract-abi --check emacs/src/kqueue.c femacs/src/kqueue.rs` is now **OK with zero `-` rows**.
2. **`docs/status.md` §3.x** for this file's round (the §3.31 text this ledger replaces) and the
   `docs/ports/pending.tsv` accounting.  **DONE (status 3.489, integrator):** §3.489 written; the
   pending ratchet needs no row for this file (718 target lines are far past the ten-percent
   threshold), and `check-port-lines.py` reports `58 ratchet row(s), 0 new, 0 stale`.
3. **`docs/baseline/feature-coverage.tsv` regeneration**: the table's `have_kqueue` row says 47
   sites in 1 file; the tree now carries **50** (`no_getrlimit` 3, `no_o_evtonly` 2,
   `no_o_symlink` 2, `no_ev_enable` 1 are unchanged and still correct).  **DONE (status 3.489,
   integrator):** regenerated - the `have_kqueue` row now reads **49 sites in 1 file** (the
   generator's own count; the four `no_*` rows match the tree).
4. **`FILE_NOTIFY_EVENT`'s value per configuration** (`include/termhooks.rs`): the frozen mirror
   carries the frozen members; in a `USE_FILE_NOTIFY` build the member sits before `SLEEP_EVENT`
   and shifts the members after it (`## Deviations` 6).
5. **The in-unit external-header section's mirror round** (P-90 ②/P-88 ⑤): if
   `<sys/event.h>`/`<sys/resource.h>`/`<fcntl.h>` acquire a second consumer, this section moves
   into a mirror of its own in the same commit.

## Not claimed

- **Zero run evidence**: no `kqueue ()` call, no `kevent ()` call, no watch registered, no file
  event delivered, no callback fired, no `O_EVTONLY` file opened - the 13 MB rlib is a compile
  artifact and the frozen object is the empty one.
- The arm was **type-checked and codegen'd into an rlib**, not linked into a program; nothing here
  exercises the directory-listing comparison (C 107-274) or the event delivery (C 276-336)
  against the reference's behavior - no differential, no mutation test of the port's semantics.
- The `#else` arms were compiled through the direct `rustc` invocation in `## Evidence` 4, **not**
  through `xtask probe`; only `have_kqueue` was flipped through the tool.
- The `O_EVTONLY`/`O_SYMLINK`/`NOTE_*`/`EV_*` values are **this machine's SDK**; a different SDK may
  differ, and the in-unit declarations are this round's transcription of that measurement.
- The `have_kqueue` gate has never been enabled in a repository-wide build: the frozen
  configuration leaves the whole file out, so this unit's participation in that configuration is
  the 336-byte empty object.
- The in-unit `/// C:` lines quote each declaration's header text **without a line number**
  (the `src/gfilenotify.rs` GIO section shape), so `check-citations.py`'s external-citation
  list does not carry them (the `CITE` pattern needs `<path> <line>`); no gate requires a
  number for a header outside the tree, and a *local* SDK's line numbers would drift with the
  toolchain.
- The three `docs/ports/macro-body-notes.tsv` rows are **records of why a token cannot appear**,
  not evidence that the surrounding body is otherwise identical (that is the flip's job, and the
  token gate keeps reporting every *other* missing token).
