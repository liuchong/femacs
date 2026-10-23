// femacs: port of lib/get-permissions.c @ fd7afced403 (gnulib).
//
// The frozen configuration resolves all 10 of this file's conditions
// (`tools/check-cond-eval.py lib/get-permissions.c`: 0 unknown, 0 disagreements), and
// USE_ACL is 0, so the whole `#if USE_ACL` chain (C 39-287: the HAVE_ACL_GET_FILE arms for
// Linux/FreeBSD/macOS and for macOS's ACL_TYPE_EXTENDED, the GETACL arm for Solaris and
// Cygwin with its ACE_GETACL half, the HAVE_GETACL arm for HP-UX with its HAVE_ACLV_H half,
// the AIX AIX_WIP TODO, the older-AIX HAVE_STATACL arm and the NonStop HAVE_ACLSORT arm) is
// dead: only C 33-37's doc comment, the signature, C 38-39's two statements and C 289's
// `return 0;` are live.  The dead chain is noted where C's `#if` stood rather than written
// out (P-51's text is optional for a file whose conditions the evaluator decides).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort,
               c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/string.rs");         /* C 22: `<string.h>` (memset).  */
include!("include/acl.rs");            /* C 23: `#include "acl.h"`.  */
include!("include/acl-internal.rs");   /* C 25: `#include "acl-internal.h"`.  */

/* Read the permissions of a file into CTX. If DESC is a valid file descriptor,
   use file descriptor operations, else use filename based operations on NAME.
   MODE is the file mode obtained from a previous stat call.
   Return 0 if successful.  Return -1 and set errno upon failure.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_permissions (name: *const c_char, desc: c_int, mode: mode_t,
                                          ctx: *mut permission_context) -> c_int
{
  memset (ctx as *mut c_void, 0, std::mem::size_of::<permission_context> ());
  (*ctx).mode = mode;

  /* C 39-287's `#if USE_ACL && …` chain is dead (USE_ACL is 0); its arms named
     `acl_get_fd`/`acl_get_file`/`acl_get_file (ACL_TYPE_NFS4)`, `acl`/`aclsort`,
     `getacl`/`getaclcnt`/`aclv`/`aclvsort`, `fstatacl`/`statacl`, the ACE_GETACL
     `ace_getacl`/`ace_getaclcnt` pair and their `ACE_*`/`ACE_GETACL` spellings, along with
     `S_ISDIR`, `NACLENTRIES`, `NACLVENTRIES`, `STX_NORMAL`, `ACL_TYPE_ACCESS`,
     `ACL_TYPE_NFS4` and `ACL_AIX_WIP`.  */

  return 0;
}
