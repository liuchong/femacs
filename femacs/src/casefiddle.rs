// femacs: port of src/casefiddle.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, field, enum member, global, statement), same control flow.  The eleven subrs use
// `DEFUN!` (PORTING.md 5.1).
//
// The `#include` list mirrors casefiddle.c 20-33 - config.h, lisp.h, character.h, buffer.h,
// commands.h, syntax.h, composite.h, keymap.h - with the mirrors those pull in.  The single
// conditional in the file is `#ifdef HAVE_TREE_SITTER` (C 543-552, 572-579), which the
// frozen configuration leaves undefined (`include/config.rs`: HAVE_TREE_SITTER = false), so
// the arm is kept as gated text (P-51) and its two callees are declared inside it.
//
// `enum case_action` (C 30) and the two Greek constants (C 197-198) are C's integer enums;
// the port spells each as its C type plus one constant per member (PORTING.md 1.3, the
// coding.rs precedent).  C's `struct casing_str_buf` is written through a cast in
// do_casify_multibyte_string (C 267-274), so `data` stays the first member, as C asserts.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]


include!("include/config.rs");
include!("../lib/include/stdio.rs");
include!("../lib/include/string.rs");
include!("include/lisp.rs");
include!("include/character.rs");
include!("include/itree.rs");
include!("include/buffer.rs");
include!("include/commands.rs");
include!("include/syntax.rs");
include!("include/composite.rs");
include!("include/keymap.rs");

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
// port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
// defines it here (status §3.29, the casetab.rs shape).
#[unsafe(no_mangle)]
pub static mut Vregion_extract_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut case_symbols_as_words: bool = false;

// casefiddle.c 30: `enum case_action {CASE_UP, CASE_DOWN, CASE_CAPITALIZE,
// CASE_CAPITALIZE_UP};` (PORTING.md 1.3).
pub type case_action = c_int;
pub const CASE_UP: case_action = 0;
pub const CASE_DOWN: case_action = 1;
pub const CASE_CAPITALIZE: case_action = 2;
pub const CASE_CAPITALIZE_UP: case_action = 3;

/* State for casing individual characters.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct casing_context
{
  /* A char-table with title-case character mappings or nil.  Non-nil implies
     flag is CASE_CAPITALIZE or CASE_CAPITALIZE_UP.  */
  pub titlecase_char_table: Lisp_Object,

  /* The unconditional special-casing Unicode property char tables for upper
     casing, lower casing and title casing respectively.  */
  pub specialcase_char_tables: [Lisp_Object; 3],

  /* User-requested action.  */
  pub flag: case_action,

  /* If true, the function operates on a buffer as opposed to a string
     or character.  When run on a buffer, syntax_prefix_flag_p is
     taken into account when determining whether the context is within
     a word.  */
  pub inbuffer: bool,

  /* Whether the context is within a word.  */
  pub inword: bool,

  /* What the last operation was.  */
  pub downcase_last: bool,
}

/* Initialize CTX structure for casing characters.  */
unsafe fn prepare_casing_context (ctx: *mut casing_context,
				  flag: case_action, inbuffer: bool)
{
  (*ctx).flag = flag;
  (*ctx).inbuffer = inbuffer;
  (*ctx).inword = false;
  (*ctx).titlecase_char_table
    = if flag < CASE_CAPITALIZE { Qnil () }
      else { uniprop_table (Qtitlecase ()) };
  (*ctx).specialcase_char_tables[CASE_UP as usize]
    = if flag == CASE_DOWN { Qnil () }
      else { uniprop_table (Qspecial_uppercase ()) };
  (*ctx).specialcase_char_tables[CASE_DOWN as usize]
    = if flag == CASE_UP { Qnil () }
      else { uniprop_table (Qspecial_lowercase ()) };
  (*ctx).specialcase_char_tables[CASE_CAPITALIZE as usize]
    = if flag < CASE_CAPITALIZE { Qnil () }
      else { uniprop_table (Qspecial_titlecase ()) };

  /* If the case table is flagged as modified, rescan it.  */
  if NILP (FAM_ELEM! ((*XCHAR_TABLE (BVAR! (current_buffer, downcase_table_))).extras, 1))
    { Fset_case_table (BVAR! (current_buffer, downcase_table_)); }

  if inbuffer && flag >= CASE_CAPITALIZE
    { SETUP_BUFFER_SYNTAX_TABLE (); }	/* For syntax_prefix_flag_p.  */
}

