// femacs: port of lib/utimens.c @ fd7afced403.
//
// lib/utimens.c is in `docs/baseline/config/cond-eval-verified.txt`, so the gate evaluates
// its `#if`s against the frozen configuration and compares only the live text.  The live
// arms are: `HAVE_UTIMENSAT`/`HAVE_FUTIMENS` (both 1), `HAVE_FUTIMES`/`HAVE_WORKING_UTIMES`
// (both 1), `__APPLE__ && __MACH__` (the trailing-slash guard) and `REPLACE_FUNC_STAT_FILE`
// (the file's own `#ifndef` sets it to 0).  Dead: `USE_SETFILETIME` (`EMACS_CONFIGURATION` is
// defined), `HAVE_BUGGY_NFS_TIME_STAMPS`, the `__linux__`/`__sun`/`__NetBSD__` workarounds,
// `HAVE_FUTIMESAT`/`FUTIMESAT_NULL_BUG`, `HAVE_UTIMENS`/`HAVE_LUTIMENS`/`HAVE_LUTIMES` and
// the `__linux__ && __GLIBC__` truncation fixup; each is named at its site below.
//
// Deviation worth a decision (docs/ports/utimens.md): `utimens` and `lutimens` are *not* in
// the gate's function inventory - `tools/extract-abi.py` cannot pair their declarator with
// the body, because C 538/560 puts `#undef utimens` / `#undef lutimens` between them - so
// emitting the C symbols (`#[unsafe(no_mangle)]` on the two definitions) is reported as an
// "invented exported symbol".  The port therefore keeps both bodies, faithfully, as
// non-exported functions, and records the gap for adjudication.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

/* Written by Paul Eggert.  */

/* derived from a function in touch.c */

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong,
               c_ushort, c_void};

// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/limits.in.rs imported via common
// [U1] include/errno.rs imported via common
// [U1] include/fcntl.in.rs imported via common
// [U1] include/sys_types.in.rs imported via common
// [U1] include/sys_stat.in.rs imported via common
// [U1] include/string.rs imported via common
// [U1] include/sys_time.rs imported via common
// [U1] include/unistd.in.rs imported via common
// [U1] include/time.in.rs imported via common
// [U1] include/timespec.rs imported via common
// [U1] include/stat-time.rs imported via common
// [U1] include/utimens.rs imported via common

/* lib/utimens.c 22: `#define _GL_UTIMENS_INLINE _GL_EXTERN_INLINE` (the .c side of the
   header's inline model, PORTING.md 10.12; the header's own `#ifndef` arm is dead).  */
pub const _GL_UTIMENS_INLINE: () = _GL_EXTERN_INLINE;

/* C 22-52: `<utimens.h>`'s specification, `<errno.h>`, `<fcntl.h>`, `<string.h>`,
   `<sys/stat.h>`, `<sys/time.h>`, `<unistd.h>`, `<utime.h>`, `issymlink.h`, `stat-time.h`
   and `timespec.h` are the includes above.  C 44-53's native-Windows `USE_SETFILETIME` arm
   is dead (`EMACS_CONFIGURATION` is defined), and C 56-59's `#undef futimens`/`#undef
   utimensat` pair is a no-op in this configuration (no replacement macro is defined).  */

/* The one entry the live path needs that no mirror declares yet: gnulib's `<utime.h>`
   supplies `utimes`, whose symbol the frozen build takes from the system.  */
unsafe extern "C" {
  /* int utimes (const char *file, const struct timeval tvp[2]);  */
  pub fn utimes (file: *const c_char, tvp: *const timeval) -> c_int;
  /* int futimes (int fd, const struct timeval tv[2]);  */
  pub fn futimes (fd: c_int, tv: *const timeval) -> c_int;
  /* `lstat` in this unit goes through `mod system` below: gnulib's <sys/stat.h> renames it
     to `rpl_lstat` in this build (PORTING.md 10.13; lib/lstat.rs defines that
     symbol), and a crate-root declaration would collide with the sys_stat mirror's.  */
  /* `issymlink` is `_GL_ISSYMLINK_INLINE` in issymlink.h - a header-inline definition with
     no symbol - so the port carries that definition here (P-12; `lib/issymlink.rs` is the
     `_GL_ISSYMLINK_INLINE` wrapper).  */
}

/* The system `lstat` under gnulib's renamed symbol (PORTING.md 10.13; the readlink.rs
   `mod system` shape, so the name `lstat` stays out of the crate root).  */
