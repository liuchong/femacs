// femacs: mirror of lib/ieee754.in.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1).  The criterion is the generated header
// (`build/emacs-ref/lib/ieee754.h`, 5,660 bytes): it spells the template's glibc arm
// `#if 0` (its line 20), so the live arm is the gnulib one, and the four order knobs name
// `<endian.h>`'s `BYTE_ORDER`/`BIG_ENDIAN`/`LITTLE_ENDIAN`/`BYTE_ORDER`.  Their owner in this
// tree is `lib/include/endian.in.rs` (1234/4321, measured on this target), so this mirror
// includes it and keeps C's own names in each initializer.
//
// The three unions are bitfield carriers.  Rust has no bitfields, so each one is a storage
// unit plus member-name accessors (PORTING.md P-80); every bit position below is proven by a
// byte dump from the frozen clang, and every size/align by that compiler's own `sizeof`/
// `_Alignof` - both are in `docs/ports/include-ieee754.in.md`'s Evidence.

// [U1-MODULE-MIRROR]
use super::*;
// [U1] endian.in.rs imported via super

/* C 19-20: the include guard has no Rust counterpart (PORTING.md 0.1-4); its C body is `1`,
   which the body check compares, so the const keeps that value (`byteswap.in.rs` 23).  */
pub const _IEEE754_H: core::ffi::c_int = 1;

/// C 32: `#define _IEEE754_BYTE_ORDER BYTE_ORDER` (the live gnulib arm; measured value 1234).
pub const _IEEE754_BYTE_ORDER: core::ffi::c_int = BYTE_ORDER;

/// C 33: `#define _IEEE754_BIG_ENDIAN BIG_ENDIAN` (measured value 4321).
pub const _IEEE754_BIG_ENDIAN: core::ffi::c_int = BIG_ENDIAN;

/// C 34: `#define _IEEE754_LITTLE_ENDIAN LITTLE_ENDIAN` (measured value 1234).
pub const _IEEE754_LITTLE_ENDIAN: core::ffi::c_int = LITTLE_ENDIAN;

/// C 35: `#define _IEEE754_FLOAT_WORD_ORDER BYTE_ORDER` (measured value 1234, i.e. the
/// little-endian arm of every `# if _IEEE754_FLOAT_WORD_ORDER == _IEEE754_BIG_ENDIAN` below
/// is the dead one).
pub const _IEEE754_FLOAT_WORD_ORDER: core::ffi::c_int = BYTE_ORDER;

/* C 44-78: `union ieee754_float`.  Measured: the union is 4 bytes with align 4, and so is
   each of its two structs.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ieee754_float_ieee {
  /// C 51-59 (the live little-endian arm): `mantissa:23` (bits 0-22), `exponent:8`
  /// (bits 23-30), `negative:1` (bit 31).  The byte dump of
  /// `{ negative = 1, exponent = 0x7f, mantissa = 1 }` is `01 00 80 bf`.
  pub units: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ieee754_float_ieee_nan {
  /// C 61-69: `mantissa:22` (bits 0-21), `quiet_nan:1` (bit 22), `exponent:8` (23-30),
  /// `negative:1` (31).
  pub units: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ieee754_float {
  pub f: core::ffi::c_float,
  pub ieee: ieee754_float_ieee,
  pub ieee_nan: ieee754_float_ieee_nan,
}

const _: () = core::assert! (std::mem::size_of::<ieee754_float> () == 4);
const _: () = core::assert! (std::mem::align_of::<ieee754_float> () == 4);
const _: () = core::assert! (std::mem::size_of::<ieee754_float_ieee> () == 4);
const _: () = core::assert! (std::mem::align_of::<ieee754_float_ieee> () == 4);
const _: () = core::assert! (std::mem::size_of::<ieee754_float_ieee_nan> () == 4);
const _: () = core::assert! (std::mem::align_of::<ieee754_float_ieee_nan> () == 4);

/* C 79: `#define IEEE754_FLOAT_BIAS 0x7f` (measured 127).  */
pub const IEEE754_FLOAT_BIAS: core::ffi::c_int = 0x7f;

/* The members of `union ieee754_float`'s two structs, as member-name accessors (PORTING.md
   1.3/P-80: each C member name stays in the port's text).  The second argument is the union
   place C writes, so the expansion is exactly C's read.  A one-bit member returns the `bool`
   C's `!` needs (a Rust `!` on an integer is bitwise, C's is logical).  */
