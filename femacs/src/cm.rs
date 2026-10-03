// femacs: port of src/cm.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct member, macro, local), same control flow, same error model.  The nine
// non-`static` definitions (`cost`, `evalcost`, `current_tty`, `cmputc`, `cmcheckmagic`,
// `cmcostinit`, `cmgoto`, `Wcm_clear`, `Wcm_init`) are C's exported symbols, so they carry
// `#[unsafe(no_mangle)]` (PORTING.md 0.2/P-52); `calccost` is C's only `static` helper and
// stays a plain `unsafe fn`.
//
// The `#include` list mirrors cm.c 21-28 - config.h, lisp.h, cm.h, frame.h, sysstdio.h,
// termchar.h, tparam.h - through the mirrors those pull in.  lisp.h brings <string.h>
// (`memset`) and <limits.h>; sysstdio.h is <stdio.h> (`putc`, `fputs`, `stdout`, `FILE`);
// termchar.h's mirror carries dispextern.h and termhooks.h itself.  cm.h's mirror is named
// here (C names it) and again inside termchar.rs, where terms.h's own `struct cm *Wcm`
// needs the type: `xtask` expands the include graph and dedupes by file (PORTING.md 0.1-4).
//
// `#if 0` at C 54-100 (`at`/`addcol`) and C 315-321 (`losecursor`) is dead text: C's own
// preprocessor never compiles it, so the port keeps it as a comment rather than behind a
// feature (the character.rs `#if 0` precedent; P-51's gate is for arms the frozen
// configuration chooses against, and no configuration defines these).
//
// `int use UNINIT;` (C 338) has no counterpart: `UNINIT` (conf_post.h) is documentation for
// a value that is assigned before use, so the port writes the explicit zero initializer the
// other units use (the `Fmake_variable_buffer_local` ledger row).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_assignments,
         unused_unsafe, static_mut_refs, unsafe_op_in_unsafe_fn)]

/* Cursor motion subroutines for GNU Emacs.
   Copyright (C) 1985, 1995, 2001-2026 Free Software Foundation, Inc.
    based primarily on public domain code written by Chris Torek

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

include!("include/config.rs");
include!("../lib/include/stdio.rs");     // cm.c 26 sysstdio.h: FILE, putc, fputs, stdout
include!("../lib/include/string.rs");    // lisp.h 30: memset
include!("../lib/include/stdcountof.rs"); // lisp.h 28 (the mirror chain's own)
include!("include/lisp.rs");             // cm.c 23: lisp.h (globals.h, `emacs_abort`)
// dispextern.h 25 includes character.h; frame.h 22-23 includes termhooks.h and window.h,
// and termhooks.h 28 includes systime.h for `Time`.  The mirrors name a few types the C
// headers reach indirectly (coding.h's `coding_system` through termhooks.h's pointer
// members), so the list below is the set scroll.rs/doc.rs proved (PORTING.md P-61).
include!("include/character.rs");
include!("include/itree.rs");
include!("include/buffer.rs");
include!("include/systime.rs");
include!("include/coding.rs");
include!("include/dispextern.rs");
include!("include/composite.rs");
include!("include/termhooks.rs");
include!("include/window.rs");
include!("include/frame.rs");            // cm.c 25: frame.h (XFRAME)
include!("include/cm.rs");               // cm.c 24: cm.h
include!("include/termchar.rs");         // cm.c 27: termchar.h (tty_display_info)
include!("include/tparam.rs");           // cm.c 28: tparam.h (tputs, tgoto, UP, BC)

/* C's `putc` (<stdio.h>) and `stdout`: the port declares the two system entries it calls
   here, which is where the C translation unit gets them (the getopt.rs shape, PORTING.md
   0.1-3).  `fputs`/`FILE` come from the stdio mirror above.  */
unsafe extern "C" {
  /* int putc (int c, FILE *stream);  */
  pub fn putc (c: c_int, stream: *mut FILE) -> c_int;
  /* extern FILE *stdout;  */
  #[cfg_attr (target_os = "macos", link_name = "__stdoutp")]
  pub static mut stdout: *mut FILE;
}

