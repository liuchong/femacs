// femacs: port of src/syntax.h @ fd7afced403.
//
// WIP mirror (P-66): syntax.h is 215 lines and the single conditional in it is the
// include guard's `#ifndef`, so nothing here is missing because of the frozen
// configuration.  Everything the header declares is below; what is *not* here is the
// code that uses it (`src/syntax.c` is not ported yet), which is why the file carries no
// `extern` definition of its own beyond `gl_state`'s declaration.
//
// The two `#include`s (syntax.h 32-33, `buffer.h` and `lisp.h`) have no counterpart here:
// a `src/` mirror never includes another mirror (PORTING.md 0.1-9), so the unit that
// includes this one includes `buffer.rs` and `lisp.rs` first, in C's order.  The inline
// functions below therefore use `CHAR_TABLE_REF` (lisp.rs), `BVAR!` (buffer.rs) and
// `BEGV_BYTE`/`BYTE_TO_CHAR`/`BUF_BEGV_BYTE` (buffer.rs) from the including unit.

// syntax.h 22: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_SYNTAX_H: bool = true;

// INLINE_HEADER_BEGIN has no Rust counterpart (PORTING.md 0.1).

/* The standard syntax table is stored where it will automatically
   be used in all new buffers.  */
// C: `#define Vstandard_syntax_table BVAR (&buffer_defaults, syntax_table)`.  The call
// site spells the member with the `_` C's BVAR appends to it (PORTING.md P-37, the
// buffer.rs BVAR! row), because `macro_rules!` cannot paste onto an identifier.
macro_rules! Vstandard_syntax_table {
    () => { BVAR! (buffer_defaults.as_mut_ptr (), syntax_table_) };
}

/* A syntax table is a chartable whose elements are cons cells
   (CODE+FLAGS . MATCHING-CHAR).  MATCHING-CHAR can be nil if the char
   is not a kind of parenthesis.

   The low 8 bits of CODE+FLAGS is a code, as follows:  */

// syntax.h 43-63: `enum syntaxcode`; the port spells an integer enum as its C type and
// one constant per member (PORTING.md 1.3, the coding.rs precedent).
pub type syntaxcode = c_int;
pub const Swhitespace: syntaxcode = 0;        /* for a whitespace character */
pub const Spunct: syntaxcode = 1;             /* for random punctuation characters */
pub const Sword: syntaxcode = 2;              /* for a word constituent */
pub const Ssymbol: syntaxcode = 3;            /* symbol constituent but not word constituent */
pub const Sopen: syntaxcode = 4;              /* for a beginning delimiter */
pub const Sclose: syntaxcode = 5;             /* for an ending delimiter */
pub const Squote: syntaxcode = 6;             /* for a prefix character like Lisp ' */
pub const Sstring: syntaxcode = 7;            /* for a string-grouping character like Lisp " */
pub const Smath: syntaxcode = 8;              /* for delimiters like $ in Tex.  */
pub const Sescape: syntaxcode = 9;            /* for a character that begins a C-style escape */
pub const Scharquote: syntaxcode = 10;        /* for a character that quotes the following character */
pub const Scomment: syntaxcode = 11;          /* for a comment-starting character */
pub const Sendcomment: syntaxcode = 12;       /* for a comment-ending character */
pub const Sinherit: syntaxcode = 13;          /* use the standard syntax table for this character */
pub const Scomment_fence: syntaxcode = 14;    /* Starts/ends comment which is delimited on the
                                                 other side by any char with the same syntaxcode.  */
pub const Sstring_fence: syntaxcode = 15;     /* Starts/ends string which is delimited on the
                                                 other side by any char with the same syntaxcode.  */
pub const Smax: syntaxcode = 16;              /* Upper bound on codes that are meaningful.  */


#[repr(C)]
#[derive(Clone, Copy)]
pub struct gl_state_s
{
  pub object: Lisp_Object,		/* The object we are scanning.  */
  pub start: ptrdiff_t,			/* Where to stop.  */
  pub stop: ptrdiff_t,			/* Where to stop.  */
  pub use_global: bool,			/* Whether to use global_code
					   or c_s_t.  */
  pub global_code: Lisp_Object,		/* Syntax code of current char.  */
  pub current_syntax_table: Lisp_Object,	/* Syntax table for current pos.  */
  pub old_prop: Lisp_Object,			/* Syntax-table prop at prev pos.  */
  pub b_property: ptrdiff_t,			/* First index where c_s_t is valid.  */
  pub e_property: ptrdiff_t,			/* First index where c_s_t is
					   not valid.  */
  pub e_property_truncated: bool,		/* true if e_property if was truncated
					   by parse_sexp_propertize_done.  */
  pub forward_i: INTERVAL,			/* Where to start lookup on forward.  */
  pub backward_i: INTERVAL,			/* or backward movement.  The
					   data in c_s_t is valid
					   between these intervals,
					   and possibly at the
					   intervals too, depending
					   on:  */
}

