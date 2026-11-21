// femacs: port of src/json.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  The four subrs
// use `DEFUN!` (PORTING.md 5.1).  json.c has no `#if` block at all: the three macros it
// defines are the two workspace sizes and JSON_OUT_STR, and the only configuration macros it
// mentions are the ones its includes bring in.
//
// `JSON_OUT_STR (jo, "null")` (C 272) is the macro `json_out_str (jo, str, sizeof (str) - 1)`;
// a Rust string literal has no `sizeof`, so the port's macro takes the count from the literal
// (`to_bytes ().len ()`) and the `sizeof` token is in docs/ports/macro-body-notes.tsv.
//
// `struct json_parser`'s two function pointers (C 720-721) are `Option<unsafe extern "C" fn>`
// fields and are called with `.unwrap () (…)`, the vm-limit.rs shape (the C field is never
// null once json_parser_init has run).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;

/* C's `<errno.h>`, `<stddef.h>`, `<stdint.h>`, `<stdlib.h>` and `<math.h>`: the first is the
   lib mirror, the last supplies `isfinite`, and the rest are the prelude's.  */
// [U1] config.rs provided by common
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/math.rs imported via common
// [U1] ../lib/include/errno.rs imported via common
// [U1] ../lib/include/stdlib.rs imported via common
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

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum json_object_type
  {
    json_object_hashtable,
    json_object_alist,
    json_object_plist,
  }

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum json_array_type
  {
    json_array_array,
    json_array_list,
  }

/* C spells the enumerators bare in the two `switch`es; the port imports the variants.  */
use json_object_type::*;
use json_array_type::*;

#[repr(C)]
pub struct json_configuration
{
  pub object_type: json_object_type,
  pub array_type: json_array_type,
  pub null_object: Lisp_Object,
  pub false_object: Lisp_Object,
}

/* gnulib's INT_BUFSIZE_BOUND (intprops.h 114) is `INT_STRLEN_BOUND (t) + 1`; the derivation
   and the instantiation for a signed 64-bit EMACS_INT (= 21) are data.rs's (the macro is
   type-generic in C and `<intprops.h>` has no generic mirror, P-26).  */
const INT_BUFSIZE_BOUND: usize = ((EMACS_INT_WIDTH - 1) * 146 + 484) / 485 + 1 + 1;

unsafe fn json_parse_args (nargs: ptrdiff_t, args: *mut Lisp_Object,
			   conf: *mut json_configuration,
			   parse_object_types: bool)
{
  if nargs % 2 != 0
    { wrong_type_argument (Qplistp (), Flist (nargs, args)); }

  /* Start from the back so keyword values appearing first take
     precedence.  */
  let mut i: ptrdiff_t = nargs;
  while i > 0
    {
      let key: Lisp_Object = *args.offset (i - 2);
      let value: Lisp_Object = *args.offset (i - 1);
      if parse_object_types && EQ (key, QCobject_type ())
	{
	  if EQ (value, Qhash_table ())
	    { (*conf).object_type = json_object_hashtable; }
	  else if EQ (value, Qalist ())
	    { (*conf).object_type = json_object_alist; }
	  else if EQ (value, Qplist ())
	    { (*conf).object_type = json_object_plist; }
	  else
	    { wrong_choice (list3 (Qhash_table (), Qalist (), Qplist ()), value); }
	}
      else if parse_object_types && EQ (key, QCarray_type ())
	{
	  if EQ (value, Qarray ())
	    { (*conf).array_type = json_array_array; }
	  else if EQ (value, Qlist ())
	    { (*conf).array_type = json_array_list; }
	  else
	    { wrong_choice (list2 (Qarray (), Qlist ()), value); }
	}
      else if EQ (key, QCnull_object ())
	{ (*conf).null_object = value; }
      else if EQ (key, QCfalse_object ())
	{ (*conf).false_object = value; }
      else if parse_object_types
	{ wrong_choice (list4 (QCobject_type (),
			       QCarray_type (),
			       QCnull_object (),
			       QCfalse_object ()),
			value); }
      else
	{ wrong_choice (list2 (QCnull_object (),
			       QCfalse_object ()),
			value); }
      i -= 2;
    }
}

/* JSON encoding context.  */
#[repr(C)]
pub struct json_out_t
{
  pub buf: *mut c_char,
  pub size: ptrdiff_t,	      /* number of bytes in buf */
  pub capacity: ptrdiff_t,	      /* allocated size of buf */
  pub chars_delta: ptrdiff_t,      /* size - {number of characters in buf} */

  pub maxdepth: c_int,
  pub ss_table: *mut symset_tbl,	/* table used by containing object */
  pub conf: json_configuration,
}

/* Set of symbols.  */
#[repr(C)]
pub struct symset_t
{
  pub count: ptrdiff_t,		/* symbols in table */
  pub bits: c_int,			/* log2(table size) */
  pub table: *mut symset_tbl,	/* heap-allocated table */
}

#[repr(C)]
pub struct symset_tbl
{
  /* Table used by the containing object if any, so that we can free all
     tables if an error occurs.  */
  pub up: *mut symset_tbl,
  /* Table of symbols (2**bits elements), Qunbound where unused.  */
  pub entries: [Lisp_Object; 0],
}

unsafe fn symset_size (bits: c_int) -> ptrdiff_t
{
  return (1 as ptrdiff_t) << bits;
}

unsafe fn make_symset_table (bits: c_int, up: *mut symset_tbl) -> *mut symset_tbl
{
  let maxbits: c_int = min ((SIZE_WIDTH - 2 - (if word_size < 8 { 2 } else { 3 })) as isize,
			    32) as c_int;
  if bits > maxbits
    { memory_full_up (); }	/* Will never happen in practice.  */
  let st: *mut symset_tbl
    = xmalloc (size_of::<symset_tbl> () + (size_of::<Lisp_Object> () << bits))
      as *mut symset_tbl;
  (*st).up = up;
  let size: ptrdiff_t = symset_size (bits);
  let mut i: ptrdiff_t = 0;
  while i < size
    {
      FAM_ELEM! ((*st).entries, i) = Qunbound ();
      i += 1;
    }
  return st;
}

/* Create a new symset to use for a new object.  */
unsafe fn push_symset (jo: *mut json_out_t) -> symset_t
{
  let bits: c_int = 4;
  let tbl: *mut symset_tbl = make_symset_table (bits, (*jo).ss_table);
  (*jo).ss_table = tbl;
  return symset_t { count: 0, bits: bits, table: tbl };
}

/* Destroy the current symset.  */
unsafe fn pop_symset (jo: *mut json_out_t, ss: *mut symset_t)
{
  (*jo).ss_table = (*(*ss).table).up;
  xfree ((*ss).table as *mut c_void);
}

/* Remove all heap-allocated symset tables, in case an error occurred.  */
unsafe fn cleanup_symset_tables (mut st: *mut symset_tbl)
{
  while !st.is_null ()
    {
      let up: *mut symset_tbl = (*st).up;
      xfree (st as *mut c_void);
      st = up;
    }
}

unsafe fn symset_hash (sym: Lisp_Object, bits: c_int) -> u32
{
  return knuth_hash (reduce_emacs_uint_to_hash_hash (XHASH (sym)), bits as c_uint) as u32;
}

/* Enlarge the table used by a symset.  */
unsafe fn symset_expand (ss: *mut symset_t)
{
  let old_table: *mut symset_tbl = (*ss).table;
  let oldbits: c_int = (*ss).bits;
  let oldsize: ptrdiff_t = symset_size (oldbits);
  let bits: c_int = oldbits + 1;
  (*ss).bits = bits;
  (*ss).table = make_symset_table (bits, (*old_table).up);
  /* Move all entries from the old table to the new one.  */
  let mask: ptrdiff_t = symset_size (bits) - 1;
  let tbl: *mut symset_tbl = (*ss).table;
  let mut i: ptrdiff_t = 0;
  while i < oldsize
    {
      let sym: Lisp_Object = FAM_ELEM! ((*old_table).entries, i);
      if !BASE_EQ (sym, Qunbound ())
	{
	  let mut j: ptrdiff_t = symset_hash (sym, bits) as ptrdiff_t;
	  while !BASE_EQ (FAM_ELEM! ((*tbl).entries, j), Qunbound ())
	    { j = (j + 1) & mask; }
	  FAM_ELEM! ((*tbl).entries, j) = sym;
	}
      i += 1;
    }
  xfree (old_table as *mut c_void);
}

/* If sym is in ss, return false; otherwise add it and return true.
   Comparison is done by strict identity.  */
unsafe fn symset_add (jo: *mut json_out_t, ss: *mut symset_t, sym: Lisp_Object) -> bool
{
  /* Make sure we don't fill more than half of the table.  */
  if (*ss).count >= (symset_size ((*ss).bits) >> 1)
    {
      symset_expand (ss);
      (*jo).ss_table = (*ss).table;
    }

  let tbl: *mut symset_tbl = (*ss).table;
  let mask: ptrdiff_t = symset_size ((*ss).bits) - 1;
  let mut i: ptrdiff_t = symset_hash (sym, (*ss).bits) as ptrdiff_t;
  loop
    {
      let s: Lisp_Object = FAM_ELEM! ((*tbl).entries, i);
      if BASE_EQ (s, sym)
	{ return false; }		/* Previous occurrence found.  */
      if BASE_EQ (s, Qunbound ())
	{
	  /* Not in set, add it.  */
	  FAM_ELEM! ((*tbl).entries, i) = sym;
	  (*ss).count += 1;
	  return true;
	}
      i = (i + 1) & mask;
    }
}