/* cm.c 30: `#define BIG 9999` - "Good on 32-bit hosts."  */
const BIG: c_int = 9999;

/* cm.c 32: sums up costs */
#[unsafe(no_mangle)]
pub static mut cost: c_int = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn evalcost (c: c_int) -> c_int
{
  cost += 1;
  return c;
}

/* The terminal to use for low-level output. */
#[unsafe(no_mangle)]
pub static mut current_tty: *mut tty_display_info = std::ptr::null_mut ();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmputc (c: c_int) -> c_int
{
  if ! (*current_tty).termscript.is_null ()
    { putc (c & 0o177, (*current_tty).termscript); }
  putc (c & 0o177, (*current_tty).output);
  return c;
}

/* C 54-100 is `#if 0`: the two macros' stand-alone versions, `at` and `addcol`.
   C's own preprocessor never compiles them, so the port keeps C's text as a
   comment (the character.rs `#if 0` precedent).

#if 0
/*
 * Assume the cursor is at row row, column col.  Normally used only after
 * clearing the screen, when the cursor is at (0, 0), but what the heck,
 * let's let the guy put it anywhere.
 */

static
at (tty, row, col) {
  curY (tty) = row;
  curX (tty)  = col;
}

/*
 * Add n columns to the current cursor position.
 */

static
addcol (tty, n) {
  curX (tty) += n;

    /*
     * If cursor hit edge of screen, what happened?
     * N.B.: DO NOT!! write past edge of screen.  If you do, you
     * deserve what you get.  Furthermore, on terminals with
     * autowrap (but not magicwrap), don't write in the last column
     * of the last line.
     */

  if (curX (tty) == tty->Wcm->cm_cols) {
	/*
	 * Well, if magicwrap, still there, past the edge of the
	 * screen (!).  If autowrap, on the col 0 of the next line.
	 * Otherwise on last column.
	 */

	if (tty->Wcm->cm_magicwrap)
	    ;			/* "limbo" */
	else if (tty->Wcm->cm_autowrap) {
          curX (tty) = 0;
          curY (tty) ++;		/* Beware end of screen! */
	}
	else
          curX (tty)--;
    }
}
#endif
*/

/*
 * Terminals with magicwrap (xn) don't all behave identically.
 * The VT100 leaves the cursor in the last column but will wrap before
 * printing the next character.  I hear that the Concept terminal does
 * the wrap immediately but ignores the next newline it sees.  And some
 * terminals just have buggy firmware, and think that the cursor is still
 * in limbo if we use direct cursor addressing from the phantom column.
 * The only guaranteed safe thing to do is to emit a CRLF immediately
 * after we reach the last column; this takes us to a known state.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmcheckmagic (tty: *mut tty_display_info)
{
  /* dispextern.h 3925: `extern bool frame_size_change_delayed (struct frame *);`.  The
     dispextern mirror does not carry the dispnew.c/terminal.c block of prototypes yet, so
     the declaration is made at its use site (the scroll.rs shape) - nothing in this file
     needs it anywhere else.  */
  unsafe extern "C" { fn frame_size_change_delayed (f: *mut frame) -> bool; }

  /* If we have an unhandled SIGWINCH, we don't really know what our
     up-to-date frame dimensions are.  */
  if frame_size_change_delayed (XFRAME ((*tty).top_frame))
    { return; }
  if curX! (tty) == FrameCols! (tty)
    {
      if ! MagicWrap! (tty) || curY! (tty) >= FrameRows! (tty) - 1
	{ emacs_abort (); }
      if ! (*tty).termscript.is_null ()
	{ putc ('\r' as c_int, (*tty).termscript); }
      putc ('\r' as c_int, (*tty).output);
      if ! (*tty).termscript.is_null ()
	{ putc ('\n' as c_int, (*tty).termscript); }
      putc ('\n' as c_int, (*tty).output);
      curX! (tty) = 0;
      curY! (tty) += 1;
    }
}


