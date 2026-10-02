// femacs: port of src/keymap.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// Header mirror (PORTING.md 0.1): types, constants, macros and the declarations C spells
// `extern`.  Nothing is defined here, so any unit may include it.  It needs `lisp.h` only,
// which is where `CHARACTERBITS` (lisp.h 3233) lives; C's keymap.h includes lisp.h and
// nothing else, so every includer of this mirror includes `lisp.rs` too.

/* Functions to manipulate keymaps.

   Copyright (C) 2001-2026 Free Software Foundation, Inc.

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

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const KEYMAP_H: bool = true;

/* The maximum byte size consumed by push_key_description.
   All callers should assure that at least this size of memory is
   allocated at the place pointed by the second argument.

   There are 6 modifiers, each consumes 2 chars.
   The octal form of a character code consumes
   (1 + CHARACTERBITS / 3 + 1) chars (including backslash at the head).
   We need one more byte for string terminator `\0'.  */
pub const KEY_DESCRIPTION_SIZE: c_int = (2 * 6) + 1 + (CHARACTERBITS / 3) + 1 + 1;

/// `KEYMAPP (m)` is C's macro (keymap.h 34), and the port keeps both its name and its macro
/// shape (PORTING.md 0.2: a C macro keeps its name).  Its argument is a plain Lisp_Object
/// expression that both languages evaluate exactly once, so a Rust `fn` would be
/// behaviourally equivalent; the macro is what C's `KEYMAPP` *is*, and the expansion below is
/// C's text (`(!NILP (get_keymap (m, false, false)))`).
macro_rules! KEYMAPP { ($m:expr) => { !NILP (get_keymap ($m, false, false)) }; }

/// C: `typedef void (*map_keymap_function_t)
///      (Lisp_Object key, Lisp_Object val, Lisp_Object args, void *data);`
/// (`Option<…>` is the port's NULL-able C function pointer, PORTING.md 5.2.)
pub type map_keymap_function_t = Option<unsafe extern "C" fn (key: Lisp_Object,
                                                             val: Lisp_Object,
                                                             args: Lisp_Object,
                                                             data: *mut c_void)>;

unsafe extern "C" {
  pub static mut current_global_map: Lisp_Object;

  // Parameter names come from the definitions in keymap.c (keymap.h names no parameter
  // except `map_keymap_canonical`'s).
  pub fn push_key_description (ch: EMACS_INT, p: *mut c_char) -> *mut c_char;
  pub fn access_keymap (map: Lisp_Object, idx: Lisp_Object, t_ok: bool,
                        noinherit: bool) -> Lisp_Object;
  pub fn get_keymap (object: Lisp_Object, error_if_not_keymap: bool,
                     autoload: bool) -> Lisp_Object;
  pub fn current_minor_maps (modeptr: *mut *mut Lisp_Object,
                             mapptr: *mut *mut Lisp_Object) -> ptrdiff_t;
  pub fn initial_define_lispy_key (keymap: Lisp_Object, keyname: *const c_char,
                                   defname: *const c_char);
  pub fn syms_of_keymap ();
  pub fn map_keymap (map: Lisp_Object, fun: map_keymap_function_t, args: Lisp_Object,
                     data: *mut c_void, autoload: bool);
  pub fn map_keymap_canonical (map: Lisp_Object, fun: map_keymap_function_t,
                               args: Lisp_Object, data: *mut c_void);
}

