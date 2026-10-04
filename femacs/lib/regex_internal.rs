// femacs: port of lib/regex_internal.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  The
// reference is *not* in `docs/baseline/config/cond-eval-verified.txt`, so the gate reads
// every `#if` arm as live (P-08's blind default); the `_LIBC` glibc arms are kept as text
// in `#[cfg (feature = "libc")]` blocks (P-51) and their bodies name glibc internals the
// frozen target has no counterpart for, which `docs/ports/regex_internal.md` records.
//
// C's `static` forward declarations (regex_internal.c 21-38) have no Rust counterpart
// (E0428, PORTING.md 0.1-12), so they are dropped and the definitions stay in C's order.
//
// The three structs whose C bitfields the functions read are packed into `bits`
// (PORTING.md 1.3); the mirror's `type_`/`set_constraint_`/… accessors are the port's
// spelling of each field, and `docs/ports/token-aliases.tsv` carries the families.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr::null_mut;

include!("include/regex_internal.rs");

/* C: `__glibc_unlikely`/`__glibc_likely` are branch hints (PORTING.md 2.6); the port keeps
   the call shape so the token is visible, with the same meaning as C's macro.  */
#[inline(always)]
fn __glibc_unlikely<T> (cond: T) -> T { cond }
#[inline(always)]
fn __glibc_likely<T> (cond: T) -> T { cond }

/* Functions for string operation.  */

/* This function allocate the buffers.  It is necessary to call
   re_string_reconstruct before using the object.  */

#[inline(never)]
unsafe fn re_string_allocate (pstr: *mut re_string_t, str_: *const c_char, len: Idx,
                              mut init_len: Idx, trans: RE_TRANSLATE_TYPE, icase: bool,
                              dfa: *const re_dfa_t) -> reg_errcode_t
{
  let mut ret: reg_errcode_t;
  let init_buf_len: Idx;

  /* Ensure at least one character fits into the buffers.  */
  if init_len < (*dfa).mb_cur_max as Idx
    { init_len = (*dfa).mb_cur_max as Idx; }
  init_buf_len = if len + 1 < init_len { len + 1 } else { init_len };
  re_string_construct_common (str_, len, pstr, trans, icase, dfa);

  ret = re_string_realloc_buffers (pstr, init_buf_len);
  if __glibc_unlikely (ret != REG_NOERROR)
    { return ret; }

  (*pstr).word_char = (*dfa).word_char.as_ptr ();
  (*pstr).word_ops_used = word_ops_used_ (dfa) as u8;
  (*pstr).mbs = if (*pstr).mbs_allocated != 0 { (*pstr).mbs } else { str_ as *mut u8 };
  (*pstr).valid_len = if (*pstr).mbs_allocated != 0 || (*dfa).mb_cur_max > 1 { 0 } else { len };
  (*pstr).valid_raw_len = (*pstr).valid_len;
  return REG_NOERROR;
}

/* This function allocate the buffers, and initialize them.  */

#[inline(never)]
unsafe fn re_string_construct (pstr: *mut re_string_t, str_: *const c_char, len: Idx,
                               trans: RE_TRANSLATE_TYPE, icase: bool,
                               dfa: *const re_dfa_t) -> reg_errcode_t
{
  let mut ret: reg_errcode_t;
  memset (pstr as *mut c_void, 0, std::mem::size_of::<re_string_t> ());
  re_string_construct_common (str_, len, pstr, trans, icase, dfa);

  if len > 0
    {
      ret = re_string_realloc_buffers (pstr, len + 1);
      if __glibc_unlikely (ret != REG_NOERROR)
	{ return ret; }
    }
  (*pstr).mbs = if (*pstr).mbs_allocated != 0 { (*pstr).mbs } else { str_ as *mut u8 };

  if icase
    {
      if (*dfa).mb_cur_max > 1
	{
	  loop
	    {
	      ret = build_wcs_upper_buffer (pstr);
	      if __glibc_unlikely (ret != REG_NOERROR)
		{ return ret; }
	      if (*pstr).valid_raw_len >= len
		{ break; }
	      if (*pstr).bufs_len > (*pstr).valid_len + (*dfa).mb_cur_max as Idx
		{ break; }
	      ret = re_string_realloc_buffers (pstr, (*pstr).bufs_len * 2);
	      if __glibc_unlikely (ret != REG_NOERROR)
		{ return ret; }
	    }
	}
      else
	{ build_upper_buffer (pstr); }
    }
  else
    {
      if (*dfa).mb_cur_max > 1
	{ build_wcs_buffer (pstr); }
      else
	{
	  if !trans.is_null ()
	    { re_string_translate_buffer (pstr); }
	  else
	    {
	      (*pstr).valid_len = (*pstr).bufs_len;
	      (*pstr).valid_raw_len = (*pstr).bufs_len;
	    }
	}
    }

  return REG_NOERROR;
}

/* Helper functions for re_string_allocate, and re_string_construct.  */

#[inline(never)]
unsafe fn re_string_realloc_buffers (pstr: *mut re_string_t, new_buf_len: Idx)
                                     -> reg_errcode_t
{
  if (*pstr).mb_cur_max > 1
    {
      let new_wcs: *mut wint_t;

      /* Avoid overflow in realloc.  */
      let max_object_size: usize =
          MAX! (std::mem::size_of::<wint_t> (), std::mem::size_of::<Idx> ());
      if __glibc_unlikely (MIN! (IDX_MAX, (SIZE_MAX / max_object_size) as Idx) < new_buf_len)
	{ return REG_ESPACE; }

      new_wcs = re_realloc! ((*pstr).wcs, wint_t, new_buf_len);
      if __glibc_unlikely (new_wcs.is_null ())
	{ return REG_ESPACE; }
      (*pstr).wcs = new_wcs;
      if !(*pstr).offsets.is_null ()
	{
	  let new_offsets: *mut Idx = re_realloc! ((*pstr).offsets, Idx, new_buf_len);
	  if __glibc_unlikely (new_offsets.is_null ())
	    { return REG_ESPACE; }
	  (*pstr).offsets = new_offsets;
	}
    }
  if (*pstr).mbs_allocated != 0
    {
      let new_mbs: *mut u8 = re_realloc! ((*pstr).mbs, u8, new_buf_len);
      if __glibc_unlikely (new_mbs.is_null ())
	{ return REG_ESPACE; }
      (*pstr).mbs = new_mbs;
    }
  (*pstr).bufs_len = new_buf_len;
  return REG_NOERROR;
}

unsafe fn re_string_construct_common (str_: *const c_char, len: Idx, pstr: *mut re_string_t,
                                      trans: RE_TRANSLATE_TYPE, icase: bool,
                                      dfa: *const re_dfa_t)
{
  (*pstr).raw_mbs = str_ as *const u8;
  (*pstr).len = len;
  (*pstr).raw_len = len;
  (*pstr).trans = trans;
  (*pstr).icase = icase as u8;
  (*pstr).mbs_allocated = (!trans.is_null () || icase) as u8;
  (*pstr).mb_cur_max = (*dfa).mb_cur_max;
  (*pstr).is_utf8 = is_utf8_ (dfa) as u8;
  (*pstr).map_notascii = map_notascii_ (dfa) as u8;
  (*pstr).stop = (*pstr).len;
  (*pstr).raw_stop = (*pstr).stop;
}

/* Build wide character buffer PSTR->WCS. ...  */

unsafe fn build_wcs_buffer (pstr: *mut re_string_t)
{
  // C: `#ifdef _LIBC unsigned char buf[MB_LEN_MAX]; ... #else unsigned char buf[64]; #endif`
  // -- the frozen target takes the 64-byte arm; the _LIBC arm is kept as text (P-51).
  #[cfg(feature = "libc")]
  { let buf: [u8; MB_LEN_MAX! ()] = [0; MB_LEN_MAX! ()]; DEBUG_ASSERT! (MB_LEN_MAX! () as c_int >= (*pstr).mb_cur_max); let _ = buf; }
  let mut buf: [u8; 64] = [0; 64];
  let mut prev_st: mbstate_t;
  let mut byte_idx: Idx;
  let mut end_idx: Idx;
  let mut remain_len: Idx;
  let mut mbclen: usize;

  /* Build the buffers from pstr->valid_len to either pstr->len or
     pstr->bufs_len.  */
  end_idx = if (*pstr).bufs_len > (*pstr).len { (*pstr).len } else { (*pstr).bufs_len };
  byte_idx = (*pstr).valid_len;
  while byte_idx < end_idx
    {
      let mut wc: wchar_t = 0;
      let p: *const c_char;

      remain_len = end_idx - byte_idx;
      prev_st = (*pstr).cur_state;
      /* Apply the translation if we need.  */
      if __glibc_unlikely (!(*pstr).trans.is_null ())
	{
	  let mut i: c_int;
	  let mut ch: c_int;

	  i = 0;
	  while i < (*pstr).mb_cur_max && i < remain_len as c_int
	    {
	      ch = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + byte_idx + i as Idx) as usize) as c_int;
	      buf[i as usize] = *(*pstr).trans.add (ch as usize);
	      *(*pstr).mbs.add ((byte_idx + i as Idx) as usize) = *(*pstr).trans.add (ch as usize);
	      i += 1;
	    }
	  p = buf.as_ptr () as *const c_char;
	}
      else
	{ p = (*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + byte_idx) as usize) as *const c_char; }
      mbclen = __mbrtowc! (&mut wc, p, remain_len as usize, &mut (*pstr).cur_state);
      if __glibc_unlikely (mbclen == 1usize.wrapping_neg () || mbclen == 0
			    || (mbclen == 2usize.wrapping_neg ()
				&& (*pstr).bufs_len >= (*pstr).len))
	{
	  /* We treat these cases as a singlebyte character.  */
	  mbclen = 1;
	  wc = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + byte_idx) as usize) as wchar_t;
	  if __glibc_unlikely (!(*pstr).trans.is_null ())
	    { wc = *(*pstr).trans.add (wc as usize) as wchar_t; }
	  (*pstr).cur_state = prev_st;
	}
      else if __glibc_unlikely (mbclen == 2usize.wrapping_neg ())
	{
	  /* The buffer doesn't have enough space, finish to build.  */
	  (*pstr).cur_state = prev_st;
	  break;
	}

      /* Write wide character and padding.  */
      *(*pstr).wcs.add (byte_idx as usize) = wc as wint_t;
      byte_idx += 1;
      /* Write paddings.  */
      remain_len = byte_idx + mbclen as Idx - 1;
      while byte_idx < remain_len
	{
	  *(*pstr).wcs.add (byte_idx as usize) = WEOF;
	  byte_idx += 1;
	}
    }
  (*pstr).valid_len = byte_idx;
  (*pstr).valid_raw_len = byte_idx;
}

