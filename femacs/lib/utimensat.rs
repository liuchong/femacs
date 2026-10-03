// femacs: port of lib/utimensat.c @ fd7afced403 .
//
// `lib/utimensat.c` is in `docs/baseline/config/cond-eval-verified.txt`.  Its `#if
// HAVE_NEARLY_WORKING_UTIMENSAT` arm is DEAD (the macro is undefined in the frozen
// configuration), so the live text is the `#else` arm: the `HAVE_UTIMENSAT` arm's
// `rpl_utimensat` (the ENOSYS/EINVAL probe plus the macOS tv_nsec validation and trailing
// slash check) and, below it, at-func.c's template bound by the six `AT_FUNC_*` macros -
// `AT_FUNC_NAME` = `local_utimensat`, `AT_FUNC_F1`/`F2` = `lutimens`/`utimens`,
// `AT_FUNC_USE_F1_COND` = `AT_SYMLINK_NOFOLLOW`.  Rust has no textual include, so the port
// writes the *expansion* of that template (save_cwd/fchdir/restore_cwd and the
// /proc/self/fd attempt) and keeps the macro names below as the record of the bindings.
// (Measured: `nm build/emacs-ref/lib/utimensat.o` defines `_rpl_utimensat` and calls
// `_lutimens`, `_save_cwd`, `_fchdir`, `_restore_cwd`, `_openat_proc_name` - the template is
// compiled in, not dead.)
//
// Deviation worth a decision (docs/ports/utimensat.md): `rpl_utimensat` (C 76) and
// `local_utimensat` (C 70) are *not* in the gate's function inventory - this file's
// definitions are generated text, and `tools/extract-abi.py` reads only the file itself
// (`# include "at-func.c"` is not followed, and C 78's `#  undef utimensat` sits between the
// declarator and the body) - so `#[unsafe(no_mangle)]` on `rpl_utimensat` is reported as an
// "invented exported symbol".  The port keeps both bodies faithfully, non-exported, and
// records the gap for adjudication (the same one utimens.c has).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Set the access and modification time of a file relative to directory fd.
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

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/errno.rs");
include!("include/fcntl.in.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/string.rs");
include!("include/stdlib.rs");
include!("include/unistd.in.rs");
include!("include/filename.rs");
include!("include/time.in.rs");
include!("include/timespec.rs");
include!("include/stat-time.rs");
include!("include/save-cwd.rs");
include!("include/utimens.rs");

/* C 30-58's `#if HAVE_NEARLY_WORKING_UTIMENSAT` arm (the trailing-slash `rpl_utimensat`)
   is dead for this target; C 36's `#else` arm - everything below - is the live text.  */

/* C 88-100's `<stdlib.h>`/`<string.h>` entries, the openat-priv.h pair the template needs,
   and gnulib's renamed `fstatat`.  */
mod system {
  use super::*;

  unsafe extern "C" {
    /* gnulib's <sys/stat.h> renames `fstatat` to `rpl_fstatat` in this build
       (PORTING.md 10.13; the reference object calls `_rpl_fstatat`).  */
    #[link_name = "rpl_fstatat"]
    pub fn fstatat (fd: c_int, file: *const c_char, buf: *mut stat, flag: c_int) -> c_int;
  }
}

unsafe extern "C" {
  /* openat-priv.h 52: char *openat_proc_name (char buf[OPENAT_BUFFER_SIZE], int fd, char
     const *file);  */
  pub fn openat_proc_name (buf: *mut c_char, fd: c_int, file: *const c_char) -> *mut c_char;
  /* openat.h 55-56: _Noreturn void openat_save_fail (int) / openat_restore_fail (int);  */
  pub fn openat_save_fail (err: c_int) -> !;
  pub fn openat_restore_fail (err: c_int) -> !;
}

/* openat-priv.h 40-49: `SAFER_ALLOCA (m) ((m) < 4096 - 64 ? (m) : 4096 - 64)` and
   `OPENAT_BUFFER_SIZE SAFER_ALLOCA (PATH_MAX)`; the frozen PATH_MAX is 1024, so the
   template's `char proc_buf[OPENAT_BUFFER_SIZE]` is this many bytes.  */
