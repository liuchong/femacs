// femacs: port of lib/fcntl.c @ fd7afced403.
//
// lib/fcntl.c is in `docs/baseline/config/cond-eval-verified.txt`, so the gate evaluates its
// 66 conditions against the frozen configuration and compares only the live text.  The live
// functions are C 200-441's `fcntl` (the variadic replacement), C 443-482's `rpl_fcntl_DUPFD`
// and C 484-541's `rpl_fcntl_DUPFD_CLOEXEC`; inside them `HAVE_FCNTL` is 1,
// `FCNTL_DUPFD_BUGGY`/`REPLACE_FCHDIR` are undefined, `GNULIB_defined_F_DUPFD_CLOEXEC` is 0
// and `__NetBSD__`/`__HAIKU__` are undefined, so the `#if !HAVE_FCNTL` arms, the
// `FCNTL_DUPFD_BUGGY || REPLACE_FCHDIR` arm, the `__NetBSD__ || __HAIKU__` arm, the
// `REPLACE_FCHDIR` block inside the live one and C 431-433's `#else errno = EINVAL` are dead -
// each is named at its site (P-51).  Whole dead regions: C 29-31's and C 545-663's
// `#ifdef __KLIBC__` arms (`klibc_dupdirfd`, `klibc_fcntl`), and C 33-172's
// `#if defined _WIN32` arm with its `dupfd` helper (they need `<windows.h>`, `<io.h>`,
// `<emx/io.h>` and `<InnoTekLIBC/backend.h>`, which this target has not got).  Inside the live
// `fcntl`, the inner `switch`'s per-command `case` labels are the 30 the frozen target's
// <fcntl.h> defines (measured; include/fcntl.in.rs records the probe and the values).
//
// The file is **not compiled** by the frozen build: `REPLACE_FCNTL` is 0 (the generated
// build/emacs-ref/lib/fcntl.h keeps C's `#define fcntl rpl_fcntl` inside `# if 0`), so there is
// no build/emacs-ref/lib/fcntl.o to compare symbols with.  The port keeps C's name and
// `#[unsafe(no_mangle)]` (what the frozen configuration's `int fcntl (...)` definition would
// emit), and reaches the native entry - the file's own comment says the replacement "forwards
// all others on to the native fcntl" - through `mod system` below (the fchmodat.rs precedent);
// docs/ports/fcntl.md records the REPLACE_FCNTL=0/1 analysis for adjudication.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Written by Eric Blake <ebb9@byu.net>.  */

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");
include!("include/limits.in.rs");
include!("include/stdlib.rs");
include!("include/unistd.in.rs");
include!("include/fcntl.in.rs");

/* C 19-29: <config.h>, the specification <fcntl.h>, <errno.h>, <limits.h>, <stdarg.h>,
   <stdlib.h> and <unistd.h> are the mirrors above (<stdarg.h> has no Rust counterpart: the
   `...` parameter is Rust's varargs, P-04).  */

/* The one frozen-configuration value the live text reads that `src/include/config.rs` does not
   carry: gnulib's config.h sets `GNULIB_defined_F_DUPFD_CLOEXEC` to 0
   (docs/baseline/config/darwin-arm64.macros.tsv 599), so this target's <fcntl.h> *does* define
   F_DUPFD_CLOEXEC and C 506's `have_dupfd_cloexec` starts at 0, not -1.  */
pub const GNULIB_defined_F_DUPFD_CLOEXEC: c_int = 0;

/* C 170-175's forward declarations (`static int rpl_fcntl_DUPFD (int, int);` and its
   `_CLOEXEC` twin) have no Rust counterpart: Rust's items are order-independent, so the
   definitions below are the single definition point (PORTING.md 0.1-12).  */

/* C 177-179's `#ifdef __KLIBC__ static int klibc_fcntl (…);` is inside the dead klibc region.  */

/* C 182-199's comment (the action list) is C's; it precedes the definition below.  */

