// femacs: port of lib/regcomp.c @ fd7afced403 (GNU Emacs 32.0.50).
//
// One-to-one port (PORTING.md): same statements in the same order, same names.  The file
// is in `docs/baseline/config/cond-eval-verified.txt`, so the gate reads it with the frozen
// configuration: the `_LIBC` arms (regcomp.c 20-27, 356-380, 649-712, 2788-3041, 3066-3412,
// 3680-...) are dead and only their *live* text is ported here (a comment marks each spot).
//
// C's `static` forward declarations (regcomp.c 29-115) have no Rust counterpart (0.1-12),
// so they are dropped.  `weak_alias`/`libc_hidden_def` only affect symbol binding; the port
// keeps the call text as no-op macros.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr::null_mut;

include!("include/regex_internal.rs");

/* C: the two binding helpers, kept as call text (PORTING.md 8: attributes are no-ops).  */
macro_rules! weak_alias {
    ($a:ident, $b:ident) => {};
}
macro_rules! libc_hidden_def {
    ($a:ident) => {};
}

/* C: `__always_inline` is an attribute.  */
macro_rules! __always_inline {
    () => {};
}

/* C: `__glibc_unlikely`/`__glibc_likely` are branch hints (PORTING.md 2.6).  */
#[inline(always)]
fn __glibc_unlikely<T> (cond: T) -> T { cond }
#[inline(always)]
fn __glibc_likely<T> (cond: T) -> T { cond }

unsafe extern "C" {
  /* void abort (void);  */
  pub fn abort () -> !;
  /* <stdlib.h> 1: `MB_CUR_MAX` is `__ctype_get_mb_cur_max ()` in glibc and
     `___mb_cur_max ()` on the frozen target (darwin).  C's macro is the call.  */
  pub fn ___mb_cur_max () -> c_int;
}

/* C: <stdlib.h>'s `MB_CUR_MAX` macro.  */
macro_rules! MB_CUR_MAX {
    () => { ___mb_cur_max () };
}

/* This table gives an error message for each of the error codes listed
   in regex.h.  Obviously the order here has to be same as there.  */
pub const __re_error_msgid: &[u8] =
  b"Success\x00No match\x00Invalid regular expression\x00Invalid collation character\x00Invalid character class name\x00Trailing backslash\x00Invalid back reference\x00Unmatched [, [^, [:, [., or [=\x00Unmatched ( or \x5c(\x00Unmatched \x5c{\x00Invalid content of \x5c{\x5c}\x00Invalid range end\x00Memory exhausted\x00Invalid preceding regular expression\x00Premature end of regular expression\x00Regular expression too big\x00Unmatched ) or \x5c)\x00";

/* C: `#define REG_X_IDX (REG_PREV_IDX + sizeof "message")`.  The port writes the measured
   byte offset next to the same predecessor; `sizeof` cannot be spelled (docs/ports/
   macro-body-notes.tsv has the `REG_*_IDX` row).  */
pub const REG_NOERROR_IDX: usize = 0;
pub const REG_NOMATCH_IDX: usize = REG_NOERROR_IDX + 8;
pub const REG_BADPAT_IDX: usize = REG_NOMATCH_IDX + 9;
pub const REG_ECOLLATE_IDX: usize = REG_BADPAT_IDX + 27;
pub const REG_ECTYPE_IDX: usize = REG_ECOLLATE_IDX + 28;
pub const REG_EESCAPE_IDX: usize = REG_ECTYPE_IDX + 29;
pub const REG_ESUBREG_IDX: usize = REG_EESCAPE_IDX + 19;
pub const REG_EBRACK_IDX: usize = REG_ESUBREG_IDX + 23;
pub const REG_EPAREN_IDX: usize = REG_EBRACK_IDX + 31;
pub const REG_EBRACE_IDX: usize = REG_EPAREN_IDX + 18;
pub const REG_BADBR_IDX: usize = REG_EBRACE_IDX + 13;
pub const REG_ERANGE_IDX: usize = REG_BADBR_IDX + 24;
pub const REG_ESPACE_IDX: usize = REG_ERANGE_IDX + 17;
pub const REG_BADRPT_IDX: usize = REG_ESPACE_IDX + 18;
pub const REG_EEND_IDX: usize = REG_BADRPT_IDX + 36;
pub const REG_ESIZE_IDX: usize = REG_EEND_IDX + 36;
pub const REG_ERPAREN_IDX: usize = REG_ESIZE_IDX + 27;

pub const __re_error_msgid_idx: [usize; 17] =
  [
    REG_NOERROR_IDX,
    REG_NOMATCH_IDX,
    REG_BADPAT_IDX,
    REG_ECOLLATE_IDX,
    REG_ECTYPE_IDX,
    REG_EESCAPE_IDX,
    REG_ESUBREG_IDX,
    REG_EBRACK_IDX,
    REG_EPAREN_IDX,
    REG_EBRACE_IDX,
    REG_BADBR_IDX,
    REG_ERANGE_IDX,
    REG_ESPACE_IDX,
    REG_BADRPT_IDX,
    REG_EEND_IDX,
    REG_ESIZE_IDX,
    REG_ERPAREN_IDX
  ];

/* Entry points for GNU code.  */

/* re_compile_pattern is the GNU regular expression compiler: it
   compiles PATTERN (of length LENGTH) and puts the result in BUFP.
   Returns 0 if the pattern was valid, otherwise an error string.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_compile_pattern (pattern: *const c_char, length: usize,
                                             bufp: *mut re_pattern_buffer) -> *const c_char
{
  let ret: reg_errcode_t;

  /* And GNU code determines whether or not to get register information
     by passing null for the REGS argument to re_match, etc., not by
     setting no_sub, unless RE_NO_SUB is set.  */
  set_no_sub_ (bufp, (re_syntax_options & RE_NO_SUB != 0) as c_uint);

  /* Match anchors at newline.  */
  set_newline_anchor_ (bufp, 1);

  ret = re_compile_internal (bufp, pattern, length, re_syntax_options);

  if ret == REG_NOERROR
    { return null_mut (); }
  return gettext! (__re_error_msgid.as_ptr ().add (__re_error_msgid_idx[ret as usize])
                   as *const c_char);
}
weak_alias! (__re_compile_pattern, re_compile_pattern);

/* Set by 're_set_syntax' to the current regexp syntax to recognize.  */
#[unsafe(no_mangle)]
pub static mut re_syntax_options: reg_syntax_t = 0;

/* Specify the precise syntax of regexps for compilation.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_set_syntax (syntax: reg_syntax_t) -> reg_syntax_t
{
  let ret: reg_syntax_t = re_syntax_options;

  re_syntax_options = syntax;
  return ret;
}
weak_alias! (__re_set_syntax, re_set_syntax);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_compile_fastmap (bufp: *mut re_pattern_buffer) -> c_int
{
  let dfa: *mut re_dfa_t = (*bufp).buffer as *mut re_dfa_t;
  let fastmap: *mut c_char = (*bufp).fastmap;

  memset (fastmap as *mut c_void, 0, std::mem::size_of::<c_char> () * SBC_MAX as usize);
  re_compile_fastmap_iter (bufp, (*dfa).init_state, fastmap);
  if (*dfa).init_state != (*dfa).init_state_word
    { re_compile_fastmap_iter (bufp, (*dfa).init_state_word, fastmap); }
  if (*dfa).init_state != (*dfa).init_state_nl
    { re_compile_fastmap_iter (bufp, (*dfa).init_state_nl, fastmap); }
  if (*dfa).init_state != (*dfa).init_state_begbuf
    { re_compile_fastmap_iter (bufp, (*dfa).init_state_begbuf, fastmap); }
  set_fastmap_accurate_ (bufp, 1);
  return 0;
}
weak_alias! (__re_compile_fastmap, re_compile_fastmap);

#[inline(always)]
unsafe fn re_set_fastmap (fastmap: *mut c_char, ch: u8)
{
  *fastmap.add (ch as usize) = 1;
}

/* Record in FASTMAP the initial byte of the representations of all
   characters that match WC ignoring case, other than WC itself.
   Use MBS as a scratch state.  */

unsafe fn re_set_fastmap_icase (fastmap: *mut c_char, wc: wchar_t, mbs: *mut mbstate_t)
{
  // C: `#if defined _LIBC || defined _REGEX_AVOID_UCHAR_H` -- the frozen configuration
  // takes this arm (`_REGEX_AVOID_UCHAR_H` is 1); the gnulib arm is dead (verified).
  let mut folded: [wchar_t; 1] = [__towlower! (wc as wint_t) as wchar_t];
  let nfolded: c_int = (folded[0] != wc) as c_int;
  let mut i: c_int = 0;
  while i < nfolded
    {
      let mut buf: [c_char; MB_LEN_MAX] = [0; MB_LEN_MAX];
      if __wcrtomb! (buf.as_mut_ptr (), folded[i as usize], mbs) != 1usize.wrapping_neg ()
	{ re_set_fastmap (fastmap, buf[0] as u8); }
      i += 1;
    }
}

/* Helper function for re_compile_fastmap.
   Compile fastmap for the initial_state INIT_STATE.  */

