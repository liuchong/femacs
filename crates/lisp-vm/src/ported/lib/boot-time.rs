// femacs: port of lib/boot-time.c @ fd7afced403.
//
// The frozen darwin-arm64 configuration (docs/baseline/config/darwin-arm64.macros.tsv) has
// HAVE_UTMPX_H 1, HAVE_UTMP_H 1, HAVE_STRUCT_UTMPX_UT_NAME undef, HAVE_STRUCT_UTMPX_UT_TYPE 1,
// HAVE_UTMPXNAME 1, HAVE_DECL_ENDUTENT 0, HAVE_SYS_SYSCTL_H 1, HAVE_SYS_PARAM_H 1,
// HAVE_SYSCTL 1, CTL_KERN 1, KERN_BOOTTIME 21 and HAVE_OS_H undef, and the compiler defines
// none of __linux__, __ANDROID__, __minix, __OpenBSD__, __HAIKU__, __CYGWIN__ or _WIN32.  So
// C 81's `#if defined __linux__ || HAVE_UTMPX_H || HAVE_UTMP_H || defined __CYGWIN__ ||
// defined _WIN32` is live, C 92's `#if defined UTMP_NAME_FUNCTION` arm is the live half of the
// utmp scan (C 178's `#else` fopen arm is dead), C 88's `ut_type` arm is live (C 214's `#else`
// is dead), C 230's sysctl fallback is live, and the other platform arms are dead.
//
// Every dead arm of this file is kept as text under the feature that names the non-frozen
// side (PORTING.md P-51, the status 3.9 policy for lib/*.c), so `--check` still sees the
// tokens of C's text and the ledger keeps the original: `linux_or_android` (C 31-34's two
// includes, recorded rather than carried: no mirror exists for <sys/sysinfo.h>, and <time.h>
// comes in through "boot-time.h"), `utmp_ut_name` (C 70's `((UT)->ut_name)` arm of UT_USER),
// `win32` (C 47-51's WIN32_LEAN_AND_MEAN and its two includes, recorded), `linux_no_android`
// (C 103/124/138/207/216), `android` (C 158), `minix` (C 129/166), `no_utmp_name_function`
// (C 178-205), `no_utmp_x_ut_type` (C 214-228), `openbsd` (C 223), `haiku` (C 239), `os_h`
// (C 246), `cygwin_win32` (C 253) with `no_cygwin` (C 258) and `utmp_h` (C 117-119's
// `ut_name`/`ut_time` half).  These features are never set: `xtask dead-arms` can only flip the
// keys config.rs carries and these are compiler/platform predicates (the status 3.9 note).
//
// "readutmp.h" (C 54) became a mirror in status 3.198, so the utmp entries and UTMP_FILE now
// come from `include/readutmp.rs`; the one name that stays here is the *type* spelling
// `UTMP_STRUCT_NAME` (the mirror explains why: C spells it as a macro used in a type position,
// and the port's type alias and the mirror's macro do not collide).  "unlocked-io.h" (C 58)
// also has a mirror now, but contributes no live name (measured: the frozen
// `HAVE_DECL_FOPEN_UNLOCKED` is 0, so its `#define fopen_unlocked fopen` arm is the one taken
// and the calls below stay plain `fopen`).  "boot-time-aux.h" (C 61) **became a mirror** in status 3.104
// (`lib/include/boot-time-aux.rs`), so the one arm of it this configuration compiles -
// its sysctl fallback - and the dead platform arms beside it are included rather than
// transcribed here; all three headers are included by this file alone (measured: `grep -l`
// over emacs/lib and emacs/src).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::sys_common::*;

/* Written by Bruno Haible <bruno@clisp.org>.  */

use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_void};

// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/stdio.rs imported via common
// [U1] include/string.in.rs imported via common
// [U1] include/sys_types.in.rs imported via common
// [U1] include/time.in.rs imported via common

