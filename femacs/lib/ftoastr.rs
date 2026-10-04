// femacs: port of lib/ftoastr.c @ fd7afced403.
//
// gnulib compiles this source only through its inclusion shims, never on its own: `LENGTH`
// selects the body (3 -> long double/ldtoastr, 2 -> double/dtoastr, else -> float/ftoastr),
// and the frozen build compiles the `LENGTH == 2` shim `lib/dtoastr.c` (build/emacs-ref/lib
// carries dtoastr.o and no ftoastr.o).  PORTING.md 10.13② asks the shared body to land first
// as a prelude `macro_rules!` and the shim to instantiate it, so the body below is
// `ftoastr_body!` and `femacs/lib/dtoastr.rs` is its only instantiation.  This file's own
// `--check` compares C's standalone reading (`LENGTH == 1`), which `mod standalone` records.
// The frozen build takes the `!GNULIB_SNPRINTF` arm (dtoastr.o imports `_sprintf`, not
// `_snprintf`), so the fallback `ftoastr_snprintf` is the live definition; `C_LOCALE` is
// undefined, so C 39-41 leave `PREFIX(name)` as `name`.  There is no `#![allow (…)]` here:
// an inner attribute inside an included file is E0753, so the unit that compiles it
// (`femacs/lib/dtoastr.rs`) carries the list.

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};
include!("../src/include/config.rs");
include!("include/float.in.rs");
include!("include/limits.in.rs");
include!("include/stdio.rs");
include!("include/stdlib.rs");

/* C 30's `"ftoastr.h"` now has a mirror (`include/ftoastr.rs`, status 3.194), which owns the five
   `FTOASTR_*` flags and `_GL_FLOAT_DIG_BITS_BOUND`; the `_GL_*_PREC_BOUND` bounds and the
   `*_BUFSIZE_BOUND` sizes stay here, with the values the frozen darwin-arm64 compiler reports
   (9/17/17 and 21/30/30), and `DBL_MIN` comes from the system `<float.h>` (which also has no
   mirror).  `MB_LEN_MAX` is in `include/limits.in.rs`.  */
include!("include/ftoastr.rs");
pub const DBL_MIN: c_double = 2.2250738585072014e-308;
pub const _GL_FLT_PREC_BOUND: c_int = 9;
pub const _GL_DBL_PREC_BOUND: c_int = 17;
pub const _GL_LDBL_PREC_BOUND: c_int = 17;
pub const FLT_BUFSIZE_BOUND: usize = 21;
pub const DBL_BUFSIZE_BOUND: usize = 30;
pub const LDBL_BUFSIZE_BOUND: usize = 30;

/* C 44-77's `LENGTH == 1` arm, the one C's standalone text compiles.  The frozen build
   compiles the `LENGTH == 2` arm through dtoastr.rs; both arms' bindings are the body
   macro's parameters.  The record sits in a module because `LENGTH` here would clash with the
   shim's own `LENGTH` in the one crate that includes this file (the strftime.rs standalone
   decision).  */
mod standalone {
  use super::*;
  macro_rules! FLOAT { () => { c_float } }
  macro_rules! FLOAT_DIG { () => { FLT_DIG } }
  macro_rules! FLOAT_MIN { () => { FLT_MIN } }
  macro_rules! FLOAT_PREC_BOUND { () => { _GL_FLT_PREC_BOUND } }
  macro_rules! FTOASTR { () => { PREFIX (ftoastr) } }
  macro_rules! PROMOTED_FLOAT { () => { c_double } }
  macro_rules! STRTOF { () => { PREFIX (strtod) } }
  macro_rules! snprintf { () => { ftoastr_snprintf } }
  macro_rules! PREFIX { ($name:ident) => { $name } }
  pub const LENGTH: c_int = 1;
}

/* C 44-51's `LENGTH == 3` arm (`ldtoastr.c`) and C 67-69's `HAVE_STRTOF` arm are dead
   in the frozen build: `HAVE_STRTOF` is undefined, so C 75-77 make `STRTOF` `strtod`.  */

