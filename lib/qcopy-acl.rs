// femacs: port of lib/qcopy-acl.c @ fd7afced403 (gnulib).
//
// The frozen configuration resolves all 6 of this file's conditions
// (`tools/check-cond-eval.py lib/qcopy-acl.c`: 0 unknown, 0 disagreements), and USE_XATTR is
// undefined, so only the live text is written: C 26-59's `#if USE_XATTR` block (the
// `is_attr_permissions` predicate and the three `XATTR_NAME_*` defaults, which need
// `<attr/libattr.h>` and `<dirent.h>` — neither exists on this target) and the function's own
// `#ifdef USE_XATTR` arm (C 79-113, the `attr_copy_*` path) are dead, and the `#else` arm
// (C 115-123) is what remains.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort,
               c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/sys_types.in.rs imported via common
// [U1] include/acl.rs imported via common
// [U1] include/acl-internal.rs imported via common

/* Copy access control lists from one file to another. If SOURCE_DESC is
   a valid file descriptor, use file descriptor operations, else use
   filename based operations on SRC_NAME. Likewise for DEST_DESC and
   DST_NAME.
   MODE should be the source file's st_mode.
   If access control lists are not available, fchmod the target file to
   MODE.  Also sets the non-permission bits of the destination file
   (S_ISUID, S_ISGID, S_ISVTX) to those from MODE if any are set.
   Return 0 if successful.
   Return -2 and set errno for an error relating to the source file.
   Return -1 and set errno for an error relating to the destination file.  */

/* C 77's `int ret;` is deferred (the live arm's only assignment is C 120, before the
   return), so the port writes `let ret: c_int;` where C declares it (PORTING.md 1.5's
   "every branch assigns" case).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn qcopy_acl (src_name: *const c_char, source_desc: c_int,
                                    dst_name: *const c_char, dest_desc: c_int,
                                    mode: mode_t) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let ret: c_int;

  /* C 79-113's `#ifdef USE_XATTR` arm is dead (USE_XATTR is undefined): the
     `chmod_or_fchmod` + `attr_copy_file`/`attr_copy_fd` path with its `EOPNOTSUPP` retry
     through `fdfile_has_aclinfo` and `_GL_DT_NOTDIR`.  */
  /* no XATTR, so we proceed the old dusty way */
  /* C 116: `struct permission_context ctx;` — the C declaration has no initializer and
     `get_permissions` fills it, so the port zero-initializes it first (PORTING.md 1.5).  */
  let mut ctx: permission_context = std::mem::zeroed ();
  if get_permissions (src_name, source_desc, mode, &mut ctx) != 0
    { return -2; }

  ret = set_permissions (&mut ctx, dst_name, dest_desc);

  free_permission_context (&mut ctx);
  return ret;

  }
}
