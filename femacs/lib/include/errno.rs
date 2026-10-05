// femacs: the parts of the system <errno.h> the port needs (PORTING.md 0.1).  C's `errno` is
// not an object on the frozen target but the macro `(*__error ())`, so the mirror keeps the
// name as an accessor rather than a static; `c_int` comes from the including unit's scope, as
// the other lib mirrors expect.
//
// ENOMEM's value was measured from the frozen target's clang:
// `printf` of `int x = ENOMEM;` expands to `int x = 12;`.

unsafe extern "C" {
  /// <errno.h>: the per-thread error number's address (`int *__error (void)`).
  pub fn __error () -> *mut c_int;
}

/// <errno.h>'s `errno` on the frozen target: `#define errno (*__error ())`, so C's
/// `errno = 0;` is the port's `*errno () = 0;` and C's `errno != 0` its `*errno () != 0`.
#[inline(always)]
pub unsafe fn errno () -> *mut c_int { __error () }

/// <errno.h>: `#define EINVAL 22`.
pub const EINVAL: c_int = 22;

/// <errno.h>: `#define ENOSYS 78`.
pub const ENOSYS: c_int = 78;

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
/// <errno.h>: `#define ENOTSUP 45`.
pub const ENOTSUP: c_int = 45;
/// <errno.h>: `#define ELOOP 62`.
pub const ELOOP: c_int = 62;
/// <errno.h>: `#define ENAMETOOLONG 63`.
pub const ENAMETOOLONG: c_int = 63;
/// <errno.h>: `#define EOVERFLOW 84`.
pub const EOVERFLOW: c_int = 84;

/// <errno.h>: `#define EOPNOTSUPP 102` (measured; the same code as ENOTSUP on Linux, a
/// different one here, which is why lib/lchmod.c can name either).
pub const EOPNOTSUPP: c_int = 102;

/// <errno.h>: `#define EISDIR 21` (measured: `printf ("%d", EISDIR)` -> 21).
pub const EISDIR: c_int = 21;

/* The socket/file codes `src/process.c`, `src/dired.c`, `src/sysdep.c` and `lib/dup2.c` each
   declared for themselves until status 3.80; measured with `clang -E -dM -include errno.h`
   (the same measurement ENOMEM's comment above records).  C spells `EWOULDBLOCK` as
   `#define EWOULDBLOCK EAGAIN`, so the port keeps both names and the same value.  */
/// <errno.h>: `#define EAGAIN 35`.
pub const EAGAIN: c_int = 35;
/// <errno.h>: `#define EWOULDBLOCK EAGAIN`.
pub const EWOULDBLOCK: c_int = 35;
/// <errno.h>: `#define EMFILE 24`.
pub const EMFILE: c_int = 24;
/// <errno.h>: `#define EINPROGRESS 36` - **except on the MS-DOS build**, where the system header
/// has no such name (`lib/errno.in.h` 97-98 says so in the comment above its own `#ifndef`) and
/// `src/msdos.h` 80 defines it as 112 first; the two would collide at a crate root that includes
/// both mirrors (measured 2026-10-05, status 3.590: `E0428` on the `msdos` arm of `src/dosfns.rs`).
#[cfg (not (feature = "msdos"))]
pub const EINPROGRESS: c_int = 36;
/// <errno.h>: `#define EISCONN 56`.
pub const EISCONN: c_int = 56;
/// <errno.h>: `#define EMSGSIZE 40`.
pub const EMSGSIZE: c_int = 40;
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
