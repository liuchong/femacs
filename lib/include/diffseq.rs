// femacs: port of lib/diffseq.h @ fd7afced403 (gnulib); PORTING.md 0.1.

// A *template* header: gnulib's contract (C 49-84) has the includer define `ELEMENT`, `EQUAL`,
// `OFFSET`, `EXTRA_CONTEXT_FIELDS`, `NOTE_DELETE`, `NOTE_INSERT` - and optionally `NOTE_ORDERED`,
// `EARLY_ABORT`, `USE_HEURISTIC`, or the abstract-array pair `XVECREF_YVECREF_EQUAL` with no
// `ELEMENT` - and C `#include`s it once per translation unit that wants the two `static`
// functions.  The mirror keeps that shape: the includer's names stay C's names, supplied as
// items in the unit that includes this file (`type OFFSET = …;`, `type ELEMENT = …;`,
// `macro_rules! EQUAL`, …), which is why the file has no compiler of its own - its gate is
// `extract-abi.py --check` and its contract is recorded in docs/ports/include-diffseq.md.
// Measured: **no file in the reference tree includes diffseq.h** (`lib/gnulib.mk.in` 1901 lists
// it in `libgnu_a_SOURCES` and nothing else names it), so no unit reaches this mirror yet.
//
// The frozen configuration's arms are the *abstract-array* ones: `ELEMENT` is undefined, so the
// `#ifdef ELEMENT` arms are dead in `struct context` (no `xvec`/`yvec` fields), in `diag` and in
// `compareseq` (no `xv`/`yv` locals), and `XREF_YREF_EQUAL` is the
// `XVECREF_YVECREF_EQUAL (ctxt, x, y)` arm; `USE_HEURISTIC` is undefined, so `diag`'s
// `heuristic` is C's `false`; `NOTE_ORDERED` and `EARLY_ABORT` take their `false` defaults and
// `OFFSET_MAX` its portable expression.  All thirteen conditions are decided
// (`tools/check-cond-eval.py lib/diffseq.h` -> `13 conditions, 0 unknown, 0 disagreements
// [VERIFIED]`), so the ELEMENT/EQUAL arms and `USE_HEURISTIC`'s `ctxt->heuristic` are recorded
// in the ledger rather than transcribed (PORTING.md P-08).
//
// Two shapes C spells as text have no Rust counterpart, both recorded in the ledger:
//   * `EXTRA_CONTEXT_FIELDS` splices *fields* into `struct context`.  Rust cannot splice fields,
//     so they get their own member `pub extra: EXTRA_CONTEXT_FIELDS` and the includer supplies
//     that type; its `NOTE_*` macros then write `(*ctxt).extra.<field>`.
//   * `XREF_YREF_EQUAL` is *defined by this file and used inside it*, and a `macro_rules!`
//     cannot name the caller's local `ctxt` (PORTING.md P-33's hygiene note), so the port passes
//     it: this macro takes three arguments and the call sites read
//     `XREF_YREF_EQUAL! (ctxt, x, y)` - C's two-argument spelling cannot be mirrored.
// C's `#pragma GCC diagnostic push`/`ignored …`/`pop` (C 106-116, C 571-573) are compiler
// diagnostics with no Rust counterpart, and the file's closing `#undef` block is C's own hygiene
// (each `#include` in C re-expands the template; in Rust the `macro_rules!`s are defined once).

include!("limits.in.rs");   // the contract's `<limits.h>` (CHAR_BIT), which C 83-84 requires
include!("minmax.rs");      // the contract's `minmax.h` (MIN, MAX)

/* Maximum value of type OFFSET.  The 1u pacifies -Wuseless-cast, and
   unlike a compound literal can appear in an integer constant expression.  */
// C 86-90: the default definition; an includer may define OFFSET_MAX before including the file.
macro_rules! OFFSET_MAX {
  () => { ((((((1 as OFFSET) << (core::mem::size_of::<OFFSET> () * CHAR_BIT as usize - 2)) - 1) * 2) + 1)) };
}

/* Default to no early abort.  */
// C 92-95: a function-like macro, so a `macro_rules!` (the C text's `ctxt` parameter is kept).
macro_rules! EARLY_ABORT {
  ($ctxt:expr) => { false };
}

