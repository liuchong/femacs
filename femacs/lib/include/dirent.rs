// femacs: mirror of the frozen target's <dirent.h> (system header, P-26/P-50).
//
// gnulib's `lib/dirent.in.h` is a template (its `@…@` placeholders keep it off the verified
// list) and the port's units include the system header directly, so this mirror carries the
// entries they name: the opaque `DIR` (the stdio.rs `FILE` shape) and the directory calls.
// `struct dirent` is not mirrored yet - no ported unit reads its fields.

/// <dirent.h>'s opaque `DIR`; the reference only ever passes `DIR *`.
pub enum DIR {}

unsafe extern "C" {
  pub fn opendir (name: *const c_char) -> *mut DIR;
  pub fn fdopendir (fd: c_int) -> *mut DIR;
  pub fn readdir (dirp: *mut DIR) -> *mut c_void;
  pub fn closedir (dirp: *mut DIR) -> c_int;
  pub fn dirfd (dirp: *mut DIR) -> c_int;
  pub fn rewinddir (dirp: *mut DIR);
  pub fn seekdir (dirp: *mut DIR, loc: c_long);
  pub fn telldir (dirp: *mut DIR) -> c_long;
}
