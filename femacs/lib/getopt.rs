// femacs: port of lib/getopt.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// lib/getopt.c is in `docs/baseline/config/cond-eval-verified.txt`, so the gate evaluates its
// five conditions against the frozen configuration and compares only the live text.  Live:
// `_LIBC` is undefined (C 22-24's `# include <config.h>` fires), `_POSIX_THREAD_SAFE_FUNCTIONS`
// is 200112L, so C 48-50's empty-macro arm is *dead* and C 283/287 and 300/301's
// `flockfile`/`funlockfile` are the real stdio functions, and C 61's `# else` arm of the
// `#ifdef _LIBC` block is the live text (gettext.h, the `_` macro, the alloca repurposing).
// Dead: C 40-60's `#ifdef _LIBC` renaming block (`fprintf` -> `__fxprintf_nocancel` and the
// two `_IO_*` locks), C 731-733's `#ifdef _LIBC` twin `GETOPT_ENTRY (__posix_getopt, 1)`, and
// C 738-811's `#ifdef TEST` program; each is named at its site (P-51).
//
// The `__GETOPT_PREFIX` layer (`rpl_`, getopt-pfx-core.rs/getopt-pfx-ext.rs) renames the
// symbols this unit defines: `optarg`/`optind`/`opterr`/`optopt` and the `GETOPT_ENTRY`
// expansion `getopt` carry `#[unsafe(export_name = ...)]`, and `_getopt_internal` is
// `rpl_getopt_internal` in the frozen build (measured: `nm build/emacs-ref/lib/getopt.o`
// defines `_rpl_getopt`, `_rpl_getopt_internal`, `_rpl_optarg`, `_rpl_opterr`, `_rpl_optind`,
// `_rpl_optopt` and the unrenamed `__getopt_internal_r`).
//
// Deviation worth a decision (docs/ports/getopt.md): C 45's `_` macro cannot be a Rust
// identifier, so it is recorded in `docs/ports/excluded-macros.txt` and its body - with
// gettext.h's live arm an identity stub, proven by the reference object's missing `dgettext`
// symbol - is written out at the five message sites.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Getopt for GNU.
   Copyright (C) 1987-2026 Free Software Foundation, Inc.
   This file is part of the GNU C Library and is also part of gnulib.
   Patches to this file should be submitted to both projects.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with the GNU C Library; if not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::{c_char, c_double, c_int, c_long, c_schar, c_short, c_uchar, c_uint,
               c_ulong, c_ushort, c_void};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/stdio.rs");
include!("include/stdlib.rs");
include!("include/string.rs");
include!("include/unistd.in.rs");
include!("include/gettext.rs");
include!("include/getopt.in.rs");
include!("include/getopt_int.rs");

/* C 25-33's <getopt.h>, <stdio.h>, <stdlib.h>, <string.h> and <unistd.h> are the mirrors
   above; the entries the live text needs that those mirrors do not carry yet are declared
   here (the fstatat.rs/mini-gmp.rs shape): `fprintf` and the `stderr` object (darwin spells
   the platform symbol `__stderrp`), the two stdio lock entries C 48-50 leaves as real
   functions, and `abort`/`getenv`.  */