unsafe fn re_compile_fastmap_iter (bufp: *mut regex_t, init_state: *const re_dfastate_t,
                                   fastmap: *mut c_char)
{
  let dfa: *mut re_dfa_t = (*bufp).buffer as *mut re_dfa_t;
  let mut node_cnt: Idx;
  node_cnt = 0;
  while node_cnt < (*init_state).nodes.nelem
    {
      let mut node: Idx = *(*init_state).nodes.elems.add (node_cnt as usize);
      let r#type: c_uint = type_ ((*dfa).nodes.add (node as usize));

      if r#type == CHARACTER as c_uint
	{
	  re_set_fastmap (fastmap, (*(*dfa).nodes.add (node as usize)).opr.c);
	  if (*bufp).syntax & RE_ICASE != 0
	    {
	      let mut buf: [u8; MB_LEN_MAX] = [0; MB_LEN_MAX];
	      let mut p: *mut u8;
	      let mut wc: wchar_t = 0;
	      let mut state: mbstate_t = mbstate_t { count: 0, value: [0; 4] };

	      p = buf.as_mut_ptr ();
	      *p = (*(*dfa).nodes.add (node as usize)).opr.c; p = p.add (1);
	      node += 1;
	      while node < (*dfa).nodes_len as Idx
		     && type_ ((*dfa).nodes.add (node as usize)) == CHARACTER as c_uint
		     && mb_partial_ ((*dfa).nodes.add (node as usize)) != 0
		{
		  *p = (*(*dfa).nodes.add (node as usize)).opr.c; p = p.add (1);
		  node += 1;
		}
	      memset (&mut state as *mut mbstate_t as *mut c_void, 0,
		      std::mem::size_of::<mbstate_t> ());
	      if __mbrtowc! (&mut wc, buf.as_ptr () as *const c_char,
			     p.offset_from (buf.as_ptr ()) as usize, &mut state)
		  == p.offset_from (buf.as_ptr ()) as usize
		{ re_set_fastmap_icase (fastmap, wc, &mut state); }
	    }
	}
      else if r#type == SIMPLE_BRACKET as c_uint
	{
	  let mut i: c_int;
	  let mut ch: c_int;
	  i = 0; ch = 0;
	  while i < BITSET_WORDS as c_int
	    {
	      let mut j: c_int;
	      let w: bitset_word_t = (*(*dfa).nodes.add (node as usize)).opr.sbcset.add (i as usize).read ();
	      j = 0;
	      while j < BITSET_WORD_BITS as c_int
		{
		  if w & ((1 as bitset_word_t) << j) != 0
		    { re_set_fastmap (fastmap, ch as u8); }
		  j += 1; ch += 1;
		}
	      i += 1;
	    }
	}
      else if r#type == COMPLEX_BRACKET as c_uint
	{
	  let cset: *mut re_charset_t = (*(*dfa).nodes.add (node as usize)).opr.mbcset;
	  let mut i: Idx;

	  // C: `#ifdef _LIBC` (regcomp.c 356-372) -- dead in the frozen configuration.

	  /* See if we have to start the match at all multibyte characters,
	     i.e. where we would not find an invalid sequence.  */
	  if (*dfa).mb_cur_max > 1
	      && ((*cset).nchar_classes != 0 || non_match_ (cset) != 0
		  || (*cset).nranges != 0)
	    {
	      let mut c: u8 = 0;
	      loop
		{
		  let mut mbs: mbstate_t = mbstate_t { count: 0, value: [0; 4] };
		  memset (&mut mbs as *mut mbstate_t as *mut c_void, 0, std::mem::size_of::<mbstate_t> ());
		  if __mbrtowc! (null_mut (), &mut c as *mut u8 as *const c_char, 1, &mut mbs)
		      == 2usize.wrapping_neg ()
		    { re_set_fastmap (fastmap, c); }
		  c = c.wrapping_add (1);
		  if c == 0 { break; }
		}
	    }

	  else
	    {
	      /* ... Else catch all bytes which can start the mbchars.  */
	      i = 0;
	      while i < (*cset).nmbchars
		{
		  let mut buf: [c_char; MB_LEN_MAX] = [0; MB_LEN_MAX];
		  let mut state: mbstate_t = mbstate_t { count: 0, value: [0; 4] };
		  memset (&mut state as *mut mbstate_t as *mut c_void, 0, std::mem::size_of::<mbstate_t> ());
		  if __wcrtomb! (buf.as_mut_ptr (), *(*cset).mbchars.add (i as usize), &mut state)
		      != 1usize.wrapping_neg ()
		    { re_set_fastmap (fastmap, buf[0] as u8); }
		  if (*bufp).syntax & RE_ICASE != 0
		    { re_set_fastmap_icase (fastmap, *(*cset).mbchars.add (i as usize), &mut state); }
		  i += 1;
		}
	    }
	}
      else if r#type == OP_PERIOD as c_uint || r#type == OP_UTF8_PERIOD as c_uint
	      || r#type == END_OF_RE as c_uint
	{
	  memset (fastmap as *mut c_void, 1, std::mem::size_of::<c_char> () * SBC_MAX as usize);
	  if r#type == END_OF_RE as c_uint
	    { set_can_be_null_ (bufp, 1); }
	  return;
	}
      node_cnt += 1;
    }
}


/* Size of the names for collating symbol/equivalence_class/character_class.  */
pub const BRACKET_NAME_BUF_SIZE: c_int = 32;

/* C: inside `build_charclass` --
     #define BUILD_CHARCLASS_LOOP(ctype_func) do { … } while (0)
   `macro_rules!` cannot name the caller's locals (`trans`/`i`/`sbcset`), so the port takes
   them as parameters and the call sites pass them (PORTING.md 2.1; the CONSIDER precedent
   in docs/ports/token-aliases.tsv).  */
macro_rules! BUILD_CHARCLASS_LOOP {
    ($ctype_func:expr, $trans:expr, $i:expr, $sbcset:expr) => {{
        if __glibc_unlikely (!($trans).is_null ())
          {
            $i = 0;
            while $i < SBC_MAX
              {
                if $ctype_func ($i) != 0
                  { bitset_set ($sbcset, *($trans).add ($i as usize) as Idx); }
                $i += 1;
              }
          }
        else
          {
            $i = 0;
            while $i < SBC_MAX
              {
                if $ctype_func ($i) != 0
                  { bitset_set ($sbcset, $i as Idx); }
                $i += 1;
              }
          }
    }};
}

/* Entry point for POSIX code.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn regcomp (preg: *mut regex_t, pattern: *const c_char, cflags: c_int)
                                   -> c_int
{
  let mut ret: reg_errcode_t;
  let mut syntax: reg_syntax_t =
      if cflags & REG_EXTENDED != 0 { RE_SYNTAX_POSIX_EXTENDED } else { RE_SYNTAX_POSIX_BASIC };

  (*preg).buffer = null_mut ();
  (*preg).allocated = 0;
  (*preg).used = 0;

  /* Try to allocate space for the fastmap.  */
  (*preg).fastmap = re_malloc! (c_char, SBC_MAX);
  if __glibc_unlikely ((*preg).fastmap.is_null ())
    { return REG_ESPACE as c_int; }

  syntax |= if cflags & REG_ICASE != 0 { RE_ICASE } else { 0 };

  /* If REG_NEWLINE is set, newlines are treated differently.  */
  if cflags & REG_NEWLINE != 0
    { /* REG_NEWLINE implies neither . nor [^...] match newline.  */
      syntax &= !RE_DOT_NEWLINE;
      syntax |= RE_HAT_LISTS_NOT_NEWLINE;
      /* It also changes the matching behavior.  */
      set_newline_anchor_ (preg, 1);
    }
  else
    { set_newline_anchor_ (preg, 0); }
  set_no_sub_ (preg, (cflags & REG_NOSUB != 0) as c_uint);
  (*preg).translate = null_mut ();

  ret = re_compile_internal (preg, pattern, strlen (pattern), syntax);

  /* POSIX doesn't distinguish between an unmatched open-group and an
     unmatched close-group: both are REG_EPAREN.  */
  if ret == REG_ERPAREN
    { ret = REG_EPAREN; }

  /* We have already checked preg->fastmap != NULL.  */
  if __glibc_likely (ret == REG_NOERROR)
    /* Compute the fastmap now, since regexec cannot modify the pattern
       buffer.  This function never fails in this implementation.  */
    { let _ = re_compile_fastmap (preg); }
  else
    {
      /* Some error occurred while compiling the expression.  */
      re_free! ((*preg).fastmap);
      (*preg).fastmap = null_mut ();
    }

  return ret as c_int;
}
libc_hidden_def! (__regcomp);
weak_alias! (__regcomp, regcomp);

/* Returns a message corresponding to an error code, ERRCODE, returned
   from either regcomp or regexec.   We don't use PREG here.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn regerror (errcode: c_int, preg: *const regex_t, errbuf: *mut c_char,
                                   errbuf_size: usize) -> usize
{
  let msg: *const c_char;
  let msg_size: usize;
  let nerrcodes: c_int = countof! (__re_error_msgid_idx) as c_int;

  if __glibc_unlikely (errcode < 0 || errcode >= nerrcodes)
    /* Only error codes returned by the rest of the code should be passed
       to this routine.  If we are given anything else, or if other regex
       code generates an invalid error code, then the program has a bug.
       Dump core so we can fix it.  */
    { abort (); }

  msg = gettext! (__re_error_msgid.as_ptr ().add (__re_error_msgid_idx[errcode as usize])
                  as *const c_char);

  msg_size = strlen (msg) + 1; /* Includes the null.  */

  if __glibc_likely (errbuf_size != 0)
    {
      let mut cpy_size: usize = msg_size;
      if __glibc_unlikely (msg_size > errbuf_size)
	{
	  cpy_size = errbuf_size - 1;
	  *errbuf.add (cpy_size) = 0;
	}
      memcpy (errbuf as *mut c_void, msg as *const c_void, cpy_size);
    }

  return msg_size;
}
weak_alias! (__regerror, regerror);

/* This static array is used for the map to single-byte characters when
   UTF-8 is used.  */
pub const utf8_sb_map: bitset_t =
  /* Set the first 128 bits.  */
  [BITSET_WORD_MAX, BITSET_WORD_MAX, 0, 0];
// C: the `#else` arm (compiler without designated initializers) is dead in the frozen
// configuration (`__GNUC__` is defined and `__STRICT_ANSI__` is not).

unsafe fn free_dfa_content (dfa: *mut re_dfa_t)
{
  let mut i: Idx;
  let mut j: Idx;

  if !(*dfa).nodes.is_null ()
    {
      i = 0;
      while i < (*dfa).nodes_len as Idx
	{
	  free_token ((*dfa).nodes.add (i as usize));
	  i += 1;
	}
    }
  re_free! ((*dfa).nexts);
  i = 0;
  while i < (*dfa).nodes_len as Idx
    {
      if !(*dfa).eclosures.is_null ()
	{ re_node_set_free! ((*dfa).eclosures.add (i as usize)); }
      if !(*dfa).inveclosures.is_null ()
	{ re_node_set_free! ((*dfa).inveclosures.add (i as usize)); }
      if !(*dfa).edests.is_null ()
	{ re_node_set_free! ((*dfa).edests.add (i as usize)); }
      i += 1;
    }
  re_free! ((*dfa).edests);
  re_free! ((*dfa).eclosures);
  re_free! ((*dfa).inveclosures);
  re_free! ((*dfa).nodes);

  if !(*dfa).state_table.is_null ()
    {
      i = 0;
      while i <= (*dfa).state_hash_mask as Idx
	{
	  let entry: *mut re_state_table_entry = (*dfa).state_table.add (i as usize);
	  j = 0;
	  while j < (*entry).num
	    {
	      let state: *mut re_dfastate_t = *(*entry).array.add (j as usize);
	      free_state (state);
	      j += 1;
	    }
	  re_free! ((*entry).array);
	  i += 1;
	}
    }
  re_free! ((*dfa).state_table);
  if (*dfa).sb_char != utf8_sb_map.as_ptr () as re_bitset_ptr_t
    { re_free! ((*dfa).sb_char); }
  re_free! ((*dfa).subexp_map);
  // C: `#ifdef DEBUG re_free (dfa->re_str); #endif` -- dead in the frozen configuration.

  re_free! (dfa);
}

/* Free dynamically allocated space used by PREG.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn regfree (preg: *mut regex_t)
{
  let dfa: *mut re_dfa_t = (*preg).buffer as *mut re_dfa_t;
  if __glibc_likely (!dfa.is_null ())
    {
      lock_fini! ((*dfa).lock);
      free_dfa_content (dfa);
    }
  (*preg).buffer = null_mut ();
  (*preg).allocated = 0;

  re_free! ((*preg).fastmap);
  (*preg).fastmap = null_mut ();

  re_free! ((*preg).translate);
  (*preg).translate = null_mut ();
}
libc_hidden_def! (__regfree);
weak_alias! (__regfree, regfree);

// C: `#if defined _REGEX_RE_COMP || defined _LIBC` (regcomp.c 649-712) -- both are
// undefined in the frozen configuration, so `re_comp`, `re_comp_buf` and
// `libc_freeres_fn` are dead (verified by tools/check-cond-eval.py).

/* Internal entry point.
   Compile the regular expression PATTERN, whose length is LENGTH.
   SYNTAX indicate regular expression's syntax.  */

