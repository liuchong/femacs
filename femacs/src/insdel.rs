// femacs: port of src/insdel.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct, struct field, file-scope object, local), same control flow, same error
// model.  The one `DEFUN` uses `DEFUN!` (PORTING.md 5.1).  C's non-`static` definitions
// (`move_gap_both`, `adjust_markers_for_delete`, `adjust_markers_bytepos`,
// `buffer_overflow`, `make_gap`, `make_gap_1`, `copy_text`, `insert`, `insert_and_inherit`,
// `insert_char`, `insert_string`, `insert_before_markers`,
// `insert_before_markers_and_inherit`, `insert_1_both`, `insert_from_string`,
// `insert_from_string_before_markers`, `insert_from_gap_1`, `insert_from_gap`,
// `insert_from_buffer`, `adjust_after_insert`, `replace_range`, `replace_range_2`,
// `del_range`, `safe_del_range`, `del_range_1`, `del_range_byte`, `del_range_both`,
// `del_range_2`, `modify_text`, `prepare_to_modify_buffer_1`, `prepare_to_modify_buffer`,
// `invalidate_buffer_caches`, `signal_after_change`, `syms_of_insdel`) carry
// `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn`
// (`gap_left`, `gap_right`, `adjust_suspend_auto_hscroll`, `adjust_markers_for_insert`,
// `adjust_point`, `adjust_markers_for_replace`, `count_bytes`, `make_gap_larger`,
// `insert_from_string_1`, `insert_from_buffer_1`, `adjust_after_replace`,
// `safe_del_range_1`, `safe_del_range_2`, `run_undoable_change`, `reset_var_on_error`,
// `signal_before_change`, `Fcombine_after_change_execute_1`), and the two static objects
// (`combine_after_change_list`, `combine_after_change_buffer`) are unexported `static mut`s.
//
// **Dead regions are recorded in `docs/ports/insdel.md`, not transcribed** (P-08):
// `src/insdel.c` is in `docs/baseline/config/cond-eval-verified.txt` and
// `python3 tools/check-cond-eval.py src/insdel.c` reports
// `24 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the frozen configuration's
// branch selection is the clang-verified one.  The dead text is C 34-35 (`HAVE_TREE_SITTER`
// is undefined: `treesit.h` is not included, and all twelve `treesit_linecol_maybe` /
// `treesit_record_change` blocks with it), C 63-83 (`MARKER_DEBUG` is undefined, so
// `check_markers` is C 86's empty statement macro), C 514-574 and the `#else` arm at
// C 597-599 (`USE_MMAP_FOR_BUFFERS`, `REL_ALLOC` and `DOUG_LEA_MALLOC` are all undefined,
// so `make_gap_smaller` is not compiled), and C 769-877 plus the eight
// `#ifdef BYTE_COMBINING_DEBUG` blocks (`count_combining_before`/`count_combining_after`
// and every call of them).  The `live` basis in `docs/ports/line-drift-tolerance.tsv` is
// declared for that reason.
//
// **The five statement/expression macros at C 2260-2282** (`PRESERVE_VALUE`,
// `RESTORE_VALUE`, `PRESERVE_START_END`, `FETCH_START`, `FETCH_END`) are written on locals
// of their caller (`preserve_ptr`, `preserve_marker`, `start_marker`, `end_marker`, `start`,
// `end`), and a `macro_rules!` cannot name a caller's locals (PORTING.md 3.8, the marker.c
// `CONSIDER` precedent): each becomes an `unsafe fn` of the same name whose parameters are
// the names the C macro captures, so every call site keeps C's call shape and the function
// body carries C's tokens.  `check_markers` (C 86) keeps C's own macro shape.
//
// C 41-44's and C 59's forward declarations are dropped (PORTING.md 0.1-12: Rust items are
// order-independent) and the C locals that C declares and then assigns in a branch are
// zero-initialised (PORTING.md 1.5) -- both are recorded in `docs/ports/insdel.md`.
//
// Names no mirror carries are declared at their use site (the terminal.rs/filelock.rs
// shape), each with C's own prototype above it.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut)]

include!("include/config.rs");                 // C 22: `#include <config.h>`
include!("include/lisp.rs");                   // C 24: `#include "lisp.h"`
include!("include/composite.rs");              // C 25: `#include "composite.h"`
include!("include/intervals.rs");              // C 26: `#include "intervals.h"`
include!("include/character.rs");              // C 27: `#include "character.h"`
include!("include/itree.rs");                 // buffer.h 35: `#include "itree.h"`
include!("include/buffer.rs");                 // C 28: `#include "buffer.h"`
include!("include/window.rs");                 // C 29: `#include "window.h"`
include!("include/dispextern.rs");            // window.h 23: `#include "dispextern.h"`,
                                               // for window.rs 159 `text_cursor_kinds`
include!("include/region-cache.rs");            // C 30: `#include "region-cache.h"`
include!("include/pdumper.rs");                // C 31: `#include "pdumper.h"`
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (memcpy, memmove, memset, strlen)

/* C 23's `<intprops.h>` contributes no name this unit uses.  C 30's `"region-cache.h"` is the
   mirror's now (`include/region-cache.rs`, included above): its one prototype this unit calls
   (`invalidate_region_cache`) and `struct region_cache`'s ABI-equal alias come from there, and
   the declaration that stood below went away with the mirror (PORTING.md 0.1-2/0.1-10).
   C 34-35's `"treesit.h"` is dead (HAVE_TREE_SITTER is undefined).  */

/* The names C gets from headers whose mirrors do not carry them yet (the terminal.rs shape),
   each with C's own prototype in a `/// C:` line.  */
unsafe extern "C" {
  /// C: lisp.h 4226: `extern ptrdiff_t chars_in_text (const unsigned char *, ptrdiff_t);`
  fn chars_in_text (string: *const u8, nbytes: ptrdiff_t) -> ptrdiff_t;
  /// C: lisp.h 4954: `extern void clear_charpos_cache (struct buffer *);`
  fn clear_charpos_cache (b: *mut buffer);
  /// C: lisp.h 4957: `extern void detach_marker (Lisp_Object);`
  fn detach_marker (marker: Lisp_Object);
  /// C: lisp.h 4811: `extern Lisp_Object internal_catch_all (Lisp_Object (*) (void *), void *,
  /// Lisp_Object (*) (enum nonlocal_exit, Lisp_Object));`
  fn internal_catch_all (func: Option<unsafe extern "C" fn (*mut c_void) -> Lisp_Object>,
                         arg: *mut c_void,
                         handler: Option<unsafe extern "C" fn (nonlocal_exit, Lisp_Object)
                                          -> Lisp_Object>) -> Lisp_Object;
  /// C: lisp.h 4996-4997: `extern void update_search_regs (ptrdiff_t oldstart,
  /// ptrdiff_t oldend, ptrdiff_t newend);`
  fn update_search_regs (oldstart: ptrdiff_t, oldend: ptrdiff_t, newend: ptrdiff_t);
  /// C: lisp.h 5040-5041: `extern ptrdiff_t find_newline_no_quit (ptrdiff_t, ptrdiff_t,
  /// ptrdiff_t, ptrdiff_t *);`
  fn find_newline_no_quit (from: ptrdiff_t, from_byte: ptrdiff_t, cnt: ptrdiff_t,
                           bytepos: *mut ptrdiff_t) -> ptrdiff_t;
  /// C: lisp.h 4927-4928: `extern Lisp_Object make_buffer_string_both (ptrdiff_t, ptrdiff_t,
  /// ptrdiff_t, ptrdiff_t, bool);`
  fn make_buffer_string_both (start: ptrdiff_t, start_byte: ptrdiff_t, end: ptrdiff_t,
                              end_byte: ptrdiff_t, props: bool) -> Lisp_Object;
  /// C: lisp.h 4939: `extern void adjust_overlays_for_insert (ptrdiff_t, ptrdiff_t, bool);`
  fn adjust_overlays_for_insert (pos: ptrdiff_t, length: ptrdiff_t, before_markers: bool);
  /// C: lisp.h 4940: `extern void adjust_overlays_for_delete (ptrdiff_t, ptrdiff_t);`
  fn adjust_overlays_for_delete (pos: ptrdiff_t, length: ptrdiff_t);
  /// C: lisp.h 4942-4943: `extern void report_overlay_modification (Lisp_Object,
  /// Lisp_Object, bool, Lisp_Object, Lisp_Object, Lisp_Object);`
  fn report_overlay_modification (start: Lisp_Object, end: Lisp_Object, after: bool,
                                  arg1: Lisp_Object, arg2: Lisp_Object, arg3: Lisp_Object);
  /// C: lisp.h 5261: `extern void report_interval_modification (Lisp_Object, Lisp_Object);`
  fn report_interval_modification (start: Lisp_Object, end: Lisp_Object);
  /// C: window.h 1169: `extern void bset_redisplay (struct buffer *b);`
  fn bset_redisplay (b: *mut buffer);
}

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
// port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
// defines it here (status §3.29, the casetab.rs shape).
#[unsafe(no_mangle)]
pub static mut Vcombine_after_change_calls: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut inhibit_modification_hooks: bool = false;

/* List of elements of the form (BEG-UNCHANGED END-UNCHANGED CHANGE-AMOUNT)
   describing changes which happened while combine_after_change_calls
   was non-nil.  We use this to decide how to call them
   once the deferral ends.

   In each element.
   BEG-UNCHANGED is the number of chars before the changed range.
   END-UNCHANGED is the number of chars after the changed range,
   and CHANGE-AMOUNT is the number of characters inserted by the change
   (negative for a deletion).  */
static mut combine_after_change_list: Lisp_Object = lisp_h_Qnil ();

/* Buffer which combine_after_change_list is about.  */
static mut combine_after_change_buffer: Lisp_Object = lisp_h_Qnil ();

/* Also used in marker.c to enable expensive marker checks.  */

/* C 63-83's `#ifdef MARKER_DEBUG` arm (the debug `check_markers`) is dead in the frozen
   configuration (MARKER_DEBUG is undefined), so C 86's no-op macro is the live text; the
   dead function is recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

macro_rules! check_markers { () => { {} } }

/* Move gap to byte position BYTEPOS, which is also char position CHARPOS.
   Note that this can quit!  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn move_gap_both (charpos: ptrdiff_t, bytepos: ptrdiff_t)
{
  eassert! (charpos == BYTE_TO_CHAR (bytepos)
	    && bytepos == CHAR_TO_BYTE (charpos));
  if bytepos < GPT_BYTE! ()
    { gap_left (charpos, bytepos, false); }
  else if bytepos > GPT_BYTE! ()
    { gap_right (charpos, bytepos); }
}

/* Move the gap to a position less than the current GPT.
   BYTEPOS describes the new position as a byte position,
   and CHARPOS is the corresponding char position.
   If NEWGAP, then don't update beg_unchanged and end_unchanged.  */

