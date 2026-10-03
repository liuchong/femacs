// femacs: port of src/data.c @ fd7afced403.
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
// C's `switch`es carry a `default: eassume (false)` arm; the port's `match`es list every
// enumerator *and* that arm, so the catch-all is unreachable by construction (P-78).
#![allow(unreachable_patterns)]

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
// composite.h 28 includes "font.h", which data.c reaches through
// keyboard.h -> coding.h -> composite.h (keyboard.h 23, coding.h 284).
include!("include/font.rs");
include!("include/composite.rs");
include!("include/coding.rs");
include!("include/buffer.rs");
include!("include/keyboard.rs");
include!("include/termhooks.rs");
include!("include/window.rs");
include!("include/process.rs");
include!("include/frame.rs");
include!("include/keymap.rs");

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
// port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
// defines it here (status §3.29, the casetab.rs shape).
#[unsafe(no_mangle)]
pub static mut Vmost_positive_fixnum: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vmost_negative_fixnum: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut symbols_with_pos_enabled: bool = false;

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

DEFUN! ("type-of", Ftype_of, Stype_of, 1, 1, 0,
       doc: "Return a symbol representing the type of OBJECT.\nThe symbol returned names the object's basic type;\nfor example, (type-of 1) returns `integer'.\nContrary to `cl-type-of', the returned type is not always the most\nprecise type possible, because instead this function tries to preserve\ncompatibility with the return value of previous Emacs versions.",
       (object: Lisp_Object),
       {
  return if SYMBOLP (object) { Qsymbol () }
         else if INTEGERP (object) { Qinteger () }
         else if SUBRP (object) { Qsubr () }
         else { Fcl_type_of (object) };
});

DEFUN! ("cl-type-of", Fcl_type_of, Scl_type_of, 1, 1, 0,
       doc: "Return a symbol representing the type of OBJECT.\nThe returned symbol names the most specific possible type of the object.\nfor example, (cl-type-of nil) returns `null'.\nThe specific type returned may change depending on Emacs versions,\nso we recommend you use `cl-typep', `cl-typecase', or other predicates\nrather than compare the return value of this function against\na fixed set of types.",
       (object: Lisp_Object),
       {
  #[allow(unreachable_patterns)]
  match XTYPE (object)
    {
    Lisp_Type::Lisp_Int0 | Lisp_Type::Lisp_Int1 =>
      { return Qfixnum (); }

    Lisp_Type::Lisp_Symbol =>
      {
        return if NILP (object) { Qnull () }
               else if EQ (object, Qt ()) { Qboolean () }
               else { Qsymbol () };
      }

    Lisp_Type::Lisp_String =>
      { return Qstring (); }

    Lisp_Type::Lisp_Cons =>
      { return Qcons (); }

    Lisp_Type::Lisp_Vectorlike =>
      {
        /* WARNING!!  Keep 'cl--type-hierarchy' in sync with this code!!  */
        match PSEUDOVECTOR_TYPE (XVECTOR (object))
          {
          pvec_type::PVEC_NORMAL_VECTOR => { return Qvector (); }
          pvec_type::PVEC_BIGNUM => { return Qbignum (); }
          pvec_type::PVEC_MARKER => { return Qmarker (); }
          pvec_type::PVEC_SYMBOL_WITH_POS => { return Qsymbol_with_pos (); }
          pvec_type::PVEC_OVERLAY => { return Qoverlay (); }
          pvec_type::PVEC_FINALIZER => { return Qfinalizer (); }
          pvec_type::PVEC_USER_PTR => { return Quser_ptr (); }
          pvec_type::PVEC_WINDOW_CONFIGURATION => { return Qwindow_configuration (); }
          pvec_type::PVEC_PROCESS => { return Qprocess (); }
          pvec_type::PVEC_WINDOW => { return Qwindow (); }
          pvec_type::PVEC_SUBR =>
            {
              return if (unsafe { (*XSUBR (object)).max_args }) as isize == UNEVALLED
                     { Qspecial_form () }
                     else if NATIVE_COMP_FUNCTIONP (object) { Qnative_comp_function () }
                     else { Qprimitive_function () };
            }
          pvec_type::PVEC_CLOSURE =>
            {
              return if CONSP (AREF (object, CLOSURE_CODE as isize))
                     { Qinterpreted_function () } else { Qbyte_code_function () };
            }
          pvec_type::PVEC_BUFFER => { return Qbuffer (); }
          pvec_type::PVEC_CHAR_TABLE => { return Qchar_table (); }
          pvec_type::PVEC_BOOL_VECTOR => { return Qbool_vector (); }
          pvec_type::PVEC_FRAME => { return Qframe (); }
          pvec_type::PVEC_HASH_TABLE => { return Qhash_table (); }
          pvec_type::PVEC_OBARRAY => { return Qobarray (); }
          pvec_type::PVEC_FONT =>
            {
              if FONT_SPEC_P (object)
                { return Qfont_spec (); }
              if FONT_ENTITY_P (object)
                { return Qfont_entity (); }
              if FONT_OBJECT_P (object)
                { return Qfont_object (); }
              else
                { emacs_abort (); } /* return Qfont?  */
            }
          pvec_type::PVEC_THREAD => { return Qthread (); }
          pvec_type::PVEC_MUTEX => { return Qmutex (); }
          pvec_type::PVEC_CONDVAR => { return Qcondition_variable (); }
          pvec_type::PVEC_TERMINAL => { return Qterminal (); }
          pvec_type::PVEC_RECORD =>
            {
              let t_: Lisp_Object = AREF (object, 0);
              if RECORDP (t_) && 1 < PVSIZE (t_)
                /* Return the type name field of the class!  */
                { return AREF (t_, 1); }
              else
                { return t_; }
            }
          pvec_type::PVEC_MODULE_FUNCTION => { return Qmodule_function (); }
          pvec_type::PVEC_NATIVE_COMP_UNIT => { return Qnative_comp_unit (); }
          pvec_type::PVEC_XWIDGET => { return Qxwidget (); }
          pvec_type::PVEC_XWIDGET_VIEW => { return Qxwidget_view (); }
          pvec_type::PVEC_TS_PARSER => { return Qtreesit_parser (); }
          pvec_type::PVEC_TS_NODE => { return Qtreesit_node (); }
          pvec_type::PVEC_TS_COMPILED_QUERY => { return Qtreesit_compiled_query (); }
          pvec_type::PVEC_SQLITE => { return Qsqlite (); }
          pvec_type::PVEC_SUB_CHAR_TABLE => { return Qsub_char_table (); }
          /* "Impossible" cases.  */
          pvec_type::PVEC_MISC_PTR
            | pvec_type::PVEC_OTHER
            | pvec_type::PVEC_FREE => { }
          _ => { }
          }
        emacs_abort ();
      }

    Lisp_Type::Lisp_Float =>
      { return Qfloat (); }

    _ => { emacs_abort (); }
    }
});


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

