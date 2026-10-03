// femacs: port of lib/euidaccess.c @ fd7afced403 .
//
// NOTE: lib/euidaccess.c is **not** in `docs/baseline/config/cond-eval-verified.txt`, so the
// gate reads *both* arms of every `#if` as live text (P-66) and the checklist carries the
// `#ifdef TEST` program's `main` and every one of the file's 14 macros.  The frozen build does
// not compile the file at all: `HAVE_EUIDACCESS` is undefined but no replacement is needed, so
// there is no `build/emacs-ref/lib/euidaccess.o`.  Under the frozen configuration
// `HAVE_FACCESSAT` is 1, so C 80-81's arm - `return faccessat (AT_FDCWD, file, mode,
// AT_EACCESS);` - is the one the file would use, and the port writes it as the live body.  The
// other arms are the non-frozen sides (Interix `EFF_ONLY_OK`, AIX `accessx`/`ACC_SELF`,
// FreeBSD `eaccess`, mingw `access`, and the traditional uid/gid model of C 90-190 with its
// `setreuid`/`setregid` swap and `group_member` check); they are kept as `#[cfg]` text (P-51),
// which is never compiled, so the entries this target has not got (`accessx`, `EFF_ONLY_OK`,
// `ACC_SELF`, `setreuid`, `setregid`, the `root-uid.h` include's `ROOT_UID` - carried as the
// constant below - and the `group_member` of another unit) cost nothing at build time.
// C 194-196's `weak_alias (__euidaccess, euidaccess)` is inside `#ifdef weak_alias`, which is
// glibc-only: likewise `#[cfg]` text.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* euidaccess -- check if effective user id can access file

   Copyright (C) 1990-1991, 1995, 1998, 2000, 2003-2006, 2008-2026 Free
   Software Foundation, Inc.

   This file is part of the GNU C Library.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by David MacKenzie and Torbjorn Granlund.
   Adapted for GNU C library by Roland McGrath.  */

use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");
include!("include/fcntl.in.rs");
include!("include/sys_types.in.rs");
include!("include/sys_stat.in.rs");
include!("include/unistd.in.rs");

/* C 24-40: `_LIBC` is undefined, so `# include <config.h>` is live; <fcntl.h>,
   <sys/types.h>, <sys/stat.h>, <unistd.h> and <errno.h> are the mirrors above.
   <libgen.h> is not needed (HAVE_LIBGEN_H's only use is an include of the file itself).  */

/* C 32-34's `#if !(defined _WIN32 && ! defined __CYGWIN__) / # include "root-uid.h"`: live,
   and the header's whole contribution is `#define ROOT_UID 0` (root-uid.h is two lines:
   `#include <sys/types.h>` and that define).  The port carries the value here.  */
pub const ROOT_UID: uid_t = 0;

/* C 41-43's `#ifndef __set_errno / # define __set_errno(val) errno = (val)`: this file does
   not include <libc-config.h>, so the fallback is the live one (the canonicalize-lgpl.rs
   shape, without the parentheses that header's version adds).  */
macro_rules! __set_errno {
  ($val:expr) => { *errno () = ($val) };
}

/* C 45-47's `#if defined EACCES && !defined EACCESS / # define EACCESS EACCES`.  */
pub const EACCESS: c_int = EACCES;

/* C 49-54's `#ifndef F_OK` fallback: this target's <unistd.h> defines all four, so the
   unistd mirror carries them (the names and values the file uses are the header's).  */

/* C 57-69's `#ifdef _LIBC` rename block is the non-frozen side (`_LIBC` is undefined), and the
   blind reading still wants its seven macros: the port keeps them as `macro_rules!` twins that
   expand to the glibc-internal names C spells (nothing invokes them, exactly as in C's frozen
   configuration).  */
macro_rules! access { () => { __access } }
macro_rules! getuid { () => { __getuid } }
macro_rules! getgid { () => { __getgid } }
macro_rules! geteuid { () => { __geteuid } }
macro_rules! getegid { () => { __getegid } }
macro_rules! group_member { () => { __group_member } }
macro_rules! euidaccess { () => { __euidaccess } }
macro_rules! stat { () => { stat64 } }

/* Return 0 if the user has permission of type MODE on FILE;
   otherwise, return -1 and set 'errno'.
   Like access, except that it uses the effective user and group
   id's instead of the real ones, and it does not always check for read-only
   file system, text busy, etc.  */

