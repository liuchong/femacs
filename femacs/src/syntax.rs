// femacs: port of src/syntax.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, local), same control flow, same error model.  The non-`static`
// definitions C exports (`syntax_prefix_flag_p`, `gl_state`, `RE_SETUP_SYNTAX_TABLE_FOR_OBJECT`,
// `update_syntax_table`, `update_syntax_table_forward`, `scan_words`, `syntax_spec_code`) carry
// `#[unsafe(no_mangle)]`; the `static` helpers stay plain `fn` (PORTING.md 0.1-3, P-52).
//
// The `#include` list mirrors syntax.c 21-28 - config.h, lisp.h, dispextern.h, character.h,
// buffer.h, regex-emacs.h, syntax.h, intervals.h, category.h - through the mirrors those pull
// in.  `regex-emacs.rs` is a new partial mirror (the character-class API `in_classes` calls);
// `intervals.rs` is a new partial mirror: syntax.c is the first unit that reads interval fields
// (`i->position`, `i->plist`) and C's `LENGTH`/`INTERVAL_LAST_POS` arithmetic, so intervals.h
// needed a home before this file could compile at all.
//
// **C 30-32 redefine syntax.h's three macros with VIA_PROPERTY = 1**:
// `#define SYNTAX(c) syntax_property (c, 1)` (likewise SYNTAX_ENTRY, SYNTAX_WITH_FLAGS).
// syntax.h's own versions pass `false`, and those are what the mirror keeps for the *other*
// units.  Inside this file every `SYNTAX (c)` therefore reads `gl_state` (the interval-provided
// table) rather than the buffer's table - the definitions below shadow the mirror's through a
// glob import (PORTING.md P-39/P-21).  `docs/ports/syntax.md` records this as the file's most
// important deviation, because a token-set gate cannot see `true` for `1`.
//
// `goto` targets (`found`, `lossage`, `done`) follow PORTING.md 2.2: a labelled block
// `'label: { ... }` or a `loop` whose `break` carries the jump, never an early return.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

include!("include/config.rs");
include!("../lib/include/stdio.rs");	// lisp.h 34: FILE
include!("../lib/include/string.rs");	// lisp.h 30: <string.h>
include!("include/lisp.rs");
include!("include/itree.rs");		// buffer.h 27: <itree.h>
include!("include/buffer.rs");
include!("include/character.rs");
include!("include/intervals.rs");
include!("include/syntax.rs");
include!("include/category.rs");
include!("include/globals.rs");
include!("include/dispextern.rs");	// bitch_at_user, backward-prefix-chars' caller
include!("include/regex-emacs.rs");	// re_wctype_parse, re_iswctype

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530) and `struct emacs_globals globals` is defined once, in alloc.c; the port's
// `include/globals.rs` mirrors the names as declarations, so the unit that owns one defines it
// here (status 3.29, the callint.rs shape).

#[unsafe(no_mangle)]
pub static mut Vcomment_use_syntax_ppss: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut parse_sexp_ignore_comments: bool = false;
#[unsafe(no_mangle)]
pub static mut parse_sexp_lookup_properties: bool = false;
#[unsafe(no_mangle)]
pub static mut syntax_propertize__done: EMACS_INT = 0;
#[unsafe(no_mangle)]
pub static mut words_include_escapes: bool = false;
#[unsafe(no_mangle)]
pub static mut multibyte_syntax_as_symbol: bool = false;
#[unsafe(no_mangle)]
pub static mut open_paren_in_column_0_is_defun_start: bool = false;
#[unsafe(no_mangle)]
pub static mut Vfind_word_boundary_function_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut comment_end_can_be_escaped: bool = false;
#[unsafe(no_mangle)]
pub static mut Vforward_comment_function: Lisp_Object = lisp_h_Qnil ();

/* C's `<intprops.h>` TYPE_MINIMUM/TYPE_MAXIMUM; the lib/ ports write them as unit-local
   macros over the type's own limits (lib/dtotimespec.rs 30, the mktime.rs precedent).  */
macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }

/* Make syntax table lookup grant data in gl_state.  */
// C 30-32 (see the note at the top of this file): the mirror keeps syntax.h's
// `via_property = false` forms inside `syntax_h_own_definitions`, so these three definitions
// shadow them through that module's glob import.
#[inline(always)]
pub fn SYNTAX (c: c_int) -> syntaxcode
{
  return syntax_property (c, true);
}
#[inline(always)]
pub fn SYNTAX_ENTRY (c: c_int) -> Lisp_Object
{
  return syntax_property_entry (c, true);
}
#[inline(always)]
pub fn SYNTAX_WITH_FLAGS (c: c_int) -> c_int
{
  return syntax_property_with_flags (c, true);
}

/* Eight single-bit flags have the following meanings:
  1. This character is the first of a two-character comment-start sequence.
  2. This character is the second of a two-character comment-start sequence.
  3. This character is the first of a two-character comment-end sequence.
  4. This character is the second of a two-character comment-end sequence.
  5. This character is a prefix, for backward-prefix-chars.
  6. The char is part of a delimiter for comments of style "b".
  7. This character is part of a nestable comment sequence.
  8. The char is part of a delimiter for comments of style "c".
  Note that any two-character sequence whose first character has flag 1
  and whose second character has flag 2 will be interpreted as a comment start.

  Bits 6 and 8 discriminate among different comment styles.
  Languages such as C++ allow two orthogonal syntax start/end pairs
  and bit 6 determines whether a comment-end or Scommentend
  ends style a or b.  Comment markers can start style a, b, c, or bc.
  Style a is always the default.
  For 2-char comment markers, the style b flag is looked up only on the second
  char of the comment marker and on the first char of the comment ender.
  For style c (like the nested flag), the flag can be placed on any of
  the chars.  */

/* These functions extract specific flags from an integer
   that holds the syntax code and the flags.  */

fn SYNTAX_FLAGS_COMSTART_FIRST (flags: c_int) -> bool
{
  return ((flags >> 16) & 1) != 0;
}
fn SYNTAX_FLAGS_COMSTART_SECOND (flags: c_int) -> bool
{
  return ((flags >> 17) & 1) != 0;
}
fn SYNTAX_FLAGS_COMEND_FIRST (flags: c_int) -> bool
{
  return ((flags >> 18) & 1) != 0;
}
fn SYNTAX_FLAGS_COMEND_SECOND (flags: c_int) -> bool
{
  return ((flags >> 19) & 1) != 0;
}
fn SYNTAX_FLAGS_COMSTARTEND_FIRST (flags: c_int) -> bool
{
  return (flags & 0x50000) != 0;
}
fn SYNTAX_FLAGS_PREFIX (flags: c_int) -> bool
{
  return ((flags >> 20) & 1) != 0;
}
fn SYNTAX_FLAGS_COMMENT_STYLEB (flags: c_int) -> bool
{
  return ((flags >> 21) & 1) != 0;
}
fn SYNTAX_FLAGS_COMMENT_STYLEC (flags: c_int) -> bool
{
  return ((flags >> 23) & 1) != 0;
}
fn SYNTAX_FLAGS_COMMENT_STYLEC2 (flags: c_int) -> c_int
{
  return (flags >> 22) & 2; /* SYNTAX_FLAGS_COMMENT_STYLEC (flags) * 2 */
}
fn SYNTAX_FLAGS_COMMENT_NESTED (flags: c_int) -> bool
{
  return ((flags >> 22) & 1) != 0;
}

/* FLAGS should be the flags of the main char of the comment marker, e.g.
   the second for comstart and the first for comend.  */
fn SYNTAX_FLAGS_COMMENT_STYLE (flags: c_int, other_flags: c_int) -> c_int
{
  return (SYNTAX_FLAGS_COMMENT_STYLEB (flags) as c_int
	  | SYNTAX_FLAGS_COMMENT_STYLEC2 (flags)
	  | SYNTAX_FLAGS_COMMENT_STYLEC2 (other_flags));
}

/* Extract a particular flag for a given character.  */

fn SYNTAX_COMEND_FIRST (c: c_int) -> bool
{
  return SYNTAX_FLAGS_COMEND_FIRST (SYNTAX_WITH_FLAGS (c));
}

/* We use these constants in place for comment-style and
   string-ender-char to distinguish comments/strings started by
   comment_fence and string_fence codes.  */

// C 125-129: `enum { ST_COMMENT_STYLE = 256 + 1, ST_STRING_STYLE = 256 + 2 };` - an unnamed
// integer enum becomes its C type with one constant per member (PORTING.md 1.3).
const ST_COMMENT_STYLE: c_int = 256 + 1;
const ST_STRING_STYLE: c_int = 256 + 2;

/* This is the internal form of the parse state used in parse-partial-sexp.  */

#[repr(C)]
pub struct lisp_parse_state
  {
    pub depth: EMACS_INT,	/* Depth at end of parsing.  */
    pub instring: c_int,  /* -1 if not within string, else desired terminator.  */
    pub incomment: EMACS_INT, /* -1 if in unnestable comment else comment nesting */
    pub comstyle: c_int,  /* comment style a=0, or b=1, or ST_COMMENT_STYLE.  */
    pub quoted: bool,   /* True if just after an escape char at end of parsing.  */
    pub mindepth: EMACS_INT,	/* Minimum depth seen while scanning.  */
    /* Char number of most recent start-of-expression at current level */
    pub thislevelstart: ptrdiff_t,
    /* Char number of start of containing expression */
    pub prevlevelstart: ptrdiff_t,
    pub location: ptrdiff_t,	     /* Char number at which parsing stopped.  */
    pub location_byte: ptrdiff_t, /* Corresponding byte position.  */
    pub comstr_start: ptrdiff_t,  /* Position of last comment/string starter.  */
    pub levelstarts: Lisp_Object, /* Char numbers of starts-of-expression
				of levels (starting from outermost).  */
    pub prev_syntax: c_int, /* Syntax of previous position scanned, when
                        that position (potentially) holds the first char
                        of a 2-char construct, i.e. comment delimiter
                        or Sescape, etc.  Smax otherwise. */
  }

/* These variables are a cache for finding the start of a defun.
   find_start_pos is the place for which the defun start was found.
   find_start_value is the defun start position found for it.
   find_start_value_byte is the corresponding byte position.
   find_start_buffer is the buffer it was found in.
   find_start_begv is the BEGV value when it was found.
   find_start_modiff is the value of MODIFF when it was found.  */

static mut find_start_pos: ptrdiff_t = 0;
static mut find_start_value: ptrdiff_t = 0;
static mut find_start_value_byte: ptrdiff_t = 0;
static mut find_start_buffer: *mut buffer = std::ptr::null_mut ();
static mut find_start_begv: ptrdiff_t = 0;
static mut find_start_modiff: modiff_count = 0;

// C 181-189's forward declarations (`skip_chars`, `skip_syntaxes`, `scan_lists`,
// `scan_sexps_forward`, `internalize_parse_state`, `in_classes`, `parse_sexp_propertize`) have
// no Rust counterpart: items are order-independent within a module (PORTING.md 0.1-12).

/* This setter is used only in this file, so it can be private.  */
fn bset_syntax_table (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).syntax_table_ = val; }
}

/* Whether the syntax of the character C has the prefix flag set.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syntax_prefix_flag_p (c: c_int) -> bool
{
  return SYNTAX_FLAGS_PREFIX (SYNTAX_WITH_FLAGS (c));
}

/* Global state of syntax parser.  */
#[unsafe(no_mangle)]
pub static mut gl_state: gl_state_s =
  gl_state_s { object: lisp_h_Qnil (), start: 0, stop: 0, use_global: false,
	       global_code: lisp_h_Qnil (), current_syntax_table: lisp_h_Qnil (),
	       old_prop: lisp_h_Qnil (), b_property: 0, e_property: 0,
	       e_property_truncated: false,
	       forward_i: std::ptr::null_mut (), backward_i: std::ptr::null_mut () };

/* 1 + max-number of intervals to scan to property-change.  */
const INTERVALS_AT_ONCE: c_int = 10;

/* Set the syntax entry VAL for char C in table TABLE.  */

fn SET_RAW_SYNTAX_ENTRY (table: Lisp_Object, c: c_int, val: Lisp_Object)
{
  CHAR_TABLE_SET (table, c as isize, val);
}

/* Set the syntax entry VAL for char-range RANGE in table TABLE.
   RANGE is a cons (FROM . TO) specifying the range of characters.  */

unsafe fn SET_RAW_SYNTAX_ENTRY_RANGE (table: Lisp_Object, range: Lisp_Object, val: Lisp_Object)
{
  unsafe { Fset_char_table_range (table, range, val); }
}

/* Extract the information from the entry for character C
   in the current syntax table.  */

unsafe fn SYNTAX_MATCH (c: c_int) -> Lisp_Object
{
  let ent = SYNTAX_ENTRY (c);
  return if CONSP (ent) { XCDR (ent) } else { Qnil () };
}

/* A "dummy" value we never dereference, distinct from NULL.  */
// C 234: `#define uninitialized_interval ((INTERVAL) (intptr_t) 1u)`.
const uninitialized_interval: INTERVAL = 1 as INTERVAL;

/* This should be called with FROM at the start of forward
   search, or after the last position of the backward search.  It
   makes sure that the first char is picked up with correct table, so
   one does not need to call UPDATE_SYNTAX_TABLE immediately after the
   call.
   Sign of COUNT gives the direction of the search.
 */

unsafe fn SETUP_SYNTAX_TABLE (from: ptrdiff_t, count: ptrdiff_t)
{
  SETUP_BUFFER_SYNTAX_TABLE ();
  unsafe { gl_state.b_property = BEGV! (); }
  unsafe { gl_state.e_property = ZV! () + 1; }
  unsafe { gl_state.object = Qnil (); }
  if unsafe { parse_sexp_lookup_properties }
    {
      unsafe { gl_state.start = gl_state.b_property; }
      unsafe { gl_state.stop = gl_state.e_property; }
      unsafe { gl_state.backward_i = uninitialized_interval; }
      unsafe { gl_state.forward_i = uninitialized_interval; }
      if count > 0
	{ unsafe { update_syntax_table_forward (from, Qnil ()) }; }
      else if from > unsafe { BEGV! () }
	{
	  unsafe { update_syntax_table (from - 1, count, Qnil ()) };
	  parse_sexp_propertize (from - 1);
	}
    }
}

/* Same as above, but in OBJECT.  If OBJECT is nil, use current buffer.
   If it is t (which is only used in fast_c_string_match_ignore_case),
   ignore properties altogether.
   FROMBYTE is an regexp-byteoffset.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RE_SETUP_SYNTAX_TABLE_FOR_OBJECT (object: Lisp_Object,
							    frombyte: ptrdiff_t)
{
  SETUP_BUFFER_SYNTAX_TABLE ();
  unsafe { gl_state.object = object; }
  if BUFFERP (unsafe { gl_state.object })
    {
      let buf: *mut buffer = XBUFFER (unsafe { gl_state.object });
      unsafe { gl_state.b_property = BEG as ptrdiff_t; }
      unsafe { gl_state.e_property = BUF_ZV (buf); }
    }
  else if NILP (unsafe { gl_state.object })
    {
      unsafe { gl_state.b_property = BEG as ptrdiff_t; }
      unsafe { gl_state.e_property = ZV! () }; /* FIXME: Why not +1 like in SETUP_SYNTAX_TABLE? */
    }
  else if EQ (unsafe { gl_state.object }, Qt ())
    {
      unsafe { gl_state.b_property = 0; }
      unsafe { gl_state.e_property = PTRDIFF_MAX; }
    }
  else
    {
      unsafe { gl_state.b_property = 0; }
      unsafe { gl_state.e_property = 1 + SCHARS (unsafe { gl_state.object }); }
    }
  if unsafe { parse_sexp_lookup_properties }
    {
      unsafe { gl_state.start = gl_state.b_property; }
      unsafe { gl_state.stop = gl_state.e_property; }
      /* Don't cover any char, to make sure we'll call `update_syntax_table`
       * the very next time we need the syntax of a character.  */
      unsafe { gl_state.e_property = gl_state.b_property; }
      unsafe { gl_state.backward_i = uninitialized_interval; }
      unsafe { gl_state.forward_i = uninitialized_interval; }
    }
}

/* Update gl_state to an appropriate interval which contains CHARPOS.  The
   sign of COUNT gives the relative position of CHARPOS wrt the previously
   valid interval.  If INIT, only [be]_property fields of gl_state are
   valid at start, the rest is filled basing on OBJECT.

   `gl_state.*_i' are the intervals, and CHARPOS is further in the search
   direction than the intervals - or in an interval.  We update the
   current syntax-table basing on the property of this interval, and
   update the interval to start further than CHARPOS - or be
   NULL.  We also update lim_property to be the next value of
   charpos to call this subroutine again - or be before/after the
   start/end of OBJECT.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_syntax_table (charpos: ptrdiff_t, count: EMACS_INT,
					      object: Lisp_Object)
{
  let mut tmp_table: Lisp_Object;
  let mut cnt: c_int = 0;
  let mut invalidate: bool = true;
  let mut i: INTERVAL;

  if unsafe { gl_state.backward_i } == uninitialized_interval
    {
      eassert! (unsafe { gl_state.forward_i } == uninitialized_interval);
      unsafe { gl_state.old_prop = Qnil (); }
      i = interval_of (charpos, object);
      unsafe { gl_state.backward_i = i; }
      unsafe { gl_state.forward_i = i; }
      invalidate = false;
      if i.is_null ()
	{
	  unsafe { gl_state.b_property = gl_state.start; }
	  unsafe { gl_state.e_property = gl_state.stop; }
	  return;
	}
      unsafe { gl_state.b_property = (*((i) as *mut interval)).position; }
      unsafe { gl_state.e_property = INTERVAL_LAST_POS! (i); }
    }
  else
    {
      eassert! (unsafe { gl_state.backward_i } != uninitialized_interval);
      eassert! (unsafe { gl_state.forward_i } != uninitialized_interval);
      i = if count > 0 { unsafe { gl_state.forward_i } } else { unsafe { gl_state.backward_i } };

      /* We are guaranteed to be called with CHARPOS either in i,
         or further off.  */
      if i.is_null ()
        { error (c"Error in syntax_table logic for to-the-end intervals".as_ptr ()); }
      else if charpos < unsafe { (*((i) as *mut interval)).position }	/* Move left.  */
        {
          if count > 0
	    { error (c"Error in syntax_table logic for intervals <-".as_ptr ()); }
          /* Update the interval.  */
          i = update_interval (i, charpos);
          if INTERVAL_LAST_POS! (i) != unsafe { gl_state.b_property }
	    {
	      invalidate = false;
	      unsafe { gl_state.forward_i = i; }
	      unsafe { gl_state.e_property = INTERVAL_LAST_POS! (i); }
	    }
        }
      else if charpos >= INTERVAL_LAST_POS! (i) /* Move right.  */
        {
          if count < 0
	    { error (c"Error in syntax_table logic for intervals ->".as_ptr ()); }
          /* Update the interval.  */
          i = update_interval (i, charpos);
          if unsafe { (*((i) as *mut interval)).position } != unsafe { gl_state.e_property }
	    {
	      invalidate = false;
	      unsafe { gl_state.backward_i = i; }
	      unsafe { gl_state.b_property = (*((i) as *mut interval)).position; }
	    }
        }
    }

  tmp_table = textget (unsafe { (*((i) as *mut interval)).plist }, Qsyntax_table ());

  if invalidate
    { invalidate = ! EQ (tmp_table, unsafe { gl_state.old_prop }); } /* Need to invalidate? */

  if invalidate		/* Did not get to adjacent interval.  */
    {				/* with the same table => */
				/* invalidate the old range.  */
      if count > 0
	{
	  unsafe { gl_state.backward_i = i; }
	  unsafe { gl_state.b_property = (*((i) as *mut interval)).position; }
	}
      else
	{
	  unsafe { gl_state.forward_i = i; }
	  unsafe { gl_state.e_property = INTERVAL_LAST_POS! (i); }
	}
    }

  if ! EQ (tmp_table, unsafe { gl_state.old_prop })
    {
      unsafe { gl_state.current_syntax_table = tmp_table; }
      unsafe { gl_state.old_prop = tmp_table; }
      if EQ (Fsyntax_table_p (tmp_table), Qt ())
	{
	  unsafe { gl_state.use_global = false; }
	}
      else if CONSP (tmp_table)
	{
	  unsafe { gl_state.use_global = true; }
	  unsafe { gl_state.global_code = tmp_table; }
	}
      else
	{
	  unsafe { gl_state.use_global = false; }
	  unsafe { gl_state.current_syntax_table = BVAR! ((current_buffer! ()), syntax_table_); }
	}
    }

  while ! i.is_null ()
    {
      if cnt != 0 && ! EQ (tmp_table, textget (unsafe { (*((i) as *mut interval)).plist },
					       Qsyntax_table ()))
	{
	  if count > 0
	    {
	      unsafe { gl_state.e_property = (*((i) as *mut interval)).position; }
	      unsafe { gl_state.forward_i = i; }
	    }
	  else
	    {
	      unsafe { gl_state.b_property
		= (*((i) as *mut interval)).position + LENGTH! (i); }
	      unsafe { gl_state.backward_i = i; }
	    }
	  return;
	}
      else if cnt == INTERVALS_AT_ONCE
	{
	  if count > 0
	    {
	      unsafe { gl_state.e_property
		= (*((i) as *mut interval)).position + LENGTH! (i)
		/* e_property at EOB is not set to ZV but to ZV+1, so that
		   we can do INC(from);UPDATE_SYNTAX_TABLE_FORWARD without
		   having to check eob between the two.  */
		+ (if ! next_interval (i).is_null () { 0 } else { 1 }) };
	      unsafe { gl_state.forward_i = i; }
	    }
	  else
	    {
	      unsafe { gl_state.b_property = (*((i) as *mut interval)).position; }
	      unsafe { gl_state.backward_i = i; }
	    }
	  return;
	}
      cnt += 1;
      i = if count > 0 { next_interval (i) } else { previous_interval (i) };
    }
  eassert! (i.is_null ()); /* This property goes to the end.  */
  if count > 0
    {
      unsafe { gl_state.e_property = gl_state.stop; }
      unsafe { gl_state.forward_i = i; }
    }
  else
    { unsafe { gl_state.b_property = gl_state.start; } }
}

