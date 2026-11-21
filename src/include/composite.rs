// femacs: port of src/composite.h @ fd7afced403.
//
// Mirror (PORTING.md 0.1-2): types, constants, `macro_rules!` definitions and
// `#[inline(always)]` value accessors only, plus the function/object declarations in
// `composite_decls`/`composite_objs` + glob import (PORTING.md 0.1-10/P-21, the tempname.rs
// shape).  Nothing here emits a C-visible symbol: the accessors keep Rust linkage, like the
// INLINE functions of a C header.  `composite.h` has no platform branch
// of its own (its conditions resolve out of `HAVE_WINDOW_SYSTEM`-style configuration plus
// its own include guard), and `tools/check-cond-eval.py` reports zero disagreements for it,
// so the frozen configuration's branch selection is the live text.  The whole header landed
// with composite.c (batch C): the COMPOSITION_*/LGSTRING_*/LGLYPH_* accessors, the five
// INLINE functions and the twelve `extern` prototypes.
//
// C's `#include "font.h"` and `#include "dispextern.h"` (composite.h 28-29) need no
// declaration here: `struct composition` reaches the font only as `void *font`, and C's own
// comment says "the actual type is device dependent".  The two macros whose C body names a
// font constant (`LGLYPH_CODE`'s `FONT_INVALID_CODE`) keep the name; the unit that defines
// it declares it (composite.c does, docs/ports/composite.md).

// composite.h 25-26: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_COMPOSITE_H: bool = true;

/* Methods to display a sequence of components of a composition.  */
// composite.h 34-49.
pub type composition_method = c_int;
pub const COMPOSITION_RELATIVE: composition_method = 0;
pub const COMPOSITION_WITH_RULE: composition_method = 1;
pub const COMPOSITION_WITH_ALTCHARS: composition_method = 2;
pub const COMPOSITION_WITH_RULE_ALTCHARS: composition_method = 3;
pub const COMPOSITION_NO: composition_method = 4;

/* Maximum number of components a single composition can have.  */
pub const MAX_COMPOSITION_COMPONENTS: c_int = 16;      // composite.h 50-51

/* Mask bits for CHECK_MASK arg to update_compositions.  */
pub const CHECK_HEAD: c_int = 1;                       // composite.h 188-192
pub const CHECK_TAIL: c_int = 2;
pub const CHECK_INSIDE: c_int = 4;
pub const CHECK_BORDER: c_int = CHECK_HEAD | CHECK_TAIL;
pub const CHECK_ALL: c_int = CHECK_BORDER | CHECK_INSIDE;

/* Data type for storing information about a composition, indexed by
   COMPOSITION-ID in `composition_table'.  */

// composite.h 146-178.  The layout is measured, not derived: `glyph_len` at 0, the five
// shorts at 4/6/8/10/12, `width` at 16 (two bytes of padding after the shorts), `method` at
// 20, `key` at 24, `font` at 32 and `offsets` at 40 — sizeof 48, align 8.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct composition
{
  /* How many glyphs a composition method arranges for.  */
  pub glyph_len: c_int,                                // composite.h 150
  /* Width, ascent, and descent pixels of the composition.  */
  pub pixel_width: c_short,                            // 153
  pub ascent: c_short,
  pub descent: c_short,
  pub lbearing: c_short,                               // 155
  pub rbearing: c_short,
  /* How many columns the overall glyphs occupy on the screen.  */
  pub width: c_int,                                    // 161
  /* Method of the composition.  */
  pub method: composition_method,                      // 164
  /* The key under which it's found in the composition hash table.  */
  pub key: Lisp_Object,                                // 167
  /* For which font we have calculated the remaining members.  The
     actual type is device dependent.  */
  pub font: *mut c_void,                               // 171
  /* Pointer to an array of x-offset and y-offset (by pixels) of glyphs.  */
  pub offsets: *mut c_short,                           // 178
}

