// femacs: port of src/marker.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  The eight
// subrs use `DEFUN!` (PORTING.md 5.1).
//
// The file's two `CONSIDER` statement macros (C 100-136, C 277-315) both end in an early
// `return` and deliberately do not always evaluate their second operand - C 106-107: "it is
// desirable that BYTEPOS is not evaluated except when we really want its value".  Rust's
// `macro_rules!` cannot name the caller's locals (hygiene: a macro's identifiers resolve at
// its definition site), and a plain function would evaluate both operands eagerly, so the
// port keeps each macro's statement sequence in a helper function that takes the lazy
// operand as an `FnOnce` and returns the value the macro would have returned (`Option`), and
// each use site returns that `Some`.  The macro's name is in
// docs/ports/excluded-macros.txt and the shape in docs/ports/marker.md.
//
// `#ifdef MARKER_DEBUG` (C 44-70) / `#else` (C 72-74): the frozen configuration leaves
// MARKER_DEBUG undefined, so C 74's no-op macro is the live text; the debug function and the
// two debug entry points C 831-859 declares are kept under `#[cfg (feature = "marker_debug")]`
// (the P-51 shape), because their bodies need buffer.h accessors this target's mirror has not
// got yet and the dead-arms gate has no key to flip for them (docs/ports/marker.md).
//
// `#if HAVE_TEXT_CONVERSION` (C 580-602) is undefined in the frozen configuration and its
// body needs `bset_redisplay`, which this target's mirror has not got; the arm is recorded in
// docs/ports/marker.md and in the ledger rather than written out (the rawmemchr.c CHERI-arm
// class).

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
// [U1] include/window.rs imported via common

/* C 22-26's `#if __GNUC__ == 13 && __GNUC_MINOR__ < 3 / #pragma GCC diagnostic ignored
   "-Wanalyzer-deref-before-check"` is a GCC workaround with no Rust counterpart, and the
   frozen target is clang, so the arm is dead text (the ledger records it).  */

/* Record one cached position found recently by
   buf_charpos_to_bytepos or buf_bytepos_to_charpos.  */

static cached_charpos: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);
static cached_bytepos: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);
static cached_buffer: VmGlobal<*mut buffer> = VmGlobal::<*mut buffer>::new (std::ptr::null_mut ());
static cached_modiff: VmGlobal<modiff_count> = VmGlobal::<modiff_count>::new (0);

/* Juanma Barranquero <lekktu@gmail.com> reported ~3x increased
   bootstrap time when byte_char_debug_check is enabled; so this
   is never turned on by --enable-checking configure option.  */

/* C 44-70's `#ifdef MARKER_DEBUG` arm: dead in the frozen configuration.  The arm's text is
   kept as Rust, gated the way P-51 keeps a dead arm's text, because its body reads
   buffer.h accessors (BUF_BEG_ADDR, BUF_GAP_END_ADDR) and character.h's
   multibyte_chars_in_text, which this target's mirrors have not got.  */
#[cfg (feature = "marker_debug")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn count_markers (buf: *mut buffer) -> c_int
{
  let mut total: c_int = 0;
  let mut tail: *mut Lisp_Marker;

  tail = BUF_MARKERS! (buf);
  while !tail.is_null ()
    {
      total += 1;
      tail = (*tail).next;
    }

  return total;
}

/* For debugging -- recompute the bytepos corresponding
   to CHARPOS in the simplest, most reliable way.  */
#[cfg (feature = "marker_debug")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn verify_bytepos (charpos: ptrdiff_t) -> ptrdiff_t
{
  let mut below: ptrdiff_t = BEG as ptrdiff_t;
  let mut below_byte: ptrdiff_t = BEG_BYTE as ptrdiff_t;

  while below != charpos
    {
      below += 1;
      below_byte += buf_next_char_len ((current_buffer! ()), below_byte) as ptrdiff_t;
    }

  return below_byte;
}

#[cfg (feature = "marker_debug")]
unsafe fn byte_char_debug_check (b: *mut buffer, charpos: ptrdiff_t, bytepos: ptrdiff_t)
{
  let mut nchars: ptrdiff_t;

  if NILP (BVAR! (b, enable_multibyte_characters_))
    { return; }

  if bytepos > BUF_GPT_BYTE! (b)
    {
      nchars
	= multibyte_chars_in_text (BUF_BEG_ADDR (b),
				   (BUF_GPT_BYTE! (b) - BUF_BEG_BYTE (b)) as usize)
	+ multibyte_chars_in_text (BUF_GAP_END_ADDR (b),
				   (bytepos - BUF_GPT_BYTE! (b)) as usize);
    }
  else
    { nchars = multibyte_chars_in_text (BUF_BEG_ADDR (b),
					(bytepos - BUF_BEG_BYTE (b)) as usize); }

  if charpos - 1 != nchars
    { emacs_abort (); }
}

