// femacs: port of lib/tempname.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// NOTE: lib/tempname.c is **not** in `docs/baseline/config/cond-eval-verified.txt`, so the
// gate reads *both* arms of every `#if` as live text (tools/cinventory.py's blind
// behaviour, P-66).  The frozen build does compile the file - `build/emacs-ref/lib/tempname.o`
// exists - and its object defines `_gen_tempname`, `_gen_tempname_len`, `_try_tempname`,
// `_try_tempname_len` and calls `_clock_gettime`, `_getrandom`, `_mkdir`, `_open`, `_rpl_lstat`,
// `_strlen`, `_strspn`, so the `#if !_LIBC` arms of C 18-21 and C 50-58 (the `struct_stat64` =
// `struct stat`, `__gen_tempname` = `gen_tempname`, `__mkdir` = `mkdir`, `__open` = `open`,
// `__lstat64_time64` = `lstat`, `__getrandom` = `getrandom`, `__clock_gettime64` =
// `clock_gettime`, `__timespec64` = `timespec` aliases) are the ones the frozen build uses;
// C 49's `_LIBC` arm (`struct __stat64_t64`) and C 122-125's `#if _LIBC` forward declaration
// are named at their sites.  The one place where the blind reading adds live text is C 109-116
// in `random_bits`: `CLOCK_REALTIME` is defined and `HAVE_CLOCK_GETTIME` is 1, so C 110-113 is
// the arm the build uses and C 115 (`v = mix_random_values (v, clock ());`) is the non-frozen
// side - the port keeps it as `#[cfg]` text (P-51).
//
// `__gen_tempname` is both a macro (C 52: `gen_tempname`) and the function's spelling at
// C 271-275: C's preprocessor renames the definition to `gen_tempname`, which is what the
// object shows, so the port keeps C's name on the Rust item and emits the real symbol with
// `#[unsafe(export_name = "gen_tempname")]` (PORTING.md 10.13, the mempcpy precedent).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Copyright (C) 1991-2026 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

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

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/errno.rs");
include!("include/stdio.rs");
include!("include/stdlib.rs");
include!("include/string.rs");
include!("include/fcntl.in.rs");
include!("include/stdint.in.rs");
include!("include/sys_random.in.rs");
include!("include/unistd.in.rs");
include!("include/sys_stat.in.rs");
include!("include/sys_types.in.rs");
include!("include/time.in.rs");
include!("include/tempname.rs");

/* C 18-21: `_LIBC` is undefined, so `# include <libc-config.h>` and
   `# include "tempname.h"` are live.  libc-config.h contributes `__set_errno` (below, the
   canonicalize-lgpl.rs shape - that header has no mirror of its own yet) and tempname.h the
   mirror above.  C 23-46's <errno.h>, <stdio.h>, <stdlib.h>, <string.h>, <fcntl.h>,
   <stdint.h>, <sys/random.h>, <sys/stat.h> and <time.h> are the mirrors above
   (<stdarg.h> has no counterpart: Rust's C-variadic `...` is the list, P-04).  */

/* C 26-28's `#ifndef TMP_MAX / # define TMP_MAX 238328`: this target's <stdio.h> does not
   define TMP_MAX (the frozen table has no row for it), so the fallback is live.  */
pub const TMP_MAX: c_uint = 238328;

/* C 29-33's `#ifndef __GT_FILE` fallback: `__GT_FILE` is a glibc-internal name this target
   does not define, so the three constants are live.  C 34-37's `#error` arm compares them with
   tempname.h's `GT_*` (the same values, which is why it is silent).  */
pub const __GT_FILE: c_int = 0;
pub const __GT_DIR: c_int = 1;
pub const __GT_NOCREATE: c_int = 2;

/* <libc-config.h> 46's `# define __set_errno(val) (errno = (val))`.  */
macro_rules! __set_errno {
  ($val:expr) => { *errno () = ($val) };
}

