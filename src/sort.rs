// femacs: port of src/sort.c @ fd7afced403.
//
// One-to-one port: same statements in the same order, same names, same comments
// (PORTING.md).  Raw pointers keep C's arithmetic and aliasing; the forward
// `goto Succeed` / `goto CopyB` tails become the labels of one state machine,
// because Rust has no goto and the tails must not be skipped (PORTING.md 2.2).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;

/* This is a version of the cpython code implementing the TIMSORT
   sorting algorithm described in
   https://github.com/python/cpython/blob/main/Objects/listsort.txt.
   This algorithm identifies and pushes naturally ordered sublists of
   the original list, or "runs", onto a stack, and merges them
   periodically according to a merge strategy called "powersort".
   State is maintained during the sort in a merge_state structure,
   which is passed around as an argument to all the subroutines.  A
   "stretch" structure includes a pointer to the run BASE of length
   LEN along with its POWER (a computed integer used by the powersort
   merge strategy that depends on this run and the succeeding run.)  */


// [U1] config.rs provided by common
// [U1] ../lib/include/stdio.rs imported via common
// [U1] include/lisp.rs imported via common

use core::mem::MaybeUninit;
use core::ptr;

/* Reverse a slice of a vector in place, from lo up to (exclusive) hi. */
unsafe fn reverse_slice (mut lo: *mut Lisp_Object, mut hi: *mut Lisp_Object)
{
  hi = hi.sub (1);
  while lo < hi
    {
      let t = *lo;
      *lo = *hi;
      *hi = t;
      lo = lo.add (1);
      hi = hi.sub (1);
    }
}

/* A sortslice contains a pointer to an array of keys and a pointer to
   an array of corresponding values.  In other words, keys[i]
   corresponds with values[i].  If values == NULL, then the keys are
   also the values.

   Several convenience routines are provided here, so that keys and
   values are always moved in sync.  */

#[repr(C)]
#[derive(Clone, Copy)]
struct sortslice
{
  keys: *mut Lisp_Object,
  values: *mut Lisp_Object,
}

/* FIXME: Instead of values=NULL, can we set values=keys, so that they
   are both moved in lockstep and we avoid a lot of branches?
   We may do some useless work but it might be cheaper overall. */

#[inline]
unsafe fn sortslice_copy (s1: *mut sortslice, i: ptrdiff_t, s2: *mut sortslice, j: ptrdiff_t)
{
  *(*s1).keys.offset (i) = *(*s2).keys.offset (j);
  if !(*s1).values.is_null ()
    { *(*s1).values.offset (i) = *(*s2).values.offset (j); }
}

#[inline]
unsafe fn sortslice_copy_incr (dst: *mut sortslice, src: *mut sortslice)
{
  { let t = *(*src).keys; (*src).keys = (*src).keys.add (1); *(*dst).keys = t; (*dst).keys = (*dst).keys.add (1); }
  if !(*dst).values.is_null ()
    { let t = *(*src).values; (*src).values = (*src).values.add (1); *(*dst).values = t; (*dst).values = (*dst).values.add (1); }
}

#[inline]
unsafe fn sortslice_copy_decr (dst: *mut sortslice, src: *mut sortslice)
{
  { let t = *(*src).keys; (*src).keys = (*src).keys.sub (1); *(*dst).keys = t; (*dst).keys = (*dst).keys.sub (1); }
  if !(*dst).values.is_null ()
    { let t = *(*src).values; (*src).values = (*src).values.sub (1); *(*dst).values = t; (*dst).values = (*dst).values.sub (1); }
}


#[inline]
unsafe fn sortslice_memcpy (s1: *mut sortslice, i: ptrdiff_t, s2: *mut sortslice, j: ptrdiff_t,
                            n: ptrdiff_t)
{
  ptr::copy_nonoverlapping ((*s2).keys.offset (j), (*s1).keys.offset (i), n as usize);
  if !(*s1).values.is_null ()
    { ptr::copy_nonoverlapping ((*s2).values.offset (j), (*s1).values.offset (i), n as usize); }
}

#[inline]
unsafe fn sortslice_memmove (s1: *mut sortslice, i: ptrdiff_t, s2: *mut sortslice, j: ptrdiff_t,
                             n: ptrdiff_t)
{
  ptr::copy ((*s2).keys.offset (j), (*s1).keys.offset (i), n as usize);
  if !(*s1).values.is_null ()
    { ptr::copy ((*s2).values.offset (j), (*s1).values.offset (i), n as usize); }
}

#[inline]
unsafe fn sortslice_advance (slice: *mut sortslice, n: ptrdiff_t)
{
  (*slice).keys = (*slice).keys.offset (n);
  if !(*slice).values.is_null ()
    { (*slice).values = (*slice).values.offset (n); }
}


/* MAX_MERGE_PENDING is the maximum number of entries in merge_state's
   pending-stretch stack.  For a list with n elements, this needs at most
   floor(log2(n)) + 1 entries even if we didn't force runs to a
   minimal length.  So the number of bits in a ptrdiff_t is plenty large
   enough for all cases.  */

const MAX_MERGE_PENDING: usize = PTRDIFF_WIDTH;

/* Once we get into galloping mode, we stay there as long as both runs
   win at least GALLOP_WIN_MIN consecutive times.  */

const GALLOP_WIN_MIN: ptrdiff_t = 7;

/* A small temp array of size MERGESTATE_TEMP_SIZE is used to avoid
   malloc when merging small lists.  */

const MERGESTATE_TEMP_SIZE: usize = 256;

#[repr(C)]
#[derive(Clone, Copy)]
struct stretch
{
  base: sortslice,
  len: ptrdiff_t,
  power: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct reloc
{
  src: *mut sortslice,
  dst: *mut sortslice,
  size: *mut ptrdiff_t,
  order: c_int, /* -1 while in merge_lo; +1 while in merg_hi; 0 otherwise.  */
}


#[repr(C)]
struct merge_state
{
  basekeys: *mut Lisp_Object,
  allocated_keys: *mut Lisp_Object,	/* heap-alloc'ed key array or NULL */
  listlen: ptrdiff_t,