// C 97-99's `#ifndef NOTE_ORDERED / # define NOTE_ORDERED false`: a value, so a `pub const`
// (the port's spelling for a value macro; the includer may define its own before including).
pub const NOTE_ORDERED: bool = false;

// C 149: `#define SNAKE_LIMIT 20` sits inside `struct context` in C.  The port declares C's
// value with the includer's `OFFSET` type, which is the type C's uses promote it to
// (`x - x0 > SNAKE_LIMIT`, `k == SNAKE_LIMIT - 1`).
pub const SNAKE_LIMIT: OFFSET = 20;

/*
 * Context of comparison operation.
 */
// C 165-193.  `EXTRA_CONTEXT_FIELDS` is the includer's field list (see the note at the top);
// the record is not `derive`d because the includer's own type decides whether that is possible
// (PORTING.md 0.1-9 asks for `Copy` only on union members, and this type is never one).
#[repr(C)]
pub struct context
{
  /// C: the includer's `EXTRA_CONTEXT_FIELDS` - one member here, because Rust cannot splice
  /// fields into a struct (docs/ports/include-diffseq.md records the deviation).
  pub extra: EXTRA_CONTEXT_FIELDS,

  /* Vector, indexed by diagonal, containing 1 + the X coordinate of the point
     furthest along the given diagonal in the forward search of the edit
     matrix.  */
  pub fdiag: *mut OFFSET,

  /* Vector, indexed by diagonal, containing the X coordinate of the point
     furthest along the given diagonal in the backward search of the edit
     matrix.  */
  pub bdiag: *mut OFFSET,

  /* Edit scripts longer than this are too expensive to compute.  */
  pub too_expensive: OFFSET,
}

// C 195-212.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct partition
{
  /* Midpoints of this partition.  */
  pub xmid: OFFSET,
  pub ymid: OFFSET,

  /* True if low half will be analyzed minimally.  */
  pub lo_minimal: bool,

  /* Likewise for high half.  */
  pub hi_minimal: bool,
}

// C 202-204 and C 472-474: C defines `XREF_YREF_EQUAL` inside each function (the abstract-array
// arm, since `ELEMENT` is undefined) and `#undef`s it at the end of each; the port defines it
// once, before its users (PORTING.md 1.18/P-57), with `ctxt` passed in (see the note at the top).
macro_rules! XREF_YREF_EQUAL {
  ($ctxt:expr, $x:expr, $y:expr) => { XVECREF_YVECREF_EQUAL ($ctxt, $x, $y) };
}


/* Find the midpoint of the shortest edit script for a specified portion
   of the two vectors.

   Scan from the beginnings of the vectors, and simultaneously from the ends,
   doing a breadth-first search through the space of edit-sequence.
   When the two searches meet, we have found the midpoint of the shortest
   edit sequence.

   If FIND_MINIMAL is true, find the minimal edit script regardless of
   expense.  Otherwise, if the search is too expensive, use heuristics to
   stop the search and report a suboptimal answer.

   Set PART->(xmid,ymid) to the midpoint (XMID,YMID).  The diagonal number
   XMID - YMID equals the number of inserted elements minus the number
   of deleted elements (counting only elements before the midpoint).

   Set PART->lo_minimal to true iff the minimal edit script for the
   left half of the partition is known; similarly for PART->hi_minimal.

   This function assumes that the first elements of the specified portions
   of the two vectors do not match, and likewise that the last elements do not
   match.  The caller must trim matching elements from the beginning and end
   of the portions it is going to specify.

   If we return the "wrong" partitions, the worst this can do is cause
   suboptimal diff output.  It cannot cause incorrect diff output.  */

