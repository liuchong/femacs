// femacs: mirror of nt/inc/dirent.h @ fd7afced403.
//
// The Windows port's own `<dirent.h>`: POSIX-compatible directory access, put on the include
// path ahead of the system headers by `nt/inc`.  This file is not on
// `docs/baseline/config/cond-eval-verified.txt`, so the extractor reads its statements without
// resolving `#if` - both arms of `__MINGW_MAJOR_VERSION >= 4` and both `MAXNAMLEN` arms are
// text here.  Nothing in the frozen configuration includes it either: no `nt/` translation unit
// is compiled (docs/baseline/frozen-build-sources.txt's batch-F section) and no `src/` unit
// includes a `nt/inc/` header, so `xtask layout-check` never compiles this mirror.  In a MinGW
// build `src/dired.c`, `src/w32.c`, `src/dosfns.c` and `src/msdos.c` are among the units whose
// `#include <dirent.h>` resolves here.
//
// The system types have no mirror in the port: `_fsize_t` is the MinGW runtime's own typedef and
// stays spelled the way C spells it, named from the including unit (as `nt/inc/sys/include/acl.rs`
// does for `ssize_t`); the `__int64` fields are spelled `c_longlong`, PORTING.md 1.1's mapping for
// MSVC's 64-bit builtin.  The frozen target's `<sys/types.h>` mirror describes darwin's types
// rather than MinGW's, so it is deliberately not pulled in here.

/// C 9: `#define DIRBLKSIZ 512`        `/* size of directory block */`
pub const DIRBLKSIZ: c_int = 512;
/// C 11 and 13: `#define MAXNAMLEN 255` under `WINDOWSNT`, `#define MAXNAMLEN 15` in the
/// `#else` arm (`/* maximum filename length */`), with C's note that it must be one less than
/// a multiple of 4.  C defines the name twice, so the value follows the arm the frozen
/// configuration's macro table would pick - `WINDOWSNT` undefined, i.e. 15, the arm the
/// extractor records (line 13).  A real NT build takes 255, because `nt/inc/ms-w32.h` defines
/// `WINDOWSNT` for it.
pub const MAXNAMLEN: c_int = 15;

/// C 17-44: `struct dirent`             `/* data from readdir() */`
///
/// The `#if __MINGW_MAJOR_VERSION >= 4` block is the MinGW.org runtime 4.x layout, which the
/// startup code's `readdir` call requires; its five fields are carried here because the
/// extractor reads the whole file as live text (see the head note).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dirent
{
  pub d_ino: c_long,                     /* inode number of entry */
  pub d_reclen: c_ushort,                /* length of this record */
  pub d_namlen: c_ushort,                /* length of string in d_name */
  pub d_type: c_uint,                    /* File attributes (MinGW 4.x) */
  pub d_time_create: c_longlong,         /* `__int64` in C (MSVC's 64-bit builtin) */
  pub d_time_access: c_longlong,         /* always midnight local time */
  pub d_time_write: c_longlong,
  pub d_size: _fsize_t,
  pub d_name: [c_char; (MAXNAMLEN * 4 + 1) as usize],   /* name of file */
}

/// C 46-52: `typedef struct { ... } DIR;`  `/* stream data from opendir() */`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DIR
{
  pub dd_fd: c_int,                      /* file descriptor */
  pub dd_loc: c_int,                     /* offset in block */
  pub dd_size: c_int,                    /* amount of valid data */
  pub dd_buf: [c_char; DIRBLKSIZ as usize],  /* directory block */
}

pub mod dirent_decls {
  use super::*;

  unsafe extern "C" {
    /// C 54: `extern DIR *opendir (const char *);`
    pub fn opendir (name: *const c_char) -> *mut DIR;
    /// C 55: `extern struct dirent *readdir (DIR *);`
    pub fn readdir (dirp: *mut DIR) -> *mut dirent;
    /// C 56: `extern void seekdir (DIR *, long);`
    pub fn seekdir (dirp: *mut DIR, loc: c_long);
    /// C 57: `extern void closedir (DIR *);`
    pub fn closedir (dirp: *mut DIR);
  }
}
use dirent_decls::*;

/// C 59: `#define rewinddir( dirp ) seekdir( dirp, 0L )`
macro_rules! rewinddir {
  ($dirp:expr) => { seekdir ($dirp, 0) };
}
