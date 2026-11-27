// femacs: mirror of the frozen target's <sys/acl.h> (system header, P-26).
//
// The frozen configuration leaves HAVE_SYS_ACL_H undefined, so gnulib's acl-internal.h does not
// include this header at all and the ported units never *call* into it; the types are still what
// their signatures mention (`acl_entries` takes an `acl_t`), so the measured ABI lives here.
//
//     $ cat > /tmp/acl.c <<'EOF'
//     #include <sys/acl.h>
//     #include <stdio.h>
//     int main(void){ printf("sizeof(acl_t)=%zu sizeof(acl_entry_t)=%zu sizeof(acl_permset_t)=%zu "
//       "sizeof(acl_flagset_t)=%zu\n", sizeof(acl_t), sizeof(acl_entry_t), sizeof(acl_permset_t),
//       sizeof(acl_flagset_t)); printf("sizeof(acl_type_t)=%zu sizeof(acl_tag_t)=%zu "
//       "sizeof(acl_flag_t)=%zu sizeof(acl_perm_t)=%zu\n", sizeof(acl_type_t), sizeof(acl_tag_t),
//       sizeof(acl_flag_t), sizeof(acl_perm_t)); printf("ACL_TYPE_EXTENDED=%d ACL_TYPE_ACCESS=%d "
//       "ACL_TYPE_DEFAULT=%d ACL_FIRST_ENTRY=%d ACL_NEXT_ENTRY=%d\n", (int)ACL_TYPE_EXTENDED,
//       (int)ACL_TYPE_ACCESS, (int)ACL_TYPE_DEFAULT, ACL_FIRST_ENTRY, ACL_NEXT_ENTRY); return 0; }
//     EOF
//     $ clang -o /tmp/acl /tmp/acl.c && /tmp/acl
//     sizeof(acl_t)=8 sizeof(acl_entry_t)=8 sizeof(acl_permset_t)=8 sizeof(acl_flagset_t)=8
//     sizeof(acl_type_t)=4 sizeof(acl_tag_t)=4 sizeof(acl_flag_t)=4 sizeof(acl_perm_t)=4
//     ACL_TYPE_EXTENDED=256 ACL_TYPE_ACCESS=0 ACL_TYPE_DEFAULT=1 ACL_FIRST_ENTRY=0 ACL_NEXT_ENTRY=-1

// [U1-MODULE-MIRROR]
use super::*;
/// `<sys/acl.h>`'s opaque handle types: all four are `void *` on this target (measured).
pub type acl_t = *mut core::ffi::c_void;
pub type acl_entry_t = *mut core::ffi::c_void;
pub type acl_permset_t = *mut core::ffi::c_void;
pub type acl_flagset_t = *mut core::ffi::c_void;
/// The four scalar ACL types are 32-bit ints on this target (measured).
pub type acl_type_t = c_int;
pub type acl_tag_t = c_int;
pub type acl_flag_t = c_int;
pub type acl_perm_t = c_int;

/* The values the measured probe printed, plus the ones the C names (measured the same way).  */
pub const ACL_TYPE_EXTENDED: acl_type_t = 0x100;
pub const ACL_TYPE_ACCESS: acl_type_t = 0;
pub const ACL_TYPE_DEFAULT: acl_type_t = 1;
pub const ACL_FIRST_ENTRY: c_int = 0;
pub const ACL_NEXT_ENTRY: c_int = -1;
pub const ACL_UNDEFINED_TAG: acl_tag_t = 0;
pub const ACL_EXTENDED_ALLOW: acl_tag_t = 1;
pub const ACL_EXTENDED_DENY: acl_tag_t = 2;
pub const ACL_ENTRY_INHERITED: acl_flag_t = 0x10;
pub const ACL_FLAG_DEFER_INHERIT: acl_flag_t = 1;
pub const ACL_FLAG_NO_INHERIT: acl_flag_t = 0x20000;
pub const ACL_ENTRY_FILE_INHERIT: acl_flag_t = 0x20;
pub const ACL_ENTRY_DIRECTORY_INHERIT: acl_flag_t = 0x40;
pub const ACL_ENTRY_LIMIT_INHERIT: acl_flag_t = 0x80;
pub const ACL_ENTRY_ONLY_INHERIT: acl_flag_t = 0x100;
/* The `__DARWIN_ACL_*` permission bits (measured).  */
pub const __DARWIN_ACL_READ_DATA: acl_perm_t = 2;
pub const __DARWIN_ACL_WRITE_DATA: acl_perm_t = 4;
pub const __DARWIN_ACL_EXECUTE: acl_perm_t = 8;
pub const __DARWIN_ACL_DELETE: acl_perm_t = 16;
pub const __DARWIN_ACL_APPEND_DATA: acl_perm_t = 32;
pub const __DARWIN_ACL_DELETE_CHILD: acl_perm_t = 64;
pub const __DARWIN_ACL_READ_ATTRIBUTES: acl_perm_t = 128;
pub const __DARWIN_ACL_WRITE_ATTRIBUTES: acl_perm_t = 256;
pub const __DARWIN_ACL_READ_EXTATTRIBUTES: acl_perm_t = 512;
pub const __DARWIN_ACL_WRITE_EXTATTRIBUTES: acl_perm_t = 1024;
pub const __DARWIN_ACL_READ_SECURITY: acl_perm_t = 2048;
pub const __DARWIN_ACL_WRITE_SECURITY: acl_perm_t = 4096;
pub const __DARWIN_ACL_CHANGE_OWNER: acl_perm_t = 8192;
pub const __DARWIN_ACL_SYNCHRONIZE: acl_perm_t = 1048576;

