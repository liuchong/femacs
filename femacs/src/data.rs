// femacs: port of src/data.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  The 120 subrs
// use `DEFUN!` (PORTING.md 5.1).
//
// The `#include` list mirrors data.c 21-35: `<math.h>`, `<stdio.h>` and `<intprops.h>` map to the
// system mirrors (`lib/include/math.rs`, `lib/include/stdio.rs` and — for `TYPE_WIDTH`, which is
// where the port put it — `lib/include/limits.in.rs`; `<intprops.h>` is a gnulib header the port
// does not mirror in full, PORTING.md P-26/P-62), then lisp.h, bignum.h, character.h, buffer.h,
// keyboard.h, process.h, frame.h and keymap.h in C's order.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Arithmetic and comparison operations on Lisp objects.
   Copyright (C) 1985-1986, 1992-1994, 1997-2026 Free Software Foundation, Inc.

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
include!("../lib/include/math.rs");
include!("../lib/include/stdio.rs");
include!("../lib/include/limits.in.rs");
include!("../lib/include/stdlib.rs");
include!("../lib/include/stdcountof.rs");
include!("include/lisp.rs");
include!("../lib/include/mini-gmp.rs");
include!("include/bignum.rs");
include!("include/character.rs");
include!("include/itree.rs");
include!("include/systime.rs");
include!("include/dispextern.rs");
include!("include/composite.rs");
include!("include/coding.rs");
include!("include/buffer.rs");
include!("include/keyboard.rs");
include!("include/termhooks.rs");
include!("include/window.rs");
include!("include/process.rs");
include!("include/frame.rs");
include!("include/keymap.rs");

// data.c 41-78: the forwarded-variable accessors.
#[inline(always)]
fn BOOLFWDP (a: lispfwd) -> bool { XFWDTYPE (a) == Lisp_Fwd_Bool }
#[inline(always)]
fn INTFWDP (a: lispfwd) -> bool { XFWDTYPE (a) == Lisp_Fwd_Int }
#[inline(always)]
fn OBJFWDP (a: lispfwd) -> bool { XFWDTYPE (a) == Lisp_Fwd_Obj }

#[inline(always)]
fn XBOOLVAR (a: lispfwd) -> *mut bool
{
  eassert! (BOOLFWDP (a));
  unsafe { (*a).u.boolvar }
}
#[inline(always)]
fn XKBOARD_OFFSET (a: lispfwd) -> c_int
{
  eassert! (KBOARD_OBJFWDP (a));
  unsafe { (*a).u.kbdoffset }
}
#[inline(always)]
fn XINTVAR (a: lispfwd) -> *mut intmax_t
{
  eassert! (INTFWDP (a));
  unsafe { (*a).u.intvar }
}
#[inline(always)]
fn XOBJVAR (a: lispfwd) -> *mut Lisp_Object
{
  eassert! (OBJFWDP (a));
  unsafe { (*a).u.objvar }
}

