// femacs: port of src/scroll.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The three
// non-`static` definitions (`scrolling_1`, `scrolling_max_lines_saved`,
// `do_line_insertion_deletion_costs`) are C's exported symbols, so they carry
// `#[unsafe(no_mangle)]` (PORTING.md 0.2/P-52); the six `static` helpers stay plain
// `unsafe fn`.
//
// The `#include` list mirrors scroll.c 20-28 - config.h, lisp.h, termchar.h, dispextern.h,
// frame.h, termhooks.h - through the mirrors those pull in; `termchar.rs` carries
// dispextern.h, termhooks.h and frame.h itself (termchar.h 23), so naming it once is C's
// inclusion graph (Rust items are order-independent, the P-61 note).
//
// `#ifndef HAVE_ANDROID` (C 26) wraps the whole file and C's `#endif` (C 993) closes it;
// C 26-28 and the `#endif` have no text between them.  HAVE_ANDROID is undefined in the
// frozen configuration, so every line below is live text and the dead arm is an empty
// translation unit - there is nothing to gate.
//
// `#ifdef GLYPH_DEBUG` at C 262-320 (`CHECK_BOUNDS` and its one use at C 372-373) is dead on
// the frozen target: GLYPH_DEBUG is `#undef` in config.h, so `CHECK_BOUNDS` is never defined.
// Its text stays behind `feature = "glyph_debug"`, the feature that names the non-frozen side
// (P-51), and the same feature is the one dispextern.rs uses for `CHECK_MATRIX`'s dead arm.
// `CHECK_MATRIX` itself is *used* live (C 372-373 is the only gated use; C 676 and C 298 of
// do_scrolling are live calls whose macro body is C's `#else` arm, `((void) 0)`).
//
// Five names this unit calls are declared in dispextern.h C 3905/3936/3945/3953-3954, which
// the dispextern mirror does not carry yet, so each is declared at its use site (the
// doc.rs/casefiddle.rs shape): `mirrored_line_dance`, `set_terminal_window`, `ins_del_lines`,
// `string_cost`, `per_line_cost`.  `frame.current_matrix` is frame.rs's `*mut c_void`
// (P-72 rule 3: frame.h cannot name `struct glyph_matrix`), so the use site casts once.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();
// [U1] ../lib/include/stdio.rs imported via common
// lisp.h 30 includes <string.h>: C's `memset` over `retained_p` and the hash table.
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/stdcountof.rs imported via common
// lisp.h: `baud_rate` (globals.h), `INT_MAX` (<limits.h> 33 through <inttypes.h>),
// `clip_to_bounds`, `min`, the `SAFE_*`/`USE_SAFE_ALLOCA` family, `xnrealloc`,
// `emacs_abort` and `eassert`.
// [U1] include/lisp.rs imported via common
// dispextern.h 25 includes character.h; termhooks.h 28 includes systime.h for `Time`;
// frame.h 22-23 includes termhooks.h and window.h.  The port's include list is that chain,
// and the mirrors name a few types from headers C reaches indirectly (coding.h's
// `coding_system` through termhooks.h's pointer members, the P-72 hazard doc.rs records):
// the includes below are the same set callint.rs/doc.rs proved (PORTING.md P-61).
// [U1] include/character.rs imported via common
// [U1] include/itree.rs imported via common
// window.rs's BUFFERP/buffer_local_value come from buffer.h (frame.h 23 includes window.h,
// which includes buffer.h); doc.rs's list carries it for the same reason.
// [U1] include/buffer.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/frame.rs imported via common
// termchar.h 23 includes dispextern.h and is written for a unit that has already seen
// frame.h/termhooks.h (its FRAME_TTY reads them); naming it last is C's own order.
// [U1] include/termchar.rs imported via common

/* The matrix_elt struct is used to determine the cost of
   updating the screen from the current matrix to the desired
   matrix.  */

/* C 34-51.  The port keeps C's declaration order and the six `int` members (the layout is
   asserted for the mirror types; this one is unit-local, so the field order below is C's by
   inspection, and the comments are C's own).  */
#[repr(C)]
pub struct matrix_elt
  {
    /* Cost of outputting through this line
       if no insert/delete is done just above it.  */
    pub writecost: c_int,
    /* Cost of outputting through this line
       if an insert is done just above it.  */
    pub insertcost: c_int,
    /* Cost of outputting through this line
       if a delete is done just above it.  */
    pub deletecost: c_int,
    /* Number of inserts so far in this run of inserts,
       for the cost in insertcost.  */
    pub insertcount: c_int,
    /* Number of deletes so far in this run of deletes,
       for the cost in deletecost.  */
    pub deletecount: c_int,
    /* Number of writes so far since the last insert
       or delete for the cost in writecost. */
    pub writecount: c_int,
  }

/* C 52-57: the two forward declarations of this file's `static` helpers.  Rust has no
   forward declarations: the definitions below are the same items (PORTING.md 0.1-12, the
   region-cache.rs note).  */

/* Determine, in matrix[i,j], the cost of updating the first j old
   lines into the first i new lines using the general scrolling method.
   This involves using insert or delete somewhere if i != j.
   For each matrix elements, three kinds of costs are recorded:
   the smallest cost that ends with an insert, the smallest
   cost that ends with a delete, and the smallest cost that
   ends with neither one.  These are kept separate because
   on some terminals the cost of doing an insert varies
   depending on whether one was just done, etc.  */

/* draw_cost[VPOS] is the cost of outputting new line at VPOS.
   old_hash[VPOS] is the hash code of the old line at VPOS.
   new_hash[VPOS] is the hash code of the new line at VPOS.
   Note that these are not true frame vpos's, but relative
   to the place at which the first mismatch between old and
   new contents appears.  */

