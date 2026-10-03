// femacs: port of lib/open.c @ fd7afced403.
/* emacs/lib/open.c (@ 240 lines) is on the verified list, so only its live text is required.
   The frozen configuration has HAVE_WORKING_O_DIRECTORY undefined, OPEN_TRAILING_SLASH_BUG
   false and REPLACE_OPEN_DIRECTORY false, so C 52-63's three `#ifndef` fallbacks are live (the
   port keeps the constants), C 196-235's `#if REPLACE_FCHDIR` arms are dead, and the Windows
   arms (`_WIN32`) are dead too.  C 22's `#define __need_system_fcntl_h` before <config.h>
   makes the first <fcntl.h> the system one, which is what `orig_open` calls; the port keeps
   that entry in a `system` module.  `PROMOTED_MODE_T` is `int` on this target (macros.tsv
   1502).  */
use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/fcntl.in.rs");
include!("include/cloexec.rs");
include!("include/errno.rs");
include!("include/string.rs");
include!("include/unistd.in.rs");   // <unistd.h> (close)

/* C 22's `#define __need_system_fcntl_h` (empty body; the port keeps the name).  */
pub const __need_system_fcntl_h: () = ();

/* C 26-40's `static int orig_open (const char *filename, int flags, mode_t mode)`: C 33-37's
   Windows arm is dead, so the body is the `open` call - and `# undef __need_system_fcntl_h`
   plus the first include mean it is the *system* open, which the port reaches through its own
   module because this unit defines `open` below.  */
unsafe fn orig_open (filename: *const c_char, flags: c_int, mode: mode_t) -> c_int
{
  /* C's `open (filename, flags, mode)` promotes `mode_t` to `int` at the call; Rust refuses
     to pass a `u16` to a variadic function, so the promotion is written out.  */
  return system::open (filename, flags, mode as c_uint);
}

/* C 52-63's three `#ifndef` fallbacks, live on this target.  */
pub const HAVE_WORKING_O_DIRECTORY: bool = false;
pub const OPEN_TRAILING_SLASH_BUG: bool = false;
pub const REPLACE_OPEN_DIRECTORY: bool = false;

/* C 65-69's `static int lstatif (…)`.  */
unsafe fn lstatif (filename: *const c_char, st: *mut stat, flags: c_int) -> c_int
{
  return if flags & O_NOFOLLOW != 0 { lstat (filename, st) } else { stat (filename, st) };
}

/* C 71-240's `int open (const char *filename, int flags, ...)`.  The `...` is Rust's
   C-variadic parameter; C's `va_list arg; va_start (arg, flags); … va_end (arg);` has no port
   counterpart, because the `...` parameter *is* the list (P-04), and `va_arg (arg,
   PROMOTED_MODE_T)` is `next_arg::<PROMOTED_MODE_T>` (the mirror's alias for C's `int`).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn open (filename: *const c_char, flags: c_int, mut ap: ...) -> c_int
{
  let mut mode: mode_t = 0;

  if flags & O_CREAT != 0
    {
      /* We have to use PROMOTED_MODE_T instead of mode_t, otherwise GCC 4
         creates crashing code when 'mode_t' is smaller than 'int'.  */
      mode = ap.next_arg::<PROMOTED_MODE_T> () as mode_t;
    }

  /* Fail if one of O_CREAT, O_WRONLY, O_RDWR is specified and the filename
     ends in a slash, as POSIX says such a filename must name a directory.
     (C's comment, kept in full above the check.)  */
  let check_for_slash_bug: bool = if OPEN_TRAILING_SLASH_BUG
    {
      let len: usize = strlen (filename);
      len != 0 && *filename.add (len - 1) == b'/' as c_char
    }
    else
    { false };

  if check_for_slash_bug
    && (flags & O_CREAT != 0
        || (flags & O_ACCMODE) == O_RDWR
        || (flags & O_ACCMODE) == O_WRONLY)
    {
      *errno () = EISDIR;
      return -1;
    }

  /* With the trailing slash bug or without working O_DIRECTORY, check with
     stat first lest we hang trying to open a fifo.  Although there is
     a race between this and opening the file, we can do no better.
     After opening the file we will check again with fstat.  */
  let check_directory: bool =
    check_for_slash_bug
    || (!HAVE_WORKING_O_DIRECTORY && flags & O_DIRECTORY != 0);
  if check_directory
    {
      let mut statbuf: stat = std::mem::zeroed ();
      if lstatif (filename, &mut statbuf, flags) < 0
        {
          if ! (flags & O_CREAT != 0 && *errno () == ENOENT)
            { return -1; }
        }
      else if !S_ISDIR! (statbuf.st_mode)
        {
          *errno () = ENOTDIR;
          return -1;
        }
    }

  /* 0 = unknown, 1 = yes, -1 = no.  */
  static mut have_cloexec: c_int = 0;

  let mut fd: c_int = orig_open (filename,
                                 flags & !(if have_cloexec < 0 { O_CLOEXEC } else { 0 }),
                                 mode);

  if flags & O_CLOEXEC != 0
    {
      if have_cloexec == 0
        {
          if 0 <= fd
            { have_cloexec = 1; }
          else if *errno () == EINVAL
            {
              fd = orig_open (filename, flags & !O_CLOEXEC, mode);
              have_cloexec = -1;
            }
        }
      if have_cloexec < 0 && 0 <= fd
        { set_cloexec_flag (fd, true); }
    }

  /* If checking for directories, fail if fd does not refer to a directory.
     Rationale: A filename ending in slash cannot name a non-directory.  */
  if check_directory && 0 <= fd
    {
      let mut statbuf: stat = std::mem::zeroed ();
      let r: c_int = fstat (fd, &mut statbuf);
      if r < 0 || !S_ISDIR! (statbuf.st_mode)
        {
          let err: c_int = if r < 0 { *errno () } else { ENOTDIR };
          close (fd);
          *errno () = err;
          return -1;
        }
    }

  return fd;
}

/* C 35's `return open (filename, flags, mode);` - the system entry, which this unit defines a
   replacement for (the getgroups.c/cloexec.c `system` shape).  */
mod system {
  use super::*;

  /* rustc checks this declaration against the runtime's own `open` ("invalid definition of the
     runtime `open` symbol used by the standard library: expected
     `unsafe extern "C" fn(*const i8, i32, ...) -> i32`"), so the port spells C's
     `int open (const char *, int, ...)` declaration exactly, `...` included; a C-variadic
     Rust declaration accepts the three-argument call below.  */
  unsafe extern "C" {
    pub fn open (filename: *const c_char, flags: c_int, ...) -> c_int;
  }
}