fn case_ch_is_word (syntax: syntaxcode) -> bool
{
  return syntax == Sword || (unsafe { case_symbols_as_words } && syntax == Ssymbol);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct casing_str_buf
{
  pub data: [u8; CASING_STR_BUF_DATA_LEN],
  pub len_chars: u8,
  pub len_bytes: u8,
}

/// C: `max (6, MAX_MULTIBYTE_LENGTH)` — the array bound of `struct casing_str_buf.data`
/// (casefiddle.c 102); the port folds it here because an array length is a constant
/// expression and the fold has no C name of its own (the `BUF_BYTES_MAX` precedent).
pub const CASING_STR_BUF_DATA_LEN: usize = if 6 > MAX_MULTIBYTE_LENGTH { 6 } else { MAX_MULTIBYTE_LENGTH as usize };

/* Based on CTX, case character CH.  If BUF is NULL, return cased character.
   Otherwise, if BUF is non-NULL, save result in it and return whether the
   character has been changed.

   Since meaning of return value depends on arguments, it’s more convenient to
   use case_single_character or case_character instead.  */
unsafe fn case_character_impl (buf: *mut casing_str_buf,
			       ctx: *mut casing_context, ch: c_int) -> c_int
{
  let mut flag: case_action;
  let mut prop: Lisp_Object;
  // C's `int cased;` is read only after the `goto done` above has assigned it or the
  // two arms below have; the port gives it the zero initializer a C local that is only
  // written through a pointer gets (PORTING.md 1.5).
  let mut cased: c_int = 0;

  /* Update inword state */
  let was_inword: bool = (*ctx).inword;
  (*ctx).inword = case_ch_is_word (SYNTAX (ch)) &&
    (!(*ctx).inbuffer || was_inword || !syntax_prefix_flag_p (ch));

  'done: {
  /* Normalize flag so its one of CASE_UP, CASE_DOWN or CASE_CAPITALIZE.  */
  if (*ctx).flag == CASE_CAPITALIZE
    { flag = (*ctx).flag - was_inword as case_action; }
  else if (*ctx).flag != CASE_CAPITALIZE_UP
    { flag = (*ctx).flag; }
  else if !was_inword
    { flag = CASE_CAPITALIZE; }
  else
    {
      cased = ch;
      break 'done;
    }

  /* Look through the special casing entries.  */
  if !buf.is_null () && !NILP ((*ctx).specialcase_char_tables[flag as usize])
    {
      prop = CHAR_TABLE_REF ((*ctx).specialcase_char_tables[flag as usize], ch as isize);
      if STRINGP (prop)
	{
	  let str_: *mut Lisp_String = XSTRING (prop);
	  if STRING_BYTES (str_) <= std::mem::size_of_val (&(*buf).data) as isize
	    {
	      (*buf).len_chars = (*str_).u.s.size as u8;
	      (*buf).len_bytes = STRING_BYTES (str_) as u8;
	      memcpy ((*buf).data.as_mut_ptr () as *mut c_void,
		      (*str_).u.s.data as *const c_void,
		      (*buf).len_bytes as usize);
	      return 1;
	    }
	}
    }

  /* Handle simple, one-to-one case.  */
  if flag == CASE_DOWN
    {
      cased = downcase (ch);
      (*ctx).downcase_last = true;
    }
  else
    {
      let mut cased_is_set: bool = false;
      (*ctx).downcase_last = false;
      if !NILP ((*ctx).titlecase_char_table)
	{
	  prop = CHAR_TABLE_REF ((*ctx).titlecase_char_table, ch as isize);
	  if CHARACTERP (prop)
	    {
	      cased = XFIXNAT (prop) as c_int;
	      cased_is_set = true;
	    }
	}
      if !cased_is_set
	{ cased = upcase (ch); }
    }
  }

  /* And we’re done.  */
  if buf.is_null ()
    { return cased; }
  (*buf).len_chars = 1;
  (*buf).len_bytes = CHAR_STRING (cased, (*buf).data.as_mut_ptr ()) as u8;
  return (cased != ch) as c_int;
}

/* In Greek, lower case sigma has two forms: one when used in the middle and one
   when used at the end of a word.  Below is to help handle those cases when
   casing.

   The rule does not conflict with any other casing rules so while it is
   a conditional one, it is independent of language.  */