/* C 48-59's `#if _LIBC` / `#else` pair.  The `#else` arm is the frozen build's (see the
   header comment); the `_LIBC` arm's two spellings are named here: C 49's
   `# define struct_stat64 struct __stat64_t64` (glibc's internal type, which this target has
   not got) and C 55's `# define __lstat64_time64(file, buf) __lstat64_time64 (file, buf)`
   sibling - C 49's type is the one the gate would compare, and it is the arm C never
   compiles here.

   C 51-58's seven aliases, each kept as the item C's preprocessor produces:
   `struct_stat64` is a type macro (the strtoimax.rs `Int!` shape), the four function-shaped
   ones are `macro_rules!` twins with C's call shape, and `__timespec64` is the type macro
   C's `struct __timespec64 tv;` needs.  */
macro_rules! struct_stat64 { () => { stat } }
macro_rules! __mkdir {
  ($path:expr, $mode:expr) => { mkdir ($path, $mode) };
}
macro_rules! __open {
  ($path:expr, $flags:expr, $mode:expr) => { open ($path, $flags, $mode) };
}
macro_rules! __lstat64_time64 {
  ($file:expr, $buf:expr) => { system::lstat ($file, $buf) };
}
macro_rules! __getrandom {
  ($buf:expr, $len:expr, $flags:expr) => { getrandom ($buf, $len, $flags) };
}
macro_rules! __clock_gettime64 {
  ($clock_id:expr, $tp:expr) => { clock_gettime ($clock_id, $tp) };
}
macro_rules! __timespec64 { () => { timespec } }

/* <time.h>'s `clock`, needed only by C 115's non-frozen arm below (gettime.rs declares the
   same two entries in-unit, with the measured values; `clock_gettime` and `CLOCK_REALTIME`
   are declared here for the live arm).  */
pub const CLOCK_REALTIME: c_int = 0;

unsafe extern "C" {
  /* int clock_gettime (clockid_t clock_id, struct timespec *tp);  */
  pub fn clock_gettime (clock_id: c_int, tp: *mut timespec) -> c_int;
  /* clock_t clock (void);  */
  pub fn clock () -> c_long;
}

/* Use getrandom if it works, falling back on a 64-bit linear
   congruential generator that starts with Var's value
   mixed in with a clock's low-order bits if available.  */
pub type random_value = u64;
/* C 65's `#define RANDOM_VALUE_MAX UINT_FAST64_MAX`.  */
pub const RANDOM_VALUE_MAX: random_value = UINT_FAST64_MAX;
/* C 66's `#define BASE_62_DIGITS 10 /* 62**10 < UINT_FAST64_MAX */`.  */
pub const BASE_62_DIGITS: c_int = 10;
/* C 67's `#define BASE_62_POWER (62LL * 62 * ... )`.  */
pub const BASE_62_POWER: random_value =
  62 * 62 * 62 * 62 * 62 * 62 * 62 * 62 * 62 * 62;

/* Return the result of mixing the entropy from R and S.
   Assume that R and S are not particularly random,
   and that the result should look randomish to an untrained eye.  */

unsafe extern "C" fn mix_random_values (r: random_value, s: random_value) -> random_value
{
  /* As this code is used only when high-quality randomness is neither
     available nor necessary, there is no need for fancier polynomials
     such as those in the Linux kernel's 'random' driver.  */
  return (2862933555777941757 * r + 3037000493) ^ s;
}

/* Set *R to a random value.
   Return true if *R is set to high-quality value taken from getrandom.
   Otherwise return false, falling back to a low-quality *R that might
   depend on S.

   This function returns false only when getrandom fails.
   On GNU systems this should happen only early in the boot process,
   when the fallback should be good enough for programs using tempname
   because any attacker likely has root privileges already.  */

