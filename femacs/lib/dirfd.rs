// femacs: port of lib/dirfd.c @ fd7afced403.  The file is blind
// (not in docs/baseline/config/cond-eval-verified.txt), so both arms of C 32's
// `#if GNULIB_defined_DIR` are carried; the frozen arm is C 38-42's, because the
// configuration does not define GNULIB_defined_DIR (macros.tsv has no row for it).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]
use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/dirent.rs");     // <dirent.h> (`DIR`)
include!("include/errno.rs");      // <errno.h> (`errno`, EINVAL, ENOTSUP)


/* C 25-27's `#if GNULIB_defined_DIR / # include "dirent-private.h" / #endif` is dead, and
   with it C 33-36's arm: it reads the `fd_to_close` member of the gnulib-private `DIR`,
   which this target's <dirent.h> has not got (P-51; the header is not in the frozen
   build).  Its two tokens are named at the site below.  */

/* <dirent.h>'s `# define DIR_TO_FD(Dir_p) -1` (macros.tsv 757).  */
macro_rules! DIR_TO_FD { ($dir_p:expr) => { -1 as c_int }; }

/* int dirfd (DIR *dir_p)  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dirfd (dir_p: *mut DIR) -> c_int
{
  /* C 33-36's dead arm - `dir_p->fd_to_close`, and `EINVAL` for its -1 - is named here
     (the tempname.rs precedent for text whose header this target has not got).  */

  /* C 38.  */
  let fd: c_int = DIR_TO_FD! (dir_p);
  /* C 39-40.  */
  if fd == -1
    { *errno () = ENOTSUP; }

  /* C 42.  */
  return fd;
}