unsafe fn json_out_grow_buf (jo: *mut json_out_t, incr_min: ptrdiff_t)
{
  let mut new_size: ptrdiff_t = (*jo).capacity;
  (*jo).buf = xpalloc ((*jo).buf as *mut c_void, &raw mut new_size, incr_min, -1, 1)
    as *mut c_char;
  (*jo).capacity = new_size;
}

pub unsafe extern "C-unwind" fn cleanup_json_out (arg: *mut c_void)
{
  let jo: *mut json_out_t = arg as *mut json_out_t;
  xfree ((*jo).buf as *mut c_void);
  (*jo).buf = std::ptr::null_mut ();
  cleanup_symset_tables ((*jo).ss_table);
}

/* Make room for `bytes` more bytes in buffer.  */
unsafe fn json_make_room (jo: *mut json_out_t, bytes: ptrdiff_t)
{
  let avail: ptrdiff_t = (*jo).capacity - (*jo).size;
  if avail < bytes
    { json_out_grow_buf (jo, bytes - avail); }
}

/* C 272's `#define JSON_OUT_STR(jo, str) (json_out_str (jo, str, sizeof (str) - 1))`: the
   byte count of the C string literal, which Rust takes from the literal itself (the `sizeof`
   token is in docs/ports/macro-body-notes.tsv).  */
macro_rules! JSON_OUT_STR {
    ($jo:expr, $str:expr) => {
        json_out_str ($jo, $str.as_ptr (), $str.to_bytes ().len () as ptrdiff_t)
    };
}

/* Add `bytes` bytes from `str` to the buffer.  */
unsafe fn json_out_str (jo: *mut json_out_t, str: *const c_char, bytes: ptrdiff_t)
{
  json_make_room (jo, bytes);
  std::ptr::copy_nonoverlapping (str as *const c_void,
		       (*jo).buf.offset ((*jo).size as isize) as *mut c_void,
		       bytes as usize);
  (*jo).size += bytes;
}

unsafe fn json_out_byte (jo: *mut json_out_t, c: u8)
{
  json_make_room (jo, 1);
  *(*jo).buf.offset ((*jo).size as isize) = c as c_char;
  (*jo).size += 1;
}

unsafe fn json_out_fixnum (jo: *mut json_out_t, x: EMACS_INT)
{
  let mut buf: [c_char; INT_BUFSIZE_BOUND] = [0; INT_BUFSIZE_BOUND];
  let end: *mut c_char = buf.as_mut_ptr ().offset (size_of::<[c_char; INT_BUFSIZE_BOUND]> ()
						   as isize);
  let p: *mut c_char = fixnum_to_string (x, buf.as_mut_ptr (), end);
  json_out_str (jo, p, (end as ptrdiff_t) - (p as ptrdiff_t));
}

unsafe fn string_not_unicode (obj: Lisp_Object) -> !
{
  /* FIXME: this is just for compatibility with existing tests, it's not
     a very descriptive error.  */
  wrong_type_argument (Qjson_value_p (), obj);
}

static json_plain_char: [u8; 256] = [
  /* 32 chars/line: 1 for printable ASCII + DEL except " and \, 0 elsewhere */
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, /* 00-1f */
  1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, /* 20-3f */
  1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1, /* 40-5f */
  1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, /* 60-7f */
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, /* 80-9f */
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, /* a0-bf */
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, /* c0-df */
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, /* e0-ff */
];

unsafe fn json_out_string (jo: *mut json_out_t, str: Lisp_Object, skip: c_int)
{
  /* FIXME: this code is slow, make faster! */

  /* C's `static const char hexchar[16] ATTRIBUTE_NONSTRING = "0123456789ABCDEF";` (C 324);
     the attribute has no Rust counterpart (docs/ports/macro-body-notes.tsv).  */
  static hexchar: [c_char; 16] = [b'0' as c_char, b'1' as c_char, b'2' as c_char,
				  b'3' as c_char, b'4' as c_char, b'5' as c_char,
				  b'6' as c_char, b'7' as c_char, b'8' as c_char,
				  b'9' as c_char, b'A' as c_char, b'B' as c_char,
				  b'C' as c_char, b'D' as c_char, b'E' as c_char,
				  b'F' as c_char];
  let len: ptrdiff_t = SBYTES (str);
  json_make_room (jo, len + 2);
  json_out_byte (jo, 0x22);
  let mut p: *mut u8 = SDATA (str);
  let end: *mut u8 = p.offset (len as isize);
  p = p.offset (skip as isize);
  while p < end
    {
      let mut run: *mut u8 = p;
      while p < end && json_plain_char[*p as usize] != 0
	{ p = p.offset (1); }
      if p > run
	{
	  json_out_str (jo, run as *const c_char, (p as ptrdiff_t) - (run as ptrdiff_t));
	  continue;
	}

      let c: u8 = *p;
      if c > 0x7f
	{
	  if STRING_MULTIBYTE (str)
	    {
	      let n: c_int;
	      if c <= 0xc1
		{ string_not_unicode (str); }
	      if c <= 0xdf
		{ n = 2; }
	      else if c <= 0xef
		{
		  let v: c_int = ((((c as c_int) & 0x0f) << 12)
				  + (((*p.offset (1) as c_int) & 0x3f) << 6)
				  + ((*p.offset (2) as c_int) & 0x3f));
		  if char_surrogate_p (v)
		    { string_not_unicode (str); }
		  n = 3;
		}
	      else if c <= 0xf7
		{
		  let v: c_int = ((((c as c_int) & 0x07) << 18)
				  + (((*p.offset (1) as c_int) & 0x3f) << 12)
				  + (((*p.offset (2) as c_int) & 0x3f) << 6)
				  + ((*p.offset (3) as c_int) & 0x3f));
		  if v > MAX_UNICODE_CHAR
		    { string_not_unicode (str); }
		  n = 4;
		}
	      else
		{ string_not_unicode (str); }
	      json_out_str (jo, p as *const c_char, n as ptrdiff_t);
	      (*jo).chars_delta += n as ptrdiff_t - 1;
	      p = p.offset (n as isize);
	    }
	  else
	    { string_not_unicode (str); }
	}
      else
	{
	  json_out_byte (jo, b'\\');
	  match c
	    {
	    0x22 => { json_out_byte (jo, c); }
	    b'\\' => { json_out_byte (jo, c); }
	    0x08 => { json_out_byte (jo, b'b'); }	/* \b */
	    0x09 => { json_out_byte (jo, b't'); }	/* \t */
	    0x0a => { json_out_byte (jo, b'n'); }	/* \n */
	    0x0c => { json_out_byte (jo, b'f'); }	/* \f */
	    0x0d => { json_out_byte (jo, b'r'); }	/* \r */
	    _ =>
	      {
		let hex: [c_char; 5] = [ b'u' as c_char, b'0' as c_char, b'0' as c_char,
					 hexchar[(c >> 4) as usize],
					 hexchar[(c & 0xf) as usize] ];
		json_out_str (jo, hex.as_ptr (), 5);
	      }
	    }
	  p = p.offset (1);
	}
    }
  json_out_byte (jo, 0x22);
}

unsafe fn json_out_nest (jo: *mut json_out_t)
{
  (*jo).maxdepth -= 1;
  if (*jo).maxdepth < 0
    { error (c"Maximum JSON serialization depth exceeded".as_ptr ()); }
}

unsafe fn json_out_unnest (jo: *mut json_out_t)
{
  (*jo).maxdepth += 1;
}

/* C 419's `static void json_out_something (…);` has no counterpart: Rust items are
   order-independent, so the definition below is visible to the calls above.  */

unsafe fn json_out_object_cons (jo: *mut json_out_t, obj: Lisp_Object)
{
  json_out_nest (jo);
  let mut ss: symset_t = push_symset (jo);
  json_out_byte (jo, 0x7b);
  let is_alist: bool = CONSP (XCAR (obj));
  let mut first: bool = true;
  let mut tail: Lisp_Object = obj;
  FOR_EACH_TAIL! (tail, {
      let key: Lisp_Object;
      let value: Lisp_Object;
      let mut key2: Lisp_Object;
      if is_alist
	{
	  let pair: Lisp_Object = XCAR (tail);
	  CHECK_CONS (pair);
	  key2 = XCAR (pair);
	  value = XCDR (pair);
	}
      else
	{
	  key2 = XCAR (tail);
	  tail = XCDR (tail);
	  CHECK_CONS (tail);
	  value = XCAR (tail);
	}
      key = key2;
      let key: Lisp_Object = maybe_remove_pos_from_symbol (key);
      CHECK_TYPE (BARE_SYMBOL_P (key), Qsymbolp (), key);

      if symset_add (jo, &raw mut ss, key)
	{
	  if !first
	    { json_out_byte (jo, b','); }
	  first = false;

	  let key_str: Lisp_Object = SYMBOL_NAME (key);
	  let str: *const c_char = SSDATA (key_str);
	  /* Skip leading ':' in plist keys.  */
	  let skip: c_int = if !is_alist && *str == b':' as c_char
			      && *str.offset (1) != 0 { 1 } else { 0 };
	  json_out_string (jo, key_str, skip);
	  json_out_byte (jo, b':');
	  json_out_something (jo, value);
	}
  });
  CHECK_LIST_END (tail, obj);
  json_out_byte (jo, 0x7d);
  pop_symset (jo, &raw mut ss);
  json_out_unnest (jo);
}