/* C 72-74's `#else` arm is the live text: with MARKER_DEBUG undefined, every
   `byte_char_debug_check (b, charpos, bytepos);` C writes is `do { } while (0)`, and the
   port's macro expands to nothing in the same statement position.  */
macro_rules! byte_char_debug_check {
    ($b:expr, $charpos:expr, $bytepos:expr) => { };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn clear_charpos_cache (b: *mut buffer)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  if (*cached_buffer.as_mut_ptr ()) == b
    { (*cached_buffer.as_mut_ptr ()) = 0 as *mut buffer; }

  }
}

/* Converting between character positions and byte positions.  */

/* There are several places in the buffer where we know
   the correspondence: BEG, BEGV, PT, GPT, ZV and Z,
   and everywhere there is a marker.  So we find the one of these places
   that is closest to the specified position, and scan from there.  */

/* This macro is a subroutine of buf_charpos_to_bytepos.
   Note that it is desirable that BYTEPOS is not evaluated
   except when we really want its value.  */

/* C 100-136's `CONSIDER (CHARPOS, BYTEPOS)`: the port's counterpart is this helper, which
   takes the operand C evaluates eagerly (`CHARPOS`, here `this_charpos`) and the one it
   evaluates only in the branches that need it (`BYTEPOS`, an `FnOnce`), reads and writes the
   four cursor values the macro captures through `&mut`, and returns the value the macro's
   two `return`s would have returned.  */
unsafe fn consider_charpos (b: *mut buffer, charpos: ptrdiff_t,
			    this_charpos: ptrdiff_t,
			    bytepos: impl FnOnce () -> ptrdiff_t,
			    best_above: &mut ptrdiff_t, best_above_byte: &mut ptrdiff_t,
			    best_below: &mut ptrdiff_t, best_below_byte: &mut ptrdiff_t)
  -> Option<ptrdiff_t>
{
  let mut changed: bool = false;

  if this_charpos == charpos
    {
      let value: ptrdiff_t = bytepos ();

      byte_char_debug_check! (b, charpos, value);
      return Some (value);
    }
  else if this_charpos > charpos
    {
      if this_charpos < *best_above
	{
	  *best_above = this_charpos;
	  *best_above_byte = bytepos ();
	  changed = true;
	}
    }
  else if this_charpos > *best_below
    {
      *best_below = this_charpos;
      *best_below_byte = bytepos ();
      changed = true;
    }

  if changed
    {
      if *best_above - *best_below == *best_above_byte - *best_below_byte
        {
	  let value: ptrdiff_t = *best_below_byte + (charpos - *best_below);

	  byte_char_debug_check! (b, charpos, value);
	  return Some (value);
	}
    }
  return None;
}

unsafe fn CHECK_MARKER (x: Lisp_Object)
{
  CHECK_TYPE (MARKERP (x), Qmarkerp (), x);
}

/* When converting bytes from/to chars, we look through the list of
   markers to try and find a good starting point (since markers keep
   track of both bytepos and charpos at the same time).
   But if there are many markers, it can take too much time to find a "good"
   marker from which to start.  Worse yet: if it takes a long time and we end
   up finding a nearby markers, we won't add a new marker to cache this
   result, so next time around we'll have to go through this same long list
   to (re)find this best marker.  So the further down the list of
   markers we go, the less demanding we are w.r.t what is a good marker.

   The previous code used INITIAL=50 and INCREMENT=0 and this lead to
   really poor performance when there are many markers.
   I haven't tried to tweak INITIAL, but experiments on my trusty Thinkpad
   T61 using various artificial test cases seem to suggest that INCREMENT=50
   might be "the best compromise": it significantly improved the
   worst case and it was rarely slower and never by much.

   The asymptotic behavior is still poor, tho, so in largish buffers with many
   overlays (e.g. 300KB and 30K overlays), it can still be a bottleneck.  */
const BYTECHAR_DISTANCE_INITIAL: c_int = 50;
const BYTECHAR_DISTANCE_INCREMENT: c_int = 50;

/* Return the byte position corresponding to CHARPOS in B.  */

