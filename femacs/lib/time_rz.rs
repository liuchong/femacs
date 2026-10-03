/* Time zone functions such as tzalloc and localtime_rz

   Copyright 2015-2026 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Paul Eggert.  */

/* Although this module is not thread-safe, any races should be fairly
   rare and reasonably benign.  For complete thread safety, use a C
   library with a working timezone_t type, so that this module is not
   needed.  */

// femacs: port of lib/time_rz.c @ fd7afced403 (gnulib).
//
// The frozen configuration resolves all 6 of this file's conditions
// (`tools/check-cond-eval.py lib/time_rz.c`: 0 unknown, 0 disagreements), so only the live
// text is written: `HAVE_TZALLOC` is undefined, which makes C 65-322's `#else` arm the one
// this target compiles and C 32-63's `#if HAVE_TZALLOC` arms (the two Android wrappers and
// the libc `tzfree`) dead text.  The six symbols this unit emits are the plain names the
// reference object carries (`nm -g build/emacs-ref/lib/time_rz.o`: `tzalloc`, `tzfree`,
// `set_tz`, `revert_tz`, `localtime_rz`, `mktime_z` — none of them renamed), plus the
// static helpers `extend_abbrs`, `save_abbr` and `change_env`.
//
// Two renames the *frozen build's generated headers* perform are spelled as macros here,
// because C's text uses the renamed call:
//   * `src/conf_post.h` 178-184 (`#define getenv_TZ emacs_getenv_TZ`,
//     `#define setenv_TZ emacs_setenv_TZ`) — which is why C 182-196's two `static`
//     definitions are dead; the reference object calls `emacs_getenv_TZ`/`emacs_setenv_TZ`.
//   * the generated `<time.h>`'s `#define mktime rpl_mktime` / `#define timegm rpl_timegm`
//     (REPLACE_MKTIME and REPLACE_TIMEGM are 1), whose replacements this port defines in
//     mktime.rs / timegm.rs; the reference object calls `rpl_mktime`/`rpl_timegm`.
//   `free` is renamed the same way (`#define free rpl_free`), and the reference object
//   shows `U _rpl_free`; the declaration is written here rather than in the stdlib mirror
//   (the lchmod.rs local-declaration shape).
//
// Deviations (docs/ports/time_rz.md): the flexible array member `struct tm_zone.abbrs` is
// reached through its base address (the lisp.rs `fam_base` idiom, PORTING.md 1.3) because a
// Rust index of the mirror's `[c_char; 0]` field is rejected outright; `struct tm tm_1;` in
// `mktime_z` is zero-initialized (PORTING.md 1.5); the unit's definitions of `set_tz` and
// `revert_tz` live in `mod tz_impl` because the time-internal mirror declares both names
// (E0428, PORTING.md 0.1-10).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_schar, c_short, c_uchar, c_uint, c_ulong,
                c_ulonglong, c_ushort, c_void};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");
include!("include/idx.rs");
include!("include/stdlib.rs");
include!("include/string.rs");
include!("include/time.in.rs");
include!("include/time-internal.rs");
include!("include/flexmember.rs");

/* C 25-30's `<config.h>`, `<time.h>` and `<errno.h>` are the mirrors above; C 67-73's
   `<stddef.h>` (the target's own header), `<stdlib.h>` and `<string.h>` are the stdlib and
   string mirrors plus the two unit-local declarations below, and `flexmember.h`/`idx.h`/
   `time-internal.h` are their mirrors.  */

/* C 68's `<stdlib.h>`: the generated header renames `free` (the reference object's
   `U _rpl_free`), while `malloc` is not renamed (the stdlib mirror's `malloc`).  */
unsafe extern "C" {
  /* void free (void *ptr); -- as `rpl_free`.  */
  pub fn rpl_free (ptr: *mut c_void);
}

