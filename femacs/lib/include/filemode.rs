// femacs: mirror of lib/filemode.h @ fd7afced403 ; PORTING.md 0.1.
//
// The reference header declares `strmode` only when the system has not got it
// (`#if !HAVE_DECL_STRMODE`), and HAVE_DECL_STRMODE is 1 on this target: the live declaration
// is the system <string.h>'s, which `femacs/lib/filemode.rs` calls through its `system`
// module.  The `#if` arm below is kept as the cfg'd declaration the port's ledger wants (the
// filemode.c dead-text arm defines that same function under the same feature, P-51).

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const FILEMODE_H_: bool = true;

include!("sys_types.in.rs");   // <sys/types.h> (mode_t)
include!("sys_stat.in.rs");    // <sys/stat.h> (struct stat)

/// C: extern void strmode (mode_t mode, char *str)
#[cfg (feature = "no_decl_strmode")]
unsafe extern "C" {
  pub fn strmode (mode: mode_t, str_: *mut c_char);
}

/// C: extern void filemodestring (struct stat const *statp, char *str)
/* Whole-import module: `femacs/lib/filemode.rs` defines this name (PORTING.md 0.1-10).  */
pub mod filemode_decls {
  use super::*;

  unsafe extern "C" {
    pub fn filemodestring (statp: *const stat, str_: *mut c_char);
  }
}
use filemode_decls::*;
