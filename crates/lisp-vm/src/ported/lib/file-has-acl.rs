// femacs: port of lib/file-has-acl.c @ fd7afced403.
//
// One-to-one port (PORTING.md 0.1/0.3): the same functions, macros, file-scope objects,
// statement order and control flow, with **both arms of every `#if`/`#ifdef` carried** as
// `#[cfg (feature = "...")]` text (P-86 clause 1).  The frozen configuration (darwin-arm64)
// resolves all 32 of the file's conditions (`python3 tools/check-cond-eval.py
// lib/file-has-acl.c`: 0 unknown, 0 disagreements) and takes:
//
//   * C 24-26   `#if _GL_GNUC_PREREQ (4, 6)` is false and a `#pragma` has no Rust spelling,
//     so C's text is kept and no arm is written (the `ccl.rs` 913 / `window.rs` 14 reading).
//   * C 42-46   `#if HAVE_LINUX_XATTR_H && HAVE_LISTXATTR` is false (HAVE_LINUX_XATTR_H is
//     undefined; HAVE_LISTXATTR is 1), so `USE_LINUX_XATTR` is `false` and C 48-382's block --
//     the five `XATTR_NAME_*` defaults, the SMACK and SELinux helpers, `get_aclinfo`,
//     `has_xattr`, `aclinfo_has_xattr`, `aclinfo_free` and the `acl_nfs4_nontrivial` XDR
//     reader -- is dead text the port carries under `use_linux_xattr`.
//   * C 384-424 `#if (!USE_LINUX_XATTR && USE_ACL && HAVE_ACL_GET_FILE && ...)` is false
//     (USE_ACL is 0), so the FreeBSD/NetBSD/Cygwin `acl_get_link_np`/`acl_get_fdfile` helper
//     pair is dead too.
//   * C 465-965 `#if USE_LINUX_XATTR` / `#else`: the `#else` arm (C 511-964) is the live one.
//   * C 519-964 `# if USE_ACL` is false (USE_ACL is 0): the whole ACL chain -- the
//     Linux/macOS/FreeBSD `HAVE_ACL_GET_FILE` arms, the Solaris/Cygwin `HAVE_FACL && GETACL`
//     arm with its `ACE_GETACL` half, the HP-UX `HAVE_GETACL` arm with its `HAVE_ACLV_H` half,
//     the AIX `ACLX_GET` and `STATACL` arms and the NonStop `HAVE_ACLSORT` arm -- is dead.
//
// Live text: C 37-40's assertion and context string, C 44-46's `USE_LINUX_XATTR`,
// `file_has_aclinfo` (C 438-443), `fdfile_has_aclinfo`'s `#else` arm (C 513-517 and C 967's
// `return 0;`) and `file_has_acl` (C 976-986).  C 1-18 is the reference's opening block; the
// port does not carry it (PORTING.md 0.1-19).
//
// **The surfaces this unit's dead arms name are declared here (P-88 clause 1).**
// `include/sys_acl.rs` mirrors the frozen target's own `<sys/acl.h>` (the POSIX-draft API:
// `acl_t`, `ACL_TYPE_*`, `acl_get_file`, `acl_get_fd_np`, `acl_free`, `acl_extended_file`,
// `acl_extended_fd`), and `include/acl.rs`/`include/acl-internal.rs` their two headers.  The
// other platforms' headers have no mirror in this tree, so each arm declares the names its own
// text reads at its own use site, block-scoped where two arms spell a name the same way
// (Solaris's and HP-UX's `acl`), and every value or layout this machine cannot measure is
// marked UNVERIFIED.  An arm's condition selects a *different* target, so nothing in an arm
// is a claim about this one.
//
// `MAYBE_UNUSED` (C 459-463) is `attribute.h`'s `_GL_ATTRIBUTE_MAYBE_UNUSED`: the port keeps
// the parameters and the variable, and `docs/ports/macro-body-notes.tsv` carries the row.
//
// `docs/ports/file-has-acl.md` carries the `## C line ranges` table, the readings, the
// deviations and what the integrator owes.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe, unreachable_code, unused_assignments, unused_parens, unused_comparisons, while_true, overflowing_literals)]
// [U1-MODULE-UNIT]
use crate::sys_common::*;
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort,
               c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/errno.rs imported via common
// [U1] include/limits.in.rs imported via common
// [U1] include/stdint.in.rs imported via common
// [U1] include/sys_types.in.rs imported via common
// [U1] include/sys_stat.in.rs imported via common
// [U1] include/dirent.rs imported via common
// [U1] include/acl.rs imported via common
// [U1] include/acl-internal.rs imported via common
/* C 34's `"attribute.h"` and C 35's `"minmax.h"` supply `MAYBE_UNUSED` and `MIN`, which only
   the dead arms read: the port keeps the parameters and gates the `MIN` user's include on its
   own arm (the `_GL_ATTRIBUTE_MAYBE_UNUSED` row is in `macro-body-notes.tsv`).  */
#[cfg (feature = "use_acl_getacl")]
const _: () = ();

/// PORTING.md 2.5: C's `static_assert` is `const _: () = core::assert! (cond);`.
macro_rules! static_assert {
  ($cond:expr) => { const _: () = core::assert! ($cond); };
}

/* C 22-26: `#if _GL_GNUC_PREREQ (4, 6)` / `# pragma GCC diagnostic ignored
   "-Wsuggest-attribute=const"` / `#endif`.  A pragma has no Rust spelling, and the guard is a
   compiler-version test rather than a configuration macro: C's text is kept and no arm is
   written.  */

/* Check the assumption that UCHAR_MAX < INT_MAX.  */
static_assert! ((ACL_SYMLINK_FOLLOW & ! (((-1i32) as c_uchar) as c_int)) != 0);

/* C 40: `static char const UNKNOWN_SECURITY_CONTEXT[] = "?";`.  C's object is `static`, so the
   port's is a plain `static` too (PORTING.md 0.2).  */
static UNKNOWN_SECURITY_CONTEXT: [c_char; 2] = [b'?' as c_char, 0];

/* C 42-46: `#if HAVE_LINUX_XATTR_H && HAVE_LISTXATTR` / `# define USE_LINUX_XATTR true` /
   `#else` / `# define USE_LINUX_XATTR false` / `#endif`.  The frozen configuration takes the
   `#else` arm (HAVE_LINUX_XATTR_H is undefined, HAVE_LISTXATTR is 1), and the port keeps that
   arm live; the other one is text under `use_linux_xattr`, the feature every later
   `#if USE_LINUX_XATTR` is gated on.  */
#[cfg (not (feature = "use_linux_xattr"))]
pub const USE_LINUX_XATTR: bool = false;
#[cfg (feature = "use_linux_xattr")]
pub const USE_LINUX_XATTR: bool = true;

/* `USE_ACL` is the configuration macro C 166 and C 519 test; the frozen table has 0
   (`docs/baseline/config/darwin-arm64.macros.tsv` 3856), which the port spells here, and the
   feature `use_acl` names the other side (P-08/P-51).  */
#[cfg (not (feature = "use_acl"))]
pub const USE_ACL: c_int = 0;
#[cfg (feature = "use_acl")]
pub const USE_ACL: c_int = 1;

/* ==========================================================================================
   C 48-382: `#if USE_LINUX_XATTR` (dead on the frozen target).  C 49-56's includes, then
   C 57-71's five `XATTR_NAME_*` defaults, the SMACK helpers (C 73-99), the ACE4 flags
   (C 101-105), `aclinfo_may_indicate_xattr` (C 113-118), `has_xattr` (C 122-138),
   `aclinfo_has_xattr` (C 142-154), `get_aclinfo` (C 160-276), `aclinfo_scontext_free`
   (C 278-294), `aclinfo_free` (C 296-303) and `acl_nfs4_nontrivial` (C 317-381).
   ========================================================================================== */

/* C 49-51: `# if USE_SELINUX_SELINUX_H` / `#  include <selinux/selinux.h>` / `# endif`.  The
   header has no mirror here, so its surface is declared under the arm's own feature below.  */
// C 50: `#include <selinux/selinux.h>`
/* C 52-53: `#include <stdckdint.h>` and `#include <string.h>`; the two template mirrors own
   `ckd_add`, `strnul!` and `memeq`, the names C 202/148/263 read.  */
