/* Internals of mktime and related functions
   Copyright 2016-2026 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Contributed by Paul Eggert <eggert@cs.ucla.edu>.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

// femacs: port of lib/mktime-internal.h @ fd7afced403 (gnulib).
//
// Header mirror (PORTING.md 0.1).  `tools/check-cond-eval.py lib/mktime-internal.h`
// reports 4 disagreements (the two `@…@` conditions it cannot decide), so this file is
// written from the *frozen* configuration's text: `_LIBC` is undefined, `TIME_T_IS_SIGNED`
// is 1 and `__TIMESIZE` is 64, so `mktime_offset_t` is C's `time_t` arm and both
// `#if ! (defined _LIBC && __TIMESIZE != 64)` and `#ifndef _LIBC` renames are live.
//
// The renames that produce a *symbol* (`__mktime64` → `mktime`, `__timegm64` → `timegm`,
// `__mktime_internal` → `mktime_internal`) are expressed by the port on the definition
// (`#[unsafe(export_name = …)]` in mktime.rs / timegm.rs, PORTING.md 10.13①) or by a type
// alias (`__time64_t` → `time_t`); the three names whose C form has therefore no Rust
// macro are the `excluded-macros.txt` rows for this header.  The renames a *call site*
// spells keep a `macro_rules!` here, so a unit that writes C's `__localtime64_r (…)` or
// `__mktime_internal (…)` gets the frozen configuration's target function.

include!("time.in.rs");

/* mktime-internal.h 27-41: the three arms of `mktime_offset_t`; the live one is the
   `#elif defined TIME_T_IS_SIGNED` arm (`typedef time_t mktime_offset_t;`).  */
pub type mktime_offset_t = time_t;

/* mktime-internal.h 47-54's live arm: `# define __time64_t time_t` (and the four
   `__*64_*` renames that arm performs; the three that name a *definition* are recorded in
   docs/ports/excluded-macros.txt, and `__mktime64`/`__timegm64` are not used by any ported
   unit yet).  */
pub type __time64_t = time_t;

/* mktime-internal.h 20-25: the `#ifndef _LIBC` arm.  `__tzset_unlocked` expands to
   `tzset ()`, and that `tzset` is the *including* unit's macro when it defines one
   (mktime.c's `# define tzset() my_tzset ()`), which is why the body below is `tzset! ()`.  */
macro_rules! __libc_lock_lock { ($lock:expr) => { () }; }
macro_rules! __libc_lock_unlock { ($lock:expr) => { () }; }
macro_rules! __tzset_unlocked { () => { tzset! () }; }

/* mktime-internal.h 50-51 / 67-70: the gmtime/localtime rename chain, in the header's own
   order (`__gmtime64_r` → `__gmtime_r` → `gmtime_r`).  */
macro_rules! __gmtime64_r { ($t:expr, $tp:expr) => { __gmtime_r! ($t, $tp) }; }
macro_rules! __localtime64_r { ($t:expr, $tp:expr) => { __localtime_r! ($t, $tp) }; }
macro_rules! __gmtime_r { ($t:expr, $tp:expr) => { gmtime_r ($t, $tp) }; }
macro_rules! __localtime_r { ($t:expr, $tp:expr) => { localtime_r ($t, $tp) }; }

/* mktime-internal.h 72: `# define __mktime_internal mktime_internal`.  */
macro_rules! __mktime_internal {
  ($tp:expr, $local:expr, $offset:expr) => { mktime_internal ($tp, $local, $offset) };
}

/* mktime-internal.h 76-83: the declaration, under the name the renames above give it.  */
unsafe extern "C" {
  /* __time64_t __mktime_internal (struct tm *tp, bool local, mktime_offset_t *offset);  */
  pub fn mktime_internal (tp: *mut tm, local: bool, offset: *mut mktime_offset_t)
    -> __time64_t;
}
