// femacs: port of lib/strtoimax.c @ fd7afced403.
/* emacs/lib/strtoimax.c: gnulib compiles this one source twice, once for `strtoimax` and once
   with UNSIGNED defined for `strtoumax` (the macro that selects the second build is not in the
   frozen configuration table, so the evaluator asks for both arms and the port keeps the second
   one under the feature naming it, the P-51 shape).  C 26's `<inttypes.h>` is the stdint mirror
   and C 28's `<stdlib.h>` supplies `strtol`/`strtoll` (declared here); C 34-36's and 43-45's
   `#ifndef HAVE_DECL_STRTO*` arms are the configure-time declaration tests, one of which the
   frozen configuration leaves undefined (HAVE_DECL_STRTOLL) and the other unknown.  */
use std::ffi::{c_char, c_int, c_long, c_uint, c_ulong};
include!("../src/include/config.rs");
include!("include/gnulib_config.rs");
include!("include/stdint.in.rs");

/* C 26's `<inttypes.h>` names the two widest integer types; the mirror of the reference's
   lisp.h carries them for the Emacs units, and this gnulib unit (which includes no such mirror)
   declares them where C gets them from <stdint.h>.  */
pub type intmax_t = i64;
pub type uintmax_t = u64;

/* C 32-40 / 41-49: the systems' own entries the file falls back to.  */
unsafe extern "C" {
  pub fn strtol (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
  pub fn strtoll (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
  #[cfg (feature = "unsigned")]
  pub fn strtoul (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
  #[cfg (feature = "unsigned")]
  pub fn strtoull (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
}

/* C 34-36's `# ifndef HAVE_DECL_STRTOULL` arm: the bare string literal C uses to make a missing
   configure-time test a compile error.  It is kept under the same feature, with the text in a
   constant so the tokens stay in the port.  */
#[cfg (feature = "unsigned")]
pub const HAVE_DECL_STRTOULL_TEST: &str = "this configure-time declaration test was not run";

/* C 51-57's `#define`s, both configurations.  */
#[cfg (not (feature = "unsigned"))]
macro_rules! Int { () => { intmax_t } }
#[cfg (feature = "unsigned")]
macro_rules! Int { () => { uintmax_t } }
#[cfg (not (feature = "unsigned"))]
macro_rules! Strtoimax { () => { strtoimax } }
#[cfg (feature = "unsigned")]
macro_rules! Strtoimax { () => { strtoumax } }
#[cfg (not (feature = "unsigned"))]
macro_rules! Strtol { () => { strtol } }
#[cfg (feature = "unsigned")]
macro_rules! Strtol { () => { strtoul } }
#[cfg (not (feature = "unsigned"))]
macro_rules! Strtoll { () => { strtoll } }
#[cfg (feature = "unsigned")]
macro_rules! Strtoll { () => { strtoull } }
#[cfg (not (feature = "unsigned"))]
macro_rules! Unsigned { () => { } }
#[cfg (feature = "unsigned")]
macro_rules! Unsigned { () => { unsigned } }

/* The definition is spelled `Strtoimax` in the reference (a macro that expands to the name of
   the build's function), so the port keeps that name for the ledger and emits the symbol each
   configuration defines through export_name (PORTING.md 10.13, the mempcpy precedent).  */
#[cfg (not (feature = "unsigned"))]
#[unsafe(export_name = "strtoimax")]
pub unsafe extern "C" fn Strtoimax (ptr: *const c_char, endptr: *mut *mut c_char, base: c_int)
  -> intmax_t
{
  const _: () = assert! (size_of::<intmax_t> () == size_of::<c_ulong> ()
			 || size_of::<intmax_t> () == size_of::<u64> ());

  if size_of::<intmax_t> () != size_of::<c_ulong> ()
    { return strtoll (ptr, endptr, base) as intmax_t; }

  return strtol (ptr, endptr, base) as intmax_t;
}

#[cfg (feature = "unsigned")]
#[unsafe(export_name = "strtoumax")]
pub unsafe extern "C" fn Strtoimax (ptr: *const c_char, endptr: *mut *mut c_char, base: c_int)
  -> uintmax_t
{
  const _: () = assert! (size_of::<uintmax_t> () == size_of::<c_ulong> ()
			 || size_of::<uintmax_t> () == size_of::<u64> ());

  if size_of::<uintmax_t> () != size_of::<c_ulong> ()
    { return strtoull (ptr, endptr, base) as uintmax_t; }

  return strtoul (ptr, endptr, base) as uintmax_t;
}