/* Build wide character buffer PSTR->WCS like build_wcs_buffer,
   but for REG_ICASE.  */

#[inline(never)]
unsafe fn build_wcs_upper_buffer (pstr: *mut re_string_t) -> reg_errcode_t
{
  let mut prev_st: mbstate_t;
  let mut src_idx: Idx;
  let mut byte_idx: Idx;
  let mut end_idx: Idx;
  let mut remain_len: Idx;
  let mut mbclen: usize;
  // C: `#ifdef _LIBC char buf[MB_LEN_MAX]; ... #else char buf[64]; #endif` -- the frozen
  // target takes the 64-byte arm (P-51 keeps the glibc arm as text).
  #[cfg(feature = "libc")]
  { let buf: [c_char; MB_LEN_MAX! ()] = [0; MB_LEN_MAX! ()]; DEBUG_ASSERT! ((*pstr).mb_cur_max <= MB_LEN_MAX! () as c_int); let _ = buf; }
  let mut buf: [c_char; 64] = [0; 64];

  byte_idx = (*pstr).valid_len;
  end_idx = if (*pstr).bufs_len > (*pstr).len { (*pstr).len } else { (*pstr).bufs_len };

  /* The following optimization assumes that ASCII characters can be
     mapped to wide characters with a simple cast.  */
  // C's `goto offsets_needed` (regex_internal.c 366) jumps into the middle of the other
  // arm of the same `if`: the port keeps one loop whose first half is the fast path and
  // whose second half is the `offsets_needed:` label (PORTING.md 2.2).
  let mut offsets_path: bool =
      (*pstr).map_notascii != 0 || !(*pstr).trans.is_null () || (*pstr).offsets_needed != 0;
  src_idx = (*pstr).valid_raw_len;
  while byte_idx < end_idx
    {
      let mut wc: wchar_t = 0;
      let p: *const c_char;
      if !offsets_path
	{
	  let mut take_slow = false;
	  let ch: u8 = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + byte_idx) as usize);

	  if isascii! (ch as c_int) && mbsinit (&(*pstr).cur_state) != 0
	    {
	      /* The next step uses the assumption that wchar_t is encoded
		 ASCII-safe: all ASCII values can be converted like this.  */
	      let wcu: wchar_t = __towupper! (ch as wint_t) as wchar_t;
	      if isascii! (wcu as c_int)
		{
		  *(*pstr).mbs.add (byte_idx as usize) = wcu as u8;
		  *(*pstr).wcs.add (byte_idx as usize) = wcu as wint_t;
		  byte_idx += 1;
		  continue;
		}
	    }

	  remain_len = end_idx - byte_idx;
	  prev_st = (*pstr).cur_state;
	  mbclen = __mbrtowc! (&mut wc,
			      (*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + byte_idx) as usize)
			      as *const c_char, remain_len as usize, &mut (*pstr).cur_state);
	  if __glibc_likely (0 < mbclen && mbclen < 2usize.wrapping_neg ())
	    {
	      let wcu: wchar_t = __towupper! (wc as wint_t) as wchar_t;
	      if wcu != wc
		{
		  let mbcdlen: usize;

		  mbcdlen = __wcrtomb! (buf.as_mut_ptr (), wcu, &mut prev_st);
		  if __glibc_likely (mbclen == mbcdlen)
		    { memcpy ((*pstr).mbs.add (byte_idx as usize) as *mut c_void,
			      buf.as_ptr () as *const c_void, mbclen); }
		  else
		    {
		      src_idx = byte_idx;
		      take_slow = true;
		    }
		}
	      else
		{ memcpy ((*pstr).mbs.add (byte_idx as usize) as *mut c_void,
			  (*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + byte_idx) as usize)
			  as *const c_void, mbclen); }
	      if !take_slow
		{
		  *(*pstr).wcs.add (byte_idx as usize) = wcu as wint_t;
		  byte_idx += 1;
		  /* Write paddings.  */
		  remain_len = byte_idx + mbclen as Idx - 1;
		  while byte_idx < remain_len
		    {
		      *(*pstr).wcs.add (byte_idx as usize) = WEOF;
		      byte_idx += 1;
		    }
		  continue;
		}
	    }
	  else if mbclen == 1usize.wrapping_neg () || mbclen == 0
		   || (mbclen == 2usize.wrapping_neg () && (*pstr).bufs_len >= (*pstr).len)
	    {
	      /* It is an invalid character, an incomplete character
		 at the end of the string, or '\0'.  Just use the byte.  */
	      *(*pstr).mbs.add (byte_idx as usize) = ch;
	      /* And also cast it to wide char.  */
	      *(*pstr).wcs.add (byte_idx as usize) = ch as wint_t;
	      byte_idx += 1;
	      if __glibc_unlikely (mbclen == 1usize.wrapping_neg ())
		{ (*pstr).cur_state = prev_st; }
	      continue;
	    }
	  else
	    {
	      /* The buffer doesn't have enough space, finish to build.  */
	      (*pstr).cur_state = prev_st;
	      break;
	    }
	  if !take_slow
	    { continue; }
	}
      // offsets_needed:
      {
	remain_len = end_idx - byte_idx;
	prev_st = (*pstr).cur_state;
	if __glibc_unlikely (!(*pstr).trans.is_null ())
	  {
	    let mut i: c_int;
	    let mut ch: c_int;

	    i = 0;
	    while i < (*pstr).mb_cur_max && i < remain_len as c_int
	      {
		ch = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + src_idx + i as Idx) as usize) as c_int;
		buf[i as usize] = *(*pstr).trans.add (ch as usize) as c_char;
		i += 1;
	      }
	    p = buf.as_ptr () as *const c_char;
	  }
	else
	  { p = (*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + src_idx) as usize) as *const c_char; }
	mbclen = __mbrtowc! (&mut wc, p, remain_len as usize, &mut (*pstr).cur_state);
	if __glibc_likely (0 < mbclen && mbclen < 2usize.wrapping_neg ())
	  {
	    let wcu: wchar_t = __towupper! (wc as wint_t) as wchar_t;
	    if wcu != wc
	      {
		let mbcdlen: usize;

		mbcdlen = __wcrtomb! (buf.as_mut_ptr (), wcu, &mut prev_st);
		if __glibc_likely (mbclen == mbcdlen)
		  { memcpy ((*pstr).mbs.add (byte_idx as usize) as *mut c_void,
			    buf.as_ptr () as *const c_void, mbclen); }
		else if mbcdlen != 1usize.wrapping_neg ()
		  {
		    let mut i: usize;

		    if byte_idx + mbcdlen as Idx > (*pstr).bufs_len
		      {
			(*pstr).cur_state = prev_st;
			break;
		      }

		    if (*pstr).offsets.is_null ()
		      {
			(*pstr).offsets = re_malloc! (Idx, (*pstr).bufs_len);

			if (*pstr).offsets.is_null ()
			  { return REG_ESPACE; }
		      }
		    if (*pstr).offsets_needed == 0
		      {
			i = 0;
			while i < byte_idx as usize
			  {
			    *(*pstr).offsets.add (i) = i as Idx;
			    i += 1;
			  }
			(*pstr).offsets_needed = 1;
		      }

		    memcpy ((*pstr).mbs.add (byte_idx as usize) as *mut c_void,
			    buf.as_ptr () as *const c_void, mbcdlen);
		    *(*pstr).wcs.add (byte_idx as usize) = wcu as wint_t;
		    *(*pstr).offsets.add (byte_idx as usize) = src_idx;
		    i = 1;
		    while i < mbcdlen
		      {
			*(*pstr).offsets.add ((byte_idx + i as Idx) as usize)
			  = src_idx + (if i < mbclen { i as Idx } else { mbclen as Idx - 1 });
			*(*pstr).wcs.add ((byte_idx + i as Idx) as usize) = WEOF;
			i += 1;
		      }
		    (*pstr).len += mbcdlen as Idx - mbclen as Idx;
		    if (*pstr).raw_stop > src_idx
		      { (*pstr).stop += mbcdlen as Idx - mbclen as Idx; }
		    end_idx = if (*pstr).bufs_len > (*pstr).len
			      { (*pstr).len } else { (*pstr).bufs_len };
		    byte_idx += mbcdlen as Idx;
		    src_idx += mbclen as Idx;
		    continue;
		  }
		else
		  { memcpy ((*pstr).mbs.add (byte_idx as usize) as *mut c_void,
			    p as *const c_void, mbclen); }
	      }
	    else
	      { memcpy ((*pstr).mbs.add (byte_idx as usize) as *mut c_void,
			p as *const c_void, mbclen); }

	    if __glibc_unlikely ((*pstr).offsets_needed != 0)
	      {
		let mut i: usize;
		i = 0;
		while i < mbclen
		  {
		    *(*pstr).offsets.add ((byte_idx + i as Idx) as usize) = src_idx + i as Idx;
		    i += 1;
		  }
	      }
	    src_idx += mbclen as Idx;

	    *(*pstr).wcs.add (byte_idx as usize) = wcu as wint_t;
	    byte_idx += 1;
	    /* Write paddings.  */
	    remain_len = byte_idx + mbclen as Idx - 1;
	    while byte_idx < remain_len
	      {
		*(*pstr).wcs.add (byte_idx as usize) = WEOF;
		byte_idx += 1;
	      }
	  }
	else if mbclen == 1usize.wrapping_neg () || mbclen == 0
		 || (mbclen == 2usize.wrapping_neg () && (*pstr).bufs_len >= (*pstr).len)
	  {
	    /* It is an invalid character or '\0'.  Just use the byte.  */
	    let mut ch: c_int = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + src_idx) as usize) as c_int;

	    if __glibc_unlikely (!(*pstr).trans.is_null ())
	      { ch = *(*pstr).trans.add (ch as usize) as c_int; }
	    *(*pstr).mbs.add (byte_idx as usize) = ch as u8;

	    if __glibc_unlikely ((*pstr).offsets_needed != 0)
	      { *(*pstr).offsets.add (byte_idx as usize) = src_idx; }
	    src_idx += 1;

	    /* And also convert it to wide char.  */
	    *(*pstr).wcs.add (byte_idx as usize) = ch as wint_t;
	    byte_idx += 1;
	    if __glibc_unlikely (mbclen == 1usize.wrapping_neg ())
	      { (*pstr).cur_state = prev_st; }
	  }
	else
	  {
	    /* The buffer doesn't have enough space, finish to build.  */
	    (*pstr).cur_state = prev_st;
	    break;
	  }
      }
    }
  if !offsets_path
    {
      (*pstr).valid_len = byte_idx;
      (*pstr).valid_raw_len = byte_idx;
      return REG_NOERROR;
    }
  (*pstr).valid_len = byte_idx;
  (*pstr).valid_raw_len = src_idx;
  return REG_NOERROR;
}