unsafe extern "C" {
  // lisp.h 4963's `build_marker` is the include/lisp.rs mirror's now; the mirror spells
  // `struct buffer *` as `*mut c_void` (P-72 rule 3), so the four call sites below cast
  // (status 3.371).
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn buf_charpos_to_bytepos (b: *mut c_void, charpos: ptrdiff_t)
  -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  /* C's prototype is `ptrdiff_t buf_charpos_to_bytepos (struct buffer *, ptrdiff_t)`; the
     parameter is `*mut c_void` because that is the type lisp.h's mirror gives every caller
     (P-39's opaque `struct buffer`), and the body casts once.  */
  let b: *mut buffer = b as *mut buffer;
  let mut tail: *mut Lisp_Marker;
  let mut best_above: ptrdiff_t;
  let mut best_above_byte: ptrdiff_t;
  let mut best_below: ptrdiff_t;
  let mut best_below_byte: ptrdiff_t;
  let mut distance: ptrdiff_t = BYTECHAR_DISTANCE_INITIAL as ptrdiff_t;

  eassert! (BUF_BEG (b) <= charpos && charpos <= BUF_Z! (b));

  best_above = BUF_Z! (b);
  best_above_byte = BUF_Z_BYTE! (b);

  /* If this buffer has as many characters as bytes,
     each character must be one byte.
     This takes care of the case where enable-multibyte-characters is nil.  */
  if best_above == best_above_byte
    { return charpos; }

  best_below = BEG as ptrdiff_t;
  best_below_byte = BEG_BYTE as ptrdiff_t;

  /* We find in best_above and best_above_byte
     the closest known point above CHARPOS,
     and in best_below and best_below_byte
     the closest known point below CHARPOS,

     If at any point we can tell that the space between those
     two best approximations is all single-byte,
     we interpolate the result immediately.  */

  if let Some (value) =
    consider_charpos (b, charpos, BUF_PT (b), || BUF_PT_BYTE (b),
		      &mut best_above, &mut best_above_byte,
		      &mut best_below, &mut best_below_byte)
    { return value; }
  if let Some (value) =
    consider_charpos (b, charpos, BUF_GPT! (b), || BUF_GPT_BYTE! (b),
		      &mut best_above, &mut best_above_byte,
		      &mut best_below, &mut best_below_byte)
    { return value; }
  if let Some (value) =
    consider_charpos (b, charpos, BUF_BEGV (b), || BUF_BEGV_BYTE (b),
		      &mut best_above, &mut best_above_byte,
		      &mut best_below, &mut best_below_byte)
    { return value; }
  if let Some (value) =
    consider_charpos (b, charpos, BUF_ZV (b), || BUF_ZV_BYTE (b),
		      &mut best_above, &mut best_above_byte,
		      &mut best_below, &mut best_below_byte)
    { return value; }

  if (*cached_buffer.as_mut_ptr ()) == b && BUF_MODIFF! (b) == (*cached_modiff.as_mut_ptr ())
    {
      if let Some (value) =
	consider_charpos (b, charpos, (*cached_charpos.as_mut_ptr ()), || (*cached_bytepos.as_mut_ptr ()),
			  &mut best_above, &mut best_above_byte,
			  &mut best_below, &mut best_below_byte)
	{ return value; }
    }

  tail = BUF_MARKERS! (b);
  while !tail.is_null ()
      && !(best_above - charpos < distance
	   || charpos - best_below < distance)
    {
      if let Some (value) =
	consider_charpos (b, charpos, (*tail).charpos, || (*tail).bytepos,
			  &mut best_above, &mut best_above_byte,
			  &mut best_below, &mut best_below_byte)
	{ return value; }
      tail = (*tail).next;
      distance += BYTECHAR_DISTANCE_INCREMENT as ptrdiff_t;
    }

  /* We get here if we did not exactly hit one of the known places.
     We have one known above and one known below.
     Scan, counting characters, from whichever one is closer.  */

  eassert! (best_below <= charpos && charpos <= best_above);
  if charpos - best_below < best_above - charpos
    {
      let record: bool = charpos - best_below > 5000;

      while best_below < charpos
	{
	  best_below += 1;
	  best_below_byte += buf_next_char_len (b, best_below_byte) as ptrdiff_t;
	}

      /* If this position is quite far from the nearest known position,
	 cache the correspondence by creating a marker here.
	 It will last until the next GC.  */
      if record
	{ build_marker (b as *mut c_void, best_below, best_below_byte); }

      byte_char_debug_check! (b, best_below, best_below_byte);

      (*cached_buffer.as_mut_ptr ()) = b;
      (*cached_modiff.as_mut_ptr ()) = BUF_MODIFF! (b);
      (*cached_charpos.as_mut_ptr ()) = best_below;
      (*cached_bytepos.as_mut_ptr ()) = best_below_byte;

      return best_below_byte;
    }
  else
    {
      let record: bool = best_above - charpos > 5000;

      while best_above > charpos
	{
	  best_above -= 1;
	  best_above_byte -= buf_prev_char_len (b, best_above_byte) as ptrdiff_t;
	}

      /* If this position is quite far from the nearest known position,
	 cache the correspondence by creating a marker here.
	 It will last until the next GC.  */
      if record
	{ build_marker (b as *mut c_void, best_above, best_above_byte); }

      byte_char_debug_check! (b, best_above, best_above_byte);

      (*cached_buffer.as_mut_ptr ()) = b;
      (*cached_modiff.as_mut_ptr ()) = BUF_MODIFF! (b);
      (*cached_charpos.as_mut_ptr ()) = best_above;
      (*cached_bytepos.as_mut_ptr ()) = best_above_byte;

      return best_above_byte;
    }

  }
}