#[cfg (feature = "use_linux_xattr")]
const _: () = ();
#[cfg (feature = "use_linux_xattr")]
const _: () = ();
#[cfg (feature = "use_linux_xattr")]
const _: () = ();
#[cfg (any (feature = "use_linux_xattr", feature = "use_acl"))]
const _: () = ();
#[cfg (all (feature = "use_linux_xattr", feature = "have_smack"))]
// C 74: `#include <sys/smack.h>`
/* C 54-56: `#include <arpa/inet.h>`, `#include <sys/xattr.h>` and `#include <linux/xattr.h>`
   have no mirror in this tree either, so each name C's text reads is declared here (P-88
   clause 1).  UNVERIFIED: this machine's `<arpa/inet.h>` agrees on `ntohl`, but its
   `<sys/xattr.h>` has the six-argument flavor where C's arm reads Linux's four-argument one,
   and there is no `<linux/xattr.h>` at all; the caller's condition, not this machine, decides
   which prototype is live.  */
#[cfg (feature = "use_linux_xattr")]
unsafe extern "C" {
  pub fn ntohl (netlong: uint32_t) -> uint32_t;
  pub fn getxattr (path: *const c_char, name: *const c_char, value: *mut c_void, size: usize)
    -> c_long;
  pub fn lgetxattr (path: *const c_char, name: *const c_char, value: *mut c_void, size: usize)
    -> c_long;
  pub fn fgetxattr (fd: c_int, name: *const c_char, value: *mut c_void, size: usize) -> c_long;
  pub fn listxattr (path: *const c_char, list: *mut c_char, size: usize) -> c_long;
  pub fn llistxattr (path: *const c_char, list: *mut c_char, size: usize) -> c_long;
  pub fn flistxattr (fd: c_int, list: *mut c_char, size: usize) -> c_long;
}

/* C 50's `<selinux/selinux.h>` surface (P-88 clause 1): UNVERIFIED, no SELinux on this
   machine.  `freecon` is read by `aclinfo_scontext_free` as well as by `get_aclinfo`, so the
   declarations stand at the crate root.  */
#[cfg (all (feature = "use_linux_xattr", feature = "use_selinux_selinux_h"))]
unsafe extern "C" {
  pub fn getfilecon (path: *const c_char, con: *mut *mut c_char) -> c_long;
  pub fn lgetfilecon (path: *const c_char, con: *mut *mut c_char) -> c_long;
  pub fn fgetfilecon (fd: c_int, con: *mut *mut c_char) -> c_long;
  pub fn freecon (con: *mut c_char);
}

/* C 117/134's `E2BIG` is `<errno.h>`'s; the mirror carries it since status 3.514 (it used to be
   declared here under P-72 clause 4 - P-88 (5)'s moved-back shape).  */

/* C 57-71: five `# ifndef XATTR_NAME_* / #  define XATTR_NAME_* "..."` arms.  The header that
   would define them is `<linux/xattr.h>`, which is not carried, so the arm C would keep on a
   target without it is the one written; C's `define` text is the value.  */
#[cfg (feature = "use_linux_xattr")]
pub const XATTR_NAME_SMACK: &core::ffi::CStr = c"security.SMACK64";
#[cfg (feature = "use_linux_xattr")]
pub const XATTR_NAME_SELINUX: &core::ffi::CStr = c"security.selinux";
#[cfg (feature = "use_linux_xattr")]
pub const XATTR_NAME_NFSV4_ACL: &core::ffi::CStr = c"system.nfs4_acl";
#[cfg (feature = "use_linux_xattr")]
pub const XATTR_NAME_POSIX_ACL_ACCESS: &core::ffi::CStr = c"system.posix_acl_access";
#[cfg (feature = "use_linux_xattr")]
pub const XATTR_NAME_POSIX_ACL_DEFAULT: &core::ffi::CStr = c"system.posix_acl_default";

/* C 73-94: `# ifdef HAVE_SMACK` / `#  include <sys/smack.h>` / `# else` ... `# endif`.  The
   `#else` arm's three stubs are C's own text; `UNNAMED` is `attribute.h`'s name-hiding
   attribute, which the port writes as `_`-prefixed parameters (PORTING.md 10.6).  UNVERIFIED:
   libsmack is not on this machine.  */
#[cfg (all (feature = "use_linux_xattr", feature = "have_smack"))]
unsafe extern "C" {
  pub fn smack_smackfs_path () -> *const c_char;
  pub fn smack_new_label_from_path (path: *const c_char, xattr: *const c_char, follow: c_int,
                                    label: *mut *mut c_char) -> c_long;
  pub fn smack_new_label_from_file (fd: c_int, xattr: *const c_char,
                                    label: *mut *mut c_char) -> c_long;
}
#[cfg (all (feature = "use_linux_xattr", not (feature = "have_smack")))]
unsafe fn smack_smackfs_path () -> *const c_char
{
  return std::ptr::null ();
}
#[cfg (all (feature = "use_linux_xattr", not (feature = "have_smack")))]
unsafe fn smack_new_label_from_path (_path: *const c_char, _xattr: *const c_char,
                                     _follow: c_int, _label: *mut *mut c_char) -> c_long
{
  return -1;
}
#[cfg (all (feature = "use_linux_xattr", not (feature = "have_smack")))]
unsafe fn smack_new_label_from_file (fd: c_int, _xattr: *const c_char,
                                     _label: *mut *mut c_char) -> c_long
{
  return -1;
}

/* C 95-99.  */
#[cfg (feature = "use_linux_xattr")]
unsafe fn is_smack_enabled () -> bool
{
  return ! smack_smackfs_path ().is_null ();
}

/* C 101-105: `enum {` with a commented-out `ACE4_ACCESS_ALLOWED_ACE_TYPE = 0x00000000,`,
   then `ACE4_ACCESS_DENIED_ACE_TYPE = 0x00000001, ACE4_IDENTIFIER_GROUP = 0x00000040 };`.  */
#[cfg (feature = "use_linux_xattr")]
pub const ACE4_ACCESS_DENIED_ACE_TYPE: c_int = 0x00000001;
#[cfg (feature = "use_linux_xattr")]
pub const ACE4_IDENTIFIER_GROUP: c_int = 0x00000040;

/* AI indicates XATTR may be present but wasn't accessible.
   This is the case when [l]listxattr failed with E2BIG,
   or is not supported (!acl_errno_valid()), or failed with EACCES
   which in Linux kernel 6.12 NFS can mean merely that we lack read access.
*/

/* C 113-118.  */
#[cfg (feature = "use_linux_xattr")]
unsafe fn aclinfo_may_indicate_xattr (ai: *const aclinfo) -> bool
{
  return (*ai).size < 0 && (! acl_errno_valid ((*ai).u.err)
                            || (*ai).u.err == EACCES || (*ai).u.err == E2BIG);
}

/* Does NAME have XATTR?  */

/* C 122-138.  */
#[cfg (feature = "use_linux_xattr")]
unsafe fn has_xattr (xattr: *const c_char, ai: *const aclinfo, fd: c_int,
                     name: *const c_char, flags: c_int) -> bool
{
  if ! ai.is_null () && aclinfo_has_xattr (ai, xattr)
    { return true; }
  else if ai.is_null () || aclinfo_may_indicate_xattr (ai)
    {
      let ret: c_long = (if fd < 0
                         { (if (flags & ACL_SYMLINK_FOLLOW) != 0 { getxattr } else { lgetxattr })
                           (name, xattr, std::ptr::null_mut (), 0) }
                         else { fgetxattr (fd, xattr, std::ptr::null_mut (), 0) });
      if 0 <= ret || (*errno () == ERANGE || *errno () == E2BIG)
        { return true; }
    }
  return false;
}

/* Does AI's xattr set contain XATTR?  */

/* C 142-154.  C's inner `for` advances `b` as well as `a`, and the outer one's increment then
   runs from wherever the inner loop stopped: the port keeps that exact shape.  */
#[cfg (feature = "use_linux_xattr")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aclinfo_has_xattr (ai: *const aclinfo, xattr: *const c_char) -> bool
{
  if 0 < (*ai).size
    {
      let blim: *const c_char = (*ai).buf.offset ((*ai).size as isize);
      let mut b: *const c_char = (*ai).buf;
      while b < blim
        {
          let mut a: *const c_char = xattr;
          while *a == *b
            {
              a = a.offset (1);
              b = b.offset (1);
            }
          if *a == 0
            { return true; }
          b = (strnul! (b)).offset (1);
        }
    }
  return false;
}

/* Get attributes of the file FD aka NAME into AI, if USE_ACL.
   Ignore FD if it is negative.
   If FLAGS & ACL_GET_SCONTEXT, also get security context.
   If FLAGS & ACL_SYMLINK_FOLLOW, follow symbolic links.  */

