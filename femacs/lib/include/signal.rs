// femacs: the parts of the system <signal.h> the port needs (PORTING.md 0.1, the errno.rs
// shape).  The values were measured from the frozen target's clang
// (`clang -dM -E` on a file that includes <signal.h>); `c_int` comes from the including unit.
// The signals this target does not define (SIGPWR, SIGSTKFLT and the other platform-specific
// ones sigdescr_np.c guards with `#if defined`) are absent on purpose: docs/ports/sigdescr_np.md
// records them.
pub const SIGABRT: c_int = 6;
pub const SIGALRM: c_int = 14;
#[cfg (not (feature = "have_android"))]
pub const SIGBUS: c_int = 10;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const SIGBUS: c_int = 7;
#[cfg (not (feature = "have_android"))]
pub const SIGCHLD: c_int = 20;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const SIGCHLD: c_int = 17;
#[cfg (not (feature = "have_android"))]
pub const SIGCONT: c_int = 19;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const SIGCONT: c_int = 18;
pub const SIGEMT: c_int = 7;
pub const SIGFPE: c_int = 8;
pub const SIGHUP: c_int = 1;
pub const SIGILL: c_int = 4;
pub const SIGINFO: c_int = 29;
pub const SIGINT: c_int = 2;
pub const SIGIOT: c_int = 6;	// = SIGABRT on this target (measured)
#[cfg (not (feature = "have_android"))]
pub const SIGIO: c_int = 23;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const SIGIO: c_int = 29;
pub const SIGKILL: c_int = 9;
pub const SIGPIPE: c_int = 13;
pub const SIGPROF: c_int = 27;
pub const SIGQUIT: c_int = 3;
pub const SIGSEGV: c_int = 11;
#[cfg (not (feature = "have_android"))]
pub const SIGSTOP: c_int = 17;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const SIGSTOP: c_int = 19;
#[cfg (not (feature = "have_android"))]
pub const SIGSYS: c_int = 12;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const SIGSYS: c_int = 31;
pub const SIGTERM: c_int = 15;
pub const SIGTRAP: c_int = 5;
#[cfg (not (feature = "have_android"))]
pub const SIGTSTP: c_int = 18;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const SIGTSTP: c_int = 20;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
#[cfg (not (feature = "have_android"))]
pub const SIGURG: c_int = 16;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const SIGURG: c_int = 23;
#[cfg (not (feature = "have_android"))]
pub const SIGUSR1: c_int = 30;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const SIGUSR1: c_int = 10;
#[cfg (not (feature = "have_android"))]
pub const SIGUSR2: c_int = 31;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const SIGUSR2: c_int = 12;
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

/* <signal.h>'s `sigset_t` has one arm per target.  The runtime arm (`windowsnt`) is MinGW's
   `_sigset_t`, a 32-bit unsigned scalar, and `src/w32proc.c`'s operations on it (`*set = 0`,
   `*set |= 1U << signo`, `sizeof (*set) * CHAR_BIT` against `NSIG`) require exactly that.  */
#[cfg (feature = "windowsnt")]
pub type sigset_t = c_uint;

/* The frozen arm: Darwin's `struct __darwin_sigset_t { __uint32_t __val; }`, the type the three
   entries lib/pthread_sigmask.c and lib/pselect.c reach through <signal.h>; the measurement
   (`printf ("%zu %d %d %d", sizeof (sigset_t), SIG_BLOCK, SIG_UNBLOCK, SIG_SETMASK)`)
   prints `4 1 2 3`.  */
/* bionic's `sigset_t` is an `unsigned long` - 8 bytes on aarch64 - while Darwin's is the
   4-byte `__darwin_sigset_t` the comment above records.  Measured with the same probe
   (`sizeof(sigset_t)` from the NDK's `<signal.h>`).  */
#[cfg (feature = "have_android")]
#[repr(C)]
#[derive (Clone, Copy)]
pub struct sigset_t
{
  pub __val: u64,
}
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<sigset_t> () == 8);
#[cfg (all (not (feature = "windowsnt"), not (feature = "have_android")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigset_t
{
  pub __val: u32,
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<sigset_t> () == 4);

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
/* C's `<signal.h>` declares `siginfo_t`, and the port's record for it is `sysdep.rs` 193's - one
   definition per crate, so this mirror carries the pointee untyped instead (P-72 rule 3, the shape
   `composite` and the `output_data` union use): an opaque second copy here was `E0428 siginfo_t`
   in every unit that defines it (measured on `src/sysdep.c`), and `sysdep.rs` reads the member
   C 1885 reads, so a claim of "no layout" is not available to it either.  */

/* bionic's `struct sigaction` is flat and 32 bytes (`sa_sigaction` and `sa_handler` are the two
   names of one anonymous union member, not a field of `__sigaction_u`), measured with
   `_Static_assert` against the NDK's own `<signal.h>` (docs/baseline/android-layout-probe.md,
   P-237).  The darwin arm keeps its 16-byte shape.

   The port names that anonymous union `__sigaction_u` with darwin's two member names, so C's own
   two spellings (`sa_handler` 1710/2014/2033/2037 and `sa_sigaction` 1837/1961 in `sysdep.c`)
   reach the same slot through one field path on both targets: Rust has no anonymous-union alias,
   and the alternative was a `transmute` at each of the twelve sites that read the slot
   (`src/sysdep.c` six, `src/process.c` six).  Offset, size and alignment are unchanged - the
   union is 8 bytes at 0 on both arms, which the two assertions below and the one at 148 hold.  */
