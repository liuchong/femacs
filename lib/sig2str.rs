// femacs: port of lib/sig2str.c @ fd7afced403.
// [U1-MODULE-UNIT]
use crate::common::*;
/* Written by Paul Eggert.  */

/* emacs/lib/sig2str.c (@ 364 lines) is on the verified list: the `#ifdef SIGxxx` guards around
   the signal table are decided by the frozen target's <signal.h> values (which the macro table
   carries), so the table below is the 32 entries C compiles here plus C's trailing
   `{ 0, "EXIT" }`.  `SIGRTMIN`/`SIGRTMAX` are *not* defined on this target, so C 33-39's
   fallbacks are the live text (0 and -1) and the two realtime branches below are inert exactly
   as in C.  C 20-31's includes are the signal, limits, stdcountof, stdlib, stdio and string
   mirrors (whose `streq` gnulib's <string.h> template defines); C 274's `sprintf` is declared in-unit (the doprnt.rs
   shape, since that mirror deliberately does not carry it).  */
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/signal.rs imported via common
// [U1] include/sig2str.rs imported via common
// [U1] include/limits.in.rs imported via common
// [U1] include/stdcountof.rs imported via common
// [U1] include/stdlib.rs imported via common
// [U1] include/string.in.rs imported via common

/* C 274's <stdio.h>: `sprintf`, declared here because the stdio mirror does not carry it (a
   mirror-level declaration is a second definition in src/doprnt.c, which declares its own).  */
unsafe extern "C" {
  pub fn sprintf (s: *mut c_char, format: *const c_char, ...) -> c_int;
}

/* C 33-39's `#ifndef SIGRTMIN / # define SIGRTMIN 0 / #endif` and its `SIGRTMAX` twin: both are
   live on this target (measured: <signal.h> defines neither), so the realtime-signal code paths
   below are inert exactly as in C.  */
pub const SIGRTMIN: c_int = 0;
pub const SIGRTMAX: c_int = SIGRTMIN - 1;

/* C 41's `#define NUMNAME(name) { SIG##name, #name }`: a Rust macro cannot concatenate an
   identifier or stringify its argument, so the port writes each entry out and records the
   macro in docs/ports/excluded-macros.txt.  */

/* Signal names and numbers.  Put the preferred name first.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct numname
{
  pub num: c_int,
  pub name: [c_char; 8],
}

/* `name8` turns C's `char const name[8] = "NAME"` (zero-padded) into the same eight bytes; the
   literal keeps C's spelled name for the ledger.  */
const fn name8 (s: &str) -> [c_char; 8]
{
  let bytes: &[u8] = s.as_bytes ();
  let mut out: [c_char; 8] = [0; 8];
  let mut i: usize = 0;
  while i < bytes.len ()
    {
      out[i] = bytes[i] as c_char;
      i += 1;
    }
  out
}

static numname_table: [numname; 33] =
  [
    /* Signals required by POSIX 1003.1-2001 base, listed in
       traditional numeric order where possible.  */
    numname { num: SIGHUP, name: name8 ("HUP") },
    numname { num: SIGINT, name: name8 ("INT") },
    numname { num: SIGQUIT, name: name8 ("QUIT") },
    numname { num: SIGILL, name: name8 ("ILL") },
    numname { num: SIGTRAP, name: name8 ("TRAP") },
    numname { num: SIGABRT, name: name8 ("ABRT") },
    numname { num: SIGFPE, name: name8 ("FPE") },
    numname { num: SIGKILL, name: name8 ("KILL") },
    numname { num: SIGSEGV, name: name8 ("SEGV") },
    /* On Haiku, SIGSEGV == SIGBUS, but we prefer SIGSEGV to match
       strsignal.c output, so SIGBUS must be listed second.  */
    numname { num: SIGBUS, name: name8 ("BUS") },
    numname { num: SIGPIPE, name: name8 ("PIPE") },
    numname { num: SIGALRM, name: name8 ("ALRM") },
    numname { num: SIGTERM, name: name8 ("TERM") },
    numname { num: SIGUSR1, name: name8 ("USR1") },
    numname { num: SIGUSR2, name: name8 ("USR2") },
    numname { num: SIGCHLD, name: name8 ("CHLD") },
    numname { num: SIGURG, name: name8 ("URG") },
    numname { num: SIGSTOP, name: name8 ("STOP") },
    numname { num: SIGTSTP, name: name8 ("TSTP") },
    numname { num: SIGCONT, name: name8 ("CONT") },
    numname { num: SIGTTIN, name: name8 ("TTIN") },
    numname { num: SIGTTOU, name: name8 ("TTOU") },
    /* Signals required by POSIX 1003.1-2001 with the XSI extension.  */
    numname { num: SIGSYS, name: name8 ("SYS") },
    /* Real-time signals; this target has none.  */
    numname { num: SIGVTALRM, name: name8 ("VTALRM") },
    numname { num: SIGPROF, name: name8 ("PROF") },
    numname { num: SIGXCPU, name: name8 ("XCPU") },
    numname { num: SIGXFSZ, name: name8 ("XFSZ") },
    /* Nonstandard signals.  */
    numname { num: SIGIOT, name: name8 ("IOT") },
    numname { num: SIGEMT, name: name8 ("EMT") },
    numname { num: SIGWINCH, name: name8 ("WINCH") },
    numname { num: SIGINFO, name: name8 ("INFO") },
    numname { num: SIGIO, name: name8 ("IO") },
    /* Korn shell and Bash, of uncertain vintage.  */
    numname { num: 0, name: name8 ("EXIT") },
  ];

