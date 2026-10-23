// femacs: port of lib/str-two-way.h @ fd7afced403.  Header mirror
// (PORTING.md 0.1): the C text is the Three-Way (Chrochemore-Perrin) byte-substring search
// that lib/memmem.c injects into its own translation unit, and its three functions are
// `static`, so each including unit gets its own copy — the port's `#[inline(always)] pub fn`
// here is that same "one copy per unit, no exported symbol" shape (the timespec.rs/c-ctype.rs
// precedent for header-inline definitions).
//
// C's parameterization is kept as macros: `RETURN_TYPE`, `AVAILABLE`, and the two optional
// bindings `CANON_ELEMENT`/`CMP_FUNC` come from the *including unit* (memmem.c defines the
// first two at C 27-28 and leaves the optional pair at these defaults), and the port's
// `macro_rules!` are text-ordered the same way: the including unit must define `RETURN_TYPE`
// and `AVAILABLE` before this file's `include!` (PORTING.md 0.1-5/P-57), exactly as C requires
// them before C 29's `#include "str-two-way.h"`.  `lib/memmem.c` is the only consumer in this
// tree, so no second instantiation exists (PORTING.md 10.13②).
//
// `LONG_NEEDLE_THRESHOLD` has two definitions in C (C 65's `32U` for `CHAR_BIT < 10` and
// C 67's `SIZE_MAX`); this header is not in cond-eval-verified.txt, so the extractor treats
// both as live and compares presence, not the body (the branch-macro rule, P-08/P-66).  The
// port keeps the 8-bit arm, which is this target's.
/* No `#![allow]` here: this file is `include!`d (PORTING.md 0.1-9), and an inner attribute is
   not permitted in that position — the including unit carries the allow block.  */

include!("limits.in.rs");    // <limits.h> (CHAR_BIT)
include!("stdint.in.rs");    // <stdint.h> (SIZE_MAX)
include!("string.rs");       // <string.h> (the default `CMP_FUNC`, memcmp)

/* C 57-63's comment is above; C 64-68's `#if CHAR_BIT < 10` arm is the 8-bit one this
   target takes (CHAR_BIT is 8, macros.tsv), and the `SIZE_MAX` arm is the non-frozen side.  */
pub const LONG_NEEDLE_THRESHOLD: u32 = 32;

/* C 70-72's `#ifndef MAX` / `# define MAX(a, b) ((a < b) ? (b) : (a))`.  */
macro_rules! MAX {
    ($a:expr, $b:expr) => { if $a < $b { $b } else { $a } };
}

/* C 74-79's two optional bindings, at their defaults: `CANON_ELEMENT (c)` is the identity and
   `CMP_FUNC (p1, p2, l)` is `memcmp`.  An including unit that defines its own (strstr.c's
   case-insensitive pair) would shadow these by text order, exactly as C's `#ifndef` does.  */
macro_rules! CANON_ELEMENT {
    ($c:expr) => { $c };
}
macro_rules! CMP_FUNC {
    ($p1:expr, $p2:expr, $l:expr) => {
        memcmp ($p1 as *const c_void, $p2 as *const c_void, $l)
    };
}

/* Perform a critical factorization of NEEDLE, of length NEEDLE_LEN.
   Return the index of the first byte in the right half, and set
   *PERIOD to the global period of the right half.

   ... (the reference's 25-line explanation is in lib/str-two-way.h 81-106)  */