/* syntax.h 90: `extern struct gl_state_s gl_state;`.  */
/// `src/syntax.c` 205 defines this object, so the declaration lives in a module and the
/// defining unit shadows it through the glob import (PORTING.md P-39/P-21).
pub mod syntax_h_own_definitions {
  use super::*;
  unsafe extern "C-unwind" {
    pub safe static gl_state: VmGlobal<gl_state_s>;
    /// C: void update_syntax_table (ptrdiff_t, EMACS_INT, Lisp_Object)
    pub safe fn update_syntax_table (charpos: ptrdiff_t, count: EMACS_INT, init: Lisp_Object);
    /// C: void update_syntax_table_forward (ptrdiff_t, Lisp_Object)
    pub safe fn update_syntax_table_forward (charpos: ptrdiff_t, init: Lisp_Object);
    /// C: bool syntax_prefix_flag_p (c_int)
    pub safe fn syntax_prefix_flag_p (c: c_int) -> bool;
    /// C: ptrdiff_t scan_words (ptrdiff_t, EMACS_INT)
    pub safe fn scan_words (from: ptrdiff_t, count: EMACS_INT) -> ptrdiff_t;
    /// C: void RE_SETUP_SYNTAX_TABLE_FOR_OBJECT (Lisp_Object, ptrdiff_t)
    pub safe fn RE_SETUP_SYNTAX_TABLE_FOR_OBJECT (object: Lisp_Object, pos: ptrdiff_t);
    /// C: `extern unsigned char const syntax_spec_code[0400];` (syntax.h 101).
    pub safe static syntax_spec_code: [u8; 256];
  }

  #[inline(always)]
  pub fn SYNTAX_ENTRY (c: c_int) -> Lisp_Object
  {
    return syntax_property_entry (c, false);
  }

  #[inline(always)]
  pub fn SYNTAX_WITH_FLAGS (c: c_int) -> c_int
  {
    return syntax_property_with_flags (c, false);
  }

  #[inline(always)]
  pub fn SYNTAX (c: c_int) -> syntaxcode
  {
    return syntax_property (c, false);
  }
}
// `src/syntax.c` owns every name in that module - `gl_state`, `syntax_spec_code`,
// `syntax_prefix_flag_p`, `scan_words`, `update_syntax_table`, `update_syntax_table_forward`,
// `RE_SETUP_SYNTAX_TABLE_FOR_OBJECT`, and the three macros it redefines with
// `via_property = 1` - so they live in one module and are pulled in with a glob import: the
// owning unit's own definitions shadow the declarations (PORTING.md P-39/P-21).
pub use self::syntax_h_own_definitions::*;

#[inline(always)]
pub fn gl_state_get () -> gl_state_s {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { *gl_state.as_ptr () }
}

#[inline(always)]
pub fn gl_state_set (val: gl_state_s) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { *gl_state.as_mut_ptr () = val; }
}

#[inline(always)]
pub fn gl_state_object () -> Lisp_Object { gl_state_get ().object }
#[inline(always)]
pub fn gl_state_set_object (v: Lisp_Object) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).object = v; }
}
#[inline(always)]
pub fn gl_state_start () -> ptrdiff_t { gl_state_get ().start }
#[inline(always)]
pub fn gl_state_set_start (v: ptrdiff_t) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).start = v; }
}
#[inline(always)]
pub fn gl_state_stop () -> ptrdiff_t { gl_state_get ().stop }
#[inline(always)]
pub fn gl_state_set_stop (v: ptrdiff_t) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).stop = v; }
}
#[inline(always)]
pub fn gl_state_use_global () -> bool { gl_state_get ().use_global }
#[inline(always)]
pub fn gl_state_set_use_global (v: bool) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).use_global = v; }
}
#[inline(always)]
pub fn gl_state_global_code () -> Lisp_Object { gl_state_get ().global_code }
#[inline(always)]
pub fn gl_state_set_global_code (v: Lisp_Object) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).global_code = v; }
}
#[inline(always)]
pub fn gl_state_current_syntax_table () -> Lisp_Object { gl_state_get ().current_syntax_table }
#[inline(always)]
pub fn gl_state_set_current_syntax_table (v: Lisp_Object) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).current_syntax_table = v; }
}
#[inline(always)]
pub fn gl_state_old_prop () -> Lisp_Object { gl_state_get ().old_prop }
#[inline(always)]
pub fn gl_state_set_old_prop (v: Lisp_Object) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).old_prop = v; }
}
#[inline(always)]
pub fn gl_state_b_property () -> ptrdiff_t { gl_state_get ().b_property }
#[inline(always)]
pub fn gl_state_set_b_property (v: ptrdiff_t) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).b_property = v; }
}
#[inline(always)]
pub fn gl_state_e_property () -> ptrdiff_t { gl_state_get ().e_property }
#[inline(always)]
pub fn gl_state_set_e_property (v: ptrdiff_t) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).e_property = v; }
}
#[inline(always)]
pub fn gl_state_e_property_truncated () -> bool { gl_state_get ().e_property_truncated }
#[inline(always)]
pub fn gl_state_set_e_property_truncated (v: bool) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).e_property_truncated = v; }
}
#[inline(always)]
pub fn gl_state_forward_i () -> INTERVAL { gl_state_get ().forward_i }
#[inline(always)]
pub fn gl_state_set_forward_i (v: INTERVAL) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).forward_i = v; }
}
#[inline(always)]
pub fn gl_state_backward_i () -> INTERVAL { gl_state_get ().backward_i }
#[inline(always)]
pub fn gl_state_set_backward_i (v: INTERVAL) {
  /* SAFETY: `gl_state` is owned by the single Lisp VM thread. */
  unsafe { (*gl_state.as_mut_ptr ()).backward_i = v; }
}

