// femacs: port of src/vm-limit.c @ fd7afced403 .
//
// One-to-one port (PORTING.md): same statements in the same order, same names.
//
// The `#include` list mirrors vm-limit.c 19-45: config.h, <unistd.h> (for `environ`, on AIX),
// lisp.h, <sys/types.h> and - HAVE_SYS_RESOURCE_H being 1 - <sys/time.h> and
// <sys/resource.h>.  `<sys/vlimit.h>` (HAVE_SYS_VLIMIT_H) and `<malloc.h>` (HAVE_MALLOC_H)
// are undef, i.e. dead.
//
// The frozen configuration takes the `#ifdef HAVE_GETRLIMIT` branch, so the WINDOWSNT and
// MSDOS arms below are dead text under the matching cfgs (P-51); FLIPS does not name those
// two features, so what verifies them is a future flip, not this build - the ledger says so.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Functions for memory limit warnings.
   Copyright (C) 1990, 1992, 2001-2026 Free Software Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

include!("include/config.rs");
include!("../lib/include/stdio.rs");
include!("../lib/include/stdlib.rs");
include!("include/lisp.rs");
include!("../lib/include/sys_types.in.rs");
include!("../lib/include/time.in.rs");    // <sys/time.h> pulls <time.h>'s time_t
include!("../lib/include/sys_time.rs");
include!("../lib/include/sys_resource.rs");

/* vm-limit.c 30-33 (the dead `#ifdef MSDOS` arm, kept for the dead-arms build).  */
#[cfg(all(feature = "msdos", not(feature = "windowsnt")))]
unsafe extern "C" {
  /* dosfns.h declares this.  */
  pub static mut etext: c_int;
}

/* Start of data.  It is OK if this is approximate; it's used only as
   a heuristic.  */
/* DATA_START and HAVE_DATA_START are undef, so C defines this object (nonzero, so it
   lands in data rather than bss).  */
#[unsafe(no_mangle)]
pub static mut data_start: [c_char; 1] = [1];

/* HAVE_MALLOC_H and DOUG_LEA_MALLOC are undef: no <malloc.h>, and the two hooks below.  */
/* vm-limit.c 61: `#define __MALLOC_HOOK_VOLATILE volatile`; Rust spells the qualifier at
   the access site, so the macro's name lives on as this marker (GCALIGNED_MARKER's shape).  */
pub const __MALLOC_HOOK_VOLATILE: () = ();
unsafe extern "C" {
  /* From the C library.  */
  pub static mut __morecore: Option<unsafe extern "C" fn (ptrdiff_t) -> *mut c_void>;
  pub static mut __after_morecore_hook: Option<unsafe extern "C" fn ()>;
}

/* REL_ALLOC is undef, so real_morecore is check_memory_limits's local, not a global.  */

/*
  Level number of warnings already issued.
  0 -- no warnings issued.
  1 -- 75% warning already issued.
  2 -- 85% warning already issued.
  3 -- 95% warning issued; keep warning frequently.
*/
#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum warnlevel { not_warned, warned_75, warned_85, warned_95 }
use warnlevel::*;
// vm-limit.c 74: `static enum warnlevel warnlevel;` - bss, i.e. `not_warned`.
static mut warnlevel: warnlevel = warnlevel::not_warned;

/* Function to call to issue a warning;
   0 means don't issue them.  */
static mut warn_function: Option<unsafe extern "C" fn (s: *const c_char)> = None;

/* Start of data space; can be changed by calling memory_warnings.  */
static mut data_space_start: *mut c_char = std::ptr::null_mut ();

/* Number of bytes of writable memory we can expect to be able to get.  */
static mut lim_data: size_t = 0;
/* HAVE_GETRLIMIT is 1, and RLIMIT_AS is defined (5), so C's `#define RLIMIT_AS RLIMIT_DATA`
   fallback is dead; lib/include/sys_resource.rs carries the real value.  */

#[cfg(not(any(feature = "windowsnt", feature = "msdos")))]
unsafe fn get_lim_data ()
{
  /* Set LIM_DATA to the minimum of the maximum object size and the
     maximum address space.  Don't bother to check for values like
     RLIM_INFINITY since in practice they are not much less than SIZE_MAX.  */
  let mut rlimit: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
  lim_data
    = if getrlimit (RLIMIT_AS, &raw mut rlimit) == 0 && rlimit.rlim_cur <= SIZE_MAX as rlim_t
        { rlimit.rlim_cur as size_t }
        else
        { SIZE_MAX };
}

