/* Return the number of entries in an ACL.

   Copyright (C) 2002-2003, 2005-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Paul Eggert and Andreas Gruenbacher.  */

/* emacs/lib/acl_entries.c (@ 68 lines) is on the verified list: USE_ACL is 0 in the frozen
   configuration and HAVE_ACL_FIRST_ENTRY / HAVE_ACL_TYPE_EXTENDED are undefined, so C 40-65's
   two inner `#if` arms - the acl_get_entry loops - are dead by measurement and the live body is
   the `int count = 0; if (acl != NULL) { } return count;` skeleton.  C 22's "acl-internal.h" is
   not mirrored as a file yet (its first heavier consumers are qcopy-acl.c and
   get-permissions.c, which need `struct permission_context`): this unit names only `acl_t`,
   which acl-internal.h gets from <sys/acl.h>, so the new sys_acl mirror is included directly
   and docs/status.md 3.23 records the deferral.  */
use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/sys_acl.rs");

/* Return the number of entries in ACL.
   Return -1 and set errno upon failure to determine it.  */

/* C 33-67's `int acl_entries (acl_t acl)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acl_entries (acl: acl_t) -> c_int
{
  let mut count: c_int = 0;

  if !acl.is_null ()
    {
      /* C 41-64's `#if HAVE_ACL_FIRST_ENTRY` arm (and, inside it, the
         `#if HAVE_ACL_TYPE_EXTENDED` pair of listxattr-style fetches) is dead on this target:
         both macros are undefined, so the port carries neither `acl_get_entry` loop.  */
    }

  return count;
}