pub const GREEK_CAPITAL_LETTER_SIGMA: c_int = 0x03A3; /* Σ */
pub const GREEK_SMALL_LETTER_FINAL_SIGMA: c_int = 0x03C2; /* ς */

/* Based on CTX, case character CH accordingly.  Update CTX as necessary.
   Return cased character.

   Special casing rules (such as upcase(ﬁ) = FI) are not handled.  For
   characters whose casing results in multiple code points, the character is
   returned unchanged.  */
#[inline(always)]
unsafe fn case_single_character (ctx: *mut casing_context, ch: c_int) -> c_int
{
  return case_character_impl (std::ptr::null_mut (), ctx, ch);
}

/* Save in BUF result of casing character CH.  Return whether casing changed the
   character.

   If not-NULL, NEXT points to the next character in the cased string.  If NULL,
   it is assumed current character is the last one being cased.  This is used to
   apply some rules which depend on proceeding state.

   This is like case_single_character but also handles one-to-many casing
   rules.  */
unsafe fn case_character (buf: *mut casing_str_buf, ctx: *mut casing_context,
			  ch: c_int, next: *const u8) -> bool
{
  let was_inword: bool = (*ctx).inword;
  let changed: bool = case_character_impl (buf, ctx, ch) != 0;

  /* If we have just down-cased a capital sigma and the next character no longer
     has a word syntax (i.e. current character is end of word), use final
     sigma.  */
  if was_inword && ch == GREEK_CAPITAL_LETTER_SIGMA && changed
      && (next.is_null () || !case_ch_is_word (SYNTAX (STRING_CHAR (next))))
    {
      (*buf).len_bytes = CHAR_STRING (GREEK_SMALL_LETTER_FINAL_SIGMA,
				      (*buf).data.as_mut_ptr ()) as u8;
      (*buf).len_chars = 1;
    }

  return changed;
}

/* If C is not ASCII, make it unibyte. */
#[inline(always)]
fn make_char_unibyte (c: c_int) -> c_int
{
  return if ASCII_CHAR_P (c as isize) { c } else { CHAR_TO_BYTE8 (c) };
}

unsafe fn do_casify_natnum (ctx: *mut casing_context, obj: Lisp_Object) -> Lisp_Object
{
  let flagbits: c_int = CHAR_ALT | CHAR_SUPER | CHAR_HYPER
		  | CHAR_SHIFT | CHAR_CTL | CHAR_META;
  let mut ch: c_int = XFIXNAT (obj) as c_int;

  /* If the character has higher bits set above the flags, return it unchanged.
     It is not a real character.  */
  if ! (0 <= ch && ch <= flagbits + MAX_CHAR)
    { return obj; }

  let flags: c_int = ch & flagbits;
  ch = ch & !flagbits;

  /* FIXME: Even if enable-multibyte-characters is nil, we may manipulate
     multibyte chars.  This means we have a bug for latin-1 chars since when we
     receive an int 128-255 we can't tell whether it's an eight-bit byte or
     a latin-1 char.  */
  let multibyte: bool = ch >= 256
		    || !NILP (BVAR! (current_buffer, enable_multibyte_characters_));
  if ! multibyte
    { ch = make_char_multibyte (ch); }
  let cased: c_int = case_single_character (ctx, ch);
  if cased == ch
    { return obj; }

  let cased: c_int = if ! multibyte { make_char_unibyte (cased) } else { cased };
  return make_fixed_natnum (cased as EMACS_INT | flags as EMACS_INT);
}