// [U1] sys_types.in.rs imported via super
// `ssize_t` is C's `long` on this target and the port already spells it that way in the unistd
// mirror; a system-header mirror does not include that one, so the one declaration that needs it
// (`acl_to_text`) writes `c_long` directly.

unsafe extern "C" {
  pub fn acl_get_file (path: *const c_char, type_: acl_type_t) -> acl_t;
  pub fn acl_get_fd_np (fd: c_int, type_: acl_type_t) -> acl_t;
  pub fn acl_set_file (path: *const c_char, type_: acl_type_t, acl: acl_t) -> c_int;
  pub fn acl_set_fd_np (fd: c_int, acl: acl_t, type_: acl_type_t) -> c_int;
  pub fn acl_free (obj: *mut core::ffi::c_void) -> c_int;
  pub fn acl_get_entry (acl: acl_t, entry_id: c_int, entry: *mut acl_entry_t) -> c_int;
  pub fn acl_get_permset (entry: acl_entry_t, permset: *mut acl_permset_t) -> c_int;
  pub fn acl_get_flagset_np (entry: acl_entry_t, flagset: *mut acl_flagset_t) -> c_int;
  pub fn acl_get_tag_type (entry: acl_entry_t, tag: *mut acl_tag_t) -> c_int;
  pub fn acl_clear_perms (permset: acl_permset_t) -> c_int;
  pub fn acl_add_perm (permset: acl_permset_t, perm: acl_perm_t) -> c_int;
  pub fn acl_clear_flags_np (flagset: acl_flagset_t) -> c_int;
  pub fn acl_add_flag_np (flagset: acl_flagset_t, flag: acl_flag_t) -> c_int;
  pub fn acl_create_entry (acl: *mut acl_t, entry: *mut acl_entry_t) -> c_int;
  pub fn acl_create_entry_np (acl: *mut acl_t, entry: *mut acl_entry_t, index: c_int) -> c_int;
  pub fn acl_delete_entry (acl: acl_t, entry: acl_entry_t) -> c_int;
  pub fn acl_set_tag_type (entry: acl_entry_t, tag: acl_tag_t) -> c_int;
  pub fn acl_delete_def_file (path: *const c_char) -> c_int;
  pub fn acl_extended_file (path: *const c_char) -> c_int;
  pub fn acl_extended_fd (fd: c_int) -> c_int;
  pub fn acl_init (count: c_int) -> acl_t;
  pub fn acl_dup (acl: acl_t) -> acl_t;
  pub fn acl_valid (acl: acl_t) -> c_int;
  pub fn acl_cmp (a1: acl_t, a2: acl_t) -> c_int;
  pub fn acl_equiv_mode (acl: acl_t, mode_p: *mut mode_t) -> c_int;
  pub fn acl_from_mode_np (mode: mode_t) -> acl_t;
  pub fn acl_to_text (acl: acl_t, len_p: *mut c_long) -> *mut c_char;
  pub fn acl_from_text (buf: *const c_char) -> acl_t;
  pub fn acl_free_text (buf: *mut c_char);
}