/* This macro is a subroutine of buf_bytepos_to_charpos.
   It is used when BYTEPOS is actually the byte position.  */

/* C 277-315's `CONSIDER (BYTEPOS, CHARPOS)`: here the eagerly evaluated operand is the byte
   position and the lazily evaluated one is the character position, so the helper takes
   `this_bytepos` and an `FnOnce` for `CHARPOS`; everything else is the first helper's
   shape (see the comment on consider_charpos).  C declares this macro's `changed` as `int`;
   the port uses `bool`, because C only ever stores false and true in it.  */
unsafe fn consider_bytepos (b: *mut buffer, bytepos: ptrdiff_t,
			    this_bytepos: ptrdiff_t,
			    charpos: impl FnOnce () -> ptrdiff_t,
			    best_above: &mut ptrdiff_t, best_above_byte: &mut ptrdiff_t,
			    best_below: &mut ptrdiff_t, best_below_byte: &mut ptrdiff_t)
  -> Option<ptrdiff_t>
{
  let mut changed: bool = false;

  if this_bytepos == bytepos
    {
      let value: ptrdiff_t = charpos ();

      byte_char_debug_check! (b, value, bytepos);
      return Some (value);
    }
  else if this_bytepos > bytepos
    {
      if this_bytepos < *best_above_byte
	{
	  *best_above = charpos ();
	  *best_above_byte = this_bytepos;
	  changed = true;
	}
    }
  else if this_bytepos > *best_below_byte
    {
      *best_below = charpos ();
      *best_below_byte = this_bytepos;
      changed = true;
    }

  if changed
    {
      if *best_above - *best_below == *best_above_byte - *best_below_byte
	{
	  let value: ptrdiff_t = *best_below + (bytepos - *best_below_byte);

	  byte_char_debug_check! (b, value, bytepos);
	  return Some (value);
	}
    }
  return None;
}

