# emacs/src/systhread.h (160 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] SYSTHREAD_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/src/systhread.h` @ `fd7afced403`; port `femacs/src/include/systhread.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/src/systhread.h femacs/src/include/systhread.rs`
-> `OK` (the header carries no function definitions, so the checker's inventory is the guard
macro alone; the types are compared through the unit that uses them).

- **Which arm is live.**  `THREADS_ENABLED` is `undef` (the frozen table 3678), so C 130-137's
  `#else` arm is the text: `typedef int sys_mutex_t/sys_cond_t/sys_thread_t` - and that is what
  `femacs/src/systhread.rs` compiles against, including `sys_thread_self`'s `return 0` and
  `sys_thread_create`'s `false`.
- **Dead text (recorded, not transcribed).**  C 22's `#include <attribute.h>` IS kept (the same
  header's `NODISCARD` token, see the unit's ledger); C 24-128 is the threaded arm:
  `#include <pthread.h>`, the `ALIGNOF_PTHREAD_{COND,MUTEX}_T` comparison with
  `SYSTHREAD_ALIGN_ROOM`/`SYSTHREAD_ALIGN_PTR` (the 32-bit HPPA alignment workaround), the
  `pthread_mutex_t`/`pthread_cond_t`/`pthread_t` typedefs, the WINDOWSNT `w32thread_critsect` /
  `CONDV_*` / `w32thread_cond_t` records and the `#error port me` fallback for other targets.
- **`thread_creation_function`** (C 139) is the one live typedef besides the three dummy types;
  C's function type becomes `Option<unsafe extern "C" fn (arg: *mut c_void) -> *mut c_void>`
  (P-27, the atimer_callback shape).
- **The declarations live in `systhread_decls` + glob import** (the tempname.rs shape): the unit
  defines all thirteen, and a Rust definition cannot share a scope with the foreign declaration
  of the same name (E0428; P-21/P-39).
- The declaration order is C's; the `#[unsafe(no_mangle)]` side is the unit's business, so the
  mirror keeps only `unsafe extern "C"` items.