  /* PENDING is a stack of N pending stretches yet to be merged.
     Stretch #i starts at address base[i] and extends for len[i]
     elements.  */

  n: c_int,
  pending: [stretch; MAX_MERGE_PENDING],

  /* The variable MIN_GALLOP, initialized to GALLOP_WIN_MIN, controls
     when we get *into* galloping mode.  merge_lo and merge_hi tend to
     nudge it higher for random data, and lower for highly structured
     data.  */

  min_gallop: ptrdiff_t,

  /* 'A' is temporary storage, able to hold ALLOCED elements, to help
     with merges.  'A' initially points to TEMPARRAY, and subsequently
     to newly allocated memory if needed.  */

  a: sortslice,
  alloced: ptrdiff_t,
  count: specpdl_ref,
  temparray: [Lisp_Object; MERGESTATE_TEMP_SIZE],

  /* If an exception is thrown while merging we might have to relocate
     some list elements from temporary storage back into the list.
     RELOC keeps track of the information needed to do this.  */

  reloc: reloc,

  /* The C ordering (less-than) predicate.  */
  pred_fun: Option<unsafe extern "C-unwind" fn (ms: *mut merge_state, a: Lisp_Object, b: Lisp_Object) -> bool>,

  /* The Lisp ordering predicate; Qnil means value<.  */
  predicate: Lisp_Object,
}


unsafe extern "C-unwind" fn order_pred_lisp (ms: *mut merge_state, a: Lisp_Object, b: Lisp_Object) -> bool
{
  return !NILP (calln! ((*ms).predicate, a, b));
}

unsafe extern "C-unwind" fn order_pred_lss (ms: *mut merge_state, a: Lisp_Object, b: Lisp_Object) -> bool
{
  if FIXNUMP (a) && FIXNUMP (b) && !debug_on_next_call
    {
      maybe_quit ();
      return XFIXNUM (a) < XFIXNUM (b);
    }
  return !NILP (calln! ((*ms).predicate, a, b));
}

unsafe extern "C-unwind" fn order_pred_gtr (ms: *mut merge_state, a: Lisp_Object, b: Lisp_Object) -> bool
{
  if FIXNUMP (a) && FIXNUMP (b) && !debug_on_next_call
    {
      maybe_quit ();
      return XFIXNUM (a) > XFIXNUM (b);
    }
  return !NILP (calln! ((*ms).predicate, a, b));
}

unsafe extern "C-unwind" fn order_pred_valuelt (ms: *mut merge_state, a: Lisp_Object, b: Lisp_Object) -> bool
{
  if FIXNUMP (a) && FIXNUMP (b)
    { return XFIXNUM (a) < XFIXNUM (b); }
  return !NILP (Fvaluelt (a, b));
}

/* Return true iff A < B according to the order predicate.  */
#[inline(always)]
unsafe fn inorder (ms: *mut merge_state, a: Lisp_Object, b: Lisp_Object) -> bool
{
  /* C calls through the pointer directly, so a NULL pointer would crash; the
     port's `unwrap_unchecked` has exactly that precondition.  */
  let pred_fn = (*ms).pred_fun.unwrap_unchecked ();
  if pred_fn as usize == order_pred_lss as *const () as usize
     && FIXNUMP (a) && FIXNUMP (b) && !debug_on_next_call
    {
      maybe_quit ();
      return XFIXNUM (a) < XFIXNUM (b);
    }
  return pred_fn (ms, a, b);
}

/* Sort the list starting at LO and ending at HI using a stable binary
   insertion sort algorithm. On entry the sublist [LO, START) (with
   START between LO and HIGH) is known to be sorted (pass START == LO
   if you are unsure).  Even in case of error, the output will be some
   permutation of the input (nothing is lost or duplicated).  */

unsafe fn binarysort (ms: *mut merge_state, lo: sortslice, hi: *const Lisp_Object,
                      mut start: *mut Lisp_Object)
{
  eassume! (lo.keys <= start && start <= hi as *mut Lisp_Object);
  if lo.keys == start
    { start = start.add (1); }
  while start < hi as *mut Lisp_Object
    {
      let mut l = lo.keys;
      let mut r = start;
      let mut pivot = *r;

      eassume! (l < r);
      loop {
        let p = l.offset (r.offset_from (l) >> 1);
        if inorder (ms, pivot, *p)
          { r = p; }
        else
          { l = p.add (1); }
        if !(l < r) { break; }
      }
      eassume! (l == r);
      { let mut p = start; while p > l { *p = *p.sub (1); p = p.sub (1); } }
      *l = pivot;

      if !lo.values.is_null ()
        {
          let offset = lo.values.offset_from (lo.keys);
          let mut p = start.offset (offset);
          pivot = *p;
          l = l.offset (offset);
          { let mut p = start.offset (offset); while p > l { *p = *p.sub (1); p = p.sub (1); } }
          *l = pivot;
        }
      start = start.add (1);
    }
}


/*  Find and return the length of the "run" (the longest
    non-decreasing sequence or the longest strictly decreasing
    sequence, with the Boolean *DESCENDING set to 0 in the former
    case, or to 1 in the latter) beginning at LO, in the slice [LO,
    HI) with LO < HI.  The strictness of the definition of
    "descending" ensures there are no equal elements to get out of
    order so the caller can safely reverse a descending sequence
    without violating stability.  */

unsafe fn count_run (ms: *mut merge_state, mut lo: *mut Lisp_Object, hi: *const Lisp_Object,
                     descending: *mut bool) -> ptrdiff_t
{
  eassume! (lo < hi as *mut Lisp_Object);
  *descending = false;
  lo = lo.add (1);
  let mut n: ptrdiff_t = 1;
  if lo == hi as *mut Lisp_Object
    { return n; }

  n = 2;
  if inorder (ms, *lo, *lo.sub (1))
    {
      *descending = true;
      lo = lo.add (1);
      while lo < hi as *mut Lisp_Object
        {
          if !inorder (ms, *lo, *lo.sub (1))
            { break; }
          lo = lo.add (1); n += 1;
        }
    }
  else
    {
      lo = lo.add (1);
      while lo < hi as *mut Lisp_Object
        {
          if inorder (ms, *lo, *lo.sub (1))
            { break; }
          lo = lo.add (1); n += 1;
        }
    }

  return n;
}


/*  Locate and return the proper insertion position of KEY in a sorted
    vector: if the vector contains an element equal to KEY, return the
    position immediately to the left of the leftmost equal element.
    [GALLOP_RIGHT does the same except it returns the position to the
    right of the rightmost equal element (if any).]

    'A' is a sorted vector of N elements. N must be > 0.

    Elements preceding HINT, a non-negative index less than N, are
    skipped.  The closer HINT is to the final result, the faster this
    runs.

    The return value is the int k in [0, N] such that

    A[k-1] < KEY <= a[k]

    pretending that *(A-1) precedes all values and *(A+N) succeeds all
    values.  In other words, the first k elements of A should precede
    KEY, and the last N-k should follow KEY.  */

unsafe fn gallop_left (ms: *mut merge_state, key: Lisp_Object, mut a: *mut Lisp_Object,
                       n: ptrdiff_t, hint: ptrdiff_t) -> ptrdiff_t
{
  eassume! (!a.is_null () && n > 0 && hint >= 0 && hint < n);

  a = a.offset (hint);
  let mut lastofs: ptrdiff_t = 0;
  let mut ofs: ptrdiff_t = 1;
  if inorder (ms, *a, key)
    {
      /* When a[hint] < key, gallop right until
	 a[hint + lastofs] < key <= a[hint + ofs].  */
      let maxofs: ptrdiff_t = n - hint;	/* This is one after the end of a.  */
      while ofs < maxofs
	{
	  if inorder (ms, *a.offset (ofs), key)
	    {
	      lastofs = ofs;
	      eassume! (ofs <= (PTRDIFF_MAX - 1) / 2);
	      ofs = (ofs << 1) + 1;
	    }
	  else
	    { break; }	/* Here key <= a[hint+ofs].  */
	}
      if ofs > maxofs
	{ ofs = maxofs; }
      /* Translate back to offsets relative to &a[0].  */
      lastofs += hint;
      ofs += hint;
    }
  else
    {
      /* When key <= a[hint], gallop left, until
	 a[hint - ofs] < key <= a[hint - lastofs].  */
      let maxofs: ptrdiff_t = hint + 1;	/* Here &a[0] is lowest.  */
      while ofs < maxofs
	{
	  if inorder (ms, *a.offset (-ofs), key)
	    { break; }
	  /* Here key <= a[hint - ofs].  */
	  lastofs = ofs;
	  eassume! (ofs <= (PTRDIFF_MAX - 1) / 2);
	  ofs = (ofs << 1) + 1;
	}
      if ofs > maxofs
	{ ofs = maxofs; }
      /* Translate back to use positive offsets relative to &a[0].  */
      let k: ptrdiff_t = lastofs;
      lastofs = hint - ofs;
      ofs = hint - k;
    }
  a = a.offset (-hint);

  eassume! (-1 <= lastofs && lastofs < ofs && ofs <= n);
  /* Now a[lastofs] < key <= a[ofs], so key belongs somewhere to the
     right of lastofs but no farther right than ofs.  Do a binary
     search, with invariant a[lastofs-1] < key <= a[ofs].  */
  lastofs += 1;
  while lastofs < ofs
    {
      let m: ptrdiff_t = lastofs + ((ofs - lastofs) >> 1);

      if inorder (ms, *a.offset (m), key)
	{ lastofs = m + 1; }		/* Here a[m] < key.  */
      else
	{ ofs = m; }			/* Here key <= a[m].  */
    }
  eassume! (lastofs == ofs);		/* Then a[ofs-1] < key <= a[ofs].  */
  return ofs;
}


/*  Locate and return the proper position of KEY in a sorted vector
    exactly like GALLOP_LEFT, except that if KEY already exists in
    A[0:N] find the position immediately to the right of the rightmost
    equal value.

    The return value is the int k in [0, N] such that

    A[k-1] <= KEY < A[k].  */

unsafe fn gallop_right (ms: *mut merge_state, key: Lisp_Object, mut a: *mut Lisp_Object,
                        n: ptrdiff_t, hint: ptrdiff_t) -> ptrdiff_t
{
  eassume! (!a.is_null () && n > 0 && hint >= 0 && hint < n);

  a = a.offset (hint);
  let mut lastofs: ptrdiff_t = 0;
  let mut ofs: ptrdiff_t = 1;
  if inorder (ms, key, *a)
    {
      /* When key < a[hint], gallop left until
	 a[hint - ofs] <= key < a[hint - lastofs].  */
      let maxofs: ptrdiff_t = hint + 1;	/* Here &a[0] is lowest.  */
      while ofs < maxofs
	{
	  if inorder (ms, key, *a.offset (-ofs))
	    {
	      lastofs = ofs;
	      eassume! (ofs <= (PTRDIFF_MAX - 1) / 2);
	      ofs = (ofs << 1) + 1;
	    }
	  else			/* Here a[hint - ofs] <= key.  */
	    { break; }
	}
      if ofs > maxofs
	{ ofs = maxofs; }
      /* Translate back to use positive offsets relative to &a[0].  */
      let k: ptrdiff_t = lastofs;
      lastofs = hint - ofs;
      ofs = hint - k;
    }
  else
    {
      /* When a[hint] <= key, gallop right, until
	 a[hint + lastofs] <= key < a[hint + ofs].  */
      let maxofs: ptrdiff_t = n - hint;	/* Here &a[n-1] is highest.  */
      while ofs < maxofs
	{
	  if inorder (ms, key, *a.offset (ofs))
	    { break; }
	  /* Here a[hint + ofs] <= key.  */
	  lastofs = ofs;
	  eassume! (ofs <= (PTRDIFF_MAX - 1) / 2);
	  ofs = (ofs << 1) + 1;
	}
      if ofs > maxofs
	{ ofs = maxofs; }
      /* Translate back to use offsets relative to &a[0].  */
      lastofs += hint;
      ofs += hint;
    }
  a = a.offset (-hint);

  eassume! (-1 <= lastofs && lastofs < ofs && ofs <= n);
  /* Now a[lastofs] <= key < a[ofs], so key belongs somewhere to the
     right of lastofs but no farther right than ofs.  Do a binary
     search, with invariant a[lastofs-1] <= key < a[ofs].  */
  lastofs += 1;
  while lastofs < ofs
    {
      let m: ptrdiff_t = lastofs + ((ofs - lastofs) >> 1);

      if inorder (ms, key, *a.offset (m))
	{ ofs = m; }			/* Here key < a[m].  */
      else
	{ lastofs = m + 1; }		/* Here a[m] <= key.  */
    }
  eassume! (lastofs == ofs);		/* Now  a[ofs-1] <= key < a[ofs].  */
  return ofs;
}


unsafe fn merge_init (ms: *mut merge_state, list_size: ptrdiff_t,
                      allocated_keys: *mut Lisp_Object, lo: *mut sortslice,
                      predicate: Lisp_Object)
{
  eassume! (!ms.is_null ());

  if !(*lo).values.is_null ()
    {
      /* The temporary space for merging will need at most half the list
	 size rounded up.  Use the minimum possible space so we can use the
	 rest of temparray for other things.  In particular, if there is
	 enough extra space, if will be used to store the keys.  */
      (*ms).alloced = (list_size + 1) / 2;

      /* ms->alloced describes how many keys will be stored at
	 ms->temparray, but we also need to store the values.  Hence,
	 ms->alloced is capped at half of MERGESTATE_TEMP_SIZE.  */
      if ((MERGESTATE_TEMP_SIZE / 2) as ptrdiff_t) < (*ms).alloced
	{ (*ms).alloced = (MERGESTATE_TEMP_SIZE / 2) as ptrdiff_t; }
      (*ms).a.values = (*ms).temparray.as_mut_ptr ().offset ((*ms).alloced);
    }
  else
    {
      (*ms).alloced = MERGESTATE_TEMP_SIZE as ptrdiff_t;
      (*ms).a.values = ptr::null_mut ();
    }
  (*ms).a.keys = (*ms).temparray.as_mut_ptr ();

  (*ms).n = 0;
  (*ms).min_gallop = GALLOP_WIN_MIN;
  (*ms).listlen = list_size;
  (*ms).basekeys = (*lo).keys;
  (*ms).allocated_keys = allocated_keys;
  (*ms).pred_fun = if NILP (predicate)
    { Some (order_pred_valuelt) }
  else if SUBRP (predicate) && (*XSUBR (predicate)).function.aMANY == Some (Flss)
    { Some (order_pred_lss) }
  else if SUBRP (predicate) && (*XSUBR (predicate)).function.aMANY == Some (Fgtr)
    { Some (order_pred_gtr) }
  else
    { Some (order_pred_lisp) };
  (*ms).predicate = predicate;
  (*ms).reloc = reloc { src: ptr::null_mut (), dst: ptr::null_mut (),
			size: ptr::null_mut (), order: 0 };
  (*ms).count = make_invalid_specpdl_ref ();
  if !allocated_keys.is_null ()
    { merge_register_cleanup (ms); }
}


/* The dynamically allocated memory may hold lisp objects during
   merging.  MERGE_MARKMEM marks them so they aren't reaped during
   GC.  */

unsafe extern "C-unwind" fn merge_markmem (arg: *mut c_void)
{
  let ms: *mut merge_state = arg as *mut merge_state;
  eassume! (!ms.is_null ());

  if !(*ms).allocated_keys.is_null ()
    { mark_objects ((*ms).allocated_keys, (*ms).listlen); }

  if !(*ms).reloc.size.is_null () && *(*ms).reloc.size > 0
    {
      let src = if !(*(*ms).reloc.src).values.is_null ()
		  { (*(*ms).reloc.src).values } else { (*(*ms).reloc.src).keys };
      eassume! (!src.is_null ());
      mark_objects (src, *(*ms).reloc.size);
    }
}


/* Free all temp storage.  If an exception occurs while merging,
   relocate any lisp elements in temp storage back to the original
   array.  */

unsafe extern "C-unwind" fn cleanup_mem (arg: *mut c_void)
{
  let ms: *mut merge_state = arg as *mut merge_state;
  eassume! (!ms.is_null ());

  /* If we have an exception while merging, some of the list elements
     might only live in temp storage; we copy everything remaining in
     the temp storage back into the original list.  This ensures that
     the original list has all of the original elements, although
     their order is unpredictable.  */

  if (*ms).reloc.order != 0 && *(*ms).reloc.size > 0
    {
      let src = if !(*(*ms).reloc.src).values.is_null ()
		  { (*(*ms).reloc.src).values } else { (*(*ms).reloc.src).keys };
      let dst = if !(*(*ms).reloc.dst).values.is_null ()
		  { (*(*ms).reloc.dst).values } else { (*(*ms).reloc.dst).keys };
      eassume! (!src.is_null () && !dst.is_null ());
      let n: ptrdiff_t = *(*ms).reloc.size;
      let shift: ptrdiff_t = if (*ms).reloc.order == -1 { 0 } else { n - 1 };
      ptr::copy_nonoverlapping (src, dst.offset (-shift), n as usize);
    }

  /* Free any remaining temp storage.  */
  if (*ms).a.keys != (*ms).temparray.as_mut_ptr ()
    {
      xfree ((*ms).a.keys as *mut c_void);
      (*ms).a.keys = ptr::null_mut ();
    }

  if !(*ms).allocated_keys.is_null ()
    {
      xfree ((*ms).allocated_keys as *mut c_void);
      (*ms).allocated_keys = ptr::null_mut ();
    }
}

unsafe fn merge_register_cleanup (ms: *mut merge_state)
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_ptr_mark (Some (cleanup_mem), ms as *mut c_void, Some (merge_markmem));
  (*ms).count = count;
}