/* C 160-276.  `sizeof ai->u._gl_acl_ch` is the mirror's own array length; C's `ckd_add` is the
   frozen spelling P-15 records (`overflowing_add` with the store at the site).  */
#[cfg (feature = "use_linux_xattr")]
unsafe fn get_aclinfo (fd: c_int, name: *const c_char, ai: *mut aclinfo, flags: c_int)
{
  (*ai).buf = (*ai).u._gl_acl_ch.as_mut_ptr ();
  let mut acl_alloc: c_long = std::mem::size_of_val (&(*ai).u._gl_acl_ch) as c_long;

  if ! (USE_ACL != 0 || (flags & ACL_GET_SCONTEXT) != 0)
    { (*ai).size = 0; }
  else
    {
      let lsxattr: unsafe extern "C" fn (*const c_char, *mut c_char, usize) -> c_long
        = if (flags & ACL_SYMLINK_FOLLOW) != 0 { listxattr } else { llistxattr };
      while true
        {
          (*ai).size = (if fd < 0
                        { lsxattr (name, (*ai).buf, acl_alloc as usize) }
                        else { flistxattr (fd, (*ai).buf, acl_alloc as usize) });
          if 0 < (*ai).size
            { break; }
          (*ai).u.err = if (*ai).size < 0 { *errno () } else { 0 };
          if ! ((*ai).size < 0 && (*ai).u.err == ERANGE && acl_alloc < SSIZE_MAX)
            { break; }

          /* The buffer was too small.  Find how large it should have been.  */
          let size: c_long = (if fd < 0
                              { lsxattr (name, std::ptr::null_mut (), 0) }
                              else { flistxattr (fd, std::ptr::null_mut (), 0) });
          if size <= 0
            {
              (*ai).size = size;
              (*ai).u.err = if size < 0 { *errno () } else { 0 };
              break;
            }

          /* Grow allocation to at least 'size'.  Grow it by a nontrivial
             amount, to defend against denial of service by an adversary
             that fiddles with ACLs.  */
          if (*ai).buf != (*ai).u._gl_acl_ch.as_mut_ptr ()
            {
              free ((*ai).buf as *mut c_void);
              (*ai).buf = (*ai).u._gl_acl_ch.as_mut_ptr ();
            }
          let (ckd_sum, ckd_overflowed) = acl_alloc.overflowing_add (acl_alloc >> 1);
          acl_alloc = ckd_sum;
          if ckd_overflowed
            { acl_alloc = SSIZE_MAX; }
          if acl_alloc < size
            { acl_alloc = size; }
          if SIZE_MAX < acl_alloc as usize
            {
              (*ai).u.err = ENOMEM;
              break;
            }
          let newbuf: *mut c_char = malloc (acl_alloc as usize) as *mut c_char;
          if newbuf.is_null ()
            {
              (*ai).u.err = *errno ();
              break;
            }
          (*ai).buf = newbuf;
        }
    }

  /* A security context can exist only if extended attributes do.  */
  let mut scontext_err: c_int = ENOTSUP;
  if (flags & ACL_GET_SCONTEXT) != 0
    && (0 < (*ai).size || aclinfo_may_indicate_xattr (ai))
    {
      if is_smack_enabled ()
        {
          if (*ai).size < 0 || aclinfo_has_xattr (ai, XATTR_NAME_SMACK.as_ptr ())
            {
              /* C 230: `static char const SMACK64[] = "security.SMACK64";`.  */
              let SMACK64: &core::ffi::CStr = c"security.SMACK64";
              let r: c_long =
                (if fd < 0
                 { smack_new_label_from_path (name, SMACK64.as_ptr (),
                                              ((flags & ACL_SYMLINK_FOLLOW) != 0) as c_int,
                                              &mut (*ai).scontext) }
                 else { smack_new_label_from_file (fd, SMACK64.as_ptr (),
                                                   &mut (*ai).scontext) });
              scontext_err = if r < 0 { *errno () } else { 0 };
            }
        }
      else
        {
          /* C 242-270: `# if USE_SELINUX_SELINUX_H` ... `# endif`; the header has no mirror
             here, so its three entry points and `freecon` are declared under the arm's own
             feature.  UNVERIFIED (no SELinux on this machine).  */
          #[cfg (feature = "use_selinux_selinux_h")]
          {
            if (*ai).size < 0 || aclinfo_has_xattr (ai, XATTR_NAME_SELINUX.as_ptr ())
              {
                let r: c_long =
                  (if fd < 0
                   { (if (flags & ACL_SYMLINK_FOLLOW) != 0 { getfilecon } else { lgetfilecon })
                     (name, &mut (*ai).scontext) }
                   else { fgetfilecon (fd, &mut (*ai).scontext) });
                scontext_err = if r < 0 { *errno () } else { 0 };
                /* C 251-268: `#  ifndef SE_SELINUX_INLINE`.  */
                #[cfg (not (feature = "se_selinux_inline"))]
                {
                  /* Gnulib's selinux-h module is not in use, so getfilecon and
                     lgetfilecon can misbehave, be it via an old version of
                     libselinux where these would return 0 and set the result
                     context to NULL, or via a modern kernel+lib operating on a
                     file from a disk whose attributes were set by a kernel from
                     around 2006.  In that latter case, the functions return a
                     length of 10 for the "unlabeled" context.  Map both failures
                     to a return value of -1, and set errno to ENOTSUP in the
                     first case, and ENODATA in the latter.  */
                  if r == 0
                    { scontext_err = ENOTSUP; }
                  if r == 10 && memeq ((*ai).scontext as *const c_void,
                                       c"unlabeled".as_ptr () as *const c_void, 10)
                    {
                      freecon ((*ai).scontext);
                      scontext_err = ENODATA;
                    }
                }
              }
          }
        }
    }
  (*ai).scontext_err = scontext_err;
  if scontext_err != 0
    { (*ai).scontext = UNKNOWN_SECURITY_CONTEXT.as_ptr () as *mut c_char; }
}

/* C 278-294: `# ifndef aclinfo_scontext_free` / ... / `# endif`.  The header's own `#if`
   defines the macro when the xattr path is on *and* one of SMACK/SELinux is configured, so the
   function is the arm for exactly that configuration.  */
#[cfg (all (feature = "use_linux_xattr",
            any (feature = "have_smack", feature = "use_selinux_selinux_h")))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aclinfo_scontext_free (scontext: *mut c_char)
{
  /* Free the pointer that file_has_aclinfo put into scontext.
     However, do nothing if the argument is a null pointer;
     This lets the caller replace the scontext member with a null pointer if it is
     willing to own the member and call this function later.  */
  if scontext != UNKNOWN_SECURITY_CONTEXT.as_ptr () as *mut c_char
    {
      if is_smack_enabled ()
        { free (scontext as *mut c_void); }
      else if ! scontext.is_null ()
        { freecon (scontext); }
    }
}

/* Free AI's heap storage.  */

/* C 296-303.  */
#[cfg (feature = "use_linux_xattr")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aclinfo_free (ai: *mut aclinfo)
{
  if (*ai).buf != (*ai).u._gl_acl_ch.as_mut_ptr ()
    { free ((*ai).buf as *mut c_void); }
  aclinfo_scontext_free ((*ai).scontext);
}

/* Return 1 if given ACL in XDR format is non-trivial, 0 if it is trivial.
   -1 upon failure to determine it.  Possibly change errno.  Assume that
   the ACL is valid, except avoid undefined behavior even if invalid.

   See <https://linux.die.net/man/5/nfs4_acl>.  The NFSv4 acls are
   defined in Internet RFC 7530 and as such, every NFSv4 server
   supporting ACLs should support NFSv4 ACLs (they differ from from
   POSIX draft ACLs).  The ACLs can be obtained via the
   nfsv4-acl-tools, e.g., the nfs4_getfacl command.  Gnulib provides
   only basic support of NFSv4 ACLs, i.e., recognize trivial vs
   nontrivial ACLs.  */

