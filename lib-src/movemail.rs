// femacs: port of lib-src/movemail.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names, the same `main`.
// This is a program of the C tree's lib-src directory (batch F), listed by the reference
// `lib-src/Makefile` under `UTILITIES`, and linked against the port's gnulib mirrors,
// `src/syswait.h`'s mirror and libSystem only.
//
// **Dead text, recorded not transcribed** (P-08; `python3 tools/check-cond-eval.py
// lib-src/movemail.c` reports `37 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the
// ledger's live set is the frozen configuration's):
//   * `MAIL_USE_POP` is undefined: the whole POP half (C 240-244, C 616-871 - `popmail`,
//     `pop_retr`, `mbx_write`, `mbx_delimit_begin`, `mbx_delimit_end`, `movemail_strftime`)
//     is not compiled, and neither is `#include "pop.h"`;
//   * `MAIL_USE_FLOCK` is 1, so `MAIL_USE_SYSTEM_LOCK` is defined: the lock-file machinery of
//     C 252-330 (the `EXXXXXX` temp file, `link`, the five-minute stale-lock rule and
//     `xmalloc`) and the `#ifndef MAIL_USE_SYSTEM_LOCK` tail of C 474-486 are dead - the live
//     lock is `flock (indesc, LOCK_EX)`;
//   * `MAIL_USE_MAILLOCK`/`HAVE_LIBMAIL`/`HAVE_MAILLOCK_H` are undefined: `mail_spool_name`
//     (C 511-547) and the `maillock`/`touchlock`/`mailunlock` arms are dead;
//   * `WINDOWSNT`, `MSDOS`, `DISABLE_DIRECT_ACCESS` and `MAIL_UNLINK_SPOOL` are undefined.
// The two variables those dead arms would assign (`lockname`, `spool_name`) therefore stay null
// in the port exactly as they do in C, and their live uses are kept.
//
// Use-site handling (P-26/P-72 (3)), because C names them from headers the port has no mirror
// for: `flock` and `LOCK_EX` (`<sys/file.h>`, `LOCK_EX` measured as 2), `mkostemp`
// (`<stdlib.h>`; only the dead lock-file arm calls it), `waitpid` (`<sys/wait.h>`),
// `fork`/`setuid`/`setregid` (`<unistd.h>`) and `WIFEXITED`/`WEXITSTATUS` (`<sys/wait.h>`'s
// macros, written as the two inline functions `src/callproc.rs` 267-268 established with the
// same measured bodies).  `<unlocked-io.h>` is handled the way the other units handle it.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong,
              c_void};

include!("../src/include/config.rs");        // C 66: `#include <config.h>`
include!("../lib/include/sys_types.in.rs");  // C 67: `#include <sys/types.h>` (uid_t, pid_t)
include!("../lib/include/sys_stat.in.rs");   // C 68: `#include <sys/stat.h>` (stat, umask)
include!("../lib/include/stdlib.rs");        // C 70: `#include <stdlib.h>`
include!("../lib/include/stdio.rs");         // `<stdio.h>`: fprintf, stderr, perror
include!("../lib/include/errno.rs");         // C 71: `#include <errno.h>`
include!("../lib/include/time.in.rs");       // C 72: `#include <time.h>`
include!("../lib/include/timespec.rs");      // C 73: `#include <timespec.h>`
include!("../lib/include/getopt.in.rs");     // C 75: `#include <getopt.h>`
include!("../lib/include/unistd.in.rs");     // C 76: `#include <unistd.h>`
include!("../lib/include/fcntl.in.rs");      // C 77: `#include <fcntl.h>`
include!("../lib/include/signal.rs");        // C 78: `#include <signal.h>`
include!("../lib/include/string.rs");        // C 79: `#include <string.h>`
include!("../src/include/syswait.rs");       // C 83: `#include "syswait.h"`

