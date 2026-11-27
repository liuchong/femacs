// femacs: mirror of the frozen target's <sys/select.h> (system header, P-26/P-50).
//
// gnulib has no `sys_select.in.h` wrapper in this build; the reference includes the system
// header, so this carries the entries the ported units name.  The layout and the constants are
// the frozen target's, measured with clang:
//
//     $ cat > /tmp/sg.c <<'EOF'
//     #include <signal.h>
//     #include <sys/select.h>
//     #include <stddef.h>
//     #include <stdio.h>
//     int main (void) { printf ("%zu %d %zu %zu\n", sizeof (fd_set), FD_SETSIZE,
//         sizeof (__int32_t), offsetof (fd_set, fds_bits)); return 0; }
//     EOF
//     $ clang -o /tmp/sg /tmp/sg.c && /tmp/sg
//     128 1024 4 0
//
// i.e. `fd_set` is `__DARWIN_FD_SETSIZE / __DARWIN_NFDBITS` (1024/32) = 32 `__int32_t`s.
//
// `FD_ISSET` is the header's `__header_always_inline int __darwin_fd_isset (int, const struct
// fd_set *)`: a bounds check through `__darwin_check_fd_set` followed by one bit test.  Its
// `if ((uintptr_t) &__darwin_check_fd_set_overflow != (uintptr_t) 0)` guard exists for systems
// that lack the symbol (with `_DARWIN_C_SOURCE` defined it passes the third argument 1 —
// config.h 2447-2448 defines `_DARWIN_C_SOURCE`, macros.tsv 4041); this target's libSystem
// exports it, so the port's call is unconditional (docs/ports/include-sys_select.md).

// [U1-MODULE-MIRROR]
use super::*;
// [U1] sys_time.rs imported via super
// [U1] time.in.rs imported via super

/* <sys/select.h> 91: `#define FD_SETSIZE __DARWIN_FD_SETSIZE` (1024).  */
pub const FD_SETSIZE: c_int = 1024;

/// <sys/select.h>'s `__DARWIN_NFDBITS`: `sizeof (__int32_t) * CHAR_BIT`.
pub const __DARWIN_NFDBITS: c_int = 32;

/// <sys/select.h>'s `fd_set`, i.e. Darwin's `struct fd_set { __int32_t fds_bits[32]; }`.
///
/// bionic's is the same 128 bytes with a different element type - `fd_mask` is
/// `unsigned long`, so `<sys/select.h>` 66's `fd_mask fds_bits[FD_SETSIZE/NFDBITS]` is
/// `[c_ulong; 16]` on a 64-bit target.  Platform predicates are `target_os` (PORTING.md P-290),
/// which is why this is a cfg rather than a configuration macro (status 4.728).
#[cfg (not (any (target_os = "android", target_os = "linux")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fd_set
{
  pub fds_bits: [i32; 32],
}

#[cfg (any (target_os = "android", target_os = "linux"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fd_set
{
  /// <sys/select.h> 66: `fd_mask fds_bits[FD_SETSIZE/NFDBITS]` with `fd_mask` = `c_ulong`.
  pub fds_bits: [core::ffi::c_ulong; 16],
}

const _: () = core::assert! (std::mem::size_of::<fd_set> () == 128);
const _: () = core::assert! (std::mem::offset_of! (fd_set, fds_bits) == 0);

unsafe extern "C" {
  /* int select (int, fd_set *restrict, fd_set *restrict, fd_set *restrict,
                struct timeval *restrict);  */
  pub fn select (nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set,
                 exceptfds: *mut fd_set, timeout: *mut timeval) -> c_int;
  /* int pselect (int, fd_set *restrict, fd_set *restrict, fd_set *restrict,
                  const struct timespec *restrict, const sigset_t *restrict);  */
  pub fn pselect (nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set,
                  exceptfds: *mut fd_set, timeout: *const timespec,
                  sigmask: *const sigset_t) -> c_int;
  /* int __darwin_check_fd_set_overflow (int, const void *, int);  - Darwin's; bionic has no
     such entry, its `FD_ISSET`/`FD_SET`/`FD_CLR` are plain bit operations (status 4.728).  */
  #[cfg (not (any (target_os = "android", target_os = "linux")))]
  pub fn __darwin_check_fd_set_overflow (fd: c_int, p: *const c_void, flag: c_int) -> c_int;
}

