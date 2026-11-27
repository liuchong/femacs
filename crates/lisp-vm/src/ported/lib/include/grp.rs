// femacs: the parts of the frozen target's <grp.h> the port needs (system header, P-26/P-50).
//
// Three units declared the record and `getgrgid` for themselves until status 3.85
// (`src/dired.rs`, `src/editfns.rs`, `src/sysdep.rs`); `gid_t` comes from the including unit's
// scope, like the other lib mirrors.

// [U1-MODULE-MIRROR]
use super::*;
/// <grp.h>'s `struct group`, measured: `sizeof` 32, `gr_name`@0, `gr_gid`@16, `gr_mem`@24.
#[repr(C)]
pub struct group {
  pub gr_name: *mut c_char,
  pub gr_passwd: *mut c_char,
  pub gr_gid: gid_t,
  pub gr_mem: *mut *mut c_char,
}

const _: () = core::assert! (std::mem::size_of::<group> () == 32);
const _: () = core::assert! (std::mem::offset_of! (group, gr_name) == 0);
const _: () = core::assert! (std::mem::offset_of! (group, gr_gid) == 16);
const _: () = core::assert! (std::mem::offset_of! (group, gr_mem) == 24);

unsafe extern "C" {
  /// C: <grp.h>: `struct group *getgrgid (gid_t);`
  pub fn getgrgid (gid: gid_t) -> *mut group;
}
