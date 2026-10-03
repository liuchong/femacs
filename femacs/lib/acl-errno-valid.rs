/* Test whether ACLs are well supported on this system.

   Copyright 2013-2026 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.

   Written by Paul Eggert.  */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
// femacs: port of lib/acl-errno-valid.c @ fd7afced403 .  Blind file; the
// two conditions are both live on this target (`__APPLE__`/`__MACH__` are defined, and
// `ENOTSUP` (45) differs from both `EOPNOTSUPP` (102) and `ENOSYS` (78)), so every `case` in
// C's `switch` is reachable.  C's `switch`/`case`/`default` become the port's `match`/pattern
// arms/`_` (PORTING.md 2.2; the switch tokens are in docs/ports/macro-body-notes.tsv).
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");      // <errno.h> (the nine codes)
include!("include/acl.rs");        // <acl.h>

/* bool acl_errno_valid (int errnum)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acl_errno_valid (errnum: c_int) -> bool
{
  /* Recognize some common errors such as from an NFS mount that does
     not support ACLs, even when local drives do.  */
  match errnum
    {
    EBUSY => { return false; }
    EINVAL => { return false; }
    /* C 38-40's `#if defined __APPLE__ && defined __MACH__` arm (both are defined).  */
    ENOENT => { return false; }
    ENOSYS => { return false; }
    /* C 43-47's `#if defined ENOTSUP && ENOTSUP != EOPNOTSUPP` and its inner
       `# if ENOTSUP != ENOSYS` (both hold on this target).  */
    ENOTSUP => { return false; }

    EOPNOTSUPP => { return false; }
    _ => { return true; }
    }
}