/// C 117-121: `#ifdef MAIL_USE_FLOCK / #define MAIL_USE_SYSTEM_LOCK / #endif` - the frozen
/// configuration defines `MAIL_USE_FLOCK` (macros.tsv), so this name is defined and C 119's
/// empty `#define` is what the port records; the `#ifdef MAIL_USE_SYSTEM_LOCK` arms below then
/// take their "system lock" side.
pub const MAIL_USE_SYSTEM_LOCK: bool = true;

/// C 186-188: the live `# define ARGSTR "p"` (the `MAIL_USE_POP` arm would add `r`).
const ARGSTR: &core::ffi::CStr = c"p";

/// `<sys/file.h>`: `#define LOCK_EX 2` (measured with clang against the frozen SDK).
const LOCK_EX: c_int = 2;

/// `<signal.h>`: `#define SIG_DFL ((void (*)(int)) 0)` - the mirror keeps the integer, because
/// the handler parameter is a `usize` there (`signal.rs` 45 says the same).
const SIG_DFL: usize = 0;

/// `<stdlib.h>`: `void exit (int);` - no mirror carries it yet (the emacs.rs/sysdep.rs
/// precedent keeps it at the use site).
unsafe extern "C" {
  pub fn exit (status: c_int) -> !;
}

unsafe extern "C" {
  /// `<sys/file.h>`: `int flock (int, int);`
  pub fn flock (fd: c_int, operation: c_int) -> c_int;
  /// `<sys/wait.h>`: `pid_t waitpid (pid_t, int *, int);`
  pub fn waitpid (pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
  /// `<unistd.h>`: `pid_t fork (void);`
  pub fn fork () -> pid_t;
  /// `<unistd.h>`: `int setuid (uid_t);`
  pub fn setuid (uid: uid_t) -> c_int;
  /// `<unistd.h>`: `int setregid (gid_t, gid_t);`
  pub fn setregid (rgid: gid_t, egid: gid_t) -> c_int;
  /// `<stdlib.h>`: `int mkostemp (char *, int);` - the dead lock-file arm's entry.
  pub fn mkostemp (template: *mut c_char, flags: c_int) -> c_int;
}

/// `<sys/wait.h>`: `WIFEXITED (status)` - measured `(status & 0x7f) == 0`; written as the
/// `src/callproc.rs` 267 inline function.
#[inline(always)]
unsafe fn WIFEXITED (status: c_int) -> bool { (status & 0o177) == 0 }

/// `<sys/wait.h>`: `WEXITSTATUS (status)` - measured `(status >> 8) & 0xff`.
#[inline(always)]
unsafe fn WEXITSTATUS (status: c_int) -> c_int { (status >> 8) & 0xff }

// C 161: `static char *delete_lockname;`  (`/* Nonzero means this is name of a lock file to
// delete on fatal error.  */`)
static mut delete_lockname: *mut c_char = std::ptr::null_mut ();

// C 164: `int main (int argc, char **argv)`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  // C 166-172
  let inname: *mut c_char;
  let outname: *mut c_char;
  let mut indesc: c_int = 0;
  let mut outdesc: c_int = 0;
  let mut nread: ssize_t;
  let mut wait_status: c_int = 0;
  let mut c: c_int;
  let mut preserve_mail: bool = false;

  // C 197-198: `uid_t real_gid = getgid (); uid_t priv_gid = getegid ();`
  let real_gid: uid_t = getgid ();
  let priv_gid: uid_t = getegid ();

  // C 200: `delete_lockname = 0;`
  delete_lockname = std::ptr::null_mut ();

  // C 202-215: `while (0 <= (c = getopt (argc, argv, ARGSTR))) { switch (c) { … } }`
  loop
    {
      c = getopt (argc, argv, ARGSTR.as_ptr ());
      if ! (0 <= c)
	{
	  break;
	}
      match c
	{
	  // C 209-211: `case 'p': preserve_mail = true; break;`
	  112 /* 'p' */ =>
	    {
	      preserve_mail = true;
	    }
	  // C 212-213: `default: return EXIT_FAILURE;`
	  _ =>
	    {
	      return EXIT_FAILURE;
	    }
	}
    }

  // C 217-231: `if ((argc - optind != 2)) { fprintf (stderr, "Usage: …"); return EXIT_FAILURE; }`
  if argc - optind != 2
    {
      fprintf (stderr, c"Usage: movemail [-p] inbox destfile%s\n".as_ptr (), c"".as_ptr ());
      return EXIT_FAILURE;
    }

  // C 233-234: `inname = argv[optind]; outname = argv[optind+1];`
  inname = *argv.offset (optind as isize);
  outname = *argv.offset (optind as isize + 1);

  // C 236-237: `if (*outname == 0) fatal ("Destination file name is empty", 0, 0);`
  if *outname == 0
    {
      fatal (c"Destination file name is empty".as_ptr (), std::ptr::null (), std::ptr::null ());
    }

  // C 250-252: `char *lockname = 0; char *spool_name = 0;`
  // (their only assignments are the dead lock-file and maillock arms, so C leaves them null
  // here too; the live uses below are kept.)
  let lockname: *mut c_char = std::ptr::null_mut ();
  let spool_name: *mut c_char = std::ptr::null_mut ();

  // C 330-332: `#ifdef SIGCHLD signal (SIGCHLD, SIG_DFL); #endif`
  signal (SIGCHLD, SIG_DFL);

  // C 334-336: `pid_t child = fork (); if (child < 0) fatal (…);`
  let child: pid_t = fork ();
  if child < 0
    {
      fatal (c"Error in fork; %s".as_ptr (), strerror (*errno ()) as *const c_char,
	     std::ptr::null ());
    }

  if child == 0
    {
      // C 340-341: `int lockcount = 0; int status = 0;`
      let mut lockcount: c_int = 0;
      let mut status: c_int = 0;

      // C 348-349: `if (setuid (getuid ()) < 0 || setregid (-1_i32 as gid_t, real_gid) < 0) fatal (…);`
      if setuid (getuid ()) < 0 || setregid (-1_i32 as gid_t, real_gid as gid_t) < 0
	{
	  fatal (c"Failed to drop privileges".as_ptr (), std::ptr::null (), std::ptr::null ());
	}

      // C 356: `indesc = open (inname, O_RDWR | O_BINARY);`
      indesc = open (inname, O_RDWR | O_BINARY);
      if indesc < 0
	{
	  pfatal_with_name (inname);
	}

      // C 363: `umask (umask (0) & 0377);`
      umask (umask (0) & 0o377);

      // C 365-367: `outdesc = open (outname, O_WRONLY | O_BINARY | O_CREAT | O_EXCL, 0666);`
      outdesc = open (outname, O_WRONLY | O_BINARY | O_CREAT | O_EXCL, 0o666);
      if outdesc < 0
	{
	  pfatal_with_name (outname);
	}

      // C 369-370
      if setregid (-1_i32 as gid_t, priv_gid as gid_t) < 0
	{
	  fatal (c"Failed to regain privileges".as_ptr (), std::ptr::null (), std::ptr::null ());
	}

      /* C 373's `retry_lock:` label with C 407's backward `goto retry_lock` becomes a labelled
	 loop whose `continue` is that jump (PORTING.md 2.2).  */
      'retry_lock: loop
	{
	  // C 383-405's live arm: `status = flock (indesc, LOCK_EX);`
	  status = flock (indesc, LOCK_EX);

	  // C 411-420: `if (status < 0) { if (++lockcount <= 5 && (errno == EAGAIN || errno ==
	  // EBUSY)) { sleep (1); goto retry_lock; } pfatal_with_name (inname); }`
	  if status < 0
	    {
	      lockcount += 1;
	      if lockcount <= 5 && (*errno () == EAGAIN || *errno () == EBUSY)
		{
		  sleep (1);
		  continue 'retry_lock;
		}

	      pfatal_with_name (inname);
	    }
	  break;
	}

      // C 423-451: the copy loop `{ char buf[1024]; while (true) { … } }`
      {
	let mut buf: [c_char; 1024] = [0; 1024];

	loop
	  {
	    nread = read (indesc, buf.as_mut_ptr () as *mut c_void, std::mem::size_of_val (&buf));
	    if nread < 0
	      {
		pfatal_with_name (inname);
	      }
	    if nread != write (outdesc, buf.as_ptr () as *const c_void, nread as usize)
	      {
		let saved_errno: c_int = *errno ();
		unlink (outname);
		*errno () = saved_errno;
		pfatal_with_name (outname);
	      }
	    if nread < std::mem::size_of_val (&buf) as ssize_t
	      {
		break;
	      }
	  }
      }

      // C 455-456: `if (fsync (outdesc) != 0 && errno != EINVAL) pfatal_and_delete (outname);`
      if fsync (outdesc) != 0 && *errno () != EINVAL
	{
	  pfatal_and_delete (outname);
	}

      // C 459-460
      if setregid (-1_i32 as gid_t, real_gid as gid_t) < 0
	{
	  fatal (c"Failed to drop privileges".as_ptr (), std::ptr::null (), std::ptr::null ());
	}

      // C 463-464: `if (close (outdesc) != 0) pfatal_and_delete (outname);`
      if close (outdesc) != 0
	{
	  pfatal_and_delete (outname);
	}

      // C 467-473: the live `MAIL_USE_SYSTEM_LOCK` arm
      if ! preserve_mail
	{
	  if ftruncate (indesc, 0) != 0
	    {
	      pfatal_with_name (inname);
	    }
	}

      // C 476: `close (indesc);`
      close (indesc);

      // C 497-498
      if setregid (-1_i32 as gid_t, priv_gid as gid_t) < 0
	{
	  fatal (c"Failed to regain privileges".as_ptr (), std::ptr::null (), std::ptr::null ());
	}

      // C 506: `return EXIT_SUCCESS;`
      return EXIT_SUCCESS;
    }

  // C 509-510: `if (waitpid (child, &wait_status, 0) < 0) fatal (…);`
  if waitpid (child, &mut wait_status, 0) < 0
    {
      fatal (c"Error in waitpid; %s".as_ptr (), strerror (*errno ()) as *const c_char,
	     std::ptr::null ());
    }

  // C 511-514
  if ! WIFEXITED (wait_status)
    {
      return EXIT_FAILURE;
    }
  else if WEXITSTATUS (wait_status) != 0
    {
      return WEXITSTATUS (wait_status);
    }

  // C 516-517: `if (lockname) unlink (lockname);`
  if ! lockname.is_null ()
    {
      unlink (lockname);
    }

  // C 521: `return EXIT_SUCCESS;`
  return EXIT_SUCCESS;
}

