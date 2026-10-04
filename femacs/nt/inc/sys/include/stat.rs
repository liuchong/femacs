// femacs: mirror of nt/inc/sys/stat.h @ fd7afced403.
//
// The Windows port's `<sys/stat.h>`: MSVCRT's own header uses too narrow a type for the inode and
// the owner/group ids, so this one replaces it - its own `struct stat`, the `_stat`/`_stati64`
// shapes MinGW's `wchar.h` expects, the emulated `mode` bits, and the entry points.  The file is
// not on `docs/baseline/config/cond-eval-verified.txt`, so the extractor resolves no `#if` in it
// - the `__MINGW32__`, `__MINGW_NOTHROW`, `_SYS_STAT_H`, `_INC_STAT_H` and `__MSVCRT__` arms are
// all live text - and the whole file is carried.  Nothing in the frozen configuration includes it
// (no `nt/` translation unit is compiled - docs/baseline/frozen-build-sources.txt's batch-F
// section - and no `src/` unit includes a `nt/inc/` header), so `xtask layout-check` never
// compiles this mirror; in a MinGW build `src/fileio.c`, `src/buffer.c`, `src/filelock.c`,
// `src/pdumper.c` and `src/process.c` are among the many units whose `#include <sys/stat.h>`
// resolves here.
//
// Names with no mirror: the MinGW runtime's and MSVCRT's own type spellings - `dev_t`, `time_t`,
// `_dev_t`, `_ino_t`, `_mode_t`, `_off_t`, `struct timespec` - and `_mingw.h` (C 24-26).  They
// stay spelled the way C spells them and are named from the including unit, as
// `nt/inc/sys/include/acl.rs` does for `ssize_t`; the `__int64` fields are spelled
// `c_longlong`/`c_ulonglong`, PORTING.md 1.1's mapping for MSVC's 64-bit builtin.  `__cdecl` and
// `__MINGW_NOTHROW` on the prototypes (C 163-168) are calling-convention/attribute spellings with
// no Rust counterpart: the port writes the declarations in an `unsafe extern "C"` block, and
// `__MINGW_NOTHROW` is carried as the constant C 30 defines.

// C 21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const INC_SYS_STAT_H_: bool = true;
/// C 30: `# define __MINGW_NOTHROW` (the `#ifndef __MINGW_NOTHROW` fallback for MinGW before
/// 3.13).  C uses it as a throw specification on the prototypes below, which Rust spells by not
/// having exceptions.
pub const __MINGW_NOTHROW: bool = true;
// C 35-39: `_SYS_STAT_H` and `_INC_STAT_H` - the guards this header sets so MinGW's own
// `stat.h` is never included.
pub const _SYS_STAT_H: bool = true;
pub const _INC_STAT_H: bool = true;

// C 44-51: the file-type mask and its values (`S_IFMT` is `0xF800` here, not POSIX's `0xF000`).
pub const S_IFMT: c_int = 0xF800;
pub const S_IFREG: c_int = 0x8000;
pub const S_IFDIR: c_int = 0x4000;
pub const S_IFBLK: c_int = 0x3000;
pub const S_IFCHR: c_int = 0x2000;
pub const S_IFIFO: c_int = 0x1000;
pub const S_IFLNK: c_int = 0x0800;
// C 53-60: the permission bits and their aliases.
pub const S_IREAD: c_int = 0x0100;
pub const S_IWRITE: c_int = 0x0080;
pub const S_IEXEC: c_int = 0x0040;
pub const S_IRUSR: c_int = S_IREAD;
pub const S_IWUSR: c_int = S_IWRITE;
pub const S_IXUSR: c_int = S_IEXEC;
pub const S_IRWXU: c_int = S_IREAD | S_IWRITE | S_IEXEC;

// C 62-67: the file-type predicates.
macro_rules! S_ISREG { ($m:expr) => { (($m) & S_IFMT) == S_IFREG }; }
macro_rules! S_ISDIR { ($m:expr) => { (($m) & S_IFMT) == S_IFDIR }; }
macro_rules! S_ISBLK { ($m:expr) => { (($m) & S_IFMT) == S_IFBLK }; }
macro_rules! S_ISCHR { ($m:expr) => { (($m) & S_IFMT) == S_IFCHR }; }
macro_rules! S_ISFIFO { ($m:expr) => { (($m) & S_IFMT) == S_IFIFO }; }
macro_rules! S_ISLNK { ($m:expr) => { (($m) & S_IFMT) == S_IFLNK }; }

// C 70-78: the bits NT has not got but `lib/filemode.c` wants, and the group/other permissions
// derived by shifting the user's.
pub const S_ISUID: c_int = 0;
pub const S_ISGID: c_int = 0;
pub const S_ISVTX: c_int = 0;
pub const S_IRGRP: c_int = S_IRUSR >> 3;
pub const S_IROTH: c_int = S_IRUSR >> 6;
pub const S_IWGRP: c_int = S_IWUSR >> 3;
pub const S_IWOTH: c_int = S_IWUSR >> 6;
pub const S_IXGRP: c_int = S_IXUSR >> 3;
pub const S_IXOTH: c_int = S_IXUSR >> 6;

