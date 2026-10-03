// femacs: port of src/textprop.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, enum, enum member, file-scope object, local), same control flow, same error model.
// The twenty `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's non-`static` definitions
// (`validate_interval_range`, `interval_of`, `get_char_property_and_overlay`,
// `set_text_properties`, `set_text_properties_1`, `text_property_stickiness`,
// `copy_text_properties`, `text_property_list`, `add_text_properties_from_list`,
// `extend_property_ranges`, `verify_interval_modification`,
// `report_interval_modification`, `syms_of_textprop`) carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2); the C `static` helpers (`text_read_only`, `modify_text_properties`,
// `CHECK_STRING_OR_BUFFER`, `validate_plist`, `interval_has_all_properties`,
// `interval_has_some_properties`, `interval_has_some_properties_list`, `property_value`,
// `set_properties`, `add_properties`, `remove_properties`, `add_text_properties_1`,
// `call_mod_hooks`) stay plain `unsafe fn`.  The two file-scope objects
// (`interval_insert_behind_hooks`, `interval_insert_in_front_hooks`) are C globals, so they
// are `#[unsafe(no_mangle)] pub static mut` and the `staticpro`s at C 2434-2435 are kept.
//
// **The file has no `#if` at all** (its only preprocessor lines are the four includes and
// the two `#define`s this unit ports as `macro_rules!`), so there is no dead arm to record
// or carry; it is in `docs/baseline/config/cond-eval-verified.txt` with zero conditions.
//
// **The P-70 same-commit mirror moves** (both recorded in `docs/ports/intervals.md` and
// `docs/ports/textprop.md`): (1) `intervals.h`'s "Defined in textprop.c" block moves into
// `pub mod textprop_decls` + glob import in `femacs/src/include/intervals.rs`, because this
// unit defines those names (PORTING.md 0.1-10; the composite.rs/`charset_decls` shape);
// the three prototypes of the "Declared in intervals.c" block this file defines
// (`verify_interval_modification`, `validate_interval_range`, `interval_of`) were already
// in a module + glob import.  (2) `buffer.h`'s two `interval_insert_*_hooks` objects move
// into `pub mod textprop_objs` + glob import in `femacs/src/include/buffer.rs` (P-21).
//
// **C's uninitialised locals are zero-initialised** (PORTING.md 1.5) and named in
// `docs/ports/textprop.md`; C's `INTERVAL` is the mirror's ABI-equal `*mut c_void` (P-72),
// so where C writes `i->field` the port writes `(*((i) as *mut interval)).field`, the
// intervals.rs shape.
//
// Names no mirror carries are declared at their use site below (the insdel.rs/terminal.rs
// shape), each with C's own prototype in a `/// C:` line; they are reported in
// `docs/ports/textprop.md` for the mirror owners.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut)]

include!("include/config.rs");                 // C 19: `#include <config.h>`
include!("include/lisp.rs");                   // C 21: `#include "lisp.h"`
include!("include/intervals.rs");              // C 22: `#include "intervals.h"`
include!("include/character.rs");              // buffer.h 26: `#include "character.h"`
include!("include/itree.rs");                  // buffer.h 28: `#include "itree.h"`
include!("include/buffer.rs");                 // C 23: `#include "buffer.h"`
include!("include/window.rs");                 // C 24: `#include "window.h"`

/* C spells `enum itree_order`'s members bare (`ASCENDING` in the `ITREE_FOREACH`
   expansion), and Rust 2024 keeps enum variants out of scope, so the port imports them
   (PORTING.md 0.1, the itree.rs shape).  */
use itree_order::*;

/* The names C gets from lisp.h whose mirror does not carry them yet (the insdel.rs/
   terminal.rs shape), each with C's own prototype in a `/// C:` line.  */
