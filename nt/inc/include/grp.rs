// femacs: mirror of nt/inc/grp.h @ fd7afced403.
//
// The Windows port's own `<grp.h>`: NT has no group database, so this shim declares the one
// `struct group` field the port reads (`gr_name`, as C's own comment says) plus the two `get*id`
// calls.  It is not on `docs/baseline/config/cond-eval-verified.txt` and has no `#if` in it, so
// the whole file is live text.  Nothing in the frozen configuration includes it (no `nt/`
// translation unit is compiled - docs/baseline/frozen-build-sources.txt's batch-F section - and
// no `src/` unit includes a `nt/inc/` header), so `xtask layout-check` never compiles this
// mirror; in a MinGW build `src/dired.c`, `src/sysdep.c`, `src/editfns.c` and `src/w32.c` are
// among the units whose `#include <grp.h>` resolves here.

include!("pwd.rs");   // C 23: `<pwd.h> /* gid_t defined here */`

// C 20: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _GRP_H: bool = true;

/// C 26-29: `struct group` - `/* Emacs uses only gr_name */`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct group
{
  pub gr_name: *mut c_char,   /* group name */
  pub gr_gid: gid_t,          /* group numerical ID */
}

pub mod grp_decls {
  use super::*;

  unsafe extern "C" {
    /// C 31: `extern unsigned getgid (void);`
    pub fn getgid () -> c_uint;
    /// C 32: `extern unsigned getegid (void);`
    pub fn getegid () -> c_uint;
    /// C 34: `extern struct group *getgrgid(gid_t);`
    pub fn getgrgid (gid: gid_t) -> *mut group;
  }
}
use grp_decls::*;
