// femacs: mirror of lib/sig2str.h @ fd7afced403 (GNU Emacs 32.0.50); PORTING.md 0.1.
//
// The reference header is the license, the include guard and one constant: the upper bound on
// signal numbers, chosen from the system's own macros.  This target defines NSIG (through
// __DARWIN_NSIG, 32) and neither _sys_nsig nor _SIG_MAXSIG, so C's third arm is the live one
// and the port keeps `NSIG - 1`; the other arms are recorded in the notes.

include!("signal.rs");   // <signal.h> (NSIG)

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const SIG2STR_H: bool = true;

/* C 24-35's `#if defined _sys_nsig … #elif defined _SIG_MAXSIG … #elif defined NSIG
   # define SIGNUM_BOUND (NSIG - 1) #else # define SIGNUM_BOUND 64 #endif`: the NSIG arm is
   the live one on this target (measured: NSIG is `__DARWIN_NSIG` = 32, so the bound is 31).  */
pub const SIGNUM_BOUND: c_int = NSIG - 1;