unsafe fn re_compile_internal (preg: *mut regex_t, pattern: *const c_char, length: usize,
                               syntax: reg_syntax_t) -> reg_errcode_t
{
  let mut err: reg_errcode_t = REG_NOERROR;
  let mut dfa: *mut re_dfa_t;
  let mut regexp: re_string_t = re_string_t {
      raw_mbs: null_mut (), mbs: null_mut (), wcs: null_mut (), offsets: null_mut (),
      cur_state: mbstate_t { count: 0, value: [0; 4] }, raw_mbs_idx: 0, valid_len: 0,
      valid_raw_len: 0, bufs_len: 0, cur_idx: 0, raw_len: 0, len: 0, raw_stop: 0, stop: 0,
      tip_context: 0, trans: null_mut (), word_char: null_mut (), icase: 0, is_utf8: 0,
      map_notascii: 0, mbs_allocated: 0, offsets_needed: 0, newline_anchor: 0,
      word_ops_used: 0, mb_cur_max: 0 };

  /* Initialize the pattern buffer.  */
  set_fastmap_accurate_ (preg, 0);
  (*preg).syntax = syntax;
  set_not_bol_ (preg, 0); set_not_eol_ (preg, 0);
  (*preg).used = 0;
  (*preg).re_nsub = 0;
  set_can_be_null_ (preg, 0);
  set_regs_allocated_ (preg, REGS_UNALLOCATED as c_uint);

  /* Initialize the dfa.  */
  dfa = (*preg).buffer as *mut re_dfa_t;
  if __glibc_unlikely ((*preg).allocated < std::mem::size_of::<re_dfa_t> () as __re_long_size_t)
    {
      /* If zero allocated, but buffer is non-null, try to realloc
	 enough space.  This loses if buffer's address is bogus, but
	 that is the user's responsibility.  If ->buffer is NULL this
	 is a simple allocation.  */
      dfa = re_realloc! ((*preg).buffer, re_dfa_t, 1);
      if dfa.is_null ()
	{ return REG_ESPACE; }
      (*preg).allocated = std::mem::size_of::<re_dfa_t> () as __re_long_size_t;
      (*preg).buffer = dfa as *mut c_void;
    }
  (*preg).used = std::mem::size_of::<re_dfa_t> () as __re_long_size_t;

  err = init_dfa (dfa, length);
  if __glibc_unlikely (err == REG_NOERROR && lock_init! ((*dfa).lock) != 0)
    { err = REG_ESPACE; }
  let mut goto_free_return = false;
  if __glibc_unlikely (err != REG_NOERROR)
    { goto_free_return = true; }
  // C: `#ifdef DEBUG dfa->re_str = re_malloc (char, length + 1); strncpy (…) #endif` --
  // dead in the frozen configuration.

  if !goto_free_return
    {
      err = re_string_construct (&mut regexp, pattern, length as Idx, (*preg).translate,
				 syntax & RE_ICASE != 0, dfa);
      if __glibc_unlikely (err != REG_NOERROR)
	{ goto_free_return = true; }
    }

  if !goto_free_return
    {
      /* Parse the regular expression, and build a structure tree.  */
      (*preg).re_nsub = 0;
      (*dfa).str_tree = parse (&mut regexp, preg, syntax, &mut err);
      if __glibc_unlikely ((*dfa).str_tree.is_null ())
	{ goto_free_return = true; }
    }
  if !goto_free_return
    {
      /* Analyze the tree and create the nfa.  */
      err = analyze (preg);
      if __glibc_unlikely (err != REG_NOERROR)
	{ goto_free_return = true; }
    }
  if !goto_free_return
    {
      /* If possible, do searching in single byte encoding to speed things up.  */
      if is_utf8_ (dfa) != 0 && syntax & RE_ICASE == 0 && (*preg).translate.is_null ()
	{ optimize_utf8 (dfa); }

      /* Then create the initial state of the dfa.  */
      err = create_initial_state (dfa);
    }

  if goto_free_return
    {
      // re_compile_internal_free_return:
      free_workarea_compile (preg);
      re_string_destruct (&mut regexp);
      lock_fini! ((*dfa).lock);
      free_dfa_content (dfa);
      (*preg).buffer = null_mut ();
      (*preg).allocated = 0;
      return err;
    }

  /* Release work areas.  */
  free_workarea_compile (preg);
  re_string_destruct (&mut regexp);

  if __glibc_unlikely (err != REG_NOERROR)
    {
      lock_fini! ((*dfa).lock);
      free_dfa_content (dfa);
      (*preg).buffer = null_mut ();
      (*preg).allocated = 0;
    }

  return err;
}

/* Initialize DFA.  We use the length of the regular expression PAT_LEN
   as the initial length of some arrays.  */

unsafe fn init_dfa (dfa: *mut re_dfa_t, pat_len: usize) -> reg_errcode_t
{
  let mut table_size: __re_size_t;
  let mut codeset_name: *const c_char;
  let max_i18n_object_size: usize = MAX! (std::mem::size_of::<wchar_t> (), std::mem::size_of::<wctype_t> ());
  let max_object_size: usize =
    MAX! (std::mem::size_of::<re_state_table_entry> (),
	 MAX! (std::mem::size_of::<re_token_t> (),
	      MAX! (std::mem::size_of::<re_node_set> (),
		   MAX! (std::mem::size_of::<regmatch_t> (), max_i18n_object_size))));

  memset (dfa as *mut c_void, 0, std::mem::size_of::<re_dfa_t> ());

  /* Force allocation of str_tree_storage the first time.  */
  (*dfa).str_tree_storage_idx = BIN_TREE_STORAGE_SIZE as c_int;

  /* Avoid overflows.  */
  if __glibc_unlikely (MIN! (IDX_MAX, (SIZE_MAX / max_object_size) as Idx) / 2
			<= pat_len as Idx)
    { return REG_ESPACE; }

  (*dfa).nodes_alloc = pat_len + 1;
  (*dfa).nodes = re_malloc! (re_token_t, (*dfa).nodes_alloc);

  /*  table_size = 2 ^ ceil(log pat_len) */
  table_size = 1;
  loop
    {
      if table_size > pat_len as __re_size_t
	{ break; }
      table_size <<= 1;
    }

  (*dfa).state_table = calloc (table_size, std::mem::size_of::<re_state_table_entry> ())
                              as *mut re_state_table_entry;
  (*dfa).state_hash_mask = (table_size - 1) as re_hashval_t;

  (*dfa).mb_cur_max = MB_CUR_MAX! () as c_int;
  // C: `#ifdef _LIBC` (regcomp.c 857-895) -- dead in the frozen configuration.
  codeset_name = nl_langinfo (CODESET);
  if (codeset_name[0] as u8 == b'U' || codeset_name[0] as u8 == b'u')
      && (codeset_name[1] as u8 == b'T' || codeset_name[1] as u8 == b't')
      && (codeset_name[2] as u8 == b'F' || codeset_name[2] as u8 == b'f')
      && strcmp (codeset_name.add ((3 + (codeset_name[3] as u8 == b'-') as usize) as usize),
		 b"8\0" as *const u8 as *const c_char) == 0
    { set_is_utf8_ (dfa, 1); }

  /* We check exhaustively in the loop below if this charset is a
     superset of ASCII.  */
  set_map_notascii_ (dfa, 0);

  if (*dfa).mb_cur_max > 1
    {
      if is_utf8_ (dfa) != 0
	{ (*dfa).sb_char = utf8_sb_map.as_ptr () as re_bitset_ptr_t; }
      else
	{
	  let mut i: c_int;
	  let mut j: c_int;
	  let mut ch: c_int;

	  (*dfa).sb_char = calloc (1, std::mem::size_of::<bitset_t> ()) as re_bitset_ptr_t;
	  if __glibc_unlikely ((*dfa).sb_char.is_null ())
	    { return REG_ESPACE; }

	  /* Set the bits corresponding to single byte chars.  */
	  i = 0; ch = 0;
	  while i < BITSET_WORDS as c_int
	    {
	      j = 0;
	      while j < BITSET_WORD_BITS as c_int
		{
		  let wch: wint_t = __btowc! (ch) as wint_t;
		  if wch != WEOF
		    { *(*dfa).sb_char.add (i as usize) |= (1 as bitset_word_t) << j; }
		  // C: `#ifndef _LIBC` -- live: the non-glibc ASCII superset check.
		  if isascii! (ch) && wch != ch as wint_t
		    { set_map_notascii_ (dfa, 1); }
		  j += 1; ch += 1;
		}
	      i += 1;
	    }
	}
    }

  if __glibc_unlikely ((*dfa).nodes.is_null () || (*dfa).state_table.is_null ())
    { return REG_ESPACE; }
  return REG_NOERROR;
}

/* Initialize WORD_CHAR table, which indicate which character is
   "word".  */

unsafe fn init_word_char (dfa: *mut re_dfa_t)
{
  let mut i: c_int = 0;
  let mut j: c_int;
  let mut ch: c_int = 0;
  set_word_ops_used_ (dfa, 1);
  let mut general_case = false;
  if __glibc_likely (map_notascii_ (dfa) == 0)
    {
      let bits0: bitset_word_t = 0x00000000;
      let bits1: bitset_word_t = 0x03ff0000;
      let bits2: bitset_word_t = 0x87fffffe;
      let bits3: bitset_word_t = 0x07fffffe;
      if BITSET_WORD_BITS == 64
	{
	  /* Pacify gcc -Woverflow on 32-bit platforms.  */
	  *(*dfa).word_char.as_mut_ptr ().add (0) = bits1 << 31 << 1 | bits0;
	  *(*dfa).word_char.as_mut_ptr ().add (1) = bits3 << 31 << 1 | bits2;
	  i = 2;
	}
      else if BITSET_WORD_BITS == 32
	{
	  *(*dfa).word_char.as_mut_ptr ().add (0) = bits0;
	  *(*dfa).word_char.as_mut_ptr ().add (1) = bits1;
	  *(*dfa).word_char.as_mut_ptr ().add (2) = bits2;
	  *(*dfa).word_char.as_mut_ptr ().add (3) = bits3;
	  i = 4;
	}
      else
        { general_case = true; }
      if !general_case
	{
	  ch = 128;

	  if __glibc_likely (is_utf8_ (dfa) != 0)
	    {
	      memset ((*dfa).word_char.as_mut_ptr ().add (i as usize) as *mut c_void, 0,
		      ((SBC_MAX - ch) / 8) as usize);
	      return;
	    }
	}
    }

 // general_case:
  while i < BITSET_WORDS as c_int
    {
      j = 0;
      while j < BITSET_WORD_BITS as c_int
	{
	  if isalnum (ch) != 0 || ch == b'_' as c_int
	    { *(*dfa).word_char.as_mut_ptr ().add (i as usize) |= (1 as bitset_word_t) << j; }
	  j += 1; ch += 1;
	}
      i += 1;
    }
}

/* Free the work area which are only used while compiling.  */

unsafe fn free_workarea_compile (preg: *mut regex_t)
{
  let dfa: *mut re_dfa_t = (*preg).buffer as *mut re_dfa_t;
  let mut storage: *mut bin_tree_storage_t;
  let mut next: *mut bin_tree_storage_t;
  storage = (*dfa).str_tree_storage;
  while !storage.is_null ()
    {
      next = (*storage).next;
      re_free! (storage);
      storage = next;
    }
  (*dfa).str_tree_storage = null_mut ();
  (*dfa).str_tree_storage_idx = BIN_TREE_STORAGE_SIZE as c_int;
  (*dfa).str_tree = null_mut ();
  re_free! ((*dfa).org_indices);
  (*dfa).org_indices = null_mut ();
}

/* Create initial states for all contexts.  */

