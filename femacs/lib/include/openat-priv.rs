// femacs: WIP mirror of lib/openat-priv.h @ fd7afced403 (PORTING.md P-26).
//
// The header is gnulib's internal contract between the openat-like replacements:
// `SAFER_ALLOCA_MAX`/`SAFER_ALLOCA` and the `OPENAT_BUFFER_SIZE` they size, the
// `openat_proc_name` prototype lib/openat-proc.c defines, the `EXPECTED_ERRNO` predicate
// lib/openat.c and lib/linkat.c filter /proc failures with, and the `at_func2` wrapper
// prototype lib/fchmodat.c and lib/linkat.c share (lib/at-func2.c defines it).
//
// The frozen target defines `PATH_MAX` (limits.in.rs 59, 1024), so C 42-44's first arm is the
// live one and `OPENAT_BUFFER_SIZE` = `SAFER_ALLOCA (PATH_MAX)` = min (1024, 4096 - 64) = 1024
// (utimensat.rs already carries that value in-unit for the same reason).

/* C 28's `#define SAFER_ALLOCA_MAX (4096 - 64)`.  */
pub const SAFER_ALLOCA_MAX: usize = 4096 - 64;

/* C 30's `#define SAFER_ALLOCA(m) ((m) < SAFER_ALLOCA_MAX ? (m) : SAFER_ALLOCA_MAX)`.  */
macro_rules! SAFER_ALLOCA {
  ($m:expr) => { if ($m) < SAFER_ALLOCA_MAX { ($m) } else { SAFER_ALLOCA_MAX } };
}

/* C 42-48's `OPENAT_BUFFER_SIZE` (the `PATH_MAX` arm; the `_XOPEN_PATH_MAX` and 1024 arms are
   the non-frozen sides).  */
pub const OPENAT_BUFFER_SIZE: usize = 1024;

/* C 52's `#define EXPECTED_ERRNO(Errno)`.  */
macro_rules! EXPECTED_ERRNO {
  ($errno:expr) => {
    (($errno) == ENOTDIR || ($errno) == ENOENT || ($errno) == EPERM || ($errno) == EACCES
     || ($errno) == ENOSYS /* Solaris 8 */ || ($errno) == EOPNOTSUPP /* FreeBSD */)
  };
}

pub mod openat_priv_decls {
  use super::*;

  unsafe extern "C" {
    /* char *openat_proc_name (char buf[OPENAT_BUFFER_SIZE], int fd, char const *file);  */
    pub fn openat_proc_name (buf: *mut c_char, fd: c_int, file: *const c_char) -> *mut c_char;
    /* int at_func2 (int fd1, char const *file1, int fd2, char const *file2,
                     int (*func) (char const *file1, char const *file2));  */
    pub fn at_func2 (fd1: c_int, file1: *const c_char, fd2: c_int, file2: *const c_char,
                     func: unsafe extern "C" fn (*const c_char, *const c_char) -> c_int)
      -> c_int;
  }
}
use openat_priv_decls::*;