unsafe fn gap_left (charpos: ptrdiff_t, bytepos: ptrdiff_t, newgap: bool)
{
  let mut charpos = charpos;
  let mut bytepos = bytepos;
  let mut to: *mut u8;
  let mut from: *mut u8;
  let mut i: ptrdiff_t;
  let mut new_s1: ptrdiff_t;

  if !newgap
    { BUF_COMPUTE_UNCHANGED ((current_buffer! ()), charpos, GPT! ()); }

  i = GPT_BYTE! ();
  to = GAP_END_ADDR! ();
  from = GPT_ADDR! ();
  new_s1 = GPT_BYTE! (); /* May point in the middle of multibyte sequences.  */

  /* Now copy the characters.  To move the gap down,
     copy characters up.  */

  while true
    {
      /* I gets number of characters left to copy.  */
      i = new_s1 - bytepos;
      if i == 0
	{ break; }
      /* If a quit is requested, stop copying now.
	 Change BYTEPOS to be where we have actually moved the gap to.
	 Note that this cannot happen when we are called to make the
	 gap larger or smaller, since make_gap_larger and
	 make_gap_smaller set inhibit-quit.  */
      if QUITP ()
	{
          /* FIXME: This can point in the middle of a multibyte character.  */
	  bytepos = new_s1;
	  charpos = BYTE_TO_CHAR (bytepos);
	  break;
	}
      /* Move at most 32000 chars before checking again for a quit.  */
      /* FIXME: This 32KB chunk size dates back to before 1991.
         Maybe we should bump it to reflect the >1000x increase
         in memory size and bandwidth since that time.
         Is it even worthwhile checking `quit` within this loop?
         Especially since make_gap_smaller/larger binds inhibit-quit anyway!  */
      if i > 32000
	{ i = 32000; }
      new_s1 -= i;
      from = from.offset (-i); to = to.offset (-i);
      memmove (to as *mut c_void, from as *const c_void, i as usize);
    }

  /* Adjust buffer data structure, to put the gap at BYTEPOS.
     BYTEPOS is where the loop above stopped, which may be what
     was specified or may be where a quit was detected.  */
  GPT_BYTE! () = bytepos;
  GPT! () = charpos;
  eassert! (charpos <= bytepos);
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; } /* Put an anchor.  */
  maybe_quit ();
}

/* Move the gap to a position greater than the current GPT.
   BYTEPOS describes the new position as a byte position,
   and CHARPOS is the corresponding char position.  */

unsafe fn gap_right (charpos: ptrdiff_t, bytepos: ptrdiff_t)
{
  let mut charpos = charpos;
  let mut bytepos = bytepos;
  let mut to: *mut u8;
  let mut from: *mut u8;
  let mut i: ptrdiff_t;
  let mut new_s1: ptrdiff_t; /* May point in the middle of multibyte sequences.  */

  BUF_COMPUTE_UNCHANGED ((current_buffer! ()), charpos, GPT! ());

  i = GPT_BYTE! ();
  from = GAP_END_ADDR! ();
  to = GPT_ADDR! ();
  new_s1 = GPT_BYTE! ();

  /* Now copy the characters.  To move the gap up,
     copy characters down.  */

  while true
    {
      /* I gets number of characters left to copy.  */
      i = bytepos - new_s1;
      if i == 0
	{ break; }
      /* If a quit is requested, stop copying now.
	 Change BYTEPOS to be where we have actually moved the gap to.
	 Note that this cannot happen when we are called to make the
	 gap larger or smaller, since make_gap_larger and
	 make_gap_smaller set inhibit-quit.  */
      if QUITP ()
	{
          /* FIXME: This can point in the middle of a multibyte character.  */
	  bytepos = new_s1;
	  charpos = BYTE_TO_CHAR (bytepos);
	  break;
	}
      /* Move at most 32000 chars before checking again for a quit.  */
      if i > 32000
	{ i = 32000; }
      new_s1 += i;
      memmove (to as *mut c_void, from as *const c_void, i as usize);
      from = from.offset (i); to = to.offset (i);
    }

  GPT! () = charpos;
  GPT_BYTE! () = bytepos;
  eassert! (charpos <= bytepos);
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; } /* Put an anchor.  */
  maybe_quit ();
}

/* If the selected window's old pointm is adjacent or covered by the
   region from FROM to TO, unsuspend auto hscroll in that window.  */

unsafe fn adjust_suspend_auto_hscroll (from: ptrdiff_t, to: ptrdiff_t)
{
  if WINDOWP (selected_window)
    {
      let w: *mut window = XWINDOW (selected_window);

      if BUFFERP ((*w).contents)
	  && XBUFFER ((*w).contents) == (current_buffer! ())
	  && (*XMARKER ((*w).old_pointm)).charpos >= from
	  && (*XMARKER ((*w).old_pointm)).charpos <= to
	{ (*w).bits &= ! (1 << WINDOW_SUSPEND_AUTO_HSCROLL_SHIFT); }
    }
}


/* Adjust all markers for a deletion
   whose range in bytes is FROM_BYTE to TO_BYTE.
   The range in charpos is FROM to TO.

   This function assumes that the gap is adjacent to
   or inside of the range being deleted.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adjust_markers_for_delete (from: ptrdiff_t, from_byte: ptrdiff_t,
						    to: ptrdiff_t, to_byte: ptrdiff_t)
{
  let mut m: *mut Lisp_Marker;
  let mut charpos: ptrdiff_t;

  adjust_suspend_auto_hscroll (from, to);
  m = BUF_MARKERS! ((current_buffer! ()));
  while !m.is_null ()
    {
      charpos = (*m).charpos;
      eassert! (charpos <= Z! ());

      /* If the marker is after the deletion,
	 relocate by number of chars / bytes deleted.  */
      if charpos > to
	{
	  (*m).charpos -= to - from;
	  (*m).bytepos -= to_byte - from_byte;
	}
      /* Here's the case where a marker is inside text being deleted.  */
      else if charpos > from
	{
	  (*m).charpos = from;
	  (*m).bytepos = from_byte;
	}
      m = (*m).next;
    }
  adjust_overlays_for_delete (from, to - from);
}


/* Adjust markers for an insertion that stretches from FROM / FROM_BYTE
   to TO / TO_BYTE.  We have to relocate the charpos of every marker
   that points after the insertion (but not their bytepos).

   When a marker points at the insertion point,
   we advance it if either its insertion-type is t
   or BEFORE_MARKERS is true.  */

unsafe fn adjust_markers_for_insert (from: ptrdiff_t, from_byte: ptrdiff_t,
				     to: ptrdiff_t, to_byte: ptrdiff_t,
				     before_markers: bool)
{
  let mut m: *mut Lisp_Marker;
  let nchars: ptrdiff_t = to - from;
  let nbytes: ptrdiff_t = to_byte - from_byte;

  adjust_suspend_auto_hscroll (from, to);
  m = BUF_MARKERS! ((current_buffer! ()));
  while !m.is_null ()
    {
      eassert! ((*m).bytepos >= (*m).charpos
		&& (*m).bytepos - (*m).charpos <= Z_BYTE! () - Z! ());

      if (*m).bytepos == from_byte
	{
	  if MARKER_INSERTION_TYPE_P (m) || before_markers
	    {
	      (*m).bytepos = to_byte;
	      (*m).charpos = to;
	    }
	}
      else if (*m).bytepos > from_byte
	{
	  (*m).bytepos += nbytes;
	  (*m).charpos += nchars;
	}
      m = (*m).next;
    }
  adjust_overlays_for_insert (from, to - from, before_markers);
}

/* Adjust point for an insertion of NBYTES bytes, which are NCHARS characters.

   This is used only when the value of point changes due to an insert
   or delete; it does not represent a conceptual change in point as a
   marker.  In particular, point is not crossing any interval
   boundaries, so there's no need to use the usual SET_PT macro.  In
   fact it would be incorrect to do so, because either the old or the
   new value of point is out of sync with the current set of
   intervals.  */

unsafe fn adjust_point (nchars: ptrdiff_t, nbytes: ptrdiff_t)
{
  SET_BUF_PT_BOTH ((current_buffer! ()), PT! () + nchars, PT_BYTE! () + nbytes);
  /* In a single-byte buffer, the two positions must be equal.  */
  eassert! (PT_BYTE! () >= PT! () && PT_BYTE! () - PT! () <= ZV_BYTE! () - ZV! ());
}


/* Adjust markers for a replacement of a text at FROM (FROM_BYTE) of
   length OLD_CHARS (OLD_BYTES) to a new text of length NEW_CHARS
   (NEW_BYTES).  It is assumed that OLD_CHARS > 0, i.e., this is not
   an insertion.  */

unsafe fn adjust_markers_for_replace (from: ptrdiff_t, from_byte: ptrdiff_t,
				      old_chars: ptrdiff_t, old_bytes: ptrdiff_t,
				      new_chars: ptrdiff_t, new_bytes: ptrdiff_t)
{
  let mut m: *mut Lisp_Marker;
  let prev_to_byte: ptrdiff_t = from_byte + old_bytes;
  let diff_chars: ptrdiff_t = new_chars - old_chars;
  let diff_bytes: ptrdiff_t = new_bytes - old_bytes;

  if old_chars == 0
    {
      /* Just an insertion: markers at FROM may need to move or not depending
	 on their marker type.  Delegate this special case to
	 'adjust_markers_for_insert' so the loop below can remain oblivious
	 to marker types.  */
      adjust_markers_for_insert (from, from_byte,
				 from + new_chars, from_byte + new_bytes,
				 false);
      return;
    }

  adjust_suspend_auto_hscroll (from, from + old_chars);

  m = BUF_MARKERS! ((current_buffer! ()));
  while !m.is_null ()
    {
      if (*m).bytepos >= prev_to_byte
	{
	  (*m).charpos += diff_chars;
	  (*m).bytepos += diff_bytes;
	}
      else if (*m).bytepos > from_byte
	{
	  (*m).charpos = from;
	  (*m).bytepos = from_byte;
	}
      m = (*m).next;
    }

  check_markers! ();

  adjust_overlays_for_insert (from + old_chars, new_chars, true);
  adjust_overlays_for_delete (from, old_chars);
}

/* Starting at POS (BYTEPOS), find the byte position corresponding to
   ENDPOS, which could be either before or after POS.  */
unsafe fn count_bytes (pos: ptrdiff_t, bytepos: ptrdiff_t, endpos: ptrdiff_t) -> ptrdiff_t
{
  let mut pos = pos;
  let mut bytepos = bytepos;

  eassert! (BEG_BYTE as ptrdiff_t <= bytepos && bytepos <= Z_BYTE! ()
	    && BEG as ptrdiff_t <= endpos && endpos <= Z! ());

  if pos <= endpos
    { while pos < endpos { bytepos += next_char_len (bytepos) as ptrdiff_t; pos += 1; } }
  else
    { while pos > endpos { bytepos -= prev_char_len (bytepos) as ptrdiff_t; pos -= 1; } }

  return bytepos;
}

/* Adjust byte positions of markers when their character positions
   didn't change.  This is used in several places that replace text,
   but keep the character positions of the markers unchanged -- the
   byte positions could still change due to different numbers of bytes
   in the new text.

   FROM (FROM_BYTE) and TO (TO_BYTE) specify the region of text where
   changes have been done.  TO_Z, if non-zero, means all the markers
   whose positions are after TO should also be adjusted.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn adjust_markers_bytepos (from: ptrdiff_t, from_byte: ptrdiff_t,
						 to: ptrdiff_t, to_byte: ptrdiff_t,
						 to_z: c_int)
{
  let mut m: *mut Lisp_Marker;
  let mut beg: ptrdiff_t = from; let mut begbyte: ptrdiff_t = from_byte;

  adjust_suspend_auto_hscroll (from, to);

  if Z! () == Z_BYTE! () || (to_z == 0 && to == to_byte)
    {
      /* Make sure each affected marker's bytepos is equal to
	 its charpos.  */
      m = BUF_MARKERS! ((current_buffer! ()));
      while !m.is_null ()
	{
	  if (*m).bytepos > from_byte
	      && (to_z != 0 || (*m).bytepos <= to_byte)
	    { (*m).bytepos = (*m).charpos; }
	  m = (*m).next;
	}
    }
  else
    {
      m = BUF_MARKERS! ((current_buffer! ()));
      while !m.is_null ()
	{
	  /* Recompute each affected marker's bytepos.  */
	  if (*m).bytepos > from_byte
	      && (to_z != 0 || (*m).bytepos <= to_byte)
	    {
	      if (*m).charpos < beg
		  && beg - (*m).charpos > (*m).charpos - from
		{
		  beg = from;
		  begbyte = from_byte;
		}
	      (*m).bytepos = count_bytes (beg, begbyte, (*m).charpos);
	      beg = (*m).charpos;
	      begbyte = (*m).bytepos;
	    }
	  m = (*m).next;
	}
    }

  /* Make sure cached charpos/bytepos is invalid.  */
  clear_charpos_cache ((current_buffer! ()));
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn buffer_overflow ()
{
  error (c"Maximum buffer size exceeded".as_ptr ());
}

