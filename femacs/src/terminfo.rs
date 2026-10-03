// femacs: port of src/terminfo.c @ fd7afced403 .
//
// One-to-one port (PORTING.md): same statements in the same order, same names.
//
// C 29's `#ifndef TERMINFO_DEFINES_BC` guards the three globals: TERMINFO_DEFINES_BC is not in
// the frozen configuration table, so the guard is decided by the terminfo headers Emacs was
// configured against and this arm is the live text in the frozen configuration (the port
// defines the three objects here; femacs/src/tparam.rs declares them, as tparam.h does).
//
// `tparm` is the curses/terminfo entry point, declared in C just above `tparam`; the port
// declares it in the unit (the tparam.rs precedent for a libc entry).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Interface to curses/terminfo library.
   Copyright (C) 1993-1995, 1997, 2001-2026 Free Software Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

include!("include/config.rs");
include!("../lib/include/stdio.rs");
include!("include/lisp.rs");
include!("include/tparam.rs");

/* Define these variables that serve as global parameters to termcap,
   so that we do not need to conditionalize the places in Emacs
   that set them.  But don't do that if terminfo defines them, as that
   could cause link errors when using -fno-common.  */

#[unsafe(no_mangle)]
pub static mut UP: *mut c_char = std::ptr::null_mut ();
#[unsafe(no_mangle)]
pub static mut BC: *mut c_char = std::ptr::null_mut ();
#[unsafe(no_mangle)]
pub static mut PC: c_char = 0;

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
pub unsafe extern "C" fn tparam (string: *const c_char, outstring: *mut c_char, len: c_int,
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