/*
 * (Re)Initialize the cost factors, given the output speed of the terminal
 * in the variable ospeed.  (Note: this holds B300, B9600, etc -- ie stuff
 * out of <sgtty.h>.)
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmcostinit (tty: *mut tty_display_info)
{
  let mut p: *mut c_char = std::ptr::null_mut ();

  /* C 146-147's two file-local macros.  `macro_rules!` has no `#undef`, so they are
     defined inside this body and cannot leak to the rest of the file (C 171-172's two
     `#undef`s have no counterpart to write).  */
  macro_rules! COST {
    ($x:expr, $e:expr) => {
      if ! ($x).is_null () { cost = 0; tputs ($x, 1, Some ($e)); cost } else { BIG }
    };
  }
  macro_rules! CMCOST {
    ($x:expr, $e:expr) => {
      if ($x).is_null () { BIG } else { p = tgoto ($x, 0, 0); COST! (p, $e) }
    };
  }

  (*(*tty).Wcm).cc_up =	 COST! ((*(*tty).Wcm).cm_up, evalcost);
  (*(*tty).Wcm).cc_down =	 COST! ((*(*tty).Wcm).cm_down, evalcost);
  (*(*tty).Wcm).cc_left =	 COST! ((*(*tty).Wcm).cm_left, evalcost);
  (*(*tty).Wcm).cc_right = COST! ((*(*tty).Wcm).cm_right, evalcost);
  (*(*tty).Wcm).cc_home =	 COST! ((*(*tty).Wcm).cm_home, evalcost);
  (*(*tty).Wcm).cc_cr =	 COST! ((*(*tty).Wcm).cm_cr, evalcost);
  (*(*tty).Wcm).cc_ll =	 COST! ((*(*tty).Wcm).cm_ll, evalcost);
  (*(*tty).Wcm).cc_tab =	 if (*(*tty).Wcm).cm_tabwidth != 0
			   { COST! ((*(*tty).Wcm).cm_tab, evalcost) } else { BIG };

  /*
   * These last three are actually minimum costs.  When (if) they are
   * candidates for the least-cost motion, the real cost is computed.
   * (Note that "0" is the assumed to generate the minimum cost.
   * While this is not necessarily true, I have yet to see a terminal
   * for which is not; all the terminals that have variable-cost
   * cursor motion seem to take straight numeric values.  --ACT)
   */

  (*(*tty).Wcm).cc_abs =  CMCOST! ((*(*tty).Wcm).cm_abs, evalcost);
  (*(*tty).Wcm).cc_habs = CMCOST! ((*(*tty).Wcm).cm_habs, evalcost);
  (*(*tty).Wcm).cc_vabs = CMCOST! ((*(*tty).Wcm).cm_vabs, evalcost);
}

/*
 * Calculate the cost to move from (srcy, srcx) to (dsty, dstx) using
 * up and down, and left and right, motions, and tabs.  If doit is set
 * actually perform the motion.
 */

