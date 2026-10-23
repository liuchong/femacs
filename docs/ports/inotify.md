# emacs/src/inotify.c (590 lines)

> **This file is P-86's unit: the frozen configuration compiles no `inotify.o`.**  `configure.ac`
> 4095-4104 is what puts it in `NOTIFY_OBJ`, under `AC_DEFINE([HAVE_INOTIFY])` and only for a
> Linux-kernel target, so `docs/baseline/frozen-build-sources.txt` lists no `inotify.o` here and
> `extract-abi.py`'s checklist for the file is empty - its checkbox sections cannot be coverage
> evidence (P-86 ②).  What follows is P-86 ②'s substitute: the **C line-range table**, the
> `## Evidence` readings and the `## Deviations`.  Port: `src/inotify.rs`, the whole file
> (836 Rust lines at the first `--check`, 844 after the three `#ifndef` pairs were written out),
> both arms of every `#if` carried.  The old revision's conclusion ("not in the frozen build, so
> the port is the empty unit plus a note") was P-86 ①'s "not started" case; its measurements (the
> 336-byte object, the empty checklist) were true and are still what the frozen arm reads.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 5

## Functions defined — 11

## Macros defined — 3

## Types and their fields — 0

## File-scope objects — 2

(source: `docs/baseline/inventory.tsv` 108 — `src/inotify.c 590 11 5 3 3 0 2`)

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 4095-4104's `NOTIFY_OBJ`, under `AC_DEFINE([HAVE_INOTIFY])`) | `feature = "have_inotify"` (on every item and on the include graph) | a Linux configuration |
| C 35-37 / C 38-40 / C 41-43 (`#ifndef IN_EXCL_UNLINK` / `IN_DONT_FOLLOW` / `IN_ONLYDIR`) | `no_in_excl_unlink` / `no_in_dont_follow` / `no_in_onlydir`, one per name; both arms written at C's own `#ifndef` site | an old header that lacks the bit; the port's default is the modern header's definition |
| C 45-47 `#ifdef HAVE_ANDROID` + C 461-469 `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` | `feature = "have_android"` (registered; the `android.h` mirror supplies `android_is_special_directory`) | an Android configuration (which is also a Linux one, so it has `HAVE_INOTIFY` too) |
| C 443 `#ifdef HAVE_INOTIFY_INIT1` … C 451 `#endif` (inside `Finotify_add_watch`) | `feature = "have_inotify_init1"`; C's `#else` arm is `not (feature = "have_inotify_init1")` | a libc with `inotify_init1` (`AC_CHECK_FUNCS([inotify_init inotify_init1])`, glibc 2.9+) |
| C 531 `#ifdef INOTIFY_DEBUG` … C 545 `#endif` and C 583-586 (the two debug subrs) | `feature = "inotify_debug"` | an `INOTIFY_DEBUG` build |
| C 571-574 `#if 0` (the `Qignored` `DEFSYM`) | **no gate**: dead in every configuration; recorded, not transcribed (the `src/xfns.rs` 958 / `androidfns.md` Deviation 2 shape) | never |

`have_inotify` is the file's build-level gate, not a `config.h` macro of this target: the frozen
configuration leaves it false, which is why the frozen `build-tu` object is the 336-byte empty one
(P-86 ⑧).  `have_inotify_init1` is `config.h`'s `HAVE_INOTIFY_INIT1` lowercased (P-86 ①'s naming).

## Decisions

- **The file gate is `feature = "have_inotify"`, on every item *and on the include graph***: the
  frozen configuration builds no object from this file (`## Gate mapping`), so every item of the
  port is gated and a flip of the feature is what makes the unit non-empty.  The text is carried
  **in full**, because a Linux configuration compiles it (P-86 ①).
- **No Linux SDK is on this machine** (`/usr/include/sys/inotify.h` does not exist; the machine is
  `arm64-apple-darwin25.6.0`), so `<sys/inotify.h>` and `<sys/ioctl.h>` stand **in-unit** (P-88 ①:
  no mirror exists for either and this file is their only consumer) and their values are a
  **cross-target clang measurement** of the upstream header text (`## Evidence` 7), marked
  **UNVERIFIED** against a Linux toolchain (`## Owed` 3).  `<errno.h>`, `<fcntl.h>`, `<unistd.h>`,
  `<limits.h>` and `<string.h>` have mirrors and are reached through them (P-26: one spelling per
  name) - with the frozen target's values, which `## Deviations` 5 records.