unsafe fn do_casify_multibyte_string (ctx: *mut casing_context, mut obj: Lisp_Object)
				      -> Lisp_Object
{
  /* Verify that ‘data’ is the first member of struct casing_str_buf
     so that when casting char * to struct casing_str_buf *, the
     representation of the character is at the beginning of the
     buffer.  This is why we don’t need a separate struct
     casing_str_buf object, and can write directly to the destination.  */
  static_assert! (std::mem::offset_of! (casing_str_buf, data) == 0);

  let size: ptrdiff_t = SCHARS (obj);
  let mut n: ptrdiff_t = 0;
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  // C's `ckd_mul (&n, size, MAX_MULTIBYTE_LENGTH) || ckd_add (&n, n, sizeof (struct
  // casing_str_buf))`: the two stores happen only when their operation did not overflow
  // (P-15, the ckd_mul/ckd_add family of docs/ports/token-aliases.tsv).
  let (n_prod, mul_overflowed) = size.overflowing_mul (MAX_MULTIBYTE_LENGTH as ptrdiff_t);
  let (n_sum, add_overflowed)
    = n_prod.overflowing_add (std::mem::size_of::<casing_str_buf> () as ptrdiff_t);
  n = if mul_overflowed || add_overflowed { PTRDIFF_MAX }
    else { n_sum };
  let dst: *mut u8 = SAFE_ALLOCA! (sa_avail, n) as *mut u8;
  let dst_end: *mut u8 = dst.wrapping_add (n as usize);
  let mut o: *mut u8 = dst;

  let mut src: *const u8 = SDATA (obj);

  let mut size = size;
  n = 0;
  while size != 0
    {
      if dst_end.offset_from (o) < std::mem::size_of::<casing_str_buf> () as isize
	{ string_overflow (); }
      let ch: c_int = string_char_advance (&raw mut src);
      case_character (o as *mut casing_str_buf, ctx, ch,
		      if size > 1 { src } else { std::ptr::null () });
      n += (*(o as *mut casing_str_buf)).len_chars as ptrdiff_t;
      o = o.wrapping_add ((*(o as *mut casing_str_buf)).len_bytes as usize);
      size -= 1;
    }
  eassert! (o <= dst_end);
  obj = make_multibyte_string (dst as *const c_char, n, o.offset_from (dst));
  SAFE_FREE! (sa_count);
  return obj;
}

fn ascii_casify_character (downcase: bool, c: c_int) -> c_int
{
  let cased: Lisp_Object = CHAR_TABLE_REF (if downcase
					   { unsafe { uniprop_table (Qlowercase ()) } }
					   else { unsafe { uniprop_table (Quppercase ()) } },
					   c as isize);
  return if FIXNATP (cased) { XFIXNAT (cased) as c_int } else { c };
}

unsafe fn do_casify_unibyte_string (ctx: *mut casing_context, mut obj: Lisp_Object)
				    -> Lisp_Object
{
  let size: ptrdiff_t = SCHARS (obj);
  let mut ch: c_int;
  let mut cased: c_int;

  obj = Fcopy_sequence (obj);
  let mut i: ptrdiff_t = 0;
  while i < size
    {
      ch = make_char_multibyte (SREF (obj, i as usize) as c_int);
      cased = case_single_character (ctx, ch);
      if ch == cased
	{ i += 1; continue; }
      /* If down/upcasing changed an ASCII character into a non-ASCII
	 character (this can happen in some locales, like the Turkish
	 "I"), downcase using the ASCII char table.  */
      if ASCII_CHAR_P (ch as isize) && !SINGLE_BYTE_CHAR_P (cased as intmax_t)
	{ cased = ascii_casify_character ((*ctx).downcase_last, ch); }
      SSET (obj, i as usize, make_char_unibyte (cased) as u8);
      i += 1;
    }
  return obj;
}

unsafe fn casify_object (flag: case_action, obj: Lisp_Object) -> Lisp_Object
{
  let mut ctx: casing_context = std::mem::zeroed ();
  prepare_casing_context (&raw mut ctx, flag, false);

  if FIXNATP (obj)
    { return do_casify_natnum (&raw mut ctx, obj); }
  else if !STRINGP (obj)
    { wrong_type_argument (Qchar_or_string_p (), obj); }
  else if SCHARS (obj) == 0
    { return obj; }
  else if STRING_MULTIBYTE (obj)
    { return do_casify_multibyte_string (&raw mut ctx, obj); }
  else
    { return do_casify_unibyte_string (&raw mut ctx, obj); }
}

DEFUN! ("upcase", Fupcase, Supcase, 1, 1, 0,
       doc: "Convert argument to upper case and return that.\nThe argument may be a character or string.  The result has the same\ntype.  (See `downcase' for further details about the type.)\n\nThe argument object is not altered--the value is a copy.  If argument\nis a character, characters which map to multiple code points when\ncased, e.g. ﬁ, are returned unchanged.\n\nSee also `capitalize', `downcase' and `upcase-initials'.",
       (obj: Lisp_Object),
       {
  return casify_object (CASE_UP, obj);
});