/* Return the character position corresponding to BYTEPOS in B.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn buf_bytepos_to_charpos (b: *mut c_void, bytepos: ptrdiff_t)
  -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  /* As in buf_charpos_to_bytepos: C's parameter is `struct buffer *`, and the `*mut c_void`
     is what lisp.h's mirror gives the callers (P-39).  */
  let b: *mut buffer = b as *mut buffer;
  let mut tail: *mut Lisp_Marker;
  let mut best_above: ptrdiff_t;
  let mut best_above_byte: ptrdiff_t;
  let mut best_below: ptrdiff_t;
  let mut best_below_byte: ptrdiff_t;
  let mut distance: ptrdiff_t = BYTECHAR_DISTANCE_INITIAL as ptrdiff_t;

  eassert! (BUF_BEG_BYTE (b) <= bytepos && bytepos <= BUF_Z_BYTE! (b));

  best_above = BUF_Z! (b);
  best_above_byte = BUF_Z_BYTE! (b);

  /* If this buffer has as many characters as bytes,
     each character must be one byte.
     This takes care of the case where enable-multibyte-characters is nil.  */
  if best_above == best_above_byte
    { return bytepos; }

  /* Check bytepos is not in the middle of a character. */
  eassert! (bytepos >= BUF_Z_BYTE! (b)
	    || CHAR_HEAD_P (BUF_FETCH_BYTE (b, bytepos) as c_int));

  best_below = BEG as ptrdiff_t;
  best_below_byte = BEG_BYTE as ptrdiff_t;

  if let Some (value) =
    consider_bytepos (b, bytepos, BUF_PT_BYTE (b), || BUF_PT (b),
		      &mut best_above, &mut best_above_byte,
		      &mut best_below, &mut best_below_byte)
    { return value; }
  if let Some (value) =
    consider_bytepos (b, bytepos, BUF_GPT_BYTE! (b), || BUF_GPT! (b),
		      &mut best_above, &mut best_above_byte,
		      &mut best_below, &mut best_below_byte)
    { return value; }
  if let Some (value) =
    consider_bytepos (b, bytepos, BUF_BEGV_BYTE (b), || BUF_BEGV (b),
		      &mut best_above, &mut best_above_byte,
		      &mut best_below, &mut best_below_byte)
    { return value; }
  if let Some (value) =
    consider_bytepos (b, bytepos, BUF_ZV_BYTE (b), || BUF_ZV (b),
		      &mut best_above, &mut best_above_byte,
		      &mut best_below, &mut best_below_byte)
    { return value; }

  if (*cached_buffer.as_mut_ptr ()) == b && BUF_MODIFF! (b) == (*cached_modiff.as_mut_ptr ())
    {
      if let Some (value) =
	consider_bytepos (b, bytepos, (*cached_bytepos.as_mut_ptr ()), || (*cached_charpos.as_mut_ptr ()),
			  &mut best_above, &mut best_above_byte,
			  &mut best_below, &mut best_below_byte)
	{ return value; }
    }

  tail = BUF_MARKERS! (b);
  while !tail.is_null ()
      && !(best_above_byte - bytepos < distance
	   || bytepos - best_below_byte < distance)
    {
      if let Some (value) =
	consider_bytepos (b, bytepos, (*tail).bytepos, || (*tail).charpos,
			  &mut best_above, &mut best_above_byte,
			  &mut best_below, &mut best_below_byte)
	{ return value; }
      tail = (*tail).next;
      distance += BYTECHAR_DISTANCE_INCREMENT as ptrdiff_t;
    }

  /* We get here if we did not exactly hit one of the known places.
     We have one known above and one known below.
     Scan, counting characters, from whichever one is closer.  */

  if bytepos - best_below_byte < best_above_byte - bytepos
    {
      let record: bool = bytepos - best_below_byte > 5000;

      while best_below_byte < bytepos
	{
	  best_below += 1;
	  best_below_byte += buf_next_char_len (b, best_below_byte) as ptrdiff_t;
	}

      /* If this position is quite far from the nearest known position,
	 cache the correspondence by creating a marker here.
	 It will last until the next GC.
	 But don't do it if BUF_MARKERS is nil;
	 that is a signal from Fset_buffer_multibyte.  */
      if record && !BUF_MARKERS! (b).is_null ()
	{ build_marker (b as *mut c_void, best_below, best_below_byte); }

      byte_char_debug_check! (b, best_below, best_below_byte);

      (*cached_buffer.as_mut_ptr ()) = b;
      (*cached_modiff.as_mut_ptr ()) = BUF_MODIFF! (b);
      (*cached_charpos.as_mut_ptr ()) = best_below;
      (*cached_bytepos.as_mut_ptr ()) = best_below_byte;

      return best_below;
    }
  else
    {
      let record: bool = best_above_byte - bytepos > 5000;

      while best_above_byte > bytepos
	{
	  best_above -= 1;
	  best_above_byte -= buf_prev_char_len (b, best_above_byte) as ptrdiff_t;
	}

      /* If this position is quite far from the nearest known position,
	 cache the correspondence by creating a marker here.
	 It will last until the next GC.
	 But don't do it if BUF_MARKERS is nil;
	 that is a signal from Fset_buffer_multibyte.  */
      if record && !BUF_MARKERS! (b).is_null ()
	{ build_marker (b as *mut c_void, best_above, best_above_byte); }

      byte_char_debug_check! (b, best_above, best_above_byte);

      (*cached_buffer.as_mut_ptr ()) = b;
      (*cached_modiff.as_mut_ptr ()) = BUF_MODIFF! (b);
      (*cached_charpos.as_mut_ptr ()) = best_above;
      (*cached_bytepos.as_mut_ptr ()) = best_above_byte;

      return best_above;
    }

  }
}

/* Operations on markers. */

DEFUN! ("marker-buffer", Fmarker_buffer, Smarker_buffer, 1, 1, 0,
       doc: "Return the buffer that MARKER points into, or nil if none.\nReturns nil if MARKER points into a dead buffer.  ",
       (marker: Lisp_Object),
       {
  let mut buf: Lisp_Object;
  CHECK_MARKER (marker);
  if !(*XMARKER (marker)).buffer.is_null ()
    {
      XSETBUFFER! (buf, (*XMARKER (marker)).buffer);
      /* If the buffer is dead, we're in trouble: the buffer pointer here
	 does not preserve the buffer from being GC'd (it's weak), so
	 markers have to be unlinked from their buffer as soon as the buffer
	 is killed.  */
      eassert! (BUFFER_LIVE_P (XBUFFER (buf)));
      return buf;
    }
  return Qnil ();
});

DEFUN! ("marker-position", Fmarker_position, Smarker_position, 1, 1, 0,
       doc: "Return the position of MARKER, or nil if it points nowhere.",
       (marker: Lisp_Object),
       {
  CHECK_MARKER (marker);
  if !(*XMARKER (marker)).buffer.is_null ()
    { return make_fixnum ((*XMARKER (marker)).charpos); }

  return Qnil ();
});

DEFUN! ("marker-last-position", Fmarker_last_position, Smarker_last_position, 1, 1, 0,
       doc: "Return last position of MARKER in its buffer.\nThis is like `marker-position' with one exception:  If the buffer of\nMARKER is dead, it returns the last position of MARKER in that buffer\nbefore it was killed.  ",
       (marker: Lisp_Object),
       {
  CHECK_MARKER (marker);

  return make_fixnum ((*XMARKER (marker)).charpos);
});

/* Change M so it points to B at CHARPOS and BYTEPOS.  */