unsafe extern "C" {
  /// C: lisp.h 4271: `extern Lisp_Object nconc2 (Lisp_Object, Lisp_Object);`
  fn nconc2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4280: `extern Lisp_Object plist_get (Lisp_Object plist, Lisp_Object prop);`
  fn plist_get (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4354: `extern void prepare_to_modify_buffer_1 (ptrdiff_t, ptrdiff_t,
  /// ptrdiff_t *);`
  fn prepare_to_modify_buffer_1 (start: ptrdiff_t, end: ptrdiff_t,
                                 preserve_ptr: *mut ptrdiff_t);
  /// C: lisp.h 5255-5256: `extern void record_property_change (ptrdiff_t, ptrdiff_t,
  /// Lisp_Object, Lisp_Object, Lisp_Object);`
  fn record_property_change (beg: ptrdiff_t, len: ptrdiff_t, prop: Lisp_Object,
                             value: Lisp_Object, buffer: Lisp_Object);
}

// The four `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
// port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
// defines it here (status 3.29, the casefiddle.rs/casetab.rs shape).
#[unsafe(no_mangle)]
pub static mut Vdefault_text_properties: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vchar_property_alias_alist: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vinhibit_point_motion_hooks: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vtext_property_default_nonsticky: Lisp_Object = lisp_h_Qnil ();

/* Test for membership, allowing for t (actually any non-cons) to mean the
   universal set.  */

macro_rules! TMEM {
  ($sym:expr, $set:expr) => {
    (if CONSP ($set) { ! NILP (Fmemq ($sym, $set)) } else { ! NILP ($set) })
  };
}


/* NOTES:  previous- and next- property change will have to skip
  zero-length intervals if they are implemented.  This could be done
  inside next_interval and previous_interval.

  set_properties needs to deal with the interval property cache.

  It is assumed that for any interval plist, a property appears
  only once on the list.  Although some code i.e., remove_properties,
  handles the more general case, the uniqueness of properties is
  necessary for the system to remain consistent.  This requirement
  is enforced by the subrs installing properties onto the intervals.  */



#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum property_set_type
{
  TEXT_PROPERTY_REPLACE,
  TEXT_PROPERTY_PREPEND,
  TEXT_PROPERTY_APPEND
}

/* If o1 is a cons whose cdr is a cons, return true and set o2 to
   the o1's cdr.  Otherwise, return false.  This is handy for
   traversing plists.  */
macro_rules! PLIST_ELT_P {
  ($o1:expr, $o2:expr) => { (CONSP ($o1) && { $o2 = XCDR ($o1); CONSP ($o2) }) };
}

/* verify_interval_modification saves insertion hooks here
   to be run later by report_interval_modification.  */
#[unsafe(no_mangle)]
pub static mut interval_insert_behind_hooks: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut interval_insert_in_front_hooks: Lisp_Object = lisp_h_Qnil ();

/* Signal a `text-read-only' error.  This function makes it easier
   to capture that error in GDB by putting a breakpoint on it.  */

unsafe fn text_read_only (propval: Lisp_Object) -> !
{
  if STRINGP (propval)
    { xsignal1 (Qtext_read_only (), propval); }

  xsignal0 (Qtext_read_only ());
}

/* Prepare to modify the text properties of BUFFER from START to END.  */

unsafe fn modify_text_properties (buffer: Lisp_Object, start: Lisp_Object, end: Lisp_Object)
{
  let b: ptrdiff_t = XFIXNUM (start);
  let e: ptrdiff_t = XFIXNUM (end);
  let buf: *mut buffer = XBUFFER (buffer);
  let old: *mut buffer = current_buffer;

  set_buffer_internal (buf);

  prepare_to_modify_buffer_1 (b, e, std::ptr::null_mut ());

  BUF_COMPUTE_UNCHANGED (buf, b - 1, e);
  if MODIFF! () <= SAVE_MODIFF! ()
    { record_first_change (); }
  modiff_incr (&raw mut MODIFF! (), 1);

  bset_point_before_scroll (current_buffer, Qnil ());

  set_buffer_internal (old);
}

/* Complain if object is not string or buffer type.  */

unsafe fn CHECK_STRING_OR_BUFFER (x: Lisp_Object)
{
  CHECK_TYPE (STRINGP (x) || BUFFERP (x), Qbuffer_or_string_p (), x);
}

/* Extract the interval at the position pointed to by BEGIN from
   OBJECT, a string or buffer.  Additionally, check that the positions
   pointed to by BEGIN and END are within the bounds of OBJECT, and
   reverse them if *BEGIN is greater than *END.  The objects pointed
   to by BEGIN and END may be integers or markers; if the latter, they
   are coerced to integers.

   Note that buffer points don't correspond to interval indices.
   For example, point-max is 1 greater than the index of the last
   character.  This difference is handled in the caller, which uses
   the validated points to determine a length, and operates on that.
   Exceptions are Ftext_properties_at, Fnext_property_change, and
   Fprevious_property_change which call this function with BEGIN == END.
   Handle this case specially.

   If FORCE is soft (false), it's OK to return NULL.  Otherwise,
   create an interval tree for OBJECT if one doesn't exist, provided
   the object actually contains text.  In the current design, if there
   is no text, there can be no text properties.  */

pub const soft: bool = false;
pub const hard: bool = true;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn validate_interval_range (object: Lisp_Object, begin: *mut Lisp_Object,
						  end: *mut Lisp_Object, force: bool)
     -> INTERVAL
{
  let mut i: INTERVAL = std::ptr::null_mut ();
  let mut searchpos: ptrdiff_t = 0;
  let begin0: Lisp_Object = *begin;
  let end0: Lisp_Object = *end;

  CHECK_STRING_OR_BUFFER (object);
  CHECK_FIXNUM_COERCE_MARKER! (*begin);
  CHECK_FIXNUM_COERCE_MARKER! (*end);

  /* If we are asked for a point, but from a subr which operates
     on a range, then return nothing.  */
  if EQ (*begin, *end) && begin != end
    { return std::ptr::null_mut (); }

  if XFIXNUM (*begin) > XFIXNUM (*end)
    {
      let mut n: Lisp_Object = Qnil ();
      n = *begin;
      *begin = *end;
      *end = n;
    }

  if BUFFERP (object)
    {
      let b: *mut buffer = XBUFFER (object);

      if !(BUF_BEGV (b) <= XFIXNUM (*begin) && XFIXNUM (*begin) <= XFIXNUM (*end)
	   && XFIXNUM (*end) <= BUF_ZV (b))
	{ args_out_of_range (begin0, end0); }
      i = buffer_intervals (b);

      /* If there's no text, there are no properties.  */
      if BUF_BEGV (b) == BUF_ZV (b)
	{ return std::ptr::null_mut (); }

      searchpos = XFIXNUM (*begin);
    }
  else
    {
      let len: ptrdiff_t = SCHARS (object);

      if ! (0 <= XFIXNUM (*begin) && XFIXNUM (*begin) <= XFIXNUM (*end)
	    && XFIXNUM (*end) <= len)
	{ args_out_of_range (begin0, end0); }
      i = string_intervals (object);

      if len == 0
	{ return std::ptr::null_mut (); }

      searchpos = XFIXNUM (*begin);
    }

  if i.is_null ()
    { return if force { create_root_interval (object) } else { i }; }

  return find_interval (i, searchpos);
}

/* Validate LIST as a property list.  If LIST is not a list, then
   make one consisting of (LIST nil).  Otherwise, verify that LIST
   is even numbered and thus suitable as a plist.  */

unsafe fn validate_plist (list: Lisp_Object) -> Lisp_Object
{
  if NILP (list)
    { return Qnil (); }

  if CONSP (list)
    {
      let mut tail: Lisp_Object = list;
      loop
	{
	  tail = XCDR (tail);
	  if ! CONSP (tail)
	    { error (c"Odd length text property list".as_ptr ()); }
	  tail = XCDR (tail);
	  maybe_quit ();
	  if ! CONSP (tail)
	    { break; }
	}

      return list;
    }

  return list2 (list, Qnil ());
}

/* Return true if interval I has all the properties,
   with the same values, of list PLIST.  */

unsafe fn interval_has_all_properties (plist: Lisp_Object, i: INTERVAL) -> bool
{
  let mut tail1: Lisp_Object = Qnil ();
  let mut tail2: Lisp_Object = Qnil ();

  /* Go through each element of PLIST.  */
  tail1 = plist;
  while CONSP (tail1)
    {
      let sym1: Lisp_Object = XCAR (tail1);
      let mut found: bool = false;

      /* Go through I's plist, looking for sym1 */
      tail2 = (*((i) as *mut interval)).plist;
      while CONSP (tail2)
	{
	  if EQ (sym1, XCAR (tail2))
	    {
	      /* Found the same property on both lists.  If the
		 values are unequal, return false.  */
	      if ! EQ (Fcar (XCDR (tail1)), Fcar (XCDR (tail2)))
		{ return false; }

	      /* Property has same value on both lists; go to next one.  */
	      found = true;
	      break;
	    }
	  tail2 = Fcdr (XCDR (tail2));
	}

      if ! found
	{ return false; }
      tail1 = Fcdr (XCDR (tail1));
    }

  return true;
}

/* Return true if the plist of interval I has any of the
   properties of PLIST, regardless of their values.  */

unsafe fn interval_has_some_properties (plist: Lisp_Object, i: INTERVAL) -> bool
{
  let mut tail1: Lisp_Object = Qnil ();
  let mut tail2: Lisp_Object = Qnil ();
  let mut sym: Lisp_Object = Qnil ();

  /* Go through each element of PLIST.  */
  tail1 = plist;
  while CONSP (tail1)
    {
      sym = XCAR (tail1);

      /* Go through i's plist, looking for tail1 */
      tail2 = (*((i) as *mut interval)).plist;
      while CONSP (tail2)
	{
	  if EQ (sym, XCAR (tail2))
	    { return true; }
	  tail2 = Fcdr (XCDR (tail2));
	}
      tail1 = Fcdr (XCDR (tail1));
    }

  return false;
}

/* Return true if the plist of interval I has any of the
   property names in LIST, regardless of their values.  */

unsafe fn interval_has_some_properties_list (list: Lisp_Object, i: INTERVAL) -> bool
{
  let mut tail1: Lisp_Object = Qnil ();
  let mut tail2: Lisp_Object = Qnil ();
  let mut sym: Lisp_Object = Qnil ();

  /* Go through each element of LIST.  */
  tail1 = list;
  while CONSP (tail1)
    {
      sym = XCAR (tail1);

      /* Go through i's plist, looking for tail1 */
      tail2 = (*((i) as *mut interval)).plist;
      while CONSP (tail2)
	{
	  if EQ (sym, XCAR (tail2))
	    { return true; }
	  tail2 = XCDR (XCDR (tail2));
	}
      tail1 = XCDR (tail1);
    }

  return false;
}


/* Changing the plists of individual intervals.  */

/* Return the value of PROP in property-list PLIST, or Qunbound if it
   has none.  */
unsafe fn property_value (mut plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object
{
  let mut value: Lisp_Object = Qnil ();

  while PLIST_ELT_P! (plist, value)
    {
      if EQ (XCAR (plist), prop)
	{ return XCAR (value); }
      else
	{ plist = XCDR (value); }
    }

  return Qunbound ();
}

/* Set the properties of INTERVAL to PROPERTIES,
   and record undo info for the previous values.
   OBJECT is the string or buffer that INTERVAL belongs to.  */

unsafe fn set_properties (properties: Lisp_Object, interval: INTERVAL, object: Lisp_Object)
{
  let mut sym: Lisp_Object = Qnil ();
  let mut value: Lisp_Object = Qnil ();

  if BUFFERP (object)
    {
      /* For each property in the old plist which is missing from PROPERTIES,
	 or has a different value in PROPERTIES, make an undo record.  */
      sym = (*((interval) as *mut interval)).plist;
      while PLIST_ELT_P! (sym, value)
	{
	  if ! EQ (property_value (properties, XCAR (sym)),
		   XCAR (value))
	    {
	      record_property_change ((*((interval) as *mut interval)).position,
				      LENGTH! (interval),
				      XCAR (sym), XCAR (value),
				      object);
	    }
	  sym = XCDR (value);
	}

      /* For each new property that has no value at all in the old plist,
	 make an undo record binding it to nil, so it will be removed.  */
      sym = properties;
      while PLIST_ELT_P! (sym, value)
	{
	  if BASE_EQ (property_value ((*((interval) as *mut interval)).plist, XCAR (sym)),
		      Qunbound ())
	    {
	      record_property_change ((*((interval) as *mut interval)).position,
				      LENGTH! (interval),
				      XCAR (sym), Qnil (),
				      object);
	    }
	  sym = XCDR (value);
	}
    }

  /* Store new properties.  */
  set_interval_plist (interval, Fcopy_sequence (properties));
}

/* Add the properties of PLIST to the interval I, or set
   the value of I's property to the value of the property on PLIST
   if they are different.

   OBJECT should be the string or buffer the interval is in.

   If DESTRUCTIVE, the function is allowed to reuse list values in the
   properties.

   Return true if this changes I (i.e., if any members of PLIST
   are actually added to I's plist) */

unsafe fn add_properties (plist: Lisp_Object, i: INTERVAL, object: Lisp_Object,
			  set_type: property_set_type, destructive: bool) -> bool
{
  let mut tail1: Lisp_Object = Qnil ();
  let mut tail2: Lisp_Object = Qnil ();
  let mut sym1: Lisp_Object = Qnil ();
  let mut val1: Lisp_Object = Qnil ();
  let mut changed: bool = false;

  tail1 = plist;
  sym1 = Qnil ();
  val1 = Qnil ();

  /* Go through each element of PLIST.  */
  tail1 = plist;
  while CONSP (tail1)
    {
      let mut found: bool = false;
      sym1 = XCAR (tail1);
      val1 = Fcar (XCDR (tail1));

      /* Go through I's plist, looking for sym1 */
      tail2 = (*((i) as *mut interval)).plist;
      while CONSP (tail2)
	{
	  if EQ (sym1, XCAR (tail2))
	    {
	      let mut this_cdr: Lisp_Object = Qnil ();

	      this_cdr = XCDR (tail2);
	      /* Found the property.  Now check its value.  */
	      found = true;

	      /* The properties have the same value on both lists.
		 Continue to the next property.  */
	      if EQ (val1, Fcar (this_cdr))
		{ break; }

	      /* Record this change in the buffer, for undo purposes.  */
	      if BUFFERP (object)
		{
		  record_property_change ((*((i) as *mut interval)).position, LENGTH! (i),
					  sym1, Fcar (this_cdr), object);
		}

	      /* I's property has a different value -- change it */
	      if set_type == property_set_type::TEXT_PROPERTY_REPLACE
		{ Fsetcar (this_cdr, val1); }
	      else {
		if CONSP (Fcar (this_cdr)) &&
		  /* Special-case anonymous face properties. */
		  (! EQ (sym1, Qface ()) ||
		   NILP (Fkeywordp (Fcar (Fcar (this_cdr)))))
		  /* The previous value is a list, so prepend (or
		     append) the new value to this list. */
		  {
		    if set_type == property_set_type::TEXT_PROPERTY_PREPEND
		      { Fsetcar (this_cdr, Fcons (val1, Fcar (this_cdr))); }
		    else
		      {
			/* Appending. */
			if destructive
			  { nconc2 (Fcar (this_cdr), list1 (val1)); }
			else
			  { Fsetcar (this_cdr, CALLN! (Fappend,
						       Fcar (this_cdr),
						       list1 (val1))); }
		      }
		  }
		else {
		  /* The previous value is a single value, so make it
		     into a list. */
		  if set_type == property_set_type::TEXT_PROPERTY_PREPEND
		    { Fsetcar (this_cdr, list2 (val1, Fcar (this_cdr))); }
		  else
		    { Fsetcar (this_cdr, list2 (Fcar (this_cdr), val1)); }
		}
	      }
	      changed = true;
	      break;
	    }
	  tail2 = Fcdr (XCDR (tail2));
	}

      if ! found
	{
	  /* Record this change in the buffer, for undo purposes.  */
	  if BUFFERP (object)
	    {
	      record_property_change ((*((i) as *mut interval)).position, LENGTH! (i),
				      sym1, Qnil (), object);
	    }
	  set_interval_plist (i, Fcons (sym1, Fcons (val1, (*((i) as *mut interval)).plist)));
	  changed = true;
	}
      tail1 = Fcdr (XCDR (tail1));
    }

  return changed;
}

/* For any members of PLIST, or LIST,
   which are properties of I, remove them from I's plist.
   (If PLIST is non-nil, use that, otherwise use LIST.)
   OBJECT is the string or buffer containing I.  */

unsafe fn remove_properties (plist: Lisp_Object, list: Lisp_Object, i: INTERVAL,
			     object: Lisp_Object) -> bool
{
  let mut changed: bool = false;

  /* True means tail1 is a plist, otherwise it is a list.  */
  let use_plist: bool = ! NILP (plist);
  let mut tail1: Lisp_Object = if use_plist { plist } else { list };

  let mut current_plist: Lisp_Object = (*((i) as *mut interval)).plist;

  /* Go through each element of LIST or PLIST.  */
  while CONSP (tail1)
    {
      let sym: Lisp_Object = XCAR (tail1);

      /* First, remove the symbol if it's at the head of the list */
      while CONSP (current_plist) && EQ (sym, XCAR (current_plist))
	{
	  if BUFFERP (object)
	    {
	      record_property_change ((*((i) as *mut interval)).position, LENGTH! (i),
				      sym, XCAR (XCDR (current_plist)),
				      object);
	    }

	  current_plist = XCDR (XCDR (current_plist));
	  changed = true;
	}

      /* Go through I's plist, looking for SYM.  */
      let mut tail2: Lisp_Object = current_plist;
      while ! NILP (tail2)
	{
	  let this: Lisp_Object = XCDR (XCDR (tail2));
	  if CONSP (this) && EQ (sym, XCAR (this))
	    {
	      if BUFFERP (object)
		{
		  record_property_change ((*((i) as *mut interval)).position, LENGTH! (i),
					  sym, XCAR (XCDR (this)), object);
		}

	      Fsetcdr (XCDR (tail2), XCDR (XCDR (this)));
	      changed = true;
	    }
	  tail2 = this;
	}

      /* Advance thru TAIL1 one way or the other.  */
      tail1 = XCDR (tail1);
      if use_plist && CONSP (tail1)
	{ tail1 = XCDR (tail1); }
    }

  if changed
    { set_interval_plist (i, current_plist); }
  return changed;
}


/* Returns the interval of POSITION in OBJECT.
   POSITION is BEG-based.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn interval_of (position: ptrdiff_t, mut object: Lisp_Object) -> INTERVAL
{
  let mut i: INTERVAL = std::ptr::null_mut ();
  let mut beg: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;

  if NILP (object)
    { XSETBUFFER! (object, current_buffer); }
  else if EQ (object, Qt ())
    { return std::ptr::null_mut (); }

  CHECK_STRING_OR_BUFFER (object);

  if BUFFERP (object)
    {
      let b: *mut buffer = XBUFFER (object);

      beg = BUF_BEGV (b);
      end = BUF_ZV (b);
      i = buffer_intervals (b);
    }
  else
    {
      beg = 0;
      end = SCHARS (object);
      i = string_intervals (object);
    }

  if !(beg <= position && position <= end)
    { args_out_of_range (make_fixnum (position), make_fixnum (position)); }
  if beg == end || i.is_null ()
    { return std::ptr::null_mut (); }

  return find_interval (i, position);
}


DEFUN! ("text-properties-at", Ftext_properties_at,
       Stext_properties_at, 1, 2, 0,
       doc: "Return the list of properties of the character at POSITION in OBJECT.\nIf the optional second argument OBJECT is a buffer (or nil, which means\nthe current buffer), POSITION is a buffer position (integer or marker).\n\nIf OBJECT is a string, POSITION is a 0-based index into it.\n\nIf POSITION is at the end of OBJECT, the value is nil, but note that\nbuffer narrowing does not affect the value.  That is, if OBJECT is a\nbuffer or nil, and the buffer is narrowed and POSITION is at the end\nof the narrowed buffer, the result may be non-nil.\n\nIf you want to display the text properties at point in a human-readable\nform, use the `describe-text-properties' command.",
       (position: Lisp_Object, object: Lisp_Object),
       {
	 let mut position = position;
	 let mut object = object;
	 let mut i: INTERVAL = std::ptr::null_mut ();

	 if NILP (object)
	   { XSETBUFFER! (object, current_buffer); }

	 i = validate_interval_range (object, &raw mut position, &raw mut position, soft);
	 if i.is_null ()
	   { return Qnil (); }
	 /* If POSITION is at the end of the interval,
	    it means it's the end of OBJECT.
	    There are no properties at the very end,
	    since no character follows.  */
	 if XFIXNUM (position) == LENGTH! (i) + (*((i) as *mut interval)).position
	   { return Qnil (); }

	 return (*((i) as *mut interval)).plist;
       });

