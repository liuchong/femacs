// femacs: mirror of nt/inc/pwd.h @ fd7afced403.
//
// The Windows port's own `<pwd.h>` (NT has not got one): the `passwd` record C reads, the
// two id typedefs `grp.h` borrows, and the four lookups.  The file has no `#if` and is not on
// `docs/baseline/config/cond-eval-verified.txt`, so the extractor resolves nothing in it - the
// whole file is live text.  Nothing in the frozen configuration includes it (no `nt/` translation
// unit is compiled - docs/baseline/frozen-build-sources.txt's batch-F section - and no `src/`
// unit includes a `nt/inc/` header), so `xtask layout-check` never compiles this mirror; in a
// MinGW build `src/fileio.c`, `src/dired.c`, `src/filelock.c`, `src/sysdep.c` and `src/w32.c`
// are among the units whose `#include <pwd.h>` resolves here.

// C 1: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _PWD_H_: bool = true;

/// C 7-16: `struct passwd` - C's eight members in order.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct passwd
{
  pub pw_name: *mut c_char,
  pub pw_passwd: *mut c_char,
  pub pw_uid: c_uint,     /* Vista's TrustedInstaller has a very large RID */
  pub pw_gid: c_uint,
  pub pw_quota: c_int,
  pub pw_gecos: *mut c_char,
  pub pw_dir: *mut c_char,
  pub pw_shell: *mut c_char,
}

/// C 18: `typedef unsigned uid_t;`
pub type uid_t = c_uint;
/// C 19: `typedef uid_t gid_t;` - `grp.h`'s `/* gid_t defined here */` points at this line.
pub type gid_t = uid_t;

pub mod pwd_decls {
  use super::*;

  unsafe extern "C" {
    /// C 21: `extern unsigned getuid (void);`
    pub fn getuid () -> c_uint;
    /// C 22: `extern unsigned geteuid (void);`
    pub fn geteuid () -> c_uint;
    /// C 24: `extern struct passwd * getpwnam (char *);`
    pub fn getpwnam (name: *mut c_char) -> *mut passwd;
    /// C 25: `extern struct passwd * getpwuid (unsigned);`
    pub fn getpwuid (uid: c_uint) -> *mut passwd;
  }
}
use pwd_decls::*;
