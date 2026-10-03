// femacs: port of lib/fdopendir.c @ fd7afced403.
/* written by Paul Eggert */

/* emacs/lib/fdopendir.c (@ 238 lines) is on the verified list: HAVE_FDOPENDIR is 1 on this
   target, so C 26-214's `#if !HAVE_FDOPENDIR` arm - the whole `dirent-private.h` fallback with
   its `openat`/`save-cwd` machinery and its own `struct dirent` - is dead by measurement, and
   the live text is C 215-238, the twelve-line `rpl_fdopendir`.  C 21's <dirent.h> is the new
   dirent mirror, C 23's <stdlib.h> and C 217's <errno.h> the stdlib and errno mirrors, and
   C 218's <sys/stat.h> the sys_stat mirror (`struct stat`, `fstat`, `S_ISDIR`).  */
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/dirent.rs");
include!("include/errno.rs");
include!("include/stdlib.rs");

/* C 221's `# undef fdopendir`: the system entry the replacement calls, which is the dirent
   mirror's own declaration (the getgroups.c `system` shape is not needed, because this unit
   defines `rpl_fdopendir` rather than `fdopendir`).  */

/* Like fdopendir, but work around GNU/Hurd bug by validating FD.  */

/* C 225-237's `DIR *rpl_fdopendir (int fd)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rpl_fdopendir (fd: c_int) -> *mut DIR
{
  let mut st: stat = std::mem::zeroed ();
  if fstat (fd, &mut st) != 0
    { return std::ptr::null_mut (); }
  if !S_ISDIR! (st.st_mode)
    {
      *errno () = ENOTDIR;
      return std::ptr::null_mut ();
    }
  return fdopendir (fd);
}