/* C 317-381.  */
#[cfg (feature = "use_linux_xattr")]
unsafe fn acl_nfs4_nontrivial (mut xattr: *mut uint32_t, mut nbytes: c_long) -> c_int
{
  const BYTES_PER_NETWORK_UINT: c_int = 4;

  /* Grab the number of aces in the acl.  */
  nbytes -= BYTES_PER_NETWORK_UINT as c_long;
  if nbytes < 0
    { return -1; }
  let num_aces: uint32_t = ntohl (*xattr);
  xattr = xattr.offset (1);
  if 6 < num_aces
    { return 1; }
  let mut ace_found: c_int = 0;

  let mut ace_n: c_int = 0;
  while ace_n < num_aces as c_int
    {
      /* Get the acl type and flag.  Skip the mask; it's too risky to
         test it and it does not seem to be needed.  Get the wholen.  */
      nbytes -= 4 * BYTES_PER_NETWORK_UINT as c_long;
      if nbytes < 0
        { return -1; }
      let r#type: uint32_t = ntohl (*xattr.offset (0));
      let flag: uint32_t = ntohl (*xattr.offset (1));
      let wholen: uint32_t = ntohl (*xattr.offset (3));
      xattr = xattr.offset (4);
      let whowords: c_int = (wholen / BYTES_PER_NETWORK_UINT as uint32_t
                             + ((wholen % BYTES_PER_NETWORK_UINT as uint32_t) != 0) as uint32_t)
                            as c_int;
      let mut wholen4: int64_t = whowords as int64_t;
      wholen4 *= BYTES_PER_NETWORK_UINT as int64_t;

      /* Trivial ACLs have only ACE4_ACCESS_ALLOWED_ACE_TYPE or
         ACE4_ACCESS_DENIED_ACE_TYPE.  */
      if (ACE4_ACCESS_DENIED_ACE_TYPE as uint32_t) < r#type
        { return 1; }

      /* RFC 7530 says FLAG should be 0, but be generous to NetApp and
         also accept the group flag.  */
      if (flag & !(ACE4_IDENTIFIER_GROUP as uint32_t)) != 0
        { return 1; }

      /* Get the who string.  Check NBYTES - WHOLEN4 before storing
         into NBYTES, to avoid truncation on conversion.  */
      if nbytes - wholen4 < 0
        { return -1; }
      nbytes -= wholen4;

      /* For a trivial ACL, max 6 (typically 3) ACEs, 3 allow, 3 deny.
         Check that there is at most one ACE of each TYPE and WHO.  */
      let who2: c_int
        = (if wholen == 6 && memeq (xattr as *const c_void,
                                    c"OWNER@".as_ptr () as *const c_void, 6) { 0 }
           else if wholen == 6 && memeq (xattr as *const c_void,
                                         c"GROUP@".as_ptr () as *const c_void, 6) { 2 }
           else if wholen == 9 && memeq (xattr as *const c_void,
                                         c"EVERYONE@".as_ptr () as *const c_void, 9) { 4 }
           else { -1 });
      if who2 < 0
        { return 1; }
      let ace_found_bit: c_int = 1 << (who2 | r#type as c_int);
      if (ace_found & ace_found_bit) != 0
        { return 1; }
      ace_found |= ace_found_bit;

      xattr = xattr.offset (whowords as isize);

      ace_n += 1;
    }

  return 0;
}
/* ==========================================================================================
   C 384-424: `#if (!USE_LINUX_XATTR && USE_ACL && HAVE_ACL_GET_FILE
                    && !HAVE_ACL_EXTENDED_FILE && !HAVE_ACL_TYPE_EXTENDED)`
   -- FreeBSD, NetBSD >= 10, Cygwin >= 2.5 (dead on the frozen target).
   ========================================================================================== */

/* C 388-389: `# if HAVE_ACL_GET_FD && !HAVE_ACL_GET_LINK_NP` / `#  include <fcntl.h>`.
   `<fcntl.h>` and `<unistd.h>` have mirrors, so the arm reaches them; `O_PATH` has no mirror on
   this target (the frozen `<fcntl.h>` has none) and is declared below, UNVERIFIED.  */
#[cfg (all (feature = "use_acl_have_acl_get_file", not (feature = "use_linux_xattr"),
            not (feature = "have_acl_extended_file"), not (feature = "have_acl_type_extended"),
            feature = "have_acl_get_fd", not (feature = "have_acl_get_link_np")))]
const _: () = ();
#[cfg (all (feature = "use_acl_have_acl_get_file", not (feature = "use_linux_xattr"),
            not (feature = "have_acl_extended_file"), not (feature = "have_acl_type_extended"),
            feature = "have_acl_get_fd", not (feature = "have_acl_get_link_np")))]
const _: () = ();

/* C 390-391: `#  ifdef O_PATH` / `#   define acl_get_fd_np(fd, type) acl_get_fd (fd)`.  The
   header that defines `O_PATH` is Linux's/Cygwin's; UNVERIFIED here (0x200000 is Linux's).
   C's `#define` is a macro, and a Rust `macro_rules!` of the same name would shadow the
   `<sys/acl.h>` mirror's own `acl_get_fd_np` function for the wrong reason (P-26: a unit-side
   macro does not collide with a mirror's function, it silently answers calls C answers with the
   function), so the port writes C's expansion at the one call site under C 390's own
   `#[cfg (feature = "o_path")]`.  */
#[cfg (feature = "o_path")]
pub const O_PATH: c_int = 0x200000;

/* Like acl_get_file, but do not follow symbolic links.  */

/* C 393-405.  */
#[cfg (all (feature = "use_acl_have_acl_get_file", not (feature = "use_linux_xattr"),
            not (feature = "have_acl_extended_file"), not (feature = "have_acl_type_extended"),
            feature = "have_acl_get_fd", not (feature = "have_acl_get_link_np"),
            feature = "o_path"))]
unsafe extern "C" fn acl_get_link_np (name: *const c_char, r#type: acl_type_t) -> acl_t
{
  let fd: c_int = open (name, O_PATH | O_NOFOLLOW);
  if fd < 0
    { return std::ptr::null_mut (); }
  let r: acl_t = acl_get_fd (fd);
  let saved_errno: c_int = *errno ();
  close (fd);
  *errno () = saved_errno;
  return r;
}
/* C 406: `#   define HAVE_ACL_GET_LINK_NP 1`.  C defines a configuration macro here, which a
   Rust `#[cfg]` cannot read: the port reads it as the feature `have_acl_get_link_np`, and
   C 414's arm is gated on it.  A Cygwin target with `O_PATH` must set both.  */

/* C 410-423.  */
#[cfg (all (feature = "use_acl_have_acl_get_file", not (feature = "use_linux_xattr"),
            not (feature = "have_acl_extended_file"), not (feature = "have_acl_type_extended")))]
