// femacs: port of src/composite.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// WIP mirror (PORTING.md P-66): it carries the frozen configuration's live type and
// constants.  `composition.h` has no platform branch of its own (its 28 conditions resolve
// out of `HAVE_WINDOW_SYSTEM`-style configuration plus its own include guard), and
// `tools/check-cond-eval.py` reports zero disagreements for it, so the macro and function
// ledger in docs/ports/include-composite.md is the only thing still unchecked: the
// COMPOSITION_*/LGLYPH_* accessor macros and the five prototypes land with the functions
// that use them (composite.c is batch C).
//
// C's `#include "font.h"` and `#include "dispextern.h"` (composite.h 28-29) need no
// declaration here: `struct composition` reaches the font only as `void *font`, and C's own
// comment says "the actual type is device dependent".

// composite.h 134: the include guard has no Rust counterpart (PORTING.md 0.1).
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
pub const MAX_COMPOSITION_COMPONENTS: c_int = 16;      // composite.h 51

/* Mask bits for CHECK_MASK arg to update_compositions.  */
pub const CHECK_HEAD: c_int = 1;                       // composite.h 188
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

/* Table of pointers to the structure `composition' indexed by
   COMPOSITION-ID.  */
unsafe extern "C" { pub static mut composition_table: *mut *mut composition; }   // composite.h 182
/* Number of the currently registered compositions.  */
unsafe extern "C" { pub static mut n_compositions: ptrdiff_t; }                  // composite.h 184
unsafe extern "C" { pub fn update_compositions (from: ptrdiff_t, to: ptrdiff_t,
                                                check_mask: c_int); }                 // composite.h 198

// C: composite.h 53-130's COMPOSITION_*/LGLYPH_* accessor macros and 194-198's five
// prototypes — (not mirrored yet; they land with composite.c, batch C).