pub const IEEE754_FLOAT_IEEE_MANTISSA_MASK: u32 = 0x007fffff;   /* mantissa : 23, bits 0-22 */
pub const IEEE754_FLOAT_IEEE_EXPONENT_MASK: u32 = 0x7f800000;   /* exponent : 8, bits 23-30 */
pub const IEEE754_FLOAT_IEEE_NEGATIVE_MASK: u32 = 1 << 31;      /* negative : 1, bit 31 */
pub const IEEE754_FLOAT_NAN_MANTISSA_MASK: u32 = 0x003fffff;    /* ieee_nan: mantissa : 22, bits 0-21 */
pub const IEEE754_FLOAT_NAN_QUIET_NAN_MASK: u32 = 1 << 22;      /* ieee_nan: quiet_nan : 1, bit 22 */
pub const IEEE754_FLOAT_NAN_EXPONENT_MASK: u32 = 0x7f800000;    /* ieee_nan: exponent : 8, bits 23-30 */
pub const IEEE754_FLOAT_NAN_NEGATIVE_MASK: u32 = 1 << 31;       /* ieee_nan: negative : 1, bit 31 */

macro_rules! ieee754_float_bits {
  (ieee.mantissa, $f:expr) => { ($f.ieee.units & IEEE754_FLOAT_IEEE_MANTISSA_MASK) };
  (ieee.exponent, $f:expr) => { ($f.ieee.units & IEEE754_FLOAT_IEEE_EXPONENT_MASK) >> 23 };
  (ieee.negative, $f:expr) => { ($f.ieee.units & IEEE754_FLOAT_IEEE_NEGATIVE_MASK) != 0 };
  (ieee_nan.mantissa, $f:expr) => { ($f.ieee_nan.units & IEEE754_FLOAT_NAN_MANTISSA_MASK) };
  (ieee_nan.quiet_nan, $f:expr) => { ($f.ieee_nan.units & IEEE754_FLOAT_NAN_QUIET_NAN_MASK) != 0 };
  (ieee_nan.exponent, $f:expr) => { ($f.ieee_nan.units & IEEE754_FLOAT_NAN_EXPONENT_MASK) >> 23 };
  (ieee_nan.negative, $f:expr) => { ($f.ieee_nan.units & IEEE754_FLOAT_NAN_NEGATIVE_MASK) != 0 };
}


/* C 81-139: `union ieee754_double`.  Measured: the union is 8 bytes with align 8 (the
   double's own), each struct 8 bytes with align 4.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ieee754_double_ieee {
  /// C 89-108 (the live little-endian/little-word-order arm): `mantissa1:32` (unit 0),
  /// `mantissa0:20` (unit 1 bits 0-19), `exponent:11` (20-30), `negative:1` (31).  The byte
  /// dump of `{ negative = 1, exponent = 0x3ff, mantissa0 = 0xfffff, mantissa1 = 1 }` is
  /// `01 00 00 00 ff ff ff bf`.
  pub units: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ieee754_double_ieee_nan {
  /// C 112-135: `mantissa1:32` (unit 0), `mantissa0:19` (unit 1 bits 0-18), `quiet_nan:1`
  /// (bit 19), `exponent:11` (20-30), `negative:1` (31).  The byte dump of
  /// `{ negative = 1, quiet_nan = 1, exponent = 0x3ff, mantissa0 = 0x1 }` is
  /// `00 00 00 00 01 00 08 ff`.
  pub units: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ieee754_double {
  pub d: core::ffi::c_double,
  pub ieee: ieee754_double_ieee,
  pub ieee_nan: ieee754_double_ieee_nan,
}

const _: () = core::assert! (std::mem::size_of::<ieee754_double> () == 8);
const _: () = core::assert! (std::mem::align_of::<ieee754_double> () == 8);
const _: () = core::assert! (std::mem::size_of::<ieee754_double_ieee> () == 8);
const _: () = core::assert! (std::mem::align_of::<ieee754_double_ieee> () == 4);
const _: () = core::assert! (std::mem::size_of::<ieee754_double_ieee_nan> () == 8);
const _: () = core::assert! (std::mem::align_of::<ieee754_double_ieee_nan> () == 4);

/* C 141: `#define IEEE754_DOUBLE_BIAS 0x3ff` (measured 1023).  */
pub const IEEE754_DOUBLE_BIAS: core::ffi::c_int = 0x3ff;

