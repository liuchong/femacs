// femacs: WIP mirror of lib/sys_stat.in.h (gnulib's <sys/stat.h>; PORTING.md P-26).
//
// In C this header is a two-layer thing: it `#include_next`s the system <sys/stat.h> (which is
// where `struct stat`, the `S_IF*` values and every declaration come from on this target), then
// adds gnulib's aliases and warnings.  The frozen configuration leaves every `REPLACE_*` macro
// zero, so all of gnulib's replacement arms are dead and the aliases resolve to the system
// entries; the port therefore declares the system entries themselves, and `struct stat` - which
// no reference file in the tree defines - is measured from the frozen target:
//
//     $ cat > /tmp/stprobe.c <<'EOF'
//     #include <sys/stat.h>
//     #include <stdio.h>
//     int main (void) {
//       printf ("sizeof=%zu align=%zu\n", sizeof (struct stat), _Alignof (struct stat));
//       printf ("st_dev@%zu st_mode@%zu st_nlink@%zu st_ino@%zu st_uid@%zu st_gid@%zu st_rdev@%zu\n",
//               offsetof (struct stat, st_dev), offsetof (struct stat, st_mode),
//               offsetof (struct stat, st_nlink), offsetof (struct stat, st_ino),
//               offsetof (struct stat, st_uid), offsetof (struct stat, st_gid),
//               offsetof (struct stat, st_rdev));
//       printf ("st_atimespec@%zu st_mtimespec@%zu st_ctimespec@%zu st_birthtimespec@%zu\n",
//               offsetof (struct stat, st_atimespec), offsetof (struct stat, st_mtimespec),
//               offsetof (struct stat, st_ctimespec), offsetof (struct stat, st_birthtimespec));
//       printf ("st_size@%zu st_blocks@%zu st_blksize@%zu st_flags@%zu st_gen@%zu st_lspare@%zu st_qspare@%zu\n",
//               offsetof (struct stat, st_size), offsetof (struct stat, st_blocks),
//               offsetof (struct stat, st_blksize), offsetof (struct stat, st_flags),
//               offsetof (struct stat, st_gen), offsetof (struct stat, st_lspare),
//               offsetof (struct stat, st_qspare));
//       printf ("S_IFMT=%d S_IFDIR=%d S_IFLNK=%d S_IFREG=%d S_IFCHR=%d S_IFBLK=%d S_IFIFO=%d S_IFSOCK=%d\n",
//               (int) S_IFMT, (int) S_IFDIR, (int) S_IFLNK, (int) S_IFREG, (int) S_IFCHR,
//               (int) S_IFBLK, (int) S_IFIFO, (int) S_IFSOCK);
//       printf ("UTIME_NOW=%ld UTIME_OMIT=%ld\n", (long) UTIME_NOW, (long) UTIME_OMIT);
//       return 0; }
//     EOF
//     $ clang -o /tmp/stprobe /tmp/stprobe.c && /tmp/stprobe
//     sizeof=144 align=8
//     st_dev@0 st_mode@4 st_nlink@6 st_ino@8 st_uid@16 st_gid@20 st_rdev@24
//     st_atimespec@32 st_mtimespec@48 st_ctimespec@64 st_birthtimespec@80
//     st_size@96 st_blocks@104 st_blksize@112 st_flags@116 st_gen@120 st_lspare@124 st_qspare@128
//     S_IFMT=61440 S_IFDIR=16384 S_IFLNK=40960 S_IFREG=32768 S_IFCHR=8192 S_IFBLK=24576 S_IFIFO=4096 S_IFSOCK=49152
//     UTIME_NOW=-1 UTIME_OMIT=-2
//
// The unchecked boxes of docs/ports/include-sys_stat.in.md are the rest of the header: the
// `_GL_WARN_ON_USE` machinery, the `_GL_CXXALIAS_*` aliases and the declarations this mirror
// has not needed yet.

include!("sys_types.in.rs");   // <sys/types.h> (mode_t, dev_t, ino_t, nlink_t, off_t, ...)
include!("time.in.rs");        // <time.h> (struct timespec, the st_*timespec members' type)

/// The frozen target's `struct stat` (144 bytes, 8-byte alignment; layouts measured above).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stat
{
  pub st_dev: dev_t,
  pub st_mode: mode_t,
  pub st_nlink: nlink_t,
  pub st_ino: ino_t,
  pub st_uid: uid_t,
  pub st_gid: gid_t,
  pub st_rdev: dev_t,
  pub st_atimespec: timespec,
  pub st_mtimespec: timespec,
  pub st_ctimespec: timespec,
  pub st_birthtimespec: timespec,
  pub st_size: off_t,
  pub st_blocks: blkcnt_t,
  pub st_blksize: blksize_t,
  pub st_flags: c_uint,
  pub st_gen: c_uint,
  pub st_lspare: c_int,
  pub st_qspare: [i64; 2],
}