/* Skip characters until the index becomes greater than NEW_RAW_IDX.
   Return the index.  */

unsafe fn re_string_skip_chars (pstr: *mut re_string_t, new_raw_idx: Idx,
                                last_wc: *mut wint_t) -> Idx
{
  let mut prev_st: mbstate_t;
  let mut rawbuf_idx: Idx;
  let mut mbclen: usize;
  let mut wc: wint_t = WEOF;

  /* Skip the characters which are not necessary to check.  */
  rawbuf_idx = (*pstr).raw_mbs_idx + (*pstr).valid_raw_len;
  while rawbuf_idx < new_raw_idx
    {
      let mut wc2: wchar_t = 0;
      let mut remain_len: Idx = (*pstr).raw_len - rawbuf_idx;
      prev_st = (*pstr).cur_state;
      mbclen = __mbrtowc! (&mut wc2, (*pstr).raw_mbs.add (rawbuf_idx as usize) as *const c_char,
			  remain_len as usize, &mut (*pstr).cur_state);
      if __glibc_unlikely (mbclen == 2usize.wrapping_neg () || mbclen == 1usize.wrapping_neg ()
			    || mbclen == 0)
	{
	  /* We treat these cases as a single byte character.  */
	  if mbclen == 0 || remain_len == 0
	    { wc = b'\0' as wint_t; }
	  else
	    { wc = *(*pstr).raw_mbs.add (rawbuf_idx as usize) as wint_t; }
	  mbclen = 1;
	  (*pstr).cur_state = prev_st;
	}
      else
	{ wc = wc2 as wint_t; }
      /* Then proceed the next character.  */
      rawbuf_idx += mbclen as Idx;
    }
  *last_wc = wc;
  return rawbuf_idx;
}

/* Build the buffer PSTR->MBS, and apply the translation if we need.
   This function is used in case of REG_ICASE.  */

unsafe fn build_upper_buffer (pstr: *mut re_string_t)
{
  let mut char_idx: Idx;
  let end_idx: Idx;
  end_idx = if (*pstr).bufs_len > (*pstr).len { (*pstr).len } else { (*pstr).bufs_len };

  char_idx = (*pstr).valid_len;
  while char_idx < end_idx
    {
      let mut ch: c_int = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + char_idx) as usize) as c_int;
      if __glibc_unlikely (!(*pstr).trans.is_null ())
	{ ch = *(*pstr).trans.add (ch as usize) as c_int; }
      *(*pstr).mbs.add (char_idx as usize) = toupper (ch) as u8;
      char_idx += 1;
    }
  (*pstr).valid_len = char_idx;
  (*pstr).valid_raw_len = char_idx;
}

/* Apply TRANS to the buffer in PSTR.  */

unsafe fn re_string_translate_buffer (pstr: *mut re_string_t)
{
  let mut buf_idx: Idx;
  let end_idx: Idx;
  end_idx = if (*pstr).bufs_len > (*pstr).len { (*pstr).len } else { (*pstr).bufs_len };

  buf_idx = (*pstr).valid_len;
  while buf_idx < end_idx
    {
      let ch: c_int = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + buf_idx) as usize) as c_int;
      *(*pstr).mbs.add (buf_idx as usize) = *(*pstr).trans.add (ch as usize);

      buf_idx += 1;
    }

  (*pstr).valid_len = buf_idx;
  (*pstr).valid_raw_len = buf_idx;
}

/* This function re-construct the buffers.
   Concretely, convert to wide character in case of pstr->mb_cur_max > 1,
   convert to upper case in case of REG_ICASE, apply translation.  */

