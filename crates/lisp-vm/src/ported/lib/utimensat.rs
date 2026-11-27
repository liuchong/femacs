// femacs: port of lib/utimensat.c @ fd7afced403.
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
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::sys_common::*;

/* written by Eric Blake */

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};

// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/limits.in.rs imported via common
// [U1] include/errno.rs imported via common
// [U1] include/fcntl.in.rs imported via common
// [U1] include/sys_types.in.rs imported via common
// [U1] include/sys_stat.in.rs imported via common
// [U1] include/string.rs imported via common
// [U1] include/stdlib.rs imported via common
// [U1] include/unistd.in.rs imported via common
// [U1] include/filename.rs imported via common
// [U1] include/time.in.rs imported via common
// [U1] include/timespec.rs imported via common
// [U1] include/stat-time.rs imported via common
// [U1] include/save-cwd.rs imported via common
// [U1] include/utimens.rs imported via common

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

/* C 13's `#include "openat.h"` now has a mirror (`include/openat.rs`, status 3.194), which owns
   C 55-56's two `_Noreturn` prototypes; this unit declares only `openat-priv.h`'s entry.  */
// [U1] include/openat.rs imported via common

unsafe extern "C" {
  /* openat-priv.h 52: char *openat_proc_name (char buf[OPENAT_BUFFER_SIZE], int fd, char
     const *file);  */
  pub fn openat_proc_name (buf: *mut c_char, fd: c_int, file: *const c_char) -> *mut c_char;
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
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
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
}

/* C 76-195's live `rpl_utimensat`.  C 78's `#  undef utimensat` means the call below names
   the *system* entry (the sys_stat mirror's declaration); C 88's `static int
   utimensat_works_really` is the function-scope cache.  The `__linux__`/`__sun`/`__NetBSD__`
   and `__hppa` arms are dead; the `__APPLE__ && __MACH__` arm is live.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_utimensat (fd: c_int, file: *const c_char,
                                        times: *const timespec, flag: c_int) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  static utimensat_works_really: VmGlobal<c_int> = VmGlobal::<c_int>::new (0);

  /* See comments in utimens.c for details.  */
  if 0 <= (*utimensat_works_really.as_mut_ptr ())
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
          (*utimensat_works_really.as_mut_ptr ()) = 1;
          return result;
        }
    }
  /* No point in trying openat/futimens, since on Linux, futimens is
     implemented with the same syscall as utimensat.  Only avoid the
     native utimensat due to an ENOSYS failure; an EINVAL error was
     data-dependent, and the next caller may pass valid data.  */
  if 0 <= (*utimensat_works_really.as_mut_ptr ()) && *errno () == ENOSYS
    { (*utimensat_works_really.as_mut_ptr ()) = -1; }
  return local_utimensat (fd, file, times, flag);

  }
}