- **The three `#ifndef` names are declared at C's own `#ifndef` site, C's arm written first.**
  C 31's `#include <sys/inotify.h>` section carries the header's *other* names; these three are
  different, because the file's own text is where they get their only definition (the
  `src/sound.rs` 499-503 reading of the same idiom: `SND_SENTRY` stands at C 93, not at C 85's
  `<mmsystem.h>` section).  Each pair is written with C's `# define X 0` first and the header's
  measured definition second, cfg-exclusive (`## Deviations` 3): a second declaration of one name
  in one unit is `E0428`, and `extract-abi.py --check`'s one-definition-per-name rule reads the
  first - which is C's own text, so the file's constant rule sees what C's text says while the
  live arm keeps the definition a Linux header really gives (`## Evidence` 8).
- **The live side is the system's semantics** (P-50's `signbit` decision: "the system's semantics
  is the reference binary's behavior"): this file's configuration *is* a Linux one, and the header
  text this round measured (Evidence 7) defines `IN_EXCL_UNLINK`, `IN_DONT_FOLLOW` and
  `IN_ONLYDIR` - C's `#ifndef` fallback exists for the headers that do not.  So the measured
  definitions are the port's default and C's `0` is the `no_in_*` arm, not the other way round
  (which would silently drop `dont-follow`/`onlydir`/`excl-unlink` from the mask C 436 builds);
  which glibc versions those are is not claimed here - the reading is one header revision.
- **`FIONREAD` is declared in-unit with the Linux value** (0x541B, `asm-generic/ioctls.h` 46),
  because `<sys/ioctl.h>` has no mirror (`src/keyboard.rs` 8908 declares the frozen target's
  0x4004667f for the same name; the two are different units, so nothing clashes - `## Findings` 4).
- **`IN_CLOEXEC` and `IN_NONBLOCK` are the header's own constants** (`bits/inotify.h`'s `enum`),
  *not* `<fcntl.h>`'s `O_CLOEXEC`/`O_NONBLOCK`: C 444 passes the former to `inotify_init1`, C 449-450
  passes the latter to `fcntl`, and the port keeps each from its own source.
- **Include-graph edges the chain reaches through other headers** are spelled (P-86 ⑧): the lisp.h
  chain (`include/globals.rs` for the `F*` prototypes), `include/coding.rs` (`ENCODE_FILE`,
  `DECODE_FILE`), `include/process.rs` (`add_read_fd`, `delete_read_fd`), `include/keyboard.rs`
  (`kbd_buffer_store_event`), `include/termhooks.rs` (`input_event`, `EVENT_INIT`),
  `include/systime.rs` (`Time`), `include/dispextern.rs` (`Emacs_Color`), `include/character.rs`
  (the helpers `buffer.rs` names), `lib/include/string.rs`, `lib/include/limits.in.rs` (`INT_MAX`).