// data.c 82-116: the buffer-local-value accessors.
#[inline(always)]
fn set_blv_found (blv: *mut Lisp_Buffer_Local_Value, found: c_int)
{
  eassert! (found == !BASE_EQ (unsafe { (*blv).defcell }, unsafe { (*blv).valcell }) as c_int);
  // C assigns the one-bit `found` field; the mirror packs it with `local_if_set` (lisp.rs
  // `BLV_FOUND`), so the port sets that bit.
  let bits = unsafe { (*blv).bits };
  unsafe { (*blv).bits = if found != 0 { bits | BLV_FOUND } else { bits & !BLV_FOUND }; }
}
#[inline(always)]
fn blv_value (blv: *mut Lisp_Buffer_Local_Value) -> Lisp_Object
{ XCDR (unsafe { (*blv).valcell }) }
#[inline(always)]
fn set_blv_value (blv: *mut Lisp_Buffer_Local_Value, val: Lisp_Object)
{ XSETCDR (unsafe { (*blv).valcell }, val); }
#[inline(always)]
fn set_blv_where (blv: *mut Lisp_Buffer_Local_Value, val: Lisp_Object)
{ unsafe { (*blv).r#where = val; } }
#[inline(always)]
fn set_blv_defcell (blv: *mut Lisp_Buffer_Local_Value, val: Lisp_Object)
{ unsafe { (*blv).defcell = val; } }
#[inline(always)]
fn set_blv_valcell (blv: *mut Lisp_Buffer_Local_Value, val: Lisp_Object)
{ unsafe { (*blv).valcell = val; } }

// data.c 119-127.  C's `static AVOID` is `_Noreturn ATTRIBUTE_COLD void` (lisp.h 153);
// the port keeps `-> !` and drops `ATTRIBUTE_COLD`, which only affects optimisation
// (PORTING.md 490-491).  It is *not* NO_INLINE.
unsafe fn wrong_length_argument (a1: Lisp_Object, a2: Lisp_Object, a3: Lisp_Object) -> !
{
  let size1 = make_fixnum (bool_vector_size (a1));
  let size2 = make_fixnum (bool_vector_size (a2));
  if NILP (a3)
    { xsignal2 (Qwrong_length_argument (), size1, size2) }
  else
    {
      xsignal3 (Qwrong_length_argument (), size1, size2,
                make_fixnum (bool_vector_size (a3)))
    }
}

// data.c 131-136.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wrong_type_argument (predicate: Lisp_Object, value: Lisp_Object) -> !
{
  eassert! (!TAGGEDP (value, Lisp_Type::Lisp_Type_Unused0));
  xsignal2 (Qwrong_type_argument (), predicate, value)
}

// data.c 138-155.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn args_out_of_range (a1: Lisp_Object, a2: Lisp_Object)
{ xsignal2 (Qargs_out_of_range (), a1, a2) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn args_out_of_range_3 (a1: Lisp_Object, a2: Lisp_Object, a3: Lisp_Object)
{ xsignal3 (Qargs_out_of_range (), a1, a2, a3) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn circular_list (list: Lisp_Object) { xsignal1 (Qcircular_list (), list) }

// data.c 157-165.  C marks it NO_INLINE to keep LTO from growing the code
// (PORTING.md 552: NO_INLINE is `#[inline(never)]`).
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn slow_eq (x: Lisp_Object, y: Lisp_Object) -> bool
{
  BASE_EQ (if symbols_with_pos_enabled && SYMBOL_WITH_POS_P (x) { XSYMBOL_WITH_POS_SYM (x) } else { x },
           if symbols_with_pos_enabled && SYMBOL_WITH_POS_P (y) { XSYMBOL_WITH_POS_SYM (y) } else { y })
}

DEFUN! ("eq", Feq, Seq, 2, 2, 0,
       doc: "Return t if the two args are the same Lisp object.",
       (obj1: Lisp_Object, obj2: Lisp_Object),
       {
  if EQ (obj1, obj2)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("null", Fnull, Snull, 1, 1, 0,
       doc: "Return t if OBJECT is nil, and return nil otherwise.",
       (object: Lisp_Object),
       {
  if NILP (object)
    { return Qt (); }
  return Qnil ();
});

// data.c 187-201: `type-of` needs `Fcl_type_of`, which data.c 202-310 defines; the port
// keeps C's order by defining `cl-type-of` here, once `thread.rs`/`font.rs` land (its
// `PVEC_THREAD`/`PVEC_MUTEX`/`PVEC_CONDVAR`/`PVEC_FONT` cases call `THREADP`, `MUTEXP`,
// `CONDVARP` and `FONT_SPEC_P`, defined in `thread.h` and `font.h`, which the port has not
// mirrored yet).  See docs/ports/data.md Decisions.

DEFUN! ("consp", Fconsp, Sconsp, 1, 1, 0,
       doc: "Return t if OBJECT is a cons cell.",
       (object: Lisp_Object),
       {
  if CONSP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("atom", Fatom, Satom, 1, 1, 0,
       doc: "Return t if OBJECT is not a cons cell.  This includes nil.",
       (object: Lisp_Object),
       {
  if CONSP (object)
    { return Qnil (); }
  return Qt ();
});

DEFUN! ("listp", Flistp, Slistp, 1, 1, 0,
       doc: "Return t if OBJECT is a list, that is, a cons cell or nil.\nOtherwise, return nil.",
       (object: Lisp_Object),
       {
  if CONSP (object) || NILP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("nlistp", Fnlistp, Snlistp, 1, 1, 0,
       doc: "Return t if OBJECT is not a list.  Lists include nil.",
       (object: Lisp_Object),
       {
  if CONSP (object) || NILP (object)
    { return Qnil (); }
  return Qt ();
});

DEFUN! ("bare-symbol-p", Fbare_symbol_p, Sbare_symbol_p, 1, 1, 0,
       doc: "Return t if OBJECT is a symbol, but not a symbol together with position.",
       (object: Lisp_Object),
       {
  if BARE_SYMBOL_P (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("symbol-with-pos-p", Fsymbol_with_pos_p, Ssymbol_with_pos_p, 1, 1, 0,
       doc: "Return t if OBJECT is a symbol together with position.\nIgnore `symbols-with-pos-enabled'.",
       (object: Lisp_Object),
       {
  if SYMBOL_WITH_POS_P (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("symbolp", Fsymbolp, Ssymbolp, 1, 1, 0,
       doc: "Return t if OBJECT is a symbol.",
       (object: Lisp_Object),
       {
  if SYMBOLP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("keywordp", Fkeywordp, Skeywordp, 1, 1, 0,
       doc: "Return t if OBJECT is a keyword.\nThis means that it is a symbol with a print name beginning with `:'\ninterned in the initial obarray.",
       (object: Lisp_Object),
       {
  if SYMBOLP (object)
    && SREF (SYMBOL_NAME (object), 0) == b':' as u8
    && SYMBOL_INTERNED_IN_INITIAL_OBARRAY_P (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("vectorp", Fvectorp, Svectorp, 1, 1, 0,
       doc: "Return t if OBJECT is a vector.",
       (object: Lisp_Object),
       {
  if VECTORP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("recordp", Frecordp, Srecordp, 1, 1, 0,
       doc: "Return t if OBJECT is a record.",
       (object: Lisp_Object),
       {
  if RECORDP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("stringp", Fstringp, Sstringp, 1, 1, 0,
       doc: "Return t if OBJECT is a string.",
       (object: Lisp_Object),
       {
  if STRINGP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("multibyte-string-p", Fmultibyte_string_p, Smultibyte_string_p,
       1, 1, 0,
       doc: "Return t if OBJECT is a multibyte string.\nReturn nil if OBJECT is either a unibyte string, or not a string.",
       (object: Lisp_Object),
       {
  if STRINGP (object) && STRING_MULTIBYTE (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("char-table-p", Fchar_table_p, Schar_table_p, 1, 1, 0,
       doc: "Return t if OBJECT is a char-table.",
       (object: Lisp_Object),
       {
  if CHAR_TABLE_P (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("vector-or-char-table-p", Fvector_or_char_table_p,
       Svector_or_char_table_p, 1, 1, 0,
       doc: "Return t if OBJECT is a char-table or vector.",
       (object: Lisp_Object),
       {
  if VECTORP (object) || CHAR_TABLE_P (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("bool-vector-p", Fbool_vector_p, Sbool_vector_p, 1, 1, 0,
       doc: "Return t if OBJECT is a bool-vector.",
       (object: Lisp_Object),
       {
  if BOOL_VECTOR_P (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("arrayp", Farrayp, Sarrayp, 1, 1, 0,
       doc: "Return t if OBJECT is an array (string or vector).",
       (object: Lisp_Object),
       {
  if ARRAYP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("sequencep", Fsequencep, Ssequencep, 1, 1, 0,
       doc: "Return t if OBJECT is a sequence (list or array).",
       (object: Lisp_Object),
       {
  if CONSP (object) || NILP (object) || ARRAYP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("bufferp", Fbufferp, Sbufferp, 1, 1, 0,
       doc: "Return t if OBJECT is an editor buffer.",
       (object: Lisp_Object),
       {
  if BUFFERP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("markerp", Fmarkerp, Smarkerp, 1, 1, 0,
       doc: "Return t if OBJECT is a marker (editor pointer).",
       (object: Lisp_Object),
       {
  if MARKERP (object)
    { return Qt (); }
  return Qnil ();
});

// data.c 499-508: `user-ptrp` lives inside `#ifdef HAVE_MODULES`, and HAVE_MODULES is
// undefined in the frozen configuration, so C defines neither `Fuser_ptrp` nor its
// `defsubr` (data.c 4360-4362).  Porting the text as `#[cfg(feature = "modules")]` was
// tried and reverted: with `src/data.c` now in the condition table the gate reads the
// frozen text, and an arm C does not compile is an invented item (D3) -- so the subr is
// simply not part of this target.  See docs/ports/data.md Decisions (and PORTING.md P-76
// for the `defsubr` half, which the tool still expects).

DEFUN! ("subrp", Fsubrp, Ssubrp, 1, 1, 0,
       doc: "Return t if OBJECT is a built-in or native-compiled Lisp function.\n\nSee also `primitive-function-p' and `native-comp-function-p'.",
       (object: Lisp_Object),
       {
  if SUBRP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("closurep", Fclosurep, Sclosurep,
       1, 1, 0,
       doc: "Return t if OBJECT is a function of type `closure'.",
       (object: Lisp_Object),
       {
  if CLOSUREP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("byte-code-function-p", Fbyte_code_function_p, Sbyte_code_function_p,
       1, 1, 0,
       doc: "Return t if OBJECT is a byte-compiled function object.",
       (object: Lisp_Object),
       {
  if CLOSUREP (object) && STRINGP (AREF (object, CLOSURE_CODE as isize))
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("interpreted-function-p", Finterpreted_function_p,
       Sinterpreted_function_p, 1, 1, 0,
       doc: "Return t if OBJECT is a function of type `interpreted-function'.",
       (object: Lisp_Object),
       {
  if CLOSUREP (object) && CONSP (AREF (object, CLOSURE_CODE as isize))
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("module-function-p", Fmodule_function_p, Smodule_function_p, 1, 1, NULL,
       doc: "Return t if OBJECT is a function loaded from a dynamic module.",
       (object: Lisp_Object),
       {
  return if MODULE_FUNCTIONP (object) { Qt () } else { Qnil () };
});

DEFUN! ("char-or-string-p", Fchar_or_string_p, Schar_or_string_p, 1, 1, 0,
       doc: "Return t if OBJECT is a character or a string.",
       (object: Lisp_Object),
       {
  if CHARACTERP (object) || STRINGP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("integerp", Fintegerp, Sintegerp, 1, 1, 0,
       doc: "Return t if OBJECT is an integer.",
       (object: Lisp_Object),
       {
  if INTEGERP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("integer-or-marker-p", Finteger_or_marker_p, Sinteger_or_marker_p, 1, 1, 0,
       doc: "Return t if OBJECT is an integer or a marker (editor pointer).",
       (object: Lisp_Object),
       {
  if MARKERP (object) || INTEGERP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("natnump", Fnatnump, Snatnump, 1, 1, 0,
       doc: "Return t if OBJECT is a nonnegative integer.",
       (object: Lisp_Object),
       {
  return if (if FIXNUMP (object) { 0 <= XFIXNUM (object) }
            else { BIGNUMP (object) && 0 <= mpz_sgn (xbignum_val (object)) })
           { Qt () } else { Qnil () };
});

DEFUN! ("numberp", Fnumberp, Snumberp, 1, 1, 0,
       doc: "Return t if OBJECT is a number (floating point or integer).",
       (object: Lisp_Object),
       {
  if NUMBERP (object)
    { return Qt (); }
  else
    { return Qnil (); }
});

DEFUN! ("number-or-marker-p", Fnumber_or_marker_p,
       Snumber_or_marker_p, 1, 1, 0,
       doc: "Return t if OBJECT is a number or a marker.",
       (object: Lisp_Object),
       {
  if NUMBERP (object) || MARKERP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("floatp", Ffloatp, Sfloatp, 1, 1, 0,
       doc: "Return t if OBJECT is a floating point number.",
       (object: Lisp_Object),
       {
  if FLOATP (object)
    { return Qt (); }
  return Qnil ();
});

// data.c 628-658: `threadp`, `mutexp` and `condition-variable-p` call `THREADP`,
// `MUTEXP` and `CONDVARP`, the inline predicates of `thread.h` (which the port has not
// mirrored yet: `thread.h` pulls `regex-emacs.h`, `android.h`, `sysselect.h` and
// `systhread.h`).  See docs/ports/data.md Decisions.

DEFUN! ("car", Fcar, Scar, 1, 1, 0,
       doc: "Return the car of LIST.  If LIST is nil, return nil.\nError if LIST is not nil and not a cons cell.  See also `car-safe'.\n\nSee Info node `(elisp)Cons Cells' for a discussion of related basic\nLisp concepts such as car, cdr, cons cell and list.",
       (list: Lisp_Object),
       {
  return CAR (list);
});

DEFUN! ("car-safe", Fcar_safe, Scar_safe, 1, 1, 0,
       doc: "Return the car of OBJECT if it is a cons cell, or else nil.",
       (object: Lisp_Object),
       {
  return CAR_SAFE (object);
});

DEFUN! ("cdr", Fcdr, Scdr, 1, 1, 0,
       doc: "Return the cdr of LIST.  If LIST is nil, return nil.\nError if LIST is not nil and not a cons cell.  See also `cdr-safe'.\n\nSee Info node `(elisp)Cons Cells' for a discussion of related basic\nLisp concepts such as cdr, car, cons cell and list.",
       (list: Lisp_Object),
       {
  return CDR (list);
});

DEFUN! ("cdr-safe", Fcdr_safe, Scdr_safe, 1, 1, 0,
       doc: "Return the cdr of OBJECT if it is a cons cell, or else nil.",
       (object: Lisp_Object),
       {
  return CDR_SAFE (object);
});

DEFUN! ("setcar", Fsetcar, Ssetcar, 2, 2, 0,
       doc: "Set the car of CELL to be NEWCAR.  Returns NEWCAR.",
       (cell: Lisp_Object, newcar: Lisp_Object),
       {
  CHECK_CONS (cell);
  XSETCAR (cell, newcar);
  return newcar;
});

DEFUN! ("setcdr", Fsetcdr, Ssetcdr, 2, 2, 0,
       doc: "Set the cdr of CELL to be NEWCDR.  Returns NEWCDR.",
       (cell: Lisp_Object, newcdr: Lisp_Object),
       {
  CHECK_CONS (cell);
  XSETCDR (cell, newcdr);
  return newcdr;
});