unsafe fn json_out_object_hash (jo: *mut json_out_t, obj: Lisp_Object)
{
  json_out_nest (jo);
  json_out_byte (jo, 0x7b);
  let h: *mut Lisp_Hash_Table = XHASH_TABLE (obj);
  let mut first: bool = true;
  DOHASH! (h, k, v, {
      if !first
	{ json_out_byte (jo, b','); }
      first = false;
      CHECK_STRING (k);
      /* It's the user's responsibility to ensure that hash keys are
	 unique; we don't check for it.  */
      json_out_string (jo, k, 0);
      json_out_byte (jo, b':');
      json_out_something (jo, v);
  });
  json_out_byte (jo, 0x7d);
  json_out_unnest (jo);

}

unsafe fn json_out_array (jo: *mut json_out_t, obj: Lisp_Object)
{
  json_out_nest (jo);
  json_out_byte (jo, b'[');
  let n: ptrdiff_t = ASIZE (obj);
  let mut i: ptrdiff_t = 0;
  while i < n
    {
      if i > 0
	{ json_out_byte (jo, b','); }
      json_out_something (jo, AREF (obj, i));
      i += 1;
    }
  json_out_byte (jo, b']');
  json_out_unnest (jo);
}

unsafe fn json_out_float (jo: *mut json_out_t, f: Lisp_Object)
{
  let x: c_double = XFLOAT_DATA (f);
  if !isfinite! (x)
    { signal_error (c"JSON does not allow Inf or NaN".as_ptr (), f); }
  /* As luck has it, float_to_string emits correct JSON float syntax for
     all numbers (because Vfloat_output_format is Qnil).  */
  json_make_room (jo, FLOAT_TO_STRING_BUFSIZE as ptrdiff_t);
  let n: c_int = float_to_string ((*jo).buf.offset ((*jo).size as isize), x);
  (*jo).size += n as ptrdiff_t;
}

unsafe fn json_out_bignum (jo: *mut json_out_t, x: Lisp_Object)
{
  let base: c_int = 10;
  let size: ptrdiff_t = bignum_bufsize (x, base);
  json_make_room (jo, size);
  let n: ptrdiff_t = bignum_to_c_string ((*jo).buf.offset ((*jo).size as isize), size, x, base);
  (*jo).size += n;
}

unsafe fn json_out_something (jo: *mut json_out_t, obj: Lisp_Object)
{
  if EQ (obj, (*jo).conf.null_object)
    { JSON_OUT_STR! (jo, c"null"); }
  else if EQ (obj, (*jo).conf.false_object)
    { JSON_OUT_STR! (jo, c"false"); }
  else if EQ (obj, Qt ())
    { JSON_OUT_STR! (jo, c"true"); }
  else if NILP (obj)
    { JSON_OUT_STR! (jo, c"{}"); }
  else if FIXNUMP (obj)
    { json_out_fixnum (jo, XFIXNUM (obj)); }
  else if STRINGP (obj)
    { json_out_string (jo, obj, 0); }
  else if CONSP (obj)
    { json_out_object_cons (jo, obj); }
  else if FLOATP (obj)
    { json_out_float (jo, obj); }
  else if HASH_TABLE_P (obj)
    { json_out_object_hash (jo, obj); }
  else if VECTORP (obj)
    { json_out_array (jo, obj); }
  else if BIGNUMP (obj)
    { json_out_bignum (jo, obj); }
  else
    { wrong_type_argument (Qjson_value_p (), obj); }
}

unsafe fn json_serialize (jo: *mut json_out_t, object: Lisp_Object,
			  nargs: ptrdiff_t, args: *mut Lisp_Object)
{
  (*jo).maxdepth = 50;
  (*jo).size = 0;
  (*jo).capacity = 0;
  (*jo).chars_delta = 0;
  (*jo).buf = std::ptr::null_mut ();
  (*jo).ss_table = std::ptr::null_mut ();
  (*jo).conf.object_type = json_object_hashtable;
  (*jo).conf.array_type = json_array_array;
  (*jo).conf.null_object = QCnull ();
  (*jo).conf.false_object = QCfalse ();

  json_parse_args (nargs, args, &raw mut (*jo).conf, false);
  record_unwind_protect_ptr (Some (cleanup_json_out), jo as *mut c_void);

  /* Make float conversion independent of float-output-format.  */
  if !NILP (Vfloat_output_format)
    { specbind (Qfloat_output_format (), Qnil ()); }

  json_out_something (jo, object);
}

DEFUN! ("json-serialize", Fjson_serialize, Sjson_serialize, 1, MANY,
       NULL,
       doc: "Return the JSON representation of OBJECT as a unibyte string.\n\nOBJECT is translated as follows:\n\n`t'        -- the JSON `true' value.\nnumber     -- a JSON number.\nstring     -- a JSON string.\nvector     -- a JSON array.\nhash-table -- a JSON object.  Keys must be strings.\nalist      -- a JSON object.  Keys must be symbols.\nplist      -- a JSON object.  Keys must be symbols.\n              A leading colon in plist key names is elided.\n\nFor duplicate object keys, the first value is used.\n\nThe Lisp equivalents to the JSON null and false values are\nconfigurable in the arguments ARGS, a list of keyword/argument pairs:\n\n:null-object OBJ -- use OBJ to represent a JSON null value.\n  It defaults to `:null'.\n\n:false-object OBJ -- use OBJ to represent a JSON false value.\n  It defaults to `:false'.\n\nIn you specify the same value for `:null-object' and `:false-object',\na potentially ambiguous situation, the JSON output will not contain\nany JSON false values.\nusage: (json-serialize OBJECT &rest ARGS)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut jo: json_out_t = std::mem::zeroed ();
  json_serialize (&raw mut jo, *args.offset (0), nargs - 1, args.offset (1));
  return unbind_to (count, make_unibyte_string (jo.buf, jo.size));
});

DEFUN! ("json-insert", Fjson_insert, Sjson_insert, 1, MANY,
       NULL,
       doc: "Insert the JSON representation of OBJECT before point.\nThis is the same as (insert (json-serialize OBJECT ...)), but potentially\nfaster, and with the difference that Unicode characters are inserted as\nthemselves into multibyte buffers, and as UTF-8 byte sequences into\nunibyte buffers.\nSee the function `json-serialize' for allowed values of OBJECT and ARGS.\nusage: (json-insert OBJECT &rest ARGS)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut jo: json_out_t = std::mem::zeroed ();
  json_serialize (&raw mut jo, *args.offset (0), nargs - 1, args.offset (1));

  prepare_to_modify_buffer (PT! (), PT! (), std::ptr::null_mut ());
  move_gap_both (PT! (), PT_BYTE! ());
  if GAP_SIZE! () < jo.size
    { make_gap (jo.size - GAP_SIZE! ()); }
  std::ptr::copy_nonoverlapping (jo.buf as *const c_void, GPT_ADDR! () as *mut c_void,
		      jo.size as usize);

  /* No need to keep allocation beyond this point.  */
  unbind_to (count, Qnil ());

  let ub_buffer: bool = NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
  let inserted_bytes: ptrdiff_t = jo.size;
  let inserted: ptrdiff_t = if ub_buffer { jo.size } else { jo.size - jo.chars_delta };
  eassert! (inserted > 0);

  insert_from_gap_1 (inserted, inserted_bytes, false);
  invalidate_buffer_caches ((current_buffer! ()) as *mut c_void, PT! (), PT! () + inserted);
  adjust_after_insert (PT! (), PT_BYTE! (), PT! () + inserted, PT_BYTE! () + inserted_bytes,
		       inserted);

  /* Call after-change hooks.  */
  signal_after_change (PT! (), 0, inserted);

  update_compositions (PT! (), PT! (), CHECK_BORDER);
  /* Move point to after the inserted text.  */
  SET_PT_BOTH (PT! () + inserted, PT_BYTE! () + inserted_bytes);

  return Qnil ();
});

/* C 672-673.  */
pub const JSON_PARSER_INTERNAL_OBJECT_WORKSPACE_SIZE: usize = 64;
pub const JSON_PARSER_INTERNAL_BYTE_WORKSPACE_SIZE: usize = 512;

#[repr(C)]
pub struct json_parser
{
  /* Because of a possible gap in the input (an emacs buffer can have
     a gap), the input is described by [input_begin;input_end) and
     [secondary_input_begin;secondary_input_end).  If the input is
     continuous, then secondary_input_begin and secondary_input_end
     should be NULL */
  pub input_current: *const u8,
  pub input_begin: *const u8,
  pub input_end: *const u8,