unsafe extern "C" fn random_bits (r: *mut random_value, s: random_value) -> bool
{
  /* Without GRND_NONBLOCK it can be blocked for minutes on some systems.  */
  if __getrandom! (r as *mut c_void, std::mem::size_of_val (&*r),
                   GRND_NONBLOCK as c_uint) == std::mem::size_of_val (&*r) as ssize_t
    { return true; }

  /* If getrandom did not work, use ersatz entropy based on low-order
     clock bits.  On GNU systems getrandom should fail only
     early in booting, when ersatz should be good enough.
     Do not use ASLR-based entropy, as that would leak ASLR info into
     the resulting file name which is typically public.

     Of course we are in a state of sin here.  */

  let mut v: random_value = s;

  /* C 109's `#if _LIBC || (defined CLOCK_REALTIME && HAVE_CLOCK_GETTIME)` arm is the live
     one (both hold here).  */
  {
    let mut tv: __timespec64! () = std::mem::zeroed ();
    __clock_gettime64! (CLOCK_REALTIME, &mut tv);
    v = mix_random_values (v, tv.tv_sec as random_value);
    v = mix_random_values (v, tv.tv_nsec as random_value);
  }
  /* C 114-116's `#else` arm (`v = mix_random_values (v, clock ());`) is the non-frozen side
     and is kept as `#[cfg]` text: the file is not on the verified list, so the gate reads it
     as live text (P-51).  */
  #[cfg (feature = "no_clock_gettime")]
  {
    v = mix_random_values (v, clock () as random_value);
  }

  *r = v;
  return false;
}

/* C 122-125's `#if _LIBC static int try_tempname_len (…);` forward declaration has no Rust
   counterpart: the definition below is the single definition point (PORTING.md 0.1-12).  */

/* C 127-135's `try_file`.  */
unsafe extern "C" fn try_file (tmpl: *mut c_char, flags: *mut c_void) -> c_int
{
  let openflags: *mut c_int = flags as *mut c_int;
  return __open! (tmpl,
                  (*openflags & !O_ACCMODE) | O_RDWR | O_CREAT | O_EXCL,
                  (S_IRUSR | S_IWUSR) as c_uint);
}

/* C 136-141's `try_dir` (`_GL_UNUSED void *flags` is Rust's unused parameter).  */
unsafe extern "C" fn try_dir (tmpl: *mut c_char, flags: *mut c_void) -> c_int
{
  return __mkdir! (tmpl, (S_IRUSR | S_IWUSR | S_IXUSR) as mode_t);
}

/* C 142-150's `try_nocreate`.  */
unsafe extern "C" fn try_nocreate (tmpl: *mut c_char, flags: *mut c_void) -> c_int
{
  let mut st: struct_stat64! () = std::mem::zeroed ();

  if __lstat64_time64! (tmpl, &mut st) == 0 || *errno () == EOVERFLOW
    { __set_errno! (EEXIST); }
  return if *errno () == ENOENT { 0 } else { -1 };
}

/* These are the characters used in temporary file names.  */
static letters: [c_char; 62] =
  [b'a' as c_char, b'b' as c_char, b'c' as c_char, b'd' as c_char, b'e' as c_char,
   b'f' as c_char, b'g' as c_char, b'h' as c_char, b'i' as c_char, b'j' as c_char,
   b'k' as c_char, b'l' as c_char, b'm' as c_char, b'n' as c_char, b'o' as c_char,
   b'p' as c_char, b'q' as c_char, b'r' as c_char, b's' as c_char, b't' as c_char,
   b'u' as c_char, b'v' as c_char, b'w' as c_char, b'x' as c_char, b'y' as c_char,
   b'z' as c_char, b'A' as c_char, b'B' as c_char, b'C' as c_char, b'D' as c_char,
   b'E' as c_char, b'F' as c_char, b'G' as c_char, b'H' as c_char, b'I' as c_char,
   b'J' as c_char, b'K' as c_char, b'L' as c_char, b'M' as c_char, b'N' as c_char,
   b'O' as c_char, b'P' as c_char, b'Q' as c_char, b'R' as c_char, b'S' as c_char,
   b'T' as c_char, b'U' as c_char, b'V' as c_char, b'W' as c_char, b'X' as c_char,
   b'Y' as c_char, b'Z' as c_char, b'0' as c_char, b'1' as c_char, b'2' as c_char,
   b'3' as c_char, b'4' as c_char, b'5' as c_char, b'6' as c_char, b'7' as c_char,
   b'8' as c_char, b'9' as c_char];