pub const IEEE754_DOUBLE_IEEE_MANTISSA0_MASK: u32 = 0x000fffff; /* mantissa0 : 20, unit 1 bits 0-19 */
pub const IEEE754_DOUBLE_IEEE_EXPONENT_MASK: u32 = 0x7ff00000;  /* exponent : 11, unit 1 bits 20-30 */
pub const IEEE754_DOUBLE_IEEE_NEGATIVE_MASK: u32 = 1 << 31;     /* negative : 1, unit 1 bit 31 */
pub const IEEE754_DOUBLE_NAN_MANTISSA0_MASK: u32 = 0x0007ffff;  /* ieee_nan: mantissa0 : 19, unit 1 bits 0-18 */
pub const IEEE754_DOUBLE_NAN_QUIET_NAN_MASK: u32 = 1 << 19;     /* ieee_nan: quiet_nan : 1, unit 1 bit 19 */
pub const IEEE754_DOUBLE_NAN_EXPONENT_MASK: u32 = 0x7ff00000;   /* ieee_nan: exponent : 11, unit 1 bits 20-30 */
pub const IEEE754_DOUBLE_NAN_NEGATIVE_MASK: u32 = 1 << 31;      /* ieee_nan: negative : 1, unit 1 bit 31 */

macro_rules! ieee754_double_bits {
  (ieee.mantissa1, $d:expr) => { ($d.ieee.units[0]) };
  (ieee.mantissa0, $d:expr) => { ($d.ieee.units[1] & IEEE754_DOUBLE_IEEE_MANTISSA0_MASK) };
  (ieee.exponent, $d:expr) => { ($d.ieee.units[1] & IEEE754_DOUBLE_IEEE_EXPONENT_MASK) >> 20 };
  (ieee.negative, $d:expr) => { ($d.ieee.units[1] & IEEE754_DOUBLE_IEEE_NEGATIVE_MASK) != 0 };
  (ieee_nan.mantissa1, $d:expr) => { ($d.ieee_nan.units[0]) };
  (ieee_nan.mantissa0, $d:expr) => { ($d.ieee_nan.units[1] & IEEE754_DOUBLE_NAN_MANTISSA0_MASK) };
  (ieee_nan.quiet_nan, $d:expr) => { ($d.ieee_nan.units[1] & IEEE754_DOUBLE_NAN_QUIET_NAN_MASK) != 0 };
  (ieee_nan.exponent, $d:expr) => { ($d.ieee_nan.units[1] & IEEE754_DOUBLE_NAN_EXPONENT_MASK) >> 20 };
  (ieee_nan.negative, $d:expr) => { ($d.ieee_nan.units[1] & IEEE754_DOUBLE_NAN_NEGATIVE_MASK) != 0 };
}


/* C 144-207: `union ieee854_long_double`.  Measured: the union is 16 bytes with align 8, and
   each struct 12 bytes with align 4 - this target's `long double` is the 8-byte double
   (`editfns.rs` 197's `pub type c_longdouble = c_double;` is the same measurement), so the
   union is `ceil (12, 8)`.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ieee854_long_double_ieee {
  /// C 152-173 (the live little-endian/little-word-order arm): `mantissa1:32` (unit 0),
  /// `mantissa0:32` (unit 1), `exponent:15` (unit 2 bits 0-14), `negative:1` (bit 15),
  /// `empty:16` (bits 16-31).  The byte dump of
  /// `{ negative = 1, exponent = 0x3fff, mantissa0 = 1, mantissa1 = 1, empty = 0 }` is
  /// `01 00 00 00 01 00 00 00 ff bf 00 00`.
  pub units: [u32; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ieee854_long_double_ieee_nan {
  /// C 176-204: `mantissa1:32` (unit 0), `mantissa0:30` (unit 1 bits 0-29), `quiet_nan:1`
  /// (bit 30), `one:1` (bit 31), `exponent:15` (unit 2 bits 0-14), `negative:1` (bit 15),
  /// `empty:16` (bits 16-31).
  pub units: [u32; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union ieee854_long_double {
  /// C 146: `long double d` - 8 bytes here (the struct's 12 bytes are what makes the union 16).
  pub d: core::ffi::c_double,
  pub ieee: ieee854_long_double_ieee,
  pub ieee_nan: ieee854_long_double_ieee_nan,
}

const _: () = core::assert! (std::mem::size_of::<ieee854_long_double> () == 16);
const _: () = core::assert! (std::mem::align_of::<ieee854_long_double> () == 8);
const _: () = core::assert! (std::mem::size_of::<ieee854_long_double_ieee> () == 12);
const _: () = core::assert! (std::mem::align_of::<ieee854_long_double_ieee> () == 4);
const _: () = core::assert! (std::mem::size_of::<ieee854_long_double_ieee_nan> () == 12);
const _: () = core::assert! (std::mem::align_of::<ieee854_long_double_ieee_nan> () == 4);

/* C 209: `#define IEEE854_LONG_DOUBLE_BIAS 0x3fff` (measured 16383).  */
pub const IEEE854_LONG_DOUBLE_BIAS: core::ffi::c_int = 0x3fff;

