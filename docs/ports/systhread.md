# emacs/src/systhread.c (553 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 13
- [x] sys_mutex_init  (line 36)
- [x] sys_mutex_lock  (line 42)
- [x] sys_mutex_unlock  (line 47)
- [x] sys_cond_init  (line 52)
- [x] sys_cond_wait  (line 58)
- [x] sys_cond_signal  (line 63)
- [x] sys_cond_broadcast  (line 68)
- [x] sys_cond_destroy  (line 73)
- [x] sys_thread_self  (line 78)
- [x] sys_thread_equal  (line 84)
- [x] sys_thread_set_name  (line 89)
- [x] sys_thread_create  (line 94)
- [x] sys_thread_yield  (line 100)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/systhread.c` @ `fd7afced403`; port `femacs/src/systhread.rs` (**462 lines** after
status 3.491, C's 553; the live baseline the tolerance row uses is 101 lines) and the header
mirror `femacs/src/include/systhread.rs` (**176 lines**).  The gates are
`build-tu src/systhread.c` -> `built` (exit 0, an unchanged 2600-byte object with exactly the
thirteen `_sys_*` symbols) and
`python3 tools/extract-abi.py --check emacs/src/systhread.c femacs/src/systhread.rs`, which
**fails on the line-count row alone** (`C 101 live (of 553) vs Rust 462`) because the
`WINDOWSNT` arm is still owed; per the 3.434 clause-5 ruling a partially ported file gets no
tolerance row.

- **Which arm is live.**  `THREADS_ENABLED` is `undef` in the frozen table
  (`docs/baseline/config/darwin-arm64.macros.tsv` 3678), so C 33's `#ifndef THREADS_ENABLED`
  arm - the thirteen no-op definitions - is the whole of this target's text, and
  `#elif defined (HAVE_PTHREAD)`'s C 104-548 is dead.  That matches the mirror: src/systhread.h
  130-137's `typedef int sys_mutex_t/sys_cond_t/sys_thread_t` is the arm the unit compiles
  against, and `sys_thread_create` returning `false` is C 97's own return.
- **C 104-278 (the `HAVE_PTHREAD` arm) is transcribed** (status 3.491), under
  `all (feature = "threads_enabled", feature = "have_pthread")`: the `#include <sched.h>`, the
  thirteen definitions, `sys_mutex_init`'s `pthread_mutexattr_init`/`settype
  (PTHREAD_MUTEX_ERRORCHECK)` pair under `ENABLE_CHECKING` (C's two `#ifdef` arms are the port's
  two `#[cfg]` blocks inside the one function), `sys_cond_wait`'s `pthread_cond_wait`, the
  `HAVE_NS` block of `sys_cond_broadcast` (C 191-197), the three `pthread_setname_np` arities and
  the `pthread_set_name_np` spelling of `sys_thread_set_name` (C 223-241), `sys_thread_create`'s
  attribute/stack-size/`pthread_create` body with C's `out:` label, and `sched_yield` (C 275-278).
  The `<pthread.h>` surface stands in `src/systhread.rs` (P-88 clause 1: the mirror cannot carry
  it - `src/android.rs`/`src/sysdep.rs` declare the same names for their own arms and a mirror-side
  second spelling is the duplicate `tools/check-mirror-dupes.py` fails on), with clang
  measurements of this target's `<pthread.h>`.
- **Still owed: C 280-548, the `WINDOWSNT` arm** (the critical-section mutexes, the event-based
  condition variable, the `_THREADNAME_INFO`/`SetThreadDescription` name path and `_beginthread`),
  text-only here: there is no Windows toolchain on this machine (`which -a x86_64-w64-mingw32-gcc`
  is empty).  C 549-551's `#else`/`#error port me` is carried as the port's `compile_error!` under
  `all (threads_enabled, not (have_pthread), not (windowsnt))`; C 25-27's `#include "nsterm.h"`
  and C 29-31's `#include <pthread_np.h>` are dead for the same reason (`HAVE_NS` is `undef` and
  `HAVE_PTHREAD_SET_NAME_NP` is `undef`, macros.tsv 1481/1507); `ns_run_loop_break` is declared
  in-unit under `have_ns` (the mirror carries it at `include/nsterm.rs` 877-882).