unsafe fn parse_sexp_propertize (charpos: ptrdiff_t)
{
  let zv: EMACS_INT = ZV! ();
  if unsafe { syntax_propertize__done } <= charpos
      && unsafe { syntax_propertize__done } < zv
    {
      let modiffs: modiff_count = CHARS_MODIFF! ();
      safe_calln! (Qinternal__syntax_propertize (),
		   make_fixnum (min (zv, 1 + charpos)));
      if modiffs != CHARS_MODIFF! ()
	{ unsafe { error (c"internal--syntax-propertize modified the buffer!".as_ptr ()); } }
      if unsafe { syntax_propertize__done } <= charpos
	  && unsafe { syntax_propertize__done } < zv
	{ unsafe { error (c"internal--syntax-propertize did not move syntax-propertize--done".as_ptr ()); } }
      SETUP_SYNTAX_TABLE (charpos, 1);
    }
  else if unsafe { gl_state.e_property } > unsafe { syntax_propertize__done }
    {
      unsafe { gl_state.e_property = syntax_propertize__done; }
      unsafe { gl_state.e_property_truncated = true; }
    }
  else if unsafe { gl_state.e_property_truncated }
	   && unsafe { gl_state.e_property } < unsafe { syntax_propertize__done }
    { /* When moving backward, e_property might be set without resetting
	 e_property_truncated, so the e_property_truncated flag may
	 occasionally be left raised spuriously.  This should be rare.  */
      unsafe { gl_state.e_property_truncated = false; }
      unsafe { update_syntax_table_forward (charpos, Qnil ()) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_syntax_table_forward (charpos: ptrdiff_t, object: Lisp_Object)
{
  if unsafe { gl_state.e_property_truncated }
    {
      eassert! (NILP (object));
      eassert! (charpos >= unsafe { gl_state.e_property });
      parse_sexp_propertize (charpos);
    }
  else
    {
      unsafe { update_syntax_table (charpos, 1, object) };
      if NILP (object) && unsafe { gl_state.e_property } > unsafe { syntax_propertize__done }
	{ parse_sexp_propertize (charpos); }
    }
}

/* Returns true if char at CHARPOS is quoted.
   Global syntax-table data should be set up already to be good at CHARPOS
   or after.  On return global syntax data is good for lookup at CHARPOS.  */

unsafe fn char_quoted (mut charpos: ptrdiff_t, mut bytepos: ptrdiff_t) -> bool
{
  let mut code: syntaxcode;
  let beg: ptrdiff_t = BEGV! ();
  let mut quoted: bool = false;
  let orig: ptrdiff_t = charpos;

  while charpos > beg
    {
      let c: c_int;
      dec_both (&mut charpos, &mut bytepos);

      UPDATE_SYNTAX_TABLE_BACKWARD (charpos);
      c = FETCH_CHAR_AS_MULTIBYTE (bytepos);
      code = SYNTAX (c);
      if ! (code == Scharquote || code == Sescape)
	{ break; }

      quoted = ! quoted;
    }

  UPDATE_SYNTAX_TABLE (orig);
  return quoted;
}

/* Return the bytepos one character before BYTEPOS.
   We assume that BYTEPOS is not at the start of the buffer.  */

unsafe fn dec_bytepos (bytepos: ptrdiff_t) -> ptrdiff_t
{
  return (bytepos
	  - (if ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	     { prev_char_len (bytepos) as ptrdiff_t } else { 1 }));
}

/* Return a defun-start position before POS and not too far before.
   It should be the last one before POS, or nearly the last.

   When open_paren_in_column_0_is_defun_start is nonzero,
   only the beginning of the buffer is treated as a defun-start.

   We record the information about where the scan started
   and what its result was, so that another call in the same area
   can return the same value very quickly.

   There is no promise at which position the global syntax data is
   valid on return from the subroutine, so the caller should explicitly
   update the global data.  */

unsafe fn find_defun_start (pos: ptrdiff_t, pos_byte: ptrdiff_t) -> ptrdiff_t
{
  let opoint: ptrdiff_t = PT! ();
  let opoint_byte: ptrdiff_t = PT_BYTE! ();

  /* Two `goto found` jumps (C 623, C 631) leave the block below; the statement after the
     label is C's shared tail, kept after the block (PORTING.md 2.2).  */
  'found: {

  /* Use previous finding, if it's valid and applies to this inquiry.  */
  if unsafe { (current_buffer! ()) } == unsafe { find_start_buffer }
      /* Reuse the defun-start even if POS is a little farther on.
	 POS might be in the next defun, but that's ok.
	 Our value may not be the best possible, but will still be usable.  */
      && pos <= unsafe { find_start_pos } + 1000
      && pos >= unsafe { find_start_value }
      && BEGV! () == unsafe { find_start_begv }
      && MODIFF! () == unsafe { find_start_modiff }
    { return unsafe { find_start_value }; }

  if ! NILP (unsafe { Vcomment_use_syntax_ppss })
    {
      let modiffs: modiff_count = CHARS_MODIFF! ();
      let ppss: Lisp_Object = calln! (Qsyntax_ppss (), make_fixnum (pos));
      if modiffs != CHARS_MODIFF! ()
	{ unsafe { error (c"syntax-ppss modified the buffer!".as_ptr ()); } }
      TEMP_SET_PT_BOTH (opoint, opoint_byte);
      let boc: Lisp_Object = unsafe { Fnth (make_fixnum (8), ppss) };
      if FIXNUMP (boc)
        {
          unsafe { find_start_value = XFIXNUM (boc); }
          unsafe { find_start_value_byte = CHAR_TO_BYTE (find_start_value); }
        }
      else
        {
          unsafe { find_start_value = pos; }
          unsafe { find_start_value_byte = pos_byte; }
        }
      break 'found;
    }
  if ! unsafe { open_paren_in_column_0_is_defun_start }
    {
      unsafe { find_start_value = BEGV! (); }
      unsafe { find_start_value_byte = BEGV_BYTE! (); }
      break 'found;
    }

  /* Back up to start of line.  */
  unsafe { scan_newline (pos, pos_byte, BEGV! (), BEGV_BYTE! (), -1, true) };

  /* We optimize syntax-table lookup for rare updates.  Thus we accept
     only those `^\s(' which are good in global _and_ text-property
     syntax-tables.  */
  SETUP_BUFFER_SYNTAX_TABLE ();
  while PT! () > BEGV! ()
    {
      /* Open-paren at start of line means we may have found our
	 defun-start.  */
      let mut c: c_int = FETCH_CHAR_AS_MULTIBYTE (PT_BYTE! ());
      if SYNTAX (c) == Sopen
	{
	  SETUP_SYNTAX_TABLE (PT! () + 1, -1);	/* Try again... */
	  c = FETCH_CHAR_AS_MULTIBYTE (PT_BYTE! ());
	  if SYNTAX (c) == Sopen
	    { break; }
	  /* Now fallback to the default value.  */
	  SETUP_BUFFER_SYNTAX_TABLE ();
	}
      /* Move to beg of previous line.  */
      unsafe { scan_newline (PT! (), PT_BYTE! (), BEGV! (), BEGV_BYTE! (), -2, true) };
    }

  /* Record what we found, for the next try.  */
  unsafe { find_start_value = PT! (); }
  unsafe { find_start_value_byte = PT_BYTE! (); }
  TEMP_SET_PT_BOTH (opoint, opoint_byte);

  } /* 'found */

  // found:
  unsafe { find_start_buffer = (current_buffer! ()); }
  unsafe { find_start_modiff = MODIFF! (); }
  unsafe { find_start_begv = BEGV! (); }
  unsafe { find_start_pos = pos; }

  return unsafe { find_start_value };
}

/* Return the SYNTAX_COMEND_FIRST of the character before POS, POS_BYTE.  */

unsafe fn prev_char_comend_first (mut pos: ptrdiff_t, mut pos_byte: ptrdiff_t) -> bool
{
  let mut c: c_int;
  let val: bool;

  dec_both (&mut pos, &mut pos_byte);
  UPDATE_SYNTAX_TABLE_BACKWARD (pos);
  c = FETCH_CHAR (pos_byte);
  val = SYNTAX_COMEND_FIRST (c);
  UPDATE_SYNTAX_TABLE_FORWARD (pos + 1);
  return val;
}

/* Check whether charpos FROM is at the end of a comment.
   FROM_BYTE is the bytepos corresponding to FROM.
   Do not move back before STOP.

   Return true if we find a comment ending at FROM/FROM_BYTE.

   If successful, store the charpos of the comment's beginning
   into *CHARPOS_PTR, and the bytepos into *BYTEPOS_PTR.

   Global syntax data remains valid for backward search starting at
   the returned value (or at FROM, if the search was not successful).  */

unsafe fn back_comment (from: ptrdiff_t, from_byte: ptrdiff_t, stop: ptrdiff_t,
		 comnested: bool, comstyle: c_int, charpos_ptr: *mut ptrdiff_t,
		 bytepos_ptr: *mut ptrdiff_t) -> bool
{
  /* Look back, counting the parity of string-quotes,
     and recording the comment-starters seen.
     When we reach a safe place, assume that's not in a string;
     then step the main scan to the earliest comment-starter seen
     an even number of string quotes away from the safe place.

     OFROM[I] is position of the earliest comment-starter seen
     which is I+2X quotes from the comment-end.
     PARITY is current parity of quotes from the comment end.  */
  let mut string_style: c_int = -1;	/* Presumed outside of any string.  */
  let mut string_lossage: bool = false;
  /* Not a real lossage: indicates that we have passed a matching comment
     starter plus a non-matching comment-ender, meaning that any matching
     comment-starter we might see later could be a false positive (hidden
     inside another comment).
     Test case:  { a (* b } c (* d *) */
  let mut comment_lossage: bool = false;
  let mut comment_end: ptrdiff_t = from;
  let mut comment_end_byte: ptrdiff_t = from_byte;
  let mut comstart_pos: ptrdiff_t = 0;
  let mut comstart_byte: ptrdiff_t = 0;	/* C leaves this uninitialised (0.1-13).  */
  /* Place where the containing defun starts,
     or 0 if we didn't come across it yet.  */
  let mut defun_start: ptrdiff_t = 0;
  let mut defun_start_byte: ptrdiff_t = 0;
  let mut code: syntaxcode;
  let mut nesting: ptrdiff_t = 1;		/* Current comment nesting.  */
  let mut c: c_int;
  let mut syntax: c_int = 0;
  let mut quit_count: u16 = 0;
  let mut from = from;
  let mut from_byte = from_byte;

  /* C's two `goto`s (`lossage` at C 786/840, `done` at C 850) both leave the scan loop but
     land in different places: `lossage` runs the decode block, `done` skips straight to the
     tail.  The port keeps the two jumps as one flag each, set where C jumps
     (PORTING.md 2.2: a forward jump out of a loop is not an early return).  */
  let mut jumped_to_lossage: bool = false;
  let mut jumped_to_done: bool = false;

  /* FIXME: A }} comment-ender style leads to incorrect behavior
     in the case of {{ c }}} because we ignore the last two chars which are
     assumed to be comment-enders although they aren't.  */

  /* At beginning of range to scan, we're outside of strings;
     that determines quote parity to the comment-end.  */
  while from != stop
    {
      quit_count = quit_count.wrapping_add (1);
      rarely_quit (quit_count as u64);

      let mut temp_byte: ptrdiff_t;
      let mut prev_syntax: c_int;
      let mut com2start: bool;
      let mut com2end: bool;
      let comstart: bool;

      /* Move back and examine a character.  */
      dec_both (&mut from, &mut from_byte);
      UPDATE_SYNTAX_TABLE_BACKWARD (from);

      prev_syntax = syntax;
      c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
      syntax = SYNTAX_WITH_FLAGS (c);
      code = SYNTAX (c);

      /* Check for 2-char comment markers.  */
      com2start = (SYNTAX_FLAGS_COMSTART_FIRST (syntax)
		   && SYNTAX_FLAGS_COMSTART_SECOND (prev_syntax)
		   && (comstyle
		       == SYNTAX_FLAGS_COMMENT_STYLE (prev_syntax, syntax))
		   && (SYNTAX_FLAGS_COMMENT_NESTED (prev_syntax)
		       || SYNTAX_FLAGS_COMMENT_NESTED (syntax)) == comnested);
      com2end = (SYNTAX_FLAGS_COMEND_FIRST (syntax)
		 && SYNTAX_FLAGS_COMEND_SECOND (prev_syntax));
      comstart = (com2start || code == Scomment);

      /* Nasty cases with overlapping 2-char comment markers:
	 - snmp-mode: -- c -- foo -- c --
	              --- c --
		      ------ c --
	 - c-mode:    *||*
		      |* *|* *|
		      |*| |* |*|
		      ///   */

      /* If a 2-char comment sequence partly overlaps with another,
	 we don't try to be clever.  E.g. |*| in C, or }% in modes that
	 have %..\n and %{..}%.  */
      if from > stop && (com2end || comstart)
	{
	  let mut next: ptrdiff_t = from;
	  let mut next_byte: ptrdiff_t = from_byte;
	  let next_c: c_int;
	  let next_syntax: c_int;
	  dec_both (&mut next, &mut next_byte);
	  UPDATE_SYNTAX_TABLE_BACKWARD (next);
	  next_c = FETCH_CHAR_AS_MULTIBYTE (next_byte);
	  next_syntax = SYNTAX_WITH_FLAGS (next_c);
	  if (((comstart || comnested)
	       && SYNTAX_FLAGS_COMEND_SECOND (syntax)
	       && SYNTAX_FLAGS_COMEND_FIRST (next_syntax))
	      || ((com2end || comnested)
		  && SYNTAX_FLAGS_COMSTART_SECOND (syntax)
		  && (comstyle
		      == SYNTAX_FLAGS_COMMENT_STYLE (syntax, prev_syntax))
		  && SYNTAX_FLAGS_COMSTART_FIRST (next_syntax)))
	    { jumped_to_lossage = true; break; }
	  /* UPDATE_SYNTAX_TABLE_FORWARD (next + 1); */
	}

      if com2start && comstart_pos == 0
	/* We're looking at a comment starter.  But it might be a comment
	   ender as well (see snmp-mode).  The first time we see one, we
	   need to consider it as a comment starter,
	   and the subsequent times as a comment ender.  */
	{ com2end = false; }

      /* Turn a 2-char comment sequences into the appropriate syntax.  */
      if com2end
	{ code = Sendcomment; }
      else if com2start
	{ code = Scomment; }
      /* Ignore comment starters of a different style.  */
      else if code == Scomment
	       && (comstyle != SYNTAX_FLAGS_COMMENT_STYLE (syntax, 0)
		   || SYNTAX_FLAGS_COMMENT_NESTED (syntax) != comnested)
	{ continue; }

      /* Ignore escaped characters, except comment-enders which cannot
         be escaped.  */
      if (unsafe { comment_end_can_be_escaped } || code != Sendcomment)
          && char_quoted (from, from_byte)
	{ continue; }

      match code
	{
	Sstring_fence | Scomment_fence | Sstring =>
	  {
	    if code == Sstring_fence || code == Scomment_fence
	      /* C 839 falls through from the two fence cases into the Sstring body
		 (PORTING.md 2.2: a shared tail is one match arm).  */
	      { c = if code == Sstring_fence { ST_STRING_STYLE } else { ST_COMMENT_STYLE }; }
	    /* Track parity of quotes.  */
	    if string_style == -1
	      /* Entering a string.  */
	      { string_style = c; }
	    else if string_style == c
	      /* Leaving the string.  */
	      { string_style = -1; }
	    else
	      /* If we have two kinds of string delimiters.
		 There's no way to grok this scanning backwards.  */
	      { string_lossage = true; }
	  }

	Scomment =>
	  {
	    /* We've already checked that it is the relevant comstyle.  */
	    if string_style != -1 || comment_lossage || string_lossage
	      /* There are odd string quotes involved, so let's be careful.
		 Test case in Pascal: " { " a { " } */
	      { jumped_to_lossage = true; break; }

	    if ! comnested
	      {
		/* Record best comment-starter so far.  */
		comstart_pos = from;
		comstart_byte = from_byte;
	      }
	    else
	      {
		nesting -= 1;
		if nesting <= 0
		  /* nested comments have to be balanced, so we don't need to
		     keep looking for earlier ones.  We use here the same (slightly
		     incorrect) reasoning as below:  since it is followed by uniform
		     paired string quotes, this comment-start has to be outside of
		     strings, else the comment-end itself would be inside a string. */
		  { jumped_to_done = true; break; }
	      }
	  }

	Sendcomment =>
	  {
	    if SYNTAX_FLAGS_COMMENT_STYLE (syntax, 0) == comstyle
	      && ((com2end && SYNTAX_FLAGS_COMMENT_NESTED (prev_syntax))
		  || SYNTAX_FLAGS_COMMENT_NESTED (syntax)) == comnested
	      /* This is the same style of comment ender as ours. */
	      {
		if comnested
		  { nesting += 1; }
		else
		  /* Anything before that can't count because it would match
		     this comment-ender rather than ours.  */
		  { from = stop; }	/* Break out of the loop.  */
	      }
	    else if comstart_pos != 0 || c != '\n' as c_int
	      /* We're mixing comment styles here, so we'd better be careful.
		 The (comstart_pos != 0 || c != '\n') check is not quite correct
		 (we should just always set comment_lossage), but removing it
		 would imply that any multiline comment in C would go through
		 lossage, which seems overkill.
		 The failure should only happen in the rare cases such as
		   { (* } *)   */
	      { comment_lossage = true; }
	  }

	Sopen =>
	  {
	    /* Assume a defun-start point is outside of strings.  */
	    if unsafe { open_paren_in_column_0_is_defun_start }
              && NILP (unsafe { Vcomment_use_syntax_ppss })
	      && (from == stop
		  || { temp_byte = dec_bytepos (from_byte);
		       FETCH_CHAR (temp_byte) == '\n' as c_int })
	      {
		defun_start = from;
		defun_start_byte = from_byte;
		from = stop;	/* Break out of the loop.  */
	      }
	  }

	_ =>
	  { }
	}
    }

  if ! jumped_to_done
    {
      if comstart_pos == 0
	{
	  from = comment_end;
	  from_byte = comment_end_byte;
	  UPDATE_SYNTAX_TABLE_FORWARD (comment_end);
	}
      /* If comstart_pos is set and we get here (ie. didn't jump to `lossage'
	 or `done'), then we've found the beginning of the non-nested comment.  */
      else if ! jumped_to_lossage	/* C's `else if (1)` */
	{
	  from = comstart_pos;
	  from_byte = comstart_byte;
	  UPDATE_SYNTAX_TABLE_FORWARD (from - 1);
	}
      else
	{ /* lossage: */
	  let mut state: lisp_parse_state = unsafe { std::mem::zeroed () };
	  let mut adjusted: bool = true;
	  /* We had two kinds of string delimiters mixed up
	     together.  Decode this going forwards.
	     Scan fwd from a known safe place (beginning-of-defun)
	     to the one in question; this records where we
	     last passed a comment starter.  */
	  /* If we did not already find the defun start, find it now.  */
	  if defun_start == 0
	    {
	      defun_start = find_defun_start (comment_end, comment_end_byte);
	      defun_start_byte = unsafe { find_start_value_byte };
	      adjusted = (defun_start > BEGV! ());
	    }
	  loop
	    {
              internalize_parse_state (Qnil (), &mut state);
	      scan_sexps_forward (&mut state,
				  defun_start, defun_start_byte,
				  comment_end, TYPE_MINIMUM! (EMACS_INT),
				  false, 0);
	      defun_start = comment_end;
	      if ! adjusted
		{
		  adjusted = true;
		  unsafe { find_start_value
		    = if CONSP (state.levelstarts) { XFIXNUM (XCAR (state.levelstarts)) }
		    else if state.thislevelstart >= 0 { state.thislevelstart }
		    else { find_start_value }; }
		  unsafe { find_start_value_byte = CHAR_TO_BYTE (find_start_value); }
		}

	      if state.incomment == (if comnested { 1 } else { -1 })
		  && state.comstyle == comstyle
		{ from = state.comstr_start; }
	      else
		{
		  from = comment_end;
		  if state.incomment != 0
		    /* If comment_end is inside some other comment, maybe ours
		       is nested, so we need to try again from within the
		       surrounding comment.  Example: { a (* " *)  */
		    {
		      /* FIXME: We should advance by one or two chars.  */
		      defun_start = state.comstr_start + 2;
		      defun_start_byte = CHAR_TO_BYTE (defun_start);
		    }
		}
	      quit_count = quit_count.wrapping_add (1);
	      rarely_quit (quit_count as u64);
	      if ! (defun_start < comment_end) { break; }
	    }

	  from_byte = CHAR_TO_BYTE (from);
	  UPDATE_SYNTAX_TABLE_FORWARD (from - 1);
	}
    }

  // done:
  unsafe { *charpos_ptr = from; }
  unsafe { *bytepos_ptr = from_byte; }

  return from != comment_end;
}

/* This array, indexed by a character less than 256, contains the syntax code which that
   character signifies, for `string-to-syntax'.  */
// C 1063-1082: `unsigned char const syntax_spec_code[0400]`; C zero-fills the 128 entries
// its initialiser does not name, so the port spells all 256 (PORTING.md 1.1).
#[unsafe(no_mangle)]
pub static syntax_spec_code: [u8; 256] =
  [
    0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377,
    0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377,
    0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377,
    0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377,
    Swhitespace as u8, Scomment_fence as u8, Sstring as u8, 0o377, Smath as u8, 0o377, 0o377, Squote as u8,
    Sopen as u8, Sclose as u8, 0o377, 0o377, 0o377, Swhitespace as u8, Spunct as u8, Scharquote as u8,
    0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377,
    0o377, 0o377, 0o377, 0o377, Scomment as u8, 0o377, Sendcomment as u8, 0o377,
    Sinherit as u8, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377,
    0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377,
    0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, Sword as u8,
    0o377, 0o377, 0o377, 0o377, Sescape as u8, 0o377, 0o377, Ssymbol as u8,
    0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377,
    0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377,
    0o377, 0o377, 0o377, 0o377, 0o377, 0o377, 0o377, Sword as u8,
    0o377, 0o377, 0o377, 0o377, Sstring_fence as u8, 0o377, 0o377, 0o377,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
  ];

/* Indexed by syntax code, give the letter that describes it.  */
static syntax_code_spec: [c_char; 16] =
  [
    b' ' as c_char, b'.' as c_char, b'w' as c_char, b'_' as c_char,
    b'(' as c_char, b')' as c_char, b'\'' as c_char, b'"' as c_char, b'$' as c_char,
    b'\\' as c_char, b'/' as c_char, b'<' as c_char, b'>' as c_char, b'@' as c_char,
    b'!' as c_char, b'|' as c_char
  ];

/* Indexed by syntax code, give the object (cons of syntax code and
   nil) to be stored in syntax table.  Since these objects can be
   shared among syntax tables, we generate them in advance.  By
   sharing objects, the function `describe-syntax' can give a more
   compact listing.  */
static mut Vsyntax_code_object: Lisp_Object = lisp_h_Qnil ();

unsafe fn check_syntax_table (obj: Lisp_Object)
{
  CHECK_TYPE (CHAR_TABLE_P (obj) && EQ (unsafe { (*XCHAR_TABLE (obj)).purpose },
					Qsyntax_table ()),
	      Qsyntax_table_p (), obj);
}

DEFUN! ("syntax-table-p", Fsyntax_table_p, Ssyntax_table_p, 1, 1, 0,
       doc: "Return t if OBJECT is a syntax table.\nCurrently, any char-table counts as a syntax table.",
       (object: Lisp_Object),
       {
  if CHAR_TABLE_P (object)
      && EQ (unsafe { (*XCHAR_TABLE (object)).purpose }, Qsyntax_table ())
    { return Qt (); }
  return Qnil ();
});

DEFUN! ("syntax-table", Fsyntax_table, Ssyntax_table, 0, 0, 0,
       doc: "Return the current syntax table.\nThis is the one specified by the current buffer.",
       (),
       {
  return BVAR! ((current_buffer! ()), syntax_table_);
});

DEFUN! ("standard-syntax-table", Fstandard_syntax_table, Sstandard_syntax_table, 0, 0, 0,
       doc: "Return the standard syntax table.\nThis is the one used for new buffers.",
       (),
       {
  return Vstandard_syntax_table! ();
});

DEFUN! ("copy-syntax-table", Fcopy_syntax_table, Scopy_syntax_table, 0, 1, 0,
       doc: "Construct a new syntax table and return it.\nIt is a copy of the TABLE, which defaults to the standard syntax table.",
       (table: Lisp_Object),
       {
  let copy: Lisp_Object;
  let mut table: Lisp_Object = table;	// C rebinds the argument below

  if ! NILP (table)
    { check_syntax_table (table); }
  else
    { table = Vstandard_syntax_table! (); }

  copy = unsafe { Fcopy_sequence (table) };

  /* Only the standard syntax table should have a default element.
     Other syntax tables should inherit from parents instead.  */
  set_char_table_defalt (copy, Qnil ());

  /* Copied syntax tables should all have parents.
     If we copied one with no parent, such as the standard syntax table,
     use the standard syntax table as the copy's parent.  */
  if NILP (unsafe { (*XCHAR_TABLE (copy)).parent })
    { unsafe { Fset_char_table_parent (copy, Vstandard_syntax_table! ()) }; }
  return copy;
});

DEFUN! ("set-syntax-table", Fset_syntax_table, Sset_syntax_table, 1, 1, 0,
       doc: "Select a new syntax table for the current buffer.\nOne argument, a syntax table.",
       (table: Lisp_Object),
       {
  let mut idx: c_int;
  check_syntax_table (table);
  bset_syntax_table (unsafe { (current_buffer! ()) }, table);
  /* Indicate that this buffer now has a specified syntax table.  */
  idx = PER_BUFFER_VAR_IDX! (syntax_table_);
  SET_PER_BUFFER_VALUE_P (unsafe { (current_buffer! ()) }, idx, true);
  return table;
});

DEFUN! ("char-syntax", Fchar_syntax, Schar_syntax, 1, 1, 0,
       doc: "Return the syntax code of CHARACTER, described by a character.\nFor example, if CHARACTER is a word constituent, the\ncharacter `w' (119) is returned.\nThe characters that correspond to various syntax codes\nare listed in the documentation of `modify-syntax-entry'.\n\nCaveat: if the current buffer is unibyte, this function interprets\nvalues between 128 and 255 as raw bytes, and returns the syntax of\nthe correspoinding `eight-bit' character.\n\nIf you're trying to determine the syntax of characters in the buffer,\nthis is probably the wrong function to use, because it can't take\n`syntax-table' text properties into account.  Consider using\n`syntax-after' instead.",
       (character: Lisp_Object),
       {
  CHECK_CHARACTER (character);
  let mut char_int: c_int = XFIXNAT (character) as c_int;
  SETUP_BUFFER_SYNTAX_TABLE ();
  if NILP (unsafe { BVAR! ((current_buffer! ()), enable_multibyte_characters_) })
      && SINGLE_BYTE_CHAR_P (char_int as intmax_t)
    { char_int = make_char_multibyte (char_int); }
  return make_fixnum (syntax_code_spec[SYNTAX (char_int) as usize] as EMACS_INT);
});

DEFUN! ("syntax-class-to-char", Fsyntax_class_to_char, Ssyntax_class_to_char, 1, 1, 0,
       doc: "Return the syntax char of CLASS, described by an integer.\nFor example, if SYNTAX is word constituent (the integer 2), the\ncharacter `w' (119) is returned.",
       (syntax: Lisp_Object),
       {
  let mut syn: c_int;
  CHECK_FIXNUM (syntax);
  syn = XFIXNUM (syntax) as c_int;

  if syn < 0 || syn >= size_of_val (&syntax_code_spec) as c_int
    { unsafe { args_out_of_range (make_fixnum (size_of_val (&syntax_code_spec) as EMACS_INT - 1),
			  syntax) }; }
  return make_fixnum (syntax_code_spec[syn as usize] as EMACS_INT);
});

DEFUN! ("matching-paren", Fmatching_paren, Smatching_paren, 1, 1, 0,
       doc: "Return the matching parenthesis of CHARACTER, or nil if none.",
       (character: Lisp_Object),
       {
  let mut char_int: c_int;
  let code: syntaxcode;
  CHECK_CHARACTER (character);
  char_int = XFIXNUM (character) as c_int;
  SETUP_BUFFER_SYNTAX_TABLE ();
  code = SYNTAX (char_int);
  if code == Sopen || code == Sclose
    { return SYNTAX_MATCH (char_int); }
  return Qnil ();
});

DEFUN! ("string-to-syntax", Fstring_to_syntax, Sstring_to_syntax, 1, 1, 0,
       doc: "Convert a syntax descriptor STRING into a raw syntax descriptor.\nSTRING should be a string of the form allowed as argument of\n`modify-syntax-entry'.  The return value is a raw syntax descriptor: a\ncons cell (CODE . MATCHING-CHAR) which can be used, for example, as\nthe value of a `syntax-table' text property.",
       (string: Lisp_Object),
       {
  let mut p: *const u8;
  let mut val: c_int;
  let mut match_lisp: Lisp_Object;

  CHECK_STRING (string);

  p = SDATA (string) as *const u8;
  val = syntax_spec_code[unsafe { *p } as usize] as c_int;
  p = unsafe { p.add (1) };
  if val == 0o377
    { unsafe { error (c"Invalid syntax description letter: %c".as_ptr (),
		      unsafe { *p.sub (1) } as c_int) }; }

  if val == Sinherit
    { return Qnil (); }

  if unsafe { *p } != 0
    {
      let mut len: c_int = 0;
      let character: c_int = string_char_and_length (p, &mut len);
      XSETINT! (match_lisp, character as EMACS_INT);
      if XFIXNAT (match_lisp) == b' ' as EMACS_INT
	{ match_lisp = Qnil (); }
      p = unsafe { p.add (len as usize) };
    }
  else
    { match_lisp = Qnil (); }

  while unsafe { *p } != 0
    {
      let ch: u8 = unsafe { *p };
      p = unsafe { p.add (1) };
      match ch
	{
	b'1' =>
	  { val |= 1 << 16; }

	b'2' =>
	  { val |= 1 << 17; }

	b'3' =>
	  { val |= 1 << 18; }

	b'4' =>
	  { val |= 1 << 19; }

	b'p' =>
	  { val |= 1 << 20; }

	b'b' =>
	  { val |= 1 << 21; }

	b'n' =>
	  { val |= 1 << 22; }

	b'c' =>
	  { val |= 1 << 23; }

	_ =>
	  { }
	}
    }

  if (val as isize) < ASIZE (unsafe { Vsyntax_code_object }) && NILP (match_lisp)
    { return AREF (unsafe { Vsyntax_code_object }, val as isize); }
  else
    /* Since we can't use a shared object, let's make a new one.  */
    { return unsafe { Fcons (make_fixnum (val as EMACS_INT), match_lisp) }; }
});

DEFUN! ("modify-syntax-entry", Fmodify_syntax_entry, Smodify_syntax_entry, 2, 3, "cSet syntax for character: \nsSet syntax for %s to: ",
       doc: "Set syntax for character CHAR according to string NEWENTRY.\nThe syntax is changed only for table SYNTAX-TABLE, which defaults to\n the current buffer's syntax table.\nCHAR may be a cons (MIN . MAX), in which case, syntaxes of all characters\nin the range MIN to MAX are changed.\nThe first character of NEWENTRY should be one of the following:\n  Space or -  whitespace syntax.    w   word constituent.\n  _           symbol constituent.   .   punctuation.\n  (           open-parenthesis.     )   close-parenthesis.\n  \"           string quote.         \\   escape.\n  $           paired delimiter.     \\='   expression quote or prefix operator.\n  <           comment starter.      >   comment ender.\n  /           character-quote.      @   inherit from parent table.\n  |           generic string fence. !   generic comment fence.\n\nOnly single-character comment start and end sequences are represented thus.\nTwo-character sequences are represented as described below.\nThe second character of NEWENTRY is the matching parenthesis,\n used only if the first character is `(' or `)'.\nAny additional characters are flags.\nDefined flags are the characters 1, 2, 3, 4, b, p, and n.\n 1 means CHAR is the start of a two-char comment start sequence.\n 2 means CHAR is the second character of such a sequence.\n 3 means CHAR is the start of a two-char comment end sequence.\n 4 means CHAR is the second character of such a sequence.\n\nThere can be several orthogonal comment sequences.  This is to support\nlanguage modes such as C++.  By default, all comment sequences are of style\na, but you can set the comment sequence style to b (on the second character\nof a comment-start, and the first character of a comment-end sequence) and/or\nc (on any of its chars) using this flag:\n b means CHAR is part of comment sequence b.\n c means CHAR is part of comment sequence c.\n n means CHAR is part of a nestable comment sequence.\n\n p means CHAR is a prefix character for `backward-prefix-chars';\n   such characters are treated as whitespace when they occur\n   between expressions.\nusage: (modify-syntax-entry CHAR NEWENTRY &optional SYNTAX-TABLE)",
       (c: Lisp_Object, newentry: Lisp_Object, syntax_table: Lisp_Object),
       {
  // C 1298 calls `clear_regexp_cache` (search.h, whose mirror does not exist yet), so it is
  // declared at its use site inside this function (the doc.rs/casefiddle.rs shape).
  unsafe extern "C" { fn clear_regexp_cache (); }

  if CONSP (c)
    {
      CHECK_CHARACTER_CAR (c);
      CHECK_CHARACTER_CDR (c);
    }
  else
    { CHECK_CHARACTER (c); }

  let mut syntax_table: Lisp_Object = syntax_table;	// C rebinds the argument below
  if NILP (syntax_table)
    { syntax_table = BVAR! ((current_buffer! ()), syntax_table_); }
  else
    { check_syntax_table (syntax_table); }

  let newentry: Lisp_Object = unsafe { Fstring_to_syntax (newentry) };
  if CONSP (c)
    { SET_RAW_SYNTAX_ENTRY_RANGE (syntax_table, c, newentry); }
  else
    { SET_RAW_SYNTAX_ENTRY (syntax_table, XFIXNUM (c) as c_int, newentry); }

  /* We clear the regexp cache, since character classes can now have
     different values from those in the compiled regexps.*/
  unsafe { clear_regexp_cache () };

  return Qnil ();
});


/* Return the position across COUNT words from FROM.
   If that many words cannot be found before the end of the buffer, return 0.
   COUNT negative means scan backward and stop at word beginning.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scan_words (from: ptrdiff_t, count: EMACS_INT) -> ptrdiff_t
{
  let beg: ptrdiff_t = BEGV! ();
  let end: ptrdiff_t = ZV! ();
  let mut from = from;
  let mut from_byte: ptrdiff_t = CHAR_TO_BYTE (from);
  let mut code: syntaxcode;
  let mut ch0: c_int = 0;	/* C leaves these uninitialised (0.1-13).  */
  let mut ch1: c_int = 0;
  let mut func: Lisp_Object;
  let mut pos: Lisp_Object;
  let mut count = count;

  SETUP_SYNTAX_TABLE (from, clip_to_bounds (PTRDIFF_MIN, count, PTRDIFF_MAX));

  while count > 0
    {
      while true
	{
	  if from == end
	    { return 0; }
	  UPDATE_SYNTAX_TABLE_FORWARD (from);
	  ch0 = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	  code = SYNTAX (ch0);
	  inc_both (&mut from, &mut from_byte);
	  if unsafe { words_include_escapes }
	      && (code == Sescape || code == Scharquote)
	    { break; }
	  if code == Sword
	    { break; }
	  rarely_quit (from as u64);
	}
      /* Now CH0 is a character which begins a word and FROM is the
         position of the next character.  */
      func = CHAR_TABLE_REF (unsafe { Vfind_word_boundary_function_table }, ch0 as isize);
      if ! NILP (unsafe { Ffboundp (func) })
	{
	  pos = calln! (func, make_fixnum (from - 1), make_fixnum (end));
	  if FIXNUMP (pos) && from < XFIXNUM (pos) && XFIXNUM (pos) <= ZV! ()
	    {
	      from = XFIXNUM (pos);
	      from_byte = CHAR_TO_BYTE (from);
	    }
	}
      else
	{
	  loop
	    {
	      if from == end { break; }
	      UPDATE_SYNTAX_TABLE_FORWARD (from);
	      ch1 = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	      code = SYNTAX (ch1);
	      if ((code != Sword
		   && (! unsafe { words_include_escapes }
		       || (code != Sescape && code != Scharquote)))
		  || word_boundary_p (ch0, ch1))
		{ break; }
	      inc_both (&mut from, &mut from_byte);
	      ch0 = ch1;
	      rarely_quit (from as u64);
	    }
	}
      count -= 1;
    }
  while count < 0
    {
      while true
	{
	  if from == beg
	    { return 0; }
	  dec_both (&mut from, &mut from_byte);
	  UPDATE_SYNTAX_TABLE_BACKWARD (from);
	  ch1 = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	  code = SYNTAX (ch1);
	  if unsafe { words_include_escapes }
	      && (code == Sescape || code == Scharquote)
	    { break; }
	  if code == Sword
	    { break; }
	  rarely_quit (from as u64);
	}
      /* Now CH1 is a character which ends a word and FROM is the
         position of it.  */
      func = CHAR_TABLE_REF (unsafe { Vfind_word_boundary_function_table }, ch1 as isize);
      if ! NILP (unsafe { Ffboundp (func) })
 	{
	  pos = calln! (func, make_fixnum (from), make_fixnum (beg));
	  if FIXNUMP (pos) && BEGV! () <= XFIXNUM (pos) && XFIXNUM (pos) < from
	    {
	      from = XFIXNUM (pos);
	      from_byte = CHAR_TO_BYTE (from);
	    }
	}
      else
	{
	  loop
	    {
	      if from == beg
		{ break; }
	      dec_both (&mut from, &mut from_byte);
	      UPDATE_SYNTAX_TABLE_BACKWARD (from);
	      ch0 = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	      code = SYNTAX (ch0);
	      if ((code != Sword
		   && (! unsafe { words_include_escapes }
		       || (code != Sescape && code != Scharquote)))
		  || word_boundary_p (ch0, ch1))
		{
		  inc_both (&mut from, &mut from_byte);
		  break;
		}
	      ch1 = ch0;
	      rarely_quit (from as u64);
	    }
	}
      count += 1;
    }

  return from;
}

DEFUN! ("internal-describe-syntax-value", Finternal_describe_syntax_value, Sinternal_describe_syntax_value, 1, 1, 0,
       doc: "Insert a description of the internal syntax description SYNTAX at point.",
       (syntax: Lisp_Object),
       {
  let mut code: c_int;
  let syntax_code: c_int;
  let start1: bool; let start2: bool; let end1: bool; let end2: bool;
  let prefix: bool; let comstyleb: bool; let comstylec: bool; let comnested: bool;
  let mut str_: [c_char; 2] = [0; 2];
  let first: Lisp_Object; let match_lisp: Lisp_Object;
  let value: Lisp_Object = syntax;

  if NILP (value)
    {
      unsafe { insert_string (c"default".as_ptr ()) };
      return syntax;
    }

  if CHAR_TABLE_P (value)
    {
      unsafe { insert_string (c"deeper char-table ...".as_ptr ()) };
      return syntax;
    }

  if ! CONSP (value)
    {
      unsafe { insert_string (c"invalid".as_ptr ()) };
      return syntax;
    }

  first = XCAR (value);
  match_lisp = XCDR (value);

  if ! FIXNUMP (first) || ! (NILP (match_lisp) || CHARACTERP (match_lisp))
    {
      unsafe { insert_string (c"invalid".as_ptr ()) };
      return syntax;
    }

  syntax_code = XFIXNUM (first) as c_int & INT_MAX;
  code = syntax_code & 0o377;
  start1 = SYNTAX_FLAGS_COMSTART_FIRST (syntax_code);
  start2 = SYNTAX_FLAGS_COMSTART_SECOND (syntax_code);
  end1 = SYNTAX_FLAGS_COMEND_FIRST (syntax_code);
  end2 = SYNTAX_FLAGS_COMEND_SECOND (syntax_code);
  prefix = SYNTAX_FLAGS_PREFIX (syntax_code);
  comstyleb = SYNTAX_FLAGS_COMMENT_STYLEB (syntax_code);
  comstylec = SYNTAX_FLAGS_COMMENT_STYLEC (syntax_code);
  comnested = SYNTAX_FLAGS_COMMENT_NESTED (syntax_code);

  if Smax <= code
    {
      unsafe { insert_string (c"invalid".as_ptr ()) };
      return syntax;
    }

  str_[0] = syntax_code_spec[code as usize];
  str_[1] = 0;
  unsafe { insert (str_.as_ptr (), 1) };

  if NILP (match_lisp)
    { unsafe { insert (c" ".as_ptr (), 1) }; }
  else
    { unsafe { insert_char (XFIXNUM (match_lisp) as c_int) }; }

  if start1
    { unsafe { insert (c"1".as_ptr (), 1) }; }
  if start2
    { unsafe { insert (c"2".as_ptr (), 1) }; }

  if end1
    { unsafe { insert (c"3".as_ptr (), 1) }; }
  if end2
    { unsafe { insert (c"4".as_ptr (), 1) }; }

  if prefix
    { unsafe { insert (c"p".as_ptr (), 1) }; }
  if comstyleb
    { unsafe { insert (c"b".as_ptr (), 1) }; }
  if comstylec
    { unsafe { insert (c"c".as_ptr (), 1) }; }
  if comnested
    { unsafe { insert (c"n".as_ptr (), 1) }; }

  unsafe { insert_string (c"\twhich means: ".as_ptr ()) };

  match code
    {
    Swhitespace =>
      { unsafe { insert_string (c"whitespace".as_ptr ()) }; }
    Spunct =>
      { unsafe { insert_string (c"punctuation".as_ptr ()) }; }
    Sword =>
      { unsafe { insert_string (c"word".as_ptr ()) }; }
    Ssymbol =>
      { unsafe { insert_string (c"symbol".as_ptr ()) }; }
    Sopen =>
      { unsafe { insert_string (c"open".as_ptr ()) }; }
    Sclose =>
      { unsafe { insert_string (c"close".as_ptr ()) }; }
    Squote =>
      { unsafe { insert_string (c"prefix".as_ptr ()) }; }
    Sstring =>
      { unsafe { insert_string (c"string".as_ptr ()) }; }
    Smath =>
      { unsafe { insert_string (c"math".as_ptr ()) }; }
    Sescape =>
      { unsafe { insert_string (c"escape".as_ptr ()) }; }
    Scharquote =>
      { unsafe { insert_string (c"charquote".as_ptr ()) }; }
    Scomment =>
      { unsafe { insert_string (c"comment".as_ptr ()) }; }
    Sendcomment =>
      { unsafe { insert_string (c"endcomment".as_ptr ()) }; }
    Sinherit =>
      { unsafe { insert_string (c"inherit".as_ptr ()) }; }
    Scomment_fence =>
      { unsafe { insert_string (c"comment fence".as_ptr ()) }; }
    Sstring_fence =>
      { unsafe { insert_string (c"string fence".as_ptr ()) }; }
    _ =>
      {
	unsafe { insert_string (c"invalid".as_ptr ()) };
	return syntax;
      }
    }

  if ! NILP (match_lisp)
    {
      unsafe { insert_string (c", matches ".as_ptr ()) };
      unsafe { insert_char (XFIXNUM (match_lisp) as c_int) };
    }

  if start1
    { unsafe { insert_string (c",\n\t  is the first character of a comment-start sequence".as_ptr ()) }; }
  if start2
    { unsafe { insert_string (c",\n\t  is the second character of a comment-start sequence".as_ptr ()) }; }

  if end1
    { unsafe { insert_string (c",\n\t  is the first character of a comment-end sequence".as_ptr ()) }; }
  if end2
    { unsafe { insert_string (c",\n\t  is the second character of a comment-end sequence".as_ptr ()) }; }
  if comstyleb
    { unsafe { insert_string (c" (comment style b)".as_ptr ()) }; }
  if comstylec
    { unsafe { insert_string (c" (comment style c)".as_ptr ()) }; }
  if comnested
    { unsafe { insert_string (c" (nestable)".as_ptr ()) }; }

  if prefix
    {
      AUTO_STRING! (prefixdoc,
		    b",\n\t  is a prefix character for `backward-prefix-chars'\0".as_ptr () as *const c_char);
      unsafe { insert1 (calln! (Qsubstitute_command_keys (), prefixdoc)) };
    }

  return syntax;
});

DEFUN! ("forward-word", Fforward_word, Sforward_word, 0, 1, "^p",
       doc: "Move point forward ARG words (backward if ARG is negative).\nIf ARG is omitted or nil, move point forward one word.\nNormally returns t.\nIf an edge of the buffer or a field boundary is reached, point is\nleft there and the function returns nil.  Field boundaries are not\nnoticed if `inhibit-field-text-motion' is non-nil.\n\nThe word boundaries are normally determined by the buffer's syntax\ntable and character script (according to `char-script-table'), but\n`find-word-boundary-function-table', such as set up by `subword-mode',\ncan change that.  If a Lisp program needs to move by words determined\nstrictly by the syntax table, it should use `forward-word-strictly'\ninstead.  See Info node `(elisp) Word Motion' for details.",
       (arg: Lisp_Object),
       {
  let tmp: Lisp_Object;
  let mut orig_val: ptrdiff_t;
  let mut val: ptrdiff_t;
  let mut arg: Lisp_Object = arg;	// C's `XSETFASTINT (arg, 1)` below rebinds it

  if NILP (arg)
    { XSETFASTINT! (arg, 1); }
  else
    { CHECK_FIXNUM (arg); }

  val = unsafe { scan_words (PT! (), XFIXNUM (arg)) };
  orig_val = val;
  if orig_val == 0
    { val = if XFIXNUM (arg) > 0 { ZV! () } else { BEGV! () }; }

  /* Avoid jumping out of an input field.  */
  tmp = unsafe { Fconstrain_to_field (make_fixnum (val), make_fixnum (PT! ()),
				       Qnil (), Qnil (), Qnil ()) };
  val = XFIXNAT (tmp);

  SET_PT (val);
  return if val == orig_val { Qt () } else { Qnil () };
});

DEFUN! ("skip-chars-forward", Fskip_chars_forward, Sskip_chars_forward, 1, 2, 0,
       doc: "Move point forward, stopping before a char not in STRING, or at pos LIM.\nSTRING is like the inside of a `[...]' in a regular expression\nexcept that `]' is never special and `\\' quotes `^', `-' or `\\'\n (but not at the end of a range; quoting is never needed there).\nThus, with arg \"a-zA-Z\", this skips letters stopping before first nonletter.\nWith arg \"^a-zA-Z\", skips nonletters stopping before first letter.\nChar classes, e.g. `[:alpha:]', are supported.\n\nReturns the distance traveled, either zero or positive.",
       (string: Lisp_Object, lim: Lisp_Object),
       {
  return skip_chars (true, string, lim);
});

DEFUN! ("skip-chars-backward", Fskip_chars_backward, Sskip_chars_backward, 1, 2, 0,
       doc: "Move point backward, stopping after a char not in STRING, or at pos LIM.\nSee `skip-chars-forward' for details.\nReturns the distance traveled, either zero or negative.",
       (string: Lisp_Object, lim: Lisp_Object),
       {
  return skip_chars (false, string, lim);
});

DEFUN! ("skip-syntax-forward", Fskip_syntax_forward, Sskip_syntax_forward, 1, 2, 0,
       doc: "Move point forward across chars in specified syntax classes.\nSYNTAX is a string of syntax code characters.\nStop before a char whose syntax is not in SYNTAX, or at position LIM.\nIf SYNTAX starts with ^, skip characters whose syntax is NOT in SYNTAX.\nThis function returns the distance traveled, either zero or positive.",
       (syntax: Lisp_Object, lim: Lisp_Object),
       {
  return skip_syntaxes (true, syntax, lim);
});

DEFUN! ("skip-syntax-backward", Fskip_syntax_backward, Sskip_syntax_backward, 1, 2, 0,
       doc: "Move point backward across chars in specified syntax classes.\nSYNTAX is a string of syntax code characters.\nStop on reaching a char whose syntax is not in SYNTAX, or at position LIM.\nIf SYNTAX starts with ^, skip characters whose syntax is NOT in SYNTAX.\nThis function returns either zero or a negative number, and the absolute value\nof this is the distance traveled.",
       (syntax: Lisp_Object, lim: Lisp_Object),
       {
  return skip_syntaxes (false, syntax, lim);
});

/* Called from skip-chars-forward/backward.  */

unsafe fn skip_chars (forwardp: bool, string: Lisp_Object, lim: Lisp_Object) -> Lisp_Object
{
  let mut c: c_int;
  let mut fastmap: [c_char; 0o400] = [0; 0o400];
  /* Store the ranges of non-ASCII characters.  */
  // C's `int *char_ranges UNINIT` is filled by SAFE_NALLOCA, whose port macro reads the
  // pointee's size from the binding, so it starts null (the callint.rs SAFE_ALLOCA shape).
  let mut char_ranges: *mut c_int = std::ptr::null_mut ();
  let mut n_char_ranges: c_int = 0;
  let mut negate: bool = false;
  let mut i: ptrdiff_t;
  let mut i_byte: ptrdiff_t;
  /* True if the current buffer is multibyte and the region contains
     non-ASCII chars.  */
  let mut multibyte: bool;
  /* True if STRING is multibyte and it contains non-ASCII chars.  */
  let mut string_multibyte: bool;
  let mut size_byte: ptrdiff_t;
  let mut str_: *const u8;
  let mut len: c_int = 0;
  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  CHECK_STRING (string);

  let mut lim: Lisp_Object = lim;	// C writes into LIM below

  if NILP (lim)
    { XSETINT! (lim, if forwardp { ZV! () } else { BEGV! () }); }
  else
    { CHECK_FIXNUM_COERCE_MARKER! (lim); }

  /* In any case, don't allow scan outside bounds of buffer.  */
  if XFIXNUM (lim) > ZV! ()
    { XSETFASTINT! (lim, ZV! ()); }
  if XFIXNUM (lim) < BEGV! ()
    { XSETFASTINT! (lim, BEGV! ()); }

  multibyte = (! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	       && (XFIXNUM (lim) - PT! () != CHAR_TO_BYTE (XFIXNUM (lim)) - PT_BYTE! ()));
  string_multibyte = SBYTES (string) > SCHARS (string);

  unsafe { memset (fastmap.as_mut_ptr () as *mut c_void, 0, size_of_val (&fastmap)) };

  str_ = SDATA (string) as *const u8;
  size_byte = SBYTES (string);

  i_byte = 0;
  if i_byte < size_byte
      && SREF (string, 0) == b'^' as u8
    {
      negate = true; i_byte += 1;
    }

  /* Find the characters specified and set their elements of fastmap.
     Handle backslashes and ranges specially.

     If STRING contains non-ASCII characters, setup char_ranges for
     them and use fastmap only for their leading codes.  */

  let mut nclasses: c_int = 0;
  let mut classes: [u8; RECC_NUM_CLASSES as usize] = [0; RECC_NUM_CLASSES as usize];
  if ! string_multibyte
    {
      let mut string_has_eight_bit: bool = false;

      /* At first setup fastmap.  */
      while i_byte < size_byte
	{
	  let mut ch: *const u8 = unsafe { str_.add (i_byte as usize) };
	  let cc: re_wctype_t = unsafe { re_wctype_parse (&mut ch, size_byte - i_byte) };
	  if cc == 0
	    { unsafe { error (c"Invalid ISO C character class".as_ptr ()) }; }
	  if cc != -1
	    {
	      if ! (nclasses != 0 && ! unsafe { memchr (classes.as_ptr () as *const c_void,
						       cc, nclasses as usize) }.is_null ())
		{ classes[nclasses as usize] = cc as u8; nclasses += 1; }
	      i_byte = unsafe { ch.offset_from (str_) };
	      continue;
	    }

	  c = unsafe { *str_.add (i_byte as usize) } as c_int;
	  i_byte += 1;

	  if c == '\\' as c_int
	    {
	      if i_byte == size_byte
		{ break; }

	      c = unsafe { *str_.add (i_byte as usize) } as c_int;
	      i_byte += 1;
	    }
	  /* Treat `-' as range character only if another character
	     follows.  */
	  if i_byte + 1 < size_byte
	      && unsafe { *str_.add (i_byte as usize) } == b'-'
	    {
	      let mut c2: c_int;

	      /* Skip over the dash.  */
	      i_byte += 1;

	      /* Get the end of the range.  */
	      c2 = unsafe { *str_.add (i_byte as usize) } as c_int;
	      i_byte += 1;
	      if c2 == '\\' as c_int
		  && i_byte < size_byte
		{ c2 = unsafe { *str_.add (i_byte as usize) } as c_int; i_byte += 1; }

	      if c <= c2
		{
		  let lim2: c_int = c2 + 1;
		  while c < lim2
		    { fastmap[c as usize] = 1; c += 1; }
		  if ! ASCII_CHAR_P (c2 as isize)
		    { string_has_eight_bit = true; }
		}
	    }
	  else
	    {
	      fastmap[c as usize] = 1;
	      if ! ASCII_CHAR_P (c as isize)
		{ string_has_eight_bit = true; }
	    }
	}

      /* If the current range is multibyte and STRING contains
	 eight-bit chars, arrange fastmap and setup char_ranges for
	 the corresponding multibyte chars.  */
      if multibyte && string_has_eight_bit
	{
	  let mut p1: *mut c_char;
	  let mut himap: [c_char; 0o200 + 1] = [0; 0o200 + 1];
	  unsafe { memcpy (himap.as_mut_ptr () as *mut c_void,
			   fastmap.as_ptr ().add (0o200) as *const c_void, 0o200) };
	  himap[0o200] = 0;
	  unsafe { memset (fastmap.as_mut_ptr ().add (0o200) as *mut c_void, 0, 0o200) };
	  SAFE_NALLOCA! (sa_avail, char_ranges, 2, 128);
	  i = 0;

	  loop
	    {
	      p1 = unsafe { memchr (himap.as_ptr ().add (i as usize) as *const c_void,
				    1, (0o200 - i) as usize) } as *mut c_char;
	      if p1.is_null () { break; }
	      /* Deduce the next range C..C2 from the next clump of 1s
		 in HIMAP starting with &HIMAP[I].  HIMAP is the high
		 order half of the old FASTMAP.  */
	      let mut c2: c_int;
	      let mut leading_code: c_int;
	      i = unsafe { p1.offset_from (himap.as_ptr ()) } as ptrdiff_t;
	      c = BYTE8_TO_CHAR ((i + 0o200) as c_int);
	      i += unsafe { strlen (p1 as *const c_char) } as ptrdiff_t;
	      c2 = BYTE8_TO_CHAR ((i + 0o200 - 1) as c_int);

	      unsafe { *char_ranges.add (n_char_ranges as usize) = c; }
	      n_char_ranges += 1;
	      unsafe { *char_ranges.add (n_char_ranges as usize) = c2; }
	      n_char_ranges += 1;
	      leading_code = CHAR_LEADING_CODE (c);
	      unsafe { memset (fastmap.as_mut_ptr ().add (leading_code as usize) as *mut c_void, 1,
			       (CHAR_LEADING_CODE (c2) - leading_code + 1) as usize) };
	    }
	}
    }
  else				/* STRING is multibyte */
    {
      SAFE_NALLOCA! (sa_avail, char_ranges, 2, SCHARS (string));

      while i_byte < size_byte
	{
	  let mut leading_code: c_int = unsafe { *str_.add (i_byte as usize) } as c_int;

	  let mut ch: *const u8 = unsafe { str_.add (i_byte as usize) };
	  let cc: re_wctype_t = unsafe { re_wctype_parse (&mut ch, size_byte - i_byte) };
	  if cc == 0
	    { unsafe { error (c"Invalid ISO C character class".as_ptr ()) }; }
	  if cc != -1
	    {
	      if ! (nclasses != 0 && ! unsafe { memchr (classes.as_ptr () as *const c_void,
						       cc, nclasses as usize) }.is_null ())
		{ classes[nclasses as usize] = cc as u8; nclasses += 1; }
	      i_byte = unsafe { ch.offset_from (str_) };
	      continue;
	    }

	  if leading_code == '\\' as c_int
	    {
	      i_byte += 1;
	      if i_byte == size_byte
		{ break; }

	      leading_code = unsafe { *str_.add (i_byte as usize) } as c_int;
	    }
	  c = string_char_and_length (unsafe { str_.add (i_byte as usize) }, &mut len);
	  i_byte += len as ptrdiff_t;


	  /* Treat `-' as range character only if another character
	     follows.  */
	  if i_byte + 1 < size_byte
	      && unsafe { *str_.add (i_byte as usize) } == b'-'
	    {
	      let mut c2: c_int;
	      let mut leading_code2: c_int;

	      /* Skip over the dash.  */
	      i_byte += 1;

	      /* Get the end of the range.  */
	      leading_code2 = unsafe { *str_.add (i_byte as usize) } as c_int;
	      c2 = string_char_and_length (unsafe { str_.add (i_byte as usize) }, &mut len);
	      i_byte += len as ptrdiff_t;

	      if c2 == '\\' as c_int
		  && i_byte < size_byte
		{
		  leading_code2 = unsafe { *str_.add (i_byte as usize) } as c_int;
		  c2 = string_char_and_length (unsafe { str_.add (i_byte as usize) }, &mut len);
		  i_byte += len as ptrdiff_t;
		}

	      if c > c2
		{ continue; }
	      if ASCII_CHAR_P (c as isize)
		{
		  while c <= c2 && c < 0x80
		    { fastmap[c as usize] = 1; c += 1; }
		  leading_code = CHAR_LEADING_CODE (c);
		}
	      if ! ASCII_CHAR_P (c as isize)
		{
		  let lim2: c_int = leading_code2 + 1;
		  while leading_code < lim2
		    { fastmap[leading_code as usize] = 1; leading_code += 1; }
		  if c <= c2
		    {
		      unsafe { *char_ranges.add (n_char_ranges as usize) = c; }
		      n_char_ranges += 1;
		      unsafe { *char_ranges.add (n_char_ranges as usize) = c2; }
		      n_char_ranges += 1;
		    }
		}
	    }
	  else
	    {
	      if ASCII_CHAR_P (c as isize)
		{ fastmap[c as usize] = 1; }
	      else
		{
		  fastmap[leading_code as usize] = 1;
		  unsafe { *char_ranges.add (n_char_ranges as usize) = c; }
		  n_char_ranges += 1;
		  unsafe { *char_ranges.add (n_char_ranges as usize) = c; }
		  n_char_ranges += 1;
		}
	    }
	}

      /* If the current range is unibyte and STRING contains non-ASCII
	 chars, arrange fastmap for the corresponding unibyte
	 chars.  */

      if ! multibyte && n_char_ranges > 0
	{
	  unsafe { memset (fastmap.as_mut_ptr ().add (0o200) as *mut c_void, 0, 0o200) };
	  i = 0;
	  while i < n_char_ranges as ptrdiff_t
	    {
	      let c1_: c_int = unsafe { *char_ranges.add (i as usize) };
	      let lim2: c_int = unsafe { *char_ranges.add (i as usize + 1) } + 1;
	      let mut c1: c_int = c1_;

	      while c1 < lim2
		{
		  let b: c_int = CHAR_TO_BYTE_SAFE (c1);
		  if b >= 0
		    { fastmap[b as usize] = 1; }
		  c1 += 1;
		}
	      i += 2;
	    }
	}
    }

  /* If ^ was the first character, complement the fastmap.  */
  if negate
    {
      if ! multibyte
	{
	  i = 0;
	  while i < size_of_val (&fastmap) as ptrdiff_t
	    { fastmap[i as usize] ^= 1; i += 1; }
	}
      else
	{
	  i = 0;
	  while i < 0o200
	    { fastmap[i as usize] ^= 1; i += 1; }
	  /* All non-ASCII chars possibly match.  */
	  while i < size_of_val (&fastmap) as ptrdiff_t
	    { fastmap[i as usize] = 1; i += 1; }
	}
    }

  {
    let start_point: ptrdiff_t = PT! ();
    let mut pos: ptrdiff_t = PT! ();
    let mut pos_byte: ptrdiff_t = PT_BYTE! ();
    let mut p: *mut u8 = PT_ADDR! ();
    let mut endp: *mut u8;
    let mut stop: *mut u8;

    if forwardp
      {
	endp = if XFIXNUM (lim) == GPT! () { GPT_ADDR! () } else { CHAR_POS_ADDR (XFIXNUM (lim)) };
	stop = if pos < GPT! () && GPT! () < XFIXNUM (lim) { GPT_ADDR! () } else { endp };
      }
    else
      {
	endp = CHAR_POS_ADDR (XFIXNUM (lim));
	stop = if pos >= GPT! () && GPT! () > XFIXNUM (lim) { GAP_END_ADDR! () } else { endp };
      }

    /* This code may look up syntax tables using functions that rely on the
       gl_state object.  To make sure this object is not out of date,
       let's initialize it manually.
       We ignore syntax-table text-properties for now, since that's
       what we've done in the past.  */
    SETUP_BUFFER_SYNTAX_TABLE ();
    if forwardp
      {
	if multibyte
	  {
	    'fwd: loop
	      {
		let mut nbytes: c_int = 0;

		if p >= stop
		  {
		    if p >= endp
		      { break 'fwd; }
		    p = GAP_END_ADDR! ();
		    stop = endp;
		  }
		c = string_char_and_length (p, &mut nbytes);
		/* C's `goto fwd_ok` (C 1975) skips the two checks below.  */
		'fwd_ok: {
		  if nclasses != 0 && in_classes (c, nclasses, classes.as_ptr () as *const u8)
		    {
		      if negate
			{ break 'fwd; }
		      else
			{ break 'fwd_ok; }
		    }

		  if fastmap[unsafe { *p } as usize] == 0
		    { break 'fwd; }
		  if ! ASCII_CHAR_P (c as isize)
		    {
		      /* As we are looking at a multibyte character, we
			 must look up the character in the table
			 CHAR_RANGES.  If there's no data in the table,
			 that character is not what we want to skip.  */

		      /* The following code do the right thing even if
			 n_char_ranges is zero (i.e. no data in
			 CHAR_RANGES).  */
		      i = 0;
		      while i < n_char_ranges as ptrdiff_t
			{
			  if c >= unsafe { *char_ranges.add (i as usize) }
			      && c <= unsafe { *char_ranges.add (i as usize + 1) }
			    { break; }
			  i += 2;
			}
		      if ! (negate ^ (i < n_char_ranges as ptrdiff_t))
			{ break 'fwd; }
		    }
		  }
		/* fwd_ok: */
		p = unsafe { p.add (nbytes as usize) };
		pos += 1;
		pos_byte += nbytes as ptrdiff_t;
		rarely_quit (pos as u64);
	      }
	  }
	else
	  {
	    'fwd_unibyte: loop
	      {
		if p >= stop
		  {
		    if p >= endp
		      { break 'fwd_unibyte; }
		    p = GAP_END_ADDR! ();
		    stop = endp;
		  }

		/* C's `goto fwd_unibyte_ok` (C 1991) skips the check below.  */
		'fwd_unibyte_ok: {
		  if nclasses != 0 && in_classes (unsafe { *p } as c_int, nclasses,
						  classes.as_ptr () as *const u8)
		    {
		      if negate
			{ break 'fwd_unibyte; }
		      else
			{ break 'fwd_unibyte_ok; }
		    }

		  if fastmap[unsafe { *p } as usize] == 0
		    { break 'fwd_unibyte; }
		  }
		/* fwd_unibyte_ok: */
		p = unsafe { p.add (1) };
		pos += 1;
		pos_byte += 1;
		rarely_quit (pos as u64);
	      }
	  }
      }
    else
      {
	if multibyte
	  {
	    'back: loop
	      {
		if p <= stop
		  {
		    if p <= endp
		      { break 'back; }
		    p = GPT_ADDR! ();
		    stop = endp;
		  }
		let prev_p: *mut u8 = p;
		loop
		  {
		    p = unsafe { p.sub (1) };
		    if ! (stop <= p && ! CHAR_HEAD_P (unsafe { *p } as c_int)) { break; }
		  }

		c = STRING_CHAR (p);

		/* C's `goto back_ok` (C 2021) skips the checks below.  */
		'back_ok: {
		  if nclasses != 0 && in_classes (c, nclasses, classes.as_ptr () as *const u8)
		    {
		      if negate
			{ break 'back; }
		      else
			{ break 'back_ok; }
		    }

		  if fastmap[unsafe { *p } as usize] == 0
		    { break 'back; }
		  if ! ASCII_CHAR_P (c as isize)
		    {
		      /* See the comment in the previous similar code.  */
		      i = 0;
		      while i < n_char_ranges as ptrdiff_t
			{
			  if c >= unsafe { *char_ranges.add (i as usize) }
			      && c <= unsafe { *char_ranges.add (i as usize + 1) }
			    { break; }
			  i += 2;
			}
		      if ! (negate ^ (i < n_char_ranges as ptrdiff_t))
			{ break 'back; }
		    }
		  }
		/* back_ok: */
		pos -= 1;
		pos_byte -= unsafe { prev_p.offset_from (p) };
		rarely_quit (pos as u64);
	      }
	  }
	else
	  {
	    'back_unibyte: loop
	      {
		if p <= stop
		  {
		    if p <= endp
		      { break 'back_unibyte; }
		    p = GPT_ADDR! ();
		    stop = endp;
		  }

		/* C's `goto back_unibyte_ok` (C 2035) skips the check below.  */
		'back_unibyte_ok: {
		  if nclasses != 0 && in_classes (unsafe { *p.sub (1) } as c_int, nclasses,
						  classes.as_ptr () as *const u8)
		    {
		      if negate
			{ break 'back_unibyte; }
		      else
			{ break 'back_unibyte_ok; }
		    }

		  if fastmap[unsafe { *p.sub (1) } as usize] == 0
		    { break 'back_unibyte; }
		  }
		/* back_unibyte_ok: */
		p = unsafe { p.sub (1) };
		pos -= 1;
		pos_byte -= 1;
		rarely_quit (pos as u64);
	      }
	  }
      }

    SET_PT_BOTH (pos, pos_byte);

    SAFE_FREE! (sa_count);
    return make_fixnum (PT! () - start_point);
  }
}


/* Same as skip_chars, but in STRING of syntax classes rather than of characters.  */

unsafe fn skip_syntaxes (forwardp: bool, string: Lisp_Object, lim: Lisp_Object) -> Lisp_Object
{
  let mut c: c_int;
  let mut fastmap: [u8; 0o400] = [0; 0o400];
  let mut negate: bool = false;
  let mut i: ptrdiff_t;
  let mut i_byte: ptrdiff_t;
  let mut multibyte: bool;
  let mut size_byte: ptrdiff_t;
  let mut str_: *mut u8;

  let mut string: Lisp_Object = string;	// C rebinds the argument below
  let mut lim: Lisp_Object = lim;	// C writes into LIM below

  CHECK_STRING (string);

  if NILP (lim)
    { XSETINT! (lim, if forwardp { ZV! () } else { BEGV! () }); }
  else
    { CHECK_FIXNUM_COERCE_MARKER! (lim); }

  /* In any case, don't allow scan outside bounds of buffer.  */
  if XFIXNUM (lim) > ZV! ()
    { XSETFASTINT! (lim, ZV! ()); }
  if XFIXNUM (lim) < BEGV! ()
    { XSETFASTINT! (lim, BEGV! ()); }

  if if forwardp { PT! () >= XFIXNAT (lim) } else { PT! () <= XFIXNAT (lim) }
    { return make_fixnum (0); }

  multibyte = (! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	       && (XFIXNUM (lim) - PT! () != CHAR_TO_BYTE (XFIXNUM (lim)) - PT_BYTE! ()));

  unsafe { memset (fastmap.as_mut_ptr () as *mut c_void, 0, size_of_val (&fastmap)) };

  if SBYTES (string) > SCHARS (string)
    /* As this is very rare case (syntax spec is ASCII only), don't
       consider efficiency.  */
    { string = unsafe { string_make_unibyte (string) }; }

  str_ = SDATA (string);
  size_byte = SBYTES (string);

  i_byte = 0;
  if i_byte < size_byte
      && SREF (string, 0) == b'^' as u8
    {
      negate = true; i_byte += 1;
    }

  /* Find the syntaxes specified and set their elements of fastmap.  */

  while i_byte < size_byte
    {
      c = unsafe { *str_.add (i_byte as usize) } as c_int;
      i_byte += 1;
      fastmap[syntax_spec_code[c as usize] as usize] = 1;
    }

  /* If ^ was the first character, complement the fastmap.  */
  if negate
    {
      i = 0;
      while i < size_of_val (&fastmap) as ptrdiff_t
	{ fastmap[i as usize] ^= 1; i += 1; }
    }

  {
    let start_point: ptrdiff_t = PT! ();
    let mut pos: ptrdiff_t = PT! ();
    let mut pos_byte: ptrdiff_t = PT_BYTE! ();
    let mut p: *mut u8;
    let mut endp: *mut u8;
    let mut stop: *mut u8;

    SETUP_SYNTAX_TABLE (pos, if forwardp { 1 } else { -1 });

    /* C's two `goto done` jumps (C 2203, C 2209) leave both loops for the tail below,
       which the normal exit also reaches.  */
    'done: {
    if forwardp
      {
	loop
	  {
	    p = BYTE_POS_ADDR (pos_byte);
	    endp = if XFIXNUM (lim) == GPT! () { GPT_ADDR! () } else { CHAR_POS_ADDR (XFIXNUM (lim)) };
	    stop = if pos < GPT! () && GPT! () < XFIXNUM (lim) { GPT_ADDR! () } else { endp };

	    loop
	      {
		let mut nbytes: c_int = 0;

		if p >= stop
		  {
		    if p >= endp
		      { break 'done; }
		    p = GAP_END_ADDR! ();
		    stop = endp;
		  }
		if multibyte
		  { c = string_char_and_length (p, &mut nbytes); }
		else
		  { c = unsafe { *p } as c_int; nbytes = 1; }
		if fastmap[SYNTAX (c) as usize] == 0
		  { break 'done; }
		p = unsafe { p.add (nbytes as usize) };
		pos += 1;
		pos_byte += nbytes as ptrdiff_t;
		rarely_quit (pos as u64);
		/* C: `do { ... } while (!parse_sexp_lookup_properties
		   || pos < gl_state.e_property);` - a `do`/`while` keeps looping while its
		   condition holds, so the trampoline breaks on the *negation* (PORTING.md P-95;
		   the un-negated form broke out of the fast scan at the first character whenever
		   syntax properties are off, and the unguarded
		   `update_syntax_table_forward (pos, gl_state.object)` that follows then asked for
		   syntax at a charpos the interval cache had already left NULL).  */
		if ! (! unsafe { parse_sexp_lookup_properties }
		      || pos < unsafe { gl_state.e_property })
		  { break; }
	      }

	    unsafe { update_syntax_table_forward (pos, gl_state.object) };
	  }
      }
    else
      {
	p = BYTE_POS_ADDR (pos_byte);
	endp = CHAR_POS_ADDR (XFIXNUM (lim));
	stop = if pos >= GPT! () && GPT! () > XFIXNUM (lim) { GAP_END_ADDR! () } else { endp };

	if multibyte
	  {
	    loop
	      {
		if p <= stop
		  {
		    if p <= endp
		      { break; }
		    p = GPT_ADDR! ();
		    stop = endp;
		  }
		UPDATE_SYNTAX_TABLE_BACKWARD (pos - 1);

		let prev_p: *mut u8 = p;
		loop
		  {
		    p = unsafe { p.sub (1) };
		    if ! (stop <= p && ! CHAR_HEAD_P (unsafe { *p } as c_int)) { break; }
		  }

		c = STRING_CHAR (p);
		if fastmap[SYNTAX (c) as usize] == 0
		  { break; }
		pos -= 1;
		pos_byte -= unsafe { prev_p.offset_from (p) };
		rarely_quit (pos as u64);
	      }
	  }
	else
	  {
	    loop
	      {
		if p <= stop
		  {
		    if p <= endp
		      { break; }
		    p = GPT_ADDR! ();
		    stop = endp;
		  }
		UPDATE_SYNTAX_TABLE_BACKWARD (pos - 1);
		if fastmap[SYNTAX (unsafe { *p.sub (1) } as c_int) as usize] == 0
		  { break; }
		p = unsafe { p.sub (1) };
		pos -= 1;
		pos_byte -= 1;
		rarely_quit (pos as u64);
	      }
	  }
      }
    }

    // done:
    SET_PT_BOTH (pos, pos_byte);

    return make_fixnum (PT! () - start_point);
  }
}

/* Return true if character C belongs to one of the ISO classes in the
   array.  */

unsafe fn in_classes (c: c_int, nclasses: c_int, classes: *const u8) -> bool
{
  let mut i: c_int = 0;
  while i < nclasses
    {
      if re_iswctype (c, unsafe { *classes.add (i as usize) } as re_wctype_t)
	{ return true; }
      i += 1;
    }
  return false;
}


/* Jump over a comment, assuming we are at the beginning of one.
   FROM is the current position.
   FROM_BYTE is the bytepos corresponding to FROM.
   Do not move past STOP (a charpos).
   The comment over which we have to jump is of style STYLE
     (either SYNTAX_FLAGS_COMMENT_STYLE (foo) or ST_COMMENT_STYLE).
   NESTING should be positive to indicate the nesting at the beginning
     for nested comments and should be zero or negative else.
     ST_COMMENT_STYLE cannot be nested.
   PREV_SYNTAX is the SYNTAX_WITH_FLAGS of the previous character
     (or 0 if the search cannot start in the middle of a two-character).

   If successful, return true and store the charpos of the comment's
   end into *CHARPOS_PTR and the corresponding bytepos into
   *BYTEPOS_PTR.  Else, return false and store the charpos STOP into
   *CHARPOS_PTR, the corresponding bytepos into *BYTEPOS_PTR and the
   current nesting (as defined for state->incomment) in
   *INCOMMENT_PTR.  Should the last character scanned in an incomplete
   comment be a possible first character of a two character construct,
   we store its SYNTAX_WITH_FLAGS into *last_syntax_ptr.  Otherwise,
   we store Smax into *last_syntax_ptr.

   The comment end is the last character of the comment rather than the
   character just after the comment.

   Global syntax data is assumed to initially be valid for FROM and
   remains valid for forward search starting at the returned position. */

unsafe fn forw_comment (from: ptrdiff_t, from_byte: ptrdiff_t, stop: ptrdiff_t,
			nesting: EMACS_INT, style: c_int, prev_syntax: c_int,
			charpos_ptr: *mut ptrdiff_t, bytepos_ptr: *mut ptrdiff_t,
			incomment_ptr: *mut EMACS_INT, last_syntax_ptr: *mut c_int) -> bool
{
  let mut quit_count: u16 = 0;
  let mut c: c_int;
  let mut c1: c_int;
  let mut code: syntaxcode;
  let mut syntax: c_int;
  let mut other_syntax: c_int;
  let mut from = from;
  let mut from_byte = from_byte;
  let mut nesting = nesting;
  let mut prev_syntax = prev_syntax;

  if nesting <= 0 { nesting = -1; }

  /* Enter the loop in the middle so that we find
     a 2-char comment ender if we start in the middle of it.  */
  syntax = prev_syntax;
  code = syntax & 0xff;
  // C 2319's `goto forw_incomment` enters the body below the top; the port carries it as a
  // flag that is true only for the first pass (PORTING.md 2.2's entry-stage variable), so a
  // `continue` inside the top half re-runs the top half exactly as C does.
  let mut first_pass_at_label: bool = syntax != 0 && from < stop;

  loop
    {
      if first_pass_at_label
	{ first_pass_at_label = false; }
      else
	{
	  if from == stop
	    {
	      unsafe { *incomment_ptr = nesting; }
	      unsafe { *charpos_ptr = from; }
	      unsafe { *bytepos_ptr = from_byte; }
              unsafe { *last_syntax_ptr =
		if code == Sescape || code == Scharquote
		 || SYNTAX_FLAGS_COMEND_FIRST (syntax)
		 || (nesting > 0 && SYNTAX_FLAGS_COMSTART_FIRST (syntax))
		{ syntax } else { Smax } };
	      return false;
	    }
	  c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	  prev_syntax = syntax;
	  syntax = SYNTAX_WITH_FLAGS (c);
	  code = syntax & 0xff;
	  if code == Sendcomment
	      && SYNTAX_FLAGS_COMMENT_STYLE (syntax, 0) == style
	      && (if SYNTAX_FLAGS_COMMENT_NESTED (syntax)
		  { if nesting > 0 { nesting -= 1; nesting == 0 } else { false } }
		  else { nesting < 0 })
              && ! (unsafe { comment_end_can_be_escaped }
		    && ((prev_syntax & 0xff) == Sescape
			|| (prev_syntax & 0xff) == Scharquote))
	    /* We have encountered a comment end of the same style
	       as the comment sequence which began this comment
	       section.  */
	    { break; }
	  if code == Scomment_fence
	      && style == ST_COMMENT_STYLE
	    /* We have encountered a comment end of the same style
	       as the comment sequence which began this comment
	       section.  */
	    { break; }
	  if nesting > 0
	      && code == Scomment
	      && SYNTAX_FLAGS_COMMENT_NESTED (syntax)
	      && SYNTAX_FLAGS_COMMENT_STYLE (syntax, 0) == style
	    /* We have encountered a nested comment of the same style
	       as the comment sequence which began this comment section.  */
	    { nesting += 1; }
	  if unsafe { comment_end_can_be_escaped }
              && (code == Sescape || code == Scharquote)
	    {
	      inc_both (&mut from, &mut from_byte);
	      UPDATE_SYNTAX_TABLE_FORWARD (from);
	      if from == stop { continue; } /* Failure */
	      c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	      prev_syntax = syntax;
	      syntax = Smax;
	      code = syntax;
	    }
	  inc_both (&mut from, &mut from_byte);
	  UPDATE_SYNTAX_TABLE_FORWARD (from);
	}

      // forw_incomment:
      if from < stop && SYNTAX_FLAGS_COMEND_FIRST (syntax)
	  && { c1 = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	       other_syntax = SYNTAX_WITH_FLAGS (c1);
	       SYNTAX_FLAGS_COMEND_SECOND (other_syntax) }
	  && SYNTAX_FLAGS_COMMENT_STYLE (syntax, other_syntax) == style
	  && (if SYNTAX_FLAGS_COMMENT_NESTED (syntax)
	      || SYNTAX_FLAGS_COMMENT_NESTED (other_syntax)
	      { nesting > 0 } else { nesting < 0 })
	{
	  syntax = Smax;        /* So that "|#" (lisp) can not return
                                   the syntax of "#" in *last_syntax_ptr. */
          nesting -= 1;
          if nesting <= 0
	    /* We have encountered a comment end of the same style
	       as the comment sequence which began this comment section.  */
	    { break; }
	  else
	    {
	      inc_both (&mut from, &mut from_byte);
	      UPDATE_SYNTAX_TABLE_FORWARD (from);
	    }
	}
      if nesting > 0
	  && from < stop
	  && SYNTAX_FLAGS_COMSTART_FIRST (syntax)
	  && { c1 = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	       other_syntax = SYNTAX_WITH_FLAGS (c1);
	       SYNTAX_FLAGS_COMMENT_STYLE (other_syntax, syntax) == style
	       && SYNTAX_FLAGS_COMSTART_SECOND (other_syntax) }
	  && (SYNTAX_FLAGS_COMMENT_NESTED (syntax)
	      || SYNTAX_FLAGS_COMMENT_NESTED (other_syntax))
	/* We have encountered a nested comment of the same style
	   as the comment sequence which began this comment section.  */
	{
          syntax = Smax; /* So that "#|#" isn't also a comment ender. */
	  inc_both (&mut from, &mut from_byte);
	  UPDATE_SYNTAX_TABLE_FORWARD (from);
	  nesting += 1;
	}

      quit_count = quit_count.wrapping_add (1);
      rarely_quit (quit_count as u64);
    }
  unsafe { *charpos_ptr = from; }
  unsafe { *bytepos_ptr = from_byte; }
  unsafe { *last_syntax_ptr = Smax; } /* Any syntactic power the last byte had is
                              used up. */
  return true;
}

DEFUN! ("forward-comment", Fforward_comment, Sforward_comment, 1, 1, 0,
       doc: "Move forward across up to COUNT comments.  If COUNT is negative, move backward.\nStop scanning if we find something other than a comment or whitespace.\nSet point to where scanning stops.\nIf COUNT comments are found as expected, with nothing except whitespace\nbetween them, return t; otherwise return nil.",
       (count: Lisp_Object),
       {
  let mut from: ptrdiff_t;
  let mut from_byte: ptrdiff_t;
  let stop: ptrdiff_t;
  let mut c: c_int;
  let mut c1: c_int;
  let mut code: syntaxcode;
  let mut comstyle: c_int = 0;	    /* style of comment encountered */
  let mut comnested: bool = false;	    /* whether the comment is nestable or not */
  let mut found: bool;
  let mut count1: EMACS_INT;
  let mut out_charpos: ptrdiff_t = 0;	/* C leaves these uninitialised (0.1-13).  */
  let mut out_bytepos: ptrdiff_t = 0;
  let mut dummy: EMACS_INT = 0;
  let mut dummy2: c_int = 0;
  let mut quit_count: u16 = 0;

  if ! NILP (unsafe { Vforward_comment_function })
    { return calln! (unsafe { Vforward_comment_function }, count); }

  CHECK_FIXNUM (count);
  count1 = XFIXNUM (count);
  stop = if count1 > 0 { ZV! () } else { BEGV! () };

  from = PT! ();
  from_byte = PT_BYTE! ();

  SETUP_SYNTAX_TABLE (from, clip_to_bounds (PTRDIFF_MIN, count1, PTRDIFF_MAX));
  while count1 > 0
    {
      loop
	{
	  let mut comstart_first: bool;
	  let mut syntax: c_int;
	  let mut other_syntax: c_int;

	  if from == stop
	    {
	      SET_PT_BOTH (from, from_byte);
	      return Qnil ();
	    }
	  c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	  syntax = SYNTAX_WITH_FLAGS (c);
	  code = SYNTAX (c);
	  comstart_first = SYNTAX_FLAGS_COMSTART_FIRST (syntax);
	  comnested = SYNTAX_FLAGS_COMMENT_NESTED (syntax);
	  comstyle = SYNTAX_FLAGS_COMMENT_STYLE (syntax, 0);
	  inc_both (&mut from, &mut from_byte);
	  UPDATE_SYNTAX_TABLE_FORWARD (from);
	  if from < stop && comstart_first
	      && { c1 = FETCH_CHAR_AS_MULTIBYTE (from_byte);
		   other_syntax = SYNTAX_WITH_FLAGS (c1);
		   SYNTAX_FLAGS_COMSTART_SECOND (other_syntax) }
	    {
	      /* We have encountered a comment start sequence and we
		 are ignoring all text inside comments.  We must record
		 the comment style this sequence begins so that later,
		 only a comment end of the same style actually ends
		 the comment section.  */
	      code = Scomment;
	      comstyle = SYNTAX_FLAGS_COMMENT_STYLE (other_syntax, syntax);
	      comnested = comnested || SYNTAX_FLAGS_COMMENT_NESTED (other_syntax);
	      inc_both (&mut from, &mut from_byte);
	      UPDATE_SYNTAX_TABLE_FORWARD (from);
	    }
	  quit_count = quit_count.wrapping_add (1);
	  rarely_quit (quit_count as u64);

	  if ! (code == Swhitespace || (code == Sendcomment && c == '\n' as c_int))
	    { break; }
	}

      if code == Scomment_fence
	{ comstyle = ST_COMMENT_STYLE; }
      else if code != Scomment
	{
	  dec_both (&mut from, &mut from_byte);
	  SET_PT_BOTH (from, from_byte);
	  return Qnil ();
	}
      /* We're at the start of a comment.  */
      found = forw_comment (from, from_byte, stop, comnested as EMACS_INT, comstyle, 0,
			    &mut out_charpos, &mut out_bytepos, &mut dummy, &mut dummy2);
      from = out_charpos; from_byte = out_bytepos;
      if ! found
	{
	  SET_PT_BOTH (from, from_byte);
	  return Qnil ();
	}
      inc_both (&mut from, &mut from_byte);
      UPDATE_SYNTAX_TABLE_FORWARD (from);
      /* We have skipped one comment.  */
      count1 -= 1;
    }

  while count1 < 0
    {
      'comment: loop
	{
	  if from <= stop
	    {
	      SET_PT_BOTH (BEGV! (), BEGV_BYTE! ());
	      return Qnil ();
	    }

	  dec_both (&mut from, &mut from_byte);
	  /* char_quoted does UPDATE_SYNTAX_TABLE_BACKWARD (from).  */
	  let quoted: bool = char_quoted (from, from_byte);
	  c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	  let syntax: c_int = SYNTAX_WITH_FLAGS (c);
	  code = SYNTAX (c);
	  comstyle = 0;
	  comnested = SYNTAX_FLAGS_COMMENT_NESTED (syntax);
	  if code == Sendcomment
	    { comstyle = SYNTAX_FLAGS_COMMENT_STYLE (syntax, 0); }
	  if from > stop && SYNTAX_FLAGS_COMEND_SECOND (syntax)
	      && prev_char_comend_first (from, from_byte)
	      && ! char_quoted (from - 1, dec_bytepos (from_byte))
	    {
	      let other_syntax: c_int;
	      /* We must record the comment style encountered so that
		 later, we can match only the proper comment begin
		 sequence of the same style.  */
	      dec_both (&mut from, &mut from_byte);
	      code = Sendcomment;
	      /* Calling char_quoted, above, set up global syntax position
		 at the new value of FROM.  */
	      c1 = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	      other_syntax = SYNTAX_WITH_FLAGS (c1);
	      comstyle = SYNTAX_FLAGS_COMMENT_STYLE (other_syntax, syntax);
	      comnested = comnested || SYNTAX_FLAGS_COMMENT_NESTED (other_syntax);
	    }

	  /* C's `goto leave` (C 2610, C 2628) reaches the same two statements as falling
	     through the last branch, so the port carries it as a flag (PORTING.md 2.2).  */
	  let mut do_leave: bool = false;
	  'dispatch: {
	    if code == Scomment_fence
	      {
		/* Skip until first preceding unquoted comment_fence.  */
		let mut fence_found: bool = false;
		let ini: ptrdiff_t = from;
		let ini_byte: ptrdiff_t = from_byte;

		if from > stop
		  {
		    loop
		      {
			dec_both (&mut from, &mut from_byte);
			UPDATE_SYNTAX_TABLE_BACKWARD (from);
			c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
			if SYNTAX (c) == Scomment_fence
			    && ! char_quoted (from, from_byte)
			  {
			    fence_found = true;
			    break;
			  }
			else if from == stop
			  { break; }
			quit_count = quit_count.wrapping_add (1);
			rarely_quit (quit_count as u64);
		      }
		  }
		if fence_found == false
		  {
		    from = ini;		/* Set point to ini + 1.  */
		    from_byte = ini_byte;
		    do_leave = true;
		  }
 		else
		  /* We have skipped one comment.  */
		  { break 'comment; }
	      }
	    else if code == Sendcomment
	      {
              found = (! quoted || ! unsafe { comment_end_can_be_escaped })
                && back_comment (from, from_byte, stop, comnested, comstyle,
                                 &mut out_charpos, &mut out_bytepos);
		if ! found
		  {
		    if c == '\n' as c_int
		      /* This end-of-line is not an end-of-comment.
			 Treat it like a whitespace.
			 CC-mode (and maybe others) relies on this behavior.  */
		      { }
		    else
		      {
			/* Failure: we should go back to the end of this
			   not-quite-endcomment.  */
			if SYNTAX (c) != code
			  /* It was a two-char Sendcomment.  */
			  { inc_both (&mut from, &mut from_byte); }
			do_leave = true;
		      }
		  }
		else
		  {
		    /* We have skipped one comment.  */
		    from = out_charpos;
		    from_byte = out_bytepos;
		    break 'comment;
		  }
	      }
	    else if code != Swhitespace || quoted
	      {
		do_leave = true;
	      }
	  } /* 'dispatch */

	  if do_leave
	    {
	      // leave:
	      inc_both (&mut from, &mut from_byte);
	      SET_PT_BOTH (from, from_byte);
	      return Qnil ();
	    }

	  quit_count = quit_count.wrapping_add (1);
	  rarely_quit (quit_count as u64);
	}

      count1 += 1;
    }

  SET_PT_BOTH (from, from_byte);
  return Qt ();
});

/* Return syntax code of character C if C is an ASCII character
   or if MULTIBYTE_SYMBOL_P is false.  Otherwise, return Ssymbol.  */

unsafe fn syntax_multibyte (c: c_int, multibyte_symbol_p: bool) -> syntaxcode
{
  return if ASCII_CHAR_P (c as isize) || ! multibyte_symbol_p { SYNTAX (c) } else { Ssymbol };
}


/* If the character at FROM_BYTE is the second part of a 2-character
   comment opener based on PREV_FROM_SYNTAX, update STATE and return
   true.  */

unsafe fn in_2char_comment_start (state: *mut lisp_parse_state, prev_from_syntax: c_int,
				  prev_from: ptrdiff_t, from_byte: ptrdiff_t) -> bool
{
  let mut c1: c_int;
  let mut syntax: c_int;
  if SYNTAX_FLAGS_COMSTART_FIRST (prev_from_syntax)
      && { c1 = FETCH_CHAR_AS_MULTIBYTE (from_byte);
	   syntax = SYNTAX_WITH_FLAGS (c1);
	   SYNTAX_FLAGS_COMSTART_SECOND (syntax) }
    {
      /* Record the comment style we have entered so that only
         the comment-end sequence of the same style actually
         terminates the comment section.  */
      unsafe { (*state).comstyle
	= SYNTAX_FLAGS_COMMENT_STYLE (syntax, prev_from_syntax); }
      let comnested: bool = (SYNTAX_FLAGS_COMMENT_NESTED (prev_from_syntax)
			     | SYNTAX_FLAGS_COMMENT_NESTED (syntax));
      unsafe { (*state).incomment = if comnested { 1 } else { -1 }; }
      unsafe { (*state).comstr_start = prev_from; }
      return true;
    }
  return false;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_syntax_once ()
{
  let mut i: c_int;
  let mut c: c_int;
  let mut temp: Lisp_Object;

  /* This has to be done here, before we call Fmake_char_table.  */
  DEFSYM! (Qsyntax_table, "syntax-table");

  /* Create objects which can be shared among syntax tables.  */
  unsafe { Vsyntax_code_object = make_nil_vector (Smax as ptrdiff_t) };
  i = 0;
  while i < Smax
    {
      unsafe { ASET (Vsyntax_code_object, i as isize, list1 (make_fixnum (i as EMACS_INT))) };
      i += 1;
    }

  /* Now we are ready to set up this property, so we can
     create syntax tables.  */
  unsafe { Fput (Qsyntax_table (), Qchar_table_extra_slots (), make_fixnum (0)) };

  temp = AREF (unsafe { Vsyntax_code_object }, Swhitespace as isize);

  unsafe { Vstandard_syntax_table! () = Fmake_char_table (Qsyntax_table (), temp) };

  /* Control characters should not be whitespace.  */
  temp = AREF (unsafe { Vsyntax_code_object }, Spunct as isize);
  i = 0;
  while i <= ' ' as c_int - 1
    {
      SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), i, temp);
      i += 1;
    }
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), 0o177, temp);

  /* Except that a few really are whitespace.  */
  temp = AREF (unsafe { Vsyntax_code_object }, Swhitespace as isize);
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), ' ' as c_int, temp);
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), '\t' as c_int, temp);
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), '\n' as c_int, temp);
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), 0o15, temp);
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), 0o14, temp);

  temp = AREF (unsafe { Vsyntax_code_object }, Sword as isize);
  i = 'a' as c_int;
  while i <= 'z' as c_int
    {
      SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), i, temp);
      i += 1;
    }
  i = 'A' as c_int;
  while i <= 'Z' as c_int
    {
      SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), i, temp);
      i += 1;
    }
  i = '0' as c_int;
  while i <= '9' as c_int
    {
      SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), i, temp);
      i += 1;
    }

  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), '$' as c_int, temp);
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), '%' as c_int, temp);

  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), '(' as c_int,
			unsafe { Fcons (make_fixnum (Sopen as EMACS_INT),
					make_fixnum (')' as EMACS_INT)) });
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), ')' as c_int,
			unsafe { Fcons (make_fixnum (Sclose as EMACS_INT),
					make_fixnum ('(' as EMACS_INT)) });
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), '[' as c_int,
			unsafe { Fcons (make_fixnum (Sopen as EMACS_INT),
					make_fixnum (']' as EMACS_INT)) });
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), ']' as c_int,
			unsafe { Fcons (make_fixnum (Sclose as EMACS_INT),
					make_fixnum ('[' as EMACS_INT)) });
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), '{' as c_int,
			unsafe { Fcons (make_fixnum (Sopen as EMACS_INT),
					make_fixnum ('}' as EMACS_INT)) });
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), '}' as c_int,
			unsafe { Fcons (make_fixnum (Sclose as EMACS_INT),
					make_fixnum ('{' as EMACS_INT)) });
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), '"' as c_int,
			unsafe { Fcons (make_fixnum (Sstring as EMACS_INT), Qnil ()) });
  SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), '\\' as c_int,
			unsafe { Fcons (make_fixnum (Sescape as EMACS_INT), Qnil ()) });

  temp = AREF (unsafe { Vsyntax_code_object }, Ssymbol as isize);
  i = 0;
  while i < 10
    {
      c = b"_-+*/&|<>="[i as usize] as c_int;
      SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), c, temp);
      i += 1;
    }

  temp = AREF (unsafe { Vsyntax_code_object }, Spunct as isize);
  i = 0;
  while i < 12
    {
      c = b".,;:?!#@~^'`"[i as usize] as c_int;
      SET_RAW_SYNTAX_ENTRY (Vstandard_syntax_table! (), c, temp);
      i += 1;
    }

  /* All multibyte characters have syntax `word' by default.  */
  temp = AREF (unsafe { Vsyntax_code_object }, Sword as isize);
  char_table_set_range (Vstandard_syntax_table! (), 0x80, MAX_CHAR, temp);
}