/* Make the gap NBYTES_ADDED bytes longer.  */

unsafe fn make_gap_larger (nbytes_added: ptrdiff_t)
{
  let mut nbytes_added = nbytes_added;
  let mut tem: Lisp_Object;
  let mut real_gap_loc: ptrdiff_t;
  let mut real_gap_loc_byte: ptrdiff_t;
  let mut old_gap_size: ptrdiff_t;
  let current_size: ptrdiff_t = Z_BYTE! () - BEG_BYTE as ptrdiff_t + GAP_SIZE! ();

  if BUF_BYTES_MAX - current_size < nbytes_added
    { buffer_overflow (); }

  /* If we have to get more space, get enough to last a while;
     but do not exceed the maximum buffer size.  */
  nbytes_added = min (nbytes_added + GAP_BYTES_DFL as ptrdiff_t,
		      BUF_BYTES_MAX - current_size);

  enlarge_buffer_text ((current_buffer! ()), nbytes_added);

  /* Prevent quitting in gap_left.  We cannot allow a quit there,
     because that would leave the buffer text in an inconsistent
     state, with 2 gap holes instead of just one.  */
  tem = Vinhibit_quit;
  Vinhibit_quit = Qt ();

  real_gap_loc = GPT! ();
  real_gap_loc_byte = GPT_BYTE! ();
  old_gap_size = GAP_SIZE! ();

  /* Call the newly allocated space a gap at the end of the whole space.  */
  GPT! () = Z! () + GAP_SIZE! ();
  GPT_BYTE! () = Z_BYTE! () + GAP_SIZE! ();
  GAP_SIZE! () = nbytes_added;

  /* Move the new gap down to be consecutive with the end of the old one.  */
  gap_left (real_gap_loc + old_gap_size, real_gap_loc_byte + old_gap_size, true);

  /* Now combine the two into one large gap.  */
  GAP_SIZE! () += old_gap_size;
  GPT! () = real_gap_loc;
  GPT_BYTE! () = real_gap_loc_byte;

  /* Put an anchor.  */
  *Z_ADDR! () = 0;

  Vinhibit_quit = tem;
}

/* C 514-574's `static void make_gap_smaller (ptrdiff_t)` (under
   `#if defined USE_MMAP_FOR_BUFFERS || defined REL_ALLOC || defined DOUG_LEA_MALLOC`) is dead
   in the frozen configuration (all three are undefined, and `src/Makefile.in` builds this
   file for the minimal configuration), so it is recorded in docs/ports/insdel.md rather than
   transcribed (P-08).  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_gap (nbytes_added: ptrdiff_t)
{
  if nbytes_added >= 0
    /* With set-buffer-multibyte on a large buffer, we can end up growing the
     * buffer *many* times.  Avoid an O(N^2) behavior by increasing by an
     * amount at least proportional to the size of the buffer.
     * On my test (a 223.9MB zip file on a Thinkpad T61):
     * With /5    =>  24s
     * With /32   =>  25s
     * With /64   =>  26s
     * With /128  =>  28s
     * With /1024 =>  51s
     * With /4096 => 131s
     * With /∞    => gave up after 858s
     * Of course, ideally we should never call set-buffer-multibyte on
     * a non-empty buffer (e.g. use buffer-swap-text instead).
     * We chose /64 because it already brings almost the best performance while
     * limiting the potential wasted memory to 1.5%.  */
    { make_gap_larger (max (nbytes_added, (Z! () - BEG as ptrdiff_t) / 64)); }
  /* C 597-599's `#else make_gap_smaller (-nbytes_added);` arm is dead in the frozen
     configuration (USE_MMAP_FOR_BUFFERS, REL_ALLOC and DOUG_LEA_MALLOC are all undefined)
     and is recorded in docs/ports/insdel.md rather than transcribed (P-08).  */
}

/* Add NBYTES to B's gap.  It's enough to temporarily
   fake current_buffer and avoid real switch to B.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_gap_1 (b: *mut buffer, nbytes: ptrdiff_t)
{
  let oldb: *mut buffer = (current_buffer! ());

  set_current_buffer! (b);
  make_gap (nbytes);
  set_current_buffer! (oldb);
}

/* Copy NBYTES bytes of text from FROM_ADDR to TO_ADDR.
   FROM_MULTIBYTE says whether the incoming text is multibyte.
   TO_MULTIBYTE says whether to store the text as multibyte.
   If FROM_MULTIBYTE != TO_MULTIBYTE, we convert.

   Return the number of bytes stored at TO_ADDR.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn copy_text (from_addr: *const u8, to_addr: *mut u8,
				    nbytes: ptrdiff_t, from_multibyte: bool,
				    to_multibyte: bool) -> ptrdiff_t
{
  let mut to_addr = to_addr;
  let mut from_addr = from_addr;
  let mut nbytes = nbytes;

  if from_multibyte == to_multibyte
    {
      memcpy (to_addr as *mut c_void, from_addr as *const c_void, nbytes as usize);
      return nbytes;
    }
  else if from_multibyte
    {
      let mut nchars: ptrdiff_t = 0;
      let mut bytes_left: ptrdiff_t = nbytes;

      while bytes_left > 0
	{
	  let mut thislen: c_int = 0;
	  let mut c: c_int = string_char_and_length (from_addr, &raw mut thislen);
	  if ! ASCII_CHAR_P (c as EMACS_INT)
	    { c &= 0xFF; }
	  *to_addr = c as u8;
	  to_addr = to_addr.offset (1);
	  from_addr = from_addr.offset (thislen as ptrdiff_t);
	  bytes_left -= thislen as ptrdiff_t;
	  nchars += 1;
	}
      return nchars;
    }
  else
    {
      let initial_to_addr: *mut u8 = to_addr;

      /* Convert single-byte to multibyte.  */
      while nbytes > 0
	{
	  let mut c: c_int = *from_addr as c_int;
	  from_addr = from_addr.offset (1);

	  if !ASCII_CHAR_P (c as EMACS_INT)
	    {
	      c = BYTE8_TO_CHAR (c);
	      to_addr = to_addr.offset (CHAR_STRING (c, to_addr) as ptrdiff_t);
	      nbytes -= 1;
	    }
	  else
	    /* Special case for speed.  */
	    { *to_addr = c as u8; to_addr = to_addr.offset (1); nbytes -= 1; }
	}
      return to_addr.offset_from (initial_to_addr);
    }
}


/* Insert a string of specified length before point.
   This function judges multibyteness based on
   enable_multibyte_characters in the current buffer;
   it never converts between single-byte and multibyte.

   DO NOT use this for the contents of a Lisp string or a Lisp buffer!
   prepare_to_modify_buffer could relocate the text.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert (string: *const c_char, nbytes: ptrdiff_t)
{
  if nbytes > 0
    {
      let len: ptrdiff_t = chars_in_text (string as *const u8, nbytes);
      let mut opoint: ptrdiff_t;
      insert_1_both (string, len, nbytes, false, true, false);
      opoint = PT! () - len;
      signal_after_change (opoint, 0, len);
      update_compositions (opoint, PT! (), CHECK_BORDER);
    }
}

/* Likewise, but inherit text properties from neighboring characters.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_and_inherit (string: *const c_char, nbytes: ptrdiff_t)
{
  if nbytes > 0
    {
      let len: ptrdiff_t = chars_in_text (string as *const u8, nbytes);
      let mut opoint: ptrdiff_t;
      insert_1_both (string, len, nbytes, true, true, false);
      opoint = PT! () - len;
      signal_after_change (opoint, 0, len);
      update_compositions (opoint, PT! (), CHECK_BORDER);
    }
}

/* Insert the character C before point.  Do not inherit text properties.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_char (c: c_int)
{
  let mut str_: [u8; MAX_MULTIBYTE_LENGTH as usize] = [0; MAX_MULTIBYTE_LENGTH as usize];
  let mut len: c_int;

  if ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { len = CHAR_STRING (c, str_.as_mut_ptr ()); }
  else
    {
      len = 1;
      str_[0] = c as u8;
    }

  insert (str_.as_ptr () as *const c_char, len as ptrdiff_t);
}

/* Insert the null-terminated string S before point.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_string (s: *const c_char)
{
  insert (s, strlen (s) as ptrdiff_t);
}

/* Like `insert' except that all markers pointing at the place where
   the insertion happens are adjusted to point after it.
   Don't use this function to insert part of a Lisp string,
   since gc could happen and relocate it.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_before_markers (string: *const c_char, nbytes: ptrdiff_t)
{
  if nbytes > 0
    {
      let len: ptrdiff_t = chars_in_text (string as *const u8, nbytes);
      let mut opoint: ptrdiff_t;
      insert_1_both (string, len, nbytes, false, true, true);
      opoint = PT! () - len;
      signal_after_change (opoint, 0, len);
      update_compositions (opoint, PT! (), CHECK_BORDER);
    }
}

/* Likewise, but inherit text properties from neighboring characters.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_before_markers_and_inherit (string: *const c_char,
							    nbytes: ptrdiff_t)
{
  if nbytes > 0
    {
      let len: ptrdiff_t = chars_in_text (string as *const u8, nbytes);
      let mut opoint: ptrdiff_t;
      insert_1_both (string, len, nbytes, true, true, true);
      opoint = PT! () - len;
      signal_after_change (opoint, 0, len);
      update_compositions (opoint, PT! (), CHECK_BORDER);
    }
}

/* C 766-880's `#ifdef BYTE_COMBINING_DEBUG` arm (`count_combining_before` and
   `count_combining_after`) is dead in the frozen configuration and is recorded in
   docs/ports/insdel.md rather than transcribed (P-08).  */


/* Insert a sequence of NCHARS chars which occupy NBYTES bytes
   starting at STRING.  INHERIT non-zero means inherit the text
   properties from neighboring characters; zero means inserted text
   will have no text properties.  PREPARE non-zero means call
   prepare_to_modify_buffer, which checks that the region is not
   read-only, and calls before-change-function and any modification
   properties the text may have.  BEFORE_MARKERS non-zero means adjust
   all markers that point at the insertion place to point after it.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_1_both (string: *const c_char,
				        nchars: ptrdiff_t, nbytes: ptrdiff_t,
				        inherit: bool, prepare: bool,
				        before_markers: bool)
{
  let mut nchars = nchars;

  if nchars == 0
    { return; }

  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { nchars = nbytes; }

  /* C 901-904's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  if prepare
    /* Do this before moving and increasing the gap,
       because the before-change hooks might move the gap
       or make it smaller.  */
    { prepare_to_modify_buffer (PT! (), PT! (), std::ptr::null_mut ()); }

  if PT! () != GPT! ()
    { move_gap_both (PT! (), PT_BYTE! ()); }
  if GAP_SIZE! () < nbytes
    { make_gap (nbytes - GAP_SIZE! ()); }

  /* C 918-921's `#ifdef BYTE_COMBINING_DEBUG` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  /* Record deletion of the surrounding text that combines with
     the insertion.  This, together with recording the insertion,
     will add up to the right stuff in the undo list.  */
  record_insert (PT! (), nchars);
  modiff_incr (&raw mut MODIFF! (), nchars);
  CHARS_MODIFF! () = MODIFF! ();

  memcpy (GPT_ADDR! () as *mut c_void, string as *const c_void, nbytes as usize);

  GAP_SIZE! () -= nbytes;
  GPT! () += nchars;
  ZV! () += nchars;
  Z! () += nchars;
  GPT_BYTE! () += nbytes;
  ZV_BYTE! () += nbytes;
  Z_BYTE! () += nbytes;
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; } /* Put an anchor.  */

  eassert! (GPT! () <= GPT_BYTE! ());

  /* The insert may have been in the unchanged region, so check again.  */
  if Z! () - GPT! () < END_UNCHANGED! ()
    { END_UNCHANGED! () = Z! () - GPT! (); }

  adjust_markers_for_insert (PT! (), PT_BYTE! (),
			     PT! () + nchars, PT_BYTE! () + nbytes,
			     before_markers);

  offset_intervals ((current_buffer! ()), PT! (), nchars);

  if !inherit && !buffer_intervals ((current_buffer! ())).is_null ()
    { set_text_properties (make_fixnum (PT! ()), make_fixnum (PT! () + nchars),
			   Qnil (), Qnil (), Qnil ()); }

  /* C 958-963's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  adjust_point (nchars, nbytes);

  check_markers! ();
}


/* Insert the part of the text of STRING, a Lisp object assumed to be
   of type string, consisting of the LENGTH characters (LENGTH_BYTE bytes)
   starting at position POS / POS_BYTE.  If the text of STRING has properties,
   copy them into the buffer.

   It does not work to use `insert' for this, because a GC could happen
   before we copy the stuff into the buffer, and relocate the string
   without insert noticing.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_from_string (string: Lisp_Object, pos: ptrdiff_t,
					     pos_byte: ptrdiff_t, length: ptrdiff_t,
					     length_byte: ptrdiff_t, inherit: bool)
{
  let opoint: ptrdiff_t = PT! ();

  if SCHARS (string) == 0
    { return; }

  insert_from_string_1 (string, pos, pos_byte, length, length_byte,
			inherit, false);
  signal_after_change (opoint, 0, PT! () - opoint);
  update_compositions (opoint, PT! (), CHECK_BORDER);
}

/* Like `insert_from_string' except that all markers pointing
   at the place where the insertion happens are adjusted to point after it.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_from_string_before_markers (string: Lisp_Object,
							    pos: ptrdiff_t, pos_byte: ptrdiff_t,
							    length: ptrdiff_t,
							    length_byte: ptrdiff_t,
							    inherit: bool)
{
  let opoint: ptrdiff_t = PT! ();

  if SCHARS (string) == 0
    { return; }

  insert_from_string_1 (string, pos, pos_byte, length, length_byte,
			inherit, true);
  signal_after_change (opoint, 0, PT! () - opoint);
  update_compositions (opoint, PT! (), CHECK_BORDER);
}

/* Subroutine of the insertion functions above.  */

