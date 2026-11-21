// femacs: port of src/terminfo.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.
//
// C 29's `#ifndef TERMINFO_DEFINES_BC` guards the three globals: TERMINFO_DEFINES_BC is not in
// the frozen configuration table, so the guard is decided by the terminfo headers C was
// configured against and this arm is the live text in the frozen configuration (the port
// defines the three objects here; src/tparam.rs declares them, as tparam.h does).
//
// `tparm` is the curses/terminfo entry point, declared in C just above `tparam`; the port
// declares it in the unit (the tparam.rs precedent for a libc entry).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/tparam.rs imported via common

/* Define these variables that serve as global parameters to termcap,
   so that we do not need to conditionalize the places in Emacs
   that set them.  But don't do that if terminfo defines them, as that
   could cause link errors when using -fno-common.  */

/* C 29-31's `#ifndef TERMINFO_DEFINES_BC` arm is **dead** in the frozen configuration:
   `build/emacs-ref/src/config.h` 2358 has `#define TERMINFO_DEFINES_BC 1` (the table in
   docs/baseline/config/darwin-arm64.macros.tsv 3740 agrees, and `check-cond-eval.py
   src/terminfo.c` reports `1 conditions, 0 unknown, 0 disagreements`), so `UP`, `BC` and `PC`
   come from the terminfo library and this file defines none of them - the reference object
   proves it, exporting only `_tparam` where the port used to export `_UP`/`_BC`/`_PC`
   (docs/status.md 3.72).  `cm.rs`'s `UP = 0; BC = 0;` assigns to the library's objects exactly
   as C's `cm.c` does.  */

/* Interface to curses/terminfo library.
   Turns out that all of the terminfo-level routines look
   like their termcap counterparts except for tparm, which replaces
   tgoto.  Not only is the calling sequence different, but the string
   format is different too.
*/

unsafe extern "C" {
  pub fn tparm (str: *const c_char, ...) -> *mut c_char;
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn tparam (string: *const c_char, outstring: *mut c_char, len: c_int,
				 arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int)
  -> *mut c_char
{
  let temp: *mut c_char;

  /* Emacs always should pass a null OUTSTRING and zero LEN.  */
  if !outstring.is_null () || len != 0
    { emacs_abort (); }

  temp = tparm (string, arg1, arg2, arg3, arg4);
  return xstrdup (temp);
}
