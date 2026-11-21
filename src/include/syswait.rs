// femacs: mirror of src/syswait.h @ fd7afced403.
//
// Live text on the frozen target: the include guard and two extern declarations.  Everything
// else is dead because `HAVE_SYS_WAIT_H` is 1 (macros.tsv), so C 30 includes the system
// `<sys/wait.h>`, which defines the POSIX status macros - every `#ifndef` fallback below it
// (`WCONTINUED` 8, `WCOREDUMP`'s `(status) & 0x80`, `WEXITSTATUS`, `WIFEXITED`, `WIFSTOPPED`,
// `WIFSIGNALED`, `WSTOPSIG`, `WTERMSIG`) is therefore not compiled; the extractor's live
// inventory is the guard alone, which is why this mirror carries only the guard and the two
// declarations (PORTING.md P-08: recorded, not transcribed - the dead arms are listed in
// docs/ports/include-syswait.md).
//
// C 24: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_SYSWAIT_H: bool = true;

// Both functions are defined in src/sysdep.c, which is ported (`src/sysdep.rs` owns
// them), so this is a declaration-only mirror (PORTING.md 0.1-3, the P-21 module shape).
pub mod syswait_decls {
  use super::*;

  unsafe extern "C-unwind" {
    /* C 59: `extern bool wait_for_termination (pid_t, int *, bool);`  */
    pub fn wait_for_termination (pid: pid_t, status: *mut c_int, flush: bool) -> bool;
    /* C 60: `extern pid_t child_status_changed (pid_t, int *, int);`  */
    pub fn child_status_changed (pid: pid_t, status: *mut c_int, flags: c_int) -> pid_t;
  }
}
pub use self::syswait_decls::*;