/* src/conf_post.h 178-184: "Tell time_rz.c to use Emacs's getter and setter for TZ."  The
   macros are what make C 182-196's two `static` definitions dead, so the port keeps C's
   call spelling and calls the src/ entries.  */
unsafe extern "C" {
  /* char *emacs_getenv_TZ (void);  */
  pub fn emacs_getenv_TZ () -> *mut c_char;
  /* int emacs_setenv_TZ (char const *tz);  */
  pub fn emacs_setenv_TZ (tz: *const c_char) -> c_int;
}
/* conf_post.h 180-181.  */
macro_rules! getenv_TZ { () => { emacs_getenv_TZ () }; }
macro_rules! setenv_TZ { ($tz:expr) => { emacs_setenv_TZ ($tz) }; }

/* The generated <time.h> 841-842 and 1144-1145: `# define mktime rpl_mktime` and
   `# define timegm rpl_timegm`, the two replacements this port defines in mktime.rs and
   timegm.rs.  */
macro_rules! mktime { ($tp:expr) => { rpl_mktime ($tp) }; }
use time_decls::rpl_timegm;

macro_rules! timegm { ($tp:expr) => { rpl_timegm ($tp) }; }

/* C 76-77 and 79-82's two `enum` constants.  `DEFAULT_MXFAST` is C's
   `64 * sizeof (size_t) / 4` and `ABBR_SIZE_MIN` subtracts
   `offsetof (struct tm_zone, abbrs)`, which the port reads with `offset_of!`.  */
pub const DEFAULT_MXFAST: usize = 64 * std::mem::size_of::<usize> () / 4;
pub const ABBR_SIZE_MIN: usize = DEFAULT_MXFAST - std::mem::offset_of! (tm_zone, abbrs);
/* Time_rz.c defines the entry points C's own header (time-internal.h) declares, so the
   unit's `set_tz`/`revert_tz` definitions would be an E0428 clash with the mirror's
   declarations in the same module (PORTING.md 0.1-10).  The definitions therefore live in
   `mod tz_impl`, whose `use super::*` gives it the mirror's types, the macros above and the
   crate-root constants; the module's own names shadow the glob-imported declarations
   (§0.1-10's shadowing), while each `#[unsafe(no_mangle)]` definition still emits its C
   symbol.  The C file's function order is kept inside the module.  */
mod tz_impl
{
  use super::*;

  /* time.in.h 494's `typedef struct tm_zone *timezone_t;`: the time.h mirror spells the
     pointer as an opaque `*mut c_void`, because a mirror that does not include
     lib/time-internal.h cannot name the forward-declared struct (PORTING.md 0.1-2).  This
     unit *does* include it, so inside this module — where the definitions C's own header
     declares live — `timezone_t` is the complete pointer: the alias below is an item, and an
     item shadows the glob-imported alias above (§0.1-10's shadowing, the same rule that lets
     `DEFUN!` definitions coexist with `globals.rs`'s prototypes).  Every body then keeps
     C's spelling, and so does `local_tz!`, whose `timezone_t` resolves here at the expansion
     site.  */
  type timezone_t = *mut tm_zone;

  /* The flexible array member `struct tm_zone.abbrs` is the mirror's `[c_char; 0]`: a Rust
     index of it is rejected outright (a zero-length array has no elements) and
     `as_mut_ptr ()` would retag a zero-size reference, so the port reaches the member
     through its base address — the lisp.rs `fam_base`/`FAM_ELEM!` idiom (PORTING.md 1.3).
     C's `tz->abbrs` (the array decaying to `char *`) is `abbrs (tz)` and `tz->abbrs[i]`
     is `*abbrs (tz).add (i)`.  */
  #[inline(always)]
  unsafe fn abbrs (tz: timezone_t) -> *mut c_char
  {
    return std::ptr::addr_of_mut! ((*tz).abbrs) as *mut c_char;
  }