/* bionic's LP64 arm really does put the flags first: the NDK's `bits/signal_types.h` 103-110
   defines `__SIGACTION_BODY` as `int sa_flags; union { ... }; sigset_t sa_mask;
   void (*sa_restorer) (void);` under `#if defined (__LP64__)`, and 155-163 (the order this mirror
   used to carry) is the *ILP32* branch.  Measured against the NDK for API 21, 28 and 36:
   `sizeof` 32, `sa_flags`@0, the handler union@8, `sa_mask`@16, `sa_restorer`@24 (status 4.831).
   The old order wrote the handler over `sa_flags` and `sigfillset`'s mask over the handler slot,
   so every `sigaction` call installed a wild handler pointer on Android.  */
#[cfg (feature = "have_android")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigaction
{
  pub sa_flags: c_int,
  pub __sigaction_u: __sigaction_u,
  pub sa_mask: sigset_t,
  pub sa_restorer: Option<unsafe extern "C" fn ()>,
}
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<sigaction> () == 32);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (sigaction, sa_flags) == 0);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (sigaction, __sigaction_u) == 8);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (sigaction, sa_mask) == 16);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (sigaction, sa_restorer) == 24);
#[cfg (not (feature = "have_android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigaction
{
  pub __sigaction_u: __sigaction_u,
  pub sa_mask: sigset_t,
  pub sa_flags: c_int,
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<sigaction> () == 16);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (sigaction, sa_mask) == 8);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (sigaction, sa_flags) == 12);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<__sigaction_u> () == 8);

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

  /* C 125-129's two macros have one body per `sigset_t` arm, because the *type* does: the
     Darwin record takes the struct literal, MinGW's `_sigset_t` scalar takes a plain `0` and a
     plain `|=` (the file note at C 52-56 records `src/w32proc.c`'s `*set = 0` /
     `*set |= 1U << signo` for that arm).  Ungated, these two bodies spelled `__val` in *both*
     arms, and the `w32font` arm read `E0610`/`E0071` on it - `u32` has no fields (status 3.541).
     No MinGW header exists on this machine, so the scalar arm's bodies are the mirror's own
     recorded evidence and carry that mark.  */
  /// C: `#define sigemptyset(set) (*(set) = 0, 0)` (Darwin arm)
  #[cfg (not (feature = "windowsnt"))]
  #[inline(always)]
  pub unsafe fn sigemptyset (set: *mut sigset_t) -> c_int
  {
    *set = sigset_t { __val: 0 };
    0
  }

  /// C: `#define sigemptyset(set) (*(set) = 0, 0)` (MinGW arm; **UNVERIFIED** reading)
  #[cfg (feature = "windowsnt")]
  #[inline(always)]
  pub unsafe fn sigemptyset (set: *mut sigset_t) -> c_int
  {
    *set = 0;
    0
  }

  /// C: `#define sigaddset(set, signo) (*(set) |= __sigbits (signo), 0)` (Darwin arm)
  ///
  /// One arm per target, as C has one macro per platform: `__val` is 4 bytes on darwin and 8 on
  /// bionic, and C's `|=` widens automatically where Rust needs the width spelled out
  /// (`u64 |= u32` fails; status 4.622).
  #[cfg (all (not (feature = "windowsnt"), not (feature = "have_android")))]
  #[inline(always)]
  pub unsafe fn sigaddset (set: *mut sigset_t, signo: c_int) -> c_int
  {
    (*set).__val |= __sigbits (signo);
    0
  }

  /// C: the same macro on bionic, where `sigset_t` is an `unsigned long`.
  #[cfg (feature = "have_android")]
  #[inline(always)]
  pub unsafe fn sigaddset (set: *mut sigset_t, signo: c_int) -> c_int
  {
    (*set).__val |= __sigbits (signo) as u64;
    0
  }

  /// C: the MinGW arm's `*set |= 1U << signo` (`src/w32proc.c`'s reading; **UNVERIFIED**)
  #[cfg (feature = "windowsnt")]
  #[inline(always)]
  pub unsafe fn sigaddset (set: *mut sigset_t, signo: c_int) -> c_int
  {
    *set |= 1u32 << (signo as u32);
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
    #[cfg (not (feature = "windowsnt"))]
    pub fn signal (signum: c_int, handler: usize) -> usize;

    /* The same entry on the runtime arm (`windowsnt`): MinGW's <signal.h> declares the handler
       as a function pointer, and `src/w32proc.rs` passes a recorded handler of that shape.  The
       type is written out structurally rather than as `nt/inc/ms-w32.h` 133's `signal_handler`
       alias, because this mirror is included by units that never see that header and a mirror
       may not name a type its includers may lack (PORTING.md P-87 ③).  */
    #[cfg (feature = "windowsnt")]
    pub fn signal (signum: c_int, handler: Option<unsafe extern "C" fn (c_int)>)
                 -> Option<unsafe extern "C" fn (c_int)>;
  }
}
use signal_decls::*;
