/* Provide gettimeofday for systems that don't have it or for which it's broken.

   Copyright (C) 2001-2003, 2005-2007, 2009-2026 Free Software Foundation, Inc.

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

/* written by Jim Meyering */

/* emacs/lib/gettimeofday.c: C 28-71's Windows arm (`#if defined _WIN32 && ! defined
   __CYGWIN__`, the `initialize` loader and its two statics, and the four macros that arm
   defines) is dead on this target, and its body needs <windows.h>'s FILETIME, HMODULE and BOOL,
   which this target has not got; the arm's tokens are recorded in
   docs/ports/macro-body-notes.tsv (the rawmemchr.c CHERI-arm treatment) rather than written out.

   C 118's `# if !defined OK_TO_USE_1S_CLOCK / # error …` arm is dead too: the frozen
   configuration defines HAVE_GETTIMEOFDAY (1), so C's `# if HAVE_GETTIMEOFDAY` arm is the live
   text and the 1-second fallback never compiles; its tokens are in the same row.

   C 22's `<sys/time.h>` is the sys_time mirror, C 24's `<time.h>` the time.in one (only the
   dead arm uses `time`) and C 84's `#undef gettimeofday` restores the system declaration for the
   call below, which the port spells with the system entry in its own module (the category.rs
   shape for a name that this unit defines).  */
use std::ffi::{c_char, c_int, c_long, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/time.in.rs");
include!("include/sys_time.rs");

/* C 84's `#undef gettimeofday`: the system entry the wrapper calls.  */
mod system {
  use super::*;

  unsafe extern "C" {
    pub fn gettimeofday (tv: *mut timeval, tz: *mut c_void) -> c_int;
  }
}

/* C 28's `#define WINDOWS_NATIVE` (the marker macro of the dead arm) and C 46-47's two statics:
   kept under the same feature, because their type is <windows.h>'s function-pointer typedef.  */
#[cfg (feature = "windows_native")]
pub const WINDOWS_NATIVE: () = ();

/* C 49-60's `initialize`: the dead arm's loader, kept as text under the same feature (it needs
   <windows.h>'s HMODULE and the arm's own typedef, which this target has not got).  */
#[cfg (feature = "windows_native")]
unsafe fn initialize ()
{
  let kernel32: HMODULE = LoadLibrary (c"kernel32.dll".as_ptr ());
  if kernel32 != std::ptr::null_mut ()
    {
      GetSystemTimePreciseAsFileTimeFunc =
	GetProcAddress (kernel32, c"GetSystemTimePreciseAsFileTime".as_ptr ())
	  as GetSystemTimePreciseAsFileTimeFuncType;
    }
  initialized = true;
}
#[cfg (feature = "windows_native")]
static mut GetSystemTimePreciseAsFileTimeFunc: *mut c_void = std::ptr::null_mut ();
#[cfg (feature = "windows_native")]
static mut initialized: c_int = 0;

/* This is a wrapper for gettimeofday.  It is used only on systems
   that lack this function, or whose implementation of this function
   causes problems.
   Work around the bug in some systems whereby gettimeofday clobbers
   the static buffer that localtime uses for its return value.  The
   gettimeofday function from Mac OS X 10.0.4 (i.e., Darwin 1.3.7) has
   this problem.  */

/* sys_time.in.h 104-105 renames the identifier (`#define gettimeofday rpl_gettimeofday`) when
   gnulib replaces the function, which is what system-dependent.m4's REPLACE_GETTIMEOFDAY
   decides; the port keeps C's name for the ledger and emits the replacement symbol the way the
   mempcpy unit does (PORTING.md 10.13).  */
#[unsafe(export_name = "rpl_gettimeofday")]
pub unsafe extern "C" fn gettimeofday (tv: *mut timeval, tz: *mut c_void) -> c_int
{
  /* C's `# if defined timeval` arm is dead: the frozen configuration does not define timeval
     (docs/baseline/config/darwin-arm64.macros.tsv has no such macro), so C's `# else` arm - the
     direct call - is the live text.  */

  let result: c_int = system::gettimeofday (tv, tz);

  return result;
}
