// femacs: port of src/marker.c @ fd7afced403.
//
// High-level marker Lisp primitives live in `#![forbid(unsafe_code)]` `crate::safe_marker`
// (`U7` / `P-486`); low-level intrusive `Lisp_Marker` buffer-chain and byte/char position
// cache primitives remain behind controlled boundaries here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

const _: () = ();

static cached_charpos: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);
static cached_bytepos: VmGlobal<ptrdiff_t> = VmGlobal::<ptrdiff_t>::new (0);
static cached_buffer: VmGlobal<*mut buffer> = VmGlobal::<*mut buffer>::new (std::ptr::null_mut ());
static cached_modiff: VmGlobal<modiff_count> = VmGlobal::<modiff_count>::new (0);

#[cfg (feature = "marker_debug")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn count_markers (buf: *mut buffer) -> c_int
{
  let mut total: c_int = 0;
  // SAFETY: debug helper walks live `BUF_MARKERS(buf)` chain.
  unsafe {
    let mut tail: *mut Lisp_Marker = BUF_MARKERS! (buf);
    while !tail.is_null () {
      total += 1;
      tail = (*tail).next;
    }
  }
  total
}

#[cfg (feature = "marker_debug")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn verify_bytepos (charpos: ptrdiff_t) -> ptrdiff_t
{
  let mut below: ptrdiff_t = BEG as ptrdiff_t;
  let mut below_byte: ptrdiff_t = BEG_BYTE as ptrdiff_t;
  // SAFETY: debug helper walks `current_buffer` bytes up to `charpos`.
  unsafe {
    while below != charpos {
      below += 1;
      below_byte += buf_next_char_len (current_buffer! (), below_byte) as ptrdiff_t;
    }
  }
  below_byte
}

