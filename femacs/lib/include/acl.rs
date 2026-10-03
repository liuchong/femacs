// femacs: mirror of lib/acl.h @ fd7afced403 (GNU Emacs 32.0.50); PORTING.md 0.1.
//
// The header is two flag constants, `struct aclinfo` (whose union holds either an errno or a
// 152-byte char array) and the declarations of the ACL entry points.  HAVE_LINUX_XATTR_H is
// undefined on this target, so C's `#if HAVE_LINUX_XATTR_H && HAVE_LISTXATTR` arms are dead and
// the port keeps the two macros those arms give instead (`aclinfo_has_xattr` = false,
// `aclinfo_free` = nothing), exactly as C's `#else` does.

include!("sys_types.in.rs");   // <sys/types.h> (ssize_t)
include!("sys_stat.in.rs");    // <sys/stat.h> (mode_t, struct stat)

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const _GL_ACL_H: c_int = 1;

/* C 36-46's `enum { ACL_GET_SCONTEXT = 0x10000, ACL_SYMLINK_FOLLOW = 0x20000 };`.  */
pub const ACL_GET_SCONTEXT: c_int = 0x10000;
pub const ACL_SYMLINK_FOLLOW: c_int = 0x20000;

/// Information about an ACL.  The layout is C's: `char *buf`, `ssize_t size`, `char *scontext`,
/// `int scontext_err` and the union `u` (an `int` errno or a `char[152]` buffer) - measured
/// with clang: sizeof 184, buf@0, size@8, scontext@16, scontext_err@24, u@28.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct aclinfo
{
  pub buf: *mut c_char,
  pub size: c_long,	// C's `ssize_t`, which is `long` on this target
  pub scontext: *mut c_char,
  pub scontext_err: c_int,
  pub u: aclinfo_u,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union aclinfo_u
{
  pub err: c_int,
  pub _gl_acl_ch: [c_char; 152],
}

const _: () = assert! (std::mem::size_of::<aclinfo> () == 184);
const _: () = assert! (std::mem::offset_of! (aclinfo, u) == 28);

/* C 99-104's `#else` arm (live here): the two macros become the constants/helpers the C's own
   `#else` defines.  */
/* C 99-104's `#else` arm (live here): `# define aclinfo_has_xattr(ai, xattr) false`.  The
   arguments are not evaluated, exactly as in C's macro.  */
macro_rules! aclinfo_has_xattr { ($ai:expr, $xattr:expr) => { false }; }

pub mod acl_decls {
  use super::*;

  unsafe extern "C" {
    pub fn acl_errno_valid (err: c_int) -> bool;
    pub fn file_has_acl (name: *const c_char, sb: *const stat) -> c_int;
    pub fn file_has_aclinfo (name: *const c_char, ai: *mut aclinfo, flags: c_int) -> c_int;
    pub fn fdfile_has_aclinfo (fd: c_int, name: *const c_char, ai: *mut aclinfo, flags: c_int)
      -> c_int;
    pub fn qset_acl (name: *const c_char, desc: c_int, mode: mode_t) -> c_int;
    pub fn xset_acl (name: *const c_char, desc: c_int, mode: mode_t) -> c_int;
    pub fn set_acl (name: *const c_char, desc: c_int, mode: mode_t) -> c_int;
    pub fn qcopy_acl (src_name: *const c_char, src_desc: c_int, dst_name: *const c_char,
                      dst_desc: c_int, mode: mode_t) -> c_int;
    pub fn xcopy_acl (src_name: *const c_char, src_desc: c_int, dst_name: *const c_char,
                      dst_desc: c_int, mode: mode_t) -> c_int;
    pub fn copy_acl (src_name: *const c_char, src_desc: c_int, dst_name: *const c_char,
                     dst_desc: c_int, mode: mode_t) -> c_int;
    pub fn chmod_or_fchmod (name: *const c_char, desc: c_int, mode: mode_t) -> c_int;
  }
}
use acl_decls::*;

/* C 105-112's `# define aclinfo_free(ai) ((void) 0)` (the live arm); `0` is the value C's
   cast reads, which the ledger compares.  */
macro_rules! aclinfo_free { ($ai:expr) => { { let _ = ($ai); 0 } }; }
/* C 113-118's `# define aclinfo_scontext_free(s) ((void) 0)` (the live arm).  */
macro_rules! aclinfo_scontext_free { ($s:expr) => { { let _ = ($s); 0 } }; }