  pub secondary_input_begin: *const u8,
  pub secondary_input_end: *const u8,

  /* The parser has a maximum allowed depth.  available_depth
     decreases at each object/array begin.  If reaches zero, then an
     error is generated */
  pub available_depth: c_int,

  pub conf: json_configuration,

  pub additional_bytes_count: ptrdiff_t,

  /* Lisp_Objects are collected in this area during object/array
     parsing.  To avoid allocations, initially
     internal_object_workspace is used.  If it runs out of space then
     we switch to allocated space.  Important note: with this design,
     GC must not run during JSON parsing, otherwise Lisp_Objects in
     the workspace may get incorrectly collected. */
  pub internal_object_workspace: [Lisp_Object; JSON_PARSER_INTERNAL_OBJECT_WORKSPACE_SIZE],
  pub object_workspace: *mut Lisp_Object,
  pub object_workspace_size: ptrdiff_t,
  pub object_workspace_current: ptrdiff_t,

  /* String and number parsing uses this workspace.  The idea behind
     internal_byte_workspace is the same as the idea behind
     internal_object_workspace */
  pub internal_byte_workspace: [u8; JSON_PARSER_INTERNAL_BYTE_WORKSPACE_SIZE],
  pub byte_workspace: *mut u8,
  pub byte_workspace_end: *mut u8,
  pub byte_workspace_current: *mut u8,

  pub obj: Lisp_Object,
  pub byte_to_pos: Option<unsafe extern "C-unwind" fn (Lisp_Object, ptrdiff_t) -> ptrdiff_t>,
  pub byte_to_line: Option<unsafe extern "C-unwind" fn (Lisp_Object, ptrdiff_t) -> ptrdiff_t>,
}

unsafe fn json_signal_error (p: *mut json_parser, error: Lisp_Object) -> !
{
  let byte: ptrdiff_t = ((*p).input_current as ptrdiff_t) - ((*p).input_begin as ptrdiff_t)
    + (*p).additional_bytes_count;
  let pos: ptrdiff_t = ((*p).byte_to_pos.unwrap ()) ((*p).obj, byte);
  let line: ptrdiff_t = ((*p).byte_to_line.unwrap ()) ((*p).obj, byte) + 1;
  /* The line number here is deprecated and provided for compatibility only.
     It is scheduled for removal in Emacs 32.  */
  xsignal3 (error, INT_TO_INTEGER (line), Qnil (), INT_TO_INTEGER (pos));
}

unsafe fn json_parser_init (parser: *mut json_parser,
			    conf: json_configuration,
			    input: *const u8,
			    input_end: *const u8,
			    secondary_input: *const u8,
			    secondary_input_end: *const u8,
			    byte_to_pos: unsafe extern "C-unwind" fn (Lisp_Object, ptrdiff_t)
			      -> ptrdiff_t,
			    byte_to_line: unsafe extern "C-unwind" fn (Lisp_Object, ptrdiff_t)
			      -> ptrdiff_t,
			    obj: Lisp_Object)
{
  let mut secondary_input: *const u8 = secondary_input;
  let mut secondary_input_end: *const u8 = secondary_input_end;
  if secondary_input >= secondary_input_end
    {
      secondary_input = std::ptr::null ();
      secondary_input_end = std::ptr::null ();
    }

  if input < input_end
    {
      (*parser).input_begin = input;
      (*parser).input_end = input_end;

      (*parser).secondary_input_begin = secondary_input;
      (*parser).secondary_input_end = secondary_input_end;
    }
  else
    {
      (*parser).input_begin = secondary_input;
      (*parser).input_end = secondary_input_end;

      (*parser).secondary_input_begin = std::ptr::null ();
      (*parser).secondary_input_end = std::ptr::null ();
    }

  (*parser).input_current = (*parser).input_begin;

  (*parser).available_depth = 10000;
  (*parser).conf = conf;

  (*parser).additional_bytes_count = 0;

  (*parser).object_workspace = (*parser).internal_object_workspace.as_mut_ptr ();
  (*parser).object_workspace_size
    = JSON_PARSER_INTERNAL_OBJECT_WORKSPACE_SIZE as ptrdiff_t;
  (*parser).object_workspace_current = 0;

  (*parser).byte_workspace = (*parser).internal_byte_workspace.as_mut_ptr ();
  (*parser).byte_workspace_end = (*parser).byte_workspace
    .offset (JSON_PARSER_INTERNAL_BYTE_WORKSPACE_SIZE as isize);
  (*parser).byte_to_pos = Some (byte_to_pos);
  (*parser).byte_to_line = Some (byte_to_line);
  (*parser).obj = obj;
}

pub unsafe extern "C-unwind" fn json_parser_done (parser: *mut c_void)
{
  let p: *mut json_parser = parser as *mut json_parser;
  if (*p).object_workspace != (*p).internal_object_workspace.as_mut_ptr ()
    { xfree ((*p).object_workspace as *mut c_void); }
  if (*p).byte_workspace != (*p).internal_byte_workspace.as_mut_ptr ()
    { xfree ((*p).byte_workspace as *mut c_void); }
}

/* Makes sure that the object_workspace has 'size' available space for
   Lisp_Objects */
unsafe fn json_make_object_workspace_for_slow_path (parser: *mut json_parser,
						    size: ptrdiff_t)
{
  let internal: bool = (*parser).object_workspace_size
    == JSON_PARSER_INTERNAL_OBJECT_WORKSPACE_SIZE as ptrdiff_t;
  let new_workspace_ptr: *mut Lisp_Object
    = xpalloc (if internal { std::ptr::null_mut () }
	       else { (*parser).object_workspace as *mut c_void },
	       &raw mut (*parser).object_workspace_size,
	       size - ((*parser).object_workspace_size
		       - (*parser).object_workspace_current),
	       -1, size_of::<Lisp_Object> () as ptrdiff_t) as *mut Lisp_Object;
  if internal
    { std::ptr::copy_nonoverlapping ((*parser).object_workspace as *const c_void,
			   new_workspace_ptr as *mut c_void,
			   (size_of::<Lisp_Object> ()
			    * (*parser).object_workspace_current as usize)); }
  (*parser).object_workspace = new_workspace_ptr;
}

unsafe fn json_make_object_workspace_for (parser: *mut json_parser,
					  size: ptrdiff_t)
{
  if (*parser).object_workspace_size - (*parser).object_workspace_current
      < size
    {
      json_make_object_workspace_for_slow_path (parser, size);
    }
}

unsafe fn json_byte_workspace_reset (parser: *mut json_parser)
{
  (*parser).byte_workspace_current = (*parser).byte_workspace;
}

/* Put VALUE into the byte_workspace, allocating space.  */
unsafe fn json_byte_workspace_put_slow_path (parser: *mut json_parser,
					     value: u8)
{
  let mut new_workspace_size: ptrdiff_t
    = ((*parser).byte_workspace_end as ptrdiff_t)
      - ((*parser).byte_workspace as ptrdiff_t);
  let offset: ptrdiff_t
    = ((*parser).byte_workspace_current as ptrdiff_t)
      - ((*parser).byte_workspace as ptrdiff_t);

  let internal: bool = (*parser).byte_workspace == (*parser).internal_byte_workspace.as_mut_ptr ();
  let new: *mut u8
    = xpalloc (if internal { std::ptr::null_mut () }
	       else { (*parser).byte_workspace as *mut c_void },
	       &raw mut new_workspace_size, 1, -1, 1) as *mut u8;
  if internal
    { std::ptr::copy_nonoverlapping ((*parser).byte_workspace as *const c_void,
				   new as *mut c_void,
			   offset as usize); }
  (*parser).byte_workspace = new;
  (*parser).byte_workspace_end = (*parser).byte_workspace.offset (new_workspace_size as isize);
  (*parser).byte_workspace_current = (*parser).byte_workspace.offset (offset as isize);
  *(*parser).byte_workspace_current = value;
  (*parser).byte_workspace_current = (*parser).byte_workspace_current.offset (1);
}

unsafe fn json_byte_workspace_put (parser: *mut json_parser, value: u8)
{
  if (*parser).byte_workspace_current < (*parser).byte_workspace_end
    {
      *(*parser).byte_workspace_current = value;
      (*parser).byte_workspace_current = (*parser).byte_workspace_current.offset (1);
    }
  else
    {
      json_byte_workspace_put_slow_path (parser, value);
    }
}

unsafe fn json_input_at_eof (parser: *mut json_parser) -> bool
{
  if (*parser).input_current < (*parser).input_end
    { return false; }
  return (*parser).secondary_input_end.is_null ();
}

/* If there is a secondary buffer, this switches to it */
unsafe fn json_input_switch_to_secondary (parser: *mut json_parser) -> c_int
{
  if (*parser).secondary_input_begin < (*parser).secondary_input_end
    {
      (*parser).additional_bytes_count
	= ((*parser).input_end as ptrdiff_t) - ((*parser).input_begin as ptrdiff_t);
      (*parser).input_begin = (*parser).secondary_input_begin;
      (*parser).input_end = (*parser).secondary_input_end;
      (*parser).input_current = (*parser).secondary_input_begin;
      (*parser).secondary_input_begin = std::ptr::null ();
      (*parser).secondary_input_end = std::ptr::null ();
      return 0;
    }
  else
    { return -1; }
}