/* <time.h>: `CLOCK_REALTIME`.  The `<time.h>` mirror carries the *type* (`struct timespec`) and
   this name is left to the unit, the way `gettime.rs` 18 and `tempname.rs` 97 already declare it;
   the mirror that uses it (`include/boot-time-aux.rs` 224, C `boot-time-aux.h` 184) reaches it
   from the including unit's scope.  Measured: the Android sweep failed on this name alone
   (status 4.708), and bionic defines it as 0 (`<time.h>`), the same value the frozen target's
   `<time.h>` has.  */
pub const CLOCK_REALTIME: c_int = 0;

/* <time.h>: the Linux/Android arm of `include/boot-time-aux.rs` calls `clock_gettime`, which the
   unit declares the way `gettime.rs` 22 does (the `<time.h>` mirror carries the type, not the
   call).  Measured: the Android sweep named it right after the gate above was fixed (4.708).  */
unsafe extern "C" {
  pub fn clock_gettime (clock_id: c_int, tp: *mut timespec) -> c_int;
}
// [U1] include/sys_time.rs imported via common
// [U1] include/utmpx.rs imported via common
// [U1] include/readutmp.rs imported via common
// [U1] include/sys_sysctl.rs imported via common
// [U1] include/boot-time.rs imported via common
// [U1] include/boot-time-aux.rs imported via common

/* C 24's `<stdcountof.h>`, C 25's `<stddef.h>`, C 29's `<sys/stat.h>`, C 53's `idx.h` and
   C 55's `stat-time.h` are includes whose names this configuration's live text does not name
   (`countof`, `idx_t` and `get_stat_mtime` are used by boot-time-aux.h's dead arms only), so
   no mirror is included for them; C 36-39's `<sys/param.h>` and C 58's `unlocked-io.h` have no
   mirror and contribute no live name (`fopen` is not renamed on this target: the frozen
   `HAVE_DECL_FOPEN_UNLOCKED` is 0, so unlocked-io.h's `#define fopen_unlocked fopen` arm is the
   one taken and the calls below stay plain `fopen`), the filelock.md `<pwd.h>` note.  C 43-45's
   `<OS.h>` and C 47-51's `<windows.h>`/`<sys/time.h>` are on dead arms and have no mirror
   either.  */

/* C 63-64: "The following macros describe the 'struct UTMP_STRUCT_NAME', *not* 'struct
   gl_utmp'."  C 65's `#undef UT_USER` has no Rust counterpart: the mirror's UT_USER is not
   included here, this unit defines it.  */

/* C 53's `#include "idx.h"` and C 55's `stat-time.h` are not the only headers the dead arms
   would need; the names their live arms took from readutmp.h are these (frozen arm of
   readutmp.h 170-178, measured with clang -dM -E on the frozen configuration):
   UTMP_STRUCT_NAME utmpx, SET_UTMP_ENT setutxent, GET_UTMP_ENT getutxent,
   END_UTMP_ENT endutxent, UTMP_NAME_FUNCTION utmpxname and UTMP_FILE "/var/run/utmpx"
   (readutmp.h 235-246 renames it to UTMPX_FILE = _PATH_UTMPX).  C spells the struct name as a
   macro used in a *type* position, which no Rust macro can be, so the port keeps this one type
   alias; the other five names come from the mirror.  */
pub type UTMP_STRUCT_NAME = utmpx;

/* C 68-73: the accessor macro for the member named ut_user or ut_name.  The frozen
   configuration leaves HAVE_STRUCT_UTMPX_UT_NAME undefined, so C 72's `ut_user` arm is live
   and C 70's `ut_name` arm is kept as text (P-51; dead).  */
#[cfg (feature = "utmp_ut_name")]
macro_rules! UT_USER { ($ut:expr) => { (*$ut).ut_name } }
#[cfg (not (feature = "utmp_ut_name"))]
macro_rules! UT_USER { ($ut:expr) => { (*$ut).ut_user } }

/* C 47-51's `#if defined _WIN32 && ! defined __CYGWIN__` arm, kept as text (P-51; dead).  Its
   two includes are recorded (no mirror for `<windows.h>`; `<sys/time.h>` is already in
   scope); the marker macro is the arm's whole live content, and the file's Windows code is
   the `cygwin_win32` arm of `get_boot_time_uncached` below.  */
