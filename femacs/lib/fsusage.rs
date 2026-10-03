// femacs: port of lib/fsusage.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// lib/fsusage.c is in `docs/baseline/config/cond-eval-verified.txt`, so the gate evaluates its
// 14 conditions against the frozen configuration and compares only the live text.  The live
// arm is `STAT_STATFS2_BSIZE` (1 in the frozen table): C 26's `#if STAT_STATVFS ||
// STAT_STATVFS64` is false, so C 28-49's `#else` include block is the live one and brings in
// <sys/mount.h> (HAVE_SYS_MOUNT_H is 1) for `struct statfs` and `statfs`; inside `get_fs_usage`
// the `STAT_STATVFS` block (C 116-139), the `STAT_STATVFS64`/`STAT_STATFS2_FRSIZE` arms, the
// `STATFS_TRUNCATES_BLOCK_COUNTS` fixup, the `STAT_STATFS2_FSIZE` and `STAT_STATFS4` arms and
// C 75-103's `#ifdef STAT_STATVFS` `statvfs_works` pair are dead, and each is named at its site
// below (P-51).  C 211-213's block is live (its `STAT_STATFS2_BSIZE` disjunct is the one that
// holds), which is the six `fsd` assignments the port writes.
//
// The port reaches the system entries through the new `include/sys_mount.rs` mirror (the
// measured `struct statfs` layout) and lib/fsusage.h through `include/fsusage.rs`.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* fsusage.c -- return space usage of mounted file systems

   Copyright (C) 1991-1992, 1996, 1998-1999, 2002-2006, 2009-2026 Free Software
   Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/limits.in.rs");
include!("include/stdint.in.rs");
include!("include/sys_types.in.rs");
include!("include/fcntl.in.rs");
include!("include/unistd.in.rs");
include!("include/sys_stat.in.rs");
include!("include/sys_mount.rs");
include!("include/fsusage.rs");

/* C 19-24: <config.h>, <fsusage.h>, <limits.h> and <sys/types.h> are the mirrors above; C 31-42's
   `#include <fcntl.h>`/`<unistd.h>`/`<sys/stat.h>`/`<sys/param.h>`/`<sys/mount.h>` are the
   `#else` arm of C 26, which is the live one (`<sys/param.h>`'s MAXPATHLEN/MNAMELEN land in the
   sys_mount mirror with the `struct statfs` they size).  */

/* Many space usage primitives use all 1 bits to denote a value that is
   not applicable or unknown.  Propagate this information by returning
   a uintmax_t value that is all 1 bits if X is all 1 bits, even if X
   is unsigned and narrower than uintmax_t.  */
/* C 55-60's `#define PROPAGATE_ALL_ONES(x)`.  C's integer promotions make the
   `sizeof (x) < sizeof (int)` arm an all-ones test for every width; the port keeps C's
   expression shape with explicit uintmax_t casts (every call site in this file passes a
   32- or 64-bit field, where the two spellings are the same test exactly).  */
macro_rules! PROPAGATE_ALL_ONES {
  ($x:expr) => {
    (if std::mem::size_of_val (&$x) < std::mem::size_of::<u64> ()
         && (! ($x) as u64
             == (if std::mem::size_of_val (&$x) < std::mem::size_of::<c_int> ()
                   { (- (1i64 << (std::mem::size_of_val (&$x) * CHAR_BIT as usize)))
                       as u64 }
                   else
                   { 0 }))
       { UINTMAX_MAX }
     else
       { ($x) as u64 })
  };
}

/* Extract the top bit of X as an uintmax_t value.  */
/* C 63-64's `#define EXTRACT_TOP_BIT(x)`.  */
macro_rules! EXTRACT_TOP_BIT {
  ($x:expr) => {
    (($x) as u64
     & (1u64 << (std::mem::size_of_val (&$x) * CHAR_BIT as usize - 1)))
  };
}

