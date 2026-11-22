// femacs: port of src/undo.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  The one
// subr uses `DEFUN!` (PORTING.md 5.1).
//
// The `#include` list mirrors undo.c 21-25 - config.h, lisp.h, buffer.h, keyboard.h - with
// the mirrors those pull in.  undo.c has no preprocessor conditional at all, so the whole
// file is live text.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] ../lib/include/mini-gmp.rs imported via common
// [U1] include/bignum.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/font.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/keyboard.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/process.rs imported via common
// [U1] include/frame.rs imported via common
// [U1] include/keymap.rs imported via common

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
// port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
// defines it here (status §3.29, the casetab.rs shape).
#[unsafe(no_mangle)]
pub static undo_limit: VmGlobal<EMACS_INT> = VmGlobal::<EMACS_INT>::new (0);
#[unsafe(no_mangle)]
pub static undo_strong_limit: VmGlobal<EMACS_INT> = VmGlobal::<EMACS_INT>::new (0);
#[unsafe(no_mangle)]
pub static Vundo_outer_limit: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static Vundo_outer_limit_function: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());
#[unsafe(no_mangle)]
pub static undo_inhibit_record_point: VmGlobal<bool> = VmGlobal::<bool>::new (false);

/* The first time a command records something for undo.
   it also allocates the undo-boundary object
   which will be added to the list at the end of the command.
   This ensures we can't run out of space while trying to make
   an undo-boundary.  */
// undo.c 32: `static Lisp_Object pending_boundary;` - a bss object, and the port's
// NIL_IS_ZERO makes its zero the same word as C's (the established UNINIT class).
static pending_boundary: VmGlobal<Lisp_Object> = VmGlobal::<Lisp_Object>::new (lisp_h_Qnil ());

/* Prepare the undo info for recording a change. */
unsafe fn prepare_record ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  /* Allocate a cons cell to be the undo boundary after this command.  */
  if NILP ((*pending_boundary.as_mut_ptr ()))
    { (*pending_boundary.as_mut_ptr ()) = Fcons (Qnil (), Qnil ()); }

  }
}

/* Record point, if necessary, as it was at beginning of this command.
   BEG is the position of point that will naturally occur as a
   result of the undo record that will be added just after this command
   terminates.  */
unsafe fn record_point (beg: ptrdiff_t)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  /* Don't record position of pt when undo_inhibit_record_point holds.  */
  if (*undo_inhibit_record_point.as_mut_ptr ())
    { return; }

  let at_boundary: bool;

  /* Check whether we are at a boundary now, in case we record the
  first change. FIXME: This check is currently dependent on being
  called before record_first_change, but could be made not to by
  ignoring timestamp undo entries */
  at_boundary = ! CONSP (BVAR! ((current_buffer! ()), undo_list_))
                || NILP (XCAR (BVAR! ((current_buffer! ()), undo_list_)));

  /* If this is the first change since save, then record this.*/
  if MODIFF! () <= SAVE_MODIFF! ()
    { record_first_change (); }

  /* We may need to record point if we are immediately after a
     boundary, so that this will be restored correctly after undo. We
     do not need to do this if point is at the start of a change
     region since it will be restored there anyway, and we must not do
     this if the buffer has changed since the last command, since the
     value of point that we have will be for that buffer, not this.*/
  if at_boundary
      && (*point_before_last_command_or_undo.as_mut_ptr ()) != beg
      && (*buffer_before_last_command_or_undo.as_mut_ptr ()) == (current_buffer! ())
    {
      bset_undo_list ((current_buffer! ()),
                      Fcons (make_fixnum ((*point_before_last_command_or_undo.as_mut_ptr ())),
                             BVAR! ((current_buffer! ()), undo_list_)));
    }

  }
}