unsafe fn insert_from_string_1 (string: Lisp_Object, pos: ptrdiff_t, pos_byte: ptrdiff_t,
				nchars: ptrdiff_t, nbytes: ptrdiff_t,
				inherit: bool, before_markers: bool)
{
  let mut outgoing_nbytes: ptrdiff_t = nbytes;
  let mut intervals: INTERVAL = std::ptr::null_mut ();

  /* Make OUTGOING_NBYTES describe the text
     as it will be inserted in this buffer.  */

  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { outgoing_nbytes = nchars; }
  else if ! STRING_MULTIBYTE (string)
    { outgoing_nbytes = count_size_as_multibyte (SDATA (string).offset (pos_byte), nbytes); }

  /* C 1035-1038's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  /* Do this before moving and increasing the gap,
     because the before-change hooks might move the gap
     or make it smaller.  */
  prepare_to_modify_buffer (PT! (), PT! (), std::ptr::null_mut ());

  if PT! () != GPT! ()
    { move_gap_both (PT! (), PT_BYTE! ()); }
  if GAP_SIZE! () < outgoing_nbytes
    { make_gap (outgoing_nbytes - GAP_SIZE! ()); }

  /* Copy the string text into the buffer, perhaps converting
     between single-byte and multibyte.  */
  copy_text (SDATA (string).offset (pos_byte), GPT_ADDR! (), nbytes,
	     STRING_MULTIBYTE (string),
	     ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)));

  /* C 1057-1065's `#ifdef BYTE_COMBINING_DEBUG` arm is dead in the frozen configuration and
     is recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  record_insert (PT! (), nchars);
  modiff_incr (&raw mut MODIFF! (), nchars);
  CHARS_MODIFF! () = MODIFF! ();

  GAP_SIZE! () -= outgoing_nbytes;
  GPT! () += nchars;
  ZV! () += nchars;
  Z! () += nchars;
  GPT_BYTE! () += outgoing_nbytes;
  ZV_BYTE! () += outgoing_nbytes;
  Z_BYTE! () += outgoing_nbytes;
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; } /* Put an anchor.  */

  eassert! (GPT! () <= GPT_BYTE! ());

  /* The insert may have been in the unchanged region, so check again.  */
  if Z! () - GPT! () < END_UNCHANGED! ()
    { END_UNCHANGED! () = Z! () - GPT! (); }

  adjust_markers_for_insert (PT! (), PT_BYTE! (), PT! () + nchars,
			     PT_BYTE! () + outgoing_nbytes,
			     before_markers);

  offset_intervals ((current_buffer! ()), PT! (), nchars);

  intervals = string_intervals (string);
  /* Get the intervals for the part of the string we are inserting.  */
  if nbytes < SBYTES (string)
    { intervals = copy_intervals (intervals, pos, nchars); }

  /* Insert those intervals.  */
  graft_intervals_into_buffer (intervals, PT! (), nchars,
			       (current_buffer! ()), inherit);

  /* C 1102-1107's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  adjust_point (nchars, outgoing_nbytes);

  check_markers! ();
}


/* Insert a sequence of NCHARS chars which occupy NBYTES bytes
   starting at GAP_END_ADDR - NBYTES (if text_at_gap_tail) and at
   GPT_ADDR (if not text_at_gap_tail).
   Contrary to insert_from_gap, this does not invalidate any cache,
   nor update any markers, nor record any buffer modification information
   of any sort, with the single exception of notifying tree-sitter and
   updating tree-sitter linecol cache.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_from_gap_1 (nchars: ptrdiff_t, nbytes: ptrdiff_t,
					    text_at_gap_tail: bool)
{
  eassert! (if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
            { nchars == nbytes } else { nchars <= nbytes });

  /* C 1128-1132's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  GAP_SIZE! () -= nbytes;
  if ! text_at_gap_tail
    {
      GPT! () += nchars;
      GPT_BYTE! () += nbytes;
    }
  ZV! () += nchars;
  Z! () += nchars;
  ZV_BYTE! () += nbytes;
  Z_BYTE! () += nbytes;

  /* Put an anchor to ensure multi-byte form ends at gap.  */
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; }
  eassert! (GPT! () <= GPT_BYTE! ());

  /* C 1150-1155's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */
}

/* Insert a sequence of NCHARS chars which occupy NBYTES bytes
   starting at GAP_END_ADDR - NBYTES (if text_at_gap_tail) and at
   GPT_ADDR (if not text_at_gap_tail).

  If BEFORE_MARKERS is true, insert before markers.  At the moment only
  read_and_insert_process_output in process.c sets this to true.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_from_gap (nchars: ptrdiff_t, nbytes: ptrdiff_t,
					  text_at_gap_tail: bool, before_markers: bool)
{
  let mut nchars = nchars;
  let ins_charpos: ptrdiff_t = GPT! (); let ins_bytepos: ptrdiff_t = GPT_BYTE! ();

  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { nchars = nbytes; }

  /* No need to call prepare_to_modify_buffer, since this is called
     from places that replace some region with a different text, so
     prepare_to_modify_buffer was already called by the deletion part
     of this dance.  */
  invalidate_buffer_caches ((current_buffer! ()), GPT! (), GPT! ());
  record_insert (GPT! (), nchars);
  modiff_incr (&raw mut MODIFF! (), nchars);
  CHARS_MODIFF! () = MODIFF! ();

  insert_from_gap_1 (nchars, nbytes, text_at_gap_tail);

  adjust_markers_for_insert (ins_charpos, ins_bytepos,
			     ins_charpos + nchars, ins_bytepos + nbytes,
			     before_markers);

  if !buffer_intervals ((current_buffer! ())).is_null ()
    {
      offset_intervals ((current_buffer! ()), ins_charpos, nchars);
      graft_intervals_into_buffer (std::ptr::null_mut (), ins_charpos, nchars,
				   (current_buffer! ()), false);
    }

  if ins_charpos < PT! ()
    { adjust_point (nchars, nbytes); }

  check_markers! ();
}


/* Insert text from BUF, NCHARS characters starting at CHARPOS, into the
   current buffer.  If the text in BUF has properties, they are absorbed
   into the current buffer.

   It does not work to use `insert' for this, because a malloc could happen
   and relocate BUF's text before the copy happens.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn insert_from_buffer (buf: *mut buffer,
					     charpos: ptrdiff_t, nchars: ptrdiff_t,
					     inherit: bool)
{
  let opoint: ptrdiff_t = PT! ();

  /* C 1216-1220's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  insert_from_buffer_1 (buf, charpos, nchars, inherit);
  signal_after_change (opoint, 0, PT! () - opoint);
  update_compositions (opoint, PT! (), CHECK_BORDER);

  /* C 1227-1233's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */
}

/* NOTE: If we ever make insert_from_buffer_1 public, make sure to
   move the call to treesit_record_change into it.  */

unsafe fn insert_from_buffer_1 (buf: *mut buffer,
				from: ptrdiff_t, nchars: ptrdiff_t, inherit: bool)
{
  let mut from = from;
  let mut chunk: ptrdiff_t = 0; let mut chunk_expanded: ptrdiff_t = 0;
  let from_byte: ptrdiff_t = buf_charpos_to_bytepos (buf as *mut c_void, from);
  let to_byte: ptrdiff_t = buf_charpos_to_bytepos (buf as *mut c_void, from + nchars);
  let incoming_nbytes: ptrdiff_t = to_byte - from_byte;
  let mut outgoing_nbytes: ptrdiff_t = incoming_nbytes;
  let mut intervals: INTERVAL = std::ptr::null_mut ();

  if nchars == 0
    { return; }

  /* Make OUTGOING_NBYTES describe the text
     as it will be inserted in this buffer.  */

  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { outgoing_nbytes = nchars; }
  else if NILP (BVAR! (buf, enable_multibyte_characters_))
    {
      let mut outgoing_before_gap: ptrdiff_t = 0;
      let mut outgoing_after_gap: ptrdiff_t = 0;

      if from < BUF_GPT! (buf)
	{
	  chunk =  BUF_GPT_BYTE! (buf) - from_byte;
	  if chunk > incoming_nbytes
	    { chunk = incoming_nbytes; }
	  outgoing_before_gap
	    = count_size_as_multibyte (BUF_BYTE_ADDRESS (buf, from_byte),
				       chunk);
	}
      else
	{ chunk = 0; }

      if chunk < incoming_nbytes
	{ outgoing_after_gap
	  = count_size_as_multibyte (BUF_BYTE_ADDRESS (buf,
						       from_byte + chunk),
				     incoming_nbytes - chunk); }

      outgoing_nbytes = outgoing_before_gap + outgoing_after_gap;
    }

  /* Do this before moving and increasing the gap,
     because the before-change hooks might move the gap
     or make it smaller.  */
  prepare_to_modify_buffer (PT! (), PT! (), std::ptr::null_mut ());

  if PT! () != GPT! ()
    { move_gap_both (PT! (), PT_BYTE! ()); }
  if GAP_SIZE! () < outgoing_nbytes
    { make_gap (outgoing_nbytes - GAP_SIZE! ()); }

  if from < BUF_GPT! (buf)
    {
      chunk = BUF_GPT_BYTE! (buf) - from_byte;
      if chunk > incoming_nbytes
	{ chunk = incoming_nbytes; }
      /* Record number of output bytes, so we know where
	 to put the output from the second copy_text.  */
      chunk_expanded
	= copy_text (BUF_BYTE_ADDRESS (buf, from_byte),
		     GPT_ADDR! (), chunk,
		     ! NILP (BVAR! (buf, enable_multibyte_characters_)),
		     ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)));
    }
  else
    { chunk = 0; chunk_expanded = 0; }

  if chunk < incoming_nbytes
    { copy_text (BUF_BYTE_ADDRESS (buf, from_byte + chunk),
		 GPT_ADDR! ().offset (chunk_expanded), incoming_nbytes - chunk,
		 ! NILP (BVAR! (buf, enable_multibyte_characters_)),
		 ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))); }

  /* C 1317-1325's `#ifdef BYTE_COMBINING_DEBUG` arm is dead in the frozen configuration and
     is recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  record_insert (PT! (), nchars);
  modiff_incr (&raw mut MODIFF! (), nchars);
  CHARS_MODIFF! () = MODIFF! ();

  GAP_SIZE! () -= outgoing_nbytes;
  GPT! () += nchars;
  ZV! () += nchars;
  Z! () += nchars;
  GPT_BYTE! () += outgoing_nbytes;
  ZV_BYTE! () += outgoing_nbytes;
  Z_BYTE! () += outgoing_nbytes;
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; } /* Put an anchor.  */

  eassert! (GPT! () <= GPT_BYTE! ());

  /* The insert may have been in the unchanged region, so check again.  */
  if Z! () - GPT! () < END_UNCHANGED! ()
    { END_UNCHANGED! () = Z! () - GPT! (); }

  adjust_markers_for_insert (PT! (), PT_BYTE! (), PT! () + nchars,
			     PT_BYTE! () + outgoing_nbytes,
			     false);

  offset_intervals ((current_buffer! ()), PT! (), nchars);

  /* Get the intervals for the part of the string we are inserting.  */
  intervals = buffer_intervals (buf);
  if nchars < BUF_Z! (buf) - BUF_BEG (buf)
    {
      if buf == (current_buffer! ()) && PT! () <= from
	{ from += nchars; }
      intervals = copy_intervals (intervals, from, nchars);
    }

  /* Insert those intervals.  */
  graft_intervals_into_buffer (intervals, PT! (), nchars, (current_buffer! ()), inherit);

  adjust_point (nchars, outgoing_nbytes);
}


