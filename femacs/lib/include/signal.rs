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

/// `<signal.h>`: `#define SIG_IGN ((void (*)(int)) 1)` - the integer one cast to the
/// handler type (`SIG_DFL` is the integer 0).  The port keeps the integer, because the
/// handler type it would be cast to (`signal_handler_t`) is declared by the src-side
/// syssignal.h mirror, which this mirror cannot name (P-72); the one call the port makes
/// is `signal (SIGPROF, SIG_IGN)` in profiler.c 500.  `sig_t` is Darwin's name for that
/// handler type (`typedef void (*sig_t) (int);`).
pub const SIG_IGN: usize = 1;

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

/* `struct sigaction` and its handler union (signal.h 73-77's `union __sigaction_u`, whose
   second member is `void (*) (int, siginfo_t *, void *)`; `siginfo_t` is not a type this
   target otherwise names, so its pointer is carried as `*mut c_void`, P-72 rule 3).
   The layout is the frozen target's, measured with the frozen configuration:

       $ printf '#include <stddef.h>\n#include <signal.h>\n#include <stdio.h>\nint main (void)\n{ printf ("%%zu %%zu %%zu %%zu\\n", sizeof (struct sigaction),\n    offsetof (struct sigaction, sa_mask), offsetof (struct sigaction, sa_flags),\n    sizeof (((struct sigaction *) 0)->__sigaction_u)); return 0; }\n' > /tmp/sa.c
       $ clang -std=gnu23 -o /tmp/sa /tmp/sa.c && /tmp/sa
       16 8 12 8

   i.e. the handler union is 8 bytes at 0, `sa_mask` is the 4-byte `sigset_t` at 8 and
   `sa_flags` is an `int` at 12.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub union __sigaction_u
{
  pub __sa_handler: Option<unsafe extern "C" fn (c_int)>,
  /// C: `void (*__sa_sigaction) (int, siginfo_t *, void *);`
  pub __sa_sigaction: Option<unsafe extern "C" fn (c_int, *mut c_void, *mut c_void)>,
}

/* struct sigaction
   {
     union __sigaction_u __sigaction_u;
     sigset_t sa_mask;
     int sa_flags;
   };  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigaction
{
  pub __sigaction_u: __sigaction_u,
  pub sa_mask: sigset_t,
  pub sa_flags: c_int,
}

const _: () = assert! (std::mem::size_of::<sigaction> () == 16);
const _: () = assert! (std::mem::offset_of! (sigaction, sa_mask) == 8);
const _: () = assert! (std::mem::offset_of! (sigaction, sa_flags) == 12);
const _: () = assert! (std::mem::size_of::<__sigaction_u> () == 8);

/* The `sigset_t` operations, in a uniquely-named module + glob import (the stdio.rs shape):
   a unit that defines one of them itself (none does yet, but lib/pthread_sigmask.c wraps
   `pthread_sigmask`) then shadows the glob instead of clashing with it (PORTING.md 0.1-10).
   C declares `pthread_sigmask` only when the platform has not got it; on this target it is
   the system entry, so the declaration is present for the units that wrap it.  */
pub mod signal_decls {
  use super::*;

  /* `sigemptyset` and `sigaddset` are *macros* on this target, not entries: the frozen
     atimer.o imports `_pthread_sigmask`/`_sigaction`/`_raise` but has no `_sigemptyset`,
     and <signal.h> 125-129 defines them as `(*(set) = 0, 0)` / `(*(set) |= __sigbits
     (signo), 0)`.  The mirror therefore carries each macro's body as an inline function
     instead of a declaration: the call site keeps C's spelling and the port emits no
     entry the reference has not got.  `__sigbits` is the header's own
     `__header_always_inline` helper (signal.h 118-124).  */
  /// C: `#define __sigbits(signo) (1U << ((signo) - 1))`, 0 for an out-of-range signal
  #[inline(always)]
  pub unsafe fn __sigbits (signo: c_int) -> c_uint
  {
    if signo == 0 || signo > NSIG
      { 0 }
    else
      { 1u32 << (signo - 1) }
  }

  /// C: `#define sigemptyset(set) (*(set) = 0, 0)`
  #[inline(always)]
  pub unsafe fn sigemptyset (set: *mut sigset_t) -> c_int
  {
    *set = sigset_t { __val: 0 };
    0
  }

  /// C: `#define sigaddset(set, signo) (*(set) |= __sigbits (signo), 0)`
  #[inline(always)]
  pub unsafe fn sigaddset (set: *mut sigset_t, signo: c_int) -> c_int
  {
    (*set).__val |= __sigbits (signo);
    0
  }

  unsafe extern "C" {
    /* int sigprocmask (int, const sigset_t *, sigset_t *);  */
    pub fn sigprocmask (how: c_int, set: *const sigset_t, oset: *mut sigset_t) -> c_int;
    /* int pthread_sigmask (int, const sigset_t *, sigset_t *);  */
    pub fn pthread_sigmask (how: c_int, set: *const sigset_t, oset: *mut sigset_t) -> c_int;
    /* int raise (int);  -- syssignal.h 80's `emacs_raise` is `raise (sig)` on this target.  */
    pub fn raise (sig: c_int) -> c_int;
    /* int sigaction (int, const struct sigaction *, struct sigaction *);  */
    pub fn sigaction (signum: c_int, act: *const sigaction, oact: *mut sigaction) -> c_int;
    /* void (*signal (int, void (*) (int))) (int);  -- Darwin spells the handler
       `sig_t`; the port passes the handler word through as `usize` so that `SIG_IGN`
       (the integer 1) reaches it unchanged, the only call profiler.c makes.  */
    pub fn signal (signum: c_int, handler: usize) -> usize;
  }
}
use signal_decls::*;