/* C 79-152 as the shared `macro_rules!` PORTING.md 10.13② requires.  `$length` is C's
   LENGTH, `$name` the identifier (ftoastr/dtoastr/ldtoastr), `$export` the symbol the
   including unit emits, and the rest are the arm's bindings (the double arm, from
   dtoastr.rs).  C's `snprintf` rename (C 85-87) is the direct `ftoastr_snprintf` call.  */
macro_rules! ftoastr_body {
  ($length:expr, $name:ident, $export:literal, $float:ty, $promoted:ty, $dig:expr,
   $min:expr, $prec_bound:expr, $strtof:expr) => {
    /* C 45/53: the arm's type names.  */
    pub type FLOAT = $float;
    pub type PROMOTED_FLOAT = $promoted;

    /* C 88-108: the `!GNULIB_SNPRINTF` fallback.  */
    unsafe fn ftoastr_snprintf (buf: *mut c_char, bufsize: usize, format: *const c_char,
                                width: c_int, prec: c_int, x: FLOAT) -> c_int
    {
      const WIDTH_0_BUFSIZE_BOUND: usize = if $length == 1 { FLT_BUFSIZE_BOUND }
          else if $length == 2 { DBL_BUFSIZE_BOUND } else { LDBL_BUFSIZE_BOUND };
      let promoted_x: PROMOTED_FLOAT = x;
      let mut width_0_buffer: [c_char; WIDTH_0_BUFSIZE_BOUND] = [0; WIDTH_0_BUFSIZE_BOUND];
      let mut n: c_int = width;
      if bufsize < width_0_buffer.len ()
        {
          n = sprintf (width_0_buffer.as_mut_ptr (), format, 0, prec, promoted_x);
          if n < 0
            { return n; }
          if n < width
            { n = width; }
        }
      if (n as usize) < bufsize
        { n = sprintf (buf, format, width, prec, promoted_x); }
      return n;
    }

    /* C 111-152.  C 125's `char format[sizeof "%-+ 0*.*Lg"]` is the literal's 10 bytes plus
       the null.  */
    #[unsafe(export_name = $export)]
    pub unsafe extern "C" fn $name (buf: *mut c_char, bufsize: usize, flags: c_int,
                                    width: c_int, x: FLOAT) -> c_int
    {
      let promoted_x: PROMOTED_FLOAT = x;
      let mut format: [c_char; 11] = [0; 11];
      let abs_x: FLOAT = if x < 0.0 { -x } else { x };
      let mut p: *mut c_char = format.as_mut_ptr ();
      *p = b'%' as c_char; p = p.add (1);
      *p = b'-' as c_char; p = p.add (((flags & FTOASTR_LEFT_JUSTIFY) != 0) as usize);
      *p = b'+' as c_char; p = p.add (((flags & FTOASTR_ALWAYS_SIGNED) != 0) as usize);
      *p = b' ' as c_char; p = p.add (((flags & FTOASTR_SPACE_POSITIVE) != 0) as usize);
      *p = b'0' as c_char; p = p.add (((flags & FTOASTR_ZERO_PAD) != 0) as usize);
      *p = b'*' as c_char; p = p.add (1);
      *p = b'.' as c_char; p = p.add (1);
      *p = b'*' as c_char; p = p.add (1);
      *p = b'L' as c_char; p = p.add ((2 < $length) as usize);
      *p = (if (flags & FTOASTR_UPPER_E) != 0 { b'G' } else { b'g' }) as c_char;
      p = p.add (1);
      *p = 0;
      let mut prec: c_int = if abs_x < $min { 1 } else { $dig };
      loop
        {
          let n: c_int = ftoastr_snprintf (buf, bufsize, format.as_ptr (), width, prec,
                                           promoted_x as FLOAT);
          if n < 0
              || $prec_bound <= prec
              || ((n as usize) < bufsize && $strtof (buf, core::ptr::null_mut ()) == x)
            { return n; }
          prec += 1;
        }
    }
  };
}