/* These operations access information about a composition that
   has `composition' property PROP.  PROP is:
	((LENGTH . COMPONENTS) . MODIFICATION-FUNC)
   or
	(COMPOSITION-ID . (LENGTH COMPONENTS . MODIFICATION-FUNC))
   They don't check validity of PROP.  */

/* These operations access information about a composition that
   has `composition' property PROP.  ... (the C comment above).  Every accessor below is a
   `#[inline(always)] fn` rather than a `macro_rules!`: C's INLINE/`#define` accessors are
   value-only helpers (the task's "safe `fn` for value-only helpers"), a fn keeps C's call
   shape at every use site (`COMPOSITION_ID (prop)`, `LGLYPH_FROM (glyph)`, no `!`), and the
   `macro_body_of` comparison reads the fn's body exactly as it reads a macro's.  The two
   statement macros whose operands are *lvalues* (`COMPOSITION_DECODE_REFS`/`_RULE`) keep the
   helper-fn shape plus `*mut` parameters, because a Rust fn cannot write its caller's locals
   (PORTING.md 3.8).  */

/* Return true if PROP is already registered.  */
#[inline(always)]                                      // composite.h 59-64
pub fn composition_registered_p (prop: Lisp_Object) -> bool { return FIXNUMP (XCAR (prop)); }

/* Return ID number of the already registered composition.  */
#[inline(always)]                                      // composite.h 66-67
pub fn COMPOSITION_ID (prop: Lisp_Object) -> EMACS_INT { return XFIXNUM (XCAR (prop)); }

/* Return length of the composition.  */
#[inline(always)]                                      // composite.h 69-73
pub fn COMPOSITION_LENGTH (prop: Lisp_Object) -> EMACS_INT
{
  return (if composition_registered_p (prop)
	  { XFIXNUM (XCAR (XCDR (prop))) }
	  else { XFIXNUM (XCAR (XCAR (prop))) });
}

/* Return components of the composition.  */
#[inline(always)]                                      // composite.h 75-79
pub fn COMPOSITION_COMPONENTS (prop: Lisp_Object) -> Lisp_Object
{
  return (if composition_registered_p (prop)
	  { XCAR (XCDR (XCDR (prop))) }
	  else { XCDR (XCAR (prop)) });
}

/* Return modification function of the composition.  */
#[inline(always)]                                      // composite.h 81-85
pub fn COMPOSITION_MODIFICATION_FUNC (prop: Lisp_Object) -> Lisp_Object
{
  return (if composition_registered_p (prop)
	  { XCDR (XCDR (XCDR (prop))) }
	  else if CONSP (prop) { XCDR (prop) } else { Qnil () });
}

#[inline(always)]                                      // composite.h 87
pub fn COMPOSITION_KEY (cmp: *mut composition) -> Lisp_Object { return unsafe { (*cmp).key }; }

/* Return the Nth glyph of composition specified by CMP.  CMP is a
   pointer to `struct composition'.  */
#[inline(always)]                                      // composite.h 89-94
pub fn COMPOSITION_GLYPH (cmp: *mut composition, n: ptrdiff_t) -> EMACS_INT
{
  return XFIXNUM (AREF (COMPOSITION_KEY (cmp),
			(if unsafe { (*cmp).method } == COMPOSITION_WITH_RULE_ALTCHARS
			 { n * 2 } else { n })));
}

/* Return the encoded composition rule to compose the Nth glyph of
   rule-base composition specified by CMP.  CMP is a pointer to
   `struct composition'. */
#[inline(always)]                                      // composite.h 96-100
pub fn COMPOSITION_RULE (cmp: *mut composition, n: ptrdiff_t) -> EMACS_INT
{
  return XFIXNUM (AREF (COMPOSITION_KEY (cmp), n * 2 - 1));
}

/* Decode encoded composition rule RULE_CODE into GREF (global
   reference point code), NREF (new ref. point code).  Don't check RULE_CODE;
   always set GREF and NREF to valid values.  By side effect,
   RULE_CODE is modified.  */

