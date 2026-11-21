// femacs: port of src/intervals.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct/union field, file-scope object, local), same control flow, same error model.
// C's 29 non-`static` definitions (`create_root_interval`, `copy_properties`,
// `intervals_equal`, `traverse_intervals`, `traverse_intervals_noorder`,
// `split_interval_right`, `split_interval_left`, `find_interval`, `next_interval`,
// `previous_interval`, `update_interval`, `offset_intervals`, `merge_interval_left`,
// `graft_intervals_into_buffer`, `textget`, `lookup_char_property`, `temp_set_point_both`,
// `temp_set_point`, `set_point`, `set_point_from_marker`, `set_point_both`,
// `move_if_not_intangible`, `get_property_and_range`, `get_local_map`, `copy_intervals`,
// `copy_intervals_to_string`, `compare_string_intervals`, `set_intervals_multibyte`) carry
// `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static` helpers (`set_interval_left`,
// `set_interval_right`, `copy_interval_parent`, `merge_properties`, `intervals_equal_1`,
// `rotate_right`, `rotate_left`, `balance_an_interval`, `balance_possible_root_interval`,
// `balance_intervals_internal`, `buffer_balance_intervals`, `interval_start_pos`,
// `adjust_intervals_for_insertion`, `merge_properties_sticky`, `delete_node`,
// `delete_interval`, `interval_deletion_adjustment`, `adjust_intervals_for_deletion`,
// `merge_interval_right`, `reproduce_interval`, `reproduce_tree`, `reproduce_tree_obj`,
// `adjust_for_invis_intang`, `set_intervals_multibyte_1`) stay plain `unsafe fn`.
//
// **`intervals.h`'s mirror lands completed with this unit** (`src/include/intervals.rs`,
// the composite.rs/ccl.rs precedent): the type, the 29 macros, the three INLINE setters and
// every `extern` prototype.  This unit defines 24 of the header's prototypes, so those moved
// into an `intervals_decls` module + glob import (PORTING.md 0.1-10, the composite.rs shape);
// the three the header declares but textprop.c defines (`verify_interval_modification`,
// `validate_interval_range` and `interval_of`, all three in textprop.c) and `make_interval`
// (alloc.c) stay at the mirror's root, as do xdisp.c's `invisible_prop` and
// the textprop.c block.  The mirror's one new shape is the six `bool_bf : 1` members of
// `struct interval`, which C stores in one byte (clang measures the bits at 40:0-40:5, the
// `bool_bf` byte `conf_post.h` 61 makes plain `bool`): the port reads and writes them with
// the `interval_bits!` macro over the `INTERVAL_*_BIT` masks (PORTING.md 1.3/P-80), and
// `docs/ports/token-aliases.tsv` carries the `up_obj`/`write_protect`/`visible`/
// `front_sticky`/`rear_sticky` families so every read and write stays occurrence-checked.
//
// **`src/intervals.c` is in `docs/baseline/config/cond-eval-verified.txt`**, so the frozen
// configuration's branch selection is the clang-verified one (P-08): the file's only `#if` is
// the literal `#if 0` at C 1643-1648 (the "this code surely has no effect" comment), which is
// recorded in `docs/ports/intervals.md` rather than transcribed.  C 41-43's `<config.h>` and
// `<intprops.h>` include: the latter has no mirror (PORTING.md P-26) and this unit uses none
// of it.
//
// C's uninitialised locals are zero-initialised (PORTING.md 1.5) and C 54-56's three forward
// declarations are dropped (PORTING.md 0.1-12); both are named in `docs/ports/intervals.md`.
// `INTERVAL` is the mirror set's ABI-equal `*mut c_void` (P-72), so where C writes `i->field`
// the port casts once, `(*((i) as *mut interval)).field`, and where a `struct interval *` the
// mirror's macros return meets a function that takes `INTERVAL` the port casts with `as
// INTERVAL`; neither changes a token C's text carries.  `%"pD"d` (C 776) is written `%td`
// (lisp.h 150: `pD` is `"t"`), the bytecode.rs/region-cache.rs precedent.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut)]
// [U1-MODULE-UNIT]
use crate::common::*;


/* NOTES:

   Have to ensure that we can't put symbol nil on a plist, or some
   functions may work incorrectly.

   An idea:  Have the owner of the tree keep count of splits and/or
   insertion lengths (in intervals), and balance after every N.

   Need to call *_left_hook when buffer is killed.

   Scan for zero-length, or 0-length to see notes about handling
   zero length interval-markers.

   There are comments around about freeing intervals.  It might be
   faster to explicitly free them (put them on the free list) than
   to GC them.

*/


// [U1] config.rs provided by common
// C 43's `#include <intprops.h>` has no mirror (PORTING.md P-26) and this unit uses none of it.
// [U1] ../lib/include/sys_types.in.rs imported via common
// [U1] ../lib/include/time.in.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/intervals.rs imported via common
// [U1] include/buffer.rs imported via common
// [U1] include/keymap.rs imported via common

/* Test for membership, allowing for t (actually any non-cons) to mean the
   universal set.  */

macro_rules! TMEM {
  ($sym:expr, $set:expr) => {
    (if CONSP ($set) { ! NILP (Fmemq ($sym, $set)) } else { ! NILP ($set) })
  };
}


/* Utility functions for intervals.  */

/* Use these functions to set pointer slots of struct interval.  */

unsafe fn set_interval_left (i: INTERVAL, left: INTERVAL)
{
  (*((i) as *mut interval)).left = left as *mut interval;
}

unsafe fn set_interval_right (i: INTERVAL, right: INTERVAL)
{
  (*((i) as *mut interval)).right = right as *mut interval;
}

/* Make the parent of D be whatever the parent of S is, regardless
   of the type.  This is used when balancing an interval tree.  */

unsafe fn copy_interval_parent (d: INTERVAL, s: INTERVAL)
{
  (*((d) as *mut interval)).up = (*((s) as *mut interval)).up;
  interval_bits! (up_obj, d, interval_bits! (up_obj, s));
}

/* Create the root interval of some object, a buffer or string.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn create_root_interval (parent: Lisp_Object) -> INTERVAL
{
  let mut new: INTERVAL = std::ptr::null_mut ();

  new = make_interval ();

  if ! STRINGP (parent)
    {
      (*((new) as *mut interval)).total_length = (BUF_Z! (XBUFFER (parent))
						  - BUF_BEG (XBUFFER (parent)));
      eassert! (TOTAL_LENGTH! (new) >= 0);
      set_buffer_intervals (XBUFFER (parent), new);
      (*((new) as *mut interval)).position = BEG as ptrdiff_t;
    }
  else
    {
      (*((new) as *mut interval)).total_length = SCHARS (parent);
      eassert! (TOTAL_LENGTH! (new) >= 0);
      set_string_intervals (parent, new);
      (*((new) as *mut interval)).position = 0;
    }
  eassert! (LENGTH! (new) > 0);

  set_interval_object (new, parent);

  return new;
}

/* Make the interval TARGET have exactly the properties of SOURCE.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn copy_properties (source: INTERVAL, target: INTERVAL)
{
  if DEFAULT_INTERVAL_P! (source) && DEFAULT_INTERVAL_P! (target)
    { return; }

  COPY_INTERVAL_CACHE! (source, target);
  set_interval_plist (target, Fcopy_sequence ((*((source) as *mut interval)).plist));
}

/* Merge the properties of interval SOURCE into the properties
   of interval TARGET.  That is to say, each property in SOURCE
   is added to TARGET if TARGET has no such property as yet.  */

unsafe fn merge_properties (source: INTERVAL, target: INTERVAL)
{
  let mut o: Lisp_Object = Qnil ();
  let mut sym: Lisp_Object = Qnil ();
  let mut val: Lisp_Object = Qnil ();

  if DEFAULT_INTERVAL_P! (source) && DEFAULT_INTERVAL_P! (target)
    { return; }

  MERGE_INTERVAL_CACHE! (source, target);

  o = (*((source) as *mut interval)).plist;
  while CONSP (o)
    {
      sym = XCAR (o);
      o = XCDR (o);
      CHECK_CONS (o);

      val = (*((target) as *mut interval)).plist;
      while CONSP (val) && !EQ (XCAR (val), sym)
	{
	  val = XCDR (val);
	  if ! CONSP (val)
	    { break; }
	  val = XCDR (val);
	}

      if NILP (val)
	{
	  val = XCAR (o);
	  set_interval_plist (target, Fcons (sym, Fcons (val, (*((target) as *mut interval)).plist)));
	}
      o = XCDR (o);
    }
}

/* Return true if the two intervals have the same properties.
   If use_equal is true, use Fequal for comparisons instead of EQ.  */

unsafe fn intervals_equal_1 (i0: INTERVAL, i1: INTERVAL, use_equal: bool) -> bool
{
  let mut i0_cdr: Lisp_Object = Qnil ();
  let mut i0_sym: Lisp_Object = Qnil ();
  let mut i1_cdr: Lisp_Object = Qnil ();
  let mut i1_val: Lisp_Object = Qnil ();

  if DEFAULT_INTERVAL_P! (i0) && DEFAULT_INTERVAL_P! (i1)
    { return true; }

  if DEFAULT_INTERVAL_P! (i0) || DEFAULT_INTERVAL_P! (i1)
    { return false; }

  i0_cdr = (*((i0) as *mut interval)).plist;
  i1_cdr = (*((i1) as *mut interval)).plist;
  while CONSP (i0_cdr) && CONSP (i1_cdr)
    {
      i0_sym = XCAR (i0_cdr);
      i0_cdr = XCDR (i0_cdr);
      if ! CONSP (i0_cdr)
	{ return false; }
      i1_val = (*((i1) as *mut interval)).plist;
      while CONSP (i1_val) && !EQ (XCAR (i1_val), i0_sym)
	{
	  i1_val = XCDR (i1_val);
	  if ! CONSP (i1_val)
	    { return false; }
	  i1_val = XCDR (i1_val);
	}

      /* i0 has something i1 doesn't.  */
      if NILP (i1_val)
	{ return false; }

      /* i0 and i1 both have sym, but it has different values in each.  */
      if (if (! CONSP (i1_val)
	      || { i1_val = XCDR (i1_val); ! CONSP (i1_val) }
	      || use_equal)
	    { NILP (Fequal (XCAR (i1_val), XCAR (i0_cdr))) }
	  else { !EQ (XCAR (i1_val), XCAR (i0_cdr)) })
	{ return false; }

      i0_cdr = XCDR (i0_cdr);

      i1_cdr = XCDR (i1_cdr);
      if ! CONSP (i1_cdr)
	{ return false; }
      i1_cdr = XCDR (i1_cdr);
    }

  /* Lengths of the two plists were equal.  */
  return (NILP (i0_cdr) && NILP (i1_cdr));
}

/* Return true if the two intervals have the same properties.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn intervals_equal (i0: INTERVAL, i1: INTERVAL) -> bool
{
  return intervals_equal_1 (i0, i1, false);
}


/* Traverse an interval tree TREE, performing FUNCTION on each node.
   No guarantee is made about the order of traversal.
   Pass FUNCTION two args: an interval, and ARG.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn traverse_intervals_noorder (mut tree: INTERVAL,
    function: Option<unsafe extern "C-unwind" fn (INTERVAL, *mut c_void)>, arg: *mut c_void)
{
  /* Minimize stack usage.  */
  while ! tree.is_null ()
    {
      (function.unwrap ()) (tree, arg);
      if (*((tree) as *mut interval)).right.is_null ()
	{ tree = (*((tree) as *mut interval)).left as INTERVAL; }
      else
	{
	  traverse_intervals_noorder ((*((tree) as *mut interval)).left as INTERVAL,
				      function, arg);
	  tree = (*((tree) as *mut interval)).right as INTERVAL;
	}
    }
}

