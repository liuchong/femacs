// femacs: the parts of the system <errno.h> the port needs (PORTING.md 0.1).  C's `errno` is
// not an object on the frozen target but the macro `(*__error ())`, so the mirror keeps the
// name as an accessor rather than a static; `c_int` comes from the including unit's scope, as
// the other lib mirrors expect.
//
// ENOMEM's value was measured from the frozen target's clang:
// `printf` of `int x = ENOMEM;` expands to `int x = 12;`.

// [U1-MODULE-MIRROR]
use super::*;
unsafe extern "C" {
  /// <errno.h>: the per-thread error number's address (`int *__error (void)`).
  #[cfg (not (any (target_os = "android", target_os = "linux", target_os = "windows")))]
  pub fn __error () -> *mut c_int;
  /// bionic's spelling of the same entry: `<errno.h>` 52 `int* __errno(void)` and 58
  /// `#define errno (*__errno())` (status 4.728).
  #[cfg (target_os = "android")]
  pub fn __errno () -> *mut c_int;
  /// glibc Linux spelling of the same entry: `int *__errno_location (void)`.
  #[cfg (target_os = "linux")]
  pub fn __errno_location () -> *mut c_int;
  /// Windows UCRT spelling of the same entry: `int *_errno (void)`.
  #[cfg (target_os = "windows")]
  pub fn _errno () -> *mut c_int;
}

/// <errno.h>'s `errno` on the frozen target: `#define errno (*__error ())`, so C's
/// `errno = 0;` is the port's `*errno () = 0;` and C's `errno != 0` its `*errno () != 0`.
/// Android's header spells the same macro over `__errno`; Linux glibc uses `__errno_location`;
/// Windows UCRT uses `_errno`.
#[inline(always)]
pub unsafe fn errno () -> *mut c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  #[cfg (not (any (target_os = "android", target_os = "linux", target_os = "windows")))]
  { __error () }
  #[cfg (target_os = "android")]
  { __errno () }
  #[cfg (target_os = "linux")]
  { __errno_location () }
  #[cfg (target_os = "windows")]
  { _errno () }

  }
}

/// <errno.h>: `#define EINVAL 22`.
pub const EINVAL: c_int = 22;

/// <errno.h>: `#define ENOSYS 78` (Darwin) / `38` (bionic & Linux) / `40` (Windows UCRT).
#[cfg (not (any (feature = "have_android", target_os = "linux", target_os = "windows")))]
pub const ENOSYS: c_int = 78;
/* Measured on the delivery target (bionic & glibc Linux), status 4.781. */
#[cfg (any (feature = "have_android", target_os = "linux"))]
pub const ENOSYS: c_int = 38;
#[cfg (target_os = "windows")]
pub const ENOSYS: c_int = 40;

/// <errno.h>: `#define ENOMEM 12`.
pub const ENOMEM: c_int = 12;

// The rest of the codes the ported lib/*.c files name, measured the same way:
//
//     $ cat > /tmp/eprobe.c <<'EOF'
//     #include <errno.h>
//     #include <stdio.h>
//     int main(void){ printf("%d %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d\n",
//       EPERM,ENOENT,EINTR,EIO,EBADF,EACCES,EEXIST,ENOTDIR,EINVAL,ENOSPC,EROFS,ELOOP,ENAMETOOLONG,
//       ENOTSUP,EOVERFLOW,ERANGE); return 0; }
//     EOF
//     $ clang -o /tmp/eprobe /tmp/eprobe.c && /tmp/eprobe
//     1 2 4 5 9 13 17 20 22 28 30 62 63 45 84 34

/// <errno.h>: `#define EPERM 1`.
pub const EPERM: c_int = 1;
/// <errno.h>: `#define EBUSY 16` (measured; docs/baseline/config/darwin-arm64.macros.tsv 802).
pub const EBUSY: c_int = 16;