DEFUN! ("downcase", Fdowncase, Sdowncase, 1, 1, 0,
       doc: "Convert argument to lower case and return that.\nThe argument may be a character or string.  The result has the same type,\nincluding the multibyteness of the string.\n\nThis means that if this function is called with a unibyte string\nargument, and downcasing it would turn it into a multibyte string\n(according to the current locale), the downcasing is done using ASCII\n\"C\" rules instead.  To accurately downcase according to the current\nlocale, the string must be converted into multibyte first.\n\nThe argument object is not altered--the value is a copy.",
       (obj: Lisp_Object),
       {
  return casify_object (CASE_DOWN, obj);
});

DEFUN! ("capitalize", Fcapitalize, Scapitalize, 1, 1, 0,
       doc: "Convert argument to capitalized form and return that.\nThis means that each word's first character is converted to either\ntitle case or upper case, and the rest to lower case.\n\nThe argument may be a character or string.  The result has the same\ntype.  (See `downcase' for further details about the type.)\n\nThe argument object is not altered--the value is a copy.  If argument\nis a character, characters which map to multiple code points when\ncased, e.g. ﬁ, are returned unchanged.",
       (obj: Lisp_Object),
       {
  return casify_object (CASE_CAPITALIZE, obj);
});

/* Like Fcapitalize but change only the initials.  */

DEFUN! ("upcase-initials", Fupcase_initials, Supcase_initials, 1, 1, 0,
       doc: "Convert the initial of each word in the argument to upper case.\nThis means that each word's first character is converted to either\ntitle case or upper case, and the rest are left unchanged.\n\nThe argument may be a character or string.  The result has the same\ntype.  (See `downcase' for further details about the type.)\n\nThe argument object is not altered--the value is a copy.  If argument\nis a character, characters which map to multiple code points when\ncased, e.g. ﬁ, are returned unchanged.",
       (obj: Lisp_Object),
       {
  return casify_object (CASE_CAPITALIZE_UP, obj);
});

/* Based on CTX, case region in a unibyte buffer from *STARTP to *ENDP.

   Save first and last positions that has changed in *STARTP and *ENDP
   respectively.  If no characters were changed, save -1 to *STARTP and leave
   *ENDP unspecified.

   Always return 0.  This is so that interface of this function is the same as
   do_casify_multibyte_region.  */
unsafe fn do_casify_unibyte_region (ctx: *mut casing_context,
				    startp: *mut ptrdiff_t, endp: *mut ptrdiff_t)
				    -> ptrdiff_t
{
  let mut first: ptrdiff_t = -1;
  let mut last: ptrdiff_t = -1;  /* Position of first and last changes.  */
  let end: ptrdiff_t = *endp;

  let mut pos: ptrdiff_t = *startp;
  while pos < end
    {
      let ch: c_int = make_char_multibyte (FETCH_BYTE! (pos) as c_int);
      let cased: c_int = case_single_character (ctx, ch);
      if cased == ch
	{ pos += 1; continue; }

      last = pos + 1;
      if first < 0
	{ first = pos; }

      FETCH_BYTE! (pos) = make_char_unibyte (cased) as u8;
      pos += 1;
    }

  *startp = first;
  *endp = last;
  return 0;
}

/* Based on CTX, case region in a multibyte buffer from *STARTP to *ENDP.

   Return number of added characters (may be negative if more characters were
   deleted then inserted), save first and last positions that has changed in
   *STARTP and *ENDP respectively.  If no characters were changed, return 0,
   save -1 to *STARTP and leave *ENDP unspecified.  */
