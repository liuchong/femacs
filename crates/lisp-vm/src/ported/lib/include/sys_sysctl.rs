// femacs: mirror of the frozen target's <sys/sysctl.h> (system header, P-26; the
// include/sys_mount.rs shape).  Values measured from the frozen macro table
// (docs/baseline/config/darwin-arm64.macros.tsv): `CTL_HW` = 6, `HW_NCPU` = 3; the target
// has no `HW_NCPUONLINE` row, so lib/nproc.c C 332-334's `#ifdef HW_NCPUONLINE` arm is dead
// (the port names it at its site).
//
// The only consumer today is lib/nproc.c's macOS/BSD fallback (C 326-345): the mib array
// `{ CTL_HW, HW_NCPU }` and the `sysctl` call that reads the CPU count into an `int`.

// [U1-MODULE-MIRROR]
use super::*;
/// <sys/sysctl.h>: `#define CTL_HW 6` (the hardware subtree selector).
pub const CTL_HW: c_int = 6;

/// <sys/sysctl.h>: `#define HW_NCPU 3` (the "number of CPUs" mib key).
pub const HW_NCPU: c_int = 3;

unsafe extern "C" {
  /* int sysctl (int *, u_int, void *, size_t *, void *, size_t);  */
  pub fn sysctl (name: *mut c_int, namelen: c_uint, oldp: *mut c_void,
                 oldlenp: *mut usize, newp: *mut c_void, newlen: usize) -> c_int;
}