/* ISDIGIT differs from isdigit, as follows:
   - Its arg may be any int or unsigned int; it need not be an unsigned char
     or EOF.
   - It's typically faster.
   POSIX says that only '0' through '9' are digits.  Prefer ISDIGIT to
   isdigit unless it's important to use the locale's definition
   of "digit" even when the host does not conform to POSIX.  */
macro_rules! ISDIGIT {
  ($c:expr) => { (($c) as c_uint).wrapping_sub (b'0' as c_uint) <= 9 };
}

/* Convert the signal name SIGNAME to a signal number.  Return the
   signal number if successful, -1 otherwise.  */

/* C 276-318's `static int str2signum (char const *signame)`.  */
unsafe fn str2signum (signame: *const c_char) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  if ISDIGIT! (*signame)
    {
      let mut endp: *mut c_char = std::ptr::null_mut ();
      let n: c_long = strtol (signame, &mut endp, 10);
      if *endp == 0 && n <= SIGNUM_BOUND as c_long
        { return n as c_int; }
    }
  else
    {
      let mut i: c_int = 0;
      while i < countof! (numname_table) as c_int
        {
          if streq (numname_table[i as usize].name.as_ptr (), signame)
            { return numname_table[i as usize].num; }
          i += 1;
        }

      {
        let rtmin: c_int = SIGRTMIN;
        let rtmax: c_int = SIGRTMAX;

        if 0 < rtmin && strncmp (signame, c"RTMIN".as_ptr (), 5) == 0
          {
            let mut endp: *mut c_char = std::ptr::null_mut ();
            let n: c_long = strtol (signame.add (5), &mut endp, 10);
            if *endp == 0 && 0 <= n && n <= (rtmax - rtmin) as c_long
              { return rtmin + n as c_int; }
          }
        else if 0 < rtmax && strncmp (signame, c"RTMAX".as_ptr (), 5) == 0
          {
            let mut endp: *mut c_char = std::ptr::null_mut ();
            let n: c_long = strtol (signame.add (5), &mut endp, 10);
            if *endp == 0 && (rtmin - rtmax) as c_long <= n && n <= 0
              { return rtmax + n as c_int; }
          }
      }
    }

  return -1;

  }
}

/* Convert the signal name SIGNAME to the signal number *SIGNUM.
   Return 0 if successful, -1 otherwise.  */

/* C 320-326's `int str2sig (char const *signame, int *signum)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn str2sig (signame: *const c_char, signum: *mut c_int) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  *signum = str2signum (signame);
  return if *signum < 0 { -1 } else { 0 };

  }
}

/* Convert SIGNUM to a signal name in SIGNAME.  SIGNAME must point to
   a buffer of at least SIG2STR_MAX bytes.  Return 0 if successful, -1
   otherwise.  */

/* C 331-364's `int sig2str (int signum, char *signame)`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sig2str (signum: c_int, signame: *mut c_char) -> c_int
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  let mut i: c_int = 0;
  while i < countof! (numname_table) as c_int
    {
      if numname_table[i as usize].num == signum
        {
          strcpy (signame, numname_table[i as usize].name.as_ptr ());
          return 0;
        }
      i += 1;
    }

  {
    let rtmin: c_int = SIGRTMIN;
    let rtmax: c_int = SIGRTMAX;

    if ! (rtmin <= signum && signum <= rtmax)
      { return -1; }

    let base: c_int;
    if signum <= rtmin + (rtmax - rtmin) / 2
      {
        strcpy (signame, c"RTMIN".as_ptr ());
        base = rtmin;
      }
    else
      {
        strcpy (signame, c"RTMAX".as_ptr ());
        base = rtmax;
      }

    let delta: c_int = signum - base;
    if delta != 0
      { sprintf (signame.add (5), c"%+d".as_ptr (), delta); }
    return 0;
  }

  }
}
