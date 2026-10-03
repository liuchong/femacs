// femacs: port of lib/getopt1.c @ fd7afced403.
//
// NOTE (docs/ports/getopt1.md): lib/getopt1.c is **not** in
// `docs/baseline/config/cond-eval-verified.txt` - `tools/check-cond-eval.py` finds one
// disagreement, C 29's `#ifdef __getopt_argv_const_is_empty` (clang true, the condensed table
// false, because getopt-pfx-ext.h 47-50 defines it) - so the gate does not consult the table
// for this file and treats *both* arms as live text: the checklist carries the dead
// `#ifdef TEST` program's `main` and both `ARGV_CAST` definitions (whose body is therefore not
// compared at all).  The port follows clang: `__getopt_argv_const_is_empty` is defined in the
// frozen build, so `ARGV_CAST` is the identity and the prototypes take `char **argv`
// (getopt-ext.rs); the `#else` arm is named at its site (P-51), and `main` is kept under
// `#[cfg (feature = "test")]`, the group-member.c treatment.
//
// The `__GETOPT_PREFIX` layer (`rpl_`) renames `getopt_long`/`getopt_long_only` and leaves the
// two reentrant entry points alone: `nm build/emacs-ref/lib/getopt1.o` defines
// `_rpl_getopt_long`, `_rpl_getopt_long_only`, `__getopt_long_r` and `__getopt_long_only_r`.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_int};

include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/getopt.in.rs");
include!("include/getopt_int.rs");

/* C 20-25: `#ifndef _LIBC / # include <config.h>` (live: `_LIBC` is undefined), <getopt.h>
   (the getopt.in.rs mirror) and getopt_int.h (the getopt_int.rs mirror).  */

/* Convert char *__getopt_argv_const * to char ** without provoking
   gcc -Wuseless-cast when __getopt_argv_const is empty.  */
/* C 29-33's `#ifdef __getopt_argv_const_is_empty` arm is the one clang takes (the frozen
   configuration defines the macro in getopt-pfx-ext.h), so `ARGV_CAST` is the identity; the
   `#else` arm - `((char **) (argv))` - is the non-frozen side (P-51).  Both arms are the same
   expression in Rust, where the prototype is already `char **` (getopt-ext.rs).  */
macro_rules! ARGV_CAST {
  ($argv:expr) => { $argv };
}

/* C 35-41's `getopt_long`.  */
#[unsafe(export_name = "rpl_getopt_long")]
pub unsafe extern "C" fn getopt_long (argc: c_int, argv: *mut *mut c_char,
                                      options: *const c_char, long_options: *const option,
                                      opt_index: *mut c_int) -> c_int
{
  return _getopt_internal (argc, ARGV_CAST! (argv), options, long_options,
                           opt_index, 0, 0);
}

/* C 43-50's `_getopt_long_r` (not in the prefix list: the symbol keeps C's name).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _getopt_long_r (argc: c_int, argv: *mut *mut c_char,
                                         options: *const c_char, long_options: *const option,
                                         opt_index: *mut c_int, d: *mut _getopt_data) -> c_int
{
  return _getopt_internal_r (argc, argv, options, long_options, opt_index,
                             0, d, 0);
}

/* Like getopt_long, but '-' as well as '--' can indicate a long option.
   If an option that starts with '-' (not '--') doesn't match a long option,
   but does match a short option, it is parsed as a short option
   instead.  */

/* C 57-64's `getopt_long_only`.  */
#[unsafe(export_name = "rpl_getopt_long_only")]
pub unsafe extern "C" fn getopt_long_only (argc: c_int, argv: *mut *mut c_char,
                                           options: *const c_char,
                                           long_options: *const option, opt_index: *mut c_int)
  -> c_int
{
  return _getopt_internal (argc, ARGV_CAST! (argv), options, long_options,
                           opt_index, 1, 0);
}