DEFUN! ("get-text-property", Fget_text_property, Sget_text_property, 2, 3, 0,
       doc: "Return the value of POSITION's property PROP, in OBJECT.\nOBJECT should be a buffer or a string; if omitted or nil, it defaults\nto the current buffer.\n\nIf POSITION is at the end of OBJECT, the value is nil, but note that\nbuffer narrowing does not affect the value.  That is, if the buffer is\nnarrowed and POSITION is at the end of the narrowed buffer, the result\nmay be non-nil.",
       (position: Lisp_Object, prop: Lisp_Object, object: Lisp_Object),
       {
	 return textget (Ftext_properties_at (position, object), prop);
       });

/* Return the value of char's property PROP, in OBJECT at POSITION.
   OBJECT is optional and defaults to the current buffer.
   If OVERLAY is non-0, then in the case that the returned property is from
   an overlay, the overlay found is returned in *OVERLAY, otherwise nil is
   returned in *OVERLAY.
   If POSITION is at the end of OBJECT, the value is nil.
   If OBJECT is a buffer, then overlay properties are considered as well as
   text properties.
   If OBJECT is a window, then that window's buffer is used, but
   window-specific overlays are considered only if they are associated
   with OBJECT. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_char_property_and_overlay (position: Lisp_Object,
							prop: Lisp_Object,
							object: Lisp_Object,
							overlay: *mut Lisp_Object)
     -> Lisp_Object
{
  let mut w: *mut window = std::ptr::null_mut ();

  let pos: EMACS_INT = fix_position (position);
  let mut object = object;

  if NILP (object)
    { XSETBUFFER! (object, current_buffer); }

  if WINDOWP (object)
    {
      CHECK_LIVE_WINDOW (object);
      w = XWINDOW (object);
      object = (*w).contents;
    }
  if BUFFERP (object)
    {
      let b: *mut buffer = XBUFFER (object);
      let mut node: *mut itree_node = std::ptr::null_mut ();
      let mut items: [sortvec; 2] = [std::mem::zeroed (); 2];
      let mut result: *mut sortvec = std::ptr::null_mut ();
      let mut result_tem: Lisp_Object = Qnil ();

      if ! (BUF_BEGV (b) <= pos
	    && pos <= BUF_ZV (b))
	{ xsignal1 (Qargs_out_of_range (), position); }

      /* Now check the overlays in order of decreasing priority.  */
      /* itree.h 166-176's `ITREE_FOREACH (node, b->overlays, pos, pos + 1, ASCENDING)` is
	 expanded here (P-33, status 3.7): C's expansion declares the iterator at the
	 expansion site and drives it with `for (…; ((n = itree_iterator_next
	 (itree_iter_)));)`, so the port writes that loop out.  */
      if (*b).overlays.is_null ()
	{ }
      else
	{
	  let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
	  let itree_iter_: *mut itree_iterator
	    = itree_iterator_start (&raw mut itree_local_iter_,
				    (*b).overlays, pos, pos + 1, ITREE_ASCENDING);
	  while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	    {
	      let tem: Lisp_Object = Foverlay_get ((*node).data, prop);
	      let mut this: *mut sortvec = std::ptr::null_mut ();

	      if NILP (tem) || (*node).end < pos + 1
		  || (!w.is_null () && ! overlay_matches_window (w, (*node).data))
		{ continue; }

	      this = if std::ptr::eq (result, &raw mut items[0])
		       { &raw mut items[1] } else { &raw mut items[0] };
	      make_sortvec_item (this, (*node).data);
	      if result.is_null ()
		  || (compare_overlays (result as *const c_void, this as *const c_void) < 0)
		{
		  result = this;
		  result_tem = tem;
		}
	    }
	}
      if !result.is_null ()
	{
	  if !overlay.is_null ()
	    { *overlay = (*result).overlay; }
	  return result_tem;
	}
    }

  if !overlay.is_null ()
    /* Indicate that the return value is not from an overlay.  */
    { *overlay = Qnil (); }

  /* Not a buffer, or no appropriate overlay, so fall through to the
     simpler case.  */
  return Fget_text_property (make_fixnum (pos), prop, object);
}

DEFUN! ("get-char-property", Fget_char_property, Sget_char_property, 2, 3, 0,
       doc: "Return the value of POSITION's property PROP, in OBJECT.\nBoth overlay properties and text properties are checked.\nOBJECT is optional and defaults to the current buffer.\nIf POSITION is at the end of OBJECT, the value is nil.\nIf OBJECT is a buffer, then overlay properties are considered as well as\ntext properties.\nIf OBJECT is a window, then that window's buffer is used, but window-specific\noverlays are considered only if they are associated with OBJECT.",
       (position: Lisp_Object, prop: Lisp_Object, object: Lisp_Object),
       {
	 return get_char_property_and_overlay (position, prop, object,
					       std::ptr::null_mut ());
       });

DEFUN! ("get-char-property-and-overlay", Fget_char_property_and_overlay,
       Sget_char_property_and_overlay, 2, 3, 0,
       doc: "Like `get-char-property', but with extra overlay information.\nThe value is a cons cell.  Its car is the return value of `get-char-property'\nwith the same arguments--that is, the value of POSITION's property\nPROP in OBJECT.  Its cdr is the overlay in which the property was\nfound, or nil, if it was found as a text property or not found at all.\n\nOBJECT is optional and defaults to the current buffer.  OBJECT may be\na string, a buffer or a window.  For strings, the cdr of the return\nvalue is always nil, since strings do not have overlays.  If OBJECT is\na window, then that window's buffer is used, but window-specific\noverlays are considered only if they are associated with OBJECT.  If\nPOSITION is at the end of OBJECT, both car and cdr are nil.",
       (position: Lisp_Object, prop: Lisp_Object, object: Lisp_Object),
       {
	 let mut overlay: Lisp_Object = Qnil ();
	 let val: Lisp_Object
	   = get_char_property_and_overlay (position, prop, object, &raw mut overlay);
	 return Fcons (val, overlay);
       });


DEFUN! ("next-char-property-change", Fnext_char_property_change,
       Snext_char_property_change, 1, 2, 0,
       doc: "Return the position of next text property or overlay change.\nThis scans characters forward in the current buffer from POSITION till\nit finds a change in some text property, or the beginning or end of an\noverlay, and returns the position of that.\nIf none is found, and LIMIT is nil or omitted, the function\nreturns (point-max).\n\nIf the optional second argument LIMIT is non-nil, the function doesn't\nsearch past position LIMIT, and returns LIMIT if nothing is found\nbefore LIMIT.  LIMIT is a no-op if it is greater than (point-max).",
       (position: Lisp_Object, limit: Lisp_Object),
       {
	 let mut limit = limit;
	 let mut temp: Lisp_Object = Qnil ();

	 temp = Fnext_overlay_change (position);
	 if ! NILP (limit)
	   {
	     CHECK_FIXNUM_COERCE_MARKER! (limit);
	     if XFIXNUM (limit) < XFIXNUM (temp)
	       { temp = limit; }
	   }
	 return Fnext_property_change (position, Qnil (), temp);
       });

DEFUN! ("previous-char-property-change", Fprevious_char_property_change,
       Sprevious_char_property_change, 1, 2, 0,
       doc: "Return the position of previous text property or overlay change.\nScans characters backward in the current buffer from POSITION till it\nfinds a change in some text property, or the beginning or end of an\noverlay, and returns the position of that.\nIf none is found, and LIMIT is nil or omitted, the function\nreturns (point-min).\n\nIf the optional second argument LIMIT is non-nil, the function doesn't\nsearch before position LIMIT, and returns LIMIT if nothing is found\nbefore LIMIT.  LIMIT is a no-op if it is less than (point-min).",
       (position: Lisp_Object, limit: Lisp_Object),
       {
	 let mut limit = limit;
	 let mut temp: Lisp_Object = Qnil ();

	 temp = Fprevious_overlay_change (position);
	 if ! NILP (limit)
	   {
	     CHECK_FIXNUM_COERCE_MARKER! (limit);
	     if XFIXNUM (limit) > XFIXNUM (temp)
	       { temp = limit; }
	   }
	 return Fprevious_property_change (position, Qnil (), temp);
       });


DEFUN! ("next-single-char-property-change", Fnext_single_char_property_change,
       Snext_single_char_property_change, 2, 4, 0,
       doc: "Return the position of next text property or overlay change for a specific property.\nScans characters forward from POSITION till it finds\na change in the PROP property, then returns the position of the change.\nIf the optional third argument OBJECT is a buffer (or nil, which means\nthe current buffer), POSITION is a buffer position (integer or marker).\nIf OBJECT is a string, POSITION is a 0-based index into it.\n\nIn a string, scan runs to the end of the string, unless LIMIT is non-nil.\nIn a buffer, scan runs to end of buffer, unless LIMIT is non-nil.\nIf the optional fourth argument LIMIT is non-nil, don't search\npast position LIMIT; return LIMIT if nothing is found before LIMIT.\nHowever, if OBJECT is a buffer and LIMIT is beyond the end of the\nbuffer, this function returns `point-max', not LIMIT.\n\nThe property values are compared with `eq'.",
       (position: Lisp_Object, prop: Lisp_Object, object: Lisp_Object, limit: Lisp_Object),
       {
	 let mut position = position;
	 let mut limit = limit;

	 if STRINGP (object)
	   {
	     position = Fnext_single_property_change (position, prop, object, limit);
	     if NILP (position)
	       {
		 if NILP (limit)
		   { position = make_fixnum (SCHARS (object)); }
		 else
		   {
		     CHECK_FIXNUM (limit);
		     position = limit;
		   }
	       }
	   }
	 else
	   {
	     let mut initial_value: Lisp_Object = Qnil ();
	     let mut value: Lisp_Object = Qnil ();
	     let count: specpdl_ref = SPECPDL_INDEX ();

	     if ! NILP (object)
	       { CHECK_BUFFER (object); }

	     if BUFFERP (object) && current_buffer != XBUFFER (object)
	       {
		 record_unwind_current_buffer ();
		 Fset_buffer (object);
	       }

	     CHECK_FIXNUM_COERCE_MARKER! (position);

	     initial_value = Fget_char_property (position, prop, object);

	     if NILP (limit)
	       { XSETFASTINT! (limit, ZV! ()); }
	     else
	       { CHECK_FIXNUM_COERCE_MARKER! (limit); }

	     if XFIXNUM (position) >= XFIXNUM (limit)
	       {
		 position = limit;
		 if XFIXNUM (position) > ZV! ()
		   { XSETFASTINT! (position, ZV! ()); }
	       }
	     else
	       {
		 while true
		   {
		     position = Fnext_char_property_change (position, limit);
		     if XFIXNAT (position) >= XFIXNAT (limit)
		       {
			 position = limit;
			 break;
		       }

		     value = Fget_char_property (position, prop, object);
		     if !EQ (value, initial_value)
		       { break; }

		     if XFIXNAT (position) >= ZV! ()
		       { break; }
		   }
	       }

	     position = unbind_to (count, position);
	   }

	 return position;
       });

