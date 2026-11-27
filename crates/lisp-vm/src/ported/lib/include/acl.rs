// femacs: mirror of lib/acl.h @ fd7afced403 ; PORTING.md 0.1.
//
// The header is two flag constants, `struct aclinfo` (whose union holds either an errno or a
// 152-byte char array) and the declarations of the ACL entry points.  HAVE_LINUX_XATTR_H is
// undefined on this target, so C's `#if HAVE_LINUX_XATTR_H && HAVE_LISTXATTR` arms are dead and
// the port keeps the two macros those arms give instead (`aclinfo_has_xattr` = false,
// `aclinfo_free` = nothing), exactly as C's `#else` does.

// [U1-MODULE-MIRROR]
use super::*;
// [U1] sys_types.in.rs imported via super
// [U1] sys_stat.in.rs imported via super

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

const _: () = core::assert! (std::mem::size_of::<aclinfo> () == 184);
const _: () = core::assert! (std::mem::offset_of! (aclinfo, u) == 28);

/* C 88-104: `#if HAVE_LINUX_XATTR_H && HAVE_LISTXATTR` declares three `aclinfo_*` *functions*
   and the `#else` arm defines the same names as macros.  C can have both texts because only one
   arm is compiled; in Rust the macro is an item of its own, so **the `#else` side has to say so**:
   an ungated macro answers the two live call sites (`lib/file-has-acl.c` 152 and 984) even in an
   arm where the unit defines and calls the functions - the silent rewrite the
   `lib/file-has-acl.rs` lane reported (status 3.510).  The feature is the one the unit gates the
   function definitions with (`use_linux_xattr`), and it names the same condition C's `#if` does.  */
/* C 99-104's `#else` arm: `# define aclinfo_has_xattr(ai, xattr) false`.  The arguments are not
   evaluated, exactly as in C's macro.  */
#[cfg (not (feature = "use_linux_xattr"))]
macro_rules! aclinfo_has_xattr { ($ai:expr, $xattr:expr) => { false }; }

#[inline(always)]
pub const fn acl_errno_valid(errnum: c_int) -> bool {
  !matches!(errnum, EBUSY | EINVAL | ENOENT | ENOSYS | ENOTSUP | EOPNOTSUPP)
}

pub mod acl_decls {
  use super::*;

  unsafe extern "C" {
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
pub use self::acl_decls::*;

/* C 99-104's `#else` arm: `# define aclinfo_free(ai) ((void) 0)`; `0` is the value C's cast
   reads, which the ledger compares.  Gated like the one above, and for the same reason.  */
#[cfg (not (feature = "use_linux_xattr"))]
macro_rules! aclinfo_free { ($ai:expr) => { { let _ = ($ai); 0 } }; }
/* C 106-112: `#if (HAVE_LINUX_XATTR_H && HAVE_LISTXATTR && (HAVE_SMACK ||
   USE_SELINUX_SELINUX_H))` declares `aclinfo_scontext_free` as a function, so the macro is the
   `#else` arm of *that* condition - narrower than `use_linux_xattr` alone.  */
#[cfg (not (all (feature = "use_linux_xattr",
                 any (feature = "have_smack", feature = "use_selinux_selinux_h"))))]
macro_rules! aclinfo_scontext_free { ($s:expr) => { { let _ = ($s); 0 } }; }