- **Names no mirror carries are declared in-unit** (P-55's shape, with the C prototype quoted):
  `assoc_no_quit`/`equal_no_quit` (lisp.h 4274/4282; the `src/xfaces.rs` 4302 / `src/font.rs` 469
  reading), `strnlen` (`lib/include/string.in.rs` 240-246 keeps only the dead `rpl_strnlen` arm;
  `lib/strnlen.rs` defines it), `FILE_NOTIFY_EVENT` (termhooks.h 291, inside `#ifdef
  USE_FILE_NOTIFY`; the value is the `src/gfilenotify.rs` 169 / `src/kqueue.rs` 153 reading, 28,
  owed to the mirror round).

## C line ranges

Every row is **transcribed this round** and none is owed: the whole file (C 1-590) is carried, the
two arms of each condition are both present, and what remains owed is `## Owed`.  The rows are
contiguous, ascend without overlapping, and the last one ends at the reference's final line
(blank lines between blocks are absorbed into the following row, the P-86 ② convention);
`docs/port-map.md` 468 binds this ledger to `src/inotify.c` alone (the reference has no
`src/inotify.h`).

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of src/inotify.c @ fd7afced403.` line) |
| 19-20 | blank 19 + C 20 `#include <config.h>` | transcribed; ungated - `include!("include/config.rs")`, the only ungated item |
| 21-26 | blank 21 + C 22-26 `lisp.h`, `coding.h`, `process.h`, `keyboard.h`, `termhooks.h` | transcribed; file gate; the mirror edges of `## Decisions` |
| 27-29 | blank 27 + C 28-29 `<errno.h>`, `<fcntl.h>` | transcribed; file gate; `lib/include/errno.rs`, `lib/include/fcntl.in.rs` |
| 30-31 | blank 30 + C 31 `#include <sys/inotify.h>` | transcribed; file gate; the in-unit header section (P-88 ①) - constants, `struct inotify_event` and the four entry points, all cross-target measured and UNVERIFIED |
| 32-33 | C 32 `#include <sys/ioctl.h>` + blank 33 | transcribed; file gate; the in-unit `<sys/ioctl.h>` section (`ioctl`, `FIONREAD`) |
| 34-43 | C 34's `/* Ignore bits … */` comment + C 35-43's three `#ifndef` fallbacks | transcribed; file gate + one `no_in_*` feature per name, both arms written at C's site (see `## Decisions` and `## Deviations` 3) |
| 44-45 | blank 44 + C 45 `#ifdef HAVE_ANDROID` | transcribed; `have_android` |
| 46-47 | C 46 `#include "android.h"` + C 47 `#endif` | transcribed; `have_android` + file gate; `include/android.rs` is the mirror (C's own file) |
| 48-50 | blank 48 + C 49-50 `static int inotifyfd = -1;` | transcribed; file gate; C's own initialiser |
| 51-75 | blank 51 + C 52-73's comment on the watch list + C 75's tentative `static Lisp_Object watch_list;` | transcribed; file gate; the comment is C's, the object is zero-initialised (`Qnil`) |
| 76-114 | blank 76 + `mask_to_aspects` (C 77-114) | transcribed; file gate; sixteen `if`s in C's order |
| 115-165 | blank 115 + `symbol_to_inotifymask` (C 116-165) | transcribed; file gate; C's `if/else if` chain, `errno = EINVAL` as `*errno () = EINVAL` |
| 166-181 | blank 166 + `aspect_to_inotifymask` (C 167-181) | transcribed; file gate; `FOR_EACH_TAIL (x)` is the mirror's `FOR_EACH_TAIL!` plus the body's `x_end` recording (`## Deviations` 8) |
| 182-209 | blank 182 + `inotifyevent_to_event` (C 183-209) | transcribed; file gate; `CONS_TO_INTEGER` and `INT_TO_INTEGER` need the explicit casts of `## Deviations` 6/7 |
| 210-248 | blank 210 + C 211-213's comment + `add_watch` (C 214-248) | transcribed; file gate; C 233's `for` head becomes a `loop` with the increment after the body (PORTING.md 2.2) |
| 249-265 | blank 249 + C 250-256's comment + `find_descriptor` (C 257-265) | transcribed; file gate; C's `for` head becomes a `while` with the stepper at the tail |
| 266-303 | blank 266 + C 267-270's comment + `remove_descriptor` (C 271-303) | transcribed; file gate |
| 304-322 | blank 304 + C 305's comment + `remove_watch` (C 306-322) | transcribed; file gate |
| 323-368 | blank 323 + C 324-325's comment + `inotify_callback` (C 326-368) | transcribed; file gate + the two `SAFE_ALLOCA!`/`SAFE_FREE!` names C's macros declare (`## Deviations` 9) |
| 369-476 | blank 369 + `DEFUN ("inotify-add-watch", …)` (C 370-476) | transcribed; file gate (`DEFUN!`'s own attribute too, P-86 ⑧); C 443-451 the `have_inotify_init1` pair, C 461-469 the `have_android` pair |
| 477-488 | blank 477 + `valid_watch_descriptor` (C 478-488) | transcribed; file gate; C's `return (…)` parentheses are kept (`## Deviations` 11) |
| 489-509 | blank 489 + `DEFUN ("inotify-rm-watch", …)` (C 490-509) | transcribed; file gate |
| 510-529 | blank 510 + `DEFUN ("inotify-valid-p", …)` (C 511-529) | transcribed; file gate |
| 530-545 | blank 530 + C 531-545 `#ifdef INOTIFY_DEBUG`'s two subrs | transcribed; file gate + `inotify_debug` |
| 546-590 | blank 546 + `syms_of_inotify` (C 547-590) | transcribed; file gate; twelve `DEFSYM!`s, three `defsubr`s, the `inotify_debug` pair, `staticpro`, `Fprovide`; C 571-574's `#if 0` is recorded in place, not transcribed (`## Gate mapping`) |

## Findings

1. **`Qignored` is not this file's symbol.**  `mask_to_aspects` (C 105-106) and
   `symbol_to_inotifymask` (C 153-154) compare against `Qignored`, whose `DEFSYM` C 571-574 disables
   with `#if 0` because `coding.c` defines it "on all platforms".  The port's `Qignored ()` is the
   generated `include/globals.rs` accessor (`builtin_lisp_symbol (1045)`), i.e. the same symbol the
   reference's `coding.c` registers - the `#if 0` is a *duplicate-definition* guard, not a missing
   name.
2. **The file's own docstring describes a watcher thread it does not have** (C 516-519: "if the
   watcher thread exits abnormally").  Nothing in this file creates a thread: C 455 registers
   `inotify_callback` on the main loop's read-fd list.  The sentence is inherited boilerplate (the
   sibling `kqueue-valid-p` docstring carries the same words), and `extract-abi.py --check`
   compares docstrings byte-for-byte against `docs/baseline/subrs.tsv` (rows 962-966), so the port
   carries it verbatim.
3. **C 404's `\(` is a make-docfile escape, not two characters.**  `lib-src/make-docfile.c`'s
   reader keeps only the escaped character, so the *docstring value* is
   `(WATCH-DESCRIPTOR ASPECTS NAME COOKIE)`.  The port therefore spells the literal with a plain
   `(`: the first edition carried `\\(` and `extract-abi.py --check` reported
   `subr Finotify_add_watch: docstring differs` (digest `d98c507f2d21` vs the baseline's
   `7dc97369f392`); `docs/baseline/subrs.tsv` 962 is the authority (`## Evidence` 8).
4. **`FIONREAD` has two values in this tree, and they are both right for their own target.**
   `src/keyboard.rs` 8908 declares the frozen target's `0x4004667f`; this unit's `<sys/ioctl.h>`
   section declares Linux's `0x541B` (measured).  Nothing clashes (different units, different
   configurations), but a *joint* Linux build of both would disagree - the integrator's call
   (`## Owed` 6).
5. **C 425-430's docstring says three bit-masks "cannot be used", and C 436 then uses two of
   them.**  `IN_MASK_ADD` and `IN_EXCL_UNLINK` are OR'd into the mask C 436 hands to
   `inotify_add_watch`, while the docstring promises the caller that they cannot be.  The promise
   is about the *aspect* symbols (`symbol_to_inotifymask` maps no aspect to either name), so both
   statements hold; the port carries both as C wrote them.
6. **`Finotify_valid_p` performs no liveness check** (C 511-529): it is a membership test on
   `watch_list`, so a descriptor stays "valid" after the watched file is gone until an `IN_IGNORED`
   event is delivered and C 361-362 drops it.  The docstring's promises hold only as a side effect
   of event delivery (the `docs/ports/kqueue.md` 96-100 reading of the sibling subr).
7. **The reference's `inotify_watch_list`/`inotify_allocated_p` subrs exist only under
   `INOTIFY_DEBUG`** (C 531-545), which `configure.ac` never defines; they are carried under
   `inotify_debug` so the file's five `subrs.tsv` rows all have a counterpart (`## Gate mapping`).

## Evidence

1. **frozen arm**: `cd femacs && CARGO_TARGET_DIR=/tmp/inotify-lane cargo run -q -p xtask --
   build-tu src/inotify.c` -> **exit 0**, `built src/inotify.rs -> ../build/femacs/inotify.o`,
   `../build/femacs/inotify.o` **336 bytes** (the empty object the file gate leaves).  Run three
   times this round - the first (draft) invocation failed, the second read 336 bytes on the
   pre-`#ifndef`-rewrite text, and the third (the revision carried now, 844 lines) read 336 bytes
   again and materialised the root the direct-`rustc` readings below use.  P-86 ⑨'s "rerun the
   frozen tier between items" is what the second and third runs are.
   The **first** run failed (`exit 1`, `E0758 unterminated block comment`) on a nested `/* … */`
   inside a Rust block comment in the port's own text; that is a defect of the first draft, fixed
   before any acceptance reading, and it is why the ledger's C-range row for C 34 is written as two
   comments (C's own sentence, then the analysis) rather than one.
2. **arm flip through the tool** (a real compile): `CARGO_TARGET_DIR=/tmp/inotify-lane cargo run -q
   -p xtask -- probe src/inotify.rs /tmp/inotify-lane/a.rlib --crate-type=lib -C panic=abort --cfg
   'feature="have_inotify"'` -> **exit 0, 0 errors, 18 warnings, a 13,779,312-byte rlib**.  The
   other arms: the same command with `--cfg 'feature="have_inotify"' --cfg
   'feature="have_inotify_init1"' --cfg 'feature="inotify_debug"' --cfg
   'feature="no_in_excl_unlink"' --cfg 'feature="no_in_dont_follow"' --cfg
   'feature="no_in_onlydir"'` (P-86 ⑥: *every* sub-gate of this file's Linux configuration) ->
   **exit 0, 0 errors, 18 warnings, a 13,783,736-byte rlib**.  Of the 18 warnings **1 is in the
   unit's own text** (`_root.rs:726` on the first revision, `734` after the `#ifndef` pairs were
   written out: `unnecessary parentheses around block return value`, i.e. C 481-487's own
   `return (…)` - the house class the tree keeps 205 times); the other 17 are mirror copies
   (`lib/include/unistd.in.rs`'s `read`/`write` runtime-symbol notes and the house-class
   parentheses/doc-comment warnings).  Both readings were taken on the **first revision** of the
   text (836 lines, the one whose `--check` still reported the six constant/macro rows); the
   revision carried now (844 lines, md5 `f8fc619d3cbc5184350e4174a9ffb14f`) is the one Evidence 3
   compiles.
3. **the same two arms, and the Android one, compiled directly on the revision carried now**
   (`rustc --edition 2024 --crate-type=lib -O -C panic=abort <cfg…>
   build/femacs/units/inotify/_root.rs -o …`, run from `` so the repository's pin applies;
   the tool's `probe` subcommand is exactly this invocation plus `materialize`, and the arm budget
   below is reported honestly): all three -> **exit 0, 0 errors, 18 warnings (1 in the unit's own
   text)**; the rlibs are **13,779,816** (default arm), **13,784,216** (all sub-gates) and
   **15,927,656** bytes (Android arm).  `nm -u` on the three rlibs is what says the arms are
   *reached*:
   the default arm references `_inotify_init` **and `_fcntl`**, and no `_inotify_init1`; the
   all-sub-gates arm references `_inotify_init1` and **no** `_fcntl`; the Android arm (file gate +
   `have_android` + `have_inotify_init1`) references `_android_is_special_directory`.  That is C
   443-451's two arms and C 461-469's arm, selected by their gates.
4. **arm-reach preflight (P-86 ⑦: the reading is not vacuous)**: the materialised root with one
   injected type error in the unit's *own* text (`staticpro (&raw mut watch_list); let _probe:
   c_int = watch_list;`, `_root.rs:833`) and the all-sub-gates flags -> **1 error**, `E0308
   mismatched types` at `/tmp/inotify-lane/mut_root.rs:833:56`; the unmutated run of the same
   command is **0 errors**.  So the compile does read this unit's statements and "0 errors of its
   own" is a measurement, not an empty reading.
5. **toolchain**: from `` the readings run under `rust-toolchain.toml`'s
   `nightly-2026-10-01`; a bare `rustc` from the repository root picks stable 1.98.1 and reports
   `include/lisp.rs`'s `verror` `c_variadic` (`E0658`) - a pre-existing mirror property
   (`docs/ports/minibuf.md`), not this unit's.
6. **budget, reported as spent**: `build-tu` 3 of 3 (one of them the failed first draft), `probe` 3
   of 3 (one of them the first draft's type errors).  The three arm readings of Evidence 3 are
   direct `rustc` invocations of the *already materialised* root, not further `probe` calls, and
   the mutation of Evidence 4 is a fourth; no `cargo test`, no `--all`, no `gate-fast`, no
   `check-all-units.sh` was run, and `/tmp/inotify-lane` was removed at the end of the round.
7. **clang measurements** (Apple clang 21.0.0, `-target x86_64-linux-gnu`, `-nostdinc` with a
   minimal `stdint.h`/`features.h` shim; the headers are the upstream text, fetched read-only into
   `/tmp`, because no Linux SDK is on this machine - `## Decisions`; the fetched files were removed
   with that directory at the end of the round, so the reading is reproducible only by refetching
   them):
   *glibc `sysdeps/unix/sysv/linux/sys/inotify.h`* + *its `sysdeps/unix/sysv/linux/bits/inotify.h`*
   (both at `bminor/glibc` `master`, fetched 2026-10-05): `IN_ACCESS` **1**, `IN_MODIFY` **2**, `IN_ATTRIB` **4**, `IN_CLOSE_WRITE` **8**,
   `IN_CLOSE_NOWRITE` **16**, `IN_CLOSE` **24**, `IN_OPEN` **32**, `IN_MOVED_FROM` **64**,
   `IN_MOVED_TO` **128**, `IN_MOVE` **192**, `IN_CREATE` **256**, `IN_DELETE` **512**,
   `IN_DELETE_SELF` **1024**, `IN_MOVE_SELF` **2048**, `IN_UNMOUNT` **8192**, `IN_Q_OVERFLOW`
   **16384**, `IN_IGNORED` **32768**, `IN_ONLYDIR` **16777216**, `IN_DONT_FOLLOW` **33554432**,
   `IN_EXCL_UNLINK` **67108864**, `IN_MASK_ADD` **536870912**, `IN_ISDIR` **1073741824**,
   `IN_ONESHOT` **2147483648**, `IN_ALL_EVENTS` **4095**, `IN_CLOEXEC` **524288** (`02000000`),
   `IN_NONBLOCK` **2048** (`00004000`); `sizeof (struct inotify_event)` = **16** with wd@**0**,
   mask@**4**, cookie@**8**, len@**12**, `name`@**16** (the flexible member's offset is the
   record's end).  *Linux `include/uapi/asm-generic/ioctls.h`* (`torvalds/linux` `v6.12`, line 46):
   `FIONREAD` **21531**.
   The reading was taken with `_Static_assert (X == 0, …)` probes, so clang itself prints each
   value (`expression evaluates to '… == 0'`).
8. **`extract-abi.py --check emacs/src/inotify.c src/inotify.rs`** -> the 7,306-macro table
   notice, then **one** `- ` row: `line count drifted 43.1%: C 590 vs Rust 844` (the tolerance row
   is the integrator's, `## Owed` 1).  The first edition's other six rows are gone: the three
   `macro … does not mention 0` rows and the three `constant …: C 0 vs Rust 0x…` rows were the
   one-definition-per-name rule meeting the armed pairs, and writing C's own arm first
   (`## Decisions`, `## Deviations` 3) is what clears them - **no
   `docs/ports/macro-body-notes.tsv` row was needed, and none was added**.
9. `python3 tools/check-mirror-dupes.py` -> **24 duplicate declaration(s), 17 ratchet row(s), 7
   failure(s)**: the 17 ratchet rows are the file's own baseline (unchanged) and all 7 failures are
   `lib/file-has-acl.rs`'s (another lane's in-flight file - attributed, not touched).  This
   unit contributes **0 duplicates and 0 rows**: `ioctl`, `strnlen`, `assoc_no_quit`,
   `equal_no_quit`, `FIONREAD`, the `IN_*` names and the four `inotify_*` entry points are declared
   by no mirror in its include graph.
10. `python3 tools/check-declarations.py` -> **exit 1**, `18 declaration sites (1 forbidden, 17
    allowed literals), 1 failures`, and the failure is `lib/file-has-acl.rs: recital x1`
    (another lane's file).  **This unit and this ledger are not named in its output** (0 `inotify`
    lines), i.e. no product-name, licence, version or `recital` finding.
11. `python3 tools/check-citations.py` -> **exit 1**, `676 file(s) scanned, 2 drift, 0 range, 1103
    external, 34 allowed, 1 stale`; the two drift rows are `src/include/dispextern.rs:600`
    and `src/xsmfns.rs:428` (other lanes' files).  **0 `inotify` lines**: this unit's
    `/// C:` citations (lisp.h 4274/4282, string.h, termhooks.h 291) and this ledger carry no
    range/drift finding.
12. `python3 tools/check-dead-file-coverage.py` -> **exit 0**, `82 ledger(s) bound (a C range
    table), 0 failure(s)`, with this ledger's own row reading `info docs/ports/inotify.md: 26 range
    row(s) cover src/inotify.c (590 lines; 0 blank line(s) between blocks need no row)` - i.e. the
    table of `## C line ranges` covers C 1-590 with no gap, and the 26 rows are the ones above.
13. `python3 tools/check-port-lines.py` (read-only) -> **exit 0**, `41 file(s) under ten percent,
    62,097 reference line(s); 47 ratchet row(s), 0 new, 6 stale`, and this file is one of the six
    stale rows (`stale src/inotify.c: no longer under the threshold`) - i.e. the port's 844 lines
    are past the ten-percent threshold this round, and the ratchet row is the integrator's to
    retire (`## Owed` 2).


## Deviations

1. **`<sys/inotify.h>` and `<sys/ioctl.h>` are this unit's sections** (C 31-32): the reference
   tree has no such file (P-88 ①), so the declarations stand at C's `#include` lines, each
   carrying a `/// C:` quote of the header's own line, and no file was added under
   `src/include/`.  If a second consumer appears they move into mirrors of their own in the
   same commit (P-88 ⑤ / P-90 ②).
2. **C's `IN_*` masks are typed `uint32_t` in the port.**  C spells them as `int` literals (except
   `IN_ONESHOT`, `0x80000000`, whose C type is `unsigned int`), and every use here is a `uint32_t`
   mask expression or a `uint32_t` return value; typing them as their C type would force a cast at
   each of the 25 sites.  The values are unchanged (`## Evidence` 7); `IN_CLOEXEC`/`IN_NONBLOCK`
   keep C's `int`, because `inotify_init1` takes an `int`.
3. **The three `#ifndef` names are declared at C's `#ifndef` site with C's own arm first** (C
   35-43): the `*_in_*` members of each pair are cfg-exclusive, C's `# define X 0` carries the
   `no_in_*` feature and the header's measured definition is the default arm.  The placement
   follows `src/sound.rs` 499-503 (`SND_SENTRY` at C 93) and the order follows
   `extract-abi.py --check`'s first-definition rule together with Rust's `E0428` (`## Decisions`);
   the arms are both carried (P-86 ①), so nothing is dropped.
4. **`IN_CLOEXEC`/`IN_NONBLOCK` are written as Rust octals** (`0o2000000`, `0o4000`): C's `02000000`
   and `00004000` are the same values, and Rust has no bare-octal literal.
5. **The system-surface mirrors carry the frozen target's values, and this file's configuration is
   a Linux one.**  `<errno.h>`'s `errno` is `(*__error ())` (macOS; a Linux build's is
   `__errno_location ()`) and `<fcntl.h>`'s `F_SETFL`/`F_SETFD`/`O_NONBLOCK`/`O_CLOEXEC` are the
   frozen target's (`O_NONBLOCK` 0x4, `O_CLOEXEC` 0x1000000, where Linux has 0x800/0x80000).  The
   port includes the mirrors (P-26: one spelling per name) and records the difference here; C
   446-450's `#else` arm is the one that would pass the frozen values, and `## Owed` 5 names the
   mirror round.  The same class `src/android-asset.rs` records for `errno`.
6. **C's `int to_read;` is `let mut to_read: c_int = 0;`** (C 329): a C local the callee writes and
   Rust must initialise to take its address (PORTING.md 1.5, the `floatfns.c`
   `rescale_for_division` shape).  The zero is never read if `ioctl` fails (the call reports the
   error), and C's own `to_read` is equally uninitialised there.
7. **`CONS_TO_INTEGER` is the mirror's trait-macro plus an explicit cast**: C's `CONS_TO_INTEGER (X,
   uint32_t, mask)` (C 188) and `(X, int, wd)` (C 280) become a `let mut …: i64`/`CONS_TO_INTEGER!`
   pair and `as uint32_t`/`as c_int` (the `src/editfns.rs` 1170-1171 shape), because the port's
   trait has the two 64-bit impls only (`ldoc`).
8. **`FOR_EACH_TAIL!` cannot hand C's terminating non-cons back to the caller** (C 174-176): the
   macro re-binds the loop variable inside its own block, so the body records `XCDR (x)` in
   `x_end` and C 176's `CHECK_LIST_END (x, aspect)` becomes `CHECK_LIST_END (x_end, aspect)` (the
   `src/fns.rs` 243-251 shape, docs/ports/fns.md).
