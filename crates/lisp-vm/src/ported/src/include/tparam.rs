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
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_TPARAM_H: bool = true;

pub mod tparam_externs {
  use super::*;

  #[cfg(feature = "tty_terminfo")]
  unsafe extern "C" {
    /* tparam.h 31-37: the termcap interface.  */
    pub fn tputs (str_: *const c_char, nlines: c_int,
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
    pub static PC: VmGlobal<c_char>;
    pub static BC: VmGlobal<*mut c_char>;
    pub static UP: VmGlobal<*mut c_char>;

    /* tparam.h 48-49, the live `#ifdef TERMINFO` branch.  */
    pub fn tigetflag (id: *const c_char) -> c_int;
    pub fn tigetstr (id: *const c_char) -> *mut c_char;
  }

  /// Headless terminfo stubs used when `feature = "tty_terminfo"` is disabled so standalone
  /// builds require zero external curses libraries and export no conflicting C symbols.
  #[cfg(not(feature = "tty_terminfo"))]
  pub fn tputs (
    _str_: *const c_char,
    _nlines: c_int,
    _fn_: Option<unsafe extern "C" fn (c_int) -> c_int>,
  ) {}

  #[cfg(not(feature = "tty_terminfo"))]
  pub fn tgetent (_bp: *mut c_char, _name: *const c_char) -> c_int { -1 }

  #[cfg(not(feature = "tty_terminfo"))]
  pub fn tgetflag (_id: *const c_char) -> c_int { 0 }

  #[cfg(not(feature = "tty_terminfo"))]
  pub fn tgetnum (_id: *const c_char) -> c_int { -1 }

  #[cfg(not(feature = "tty_terminfo"))]
  pub fn tgetstr (_id: *const c_char, _area: *mut *mut c_char) -> *mut c_char {
    core::ptr::null_mut ()
  }

  #[cfg(not(feature = "tty_terminfo"))]
  pub fn tgoto (_cm: *const c_char, _hpos: c_int, _vpos: c_int) -> *mut c_char {
    core::ptr::null_mut ()
  }

  #[cfg(not(feature = "tty_terminfo"))]
  pub fn tparam (
    _string: *const c_char,
    _outstring: *mut c_char,
    _len: c_int,
    _arg0: c_int,
    _arg1: c_int,
    _arg2: c_int,
    _arg3: c_int,
  ) -> *mut c_char {
    core::ptr::null_mut ()
  }

  #[cfg(not(feature = "tty_terminfo"))]
  pub static PC: VmGlobal<c_char> = VmGlobal::new (0);
  #[cfg(not(feature = "tty_terminfo"))]
  pub static BC: VmGlobal<*mut c_char> = VmGlobal::new (core::ptr::null_mut ());
  #[cfg(not(feature = "tty_terminfo"))]
  pub static UP: VmGlobal<*mut c_char> = VmGlobal::new (core::ptr::null_mut ());

  #[cfg(not(feature = "tty_terminfo"))]
  pub fn tigetflag (_id: *const c_char) -> c_int { -1 }

  #[cfg(not(feature = "tty_terminfo"))]
  pub fn tigetstr (_id: *const c_char) -> *mut c_char { core::ptr::null_mut () }
}

pub use self::tparam_externs::*;