#[cfg (not (feature = "have_android"))]
unsafe fn calculate_scrolling (frame: *mut frame,
			       /* matrix is of size window_size + 1 on each side.  */
			       matrix: *mut matrix_elt,
			       window_size: c_int, lines_below: c_int,
			       draw_cost: *mut c_int, old_hash: *mut c_uint,
			       new_hash: *mut c_uint,
			       free_at_end: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut i: c_int;
  let mut j: c_int;
  let frame_total_lines: c_int = FRAME_TOTAL_LINES (frame);
  let mut p: *mut matrix_elt;
  let mut p1: *mut matrix_elt;
  let mut cost: c_int;
  let mut cost1: c_int;

  let lines_moved: c_int = window_size
    + (if FRAME_SCROLL_REGION_OK! (frame) { 0 } else { lines_below });
  /* first_insert_cost[I] is the cost of doing the first insert-line
     at the i'th line of the lines we are considering,
     where I is origin 1 (as it is below).  */
  let first_insert_cost: *mut c_int
    = FRAME_INSERT_COST (frame).offset ((frame_total_lines - 1 - lines_moved) as isize);
  let first_delete_cost: *mut c_int
    = FRAME_DELETE_COST (frame).offset ((frame_total_lines - 1 - lines_moved) as isize);
  let next_insert_cost: *mut c_int
    = FRAME_INSERTN_COST (frame).offset ((frame_total_lines - 1 - lines_moved) as isize);
  let next_delete_cost: *mut c_int
    = FRAME_DELETEN_COST (frame).offset ((frame_total_lines - 1 - lines_moved) as isize);

  /* Discourage long scrolls on fast lines.
     Don't scroll nearly a full frame height unless it saves
     at least 1/4 second.  */
  let extra_cost: c_int
    = clip_to_bounds (1, (*baud_rate.as_mut_ptr ()) / (10 * 4) / (frame_total_lines as isize),
		      (INT_MAX / 2) as isize) as c_int;

  /* initialize the top left corner of the matrix */
  (*matrix).writecost = 0;
  (*matrix).insertcost = SCROLL_INFINITY;
  (*matrix).deletecost = SCROLL_INFINITY;
  (*matrix).insertcount = 0;
  (*matrix).deletecount = 0;

  /* initialize the left edge of the matrix */
  cost = *first_insert_cost.offset (1) - *next_insert_cost.offset (1);
  i = 1;
  while i <= window_size
    {
      p = matrix.offset ((i * (window_size + 1)) as isize);
      cost += *draw_cost.offset (i as isize) + *next_insert_cost.offset (i as isize) + extra_cost;
      (*p).insertcost = cost;
      (*p).writecost = SCROLL_INFINITY;
      (*p).deletecost = SCROLL_INFINITY;
      (*p).insertcount = i;
      (*p).deletecount = 0;
      i += 1;
    }

  /* initialize the top edge of the matrix */
  cost = *first_delete_cost.offset (1) - *next_delete_cost.offset (1);
  j = 1;
  while j <= window_size
    {
      cost += *next_delete_cost.offset (j as isize);
      (*matrix.offset (j as isize)).deletecost = cost;
      (*matrix.offset (j as isize)).writecost = SCROLL_INFINITY;
      (*matrix.offset (j as isize)).insertcost = SCROLL_INFINITY;
      (*matrix.offset (j as isize)).deletecount = j;
      (*matrix.offset (j as isize)).insertcount = 0;
      j += 1;
    }

  /* `i' represents the vpos among new frame contents.
     `j' represents the vpos among the old frame contents.  */
  p = matrix.offset ((window_size + 2) as isize);	/* matrix [1, 1] */
  i = 1;
  while i <= window_size
    {
      j = 1;
      while j <= window_size
	{
	  /* p contains the address of matrix [i, j] */

	  /* First calculate the cost assuming we do
	     not insert or delete above this line.
	     That is, if we update through line i-1
	     based on old lines through j-1,
	     and then just change old line j to new line i.  */
	  p1 = p.offset (-(window_size as isize) - 2); /* matrix [i-1, j-1] */
	  cost = (*p1).writecost;
	  if cost > (*p1).insertcost
	    { cost = (*p1).insertcost; }
	  if cost > (*p1).deletecost
	    { cost = (*p1).deletecost; }
	  if *old_hash.offset (j as isize) != *new_hash.offset (i as isize)
	    { cost += *draw_cost.offset (i as isize); }
	  (*p).writecost = cost;

	  /* Calculate the cost if we do an insert-line
	     before outputting this line.
	     That is, we update through line i-1
	     based on old lines through j,
	     do an insert-line on line i,
	     and then output line i from scratch,
	     leaving old lines starting from j for reuse below.  */
	  p1 = p.offset (-(window_size as isize) - 1); /* matrix [i-1, j] */
	  /* No need to think about doing a delete followed
	     immediately by an insert.  It cannot be as good
	     as not doing either of them.  */
	  if free_at_end == i
	    {
	      cost = (*p1).writecost;
	      cost1 = (*p1).insertcost;
	    }
	  else
	    {
	      cost = (*p1).writecost + *first_insert_cost.offset (-(i as isize));
	      if (*p1).insertcount > i
		{ emacs_abort (); }
	      cost1 = (*p1).insertcost
		+ *next_insert_cost.offset (-((i - (*p1).insertcount) as isize));
	    }
	  (*p).insertcost = min (cost as isize, cost1 as isize) as c_int
	    + *draw_cost.offset (i as isize) + extra_cost;
	  (*p).insertcount = if cost < cost1 { 1 } else { (*p1).insertcount + 1 };
	  if (*p).insertcount > i
	    { emacs_abort (); }

	  /* Calculate the cost if we do a delete line after
	     outputting this line.
	     That is, we update through line i
	     based on old lines through j-1,
	     and throw away old line j.  */
	  p1 = p.offset (-1);		/* matrix [i, j-1] */
	  /* No need to think about doing an insert followed
	     immediately by a delete.  */
	  if free_at_end == i
	    {
	      cost = (*p1).writecost;
	      cost1 = (*p1).deletecost;
	    }
	  else
	    {
	      cost = (*p1).writecost + *first_delete_cost.offset (-(i as isize));
	      cost1 = (*p1).deletecost + *next_delete_cost.offset (-(i as isize));
	    }
	  (*p).deletecost = min (cost as isize, cost1 as isize) as c_int;
	  (*p).deletecount = if cost < cost1 { 1 } else { (*p1).deletecount + 1 };
	  j += 1;
	  p = p.offset (1);
	}
      i += 1;
      p = p.offset (1);
    }

  }
}