/* Record undo information and adjust markers and position keepers for
   a replacement of a text PREV_TEXT at FROM to a new text of LEN
   chars (LEN_BYTE bytes) which resides in the gap just after
   GPT_ADDR.

   PREV_TEXT nil means the new text was just inserted.  */

unsafe fn adjust_after_replace (from: ptrdiff_t, from_byte: ptrdiff_t,
				prev_text: Lisp_Object, len: ptrdiff_t,
				len_byte: ptrdiff_t)
{
  let mut nchars_del: ptrdiff_t = 0; let mut nbytes_del: ptrdiff_t = 0;

  /* C 1381-1384's `#ifdef BYTE_COMBINING_DEBUG` arm is dead in the frozen configuration and
     is recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  if STRINGP (prev_text)
    {
      nchars_del = SCHARS (prev_text);
      nbytes_del = SBYTES (prev_text);
    }

  /* Update various buffer positions for the new text.  */
  GAP_SIZE! () -= len_byte;
  ZV! () += len; Z! () += len;
  ZV_BYTE! () += len_byte; Z_BYTE! () += len_byte;
  GPT! () += len; GPT_BYTE! () += len_byte;
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; } /* Put an anchor.  */

  if nchars_del > 0
    { adjust_markers_for_replace (from, from_byte, nchars_del, nbytes_del,
				  len, len_byte); }
  else
    { adjust_markers_for_insert (from, from_byte,
				 from + len, from_byte + len_byte, false); }

  if nchars_del > 0
    { record_delete (from, prev_text, false); }
  record_insert (from, len);

  offset_intervals ((current_buffer! ()), from, len - nchars_del);

  if from < PT! ()
    { adjust_point (len - nchars_del, len_byte - nbytes_del); }

  /* As byte combining will decrease Z, we must check this again.  */
  if Z! () - GPT! () < END_UNCHANGED! ()
    { END_UNCHANGED! () = Z! () - GPT! (); }

  check_markers! ();

  modiff_incr (&raw mut MODIFF! (), nchars_del + len);
  CHARS_MODIFF! () = MODIFF! ();
}

/* Record undo information, adjust markers and position keepers for an
   insertion of a text from FROM (FROM_BYTE) to TO (TO_BYTE).  The
   text already exists in the current buffer but character length (TO
   - FROM) may be incorrect, the correct length is NEWLEN.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adjust_after_insert (from: ptrdiff_t, from_byte: ptrdiff_t,
					      to: ptrdiff_t, to_byte: ptrdiff_t,
					      newlen: ptrdiff_t)
{
  let len: ptrdiff_t = to - from; let len_byte: ptrdiff_t = to_byte - from_byte;

  if GPT! () != to
    { move_gap_both (to, to_byte); }
  GAP_SIZE! () += len_byte;
  GPT! () -= len; GPT_BYTE! () -= len_byte;
  ZV! () -= len; ZV_BYTE! () -= len_byte;
  Z! () -= len; Z_BYTE! () -= len_byte;
  adjust_after_replace (from, from_byte, Qnil (), newlen, len_byte);
}


/* Replace the text from character positions FROM to TO with the
   replacement text NEW.  NEW could either be a string, a buffer, or
   a vector [BUFFER BEG END], where BUFFER is the buffer with the replacement
   text and BEG and END are buffer positions in BUFFER that give the
   replacement text beginning and end.
   If PREPARE, call prepare_to_modify_buffer.
   If INHERIT, the newly inserted text should inherit text properties
   from the surrounding non-deleted text.
   If ADJUST_MATCH_DATA, then adjust the match data before calling
   signal_after_change.  */

/* Note that this does not yet handle markers quite right.
   Also it needs to record a single undo-entry that does a replacement
   rather than a separate delete and insert.
   That way, undo will also handle markers properly.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn replace_range (from: ptrdiff_t, to: ptrdiff_t, new: Lisp_Object,
					run_mod_hooks: bool, inherit: bool,
					adjust_match_data: bool)
{
  let mut from = from;
  let mut to = to;
  let mut inschars: ptrdiff_t;
  let mut insbeg: ptrdiff_t;
  let mut insbuf: *mut buffer;
  if STRINGP (new)
    {
      insbuf = std::ptr::null_mut ();
      insbeg = 0;
      inschars = SCHARS (new);
    }
  else if BUFFERP (new)
    {
      insbuf = XBUFFER (new);
      insbeg = BUF_BEGV (insbuf);
      inschars = BUF_ZV (insbuf) - insbeg;
    }
  else
    {
      CHECK_VECTOR (new);
      /* Let `Faref' signal an error if it's too small.  */
      let insend: Lisp_Object = Faref (new, make_fixnum (2));
      CHECK_BUFFER (AREF (new, 0));
      CHECK_FIXNUM (AREF (new, 1));
      CHECK_FIXNUM (insend);
      insbuf = XBUFFER (AREF (new, 0));
      insbeg = XFIXNUM (AREF (new, 1));
      inschars = XFIXNUM (insend) - insbeg;
    }
  let mut from_byte: ptrdiff_t; let mut to_byte: ptrdiff_t;
  let mut nbytes_del: ptrdiff_t; let mut nchars_del: ptrdiff_t;
  let mut deletion: Lisp_Object;

  check_markers! ();

  deletion = Qnil ();

  if run_mod_hooks
    {
      let range_length: ptrdiff_t = to - from;
      prepare_to_modify_buffer (from, to, &raw mut from);
      to = from + range_length;
    }

  /* Make args be valid.  */
  if from < BEGV! ()
    { from = BEGV! (); }
  if to > ZV! ()
    { to = ZV! (); }

  from_byte = CHAR_TO_BYTE (from);
  to_byte = CHAR_TO_BYTE (to);

  nchars_del = to - from;
  nbytes_del = to_byte - from_byte;

  if nbytes_del <= 0 && inschars == 0
    { return; }

  /* C 1524-1530's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  let mut insbeg_bytes: ptrdiff_t; let mut insend_bytes: ptrdiff_t;
  let mut insbytes: ptrdiff_t;
  let mut insbeg_ptr: *mut u8;
  let mut new_is_multibyte: bool;
  if insbuf.is_null ()
    {
      new_is_multibyte = STRING_MULTIBYTE (new);
      insbytes = SBYTES (new);
      insbeg_ptr = SDATA (new);
    }
  else
    {
      new_is_multibyte = !NILP (BVAR! (insbuf, enable_multibyte_characters_));
      let insend: ptrdiff_t = insbeg + inschars;
      if new_is_multibyte
	{
	  insbeg_bytes = buf_charpos_to_bytepos (insbuf as *mut c_void, insbeg);
	  insend_bytes = buf_charpos_to_bytepos (insbuf as *mut c_void, insend);
	}
      else
	{
	  insbeg_bytes = insbeg;
	  insend_bytes = insend;
	}
      insbytes = insend_bytes - insbeg_bytes;
      /* Move gap out of the replacement text, to arrange for
         replacement text to be contiguous in the source buffer, so that
         we could copy it in one go.  */
      if (*(*insbuf).text).gpt_byte > insbeg_bytes
	  && (*(*insbuf).text).gpt_byte < insend_bytes
	{
	  let old: *mut buffer = (current_buffer! ());
	  if insbuf != old
	    { set_buffer_internal (insbuf); }
	  move_gap_both (insbeg, insbeg_bytes);
	  if insbuf != old
	    { set_buffer_internal (old); }
	}
      insbeg_ptr = BUF_BYTE_ADDRESS (insbuf, insbeg_bytes);
      eassert! ((*(*insbuf).text).gpt_byte <= insbeg_bytes
		|| (*(*insbuf).text).gpt_byte >= insend_bytes);
    }
  let mut outgoing_insbytes: ptrdiff_t = insbytes;

  /* Make OUTGOING_INSBYTES describe the text
     as it will be inserted in this buffer.  */

  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { outgoing_insbytes = inschars; }
  else if ! new_is_multibyte
    { outgoing_insbytes = count_size_as_multibyte (insbeg_ptr, insbytes); }

  /* Make sure the gap is somewhere in or next to what we are deleting.  */
  if from > GPT! ()
    { gap_right (from, from_byte); }
  if to < GPT! ()
    { gap_left (to, to_byte, false); }

  if ! EQ (BVAR! ((current_buffer! ()), undo_list_), Qt ())
    { deletion = make_buffer_string_both (from, from_byte, to, to_byte, true); }

  GAP_SIZE! () += nbytes_del;
  ZV! () -= nchars_del;
  Z! () -= nchars_del;
  ZV_BYTE! () -= nbytes_del;
  Z_BYTE! () -= nbytes_del;
  GPT! () = from;
  GPT_BYTE! () = from_byte;
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; } /* Put an anchor.  */

  eassert! (GPT! () <= GPT_BYTE! ());

  if GPT! () - (BEG as ptrdiff_t) < BEG_UNCHANGED! ()
    { BEG_UNCHANGED! () = GPT! () - BEG as ptrdiff_t; }
  if Z! () - GPT! () < END_UNCHANGED! ()
    { END_UNCHANGED! () = Z! () - GPT! (); }

  if GAP_SIZE! () < outgoing_insbytes
    { make_gap (outgoing_insbytes - GAP_SIZE! ()); }

  /* Copy the string text into the buffer, perhaps converting
     between single-byte and multibyte.  */
  copy_text (insbeg_ptr, GPT_ADDR! (), insbytes,
	     new_is_multibyte,
	     ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)));

  /* C 1621-1629's `#ifdef BYTE_COMBINING_DEBUG` arm is dead in the frozen configuration and
     is recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  /* Record the insertion first, so that when we undo,
     the deletion will be undone first.  Thus, undo
     will insert before deleting, and thus will keep
     the markers before and after this text separate.  */
  if !NILP (deletion)
    {
      record_insert (from + SCHARS (deletion), inschars);
      record_delete (from, deletion, false);
    }

  GAP_SIZE! () -= outgoing_insbytes;
  GPT! () += inschars;
  ZV! () += inschars;
  Z! () += inschars;
  GPT_BYTE! () += outgoing_insbytes;
  ZV_BYTE! () += outgoing_insbytes;
  Z_BYTE! () += outgoing_insbytes;
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; } /* Put an anchor.  */

  eassert! (GPT! () <= GPT_BYTE! ());

  /* Adjust markers for the deletion and the insertion.  */
  adjust_markers_for_replace (from, from_byte, nchars_del, nbytes_del,
			      inschars, outgoing_insbytes);

  offset_intervals ((current_buffer! ()), from, inschars - nchars_del);

  /* Get the intervals for the part of the string we are inserting--
     not including the combined-before bytes.  */
  let intervals: INTERVAL
    = (if insbuf.is_null () { string_intervals (new) }
       else { copy_intervals (buffer_intervals (insbuf), insbeg, inschars) });

  /* Insert those intervals.  */
  graft_intervals_into_buffer (intervals, from, inschars,
			       (current_buffer! ()), inherit);

  /* C 1669-1675's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  /* Relocate point as if it were a marker.  */
  if from < PT! ()
      /* Mimic 'insert' when FROM==TO==PT).  */
      || PT! () == to
    { adjust_point ((from + inschars - min (PT! (), to)),
		    (from_byte + outgoing_insbytes - min (PT_BYTE! (), to_byte))); }

  check_markers! ();

  modiff_incr (&raw mut MODIFF! (), nchars_del + inschars);
  CHARS_MODIFF! () = MODIFF! ();

  if adjust_match_data
    { update_search_regs (from, to, from + inschars); }

  if run_mod_hooks
    {
      signal_after_change (from, nchars_del, GPT! () - from);
      update_compositions (from, GPT! (), CHECK_BORDER);
    }
}


