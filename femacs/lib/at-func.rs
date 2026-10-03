// femacs: port of lib/at-func.c @ fd7afced403.
//
// gnulib compiles this source only through inclusion: the five consumers `faccessat.c`,
// `fchmodat.c`, `fstatat.c`, `readlinkat.c` and `utimensat.c` each `# define` the `AT_FUNC_*`
// bindings and `#include "at-func.c"`.  PORTING.md 10.13②'s shape for a template with no
// instantiation of its own is the prelude `macro_rules!` below; this file instantiates
// nothing.  The five consumers keep their own copies (`femacs/lib/{faccessat,fchmodat,
// fstatat,readlinkat,utimensat}.rs`): four only record the `AT_FUNC_*` parameters, because
// their `#include` arm is dead under the frozen configuration, and `utimensat.rs` writes the
// expansion out (see docs/ports/at-func.md).
//
// The frozen configuration resolves all 6 of C's conditions (`tools/check-cond-eval.py
// lib/at-func.c` -> 6 conditions, 0 unknown, 0 disagreements).  Its live reading has
// `GNULIB_SUPPORT_ONLY_AT_FDCWD`, `AT_FUNC_USE_F1_COND`, `AT_FUNC_RESULT` and `AT_FUNC_FAIL`
// all undefined, so C 21-25's `<errno.h>` include, C 32-42's conditional
// `CALL_FUNC`/`VALIDATE_FLAG`, C 48-52's `AT_FUNC_RESULT`, C 54-58's `AT_FUNC_FAIL` and
// C 77-79's `errno = ENOTSUP` return are dead; `mod standalone` records the names that
// reading leaves, and docs/ports/at-func.md records the arms.  The line-count basis is C's
// live lines (`docs/ports/line-drift-tolerance.tsv`).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_mut, unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Define at-style functions like fstatat, unlinkat, fchownat, etc.
   Copyright (C) 2006, 2009-2026 Free Software Foundation, Inc.

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

use std::ffi::{c_char, c_int, c_void};
include!("../src/include/config.rs");
include!("include/errno.rs");

/* C 19's `# include "filename.h"` (IS_ABSOLUTE_FILE_NAME), C 27-29's `# include "openat.h"` /
   `"openat-priv.h"` / `"save-cwd.h"` (AT_FDCWD; openat_proc_name/EXPECTED_ERRNO/
   OPENAT_BUFFER_SIZE; save_cwd/free_cwd/restore_cwd) and C 28's `<unistd.h>` (fchdir) are the
   consumers' includes: this file instantiates nothing, so it needs none of them (the ftoastr.c
   prelude includes only what its own text names).  */

/* C 32-58's standalone reading (`AT_FUNC_USE_F1_COND`, `AT_FUNC_RESULT` and `AT_FUNC_FAIL`
   all undefined) plus C 70's entry name, in a module because these are the names the ledger's
   macro check compares and `AT_FUNC_NAME` is not a `#define` in C (the ftoastr.rs `standalone`
   decision).  `AT_FUNC_NAME`'s identity record is C's own undefined-macro binding: the
   consumer's `# define` supplies the definition.  */
mod standalone {
  use super::*;
  macro_rules! AT_FUNC_NAME { ($name:ident) => { $name } }
  macro_rules! CALL_FUNC { ($f:expr) => { AT_FUNC_F1! ($f AT_FUNC_POST_FILE_ARGS! ()) } }
  macro_rules! VALIDATE_FLAG { ($f:expr) => { } }
  macro_rules! FUNC_RESULT { () => { int } }
  pub const FUNC_FAIL: c_int = -1;
}

/* C 60-139 as the shared `macro_rules!` PORTING.md 10.13② requires (the ftoastr.c /
   strftime.c shape).  `$name` is `AT_FUNC_NAME`, `$f1` `AT_FUNC_F1`, `$post_params`/
   `$post_args` `AT_FUNC_POST_FILE_PARAM_DECLS`/`AT_FUNC_POST_FILE_ARGS`, and `$func_result`/
   `$func_fail` `FUNC_RESULT`/`FUNC_FAIL` (a consumer's `AT_FUNC_RESULT`/`AT_FUNC_FAIL`
   binding).  `$post_params`/`$post_args` are spliced as token trees: an instance passes the
   Rust spelling of C's declaration/argument lists.  The conditional `AT_FUNC_USE_F1_COND`
   arm is the consumers' (`mod standalone`'s `CALL_FUNC`/`VALIDATE_FLAG` record its text).  */
macro_rules! at_func_body {
  ($name:ident, $export:literal, $func_result:ty, $func_fail:expr, $f1:ident,
   $post_params:tt, $post_args:tt) => {
    /* C 69-71.  */
    #[unsafe(export_name = $export)]
    pub unsafe extern "C" fn $name (fd: c_int, file: *const c_char $post_params)
      -> $func_result
    {
      /* C 72: `VALIDATE_FLAG (flag)` is empty in this reading.  */

      if fd == AT_FDCWD || IS_ABSOLUTE_FILE_NAME! (file)
        { return $f1 (file $post_args); }

      /* C 77-79's `#ifdef GNULIB_SUPPORT_ONLY_AT_FDCWD` arm is dead.  */

      /* Be careful to choose variable names unlikely to conflict with
         AT_FUNC_POST_FILE_PARAM_DECLS.  */

      {
        let mut proc_buf: [c_char; OPENAT_BUFFER_SIZE] = [0; OPENAT_BUFFER_SIZE];
        let proc_file: *mut c_char = openat_proc_name (proc_buf.as_mut_ptr (), fd, file);
        if ! proc_file.is_null ()
          {
            let proc_result: $func_result = $f1 (proc_file $post_args);
            let proc_errno: c_int = *errno ();
            if proc_file != proc_buf.as_mut_ptr ()
              { free (proc_file as *mut c_void); }
            /* If the syscall succeeds, or if it fails with an unexpected
               errno value, then return right away.  Otherwise, fall through
               and resort to using save_cwd/restore_cwd.  */
            if $func_fail != proc_result
              { return proc_result; }
            if ! EXPECTED_ERRNO! (proc_errno)
              {
                *errno () = proc_errno;
                return proc_result;
              }
          }
      }

      let mut saved_cwd: saved_cwd = std::mem::zeroed ();
      if save_cwd (&mut saved_cwd) != 0
        { openat_save_fail (*errno ()); }
      if 0 <= fd && fd == saved_cwd.desc
        {
          /* If saving the working directory collides with the user's
             requested fd, then the user's fd must have been closed to
             begin with.  */
          free_cwd (&mut saved_cwd);
          *errno () = EBADF;
          return $func_fail;
        }

      if fchdir (fd) != 0
        {
          let saved_errno: c_int = *errno ();
          free_cwd (&mut saved_cwd);
          *errno () = saved_errno;
          return $func_fail;
        }

      let err: $func_result = $f1 (file $post_args);
      let saved_errno: c_int = if err == $func_fail { *errno () } else { 0 };

      if restore_cwd (&mut saved_cwd) != 0
        { openat_restore_fail (*errno ()); }

      free_cwd (&mut saved_cwd);

      if saved_errno != 0
        { *errno () = saved_errno; }
      return err;
    }
  };
}
