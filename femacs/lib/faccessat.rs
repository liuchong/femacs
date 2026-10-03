// femacs: port of lib/faccessat.c @ fd7afced403.
//
// `lib/faccessat.c` is in `docs/baseline/config/cond-eval-verified.txt`, so the gate evaluates
// its `#if`s against the frozen configuration and compares only the live text.  The live arms
// are `HAVE_FACCESSAT` (1) - `orig_faccessat` (C 36-40) and `rpl_faccessat` (C 55-78) - and
// the file's own `#ifndef HAVE_ACCESS` (C 46-51), whose `# undef access` / `# define access
// euidaccess` pair the frozen table leaves live (`HAVE_ACCESS` is undefined).  C 80-98's
// `#else` arm (the at-func.c template with `AT_FUNC_NAME` = `faccessat`) is dead and is not
// expanded (P-51): its six `AT_FUNC_*` renames are kept as `#[cfg]` text, and the
// `#include "at-func.c"` expansion - save_cwd/fchdir/(access|euidaccess)/restore_cwd, whose
// headers this target has not got - is named at its site rather than written out (the
// fstatat.c/gettimeofday.c treatment).
//
// Deviation worth a note (docs/ports/faccessat.md): the C file arranges for its first
// `<unistd.h>` (C 26) to be the *system* one via C 22's `#define _GL_INCLUDING_UNISTD_H`, so
// `orig_faccessat`'s `faccessat` is the system entry - the sys_stat mirror's declaration -
// and `rpl_faccessat` is this unit's own symbol.  `REPLACE_FACCESSAT` is not set, so nothing
// in the headers renames `faccessat` to `rpl_faccessat`.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Check the access rights of a file relative to an open directory.
   Copyright (C) 2009-2026 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* written by Eric Blake */

use std::ffi::{c_char, c_int, c_long, c_uint, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");
include!("include/fcntl.in.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/string.rs");
include!("include/unistd.in.rs");

/* C 22's `#define _GL_INCLUDING_UNISTD_H` (empty body; the port keeps the name, the
   utimens.c `_GL_UTIMENS_INLINE` shape).  C 33's `#undef _GL_INCLUDING_UNISTD_H` has no
   counterpart: the include! graph names each mirror once (PORTING.md 0.1-4).  */
pub const _GL_INCLUDING_UNISTD_H: () = ();

/* C 23-32: <config.h>, the first (system) <unistd.h>, <errno.h>, <fcntl.h>, <stdlib.h>,
   <string.h> and <sys/stat.h> are the includes above; <stdlib.h> is needed only by the dead
   at-func.c expansion.  */

/* C 35-41's `#if HAVE_FACCESSAT` arm (live).  */
unsafe fn orig_faccessat (fd: c_int, name: *const c_char, mode: c_int, flag: c_int) -> c_int
{
  return faccessat (fd, name, mode, flag);
}

/* C 43-44's second <unistd.h> is the "Specification" include - the gnulib mirror above -
   and C 46-51's `#ifndef HAVE_ACCESS` (live: the frozen table leaves HAVE_ACCESS undefined)
   is the `access` rename.  The port keeps it as a `macro_rules!` twin with the same name and
   call shape, the dup2.rs `# define dup2 …` shape; nothing in the live text calls it.  */
macro_rules! access {
  ($file:expr, $mode:expr) => { euidaccess ($file, $mode) };
}

/* C 53-78's `#if HAVE_FACCESSAT` arm (live): the trailing-slash check, so that a
   non-directory named with a trailing slash fails with ENOTDIR.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_faccessat (fd: c_int, file: *const c_char, mode: c_int,
                                        flags: c_int) -> c_int
{
  let mut result: c_int = orig_faccessat (fd, file, mode, flags);

  if ! file.is_null ()
    {
      let len: usize = strlen (file);
      if len != 0 && *file.add (len - 1) == b'/' as c_char
        {
          /* `struct stat st;` - written by fstatat before it is read (PORTING.md 1.5).  */
          let mut st: stat = std::mem::zeroed ();
          let ret: c_int = fstatat (fd, file, &mut st, 0);
          if ret == 0 && ! S_ISDIR! (st.st_mode)
            {
              *errno () = ENOTDIR;
              return -1;
            }
          if result == 0
            { result = ret; }
        }
    }

  return result;
}

/* C 80-98's `#else` arm, kept as text (P-51; dead: HAVE_FACCESSAT is 1).  The `# include
   "at-func.c"` expansion at C 96 is the /proc/self/fd simulation built on save_cwd.h,
   fchdir and restore_cwd, headers this target has not got, so it is not written out (the
   fstatat.c treatment); the `AT_FUNC_*` macros it is parameterised by are below.  */

/* C 90-95's `# define AT_FUNC_*` block: the at-func.c template's parameters.  */
#[cfg (feature = "no_faccessat")]
macro_rules! AT_FUNC_NAME { () => { faccessat } }
#[cfg (feature = "no_faccessat")]
macro_rules! AT_FUNC_F1 { () => { euidaccess } }
#[cfg (feature = "no_faccessat")]
macro_rules! AT_FUNC_F2 { () => { access } }
#[cfg (feature = "no_faccessat")]
macro_rules! AT_FUNC_USE_F1_COND { () => { AT_EACCESS } }
#[cfg (feature = "no_faccessat")]
macro_rules! AT_FUNC_POST_FILE_PARAM_DECLS { () => { , int mode, int flag } }
#[cfg (feature = "no_faccessat")]
macro_rules! AT_FUNC_POST_FILE_ARGS { () => { , mode } }