9. **`USE_SAFE_ALLOCA!`/`SAFE_ALLOCA!`/`SAFE_FREE!` take C's own names explicitly** (C 333-334,
   367): C's `USE_SAFE_ALLOCA;` declares `sa_avail`/`sa_count` and `SAFE_ALLOCA (size)`/`SAFE_FREE
   ()` are one- and zero-argument macros; `macro_rules!` hygiene cannot hand the `let`s out, so the
   call site names them (`include/lisp.rs` 2974-2985's documented deviation, the `src/bignum.rs`
   428-432 shape).
10. **`INT_TO_INTEGER` needs its argument's cast** (C 204/207, 218, 347): the port's helper takes
    `EMACS_INT`, so C's implicit conversions from `int`/`uint32_t` are written
    (`… as EMACS_INT`); `strnlen`'s `usize` return is cast back to `ptrdiff_t` for
    `make_unibyte_string` (C 198), and `name` is `*mut c_char` (what the mirror's `SSDATA`
    returns).
11. **C 481-487's `return (CONSP (wd) && … );` keeps its parentheses**, which Rust reports as
    `unnecessary parentheses around block return value` - the one warning in this unit's own text
    (`## Evidence` 2), and the tree's house style keeps C's parentheses (205 `return (` sites).
12. **C 571-574's `#if 0` is recorded, not transcribed** (the `src/xfns.rs` 958 /
    `src/androidfns.rs` precedent): a `DEFSYM` inside `#if 0` is dead in every configuration, and
    the symbol it would register is `coding.c`'s (`## Findings` 1).