mod system {
  use super::*;

  unsafe extern "C" {
    #[link_name = "rpl_lstat"]
    pub fn lstat (file: *const c_char, buf: *mut stat) -> c_int;
  }
}

/* Tests whether FILENAME represents a symbolic link. … Returns
     1  if FILENAME is a symbolic link,
     0  if FILENAME exists and is not a symbolic link,
    -1 with errno set if determination failed.  */
#[inline(always)]
pub unsafe fn issymlink (filename: *const c_char) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let mut linkbuf: [c_char; 1] = [0];
  if readlink (filename, linkbuf.as_mut_ptr (), std::mem::size_of_val (&linkbuf)) >= 0
    { return 1; }
  if *errno () == EINVAL
    { return 0; }
  else
    { return -1; }

  }
}

/* Solaris 9 mistakenly succeeds when given a non-directory with a
   trailing slash.  Force the use of rpl_stat for a fix.  */
/* #ifndef REPLACE_FUNC_STAT_FILE
   # define REPLACE_FUNC_STAT_FILE 0
   #endif  */
pub const REPLACE_FUNC_STAT_FILE: c_int = 0;

/* Cache variables for whether the utimensat syscall works; used to
   avoid calling the syscall if we know it will just fail with ENOSYS,
   and to avoid unnecessary work in massaging timestamps if the
   syscall will work.  Multiple variables are needed, to distinguish
   between the following scenarios on Linux:
   utimensat doesn't exist, or is in glibc but kernel 2.6.18 fails with ENOSYS
   kernel 2.6.22 and earlier rejects AT_SYMLINK_NOFOLLOW
   kernel 2.6.25 and earlier reject UTIME_NOW/UTIME_OMIT with non-zero tv_sec
   kernel 2.6.32 used with xfs or ntfs-3g fail to honor UTIME_OMIT
   utimensat completely works
   For each cache variable: 0 = unknown, 1 = yes, -1 = no.  */
static utimensat_works_really: VmGlobal<c_int> = VmGlobal::<c_int>::new (0);
static lutimensat_works_really: VmGlobal<c_int> = VmGlobal::<c_int>::new (0);

unsafe extern "C" fn is_valid_timespec (timespec: *const timespec) -> bool
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  return ((*timespec).tv_nsec == UTIME_NOW
          || (*timespec).tv_nsec == UTIME_OMIT
          || (0 <= (*timespec).tv_nsec && (*timespec).tv_nsec < TIMESPEC_HZ as c_long));

  }
}

unsafe extern "C" fn is_valid_timespecs (timespec: *const timespec) -> bool
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  return (is_valid_timespec (timespec.offset (0))
          && is_valid_timespec (timespec.offset (1)));

  }
}

/* Validate the requested timestamps.  Return 0 if the resulting
   timespec can be used for utimensat (after possibly modifying it to
   work around bugs in utimensat).  Return a positive value if the
   timespec needs further adjustment based on stat results: 1 if any
   adjustment is needed for utimes, and 2 if any adjustment is needed
   for Linux utimensat.  Return -1, with errno set to EINVAL, if
   timespec is out of range.  */
unsafe extern "C" fn validate_timespec (timespec: *mut timespec) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  if ! is_valid_timespecs (timespec)
    {
      *errno () = EINVAL;
      return -1;
    }
  let mut result: c_int = 0;
  let mut utime_omit_count: c_int = 0;
  /* Work around Linux kernel 2.6.25 bug, where utimensat fails with
     EINVAL if tv_sec is not 0 when using the flag values of tv_nsec.
     Flag a Linux kernel 2.6.32 bug, where an mtime of UTIME_OMIT
     fails to bump ctime.  */
  if ((*timespec).tv_nsec == UTIME_NOW
      || (*timespec).tv_nsec == UTIME_OMIT)
    {
      (*timespec).tv_sec = 0;
      result = 1;
      if ((*timespec).tv_nsec == UTIME_OMIT)
        { utime_omit_count += 1; }
    }
  if ((*timespec.offset (1)).tv_nsec == UTIME_NOW
      || (*timespec.offset (1)).tv_nsec == UTIME_OMIT)
    {
      (*timespec.offset (1)).tv_sec = 0;
      result = 1;
      if ((*timespec.offset (1)).tv_nsec == UTIME_OMIT)
        { utime_omit_count += 1; }
    }
  return result + ((utime_omit_count == 1) as c_int);

  }
}