/* Traverse an interval tree TREE, performing FUNCTION on each node.
   Pass FUNCTION two args: an interval, and ARG.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn traverse_intervals (mut tree: INTERVAL, mut position: ptrdiff_t,
    function: Option<unsafe extern "C-unwind" fn (INTERVAL, *mut c_void)>, arg: *mut c_void)
{
  while ! tree.is_null ()
    {
      traverse_intervals ((*((tree) as *mut interval)).left as INTERVAL, position,
			  function, arg);
      position += LEFT_TOTAL_LENGTH! (tree);
      (*((tree) as *mut interval)).position = position;
      (function.unwrap ()) (tree, arg);
      position += LENGTH! (tree); tree = (*((tree) as *mut interval)).right as INTERVAL;
    }
}


/* Assuming that a left child exists, perform the following operation:

     A		  B
    / \		 / \
   B       =>       A
  / \		   / \
     c		  c
*/

unsafe fn rotate_right (A: INTERVAL) -> INTERVAL
{
  let B: INTERVAL = (*((A) as *mut interval)).left as INTERVAL;
  let c: INTERVAL = (*((B) as *mut interval)).right as INTERVAL;
  let old_total: ptrdiff_t = (*((A) as *mut interval)).total_length;

  eassert! (old_total > 0);
  eassert! (LENGTH! (A) > 0);
  eassert! (LENGTH! (B) > 0);

  /* Deal with any Parent of A;  make it point to B.  */
  if ! ROOT_INTERVAL_P! (A)
    {
      if AM_LEFT_CHILD! (A)
	{ set_interval_left (INTERVAL_PARENT! (A) as INTERVAL, B); }
      else
	{ set_interval_right (INTERVAL_PARENT! (A) as INTERVAL, B); }
    }
  copy_interval_parent (B, A);

  /* Make B the parent of A.  */
  set_interval_right (B, A);
  set_interval_parent (A, B);

  /* Make A point to c.  */
  set_interval_left (A, c);
  if ! c.is_null ()
    { set_interval_parent (c, A); }

  /* A's total length is decreased by the length of B and its left child.  */
  (*((A) as *mut interval)).total_length -= TOTAL_LENGTH! (B) - TOTAL_LENGTH0! (c);
  eassert! (TOTAL_LENGTH! (A) > 0);
  eassert! (LENGTH! (A) > 0);

  /* B must have the same total length of A.  */
  (*((B) as *mut interval)).total_length = old_total;
  eassert! (LENGTH! (B) > 0);

  return B;
}

/* Assuming that a right child exists, perform the following operation:

    A               B
   / \	           / \
      B	   =>     A
     / \         / \
    c               c
*/

unsafe fn rotate_left (A: INTERVAL) -> INTERVAL
{
  let B: INTERVAL = (*((A) as *mut interval)).right as INTERVAL;
  let c: INTERVAL = (*((B) as *mut interval)).left as INTERVAL;
  let old_total: ptrdiff_t = (*((A) as *mut interval)).total_length;

  eassert! (old_total > 0);
  eassert! (LENGTH! (A) > 0);
  eassert! (LENGTH! (B) > 0);

  /* Deal with any parent of A;  make it point to B.  */
  if ! ROOT_INTERVAL_P! (A)
    {
      if AM_LEFT_CHILD! (A)
	{ set_interval_left (INTERVAL_PARENT! (A) as INTERVAL, B); }
      else
	{ set_interval_right (INTERVAL_PARENT! (A) as INTERVAL, B); }
    }
  copy_interval_parent (B, A);

  /* Make B the parent of A.  */
  set_interval_left (B, A);
  set_interval_parent (A, B);

  /* Make A point to c.  */
  set_interval_right (A, c);
  if ! c.is_null ()
    { set_interval_parent (c, A); }

  /* A's total length is decreased by the length of B and its right child.  */
  (*((A) as *mut interval)).total_length -= TOTAL_LENGTH! (B) - TOTAL_LENGTH0! (c);
  eassert! (TOTAL_LENGTH! (A) > 0);
  eassert! (LENGTH! (A) > 0);

  /* B must have the same total length of A.  */
  (*((B) as *mut interval)).total_length = old_total;
  eassert! (LENGTH! (B) > 0);

  return B;
}


/* Balance an interval tree with the assumption that the subtrees
   themselves are already balanced.  */

unsafe fn balance_an_interval (mut i: INTERVAL) -> INTERVAL
{
  let mut old_diff: ptrdiff_t = 0;
  let mut new_diff: ptrdiff_t = 0;

  eassert! (LENGTH! (i) > 0);
  eassert! (TOTAL_LENGTH! (i) >= LENGTH! (i));

  while true
    {
      old_diff = LEFT_TOTAL_LENGTH! (i) - RIGHT_TOTAL_LENGTH! (i);
      if old_diff > 0
	{
	  /* Since the left child is longer, there must be one.  */
	  new_diff = (*((i) as *mut interval)).total_length
	    - (*((*((i) as *mut interval)).left)).total_length
	    + RIGHT_TOTAL_LENGTH! ((*((i) as *mut interval)).left)
	    - LEFT_TOTAL_LENGTH! ((*((i) as *mut interval)).left);
	  if eabs! (new_diff) >= old_diff
	    { break; }
	  i = rotate_right (i);
	  balance_an_interval ((*((i) as *mut interval)).right as INTERVAL);
	}
      else if old_diff < 0
	{
	  /* Since the right child is longer, there must be one.  */
	  new_diff = (*((i) as *mut interval)).total_length
	    - (*((*((i) as *mut interval)).right)).total_length
	    + LEFT_TOTAL_LENGTH! ((*((i) as *mut interval)).right)
	    - RIGHT_TOTAL_LENGTH! ((*((i) as *mut interval)).right);
	  if eabs! (new_diff) >= -old_diff
	    { break; }
	  i = rotate_left (i);
	  balance_an_interval ((*((i) as *mut interval)).left as INTERVAL);
	}
      else
	{ break; }
    }
  return i;
}

/* Balance INTERVAL, potentially stuffing it back into its parent
   Lisp Object.  */

unsafe fn balance_possible_root_interval (mut interval: INTERVAL) -> INTERVAL
{
  let mut parent: Lisp_Object = Qnil ();
  let mut have_parent: bool = false;

  if INTERVAL_HAS_OBJECT! (interval)
    {
      have_parent = true;
      GET_INTERVAL_OBJECT! (parent, interval);
    }
  else if ! INTERVAL_HAS_PARENT! (interval)
    { return interval; }

  interval = balance_an_interval (interval);

  if have_parent
    {
      if BUFFERP (parent)
	{ set_buffer_intervals (XBUFFER (parent), interval); }
      else if STRINGP (parent)
	{ set_string_intervals (parent, interval); }
    }

  return interval;
}

/* Balance the interval tree TREE.  Balancing is by weight
   (the amount of text).  */

unsafe fn balance_intervals_internal (tree: INTERVAL) -> INTERVAL
{
  /* Balance within each side.  */
  if ! (*((tree) as *mut interval)).left.is_null ()
    { balance_intervals_internal ((*((tree) as *mut interval)).left as INTERVAL); }
  if ! (*((tree) as *mut interval)).right.is_null ()
    { balance_intervals_internal ((*((tree) as *mut interval)).right as INTERVAL); }
  return balance_an_interval (tree);
}

/* Advertised interface to balance intervals.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn balance_intervals (tree: INTERVAL) -> INTERVAL
{
  return if ! tree.is_null () { balance_intervals_internal (tree) }
	 else { std::ptr::null_mut () };
}

/* Rebalance text properties of B.  */

unsafe fn buffer_balance_intervals (b: *mut buffer)
{
  let mut i: INTERVAL = std::ptr::null_mut ();

  eassert! (b != std::ptr::null_mut ());
  i = buffer_intervals (b);
  if ! i.is_null ()
    { set_buffer_intervals (b, balance_an_interval (i)); }
}

/* Split INTERVAL into two pieces, starting the second piece at
   character position OFFSET (counting from 0), relative to INTERVAL.
   INTERVAL becomes the left-hand piece, and the right-hand piece
   (second, lexicographically) is returned.

   The size and position fields of the two intervals are set based upon
   those of the original interval.  The property list of the new interval
   is reset, thus it is up to the caller to do the right thing with the
   result.

   Note that this does not change the position of INTERVAL;  if it is a root,
   it is still a root after this operation.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn split_interval_right (interval: INTERVAL, offset: ptrdiff_t) -> INTERVAL
{
  let new: INTERVAL = make_interval ();
  let position: ptrdiff_t = (*((interval) as *mut interval)).position;
  let new_length: ptrdiff_t = LENGTH! (interval) - offset;

  (*((new) as *mut interval)).position = position + offset;
  set_interval_parent (new, interval);

  if NULL_RIGHT_CHILD! (interval)
    {
      set_interval_right (interval, new);
      (*((new) as *mut interval)).total_length = new_length;
      eassert! (LENGTH! (new) > 0);
    }
  else
    {
      /* Insert the new node between INTERVAL and its right child.  */
      set_interval_right (new, (*((interval) as *mut interval)).right as INTERVAL);
      set_interval_parent ((*((interval) as *mut interval)).right as INTERVAL, new);
      set_interval_right (interval, new);
      (*((new) as *mut interval)).total_length = new_length
	+ (*((*((new) as *mut interval)).right)).total_length;
      balance_an_interval (new);
    }

  balance_possible_root_interval (interval);

  return new;
}

/* Split INTERVAL into two pieces, starting the second piece at
   character position OFFSET (counting from 0), relative to INTERVAL.
   INTERVAL becomes the right-hand piece, and the left-hand piece
   (first, lexicographically) is returned.

   The size and position fields of the two intervals are set based upon
   those of the original interval.  The property list of the new interval
   is reset, thus it is up to the caller to do the right thing with the
   result.

   Note that this does not change the position of INTERVAL;  if it is a root,
   it is still a root after this operation.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn split_interval_left (interval: INTERVAL, offset: ptrdiff_t) -> INTERVAL
{
  let new: INTERVAL = make_interval ();
  let new_length: ptrdiff_t = offset;

  (*((new) as *mut interval)).position = (*((interval) as *mut interval)).position;
  (*((interval) as *mut interval)).position = (*((interval) as *mut interval)).position + offset;
  set_interval_parent (new, interval);

  if NULL_LEFT_CHILD! (interval)
    {
      set_interval_left (interval, new);
      (*((new) as *mut interval)).total_length = new_length;
      eassert! (LENGTH! (new) > 0);
    }
  else
    {
      /* Insert the new node between INTERVAL and its left child.  */
      set_interval_left (new, (*((interval) as *mut interval)).left as INTERVAL);
      set_interval_parent ((*((new) as *mut interval)).left as INTERVAL, new);
      set_interval_left (interval, new);
      (*((new) as *mut interval)).total_length = new_length
	+ (*((*((new) as *mut interval)).left)).total_length;
      balance_an_interval (new);
    }

  balance_possible_root_interval (interval);

  return new;
}


/* Return the proper position for the first character
   described by the interval tree SOURCE.
   This is 1 if the parent is a buffer,
   0 if the parent is a string or if there is no parent.

   Don't use this function on an interval which is the child
   of another interval!  */

unsafe fn interval_start_pos (source: INTERVAL) -> c_int
{
  let mut parent: Lisp_Object = Qnil ();

  if source.is_null ()
    { return 0; }

  if ! INTERVAL_HAS_OBJECT! (source)
    { return 0; }
  GET_INTERVAL_OBJECT! (parent, source);
  if BUFFERP (parent)
    { return BUF_BEG (XBUFFER (parent)) as c_int; }
  return 0;
}

