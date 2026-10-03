// femacs: mirror of nt/inc/sys/wait.h @ fd7afced403.
//
// A limited emulation of `<sys/wait.h>` for the Windows build; its own comment says the `WIF*`
// macros come from `src/syswait.h` (whose mirror this port carries separately).  Nothing in the
// frozen configuration includes it, so it is never compiled by the layout probe.

// C 21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const INC_SYS_WAIT_H_: bool = true;

/// C 23: `#define WNOHANG 1`
pub const WNOHANG: c_int = 1;
/// C 24: `#define WUNTRACED 2`
pub const WUNTRACED: c_int = 2;
/// C 25: `#define WSTOPPED 2  /* same as WUNTRACED */`
pub const WSTOPPED: c_int = 2;
/// C 26: `#define WEXITED 4`
pub const WEXITED: c_int = 4;
/// C 27: `#define WCONTINUED 8`
pub const WCONTINUED: c_int = 8;

unsafe extern "C" {
  /// C 31: `extern pid_t waitpid (pid_t, int *, int);`
  pub fn waitpid (pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
}