13. **C 404's `\(` becomes a plain `(` in the docstring literal** (C's make-docfile escape,
    `## Findings` 3), and C's `else if (EQ (symb, Qt) || EQ (symb, Qall_events))` keeps its
    parentheses inside the condition (Rust needs them for precedence).
14. **`inotify-allocated-p`'s C `return inotifyfd < 0 ? Qnil : Qt;` is the `if … { Qnil () } else
    { Qt () }` expression** (`Qt`/`Qnil` are the mirror's accessor functions, not objects).
15. **`sizeof (*ev)` is `core::mem::size_of::<inotify_event> ()`** (C 364): the C operator never
    evaluates `ev`, so the port asks for the type's size, and C's `buffer[i]` pointer arithmetic is
    `buffer.add (i as usize).cast ()`.

## Owed by the integrator (nothing here is claimed as done)

1. **`docs/ports/line-drift-tolerance.tsv`: an `inotify` row.**  `extract-abi.py --check` reports
   `line count drifted 43.1%: C 590 vs Rust 844` (basis `total` - the port carries more text: the
   gate attributes and the two header sections).  Only the integrator writes that file
   (P-86 ⑩), so the pair's `--check` stays FAIL until the row lands.  The measured reading may be
   stale by a line or two if the port's text is edited later: re-derive it from the tree.
2. **`docs/status.md` §3.x** for this file's round (the §3.31 text this ledger replaces) and the
   `check-port-lines.py` ratchet: `--write-pending` must retire `src/inotify.c`'s row
   (`## Evidence` 13 reads `stale src/inotify.c: no longer under the threshold`, i.e. the file has
   left the "fewer than ten percent ported" class this round; that command writes
   `docs/baseline/port-lines-pending.tsv`, which is not in this lane's write set).
