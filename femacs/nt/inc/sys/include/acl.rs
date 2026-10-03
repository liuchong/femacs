// femacs: mirror of nt/inc/sys/acl.h @ fd7afced403.
//
// The Windows port's emulation of POSIX ACLs.  Nothing in the frozen configuration includes it
// (no `nt/` unit is compiled), so the mirror is never compiled by the layout probe.
//
// `PSECURITY_DESCRIPTOR` is `<windows.h>`'s `void *` typedef; it cannot be measured on this
// target, and it is only ever used through a pointer, so the mirror carries it as
// `*mut c_void` with the C spelling above it (PORTING.md P-72 (3)).

include!("../../../../lib/include/sys_types.in.rs");   // C 4: `<sys/types.h>` (pid_t)
// `ssize_t` is not in that mirror - the port's units take it from `<unistd.h>`'s
// (`lib/include/unistd.in.rs`) - so `acl_to_text` names it the way this mirror's
// other system types are named: from the including unit (spikes/f_inc_probe.rs
// supplies it, docs/status.md 3.102).

// C 4: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const ACL_H: bool = true;

/// C 6: `#define NOMINMAX 1`  (`/* don't define min and max */`)
pub const NOMINMAX: c_int = 1;

/// `<windows.h>`: `typedef void *PSECURITY_DESCRIPTOR;`
pub type PSECURITY_DESCRIPTOR = *mut c_void;

/// C 9: `typedef PSECURITY_DESCRIPTOR acl_t;`
pub type acl_t = PSECURITY_DESCRIPTOR;
/// C 10: `typedef unsigned acl_type_t;`
pub type acl_type_t = c_uint;

/// C 13: `#define ACL_TYPE_ACCESS 0`
pub const ACL_TYPE_ACCESS: c_int = 0;
/// C 14: `#define ACL_TYPE_DEFAULT 1`
pub const ACL_TYPE_DEFAULT: c_int = 1;

/// C 16: `typedef unsigned acl_perm_t;`
pub type acl_perm_t = c_uint;

pub mod acl_decls {
  use super::*;

  unsafe extern "C" {
    /// C 18: `extern int acl_valid (acl_t);`
    pub fn acl_valid (acl: acl_t) -> c_int;
    /// C 19: `extern acl_t acl_get_file (const char *, acl_type_t);`
    pub fn acl_get_file (path: *const c_char, type_: acl_type_t) -> acl_t;
    /// C 20: `extern int acl_set_file (const char *, acl_type_t, acl_t);`
    pub fn acl_set_file (path: *const c_char, type_: acl_type_t, acl: acl_t) -> c_int;
    /// C 21: `extern char * acl_to_text (acl_t, ssize_t *);`
    pub fn acl_to_text (acl: acl_t, len: *mut ssize_t) -> *mut c_char;
    /// C 22: `extern acl_t acl_from_text (const char *);`
    pub fn acl_from_text (text: *const c_char) -> acl_t;
    /// C 23: `extern int acl_free (void *);`
    pub fn acl_free (obj: *mut c_void) -> c_int;
  }
}
use acl_decls::*;