DEFUN! ("previous-single-char-property-change",
       Fprevious_single_char_property_change,
       Sprevious_single_char_property_change, 2, 4, 0,
       doc: "Return the position of previous text property or overlay change for a specific property.\nScans characters backward from POSITION till it finds\na change in the PROP property, then returns the position of the change.\nIf the optional third argument OBJECT is a buffer (or nil, which means\nthe current buffer), POSITION is a buffer position (integer or marker).\nIf OBJECT is a string, POSITION is a 0-based index into it.\n\nIn a string, scan runs to the start of the string, unless LIMIT is non-nil.\nIn a buffer, if LIMIT is nil or omitted, it runs to (point-min), and the\nvalue cannot be less than that.\nIf the optional fourth argument LIMIT is non-nil, don't search back past\nposition LIMIT; return LIMIT if nothing is found before reaching LIMIT.\n\nThe property values are compared with `eq'.\nIf the property is constant all the way to the start of OBJECT, return the\nfirst valid position in OBJECT.",
       (position: Lisp_Object, prop: Lisp_Object, object: Lisp_Object, limit: Lisp_Object),
       {
	 let mut position = position;
	 let mut limit = limit;

	 if STRINGP (object)
	   {
	     position = Fprevious_single_property_change (position, prop, object, limit);
	     if NILP (position)
	       {
		 if NILP (limit)
		   { position = make_fixnum (0); }
		 else
		   {
		     CHECK_FIXNUM (limit);
		     position = limit;
		   }
	       }
	   }
	 else
	   {
	     let count: specpdl_ref = SPECPDL_INDEX ();

	     if ! NILP (object)
	       { CHECK_BUFFER (object); }

	     if BUFFERP (object) && current_buffer != XBUFFER (object)
	       {
		 record_unwind_current_buffer ();
		 Fset_buffer (object);
	       }

	     CHECK_FIXNUM_COERCE_MARKER! (position);

	     if NILP (limit)
	       { XSETFASTINT! (limit, BEGV! ()); }
	     else
	       { CHECK_FIXNUM_COERCE_MARKER! (limit); }

	     if XFIXNUM (position) <= XFIXNUM (limit)
	       {
		 position = limit;
		 if XFIXNUM (position) < BEGV! ()
		   { XSETFASTINT! (position, BEGV! ()); }
	       }
	     else
	       {
		 let initial_value: Lisp_Object
		   = Fget_char_property (make_fixnum (XFIXNUM (position)
						      - (0 <= XFIXNUM (position)) as ptrdiff_t),
					 prop, object);

		 while true
		   {
		     position = Fprevious_char_property_change (position, limit);

		     if XFIXNAT (position) <= XFIXNAT (limit)
		       {
			 position = limit;
			 break;
		       }
		     else
		       {
			 let value: Lisp_Object
			   = Fget_char_property (make_fixnum (XFIXNAT (position) - 1),
						 prop, object);

			 if !EQ (value, initial_value)
			   { break; }
		       }
		   }
	       }

	     position = unbind_to (count, position);
	   }

	 return position;
       });

DEFUN! ("next-property-change", Fnext_property_change,
       Snext_property_change, 1, 3, 0,
       doc: "Return the position of next property change.\nScans characters forward from POSITION in OBJECT till it finds\na change in some text property, then returns the position of the change.\nIf the optional second argument OBJECT is a buffer (or nil, which means\nthe current buffer), POSITION is a buffer position (integer or marker).\nIf OBJECT is a string, POSITION is a 0-based index into it.\nReturn nil if LIMIT is nil or omitted, and the property is constant all\nthe way to the end of OBJECT; if the value is non-nil, it is a position\ngreater than POSITION, never equal.\n\nIf the optional third argument LIMIT is non-nil, don't search\npast position LIMIT; return LIMIT if nothing is found before LIMIT.",
       (position: Lisp_Object, object: Lisp_Object, limit: Lisp_Object),
       {
	 let mut position = position;
	 let mut object = object;
	 let mut limit = limit;
	 let mut i: INTERVAL = std::ptr::null_mut ();
	 let mut next: INTERVAL = std::ptr::null_mut ();

	 if NILP (object)
	   { XSETBUFFER! (object, current_buffer); }

	 if !NILP (limit) && !EQ (limit, Qt ())
	   { CHECK_FIXNUM_COERCE_MARKER! (limit); }

	 i = validate_interval_range (object, &raw mut position, &raw mut position, soft);

	 /* If LIMIT is t, return start of next interval--don't
	    bother checking further intervals.  */
	 if EQ (limit, Qt ())
	   {
	     if i.is_null ()
	       { next = i; }
	     else
	       { next = next_interval (i); }

	     if next.is_null ()
	       { XSETFASTINT! (position, (if STRINGP (object)
					  { SCHARS (object) }
					  else { BUF_ZV (XBUFFER (object)) })); }
	     else
	       { XSETFASTINT! (position, (*((next) as *mut interval)).position); }
	     return position;
	   }

	 if i.is_null ()
	   { return limit; }

	 next = next_interval (i);

	 while !next.is_null () && intervals_equal (i, next)
	       && (NILP (limit) || (*((next) as *mut interval)).position < XFIXNUM (limit))
	   { next = next_interval (next); }

	 if next.is_null ()
	     || ((*((next) as *mut interval)).position
		 >= (if FIXNUMP (limit)
		       { XFIXNUM (limit) }
		       else { if STRINGP (object)
				{ SCHARS (object) }
				else { BUF_ZV (XBUFFER (object)) } }))
	   { return limit; }
	 else
	   { return make_fixnum ((*((next) as *mut interval)).position); }
       });

DEFUN! ("next-single-property-change", Fnext_single_property_change,
       Snext_single_property_change, 2, 4, 0,
       doc: "Return the position of next property change for a specific property.\nScans characters forward from POSITION till it finds\na change in the PROP property, then returns the position of the change.\nIf the optional third argument OBJECT is a buffer (or nil, which means\nthe current buffer), POSITION is a buffer position (integer or marker).\nIf OBJECT is a string, POSITION is a 0-based index into it.\nThe property values are compared with `eq'.\nReturn nil if LIMIT is nil or omitted, and the property is constant all\nthe way to the end of OBJECT; if the value is non-nil, it is a position\ngreater than POSITION, never equal.\n\nIf the optional fourth argument LIMIT is non-nil, don't search\npast position LIMIT; return LIMIT if nothing is found before LIMIT.",
       (position: Lisp_Object, prop: Lisp_Object, object: Lisp_Object, limit: Lisp_Object),
       {
	 let mut position = position;
	 let mut object = object;
	 let mut limit = limit;
	 let mut i: INTERVAL = std::ptr::null_mut ();
	 let mut next: INTERVAL = std::ptr::null_mut ();
	 let mut here_val: Lisp_Object = Qnil ();

	 if NILP (object)
	   { XSETBUFFER! (object, current_buffer); }

	 if !NILP (limit)
	   { CHECK_FIXNUM_COERCE_MARKER! (limit); }

	 i = validate_interval_range (object, &raw mut position, &raw mut position, soft);
	 if i.is_null ()
	   { return limit; }

	 here_val = textget ((*((i) as *mut interval)).plist, prop);
	 next = next_interval (i);
	 while !next.is_null ()
	       && EQ (here_val, textget ((*((next) as *mut interval)).plist, prop))
	       && (NILP (limit) || (*((next) as *mut interval)).position < XFIXNUM (limit))
	   { next = next_interval (next); }

	 if next.is_null ()
	     || ((*((next) as *mut interval)).position
		 >= (if FIXNUMP (limit)
		       { XFIXNUM (limit) }
		       else { if STRINGP (object)
				{ SCHARS (object) }
				else { BUF_ZV (XBUFFER (object)) } }))
	   { return limit; }
	 else
	   { return make_fixnum ((*((next) as *mut interval)).position); }
       });

DEFUN! ("previous-property-change", Fprevious_property_change,
       Sprevious_property_change, 1, 3, 0,
       doc: "Return the position of previous property change.\nScans characters backwards from POSITION in OBJECT till it finds\na change in some text property, then returns the position of the change.\nIf the optional second argument OBJECT is a buffer (or nil, which means\nthe current buffer), POSITION is a buffer position (integer or marker).\nIf OBJECT is a string, POSITION is a 0-based index into it.\nReturn nil if LIMIT is nil or omitted, and the property is constant all\nthe way to the start of OBJECT; if the value is non-nil, it is a position\nless than POSITION, never equal.\n\nIf the optional third argument LIMIT is non-nil, don't search\nback past position LIMIT; return LIMIT if nothing is found until LIMIT.",
       (position: Lisp_Object, object: Lisp_Object, limit: Lisp_Object),
       {
	 let mut position = position;
	 let mut object = object;
	 let mut limit = limit;
	 let mut i: INTERVAL = std::ptr::null_mut ();
	 let mut previous: INTERVAL = std::ptr::null_mut ();

	 if NILP (object)
	   { XSETBUFFER! (object, current_buffer); }

	 if !NILP (limit)
	   { CHECK_FIXNUM_COERCE_MARKER! (limit); }

	 i = validate_interval_range (object, &raw mut position, &raw mut position, soft);
	 if i.is_null ()
	   { return limit; }

	 /* Start with the interval containing the char before point.  */
	 if (*((i) as *mut interval)).position == XFIXNAT (position)
	   { i = previous_interval (i); }

	 previous = previous_interval (i);
	 while !previous.is_null () && intervals_equal (previous, i)
	       && (NILP (limit)
		   || ((*((previous) as *mut interval)).position
		       + LENGTH! (previous) > XFIXNUM (limit)))
	   { previous = previous_interval (previous); }

	 if previous.is_null ()
	     || ((*((previous) as *mut interval)).position + LENGTH! (previous)
		 <= (if FIXNUMP (limit)
		       { XFIXNUM (limit) }
		       else { if STRINGP (object) { 0 } else { BUF_BEGV (XBUFFER (object)) } }))
	   { return limit; }
	 else
	   { return make_fixnum ((*((previous) as *mut interval)).position
				 + LENGTH! (previous)); }
       });

DEFUN! ("previous-single-property-change", Fprevious_single_property_change,
       Sprevious_single_property_change, 2, 4, 0,
       doc: "Return the position of previous property change for a specific property.\nScans characters backward from POSITION till it finds\na change in the PROP property, then returns the position of the change.\nIf the optional third argument OBJECT is a buffer (or nil, which means\nthe current buffer), POSITION is a buffer position (integer or marker).\nIf OBJECT is a string, POSITION is a 0-based index into it.\nThe property values are compared with `eq'.\nReturn nil if LIMIT is nil or omitted, and the property is constant all\nthe way to the start of OBJECT; if the value is non-nil, it is a position\nless than POSITION, never equal.\n\nIf the optional fourth argument LIMIT is non-nil, don't search\nback past position LIMIT; return LIMIT if nothing is found until LIMIT.",
       (position: Lisp_Object, prop: Lisp_Object, object: Lisp_Object, limit: Lisp_Object),
       {
	 let mut position = position;
	 let mut object = object;
	 let mut limit = limit;
	 let mut i: INTERVAL = std::ptr::null_mut ();
	 let mut previous: INTERVAL = std::ptr::null_mut ();
	 let mut here_val: Lisp_Object = Qnil ();

	 if NILP (object)
	   { XSETBUFFER! (object, current_buffer); }

	 if !NILP (limit)
	   { CHECK_FIXNUM_COERCE_MARKER! (limit); }

	 i = validate_interval_range (object, &raw mut position, &raw mut position, soft);

	 /* Start with the interval containing the char before point.  */
	 if !i.is_null () && (*((i) as *mut interval)).position == XFIXNAT (position)
	   { i = previous_interval (i); }

	 if i.is_null ()
	   { return limit; }

	 here_val = textget ((*((i) as *mut interval)).plist, prop);
	 previous = previous_interval (i);
	 while !previous.is_null ()
	       && EQ (here_val, textget ((*((previous) as *mut interval)).plist, prop))
	       && (NILP (limit)
		   || ((*((previous) as *mut interval)).position
		       + LENGTH! (previous) > XFIXNUM (limit)))
	   { previous = previous_interval (previous); }

	 if previous.is_null ()
	     || ((*((previous) as *mut interval)).position + LENGTH! (previous)
		 <= (if FIXNUMP (limit)
		       { XFIXNUM (limit) }
		       else { if STRINGP (object) { 0 } else { BUF_BEGV (XBUFFER (object)) } }))
	   { return limit; }
	 else
	   { return make_fixnum ((*((previous) as *mut interval)).position
				 + LENGTH! (previous)); }
       });


/* Used by add-text-properties and add-face-text-property. */