/* Replace the text from character positions FROM to TO with
   the text in INS of length INSCHARS.
   Keep the text properties that applied to the old characters
   (extending them to all the new chars if there are more new chars).

   Note that this does not yet handle markers quite right.

   If MARKERS, relocate markers.

   Unlike most functions at this level, never call
   prepare_to_modify_buffer and never call signal_after_change.
   Because this function is called in a loop, one character at a time.
   The caller of 'replace_range_2' calls these hooks for the entire
   region once.  Apart from signal_after_change, any caller of this
   function should also call treesit_record_change.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn replace_range_2 (from: ptrdiff_t, from_byte: ptrdiff_t,
					  to: ptrdiff_t, to_byte: ptrdiff_t,
					  ins: *const c_char, inschars: ptrdiff_t,
					  insbytes: ptrdiff_t, markers: bool)
{
  let mut nbytes_del: ptrdiff_t; let mut nchars_del: ptrdiff_t;

  check_markers! ();

  nchars_del = to - from;
  nbytes_del = to_byte - from_byte;

  if nbytes_del <= 0 && insbytes == 0
    { return; }

  /* Make sure the gap is somewhere in or next to what we are deleting.  */
  if from > GPT! ()
    { gap_right (from, from_byte); }
  if to < GPT! ()
    { gap_left (to, to_byte, false); }

  GAP_SIZE! () += nbytes_del;
  ZV! () -= nchars_del;
  Z! () -= nchars_del;
  ZV_BYTE! () -= nbytes_del;
  Z_BYTE! () -= nbytes_del;
  GPT! () = from;
  GPT_BYTE! () = from_byte;
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; } /* Put an anchor.  */

  eassert! (GPT! () <= GPT_BYTE! ());

  if GPT! () - (BEG as ptrdiff_t) < BEG_UNCHANGED! ()
    { BEG_UNCHANGED! () = GPT! () - BEG as ptrdiff_t; }
  if Z! () - GPT! () < END_UNCHANGED! ()
    { END_UNCHANGED! () = Z! () - GPT! (); }

  if GAP_SIZE! () < insbytes
    { make_gap (insbytes - GAP_SIZE! ()); }

  /* Copy the replacement text into the buffer.  */
  memcpy (GPT_ADDR! () as *mut c_void, ins as *const c_void, insbytes as usize);

  /* C 1760-1768's `#ifdef BYTE_COMBINING_DEBUG` arm is dead in the frozen configuration and
     is recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  GAP_SIZE! () -= insbytes;
  GPT! () += inschars;
  ZV! () += inschars;
  Z! () += inschars;
  GPT_BYTE! () += insbytes;
  ZV_BYTE! () += insbytes;
  Z_BYTE! () += insbytes;
  if GAP_SIZE! () > 0 { *GPT_ADDR! () = 0; } /* Put an anchor.  */

  eassert! (GPT! () <= GPT_BYTE! ());

  /* Adjust markers for the deletion and the insertion.  */
  if ! (nchars_del == 1 && inschars == 1 && nbytes_del == insbytes)
    {
      if markers
	{ adjust_markers_for_replace (from, from_byte, nchars_del, nbytes_del,
				      inschars, insbytes); }
      else
	{
	  /* The character positions of the markers remain intact, but
	     we still need to update their byte positions, because the
	     deleted and the inserted text might have multibyte
	     sequences which make the original byte positions of the
	     markers invalid.  */
	  adjust_markers_bytepos (from, from_byte, from + inschars,
				  from_byte + insbytes, true as c_int);
	}
    }

  offset_intervals ((current_buffer! ()), from, inschars - nchars_del);

  /* Relocate point as if it were a marker.  */
  if from < PT! () && (nchars_del != inschars || nbytes_del != insbytes)
    {
      if PT! () < to
	/* PT was within the deleted text.  Move it to FROM.  */
	{ adjust_point (from - PT! (), from_byte - PT_BYTE! ()); }
      else
	{ adjust_point (inschars - nchars_del, insbytes - nbytes_del); }
    }

  check_markers! ();

  modiff_incr (&raw mut MODIFF! (), nchars_del + inschars);
  CHARS_MODIFF! () = MODIFF! ();
}


/* Delete characters in current buffer
   from FROM up to (but not including) TO.
   If TO comes before FROM, we delete nothing.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn del_range (from: ptrdiff_t, to: ptrdiff_t)
{
  del_range_1 (from, to, true, false);
}

#[repr(C)]
pub struct safe_del_range_context
{
  /* From and to positions.  */
  pub from: ptrdiff_t,
  pub to: ptrdiff_t,
}

unsafe extern "C" fn safe_del_range_1 (ptr: *mut c_void) -> Lisp_Object
{
  let mut context: *mut safe_del_range_context;

  context = ptr as *mut safe_del_range_context;
  del_range ((*context).from, (*context).to);
  return Qnil ();
}

unsafe extern "C" fn safe_del_range_2 (type_: nonlocal_exit, value: Lisp_Object) -> Lisp_Object
{
  return Qt ();
}

/* Like del_range; however, catch all non-local exits.  Value is 0 if
   the buffer contents were really deleted.  Otherwise, it is 1.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_del_range (from: ptrdiff_t, to: ptrdiff_t) -> c_int
{
  let mut context: safe_del_range_context
    = safe_del_range_context { from: 0, to: 0 };

  context.from = from;
  context.to = to;

  return !NILP (internal_catch_all (Some (safe_del_range_1),
				    &raw mut context as *mut c_void,
				    Some (safe_del_range_2))) as c_int;
}

/* Like del_range; PREPARE says whether to call prepare_to_modify_buffer.
   RET_STRING says to return the deleted text. */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn del_range_1 (from: ptrdiff_t, to: ptrdiff_t, prepare: bool,
				      ret_string: bool) -> Lisp_Object
{
  let mut from = from;
  let mut to = to;
  let mut from_byte: ptrdiff_t; let mut to_byte: ptrdiff_t;
  let mut deletion: Lisp_Object;

  /* Make args be valid */
  if from < BEGV! ()
    { from = BEGV! (); }
  if to > ZV! ()
    { to = ZV! (); }

  if to <= from
    { return Qnil (); }

  if prepare
    {
      let range_length: ptrdiff_t = to - from;
      prepare_to_modify_buffer (from, to, &raw mut from);
      to = min (ZV! (), from + range_length);
    }

  from_byte = CHAR_TO_BYTE (from);
  to_byte = CHAR_TO_BYTE (to);

  deletion = del_range_2 (from, from_byte, to, to_byte, ret_string);
  signal_after_change (from, to - from, 0);
  update_compositions (from, from, CHECK_HEAD);
  return deletion;
}

/* Like del_range_1 but args are byte positions, not char positions.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn del_range_byte (from_byte: ptrdiff_t, to_byte: ptrdiff_t)
{
  let mut from_byte = from_byte;
  let mut to_byte = to_byte;
  let mut from: ptrdiff_t; let mut to: ptrdiff_t;

  /* Make args be valid.  */
  if from_byte < BEGV_BYTE! ()
    { from_byte = BEGV_BYTE! (); }
  if to_byte > ZV_BYTE! ()
    { to_byte = ZV_BYTE! (); }

  if to_byte <= from_byte
    { return; }

  from = BYTE_TO_CHAR (from_byte);
  to = BYTE_TO_CHAR (to_byte);

  {
    let old_from: ptrdiff_t = from; let old_to: ptrdiff_t = Z! () - to;
    let range_length: ptrdiff_t = to - from;
    prepare_to_modify_buffer (from, to, &raw mut from);
    to = from + range_length;

    if old_from != from
      { from_byte = CHAR_TO_BYTE (from); }
    if to > ZV! ()
      {
	to = ZV! ();
	to_byte = ZV_BYTE! ();
      }
    else if old_to == Z! () - to
      { to_byte = CHAR_TO_BYTE (to); }
  }

  del_range_2 (from, from_byte, to, to_byte, false);
  signal_after_change (from, to - from, 0);
  update_compositions (from, from, CHECK_HEAD);
}

/* Like del_range_1, but positions are specified both as charpos
   and bytepos.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn del_range_both (from: ptrdiff_t, from_byte: ptrdiff_t,
					 to: ptrdiff_t, to_byte: ptrdiff_t,
					 prepare: bool)
{
  let mut from = from; let mut to = to;
  let mut from_byte = from_byte; let mut to_byte = to_byte;

  /* Make args be valid */
  if from_byte < BEGV_BYTE! ()
    { from_byte = BEGV_BYTE! (); }
  if to_byte > ZV_BYTE! ()
    { to_byte = ZV_BYTE! (); }

  if to_byte <= from_byte
    { return; }

  if from < BEGV! ()
    { from = BEGV! (); }
  if to > ZV! ()
    { to = ZV! (); }

  if prepare
    {
      let old_from: ptrdiff_t = from; let old_to: ptrdiff_t = Z! () - to;
      let range_length: ptrdiff_t = to - from;
      prepare_to_modify_buffer (from, to, &raw mut from);
      to = from + range_length;

      if old_from != from
	{ from_byte = CHAR_TO_BYTE (from); }
      if to > ZV! ()
	{
	  to = ZV! ();
	  to_byte = ZV_BYTE! ();
	}
      else if old_to == Z! () - to
	{ to_byte = CHAR_TO_BYTE (to); }
    }

  del_range_2 (from, from_byte, to, to_byte, false);
  signal_after_change (from, to - from, 0);
  update_compositions (from, from, CHECK_HEAD);
}

