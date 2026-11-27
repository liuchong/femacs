// femacs: port of lib/set-permissions.c @ fd7afced403 (gnulib).
//
// The frozen configuration resolves all 25 of this file's conditions
// (`tools/check-cond-eval.py lib/set-permissions.c`: 0 unknown, 0 disagreements), and USE_ACL
// is 0, so C 29-472's whole `#if USE_ACL` chain (the FreeBSD/Cygwin `HAVE_ACL_FROM_TEXT` arm,
// the Solaris/Cygwin `HAVE_FACL && GETACL` arm with its `ACE_GETACL` half, the HP-UX
// `HAVE_GETACL` arm with its `HAVE_ACLV_H` half, the AIX `ACLX_GET && ACL_AIX_WIP` arm, the
// older-AIX `HAVE_STATACL` arm and the NonStop `HAVE_ACLSORT` arm — `set_acls_*` and the
// `set_acls_from_mode` chain) is dead, as are `set_acls` (C 476-735) and the two `#if USE_ACL`
// blocks inside `set_permissions` (C 771-790's `early_chmod` computation and C 800-820's
// `set_acls` calls).  What is live: C 747-755's `chmod_or_fchmod` and C 792-835's
// `set_permissions` body without those blocks.
//
// `HAVE_FCHMOD` and the other frozen ACL/permission constants are not in the generated
// `src/include/config.rs` (its curated list stops short of them), so `HAVE_FCHMOD` is
// declared here with the value the frozen configuration gives it (docs/baseline/config/
// darwin-arm64.macros.tsv) — the constant the live `if` reads.
// [U1-MODULE-UNIT]
use crate::sys_common::*;
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort,
               c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/errno.rs imported via common
// [U1] include/stdcountof.rs imported via common
// [U1] include/sys_types.in.rs imported via common
// [U1] include/sys_stat.in.rs imported via common
// [U1] include/acl.rs imported via common
// [U1] include/acl-internal.rs imported via common
/* C 27's `#include "minmax.h"` is only used by the dead arms above, so this unit does not
   carry the mirror.  */

/* The frozen configuration's `HAVE_FCHMOD` (the macro table's 738th row is `HAVE_FCHMOD 1`),
   which C's `chmod_or_fchmod` reads.  */
pub const HAVE_FCHMOD: c_int = 1;

/* If DESC is a valid file descriptor use fchmod to change the
   file's mode to MODE on systems that have fchmod. On systems
   that don't have fchmod and if DESC is invalid, use chmod on
   NAME instead.
   Return 0 if successful.  Return -1 and set errno upon failure.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn chmod_or_fchmod (name: *const c_char, desc: c_int, mode: mode_t)
  -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  if HAVE_FCHMOD != 0 && desc != -1
    { return fchmod (desc, mode); }
  else
    { return chmod (name, mode); }

  }
}

/* Set the permissions in CTX on a file. If DESC is a valid file descriptor,
   use file descriptor operations, else use filename based operations on NAME.
   If access control lists are not available, fchmod the target file to the
   mode in CTX.  Also sets the non-permission bits of the destination file
   (S_ISUID, S_ISGID, S_ISVTX) to those from the mode in CTX if any are set.
   Return 0 if successful.  Return -1 and set errno upon failure.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_permissions (ctx: *mut permission_context, name: *const c_char,
                                          desc: c_int) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let acls_set: bool = false;
  let early_chmod: bool;
  let mut must_chmod: bool = false;

  /* C 771-790's `#if USE_ACL` arm (the `HAVE_STATACL` TODO and the
     `! MODE_INSIDE_ACL || (ctx->mode & (S_ISUID | S_ISGID | S_ISVTX))` computation) is dead
     (USE_ACL is 0); this is C 786-789's `#else` arm.  */
  /* No ACLs */
  early_chmod = true;

  if early_chmod
    {
      let ret: c_int = chmod_or_fchmod (name, desc, (*ctx).mode);
      if ret != 0
        { return -1; }
    }

  let mut ret: c_int = 0;
  /* C 800-820's `#if USE_ACL` arm is dead (USE_ACL is 0): its two `set_acls` calls with the
     `acls_set`/`must_chmod` bookkeeping.  */

  if must_chmod && ! early_chmod
    {
      let saved_errno: c_int = if ret != 0 { *errno () } else { 0 };

      ret = chmod_or_fchmod (name, desc, (*ctx).mode);

      if saved_errno != 0
        {
          *errno () = saved_errno;
          ret = -1;
        }
    }

  return ret;

  }
}