/* Generate a temporary file name based on TMPL.  TMPL must match the
   rules for mk[s]temp (i.e., end in at least X_SUFFIX_LEN "X"s,
   possibly with a suffix).
   The name constructed does not exist at the time of the call to
   this function.  TMPL is overwritten with the result.

   KIND may be one of:
   __GT_NOCREATE:       simply verify that the name does not exist
                        at the time of the call.
   __GT_FILE:           create the file using open(O_CREAT|O_EXCL)
                        and return a read-write fd.  The file is mode 0600.
   __GT_DIR:            create a directory, which will be mode 0700.

   */
/* C 170-175's `#ifdef _LIBC static #endif int gen_tempname_len (…)`: `_LIBC` is undefined, so
   the definition is non-static and the object exports `gen_tempname_len` (nm evidence).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gen_tempname_len (tmpl: *mut c_char, suffixlen: c_int,
                                           mut flags: c_int, kind: c_int, x_suffix_len: usize)
  -> c_int
{
  /* C 177-182's `static int (*const tryfunc[]) (char *, void *)` with C99 designated
     initializers `[__GT_FILE] = try_file`, `[__GT_DIR] = try_dir`, `[__GT_NOCREATE] =
     try_nocreate`.  Rust has no designated initializers, so the port fills the three slots by
     their `__GT_*` index in a const block - the same entries, in the same positions.  */
  static tryfunc: [unsafe extern "C" fn (*mut c_char, *mut c_void) -> c_int; 3] = {
    let mut t: [unsafe extern "C" fn (*mut c_char, *mut c_void) -> c_int; 3] =
      [try_file; 3];
    t[__GT_FILE as usize] = try_file;
    t[__GT_DIR as usize] = try_dir;
    t[__GT_NOCREATE as usize] = try_nocreate;
    t
  };
  /* C's `&flags` is the address of the parameter itself (C 183).  */
  return try_tempname_len (tmpl, suffixlen, &raw mut flags as *mut c_void,
                           tryfunc[kind as usize], x_suffix_len);
}