/* Record an insertion that just happened or is about to happen,
   for LENGTH characters at position BEG.
   (It is possible to record an insertion before or after the fact
   because we don't need to record the contents.)  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_insert (beg: ptrdiff_t, length: ptrdiff_t)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut lbeg: Lisp_Object = Qnil ();
  let mut lend: Lisp_Object = Qnil ();

  if EQ (BVAR! ((current_buffer! ()), undo_list_), Qt ())
    { return; }

  prepare_record ();

  record_point (beg);

  /* If this is following another insertion and consecutive with it
     in the buffer, combine the two.  */
  if CONSP (BVAR! ((current_buffer! ()), undo_list_))
    {
      let elt: Lisp_Object = XCAR (BVAR! ((current_buffer! ()), undo_list_));
      if CONSP (elt)
          && FIXNUMP (XCAR (elt))
          && FIXNUMP (XCDR (elt))
          && XFIXNUM (XCDR (elt)) == beg
        {
          XSETCDR (elt, make_fixnum (beg + length));
          return;
        }
    }

  XSETFASTINT! (lbeg, beg);
  XSETINT! (lend, beg + length);
  bset_undo_list ((current_buffer! ()),
                  Fcons (Fcons (lbeg, lend), BVAR! ((current_buffer! ()), undo_list_)));

  }
}

/* Record the fact that markers in the region of FROM, TO are about to
   be adjusted.  This is done only when a marker points within text
   being deleted, because that's the only case where an automatic
   marker adjustment won't be inverted automatically by undoing the
   buffer modification.  */

unsafe fn record_marker_adjustments (from: ptrdiff_t, to: ptrdiff_t)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  prepare_record ();

  let mut m: *mut Lisp_Marker = BUF_MARKERS! ((current_buffer! ()));
  while ! m.is_null ()
    {
      let charpos: ptrdiff_t = (*m).charpos;
      eassert! (charpos <= Z! () as ptrdiff_t);

      if from <= charpos && charpos <= to
        {
          /* insertion_type nil markers will end up at the beginning of
             the re-inserted text after undoing a deletion, and must be
             adjusted to move them to the correct place.

             insertion_type t markers will automatically move forward
             upon re-inserting the deleted text, so we have to arrange
             for them to move backward to the correct position.  */
          let adjustment: ptrdiff_t =
            (if MARKER_INSERTION_TYPE_P (m) { to } else { from }) - charpos;

          if adjustment != 0
            {
              let marker: Lisp_Object = make_lisp_ptr (m as *mut c_void, Lisp_Type::Lisp_Vectorlike);
              bset_undo_list
                ((current_buffer! ()),
                 Fcons (Fcons (marker, make_fixnum (adjustment)),
                        BVAR! ((current_buffer! ()), undo_list_)));
            }
        }
      m = (*m).next;
    }

  }
}

/* Record that a deletion is about to take place, of the characters in
   STRING, at location BEG.  Optionally record adjustments for markers
   in the region STRING occupies in the current buffer.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_delete (beg: ptrdiff_t, string: Lisp_Object,
                                        record_markers: bool)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut sbeg: Lisp_Object = Qnil ();

  if EQ (BVAR! ((current_buffer! ()), undo_list_), Qt ())
    { return; }

  prepare_record ();

  record_point (beg);

  if PT! () as ptrdiff_t == beg + SCHARS (string)
    { XSETINT! (sbeg, -beg); }
  else
    { XSETFASTINT! (sbeg, beg); }

  /* primitive-undo assumes marker adjustments are recorded
     immediately before the deletion is recorded.  See bug 16818
     discussion.  */
  if record_markers
    { record_marker_adjustments (beg, beg + SCHARS (string)); }

  bset_undo_list
    ((current_buffer! ()),
     Fcons (Fcons (string, sbeg), BVAR! ((current_buffer! ()), undo_list_)));

  }
}