/* Allocate enough temp memory for NEED array slots.  Any previously
   allocated memory is first freed, and a cleanup routine is
   registered to free memory at the very end of the sort, or on
   exception.  */

unsafe fn merge_getmem (ms: *mut merge_state, need: ptrdiff_t)
{
  eassume! (!ms.is_null ());

  if (*ms).a.keys == (*ms).temparray.as_mut_ptr ()
    {
      /* We only get here if alloc is needed and this is the first
	 time, so we set up the unwind protection.  */
      if !specpdl_ref_valid_p ((*ms).count)
	{ merge_register_cleanup (ms); }
    }
  else
    {
      /* We have previously alloced storage.  Since we don't care
         what's in the block we don't use realloc which would waste
         cycles copying the old data.  We just free and alloc
         again.  */
      xfree ((*ms).a.keys as *mut c_void);
    }
  let bytes: ptrdiff_t = (need * word_size) << if !(*ms).a.values.is_null () { 1 } else { 0 };
  (*ms).a.keys = xmalloc (bytes as usize) as *mut Lisp_Object;
  (*ms).alloced = need;
  if !(*ms).a.values.is_null ()
    { (*ms).a.values = (*ms).a.keys.offset (need); }
}


#[inline]
unsafe fn needmem (ms: *mut merge_state, na: ptrdiff_t)
{
  if na > (*ms).alloced
    { merge_getmem (ms, na); }
}