/* Delete a range of text, specified both as character positions
   and byte positions.  FROM and TO are character positions,
   while FROM_BYTE and TO_BYTE are byte positions.
   If RET_STRING, the deleted area is returned as a string.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn del_range_2 (from: ptrdiff_t, from_byte: ptrdiff_t,
				      to: ptrdiff_t, to_byte: ptrdiff_t,
				      ret_string: bool) -> Lisp_Object
{
  let mut nbytes_del: ptrdiff_t; let mut nchars_del: ptrdiff_t;
  let mut deletion: Lisp_Object;

  check_markers! ();

  nchars_del = to - from;
  nbytes_del = to_byte - from_byte;

  /* C 2002-2008's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  /* Make sure the gap is somewhere in or next to what we are deleting.  */
  if from > GPT! ()
    { gap_right (from, from_byte); }
  if to < GPT! ()
    { gap_left (to, to_byte, false); }

  /* C 2017-2020's `#ifdef BYTE_COMBINING_DEBUG` arm is dead in the frozen configuration and
     is recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  if ret_string || ! EQ (BVAR! ((current_buffer! ()), undo_list_), Qt ())
    { deletion = make_buffer_string_both (from, from_byte, to, to_byte, true); }
  else
    { deletion = Qnil (); }

  /* Record marker adjustments, and text deletion into undo
     history.  */
  record_delete (from, deletion, true);

  /* Relocate all markers pointing into the new, larger gap to point
     at the end of the text before the gap.  */
  adjust_markers_for_delete (from, from_byte, to, to_byte);

  modiff_incr (&raw mut MODIFF! (), nchars_del);
  CHARS_MODIFF! () = MODIFF! ();

  /* Relocate point as if it were a marker.  */
  if from < PT! ()
    { adjust_point (from - min (PT! (), to),
		    from_byte - min (PT_BYTE! (), to_byte)); }

  offset_intervals ((current_buffer! ()), from, - nchars_del);

  GAP_SIZE! () += nbytes_del;
  ZV! () -= nchars_del;
  Z! () -= nchars_del;
  ZV_BYTE! () -= nbytes_del;
  Z_BYTE! () -= nbytes_del;
  GPT! () = from;
  GPT_BYTE! () = from_byte;
  if GAP_SIZE! () > 0 && (*(*(current_buffer! ())).text).bits & BUFFER_TEXT_INHIBIT_SHRINKING == 0
    /* Put an anchor, unless called from decode_coding_object which
       needs to access the previous gap contents.  */
    { *GPT_ADDR! () = 0; }

  eassert! (GPT! () <= GPT_BYTE! ());

  if GPT! () - (BEG as ptrdiff_t) < BEG_UNCHANGED! ()
    { BEG_UNCHANGED! () = GPT! () - BEG as ptrdiff_t; }
  if Z! () - GPT! () < END_UNCHANGED! ()
    { END_UNCHANGED! () = Z! () - GPT! (); }

  check_markers! ();

  /* C 2067-2071's `#ifdef HAVE_TREE_SITTER` arm is dead in the frozen configuration and is
     recorded in docs/ports/insdel.md rather than transcribed (P-08).  */

  return deletion;
}


/* Call this if you're about to change the text of current buffer
   from character positions START to END.  This checks the read-only
   properties of the region, calls the necessary modification hooks,
   and warns the next redisplay that it should pay attention to that
   area.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn modify_text (start: ptrdiff_t, end: ptrdiff_t)
{
  prepare_to_modify_buffer (start, end, std::ptr::null_mut ());

  BUF_COMPUTE_UNCHANGED ((current_buffer! ()), start - 1, end);
  if MODIFF! () <= SAVE_MODIFF! ()
    { record_first_change (); }
  modiff_incr (&raw mut MODIFF! (), end - start);
  CHARS_MODIFF! () = MODIFF! ();

  bset_point_before_scroll ((current_buffer! ()), Qnil ());
}

/* Signal that we are about to make a change that may result in new
   undo information.
 */
unsafe fn run_undoable_change ()
{
  if EQ (BVAR! ((current_buffer! ()), undo_list_), Qt ())
    { return; }

  call0 (Qundo_auto__undoable_change ());
}

/* Check that it is okay to modify the buffer between START and END,
   which are char positions.

   Run the before-change-function, if any.  If intervals are in use,
   verify that the text to be modified is not read-only, and call
   any modification properties the text may have.

   If PRESERVE_PTR is nonzero, we relocate *PRESERVE_PTR
   by holding its value temporarily in a marker.

   This function runs Lisp, which means it can GC, which means it can
   compact buffers, including the current buffer being worked on here.
   So don't you dare calling this function while manipulating the gap,
   or during some other similar "critical section".  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_to_modify_buffer_1 (start: ptrdiff_t, end: ptrdiff_t,
						     preserve_ptr: *mut ptrdiff_t)
{
  let mut base_buffer: *mut buffer;
  let mut temp: Lisp_Object;

  XSETFASTINT! (temp, start);
  if !NILP (BVAR! ((current_buffer! ()), read_only_))
    { Fbarf_if_buffer_read_only (temp); }

  /* If we're about to modify a buffer the contents of which come from
     a dump file, copy the contents to private storage first so we
     don't take a COW fault on the buffer text and keep it around
     forever.  */
  if pdumper_object_p (BEG_ADDR! () as *const c_void)
    { enlarge_buffer_text ((current_buffer! ()), 0); }
  eassert! (! pdumper_object_p (BEG_ADDR! () as *const c_void));

  run_undoable_change();

  bset_redisplay ((current_buffer! ()));

  if !buffer_intervals ((current_buffer! ())).is_null ()
    {
      if !preserve_ptr.is_null ()
	{
	  let mut preserve_marker: Lisp_Object;
	  preserve_marker = Fcopy_marker (make_fixnum (*preserve_ptr), Qnil ());
	  verify_interval_modification ((current_buffer! ()), start, end);
	  *preserve_ptr = marker_position (preserve_marker);
	  unchain_marker (XMARKER (preserve_marker) as *mut c_void);
	}
      else
	{ verify_interval_modification ((current_buffer! ()), start, end); }
    }

  /* For indirect buffers, use the base buffer to check clashes.  */
  if !(*(current_buffer! ())).base_buffer.is_null ()
    { base_buffer = (*(current_buffer! ())).base_buffer; }
  else
    { base_buffer = (current_buffer! ()); }

  if inhibit_modification_hooks
    { return; }

  if !NILP (BVAR! (base_buffer, file_truename_))
      /* Make binding buffer-file-name to nil effective.  */
      && !NILP (BVAR! (base_buffer, filename_))
      && SAVE_MODIFF! () >= MODIFF! ()
    { Flock_file (BVAR! (base_buffer, file_truename_)); }

  /* If `select-active-regions' is non-nil, save the region text.  */
  /* FIXME: Move this to Elisp (via before-change-functions).  */
  if !NILP (BVAR! ((current_buffer! ()), mark_active_))
      && !(*XMARKER (BVAR! ((current_buffer! ()), mark_))).buffer.is_null ()
      && NILP (Vsaved_region_selection)
      && (if EQ (Vselect_active_regions, Qonly ())
	  { EQ (CAR_SAFE (Vtransient_mark_mode), Qonly ()) }
	  else
	  { (!NILP (Vselect_active_regions)
	     && !NILP (Vtransient_mark_mode)) })
    { Vsaved_region_selection
      = calln! (Vregion_extract_function, Qnil ()); }

  signal_before_change (start, end, preserve_ptr);
  Fset (Qdeactivate_mark (), Qt ());
}

/* Like above, but called when we know that the buffer text
   will be modified and region caches should be invalidated.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn prepare_to_modify_buffer (start: ptrdiff_t, end: ptrdiff_t,
						   preserve_ptr: *mut ptrdiff_t)
{
  prepare_to_modify_buffer_1 (start, end, preserve_ptr);
  invalidate_buffer_caches ((current_buffer! ()), start, end);
}

/* Invalidate the caches maintained by the buffer BUF, if any, for the
   region between buffer positions START and END.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn invalidate_buffer_caches (buf: *mut buffer, start: ptrdiff_t,
						   end: ptrdiff_t)
{
  let mut buf = buf;
  let mut start = start;

  /* Indirect buffers usually have their caches set to NULL, but we
     need to consider the caches of their base buffer.  */
  if !(*buf).base_buffer.is_null ()
    { buf = (*buf).base_buffer; }
  /* The bidi_paragraph_cache must be invalidated first, because doing
     so might need to use the newline_cache (via find_newline_no_quit,
     see below).  */
  if !(*buf).bidi_paragraph_cache.is_null ()
    {
      if start > BUF_BEG (buf)
	{
	  /* If we are deleting or replacing characters, we could
	     create a paragraph start, because all of the characters
	     from START to the beginning of START's line are
	     whitespace.  Therefore, we must extend the region to be
	     invalidated up to the newline before START.  Similarly,
	     if we are inserting characters immediately after a
	     newline, we could create a paragraph start if the
	     inserted characters start with a newline.  */
	  let mut line_beg: ptrdiff_t = start;
	  let mut start_byte: ptrdiff_t = buf_charpos_to_bytepos (buf as *mut c_void, start);
	  let prev_char: c_int = BUF_FETCH_BYTE (buf, start_byte - 1) as c_int;

	  if (start == end) == (prev_char == '\n' as c_int)
	    {
	      let old: *mut buffer = (current_buffer! ());

	      set_buffer_internal (buf);

	      line_beg = find_newline_no_quit (start, start_byte, -1,
					       &raw mut start_byte);
	      set_buffer_internal (old);
	    }
	  start = line_beg - (line_beg > BUF_BEG (buf)) as ptrdiff_t;
	}
      invalidate_region_cache (buf,
			       (*buf).bidi_paragraph_cache,
			       start - BUF_BEG (buf), BUF_Z! (buf) - end);
    }
  if !(*buf).newline_cache.is_null ()
    { invalidate_region_cache (buf,
			       (*buf).newline_cache,
			       start - BUF_BEG (buf), BUF_Z! (buf) - end); }
  if !(*buf).width_run_cache.is_null ()
    { invalidate_region_cache (buf,
			       (*buf).width_run_cache,
			       start - BUF_BEG (buf), BUF_Z! (buf) - end); }
}

/* These macros work with an argument named `preserve_ptr'
   and a local variable named `preserve_marker'.  */

/* C: `#define PRESERVE_VALUE							\
     if (preserve_ptr && NILP (preserve_marker))				\
       preserve_marker = Fcopy_marker (make_fixnum (*preserve_ptr), Qnil)`.
   A `macro_rules!` cannot name its caller's locals (PORTING.md 3.8), so the macro is this
   function; the call sites pass the two locals C's macro captures.  */
unsafe fn PRESERVE_VALUE (preserve_ptr: *mut ptrdiff_t, preserve_marker: *mut Lisp_Object)
{
  if ! preserve_ptr.is_null () && NILP (*preserve_marker)
    { *preserve_marker = Fcopy_marker (make_fixnum (*preserve_ptr), Qnil ()); }
}

/* C: `#define RESTORE_VALUE						\
     if (! NILP (preserve_marker))					\
       {								\
         *preserve_ptr = marker_position (preserve_marker);		\
         unchain_marker (XMARKER (preserve_marker));		\
       }`  */
unsafe fn RESTORE_VALUE (preserve_ptr: *mut ptrdiff_t, preserve_marker: *mut Lisp_Object)
{
  if ! NILP (*preserve_marker)
    {
      *preserve_ptr = marker_position (*preserve_marker);
      unchain_marker (XMARKER (*preserve_marker) as *mut c_void);
    }
}

/* C: `#define PRESERVE_START_END			\
     if (NILP (start_marker))			\
       start_marker = Fcopy_marker (start, Qnil);	\
     if (NILP (end_marker))			\
       end_marker = Fcopy_marker (end, Qnil);`  */
unsafe fn PRESERVE_START_END (start_marker: *mut Lisp_Object, end_marker: *mut Lisp_Object,
			      start: Lisp_Object, end: Lisp_Object)
{
  if NILP (*start_marker)
    { *start_marker = Fcopy_marker (start, Qnil ()); }
  if NILP (*end_marker)
    { *end_marker = Fcopy_marker (end, Qnil ()); }
}

/* C: `#define FETCH_START				\
     (! NILP (start_marker) ? Fmarker_position (start_marker) : start)`  */
unsafe fn FETCH_START (start_marker: Lisp_Object, start: Lisp_Object) -> Lisp_Object
{
  return if ! NILP (start_marker) { Fmarker_position (start_marker) } else { start };
}

/* C: `#define FETCH_END				\
     (! NILP (end_marker) ? Fmarker_position (end_marker) : end)`  */
unsafe fn FETCH_END (end_marker: Lisp_Object, end: Lisp_Object) -> Lisp_Object
{
  return if ! NILP (end_marker) { Fmarker_position (end_marker) } else { end };
}