/* Find the interval containing text position POSITION in the text
   represented by the interval tree TREE.  POSITION is a buffer
   position (starting from 1) or a string index (starting from 0).
   If POSITION is at the end of the buffer or string,
   return the interval containing the last character.

   The `position' field, which is a cache of an interval's position,
   is updated in the interval found.  Other functions (e.g., next_interval)
   will update this cache based on the result of find_interval.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn find_interval (mut tree: INTERVAL, position: ptrdiff_t) -> INTERVAL
{
  /* The distance from the left edge of the subtree at TREE
                    to POSITION.  */
  let mut relative_position: ptrdiff_t = 0;

  if tree.is_null ()
    { return std::ptr::null_mut (); }

  relative_position = position;
  if INTERVAL_HAS_OBJECT! (tree)
    {
      let mut parent: Lisp_Object = Qnil ();
      GET_INTERVAL_OBJECT! (parent, tree);
      if BUFFERP (parent)
	{ relative_position -= BUF_BEG (XBUFFER (parent)); }
    }

  eassert! (relative_position <= TOTAL_LENGTH! (tree));

  tree = balance_possible_root_interval (tree);

  loop
    {
      eassert! (! tree.is_null ());
      if relative_position < LEFT_TOTAL_LENGTH! (tree)
	{
	  tree = (*((tree) as *mut interval)).left as INTERVAL;
	}
      else if ! NULL_RIGHT_CHILD! (tree)
	       && relative_position >= (TOTAL_LENGTH! (tree)
					- RIGHT_TOTAL_LENGTH! (tree))
	{
	  relative_position -= (TOTAL_LENGTH! (tree)
				- RIGHT_TOTAL_LENGTH! (tree));
	  tree = (*((tree) as *mut interval)).right as INTERVAL;
	}
      else
	{
	  (*((tree) as *mut interval)).position
	    = (position - relative_position /* left edge of *tree.  */
	       + LEFT_TOTAL_LENGTH! (tree)); /* left edge of this interval.  */

	  return tree;
	}
    }
}

/* Find the succeeding interval (lexicographically) to INTERVAL.
   Sets the `position' field based on that of INTERVAL (see
   find_interval).  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn next_interval (interval: INTERVAL) -> INTERVAL
{
  let mut i: INTERVAL = interval;
  let mut next_position: ptrdiff_t = 0;

  if i.is_null ()
    { return std::ptr::null_mut (); }
  next_position = (*((interval) as *mut interval)).position + LENGTH! (interval);

  if ! NULL_RIGHT_CHILD! (i)
    {
      i = (*((i) as *mut interval)).right as INTERVAL;
      while ! NULL_LEFT_CHILD! (i)
	{ i = (*((i) as *mut interval)).left as INTERVAL; }

      (*((i) as *mut interval)).position = next_position;
      return i;
    }

  while ! NULL_PARENT! (i)
    {
      if AM_LEFT_CHILD! (i)
	{
	  i = INTERVAL_PARENT! (i) as INTERVAL;
	  (*((i) as *mut interval)).position = next_position;
	  return i;
	}

      i = INTERVAL_PARENT! (i) as INTERVAL;
    }

  return std::ptr::null_mut ();
}

/* Find the preceding interval (lexicographically) to INTERVAL.
   Sets the `position' field based on that of INTERVAL (see
   find_interval).  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn previous_interval (interval: INTERVAL) -> INTERVAL
{
  let mut i: INTERVAL = std::ptr::null_mut ();

  if interval.is_null ()
    { return std::ptr::null_mut (); }

  if ! NULL_LEFT_CHILD! (interval)
    {
      i = (*((interval) as *mut interval)).left as INTERVAL;
      while ! NULL_RIGHT_CHILD! (i)
	{ i = (*((i) as *mut interval)).right as INTERVAL; }

      (*((i) as *mut interval)).position = (*((interval) as *mut interval)).position - LENGTH! (i);
      return i;
    }

  i = interval;
  while ! NULL_PARENT! (i)
    {
      if AM_RIGHT_CHILD! (i)
	{
	  i = INTERVAL_PARENT! (i) as INTERVAL;

	  (*((i) as *mut interval)).position = (*((interval) as *mut interval)).position - LENGTH! (i);
	  return i;
	}
      i = INTERVAL_PARENT! (i) as INTERVAL;
    }

  return std::ptr::null_mut ();
}

/* Set the ->position field of I's parent, based on I->position. */
macro_rules! SET_PARENT_POSITION {
  ($i:expr) => {
      if AM_LEFT_CHILD! ($i)
	{ (*((INTERVAL_PARENT! ($i)) as *mut interval)).position
	    = (*((($i) as *mut interval))).position + TOTAL_LENGTH! ($i) - LEFT_TOTAL_LENGTH! ($i); }
      else
	{ (*((INTERVAL_PARENT! ($i)) as *mut interval)).position
	    = (*((($i) as *mut interval))).position - LEFT_TOTAL_LENGTH! ($i)
	      - LENGTH! (INTERVAL_PARENT! ($i)) }
  };
}

/* Find the interval containing POS, given some interval I in
   the same tree.  Note that we update interval->position in each
   interval we traverse, assuming it is already correctly set for the
   argument I.  We don't assume that any other interval already has a
   correctly set ->position.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn update_interval (mut i: INTERVAL, pos: ptrdiff_t) -> INTERVAL
{
  if i.is_null ()
    { return std::ptr::null_mut (); }

  loop
    {
      if pos < (*((i) as *mut interval)).position
	{
	  /* Move left.  */
	  if pos >= (*((i) as *mut interval)).position - LEFT_TOTAL_LENGTH! (i)
	    {
	      (*(((*((i) as *mut interval)).left) as *mut interval)).position
		= (*((i) as *mut interval)).position - TOTAL_LENGTH! ((*((i) as *mut interval)).left)
		  + LEFT_TOTAL_LENGTH! ((*((i) as *mut interval)).left);
	      i = (*((i) as *mut interval)).left as INTERVAL;		/* Move to the left child.  */
	    }
	  else if NULL_PARENT! (i)
	    { error (c"Point before start of properties".as_ptr ()); }
	  else
	    {
	      SET_PARENT_POSITION! (i);
	      i = INTERVAL_PARENT! (i) as INTERVAL;
	    }
	  continue;
	}
      else if pos >= INTERVAL_LAST_POS! (i)
	{
	  /* Move right.  */
	  if pos < INTERVAL_LAST_POS! (i) + RIGHT_TOTAL_LENGTH! (i)
	    {
	      (*(((*((i) as *mut interval)).right) as *mut interval)).position
		= INTERVAL_LAST_POS! (i) + LEFT_TOTAL_LENGTH! ((*((i) as *mut interval)).right);
	      i = (*((i) as *mut interval)).right as INTERVAL;	/* Move to the right child.  */
	    }
	  else if NULL_PARENT! (i)
	    { error (c"Point %td after end of properties".as_ptr (), pos); }
	  else
	    {
	      SET_PARENT_POSITION! (i);
	      i = INTERVAL_PARENT! (i) as INTERVAL;
	    }
	  continue;
	}
      else
	{ return i; }
    }
}


/* Effect an adjustment corresponding to the addition of LENGTH characters
   of text.  Do this by finding the interval containing POSITION in the
   interval tree TREE, and then adjusting all of its ancestors by adding
   LENGTH to them.

   If POSITION is the first character of an interval, meaning that point
   is actually between the two intervals, make the new text belong to
   the interval which is "sticky".

   If both intervals are "sticky", then make them belong to the left-most
   interval.  Another possibility would be to create a new interval for
   this text, and make it have the merged properties of both ends.  */