- **Include list.**  C 20-23's `config.h`, `<setjmp.h>`, `<stdio.h>`, `<string.h>` plus C 24's
  `lisp.h`: the port keeps `config.rs`, `string.rs`, `lisp.rs` (and `systhread.rs`), because
  `lisp.h` itself includes `<setjmp.h>`/`<stdio.h>` and the include graph de-duplicates
  (PORTING.md 0.1-4).  Nothing in the live arm uses `memcpy` or `FILE`; the two mirrors are
  kept for C's include list, not for tokens.
- **`NODISCARD`.**  src/systhread.h 151-156 puts it in front of `sys_thread_self`,
  `sys_thread_equal` and `sys_thread_create`; attribute.h 187 spells it
  `_GL_ATTRIBUTE_NODISCARD` = `__attribute__ ((__warn_unused_result__))`, and Rust has no
  counterpart to the *declaration* form (PORTING.md 0.3).  The token is defined as a no-op
  macro in `lib/include/attribute.rs` (next to the existing `__attribute_warn_unused_result__`)
  and is not written at the declaration site; the alternative, `#[must_use]` on the foreign
  declarations, would attach a Rust-only attribute to C's ABI and is not what the other
  attribute rows do.
- **`thread_creation_function`** is `void *(thread_creation_function) (void *)` - a function
  *type* in C, passed as `thread_creation_function *`.  The port names the pointer type the way
  the atimer.h mirror names `atimer_callback` (P-27: `Option<unsafe extern "C" fn (...) -> T>`)
  and writes it bare at the one call site, which is where C's extra `*` disappears.

## C line ranges