#[inline(never)]
unsafe fn re_string_reconstruct (pstr: *mut re_string_t, idx: Idx, eflags: c_int)
                                 -> reg_errcode_t
{
  let mut offset: Idx;

  if __glibc_unlikely ((*pstr).raw_mbs_idx <= idx)
    { offset = idx - (*pstr).raw_mbs_idx; }
  else
    {
      /* Reset buffer.  */
      if (*pstr).mb_cur_max > 1
	{ memset (&mut (*pstr).cur_state as *mut mbstate_t as *mut c_void, 0,
	          std::mem::size_of::<mbstate_t> ()); }
      (*pstr).len = (*pstr).raw_len;
      (*pstr).stop = (*pstr).raw_stop;
      (*pstr).valid_len = 0;
      (*pstr).raw_mbs_idx = 0;
      (*pstr).valid_raw_len = 0;
      (*pstr).offsets_needed = 0;
      (*pstr).tip_context = if eflags & REG_NOTBOL != 0 { CONTEXT_BEGBUF as c_uint }
			   else { (CONTEXT_NEWLINE | CONTEXT_BEGBUF) as c_uint };
      if (*pstr).mbs_allocated == 0
	{ (*pstr).mbs = (*pstr).raw_mbs as *mut u8; }
      offset = idx;
    }

  if __glibc_likely (offset != 0)
    {
      /* Should the already checked characters be kept?  */
      if __glibc_likely (offset < (*pstr).valid_raw_len)
	{
	  /* Yes, move them to the front of the buffer.  */
	  if __glibc_unlikely ((*pstr).offsets_needed != 0)
	    {
	      let mut low: Idx = 0;
	      let mut high: Idx = (*pstr).valid_len;
	      let mut mid: Idx;
	      loop
		{
		  mid = (high + low) / 2;
		  if *(*pstr).offsets.add (mid as usize) > offset
		    { high = mid; }
		  else if *(*pstr).offsets.add (mid as usize) < offset
		    { low = mid + 1; }
		  else
		    { break; }
		  if !(low < high) { break; }
		}
	      if *(*pstr).offsets.add (mid as usize) < offset
		{ mid += 1; }
	      (*pstr).tip_context = re_string_context_at (pstr, mid - 1, eflags);
	      /* This can be quite complicated, so handle specially
		 only the common and easy case where the character with
		 different length representation of lower and upper
		 case is present at or after offset.  */
	      if (*pstr).valid_len > offset
		  && mid == offset && *(*pstr).offsets.add (mid as usize) == offset
		{
		  memmove ((*pstr).wcs as *mut c_void,
			   (*pstr).wcs.add (offset as usize) as *const c_void,
			   ((*pstr).valid_len - offset) as usize * std::mem::size_of::<wint_t> ());
		  memmove ((*pstr).mbs as *mut c_void,
			   (*pstr).mbs.add (offset as usize) as *const c_void,
			   ((*pstr).valid_len - offset) as usize);
		  (*pstr).valid_len -= offset;
		  (*pstr).valid_raw_len -= offset;
		  low = 0;
		  while low < (*pstr).valid_len
		    {
		      *(*pstr).offsets.add (low as usize) =
			  *(*pstr).offsets.add ((low + offset) as usize) - offset;
		      low += 1;
		    }
		}
	      else
		{
		  /* Otherwise, just find out how long the partial multibyte
		     character at offset is and fill it with WEOF/255.  */
		  (*pstr).len = (*pstr).raw_len - idx + offset;
		  (*pstr).stop = (*pstr).raw_stop - idx + offset;
		  (*pstr).offsets_needed = 0;
		  while mid > 0 && *(*pstr).offsets.add ((mid - 1) as usize) == offset
		    { mid -= 1; }
		  while mid < (*pstr).valid_len
		    {
		      if *(*pstr).wcs.add (mid as usize) != WEOF
			{ break; }
		      else
			{ mid += 1; }
		    }
		  if mid == (*pstr).valid_len
		    { (*pstr).valid_len = 0; }
		  else
		    {
		      (*pstr).valid_len = *(*pstr).offsets.add (mid as usize) - offset;
		      if (*pstr).valid_len != 0
			{
			  low = 0;
			  while low < (*pstr).valid_len
			    {
			      *(*pstr).wcs.add (low as usize) = WEOF;
			      low += 1;
			    }
			  memset ((*pstr).mbs as *mut c_void, 255, (*pstr).valid_len as usize);
			}
		    }
		  (*pstr).valid_raw_len = (*pstr).valid_len;
		}
	    }
	  else
	    {
	      (*pstr).tip_context = re_string_context_at (pstr, offset - 1, eflags);
	      if (*pstr).mb_cur_max > 1
		{ memmove ((*pstr).wcs as *mut c_void,
			   (*pstr).wcs.add (offset as usize) as *const c_void,
			   ((*pstr).valid_len - offset) as usize * std::mem::size_of::<wint_t> ()); }
	      if __glibc_unlikely ((*pstr).mbs_allocated != 0)
		{ memmove ((*pstr).mbs as *mut c_void,
			   (*pstr).mbs.add (offset as usize) as *const c_void,
			   ((*pstr).valid_len - offset) as usize); }
	      (*pstr).valid_len -= offset;
	      (*pstr).valid_raw_len -= offset;
	      DEBUG_ASSERT! ((*pstr).valid_len > 0);
	    }
	}
      else
	{
	  /* No, skip all characters until IDX.  */
	  let prev_valid_len: Idx = (*pstr).valid_len;

	  if __glibc_unlikely ((*pstr).offsets_needed != 0)
	    {
	      (*pstr).len = (*pstr).raw_len - idx + offset;
	      (*pstr).stop = (*pstr).raw_stop - idx + offset;
	      (*pstr).offsets_needed = 0;
	    }
	  (*pstr).valid_len = 0;
	  if (*pstr).mb_cur_max > 1
	    {
	      let mut wcs_idx: Idx;
	      let mut wc: wint_t = WEOF;

	      if (*pstr).is_utf8 != 0
		{
		  let raw: *const u8;
		  let mut p: *const u8;
		  let mut end: *const u8;

		  /* Special case UTF-8.  Multi-byte chars start with any
		     byte other than 0x80 - 0xbf.  */
		  raw = (*pstr).raw_mbs.add ((*pstr).raw_mbs_idx as usize);
		  end = raw.add ((offset - (*pstr).mb_cur_max as Idx) as usize);
		  if end < (*pstr).raw_mbs
		    { end = (*pstr).raw_mbs; }
		  p = raw.add ((offset - 1) as usize);
		  // C: `#ifdef _LIBC` (regex_internal.c 706-720) -- the glibc arm is kept as
		  // text (P-51); C glues it to the live arm (`… else for (…);`), which Rust
		  // cannot spell across a `#[cfg]`, so the dead `if` stands alone and the
		  // live `while` follows (the `else`/`for` tokens are MACRO_TOKEN_SKIP).
		  #[cfg(feature = "libc")]
		  {
		    if isascii! (*p as c_int) != 0 && __glibc_likely ((*pstr).trans.is_null ())
		      {
			memset (&mut (*pstr).cur_state as *mut mbstate_t as *mut c_void, 0,
				std::mem::size_of::<mbstate_t> ());
			/* pstr->valid_len = 0; */
			wc = *p as wint_t;
		      }
		  }
		    {
		      while p >= end
			{
			  if (*p & 0xc0) != 0x80
			    {
			      let mut cur_state: mbstate_t = mbstate_t { count: 0, value: [0; 4] };
			      let mut wc2: wchar_t = 0;
			      let mlen: Idx = (*pstr).len - (p.offset_from (raw)) as Idx;
			      let mut buf: [u8; 6] = [0; 6];
			      let mbclen: usize;

			      let mut pp: *const u8 = p;
			      if __glibc_unlikely (!(*pstr).trans.is_null ())
				{
				  let mut i: Idx = if mlen < 6 { mlen } else { 6 };
				  loop
				    {
				      i -= 1;
				      if i < 0 { break; }
				      buf[i as usize] = *(*pstr).trans.add (*p.add (i as usize) as usize);
				    }
				  pp = buf.as_ptr ();
				}
			      /* XXX Don't use mbrtowc, we know which conversion
				 to use (UTF-8 -> UCS4).  */
			      memset (&mut cur_state as *mut mbstate_t as *mut c_void, 0,
				      std::mem::size_of::<mbstate_t> ());
			      mbclen = __mbrtowc! (&mut wc2, pp as *const c_char, mlen as usize,
						  &mut cur_state);
			      if (offset - p.offset_from (raw) as Idx) <= mbclen as Idx
				  && mbclen < 2usize.wrapping_neg ()
				{
				  memset (&mut (*pstr).cur_state as *mut mbstate_t as *mut c_void, 0,
					  std::mem::size_of::<mbstate_t> ());
				  (*pstr).valid_len = mbclen as Idx - (offset - p.offset_from (raw) as Idx);
				  wc = wc2 as wint_t;
				}
			      break;
			    }
			  p = p.offset (-1);
			}
		    }
		}

	      if wc == WEOF
		{ (*pstr).valid_len = re_string_skip_chars (pstr, idx, &mut wc) - idx; }
	      if wc == WEOF
		{ (*pstr).tip_context = re_string_context_at (pstr, prev_valid_len - 1, eflags); }
	      else
		{ (*pstr).tip_context = (if __glibc_unlikely ((*pstr).word_ops_used != 0)
					 && IS_WIDE_WORD_CHAR! (wc)
					 { CONTEXT_WORD }
					 else if IS_WIDE_NEWLINE! (wc)
					 && (*pstr).newline_anchor != 0
					 { CONTEXT_NEWLINE } else { 0 }) as c_uint; }
	      if __glibc_unlikely ((*pstr).valid_len != 0)
		{
		  wcs_idx = 0;
		  while wcs_idx < (*pstr).valid_len
		    {
		      *(*pstr).wcs.add (wcs_idx as usize) = WEOF;
		      wcs_idx += 1;
		    }
		  if (*pstr).mbs_allocated != 0
		    { memset ((*pstr).mbs as *mut c_void, 255, (*pstr).valid_len as usize); }
		}
	      (*pstr).valid_raw_len = (*pstr).valid_len;
	    }
	  else
	    {
	      let mut c: c_int = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + offset - 1) as usize) as c_int;
	      (*pstr).valid_raw_len = 0;
	      if !(*pstr).trans.is_null ()
		{ c = *(*pstr).trans.add (c as usize) as c_int; }
	      (*pstr).tip_context = (if bitset_contain ((*pstr).word_char, c as Idx)
				   { CONTEXT_WORD }
				   else if IS_NEWLINE! (c) && (*pstr).newline_anchor != 0
				   { CONTEXT_NEWLINE } else { 0 }) as c_uint;
	    }
	}
      if (*pstr).mbs_allocated == 0
	{ (*pstr).mbs = (*pstr).mbs.add (offset as usize); }
    }
  (*pstr).raw_mbs_idx = idx;
  (*pstr).len -= offset;
  (*pstr).stop -= offset;

  /* Then build the buffers.  */
  if (*pstr).mb_cur_max > 1
    {
      if (*pstr).icase != 0
	{
	  let ret: reg_errcode_t = build_wcs_upper_buffer (pstr);
	  if __glibc_unlikely (ret != REG_NOERROR)
	    { return ret; }
	}
      else
	{ build_wcs_buffer (pstr); }
    }
  else
    {
      if __glibc_unlikely ((*pstr).mbs_allocated != 0)
	{
	  if (*pstr).icase != 0
	    { build_upper_buffer (pstr); }
	  else if !(*pstr).trans.is_null ()
	    { re_string_translate_buffer (pstr); }
	}
      else
	{ (*pstr).valid_len = (*pstr).len; }
    }

  (*pstr).cur_idx = 0;
  return REG_NOERROR;
}

