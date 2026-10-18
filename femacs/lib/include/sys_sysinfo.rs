// femacs: mirror of the system's <sys/sysinfo.h> (PORTING.md 0.1), measured from the Android NDK's
// header (`ndk/29.0.14206865/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/include/`).
//
// The file exists because gnulib's `boot-time-aux.h` uses `struct sysinfo` and `sysinfo ()` in its
// `# if HAVE_DECL_SYSINFO` arm (C 66-75) without including the header itself: C reaches it through
// the configuration's own include chain (`HAVE_SYS_SYSINFO_H`), and on the frozen target the arm
// is dead, so nothing in the port needed the mirror until the Android configuration compiled it
// (status 4.708, `femacs/lib/include/boot-time-aux.rs`).
//
// bionic's `<sys/sysinfo.h>` declares `sysinfo ()` (line 46) and gets the struct from
// `<linux/kernel.h>`, which includes `<linux/sysinfo.h>` 11-26.  The kernel's field types are
// `__kernel_long_t` (`c_long` on aarch64), `__kernel_ulong_t` (`c_ulong`), `__u16` (`u16`) and
// `__u32` (`u32`); the trailing `char _f[20 - 2 * sizeof(__kernel_ulong_t) - sizeof(__u32)]` is
// `[c_char; 0]` for those widths, and it is carried as a named field so the size stays the
// kernel's rather than becoming "some struct Rust lays out".

/// <linux/sysinfo.h> 11-26: `struct sysinfo`.
#[repr (C)]
#[derive (Clone, Copy)]
pub struct sysinfo
{
  pub uptime: core::ffi::c_long,
  pub loads: [core::ffi::c_ulong; 3],
  pub totalram: core::ffi::c_ulong,
  pub freeram: core::ffi::c_ulong,
  pub sharedram: core::ffi::c_ulong,
  pub bufferram: core::ffi::c_ulong,
  pub totalswap: core::ffi::c_ulong,
  pub freeswap: core::ffi::c_ulong,
  pub procs: core::ffi::c_ushort,
  pub pad: core::ffi::c_ushort,
  pub totalhigh: core::ffi::c_ulong,
  pub freehigh: core::ffi::c_ulong,
  pub mem_unit: core::ffi::c_uint,
  pub _f: [core::ffi::c_char; 20 - 2 * 8 - 4],
}

unsafe extern "C" {
  /// <sys/sysinfo.h> 46: `int sysinfo (struct sysinfo *__info)`.
  pub fn sysinfo (__info: *mut sysinfo) -> core::ffi::c_int;
}