unsafe fn calccost (tty: *mut tty_display_info,
		    srcy: c_int, mut srcx: c_int, dsty: c_int, dstx: c_int, doit: c_int)
  -> c_int
{
  let mut deltay: c_int = 0;
  let mut deltax: c_int = 0;
  let mut c: c_int = 0;
  let mut totalcost: c_int = 0;
  let mut ntabs: c_int = 0;
  let mut n2tabs: c_int = 0;
  let mut tabx: c_int = 0;
  let mut tab2x: c_int = 0;
  let mut tabcost: c_int = 0;
  let mut p: *const c_char = std::ptr::null ();

  /* C's five forward jumps are `goto fail` (C 201, which lands *inside* the `c == BIG`
     arm at C 300-305), `goto x` (C 205), `goto dodelta` (C 225), `goto done` (C 222/291)
     and `goto newdelta`/`goto olddelta` (C 272/230).  Each label's statements appear once
     below, in C's order, and the jumps are the flags named after them - PORTING.md 2.2
     forbids turning a `goto` into an early `return` that skips the statements after it.  */
  let mut fail: bool = false;		/* C: `goto fail` */
  let mut done: bool = false;		/* C: `goto done` */
  let mut dodelta: bool = false;	/* C: `goto dodelta` */
  let mut newdelta: bool = true;	/* C: fall into `newdelta:` */

  /* If we have just wrapped on a terminal with xn,
     don't believe the cursor position: give up here
     and force use of absolute positioning.  */

  if curX! (tty) == (*(*tty).Wcm).cm_cols
    { fail = true; }
  else
    {
      totalcost = 0;
      deltay = dsty - srcy;
      if deltay == 0
	{ newdelta = true; }		/* C: `goto x` - the vertical block is skipped.  */
      else
	{
	  if deltay < 0
	    { p = (*(*tty).Wcm).cm_up; c = (*(*tty).Wcm).cc_up; deltay = -deltay; }
	  else
	    { p = (*(*tty).Wcm).cm_down; c = (*(*tty).Wcm).cc_down; }
	  if c == BIG
	    {		/* caint get thar from here */
	      if doit != 0
		{ fputs (c"OOPS".as_ptr (), stdout); }
	      return c;
	    }
	  totalcost = c * deltay;
	  if doit != 0
	    {
	      loop
		{
		  emacs_tputs! (tty, p, 1, cmputc);
		  deltay -= 1;
		  if ! (deltay > 0) { break; }
		}
	    }
	}

      /* x: */
      deltax = dstx - srcx;
      if deltax == 0
	{ done = true; }
      else if deltax < 0
	{
	  p = (*(*tty).Wcm).cm_left; c = (*(*tty).Wcm).cc_left; deltax = -deltax;
	  dodelta = true;		/* skip all the tab junk */
	}
      else
	{
	  /* Tabs (the toughie) */
	  if ! tty_cursor_movement_use_TAB
	    || (*(*tty).Wcm).cc_tab >= BIG || ((*(*tty).Wcm).bits & CM_USETABS) == 0
	    { newdelta = false; }	/* C: `goto olddelta` */
	  else
	    {
	      /*
	       * ntabs is # tabs towards but not past dstx.  tabx is where we
	       * would be if we put those ntabs tabulations.
	       */

	      ntabs = (deltax + srcx % (*(*tty).Wcm).cm_tabwidth)
		      / (*(*tty).Wcm).cm_tabwidth;
	      tabx = (srcx / (*(*tty).Wcm).cm_tabwidth + ntabs) * (*(*tty).Wcm).cm_tabwidth;

	      if tty_cursor_movement_use_TAB_BS
		{
		  /*
		   * n2tabs is one more tab than ntabs (i.e. past dstx), but this is
		   * only valid if that is not past the right edge of the screen.
		   * tab2x is where we would be if we put those n2tabs tabulations.
		   */
		  n2tabs = ntabs + 1;
		  tab2x = tabx + (*(*tty).Wcm).cm_tabwidth;
		  if tab2x >= (*(*tty).Wcm).cm_cols	/* too far (past edge) */
		    { n2tabs = 0; }

		  /*
		   * Set c to the cost for using n2tabs
		   */
		  /* cost for n2tabs          +    cost for left motion */
		  c = if n2tabs != 0
			{ n2tabs * (*(*tty).Wcm).cc_tab + (tab2x - dstx) * (*(*tty).Wcm).cc_left }
		      else
			{ BIG };
		}

	      /*
	       * Set tabcost to the cost for using ntabs.
	       */

	      /* cost for ntabs           +    cost for right motion */
	      tabcost = if ntabs != 0
			  { ntabs * (*(*tty).Wcm).cc_tab + (dstx - tabx) * (*(*tty).Wcm).cc_right }
			else
			  { BIG };

	      /* Is it allowed and cheaper to overshoot & back up? */
	      if tty_cursor_movement_use_TAB_BS && (c < tabcost)
		{ ntabs = n2tabs; tabcost = c; tabx = tab2x; }

	      if tabcost >= BIG		/* caint use tabs */
		{ /* C: `goto newdelta` - the tab-using block below is skipped.  */ }
	      else
		{
		  /*
		   * See if tabcost is less than just moving right
		   */

		  if tabcost < (deltax * (*(*tty).Wcm).cc_right)
		    {
		      totalcost += tabcost;	/* use the tabs */
		      if doit != 0
			{
			  while { ntabs -= 1; ntabs >= 0 }
			    { emacs_tputs! (tty, (*(*tty).Wcm).cm_tab, 1, cmputc); }
			}
		      srcx = tabx;
		    }
		}

	      /*
	       * Now might as well just recompute the delta.
	       */

	      /* newdelta: */
	      deltax = dstx - srcx;
	      if deltax == 0
		{ done = true; }
	    }

	  if ! done && newdelta
	    {
	      /* olddelta: */
	      if deltax > 0
		{ p = (*(*tty).Wcm).cm_right; c = (*(*tty).Wcm).cc_right; }
	      else
		{ p = (*(*tty).Wcm).cm_left; c = (*(*tty).Wcm).cc_left; deltax = -deltax; }

	      dodelta = true;
	    }
	}

      if dodelta
	{
	  /* dodelta: */
	  if c == BIG
	    {		/* caint get thar from here */
	      /* fail: */
	      if doit != 0
		{ fputs (c"OOPS".as_ptr (), stdout); }
	      return BIG;
	    }
	  totalcost += c * deltax;
	  if doit != 0
	    {
	      loop
		{
		  emacs_tputs! (tty, p, 1, cmputc);
		  deltax -= 1;
		  if ! (deltax > 0) { break; }
		}
	    }
	}
    }

  if fail
    {
      /* fail: — C's `goto fail` (C 201) enters the arm above at its label, so the OOPS
	 print and `return BIG` are the same two statements, reached without the `c == BIG`
	 test.  */
      if doit != 0
	{ fputs (c"OOPS".as_ptr (), stdout); }
      return BIG;
    }

  /* done: */
  return totalcost;
}