unsafe fn re_string_peek_byte_case (pstr: *const re_string_t, idx: Idx) -> u8
{
  let mut ch: c_int;
  let mut off: Idx;

  /* Handle the common (easiest) cases first.  */
  if __glibc_likely ((*pstr).mbs_allocated == 0)
    { return re_string_peek_byte! (pstr, idx); }

  if (*pstr).mb_cur_max > 1
      && !re_string_is_single_byte_char! (pstr, (*pstr).cur_idx + idx)
    { return re_string_peek_byte! (pstr, idx); }

  off = (*pstr).cur_idx + idx;
  if (*pstr).offsets_needed != 0
    { off = *(*pstr).offsets.add (off as usize); }

  ch = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + off) as usize) as c_int;

  /* Ensure that e.g. for tr_TR.UTF-8 BACKSLASH DOTLESS SMALL LETTER I
     this function returns CAPITAL LETTER I instead of first byte of
     DOTLESS SMALL LETTER I.  The latter would confuse the parser,
     since peek_byte_case doesn't advance cur_idx in any way.  */
  if (*pstr).offsets_needed != 0 && !isascii! (ch)
    { return re_string_peek_byte! (pstr, idx); }

  return ch as u8;
}

unsafe fn re_string_fetch_byte_case (pstr: *mut re_string_t) -> u8
{
  if __glibc_likely ((*pstr).mbs_allocated == 0)
    { return re_string_fetch_byte! (pstr); }

  if (*pstr).offsets_needed != 0
    {
      let off: Idx;
      let ch: c_int;

      /* For tr_TR.UTF-8 [[:islower:]] there is
	 [[: CAPITAL LETTER I WITH DOT lower:]] in mbs.  Skip
	 in that case the whole multi-byte character and return
	 the original letter.  On the other side, with
	 [[: DOTLESS SMALL LETTER I return [[:I, as doing
	 anything else would complicate things too much.  */

      if !re_string_first_byte! (pstr, (*pstr).cur_idx)
	{ return re_string_fetch_byte! (pstr); }

      off = *(*pstr).offsets.add ((*pstr).cur_idx as usize);
      ch = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + off) as usize) as c_int;

      if !isascii! (ch)
	{ return re_string_fetch_byte! (pstr); }

      re_string_skip_bytes! (pstr, re_string_char_size_at (pstr, (*pstr).cur_idx) as Idx);
      return ch as u8;
    }

  let r = *(*pstr).raw_mbs.add (((*pstr).raw_mbs_idx + (*pstr).cur_idx) as usize);
  (*pstr).cur_idx += 1;
  return r;
}

unsafe fn re_string_destruct (pstr: *mut re_string_t)
{
  re_free! ((*pstr).wcs);
  re_free! ((*pstr).offsets);
  if (*pstr).mbs_allocated != 0
    { re_free! ((*pstr).mbs); }
}

/* Return the context at IDX in INPUT.  */

unsafe fn re_string_context_at (input: *const re_string_t, idx: Idx, eflags: c_int) -> c_uint
{
  let c: c_int;
  if __glibc_unlikely (idx < 0)
    /* In this case, we use the value stored in input->tip_context,
       since we can't know the character in input->mbs[-1] here.  */
    { return (*input).tip_context; }
  if __glibc_unlikely (idx == (*input).len)
    { return if eflags & REG_NOTEOL != 0 { CONTEXT_ENDBUF as c_uint }
	     else { (CONTEXT_NEWLINE | CONTEXT_ENDBUF) as c_uint }; }
  if (*input).mb_cur_max > 1
    {
      let wc: wint_t;
      let mut wc_idx: Idx = idx;
      while *(*input).wcs.add (wc_idx as usize) == WEOF
	{
	  DEBUG_ASSERT! (wc_idx >= 0);
	  wc_idx -= 1;
	  if wc_idx < 0
	    { return (*input).tip_context; }
	}
      wc = *(*input).wcs.add (wc_idx as usize);
      if __glibc_unlikely ((*input).word_ops_used != 0)
	  && IS_WIDE_WORD_CHAR! (wc)
	{ return CONTEXT_WORD as c_uint; }
      return (if IS_WIDE_NEWLINE! (wc) && (*input).newline_anchor != 0
	      { CONTEXT_NEWLINE } else { 0 }) as c_uint;
    }
  else
    {
      c = re_string_byte_at! (input, idx) as c_int;
      if bitset_contain ((*input).word_char, c as Idx)
	{ return CONTEXT_WORD as c_uint; }
      return (if IS_NEWLINE! (c) && (*input).newline_anchor != 0
	      { CONTEXT_NEWLINE } else { 0 }) as c_uint;
    }
}

/* Functions for set operation.  */

#[inline(never)]
unsafe fn re_node_set_alloc (set: *mut re_node_set, size: Idx) -> reg_errcode_t
{
  (*set).alloc = size;
  (*set).nelem = 0;
  (*set).elems = re_malloc! (Idx, size);
  if __glibc_unlikely ((*set).elems.is_null ())
    { return REG_ESPACE; }
  return REG_NOERROR;
}

#[inline(never)]
unsafe fn re_node_set_init_1 (set: *mut re_node_set, elem: Idx) -> reg_errcode_t
{
  (*set).alloc = 1;
  (*set).nelem = 1;
  (*set).elems = re_malloc! (Idx, 1);
  if __glibc_unlikely ((*set).elems.is_null ())
    {
      (*set).alloc = 0;
      (*set).nelem = 0;
      return REG_ESPACE;
    }
  *(*set).elems.add (0) = elem;
  return REG_NOERROR;
}

#[inline(never)]
unsafe fn re_node_set_init_2 (set: *mut re_node_set, elem1: Idx, elem2: Idx) -> reg_errcode_t
{
  (*set).alloc = 2;
  (*set).elems = re_malloc! (Idx, 2);
  if __glibc_unlikely ((*set).elems.is_null ())
    { return REG_ESPACE; }
  if elem1 == elem2
    {
      (*set).nelem = 1;
      *(*set).elems.add (0) = elem1;
    }
  else
    {
      (*set).nelem = 2;
      if elem1 < elem2
	{
	  *(*set).elems.add (0) = elem1;
	  *(*set).elems.add (1) = elem2;
	}
      else
	{
	  *(*set).elems.add (0) = elem2;
	  *(*set).elems.add (1) = elem1;
	}
    }
  return REG_NOERROR;
}

#[inline(never)]
unsafe fn re_node_set_init_copy (dest: *mut re_node_set, src: *const re_node_set)
                                 -> reg_errcode_t
{
  (*dest).nelem = (*src).nelem;
  if (*src).nelem > 0
    {
      (*dest).alloc = (*dest).nelem;
      (*dest).elems = re_malloc! (Idx, (*dest).alloc);
      if __glibc_unlikely ((*dest).elems.is_null ())
	{
	  (*dest).alloc = 0;
	  (*dest).nelem = 0;
	  return REG_ESPACE;
	}
      memcpy ((*dest).elems as *mut c_void, (*src).elems as *const c_void,
	      (*src).nelem as usize * std::mem::size_of::<Idx> ());
    }
  else
    { re_node_set_init_empty! (dest); }
  return REG_NOERROR;
}

/* Calculate the intersection of the sets SRC1 and SRC2. And merge it to
   DEST. Return value indicate the error code or REG_NOERROR if succeeded.
   Note: We assume dest->elems is NULL, when dest->alloc is 0.  */