unsafe fn attach_marker (m: *mut Lisp_Marker, b: *mut buffer,
			 charpos: ptrdiff_t, bytepos: ptrdiff_t)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  /* In a single-byte buffer, two positions must be equal.
     Otherwise, every character is at least one byte.  */
  if BUF_Z! (b) == BUF_Z_BYTE! (b)
    { eassert! (charpos == bytepos); }
  else
    { eassert! (charpos <= bytepos); }

  (*m).charpos = charpos;
  (*m).bytepos = bytepos;

  /* `Lisp_Marker.buffer` is `struct buffer *` in C but `*mut c_void` in the lisp.h mirror
     (the same opaque-type consequence as the two conversion functions' parameters).  */
  if (*m).buffer != b as *mut c_void
    {
      unchain_marker (m);
      (*m).buffer = b as *mut c_void;
      (*m).next = BUF_MARKERS! (b);
      BUF_MARKERS! (b) = m;
    }

  }
}

/* If BUFFER is nil, return current buffer pointer.  Next, check
   whether BUFFER is a buffer object and return buffer pointer
   corresponding to BUFFER if BUFFER is live, or NULL otherwise.  */

unsafe fn live_buffer (buffer: Lisp_Object) -> *mut buffer
{
  let b: *mut buffer = decode_buffer (buffer);
  return if BUFFER_LIVE_P (b) { b } else { std::ptr::null_mut () };
}

/* Internal function to set MARKER in BUFFER at POSITION.  Non-zero
   RESTRICTED means limit the POSITION by the visible part of BUFFER.  */

unsafe fn set_marker_internal (marker: Lisp_Object, position: Lisp_Object,
			       buffer: Lisp_Object, restricted: bool) -> Lisp_Object
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let m: *mut Lisp_Marker;
  let b: *mut buffer = live_buffer (buffer);

  CHECK_MARKER (marker);
  m = XMARKER (marker);

  /* Set MARKER to point nowhere if BUFFER is dead, or
     POSITION is nil or a marker points to nowhere.  */
  if NILP (position)
      || (MARKERP (position) && (*XMARKER (position)).buffer.is_null ())
      || b.is_null ()
    { unchain_marker (m); }

  /* Optimize the special case where we are copying the position of
     an existing marker, and MARKER is already in the same buffer.  */
  else if MARKERP (position) && b as *mut c_void == (*XMARKER (position)).buffer
	   && b as *mut c_void == (*m).buffer
    {
      (*m).bytepos = (*XMARKER (position)).bytepos;
      (*m).charpos = (*XMARKER (position)).charpos;
    }

  else
    {
      let mut charpos: ptrdiff_t;
      let mut bytepos: ptrdiff_t;

      /* Do not use CHECK_FIXNUM_COERCE_MARKER because we
	 don't want to call buf_charpos_to_bytepos if POSITION
	 is a marker and so we know the bytepos already.  */
      if FIXNUMP (position)
	{
	  /* C 544-553's `#if EMACS_INT_MAX > PTRDIFF_MAX` is false on the frozen target
	     (both are the 64-bit word), so C's `#else` arm is the live text; the
	     wide-int arm is recorded in docs/ports/marker.md.  */
	  charpos = XFIXNUM (position);
	  bytepos = -1;
	}
      else if MARKERP (position)
	{
	  charpos = (*XMARKER (position)).charpos;
	  bytepos = (*XMARKER (position)).bytepos;
	}
      else
	{ wrong_type_argument (Qinteger_or_marker_p (), position); }

      charpos = clip_to_bounds
	(if restricted { BUF_BEGV (b) } else { BUF_BEG (b) }, charpos,
	 if restricted { BUF_ZV (b) } else { BUF_Z! (b) });
      /* Don't believe BYTEPOS if it comes from a different buffer,
	 since that buffer might have a very different correspondence
	 between character and byte positions.  */
      if bytepos == -1
	  || !(MARKERP (position)
	       && (*XMARKER (position)).buffer == b as *mut c_void)
	{ bytepos = buf_charpos_to_bytepos (b as *mut c_void, charpos); }
      else
	{ bytepos = clip_to_bounds
	  (if restricted { BUF_BEGV_BYTE (b) } else { BUF_BEG_BYTE (b) },
	   bytepos, if restricted { BUF_ZV_BYTE (b) } else { BUF_Z_BYTE! (b) }); }

      attach_marker (m, b, charpos, bytepos);
    }

  /* C 580-602's `#ifdef HAVE_TEXT_CONVERSION` arm is dead in the frozen configuration, and
     its body needs `bset_redisplay`, which this target's mirror has not got; the arm's text
     is recorded in docs/ports/marker.md (the rawmemchr.c CHERI-arm class).  */

  return marker;

  }
}