unsafe fn add_text_properties_1 (start: Lisp_Object, end: Lisp_Object,
				 properties: Lisp_Object, object: Lisp_Object,
				 set_type: property_set_type,
				 destructive: bool) -> Lisp_Object
{
  // C rebinds its parameters (`properties = validate_plist (…)`, `XSETBUFFER (object,
  // current_buffer)`) and hands `&start`/`&end` to validate_interval_range; the port keeps
  // the same names in mutable bindings (the composite.rs `let mut start = start` shape).
  let mut start = start;
  let mut end = end;
  let mut properties = properties;
  let mut object = object;

  /* Ensure we run the modification hooks for the right buffer,
     without switching buffers twice (bug 36190).  FIXME: Switching
     buffers is slow and often unnecessary.  */
  if BUFFERP (object) && XBUFFER (object) != current_buffer
    {
      let count: specpdl_ref = SPECPDL_INDEX ();
      record_unwind_current_buffer ();
      set_buffer_internal (XBUFFER (object));
      return unbind_to (count, add_text_properties_1 (start, end, properties,
						      object, set_type,
						      destructive));
    }

  let mut i: INTERVAL = std::ptr::null_mut ();
  let mut unchanged: INTERVAL = std::ptr::null_mut ();
  let mut s: ptrdiff_t = 0;
  let mut len: ptrdiff_t = 0;
  let mut modified: bool = false;
  let mut first_time: bool = true;

  properties = validate_plist (properties);
  if NILP (properties)
    { return Qnil (); }

  if NILP (object)
    { XSETBUFFER! (object, current_buffer); }

 'retry: loop {
  i = validate_interval_range (object, &raw mut start, &raw mut end, hard);
  if i.is_null ()
    { return Qnil (); }

  s = XFIXNUM (start);
  len = XFIXNUM (end) - s;

  /* If this interval already has the properties, we can skip it.  */
  if interval_has_all_properties (properties, i)
    {
      let mut got: ptrdiff_t = LENGTH! (i) - (s - (*((i) as *mut interval)).position);

      loop
	{
	  if got >= len
	    { return Qnil (); }
	  len -= got;
	  i = next_interval (i);
	  got = LENGTH! (i);
	  if ! interval_has_all_properties (properties, i)
	    { break; }
	}
    }
  else if (*((i) as *mut interval)).position != s
    {
      /* If we're not starting on an interval boundary, we have to
	 split this interval.  */
      unchanged = i;
      i = split_interval_right (unchanged,
				s - (*((unchanged) as *mut interval)).position);
      copy_properties (unchanged, i);
    }

  if BUFFERP (object) && first_time
    {
      let prev_total_length: ptrdiff_t = TOTAL_LENGTH! (i);
      let prev_pos: ptrdiff_t = (*((i) as *mut interval)).position;

      modify_text_properties (object, start, end);
      /* If someone called us recursively as a side effect of
	 modify_text_properties, and changed the intervals behind our back
	 (could happen if lock_file, called by prepare_to_modify_buffer,
	 triggers redisplay, and that calls add-text-properties again
	 in the same buffer), we cannot continue with I, because its
	 data changed.  So we restart the interval analysis anew.  */
      if TOTAL_LENGTH! (i) != prev_total_length
	  || (*((i) as *mut interval)).position != prev_pos
	{
	  first_time = false;
	  continue 'retry;
	}
    }

  /* We are at the beginning of interval I, with LEN chars to scan.  */
  loop
    {
      eassert! (!i.is_null ());

      if LENGTH! (i) >= len
	{
	  if interval_has_all_properties (properties, i)
	    {
	      if BUFFERP (object)
		{ signal_after_change (XFIXNUM (start), XFIXNUM (end) - XFIXNUM (start),
				       XFIXNUM (end) - XFIXNUM (start)); }

	      eassert! (modified);
	      return Qt ();
	    }

	  if LENGTH! (i) == len
	    {
	      add_properties (properties, i, object, set_type, destructive);
	      if BUFFERP (object)
		{ signal_after_change (XFIXNUM (start), XFIXNUM (end) - XFIXNUM (start),
				       XFIXNUM (end) - XFIXNUM (start)); }
	      return Qt ();
	    }

	  /* i doesn't have the properties, and goes past the change limit */
	  unchanged = i;
	  i = split_interval_left (unchanged, len);
	  copy_properties (unchanged, i);
	  add_properties (properties, i, object, set_type, destructive);
	  if BUFFERP (object)
	    { signal_after_change (XFIXNUM (start), XFIXNUM (end) - XFIXNUM (start),
				   XFIXNUM (end) - XFIXNUM (start)); }
	  return Qt ();
	}

      len -= LENGTH! (i);
      modified |= add_properties (properties, i, object, set_type, destructive);
      i = next_interval (i);
    }
 }
}

/* Callers note, this can GC when OBJECT is a buffer (or nil).  */

DEFUN! ("add-text-properties", Fadd_text_properties,
       Sadd_text_properties, 3, 4, 0,
       doc: "Add properties to the text from START to END.\nThe third argument PROPERTIES is a property list\nspecifying the property values to add.  If the optional fourth argument\nOBJECT is a buffer (or nil, which means the current buffer),\nSTART and END are buffer positions (integers or markers).\nIf OBJECT is a string, START and END are 0-based indices into it.\nReturn t if any property value actually changed, nil otherwise.",
       (start: Lisp_Object, end: Lisp_Object, properties: Lisp_Object,
	object: Lisp_Object),
       {
	 return add_text_properties_1 (start, end, properties, object,
				       property_set_type::TEXT_PROPERTY_REPLACE, true);
       });

/* Callers note, this can GC when OBJECT is a buffer (or nil).  */

DEFUN! ("put-text-property", Fput_text_property,
       Sput_text_property, 4, 5, 0,
       doc: "Set one property of the text from START to END.\nThe third and fourth arguments PROPERTY and VALUE\nspecify the property to add.\nIf the optional fifth argument OBJECT is a buffer (or nil, which means\nthe current buffer), START and END are buffer positions (integers or\nmarkers).  If OBJECT is a string, START and END are 0-based indices into it.",
       (start: Lisp_Object, end: Lisp_Object, property: Lisp_Object,
	value: Lisp_Object, object: Lisp_Object),
       {
	 AUTO_LIST2! (properties, property, value);
	 Fadd_text_properties (start, end, properties, object);
	 return Qnil ();
       });

DEFUN! ("set-text-properties", Fset_text_properties,
       Sset_text_properties, 3, 4, 0,
       doc: "Completely replace properties of text from START to END.\nThe third argument PROPERTIES is the new property list.\nIf the optional fourth argument OBJECT is a buffer (or nil, which means\nthe current buffer), START and END are buffer positions (integers or\nmarkers).  If OBJECT is a string, START and END are 0-based indices into it.\nIf PROPERTIES is nil, the effect is to remove all properties from\nthe designated part of OBJECT.",
       (start: Lisp_Object, end: Lisp_Object, properties: Lisp_Object, object: Lisp_Object),
       {
	 return set_text_properties (start, end, properties, object, Qt ());
       });


DEFUN! ("add-face-text-property", Fadd_face_text_property,
       Sadd_face_text_property, 3, 5, 0,
       doc: "Add the face property to the text from START to END.\nFACE specifies the face to add.  It should be a valid value of the\n`face' property (typically a face name or a plist of face attributes\nand values).\n\nIf any text in the region already has a non-nil `face' property, those\nface(s) are retained.  This is done by setting the `face' property to\na list of faces, with FACE as the first element (by default) and the\npre-existing faces as the remaining elements.\n\nIf optional fourth argument APPEND is non-nil, append FACE to the end\nof the face list instead.\n\nIf optional fifth argument OBJECT is a buffer (or nil, which means the\ncurrent buffer), START and END are buffer positions (integers or\nmarkers).  If OBJECT is a string, START and END are 0-based indices\ninto it.",
       (start: Lisp_Object, end: Lisp_Object, face: Lisp_Object,
	append: Lisp_Object, object: Lisp_Object),
       {
	 AUTO_LIST2! (properties, Qface (), face);
	 add_text_properties_1 (start, end, properties, object,
				(if NILP (append)
				   { property_set_type::TEXT_PROPERTY_PREPEND }
				   else { property_set_type::TEXT_PROPERTY_APPEND }),
				false);
	 return Qnil ();
       });

/* Replace properties of text from START to END with new list of
   properties PROPERTIES.  OBJECT is the buffer or string containing
   the text.  OBJECT nil means use the current buffer.
   COHERENT_CHANGE_P nil means this is being called as an internal
   subroutine, rather than as a change primitive with checking of
   read-only, invoking change hooks, etc..  Value is nil if the
   function _detected_ that it did not replace any properties, non-nil
   otherwise.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_text_properties (start: Lisp_Object, end: Lisp_Object,
					      properties: Lisp_Object,
					      object: Lisp_Object,
					      coherent_change_p: Lisp_Object) -> Lisp_Object
{
  /* Ensure we run the modification hooks for the right buffer,
     without switching buffers twice (bug 36190).  FIXME: Switching
     buffers is slow and often unnecessary.  */
  if BUFFERP (object) && XBUFFER (object) != current_buffer
    {
      let count: specpdl_ref = SPECPDL_INDEX ();
      record_unwind_current_buffer ();
      set_buffer_internal (XBUFFER (object));
      return unbind_to (count,
			set_text_properties (start, end, properties,
					     object, coherent_change_p));
    }

  let mut start = start;
  let mut end = end;
  let mut properties = properties;
  let mut object = object;
  let mut i: INTERVAL = std::ptr::null_mut ();
  let mut first_time: bool = true;

  properties = validate_plist (properties);

  if NILP (object)
    { XSETBUFFER! (object, current_buffer); }

  /* If we want no properties for a whole string,
     get rid of its intervals.  */
  if NILP (properties) && STRINGP (object)
      && BASE_EQ (start, make_fixnum (0))
      && BASE_EQ (end, make_fixnum (SCHARS (object)))
    {
      if string_intervals (object).is_null ()
	{ return Qnil (); }

      set_string_intervals (object, std::ptr::null_mut ());
      return Qt ();
    }

 'retry: loop {
  i = validate_interval_range (object, &raw mut start, &raw mut end, soft);

  if i.is_null ()
    {
      /* If buffer has no properties, and we want none, return now.  */
      if NILP (properties)
	{ return Qnil (); }

      i = validate_interval_range (object, &raw mut start, &raw mut end, hard);
      /* This can return if start == end.  */
      if i.is_null ()
	{ return Qnil (); }
    }

  if BUFFERP (object) && !NILP (coherent_change_p) && first_time
    {
      let prev_length: ptrdiff_t = LENGTH! (i);
      let prev_pos: ptrdiff_t = (*((i) as *mut interval)).position;

      modify_text_properties (object, start, end);
      /* If someone called us recursively as a side effect of
	 modify_text_properties, and changed the intervals behind our
	 back, we cannot continue with I, because its data changed.
	 So we restart the interval analysis anew.  */
      if LENGTH! (i) != prev_length || (*((i) as *mut interval)).position != prev_pos
	{
	  first_time = false;
	  continue 'retry;
	}
    }

  set_text_properties_1 (start, end, properties, object, i);

  if BUFFERP (object) && !NILP (coherent_change_p)
    { signal_after_change (XFIXNUM (start), XFIXNUM (end) - XFIXNUM (start),
			   XFIXNUM (end) - XFIXNUM (start)); }
  return Qt ();
 }
}

/* Replace properties of text from START to END with new list of
   properties PROPERTIES.  OBJECT is the buffer or string containing
   the text.  This does not obey any hooks.
   I is the interval that START is located in.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_text_properties_1 (start: Lisp_Object, end: Lisp_Object,
						properties: Lisp_Object, object: Lisp_Object,
						i: INTERVAL)
{
  /* Ensure we run the modification hooks for the right buffer,
     without switching buffers twice (bug 36190).  FIXME: Switching
     buffers is slow and often unnecessary.  */
  if BUFFERP (object) && XBUFFER (object) != current_buffer
    {
      let count: specpdl_ref = SPECPDL_INDEX ();
      record_unwind_current_buffer ();
      set_buffer_internal (XBUFFER (object));

      set_text_properties_1 (start, end, properties, object, i);
      unbind_to (count, Qnil ());
      return;
    }

  let mut prev_changed: INTERVAL = std::ptr::null_mut ();
  let s: ptrdiff_t = XFIXNUM (start);
  let mut len: ptrdiff_t = XFIXNUM (end) - s;
  let mut i = i;

  if len == 0
    { return; }
  eassert! (0 < len);

  eassert! (!i.is_null ());

  if (*((i) as *mut interval)).position != s
    {
      let unchanged: INTERVAL = i;
      i = split_interval_right (unchanged, s - (*((unchanged) as *mut interval)).position);

      if LENGTH! (i) > len
	{
	  copy_properties (unchanged, i);
	  i = split_interval_left (i, len);
	  set_properties (properties, i, object);
	  return;
	}

      set_properties (properties, i, object);

      if LENGTH! (i) == len
	{ return; }

      prev_changed = i;
      len -= LENGTH! (i);
      i = next_interval (i);
    }

  /* We are starting at the beginning of an interval I.  LEN is positive.  */
  loop
    {
      eassert! (!i.is_null ());

      if LENGTH! (i) >= len
	{
	  if LENGTH! (i) > len
	    { i = split_interval_left (i, len); }

	  /* We have to call set_properties even if we are going to
	     merge the intervals, so as to make the undo records
	     and cause redisplay to happen.  */
	  set_properties (properties, i, object);
	  if !prev_changed.is_null ()
	    { merge_interval_left (i); }
	  return;
	}

      len -= LENGTH! (i);

      /* We have to call set_properties even if we are going to
	 merge the intervals, so as to make the undo records
	 and cause redisplay to happen.  */
      set_properties (properties, i, object);
      if prev_changed.is_null ()
	{ prev_changed = i; }
      else
	/* C chains the two assignments (`prev_changed = i = merge_interval_left (i))`);
	   Rust has no assignment expression, so the inner one is a block (the menu.rs
	   `*maps.offset (i) = { keymap = …; keymap }` shape).  */
	{ prev_changed = { i = merge_interval_left (i); i }; }

      i = next_interval (i);
      if !(len > 0)
	{ break; }
    }
}