/* Reads a byte from the JSON input stream */
unsafe fn json_input_get_slow_path (parser: *mut json_parser) -> u8
{
  if json_input_switch_to_secondary (parser) < 0
    { json_signal_error (parser, Qjson_end_of_file ()); }
  let c: u8 = *(*parser).input_current;
  (*parser).input_current = (*parser).input_current.offset (1);
  return c;
}

unsafe fn json_input_get (parser: *mut json_parser) -> u8
{
  if (*parser).input_current < (*parser).input_end
    {
      let c: u8 = *(*parser).input_current;
      (*parser).input_current = (*parser).input_current.offset (1);
      return c;
    }
  return json_input_get_slow_path (parser);
}

/* Reads a byte from the JSON input stream, if the stream is not at
 * eof.  At eof, returns -1 */
unsafe fn json_input_get_if_possible (parser: *mut json_parser) -> c_int
{
  if (*parser).input_current >= (*parser).input_end
      && json_input_switch_to_secondary (parser) < 0
    { return -1; }
  let c: u8 = *(*parser).input_current;
  (*parser).input_current = (*parser).input_current.offset (1);
  return c as c_int;
}

/* Puts back the last read input byte.  Only one byte can be put back,
   because otherwise this code would need to handle switching from
   the secondary buffer to the initial */
unsafe fn json_input_put_back (parser: *mut json_parser)
{
  (*parser).input_current = (*parser).input_current.offset (-1);
}

unsafe fn is_json_whitespace (c: c_int) -> bool
{
  return c == 0x20 || c == 0x09 || c == 0x0d || c == 0x0a;
}

/* Skips JSON whitespace, and returns with the first non-whitespace
 * character */
unsafe fn json_skip_whitespace (parser: *mut json_parser) -> c_int
{
  loop
    {
      let c: c_int = json_input_get (parser) as c_int;
      if !is_json_whitespace (c)
	{ return c; }
    }
}

/* Skips JSON whitespace, and returns with the first non-whitespace
 * character, if possible.  If there is no non-whitespace character
 * (because we reached the end), it returns -1 */
unsafe fn json_skip_whitespace_if_possible (parser: *mut json_parser) -> c_int
{
  loop
    {
      let c: c_int = json_input_get_if_possible (parser);
      if !is_json_whitespace (c) || c < 0
	{ return c; }
    }
}

unsafe fn json_hex_value (c: c_int) -> c_int
{
  if c >= b'0' as c_int && c <= b'9' as c_int
    { return c - b'0' as c_int; }
  else if c >= b'A' as c_int && c <= b'F' as c_int
    { return c - b'A' as c_int + 10; }
  else if c >= b'a' as c_int && c <= b'f' as c_int
    { return c - b'a' as c_int + 10; }
  else
    { return -1; }
}

/* Parses the CCCC part of the unicode escape sequence \uCCCC */
unsafe fn json_parse_unicode (parser: *mut json_parser) -> c_int
{
  let mut v: [u8; 4] = [0; 4];
  let mut i: c_int = 0;
  while i < 4
    {
      let c: c_int = json_hex_value (json_input_get (parser) as c_int);
      if c < 0
	{ json_signal_error (parser, Qjson_escape_sequence_error ()); }
      v[i as usize] = c as u8;
      i += 1;
    }

  return ((v[0] as c_int) << 12) | ((v[1] as c_int) << 8) | ((v[2] as c_int) << 4)
    | (v[3] as c_int);
}

unsafe fn utf8_error (parser: *mut json_parser) -> !
{
  json_signal_error (parser, Qjson_utf8_decode_error ());
}

/* Parse a string literal.  Optionally prepend a ':'.
   Return the string or an interned symbol.  */
unsafe fn json_parse_string (parser: *mut json_parser, intern: bool,
			     leading_colon: bool) -> Lisp_Object
{
  json_byte_workspace_reset (parser);
  if leading_colon
    { json_byte_workspace_put (parser, b':'); }
  let mut chars_delta: ptrdiff_t = 0;	/* nbytes - nchars */
  loop
    {
      /* This if is only here for a possible speedup.  If there are 4
	 bytes available, and all of them are single_uninteresting,
	 then we can just copy these 4 bytes to output */
      if ((*parser).input_end as ptrdiff_t) - ((*parser).input_current as ptrdiff_t) >= 4
	{
	  let c0: c_int = *(*parser).input_current as c_int;
	  let c1: c_int = *(*parser).input_current.offset (1) as c_int;
	  let c2: c_int = *(*parser).input_current.offset (2) as c_int;
	  let c3: c_int = *(*parser).input_current.offset (3) as c_int;
	  let v0: bool = json_plain_char[c0 as usize] != 0;
	  let v1: bool = json_plain_char[c1 as usize] != 0;
	  let v2: bool = json_plain_char[c2 as usize] != 0;
	  let v3: bool = json_plain_char[c3 as usize] != 0;
	  if v0 && v1 && v2 && v3
	    {
	      json_byte_workspace_put (parser, c0 as u8);
	      json_byte_workspace_put (parser, c1 as u8);
	      json_byte_workspace_put (parser, c2 as u8);
	      json_byte_workspace_put (parser, c3 as u8);
	      (*parser).input_current = (*parser).input_current.offset (4);
	      continue;
	    }
	}

      let mut c: c_int = json_input_get (parser) as c_int;
      if json_plain_char[c as usize] != 0
	{
	  json_byte_workspace_put (parser, c as u8);
	  continue;
	}

      if c == 0x22 as c_int
	{
	  let nbytes: ptrdiff_t
	    = ((*parser).byte_workspace_current as ptrdiff_t)
	      - ((*parser).byte_workspace as ptrdiff_t);
	  let nchars: ptrdiff_t = nbytes - chars_delta;
	  let str: *const c_char = (*parser).byte_workspace as *const c_char;
	  return if intern
		 { intern_c_multibyte (str, nchars, nbytes) }
		 else
		 { make_multibyte_string (str, nchars, nbytes) };
	}

      if c & 0x80 != 0
	{
	  /* Parse UTF-8, strictly.  This is the correct thing to do
	     whether the input is a unibyte or multibyte string.  */
	  json_byte_workspace_put (parser, c as u8);
	  let c1: u8 = json_input_get (parser);
	  if (c1 as c_int & 0xc0) != 0x80
	    { utf8_error (parser); }
	  json_byte_workspace_put (parser, c1);
	  if c <= 0xc1
	    { utf8_error (parser); }
	  else if c <= 0xdf
	    { chars_delta += 1; }
	  else if c <= 0xef
	    {
	      let c2: u8 = json_input_get (parser);
	      if (c2 as c_int & 0xc0) != 0x80
		{ utf8_error (parser); }
	      let v: c_int = ((c & 0x0f) << 12) + ((c1 as c_int & 0x3f) << 6)
		+ (c2 as c_int & 0x3f);
	      if v < 0x800 || (v >= 0xd800 && v <= 0xdfff)
		{ utf8_error (parser); }
	      json_byte_workspace_put (parser, c2);
	      chars_delta += 2;
	    }
	  else if c <= 0xf7
	    {
	      let c2: u8 = json_input_get (parser);
	      let c3: u8 = json_input_get (parser);
	      if (c2 as c_int & 0xc0) != 0x80 || (c3 as c_int & 0xc0) != 0x80
		{ utf8_error (parser); }
	      let v: c_int = ((c & 0x07) << 18) + ((c1 as c_int & 0x3f) << 12)
		+ ((c2 as c_int & 0x3f) << 6) + (c3 as c_int & 0x3f);
	      if v < 0x10000 || v > 0x10ffff
		{ utf8_error (parser); }
	      json_byte_workspace_put (parser, c2);
	      json_byte_workspace_put (parser, c3);
	      chars_delta += 3;
	    }
	  else
	    { utf8_error (parser); }
	}
      else if c == b'\\' as c_int
	{
	  /* Handle escape sequences */
	  c = json_input_get (parser) as c_int;
	  if c == 0x22 as c_int
	    { json_byte_workspace_put (parser, 0x22); }
	  else if c == b'\\' as c_int
	    { json_byte_workspace_put (parser, b'\\'); }
	  else if c == b'/' as c_int
	    { json_byte_workspace_put (parser, b'/'); }
	  else if c == b'b' as c_int
	    { json_byte_workspace_put (parser, 0x08); }
	  else if c == b'f' as c_int
	    { json_byte_workspace_put (parser, 0x0c); }
	  else if c == b'n' as c_int
	    { json_byte_workspace_put (parser, 0x0a); }
	  else if c == b'r' as c_int
	    { json_byte_workspace_put (parser, 0x0d); }
	  else if c == b't' as c_int
	    { json_byte_workspace_put (parser, 0x09); }
	  else if c == b'u' as c_int
	    {
	      let mut num: c_int = json_parse_unicode (parser);
	      /* is the first half of the surrogate pair */
	      if num >= 0xd800 && num < 0xdc00
		{
		  if json_input_get (parser) as c_int != b'\\' as c_int
		    { json_signal_error (parser,
					 Qjson_invalid_surrogate_error ()); }
		  if json_input_get (parser) as c_int != b'u' as c_int
		    { json_signal_error (parser,
					 Qjson_invalid_surrogate_error ()); }
		  let num2: c_int = json_parse_unicode (parser);
		  if num2 < 0xdc00 || num2 >= 0xe000
		    { json_signal_error (parser,
					 Qjson_invalid_surrogate_error ()); }
		  num = 0x10000 + (((num - 0xd800) << 10) | (num2 - 0xdc00));
		}
	      else if num >= 0xdc00 && num < 0xe000
		/* is the second half of the surrogate pair without
		   the first half */
		{ json_signal_error (parser,
				     Qjson_invalid_surrogate_error ()); }

	      /* utf-8 encode the code-point */
	      if num < 0x80
		{ json_byte_workspace_put (parser, num as u8); }
	      else if num < 0x800
		{
		  json_byte_workspace_put (parser, (0xc0 | num >> 6) as u8);
		  json_byte_workspace_put (parser, (0x80 | (num & 0x3f)) as u8);
		  chars_delta += 1;
		}
	      else if num < 0x10000
		{
		  json_byte_workspace_put (parser, (0xe0 | num >> 12) as u8);
		  json_byte_workspace_put (parser, (0x80 | ((num >> 6) & 0x3f)) as u8);
		  json_byte_workspace_put (parser, (0x80 | (num & 0x3f)) as u8);
		  chars_delta += 2;
		}
	      else
		{
		  json_byte_workspace_put (parser, (0xf0 | num >> 18) as u8);
		  json_byte_workspace_put (parser, (0x80 | ((num >> 12) & 0x3f)) as u8);
		  json_byte_workspace_put (parser, (0x80 | ((num >> 6) & 0x3f)) as u8);
		  json_byte_workspace_put (parser, (0x80 | (num & 0x3f)) as u8);
		  chars_delta += 3;
		}
	    }
	  else
	    { json_signal_error (parser, Qjson_escape_sequence_error ()); }
	}
      else
	{ json_signal_error (parser, Qjson_parse_error ()); }
    }
}