DEFUN! ("threadp", Fthreadp, Sthreadp, 1, 1, 0,
       doc: "Return t if OBJECT is a thread.",
       (object: Lisp_Object),
       {
  if THREADP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("mutexp", Fmutexp, Smutexp, 1, 1, 0,
       doc: "Return t if OBJECT is a mutex.",
       (object: Lisp_Object),
       {
  if MUTEXP (object)
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("condition-variable-p", Fcondition_variable_p, Scondition_variable_p,
       1, 1, 0,
       doc: "Return t if OBJECT is a condition variable.",
       (object: Lisp_Object),
       {
  if CONDVARP (object)
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

DEFUN! ("boundp", Fboundp, Sboundp, 1, 1, 0,
       doc: "Return t if SYMBOL's value is not void.\nNote that if `lexical-binding' is in effect, this refers to the\nglobal value outside of any lexical scope.",
       (symbol: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  let mut sym: *mut Lisp_Symbol = XSYMBOL (symbol);

  // C's `start:` label and its `goto start` (data.c 730, 736) become this loop.
  let valcontents = loop
    {
      match unsafe { ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT }
        {
          SYMBOL_PLAINVAL => { break SYMBOL_VAL (sym); }
          SYMBOL_VARALIAS => { sym = SYMBOL_ALIAS (sym); continue; }
          SYMBOL_LOCALIZED =>
            {
              let blv = SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value;
              if ! unsafe { (*blv).fwd }.is_null ()
                /* In set_internal, we un-forward vars when their value is
                   set to Qunbound.  */
                { return Qt (); }
              else
                {
                  swap_in_symval_forwarding (sym, blv);
                  break blv_value (blv);
                }
            }
          SYMBOL_FORWARDED =>
            /* In set_internal, we un-forward vars when their value is
               set to Qunbound.  */
            { return Qt (); }
          _ => emacs_abort (),
        }
    };

  return if BASE_EQ (valcontents, Qunbound ()) { Qnil () } else { Qt () };
});

DEFUN! ("fboundp", Ffboundp, Sfboundp, 1, 1, 0,
       doc: "Return t if SYMBOL's function definition is not nil.",
       (symbol: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  return if NILP (unsafe { (*XSYMBOL (symbol)).u.s.function }) { Qnil () } else { Qt () };
});

DEFUN! ("makunbound", Fmakunbound, Smakunbound, 1, 1, 0,
       doc: "Empty out the value cell of SYMBOL, making it void as a variable.\nReturn SYMBOL.\nWhen applied to a variable alias, it undoes the `defvaralias'.\n\nIf a variable is void, trying to evaluate the variable signals a\n`void-variable' error, instead of returning a value.  For more\ndetails, see Info node `(elisp) Void Variables'.\n\nSee also `fmakunbound'.",
       (symbol: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  if SYMBOL_CONSTANT_P (symbol)
    { xsignal1 (Qsetting_constant (), symbol); }
  if unsafe { ((*XSYMBOL (symbol)).u.s.bits & SYMBOL_REDIRECT_MASK)
               >> SYMBOL_REDIRECT_SHIFT } == SYMBOL_VARALIAS
    {
      // C assigns the bitfield `redirect`; the mirror packs it with the other
      // symbol bits (lisp.rs 1408-1500), so the port sets that field.
      let sym = XSYMBOL (symbol);
      let bits = unsafe { (*sym).u.s.bits };
      unsafe { (*sym).u.s.bits = (bits & !SYMBOL_REDIRECT_MASK)
                                 | (SYMBOL_PLAINVAL << SYMBOL_REDIRECT_SHIFT); }
      SET_SYMBOL_VAL (XSYMBOL (symbol), Qunbound ());
    }
  else
    { Fset (symbol, Qunbound ()); }
  return symbol;
});

DEFUN! ("fmakunbound", Ffmakunbound, Sfmakunbound, 1, 1, 0,
       doc: "Make SYMBOL's function definition be nil.\nReturn SYMBOL.\n\nIf a function definition is nil, trying to call a function by\nthat name will cause a `void-function' error.  For more details, see\nInfo node `(elisp) Function Cells'.\n\nSee also `makunbound'.",
       (symbol: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  if NILP (symbol) || EQ (symbol, Qt ())
    { xsignal1 (Qsetting_constant (), symbol); }
  set_symbol_function (symbol, Qnil ());
  return symbol;
});

DEFUN! ("symbol-function", Fsymbol_function, Ssymbol_function, 1, 1, 0,
       doc: "Return SYMBOL's function definition.",
       (symbol: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  return unsafe { (*XSYMBOL (symbol)).u.s.function };
});

DEFUN! ("symbol-plist", Fsymbol_plist, Ssymbol_plist, 1, 1, 0,
       doc: "Return SYMBOL's property list.",
       (symbol: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  return unsafe { (*XSYMBOL (symbol)).u.s.plist };
});

DEFUN! ("symbol-name", Fsymbol_name, Ssymbol_name, 1, 1, 0,
       doc: "Return SYMBOL's name, a string.\n\nWarning: never alter the string returned by `symbol-name'.\nDoing that might make Emacs dysfunctional, and might even crash Emacs.",
       (symbol: Lisp_Object),
       {
  let name: Lisp_Object;

  CHECK_SYMBOL (symbol);
  name = SYMBOL_NAME (symbol);
  return name;
});

DEFUN! ("bare-symbol", Fbare_symbol, Sbare_symbol, 1, 1, 0,
       doc: "Extract, if need be, the bare symbol from SYM.\nSYM must be either a symbol or a symbol with position, otherwise\nthe function will signal the `wrong-type-argument' error.\nThis function ignores `symbols-with-pos-enabled'.",
       (sym: Lisp_Object),
       {
  if BARE_SYMBOL_P (sym)
    { return sym; }
  if SYMBOL_WITH_POS_P (sym)
    { return XSYMBOL_WITH_POS_SYM (sym); }
  wrong_type_argument (list2 (Qsymbolp (), Qsymbol_with_pos_p ()), sym);
});

DEFUN! ("symbol-with-pos-pos", Fsymbol_with_pos_pos, Ssymbol_with_pos_pos, 1, 1, 0,
       doc: "Extract the position from the symbol with position SYMPOS.\nIgnore `symbols-with-pos-enabled'.",
       (sympos: Lisp_Object),
       {
  CHECK_TYPE (SYMBOL_WITH_POS_P (sympos), Qsymbol_with_pos_p (), sympos);
  return XSYMBOL_WITH_POS_POS (sympos);
});

DEFUN! ("remove-pos-from-symbol", Fremove_pos_from_symbol,
       Sremove_pos_from_symbol, 1, 1, 0,
       doc: "If ARG is a symbol with position, return it without the position.\nOtherwise, return ARG unchanged.  Ignore `symbols-with-pos-enabled'.\nCompare with `bare-symbol', which does the same, but signals an error\nif ARG is not a symbol.",
       (arg: Lisp_Object),
       {
  if SYMBOL_WITH_POS_P (arg)
    { return XSYMBOL_WITH_POS_SYM (arg); }
  return arg;
});

DEFUN! ("position-symbol", Fposition_symbol, Sposition_symbol, 2, 2, 0,
       doc: "Make a new symbol with position.\nSYM is a symbol, with or without position, the symbol to position.\nPOS, the position, is either a nonnegative fixnum,\nor a symbol with position from which the position will be taken.\nIgnore `symbols-with-pos-enabled'.",
       (sym: Lisp_Object, pos: Lisp_Object),
       {
  let bare = Fbare_symbol (sym);
  let position: Lisp_Object;

  if FIXNUMP (pos)
    { position = pos; }
  else if SYMBOL_WITH_POS_P (pos)
    { position = XSYMBOL_WITH_POS_POS (pos); }
  else
    { wrong_type_argument (Qfixnum_or_symbol_with_pos_p (), pos); }

  return build_symbol_with_pos (bare, position);
});

DEFUN! ("fset", Ffset, Sfset, 2, 2, 0,
       doc: "Set SYMBOL's function definition to DEFINITION, and return DEFINITION.\nIf the resulting chain of function definitions would contain a loop,\nsignal a `cyclic-function-indirection' error.",
       (symbol: Lisp_Object, definition: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  /* Perhaps not quite the right error signal, but seems good enough.  */
  if NILP (symbol) && ! NILP (definition)
    /* There are so many other ways to shoot oneself in the foot, I don't
       think this one little sanity check is worth its cost, but anyway.  */
    { xsignal1 (Qsetting_constant (), symbol); }

  eassert! (valid_lisp_object_p (definition) != 0);

  /* Ensure non-circularity.  */
  {
    let mut s = definition;
    while SYMBOLP (s) && ! NILP (s)
      {
        if EQ (s, symbol)
          { xsignal1 (Qcyclic_function_indirection (), symbol); }
        s = unsafe { (*XSYMBOL (s)).u.s.function };
      }
  }

  // C gates this block on `#ifdef HAVE_NATIVE_COMP`; HAVE_NATIVE_COMP is undefined in
  // the frozen configuration, so the port keeps the text under the same condition
  // (PORTING.md P-08/P-51).  `native_comp` is what `xtask dead-arms` flips.
  #[cfg(feature = "native_comp")]
  {
    let function = unsafe { (*XSYMBOL (symbol)).u.s.function };

    if ! NILP (unsafe { Vnative_comp_enable_subr_trampolines })
      && SUBRP (function)
      && ! NATIVE_COMP_FUNCTIONP (function)
      && ! EQ (definition, Fsymbol_function (symbol))
      { calln! (Qcomp_subr_trampoline_install (), symbol); }
  }

  set_symbol_function (symbol, definition);

  return definition;
});

DEFUN! ("setplist", Fsetplist, Ssetplist, 2, 2, 0,
       doc: "Set SYMBOL's property list to NEWPLIST, and return NEWPLIST.",
       (symbol: Lisp_Object, newplist: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);
  set_symbol_plist (symbol, newplist);
  return newplist;
});

DEFUN! ("subr-arity", Fsubr_arity, Ssubr_arity, 1, 1, 0,
       doc: "Return minimum and maximum number of args allowed for SUBR.\nSUBR must be a built-in function.\nThe returned value is a pair (MIN . MAX).  MIN is the minimum number\nof args.  MAX is the maximum number or the symbol `many', for a\nfunction with `&rest' args, or `unevalled' for a special form.",
       (subr: Lisp_Object),
       {
  let minargs: c_short;
  let maxargs: c_short;
  CHECK_SUBR (subr);
  minargs = (*XSUBR (subr)).min_args;
  maxargs = (*XSUBR (subr)).max_args;
  return Fcons (make_fixnum (minargs as EMACS_INT),
                if maxargs as isize == MANY { Qmany () }
                else if maxargs as isize == UNEVALLED { Qunevalled () }
                else { make_fixnum (maxargs as EMACS_INT) });
});

DEFUN! ("subr-name", Fsubr_name, Ssubr_name, 1, 1, 0,
       doc: "Return name of subroutine SUBR.\nSUBR must be a built-in function.",
       (subr: Lisp_Object),
       {
  let name: *const c_char;
  CHECK_SUBR (subr);
  name = (*XSUBR (subr)).symbol_name;
  return build_string (name);
});

DEFUN! ("native-comp-function-p", Fnative_comp_function_p, Snative_comp_function_p, 1, 1,
       0,
       doc: "Return t if the object is native-compiled Lisp function, nil otherwise.",
       (object: Lisp_Object),
       {
  return if NATIVE_COMP_FUNCTIONP (object) { Qt () } else { Qnil () };
});

DEFUN! ("subr-native-lambda-list", Fsubr_native_lambda_list,
       Ssubr_native_lambda_list, 1, 1, 0,
       doc: "Return the lambda list for a native-compiled lisp/d\nfunction or t otherwise.",
       (subr: Lisp_Object),
       {
  CHECK_SUBR (subr);

  // C gates this block on `#ifdef HAVE_NATIVE_COMP`; HAVE_NATIVE_COMP is undefined in the
  // frozen configuration, so the port keeps the text under the same condition
  // (PORTING.md P-08/P-51), and `xtask dead-arms` is what compiles it.
  #[cfg(feature = "native_comp")]
  {
    if NATIVE_COMP_FUNCTION_DYNP (subr)
      { return (*XSUBR (subr)).lambda_list; }
  }
  return Qt ();
});

DEFUN! ("subr-type", Fsubr_type,
       Ssubr_type, 1, 1, 0,
       doc: "Return the type of SUBR.",
       (subr: Lisp_Object),
       {
  CHECK_SUBR (subr);
  #[cfg(feature = "native_comp")]
  {
    return SUBR_TYPE (subr);
  }
  #[cfg(not(feature = "native_comp"))]
  {
    return Qnil ();
  }
});

// data.c 1104-1134: `subr-native-comp-unit`, `native-comp-unit-file` and
// `native-comp-unit-set-file` sit inside `#ifdef HAVE_NATIVE_COMP`, undefined in the
// frozen configuration, so the reference has no such subrs and the port has none either
// (the `user-ptrp` rule, docs/ports/data.md Decisions).  Their `defsubr` calls are
// guarded by the same `#ifdef`, but `tools/extract-abi.py`'s defsubr extraction does not
// consult the condition table yet (PORTING.md P-76), so `syms_of_data` must record those
// three lines as a ledger deviation instead of writing calls the frozen build never
// compiles.

DEFUN! ("interactive-form", Finteractive_form, Sinteractive_form, 1, 1, 0,
       doc: "Return the interactive form of CMD or nil if none.\nIf CMD is not a command, the return value is nil.\nValue, if non-nil, is a list (interactive SPEC).",
       (cmd: Lisp_Object),
       {
  let mut fun = indirect_function (cmd);
  let mut genfun: bool = false;

  if NILP (fun)
    { return Qnil (); }

  /* Use an `interactive-form' property if present, analogous to the
     function-documentation property.  */
  fun = cmd;
  while SYMBOLP (fun)
    {
      let tmp = Fget (fun, Qinteractive_form ());
      if ! NILP (tmp)
        { return tmp; }
      else
        { fun = Fsymbol_function (fun); }
    }

  if SUBRP (fun)
    {
      if NATIVE_COMP_FUNCTIONP (fun) && ! NILP ((*XSUBR (fun)).intspec.native)
        { return (*XSUBR (fun)).intspec.native; }

      let spec: *const c_char = (*XSUBR (fun)).intspec.string;
      if ! spec.is_null ()
        {
          return list2 (Qinteractive (),
                        if *spec != b'(' as c_char
                          { build_string (spec) }
                        else
                          { Fcar (Fread_from_string (build_string (spec), Qnil (), Qnil ())) });
        }
    }
  else if CLOSUREP (fun)
    {
      if PVSIZE (fun) > CLOSURE_INTERACTIVE as isize
        {
          let form = AREF (fun, CLOSURE_INTERACTIVE as isize);
          /* The vector form is the new form, where the first
             element is the interactive spec, and the second is the
             command modes. */
          return list2 (Qinteractive (), if VECTORP (form) { AREF (form, 0) } else { form });
        }
      else if PVSIZE (fun) > CLOSURE_DOC_STRING as isize
        {
          let doc = AREF (fun, CLOSURE_DOC_STRING as isize);
          /* An invalid "docstring" is a sign that we have an OClosure.  */
          genfun = ! (NILP (doc) || VALID_DOCSTRING_P (doc));
        }
    }
  // C's `#ifdef HAVE_MODULES` arm (`module_function_interactive_form`) is dead in the
  // frozen configuration and the port does not carry it: the declaration lives in
  // emacs-module.h, which is not mirrored (the `user-ptrp` rule; see the ledger).
  else if AUTOLOADP (fun)
    { return Finteractive_form (Fautoload_do_load (fun, cmd, Qnil ())); }
  else if CONSP (fun)
    {
      let funcar = XCAR (fun);
      if EQ (funcar, Qlambda ())
        {
          let form = Fcdr (XCDR (fun));
          let spec = Fassq (Qinteractive (), form);
          if NILP (Fcdr (Fcdr (spec)))
            { return spec; }
          else
            { return list2 (Qinteractive (), Fcar (Fcdr (spec))); }
        }
    }
  if genfun
      /* Avoid burping during bootstrap.  */
      && ! NILP (Fsymbol_function (Qoclosure_interactive_form ()))
    { return calln! (Qoclosure_interactive_form (), fun); }
  else
    { return Qnil (); }
});

DEFUN! ("command-modes", Fcommand_modes, Scommand_modes, 1, 1, 0,
       doc: "Return the modes COMMAND is defined for.\nIf COMMAND is not a command, the return value is nil.\nThe value, if non-nil, is a list of mode name symbols.",
       (command: Lisp_Object),
       {
  let mut fun = indirect_function (command);

  if NILP (fun)
    { return Qnil (); }

  /* Use a `command-modes' property if present, analogous to the
     function-documentation property.  */
  fun = command;
  while SYMBOLP (fun)
    {
      let modes = Fget (fun, Qcommand_modes ());
      if ! NILP (modes)
        { return modes; }
      else
        { fun = Fsymbol_function (fun); }
    }

  if SUBRP (fun)
    {
      return (*XSUBR (fun)).command_modes;
    }
  else if CLOSUREP (fun)
    {
      if PVSIZE (fun) <= CLOSURE_INTERACTIVE as isize
        { return Qnil (); }
      let form = AREF (fun, CLOSURE_INTERACTIVE as isize);
      if VECTORP (form)
        /* New form -- the second element is the command modes. */
        { return AREF (form, 1); }
      else
        /* Old .elc file -- no command modes. */
        { return Qnil (); }
    }
  // `#ifdef HAVE_MODULES` arm omitted, as in interactive-form above.
  else if AUTOLOADP (fun)
    {
      let modes = Fnth (make_int (3), fun);
      if CONSP (modes)
        { return modes; }
      else
        { return Qnil (); }
    }
  else if CONSP (fun)
    {
      let funcar = XCAR (fun);
      if EQ (funcar, Qlambda ())
        {
          let form = Fcdr (XCDR (fun));
          return Fcdr (Fcdr (Fassq (Qinteractive (), form)));
        }
    }
  return Qnil ();
});

/* Getting and Setting Values of Symbols */

DEFUN! ("indirect-variable", Findirect_variable, Sindirect_variable, 1, 1, 0,
       doc: "Return the variable at the end of OBJECT's variable chain.\nIf OBJECT is a symbol, follow its variable indirections (if any), and\nreturn the variable at the end of the chain of aliases.  See Info node\n`(elisp)Variable Aliases'.\n\nIf OBJECT is not a symbol, just return it.",
       (object: Lisp_Object),
       {
  // C rebinds its parameter (`XSETSYMBOL (object, sym)`) and returns it; `DEFUN!`'s
  // pattern takes plain bindings, so the port takes the same value into a mutable one.
  // It has to be *this* scope: with the binding inside the `if` the write would land on a
  // shadow and the return value would stay the parameter (review B's P1).
  let mut object = object;
  if SYMBOLP (object)
    {
      let mut sym: *mut Lisp_Symbol = XSYMBOL (object);
      while ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT == SYMBOL_VARALIAS
        { sym = SYMBOL_ALIAS (sym); }
      XSETSYMBOL! (object, sym);
    }
  return object;
});

/* Return the KBOARD to which bindings currently established and values
   set should apply.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kboard_for_bindings () -> *mut KBOARD
{
  /* We used to simply use current_kboard here, but from Lisp code, its
     value is often unexpected.  It seems nicer to allow constructions
     like this to work as intuitively expected:

     (with-selected-frame frame
     (define-key local-function-map "\eOP" [f1]))

     On the other hand, this affects the semantics of last-command and
     real-last-command, and people may rely on that.  I took a quick
     look at the Lisp codebase, and I don't think anything will break.
     --lorentey */

  return FRAME_KBOARD (SELECTED_FRAME ()) as *mut KBOARD;
}

/* Given the raw contents of a symbol value cell,
   return the Lisp value of the symbol.
   This does not handle buffer-local variables; use
   swap_in_symval_forwarding for that.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_symval_forwarding (valcontents: lispfwd) -> Lisp_Object
{
  match XFWDTYPE (valcontents)
    {
    Lisp_Fwd_Int =>
      { return make_int (*XINTVAR (valcontents)); }

    Lisp_Fwd_Bool =>
      { return if *XBOOLVAR (valcontents) { Qt () } else { Qnil () }; }

    Lisp_Fwd_Obj =>
      { return *XOBJVAR (valcontents); }

    Lisp_Fwd_Buffer_Obj =>
      { return per_buffer_value (current_buffer, XBUFFER_OFFSET (valcontents) as c_int); }

    Lisp_Fwd_Kboard_Obj =>
      { return *((kboard_for_bindings () as *mut c_char)
                 .offset (XKBOARD_OFFSET (valcontents) as isize) as *mut Lisp_Object); }
    _ => { emacs_abort (); }
    }
}

/* Used to signal a user-friendly error when symbol WRONG is
   not a member of CHOICE, which should be a list of symbols.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wrong_choice (choice: Lisp_Object, wrong: Lisp_Object) -> !
{
  let mut i: ptrdiff_t = 0;
  let len: ptrdiff_t = list_length (choice);
  let mut obj: Lisp_Object;
  let mut args: *mut Lisp_Object;
  AUTO_STRING! (one_of, b"One of \0".as_ptr () as *const c_char);
  AUTO_STRING! (comma, b", \0".as_ptr () as *const c_char);
  AUTO_STRING! (or, b" or \0".as_ptr () as *const c_char);
  AUTO_STRING! (should_be_specified, b" should be specified\0".as_ptr () as *const c_char);

  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  SAFE_ALLOCA_LISP! (sa_avail, args, len * 2 + 1);

  *args.offset (i) = one_of;
  i += 1;

  obj = choice;
  while ! NILP (obj)
    {
      *args.offset (i) = SYMBOL_NAME (XCAR (obj));
      i += 1;
      *args.offset (i) = if NILP (XCDR (obj)) { should_be_specified }
                         else if NILP (XCDR (XCDR (obj))) { or } else { comma };
      i += 1;
      obj = XCDR (obj);
    }

  let obj2 = Fconcat (i, args);

  /* No need to call SAFE_FREE, since signaling does that for us.  */
  let _ = sa_count;

  xsignal2 (Qerror (), obj2, wrong);
}

/* Used to signal a user-friendly error if WRONG is not a number or
   integer/floating-point number outsize of inclusive MIN..MAX range.  */

unsafe fn wrong_range (min: Lisp_Object, max: Lisp_Object, wrong: Lisp_Object)
{
  AUTO_STRING! (value_should_be_from, b"Value should be from \0".as_ptr () as *const c_char);
  AUTO_STRING! (to, b" to \0".as_ptr () as *const c_char);
  xsignal2 (Qerror (),
            CALLN! (Fconcat, value_should_be_from, Fnumber_to_string (min),
                    to, Fnumber_to_string (max)),
            wrong);
}

unsafe fn check_choice (choice: Lisp_Object, val: Lisp_Object)
{
  eassert! (CONSP (choice));
  if NILP (Fmemq (val, choice))
    { wrong_choice (choice, val); }
}

unsafe fn check_fwd_predicate (p: u32, val: Lisp_Object)
{
  match p
    {
    FWDPRED_Qnil =>
      { return; }
    FWDPRED_Qstringp =>
      {
        if ! STRINGP (val)
          { wrong_type_argument (Qstringp (), val); }
        return;
      }
    FWDPRED_Qsymbolp =>
      {
        if ! SYMBOLP (val)
          { wrong_type_argument (Qsymbolp (), val); }
        return;
      }
    FWDPRED_Qintegerp =>
      {
        if ! INTEGERP (val)
          { wrong_type_argument (Qintegerp (), val); }
        return;
      }
    FWDPRED_Qnumberp =>
      {
        if ! NUMBERP (val)
          { wrong_type_argument (Qnumberp (), val); }
        return;
      }
    FWDPRED_Qfraction =>
      {
        if ! NUMBERP (val)
          { wrong_type_argument (Qnumberp (), val); }
        let range = Fget (Qfraction (), Qrange ());
        eassert! (CONSP (range));
        let min = XCAR (range);
        let max = XCDR (range);
        if NILP (CALLN! (Fleq, min, val, max))
          { wrong_range (min, max, val); }
        return;
      }
    FWDPRED_Qvertical_scroll_bar =>
      {
        check_choice (Fget (Qvertical_scroll_bar (), Qchoice ()), val);
        return;
      }
    FWDPRED_Qoverwrite_mode =>
      {
        check_choice (Fget (Qoverwrite_mode (), Qchoice ()), val);
        return;
      }
    _ => {}
    }
  emacs_abort ();
}

/* Store NEWVAL into SYMBOL, where VALCONTENTS is found in the value cell
   of SYMBOL.  If SYMBOL is buffer-local, VALCONTENTS should be the
   buffer-independent contents of the value cell: forwarded just one
   step past the buffer-localness.

   BUF non-zero means set the value in buffer BUF instead of the
   current buffer.  This only plays a role for per-buffer variables.  */

unsafe fn store_symval_forwarding (valcontents: lispfwd, newval: Lisp_Object,
                            mut buf: *mut buffer)
{
  match XFWDTYPE (valcontents)
    {
    Lisp_Fwd_Int =>
      {
        let mut i: intmax_t = 0;
        CHECK_INTEGER (newval);
        if ! integer_to_intmax (newval, &mut i)
          { xsignal1 (Qoverflow_error (), newval); }
        *XINTVAR (valcontents) = i;
      }
    Lisp_Fwd_Bool =>
      { *XBOOLVAR (valcontents) = ! NILP (newval); }
    Lisp_Fwd_Obj =>
      'case: {
        *XOBJVAR (valcontents) = newval;

        /* If this variable is a default for something stored
           in the buffer itself, such as default-fill-column,
           find the buffers that don't have local values for it
           and update them.  */
        if XOBJVAR (valcontents) > (std::ptr::addr_of_mut! (buffer_defaults) as *mut Lisp_Object)
          && XOBJVAR (valcontents) < (std::ptr::addr_of_mut! (buffer_defaults).add (1)
                                      as *mut Lisp_Object)
          {
            let offset: c_int = (XOBJVAR (valcontents) as *mut c_char)
              .offset_from (std::ptr::addr_of_mut! (buffer_defaults) as *mut c_char) as c_int;
            let idx = PER_BUFFER_IDX (offset as ptrdiff_t);

            /* C's `FOR_EACH_LIVE_BUFFER (tail, buf)` is a two-variable alist loop
               macro the mirror does not carry (P-22, docs/ports/excluded-macros.txt);
               the port writes the expansion C's `FOR_EACH_ALIST_VALUE` gives it.  */
            if idx <= 0
              { break 'case; }

            let mut list_var: Lisp_Object = Vbuffer_alist;
            while CONSP (list_var)
              {
                let buf = XCDR (XCAR (list_var));
                let b = XBUFFER (buf);

                if ! PER_BUFFER_VALUE_P (b, idx)
                  { set_per_buffer_value (b, offset, newval); }

                list_var = XCDR (list_var);
              }
          }
      }
    Lisp_Fwd_Buffer_Obj =>
      {
        let offset: c_int = XBUFFER_OFFSET (valcontents) as c_int;
        if ! NILP (newval)
          { check_fwd_predicate ((*valcontents).u.buf.predicate as u32, newval); }
        if buf.is_null ()
          { buf = current_buffer; }
        set_per_buffer_value (buf, offset, newval);
      }
    Lisp_Fwd_Kboard_Obj =>
      {
        let base = kboard_for_bindings () as *mut c_char;
        let p = base.offset (XKBOARD_OFFSET (valcontents) as isize);
        *(p as *mut Lisp_Object) = newval;
      }
    _ =>
      { emacs_abort (); /* goto def; */ }
    }
}

/* Set up SYMBOL to refer to its global binding.  This makes it safe
   to alter the status of other bindings.  BEWARE: this may be called
   during the mark phase of GC, where we assume that Lisp_Object slots
   of BLV are marked after this function has changed them.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn swap_in_global_binding (symbol: *mut Lisp_Symbol)
{
  let blv = SYMBOL_BLV (symbol) as *mut Lisp_Buffer_Local_Value;

  /* Unload the previously loaded binding.  */
  if ! (*blv).fwd.is_null ()
    { set_blv_value (blv, do_symval_forwarding ((*blv).fwd)); }

  /* Select the global binding in the symbol.  */
  set_blv_valcell (blv, (*blv).defcell);
  if ! (*blv).fwd.is_null ()
    { store_symval_forwarding ((*blv).fwd, XCDR ((*blv).defcell), std::ptr::null_mut ()); }

  /* Indicate that the global binding is set up now.  */
  set_blv_where (blv, Qnil ());
  set_blv_found (blv, false as c_int);
}

/* Set up the buffer-local symbol SYMBOL for validity in the current buffer.
   VALCONTENTS is the contents of its value cell,
   which points to a struct Lisp_Buffer_Local_Value.

   Return the value forwarded one step past the buffer-local stage.
   This could be another forwarding pointer.  */

unsafe fn swap_in_symval_forwarding (symbol: *mut Lisp_Symbol, blv: *mut Lisp_Buffer_Local_Value)
{
  let mut tem1: Lisp_Object;

  eassert! (blv as *mut c_void == SYMBOL_BLV (symbol));

  tem1 = (*blv).r#where;

  if NILP (tem1)
    || current_buffer != XBUFFER (tem1)
    {

      /* Unload the previously loaded binding.  */
      tem1 = (*blv).valcell;
      if ! (*blv).fwd.is_null ()
        { set_blv_value (blv, do_symval_forwarding ((*blv).fwd)); }
      /* Choose the new binding.  */
      {
        let mut var: Lisp_Object = Qnil ();
        XSETSYMBOL! (var, symbol);
        tem1 = assq_no_quit (var, BVAR! (current_buffer, local_var_alist_));
        set_blv_where (blv, Fcurrent_buffer ());
      }
      /* C assigns the bitfield inside the condition (`if (!(blv->found = !NILP (tem1)))`).
         It must not go through the mirror's `set_blv_found`: that helper carries C's
         eassert (data.c 84), which does not hold here, because `set_blv_valcell` below is
         what makes `found` agree with `valcell` (review A's D2).  */
      let found: c_int = ! NILP (tem1) as c_int;
      {
        let bits = (*blv).bits;
        (*blv).bits = if found != 0 { bits | BLV_FOUND } else { bits & ! BLV_FOUND };
      }
      if found == 0
        { tem1 = (*blv).defcell; }

      /* Load the new binding.  */
      set_blv_valcell (blv, tem1);
      if ! (*blv).fwd.is_null ()
        { store_symval_forwarding ((*blv).fwd, blv_value (blv), std::ptr::null_mut ()); }
    }
}
/* Find the value of a symbol, returning Qunbound if it's not bound.
   This is helpful for code which just wants to get a variable's value
   if it has one, without signaling an error.

   This function is very similar to buffer_local_value, but we have
   two separate code paths here since find_symbol_value has to be very
   efficient, while buffer_local_value doesn't have to be.

   Note that it must not be possible to quit within this function.
   Great care is required for this.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_symbol_value (symbol: Lisp_Object) -> Lisp_Object
{
  CHECK_SYMBOL (symbol);
  let mut sym: *mut Lisp_Symbol = XSYMBOL (symbol);

  /* C's `start:` label and its `goto start` become this loop.  */
  loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
        {
        SYMBOL_VARALIAS =>
          { sym = SYMBOL_ALIAS (sym); continue; }
        SYMBOL_PLAINVAL =>
          { return SYMBOL_VAL (sym); }
        SYMBOL_LOCALIZED =>
          {
            let blv = SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value;
            swap_in_symval_forwarding (sym, blv);
            return if ! (*blv).fwd.is_null ()
                     { do_symval_forwarding ((*blv).fwd) }
                   else
                     { blv_value (blv) };
          }
        SYMBOL_FORWARDED =>
          { return do_symval_forwarding (SYMBOL_FWD (sym) as *mut Lisp_Fwd); }
        _ =>
          { emacs_abort (); }
        }
    }
}