// C 548-553: `static void fatal (const char *s1, const char *s2, const char *s3)`
unsafe fn fatal (s1: *const c_char, s2: *const c_char, s3: *const c_char)
{
  if ! delete_lockname.is_null ()
    {
      unlink (delete_lockname);
    }
  error (s1, s2, s3);
  exit (EXIT_FAILURE);
}

// C 560-571: `static void error (const char *s1, const char *s2, const char *s3)`
unsafe fn error (s1: *const c_char, s2: *const c_char, s3: *const c_char)
{
  fprintf (stderr, c"movemail: ".as_ptr ());
  if ! s3.is_null ()
    {
      fprintf (stderr, s1, s2, s3);
    }
  else if ! s2.is_null ()
    {
      fprintf (stderr, s1, s2);
    }
  else
    {
      fprintf (stderr, c"%s".as_ptr (), s1);
    }
  fprintf (stderr, c"\n".as_ptr ());
}

// C 573-577: `static void pfatal_with_name (char *name)`
unsafe fn pfatal_with_name (name: *mut c_char)
{
  fatal (c"%s for %s".as_ptr (), strerror (*errno ()) as *const c_char, name);
}

// C 579-586: `static void pfatal_and_delete (char *name)`
unsafe fn pfatal_and_delete (name: *mut c_char)
{
  let s: *const c_char = strerror (*errno ()) as *const c_char;
  unlink (name);
  fatal (c"%s for %s".as_ptr (), s, name);
}