fn diag (xoff: OFFSET, xlim: OFFSET, yoff: OFFSET, ylim: OFFSET, find_minimal: bool,
         part: *mut partition, ctxt: *mut context)
{
  let fd: *mut OFFSET = unsafe { (*ctxt).fdiag };      /* Give the compiler a chance. */
  let bd: *mut OFFSET = unsafe { (*ctxt).bdiag };      /* Additional help for the compiler. */
  let dmin: OFFSET = xoff - ylim;       /* Minimum valid diagonal. */
  let dmax: OFFSET = xlim - yoff;       /* Maximum valid diagonal. */
  let fmid: OFFSET = xoff - yoff;       /* Center diagonal of top-down search. */
  let bmid: OFFSET = xlim - ylim;       /* Center diagonal of bottom-up search. */
  let mut fmin: OFFSET = fmid;
  let mut fmax: OFFSET = fmid;          /* Limits of top-down search. */
  let mut bmin: OFFSET = bmid;
  let mut bmax: OFFSET = bmid;          /* Limits of bottom-up search. */
  let mut c: OFFSET;                    /* Cost. */
  let odd: bool = ((fmid - bmid) & 1) != 0;  /* True if southeast corner is on an odd
                                                diagonal with respect to the northwest. */

  unsafe { *fd.offset (fmid) = xoff; }
  unsafe { *bd.offset (bmid) = xlim; }

  c = 1;
  loop
    {
      let mut d: OFFSET;                 /* Active diagonal. */
      let mut big_snake: bool = false;

      /* Extend the top-down search by an edit step in each diagonal. */
      if fmin > dmin
        {
          fmin -= 1;
          unsafe { *fd.offset (fmin - 1) = -1; }
        }
      else
        {
          fmin += 1;
        }
      if fmax < dmax
        {
          fmax += 1;
          unsafe { *fd.offset (fmax + 1) = -1; }
        }
      else
        {
          fmax -= 1;
        }
      d = fmax;
      while d >= fmin
        {
          let tlo: OFFSET = unsafe { *fd.offset (d - 1) };
          let thi: OFFSET = unsafe { *fd.offset (d + 1) };
          let x0: OFFSET = if tlo < thi { thi } else { tlo + 1 };
          let mut x: OFFSET = x0;
          let mut y: OFFSET = x0 - d;

          while x < xlim && y < ylim && XREF_YREF_EQUAL! (ctxt, x, y)
            {
              x += 1;
              y += 1;
              continue;
            }
          if x - x0 > SNAKE_LIMIT
            {
              big_snake = true;
            }
          unsafe { *fd.offset (d) = x; }
          if odd && bmin <= d && d <= bmax && unsafe { *bd.offset (d) } <= x
            {
              unsafe { (*part).xmid = x; }
              unsafe { (*part).ymid = y; }
              unsafe { (*part).lo_minimal = true; (*part).hi_minimal = true; }
              return;
            }
          d -= 2;
        }

      /* Similarly extend the bottom-up search.  */
      if bmin > dmin
        {
          bmin -= 1;
          unsafe { *bd.offset (bmin - 1) = OFFSET_MAX! (); }
        }
      else
        {
          bmin += 1;
        }
      if bmax < dmax
        {
          bmax += 1;
          unsafe { *bd.offset (bmax + 1) = OFFSET_MAX! (); }
        }
      else
        {
          bmax -= 1;
        }
      d = bmax;
      while d >= bmin
        {
          let tlo: OFFSET = unsafe { *bd.offset (d - 1) };
          let thi: OFFSET = unsafe { *bd.offset (d + 1) };
          let x0: OFFSET = if tlo < thi { tlo } else { thi - 1 };
          let mut x: OFFSET = x0;
          let mut y: OFFSET = x0 - d;

          while xoff < x && yoff < y && XREF_YREF_EQUAL! (ctxt, x - 1, y - 1)
            {
              x -= 1;
              y -= 1;
              continue;
            }
          if x0 - x > SNAKE_LIMIT
            {
              big_snake = true;
            }
          unsafe { *bd.offset (d) = x; }
          if ! odd && fmin <= d && d <= fmax && x <= unsafe { *fd.offset (d) }
            {
              unsafe { (*part).xmid = x; }
              unsafe { (*part).ymid = y; }
              unsafe { (*part).lo_minimal = true; (*part).hi_minimal = true; }
              return;
            }
          d -= 2;
        }

      if ! find_minimal
        {
          /* C's live arm (`#else` of `#ifdef USE_HEURISTIC`): the frozen configuration has
             USE_HEURISTIC undefined, so the heuristic flag is C's `false`.  */
          let heuristic: bool = false;

          /* Heuristic: check occasionally for a diagonal that has made lots
             of progress compared with the edit distance.  If we have any
             such, find the one that has made the most progress and return it
             as if it had succeeded.

             With this heuristic, for vectors with a constant small density
             of changes, the algorithm is linear in the vector size.  */

          if 200 < c && big_snake && heuristic
            {
              {
                let mut best: OFFSET = 0;

                d = fmax;
                while d >= fmin
                  {
                    let dd: OFFSET = d - fmid;
                    let x: OFFSET = unsafe { *fd.offset (d) };
                    let y: OFFSET = x - d;
                    let v: OFFSET = (x - xoff) * 2 - dd;

                    if v > 12 * (c + (if dd < 0 { - dd } else { dd }))
                      {
                        if v > best
                            && xoff + SNAKE_LIMIT <= x && x < xlim
                            && yoff + SNAKE_LIMIT <= y && y < ylim
                          {
                            /* We have a good enough best diagonal; now insist
                               that it end with a significant snake.  */
                            let mut k: OFFSET = 1;
                            while XREF_YREF_EQUAL! (ctxt, x - k, y - k)
                              {
                                if k == SNAKE_LIMIT
                                  {
                                    best = v;
                                    unsafe { (*part).xmid = x; }
                                    unsafe { (*part).ymid = y; }
                                    break;
                                  }
                                k += 1;
                              }
                          }
                      }
                    d -= 2;
                  }
                if best > 0
                  {
                    unsafe { (*part).lo_minimal = true; }
                    unsafe { (*part).hi_minimal = false; }
                    return;
                  }
              }

              {
                let mut best: OFFSET = 0;

                d = bmax;
                while d >= bmin
                  {
                    let dd: OFFSET = d - bmid;
                    let x: OFFSET = unsafe { *bd.offset (d) };
                    let y: OFFSET = x - d;
                    let v: OFFSET = (xlim - x) * 2 + dd;

                    if v > 12 * (c + (if dd < 0 { - dd } else { dd }))
                      {
                        if v > best
                            && xoff < x && x <= xlim - SNAKE_LIMIT
                            && yoff < y && y <= ylim - SNAKE_LIMIT
                          {
                            /* We have a good enough best diagonal; now insist
                               that it end with a significant snake.  */
                            let mut k: OFFSET = 0;
                            while XREF_YREF_EQUAL! (ctxt, x + k, y + k)
                              {
                                if k == SNAKE_LIMIT - 1
                                  {
                                    best = v;
                                    unsafe { (*part).xmid = x; }
                                    unsafe { (*part).ymid = y; }
                                    break;
                                  }
                                k += 1;
                              }
                          }
                      }
                    d -= 2;
                  }
                if best > 0
                  {
                    unsafe { (*part).lo_minimal = false; }
                    unsafe { (*part).hi_minimal = true; }
                    return;
                  }
              }
            }

          /* Heuristic: if we've gone well beyond the call of duty, give up
             and report halfway between our best results so far.  */
          if c >= unsafe { (*ctxt).too_expensive }
            {
              /* Find forward diagonal that maximizes X + Y.  */
              let mut fxybest: OFFSET = -1;
              let mut fxbest: OFFSET = 0;   /* C leaves it uninitialised (PORTING.md 1.5).  */
              d = fmax;
              while d >= fmin
                {
                  let mut x: OFFSET = MIN! (unsafe { *fd.offset (d) }, xlim);
                  let mut y: OFFSET = x - d;
                  if ylim < y
                    {
                      x = ylim + d;
                      y = ylim;
                    }
                  if fxybest < x + y
                    {
                      fxybest = x + y;
                      fxbest = x;
                    }
                  d -= 2;
                }

              /* Find backward diagonal that minimizes X + Y.  */
              let mut bxybest: OFFSET = OFFSET_MAX! ();
              let mut bxbest: OFFSET = 0;   /* C leaves it uninitialised (PORTING.md 1.5).  */
              d = bmax;
              while d >= bmin
                {
                  let mut x: OFFSET = MAX! (xoff, unsafe { *bd.offset (d) });
                  let mut y: OFFSET = x - d;
                  if y < yoff
                    {
                      x = yoff + d;
                      y = yoff;
                    }
                  if x + y < bxybest
                    {
                      bxybest = x + y;
                      bxbest = x;
                    }
                  d -= 2;
                }

              /* Use the better of the two diagonals.  */
              if (xlim + ylim) - bxybest < fxybest - (xoff + yoff)
                {
                  unsafe { (*part).xmid = fxbest; }
                  unsafe { (*part).ymid = fxybest - fxbest; }
                  unsafe { (*part).lo_minimal = true; }
                  unsafe { (*part).hi_minimal = false; }
                }
              else
                {
                  unsafe { (*part).xmid = bxbest; }
                  unsafe { (*part).ymid = bxybest - bxbest; }
                  unsafe { (*part).lo_minimal = false; }
                  unsafe { (*part).hi_minimal = true; }
                }
              return;
            }
        }
      c += 1;
    }
}


