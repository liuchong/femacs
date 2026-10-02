// femacs: port of src/doprnt.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// One-to-one port: same statements in the same order, same names (PORTING.md).  The
// C-variadic shapes follow docs/spikes/2026-10-02-variadics.md (P-04): a `...` parameter
// *is* the `VaList`, `va_arg` is `next_arg`, `va_copy` is `clone`, and C's by-value
// `va_list` pass is a `clone ()` at the call site.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

/* Output like sprintf to a buffer of specified size.    -*- coding: utf-8 -*-
   Also takes args differently: pass one pointer to the end
   of the format string in addition to the format string itself.
   Copyright (C) 1985, 2001-2026 Free Software Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

/* If you think about replacing this with some similar standard C function of
   the printf family (such as vsnprintf), please note that this function
   supports the following Emacs-specific features:

   . For %c conversions, it produces a string with the multibyte representation
     of the ('int') argument, suitable for display in an Emacs buffer.

   . For %s and %c, when field width is specified (e.g., %25s), it accounts for
     the display width of each character, according to char-width-table.  That
     is, it does not assume that each character takes one column on display.
     Nor does it assume that each character is a single byte.

   . If the size of the buffer is not enough to produce the formatted string in
     its entirety, it makes sure that truncation does not chop the last
     character in the middle of its multibyte sequence, producing an invalid
     sequence.

   . It accepts a pointer to the end of the format string, so the format string
     could include embedded null characters.

   . It signals an error if the length of the formatted string is about to
     overflow ptrdiff_t or size_t, to avoid producing strings longer than what
     Emacs can handle.

   On the other hand, this function supports only a small subset of the
   standard C formatted output facilities.  E.g., precision is ignored
   in %s and %c conversions, and %lld does not necessarily work and
   code should use something like %"pM"d with intmax_t instead.
   (See below for the detailed documentation of what is supported.)
   However, this is okay, as this function is supposed to be called
   from 'error' and similar C functions, and thus does not need to
   support all the features of 'Fformat_message', which is used by the
   Lisp 'error' function.  */

// src/doprnt.c 119-127: config.h, stdio.h, stdlib.h, float.h, unistd.h, limits.h, then
// lisp.h and character.h.  The system headers reach the port through the WIP mirrors that
// lisp.rs pulls in (stdint/limits/float; P-26), and the four functions this file needs
// from <stdio.h>/<string.h> are declared below until those mirrors carry them.
include!("include/config.rs");
include!("../lib/include/stdio.rs");
include!("include/lisp.rs");
include!("include/character.rs");

// C gets `va_list` from <stdarg.h>; the port's equivalent is core::ffi::VaList.
use core::ffi::VaList;