#[cfg (feature = "win32")]
macro_rules! WIN32_LEAN_AND_MEAN { () => {}; }

/* C 75-79's `#if !HAVE_UTMPX_H && HAVE_UTMP_H && defined UTMP_NAME_FUNCTION` arm is dead
   (HAVE_UTMPX_H is 1 on both delivered targets), so its `void endutent (void);` declaration is
   not transcribed.  The *loop* below carries that same condition: ungated it compiled on
   Android, whose configuration also has `HAVE_UTMPX_H` 1 - C compiles no part of it - and the
   unit imported `setutxent`/`getutxent`/`endutxent`/`utmpxname`, four names bionic has no
   implementation for at all (status 4.750, docs/baseline/android-gaps.tsv).  */

/* C 61's `#include "boot-time-aux.h"` is the mirror `include/boot-time-aux.rs`: it carries
   the two <sys/sysctl.h> rows (`CTL_KERN` 1, `KERN_BOOTTIME` 21) and the one arm this
   configuration compiles (boot-time-aux.h 230-256, the same sysctl gate as C 230-232
   below), plus the header's dead platform arms under their own features.  */

/* C 81's `#if ...` is live (HAVE_UTMPX_H is 1), so C 83-296 is transcribed; C 298-306's
   `#else` arm (`get_boot_time` returning -1) is dead and is recorded in
   docs/ports/boot-time.md instead of being carried (P-08).  */