/* Perform insert-lines and delete-lines operations on CURRENT_MATRIX
   according to the costs in MATRIX, using the general scrolling
   method that is used if the terminal does not support the setting of
   scroll windows (scroll_region_ok == 0).

   WINDOW_SIZE is the number of lines being considered for scrolling
   and UNCHANGED_AT_TOP is the vpos of the first line being
   considered.  These two arguments can specify any contiguous range
   of lines.  */

#[cfg (not (feature = "have_android"))]
unsafe fn do_scrolling (frame: *mut frame, current_matrix: *mut glyph_matrix,
			matrix: *mut matrix_elt, window_size: c_int,
			unchanged_at_top: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut p: *mut matrix_elt;
  let mut i: c_int;
  let mut j: c_int;
  let mut k: c_int;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  /* True if we have set a terminal window with set_terminal_window.  */
  let mut terminal_window_p: bool = false;

  /* A queue for line insertions to be done.  */
  #[repr(C)]
  struct queue { count: c_int, pos: c_int }
  let mut queue_start: *mut queue = std::ptr::null_mut ();
  SAFE_NALLOCA! (sa_avail, queue_start, 1, (*current_matrix).nrows);
  let mut queue: *mut queue = queue_start;

  let retained_p: *mut c_char = SAFE_ALLOCA! (sa_avail, window_size) as *mut c_char;
  let mut copy_from: *mut c_int = std::ptr::null_mut ();
  SAFE_NALLOCA! (sa_avail, copy_from, 1, window_size);

  /* Zero means line is empty.  */
  memset (retained_p as *mut c_void, 0,
	  (window_size as usize) * std::mem::size_of::<c_char> ());
  k = 0;
  while k < window_size
    {
      *copy_from.offset (k as isize) = -1;
      k += 1;
    }

  // C 262-320: the `#ifdef GLYPH_DEBUG` arm that defines CHECK_BOUNDS (dead on the frozen
  // target, P-51), and its use at C 372-373 below.  A `#[cfg]`-false `macro_rules!` item is
  // still textually in scope for the gated use, so the two arms stay paired.
  #[cfg (feature = "glyph_debug")]
  macro_rules! CHECK_BOUNDS {
    () => {
      {
	let mut ck: c_int = 0;
	while ck < window_size
	  {
	    eassert! (*copy_from.offset (ck as isize) == -1
		      || (*copy_from.offset (ck as isize) >= 0
			  && *copy_from.offset (ck as isize) < window_size));
	    ck += 1;
	  }
      }
    };
  }

  // dispextern.h 3905/3936/3945: the three terminal entry points this function calls, none
  // of which the dispextern mirror carries yet; declaring them at the use site is the
  // doc.rs/casefiddle.rs shape (the mirror's own term.c section will carry them later).
  unsafe extern "C-unwind" {
    /// C: void mirrored_line_dance (struct frame *f, int, int, int *, char *)
    pub fn mirrored_line_dance (f: *mut frame, unchanged_at_top: c_int, window_size: c_int,
				copy_from: *mut c_int, retained_p: *mut c_char);
    /// C: extern void set_terminal_window (struct frame *, int)
    pub fn set_terminal_window (frame: *mut frame, size: c_int);
    /// C: extern void ins_del_lines (struct frame *, int, int)
    pub fn ins_del_lines (frame: *mut frame, vpos: c_int, n: c_int);
  }

  /* When j is advanced, this corresponds to deleted lines.
     When i is advanced, this corresponds to inserted lines.  */
  i = window_size;
  j = window_size;
  while i > 0 || j > 0
    {
      p = matrix.offset ((i * (window_size + 1) + j) as isize);

      if (*p).insertcost < (*p).writecost && (*p).insertcost < (*p).deletecost
	{
	  /* Insert should be done at vpos i-1, plus maybe some before.
	     Queue the screen operation to be performed.  */
	  (*queue).count = (*p).insertcount;
	  (*queue).pos = i + unchanged_at_top - (*p).insertcount;
	  queue = queue.offset (1);

	  /* By incrementing I, we leave room in the result rows
	     for the empty rows opened up.  */
	  i -= (*p).insertcount;
	}
      else if (*p).deletecost < (*p).writecost
	{
	  /* Old line at vpos j-1, and maybe some before it, should be
	     deleted.  By decrementing J, we skip some lines in the
	     temp_rows which is equivalent to omitting these lines in
	     the result rows, thus deleting them.  */
	  j -= (*p).deletecount;

	  /* Set the terminal window, if not done already.  */
 	  if !terminal_window_p
	    {
	      set_terminal_window (frame, window_size + unchanged_at_top);
	      terminal_window_p = true;
	    }

	  /* Delete lines on the terminal.  */
	  ins_del_lines (frame, j + unchanged_at_top, - (*p).deletecount);
	}
      else
	{
	  /* Best thing done here is no insert or delete, i.e. a write.  */
	  i -= 1;
	  j -= 1;
	  eassert! (i >= 0 && i < window_size);
	  eassert! (j >= 0 && j < window_size);
	  *copy_from.offset (i as isize) = j;
	  *retained_p.offset (j as isize) = 1;

	  #[cfg (feature = "glyph_debug")]
	  { CHECK_BOUNDS! (); }
	}
    }

  /* Now do all insertions queued above.  */
  if queue > queue_start
    {
      let mut next: c_int = -1;

      /* Set the terminal window if not yet done.  */
      if !terminal_window_p
	{
	  set_terminal_window (frame, window_size + unchanged_at_top);
	  terminal_window_p = true;
	}

      loop
	{
	  queue = queue.offset (-1);

	  /* Do the deletion on the terminal.  */
	  ins_del_lines (frame, (*queue).pos, (*queue).count);

	  /* All lines in the range deleted become empty in the glyph
	     matrix.  Assign to them glyph rows that are not retained.
	     K is the starting position of the deleted range relative
	     to the window we are working in.  */
	  k = (*queue).pos - unchanged_at_top;
	  j = 0;
	  while j < (*queue).count
	    {
	      /* Find the next row not retained.  */
	      while { next += 1; *retained_p.offset (next as isize) != 0 }
		{ }

	      /* Record that this row is to be used for the empty
		 glyph row j.  */
	      *copy_from.offset ((k + j) as isize) = next;
	      j += 1;
	    }

	  if !(queue > queue_start)
	    { break; }
	}

    }

  k = 0;
  while k < window_size
    {
      eassert! (*copy_from.offset (k as isize) >= 0
		&& *copy_from.offset (k as isize) < window_size);
      k += 1;
    }

  /* Perform the row swizzling.  */
  mirrored_line_dance (frame, unchanged_at_top, window_size,
		       copy_from, retained_p);

  /* Some sanity checks if GLYPH_DEBUG is defined.  */
  CHECK_MATRIX! (current_matrix);

  if terminal_window_p
    { set_terminal_window (frame, 0); }
  SAFE_FREE! (sa_count, sa_mark);

  }
}


