// femacs: port of lib/file-has-acl.c @ fd7afced403 (gnulib).
//
// The frozen configuration resolves all 32 of this file's conditions
// (`tools/check-cond-eval.py lib/file-has-acl.c`: 0 unknown, 0 disagreements).  HAVE_LINUX_XATTR_H
// and HAVE_LISTXATTR are undefined, so `USE_LINUX_XATTR` is false: C 48-509's `#if
// USE_LINUX_XATTR` block (the `<selinux/selinux.h>`/`<sys/xattr.h>`/`<linux/xattr.h>` includes,
// the `XATTR_NAME_*` defaults and the `get_aclinfo`/`aclinfo_may_indicate_xattr`/`listxattr`
// path of `fdfile_has_aclinfo`) is dead, and USE_ACL is 0, so the `#if USE_ACL` chain inside
// the `#else` arm (C 519-964: the `HAVE_ACL_GET_FILE` arms, the `HAVE_FACL && GETACL` arm with
// its `ACE_GETACL` half, the HP-UX, AIX and NonStop arms) is dead as well; C 384-424's
// `#if (!USE_LINUX_XATTR && USE_ACL && HAVE_ACL_GET_FILE …)` helper block is dead for the same
// reason.  Live: C 37-40's assertion and context string, C 42-46's `USE_LINUX_XATTR`,
// `file_has_aclinfo` (C 438-443), `fdfile_has_aclinfo`'s `#else` arm (C 513-517 and C 967's
// `return 0;`) and `file_has_acl` (C 976-986).
//
// `MAYBE_UNUSED` (C 459-463) is an attribute from `attribute.h`: the port keeps the
// declaration and the variable, and the token gate is told so by a macro-body-notes row.
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort,
               c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");
include!("include/limits.in.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/dirent.rs");         /* C 30: `<dirent.h>` (IFTODT, _GL_DT_NOTDIR).  */
include!("include/acl.rs");            /* C 28: `#include "acl.h"`.  */
include!("include/acl-internal.rs");   /* C 33: `#include "acl-internal.h"`.  */
/* C 34's `"attribute.h"` and C 35's `"minmax.h"` are only used by the dead arms above.  */

/// PORTING.md 2.5: C's `static_assert` is `const _: () = assert! (cond);`.
macro_rules! static_assert {
  ($cond:expr) => { const _: () = assert! ($cond); };
}

/* Check the assumption that UCHAR_MAX < INT_MAX.  */
static_assert! ((ACL_SYMLINK_FOLLOW & ! (((-1i32) as c_uchar) as c_int)) != 0);

/* C 40: `static char const UNKNOWN_SECURITY_CONTEXT[] = "?";`.  */
static UNKNOWN_SECURITY_CONTEXT: [c_char; 2] = [b'?' as c_char, 0];

/* C 42-46: `#if HAVE_LINUX_XATTR_H && HAVE_LISTXATTR … #else # define USE_LINUX_XATTR false` —
   the frozen configuration takes the `#else` arm (both macros are undefined).  */
pub const USE_LINUX_XATTR: bool = false;

/* C 426-443.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_has_aclinfo (name: *const c_char, ai: *mut aclinfo,
                                           flags: c_int) -> c_int
{
  return fdfile_has_aclinfo (-1, name, ai, flags);
}

/* C 458-968.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdfile_has_aclinfo (fd: c_int, name: *const c_char,
                                             ai: *mut aclinfo, flags: c_int) -> c_int
{
  let d_type: c_uchar = (flags & UCHAR_MAX) as c_uchar;

  /* C 465-509's `#if USE_LINUX_XATTR` arm is dead (USE_LINUX_XATTR is false): `get_aclinfo`,
     `aclinfo_may_indicate_xattr`, `listxattr`, `listxattrat` and `acl_nfs4_nontrivial`.  */

  /* C 513-517: the `#else /* !USE_LINUX_XATTR */` arm's initialization.  */
  (*ai).buf = (*ai).u._gl_acl_ch.as_mut_ptr ();
  (*ai).size = -1;
  (*ai).u.err = ENOTSUP;
  (*ai).scontext = UNKNOWN_SECURITY_CONTEXT.as_ptr () as *mut c_char;
  (*ai).scontext_err = ENOTSUP;

  /* C 519-964's `# if USE_ACL` chain is dead (USE_ACL is 0): the `HAVE_ACL_GET_FILE` arms
     (`acl_extended_file`/`acl_extended_fd`, `acl_get_file`/`acl_get_link_np`/`acl_get_fd_np`
     with `ACL_TYPE_EXTENDED`, and `acl_get_fdfile` with `ACL_TYPE_ACCESS`/`ACL_TYPE_DEFAULT`
     plus `acl_entries`/`acl_free` and the Cygwin `__CYGWIN__` arms), the Solaris/Cygwin
     `GETACL` arm (`acl`/`acl_get`/`acl_entries`/`acl_nontrivial` with the `ace_t` and
     `aclent_t` families and `NEW_ACE_*`), the HP-UX `HAVE_GETACL` arm (with `acl`/`aclvsort`
     and `HAVE_ACLV_H`), the AIX `ACLX_GET && ACL_AIX_WIP` arm (`statacl`/`fstatacl`, `STX_*`,
     `ACL_*`) and the NonStop `HAVE_ACLSORT` arm (`aclsort`, `NACLENTRIES`, `abort`), along
     with `DT_DIR`/`DT_UNKNOWN`, `_GL_DT_NOTDIR`, `UCHAR_MAX` and `d_type`'s uses in them.  */

  return 0;
}

/* C 970-986.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_has_acl (name: *const c_char, sb: *const stat) -> c_int
{
  let mut flags: c_int = IFTODT! ((*sb).st_mode);
  if ! S_ISDIR! ((*sb).st_mode)
    { flags |= _GL_DT_NOTDIR; }
  let mut ai: aclinfo = std::mem::zeroed ();
  let r: c_int = file_has_aclinfo (name, &mut ai, flags);
  aclinfo_free! (&mut ai);
  return r;
}