unsafe fn scan_lists (mut from0: EMACS_INT, mut count: EMACS_INT, mut depth: EMACS_INT,
		      sexpflag: bool) -> Lisp_Object
{
  let mut val: Lisp_Object;
  let stop: ptrdiff_t = if count > 0 { ZV! () } else { BEGV! () };
  let mut c: c_int = 0;
  let mut c1: c_int = 0;
  let mut stringterm: c_int = 0;
  let mut quoted: bool = false;
  let mut mathexit: bool = false;
  let mut code: syntaxcode = 0;
  let mut min_depth: EMACS_INT = depth;  /* Err out if depth gets less than this.  */
  let mut comstyle: c_int = 0;		/* Style of comment encountered.  */
  let mut comnested: bool = false;		/* Whether the comment is nestable or not.  */
  let mut temp_pos: ptrdiff_t = 0;
  let mut last_good: EMACS_INT = from0;
  let mut found: bool = false;
  let mut from_byte: ptrdiff_t;
  let mut out_bytepos: ptrdiff_t = 0;
  let mut out_charpos: ptrdiff_t = 0;
  let mut dummy: EMACS_INT = 0;
  let mut dummy2: c_int = 0;
  let multibyte_symbol_p: bool = sexpflag && unsafe { multibyte_syntax_as_symbol };
  let mut quit_count: u16 = 0;

  if depth > 0 { min_depth = 0; }

  let mut from: ptrdiff_t = clip_to_bounds (BEGV! (), from0, ZV! ());

  from_byte = CHAR_TO_BYTE (from);

  maybe_quit ();

  SETUP_SYNTAX_TABLE (from, clip_to_bounds (PTRDIFF_MIN, count, PTRDIFF_MAX));
  /* C's `lose:` label is the function's tail (the `xsignal3` below); both scan loops
     run inside this labelled block, so `break 'lose` leaves the whole scan
     (PORTING.md 2.2).  `done`/`done2` are the blocks the object-ended jumps leave.  */
  'lose: {
    while count > 0
      {
	'done: {
	    while from < stop
	      {
		let mut comstart_first: bool;
		let mut prefix: bool;
		let mut syntax: c_int;
		let mut other_syntax: c_int = 0;
		quit_count = quit_count.wrapping_add (1);
		rarely_quit (quit_count as u64);
		UPDATE_SYNTAX_TABLE_FORWARD (from);
		c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
		syntax = SYNTAX_WITH_FLAGS (c);
		code = syntax_multibyte (c, multibyte_symbol_p);
		comstart_first = SYNTAX_FLAGS_COMSTART_FIRST (syntax);
		comnested = SYNTAX_FLAGS_COMMENT_NESTED (syntax);
		comstyle = SYNTAX_FLAGS_COMMENT_STYLE (syntax, 0);
		prefix = SYNTAX_FLAGS_PREFIX (syntax);
		if depth == min_depth
		  { last_good = from as EMACS_INT; }
		inc_both (&mut from, &mut from_byte);
		UPDATE_SYNTAX_TABLE_FORWARD (from);
		/* C's `from < stop && comstart_first && (c = …, other_syntax = …,
		   SYNTAX_FLAGS_COMSTART_SECOND (other_syntax)) && parse_sexp_ignore_comments`:
		   the comma expression's two assignments happen exactly when `from < stop
		   && comstart_first` holds, so the port splits the condition there (0.1-12).  */
		let mut comstart_second: bool = false;
		if from < stop && comstart_first
		  {
		    c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
		    other_syntax = SYNTAX_WITH_FLAGS (c);
		    comstart_second = SYNTAX_FLAGS_COMSTART_SECOND (other_syntax);
		  }
		if comstart_second && unsafe { parse_sexp_ignore_comments }
		  {
		    /* We have encountered a comment start sequence and we
		       are ignoring all text inside comments.  We must record
		       the comment style this sequence begins so that later,
		       only a comment end of the same style actually ends
		       the comment section.  */
		    code = Scomment;
		    comstyle = SYNTAX_FLAGS_COMMENT_STYLE (other_syntax, syntax);
		    comnested = comnested | SYNTAX_FLAGS_COMMENT_NESTED (other_syntax);
		    inc_both (&mut from, &mut from_byte);
		    UPDATE_SYNTAX_TABLE_FORWARD (from);
		  }

		if prefix
		  { continue; }

		let mut jumped_to_close1: bool = false;
		match code
		  {
		    Sescape | Scharquote | Sword | Ssymbol =>
		      {
			/* C's Sescape/Scharquote arms fall through into the word
			   handling below (PORTING.md 2.2: a shared tail is one arm).  */
			if code == Sescape || code == Scharquote
			  {
			    if from == stop
			      { break 'lose; }
			    inc_both (&mut from, &mut from_byte);
			    /* Treat following character as a word constituent.  */
			  }
			if depth == 0 && sexpflag
			  {
			    /* This word counts as a sexp; return at end of it.  */
			    while from < stop
			      {
				UPDATE_SYNTAX_TABLE_FORWARD (from);

				c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
				match syntax_multibyte (c, multibyte_symbol_p)
				  {
				    Scharquote | Sescape =>
				      {
					inc_both (&mut from, &mut from_byte);
					if from == stop
					  { break 'lose; }
				      }
				    Sword | Ssymbol | Squote =>
				      { }
				    _ =>
				      { break 'done; }
				  }
				inc_both (&mut from, &mut from_byte);
				quit_count = quit_count.wrapping_add (1);
				rarely_quit (quit_count as u64);
			      }
			    break 'done;
			  }
		      }

		    Scomment_fence | Scomment =>
		      {
			if code == Scomment_fence
			  /* C 2735 falls through from Scomment_fence into Scomment,
			     carrying the style it just set (PORTING.md 2.2).  */
			  { comstyle = ST_COMMENT_STYLE; }
			if unsafe { parse_sexp_ignore_comments }
			  {
			    UPDATE_SYNTAX_TABLE_FORWARD (from);
			    found = forw_comment (from, from_byte, stop,
						  comnested as EMACS_INT, comstyle, 0,
						  &mut out_charpos, &mut out_bytepos, &mut dummy,
						  &mut dummy2);
			    from = out_charpos;
			    from_byte = out_bytepos;
			    if ! found
			      {
				if depth == 0
				  { break 'done; }
				break 'lose;
			      }
			    inc_both (&mut from, &mut from_byte);
			    UPDATE_SYNTAX_TABLE_FORWARD (from);
			  }
		      }

		    Smath =>
		      {
			/* C's Smath arm can jump to the `close1:` label inside the
			   Sclose arm, and otherwise falls through into the Sopen arm;
			   the port runs the shared tails after the match (2.2).  */
			if sexpflag
			  {
			    if from != stop && c == FETCH_CHAR_AS_MULTIBYTE (from_byte)
			      {
				inc_both (&mut from, &mut from_byte);
			      }
			    if mathexit
			      {
				mathexit = false;
				jumped_to_close1 = true;
			      }
			    else
			      { mathexit = true; }
			  }
			if ! jumped_to_close1 && sexpflag
			  {
			    /* case Sopen: */
			    depth += 1;
			    if depth == 0
			      { break 'done; }
			  }
		      }

		    Sopen =>
		      {
			depth += 1;
			if depth == 0
			  { break 'done; }
		      }

		    Sclose =>
		      { jumped_to_close1 = true; }

		    Sstring | Sstring_fence =>
		      {
			temp_pos = dec_bytepos (from_byte);
			stringterm = FETCH_CHAR_AS_MULTIBYTE (temp_pos);
			loop
			  {
			    let mut c_code: syntaxcode;
			    if from >= stop
			      { break 'lose; }
			    UPDATE_SYNTAX_TABLE_FORWARD (from);
			    c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
			    c_code = syntax_multibyte (c, multibyte_symbol_p);
			    if if code == Sstring
			      { c == stringterm && c_code == Sstring }
			      else
			      { c_code == Sstring_fence }
			      { break; }

			    if c_code == Scharquote || c_code == Sescape
			      { inc_both (&mut from, &mut from_byte); }
			    inc_both (&mut from, &mut from_byte);
			    quit_count = quit_count.wrapping_add (1);
			    rarely_quit (quit_count as u64);
			  }
			inc_both (&mut from, &mut from_byte);
			if depth == 0 && sexpflag
			  { break 'done; }
		      }
		    _ =>
		      {
			/* Ignore whitespace, punctuation, quote, endcomment.  */
		      }
		  }

		if jumped_to_close1
		  {
		    /* C's `close1:` label: the body of `case Sclose`, which Smath's
		       `goto close1` also enters (PORTING.md 2.2).  */
		    depth -= 1;
		    if depth == 0
		      { break 'done; }
		    if depth < min_depth
		      {
			xsignal3 (Qscan_error (),
				  build_string (c"Containing expression ends prematurely".as_ptr ()),
				  make_fixnum (last_good), make_fixnum (from as EMACS_INT));
		      }
		  }
	      }

	    /* Reached end of buffer.  Error if within object, return nil if between */
	    if depth != 0
	      { break 'lose; }

	    return Qnil ();
	  }
	count -= 1;
      }

    while count < 0
      {
	'done2: {
	    while from > stop
	      {
		quit_count = quit_count.wrapping_add (1);
		rarely_quit (quit_count as u64);
		dec_both (&mut from, &mut from_byte);
		UPDATE_SYNTAX_TABLE_BACKWARD (from);
		c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
		let syntax: c_int = SYNTAX_WITH_FLAGS (c);
		code = syntax_multibyte (c, multibyte_symbol_p);
		if depth == min_depth
		  { last_good = from as EMACS_INT; }
		comstyle = 0;
		comnested = SYNTAX_FLAGS_COMMENT_NESTED (syntax);
		if code == Sendcomment
		  { comstyle = SYNTAX_FLAGS_COMMENT_STYLE (syntax, 0); }
		/* C's comma-free `&&` chain: `prev_char_comend_first` is called exactly
		   when the two flags before it hold (0.1-12).  */
		let mut comend_second_first: bool = false;
		if from > stop && SYNTAX_FLAGS_COMEND_SECOND (syntax)
		  {
		    comend_second_first = prev_char_comend_first (from, from_byte);
		  }
		if comend_second_first && unsafe { parse_sexp_ignore_comments }
		  {
		    /* We must record the comment style encountered so that
		       later, we can match only the proper comment begin
		       sequence of the same style.  */
		    let mut c2: c_int;
		    let mut other_syntax: c_int;
		    dec_both (&mut from, &mut from_byte);
		    UPDATE_SYNTAX_TABLE_BACKWARD (from);
		    code = Sendcomment;
		    c2 = FETCH_CHAR_AS_MULTIBYTE (from_byte);
		    other_syntax = SYNTAX_WITH_FLAGS (c2);
		    comstyle = SYNTAX_FLAGS_COMMENT_STYLE (other_syntax, syntax);
		    comnested = comnested | SYNTAX_FLAGS_COMMENT_NESTED (other_syntax);
		  }

		/* Quoting turns anything except a comment-ender
		   into a word character.  Note that this cannot be true
		   if we decremented FROM in the if-statement above.  */
		if code != Sendcomment && char_quoted (from, from_byte)
		  {
		    dec_both (&mut from, &mut from_byte);
		    code = Sword;
		  }
		else if SYNTAX_FLAGS_PREFIX (syntax)
		  { continue; }

		let mut jumped_to_open2: bool = false;
		match code
		  {
		    Sword | Ssymbol | Sescape | Scharquote =>
		      {
			if depth == 0 && sexpflag
			  {
			    /* This word counts as a sexp; count object finished
			       after passing it.  */
			    while from > stop
			      {
				temp_pos = dec_bytepos (from_byte);
				UPDATE_SYNTAX_TABLE_BACKWARD (from - 1);
				c1 = FETCH_CHAR_AS_MULTIBYTE (temp_pos);
				/* Don't allow comment-end to be quoted.  */
				if syntax_multibyte (c1, multibyte_symbol_p) == Sendcomment
				  { break 'done2; }
				quoted = char_quoted (from - 1, temp_pos);
				if quoted
				  {
				    dec_both (&mut from, &mut from_byte);
				    temp_pos = dec_bytepos (temp_pos);
				    UPDATE_SYNTAX_TABLE_BACKWARD (from - 1);
				  }
				c1 = FETCH_CHAR_AS_MULTIBYTE (temp_pos);
				if ! quoted
				  {
				    match syntax_multibyte (c1, multibyte_symbol_p)
				      {
					Sword | Ssymbol | Squote =>
					  { }
					_ =>
					  { break 'done2; }
				      }
				  }
				dec_both (&mut from, &mut from_byte);
				quit_count = quit_count.wrapping_add (1);
				rarely_quit (quit_count as u64);
			      }
			    break 'done2;
			  }
		      }

		    Smath | Sclose =>
		      {
			/* C's Smath arm either jumps to the `open2:` label in the Sopen
			   arm or falls through into Sclose (PORTING.md 2.2).  */
			let mut fall_to_sclose: bool = true;
			if code == Smath
			  {
			    if sexpflag
			      {
				if from > BEGV! ()
				  {
				    temp_pos = dec_bytepos (from_byte);
				    UPDATE_SYNTAX_TABLE_BACKWARD (from - 1);
				    if from != stop && c == FETCH_CHAR_AS_MULTIBYTE (temp_pos)
				      { dec_both (&mut from, &mut from_byte); }
				  }
				if mathexit
				  {
				    mathexit = false;
				    jumped_to_open2 = true;
				    fall_to_sclose = false;
				  }
				else
				  { mathexit = true; }
			      }
			    else
			      { fall_to_sclose = false; }
			  }
			if fall_to_sclose
			  {
			    /* case Sclose: */
			    depth += 1;
			    if depth == 0
			      { break 'done2; }
			  }
		      }

		    Sopen =>
		      { jumped_to_open2 = true; }

		    Sendcomment =>
		      {
			if unsafe { parse_sexp_ignore_comments }
			  {
			    found = back_comment (from, from_byte, stop, comnested, comstyle,
						  &mut out_charpos, &mut out_bytepos);
			    /* FIXME:  if !found, it really wasn't a comment-end.
			       For single-char Sendcomment, we can't do much about it apart
			       from skipping the char.
			       For 2-char endcomments, we could try again, taking both
			       chars as separate entities, but it's a lot of trouble
			       for very little gain, so we don't bother either.  -sm */
			    if found
			      {
				from = out_charpos;
				from_byte = out_bytepos;
			      }
			  }
		      }

		    Scomment_fence | Sstring_fence =>
		      {
			loop
			  {
			    if from == stop
			      { break 'lose; }
			    dec_both (&mut from, &mut from_byte);
			    UPDATE_SYNTAX_TABLE_BACKWARD (from);
			    if ! char_quoted (from, from_byte)
			      {
				c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
				if syntax_multibyte (c, multibyte_symbol_p) == code
				  { break; }
			      }
			    quit_count = quit_count.wrapping_add (1);
			    rarely_quit (quit_count as u64);
			  }
			if code == Sstring_fence && depth == 0 && sexpflag
			  { break 'done2; }
		      }

		    Sstring =>
		      {
			stringterm = FETCH_CHAR_AS_MULTIBYTE (from_byte);
			loop
			  {
			    if from == stop
			      { break 'lose; }
			    dec_both (&mut from, &mut from_byte);
			    UPDATE_SYNTAX_TABLE_BACKWARD (from);
			    if ! char_quoted (from, from_byte)
			      {
				c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
				if c == stringterm
				  && syntax_multibyte (c, multibyte_symbol_p) == Sstring
				  { break; }
			      }
			    quit_count = quit_count.wrapping_add (1);
			    rarely_quit (quit_count as u64);
			  }
			if depth == 0 && sexpflag
			  { break 'done2; }
		      }
		    _ =>
		      {
			/* Ignore whitespace, punctuation, quote, endcomment.  */
		      }
		  }

		if jumped_to_open2
		  {
		    /* C's `open2:` label: the body of `case Sopen`, which Smath's
		       `goto open2` also enters (PORTING.md 2.2).  */
		    depth -= 1;
		    if depth == 0
		      { break 'done2; }
		    if depth < min_depth
		      {
			xsignal3 (Qscan_error (),
				  build_string (c"Containing expression ends prematurely".as_ptr ()),
				  make_fixnum (last_good), make_fixnum (from as EMACS_INT));
		      }
		  }
	      }

	    /* Reached start of buffer.  Error if within object, return nil if between */
	    if depth != 0
	      { break 'lose; }

	    return Qnil ();
	  }
	count += 1;
      }

    XSETFASTINT! (val, from);
    return val;
  }

  xsignal3 (Qscan_error (),
	    build_string (c"Unbalanced parentheses".as_ptr ()),
	    make_fixnum (last_good), make_fixnum (from as EMACS_INT));
}

/* Parse forward from FROM / FROM_BYTE to END,
   assuming that FROM has state STATE,
   and return a description of the state of the parse at END.
   If STOPBEFORE, stop at the start of an atom.
   If COMMENTSTOP is 1, stop at the start of a comment.
   If COMMENTSTOP is -1, stop at the start or end of a comment,
   after the beginning of a string, or after the end of a string.  */

unsafe fn scan_sexps_forward (state: *mut lisp_parse_state,
			      mut from: ptrdiff_t, mut from_byte: ptrdiff_t, end: ptrdiff_t,
			      targetdepth: EMACS_INT, stopbefore: bool,
			      commentstop: c_int)
{
  let mut code: syntaxcode = 0;
  /* C's function-local `struct level` (PORTING.md 1.3: same fields, same order).  */
  #[derive(Copy, Clone)]
  #[repr(C)]
  struct level { last: ptrdiff_t, prev: ptrdiff_t }
  let mut levelstart: [level; 100] = [level { last: 0, prev: 0 }; 100];
  let mut curlevel: *mut level = levelstart.as_mut_ptr ();
  let endlevel: *mut level = levelstart.as_mut_ptr ().add (100);
  let mut depth: EMACS_INT = 0;      /* Paren depth of current scanning location.
					level - levelstart equals this except
					when the depth becomes negative.  */
  let mut mindepth: EMACS_INT = 0;		/* Lowest DEPTH value seen.  */
  let mut start_quoted: bool = false;	/* True means starting after a char quote.  */
  let mut tem: Lisp_Object;
  let mut prev_from: ptrdiff_t = 0;		/* Keep one character before FROM.  */
  let mut prev_from_byte: ptrdiff_t = 0;
  let mut prev_from_syntax: c_int = 0;
  let mut prev_prev_from_syntax: c_int = 0;
  let boundary_stop: bool = commentstop == -1;
  let mut nofence: bool = false;
  let mut found: bool = false;
  let mut out_bytepos: ptrdiff_t = 0;
  let mut out_charpos: ptrdiff_t = 0;
  let mut temp: c_int = 0;
  let mut quit_count: u16 = 0;
  let started_from: ptrdiff_t = from;

  prev_from = from;
  prev_from_byte = from_byte;
  if from != BEGV! ()
    { dec_both (&mut prev_from, &mut prev_from_byte); }

  /* Use this macro instead of `from++'.  INC_FROM reads and writes this function's
     locals, so the port defines it inside the body (the scroll.rs CHECK_BOUNDS
     precedent) and keeps C's statements in C's order; `do { … } while (0)` is the
     single block below.  */
  macro_rules! INC_FROM {
    () => {
      {
	prev_from = from;
	prev_from_byte = from_byte;
	temp = FETCH_CHAR_AS_MULTIBYTE (prev_from_byte);
	prev_prev_from_syntax = prev_from_syntax;
	prev_from_syntax = SYNTAX_WITH_FLAGS (temp);
	inc_both (&mut from, &mut from_byte);
	if from < end
	  { UPDATE_SYNTAX_TABLE_FORWARD (from); }
      }
    };
  }

  maybe_quit ();

  depth = unsafe { (*state).depth };
  start_quoted = unsafe { (*state).quoted };
  prev_prev_from_syntax = Smax;
  prev_from_syntax = unsafe { (*state).prev_syntax };

  tem = unsafe { (*state).levelstarts };
  while ! NILP (tem)		/* >= second enclosing sexps.  */
    {
      let temhd: Lisp_Object = unsafe { Fcar (tem) };
      if RANGED_FIXNUMP (PTRDIFF_MIN, temhd, PTRDIFF_MAX)
	{ unsafe { (*curlevel).last = XFIXNUM (temhd) }; }
      curlevel = curlevel.add (1);
      if curlevel == endlevel
	{ curlevel = curlevel.sub (1); } /* error ("Nesting too deep for parser"); */
      unsafe { (*curlevel).prev = -1; }
      unsafe { (*curlevel).last = -1; }
      tem = unsafe { Fcdr (tem) };
    }
  unsafe { (*curlevel).prev = -1; }
  unsafe { (*curlevel).last = -1; }

  unsafe { (*state).quoted = false; }
  mindepth = depth;

  SETUP_SYNTAX_TABLE (from, 1);

  /* Enter the loop at a place appropriate for initial state.  C's `goto`s into the
     loop body (`startincomment`, `atcomment`, `startquoted`, `symstarted`,
     `startinstring`, `startquotedinstring`) become this entry stage, so every C label
     keeps its own arm and no code is duplicated (PORTING.md 2.2: the forw_comment
     "entry-stage variable" precedent).  `exit_kind` records which of C's `stop:` and
     `endquoted:` labels the walk took on its way to the shared `done:` tail.  */
  const STAGE_LOOP: c_int = 0;
  const STAGE_ATCOMMENT: c_int = 1;
  const STAGE_STARTINCOMMENT: c_int = 2;
  const STAGE_STARTQUOTED: c_int = 3;
  const STAGE_SYMSTARTED: c_int = 4;
  const STAGE_STARTINSTRING: c_int = 5;
  const STAGE_STARTQUOTEDINSTRING: c_int = 6;
  const EXIT_DONE: c_int = 0;
  const EXIT_STOP: c_int = 1;
  const EXIT_ENDQUOTED: c_int = 2;
  let mut stage: c_int = STAGE_LOOP;
  let mut exit_kind: c_int = EXIT_DONE;

  /* dispextern.h 3721: the redisplay tick budget this function charges.  Its mirror
     does not exist yet, so it is declared at its use site (the doc.rs shape).  */
  unsafe extern "C" {
    /// C: void update_redisplay_ticks (int, struct window *)
    pub fn update_redisplay_ticks (ticks: c_int, row: *mut c_void);
  }

  if unsafe { (*state).incomment } != 0
    { stage = STAGE_STARTINCOMMENT; }
  else if unsafe { (*state).instring } >= 0
    {
      nofence = unsafe { (*state).instring } != ST_STRING_STYLE;
      if start_quoted
	{ stage = STAGE_STARTQUOTEDINSTRING; }
      else
	{ stage = STAGE_STARTINSTRING; }
    }
  else if start_quoted
    { stage = STAGE_STARTQUOTED; }
  else if from < end
	   && in_2char_comment_start (state, prev_from_syntax, prev_from, from_byte)
    {
      INC_FROM! ();
      prev_from_syntax = Smax; /* the syntax has already been "used up". */
      stage = STAGE_ATCOMMENT;
    }

  'scan: loop
    {
      let entry: c_int = stage;
      stage = STAGE_LOOP;

      if entry == STAGE_LOOP
	{
	  if ! (from < end)
	    { break 'scan; }
	  quit_count = quit_count.wrapping_add (1);
	  rarely_quit (quit_count as u64);
	  INC_FROM! ();

	  if from < end
	     && in_2char_comment_start (state, prev_from_syntax, prev_from, from_byte)
	    {
	      INC_FROM! ();
	      prev_from_syntax = Smax; /* the syntax has already been "used up". */
	      stage = STAGE_ATCOMMENT;
	      continue;
	    }

	  if SYNTAX_FLAGS_PREFIX (prev_from_syntax)
	    { continue; }
	  code = (prev_from_syntax & 0xff) as syntaxcode;
	}

      match entry
	{
	  STAGE_ATCOMMENT | STAGE_STARTINCOMMENT =>
	    {
	      if entry == STAGE_ATCOMMENT
		{
		  if commentstop != 0 || boundary_stop
		    {
		      exit_kind = EXIT_DONE;
		      break 'scan;
		    }
		}
	      /* startincomment: */
	      /* The (from == BEGV) test was to enter the loop in the middle so
		 that we find a 2-char comment ender even if we start in the
		 middle of it.  We don't want to do that if we're just at the
		 beginning of the comment (think of (*) ... (*)).  */
	      found = forw_comment (from, from_byte, end,
				    unsafe { (*state).incomment }, unsafe { (*state).comstyle },
				    if from == BEGV! () { 0 } else { prev_from_syntax },
				    &raw mut out_charpos, &raw mut out_bytepos,
				    &raw mut (*state).incomment, &raw mut prev_from_syntax);
	      from = out_charpos;
	      from_byte = out_bytepos;
	      /* Beware!  prev_from and friends (except prev_from_syntax)
		 are invalid now.  Luckily, the `done' doesn't use them
		 and the INC_FROM sets them to a sane value without
		 looking at them. */
	      if ! found
		{
		  if (prev_from_syntax & 0xff) == Sescape
		     || (prev_from_syntax & 0xff) == Scharquote
		    { exit_kind = EXIT_ENDQUOTED; }
		  else
		    { exit_kind = EXIT_DONE; }
		  break 'scan;
		}
	      INC_FROM! ();
	      unsafe { (*state).incomment = 0; }
	      unsafe { (*state).comstyle = 0; }	/* reset the comment style */
	      prev_from_syntax = Smax; /* For the comment closer */
	      if boundary_stop
		{
		  exit_kind = EXIT_DONE;
		  break 'scan;
		}
	    }

	  STAGE_STARTQUOTED =>
	    {
	      /* C's `startquoted:` label, reached only from the prologue.  */
	      if from == end
		{
		  exit_kind = EXIT_ENDQUOTED;
		  break 'scan;
		}
	      INC_FROM! ();
	      stage = STAGE_SYMSTARTED;	/* C's `goto symstarted` */
	      continue 'scan;
	    }

	  STAGE_SYMSTARTED =>
	    {
	      /* C's `symstarted:` label; `symdone:` is the tail after the loop.  */
	      'symloop: while from < end
		{
		  if in_2char_comment_start (state, prev_from_syntax, prev_from, from_byte)
		    {
		      INC_FROM! ();
		      prev_from_syntax = Smax; /* the syntax has already been "used up". */
		      stage = STAGE_ATCOMMENT;
		      continue 'scan;
		    }

		  let symchar: c_int = FETCH_CHAR_AS_MULTIBYTE (from_byte);
		  match SYNTAX (symchar)
		    {
		      Scharquote | Sescape =>
			{
			  INC_FROM! ();
			  if from == end
			    {
			      exit_kind = EXIT_ENDQUOTED;
			      break 'scan;
			    }
			}
		      Sword | Ssymbol | Squote =>
			{ }
		      _ =>
			{ break 'symloop; }
		    }
		  INC_FROM! ();
		  quit_count = quit_count.wrapping_add (1);
		  rarely_quit (quit_count as u64);
		}
	      /* symdone: */
	      unsafe { (*curlevel).prev = (*curlevel).last; }
	    }

	  STAGE_STARTINSTRING | STAGE_STARTQUOTEDINSTRING =>
	    {
	      /* C's `startinstring:` label; `startquotedinstring:` is the label the
		 prologue can enter instead.  */
	      let mut at_quotedinstring: bool = entry == STAGE_STARTQUOTEDINSTRING;
	      loop
		{
		  if ! at_quotedinstring
		    {
		      let mut c: c_int;
		      let mut c_code: syntaxcode;

		      if from >= end
			{
			  exit_kind = EXIT_DONE;
			  break 'scan;
			}
		      c = FETCH_CHAR_AS_MULTIBYTE (from_byte);
		      c_code = SYNTAX (c);

		      /* Check C_CODE here so that if the char has
			 a syntax-table property which says it is NOT
			 a string character, it does not end the string.  */
		      if nofence && c == unsafe { (*state).instring } && c_code == Sstring
			{ break; }

		      match c_code
			{
			  Sstring_fence =>
			    {
			      if ! nofence
				{ break; }	/* C's `goto string_end` */
			    }
			  Scharquote | Sescape =>
			    {
			      INC_FROM! ();
			      /* startquotedinstring: */
			      if from >= end
				{
				  exit_kind = EXIT_ENDQUOTED;
				  break 'scan;
				}
			    }
			  _ =>
			    { }
			}
		    }
		  else
		    {
		      at_quotedinstring = false;
		      if from >= end
			{
			  exit_kind = EXIT_ENDQUOTED;
			  break 'scan;
			}
		    }
		  INC_FROM! ();
		  quit_count = quit_count.wrapping_add (1);
		  rarely_quit (quit_count as u64);
		}
	      /* string_end: */
	      unsafe { (*state).instring = -1; }
	      unsafe { (*curlevel).prev = (*curlevel).last; }
	      INC_FROM! ();
	      if boundary_stop
		{
		  exit_kind = EXIT_DONE;
		  break 'scan;
		}
	    }

	  _ =>
	    match code
	      {
		Sescape | Scharquote | Sword | Ssymbol =>
		  {
		    if stopbefore
		      {
			exit_kind = EXIT_STOP;
			break 'scan;
		      }
		    unsafe { (*curlevel).last = prev_from; }
		    if code == Sescape || code == Scharquote
		      {
			/* C's `startquoted:` label: the escape cases fall through
			   into it, and the prologue can enter it directly.  */
			if from == end
			  {
			    exit_kind = EXIT_ENDQUOTED;
			    break 'scan;
			  }
			INC_FROM! ();
		      }
		    stage = STAGE_SYMSTARTED;	/* C's `goto symstarted` */
		    continue 'scan;
		  }

		Scomment_fence =>
		  {
		    /* Record the comment style we have entered so that only
		       the comment-end sequence of the same style actually
		       terminates the comment section.  */
		    unsafe { (*state).comstyle = ST_COMMENT_STYLE; }
		    unsafe { (*state).incomment = -1; }
		    unsafe { (*state).comstr_start = prev_from; }
		    stage = STAGE_ATCOMMENT;	/* C's `goto atcomment` */
		    continue 'scan;
		  }

		Scomment =>
		  {
		    unsafe { (*state).comstyle =
			SYNTAX_FLAGS_COMMENT_STYLE (prev_from_syntax, 0) as c_int; }
		    unsafe { (*state).incomment =
			if SYNTAX_FLAGS_COMMENT_NESTED (prev_from_syntax) { 1 } else { -1 }; }
		    unsafe { (*state).comstr_start = prev_from; }
		    stage = STAGE_ATCOMMENT;	/* C's `goto atcomment` */
		    continue 'scan;
		  }

		Sopen =>
		  {
		    if stopbefore
		      {
			exit_kind = EXIT_STOP;
			break 'scan;
		      }
		    depth += 1;
		    /* curlevel++->last ran into compiler bug on Apollo */
		    unsafe { (*curlevel).last = prev_from; }
		    curlevel = curlevel.add (1);
		    if curlevel == endlevel
		      { curlevel = curlevel.sub (1); } /* error ("Nesting too deep for parser"); */
		    unsafe { (*curlevel).prev = -1; }
		    unsafe { (*curlevel).last = -1; }
		    if targetdepth == depth
		      {
			exit_kind = EXIT_DONE;
			break 'scan;
		      }
		  }

		Sclose =>
		  {
		    depth -= 1;
		    if depth < mindepth
		      { mindepth = depth; }
		    if curlevel != levelstart.as_mut_ptr ()
		      { curlevel = curlevel.sub (1); }
		    unsafe { (*curlevel).prev = (*curlevel).last; }
		    if targetdepth == depth
		      {
			exit_kind = EXIT_DONE;
			break 'scan;
		      }
		  }

		Sstring | Sstring_fence =>
		  {
		    unsafe { (*state).comstr_start = from - 1; }
		    if stopbefore
		      {
			exit_kind = EXIT_STOP;
			break 'scan;
		      }
		    unsafe { (*curlevel).last = prev_from; }
		    unsafe { (*state).instring =
			if code == Sstring
			  { FETCH_CHAR_AS_MULTIBYTE (prev_from_byte) }
			  else
			  { ST_STRING_STYLE }; }
		    if boundary_stop
		      {
			exit_kind = EXIT_DONE;
			break 'scan;
		      }
		    stage = STAGE_STARTINSTRING;
		    continue 'scan;
		  }

		Smath =>
		  {
		    /* FIXME: We should do something with it.  */
		  }

		_ =>
		  {
		    /* Ignore whitespace, punctuation, quote, endcomment.  */
		  }
	      }
	}
    }

  /* C's `stop:` and `endquoted:` labels both fall into `done:` (PORTING.md 2.2:
     one block per label, only one of them runs).  */
  if exit_kind == EXIT_STOP
    {
      from = prev_from;    /* We have just fetched the char that starts it; */
      from_byte = prev_from_byte;
      prev_from_syntax = prev_prev_from_syntax;
      /* but return the position before it. */
    }
  else if exit_kind == EXIT_ENDQUOTED
    { unsafe { (*state).quoted = true; } }

  /* done: */
  unsafe { (*state).depth = depth; }
  unsafe { (*state).mindepth = mindepth; }
  unsafe { (*state).thislevelstart = (*curlevel).prev; }
  unsafe { (*state).prevlevelstart =
    if curlevel == levelstart.as_mut_ptr () { -1 } else { (*curlevel.sub (1)).last }; }
  unsafe { (*state).location = from; }
  unsafe { (*state).location_byte = from_byte; }
  unsafe { (*state).levelstarts = Qnil (); }
  while curlevel > levelstart.as_mut_ptr ()
    {
      curlevel = curlevel.sub (1);
      unsafe { (*state).levelstarts = Fcons (make_fixnum ((*curlevel).last as EMACS_INT),
					     (*state).levelstarts); }
    }
  unsafe { (*state).prev_syntax =
    if SYNTAX_FLAGS_COMSTARTEND_FIRST (prev_from_syntax) || (*state).quoted
      { prev_from_syntax } else { Smax }; }

  /* The factor of 10 below is a heuristic that needs to be tuned.  It
     means we consider 10 buffer positions examined by this function
     roughly equivalent to the display engine iterating over a single
     buffer position.  */
  if unsafe { max_redisplay_ticks } > 0 && from > started_from
    { update_redisplay_ticks (((from - started_from) / 10 + 1) as c_int,
			      std::ptr::null_mut ()); }
}

/* Convert a (lisp) parse state to the internal form used in
   scan_sexps_forward.  */
unsafe fn internalize_parse_state (mut external: Lisp_Object, state: *mut lisp_parse_state)
{
  let mut tem: Lisp_Object;

  if NILP (external)
    {
      unsafe { (*state).depth = 0; }
      unsafe { (*state).instring = -1; }
      unsafe { (*state).incomment = 0; }
      unsafe { (*state).quoted = false; }
      unsafe { (*state).comstyle = 0; }	/* comment style a by default.  */
      unsafe { (*state).comstr_start = -1; }	/* no comment/string seen.  */
      unsafe { (*state).levelstarts = Qnil (); }
      unsafe { (*state).prev_syntax = Smax; }
    }
  else
    {
      tem = unsafe { Fcar (external) };
      unsafe { (*state).depth = if FIXNUMP (tem) { XFIXNUM (tem) } else { 0 }; }

      external = unsafe { Fcdr (external) };
      external = unsafe { Fcdr (external) };
      external = unsafe { Fcdr (external) };
      tem = unsafe { Fcar (external) };
      /* Check whether we are inside string_fence-style string: */
      unsafe { (*state).instring =
	if ! NILP (tem)
	  { if CHARACTERP (tem) { XFIXNAT (tem) as c_int } else { ST_STRING_STYLE } }
	  else { -1 }; }

      external = unsafe { Fcdr (external) };
      tem = unsafe { Fcar (external) };
      unsafe { (*state).incomment =
	if ! NILP (tem)
	  { if FIXNUMP (tem) { XFIXNUM (tem) } else { -1 } }
	  else { 0 }; }

      external = unsafe { Fcdr (external) };
      tem = unsafe { Fcar (external) };
      unsafe { (*state).quoted = ! NILP (tem); }

      /* if the eighth element of the list is nil, we are in comment
	 style a.  If it is non-nil, we are in comment style b */
      external = unsafe { Fcdr (external) };
      external = unsafe { Fcdr (external) };
      tem = unsafe { Fcar (external) };
      unsafe { (*state).comstyle =
	if NILP (tem)
	  { 0 }
	  else if RANGED_FIXNUMP (0, tem, ST_COMMENT_STYLE as EMACS_INT)
	  { XFIXNUM (tem) as c_int }
	  else
	  { ST_COMMENT_STYLE }; }

      external = unsafe { Fcdr (external) };
      tem = unsafe { Fcar (external) };
      unsafe { (*state).comstr_start =
	if RANGED_FIXNUMP (PTRDIFF_MIN, tem, PTRDIFF_MAX)
	  { XFIXNUM (tem) } else { -1 }; }
      external = unsafe { Fcdr (external) };
      tem = unsafe { Fcar (external) };
      unsafe { (*state).levelstarts = tem; }

      external = unsafe { Fcdr (external) };
      tem = unsafe { Fcar (external) };
      unsafe { (*state).prev_syntax = if NILP (tem) { Smax } else { XFIXNUM (tem) as c_int }; }
    }
}

DEFUN! ("scan-lists", Fscan_lists, Sscan_lists, 3, 3, 0,
       doc: "Scan from character number FROM by COUNT lists.\nScan forward if COUNT is positive, backward if COUNT is negative.\nReturn the character number of the position thus found.\n\nA \"list\", in this context, refers to a balanced parenthetical\ngrouping, as determined by the syntax table.\n\nIf DEPTH is nonzero, treat that as the nesting depth of the starting\npoint (i.e. the starting point is DEPTH parentheses deep).  This\nfunction scans over parentheses until the depth goes to zero COUNT\ntimes.  Hence, positive DEPTH moves out that number of levels of\nparentheses, while negative DEPTH moves to a deeper level.\n\nComments are ignored if `parse-sexp-ignore-comments' is non-nil.\n\nIf we reach the beginning or end of the accessible part of the buffer\nbefore we have scanned over COUNT lists, return nil if the depth at\nthat point is zero, and signal an error if the depth is nonzero.",
       (from: Lisp_Object, count: Lisp_Object, depth: Lisp_Object),
       {
  CHECK_FIXNUM (from);
  CHECK_FIXNUM (count);
  CHECK_FIXNUM (depth);

  return scan_lists (XFIXNUM (from), XFIXNUM (count), XFIXNUM (depth), false);
});

DEFUN! ("scan-sexps", Fscan_sexps, Sscan_sexps, 2, 2, 0,
       doc: "Scan from character number FROM by COUNT balanced expressions.\nIf COUNT is negative, scan backwards.\nReturns the character number of the position thus found.\n\nComments are ignored if `parse-sexp-ignore-comments' is non-nil.\n\nIf the beginning or end of (the accessible part of) the buffer is reached\nin the middle of a parenthetical grouping, an error is signaled.\nIf the beginning or end is reached between groupings\nbut before count is used up, nil is returned.",
       (from: Lisp_Object, count: Lisp_Object),
       {
  CHECK_FIXNUM (from);
  CHECK_FIXNUM (count);

  return scan_lists (XFIXNUM (from), XFIXNUM (count), 0, true);
});

DEFUN! ("backward-prefix-chars", Fbackward_prefix_chars, Sbackward_prefix_chars,
       0, 0, 0,
       doc: "Move point backward over any number of chars with prefix syntax.\nThis includes chars with expression prefix syntax class (\\=') and those with\nthe prefix syntax flag (p).",
       (),
       {
  let beg: ptrdiff_t = BEGV! ();
  let mut opoint: ptrdiff_t = PT! ();
  let mut opoint_byte: ptrdiff_t = PT_BYTE! ();
  let mut pos: ptrdiff_t = PT! ();
  let mut pos_byte: ptrdiff_t = PT_BYTE! ();
  let mut c: c_int = 0;

  if pos <= beg
    {
      SET_PT_BOTH (opoint, opoint_byte);

      return Qnil ();
    }

  SETUP_SYNTAX_TABLE (pos, -1);

  dec_both (&mut pos, &mut pos_byte);

  loop
    {
      /* C's `(!char_quoted (…) && ((c = FETCH…, SYNTAX (c) == Squote)
	 || syntax_prefix_flag_p (c)))`; the comma expression's assignment runs
	 exactly when the first operand holds (PORTING.md 0.1-12).  */
      let mut quoted_or_prefix: bool = false;
      if ! char_quoted (pos, pos_byte)
	{
	  c = FETCH_CHAR_AS_MULTIBYTE (pos_byte);
	  quoted_or_prefix = SYNTAX (c) == Squote || syntax_prefix_flag_p (c);
	}
      if ! quoted_or_prefix
	{ break; }

      opoint = pos;
      opoint_byte = pos_byte;

      if pos <= beg
	{ break; }
      dec_both (&mut pos, &mut pos_byte);
      rarely_quit (pos as u64);
    }

  SET_PT_BOTH (opoint, opoint_byte);

  return Qnil ();
});

DEFUN! ("parse-partial-sexp", Fparse_partial_sexp, Sparse_partial_sexp, 2, 6, 0,
       doc: "Parse Lisp syntax starting at FROM until TO; return status of parse at TO.\nParsing stops at TO or when certain criteria are met;\n point is set to where parsing stops.\n\nIf OLDSTATE is omitted or nil, parsing assumes that FROM is the\n beginning of a function.  If not, OLDSTATE should be the state at\n FROM.\n\nValue is a list of elements describing final state of parsing:\n 0. depth in parens.\n 1. character address of start of innermost containing list; nil if none.\n 2. character address of start of last complete sexp terminated.\n 3. non-nil if inside a string.\n    (it is the character that will terminate the string,\n     or t if the string should be terminated by a generic string delimiter.)\n 4. nil if outside a comment, t if inside a non-nestable comment,\n    else an integer (the current comment nesting).\n 5. t if following a quote character.\n 6. the minimum paren-depth encountered during this scan.\n 7. style of comment, if any.\n 8. character address of start of comment or string; nil if not in one.\n 9. List of positions of currently open parens, outermost first.\n10. When the last position scanned holds the first character of a\n    (potential) two character construct, the syntax of that position,\n    otherwise nil.  That construct can be a two character comment\n    delimiter or an Escaped or Char-quoted character.\n11..... Possible further internal information used by `parse-partial-sexp'.\n\nIf third arg TARGETDEPTH is non-nil, parsing stops if the depth\nin parentheses becomes equal to TARGETDEPTH.\nFourth arg STOPBEFORE non-nil means stop when we come to\n any character that starts a sexp.\nFifth arg OLDSTATE is a list like what this function returns.\n It is used to initialize the state of the parse.  Elements number 1, 2, 6\n are ignored.\nSixth arg COMMENTSTOP non-nil means stop after the start of a comment.\n If it is the symbol `syntax-table', stop after the start of a comment or a\n string, or after end of a comment or a string.",
       (from: Lisp_Object, to: Lisp_Object, targetdepth: Lisp_Object,
	stopbefore: Lisp_Object, oldstate: Lisp_Object, commentstop: Lisp_Object),
       {
  let mut state: lisp_parse_state = std::mem::zeroed ();
  let mut target: EMACS_INT;
  let mut from: Lisp_Object = from;	// C takes the arguments' addresses below
  let mut to: Lisp_Object = to;

  if ! NILP (targetdepth)
    {
      CHECK_FIXNUM (targetdepth);
      target = XFIXNUM (targetdepth);
    }
  else
    { target = TYPE_MINIMUM! (EMACS_INT); }	/* We won't reach this depth.  */

  if fix_position (to) < fix_position (from)
    { error (c"End position is smaller than start position".as_ptr ()); }

  validate_region (&raw mut from, &raw mut to);
  internalize_parse_state (oldstate, &raw mut state);
  scan_sexps_forward (&raw mut state, XFIXNUM (from), CHAR_TO_BYTE (XFIXNUM (from)),
		      XFIXNUM (to),
		      target, ! NILP (stopbefore),
		      if NILP (commentstop)
		      { 0 }
		      else if EQ (commentstop, Qsyntax_table ())
		      { -1 }
		      else
		      { 1 });

  SET_PT_BOTH (state.location, state.location_byte);

  return
    Fcons (make_fixnum (state.depth),
	   Fcons (if state.prevlevelstart < 0
		  { Qnil () } else { make_fixnum (state.prevlevelstart) },
	     Fcons (if state.thislevelstart < 0
		    { Qnil () } else { make_fixnum (state.thislevelstart) },
	       Fcons (if state.instring >= 0
		      {
			if state.instring == ST_STRING_STYLE
			  { Qt () } else { make_fixnum (state.instring as EMACS_INT) }
		      } else { Qnil () },
		 Fcons (if state.incomment < 0
			{ Qt () }
			else if state.incomment == 0
			{ Qnil () }
			else
			{ make_fixnum (state.incomment) },
		   Fcons (if state.quoted { Qt () } else { Qnil () },
		     Fcons (make_fixnum (state.mindepth),
		       Fcons (if state.comstyle != 0
			      {
				if state.comstyle == ST_COMMENT_STYLE
				  { Qsyntax_table () }
				else
				  { make_fixnum (state.comstyle as EMACS_INT) }
			      }
			      else { Qnil () },
			 Fcons (if state.incomment != 0 || state.instring >= 0
				{ make_fixnum (state.comstr_start) }
				else { Qnil () },
			   Fcons (state.levelstarts,
                             Fcons (if state.prev_syntax == Smax
				    { Qnil () }
				    else { make_fixnum (state.prev_syntax as EMACS_INT) },
                                Qnil ())))))))))));
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_syntax ()
{
  DEFSYM! (Qsyntax_table_p, "syntax-table-p");
  DEFSYM! (Qsyntax_ppss, "syntax-ppss");
  DEFVAR_LISP! (c"comment-use-syntax-ppss", Vcomment_use_syntax_ppss,
    "Non-nil means `forward-comment' can use `syntax-ppss' internally.");
  Vcomment_use_syntax_ppss = Qt ();

  staticpro (&raw const Vsyntax_code_object);

  staticpro (&raw const gl_state.object);
  staticpro (&raw const gl_state.global_code);
  staticpro (&raw const gl_state.current_syntax_table);
  staticpro (&raw const gl_state.old_prop);

  DEFSYM! (Qscan_error, "scan-error");
  Fput (Qscan_error (), Qerror_conditions (), list2 (Qscan_error (), Qerror ()));
  Fput (Qscan_error (), Qerror_message (), build_string (c"Scan error".as_ptr ()));

  DEFVAR_BOOL! (c"parse-sexp-ignore-comments", parse_sexp_ignore_comments,
    "Non-nil means `forward-sexp', etc., should treat comments as whitespace.");

  DEFVAR_BOOL! (c"parse-sexp-lookup-properties", parse_sexp_lookup_properties,
    "Non-nil means `forward-sexp', etc., obey `syntax-table' property.\nOtherwise, that text property is simply ignored.\nSee the info node `(elisp)Syntax Properties' for a description of the\n`syntax-table' property.");

  DEFVAR_INT! (c"syntax-propertize--done", syntax_propertize__done,
    "Position up to which syntax-table properties have been set.");
  syntax_propertize__done = -1;
  DEFSYM! (Qinternal__syntax_propertize, "internal--syntax-propertize");
  Fmake_variable_buffer_local (intern (c"syntax-propertize--done".as_ptr ()));

  words_include_escapes = false;
  DEFVAR_BOOL! (c"words-include-escapes", words_include_escapes,
    "Non-nil means `forward-word', etc., should treat escape chars part of words.");

  DEFVAR_BOOL! (c"multibyte-syntax-as-symbol", multibyte_syntax_as_symbol,
    "Non-nil means `scan-sexps' treats all multibyte characters as symbol.");
  multibyte_syntax_as_symbol = false;

  DEFVAR_BOOL! (c"open-paren-in-column-0-is-defun-start",
		open_paren_in_column_0_is_defun_start,
    "Non-nil means an open paren in column 0 denotes the start of a defun.  ");
  open_paren_in_column_0_is_defun_start = true;


  DEFVAR_LISP! (c"find-word-boundary-function-table",
		Vfind_word_boundary_function_table,
    "Char table of functions to search for the word boundary.\nEach function is called with two arguments; POS and LIMIT.\nPOS and LIMIT are character positions in the current buffer.\n\nIf POS is less than LIMIT, POS is at the first character of a word,\nand the return value of a function should be a position after the\nlast character of that word.\n\nIf POS is not less than LIMIT, POS is at the last character of a word,\nand the return value of a function should be a position at the first\ncharacter of that word.\n\nIn both cases, LIMIT bounds the search.  ");
  Vfind_word_boundary_function_table = Fmake_char_table (Qnil (), Qnil ());

  DEFVAR_BOOL! (c"comment-end-can-be-escaped", comment_end_can_be_escaped,
    "Non-nil means an escaped ender inside a comment doesn't end the comment.  ");
  comment_end_can_be_escaped = false;
  DEFSYM! (Qcomment_end_can_be_escaped, "comment-end-can-be-escaped");
  Fmake_variable_buffer_local (Qcomment_end_can_be_escaped ());

  DEFVAR_LISP! (c"forward-comment-function", Vforward_comment_function,
    "If non-nil, `forward-comment' delegates to this function.\nShould take the same arguments and behave similarly to `forward-comment'.  ");
  Vforward_comment_function = Qnil ();

  defsubr (&raw mut Ssyntax_table_p);
  defsubr (&raw mut Ssyntax_table);
  defsubr (&raw mut Sstandard_syntax_table);
  defsubr (&raw mut Scopy_syntax_table);
  defsubr (&raw mut Sset_syntax_table);
  defsubr (&raw mut Schar_syntax);
  defsubr (&raw mut Ssyntax_class_to_char);
  defsubr (&raw mut Smatching_paren);
  defsubr (&raw mut Sstring_to_syntax);
  defsubr (&raw mut Smodify_syntax_entry);
  defsubr (&raw mut Sinternal_describe_syntax_value);

  defsubr (&raw mut Sforward_word);

  defsubr (&raw mut Sskip_chars_forward);
  defsubr (&raw mut Sskip_chars_backward);
  defsubr (&raw mut Sskip_syntax_forward);
  defsubr (&raw mut Sskip_syntax_backward);

  defsubr (&raw mut Sforward_comment);
  defsubr (&raw mut Sscan_lists);
  defsubr (&raw mut Sscan_sexps);
  defsubr (&raw mut Sbackward_prefix_chars);
  defsubr (&raw mut Sparse_partial_sexp);
}
