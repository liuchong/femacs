// femacs: port of src/cm.h @ fd7afced403.
//
// cm.h has no `#include` of its own (C 19-21 are the guard), so this mirror names no other
// mirror: `struct tty_display_info` occurs only in pointer positions and in prototypes, and
// C's cm.c reaches the real definition through termchar.h, which it includes *after* cm.h.
// The port keeps that shape - the type is termchar.rs's - so a unit that includes this
// mirror must also carry termchar.rs, exactly as C's tty units include termchar.h.  P-70's
// placeholder is not needed here precisely because termchar.rs, the file that defines the
// type, is the one that includes this mirror (P-70's own point: the first mirror that names
// a type gives it a home, and the real definition replaces the stand-in).
//
// `#if false` at C 53-55 and C 75-92 is dead in *every* configuration, not an arm the frozen
// config chooses: `cm_ds` and the seven `struct parmcap cc_*` members never exist in any
// C build, so they are not mirrored and P-51's feature gate does not apply (the
// font.rs/intervals.h `#if false` precedent: literal-false text is not configuration
// dependent).  `struct parmcap` itself is live C text (C 23-26) and is mirrored below even
// though only those dead members ever name it.
//
// cm.h 63-70's five `bool_bf : 1` members are one storage byte in C; the port keeps
// `cm.bits` with the `CM_*` masks (PORTING.md P-80, the termchar.rs `bits_*` shape), and
// each member is named at its use site.  The byte's offset is asserted by the layout probe
// (tools/gen-layout-fields.py keys the non-bitfield members around it); see
// docs/ports/include-cm.md.

// cm.h 20: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_CM_H: bool = true;

/* Holds the minimum and maximum costs for the parameterized capabilities.  */
#[repr(C)]
pub struct parmcap
  {
    pub mincost: c_int,
    pub maxcost: c_int,
  }

/* This structure holds everything needed to do cursor motion except the pad
   character (PC) and the output speed of the terminal (ospeed), which
   termcap wants in global variables.  */

#[repr(C)]
pub struct cm
  {
    /* Cursor position.  -1 in *both* variables means the cursor
       position is unknown, in order to force absolute cursor motion. */

    pub cm_curY: c_int,			/* Current row */
    pub cm_curX: c_int,			/* Current column */

    /* Capabilities from termcap */
    pub cm_up: *const c_char,		/* up (up) */
    pub cm_down: *const c_char,		/* down (do) */
    pub cm_left: *const c_char,		/* left (le) */
    pub cm_right: *const c_char,		/* right (nd) */
    pub cm_home: *const c_char,		/* home (ho) */
    pub cm_cr: *const c_char,		/* carriage return (cr) */
    pub cm_ll: *const c_char,		/* last line (ll) */
    pub cm_tab: *const c_char,		/* tab (ta) */
    pub cm_backtab: *const c_char,	/* backtab (bt) */
    pub cm_abs: *mut c_char,		/* absolute (cm) */
    pub cm_habs: *const c_char,		/* horizontal absolute (ch) */
    pub cm_vabs: *const c_char,		/* vertical absolute (cv) */
    /* cm.h 53-55 is `#if false`: `const char *cm_ds;` ("don't send" string (ds)) never
       exists in any configuration.  */
    pub cm_multiup: *const c_char,	/* multiple up (UP) */
    pub cm_multidown: *const c_char,	/* multiple down (DO) */
    pub cm_multileft: *const c_char,	/* multiple left (LE) */
    pub cm_multiright: *const c_char,	/* multiple right (RI) */
    pub cm_cols: c_int,			/* number of cols on screen (co) */
    pub cm_rows: c_int,			/* number of rows on screen (li) */
    pub cm_tabwidth: c_int,		/* tab width (it) */
    // cm.h 63-70: `bool_bf cm_autowrap : 1;` and the four that follow are one byte
    // (bits 0-4), the `bits` storage the masks below name (PORTING.md P-80).
    pub bits: u8,

    /* Parameterized capabilities.  This needs to be a struct since
       the costs are accessed through pointers.  */

    /* cm.h 75-92 is `#if false`: the seven `struct parmcap cc_*` members never exist.  */

    /* Costs for the non-parameterized capabilities */
    pub cc_up: c_int,			/* cost for up */
    pub cc_down: c_int,			/* etc. */
    pub cc_left: c_int,
    pub cc_right: c_int,
    pub cc_home: c_int,
    pub cc_cr: c_int,
    pub cc_ll: c_int,
    pub cc_tab: c_int,
    pub cc_backtab: c_int,
    /* These are temporary, until the code is installed to use the
       struct parmcap fields above.  */
    pub cc_abs: c_int,
    pub cc_habs: c_int,
    pub cc_vabs: c_int,
  }