unsafe fn acl_get_fdfile (fd: c_int, name: *const c_char, r#type: acl_type_t, flags: c_int)
  -> acl_t
{
  let mut get: unsafe extern "C" fn (*const c_char, acl_type_t) -> acl_t = acl_get_file;
  /* C 414-421: `# if HAVE_ACL_GET_LINK_NP` ... `# else` with C's own comment
     `Ignore FD and FLAGS, unfortunately.` ... `# endif`.  C 406 is what makes the first arm
     live in the Cygwin + `O_PATH` case (it defines HAVE_ACL_GET_LINK_NP for every line after
     it), so the gate here is that definition's own condition as well as the system's.  */
  #[cfg (any (feature = "have_acl_get_link_np",
              all (feature = "have_acl_get_fd", not (feature = "have_acl_get_link_np"),
                   feature = "o_path")))]
  {
    if 0 <= fd
      {
        /* C 391's `#define acl_get_fd_np(fd, type) acl_get_fd (fd)` is the spelling exactly
           when the system does not define HAVE_ACL_GET_LINK_NP (C 388) and C 390's
           `#ifdef O_PATH` ran, which is where that `#define` sits.  */
        #[cfg (all (not (feature = "have_acl_get_link_np"), feature = "o_path"))]
        { return acl_get_fd (fd); }
        #[cfg (any (feature = "have_acl_get_link_np", not (feature = "o_path")))]
        { return acl_get_fd_np (fd, r#type); }
      }
    if (flags & ACL_SYMLINK_FOLLOW) == 0
      { get = acl_get_link_np; }
  }
  return get (name, r#type);
}

/* ==========================================================================================
   The foreign surfaces this unit's dead arms read (P-88 clause 1).  None of these headers has
   a mirror in this tree, so each surface is declared here; each is **module-scoped** so that
   two arms spelling a name the same way (`acl` is Solaris's, HP-UX's and NonStop's) cannot
   collide, and so that a name a mirror already declares keeps one spelling in the unit
   (P-26: the arm's block glob-imports its own surface and the module's item is what answers).
   Every width and layout below is UNVERIFIED -- none of these headers is on this machine, and
   the arm's condition, not this target, says which prototype is live.
   ========================================================================================== */

/* The four foreign aggregate types the mirrors' own declarations mention by name.  */
#[cfg (feature = "use_acl_getacl")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct aclent_t
{
  pub a_type: c_int,
  pub a_id: c_uint,
  pub a_perm: c_uint,
}
#[cfg (feature = "use_acl_ace_getacl")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ace_t
{
  pub a_who: uint32_t,
  pub a_access_mask: uint32_t,
  pub a_flags: c_ushort,
  pub a_type: c_ushort,
}
#[cfg (feature = "use_acl_have_getacl")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct acl_entry
{
  pub ae_id: c_int,
}
#[cfg (any (feature = "use_acl_have_getacl", feature = "use_acl_have_statacl",
            feature = "use_acl_have_aclsort"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct acl
{
  pub a_room: [c_char; 12],
}
#[cfg (any (feature = "use_acl_have_getacl", feature = "use_acl_have_aclsort"))]
pub const NACLENTRIES: usize = 100;
#[cfg (feature = "use_acl_have_getacl")]
pub const NACLVENTRIES: usize = 100;

/* C 520-599's three sub-arms (Linux, Mac OS X, FreeBSD/NetBSD/Cygwin).  */
#[cfg (feature = "use_acl_have_acl_get_file")]
mod acl_get_file_surface {
  use super::*;

  unsafe extern "C" {
    pub fn acl_get_fd (fd: c_int) -> acl_t;
    pub fn acl_extended_file_nofollow (path: *const c_char) -> c_int;
    pub fn acl_extended_nontrivial (acl: acl_t) -> c_int;
    pub fn acl_access_nontrivial (acl: acl_t) -> c_int;
    pub fn acl_entries (acl: acl_t) -> c_int;
    pub fn acl_get_link_np (name: *const c_char, r#type: acl_type_t) -> acl_t;
  }
}

/* `acl_get_fdfile` and the `HAVE_ACL_GET_FILE` arms are top-level items, so the surface is
   reached at the crate root as well as inside those arms' own blocks (a glob import is
   shadowed by any definition, PORTING.md 0.1-10).  */
#[cfg (feature = "use_acl_have_acl_get_file")]
use acl_get_file_surface::*;

/* C 612-777: Solaris, Cygwin < 2.5.  */
#[cfg (feature = "use_acl_getacl")]
mod solaris_acl_surface {
  use super::*;

  pub const GETACL: c_int = 1;
  pub const ACE_GETACL: c_int = 4;

  unsafe extern "C" {
    pub fn acl (pathp: *mut c_char, cmd: c_int, nentries: c_int, aclbufp: *mut c_void)
      -> c_int;
    pub fn facl (fildes: c_int, cmd: c_int, nentries: c_int, aclbufp: *mut c_void) -> c_int;
    pub fn acl_trivial (path: *const c_char) -> c_int;
    pub fn acl_nontrivial (count: c_int, entries: *const aclent_t) -> c_int;
  }

  /* C 205-209's `# ifdef ACE_GETACL` (Solaris 10) declares the `ace_t` predicate inside this
     same Solaris arm, and `ace_t` itself is declared under that condition above - so this
     declaration carries it too: `use_acl_getacl` alone is not enough, and the flip read
     `cannot find type ace_t` without it (status 3.510).  */
  #[cfg (feature = "use_acl_ace_getacl")]
  unsafe extern "C" {
    pub fn acl_ace_nontrivial (count: c_int, entries: *const ace_t) -> c_int;
  }
}

/* C 779-858: HP-UX.  */
#[cfg (all (not (feature = "use_acl_getacl"), feature = "use_acl_have_getacl"))]
mod hpux_acl_surface {
  use super::*;

  pub const ACL_GET: c_int = 1;

  unsafe extern "C" {
    pub fn getacl (path: *const c_char, nentries: c_int, aclbufp: *mut acl_entry) -> c_int;
    pub fn fgetacl (fildes: c_int, nentries: c_int, aclbufp: *mut acl_entry) -> c_int;
    pub fn acl (pathp: *mut c_char, cmd: c_int, nentries: c_int, aclbufp: *mut acl) -> c_int;
    pub fn acl_nontrivial (count: c_int, entries: *const acl_entry) -> c_int;
    pub fn aclv_nontrivial (count: c_int, entries: *const acl) -> c_int;
    pub fn abort () -> !;
  }
}

/* C 860-915: AIX.  */
#[cfg (all (not (feature = "use_acl_getacl"), not (feature = "use_acl_have_getacl"),
            feature = "have_aclx_get", feature = "acl_aix_wip"))]
mod aix_acl_surface {
  use super::*;

  #[repr(C)]
  pub union acl_type_t
  {
    pub u64: c_ulonglong,
    pub u32: c_uint,
  }
  #[repr(C)]
  #[derive(Clone, Copy)]
  pub struct acl
  {
    pub a_room: [c_char; 4096],
  }
  #[repr(C)]
  #[derive(Clone, Copy)]
  pub struct nfs4_acl_int_t
  {
    pub a_room: [c_char; 4096],
  }
  pub const ACL_ANY: c_ulonglong = 0;
  pub const ACL_AIXC: c_ulonglong = 1;
  pub const ACL_NFS4: c_ulonglong = 2;

  unsafe extern "C" {
    pub fn aclx_get (path: *const c_char, flags: c_int, r#type: *mut acl_type_t,
                     acl: *mut c_char, aclsize: *mut usize, mode: *mut mode_t) -> c_int;
    pub fn aclx_fget (fd: c_int, flags: c_int, r#type: *mut acl_type_t, acl: *mut c_char,
                      aclsize: *mut usize, mode: *mut mode_t) -> c_int;
    pub fn acl_nontrivial (acl: *mut acl) -> c_int;
    /* C 902 calls the reader C defines only inside `#if USE_LINUX_XATTR` (C 317); the two
       conditions never hold together, so this is C's own text in its own configuration.  */
    pub fn acl_nfs4_nontrivial (xattr: *mut uint32_t, nbytes: c_long) -> c_int;
  }
}

/* C 917-927: older AIX.  */
#[cfg (all (not (feature = "use_acl_getacl"), not (feature = "use_acl_have_getacl"),
            not (feature = "have_aclx_get"), feature = "use_acl_have_statacl"))]
mod aix_statacl_surface {
  use super::*;

  pub const STX_NORMAL: c_int = 0;

  /* AIX's `struct acl`; this file only takes its address and size.  UNVERIFIED.  */
  #[repr(C)]
  #[derive(Clone, Copy)]
  pub struct acl
  {
    pub a_room: [c_char; 4096],
  }

  unsafe extern "C" {
    pub fn statacl (path: *const c_char, stx: c_int, acl: *mut acl, size: usize) -> c_int;
    pub fn fstatacl (fd: c_int, stx: c_int, acl: *mut acl, size: usize) -> c_int;
    pub fn acl_nontrivial (acl: *mut acl) -> c_int;
  }
}

/* C 929-960: NonStop Kernel.  */
#[cfg (all (not (feature = "use_acl_getacl"), not (feature = "use_acl_have_getacl"),
            not (feature = "have_aclx_get"), not (feature = "use_acl_have_statacl"),
            feature = "use_acl_have_aclsort"))]
mod nonstop_acl_surface {
  use super::*;

  pub const ACL_GET: c_int = 1;

  unsafe extern "C" {
    pub fn acl (pathp: *mut c_char, cmd: c_int, nentries: c_int, aclbufp: *mut c_void)
      -> c_int;
    pub fn acl_nontrivial (count: c_int, entries: *const acl) -> c_int;
    pub fn abort () -> !;
  }
}

/* ==========================================================================================
   C 426-968: `file_has_aclinfo` and `fdfile_has_aclinfo`.
   ========================================================================================== */

/* Return 1 if NAME has a nontrivial access control list,
   0 if ACLs are not supported, or if NAME has no or only a base ACL,
   and -1 (setting errno) on error.  Note callers can determine
   if ACLs are not supported as errno is set in that case also.
   Set *AI to ACL info regardless of return value.
   FLAGS should be a <dirent.h> d_type value, optionally ORed with
     - _GL_DT_NOTDIR if it is known that NAME is not a directory,
     - ACL_GET_SCONTEXT to retrieve security context and return 1 if present,
     - ACL_SYMLINK_FOLLOW to follow the link if NAME is a symbolic link;
       otherwise do not follow them if possible.
   If the d_type value is not known, use DT_UNKNOWN though this may be less
   efficient.  */