DEFUN! ("set-marker", Fset_marker, Sset_marker, 2, 3, 0,
       doc: "Position MARKER before character number POSITION in BUFFER.\nIf BUFFER is omitted or nil, it defaults to the current buffer.  If\nPOSITION is nil, makes marker point nowhere so it no longer slows down\nediting in any buffer.  Returns MARKER.",
       (marker: Lisp_Object, position: Lisp_Object, buffer: Lisp_Object),
       {
  return set_marker_internal (marker, position, buffer, false);
});

/* Like the above, but won't let the position be outside the visible part.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_marker_restricted (marker: Lisp_Object, position: Lisp_Object,
						buffer: Lisp_Object) -> Lisp_Object
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  return set_marker_internal (marker, position, buffer, true);

  }
}

/* Set the position of MARKER, specifying both the
   character position and the corresponding byte position.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_marker_both (marker: Lisp_Object, buffer: Lisp_Object,
					  charpos: ptrdiff_t, bytepos: ptrdiff_t)
  -> Lisp_Object
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let m: *mut Lisp_Marker;
  let b: *mut buffer = live_buffer (buffer);

  CHECK_MARKER (marker);
  m = XMARKER (marker);

  if !b.is_null ()
    { attach_marker (m, b, charpos, bytepos); }
  else
    { unchain_marker (m); }
  return marker;

  }
}

/* Like the above, but won't let the position be outside the visible part.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_marker_restricted_both (marker: Lisp_Object,
						     buffer: Lisp_Object,
						     charpos: ptrdiff_t,
						     bytepos: ptrdiff_t) -> Lisp_Object
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let m: *mut Lisp_Marker;
  let b: *mut buffer = live_buffer (buffer);

  CHECK_MARKER (marker);
  m = XMARKER (marker);

  if !b.is_null ()
    {
      attach_marker
	(m, b,
	 clip_to_bounds (BUF_BEGV (b), charpos, BUF_ZV (b)),
	 clip_to_bounds (BUF_BEGV_BYTE (b), bytepos, BUF_ZV_BYTE (b)));
    }
  else
    { unchain_marker (m); }
  return marker;

  }
}

/* Detach a marker so that it no longer points anywhere and no longer
   slows down editing.  Do not free the marker, though, as a change
   function could have inserted it into an undo list (Bug#30931).  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn detach_marker (marker: Lisp_Object)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  Fset_marker (marker, Qnil (), Qnil ());

  }
}

/* Remove MARKER from the chain of whatever buffer it is in.  Set its
   buffer NULL.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn unchain_marker (marker: *mut Lisp_Marker)
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let b: *mut buffer = (*marker).buffer as *mut buffer;

  if !b.is_null ()
    {
      let mut tail: *mut Lisp_Marker;
      let mut prev: *mut *mut Lisp_Marker;

      /* No dead buffers here.  */
      eassert! (BUFFER_LIVE_P (b));

      (*marker).buffer = std::ptr::null_mut ();
      prev = &raw mut BUF_MARKERS! (b);

      tail = BUF_MARKERS! (b);
      while !tail.is_null ()
	{
	  if marker == tail
	    {
	      if *prev == BUF_MARKERS! (b)
		{
		  /* Deleting first marker from the buffer's chain.  Crash
		     if new first marker in chain does not say it belongs
		     to the same buffer, or at least that they have the same
		     base buffer.  */
		  if !(*tail).next.is_null ()
		    {
		      /* C reaches this through `tail->next->buffer->text`; the mirror types
			 `buffer` as `*mut c_void` (lisp.h's opaque `struct buffer`), so the
			 cast sits here.  */
		      let next_buffer: *mut buffer = (*((*tail).next)).buffer as *mut buffer;
		      if (*b).text != (*next_buffer).text
			{ emacs_abort (); }
		    }
		}
	      *prev = (*tail).next;
	      /* We have removed the marker from the chain;
		 no need to scan the rest of the chain.  */
	      break;
	    }
	  prev = &raw mut (*tail).next;
	  tail = *prev;
	}

      /* Error if marker was not in it's chain.  */
      eassert! (!tail.is_null ());
    }

  }
}

/* Return the char position of marker MARKER, as a C integer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn marker_position (marker: Lisp_Object) -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let m: *mut Lisp_Marker = XMARKER (marker);
  let buf: *mut buffer = (*m).buffer as *mut buffer;

  if buf.is_null ()
    { error (c"Marker does not point anywhere".as_ptr ()); }

  eassert! (BUF_BEG (buf) <= (*m).charpos && (*m).charpos <= BUF_Z! (buf));

  return (*m).charpos;

  }
}

/* Return the byte position of marker MARKER, as a C integer.  */

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn marker_byte_position (marker: Lisp_Object) -> ptrdiff_t
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  let m: *mut Lisp_Marker = XMARKER (marker);
  let buf: *mut buffer = (*m).buffer as *mut buffer;

  if buf.is_null ()
    { error (c"Marker does not point anywhere".as_ptr ()); }

  eassert! (BUF_BEG_BYTE (buf) <= (*m).bytepos && (*m).bytepos <= BUF_Z_BYTE! (buf));

  return (*m).bytepos;

  }
}