#[inline(never)]
unsafe fn re_node_set_add_intersect (dest: *mut re_node_set, src1: *const re_node_set,
                                     src2: *const re_node_set) -> reg_errcode_t
{
  let mut i1: Idx;
  let mut i2: Idx;
  let mut is: Idx;
  let mut id: Idx;
  let mut delta: Idx;
  let mut sbase: Idx;
  if (*src1).nelem == 0 || (*src2).nelem == 0
    { return REG_NOERROR; }

  /* We need dest->nelem + 2 * elems_in_intersection; this is a
     conservative estimate.  */
  if (*src1).nelem + (*src2).nelem + (*dest).nelem > (*dest).alloc
    {
      let new_alloc: Idx = (*src1).nelem + (*src2).nelem + (*dest).alloc;
      let new_elems: *mut Idx = re_realloc! ((*dest).elems, Idx, new_alloc);
      if __glibc_unlikely (new_elems.is_null ())
	{ return REG_ESPACE; }
      (*dest).elems = new_elems;
      (*dest).alloc = new_alloc;
    }

  /* Find the items in the intersection of SRC1 and SRC2, and copy
     into the top of DEST those that are not already in DEST itself.  */
  sbase = (*dest).nelem + (*src1).nelem + (*src2).nelem;
  i1 = (*src1).nelem - 1;
  i2 = (*src2).nelem - 1;
  id = (*dest).nelem - 1;
  loop
    {
      if *(*src1).elems.add (i1 as usize) == *(*src2).elems.add (i2 as usize)
	{
	  /* Try to find the item in DEST.  Maybe we could binary search?  */
	  while id >= 0 && *(*dest).elems.add (id as usize) > *(*src1).elems.add (i1 as usize)
	    { id -= 1; }

	  if id < 0 || *(*dest).elems.add (id as usize) != *(*src1).elems.add (i1 as usize)
            {
	      sbase -= 1;
	      *(*dest).elems.add (sbase as usize) = *(*src1).elems.add (i1 as usize);
	    }

	  i1 -= 1;
	  i2 -= 1;
	  if i1 < 0 || i2 < 0
	    { break; }
	}

      /* Lower the highest of the two items.  */
      else if *(*src1).elems.add (i1 as usize) < *(*src2).elems.add (i2 as usize)
	{
	  i2 -= 1;
	  if i2 < 0
	    { break; }
	}
      else
	{
	  i1 -= 1;
	  if i1 < 0
	    { break; }
	}
    }

  id = (*dest).nelem - 1;
  is = (*dest).nelem + (*src1).nelem + (*src2).nelem - 1;
  delta = is - sbase + 1;

  /* Now copy.  When DELTA becomes zero, the remaining
     DEST elements are already in place; this is more or
     less the same loop that is in re_node_set_merge.  */
  (*dest).nelem += delta;
  if delta > 0 && id >= 0
    {
      loop
	{
	  if *(*dest).elems.add (is as usize) > *(*dest).elems.add (id as usize)
	    {
	      /* Copy from the top.  */
	      let d = delta; delta -= 1;
	      *(*dest).elems.add ((id + d) as usize) = *(*dest).elems.add (is as usize);
	      is -= 1;
	      if delta == 0
		{ break; }
	    }
	  else
	    {
	      /* Slide from the bottom.  */
	      *(*dest).elems.add ((id + delta) as usize) = *(*dest).elems.add (id as usize);
	      id -= 1;
	      if id < 0
		{ break; }
	    }
	}
    }

  /* Copy remaining SRC elements.  */
  memcpy ((*dest).elems as *mut c_void, (*dest).elems.add (sbase as usize) as *const c_void,
	  delta as usize * std::mem::size_of::<Idx> ());

  return REG_NOERROR;
}

/* Calculate the union set of the sets SRC1 and SRC2. And store it to
   DEST. Return value indicate the error code or REG_NOERROR if succeeded.  */

#[inline(never)]
unsafe fn re_node_set_init_union (dest: *mut re_node_set, src1: *const re_node_set,
                                  src2: *const re_node_set) -> reg_errcode_t
{
  let mut i1: Idx;
  let mut i2: Idx;
  let mut id: Idx;
  if !src1.is_null () && (*src1).nelem > 0 && !src2.is_null () && (*src2).nelem > 0
    {
      (*dest).alloc = (*src1).nelem + (*src2).nelem;
      (*dest).elems = re_malloc! (Idx, (*dest).alloc);
      if __glibc_unlikely ((*dest).elems.is_null ())
	{ return REG_ESPACE; }
    }
  else
    {
      if !src1.is_null () && (*src1).nelem > 0
	{ return re_node_set_init_copy (dest, src1); }
      else if !src2.is_null () && (*src2).nelem > 0
	{ return re_node_set_init_copy (dest, src2); }
      else
	{ re_node_set_init_empty! (dest); }
      return REG_NOERROR;
    }
  i1 = 0; i2 = 0; id = 0;
  while i1 < (*src1).nelem && i2 < (*src2).nelem
    {
      if *(*src1).elems.add (i1 as usize) > *(*src2).elems.add (i2 as usize)
	{
	  *(*dest).elems.add (id as usize) = *(*src2).elems.add (i2 as usize);
	  id += 1;
	  i2 += 1;
	  continue;
	}
      if *(*src1).elems.add (i1 as usize) == *(*src2).elems.add (i2 as usize)
	{ i2 += 1; }
      *(*dest).elems.add (id as usize) = *(*src1).elems.add (i1 as usize);
      id += 1;
      i1 += 1;
    }
  if i1 < (*src1).nelem
    {
      memcpy ((*dest).elems.add (id as usize) as *mut c_void,
	      (*src1).elems.add (i1 as usize) as *const c_void,
	      ((*src1).nelem - i1) as usize * std::mem::size_of::<Idx> ());
      id += (*src1).nelem - i1;
    }
  else if i2 < (*src2).nelem
    {
      memcpy ((*dest).elems.add (id as usize) as *mut c_void,
	      (*src2).elems.add (i2 as usize) as *const c_void,
	      ((*src2).nelem - i2) as usize * std::mem::size_of::<Idx> ());
      id += (*src2).nelem - i2;
    }
  (*dest).nelem = id;
  return REG_NOERROR;
}

/* Calculate the union set of the sets DEST and SRC. And store it to
   DEST. Return value indicate the error code or REG_NOERROR if succeeded.  */

#[inline(never)]
unsafe fn re_node_set_merge (dest: *mut re_node_set, src: *const re_node_set) -> reg_errcode_t
{
  let mut is: Idx;
  let mut id: Idx;
  let mut sbase: Idx;
  let mut delta: Idx;
  if src.is_null () || (*src).nelem == 0
    { return REG_NOERROR; }
  if (*dest).alloc < 2 * (*src).nelem + (*dest).nelem
    {
      let new_alloc: Idx = 2 * ((*src).nelem + (*dest).alloc);
      let new_buffer: *mut Idx = re_realloc! ((*dest).elems, Idx, new_alloc);
      if __glibc_unlikely (new_buffer.is_null ())
	{ return REG_ESPACE; }
      (*dest).elems = new_buffer;
      (*dest).alloc = new_alloc;
    }

  if __glibc_unlikely ((*dest).nelem == 0)
    {
      /* Although we already guaranteed above that dest->alloc != 0 and
         therefore dest->elems != NULL, add a debug assertion to pacify
         GCC 11.2.1's -fanalyzer.  */
      DEBUG_ASSERT! (!(*dest).elems.is_null ());
      (*dest).nelem = (*src).nelem;
      memcpy ((*dest).elems as *mut c_void, (*src).elems as *const c_void,
	      (*src).nelem as usize * std::mem::size_of::<Idx> ());
      return REG_NOERROR;
    }

  /* Copy into the top of DEST the items of SRC that are not
     found in DEST.  Maybe we could binary search in DEST?  */
  sbase = (*dest).nelem + 2 * (*src).nelem;
  is = (*src).nelem - 1;
  id = (*dest).nelem - 1;
  while is >= 0 && id >= 0
    {
      if *(*dest).elems.add (id as usize) == *(*src).elems.add (is as usize)
	{
	  is -= 1; id -= 1;
	}
      else if *(*dest).elems.add (id as usize) < *(*src).elems.add (is as usize)
	{
	  sbase -= 1;
	  *(*dest).elems.add (sbase as usize) = *(*src).elems.add (is as usize);
	  is -= 1;
	}
      else /* if (dest->elems[id] > src->elems[is]) */
	{ id -= 1; }
    }

  if is >= 0
    {
      /* If DEST is exhausted, the remaining items of SRC must be unique.  */
      sbase -= is + 1;
      memcpy ((*dest).elems.add (sbase as usize) as *mut c_void,
	      (*src).elems as *const c_void, (is + 1) as usize * std::mem::size_of::<Idx> ());
    }

  id = (*dest).nelem - 1;
  is = (*dest).nelem + 2 * (*src).nelem - 1;
  delta = is - sbase + 1;
  if delta == 0
    { return REG_NOERROR; }

  /* Now copy.  When DELTA becomes zero, the remaining
     DEST elements are already in place.  */
  (*dest).nelem += delta;
  loop
    {
      if *(*dest).elems.add (is as usize) > *(*dest).elems.add (id as usize)
	{
	  /* Copy from the top.  */
	  let d = delta; delta -= 1;
	  *(*dest).elems.add ((id + d) as usize) = *(*dest).elems.add (is as usize);
	  is -= 1;
	  if delta == 0
	    { break; }
	}
      else
	{
	  /* Slide from the bottom.  */
	  *(*dest).elems.add ((id + delta) as usize) = *(*dest).elems.add (id as usize);
	  id -= 1;
	  if id < 0
	    {
	      /* Copy remaining SRC elements.  */
	      memcpy ((*dest).elems as *mut c_void,
		      (*dest).elems.add (sbase as usize) as *const c_void,
		      delta as usize * std::mem::size_of::<Idx> ());
	      break;
	    }
	}
    }

  return REG_NOERROR;
}

/* Insert the new element ELEM to the re_node_set* SET.
   SET is not expected to already contain ELEM, but tolerate
   duplicates as a no-op.  Return true if successful.  */