pub const OPENAT_BUFFER_SIZE: usize = 1024;

/* openat-priv.h 58-62: `EXPECTED_ERRNO (Errno)`.  */
macro_rules! EXPECTED_ERRNO {
    ($errno:expr) => {
        (($errno) == ENOTDIR || ($errno) == ENOENT
         || ($errno) == EPERM || ($errno) == EACCES
         || ($errno) == ENOSYS /* Solaris 8 */
         || ($errno) == EOPNOTSUPP /* FreeBSD */)
    };
}

/* C 71's `# define AT_FUNC_NAME local_utimensat` and C 209-213's block: the at-func.c
   template's parameters, kept verbatim (their bodies are the token lists C passes to the
   template; Rust has no textual include, so the port writes the expansion below).  */
macro_rules! AT_FUNC_NAME { () => { local_utimensat } }
/* AT_FUNC_NAME is now utimensat or local_utimensat.  */
macro_rules! AT_FUNC_F1 { () => { lutimens } }
macro_rules! AT_FUNC_F2 { () => { utimens } }
macro_rules! AT_FUNC_USE_F1_COND { () => { AT_SYMLINK_NOFOLLOW } }
macro_rules! AT_FUNC_POST_FILE_PARAM_DECLS { () => { , struct timespec const ts[2], int flag } }
macro_rules! AT_FUNC_POST_FILE_ARGS { () => { , ts } }

/* C 70's `static int local_utimensat (int, char const *, struct timespec const[2], int);`
   and C 69-155's at-func.c expansion for these bindings: `FUNC_RESULT`/`FUNC_FAIL` are
   `int`/`-1`, `VALIDATE_FLAG` rejects any bit outside `AT_FUNC_USE_F1_COND`, `CALL_FUNC`
   picks `lutimens` for `AT_SYMLINK_NOFOLLOW` and `utimens` otherwise, and the fallback is
   the openat_proc_name attempt, then save_cwd/fchdir/restore_cwd.  */
unsafe extern "C" fn local_utimensat (fd: c_int, file: *const c_char, ts: *const timespec,
                                      flag: c_int) -> c_int
{
  if flag & !AT_SYMLINK_NOFOLLOW != 0
    {
      *errno () = EINVAL;
      return -1;
    }

  if fd == AT_FDCWD || IS_ABSOLUTE_FILE_NAME! (file)
    {
      return if flag == AT_SYMLINK_NOFOLLOW { lutimens (file, ts) }
             else { utimens (file, ts) };
    }

  /* Be careful to choose variable names unlikely to conflict with
     AT_FUNC_POST_FILE_PARAM_DECLS.  */

  {
    let mut proc_buf: [c_char; OPENAT_BUFFER_SIZE] = [0; OPENAT_BUFFER_SIZE];
    let proc_file: *mut c_char = openat_proc_name (proc_buf.as_mut_ptr (), fd, file);
    if !proc_file.is_null ()
      {
        let proc_result: c_int
          = if flag == AT_SYMLINK_NOFOLLOW { lutimens (proc_file, ts) }
            else { utimens (proc_file, ts) };
        let proc_errno: c_int = *errno ();
        if proc_file != proc_buf.as_mut_ptr ()
          { free (proc_file as *mut c_void); }
        /* If the syscall succeeds, or if it fails with an unexpected
           errno value, then return right away.  Otherwise, fall through
           and resort to using save_cwd/restore_cwd.  */
        if -1 != proc_result
          { return proc_result; }
        if ! EXPECTED_ERRNO! (proc_errno)
          {
            *errno () = proc_errno;
            return proc_result;
          }
      }
  }

  let mut saved_cwd: saved_cwd = std::mem::zeroed ();
  if save_cwd_decls::save_cwd (&mut saved_cwd) != 0
    { openat_save_fail (*errno ()); }
  if 0 <= fd && fd == saved_cwd.desc
    {
      /* If saving the working directory collides with the user's
         requested fd, then the user's fd must have been closed to
         begin with.  */
      save_cwd_decls::free_cwd (&mut saved_cwd);
      *errno () = EBADF;
      return -1;
    }

  if fchdir (fd) != 0
    {
      let saved_errno: c_int = *errno ();
      save_cwd_decls::free_cwd (&mut saved_cwd);
      *errno () = saved_errno;
      return -1;
    }

  let err: c_int = if flag == AT_SYMLINK_NOFOLLOW { lutimens (file, ts) }
                   else { utimens (file, ts) };
  let saved_errno: c_int = if err == -1 { *errno () } else { 0 };

  if save_cwd_decls::restore_cwd (&mut saved_cwd) != 0
    { openat_restore_fail (*errno ()); }

  save_cwd_decls::free_cwd (&mut saved_cwd);

  if saved_errno != 0
    { *errno () = saved_errno; }
  return err;
}