unsafe fn adjust_intervals_for_insertion (mut tree: INTERVAL,
					  mut position: ptrdiff_t, length: ptrdiff_t)
  -> INTERVAL
{
  let mut i: INTERVAL = std::ptr::null_mut ();
  let mut temp: INTERVAL = std::ptr::null_mut ();
  let mut eobp: bool = false;
  let mut parent: Lisp_Object = Qnil ();
  let mut offset: ptrdiff_t = 0;

  eassert! (TOTAL_LENGTH! (tree) > 0);

  GET_INTERVAL_OBJECT! (parent, tree);
  offset = (if BUFFERP (parent) { BUF_BEG (XBUFFER (parent)) } else { 0 });

  /* If inserting at point-max of a buffer, that position will be out
     of range.  Remember that buffer positions are 1-based.  */
  if position >= TOTAL_LENGTH! (tree) + offset
    {
      position = TOTAL_LENGTH! (tree) + offset;
      eobp = true;
    }

  i = find_interval (tree, position);

  /* If in middle of an interval which is not sticky either way,
     we must not just give its properties to the insertion.
     So split this interval at the insertion point.

     Originally, the if condition here was this:
	(! (position == i->position || eobp)
	 && END_NONSTICKY_P (i)
	 && FRONT_NONSTICKY_P (i))
     But, these macros are now unreliable because of introduction of
     Vtext_property_default_nonsticky.  So, we always check properties
     one by one if POSITION is in middle of an interval.  */
  if ! (position == (*((i) as *mut interval)).position || eobp)
    {
      let mut tail: Lisp_Object = Qnil ();
      let mut front: Lisp_Object = Qnil ();
      let mut rear: Lisp_Object = Qnil ();

      'check_done: {
	tail = (*((i) as *mut interval)).plist;

	/* Properties font-sticky and rear-nonsticky override
	   Vtext_property_default_nonsticky.  So, if they are t, we can
	   skip one by one checking of properties.  */
	rear = textget ((*((i) as *mut interval)).plist, Qrear_nonsticky ());
	if ! CONSP (rear) && ! NILP (rear)
	  {
	    /* All properties are nonsticky.  We split the interval.  */
	    break 'check_done;
	  }
	front = textget ((*((i) as *mut interval)).plist, Qfront_sticky ());
	if ! CONSP (front) && ! NILP (front)
	  {
	    /* All properties are sticky.  We don't split the interval.  */
	    tail = Qnil ();
	    break 'check_done;
	  }

	/* Does any actual property pose an actual problem?  We break
	   the loop if we find a nonsticky property.  */
	'tail_loop: while CONSP (tail)
	  {
	    let mut prop: Lisp_Object = Qnil ();
	    let mut tmp: Lisp_Object = Qnil ();
	    'tail_step: {
	      prop = XCAR (tail);

	      /* Is this particular property front-sticky?  */
	      if CONSP (front) && ! NILP (Fmemq (prop, front))
		{ break 'tail_step; }

	      /* Is this particular property rear-nonsticky?  */
	      if CONSP (rear) && ! NILP (Fmemq (prop, rear))
		{ break 'tail_loop; }

	      /* Is this particular property recorded as sticky or
		 nonsticky in Vtext_property_default_nonsticky?  */
	      tmp = Fassq (prop, Vtext_property_default_nonsticky);
	      if CONSP (tmp)
		{
		  if NILP (tmp)
		    { break 'tail_step; }
		  break 'tail_loop;
		}

	      /* By default, a text property is rear-sticky, thus we
		 continue the loop.  */
	    }
	    tail = Fcdr (XCDR (tail));
	  }
      }

      /* If any property is a real problem, split the interval.  */
      if ! NILP (tail)
	{
	  temp = split_interval_right (i, position - (*((i) as *mut interval)).position);
	  copy_properties (i, temp);
	  i = temp;
	}
    }

  /* If we are positioned between intervals, check the stickiness of
     both of them.  We have to do this too, if we are at BEG or Z.  */
  if position == (*((i) as *mut interval)).position || eobp
    {
      let mut prev: INTERVAL = std::ptr::null_mut ();

      if position == BEG as ptrdiff_t
	{ prev = 0 as INTERVAL; }
      else if eobp
	{
	  prev = i;
	  i = 0 as INTERVAL;
	}
      else
	{ prev = previous_interval (i); }

      /* Even if we are positioned between intervals, we default
	 to the left one if it exists.  We extend it now and split
	 off a part later, if stickiness demands it.  */
      temp = if ! prev.is_null () { prev } else { i };
      while ! temp.is_null ()
	{
	  (*((temp) as *mut interval)).total_length += length;
	  temp = balance_possible_root_interval (temp);
	  temp = INTERVAL_PARENT_OR_NULL! (temp);
	}

      /* If at least one interval has sticky properties,
	 we check the stickiness property by property.

	 Originally, the if condition here was this:
		(END_NONSTICKY_P (prev) || FRONT_STICKY_P (i))
	 But, these macros are now unreliable because of introduction
	 of Vtext_property_default_nonsticky.  So, we always have to
	 check stickiness of properties one by one.  If cache of
	 stickiness is implemented in the future, we may be able to
	 use those macros again.  */
      if true
	{
	  let mut pleft: Lisp_Object = Qnil ();
	  let mut pright: Lisp_Object = Qnil ();
	  let mut newi: interval = interval {
	    total_length: 0, position: 0, left: std::ptr::null_mut (),
	    right: std::ptr::null_mut (), up: interval_up { interval: std::ptr::null_mut () },
	    bits: 0, plist: Qnil () };

	  RESET_INTERVAL! (&raw mut newi);
	  pleft = if ! prev.is_null () { (*((prev) as *mut interval)).plist } else { Qnil () };
	  pright = if ! i.is_null () { (*((i) as *mut interval)).plist } else { Qnil () };
	  set_interval_plist (&raw mut newi as INTERVAL,
			      merge_properties_sticky (pleft, pright));

	  if prev.is_null () /* i.e. position == BEG */
	    {
	      if ! intervals_equal (i, &raw mut newi as INTERVAL)
		{
		  i = split_interval_left (i, length);
		  set_interval_plist (i, newi.plist);
		}
	    }
	  else if ! intervals_equal (prev, &raw mut newi as INTERVAL)
	    {
	      prev = split_interval_right (prev, position - (*((prev) as *mut interval)).position);
	      set_interval_plist (prev, newi.plist);
	      if ! i.is_null () && intervals_equal (prev, i)
		{ merge_interval_right (prev); }
	    }

	  /* We will need to update the cache here later.  */
	}
      else if prev.is_null () && ! NILP ((*((i) as *mut interval)).plist)
        {
	  /* Just split off a new interval at the left.
	     Since I wasn't front-sticky, the empty plist is ok.  */
	  i = split_interval_left (i, length);
        }
    }

  /* Otherwise just extend the interval.  */
  else
    {
      temp = i;
      while ! temp.is_null ()
	{
	  (*((temp) as *mut interval)).total_length += length;
	  temp = balance_possible_root_interval (temp);
	  temp = INTERVAL_PARENT_OR_NULL! (temp);
	}
    }

  return tree;
}

/* Any property might be front-sticky on the left, rear-sticky on the left,
   front-sticky on the right, or rear-sticky on the right; the 16 combinations
   can be arranged in a matrix with rows denoting the left conditions and
   columns denoting the right conditions:
      _  __  _
_     FR FR FR FR
FR__   0  1  2  3
 _FR   4  5  6  7
FR     8  9  A  B
  FR   C  D  E  F

   left-props  = '(front-sticky (p8 p9 pa pb pc pd pe pf)
		   rear-nonsticky (p4 p5 p6 p7 p8 p9 pa pb)
		   p0 L p1 L p2 L p3 L p4 L p5 L p6 L p7 L
		   p8 L p9 L pa L pb L pc L pd L pe L pf L)
   right-props = '(front-sticky (p2 p3 p6 p7 pa pb pe pf)
		   rear-nonsticky (p1 p2 p5 p6 p9 pa pd pe)
		   p0 R p1 R p2 R p3 R p4 R p5 R p6 R p7 R
		   p8 R p9 R pa R pb R pc R pd R pe R pf R)

   We inherit from whoever has a sticky side facing us.  If both sides
   do (cases 2, 3, E, and F), then we inherit from whichever side has a
   non-nil value for the current property.  If both sides do, then we take
   from the left.

   When we inherit a property, we get its stickiness as well as its value.
   So, when we merge the above two lists, we expect to get this:

   result      = '(front-sticky (p6 p7 pa pb pc pd pe pf)
		   rear-nonsticky (p6 pa)
		   p0 L p1 L p2 L p3 L p6 R p7 R
		   pa R pb R pc L pd L pe L pf L)

   The optimizable special cases are:
       left rear-nonsticky = nil, right front-sticky = nil (inherit left)
       left rear-nonsticky = t,   right front-sticky = t   (inherit right)
       left rear-nonsticky = t,   right front-sticky = nil (inherit none)
*/

unsafe fn merge_properties_sticky (pleft: Lisp_Object, pright: Lisp_Object) -> Lisp_Object
{
  let mut props: Lisp_Object = Qnil ();
  let mut front: Lisp_Object = Qnil ();
  let mut rear: Lisp_Object = Qnil ();
  let mut lfront: Lisp_Object = Qnil ();
  let mut lrear: Lisp_Object = Qnil ();
  let mut rfront: Lisp_Object = Qnil ();
  let mut rrear: Lisp_Object = Qnil ();
  let mut tail1: Lisp_Object = Qnil ();
  let mut tail2: Lisp_Object = Qnil ();
  let mut sym: Lisp_Object = Qnil ();
  let mut lval: Lisp_Object = Qnil ();
  let mut rval: Lisp_Object = Qnil ();
  let mut cat: Lisp_Object = Qnil ();
  let mut use_left: bool = false;
  let mut use_right: bool = false;
  let mut lpresent: bool = false;

  props = Qnil ();
  front = Qnil ();
  rear  = Qnil ();
  lfront = textget (pleft, Qfront_sticky ());
  lrear  = textget (pleft, Qrear_nonsticky ());
  rfront = textget (pright, Qfront_sticky ());
  rrear  = textget (pright, Qrear_nonsticky ());

  /* Go through each element of PRIGHT.  */
  tail1 = pright;
  while CONSP (tail1)
    {
      let mut tmp: Lisp_Object = Qnil ();

      'right_step: {
	sym = XCAR (tail1);

	/* Sticky properties get special treatment.  */
	if EQ (sym, Qrear_nonsticky ()) || EQ (sym, Qfront_sticky ())
	  { break 'right_step; }

	rval = Fcar (XCDR (tail1));
	tail2 = pleft;
	'right_search: while CONSP (tail2)
	  {
	    if EQ (sym, XCAR (tail2))
	      { break 'right_search; }
	    tail2 = Fcdr (XCDR (tail2));
	  }

	/* Indicate whether the property is explicitly defined on the left.
	   (We know it is defined explicitly on the right
	   because otherwise we don't get here.)  */
	lpresent = ! NILP (tail2);
	lval = if NILP (tail2) { Qnil () } else { Fcar (Fcdr (tail2)) };

	/* Even if lrear or rfront say nothing about the stickiness of
	   SYM, Vtext_property_default_nonsticky may give default
	   stickiness to SYM.  */
	tmp = Fassq (sym, Vtext_property_default_nonsticky);
	use_left = (lpresent
		    && ! (TMEM! (sym, lrear)
			  || (CONSP (tmp) && ! NILP (XCDR (tmp)))));
	use_right = (TMEM! (sym, rfront)
		     || (CONSP (tmp) && NILP (XCDR (tmp))));
	if use_left && use_right
	  {
	    if NILP (lval)
	      { use_left = false; }
	    else if NILP (rval)
	      { use_right = false; }
	  }
	if use_left
	  {
	    /* We build props as (value sym ...) rather than (sym value ...)
	       because we plan to nreverse it when we're done.  */
	    props = Fcons (lval, Fcons (sym, props));
	    if TMEM! (sym, lfront)
	      { front = Fcons (sym, front); }
	    if TMEM! (sym, lrear)
	      { rear = Fcons (sym, rear); }
	  }
	else if use_right
	  {
	    props = Fcons (rval, Fcons (sym, props));
	    if TMEM! (sym, rfront)
	      { front = Fcons (sym, front); }
	    if TMEM! (sym, rrear)
	      { rear = Fcons (sym, rear); }
	  }
      }
      tail1 = Fcdr (XCDR (tail1));
    }

  /* Now go through each element of PLEFT.  */
  tail2 = pleft;
  while CONSP (tail2)
    {
      let mut tmp: Lisp_Object = Qnil ();

      'left_step: {
	sym = XCAR (tail2);

	/* Sticky properties get special treatment.  */
	if EQ (sym, Qrear_nonsticky ()) || EQ (sym, Qfront_sticky ())
	  { break 'left_step; }

	/* If sym is in PRIGHT, we've already considered it.  */
	tail1 = pright;
	'left_search: while CONSP (tail1)
	  {
	    if EQ (sym, XCAR (tail1))
	      { break 'left_search; }
	    tail1 = Fcdr (XCDR (tail1));
	  }
	if ! NILP (tail1)
	  { break 'left_step; }

	lval = Fcar (XCDR (tail2));

	/* Even if lrear or rfront say nothing about the stickiness of
	   SYM, Vtext_property_default_nonsticky may give default
	   stickiness to SYM.  */
	tmp = Fassq (sym, Vtext_property_default_nonsticky);

	/* Since rval is known to be nil in this loop, the test simplifies.  */
	if ! (TMEM! (sym, lrear) || (CONSP (tmp) && ! NILP (XCDR (tmp))))
	  {
	    props = Fcons (lval, Fcons (sym, props));
	    if TMEM! (sym, lfront)
	      { front = Fcons (sym, front); }
	  }
	else if TMEM! (sym, rfront) || (CONSP (tmp) && NILP (XCDR (tmp)))
	  {
	    /* The value is nil, but we still inherit the stickiness
	       from the right.  */
	    front = Fcons (sym, front);
	    if TMEM! (sym, rrear)
	      { rear = Fcons (sym, rear); }
	  }
      }
      tail2 = Fcdr (XCDR (tail2));
    }
  props = Fnreverse (props);
  if ! NILP (rear)
    { props = Fcons (Qrear_nonsticky (), Fcons (Fnreverse (rear), props)); }

  cat = textget (props, Qcategory ());
  if ! NILP (front)
    &&
      /* If we have inherited a front-stick category property that is t,
	 we don't need to set up a detailed one.  */
      ! (! NILP (cat) && SYMBOLP (cat)
	 && EQ (Fget (cat, Qfront_sticky ()), Qt ()))
    { props = Fcons (Qfront_sticky (), Fcons (Fnreverse (front), props)); }
  return props;
}


/* Delete a node I from its interval tree by merging its subtrees
   into one subtree which is then returned.  Caller is responsible for
   storing the resulting subtree into its parent.  */

unsafe fn delete_node (i: INTERVAL) -> INTERVAL
{
  let mut migrate: INTERVAL = std::ptr::null_mut ();
  let mut this: INTERVAL = std::ptr::null_mut ();
  let mut migrate_amt: ptrdiff_t = 0;

  if (*((i) as *mut interval)).left.is_null ()
    { return (*((i) as *mut interval)).right as INTERVAL; }
  if (*((i) as *mut interval)).right.is_null ()
    { return (*((i) as *mut interval)).left as INTERVAL; }

  migrate = (*((i) as *mut interval)).left as INTERVAL;
  migrate_amt = (*((*((i) as *mut interval)).left)).total_length;
  this = (*((i) as *mut interval)).right as INTERVAL;
  (*((this) as *mut interval)).total_length += migrate_amt;
  while ! (*((this) as *mut interval)).left.is_null ()
    {
      this = (*((this) as *mut interval)).left as INTERVAL;
      (*((this) as *mut interval)).total_length += migrate_amt;
    }
  set_interval_left (this, migrate);
  set_interval_parent (migrate, this);
  eassert! (LENGTH! (this) > 0);
  eassert! (LENGTH! ((*((i) as *mut interval)).right as INTERVAL) > 0);

  return (*((i) as *mut interval)).right as INTERVAL;
}

/* Delete interval I from its tree by calling `delete_node'
   and properly connecting the resultant subtree.

   I is presumed to be empty; that is, no adjustments are made
   for the length of I.  */

unsafe fn delete_interval (i: INTERVAL)
{
  let mut parent: INTERVAL = std::ptr::null_mut ();
  let amt: ptrdiff_t = LENGTH! (i);

  eassert! (amt <= 0);	/* Only used on zero total-length intervals now.  */

  if ROOT_INTERVAL_P! (i)
    {
      let mut owner: Lisp_Object = Qnil ();
      GET_INTERVAL_OBJECT! (owner, i);
      parent = delete_node (i);
      if ! parent.is_null ()
	{ set_interval_object (parent, owner); }

      if BUFFERP (owner)
	{ set_buffer_intervals (XBUFFER (owner), parent); }
      else if STRINGP (owner)
	{ set_string_intervals (owner, parent); }
      else
	{ emacs_abort (); }

      return;
    }

  parent = INTERVAL_PARENT! (i) as INTERVAL;
  if AM_LEFT_CHILD! (i)
    {
      set_interval_left (parent, delete_node (i));
      if ! (*((parent) as *mut interval)).left.is_null ()
	{ set_interval_parent ((*((parent) as *mut interval)).left as INTERVAL, parent); }
    }
  else
    {
      set_interval_right (parent, delete_node (i));
      if ! (*((parent) as *mut interval)).right.is_null ()
	{ set_interval_parent ((*((parent) as *mut interval)).right as INTERVAL, parent); }
    }
}


/* Find the interval in TREE corresponding to the relative position
   FROM and delete as much as possible of AMOUNT from that interval.
   Return the amount actually deleted, and if the interval was
   zeroed-out, delete that interval node from the tree.

   Note that FROM is actually origin zero, aka relative to the
   leftmost edge of tree.  This is appropriate since we call ourselves
   recursively on subtrees.

   Do this by recursing down TREE to the interval in question, and
   deleting the appropriate amount of text.  */

unsafe fn interval_deletion_adjustment (tree: INTERVAL, from: ptrdiff_t,
					mut amount: ptrdiff_t) -> ptrdiff_t
{
  let mut relative_position: ptrdiff_t = from;

  if tree.is_null ()
    { return 0; }

  /* Left branch.  */
  if relative_position < LEFT_TOTAL_LENGTH! (tree)
    {
      let subtract: ptrdiff_t = interval_deletion_adjustment (
	(*((tree) as *mut interval)).left as INTERVAL, relative_position, amount);
      (*((tree) as *mut interval)).total_length -= subtract;
      eassert! (LENGTH! (tree) > 0);
      return subtract;
    }
  /* Right branch.  */
  else if relative_position >= (TOTAL_LENGTH! (tree)
				- RIGHT_TOTAL_LENGTH! (tree))
    {
      let mut subtract: ptrdiff_t = 0;

      relative_position -= ((*((tree) as *mut interval)).total_length
			    - RIGHT_TOTAL_LENGTH! (tree));
      subtract = interval_deletion_adjustment ((*((tree) as *mut interval)).right as INTERVAL,
					       relative_position,
					       amount);
      (*((tree) as *mut interval)).total_length -= subtract;
      eassert! (LENGTH! (tree) > 0);
      return subtract;
    }
  /* Here -- this node.  */
  else
    {
      /* How much can we delete from this interval?  */
      let my_amount: ptrdiff_t = (((*((tree) as *mut interval)).total_length
				   - RIGHT_TOTAL_LENGTH! (tree))
				  - relative_position);

      if amount > my_amount
	{ amount = my_amount; }

      (*((tree) as *mut interval)).total_length -= amount;
      eassert! (LENGTH! (tree) >= 0);
      if LENGTH! (tree) == 0
	{ delete_interval (tree); }

      return amount;
    }

  /* Never reach here.  */
}

/* Effect the adjustments necessary to the interval tree of BUFFER to
   correspond to the deletion of LENGTH characters from that buffer
   text.  The deletion is effected at position START (which is a
   buffer position, i.e. origin 1).  */

unsafe fn adjust_intervals_for_deletion (buffer: *mut buffer,
					 mut start: ptrdiff_t, length: ptrdiff_t)
{
  let mut left_to_delete: ptrdiff_t = length;
  let mut tree: INTERVAL = buffer_intervals (buffer);
  let mut parent: Lisp_Object = Qnil ();
  let mut offset: ptrdiff_t = 0;

  GET_INTERVAL_OBJECT! (parent, tree);
  offset = (if BUFFERP (parent) { BUF_BEG (XBUFFER (parent)) } else { 0 });

  if tree.is_null ()
    { return; }

  eassert! (start <= offset + TOTAL_LENGTH! (tree)
	    && start + length <= offset + TOTAL_LENGTH! (tree));

  if length == TOTAL_LENGTH! (tree)
    {
      set_buffer_intervals (buffer, std::ptr::null_mut ());
      return;
    }

  if ONLY_INTERVAL_P! (tree)
    {
      (*((tree) as *mut interval)).total_length -= length;
      eassert! (LENGTH! (tree) > 0);
      return;
    }

  if start > offset + TOTAL_LENGTH! (tree)
    { start = offset + TOTAL_LENGTH! (tree); }
  while left_to_delete > 0
    {
      left_to_delete -= interval_deletion_adjustment (tree, start - offset,
						      left_to_delete);
      tree = buffer_intervals (buffer);
      if left_to_delete == (*((tree) as *mut interval)).total_length
	{
	  set_buffer_intervals (buffer, std::ptr::null_mut ());
	  return;
	}
    }
}


/* Make the adjustments necessary to the interval tree of BUFFER to
   represent an addition or deletion of LENGTH characters starting
   at position START.  Addition or deletion is indicated by the sign
   of LENGTH.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn offset_intervals (buffer: *mut buffer, start: ptrdiff_t, length: ptrdiff_t)
{
  if buffer_intervals (buffer).is_null () || length == 0
    { return; }

  if length > 0
    { adjust_intervals_for_insertion (buffer_intervals (buffer), start, length); }
  else
    { adjust_intervals_for_deletion (buffer, start, -length); }
}


/* Merge interval I with its lexicographic successor. The resulting
   interval is returned, and has the properties of the original
   successor.  The properties of I are lost.  I is removed from the
   interval tree.

   IMPORTANT:
   The caller must verify that this is not the last (rightmost)
   interval.  */

unsafe fn merge_interval_right (i: INTERVAL) -> INTERVAL
{
  let absorb: ptrdiff_t = LENGTH! (i);
  let mut successor: INTERVAL = std::ptr::null_mut ();

  /* Find the succeeding interval.  */
  if ! NULL_RIGHT_CHILD! (i)      /* It's below us.  Add absorb
				      as we descend.  */
    {
      successor = (*((i) as *mut interval)).right as INTERVAL;
      while ! NULL_LEFT_CHILD! (successor)
	{
	  (*((successor) as *mut interval)).total_length += absorb;
	  eassert! (LENGTH! (successor) > 0);
	  successor = (*((successor) as *mut interval)).left as INTERVAL;
	}

      (*((successor) as *mut interval)).total_length += absorb;
      eassert! (LENGTH! (successor) > 0);
      delete_interval (i);
      return successor;
    }

  /* Zero out this interval.  */
  (*((i) as *mut interval)).total_length -= absorb;
  eassert! (TOTAL_LENGTH! (i) >= 0);

  successor = i;
  while ! NULL_PARENT! (successor)	   /* It's above us.  Subtract as
					      we ascend.  */
    {
      if AM_LEFT_CHILD! (successor)
	{
	  successor = INTERVAL_PARENT! (successor) as INTERVAL;
	  delete_interval (i);
	  return successor;
	}

      successor = INTERVAL_PARENT! (successor) as INTERVAL;
      (*((successor) as *mut interval)).total_length -= absorb;
      eassert! (LENGTH! (successor) > 0);
    }

  /* This must be the rightmost or last interval and cannot
     be merged right.  The caller should have known.  */
  emacs_abort ()
}

/* Merge interval I with its lexicographic predecessor. The resulting
   interval is returned, and has the properties of the original predecessor.
   The properties of I are lost.  Interval node I is removed from the tree.

   IMPORTANT:
   The caller must verify that this is not the first (leftmost) interval.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn merge_interval_left (i: INTERVAL) -> INTERVAL
{
  let absorb: ptrdiff_t = LENGTH! (i);
  let mut predecessor: INTERVAL = std::ptr::null_mut ();

  /* Find the preceding interval.  */
  if ! NULL_LEFT_CHILD! (i)	/* It's below us. Go down,
				   adding ABSORB as we go.  */
    {
      predecessor = (*((i) as *mut interval)).left as INTERVAL;
      while ! NULL_RIGHT_CHILD! (predecessor)
	{
	  (*((predecessor) as *mut interval)).total_length += absorb;
	  eassert! (LENGTH! (predecessor) > 0);
	  predecessor = (*((predecessor) as *mut interval)).right as INTERVAL;
	}

      (*((predecessor) as *mut interval)).total_length += absorb;
      eassert! (LENGTH! (predecessor) > 0);
      delete_interval (i);
      return predecessor;
    }

  /* Zero out this interval.  */
  (*((i) as *mut interval)).total_length -= absorb;
  eassert! (TOTAL_LENGTH! (i) >= 0);

  predecessor = i;
  while ! NULL_PARENT! (predecessor)	/* It's above us.  Go up,
					   subtracting ABSORB.  */
    {
      if AM_RIGHT_CHILD! (predecessor)
	{
	  predecessor = INTERVAL_PARENT! (predecessor) as INTERVAL;
	  delete_interval (i);
	  return predecessor;
	}

      predecessor = INTERVAL_PARENT! (predecessor) as INTERVAL;
      (*((predecessor) as *mut interval)).total_length -= absorb;
      eassert! (LENGTH! (predecessor) > 0);
    }

  /* This must be the leftmost or first interval and cannot
     be merged left.  The caller should have known.  */
  emacs_abort ()
}


/* Create a copy of SOURCE but with the default value of UP.  */

unsafe fn reproduce_interval (source: INTERVAL) -> INTERVAL
{
  let target: INTERVAL = make_interval ();

  eassert! (LENGTH! (source) > 0);

  (*((target) as *mut interval)).total_length = (*((source) as *mut interval)).total_length;
  (*((target) as *mut interval)).position = (*((source) as *mut interval)).position;

  copy_properties (source, target);

  if ! NULL_LEFT_CHILD! (source)
    { set_interval_left (target, reproduce_tree ((*((source) as *mut interval)).left as INTERVAL, target)); }
  if ! NULL_RIGHT_CHILD! (source)
    { set_interval_right (target, reproduce_tree ((*((source) as *mut interval)).right as INTERVAL, target)); }

  eassert! (LENGTH! (target) > 0);
  return target;
}

/* Make an exact copy of interval tree SOURCE which descends from
   PARENT.  This is done by recursing through SOURCE, copying
   the current interval and its properties, and then adjusting
   the pointers of the copy.  */

unsafe fn reproduce_tree (source: INTERVAL, parent: INTERVAL) -> INTERVAL
{
  let target: INTERVAL = reproduce_interval (source);
  set_interval_parent (target, parent);
  return target;
}

unsafe fn reproduce_tree_obj (source: INTERVAL, parent: Lisp_Object) -> INTERVAL
{
  let target: INTERVAL = reproduce_interval (source);
  set_interval_object (target, parent);
  return target;
}


/* Insert the intervals of SOURCE into BUFFER at POSITION.
   LENGTH is the length of the text in SOURCE.

   The `position' field of the SOURCE intervals is assumed to be
   consistent with its parent; therefore, SOURCE must be an
   interval tree made with copy_interval or must be the whole
   tree of a buffer or a string.

   This is used in insdel.c when inserting Lisp_Strings into the
   buffer.  The text corresponding to SOURCE is already in the buffer
   when this is called.  The intervals of new tree are a copy of those
   belonging to the string being inserted; intervals are never
   shared.

   If the inserted text had no intervals associated, and we don't
   want to inherit the surrounding text's properties, this function
   simply returns -- offset_intervals should handle placing the
   text in the correct interval, depending on the sticky bits.

   If the inserted text had properties (intervals), then there are two
   cases -- either insertion happened in the middle of some interval,
   or between two intervals.

   If the text goes into the middle of an interval, then new intervals
   are created in the middle, and new text has the union of its properties
   and those of the text into which it was inserted.

   If the text goes between two intervals, then if neither interval
   had its appropriate sticky property set (front_sticky, rear_sticky),
   the new text has only its properties.  If one of the sticky properties
   is set, then the new text "sticks" to that region and its properties
   depend on merging as above.  If both the preceding and succeeding
   intervals to the new text are "sticky", then the new text retains
   only its properties, as if neither sticky property were set.  Perhaps
   we should consider merging all three sets of properties onto the new
   text...  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn graft_intervals_into_buffer (source: INTERVAL, mut position: ptrdiff_t,
						      length: ptrdiff_t, buffer: *mut buffer,
						      inherit: bool)
{
  let mut tree: INTERVAL = buffer_intervals (buffer);
  let mut under: INTERVAL = std::ptr::null_mut ();
  let mut over: INTERVAL = std::ptr::null_mut ();
  let mut this: INTERVAL = std::ptr::null_mut ();
  let mut over_used: ptrdiff_t = 0;

  /* If the new text has no properties, then with inheritance it
     becomes part of whatever interval it was inserted into.
     To prevent inheritance, we must clear out the properties
     of the newly inserted text.  */
  if source.is_null ()
    {
      let mut buf: Lisp_Object = Qnil ();
      if ! inherit && ! tree.is_null () && length > 0
	{
	  XSETBUFFER! (buf, buffer);
	  set_text_properties_1 (make_fixnum (position),
				 make_fixnum (position + length),
				 Qnil (), buf,
				 find_interval (tree, position));
	}
      /* Shouldn't be necessary.  --Stef  */
      buffer_balance_intervals (buffer);
      return;
    }

  eassert! (length == TOTAL_LENGTH! (source));

  if (BUF_Z! (buffer) - BUF_BEG (buffer)) == length
    {
      /* The inserted text constitutes the whole buffer, so
	 simply copy over the interval structure.  */
      let mut buf: Lisp_Object = Qnil ();

      XSETBUFFER! (buf, buffer);
      set_buffer_intervals (buffer, reproduce_tree_obj (source, buf));
      (*((buffer_intervals (buffer)) as *mut interval)).position = BUF_BEG (buffer);
      eassert! (interval_bits! (up_obj, buffer_intervals (buffer)) == true);
      return;
    }
  else if tree.is_null ()
    {
      /* Create an interval tree in which to place a copy
	 of the intervals of the inserted string.  */
	let mut buf: Lisp_Object = Qnil ();

	XSETBUFFER! (buf, buffer);
	tree = create_root_interval (buf);
    }
  /* Paranoia -- the text has already been added, so
     this buffer should be of non-zero length.  */
  eassert! (TOTAL_LENGTH! (tree) > 0);

  this = find_interval (tree, position);
  under = this;
  eassert! (! under.is_null ());
  over = find_interval (source, interval_start_pos (source) as ptrdiff_t);

  /* Here for insertion in the middle of an interval.
     Split off an equivalent interval to the right,
     then don't bother with it any more.  */

  if position > (*((under) as *mut interval)).position
    {
      let end_unchanged: INTERVAL
	= split_interval_left (this, position - (*((under) as *mut interval)).position);
      copy_properties (under, end_unchanged);
      (*((under) as *mut interval)).position = position;
    }
  else
    {
      /* This call may have some effect because previous_interval may
         update `position' fields of intervals.  Thus, don't ignore it
         for the moment.  Someone please tell me the truth (K.Handa).  */
      let prev: INTERVAL = previous_interval (under);
      let _ = prev;
/* C 1643-1648: `#if 0` - the "this code surely has no effect" block naming END_NONSTICKY_P,
   recorded in docs/ports/intervals.md (the file is in cond-eval-verified.txt, P-08).  */
    }

  /* Insertion is now at beginning of UNDER.  */

  /* The inserted text "sticks" to the interval `under',
     which means it gets those properties.
     The properties of under are the result of
     adjust_intervals_for_insertion, so stickiness has
     already been taken care of.  */

  /* OVER is the interval we are copying from next.
     OVER_USED says how many characters' worth of OVER
     have already been copied into target intervals.
     UNDER is the next interval in the target.  */
  over_used = 0;
  while ! over.is_null ()
    {
      /* If UNDER is longer than OVER, split it.  */
      if LENGTH! (over) - over_used < LENGTH! (under)
	{
	  this = split_interval_left (under, LENGTH! (over) - over_used);
	  copy_properties (under, this);
	}
      else
	{ this = under; }

      /* THIS is now the interval to copy or merge into.
	 OVER covers all of it.  */
      if inherit
	{ merge_properties (over, this); }
      else
	{ copy_properties (over, this); }

      /* If THIS and OVER end at the same place,
	 advance OVER to a new source interval.  */
      if LENGTH! (this) == LENGTH! (over) - over_used
	{
	  over = next_interval (over);
	  over_used = 0;
	}
      else
	/* Otherwise just record that more of OVER has been used.  */
	{ over_used += LENGTH! (this); }

      /* Always advance to a new target interval.  */
      under = next_interval (this);
    }

  buffer_balance_intervals (buffer);
}

/* Get the value of property PROP from PLIST,
   which is the plist of an interval.
   We check for direct properties, for categories with property PROP,
   and for PROP appearing on the default-text-properties list.  */

/* C: lisp.h 4280: `extern Lisp_Object plist_get (Lisp_Object plist, Lisp_Object prop);` -
   defined in fns.c.  The `lisp.rs` mirror does not carry it yet (a mirror gap reported in
   docs/ports/intervals.md, PORTING.md 0.1-2/P-72 rule 3).  */
unsafe extern "C" {
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn textget (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object
{
  return lookup_char_property (plist, prop, true);
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn lookup_char_property (plist: Lisp_Object, prop: Lisp_Object,
					       textprop: bool) -> Lisp_Object
{
  let mut tail: Lisp_Object = Qnil ();
  let mut fallback: Lisp_Object = Qnil ();

  tail = plist;
  while CONSP (tail)
    {
      let mut tem: Lisp_Object = Qnil ();
      tem = XCAR (tail);
      if EQ (prop, tem)
	{ return Fcar (XCDR (tail)); }
      if EQ (tem, Qcategory ())
	{
	  tem = Fcar (XCDR (tail));
	  if SYMBOLP (tem)
	    { fallback = Fget (tem, prop); }
	}
      tail = Fcdr (XCDR (tail));
    }

  if ! NILP (fallback)
    { return fallback; }
  /* Check for alternative properties.  */
  tail = Fassq (prop, Vchar_property_alias_alist);
  if ! NILP (tail)
    {
      tail = XCDR (tail);
      while NILP (fallback) && CONSP (tail)
	{
	  fallback = plist_get (plist, XCAR (tail));
	  tail = XCDR (tail);
	}
    }

  if textprop && NILP (fallback) && CONSP (Vdefault_text_properties)
    { fallback = plist_get (Vdefault_text_properties, prop); }
  return fallback;
}


/* Set point in BUFFER "temporarily" to CHARPOS, which corresponds to
   byte position BYTEPOS.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn temp_set_point_both (buffer: *mut buffer,
					      charpos: ptrdiff_t, bytepos: ptrdiff_t)
{
  /* In a single-byte buffer, the two positions must be equal.  */
  eassert! (BUF_ZV (buffer) != BUF_ZV_BYTE (buffer) || charpos == bytepos);

  eassert! (charpos <= bytepos);
  eassert! (charpos <= BUF_ZV (buffer) || BUF_BEGV (buffer) <= charpos);

  SET_BUF_PT_BOTH (buffer, charpos, bytepos);
}

/* Set point "temporarily", without checking any text properties.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn temp_set_point (buffer: *mut buffer, charpos: ptrdiff_t)
{
  temp_set_point_both (buffer, charpos,
		       buf_charpos_to_bytepos (buffer as *mut c_void, charpos));
}

/* Set point in BUFFER to CHARPOS.  If the target position is
   before an intangible character, move to an ok place.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_point (charpos: ptrdiff_t)
{
  set_point_both (charpos, buf_charpos_to_bytepos ((current_buffer! ()) as *mut c_void, charpos));
}

/* Set PT from MARKER's clipped position.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_point_from_marker (marker: Lisp_Object)
{
  let charpos: ptrdiff_t = clip_to_bounds (BEGV! (), marker_position (marker), ZV! ());
  let mut bytepos: ptrdiff_t = marker_byte_position (marker);

  /* Don't trust the byte position if the marker belongs to a
     different buffer.  */
  if (*XMARKER (marker)).buffer != (current_buffer! ()) as *mut c_void
    { bytepos = buf_charpos_to_bytepos ((current_buffer! ()) as *mut c_void, charpos); }
  else
    { bytepos = clip_to_bounds (BEGV_BYTE! (), bytepos, ZV_BYTE! ()); }
  set_point_both (charpos, bytepos);
}

/* If there's an invisible character at position POS + TEST_OFFS in the
   current buffer, and the invisible property has a `stickiness' such that
   inserting a character at position POS would inherit the property it,
   return POS + ADJ, otherwise return POS.  If TEST_INTANG, intangibility
   is required as well as invisibility.

   TEST_OFFS should be either 0 or -1, and ADJ should be either 1 or -1.

   Note that `stickiness' is determined by overlay marker insertion types,
   if the invisible property comes from an overlay.  */

unsafe fn adjust_for_invis_intang (mut pos: ptrdiff_t, test_offs: ptrdiff_t,
				   adj: ptrdiff_t, test_intang: bool) -> ptrdiff_t
{
  let mut invis_propval: Lisp_Object = Qnil ();
  let mut invis_overlay: Lisp_Object = Qnil ();
  let mut test_pos: Lisp_Object = Qnil ();

  if (adj < 0 && pos + adj < BEGV! ()) || (adj > 0 && pos + adj > ZV! ())
    /* POS + ADJ would be beyond the buffer bounds, so do no adjustment.  */
    { return pos; }

  test_pos = make_fixnum (pos + test_offs);

  invis_propval
    = get_char_property_and_overlay (test_pos, Qinvisible (), Qnil (),
				     &raw mut invis_overlay);

  if (! test_intang
      || ! NILP (Fget_char_property (test_pos, Qintangible (), Qnil ())))
      && TEXT_PROP_MEANS_INVISIBLE! (invis_propval) != 0
      /* This next test is true if the invisible property has a stickiness
	 such that an insertion at POS would inherit it.  */
      && (if NILP (invis_overlay)
	  /* Invisible property is from a text-property.  */
	  { (text_property_stickiness (Qinvisible (), make_fixnum (pos), Qnil ())
	     == (if test_offs == 0 { 1 } else { -1 })) }
	  /* Invisible property is from an overlay.  */
	  else
	  { (if test_offs == 0
	     { ! OVERLAY_FRONT_ADVANCE_P (invis_overlay) }
	     else { OVERLAY_REAR_ADVANCE_P (invis_overlay) }) })
    { pos += adj; }

  return pos;
}


/* Set point in BUFFER to CHARPOS, which corresponds to byte
   position BYTEPOS.  If the target position is
   before an intangible character, move to an ok place.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_point_both (mut charpos: ptrdiff_t, mut bytepos: ptrdiff_t)
{
  let mut to: INTERVAL = std::ptr::null_mut ();
  let mut from: INTERVAL = std::ptr::null_mut ();
  let mut toprev: INTERVAL = std::ptr::null_mut ();
  let mut fromprev: INTERVAL = std::ptr::null_mut ();
  let mut buffer_point: ptrdiff_t = 0;
  let old_position: ptrdiff_t = PT! ();
  /* This ensures that we move forward past intangible text when the
     initial position is the same as the destination, in the rare
     instances where this is important, e.g. in line-move-finish
     (simple.el).  */
  let backwards: bool = charpos < old_position;
  let mut have_overlays: bool = false;
  let mut original_position: ptrdiff_t = 0;

  bset_point_before_scroll ((current_buffer! ()), Qnil ());

  if charpos == PT! ()
    { return; }

  /* In a single-byte buffer, the two positions must be equal.  */
  eassert! (ZV! () != ZV_BYTE! () || charpos == bytepos);

  /* Check this now, before checking if the buffer has any intervals.
     That way, we can catch conditions which break this sanity check
     whether or not there are intervals in the buffer.  */
  eassert! (charpos <= ZV! () && charpos >= BEGV! ());

  have_overlays = buffer_has_overlays ();

  /* If we have no text properties and overlays,
     then we can do it quickly.  */
  if buffer_intervals ((current_buffer! ())).is_null () && ! have_overlays
    {
      temp_set_point_both ((current_buffer! ()), charpos, bytepos);
      return;
    }

  /* Set TO to the interval containing the char after CHARPOS,
     and TOPREV to the interval containing the char before CHARPOS.
     Either one may be null.  They may be equal.  */
  to = find_interval (buffer_intervals ((current_buffer! ())), charpos);
  if charpos == BEGV! ()
    { toprev = 0 as INTERVAL; }
  else if ! to.is_null () && (*((to) as *mut interval)).position == charpos
    { toprev = previous_interval (to); }
  else
    { toprev = to; }

  buffer_point = (if PT! () == ZV! () { ZV! () - 1 } else { PT! () });

  /* Set FROM to the interval containing the char after PT,
     and FROMPREV to the interval containing the char before PT.
     Either one may be null.  They may be equal.  */
  /* We could cache this and save time.  */
  from = find_interval (buffer_intervals ((current_buffer! ())), buffer_point);
  if buffer_point == BEGV! ()
    { fromprev = 0 as INTERVAL; }
  else if ! from.is_null () && (*((from) as *mut interval)).position == PT! ()
    { fromprev = previous_interval (from); }
  else if buffer_point != PT! ()
    { fromprev = from; from = 0 as INTERVAL; }
  else
    { fromprev = from; }

  /* Moving within an interval.  */
  if to == from && toprev == fromprev && INTERVAL_VISIBLE_P! (to)
      && ! have_overlays
    {
      temp_set_point_both ((current_buffer! ()), charpos, bytepos);
      return;
    }

  original_position = charpos;

  /* If the new position is between two intangible characters
     with the same intangible property value,
     move forward or backward until a change in that property.  */
  if NILP (Vinhibit_point_motion_hooks)
      && (((! to.is_null () && ! toprev.is_null ()))
	  || have_overlays)
      /* Intangibility never stops us from positioning at the beginning
	 or end of the buffer, so don't bother checking in that case.  */
      && charpos != BEGV! () && charpos != ZV! ()
    {
      let mut pos: Lisp_Object = Qnil ();
      let mut intangible_propval: Lisp_Object = Qnil ();

      if backwards
	{
	  /* If the preceding character is both intangible and invisible,
	     and the invisible property is `rear-sticky', perturb it so
	     that the search starts one character earlier -- this ensures
	     that point can never move to the end of an invisible/
	     intangible/rear-sticky region.  */
	  charpos = adjust_for_invis_intang (charpos, -1, -1, true);

	  XSETINT! (pos, charpos);

	  /* If following char is intangible,
	     skip back over all chars with matching intangible property.  */

	  intangible_propval = Fget_char_property (pos, Qintangible (), Qnil ());

	  if ! NILP (intangible_propval)
	    {
	      while XFIXNUM (pos) > BEGV! ()
		     && EQ (Fget_char_property (make_fixnum (XFIXNUM (pos) - 1),
						Qintangible (), Qnil ()),
			    intangible_propval)
		{ pos = Fprevious_char_property_change (pos, Qnil ()); }

	      /* Set CHARPOS from POS, and if the final intangible character
		 that we skipped over is also invisible, and the invisible
		 property is `front-sticky', perturb it to be one character
		 earlier -- this ensures that point can never move to the
		 beginning of an invisible/intangible/front-sticky region.  */
	      charpos = adjust_for_invis_intang (XFIXNUM (pos), 0, -1, false);
	    }
	}
      else
	{
	  /* If the following character is both intangible and invisible,
	     and the invisible property is `front-sticky', perturb it so
	     that the search starts one character later -- this ensures
	     that point can never move to the beginning of an
	     invisible/intangible/front-sticky region.  */
	  charpos = adjust_for_invis_intang (charpos, 0, 1, true);

	  XSETINT! (pos, charpos);

	  /* If preceding char is intangible,
	     skip forward over all chars with matching intangible property.  */

	  intangible_propval = Fget_char_property (make_fixnum (charpos - 1),
						   Qintangible (), Qnil ());

	  if ! NILP (intangible_propval)
	    {
	      while XFIXNUM (pos) < ZV! ()
		     && EQ (Fget_char_property (pos, Qintangible (), Qnil ()),
			    intangible_propval)
		{ pos = Fnext_char_property_change (pos, Qnil ()); }

	      /* Set CHARPOS from POS, and if the final intangible character
		 that we skipped over is also invisible, and the invisible
		 property is `rear-sticky', perturb it to be one character
		 later -- this ensures that point can never move to the
		 end of an invisible/intangible/rear-sticky region.  */
	      charpos = adjust_for_invis_intang (XFIXNUM (pos), -1, 1, false);
	    }
	}

      bytepos = buf_charpos_to_bytepos ((current_buffer! ()) as *mut c_void, charpos);
    }

  if charpos != original_position
    {
      /* Set TO to the interval containing the char after CHARPOS,
	 and TOPREV to the interval containing the char before CHARPOS.
	 Either one may be null.  They may be equal.  */
      to = find_interval (buffer_intervals ((current_buffer! ())), charpos);
      if charpos == BEGV! ()
	{ toprev = 0 as INTERVAL; }
      else if ! to.is_null () && (*((to) as *mut interval)).position == charpos
	{ toprev = previous_interval (to); }
      else
	{ toprev = to; }
    }

  /* Here TO is the interval after the stopping point
     and TOPREV is the interval before the stopping point.
     One or the other may be null.  */

  temp_set_point_both ((current_buffer! ()), charpos, bytepos);

  /* We run point-left and point-entered hooks here, if the
     two intervals are not equivalent.  These hooks take
     (old_point, new_point) as arguments.  */
  if NILP (Vinhibit_point_motion_hooks)
      && (! intervals_equal (from, to)
	  || ! intervals_equal (fromprev, toprev))
    {
      let mut leave_after: Lisp_Object = Qnil ();
      let mut leave_before: Lisp_Object = Qnil ();
      let mut enter_after: Lisp_Object = Qnil ();
      let mut enter_before: Lisp_Object = Qnil ();

      if ! fromprev.is_null ()
	{ leave_before = textget ((*((fromprev) as *mut interval)).plist, Qpoint_left ()); }
      else
	{ leave_before = Qnil (); }

      if ! from.is_null ()
	{ leave_after = textget ((*((from) as *mut interval)).plist, Qpoint_left ()); }
      else
	{ leave_after = Qnil (); }

      if ! toprev.is_null ()
	{ enter_before = textget ((*((toprev) as *mut interval)).plist, Qpoint_entered ()); }
      else
	{ enter_before = Qnil (); }

      if ! to.is_null ()
	{ enter_after = textget ((*((to) as *mut interval)).plist, Qpoint_entered ()); }
      else
	{ enter_after = Qnil (); }

      if ! EQ (leave_before, enter_before) && !NILP (leave_before)
	{ calln! (leave_before, make_fixnum (old_position), make_fixnum (charpos)); }
      if ! EQ (leave_after, enter_after) && !NILP (leave_after)
	{ calln! (leave_after, make_fixnum (old_position), make_fixnum (charpos)); }

      if ! EQ (enter_before, leave_before) && !NILP (enter_before)
	{ calln! (enter_before, make_fixnum (old_position), make_fixnum (charpos)); }
      if ! EQ (enter_after, leave_after) && !NILP (enter_after)
	{ calln! (enter_after, make_fixnum (old_position), make_fixnum (charpos)); }
    }
}


/* Move point to POSITION, unless POSITION is inside an intangible
   segment that reaches all the way to point.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn move_if_not_intangible (mut position: ptrdiff_t)
{
  let mut pos: Lisp_Object = Qnil ();
  let mut intangible_propval: Lisp_Object = Qnil ();

  XSETINT! (pos, position);

  if ! NILP (Vinhibit_point_motion_hooks)
    /* If intangible is inhibited, always move point to POSITION.  */
    { }
  else if PT! () < position && XFIXNUM (pos) < ZV! ()
    {
      /* We want to move forward, so check the text before POSITION.  */

      intangible_propval = Fget_char_property (pos,
					       Qintangible (), Qnil ());

      /* If following char is intangible,
	 skip back over all chars with matching intangible property.  */
      if ! NILP (intangible_propval)
	{
	  while XFIXNUM (pos) > BEGV! ()
	       && EQ (Fget_char_property (make_fixnum (XFIXNUM (pos) - 1),
					  Qintangible (), Qnil ()),
		      intangible_propval)
	    { pos = Fprevious_char_property_change (pos, Qnil ()); }
	}
    }
  else if XFIXNUM (pos) > BEGV! ()
    {
      /* We want to move backward, so check the text after POSITION.  */

      intangible_propval = Fget_char_property (make_fixnum (XFIXNUM (pos) - 1),
					       Qintangible (), Qnil ());

      /* If following char is intangible,
	 skip forward over all chars with matching intangible property.  */
      if ! NILP (intangible_propval)
	{
	  while XFIXNUM (pos) < ZV! ()
	       && EQ (Fget_char_property (pos, Qintangible (), Qnil ()),
		      intangible_propval)
	    { pos = Fnext_char_property_change (pos, Qnil ()); }
	}

    }
  else if position < BEGV! ()
    { position = BEGV! (); }
  else if position > ZV! ()
    { position = ZV! (); }

  /* If the whole stretch between PT and POSITION isn't intangible,
     try moving to POSITION (which means we actually move farther
     if POSITION is inside of intangible text).  */

  if XFIXNUM (pos) != PT! ()
    { SET_PT (position); }
}


/* If text at position POS has property PROP, set *VAL to the property
   value, *START and *END to the beginning and end of a region that
   has the same property, and return true.  Otherwise return false.

   OBJECT is the string or buffer to look for the property in;
   nil means the current buffer. */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn get_property_and_range (pos: ptrdiff_t, prop: Lisp_Object,
						 val: *mut Lisp_Object,
						 start: *mut ptrdiff_t, end: *mut ptrdiff_t,
						 object: Lisp_Object) -> bool
{
  let mut i: INTERVAL = std::ptr::null_mut ();
  let mut prev: INTERVAL = std::ptr::null_mut ();
  let mut next: INTERVAL = std::ptr::null_mut ();

  if NILP (object)
    { i = find_interval (buffer_intervals ((current_buffer! ())), pos); }
  else if BUFFERP (object)
    { i = find_interval (buffer_intervals (XBUFFER (object)), pos); }
  else if STRINGP (object)
    { i = find_interval (string_intervals (object), pos); }
  else
    { emacs_abort (); }

  if i.is_null () || ((*((i) as *mut interval)).position + LENGTH! (i) <= pos)
    { return false; }
  *val = textget ((*((i) as *mut interval)).plist, prop);
  if NILP (*val)
    { return false; }

  next = i;			/* remember it in advance */
  prev = previous_interval (i);
  while ! prev.is_null ()
	&& EQ (*val, textget ((*((prev) as *mut interval)).plist, prop))
    { i = prev; prev = previous_interval (prev); }
  *start = (*((i) as *mut interval)).position;

  next = next_interval (i);
  while ! next.is_null ()
	&& EQ (*val, textget ((*((next) as *mut interval)).plist, prop))
    { i = next; next = next_interval (next); }
  *end = (*((i) as *mut interval)).position + LENGTH! (i);

  return true;
}


/* Return the proper local keymap TYPE for position POSITION in
   BUFFER; TYPE should be one of `keymap' or `local-map'.  Use the map
   specified by the TYPE property, if any.  Otherwise, if TYPE is
   `local-map', use BUFFER's local map.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn get_local_map (mut position: ptrdiff_t, buffer: *mut buffer,
					r#type: Lisp_Object) -> Lisp_Object
{
  let mut prop: Lisp_Object = Qnil ();
  let mut lispy_position: Lisp_Object = Qnil ();
  let mut lispy_buffer: Lisp_Object = Qnil ();
  let mut old_begv: ptrdiff_t = 0;
  let mut old_zv: ptrdiff_t = 0;
  let mut old_begv_byte: ptrdiff_t = 0;
  let mut old_zv_byte: ptrdiff_t = 0;
  let count: specpdl_ref = SPECPDL_INDEX ();

  position = clip_to_bounds (BUF_BEGV (buffer), position, BUF_ZV (buffer));

  /* Ignore narrowing, so that a local map continues to be valid even if
     the visible region contains no characters and hence no properties.  */
  old_begv = BUF_BEGV (buffer);
  old_zv = BUF_ZV (buffer);
  old_begv_byte = BUF_BEGV_BYTE (buffer);
  old_zv_byte = BUF_ZV_BYTE (buffer);

  specbind (Qinhibit_quit (), Qt ());
  SET_BUF_BEGV_BOTH (buffer, BUF_BEG (buffer), BUF_BEG_BYTE (buffer));
  SET_BUF_ZV_BOTH (buffer, BUF_Z! (buffer), BUF_Z_BYTE! (buffer));

  XSETFASTINT! (lispy_position, position);
  XSETBUFFER! (lispy_buffer, buffer);
  /* First check if the CHAR has any property.  This is because when
     we click with the mouse, the mouse pointer is really pointing
     to the CHAR after POS.  */
  prop = Fget_char_property (lispy_position, r#type, lispy_buffer);
  /* If not, look at the POS's properties.  This is necessary because when
     editing a field with a `local-map' property, we want insertion at the end
     to obey the `local-map' property.  */
  if NILP (prop)
    { prop = Fget_pos_property (lispy_position, r#type, lispy_buffer); }

  SET_BUF_BEGV_BOTH (buffer, old_begv, old_begv_byte);
  SET_BUF_ZV_BOTH (buffer, old_zv, old_zv_byte);
  unbind_to (count, Qnil ());

  /* Use the local map only if it is valid.  */
  prop = get_keymap (prop, false, false);
  if CONSP (prop)
    { return prop; }

  if EQ (r#type, Qkeymap ())
    { return Qnil (); }
  else
    { return BVAR! (buffer, keymap_); }
}

/* Produce an interval tree reflecting the intervals in
   TREE from START to START + LENGTH.
   The new interval tree has no parent and has a starting-position of 0.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn copy_intervals (tree: INTERVAL, start: ptrdiff_t, length: ptrdiff_t)
  -> INTERVAL
{
  let mut i: INTERVAL = std::ptr::null_mut ();
  let mut new: INTERVAL = std::ptr::null_mut ();
  let mut t: INTERVAL = std::ptr::null_mut ();
  let mut got: ptrdiff_t = 0;
  let mut prevlen: ptrdiff_t = 0;

  if tree.is_null () || length <= 0
    { return std::ptr::null_mut (); }

  i = find_interval (tree, start);
  eassert! (! i.is_null () && LENGTH! (i) > 0);

  /* If there is only one interval and it's the default, return nil.  */
  if (start - (*((i) as *mut interval)).position + 1 + length) < LENGTH! (i)
      && DEFAULT_INTERVAL_P! (i)
    { return std::ptr::null_mut (); }

  new = make_interval ();
  (*((new) as *mut interval)).position = 0;
  got = (LENGTH! (i) - (start - (*((i) as *mut interval)).position));
  (*((new) as *mut interval)).total_length = length;
  eassert! (TOTAL_LENGTH! (new) >= 0);
  copy_properties (i, new);

  t = new;
  prevlen = got;
  while got < length
    {
      i = next_interval (i);
      t = split_interval_right (t, prevlen);
      copy_properties (i, t);
      prevlen = LENGTH! (i);
      got += prevlen;
    }

  return balance_an_interval (new);
}

/* Give STRING the properties of BUFFER from POSITION to LENGTH.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn copy_intervals_to_string (string: Lisp_Object, buffer: *mut buffer,
						   position: ptrdiff_t, length: ptrdiff_t)
{
  let interval_copy: INTERVAL = copy_intervals (buffer_intervals (buffer),
						position, length);
  if interval_copy.is_null ()
    { return; }

  set_interval_object (interval_copy, string);
  set_string_intervals (string, interval_copy);
}


/* Return true if strings S1 and S2 have identical properties.
   Assume they have identical characters.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn compare_string_intervals (s1: Lisp_Object, s2: Lisp_Object) -> bool
{
  let mut i1: INTERVAL = std::ptr::null_mut ();
  let mut i2: INTERVAL = std::ptr::null_mut ();
  let mut pos: ptrdiff_t = 0;
  let end: ptrdiff_t = SCHARS (s1);

  i1 = find_interval (string_intervals (s1), 0);
  i2 = find_interval (string_intervals (s2), 0);

  while pos < end
    {
      /* Determine how far we can go before we reach the end of I1 or I2.  */
      let len1: ptrdiff_t = (if ! i1.is_null () { INTERVAL_LAST_POS! (i1) } else { end }) - pos;
      let len2: ptrdiff_t = (if ! i2.is_null () { INTERVAL_LAST_POS! (i2) } else { end }) - pos;
      let distance: ptrdiff_t = min (len1, len2);

      /* If we ever find a mismatch between the strings,
	 they differ.  */
      if ! intervals_equal_1 (i1, i2, true)
	{ return false; }

      /* Advance POS till the end of the shorter interval,
	 and advance one or both interval pointers for the new position.  */
      pos += distance;
      if len1 == distance
	{ i1 = next_interval (i1); }
      if len2 == distance
	{ i2 = next_interval (i2); }
    }
  return true;
}


/* Recursively adjust interval I in the current buffer
   for setting enable_multibyte_characters to MULTI_FLAG.
   The range of interval I is START ... END in characters,
   START_BYTE ... END_BYTE in bytes.  */

unsafe fn set_intervals_multibyte_1 (i: INTERVAL, multi_flag: bool,
				     start: ptrdiff_t, start_byte: ptrdiff_t,
				     end: ptrdiff_t, end_byte: ptrdiff_t)
{
  /* Fix the length of this interval.  */
  if multi_flag
    { (*((i) as *mut interval)).total_length = end - start; }
  else
    { (*((i) as *mut interval)).total_length = end_byte - start_byte; }
  eassert! (TOTAL_LENGTH! (i) >= 0);

  if TOTAL_LENGTH! (i) == 0
    {
      /* Delete the whole subtree.  */
      (*((i) as *mut interval)).left = std::ptr::null_mut ();
      (*((i) as *mut interval)).right = std::ptr::null_mut ();
      delete_interval (i);
      return;
    }

  /* Recursively fix the length of the subintervals.  */
  if ! (*((i) as *mut interval)).left.is_null ()
    {
      let mut left_end: ptrdiff_t = 0;
      let mut left_end_byte: ptrdiff_t = 0;

      if multi_flag
	{
	  left_end_byte
	    = advance_to_char_boundary (start_byte + LEFT_TOTAL_LENGTH! (i));
	  left_end = BYTE_TO_CHAR (left_end_byte);
	  eassert! (CHAR_TO_BYTE (left_end) == left_end_byte);
	}
      else
	{
	  left_end = start + LEFT_TOTAL_LENGTH! (i);
	  left_end_byte = CHAR_TO_BYTE (left_end);
	}

      set_intervals_multibyte_1 ((*((i) as *mut interval)).left as INTERVAL, multi_flag,
				 start, start_byte, left_end, left_end_byte);
    }
  if ! (*((i) as *mut interval)).right.is_null ()
    {
      let mut right_start_byte: ptrdiff_t = 0;
      let mut right_start: ptrdiff_t = 0;

      if multi_flag
	{
	  right_start_byte
	    = advance_to_char_boundary (end_byte - RIGHT_TOTAL_LENGTH! (i));
	  right_start = BYTE_TO_CHAR (right_start_byte);
	  eassert! (CHAR_TO_BYTE (right_start) == right_start_byte);
	}
      else
	{
	  right_start = end - RIGHT_TOTAL_LENGTH! (i);
	  right_start_byte = CHAR_TO_BYTE (right_start);
	}

      set_intervals_multibyte_1 ((*((i) as *mut interval)).right as INTERVAL, multi_flag,
				 right_start, right_start_byte,
				 end, end_byte);
    }

  /* Rounding to char boundaries can theoretically make this interval
     spurious.  If so, delete one child, and copy its property list
     to this interval.  */
  if LEFT_TOTAL_LENGTH! (i) + RIGHT_TOTAL_LENGTH! (i) >= TOTAL_LENGTH! (i)
    {
      if ! (*((i) as *mut interval)).left.is_null ()
	{
	  set_interval_plist (i, (*((*((i) as *mut interval)).left)).plist);
	  (*((*((i) as *mut interval)).left)).total_length = 0;
	  delete_interval ((*((i) as *mut interval)).left as INTERVAL);
	}
      else
	{
	  eassume! (! (*((i) as *mut interval)).right.is_null ());
	  set_interval_plist (i, (*((*((i) as *mut interval)).right)).plist);
	  (*((*((i) as *mut interval)).right)).total_length = 0;
	  delete_interval ((*((i) as *mut interval)).right as INTERVAL);
	}
    }
}

/* Update the intervals of the current buffer
   to fit the contents as multibyte (if MULTI_FLAG)
   or to fit them as non-multibyte (if not MULTI_FLAG).  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_intervals_multibyte (multi_flag: bool)
{
  let i: INTERVAL = buffer_intervals ((current_buffer! ()));

  if ! i.is_null ()
    { set_intervals_multibyte_1 (i, multi_flag, BEG as ptrdiff_t, BEG_BYTE as ptrdiff_t, Z! (), Z_BYTE! ()); }
}