#[inline(always)]
pub unsafe fn critical_factorization (needle: *const u8, needle_len: usize,
                                      period: *mut usize) -> usize
{
  /* Special case NEEDLE_LEN of 1 or 2 (all callers already filtered
     out 0-length needles.  */
  if needle_len < 3
    {
      *period = 1;
      return needle_len - 1;
    }

  /* Perform lexicographic search.  */
  let mut max_suffix: usize = SIZE_MAX;   /* Index of last byte of left half, or SIZE_MAX.  */
  {
    let mut j: usize = 0;   /* Index into NEEDLE for current candidate suffix.  */
    let mut k: usize = 1;   /* Offset into current period.  */
    let mut p: usize = 1;   /* Intermediate period.  */
    while j + k < needle_len
      {
        let a: u8 = CANON_ELEMENT! (*needle.add (j + k));
        let b: u8 = CANON_ELEMENT! (*needle.add (max_suffix.wrapping_add (k)));
        if a < b
          {
            /* Suffix is smaller, period is entire prefix so far.  */
            j += k;
            k = 1;
            /* C: `p = j - max_suffix;` - `max_suffix` is seeded with `SIZE_MAX`, so the first
               iteration relies on *unsigned wraparound* (`0 - SIZE_MAX` is 1, the period the
               algorithm wants); Rust's checked subtraction panics instead (PORTING.md P-96).  */
            p = j.wrapping_sub (max_suffix);
          }
        else if a == b
          {
            /* Advance through repetition of the current period.  */
            if k != p
              { k += 1; }
            else
              {
                j += p;
                k = 1;
              }
          }
        else /* b < a */
          {
            /* Suffix is larger, start over from current location.  */
            max_suffix = j;
            j += 1;
            k = 1;
            p = 1;
          }
      }
    *period = p;
  }

  /* Perform reverse lexicographic search.  */
  let mut max_suffix_rev: usize = SIZE_MAX;  /* Index of last byte of left half, or SIZE_MAX.  */
  let mut p_rev: usize;
  {
    let mut j: usize = 0;   /* Index into NEEDLE for current candidate suffix.  */
    let mut k: usize = 1;   /* Offset into current period.  */
    let mut p: usize = 1;   /* Intermediate period.  */
    while j + k < needle_len
      {
        let a: u8 = CANON_ELEMENT! (*needle.add (j + k));
        let b: u8 = CANON_ELEMENT! (*needle.add (max_suffix_rev.wrapping_add (k)));
        if b < a
          {
            /* Suffix is smaller, period is entire prefix so far.  */
            j += k;
            k = 1;
            /* C: `p = j - max_suffix_rev;` - same `SIZE_MAX` seed and the same wraparound
               (PORTING.md P-96; this is the site the dump path's first `memmem` call panicked on).  */
            p = j.wrapping_sub (max_suffix_rev);
          }
        else if a == b
          {
            /* Advance through repetition of the current period.  */
            if k != p
              { k += 1; }
            else
              {
                j += p;
                k = 1;
              }
          }
        else /* a < b */
          {
            /* Suffix is larger, start over from current location.  */
            max_suffix_rev = j;
            j += 1;
            k = 1;
            p = 1;
          }
      }
    p_rev = p;
  }

  /* Choose the shorter suffix.  Return the index of the first byte of
     the right half, rather than the last byte of the left half.  */
  /* C 221-224: `if (max_suffix_rev + 1 < max_suffix + 1) return max_suffix + 1;` and the
     `max_suffix_rev + 1` return below.  Both operands are seeded with `SIZE_MAX` and C relies
     on *unsigned wraparound* here: with no suffix found, `SIZE_MAX + 1` is 0, the comparison is
     false, and the function returns 0 - the "no factorization" the caller expects
     (PORTING.md P-96).  Rust's checked addition panicked instead (status 4.419: `attempt to add
     with overflow`, raised by `units/memmem/...str-two-way_rs.rs`).  */
  if max_suffix_rev.wrapping_add (1) < max_suffix.wrapping_add (1)
    { return max_suffix.wrapping_add (1); }
  *period = p_rev;
  return max_suffix_rev.wrapping_add (1);
}

/* Return the first location of non-empty NEEDLE within HAYSTACK, or
   NULL.  HAYSTACK_LEN is the minimum known length of HAYSTACK.  This
   method is optimized for NEEDLE_LEN < LONG_NEEDLE_THRESHOLD.  */
