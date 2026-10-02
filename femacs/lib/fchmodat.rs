// femacs: port of lib/fchmodat.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// `lib/fchmodat.c` is in `docs/baseline/config/cond-eval-verified.txt`.  `HAVE_FCHMODAT` is
// 1, so C 30-33's `orig_fchmodat` and C 64-153's `fchmodat` are the live text and C 155-159's
// `#else` arm (the at-func.c template, with `AT_FUNC_NAME` = `fchmodat`) is dead - it is not
// written out (P-51).  Inside the live function, `HAVE_NEARLY_WORKING_FCHMODAT` and
// `NEED_FCHMODAT_NONSYMLINK_FIX` are both undefined in the frozen configuration, so the
// trailing-slash correction and the O_PATH/`/proc/self/fd` non-symlink fix are dead too
// (named at their sites below); what remains is the `AT_EMPTY_PATH` argument handling plus
// the call to `orig_fchmodat`.
//
// The C file's first `<sys/stat.h>` include (C 26) is the *system* one, arranged by C 22's
// `#define __need_system_sys_stat_h`, which is why `orig_fchmodat` calls the system
// `fchmodat`; the port reaches it through `mod system` below, so the definition here does not
// recurse into itself (the utimens.rs `lstat` shape).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* fchmodat.c -- provide fchmodat, if missing
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

use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/errno.rs");
include!("include/fcntl.in.rs");
include!("include/sys_types.in.rs");
include!("include/string.rs");

/* C 22's `#define __need_system_sys_stat_h` (empty body; the port keeps the name).  */
pub const __need_system_sys_stat_h: () = ();

/* C 36-52's `<sys/stat.h>` entries this unit uses, in a module of their own: the sys_stat
   mirror declares `fchmodat` at the crate root, where this unit *defines* it (the frozen
   sys/stat.h renames nothing here, so C's symbol is the plain name), and two same-named
   items in one module are E0428 - the readlink.rs `mod system` shape keeps the system entry
   reachable without the clash.  */
mod system {
  use super::*;

  unsafe extern "C" {
    pub fn fchmodat (dir: c_int, file: *const c_char, mode: mode_t, flags: c_int) -> c_int;
    pub fn fchmod (fd: c_int, mode: mode_t) -> c_int;
  }
}

/* C 31-34's `orig_fchmodat` (live: HAVE_FCHMODAT is 1).  */
unsafe extern "C" fn orig_fchmodat (dir: c_int, file: *const c_char, mode: mode_t,
                                    flags: c_int) -> c_int
{
  return system::fchmodat (dir, file, mode, flags);
}

/* C 36-52: `<errno.h>`, `<fcntl.h>`, `<stdio.h>`, `<stdlib.h>`, `<string.h>`, `<unistd.h>`,
   the second `<sys/stat.h>`, `<intprops.h>` and `issymlinkat.h` are the mirrors above
   (`intprops`/`issymlinkat` are only used by the dead arms).  */

/* #ifndef AT_EMPTY_PATH
   # define AT_EMPTY_PATH 0
   #endif  */
pub const AT_EMPTY_PATH: c_int = 0;

/* Invoke chmod or lchmod on FILE, using mode MODE, in the directory
   open on descriptor FD.  If possible, do it without changing the
   working directory.  Otherwise, resort to using save_cwd/fchdir,
   then (chmod|lchmod)/restore_cwd.  If either the save_cwd or the
   restore_cwd fails, then give a diagnostic and exit nonzero.
   Fail if FLAGS contains AT_SYMLINK_NOFOLLOW on a system lacking
   lchmod support.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fchmodat (dir: c_int, file: *const c_char, mode: mode_t,
                                   flags: c_int) -> c_int
{
  let mut flags = flags;
  if (!file.is_null () && *file != 0)
    { flags &= !AT_EMPTY_PATH; }
  else if (flags & AT_EMPTY_PATH) == 0
    {
      *errno () = ENOENT;
      return -1;
    }
  else if (flags & AT_SYMLINK_NOFOLLOW) == 0 && 0 <= dir
    { return system::fchmod (dir, mode); }

  /* C 77-89's `# if HAVE_NEARLY_WORKING_FCHMODAT` arm (the trailing-slash correction) is
     dead: the macro is undefined.  */

  /* C 91-143's `# if NEED_FCHMODAT_NONSYMLINK_FIX` arm (the `O_PATH`/`/proc/self/fd`
     non-symlink fix built on `openat`, `issymlinkat`, `sprintf` and `chmod`) is dead: the
     macro is undefined.  */

  return orig_fchmodat (dir, file, mode, flags);
}