  /* C 84-92.  */
  /* Copy to ABBRS the abbreviation at ABBR with size ABBR_SIZE (this
     includes its trailing null byte).  Append an extra null byte to
     mark the end of ABBRS.  */
  unsafe fn extend_abbrs (abbrs: *mut c_char, abbr: *const c_char, abbr_size: usize)
  {
    memcpy (abbrs as *mut c_void, abbr as *const c_void, abbr_size);
    *abbrs.add (abbr_size) = b'\0' as c_char;
  }

  /* C 94-111.  */
  /* Return a newly allocated time zone for NAME, or NULL on failure.
     A null NAME stands for wall clock time (which is like unset TZ).  */
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn tzalloc (name: *const c_char) -> timezone_t
  {
    let name_size: usize = if ! name.is_null () { strlen (name) + 1 } else { 0 };
    let abbr_size: usize = if name_size < ABBR_SIZE_MIN
                           { ABBR_SIZE_MIN } else { name_size + 1 };
    let tz: timezone_t = malloc (FLEXSIZEOF! (tm_zone, abbrs, abbr_size)) as timezone_t;
    if ! tz.is_null ()
      {
        (*tz).next = std::ptr::null_mut ();
        (*tz).tz_is_set = (! name.is_null ()) as c_char;
        *abbrs (tz) = b'\0' as c_char;
        if ! name.is_null ()
          { extend_abbrs (abbrs (tz), name, name_size); }
      }
    return tz;
  }

  /* C 113-164.  */
  /* If HAVE_STRUCT_TM_TM_ZONE, save into TZ any nontrivial time zone
     abbreviation used by TM, and update *TM to contain the saved abbreviation.
     Return true if successful, false (setting errno) otherwise.  */
  unsafe fn save_abbr (mut tz: timezone_t, tm: *mut tm) -> bool
  {
    /* C 119's `#if HAVE_STRUCT_TM_TM_ZONE` is live (the frozen `struct tm` has `tm_zone`);
       the `#else` arm is dead.  */
    let zone: *const c_char = (*tm).tm_zone;
    static mt: [c_char; 1] = [0];
    let mut zone_copy: *mut c_char = mt.as_ptr () as *mut c_char;

    /* No need to replace null zones, or zones within the struct tm.  */
    if zone.is_null ()
       || ((tm as *const c_char) <= zone && zone < (tm.add (1) as *const c_char))
      { return true; }

    if *zone != 0
      {
        zone_copy = abbrs (tz);

        while ! streq (zone_copy, zone)
          {
            if ! (*zone_copy != 0
                  || (zone_copy == abbrs (tz) && (*tz).tz_is_set != 0))
              {
                let zone_size: idx_t = (strlen (zone) + 1) as idx_t;
                if zone_size < ((abbrs (tz) as usize).wrapping_add (ABBR_SIZE_MIN)
                                .wrapping_sub (zone_copy as usize)) as idx_t
                  { extend_abbrs (zone_copy, zone, zone_size as usize); }
                else
                  {
                    (*tz).next = tzalloc (zone);
                    tz = (*tz).next;
                    if tz.is_null ()
                      { return false; }
                    (*tz).tz_is_set = 0;
                    zone_copy = abbrs (tz);
                  }
                break;
              }

            zone_copy = strnul! (zone_copy).add (1);
            if *zone_copy == 0 && ! (*tz).next.is_null ()
              {
                tz = (*tz).next;
                zone_copy = abbrs (tz);
              }
          }
      }

    /* Replace the zone name so that its lifetime matches that of TZ.  */
    (*tm).tm_zone = zone_copy;
    return true;
  }

  /* C 166-177.  */
  /* Free a time zone.  */
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn tzfree (mut tz: timezone_t)
  {
    if tz != local_tz! ()
      {
        while ! tz.is_null ()
          {
            let next: timezone_t = (*tz).next;
            rpl_free (tz as *mut c_void);
            tz = next;
          }
      }
  }

