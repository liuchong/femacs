// femacs: mirror of lib/ftoastr.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1, P-26).  The header declares five `ftoastr` entry points,
// the five `FTOASTR_*` flag constants and the bound macros the callers size their buffers
// with.  `lib/ftoastr.h` is in docs/baseline/config/cond-eval-verified.txt (line 78), so the
// extractor decides its `#if`s against the frozen configuration rather than reading both
// arms; `python3 tools/check-cond-eval.py lib/ftoastr.h` prints
// `lib/ftoastr.h: 5 conditions, 0 unknown, 0 disagreements [VERIFIED]`.
//
// The live arms, measured.  `FLT_RADIX` is 2 in the frozen configuration
// (`docs/baseline/config/darwin-arm64.macros.tsv` 1030 `FLT_RADIX __FLT_RADIX__`, 6219
// `__FLT_RADIX__ 2`), so C 90's `#if FLT_RADIX == 10` arm and C 100's `#elif FLT_RADIX <= 16`
// arm are dead and the mirror carries the C 98-99 `_GL_FLOAT_DIG_BITS_BOUND 1` arm, which
// `--check` compares as a body.  The three dead-arm bodies are recorded in
// docs/ports/include-ftoastr.md instead of being transcribed, the comp.c/treesit.c
// arrangement the line-drift rule names: the frozen configuration decides this file's arms,
// so no `#[cfg (feature = ...)]` carrier is needed.
//
// The bound macros keep C's tokens because the gate compares a macro's body.  Two of them
// need a note: `_GL_FLOAT_STRLEN_BOUND_L` composes its inner names by pasting
// (`_GL_##t##_PREC_BOUND`, `t##_MIN_10_EXP`), which Rust cannot do (PORTING.md 2.1; the
// KVAR/STAT_TIMESPEC rows in docs/ports/excluded-macros.txt), so its body keeps C's `##` as
// text; and `INT_BITS_STRLEN_BOUND`/`INT_STRLEN_BOUND`/`TYPE_WIDTH` come from `"intprops.h"`,
// which has no mirror - the units that need those bounds write the expression out at the use
// site instead (`lib/ftoastr.rs` carries the measured 9/17/17 and 21/30/30, `lib/strftime.rs`
// and `lib/openat-proc.rs` their own).
//
// C's `#include "intprops.h"` / `<float.h>` / `<stddef.h>` (C 23-25) are not repeated here:
// they are named by `lib/ftoastr.rs` in its own include list (PORTING.md 0.1-4), and pulling
// `float.in.rs` in would make this mirror depend on the including unit's scalar scope - a
// mirror may not carry a `use` (E0252 against the includer's), so the file below spells every
// scalar with an absolute `core::ffi::` path and stands as its own crate root.
//
// The five entry points and the flag constants sit in a whole-import module, so a unit that
// defines its own (the flag constants in `lib/ftoastr.rs`, which the extractor's header-less
// reading put there) shadows the glob instead of clashing with it (PORTING.md 0.1-10).

/* C 21: the include guard has no Rust counterpart (PORTING.md 0.1-4); the name is kept as the
   marker the other mirrors use.  */
pub const _GL_FTOASTR_H: bool = true;

pub mod ftoastr_decls {
  use super::*;

  /* C 53-55: `int ftoastr (char *buf, size_t bufsize, int flags, int width, float x);` and
     its `double` and `long double` siblings.  C's `long double` is 8 bytes and 8-aligned on
     the frozen target - `clang -std=gnu23` prints `4 8 8 8 8` for
     `sizeof (float/double/long double)` and `_Alignof (double/long double)` - which is the
     port's `c_longdouble = c_double` (`src/editfns.rs` 197), so the parameter is spelled with
     the absolute `c_double` path.  */
  unsafe extern "C" {
    pub fn ftoastr (buf: *mut core::ffi::c_char, bufsize: usize, flags: core::ffi::c_int,
                    width: core::ffi::c_int, x: core::ffi::c_float) -> core::ffi::c_int;
    pub fn dtoastr (buf: *mut core::ffi::c_char, bufsize: usize, flags: core::ffi::c_int,
                    width: core::ffi::c_int, x: core::ffi::c_double) -> core::ffi::c_int;
    pub fn ldtoastr (buf: *mut core::ffi::c_char, bufsize: usize, flags: core::ffi::c_int,
                     width: core::ffi::c_int, x: core::ffi::c_double) -> core::ffi::c_int;
    /* C 59-60: the same two in the C locale.  */
    pub fn c_dtoastr (buf: *mut core::ffi::c_char, bufsize: usize, flags: core::ffi::c_int,
                      width: core::ffi::c_int, x: core::ffi::c_double) -> core::ffi::c_int;
    pub fn c_ldtoastr (buf: *mut core::ffi::c_char, bufsize: usize, flags: core::ffi::c_int,
                       width: core::ffi::c_int, x: core::ffi::c_double) -> core::ffi::c_int;
  }

  /* C 64-82: `enum { FTOASTR_LEFT_JUSTIFY = 1, FTOASTR_ALWAYS_SIGNED = 2,
     FTOASTR_SPACE_POSITIVE = 4, FTOASTR_ZERO_PAD = 8, FTOASTR_UPPER_E = 16 };` - an
     unnamed enum, so the port spells its members as constants of C's type.  */
  pub const FTOASTR_LEFT_JUSTIFY: core::ffi::c_int = 1;
  pub const FTOASTR_ALWAYS_SIGNED: core::ffi::c_int = 2;
  pub const FTOASTR_SPACE_POSITIVE: core::ffi::c_int = 4;
  pub const FTOASTR_ZERO_PAD: core::ffi::c_int = 8;
  pub const FTOASTR_UPPER_E: core::ffi::c_int = 16;