/* If there was no integer overflow during parsing the integer, this
   puts 'value' to the output. Otherwise this calls string_to_number
   to parse integer on the byte workspace.  This could just always
   call string_to_number, but for performance reasons, during parsing
   the code tries to calculate the value, so in most cases, we can
   save call of string_to_number */
unsafe fn json_create_integer (parser: *mut json_parser,
			       integer_overflow: bool, negative: bool,
			       value: EMACS_UINT) -> Lisp_Object
{
  if !integer_overflow
    {
      if negative
	{
	  let v: u64 = value as u64;
	  if v <= (INTMAX_MAX as u64) + 1
	    { return INT_TO_INTEGER (-(v as isize)); }
	}
      else
	{ return INT_TO_INTEGER (value as isize); }
    }

  json_byte_workspace_put (parser, 0);
  let mut len: ptrdiff_t = 0;
  let result: Lisp_Object
    = string_to_number ((*parser).byte_workspace as *const c_char, 10, &raw mut len);
  if len != ((*parser).byte_workspace_current as ptrdiff_t)
      - ((*parser).byte_workspace as ptrdiff_t) - 1
    { json_signal_error (parser, Qjson_error ()); }
  return result;
}

/* Parses a float using the byte workspace */
unsafe fn json_create_float (parser: *mut json_parser) -> Lisp_Object
{
  json_byte_workspace_put (parser, 0);
  *errno () = 0;
  let mut e: *mut c_char = std::ptr::null_mut ();
  let value: c_double = strtod ((*parser).byte_workspace as *const c_char,
				       &raw mut e);
  let out_of_range: bool = *errno () != 0
    && (value == HUGE_VAL || value == -HUGE_VAL);
  if out_of_range
    { json_signal_error (parser, Qjson_number_out_of_range ()); }
  else if (e as *const u8) != (*parser).byte_workspace_current.offset (-1)
    { json_signal_error (parser, Qjson_error ()); }
  else
    { return make_float (value); }
}

/* Parses a number.  The first character is the input parameter 'c'.
 */
unsafe fn json_parse_number (parser: *mut json_parser, c_in: c_int) -> Lisp_Object
{
  let mut c: c_int = c_in;
  json_byte_workspace_reset (parser);
  json_byte_workspace_put (parser, c as u8);

  let mut negative: bool = false;
  if c == b'-' as c_int
    {
      negative = true;
      c = json_input_get (parser) as c_int;
      json_byte_workspace_put (parser, c as u8);
    }
  if c < b'0' as c_int || c > b'9' as c_int
    { json_signal_error (parser, Qjson_parse_error ()); }

  /* The idea is that during finding the last character of the
     number, the for loop below also tries to calculate the value.  If
     the parsed number is an integer which fits into unsigned long,
     then the parser can use the value of 'integer' right away,
     instead of having to re-parse the byte workspace later.
     Ideally, this integer should have the same size as a CPU general
     purpose register. */
  let mut integer: EMACS_UINT = (c - b'0' as c_int) as EMACS_UINT;
  let mut integer_overflow: bool = false;

  if integer == 0
    {
      if json_input_at_eof (parser)
	{ return INT_TO_INTEGER (0); }
      c = json_input_get (parser) as c_int;
    }
  else
    {
      loop
	{
	  if json_input_at_eof (parser)
	    { return json_create_integer (parser, integer_overflow,
					  negative, integer); }
	  c = json_input_get (parser) as c_int;
	  if c < b'0' as c_int || c > b'9' as c_int
	    { break; }
	  json_byte_workspace_put (parser, c as u8);

	  let (product, mul_overflowed) = integer.overflowing_mul (10);
	  integer = product;
	  integer_overflow |= mul_overflowed;
	  let (sum, add_overflowed) = integer.overflowing_add ((c - b'0' as c_int) as EMACS_UINT);
	  integer = sum;
	  integer_overflow |= add_overflowed;
	}
    }

  let mut is_float: bool = false;
  if c == b'.' as c_int
    {
      json_byte_workspace_put (parser, c as u8);

      is_float = true;
      c = json_input_get (parser) as c_int;
      json_byte_workspace_put (parser, c as u8);
      if c < b'0' as c_int || c > b'9' as c_int
	{ json_signal_error (parser, Qjson_parse_error ()); }
      loop
	{
	  if json_input_at_eof (parser)
	    { return json_create_float (parser); }
	  c = json_input_get (parser) as c_int;
	  if c < b'0' as c_int || c > b'9' as c_int
	    { break; }
	  json_byte_workspace_put (parser, c as u8);
	}
    }
  if c == b'e' as c_int || c == b'E' as c_int
    {
      json_byte_workspace_put (parser, c as u8);

      is_float = true;
      c = json_input_get (parser) as c_int;
      json_byte_workspace_put (parser, c as u8);
      if c == b'-' as c_int || c == b'+' as c_int
	{
	  c = json_input_get (parser) as c_int;
	  json_byte_workspace_put (parser, c as u8);
	}
      if c < b'0' as c_int || c > b'9' as c_int
	{ json_signal_error (parser, Qjson_parse_error ()); }
      loop
	{
	  if json_input_at_eof (parser)
	    { return json_create_float (parser); }
	  c = json_input_get (parser) as c_int;
	  if c < b'0' as c_int || c > b'9' as c_int
	    { break; }
	  json_byte_workspace_put (parser, c as u8);
	}
    }

  /* 'c' contains a character which is not part of the number,
     so it is need to be put back */
  json_input_put_back (parser);

  if is_float
    { return json_create_float (parser); }
  else
    { return json_create_integer (parser, integer_overflow, negative,
				  integer); }
}

/* C 1342-1343's `static Lisp_Object json_parse_value (…);` has no counterpart: Rust items are
   order-independent.  */

/* Parses a JSON array. */
unsafe fn json_parse_array (parser: *mut json_parser) -> Lisp_Object
{
  let mut c: c_int = json_skip_whitespace (parser);

  let first: ptrdiff_t = (*parser).object_workspace_current;
  let mut result: Lisp_Object = Qnil ();

  if c != b']' as c_int
    {
      (*parser).available_depth -= 1;
      if (*parser).available_depth < 0
	{ json_signal_error (parser, Qjson_object_too_deep ()); }

      let mut cdr: *mut Lisp_Object = &raw mut result;
      /* This loop collects the array elements in the object workspace
       */
      loop
	{
	  let element: Lisp_Object = json_parse_value (parser, c);
	  match (*parser).conf.array_type
	    {
	    json_array_array =>
	      {
		json_make_object_workspace_for (parser, 1);
		*(*parser).object_workspace.offset ((*parser).object_workspace_current as isize)
		  = element;
		(*parser).object_workspace_current += 1;
	      }
	    json_array_list =>
	      {
		let nc: Lisp_Object = Fcons (element, Qnil ());
		*cdr = nc;
		cdr = xcdr_addr (nc);
	      }
	    _ =>
	      { emacs_abort (); }
	    }

	  c = json_skip_whitespace (parser);
	  if c == b']' as c_int
	    {
	      (*parser).available_depth += 1;
	      break;
	    }

	  if c != b',' as c_int
	    { json_signal_error (parser, Qjson_parse_error ()); }

	  c = json_skip_whitespace (parser);
	}
    }

  match (*parser).conf.array_type
    {
    json_array_array =>
      {
	let number_of_elements: ptrdiff_t
	  = (*parser).object_workspace_current - first;
	result = make_vector (number_of_elements, Qnil ());
	let mut i: ptrdiff_t = 0;
	while i < number_of_elements
	  {
	    rarely_quit (i as u64);
	    ASET (result, i, *(*parser).object_workspace.offset ((first + i) as isize));
	    i += 1;
	  }
	(*parser).object_workspace_current = first;
      }
    json_array_list =>
      { }
    _ =>
      { emacs_abort (); }
    }

  return result;
}