DEFUN! ("symbol-value", Fsymbol_value, Ssymbol_value, 1, 1, 0,
       doc: "Return SYMBOL's value.  Error if that is void.\nNote that if `lexical-binding' is in effect, this returns the\nglobal value outside of any lexical scope.",
       (symbol: Lisp_Object),
       {
  let val: Lisp_Object;

  val = find_symbol_value (symbol);
  if ! BASE_EQ (val, Qunbound ())
    { return val; }

  xsignal1 (Qvoid_variable (), symbol);
});
DEFUN! ("set", Fset, Sset, 2, 2, 0,
       doc: "Set SYMBOL's value to NEWVAL, and return NEWVAL.",
       (symbol: Lisp_Object, newval: Lisp_Object),
       {
  set_internal (symbol, newval, Qnil (), SET_INTERNAL_SET);
  return newval;
});
/* Store the value NEWVAL into SYMBOL.
   If buffer-locality is an issue, WHERE specifies which context to use.
   (nil stands for the current buffer/frame).

   If BINDFLAG is SET_INTERNAL_SET, then if this symbol is supposed to
   become local in every buffer where it is set, then we make it
   local.  If BINDFLAG is SET_INTERNAL_BIND or SET_INTERNAL_UNBIND, we
   don't do that.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_internal (symbol: Lisp_Object, newval: Lisp_Object,
                                       r#where: Lisp_Object, bindflag: Set_Internal_Bind)
{
  let unbinding_p: bool = BASE_EQ (newval, Qunbound ());
  let mut symbol = symbol;   // C's `XSETSYMBOL (symbol, sym)` below rebinds the argument

  /* If restoring in a dead buffer, do nothing.  */

  CHECK_SYMBOL (symbol);
  let mut sym: *mut Lisp_Symbol = XSYMBOL (symbol);
  match ((*sym).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK) >> SYMBOL_TRAPPED_WRITE_SHIFT
    {
    SYMBOL_NOWRITE =>
      {
        if NILP (Fkeywordp (symbol))
          || ! EQ (newval, Fsymbol_value (symbol))
          { xsignal1 (Qsetting_constant (), symbol); }
        else
          /* Allow setting keywords to their own value.  */
          { return; }
      }

    SYMBOL_TRAPPED_WRITE =>
      {
        /* Setting due to thread-switching doesn't count.  */
        if bindflag != SET_INTERNAL_THREAD_SWITCH
          { notify_variable_watchers (symbol, if unbinding_p { Qnil () } else { newval },
                                      if bindflag == SET_INTERNAL_BIND { Qlet () }
                                      else if bindflag == SET_INTERNAL_UNBIND { Qunlet () }
                                      else if unbinding_p { Qmakunbound () } else { Qset () },
                                      r#where); }
      }

    SYMBOL_UNTRAPPED_WRITE => {}

    _ => { emacs_abort (); }
    }

  /* C's `start:` label and its `goto start` become this loop.  */
  let mut r#where = r#where;
  loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
        {
        SYMBOL_VARALIAS =>
          { sym = SYMBOL_ALIAS (sym); continue; }
        SYMBOL_PLAINVAL =>
          { SET_SYMBOL_VAL (sym, newval); return; }
        SYMBOL_LOCALIZED =>
          {
            let blv = SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value;

            if unbinding_p && ! (*blv).fwd.is_null ()
              /* Forbid unbinding built-in variables.  */
              { error (c"Built-in variable may not be unbound : %s".as_ptr (),
                       SDATA (SYMBOL_NAME (symbol))); }

            if NILP (r#where)
              { XSETBUFFER! (r#where, current_buffer); }

            /* If the current buffer is not the buffer whose binding is
               loaded, or if it's a Lisp_Buffer_Local_Value and
               the default binding is loaded, the loaded binding may be the
               wrong one.  */
            if ! BASE_EQ ((*blv).r#where, r#where)
              /* Also unload a global binding (if the var is local_if_set).  */
              || (BASE_EQ ((*blv).valcell, (*blv).defcell))
              {
                /* The currently loaded binding is not necessarily valid.
                   We need to unload it, and choose a new binding.  */

                /* Write out `realvalue' to the old loaded binding.  */
                if ! (*blv).fwd.is_null ()
                  { set_blv_value (blv, do_symval_forwarding ((*blv).fwd)); }

                /* Find the new binding.  */
                XSETSYMBOL! (symbol, sym); /* May have changed via aliasing.  */
                let mut tem1: Lisp_Object
                  = assq_no_quit (symbol,
                                  BVAR! (XBUFFER (r#where), local_var_alist_));
                set_blv_where (blv, r#where);
                /* C writes the one-bit field directly here (as in
                   swap_in_symval_forwarding, PORTING.md 1.3): the asserted
                   `set_blv_found` does not hold before `set_blv_valcell` below.  */
                {
                  let bits = (*blv).bits;
                  (*blv).bits = bits | BLV_FOUND;
                }

                if NILP (tem1)
                  {
                    /* This buffer still sees the default value.  */

                    /* If the variable is a Lisp_Some_Buffer_Local_Value,
                       or if this is `let' rather than `set',
                       make CURRENT-ALIST-ELEMENT point to itself,
                       indicating that we're seeing the default value.
                       Likewise if the variable has been let-bound
                       in the current buffer.  */
                    if bindflag != 0 || (*blv).bits & BLV_LOCAL_IF_SET == 0
                      || let_shadows_buffer_binding_p (sym)
                      {
                        (*blv).bits = (*blv).bits & ! BLV_FOUND;
                        tem1 = (*blv).defcell;
                      }
                    /* If it's a local_if_set, being set not bound,
                       and we're not within a let that was made for this buffer,
                       create a new buffer-local binding for the variable.
                       That means, give this buffer a new assoc for a local value
                       and load that binding.  */
                    else
                      {
                        tem1 = Fcons (symbol, XCDR ((*blv).defcell));
                        bset_local_var_alist
                          (XBUFFER (r#where),
                           Fcons (tem1, BVAR! (XBUFFER (r#where), local_var_alist_)));
                      }
                  }

                /* Record which binding is now loaded.  */
                set_blv_valcell (blv, tem1);
              }

            /* Store the new value in the cons cell.  */
            set_blv_value (blv, newval);

            if ! (*blv).fwd.is_null ()
              { store_symval_forwarding ((*blv).fwd, newval,
                                         if BUFFERP (r#where) { XBUFFER (r#where) }
                                         else { current_buffer }); }
            return;
          }
        SYMBOL_FORWARDED =>
          {
            let buf: *mut buffer
              = if BUFFERP (r#where) { XBUFFER (r#where) } else { current_buffer };
            let innercontents: lispfwd = SYMBOL_FWD (sym) as *mut Lisp_Fwd;

            if unbinding_p
              /* Forbid unbinding built-in variables.  */
              { error (c"Built-in variable may not be unbound : %s".as_ptr (),
                       SDATA (SYMBOL_NAME (symbol))); }

            if BUFFER_OBJFWDP (innercontents)
              {
                let offset: c_int = XBUFFER_OFFSET (innercontents) as c_int;
                let idx = PER_BUFFER_IDX (offset as ptrdiff_t);
                if idx > 0 && bindflag == SET_INTERNAL_SET
                  && ! PER_BUFFER_VALUE_P (buf, idx)
                  {
                    if let_shadows_buffer_binding_p (sym)
                      { set_default_internal (symbol, newval, bindflag,
                                              std::ptr::null_mut ()); }
                    else
                      { SET_PER_BUFFER_VALUE_P (buf, idx, true); }
                  }
              }

            store_symval_forwarding (/* sym, */ innercontents, newval, buf);
            return;
          }
        _ =>
          { emacs_abort (); }
        }
    }
}

// data.c 1833-1840.  C reads and writes the `trapped_write` bitfield; the mirror packs it
// into `u.s.bits` (lisp.rs 224-225), so both sides go through the mask/shift.
unsafe extern "C" fn set_symbol_trapped_write (symbol: Lisp_Object, trap: u32)
{
  let sym: *mut Lisp_Symbol = XSYMBOL (symbol);
  if ((*sym).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK) >> SYMBOL_TRAPPED_WRITE_SHIFT
     == SYMBOL_NOWRITE
    { xsignal1 (Qtrapping_constant (), symbol); }
  let bits = (*sym).u.s.bits;
  (*sym).u.s.bits = (bits & ! SYMBOL_TRAPPED_WRITE_MASK)
                    | ((trap << SYMBOL_TRAPPED_WRITE_SHIFT) & SYMBOL_TRAPPED_WRITE_MASK);
}

// data.c 1842-1846.
unsafe extern "C" fn restore_symbol_trapped_write (symbol: Lisp_Object)
{
  set_symbol_trapped_write (symbol, SYMBOL_TRAPPED_WRITE);
}

/* Called after `defvaralias' to make sure all aliases have the same
   trap.  */

unsafe extern "C" fn harmonize_variable_watchers (alias: Lisp_Object,
                                                  base_variable: Lisp_Object)
{
  if ! EQ (base_variable, alias)
    && EQ (base_variable, Findirect_variable (alias))
    { set_symbol_trapped_write
        (alias, ((*XSYMBOL (base_variable)).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK)
               >> SYMBOL_TRAPPED_WRITE_SHIFT); }
}

DEFUN! ("add-variable-watcher", Fadd_variable_watcher, Sadd_variable_watcher,
       2, 2, 0,
       doc: "Cause WATCH-FUNCTION to be called when SYMBOL is about to be set.\n\nIt will be called with 4 arguments: (SYMBOL NEWVAL OPERATION WHERE).\nSYMBOL is the variable being changed.\nNEWVAL is the value it will be changed to.  (The variable still has\nthe old value when WATCH-FUNCTION is called.)\nOPERATION is a symbol representing the kind of change, one of: `set',\n`let', `unlet', `makunbound', and `defvaralias'.\nWHERE is a buffer if the buffer-local value of the variable is being\nchanged, nil otherwise.\n\nAll writes to aliases of SYMBOL will call WATCH-FUNCTION too.",
       (symbol: Lisp_Object, watch_function: Lisp_Object),
       {
  let mut symbol = symbol;
  symbol = Findirect_variable (symbol);
  CHECK_SYMBOL (symbol);
  set_symbol_trapped_write (symbol, SYMBOL_TRAPPED_WRITE);
  map_obarray (Vobarray, Some (harmonize_variable_watchers), symbol);

  let watchers = Fget (symbol, Qwatchers ());
  let member = Fmember (watch_function, watchers);
  if NILP (member)
    { Fput (symbol, Qwatchers (), Fcons (watch_function, watchers)); }
  return Qnil ();
});

DEFUN! ("remove-variable-watcher", Fremove_variable_watcher, Sremove_variable_watcher,
       2, 2, 0,
       doc: "Undo the effect of `add-variable-watcher'.\nRemove WATCH-FUNCTION from the list of functions to be called when\nSYMBOL (or its aliases) are set.",
       (symbol: Lisp_Object, watch_function: Lisp_Object),
       {
  let mut symbol = symbol;
  symbol = Findirect_variable (symbol);
  let mut watchers = Fget (symbol, Qwatchers ());
  watchers = Fdelete (watch_function, watchers);
  if NILP (watchers)
    {
      set_symbol_trapped_write (symbol, SYMBOL_UNTRAPPED_WRITE);
      map_obarray (Vobarray, Some (harmonize_variable_watchers), symbol);
    }
  Fput (symbol, Qwatchers (), watchers);
  return Qnil ();
});

DEFUN! ("get-variable-watchers", Fget_variable_watchers, Sget_variable_watchers,
       1, 1, 0,
       doc: "Return a list of SYMBOL's active watchers.",
       (symbol: Lisp_Object),
       {
  return if SYMBOL_TRAPPED_WRITE_P (symbol) == SYMBOL_TRAPPED_WRITE
           { Fget (Findirect_variable (symbol), Qwatchers ()) }
         else { Qnil () };
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn notify_variable_watchers (symbol: Lisp_Object,
                                                   newval: Lisp_Object,
                                                   operation: Lisp_Object,
                                                   r#where: Lisp_Object)
{
  let mut symbol = symbol;
  let mut operation = operation;
  let mut r#where = r#where;

  symbol = Findirect_variable (symbol);

  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect (Some (restore_symbol_trapped_write), symbol);
  /* Avoid recursion.  */
  set_symbol_trapped_write (symbol, SYMBOL_UNTRAPPED_WRITE);

  if NILP (r#where)
    && ! EQ (operation, Qset_default ()) && ! EQ (operation, Qmakunbound ())
    && ! NILP (Flocal_variable_if_set_p (symbol, Fcurrent_buffer ()))
    {
      XSETBUFFER! (r#where, current_buffer);
    }

  if EQ (operation, Qset_default ())
    { operation = Qset (); }

  let mut watchers = Fget (symbol, Qwatchers ());
  while CONSP (watchers)
    {
      let watcher = XCAR (watchers);
      /* Call subr directly to avoid gc.  */
      if SUBRP (watcher)
        {
          let args: [Lisp_Object; 4] = [symbol, newval, operation, r#where];
          funcall_subr (XSUBR (watcher), countof! (args) as ptrdiff_t,
                        args.as_ptr () as *mut Lisp_Object);
        }
      else
        { calln! (watcher, symbol, newval, operation, r#where); }
      watchers = XCDR (watchers);
    }

  unbind_to (count, Qnil ());
}
/* Access or set a buffer-local symbol's default value.  */

/* Return the default value of SYMBOL, but don't check for voidness.
   Return Qunbound if it is void.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn default_value (symbol: Lisp_Object) -> Lisp_Object
{
  let mut sym: *mut Lisp_Symbol;

  CHECK_SYMBOL (symbol);
  sym = XSYMBOL (symbol);

  /* C's `start:` label and its `goto start` become this loop.  */
  loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
        {
        SYMBOL_VARALIAS =>
          { sym = SYMBOL_ALIAS (sym); continue; }
        SYMBOL_PLAINVAL =>
          { return SYMBOL_VAL (sym); }
        SYMBOL_LOCALIZED =>
          {
            /* If var is set up for a buffer that lacks a local value for it,
               the current value is nominally the default value.
               But the `realvalue' slot may be more up to date, since
               ordinary setq stores just that slot.  So use that.  */
            let blv = SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value;
            if ! (*blv).fwd.is_null () && BASE_EQ ((*blv).valcell, (*blv).defcell)
              { return do_symval_forwarding ((*blv).fwd); }
            else
              { return XCDR ((*blv).defcell); }
          }
        SYMBOL_FORWARDED =>
          {
            let valcontents: lispfwd = SYMBOL_FWD (sym) as *mut Lisp_Fwd;

            /* For a built-in buffer-local variable, get the default value
               rather than letting do_symval_forwarding get the current value.  */
            if BUFFER_OBJFWDP (valcontents)
              {
                let offset: c_int = XBUFFER_OFFSET (valcontents) as c_int;
                if PER_BUFFER_IDX (offset as ptrdiff_t) != 0
                  { return per_buffer_default (offset); }
              }

            /* For other variables, get the current value.  */
            return do_symval_forwarding (valcontents);
          }
        _ =>
          { emacs_abort (); }
        }
    }
}

DEFUN! ("default-boundp", Fdefault_boundp, Sdefault_boundp, 1, 1, 0,
       doc: "Return t if SYMBOL has a non-void default value.\nA variable may have a buffer-local value.  This function says whether\nthe variable has a non-void value outside of the current buffer\ncontext.  Also see `default-value'.",
       (symbol: Lisp_Object),
       {
  let value: Lisp_Object;

  value = default_value (symbol);
  return if BASE_EQ (value, Qunbound ()) { Qnil () } else { Qt () };
});

DEFUN! ("default-value", Fdefault_value, Sdefault_value, 1, 1, 0,
       doc: "Return SYMBOL's default value.\nThis is the value that is seen in buffers that do not have their own values\nfor this variable.  The default value is meaningful for variables with\nlocal bindings in certain buffers.",
       (symbol: Lisp_Object),
       {
  let value: Lisp_Object = default_value (symbol);
  if ! BASE_EQ (value, Qunbound ())
    { return value; }

  xsignal1 (Qvoid_variable (), symbol);
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_default_internal (symbol: Lisp_Object, value: Lisp_Object,
                                               bindflag: Set_Internal_Bind,
                                               r#where: *mut KBOARD)
{
  CHECK_SYMBOL (symbol);
  let mut sym: *mut Lisp_Symbol = XSYMBOL (symbol);
  match ((*sym).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK) >> SYMBOL_TRAPPED_WRITE_SHIFT
    {
    SYMBOL_NOWRITE =>
      {
        if NILP (Fkeywordp (symbol))
          || ! EQ (value, Fsymbol_value (symbol))
          { xsignal1 (Qsetting_constant (), symbol); }
        else
          /* Allow setting keywords to their own value.  */
          { return; }
      }

    SYMBOL_TRAPPED_WRITE =>
      {
        /* Don't notify here if we're going to call Fset anyway.  */
        if ((((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
             != SYMBOL_PLAINVAL)
            /* Setting due to thread switching doesn't count.  */
            && bindflag != SET_INTERNAL_THREAD_SWITCH)
          { notify_variable_watchers (symbol, value, Qset_default (), Qnil ()); }
      }

    SYMBOL_UNTRAPPED_WRITE => {}

    _ => { emacs_abort (); }
    }

  /* C's `start:` label and its `goto start` become this loop.  */
  loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
        {
        SYMBOL_VARALIAS =>
          { sym = SYMBOL_ALIAS (sym); continue; }
        SYMBOL_PLAINVAL =>
          { set_internal (symbol, value, Qnil (), bindflag); return; }
        SYMBOL_LOCALIZED =>
          {
            let blv = SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value;

            /* Store new value into the DEFAULT-VALUE slot.  */
            XSETCDR ((*blv).defcell, value);

            /* If the default binding is now loaded, set the REALVALUE slot too.  */
            if ! (*blv).fwd.is_null () && BASE_EQ ((*blv).defcell, (*blv).valcell)
              { store_symval_forwarding ((*blv).fwd, value, std::ptr::null_mut ()); }
            return;
          }
        SYMBOL_FORWARDED =>
          {
            let valcontents: lispfwd = SYMBOL_FWD (sym) as *mut Lisp_Fwd;

            /* Handle variables like case-fold-search that have special slots
               in the buffer.
               Make them work apparently like Lisp_Buffer_Local_Value variables.  */
            if BUFFER_OBJFWDP (valcontents)
              {
                let offset: c_int = XBUFFER_OFFSET (valcontents) as c_int;
                let idx = PER_BUFFER_IDX (offset as ptrdiff_t);

                set_per_buffer_default (offset, value);

                /* If this variable is not always local in all buffers,
                   set it in the buffers that don't nominally have a local value.  */
                if idx > 0
                  {
                    /* C's `FOR_EACH_LIVE_BUFFER (tail, buf)` is a two-variable alist
                       loop macro the mirror does not carry (P-22,
                       docs/ports/excluded-macros.txt); the port writes the expansion
                       C's `FOR_EACH_ALIST_VALUE` gives it.  */
                    let mut list_var: Lisp_Object = Vbuffer_alist;
                    while CONSP (list_var)
                      {
                        let buf = XCDR (XCAR (list_var));
                        let b = XBUFFER (buf);

                        if ! PER_BUFFER_VALUE_P (b, idx)
                          { set_per_buffer_value (b, offset, value); }

                        list_var = XCDR (list_var);
                      }
                  }
              }
            else if KBOARD_OBJFWDP (valcontents)
              {
                let base = if ! r#where.is_null () { r#where as *mut c_char }
                           else { kboard_for_bindings () as *mut c_char };
                let p = base.offset (XKBOARD_OFFSET (valcontents) as isize);
                *(p as *mut Lisp_Object) = value;
              }
            else
              { set_internal (symbol, value, Qnil (), bindflag); }
            return;
          }
        _ =>
          { emacs_abort (); }
        }
    }
}

DEFUN! ("set-default", Fset_default, Sset_default, 2, 2, 0,
       doc: "Set SYMBOL's default value to VALUE.  SYMBOL and VALUE are evaluated.\nThe default value is seen in buffers that do not have their own values\nfor this variable.",
       (symbol: Lisp_Object, value: Lisp_Object),
       {
  set_default_internal (symbol, value, SET_INTERNAL_SET, std::ptr::null_mut ());
  return value;
});

/* Lisp functions for creating and removing buffer-local variables.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub union Lisp_Val_Fwd {
  pub value: Lisp_Object,
  pub fwd: lispfwd,
}
// The union is defined in data.c itself, so no header mirror can name it and
// `xtask layout-check` (which compiles only `include/*.rs`) can never carry a
// `c-layout` key for it: the port asserts the same two facts here instead, the
// `bignum.rs`/`doprnt.rs` precedent.  Both members are one word wide.
const _: () = assert! (size_of::<Lisp_Val_Fwd> () == 8 && align_of::<Lisp_Val_Fwd> () == 8);

unsafe fn make_blv (sym: *mut Lisp_Symbol, forwarded: bool,
                    valcontents: Lisp_Val_Fwd) -> *mut Lisp_Buffer_Local_Value
{
  let blv = xmalloc (size_of::<Lisp_Buffer_Local_Value> ()) as *mut Lisp_Buffer_Local_Value;
  let symbol: Lisp_Object;
  let tem: Lisp_Object;

  XSETSYMBOL! (symbol, sym);
  tem = Fcons (symbol, if forwarded
                         { do_symval_forwarding (valcontents.fwd) }
                       else
                         { valcontents.value });

  /* Buffer_Local_Values cannot have as realval a buffer-local
     or keyboard-local forwarding.  */
  eassert! (! (forwarded && BUFFER_OBJFWDP (valcontents.fwd)));
  eassert! (! (forwarded && KBOARD_OBJFWDP (valcontents.fwd)));
  (*blv).fwd = if forwarded { valcontents.fwd } else { std::ptr::null_mut () };
  set_blv_where (blv, Qnil ());
  /* C writes the one-bit `local_if_set` field (then `set_blv_found` writes `found`);
     the mirror packs both into `u.s.bits` (lisp.rs 1970-1971).  The substitute spelling
     is pinned by the `local_if_set` token family (docs/ports/token-aliases.tsv), whose
     occurrence check reports this statement's deletion.  */
  (*blv).bits = (*blv).bits & ! BLV_LOCAL_IF_SET;
  set_blv_defcell (blv, tem);
  set_blv_valcell (blv, tem);
  set_blv_found (blv, false as c_int);
  __lsan_ignore_object (blv as *const c_void);
  return blv;
}

DEFUN! ("make-variable-buffer-local", Fmake_variable_buffer_local,
       Smake_variable_buffer_local, 1, 1, "vMake Variable Buffer Local: ",
       doc: "Make VARIABLE become buffer-local whenever it is set.\nAt any time, the value for the current buffer is in effect,\nunless the variable has never been set in this buffer,\nin which case the default value is in effect.\nNote that binding the variable with `let', or setting it while\na `let'-style binding made in this buffer is in effect,\ndoes not make the variable buffer-local.  Return VARIABLE.\n\nThis globally affects all uses of this variable, so it belongs together with\nthe variable declaration, rather than with its uses (if you just want to make\na variable local to the current buffer for one particular use, use\n`make-local-variable').  Buffer-local bindings are normally cleared\nwhile setting up a new major mode, unless they have a `permanent-local'\nproperty.\n\nThe function `default-value' gets the default value and `set-default' sets it.\n\nSee also `defvar-local'.",
       (variable: Lisp_Object),
       {
  let mut sym: *mut Lisp_Symbol;
  let mut blv: *mut Lisp_Buffer_Local_Value = std::ptr::null_mut ();
  /* C's `UNINIT` is `= {0,}` in this configuration (conf_post.h 369: GCC_LINT is
     defined, src/config.h 209), so both locals start zeroed; the raw text carries no
     member name, which is why this body's notes row exempts `uninit`.  */
  let mut valcontents: Lisp_Val_Fwd = Lisp_Val_Fwd { value: Qnil () };
  let mut forwarded: bool = false;

  CHECK_SYMBOL (variable);
  sym = XSYMBOL (variable);

  loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
        {
        SYMBOL_VARALIAS =>
          { sym = SYMBOL_ALIAS (sym); continue; }
        SYMBOL_PLAINVAL =>
          {
            forwarded = false; valcontents.value = SYMBOL_VAL (sym);
            if BASE_EQ (valcontents.value, Qunbound ())
              { valcontents.value = Qnil (); }
          }
        SYMBOL_LOCALIZED =>
          { blv = SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value; }
        SYMBOL_FORWARDED =>
          {
            forwarded = true; valcontents.fwd = SYMBOL_FWD (sym) as lispfwd;
            if KBOARD_OBJFWDP (valcontents.fwd)
              {
                error (b"Symbol %s may not be buffer-local\0".as_ptr () as *const c_char,
                       SDATA (SYMBOL_NAME (variable)));
              }
            else if BUFFER_OBJFWDP (valcontents.fwd)
              { return variable; }
          }
        _ =>
          { emacs_abort (); }
        }
      break;
    }

  if SYMBOL_CONSTANT_P (variable)
    { xsignal1 (Qsetting_constant (), variable); }

  if blv.is_null ()
    {
      blv = make_blv (sym, forwarded, valcontents);
      /* C assigns the bitfield `redirect`; the mirror packs it into `u.s.bits`
         (lisp.rs 221-226).  */
      let bits = (*sym).u.s.bits;
      (*sym).u.s.bits = (bits & ! SYMBOL_REDIRECT_MASK)
                        | (SYMBOL_LOCALIZED << SYMBOL_REDIRECT_SHIFT);
      SET_SYMBOL_BLV (sym, blv as *mut c_void);
    }

  (*blv).bits = (*blv).bits | BLV_LOCAL_IF_SET;
  return variable;
});

DEFUN! ("make-local-variable", Fmake_local_variable,
       Smake_local_variable, 1, 1, "vMake Local Variable: ",
       doc: "Make VARIABLE have a separate value in the current buffer.\nOther buffers will continue to share a common default value.\n(The buffer-local value of VARIABLE starts out as the same value\nVARIABLE previously had.  If VARIABLE was void, it remains void.)\nReturn VARIABLE.\n\nIf the variable is already arranged to become local when set,\nthis function causes a local value to exist for this buffer,\njust as setting the variable would do.\n\nThis function returns VARIABLE, and therefore\n  (set (make-local-variable \\='VARIABLE) VALUE-EXP)\nworks.\n\nSee also `make-variable-buffer-local'.\n\nDo not use `make-local-variable' to make a hook variable buffer-local.\nInstead, use `add-hook' and specify t for the LOCAL argument.",
       (variable: Lisp_Object),
       {
  let mut variable = variable;   // C's `XSETSYMBOL (variable, sym)` below rebinds the argument
  let tem: Lisp_Object;
  let mut forwarded: bool = false;   /* C's `bool forwarded UNINIT;` (see
                                        `make-variable-buffer-local`).  */
  let mut valcontents: Lisp_Val_Fwd = Lisp_Val_Fwd { value: Qnil () };
  let mut sym: *mut Lisp_Symbol;
  let mut blv: *mut Lisp_Buffer_Local_Value = std::ptr::null_mut ();

  CHECK_SYMBOL (variable);
  sym = XSYMBOL (variable);

  loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
        {
        SYMBOL_VARALIAS =>
          { sym = SYMBOL_ALIAS (sym); continue; }
        SYMBOL_PLAINVAL =>
          { forwarded = false; valcontents.value = SYMBOL_VAL (sym); }
        SYMBOL_LOCALIZED =>
          { blv = SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value; }
        SYMBOL_FORWARDED =>
          {
            forwarded = true; valcontents.fwd = SYMBOL_FWD (sym) as lispfwd;
            if KBOARD_OBJFWDP (valcontents.fwd)
              {
                error (b"Symbol %s may not be buffer-local\0".as_ptr () as *const c_char,
                       SDATA (SYMBOL_NAME (variable)));
              }
          }
        _ =>
          { emacs_abort (); }
        }
      break;
    }

  if ((((*sym).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK) >> SYMBOL_TRAPPED_WRITE_SHIFT)
      == SYMBOL_NOWRITE)
    { xsignal1 (Qsetting_constant (), variable); }

  if blv.is_null ()
    {
      if forwarded && BUFFER_OBJFWDP (valcontents.fwd)
        {
          let offset = XBUFFER_OFFSET (valcontents.fwd) as c_int;
          let idx = PER_BUFFER_IDX (offset as ptrdiff_t);
          eassert! (idx != 0);
          if idx > 0
            /* If idx < 0, it's always buffer local, like `mode-name`.  */
            { SET_PER_BUFFER_VALUE_P (current_buffer, idx, true); }
          return variable;
        }
      blv = make_blv (sym, forwarded, valcontents);
      let bits = (*sym).u.s.bits;
      (*sym).u.s.bits = (bits & ! SYMBOL_REDIRECT_MASK)
                        | (SYMBOL_LOCALIZED << SYMBOL_REDIRECT_SHIFT);
      SET_SYMBOL_BLV (sym, blv as *mut c_void);
    }

  /* Make sure this buffer has its own value of symbol.  */
  XSETSYMBOL! (variable, sym);	/* Update in case of aliasing.  */
  tem = assq_no_quit (variable, BVAR! (current_buffer, local_var_alist_));
  if NILP (tem)
    {
      if let_shadows_buffer_binding_p (sym)
        {
          AUTO_STRING! (format, b"Making %s buffer-local while locally let-bound!\0".as_ptr () as *const c_char);
          CALLN! (Fmessage, format, SYMBOL_NAME (variable));
        }

      if BUFFERP ((*blv).r#where) && current_buffer == XBUFFER ((*blv).r#where)
        /* Make sure the current value is permanently recorded, if it's the
           default value.  */
        { swap_in_global_binding (sym); }

      bset_local_var_alist
        (current_buffer,
         Fcons (Fcons (variable, XCDR ((*blv).defcell)),
                BVAR! (current_buffer, local_var_alist_)));

      /* If the symbol forwards into a C variable, then load the binding
         for this buffer now, to preserve the invariant that forwarded
         variables must always hold the value corresponding to the
         current buffer (they are swapped eagerly).
         Otherwise, if C code modifies the variable before we load the
         binding in, then that new value would clobber the default binding
         the next time we unload it.  See bug#34318.  */
      if ! (*blv).fwd.is_null ()
        { swap_in_symval_forwarding (sym, blv); }
    }

  return variable;
});

DEFUN! ("kill-local-variable", Fkill_local_variable,
       Skill_local_variable, 1, 1, "vKill Local Variable: ",
       doc: "Make VARIABLE no longer have a separate value in the current buffer.\nFrom now on the default value will apply in this buffer.  Return VARIABLE.",
       (variable: Lisp_Object),
       {
  let mut variable = variable;   // C's `XSETSYMBOL (variable, sym)` below rebinds the argument
  let tem: Lisp_Object;
  let blv: *mut Lisp_Buffer_Local_Value;
  let mut sym: *mut Lisp_Symbol;

  CHECK_SYMBOL (variable);
  sym = XSYMBOL (variable);

  loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
        {
        SYMBOL_VARALIAS =>
          { sym = SYMBOL_ALIAS (sym); continue; }
        SYMBOL_PLAINVAL =>
          { return variable; }
        SYMBOL_FORWARDED =>
          {
            let valcontents: lispfwd = SYMBOL_FWD (sym) as lispfwd;
            if BUFFER_OBJFWDP (valcontents)
              {
                let offset = XBUFFER_OFFSET (valcontents) as c_int;
                let idx = PER_BUFFER_IDX (offset as ptrdiff_t);

                if idx > 0
                  {
                    SET_PER_BUFFER_VALUE_P (current_buffer, idx, false);
                    set_per_buffer_value (current_buffer, offset,
                                          per_buffer_default (offset));
                  }
              }
            return variable;
          }
        SYMBOL_LOCALIZED =>
          { blv = SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value; }
        _ =>
          { emacs_abort (); }
        }
      break;
    }

  if ((((*sym).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK) >> SYMBOL_TRAPPED_WRITE_SHIFT)
      == SYMBOL_TRAPPED_WRITE)
    { notify_variable_watchers (variable, Qnil (), Qmakunbound (), Fcurrent_buffer ()); }

  /* Get rid of this buffer's alist element, if any.  */
  XSETSYMBOL! (variable, sym);	/* Propagate variable indirection.  */
  tem = assq_no_quit (variable, BVAR! (current_buffer, local_var_alist_));
  if ! NILP (tem)
    {
      bset_local_var_alist
        (current_buffer,
         Fdelq (tem, BVAR! (current_buffer, local_var_alist_)));
    }

  /* If the symbol is set up with the current buffer's binding
     loaded, recompute its value.  We have to do it now, or else
     forwarded objects won't work right.  */
  {
    let buf: Lisp_Object;
    XSETBUFFER! (buf, current_buffer);
    if BASE_EQ (buf, (*blv).r#where)
      { swap_in_global_binding (sym); }
  }

  return variable;
});

/* Lisp functions for creating and removing buffer-local variables.  */

DEFUN! ("local-variable-p", Flocal_variable_p, Slocal_variable_p,
       1, 2, 0,
       doc: "Non-nil if VARIABLE has a local binding in buffer BUFFER.\nBUFFER defaults to the current buffer.\n\nAlso see `buffer-local-boundp'.",
       (variable: Lisp_Object, buffer: Lisp_Object),
       {
  let mut variable = variable;   // C's `XSETSYMBOL (variable, sym)` below rebinds the argument
  let buf: *mut buffer = decode_buffer (buffer);
  let mut sym: *mut Lisp_Symbol;

  CHECK_SYMBOL (variable);
  sym = XSYMBOL (variable);

  loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
        {
        SYMBOL_VARALIAS =>
          { sym = SYMBOL_ALIAS (sym); continue; }
        SYMBOL_PLAINVAL =>
          { return Qnil (); }
        SYMBOL_LOCALIZED =>
          {
            let tmp: Lisp_Object;
            let blv = SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value;
            XSETBUFFER! (tmp, buf);
            XSETSYMBOL! (variable, sym); /* Update in case of aliasing.  */

            if BASE_EQ ((*blv).r#where, tmp) /* The binding is already loaded.  */
              { return if blv_found (blv) { Qt () } else { Qnil () }; }
            else
              { return if NILP (assq_no_quit (variable, BVAR! (buf, local_var_alist_)))
                          { Qnil () }
                        else
                          { Qt () }; }
          }
        SYMBOL_FORWARDED =>
          {
            let valcontents: lispfwd = SYMBOL_FWD (sym) as lispfwd;
            if BUFFER_OBJFWDP (valcontents)
              {
                let offset = XBUFFER_OFFSET (valcontents) as c_int;
                let idx = PER_BUFFER_IDX (offset as ptrdiff_t);
                if idx == -1 || PER_BUFFER_VALUE_P (buf, idx)
                  { return Qt (); }
              }
            return Qnil ();
          }
        _ =>
          { emacs_abort (); }
        }
    }
});

DEFUN! ("local-variable-if-set-p", Flocal_variable_if_set_p, Slocal_variable_if_set_p,
       1, 2, 0,
       doc: "Non-nil if VARIABLE is local in buffer BUFFER when set there.\nBUFFER defaults to the current buffer.\n\nMore precisely, return non-nil if either VARIABLE already has a local\nvalue in BUFFER, or if VARIABLE is automatically buffer-local (see\n`make-variable-buffer-local').",
       (variable: Lisp_Object, buffer: Lisp_Object),
       {
  let mut variable = variable;   // C's `XSETSYMBOL (variable, sym)` below rebinds the argument
  let mut sym: *mut Lisp_Symbol;

  CHECK_SYMBOL (variable);
  sym = XSYMBOL (variable);

  loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
        {
        SYMBOL_VARALIAS =>
          { sym = SYMBOL_ALIAS (sym); continue; }
        SYMBOL_PLAINVAL =>
          { return Qnil (); }
        SYMBOL_LOCALIZED =>
          {
            let blv = SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value;
            /* C reads the one-bit `local_if_set` field; the mirror reads the same bit
               out of `u.s.bits` (lisp.rs 1971).  */
            if (*blv).bits & BLV_LOCAL_IF_SET != 0
              { return Qt (); }
            XSETSYMBOL! (variable, sym); /* Update in case of aliasing.  */
            return Flocal_variable_p (variable, buffer);
          }
        SYMBOL_FORWARDED =>
          /* All BUFFER_OBJFWD slots become local if they are set.  */
          { return if BUFFER_OBJFWDP (SYMBOL_FWD (sym) as lispfwd) { Qt () } else { Qnil () }; }
        _ =>
          { emacs_abort (); }
        }
    }
});

DEFUN! ("variable-binding-locus", Fvariable_binding_locus, Svariable_binding_locus,
       1, 1, 0,
       doc: "Return a value indicating where VARIABLE's current binding comes from.\nIf the current binding is buffer-local, the value is the current buffer.\nIf the current binding is global (the default), the value is nil.",
       (variable: Lisp_Object),
       {
  let mut sym: *mut Lisp_Symbol;

  CHECK_SYMBOL (variable);
  sym = XSYMBOL (variable);

  /* Make sure the current binding is actually swapped in.  */
  find_symbol_value (variable);

  loop
    {
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
        {
        SYMBOL_VARALIAS =>
          { sym = SYMBOL_ALIAS (sym); continue; }
        SYMBOL_PLAINVAL =>
          { return Qnil (); }
        SYMBOL_FORWARDED =>
          {
            let valcontents: lispfwd = SYMBOL_FWD (sym) as lispfwd;
            if KBOARD_OBJFWDP (valcontents)
              { return Fframe_terminal (selected_frame); }
            else if ! BUFFER_OBJFWDP (valcontents)
              { return Qnil (); }
            /* C's `FALLTHROUGH;` sends this arm on into the SYMBOL_LOCALIZED arm's
               statements: the port breaks out of the match to those statements.  */
          }
        SYMBOL_LOCALIZED =>
          {}
        _ =>
          { emacs_abort (); }
        }
      break;
    }

  /* For a local variable, record both the symbol and which
     buffer's or frame's value we are saving.  */
  if ! NILP (Flocal_variable_p (variable, Qnil ()))
    { return Fcurrent_buffer (); }
  else if (((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT) == SYMBOL_LOCALIZED
           && blv_found (SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value)
    { return (* (SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value)).r#where; }
  else
    { return Qnil (); }
});

/* Find the function at the end of a chain of symbol function indirections.  */

/* If OBJECT is a symbol, find the end of its function chain and
   return the value found there.  If OBJECT is not a symbol, just
   return it.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn indirect_function (object: Lisp_Object) -> Lisp_Object
{
  let mut object = object;
  while SYMBOLP (object) && ! NILP (object)
    { object = (*XSYMBOL (object)).u.s.function; }
  return object;
}

DEFUN! ("indirect-function", Findirect_function, Sindirect_function, 1, 2, 0,
       doc: "Return the function at the end of OBJECT's function chain.\nIf OBJECT is not a symbol, just return it.  Otherwise, follow all\nfunction indirections to find the final function binding and return it.",
       (object: Lisp_Object, noerror: Lisp_Object),
       {
  return indirect_function (object);
});

/* Extract and set vector and string elements.  */

DEFUN! ("aref", Faref, Saref, 2, 2, 0,
       doc: "Return the element of ARRAY at index IDX.\nARRAY may be a vector, a string, a char-table, a bool-vector, a record,\nor a byte-code object.  IDX starts at 0.",
       (array: Lisp_Object, idx: Lisp_Object),
       {
  let idxval: EMACS_INT;

  CHECK_FIXNUM (idx);
  idxval = XFIXNUM (idx);

  if STRINGP (array)
    {
      let c: c_int;
      let idxval_byte: ptrdiff_t;

      if idxval < 0 || idxval >= SCHARS (array)
        { args_out_of_range (array, idx); }
      if ! STRING_MULTIBYTE (array)
        { return make_fixnum (SREF (array, idxval as usize) as EMACS_INT); }
      idxval_byte = string_char_to_byte (array, idxval);

      c = STRING_CHAR (unsafe { SDATA (array).offset (idxval_byte) });
      return make_fixnum (c as EMACS_INT);
    }
  else if BOOL_VECTOR_P (array)
    {
      if idxval < 0 || idxval >= bool_vector_size (array)
        { args_out_of_range (array, idx); }
      return bool_vector_ref (array, idxval);
    }
  else if CHAR_TABLE_P (array)
    {
      CHECK_CHARACTER (idx);
      return CHAR_TABLE_REF (array, idxval);
    }
  else
    {
      let mut size: ptrdiff_t = 0;
      if VECTORP (array)
        { size = ASIZE (array); }
      else if CLOSUREP (array) || RECORDP (array)
        { size = PVSIZE (array); }
      else
        { wrong_type_argument (Qarrayp (), array); }

      if idxval < 0 || idxval >= size
        { args_out_of_range (array, idx); }
      return AREF (array, idxval);
    }
});

DEFUN! ("aset", Faset, Saset, 3, 3, 0,
       doc: "Store into the element of ARRAY at index IDX the value NEWELT.\nReturn NEWELT.  ARRAY may be a vector, a string, a char-table or a\nbool-vector.  IDX starts at 0.\nIf ARRAY is a unibyte string, NEWELT must be a single byte (0-255).\nIf ARRAY is a multibyte string, NEWELT and the previous character at\nindex IDX must both be ASCII (0-127).",
       (array: Lisp_Object, idx: Lisp_Object, newelt: Lisp_Object),
       {
  let idxval: EMACS_INT;

  CHECK_FIXNUM (idx);
  idxval = XFIXNUM (idx);
  if ! RECORDP (array)
    { CHECK_ARRAY (array, Qarrayp ()); }

  if VECTORP (array)
    {
      if idxval < 0 || idxval >= ASIZE (array)
        { args_out_of_range (array, idx); }
      ASET (array, idxval, newelt);
    }
  else if BOOL_VECTOR_P (array)
    {
      if idxval < 0 || idxval >= bool_vector_size (array)
        { args_out_of_range (array, idx); }
      bool_vector_set (array, idxval, ! NILP (newelt));
    }
  else if CHAR_TABLE_P (array)
    {
      CHECK_CHARACTER (idx);
      CHAR_TABLE_SET (array, idxval, newelt);
    }
  else if RECORDP (array)
    {
      if idxval < 0 || idxval >= PVSIZE (array)
        { args_out_of_range (array, idx); }
      ASET (array, idxval, newelt);
    }
  else /* STRINGP */
    {
      if idxval < 0 || idxval >= SCHARS (array)
        { args_out_of_range (array, idx); }
      CHECK_CHARACTER (newelt);
      let c = XFIXNAT (newelt) as c_int;

      if STRING_MULTIBYTE (array)
        {
          if c > 0x7f
            { error (b"Attempt to store non-ASCII char into multibyte string\0".as_ptr ()
                       as *const c_char); }
          let idxval_byte = string_char_to_byte (array, idxval);
          /* C's `unsigned char *p = SDATA (array) + idxval_byte;` - the byte itself is
             reached through the mirror's `SREF`/`SSET`, which take its index, so the port
             keeps C's name `p` as that index instead of a raw pointer (PORTING.md P-79:
             no `unsafe` where a safe helper exists, and the unibyte branch below already
             uses `SSET`).  */
          let p = idxval_byte as usize;
          if SREF (array, p) > 0x7f
            { error (b"Attempt to replace non-ASCII char in multibyte string\0".as_ptr ()
                       as *const c_char); }
          SSET (array, p, c as u8);
        }
      else
        {
          if c > 0xff
            { error (b"Attempt to store non-byte value into unibyte string\0".as_ptr ()
                       as *const c_char); }
          SSET (array, idxval as usize, c as u8);
        }
    }
  return newelt;
});

/* Arithmetic functions */

fn check_integer_coerce_marker (x: Lisp_Object) -> Lisp_Object
{
  if MARKERP (x)
    { return make_fixnum (unsafe { marker_position (x) }); }
  CHECK_TYPE (INTEGERP (x), Qinteger_or_marker_p (), x);
  return x;
}

fn check_number_coerce_marker (x: Lisp_Object) -> Lisp_Object
{
  if MARKERP (x)
    { return make_fixnum (unsafe { marker_position (x) }); }
  CHECK_TYPE (NUMBERP (x), Qnumber_or_marker_p (), x);
  return x;
}

fn coerce_marker (x: Lisp_Object) -> Lisp_Object
{
  return if MARKERP (x) { make_fixnum (unsafe { marker_position (x) }) } else { x };
}

fn not_number_or_marker (x: Lisp_Object) -> !
{
  unsafe { wrong_type_argument (Qnumber_or_marker_p (), x); }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arithcompare (mut num1: Lisp_Object, mut num2: Lisp_Object)
  -> cmp_bits_t
{
  num1 = coerce_marker (num1);
  num2 = coerce_marker (num2);

  let lt: bool;
  let eq: bool;
  let gt: bool;
  if FLOATP (num1)
    {
      let f1: c_double = XFLOAT_DATA (num1);
      if FLOATP (num2)
        {
          let f2: c_double = XFLOAT_DATA (num2);
          lt = f1 < f2;
          eq = f1 == f2;
          gt = f1 > f2;
        }
      else if FIXNUMP (num2)
        {
          /* Compare a float NUM1 to an integer NUM2 by converting the
             integer I2 (i.e., NUM2) to the double F2 (a conversion that
             can round on some platforms, if I2 is large enough), and then
             converting F2 back to the integer I1 (a conversion that is
             always exact), so that I1 exactly equals ((double) NUM2).  If
             floating-point comparison reports a tie, NUM1 = F1 = F2 = I1
             (exactly) so I1 - I2 = NUM1 - NUM2 (exactly), so comparing I1
             to I2 will break the tie correctly.  */
          let f2: c_double = XFIXNUM (num2) as c_double;
          if f1 == f2
            {
              let i1: EMACS_INT = f2 as EMACS_INT;
              let i2: EMACS_INT = XFIXNUM (num2);
              eq = i1 == i2;
              lt = i1 < i2;
              gt = i1 > i2;
            }
          else
            {
              eq = false;
              lt = f1 < f2;
              gt = f1 > f2;
            }
        }
      else if BIGNUMP (num2)
        {
          if isnan! (f1)
            { lt = false; eq = false; gt = false; }
          else
            {
              let cmp = unsafe { mpz_cmp_d (xbignum_val (num2), f1) };
              eq = cmp == 0;
              lt = cmp > 0;
              gt = cmp < 0;
            }
        }
      else
        { not_number_or_marker (num2); }
    }
  else if FIXNUMP (num1)
    {
      if FLOATP (num2)
        {
          /* Compare an integer NUM1 to a float NUM2.  This is the
             converse of comparing float to integer (see above).  */
          let f1: c_double = XFIXNUM (num1) as c_double;
          let f2: c_double = XFLOAT_DATA (num2);
          if f1 == f2
            {
              let i1: EMACS_INT = XFIXNUM (num1);
              let i2: EMACS_INT = f1 as EMACS_INT;
              eq = i1 == i2;
              lt = i1 < i2;
              gt = i1 > i2;
            }
          else
            {
              eq = false;
              lt = f1 < f2;
              gt = f1 > f2;
            }
        }
      else if FIXNUMP (num2)
        {
          let i1: EMACS_INT = XFIXNUM (num1);
          let i2: EMACS_INT = XFIXNUM (num2);
          eq = i1 == i2;
          lt = i1 < i2;
          gt = i1 > i2;
        }
      else if BIGNUMP (num2)
        {
          let sgn = unsafe { mpz_sgn (xbignum_val (num2)) };
          eassume! (sgn != 0);
          eq = false;
          lt = sgn > 0;
          gt = sgn < 0;
        }
      else
        { not_number_or_marker (num2); }
    }
  else if BIGNUMP (num1)
    {
      if FLOATP (num2)
        {
          let f2: c_double = XFLOAT_DATA (num2);
          if isnan! (f2)
            { lt = false; eq = false; gt = false; }
          else
            {
              let cmp = unsafe { mpz_cmp_d (xbignum_val (num1), f2) };
              eq = cmp == 0;
              lt = cmp < 0;
              gt = cmp > 0;
            }
        }
      else if FIXNUMP (num2)
        {
          let sgn = unsafe { mpz_sgn (xbignum_val (num1)) };
          eassume! (sgn != 0);
          eq = false;
          lt = sgn < 0;
          gt = sgn > 0;
        }
      else if BIGNUMP (num2)
        {
          let cmp = unsafe { mpz_cmp (xbignum_val (num1), xbignum_val (num2)) };
          eq = cmp == 0;
          lt = cmp < 0;
          gt = cmp > 0;
        }
      else
        { not_number_or_marker (num2); }
    }
  else
    { not_number_or_marker (num1); }

  return (lt as cmp_bits_t) << Cmp_Bit_LT | (gt as cmp_bits_t) << Cmp_Bit_GT
         | (eq as cmp_bits_t) << Cmp_Bit_EQ;
}

// unsafe: the body dereferences `args`, and only the caller knows it holds `nargs` valid
// objects (reviewer A's F3; the six callers are `unsafe extern "C" fn`s already).
unsafe fn arithcompare_driver (nargs: ptrdiff_t, args: *mut Lisp_Object, cmpmask: cmp_bits_t)
  -> Lisp_Object
{
  let mut i: ptrdiff_t = 1;
  while i < nargs
    {
      /* C's `&` binds *looser* than `==`, so C must write `!(x & mask)`; Rust's `&` binds
         *tighter*, so `x & mask != 0` already means `(x & mask) != 0` and the parentheses
         below are only defensive (reviewer A's F1).  */
      if ! ((unsafe { arithcompare (*args.offset (i - 1), *args.offset (i)) } & cmpmask) != 0)
        { return Qnil (); }
      i += 1;
    }
  return Qt ();
}

DEFUN! ("=", Feqlsign, Seqlsign, 1, MANY, 0,
       doc: "Return t if args, all numbers or markers, are equal.\nusage: (= NUMBER-OR-MARKER &rest NUMBERS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return arithcompare_driver (nargs, args, Cmp_EQ as cmp_bits_t);
});

DEFUN! ("<", Flss, Slss, 1, MANY, 0,
       doc: "Return t if each arg (a number or marker), is less than the next arg.\nusage: (< NUMBER-OR-MARKER &rest NUMBERS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 2 && FIXNUMP (unsafe { *args }) && FIXNUMP (unsafe { *args.offset (1) })
    {
      return if XFIXNUM (unsafe { *args }) < XFIXNUM (unsafe { *args.offset (1) })
                     { Qt () } else { Qnil () };
    }

  return arithcompare_driver (nargs, args, Cmp_LT as cmp_bits_t);
});

DEFUN! (">", Fgtr, Sgtr, 1, MANY, 0,
       doc: "Return t if each arg (a number or marker) is greater than the next arg.\nusage: (> NUMBER-OR-MARKER &rest NUMBERS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 2 && FIXNUMP (unsafe { *args }) && FIXNUMP (unsafe { *args.offset (1) })
    {
      return if XFIXNUM (unsafe { *args }) > XFIXNUM (unsafe { *args.offset (1) })
                     { Qt () } else { Qnil () };
    }

  return arithcompare_driver (nargs, args, Cmp_GT as cmp_bits_t);
});

DEFUN! ("<=", Fleq, Sleq, 1, MANY, 0,
       doc: "Return t if each arg (a number or marker) is less than or equal to the next.\nusage: (<= NUMBER-OR-MARKER &rest NUMBERS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 2 && FIXNUMP (unsafe { *args }) && FIXNUMP (unsafe { *args.offset (1) })
    {
      return if XFIXNUM (unsafe { *args }) <= XFIXNUM (unsafe { *args.offset (1) })
                     { Qt () } else { Qnil () };
    }

  return arithcompare_driver (nargs, args, (Cmp_LT | Cmp_EQ) as cmp_bits_t);
});

DEFUN! (">=", Fgeq, Sgeq, 1, MANY, 0,
       doc: "Return t if each arg (a number or marker) is greater than or equal to the next.\nusage: (>= NUMBER-OR-MARKER &rest NUMBERS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 2 && FIXNUMP (unsafe { *args }) && FIXNUMP (unsafe { *args.offset (1) })
    {
      return if XFIXNUM (unsafe { *args }) >= XFIXNUM (unsafe { *args.offset (1) })
                     { Qt () } else { Qnil () };
    }

  return arithcompare_driver (nargs, args, (Cmp_GT | Cmp_EQ) as cmp_bits_t);
});

DEFUN! ("/=", Fneq, Sneq, 2, 2, 0,
       doc: "Return t if first arg is not equal to second arg.  Both must be numbers or markers.",
       (num1: Lisp_Object, num2: Lisp_Object),
       {
  return if arithcompare (num1, num2) & (Cmp_EQ as cmp_bits_t) != 0 { Qnil () } else { Qt () };
});

/* gnulib's INT_BUFSIZE_BOUND (intprops.h 108-114) is `INT_STRLEN_BOUND (t) + 1`, where
   `INT_STRLEN_BOUND (t)` is `INT_BITS_STRLEN_BOUND (TYPE_WIDTH (t) - signed) + signed` and
   `INT_BITS_STRLEN_BOUND (b)` is `(b * 146 + 484) / 485`.  The macro is type-generic in C
   (`_GL_SIGNED_TYPE_OR_EXPR`), and `<intprops.h>` has no generic mirror (P-26/P-62), so
   this is the instantiation the one body below uses: `INT_BUFSIZE_BOUND (EMACS_INT)` = 21
   for the signed 64-bit EMACS_INT.  C's `max` (lisp.h 84) is the port's if/else.  */
const INT_BUFSIZE_BOUND: usize = ((EMACS_INT_WIDTH - 1) * 146 + 484) / 485 + 1 + 1;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cons_to_unsigned (c: Lisp_Object, max: uintmax_t) -> uintmax_t
{
  let mut valid: bool = false;
  /* C: `uintmax_t val UNINIT;` - `= {0,}` in this configuration (the round-7 decision).  */
  let mut val: uintmax_t = 0;

  if FLOATP (c)
    {
      let d: c_double = XFLOAT_DATA (c);
      if d >= 0.0 && d < 1.0 + max as c_double
        {
          val = d as uintmax_t;
          valid = val as c_double == d;
        }
    }
  else
    {
      let hi: Lisp_Object = if CONSP (c) { XCAR (c) } else { c };
      valid = INTEGERP (hi) && integer_to_uintmax (hi, &raw mut val);

      if valid && CONSP (c)
        {
          let top: uintmax_t = val;
          let mut rest: Lisp_Object = XCDR (c);
          if top <= UINTMAX_MAX >> 24 >> 16
              && CONSP (rest)
              && FIXNATP (XCAR (rest)) && XFIXNAT (XCAR (rest)) < 1 << 24
              && FIXNATP (XCDR (rest)) && XFIXNAT (XCDR (rest)) < 1 << 16
            {
              let mid: uintmax_t = XFIXNAT (XCAR (rest)) as uintmax_t;
              val = top << 24 << 16 | mid << 16 | XFIXNAT (XCDR (rest)) as uintmax_t;
            }
          else
            {
              valid = top <= UINTMAX_MAX >> 16;
              if valid
                {
                  if CONSP (rest)
                    { rest = XCAR (rest); }
                  valid = FIXNATP (rest) && XFIXNAT (rest) < 1 << 16;
                  if valid
                    { val = top << 16 | XFIXNAT (rest) as uintmax_t; }
                }
            }
        }
    }

  if ! (valid && val <= max)
    {
      error (b"Not an in-range integer, integral float, or cons of integers\0".as_ptr ()
               as *const c_char);
    }
  return val;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cons_to_signed (c: Lisp_Object, min: intmax_t, max: intmax_t)
  -> intmax_t
{
  let mut valid: bool = false;
  /* C: `intmax_t val UNINIT;` - `= {0,}` in this configuration (the round-7 decision).  */
  let mut val: intmax_t = 0;

  if FLOATP (c)
    {
      let d: c_double = XFLOAT_DATA (c);
      if d >= min as c_double && d < 1.0 + max as c_double
        {
          val = d as intmax_t;
          valid = val as c_double == d;
        }
    }
  else
    {
      let hi: Lisp_Object = if CONSP (c) { XCAR (c) } else { c };
      valid = INTEGERP (hi) && integer_to_intmax (hi, &raw mut val);

      if valid && CONSP (c)
        {
          let top: intmax_t = val;
          let mut rest: Lisp_Object = XCDR (c);
          if top >= INTMAX_MIN >> 24 >> 16 && top <= INTMAX_MAX >> 24 >> 16
              && CONSP (rest)
              && FIXNATP (XCAR (rest)) && XFIXNAT (XCAR (rest)) < 1 << 24
              && FIXNATP (XCDR (rest)) && XFIXNAT (XCDR (rest)) < 1 << 16
            {
              let mid: intmax_t = XFIXNAT (XCAR (rest)) as intmax_t;
              val = top << 24 << 16 | mid << 16 | XFIXNAT (XCDR (rest)) as intmax_t;
            }
          else
            {
              valid = INTMAX_MIN >> 16 <= top && top <= INTMAX_MAX >> 16;
              if valid
                {
                  if CONSP (rest)
                    { rest = XCAR (rest); }
                  valid = FIXNATP (rest) && XFIXNAT (rest) < 1 << 16;
                  if valid
                    { val = top << 16 | XFIXNAT (rest) as intmax_t; }
                }
            }
        }
    }

  if ! (valid && min <= val && val <= max)
    {
      error (b"Not an in-range integer, integral float, or cons of integers\0".as_ptr ()
               as *const c_char);
    }
  return val;
}

/* Render NUMBER in decimal into BUFFER which ends right before END.
   Return the start of the string; the end is always at END.
   The string is not null-terminated.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fixnum_to_string (number: EMACS_INT, buffer: *mut c_char,
                                           end: *mut c_char) -> *mut c_char
{
  let mut x: EMACS_INT = number;
  let negative: bool = x < 0;
  if negative
    { x = -x; }
  let mut p: *mut c_char = end;
  loop
    {
      eassume! (p > buffer && unsafe { p.offset (-1) } < end);
      p = unsafe { p.offset (-1) };
      unsafe { *p = b'0' as c_char + (x % 10) as c_char; };
      x /= 10;
      if x == 0
        { break; }
    }
  if negative
    {
      p = unsafe { p.offset (-1) };
      unsafe { *p = b'-' as c_char; };
    }
  return p;
}

DEFUN! ("number-to-string", Fnumber_to_string, Snumber_to_string, 1, 1, 0,
       doc: "Return the decimal representation of NUMBER as a string.\nUses a minus sign if negative.\nNUMBER may be an integer or a floating point number.",
       (number: Lisp_Object),
       {
  /* C: `char buffer[max (FLOAT_TO_STRING_BUFSIZE, INT_BUFSIZE_BOUND (EMACS_INT))];` - it is
     uninitialised storage that the branches below fill through a `char *`, which the port
     says with MaybeUninit rather than zeroing 350 bytes C never writes (the lisp.rs
     `candidate_string` precedent).  */
  let mut buffer = core::mem::MaybeUninit::<[c_char;
      if FLOAT_TO_STRING_BUFSIZE > INT_BUFSIZE_BOUND { FLOAT_TO_STRING_BUFSIZE }
      else { INT_BUFSIZE_BOUND }]>::uninit ();

  if FIXNUMP (number)
    {
      let base = buffer.as_mut_ptr () as *mut c_char;
      let end = unsafe { base.add (core::mem::size_of_val (&buffer)) };
      let p = unsafe { fixnum_to_string (XFIXNUM (number), base, end) };
      return unsafe { make_unibyte_string (p, end.offset_from (p)) };
    }

  if BIGNUMP (number)
    { return unsafe { bignum_to_string (number, 10) }; }

  if FLOATP (number)
    {
      return unsafe {
        make_unibyte_string (buffer.as_ptr () as *const c_char,
                             float_to_string (buffer.as_mut_ptr () as *mut c_char,
                                              XFLOAT_DATA (number)) as ptrdiff_t)
      };
    }

  wrong_type_argument (Qnumberp (), number);
});

DEFUN! ("string-to-number", Fstring_to_number, Sstring_to_number, 1, 2, 0,
       doc: "Parse STRING as a decimal number and return the number.\nIgnore leading spaces and tabs, and all trailing chars.  Return 0 if\nSTRING cannot be parsed as an integer or floating point number.\n\nIf BASE, interpret STRING as a number in that base.  If BASE isn't\npresent, base 10 is used.  BASE must be between 2 and 16 (inclusive).\nIf the base used is not 10, STRING is always parsed as an integer.",
       (string: Lisp_Object, base: Lisp_Object),
       {
  let b: c_int;

  CHECK_STRING (string);

  if NILP (base)
    { b = 10; }
  else
    {
      CHECK_FIXNUM (base);
      if ! (XFIXNUM (base) >= 2 && XFIXNUM (base) <= 16)
        { xsignal1 (Qargs_out_of_range (), base); }
      b = XFIXNUM (base) as c_int;
    }

  let mut p: *mut c_char = SSDATA (string);
  while unsafe { *p } == b' ' as c_char || unsafe { *p } == b'\t' as c_char
    { p = unsafe { p.offset (1) }; }

  let val: Lisp_Object = string_to_number (p, b, std::ptr::null_mut ());
  return if (if IEEE_FLOATING_POINT { NILP (val) } else { ! NUMBERP (val) })
           { make_fixnum (0) } else { val };
});

// data.c 3125-3135: the arithmetic codes the drivers below switch on.  C's plain enum is
// a tag that never reaches an ABI, but `#[repr(C)]` keeps C's representation and the
// derived `PartialOrd` keeps `code <= Adiv`'s meaning (`floating_point_op`).
#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum arithop
  {
    Aadd,
    Asub,
    Amult,
    Adiv,
    Alogand,
    Alogior,
    Alogxor
  }
use arithop::*;

fn floating_point_op (code: arithop) -> bool
{
  return code <= Adiv;
}

/* Return the result of applying the floating-point operation CODE to
   the NARGS arguments starting at ARGS.  If ARGNUM is positive,
   ARGNUM of the arguments were already consumed, yielding ACCUM.
   0 <= ARGNUM < NARGS, 2 <= NARGS, and NEXT is the value of
   ARGS[ARGSNUM], converted to double.  */

unsafe fn floatop_arith_driver (code: arithop, nargs: ptrdiff_t, args: *mut Lisp_Object,
                                mut argnum: ptrdiff_t, mut accum: c_double,
                                mut next: c_double)
  -> Lisp_Object
{
  // C's `goto next_arg` (data.c 3156) jumps from above the `while` into the loop's
  // tail, skipping the switch on that first pass; `skip_switch` is the jump and the
  // statements after it are the loop's `next_arg:` block.
  let mut skip_switch: bool = argnum == 0;
  if skip_switch
    { accum = next; }

  'next_arg: loop
    {
      if ! skip_switch
        {
          match code
            {
            Aadd => { accum += next; }
            Asub => { accum -= next; }
            Amult => { accum *= next; }
            Adiv =>
              {
                if ! IEEE_FLOATING_POINT && next == 0.0
                  { xsignal0 (Qarith_error ()); }
                accum /= next;
              }
            _ => { eassume! (false); }
            };
        }
      skip_switch = false;

      /* C: `next_arg:` */
      argnum += 1;
      if argnum == nargs
        { return make_float (accum); }
      next = XFLOATINT (check_number_coerce_marker (unsafe { *args.offset (argnum) }));
      continue 'next_arg;
    }
}

/* Like floatop_arith_driver, except CODE might not be a floating-point
   operation, and NEXT is a Lisp float rather than a C double.  */

unsafe fn float_arith_driver (code: arithop, nargs: ptrdiff_t, args: *mut Lisp_Object,
                              argnum: ptrdiff_t, accum: c_double, next: Lisp_Object)
  -> Lisp_Object
{
  if ! floating_point_op (code)
    { wrong_type_argument (Qinteger_or_marker_p (), next); }
  return floatop_arith_driver (code, nargs, args, argnum, accum,
                               XFLOAT_DATA (next));
}

/* Return the result of applying the arithmetic operation CODE to the
   NARGS arguments starting at ARGS.  If ARGNUM is positive, ARGNUM of
   the arguments were already consumed, yielding IACCUM.  0 <= ARGNUM
   < NARGS, 2 <= NARGS, and VAL is the value of ARGS[ARGSNUM],
   converted to integer.  */

unsafe fn bignum_arith_driver (code: arithop, nargs: ptrdiff_t, args: *mut Lisp_Object,
                               mut argnum: ptrdiff_t, iaccum: intmax_t,
                               mut val: Lisp_Object)
  -> Lisp_Object
{
  let mut accum: *const __mpz_struct;
  // C's `goto next_arg` (data.c 3203) is the same jump shape as the float driver's: it
  // enters the loop's tail and skips the switch on the first pass.
  let mut skip_switch: bool = argnum == 0;
  if argnum == 0
    { accum = bignum_integer (&raw mut mpz[0], val); }
  else
    {
      mpz_set_intmax (mpz[0].as_mut_ptr (), iaccum);
      accum = &raw const mpz[0] as *const __mpz_struct;
    }

  'next_arg: loop
    {
      if ! skip_switch
        {
          /* C: `mpz_t const *next = bignum_integer (&mpz[1], val);` - the declaration
             sits inside the loop but the `goto next_arg` entry skips it, so the port
             spells it on the fall-through path only.  `*next`/`*accum` are the port's
             already-decayed `mpz_t const *` (the floatfns.rs precedent).  */
          let next: *const __mpz_struct = bignum_integer (&raw mut mpz[1], val);

          match code
            {
            Aadd   => {       mpz_add (mpz[0].as_mut_ptr (), accum, next); }
            Asub   => {       mpz_sub (mpz[0].as_mut_ptr (), accum, next); }
            Amult  => { emacs_mpz_mul (mpz[0].as_mut_ptr (), accum, next); }
            Alogand => {      mpz_and (mpz[0].as_mut_ptr (), accum, next); }
            Alogior => {      mpz_ior (mpz[0].as_mut_ptr (), accum, next); }
            Alogxor => {      mpz_xor (mpz[0].as_mut_ptr (), accum, next); }
            Adiv =>
              {
                if mpz_sgn (next) == 0
                  { xsignal0 (Qarith_error ()); }
                mpz_tdiv_q (mpz[0].as_mut_ptr (), accum, next);
              }
            _ => { eassume! (false); }
            };
          accum = &raw const mpz[0] as *const __mpz_struct;
        }
      skip_switch = false;

      /* C: `next_arg:` */
      argnum += 1;
      if argnum == nargs
        { return make_integer_mpz (); }
      val = check_number_coerce_marker (unsafe { *args.offset (argnum) });
      if FLOATP (val)
        { return float_arith_driver (code, nargs, args, argnum,
                                     mpz_get_d_rounded (accum), val); }
      continue 'next_arg;
    }
}

/* Return the result of applying the arithmetic operation CODE to the
   NARGS arguments starting at ARGS, with the first argument being the
   number VAL.  2 <= NARGS.  Check that the remaining arguments are
   numbers or markers.  */

unsafe fn arith_driver (code: arithop, nargs: ptrdiff_t, args: *mut Lisp_Object,
                        mut val: Lisp_Object)
  -> Lisp_Object
{
  eassume! (2 <= nargs);

  let mut argnum: ptrdiff_t = 0;
  /* Set ACCUM to VAL's value if it is a fixnum, otherwise to some
     ignored value to avoid using an uninitialized variable later.  */
  let mut accum: intmax_t = XFIXNUM_RAW (val) as intmax_t;

  if FIXNUMP (val)
    { while true
      {
        argnum += 1;
        if argnum == nargs
          { return make_int (accum); }
        val = check_number_coerce_marker (unsafe { *args.offset (argnum) });

        /* Set NEXT to the next value if it fits, else exit the loop.  */
        let mut next: intmax_t = 0;
        if ! (INTEGERP (val) && integer_to_intmax (val, &raw mut next))
          { break; }

        /* Set ACCUM to the next operation's result if it fits,
           else exit the loop.  */
        /* C's `bool overflow; intmax_t a;` are filled by every arm that does not
           `continue`, which is why the two zero initialisers are unobservable.  */
        let mut overflow: bool = false;
        let mut a: intmax_t = 0;
              match code
          {
          Aadd  => { (a, overflow) = accum.overflowing_add (next); }
          Amult => { (a, overflow) = accum.overflowing_mul (next); }
          Asub  => { (a, overflow) = accum.overflowing_sub (next); }
          Adiv =>
            {
              if next == 0
                { xsignal0 (Qarith_error ()); }
              /* This cannot overflow, as integer overflow can
                 occur only if the dividend is INTMAX_MIN, but
                 INTMAX_MIN < MOST_NEGATIVE_FIXNUM <= accum.  */
              accum /= next;
              continue;
            }
          Alogand => { accum &= next; continue; }
          Alogior => { accum |= next; continue; }
          Alogxor => { accum ^= next; continue; }
          _ => { eassume! (false); }
          };
        if overflow
          { break; }
        accum = a;
      }
    }

  return if FLOATP (val)
           { float_arith_driver (code, nargs, args, argnum, accum as c_double, val) }
         else
           { bignum_arith_driver (code, nargs, args, argnum, accum, val) };
}

DEFUN! ("+", Fplus, Splus, 0, MANY, 0,
       doc: "Return sum of any number of arguments, which are numbers or markers.\nusage: (+ &rest NUMBERS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 0
    { return make_fixnum (0); }
  let a: Lisp_Object = check_number_coerce_marker (unsafe { *args });
  return if nargs == 1 { a } else { arith_driver (Aadd, nargs, args, a) };
});

DEFUN! ("-", Fminus, Sminus, 0, MANY, 0,
       doc: "Negate number or subtract numbers or markers and return the result.\nWith one arg, negates it.  With more than one arg,\nsubtracts all but the first from the first.\nusage: (- &optional NUMBER-OR-MARKER &rest MORE-NUMBERS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 0
    { return make_fixnum (0); }
  let a: Lisp_Object = check_number_coerce_marker (unsafe { *args });
  if nargs == 1
    {
      if FIXNUMP (a)
        { return make_int ((-XFIXNUM (a)) as intmax_t); }
      if FLOATP (a)
        { return make_float (-XFLOAT_DATA (a)); }
      mpz_neg (mpz[0].as_mut_ptr (), xbignum_val (a));
      return make_integer_mpz ();
    }
  return arith_driver (Asub, nargs, args, a);
});

DEFUN! ("*", Ftimes, Stimes, 0, MANY, 0,
       doc: "Return product of any number of arguments, which are numbers or markers.\nusage: (* &rest NUMBERS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 0
    { return make_fixnum (1); }
  let a: Lisp_Object = check_number_coerce_marker (unsafe { *args });
  return if nargs == 1 { a } else { arith_driver (Amult, nargs, args, a) };
});

DEFUN! ("/", Fquo, Squo, 1, MANY, 0,
       doc: "Divide number by divisors and return the result.\nWith two or more arguments, return first argument divided by the rest.\nWith one argument, return 1 divided by the argument.\nThe arguments must be numbers or markers.\nusage: (/ NUMBER &rest DIVISORS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let a: Lisp_Object = check_number_coerce_marker (unsafe { *args });
  if nargs == 1
    {
      if FIXNUMP (a)
        {
          if XFIXNUM (a) == 0
            { xsignal0 (Qarith_error ()); }
          return make_fixnum (1 / XFIXNUM (a));
        }
      if FLOATP (a)
        {
          if ! IEEE_FLOATING_POINT && XFLOAT_DATA (a) == 0.0
            { xsignal0 (Qarith_error ()); }
          return make_float (1.0 / XFLOAT_DATA (a));
        }
      /* Dividing 1 by any bignum yields 0.  */
      return make_fixnum (0);
    }

  /* Do all computation in floating-point if any arg is a float.  */
  let mut argnum: ptrdiff_t = 2;
  while argnum < nargs
    {
      if FLOATP (unsafe { *args.offset (argnum) })
        { return floatop_arith_driver (Adiv, nargs, args, 0, 0.0, XFLOATINT (a)); }
      argnum += 1;
    }
  return arith_driver (Adiv, nargs, args, a);
});

/* Return NUM % DEN (or NUM mod DEN, if MODULO).  NUM and DEN must be
   integers.  */
unsafe fn integer_remainder (num: Lisp_Object, den: Lisp_Object, modulo: bool)
  -> Lisp_Object
{
  if FIXNUMP (den)
    {
      let d: EMACS_INT = XFIXNUM (den);
      if d == 0
        { xsignal0 (Qarith_error ()); }

      /* C: `EMACS_INT r;` - the port's zero initialiser is the UNINIT class (C's
         `= {0,}` spelling), and `r` is read only under HAVE_R.  */
      let mut r: EMACS_INT = 0;
      let mut have_r: bool = false;
      if FIXNUMP (num)
        {
          r = XFIXNUM (num) % d;
          have_r = true;
        }
      else if eabs! (d) as c_ulong <= ULONG_MAX
        {
          let n: *const __mpz_struct = xbignum_val (num);
          let neg_n: bool = mpz_sgn (n) < 0;
          r = mpz_tdiv_ui (n, eabs! (d) as c_ulong) as EMACS_INT;
          if neg_n
            { r = -r; }
          have_r = true;
        }

      if have_r
        {
          /* If MODULO and the remainder has the wrong sign, fix it.  */
          if modulo && (if d < 0 { r > 0 } else { r < 0 })
            { r += d; }

          return make_fixnum (r);
        }
    }

  let d: *const __mpz_struct = bignum_integer (&raw mut mpz[1], den);
  let r: *mut mpz_t = &raw mut mpz[0];
  mpz_tdiv_r ((*r).as_mut_ptr (), bignum_integer (&raw mut mpz[0], num), d);

  if modulo
    {
      /* If the remainder has the wrong sign, fix it.  */
      let sgn_r: c_int = mpz_sgn ((*r).as_ptr ());
      if (if mpz_sgn (d) < 0 { sgn_r > 0 } else { sgn_r < 0 })
        { mpz_add ((*r).as_mut_ptr (), (*r).as_ptr (), d); }
    }

  return make_integer_mpz ();
}

DEFUN! ("%", Frem, Srem, 2, 2, 0,
       doc: "Return remainder of X divided by Y.\nBoth must be integers or markers.",
       (x: Lisp_Object, y: Lisp_Object),
       {
  /* C's parameters are writable; DEFUN!'s pattern takes plain idents, so the port
     re-binds them (the `let mut X = X;` convention).  */
  let mut x: Lisp_Object = x;
  let mut y: Lisp_Object = y;
  x = check_integer_coerce_marker (x);
  y = check_integer_coerce_marker (y);
  return integer_remainder (x, y, false);
});

DEFUN! ("mod", Fmod, Smod, 2, 2, 0,
       doc: "Return X modulo Y.\nThe result falls between zero (inclusive) and Y (exclusive).\nBoth X and Y must be numbers or markers.",
       (x: Lisp_Object, y: Lisp_Object),
       {
  let mut x: Lisp_Object = x;
  let mut y: Lisp_Object = y;
  x = check_number_coerce_marker (x);
  y = check_number_coerce_marker (y);
  if FLOATP (x) || FLOATP (y)
    { return fmod_float (x, y); }
  return integer_remainder (x, y, true);
});

unsafe fn minmax_driver (nargs: ptrdiff_t, args: *mut Lisp_Object, cmpmask: cmp_bits_t)
  -> Lisp_Object
{
  let mut accum: Lisp_Object = check_number_coerce_marker (unsafe { *args });
  let mut argnum: ptrdiff_t = 1;
  while argnum < nargs
    {
      let val: Lisp_Object = check_number_coerce_marker (unsafe { *args.offset (argnum) });
      if (unsafe { arithcompare (val, accum) } & cmpmask) != 0
        { accum = val; }
      else if FLOATP (val) && isnan! (XFLOAT_DATA (val))
        { return val; }
      argnum += 1;
    }
  return accum;
}

DEFUN! ("max", Fmax, Smax, 1, MANY, 0,
       doc: "Return largest of all the arguments (which must be numbers or markers).\nThe value is always a number; markers are converted to numbers.\nusage: (max NUMBER-OR-MARKER &rest NUMBERS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return minmax_driver (nargs, args, Cmp_GT as cmp_bits_t);
});

DEFUN! ("min", Fmin, Smin, 1, MANY, 0,
       doc: "Return smallest of all the arguments (which must be numbers or markers).\nThe value is always a number; markers are converted to numbers.\nusage: (min NUMBER-OR-MARKER &rest NUMBERS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  return minmax_driver (nargs, args, Cmp_LT as cmp_bits_t);
});

DEFUN! ("logand", Flogand, Slogand, 0, MANY, 0,
       doc: "Return bitwise-and of all the arguments.\nArguments may be integers, or markers converted to integers.\nusage: (logand &rest INTS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 0
    { return make_fixnum (-1); }
  let a: Lisp_Object = check_integer_coerce_marker (unsafe { *args });
  return if nargs == 1 { a } else { arith_driver (Alogand, nargs, args, a) };
});

DEFUN! ("logior", Flogior, Slogior, 0, MANY, 0,
       doc: "Return bitwise-or of all the arguments.\nArguments may be integers, or markers converted to integers.\nusage: (logior &rest INTS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 0
    { return make_fixnum (0); }
  let a: Lisp_Object = check_integer_coerce_marker (unsafe { *args });
  return if nargs == 1 { a } else { arith_driver (Alogior, nargs, args, a) };
});

DEFUN! ("logxor", Flogxor, Slogxor, 0, MANY, 0,
       doc: "Return bitwise-exclusive-or of all the arguments.\nArguments may be integers, or markers converted to integers.\nusage: (logxor &rest INTS-OR-MARKERS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  if nargs == 0
    { return make_fixnum (0); }
  let a: Lisp_Object = check_integer_coerce_marker (unsafe { *args });
  return if nargs == 1 { a } else { arith_driver (Alogxor, nargs, args, a) };
});

DEFUN! ("logcount", Flogcount, Slogcount, 1, 1, 0,
       doc: "Return population count of VALUE.\nThis is the number of one bits in the two's complement representation\nof VALUE.  If VALUE is negative, return the number of zero bits in the\nrepresentation.",
       (value: Lisp_Object),
       {
  CHECK_INTEGER (value);

  if BIGNUMP (value)
    {
      let mut nonneg: *const __mpz_struct = xbignum_val (value);
      if mpz_sgn (nonneg) < 0
        {
          mpz_com (mpz[0].as_mut_ptr (), nonneg);
          nonneg = &raw const mpz[0] as *const __mpz_struct;
        }
      return make_fixnum (mpz_popcount (nonneg) as EMACS_INT);
    }

  eassume! (FIXNUMP (value));
  let v: EMACS_UINT = (if XFIXNUM (value) < 0 { -1 - XFIXNUM (value) }
                       else { XFIXNUM (value) }) as EMACS_UINT;
  return make_fixnum (stdc_count_ones! (v) as EMACS_INT);
});

DEFUN! ("ash", Fash, Sash, 2, 2, 0,
       doc: "Return integer VALUE with its bits shifted left by COUNT bit positions.\nIf COUNT is negative, shift VALUE to the right instead.\nVALUE and COUNT must be integers.\nMathematically, the return value is VALUE multiplied by 2 to the\npower of COUNT, rounded down.  If the result is non-zero, its sign\nis the same as that of VALUE.\nIn terms of bits, when COUNT is positive, the function moves\nthe bits of VALUE to the left, adding zero bits on the right; when\nCOUNT is negative, it moves the bits of VALUE to the right,\ndiscarding bits.",
       (value: Lisp_Object, count: Lisp_Object),
       {
  CHECK_INTEGER (value);
  CHECK_INTEGER (count);

  if ! FIXNUMP (count)
    {
      if BASE_EQ (value, make_fixnum (0))
        { return value; }
      if mpz_sgn (xbignum_val (count)) < 0
        {
          let v: EMACS_INT = if FIXNUMP (value) { XFIXNUM (value) }
                             else { mpz_sgn (xbignum_val (value)) as EMACS_INT };
          return make_fixnum (if v < 0 { -1 } else { 0 });
        }
      overflow_error ();
    }

  let c: EMACS_INT = XFIXNUM (count);
  if c <= 0
    {
      if c == 0
        { return value; }

      if (-1 as EMACS_INT) >> 1 == -1 && FIXNUMP (value)
        {
          let shift: EMACS_INT = -c;
          let result: EMACS_INT = if shift < EMACS_INT_WIDTH as EMACS_INT
                                  { XFIXNUM (value) >> shift }
                                  else if XFIXNUM (value) < 0 { -1 } else { 0 };
          return make_fixnum (result);
        }
    }
  else if FIXNUMP (value)
    {
      let v: EMACS_INT = XFIXNUM (value);
      let uv: EMACS_UINT = v as EMACS_UINT;
      let uvcomp: EMACS_UINT = if v < 0 { ! uv } else { uv };
      let lz: EMACS_INT = stdc_leading_zeros! (uvcomp) as EMACS_INT;
      if (EMACS_INT_WIDTH as EMACS_INT) - (FIXNUM_BITS as EMACS_INT) < lz - c
        { return make_fixnum ((uv << c) as EMACS_INT); }
    }

  let zval: *const __mpz_struct = bignum_integer (&raw mut mpz[0], value);
  if c < 0
    {
      // C's `TYPE_MAXIMUM (mp_bitcnt_t) < -c` is gnulib intprops.h's type maximum,
      // ULONG_MAX for mp_bitcnt_t (see docs/ports/macro-body-notes.tsv).
      if mp_bitcnt_t::MAX < (-c) as mp_bitcnt_t
        { return make_fixnum (if mpz_sgn (zval) < 0 { -1 } else { 0 }); }
      mpz_fdiv_q_2exp (mpz[0].as_mut_ptr (), zval, (-c) as mp_bitcnt_t);
    }
  else
    { emacs_mpz_mul_2exp (mpz[0].as_mut_ptr (), zval, c); }
  return make_integer_mpz ();
});

/* Return X ** Y as an integer.  X and Y must be integers, and Y must
   be nonnegative.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn expt_integer (x: Lisp_Object, y: Lisp_Object) -> Lisp_Object
{
  /* Special cases for -1 <= x <= 1, which never overflow.  */
  if BASE_EQ (x, make_fixnum (1))
    { return x; }
  if BASE_EQ (x, make_fixnum (0))
    { return if BASE_EQ (x, y) { make_fixnum (1) } else { x }; }
  if BASE_EQ (x, make_fixnum (-1))
    {
      return if (if FIXNUMP (y) { XFIXNUM (y) & 1 != 0 }
                 else { mpz_odd_p! (xbignum_val (y)) != 0 })
               { x } else { make_fixnum (1) };
    }

  let exp: c_ulong;
  if FIXNUMP (y)
    {
      if ULONG_MAX < XFIXNUM (y) as c_ulong
        { overflow_error (); }
      exp = XFIXNUM (y) as c_ulong;
    }
  else
    {
      if ULONG_MAX <= MOST_POSITIVE_FIXNUM as c_ulong
          || mpz_fits_ulong_p (xbignum_val (y)) == 0
        { overflow_error (); }
      exp = mpz_get_ui (xbignum_val (y));
    }

  emacs_mpz_pow_ui (mpz[0].as_mut_ptr (), bignum_integer (&raw mut mpz[0], x), exp);
  return make_integer_mpz ();
}

DEFUN! ("1+", Fadd1, Sadd1, 1, 1, 0,
       doc: "Return NUMBER plus one.  NUMBER may be a number or a marker.\nMarkers are converted to integers.",
       (number: Lisp_Object),
       {
  let number: Lisp_Object = check_number_coerce_marker (number);

  if FIXNUMP (number)
    { return make_int ((XFIXNUM (number) + 1) as intmax_t); }
  if FLOATP (number)
    { return make_float (1.0 + XFLOAT_DATA (number)); }
  mpz_add_ui (mpz[0].as_mut_ptr (), xbignum_val (number), 1);
  return make_integer_mpz ();
});

DEFUN! ("1-", Fsub1, Ssub1, 1, 1, 0,
       doc: "Return NUMBER minus one.  NUMBER may be a number or a marker.\nMarkers are converted to integers.",
       (number: Lisp_Object),
       {
  let number: Lisp_Object = check_number_coerce_marker (number);

  if FIXNUMP (number)
    { return make_int ((XFIXNUM (number) - 1) as intmax_t); }
  if FLOATP (number)
    { return make_float (-1.0 + XFLOAT_DATA (number)); }
  mpz_sub_ui (mpz[0].as_mut_ptr (), xbignum_val (number), 1);
  return make_integer_mpz ();
});

DEFUN! ("lognot", Flognot, Slognot, 1, 1, 0,
       doc: "Return the bitwise complement of NUMBER.  NUMBER must be an integer.",
       (number: Lisp_Object),
       {
  CHECK_INTEGER (number);
  if FIXNUMP (number)
    { return make_fixnum (! XFIXNUM (number)); }
  mpz_com (mpz[0].as_mut_ptr (), xbignum_val (number));
  return make_integer_mpz ();
});

DEFUN! ("byteorder", Fbyteorder, Sbyteorder, 0, 0, 0,
       doc: "Return the byteorder for the machine.\nReturns 66 (ASCII uppercase B) for big endian machines or 108 (ASCII\nlowercase l) for small endian machines.",
       (),
       {
  let i: c_uint = 0x04030201;
  let order: c_int = if unsafe { *(&i as *const c_uint as *const c_char) } == 1 { 108 } else { 66 };

  return make_fixnum (order as EMACS_INT);
});

/* Because we round up the bool vector allocate size to word_size
   units, we can safely read past the "end" of the vector in the
   operations below.  These extra bits are always zero.  */

fn bool_vector_spare_mask (nr_bits: EMACS_INT) -> bits_word
{
  return ((1 as bits_word) << (nr_bits as usize % BITS_PER_BITS_WORD)) - 1;
}

// data.c 3711-3716: the destination-vector operations the driver switches on.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum bool_vector_op { bool_vector_exclusive_or,
                      bool_vector_union,
                      bool_vector_intersection,
                      bool_vector_set_difference,
                      bool_vector_subsetp }
use bool_vector_op::*;

unsafe fn bool_vector_binop_driver (a: Lisp_Object, b: Lisp_Object, mut dest: Lisp_Object,
                                    op: bool_vector_op)
  -> Lisp_Object
{
  let nr_bits: EMACS_INT;
  let adata: *mut bits_word;
  let bdata: *mut bits_word;
  let destdata: *mut bits_word;
  let mut i: ptrdiff_t = 0;
  let nr_words: ptrdiff_t;

  CHECK_BOOL_VECTOR (a);
  CHECK_BOOL_VECTOR (b);

  nr_bits = bool_vector_size (a);
  if bool_vector_size (b) != nr_bits
    { wrong_length_argument (a, b, dest); }

  nr_words = bool_vector_words (nr_bits);
  adata = bool_vector_data (a);
  bdata = bool_vector_data (b);

  // C's `goto set_dest` (data.c 3778…) leaves the `if` branch's switch and lands on the
  // tail switch, skipping the `return Qnil`; the port spells that jump as a labelled
  // loop whose head is the `if`/`else` and whose label is C's target.
  'set_dest: loop
    {
      if NILP (dest)
        {
          dest = make_uninit_bool_vector (nr_bits);
          destdata = bool_vector_data (dest);
        }
      else
        {
          CHECK_BOOL_VECTOR (dest);
          destdata = bool_vector_data (dest);
          if bool_vector_size (dest) != nr_bits
            { wrong_length_argument (a, b, dest); }

          match op
            {
            bool_vector_exclusive_or =>
              {
                while i < nr_words
                  {
                    if unsafe { *destdata.offset (i) }
                       != (unsafe { *adata.offset (i) } ^ unsafe { *bdata.offset (i) })
                      { break 'set_dest; }
                    i += 1;
                  }
              }

            bool_vector_subsetp =>
              {
                while i < nr_words
                  {
                    if unsafe { *adata.offset (i) } & (! unsafe { *bdata.offset (i) }) != 0
                      { return Qnil (); }
                    i += 1;
                  }
                return Qt ();
              }

            bool_vector_union =>
              {
                while i < nr_words
                  {
                    if unsafe { *destdata.offset (i) }
                       != (unsafe { *adata.offset (i) } | unsafe { *bdata.offset (i) })
                      { break 'set_dest; }
                    i += 1;
                  }
              }

            bool_vector_intersection =>
              {
                while i < nr_words
                  {
                    if unsafe { *destdata.offset (i) }
                       != (unsafe { *adata.offset (i) } & unsafe { *bdata.offset (i) })
                      { break 'set_dest; }
                    i += 1;
                  }
              }

            bool_vector_set_difference =>
              {
                while i < nr_words
                  {
                    if unsafe { *destdata.offset (i) }
                       != (unsafe { *adata.offset (i) } & (! unsafe { *bdata.offset (i) }))
                      { break 'set_dest; }
                    i += 1;
                  }
              }

            _ => { eassume! (false); }
            };

          return Qnil ();
        }
      break;
    }

  // C: `set_dest:`
  match op
    {
    bool_vector_exclusive_or =>
      {
        while i < nr_words
          {
            unsafe { *destdata.offset (i) = unsafe { *adata.offset (i) } ^ unsafe { *bdata.offset (i) } };
            i += 1;
          }
      }

    bool_vector_union =>
      {
        while i < nr_words
          {
            unsafe { *destdata.offset (i) = unsafe { *adata.offset (i) } | unsafe { *bdata.offset (i) } };
            i += 1;
          }
      }

    bool_vector_intersection =>
      {
        while i < nr_words
          {
            unsafe { *destdata.offset (i) = unsafe { *adata.offset (i) } & unsafe { *bdata.offset (i) } };
            i += 1;
          }
      }

    bool_vector_set_difference =>
      {
        while i < nr_words
          {
            unsafe { *destdata.offset (i) = unsafe { *adata.offset (i) } & (! unsafe { *bdata.offset (i) }) };
            i += 1;
          }
      }

    _ => { eassume! (false); }
    };

  return dest;
}

DEFUN! ("bool-vector-exclusive-or", Fbool_vector_exclusive_or,
       Sbool_vector_exclusive_or, 2, 3, 0,
       doc: "Return A ^ B, bitwise exclusive or.\nIf optional third argument C is given, store result into C.\nA, B, and C must be bool vectors of the same length.\nReturn the destination vector if it changed or nil otherwise.",
       (a: Lisp_Object, b: Lisp_Object, c: Lisp_Object),
       {
  return bool_vector_binop_driver (a, b, c, bool_vector_exclusive_or);
});

DEFUN! ("bool-vector-union", Fbool_vector_union,
       Sbool_vector_union, 2, 3, 0,
       doc: "Return A | B, bitwise or.\nIf optional third argument C is given, store result into C.\nA, B, and C must be bool vectors of the same length.\nReturn the destination vector if it changed or nil otherwise.",
       (a: Lisp_Object, b: Lisp_Object, c: Lisp_Object),
       {
  return bool_vector_binop_driver (a, b, c, bool_vector_union);
});

DEFUN! ("bool-vector-intersection", Fbool_vector_intersection,
       Sbool_vector_intersection, 2, 3, 0,
       doc: "Return A & B, bitwise and.\nIf optional third argument C is given, store result into C.\nA, B, and C must be bool vectors of the same length.\nReturn the destination vector if it changed or nil otherwise.",
       (a: Lisp_Object, b: Lisp_Object, c: Lisp_Object),
       {
  return bool_vector_binop_driver (a, b, c, bool_vector_intersection);
});

DEFUN! ("bool-vector-set-difference", Fbool_vector_set_difference,
       Sbool_vector_set_difference, 2, 3, 0,
       doc: "Return A &~ B, set difference.\nIf optional third argument C is given, store result into C.\nA, B, and C must be bool vectors of the same length.\nReturn the destination vector if it changed or nil otherwise.",
       (a: Lisp_Object, b: Lisp_Object, c: Lisp_Object),
       {
  return bool_vector_binop_driver (a, b, c, bool_vector_set_difference);
});

DEFUN! ("bool-vector-subsetp", Fbool_vector_subsetp,
       Sbool_vector_subsetp, 2, 2, 0,
       doc: "Return t if every t value in A is also t in B, nil otherwise.\nA and B must be bool vectors of the same length.",
       (a: Lisp_Object, b: Lisp_Object),
       {
  return bool_vector_binop_driver (a, b, b, bool_vector_subsetp);
});

DEFUN! ("bool-vector-not", Fbool_vector_not,
       Sbool_vector_not, 1, 2, 0,
       doc: "Compute ~A, set complement.\nIf optional second argument B is given, store result into B.\nA and B must be bool vectors of the same length.\nReturn the destination vector.",
       (a: Lisp_Object, b: Lisp_Object),
       {
  let mut b: Lisp_Object = b;
  let nr_bits: EMACS_INT;
  let adata: *mut bits_word;
  let bdata: *mut bits_word;
  let mut i: ptrdiff_t;

  CHECK_BOOL_VECTOR (a);
  nr_bits = bool_vector_size (a);

  if NILP (b)
    { b = make_uninit_bool_vector (nr_bits); }
  else
    {
      CHECK_BOOL_VECTOR (b);
      if bool_vector_size (b) != nr_bits
        { wrong_length_argument (a, b, Qnil ()); }
    }

  bdata = bool_vector_data (b);
  adata = bool_vector_data (a);

  i = 0;
  while i < nr_bits / BITS_PER_BITS_WORD as EMACS_INT
    {
      unsafe { *bdata.offset (i) = BITS_WORD_MAX & (! unsafe { *adata.offset (i) }) };
      i += 1;
    }

  if nr_bits % BITS_PER_BITS_WORD as EMACS_INT != 0
    {
      let mut mword: bits_word = bits_word_to_host_endian (unsafe { *adata.offset (i) });
      mword = ! mword;
      mword &= bool_vector_spare_mask (nr_bits);
      unsafe { *bdata.offset (i) = bits_word_to_host_endian (mword) };
    }

  return b;
});

DEFUN! ("bool-vector-count-population", Fbool_vector_count_population,
       Sbool_vector_count_population, 1, 1, 0,
       doc: "Count how many elements in A are t.\nA is a bool vector.  To count A's nil elements, subtract the return\nvalue from A's length.",
       (a: Lisp_Object),
       {
  let mut count: EMACS_INT;
  let nr_bits: EMACS_INT;
  let adata: *mut bits_word;
  let mut i: ptrdiff_t;
  let nwords: ptrdiff_t;

  CHECK_BOOL_VECTOR (a);

  nr_bits = bool_vector_size (a);
  nwords = bool_vector_words (nr_bits);
  count = 0;
  adata = bool_vector_data (a);

  i = 0;
  while i < nwords
    {
      count += stdc_count_ones! (unsafe { *adata.offset (i) }) as EMACS_INT;
      i += 1;
    }

  return make_fixnum (count);
});

DEFUN! ("bool-vector-count-consecutive", Fbool_vector_count_consecutive,
       Sbool_vector_count_consecutive, 3, 3, 0,
       doc: "Count how many consecutive elements in A equal B starting at I.\nA is a bool vector, B is t or nil, and I is an index into A.",
       (a: Lisp_Object, b: Lisp_Object, i: Lisp_Object),
       {
  let mut count: EMACS_INT;
  let nr_bits: EMACS_INT;
  let mut offset: c_int;
  let adata: *mut bits_word;
  let mut twiddle: bits_word;
  let mut mword: bits_word; /* Machine word.  */
  let mut pos: ptrdiff_t;
  let pos0: ptrdiff_t;
  let nr_words: ptrdiff_t;

  CHECK_BOOL_VECTOR (a);
  CHECK_FIXNAT (i);

  nr_bits = bool_vector_size (a);
  if XFIXNAT (i) > nr_bits /* Allow one past the end for convenience */
    { args_out_of_range (a, i); }

  adata = bool_vector_data (a);
  nr_words = bool_vector_words (nr_bits);
  pos = XFIXNAT (i) / BITS_PER_BITS_WORD as EMACS_INT;
  offset = (XFIXNAT (i) % BITS_PER_BITS_WORD as EMACS_INT) as c_int;
  count = 0;

  /* By XORing with twiddle, we transform the problem of "count
     consecutive equal values" into "count the zero bits".  The latter
     operation usually has hardware support.  */
  twiddle = if NILP (b) { 0 } else { BITS_WORD_MAX };

  /* Scan the remainder of the mword at the current offset.  */
  if pos < nr_words && offset != 0
    {
      mword = bits_word_to_host_endian (unsafe { *adata.offset (pos) });
      mword ^= twiddle;
      mword >>= offset;

      /* Do not count the pad bits.  */
      mword |= (1 as bits_word) << ((BITS_PER_BITS_WORD as c_int - offset) as usize);

      count = stdc_trailing_zeros! (mword) as EMACS_INT;
      pos += 1;
      if (count + offset as EMACS_INT) < (BITS_PER_BITS_WORD as EMACS_INT)
        { return make_fixnum (count); }
    }

  /* Scan whole words until we either reach the end of the vector or
     find an mword that doesn't completely match.  twiddle is
     endian-independent.  */
  pos0 = pos;
  while pos < nr_words && unsafe { *adata.offset (pos) } == twiddle
    { pos += 1; }
  count += (pos - pos0) * BITS_PER_BITS_WORD as EMACS_INT;

  if pos < nr_words
    {
      /* If we stopped because of a mismatch, see how many bits match
         in the current mword.  */
      mword = bits_word_to_host_endian (unsafe { *adata.offset (pos) });
      mword ^= twiddle;
      count += stdc_trailing_zeros! (mword) as EMACS_INT;
    }
  else if nr_bits % BITS_PER_BITS_WORD as EMACS_INT != 0
    {
      /* If we hit the end, we might have overshot our count.  Reduce
         the total by the number of spare bits at the end of the
         vector.  */
      count -= BITS_PER_BITS_WORD as EMACS_INT - nr_bits % BITS_PER_BITS_WORD as EMACS_INT;
    }

  return make_fixnum (count);
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_data ()
{
  /* C's `Lisp_Object error_tail, arith_tail, recursion_tail;` are filled
     before every read; the port's zero value is Qnil, C's `= {0,}` (the
     NIL_IS_ZERO convention, PORTING.md 1.3).  */
  let mut error_tail: Lisp_Object = Qnil ();
  let mut arith_tail: Lisp_Object = Qnil ();
  let mut recursion_tail: Lisp_Object = Qnil ();
  DEFSYM! (Qquote, "quote");
  DEFSYM! (Qlambda, "lambda");
  DEFSYM! (Qerror_conditions, "error-conditions");
  DEFSYM! (Qerror_message, "error-message");
  DEFSYM! (Qtop_level, "top-level");
  DEFSYM! (Qerror, "error");
  DEFSYM! (Quser_error, "user-error");
  DEFSYM! (Qquit, "quit");
  DEFSYM! (Qminibuffer_quit, "minibuffer-quit");
  DEFSYM! (Qwrong_length_argument, "wrong-length-argument");
  DEFSYM! (Qwrong_type_argument, "wrong-type-argument");
  DEFSYM! (Qtype_mismatch, "type-mismatch");
  DEFSYM! (Qargs_out_of_range, "args-out-of-range");
  DEFSYM! (Qvoid_function, "void-function");
  DEFSYM! (Qcyclic_function_indirection, "cyclic-function-indirection");
  DEFSYM! (Qcyclic_variable_indirection, "cyclic-variable-indirection");
  DEFSYM! (Qvoid_variable, "void-variable");
  DEFSYM! (Qsetting_constant, "setting-constant");
  DEFSYM! (Qtrapping_constant, "trapping-constant");
  DEFSYM! (Qinvalid_read_syntax, "invalid-read-syntax");
  DEFSYM! (Qinvalid_function, "invalid-function");
  DEFSYM! (Qwrong_number_of_arguments, "wrong-number-of-arguments");
  DEFSYM! (Qmalformed_keyword_arg_list, "malformed-keyword-arg-list");
  DEFSYM! (Qno_catch, "no-catch");
  DEFSYM! (Qend_of_file, "end-of-file");
  DEFSYM! (Qarith_error, "arith-error");
  DEFSYM! (Qbeginning_of_buffer, "beginning-of-buffer");
  DEFSYM! (Qend_of_buffer, "end-of-buffer");
  DEFSYM! (Qbuffer_read_only, "buffer-read-only");
  DEFSYM! (Qtext_read_only, "text-read-only");
  DEFSYM! (Qmark_inactive, "mark-inactive");
  DEFSYM! (Qinhibited_interaction, "inhibited-interaction");
  DEFSYM! (Qrecursion_error, "recursion-error");
  DEFSYM! (Qexcessive_variable_binding, "excessive-variable-binding");
  DEFSYM! (Qexcessive_lisp_nesting, "excessive-lisp-nesting");
  DEFSYM! (Qlistp, "listp");
  DEFSYM! (Qconsp, "consp");
  DEFSYM! (Qbare_symbol_p, "bare-symbol-p");
  DEFSYM! (Qsymbol_with_pos_p, "symbol-with-pos-p");
  DEFSYM! (Qsymbolp, "symbolp");
  DEFSYM! (Qfixnump, "fixnump");
  DEFSYM! (Qintegerp, "integerp");
  DEFSYM! (Qbooleanp, "booleanp");
  DEFSYM! (Qnatnump, "natnump");
  DEFSYM! (Qwholenump, "wholenump");
  DEFSYM! (Qstringp, "stringp");
  DEFSYM! (Qarrayp, "arrayp");
  DEFSYM! (Qsequencep, "sequencep");
  DEFSYM! (Qbufferp, "bufferp");
  DEFSYM! (Qvectorp, "vectorp");
  DEFSYM! (Qrecordp, "recordp");
  DEFSYM! (Qbool_vector_p, "bool-vector-p");
  DEFSYM! (Qchar_or_string_p, "char-or-string-p");
  DEFSYM! (Qmarkerp, "markerp");
  DEFSYM! (Quser_ptrp, "user-ptrp");
  DEFSYM! (Qbuffer_or_string_p, "buffer-or-string-p");
  DEFSYM! (Qinteger_or_marker_p, "integer-or-marker-p");
  DEFSYM! (Qfboundp, "fboundp");
  DEFSYM! (Qfloatp, "floatp");
  DEFSYM! (Qnumberp, "numberp");
  DEFSYM! (Qnumber_or_marker_p, "number-or-marker-p");
  DEFSYM! (Qchar_table_p, "char-table-p");
  DEFSYM! (Qvector_or_char_table_p, "vector-or-char-table-p");
  DEFSYM! (Qfixnum_or_symbol_with_pos_p, "fixnum-or-symbol-with-pos-p");
  DEFSYM! (Qoclosure_interactive_form, "oclosure-interactive-form");
  DEFSYM! (Qsubrp, "subrp");
  DEFSYM! (Qunevalled, "unevalled");
  DEFSYM! (Qmany, "many");
  DEFSYM! (Qcar, "car");
  DEFSYM! (Qcdr, "cdr");
  DEFSYM! (Qnth, "nth");
  DEFSYM! (Qelt, "elt");
  DEFSYM! (Qsetcar, "setcar");
  DEFSYM! (Qsetcdr, "setcdr");
  DEFSYM! (Qaref, "aref");
  DEFSYM! (Qaset, "aset");
  /* ERROR is used as a signaler for random errors for which nothing else is
     right.  */

  error_tail = Fcons (Qerror (), Qnil ());
  Fput (Qerror (), Qerror_conditions (), error_tail);
  Fput (Qerror (), Qerror_message (), build_string (c"error".as_ptr ()));
  /* C's PUT_ERROR (data.c 4120-4122), which C defines here inside the body.  The
     port's macro is `macro_rules!`: the two Fput calls are the expansion, and the
     parameter names stay C's so the body comparison still sees them.  */
  macro_rules! PUT_ERROR {
    ($sym:expr, $tail:expr, $msg:expr) => {
      Fput ($sym, Qerror_conditions (), Fcons ($sym, $tail));
      Fput ($sym, Qerror_message (), build_string ($msg.as_ptr ()));
    };
  }
  PUT_ERROR! (Qquit (), Qnil (), c"Quit");
  PUT_ERROR! (Qminibuffer_quit (), Fcons (Qquit (), Qnil ()), c"Quit");
  PUT_ERROR! (Quser_error (), error_tail, c"");
  PUT_ERROR! (Qwrong_length_argument (), error_tail, c"Wrong length argument");
  PUT_ERROR! (Qwrong_type_argument (), error_tail, c"Wrong type argument");
  PUT_ERROR! (Qtype_mismatch (), error_tail, c"Types do not match");
  PUT_ERROR! (Qargs_out_of_range (), error_tail, c"Args out of range");
  PUT_ERROR! (Qvoid_function (), error_tail, c"Symbol's function definition is void");
  PUT_ERROR! (Qcyclic_function_indirection (), error_tail, c"Symbol's chain of function indirections contains a loop");
  PUT_ERROR! (Qcyclic_variable_indirection (), error_tail, c"Symbol's chain of variable indirections contains a loop");
  DEFSYM! (Qcircular_list, "circular-list");
  PUT_ERROR! (Qcircular_list (), error_tail, c"List contains a loop");
  PUT_ERROR! (Qvoid_variable (), error_tail, c"Symbol's value as variable is void");
  PUT_ERROR! (Qsetting_constant (), error_tail, c"Attempt to set a constant symbol");
  PUT_ERROR! (Qtrapping_constant (), error_tail, c"Attempt to trap writes to a constant symbol");
  PUT_ERROR! (Qinvalid_read_syntax (), error_tail, c"Invalid read syntax");
  PUT_ERROR! (Qinvalid_function (), error_tail, c"Invalid function");
  PUT_ERROR! (Qwrong_number_of_arguments (), error_tail, c"Wrong number of arguments");
  PUT_ERROR! (Qmalformed_keyword_arg_list (), error_tail, c"Keyword lacks a corresponding value");
  PUT_ERROR! (Qno_catch (), error_tail, c"No catch for tag");
  PUT_ERROR! (Qend_of_file (), error_tail, c"End of file during parsing");
  arith_tail = Fcons (Qarith_error (), error_tail);
  Fput (Qarith_error (), Qerror_conditions (), arith_tail);
  Fput (Qarith_error (), Qerror_message (), build_string (c"Arithmetic error".as_ptr ()));
  PUT_ERROR! (Qbeginning_of_buffer (), error_tail, c"Beginning of buffer");
  PUT_ERROR! (Qend_of_buffer (), error_tail, c"End of buffer");
  PUT_ERROR! (Qbuffer_read_only (), error_tail, c"Buffer is read-only");
  PUT_ERROR! (Qtext_read_only (), Fcons (Qbuffer_read_only (), error_tail), c"Text is read-only");
  PUT_ERROR! (Qinhibited_interaction (), error_tail, c"User interaction while inhibited");
  DEFSYM! (Qrange_error, "range-error");
  DEFSYM! (Qdomain_error, "domain-error");
  DEFSYM! (Qsingularity_error, "singularity-error");
  DEFSYM! (Qoverflow_error, "overflow-error");
  DEFSYM! (Qunderflow_error, "underflow-error");
  PUT_ERROR! (Qdomain_error (), arith_tail, c"Arithmetic domain error");
  PUT_ERROR! (Qrange_error (), arith_tail, c"Arithmetic range error");
  PUT_ERROR! (Qsingularity_error (), Fcons (Qdomain_error (), arith_tail), c"Arithmetic singularity error");
  PUT_ERROR! (Qoverflow_error (), Fcons (Qrange_error (), arith_tail), c"Arithmetic overflow error");
  PUT_ERROR! (Qunderflow_error (), Fcons (Qrange_error (), arith_tail), c"Arithmetic underflow error");
  recursion_tail = Fcons (Qrecursion_error (), error_tail);
  Fput (Qrecursion_error (), Qerror_conditions (), recursion_tail);
  Fput (Qrecursion_error (), Qerror_message (), build_string (c"Excessive recursive calling error".as_ptr ()));
  PUT_ERROR! (Qexcessive_lisp_nesting (), recursion_tail, c"Lisp nesting exceeds `max-lisp-eval-depth'");
  /* Error obsolete (from 29.1), kept for compatibility.  */
  /* Error obsolete (from 29.1), kept for compatibility.  */
  PUT_ERROR! (Qexcessive_variable_binding (), recursion_tail, c"Variable binding depth exceeds max-specpdl-size");
  /* Types that type-of returns.  */
  /* Types that type-of returns.  */
  DEFSYM! (Qboolean, "boolean");
  DEFSYM! (Qinteger, "integer");
  DEFSYM! (Qbignum, "bignum");
  DEFSYM! (Qsymbol, "symbol");
  DEFSYM! (Qstring, "string");
  DEFSYM! (Qcons, "cons");
  DEFSYM! (Qmarker, "marker");
  DEFSYM! (Qsymbol_with_pos, "symbol-with-pos");
  DEFSYM! (Qoverlay, "overlay");
  DEFSYM! (Qfinalizer, "finalizer");
  DEFSYM! (Qmodule_function, "module-function");
  DEFSYM! (Qnative_comp_unit, "native-comp-unit");
  DEFSYM! (Quser_ptr, "user-ptr");
  DEFSYM! (Qfloat, "float");
  DEFSYM! (Qwindow_configuration, "window-configuration");
  DEFSYM! (Qprocess, "process");
  DEFSYM! (Qwindow, "window");
  DEFSYM! (Qsubr, "subr");
  DEFSYM! (Qspecial_form, "special-form");
  DEFSYM! (Qprimitive_function, "primitive-function");
  DEFSYM! (Qsubr_native_elisp, "subr-native-elisp"); /* Deprecated name.  */
  DEFSYM! (Qnative_comp_function, "native-comp-function");
  DEFSYM! (Qbyte_code_function, "byte-code-function");
  DEFSYM! (Qinterpreted_function, "interpreted-function");
  DEFSYM! (Qbuffer, "buffer");
  DEFSYM! (Qframe, "frame");
  DEFSYM! (Qvector, "vector");
  DEFSYM! (Qrecord, "record");
  DEFSYM! (Qchar_table, "char-table");
  DEFSYM! (Qsub_char_table, "sub-char-table");
  DEFSYM! (Qbool_vector, "bool-vector");
  DEFSYM! (Qhash_table, "hash-table");
  DEFSYM! (Qthread, "thread");
  DEFSYM! (Qmutex, "mutex");
  DEFSYM! (Qcondition_variable, "condition-variable");
  DEFSYM! (Qfont_spec, "font-spec");
  DEFSYM! (Qfont_entity, "font-entity");
  DEFSYM! (Qfont_object, "font-object");
  DEFSYM! (Qterminal, "terminal");
  DEFSYM! (Qxwidget, "xwidget");
  DEFSYM! (Qxwidget_view, "xwidget-view");
  DEFSYM! (Qtreesit_parser, "treesit-parser");
  DEFSYM! (Qtreesit_node, "treesit-node");
  DEFSYM! (Qtreesit_compiled_query, "treesit-compiled-query");
  DEFSYM! (Qobarray, "obarray");
  DEFSYM! (Qdefun, "defun");
  DEFSYM! (Qinteractive_form, "interactive-form");
  DEFSYM! (Qdefalias_fset_function, "defalias-fset-function");
  DEFSYM! (Qfunction_history, "function-history");
  DEFSYM! (Qbyte_code_function_p, "byte-code-function-p");
  defsubr (&raw mut Sindirect_variable);
  defsubr (&raw mut Sinteractive_form);
  defsubr (&raw mut Scommand_modes);
  defsubr (&raw mut Seq);
  defsubr (&raw mut Snull);
  defsubr (&raw mut Stype_of);
  defsubr (&raw mut Scl_type_of);
  defsubr (&raw mut Slistp);
  defsubr (&raw mut Snlistp);
  defsubr (&raw mut Sconsp);
  defsubr (&raw mut Satom);
  defsubr (&raw mut Sintegerp);
  defsubr (&raw mut Sinteger_or_marker_p);
  defsubr (&raw mut Snumberp);
  defsubr (&raw mut Snumber_or_marker_p);
  defsubr (&raw mut Sfloatp);
  defsubr (&raw mut Snatnump);
  defsubr (&raw mut Sbare_symbol_p);
  defsubr (&raw mut Ssymbol_with_pos_p);
  defsubr (&raw mut Ssymbolp);
  defsubr (&raw mut Skeywordp);
  defsubr (&raw mut Sstringp);
  defsubr (&raw mut Smultibyte_string_p);
  defsubr (&raw mut Svectorp);
  defsubr (&raw mut Srecordp);
  defsubr (&raw mut Schar_table_p);
  defsubr (&raw mut Svector_or_char_table_p);
  defsubr (&raw mut Sbool_vector_p);
  defsubr (&raw mut Sarrayp);
  defsubr (&raw mut Ssequencep);
  defsubr (&raw mut Sbufferp);
  defsubr (&raw mut Smarkerp);
  defsubr (&raw mut Ssubrp);
  defsubr (&raw mut Sbyte_code_function_p);
  defsubr (&raw mut Sinterpreted_function_p);
  defsubr (&raw mut Sclosurep);
  defsubr (&raw mut Smodule_function_p);
  defsubr (&raw mut Schar_or_string_p);
  defsubr (&raw mut Sthreadp);
  defsubr (&raw mut Smutexp);
  defsubr (&raw mut Scondition_variable_p);
  defsubr (&raw mut Scar);
  defsubr (&raw mut Scdr);
  defsubr (&raw mut Scar_safe);
  defsubr (&raw mut Scdr_safe);
  defsubr (&raw mut Ssetcar);
  defsubr (&raw mut Ssetcdr);
  defsubr (&raw mut Ssymbol_function);
  defsubr (&raw mut Sindirect_function);
  defsubr (&raw mut Ssymbol_plist);
  defsubr (&raw mut Ssymbol_name);
  defsubr (&raw mut Sbare_symbol);
  defsubr (&raw mut Ssymbol_with_pos_pos);
  defsubr (&raw mut Sremove_pos_from_symbol);
  defsubr (&raw mut Sposition_symbol);
  defsubr (&raw mut Smakunbound);
  defsubr (&raw mut Sfmakunbound);
  defsubr (&raw mut Sboundp);
  defsubr (&raw mut Sfboundp);
  defsubr (&raw mut Sfset);
  defsubr (&raw mut Sdefalias);
  defsubr (&raw mut Ssetplist);
  defsubr (&raw mut Ssymbol_value);
  defsubr (&raw mut Sset);
  defsubr (&raw mut Sdefault_boundp);
  defsubr (&raw mut Sdefault_value);
  defsubr (&raw mut Sset_default);
  defsubr (&raw mut Smake_variable_buffer_local);
  defsubr (&raw mut Smake_local_variable);
  defsubr (&raw mut Skill_local_variable);
  defsubr (&raw mut Slocal_variable_p);
  defsubr (&raw mut Slocal_variable_if_set_p);
  defsubr (&raw mut Svariable_binding_locus);
  defsubr (&raw mut Saref);
  defsubr (&raw mut Saset);
  defsubr (&raw mut Snumber_to_string);
  defsubr (&raw mut Sstring_to_number);
  defsubr (&raw mut Seqlsign);
  defsubr (&raw mut Slss);
  defsubr (&raw mut Sgtr);
  defsubr (&raw mut Sleq);
  defsubr (&raw mut Sgeq);
  defsubr (&raw mut Sneq);
  defsubr (&raw mut Splus);
  defsubr (&raw mut Sminus);
  defsubr (&raw mut Stimes);
  defsubr (&raw mut Squo);
  defsubr (&raw mut Srem);
  defsubr (&raw mut Smod);
  defsubr (&raw mut Smax);
  defsubr (&raw mut Smin);
  defsubr (&raw mut Slogand);
  defsubr (&raw mut Slogior);
  defsubr (&raw mut Slogxor);
  defsubr (&raw mut Slogcount);
  defsubr (&raw mut Sash);
  defsubr (&raw mut Sadd1);
  defsubr (&raw mut Ssub1);
  defsubr (&raw mut Slognot);
  defsubr (&raw mut Sbyteorder);
  defsubr (&raw mut Ssubr_arity);
  defsubr (&raw mut Ssubr_name);
  defsubr (&raw mut Snative_comp_function_p);
  defsubr (&raw mut Ssubr_native_lambda_list);
  defsubr (&raw mut Ssubr_type);
  defsubr (&raw mut Sbool_vector_exclusive_or);
  defsubr (&raw mut Sbool_vector_union);
  defsubr (&raw mut Sbool_vector_intersection);
  defsubr (&raw mut Sbool_vector_set_difference);
  defsubr (&raw mut Sbool_vector_not);
  defsubr (&raw mut Sbool_vector_subsetp);
  defsubr (&raw mut Sbool_vector_count_consecutive);
  defsubr (&raw mut Sbool_vector_count_population);
  DEFVAR_LISP! (c"most-positive-fixnum", Vmost_positive_fixnum, "The greatest integer that is represented efficiently.\nThis variable cannot be set; trying to do so will signal an error.");
  Vmost_positive_fixnum = make_fixnum (MOST_POSITIVE_FIXNUM);
  make_symbol_constant (intern_c_string (c"most-positive-fixnum".as_ptr ()));
  DEFVAR_LISP! (c"most-negative-fixnum", Vmost_negative_fixnum, "The least integer that is represented efficiently.\nThis variable cannot be set; trying to do so will signal an error.");
  Vmost_negative_fixnum = make_fixnum (MOST_NEGATIVE_FIXNUM);
  make_symbol_constant (intern_c_string (c"most-negative-fixnum".as_ptr ()));
  DEFSYM! (Qsymbols_with_pos_enabled, "symbols-with-pos-enabled");
  DEFVAR_BOOL! (c"symbols-with-pos-enabled", symbols_with_pos_enabled, "If non-nil, a symbol with position ordinarily behaves as its bare symbol.\nBind this to non-nil in applications such as the byte compiler.");
  symbols_with_pos_enabled = false;
  DEFSYM! (Qwatchers, "watchers");
  DEFSYM! (Qmakunbound, "makunbound");
  DEFSYM! (Qunlet, "unlet");
  DEFSYM! (Qset, "set");
  DEFSYM! (Qset_default, "set-default");
  DEFSYM! (Qcommand_modes, "command-modes");
  defsubr (&raw mut Sadd_variable_watcher);
  defsubr (&raw mut Sremove_variable_watcher);
  defsubr (&raw mut Sget_variable_watchers);

}

/* Add to FUNCTION's history a description of its previous definition OLDDEFF.  Called
   by `defalias', and by `fset' and `defvar' via that.  */
unsafe fn add_to_function_history (symbol: Lisp_Object, olddef: Lisp_Object)
{
  eassert! (! NILP (olddef));

  let past: Lisp_Object = Fget (symbol, Qfunction_history ());
  let mut file: Lisp_Object = Qnil ();
  /* FIXME: Sadly, `Vload_file_name` gives less precise information
     (it's sometimes non-nil when it should be nil).  */
  let mut tail: Lisp_Object = Vcurrent_load_list;
  FOR_EACH_TAIL_SAFE! (tail, {
    if NILP (XCDR (tail)) && STRINGP (XCAR (tail))
      { file = XCAR (tail); }
  });

  let tem: Lisp_Object = Fplist_member (past, file, Qequal ());
  if ! NILP (tem)
    { /* New def from a file used before.
         Overwrite the previous record associated with this file.  */
      if EQ (tem, past)
        /* The new def is from the same file as the last change, so
           there's nothing to do: unloading the file should revert to
           the status before the last change rather than before this load.  */
        { return; }
      let pastlen: Lisp_Object = Flength (past);
      let temlen: Lisp_Object = Flength (tem);
      let tempos: EMACS_INT = XFIXNUM (pastlen) - XFIXNUM (temlen);
      eassert! (tempos > 1);
      let prev: Lisp_Object = Fnthcdr (make_fixnum (tempos - 2), past);
      /* Remove the previous info for this file.
         E.g. change `hist` from (... OTHERFILE DEF3 THISFILE DEF2 ...)
         to (... OTHERFILE DEF2). */
      XSETCDR (prev, XCDR (tem));
    }
  /* Push new def from new file.  */
  Fput (symbol, Qfunction_history (), Fcons (file, Fcons (olddef, past)));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn defalias (symbol: Lisp_Object, definition: Lisp_Object)
{
  {
    let autoload: bool = AUTOLOADP (definition);
    if ! will_dump_p () || ! autoload
      { /* Only add autoload entries after dumping, because the ones before are
	   not useful and else we get loads of them from the loaddefs.el.
	   That saves us about 110KB in the pdmp file (Jan 2022).  */
	LOADHIST_ATTACH (Fcons (Qdefun (), symbol));
      }
  }

  {
    let olddef: Lisp_Object = unsafe { (*XSYMBOL (symbol)).u.s.function };
    if ! NILP (olddef)
      {
        if ! NILP (Vautoload_queue)
          { Vautoload_queue = Fcons (symbol, Vautoload_queue); }
        add_to_function_history (symbol, olddef);
      }
  }

  { /* Handle automatic advice activation.  */
    let hook: Lisp_Object = Fget (symbol, Qdefalias_fset_function ());
    if ! NILP (hook)
      { calln! (hook, symbol, definition); }
    else
      { Ffset (symbol, definition); }
  }
}

DEFUN! ("defalias", Fdefalias, Sdefalias, 2, 3, 0,
       doc: "Set SYMBOL's function definition to DEFINITION.\nAssociates the function with the current load file, if any.\nThe optional third argument DOCSTRING specifies the documentation string\nfor SYMBOL; if it is omitted or nil, SYMBOL uses the documentation string\ndetermined by DEFINITION.\n\nInternally, this normally uses `fset', but if SYMBOL has a\n`defalias-fset-function' property, the associated value is used instead.\n\nThe return value is undefined.",
       (symbol: Lisp_Object, definition: Lisp_Object, docstring: Lisp_Object),
       {
  CHECK_SYMBOL (symbol);

  defalias (symbol, definition);

  maybe_defer_native_compilation (symbol, definition);

  if ! NILP (docstring)
    { Fput (symbol, Qfunction_documentation (), docstring); }
  /* We used to return `definition', but now that `defun' and `defmacro' expand
     to a call to `defalias', we return `symbol' for backward compatibility
     (bug#11686).  */
  return symbol;
});
