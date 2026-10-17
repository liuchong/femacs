// femacs: mirror of the `<sys/sysinfo.h>` surface alloc.c 68 includes (`HAVE_LINUX_SYSINFO`).
//
// C never declares `struct sysinfo` - the platform's header does.  The port has to, and bionic's
// layout is fixed (aarch64: eight 8-byte counters, the procs/pad pair, two more counters, then
// `mem_unit`), so only the fields alloc.c 7170-7205 reads are named; the rest is one padding
// array of the same total size so that `sizeof (struct sysinfo)` stays the system's.

#[cfg (feature = "have_linux_sysinfo")]
#[repr (C)]
pub struct sysinfo
{
  pub uptime: core::ffi::c_ulong,
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
  pub __reserved: [core::ffi::c_char; 0],
}

#[cfg (feature = "have_linux_sysinfo")]
unsafe extern "C" {
  pub fn sysinfo (info: *mut sysinfo) -> core::ffi::c_int;
}