unsafe fn create_initial_state (dfa: *mut re_dfa_t) -> reg_errcode_t
{
  let first: Idx;
  let mut i: Idx;
  let mut err: reg_errcode_t;
  let mut init_nodes: re_node_set = re_node_set { alloc: 0, nelem: 0, elems: null_mut () };

  /* Initial states have the epsilon closure of the node which is
     the first node of the regular expression.  */
  first = (*(*(*dfa).str_tree).first).node_idx;
  (*dfa).init_node = first;
  err = re_node_set_init_copy (&mut init_nodes, (*dfa).eclosures.add (first as usize));
  if __glibc_unlikely (err != REG_NOERROR)
    { return err; }

  /* The back-references which are in initial states can epsilon transit,
     since in this case all of the subexpressions can be null.  */
  if (*dfa).nbackref > 0
    {
      i = 0;
      while i < init_nodes.nelem
	{
	  let node_idx: Idx = *init_nodes.elems.add (i as usize);
	  let r#type: c_uint = type_ ((*dfa).nodes.add (node_idx as usize));

	  let mut clexp_idx: Idx;
	  if r#type != OP_BACK_REF as c_uint
	    { i += 1; continue; }
	  clexp_idx = 0;
	  while clexp_idx < init_nodes.nelem
	    {
	      let clexp_node: *mut re_token_t =
	          (*dfa).nodes.add (*(init_nodes.elems.add (clexp_idx as usize)) as usize);
	      if type_ (clexp_node) == OP_CLOSE_SUBEXP as c_uint
		  && (*clexp_node).opr.idx == (*(*dfa).nodes.add (node_idx as usize)).opr.idx
		{ break; }
	      clexp_idx += 1;
	    }
	  if clexp_idx == init_nodes.nelem
	    {
	      i += 1; continue;
	    }

	  if r#type == OP_BACK_REF as c_uint
	    {
	      let dest_idx: Idx = *(*((*dfa).edests.add (node_idx as usize))).elems.add (0);
	      if re_node_set_contains (&raw const init_nodes, dest_idx) == 0
		{
		  err = re_node_set_merge (&mut init_nodes,
					   (*dfa).eclosures.add (dest_idx as usize));
		  if err != REG_NOERROR
		    { break; }
		  i = 0;
		}
	    }
	  i += 1;
	}
    }

  /* It must be the first time to invoke acquire_state.  */
  (*dfa).init_state =
    if err == REG_NOERROR
    { re_acquire_state_context (&mut err, dfa, &raw const init_nodes, 0) }
    else { null_mut () };
  if __glibc_unlikely ((*dfa).init_state.is_null ())
    {
      /* Don't check ERR here, as the initial state must not be null.  */
    }
  else if has_constraint_ ((*dfa).init_state) != 0
    {
      (*dfa).init_state_word = re_acquire_state_context (&mut err, dfa, &raw const init_nodes,
						       CONTEXT_WORD as c_uint);
      (*dfa).init_state_nl = re_acquire_state_context (&mut err, dfa, &raw const init_nodes,
						     CONTEXT_NEWLINE as c_uint);
      (*dfa).init_state_begbuf = re_acquire_state_context (&mut err, dfa,
							 &raw const init_nodes,
							 (CONTEXT_NEWLINE
							  | CONTEXT_BEGBUF) as c_uint);
    }
  else
    {
      (*dfa).init_state_word = (*dfa).init_state;
      (*dfa).init_state_nl = (*dfa).init_state;
      (*dfa).init_state_begbuf = (*dfa).init_state;
    }

  re_node_set_free! (&mut init_nodes);
  return err;
}

/* If it is possible to do searching in single byte encoding instead of UTF-8
   to speed things up, set dfa->mb_cur_max to 1, clear is_utf8 and change
   DFA nodes where needed.  */