pub fn COMPOSITION_DECODE_REFS (rule_code: *mut c_int, gref: *mut c_int, nref: *mut c_int)
{                                                       // composite.h 102-113
  unsafe {
    *rule_code &= 0xFF;
    *gref = *rule_code / 12;
    if *gref > 12 { *gref = 11; }
    *nref = *rule_code % 12;
  }
}

/* Like COMPOSITION_DECODE_REFS (RULE_CODE, GREF, NREF), but also
   decode RULE_CODE into XOFF and YOFF (vertical offset).  */

pub fn COMPOSITION_DECODE_RULE (rule_code: *mut c_int, gref: *mut c_int, nref: *mut c_int,
				xoff: *mut c_int, yoff: *mut c_int)
{                                                       // composite.h 115-122
  unsafe {
    *xoff = *rule_code >> 16;
    *yoff = (*rule_code >> 8) & 0xFF;
    COMPOSITION_DECODE_REFS (rule_code, gref, nref);
  }
}

/* True if the global reference point GREF and new reference point NREF are
   valid.  */
#[inline(always)]                                      // composite.h 124-129
pub fn COMPOSITION_ENCODE_RULE_VALID (gref: c_int, nref: c_int) -> bool
{
  return 0 <= gref && gref < 12 && 0 <= nref && nref < 12;
}

/* Return encoded composition rule for the pair of global reference
   point GREF and new reference point NREF.  Arguments must be valid.  */
#[inline(always)]                                      // composite.h 131-135
pub fn COMPOSITION_ENCODE_RULE (gref: c_int, nref: c_int) -> c_int { return gref * 12 + nref; }

/* Return the method of a composition with property PROP.  */

#[inline(always)]                                      // composite.h 206-222
pub fn composition_method (prop: Lisp_Object) -> composition_method
{
  if composition_registered_p (prop)
    { return unsafe { (**composition_table.add (COMPOSITION_ID (prop) as usize)).method }; }
  else
    {
      let temp = XCDR (XCAR (prop));
      return (if NILP (temp)
	      { COMPOSITION_RELATIVE }
	      else if FIXNUMP (temp) || STRINGP (temp)
	      { COMPOSITION_WITH_ALTCHARS }
	      else { COMPOSITION_WITH_RULE_ALTCHARS });
    }
}

/* Given offsets START and END, return true if PROP is a valid composition
   property with length END - START.  */

#[inline(always)]                                      // composite.h 224-240
pub fn composition_valid_p (start: ptrdiff_t, end: ptrdiff_t, prop: Lisp_Object) -> bool
{
  return (CONSP (prop)
	  && (if composition_registered_p (prop)
	      { (COMPOSITION_ID (prop) >= 0
		 && COMPOSITION_ID (prop) <= unsafe { n_compositions }
		 && CONSP (XCDR (prop))) }
	      else { (CONSP (XCAR (prop))
		      && (NILP (XCDR (XCAR (prop)))
			  || STRINGP (XCDR (XCAR (prop)))
			  || VECTORP (XCDR (XCAR (prop)))
			  || FIXNUMP (XCDR (XCAR (prop)))
			  || CONSP (XCDR (XCAR (prop))))) })
	  && COMPOSITION_LENGTH (prop) == end - start);
}

/* Macros for lispy glyph-string.  This is completely different from
   struct glyph_string.  */

/* LGSTRING is a string of font glyphs, LGLYPHs.  It is represented as
   a Lisp vector, with components shown below.  Once LGSTRING was
   processed by a shaping engine, it holds font glyphs for one or more
   grapheme clusters.  */

#[inline(always)]                                      // composite.h 251
pub fn LGSTRING_HEADER (lgs: Lisp_Object) -> Lisp_Object { return AREF (lgs, 0); }
#[inline(always)]                                      // composite.h 252
pub fn LGSTRING_SET_HEADER (lgs: Lisp_Object, header: Lisp_Object) { ASET (lgs, 0, header); }