/* Determine, in matrix[i,j], the cost of updating the first j
   old lines into the first i new lines using the direct
   scrolling method.  When the old line and the new line have
   different hash codes, the calculated cost of updating old
   line j into new line i includes the cost of outputting new
   line i, and if i != j, the cost of outputting the old line j
   is also included, as a penalty for moving the line and then
   erasing it.  In addition, the cost of updating a sequence of
   lines with constant i - j includes the cost of scrolling the
   old lines into their new positions, unless i == j.  Scrolling
   is achieved by setting the screen window to avoid affecting
   other lines below, and inserting or deleting lines at the top
   of the scrolled region.  The cost of scrolling a sequence of
   lines includes the fixed cost of specifying a scroll region,
   plus a variable cost which can depend upon the number of lines
   involved and the distance by which they are scrolled, and an
   extra cost to discourage long scrolls.

   As reflected in the matrix, an insert or delete does not
   correspond directly to the insertion or deletion which is
   used in scrolling lines.  An insert means that the value of i
   has increased without a corresponding increase in the value
   of j.  A delete means that the value of j has increased
   without a corresponding increase in the value of i.  A write
   means that i and j are both increased by the same amount, and
   that the old lines will be moved to their new positions.

   An insert following a delete is allowed only if i > j.
   A delete following an insert is allowed only if i < j.
   These restrictions ensure that the new lines in an insert
   will always be blank as an effect of the neighboring writes.
   Thus the calculated cost of an insert is simply the cost of
   outputting the new line contents.  The direct cost of a
   delete is zero.  Inserts and deletes indirectly affect the
   total cost through their influence on subsequent writes. */

/* The vectors draw_cost, old_hash, and new_hash have the same
   meanings here as in calculate_scrolling, and old_draw_cost
   is the equivalent of draw_cost for the old line contents */