  /* C 198-207.  */
  /* Change the environment to match the specified timezone_t value.
     Return true if successful, false (setting errno) otherwise.  */
  unsafe fn change_env (tz: timezone_t) -> bool
  {
    if setenv_TZ! (if (*tz).tz_is_set != 0
                   { abbrs (tz) as *const c_char } else { std::ptr::null () }) != 0
      { return false; }
    tzset ();
    return true;
  }

  /* C 209-233.  */
  /* Temporarily set the time zone to TZ, which must not be null.
     Return LOCAL_TZ if the time zone setting is already correct.
     Otherwise return a newly allocated time zone representing the old
     setting, or NULL (setting errno) on failure.  */
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn set_tz (tz: timezone_t) -> timezone_t
  {
    let env_tz: *mut c_char = getenv_TZ! ();
    if if ! env_tz.is_null ()
         { (*tz).tz_is_set != 0 && streq (abbrs (tz), env_tz) }
       else
         { ! ((*tz).tz_is_set != 0) }
      { return local_tz! (); }
    else
      {
        let old_tz: timezone_t = tzalloc (env_tz);
        if old_tz.is_null ()
          { return old_tz; }
        if ! change_env (tz)
          {
            tzfree (old_tz);
            return std::ptr::null_mut ();
          }
        return old_tz;
      }
  }

  /* C 235-253.  */
  /* Restore an old setting returned by set_tz.  It must not be null.
     Return true (preserving errno) if successful, false (setting errno)
     otherwise.  */
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn revert_tz (tz: timezone_t) -> bool
  {
    if tz == local_tz! ()
      { return true; }
    else
      {
        let mut saved_errno: c_int = *errno ();
        let ok: bool = change_env (tz);
        if ! ok
          { saved_errno = *errno (); }
        tzfree (tz);
        *errno () = saved_errno;
        return ok;
      }
  }

  /* C 255-287.  */
  /* Use time zone TZ to compute localtime_r (T, TM).  */
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn localtime_rz (tz: timezone_t, t: *const time_t,
                                         tm: *mut tm) -> *mut tm
  {
    /* C 259-272's `#ifdef HAVE_LOCALTIME_INFLOOP_BUG` arm is dead (the macro is undefined
       on this target).  */

    if tz.is_null ()
      { return gmtime_r (t, tm); }
    else
      {
        let old_tz: timezone_t = set_tz (tz);
        if ! old_tz.is_null ()
          {
            let abbr_saved: bool = ! localtime_r (t, tm).is_null () && save_abbr (tz, tm);
            if revert_tz (old_tz) && abbr_saved
              { return tm; }
          }
        return std::ptr::null_mut ();
      }
  }

  /* C 289-320.  */
  /* Use time zone TZ to compute mktime (TM).  */
  #[unsafe(no_mangle)]
  pub unsafe extern "C" fn mktime_z (tz: timezone_t, tm: *mut tm) -> time_t
  {
    if tz.is_null ()
      { return timegm! (tm); }
    else
      {
        let old_tz: timezone_t = set_tz (tz);
        if ! old_tz.is_null ()
          {
            let mut tm_1: tm = std::mem::zeroed ();   /* the C declaration, PORTING.md 1.5  */
            tm_1.tm_sec = (*tm).tm_sec;
            tm_1.tm_min = (*tm).tm_min;
            tm_1.tm_hour = (*tm).tm_hour;
            tm_1.tm_mday = (*tm).tm_mday;
            tm_1.tm_mon = (*tm).tm_mon;
            tm_1.tm_year = (*tm).tm_year;
            tm_1.tm_yday = -1;
            tm_1.tm_isdst = (*tm).tm_isdst;
            let t: time_t = mktime! (&mut tm_1);
            let mut ok: bool = 0 <= tm_1.tm_yday;
            ok = ok && save_abbr (tz, &mut tm_1);
            if revert_tz (old_tz) && ok
              {
                *tm = tm_1;
                return t;
              }
          }
        return -1;
      }
  }
}