#[inline(always)]
pub unsafe fn two_way_short_needle (haystack: *const u8, mut haystack_len: usize,
                                    needle: *const u8, needle_len: usize) -> RETURN_TYPE! ()
{
  /* Factor the needle into two halves, such that the left half is
     smaller than the global period, and the right half is
     periodic (with a period as large as NEEDLE_LEN - suffix).  */
  /* C leaves `period` unset on the `needle_len < 3` path; Rust requires the whole
     initialisation up front (PORTING.md 0.1-13).  */
  let mut period: usize = 0;   /* The period of the right half of needle.  */
  let suffix: usize = critical_factorization (needle, needle_len, &raw mut period);

  /* Perform the search.  Each iteration compares the right half
     first.  */
  if CMP_FUNC! (needle, needle.add (period), suffix) == 0
    {
      /* Entire needle is periodic; a mismatch in the left half can
         only advance by the period, so use memory to avoid rescanning
         known occurrences of the period in the right half.  */
      let mut memory: usize = 0;
      let mut j: usize = 0;   /* Index into current window of HAYSTACK.  */
      while AVAILABLE! (haystack, haystack_len, j, needle_len)
        {
          /* Scan for matches in right half.  */
          let mut i: usize = MAX! (suffix, memory);   /* Index into current byte of NEEDLE.  */
          while i < needle_len && CANON_ELEMENT! (*needle.add (i))
                                  == CANON_ELEMENT! (*haystack.add (i + j))
            { i += 1; }
          if needle_len <= i
            {
              /* Scan for matches in left half.  */
      /* C: `i = suffix - 1;` - with `suffix == 0` this must wrap to `SIZE_MAX`, which the
         loop right below tests for (`while i != SIZE_MAX && ...`); PORTING.md P-96.  */
              i = suffix.wrapping_sub (1);
              /* C 269: `while (memory < i + 1 && ...)` - `i` is the `SIZE_MAX` sentinel the loop
                 below tests for, so `i + 1` is *unsigned wraparound* in C (PORTING.md P-96).  */
              while memory < i.wrapping_add (1) && CANON_ELEMENT! (*needle.add (i))
                                      == CANON_ELEMENT! (*haystack.add (i + j))
                /* C: `i--;` - `i == 0` must wrap to `SIZE_MAX`, the sentinel the loop's own test
   uses (`while i != SIZE_MAX && ...`); PORTING.md P-96.  */
                { i = i.wrapping_sub (1); }
              /* C 272: `if (i + 1 < memory + 1)` - the same wraparound (P-96).  */
              if i.wrapping_add (1) < memory.wrapping_add (1)
                { return haystack.add (j) as RETURN_TYPE! (); }
              /* No match, so remember how many repetitions of period
                 on the right half were scanned.  */
              j += period;
              memory = needle_len - period;
            }
          else
            {
              /* C: `j += i - suffix + 1;` - `i - suffix` runs below zero when the search
                 restarts at the first byte, which is what C's `size_t` arithmetic does here
                 (PORTING.md P-96); Rust's checked arithmetic panicked instead (status 4.419).  */
              j = j.wrapping_add (i.wrapping_sub (suffix).wrapping_add (1));
              memory = 0;
            }
        }
    }
  else
    {
      /* The two halves of needle are distinct; no extra memory is
         required, and any mismatch results in a maximal shift.  */
      period = MAX! (suffix, needle_len - suffix) + 1;
      let mut j: usize = 0;   /* Index into current window of HAYSTACK.  */
      while AVAILABLE! (haystack, haystack_len, j, needle_len)
        {
          /* Scan for matches in right half.  */
          let mut i: usize = suffix;   /* Index into current byte of NEEDLE.  */
          while i < needle_len && CANON_ELEMENT! (*needle.add (i))
                                  == CANON_ELEMENT! (*haystack.add (i + j))
            { i += 1; }
          if needle_len <= i
            {
              /* Scan for matches in left half.  */
              i = suffix.wrapping_sub (1);
              while i != SIZE_MAX && CANON_ELEMENT! (*needle.add (i))
                                     == CANON_ELEMENT! (*haystack.add (i + j))
                { i = i.wrapping_sub (1); }
              if i == SIZE_MAX
                { return haystack.add (j) as RETURN_TYPE! (); }
              j += period;
            }
          else
            {
              /* C: `j += i - suffix + 1;` - `i - suffix` runs below zero when the search
                 restarts at the first byte, which is what C's `size_t` arithmetic does here
                 (PORTING.md P-96); Rust's checked arithmetic panicked instead (status 4.419).  */
              j = j.wrapping_add (i.wrapping_sub (suffix).wrapping_add (1));
            }
        }
    }
  return std::ptr::null_mut ();
}

/* Return the first location of non-empty NEEDLE within HAYSTACK, or
   NULL.  HAYSTACK_LEN is the minimum known length of HAYSTACK.  This
   method is optimized for LONG_NEEDLE_THRESHOLD <= NEEDLE_LEN.  */