pub const IEEE854_LONG_DOUBLE_IEEE_MANTISSA0_MASK: u32 = 0xffffffff; /* mantissa0 : 32, unit 1 */
pub const IEEE854_LONG_DOUBLE_IEEE_EXPONENT_MASK: u32 = 0x00007fff;  /* exponent : 15, unit 2 bits 0-14 */
pub const IEEE854_LONG_DOUBLE_IEEE_NEGATIVE_MASK: u32 = 1 << 15;     /* negative : 1, unit 2 bit 15 */
pub const IEEE854_LONG_DOUBLE_IEEE_EMPTY_MASK: u32 = 0xffff0000;     /* empty : 16, unit 2 bits 16-31 */
pub const IEEE854_LONG_DOUBLE_NAN_MANTISSA0_MASK: u32 = 0x3fffffff;  /* ieee_nan: mantissa0 : 30, unit 1 bits 0-29 */
pub const IEEE854_LONG_DOUBLE_NAN_QUIET_NAN_MASK: u32 = 1 << 30;     /* ieee_nan: quiet_nan : 1, unit 1 bit 30 */
pub const IEEE854_LONG_DOUBLE_NAN_ONE_MASK: u32 = 1 << 31;           /* ieee_nan: one : 1, unit 1 bit 31 */
pub const IEEE854_LONG_DOUBLE_NAN_EXPONENT_MASK: u32 = 0x00007fff;   /* ieee_nan: exponent : 15, unit 2 bits 0-14 */
pub const IEEE854_LONG_DOUBLE_NAN_NEGATIVE_MASK: u32 = 1 << 15;      /* ieee_nan: negative : 1, unit 2 bit 15 */
pub const IEEE854_LONG_DOUBLE_NAN_EMPTY_MASK: u32 = 0xffff0000;      /* ieee_nan: empty : 16, unit 2 bits 16-31 */

macro_rules! ieee854_long_double_bits {
  (ieee.mantissa1, $d:expr) => { ($d.ieee.units[0]) };
  (ieee.mantissa0, $d:expr) => { ($d.ieee.units[1] & IEEE854_LONG_DOUBLE_IEEE_MANTISSA0_MASK) };
  (ieee.exponent, $d:expr) => { ($d.ieee.units[2] & IEEE854_LONG_DOUBLE_IEEE_EXPONENT_MASK) };
  (ieee.negative, $d:expr) => { ($d.ieee.units[2] & IEEE854_LONG_DOUBLE_IEEE_NEGATIVE_MASK) != 0 };
  (ieee.empty, $d:expr) => { ($d.ieee.units[2] & IEEE854_LONG_DOUBLE_IEEE_EMPTY_MASK) >> 16 };
  (ieee_nan.mantissa1, $d:expr) => { ($d.ieee_nan.units[0]) };
  (ieee_nan.mantissa0, $d:expr) => { ($d.ieee_nan.units[1] & IEEE854_LONG_DOUBLE_NAN_MANTISSA0_MASK) };
  (ieee_nan.quiet_nan, $d:expr) => { ($d.ieee_nan.units[1] & IEEE854_LONG_DOUBLE_NAN_QUIET_NAN_MASK) != 0 };
  (ieee_nan.one, $d:expr) => { ($d.ieee_nan.units[1] & IEEE854_LONG_DOUBLE_NAN_ONE_MASK) != 0 };
  (ieee_nan.exponent, $d:expr) => { ($d.ieee_nan.units[2] & IEEE854_LONG_DOUBLE_NAN_EXPONENT_MASK) };
  (ieee_nan.negative, $d:expr) => { ($d.ieee_nan.units[2] & IEEE854_LONG_DOUBLE_NAN_NEGATIVE_MASK) != 0 };
  (ieee_nan.empty, $d:expr) => { ($d.ieee_nan.units[2] & IEEE854_LONG_DOUBLE_NAN_EMPTY_MASK) >> 16 };
}

