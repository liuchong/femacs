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

`emacs/src/systhread.c` @ `fd7afced403`; port `femacs/src/systhread.rs` (124 lines vs C's 553 -
the tolerance is declared in `docs/ports/line-drift-tolerance.tsv` as `systhread 0.80`); the
gates are `build-tu src/systhread.c` -> `built` and
`python3 tools/extract-abi.py --check emacs/src/systhread.c femacs/src/systhread.rs` -> `OK`.

- **Which arm is live.**  `THREADS_ENABLED` is `undef` in the frozen table
  (`docs/baseline/config/darwin-arm64.macros.tsv` 3678), so C 33's `#ifndef THREADS_ENABLED`
  arm - the thirteen no-op definitions - is the whole of this target's text, and
  `#elif defined (HAVE_PTHREAD)`'s C 104-548 is dead.  That matches the mirror: src/systhread.h
  130-137's `typedef int sys_mutex_t/sys_cond_t/sys_thread_t` is the arm the unit compiles
  against, and `sys_thread_create` returning `false` is C 97's own return.
- **Dead text (not transcribed, recorded here).**  C 104-548: the `#include <sched.h>`,
  `sys_mutex_init`'s `pthread_mutexattr_init`/`settype (PTHREAD_MUTEX_ERRORCHECK)` pair under
  `ENABLE_CHECKING` (which is `undef` here anyway), `sys_cond_wait`'s
  `pthread_cond_wait`/`sched_yield` choice, the `HAVE_NS` longjmp path (C 190-196), the
  `HAVE_PTHREAD_SETNAME_NP`/`HAVE_PTHREAD_SET_NAME_NP` spellings of `sys_thread_set_name`
  (C 222-241 - both undefined on this target), `sys_thread_create`'s
  `pthread_create`+`pthread_sigmask` body, and C 280-548's `#elif defined (WINDOWSNT)` arm with
  its `CreateThread`/critical-section code.  C 25-27's `#include "nsterm.h"` and C 29-31's
  `#include <pthread_np.h>` are dead for the same reason (`HAVE_NS`,
  `HAVE_PTHREAD_SET_NAME_NP` are both `undef`).
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
