// femacs: mirror of lib/fsusage.h @ fd7afced403 (PORTING.md P-26).
//
// The header's `struct fs_usage` (C 31-40) and the one prototype (C 42).  C's `uintmax_t`
// fields are the port's `u64` (lisp.h's alias; the lib mirrors stand alone, limits.in.rs), and
// C's `bool` is Rust's `bool` (one byte in both).  `FSUSAGE_H_` (C 21-22) and the
// `#ifdef __cplusplus` pair (C 26-47) are the include guard and the C++ wrapper: PORTING.md
// 0.1 drops both.
//
// The declaration sits in `pub mod fsusage_decls` + glob import (the stdio.rs/getopt-core.rs
// shape): lib/fsusage.c defines `get_fs_usage`, and an explicit item shadows a glob import
// (PORTING.md 0.1-10).

// [U1-MODULE-MIRROR]
use super::*;
/* C 21-22's include guard `FSUSAGE_H_` has no Rust counterpart (PORTING.md 0.1 fixes the
   include! graph instead); the port keeps the name as the marker constant, the idx.rs shape.  */
pub const FSUSAGE_H_: bool = true;

/* Space usage statistics for a file system.  Blocks are 512-byte. */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fs_usage {
  pub fsu_blocksize: u64,     /* Size of a block.  */
  pub fsu_blocks: u64,        /* Total blocks. */
  pub fsu_bfree: u64,         /* Free blocks available to superuser. */
  pub fsu_bavail: u64,        /* Free blocks available to non-superuser. */
  pub fsu_bavail_top_bit_set: bool, /* 1 if fsu_bavail represents a value < 0.  */
  pub fsu_files: u64,         /* Total file nodes. */
  pub fsu_ffree: u64,         /* Free file nodes. */
}

pub mod fsusage_decls {
  use super::*;

  unsafe extern "C" {
    /* int get_fs_usage (char const *file, char const *disk, struct fs_usage *fsp);  */
    pub fn get_fs_usage (file: *const c_char, disk: *const c_char, fsp: *mut fs_usage)
      -> c_int;
  }
}
pub use self::fsusage_decls::*;