/* C 438-443.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_has_aclinfo (name: *const c_char, ai: *mut aclinfo, flags: c_int)
  -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  return fdfile_has_aclinfo (-1, name, ai, flags);

  }
}

/* Return 1 if FD aka NAME has a nontrivial access control list,
   0 if ACLs are not supported, or if NAME has no or only a base ACL,
   and -1 (setting errno) on error.  Note callers can determine
   if ACLs are not supported as errno is set in that case also.
   Ignore FD if it is negative.
   Set *AI to ACL info regardless of return value.
   FLAGS should be a <dirent.h> d_type value, optionally ORed with
     - _GL_DT_NOTDIR if it is known that NAME is not a directory,
     - ACL_GET_SCONTEXT to retrieve security context and return 1 if present,
     - ACL_SYMLINK_FOLLOW to follow the link if NAME is a symbolic link;
       otherwise do not follow them if possible.
   If the d_type value is not known, use DT_UNKNOWN though this may be less
   efficient.  */

/* C 458-968: `MAYBE_UNUSED int fd` / `MAYBE_UNUSED char const *restrict name` /
   `MAYBE_UNUSED unsigned char d_type = flags & UCHAR_MAX;`.  `MAYBE_UNUSED` is
   `attribute.h`'s attribute (macro-body-notes row); the port keeps both parameters and the
   variable.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdfile_has_aclinfo (fd: c_int, name: *const c_char, ai: *mut aclinfo,
                                             flags: c_int) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let d_type: c_uchar = (flags & UCHAR_MAX) as c_uchar;

  /* C 465-509: the `#if USE_LINUX_XATTR` arm (dead on the frozen target).  */
  #[cfg (feature = "use_linux_xattr")]
  {
    let initial_errno: c_int = *errno ();
    get_aclinfo (fd, name, ai, flags);

    if ! aclinfo_may_indicate_xattr (ai) && (*ai).size <= 0
      {
        *errno () = if (*ai).size < 0 { (*ai).u.err } else { initial_errno };
        return (*ai).size as c_int;
      }

    /* In Fedora 39, a file can have both NFSv4 and POSIX ACLs,
       but if it has an NFSv4 ACL that's the one that matters.
       In earlier Fedora the two types of ACLs were mutually exclusive.
       Attempt to work correctly on both kinds of systems.  */

    if ! has_xattr (XATTR_NAME_NFSV4_ACL.as_ptr (), ai, fd, name, flags)
      {
        return (has_xattr (XATTR_NAME_POSIX_ACL_ACCESS.as_ptr (), ai, fd, name, flags)
                || (((d_type as c_int) == DT_DIR || (d_type as c_int) == DT_UNKNOWN)
                    && has_xattr (XATTR_NAME_POSIX_ACL_DEFAULT.as_ptr (), ai, fd, name,
                                  flags))) as c_int;
      }

    /* A buffer large enough to hold any trivial NFSv4 ACL.
       The max length of a trivial NFSv4 ACL is 6 words for owner,
       6 for group, 7 for everyone, all times 2 because there are both
       allow and deny ACEs.  There are 6 words for owner because of
       type, flag, mask, wholen, "OWNER@"+pad and similarly for group;
       everyone is another word to hold "EVERYONE@".  */
    let mut buf: [uint32_t; 2 * (6 + 6 + 7)] = [0; 2 * (6 + 6 + 7)];

    let ret: c_int = (if fd < 0
                      { (if (flags & ACL_SYMLINK_FOLLOW) != 0 { getxattr } else { lgetxattr })
                        (name, XATTR_NAME_NFSV4_ACL.as_ptr (), buf.as_mut_ptr() as *mut c_void,
                         std::mem::size_of_val (&buf)) }
                      else { fgetxattr (fd, XATTR_NAME_NFSV4_ACL.as_ptr (),
                                        buf.as_mut_ptr () as *mut c_void,
                                        std::mem::size_of_val (&buf)) }) as c_int;
    if ret < 0
      {
        match *errno ()
          {
            ENODATA => { return 0; }
            ERANGE => { return 1; }        /* ACL must be nontrivial.  */
            _ => { return - (acl_errno_valid (*errno ()) as c_int); }
          }
      }

    /* It looks like a trivial ACL, but investigate further.  */
    let ret2: c_int = acl_nfs4_nontrivial (buf.as_mut_ptr (), ret as c_long);
    *errno () = if ret2 < 0 { EINVAL } else { initial_errno };
    return ret2;
  }

  /* C 511-964: the `#else` arm marked `!USE_LINUX_XATTR` -- the live one on the frozen target.  */
  #[cfg (not (feature = "use_linux_xattr"))]
  {
    (*ai).buf = (*ai).u._gl_acl_ch.as_mut_ptr ();
    (*ai).size = -1;
    (*ai).u.err = ENOTSUP;
    (*ai).scontext = UNKNOWN_SECURITY_CONTEXT.as_ptr () as *mut c_char;
    (*ai).scontext_err = ENOTSUP;

    /* C 519-964: `# if USE_ACL` (dead on the frozen target, USE_ACL is 0).  */
    #[cfg (feature = "use_acl")]
    {
      /* C 519-602: `#  if HAVE_ACL_GET_FILE` -- Linux, FreeBSD, NetBSD >= 10, Mac OS X,
         Cygwin >= 2.5.  The four names the arms read but the frozen <sys/acl.h> mirror does
         not carry are declared once here, UNVERIFIED (they come from a different target's
         header; the arm's condition says which).  */
      #[cfg (feature = "use_acl_have_acl_get_file")]
      {
        use acl_get_file_surface::*;

        {
          /* POSIX 1003.1e (draft 17 -- abandoned) specific version.  */
          /* Linux, FreeBSD, NetBSD >= 10, Mac OS X, Cygwin >= 2.5 */
          let mut ret: c_int;

          /* C 527-536: `#   if HAVE_ACL_EXTENDED_FILE` (Linux).  */
          #[cfg (feature = "have_acl_extended_file")]
          {
            /* On Linux, acl_extended_file is an optimized function: It only
               makes two calls to getxattr(), one for ACL_TYPE_ACCESS, one for
               ACL_TYPE_DEFAULT.  */
            ret = (if fd < 0
                   { (if (flags & ACL_SYMLINK_FOLLOW) != 0 { acl_extended_file }
                      else { acl_extended_file_nofollow }) (name) }
                   else { acl_extended_fd (fd) });
          }

          /* C 537-554: `#   elif HAVE_ACL_TYPE_EXTENDED` (Mac OS X).  */
          #[cfg (all (not (feature = "have_acl_extended_file"),
                      feature = "have_acl_type_extended"))]
          {
            /* On Mac OS X, acl_get_file (name, ACL_TYPE_ACCESS)
               and acl_get_file (name, ACL_TYPE_DEFAULT)
               always return NULL / EINVAL.  There is no point in making
               these two useless calls.  The real ACL is retrieved through
               ACL_TYPE_EXTENDED.  */
            let acl: acl_t =
              (if fd < 0
               { (if (flags & ACL_SYMLINK_FOLLOW) != 0 { acl_get_file } else { acl_get_link_np })
                 (name, ACL_TYPE_EXTENDED) }
               else { acl_get_fd_np (fd, ACL_TYPE_EXTENDED) });
            if ! acl.is_null ()
              {
                ret = acl_extended_nontrivial (acl);
                acl_free (acl);
              }
            else
              { ret = -1; }
          }

          /* C 555-599: `#   else` (FreeBSD, NetBSD >= 10, Cygwin >= 2.5).  */
          #[cfg (all (not (feature = "have_acl_extended_file"),
                      not (feature = "have_acl_type_extended"),
                      not (feature = "use_linux_xattr")))]
          {
            let mut acl: acl_t = acl_get_fdfile (fd, name, ACL_TYPE_ACCESS, flags);
            if ! acl.is_null ()
              {
                ret = acl_access_nontrivial (acl);
                let mut saved_errno: c_int = *errno ();
                acl_free (acl);
                *errno () = saved_errno;
                /* On Linux, FreeBSD, NetBSD,
                       acl_get_file (name, ACL_TYPE_ACCESS)
                   and acl_get_file (name, ACL_TYPE_DEFAULT) on a directory
                   either both succeed or both fail; it depends on the
                   file system.  Therefore there is no point in making the second
                   call if the first one already failed.  */
                if ret == 0
                  && ((d_type as c_int) == DT_DIR
                      || ((d_type as c_int) == DT_UNKNOWN
                          && (flags & _GL_DT_NOTDIR) == 0))
                  {
                    acl = acl_get_fdfile (fd, name, ACL_TYPE_DEFAULT, flags);
                    if ! acl.is_null ()
                      {
                        /* C 577-585: `#    ifdef __CYGWIN__` / `#    else` / `#    endif`.  */
                        #[cfg (feature = "cygwin")]
                        {
                          ret = acl_access_nontrivial (acl);
                          saved_errno = *errno ();
                          acl_free (acl);
                          *errno () = saved_errno;
                        }
                        #[cfg (not (feature = "cygwin"))]
                        {
                          ret = (0 < acl_entries (acl)) as c_int;
                          acl_free (acl);
                        }
                      }
                    else
                      {
                        ret = -1;
                        /* C 590-593: `#    ifdef __CYGWIN__`.  */
                        #[cfg (feature = "cygwin")]
                        {
                          if (d_type as c_int) == DT_UNKNOWN
                            { ret = 0; }
                        }
                      }
                  }
              }
            else
              { ret = -1; }
          }

          return if ret < 0 { - (acl_errno_valid (*errno ()) as c_int) } else { ret };
        }
      }

      /* C 604-962: `#  else`, marked `!HAVE_ACL_GET_FILE`.  The remaining APIs always follow
         symlinks and operate on platforms where symlinks do not have ACLs, so C skips them if
         NAME is known to be a symlink.  Each `#elif` arm declares the foreign platform surface
         its own statements read (P-88 clause 1); none of it is on this machine, so every value
         and layout below is UNVERIFIED, and the arm's condition -- not this target -- says
         which prototype is live.  */
      #[cfg (not (feature = "use_acl_have_acl_get_file"))]
      {
        if (d_type as c_int) != DT_LNK
          {
            /* C 612-777: `#   if HAVE_FACL && defined GETACL` -- Solaris, Cygwin < 2.5, not
               HP-UX.  */
            #[cfg (feature = "use_acl_getacl")]
            {
              use solaris_acl_surface::*;

              /* C 614-622: `#    ifdef ACL_NO_TRIVIAL` -- Solaris 10 (newer version).  */
              #[cfg (feature = "acl_no_trivial")]
              {
                /* C 622.  */
                return acl_trivial (name);
              }

              /* C 624-776: `#    else`, marked `Solaris, Cygwin, general case`.  */
              #[cfg (not (feature = "acl_no_trivial"))]
              {
                /* C 629-697.  */
                {
                  /* Initially, try to read the entries into a stack-allocated buffer.
                     Use malloc if it does not fit.  */
                  const alloc_init: usize = 4000 / std::mem::size_of::<aclent_t> ();
                  const alloc_max: usize
                    = MIN! (INT_MAX as usize, SIZE_MAX / std::mem::size_of::<aclent_t> ());
                  let mut buf: [aclent_t; alloc_init]
                    = [aclent_t { a_type: 0, a_id: 0, a_perm: 0 }; alloc_init];
                  let mut alloc: usize = alloc_init;
                  let mut entries: *mut aclent_t = buf.as_mut_ptr ();
                  let mut malloced: *mut aclent_t = std::ptr::null_mut ();
                  let mut count: c_int;

                  loop
                    {
                      count = (if fd < 0
                               { acl (name as *mut c_char, GETACL, alloc as c_int,
                                      entries as *mut c_void) }
                               else { facl (fd, GETACL, alloc as c_int,
                                            entries as *mut c_void) });
                      if count < 0 && *errno () == ENOSPC
                        {
                          /* Increase the size of the buffer.  */
                          free (malloced as *mut c_void);
                          if alloc > alloc_max / 2
                            {
                              *errno () = ENOMEM;
                              return -1;
                            }
                          alloc = 2 * alloc; /* <= alloc_max */
                          malloced = malloc (alloc * std::mem::size_of::<aclent_t> ())
                                     as *mut aclent_t;
                          entries = malloced;
                          if entries.is_null ()
                            { return -1; }
                        }
                      else
                        { break; }
                    }
                  if count < 0
                    {
                      if *errno () == ENOSYS || *errno () == ENOTSUP
                        { }
                      else
                        {
                          free (malloced as *mut c_void);
                          return -1;
                        }
                    }
                  else if count == 0
                    { }
                  else
                    {
                      /* Don't use MIN_ACL_ENTRIES:  It's set to 4 on Cygwin, but Cygwin
                         returns only 3 entries for files with no ACL.  But this is safe:
                         If there are more than 4 entries, there cannot be only the
                         "user::", "group::", "other:", and "mask:" entries.  */
                      if count > 4
                        {
                          free (malloced as *mut c_void);
                          return 1;
                        }

                      if acl_nontrivial (count, entries) != 0
                        {
                          free (malloced as *mut c_void);
                          return 1;
                        }
                    }
                  free (malloced as *mut c_void);
                }

                /* C 699-774: `#     ifdef ACE_GETACL` -- Solaris also has a different variant
                   of ACLs, used in ZFS and NFSv4 file systems.  */
                #[cfg (feature = "use_acl_ace_getacl")]
                {
                  /* C 702-773.  */
                  {
                    /* Initially, try to read the entries into a stack-allocated buffer.
                       Use malloc if it does not fit.  */
                    const alloc_init: usize = 4000 / std::mem::size_of::<ace_t> ();
                    const alloc_max: usize
                      = MIN! (INT_MAX as usize, SIZE_MAX / std::mem::size_of::<ace_t> ());
                    let mut buf: [ace_t; alloc_init]
                      = [ace_t { a_who: 0, a_access_mask: 0, a_flags: 0, a_type: 0 };
                         alloc_init];
                    let mut alloc: usize = alloc_init;
                    let mut entries: *mut ace_t = buf.as_mut_ptr ();
                    let mut malloced: *mut ace_t = std::ptr::null_mut ();
                    let mut count: c_int;

                    loop
                      {
                        count = (if fd < 0
                                 { acl (name as *mut c_char, ACE_GETACL, alloc as c_int,
                                        entries as *mut c_void) }
                                 else { facl (fd, ACE_GETACL, alloc as c_int,
                                              entries as *mut c_void) });
                        if count < 0 && *errno () == ENOSPC
                          {
                            /* Increase the size of the buffer.  */
                            free (malloced as *mut c_void);
                            if alloc > alloc_max / 2
                              {
                                *errno () = ENOMEM;
                                return -1;
                              }
                            alloc = 2 * alloc; /* <= alloc_max */
                            malloced = malloc (alloc * std::mem::size_of::<ace_t> ())
                                       as *mut ace_t;
                            entries = malloced;
                            if entries.is_null ()
                              { return -1; }
                          }
                        else
                          { break; }
                      }
                    if count < 0
                      {
                        if *errno () == ENOSYS || *errno () == EINVAL
                          { }
                        else
                          {
                            free (malloced as *mut c_void);
                            return -1;
                          }
                      }
                    else if count == 0
                      { }
                    else
                      {
                        /* In the old (original Solaris 10) convention:
                           If there are more than 3 entries, there cannot be only the
                           ACE_OWNER, ACE_GROUP, ACE_OTHER entries.
                           In the newer Solaris 10 and Solaris 11 convention:
                           If there are more than 6 entries, there cannot be only the
                           ACE_OWNER, ACE_GROUP, ACE_EVERYONE entries, each once with
                           NEW_ACE_ACCESS_ALLOWED_ACE_TYPE and once with
                           NEW_ACE_ACCESS_DENIED_ACE_TYPE.  */
                        if count > 6
                          {
                            free (malloced as *mut c_void);
                            return 1;
                          }

                        if acl_ace_nontrivial (count, entries) != 0
                          {
                            free (malloced as *mut c_void);
                            return 1;
                          }
                      }
                    free (malloced as *mut c_void);
                  }
                }

                /* C 776.  */
                return 0;
              }
            }

            /* C 779-858: `#   elif HAVE_GETACL` -- HP-UX.  */
            #[cfg (all (not (feature = "use_acl_getacl"), feature = "use_acl_have_getacl"))]
            {
              use hpux_acl_surface::*;

              /* C 781-821.  */
              {
                let mut entries: [acl_entry; NACLENTRIES] = [acl_entry { ae_id: 0 }; NACLENTRIES];
                let count: c_int = (if fd < 0
                                    { getacl (name, NACLENTRIES as c_int,
                                              entries.as_mut_ptr ()) }
                                    else { fgetacl (fd, NACLENTRIES as c_int,
                                                    entries.as_mut_ptr ()) });

                if count < 0
                  {
                    /* ENOSYS is seen on newer HP-UX versions.
                       EOPNOTSUPP is typically seen on NFS mounts.
                       ENOTSUP was seen on Quantum StorNext file systems (cvfs).  */
                    if *errno () == ENOSYS || *errno () == EOPNOTSUPP || *errno () == ENOTSUP
                      { }
                    else
                      { return -1; }
                  }
                else if count == 0
                  { return 0; }
                else /* count > 0 */
                  {
                    if count > NACLENTRIES as c_int
                      /* If NACLENTRIES cannot be trusted, use dynamic memory
                         allocation.  */
                      { abort (); }

                    /* If there are more than 3 entries, there cannot be only the
                       (uid,%), (%,gid), (%,%) entries.  */
                    if count > 3
                      { return 1; }

                    {
                      let mut statbuf: stat = std::mem::zeroed ();

                      if (if fd < 0 { stat (name, &mut statbuf) }
                          else { fstat (fd, &mut statbuf) }) < 0
                        && *errno () != EOVERFLOW
                        { return -1; }

                      return acl_nontrivial (count, entries.as_mut_ptr ());
                    }
                  }
              }

              /* C 823-858: `#    if HAVE_ACLV_H` (HP-UX >= 11.11).  */
              #[cfg (feature = "have_aclv_h")]
              {
                /* C 825-856.  */
                {
                  let mut entries: [acl; NACLVENTRIES] = [acl { a_room: [0; 12] }; NACLVENTRIES];

                  /* Ignore FD, unfortunately.  */
                  let count: c_int = acl (name as *mut c_char, ACL_GET, NACLVENTRIES as c_int,
                                          entries.as_mut_ptr ());

                  if count < 0
                    {
                      /* EOPNOTSUPP is seen on NFS in HP-UX 11.11, 11.23.
                         EINVAL is seen on NFS in HP-UX 11.31.  */
                      if *errno () == ENOSYS || *errno () == EOPNOTSUPP || *errno () == EINVAL
                        { }
                      else
                        { return -1; }
                    }
                  else if count == 0
                    { return 0; }
                  else /* count > 0 */
                    {
                      if count > NACLVENTRIES as c_int
                        /* If NACLVENTRIES cannot be trusted, use dynamic memory
                           allocation.  */
                        { abort (); }

                      /* If there are more than 4 entries, there cannot be only the
                         four base ACL entries.  */
                      if count > 4
                        { return 1; }

                      return aclv_nontrivial (count, entries.as_mut_ptr ());
                    }
                }
              }
            }

            /* C 860-915: `#   elif HAVE_ACLX_GET && defined ACL_AIX_WIP` -- AIX.  */
            #[cfg (all (not (feature = "use_acl_getacl"),
                        not (feature = "use_acl_have_getacl"),
                        feature = "have_aclx_get", feature = "acl_aix_wip"))]
            {
              use aix_acl_surface::*;

              /* C 862-915.  */
              let mut r#type: acl_type_t = acl_type_t { u64: 0 };
              let mut aclbuf: [c_char; 1024] = [0; 1024];
              let mut acl: *mut c_void = aclbuf.as_mut_ptr () as *mut c_void;
              let mut aclsize: usize = std::mem::size_of_val (&aclbuf);

              loop
                {
                  /* The docs say that type being 0 is equivalent to ACL_ANY, but it
                     is not true, in AIX 5.3.  */
                  r#type.u64 = ACL_ANY;
                  let mut mode: mode_t = 0;
                  if 0 <= (if fd < 0
                           { aclx_get (name, 0, &mut r#type, acl as *mut c_char, &mut aclsize,
                                       &mut mode) }
                           else { aclx_fget (fd, 0, &mut r#type, acl as *mut c_char,
                                             &mut aclsize, &mut mode) })
                    { break; }
                  if *errno () == ENOSYS
                    { return 0; }
                  if *errno () != ENOSPC
                    {
                      if acl != aclbuf.as_mut_ptr () as *mut c_void
                        { free (acl); }
                      return -1;
                    }
                  aclsize = 2 * aclsize;
                  if acl != aclbuf.as_mut_ptr () as *mut c_void
                    { free (acl); }
                  acl = malloc (aclsize);
                  if acl.is_null ()
                    { return -1; }
                }

              if r#type.u64 == ACL_AIXC
                {
                  let result: c_int = acl_nontrivial (acl as *mut acl);
                  if acl != aclbuf.as_mut_ptr () as *mut c_void
                    { free (acl); }
                  return result;
                }
              else if r#type.u64 == ACL_NFS4
                {
                  /* C 902's `acl_nfs4_nontrivial` is the reader C defines only inside
                     `#if USE_LINUX_XATTR` (C 317); its declaration is in the surface above.  */
                  let result: c_int = acl_nfs4_nontrivial (acl as *mut uint32_t, aclsize as c_long);
                  if acl != aclbuf.as_mut_ptr () as *mut c_void
                    { free (acl); }
                  return result;
                }
              else
                {
                  /* A newer type of ACL has been introduced in the system.
                     We should better support it.  */
                  if acl != aclbuf.as_mut_ptr () as *mut c_void
                    { free (acl); }
                  *errno () = EINVAL;
                  return -1;
                }
            }

            /* C 917-927: `#   elif HAVE_STATACL` -- older AIX.  */
            #[cfg (all (not (feature = "use_acl_getacl"), not (feature = "use_acl_have_getacl"),
                        not (feature = "have_aclx_get"), feature = "use_acl_have_statacl"))]
            {
              use aix_statacl_surface::*;

              /* C 919-927.  */
              {
                let mut u: acl = acl { a_room: [0; 4096] };

                if (if fd < 0
                    { statacl (name as *mut c_char, STX_NORMAL,
                               &mut u as *mut acl, std::mem::size_of_val (&u)) }
                    else { fstatacl (fd, STX_NORMAL, &mut u as *mut acl,
                                     std::mem::size_of_val (&u)) })
                   < 0
                  { return -1; }

                return acl_nontrivial (&mut u as *mut acl);
              }
            }

            /* C 929-960: `#   elif HAVE_ACLSORT` -- NonStop Kernel.  */
            #[cfg (all (not (feature = "use_acl_getacl"), not (feature = "use_acl_have_getacl"),
                        not (feature = "have_aclx_get"), not (feature = "use_acl_have_statacl"),
                        feature = "use_acl_have_aclsort"))]
            {
              use nonstop_acl_surface::*;

              /* C 931-960.  */
              {
                let mut entries: [acl; NACLENTRIES] = [acl { a_room: [0; 12] }; NACLENTRIES];

                /* Ignore FD, unfortunately.  */
                let count: c_int = acl (name as *mut c_char, ACL_GET, NACLENTRIES as c_int,
                                        entries.as_mut_ptr () as *mut c_void);

                if count < 0
                  {
                    if *errno () == ENOSYS || *errno () == ENOTSUP
                      { }
                    else
                      { return -1; }
                  }
                else if count == 0
                  { return 0; }
                else /* count > 0 */
                  {
                    if count > NACLENTRIES as c_int
                      /* If NACLENTRIES cannot be trusted, use dynamic memory
                         allocation.  */
                      { abort (); }

                    /* If there are more than 4 entries, there cannot be only the
                       four base ACL entries.  */
                    if count > 4
                      { return 1; }

                    return acl_nontrivial (count, entries.as_mut_ptr ());
                  }
              }
            }
          }
      }
    }
  }

  /* C 967.  */
  return 0;

  }
}

