// femacs: WIP mirror of lib/unistd.in.h (gnulib's <unistd.h>; PORTING.md P-26).
//
// The frozen configuration leaves every `REPLACE_*` macro zero, so gnulib's header is the
// system <unistd.h> plus aliases that resolve to the system entries and a set of
// `_GL_WARN_ON_USE` portability warnings (attributes, which the port drops - PORTING.md 0.3).
// This mirror carries the entries the ported units name so far; the unchecked boxes of
// docs/ports/include-unistd.in.md are the rest of the 2447-line reference header.
//
// `ssize_t` is the frozen target's `long` (`sizeof (ssize_t) == 8`, measured).

include!("sys_types.in.rs");   // <sys/types.h>

/// The frozen target's `ssize_t` (`long`, 8 bytes).
pub type ssize_t = c_long;

/* C declares `chmod` in both <unistd.h> and <sys/stat.h>; the port keeps one declaration per
   name (PORTING.md 0.1-3), so it lives in sys_stat.in.rs and is *not* repeated here.  */

unsafe extern "C" {
  pub fn readlink (file: *const c_char, buf: *mut c_char, bufsize: usize) -> ssize_t;
  pub fn symlink (contents: *const c_char, name: *const c_char) -> c_int;
  pub fn readlinkat (fd: c_int, file: *const c_char, buf: *mut c_char, bufsize: usize)
    -> ssize_t;
  pub fn unlink (file: *const c_char) -> c_int;
  pub fn unlinkat (fd: c_int, file: *const c_char, flag: c_int) -> c_int;
  pub fn rmdir (file: *const c_char) -> c_int;
  pub fn link (from: *const c_char, to: *const c_char) -> c_int;
  pub fn linkat (fd1: c_int, from: *const c_char, fd2: c_int, to: *const c_char, flag: c_int)
    -> c_int;
  pub fn chdir (file: *const c_char) -> c_int;
  pub fn fchdir (fd: c_int) -> c_int;
  pub fn getcwd (buf: *mut c_char, size: usize) -> *mut c_char;
  pub fn access (file: *const c_char, mode: c_int) -> c_int;
  pub fn close (fd: c_int) -> c_int;
  pub fn read (fd: c_int, buf: *mut c_void, n: usize) -> ssize_t;
  pub fn write (fd: c_int, buf: *const c_void, n: usize) -> ssize_t;
  pub fn lseek (fd: c_int, offset: off_t, whence: c_int) -> off_t;
  pub fn ftruncate (fd: c_int, length: off_t) -> c_int;
  pub fn truncate (file: *const c_char, length: off_t) -> c_int;
  pub fn fsync (fd: c_int) -> c_int;
  pub fn dup (fd: c_int) -> c_int;
  pub fn dup2 (fd: c_int, fd2: c_int) -> c_int;
  pub fn pipe (fildes: *mut c_int) -> c_int;
  pub fn getpid () -> pid_t;
  pub fn getppid () -> pid_t;
  pub fn getuid () -> uid_t;
  pub fn geteuid () -> uid_t;
  pub fn getgid () -> gid_t;
  pub fn getegid () -> gid_t;
  pub fn chown (file: *const c_char, owner: uid_t, group: gid_t) -> c_int;
  pub fn fchown (fd: c_int, owner: uid_t, group: gid_t) -> c_int;
  pub fn lchown (file: *const c_char, owner: uid_t, group: gid_t) -> c_int;
  pub fn isatty (fd: c_int) -> c_int;
  /* unsigned int alarm (unsigned int);  (src/atimer.c's two live calls)  */
  pub fn alarm (seconds: c_uint) -> c_uint;
  pub fn sleep (seconds: c_uint) -> c_uint;
  pub fn usleep (useconds: c_uint) -> c_int;
  pub fn sysconf (name: c_int) -> c_long;
  pub fn getpagesize () -> c_int;

}
/* <unistd.h>'s `access` mode flags (measured: F_OK 0, X_OK 1, W_OK 2, R_OK 4).  */
pub const F_OK: c_int = 0;
pub const X_OK: c_int = 1;
pub const W_OK: c_int = 2;
pub const R_OK: c_int = 4;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

/* <unistd.h>: the two `sysconf` names lib/nproc.c's live branches test.  Values measured
   from the frozen target (`docs/baseline/config/darwin-arm64.macros.tsv` has
   `_SC_NPROCESSORS_CONF 57` and `_SC_NPROCESSORS_ONLN 58`).  */
pub const _SC_NPROCESSORS_CONF: c_int = 57;
pub const _SC_NPROCESSORS_ONLN: c_int = 58;