/* Parses the ": value" part of a JSON object member. */
unsafe fn json_parse_object_member_value (parser: *mut json_parser) -> Lisp_Object
{
  let mut c: c_int = json_skip_whitespace (parser);
  if c != b':' as c_int
    { json_signal_error (parser, Qjson_parse_error ()); }

  c = json_skip_whitespace (parser);

  return json_parse_value (parser, c);
}

/* Parses a JSON object. */
unsafe fn json_parse_object (parser: *mut json_parser) -> Lisp_Object
{
  let mut c: c_int = json_skip_whitespace (parser);

  let first: ptrdiff_t = (*parser).object_workspace_current;
  let mut result: Lisp_Object = Qnil ();

  if c != 0x7d as c_int
    {
      (*parser).available_depth -= 1;
      if (*parser).available_depth < 0
	{ json_signal_error (parser, Qjson_object_too_deep ()); }

      let mut cdr: *mut Lisp_Object = &raw mut result;

      /* This loop collects the object members (key/value pairs) in
       * the object workspace */
      loop
	{
	  if c != 0x22 as c_int
	    { json_signal_error (parser, Qjson_parse_error ()); }

	  match (*parser).conf.object_type
	    {
	    json_object_hashtable =>
	      {
		let key: Lisp_Object = json_parse_string (parser, false, false);
		let value: Lisp_Object = json_parse_object_member_value (parser);
		json_make_object_workspace_for (parser, 2);
		*(*parser).object_workspace.offset ((*parser).object_workspace_current
						    as isize) = key;
		(*parser).object_workspace_current += 1;
		*(*parser).object_workspace.offset ((*parser).object_workspace_current
						    as isize) = value;
		(*parser).object_workspace_current += 1;
	      }
	    json_object_alist =>
	      {
		let key: Lisp_Object = json_parse_string (parser, true, false);
		let value: Lisp_Object = json_parse_object_member_value (parser);
		let nc: Lisp_Object = Fcons (Fcons (key, value), Qnil ());
		*cdr = nc;
		cdr = xcdr_addr (nc);
	      }
	    json_object_plist =>
	      {
		let key: Lisp_Object = json_parse_string (parser, true, true);
		let value: Lisp_Object = json_parse_object_member_value (parser);
		let nc: Lisp_Object = Fcons (key, Qnil ());
		*cdr = nc;
		cdr = xcdr_addr (nc);

		let nc: Lisp_Object = Fcons (value, Qnil ());
		*cdr = nc;
		cdr = xcdr_addr (nc);
	      }
	    _ =>
	      { emacs_abort (); }
	    }

	  c = json_skip_whitespace (parser);

	  if c == 0x7d as c_int
	    {
	      (*parser).available_depth += 1;
	      break;
	    }

	  if c != b',' as c_int
	    { json_signal_error (parser, Qjson_parse_error ()); }

	  c = json_skip_whitespace (parser);
	}
    }

  match (*parser).conf.object_type
    {
    json_object_hashtable =>
      {
	let value: EMACS_INT
	  = ((*parser).object_workspace_current - first) >> 1;
	result = make_hash_table (&raw const hashtest_equal, value, Weak_None);
	let h: *mut Lisp_Hash_Table = XHASH_TABLE (result);
	let mut i: ptrdiff_t = first;
	while i < (*parser).object_workspace_current
	  {
	    let mut hash: hash_hash_t = 0;
	    let key: Lisp_Object = *(*parser).object_workspace.offset (i as isize);
	    let value: Lisp_Object = *(*parser).object_workspace.offset ((i + 1) as isize);
	    let found: ptrdiff_t
	      = hash_find_get_hash (h, key, &raw mut hash);
	    if found < 0
	      { hash_put (h, key, value, hash); }
	    else
	      { set_hash_value_slot (h, found as hash_idx_t, value); }
	    i += 2;
	  }
	(*parser).object_workspace_current = first;
      }
    json_object_alist =>
      { }
    json_object_plist =>
      { }
    _ =>
      { emacs_abort (); }
    }

  return result;
}

/* Token-char is not a JSON terminology.  When parsing
   null/false/true, this function tells the character set that is need
   to be considered as part of a token.  For example, if the input is
   "truesomething", then the parser shouldn't consider it as "true",
   and an additional later "something" token. An additional example:
   if the input is "truetrue", then calling (json-parse-buffer) twice
   shouldn't produce two successful calls which return t, but a
   parsing error */
unsafe fn json_is_token_char (c: c_int) -> bool
{
  return ((c >= b'A' as c_int && c <= b'Z' as c_int)
	  || (c >= b'a' as c_int && c <= b'z' as c_int)
	  || (c >= b'0' as c_int && c <= b'9' as c_int)
	  || (c == b'-' as c_int));
}

unsafe fn json_parse_value (parser: *mut json_parser, c: c_int) -> Lisp_Object
{
  match c as u8
    {
    0x7b =>
      { return json_parse_object (parser); }
    b'[' =>
      { return json_parse_array (parser); }
    0x22 =>
      { return json_parse_string (parser, false, false); }
    b'0' | b'1' | b'2' | b'3' | b'4' =>
      { return json_parse_number (parser, c); }
    b'5' | b'6' | b'7' | b'8' | b'9' =>
      { return json_parse_number (parser, c); }
    b'-' =>
      { return json_parse_number (parser, c); }
    b't' =>
      {
	if json_input_get_if_possible (parser) == b'r' as c_int
	    && json_input_get_if_possible (parser) == b'u' as c_int
	    && json_input_get_if_possible (parser) == b'e' as c_int
	  {
	    let c2: c_int = json_input_get_if_possible (parser);
	    if !json_is_token_char (c2)
	      {
		if c2 >= 0
		  { json_input_put_back (parser); }
		return Qt ();
	      }
	  }
      }
    b'f' =>
      {
	if json_input_get_if_possible (parser) == b'a' as c_int
	    && json_input_get_if_possible (parser) == b'l' as c_int
	    && json_input_get_if_possible (parser) == b's' as c_int
	    && json_input_get_if_possible (parser) == b'e' as c_int
	  {
	    let c2: c_int = json_input_get_if_possible (parser);
	    if !json_is_token_char (c2)
	      {
		if c2 >= 0
		  { json_input_put_back (parser); }
		return (*parser).conf.false_object;
	      }
	  }
      }
    b'n' =>
      {
	if json_input_get_if_possible (parser) == b'u' as c_int
	    && json_input_get_if_possible (parser) == b'l' as c_int
	    && json_input_get_if_possible (parser) == b'l' as c_int
	  {
	    let c2: c_int = json_input_get_if_possible (parser);
	    if !json_is_token_char (c2)
	      {
		if c2 >= 0
		  { json_input_put_back (parser); }
		return (*parser).conf.null_object;
	      }
	  }
      }
    _ =>
      { }
    }

  json_signal_error (parser, Qjson_parse_error ());
}

unsafe fn json_parse (parser: *mut json_parser) -> Lisp_Object
{
  return json_parse_value (parser, json_skip_whitespace (parser));
}

/* Count number of characters in the NBYTES bytes at S.  */
unsafe fn count_chars (s: *const u8, nbytes: ptrdiff_t) -> ptrdiff_t
{
  let mut nchars: ptrdiff_t = 0;
  let mut i: ptrdiff_t = 0;
  while i < nbytes
    {
      nchars += ((*s.offset (i as isize) & 0xc0) != 0x80) as ptrdiff_t;
      i += 1;
    }
  return nchars;
}

/* Count number of newlines in the NBYTES bytes at S.  */
unsafe fn count_newlines (s: *const u8, nbytes: ptrdiff_t) -> ptrdiff_t
{
  let mut nls: ptrdiff_t = 0;
  let mut i: ptrdiff_t = 0;
  while i < nbytes
    {
      nls += ((*s.offset (i as isize)) == b'\n') as ptrdiff_t;
      i += 1;
    }
  return nls;
}