/* Compare in detail contiguous subsequences of the two vectors
   which are known, as a whole, to match each other.

   The subsequence of vector 0 is [XOFF, XLIM) and likewise for vector 1.

   Note that XLIM, YLIM are exclusive bounds.  All indices into the vectors
   are origin-0.

   If FIND_MINIMAL, find a minimal difference no matter how
   expensive it is.

   The results are recorded by invoking NOTE_DELETE and NOTE_INSERT.

   Return false if terminated normally, or true if terminated through early
   abort.  */

fn compareseq (mut xoff: OFFSET, mut xlim: OFFSET, mut yoff: OFFSET, mut ylim: OFFSET,
               mut find_minimal: bool, ctxt: *mut context) -> bool
{
  while true
    {
      /* Slide down the bottom initial diagonal.  */
      while xoff < xlim && yoff < ylim && XREF_YREF_EQUAL! (ctxt, xoff, yoff)
        {
          xoff += 1;
          yoff += 1;
        }

      /* Slide up the top initial diagonal. */
      while xoff < xlim && yoff < ylim && XREF_YREF_EQUAL! (ctxt, xlim - 1, ylim - 1)
        {
          xlim -= 1;
          ylim -= 1;
        }

      /* Handle simple cases. */
      if xoff == xlim
        {
          while yoff < ylim
            {
              NOTE_INSERT! (ctxt, yoff);
              if EARLY_ABORT! (ctxt)
                {
                  return true;
                }
              yoff += 1;
            }
          break;
        }
      if yoff == ylim
        {
          while xoff < xlim
            {
              NOTE_DELETE! (ctxt, xoff);
              if EARLY_ABORT! (ctxt)
                {
                  return true;
                }
              xoff += 1;
            }
          break;
        }

      let mut part: partition = unsafe { std::mem::zeroed () };

      /* Find a point of correspondence in the middle of the vectors.  */
      diag (xoff, xlim, yoff, ylim, find_minimal, &mut part, ctxt);

      /* Use the partitions to split this problem into subproblems.  */
      let mut xoff1: OFFSET; let mut xlim1: OFFSET;
      let mut yoff1: OFFSET; let mut ylim1: OFFSET;
      let mut xoff2: OFFSET; let mut xlim2: OFFSET;
      let mut yoff2: OFFSET; let mut ylim2: OFFSET;
      let mut find_minimal1: bool; let mut find_minimal2: bool;
      if ! NOTE_ORDERED
          && ((xlim + ylim) - (part.xmid + part.ymid)
              < (part.xmid + part.ymid) - (xoff + yoff))
        {
          /* The second problem is smaller and the caller doesn't
             care about order, so do the second problem first to
             lessen recursion.  */
          xoff1 = part.xmid; xlim1 = xlim;
          yoff1 = part.ymid; ylim1 = ylim;
          find_minimal1 = part.hi_minimal;

          xoff2 = xoff; xlim2 = part.xmid;
          yoff2 = yoff; ylim2 = part.ymid;
          find_minimal2 = part.lo_minimal;
        }
      else
        {
          xoff1 = xoff; xlim1 = part.xmid;
          yoff1 = yoff; ylim1 = part.ymid;
          find_minimal1 = part.lo_minimal;

          xoff2 = part.xmid; xlim2 = xlim;
          yoff2 = part.ymid; ylim2 = ylim;
          find_minimal2 = part.hi_minimal;
        }

      /* Recurse to do one subproblem.  */
      let early: bool = compareseq (xoff1, xlim1, yoff1, ylim1, find_minimal1, ctxt);
      if early
        {
          return early;
        }

      /* Iterate to do the other subproblem.  */
      xoff = xoff2; xlim = xlim2;
      yoff = yoff2; ylim = ylim2;
      find_minimal = find_minimal2;
    }

  return false;
}
