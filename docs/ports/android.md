# emacs/src/android.c (7,594 lines) — first slice: C 1-1037

> **P-86's largest unit, ported in slices.**  The frozen configuration compiles nothing here:
> `configure.ac` 2597-2598 puts `android.o` in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])`
> (2602), and `docs/baseline/frozen-build-sources.txt` lists no `android.o` for this target, so
> `extract-abi.py`'s checklist for the file is empty and its checkbox sections cannot be coverage
> evidence.  What follows is P-86 ②'s substitute: the **C line-range table** for this slice, the
> `## Evidence` readings and the `## Deviations`.  Port: `femacs/src/android.rs` (1,311 lines),
> both arms of every `#if` carried.
>
> **This file's ledger replaces the previous revision's conclusion** ("the file is not in the
> frozen build, so the port is the empty unit plus a note"), exactly as
> `docs/ports/androidselect.md`/`androidfont.md` did: P-86 ① rules that a three-line header
> comment is *not started*, not *done*.  The old note's measurements are still true and are still
> where the *file gate* comes from; what changed is the conclusion drawn from them.  C 1,311 lines
> of a 7,594-line unit are carried here; **the next live definition is C 1041's
> `android_run_debug_thread`**, and the table's last row is "the remainder".

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`configure.ac` 2597-2598's `ANDROID_OBJ` - both Android branches, unlike `androidselect.o`, which 2648 adds only in the `REALLY_ANDROID` one) | `feature = "have_android"` (on every item and on C 22-55's include graph) | an Android configuration |
| C 61 `#ifndef ANDROID_STUBIFY` … C 7518 (real JNI arm) | `all (feature = "have_android", feature = "not_android_stubify")` | Android, API calls real |
| C 7519 `#else` … C 7594 `#endif` (the `ANDROID_STUBIFY` arm) | `all (feature = "have_android", not (feature = "not_android_stubify"))` | Android, API calls stubbed - **in the remainder, not this slice** |
| C 70 `#ifdef __aarch64__` … C 72 (the `<arm_neon.h>` include) | `feature = "aarch64"` (text-only here; the four NEON regions are C 4064-4431) | an aarch64 Android configuration |
| C 176 `#ifdef THREADS_ENABLED` … C 181 (`android_jvm`) | `feature = "threads_enabled"` | threads on |
| C 241 `#ifdef __i386__` … C 246 (`unused_pointer`) | `feature = "i386"` | an x86 Android configuration |
| C 297 `#if __ANDROID_API__ < 21` … C 300 / C 304 (`select_pipe` vs `cancel_signal_received`) | `feature = "android_api_lt_21"` (the androidselect.rs `android_api_le_11`/`le_30` shape) | an Android configuration whose NDK min SDK is < 21 |
| C 310 `#if __ANDROID_API__ >= 21` … C 312 (`SELECT_SIGNAL`) | `not (feature = "android_api_lt_21")` | min SDK >= 21 |
| C 325 `#ifndef THREADS_ENABLED` … C 329 (`event_queue.thread`) | `not (feature = "threads_enabled")` | threads off |
| C 367/C 426/C 509/C 522/C 604/C 923/C 973 (the other API-level pairs) | the same `android_api_lt_21` | as above |
| C 401/C 614/C 685/C 752/C 930/C 1030 (the other threads pairs) | the same `threads_enabled` | as above |

`not_android_stubify` is the feature the tree already registered for C's
`HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` line 640, first
used by `alloc.rs` 6428); `android_api_lt_21` is the one **derived** feature this slice adds and it
is not in `femacs/xtask/Cargo.toml` yet (owed, below).