#[inline(always)]
pub fn SYNTAX_FLAGS_COMSTART_FIRST (flags: c_int) -> bool { ((flags >> 16) & 1) != 0 }
#[inline(always)]
pub fn SYNTAX_FLAGS_COMSTART_SECOND (flags: c_int) -> bool { ((flags >> 17) & 1) != 0 }
#[inline(always)]
pub fn SYNTAX_FLAGS_COMEND_FIRST (flags: c_int) -> bool { ((flags >> 18) & 1) != 0 }
#[inline(always)]
pub fn SYNTAX_FLAGS_COMEND_SECOND (flags: c_int) -> bool { ((flags >> 19) & 1) != 0 }
#[inline(always)]
pub fn SYNTAX_FLAGS_PREFIX (flags: c_int) -> bool { ((flags >> 20) & 1) != 0 }
#[inline(always)]
pub fn SYNTAX_FLAGS_COMMENT_STYLEB (flags: c_int) -> bool { ((flags >> 21) & 1) != 0 }
#[inline(always)]
pub fn SYNTAX_FLAGS_COMMENT_NESTED (flags: c_int) -> bool { ((flags >> 22) & 1) != 0 }
#[inline(always)]
pub fn SYNTAX_FLAGS_COMMENT_STYLEC (flags: c_int) -> bool { ((flags >> 23) & 1) != 0 }

#[inline(always)]
pub fn SET_RAW_SYNTAX_ENTRY (table: Lisp_Object, c: c_int, val: Lisp_Object) {
  CHAR_TABLE_SET (table, c as isize, val);
}

#[inline(always)]
pub fn SET_RAW_SYNTAX_ENTRY_RANGE (table: Lisp_Object, range: Lisp_Object, val: Lisp_Object) {
  Fset_char_table_range (table, range, val);
}

#[inline(always)]
pub fn set_current_buffer_syntax_table_local (table: Lisp_Object) {
  let cb = /* SAFETY: reads active `current_buffer` pointer. */ unsafe { current_buffer! () };
  assert! (!cb.is_null ());
  /* SAFETY: `cb` verified non-null live buffer; sets syntax table and marks its per-buffer local bit. */
  unsafe {
    (*cb).syntax_table_ = table;
    let idx = PER_BUFFER_VAR_IDX! (syntax_table_);
    SET_PER_BUFFER_VALUE_P (cb, idx, true);
  }
}

#[inline(always)]
pub fn error_invalid_syntax_letter (ch: c_int) -> ! {
  /* SAFETY: fixed format string with scalar `c_int` argument. */
  unsafe { error (c"Invalid syntax description letter: %c".as_ptr (), ch) }
}

#[inline(always)]
pub fn current_buffer_syntax_table () -> Lisp_Object {
  let cb = /* SAFETY: reads active `current_buffer` pointer. */ unsafe { current_buffer! () };
  assert! (!cb.is_null ());
  /* SAFETY: `cb` verified non-null live buffer on the owning Lisp thread. */
  unsafe { BVAR! (cb, syntax_table_) }
}

#[inline(always)]
pub fn set_current_buffer_syntax_table (table: Lisp_Object) {
  let cb = /* SAFETY: reads active `current_buffer` pointer. */ unsafe { current_buffer! () };
  assert! (!cb.is_null ());
  /* SAFETY: `cb` verified non-null live buffer on the owning Lisp thread. */
  unsafe { (*cb).syntax_table_ = table; }
}