unsafe fn do_casify_multibyte_region (ctx: *mut casing_context,
				      startp: *mut ptrdiff_t, endp: *mut ptrdiff_t)
				      -> ptrdiff_t
{
  let mut first: ptrdiff_t = -1;
  let mut last: ptrdiff_t = -1;  /* Position of first and last changes.  */
  let mut pos: ptrdiff_t = *startp;
  let mut pos_byte: ptrdiff_t = CHAR_TO_BYTE (pos);
  let mut size: ptrdiff_t = *endp - pos;
  let mut opoint: ptrdiff_t = PT! ();
  let mut added: ptrdiff_t = 0;

  // C's `for (; size; --size)` steps `size` on the `continue` path too, so the decrement
  // is written before that `continue` as well as at the end of the body.
  while size != 0
    {
      let mut len: c_int = 0;
      let ch: c_int = string_char_and_length (BYTE_POS_ADDR (pos_byte), &raw mut len);
      let mut buf: casing_str_buf = std::mem::zeroed ();
      if !case_character (&raw mut buf, ctx, ch,
			  if size > 1 { BYTE_POS_ADDR (pos_byte + len as ptrdiff_t) }
			  else { std::ptr::null () })
	{
	  pos_byte += len as ptrdiff_t;
	  pos += 1;
	  size -= 1;
	  continue;
	}

      last = pos + buf.len_chars as ptrdiff_t;
      if first < 0
	{ first = pos; }

      if buf.len_chars as c_int == 1 && buf.len_bytes as c_int == len
	{ memcpy (BYTE_POS_ADDR (pos_byte) as *mut c_void,
		  buf.data.as_ptr () as *const c_void, len as usize); }
      else
	{
	  /* Replace one character with the other(s), keeping text
	     properties the same.  */
	  replace_range_2 (pos, pos_byte, pos + 1, pos_byte + len as ptrdiff_t,
			   buf.data.as_ptr () as *const c_char,
			   buf.len_chars as ptrdiff_t,
			   buf.len_bytes as ptrdiff_t,
			   false);
	  added += buf.len_chars as ptrdiff_t - 1;
	  if opoint > pos
	    { opoint += buf.len_chars as ptrdiff_t - 1; }
	}

      pos_byte += buf.len_bytes as ptrdiff_t;
      pos += buf.len_chars as ptrdiff_t;
      size -= 1;
    }

  if PT! () != opoint
    { TEMP_SET_PT_BOTH (opoint, CHAR_TO_BYTE (opoint)); }

  *startp = first;
  *endp = last;
  return added;
}

/* flag is CASE_UP, CASE_DOWN or CASE_CAPITALIZE or CASE_CAPITALIZE_UP.  b and
   e specify range of buffer to operate on.  Return character position of the
   end of the region after changes.  */
unsafe fn casify_region (flag: case_action, mut b: Lisp_Object, mut e: Lisp_Object)
			 -> ptrdiff_t
{
  let mut added: ptrdiff_t;
  let mut ctx: casing_context = std::mem::zeroed ();

  validate_region (&raw mut b, &raw mut e);
  let mut start: ptrdiff_t = XFIXNAT (b);
  let mut end: ptrdiff_t = XFIXNAT (e);
  if start == end
    /* Not modifying because nothing marked.  */
    { return end; }
  modify_text (start, end);
  prepare_casing_context (&raw mut ctx, flag, true);

  // The two gated arms below follow the port's convention for configuration-dependent code
  // (P-51); this assertion keeps the `cfg` tied to the frozen configuration, so enabling the
  // feature without regenerating `config.rs` fails to compile instead of compiling text that
  // the C side never had (the buffer.rs precedent, P-08/P-34).
  const _: () = assert! (!cfg! (feature = "have_tree_sitter") || HAVE_TREE_SITTER);

  #[cfg (feature = "have_tree_sitter")]
  {
    // C's `#ifdef HAVE_TREE_SITTER` arm (casefiddle.c 543-552): gated text (P-51) that the
    // frozen configuration never compiles.  The two entries it calls live in treesit.h's
    // dead `#ifdef HAVE_TREE_SITTER` arm (that header's mirror carries only the one live
    // declaration outside the guard), so they are declared here, as vm-limit.c's dead arms are.
    /// C: struct ts_linecol treesit_linecol_maybe (ptrdiff_t, ptrdiff_t, struct ts_linecol)
    unsafe extern "C" {
      pub fn treesit_linecol_maybe (charpos: ptrdiff_t, bytepos: ptrdiff_t,
				    linecol: ts_linecol) -> ts_linecol;
    }
    let start_byte: ptrdiff_t = CHAR_TO_BYTE (start);
    let old_end_byte: ptrdiff_t = CHAR_TO_BYTE (end);
    let start_linecol: ts_linecol
      = treesit_linecol_maybe (start, start_byte,
			       BUF_TS_LINECOL_POINT (current_buffer));
    let old_end_linecol: ts_linecol
      = treesit_linecol_maybe (end, old_end_byte,
			       BUF_TS_LINECOL_POINT (current_buffer));
    let _ = (start_linecol, old_end_linecol);
  }

  let orig_end: ptrdiff_t = end;
  record_delete (start, make_buffer_string (start, end, true), false);
  if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
    {
      record_insert (start, end - start);
      added = do_casify_unibyte_region (&raw mut ctx, &raw mut start, &raw mut end);
    }
  else
    {
      let len: ptrdiff_t = end - start;
      let ostart: ptrdiff_t = start;
      added = do_casify_multibyte_region (&raw mut ctx, &raw mut start, &raw mut end);
      record_insert (ostart, len + added);
    }

  if start >= 0
    {
      signal_after_change (start, end - start - added, end - start);
      update_compositions (start, end, CHECK_ALL);
    }
  #[cfg (feature = "have_tree_sitter")]
  {
    // C's second `#ifdef HAVE_TREE_SITTER` arm (casefiddle.c 572-579).
    unsafe extern "C" {
      /// C: void treesit_record_change (ptrdiff_t, ptrdiff_t, ptrdiff_t, struct ts_linecol, struct ts_linecol, ptrdiff_t)
      pub fn treesit_record_change (start: ptrdiff_t, old_end: ptrdiff_t, new_end: ptrdiff_t,
				    start_linecol: ts_linecol, old_end_linecol: ts_linecol,
				    new_end_pos: ptrdiff_t);
    }
  }

  return orig_end + added;
}