#[cfg (not (feature = "have_android"))]
unsafe fn calculate_direct_scrolling (frame: *mut frame,
				      /* matrix is of size window_size + 1 on each side.  */
				      matrix: *mut matrix_elt,
				      window_size: c_int, lines_below: c_int,
				      draw_cost: *mut c_int, old_draw_cost: *mut c_int,
				      old_hash: *mut c_uint, new_hash: *mut c_uint,
				      free_at_end: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut i: c_int;
  let mut j: c_int;
  let frame_total_lines: c_int = FRAME_TOTAL_LINES (frame);
  let mut p: *mut matrix_elt;
  let mut p1: *mut matrix_elt;
  let mut cost: c_int;
  let mut cost1: c_int;
  let mut delta: c_int;

  /* first_insert_cost[-I] is the cost of doing the first insert-line
     at a position I lines above the bottom line in the scroll window. */
  let first_insert_cost: *mut c_int
    = FRAME_INSERT_COST (frame).offset ((frame_total_lines - 1) as isize);
  let first_delete_cost: *mut c_int
    = FRAME_DELETE_COST (frame).offset ((frame_total_lines - 1) as isize);
  let next_insert_cost: *mut c_int
    = FRAME_INSERTN_COST (frame).offset ((frame_total_lines - 1) as isize);
  let next_delete_cost: *mut c_int
    = FRAME_DELETEN_COST (frame).offset ((frame_total_lines - 1) as isize);

  let scroll_overhead: c_int;

  /* Discourage long scrolls on fast lines.
     Don't scroll nearly a full frame height unless it saves
     at least 1/4 second.  */
  let extra_cost: c_int
    = clip_to_bounds (1, (*baud_rate.as_mut_ptr ()) / (10 * 4) / (frame_total_lines as isize),
		      (INT_MAX / 2) as isize) as c_int;

  /* Overhead of setting the scroll window, plus the extra
     cost of scrolling by a distance of one.  The extra cost is
     added once for consistency with the cost vectors */
  scroll_overhead
    = FRAME_SCROLL_REGION_COST! (frame) + extra_cost;

  /* initialize the top left corner of the matrix */
  (*matrix).writecost = 0;
  (*matrix).insertcost = SCROLL_INFINITY;
  (*matrix).deletecost = SCROLL_INFINITY;
  (*matrix).writecount = 0;
  (*matrix).insertcount = 0;
  (*matrix).deletecount = 0;

  /* initialize the left edge of the matrix */
  cost = 0;
  i = 1;
  while i <= window_size
    {
      p = matrix.offset ((i * (window_size + 1)) as isize);
      cost += *draw_cost.offset (i as isize);
      (*p).insertcost = cost;
      (*p).writecost = SCROLL_INFINITY;
      (*p).deletecost = SCROLL_INFINITY;
      (*p).insertcount = i;
      (*p).writecount = 0;
      (*p).deletecount = 0;
      i += 1;
    }

  /* initialize the top edge of the matrix */
  j = 1;
  while j <= window_size
    {
      (*matrix.offset (j as isize)).deletecost = 0;
      (*matrix.offset (j as isize)).writecost = SCROLL_INFINITY;
      (*matrix.offset (j as isize)).insertcost = SCROLL_INFINITY;
      (*matrix.offset (j as isize)).deletecount = j;
      (*matrix.offset (j as isize)).writecount = 0;
      (*matrix.offset (j as isize)).insertcount = 0;
      j += 1;
    }

  /* `i' represents the vpos among new frame contents.
     `j' represents the vpos among the old frame contents.  */
  p = matrix.offset ((window_size + 2) as isize);	/* matrix [1, 1] */

  i = 1;
  while i <= window_size
    {
      j = 1;
      while j <= window_size
	{
	  /* p contains the address of matrix [i, j] */

	  /* First calculate the cost assuming we do
	     not insert or delete above this line.
	     That is, if we update through line i-1
	     based on old lines through j-1,
	     and then just change old line j to new line i.

	     Depending on which choice gives the lower cost,
	     this usually involves either scrolling a single line
	     or extending a sequence of scrolled lines, but
	     when i == j, no scrolling is required. */
	  p1 = p.offset (-(window_size as isize) - 2); /* matrix [i-1, j-1] */
	  cost = (*p1).insertcost;
	  if cost > (*p1).deletecost
	    { cost = (*p1).deletecost; }
	  cost1 = (*p1).writecost;
	  if i == j
	    {
	      if cost > cost1
		{
		  cost = cost1;
		  (*p).writecount = (*p1).writecount + 1;
		}
	      else
		{ (*p).writecount = 1; }
	      if *old_hash.offset (j as isize) != *new_hash.offset (i as isize)
		{
		  cost += *draw_cost.offset (i as isize);
		}
	    }
	  else
	    {
	      if i > j
		{
		  delta = i - j;

		  /* The cost added here for scrolling the first line by
		     a distance N includes the overhead of setting the
		     scroll window, the cost of inserting N lines at a
		     position N lines above the bottom line of the window,
		     and an extra cost which is proportional to N. */
		  cost += scroll_overhead + *first_insert_cost.offset (-(delta as isize))
		    + (delta - 1) * (*next_insert_cost.offset (-(delta as isize)) + extra_cost);

		  /* In the most general case, the insertion overhead and
		     the multiply factor can grow linearly as the distance
		     from the bottom of the window increases.  The incremental
		     cost of scrolling an additional line depends upon the
		     rate of change of these two parameters.  Each of these
		     growth rates can be determined by a simple difference.
		     To reduce the cumulative effects of rounding error, we
		     vary the position at which the difference is computed. */
		  cost1 += *first_insert_cost.offset (-(j as isize))
		    - *first_insert_cost.offset (-((j - 1) as isize))
		    + (delta - 1) * (*next_insert_cost.offset (-(j as isize))
				     - *next_insert_cost.offset (-((j - 1) as isize)));
		}
	      else
		{
		  delta = j - i;
		  cost += scroll_overhead + *first_delete_cost.offset (-(delta as isize))
		    + (delta - 1) * (*next_delete_cost.offset (-(delta as isize)) + extra_cost);
		  cost1 += *first_delete_cost.offset (-(i as isize))
		    - *first_delete_cost.offset (-((i - 1) as isize))
		    + (delta - 1) * (*next_delete_cost.offset (-(i as isize))
				     - *next_delete_cost.offset (-((i - 1) as isize)));
		}
	      if cost1 < cost
		{
		  cost = cost1;
		  (*p).writecount = (*p1).writecount + 1;
		}
	      else
		{ (*p).writecount = 1; }
	      if *old_hash.offset (j as isize) != *new_hash.offset (i as isize)
		{
		  cost += *draw_cost.offset (i as isize) + *old_draw_cost.offset (j as isize);
		}
	    }
	  (*p).writecost = cost;

	  /* Calculate the cost if we do an insert-line
	     before outputting this line.
	     That is, we update through line i-1
	     based on old lines through j,
	     do an insert-line on line i,
	     and then output line i from scratch,
	     leaving old lines starting from j for reuse below.  */
	  p1 = p.offset (-(window_size as isize) - 1); /* matrix [i-1, j] */
	  cost = (*p1).writecost;
	  /* If i > j, an insert is allowed after a delete. */
	  if i > j && (*p1).deletecost < cost
	    { cost = (*p1).deletecost; }
	  if (*p1).insertcost <= cost
	    {
	      cost = (*p1).insertcost;
	      (*p).insertcount = (*p1).insertcount + 1;
	    }
	  else
	    { (*p).insertcount = 1; }
	  cost += *draw_cost.offset (i as isize);
	  (*p).insertcost = cost;

	  /* Calculate the cost if we do a delete line after
	     outputting this line.
	     That is, we update through line i
	     based on old lines through j-1,
	     and throw away old line j.  */
	  p1 = p.offset (-1);		/* matrix [i, j-1] */
	  cost = (*p1).writecost;
	  /* If i < j, a delete is allowed after an insert. */
	  if i < j && (*p1).insertcost < cost
	    { cost = (*p1).insertcost; }
	  cost1 = (*p1).deletecost;
	  if (*p1).deletecost <= cost
	    {
	      cost = (*p1).deletecost;
	      (*p).deletecount = (*p1).deletecount + 1;
	    }
	  else
	    { (*p).deletecount = 1; }
	  (*p).deletecost = cost;
	  j += 1;
	  p = p.offset (1);
	}
      i += 1;
      p = p.offset (1);
    }

  }
}


