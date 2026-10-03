// femacs: the parts of the system <signal.h> the port needs (PORTING.md 0.1, the errno.rs
// shape).  The values were measured from the frozen target's clang
// (`clang -dM -E` on a file that includes <signal.h>); `c_int` comes from the including unit.
// The signals this target does not define (SIGPWR, SIGSTKFLT and the other platform-specific
// ones sigdescr_np.c guards with `#if defined`) are absent on purpose: docs/ports/sigdescr_np.md
// records them.
pub const SIGABRT: c_int = 6;
pub const SIGALRM: c_int = 14;
pub const SIGBUS: c_int = 10;
pub const SIGCHLD: c_int = 20;
pub const SIGCONT: c_int = 19;
pub const SIGEMT: c_int = 7;
pub const SIGFPE: c_int = 8;
pub const SIGHUP: c_int = 1;
pub const SIGILL: c_int = 4;
pub const SIGINFO: c_int = 29;
pub const SIGINT: c_int = 2;
pub const SIGIOT: c_int = 6;	// = SIGABRT on this target (measured)
pub const SIGIO: c_int = 23;
pub const SIGKILL: c_int = 9;
pub const SIGPIPE: c_int = 13;
pub const SIGPROF: c_int = 27;
pub const SIGQUIT: c_int = 3;
pub const SIGSEGV: c_int = 11;
pub const SIGSTOP: c_int = 17;
pub const SIGSYS: c_int = 12;
pub const SIGTERM: c_int = 15;
pub const SIGTRAP: c_int = 5;
pub const SIGTSTP: c_int = 18;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGURG: c_int = 16;
pub const SIGUSR1: c_int = 30;
pub const SIGUSR2: c_int = 31;
pub const SIGVTALRM: c_int = 26;
pub const SIGWINCH: c_int = 28;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;

/// `<signal.h>`'s `NSIG` is `__DARWIN_NSIG` on this target (measured: 32); sig2str.h's
/// `SIGNUM_BOUND` is `NSIG - 1`.
pub const NSIG: c_int = 32;

/* The three entries lib/pthread_sigmask.c and lib/pselect.c reach through <signal.h>.
   `sigset_t` is Darwin's `struct __darwin_sigset_t { __uint32_t __val; }`; the measurement
   (`printf ("%zu %d %d %d", sizeof (sigset_t), SIG_BLOCK, SIG_UNBLOCK, SIG_SETMASK)`)
   prints `4 1 2 3`.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigset_t
{
  pub __val: u32,
}

const _: () = assert! (std::mem::size_of::<sigset_t> () == 4);

/// <signal.h>: `#define SIG_BLOCK 1`.
pub const SIG_BLOCK: c_int = 1;
/// <signal.h>: `#define SIG_UNBLOCK 2`.
pub const SIG_UNBLOCK: c_int = 2;
/// <signal.h>: `#define SIG_SETMASK 3`.
pub const SIG_SETMASK: c_int = 3;

/* The `sigset_t` operations, in a uniquely-named module + glob import (the stdio.rs shape):
   a unit that defines one of them itself (none does yet, but lib/pthread_sigmask.c wraps
   `pthread_sigmask`) then shadows the glob instead of clashing with it (PORTING.md 0.1-10).
   C declares `pthread_sigmask` only when the platform has not got it; on this target it is
   the system entry, so the declaration is present for the units that wrap it.  */
pub mod signal_decls {
  use super::*;

  unsafe extern "C" {
    /* int sigemptyset (sigset_t *);  */
    pub fn sigemptyset (set: *mut sigset_t) -> c_int;
    /* int sigaddset (sigset_t *, int);  */
    pub fn sigaddset (set: *mut sigset_t, signo: c_int) -> c_int;
    /* int sigprocmask (int, const sigset_t *, sigset_t *);  */
    pub fn sigprocmask (how: c_int, set: *const sigset_t, oset: *mut sigset_t) -> c_int;
    /* int pthread_sigmask (int, const sigset_t *, sigset_t *);  */
    pub fn pthread_sigmask (how: c_int, set: *const sigset_t, oset: *mut sigset_t) -> c_int;
    /* int raise (int);  -- syssignal.h 80's `emacs_raise` is `raise (sig)` on this target.  */
    pub fn raise (sig: c_int) -> c_int;
  }
}
use signal_decls::*;