/* LGSTRING_FONT retrieves the font used for LGSTRING, if we are going
   to display it on a GUI frame.  On text-mode frames, that slot
   stores the coding-system that should be used to write output to the
   frame's terminal.  */
#[inline(always)]                                      // composite.h 254-258
pub fn LGSTRING_FONT (lgs: Lisp_Object) -> Lisp_Object
{ return AREF (LGSTRING_HEADER (lgs), 0); }
#[inline(always)]                                      // composite.h 259
pub fn LGSTRING_CHAR (lgs: Lisp_Object, i: ptrdiff_t) -> Lisp_Object
{ return AREF (LGSTRING_HEADER (lgs), i + 1); }
#[inline(always)]                                      // composite.h 260
pub fn LGSTRING_CHAR_LEN (lgs: Lisp_Object) -> ptrdiff_t
{ return ASIZE (LGSTRING_HEADER (lgs)) - 1; }

#[inline(always)]                                      // composite.h 262
pub fn LGSTRING_SET_FONT (lgs: Lisp_Object, val: Lisp_Object)
{ ASET (LGSTRING_HEADER (lgs), 0, val); }
#[inline(always)]                                      // composite.h 263
pub fn LGSTRING_SET_CHAR (lgs: Lisp_Object, i: ptrdiff_t, c: Lisp_Object)
{ ASET (LGSTRING_HEADER (lgs), i + 1, c); }

#[inline(always)]                                      // composite.h 265
pub fn LGSTRING_ID (lgs: Lisp_Object) -> Lisp_Object { return AREF (lgs, 1); }
#[inline(always)]                                      // composite.h 266
pub fn LGSTRING_SET_ID (lgs: Lisp_Object, id: Lisp_Object) { ASET (lgs, 1, id); }

/* LGSTRING_GLYPH_LEN is the maximum number of LGLYPHs that the
   LGSTRING can hold.  This is NOT the actual number of valid LGLYPHs;
   to find the latter, walk the glyphs returned by LGSTRING_GLYPH
   until the first one that is nil.  */
#[inline(always)]                                      // composite.h 270-272
pub fn LGSTRING_GLYPH_LEN (lgs: Lisp_Object) -> ptrdiff_t { return ASIZE (lgs) - 2; }
#[inline(always)]                                      // composite.h 273
pub fn LGSTRING_GLYPH (lgs: Lisp_Object, idx: ptrdiff_t) -> Lisp_Object
{ return AREF (lgs, idx + 2); }
#[inline(always)]                                      // composite.h 274
pub fn LGSTRING_SET_GLYPH (lgs: Lisp_Object, idx: ptrdiff_t, val: Lisp_Object)
{ ASET (lgs, idx + 2, val); }
#[inline(always)]                                      // composite.h 275-279
pub fn lgstring_glyph_addr (lgs: Lisp_Object, idx: ptrdiff_t) -> *mut Lisp_Object
{
  return aref_addr (lgs, idx + 2);
}

/* Vector size of Lispy glyph.  */
pub const LGLYPH_IX_FROM: isize = 0;                   // composite.h 282-289
pub const LGLYPH_IX_TO: isize = 1;
pub const LGLYPH_IX_CHAR: isize = 2;
pub const LGLYPH_IX_CODE: isize = 3;
pub const LGLYPH_IX_WIDTH: isize = 4;
pub const LGLYPH_IX_LBEARING: isize = 5;
pub const LGLYPH_IX_RBEARING: isize = 6;
pub const LGLYPH_IX_ASCENT: isize = 7;
pub const LGLYPH_IX_DESCENT: isize = 8;
pub const LGLYPH_IX_ADJUSTMENT: isize = 9;
/* Not an index.  */
pub const LGLYPH_SIZE: isize = 10;

/* Each LGLYPH is a single font glyph, whose font code is in
   LGLYPH_CODE.  ...  */
