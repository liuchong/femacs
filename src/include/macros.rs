// femacs: port of src/macros.h @ fd7afced403.
//
// macros.h is 47 lines and this mirror is complete: two file-scope objects and four
// prototypes.  Its `#include "lisp.h"` (macros.h 24) has no counterpart here, because a
// `src/` mirror never includes lisp.h - the unit that owns it includes both in C's order
// (PORTING.md 0.1-9).
//
// The two objects are macros.c's (`EMACS_INT executing_kbd_macro_iterations` and
// `Lisp_Object executing_kbd_macro`), so they are declared in a module the way
// include/bignum.rs declares `mpz`: the unit that *defines* them shadows the import instead
// of colliding with it (P-21/P-39).

// macros.h 22: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_MACROS_H: bool = true;

pub mod macros_externs {
  use super::*;

  unsafe extern "C" {
    /* macros.h 30: `extern EMACS_INT executing_kbd_macro_iterations;` */
    pub static mut executing_kbd_macro_iterations: EMACS_INT;
    /* macros.h 32: `extern Lisp_Object executing_kbd_macro;` */
    pub static mut executing_kbd_macro: Lisp_Object;

    /* macros.h 36, 41, 46: the three kbd-macro entry points keyboard.c calls.  */
    pub fn end_kbd_macro ();
    pub fn finalize_kbd_macro_chars ();
    pub fn store_kbd_macro_char (c: Lisp_Object);
    /* macros.h 51: `extern bool at_end_of_macro_p (void);` */
    pub fn at_end_of_macro_p () -> bool;
  }
}

use macros_externs::*;
