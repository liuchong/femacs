// femacs: the parts of the frozen target's <sys/utsname.h> the port needs (P-26/P-50).
//
// Three units declared the record and `uname` for themselves until status 3.85
// (`src/dired.rs`, `src/editfns.rs`, `src/sysdep.rs`).

/// <sys/utsname.h>'s `struct utsname`, measured: `sizeof` 1280, `nodename`@256.
#[repr(C)]
pub struct utsname {
  pub sysname: [c_char; 256],
  pub nodename: [c_char; 256],
  pub release: [c_char; 256],
  pub version: [c_char; 256],
  pub machine: [c_char; 256],
}

const _: () = core::assert! (std::mem::size_of::<utsname> () == 1280);
const _: () = core::assert! (std::mem::offset_of! (utsname, nodename) == 256);

unsafe extern "C" {
  /// C: <sys/utsname.h>: `int uname (struct utsname *);`
  pub fn uname (name: *mut utsname) -> c_int;
}
