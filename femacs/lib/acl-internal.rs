// femacs: port of lib/acl-internal.c @ fd7afced403 (gnulib).
//
// The frozen configuration resolves all 22 of this file's conditions
// (`tools/check-cond-eval.py lib/acl-internal.c`: 0 unknown, 0 disagreements), and USE_ACL
// is 0, so C 34-492's whole `#if USE_ACL && …` chain — the `HAVE_ACL_GET_FILE` arms (macOS's
// `ACL_TYPE_EXTENDED` and the Linux/FreeBSD/NetBSD/Cygwin `HAVE_ACL_FIRST_ENTRY` one with its
// `__CYGWIN__` half), the `HAVE_FACL && GETACL` arm with its `ACE_GETACL` half, the
// `HAVE_GETACL` arm with its `HAVE_ACLV_H` half, the AIX `ACLX_GET || STATACL` arm and the
// NonStop `HAVE_ACLSORT` arm — is dead, as is `free_permission_context`'s own `#if USE_ACL`
// body (C 497-522).  What the frozen configuration keeps is C 20's qualifier and C 494-523's
// empty-bodied `free_permission_context`.
//
// C 28-32's `#if defined __CYGWIN__` includes (`<sys/types.h>`, `<grp.h>`, `<string.h>`) are
// dead too; they belong to the dead arms above.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort,
               c_void};
include!("../src/include/config.rs");  /* C 21: `#include <config.h>`.  */
include!("include/gnulib_config.rs");
include!("include/acl.rs");            /* C 26: `#include "acl.h"`.  */
include!("include/acl-internal.rs");   /* C 24: `#include "acl-internal.h"`.  */

/* C 20: `#define ACL_INTERNAL_INLINE _GL_EXTERN_INLINE` — this unit is the header's wrapper
   (PORTING.md 10.12), and its own definition here shadows the mirror's `ACL_INTERNAL_INLINE`
   in `acl_internal_decls` (the whole-import module's item loses to a definition, PORTING.md
   0.1-10; it is also why clang sees the header's `#ifndef` as false).  */
pub const ACL_INTERNAL_INLINE: () = _GL_EXTERN_INLINE;

/* C 494-523: the definition C 497-522's `#if USE_ACL` chain leaves with an empty body on this
   target.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_permission_context (ctx: *mut permission_context)
{
  /* C 497-522's `#if USE_ACL` body is dead (USE_ACL is 0), so C's live body is empty.  The
     port keeps the body's text under the feature that names this target's other side (P-51):
     it is never compiled, and it is what the token/stub gate sees instead of an empty body.  */
  #[cfg (feature = "use_acl")]
  {
    #[cfg (feature = "use_acl_have_acl_get_file")]
    {
      if ! (*ctx).acl.is_null ()
        { acl_free ((*ctx).acl); }
      if ! (*ctx).default_acl.is_null ()
        { acl_free ((*ctx).default_acl); }
    }

    #[cfg (feature = "use_acl_getacl")]
    {
      free ((*ctx).entries as *mut c_void);
      free ((*ctx).ace_entries as *mut c_void);
    }

    /* C 504-522's HP-UX (`HAVE_ACLV_H`), older-AIX (`HAVE_STATACL`) and NonStop
       (`HAVE_ACLSORT`) arms are empty.  */
  }
}