/* C 66-73's `_getopt_long_only_r`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _getopt_long_only_r (argc: c_int, argv: *mut *mut c_char,
                                              options: *const c_char,
                                              long_options: *const option,
                                              opt_index: *mut c_int, d: *mut _getopt_data)
  -> c_int
{
  return _getopt_internal_r (argc, argv, options, long_options, opt_index,
                             1, d, 0);
}

/* C 76-166's `#ifdef TEST` program: dead in the frozen configuration (TEST is the macro the
   reference's own harness defines), kept as gated text (P-51, the group-member.c shape).  It
   needs a `main` because C defines one non-static in that arm, and the stdio entries it names
   are declared with it.  */
#[cfg (feature = "test")]
unsafe extern "C" {
  /* int printf (const char *format, ...);  */
  pub fn printf (format: *const c_char, ...) -> c_int;
  /* void exit (int status) __attribute__ ((__noreturn__));  */
  pub fn exit (status: c_int) -> !;
}

#[cfg (feature = "test")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  let mut digit_optind: c_int = 0;

  loop
    {
      let this_option_optind: c_int = if optind != 0 { optind } else { 1 };
      let mut option_index: c_int = 0;
      let long_options: [option; 8] =
	[
	  option { name: c"add".as_ptr (), has_arg: 1, flag: std::ptr::null_mut (), val: 0 },
	  option { name: c"append".as_ptr (), has_arg: 0, flag: std::ptr::null_mut (), val: 0 },
	  option { name: c"delete".as_ptr (), has_arg: 1, flag: std::ptr::null_mut (), val: 0 },
	  option { name: c"verbose".as_ptr (), has_arg: 0, flag: std::ptr::null_mut (), val: 0 },
	  option { name: c"create".as_ptr (), has_arg: 0, flag: std::ptr::null_mut (), val: 0 },
	  option { name: c"file".as_ptr (), has_arg: 1, flag: std::ptr::null_mut (), val: 0 },
	  option { name: std::ptr::null (), has_arg: 0, flag: std::ptr::null_mut (), val: 0 },
	  option { name: std::ptr::null (), has_arg: 0, flag: std::ptr::null_mut (), val: 0 },
	];

      let c: c_int = getopt_long (argc, argv, c"abc:d:0123456789".as_ptr (),
				  long_options.as_ptr (), &raw mut option_index);
      if c == -1
	{ break; }

      match c
	{
	  0 =>
	    {
	      printf (c"option %s".as_ptr (),
		      (*long_options.as_ptr ().offset (option_index as isize)).name);
	      if !optarg.is_null ()
		{ printf (c" with arg %s".as_ptr (), optarg); }
	      printf (c"\n".as_ptr ());
	    }

	  x if x == b'0' as c_int || x == b'1' as c_int || x == b'2' as c_int
	    || x == b'3' as c_int || x == b'4' as c_int || x == b'5' as c_int
	    || x == b'6' as c_int || x == b'7' as c_int || x == b'8' as c_int
	    || x == b'9' as c_int =>
	    {
	      if digit_optind != 0 && digit_optind != this_option_optind
		{ printf (c"digits occur in two different argv-elements.\n".as_ptr ()); }
	      digit_optind = this_option_optind;
	      printf (c"option %c\n".as_ptr (), c);
	    }

	  x if x == b'a' as c_int =>
	    { printf (c"option a\n".as_ptr ()); }

	  x if x == b'b' as c_int =>
	    { printf (c"option b\n".as_ptr ()); }

	  x if x == b'c' as c_int =>
	    { printf (c"option c with value '%s'\n".as_ptr (), optarg); }

	  x if x == b'd' as c_int =>
	    { printf (c"option d with value '%s'\n".as_ptr (), optarg); }

	  x if x == b'?' as c_int =>
	    { }

	  _ =>
	    { printf (c"?? getopt returned character code 0%o ??\n".as_ptr (), c); }
	}
    }

  if optind < argc
    {
      printf (c"non-option ARGV-elements: ".as_ptr ());
      while optind < argc
	{
	  printf (c"%s ".as_ptr (), *argv.offset (optind as isize));
	  optind += 1;
	}
      printf (c"\n".as_ptr ());
    }

  exit (0);
}
