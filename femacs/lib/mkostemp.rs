/* Copyright (C) 1998-1999, 2001, 2005-2007, 2009-2026 Free Software
   Foundation, Inc.
   This file is derived from the one in the GNU C Library.

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
// femacs: port of lib/mkostemp.c @ fd7afced403 (GNU Emacs 32.0.50).  Blind file (not in
// cond-eval-verified.txt): C 18-20's `#if !_LIBC` include of <config.h> is live, C 24-30's
// `#if !_LIBC` block is live (so `__gen_tempname` is `gen_tempname` and `__GT_FILE` is
// `GT_FILE`), and C 34-36's `#ifndef __GT_FILE` arm is dead but carried.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/stdlib.rs");     // <stdlib.h>
include!("include/stdio.rs");
include!("include/tempname.rs");   // "tempname.h" (`GT_FILE`, `gen_tempname`)


/* C 26's `# define __gen_tempname gen_tempname`: this target's preprocessor renames the
   call below to the system replacement, which tempname.h declares and lib/tempname.c
   defines (`#[unsafe(export_name = "gen_tempname")]` there).  The macro keeps its row in
   the ledger as a nullary macro whose body is the name (the strtoimax.rs `Strtoimax`
   shape), so the call site is `__gen_tempname! () (...)`.  */
macro_rules! __gen_tempname { () => { gen_tempname }; }

/* C 27-29's `#ifndef __GTFILE / # define __GT_FILE GT_FILE / #endif` is live (`__GTFILE`
   is a name only glibc's build defines); C 34-36's `#ifndef __GT_FILE / # define __GT_FILE
   0 / #endif` is dead, and the extractor keeps the last definition, so the ledger compares
   presence only (the branch-macro rule, PORTING.md P-08).  Both spell 0 on this target.  */
pub const __GT_FILE: c_int = GT_FILE;

/* int mkostemp (char *xtemplate, int flags)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mkostemp (xtemplate: *mut c_char, flags: c_int) -> c_int
{
  return __gen_tempname! () (xtemplate, 0, flags, __GT_FILE);
}