/// <errno.h>: `#define ENOENT 2`.
pub const ENOENT: c_int = 2;
/// <errno.h>: `#define EINTR 4`.
pub const EINTR: c_int = 4;
/// <errno.h>: `#define EIO 5`.
pub const EIO: c_int = 5;
/// <errno.h>: `#define EBADF 9`.
pub const EBADF: c_int = 9;
/// <errno.h>: `#define EACCES 13`.
pub const EACCES: c_int = 13;
/// <errno.h>: `#define EEXIST 17`.
pub const EEXIST: c_int = 17;
/// <errno.h>: `#define ENOTDIR 20`.
pub const ENOTDIR: c_int = 20;
/// <errno.h>: `#define ENOSPC 28`.
pub const ENOSPC: c_int = 28;
/// <errno.h>: `#define EROFS 30`.
pub const EROFS: c_int = 30;
/// <errno.h>: `#define ERANGE 34`.
pub const ERANGE: c_int = 34;
/// <errno.h>: `#define ENOTSUP 45` (Darwin) / `95` (Linux) / `129` (Windows UCRT).
#[cfg (not (any (target_os = "linux", target_os = "windows")))]
pub const ENOTSUP: c_int = 45;
#[cfg (target_os = "linux")]
pub const ENOTSUP: c_int = 95;
#[cfg (target_os = "windows")]
pub const ENOTSUP: c_int = 129;
/// <errno.h>: `#define ELOOP 62` (Darwin) / `40` (Linux) / `114` (Windows UCRT).
#[cfg (not (any (target_os = "linux", target_os = "windows")))]
pub const ELOOP: c_int = 62;
#[cfg (target_os = "linux")]
pub const ELOOP: c_int = 40;
#[cfg (target_os = "windows")]
pub const ELOOP: c_int = 114;
/// <errno.h>: `#define ENAMETOOLONG 63` (Darwin) / `36` (bionic & Linux) / `38` (Windows UCRT).
#[cfg (not (any (feature = "have_android", target_os = "linux", target_os = "windows")))]
pub const ENAMETOOLONG: c_int = 63;
/* Measured on the delivery target (bionic & glibc Linux), status 4.781. */
#[cfg (any (feature = "have_android", target_os = "linux"))]
pub const ENAMETOOLONG: c_int = 36;
#[cfg (target_os = "windows")]
pub const ENAMETOOLONG: c_int = 38;
/// <errno.h>: `#define EOVERFLOW 84` (Darwin) / `75` (Linux) / `132` (Windows UCRT).
#[cfg (not (any (target_os = "linux", target_os = "windows")))]
pub const EOVERFLOW: c_int = 84;
#[cfg (target_os = "linux")]
pub const EOVERFLOW: c_int = 75;
#[cfg (target_os = "windows")]
pub const EOVERFLOW: c_int = 132;

/// <errno.h>: `#define EOPNOTSUPP 102` (Darwin) / `95` (Linux) / `130` (Windows UCRT).
#[cfg (not (any (target_os = "linux", target_os = "windows")))]
pub const EOPNOTSUPP: c_int = 102;
#[cfg (target_os = "linux")]
pub const EOPNOTSUPP: c_int = 95;
#[cfg (target_os = "windows")]
pub const EOPNOTSUPP: c_int = 130;

/// <errno.h>: `#define EISDIR 21` (measured: `printf ("%d", EISDIR)` -> 21).
pub const EISDIR: c_int = 21;

/* The socket/file codes `src/process.c`, `src/dired.c`, `src/sysdep.c` and `lib/dup2.c` each
   declared for themselves until status 3.80; measured with `clang -E -dM -include errno.h`
   (the same measurement ENOMEM's comment above records).  C spells `EWOULDBLOCK` as
   `#define EWOULDBLOCK EAGAIN`, so the port keeps both names and the same value.  */