#[inline(always)]
pub fn standard_syntax_table () -> Lisp_Object {
  /* SAFETY: `buffer_defaults` is static VM storage owned by the single Lisp thread. */
  unsafe { BVAR! (buffer_defaults.as_mut_ptr (), syntax_table_) }
}

#[inline(always)]
pub fn set_standard_syntax_table (table: Lisp_Object) {
  /* SAFETY: `buffer_defaults` is static VM storage owned by the single Lisp thread. */
  unsafe { (*buffer_defaults.as_mut_ptr ()).syntax_table_ = table; }
}

/* Fetch the information from the entry for character C
   in the current buffer's syntax table,
   or (if VIA_PROPERTY) from globally kept data (gl_state).
   Does inheritance.  */

#[inline(always)]
pub fn syntax_property_entry (c: c_int, via_property: bool) -> Lisp_Object
{
  if via_property
    { return if gl_state_use_global ()
	{ gl_state_global_code () }
	else { CHAR_TABLE_REF (gl_state_current_syntax_table (), c as isize) }; }
  return CHAR_TABLE_REF (current_buffer_syntax_table (), c as isize);
}
/* Extract the information from the entry for character C
   in the current syntax table.  */

#[inline(always)]
pub fn syntax_property_with_flags (c: c_int, via_property: bool) -> c_int
{
  let ent = syntax_property_entry (c, via_property);
  return if CONSP (ent) { XFIXNUM (XCAR (ent)) as c_int } else { Swhitespace };
}
#[inline(always)]
pub fn syntax_property (c: c_int, via_property: bool) -> syntaxcode
{
  return syntax_property_with_flags (c, via_property) & 0xff;
}
/* This array, indexed by a character less than 256, contains the
   syntax code which that character signifies (as an unsigned char).
   For example, syntax_spec_code['w'] == Sword.  */

/* Convert the regexp's BYTEOFFSET into a character position,
   for the object recorded in gl_state with RE_SETUP_SYNTAX_TABLE_FOR_OBJECT.

   The value is meant for use in code that does nothing when
   parse_sexp_lookup_properties is false, so return 0 in that case,
   for speed.  */

#[inline(always)]
pub fn RE_SYNTAX_TABLE_BYTE_TO_CHAR (byteoffset: ptrdiff_t) -> ptrdiff_t
{
  if !parse_sexp_lookup_properties.get () {
    return 0;
  }
  let obj = gl_state_object ();
  if STRINGP (obj) {
    string_byte_to_char (obj, byteoffset)
  } else if BUFFERP (obj) {
    /* SAFETY: `BUFFERP(obj)` verified above. */
    unsafe {
      buf_bytepos_to_charpos (
        XBUFFER (obj) as *mut c_void,
        byteoffset + BUF_BEGV_BYTE (XBUFFER (obj)),
      )
    }
  } else if NILP (obj) {
    BYTE_TO_CHAR (byteoffset + current_buffer_begv_byte ())
  } else {
    byteoffset
  }
}

/* Make syntax table state (gl_state) good for CHARPOS, assuming it is
   currently good for a position before CHARPOS.  */

#[inline(always)]
pub fn UPDATE_SYNTAX_TABLE_FORWARD (charpos: ptrdiff_t)
{ /* Performs just-in-time syntax-propertization.  */
  if parse_sexp_lookup_properties.get () && charpos >= gl_state_e_property ()
    { update_syntax_table_forward (charpos, gl_state_object ()) }
}

/* Make syntax table state (gl_state) good for CHARPOS, assuming it is
   currently good for a position after CHARPOS.  */

#[inline(always)]
pub fn UPDATE_SYNTAX_TABLE_BACKWARD (charpos: ptrdiff_t)
{
  if parse_sexp_lookup_properties.get () && charpos < gl_state_b_property ()
    { update_syntax_table (charpos, -1, gl_state_object ()) }
}

/* Make syntax table good for CHARPOS.  */

#[inline(always)]
pub fn UPDATE_SYNTAX_TABLE (charpos: ptrdiff_t)
{
  UPDATE_SYNTAX_TABLE_BACKWARD (charpos);
  UPDATE_SYNTAX_TABLE_FORWARD (charpos);
}

/* Set up the buffer-global syntax table.  */

#[inline(always)]
pub fn SETUP_BUFFER_SYNTAX_TABLE ()
{
  gl_state_set_use_global (false);
  gl_state_set_e_property_truncated (false);
  gl_state_set_current_syntax_table (current_buffer_syntax_table ());
}

// INLINE_HEADER_END has no Rust counterpart (PORTING.md 0.1).
