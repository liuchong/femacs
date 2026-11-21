// femacs: port of lib/openat-proc.c @ fd7afced403.
//
// NOTE: lib/openat-proc.c is **not** in `docs/baseline/config/cond-eval-verified.txt`, so the
// gate reads *both* arms of every `#if` as live text (P-66).  The frozen build does compile the
// file - `build/emacs-ref/lib/openat-proc.o` exists and defines `_openat_proc_name`, calling
// `_access`, `_close`, `_malloc`, `_open`, `_sprintf`, `_strcpy`, `_strlen` - so C 59-113's
// generic `/proc/self/fd` arm is the live text and C 33-38's `__KLIBC__`/`__MVS__` includes and
// C 114-150's klibc / z-OS arm are the non-frozen sides; the latter is kept as `#[cfg]` text
// (P-51), whose entries this target has not got (`__libc_Back_ioFHToPath`, `_MAX_PATH`,
// `w_ioctl`, `_IOCC_GPN`, `_XOPEN_PATH_MAX`, `__e2a_l`).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;

/* Written by Paul Eggert.  */

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/errno.rs imported via common
// [U1] include/limits.in.rs imported via common
// [U1] include/stdlib.rs imported via common
// [U1] include/string.rs imported via common
// [U1] include/stdio.rs imported via common
// [U1] include/fcntl.in.rs imported via common
// [U1] include/sys_types.in.rs imported via common
// [U1] include/sys_stat.in.rs imported via common
// [U1] include/unistd.in.rs imported via common
// [U1] include/openat-priv.rs imported via common

/* C 20-31: <config.h>, "openat-priv.h", <sys/types.h>, <sys/stat.h>, <fcntl.h>, <stdio.h>,
   <stdlib.h>, <string.h> and <unistd.h> are the mirrors above.  C 33-38's `#ifdef __KLIBC__`
   and `#ifdef __MVS__` includes are dead (neither macro is defined).  */

/* C 40's "intprops.h" contributes `INT_STRLEN_BOUND`, whose gnulib definition is
   `((TYPE_WIDTH (t) - TYPE_SIGNED (t)) * 146 + 484) / 485 + TYPE_SIGNED (t)`; the port writes
   that formula with the type's width and signedness read from the type itself (the only use in
   this file is `INT_STRLEN_BOUND (int)` = (31 * 146 + 484) / 485 + 1 = 11).  The header itself
   has no mirror yet; the construct lands here with its only consumer.  */
macro_rules! INT_STRLEN_BOUND {
  ($t:ty) => {
    ((std::mem::size_of::<$t> () * 8 - 1) * 146 + 484) / 485 + 1
  };
}

/* C 61's `# define PROC_SELF_FD_FORMAT "/proc/self/fd/%d/"` (the live arm's format).  The
   port keeps C's bytes, NUL included, as a `&[u8; 18]` constant (17 characters and the NUL) (PORTING.md 2.1's `&CStr`
   shape for a string macro): `size_of_val` is then C's `sizeof PROC_SELF_FD_FORMAT`, and
   `.as_ptr ()` is the `char *` the `sprintf`/`strcpy` calls take.  */
pub const PROC_SELF_FD_FORMAT: &[u8; 18] = b"/proc/self/fd/%d/\0";