/// <errno.h>: `#define EAGAIN 35` (Darwin) / `11` (bionic & Linux & Windows).
#[cfg (not (any (feature = "have_android", target_os = "linux", target_os = "windows")))]
pub const EAGAIN: c_int = 35;
/* Measured on the delivery target (bionic & glibc Linux), status 4.781. */
#[cfg (any (feature = "have_android", target_os = "linux", target_os = "windows"))]
pub const EAGAIN: c_int = 11;
/// <errno.h>: `#define EWOULDBLOCK EAGAIN` (140 on Windows UCRT).
#[cfg (not (any (target_os = "linux", target_os = "windows")))]
pub const EWOULDBLOCK: c_int = 35;
#[cfg (target_os = "linux")]
pub const EWOULDBLOCK: c_int = 11;
#[cfg (target_os = "windows")]
pub const EWOULDBLOCK: c_int = 140;
/// <errno.h>: `#define EMFILE 24`.
pub const EMFILE: c_int = 24;
/// <errno.h>: `#define EINPROGRESS 36` (Darwin) / `115` (Linux) / `112` (Windows) - **except on the MS-DOS build**.
#[cfg (all (not (feature = "msdos"), not (any (target_os = "linux", target_os = "windows"))))]
pub const EINPROGRESS: c_int = 36;
#[cfg (all (not (feature = "msdos"), target_os = "linux"))]
pub const EINPROGRESS: c_int = 115;
#[cfg (all (not (feature = "msdos"), target_os = "windows"))]
pub const EINPROGRESS: c_int = 112;
/// <errno.h>: `#define EISCONN 56` (Darwin) / `106` (Linux) / `113` (Windows UCRT).
#[cfg (not (any (target_os = "linux", target_os = "windows")))]
pub const EISCONN: c_int = 56;
#[cfg (target_os = "linux")]
pub const EISCONN: c_int = 106;
#[cfg (target_os = "windows")]
pub const EISCONN: c_int = 113;
/// <errno.h>: `#define EMSGSIZE 40` (Darwin) / `90` (Linux) / `115` (Windows UCRT).
#[cfg (not (any (target_os = "linux", target_os = "windows")))]
pub const EMSGSIZE: c_int = 40;
#[cfg (target_os = "linux")]
pub const EMSGSIZE: c_int = 90;
#[cfg (target_os = "windows")]
pub const EMSGSIZE: c_int = 115;
/// <errno.h>: `#define EPIPE 32`.
pub const EPIPE: c_int = 32;
/// <errno.h>: `#define EDOM 33`.
pub const EDOM: c_int = 33;
/// <errno.h>: `#define ECHILD 10`.
pub const ECHILD: c_int = 10;
/// <errno.h>: `#define EXDEV 18` (measured; `src/fileio.c`'s rename path is the one caller).
pub const EXDEV: c_int = 18;


/* Three codes `exec/`'s loader and the runtime arm name, measured with the same probe as the block
   above (`ENOEXEC 8`, `ESRCH 3`, `EFAULT 14`); `ELIBBAD` is Linux's and is *not* on this target, so
   `exec/mipsfpu.c` carries that one itself (status 3.446).  */
/// <errno.h>: `#define ENOEXEC 8`.
pub const ENOEXEC: c_int = 8;
/// <errno.h>: `#define ESRCH 3`.
pub const ESRCH: c_int = 3;
/// <errno.h>: `#define EFAULT 14`.
pub const EFAULT: c_int = 14;
/// <errno.h>: `#define E2BIG 7`.  The mirror did not carry it, so `lib/file-has-acl.rs` declared
/// it in place (P-72 clause 4) until status 3.514 moved it back here (P-88 (5)); the value is
/// measured on this machine's own header, and the same 7 every ABI `lib/` targets.
pub const E2BIG: c_int = 7;
/// <errno.h>: `#define ENOTCONN 57` (Darwin; bionic/Linux is 107; Windows UCRT is 126).
#[cfg (not (any (feature = "have_android", target_os = "linux", target_os = "windows")))]
pub const ENOTCONN: c_int = 57;
#[cfg (any (feature = "have_android", target_os = "linux"))]
pub const ENOTCONN: c_int = 107;
#[cfg (target_os = "windows")]
pub const ENOTCONN: c_int = 126;