// C 80-92: the predicates for the file types NT cannot report; all are constant `0`.
macro_rules! S_ISSOCK { ($m:expr) => { 0 }; }
macro_rules! S_ISCTG { ($p:expr) => { 0 }; }
macro_rules! S_ISDOOR { ($m:expr) => { 0 }; }
macro_rules! S_ISMPB { ($m:expr) => { 0 }; }
macro_rules! S_ISMPC { ($m:expr) => { 0 }; }
macro_rules! S_ISMPX { ($m:expr) => { 0 }; }
macro_rules! S_ISNWK { ($m:expr) => { 0 }; }
macro_rules! S_ISPORT { ($m:expr) => { 0 }; }
macro_rules! S_ISWHT { ($m:expr) => { 0 }; }
macro_rules! S_TYPEISMQ { ($p:expr) => { 0 }; }
macro_rules! S_TYPEISSEM { ($p:expr) => { 0 }; }
macro_rules! S_TYPEISSHM { ($p:expr) => { 0 }; }
macro_rules! S_TYPEISTMO { ($p:expr) => { 0 }; }

// C 94-95: the two `utimensat` selectors (`-1` and `-2`).
pub const UTIME_NOW: c_int = -1;
pub const UTIME_OMIT: c_int = -2;

/// C 97-114: `struct stat` - C's own shape, with `st_ino`/`st_size` as the 64-bit fields
/// MSVCRT's narrower ones cannot hold.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stat
{
  pub st_ino: c_ulonglong,        /* ino_t in sys/types.h is too narrow */
  pub st_dev: dev_t,
  pub st_mode: c_ushort,
  pub st_nlink: c_short,
  pub st_uid: c_uint,             /* Vista's TrustedInstaller has a very large RID */
  pub st_gid: c_uint,
  pub st_size: c_ulonglong,
  pub st_rdev: dev_t,
  pub st_atime: time_t,
  pub st_mtime: time_t,
  pub st_ctime: time_t,
  pub st_uname: [c_char; 260],
  pub st_gname: [c_char; 260],
  pub st_atimensec: c_int,
  pub st_mtimensec: c_int,
  pub st_ctimensec: c_int,
}

/// C 117-131: `struct _stat` - `/* These are here to avoid compiler warnings when using
/// wchar.h.  */`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _stat
{
  pub st_dev: _dev_t,      /* Equivalent to drive number 0=A 1=B ... */
  pub st_ino: _ino_t,      /* Always zero ? */
  pub st_mode: _mode_t,    /* See above constants */
  pub st_nlink: c_short,   /* Number of links. */
  pub st_uid: c_short,     /* User: Maybe significant on NT ? */
  pub st_gid: c_short,     /* Group: Ditto */
  pub st_rdev: _dev_t,     /* Seems useless (not even filled in) */
  pub st_size: _off_t,     /* File size in bytes */
  pub st_atime: time_t,    /* Accessed date (always 00:00 hrs local on FAT) */
  pub st_mtime: time_t,    /* Modified time */
  pub st_ctime: time_t,    /* Creation time */
}

/// C 133-147: `struct _stati64` (the `__MSVCRT__` arm) - the same layout with a 64-bit
/// `st_size`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _stati64
{
  pub st_dev: _dev_t,
  pub st_ino: _ino_t,
  pub st_mode: _mode_t,
  pub st_nlink: c_short,
  pub st_uid: c_short,
  pub st_gid: c_short,
  pub st_rdev: _dev_t,
  pub st_size: c_longlong,
  pub st_atime: time_t,
  pub st_mtime: time_t,
  pub st_ctime: time_t,
}

// C 154 and 161: `_STAT_DEFINED` and `_WSTAT_DEFINED` - the two guards that stop other headers
// (`wchar.h`) from redefining the structs above.
pub const _STAT_DEFINED: bool = true;
pub const _WSTAT_DEFINED: bool = true;

pub mod stat_decls {
  use super::*;

  unsafe extern "C" {
    /// C 151: `extern int w32_stat_get_owner_group;` - the file's one file-scope object, which
    /// asks `stat`/`lstat` for accurate owner and group info.
    pub static mut w32_stat_get_owner_group: c_int;
    /// C 163: `int fstat (int, struct stat*);`
    pub fn fstat (fd: c_int, buffer: *mut stat) -> c_int;
    /// C 164: `int stat (const char*, struct stat*);`
    pub fn stat (name: *const c_char, buffer: *mut stat) -> c_int;
    /// C 165: `int lstat (const char*, struct stat*);`
    pub fn lstat (name: *const c_char, buffer: *mut stat) -> c_int;
    /// C 166-167: `int fstatat (int, char const *, struct stat *, int);`
    pub fn fstatat (fd: c_int, name: *const c_char, buffer: *mut stat, flags: c_int) -> c_int;
    /// C 168: `int chmod (const char*, int);`
    pub fn chmod (name: *const c_char, mode: c_int) -> c_int;
    /// C 173: `extern int utimensat (int, const char *, struct timespec const[2], int);`
    pub fn utimensat (fd: c_int, name: *const c_char, times: *const timespec,
                      flags: c_int) -> c_int;
  }
}
use stat_decls::*;