/* Set BUF to the name of the subfile of the directory identified by
   FD, where the subfile is named FILE.  If successful, return BUF if
   the result fits in BUF, dynamically allocated memory otherwise.
   Return NULL (setting errno) on error.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openat_proc_name (buf: *mut c_char, fd: c_int, file: *const c_char)
  -> *mut c_char
{
  /* Make sure the caller gets ENOENT when appropriate.  */
  if *file == 0
    {
      *buf = 0;
      return buf;
    }

  let mut result: *mut c_char = buf;
  let mut dirlen: c_int;

  /* C 59-113's `#if !(defined __KLIBC__ || defined __MVS__)` arm is the live text.  */
  {
    /* C 63-67's `enum { PROC_SELF_FD_DIR_SIZE_BOUND = ... };`.  */
    const PROC_SELF_FD_DIR_SIZE_BOUND: usize =
      (std::mem::size_of_val (PROC_SELF_FD_FORMAT) - (std::mem::size_of_val (b"%d") - 1)
       + INT_STRLEN_BOUND! (c_int) as usize);

    static mut proc_status: c_int = 0;
    if proc_status == 0
      {
        /* Set PROC_STATUS to a positive value if /proc/self/fd is
           reliable, and a negative value otherwise.  Solaris 10
           /proc/self/fd mishandles "..", and any file name might expand
           to ".." after symbolic link expansion, so avoid /proc/self/fd
           if it mishandles "..".  Solaris 10 has openat, but this
           problem is exhibited on code that built on Solaris 8 and
           running on Solaris 10.  */

        let proc_self_fd: c_int =
          open (c"/proc/self/fd".as_ptr (),
                O_SEARCH | O_DIRECTORY | O_NOCTTY | O_NONBLOCK | O_CLOEXEC);
        if proc_self_fd < 0
          { proc_status = -1; }
        else
          {
            /* Detect whether /proc/self/fd/%i/../fd exists, where %i is the
               number of a file descriptor open on /proc/self/fd.  On Linux,
               that name resolves to /proc/self/fd, which was opened above.
               However, on Solaris, it may resolve to /proc/self/fd/fd, which
               cannot exist, since all names in /proc/self/fd are numeric.  */
            let mut dotdot_buf: [c_char; PROC_SELF_FD_DIR_SIZE_BOUND + 5] =
              [0; PROC_SELF_FD_DIR_SIZE_BOUND + 5];
            sprintf (dotdot_buf.as_mut_ptr (), c"/proc/self/fd/%d/../fd".as_ptr (),
                     proc_self_fd);
            proc_status = if access (dotdot_buf.as_ptr (), F_OK) != 0 { -1 } else { 1 };
            close (proc_self_fd);
          }
      }

    if proc_status < 0
      { return std::ptr::null_mut (); }
    else
      {
        let bufsize: usize = PROC_SELF_FD_DIR_SIZE_BOUND + strlen (file);
        if OPENAT_BUFFER_SIZE < bufsize
          {
            result = malloc (bufsize) as *mut c_char;
            if result.is_null ()
              { return std::ptr::null_mut (); }
          }

        dirlen = sprintf (result, c"/proc/self/fd/%d/".as_ptr (), fd);
      }
  }

  /* C 114-150's `#else` arm (OS/2 kLIBC, z/OS) is the non-frozen side, kept as `#[cfg]` text
     (P-51).  */
  #[cfg (feature = "klibc_or_mvs")]
  {
    let bufsize: usize;

    #[cfg (feature = "klibc")]
    {
      let mut dir: [c_char; _MAX_PATH as usize] = [0; _MAX_PATH as usize];
      if __libc_Back_ioFHToPath (fd, dir.as_mut_ptr (), dir.len ()) != 0
        { return std::ptr::null_mut (); }
    }
    #[cfg (feature = "mvs")]
    {
      let mut dir: [c_char; _XOPEN_PATH_MAX as usize] = [0; _XOPEN_PATH_MAX as usize];
      /* Documentation:
         https://www.ibm.com/docs/en/zos/2.2.0?topic=functions-w-ioctl-w-pioctl-control-devices */
      if w_ioctl (fd, _IOCC_GPN, dir.len (), dir.as_mut_ptr ()) < 0
        { return std::ptr::null_mut (); }
      /* Documentation:
         https://www.ibm.com/docs/en/zos/2.2.0?topic=functions-e2a-l-convert-characters-from-ebcdic-ascii */
      dirlen = __e2a_l (dir.as_mut_ptr (), strlen (dir.as_ptr ()));
      if dirlen < 0 || dirlen as usize >= dir.len ()
        { return std::ptr::null_mut (); }
      dir[dirlen as usize] = 0;
    }

    dirlen = strlen (dir.as_ptr ()) as c_int;
    bufsize = dirlen as usize + 1 + strlen (file) + 1; /* 1 for '/', 1 for null */
    if OPENAT_BUFFER_SIZE < bufsize
      {
        result = malloc (bufsize) as *mut c_char;
        if result.is_null ()
          { return std::ptr::null_mut (); }
      }

    strcpy (result, dir.as_ptr ());
    result = result.add (dirlen as usize);
    *result = b'/' as c_char;
    dirlen += 1;
    result = result.sub (dirlen as usize);
  }

  strcpy (result.add (dirlen as usize), file);
  return result;
}