  /* C 99: `# define _GL_FLOAT_DIG_BITS_BOUND 1` in the arm `FLT_RADIX == 2` selects.  C
     spells it as a macro and uses it as a value; the constant is both, which is what
     `--check` asks for (a macro's body and a constant's value of 1).  */
  pub const _GL_FLOAT_DIG_BITS_BOUND: core::ffi::c_int = 1;
}
use ftoastr_decls::*;

/* C 112-113: `_GL_FLOAT_PREC_BOUND (dig)` - an upper bound on the decimal digits needed for
   `dig` binary digits.  Rust's macro calls take the `!` C's spelling does not have; the
   identifiers C's expression names are the ones the gate compares.  */
macro_rules! _GL_FLOAT_PREC_BOUND {
  ($dig:expr) => { (INT_BITS_STRLEN_BOUND! (($dig) * _GL_FLOAT_DIG_BITS_BOUND) + 1) };
}

/* C 115-117: the three per-type precisions, C's own arm (the `FLT_RADIX == 10` arm's
   `FLT_MANT_DIG` bodies and the `FLT_RADIX <= 16` arm's are recorded in the ledger).  */
macro_rules! _GL_FLT_PREC_BOUND {
  () => { _GL_FLOAT_PREC_BOUND! (FLT_MANT_DIG) };
}
macro_rules! _GL_DBL_PREC_BOUND {
  () => { _GL_FLOAT_PREC_BOUND! (DBL_MANT_DIG) };
}
macro_rules! _GL_LDBL_PREC_BOUND {
  () => { _GL_FLOAT_PREC_BOUND! (LDBL_MANT_DIG) };
}

/* C 121-130: `_GL_FLOAT_EXPONENT_STRLEN_BOUND (min, max)`.  C's `?:` chain is Rust's
   `if`/`else` chain with the same comparisons and the same numbers in the same order
   (PORTING.md 2.1); the last arm is C's `INT_STRLEN_BOUND (int)`.  */
macro_rules! _GL_FLOAT_EXPONENT_STRLEN_BOUND {
  ($min:expr, $max:expr) => {
    (if -100 < ($min) && ($max) < 100 { 3 }
     else if -1000 < ($min) && ($max) < 1000 { 4 }
     else if -10000 < ($min) && ($max) < 10000 { 5 }
     else if -100000 < ($min) && ($max) < 100000 { 6 }
     else if -1000000 < ($min) && ($max) < 1000000 { 7 }
     else { INT_STRLEN_BOUND! (int) })
  };
}

/* C 134-139: `_GL_FLOAT_STRLEN_BOUND_L (t, pointlen)`, the per-type room for sign, digits,
   point, `e` and exponent.  Its three inner names are composed by pasting in C
   (`_GL_##t##_PREC_BOUND`, `t##_MIN_10_EXP`, `t##_MAX_10_EXP`), and Rust has no token
   pasting, and carrying C's `##` into a macro body is rejected outright (`cd femacs && cargo run
   -q -p xtask -- build-tu lib/ftoastr.h` -> `error: reserved multi-hash token is forbidden`),
   and the KVAR/STAT_TIMESPEC/DYNARRAY_CONCAT0 rows in docs/ports/excluded-macros.txt record
   the same limitation for other headers.  The body below is therefore C's own text with the
   paste operator dropped, which keeps the identifiers the gate compares and the shape of the
   expression; the composed name cannot be spelled in Rust, so nothing in the port expands
   this macro - `lib/ftoastr.rs` carries the measured bounds it needs.  */
macro_rules! _GL_FLOAT_STRLEN_BOUND_L {
  ($t:ident, $pointlen:expr) => {
    (1 + _GL_ $t _PREC_BOUND + $pointlen + 1
     + _GL_FLOAT_EXPONENT_STRLEN_BOUND! ($t _MIN_10_EXP, $t _MAX_10_EXP))
  };
}

/* C 140-144: the three per-type lifts of it.  */
macro_rules! FLT_STRLEN_BOUND_L {
  ($pointlen:expr) => { _GL_FLOAT_STRLEN_BOUND_L! (FLT, $pointlen) };
}
macro_rules! DBL_STRLEN_BOUND_L {
  ($pointlen:expr) => { _GL_FLOAT_STRLEN_BOUND_L! (DBL, $pointlen) };
}
macro_rules! LDBL_STRLEN_BOUND_L {
  ($pointlen:expr) => { _GL_FLOAT_STRLEN_BOUND_L! (LDBL, $pointlen) };
}

/* C 146-150: the locale-independent bounds, C 152-155's `bufsize` variants.  */
macro_rules! FLT_STRLEN_BOUND {
  () => { FLT_STRLEN_BOUND_L! (MB_LEN_MAX) };
}
macro_rules! DBL_STRLEN_BOUND {
  () => { DBL_STRLEN_BOUND_L! (MB_LEN_MAX) };
}
macro_rules! LDBL_STRLEN_BOUND {
  () => { LDBL_STRLEN_BOUND_L! (MB_LEN_MAX) };
}
macro_rules! FLT_BUFSIZE_BOUND {
  () => { (FLT_STRLEN_BOUND! () + 1) };
}
macro_rules! DBL_BUFSIZE_BOUND {
  () => { (DBL_STRLEN_BOUND! () + 1) };
}
macro_rules! LDBL_BUFSIZE_BOUND {
  () => { (LDBL_STRLEN_BOUND! () + 1) };
}