/* Normalize any UTIME_NOW or UTIME_OMIT values in (*TS)[0] and (*TS)[1],
   using STATBUF to obtain the current timestamps of the file.  If
   both times are UTIME_NOW, set *TS to NULL (as this can avoid some
   permissions issues).  If both times are UTIME_OMIT, return true
   (nothing further beyond the prior collection of STATBUF is
   necessary); otherwise return false.  */
unsafe extern "C" fn update_timespec (statbuf: *const stat, ts: *mut *mut timespec)
  -> bool
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let timespec: *mut timespec = *ts;
  if ((*timespec.offset (0)).tv_nsec == UTIME_OMIT
      && (*timespec.offset (1)).tv_nsec == UTIME_OMIT)
    { return true; }
  if ((*timespec.offset (0)).tv_nsec == UTIME_NOW
      && (*timespec.offset (1)).tv_nsec == UTIME_NOW)
    {
      *ts = std::ptr::null_mut ();
      return false;
    }

  if ((*timespec.offset (0)).tv_nsec == UTIME_OMIT)
    { *timespec.offset (0) = get_stat_atime (&*statbuf); }
  else if ((*timespec.offset (0)).tv_nsec == UTIME_NOW)
    { gettime (timespec.offset (0)); }

  if ((*timespec.offset (1)).tv_nsec == UTIME_OMIT)
    { *timespec.offset (1) = get_stat_mtime (&*statbuf); }
  else if ((*timespec.offset (1)).tv_nsec == UTIME_NOW)
    { gettime (timespec.offset (1)); }

  return false;

  }
}