macro_rules! byte_char_debug_check {
    ($b:expr, $charpos:expr, $bytepos:expr) => { };
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn clear_charpos_cache (b: *mut buffer)
{
  if cached_buffer.get () == b {
    cached_buffer.set (core::ptr::null_mut ());
  }
}

fn consider_charpos (
  b: *mut buffer,
  charpos: ptrdiff_t,
  this_charpos: ptrdiff_t,
  bytepos: impl FnOnce () -> ptrdiff_t,
  best_above: &mut ptrdiff_t,
  best_above_byte: &mut ptrdiff_t,
  best_below: &mut ptrdiff_t,
  best_below_byte: &mut ptrdiff_t,
) -> Option<ptrdiff_t>
{
  let mut changed = false;

  if this_charpos == charpos {
    let value = bytepos ();
    byte_char_debug_check! (b, charpos, value);
    return Some (value);
  } else if this_charpos > charpos {
    if this_charpos < *best_above {
      *best_above = this_charpos;
      *best_above_byte = bytepos ();
      changed = true;
    }
  } else if this_charpos > *best_below {
    *best_below = this_charpos;
    *best_below_byte = bytepos ();
    changed = true;
  }

  if changed && *best_above - *best_below == *best_above_byte - *best_below_byte {
    let value = *best_below_byte + (charpos - *best_below);
    byte_char_debug_check! (b, charpos, value);
    return Some (value);
  }
  None
}

#[inline(always)]
fn CHECK_MARKER (x: Lisp_Object)
{
  CHECK_TYPE (MARKERP (x), Qmarkerp (), x);
}

#[inline(always)]
pub(crate) fn marker_raw_charpos (marker: Lisp_Object) -> ptrdiff_t
{
  assert! (MARKERP (marker));
  // SAFETY: `MARKERP(marker)` verified above; reads `charpos` even when detached (`marker-last-position`).
  unsafe { (*XMARKER (marker)).charpos }
}

const BYTECHAR_DISTANCE_INITIAL: c_int = 50;
const BYTECHAR_DISTANCE_INCREMENT: c_int = 50;

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn buf_charpos_to_bytepos (b: *mut c_void, charpos: ptrdiff_t)
  -> ptrdiff_t
{
  // SAFETY: caller supplies a live `buffer *` (as `*mut c_void`) and `charpos` within `[BEG, Z]`.
  unsafe {
    let b: *mut buffer = b as *mut buffer;
    let mut distance: ptrdiff_t = BYTECHAR_DISTANCE_INITIAL as ptrdiff_t;

    eassert! (BUF_BEG (b) <= charpos && charpos <= BUF_Z! (b));

    let mut best_above = BUF_Z! (b);
    let mut best_above_byte = BUF_Z_BYTE! (b);
    if best_above == best_above_byte {
      return charpos;
    }

    let mut best_below = BEG as ptrdiff_t;
    let mut best_below_byte = BEG_BYTE as ptrdiff_t;

    if let Some (value) = consider_charpos (
      b, charpos, BUF_PT (b), || BUF_PT_BYTE (b),
      &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
    ) {
      return value;
    }
    if let Some (value) = consider_charpos (
      b, charpos, BUF_GPT! (b), || BUF_GPT_BYTE! (b),
      &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
    ) {
      return value;
    }
    if let Some (value) = consider_charpos (
      b, charpos, BUF_BEGV (b), || BUF_BEGV_BYTE (b),
      &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
    ) {
      return value;
    }
    if let Some (value) = consider_charpos (
      b, charpos, BUF_ZV (b), || BUF_ZV_BYTE (b),
      &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
    ) {
      return value;
    }

    if cached_buffer.get () == b && BUF_MODIFF! (b) == cached_modiff.get () {
      if let Some (value) = consider_charpos (
        b, charpos, cached_charpos.get (), || cached_bytepos.get (),
        &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
      ) {
        return value;
      }
    }

    let mut tail = BUF_MARKERS! (b);
    while !tail.is_null ()
      && !(best_above - charpos < distance || charpos - best_below < distance)
    {
      if let Some (value) = consider_charpos (
        b, charpos, (*tail).charpos, || (*tail).bytepos,
        &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
      ) {
        return value;
      }
      tail = (*tail).next;
      distance += BYTECHAR_DISTANCE_INCREMENT as ptrdiff_t;
    }

    eassert! (best_below <= charpos && charpos <= best_above);
    if charpos - best_below < best_above - charpos {
      let record = charpos - best_below > 5000;
      while best_below < charpos {
        best_below += 1;
        best_below_byte += buf_next_char_len (b, best_below_byte) as ptrdiff_t;
      }
      if record {
        build_marker (b as *mut c_void, best_below, best_below_byte);
      }
      byte_char_debug_check! (b, best_below, best_below_byte);
      cached_buffer.set (b);
      cached_modiff.set (BUF_MODIFF! (b));
      cached_charpos.set (best_below);
      cached_bytepos.set (best_below_byte);
      best_below_byte
    } else {
      let record = best_above - charpos > 5000;
      while best_above > charpos {
        best_above -= 1;
        best_above_byte -= buf_prev_char_len (b, best_above_byte) as ptrdiff_t;
      }
      if record {
        build_marker (b as *mut c_void, best_above, best_above_byte);
      }
      byte_char_debug_check! (b, best_above, best_above_byte);
      cached_buffer.set (b);
      cached_modiff.set (BUF_MODIFF! (b));
      cached_charpos.set (best_above);
      cached_bytepos.set (best_above_byte);
      best_above_byte
    }
  }
}

fn consider_bytepos (
  b: *mut buffer,
  bytepos: ptrdiff_t,
  this_bytepos: ptrdiff_t,
  charpos: impl FnOnce () -> ptrdiff_t,
  best_above: &mut ptrdiff_t,
  best_above_byte: &mut ptrdiff_t,
  best_below: &mut ptrdiff_t,
  best_below_byte: &mut ptrdiff_t,
) -> Option<ptrdiff_t>
{
  let mut changed = false;

  if this_bytepos == bytepos {
    let value = charpos ();
    byte_char_debug_check! (b, value, bytepos);
    return Some (value);
  } else if this_bytepos > bytepos {
    if this_bytepos < *best_above_byte {
      *best_above = charpos ();
      *best_above_byte = this_bytepos;
      changed = true;
    }
  } else if this_bytepos > *best_below_byte {
    *best_below = charpos ();
    *best_below_byte = this_bytepos;
    changed = true;
  }

  if changed && *best_above - *best_below == *best_above_byte - *best_below_byte {
    let value = *best_below + (bytepos - *best_below_byte);
    byte_char_debug_check! (b, value, bytepos);
    return Some (value);
  }
  None
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn buf_bytepos_to_charpos (b: *mut c_void, bytepos: ptrdiff_t)
  -> ptrdiff_t
{
  // SAFETY: caller supplies a live `buffer *` (as `*mut c_void`) and character-head `bytepos` within `[BEG_BYTE, Z_BYTE]`.
  unsafe {
    let b: *mut buffer = b as *mut buffer;
    let mut distance: ptrdiff_t = BYTECHAR_DISTANCE_INITIAL as ptrdiff_t;

    eassert! (BUF_BEG_BYTE (b) <= bytepos && bytepos <= BUF_Z_BYTE! (b));

    let mut best_above = BUF_Z! (b);
    let mut best_above_byte = BUF_Z_BYTE! (b);
    if best_above == best_above_byte {
      return bytepos;
    }

    eassert! (bytepos >= BUF_Z_BYTE! (b) || CHAR_HEAD_P (BUF_FETCH_BYTE (b, bytepos) as c_int));

    let mut best_below = BEG as ptrdiff_t;
    let mut best_below_byte = BEG_BYTE as ptrdiff_t;

    if let Some (value) = consider_bytepos (
      b, bytepos, BUF_PT_BYTE (b), || BUF_PT (b),
      &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
    ) {
      return value;
    }
    if let Some (value) = consider_bytepos (
      b, bytepos, BUF_GPT_BYTE! (b), || BUF_GPT! (b),
      &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
    ) {
      return value;
    }
    if let Some (value) = consider_bytepos (
      b, bytepos, BUF_BEGV_BYTE (b), || BUF_BEGV (b),
      &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
    ) {
      return value;
    }
    if let Some (value) = consider_bytepos (
      b, bytepos, BUF_ZV_BYTE (b), || BUF_ZV (b),
      &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
    ) {
      return value;
    }

    if cached_buffer.get () == b && BUF_MODIFF! (b) == cached_modiff.get () {
      if let Some (value) = consider_bytepos (
        b, bytepos, cached_bytepos.get (), || cached_charpos.get (),
        &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
      ) {
        return value;
      }
    }

    let mut tail = BUF_MARKERS! (b);
    while !tail.is_null ()
      && !(best_above_byte - bytepos < distance || bytepos - best_below_byte < distance)
    {
      if let Some (value) = consider_bytepos (
        b, bytepos, (*tail).bytepos, || (*tail).charpos,
        &mut best_above, &mut best_above_byte, &mut best_below, &mut best_below_byte,
      ) {
        return value;
      }
      tail = (*tail).next;
      distance += BYTECHAR_DISTANCE_INCREMENT as ptrdiff_t;
    }

    if bytepos - best_below_byte < best_above_byte - bytepos {
      let record = bytepos - best_below_byte > 5000;
      while best_below_byte < bytepos {
        best_below += 1;
        best_below_byte += buf_next_char_len (b, best_below_byte) as ptrdiff_t;
      }
      if record && !BUF_MARKERS! (b).is_null () {
        build_marker (b as *mut c_void, best_below, best_below_byte);
      }
      byte_char_debug_check! (b, best_below, best_below_byte);
      cached_buffer.set (b);
      cached_modiff.set (BUF_MODIFF! (b));
      cached_charpos.set (best_below);
      cached_bytepos.set (best_below_byte);
      best_below
    } else {
      let record = best_above_byte - bytepos > 5000;
      while best_above_byte > bytepos {
        best_above -= 1;
        best_above_byte -= buf_prev_char_len (b, best_above_byte) as ptrdiff_t;
      }
      if record && !BUF_MARKERS! (b).is_null () {
        build_marker (b as *mut c_void, best_above, best_above_byte);
      }
      byte_char_debug_check! (b, best_above, best_above_byte);
      cached_buffer.set (b);
      cached_modiff.set (BUF_MODIFF! (b));
      cached_charpos.set (best_above);
      cached_bytepos.set (best_above_byte);
      best_above
    }
  }
}

DEFUN! ("marker-buffer", Fmarker_buffer, Smarker_buffer, 1, 1, 0,
       doc: "Return the buffer that MARKER points into, or nil if none.\nReturns nil if MARKER points into a dead buffer.  ",
       (marker: Lisp_Object),
       {
  crate::safe_marker::marker_buffer (marker)
});

DEFUN! ("marker-position", Fmarker_position, Smarker_position, 1, 1, 0,
       doc: "Return the position of MARKER, or nil if it points nowhere.",
       (marker: Lisp_Object),
       {
  crate::safe_marker::marker_position_subr (marker)
});

DEFUN! ("marker-last-position", Fmarker_last_position, Smarker_last_position, 1, 1, 0,
       doc: "Return last position of MARKER in its buffer.\nThis is like `marker-position' with one exception:  If the buffer of\nMARKER is dead, it returns the last position of MARKER in that buffer\nbefore it was killed.  ",
       (marker: Lisp_Object),
       {
  crate::safe_marker::marker_last_position_subr (marker)
});

unsafe fn attach_marker (
  m: *mut Lisp_Marker,
  b: *mut buffer,
  charpos: ptrdiff_t,
  bytepos: ptrdiff_t,
)
{
  // SAFETY: `m` is a valid `Lisp_Marker` and `b` is a live `buffer`.
  unsafe {
    if BUF_Z! (b) == BUF_Z_BYTE! (b) {
      eassert! (charpos == bytepos);
    } else {
      eassert! (charpos <= bytepos);
    }

    (*m).charpos = charpos;
    (*m).bytepos = bytepos;

    if (*m).buffer != b as *mut c_void {
      unchain_marker (m);
      (*m).buffer = b as *mut c_void;
      (*m).next = BUF_MARKERS! (b);
      BUF_MARKERS! (b) = m;
    }
  }
}

#[inline(always)]
fn live_buffer (buffer: Lisp_Object) -> *mut buffer
{
  let b = decode_buffer (buffer);
  if BUFFER_LIVE_P (b) { b } else { core::ptr::null_mut () }
}

fn set_marker_internal (
  marker: Lisp_Object,
  position: Lisp_Object,
  buffer: Lisp_Object,
  restricted: bool,
) -> Lisp_Object
{
  let b = live_buffer (buffer);
  CHECK_MARKER (marker);
  let m = XMARKER (marker);

  // SAFETY: `m` verified `MARKERP` above and `b` is either null or a live `buffer`.
  unsafe {
    if NILP (position)
      || (MARKERP (position) && (*XMARKER (position)).buffer.is_null ())
      || b.is_null ()
    {
      unchain_marker (m);
    } else if MARKERP (position)
      && b as *mut c_void == (*XMARKER (position)).buffer
      && b as *mut c_void == (*m).buffer
    {
      (*m).bytepos = (*XMARKER (position)).bytepos;
      (*m).charpos = (*XMARKER (position)).charpos;
    } else {
      let (mut charpos, mut bytepos) = if FIXNUMP (position) {
        (XFIXNUM (position), -1)
      } else if MARKERP (position) {
        ((*XMARKER (position)).charpos, (*XMARKER (position)).bytepos)
      } else {
        wrong_type_argument (Qinteger_or_marker_p (), position)
      };

      charpos = clip_to_bounds (
        if restricted { BUF_BEGV (b) } else { BUF_BEG (b) },
        charpos,
        if restricted { BUF_ZV (b) } else { BUF_Z! (b) },
      );
      if bytepos == -1
        || !(MARKERP (position) && (*XMARKER (position)).buffer == b as *mut c_void)
      {
        bytepos = buf_charpos_to_bytepos (b as *mut c_void, charpos);
      } else {
        bytepos = clip_to_bounds (
          if restricted { BUF_BEGV_BYTE (b) } else { BUF_BEG_BYTE (b) },
          bytepos,
          if restricted { BUF_ZV_BYTE (b) } else { BUF_Z_BYTE! (b) },
        );
      }

      attach_marker (m, b, charpos, bytepos);
    }
  }
  marker
}

DEFUN! ("set-marker", Fset_marker, Sset_marker, 2, 3, 0,
       doc: "Position MARKER before character number POSITION in BUFFER.\nIf BUFFER is omitted or nil, it defaults to the current buffer.  If\nPOSITION is nil, makes marker point nowhere so it no longer slows down\nediting in any buffer.  Returns MARKER.",
       (marker: Lisp_Object, position: Lisp_Object, buffer: Lisp_Object),
       {
  set_marker_internal (marker, position, buffer, false)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn set_marker_restricted (
  marker: Lisp_Object,
  position: Lisp_Object,
  buffer: Lisp_Object,
) -> Lisp_Object
{
  set_marker_internal (marker, position, buffer, true)
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn set_marker_both (
  marker: Lisp_Object,
  buffer: Lisp_Object,
  charpos: ptrdiff_t,
  bytepos: ptrdiff_t,
) -> Lisp_Object
{
  let b = live_buffer (buffer);
  CHECK_MARKER (marker);
  let m = XMARKER (marker);
  // SAFETY: `m` verified `MARKERP`; `b` checked non-null live buffer before `attach_marker`.
  unsafe {
    if !b.is_null () {
      attach_marker (m, b, charpos, bytepos);
    } else {
      unchain_marker (m);
    }
  }
  marker
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn set_marker_restricted_both (
  marker: Lisp_Object,
  buffer: Lisp_Object,
  charpos: ptrdiff_t,
  bytepos: ptrdiff_t,
) -> Lisp_Object
{
  let b = live_buffer (buffer);
  CHECK_MARKER (marker);
  let m = XMARKER (marker);
  // SAFETY: `m` verified `MARKERP`; `b` checked non-null live buffer before `attach_marker`.
  unsafe {
    if !b.is_null () {
      attach_marker (
        m,
        b,
        clip_to_bounds (BUF_BEGV (b), charpos, BUF_ZV (b)),
        clip_to_bounds (BUF_BEGV_BYTE (b), bytepos, BUF_ZV_BYTE (b)),
      );
    } else {
      unchain_marker (m);
    }
  }
  marker
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn detach_marker (marker: Lisp_Object)
{
  crate::safe_marker::detach_marker_impl (marker);
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn unchain_marker (marker: *mut Lisp_Marker)
{
  // SAFETY: caller supplies a valid `Lisp_Marker *`; unlinks from live buffer's intrusive marker list.
  unsafe {
    let b: *mut buffer = (*marker).buffer as *mut buffer;
    if !b.is_null () {
      eassert! (BUFFER_LIVE_P (b));
      (*marker).buffer = core::ptr::null_mut ();
      let mut prev: *mut *mut Lisp_Marker = &raw mut BUF_MARKERS! (b);
      let mut tail: *mut Lisp_Marker = BUF_MARKERS! (b);
      while !tail.is_null () {
        if marker == tail {
          if *prev == BUF_MARKERS! (b) && !(*tail).next.is_null () {
            let next_buffer: *mut buffer = (*((*tail).next)).buffer as *mut buffer;
            if (*b).text != (*next_buffer).text {
              emacs_abort ();
            }
          }
          *prev = (*tail).next;
          break;
        }
        prev = &raw mut (*tail).next;
        tail = *prev;
      }
      eassert! (!tail.is_null ());
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn marker_position (marker: Lisp_Object) -> ptrdiff_t
{
  let m = XMARKER (marker);
  // SAFETY: `XMARKER(marker)` asserts `MARKERP(marker)`.
  unsafe {
    let buf: *mut buffer = (*m).buffer as *mut buffer;
    if buf.is_null () {
      error (c"Marker does not point anywhere".as_ptr ());
    }
    eassert! (BUF_BEG (buf) <= (*m).charpos && (*m).charpos <= BUF_Z! (buf));
    (*m).charpos
  }
}

#[inline(always)]
pub fn marker_byte_position_safe (marker: Lisp_Object) -> ptrdiff_t
{
  marker_byte_position (marker)
}

#[unsafe(no_mangle)]
pub extern "C-unwind" fn marker_byte_position (marker: Lisp_Object) -> ptrdiff_t
{
  let m = XMARKER (marker);
  // SAFETY: `XMARKER(marker)` asserts `MARKERP(marker)`.
  unsafe {
    let buf: *mut buffer = (*m).buffer as *mut buffer;
    if buf.is_null () {
      error (c"Marker does not point anywhere".as_ptr ());
    }
    eassert! (BUF_BEG_BYTE (buf) <= (*m).bytepos && (*m).bytepos <= BUF_Z_BYTE! (buf));
    (*m).bytepos
  }
}

DEFUN! ("copy-marker", Fcopy_marker, Scopy_marker, 0, 2, 0,
       doc: "Return a new marker pointing at the same place as MARKER.\nIf argument is a number, makes a new marker pointing\nat that position in the current buffer.\nIf MARKER is not specified, the new marker does not point anywhere.\nThe optional argument TYPE specifies the insertion type of the new marker;\nsee `marker-insertion-type'.  ",
       (marker: Lisp_Object, r#type: Lisp_Object),
       {
  crate::safe_marker::copy_marker (marker, r#type)
});

DEFUN! ("marker-insertion-type", Fmarker_insertion_type,
	Smarker_insertion_type, 1, 1, 0,
       doc: "Return insertion type of MARKER: t if it stays after inserted text.\nThe value nil means the marker stays before text inserted there.  ",
       (marker: Lisp_Object),
       {
  crate::safe_marker::marker_insertion_type (marker)
});

DEFUN! ("set-marker-insertion-type", Fset_marker_insertion_type,
	Sset_marker_insertion_type, 2, 2, 0,
       doc: "Set the insertion-type of MARKER to TYPE.\nIf TYPE is t, it means the marker advances when you insert text at it.\nIf TYPE is nil, it means the marker stays behind when you insert text at it.  ",
       (marker: Lisp_Object, r#type: Lisp_Object),
       {
  crate::safe_marker::set_marker_insertion_type (marker, r#type)
});

DEFUN! ("markers-in", Fmarkers_in, Smarkers_in, 0, 2, 0,
       doc: "Return the list of markers in region BEG..END.\nThe list includes markers at BEG and at END.  ",
       (beg: Lisp_Object, end: Lisp_Object),
       {
  crate::safe_marker::markers_in (beg, end)
});

#[unsafe(no_mangle)]
pub extern "C-unwind" fn syms_of_marker ()
{
  defsubr_global (&Smarker_position);
  defsubr_global (&Smarker_last_position);
  defsubr_global (&Smarker_buffer);
  defsubr_global (&Sset_marker);
  defsubr_global (&Scopy_marker);
  defsubr_global (&Smarker_insertion_type);
  defsubr_global (&Sset_marker_insertion_type);
  defsubr_global (&Smarkers_in);
}