/* Casify a possibly noncontiguous region according to FLAG.  BEG and
   END specify the bounds, except that if REGION_NONCONTIGUOUS_P is
   non-nil, the region's bounds are specified by (funcall
   region-extract-function 'bounds) instead.  */

unsafe fn casify_pnc_region (flag: case_action, beg: Lisp_Object, end: Lisp_Object,
			     region_noncontiguous_p: Lisp_Object) -> Lisp_Object
{
  if !NILP (region_noncontiguous_p)
    {
      let mut bounds: Lisp_Object = calln! (Vregion_extract_function, Qbounds ());
      FOR_EACH_TAIL! (bounds, {
	  CHECK_CONS (XCAR (bounds));
	  casify_region (flag, XCAR (XCAR (bounds)), XCDR (XCAR (bounds)));
      });
      CHECK_LIST_END (bounds, bounds);
    }
  else
    { casify_region (flag, beg, end); }

  return Qnil ();
}

DEFUN! ("upcase-region", Fupcase_region, Supcase_region, 2, 3,
       "(list (region-beginning) (region-end) (region-noncontiguous-p))",
       doc: "Convert the region to upper case.  In programs, wants two arguments.\nThese arguments specify the starting and ending character numbers of\nthe region to operate on.  When used as a command, the text between\npoint and the mark is operated on.\nSee also `capitalize-region'.",
       (beg: Lisp_Object, end: Lisp_Object, region_noncontiguous_p: Lisp_Object),
       {
  return casify_pnc_region (CASE_UP, beg, end, region_noncontiguous_p);
});

DEFUN! ("downcase-region", Fdowncase_region, Sdowncase_region, 2, 3,
       "(list (region-beginning) (region-end) (region-noncontiguous-p))",
       doc: "Convert the region to lower case.  In programs, wants two arguments.\nThese arguments specify the starting and ending character numbers of\nthe region to operate on.  When used as a command, the text between\npoint and the mark is operated on.",
       (beg: Lisp_Object, end: Lisp_Object, region_noncontiguous_p: Lisp_Object),
       {
  return casify_pnc_region (CASE_DOWN, beg, end, region_noncontiguous_p);
});

DEFUN! ("capitalize-region", Fcapitalize_region, Scapitalize_region, 2, 3,
       "(list (region-beginning) (region-end) (region-noncontiguous-p))",
       doc: "Convert the region to capitalized form.\nThis means that each word's first character is converted to either\ntitle case or upper case, and the rest to lower case.\nIn programs, give two arguments, the starting and ending\ncharacter positions to operate on.",
       (beg: Lisp_Object, end: Lisp_Object, region_noncontiguous_p: Lisp_Object),
       {
  return casify_pnc_region (CASE_CAPITALIZE, beg, end, region_noncontiguous_p);
});

/* Like Fcapitalize_region but change only the initials.  */

DEFUN! ("upcase-initials-region", Fupcase_initials_region,
       Supcase_initials_region, 2, 3,
       "(list (region-beginning) (region-end) (region-noncontiguous-p))",
       doc: "Upcase the initial of each word in the region.\nThis means that each word's first character is converted to either\ntitle case or upper case, and the rest are left unchanged.\nIn programs, give two arguments, the starting and ending\ncharacter positions to operate on.",
       (beg: Lisp_Object, end: Lisp_Object, region_noncontiguous_p: Lisp_Object),
       {
  return casify_pnc_region (CASE_CAPITALIZE_UP, beg, end,
			    region_noncontiguous_p);
});