DEFUN! ("remove-text-properties", Fremove_text_properties,
       Sremove_text_properties, 3, 4, 0,
       doc: "Remove some properties from text from START to END.\nThe third argument PROPERTIES is a property list\nwhose property names specify the properties to remove.\n(The values stored in PROPERTIES are ignored.)\nIf the optional fourth argument OBJECT is a buffer (or nil, which means\nthe current buffer), START and END are buffer positions (integers or\nmarkers).  If OBJECT is a string, START and END are 0-based indices into it.\nReturn t if any property was actually removed, nil otherwise.\n\nUse `set-text-properties' if you want to remove all text properties.",
       (start: Lisp_Object, end: Lisp_Object, properties: Lisp_Object,
	object: Lisp_Object),
       {
	 let mut start = start;
	 let mut end = end;
	 let mut object = object;

	 /* Ensure we run the modification hooks for the right buffer,
	    without switching buffers twice (bug 36190).  FIXME: Switching
	    buffers is slow and often unnecessary.  */
	 if BUFFERP (object) && XBUFFER (object) != current_buffer
	   {
	     let count: specpdl_ref = SPECPDL_INDEX ();
	     record_unwind_current_buffer ();
	     set_buffer_internal (XBUFFER (object));
	     return unbind_to (count,
			       Fremove_text_properties (start, end, properties,
							object));
	   }

	 let mut i: INTERVAL = std::ptr::null_mut ();
	 let mut unchanged: INTERVAL = std::ptr::null_mut ();
	 let mut s: ptrdiff_t = 0;
	 let mut len: ptrdiff_t = 0;
	 let mut modified: bool = false;
	 let mut first_time: bool = true;

	 if NILP (object)
	   { XSETBUFFER! (object, current_buffer); }

	'retry: loop {
	 i = validate_interval_range (object, &raw mut start, &raw mut end, soft);
	 if i.is_null ()
	   { return Qnil (); }

	 s = XFIXNUM (start);
	 len = XFIXNUM (end) - s;

	 /* If there are no properties on this entire interval, return.  */
	 if ! interval_has_some_properties (properties, i)
	   {
	     let mut got: ptrdiff_t = LENGTH! (i) - (s - (*((i) as *mut interval)).position);

	     loop
	       {
		 if got >= len
		   { return Qnil (); }
		 len -= got;
		 i = next_interval (i);
		 got = LENGTH! (i);
		 if interval_has_some_properties (properties, i)
		   { break; }
	       }
	   }
	 /* Split away the beginning of this interval; what we don't
	    want to modify.  */
	 else if (*((i) as *mut interval)).position != s
	   {
	     unchanged = i;
	     i = split_interval_right (unchanged,
				       s - (*((unchanged) as *mut interval)).position);
	     copy_properties (unchanged, i);
	   }

	 if BUFFERP (object) && first_time
	   {
	     let prev_total_length: ptrdiff_t = TOTAL_LENGTH! (i);
	     let prev_pos: ptrdiff_t = (*((i) as *mut interval)).position;

	     modify_text_properties (object, start, end);
	     /* If someone called us recursively as a side effect of
		modify_text_properties, and changed the intervals behind our back
		(could happen if lock_file, called by prepare_to_modify_buffer,
		triggers redisplay, and that calls add-text-properties again
		in the same buffer), we cannot continue with I, because its
		data changed.  So we restart the interval analysis anew.  */
	     if TOTAL_LENGTH! (i) != prev_total_length
		 || (*((i) as *mut interval)).position != prev_pos
	       {
		 first_time = false;
		 continue 'retry;
	       }
	   }

	 /* We are at the beginning of an interval, with len to scan */
	 loop
	   {
	     eassert! (!i.is_null ());

	     if LENGTH! (i) >= len
	       {
		 if ! interval_has_some_properties (properties, i)
		   {
		     eassert! (modified);
		     if BUFFERP (object)
		       { signal_after_change (XFIXNUM (start),
					      XFIXNUM (end) - XFIXNUM (start),
					      XFIXNUM (end) - XFIXNUM (start)); }
		     return Qt ();
		   }

		 if LENGTH! (i) == len
		   {
		     remove_properties (properties, Qnil (), i, object);
		     if BUFFERP (object)
		       { signal_after_change (XFIXNUM (start),
					      XFIXNUM (end) - XFIXNUM (start),
					      XFIXNUM (end) - XFIXNUM (start)); }
		     return Qt ();
		   }

		 /* i has the properties, and goes past the change limit */
		 unchanged = i;
		 i = split_interval_left (i, len);
		 copy_properties (unchanged, i);
		 remove_properties (properties, Qnil (), i, object);
		 if BUFFERP (object)
		   { signal_after_change (XFIXNUM (start),
					  XFIXNUM (end) - XFIXNUM (start),
					  XFIXNUM (end) - XFIXNUM (start)); }
		 return Qt ();
	       }

	     len -= LENGTH! (i);
	     modified |= remove_properties (properties, Qnil (), i, object);
	     i = next_interval (i);
	   }
	}
       });

DEFUN! ("remove-list-of-text-properties", Fremove_list_of_text_properties,
       Sremove_list_of_text_properties, 3, 4, 0,
       doc: "Remove some properties from text from START to END.\nThe third argument LIST-OF-PROPERTIES is a list of property names to remove.\nIf the optional fourth argument OBJECT is a buffer (or nil, which means\nthe current buffer), START and END are buffer positions (integers or\nmarkers).  If OBJECT is a string, START and END are 0-based indices into it.\nReturn t if any property was actually removed, nil otherwise.",
       (start: Lisp_Object, end: Lisp_Object, list_of_properties: Lisp_Object,
	object: Lisp_Object),
       {
	 let mut start = start;
	 let mut end = end;
	 let mut object = object;

	 /* Ensure we run the modification hooks for the right buffer,
	    without switching buffers twice (bug 36190).  FIXME: Switching
	    buffers is slow and often unnecessary.  */
	 if BUFFERP (object) && XBUFFER (object) != current_buffer
	   {
	     let count: specpdl_ref = SPECPDL_INDEX ();
	     record_unwind_current_buffer ();
	     set_buffer_internal (XBUFFER (object));
	     return unbind_to (count,
			       Fremove_list_of_text_properties (start, end,
								list_of_properties,
								object));
	   }

	 let mut i: INTERVAL = std::ptr::null_mut ();
	 let mut unchanged: INTERVAL = std::ptr::null_mut ();
	 let mut s: ptrdiff_t = 0;
	 let mut len: ptrdiff_t = 0;
	 let mut modified: bool = false;
	 let mut properties: Lisp_Object = Qnil ();
	 properties = list_of_properties;

	 if NILP (object)
	   { XSETBUFFER! (object, current_buffer); }

	 i = validate_interval_range (object, &raw mut start, &raw mut end, soft);
	 if i.is_null ()
	   { return Qnil (); }

	 s = XFIXNUM (start);
	 len = XFIXNUM (end) - s;

	 /* If there are no properties on the interval, return.  */
	 if ! interval_has_some_properties_list (properties, i)
	   {
	     let mut got: ptrdiff_t = LENGTH! (i) - (s - (*((i) as *mut interval)).position);

	     loop
	       {
		 if got >= len
		   { return Qnil (); }
		 len -= got;
		 i = next_interval (i);
		 got = LENGTH! (i);
		 if interval_has_some_properties_list (properties, i)
		   { break; }
	       }
	   }
	 /* Split away the beginning of this interval; what we don't
	    want to modify.  */
	 else if (*((i) as *mut interval)).position != s
	   {
	     unchanged = i;
	     i = split_interval_right (unchanged,
				       s - (*((unchanged) as *mut interval)).position);
	     copy_properties (unchanged, i);
	   }

	 /* We are at the beginning of an interval, with len to scan.
	    The flag MODIFIED records if changes have been made.
	    When object is a buffer, we must call modify_text_properties
	    before changes are made and signal_after_change when we are done.
	    Call modify_text_properties before calling remove_properties if !MODIFIED,
	    and call signal_after_change before returning if MODIFIED. */
	 loop
	   {
	     eassert! (!i.is_null ());

	     if LENGTH! (i) >= len
	       {
		 if ! interval_has_some_properties_list (properties, i)
		   {
		     if modified
		       {
			 if BUFFERP (object)
			   { signal_after_change (XFIXNUM (start),
						  XFIXNUM (end) - XFIXNUM (start),
						  XFIXNUM (end) - XFIXNUM (start)); }
			 return Qt ();
		       }
		     else
		       { return Qnil (); }
		   }
		 else if LENGTH! (i) == len
		   {
		     if !modified && BUFFERP (object)
		       { modify_text_properties (object, start, end); }
		     remove_properties (Qnil (), properties, i, object);
		     if BUFFERP (object)
		       { signal_after_change (XFIXNUM (start),
					      XFIXNUM (end) - XFIXNUM (start),
					      XFIXNUM (end) - XFIXNUM (start)); }
		     return Qt ();
		   }
		 else
		   { /* i has the properties, and goes past the change limit.  */
		     unchanged = i;
		     i = split_interval_left (i, len);
		     copy_properties (unchanged, i);
		     if !modified && BUFFERP (object)
		       { modify_text_properties (object, start, end); }
		     remove_properties (Qnil (), properties, i, object);
		     if BUFFERP (object)
		       { signal_after_change (XFIXNUM (start),
					      XFIXNUM (end) - XFIXNUM (start),
					      XFIXNUM (end) - XFIXNUM (start)); }
		     return Qt ();
		   }
	       }
	     if interval_has_some_properties_list (properties, i)
	       {
		 if !modified && BUFFERP (object)
		   { modify_text_properties (object, start, end); }
		 remove_properties (Qnil (), properties, i, object);
		 modified = true;
	       }
	     len -= LENGTH! (i);
	     i = next_interval (i);
	     if i.is_null ()
	       {
		 if modified
		   {
		     if BUFFERP (object)
		       { signal_after_change (XFIXNUM (start),
					      XFIXNUM (end) - XFIXNUM (start),
					      XFIXNUM (end) - XFIXNUM (start)); }
		     return Qt ();
		   }
		 else
		   { return Qnil (); }
	       }
	   }
       });


DEFUN! ("text-property-any", Ftext_property_any,
       Stext_property_any, 4, 5, 0,
       doc: "Check text from START to END for property PROPERTY equaling VALUE.\nIf so, return the position of the first character whose property PROPERTY\nis `eq' to VALUE.  Otherwise return nil.\nIf the optional fifth argument OBJECT is a buffer (or nil, which means\nthe current buffer), START and END are buffer positions (integers or\nmarkers).  If OBJECT is a string, START and END are 0-based indices into it.",
       (start: Lisp_Object, end: Lisp_Object, property: Lisp_Object,
	value: Lisp_Object, object: Lisp_Object),
       {
	 let mut start = start;
	 let mut end = end;
	 let mut object = object;
	 let mut i: INTERVAL = std::ptr::null_mut ();
	 let mut e: ptrdiff_t = 0;
	 let mut pos: ptrdiff_t = 0;

	 if NILP (object)
	   { XSETBUFFER! (object, current_buffer); }
	 i = validate_interval_range (object, &raw mut start, &raw mut end, soft);
	 if i.is_null ()
	   { return if !NILP (value) || EQ (start, end) { Qnil () } else { start }; }
	 e = XFIXNUM (end);

	 while !i.is_null ()
	   {
	     if (*((i) as *mut interval)).position >= e
	       { break; }
	     if EQ (textget ((*((i) as *mut interval)).plist, property), value)
	       {
		 pos = (*((i) as *mut interval)).position;
		 if pos < XFIXNUM (start)
		   { pos = XFIXNUM (start); }
		 return make_fixnum (pos);
	       }
	     i = next_interval (i);
	   }
	 return Qnil ();
       });

