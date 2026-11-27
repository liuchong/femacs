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

// [U1-MODULE-MIRROR]
use super::*;
// [U1] sys_types.in.rs imported via super
// [U1] time.in.rs imported via super

/* C 78-94's `#ifndef _GL_ATTRIBUTE_NOTHROW` fallback: the attribute that says a function does
   not throw.  C defines it five times (the C++ arms and the `__attribute__ ((__nothrow__))`
   arm); the port drops attributes (PORTING.md 0.3) and spells it as the no-op macro its call
   sites do not need - the `attribute.rs` 184 shape, which this header's own `#ifndef` guard
   makes a second, identical definition of in C.  */
macro_rules! _GL_ATTRIBUTE_NOTHROW {
  () => {};
}

/* C 172-174: the marker of the dead `#if !GNULIB_defined_struct_stat` arm above (it says "the
   second half of this header already defined the type").  */
pub const GNULIB_defined_struct_stat: c_int = 1;

#[cfg (not (any (feature = "have_android", target_os = "linux")))]
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
  /* C 118-175's other *definition* of this type - the `#if @GNULIB_OVERRIDES_STRUCT_STAT@`
     arm, dead on this target - spells the three timestamps as members this target's `stat`
     does not have: `struct timespec st_atim/st_mtim/st_ctim` in its
     `#if @WINDOWS_STAT_TIMESPEC@` arm (C 152-158) and `time_t st_atime/st_mtime/st_ctime` in
     the `#else` arm (C 159-162).  The port has one `stat` (the system's, above), so the other
     configuration's six members are carried as `#[cfg]`-gated fields under the features that
     name the C conditions (P-51): the extractor reads this template blind and asks for every
     field name it can define, while the layout assertions below still measure the live
     struct.  */
  #[cfg (feature = "windows_stat_timespec")]
  pub st_atim: timespec,
  #[cfg (feature = "windows_stat_timespec")]
  pub st_mtim: timespec,
  #[cfg (feature = "windows_stat_timespec")]
  pub st_ctim: timespec,
  #[cfg (feature = "no_windows_stat_timespec")]
  pub st_atime: time_t,
  #[cfg (feature = "no_windows_stat_timespec")]
  pub st_mtime: time_t,
  #[cfg (feature = "no_windows_stat_timespec")]
  pub st_ctime: time_t,
}

/// C's `<sys/stat.h>` differs per target, and `struct stat` is written by the C library itself, so
/// the record has to be the target's.  The host's is above (144 bytes); bionic's arm64 record is
/// 128 bytes with a different order entirely, measured with the NDK for aarch64-linux-android21
/// (`char a[sizeof (struct stat)]; char b[offsetof (struct stat, st_size)];` … read back with
/// `llvm-nm -S`: size 128, `st_ino` 8, `st_mode` 16, `st_nlink` 20, `st_uid` 24, `st_gid` 28,
/// `st_rdev` 32, `st_size` 48, `st_blksize` 56, `st_blocks` 64, `st_atim` 72, `st_mtim` 88,
/// `st_ctim` 104).  A host-shaped record made every `stat`-based decision on the device read
/// another field's bytes (status 4.794).  */
#[cfg (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stat
{
  pub st_dev: dev_t,          /* 0 */
  pub st_ino: ino_t,          /* 8 */
  pub st_mode: mode_t,        /* 16 */
  pub st_nlink: nlink_t,      /* 20 */
  pub st_uid: uid_t,          /* 24 */
  pub st_gid: gid_t,          /* 28 */
  pub st_rdev: dev_t,         /* 32 */
  pub __pad1: u64,            /* 40 */
  pub st_size: off_t,         /* 48 */
  pub st_blksize: c_int,      /* 56 */
  pub __pad2: c_int,          /* 60 */
  pub st_blocks: blkcnt_t,    /* 64 */
  pub st_atim: timespec,      /* 72 */
  pub st_mtim: timespec,      /* 88 */
  pub st_ctim: timespec,      /* 104 */
  pub __unused4: c_uint,      /* 120 */
  pub __unused5: c_uint,      /* 124 */
}

#[cfg (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64")))]
const _: () = core::assert! (std::mem::size_of::<stat> () == 128);
#[cfg (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_ino) == 8);
#[cfg (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_mode) == 16);
#[cfg (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_nlink) == 20);
#[cfg (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_size) == 48);
#[cfg (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_blocks) == 64);
#[cfg (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_mtim) == 88);
#[cfg (any (feature = "have_android", all (target_os = "linux", target_arch = "aarch64")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_ctim) == 104);