unsafe fn casify_word (flag: case_action, arg: Lisp_Object) -> Lisp_Object
{
  CHECK_FIXNUM (arg);
  let mut farend: ptrdiff_t = scan_words (PT! (), XFIXNUM (arg));
  if farend == 0
    { farend = if XFIXNUM (arg) <= 0 { BEGV! () } else { ZV! () }; }
  SET_PT (casify_region (flag, make_fixnum (PT! ()), make_fixnum (farend)));
  return Qnil ();
}

DEFUN! ("upcase-word", Fupcase_word, Supcase_word, 1, 1, "p",
       doc: "Convert to upper case from point to end of word, moving over.\n\nIf point is in the middle of a word, the part of that word before point\nis ignored when moving forward.\n\nWith negative argument, convert previous words but do not move.\nSee also `capitalize-word'.",
       (arg: Lisp_Object),
       {
  return casify_word (CASE_UP, arg);
});

DEFUN! ("downcase-word", Fdowncase_word, Sdowncase_word, 1, 1, "p",
       doc: "Convert to lower case from point to end of word, moving over.\n\nIf point is in the middle of a word, the part of that word before point\nis ignored when moving forward.\n\nWith negative argument, convert previous words but do not move.",
       (arg: Lisp_Object),
       {
  return casify_word (CASE_DOWN, arg);
});

DEFUN! ("capitalize-word", Fcapitalize_word, Scapitalize_word, 1, 1, "p",
       doc: "Capitalize from point to the end of word, moving over.\nWith numerical argument ARG, capitalize the next ARG-1 words as well.\nThis gives the word(s) a first character in upper case\nand the rest lower case.\n\nIf point is in the middle of a word, the part of that word before point\nis ignored when moving forward.\n\nWith negative argument, capitalize previous words but do not move.",
       (arg: Lisp_Object),
       {
  return casify_word (CASE_CAPITALIZE, arg);
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_casefiddle ()
{
  DEFSYM! (Qbounds, "bounds");
  DEFSYM! (Qidentity, "identity");
  DEFSYM! (Qtitlecase, "titlecase");
  DEFSYM! (Qlowercase, "lowercase");
  DEFSYM! (Quppercase, "uppercase");
  DEFSYM! (Qspecial_uppercase, "special-uppercase");
  DEFSYM! (Qspecial_lowercase, "special-lowercase");
  DEFSYM! (Qspecial_titlecase, "special-titlecase");

  DEFVAR_LISP! (c"region-extract-function", Vregion_extract_function,
	       "Function to get the region's content.\nCalled with one argument METHOD which can be:\n- nil: return the content as a string (list of strings for\n  non-contiguous regions).\n- `delete-only': delete the region; the return value is undefined.\n- `bounds': return the boundaries of the region as a list of one\n  or more cons cells of the form (START . END).\n- anything else: delete the region and return its content\n  as a string (or list of strings for non-contiguous regions),\n  after filtering it with `filter-buffer-substring', which\n  is called, for each contiguous sub-region, with METHOD as its\n  3rd argument.  ");
  Vregion_extract_function = Qnil (); /* simple.el sets this.  */

  DEFVAR_BOOL! (c"case-symbols-as-words", case_symbols_as_words,
    "If non-nil, case functions treat symbol syntax as part of words.\n\nFunctions such as `upcase-initials' and `replace-match' check or modify\nthe case pattern of sequences of characters.  Normally, these operate on\nsequences of characters whose syntax is word constituent.  If this\nvariable is non-nil, then they operate on sequences of characters whose\nsyntax is either word constituent or symbol constituent.\n\nThis is useful for programming languages and styles where only the first\nletter of a symbol's name is ever capitalized.");
  case_symbols_as_words = false;
  DEFSYM! (Qcase_symbols_as_words, "case-symbols-as-words");
  Fmake_variable_buffer_local (Qcase_symbols_as_words ());

  defsubr (&raw mut Supcase);
  defsubr (&raw mut Sdowncase);
  defsubr (&raw mut Scapitalize);
  defsubr (&raw mut Supcase_initials);
  defsubr (&raw mut Supcase_region);
  defsubr (&raw mut Sdowncase_region);
  defsubr (&raw mut Scapitalize_region);
  defsubr (&raw mut Supcase_initials_region);
  defsubr (&raw mut Supcase_word);
  defsubr (&raw mut Sdowncase_word);
  defsubr (&raw mut Scapitalize_word);
}
