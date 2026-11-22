// femacs: port of src/category.h @ fd7afced403.
//
// WIP mirror (P-66): category.h is 124 lines with a single conditional (its include guard's
// `#ifndef`), so what is missing below is missing because the code that uses it has not
// been ported yet - category.c is the next unit.  `tools/check-cond-eval.py` reports zero
// disagreements for the header.
//
// `#include "lisp.h"` (category.h 62) has no counterpart here: a `src/` mirror never
// includes lisp.h (PORTING.md 0.1-9).

// category.h 47: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_CATEGORY_H: bool = true;

/* We introduce here three types of object: category, category set,
   and category table.

   A category is like syntax but differs in the following points:

   o A category is represented by a mnemonic character of the range
   ` '(32)..`~'(126) (printable ASCII characters).

   o A category is not exclusive, i.e. a character has multiple
   categories (category set).  Of course, there's a case that a
   category set is empty, i.e. the character has no category.

   o In addition to the predefined categories, a user can define new
   categories.  Total number of categories is limited to 95.

   A category set is a set of categories represented by Lisp
   bool-vector of length 128 (only elements of 31th through 126th
   are used).

   A category table is like syntax-table, represented by a Lisp
   char-table.  The contents are category sets or nil.  It has two
   extra slots, for a vector of doc string of each category and a
   version number.

   The first extra slot is a vector of doc strings of categories, the
   length is 95.  The Nth element corresponding to the category N+32.

   The second extra slot is a version number of the category table.
   But, for the moment, we are not using this slot.  */

/* category.h 64: `#define CATEGORYP(x) RANGED_FIXNUMP (0x20, x, 0x7E)` */
#[inline(always)]
pub fn CATEGORYP (x: Lisp_Object) -> bool { RANGED_FIXNUMP (0x20, x, 0x7E) }

/* category.h 66-67: `CHECK_TYPE (CATEGORYP (x), Qcategoryp, x)` */
#[inline(always)]
pub fn CHECK_CATEGORY (x: Lisp_Object)
{
  CHECK_TYPE (CATEGORYP (x), Qcategoryp (), x);
}

/* category.h 69-70: `(BOOL_VECTOR_P (x) && bool_vector_size (x) == 128)` */
#[inline(always)]
pub fn CATEGORY_SET_P (x: Lisp_Object) -> bool
{
  BOOL_VECTOR_P (x) && bool_vector_size (x) == 128
}

/* Return a new empty category set.  */
/* category.h 73: `#define MAKE_CATEGORY_SET (Fmake_bool_vector (make_fixnum (128), Qnil))`.
   C uses the name as an expression, so the port gives it a zero-argument function and the
   callers write `MAKE_CATEGORY_SET ()`.  */
#[inline(always)]
pub fn MAKE_CATEGORY_SET () -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { Fmake_bool_vector (make_fixnum (128), Qnil ()) }
}

/* category.h 75-76: `CHECK_TYPE (CATEGORY_SET_P (x), Qcategorysetp, x)` */
#[inline(always)]
pub fn CHECK_CATEGORY_SET (x: Lisp_Object)
{
  CHECK_TYPE (CATEGORY_SET_P (x), Qcategorysetp (), x);
}

/* Return the category set of character C in the current category table.  */
/* category.h 79: `#define CATEGORY_SET(c) char_category_set (c)` - the function itself is
   declared in lisp.h 5354 (lisp.rs), as in C.  */
#[inline(always)]
pub fn CATEGORY_SET (c: c_int) -> Lisp_Object { /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { char_category_set (c) } }

/* Return true if CATEGORY_SET contains CATEGORY.
   Faster than '!NILP (Faref (category_set, make_fixnum (category)))'.  */
#[inline(always)]
pub fn CATEGORY_MEMBER (category: EMACS_INT, category_set: Lisp_Object) -> bool
{
  bool_vector_bitref (category_set, category)
}

/* Return true if category set of CH contains CATEGORY.  */
#[inline(always)]
pub fn CHAR_HAS_CATEGORY (ch: c_int, category: c_int) -> bool
{
  let category_set: Lisp_Object = CATEGORY_SET (ch);
  CATEGORY_MEMBER (category as EMACS_INT, category_set)
}

/* The standard category table is stored where it will automatically
   be used in all new buffers.  */
// category.h 103: `#define Vstandard_category_table BVAR (&buffer_defaults, category_table)`
// is one of the token-pasting macros the port writes out at each use site; it is recorded
// in docs/ports/excluded-macros.txt (the `KVAR` precedent).

/* Return the doc string of CATEGORY in category table TABLE.  */
/* category.h 106-107: `AREF (Fchar_table_extra_slot (table, make_fixnum (0)), ((category)
   - ' '))` */
#[inline(always)]
pub fn CATEGORY_DOCSTRING (table: Lisp_Object, category: c_int) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { AREF (Fchar_table_extra_slot (table, make_fixnum (0)),
                 (category - ' ' as c_int) as isize) }
}

/* Set the doc string of CATEGORY to VALUE in category table TABLE.  */
/// category.h 110-111 is C's `ASET (…)` *assignment* macro, so the port keeps it a macro
/// (`ASET!` is lisp.rs's) and the callers write it as a statement.
macro_rules! SET_CATEGORY_DOCSTRING {
    ($table:expr, $category:expr, $value:expr) => {
        ASET (Fchar_table_extra_slot ($table, make_fixnum (0)),
               ($category) as isize - (' ' as isize), $value)
    };
}

/* Return the version number of category table TABLE.  Not used for
   the moment.  */
/* category.h 115-116: `Fchar_table_extra_slot (table, make_fixnum (1))` */
#[inline(always)]
pub fn CATEGORY_TABLE_VERSION (table: Lisp_Object) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { Fchar_table_extra_slot (table, make_fixnum (1)) }
}

/* Return true if there is a word boundary between two
   word-constituent characters C1 and C2 if they appear in this order.
   There is no word boundary between two word-constituent ASCII and
   Latin-1 characters.  */
/* category.h 122-124: `(!(SINGLE_BYTE_CHAR_P (c1) && SINGLE_BYTE_CHAR_P (c2)) &&
   word_boundary_p (c1, c2))` */
#[inline(always)]
pub fn WORD_BOUNDARY_P (c1: c_int, c2: c_int) -> bool
{
  // C's macro is applied to ints; the mirror's SINGLE_BYTE_CHAR_P takes intmax_t
  // (lisp.rs 1755), so the two arguments are widened exactly as C widens them.
  ! (SINGLE_BYTE_CHAR_P (c1 as i64) && SINGLE_BYTE_CHAR_P (c2 as i64))
  && /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { word_boundary_p (c1, c2) }
}

/* The definition is category.c's; it is declared in a module so the unit that defines it
   shadows the import instead of colliding with it (P-21/P-39).  */
pub mod category_externs {
  use super::*;

  unsafe extern "C-unwind" {
    /* category.h 120: `extern bool word_boundary_p (int, int);` */
    pub fn word_boundary_p (c1: c_int, c2: c_int) -> bool;
  }
}

pub use self::category_externs::*;