const _: () = assert! (std::mem::size_of::<stat> () == 144);
const _: () = assert! (std::mem::offset_of! (stat, st_dev) == 0);
const _: () = assert! (std::mem::offset_of! (stat, st_mode) == 4);
const _: () = assert! (std::mem::offset_of! (stat, st_nlink) == 6);
const _: () = assert! (std::mem::offset_of! (stat, st_ino) == 8);
const _: () = assert! (std::mem::offset_of! (stat, st_uid) == 16);
const _: () = assert! (std::mem::offset_of! (stat, st_gid) == 20);
const _: () = assert! (std::mem::offset_of! (stat, st_rdev) == 24);
const _: () = assert! (std::mem::offset_of! (stat, st_atimespec) == 32);
const _: () = assert! (std::mem::offset_of! (stat, st_mtimespec) == 48);
const _: () = assert! (std::mem::offset_of! (stat, st_ctimespec) == 64);
const _: () = assert! (std::mem::offset_of! (stat, st_birthtimespec) == 80);
const _: () = assert! (std::mem::offset_of! (stat, st_size) == 96);
const _: () = assert! (std::mem::offset_of! (stat, st_blocks) == 104);
const _: () = assert! (std::mem::offset_of! (stat, st_blksize) == 112);

/* The mode bits (the frozen target's <sys/stat.h>, measured above).  */
pub const S_IFMT: mode_t = 0o170000;
pub const S_IFDIR: mode_t = 0o040000;
pub const S_IFCHR: mode_t = 0o020000;
pub const S_IFBLK: mode_t = 0o060000;
pub const S_IFREG: mode_t = 0o100000;
pub const S_IFIFO: mode_t = 0o010000;
pub const S_IFLNK: mode_t = 0o120000;
pub const S_IFSOCK: mode_t = 0o140000;
pub const S_ISUID: mode_t = 0o4000;
pub const S_ISGID: mode_t = 0o2000;
pub const S_ISVTX: mode_t = 0o1000;
pub const S_IRWXU: mode_t = 0o700;
pub const S_IRUSR: mode_t = 0o400;
pub const S_IWUSR: mode_t = 0o200;
pub const S_IXUSR: mode_t = 0o100;
pub const S_IRWXG: mode_t = 0o070;
pub const S_IRGRP: mode_t = 0o040;
pub const S_IWGRP: mode_t = 0o020;
pub const S_IXGRP: mode_t = 0o010;
pub const S_IRWXO: mode_t = 0o007;
pub const S_IROTH: mode_t = 0o004;
pub const S_IWOTH: mode_t = 0o002;
pub const S_IXOTH: mode_t = 0o001;

/* C 230-234's `#ifndef S_ISDIR ... #endif` and its siblings: gnulib's copies of the system
   macros.  The system header defines them on this target, so the arms are dead in C, but the
   port keeps gnulib's bodies (the ledger compares them).  */
macro_rules! S_ISDIR { ($m:expr) => { (($m) & S_IFMT) == S_IFDIR }; }
macro_rules! S_ISLNK { ($m:expr) => { (($m) & S_IFMT) == S_IFLNK }; }
macro_rules! S_ISREG { ($m:expr) => { (($m) & S_IFMT) == S_IFREG }; }
macro_rules! S_ISCHR { ($m:expr) => { (($m) & S_IFMT) == S_IFCHR }; }
macro_rules! S_ISBLK { ($m:expr) => { (($m) & S_IFMT) == S_IFBLK }; }
macro_rules! S_ISFIFO { ($m:expr) => { (($m) & S_IFMT) == S_IFIFO }; }
macro_rules! S_ISSOCK { ($m:expr) => { (($m) & S_IFMT) == S_IFSOCK }; }

/* The `utimensat` special values (the frozen target's <sys/stat.h>, measured above).  */
pub const UTIME_NOW: c_long = -1;
pub const UTIME_OMIT: c_long = -2;

unsafe extern "C" {
  /* C 511-530's `_GL_CXXALIAS_SYS (fstat, ...)` and the `stat`/`lstat` twins resolve to the
     system entries in the frozen configuration (no `REPLACE_*` macro is nonzero).  */
  pub fn stat (file: *const c_char, buf: *mut stat) -> c_int;
  pub fn lstat (file: *const c_char, buf: *mut stat) -> c_int;
  pub fn fstat (fd: c_int, buf: *mut stat) -> c_int;
  /* C 542-568's `fstatat`.  */
  pub fn fstatat (fd: c_int, file: *const c_char, buf: *mut stat, flag: c_int) -> c_int;
  /* C 439-499's `chmod`, `fchmod` and `fchmodat` (the last one `_GL_ATTRIBUTE_NODISCARD`).  */
  pub fn chmod (file: *const c_char, mode: mode_t) -> c_int;
  pub fn fchmod (fd: c_int, mode: mode_t) -> c_int;
  pub fn fchmodat (fd: c_int, file: *const c_char, mode: mode_t, flag: c_int) -> c_int;
  /* `faccessat` (declared by the same header; the `AT_EACCESS` arm is dead, `REPLACE_FACCESSAT`
     is zero).  */
  pub fn faccessat (fd: c_int, file: *const c_char, mode: c_int, flag: c_int) -> c_int;
  /* `mkdir`/`mkdirat` and `mknod`/`mknodat`.  */
  pub fn mkdir (file: *const c_char, mode: mode_t) -> c_int;
  pub fn mkdirat (fd: c_int, file: *const c_char, mode: mode_t) -> c_int;
  pub fn mknod (file: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
  pub fn mknodat (fd: c_int, file: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
  /* C 584-597's `futimens` and `utimensat`.  */
  pub fn futimens (fd: c_int, times: *const timespec) -> c_int;
  pub fn utimensat (fd: c_int, file: *const c_char, times: *const timespec, flag: c_int)
    -> c_int;
  /* `stat`'s mode-setting twin from the same header: `umask`.  */
  pub fn umask (mask: mode_t) -> mode_t;
}