/* Set a variable to nil if an error occurred.
   Don't change the variable if there was no error.
   VAL is a cons-cell (VARIABLE . NO-ERROR-FLAG).
   VARIABLE is the variable to maybe set to nil.
   NO-ERROR-FLAG is nil if there was an error,
   anything else meaning no error (so this function does nothing).  */
#[repr(C)]
pub struct rvoe_arg
{
  pub location: *mut Lisp_Object,
  pub errorp: bool,
}

unsafe extern "C" fn reset_var_on_error (ptr: *mut c_void)
{
  let p: *mut rvoe_arg = ptr as *mut rvoe_arg;
  if (*p).errorp
    { *(*p).location = Qnil (); }
}

/* Signal a change to the buffer immediately before it happens.
   START_INT and END_INT are the bounds of the text to be changed.

   If PRESERVE_PTR is nonzero, we relocate *PRESERVE_PTR
   by holding its value temporarily in a marker.  */

unsafe fn signal_before_change (start_int: ptrdiff_t, end_int: ptrdiff_t,
				preserve_ptr: *mut ptrdiff_t)
{
  let mut start: Lisp_Object; let mut end: Lisp_Object;
  let mut start_marker: Lisp_Object; let mut end_marker: Lisp_Object;
  let mut preserve_marker: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut rvoe_arg: rvoe_arg = rvoe_arg { location: std::ptr::null_mut (), errorp: false };

  start = make_fixnum (start_int);
  end = make_fixnum (end_int);
  preserve_marker = Qnil ();
  start_marker = Qnil ();
  end_marker = Qnil ();

  specbind (Qinhibit_modification_hooks (), Qt ());

  /* If buffer is unmodified, run a special hook for that case.  The
     check for Vfirst_change_hook is just a minor optimization.  */
  if SAVE_MODIFF! () >= MODIFF! ()
      && !NILP (Vfirst_change_hook)
    {
      PRESERVE_VALUE (preserve_ptr, &raw mut preserve_marker);
      PRESERVE_START_END (&raw mut start_marker, &raw mut end_marker, start, end);
      run_hook (Qfirst_change_hook ());
    }

  /* Now run the before-change-functions if any.  */
  if !NILP (Vbefore_change_functions)
    {
      rvoe_arg.location = &raw mut Vbefore_change_functions;
      rvoe_arg.errorp = true;

      PRESERVE_VALUE (preserve_ptr, &raw mut preserve_marker);
      PRESERVE_START_END (&raw mut start_marker, &raw mut end_marker, start, end);

      /* Mark before-change-functions to be reset to nil in case of error.  */
      record_unwind_protect_ptr (Some (reset_var_on_error),
				 &raw mut rvoe_arg as *mut c_void);

      /* Actually run the hook functions.  */
      CALLN! (Frun_hook_with_args, Qbefore_change_functions (),
	      FETCH_START (start_marker, start), FETCH_END (end_marker, end));

      /* There was no error: unarm the reset_on_error.  */
      rvoe_arg.errorp = false;
    }

  if buffer_has_overlays ()
    {
      PRESERVE_VALUE (preserve_ptr, &raw mut preserve_marker);
      report_overlay_modification (FETCH_START (start_marker, start),
				   FETCH_END (end_marker, end), false,
				   FETCH_START (start_marker, start),
				   FETCH_END (end_marker, end), Qnil ());
    }

  if ! NILP (start_marker)
    { detach_marker (start_marker); }
  if ! NILP (end_marker)
    { detach_marker (end_marker); }
  RESTORE_VALUE (preserve_ptr, &raw mut preserve_marker);

  unbind_to (count, Qnil ());
}

/* Signal a change immediately after it happens.
   CHARPOS is the character position of the start of the changed text.
   LENDEL is the number of characters of the text before the change.
   (Not the whole buffer; just the part that was changed.)
   LENINS is the number of characters in that part of the text
   after the change.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn signal_after_change (charpos: ptrdiff_t, lendel: ptrdiff_t,
					      lenins: ptrdiff_t)
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut rvoe_arg: rvoe_arg = rvoe_arg { location: std::ptr::null_mut (), errorp: false };
  let mut tmp: Lisp_Object = Qnil ();
  let mut save_insert_behind_hooks: Lisp_Object;
  let mut save_insert_in_from_hooks: Lisp_Object;

  if inhibit_modification_hooks
    { return; }

  /* If we are deferring calls to the after-change functions
     and there are no before-change functions,
     just record the args that we were going to use.  */
  if ! NILP (Vcombine_after_change_calls)
      /* It's OK to defer after-changes even if syntax-ppss-flush-cache
       * is on before-change-functions, which is common enough to be worth
       * adding a special case for it.  */
      && (NILP (Vbefore_change_functions)
          || (CONSP (Vbefore_change_functions)
              && EQ (Qt (), XCAR (Vbefore_change_functions))
              && NILP (Fdefault_value (Qbefore_change_functions ()))
              /* C 2406 assigns into `tmp' inside the condition; the port keeps the
                 assignment at the same point, as a block expression.  */
              && { tmp = XCDR (Vbefore_change_functions); CONSP (tmp) }
              && NILP (XCDR (tmp))
              && EQ (XCAR (tmp), Qsyntax_ppss_flush_cache ())))
      && !buffer_has_overlays ()
    {
      let mut elt: Lisp_Object;

      if !NILP (combine_after_change_list)
	  && (current_buffer! ()) != XBUFFER (combine_after_change_buffer)
	{ Fcombine_after_change_execute (); }

      elt = list3i (charpos - BEG as ptrdiff_t, Z! () - (charpos - lendel + lenins),
		    lenins - lendel);
      combine_after_change_list
	= Fcons (elt, combine_after_change_list);
      combine_after_change_buffer = Fcurrent_buffer ();

      return;
    }

  /* Save and restore the insert-*-hooks, because other hooks like
     after-change-functions, called below, could clobber them if they
     manipulate text properties.  */
  save_insert_behind_hooks = interval_insert_behind_hooks;
  save_insert_in_from_hooks = interval_insert_in_front_hooks;

  if !NILP (combine_after_change_list)
    { Fcombine_after_change_execute (); }

  specbind (Qinhibit_modification_hooks (), Qt ());

  if !NILP (Vafter_change_functions)
    {
      rvoe_arg.location = &raw mut Vafter_change_functions;
      rvoe_arg.errorp = true;

      /* Mark after-change-functions to be reset to nil in case of error.  */
      record_unwind_protect_ptr (Some (reset_var_on_error),
				 &raw mut rvoe_arg as *mut c_void);

      /* Actually run the hook functions.  */
      CALLN! (Frun_hook_with_args, Qafter_change_functions (),
	      make_fixnum (charpos), make_fixnum (charpos + lenins),
	      make_fixnum (lendel));

      /* There was no error: unarm the reset_on_error.  */
      rvoe_arg.errorp = false;
    }

  interval_insert_behind_hooks = save_insert_behind_hooks;
  interval_insert_in_front_hooks = save_insert_in_from_hooks;

  if buffer_has_overlays ()
    { report_overlay_modification (make_fixnum (charpos),
				   make_fixnum (charpos + lenins),
				   true,
				   make_fixnum (charpos),
				   make_fixnum (charpos + lenins),
				   make_fixnum (lendel)); }

  /* After an insertion, call the text properties
     insert-behind-hooks or insert-in-front-hooks.  */
  if lendel == 0
    { report_interval_modification (make_fixnum (charpos),
				    make_fixnum (charpos + lenins)); }

  unbind_to (count, Qnil ());
}

unsafe extern "C" fn Fcombine_after_change_execute_1 (val: Lisp_Object)
{
  Vcombine_after_change_calls = val;
}

DEFUN! ("combine-after-change-execute", Fcombine_after_change_execute,
       Scombine_after_change_execute, 0, 0, 0,
       doc: "This function is for use internally in the function `combine-after-change-calls'.",
       (),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut beg: ptrdiff_t; let mut end: ptrdiff_t; let mut change: ptrdiff_t;
  let mut begpos: ptrdiff_t; let mut endpos: ptrdiff_t;
  let mut tail: Lisp_Object;

  if NILP (combine_after_change_list)
    { return Qnil (); }

  /* It is rare for combine_after_change_buffer to be invalid, but
     possible.  It can happen when combine-after-change-calls is
     non-nil, and insertion calls a file name handler (e.g. through
     lock_file) which scribbles into a temp file -- cyd  */
  if !BUFFERP (combine_after_change_buffer)
      || !BUFFER_LIVE_P (XBUFFER (combine_after_change_buffer))
    {
      combine_after_change_list = Qnil ();
      return Qnil ();
    }

  record_unwind_current_buffer ();

  Fset_buffer (combine_after_change_buffer);

  /* # chars unchanged at beginning of buffer.  */
  beg = Z! () - BEG as ptrdiff_t;
  /* # chars unchanged at end of buffer.  */
  end = beg;
  /* Total amount of insertion (negative for deletion).  */
  change = 0;

  /* Scan the various individual changes,
     accumulating the range info in BEG, END and CHANGE.  */
  tail = combine_after_change_list;
  while CONSP (tail)
    {
      let mut elt: Lisp_Object;
      let mut thisbeg: ptrdiff_t; let mut thisend: ptrdiff_t; let mut thischange: ptrdiff_t;

      /* Extract the info from the next element.  */
      elt = XCAR (tail);
      if ! CONSP (elt)
	{ tail = XCDR (tail); continue; }
      thisbeg = XFIXNUM (XCAR (elt));

      elt = XCDR (elt);
      if ! CONSP (elt)
	{ tail = XCDR (tail); continue; }
      thisend = XFIXNUM (XCAR (elt));

      elt = XCDR (elt);
      if ! CONSP (elt)
	{ tail = XCDR (tail); continue; }
      thischange = XFIXNUM (XCAR (elt));

      /* Merge this range into the accumulated range.  */
      change += thischange;
      if thisbeg < beg
	{ beg = thisbeg; }
      if thisend < end
	{ end = thisend; }

      tail = XCDR (tail);
    }

  /* Get the current start and end positions of the range
     that was changed.  */
  begpos = BEG as ptrdiff_t + beg;
  endpos = Z! () - end;

  /* We are about to handle these, so discard them.  */
  combine_after_change_list = Qnil ();

  /* Now run the after-change functions for real.
     Turn off the flag that defers them.  */
  record_unwind_protect (Some (Fcombine_after_change_execute_1),
			 Vcombine_after_change_calls);
  signal_after_change (begpos, endpos - begpos - change, endpos - begpos);
  update_compositions (begpos, endpos, CHECK_ALL);

  return unbind_to (count, Qnil ());
});


#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_insdel ()
{
  staticpro (&raw const combine_after_change_list);
  staticpro (&raw const combine_after_change_buffer);
  combine_after_change_list = Qnil ();
  combine_after_change_buffer = Qnil ();

  DEFSYM! (Qundo_auto__undoable_change, c"undo-auto--undoable-change");
  DEFSYM! (Qsyntax_ppss_flush_cache, c"syntax-ppss-flush-cache");

  DEFVAR_LISP! (c"combine-after-change-calls", Vcombine_after_change_calls,
		"Used internally by the function `combine-after-change-calls' macro.");
  Vcombine_after_change_calls = Qnil ();

  DEFVAR_BOOL! (c"inhibit-modification-hooks", inhibit_modification_hooks,
		"Non-nil means don't run any of the hooks that respond to buffer changes.\nThis affects `before-change-functions' and `after-change-functions',\nas well as hooks attached to text properties and overlays.\nSetting this variable non-nil also inhibits file locks and checks\nwhether files are locked by another Emacs session, as well as\nhandling of the active region per `select-active-regions'.\n\nTo delay change hooks during a series of changes, use\n`combine-change-calls' or `combine-after-change-calls' instead of\nbinding this variable.\n\nSee also the info node `(elisp) Change Hooks'.");
  inhibit_modification_hooks = false;
  DEFSYM! (Qinhibit_modification_hooks, c"inhibit-modification-hooks");

  defsubr (&raw mut Scombine_after_change_execute);
}