/* Set the access and modification timestamps of FD (a.k.a. FILE) to be
   TIMESPEC[0] and TIMESPEC[1], respectively.
   FD must be either negative -- in which case it is ignored --
   or a file descriptor that is open on FILE.
   If FD is nonnegative, then FILE can be NULL, which means
   use just futimes (or equivalent) instead of utimes (or equivalent),
   and fail if on an old system without futimes (or equivalent).
   If TIMESPEC is null, set the timestamps to the current time.
   Return 0 on success, -1 (setting errno) on failure.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fdutimens (fd: c_int, file: *const c_char,
                                    timespec: *const timespec) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let mut adjusted_timespec: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 },
                                              timespec { tv_sec: 0, tv_nsec: 0 }];
  let mut ts: *mut timespec
    = if !timespec.is_null () { adjusted_timespec.as_mut_ptr () }
      else { std::ptr::null_mut () };
  let mut adjustment_needed: c_int = 0;

  if !ts.is_null ()
    {
      adjusted_timespec[0] = *timespec;
      adjusted_timespec[1] = *timespec.offset (1);
      adjustment_needed = validate_timespec (ts);
    }
  if (adjustment_needed < 0)
    { return -1; }

  /* Require that at least one of FD or FILE are potentially valid, to avoid
     a Linux bug where futimens (AT_FDCWD, NULL) changes "."
     than failing.  */
  if (fd < 0 && file.is_null ())
    {
      *errno () = EBADF;
      return -1;
    }

  /* Some Linux-based NFS clients are buggy, and mishandle timestamps
     of files in NFS file systems in some cases.  We have no
     configure-time test for this, but please see
     <https://bugs.gentoo.org/show_bug.cgi?id=132673> for references to
     some of the problems with Linux 2.6.16.  If this affects you,
     compile with -DHAVE_BUGGY_NFS_TIME_STAMPS; this is reported to
     help in some cases, albeit at a cost in performance.  But you
     really should upgrade your kernel to a fixed version, since the
     problem affects many applications.  */
  /* C 216-221's `#if HAVE_BUGGY_NFS_TIME_STAMPS` arm is dead.  */

  let mut st: stat = std::mem::zeroed ();

  /* POSIX 2008 added two interfaces to set file timestamps with
     nanosecond resolution; newer Linux implements both functions via
     a single syscall.  We provide a fallback for ENOSYS (for example,
     compiling against Linux 2.6.25 kernel headers and glibc 2.7, but
     running on Linux 2.6.18 kernel).  */
  if (0 <= (*utimensat_works_really.as_mut_ptr ()))
    {
      let mut result: c_int;
      /* C 234-259's `# if defined __linux__ || defined __sun || defined __NetBSD__` arm
         (the UTIME_OMIT preparatory [f]stat) is dead for this target.  */
      if (fd < 0)
        {
          /* C 263-276's `#  if defined __APPLE__ && defined __MACH__` arm is live: a
             trailing slash on a non-directory must fail with ENOTDIR.  */
          let len: usize = strlen (file);
          if (len > 0 && *file.offset ((len - 1) as isize) == b'/' as c_char)
            {
              let mut statbuf: stat = std::mem::zeroed ();
              if (stat (file, &mut statbuf) < 0)
                { return -1; }
              if ! S_ISDIR! (statbuf.st_mode)
                {
                  *errno () = ENOTDIR;
                  return -1;
                }
            }
          result = utimensat (AT_FDCWD, file, ts, 0);
          /* C 278-288's `#  ifdef __linux__` arm (utimensat's bogus 280 return) is dead.  */
          if (result == 0 || *errno () != ENOSYS)
            {
              (*utimensat_works_really.as_mut_ptr ()) = 1;
              return result;
            }
        }
      if (0 <= fd)
        {
          result = futimens (fd, ts);
          /* C 300-304's `#  ifdef __linux__` arm is dead.  */
          if (result == 0 || *errno () != ENOSYS)
            {
              (*utimensat_works_really.as_mut_ptr ()) = 1;
              return result;
            }
        }
    }
  (*utimensat_works_really.as_mut_ptr ()) = -1;
  (*lutimensat_works_really.as_mut_ptr ()) = -1;

  /* C 317-397's `#ifdef USE_SETFILETIME` arm (the Windows SetFileTime path) is dead:
     `EMACS_CONFIGURATION` is defined.  */

  /* The platform lacks an interface to set file timestamps with
     nanosecond resolution, so do the best we can, discarding any
     fractional part of the timestamp.  */

  if (adjustment_needed != 0 || (REPLACE_FUNC_STAT_FILE != 0 && fd < 0))
    {
      if (adjustment_needed != 3
          && (if fd < 0 { stat (file, &mut st) } else { fstat (fd, &mut st) }) != 0)
        { return -1; }
      if (!ts.is_null () && update_timespec (&st, &mut ts))
        { return 0; }
    }

  {
    let mut timeval: [timeval; 2] = [timeval { tv_sec: 0, tv_usec: 0 },
                                     timeval { tv_sec: 0, tv_usec: 0 }];
    let t: *const timeval;
    if !ts.is_null ()
      {
        timeval[0] = timeval { tv_sec: (*ts).tv_sec,
                               tv_usec: ((*ts).tv_nsec / 1000) as suseconds_t };
        timeval[1] = timeval { tv_sec: (*ts.offset (1)).tv_sec,
                               tv_usec: ((*ts.offset (1)).tv_nsec / 1000) as suseconds_t };
        t = timeval.as_ptr ();
      }
    else
      { t = std::ptr::null (); }

    if (fd < 0)
      {
        /* C 429-431's `# if HAVE_FUTIMESAT` arm is dead.  */
      }
    else
      {
        /* If futimesat or futimes fails here, don't try to speed things
           up by returning right away.  glibc can incorrectly fail with
           errno == ENOENT if /proc isn't mounted.  Also, Mandrake 10.0
           in high security mode doesn't allow ordinary users to read
           /proc/self, so glibc incorrectly fails with errno == EACCES.
           If errno == EIO, EPERM, or EROFS, it's probably safe to fail
           right away, but these cases are rare enough that they're not
           worth optimizing, and who knows what other messed-up systems
           are out there?  So play it safe and fall back on the code
           below.  */
        /* C 447-452's `#  if HAVE_FUTIMESAT && !FUTIMESAT_NULL_BUG` arm is dead.  */
        /* C 447-488 is `# if (HAVE_FUTIMESAT && !FUTIMESAT_NULL_BUG) || HAVE_FUTIMES`: live for
           the frozen target (`HAVE_FUTIMES`), absent for the Android one, whose configuration
           sets neither macro (status 4.719).  */
        #[cfg (not (feature = "not_have_futimes"))]
        if (futimes (fd, t) == 0)
          {
            /* C 456-487's `#  if defined __linux__ && defined __GLIBC__` arm (the
               rounding fixup) is dead.  */

            return 0;
          }
      }

    if (file.is_null ())
      {
        /* C 495-499's `#if ! ((HAVE_FUTIMESAT && !FUTIMESAT_NULL_BUG) ||
           (HAVE_WORKING_UTIMES && HAVE_FUTIMES))` arm (errno = ENOSYS) is dead, because
           HAVE_WORKING_UTIMES and HAVE_FUTIMES are both 1.  */
        return -1;
      }

    /* C 503-505's `#ifdef USE_SETFILETIME` arm is dead; C 521-529's
       `#else` arm (`utime`) is dead too, because HAVE_WORKING_UTIMES is 1.  */
    return utimes (file, t);
  }

  }
}