/* If a value is negative, many space usage primitives store it into an
   integer variable by assignment, even if the variable's type is unsigned.
   So, if a space usage variable X's top bit is set, convert X to the
   uintmax_t value V such that (- (uintmax_t) V) is the negative of
   the original value.  If X's top bit is clear, just yield X.
   Use PROPAGATE_TOP_BIT if the original value might be negative;
   otherwise, use PROPAGATE_ALL_ONES.  */
/* C 73's `#define PROPAGATE_TOP_BIT(x)`.  */
macro_rules! PROPAGATE_TOP_BIT {
  ($x:expr) => {
    (($x) as u64 | ! (EXTRACT_TOP_BIT! ($x) - 1))
  };
}

/* C 75-103's `#ifdef STAT_STATVFS` arm is dead (`STAT_STATVFS` is undefined): it defines
   `statvfs_works` either as the constant-1 stub (C 83) or, for glibc/Linux, as the
   uname/strverscmp probe (C 90-101).  Neither is written out (P-51); the live `get_fs_usage`
   below therefore starts at C 141's `#if defined STAT_STATVFS64` chain instead.  */

/* Fill in the fields of FSP with information about space usage for
   the file system on which FILE resides.
   DISK is the device on which FILE is mounted, for space-getting
   methods that need to know it.
   Return 0 if successful, -1 if not.  When returning -1, ensure that
   ERRNO is either a system error value, or zero if DISK is NULL
   on a system that requires a non-NULL value.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_fs_usage (file: *const c_char, disk: *const c_char,
                                       fsp: *mut fs_usage) -> c_int
{
  /* C 116-139's `#ifdef STAT_STATVFS` block (`statvfs_works ()` plus the `struct statvfs vfsd`
     reads of f_frsize/f_bsize/f_blocks/f_bfree/f_bavail/f_files/f_ffree) is dead: the macro is
     undefined.  */

  /* C 141-151's `#if defined STAT_STATVFS64` arm (`struct statvfs64 fsd; statvfs64 (...)`) is
     dead: the macro is undefined.  */

  /* C 153-160's `#elif defined STAT_STATFS2_FRSIZE` arm (`fsd.f_frsize`) is dead.  */

  /* C 162-171's `#elif defined STAT_STATFS2_BSIZE` arm is the live text.  C's `struct statfs
     fsd;` is a function-scope object (`#elif` does not open a scope), which is why C 215-220
     can read it after the arm closes; the port declares it here, at the same statement
     position, and the later block reads the same binding.  */
  let mut fsd: statfs = std::mem::zeroed ();

  if statfs (file, &mut fsd) < 0
    { return -1; }

  (*fsp).fsu_blocksize = PROPAGATE_ALL_ONES! (fsd.f_bsize);

  /* C 173-186's `# ifdef STATFS_TRUNCATES_BLOCK_COUNTS` fixup (the SunOS 4.1.2/4.1.3
     `f_spare` correction) is dead: the macro is undefined.  */

  /* C 188-195's `#elif defined STAT_STATFS2_FSIZE` arm (`fsd.f_fsize`) is dead, and C 197-207's
     `#elif defined STAT_STATFS4` arm (the SVR3 `statfs (file, &fsd, sizeof fsd, 0)` form and
     its 512-byte block size) is dead too: both macros are undefined.  */

  /* C 215-220's block is live (`STAT_STATFS2_BSIZE` is the disjunct of C 211-213 that holds).  */
  (*fsp).fsu_blocks = PROPAGATE_ALL_ONES! (fsd.f_blocks);
  (*fsp).fsu_bfree = PROPAGATE_ALL_ONES! (fsd.f_bfree);
  (*fsp).fsu_bavail = PROPAGATE_TOP_BIT! (fsd.f_bavail);
  (*fsp).fsu_bavail_top_bit_set = EXTRACT_TOP_BIT! (fsd.f_bavail) != 0;
  (*fsp).fsu_files = PROPAGATE_ALL_ONES! (fsd.f_files);
  (*fsp).fsu_ffree = PROPAGATE_ALL_ONES! (fsd.f_ffree);

  let _ = disk;  /* (void) disk;  avoid argument-unused warning */
  return 0;
}