/* Perform insert-lines and delete-lines operations on CURRENT_MATRIX
   according to the costs in MATRIX, using the direct scrolling method
   which is used when the terminal supports setting a scroll window
   (scroll_region_ok).

   WINDOW_SIZE is the number of lines being considered for scrolling
   and UNCHANGED_AT_TOP is the vpos of the first line being
   considered.  These two arguments can specify any contiguous range
   of lines.

   In the direct scrolling method, a new scroll window is selected
   before each insertion or deletion, so that groups of lines can be
   scrolled directly to their final vertical positions.  This method
   is described in more detail in calculate_direct_scrolling, where
   the cost matrix for this approach is constructed. */

#[cfg (not (feature = "have_android"))]
unsafe fn do_direct_scrolling (frame: *mut frame, current_matrix: *mut glyph_matrix,
			       cost_matrix: *mut matrix_elt, window_size: c_int,
			       unchanged_at_top: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut p: *mut matrix_elt;
  let mut i: c_int;
  let mut j: c_int;
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  /* A queue of deletions and insertions to be performed.  */
  #[repr(C)]
  struct alt_queue { count: c_int, pos: c_int, window: c_int }
  let mut queue_start: *mut alt_queue = std::ptr::null_mut ();
  SAFE_NALLOCA! (sa_avail, queue_start, 1, window_size);
  let mut queue: *mut alt_queue = queue_start;

  /* True if a terminal window has been set with set_terminal_window.  */
  let mut terminal_window_p: bool = false;

  /* If true, a write has been selected, allowing either an insert or a
     delete to be selected next.  If false, a delete cannot be selected
     unless j < i, and an insert cannot be selected unless i < j.
     This corresponds to a similar restriction (with the ordering
     reversed) in calculate_direct_scrolling, which is intended to
     ensure that lines marked as inserted will be blank. */
  let mut write_follows_p: bool = true;

  /* For each row in the new matrix what row of the old matrix it is.  */
  let mut copy_from: *mut c_int = std::ptr::null_mut ();
  SAFE_NALLOCA! (sa_avail, copy_from, 1, window_size);

  /* Non-zero for each row in the new matrix that is retained from the
     old matrix.  Lines not retained are empty.  */
  let retained_p: *mut c_char = SAFE_ALLOCA! (sa_avail, window_size) as *mut c_char;

  memset (retained_p as *mut c_void, 0,
	  (window_size as usize) * std::mem::size_of::<c_char> ());

  /* Perform some sanity checks when GLYPH_DEBUG is on.  */
  CHECK_MATRIX! (current_matrix);

  // dispextern.h 3905/3936/3945: the same terminal entry points as in do_scrolling.
  unsafe extern "C-unwind" {
    /// C: void mirrored_line_dance (struct frame *f, int, int, int *, char *)
    pub fn mirrored_line_dance (f: *mut frame, unchanged_at_top: c_int, window_size: c_int,
				copy_from: *mut c_int, retained_p: *mut c_char);
    /// C: extern void set_terminal_window (struct frame *, int)
    pub fn set_terminal_window (frame: *mut frame, size: c_int);
    /// C: extern void ins_del_lines (struct frame *, int, int)
    pub fn ins_del_lines (frame: *mut frame, vpos: c_int, n: c_int);
  }

  /* We are working on the line range UNCHANGED_AT_TOP ...
     UNCHANGED_AT_TOP + WINDOW_SIZE (not including) in CURRENT_MATRIX.
     We step through lines in this range from the end to the start.  I
     is an index into new lines, j an index into old lines.  The cost
     matrix determines what to do for ranges of indices.

     If i is decremented without also decrementing j, this corresponds
     to inserting empty lines in the result.  If j is decremented
     without also decrementing i, this corresponds to omitting these
     lines in the new rows, i.e. rows are deleted.  */
  i = window_size;
  j = window_size;

  while i > 0 || j > 0
    {
      p = cost_matrix.offset ((i * (window_size + 1) + j) as isize);

      if (*p).insertcost < (*p).writecost
	  && (*p).insertcost < (*p).deletecost
	  && (write_follows_p || i < j)
	{
	  /* Insert is cheaper than deleting or writing lines.  Leave
	     a hole in the result display that will be filled with
	     empty lines when the queue is emptied.  */
	  (*queue).count = 0;
	  (*queue).window = i;
	  (*queue).pos = i - (*p).insertcount;
	  queue = queue.offset (1);

	  i -= (*p).insertcount;
	  write_follows_p = false;
	}
      else if (*p).deletecost < (*p).writecost
	       && (write_follows_p || i > j)
	{
	  /* Deleting lines is cheaper.  By decrementing J, omit
	     deletecount lines from the original.  */
	  write_follows_p = false;
	  j -= (*p).deletecount;
	}
      else
	{
	  /* One or more lines should be written.  In the direct
	     scrolling method we do this by scrolling the lines to the
	     place they belong.  */
	  let mut n_to_write: c_int = (*p).writecount;
	  write_follows_p = true;
	  eassert! (n_to_write > 0);

	  if i > j
	    {
	      /* Immediately insert lines */
	      set_terminal_window (frame, i + unchanged_at_top);
	      terminal_window_p = true;
	      ins_del_lines (frame, j - n_to_write + unchanged_at_top, i - j);
	    }
	  else if i < j
	    {
	      /* Queue the deletion of a group of lines */
	      (*queue).pos = i - n_to_write + unchanged_at_top;
	      (*queue).window = j + unchanged_at_top;
	      (*queue).count = i - j;
	      queue = queue.offset (1);
	    }

	  while n_to_write > 0
	    {
	      i -= 1;
	      j -= 1;
	      n_to_write -= 1;
	      *copy_from.offset (i as isize) = j;
	      *retained_p.offset (j as isize) = 1;
	    }
	}
    }

  /* Do queued operations.  */
  if queue > queue_start
    {
      let mut next: c_int = -1;

      loop
	{
	  queue = queue.offset (-1);
	  if (*queue).count != 0
	    {
	      set_terminal_window (frame, (*queue).window);
	      terminal_window_p = true;
	      ins_del_lines (frame, (*queue).pos, (*queue).count);
	    }
	  else
	    {
	      j = (*queue).window - 1;
	      while j >= (*queue).pos
		{
		  while { next += 1; *retained_p.offset (next as isize) != 0 }
		    { }
		  *copy_from.offset (j as isize) = next;
		  j -= 1;
		}
	    }

	  if !(queue > queue_start)
	    { break; }
	}
    }

  /* Now, for each row I in the range of rows we are working on,
     copy_from[i] gives the original line to copy to I, and
     retained_p[copy_from[i]] is zero if line I in the new display is
     empty.  */
  mirrored_line_dance (frame, unchanged_at_top, window_size,
		       copy_from, retained_p);

  if terminal_window_p
    { set_terminal_window (frame, 0); }
  SAFE_FREE! (sa_count, sa_mark);

  }
}