/* C 83-84: `static int get_boot_time_uncached (struct timespec *p_boot_time)`.  */
unsafe fn get_boot_time_uncached (p_boot_time: *mut timespec) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let mut found_boot_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };   /* C 86: `= {0}` */

  /* C 88's `#if (HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_TYPE : HAVE_STRUCT_UTMP_UT_TYPE)` is
     live; C 214-228's `#else` arm is kept as text below.  */

  /* Try to find the boot time in the /var/run/utmp file.  */

  /* C 92's `#if defined UTMP_NAME_FUNCTION ... #else` (C 178) closes at C 205, inside C 75's
     arm: `endutent`'s twin above carries the same condition, and it is the condition that
     decides this loop, not `UTMP_NAME_FUNCTION` alone.  */
  #[cfg (all (feature = "not_have_utmpx_h", not (feature = "not_have_utmp_h")))]
  {
    /* Ignore the return value for now.
       Solaris' utmpname returns 1 upon success -- which is contrary
       to what the GNU libc version does.  In addition, older GNU libc
       versions are actually void.   */
    static utmp_file: [u8; 15] = *UTMP_FILE;
    UTMP_NAME_FUNCTION! (utmp_file.as_ptr () as *const c_char);

    SET_UTMP_ENT! ();

    #[cfg (any (feature = "linux_no_android", feature = "minix"))]
    /* Timestamp of the "runlevel" entry, if any.  */
    let mut runlevel_ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };

    /* C 108: `void const *entry;` - C leaves it uninitialized and the loop condition is the
       first write (PORTING.md 1.5).  */
    let mut entry: *const c_void = std::ptr::null ();

    while { entry = GET_UTMP_ENT! () as *const c_void; !entry.is_null () }
      {
        let ut: *const UTMP_STRUCT_NAME = entry as *const UTMP_STRUCT_NAME;

        /* C 115-119's `#if (HAVE_UTMPX_H ? 1 : HAVE_STRUCT_UTMP_UT_TV)`: the utmpx arm is
           live, the utmp arm is kept as text (P-51; dead).  */
        #[cfg (not (feature = "utmp_h"))]
        let ts: timespec =
          timespec { tv_sec: (*ut).ut_tv.tv_sec,
                     tv_nsec: ((*ut).ut_tv.tv_usec * 1000) as c_long };
        #[cfg (feature = "utmp_h")]
        let ts: timespec = timespec { tv_sec: (*ut).ut_time, tv_nsec: 0 };

        if (*ut).ut_type == BOOT_TIME
          { found_boot_time = ts; }

        #[cfg (feature = "linux_no_android")]
        if memeq (UT_USER! (ut).as_ptr () as *const c_void,
                  c"runlevel".as_ptr () as *const c_void,
                  strlen (c"runlevel".as_ptr ()) + 1)
            && memeq ((*ut).ut_line.as_ptr () as *const c_void,
                      c"~".as_ptr () as *const c_void, strlen (c"~".as_ptr ()) + 1)
          { runlevel_ts = ts; }
        #[cfg (feature = "minix")]
        if UT_USER! (ut)[0] == 0
            && memeq ((*ut).ut_line.as_ptr () as *const c_void,
                      c"run-level ".as_ptr () as *const c_void,
                      strlen (c"run-level ".as_ptr ()))
          { runlevel_ts = ts; }
      }

    END_UTMP_ENT! ();

    /* On Raspbian, which runs on hardware without a real-time clock, during boot,
         1. the clock gets set to 1970-01-01 00:00:00,
         2. an entry gets written into /var/run/utmp, with ut_type = BOOT_TIME,
            ut_user = "reboot", ut_line = "~", time = 1970-01-01 00:00:05 or so,
         3. the clock gets set to a correct value through NTP,
         4. an entry gets written into /var/run/utmp, with
            ut_user = "runlevel", ut_line = "~", time = correct value.
       In this case, get the time from the "runlevel" entry.  */

    /* Workaround for Raspbian:  */
    #[cfg (feature = "linux_no_android")]
    if found_boot_time.tv_sec <= 60 && runlevel_ts.tv_sec != 0
      { found_boot_time = runlevel_ts; }
    #[cfg (feature = "linux_no_android")]
    if found_boot_time.tv_sec == 0
      {
        /* Workaround for Alpine Linux:  */
        get_linux_boot_time_fallback (&mut found_boot_time);
      }

    #[cfg (feature = "have_android")]
    if found_boot_time.tv_sec == 0
      {
        /* Workaround for Android:  */
        get_android_boot_time (&mut found_boot_time);
      }

    /* On Minix, during boot,
         1. an entry gets written into /var/run/utmp, with ut_type = BOOT_TIME,
            ut_user = "", ut_line = "system boot", time = 1970-01-01 00:00:00,
         2. an entry gets written into /var/run/utmp, with
            ut_user = "", ut_line = "run-level m", time = correct value.
       In this case, copy the time from the "run-level m" entry to the
       "system boot" entry.  */
    #[cfg (feature = "minix")]
    if found_boot_time.tv_sec <= 60 && runlevel_ts.tv_sec != 0
      { found_boot_time = runlevel_ts; }
  }

  /* C 178's `#else` arm (the HP-UX, Haiku half of that branch), kept as text (P-51; dead:
     UTMP_NAME_FUNCTION is defined on this target).  Its `FILE`/`fread`/`fclose` come from
     include/stdio.rs; `fopen` is not in that mirror yet, so this arm names it the way the
     binary-io.rs dead arm names `_setmode` - the arm is not compiled.  */
  #[cfg (feature = "no_utmp_name_function")]
  {
    let mut f: *mut FILE = fopen (UTMP_FILE.as_ptr () as *const c_char, c"re".as_ptr ());

    if !f.is_null ()
      {
        loop
          {
            let mut ut: UTMP_STRUCT_NAME = std::mem::zeroed ();

            if fread (&mut ut as *mut UTMP_STRUCT_NAME as *mut c_void,
                      std::mem::size_of::<UTMP_STRUCT_NAME> (), 1, f) == 0
              { break; }

            #[cfg (not (feature = "utmp_h"))]
            let ts: timespec = timespec { tv_sec: ut.ut_tv.tv_sec,
                                          tv_nsec: (ut.ut_tv.tv_usec * 1000) as c_long };
            #[cfg (feature = "utmp_h")]
            let ts: timespec = timespec { tv_sec: ut.ut_time, tv_nsec: 0 };

            if ut.ut_type == BOOT_TIME
              { found_boot_time = ts; }
          }

        fclose (f);
      }
  }

  #[cfg (feature = "linux_no_android")]
  if found_boot_time.tv_sec == 0
    {
      get_linux_boot_time_final_fallback (&mut found_boot_time);
    }

  /* C 214's `#else` arm (Adélie Linux, old FreeBSD, OpenBSD, native Windows), kept as text
     (P-51; dead: HAVE_STRUCT_UTMPX_UT_TYPE is 1).  */
  #[cfg (feature = "no_utmp_x_ut_type")]
  {
    #[cfg (feature = "linux_no_android")]
    {
      /* Workaround for Adélie Linux:  */
      get_linux_boot_time_fallback (&mut found_boot_time);
      if found_boot_time.tv_sec == 0
        { get_linux_boot_time_final_fallback (&mut found_boot_time); }
    }

    #[cfg (feature = "openbsd")]
    {
      /* Workaround for OpenBSD:  */
      get_openbsd_boot_time (&mut found_boot_time);
    }
  }

  /* C 230-232's `#if HAVE_SYS_SYSCTL_H && HAVE_SYSCTL && defined CTL_KERN && defined
     KERN_BOOTTIME && !defined __minix` is live on the frozen target and **dead on Android**
     (both `HAVE_SYS_SYSCTL_H` and `HAVE_SYSCTL` are undefined there, and the device exports no
     `sysctl`).  The call carries the same gate as the function it calls - ungated, the Android
     unit imported `sysctl` (status 4.749).  The three names its continuation lines spell are
     recorded in docs/ports/macro-body-notes.tsv, because the port's gate is a cfg the tokeniser
     cannot see.  */
  #[cfg (all (not (feature = "not_have_sys_sysctl_h"), not (feature = "not_have_sysctl")))]
  if found_boot_time.tv_sec == 0
    {
      get_bsd_boot_time_final_fallback (&mut found_boot_time);
    }

  #[cfg (feature = "have_haiku")]
  if found_boot_time.tv_sec == 0
    {
      get_haiku_boot_time (&mut found_boot_time);
    }

  #[cfg (feature = "os_h")]
  if found_boot_time.tv_sec == 0
    {
      get_haiku_boot_time_final_fallback (&mut found_boot_time);
    }

  #[cfg (feature = "cygwin_win32")]
  if found_boot_time.tv_sec == 0
    {
      /* Workaround for Windows:  */
      get_windows_boot_time (&mut found_boot_time);
      #[cfg (feature = "no_cygwin")]
      if found_boot_time.tv_sec == 0
        { get_windows_boot_time_fallback (&mut found_boot_time); }
    }

  if found_boot_time.tv_sec != 0
    {
      *p_boot_time = found_boot_time;
      return 0;
    }
  else
    { return -1; }

  }
}