/* Record that a replacement is about to take place,
   for LENGTH characters at location BEG.
   The replacement must not change the number of characters.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_change (beg: ptrdiff_t, length: ptrdiff_t)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  record_delete (beg, make_buffer_string (beg, beg + length, true), false);
  record_insert (beg, length);

  }
}

/* Record that an unmodified buffer is about to be changed.
   Record the file modification date so that when undoing this entry
   we can tell whether it is obsolete because the file was saved again.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_first_change ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut base_buffer: *mut buffer = (current_buffer! ());

  if EQ (BVAR! ((current_buffer! ()), undo_list_), Qt ())
    { return; }

  if ! (*base_buffer).base_buffer.is_null ()
    { base_buffer = (*base_buffer).base_buffer; }

  bset_undo_list ((current_buffer! ()),
                  Fcons (Fcons (Qt (), buffer_visited_file_modtime (base_buffer as *mut c_void)),
                         BVAR! ((current_buffer! ()), undo_list_)));

  }
}

/* Record a change in property PROP (whose old value was VAL)
   for LENGTH characters starting at position BEG in BUFFER.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn record_property_change (beg: ptrdiff_t, length: ptrdiff_t,
                                                 prop: Lisp_Object, value: Lisp_Object,
                                                 buffer: Lisp_Object)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut lbeg: Lisp_Object = Qnil ();
  let mut lend: Lisp_Object = Qnil ();
  let mut entry: Lisp_Object = Qnil ();
  let buf: *mut buffer = XBUFFER (buffer);

  if EQ (BVAR! (buf, undo_list_), Qt ())
    { return; }

  prepare_record ();

  if MODIFF! () <= SAVE_MODIFF! ()
    { record_first_change (); }

  XSETINT! (lbeg, beg);
  XSETINT! (lend, beg + length);
  entry = Fcons (Qnil (), Fcons (prop, Fcons (value, Fcons (lbeg, lend))));
  bset_undo_list ((current_buffer! ()),
                  Fcons (entry, BVAR! ((current_buffer! ()), undo_list_)));

  }
}

DEFUN! ("undo-boundary", Fundo_boundary, Sundo_boundary, 0, 0, 0,
       doc: "Mark a boundary between units of undo.\nAn undo command will stop at this point,\nbut another undo command will undo to the previous boundary.",
       (),
       {
  let mut tem: Lisp_Object = Qnil ();
  if EQ (BVAR! ((current_buffer! ()), undo_list_), Qt ())
    { return Qnil (); }
  tem = Fcar (BVAR! ((current_buffer! ()), undo_list_));
  if ! NILP (tem)
    {
      /* One way or another, cons nil onto the front of the undo list.  */
      if ! NILP ((*pending_boundary.as_mut_ptr ()))
        {
          /* If we have preallocated the cons cell to use here,
             use that one.  */
          XSETCDR ((*pending_boundary.as_mut_ptr ()), BVAR! ((current_buffer! ()), undo_list_));
          bset_undo_list ((current_buffer! ()), (*pending_boundary.as_mut_ptr ()));
          (*pending_boundary.as_mut_ptr ()) = Qnil ();
        }
      else
        { bset_undo_list ((current_buffer! ()),
                          Fcons (Qnil (), BVAR! ((current_buffer! ()), undo_list_))); }
    }

  Fset (Qundo_auto__last_boundary_cause (), Qexplicit ());
  (*point_before_last_command_or_undo.as_mut_ptr ()) = PT! () as ptrdiff_t;
  (*buffer_before_last_command_or_undo.as_mut_ptr ()) = (current_buffer! ());

  return Qnil ();
});