#[inline(always)]                                      // composite.h 299
pub fn LGLYPH_NEW () -> Lisp_Object { return make_nil_vector (LGLYPH_SIZE); }
#[inline(always)]                                      // composite.h 300
pub fn LGLYPH_FROM (g: Lisp_Object) -> EMACS_INT
{ return XFIXNUM (AREF (g, LGLYPH_IX_FROM)); }
#[inline(always)]                                      // composite.h 301
pub fn LGLYPH_TO (g: Lisp_Object) -> EMACS_INT
{ return XFIXNUM (AREF (g, LGLYPH_IX_TO)); }
#[inline(always)]                                      // composite.h 302
pub fn LGLYPH_CHAR (g: Lisp_Object) -> EMACS_INT
{ return XFIXNUM (AREF (g, LGLYPH_IX_CHAR)); }
/* These two are the only accessors kept as `macro_rules!`: their C bodies name
   `FONT_INVALID_CODE`, font.h's macro, which the composite.h mirror cannot carry (it would be
   a second declaration of a font.h name in every unit that also includes font.rs).  A
   function body is resolved where the mirror is included; a macro body only where it is
   *expanded*, i.e. in composite.rs, which declares the constant at its use site
   (docs/ports/composite.md's mirror-gap list).  */
macro_rules! LGLYPH_CODE {                             // composite.h 303-306
  ($g:expr) => {
    /* C's `FONT_INVALID_CODE` is `unsigned int` and `cons_to_unsigned` returns `uintmax_t`; the
       two arms have to agree on a type here, and every use site wants the 32-bit one (status
       4.766 measured `if and else have incompatible types` in the first unit that expanded it
       with `have_window_system`).  */
    (if NILP (AREF ($g, LGLYPH_IX_CODE)) { FONT_INVALID_CODE }
     else { (unsafe { cons_to_unsigned (AREF ($g, LGLYPH_IX_CODE), u32::MAX as u64) }) as u32 })
  };
}
#[inline(always)]                                      // composite.h 307
pub fn LGLYPH_WIDTH (g: Lisp_Object) -> EMACS_INT
{ return XFIXNUM (AREF (g, LGLYPH_IX_WIDTH)); }
#[inline(always)]                                      // composite.h 308
pub fn LGLYPH_LBEARING (g: Lisp_Object) -> EMACS_INT
{ return XFIXNUM (AREF (g, LGLYPH_IX_LBEARING)); }
#[inline(always)]                                      // composite.h 309
pub fn LGLYPH_RBEARING (g: Lisp_Object) -> EMACS_INT
{ return XFIXNUM (AREF (g, LGLYPH_IX_RBEARING)); }
#[inline(always)]                                      // composite.h 310
pub fn LGLYPH_ASCENT (g: Lisp_Object) -> EMACS_INT
{ return XFIXNUM (AREF (g, LGLYPH_IX_ASCENT)); }
#[inline(always)]                                      // composite.h 311
pub fn LGLYPH_DESCENT (g: Lisp_Object) -> EMACS_INT
{ return XFIXNUM (AREF (g, LGLYPH_IX_DESCENT)); }
#[inline(always)]                                      // composite.h 312
pub fn LGLYPH_ADJUSTMENT (g: Lisp_Object) -> Lisp_Object
{ return AREF (g, LGLYPH_IX_ADJUSTMENT); }
#[inline(always)]                                      // composite.h 313
pub fn LGLYPH_SET_FROM (g: Lisp_Object, val: EMACS_INT)
{ ASET (g, LGLYPH_IX_FROM, make_fixnum (val)); }
#[inline(always)]                                      // composite.h 314
pub fn LGLYPH_SET_TO (g: Lisp_Object, val: EMACS_INT)
{ ASET (g, LGLYPH_IX_TO, make_fixnum (val)); }
#[inline(always)]                                      // composite.h 315
pub fn LGLYPH_SET_CHAR (g: Lisp_Object, val: EMACS_INT)
{ ASET (g, LGLYPH_IX_CHAR, make_fixnum (val)); }
/* Callers must assure that VAL is not negative!  */
macro_rules! LGLYPH_SET_CODE {                         // composite.h 316-319
  ($g:expr, $val:expr) => {
    ASET ($g, LGLYPH_IX_CODE,
	  if ($val as u64) == FONT_INVALID_CODE as u64 { Qnil () } else { INT_TO_INTEGER ($val as EMACS_INT) })
  };
}

