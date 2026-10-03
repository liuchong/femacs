# emacs/src/syssignal.h (88 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] EMACS_SYSSIGNAL_H  (line 21, body da39a3ee5e6b)
- [x] FORWARD_SIGNAL_TO_MAIN_THREAD  (line 39, body da39a3ee5e6b)
- [x] PROFILER_CPU_SUPPORT  (line 54, body da39a3ee5e6b)
- [x] emacs_raise  (line 79, body 72bade2c75b4)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern sigset_t empty_mask  (line 57)

## Decisions

- **The four feature macros follow the frozen table** (`docs/baseline/config/darwin-arm64.macros.tsv`):
  `HAVE_PTHREAD` = 1 makes `FORWARD_SIGNAL_TO_MAIN_THREAD` live text; `HAVE_TIMER_SETTIME` is
  undefined, so `HAVE_ITIMERSPEC` is *not* defined; `SIGPROF` = 27 with `PROFILING` undefined
  and `HAVE_SETITIMER` = 1, so `PROFILER_CPU_SUPPORT` *is*; `NSIG_MINIMUM` is undefined, so
  `#if NSIG < NSIG_MINIMUM` is false (an undefined identifier is 0 in a `#if`) and `NSIG` keeps
  `<signal.h>`'s 32.
- **`#ifndef SA_SIGINFO` and `#ifndef HAVE_STRSIGNAL` are dead** (both are defined in the
  table), so `SA_SIGINFO` keeps the system value and `strsignal` stays the system function;
  `safe_strsignal` keeps its declaration only.  `#ifdef WINDOWSNT`'s `raise` -> `w32_raise`
  rename and `int w32_raise (int)` are dead with it.
- **`emacs_raise` stays a macro** (P-37): nothing in the tree defines the name, so
  `#ifndef emacs_raise` is live and the port keeps C's spelling — the call site reads
  `emacs_raise! (sig)`.  Its body needs `raise`, which the `<signal.h>` mirror now declares
  (`femacs/lib/include/signal.rs`, right after `pthread_sigmask`).
- **`struct sigaction` is the measured system record, not a stand-in** (it was P-70's
  2-line stand-in until `src/atimer.c` needed it): the type belongs to `<signal.h>`, so the
  record lives in `femacs/lib/include/signal.rs` beside `sigset_t`, with the frozen target's
  measured layout (`sizeof 16`, `sa_mask` at 8, `sa_flags` at 12, handler union 8 bytes at 0)
  pinned by `const _` assertions; this mirror only names it.  P-70 requires the stand-in to go
  in the same commit that lands the real record (two declarations of one name in a TU are
  E0428), which is what happened.
- **`signal_handler_t` is `Option<unsafe extern "C" fn (c_int)>`** (P-27): C's function
  pointers are nullable and `emacs_sigaction_init`'s callers pass null.
- **Compile evidence**: this mirror was first compiled inside a `config.rs` + `lisp.rs` +
  mirror probe with `rustc --edition 2024 --crate-type lib --emit metadata` — RC=0.  It is now
  included by `src/atimer.rs`, which `xtask build-tu` compiles as a whole translation unit
  (the ledger for that unit records the result).