pub unsafe extern "C-unwind" fn string_byte_to_pos (obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t
{
  eassert! (STRINGP (obj));
  eassert! (byte <= SBYTES (obj));
  return if STRING_MULTIBYTE (obj) { count_chars (SDATA (obj), byte) } else { byte };
}

pub unsafe extern "C-unwind" fn string_byte_to_line (obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t
{
  eassert! (STRINGP (obj));
  eassert! (byte <= SBYTES (obj));
  return count_newlines (SDATA (obj), byte);
}

DEFUN! ("json-parse-string", Fjson_parse_string, Sjson_parse_string, 1, MANY,
       NULL,
       doc: "Parse the JSON STRING into a Lisp value.\nThis is essentially the reverse operation of `json-serialize', which\nsee.  The returned value will be the JSON null value, the JSON false\nvalue, t, a number, a string, a vector, a list, a hash-table, an alist,\nor a plist.  Its elements will be further values of these types.\nIf STRING doesn't contain a valid JSON value, this function\nsignals an error of type `json-parse-error'.\n\nThe arguments ARGS are a list of keyword/argument pairs:\n\n:object-type TYPE -- use TYPE to represent JSON objects.\n  TYPE can be `hash-table' (the default), `alist' or `plist'.\n  If an object has members with the same key, `hash-table' keeps only\n  the last value of such keys, while `alist' and `plist' keep all the\n  members.\n\n:array-type TYPE -- use TYPE to represent JSON arrays.\n  TYPE can be `array' (the default) or `list'.\n\n:null-object OBJ -- use OBJ to represent a JSON null value.\n  It defaults to `:null'.\n\n:false-object OBJ -- use OBJ to represent a JSON false value.\n  It defaults to `:false'.\nusage: (json-parse-string STRING &rest ARGS) ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();

  let string: Lisp_Object = *args;
  CHECK_STRING (string);
  let mut conf: json_configuration
    = json_configuration { object_type: json_object_hashtable,
			   array_type: json_array_array,
			   null_object: QCnull (), false_object: QCfalse () };
  json_parse_args (nargs - 1, args.offset (1), &raw mut conf, true);

  let mut p: json_parser = std::mem::zeroed ();
  let begin: *const u8 = SDATA (string);
  json_parser_init (&raw mut p, conf, begin, begin.offset (SBYTES (string) as isize),
		    std::ptr::null (), std::ptr::null (),
		    string_byte_to_pos, string_byte_to_line, string);
  record_unwind_protect_ptr (Some (json_parser_done), (&raw mut p) as *mut c_void);
  let result: Lisp_Object = json_parse (&raw mut p);

  if json_skip_whitespace_if_possible (&raw mut p) >= 0
    { json_signal_error (&raw mut p, Qjson_trailing_content ()); }

  return unbind_to (count, result);
});

pub unsafe extern "C-unwind" fn buffer_byte_to_pos (obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t
{
  /* The position from the start of the parse (for compatibility).  */
  return BYTE_TO_CHAR (PT_BYTE! () + byte) - PT! ();
}

pub unsafe extern "C-unwind" fn buffer_byte_to_line (obj: Lisp_Object, byte: ptrdiff_t) -> ptrdiff_t
{
  /* Line from start of the parse (for compatibility). */
  let to_gap: ptrdiff_t = GPT_BYTE! () - PT_BYTE! ();
  return if to_gap > 0 && to_gap < byte
	   { count_newlines (PT_ADDR! (), to_gap)
	     + count_newlines (GAP_END_ADDR! (), byte - to_gap) }
	 else
	   { count_newlines (PT_ADDR! (), byte) };
}

DEFUN! ("json-parse-buffer", Fjson_parse_buffer, Sjson_parse_buffer,
       0, MANY,
       NULL,
       doc: "Read a JSON value from current buffer starting at point.\nMove point after the end of the value if parsing was successful.\nOn error, don't move point.\n\nThe returned value will be a vector, list, hashtable, alist, or\nplist.  Its elements will be the JSON null value, the JSON false\nvalue, t, numbers, strings, or further vectors, lists, hashtables,\nalists, or plists.\n\nIf the current buffer doesn't contain a valid JSON value, the\nfunction signals an error of type `json-parse-error'.\n\nThe arguments ARGS are a list of keyword/argument pairs:\n\n:object-type TYPE -- use TYPE to represent JSON objects.\n  TYPE can be `hash-table' (the default), `alist' or `plist'.\n  If an object has members with the same key, `hash-table' keeps only\n  the last value of such keys, while `alist' and `plist' keep all the\n  members.\n\n:array-type TYPE -- use TYPE to represent JSON arrays.\n  TYPE can be `array' (the default) or `list'.\n\n:null-object OBJ -- use OBJ to represent a JSON null value.\n  It defaults to `:null'.\n\n:false-object OBJ -- use OBJ to represent a JSON false value.\n  It defaults to `:false'.\nusage: (json-parse-buffer &rest args) ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();

  let mut conf: json_configuration
    = json_configuration { object_type: json_object_hashtable,
			   array_type: json_array_array,
			   null_object: QCnull (), false_object: QCfalse () };
  json_parse_args (nargs, args, &raw mut conf, true);

  let mut p: json_parser = std::mem::zeroed ();
  let mut begin: *mut u8 = PT_ADDR! ();
  let mut end: *mut u8 = if GPT! () == ZV! () { GPT_ADDR! () } else { ZV_ADDR! () };
  let mut secondary_begin: *mut u8 = std::ptr::null_mut ();
  let mut secondary_end: *mut u8 = std::ptr::null_mut ();
  if PT! () == ZV! ()
    {
      begin = std::ptr::null_mut ();
      end = std::ptr::null_mut ();
    }
  else if GPT! () > PT! () && GPT! () < ZV! () && GAP_SIZE! () > 0
    {
      end = GPT_ADDR! ();
      secondary_begin = GAP_END_ADDR! ();
      secondary_end = ZV_ADDR! ();
    }

  json_parser_init (&raw mut p, conf, begin, end, secondary_begin, secondary_end,
		    buffer_byte_to_pos, buffer_byte_to_line, Qnil ());
  record_unwind_protect_ptr (Some (json_parser_done), (&raw mut p) as *mut c_void);
  let result: Lisp_Object = json_parse (&raw mut p);

  let byte: ptrdiff_t = PT_BYTE! () + ((p.input_current as ptrdiff_t)
				      - (p.input_begin as ptrdiff_t))
    + p.additional_bytes_count;
  let position: ptrdiff_t = if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
			      { byte } else { BYTE_TO_CHAR (byte) };
  SET_PT_BOTH (position, byte);

  return unbind_to (count, result);
});

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn syms_of_json ()
{
  DEFSYM! (QCnull, ":null");
  DEFSYM! (QCfalse, ":false");

  DEFSYM! (Qjson_value_p, "json-value-p");

  DEFSYM! (Qjson_error, "json-error");
  DEFSYM! (Qjson_out_of_memory, "json-out-of-memory");
  DEFSYM! (Qjson_parse_error, "json-parse-error");
  DEFSYM! (Qjson_end_of_file, "json-end-of-file");
  DEFSYM! (Qjson_trailing_content, "json-trailing-content");
  DEFSYM! (Qjson_object_too_deep, "json-object-too-deep");
  DEFSYM! (Qjson_utf8_decode_error, "json-utf8-decode-error");
  DEFSYM! (Qjson_invalid_surrogate_error, "json-invalid-surrogate-error");
  DEFSYM! (Qjson_number_out_of_range, "json-number-out-of-range-error");
  DEFSYM! (Qjson_escape_sequence_error, "json-escape-sequence-error");
  define_error (Qjson_error (), c"generic JSON error".as_ptr (), Qerror ());
  define_error (Qjson_out_of_memory (),
		c"not enough memory for creating JSON object".as_ptr (), Qjson_error ());
  define_error (Qjson_parse_error (), c"could not parse JSON stream".as_ptr (),
		Qjson_error ());
  define_error (Qjson_end_of_file (), c"end of JSON stream".as_ptr (),
		Qjson_parse_error ());
  define_error (Qjson_trailing_content (), c"trailing content after JSON stream".as_ptr (),
		Qjson_parse_error ());
  define_error (Qjson_object_too_deep (),
		c"object cyclic or Lisp evaluation too deep".as_ptr (), Qjson_error ());
  define_error (Qjson_utf8_decode_error (),
		c"invalid utf-8 encoding".as_ptr (), Qjson_error ());
  define_error (Qjson_invalid_surrogate_error (),
		c"invalid surrogate pair".as_ptr (), Qjson_error ());
  define_error (Qjson_number_out_of_range (),
		c"number out of range".as_ptr (), Qjson_error ());
  define_error (Qjson_escape_sequence_error (),
		c"invalid escape sequence".as_ptr (), Qjson_parse_error ());

  DEFSYM! (QCobject_type, ":object-type");
  DEFSYM! (QCarray_type, ":array-type");
  DEFSYM! (QCnull_object, ":null-object");
  DEFSYM! (QCfalse_object, ":false-object");
  DEFSYM! (Qalist, "alist");
  DEFSYM! (Qplist, "plist");
  DEFSYM! (Qarray, "array");

  defsubr (&raw mut Sjson_serialize);
  defsubr (&raw mut Sjson_insert);
  defsubr (&raw mut Sjson_parse_string);
  defsubr (&raw mut Sjson_parse_buffer);
}
