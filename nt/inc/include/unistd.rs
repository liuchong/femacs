// femacs: mirror of nt/inc/unistd.h @ fd7afced403.
//
// The Windows port's fake `<unistd.h>`: `config.h` already carries most of what the name means on
// NT, so this is the prototypes of the functions C emulates, the `AT_*`/`O_*` names that
// normally live in `<fcntl.h>`, and `unsetenv` (normally `<stdlib.h>`).  The file is not on
// `docs/baseline/config/cond-eval-verified.txt` and has no `#if`, so the whole file is live text.
// Nothing in the frozen configuration includes it (no `nt/` translation unit is compiled -
// docs/baseline/frozen-build-sources.txt's batch-F section - and no `src/` unit includes a
// `nt/inc/` header), so `xtask layout-check` never compiles this mirror; in a MinGW build
// `src/buffer.c`, `src/dispnew.c`, `src/pdumper.c`, `src/doprnt.c` and `src/pgtkterm.c` are among
// the many units whose `#include <unistd.h>` resolves here.
//
// `environ`: C's comment says it is deliberately not declared here, because every unit that
// includes `<unistd.h>` also includes `<stdlib.h>`, where Microsoft's platform declares it.  The
// mirror keeps that decision.
//
// Names with no mirror: `ssize_t` and `size_t` are not in the frozen target's `<sys/types.h>`
// mirror (the port's units take them from `<unistd.h>`'s own, `lib/include/unistd.in.rs`, which
// describes darwin's header rather than MinGW's and would clash with this file's `AT_*`/`O_*`
// names if it were pulled in), so they stay spelled the way C spells them and are named from the
// including unit, as `nt/inc/sys/include/acl.rs` does for `ssize_t`.

include!("../../../lib/include/sys_types.in.rs");   // C 6: `<sys/types.h>` (pid_t)
include!("pwd.rs");                                 // C 7: `<pwd.h>`

// C 3: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _UNISTD_H: bool = true;

// C 29-32: the `*at` selectors, `/* Use values compatible with gnulib, as there's no reason to
// differ.  */`
pub const AT_FDCWD: c_int = -3041965;
pub const AT_EACCESS: c_int = 4;
pub const AT_SYMLINK_NOFOLLOW: c_int = 4096;
pub const AT_EMPTY_PATH: c_int = 8192;

// C 34-36: `/* These are normally on fcntl.h, but we don't override that header.  */`
pub const O_IGNORE_CTTY: c_int = 0;
pub const O_NOCTTY: c_int = 0;
pub const O_NOFOLLOW: c_int = 0;

pub mod unistd_decls {
  use super::*;

  unsafe extern "C" {
    /// C 17: `extern ssize_t readlink (const char *, char *, size_t);`
    pub fn readlink (name: *const c_char, buffer: *mut c_char, size: size_t) -> ssize_t;
    /// C 18: `extern ssize_t readlinkat (int, const char *, char *, size_t);`
    pub fn readlinkat (fd: c_int, name: *const c_char, buffer: *mut c_char,
                       size: size_t) -> ssize_t;
    /// C 19: `extern int symlink (char const *, char const *);`
    pub fn symlink (target: *const c_char, linkpath: *const c_char) -> c_int;
    /// C 20: `extern int setpgid (pid_t, pid_t);`
    pub fn setpgid (pid: pid_t, pgid: pid_t) -> c_int;
    /// C 21: `extern pid_t getpgrp (void);`
    pub fn getpgrp () -> pid_t;
    /// C 22: `extern pid_t setsid (void);`
    pub fn setsid () -> pid_t;
    /// C 23: `extern pid_t tcgetpgrp (int);`
    pub fn tcgetpgrp (fd: c_int) -> pid_t;
    /// C 25: `extern int faccessat (int, char const *, int, int);`
    pub fn faccessat (fd: c_int, name: *const c_char, mode: c_int, flags: c_int) -> c_int;
    /// C 39: `extern int unsetenv (const char *);`
    pub fn unsetenv (name: *const c_char) -> c_int;
  }
}
use unistd_decls::*;