unsafe fn optimize_utf8 (dfa: *mut re_dfa_t)
{
  let mut node: Idx;
  let mut i: c_int;
  let mut mb_chars: bool = false;
  let mut has_period: bool = false;

  node = 0;
  while node < (*dfa).nodes_len as Idx
    {
      let r#type: c_uint = type_ ((*dfa).nodes.add (node as usize));
      if r#type == CHARACTER as c_uint
	{
	  if (*(*dfa).nodes.add (node as usize)).opr.c as c_int >= ASCII_CHARS
	    { mb_chars = true; }
	}
      else if r#type == ANCHOR as c_uint
	{
	  let ctx: c_uint = (*(*dfa).nodes.add (node as usize)).opr.ctx_type as c_uint;
	  if !(ctx == LINE_FIRST as c_uint || ctx == LINE_LAST as c_uint
	       || ctx == BUF_FIRST as c_uint || ctx == BUF_LAST as c_uint)
	    {
	      /* Word anchors etc. cannot be handled.  It's okay to test
		 opr.ctx_type since constraints (for all DFA nodes) are
		 created by ORing one or more opr.ctx_type values.  */
	      return;
	    }
	}
      else if r#type == OP_PERIOD as c_uint
	{
	  has_period = true;
	}
      else if r#type == OP_BACK_REF as c_uint
	   || r#type == OP_ALT as c_uint
	   || r#type == END_OF_RE as c_uint
	   || r#type == OP_DUP_ASTERISK as c_uint
	   || r#type == OP_OPEN_SUBEXP as c_uint
	   || r#type == OP_CLOSE_SUBEXP as c_uint
	{
	}
      else if r#type == COMPLEX_BRACKET as c_uint
	{
	  return;
	}
      else if r#type == SIMPLE_BRACKET as c_uint
	{
	  /* Just double check.  */
	  let mut rshift: c_int = (if ASCII_CHARS % BITSET_WORD_BITS == 0
				   { 0 } else { BITSET_WORD_BITS - ASCII_CHARS % BITSET_WORD_BITS })
				   as c_int;
	  i = (ASCII_CHARS / BITSET_WORD_BITS) as c_int;
	  while i < BITSET_WORDS as c_int
	    {
	      if (*(*dfa).nodes.add (node as usize)).opr.sbcset.add (i as usize).read () >> rshift != 0
		{ return; }
	      rshift = 0;
	      i += 1;
	    }
	}
      else
	{
	  abort ();
	}
      node += 1;
    }

  if mb_chars || has_period
    {
      node = 0;
      while node < (*dfa).nodes_len as Idx
	{
	  if type_ ((*dfa).nodes.add (node as usize)) == CHARACTER as c_uint
	      && (*(*dfa).nodes.add (node as usize)).opr.c as c_int >= ASCII_CHARS
	    { set_mb_partial_ ((*dfa).nodes.add (node as usize), 0); }
	  else if type_ ((*dfa).nodes.add (node as usize)) == OP_PERIOD as c_uint
	    {
	      let r#type: c_uint = OP_UTF8_PERIOD as c_uint;
	      set_type_ ((*dfa).nodes.add (node as usize), r#type);
	    }
	  node += 1;
	}
    }

  /* The search can be in single byte locale.  */
  (*dfa).mb_cur_max = 1;
  set_is_utf8_ (dfa, 0);
  set_has_mb_node_ (dfa, ((*dfa).nbackref > 0 || has_period) as c_uint);
}

/* Analyze the structure tree, and calculate "first", "next", "edest",
   "eclosure", and "inveclosure".  */

unsafe fn analyze (preg: *mut regex_t) -> reg_errcode_t
{
  let dfa: *mut re_dfa_t = (*preg).buffer as *mut re_dfa_t;
  let mut ret: reg_errcode_t;

  /* Allocate arrays.  */
  (*dfa).nexts = re_malloc! (Idx, (*dfa).nodes_alloc);
  (*dfa).org_indices = re_malloc! (Idx, (*dfa).nodes_alloc);
  (*dfa).edests = re_malloc! (re_node_set, (*dfa).nodes_alloc);
  (*dfa).eclosures = re_malloc! (re_node_set, (*dfa).nodes_alloc);
  if __glibc_unlikely ((*dfa).nexts.is_null () || (*dfa).org_indices.is_null ()
			|| (*dfa).edests.is_null () || (*dfa).eclosures.is_null ())
    { return REG_ESPACE; }

  (*dfa).subexp_map = re_malloc! (Idx, (*preg).re_nsub);
  if !(*dfa).subexp_map.is_null ()
    {
      let mut i: Idx;
      i = 0;
      while i < (*preg).re_nsub as Idx
	{
	  *(*dfa).subexp_map.add (i as usize) = i;
	  i += 1;
	}
      preorder ((*dfa).str_tree, optimize_subexps, dfa);
      i = 0;
      while i < (*preg).re_nsub as Idx
	{
	  if *(*dfa).subexp_map.add (i as usize) != i
	    { break; }
	  i += 1;
	}
      if i == (*preg).re_nsub as Idx
	{
	  re_free! ((*dfa).subexp_map);
	  (*dfa).subexp_map = null_mut ();
	}
    }

  ret = postorder ((*dfa).str_tree, lower_subexps, preg);
  if __glibc_unlikely (ret != REG_NOERROR)
    { return ret; }
  ret = postorder ((*dfa).str_tree, calc_first, dfa);
  if __glibc_unlikely (ret != REG_NOERROR)
    { return ret; }
  preorder ((*dfa).str_tree, calc_next, dfa);
  ret = preorder ((*dfa).str_tree, link_nfa_nodes, dfa);
  if __glibc_unlikely (ret != REG_NOERROR)
    { return ret; }
  ret = calc_eclosure (dfa);
  if __glibc_unlikely (ret != REG_NOERROR)
    { return ret; }

  /* We only need this during the prune_impossible_nodes pass in regexec.c;
     skip it if p_i_n will not run, as calc_inveclosure can be quadratic.  */
  if ((no_sub_ (preg) == 0 && (*preg).re_nsub > 0 && has_plural_match_ (dfa) != 0)
      || (*dfa).nbackref != 0)
    {
      (*dfa).inveclosures = re_malloc! (re_node_set, (*dfa).nodes_len);
      if __glibc_unlikely ((*dfa).inveclosures.is_null ())
	{ return REG_ESPACE; }
      ret = calc_inveclosure (dfa);
    }

  return ret;
}

/* Our parse trees are very unbalanced, so we cannot use a stack to
   implement parse tree visits.  Instead, we use parent pointers and
   some hairy code in these two functions.  */

unsafe fn postorder (root: *mut bin_tree_t, f: unsafe fn (*mut c_void, *mut bin_tree_t) -> reg_errcode_t,
                     extra: *mut c_void) -> reg_errcode_t
{
  let mut node: *mut bin_tree_t;
  let mut prev: *mut bin_tree_t;

  node = root;
  loop
    {
      /* Descend down the tree, preferably to the left (or to the right
	 if that's the only child).  */
      while !(*node).left.is_null () || !(*node).right.is_null ()
	{
	  if !(*node).left.is_null ()
	    { node = (*node).left; }
	  else
	    { node = (*node).right; }
	}

      loop
	{
	  let err: reg_errcode_t = f (extra, node);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { return err; }
	  if (*node).parent.is_null ()
	    { return REG_NOERROR; }
	  prev = node;
	  node = (*node).parent;
	  /* Go up while we have a node that is reached from the right.  */
	  if !((*node).right == prev || (*node).right.is_null ())
	    { break; }
	}
      node = (*node).right;
    }
}

unsafe fn preorder (root: *mut bin_tree_t, f: unsafe fn (*mut c_void, *mut bin_tree_t) -> reg_errcode_t,
                    extra: *mut c_void) -> reg_errcode_t
{
  let mut node: *mut bin_tree_t;

  node = root;
  loop
    {
      let err: reg_errcode_t = f (extra, node);
      if __glibc_unlikely (err != REG_NOERROR)
	{ return err; }

      /* Go to the left node, or up and to the right.  */
      if !(*node).left.is_null ()
	{ node = (*node).left; }
      else
	{
	  let mut prev: *mut bin_tree_t = null_mut ();
	  while (*node).right == prev || (*node).right.is_null ()
	    {
	      prev = node;
	      node = (*node).parent;
	      if node.is_null ()
		{ return REG_NOERROR; }
	    }
	  node = (*node).right;
	}
    }
}

/* Optimization pass: if a SUBEXP is entirely contained, strip it and tell
   re_search_internal to map the inner one's opr.idx to this one's.  */

unsafe fn optimize_subexps (extra: *mut c_void, node: *mut bin_tree_t) -> reg_errcode_t
{
  let dfa: *mut re_dfa_t = extra as *mut re_dfa_t;

  if type_ (&mut (*node).token) == OP_BACK_REF as c_uint && !(*dfa).subexp_map.is_null ()
    {
      let idx: c_int = (*node).token.opr.idx as c_int;
      (*node).token.opr.idx = *(*dfa).subexp_map.add (idx as usize);
      (*dfa).used_bkref_map |= 1 << (*node).token.opr.idx;
    }

  else if type_ (&mut (*node).token) == SUBEXP as c_uint
	   && !(*node).left.is_null () && type_ (&mut (*(*node).left).token) == SUBEXP as c_uint
    {
      let other_idx: Idx = (*(*node).left).token.opr.idx;

      (*node).left = (*(*node).left).left;
      if !(*node).left.is_null ()
	{ (*(*node).left).parent = node; }

      *(*dfa).subexp_map.add (other_idx as usize) =
          *(*dfa).subexp_map.add ((*node).token.opr.idx as usize);
      if other_idx < BITSET_WORD_BITS
	{ (*dfa).used_bkref_map &= !((1 as bitset_word_t) << other_idx); }
    }

  return REG_NOERROR;
}

/* Lowering pass: Turn each SUBEXP node into the appropriate concatenation
   of OP_OPEN_SUBEXP, the body of the SUBEXP (if any) and OP_CLOSE_SUBEXP.  */

unsafe fn lower_subexps (extra: *mut c_void, node: *mut bin_tree_t) -> reg_errcode_t
{
  let preg: *mut regex_t = extra as *mut regex_t;
  let mut err: reg_errcode_t = REG_NOERROR;

  if !(*node).left.is_null () && type_ (&mut (*(*node).left).token) == SUBEXP as c_uint
    {
      (*node).left = lower_subexp (&mut err, preg, (*node).left);
      if !(*node).left.is_null ()
	{ (*(*node).left).parent = node; }
    }
  if !(*node).right.is_null () && type_ (&mut (*(*node).right).token) == SUBEXP as c_uint
    {
      (*node).right = lower_subexp (&mut err, preg, (*node).right);
      if !(*node).right.is_null ()
	{ (*(*node).right).parent = node; }
    }

  return err;
}

unsafe fn lower_subexp (err: *mut reg_errcode_t, preg: *mut regex_t, node: *mut bin_tree_t)
                       -> *mut bin_tree_t
{
  let dfa: *mut re_dfa_t = (*preg).buffer as *mut re_dfa_t;
  let body: *mut bin_tree_t = (*node).left;
  let mut op: *mut bin_tree_t;
  let mut cls: *mut bin_tree_t;
  let mut tree1: *mut bin_tree_t;
  let mut tree: *mut bin_tree_t;

  if no_sub_ (preg) != 0
      /* We do not optimize empty subexpressions, because otherwise we may
	 have bad CONCAT nodes with NULL children.  */
      && !(*node).left.is_null ()
      && ((*node).token.opr.idx >= BITSET_WORD_BITS
	  || ((*dfa).used_bkref_map
	      & ((1 as bitset_word_t) << (*node).token.opr.idx)) == 0)
    { return (*node).left; }

  /* Convert the SUBEXP node to the concatenation of an
     OP_OPEN_SUBEXP, the contents, and an OP_CLOSE_SUBEXP.  */
  op = create_tree (dfa, null_mut (), null_mut (), OP_OPEN_SUBEXP as c_int);
  cls = create_tree (dfa, null_mut (), null_mut (), OP_CLOSE_SUBEXP as c_int);
  tree1 = if !body.is_null () { create_tree (dfa, body, cls, CONCAT) } else { cls };
  tree = create_tree (dfa, op, tree1, CONCAT);
  if __glibc_unlikely (tree.is_null () || tree1.is_null ()
			|| op.is_null () || cls.is_null ())
    {
      *err = REG_ESPACE;
      return null_mut ();
    }

  (*op).token.opr.idx = (*node).token.opr.idx;
  (*cls).token.opr.idx = (*node).token.opr.idx;
  set_opt_subexp_ (&mut (*op).token, opt_subexp_ (&mut (*node).token));
  set_opt_subexp_ (&mut (*cls).token, opt_subexp_ (&mut (*node).token));
  return tree;
}

/* Pass 1 in building the NFA: compute FIRST and create unlinked automaton
   nodes.  Requires a postorder visit.  */

unsafe fn calc_first (extra: *mut c_void, node: *mut bin_tree_t) -> reg_errcode_t
{
  let dfa: *mut re_dfa_t = extra as *mut re_dfa_t;
  if type_ (&mut (*node).token) == CONCAT as c_uint
    {
      (*node).first = (*(*node).left).first;
      (*node).node_idx = (*(*node).left).node_idx;
    }
  else
    {
      (*node).first = node;
      (*node).node_idx = re_dfa_add_node (dfa, (*node).token);
      if __glibc_unlikely ((*node).node_idx == -1)
	{ return REG_ESPACE; }
      if type_ (&mut (*node).token) == ANCHOR as c_uint
	{ set_constraint_ ((*dfa).nodes.add ((*node).node_idx as usize),
			   (*(*node).token).opr.ctx_type as c_uint); }
    }
  return REG_NOERROR;
}

/* Pass 2: compute NEXT on the tree.  Preorder visit.  */

unsafe fn calc_next (extra: *mut c_void, node: *mut bin_tree_t) -> reg_errcode_t
{
  let r#type: c_uint = type_ (&mut (*node).token);
  if r#type == OP_DUP_ASTERISK as c_uint
    {
      (*(*node).left).next = node;
    }
  else if r#type == CONCAT as c_uint
    {
      (*(*node).left).next = (*(*node).right).first;
      (*(*node).right).next = (*node).next;
    }
  else
    {
      if !(*node).left.is_null ()
	{ (*(*node).left).next = (*node).next; }
      if !(*node).right.is_null ()
	{ (*(*node).right).next = (*node).next; }
    }
  return REG_NOERROR;
}

/* Pass 3: link all DFA nodes to their NEXT node (any order will do).  */

unsafe fn link_nfa_nodes (extra: *mut c_void, node: *mut bin_tree_t) -> reg_errcode_t
{
  let dfa: *mut re_dfa_t = extra as *mut re_dfa_t;
  let idx: Idx = (*node).node_idx;
  let mut err: reg_errcode_t = REG_NOERROR;

  let r#type: c_uint = type_ (&mut (*node).token);
  if r#type == CONCAT as c_uint
    {
    }

  else if r#type == END_OF_RE as c_uint
    {
      DEBUG_ASSERT! ((*node).next.is_null ());
    }

  else if r#type == OP_DUP_ASTERISK as c_uint || r#type == OP_ALT as c_uint
    {
      let left: Idx;
      let right: Idx;
      set_has_plural_match_ (dfa, 1);
      if !(*node).left.is_null ()
	{ left = (*(*(*node).left).first).node_idx; }
      else
	{ left = (*(*node).next).node_idx; }
      if !(*node).right.is_null ()
	{ right = (*(*(*node).right).first).node_idx; }
      else
	{ right = (*(*node).next).node_idx; }
      DEBUG_ASSERT! (left > -1);
      DEBUG_ASSERT! (right > -1);
      err = re_node_set_init_2 ((*dfa).edests.add (idx as usize), left, right);
    }

  else if r#type == ANCHOR as c_uint
       || r#type == OP_OPEN_SUBEXP as c_uint
       || r#type == OP_CLOSE_SUBEXP as c_uint
    {
      err = re_node_set_init_1 ((*dfa).edests.add (idx as usize), (*(*node).next).node_idx);
    }

  else if r#type == OP_BACK_REF as c_uint
    {
      *(*dfa).nexts.add (idx as usize) = (*(*node).next).node_idx;
      if type_ (&mut (*node).token) == OP_BACK_REF as c_uint
	{ err = re_node_set_init_1 ((*dfa).edests.add (idx as usize), *(*dfa).nexts.add (idx as usize)); }
    }

  else
    {
      DEBUG_ASSERT! (IS_EPSILON_NODE! (type_ (&mut (*node).token)) == 0);
      *(*dfa).nexts.add (idx as usize) = (*(*node).next).node_idx;
    }

  return err;
}

/* Duplicate the epsilon closure of the node ROOT_NODE.
   Note that duplicated nodes have constraint INIT_CONSTRAINT in addition
   to their own constraint.  */

unsafe fn duplicate_node_closure (dfa: *mut re_dfa_t, top_org_node: Idx,
                                  top_clone_node: Idx, root_node: Idx,
                                  init_constraint: c_uint) -> reg_errcode_t
{
  let mut org_node: Idx;
  let mut clone_node: Idx;
  let mut ok: bool;
  let mut constraint: c_uint = init_constraint;
  org_node = top_org_node;
  clone_node = top_clone_node;
  loop
    {
      let mut org_dest: Idx;
      let mut clone_dest: Idx;
      if type_ ((*dfa).nodes.add (org_node as usize)) == OP_BACK_REF as c_uint
	{
	  /* If the back reference epsilon-transit, its destination must
	     also have the constraint.  Then duplicate the epsilon closure
	     of the destination of the back reference, and store it in
	     edests of the back reference.  */
	  org_dest = *(*dfa).nexts.add (org_node as usize);
	  re_node_set_empty! ((*dfa).edests.add (clone_node as usize));
	  clone_dest = duplicate_node (dfa, org_dest, constraint);
	  if __glibc_unlikely (clone_dest == -1)
	    { return REG_ESPACE; }
	  *(*dfa).nexts.add (clone_node as usize) = *(*dfa).nexts.add (org_node as usize);
	  ok = re_node_set_insert ((*dfa).edests.add (clone_node as usize), clone_dest);
	  if __glibc_unlikely (! ok)
	    { return REG_ESPACE; }
	}
      else if (*((*dfa).edests.add (org_node as usize))).nelem == 0
	{
	  /* In case of the node can't epsilon-transit, don't duplicate the
	     destination and store the original destination as the
	     destination of the node.  */
	  *(*dfa).nexts.add (clone_node as usize) = *(*dfa).nexts.add (org_node as usize);
	  break;
	}
      else if (*((*dfa).edests.add (org_node as usize))).nelem == 1
	{
	  /* In case of the node can epsilon-transit, and it has only one
	     destination.  */
	  org_dest = *(*((*dfa).edests.add (org_node as usize))).elems.add (0);
	  re_node_set_empty! ((*dfa).edests.add (clone_node as usize));
	  /* If the node is root_node itself, it means the epsilon closure
	     has a loop.  Then tie it to the destination of the root_node.  */
	  if org_node == root_node && clone_node != org_node
	    {
	      ok = re_node_set_insert ((*dfa).edests.add (clone_node as usize), org_dest);
	      if __glibc_unlikely (! ok)
	        { return REG_ESPACE; }
	      break;
	    }
	  /* In case the node has another constraint, append it.  */
	  constraint |= constraint_ ((*dfa).nodes.add (org_node as usize));
	  clone_dest = duplicate_node (dfa, org_dest, constraint);
	  if __glibc_unlikely (clone_dest == -1)
	    { return REG_ESPACE; }
	  ok = re_node_set_insert ((*dfa).edests.add (clone_node as usize), clone_dest);
	  if __glibc_unlikely (! ok)
	    { return REG_ESPACE; }
	}
      else /* dfa->edests[org_node].nelem == 2 */
	{
	  /* In case of the node can epsilon-transit, and it has two
	     destinations. In the bin_tree_t and DFA, that's '|' and '*'.   */
	  org_dest = *(*((*dfa).edests.add (org_node as usize))).elems.add (0);
	  re_node_set_empty! ((*dfa).edests.add (clone_node as usize));
	  /* Search for a duplicated node which satisfies the constraint.  */
	  clone_dest = search_duplicated_node (dfa, org_dest, constraint);
	  if clone_dest == -1
	    {
	      /* There is no such duplicated node, create a new one.  */
	      let err: reg_errcode_t;
	      clone_dest = duplicate_node (dfa, org_dest, constraint);
	      if __glibc_unlikely (clone_dest == -1)
		{ return REG_ESPACE; }
	      ok = re_node_set_insert ((*dfa).edests.add (clone_node as usize), clone_dest);
	      if __glibc_unlikely (! ok)
		{ return REG_ESPACE; }
	      err = duplicate_node_closure (dfa, org_dest, clone_dest,
					    root_node, constraint);
	      if __glibc_unlikely (err != REG_NOERROR)
		{ return err; }
	    }
	  else
	    {
	      /* There is a duplicated node which satisfies the constraint,
		 use it to avoid infinite loop.  */
	      ok = re_node_set_insert ((*dfa).edests.add (clone_node as usize), clone_dest);
	      if __glibc_unlikely (! ok)
		{ return REG_ESPACE; }
	    }

	  org_dest = *(*((*dfa).edests.add (org_node as usize))).elems.add (1);
	  clone_dest = duplicate_node (dfa, org_dest, constraint);
	  if __glibc_unlikely (clone_dest == -1)
	    { return REG_ESPACE; }
	  ok = re_node_set_insert ((*dfa).edests.add (clone_node as usize), clone_dest);
	  if __glibc_unlikely (! ok)
	    { return REG_ESPACE; }
	}
      org_node = org_dest;
      clone_node = clone_dest;
    }
  return REG_NOERROR;
}

/* Search for a node which is duplicated from the node ORG_NODE, and
   satisfies the constraint CONSTRAINT.  */

unsafe fn search_duplicated_node (dfa: *const re_dfa_t, org_node: Idx,
                                  constraint: c_uint) -> Idx
{
  let mut idx: Idx;
  idx = (*dfa).nodes_len as Idx - 1;
  while duplicated_ ((*dfa).nodes.add (idx as usize)) != 0 && idx > 0
    {
      if org_node == *(*dfa).org_indices.add (idx as usize)
	  && constraint == constraint_ ((*dfa).nodes.add (idx as usize))
	{ return idx; /* Found.  */ }
      idx -= 1;
    }
  return -1; /* Not found.  */
}

/* Duplicate the node whose index is ORG_IDX and set the constraint CONSTRAINT.  */

unsafe fn duplicate_node (dfa: *mut re_dfa_t, org_idx: Idx, constraint: c_uint) -> Idx
{
  let dup_idx: Idx = re_dfa_add_node (dfa, *(*dfa).nodes.add (org_idx as usize));
  if __glibc_likely (dup_idx != -1)
    {
      set_constraint_ ((*dfa).nodes.add (dup_idx as usize), constraint);
      set_constraint_ ((*dfa).nodes.add (dup_idx as usize),
		       constraint_ ((*dfa).nodes.add (dup_idx as usize))
		       | constraint_ ((*dfa).nodes.add (org_idx as usize)));
      set_duplicated_ ((*dfa).nodes.add (dup_idx as usize), 1);

      /* Store the index of the original node.  */
      *(*dfa).org_indices.add (dup_idx as usize) = org_idx;
    }
  return dup_idx;
}

unsafe fn calc_inveclosure (dfa: *mut re_dfa_t) -> reg_errcode_t
{
  let mut src: Idx;
  let mut idx: Idx;
  let mut ok: bool;
  idx = 0;
  while idx < (*dfa).nodes_len as Idx
    {
      re_node_set_init_empty! ((*dfa).inveclosures.add (idx as usize));
      idx += 1;
    }

  src = 0;
  while src < (*dfa).nodes_len as Idx
    {
      let elems: *mut Idx = (*((*dfa).eclosures.add (src as usize))).elems;
      idx = 0;
      while idx < (*((*dfa).eclosures.add (src as usize))).nelem
	{
	  ok = re_node_set_insert_last ((*dfa).inveclosures.add (*elems.add (idx as usize) as usize), src);
	  if __glibc_unlikely (! ok)
	    { return REG_ESPACE; }
	  idx += 1;
	}
      src += 1;
    }

  return REG_NOERROR;
}

/* Calculate "eclosure" for all the node in DFA.  */

unsafe fn calc_eclosure (dfa: *mut re_dfa_t) -> reg_errcode_t
{
  let mut node_idx: Idx;
  let mut incomplete: bool;
  DEBUG_ASSERT! ((*dfa).nodes_len > 0);
  incomplete = false;
  /* For each nodes, calculate epsilon closure.  */
  node_idx = 0;
  loop
    {
      let err: reg_errcode_t;
      let mut eclosure_elem: re_node_set = re_node_set { alloc: 0, nelem: 0, elems: null_mut () };
      if node_idx == (*dfa).nodes_len as Idx
	{
	  if !incomplete
	    { break; }
	  incomplete = false;
	  node_idx = 0;
	}

      DEBUG_ASSERT! ((*((*dfa).eclosures.add (node_idx as usize))).nelem != -1);

      /* If we have already calculated, skip it.  */
      if (*((*dfa).eclosures.add (node_idx as usize))).nelem != 0
	{
	  node_idx += 1; continue;
	}
      /* Calculate epsilon closure of 'node_idx'.  */
      err = calc_eclosure_iter (&mut eclosure_elem, dfa, node_idx, true);
      if __glibc_unlikely (err != REG_NOERROR)
	{ return err; }

      if (*((*dfa).eclosures.add (node_idx as usize))).nelem == 0
	{
	  incomplete = true;
	  re_node_set_free! (&mut eclosure_elem);
	}
      node_idx += 1;
    }
  return REG_NOERROR;
}

/* Calculate epsilon closure of NODE.  */

unsafe fn calc_eclosure_iter (new_set: *mut re_node_set, dfa: *mut re_dfa_t, node: Idx,
                              root: bool) -> reg_errcode_t
{
  let mut err: reg_errcode_t;
  let mut i: Idx;
  let mut eclosure: re_node_set = re_node_set { alloc: 0, nelem: 0, elems: null_mut () };
  let mut incomplete: bool = false;
  err = re_node_set_alloc (&mut eclosure, (*((*dfa).edests.add (node as usize))).nelem + 1);
  if __glibc_unlikely (err != REG_NOERROR)
    { return err; }

  /* An epsilon closure includes itself.  */
  *eclosure.elems.add (eclosure.nelem as usize) = node;
  eclosure.nelem += 1;

  /* This indicates that we are calculating this node now.
     We reference this value to avoid infinite loop.  */
  (*((*dfa).eclosures.add (node as usize))).nelem = -1;

  /* If the current node has constraints, duplicate all nodes
     since they must inherit the constraints.  */
  if constraint_ ((*dfa).nodes.add (node as usize)) != 0
      && (*((*dfa).edests.add (node as usize))).nelem != 0
      && duplicated_ ((*dfa).nodes.add (
             *(*((*dfa).edests.add (node as usize))).elems.add (0) as usize)) == 0
    {
      err = duplicate_node_closure (dfa, node, node, node,
				    constraint_ ((*dfa).nodes.add (node as usize)));
    }

  /* Expand each epsilon destination nodes.  */
  if __glibc_likely (err == REG_NOERROR)
      && IS_EPSILON_NODE! (type_ ((*dfa).nodes.add (node as usize))) != 0
    {
      i = 0;
      while i < (*((*dfa).edests.add (node as usize))).nelem
	{
	  let mut eclosure_elem: re_node_set = re_node_set { alloc: 0, nelem: 0, elems: null_mut () };
	  let edest: Idx = *(*((*dfa).edests.add (node as usize))).elems.add (i as usize);
	  /* If calculating the epsilon closure of 'edest' is in progress,
	     return intermediate result.  */
	  if (*((*dfa).eclosures.add (edest as usize))).nelem == -1
	    {
	      incomplete = true;
	      i += 1; continue;
	    }
	  /* If we haven't calculated the epsilon closure of 'edest' yet,
	     calculate now. Otherwise use calculated epsilon closure.  */
	  if (*((*dfa).eclosures.add (edest as usize))).nelem == 0
	    {
	      err = calc_eclosure_iter (&mut eclosure_elem, dfa, edest, false);
	      if __glibc_unlikely (err != REG_NOERROR)
		{ break; }
	    }
	  else
	    { eclosure_elem = *(*dfa).eclosures.add (edest as usize); }
	  /* Merge the epsilon closure of 'edest'.  */
	  err = re_node_set_merge (&mut eclosure, &eclosure_elem);
	  if __glibc_unlikely (err != REG_NOERROR)
	    { break; }
	  /* If the epsilon closure of 'edest' is incomplete,
	     the epsilon closure of this node is also incomplete.  */
	  if (*((*dfa).eclosures.add (edest as usize))).nelem == 0
	    {
	      incomplete = true;
	      re_node_set_free! (&mut eclosure_elem);
	    }
	  i += 1;
	}
    }

  if err != REG_NOERROR
    { re_node_set_free! (&mut eclosure); }
  else
    {
      if incomplete && !root
	{ (*((*dfa).eclosures.add (node as usize))).nelem = 0; }
      else
	{ *(*dfa).eclosures.add (node as usize) = eclosure; }
      *new_set = eclosure;
    }

  return err;
}

/* Functions for token which are used in the parser.  */

/* Fetch a token from INPUT.
   We must not use this function inside bracket expressions.  */

unsafe fn fetch_token (result: *mut re_token_t, input: *mut re_string_t, syntax: reg_syntax_t)
{
  re_string_skip_bytes! (input, peek_token (result, input, syntax));
}

/* Peek a token from INPUT, and return the length of the token.
   We must not use this function inside bracket expressions.  */

unsafe fn peek_token (token: *mut re_token_t, input: *mut re_string_t, syntax: reg_syntax_t)
                      -> c_int
{
  let c: u8;

  if re_string_eoi! (input) != 0
    {
      set_type_ (token, END_OF_RE as c_uint);
      return 0;
    }

  c = re_string_peek_byte! (input, 0);
  (*token).opr.c = c;

  set_word_char_ (token, 0);
  set_mb_partial_ (token, 0);
  if (*input).mb_cur_max > 1
      && re_string_first_byte! (input, re_string_cur_idx! (input)) == 0
    {
      set_type_ (token, CHARACTER as c_uint);
      set_mb_partial_ (token, 1);
      return 1;
    }
  if c == b'\\'
    {
      let mut c2: u8;
      if re_string_cur_idx! (input) + 1 >= re_string_length! (input)
	{
	  set_type_ (token, BACK_SLASH as c_uint);
	  return 1;
	}

      c2 = re_string_peek_byte_case (input, 1);
      (*token).opr.c = c2;
      set_type_ (token, CHARACTER as c_uint);
      if (*input).mb_cur_max > 1
	{
	  let wc: wint_t = re_string_wchar_at (input, re_string_cur_idx! (input) + 1);
	  set_word_char_ (token, IS_WIDE_WORD_CHAR! (wc) as c_uint);
	}
      else
	{ set_word_char_ (token, IS_WORD_CHAR! (c2 as c_int) as c_uint); }

      if c2 == b'|'
	{
	  if syntax & RE_LIMITED_OPS == 0 && syntax & RE_NO_BK_VBAR == 0
	    { set_type_ (token, OP_ALT as c_uint); }
	}
      else if c2 == b'1' || c2 == b'2' || c2 == b'3' || c2 == b'4' || c2 == b'5'
	   || c2 == b'6' || c2 == b'7' || c2 == b'8' || c2 == b'9'
	{
	  if syntax & RE_NO_BK_REFS == 0
	    {
	      set_type_ (token, OP_BACK_REF as c_uint);
	      (*token).opr.idx = (c2 - b'1') as Idx;
	    }
	}
      else if c2 == b'<'
	{
	  if syntax & RE_NO_GNU_OPS == 0
	    {
	      set_type_ (token, ANCHOR as c_uint);
	      (*token).opr.ctx_type = re_context_type::WORD_FIRST;
	    }
	}
      else if c2 == b'>'
	{
	  if syntax & RE_NO_GNU_OPS == 0
	    {
	      set_type_ (token, ANCHOR as c_uint);
	      (*token).opr.ctx_type = re_context_type::WORD_LAST;
	    }
	}
      else if c2 == b'b'
	{
	  if syntax & RE_NO_GNU_OPS == 0
	    {
	      set_type_ (token, ANCHOR as c_uint);
	      (*token).opr.ctx_type = re_context_type::WORD_DELIM;
	    }
	}
      else if c2 == b'B'
	{
	  if syntax & RE_NO_GNU_OPS == 0
	    {
	      set_type_ (token, ANCHOR as c_uint);
	      (*token).opr.ctx_type = re_context_type::NOT_WORD_DELIM;
	    }
	}
      else if c2 == b'w'
	{
	  if syntax & RE_NO_GNU_OPS == 0
	    { set_type_ (token, OP_WORD as c_uint); }
	}
      else if c2 == b'W'
	{
	  if syntax & RE_NO_GNU_OPS == 0
	    { set_type_ (token, OP_NOTWORD as c_uint); }
	}
      else if c2 == b's'
	{
	  if syntax & RE_NO_GNU_OPS == 0
	    { set_type_ (token, OP_SPACE as c_uint); }
	}
      else if c2 == b'S'
	{
	  if syntax & RE_NO_GNU_OPS == 0
	    { set_type_ (token, OP_NOTSPACE as c_uint); }
	}
      else if c2 == b'`'
	{
	  if syntax & RE_NO_GNU_OPS == 0
	    {
	      set_type_ (token, ANCHOR as c_uint);
	      (*token).opr.ctx_type = re_context_type::BUF_FIRST;
	    }
	}
      else if c2 == b'\''
	{
	  if syntax & RE_NO_GNU_OPS == 0
	    {
	      set_type_ (token, ANCHOR as c_uint);
	      (*token).opr.ctx_type = re_context_type::BUF_LAST;
	    }
	}
      else if c2 == b'('
	{
	  if syntax & RE_NO_BK_PARENS == 0
	    { set_type_ (token, OP_OPEN_SUBEXP as c_uint); }
	}
      else if c2 == b')'
	{
	  if syntax & RE_NO_BK_PARENS == 0
	    { set_type_ (token, OP_CLOSE_SUBEXP as c_uint); }
	}
      else if c2 == b'+'
	{
	  if syntax & RE_LIMITED_OPS == 0 && syntax & RE_BK_PLUS_QM != 0
	    { set_type_ (token, OP_DUP_PLUS as c_uint); }
	}
      else if c2 == b'?'
	{
	  if syntax & RE_LIMITED_OPS == 0 && syntax & RE_BK_PLUS_QM != 0
	    { set_type_ (token, OP_DUP_QUESTION as c_uint); }
	}
      else if c2 == b'{'
	{
	  if syntax & RE_INTERVALS != 0 && syntax & RE_NO_BK_BRACES == 0
	    { set_type_ (token, OP_OPEN_DUP_NUM as c_uint); }
	}
      else if c2 == b'}'
	{
	  if syntax & RE_INTERVALS != 0 && syntax & RE_NO_BK_BRACES == 0
	    { set_type_ (token, OP_CLOSE_DUP_NUM as c_uint); }
	}
      return 2;
    }

  set_type_ (token, CHARACTER as c_uint);
  if (*input).mb_cur_max > 1
    {
      let wc: wint_t = re_string_wchar_at (input, re_string_cur_idx! (input));
      set_word_char_ (token, IS_WIDE_WORD_CHAR! (wc) as c_uint);
    }
  else
    { set_word_char_ (token, IS_WORD_CHAR! ((*token).opr.c as c_int) as c_uint); }

  if c == b'\n'
    {
      if syntax & RE_NEWLINE_ALT != 0
	{ set_type_ (token, OP_ALT as c_uint); }
    }
  else if c == b'|'
    {
      if syntax & RE_LIMITED_OPS == 0 && syntax & RE_NO_BK_VBAR != 0
	{ set_type_ (token, OP_ALT as c_uint); }
    }
  else if c == b'*'
    {
      set_type_ (token, OP_DUP_ASTERISK as c_uint);
    }
  else if c == b'+'
    {
      if syntax & RE_LIMITED_OPS == 0 && syntax & RE_BK_PLUS_QM == 0
	{ set_type_ (token, OP_DUP_PLUS as c_uint); }
    }
  else if c == b'?'
    {
      if syntax & RE_LIMITED_OPS == 0 && syntax & RE_BK_PLUS_QM == 0
	{ set_type_ (token, OP_DUP_QUESTION as c_uint); }
    }
  else if c == b'{'
    {
      if syntax & RE_INTERVALS != 0 && syntax & RE_NO_BK_BRACES != 0
	{ set_type_ (token, OP_OPEN_DUP_NUM as c_uint); }
    }
  else if c == b'}'
    {
      if syntax & RE_INTERVALS != 0 && syntax & RE_NO_BK_BRACES != 0
	{ set_type_ (token, OP_CLOSE_DUP_NUM as c_uint); }
    }
  else if c == b'('
    {
      if syntax & RE_NO_BK_PARENS != 0
	{ set_type_ (token, OP_OPEN_SUBEXP as c_uint); }
    }
  else if c == b')'
    {
      if syntax & RE_NO_BK_PARENS != 0
	{ set_type_ (token, OP_CLOSE_SUBEXP as c_uint); }
    }
  else if c == b'['
    {
      set_type_ (token, OP_OPEN_BRACKET as c_uint);
    }
  else if c == b'.'
    {
      set_type_ (token, OP_PERIOD as c_uint);
    }
  else if c == b'^'
    {
      let mut matched = true;
      if syntax & (RE_CONTEXT_INDEP_ANCHORS | RE_CARET_ANCHORS_HERE) == 0
	  && re_string_cur_idx! (input) != 0
	{
	  let prev: c_char = re_string_peek_byte! (input, -1) as c_char;
	  if syntax & RE_NEWLINE_ALT == 0 || prev != b'\n' as c_char
	    { matched = false; }
	}
      if matched
	{
	  set_type_ (token, ANCHOR as c_uint);
	  (*token).opr.ctx_type = re_context_type::LINE_FIRST;
	}
    }
  else if c == b'$'
    {
      let mut matched = true;
      if syntax & RE_CONTEXT_INDEP_ANCHORS == 0
	  && re_string_cur_idx! (input) + 1 != re_string_length! (input)
	{
	  let mut next: re_token_t = re_token_t { opr: re_token_opr_t { c: 0 }, bits: 0 };
	  re_string_skip_bytes! (input, 1);
	  peek_token (&mut next, input, syntax);
	  re_string_skip_bytes! (input, -1);
	  if type_ (&mut next) != OP_ALT as c_uint && type_ (&mut next) != OP_CLOSE_SUBEXP as c_uint
	    { matched = false; }
	}
      if matched
	{
	  set_type_ (token, ANCHOR as c_uint);
	  (*token).opr.ctx_type = re_context_type::LINE_LAST;
	}
    }
  return 1;
}

/* Peek a token from INPUT, and return the length of the token.
   We must not use this function out of bracket expressions.  */

unsafe fn peek_token_bracket (token: *mut re_token_t, input: *mut re_string_t,
                              syntax: reg_syntax_t) -> c_int
{
  let c: u8;
  if re_string_eoi! (input) != 0
    {
      set_type_ (token, END_OF_RE as c_uint);
      return 0;
    }
  c = re_string_peek_byte! (input, 0);
  (*token).opr.c = c;

  if (*input).mb_cur_max > 1
      && re_string_first_byte! (input, re_string_cur_idx! (input)) == 0
    {
      set_type_ (token, CHARACTER as c_uint);
      return 1;
    }

  if c == b'\\' && syntax & RE_BACKSLASH_ESCAPE_IN_LISTS != 0
      && re_string_cur_idx! (input) + 1 < re_string_length! (input)
    {
      /* In this case, '\' escape a character.  */
      let c2: u8;
      re_string_skip_bytes! (input, 1);
      c2 = re_string_peek_byte! (input, 0);
      (*token).opr.c = c2;
      set_type_ (token, CHARACTER as c_uint);
      return 1;
    }
  if c == b'[' /* '[' is a special char in a bracket exps.  */
    {
      let mut c2: u8;
      let mut token_len: c_int;
      if re_string_cur_idx! (input) + 1 < re_string_length! (input)
	{ c2 = re_string_peek_byte! (input, 1); }
      else
	{ c2 = 0; }
      (*token).opr.c = c2;
      token_len = 2;
      if c2 == b'.'
	{
	  set_type_ (token, OP_OPEN_COLL_ELEM as c_uint);
	}

      else if c2 == b'='
	{
	  set_type_ (token, OP_OPEN_EQUIV_CLASS as c_uint);
	}

      else if c2 == b':'
	{
	  if syntax & RE_CHAR_CLASSES != 0
	    {
	      set_type_ (token, OP_OPEN_CHAR_CLASS as c_uint);
	    }
	  else
	    {
	      FALLTHROUGH! ();
	      set_type_ (token, CHARACTER as c_uint);
	      (*token).opr.c = c;
	      token_len = 1;
	    }
	}
      else
	{
	  set_type_ (token, CHARACTER as c_uint);
	  (*token).opr.c = c;
	  token_len = 1;
	}
      return token_len;
    }
  if c == b']'
    {
      set_type_ (token, OP_CLOSE_BRACKET as c_uint);
    }
  else if c == b'^'
    {
      set_type_ (token, OP_NON_MATCH_LIST as c_uint);
    }
  else if c == b'-'
    {
      /* In V7 Unix grep and Unix awk and mawk, [...---...]
         (3 adjacent minus signs) stands for a single minus sign.
         Support that without breaking anything else.  */
      if ! (re_string_cur_idx! (input) + 2 < re_string_length! (input)
            && re_string_peek_byte! (input, 1) == b'-'
            && re_string_peek_byte! (input, 2) == b'-')
        {
          set_type_ (token, OP_CHARSET_RANGE as c_uint);
        }
      else
	{
	  re_string_skip_bytes! (input, 2);
	  FALLTHROUGH! ();
	  set_type_ (token, CHARACTER as c_uint);
	}
    }
  else
    {
      set_type_ (token, CHARACTER as c_uint);
    }
  return 1;
}

/* Fetch a number.  Return the number,
   Return -1 if the number field is empty like "{,1}".
   Return RE_DUP_MAX + 1 if the number field is too large.
   Return -2 if an error occurred.  */

unsafe fn fetch_number (input: *mut re_string_t, token: *mut re_token_t,
                        syntax: reg_syntax_t) -> Idx
{
  let mut num: Idx = -1;
  let mut c: u8;
  loop
    {
      fetch_token (token, input, syntax);
      c = (*token).opr.c;
      if __glibc_unlikely (type_ (token) == END_OF_RE as c_uint)
	{ return -2; }
      if type_ (token) == OP_CLOSE_DUP_NUM as c_uint || c == b','
	{ break; }
      num = if type_ (token) != CHARACTER as c_uint || c < b'0' || b'9' < c || num == -2
	    { -2 }
	    else if num == -1
	    { (c - b'0') as Idx }
	    else
	    { MIN! (RE_DUP_MAX + 1, num * 10 + (c - b'0') as Idx) };
    }
  return num;
}

unsafe fn free_charset (cset: *mut re_charset_t)
{
  re_free! ((*cset).mbchars);
  // C: `#ifdef _LIBC re_free (cset->coll_syms); re_free (cset->equiv_classes); #endif` --
  // dead in the frozen configuration (the members do not exist on this target).
  re_free! ((*cset).range_starts);
  re_free! ((*cset).range_ends);
  re_free! ((*cset).char_classes);
  re_free! (cset);
}

/* Functions for binary tree operation.  */

/* Create a tree node.  */

unsafe fn create_tree (dfa: *mut re_dfa_t, left: *mut bin_tree_t, right: *mut bin_tree_t,
                       type_: c_int) -> *mut bin_tree_t
{
  let mut t: re_token_t = re_token_t { opr: re_token_opr_t { c: 0 }, bits: 0 };
  set_type_ (&mut t, type_ as c_uint);
  return create_token_tree (dfa, left, right, &mut t);
}

unsafe fn create_token_tree (dfa: *mut re_dfa_t, left: *mut bin_tree_t,
                             right: *mut bin_tree_t, token: *const re_token_t)
                             -> *mut bin_tree_t
{
  let mut tree: *mut bin_tree_t;
  if __glibc_unlikely ((*dfa).str_tree_storage_idx == BIN_TREE_STORAGE_SIZE as c_int)
    {
      let storage: *mut bin_tree_storage_t = re_malloc! (bin_tree_storage_t, 1);

      if storage.is_null ()
	{ return null_mut (); }
      (*storage).next = (*dfa).str_tree_storage;
      (*dfa).str_tree_storage = storage;
      (*dfa).str_tree_storage_idx = 0;
    }
  tree = (*((*dfa).str_tree_storage)).data
             .as_mut_ptr ().add ((*dfa).str_tree_storage_idx as usize);
  (*dfa).str_tree_storage_idx += 1;

  (*tree).parent = null_mut ();
  (*tree).left = left;
  (*tree).right = right;
  (*tree).token = *token;
  set_duplicated_ (&mut (*tree).token, 0);
  set_opt_subexp_ (&mut (*tree).token, 0);
  (*tree).first = null_mut ();
  (*tree).next = null_mut ();
  (*tree).node_idx = -1;

  if !left.is_null ()
    { (*left).parent = tree; }
  if !right.is_null ()
    { (*right).parent = tree; }
  return tree;
}

/* Mark the tree SRC as an optional subexpression.
   To be called from preorder or postorder.  */

unsafe fn mark_opt_subexp (extra: *mut c_void, node: *mut bin_tree_t) -> reg_errcode_t
{
  let idx: Idx = extra as usize as Idx;
  if type_ (&mut (*node).token) == SUBEXP as c_uint && (*node).token.opr.idx == idx
    { set_opt_subexp_ (&mut (*node).token, 1); }

  return REG_NOERROR;
}

/* Free the allocated memory inside NODE. */

unsafe fn free_token (node: *mut re_token_t)
{
  if type_ (node) == COMPLEX_BRACKET as c_uint && duplicated_ (node) == 0
    { free_charset ((*node).opr.mbcset); }
  else if type_ (node) == SIMPLE_BRACKET as c_uint && duplicated_ (node) == 0
    { re_free! ((*node).opr.sbcset); }
}

/* Worker function for tree walking.  Free the allocated memory inside NODE
   and its children. */

unsafe fn free_tree (extra: *mut c_void, node: *mut bin_tree_t) -> reg_errcode_t
{
  free_token (&mut (*node).token);
  return REG_NOERROR;
}

/* Duplicate the node SRC, and return new node.  */

unsafe fn duplicate_tree (root: *const bin_tree_t, dfa: *mut re_dfa_t) -> *mut bin_tree_t
{
  let mut node: *const bin_tree_t;
  let mut dup_root: *mut bin_tree_t = null_mut ();
  let mut p_new: *mut *mut bin_tree_t = &mut dup_root;
  let mut dup_node: *mut bin_tree_t = (*root).parent;

  node = root;
  loop
    {
      /* Create a new tree and link it back to the current parent.  */
      *p_new = create_token_tree (dfa, null_mut (), null_mut (), &(*node).token);
      if (*p_new).is_null ()
	{ return null_mut (); }
      (**p_new).parent = dup_node;
      set_duplicated_ (&mut (**p_new).token, 1);
      dup_node = *p_new;

      /* Go to the left node, or up and to the right.  */
      if !(*node).left.is_null ()
	{
	  node = (*node).left;
	  p_new = &raw mut (*dup_node).left;
	}
      else
	{
	  let mut prev: *const bin_tree_t = null_mut ();
	  while (*node).right == prev || (*node).right.is_null ()
	    {
	      prev = node;
	      node = (*node).parent;
	      dup_node = (*dup_node).parent;
	      if node.is_null ()
		{ return dup_root; }
	    }
	  node = (*node).right;
	  p_new = &raw mut (*dup_node).right;
	}
    }
}

/* Functions for parser.  */

/* Entry point of the parser.  */

unsafe fn parse (regexp: *mut re_string_t, preg: *mut regex_t, syntax: reg_syntax_t,
                 err: *mut reg_errcode_t) -> *mut bin_tree_t
{
  let dfa: *mut re_dfa_t = (*preg).buffer as *mut re_dfa_t;
  let mut tree: *mut bin_tree_t;
  let mut eor: *mut bin_tree_t;
  let mut root: *mut bin_tree_t;
  let mut current_token: re_token_t = re_token_t { opr: re_token_opr_t { c: 0 }, bits: 0 };
  (*dfa).syntax = syntax;
  fetch_token (&mut current_token, regexp, syntax | RE_CARET_ANCHORS_HERE);
  tree = parse_reg_exp (regexp, preg, &mut current_token, syntax, 0, err);
  if __glibc_unlikely (*err != REG_NOERROR && tree.is_null ())
    { return null_mut (); }
  eor = create_tree (dfa, null_mut (), null_mut (), END_OF_RE);
  if !tree.is_null ()
    { root = create_tree (dfa, tree, eor, CONCAT); }
  else
    { root = eor; }
  if __glibc_unlikely (eor.is_null () || root.is_null ())
    {
      *err = REG_ESPACE;
      return null_mut ();
    }
  return root;
}

/* This function build the following tree, from regular expression
   <branch1>|<branch2>:
	   ALT
	   / \
	  /   \
   <branch1> <branch2>  */

unsafe fn parse_reg_exp (regexp: *mut re_string_t, preg: *mut regex_t,
                         token: *mut re_token_t, syntax: reg_syntax_t, nest: Idx,
                         err: *mut reg_errcode_t) -> *mut bin_tree_t
{
  let dfa: *mut re_dfa_t = (*preg).buffer as *mut re_dfa_t;
  let mut tree: *mut bin_tree_t;
  let mut branch: *mut bin_tree_t = null_mut ();
  let initial_bkref_map: bitset_word_t = (*dfa).completed_bkref_map;
  tree = parse_branch (regexp, preg, token, syntax, nest, err);
  if __glibc_unlikely (*err != REG_NOERROR && tree.is_null ())
    { return null_mut (); }

  while type_ (token) == OP_ALT as c_uint
    {
      fetch_token (token, regexp, syntax | RE_CARET_ANCHORS_HERE);
      if type_ (token) != OP_ALT as c_uint && type_ (token) != END_OF_RE as c_uint
	  && (nest == 0 || type_ (token) != OP_CLOSE_SUBEXP as c_uint)
	{
	  let accumulated_bkref_map: bitset_word_t = (*dfa).completed_bkref_map;
	  (*dfa).completed_bkref_map = initial_bkref_map;
	  branch = parse_branch (regexp, preg, token, syntax, nest, err);
	  if __glibc_unlikely (*err != REG_NOERROR && branch.is_null ())
	    {
	      if !tree.is_null ()
		{ let _ = postorder (tree, free_tree, null_mut ()); }
	      return null_mut ();
	    }
	  (*dfa).completed_bkref_map |= accumulated_bkref_map;
	}
      else
	{ branch = null_mut (); }
      tree = create_tree (dfa, tree, branch, OP_ALT as c_int);
      if __glibc_unlikely (tree.is_null ())
	{
	  *err = REG_ESPACE;
	  return null_mut ();
	}
    }
  return tree;
}

/* This function build the following tree, from regular expression
   <exp1><exp2>:
	CAT
	/ \
       /   \
   <exp1> <exp2>  */

unsafe fn parse_branch (regexp: *mut re_string_t, preg: *mut regex_t, token: *mut re_token_t,
                        syntax: reg_syntax_t, nest: Idx, err: *mut reg_errcode_t)
                        -> *mut bin_tree_t
{
  let mut tree: *mut bin_tree_t;
  let mut expr: *mut bin_tree_t;
  let dfa: *mut re_dfa_t = (*preg).buffer as *mut re_dfa_t;
  tree = parse_expression (regexp, preg, token, syntax, nest, err);
  if __glibc_unlikely (*err != REG_NOERROR && tree.is_null ())
    { return null_mut (); }

  while type_ (token) != OP_ALT as c_uint && type_ (token) != END_OF_RE as c_uint
	 && (nest == 0 || type_ (token) != OP_CLOSE_SUBEXP as c_uint)
    {
      expr = parse_expression (regexp, preg, token, syntax, nest, err);
      if __glibc_unlikely (*err != REG_NOERROR && expr.is_null ())
	{
	  if !tree.is_null ()
	    { let _ = postorder (tree, free_tree, null_mut ()); }
	  return null_mut ();
	}
      if !tree.is_null () && !expr.is_null ()
	{
	  let newtree: *mut bin_tree_t = create_tree (dfa, tree, expr, CONCAT);
	  if newtree.is_null ()
	    {
	      let _ = postorder (expr, free_tree, null_mut ());
	      let _ = postorder (tree, free_tree, null_mut ());
	      *err = REG_ESPACE;
	      return null_mut ();
	    }
	  tree = newtree;
	}
      else if tree.is_null ()
	{ tree = expr; }
      /* Otherwise expr == NULL, we don't need to create new tree.  */
    }
  return tree;
}

/* Convert the byte B to the corresponding wide character.  */

unsafe fn parse_byte (b: u8, dfa: *const re_dfa_t) -> wint_t
{
  return if (*dfa).mb_cur_max > 1 { __btowc! (b as c_int) as wint_t } else { b as wint_t };
}