unsafe extern "C" {
  // <stdio.h>/<string.h> on the frozen target (HAVE_MEMCPY... the config has no
  // HAVE_MEMPCPY, so gnulib's `mempcpy` is `rpl_mempcpy`, which is what C calls here).
  pub fn sprintf (buf: *mut c_char, format: *const c_char, ...) -> c_int;
  pub fn strnlen (s: *const c_char, maxlen: size_t) -> size_t;
  pub fn strncmp (a: *const c_char, b: *const c_char, n: size_t) -> c_int;
  pub fn rpl_mempcpy (dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
  pub fn xpalloc (block: *mut c_void, n: *mut ptrdiff_t, incr: ptrdiff_t,
                  min_size: ptrdiff_t, size: ptrdiff_t) -> *mut c_void;
}

/* Enough to handle floating point formats with large numbers.  */
pub const SIZE_BOUND_EXTRA: c_int = DBL_MAX_10_EXP + 50;

/* C: `char tembuf[SIZE_BOUND_EXTRA + 50]`.  */
const TBUF_LEN: usize = (SIZE_BOUND_EXTRA + 50) as usize;

/* Parse FMT as an unsigned decimal integer, putting its value into *VALUE.
   Return the address of the first byte after the integer.
   If FMT is not an integer, return FMT and store zero into *VALUE.  */
fn parse_format_integer (mut fmt: *const c_char, value: *mut c_int) -> *const c_char
{
  let mut n: c_int = 0;
  let mut overflow = false;
  while unsafe { b'0' as c_char <= *fmt && *fmt <= b'9' as c_char }
    {
      // C's ckd_mul/ckd_add (P-15): they store the wrapped product and say whether it
      // overflowed; `overflowing_*` is the same pair.
      let (prod, mul_over) = n.overflowing_mul (10);
      overflow |= mul_over;
      let (sum, add_over) = prod.overflowing_add (unsafe { *fmt } as c_int - b'0' as c_int);
      overflow |= add_over;
      n = sum;
      fmt = unsafe { fmt.add (1) };
    }
  // C: `min (PTRDIFF_MAX, SIZE_MAX) - SIZE_BOUND_EXTRA < n`, compared in C's common
  // unsigned type.
  let limit = (PTRDIFF_MAX as usize).min (SIZE_MAX) - SIZE_BOUND_EXTRA as usize;
  if overflow || limit < n as usize
    {
      unsafe { error (c"Format width or precision too large".as_ptr ()) }
    }
  unsafe { *value = n };
  return fmt;
}

// <inttypes.h> (gnulib's inttypes.in.h) on the frozen target: the C compiler reports
// `printf ("[%s] [%s] [%s]\n", pD, pI, PRIdMAX)` -> `[t] [l] [jd]`.  The port has no
// inttypes mirror yet (P-26), so the value is recorded here; pD/pI come from lisp.rs.
const PRIdMAX: &str = "jd";

/* Like doprnt, except FORMAT_END must be non-null.
   Although this function is never exercised in current Emacs,
   it is retained in case some future Emacs version
   contains doprnt callers that need such formats.
   Having a separate function helps GCC optimize doprnt better.  */
fn doprnt_non_null_end (buffer: *mut c_char, bufsize: ptrdiff_t, format: *const c_char,
                        format_end: *const c_char, ap: VaList<'_>) -> ptrdiff_t
{
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let fmtlen = unsafe { format_end.offset_from (format) };
  let fmt = SAFE_ALLOCA! (sa_avail, fmtlen + 1) as *mut c_char;
  unsafe {
    std::ptr::copy_nonoverlapping (format, fmt, fmtlen as usize);
    *fmt.offset (fmtlen) = 0;
  }
  let nbytes = unsafe { doprnt (buffer, bufsize, fmt, std::ptr::null (), ap) };
  SAFE_FREE! (sa_count);
  return nbytes;
}

/* Format to BUFFER (of positive size BUFSIZE) data formatted by FORMAT, ... (see the C
   comment above `doprnt` in src/doprnt.c 165-186 for the full contract).  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn doprnt (buffer: *mut c_char, mut bufsize: ptrdiff_t,
                                 format: *const c_char, format_end: *const c_char,
                                 mut ap: VaList<'_>) -> ptrdiff_t
{
  if !format_end.is_null ()
    {
      return doprnt_non_null_end (buffer, bufsize, format, format_end, ap);
    }

  let mut fmt = format;		/* Pointer into format string.  */
  let mut bufptr = buffer;	/* Pointer into output buffer.  */

  /* Use this for sprintf unless we need something really big.  */
  let mut tembuf = [0 as c_char; TBUF_LEN];

  /* Size of sprintf_buffer.  */
  let mut size_allocated: ptrdiff_t = std::mem::size_of_val (&tembuf) as ptrdiff_t;

  /* Buffer to use for sprintf.  Either tembuf or same as BIG_BUFFER.  */
  let mut sprintf_buffer: *mut c_char = tembuf.as_mut_ptr ();

  /* Buffer we have got with malloc.  */
  let mut big_buffer: *mut c_char = std::ptr::null_mut ();

  let quoting_style = Ftext_quoting_style ();

  bufsize -= 1;

  /* Loop until end of format string or buffer full. */
  'outer: while unsafe { *fmt } != 0 && bufsize > 0
    {
      let fmt0 = fmt;
      let mut fmtchar = unsafe { *fmt };
      fmt = unsafe { fmt.add (1) };
      // C's `goto doit` / `goto doit1` are forward jumps inside the same iteration, so
      // the port runs that shared tail in place and remembers which entry to use.
      let mut tail: u8 = 0;	/* 0: ordinary character, 1: doit, 2: doit1 */
      if fmtchar == b'%' as c_char
	{
	  let mut width: ptrdiff_t = 0;	/* Columns occupied by STRING on display.  */
	  const PDLEN: isize = pD.len () as isize;
	  const PILEN: isize = pI.len () as isize;
	  const PMLEN: isize = PRIdMAX.len () as isize - 1;
	  // C: `max (max (1, pDlen), max (pIlen, pMlen))`.
	  const MAXMLEN: isize = {
	      let a = if PDLEN > 1 { PDLEN } else { 1 };
	      let b = if PILEN > PMLEN { PILEN } else { PMLEN };
	      if a > b { a } else { b }
	  };
	  #[derive(PartialEq, Clone, Copy)]
	  #[allow(non_camel_case_types)]
	  enum length_modifier { no_modifier, long_modifier, pD_modifier, pI_modifier, pM_modifier }
	  #[allow(non_upper_case_globals)]
	  const modifier_len: [isize; 5] = [0, 1, PDLEN, PILEN, PMLEN];
	  let mut charbuf_storage = [0 as c_char; (MAX_MULTIBYTE_LENGTH + 1) as usize];
	  let charbuf: *mut c_char;

	  /* Width and precision specified by this %-sequence.  */
	  let mut wid: c_int = 0;
	  let mut prec: c_int = -1;

	  /* FMTSTAR will be a "%*.*X"-like version of this %-sequence.
	     Start by putting '%' into FMTSTAR.  */
	  let mut fmtstar = [0 as c_char; 10 + MAXMLEN as usize];
	  let mut string: *mut c_char = fmtstar.as_mut_ptr ();
	  unsafe { *string = b'%' as c_char };
	  string = unsafe { string.add (1) };

	  /* Copy at most one instance of each flag into FMTSTAR.  */
	  let mut minusflag = false;
	  let mut plusflag = false;
	  let mut zeroflag = false;
	  let mut spaceflag = false;
	  loop
	    {
	      unsafe { *string = *fmt };
	      let c = unsafe { *fmt } as u8 as char;
	      match c
		{
		  '-' => { if !minusflag { string = unsafe { string.add (1) } } minusflag = true; fmt = unsafe { fmt.add (1) }; continue; }
		  '+' => { if !plusflag { string = unsafe { string.add (1) } } plusflag = true; fmt = unsafe { fmt.add (1) }; continue; }
		  ' ' => { if !spaceflag { string = unsafe { string.add (1) } } spaceflag = true; fmt = unsafe { fmt.add (1) }; continue; }
		  '0' => { if !zeroflag { string = unsafe { string.add (1) } } zeroflag = true; fmt = unsafe { fmt.add (1) }; continue; }
		  _ => {}
		}
	      break;
	    }

	  /* Parse width and precision, putting "*.*" into FMTSTAR.  */
	  if unsafe { b'1' as c_char <= *fmt && *fmt <= b'9' as c_char }
	    {
	      fmt = parse_format_integer (fmt, &raw mut wid);
	    }
	  if unsafe { *fmt } == b'.' as c_char
	    {
	      fmt = parse_format_integer (unsafe { fmt.add (1) }, &raw mut prec);
	    }
	  unsafe {
	    *string = b'*' as c_char;
	    string = string.add (1);
	    *string = b'.' as c_char;
	    string = string.add (1);
	    *string = b'*' as c_char;
	    string = string.add (1);
	  }

	  /* Check for the length modifiers in textual length order, so
	     that longer modifiers override shorter ones.  */
	  let mut length_mod: length_modifier = length_modifier::no_modifier;
	  let mut mlen: isize = 1;
	  while mlen <= MAXMLEN
	    {
	      if mlen == 1 && unsafe { *fmt } == b'l' as c_char
		{ length_mod = length_modifier::long_modifier; }
	      if mlen == PDLEN && unsafe { strncmp (fmt, pD.as_ptr () as *const c_char, PDLEN as usize) } == 0
		{ length_mod = length_modifier::pD_modifier; }
	      if mlen == PILEN && unsafe { strncmp (fmt, pI.as_ptr () as *const c_char, PILEN as usize) } == 0
		{ length_mod = length_modifier::pI_modifier; }
	      if mlen == PMLEN && unsafe { strncmp (fmt, PRIdMAX.as_ptr () as *const c_char, PMLEN as usize) } == 0
		{ length_mod = length_modifier::pM_modifier; }
	      mlen += 1;
	    }

	  /* Copy optional length modifier and conversion specifier
	     character into FMTSTAR, and append a NUL.  */
	  mlen = modifier_len[length_mod as usize];
	  string = unsafe {
	    rpl_mempcpy (string as *mut c_void, fmt as *const c_void, (mlen + 1) as size_t)
	      as *mut c_char
	  };
	  fmt = unsafe { fmt.add (mlen as usize) };
	  unsafe { *string = 0 };

	  /* An idea of how much space we might need. ... */
	  let size_bound = (if wid > prec { wid } else { prec }) as ptrdiff_t + SIZE_BOUND_EXTRA as ptrdiff_t;

	  /* Make sure we have that much.  */
	  if size_bound > size_allocated
	    {
	      if !big_buffer.is_null () { unsafe { xfree (big_buffer as *mut c_void) } }
	      big_buffer = unsafe { xmalloc (size_bound as size_t) } as *mut c_char;
	      sprintf_buffer = big_buffer;
	      size_allocated = size_bound;
	    }
	  let mut minlen: c_int = 0;
	  let mut tem: ptrdiff_t = 0;

	  let conversion = unsafe { *fmt } as u8 as char;
	  fmt = unsafe { fmt.add (1) };
	  match conversion
	    {
	      /* C's `default:` comes first there; Rust's match arms are unordered, so this
		 arm takes anything the others do not.  */
	      'd' | 'o' | 'u' | 'x' | 'X' => {
		let unsigned = conversion != 'd';
		match length_mod
		  {
		    length_modifier::no_modifier => {
		      if unsigned
			{
			  let v = unsafe { ap.next_arg::<c_uint> () };
			  tem = unsafe { sprintf (sprintf_buffer, fmtstar.as_ptr (), wid, prec, v) } as ptrdiff_t;
			}
		      else
			{
			  let v = unsafe { ap.next_arg::<c_int> () };
			  tem = unsafe { sprintf (sprintf_buffer, fmtstar.as_ptr (), wid, prec, v) } as ptrdiff_t;
			}
		    }
		    length_modifier::long_modifier => {
		      if unsigned
			{
			  let v = unsafe { ap.next_arg::<c_ulong> () };
			  tem = unsafe { sprintf (sprintf_buffer, fmtstar.as_ptr (), wid, prec, v) } as ptrdiff_t;
			}
		      else
			{
			  let v = unsafe { ap.next_arg::<c_long> () };
			  tem = unsafe { sprintf (sprintf_buffer, fmtstar.as_ptr (), wid, prec, v) } as ptrdiff_t;
			}
		    }
		    /* C's `goto signed_pD_modifier` from the unsigned cases lands on the same
		       block the 'd' case uses: a ptrdiff_t read, formatted with FMTSTAR.  */
		    length_modifier::pD_modifier => {
		      let v = unsafe { ap.next_arg::<ptrdiff_t> () };
		      tem = unsafe { sprintf (sprintf_buffer, fmtstar.as_ptr (), wid, prec, v) } as ptrdiff_t;
		    }
		    length_modifier::pI_modifier => {
		      if unsigned
			{
			  let v = unsafe { ap.next_arg::<EMACS_UINT> () };
			  tem = unsafe { sprintf (sprintf_buffer, fmtstar.as_ptr (), wid, prec, v) } as ptrdiff_t;
			}
		      else
			{
			  let v = unsafe { ap.next_arg::<EMACS_INT> () };
			  tem = unsafe { sprintf (sprintf_buffer, fmtstar.as_ptr (), wid, prec, v) } as ptrdiff_t;
			}
		    }
		    length_modifier::pM_modifier => {
		      if unsigned
			{
			  let v = unsafe { ap.next_arg::<u64> () };
			  tem = unsafe { sprintf (sprintf_buffer, fmtstar.as_ptr (), wid, prec, v) } as ptrdiff_t;
			}
		      else
			{
			  let v = unsafe { ap.next_arg::<i64> () };
			  tem = unsafe { sprintf (sprintf_buffer, fmtstar.as_ptr (), wid, prec, v) } as ptrdiff_t;
			}
		    }
		  }
		/* Now copy into final output, truncating as necessary.  */
		string = sprintf_buffer;
		tail = 1;
	      }
	      'f' | 'e' | 'g' => {
		let d = unsafe { ap.next_arg::<c_double> () };
		tem = unsafe { sprintf (sprintf_buffer, fmtstar.as_ptr (), wid, prec, d) } as ptrdiff_t;
		/* Now copy into final output, truncating as necessary.  */
		string = sprintf_buffer;
		tail = 1;
	      }
	      'S' | 's' => {
		minlen = if minusflag { -(wid) } else { wid };
		string = unsafe { ap.next_arg::<*mut c_char> () };
		tem = unsafe { strnlen (string, (STRING_BYTES_BOUND + 1) as size_t) } as ptrdiff_t;
		if tem == STRING_BYTES_BOUND + 1
		  {
		    unsafe { error (c"String for %%s or %%S format is too long".as_ptr ()) }
		  }
		width = strwidth (string, tem);
		tail = 2;
	      }
	      'c' => {
		charbuf = unsafe { charbuf_storage.as_mut_ptr () };
		let chr = unsafe { ap.next_arg::<c_int> () };
		tem = CHAR_STRING (chr, unsafe { charbuf as *mut u8 }) as ptrdiff_t;
		string = charbuf;
		unsafe { *string.offset (tem) = 0 };
		width = strwidth (string, tem);
		minlen = if minusflag { -(wid) } else { wid };
		tail = 2;
	      }
	      '%' => {
		/* Treat this '%' as normal: fall through to the ordinary-character code.  */
		tail = 0;
	      }
	      _ => {
		unsafe { error (c"Invalid format operation %s".as_ptr (), fmt0) }
	      }
	    }

	  if tail != 0
	    {
	      if tail == 1
		{
		  /* C's `doit:` — coming here means STRING contains ASCII only.  */
		  eassert! (0 <= tem);
		  if STRING_BYTES_BOUND < tem
		    {
		      unsafe { error (c"Format width or precision too large".as_ptr ()) }
		    }
		  width = tem;
		}
	      /* C's `doit1:` ...  */
	      if minlen > 0
		{
		  while (minlen as ptrdiff_t) > width && bufsize > 0
		    {
		      unsafe { *bufptr = b' ' as c_char };
		      bufptr = unsafe { bufptr.add (1) };
		      bufsize -= 1;
		      minlen -= 1;
		    }
		  minlen = 0;
		}
	      if tem > bufsize
		{
		  /* Truncate the string at character boundary.  */
		  tem = bufsize;
		  while tem != 0
		    {
		      tem -= 1;
		      if CHAR_HEAD_P (unsafe { *string.offset (tem) } as c_int)
			{
			  if BYTES_BY_CHAR_HEAD (unsafe { *string.offset (tem) } as c_int) as ptrdiff_t
			     <= bufsize - tem
			    {
			      tem = bufsize;
			    }
			  break;
			}
		    }

		  unsafe { std::ptr::copy_nonoverlapping (string, bufptr, tem as usize) };
		  while tem < bufsize
		    {
		      unsafe { *bufptr.offset (tem) = 0 };
		      tem += 1;
		    }
		  /* Trigger exit from the loop, but make sure we
		     return to the caller a value which will indicate
		     that the buffer was too small.  */
		  bufptr = unsafe { bufptr.offset (bufsize) };
		  bufsize = 0;
		  continue 'outer;
		}
	      unsafe { std::ptr::copy_nonoverlapping (string, bufptr, tem as usize) };
	      bufptr = unsafe { bufptr.add (tem as usize) };
	      bufsize -= tem;
	      if minlen < 0
		{
		  while (minlen as ptrdiff_t) < -width && bufsize > 0
		    {
		      unsafe { *bufptr = b' ' as c_char };
		      bufptr = unsafe { bufptr.add (1) };
		      bufsize -= 1;
		      minlen += 1;
		    }
		  minlen = 0;
		}
	      continue 'outer;
	    }
	}

      let mut src: *const c_char;
      let mut srclen: ptrdiff_t;
      if EQ (quoting_style, Qcurve ()) && fmtchar == b'`' as c_char
	{
	  src = uLSQM.as_ptr ();
	  srclen = uLSQM.to_bytes ().len () as ptrdiff_t;
	}
      else if EQ (quoting_style, Qcurve ()) && fmtchar == b'\'' as c_char
	{
	  src = uRSQM.as_ptr ();
	  srclen = uRSQM.to_bytes ().len () as ptrdiff_t;
	}
      else if !LEADING_CODE_P (fmtchar as c_int)
	{
	  if EQ (quoting_style, Qstraight ()) && fmtchar == b'`' as c_char
	    {
	      fmtchar = b'\'' as c_char;
	    }
	  unsafe { *bufptr = fmtchar };
	  bufptr = unsafe { bufptr.add (1) };
	  bufsize -= 1;
	  continue 'outer;
	}
      else
	{
	  let charlen = BYTES_BY_CHAR_HEAD (fmtchar as c_int) as ptrdiff_t;
	  src = fmt0;

	  /* If the format string ends in the middle of a multibyte
	     character we don't want to skip over the NUL byte.  */
	  srclen = 1;
	  while unsafe { *src.offset (srclen) } != 0 && srclen < charlen
	    {
	      srclen += 1;
	    }

	  fmt = unsafe { src.offset (srclen) };
	}

      if bufsize < srclen
	{
	  /* Truncate, but return value that will signal to caller
	     that the buffer was too small.  */
	  loop
	    {
	      unsafe { *bufptr = 0 };
	      bufptr = unsafe { bufptr.add (1) };
	      bufsize -= 1;
	      if bufsize == 0
		{
		  break;
		}
	    }
	}
      else
	{
	  loop
	    {
	      unsafe {
		*bufptr = *src;
		bufptr = bufptr.add (1);
		src = src.add (1);
	      }
	      bufsize -= 1;
	      srclen -= 1;
	      if srclen == 0
		{
		  break;
		}
	    }
	}
    }

  /* If we had to malloc something, free it.  */
  unsafe { xfree (big_buffer as *mut c_void) };

  unsafe { *bufptr = 0 };	/* Make sure our string ends with a '\0' */
  return unsafe { bufptr.offset_from (buffer) };
}

/* Format to an unbounded buffer BUF.  This is like sprintf, except it
   is not limited to returning an 'int' so it doesn't have a silly 2
   GiB limit on typical 64-bit hosts.  However, it is limited to the
   Emacs-style formats that doprnt supports, and it requotes ` and '
   as per ‘text-quoting-style’.

   Return the number of bytes put into BUF, excluding the terminating
   '\0'.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn esprintf (buf: *mut c_char, format: *const c_char,
                                   mut ap: ...) -> ptrdiff_t
{
  // C's `va_start (ap, format)` / `va_end (ap)` have no port counterpart: the `...`
  // parameter is the list (P-04).  `TYPE_MAXIMUM (ptrdiff_t)` from gnulib's intprops.h
  // is the type's maximum, i.e. isize::MAX.
  let nbytes = doprnt (buf, isize::MAX, format, std::ptr::null (), ap);
  return nbytes;
}

#[cfg(feature = "x_windows")]
pub unsafe extern "C" fn exprintf (buf: *mut *mut c_char, bufsize: *mut ptrdiff_t,
                                   nonheapbuf: *mut c_char, bufsize_max: ptrdiff_t,
                                   format: *const c_char, mut ap: ...) -> ptrdiff_t
{
  let nbytes = evxprintf (buf, bufsize, nonheapbuf, bufsize_max, format, ap);
  return nbytes;
}

// C gates `exprintf` on `HAVE_X_WINDOWS && USE_X_TOOLKIT` (doprnt.c 561); the frozen
// configuration has neither, so the port gates the function on a feature and keeps the
// two constants from drifting (P-08), the shape buffer.rs uses for tree-sitter.
const _: () = assert! (!cfg! (feature = "x_windows") || (HAVE_X_WINDOWS && USE_X_TOOLKIT));

/* Act like exprintf, except take a va_list.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn evxprintf (buf: *mut *mut c_char, bufsize: *mut ptrdiff_t,
                                    nonheapbuf: *mut c_char, bufsize_max: ptrdiff_t,
                                    format: *const c_char, mut ap: VaList<'_>) -> ptrdiff_t
{
  loop
    {
      // C: `va_list ap_copy; va_copy (ap_copy, ap);` … `va_end (ap_copy);` — the clone
      // is the copy and its drop is the va_end, so the outer list survives the loop.
      let mut ap_copy = ap.clone ();
      let nbytes = doprnt (*buf, *bufsize, format, std::ptr::null (), ap_copy);
      if nbytes < *bufsize - 1
	{
	  return nbytes;
	}
      if *buf != nonheapbuf
	{
	  xfree (*buf as *mut c_void);
	  *buf = std::ptr::null_mut ();
	}
      *buf = xpalloc (std::ptr::null_mut (), bufsize, 1, bufsize_max, 1) as *mut c_char;
    }
}
