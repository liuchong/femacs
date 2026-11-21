// femacs: port of src/syssignal.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): C's declarations, the three typedefs/objects and the four
// feature macros, whose arms the frozen configuration decides
// (docs/baseline/config/darwin-arm64.macros.tsv):
//
//   HAVE_PTHREAD 1      -> `FORWARD_SIGNAL_TO_MAIN_THREAD` is live text
//   HAVE_TIMER_SETTIME undef, SIGEV_SIGNAL 1 -> `HAVE_ITIMERSPEC` is *not* defined
//   SIGPROF 27, PROFILING undef, HAVE_SETITIMER 1 -> `PROFILER_CPU_SUPPORT` *is* defined
//   NSIG_MINIMUM undef  -> `#if NSIG < NSIG_MINIMUM` is false (an undefined identifier is 0 in
//                          a `#if`), so the `#undef NSIG` arm is dead
//   SA_SIGINFO 0x0040   -> `#ifndef SA_SIGINFO` is dead
//   HAVE_STRSIGNAL 1    -> `#ifndef HAVE_STRSIGNAL` is dead
//
// `#include <attribute.h>` has no counterpart: its content is the `ATTRIBUTE_*` spellings,
// which docs/ports/excluded-macros.txt records as attributes with no Rust form (PORTING 0.3).

// [U1-MODULE-MIRROR]
use super::*;
// [U1] ../../lib/include/signal.rs imported via super

// syssignal.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_SYSSIGNAL_H: bool = true;

/* syssignal.h 24-30 and 61-86's prototypes, in a uniquely-named module + glob import: the
   mirror declares what `sysdep.c` *defines*, and a definition at the crate root would be
   E0428 with a declaration there (`emacs_sigaction_init`, `safe_strsignal`,
   `deliver_process_signal` and the six signal-mask entries).  The definition shadows the
   glob import, which is exactly C's "prototype + definition" (PORTING.md 0.1-10, the
   indent.rs `indent_decls` shape).  */
pub mod syssignal_decls {
  use super::*;

  unsafe extern "C" {
    pub fn init_signals ();                                                 // syssignal.h 24
    pub fn block_child_signal (set: *mut sigset_t);                         // 25
    pub fn unblock_child_signal (set: *const sigset_t);                     // 26
    pub fn block_interrupt_signal (set: *mut sigset_t);                     // 27
    pub fn restore_signal_mask (set: *const sigset_t);                      // 28
    pub fn block_tty_out_signal (set: *mut sigset_t);                       // 29
    pub fn unblock_tty_out_signal (set: *const sigset_t);                   // 30
  }

  unsafe extern "C" {
    pub static mut empty_mask: sigset_t;                                           // 54
  }

  unsafe extern "C" {
    pub fn emacs_sigaction_init (act: *mut sigaction, handler: signal_handler_t);  // 61
    pub fn safe_strsignal (sig: c_int) -> *const c_char;                           // 62
    pub fn deliver_process_signal (sig: c_int, handler: signal_handler_t);         // 86
  }
}
pub use self::syssignal_decls::*;

/* If defined, asynchronous signals delivered to a non-main thread are
   forwarded to the main thread.  */
// syssignal.h 33-35: `#ifdef HAVE_PTHREAD` (1 on this target) around
// `#define FORWARD_SIGNAL_TO_MAIN_THREAD`; the name is what the other units test, so the port
// keeps it as a constant.
pub const FORWARD_SIGNAL_TO_MAIN_THREAD: bool = true;

// syssignal.h 40-42's `#if defined CYGWIN && !defined SIGEV_SIGNAL` is dead (not Cygwin).
// syssignal.h 44-46's `#if defined HAVE_TIMER_SETTIME && defined SIGEV_SIGNAL` defines
// HAVE_ITIMERSPEC only when the timer interface is used; this target's table has
// HAVE_TIMER_SETTIME undefined, so it is not.
// syssignal.h 48-51's `#if (defined SIGPROF && !defined PROFILING && (defined HAVE_SETITIMER
// || defined HAVE_ITIMERSPEC))`: SIGPROF is 27, PROFILING is undefined and HAVE_SETITIMER is 1,
// so the macro is defined here.
pub const PROFILER_CPU_SUPPORT: bool = true;

// syssignal.h 54's `extern sigset_t empty_mask;`: `sysdep.c` defines it, so the
// declaration lives in the module below with the prototypes (PORTING.md 0.1-10).


/* typedef void (*signal_handler_t) (int);  */
// The pointer is nullable in C (handlers installed by `emacs_sigaction_init` may be null), so
// the port spells it `Option<unsafe extern "C" fn (…)>` (P-27).
pub type signal_handler_t = Option<unsafe extern "C" fn (sig: c_int)>;

// `struct sigaction` is a *system* type this target's `<signal.h>` provides.  It was P-70's
// stand-in until a unit needed its size; `emacs_sigaction_init` writes one and `sigaction`
// reads it, so the real record (with the frozen target's measured layout) now lives in the
// header that declares it — `lib/include/signal.rs`, next to `sigset_t` and
// `pthread_sigmask` — and the stand-in is gone in the same commit (P-70).
// syssignal.h 64-67's `#if NSIG < NSIG_MINIMUM` is false here (NSIG_MINIMUM is undefined and
// an undefined identifier is 0 in a `#if`), so `NSIG` keeps `<signal.h>`'s 32.
// syssignal.h 69-71's `#ifndef SA_SIGINFO` is dead (SA_SIGINFO is 0x0040 in the table).
// syssignal.h 73-77's `#ifdef WINDOWSNT` (the `raise` -> `w32_raise` rename) is dead, and with
// it `int w32_raise (int);`.
// syssignal.h 79-81: `#ifndef emacs_raise` is live (nothing defines the name), so the port
// keeps C's macro (P-37) — the call site reads `emacs_raise! (sig)`.
macro_rules! emacs_raise { ($sig:expr) => { raise ($sig) }; }
// syssignal.h 83-85's `#ifndef HAVE_STRSIGNAL` is dead: the table defines HAVE_STRSIGNAL, so
// `strsignal` is the system one and `safe_strsignal` keeps only its declaration above.