/* Stably merge (in-place) the NA elements starting at SSA with the NB
   elements starting at SSB = SSA + NA.  NA and NB must be positive.
   Require that SSA[NA-1] belongs at the end of the merge, and NA <=
   NB.  */

unsafe fn merge_lo (ms: *mut merge_state, mut ssa: sortslice, mut na: ptrdiff_t,
		    mut ssb: sortslice, mut nb: ptrdiff_t)
{
  eassume! (!ms.is_null () && !ssa.keys.is_null () && !ssb.keys.is_null () && na > 0 && nb > 0);
  eassume! (ssa.keys.offset (na) == ssb.keys);
  needmem (ms, na);
  sortslice_memcpy (&raw mut (*ms).a, 0, &raw mut ssa, 0, na);
  let mut dest = ssa;
  ssa = (*ms).a;

  (*ms).reloc = reloc { src: &raw mut ssa, dst: &raw mut dest, size: &raw mut na, order: -1 };

  sortslice_copy_incr (&raw mut dest, &raw mut ssb);
  nb -= 1;

  /* The C body reaches the end of the merge through two forward gotos; the port
     keeps Succeed and CopyB as the labels of one state machine, so no tail is
     skipped (PORTING.md 2.2).  */
  enum Label { Succeed, CopyB }
  let label = 'flow: {
      if nb == 0
	{ break 'flow Label::Succeed; }
      if na == 1
	{ break 'flow Label::CopyB; }

      let mut min_gallop = (*ms).min_gallop;
      loop
	{
	  let mut acount: ptrdiff_t = 0;   /* The # of consecutive times A won.  */

	  let mut bcount: ptrdiff_t = 0;   /* The # of consecutive times B won.  */

	  loop
	    {
	      eassume! (na > 1 && nb > 0);
	      if inorder (ms, *ssb.keys, *ssa.keys)
		{
		  sortslice_copy_incr (&raw mut dest, &raw mut ssb);
		  bcount += 1;
		  acount = 0;
		  nb -= 1;
		  if nb == 0
		    { break 'flow Label::Succeed; }
		  if bcount >= min_gallop
		    { break; }
		}
	      else
		{
		  sortslice_copy_incr (&raw mut dest, &raw mut ssa);
		  acount += 1;
		  bcount = 0;
		  na -= 1;
		  if na == 1
		    { break 'flow Label::CopyB; }
		  if acount >= min_gallop
		    { break; }
		}
	    }

	  /* One run is winning so consistently that galloping may be a
	     huge speedup.  We try that, and continue galloping until (if
	     ever) neither run appears to be winning consistently
	     anymore.  */
	  min_gallop += 1;
	  loop {
	    eassume! (na > 1 && nb > 0);
	    min_gallop -= (min_gallop > 1) as ptrdiff_t;
	    (*ms).min_gallop = min_gallop;
	    let mut k = gallop_right (ms, *ssb.keys, ssa.keys, na, 0);
	    acount = k;
	    if k != 0
	      {
		sortslice_memcpy (&raw mut dest, 0, &raw mut ssa, 0, k);
		sortslice_advance (&raw mut dest, k);
		sortslice_advance (&raw mut ssa, k);
		na -= k;
		if na == 1
		  { break 'flow Label::CopyB; }
		/* While na==0 is impossible for a consistent comparison
		   function, we shouldn't assume that it is.  */
		if na == 0
		  { break 'flow Label::Succeed; }
	      }
	    sortslice_copy_incr (&raw mut dest, &raw mut ssb);
	    nb -= 1;
	    if nb == 0
	      { break 'flow Label::Succeed; }

	    k = gallop_left (ms, *ssa.keys, ssb.keys, nb, 0);
	    bcount = k;
	    if k != 0
	      {
		sortslice_memmove (&raw mut dest, 0, &raw mut ssb, 0, k);
		sortslice_advance (&raw mut dest, k);
		sortslice_advance (&raw mut ssb, k);
		nb -= k;
		if nb == 0
		  { break 'flow Label::Succeed; }
	      }
	    sortslice_copy_incr (&raw mut dest, &raw mut ssa);
	    na -= 1;
	    if na == 1
	      { break 'flow Label::CopyB; }
	    if !(acount >= GALLOP_WIN_MIN || bcount >= GALLOP_WIN_MIN)
	      { break; }
	  }
	  min_gallop += 1;   /* Apply a penalty for leaving galloping mode.  */
	  (*ms).min_gallop = min_gallop;
	}
  };
  if let Label::Succeed = label
    {
      (*ms).reloc = reloc { src: ptr::null_mut (), dst: ptr::null_mut (),
			    size: ptr::null_mut (), order: 0 };

      if na != 0
	{ sortslice_memcpy (&raw mut dest, 0, &raw mut ssa, 0, na); }
    }
  else
    {
      eassume! (na == 1 && nb > 0);
      (*ms).reloc = reloc { src: ptr::null_mut (), dst: ptr::null_mut (),
			    size: ptr::null_mut (), order: 0 };

      /* The last element of ssa belongs at the end of the merge.  */
      sortslice_memmove (&raw mut dest, 0, &raw mut ssb, 0, nb);
      sortslice_copy (&raw mut dest, nb, &raw mut ssa, 0);
    }
}


/* Stably merge (in-place) the NA elements starting at SSA with the NB
   elements starting at SSB = SSA + NA.  NA and NB must be positive.
   Require that SSA[NA-1] belongs at the end of the merge, and NA >=
   NB.  */

unsafe fn merge_hi (ms: *mut merge_state, mut ssa: sortslice, mut na: ptrdiff_t,
		    mut ssb: sortslice, mut nb: ptrdiff_t)
{
  eassume! (!ms.is_null () && !ssa.keys.is_null () && !ssb.keys.is_null () && na > 0 && nb > 0);
  eassume! (ssa.keys.offset (na) == ssb.keys);
  needmem (ms, nb);
  let mut dest = ssb;
  sortslice_advance (&raw mut dest, nb-1);
  sortslice_memcpy (&raw mut (*ms).a, 0, &raw mut ssb, 0, nb);
  let basea = ssa;
  let mut baseb = (*ms).a;
  ssb.keys = (*ms).a.keys.offset (nb - 1);
  if !ssb.values.is_null ()
    { ssb.values = (*ms).a.values.offset (nb - 1); }
  sortslice_advance (&raw mut ssa, na - 1);

  (*ms).reloc = reloc { src: &raw mut baseb, dst: &raw mut dest, size: &raw mut nb, order: 1 };

  sortslice_copy_decr (&raw mut dest, &raw mut ssa);
  na -= 1;

  /* The C body reaches the end of the merge through two forward gotos; the port
     keeps Succeed and CopyA as the labels of one state machine (PORTING.md 2.2).  */
  enum Label { Succeed, CopyA }
  let label = 'flow: {
      if na == 0
	{ break 'flow Label::Succeed; }
      if nb == 1
	{ break 'flow Label::CopyA; }

      let mut min_gallop = (*ms).min_gallop;
      loop {
	let mut acount: ptrdiff_t = 0;   /* The # of consecutive times A won.  */
	let mut bcount: ptrdiff_t = 0;   /* The # of consecutive times B won.  */

	loop {
	  eassume! (na > 0 && nb > 1);
	  if inorder (ms, *ssb.keys, *ssa.keys)
	    {
	      sortslice_copy_decr (&raw mut dest, &raw mut ssa);
	      acount += 1;
	      bcount = 0;
	      na -= 1;
	      if na == 0
		{ break 'flow Label::Succeed; }
	      if acount >= min_gallop
		{ break; }
	    }
	  else
	    {
	      sortslice_copy_decr (&raw mut dest, &raw mut ssb);
	      bcount += 1;
	      acount = 0;
	      nb -= 1;
	      if nb == 1
		{ break 'flow Label::CopyA; }
	      if bcount >= min_gallop
		{ break; }
	    }
	}

	/* One run is winning so consistently that galloping may be a huge
	   speedup.  Try that, and continue galloping until (if ever)
	   neither run appears to be winning consistently anymore.  */
	min_gallop += 1;
	loop {
	  eassume! (na > 0 && nb > 1);
	  min_gallop -= (min_gallop > 1) as ptrdiff_t;
	  (*ms).min_gallop = min_gallop;
	  let mut k = gallop_right (ms, *ssb.keys, basea.keys, na, na - 1);
	  k = na - k;
	  acount = k;
	  if k != 0
	    {
	      sortslice_advance (&raw mut dest, -k);
	      sortslice_advance (&raw mut ssa, -k);
	      sortslice_memmove (&raw mut dest, 1, &raw mut ssa, 1, k);
	      na -= k;
	      if na == 0
		{ break 'flow Label::Succeed; }
	    }
	  sortslice_copy_decr (&raw mut dest, &raw mut ssb);
	  nb -= 1;
	  if nb == 1
	    { break 'flow Label::CopyA; }

	  k = gallop_left (ms, *ssa.keys, baseb.keys, nb, nb - 1);
	  k = nb - k;
	  bcount = k;
	  if k != 0
	    {
	      sortslice_advance (&raw mut dest, -k);
	      sortslice_advance (&raw mut ssb, -k);
	      sortslice_memcpy (&raw mut dest, 1, &raw mut ssb, 1, k);
	      nb -= k;
	      if nb == 1
		{ break 'flow Label::CopyA; }
	      /* While nb==0 is impossible for a consistent comparison
		  function we shouldn't assume that it is.  */
	      if nb == 0
		{ break 'flow Label::Succeed; }
	    }
	  sortslice_copy_decr (&raw mut dest, &raw mut ssa);
	  na -= 1;
	  if na == 0
	    { break 'flow Label::Succeed; }
	  if !(acount >= GALLOP_WIN_MIN || bcount >= GALLOP_WIN_MIN)
	    { break; }
	}
	min_gallop += 1;      /* Apply a penalty for leaving galloping mode.  */
	(*ms).min_gallop = min_gallop;
      }
  };
  if let Label::Succeed = label
    {
      (*ms).reloc = reloc { src: ptr::null_mut (), dst: ptr::null_mut (),
			    size: ptr::null_mut (), order: 0 };
      if nb != 0
	{ sortslice_memcpy (&raw mut dest, -(nb-1), &raw mut baseb, 0, nb); }
    }
  else
    {
      eassume! (nb == 1 && na > 0);
      (*ms).reloc = reloc { src: ptr::null_mut (), dst: ptr::null_mut (),
			    size: ptr::null_mut (), order: 0 };
      /* The first element of ssb belongs at the front of the merge.  */
      sortslice_memmove (&raw mut dest, 1-na, &raw mut ssa, 1-na, na);
      sortslice_advance (&raw mut dest, -na);
      sortslice_advance (&raw mut ssa, -na);
      sortslice_copy (&raw mut dest, 0, &raw mut ssb, 0);
    }
}


/* Merge the two runs at stack indices I and I+1.  */

unsafe fn merge_at (ms: *mut merge_state, i: ptrdiff_t)
{
  eassume! (!ms.is_null ());
  eassume! ((*ms).n >= 2);
  eassume! (i >= 0);
  eassume! (i == (*ms).n as ptrdiff_t - 2 || i == (*ms).n as ptrdiff_t - 3);

  let mut ssa = (*ms).pending[i as usize].base;
  let mut na = (*ms).pending[i as usize].len;
  let ssb = (*ms).pending[i as usize + 1].base;
  let mut nb = (*ms).pending[i as usize + 1].len;
  eassume! (na > 0 && nb > 0);
  eassume! (ssa.keys.offset (na) == ssb.keys);

  /* Record the length of the combined runs. The current run i+1 goes
     away after the merge.  If i is the 3rd-last run now, slide the
     last run (which isn't involved in this merge) over to i+1.  */
  (*ms).pending[i as usize].len = na + nb;
  if i == (*ms).n as ptrdiff_t - 3
    { (*ms).pending[i as usize + 1] = (*ms).pending[i as usize + 2]; }
  (*ms).n -= 1;

  /* Where does b start in a?  Elements in a before that can be
     ignored (they are already in place).  */
  let k = gallop_right (ms, *ssb.keys, ssa.keys, na, 0);
  eassume! (k >= 0);
  sortslice_advance (&raw mut ssa, k);
  na -= k;
  if na == 0
    { return; }

  /* Where does a end in b?  Elements in b after that can be ignored
     (they are already in place).  */
  nb = gallop_left (ms, *ssa.keys.offset (na - 1), ssb.keys, nb, nb - 1);
  if nb == 0
    { return; }
  eassume! (nb > 0);
  /* Merge what remains of the runs using a temp array with size
     min(na, nb) elements.  */
  if na <= nb
    { merge_lo (ms, ssa, na, ssb, nb); }
  else
    { merge_hi (ms, ssa, na, ssb, nb); }
}


/* Compute the "power" of the first of two adjacent runs beginning at
   index S1, with the first having length N1 and the second (starting
   at index S1+N1) having length N2.  The run has total length N.  */

unsafe fn powerloop (s1: ptrdiff_t, n1: ptrdiff_t, n2: ptrdiff_t, n: ptrdiff_t) -> c_int
{
  eassume! (s1 >= 0);
  eassume! (n1 > 0 && n2 > 0);
  eassume! (s1 + n1 + n2 <= n);
  /* The midpoints a and b are
     a = s1 + n1/2
     b = s1 + n1 + n2/2 = a + (n1 + n2)/2

     These may not be integers because of the "/2", so we work with
     2*a and 2*b instead.  It makes no difference to the outcome,
     since the bits in the expansion of (2*i)/n are merely shifted one
     position from those of i/n.  */
  let mut a: ptrdiff_t = 2 * s1 + n1;
  let mut b: ptrdiff_t = a + n1 + n2;
  let mut result: c_int = 0;
  /* Emulate a/n and b/n one bit a time, until their bits differ.  */
  loop
    {
      result += 1;
      if a >= n
	{  /* Both quotient bits are now 1.  */
	  eassume! (b >= a);
	  a -= n;
	  b -= n;
	}
      else if b >= n
	{  /* a/n bit is 0 and b/n bit is 1.  */
	  break;
	} /* Otherwise both quotient bits are 0.  */
      eassume! (a < b && b < n);
      a <<= 1;
      b <<= 1;
    }
  return result;
}


/* Update the state upon identifying a run of length N2.  If there's
   already a stretch on the stack, apply the "powersort" merge
   strategy: compute the topmost stretch's "power" (depth in a
   conceptual binary merge tree) and merge adjacent runs on the stack
   with greater power.  */

unsafe fn found_new_run (ms: *mut merge_state, n2: ptrdiff_t)
{
  eassume! (!ms.is_null ());
  if (*ms).n != 0
    {
      eassume! ((*ms).n > 0);
      let p = (*ms).pending.as_mut_ptr ();
      let s1 = (*p.offset (((*ms).n - 1) as ptrdiff_t)).base.keys.offset_from ((*ms).basekeys);
      let n1 = (*p.offset (((*ms).n - 1) as ptrdiff_t)).len;
      let power = powerloop (s1, n1, n2, (*ms).listlen);
      while (*ms).n > 1 && (*p.offset (((*ms).n - 2) as ptrdiff_t)).power > power
	{
	  merge_at (ms, (*ms).n as ptrdiff_t - 2);
	}
      eassume! ((*ms).n < 2 || (*p.offset (((*ms).n - 2) as ptrdiff_t)).power < power);
      (*p.offset (((*ms).n - 1) as ptrdiff_t)).power = power;
    }
}


/* Unconditionally merge all stretches on the stack until only one
   remains.  */

unsafe fn merge_force_collapse (ms: *mut merge_state)
{
  let p = (*ms).pending.as_mut_ptr ();

  eassume! (!ms.is_null ());
  while (*ms).n > 1
    {
      let mut n = (*ms).n as ptrdiff_t - 2;
      if n > 0 && (*p.offset (n - 1)).len < (*p.offset (n + 1)).len
	{ n -= 1; }
      merge_at (ms, n);
    }
}


/* Compute a good value for the minimum run length; natural runs
   shorter than this are boosted artificially via binary insertion.

   If N < 64, return N (it's too small to bother with fancy stuff).
   Otherwise if N is an exact power of 2, return 32.  Finally, return
   an int k, 32 <= k <= 64, such that N/k is close to, but strictly
   less than, an exact power of 2.  */

unsafe fn merge_compute_minrun (mut n: ptrdiff_t) -> ptrdiff_t
{
  let mut r: ptrdiff_t = 0;           /* r will become 1 if any non-zero bits are
				shifted off.  */

  eassume! (n >= 0);
  while n >= 64
    {
      r |= n & 1;
      n >>= 1;
    }
  return n + r;
}


unsafe fn reverse_sortslice (s: *mut sortslice, n: ptrdiff_t)
{
  reverse_slice ((*s).keys, (*s).keys.offset (n));
  if !(*s).values.is_null ()
    { reverse_slice ((*s).values, (*s).values.offset (n)); }
}

unsafe fn resolve_fun (mut fun: Lisp_Object) -> Lisp_Object
{
  if SYMBOLP (fun)
    {
      /* Attempt to resolve the function as far as possible ahead of time,
	 to avoid having to do it for each call.  */
      let mut f = (*XSYMBOL (fun)).u.s.function;
      if SYMBOLP (f)
	/* Function was an alias; use slow-path resolution.  */
	{ f = indirect_function (f); }
      /* Don't resolve to an autoload spec; that would be very slow.  */
      if !NILP (f) && !(CONSP (f) && EQ (XCAR (f), Qautoload ()))
	{ fun = f; }
    }
  return fun;
}

/* Sort the array SEQ with LENGTH elements in the order determined by
   PREDICATE (where Qnil means value<) and KEYFUNC (where Qnil means identity),
   optionally reversed.  */
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn tim_sort (mut predicate: Lisp_Object, mut keyfunc: Lisp_Object,
				   seq: *mut Lisp_Object, length: ptrdiff_t, reverse: bool)
{
  /* FIXME: hoist this to the caller? */
  if EQ (predicate, Qvaluelt ())
    { predicate = Qnil (); }
  if !NILP (predicate)
    { predicate = resolve_fun (predicate); }
  if EQ (keyfunc, Qidentity ())
    { keyfunc = Qnil (); }

  /* Rust has no partial initialization (E0381): C's `sortslice lo;` becomes the
     same zero value C's own compound-literal idiom spells (PORTING.md 0.3).  */
  let mut lo = sortslice { keys: ptr::null_mut (), values: ptr::null_mut () };
  let mut keys: *mut Lisp_Object;
  let mut allocated_keys: *mut Lisp_Object = ptr::null_mut ();
  let mut ms = MaybeUninit::<merge_state>::uninit ();

  if reverse && 0 < length
    { reverse_slice (seq, seq.offset (length)); }    /* preserve stability */

  if NILP (keyfunc)
    {
      keys = ptr::null_mut ();
      lo.keys = seq;
      lo.values = ptr::null_mut ();
    }
  else
    {
      keyfunc = resolve_fun (keyfunc);
      if length < (MERGESTATE_TEMP_SIZE / 2) as ptrdiff_t
	{ keys = (*ms.as_mut_ptr ()).temparray.as_mut_ptr ().offset (length + 1); }
      else
	{
	  /* Fill with valid Lisp values in case a GC occurs before all
	     keys have been computed.  */
	  static_assert! (NIL_IS_ZERO);
	  keys = xzalloc (length as usize * word_size as usize) as *mut Lisp_Object;
	  allocated_keys = keys;
	}

      lo.keys = keys;
      lo.values = seq;
    }

  let ms: *mut merge_state = ms.as_mut_ptr ();
  merge_init (ms, length, allocated_keys, &raw mut lo, predicate);

  /* Compute keys after merge_markmem has been registered by merge_init
     (any call to keyfunc might trigger a GC).  */
  if !NILP (keyfunc)
    {
      let mut i: ptrdiff_t = 0;
      while i < length
	{
	  *keys.offset (i) = calln! (keyfunc, *seq.offset (i));
	  i += 1;
	}
    }

  /* FIXME: This is where we would check the keys for interesting
     properties for more optimized comparison (such as all being fixnums
     etc).  */

  /* March over the array once, left to right, finding natural runs,
     and extending short natural runs to minrun elements.  */
  let minrun = merge_compute_minrun (length);
  let mut nremaining = length;
  loop {
    let mut descending: bool = false;

    /* Identify the next run.  */
    let mut n = count_run (ms, lo.keys, (lo.keys as *const Lisp_Object).offset (nremaining),
			   &raw mut descending);
    if descending
      { reverse_sortslice (&raw mut lo, n); }
    /* If the run is short, extend it to min(minrun, nremaining).  */
    if n < minrun
      {
	let force = min (nremaining, minrun);
	binarysort (ms, lo, (lo.keys as *const Lisp_Object).offset (force), lo.keys.offset (n));
	n = force;
      }
    eassume! ((*ms).n == 0
	      || ((*ms).pending[((*ms).n - 1) as usize].base.keys.offset ((*ms).pending[((*ms).n - 1) as usize].len)
		  == lo.keys));
    found_new_run (ms, n);
    /* Push the new run on to the stack.  */
    eassume! (((*ms).n as usize) < MAX_MERGE_PENDING);
    (*ms).pending[(*ms).n as usize].base = lo;
    (*ms).pending[(*ms).n as usize].len = n;
    (*ms).n += 1;
    /* Advance to find the next run.  */
    sortslice_advance (&raw mut lo, n);
    nremaining -= n;
    if nremaining == 0
      { break; }
  }

  merge_force_collapse (ms);
  eassume! ((*ms).n == 1);
  eassume! ((*ms).pending[0].len == length);
  lo = (*ms).pending[0].base;

  if reverse
    { reverse_slice (seq, seq.offset (length)); }

  if (*ms).a.keys != (*ms).temparray.as_mut_ptr () || !allocated_keys.is_null ()
    { unbind_to ((*ms).count, Qnil ()); }
}