#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stat
{
  pub st_dev: dev_t,          /* 0 */
  pub st_ino: ino_t,          /* 8 */
  pub st_nlink: nlink_t,      /* 16 */
  pub st_mode: mode_t,        /* 24 */
  pub st_uid: uid_t,          /* 28 */
  pub st_gid: gid_t,          /* 32 */
  pub __pad0: c_int,          /* 36 */
  pub st_rdev: dev_t,         /* 40 */
  pub st_size: off_t,         /* 48 */
  pub st_blksize: blksize_t,  /* 56 */
  pub st_blocks: blkcnt_t,    /* 64 */
  pub st_atim: timespec,      /* 72 */
  pub st_mtim: timespec,      /* 88 */
  pub st_ctim: timespec,      /* 104 */
  pub __glibc_reserved: [c_long; 3], /* 120 */
}

#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
const _: () = core::assert! (std::mem::size_of::<stat> () == 144);
#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_nlink) == 16);
#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_mode) == 24);
#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_size) == 48);
#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_blocks) == 64);
#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_mtim) == 88);
#[cfg (all (target_os = "linux", target_arch = "x86_64"))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_ctim) == 104);

#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::size_of::<stat> () == 144);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_dev) == 0);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_mode) == 4);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_nlink) == 6);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_ino) == 8);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_uid) == 16);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_gid) == 20);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_rdev) == 24);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_atimespec) == 32);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_mtimespec) == 48);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_ctimespec) == 64);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_birthtimespec) == 80);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_size) == 96);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_blocks) == 104);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (stat, st_blksize) == 112);

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
pub const S_IRUSR: mode_t = 0o400;
pub const S_IWUSR: mode_t = 0o200;
pub const S_IXUSR: mode_t = 0o100;

/* C 372-429's `#if !S_IRGRP / # define S_IRGRP (S_IRUSR >> 3)` family: the nine names whose
   C body is an expression are spelled with C's own expression, because the extractor compares
   the tokens of the body (the literals above are what C 352-398 define them as, and the
   system header defines them too - the arms are dead here).  */
pub const S_IRGRP: mode_t = S_IRUSR >> 3;
pub const S_IROTH: mode_t = S_IRUSR >> 6;
pub const S_IWGRP: mode_t = S_IWUSR >> 3;
pub const S_IWOTH: mode_t = S_IWUSR >> 6;
pub const S_IXGRP: mode_t = S_IXUSR >> 3;
pub const S_IXOTH: mode_t = S_IXUSR >> 6;
pub const S_IRWXU: mode_t = S_IRUSR | S_IWUSR | S_IXUSR;
pub const S_IRWXG: mode_t = S_IRGRP | S_IWGRP | S_IXGRP;
pub const S_IRWXO: mode_t = S_IROTH | S_IWOTH | S_IXOTH;

/* C 418-425: the two composed masks gnulib adds for the apps that use them (neither is POSIX
   and neither is in the system header).  */