The rows are contiguous and ascending; `docs/port-map.md` binds this ledger to `src/systhread.c`
(the header is bound to `docs/ports/include-systhread.md`).

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 clause 1) |
| 19-24 | `config.h`, `<setjmp.h>`, `<stdio.h>`, `<string.h>`, `"lisp.h"` | no gate (the two headers come through lisp.h's chain, 0.1-4) |
| 25-28 | `#ifdef HAVE_NS` -> `#include "nsterm.h"` | dead both ways (`HAVE_NS` undef) - recorded |
| 29-32 | `#ifdef HAVE_PTHREAD_SET_NAME_NP` -> `#include <pthread_np.h>` | dead both ways (`HAVE_PTHREAD_SET_NAME_NP` undef, macros.tsv 1507) |
| 33-103 | `#ifndef THREADS_ENABLED`: the thirteen no-op definitions | `not (threads_enabled)` (live here) |
| 104-279 | `#elif defined (HAVE_PTHREAD)`: the thirteen pthread definitions | `threads_enabled` + `have_pthread` (**transcribed 3.491**) |
| 280-548 | `#elif defined (WINDOWSNT)`: the critical-section/event implementations | `threads_enabled` + `not (have_pthread)` + `windowsnt` - **owed** (text-only) |
| 549-553 | `#else` / `#error port me` / `#endif` | `compile_error!` under the fallback gate |

## Evidence

- frozen `build-tu src/systhread.c` -> **exit 0**, `build/femacs/systhread.o` **2600 bytes** with
  exactly **13** exported `_sys_*` symbols and **zero** `pthread*`/`sched_yield` references
  (`nm -g`), i.e. the transcribed arm sits behind its gate (P-86 clause 8).
- **arm flip (a real compile: pthreads are on this machine)**, status 3.491:
  `probe src/systhread.rs --crate-type=lib -C panic=abort` with `threads_enabled`, `have_pthread`,
  `have_pthread_setname_np`, `have_pthread_setname_np_1arg`, `enable_checking` and both
  `alignof_pthread_{cond,mutex}_t_le_alignof_max_align_t` -> **6 errors, every one of them
  `E0080` layout assertions inside mirror copies** (`thread_state` 472, `not_holding_lock` 432,
  `bc` 448, `lisp_mutex_t` 16, `Lisp_Mutex` 32, `Lisp_CondVar` 32 - those mirrors record the
  frozen arm's dummy-type sizes), **0 errors in this unit's own text**.
- **mutation check (P-86 clause 7)**: injecting `let _mutation: c_int = c"injected type error".as_ptr ();`
  into `sys_mutex_lock` took the same command from 6 to **9** errors; the file was then restored
  from the pre-mutation copy (md5 `85375026454b2e80737ba32cc23a055b`).
- **clang measurements** (this machine's `pthread.h`, arm64): `sizeof (pthread_mutex_t)` 64 align 8,
  `sizeof (pthread_cond_t)` 48 align 8, `sizeof (pthread_attr_t)` 64, `sizeof (pthread_mutexattr_t)`
  16, `sizeof (pthread_t)` 8, `PTHREAD_MUTEX_ERRORCHECK` 1, `PTHREAD_CREATE_DETACHED` 2,
  `ALIGNOF_PTHREAD_MUTEX_T`/`ALIGNOF_PTHREAD_COND_T`/`ALIGNOF_MAX_ALIGN_T` all 8.
- `check-mirror-dupes` **17 duplicate declaration(s), 17 ratchet row(s), 0 failure(s)**;
  `check-declarations` 10 sites / 0 failures; `check-citations` 0 drift;
  `check-dead-file-coverage` binds the table above with 0 failures.

## Deviations

1. **`attr`/`stack_size` are zeroed before their `&raw mut` borrow** (C declares them
   uninitialized): Rust rejects a borrow of an uninitialized binding (`E0381`).  The first
   statement that reads either is the C call that initializes it, so the value is not observable.
2. **`sys_thread_set_name`'s three `pthread_setname_np` arities** are one C name spelled three ways
   by `HAVE_PTHREAD_SETNAME_NP_{1ARG,3ARG}`; Rust cannot overload, so the port declares
   `pthread_setname_np_1`/`_2`/`_3` with `#[link_name = "pthread_setname_np"]` - the symbol is C's,
   only the Rust-side name differs (the `charset.rs` 115-116 shape).
3. **The `<pthread.h>` surface lives in the unit, not in the header mirror** (P-88 clause 1):
   `tools/check-mirror-dupes.py` fails on a mirror-side spelling of names `src/android.rs` and
   `src/sysdep.rs` already declare for their own arms (measured: 10 failures while the surface sat
   in `include/systhread.rs`).
4. **`strncpy` is declared at this unit's C 229 use site** - `lib/include/string.rs` carries
   `strcmp`/`strlen` and friends but not this entry point.

## Owed

1. **C 280-548, the `WINDOWSNT` arm** (text-only here): the critical-section mutexes, the
   event-based condition variable, the `_THREADNAME_INFO`/`SetThreadDescription` name path and
   `_beginthread`, plus the `w32thread_critsect`/`w32thread_cond_t` records the header mirror
   already carries under `windowsnt`.
2. **The mirror copies' per-arm layout values** (`include/thread.rs`'s `thread_state`,
   `lisp_mutex_t`, `Lisp_Mutex`, `Lisp_CondVar`): they record the frozen arm's dummy-type sizes, so
   a `have_pthread` build trips their assertions (the six `E0080`s above).  A mirror round must
   carry both arms' values.
3. **`docs/baseline/feature-coverage.tsv`** has been regenerated (status 3.491) with the eight
   features this slice added.

## Not claimed

- **Zero run evidence.**  No thread, mutex, condition variable or attribute object was ever
  created; nothing was linked; the frozen object is the thirteen no-ops and it is never executed.
- The flip is a **compile reading** and it does not reach `exit 0`: the six `E0080`s are the mirror
  gap named in `## Owed` 2, not a claim that the arm is verified.
- The `WINDOWSNT` arm is **text only**: no Windows API type, constant or width in it has been
  measured or compiled.
- The `alignof_*` comparisons and the room-allocating arm are measured for **this** target;
  another platform's numbers would move both the features and the records.