// cm.h 63-70: one mask per `bool_bf : 1` member, in C's declaration order (the termchar.rs
// `TTY_*` shape).  C reads them as plain members; the port names the member at each use
// site, e.g. `((*(*tty).Wcm).bits & CM_AUTOLF) != 0` for C's `tty->Wcm->cm_autolf`.
pub const CM_AUTOWRAP: u8 = 1 << 0;	/* autowrap flag (am) */
pub const CM_MAGICWRAP: u8 = 1 << 1;	/* VT-100: cursor stays in last col but */
pub const CM_USETABS: u8 = 1 << 2;	/* if set, use tabs */
pub const CM_LOSEWRAP: u8 = 1 << 3;	/* if reach right margin, forget cursor */
pub const CM_AUTOLF: u8 = 1 << 4;	/* \r performs a \r\n (rn) */

/* Shorthand */
// C 103's `#ifndef NoCMShortHand` guard is live (nothing defines NoCMShortHand anywhere in
// the tree, and the frozen macro table has no entry for it), so every macro below is
// C text that compiles.  Each is an l-value where C's is one: `curY (tty) = row` assigns
// through the macro, and the port's expansion is a field place for the same reason.
macro_rules! curY {
  ($tty:expr) => { (*(*$tty).Wcm).cm_curY };
}
macro_rules! curX {
  ($tty:expr) => { (*(*$tty).Wcm).cm_curX };
}
macro_rules! Up {
  ($tty:expr) => { (*(*$tty).Wcm).cm_up };
}
macro_rules! Down {
  ($tty:expr) => { (*(*$tty).Wcm).cm_down };
}
macro_rules! Left {
  ($tty:expr) => { (*(*$tty).Wcm).cm_left };
}
macro_rules! Right {
  ($tty:expr) => { (*(*$tty).Wcm).cm_right };
}
macro_rules! Tab {
  ($tty:expr) => { (*(*$tty).Wcm).cm_tab };
}
macro_rules! BackTab {
  ($tty:expr) => { (*(*$tty).Wcm).cm_backtab };
}
macro_rules! TabWidth {
  ($tty:expr) => { (*(*$tty).Wcm).cm_tabwidth };
}
macro_rules! CR {
  ($tty:expr) => { (*(*$tty).Wcm).cm_cr };
}
macro_rules! Home {
  ($tty:expr) => { (*(*$tty).Wcm).cm_home };
}
macro_rules! LastLine {
  ($tty:expr) => { (*(*$tty).Wcm).cm_ll };
}
macro_rules! AbsPosition {
  ($tty:expr) => { (*(*$tty).Wcm).cm_abs };
}
macro_rules! ColPosition {
  ($tty:expr) => { (*(*$tty).Wcm).cm_habs };
}
macro_rules! RowPosition {
  ($tty:expr) => { (*(*$tty).Wcm).cm_vabs };
}
macro_rules! MultiUp {
  ($tty:expr) => { (*(*$tty).Wcm).cm_multiup };
}
macro_rules! MultiDown {
  ($tty:expr) => { (*(*$tty).Wcm).cm_multidown };
}
macro_rules! MultiLeft {
  ($tty:expr) => { (*(*$tty).Wcm).cm_multileft };
}
macro_rules! MultiRight {
  ($tty:expr) => { (*(*$tty).Wcm).cm_multiright };
}
// cm.h 123-127 default to the `bool_bf` members, which have no Rust field (P-80): the
// expansion reads the storage byte with the member's own mask, which is the spelling
// termchar.h's mirror uses for its `bool_bf` accessors.
macro_rules! AutoWrap {
  ($tty:expr) => { ((*(*$tty).Wcm).bits & CM_AUTOWRAP) != 0 };
}
macro_rules! MagicWrap {
  ($tty:expr) => { ((*(*$tty).Wcm).bits & CM_MAGICWRAP) != 0 };
}
macro_rules! UseTabs {
  ($tty:expr) => { ((*(*$tty).Wcm).bits & CM_USETABS) != 0 };
}
macro_rules! FrameRows {
  ($tty:expr) => { (*(*$tty).Wcm).cm_rows };
}
macro_rules! FrameCols {
  ($tty:expr) => { (*(*$tty).Wcm).cm_cols };
}