DEFUN! ("text-property-not-all", Ftext_property_not_all,
       Stext_property_not_all, 4, 5, 0,
       doc: "Check text from START to END for property PROPERTY not equaling VALUE.\nIf so, return the position of the first character whose property PROPERTY\nis not `eq' to VALUE.  Otherwise, return nil.\nIf the optional fifth argument OBJECT is a buffer (or nil, which means\nthe current buffer), START and END are buffer positions (integers or\nmarkers).  If OBJECT is a string, START and END are 0-based indices into it.",
       (start: Lisp_Object, end: Lisp_Object, property: Lisp_Object,
	value: Lisp_Object, object: Lisp_Object),
       {
	 let mut start = start;
	 let mut end = end;
	 let mut object = object;
	 let mut i: INTERVAL = std::ptr::null_mut ();
	 let mut s: ptrdiff_t = 0;
	 let mut e: ptrdiff_t = 0;

	 if NILP (object)
	   { XSETBUFFER! (object, current_buffer); }
	 i = validate_interval_range (object, &raw mut start, &raw mut end, soft);
	 if i.is_null ()
	   { return if NILP (value) || EQ (start, end) { Qnil () } else { start }; }
	 s = XFIXNUM (start);
	 e = XFIXNUM (end);

	 while !i.is_null ()
	   {
	     if (*((i) as *mut interval)).position >= e
	       { break; }
	     if ! EQ (textget ((*((i) as *mut interval)).plist, property), value)
	       {
		 if (*((i) as *mut interval)).position > s
		   { s = (*((i) as *mut interval)).position; }
		 return make_fixnum (s);
	       }
	     i = next_interval (i);
	   }
	 return Qnil ();
       });


/* Return the direction from which the text-property PROP would be
   inherited by any new text inserted at POS: 1 if it would be
   inherited from the char after POS, -1 if it would be inherited from
   the char before POS, and 0 if from neither.
   BUFFER can be either a buffer or nil (meaning current buffer).  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn text_property_stickiness (prop: Lisp_Object, pos: Lisp_Object,
						   buffer: Lisp_Object) -> c_int
{
  let mut ignore_previous_character: bool;
  let prev_pos: Lisp_Object = make_fixnum (XFIXNUM (pos) - 1);
  let mut front_sticky: Lisp_Object = Qnil ();
  let mut is_rear_sticky: bool = true;
  let mut is_front_sticky: bool = false; /* defaults */
  let defalt: Lisp_Object = Fassq (prop, Vtext_property_default_nonsticky);
  let mut buffer = buffer;

  if NILP (buffer)
    { XSETBUFFER! (buffer, current_buffer); }

  ignore_previous_character = XFIXNUM (pos) <= BUF_BEGV (XBUFFER (buffer));

  if ignore_previous_character || (CONSP (defalt) && !NILP (XCDR (defalt)))
    { is_rear_sticky = false; }
  else
    {
      let rear_non_sticky: Lisp_Object
	= Fget_text_property (prev_pos, Qrear_nonsticky (), buffer);

      if !NILP (if CONSP (rear_non_sticky)
		  { Fmemq (prop, rear_non_sticky) }
		  else { rear_non_sticky })
	/* PROP is rear-non-sticky.  */
	{ is_rear_sticky = false; }
    }

  /* Consider following character.  */
  /* This signals an arg-out-of-range error if pos is outside the
     buffer's accessible range.  */
  front_sticky = Fget_text_property (pos, Qfront_sticky (), buffer);

  if EQ (front_sticky, Qt ())
      || (CONSP (front_sticky)
	  && !NILP (Fmemq (prop, front_sticky)))
    /* PROP is inherited from after.  */
    { is_front_sticky = true; }

  /* Simple cases, where the properties are consistent.  */
  if is_rear_sticky && !is_front_sticky
    { return -1; }
  else if !is_rear_sticky && is_front_sticky
    { return 1; }
  else if !is_rear_sticky && !is_front_sticky
    { return 0; }

  /* The stickiness properties are inconsistent, so we have to
     disambiguate.  Basically, rear-sticky wins, _except_ if the
     property that would be inherited has a value of nil, in which case
     front-sticky wins.  */
  if ignore_previous_character
      || NILP (Fget_text_property (prev_pos, prop, buffer))
    { return 1; }
  else
    { return -1; }
}


/* Copying properties between objects. */

/* Add properties from START to END of SRC, starting at POS in DEST.
   SRC and DEST may each refer to strings or buffers.
   Optional sixth argument PROP causes only that property to be copied.
   Properties are copied to DEST as if by `add-text-properties'.
   Return t if any property value actually changed, nil otherwise.  */

/* Note this can GC when DEST is a buffer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn copy_text_properties (start: Lisp_Object, end: Lisp_Object,
					       src: Lisp_Object, pos: Lisp_Object,
					       dest: Lisp_Object, prop: Lisp_Object)
     -> Lisp_Object
{
  let mut start = start;
  let mut end = end;
  let mut pos = pos;
  let mut i: INTERVAL = validate_interval_range (src, &raw mut start, &raw mut end, soft);
  if i.is_null ()
    { return Qnil (); }

  CHECK_FIXNUM_COERCE_MARKER! (pos);

  let dest_e: EMACS_INT = XFIXNUM (pos) + (XFIXNUM (end) - XFIXNUM (start));
  if MOST_POSITIVE_FIXNUM < dest_e
    { args_out_of_range (pos, end); }
  let mut dest_end: Lisp_Object = make_fixnum (dest_e);
  validate_interval_range (dest, &raw mut pos, &raw mut dest_end, soft);

  let mut s: ptrdiff_t = XFIXNUM (start);
  let e: ptrdiff_t = XFIXNUM (end);
  let mut p: ptrdiff_t = XFIXNUM (pos);

  let mut stuff: Lisp_Object = Qnil ();

  while s < e
    {
      let mut e2: ptrdiff_t = (*((i) as *mut interval)).position + LENGTH! (i);
      if e2 > e
	{ e2 = e; }
      let len: ptrdiff_t = e2 - s;

      let mut plist: Lisp_Object = (*((i) as *mut interval)).plist;
      if ! NILP (prop)
	{
	  while ! NILP (plist)
	    {
	      if EQ (Fcar (plist), prop)
		{
		  plist = list2 (prop, Fcar (Fcdr (plist)));
		  break;
		}
	      plist = Fcdr (Fcdr (plist));
	    }
	}
      if ! NILP (plist)
	/* Must defer modifications to the interval tree in case
	   src and dest refer to the same string or buffer.  */
	{ stuff = Fcons (list3 (make_fixnum (p), make_fixnum (p + len), plist),
			 stuff); }

      i = next_interval (i);
      if i.is_null ()
	{ break; }

      p += len;
      s = (*((i) as *mut interval)).position;
    }

  let mut modified: bool = false;

  while ! NILP (stuff)
    {
      let mut res: Lisp_Object = Fcar (stuff);
      res = Fadd_text_properties (Fcar (res), Fcar (Fcdr (res)),
				  Fcar (Fcdr (Fcdr (res))), dest);
      if ! NILP (res)
	{ modified = true; }
      stuff = Fcdr (stuff);
    }

  return if modified { Qt () } else { Qnil () };
}


/* Return a list representing the text properties of OBJECT between
   START and END.  if PROP is non-nil, report only on that property.
   Each result list element has the form (S E PLIST), where S and E
   are positions in OBJECT and PLIST is a property list containing the
   text properties of OBJECT between S and E.  Value is nil if OBJECT
   doesn't contain text properties between START and END.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn text_property_list (object: Lisp_Object, start: Lisp_Object,
					     end: Lisp_Object, prop: Lisp_Object)
     -> Lisp_Object
{
  let mut start = start;
  let mut end = end;
  let mut i: INTERVAL = std::ptr::null_mut ();
  let mut result: Lisp_Object = Qnil ();

  result = Qnil ();

  i = validate_interval_range (object, &raw mut start, &raw mut end, soft);
  if !i.is_null ()
    {
      let mut s: ptrdiff_t = XFIXNUM (start);
      let e: ptrdiff_t = XFIXNUM (end);

      while s < e
	{
	  let mut interval_end: ptrdiff_t = 0;
	  let mut len: ptrdiff_t = 0;
	  let mut plist: Lisp_Object = Qnil ();

	  interval_end = (*((i) as *mut interval)).position + LENGTH! (i);
	  if interval_end > e
	    { interval_end = e; }
	  len = interval_end - s;

	  plist = (*((i) as *mut interval)).plist;

	  if !NILP (prop)
	    {
	      while CONSP (plist)
		{
		  if EQ (XCAR (plist), prop)
		    {
		      plist = list2 (prop, Fcar (XCDR (plist)));
		      break;
		    }
		  plist = Fcdr (XCDR (plist));
		}
	    }

	  if !NILP (plist)
	    { result = Fcons (list3 (make_fixnum (s), make_fixnum (s + len),
				     plist),
			      result); }

	  i = next_interval (i);
	  if i.is_null ()
	    { break; }
	  s = (*((i) as *mut interval)).position;
	}
    }

  return result;
}


/* Add text properties to OBJECT from LIST.  LIST is a list of triples
   (START END PLIST), where START and END are positions and PLIST is a
   property list containing the text properties to add.  Adjust START
   and END positions by DELTA before adding properties.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn add_text_properties_from_list (object: Lisp_Object, list: Lisp_Object,
							delta: Lisp_Object)
{
  let mut list = list;
  while CONSP (list)
    {
      let mut item: Lisp_Object = Qnil ();
      let mut start: Lisp_Object = Qnil ();
      let mut end: Lisp_Object = Qnil ();
      let mut plist: Lisp_Object = Qnil ();

      item = XCAR (list);
      start = make_fixnum (XFIXNUM (XCAR (item)) + XFIXNUM (delta));
      end = make_fixnum (XFIXNUM (XCAR (XCDR (item))) + XFIXNUM (delta));
      plist = XCAR (XCDR (XCDR (item)));

      Fadd_text_properties (start, end, plist, object);
      list = XCDR (list);
    }
}



/* Modify end-points of ranges in LIST destructively, and return the
   new list.  LIST is a list as returned from text_property_list.
   Discard properties that begin at or after NEW_END, and limit
   end-points to NEW_END.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn extend_property_ranges (list: Lisp_Object, old_end: Lisp_Object,
						 new_end: Lisp_Object) -> Lisp_Object
{
  let mut list = list;
  let mut prev: Lisp_Object = Qnil ();
  let mut head: Lisp_Object = list;
  let max: ptrdiff_t = XFIXNUM (new_end);

  loop
    {
      if !CONSP (list)
	{ break; }
      let mut item: Lisp_Object = Qnil ();
      let mut beg: Lisp_Object = Qnil ();
      let mut end: ptrdiff_t = 0;

      item = XCAR (list);
      beg = XCAR (item);
      end = XFIXNUM (XCAR (XCDR (item)));

      if XFIXNUM (beg) >= max
	{
	  /* The start-point is past the end of the new string.
	     Discard this property.  */
	  if EQ (head, list)
	    { head = XCDR (list); }
	  else
	    { XSETCDR (prev, XCDR (list)); }
	}
      else if (end == XFIXNUM (old_end) && end != max)
	       || end > max
	{
	  /* Either the end-point is past the end of the new string,
	     and we need to discard the properties past the new end,
	     or the caller is extending the property range, and we
	     should update all end-points that are on the old end of
	     the range to reflect that.  */
	  XSETCAR (XCDR (item), new_end);
	}

      prev = list;
      list = XCDR (list);
    }

  return head;
}



/* Call the modification hook functions in LIST, each with START and END.  */

unsafe fn call_mod_hooks (list: Lisp_Object, start: Lisp_Object, end: Lisp_Object)
{
  let mut list = list;
  while !NILP (list)
    {
      calln! (Fcar (list), start, end);
      list = Fcdr (list);
    }
}