#[inline(always)]                                      // composite.h 321
pub fn LGLYPH_SET_WIDTH (g: Lisp_Object, val: EMACS_INT)
{ ASET (g, LGLYPH_IX_WIDTH, make_fixnum (val)); }
#[inline(always)]                                      // composite.h 322
pub fn LGLYPH_SET_LBEARING (g: Lisp_Object, val: EMACS_INT)
{ ASET (g, LGLYPH_IX_LBEARING, make_fixnum (val)); }
#[inline(always)]                                      // composite.h 323
pub fn LGLYPH_SET_RBEARING (g: Lisp_Object, val: EMACS_INT)
{ ASET (g, LGLYPH_IX_RBEARING, make_fixnum (val)); }
#[inline(always)]                                      // composite.h 324
pub fn LGLYPH_SET_ASCENT (g: Lisp_Object, val: EMACS_INT)
{ ASET (g, LGLYPH_IX_ASCENT, make_fixnum (val)); }
#[inline(always)]                                      // composite.h 325
pub fn LGLYPH_SET_DESCENT (g: Lisp_Object, val: EMACS_INT)
{ ASET (g, LGLYPH_IX_DESCENT, make_fixnum (val)); }
#[inline(always)]                                      // composite.h 326
pub fn LGLYPH_SET_ADJUSTMENT (g: Lisp_Object, val: Lisp_Object)
{ ASET (g, LGLYPH_IX_ADJUSTMENT, val); }

#[inline(always)]                                      // composite.h 328-329
pub fn LGLYPH_XOFF (g: Lisp_Object) -> EMACS_INT
{
  return (if VECTORP (LGLYPH_ADJUSTMENT (g))
	  { XFIXNUM (AREF (LGLYPH_ADJUSTMENT (g), 0)) } else { 0 });
}
#[inline(always)]                                      // composite.h 330-331
pub fn LGLYPH_YOFF (g: Lisp_Object) -> EMACS_INT
{
  return (if VECTORP (LGLYPH_ADJUSTMENT (g))
	  { XFIXNUM (AREF (LGLYPH_ADJUSTMENT (g), 1)) } else { 0 });
}
#[inline(always)]                                      // composite.h 332-333
pub fn LGLYPH_WADJUST (g: Lisp_Object) -> EMACS_INT
{
  return (if VECTORP (LGLYPH_ADJUSTMENT (g))
	  { XFIXNUM (AREF (LGLYPH_ADJUSTMENT (g), 2)) } else { 0 });
}

// The two objects composite.h only declares (`extern`): composite.c defines them, and an
// explicit item shadows a glob import where a same-named declaration at this scope would
// be `E0428` (PORTING.md 0.1-10/P-21, the tempname.rs shape).
pub mod composite_objs {
  use super::*;

    // composite.h 180-184.
    unsafe extern "C" {
    pub static mut composition_table: *mut *mut composition;
    pub static mut n_compositions: ptrdiff_t;
    }
}
pub use self::composite_objs::*;

// Every `extern` prototype of composite.h.  All twelve are defined in composite.c, so they
// live here + glob import for the same reason (PORTING.md 0.1-10, the tempname.rs shape).
pub mod composite_decls {
  use super::*;