macro_rules! UpCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_up };
}
macro_rules! DownCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_down };
}
macro_rules! LeftCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_left };
}
macro_rules! RightCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_right };
}
macro_rules! HomeCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_home };
}
macro_rules! CRCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_cr };
}
macro_rules! LastLineCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_ll };
}
macro_rules! TabCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_tab };
}
macro_rules! BackTabCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_backtab };
}
macro_rules! AbsPositionCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_abs };
}
macro_rules! ColPositionCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_habs };
}
macro_rules! RowPositionCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_vabs };
}
// cm.h 141-144 name the four `struct parmcap cc_multi*` members, which live in the
// `#if false` block at C 75-92 and therefore do not exist in `struct cm` at all: these
// four macros cannot be expanded by any C unit either (the preprocessor does not check
// member names, so the header still compiles).  The port mirrors the text as it stands -
// the same brokenness, and nothing in the tree expands them.
macro_rules! MultiUpCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_multiup };
}
macro_rules! MultiDownCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_multidown };
}
macro_rules! MultiLeftCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_multileft };
}
macro_rules! MultiRightCost {
  ($tty:expr) => { (*(*$tty).Wcm).cc_multiright };
}

// cm.h 147: `cmat (tty,row,col)	(curY(tty) = (row), curX (tty) = (col))` - the two
// assignments of C's comma expression, in C's order.
macro_rules! cmat {
  ($tty:expr, $row:expr, $col:expr) => {
    { curY! ($tty) = $row; curX! ($tty) = $col; }
  };
}

// cm.h 148-156: `cmplus (tty,n)` adds N columns and then handles the right edge.  C's
// `if ((curX (tty) += (n)) >= FrameCols (tty) && !MagicWrap (tty))` becomes the two
// statements below, and `(tty)->Wcm->cm_losewrap` the mask read (C 152, a `bool_bf`).
macro_rules! cmplus {
  ($tty:expr, $n:expr) => {
    {
      curX! ($tty) += $n;
      if curX! ($tty) >= FrameCols! ($tty) && ! MagicWrap! ($tty)
	{
	  if ((*(*$tty).Wcm).bits & CM_LOSEWRAP) != 0 { losecursor! ($tty); }
	  else if AutoWrap! ($tty) { curX! ($tty) = 0; curY! ($tty) += 1; }
	  else { curX! ($tty) -= 1; }
	}
    }
  };
}

// cm.h 158: `losecursor(tty)	 (curX(tty) = -1, curY(tty) = -1)`.
macro_rules! losecursor {
  ($tty:expr) => {
    { curX! ($tty) = -1; curY! ($tty) = -1; }
  };
}

// cm.h 160-161 and 165: the cursor-motion globals.  C's own units name locals `cost`
// (scroll.c 95 has `int cost, cost1;`), and a Rust `let` binding *cannot* shadow a static
// (E0530) - measured, not guessed: with these three glob-imported into the unit,
// `build-tu src/scroll.c` fails with three E0530s at scroll.rs 171/562/1004.  They
// therefore live in a module the mirror does not glob-import (the strftime.rs
// `mod standalone` shape), and a unit that means C's global writes `cm_globals::cost`.
// The header's declarations are all still here, which is what the mirror check reads.
pub mod cm_globals {
  use super::*;

  unsafe extern "C" {
    pub fn evalcost (c: c_int) -> c_int;
  }
  unsafe extern "C-unwind" {
    pub static mut cost: c_int;

    pub static mut current_tty: *mut tty_display_info;
  }
}

// cm.h 166-171's six functions.  A function item *can* be shadowed by a local binding, so
// these are glob-imported the way the rest of the port declares a unit's entry points
// (P-21/P-39: cm.rs's own definitions shadow the import).
pub mod cm_externs {
  use super::*;

  unsafe extern "C-unwind" {
    pub fn cmcheckmagic (tty: *mut tty_display_info);
  }
  unsafe extern "C" {
    pub fn cmputc (c: c_int) -> c_int;
  }
  unsafe extern "C-unwind" {
    pub fn cmcostinit (tty: *mut tty_display_info);
    pub fn cmgoto (tty: *mut tty_display_info, row: c_int, col: c_int);
    pub fn Wcm_clear (tty: *mut tty_display_info);
    pub fn Wcm_init (tty: *mut tty_display_info) -> c_int;
  }
}

pub use self::cm_externs::*;

// cm.h 163: `emacs_tputs(tty, str, affcnt, putc) (current_tty = (tty), tputs (str, affcnt, putc))`.
// The third argument is a function pointer in C and an `Option<...>` in the port's
// `tputs` (tparam.rs), so the expansion wraps it in `Some`.
macro_rules! emacs_tputs {
  ($tty:expr, $str:expr, $affcnt:expr, $putc:expr) => {
    { current_tty = ($tty); tputs ($str, $affcnt, Some ($putc)) }
  };
}