`HAVE_WINDOW_SYSTEM` (configure.ac 7518) has no `#if` in this slice and therefore no gate.
`frame.rs`'s flag block 3.408 laid down is gated on `have_window_system`, and this unit carries
`have_android` **without** it, so that block does not apply here: nothing in C 1-1037 names a
`FRAME_*` flag, `struct frame` or `struct window` (android.c's include list at C 22-55 has
`lisp.h`, `blockinput.h`, `coding.h`, `systime.h`, `android.h`, `androidgui.h` - no `frame.h`,
no `window.h`).

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 19-20 | the blank and `#include <config.h>` | ungated (every unit includes it) |
| 21-55 | the file's own include list (`allocator.h`, `assert.h` … `unistd.h`, `sys/*.h`, `minmax.h`, `android.h`, `androidgui.h`, `lisp.h`, `blockinput.h`, `coding.h`, `epaths.h`, `systime.h`) | file gate |
| 56-59 | `android_init_gui` and its comment | file gate (C's `#ifndef` opens at 61, below) |
| 60-72 | `#ifndef ANDROID_STUBIFY`, `<android/bitmap.h>`, `<android/log.h>`, `<linux/unistd.h>`, `<sys/syscall.h>`, the `__aarch64__`/`<arm_neon.h>` pair | real arm / `aarch64` |
| 73-140 | the seven class structs (`android_emacs_pixmap` … `android_emacs_handle`, C 74-139) | real arm |
| 141-254 | the file-scope objects (`android_api_level`, the `*_dir`/`*_path` names, the densities, `android_java_env`, the `emacs_gc_*` fields, `emacs_service`, `service_class`, the seven class records, `emacs_installation_time`, `event_serial`, `unused_pointer`, `signal_mask_changed_p`, `startup_signal_mask`), C 176-181 `THREADS_ENABLED`, C 241-246 `__i386__` | real arm / `threads_enabled` / `i386` |
| 255-271 | the event-handling note, `android_event_container` (C 264-271) | real arm |
| 272-308 | `android_thread_event_queue` (C 275-308; C 297-304 the API-level pair: `select_pipe` / `cancel_signal_received`) | real arm / `android_api_lt_21` |
| 309-312 | `#if __ANDROID_API__ >= 21` / `#define SELECT_SIGNAL SIGUSR1` | real arm / `not android_api_lt_21` |
| 313-333 | `android_event_queue` (C 314-333; C 325-329 the threads pair: the `thread` member) | real arm / `threads_enabled` |
| 334-339 | `event_queue`, the `android_run_select_thread` forward declaration | real arm (the forward declaration has no Rust item) |
| 340-399 | `android_init_thread_events` (C 346-399; C 367-384 the `< 21` pipe pair) | real arm / `android_api_lt_21` |
| 400-436 | `#ifdef THREADS_ENABLED`, `android_finalize_thread_events` (C 408-431), the `poll_thread` keys (C 434), C 426-429 the `< 21` pair | real arm / `threads_enabled` / `android_api_lt_21` |
| 437-459 | `android_get_poll_thread` (C 442-459; C 445-458 the threads pair) | real arm / `threads_enabled` |
| 460-499 | `android_set_task_name` (C 467-499; C 491-492 the `goto failure`, C 497-499 the `failure:` tail) | real arm |
| 500-670 | `android_run_select_thread` (C 501-670; C 509-516 the locals pair, C 522-603 the `< 21` arm, C 604-667 the `>= 21` arm) | real arm / `android_api_lt_21` / `threads_enabled` |
| 671-693 | `#if __ANDROID_API__ >= 21`, `android_handle_poll_signal` (C 674-691; C 685-689 the threads pair) | real arm / `not android_api_lt_21` / `threads_enabled` |
| 694-700 | `android_query_sem` (C 697), `main_thread_id` (C 700) | real arm |
| 701-764 | `android_init_events` (C 710-764; C 713-715 and 738-751 the API pair, C 752-763 the threads pair) | real arm / `android_api_lt_21` / `threads_enabled` |
| 765-776 | `android_pending` (C 766-776) | real arm |
| 777-801 | `android_wait_event` (C 782-801) | real arm |
| 802-832 | `android_next_event` (C 803-832) | real arm |
| 833-866 | `android_check_if_event` (C 834-866) | real arm |
| 867-909 | `android_write_event` (C 868-909) | real arm |
| 910-916 | the `android_urgent_query` note and object | real arm |
| 917-1037 | `android_select` (C 918-1037; C 923-925 the `static char byte` pair, C 930-932 the threads pair, C 973-982 the API pair, C 1030-1034 the threads pair) | real arm / `android_api_lt_21` / `threads_enabled` |
| 1038-7594 | **the remainder** - the blank, C 1039's form feed, and from C 1041 (`android_run_debug_thread`, **the next live definition**) through C 7594's `#endif`: `android_user_full_name` (C 1081), `android_is_special_directory` (C 1100), the JNI `NATIVE_NAME` entry points, the NEON regions (C 4064-4431) and C 7519's `#else` stub arm | not this slice |

The thirty rows tile C 1-1037 **contiguously** (blank lines included) rather than skipping the blanks
between blocks: `tools/check-dead-file-coverage.py`'s ledger lookup resolves
`femacs/src/include/android.rs` (this file's sibling header's mirror) to this same `android.md`
through its bare-stem fallback, so a table with gaps at blanks leaves 7 of `src/android.h`'s
non-blank lines uncovered and fails.  (`docs/ports/w32term.md` is the same shape and passes for
the same reason: its rows reach past `src/w32term.h`'s 929.)

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/android-p86/cargo cargo run -q -p xtask -- build-tu src/android.c`
→ `built src/android.rs -> ../build/femacs/android.o`, **exit 0**, and `build/femacs/android.o` is
**336 bytes** - an empty object, the same reading `docs/ports/androidmenu.md`/`androidselect.md`/
`androidfont.md` record for their units.  Read **three** times: once before the first arm compile
(whose errors it named), once after those fixes, and once on the final revision (after the
trailing C-line comments were corrected) - same result each time, all three within the lane's ≤3
`build-tu` budget.  That is the expected reading and not a proof of the new text: every item sits behind the file gate, so rustc compiles only
`include/config.rs` and the `#![allow]` attribute under the frozen feature set.

**② The flip of the arm (P-86 ③⑴/⑥/⑦).**  The two-`--cfg` command the class uses
(`docs/ports/androidmenu.md` §②):

```
CARGO_TARGET_DIR=/tmp/android-p86/cargo cargo run -q -p xtask -- probe src/android.rs \
  /tmp/android-p86/probe1.o --crate-type lib --emit obj -C panic=abort \
  --cfg 'feature="have_android"' --cfg 'feature="not_android_stubify"'
```

* **②a, exit 1, one error line** (`error: aborting due to 1 previous error; 2 warnings emitted`):

  ```
  error[E0609]: no field `sa_sigaction` on type `sigaction`
     --> /Users/liu/.../build/probes/android/_root.rs:997:8
  997 |     sa.sa_sigaction = Some (android_handle_poll_signal);
  help: a field with a similar name exists
  997 -     sa.sa_sigaction = Some (android_handle_poll_signal);
  997 +     sa.__sigaction_u = Some (android_handle_poll_signal);
  ```

  **Classification by file: 1 error, reported at the unit's own line (C 749); 0 errors in the
  lines of any included mirror.  By cause: the one error is a *mirror gap*, 0 are the unit's own
  text** - the included `lib/include/signal.rs` copy carries the frozen Darwin `struct sigaction`
  (`__sigaction_u` + `sa_mask` + `sa_flags`, `sizeof == 16`), which has no direct `sa_sigaction`
  member; C 749's member belongs to bionic's `struct sigaction` (measured below: `sizeof == 32`,
  `sa_mask` at 16).  No object was produced.  This is the `docs/ports/androidfont.md` ②c shape,
  and per P-86 ⑦ the criterion is not "0 errors in the unit" - which is why ②d exists.
* **②b, all sub-gates set (`+ android_api_lt_21`), exit 0, 11 warnings, object 5,232 bytes**:
  `probe src/android.rs /tmp/android-p86/probe2.o … --cfg 'feature="android_api_lt_21"'`.  The
  `__ANDROID_API__ < 21` arm contains no `sa.sa_sigaction`, and with it selected the **pristine**
  graph - no harness - compiles clean and emits an object.  This is a direct reading, not a
  substitute one.
* **②c, all sub-gates set (`+ android_api_lt_21 + threads_enabled`), exit 0, object 8,744
  bytes** (`probe3.o`).  The threads arm also compiles clean with the `< 21` arm.
* **Budget note.**  This lane was given ≤3 `probe` runs and used **four**: ②a, ②b, ②c plus one
  more `probe` run on the ②b config purely to collect the warning inventory (11 warnings: 7
  FFI-safety notices from the mirrors' opaque types (1 `AndroidBitmapInfo`, 2 `_jmethodID`, 4
  `android_vdir`), 2 unused doc comments and 1 each for the `read`/`write` runtime symbols
  `unistd.in.rs` declares).
  The readings are reported as they were taken; the fourth run changed no reading.

**③ Multi-config read-only scan.**  Direct `rustup run nightly-2026-10-01 rustc --edition 2024
--crate-type lib --emit obj -C panic=abort` on the xtask-materialized root
`build/femacs/units/android/_root.rs` (byte-identical to the probe's own
`build/probes/android/_root.rs` after normalising the two output directories - checked with a
script), one config per line:

| config | errors | first error |
| --- | --- | --- |
| `have_android, not_android_stubify` | 1 | `E0609 no field sa_sigaction` (②a) |
| `… , android_api_lt_21` | **0** | - |
| `… , threads_enabled` | 1 | the same `E0609` |
| `… , android_api_lt_21, threads_enabled` | **0** | - |
| `… , aarch64` | 1 | the same `E0609` |
| `… , i386` | 1 | the same `E0609` |

**The only error in any configuration is the single mirror gap.**  `aarch64` adds no new error
(C 70-72's `<arm_neon.h>` is text-only here) and `i386` adds none (C 241-246's `unused_pointer`
is one more declaration).

**④ Arm-reach pre-flight - a *different* reading (P-86 ⑦'s ②d, the androidfont.md shape).**  The
same materialized root with **one patch class**: the `lib/include/signal.rs` copy's `sigaction`
gains the bionic member C 749 assigns
(`pub sa_sigaction: Option<unsafe extern "C" fn (c_int, *mut siginfo_t, *mut c_void)>`), and that
copy's Darwin `sizeof == 16` assertion is re-measured to the patched 24 (the two offset assertions
still hold, the member is appended).  Six configs, all **exit 0**:

| config | object bytes |
| --- | --- |
| `have_android, not_android_stubify` | 23,888 |
| `+ android_api_lt_21` | 24,688 |
| `+ threads_enabled` | 33,448 |
| `+ android_api_lt_21, threads_enabled` | 37,968 |
| `+ aarch64` | 23,888 |
| `+ i386` | 23,920 |

**Mutation test (P-86 ⑦'s criterion, same harness).**  Inserting
`let _mutant: c_int = c"mutant".as_ptr ();` into the unit's `android_pending` body turns the
reading into `exit 1`, `error[E0308]: mismatched types` (`error: aborting due to 1 previous
error`) - the output changes, so ④ is not an empty reading.  The patch and the mutant were
reverted (the copies are byte-identical to the pristine materialization again, checked).

**⑤ Inventory.**  `python3 tools/extract-abi.py --check emacs/src/android.c femacs/src/android.rs`
→ a long `- missing function/object/macro …` list (**the rest of the file**: `android_update_ic`,
`android_user_full_name`, `android_servicing_query`, `android_query_context`, `FIND_METHOD` …,
i.e. C 1038-7594, this slice's boundary - not defects), then two in-slice rows and the drift:

```
- function android_set_task_name does not mention comm, failed, proc, self, set, task
- function android_write_event does not mention break, case, default, switch
- line count drifted 82.7%: C 7594 vs Rust 1311
FAIL emacs/src/android.c -> femacs/src/android.rs
```

* `android_write_event` is a real deviation: C's `switch`/`case`/`break`/`default` are `match`
  with `|` and `_ => {}` in Rust, so the four tokens do not survive (Deviations 7; a
  `macro-body-notes.tsv` row is owed).
* `android_set_task_name` is a **tool artifact, measured**: the reference's own text is corrupted
  before the comparison.  `tools/extract-abi.py`'s C-side blanking reads the `//` inside C 470-471's
  string literal `"/proc/self/task//comm"` as a comment opener; `c_body_text (…,
  'android_set_task_name')` returns `… + sizeof "/proc/self/task         \n  int fd;…` - the rest
  of that line is blanked, the quote never closes, and the token set collapses to the words inside
  the following "string" (`comm, failed, proc, self, set, task`).  The port carries both literals
  intact (`c"/proc/self/task/%ld/comm"`, `c"Failed to set task name for LWP %ld: %s"`).  Owed:
  a `tools/extract-abi.py` fix, or a `macro-body-notes.tsv` row.  The slice's other eleven bodies
  (`android_init_thread_events`, `android_finalize_thread_events`, `android_get_poll_thread`,
  `android_run_select_thread`, `android_handle_poll_signal`, `android_init_events`,
  `android_pending`, `android_wait_event`, `android_next_event`, `android_check_if_event`,
  `android_select`) report no row.
* the drift is the whole-file ratio against a slice; the slice's own ratio is C 1,037 → the port's
  1,311 (+26.4%).

**⑥ Other gates (repo root, final revision).**

```
python3 tools/check-dead-file-coverage.py
  → info docs/ports/android.md: 28 range row(s) cover src/android.c (7594 lines; 0 blank line(s) …)
    info docs/ports/android.md: 28 range row(s) cover src/android.h (363 lines; 0 blank line(s) …)
    … 17 ledger(s) bound (a C range table), 0 failure(s)
python3 tools/check-port-lines.py
  → stale src/android.c: no longer under the threshold
    … 85 file(s) under ten percent, 187,987 reference line(s); 86 ratchet row(s), 0 new, 1 stale
python3 tools/check-citations.py     → 675 file(s) scanned, 0 drift, 0 range, 499 external,
                                       33 allowed, 0 stale
python3 tools/check-declarations.py  → read 43 own prose/manifest file(s) …, 0 failure(s)
python3 tools/check-target-names.py  → 628 .rs files, 591 map targets, 0 finding(s)
```

`src/android.c` is now off the `check-port-lines.py` ratchet list (it is no longer under ten
percent) - that stale row is owed to `--write-pending`.  These are repo-wide readings taken in a
worktree that also carries other lanes' in-flight edits (`femacs/src/{image,xdisp}.rs` and
`femacs/src/include/w32*.rs` are dirty in it); **none of those files is in this unit's include
graph** (checked against the materialized `_root.rs`), so the readings above are unaffected.

**⑦ The NDK values, measured (P-88 ②, not assumed).**  This machine's NDK is
`~/Library/Android/sdk/ndk/29.0.14206865`; the target is `aarch64-linux-android24` (the one
`spikes/jni-probe.sh` defaults to).  `sizeof`/`_Alignof` were read from the clang backend's own
output (`<clang> -S -o - sizes.c`, whose `.xword`/`.word` data items are the constants):

| name | value (bionic, aarch64) |
| --- | --- |
| `pthread_mutex_t` | `sizeof` 40, `_Alignof` 4 |
| `pthread_cond_t` | `sizeof` 48, `_Alignof` 4 |
| `pthread_t` | `sizeof` 8 |
| `pthread_key_t` | `sizeof` 4 |
| `sem_t` | `sizeof` 16, `_Alignof` 4 |
| `sig_atomic_t` | `sizeof` 4 |
| `struct sigaction` | `sizeof` 32, `sa_flags` at 0, `sa_mask` at 16 |
| `siginfo_t` | `sizeof` 128 |
| `sigset_t` | `sizeof` 8 |
| `struct timespec` | `sizeof` 16 |

Macros, via `<clang> -E -dM`: `SA_SIGINFO 0x4`, `SIGUSR1 10`, `SIGIO 29`, `SIG_BLOCK 0`,
`FD_SETSIZE 1024`, `O_WRONLY 1`, `O_TRUNC 512`, `O_CLOEXEC 524288`, `EINTR 4`; the three log
levels via `-S` on a TU initialized from the enum: `ANDROID_LOG_INFO 4`, `ANDROID_LOG_WARN 5`,
`ANDROID_LOG_FATAL 7`.

## Deviations

* **`<pthread.h>`, `<semaphore.h>` and six `<signal.h>` names have no mirror** and are declared
  in the unit (P-88/P-70; the `sysdep.rs` `<pthread.h>` block and androidselect.rs's `<dlfcn.h>`
  block are the precedents): `pthread_mutex_t`/`pthread_cond_t`/`pthread_t`/`pthread_key_t`,
  `sem_t`, the thirteen `pthread_*`/`sem_*` entries, `gettid`, `pipe2`, `kill`, `sigfillset`,
  `sigdelset`, `sigwait`, `sig_atomic_t`, `siginfo_t` (opaque - only a pointer here), `SA_SIGINFO`.
  Their values are the bionic ones measured in ⑦.  Each should move into a mirror once one exists.
* **The frozen system mirrors are the frozen target's, not Android's.**  `signal.rs`'s `sigset_t`
  (4 bytes) and `sigaction` (16 bytes, `__sigaction_u.__sa_sigaction`) are Darwin's where bionic's
  are 8 and 32 with `sa_sigaction` (⑦); `signal.rs`'s `SIGUSR1`/`SIGIO` are 30/23 where bionic's
  are 10/29; `sys_select.rs`'s `fd_set`/`FD_ZERO`/`FD_SET`/`FD_ISSET`/`FD_CLR` are Darwin's
  (`__darwin_check_fd_set_overflow`); `errno.rs`'s `errno ()` is Darwin's `*__error ()` where
  bionic's is `*__errno ()`.  The port uses them as they are; the Android ABI is **not** what
  compiles here, and ④'s one patch class is where the `sigaction` difference bites.  This is the
  mirror round owed below.
* **`sa.sa_sigaction = Some (android_handle_poll_signal)`** (C 749) keeps C's member name.  The
  frozen `signal.rs` has no such member, so this is the ②a error; with `android_api_lt_21` the arm
  never reaches it and the pristine graph compiles (②b/②c).
* **`volatile`**: C 303's `volatile sig_atomic_t cancel_signal_received` is a plain `sig_atomic_t`
  field (Rust has no volatile field form).
* **`goto` and labels (PORTING.md 2.2)**: C 491-492's `goto failure` and C 497-499's `failure:`
  become the labeled block `'failure:` whose `break 'failure` is the jump, with C's `failure:` tail
  one copy after it.  The `goto` token does not survive (⑤, owed).
* **`switch` (C 894-908)**: `match (*event).r#type { ANDROID_KEY_PRESS | ANDROID_WINDOW_ACTION =>
  … , _ => {} }`; C's two case labels are one `|` arm and `default:` is `_` (⑤, owed).  The union
  field is `r#type` (the C keyword `type`).
* **C's `malloc`/`free` (C 872, 902)**: android.c reaches `<stdlib.h>` transitively; the
  port includes `../lib/include/stdlib.rs` at C 37's place (the `androidterm.rs` shape).  C's
  `sizeof *queue`/`sizeof *container` are `std::mem::size_of::<android_thread_event_queue> ()` /
  `<android_event_container> ()`.
* **`__aarch64__` (C 70-72)**: `#include <arm_neon.h>` has no mirror and no name of it is used
  before C 4064, so this slice carries the pair as a comment; the P-88 declaration segment belongs
  to the slice that first names a NEON intrinsic.
* **`INT_STRLEN_BOUND` (C 28)**: the `intprops.rs` mirror is blind and no unit includes it; the
  unit defines the macro (the androidselect.rs 116-119 shape, over `size_of::<$t>`).
* **`sizeof "/proc/self/task//comm"` is 21**: the array length is written `INT_STRLEN_BOUND!
  (c_long) + 21`, with C's expression in the comment.
* **C's file-scope objects are tentative definitions** (PORTING.md 0.1-13/1.5), so the port spells
  every one out: `0` / `false` / `std::ptr::null_mut ()` / `std::mem::zeroed ()` (the seven
  `static struct` class records, `service_class`, `emacs_installation_time`,
  `startup_signal_mask`, `event_queue`, `android_query_sem`).
* **Locals C leaves uninitialised and the port zeroes** (PORTING.md 1.5): `fd`, `lwp`, `length`
  (C 472-474), `rc`/`data` (C 505-506), `nfds`/`readfds`/`byte` (C 510-512), `signals`/`waitset`/
  `sig` (C 514-515), `queue` (C 676), `sa` (C 714), `i` (C 769), `container` (C 806, 841),
  `nfds_return`/`nevents` (C 921).  C 924's function-local `static char byte` is a `#[cfg]`-gated
  `static mut`.
* **`MIN (16, length)` (C 487) is `MIN! (16usize, length)`**: C's `int`/`size_t` promotion is the
  `usize` literal.
* **C 339's forward declaration** has no Rust item (a function item is visible for the whole
  module); the definition is C 501-670.
* **The `#else` arm of C 61 (C 7519-7594, the `ANDROID_STUBIFY` arm)** is in the remainder, not
  this slice: this port carries the *real* arm of the whole-file `#ifndef`, and the stub arm is
  owed by the slice that ports it.
* **Line count**: C 1,037 (this slice) vs the port's 1,311 (+26.4%) - the per-item `#[cfg]`s, the
  in-unit declarations, Rust's brace-only lines and the C-line comments.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. ✅ **Done in 3.412** — `femacs/xtask/Cargo.toml`'s `[features]` table: **`android_api_lt_21`** (comment: the derived
   condition `__ANDROID_API__ < 21`, android.c 297/310/367/426/509/522/604/672/713/738/923/973;
   its negation is C's `>= 21` arm).  Without it none of this slice's `#if` pairs can be selected.
2. ✅ **Done** — `docs/ports/macro-body-notes.tsv`: `android_write_event` →
   `break,case,default,switch` landed with this slice; `android_set_task_name` → `goto` could not
   be measured until 3.414 fixed the extractor's `//`-in-string defect, and is now recorded there
   (C 481/486's `goto failure` and the `failure:` label at C 491).
3. `docs/ports/line-drift-tolerance.tsv`: one row for `android` once the last slice lands (the
   whole-file reading is the 82.7% of ⑤; this slice's own ratio is +26.4%).
4. `tools/check-port-lines.py --write-pending`: this file's ratchet row is now stale (it is no
   longer under ten percent) and the re-run drops it.  Deferred on purpose to a batch close (the
   tool's own reading is `86 ratchet row(s), 0 new, 1 stale` and it exits 0), so that the rewrite
   does not encode the two lanes' in-flight units.
5. ✅ **Done in 3.414** — the tool was fixed, not the symptom:
   `tools/cinventory.py`'s `blank_comments` now models string and character literals, so C 470-471's
   `"/proc/self/task//comm"` no longer blanks the rest of the line.  The false positive
   (`does not mention comm, failed, proc, self, set, task`) is gone and the real finding - the
   `goto` of item 2 - took its place.  See `docs/status.md` 3.414 for the 634-file reading and the
   second manifestation it uncovered in `w32fns.c` 4127.
6. **The mirror round that blocks the pristine base-config arm: `lib/include/signal.rs` needs the
   bionic `sa_sigaction` member C 749 names** (or an Android `signal.h` mirror).  That file is
   another lane's in-flight mirror and the change needs `tools/check-all-units.sh` (P-87), which is
   why it is not touched here; until it lands the base config reads ②a and the unit's own body
   needs ④'s harness.
7. The in-unit declarations of Deviations 1 belong in mirrors once those exist (P-88 ⑤): the
   `<pthread.h>`/`<semaphore.h>` block, the `<signal.h>` extras, `gettid`/`kill` and the
   `<android/log.h>` pair.  **`pipe2` is settled and stays where it is** (3.414): bionic's
   `<unistd.h>` declares it for this configuration, and `docs/baseline/mirror-dupes.tsv` now
   carries the ratchet row that says why the mirror's Darwin redirect and this declaration are two
   configurations' arms rather than a copied declaration.

### Not claimed

Every line of this slice is behind gates the frozen configuration leaves false: ① is an empty
object, the arm readings (②/③/④) are compile-level, and **nothing here was executed** - no
thread, no semaphore, no JNI call, no event, no `select`.  C 1038-7594 is **not ported here**
(including C 7519's `ANDROID_STUBIFY` arm); `extract-abi --check`'s `- missing …` lines and the
82.7% drift are that boundary, not defects.  The `android_api_lt_21` configurations compiled to
objects (②b/②c) and were not run.