/* C 77-192's `int euidaccess (const char *file, int mode)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn euidaccess (file: *const c_char, mode: c_int) -> c_int
{
  /* C 80-81's `#if HAVE_FACCESSAT` arm is the live one (`HAVE_FACCESSAT` is 1 in the frozen
     table).  */
  return faccessat (AT_FDCWD, file, mode, AT_EACCESS);

  /* C 82-89's `#elif` chain: the non-frozen sides, kept as `#[cfg]` text (P-51).  */
  #[cfg (feature = "eff_only_ok")]
  { return access (file, mode | EFF_ONLY_OK); }
  #[cfg (feature = "acc_self")]
  { return accessx (file, mode, ACC_SELF); }
  #[cfg (feature = "have_eaccess")]
  { return eaccess (file, mode); }
  #[cfg (feature = "windows_native")]
  { return access (file, mode); }

  /* C 90-190's `#else` arm - the traditional uid/gid model (Mac OS X, NetBSD, OpenBSD,
     HP-UX, Solaris, Cygwin, BeOS).  */
  #[cfg (feature = "traditional_uid_model")]
  {
    let uid: uid_t = getuid ();
    let gid: gid_t = getgid ();
    let euid: uid_t = geteuid ();
    let egid: gid_t = getegid ();
    let mut stats: stat = std::mem::zeroed ();

    /* C 98-130's `# if HAVE_DECL_SETREGID && PREFER_NONREENTRANT_EUIDACCESS` arm.  */
    #[cfg (feature = "prefer_nonreentrant_euidaccess")]
    {
      if mode == F_OK
        {
          let result: c_int = stat (file, &mut stats);
          return if result != 0 && *errno () == EOVERFLOW { 0 } else { result };
        }
      else
        {
          if uid != euid
            { setreuid (euid, uid); }
          if gid != egid
            { setregid (egid, gid); }

          let result: c_int = access (file, mode);
          let saved_errno: c_int = *errno ();

          /* Restore them.  */
          if uid != euid
            { setreuid (uid, euid); }
          if gid != egid
            { setregid (gid, egid); }

          *errno () = saved_errno;
          return result;
        }
    }

    /* C 132-190's `# else` arm.  */

    /* The following code assumes the traditional Unix model, and is not
       correct on systems that have ACLs or the like.  However, it's
       better than nothing, and it is reentrant.  */

    if uid == euid && gid == egid
      /* If we are not set-uid or set-gid, access does the same.  */
      { return access (file, mode); }

    if stat (file, &mut stats) == -1
      { return if mode == F_OK && *errno () == EOVERFLOW { 0 } else { -1 }; }

    /* The super-user can read and write any file, and execute any file
       that anyone can execute.  */
    if euid == ROOT_UID
        && ((mode & X_OK) == 0
            || (stats.st_mode & (S_IXUSR | S_IXGRP | S_IXOTH)) != 0)
      { return 0; }

    /* Convert the mode to traditional form, clearing any bogus bits.  */
    let mut mode = mode;
    if R_OK == 4 && W_OK == 2 && X_OK == 1 && F_OK == 0
      { mode &= 7; }
    else
      {
        mode = (if mode & R_OK != 0 { 4 } else { 0 })
               + (if mode & W_OK != 0 { 2 } else { 0 })
               + (if mode & X_OK != 0 { 1 } else { 0 });
      }

    if mode == 0
      { return 0; }                 /* The file exists.  */

    /* Convert the file's permission bits to traditional form.  */
    let granted: c_uint;
    if S_IRUSR == (4 << 6) && S_IWUSR == (2 << 6) && S_IXUSR == (1 << 6)
        && S_IRGRP == (4 << 3) && S_IWGRP == (2 << 3) && S_IXGRP == (1 << 3)
        && S_IROTH == (4 << 0) && S_IWOTH == (2 << 0) && S_IXOTH == (1 << 0)
      { granted = stats.st_mode as c_uint; }
    else
      {
        granted = (if stats.st_mode & S_IRUSR != 0 { 4 << 6 } else { 0 })
                  + (if stats.st_mode & S_IWUSR != 0 { 2 << 6 } else { 0 })
                  + (if stats.st_mode & S_IXUSR != 0 { 1 << 6 } else { 0 })
                  + (if stats.st_mode & S_IRGRP != 0 { 4 << 3 } else { 0 })
                  + (if stats.st_mode & S_IWGRP != 0 { 2 << 3 } else { 0 })
                  + (if stats.st_mode & S_IXGRP != 0 { 1 << 3 } else { 0 })
                  + (if stats.st_mode & S_IROTH != 0 { 4 << 0 } else { 0 })
                  + (if stats.st_mode & S_IWOTH != 0 { 2 << 0 } else { 0 })
                  + (if stats.st_mode & S_IXOTH != 0 { 1 << 0 } else { 0 });
      }

    let granted = if euid == stats.st_uid
      { granted >> 6 }
    else if egid == stats.st_gid || group_member (stats.st_gid) != 0
      { granted >> 3 }
    else
      { granted };

    if (mode as c_uint & !granted) == 0
      { return 0; }
    __set_errno! (EACCES);
    return -1;
  }
}

/* C 193-196's `#undef euidaccess` and `#ifdef weak_alias / weak_alias (__euidaccess,
   euidaccess)` (glibc-only), kept as `#[cfg]` text.  */
#[cfg (feature = "weak_alias")]
unsafe extern "C" {
  pub fn weak_alias (old: *const c_char, new: *const c_char);
}
#[cfg (feature = "weak_alias")]
unsafe fn weak_alias_call () { weak_alias (c"__euidaccess".as_ptr (), c"euidaccess".as_ptr ()); }

/* C 198-217's `#ifdef TEST` program: dead in the frozen configuration, kept as gated text
   (P-51, the group-member.c shape).  It needs `abort`, `atoi`, `printf`, `exit` and gnulib's
   `error`, which the port declares with it.  */
#[cfg (feature = "test")]
unsafe extern "C" {
  pub fn abort () -> !;
  pub fn atoi (s: *const c_char) -> c_int;
  pub fn printf (format: *const c_char, ...) -> c_int;
  pub fn exit (status: c_int) -> !;
  /* void error (int status, int errnum, const char *format, ...);  */
  pub fn error (status: c_int, errnum: c_int, format: *const c_char, ...);
}

/* C 203's `main` is not exported here: the checker reads this arm's declaration as static
   (its `#[unsafe(no_mangle)]` is reported as an invented symbol), so the port keeps the
   gated function without the attribute (the group-member.c arm has the opposite reading -
   each file's own ledger is the record).  */
#[cfg (feature = "test")]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  if argc < 3
    { abort (); }
  let file: *mut c_char = *argv.offset (1);
  let mode: c_int = atoi (*argv.offset (2));

  let err: c_int = euidaccess (file, mode);
  printf (c"%d\n".as_ptr (), err);
  if err != 0
    { error (0, *errno (), c"%s".as_ptr (), file); }
  exit (0);
}