DEFUN! ("copy-marker", Fcopy_marker, Scopy_marker, 0, 2, 0,
       doc: "Return a new marker pointing at the same place as MARKER.\nIf argument is a number, makes a new marker pointing\nat that position in the current buffer.\nIf MARKER is not specified, the new marker does not point anywhere.\nThe optional argument TYPE specifies the insertion type of the new marker;\nsee `marker-insertion-type'.  ",
       (marker: Lisp_Object, r#type: Lisp_Object),
       {
  let new: Lisp_Object;

  if !NILP (marker)
    { CHECK_TYPE (FIXNUMP (marker) || MARKERP (marker), Qinteger_or_marker_p (), marker); }

  new = Fmake_marker ();
  Fset_marker (new, marker,
	       if MARKERP (marker) { Fmarker_buffer (marker) } else { Qnil () });
  (*XMARKER (new)).bits = if !NILP (r#type)
    { (*XMARKER (new)).bits | MARKER_INSERTION_TYPE }
    else
    { (*XMARKER (new)).bits & !MARKER_INSERTION_TYPE };
  return new;
});

DEFUN! ("marker-insertion-type", Fmarker_insertion_type,
	Smarker_insertion_type, 1, 1, 0,
       doc: "Return insertion type of MARKER: t if it stays after inserted text.\nThe value nil means the marker stays before text inserted there.  ",
       (marker: Lisp_Object),
       {
  CHECK_MARKER (marker);
  return if MARKER_INSERTION_TYPE_P (XMARKER (marker)) { Qt () } else { Qnil () };
});

DEFUN! ("set-marker-insertion-type", Fset_marker_insertion_type,
	Sset_marker_insertion_type, 2, 2, 0,
       doc: "Set the insertion-type of MARKER to TYPE.\nIf TYPE is t, it means the marker advances when you insert text at it.\nIf TYPE is nil, it means the marker stays behind when you insert text at it.  ",
       (marker: Lisp_Object, r#type: Lisp_Object),
       {
  CHECK_MARKER (marker);

  (*XMARKER (marker)).bits = if !NILP (r#type)
    { (*XMARKER (marker)).bits | MARKER_INSERTION_TYPE }
    else
    { (*XMARKER (marker)).bits & !MARKER_INSERTION_TYPE };
  return r#type;
});

DEFUN! ("markers-in", Fmarkers_in, Smarkers_in, 0, 2, 0,
       doc: "Return the list of markers in region BEG..END.\nThe list includes markers at BEG and at END.  ",
       (beg: Lisp_Object, end: Lisp_Object),
       {
  let mut res: Lisp_Object = Qnil ();
  let mut tail: *mut Lisp_Marker;
  let mut ibeg: ptrdiff_t;
  let mut iend: ptrdiff_t;
  let mut beg: Lisp_Object = beg;
  let mut end: Lisp_Object = end;
  if NILP (beg)
    { ibeg = BEGV! (); }
  else
    {
      CHECK_FIXNUM_COERCE_MARKER! (beg);
      ibeg = clip_to_bounds (BEGV! (), XFIXNUM (beg), ZV! ());
    }
  if NILP (end)
    { iend = ZV! (); }
  else
    {
      CHECK_FIXNUM_COERCE_MARKER! (end);
      iend = clip_to_bounds (BEGV! (), XFIXNUM (end), ZV! ());
    }

  tail = BUF_MARKERS! ((current_buffer! ()));
  while !tail.is_null ()
    {
      if ibeg <= (*tail).charpos && (*tail).charpos <= iend
	{ res = Fcons (make_lisp_ptr (tail as *mut c_void, Lisp_Type::Lisp_Vectorlike), res); }
      tail = (*tail).next;
    }

  return res;
});

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_marker ()
{
  // SAFETY: low-level VM/OS subsystem boundary; caller upholds active Lisp thread ownership, tagged `Lisp_Object` / buffer pointer validity, and `NonLocalExit` error propagation.
  unsafe {
  defsubr (Smarker_position.as_mut_ptr ());
  defsubr (Smarker_last_position.as_mut_ptr ());
  defsubr (Smarker_buffer.as_mut_ptr ());
  defsubr (Sset_marker.as_mut_ptr ());
  defsubr (Scopy_marker.as_mut_ptr ());
  defsubr (Smarker_insertion_type.as_mut_ptr ());
  defsubr (Sset_marker_insertion_type.as_mut_ptr ());
  defsubr (Smarkers_in.as_mut_ptr ());

  }
}