3. **`docs/baseline/feature-coverage.tsv` regeneration**: this unit introduces **six** feature
   names - `have_inotify` (the file gate), `have_inotify_init1`, `inotify_debug`,
   `no_in_excl_unlink`, `no_in_dont_follow`, `no_in_onlydir`.  Nothing enables any of them yet
   (P-51: text that has never been through rustc in a repository-wide build), and `config.rs` has
   no constant for them.
4. **`FILE_NOTIFY_EVENT`'s value per configuration** (`include/termhooks.rs`): the frozen mirror
   carries the frozen members; in a `USE_FILE_NOTIFY` build the member sits before `SLEEP_EVENT`
   and shifts the members after it, which is why this unit declares it (28, the
   `src/gfilenotify.rs` 169 reading).
5. **The frozen-target system mirrors vs a Linux configuration**: `lib/include/errno.rs`'s
   `__error ()` and `lib/include/fcntl.in.rs`'s `O_NONBLOCK`/`O_CLOEXEC` are the frozen target's
   (`## Deviations` 5).  This file's `#else` arm (C 446-450) is only *semantically* right in a
   Linux configuration, where those constants come from the Linux headers; the mirror round is the
   place to decide what a Linux `have_inotify` build should see.
6. **`FIONREAD`'s two declarations** (`src/keyboard.rs` 8908's 0x4004667f and this unit's 0x541B,
   `## Findings` 4): if a joint Linux build is ever attempted, one of them has to move into a
   mirror.  No gate sees the disagreement today.
