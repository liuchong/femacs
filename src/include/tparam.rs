// femacs: port of src/tparam.h @ fd7afced403.
//
// tparam.h is 55 lines and this mirror is complete.  Its `#include <stdlib.h>` and
// `<attribute.h>` (lines 24-26) have no counterpart here: the unit includes the mirrors it
// needs, and the port's attributes are items, not text (PORTING.md 0.1-9).
//
// The frozen configuration defines TERMINFO (docs/baseline/config/darwin-arm64.macros.tsv),
// so the header's `#ifdef TERMINFO` block is live and its two prototypes are below.
//
// `PC`, `BC` and `UP` are termcap's globals; `BC` and `UP` are *defined* by tparam.c, so
// they are declared in a module the way include/bignum.rs declares `mpz`: the unit that
// defines them shadows the import instead of colliding with it (P-21/P-39).  The four
// termcap entry points are library functions, and `tparam`/`tgoto` are tparam.c's own.

// tparam.h 21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_TPARAM_H: bool = true;

pub mod tparam_externs {
  use super::*;

  unsafe extern "C" {
    /* tparam.h 31-37: the termcap interface.  */
    pub fn tputs (str_: *const c_char, n: c_int,
                  fn_: Option<unsafe extern "C" fn (c_int) -> c_int>);
    pub fn tgetent (bp: *mut c_char, name: *const c_char) -> c_int;
    pub fn tgetflag (id: *const c_char) -> c_int;
    pub fn tgetnum (id: *const c_char) -> c_int;
    pub fn tgetstr (id: *const c_char, area: *mut *mut c_char) -> *mut c_char;
    pub fn tgoto (cm: *const c_char, hpos: c_int, vpos: c_int) -> *mut c_char;

    /* tparam.h 39-40: tparam.c's own.  */
    pub fn tparam (string: *const c_char, outstring: *mut c_char, len: c_int,
                   arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_char;

    /* tparam.h 43-45: the three termcap globals.  */
    pub static mut PC: c_char;
    pub static mut BC: *mut c_char;
    pub static mut UP: *mut c_char;

    /* tparam.h 48-49, the live `#ifdef TERMINFO` branch.  */
    pub fn tigetflag (id: *const c_char) -> c_int;
    pub fn tigetstr (id: *const c_char) -> *mut c_char;
  }
}

use tparam_externs::*;