/* At garbage collection time, make an undo list shorter at the end,
   returning the truncated list.  How this is done depends on the
   variables undo-limit, undo-strong-limit and undo-outer-limit.
   In some cases this works by calling undo-outer-limit-function.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn truncate_undo_list (b: *mut buffer)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let mut list: Lisp_Object;
  let mut prev: Lisp_Object;
  let mut next: Lisp_Object;
  let mut last_boundary: Lisp_Object;
  let mut size_so_far: intmax_t = 0;

  /* Make sure that calling undo-outer-limit-function
     won't cause another GC.  */
  let count: specpdl_ref = inhibit_garbage_collection ();

  /* Make the buffer current to get its local values of variables such
     as undo_limit.  Also so that Vundo_outer_limit_function can
     tell which buffer to operate on.  */
  record_unwind_current_buffer ();
  set_buffer_internal (b);

  list = BVAR! (b, undo_list_);

  prev = Qnil ();
  next = list;
  last_boundary = Qnil ();

  /* If the first element is an undo boundary, skip past it.  */
  if CONSP (next) && NILP (XCAR (next))
    {
      /* Add in the space occupied by this element and its chain link.  */
      size_so_far += std::mem::size_of::<Lisp_Cons> () as intmax_t;

      /* Advance to next element.  */
      prev = next;
      next = XCDR (next);
    }

  /* Always preserve at least the most recent undo record
     unless it is really horribly big.

     Skip, skip, skip the undo, skip, skip, skip the undo,
     Skip, skip, skip the undo, skip to the undo bound'ry.  */

  while CONSP (next) && ! NILP (XCAR (next))
    {
      let elt: Lisp_Object = XCAR (next);

      /* Add in the space occupied by this element and its chain link.  */
      size_so_far += std::mem::size_of::<Lisp_Cons> () as intmax_t;
      if CONSP (elt)
        {
          size_so_far += std::mem::size_of::<Lisp_Cons> () as intmax_t;
          if STRINGP (XCAR (elt))
            {
              size_so_far += (std::mem::size_of::<Lisp_String> () - 1
                              + SCHARS (XCAR (elt)) as usize) as intmax_t;
            }
        }

      /* Advance to next element.  */
      prev = next;
      next = XCDR (next);
    }

  /* If by the first boundary we have already passed undo_outer_limit,
     we're heading for memory full, so offer to clear out the list.  */
  /* C's `intmax_t undo_outer_limit;` is filled only by the `integer_to_intmax` below.  */
  let mut undo_outer_limit: intmax_t = 0;
  if (if INTEGERP ((*Vundo_outer_limit.as_mut_ptr ()))
        {
          (if integer_to_intmax ((*Vundo_outer_limit.as_mut_ptr ()), &raw mut undo_outer_limit)
             { undo_outer_limit < size_so_far }
             else { NILP (Fnatnump ((*Vundo_outer_limit.as_mut_ptr ()))) })
        }
        else { false })
      && ! NILP ((*Vundo_outer_limit_function.as_mut_ptr ()))
    {
      let tem: Lisp_Object;

      /* Normally the function this calls is undo-outer-limit-truncate.  */
      tem = calln! ((*Vundo_outer_limit_function.as_mut_ptr ()), make_int (size_so_far));
      if ! NILP (tem)
        {
          /* The function is responsible for making
             any desired changes in buffer-undo-list.  */
          unbind_to (count, Qnil ());
          return;
        }
    }

  if CONSP (next)
    { last_boundary = prev; }

  /* Keep additional undo data, if it fits in the limits.  */
  while CONSP (next)
    {
      let elt: Lisp_Object = XCAR (next);

      /* When we get to a boundary, decide whether to truncate
	 either before or after it.  The lower threshold, undo_limit,
	 tells us to truncate after it.  If its size pushes past
	 the higher threshold undo_strong_limit, we truncate before it.  */
      if NILP (elt)
        {
          if size_so_far > (*undo_strong_limit.as_mut_ptr ()) as intmax_t
            { break; }
          last_boundary = prev;
          if size_so_far > (*undo_limit.as_mut_ptr ()) as intmax_t
            { break; }
        }

      /* Add in the space occupied by this element and its chain link.  */
      size_so_far += std::mem::size_of::<Lisp_Cons> () as intmax_t;
      if CONSP (elt)
        {
          size_so_far += std::mem::size_of::<Lisp_Cons> () as intmax_t;
          if STRINGP (XCAR (elt))
            {
              size_so_far += (std::mem::size_of::<Lisp_String> () - 1
                              + SCHARS (XCAR (elt)) as usize) as intmax_t;
            }
        }

      /* Advance to next element.  */
      prev = next;
      next = XCDR (next);
    }

  /* If we scanned the whole list, it is short enough; don't change it.  */
  if NILP (next)
    { }
  /* Truncate at the boundary where we decided to truncate.  */
  else if ! NILP (last_boundary)
    { XSETCDR (last_boundary, Qnil ()); }
  /* There's nothing we decided to keep, so clear it out.  */
  else
    { bset_undo_list (b, Qnil ()); }

  unbind_to (count, Qnil ());

  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_undo ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  DEFSYM! (Qinhibit_read_only, "inhibit-read-only");
  DEFSYM! (Qundo_auto__last_boundary_cause, "undo-auto--last-boundary-cause");
  DEFSYM! (Qexplicit, "explicit");

  /* Marker for function call undo list elements.  */
  DEFSYM! (Qapply, "apply");

  (*pending_boundary.as_mut_ptr ()) = Qnil ();
  staticpro (pending_boundary.as_ptr ());

  defsubr (Sundo_boundary.as_mut_ptr ());

  DEFVAR_INT! (c"undo-limit", undo_limit,
               "Keep no more undo information once it exceeds this size.\nThis limit is applied when garbage collection happens.\nWhen a previous command increases the total undo list size past this\nvalue, the earlier commands that came before it are forgotten.\n\nThe size is counted as the number of bytes occupied,\nwhich includes both saved text and other data.");
  (*undo_limit.as_mut_ptr ()) = 160000;

  DEFVAR_INT! (c"undo-strong-limit", undo_strong_limit,
               "Don't keep more than this much size of undo information.\nThis limit is applied when garbage collection happens.\nWhen a previous command increases the total undo list size past this\nvalue, that command and the earlier commands that came before it are forgotten.\nHowever, the most recent buffer-modifying command's undo info\nis never discarded for this reason.\n\nThe size is counted as the number of bytes occupied,\nwhich includes both saved text and other data.");
  (*undo_strong_limit.as_mut_ptr ()) = 240000;

  DEFVAR_LISP! (c"undo-outer-limit", Vundo_outer_limit,
                "Outer limit on size of undo information for one command.\nAt garbage collection time, if the current command has produced\nmore than this much undo information, it discards the info and displays\na warning.  This is a last-ditch limit to prevent memory overflow.\n\nThe size is counted as the number of bytes occupied, which includes\nboth saved text and other data.  A value of nil means no limit.  In\nthis case, accumulating one huge undo entry could make Emacs crash as\na result of memory overflow.\n\nIn fact, this calls the function which is the value of\n`undo-outer-limit-function' with one argument, the size.\nThe text above describes the behavior of the function\nthat variable usually specifies.");
  (*Vundo_outer_limit.as_mut_ptr ()) = make_fixnum (24000000);

  DEFVAR_LISP! (c"undo-outer-limit-function", Vundo_outer_limit_function,
                "Function to call when an undo list exceeds `undo-outer-limit'.\nThis function is called with one argument, the current undo list size\nfor the most recent command (since the last undo boundary).\nIf the function returns t, that means truncation has been fully handled.\nIf it returns nil, the other forms of truncation are done.\n\nGarbage collection is inhibited around the call to this function,\nso it must make sure not to do a lot of consing.");
  (*Vundo_outer_limit_function.as_mut_ptr ()) = Qnil ();

  DEFVAR_BOOL! (c"undo-inhibit-record-point", undo_inhibit_record_point,
                "Non-nil means do not record `point' in `buffer-undo-list'.");
  (*undo_inhibit_record_point.as_mut_ptr ()) = false;

  }
}