unsafe extern "C" {
  /* int fprintf (FILE *stream, const char *format, ...);  */
  pub fn fprintf (stream: *mut FILE, format: *const c_char, ...) -> c_int;
  /* extern FILE *stderr;  */
  #[cfg_attr (target_os = "macos", link_name = "__stderrp")]
  pub static mut stderr: *mut FILE;
  /* void flockfile (FILE *stream);  void funlockfile (FILE *stream);  */
  pub fn flockfile (stream: *mut FILE);
  pub fn funlockfile (stream: *mut FILE);
  /* void abort (void);  */
  pub fn abort () -> !;
  /* char *getenv (const char *name);  */
  pub fn getenv (name: *const c_char) -> *mut c_char;
  /* int strncmp (const char *s1, const char *s2, size_t n);  */
  pub fn strncmp (s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

/* C 40-60's `#ifdef _LIBC` arm is dead: `_LIBC` is undefined in the frozen configuration.
   The `# else` arm is the live text and needs no declaration of its own beyond the mirrors
   above; C 45's `_` macro is the one construct it cannot be spelled as, below.  */

/* C 45's `# define _(msgid) dgettext (GNULIB_TEXT_DOMAIN, msgid)`.  `_` is not a Rust
   identifier (PORTING.md 0.2), so the name is recorded in docs/ports/excluded-macros.txt.
   With gettext.h's live arm, `dgettext` is the inlined identity stub - the reference object
   has no `dgettext` symbol at all (`nm build/emacs-ref/lib/getopt.o`) - so each of the five
   call sites below passes the message pointer itself, exactly what the preprocessor leaves
   there: C 283, 289, 323, 348 and 363.  */

/* C 54's `# define __libc_use_alloca(size) 0`.  */
macro_rules! __libc_use_alloca {
  ($size:expr) => { 0 };
}

/* C 55-56's `# undef alloca` / `# define alloca(size) (abort (), (void *) 0)`: C's comma
   operator yields its second operand, which Rust spells as a block whose last expression is
   the value (the sha1.rs `M` shape).  */
macro_rules! alloca {
  ($size:expr) => { { abort (); 0 as *mut c_void } };
}

/* This implementation of 'getopt' has three modes for handling
   options interspersed with non-option arguments.  It can stop
   scanning for options at the first non-option argument encountered,
   as POSIX specifies.  It can continue scanning for options after the
   first non-option argument, but permute 'argv' as it goes so that,
   after 'getopt' is done, all the options precede all the non-option
   arguments and 'optind' points to the first non-option argument.
   Or, it can report non-option arguments as if they were arguments to
   the option character '\x01'.

   The default behavior of 'getopt_long' is to permute the argument list.
   When this implementation is used standalone, the default behavior of
   'getopt' is to stop at the first non-option argument, but when it is
   used as part of GNU libc it also permutes the argument list.  In both
   cases, setting the environment variable POSIXLY_CORRECT to any value
   disables permutation.

   If the first character of the OPTSTRING argument to 'getopt' or
   'getopt_long' is '+', both functions will stop at the first
   non-option argument.  If it is '-', both functions will report
   non-option arguments as arguments to the option character '\x01'.  */

/* C 89-99: `char *optarg;`

   For communication from 'getopt' to the caller.
   When 'getopt' finds an option that takes an argument,
   the argument value is returned here.
   Also, when 'ordering' is RETURN_IN_ORDER,
   each non-option ARGV-element is returned here.  */

#[unsafe(export_name = "rpl_optarg")]
pub static mut optarg: *mut c_char = std::ptr::null_mut ();

/* C 100-104: `int optind = 1;`

   Index in ARGV of the next element to be scanned.
   This is used for communication to and from the caller
   and for communication between successive calls to 'getopt'.

   On entry to 'getopt', zero means this is the first call; initialize.

   When 'getopt' returns -1, this is the index of the first of the
   non-option elements that the caller should itself scan.

   Otherwise, 'optind' communicates from one call to the next
   how much of ARGV has been scanned so far.

   1003.2 says this must be 1 before any call.  */

#[unsafe(export_name = "rpl_optind")]
pub static mut optind: c_int = 1;

/* C 106-109: `int opterr = 1;`

   Callers store zero here to inhibit the error message
   for unrecognized options.  */

#[unsafe(export_name = "rpl_opterr")]
pub static mut opterr: c_int = 1;

/* C 111-115: `int optopt = '?';`

   Set to an option character which was unrecognized.
   This must be initialized on some systems to avoid linking in the
   system's own getopt implementation.  */

#[unsafe(export_name = "rpl_optopt")]
pub static mut optopt: c_int = '?' as c_int;

/* C 117-119: `static struct _getopt_data getopt_data;`

   Keep a global copy of all internal members of getopt_data.  C zero-initialises the static;
   the port writes that value out (PORTING.md 0.1-13).  */

static mut getopt_data: _getopt_data = _getopt_data {
  optind: 0,
  opterr: 0,
  optopt: 0,
  optarg: std::ptr::null_mut (),
  __initialized: 0,
  __nextchar: std::ptr::null_mut (),
  __ordering: __ord::REQUIRE_ORDER,
  __first_nonopt: 0,
  __last_nonopt: 0,
};

/* Exchange two adjacent subsequences of ARGV.
   One subsequence is elements [first_nonopt,last_nonopt)
   which contains all the non-options that have been skipped so far.
   The other is elements [last_nonopt,optind), which contains all
   the options processed since those non-options were skipped.

   'first_nonopt' and 'last_nonopt' are relocated so that they describe
   the new indices of the non-options in ARGV after they are moved.  */

unsafe fn exchange (argv: *mut *mut c_char, d: *mut _getopt_data)
{
  let mut bottom: c_int = (*d).__first_nonopt;
  let mut middle: c_int = (*d).__last_nonopt;
  let mut top: c_int = (*d).optind;

  /* Exchange the shorter segment with the far end of the longer segment.
     That puts the shorter segment into the right place.
     It leaves the longer segment in the right place overall,
     but it consists of two parts that need to be swapped next.  */

  while top > middle && middle > bottom
    {
      if top - middle > middle - bottom
        {
          /* Bottom segment is the short one.  */
          let len: c_int = middle - bottom;
          let mut i: c_int;

          /* Swap it with the top part of the top segment.  */
          i = 0;
          while i < len
            {
              let tem: *mut c_char = *argv.offset ((bottom + i) as isize);
              *argv.offset ((bottom + i) as isize)
                = *argv.offset ((top - (middle - bottom) + i) as isize);
              *argv.offset ((top - (middle - bottom) + i) as isize) = tem;
              i += 1;
            }
          /* Exclude the moved bottom segment from further swapping.  */
          top -= len;
        }
      else
        {
          /* Top segment is the short one.  */
          let len: c_int = top - middle;
          let mut i: c_int;

          /* Swap it with the bottom part of the bottom segment.  */
          i = 0;
          while i < len
            {
              let tem: *mut c_char = *argv.offset ((bottom + i) as isize);
              *argv.offset ((bottom + i) as isize) = *argv.offset ((middle + i) as isize);
              *argv.offset ((middle + i) as isize) = tem;
              i += 1;
            }
          /* Exclude the moved top segment from further swapping.  */
          bottom += len;
        }
    }

  /* Update records for the slots the non-options now occupy.  */

  (*d).__first_nonopt += (*d).optind - (*d).__last_nonopt;
  (*d).__last_nonopt = (*d).optind;
}

/* Process the argument starting with d->__nextchar as a long option.
   d->optind should *not* have been advanced over this argument.

   If the value returned is -1, it was not actually a long option, the
   state is unchanged, and the argument should be processed as a set
   of short options (this can only happen when long_only is true).
   Otherwise, the option (and its argument, if any) have been consumed
   and the return value is the value to return from _getopt_internal_r.  */
unsafe fn process_long_option (argc: c_int, argv: *mut *mut c_char, optstring: *const c_char,
                               longopts: *const option, longind: *mut c_int, long_only: c_int,
                               d: *mut _getopt_data, print_errors: c_int,
                               prefix: *const c_char) -> c_int
{
  let mut nameend: *mut c_char;
  nameend = (*d).__nextchar;
  while *nameend != 0 && *nameend != '=' as c_char
    { nameend = nameend.add (1); }
    /* Do nothing.  */ ;

  let namelen: usize = nameend.offset_from ((*d).__nextchar) as usize;

  /* First look for an exact match, counting the options as a side
     effect.  */
  let mut pfound: *const option = std::ptr::null ();
  let mut n_options: c_int;
  /* C declares `int option_index;` and fills it in below; Rust needs a value before the
     binding can be read, so it starts at C's zero for the type (PORTING.md 0.1-13).  */
  let mut option_index: c_int = 0;
  {
    let mut p: *const option;
    p = longopts;
    n_options = 0;
    while !(*p).name.is_null ()
      {
        if strncmp ((*p).name, (*d).__nextchar, namelen) == 0
          && namelen == strlen ((*p).name)
          {
            /* Exact match found.  */
            pfound = p;
            option_index = n_options;
            break;
          }
        p = p.offset (1);
        n_options += 1;
      }
  }

  if pfound.is_null ()
    {
      /* Didn't find an exact match, so look for abbreviations.  */
      let mut ambig_set: *mut c_uchar = std::ptr::null_mut ();
      /* Use simpler fallback diagnostic if ambig_set == &ambig_fallback.  */
      let mut ambig_fallback: c_uchar = 0;
      let mut ambig_malloced: *mut c_void = std::ptr::null_mut ();
      let mut indfound: c_int = -1;
      let mut p: *const option;

      p = longopts;
      option_index = 0;
      while !(*p).name.is_null ()
        {
          if strncmp ((*p).name, (*d).__nextchar, namelen) == 0
            {
              if pfound.is_null ()
                {
                  /* First nonexact match found.  */
                  pfound = p;
                  indfound = option_index;
                }
              else if long_only != 0
                       || (*pfound).has_arg != (*p).has_arg
                       || (*pfound).flag != (*p).flag
                       || (*pfound).val != (*p).val
                {
                  /* Second or later nonexact match found.  */
                  if ambig_set != &raw mut ambig_fallback
                    {
                      if print_errors == 0
                        /* Don't waste effort tracking the ambig set if
                           we're not going to print it anyway.  */
                        { ambig_set = &raw mut ambig_fallback; }
                      else if ambig_set.is_null ()
                        {
                          if __libc_use_alloca! (n_options) != 0
                            { ambig_set = alloca! (n_options) as *mut c_uchar; }
                          else
                            {
                              ambig_malloced = malloc (n_options as usize);
                              /* Fall back to simpler diagnostic if
                                 memory allocation fails.  */
                              ambig_set = (if !ambig_malloced.is_null ()
                                           { ambig_malloced as *mut c_uchar }
                                           else
                                           { &raw mut ambig_fallback });
                            }

                          if ambig_set != &raw mut ambig_fallback
                            {
                              memset (ambig_set as *mut c_void, 0, n_options as usize);
                              *ambig_set.offset (indfound as isize) = 1;
                            }
                        }
                      if !ambig_set.is_null () && ambig_set != &raw mut ambig_fallback
                        { *ambig_set.offset (option_index as isize) = 1; }
                    }
                }
            }
          p = p.offset (1);
          option_index += 1;
        }

      if !ambig_set.is_null ()
        {
          if print_errors != 0
            {
              if ambig_set == &raw mut ambig_fallback
                { fprintf (stderr, c"%s: option '%s%s' is ambiguous\n".as_ptr (),
                           *argv, prefix, (*d).__nextchar); }
              else
                {
                  flockfile (stderr);
                  fprintf (stderr,
                           c"%s: option '%s%s' is ambiguous; possibilities:".as_ptr (),
                           *argv, prefix, (*d).__nextchar);

                  option_index = 0;
                  while option_index < n_options
                    {
                      if *ambig_set.offset (option_index as isize) != 0
                        { fprintf (stderr, c" '%s%s'".as_ptr (),
                                   prefix, (*longopts.offset (option_index as isize)).name); }
                      option_index += 1;
                    }

                  /* This must use 'fprintf' even though it's only
                     printing a single character, so that it goes through
                     __fxprintf_nocancel when compiled as part of glibc.  */
                  fprintf (stderr, c"\n".as_ptr ());
                  funlockfile (stderr);
                }
            }
          free (ambig_malloced);
          (*d).__nextchar = (*d).__nextchar.add (strlen ((*d).__nextchar));
          (*d).optind += 1;
          (*d).optopt = 0;
          return '?' as c_int;
        }

      option_index = indfound;
    }

  if pfound.is_null ()
    {
      /* Can't find it as a long option.  If this is not getopt_long_only,
         or the option starts with '--' or is not a valid short option,
         then it's an error.  */
      if long_only == 0 || *(*argv.offset ((*d).optind as isize)).offset (1) == b'-' as c_char
        || strchr (optstring, *(*d).__nextchar as c_int).is_null ()
        {
          if print_errors != 0
            { fprintf (stderr, c"%s: unrecognized option '%s%s'\n".as_ptr (),
                       *argv, prefix, (*d).__nextchar); }

          (*d).__nextchar = std::ptr::null_mut ();
          (*d).optind += 1;
          (*d).optopt = 0;
          return '?' as c_int;
        }

      /* Otherwise interpret it as a short option.  */
      return -1;
    }

  /* We have found a matching long option.  Consume it.  */
  (*d).optind += 1;
  (*d).__nextchar = std::ptr::null_mut ();
  if *nameend != 0
    {
      /* Don't test has_arg with >, because some C compilers don't
         allow it to be used on enums.  */
      if (*pfound).has_arg != 0
        { (*d).optarg = nameend.add (1); }
      else
        {
          if print_errors != 0
            { fprintf (stderr, c"%s: option '%s%s' doesn't allow an argument\n".as_ptr (),
                       *argv, prefix, (*pfound).name); }

          (*d).optopt = (*pfound).val;
          return '?' as c_int;
        }
    }
  else if (*pfound).has_arg == 1
    {
      if (*d).optind < argc
        {
          (*d).optarg = *argv.offset ((*d).optind as isize);
          (*d).optind += 1;
        }
      else
        {
          if print_errors != 0
            { fprintf (stderr, c"%s: option '%s%s' requires an argument\n".as_ptr (),
                       *argv, prefix, (*pfound).name); }

          (*d).optopt = (*pfound).val;
          return if *optstring == b':' as c_char { ':' as c_int } else { '?' as c_int };
        }
    }

  if !longind.is_null ()
    { *longind = option_index; }
  if !(*pfound).flag.is_null ()
    {
      *(*pfound).flag = (*pfound).val;
      return 0;
    }
  return (*pfound).val;
}

/* Initialize internal data upon the first call to getopt.  */

unsafe fn _getopt_initialize (argc: c_int, argv: *mut *mut c_char, optstring: *const c_char,
                              d: *mut _getopt_data, posixly_correct: c_int) -> *const c_char
{
  let mut optstring = optstring;

  /* Start processing options with ARGV-element 1 (since ARGV-element 0
     is the program name); the sequence of previously skipped
     non-option ARGV-elements is empty.  */
  if (*d).optind == 0
    { (*d).optind = 1; }

  (*d).__first_nonopt = (*d).optind;
  (*d).__last_nonopt = (*d).optind;
  (*d).__nextchar = std::ptr::null_mut ();

  /* Determine how to handle the ordering of options and nonoptions.  */
  if *optstring == b'-' as c_char
    {
      (*d).__ordering = __ord::RETURN_IN_ORDER;
      optstring = optstring.add (1);
    }
  else if *optstring == b'+' as c_char
    {
      (*d).__ordering = __ord::REQUIRE_ORDER;
      optstring = optstring.add (1);
    }
  else if posixly_correct != 0 || !getenv (c"POSIXLY_CORRECT".as_ptr ()).is_null ()
    { (*d).__ordering = __ord::REQUIRE_ORDER; }
  else
    { (*d).__ordering = __ord::PERMUTE; }

  (*d).__initialized = 1;
  return optstring;
}

/* Scan elements of ARGV (whose length is ARGC) for option characters
   given in OPTSTRING.

   If an element of ARGV starts with '-', and is not exactly "-" or "--",
   then it is an option element.  The characters of this element
   (aside from the initial '-') are option characters.  If 'getopt'
   is called repeatedly, it returns successively each of the option characters
   from each of the option elements.

   If 'getopt' finds another option character, it returns that character,
   updating 'optind' and 'nextchar' so that the next call to 'getopt' can
   resume the scan with the following option character or ARGV-element.

   If there are no more option characters, 'getopt' returns -1.
   Then 'optind' is the index in ARGV of the first ARGV-element
   that is not an option.  (The ARGV-elements have been permuted
   so that those that are not options now come last.)

   OPTSTRING is a string containing the legitimate option characters.
   If an option character is seen that is not listed in OPTSTRING,
   return '?' after printing an error message.  If you set 'opterr' to
   zero, the error message is suppressed but we still return '?'.

   If a char in OPTSTRING is followed by a colon, that means it wants an arg,
   so the following text in the same ARGV-element, or the text of the following
   ARGV-element, is returned in 'optarg'.  Two colons mean an option that
   wants an optional arg; if there is text in the current ARGV-element,
   it is returned in 'optarg', otherwise 'optarg' is set to zero.

   If OPTSTRING starts with '-' or '+', it requests different methods of
   handling the non-option ARGV-elements.
   See the comments about RETURN_IN_ORDER and REQUIRE_ORDER, above.

   Long-named options begin with '--' instead of '-'.
   Their names may be abbreviated as long as the abbreviation is unique
   or is an exact match for some defined option.  If they have an
   argument, it follows the option name in the same ARGV-element, separated
   from the option name by a '=', or else the in next ARGV-element.
   When 'getopt' finds a long-named option, it returns 0 if that option's
   'flag' field is nonzero, the value of the option's 'val' field
   if the 'flag' field is zero.

   The elements of ARGV aren't really const, because we permute them.
   But we pretend they're const in the prototype to be compatible
   with other systems.

   LONGOPTS is a vector of 'struct option' terminated by an
   element containing a name which is zero.

   LONGIND returns the index in LONGOPT of the long-named option found.
   It is only valid when a long-named option has been found by the most
   recent call.

   If LONG_ONLY is nonzero, '-' as well as '--' can introduce
   long-named options.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _getopt_internal_r (argc: c_int, argv: *mut *mut c_char,
                                             optstring: *const c_char, longopts: *const option,
                                             longind: *mut c_int, long_only: c_int,
                                             d: *mut _getopt_data, posixly_correct: c_int)
  -> c_int
{
  if argc < 1
    { return -1; }

  let mut print_errors: c_int = (*d).opterr;

  (*d).optarg = std::ptr::null_mut ();

  let mut optstring = optstring;

  if (*d).optind == 0 || (*d).__initialized == 0
    { optstring = _getopt_initialize (argc, argv, optstring, d, posixly_correct); }
  else if *optstring == b'-' as c_char || *optstring == b'+' as c_char
    { optstring = optstring.add (1); }

  if *optstring == b':' as c_char
    { print_errors = 0; }

  /* Test whether ARGV[optind] points to a non-option argument.  */
  /* C 495's `#define NONOPTION_P (...)` names the enclosing function's `argv` and `d`, which a
     Rust `macro_rules!` cannot (hygiene: a macro cannot name the caller's locals), so the port
     passes them in (the strftime.rs/md5.rs shape).  */
  macro_rules! NONOPTION_P {
    ($argv:expr, $d:expr) => {
      (*(*$argv.offset ((*$d).optind as isize)) != b'-' as c_char
       || *(*$argv.offset ((*$d).optind as isize)).offset (1) == 0)
    };
  }

  if (*d).__nextchar.is_null () || *(*d).__nextchar == 0
    {
      /* Advance to the next ARGV-element.  */

      /* Give FIRST_NONOPT & LAST_NONOPT rational values if OPTIND has been
	 moved back by the user (who may also have changed the arguments).  */
      if (*d).__last_nonopt > (*d).optind
	{ (*d).__last_nonopt = (*d).optind; }
      if (*d).__first_nonopt > (*d).optind
	{ (*d).__first_nonopt = (*d).optind; }

      if (*d).__ordering == __ord::PERMUTE
	{
	  /* If we have just processed some options following some non-options,
	     exchange them so that the options come first.  */

	  if (*d).__first_nonopt != (*d).__last_nonopt
	      && (*d).__last_nonopt != (*d).optind
	    { exchange (argv, d); }
	  else if (*d).__last_nonopt != (*d).optind
	    { (*d).__first_nonopt = (*d).optind; }

	  /* Skip any additional non-options
	     and extend the range of non-options previously skipped.  */

	  while (*d).optind < argc && NONOPTION_P! (argv, d)
	    { (*d).optind += 1; }
	  (*d).__last_nonopt = (*d).optind;
	}

      /* The special ARGV-element '--' means premature end of options.
	 Skip it like a null option,
	 then exchange with previous non-options as if it were an option,
	 then skip everything else like a non-option.  */

      if (*d).optind != argc && strcmp (*argv.offset ((*d).optind as isize),
                                        c"--".as_ptr ()) == 0
	{
	  (*d).optind += 1;

	  if (*d).__first_nonopt != (*d).__last_nonopt
	      && (*d).__last_nonopt != (*d).optind
	    { exchange (argv, d); }
	  else if (*d).__first_nonopt == (*d).__last_nonopt
	    { (*d).__first_nonopt = (*d).optind; }
	  (*d).__last_nonopt = argc;

	  (*d).optind = argc;
	}

      /* If we have done all the ARGV-elements, stop the scan
	 and back over any non-options that we skipped and permuted.  */

      if (*d).optind == argc
	{
	  /* Set the next-arg-index to point at the non-options
	     that we previously skipped, so the caller will digest them.  */
	  if (*d).__first_nonopt != (*d).__last_nonopt
	    { (*d).optind = (*d).__first_nonopt; }
	  return -1;
	}

      /* If we have come to a non-option and did not permute it,
	 either stop the scan or describe it to the caller and pass it by.  */

      if NONOPTION_P! (argv, d)
	{
	  if (*d).__ordering == __ord::REQUIRE_ORDER
	    { return -1; }
	  (*d).optarg = *argv.offset ((*d).optind as isize);
	  (*d).optind += 1;
	  return 1;
	}

      /* We have found another option-ARGV-element.
	 Check whether it might be a long option.  */
      if !longopts.is_null ()
	{
	  if *(*argv.offset ((*d).optind as isize)).offset (1) == b'-' as c_char
	    {
	      /* "--foo" is always a long option.  The special option
		 "--" was handled above.  */
	      (*d).__nextchar = (*argv.offset ((*d).optind as isize)).offset (2);
	      return process_long_option (argc, argv, optstring, longopts,
					  longind, long_only, d,
					  print_errors, c"--".as_ptr ());
	    }

	  /* If long_only and the ARGV-element has the form "-f",
	     where f is a valid short option, don't consider it an
	     abbreviated form of a long option that starts with f.
	     Otherwise there would be no way to give the -f short
	     option.

	     On the other hand, if there's a long option "fubar" and
	     the ARGV-element is "-fu", do consider that an
	     abbreviation of the long option, just like "--fu", and
	     not "-f" with arg "u".

	     This distinction seems to be the most useful approach.  */
	  if long_only != 0 && (*(*argv.offset ((*d).optind as isize)).offset (2) != 0
				|| strchr (optstring,
                                           *(*argv.offset ((*d).optind as isize)).offset (1)
                                           as c_int).is_null ())
	    {
	      (*d).__nextchar = (*argv.offset ((*d).optind as isize)).offset (1);
	      let code: c_int = process_long_option (argc, argv, optstring, longopts,
						     longind, long_only, d,
						     print_errors, c"-".as_ptr ());
	      if code != -1
		{ return code; }
	    }
	}

      /* It is not a long option.  Skip the initial punctuation.  */
      (*d).__nextchar = (*argv.offset ((*d).optind as isize)).offset (1);
    }

  /* Look at and handle the next short option-character.  */

  {
    let mut c: c_char = *(*d).__nextchar;
    (*d).__nextchar = (*d).__nextchar.add (1);
    let temp: *const c_char = strchr (optstring, c as c_int);

    /* Increment 'optind' when we start to process its last character.  */
    if *(*d).__nextchar == 0
      { (*d).optind += 1; }

    if temp.is_null () || c == b':' as c_char || c == b';' as c_char
      {
	if print_errors != 0
	  { fprintf (stderr, c"%s: invalid option -- '%c'\n".as_ptr (), *argv, c as c_int); }
	(*d).optopt = c as c_int;
	return '?' as c_int;
      }

    /* Convenience. Treat POSIX -W foo same as long option --foo */
    if *temp == b'W' as c_char && *temp.offset (1) == b';' as c_char && !longopts.is_null ()
      {
	/* This is an option that requires an argument.  */
	if *(*d).__nextchar != 0
	  { (*d).optarg = (*d).__nextchar; }
	else if (*d).optind == argc
	  {
	    if print_errors != 0
	      { fprintf (stderr, c"%s: option requires an argument -- '%c'\n".as_ptr (),
			 *argv, c as c_int); }

	    (*d).optopt = c as c_int;
	    if *optstring == b':' as c_char
	      { c = ':' as c_char; }
	    else
	      { c = '?' as c_char; }
	    return c as c_int;
	  }
	else
	  { (*d).optarg = *argv.offset ((*d).optind as isize); }

	(*d).__nextchar = (*d).optarg;
	(*d).optarg = std::ptr::null_mut ();
	return process_long_option (argc, argv, optstring, longopts, longind,
				    0 /* long_only */, d, print_errors, c"-W ".as_ptr ());
      }
    if *temp.offset (1) == b':' as c_char
      {
	if *temp.offset (2) == b':' as c_char
	  {
	    /* This is an option that accepts an argument optionally.  */
	    if *(*d).__nextchar != 0
	      {
		(*d).optarg = (*d).__nextchar;
		(*d).optind += 1;
	      }
	    else
	      { (*d).optarg = std::ptr::null_mut (); }
	    (*d).__nextchar = std::ptr::null_mut ();
	  }
	else
	  {
	    /* This is an option that requires an argument.  */
	    if *(*d).__nextchar != 0
	      {
		(*d).optarg = (*d).__nextchar;
		/* If we end this ARGV-element by taking the rest as an arg,
		   we must advance to the next element now.  */
		(*d).optind += 1;
	      }
	    else if (*d).optind == argc
	      {
		if print_errors != 0
		  { fprintf (stderr, c"%s: option requires an argument -- '%c'\n".as_ptr (),
			     *argv, c as c_int); }

		(*d).optopt = c as c_int;
		if *optstring == b':' as c_char
		  { c = ':' as c_char; }
		else
		  { c = '?' as c_char; }
	      }
	    else
	      /* We already incremented 'optind' once;
		 increment it again when taking next ARGV-elt as argument.  */
	      {
		(*d).optarg = *argv.offset ((*d).optind as isize);
		(*d).optind += 1;
	      }
	    (*d).__nextchar = std::ptr::null_mut ();
	  }
      }
    return c as c_int;
  }
}

/* C 700-717's `int _getopt_internal (...)`.  `_getopt_internal` is one of the names
   getopt-pfx-ext.h prefixes, so the frozen build's symbol is `rpl_getopt_internal`
   (PORTING.md 10.13; measured on build/emacs-ref/lib/getopt.o).  */
#[unsafe(export_name = "rpl_getopt_internal")]
pub unsafe extern "C" fn _getopt_internal (argc: c_int, argv: *mut *mut c_char,
                                           optstring: *const c_char, longopts: *const option,
                                           longind: *mut c_int, long_only: c_int,
                                           posixly_correct: c_int) -> c_int
{
  getopt_data.optind = optind;
  getopt_data.opterr = opterr;

  let result: c_int = _getopt_internal_r (argc, argv, optstring, longopts,
					  longind, long_only, &raw mut getopt_data,
					  posixly_correct);

  optind = getopt_data.optind;
  optarg = getopt_data.optarg;
  optopt = getopt_data.optopt;

  return result;
}

/* glibc gets a LSB-compliant getopt and a POSIX-compliant __posix_getopt.
   Standalone applications just get a POSIX-compliant getopt.
   POSIX and LSB both require these functions to take 'char *const *argv'
   even though this is incorrect (because of the permutation).  */

/* C 728-734's `#define GETOPT_ENTRY(NAME, POSIXLY_CORRECT)`.  The macro defines a function,
   which a Rust `macro_rules!` can do when the name is a metavariable (PORTING.md 0.2 keeps the
   two C spellings); the body's `_getopt_internal` resolves at the invocation site below.  */
macro_rules! GETOPT_ENTRY {
  ($name:ident, $posixly_correct:expr) => {
    #[unsafe(export_name = "rpl_getopt")]
    pub unsafe extern "C" fn $name (argc: c_int, argv: *const *mut c_char,
                                    optstring: *const c_char) -> c_int
    {
      return _getopt_internal (argc, argv as *mut *mut c_char, optstring,
			       std::ptr::null (), std::ptr::null_mut (), 0,
			       $posixly_correct);
    }
  };
}

/* C 736-737's `#ifdef _LIBC` arm is dead, so the live text is C 738's
   `GETOPT_ENTRY (getopt, 1)`; C 737's `GETOPT_ENTRY (__posix_getopt, 1)` twin is the dead
   side (P-51).  */
GETOPT_ENTRY! (getopt, 1);

/* C 740-811's `#ifdef TEST` program (`main`) is dead in the frozen configuration - TEST is the
   macro the reference's own harness defines (the group-member.c case) - so, like the other
   dead arms, it is named here and not written out (P-51).  */