unsafe fn re_node_set_insert (set: *mut re_node_set, elem: Idx) -> bool
{
  let mut idx: Idx;
  /* In case the set is empty.  */
  if (*set).alloc == 0
    { return __glibc_likely (re_node_set_init_1 (set, elem) == REG_NOERROR); }

  if __glibc_unlikely ((*set).nelem == 0)
    {
      /* Although we already guaranteed above that set->alloc != 0 and
         therefore set->elems != NULL, add a debug assertion to pacify
         GCC 11.2 -fanalyzer.  */
      DEBUG_ASSERT! (!(*set).elems.is_null ());
      *(*set).elems.add (0) = elem;
      (*set).nelem += 1;
      return true;
    }

  /* Realloc if we need.  */
  if (*set).alloc == (*set).nelem
    {
      let new_elems: *mut Idx;
      (*set).alloc = (*set).alloc * 2;
      new_elems = re_realloc! ((*set).elems, Idx, (*set).alloc);
      if __glibc_unlikely (new_elems.is_null ())
	{ return false; }
      (*set).elems = new_elems;
    }

  /* Move the elements which follows the new element.  Test the
     first element separately to skip a check in the inner loop.  */
  if elem < *(*set).elems.add (0)
    {
      idx = (*set).nelem;
      while idx > 0
	{
	  *(*set).elems.add (idx as usize) = *(*set).elems.add ((idx - 1) as usize);
	  idx -= 1;
	}
    }
  else
    {
      idx = (*set).nelem;
      while *(*set).elems.add ((idx - 1) as usize) > elem
        {
          *(*set).elems.add (idx as usize) = *(*set).elems.add ((idx - 1) as usize);
          /* Although we already guaranteed that idx is at least 2 here,
             add an assertion to pacify GCC 16.1.1 -Wanalyzer-out-of-bounds
             when _REGEX_AVOID_UCHAR_H is defined.  */
          DEBUG_ASSERT! (1 < idx);
          idx -= 1;
        }
      /* Already in set.  Return early.  */
      if __glibc_unlikely (*(*set).elems.add ((idx - 1) as usize) == elem)
	{ return true; }
    }

  /* Insert the new element.  */
  *(*set).elems.add (idx as usize) = elem;
  (*set).nelem += 1;
  return true;
}

/* Insert the new element ELEM to the re_node_set* SET.
   SET should not already have any element greater than or equal to ELEM.
   Return true if successful.  */

unsafe fn re_node_set_insert_last (set: *mut re_node_set, elem: Idx) -> bool
{
  /* Realloc if we need.  */
  if (*set).alloc == (*set).nelem
    {
      let new_elems: *mut Idx;
      (*set).alloc = ((*set).alloc + 1) * 2;
      new_elems = re_realloc! ((*set).elems, Idx, (*set).alloc);
      if __glibc_unlikely (new_elems.is_null ())
	{ return false; }
      (*set).elems = new_elems;
    }

  /* Insert the new element.  */
  let n = (*set).nelem;
  (*set).nelem += 1;
  *(*set).elems.add (n as usize) = elem;
  return true;
}

/* Compare two node sets SET1 and SET2.
   Return true if SET1 and SET2 are equivalent.  */

unsafe fn re_node_set_compare (set1: *const re_node_set, set2: *const re_node_set) -> bool
{
  let mut i: Idx;
  if set1.is_null () || set2.is_null () || (*set1).nelem != (*set2).nelem
    { return false; }
  i = (*set1).nelem;
  loop
    {
      i -= 1;
      if !(i >= 0) { break; }
      if *(*set1).elems.add (i as usize) != *(*set2).elems.add (i as usize)
	{ return false; }
    }
  return true;
}

/* Return (idx + 1) if SET contains the element ELEM, return 0 otherwise.  */

unsafe fn re_node_set_contains (set: *const re_node_set, elem: Idx) -> Idx
{
  let mut idx: __re_size_t;
  let mut right: __re_size_t;
  let mut mid: __re_size_t;
  if (*set).nelem <= 0
    { return 0; }

  /* Binary search the element.  */
  idx = 0;
  right = (*set).nelem as __re_size_t - 1;
  while idx < right
    {
      mid = (idx + right) / 2;
      if *(*set).elems.add (mid) < elem
	{ idx = mid + 1; }
      else
	{ right = mid; }
    }
  return if *(*set).elems.add (idx) == elem { idx as Idx + 1 } else { 0 };
}

unsafe fn re_node_set_remove_at (set: *mut re_node_set, mut idx: Idx)
{
  if idx < 0 || idx >= (*set).nelem
    { return; }
  (*set).nelem -= 1;
  while idx < (*set).nelem
    {
      *(*set).elems.add (idx as usize) = *(*set).elems.add ((idx + 1) as usize);
      idx += 1;
    }
}

/* Add the token TOKEN to dfa->nodes, and return the index of the token.
   Or return -1 if an error occurred.  */

#[inline(never)]
unsafe fn re_dfa_add_node (dfa: *mut re_dfa_t, token: re_token_t) -> Idx
{
  if __glibc_unlikely ((*dfa).nodes_len >= (*dfa).nodes_alloc)
    {
      let new_nodes_alloc: usize = (*dfa).nodes_alloc * 2;
      let mut new_nexts: *mut Idx;
      let mut new_indices: *mut Idx;
      let mut new_edests: *mut re_node_set;
      let mut new_eclosures: *mut re_node_set;
      let new_nodes: *mut re_token_t;

      /* Avoid overflows in realloc.  */
      let max_object_size: usize =
          MAX! (std::mem::size_of::<re_token_t> (),
		MAX! (std::mem::size_of::<re_node_set> (), std::mem::size_of::<Idx> ()));
      if __glibc_unlikely (MIN! (IDX_MAX, (SIZE_MAX / max_object_size) as Idx) < new_nodes_alloc as Idx)
	{ return -1; }

      new_nodes = re_realloc! ((*dfa).nodes, re_token_t, new_nodes_alloc);
      if __glibc_unlikely (new_nodes.is_null ())
	{ return -1; }
      (*dfa).nodes = new_nodes;
      (*dfa).nodes_alloc = new_nodes_alloc;
      new_nexts = re_realloc! ((*dfa).nexts, Idx, new_nodes_alloc);
      if !new_nexts.is_null ()
	{ (*dfa).nexts = new_nexts; }
      new_indices = re_realloc! ((*dfa).org_indices, Idx, new_nodes_alloc);
      if !new_indices.is_null ()
	{ (*dfa).org_indices = new_indices; }
      new_edests = re_realloc! ((*dfa).edests, re_node_set, new_nodes_alloc);
      if !new_edests.is_null ()
	{ (*dfa).edests = new_edests; }
      new_eclosures = re_realloc! ((*dfa).eclosures, re_node_set, new_nodes_alloc);
      if !new_eclosures.is_null ()
	{ (*dfa).eclosures = new_eclosures; }
      if __glibc_unlikely (new_nexts.is_null () || new_indices.is_null ()
			    || new_edests.is_null () || new_eclosures.is_null ())
	{ return -1; }
    }
  *(*dfa).nodes.add ((*dfa).nodes_len) = token;
  set_constraint_ ((*dfa).nodes.add ((*dfa).nodes_len), 0);
  set_accept_mb_ ((*dfa).nodes.add ((*dfa).nodes_len),
    ((type_ (&token) == OP_PERIOD as c_uint && (*dfa).mb_cur_max > 1)
     || type_ (&token) == COMPLEX_BRACKET as c_uint) as c_uint);
  *(*dfa).nexts.add ((*dfa).nodes_len) = -1;
  re_node_set_init_empty! ((*dfa).edests.add ((*dfa).nodes_len));
  re_node_set_init_empty! ((*dfa).eclosures.add ((*dfa).nodes_len));
  let r = (*dfa).nodes_len;
  (*dfa).nodes_len += 1;
  return r as Idx;
}

unsafe fn calc_state_hash (nodes: *const re_node_set, context: c_uint) -> re_hashval_t
{
  let mut hash: re_hashval_t = (*nodes).nelem as re_hashval_t + context as re_hashval_t;
  let mut i: Idx;
  i = 0;
  while i < (*nodes).nelem
    {
      hash += *(*nodes).elems.add (i as usize) as re_hashval_t;
      i += 1;
    }
  return hash;
}

/* Search for the state whose node_set is equivalent to NODES. ...  */

#[inline(never)]
unsafe fn re_acquire_state (err: *mut reg_errcode_t, dfa: *const re_dfa_t,
                            nodes: *const re_node_set) -> *mut re_dfastate_t
{
  let hash: re_hashval_t;
  let new_state: *mut re_dfastate_t;
  let spot: *mut re_state_table_entry;
  let mut i: Idx;
  // C: `#if defined GCC_LINT || defined lint *err = REG_NOERROR; #endif` -- dead on the
  // frozen target, kept as text (P-51).
  #[cfg(feature = "gcc_lint")]
  { *err = REG_NOERROR; }
  if __glibc_unlikely ((*nodes).nelem == 0)
    {
      *err = REG_NOERROR;
      return null_mut ();
    }
  hash = calc_state_hash (nodes, 0);
  spot = (*dfa).state_table.add ((hash & (*dfa).state_hash_mask) as usize);

  i = 0;
  while i < (*spot).num
    {
      let state: *mut re_dfastate_t = *(*spot).array.add (i as usize);
      if hash != (*state).hash
	{ i += 1; continue; }
      if re_node_set_compare (&raw const (*state).nodes, nodes)
	{ return state; }
      i += 1;
    }

  /* There are no appropriate state in the dfa, create the new one.  */
  new_state = create_ci_newstate (dfa, nodes, hash);
  if __glibc_unlikely (new_state.is_null ())
    { *err = REG_ESPACE; }

  return new_state;
}