/* C 274-275: `int get_boot_time (struct timespec *p_boot_time)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_boot_time (p_boot_time: *mut timespec) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* Cache the result from get_boot_time_uncached.  */
  /* C 278-279's `int volatile`/`struct timespec volatile` qualifiers have no Rust counterpart
     on a `static`; the reads and writes keep C's order (docs/ports/boot-time.md).  */
  static cached_result: VmGlobal<c_int> = VmGlobal::<c_int>::new (-1);
  static cached_boot_time: VmGlobal<timespec> = VmGlobal::<timespec>::new (timespec { tv_sec: 0, tv_nsec: 0 });

  if (*cached_result.as_mut_ptr ()) < 0
    {
      let mut boot_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
      let result: c_int = get_boot_time_uncached (&mut boot_time);
      (*cached_boot_time.as_mut_ptr ()) = boot_time;
      (*cached_result.as_mut_ptr ()) = result;
    }

  if (*cached_result.as_mut_ptr ()) == 0
    {
      *p_boot_time = (*cached_boot_time.as_mut_ptr ());
      return 0;
    }
  else
    { return -1; }

  }
}

/* C 298-306's `#else` arm - `int get_boot_time (struct timespec *p_boot_time) { return -1; }`
   - is dead (C 81's `#if` is true on this target) and is recorded in docs/ports/boot-time.md,
   not carried: a second `#[unsafe(no_mangle)]` definition of the same name would be the only
   other Rust spelling.  */