7. **The in-unit header sections' mirror round** (P-88 ⑤/P-90 ②): if `<sys/inotify.h>` or
   `<sys/ioctl.h>` acquires a second consumer, this unit's sections move into mirrors of their own
   in the same commit.

## Not claimed

- **Zero run evidence**: no `inotify_init`/`inotify_init1` call, no watch registered, no event
  read, no callback fired, no `file-notify-error` raised - the 13.8 MB rlibs are compile artifacts
  and the frozen object is the 336-byte empty one.  No differential or mutation test of the port's
  *semantics* was run; the only mutation is Evidence 4's type-error injection, which proves the
  compile reads the unit's text, not that the text is right.
- **No Linux SDK was used or is present**: every `<sys/inotify.h>`/`<sys/ioctl.h>` value is a
  cross-target measurement of fetched upstream header text (Evidence 7) and is **UNVERIFIED**
  against a real Linux toolchain; a different glibc/kernel combination could differ (the ABI is
  frozen, but this round measured one revision).
- The `have_inotify_init1` pair, the three `no_in_*` pairs, the `inotify_debug` pair and the
  `have_android` pair were compiled through the direct `rustc` invocations of Evidence 3, **not**
  through `xtask probe` (whose budget was spent on the two readings of Evidence 2); the
  `have_android` arm in particular is the first reading of that arm for this file and says only
  that it *compiles* here (P-86 ⚠ still stands for other Android units).
- The `have_inotify` gate has never been enabled in a repository-wide build: the frozen
  configuration leaves the whole file out, so this unit's participation in that configuration is
  the 336-byte empty object.
- `extract-abi.py --check`'s remaining row is the line-drift one (`## Owed` 1); `check-citations`,
  `check-declarations` and `check-mirror-dupes` each exit non-zero this round for **other lanes'
  files** (Evidence 9-11), and none of the three names this unit or this ledger.
- The in-unit `/// C:` lines quote each declaration's header text; the two system headers'
  citations are outside the tree, so `check-citations.py` counts them as `external` and no gate
  requires a line number for them (a *local* SDK's numbers would drift with the toolchain).
