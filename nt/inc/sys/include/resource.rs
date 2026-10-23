// femacs: mirror of nt/inc/sys/resource.h @ fd7afced403.
//
// A limited emulation of `<sys/resource.h>` for the Windows build (only `RLIMIT_STACK` and
// `RLIMIT_NOFILE`).  Nothing in the frozen configuration includes it, so the mirror is never
// compiled by the layout probe; the enum keeps C's shape (PORTING.md 1.3: the C type plus one
// constant per member) and the two prototypes stay declarations.

// C 17-19's `<sys/types.h>` chain gives this header C's own names; the port names the ones
// it uses (PORTING.md P-70's rule for a mirror whose header is not carried).
use core::ffi::*;

// C 21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const INC_SYS_RESOURCE_H_: bool = true;

/// C 24-33: `enum rlimit_resource` - the port spells the C enum as `c_int` with one constant per
/// member (PORTING.md 1.3, the callint.rs shape), keeping the `#define RLIMIT_STACK RLIMIT_STACK`
/// spellings C adds so the names work in `#if` too.
pub type rlimit_resource = c_int;
/// C 26: `RLIMIT_STACK = 0`
pub const RLIMIT_STACK: rlimit_resource = 0;
/// C 29: `RLIMIT_NOFILE = 1`
pub const RLIMIT_NOFILE: rlimit_resource = 1;
/// C 32: `RLIMIT_NLIMITS` (the last member: 2)
pub const RLIMIT_NLIMITS: rlimit_resource = 2;

/// C 37: `typedef enum rlimit_resource rlimit_resource_t;`
pub type rlimit_resource_t = rlimit_resource;

/// C 41: `typedef long long rlim_t;`
pub type rlim_t = c_longlong;

/// C 43: `#define RLIMIT_INFINITY ((rlim_t) -1)`
pub const RLIMIT_INFINITY: rlim_t = -1i64 as rlim_t;

/// C 45-48: `struct rlimit` - both fields are `rlim_t`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct rlimit
{
  pub rlim_cur: rlim_t,   /* current soft limit */
  pub rlim_max: rlim_t,   /* hard limit */
}

/* C 50-51's two entry points `src/w32heap.c` **defines** (C 483-554), so a copy at this mirror's
   top level is `E0428` in that unit - measured: two of them on `src/w32heap.rs`'s flip
   (status 3.504).  They sit in a module the mirror glob-imports (P-39), whose items an including
   unit's own definitions shadow.  */
pub mod resource_decls {
  use super::*;

  unsafe extern "C" {
    /// C 50: `extern int getrlimit (rlimit_resource_t, struct rlimit *);`
    pub fn getrlimit (resource: rlimit_resource_t, rlp: *mut rlimit) -> c_int;
    /// C 51: `extern int setrlimit (rlimit_resource_t, const struct rlimit *);`
    pub fn setrlimit (resource: rlimit_resource_t, rlp: *const rlimit) -> c_int;
  }
}
use resource_decls::*;
