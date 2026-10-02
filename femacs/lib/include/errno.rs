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