pub const S_IXUGO: mode_t = S_IXUSR | S_IXGRP | S_IXOTH;
pub const S_IRWXUGO: mode_t = S_IRWXU | S_IRWXG | S_IRWXO;

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
     system entries in the frozen configuration (no `REPLACE_*` macro is nonzero).  **All five of
     them are also names `src/w32.c` defines** (C 5782/5788/5794/5826/5946), so - like the
     `fchmod`/`fchmodat`/`faccessat` trio below - they are gated out of the `windowsnt` arm (the
     `unistd.in.rs` 67 `getwd` device; `docs/ports/w32.md` Owed 16 measured five such rows).  */
  #[cfg (not (feature = "windowsnt"))]
  pub fn stat (file: *const c_char, buf: *mut stat) -> c_int;
  #[cfg (not (feature = "windowsnt"))]
  pub fn lstat (file: *const c_char, buf: *mut stat) -> c_int;
  #[cfg (not (feature = "windowsnt"))]
  pub fn fstat (fd: c_int, buf: *mut stat) -> c_int;
  /* C 542-568's `fstatat`.  */
  #[cfg (not (feature = "windowsnt"))]
  pub fn fstatat (fd: c_int, file: *const c_char, buf: *mut stat, flag: c_int) -> c_int;
  /* C 439-499's `chmod`, `fchmod` and `fchmodat` (the last one `_GL_ATTRIBUTE_NODISCARD`), and
     the `faccessat` below.  **The three whose `w32.c` counterparts are definitions** (`w32.c`
     4177/4757/4763 define `faccessat`, `fchmod` and `fchmodat` for the Windows arm) are gated
     out of the `windowsnt` arm: C's own `<sys/stat.h>` there is `nt/inc/sys/stat.h`, and a
     declaration at this scope is an `E0428` against those definitions (the `unistd.in.rs` 67
     `getwd` device; `docs/ports/w32.md` Owed 14 measured 3 such rows before this gate).  */
  pub fn chmod (file: *const c_char, mode: mode_t) -> c_int;
  #[cfg (not (feature = "windowsnt"))]
  pub fn fchmod (fd: c_int, mode: mode_t) -> c_int;
  #[cfg (not (feature = "windowsnt"))]
  pub fn fchmodat (fd: c_int, file: *const c_char, mode: mode_t, flag: c_int) -> c_int;
  /* `faccessat` (declared by the same header; the `AT_EACCESS` arm is dead, `REPLACE_FACCESSAT`
     is zero).  */
  #[cfg (not (feature = "windowsnt"))]
  pub fn faccessat (fd: c_int, file: *const c_char, mode: c_int, flag: c_int) -> c_int;
  /* `mkdir`/`mkdirat` and `mknod`/`mknodat`.  */
  pub fn mkdir (file: *const c_char, mode: mode_t) -> c_int;
  pub fn mkdirat (fd: c_int, file: *const c_char, mode: mode_t) -> c_int;
  pub fn mknod (file: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
  pub fn mknodat (fd: c_int, file: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
  /* C 584-597's `futimens` and `utimensat`.  (`utimensat` is the fifth name `src/w32.c`
     defines, C 5946; `futimens` is not.)  */
  pub fn futimens (fd: c_int, times: *const timespec) -> c_int;
  #[cfg (not (feature = "windowsnt"))]
  pub fn utimensat (fd: c_int, file: *const c_char, times: *const timespec, flag: c_int)
    -> c_int;
  /* `stat`'s mode-setting twin from the same header: `umask`.  */
  pub fn umask (mask: mode_t) -> mode_t;
}

/* C 238-352's fallbacks for the file-type predicates this target's <sys/stat.h> does not
   define (every one of them is `0` here, which is why `ftypelet`'s "nonstandard" tests in
   lib/filemode.c are the tests that answer '?' on this platform).  The `#else` arms that name
   S_IFMPB/S_IFMPC/S_IFNWK/S_ISNAM (AIX, V7, HP-UX) are dead on this target and their tokens
   are in docs/ports/macro-body-notes.tsv.  */
macro_rules! S_ISDOOR { ($m:expr) => { 0 }; }
macro_rules! S_ISCTG { ($m:expr) => { 0 }; }
macro_rules! S_ISMPB { ($m:expr) => { 0 }; }
macro_rules! S_ISMPC { ($m:expr) => { 0 }; }
macro_rules! S_ISMPX { ($m:expr) => { 0 }; }
macro_rules! S_ISNWK { ($m:expr) => { 0 }; }
macro_rules! S_ISPORT { ($m:expr) => { 0 }; }
macro_rules! S_ISWHT { ($m:expr) => { 0 }; }
macro_rules! S_TYPEISMQ { ($p:expr) => { 0 }; }
macro_rules! S_TYPEISTMO { ($p:expr) => { 0 }; }
macro_rules! S_TYPEISSEM { ($p:expr) => { 0 }; }
macro_rules! S_TYPEISSHM { ($p:expr) => { 0 }; }

/* ---- the template's own half: the `@PLACEHOLDER@` arms the frozen substitution settles ---
   Every name below is a `#define` of an arm that is dead on this target; each stands under the
   feature naming the C condition that selects it (P-08/P-51), because `extract-abi.py` reads
   this template blind and asks for the name (and, for a single-definition macro, its body's
   tokens).  */

/* C 186-194: `#ifndef S_IFIFO / # ifdef _S_IFIFO / #  define S_IFIFO _S_IFIFO`.  The system
   header defines `S_IFIFO`, so the arm is dead; the mirror's `S_IFIFO` constant above is the
   system value and this is C's other spelling of the name.  */
#[cfg (feature = "no_s_ififo")]
macro_rules! S_IFIFO { () => { _S_IFIFO } }

/* C 276-291: `# ifdef S_IFNAM / ... / # else / #  define S_ISNAM(m) 0` (and its two
   neighbours, the V7 / Stellix `S_ISNWK`-family tests C 262-291).  C defines `S_ISNAM`
   twice, and every one of these bodies is `0` on this target.  */
macro_rules! S_ISNAM { ($m:expr) => { 0 }; }
/* C 336-349: the Cray DMF pair.  */
macro_rules! S_ISOFD { ($p:expr) => { 0 }; }
macro_rules! S_ISOFL { ($p:expr) => { 0 }; }

/* C 145-171: the field-renaming macros of the other `struct stat` definition (see the
   `#[cfg]`-gated fields above): in the `#if @WINDOWS_STAT_TIMESPEC@` arm the three names are
   spelled over the timespec members.  */
#[cfg (feature = "windows_stat_timespec")]
macro_rules! st_atime { () => { st_atim.tv_sec } }
#[cfg (feature = "windows_stat_timespec")]
macro_rules! st_mtime { () => { st_mtim.tv_sec } }
#[cfg (feature = "windows_stat_timespec")]
macro_rules! st_ctime { () => { st_ctim.tv_sec } }
/* C 172: the arm's "gnulib internal purposes" indicator.  */
#[cfg (feature = "windows_stat_timespec")]
pub const _GL_WINDOWS_STAT_TIMESPEC: c_int = 1;

/* C 656-661 / C 678-683: the two `static int rpl_mkdir (char const *name, mode_t mode) {
   return _mkdir (name); }` definitions of the native-Windows arms (`@REPLACE_MKDIR@` and
   `@GNULIB_MDA_MKDIR@`).  C defines it as a function; `_mkdir` is not on this target, so the
   mirror carries the name and C's call as a macro rather than a definition that could never
   be compiled.  */
#[cfg (feature = "windows")]
macro_rules! rpl_mkdir { ($name:expr, $mode:expr) => { _mkdir ($name) } }
/* C 662/684: the marker that guards the second definition (`#    define
   GNULIB_defined_rpl_mkdir 1`); C defines the name in two arms, so the extractor asks for
   presence rather than the body.  */
#[cfg (feature = "windows")]
pub const GNULIB_defined_rpl_mkdir: c_int = 1;

/* C 720-722 / C 745-747: the `@REPLACE_MKFIFO@`/`@REPLACE_MKFIFOAT@` renames (dead: no
   `REPLACE_*` of the frozen configuration is set for either - the four `mknod`/`mknodat`/
   `futimens`/`utimensat` names of the same family are reached through the declarations
   above).  */
#[cfg (feature = "replace_mkfifo")]
macro_rules! mkfifo { () => { rpl_mkfifo } }
#[cfg (feature = "replace_mkfifoat")]
macro_rules! mkfifoat { () => { rpl_mkfifoat } }

/* C 828-886: the `@GNULIB_STAT@`/`@REPLACE_STAT@` block's five platform renames of the
   `stat` family, all dead here (`@REPLACE_STAT@` is 0).  C's body is `rpl_stat (name, st)`
   in every one of them, and the AIX / MinGW / MSVC / `@WINDOWS_64_BIT_ST_SIZE@` conditions
   are the features named.  */
#[cfg (feature = "aix_large_files")]
macro_rules! stat64 { ($name:expr, $st:expr) => { rpl_stat ($name, $st) } }
#[cfg (feature = "windows_64_bit_st_size")]
macro_rules! _stati64 { ($name:expr, $st:expr) => { rpl_stat ($name, $st) } }
#[cfg (feature = "windows")]
macro_rules! _stat32i64 { ($name:expr, $st:expr) => { rpl_stat ($name, $st) } }
#[cfg (feature = "windows")]
macro_rules! _stat64 { ($name:expr, $st:expr) => { rpl_stat ($name, $st) } }
#[cfg (feature = "windows")]
macro_rules! _stat32 { ($name:expr, $st:expr) => { rpl_stat ($name, $st) } }
#[cfg (feature = "windows")]
macro_rules! _stat64i32 { ($name:expr, $st:expr) => { rpl_stat ($name, $st) } }
