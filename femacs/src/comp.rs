// femacs: port of src/comp.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.
//
// `HAVE_NATIVE_COMP` is undefined (docs/baseline/config/darwin-arm64.macros.tsv 10), so all
// but 45 lines of this file are dead text: C 25-5578's `#ifdef HAVE_NATIVE_COMP` region (the
// `<dynlib.h>` include, the `comp` struct and its `XCOMP`/`COMP_UNIT` accessors, the Limple
// opcode and cast enums, the GIMPLE-to-Limple pass, `load_comp_unit` and the rest of the
// compiler interface) and C 5596-5854's `#ifdef HAVE_NATIVE_COMP` region inside `syms_of_comp`
// (the `native-comp-jit-compilation` DEFVAR_BOOL, the `comp--#\$` DEFVAR_LISP and the
// fifty-odd DEFSYMs).  They are recorded in docs/ports/comp.md, not transcribed (P-08).
//
// The live text is C 21-23's two includes, `Fnative_comp_available_p` (whose `#ifdef` arm is
// dead, leaving C 5588's `return Qnil;`) and `syms_of_comp`'s one `defsubr`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Compile Emacs Lisp into native code.
   Copyright (C) 2019-2026 Free Software Foundation, Inc.

Author: Andrea Corallo <acorallo@gnu.org>

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
include!("include/lisp.rs");

DEFUN! ("native-comp-available-p", Fnative_comp_available_p,
       Snative_comp_available_p, 0, 0, 0,
       doc: "Return non-nil if native compilation support is built-in.",
       (),
       {
  // C 5586-5589: `return load_gccjit_if_necessary (false) ? Qt : Qnil;` is the dead arm; the
  // live body is C 5588's `return Qnil;`.
  return Qnil ();
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_comp ()
{
  // C 5596-5854's `#ifdef HAVE_NATIVE_COMP` region is dead text (docs/ports/comp.md).
  defsubr (&raw mut Snative_comp_available_p);
}