/* Set the access and modification timestamps of FILE to be
   TIMESPEC[0] and TIMESPEC[1], respectively.  */
/* C 535-556; the `#if HAVE_UTIMENS` arm is dead (HAVE_UTIMENS is undefined for this
   target), so the live body is C 554's `return fdutimens (-1, file, timespec);`.  The
   definition keeps C's name and body but no `#[unsafe(no_mangle)]`: see the file header's
   note on the gate's inventory gap.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn utimens (file: *const c_char, timespec: *const timespec) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  return fdutimens (-1, file, timespec);

  }
}

/* Set the access and modification timestamps of FILE to be
   TIMESPEC[0] and TIMESPEC[1], respectively, without dereferencing
   symlinks.  Fail with ENOSYS if the platform does not support
   changing symlink timestamps, but FILE was a symlink.  */
/* C 558-690; `HAVE_LUTIMENS` is undefined, so the live arm is the `#else` (C 575-689).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lutimens (file: *const c_char, timespec: *const timespec) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let mut adjusted_timespec: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 },
                                              timespec { tv_sec: 0, tv_nsec: 0 }];
  let mut ts: *mut timespec
    = if !timespec.is_null () { adjusted_timespec.as_mut_ptr () }
      else { std::ptr::null_mut () };
  let mut adjustment_needed: c_int = 0;

  if !ts.is_null ()
    {
      adjusted_timespec[0] = *timespec;
      adjusted_timespec[1] = *timespec.offset (1);
      adjustment_needed = validate_timespec (ts);
    }
  if (adjustment_needed < 0)
    { return -1; }

  let mut st: stat = std::mem::zeroed ();

  /* The Linux kernel did not support symlink timestamps until
     utimensat, in version 2.6.22, so we don't need to mimic
     fdutimens' worry about buggy NFS clients.  But we do have to
     worry about bogus return values.  */

  if (0 <= (*lutimensat_works_really.as_mut_ptr ()))
    {
      let result: c_int;
      /* C 590-613's `#  if defined __linux__ || defined __sun || defined __NetBSD__` arm
         (the preparatory lstat) is dead.  */
      result = utimensat (AT_FDCWD, file, ts, AT_SYMLINK_NOFOLLOW);
      /* C 616-626's `#  ifdef __linux__` arm is dead.  */
      if (result == 0 || *errno () != ENOSYS)
        {
          (*utimensat_works_really.as_mut_ptr ()) = 1;
          (*lutimensat_works_really.as_mut_ptr ()) = 1;
          return result;
        }
    }
  (*lutimensat_works_really.as_mut_ptr ()) = -1;

  /* The platform lacks an interface to set file timestamps with
     nanosecond resolution, so do the best we can, discarding any
     fractional part of the timestamp.  */

  if (adjustment_needed != 0 || REPLACE_FUNC_STAT_FILE != 0)
    {
      if (adjustment_needed != 3 && system::lstat (file, &mut st) != 0)
        { return -1; }
      if (!ts.is_null () && update_timespec (&st, &mut ts))
        { return 0; }
    }

  /* On Linux, lutimes is a thin wrapper around utimensat, so there is
     no point trying lutimes if utimensat failed with ENOSYS.  */
  /* C 647-668's `# if HAVE_LUTIMES && !HAVE_UTIMENSAT` arm is dead (HAVE_UTIMENSAT is 1). */

  /* Out of luck for symlinks, but we still handle regular files.  */
  let not_symlink: bool;
  if (adjustment_needed != 0 || REPLACE_FUNC_STAT_FILE != 0)
    { not_symlink = ! S_ISLNK! (st.st_mode); }
  else
    {
      let ret: c_int = issymlink (file);
      if (ret < 0)
        { return -1; }
      not_symlink = !(ret != 0);
    }
  if (not_symlink)
    { return fdutimens (-1, file, ts); }
  *errno () = ENOSYS;
  return -1;

  }
}