/* C 443-482's `static int rpl_fcntl_DUPFD (int fd, int target)`.  */
unsafe fn rpl_fcntl_DUPFD (fd: c_int, target: c_int) -> c_int
{
  let mut result: c_int;
  /* C 447-448's `#if !HAVE_FCNTL` arm (`result = dupfd (fd, target, 0);`) is dead, and so is
     C 449-477's `#elif FCNTL_DUPFD_BUGGY || REPLACE_FCHDIR` arm (the `getdtablesize` check,
     the F_GETFD/F_DUPFD/F_SETFD dance and its `REPLACE_FCHDIR` tail): both macros are
     undefined.  */
  /* C 478-480's `#else` arm is the live text.  */
  result = system::fcntl (fd, F_DUPFD, target);
  return result;
}

/* C 484-541's `static int rpl_fcntl_DUPFD_CLOEXEC (int fd, int target)`.  */
unsafe fn rpl_fcntl_DUPFD_CLOEXEC (fd: c_int, target: c_int) -> c_int
{
  let mut result: c_int;
  /* C 488-489's `#if !HAVE_FCNTL` arm (`result = dupfd (fd, target, O_CLOEXEC);`) is dead.  */
  /* C 491-497's `# if defined __NetBSD__ || defined __HAIKU__` arm - which `#define`s
     `have_dupfd_cloexec` to -1 instead of declaring the static - is dead: neither macro is
     defined here.  */
  /* C 499-506: try the system call first, if the headers claim it exists (that is, if
     GNULIB_defined_F_DUPFD_CLOEXEC is 0), since we may be running with a glibc that has the
     macro but with an older kernel that does not support it.  Cache the information on
     whether the system call really works, but avoid caching failure if the corresponding
     F_DUPFD fails for any reason.  0 = unknown, 1 = yes, -1 = no.  */
  static mut have_dupfd_cloexec: c_int =
    if GNULIB_defined_F_DUPFD_CLOEXEC != 0 { -1 } else { 0 };
  if 0 <= have_dupfd_cloexec
    {
      result = system::fcntl (fd, F_DUPFD_CLOEXEC, target);
      if 0 <= result || *errno () != EINVAL
        {
          have_dupfd_cloexec = 1;
          /* C 513-516's `#  if REPLACE_FCHDIR` arm (`result = _gl_register_dup (fd, result);`)
             is dead: the macro is undefined.  */
        }
      else
        {
          result = rpl_fcntl_DUPFD (fd, target);
          if result >= 0
            { have_dupfd_cloexec = -1; }
        }
    }
  else
    { result = rpl_fcntl_DUPFD (fd, target); }
  if 0 <= result && have_dupfd_cloexec == -1
    {
      let flags: c_int = system::fcntl (result, F_GETFD);
      if flags < 0 || system::fcntl (result, F_SETFD, flags | FD_CLOEXEC) == -1
        {
          let saved_errno: c_int = *errno ();
          close (result);
          *errno () = saved_errno;
          result = -1;
        }
    }
  /* C 539's `#endif /* HAVE_FCNTL */` closes the live arm.  */
  return result;
}

/* Perform the specified ACTION on the file descriptor FD, possibly
   using the argument ARG further described below.  This replacement
   handles the following actions, and forwards all others on to the
   native fcntl.  An unrecognized ACTION returns -1 with errno set to
   EINVAL.

   F_DUPFD - duplicate FD, with int ARG being the minimum target fd.
   If successful, return the duplicate, which will be inheritable;
   otherwise return -1 and set errno.

   F_DUPFD_CLOEXEC - duplicate FD, with int ARG being the minimum
   target fd.  If successful, return the duplicate, which will not be
   inheritable; otherwise return -1 and set errno.

   F_GETFD - ARG need not be present.  If successful, return a
   non-negative value containing the descriptor flags of FD (only
   FD_CLOEXEC is portable, but other flags may be present); otherwise
   return -1 and set errno.  */

/* C 200-441's `int fcntl (int fd, int action, /* arg */...)`.  C's `va_list arg; va_start (arg,
   action); … va_end (arg);` has no port counterpart: the `...` parameter *is* the list and
   every `va_arg (arg, T)` is `ap.next_arg::<T> ()` (P-04, the open.rs precedent), with the two
   C types recorded in docs/ports/va-arg-types.tsv.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fcntl (fd: c_int, action: c_int, mut ap: ...) -> c_int
{
  let mut result: c_int = -1;

  match action
    {
      F_DUPFD =>
        {
          let target: c_int = ap.next_arg::<c_int> ();
          result = rpl_fcntl_DUPFD (fd, target);
        }

      F_DUPFD_CLOEXEC =>
        {
          let target: c_int = ap.next_arg::<c_int> ();
          result = rpl_fcntl_DUPFD_CLOEXEC (fd, target);
        }

      _ =>
        {
          /* C 228-247's `#if !HAVE_FCNTL` arm - the `case F_GETFD:` that either asks
             `GetHandleInformation` (Windows) or calls `dup2 (fd, fd)` to reject an invalid
             descriptor - is dead: HAVE_FCNTL is 1.  */

          /* Implementing F_SETFD on mingw is not trivial - there is no
             API for changing the O_NOINHERIT bit on an fd, and merely
             changing the HANDLE_FLAG_INHERIT bit on the underlying handle
             can lead to odd state.  It may be possible by duplicating the
             handle, using _open_osfhandle with the right flags, then
             using dup2 to move the duplicate onto the original, but that
             is not supported for now.  */

          /* C 257's `default:` arm, whose block is C 258-435.  C 259's `#if HAVE_FCNTL` is
             live; C 431-433's `#else errno = EINVAL;` is dead.  */
          {
            match action
              {
                /* These actions take no argument.  */
                F_BARRIERFSYNC | F_CHKCLEAN | F_FLUSH_DATA | F_FREEZE_FS | F_FULLFSYNC
                | F_GETFD | F_GETFL | F_GETLEASE | F_GETNOSIGPIPE | F_GETOWN
                | F_GETPROTECTIONCLASS | F_GETPROTECTIONLEVEL | F_THAW_FS
                => { result = system::fcntl (fd, action); }

                /* These actions take an 'int' argument.  */
                F_DUPFD | F_DUPFD_CLOEXEC | F_DUPFD_CLOFORK | F_GLOBAL_NOCACHE | F_NOCACHE
                | F_NODIRECT | F_RDAHEAD | F_SETBACKINGSTORE | F_SETFD | F_SETFL
                | F_SETLEASE | F_SETNOSIGPIPE | F_SETOWN | F_SETPROTECTIONCLASS
                | F_SINGLE_WRITER
                =>
                  {
                    let x: c_int = ap.next_arg::<c_int> ();
                    result = system::fcntl (fd, action, x);
                  }

                /* Other actions take a pointer argument.  */
                _ =>
                  {
                    let p: *mut c_void = ap.next_arg::<*mut c_void> ();
                    result = system::fcntl (fd, action, p);
                  }
              }
          }
        }
    }

  return result;
}

/* C 543's `#undef fcntl` ends the name games the file's own comment (C 170-171) names: after
   it, the body's `fcntl` is the native entry, which is what the port's `system` module
   declares.  A crate-root declaration would be E0428 (the unit defines `fn fcntl`; PORTING.md
   0.1-10), so the declaration lives in a module, the fchmodat.rs/open.rs shape.  */
mod system {
  use super::*;

  unsafe extern "C" {
    /* rustc checks this declaration against the runtime's own `fcntl` (the
       `suspicious_runtime_symbol_definitions` lint warns that the signature must be the
       runtime's `unsafe extern "C" fn(i32, i32, ...) -> i32`), so the port spells C's
       `int fcntl (int, int, ...)` declaration exactly, `...` included.  */
    pub fn fcntl (fd: c_int, action: c_int, ...) -> c_int;
  }
}

/* C 545-663's `#ifdef __KLIBC__` region (`klibc_dupdirfd`, `klibc_fcntl`) is dead: the macro
   is undefined, and its body needs `<emx/io.h>` and `<InnoTekLIBC/backend.h>`, which this
   target has not got (the gettimeofday.c treatment).  */