/* Return 1 if NAME has a nontrivial access control list,
   0 if ACLs are not supported, or if NAME has no or only a base ACL,
   and -1 (setting errno) on error.  Note callers can determine
   if ACLs are not supported as errno is set in that case also.
   SB must be set to the stat buffer of NAME,
   obtained through stat() or lstat().  */

/* C 976-986.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_has_acl (name: *const c_char, sb: *const stat) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let mut flags: c_int = IFTODT! ((*sb).st_mode);
  if ! S_ISDIR! ((*sb).st_mode)
    { flags |= _GL_DT_NOTDIR; }
  let mut ai: aclinfo = std::mem::zeroed ();
  let r: c_int = file_has_aclinfo (name, &mut ai, flags);
  /* C 985: `aclinfo_free (&ai);` - a *function* under C 88's `HAVE_LINUX_XATTR_H &&
     HAVE_LISTXATTR` (this unit defines it above) and `<acl.h>` 99-104's `#else` *macro*
     otherwise.  Each arm spells what its own arm of C spells; the two share a name in Rust
     (macros live in their own namespace), so only the call site has to say which one it means -
     status 3.511 measured the macro silently answering this live call site before.  */
  #[cfg (feature = "use_linux_xattr")]
  aclinfo_free (&mut ai);
  #[cfg (not (feature = "use_linux_xattr"))]
  aclinfo_free! (&mut ai);
  return r;

  }
}