/* The WINDOWSNT and MSDOS arms are dead here; their platform headers are not mirrored
   yet, so what they need is declared in the arms themselves.  */
#[cfg(all(feature = "windowsnt", not(feature = "msdos")))]
unsafe extern "C" {
  /* w32heap.h declares this.  */
  pub static mut reserved_heap_size: size_t;
}

#[cfg(feature = "windowsnt")]
unsafe fn get_lim_data ()
{
  lim_data = reserved_heap_size;
}

#[cfg(feature = "msdos")]
unsafe extern "C" {
  /* dosfns.h declares this.  */
  pub fn dos_memory_info (totalram: *mut c_ulong, freeram: *mut c_ulong,
                          totalswap: *mut c_ulong, freeswap: *mut c_ulong);
}

#[cfg(all(feature = "msdos", not(feature = "windowsnt")))] #[unsafe(no_mangle)]
pub unsafe extern "C" fn get_lim_data ()
{
  let mut totalram: c_ulong = 0;
  let mut freeram: c_ulong = 0;
  let mut totalswap: c_ulong = 0;
  let mut freeswap: c_ulong = 0;

  dos_memory_info (&raw mut totalram, &raw mut freeram, &raw mut totalswap,
                   &raw mut freeswap);
  lim_data = freeram as size_t;
  /* Don't believe they will give us more than 0.5 GB.   */
  if lim_data > 512 * 1024 * 1024
    { lim_data = 512 * 1024 * 1024; }
}

#[cfg(feature = "msdos")] #[unsafe(no_mangle)]
pub unsafe extern "C" fn ret_lim_data () -> c_ulong
{
  get_lim_data ();
  return lim_data as c_ulong;
}

/* Verify amount of memory available, complaining if we're near the end. */

unsafe extern "C" fn check_memory_limits ()
{
  let real_morecore: Option<unsafe extern "C" fn (ptrdiff_t) -> *mut c_void> = None;

  let mut cp: *mut c_char;
  let mut five_percent: size_t;
  let mut data_size: size_t;
  let new_warnlevel: warnlevel;

  if lim_data == 0
    { get_lim_data (); }
  five_percent = lim_data / 20;

  /* Find current end of memory and issue warning if getting near max */
  cp = (if real_morecore.is_some () { real_morecore } else { __morecore }).unwrap () (0)
       as *mut c_char;
  data_size = cp.offset_from (data_space_start) as size_t;

  if warn_function.is_none ()
    { return; }

  /* What level of warning does current memory usage demand?  */
  new_warnlevel
    = if data_size > five_percent * 19 { warned_95 }
      else if data_size > five_percent * 17 { warned_85 }
      else if data_size > five_percent * 15 { warned_75 }
      else { not_warned };

  /* If we have gone up a level, give the appropriate warning.  */
  if new_warnlevel > warnlevel || new_warnlevel == warned_95
    {
      warnlevel = new_warnlevel;
      let warn_diagnostic: [*const c_char; 3] =
	[
	  c"Warning: past 75% of memory limit".as_ptr (),
	  c"Warning: past 85% of memory limit".as_ptr (),
	  c"Warning: past 95% of memory limit".as_ptr ()
	];
      warn_function.unwrap () (warn_diagnostic[warnlevel as usize - 1]);
    }
  /* Handle going down in usage levels, with some hysteresis.  */
  else
    {
      /* If we go down below 70% full, issue another 75% warning
	 when we go up again.  */
      if data_size < five_percent * 14
	{ warnlevel = not_warned; }
      /* If we go down below 80% full, issue another 85% warning
	 when we go up again.  */
      else if warnlevel > warned_75 && data_size < five_percent * 16
	{ warnlevel = warned_75; }
      /* If we go down below 90% full, issue another 95% warning
	 when we go up again.  */
      else if warnlevel > warned_85 && data_size < five_percent * 18
	{ warnlevel = warned_85; }
    }
}

/* Enable memory usage warnings.
   START says where the end of pure storage is.
   WARNFUN specifies the function to call to issue a warning.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memory_warnings (start: *mut c_void,
                                          warnfun: Option<unsafe extern "C" fn (s: *const c_char)>)
{
  data_space_start = if ! start.is_null () { start as *mut c_char }
                     else { data_start.as_mut_ptr () };

  warn_function = warnfun;
  __after_morecore_hook = Some (check_memory_limits);

  /* Force data limit to be recalculated on each run.  */
  lim_data = 0;
}