/* C 187-269's `try_tempname_len` (`#ifdef _LIBC static #endif` again: the object exports
   `try_tempname_len`).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn try_tempname_len (tmpl: *mut c_char, suffixlen: c_int,
                                           args: *mut c_void,
                                           tryfunc: unsafe extern "C" fn (*mut c_char,
                                                                           *mut c_void)
                                                     -> c_int,
                                           x_suffix_len: usize) -> c_int
{
  let saved_errno: c_int = *errno ();

  /* A lower bound on the number of temporary files to attempt to
     generate.  The maximum total number of temporary file names that
     can exist for a given template is 62**6.  It should never be
     necessary to try all of these combinations.  Instead if a reasonable
     number of names is tried (we define reasonable as 62**3) fail to
     give the system administrator the chance to remove the problems.
     This value requires that X_SUFFIX_LEN be at least 3.  */
  macro_rules! ATTEMPTS_MIN { () => { (62 * 62 * 62) }; }

  /* The number of times to attempt to generate a temporary file.  To
     conform to POSIX, this must be no smaller than TMP_MAX.  */
  let attempts: c_uint = if ATTEMPTS_MIN! () < TMP_MAX { TMP_MAX } else { ATTEMPTS_MIN! () };

  /* A random variable.  */
  let mut v: random_value = 0;

  /* A value derived from the random variable, and how many random
     base-62 digits can currently be extracted from VDIGBUF.  */
  /* C declares `random_value vdigbuf;` and fills it in below only when VDIGITS runs out;
     Rust needs a value before the binding can be read, so it starts at C's zero for the
     type (PORTING.md 0.1-13).  */
  let mut vdigbuf: random_value = 0;
  let mut vdigits: c_int = 0;

  /* Least biased value for V.  If V is less than this, V can generate
     BASE_62_DIGITS unbiased digits.  Otherwise the digits are biased.  */
  let biased_min: random_value = RANDOM_VALUE_MAX - RANDOM_VALUE_MAX % BASE_62_POWER;

  let len: usize = strlen (tmpl);
  if len < x_suffix_len + suffixlen as usize
      || strspn (tmpl.add (len - x_suffix_len - suffixlen as usize),
                 c"X".as_ptr ()) < x_suffix_len
    {
      __set_errno! (EINVAL);
      return -1;
    }

  /* This is where the Xs start.  */
  let xxxxxx: *mut c_char = tmpl.add (len - x_suffix_len - suffixlen as usize);

  let mut count: c_uint = 0;
  while count < attempts
    {
      let mut i: usize = 0;
      while i < x_suffix_len
        {
          if vdigits == 0
            {
              /* Worry about bias only if the bits are high quality.  */
              while random_bits (&raw mut v, v) && biased_min <= v
                { continue; }

              vdigbuf = v;
              vdigits = BASE_62_DIGITS;
            }

          *xxxxxx.add (i) = letters[(vdigbuf % 62) as usize];
          vdigbuf /= 62;
          vdigits -= 1;
          i += 1;
        }

      let fd: c_int = tryfunc (tmpl, args);
      if fd >= 0
        {
          __set_errno! (saved_errno);
          return fd;
        }
      else if *errno () != EEXIST
        { return -1; }
      count += 1;
    }

  /* We got out of the loop because we ran out of combinations to try.  */
  __set_errno! (EEXIST);
  return -1;
}

/* C 52's `# define __gen_tempname gen_tempname`: the macro the definition below is spelled
   with.  Rust has no preprocessor, so the port keeps the rename as a `macro_rules!` twin with
   C's role (it expands to the *name* `gen_tempname`), and the function below keeps C's source
   spelling with `#[unsafe(export_name = "gen_tempname")]` so the object emits what C's
   preprocessor produces.  */
macro_rules! __gen_tempname {
  () => { gen_tempname };
}

/* C 271-275's `int __gen_tempname (char *tmpl, int suffixlen, int flags, int kind)`: the
   `# define __gen_tempname gen_tempname` of C 52 renames the definition, which is why the
   reference object exports `_gen_tempname` and not `___gen_tempname`.  */
#[unsafe(export_name = "gen_tempname")]
pub unsafe extern "C" fn __gen_tempname (tmpl: *mut c_char, suffixlen: c_int, flags: c_int,
                                         kind: c_int) -> c_int
{
  return gen_tempname_len (tmpl, suffixlen, flags, kind, 6);
}

/* C 277-284's `#if !_LIBC int try_tempname (…)`: live, and the object exports it.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn try_tempname (tmpl: *mut c_char, suffixlen: c_int, args: *mut c_void,
                                       tryfunc: unsafe extern "C" fn (*mut c_char, *mut c_void)
                                                 -> c_int) -> c_int
{
  return try_tempname_len (tmpl, suffixlen, args, tryfunc, 6);
}

/* C 35's `return open (filename, flags, mode);`-style entry the file's own macros name: the
   native `lstat` under gnulib's renamed symbol (PORTING.md 10.13; the utimens.rs `mod system`
   shape, so the name `lstat` stays out of the crate root).  */
mod system {
  use super::*;

  unsafe extern "C" {
    #[link_name = "rpl_lstat"]
    pub fn lstat (file: *const c_char, buf: *mut stat) -> c_int;
  }
}