#[unsafe(no_mangle)]
#[cfg (not (feature = "have_android"))]
pub unsafe extern "C-unwind" fn scrolling_1 (frame: *mut frame, window_size: c_int,
				      unchanged_at_top: c_int, unchanged_at_bottom: c_int,
				      draw_cost: *mut c_int, old_draw_cost: *mut c_int,
				      old_hash: *mut c_uint, new_hash: *mut c_uint,
				      free_at_end: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  let mut matrix: *mut matrix_elt = std::ptr::null_mut ();
  SAFE_NALLOCA! (sa_avail, matrix, window_size + 1, window_size + 1);

  if FRAME_SCROLL_REGION_OK! (frame)
    {
      calculate_direct_scrolling (frame, matrix, window_size,
				  unchanged_at_bottom,
				  draw_cost, old_draw_cost,
				  old_hash, new_hash, free_at_end);
      // frame.h 325: `current_matrix` is a `struct glyph_matrix *` in C; the frame mirror
      // keeps it as `*mut c_void` (P-72 rule 3), so the use site casts once.
      do_direct_scrolling (frame, (*frame).current_matrix as *mut glyph_matrix,
			   matrix, window_size, unchanged_at_top);
    }
  else
    {
      calculate_scrolling (frame, matrix, window_size, unchanged_at_bottom,
			   draw_cost, old_hash, new_hash,
			   free_at_end);
      do_scrolling (frame,
		    (*frame).current_matrix as *mut glyph_matrix, matrix, window_size,
		    unchanged_at_top);
    }

  SAFE_FREE! (sa_count, sa_mark);

  }
}


/* Return number of lines in common between current and desired frame
   contents described to us only as vectors of hash codes OLDHASH and
   NEWHASH.  Consider only vpos range START to END (not including
   END).  Ignore short lines on the assumption that avoiding redrawing
   such a line will have little weight.  */

#[unsafe(no_mangle)]
#[cfg (not (feature = "have_android"))]
pub unsafe extern "C-unwind" fn scrolling_max_lines_saved (start: c_int, end: c_int,
						    oldhash: *mut c_uint, newhash: *mut c_uint,
						    cost: *mut c_int) -> c_int
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  const LOG2_NLINES: usize = 9;
  const NLINES: usize = 1 << LOG2_NLINES;
  /* C 840-841 declares the array's element type anonymously (`struct { unsigned hash; int
     count; } lines[NLINES];`); the port names it after the variable, the `<variable>_elt`
     shape P-67 gives an anonymous member.  The array is C's stack object, zeroed by the
     `memset` below.  */
  #[repr(C)]
  #[derive(Clone, Copy)]
  struct lines_elt { hash: c_uint, count: c_int }
  let mut lines: [lines_elt; NLINES] = [lines_elt { hash: 0, count: 0 }; NLINES];
  let mut i: c_int;
  let mut h: c_int;
  let mut matchcount: c_int = 0;
  let mut avg_length: c_int = 0;
  let threshold: c_int;

  /* Compute a threshold which is 1/4 of average length of these lines.  */

  i = start;
  while i < end
    {
      avg_length += *cost.offset (i as isize);
      i += 1;
    }

  avg_length /= end - start;
  threshold = avg_length / 4;

  memset (lines.as_mut_ptr () as *mut c_void, 0, std::mem::size_of_val (&lines));

  /* Put new lines' hash codes in hash table.  Ignore lines shorter
     than the threshold.  Thus, if the lines that are in common are
     mainly the ones that are short, they won't count.  */
  i = start;
  while i < end
    {
      if *cost.offset (i as isize) > threshold
	{
	  h = (*newhash.offset (i as isize) & (NLINES as c_uint - 1)) as c_int;
	  lines[h as usize].hash = *newhash.offset (i as isize);
	  lines[h as usize].count += 1;
	}
      i += 1;
    }

  /* Look up old line hash codes in the hash table.  Count number of
     matches between old lines and new.  */
  i = start;
  while i < end
    {
      h = (*oldhash.offset (i as isize) & (NLINES as c_uint - 1)) as c_int;
      if *oldhash.offset (i as isize) == lines[h as usize].hash
	{
	  matchcount += 1;
	  lines[h as usize].count -= 1;
	  if lines[h as usize].count == 0
	    { lines[h as usize].hash = 0; }
	}
      i += 1;
    }

  return matchcount;

  }
}

/* Calculate the line insertion/deletion
   overhead and multiply factor values */