#[cfg (not (any (target_os = "android", target_os = "linux")))]
/// <sys/select.h>'s `#define FD_ISSET(n, p) __DARWIN_FD_ISSET (n, p)` →
/// `__darwin_fd_isset ((n), (p))`, the header's `__header_always_inline` definition.
#[inline(always)]
pub unsafe fn FD_ISSET (fd: c_int, p: *const fd_set) -> bool
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  if __darwin_check_fd_set (fd, p as *const c_void) != 0
    {
      return ((*p).fds_bits [(fd as u64 / __DARWIN_NFDBITS as u64) as usize]
              & 1i32.wrapping_shl ((fd as u64 % __DARWIN_NFDBITS as u64) as u32)) != 0;
    }

  return false;

  }
}

#[cfg (not (any (target_os = "android", target_os = "linux")))]
/// `__darwin_check_fd_set`: with `_DARWIN_C_SOURCE` defined (config.h 2447-2448) the
/// overflow entry is called with the third argument 1.
#[inline(always)]
pub unsafe fn __darwin_check_fd_set (fd: c_int, p: *const c_void) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  return __darwin_check_fd_set_overflow (fd, p, 1);

  }
}

#[cfg (not (any (target_os = "android", target_os = "linux")))]
/// <sys/select.h>'s `#define FD_SET(n, p) __DARWIN_FD_SET (n, p)` → `__darwin_fd_set ((n), (p))`,
/// the header's `__header_always_inline` definition: the same `__darwin_check_fd_set` guard as
/// `FD_ISSET`, then `fds_bits[fd / __DARWIN_NFDBITS] |= 1 << (fd % __DARWIN_NFDBITS)`.
#[inline(always)]
pub unsafe fn FD_SET (fd: c_int, p: *mut fd_set)
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  if __darwin_check_fd_set (fd, p as *const c_void) != 0
    {
      (*p).fds_bits [(fd as u64 / __DARWIN_NFDBITS as u64) as usize]
        |= 1i32.wrapping_shl ((fd as u64 % __DARWIN_NFDBITS as u64) as u32);
    }

  }
}

#[cfg (not (any (target_os = "android", target_os = "linux")))]
/// `__darwin_fd_clr`: the same guard, then
/// `fds_bits[fd / __DARWIN_NFDBITS] &= ~(1 << (fd % __DARWIN_NFDBITS))`.
#[inline(always)]
pub unsafe fn FD_CLR (fd: c_int, p: *mut fd_set)
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  if __darwin_check_fd_set (fd, p as *const c_void) != 0
    {
      (*p).fds_bits [(fd as u64 / __DARWIN_NFDBITS as u64) as usize]
        &= !1i32.wrapping_shl ((fd as u64 % __DARWIN_NFDBITS as u64) as u32);
    }

  }
}

/* bionic & glibc Linux `<sys/select.h>`: `__FD_SET`/`__FD_ISSET`/`__FD_CLR` are plain bit
   operations over `fds_bits`, with `__FDELT(fd) = fd / 64` and `__FDMASK(fd) = 1UL << (fd % 64)`. */
#[cfg (any (target_os = "android", target_os = "linux"))]
#[inline(always)]
pub unsafe fn FD_ISSET (fd: c_int, p: *const fd_set) -> bool
{
  // SAFETY: Caller passes valid `*const fd_set` and non-negative fd < FD_SETSIZE.
  unsafe {
  ((*p).fds_bits [(fd as usize) / 64] & (1 as core::ffi::c_ulong) << ((fd as usize) % 64)) != 0
  }
}

#[cfg (any (target_os = "android", target_os = "linux"))]
#[inline(always)]
pub unsafe fn FD_SET (fd: c_int, p: *mut fd_set)
{
  // SAFETY: Caller passes valid `*mut fd_set` and non-negative fd < FD_SETSIZE.
  unsafe {
  (*p).fds_bits [(fd as usize) / 64] |= (1 as core::ffi::c_ulong) << ((fd as usize) % 64);
  }
}

#[cfg (any (target_os = "android", target_os = "linux"))]
#[inline(always)]
pub unsafe fn FD_CLR (fd: c_int, p: *mut fd_set)
{
  // SAFETY: Caller passes valid `*mut fd_set` and non-negative fd < FD_SETSIZE.
  unsafe {
  (*p).fds_bits [(fd as usize) / 64] &= !((1 as core::ffi::c_ulong) << ((fd as usize) % 64));
  }
}

/// <sys/select.h>'s `#define FD_ZERO(p) __DARWIN_FD_ZERO (p)` →
/// `__builtin_bzero (p, sizeof (*(p)))`: all 128 bytes of the `fd_set` become 0.
#[inline(always)]
pub unsafe fn FD_ZERO (p: *mut fd_set)
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  std::ptr::write_bytes (p, 0, 1);

  }
}