/* Search for the state whose node_set is equivalent to NODES and
   whose context is equivalent to CONTEXT. ...  */

#[inline(never)]
unsafe fn re_acquire_state_context (err: *mut reg_errcode_t, dfa: *const re_dfa_t,
                                    nodes: *const re_node_set, context: c_uint)
                                    -> *mut re_dfastate_t
{
  let hash: re_hashval_t;
  let new_state: *mut re_dfastate_t;
  let spot: *mut re_state_table_entry;
  let mut i: Idx;
  // C: `#if defined GCC_LINT || defined lint *err = REG_NOERROR; #endif` (P-51 text).
  #[cfg(feature = "gcc_lint")]
  { *err = REG_NOERROR; }
  if (*nodes).nelem == 0
    {
      *err = REG_NOERROR;
      return null_mut ();
    }
  hash = calc_state_hash (nodes, context);
  spot = (*dfa).state_table.add ((hash & (*dfa).state_hash_mask) as usize);

  i = 0;
  while i < (*spot).num
    {
      let state: *mut re_dfastate_t = *(*spot).array.add (i as usize);
      if (*state).hash == hash
	  && context_ (state) == context
	  && re_node_set_compare ((*state).entrance_nodes, nodes)
	{ return state; }
      i += 1;
    }
  /* There are no appropriate state in 'dfa', create the new one.  */
  new_state = create_cd_newstate (dfa, nodes, context, hash);
  if __glibc_unlikely (new_state.is_null ())
    { *err = REG_ESPACE; }

  return new_state;
}

/* Finish initialization of the new state NEWSTATE, and using its hash value
   HASH put in the appropriate bucket of DFA's state table.  Return value
   indicates the error code if failed.  */

#[inline(never)]
unsafe fn register_state (dfa: *const re_dfa_t, newstate: *mut re_dfastate_t,
                          hash: re_hashval_t) -> reg_errcode_t
{
  let spot: *mut re_state_table_entry;
  let err: reg_errcode_t;
  let mut i: Idx;

  (*newstate).hash = hash;
  err = re_node_set_alloc (&raw mut (*newstate).non_eps_nodes, (*newstate).nodes.nelem);
  if __glibc_unlikely (err != REG_NOERROR)
    { return REG_ESPACE; }
  i = 0;
  while i < (*newstate).nodes.nelem
    {
      let elem: Idx = *(*newstate).nodes.elems.add (i as usize);
      if IS_EPSILON_NODE! (type_ ((*dfa).nodes.add (elem as usize))) == 0
	{
	  if !re_node_set_insert_last (&raw mut (*newstate).non_eps_nodes, elem)
	    { return REG_ESPACE; }
	}
      i += 1;
    }

  spot = (*dfa).state_table.add ((hash & (*dfa).state_hash_mask) as usize);
  if __glibc_unlikely ((*spot).alloc <= (*spot).num)
    {
      let new_alloc: Idx = 2 * (*spot).num + 2;
      let new_array: *mut *mut re_dfastate_t =
          re_realloc! ((*spot).array, *mut re_dfastate_t, new_alloc);
      if __glibc_unlikely (new_array.is_null ())
	{ return REG_ESPACE; }
      (*spot).array = new_array;
      (*spot).alloc = new_alloc;
    }
  *(*spot).array.add ((*spot).num as usize) = newstate;
  (*spot).num += 1;
  return REG_NOERROR;
}

unsafe fn free_state (state: *mut re_dfastate_t)
{
  re_node_set_free! (&raw mut (*state).non_eps_nodes);
  re_node_set_free! (&raw mut (*state).inveclosure);
  if (*state).entrance_nodes != (&raw mut (*state).nodes)
    {
      re_node_set_free! ((*state).entrance_nodes);
      re_free! ((*state).entrance_nodes);
    }
  re_node_set_free! (&raw mut (*state).nodes);
  re_free! ((*state).word_trtable);
  re_free! ((*state).trtable);
  re_free! (state);
}

/* Create the new state which is independent of contexts.
   Return the new state if succeeded, otherwise return NULL.  */

#[inline(never)]
unsafe fn create_ci_newstate (dfa: *const re_dfa_t, nodes: *const re_node_set,
                              hash: re_hashval_t) -> *mut re_dfastate_t
{
  let mut i: Idx;
  let mut err: reg_errcode_t;
  let mut newstate: *mut re_dfastate_t;

  newstate = calloc (1, std::mem::size_of::<re_dfastate_t> ()) as *mut re_dfastate_t;
  if __glibc_unlikely (newstate.is_null ())
    { return null_mut (); }
  err = re_node_set_init_copy (&raw mut (*newstate).nodes, nodes);
  if __glibc_unlikely (err != REG_NOERROR)
    {
      re_free! (newstate);
      return null_mut ();
    }

  (*newstate).entrance_nodes = &raw mut (*newstate).nodes;
  i = 0;
  while i < (*nodes).nelem
    {
      let node: *mut re_token_t = (*dfa).nodes.add (*(*nodes).elems.add (i as usize) as usize);
      let r#type: c_uint = type_ (node);
      if r#type == CHARACTER as c_uint && constraint_ (node) == 0
	{ i += 1; continue; }
      set_state_accept_mb_ (newstate,
	state_accept_mb_ (newstate) | accept_mb_ (node));

      /* If the state has the halt node, the state is a halt state.  */
      if r#type == END_OF_RE as c_uint
	{ set_halt_ (newstate, 1); }
      else if r#type == OP_BACK_REF as c_uint
	{ set_has_backref_ (newstate, 1); }
      else if r#type == ANCHOR as c_uint || constraint_ (node) != 0
	{ set_has_constraint_ (newstate, 1); }
      i += 1;
    }
  err = register_state (dfa, newstate, hash);
  if __glibc_unlikely (err != REG_NOERROR)
    {
      free_state (newstate);
      newstate = null_mut ();
    }
  return newstate;
}

/* Create the new state which is depend on the context CONTEXT.
   Return the new state if succeeded, otherwise return NULL.  */

#[inline(never)]
unsafe fn create_cd_newstate (dfa: *const re_dfa_t, nodes: *const re_node_set,
                              context: c_uint, hash: re_hashval_t) -> *mut re_dfastate_t
{
  let mut i: Idx;
  let mut nctx_nodes: Idx = 0;
  let mut err: reg_errcode_t;
  let mut newstate: *mut re_dfastate_t;

  newstate = calloc (1, std::mem::size_of::<re_dfastate_t> ()) as *mut re_dfastate_t;
  if __glibc_unlikely (newstate.is_null ())
    { return null_mut (); }
  err = re_node_set_init_copy (&raw mut (*newstate).nodes, nodes);
  if __glibc_unlikely (err != REG_NOERROR)
    {
      re_free! (newstate);
      return null_mut ();
    }

  set_context_ (newstate, context);
  (*newstate).entrance_nodes = &raw mut (*newstate).nodes;

  i = 0;
  while i < (*nodes).nelem
    {
      let node: *mut re_token_t = (*dfa).nodes.add (*(*nodes).elems.add (i as usize) as usize);
      let r#type: c_uint = type_ (node);
      let constraint: c_uint = constraint_ (node);

      if r#type == CHARACTER as c_uint && constraint == 0
	{ i += 1; continue; }
      set_state_accept_mb_ (newstate,
	state_accept_mb_ (newstate) | accept_mb_ (node));

      /* If the state has the halt node, the state is a halt state.  */
      if r#type == END_OF_RE as c_uint
	{ set_halt_ (newstate, 1); }
      else if r#type == OP_BACK_REF as c_uint
	{ set_has_backref_ (newstate, 1); }

      if constraint != 0
	{
	  if (*newstate).entrance_nodes == (&raw mut (*newstate).nodes)
	    {
	      let entrance_nodes: *mut re_node_set = re_malloc! (re_node_set, 1);
	      if __glibc_unlikely (entrance_nodes.is_null ())
		{
		  free_state (newstate);
		  return null_mut ();
		}
	      (*newstate).entrance_nodes = entrance_nodes;
	      if re_node_set_init_copy ((*newstate).entrance_nodes, nodes)
		  != REG_NOERROR
		{
		  free_state (newstate);
		  return null_mut ();
		}
	      nctx_nodes = 0;
	      set_has_constraint_ (newstate, 1);
	    }

	  if NOT_SATISFY_PREV_CONSTRAINT! (constraint, context)
	    {
	      re_node_set_remove_at (&raw mut (*newstate).nodes, i - nctx_nodes);
	      nctx_nodes += 1;
	    }
	}
      i += 1;
    }
  err = register_state (dfa, newstate, hash);
  if __glibc_unlikely (err != REG_NOERROR)
    {
      free_state (newstate);
      newstate = null_mut ();
    }
  return newstate;
}