#[cfg (not (feature = "have_android"))]
unsafe fn line_ins_del (frame: *mut frame, ov1: c_int, pf1: c_int, ovn: c_int, pfn: c_int,
			ov: *mut c_int, mf: *mut c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let mut i: c_int;
  let frame_total_lines: c_int = FRAME_TOTAL_LINES (frame);
  let mut insert_overhead: c_int = ov1 * 10;
  let mut next_insert_cost: c_int = ovn * 10;

  i = frame_total_lines - 1;
  while i >= 0
    {
      *mf.offset (i as isize) = next_insert_cost / 10;
      next_insert_cost += pfn;
      *ov.offset (i as isize) = (insert_overhead + next_insert_cost) / 10;
      insert_overhead += pf1;
      i -= 1;
    }

  }
}

#[cfg (not (feature = "have_android"))]
unsafe fn ins_del_costs (frame: *mut frame,
			 one_line_string: *const c_char, multi_string: *const c_char,
			 setup_string: *const c_char, cleanup_string: *const c_char,
			 costvec: *mut c_int, ncostvec: *mut c_int,
			 coefficient: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  // dispextern.h 3953-3954: `extern int string_cost (const char *);` and
  // `extern int per_line_cost (const char *);` (defined in term.c).
  unsafe extern "C-unwind" {
    /// C: int string_cost (const char *)
    pub fn string_cost (s: *const c_char) -> c_int;
    /// C: int per_line_cost (const char *)
    pub fn per_line_cost (s: *const c_char) -> c_int;
  }
  if !multi_string.is_null ()
    { line_ins_del (frame,
		    string_cost (multi_string) * coefficient,
		    per_line_cost (multi_string) * coefficient,
		    0, 0, costvec, ncostvec); }
  else if !one_line_string.is_null ()
    { line_ins_del (frame,
		    string_cost (setup_string) + string_cost (cleanup_string), 0,
		    string_cost (one_line_string),
		    per_line_cost (one_line_string),
		    costvec, ncostvec); }
  else
    { line_ins_del (frame,
		    9999, 0, 9999, 0,
		    costvec, ncostvec); }

  }
}

/* Calculate the insert and delete line costs.
   Note that this is done even when running with a window system
   because we want to know how long scrolling takes (and avoid it).
   This must be redone whenever the frame height changes.

   We keep the ID costs in a precomputed array based on the position
   at which the I or D is performed.  Also, there are two kinds of ID
   costs: the "once-only" and the "repeated".  This is to handle both
   those terminals that are able to insert N lines at a time (once-
   only) and those that must repeatedly insert one line.

   The cost to insert N lines at line L is
	    [tt.t_ILov  + (frame_total_lines + 1 - L) * tt.t_ILpf] +
	N * [tt.t_ILnov + (frame_total_lines + 1 - L) * tt.t_ILnpf]

   ILov represents the basic insert line overhead.  ILpf is the padding
   required to allow the terminal time to move a line: insertion at line
   L changes (frame_total_lines + 1 - L) lines.

   The first bracketed expression above is the overhead; the second is
   the multiply factor.  Both are dependent only on the position at
   which the insert is performed.  We store the overhead in
   FRAME_INSERT_COST (frame) and the multiply factor in
   FRAME_INSERTN_COST (frame).  Note however that any insertion
   must include at least one multiply factor.  Rather than compute this
   as FRAME_INSERT_COST (frame)[line]+FRAME_INSERTN_COST (frame)[line],
   we add FRAME_INSERTN_COST (frame) into FRAME_INSERT_COST (frame).
   This is reasonable because of the particular algorithm used in calcM.

   Deletion is essentially the same as insertion.
 */

#[unsafe(no_mangle)]
#[cfg (not (feature = "have_android"))]
pub unsafe extern "C-unwind" fn do_line_insertion_deletion_costs (frame: *mut frame,
							   ins_line_string: *const c_char,
							   multi_ins_string: *const c_char,
							   del_line_string: *const c_char,
							   multi_del_string: *const c_char,
							   setup_string: *const c_char,
							   cleanup_string: *const c_char,
							   coefficient: c_int)
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let frame_total_lines: c_int = FRAME_TOTAL_LINES (frame);
  /* C writes through the object-like macros (`FRAME_INSERT_COST (frame) = ...`), which
     frame.rs mirrors as functions returning the field; the assignment target is therefore
     the same place, spelled `(*frame).<field>` (the macro still names the read in the
     right-hand side, so every C token survives, PORTING.md P-37).  */
  (*frame).insert_line_cost
    = xnrealloc (FRAME_INSERT_COST (frame) as *mut c_void, frame_total_lines as ptrdiff_t,
		 std::mem::size_of::<c_int> () as ptrdiff_t) as *mut c_int;
  (*frame).delete_n_lines_cost
    = xnrealloc (FRAME_DELETEN_COST (frame) as *mut c_void, frame_total_lines as ptrdiff_t,
		 std::mem::size_of::<c_int> () as ptrdiff_t) as *mut c_int;
  (*frame).insert_n_lines_cost
    = xnrealloc (FRAME_INSERTN_COST (frame) as *mut c_void, frame_total_lines as ptrdiff_t,
		 std::mem::size_of::<c_int> () as ptrdiff_t) as *mut c_int;
  (*frame).delete_line_cost
    = xnrealloc (FRAME_DELETE_COST (frame) as *mut c_void, frame_total_lines as ptrdiff_t,
		 std::mem::size_of::<c_int> () as ptrdiff_t) as *mut c_int;

  ins_del_costs (frame,
		 ins_line_string, multi_ins_string,
		 setup_string, cleanup_string,
		 FRAME_INSERT_COST (frame), FRAME_INSERTN_COST (frame),
		 coefficient);
  ins_del_costs (frame,
		 del_line_string, multi_del_string,
		 setup_string, cleanup_string,
		 FRAME_DELETE_COST (frame), FRAME_DELETEN_COST (frame),
		 coefficient);

  }
}

// C 993: `#endif` of the file's `#ifndef HAVE_ANDROID` (see the header note).