/* C 315-321 is `#if 0`: a stand-alone `losecursor` (the live spelling is the cm.h
   macro).

#if 0
void
losecursor (void)
{
  curY = -1;
}
#endif
*/

const USEREL: c_int = 0;
const USEHOME: c_int = 1;
const USELL: c_int = 2;
const USECR: c_int = 3;

/* Move the cursor to (ROW, COL), by computing the optimal way.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmgoto (tty: *mut tty_display_info, row: c_int, col: c_int)
{
  let mut homecost: c_int = 0;
  let mut crcost: c_int = 0;
  let mut llcost: c_int = 0;
  let mut relcost: c_int = 0;
  let mut directcost: c_int = 0;
  /* C 338: `int use UNINIT;` - assigned before every read; the port starts it at zero
     (`UNINIT` is a comment in C, PORTING.md's ledger row).  `r#use` keeps C's name.  */
  let mut r#use: c_int = 0;
  let mut p: *mut c_char = std::ptr::null_mut ();
  let mut dcm: *const c_char = std::ptr::null ();

  /* First the degenerate case */
  if row == curY! (tty) && col == curX! (tty)	/* already there */
    { return; }

  if curY! (tty) >= 0 && curX! (tty) >= 0
    {
      /* We may have quick ways to go to the upper-left, bottom-left,
       * start-of-line, or start-of-next-line.  Or it might be best to
       * start where we are.  Examine the options, and pick the cheapest.
       */

      relcost = calccost (tty, curY! (tty), curX! (tty), row, col, 0);
      r#use = USEREL;
      homecost = (*(*tty).Wcm).cc_home;
      if homecost < BIG
	{ homecost += calccost (tty, 0, 0, row, col, 0); }
      if homecost < relcost
	{ relcost = homecost; r#use = USEHOME; }
      llcost = (*(*tty).Wcm).cc_ll;
      if llcost < BIG
	{ llcost += calccost (tty, (*(*tty).Wcm).cm_rows - 1, 0, row, col, 0); }
      if llcost < relcost
	{ relcost = llcost; r#use = USELL; }
      crcost = (*(*tty).Wcm).cc_cr;
      if crcost < BIG
	{
	  if ((*(*tty).Wcm).bits & CM_AUTOLF) != 0
	    {
	      if curY! (tty) + 1 >= (*(*tty).Wcm).cm_rows
		{ crcost = BIG; }
	      else
		{ crcost += calccost (tty, curY! (tty) + 1, 0, row, col, 0); }
	    }
	  else
	    { crcost += calccost (tty, curY! (tty), 0, row, col, 0); }
	}
      if crcost < relcost
	{ relcost = crcost; r#use = USECR; }
      directcost = (*(*tty).Wcm).cc_abs; dcm = (*(*tty).Wcm).cm_abs;
      if row == curY! (tty) && (*(*tty).Wcm).cc_habs < BIG
	{ directcost = (*(*tty).Wcm).cc_habs; dcm = (*(*tty).Wcm).cm_habs; }
      else if col == curX! (tty) && (*(*tty).Wcm).cc_vabs < BIG
	{ directcost = (*(*tty).Wcm).cc_vabs; dcm = (*(*tty).Wcm).cm_vabs; }
    }
  else
    {
      directcost = 0; relcost = 100000;
      dcm = (*(*tty).Wcm).cm_abs;
    }

  /*
   * In the following comparison, the = in <= is because when the costs
   * are the same, it looks nicer (I think) to move directly there.
   */
  if directcost <= relcost
    {
      /* compute REAL direct cost */
      cost = 0;
      p = if dcm == (*(*tty).Wcm).cm_habs
	    { tgoto (dcm, row, col) }
	  else
	    { tgoto (dcm, col, row) };
      emacs_tputs! (tty, p, 1, evalcost);
      if cost <= relcost
	{	/* really is cheaper */
	  emacs_tputs! (tty, p, 1, cmputc);
	  curY! (tty) = row; curX! (tty) = col;
	  return;
	}
    }

  match r#use
    {
    USEHOME =>						/* case USEHOME: */
      {
	emacs_tputs! (tty, (*(*tty).Wcm).cm_home, 1, cmputc);
	curY! (tty) = 0; curX! (tty) = 0;
      }

    USELL =>						/* case USELL: */
      {
	emacs_tputs! (tty, (*(*tty).Wcm).cm_ll, 1, cmputc);
	curY! (tty) = (*(*tty).Wcm).cm_rows - 1; curX! (tty) = 0;
      }

    USECR =>						/* case USECR: */
      {
	emacs_tputs! (tty, (*(*tty).Wcm).cm_cr, 1, cmputc);
	if ((*(*tty).Wcm).bits & CM_AUTOLF) != 0
	  { curY! (tty) += 1; }
	curX! (tty) = 0;
      }

    /* C's switch has no `default:` and no `case USEREL:`: a `use` of USEREL falls out of
       the switch with nothing done, which is this arm (the sigdescr_np precedent).  */
    _ => { }
    }

  /* C 426: `(void) calccost (...)` - the cost is computed for its side effects on the
     terminal (doit is 1).  */
  let _ = calccost (tty, curY! (tty), curX! (tty), row, col, 1);
  curY! (tty) = row; curX! (tty) = col;
}

/* Clear out all terminal info.
   Used before copying into it the info on the actual terminal.
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Wcm_clear (tty: *mut tty_display_info)
{
  memset ((*tty).Wcm.cast (), 0, size_of::<cm> ());
  UP = std::ptr::null_mut ();
  BC = std::ptr::null_mut ();
}

/*
 * Initialized stuff
 * Return 0 if can do CM.
 * Return -1 if cannot.
 * Return -2 if size not specified.
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Wcm_init (tty: *mut tty_display_info) -> c_int
{
  /* C 452-455 is `#if 0`: `if (tty->Wcm->cm_abs && !tty->Wcm->cm_ds) return 0;`.  */
  if ! (*(*tty).Wcm).cm_abs.is_null ()
    { return 0; }
  /* Require up and left, and, if no absolute, down and right */
  if (*(*tty).Wcm).cm_up.is_null () || (*(*tty).Wcm).cm_left.is_null ()
    { return - 1; }
  if (*(*tty).Wcm).cm_abs.is_null ()
    && ((*(*tty).Wcm).cm_down.is_null () || (*(*tty).Wcm).cm_right.is_null ())
    { return - 1; }
  /* Check that we know the size of the screen.... */
  if (*(*tty).Wcm).cm_rows <= 0 || (*(*tty).Wcm).cm_cols <= 0
    { return - 2; }
  return 0;
}