#[inline(always)]
pub unsafe fn two_way_long_needle (haystack: *const u8, haystack_len: usize,
                                   needle: *const u8, needle_len: usize) -> RETURN_TYPE! ()
{
  /* Factor the needle into two halves, such that the left half is
     smaller than the global period, and the right half is
     periodic (with a period as large as NEEDLE_LEN - suffix).  */
  /* C leaves `period` unset on the `needle_len < 3` path; Rust requires the whole
     initialisation up front (PORTING.md 0.1-13).  */
  let mut period: usize = 0;   /* The period of the right half of needle.  */
  let suffix: usize = critical_factorization (needle, needle_len, &raw mut period);

  /* Populate shift_table.  For each possible byte value c,
     shift_table[c] is the distance from the last occurrence of c to
     the end of NEEDLE, or NEEDLE_LEN if c is absent from the NEEDLE.
     shift_table[NEEDLE[NEEDLE_LEN - 1]] contains the only 0.  */
  let mut shift_table: [usize; 1 << CHAR_BIT] = [0; 1 << CHAR_BIT];
  let mut i: usize = 0;
  while i < (1u32 << CHAR_BIT) as usize
    {
      shift_table[i] = needle_len;
      i += 1;
    }
  i = 0;
  while i < needle_len
    {
      shift_table[CANON_ELEMENT! (*needle.add (i)) as usize] = needle_len - i - 1;
      i += 1;
    }

  /* Perform the search.  Each iteration compares the right half
     first.  */
  if CMP_FUNC! (needle, needle.add (period), suffix) == 0
    {
      /* Entire needle is periodic; a mismatch in the left half can
         only advance by the period, so use memory to avoid rescanning
         known occurrences of the period in the right half.  */
      let mut memory: usize = 0;
      let mut j: usize = 0;   /* Index into current window of HAYSTACK.  */
      while AVAILABLE! (haystack, haystack_len, j, needle_len)
        {
          /* Check the last byte first; if it does not match, then
             shift to the next possible match location.  */
          let mut shift: usize =
            shift_table[CANON_ELEMENT! (*haystack.add (j + needle_len - 1)) as usize];
          if 0 < shift
            {
              if memory != 0 && shift < period
                {
                  /* Since needle is periodic, but the last period has
                     a byte out of place, there can be no match until
                     after the mismatch.  */
                  shift = needle_len - period;
                }
              memory = 0;
              j += shift;
            }
          else
            {
              /* Scan for matches in right half.  The last byte has
                 already been matched, by virtue of the shift table.  */
              let mut i: usize = MAX! (suffix, memory);
              while i < needle_len - 1 && CANON_ELEMENT! (*needle.add (i))
                                          == CANON_ELEMENT! (*haystack.add (i + j))
                { i += 1; }
              if needle_len - 1 <= i
                {
                  /* Scan for matches in left half.  */
              i = suffix.wrapping_sub (1);
                  /* C 390: `while (memory < i + 1 && ...)` - the same sentinel and wraparound
                     (PORTING.md P-96).  */
                  while memory < i.wrapping_add (1) && CANON_ELEMENT! (*needle.add (i))
                                          == CANON_ELEMENT! (*haystack.add (i + j))
                    { i = i.wrapping_sub (1); }
                  /* C 393: `if (i + 1 < memory + 1)` - the same wraparound (P-96).  */
                  if i.wrapping_add (1) < memory.wrapping_add (1)
                    { return haystack.add (j) as RETURN_TYPE! (); }
                  /* No match, so remember how many repetitions of period
                     on the right half were scanned.  */
                  j += period;
                  memory = needle_len - period;
                }
              else
                {
                  /* C: `j += i - suffix + 1;` - `i - suffix` runs below zero when the search
                     restarts at the first byte, which is what C's `size_t` arithmetic does here
                     (PORTING.md P-96); Rust's checked arithmetic panicked instead (status 4.419).  */
                  j = j.wrapping_add (i.wrapping_sub (suffix).wrapping_add (1));
                  memory = 0;
                }
            }
        }
    }
  else
    {
      /* The two halves of needle are distinct; no extra memory is
         required, and any mismatch results in a maximal shift.  */
      period = MAX! (suffix, needle_len - suffix) + 1;
      let mut j: usize = 0;   /* Index into current window of HAYSTACK.  */
      while AVAILABLE! (haystack, haystack_len, j, needle_len)
        {
          /* Check the last byte first; if it does not match, then
             shift to the next possible match location.  */
          let shift: usize =
            shift_table[CANON_ELEMENT! (*haystack.add (j + needle_len - 1)) as usize];
          if 0 < shift
            { j += shift; }
          else
            {
              /* Scan for matches in right half.  The last byte has
                 already been matched, by virtue of the shift table.  */
              let mut i: usize = suffix;
              while i < needle_len - 1 && CANON_ELEMENT! (*needle.add (i))
                                          == CANON_ELEMENT! (*haystack.add (i + j))
                { i += 1; }
              if needle_len - 1 <= i
                {
                  /* Scan for matches in left half.  */
              i = suffix.wrapping_sub (1);
                  while i != SIZE_MAX && CANON_ELEMENT! (*needle.add (i))
                                         == CANON_ELEMENT! (*haystack.add (i + j))
                    { i = i.wrapping_sub (1); }
                  if i == SIZE_MAX
                    { return haystack.add (j) as RETURN_TYPE! (); }
                  j += period;
                }
              else
                {
                  /* C: `j += i - suffix + 1;` - `i - suffix` runs below zero when the search
                     restarts at the first byte, which is what C's `size_t` arithmetic does here
                     (PORTING.md P-96); Rust's checked arithmetic panicked instead (status 4.419).  */
                  j = j.wrapping_add (i.wrapping_sub (suffix).wrapping_add (1));
                }
            }
        }
    }
  return std::ptr::null_mut ();
}

/* C 451-455's `#undef AVAILABLE` / `CANON_ELEMENT` / `CMP_FUNC` / `MAX` / `RETURN_TYPE`: the
   port's macros are text-scoped by their definitions, and lib/memmem.rs undefines its own
   pair at the same place.  */
