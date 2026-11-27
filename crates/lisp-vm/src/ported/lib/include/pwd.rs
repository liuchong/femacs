// femacs: the parts of the frozen target's <pwd.h> the port needs (system header, P-26/P-50).
//
// Four units declared the record and the lookups for themselves until status 3.85
// (`src/dired.rs`, `src/editfns.rs`, `src/fileio.rs`, `src/sysdep.rs`); the layout is
// clang-measured on the frozen target and pinned by `const _` below.  `uid_t`/`gid_t`/`time_t`
// come from the including unit's scope (`lib/include/sys_types.in.rs` and `time.in.rs`), like the
// other lib mirrors.

// [U1-MODULE-MIRROR]
use super::*;
/// <pwd.h>'s `struct passwd`, measured on the frozen target (macOS): `sizeof` 72,
/// `pw_name`@0, `pw_uid`@16, `pw_dir`@48, `pw_shell`@56, `pw_expire`@64.  The Android arm below
/// is bionic's LP64 record (measured against the NDK's `<pwd.h>`), which has no `pw_change` /
/// `pw_class` / `pw_expire` and puts `pw_dir` at 32.
#[cfg (not (feature = "have_android"))]
#[repr(C)]
pub struct passwd {
  pub pw_name: *mut c_char,
  pub pw_passwd: *mut c_char,
  pub pw_uid: uid_t,
  pub pw_gid: gid_t,
  pub pw_change: time_t,
  pub pw_class: *mut c_char,
  pub pw_gecos: *mut c_char,
  pub pw_dir: *mut c_char,
  pub pw_shell: *mut c_char,
  pub pw_expire: time_t,
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<passwd> () == 72);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_name) == 0);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_uid) == 16);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_dir) == 48);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_shell) == 56);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_expire) == 64);

/// bionic's `<pwd.h>` record (Android, LP64): `sizeof` 48, `pw_name`@0, `pw_uid`@16,
/// `pw_gecos`@24, `pw_dir`@32, `pw_shell`@40.  Reading the macOS layout here used to make
/// `get_homedir` load `pw_dir` from offset 48, i.e. past the end of the record
/// (`docs/baseline/android-tombstone-dump.txt`: SEGV at `0x746f6f72` = "root" read as a pointer).
#[cfg (feature = "have_android")]
#[repr(C)]
pub struct passwd {
  pub pw_name: *mut c_char,
  pub pw_passwd: *mut c_char,
  pub pw_uid: uid_t,
  pub pw_gid: gid_t,
  pub pw_gecos: *mut c_char,
  pub pw_dir: *mut c_char,
  pub pw_shell: *mut c_char,
}

#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<passwd> () == 48);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_name) == 0);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_uid) == 16);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_gecos) == 24);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_dir) == 32);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_shell) == 40);

unsafe extern "C" {
  /// C: <pwd.h>: `struct passwd *getpwuid (uid_t);`
  pub fn getpwuid (uid: uid_t) -> *mut passwd;
  /// C: <pwd.h>: `struct passwd *getpwnam (const char *);`
  pub fn getpwnam (name: *const c_char) -> *mut passwd;
  /// C: <pwd.h>: `struct passwd *getpwent (void);`  (`src/dired.c`'s `list-system-processes`.)
  pub fn getpwent () -> *mut passwd;
}
