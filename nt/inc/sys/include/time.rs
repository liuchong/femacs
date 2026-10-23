// femacs: mirror of nt/inc/sys/time.h @ fd7afced403.
//
// The Windows port's shim: it pulls the real `<sys/time.h>` in first (`#include_next`) and then
// adds the two interval timers NT lacks.
//
// **The `#include_next` target is MinGW's header, not the frozen target's**, and the two disagree:
// `lib/include/sys_time.rs` (the mirror of the frozen target's `<sys/time.h>`) fixes
// `ITIMER_VIRTUAL` at 1 and `ITIMER_PROF` at 2, while this shim's own text says `ITIMER_PROF` is 1
// and defines `struct itimerval` and `setitimer` as well.  Including that mirror therefore gives
// `E0428` four times over and would put the wrong value on `ITIMER_PROF`; the shim instead takes
// `struct timeval` from the *including unit*, exactly as C takes it from the system header
// (spikes/f_inc_probe.rs supplies it, and docs/ports/include-time.md records the decision).
//
// The two prototypes are the MSVC CRT's; they are declarations only (the port has no definition
// point for them, and none is needed on this target).

// C 2: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const SYS_TIME_H_INCLUDED: bool = true;

/// C 6: `#define ITIMER_REAL 0`
pub const ITIMER_REAL: c_int = 0;
/// C 7: `#define ITIMER_PROF 1`
pub const ITIMER_PROF: c_int = 1;

/// C 9-13: `struct itimerval` (measured on the Windows CRT: two `struct timeval`s).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct itimerval
{
  pub it_interval: timeval,   /* timer interval */
  pub it_value: timeval,      /* current value */
}

unsafe extern "C" {
  /// C 15: `int getitimer (int, struct itimerval *);`
  pub fn getitimer (which: c_int, value: *mut itimerval) -> c_int;
  /// C 16: `int setitimer (int, struct itimerval *, struct itimerval *);`
  pub fn setitimer (which: c_int, new: *mut itimerval, old: *mut itimerval) -> c_int;
}