/* C 76-195's live `rpl_utimensat`.  C 78's `#  undef utimensat` means the call below names
   the *system* entry (the sys_stat mirror's declaration); C 88's `static int
   utimensat_works_really` is the function-scope cache.  The `__linux__`/`__sun`/`__NetBSD__`
   and `__hppa` arms are dead; the `__APPLE__ && __MACH__` arm is live.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_utimensat (fd: c_int, file: *const c_char,
                                        times: *const timespec, flag: c_int) -> c_int
{
  static mut utimensat_works_really: c_int = 0;

  /* See comments in utimens.c for details.  */
  if 0 <= utimensat_works_really
    {
      /* macOS 10.13 and GNU Hurd do not reject invalid tv_nsec values
         either.  */
      if !times.is_null ()
          && (((*times.offset (0)).tv_nsec != UTIME_OMIT
               && (*times.offset (0)).tv_nsec != UTIME_NOW
               && ! (0 <= (*times.offset (0)).tv_nsec
                     && (*times.offset (0)).tv_nsec < TIMESPEC_HZ as c_long))
              || ((*times.offset (1)).tv_nsec != UTIME_OMIT
                  && (*times.offset (1)).tv_nsec != UTIME_NOW
                  && ! (0 <= (*times.offset (1)).tv_nsec
                        && (*times.offset (1)).tv_nsec < TIMESPEC_HZ as c_long)))
        {
          *errno () = EINVAL;
          return -1;
        }
      let len: usize = strlen (file);
      if len > 0 && *file.offset ((len - 1) as isize) == b'/' as c_char
        {
          let mut statbuf: stat = std::mem::zeroed ();
          if system::fstatat (fd, file, &mut statbuf, 0) < 0
            { return -1; }
          if ! S_ISDIR! (statbuf.st_mode)
            {
              *errno () = ENOTDIR;
              return -1;
            }
        }
      let result: c_int = utimensat (fd, file, times, flag);
      /* Linux kernel 2.6.25 has a bug where it returns EINVAL for
         UTIME_NOW or UTIME_OMIT with non-zero tv_sec, which
         local_utimensat works around.  Meanwhile, EINVAL for a bad
         flag is indeterminate whether the native utimensat works, but
         local_utimensat will also reject it.  */
      if result == -1 && *errno () == EINVAL && (flag & !AT_SYMLINK_NOFOLLOW) != 0
        { return result; }
      if result == 0 || (*errno () != ENOSYS && *errno () != EINVAL)
        {
          utimensat_works_really = 1;
          return result;
        }
    }
  /* No point in trying openat/futimens, since on Linux, futimens is
     implemented with the same syscall as utimensat.  Only avoid the
     native utimensat due to an ENOSYS failure; an EINVAL error was
     data-dependent, and the next caller may pass valid data.  */
  if 0 <= utimensat_works_really && *errno () == ENOSYS
    { utimensat_works_really = -1; }
  return local_utimensat (fd, file, times, flag);
}