/* Check for read-only intervals between character positions START ... END,
   in BUF, and signal an error if we find one.

   Then check for any modification hooks in the range.
   Create a list of all these hooks in lexicographic order,
   eliminating consecutive extra copies of the same hook.  Then call
   those hooks in order, with START and END - 1 as arguments.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn verify_interval_modification (buf: *mut buffer, mut start: ptrdiff_t,
						       mut end: ptrdiff_t)
{
  let intervals: INTERVAL = buffer_intervals (buf);
  let mut i: INTERVAL = std::ptr::null_mut ();
  let mut p: ptrdiff_t = 0;
  let mut hooks: Lisp_Object = Qnil ();
  let mut prev_mod_hooks: Lisp_Object = Qnil ();
  let mut mod_hooks: Lisp_Object = Qnil ();

  hooks = Qnil ();
  prev_mod_hooks = Qnil ();
  mod_hooks = Qnil ();

  interval_insert_behind_hooks = Qnil ();
  interval_insert_in_front_hooks = Qnil ();

  if intervals.is_null ()
    { return; }

  if start > end
    {
      let temp: ptrdiff_t = start;
      start = end;
      end = temp;
    }

  /* For an insert operation, check the two chars around the position.  */
  if start == end
    {
      let mut prev: INTERVAL = std::ptr::null_mut ();
      let mut before: Lisp_Object = Qnil ();
      let mut after: Lisp_Object = Qnil ();

      /* Set I to the interval containing the char after START,
	 and PREV to the interval containing the char before START.
	 Either one may be null.  They may be equal.  */
      i = find_interval (intervals, start);

      if start == BUF_BEGV (buf)
	{ prev = std::ptr::null_mut (); }
      else if (*((i) as *mut interval)).position == start
	{ prev = previous_interval (i); }
      else if (*((i) as *mut interval)).position < start
	{ prev = i; }
      if start == BUF_ZV (buf)
	{ i = std::ptr::null_mut (); }

      /* If Vinhibit_read_only is set and is not a list, we can
	 skip the read_only checks.  */
      if NILP (Vinhibit_read_only) || CONSP (Vinhibit_read_only)
	{
	  /* If I and PREV differ we need to check for the read-only
	     property together with its stickiness.  If either I or
	     PREV are 0, this check is all we need.
	     We have to take special care, since read-only may be
	     indirectly defined via the category property.  */
	  if i != prev
	    {
	      if !i.is_null ()
		{
		  after = textget ((*((i) as *mut interval)).plist, Qread_only ());

		  /* If interval I is read-only and read-only is
		     front-sticky, inhibit insertion.
		     Check for read-only as well as category.  */
		  if ! NILP (after)
		      && NILP (Fmemq (after, Vinhibit_read_only))
		    {
		      let mut tem: Lisp_Object = Qnil ();

		      tem = textget ((*((i) as *mut interval)).plist, Qfront_sticky ());
		      if TMEM! (Qread_only (), tem)
			  || (NILP (plist_get ((*((i) as *mut interval)).plist,
					       Qread_only ()))
			      && TMEM! (Qcategory (), tem))
			{ text_read_only (after); }
		    }
		}

	      if !prev.is_null ()
		{
		  before = textget ((*((prev) as *mut interval)).plist, Qread_only ());

		  /* If interval PREV is read-only and read-only isn't
		     rear-nonsticky, inhibit insertion.
		     Check for read-only as well as category.  */
		  if ! NILP (before)
		      && NILP (Fmemq (before, Vinhibit_read_only))
		    {
		      let mut tem: Lisp_Object = Qnil ();

		      tem = textget ((*((prev) as *mut interval)).plist,
				     Qrear_nonsticky ());
		      if ! TMEM! (Qread_only (), tem)
			  && (! NILP (plist_get ((*((prev) as *mut interval)).plist,
						 Qread_only ()))
			      || ! TMEM! (Qcategory (), tem))
			{ text_read_only (before); }
		    }
		}
	    }
	  else if !i.is_null ()
	    {
	      after = textget ((*((i) as *mut interval)).plist, Qread_only ());

	      /* If interval I is read-only and read-only is
		 front-sticky, inhibit insertion.
		 Check for read-only as well as category.  */
	      if ! NILP (after) && NILP (Fmemq (after, Vinhibit_read_only))
		{
		  let mut tem: Lisp_Object = Qnil ();

		  tem = textget ((*((i) as *mut interval)).plist, Qfront_sticky ());
		  if TMEM! (Qread_only (), tem)
		      || (NILP (plist_get ((*((i) as *mut interval)).plist, Qread_only ()))
			  && TMEM! (Qcategory (), tem))
		    { text_read_only (after); }

		  tem = textget ((*((prev) as *mut interval)).plist, Qrear_nonsticky ());
		  if ! TMEM! (Qread_only (), tem)
		      && (! NILP (plist_get ((*((prev) as *mut interval)).plist,
					     Qread_only ()))
			  || ! TMEM! (Qcategory (), tem))
		    { text_read_only (after); }
		}
	    }
	}

      /* Run both insert hooks (just once if they're the same).  */
      if !prev.is_null ()
	{ interval_insert_behind_hooks
	    = textget ((*((prev) as *mut interval)).plist, Qinsert_behind_hooks ()); }
      if !i.is_null ()
	{ interval_insert_in_front_hooks
	    = textget ((*((i) as *mut interval)).plist, Qinsert_in_front_hooks ()); }
    }
  else
    {
      let mut buffer_read_only: bool;

      /* Loop over intervals on or next to START...END,
	 collecting their hooks.  */

      /* Extent of last writable interval.  */
      i = find_interval (intervals, start);
      p = 0;
      buffer_read_only = (!NILP (BVAR! (current_buffer, read_only_))
			  && NILP (Vinhibit_read_only));
      loop
	{
	  let mut implied: bool;
	  let mut express: bool;
	  let mut read_only: Lisp_Object = Qnil ();

	  read_only = textget ((*((i) as *mut interval)).plist, Qread_only ());
	  implied = INTERVAL_GENERALLY_WRITABLE_P! (i, read_only);
	  express = INTERVAL_EXPRESSLY_WRITABLE_P! (i, read_only);
	  if !implied && !express
	    { text_read_only (read_only); }
	  /* If this interval is only implicitly read only and the
	     buffer is read only as a whole, signal an error.  */
	  else if !express && buffer_read_only
	    { xsignal1 (Qbuffer_read_only (), Fcurrent_buffer ()); }

	  if !inhibit_modification_hooks
	    {
	      mod_hooks = textget ((*((i) as *mut interval)).plist,
				   Qmodification_hooks ());
	      if ! NILP (mod_hooks) && ! EQ (mod_hooks, prev_mod_hooks)
		{
		  hooks = Fcons (mod_hooks, hooks);
		  prev_mod_hooks = mod_hooks;
		}
	    }

	  p = (*((i) as *mut interval)).position + LENGTH! (i);
	  i = next_interval (i);
	  /* Keep going thru the interval containing the char before END.  */
	  if !(!i.is_null () && (*((i) as *mut interval)).position < end)
	    { break; }
	}

      /* Should the buffer be read only while the last interval with an
	 `inhibit-read-only' property does not enclose the entire change
	 under consideration, signal error.  */
      if p < end && buffer_read_only
	{ xsignal1 (Qbuffer_read_only (), Fcurrent_buffer ()); }

      if !inhibit_modification_hooks
	{
	  hooks = Fnreverse (hooks);
	  while ! NILP (hooks)
	    {
	      call_mod_hooks (Fcar (hooks), make_fixnum (start),
			      make_fixnum (end));
	      hooks = Fcdr (hooks);
	    }
	}
    }
}

/* Run the interval hooks for an insertion on character range START ... END.
   verify_interval_modification chose which hooks to run;
   this function is called after the insertion happens
   so it can indicate the range of inserted text.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn report_interval_modification (start: Lisp_Object, end: Lisp_Object)
{
  if ! NILP (interval_insert_behind_hooks)
    { call_mod_hooks (interval_insert_behind_hooks, start, end); }
  if ! NILP (interval_insert_in_front_hooks)
      && ! EQ (interval_insert_in_front_hooks,
	       interval_insert_behind_hooks)
    { call_mod_hooks (interval_insert_in_front_hooks, start, end); }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_textprop ()
{
  DEFVAR_LISP! (c"default-text-properties", Vdefault_text_properties,
		"Property-list used as default values.\nThe value of a property in this list is seen as the value for every\ncharacter that does not have its own value for that property.  ");
  Vdefault_text_properties = Qnil ();

  DEFVAR_LISP! (c"char-property-alias-alist", Vchar_property_alias_alist,
		"Alist of alternative properties for properties without a value.\nEach element should look like (PROPERTY ALTERNATIVE1 ALTERNATIVE2...).\nIf a piece of text has no direct value for a particular property, then\nthis alist is consulted.  If that property appears in the alist, then\nthe first non-nil value from the associated alternative properties is\nreturned. ");
  Vchar_property_alias_alist = Qnil ();

  DEFVAR_LISP! (c"inhibit-point-motion-hooks", Vinhibit_point_motion_hooks,
		"If non-nil, don't run `point-left' and `point-entered' text properties.\nThis also inhibits the use of the `intangible' text property.  ");
  Vinhibit_point_motion_hooks = Qt ();

  DEFVAR_LISP! (c"text-property-default-nonsticky",
		Vtext_property_default_nonsticky,
		"Alist of properties vs the corresponding non-stickiness.\nEach element has the form (PROPERTY . NONSTICKINESS).\n\nIf a character in a buffer has PROPERTY, new text inserted adjacent to\nthe character doesn't inherit PROPERTY if NONSTICKINESS is non-nil,\ninherits it if NONSTICKINESS is nil.  The `front-sticky' and\n`rear-nonsticky' properties of the character override NONSTICKINESS.  ");
  /* Text properties `syntax-table'and `display' should be nonsticky
     by default.  */
  Vtext_property_default_nonsticky
    = list2 (Fcons (Qsyntax_table (), Qt ()), Fcons (Qdisplay (), Qt ()));
  Fmake_variable_buffer_local (Qtext_property_default_nonsticky ());

  interval_insert_behind_hooks = Qnil ();
  interval_insert_in_front_hooks = Qnil ();
  staticpro (&raw const interval_insert_behind_hooks);
  staticpro (&raw const interval_insert_in_front_hooks);

  /* Common attributes one might give text.  */

  DEFSYM! (Qfont, c"font");
  DEFSYM! (Qface, c"face");
  DEFSYM! (Qread_only, c"read-only");
  DEFSYM! (Qinvisible, c"invisible");
  DEFSYM! (Qintangible, c"intangible");
  DEFSYM! (Qcategory, c"category");
  DEFSYM! (Qlocal_map, c"local-map");
  DEFSYM! (Qfront_sticky, c"front-sticky");
  DEFSYM! (Qrear_nonsticky, c"rear-nonsticky");
  DEFSYM! (Qtext_property_default_nonsticky, c"text-property-default-nonsticky");
  DEFSYM! (Qmouse_face, c"mouse-face");
  DEFSYM! (Qminibuffer_prompt, c"minibuffer-prompt");

  /* Properties that text might use to specify certain actions.  */

  DEFSYM! (Qpoint_left, c"point-left");
  DEFSYM! (Qpoint_entered, c"point-entered");

  defsubr (&raw mut Stext_properties_at);
  defsubr (&raw mut Sget_text_property);
  defsubr (&raw mut Sget_char_property);
  defsubr (&raw mut Sget_char_property_and_overlay);
  defsubr (&raw mut Snext_char_property_change);
  defsubr (&raw mut Sprevious_char_property_change);
  defsubr (&raw mut Snext_single_char_property_change);
  defsubr (&raw mut Sprevious_single_char_property_change);
  defsubr (&raw mut Snext_property_change);
  defsubr (&raw mut Snext_single_property_change);
  defsubr (&raw mut Sprevious_property_change);
  defsubr (&raw mut Sprevious_single_property_change);
  defsubr (&raw mut Sadd_text_properties);
  defsubr (&raw mut Sput_text_property);
  defsubr (&raw mut Sset_text_properties);
  defsubr (&raw mut Sadd_face_text_property);
  defsubr (&raw mut Sremove_text_properties);
  defsubr (&raw mut Sremove_list_of_text_properties);
  defsubr (&raw mut Stext_property_any);
  defsubr (&raw mut Stext_property_not_all);
}