    unsafe extern "C" {
    pub fn get_composition_id (charpos: ptrdiff_t, bytepos: ptrdiff_t, len: ptrdiff_t,
			       prop: Lisp_Object, string: Lisp_Object) -> ptrdiff_t;
    pub fn find_composition (pos: ptrdiff_t, limit: ptrdiff_t, start: *mut ptrdiff_t,
			     end: *mut ptrdiff_t, prop: *mut Lisp_Object,
			     string: Lisp_Object) -> bool;
    pub fn update_compositions (from: ptrdiff_t, to: ptrdiff_t, check_mask: c_int);
    pub fn make_composition_value_copy (list: Lisp_Object);
    pub fn syms_of_composite ();
    pub fn mark_composite ();
    pub fn compose_text (start: ptrdiff_t, end: ptrdiff_t, components: Lisp_Object,
			 modification_func: Lisp_Object, string: Lisp_Object);

    pub fn composition_gstring_put_cache (gstring: Lisp_Object, len: ptrdiff_t)
					  -> Lisp_Object;
    pub fn composition_gstring_from_id (id: ptrdiff_t) -> Lisp_Object;
    pub fn composition_gstring_p (gstring: Lisp_Object) -> bool;
    /// C: `extern int composition_gstring_width (Lisp_Object, ptrdiff_t, ptrdiff_t, struct
    /// font_metrics *);` — `struct font_metrics` is not mirrored on this target (PORTING.md
    /// P-66), so the fourth parameter is the ABI-equal `*mut c_void` the only other caller
    /// (character.c) passes as NULL.
    pub fn composition_gstring_width (gstring: Lisp_Object, from: ptrdiff_t,
				      to: ptrdiff_t, metrics: *mut c_void) -> c_int;
    pub fn composition_gstring_adjust_zero_width (gstring: Lisp_Object);

    pub fn find_automatic_composition (pos: ptrdiff_t, limit: ptrdiff_t,
				       backlim: ptrdiff_t, start: *mut ptrdiff_t,
				       end: *mut ptrdiff_t, gstring: *mut Lisp_Object,
				       string: Lisp_Object) -> bool;

    /// C: `extern void composition_compute_stop_pos (struct composition_it *, ptrdiff_t,
    /// ptrdiff_t, ptrdiff_t, Lisp_Object, bool);` — `struct composition_it` is not mirrored
    /// (PORTING.md P-66), so the first parameter is the ABI-equal `*mut c_void`.
    pub fn composition_compute_stop_pos (cmp_it: *mut c_void, charpos: ptrdiff_t,
					 bytepos: ptrdiff_t, endpos: ptrdiff_t,
					 string: Lisp_Object, include_static: bool);
    /// C: `extern bool composition_reseat_it (struct composition_it *, ptrdiff_t,
    /// ptrdiff_t, ptrdiff_t, struct window *, signed char, struct face *, Lisp_Object);`
    /// — `struct composition_it`, `struct window` and `struct face` are not reached from this
    /// mirror (PORTING.md P-66/P-72: a mirror may only name a type it declares itself or pulls
    /// in, and casefiddle.c includes composite.h without window.h), so all three are the
    /// ABI-equal `*mut c_void`.
    pub fn composition_reseat_it (cmp_it: *mut c_void, charpos: ptrdiff_t,
				  bytepos: ptrdiff_t, endpos: ptrdiff_t, w: *mut c_void,
				  bidi_level: i8, face: *mut c_void,
				  string: Lisp_Object) -> bool;
    /// C: `extern int composition_update_it (struct composition_it *, ptrdiff_t,
    /// ptrdiff_t, Lisp_Object);`
    pub fn composition_update_it (cmp_it: *mut c_void, charpos: ptrdiff_t,
				  bytepos: ptrdiff_t, string: Lisp_Object) -> c_int;

    pub fn composition_adjust_point (last_pt: ptrdiff_t, new_pt: ptrdiff_t) -> ptrdiff_t;
    pub fn composition_gstring_lookup_cache (header: Lisp_Object) -> Lisp_Object;

    pub fn composition_gstring_cache_clear_font (font_object: Lisp_Object);
    }
}
pub use self::composite_decls::*;
