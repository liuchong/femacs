// femacs: the parts of the frozen target's <pwd.h> the port needs (system header, P-26/P-50).
//
// Four units declared the record and the lookups for themselves until status 3.85
// (`src/dired.rs`, `src/editfns.rs`, `src/fileio.rs`, `src/sysdep.rs`); the layout is
// clang-measured on the frozen target and pinned by `const _` below.  `uid_t`/`gid_t`/`time_t`
// come from the including unit's scope (`lib/include/sys_types.in.rs` and `time.in.rs`), like the
// other lib mirrors.

/// <pwd.h>'s `struct passwd`, measured: `sizeof` 72, `pw_name`@0, `pw_uid`@16, `pw_dir`@48,
/// `pw_shell`@56, `pw_expire`@64.
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

const _: () = core::assert! (std::mem::size_of::<passwd> () == 72);
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_name) == 0);
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_uid) == 16);
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_dir) == 48);
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_shell) == 56);
const _: () = core::assert! (std::mem::offset_of! (passwd, pw_expire) == 64);

unsafe extern "C" {
  /// C: <pwd.h>: `struct passwd *getpwuid (uid_t);`
  pub fn getpwuid (uid: uid_t) -> *mut passwd;
  /// C: <pwd.h>: `struct passwd *getpwnam (const char *);`
  pub fn getpwnam (name: *const c_char) -> *mut passwd;
  /// C: <pwd.h>: `struct passwd *getpwent (void);`  (`src/dired.c`'s `list-system-processes`.)
  pub fn getpwent () -> *mut passwd;
}
