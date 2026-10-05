// femacs: port of src/composite.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct, struct field, file-scope object, local), same control flow, same error
// model.  The six `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's non-`static` definitions
// (`get_composition_id`, `find_composition`, `update_compositions`,
// `make_composition_value_copy`, `compose_text`, `composition_gstring_lookup_cache`,
// `composition_gstring_put_cache`, `composition_gstring_from_id`,
// `composition_gstring_cache_clear_font`, `composition_gstring_p`,
// `composition_gstring_width`, `composition_gstring_adjust_zero_width`,
// `composition_compute_stop_pos`, `composition_reseat_it`, `composition_update_it`,
// `find_automatic_composition`, `composition_adjust_point`, `mark_composite`,
// `syms_of_composite`) carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static`
// helpers stay plain `unsafe fn` (`run_composition_function`, `fill_gstring_header`,
// `fill_gstring_body`, `autocmp_chars`, `char_composable_p`, `inhibit_auto_composition`,
// `compare_composition_rules`), and the four static objects (`composition_table_size`,
// `composition_hash_table`, `gstring_hash_table`, `gstring_work`, `gstring_work_headers`)
// are unexported `static mut`s.  `composition_table` and `n_compositions` are C globals:
// `#[unsafe(no_mangle)] pub static mut`.
//
// **The dead region is recorded in `docs/ports/composite.md`, not transcribed** (P-08):
// `src/composite.c` is in `docs/baseline/config/cond-eval-verified.txt` and
// `python3 tools/check-cond-eval.py src/composite.c` reports
// `1 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the frozen configuration's
// branch selection is the clang-verified one.  The only `#if` is C 959-970
// (`#ifdef HAVE_WINDOW_SYSTEM` around `autocmp_chars`'s `font_range` call), and
// `HAVE_WINDOW_SYSTEM` is undefined in the frozen `config.h` (macros.tsv 1647), so that
// block is not compiled and `font_range`, `FRAME_WINDOW_P` and `XFRAME` are not reached.
//
// C's uninitialised locals are zero-initialised (PORTING.md 1.5), each named in
// `docs/ports/composite.md`; C 41-44's and the other forward declarations are dropped
// (PORTING.md 0.1-12).  Names no mirror carries are declared at their use site (the
// terminal.rs/filelock.rs shape), each with C's own prototype above it.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut)]

include!("include/config.rs");                 // C 22: `#include <config.h>`
include!("../lib/include/stdlib.rs");          // C 24: `#include <stdlib.h>` /* for qsort */
include!("include/lisp.rs");                   // C 26: `#include "lisp.h"`
include!("include/character.rs");              // C 27: `#include "character.h"`
include!("include/font.rs");                   // composite.h 28: `#include "font.h"`
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>` (strcmp)
include!("include/itree.rs");                 // buffer.h 28: `#include "itree.h"`
include!("include/composite.rs");              // C 28: `#include "composite.h"`
include!("include/buffer.rs");                 // C 29: `#include "buffer.h"`
include!("include/coding.rs");                 // C 30: `#include "coding.h"`
include!("include/window.rs");                 // buffer.h 29/frame.h 23: `#include "window.h"`
include!("include/intervals.rs");              // C 31: `#include "intervals.h"`
include!("include/frame.rs");                  // C 32: `#include "frame.h"`
include!("include/dispextern.rs");             // C 33: `#include "dispextern.h"`
include!("include/systime.rs");                // termhooks.h 28: `#include "systime.h"`
include!("include/termhooks.rs");              // C 34: `#include "termhooks.h"`

/* The names C gets from headers whose mirrors do not carry them yet (the
   terminal.rs/filelock.rs shape), each with C's own prototype in a `/// C:` line.  */
unsafe extern "C" {
  /// C: lisp.h 4998: `extern void record_unwind_save_match_data (void);`
  fn record_unwind_save_match_data ();
  /// C: lisp.h 4978: `extern void restore_point_unwind (Lisp_Object);`
  fn restore_point_unwind (location: Lisp_Object);
  /// C: lisp.h 5031-5032: `extern ptrdiff_t fast_looking_at (Lisp_Object, ptrdiff_t,
  /// ptrdiff_t, ptrdiff_t, ptrdiff_t, Lisp_Object);`
  fn fast_looking_at (regexp: Lisp_Object, pos: ptrdiff_t, pos_byte: ptrdiff_t,
		      limit: ptrdiff_t, limit_byte: ptrdiff_t, string: Lisp_Object)
		      -> ptrdiff_t;
  /// C: lisp.h 5035-5036: `extern ptrdiff_t find_newline (ptrdiff_t, ptrdiff_t, ptrdiff_t,
  /// ptrdiff_t, ptrdiff_t, ptrdiff_t *, ptrdiff_t *, bool);`
  fn find_newline (start: ptrdiff_t, start_byte: ptrdiff_t, end: ptrdiff_t,
		   end_byte: ptrdiff_t, count: ptrdiff_t, shortage: *mut ptrdiff_t,
		   shortage_byte: *mut ptrdiff_t, allow_quit: bool) -> ptrdiff_t;
  /// C: lisp.h 5375: `extern char * tty_type_name (Lisp_Object);`
  fn tty_type_name (terminal: Lisp_Object) -> *mut c_char;
  /// C: termhooks.h 961: `extern struct terminal *decode_live_terminal (Lisp_Object);`
  /// (terminal.c defines it and the termhooks.h mirror does not carry the prototype yet.)
  fn decode_live_terminal (terminal: Lisp_Object) -> *mut terminal;
  /// C: lisp.h 4436: `extern void mark_object (Lisp_Object);`
  fn mark_object (obj: Lisp_Object);
  /// C: lisp.h 4259: `void hash_remove_from_table (struct Lisp_Hash_Table *, Lisp_Object);`
  fn hash_remove_from_table (h: *mut Lisp_Hash_Table, key: Lisp_Object);
  // (window.h 1233's `window_point` is the window.rs mirror's now; the use-site copy is gone, P-26.)
  /// C: dispextern.h 3592: `ptrdiff_t get_small_narrowing_begv (struct window *, ptrdiff_t);`
  fn get_small_narrowing_begv (w: *mut window, pos: ptrdiff_t) -> ptrdiff_t;
  /// C: <stdlib.h>: `void qsort (void *base, size_t nmemb, size_t size,
  /// int (*compar) (const void *, const void *));` (`lib/include/stdlib.rs` does not
  /// carry it yet; reported as a mirror gap in docs/ports/composite.md).
  fn qsort (base: *mut c_void, nmemb: usize, size: usize,
	    compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);
  /// C: font.h 911: `extern void font_fill_lglyph_metrics (Lisp_Object, struct font *,
  /// unsigned int);` — `struct font` is declared below (font.h is not mirrored that far,
  /// PORTING.md P-66).
  fn font_fill_lglyph_metrics (glyph: Lisp_Object, font: *mut font, code: c_uint);
  /// C: dispextern.h 3631: `extern void get_font_ascent_descent (struct font *, int *,
  /// int *);`
  fn get_font_ascent_descent (font: *mut font, ascent: *mut c_int, descent: *mut c_int);
  /// C: lisp.h 3525: `DEFVAR_BOOL ("inhibit-modification-hooks",
  /// inhibit_modification_hooks, …)`.  The storage lives in `insdel.c`, its definition
  /// unit (`femacs/src/insdel.rs`); no mirror declares the name (status §3.29).
  pub static mut inhibit_modification_hooks: bool;
}

/* C: font.h 414-417's `struct font_metrics` is the font mirror's now
   (`include/font.rs` 293-302, status 3.225): the record landed there with its measured layout,
   and this unit includes that mirror at C's position (composite.h 28), so a second copy here is
   `E0428` (the face-record move of 3.211/3.220).  */

/* C: font.h 277-404 `struct font` and 589-821 `struct font_driver`.  Both now come from
   `include/font.rs` (this unit includes that mirror at C's position, composite.h 28).  The
   byte-padded `font_driver` prefix this unit carried for `driver->encode_char`
   (`offsetof (struct font_driver, encode_char)` = 96, measured) **retired here** in 3.238
   when the mirror grew the whole record: a second definition is `E0428`, the same ownership
   move as `XFONT_OBJECT` (3.225) and the face records (3.211/3.220).  The mirror's
   `font.driver` stays untyped and the read below casts (PORTING.md P-72 rule 3);
   `encode_char` sits at the mirror's measured offset, so the cast resolves to it.  */

/* C: font.h 537-544's `XFONT_OBJECT` is the font mirror's now (font.rs 415-418, status 3.225):
   this unit includes that mirror at C's position (composite.h 28), so a second copy here is
   `E0428` - the same ownership move as the face records (3.211/3.220).  */

/* C: dispextern.h 2386-2420 `struct composition_it`, in C's field order.  dispextern.h is
   not mirrored this far (PORTING.md P-66); the type is declared here and reported as a
   mirror gap.  */
#[repr(C)]
pub struct composition_it
{
  /* Next position at which to check the composition.  */
  pub stop_pos: ptrdiff_t,
  /* ID number of the composition or glyph-string.  */
  pub id: ptrdiff_t,
  /* Character that triggers the automatic composition at `stop_pos'.  */
  pub ch: c_int,
  /* Index of a rule for making the automatic composition.  */
  pub rule_idx: EMACS_INT,
  /* How many characters to look back.  */
  pub lookback: ptrdiff_t,
  /* Number of glyphs of the glyph-string.  */
  pub nglyphs: c_int,
  /* Composition created while the buffer is scanned in reverse order.  */
  pub reversed_p: bool,
  /* Parent iterator.  */
  pub parent_it: *mut it,
  /* Position of the first character of the current grapheme cluster.  */
  pub charpos: ptrdiff_t,
  /* Number of characters and bytes of the current grapheme cluster.  */
  pub nchars: c_int,
  pub nbytes: c_int,
  /* Indices of the glyphs for the current grapheme cluster.  */
  pub from: c_int,
  pub to: c_int,
  /* Width of the current grapheme cluster in columns.  */
  pub width: c_int,
}

/* C: dispextern.h 2430-2437 `struct it`.  Only the first three members are reached (the
   `parent_it->w` and `parent_it->f` reads), and their offsets are C's 0/8/16; the rest of
   the record lands with xdisp.c (PORTING.md P-66).  */
#[repr(C)]
pub struct it
{
  /* The window in which we iterate over current_buffer (or a string).  */
  pub window: Lisp_Object,
  pub w: *mut window,
  /* The window's frame.  */
  pub f: *mut frame,
}

/* font.h 585's `FONT_INVALID_CODE` is the font mirror's now - this unit reaches it through
   the font header's position in composite.h (28), which is in this file's include list. */

/* Table of pointers to the structure `composition' indexed by
   COMPOSITION-ID.  This structure is for storing information about
   each composition except for COMPONENTS-VEC.  */
#[unsafe(no_mangle)]
pub static mut composition_table: *mut *mut composition = std::ptr::null_mut ();

/* The current size of `composition_table'.  */
static mut composition_table_size: ptrdiff_t = 0;

/* Number of compositions currently made. */
#[unsafe(no_mangle)]
pub static mut n_compositions: ptrdiff_t = 0;

/* Hash table for compositions.  The key is COMPONENTS-VEC of
   `composition' property.  The value is the corresponding
   COMPOSITION-ID.  */
static mut composition_hash_table: Lisp_Object = lisp_h_Qnil ();

/* Maximum number of characters to look back for
   auto-compositions.  */
pub const MAX_AUTO_COMPOSITION_LOOKBACK: c_int = 3;

/* Return COMPOSITION-ID of a composition at buffer position
   CHARPOS/BYTEPOS and length NCHARS.  The `composition' property of
   the sequence is PROP.  STRING, if non-nil, is a string that
   contains the composition instead of the current buffer.

   If the composition is invalid, return -1.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_composition_id (mut charpos: ptrdiff_t, mut bytepos: ptrdiff_t,
					     nchars: ptrdiff_t, prop: Lisp_Object,
					     string: Lisp_Object) -> ptrdiff_t
{
  let mut id: Lisp_Object = lisp_h_Qnil ();
  let mut length: Lisp_Object = lisp_h_Qnil ();
  let mut components: Lisp_Object = lisp_h_Qnil ();
  let mut key: Lisp_Object = lisp_h_Qnil ();
  let mut key_contents: *mut Lisp_Object = std::ptr::null_mut ();
  let mut glyph_len: ptrdiff_t = 0;
  let hash_table: *mut Lisp_Hash_Table = XHASH_TABLE (composition_hash_table);
  let mut hash_index: ptrdiff_t = 0;
  let mut method: composition_method = COMPOSITION_RELATIVE;
  let mut cmp: *mut composition = std::ptr::null_mut ();
  let mut ch: c_int = 0;

  /* Maximum length of a string of glyphs.  XftGlyphExtents limits
     this to INT_MAX, and Emacs limits it further.  Divide INT_MAX - 1
     by 2 because gui_produce_glyphs computes glyph_len * 2 + 1.  Divide
     the size by MAX_MULTIBYTE_LENGTH because encode_terminal_code
     multiplies glyph_len by MAX_MULTIBYTE_LENGTH.  */
  let GLYPH_LEN_MAX: ptrdiff_t =
    min (((INT_MAX - 1) / 2) as isize,
	 (core::cmp::min (PTRDIFF_MAX as usize, SIZE_MAX)
	  / MAX_MULTIBYTE_LENGTH as usize) as isize);

  /* PROP should be
	Form-A: ((LENGTH . COMPONENTS) . MODIFICATION-FUNC)
     or
	Form-B: (COMPOSITION-ID . (LENGTH COMPONENTS-VEC . MODIFICATION-FUNC))
  */
  'invalid_composition: loop
  {
    if nchars == 0 || !CONSP (prop)
      { break 'invalid_composition; }

    id = XCAR (prop);
    if FIXNUMP (id)
      {
	/* PROP should be Form-B.  */
	if XFIXNUM (id) < 0 || XFIXNUM (id) >= n_compositions
	  { break 'invalid_composition; }
	return XFIXNUM (id);
      }

    /* PROP should be Form-A.
       Thus, ID should be (LENGTH . COMPONENTS).  */
    if !CONSP (id)
      { break 'invalid_composition; }
    length = XCAR (id);
    if !FIXNUMP (length) || XFIXNUM (length) != nchars
      { break 'invalid_composition; }

    components = XCDR (id);

    /* Check if the same composition has already been registered or not
       by consulting composition_hash_table.  The key for this table is
       COMPONENTS (converted to a vector COMPONENTS-VEC) or, if it is
       nil, vector of characters in the composition range.  */
    if FIXNUMP (components)
      { key = make_vector (1, components); }
    else if STRINGP (components) || CONSP (components)
      { key = Fvconcat (1, &raw mut components); }
    else if VECTORP (components)
      { key = components; }
    else if NILP (components)
      {
	key = make_uninit_vector (nchars);
	if STRINGP (string)
	  { let mut i: ptrdiff_t = 0;
	    while i < nchars
	      {
		ch = fetch_string_char_advance (string, &raw mut charpos,
						&raw mut bytepos);
		ASET (key, i, make_fixnum (ch as EMACS_INT));
		i += 1;
	      } }
	else
	  { let mut i: ptrdiff_t = 0;
	    while i < nchars
	      {
		ch = fetch_char_advance (&raw mut charpos, &raw mut bytepos);
		ASET (key, i, make_fixnum (ch as EMACS_INT));
		i += 1;
	      } }
      }
    else
      { break 'invalid_composition; }

    let mut hash_code: hash_hash_t = 0;
    hash_index = hash_find_get_hash (hash_table, key, &raw mut hash_code);
    if hash_index >= 0
      {
	/* We have already registered the same composition.  Change PROP
	   from Form-A above to Form-B while replacing COMPONENTS with
	   COMPONENTS-VEC stored in the hash table.  We can directly
	   modify the cons cell of PROP because it is not shared.  */
	key = HASH_KEY (hash_table, hash_index as hash_idx_t);
	id = HASH_VALUE (hash_table, hash_index as hash_idx_t);
	XSETCAR (prop, id);
	XSETCDR (prop, Fcons (make_fixnum (nchars), Fcons (key, XCDR (prop))));
	return XFIXNUM (id);
      }

    /* This composition is a new one.  We must register it.  */

    /* Check if we have sufficient memory to store this information.  */
    if composition_table_size <= n_compositions
      { composition_table
	  = xpalloc (composition_table as *mut c_void, &raw mut composition_table_size,
		     1, -1, std::mem::size_of::<*mut composition> () as ptrdiff_t)
	    as *mut *mut composition; }

    key_contents = fam_base (core::ptr::addr_of_mut! ((*XVECTOR (key)).contents));

    /* Check if the contents of COMPONENTS are valid if COMPONENTS is a
       vector or a list.  It should be a sequence of:
	char1 rule1 char2 rule2 char3 ...    ruleN charN+1  */

    if VECTORP (components)
	&& ASIZE (components) >= 2
	&& VECTORP (AREF (components, 0))
      {
	/* COMPONENTS is a glyph-string.  */
	let len: ptrdiff_t = ASIZE (key);

	let mut i: ptrdiff_t = 1;
	while i < len
	  {
	    if ! VECTORP (AREF (key, i))
	      { break 'invalid_composition; }
	    i += 1;
	  }
      }
    else if VECTORP (components) || CONSP (components)
      {
	let len: ptrdiff_t = ASIZE (key);

	/* The number of elements should be odd.  */
	if (len % 2) == 0
	  { break 'invalid_composition; }
	/* All elements should be integers (character or encoded
           composition rule).  */
	let mut i: ptrdiff_t = 0;
	while i < len
	  {
	    if !FIXNUMP (*key_contents.offset (i as isize))
	      { break 'invalid_composition; }
	    i += 1;
	  }
      }

    /* Change PROP from Form-A above to Form-B.  We can directly modify
       the cons cell of PROP because it is not shared.  */
    XSETFASTINT! (id, n_compositions);
    XSETCAR (prop, id);
    XSETCDR (prop, Fcons (make_fixnum (nchars), Fcons (key, XCDR (prop))));

    /* Register the composition in composition_hash_table.  */
    hash_index = hash_put (hash_table, key, id, hash_code);

    method = (if NILP (components)
	      { COMPOSITION_RELATIVE }
	      else if (FIXNUMP (components) || STRINGP (components))
	      { COMPOSITION_WITH_ALTCHARS }
	      else { COMPOSITION_WITH_RULE_ALTCHARS });

    glyph_len = (if method == COMPOSITION_WITH_RULE_ALTCHARS
		 { (ASIZE (key) + 1) / 2 }
		 else { ASIZE (key) });

    if GLYPH_LEN_MAX < glyph_len
      { memory_full_up (); }

    /* Register the composition in composition_table.  */
    cmp = xmalloc (std::mem::size_of::<composition> ()) as *mut composition;

    (*cmp).method = method;
    (*cmp).key = key;
    (*cmp).glyph_len = glyph_len as c_int;
    (*cmp).offsets = xnmalloc (glyph_len, 2 * std::mem::size_of::<c_short> () as isize)
		     as *mut c_short;
    (*cmp).font = std::ptr::null_mut ();

    if (*cmp).method != COMPOSITION_WITH_RULE_ALTCHARS
      {
	/* Relative composition.  */
	(*cmp).width = 0;
	let mut i: ptrdiff_t = 0;
	while i < glyph_len
	  {
	    let mut this_width: c_int = 0;
	    ch = XFIXNUM (*key_contents.offset (i as isize)) as c_int;
	    /* TAB in a composition means display glyphs with padding
	       space on the left or right.  */
	    this_width = (if ch == '\t' as c_int { 1 } else { CHARACTER_WIDTH (ch) });
	    if (*cmp).width < this_width
	      { (*cmp).width = this_width; }
	    i += 1;
	  }
      }
    else
      {
	/* Rule-base composition.  */
	let mut leftmost: c_double = 0.0;
	let mut rightmost: c_double = 0.0;

	ch = XFIXNUM (*key_contents.offset (0)) as c_int;
	rightmost = if ch != '\t' as c_int { CHARACTER_WIDTH (ch) as f64 } else { 1.0 };

	let mut i: ptrdiff_t = 1;
	while i < glyph_len
	  {
	    let mut rule: c_int = 0;
	    let mut gref: c_int = 0;
	    let mut nref: c_int = 0;
	    let mut this_width: c_int = 0;
	    let mut this_left: c_double = 0.0;

	    rule = XFIXNUM (*key_contents.offset (i as isize)) as c_int;
	    ch = XFIXNUM (*key_contents.offset ((i + 1) as isize)) as c_int;
	    this_width = if ch != '\t' as c_int { CHARACTER_WIDTH (ch) } else { 1 };

	    /* A composition rule is specified by an integer value
	       that encodes global and new reference points (GREF and
	       NREF).  GREF and NREF are specified by numbers as
	       below:
		0---1---2 -- ascent
		|       |
		|       |
		|       |
		9--10--11 -- center
		|       |
	     ---3---4---5--- baseline
		|       |
		6---7---8 -- descent
	    */
	    COMPOSITION_DECODE_REFS (&raw mut rule, &raw mut gref, &raw mut nref);
	    this_left = ((leftmost
			  + (gref % 3) as f64 * (rightmost - leftmost) / 2.0
			  - (nref % 3) as f64 * this_width as f64 / 2.0));

	    if this_left < leftmost
	      { leftmost = this_left; }
	    if this_left + this_width as f64 > rightmost
	      { rightmost = this_left + this_width as f64; }
	    i += 2;
	  }

	(*cmp).width = (rightmost - leftmost) as c_int;
	if ((*cmp).width as f64) < (rightmost - leftmost)
	  /* To get a ceiling integer value.  */
	  { (*cmp).width += 1; }
      }

    *composition_table.offset (n_compositions as isize) = cmp;

    let old_n_compositions: ptrdiff_t = n_compositions;
    n_compositions += 1;
    return old_n_compositions;
  }

  /* invalid_composition: */
  /* Would it be better to remove this `composition' property?  */
  return -1;
}

/* Find a static composition at or nearest to position POS of OBJECT
   (buffer or string).

   OBJECT defaults to the current buffer.  If there's a composition at
   POS, set *START and *END to the start and end of the sequence,
   *PROP to the `composition' property, and return 1.

   If there's no composition at POS and LIMIT is negative, return 0.

   Otherwise, search for a composition forward (LIMIT > POS) or
   backward (LIMIT < POS).  In this case, LIMIT bounds the search.

   If a composition is found, set *START, *END, and *PROP as above,
   and return 1, else return 0.

   This doesn't check the validity of composition.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_composition (mut pos: ptrdiff_t, limit: ptrdiff_t,
					   start: *mut ptrdiff_t, end: *mut ptrdiff_t,
					   prop: *mut Lisp_Object, object: Lisp_Object)
					   -> bool
{
  let mut val: Lisp_Object = lisp_h_Qnil ();

  if get_property_and_range (pos, Qcomposition (), prop, start, end, object)
    { return true; }

  if limit < 0 || limit == pos
    { return false; }

  if limit > pos		/* search forward */
    {
      val = Fnext_single_property_change (make_fixnum (pos), Qcomposition (),
					  object, make_fixnum (limit));
      pos = XFIXNUM (val);
      if pos == limit
	{ return false; }
    }
  else				/* search backward */
    {
      if get_property_and_range (pos - 1, Qcomposition (), prop, start, end, object)
	{ return true; }
      val = Fprevious_single_property_change (make_fixnum (pos), Qcomposition (),
					      object, make_fixnum (limit));
      pos = XFIXNUM (val);
      if pos == limit
	{ return false; }
      pos -= 1;
    }
  get_property_and_range (pos, Qcomposition (), prop, start, end, object);
  return true;
}

/* Run a proper function to adjust the composition sitting between
   FROM and TO with property PROP.  */

unsafe fn run_composition_function (mut from: ptrdiff_t, mut to: ptrdiff_t,
				    mut prop: Lisp_Object)
{
  let func: Lisp_Object = COMPOSITION_MODIFICATION_FUNC (prop);
  let mut start: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;

  /* If an invalid composition precedes or follows, try to make them
     valid too.  */
  if from > BEGV! ()
      && find_composition (from - 1, -1, &raw mut start, &raw mut end, &raw mut prop,
			   Qnil ())
      && !composition_valid_p (start, end, prop)
    { from = start; }
  if to < ZV! ()
      && find_composition (to, -1, &raw mut start, &raw mut end, &raw mut prop, Qnil ())
      && !composition_valid_p (start, end, prop)
    { to = end; }
  if !NILP (Ffboundp (func))
    { calln! (func, make_fixnum (from), make_fixnum (to)); }
}

/* Make invalid compositions adjacent to or inside FROM and TO valid.
   CHECK_MASK is bitwise `or' of mask bits defined by macros
   CHECK_XXX (see the comment in composite.h).

   It also resets the text-property `auto-composed' to a proper region
   so that automatic character composition works correctly later while
   displaying the region.

   This function is called when a buffer text is changed.  If the
   change is deletion, FROM == TO.  Otherwise, FROM < TO.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_compositions (mut from: ptrdiff_t, mut to: ptrdiff_t,
					      check_mask: c_int)
{
  let mut prop: Lisp_Object = lisp_h_Qnil ();
  let mut start: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  /* The beginning and end of the region to set the property
     `auto-composed' to nil.  */
  let mut min_pos: ptrdiff_t = from;
  let mut max_pos: ptrdiff_t = to;

  if inhibit_modification_hooks
    { return; }

  /* If FROM and TO are not in a valid range, do nothing.  */
  if ! (BEGV! () <= from && from <= to && to <= ZV! ())
    { return; }

  if (check_mask & CHECK_HEAD) != 0
    {
      /* FROM should be at composition boundary.  But, insertion or
	 deletion will make two compositions adjacent and
	 indistinguishable when they have same (eq) property.  To
	 avoid it, in such a case, we change the property of the
	 latter to the copy of it.  */
      if from > BEGV! ()
	  && find_composition (from - 1, -1, &raw mut start, &raw mut end, &raw mut prop,
			       Qnil ())
	  && composition_valid_p (start, end, prop)
	{
	  min_pos = start;
	  if end > to
	    { max_pos = end; }
	  if from < end
	    { Fput_text_property (make_fixnum (from), make_fixnum (end),
				  Qcomposition (),
				  Fcons (XCAR (prop), XCDR (prop)), Qnil ()); }
	  run_composition_function (start, end, prop);
	  from = end;
	}
      else if from < ZV! ()
	       && find_composition (from, -1, &raw mut start, &raw mut from, &raw mut prop,
				    Qnil ())
	       && composition_valid_p (start, from, prop)
	{
	  if from > to
	    { max_pos = from; }
	  run_composition_function (start, from, prop);
	}
    }

  if (check_mask & CHECK_INSIDE) != 0
    {
      /* In this case, we are sure that (check & CHECK_TAIL) is also
         nonzero.  Thus, here we should check only compositions before
         (to - 1).  */
      while from < to - 1
	    && find_composition (from, to, &raw mut start, &raw mut from, &raw mut prop,
				 Qnil ())
	    && composition_valid_p (start, from, prop)
	    && from < to - 1
	{ run_composition_function (start, from, prop); }
    }

  if (check_mask & CHECK_TAIL) != 0
    {
      if from < to
	  && find_composition (to - 1, -1, &raw mut start, &raw mut end, &raw mut prop,
			       Qnil ())
	  && composition_valid_p (start, end, prop)
	{
	  /* TO should be also at composition boundary.  But,
	     insertion or deletion will make two compositions adjacent
	     and indistinguishable when they have same (eq) property.
	     To avoid it, in such a case, we change the property of
	     the former to the copy of it.  */
	  if to < end
	    {
	      Fput_text_property (make_fixnum (start), make_fixnum (to),
				  Qcomposition (),
				  Fcons (XCAR (prop), XCDR (prop)), Qnil ());
	      max_pos = end;
	    }
	  run_composition_function (start, end, prop);
	}
      else if to < ZV! ()
	       && find_composition (to, -1, &raw mut start, &raw mut end, &raw mut prop,
				    Qnil ())
	       && composition_valid_p (start, end, prop)
	{
	  run_composition_function (start, end, prop);
	  max_pos = end;
	}
    }
  if min_pos < max_pos
    {
      let count: specpdl_ref = SPECPDL_INDEX ();

      specbind (Qinhibit_read_only (), Qt ());
      specbind (Qinhibit_modification_hooks (), Qt ());
      specbind (Qinhibit_point_motion_hooks (), Qt ());
      Fremove_list_of_text_properties (make_fixnum (min_pos),
				       make_fixnum (max_pos),
				       list1 (Qauto_composed ()), Qnil ());
      unbind_to (count, Qnil ());
    }
}

/* Modify composition property values in LIST destructively.  LIST is
   a list as returned from text_property_list.  Change values to the
   top-level copies of them so that none of them are `eq'.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_composition_value_copy (mut list: Lisp_Object)
{
  let mut plist: Lisp_Object = lisp_h_Qnil ();
  let mut val: Lisp_Object = lisp_h_Qnil ();

  while CONSP (list)
    {
      plist = XCAR (XCDR (XCDR (XCAR (list))));
      while CONSP (plist) && CONSP (XCDR (plist))
	{
	  if EQ (XCAR (plist), Qcomposition ())
	      && { val = XCAR (XCDR (plist)); CONSP (val) }
	    { XSETCAR (XCDR (plist), Fcons (XCAR (val), XCDR (val))); }
	  plist = XCDR (XCDR (plist));
	}
      list = XCDR (list);
    }
}

/* Make text in the region between START and END a composition that
   has COMPONENTS and MODIFICATION-FUNC.

   If STRING is non-nil, then operate on characters contained between
   indices START and END in STRING.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compose_text (start: ptrdiff_t, end: ptrdiff_t,
				       components: Lisp_Object,
				       modification_func: Lisp_Object,
				       string: Lisp_Object)
{
  let prop: Lisp_Object = Fcons (Fcons (make_fixnum (end - start), components),
				 modification_func);
  Fput_text_property (make_fixnum (start), make_fixnum (end),
		      Qcomposition (), prop, string);
}

/* Lisp glyph-string handlers.  */

/* Hash table for automatic composition.  The key is a header of a
   lgstring (Lispy glyph-string), and the value is a body of a
   lgstring.  */

static mut gstring_hash_table: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_gstring_lookup_cache (header: Lisp_Object) -> Lisp_Object
{
  return Fgethash (header, gstring_hash_table, Qnil ());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_gstring_put_cache (gstring: Lisp_Object,
							mut len: ptrdiff_t) -> Lisp_Object
{
  let h: *mut Lisp_Hash_Table = XHASH_TABLE (gstring_hash_table);
  let header: Lisp_Object = LGSTRING_HEADER (gstring);
  let hash: EMACS_UINT = hash_from_key (h, header) as EMACS_UINT;
  if len < 0
    {
      let glyph_len: ptrdiff_t = LGSTRING_GLYPH_LEN (gstring);
      len = 0;
      while len < glyph_len
	{
	  if NILP (LGSTRING_GLYPH (gstring, len))
	    { break; }
	  len += 1;
	}
    }

  let copy: Lisp_Object = make_nil_vector (len + 2);
  LGSTRING_SET_HEADER (copy, Fcopy_sequence (header));
  let mut i: ptrdiff_t = 0;
  while i < len
    {
      LGSTRING_SET_GLYPH (copy, i, Fcopy_sequence (LGSTRING_GLYPH (gstring, i)));
      i += 1;
    }
  let id: ptrdiff_t = hash_put (h, LGSTRING_HEADER (copy), copy, hash as hash_hash_t);
  LGSTRING_SET_ID (copy, make_fixnum (id));
  return copy;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_gstring_from_id (id: ptrdiff_t) -> Lisp_Object
{
  let h: *mut Lisp_Hash_Table = XHASH_TABLE (gstring_hash_table);
  /* FIXME: The stability of this value depends on the hash table internals!  */
  return HASH_VALUE (h, id as hash_idx_t);
}

/* Remove from the composition hash table every lgstring that
   references the given FONT_OBJECT.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_gstring_cache_clear_font (font_object: Lisp_Object)
{
  let h: *mut Lisp_Hash_Table = XHASH_TABLE (gstring_hash_table);

  DOHASH! (h, k, gstring, {
    if EQ (LGSTRING_FONT (gstring), font_object)
      { hash_remove_from_table (h, k); }
  });
}

DEFUN! ("clear-composition-cache", Fclear_composition_cache,
       Sclear_composition_cache, 0, 0, 0,
       doc: "Internal use only.\nClear composition cache.",
       (),
       {
  gstring_hash_table = CALLN! (Fmake_hash_table, QCtest (), Qequal (),
			       QCsize (), make_fixnum (311));
  /* Fixme: We call Fclear_face_cache to force complete re-building of
     display glyphs.  But, it may be better to call this function from
     Fclear_face_cache instead.  */
  return Fclear_face_cache (Qt ());
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_gstring_p (gstring: Lisp_Object) -> bool
{
  let mut header: Lisp_Object = lisp_h_Qnil ();
  let mut i: ptrdiff_t = 0;

  if ! VECTORP (gstring) || ASIZE (gstring) < 2
    { return false; }
  header = LGSTRING_HEADER (gstring);
  if ! VECTORP (header) || ASIZE (header) < 2
    { return false; }
  if ! NILP (LGSTRING_FONT (gstring))
      && (! FONT_OBJECT_P (LGSTRING_FONT (gstring))
	  && ! CODING_SYSTEM_P (LGSTRING_FONT (gstring)))
    { return false; }
  i = 1;
  while i < ASIZE (LGSTRING_HEADER (gstring))
    {
      if ! FIXNATP (AREF (LGSTRING_HEADER (gstring), i))
	{ return false; }
      i += 1;
    }
  if ! NILP (LGSTRING_ID (gstring)) && ! FIXNATP (LGSTRING_ID (gstring))
    { return false; }
  i = 0;
  while i < LGSTRING_GLYPH_LEN (gstring)
    {
      let glyph: Lisp_Object = LGSTRING_GLYPH (gstring, i);
      if NILP (glyph)
	{ break; }
      if ! VECTORP (glyph) || ASIZE (glyph) != LGLYPH_SIZE
	{ return false; }
      i += 1;
    }
  return true;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_gstring_width (gstring: Lisp_Object, mut from: ptrdiff_t,
						    to: ptrdiff_t,
						    metrics: *mut font_metrics) -> c_int
{
  let mut glyph: *mut Lisp_Object = std::ptr::null_mut ();
  let mut width: c_int = 0;

  if !metrics.is_null ()
    {
      let font_object: Lisp_Object = LGSTRING_FONT (gstring);

      if FONT_OBJECT_P (font_object)
	{
	  let font: *mut font = XFONT_OBJECT (font_object);
	  let mut font_ascent: c_int = 0;
	  let mut font_descent: c_int = 0;

	  get_font_ascent_descent (font, &raw mut font_ascent, &raw mut font_descent);
	  (*metrics).ascent = font_ascent as c_short;
	  (*metrics).descent = font_descent as c_short;
	}
      else
	{
	  (*metrics).ascent = 1;
	  (*metrics).descent = 0;
	}
      (*metrics).rbearing = 0;
      (*metrics).lbearing = 0;
      (*metrics).width = 0;
    }
  glyph = lgstring_glyph_addr (gstring, from);
  while from < to
    {
      let mut x: c_int = 0;

      if NILP (LGLYPH_ADJUSTMENT (*glyph))
	{ width += LGLYPH_WIDTH (*glyph) as c_int; }
      else
	{ width += LGLYPH_WADJUST (*glyph) as c_int; }
      if !metrics.is_null ()
	{
	  x = ((*metrics).width as EMACS_INT + LGLYPH_LBEARING (*glyph)
	       + LGLYPH_XOFF (*glyph)) as c_int;
	  if ((*metrics).lbearing as c_int) > x
	    { (*metrics).lbearing = x as c_short; }
	  x = ((*metrics).width as EMACS_INT + LGLYPH_RBEARING (*glyph)
	       + LGLYPH_XOFF (*glyph)) as c_int;
	  if ((*metrics).rbearing as c_int) < x
	    { (*metrics).rbearing = x as c_short; }
	  (*metrics).width = width as c_short;
	  x = (LGLYPH_ASCENT (*glyph) - LGLYPH_YOFF (*glyph)) as c_int;
	  if ((*metrics).ascent as c_int) < x
	    { (*metrics).ascent = x as c_short; }
	  x = (LGLYPH_DESCENT (*glyph) + LGLYPH_YOFF (*glyph)) as c_int;
	  if ((*metrics).descent as c_int) < x
	    { (*metrics).descent = x as c_short; }
	}
      from += 1;
      glyph = glyph.offset (1);
    }
  return width;
}

/* Adjust the width of each grapheme cluster of GSTRING because
   zero-width grapheme clusters are not displayed.  If the width is
   zero, then the width of the last glyph in the cluster is
   incremented.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_gstring_adjust_zero_width (gstring: Lisp_Object)
{
  let mut from: ptrdiff_t = 0;
  let mut width: c_int = 0;

  let mut i: ptrdiff_t = 0;
  loop
    {
      let mut glyph: Lisp_Object = lisp_h_Qnil ();

      if i < LGSTRING_GLYPH_LEN (gstring)
	{ glyph = LGSTRING_GLYPH (gstring, i); }
      else
	{ glyph = Qnil (); }

      if NILP (glyph) || from != LGLYPH_FROM (glyph)
	{
	  eassert! (i > 0);
	  let last: Lisp_Object = LGSTRING_GLYPH (gstring, i - 1);

	  if width == 0
	    {
	      if NILP (LGLYPH_ADJUSTMENT (last))
		{ LGLYPH_SET_ADJUSTMENT (last,
					  CALLN! (Fvector,
						  make_fixnum (0), make_fixnum (0),
						  make_fixnum (LGLYPH_WIDTH (last)
							       + 1))); }
	      else
		{ ASET (LGLYPH_ADJUSTMENT (last), 2,
			make_fixnum (LGLYPH_WADJUST (last) + 1)); }
	    }
	  if NILP (glyph)
	    { break; }
	  from = LGLYPH_FROM (glyph);
	  width = 0;
	}
      width += (if NILP (LGLYPH_ADJUSTMENT (glyph))
		{ LGLYPH_WIDTH (glyph) as c_int } else { LGLYPH_WADJUST (glyph) as c_int });
      i += 1;
    }
}


static mut gstring_work: Lisp_Object = lisp_h_Qnil ();
static mut gstring_work_headers: Lisp_Object = lisp_h_Qnil ();

unsafe fn fill_gstring_header (mut from: ptrdiff_t, mut from_byte: ptrdiff_t,
			       to: ptrdiff_t, font_object: Lisp_Object,
			       string: Lisp_Object) -> Lisp_Object
{
  let len: ptrdiff_t = to - from;
  if len == 0
    { error (c"Attempt to shape zero-length text".as_ptr ()); }
  eassume! (0 < len);
  let header: Lisp_Object = (if len <= 8
			     { AREF (gstring_work_headers, len - 1) }
			     else { make_uninit_vector (len + 1) });

  ASET (header, 0, font_object);
  let mut i: ptrdiff_t = 0;
  while i < len
    {
      let c: c_int
	= (if NILP (string)
	   { fetch_char_advance_no_check (&raw mut from, &raw mut from_byte) }
	   else { fetch_string_char_advance_no_check (string, &raw mut from,
						      &raw mut from_byte) });
      ASET (header, i + 1, make_fixnum (c as EMACS_INT));
      i += 1;
    }
  return header;
}

unsafe fn fill_gstring_body (gstring: Lisp_Object)
{
  let font_object: Lisp_Object = LGSTRING_FONT (gstring);
  let header: Lisp_Object = AREF (gstring, 0);
  let mut len: ptrdiff_t = LGSTRING_CHAR_LEN (gstring);
  let mut i: ptrdiff_t = 0;
  let mut font: *mut font = std::ptr::null_mut ();
  let mut code: c_uint = 0;

  if FONT_OBJECT_P (font_object)
    { font = XFONT_OBJECT (font_object); }

  while i < len
    {
      let mut g: Lisp_Object = LGSTRING_GLYPH (gstring, i);
      let c: c_int = XFIXNAT (AREF (header, i + 1)) as c_int;

      if NILP (g)
	{
	  g = LGLYPH_NEW ();
	  LGSTRING_SET_GLYPH (gstring, i, g);
	}
      LGLYPH_SET_FROM (g, i);
      LGLYPH_SET_TO (g, i);
      LGLYPH_SET_CHAR (g, c as EMACS_INT);

      if !font.is_null ()
	{ code = ((*( (*font).driver as *const font_driver )).encode_char.unwrap ()) (font, LGLYPH_CHAR (g) as c_int); }
      else
	{ code = FONT_INVALID_CODE; }
      if code != FONT_INVALID_CODE
	{ font_fill_lglyph_metrics (g, font, code); }
      else
	{
	  let width: c_int = XFIXNAT (CHAR_TABLE_REF (Vchar_width_table, c as isize)) as c_int;

	  LGLYPH_SET_CODE! (g, c as c_uint);
	  LGLYPH_SET_LBEARING (g, 0);
	  LGLYPH_SET_RBEARING (g, width as EMACS_INT);
	  LGLYPH_SET_WIDTH (g, width as EMACS_INT);
	  LGLYPH_SET_ASCENT (g, 1);
	  LGLYPH_SET_DESCENT (g, 0);
	}
      LGLYPH_SET_ADJUSTMENT (g, Qnil ());
      i += 1;
    }
  len = LGSTRING_GLYPH_LEN (gstring);
  while i < len
    { LGSTRING_SET_GLYPH (gstring, i, Qnil ()); i += 1; }
}


/* Try to compose the characters at CHARPOS according to composition
   rule RULE ([PATTERN PREV-CHARS FUNC]).  LIMIT limits the characters
   to compose.  STRING, if not nil, is a target string.  WIN is a
   window where the characters are being displayed.  CH is the
   character that triggered the composition check.  If characters are
   successfully composed, return the composition as a glyph-string
   object.  Otherwise return nil.  */

unsafe fn autocmp_chars (rule: Lisp_Object, charpos: ptrdiff_t, bytepos: ptrdiff_t,
			 mut limit: ptrdiff_t, win: *mut window, face: *mut c_void,
			 string: Lisp_Object, direction: Lisp_Object, ch: c_int)
			 -> Lisp_Object
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  let pos: Lisp_Object = make_fixnum (charpos);
  let mut to: ptrdiff_t = 0;
  let pt: ptrdiff_t = PT! ();
  let pt_byte: ptrdiff_t = PT_BYTE! ();
  let mut re: Lisp_Object = lisp_h_Qnil ();
  let mut font_object: Lisp_Object = lisp_h_Qnil ();
  let mut lgstring: Lisp_Object = lisp_h_Qnil ();
  let mut len: ptrdiff_t = 0;

  record_unwind_save_match_data ();
  re = AREF (rule, 0);
  if NILP (re)
    { len = 1; }
  else if ! STRINGP (re)
    { return unbind_to (count, Qnil ()); }
  else if ({ len = fast_looking_at (re, charpos, bytepos, limit, -1, string); len > 0 })
    {
      if NILP (string)
	{ len = BYTE_TO_CHAR (bytepos + len) - charpos; }
      else
	{ len = string_byte_to_char (string, bytepos + len) - charpos; }
    }
  if len <= 0
    { return unbind_to (count, Qnil ()); }
  limit = charpos + len;
  to = limit;
  font_object = (*win).frame;
  /* C 959-970's `#ifdef HAVE_WINDOW_SYSTEM` block (`struct frame *f = XFRAME
     (font_object); if (FRAME_WINDOW_P (f)) { font_object = font_range (…); … }`) is dead in
     the frozen configuration and recorded in docs/ports/composite.md.  */
  lgstring = Fcomposition_get_gstring (pos, make_fixnum (to), font_object,
				       string);
  if NILP (LGSTRING_ID (lgstring))
    {
      /* Save point as marker before calling out to lisp.  */
      if NILP (string)
	{ record_unwind_protect (Some (restore_point_unwind),
				 build_marker (current_buffer as *mut c_void, pt, pt_byte)); }
      lgstring = safe_calln! (Vauto_composition_function, AREF (rule, 2),
			      pos, make_fixnum (to), font_object, string,
			      direction);
    }
  return unbind_to (count, lgstring);
}

/* 1 iff the character C is composable.  Characters of general
   category Z? or C? are not composable except for ZWNJ and ZWJ,
   and characters of category Zs. */

unsafe fn char_composable_p (c: c_int) -> bool
{
  let mut val: Lisp_Object = lisp_h_Qnil ();
  return (c >= ' ' as c_int
	  && (c == ZERO_WIDTH_NON_JOINER || c == ZERO_WIDTH_JOINER
	      /* Per Unicode TR51, these tag characters can be part of
		 Emoji sequences.  */
	      || (TAG_SPACE <= c && c <= CANCEL_TAG)
	      /* unicode-category-table may not be available during
		 dumping.  */
	      || (CHAR_TABLE_P (Vunicode_category_table)
		  && { val = CHAR_TABLE_REF (Vunicode_category_table, c as isize);
		       (FIXNUMP (val)
			&& (XFIXNUM (val) <= unicode_category_t::UNICODE_CATEGORY_Zs as EMACS_INT)) })));
}

#[inline]
unsafe fn inhibit_auto_composition () -> bool
{
  if NILP (Vauto_composition_mode)
    { return true; }

  if STRINGP (Vauto_composition_mode)
    {
      let name: *mut c_char = tty_type_name (Qnil ());

      if !name.is_null () && strcmp (SSDATA (Vauto_composition_mode), name) == 0
	{ return true; }
    }

  return false;
}


/* Update cmp_it->stop_pos to the next position after CHARPOS (and
   BYTEPOS) where character composition may happen.  If BYTEPOS is
   negative, compute it.  ENDPOS is a limit of searching.  If it is
   less than CHARPOS, search backward to ENDPOS+1 assuming that
   set_iterator_to_next works in reverse order.  In this case, if a
   composition closest to CHARPOS is found, set cmp_it->stop_pos to
   the last character of the composition.  STRING, if non-nil, is
   the string (as opposed to a buffer) whose characters should be
   tested for being composable.  INCLUDE_STATIC non-zero means
   consider both static and automatic compositions; if zero, look
   only for potential automatic compositions.

   If no composition is found, set cmp_it->ch to -2.  If a static
   composition is found, set cmp_it->ch to -1.  Otherwise, set
   cmp_it->ch to the character that triggers the automatic
   composition.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_compute_stop_pos (cmp_it: *mut composition_it,
						       mut charpos: ptrdiff_t,
						       mut bytepos: ptrdiff_t,
						       mut endpos: ptrdiff_t,
						       string: Lisp_Object,
						       include_static: bool)
{
  let mut start: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  let mut c: c_int = 0;
  let mut prop: Lisp_Object = lisp_h_Qnil ();
  let mut val: Lisp_Object = lisp_h_Qnil ();
  /* This is from forward_to_next_line_start in xdisp.c.  */
  let MAX_NEWLINE_DISTANCE: c_int = 500;

  if charpos < endpos
    {
      if endpos > charpos + MAX_NEWLINE_DISTANCE as ptrdiff_t
	{ endpos = charpos + MAX_NEWLINE_DISTANCE as ptrdiff_t; }
    }
  else if endpos < charpos
    {
      /* We search backward for a position to check composition.  */
      if endpos < 0
	{
	  /* But we don't know where to stop the searching.  */
	  endpos = if NILP (string) { BEGV! () - 1 } else { -1 };
	  /* Usually we don't reach ENDPOS because we stop searching
	     at an uncomposable character (NL, LRE, etc).  In buffers
	     with long lines, however, NL might be far away, so
	     pretend that the buffer is smaller.  */
	  if (*current_buffer).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P != 0
	    { endpos = get_small_narrowing_begv ((*(*cmp_it).parent_it).w, charpos); }
	}
    }
  (*cmp_it).id = -1;
  (*cmp_it).ch = -2;
  (*cmp_it).reversed_p = false;
  (*cmp_it).stop_pos = endpos;
  if charpos == endpos
    { return; }
  /* Look for static compositions.  */
  /* FIXME: Bidi is not yet handled well in static composition.  */
  if include_static
      && charpos < endpos
      && find_composition (charpos, endpos, &raw mut start, &raw mut end, &raw mut prop,
			   string)
      && start >= charpos
      && composition_valid_p (start, end, prop)
    {
      endpos = start;
      (*cmp_it).stop_pos = endpos;
      (*cmp_it).ch = -1;
    }
  if (NILP (string)
      && NILP (BVAR! (current_buffer, enable_multibyte_characters_)))
      || (STRINGP (string) && ! STRING_MULTIBYTE (string))
      || inhibit_auto_composition ()
    { return; }
  if bytepos < 0
    {
      if NILP (string)
	{ bytepos = CHAR_TO_BYTE (charpos); }
      else
	{ bytepos = string_char_to_byte (string, charpos); }
    }

  /* Look for automatic compositions.  */
  start = charpos;
  /* For the case that they set it to something weird, like nil.  */
  if !CHAR_TABLE_P (Vcomposition_function_table)
    { Vcomposition_function_table = Fmake_char_table (Qnil (), Qnil ()); }
  if charpos < endpos
    {
      /* Forward search.  */
      while charpos < endpos
	{
	  c = (if STRINGP (string)
	       { fetch_string_char_advance (string, &raw mut charpos, &raw mut bytepos) }
	       else { fetch_char_advance (&raw mut charpos, &raw mut bytepos) });
	  if c == '\n' as c_int
	    {
	      (*cmp_it).ch = -2;
	      break;
	    }
	  val = CHAR_TABLE_REF (Vcomposition_function_table, c as isize);
	  if ! NILP (val)
	    {
	      let mut ridx: EMACS_INT = 0;
	      while CONSP (val)
		{
		  let elt: Lisp_Object = XCAR (val);
		  if VECTORP (elt) && ASIZE (elt) == 3
		      && FIXNATP (AREF (elt, 1))
		      && charpos - 1 - XFIXNAT (AREF (elt, 1)) >= start
		    {
		      (*cmp_it).rule_idx = ridx;
		      (*cmp_it).lookback = XFIXNAT (AREF (elt, 1));
		      (*cmp_it).stop_pos = charpos - 1 - (*cmp_it).lookback;
		      (*cmp_it).ch = c;
		      return;
		    }
		  val = XCDR (val);
		  ridx += 1;
		}
	    }
	}
      if charpos == endpos
	  && !(STRINGP (string) && endpos == SCHARS (string))
	{
	  /* We couldn't find a composition point before ENDPOS.  But,
	     some character after ENDPOS may be composed with
	     characters before ENDPOS.  So, we should stop at the safe
	     point.  */
	  charpos = endpos - MAX_AUTO_COMPOSITION_LOOKBACK as ptrdiff_t;
	  if charpos < start
	    { charpos = start; }
	}
    }
  else if charpos > endpos
    {
      /* Search backward for a pattern that may be composed such that
	 the position of (possibly) the last character of the match is
	 closest to (but not after) START.  The reason for the last
	 character is that set_iterator_to_next works in reverse
	 order, and thus we must stop at the last character for
	 composition check.  */
      let mut p: *mut u8 = std::ptr::null_mut ();
      let mut len: c_int = 0;
      /* Limit byte position used in fast_looking_at.  This is the
	 byte position of the character after START. */
      let mut limit: ptrdiff_t = 0;

      if NILP (string)
	{ p = BYTE_POS_ADDR (bytepos); }
      else
	{ p = SDATA (string).offset (bytepos); }
      c = string_char_and_length (p, &raw mut len);
      limit = bytepos + len as ptrdiff_t;
      /* The algorithmic idea behind the loop below is somewhat tricky
         and subtle.  ...  */
      while char_composable_p (c)
	{
	  val = CHAR_TABLE_REF (Vcomposition_function_table, c as isize);
	  let mut ridx: EMACS_INT = 0;
	  while CONSP (val)
	    {
	      let elt: Lisp_Object = XCAR (val);
	      if VECTORP (elt) && ASIZE (elt) == 3
		  && FIXNATP (AREF (elt, 1))
		  && charpos - XFIXNAT (AREF (elt, 1)) > endpos
		{
		  let mut back: ptrdiff_t = XFIXNAT (AREF (elt, 1));
		  let mut cpos: ptrdiff_t = charpos - back;
		  let mut bpos: ptrdiff_t = 0;

		  if back == 0
		    { bpos = bytepos; }
		  else
		    { bpos = (if NILP (string) { CHAR_TO_BYTE (cpos) }
			      else { string_char_to_byte (string, cpos) }); }
		  let mut blen: ptrdiff_t
		    = (if STRINGP (AREF (elt, 0))
		       { fast_looking_at (AREF (elt, 0), cpos, bpos,
					  start + 1, limit, string) }
		       else { 1 });
		  if blen > 0
		    {
		      /* Make CPOS point to the last character of
			 match.  Note that BLEN is byte-length.  */
		      if blen > 1
			{
			  bpos += blen;
			  if NILP (string)
			    { cpos = BYTE_TO_CHAR (bpos) - 1; }
			  else
			    { cpos = string_byte_to_char (string, bpos) - 1; }
			}
		      back = cpos - (charpos - back);
		      if (*cmp_it).stop_pos < cpos
			  || ((*cmp_it).stop_pos == cpos
			      && (*cmp_it).lookback < back)
			{
			  (*cmp_it).rule_idx = ridx;
			  (*cmp_it).stop_pos = cpos;
			  (*cmp_it).ch = c;
			  (*cmp_it).lookback = back;
			  (*cmp_it).nchars = back as c_int + 1;
			}
		    }
		}
	      val = XCDR (val);
	      ridx += 1;
	    }
	  if charpos - 1 == endpos
	    { break; }
	  if STRINGP (string)
	    {
	      p = p.offset (-1);
	      bytepos -= 1;
	      while ! CHAR_HEAD_P (*p as c_int)
		{
		  p = p.offset (-1);
		  bytepos -= 1;
		}
	      charpos -= 1;
	    }
	  else
	    {
	      dec_both (&raw mut charpos, &raw mut bytepos);
	      p = BYTE_POS_ADDR (bytepos);
	    }
	  c = STRING_CHAR (p);
	}
      if (*cmp_it).ch >= 0
	/* We found a position to check.  */
	{ return; }
      /* Skip all uncomposable characters.  */
      if NILP (string)
	{
	  while charpos - 1 > endpos && ! char_composable_p (c)
	    {
	      dec_both (&raw mut charpos, &raw mut bytepos);
	      c = FETCH_MULTIBYTE_CHAR (bytepos);
	    }
	}
      else
	{
	  while charpos - 1 > endpos && ! char_composable_p (c)
	    {
	      p = p.offset (-1);
	      while ! CHAR_HEAD_P (*p as c_int)
		{ p = p.offset (-1); }
	      charpos -= 1;
	      c = STRING_CHAR (p);
	    }
	}
    }
  (*cmp_it).stop_pos = charpos;
}

/* Check if the character at CHARPOS (and BYTEPOS) is composed
   (possibly with the following characters) on window W.  ENDPOS limits
   characters to be composed.  FACE, if non-NULL, is a base face of
   the character.  If STRING is not nil, it is a string containing the
   character to check, and CHARPOS and BYTEPOS are indices in the
   string.  In that case, FACE must not be NULL.  BIDI_LEVEL is the bidi
   embedding level of the current paragraph, and is used to calculate the
   direction argument to pass to the font shaper; value of -1 means the
   caller doesn't know the embedding level (used by callers which didn't
   invoke the display routines that perform bidi-display-reordering).

   If the character is composed, setup members of CMP_IT (id, nglyphs,
   from, to, reversed_p), and return true.  Otherwise, update
   CMP_IT->stop_pos, and return false.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_reseat_it (cmp_it: *mut composition_it,
						mut charpos: ptrdiff_t,
						mut bytepos: ptrdiff_t,
						mut endpos: ptrdiff_t, w: *mut window,
						bidi_level: c_char, face: *mut c_void,
						string: Lisp_Object) -> bool
{
  if (*cmp_it).ch == -2
    {
      composition_compute_stop_pos (cmp_it, charpos, bytepos, endpos, string,
				    true);
      if (*cmp_it).ch == -2 || (*cmp_it).stop_pos != charpos
	/* The current position is not composed.  */
	{ return false; }
    }

  if endpos < 0
    { endpos = if NILP (string) { BEGV! () } else { 0 }; }

  'no_composition: loop
  {
    if (*cmp_it).ch < 0
      {
	/* We are looking at a static composition.  */
	let mut start: ptrdiff_t = 0;
	let mut end: ptrdiff_t = 0;
	let mut prop: Lisp_Object = lisp_h_Qnil ();

	find_composition (charpos, -1, &raw mut start, &raw mut end, &raw mut prop, string);
	(*cmp_it).id = get_composition_id (charpos, bytepos, end - start,
					   prop, string);
	if (*cmp_it).id < 0
	  { break 'no_composition; }
	(*cmp_it).nchars = (end - start) as c_int;
	(*cmp_it).nglyphs = (**composition_table.add ((*cmp_it).id as usize)).glyph_len;
      }
    else if !w.is_null ()
      {
	let mut lgstring: Lisp_Object = Qnil ();
	let mut val: Lisp_Object = lisp_h_Qnil ();
	let mut elt: Lisp_Object = lisp_h_Qnil ();
	let mut direction: Lisp_Object = Qnil ();

	/* For the case that they set it to something weird, like nil.  */
	if !CHAR_TABLE_P (Vcomposition_function_table)
	  { Vcomposition_function_table = Fmake_char_table (Qnil (), Qnil ()); }
	val = CHAR_TABLE_REF (Vcomposition_function_table, (*cmp_it).ch as isize);
	let mut i: EMACS_INT = 0;
	while i < (*cmp_it).rule_idx
	  {
	    val = XCDR (val);
	    i += 1;
	  }
	if charpos < endpos
	  {
	    if bidi_level < 0
	      { direction = Qnil (); }
	    else if (bidi_level & 1) == 0
	      { direction = QL2R (); }
	    else
	      { direction = QR2L (); }
	    while CONSP (val)
	      {
		elt = XCAR (val);
		if ! VECTORP (elt) || ASIZE (elt) != 3
		    || ! FIXNUMP (AREF (elt, 1))
		  { val = XCDR (val); continue; }
		if XFIXNAT (AREF (elt, 1)) != (*cmp_it).lookback
		  { break 'no_composition; }
		lgstring = autocmp_chars (elt, charpos, bytepos, endpos,
					  w, face, string, direction, (*cmp_it).ch);
		if composition_gstring_p (lgstring)
		  { break; }
		lgstring = Qnil ();
		/* Composition failed perhaps because the font doesn't
		   support sufficient range of characters.  Try the
		   other composition rules if any.  */
		val = XCDR (val);
	      }
	    (*cmp_it).reversed_p = false;
	  }
	else
	  {
	    let mut cpos: ptrdiff_t = charpos;
	    let mut bpos: ptrdiff_t = bytepos;

	    (*cmp_it).reversed_p = true;
	    elt = XCAR (val);
	    if (*cmp_it).lookback > 0
	      {
		cpos = charpos - (*cmp_it).lookback;
		/* Reject the composition if it starts before ENDPOS,
		   which here can only happen if
		   composition-break-at-point is non-nil and point is
		   inside the composition.  */
		if cpos < endpos
		  {
		    eassert! (composition_break_at_point);
		    eassert! (endpos == PT! ());
		    break 'no_composition;
		  }
		if STRINGP (string)
		  { bpos = string_char_to_byte (string, cpos); }
		else
		  { bpos = CHAR_TO_BYTE (cpos); }
	      }
	    /* The bidi_level < 0 case below strictly speaking should
	       never happen, since we get here when bidi scan direction
	       is backward in the buffer, which can only happen if the
	       display routines were called to perform the bidi
	       reordering.  But it doesn't harm to test for that, and
	       avoid someone raising their brows and thinking it's a
	       subtle bug...  */
	    if bidi_level < 0
	      { direction = Qnil (); }
	    else if (bidi_level & 1) == 0
	      { direction = QL2R (); }
	    else
	      { direction = QR2L (); }
	    lgstring = autocmp_chars (elt, cpos, bpos, charpos + 1, w, face,
				      string, direction, (*cmp_it).ch);
	    if ! composition_gstring_p (lgstring)
		|| cpos + LGSTRING_CHAR_LEN (lgstring) as ptrdiff_t - 1 != charpos
	      /* Composition failed or didn't cover the current
		 character.  */
	      { break 'no_composition; }
	  }
	if NILP (lgstring)
	  { break 'no_composition; }
	if NILP (LGSTRING_ID (lgstring))
	  { lgstring = composition_gstring_put_cache (lgstring, -1); }
	(*cmp_it).id = XFIXNUM (LGSTRING_ID (lgstring));
	let mut j: c_int = 0;
	while (j as ptrdiff_t) < LGSTRING_GLYPH_LEN (lgstring)
	  {
	    if NILP (LGSTRING_GLYPH (lgstring, j as isize))
	      { break; }
	    j += 1;
	  }
	(*cmp_it).nglyphs = j;
	(*cmp_it).from = 0;
	(*cmp_it).to = j;
      }
    else
      { break 'no_composition; }
    return true;
  }

  /* no_composition: */
  if charpos == endpos
    { return false; }
  if charpos < endpos
    {
      charpos += 1;
      if NILP (string)
	{ bytepos += next_char_len (bytepos) as ptrdiff_t; }
      else
	{ bytepos += BYTES_BY_CHAR_HEAD (*SDATA (string).offset (bytepos) as c_int) as ptrdiff_t; }
    }
  else
    {
      charpos -= 1;
      /* BYTEPOS is calculated in composition_compute_stop_pos */
      bytepos = -1;
    }
  if (*cmp_it).reversed_p
    { endpos = -1; }
  composition_compute_stop_pos (cmp_it, charpos, bytepos, endpos, string, true);
  return false;
}

/* Update charpos, nchars, nbytes, and width of the current grapheme
   cluster.  ...  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_update_it (cmp_it: *mut composition_it,
						mut charpos: ptrdiff_t,
						bytepos: ptrdiff_t,
						string: Lisp_Object) -> c_int
{
  let mut i: c_int = 0;
  let mut c: c_int = 0;

  if (*cmp_it).ch < 0
    {
      /* static composition */
      let cmp: *mut composition = *composition_table.add ((*cmp_it).id as usize);

      (*cmp_it).charpos = charpos;
      (*cmp_it).to = (*cmp_it).nglyphs;
      if (*cmp_it).nglyphs == 0
	{ c = -1; }
      else
	{
	  i = 0;
	  while i < (*cmp).glyph_len
	    {
	      /* TAB in a composition means display glyphs with padding
		 space on the left or right.  */
	      c = COMPOSITION_GLYPH (cmp, i as isize) as c_int;
	      if c != '\t' as c_int
		{ break; }
	      i += 1;
	    }
	  if c == '\t' as c_int
	    { c = ' ' as c_int; }
	}
      (*cmp_it).width = (*cmp).width;
      charpos += (*cmp_it).nchars as ptrdiff_t;
      if STRINGP (string)
	{ (*cmp_it).nbytes = (string_char_to_byte (string, charpos) - bytepos) as c_int; }
      else
	{ (*cmp_it).nbytes = (CHAR_TO_BYTE (charpos) - bytepos) as c_int; }
    }
  else
    {
      /* Automatic composition.  */
      let gstring: Lisp_Object = composition_gstring_from_id ((*cmp_it).id);
      let mut glyph: Lisp_Object = lisp_h_Qnil ();
      let mut from: ptrdiff_t = 0;

      if (*cmp_it).nglyphs == 0
	{
	  (*cmp_it).nchars = LGSTRING_CHAR_LEN (gstring) as c_int;
	  (*cmp_it).width = 0;
	  (*cmp_it).from = 0;
	  (*cmp_it).to = 0;
	  return -1;
	}
      if ! (*cmp_it).reversed_p
	{
	  glyph = LGSTRING_GLYPH (gstring, (*cmp_it).from as isize);
	  from = LGLYPH_FROM (glyph);
	  (*cmp_it).to = (*cmp_it).from + 1;
	  while (*cmp_it).to < (*cmp_it).nglyphs
	    {
	      glyph = LGSTRING_GLYPH (gstring, (*cmp_it).to as isize);
	      if LGLYPH_FROM (glyph) != from
		{ break; }
	      (*cmp_it).to += 1;
	    }
	  (*cmp_it).charpos = charpos;
	}
      else
	{
	  glyph = LGSTRING_GLYPH (gstring, ((*cmp_it).to - 1) as isize);
	  from = LGLYPH_FROM (glyph);
	  (*cmp_it).charpos = charpos - (LGLYPH_TO (glyph) - from);
	  (*cmp_it).from = (*cmp_it).to - 1;
	  while (*cmp_it).from > 0
	    {
	      glyph = LGSTRING_GLYPH (gstring, ((*cmp_it).from - 1) as isize);
	      if LGLYPH_FROM (glyph) != from
		{ break; }
	      (*cmp_it).from -= 1;
	    }
	}
      glyph = LGSTRING_GLYPH (gstring, (*cmp_it).from as isize);
      (*cmp_it).nchars = (LGLYPH_TO (glyph) + 1 - from) as c_int;
      (*cmp_it).nbytes = 0;
      let mut column_width: c_int = 1;
      /* Compute column width for GUI frames.  */
      if FONT_OBJECT_P (LGSTRING_FONT (gstring))
	  && !(*cmp_it).parent_it.is_null ()
	  && !(*(*cmp_it).parent_it).f.is_null ()
	{ column_width = FRAME_COLUMN_WIDTH ((*(*cmp_it).parent_it).f); }
      (*cmp_it).width = composition_gstring_width (gstring, (*cmp_it).from as ptrdiff_t,
						   (*cmp_it).to as ptrdiff_t,
						   std::ptr::null_mut ());
      /* We need width in column units!  */
      (*cmp_it).width /= column_width;

      i = (*cmp_it).nchars - 1;
      while i >= 0
	{
	  c = XFIXNUM (LGSTRING_CHAR (gstring, from + i as ptrdiff_t)) as c_int;
	  (*cmp_it).nbytes += CHAR_BYTES (c);
	  i -= 1;
	}
    }
  return c;
}


/* Update the members of POSITION to the next character boundary.  */
macro_rules! FORWARD_CHAR {
  ($POSITION:expr, $STOP:expr) => {{
    $POSITION.pos += 1;
    if $POSITION.pos == $STOP
      {
	$POSITION.p = GAP_END_ADDR! ();
	$POSITION.pos_byte = GPT_BYTE! ();
      }
    else
      {
	$POSITION.pos_byte += BYTES_BY_CHAR_HEAD (*$POSITION.p as c_int) as ptrdiff_t;
	$POSITION.p = $POSITION.p.offset (BYTES_BY_CHAR_HEAD (*$POSITION.p as c_int) as isize);
      }
  }};
}

/* Update the members of POSITION to the previous character boundary.  */
macro_rules! BACKWARD_CHAR {
  ($POSITION:expr, $STOP:expr) => {{
    if $POSITION.pos == $STOP
      { $POSITION.p = GPT_ADDR! (); }
    loop
      {
	$POSITION.pos_byte -= 1;
	$POSITION.p = $POSITION.p.offset (-1);
	if CHAR_HEAD_P (*$POSITION.p as c_int)
	  { break; }
      }
    $POSITION.pos -= 1;
  }};
}

/* C 1564-1568.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct position_record
{
  pub pos: ptrdiff_t,
  pub pos_byte: ptrdiff_t,
  pub p: *mut u8,
}

/* Similar to find_composition, but find an automatic composition instead.
   ...  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_automatic_composition (pos: ptrdiff_t, mut limit: ptrdiff_t,
						     backlim: ptrdiff_t,
						     start: *mut ptrdiff_t,
						     end: *mut ptrdiff_t,
						     gstring: *mut Lisp_Object,
						     string: Lisp_Object) -> bool
{
  let mut head: ptrdiff_t = 0;
  let mut tail: ptrdiff_t = 0;
  let mut stop: ptrdiff_t = 0;
  /* Forward limit position of checking a composition taking a
     looking-back count into account.  */
  let mut fore_check_limit: ptrdiff_t = 0;
  let mut cur: position_record = position_record { pos: 0, pos_byte: 0,
						   p: std::ptr::null_mut () };
  let mut prev: position_record = position_record { pos: 0, pos_byte: 0,
						    p: std::ptr::null_mut () };
  let mut c: c_int = 0;
  let window: Lisp_Object = Fget_buffer_window (Fcurrent_buffer (), Qnil ());
  let mut w: *mut window = std::ptr::null_mut ();
  let mut need_adjustment: bool = false;

  if NILP (window)
    { return false; }
  w = XWINDOW (window);

  cur.pos = pos;
  if NILP (string)
    {
      if backlim < 0
	{
	  /* This assumes a newline can never be composed.  */
	  head = find_newline (pos, -1, 0, -1, -1, std::ptr::null_mut (),
			       std::ptr::null_mut (), false);
	}
      else
	{ head = backlim; }
      if (*current_buffer).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P != 0
	{
	  /* In buffers with very long lines, this function becomes very
	     slow.  Pretend that the buffer is narrowed to make it fast.  */
	  let begv: ptrdiff_t = get_small_narrowing_begv (w, window_point (w));
	  if pos > begv
	    { head = begv; }
	}
      tail = ZV! ();
      stop = GPT! ();
      cur.pos_byte = CHAR_TO_BYTE (cur.pos);
      cur.p = BYTE_POS_ADDR (cur.pos_byte);
    }
  else
    {
      head = max (backlim, 0);
      tail = SCHARS (string);
      stop = -1;
      cur.pos_byte = string_char_to_byte (string, cur.pos);
      cur.p = SDATA (string).offset (cur.pos_byte);
    }
  if limit < 0
    /* Finding a composition covering the character after POS is the
       same as setting LIMIT to POS.  */
    { limit = pos; }

  eassert! (backlim < 0 || backlim <= limit);

  if limit <= pos
    { fore_check_limit = min (tail, pos + 1 + MAX_AUTO_COMPOSITION_LOOKBACK as ptrdiff_t); }
  else
    { fore_check_limit = min (tail, limit + MAX_AUTO_COMPOSITION_LOOKBACK as ptrdiff_t); }

  /* Provided that we have these possible compositions now:
     ...  */
  loop
    {
      /* C's `goto search_forward` (case (2)) skips the rewind below; this flag is that
	 jump (PORTING.md 2.2).  */
      let mut search_forward: bool = false;

      c = STRING_CHAR (cur.p);
      if ! char_composable_p (c)
	{
	  if limit <= pos	/* case (1)  */
	    {
	      loop
		{
		  if cur.pos <= limit
		    { return false; }
		  BACKWARD_CHAR! (cur, stop);
		  c = STRING_CHAR (cur.p);
		  if char_composable_p (c)
		    { break; }
		}
	      fore_check_limit = cur.pos + 1;
	    }
	  else			/* case (2) */
	    /* No need of rewinding back.  */
	    { search_forward = true; }
	}

      /* Rewind back to the position where we can safely search
	 forward for compositions.  It is assured that the character
	 at cur.pos is composable.  */
      if !search_forward
	{
	  while head < cur.pos
	    {
	      prev = cur;
	      BACKWARD_CHAR! (cur, stop);
	      c = STRING_CHAR (cur.p);
	      if ! char_composable_p (c)
		{
		  cur = prev;
		  break;
		}
	    }
	}

      /* Search forward. */
      *gstring = Qnil ();
      prev = cur;	/* remember the start of searching position. */
      while cur.pos < fore_check_limit
	{
	  let mut val: Lisp_Object = lisp_h_Qnil ();

	  c = STRING_CHAR (cur.p);
	  val = CHAR_TABLE_REF (Vcomposition_function_table, c as isize);
	  while CONSP (val)
	    {
	      let elt: Lisp_Object = XCAR (val);

	      if VECTORP (elt) && ASIZE (elt) == 3 && FIXNATP (AREF (elt, 1))
		{
		  let check_pos: EMACS_INT = cur.pos - XFIXNAT (AREF (elt, 1));
		  let mut check: position_record = position_record { pos: 0, pos_byte: 0,
								     p: std::ptr::null_mut () };

		  if check_pos < head
		      || (if limit <= pos { pos < check_pos }
			  else { limit <= check_pos })
		    { val = XCDR (val); continue; }
		  check = cur;
		  while check_pos < check.pos
		    { BACKWARD_CHAR! (check, stop); }
		  *gstring = autocmp_chars (elt, check.pos, check.pos_byte,
					    tail, w, std::ptr::null_mut (), string,
					    Qnil (), c);
		  need_adjustment = true;
		  if NILP (*gstring)
		    {
		      /* As we have called Lisp, there's a possibility
			 that buffer/string is relocated.  */
		      if NILP (string)
			{ cur.p = BYTE_POS_ADDR (cur.pos_byte); }
		      else
			{ cur.p = SDATA (string).offset (cur.pos_byte); }
		    }
		  else
		    {
		      /* We found a candidate of a target composition.  */
		      *start = check.pos;
		      *end = check.pos + LGSTRING_CHAR_LEN (*gstring) as ptrdiff_t;
		      if (if pos < limit { pos < *end }
			  else { *start <= pos && pos < *end })
			/* This is the target composition. */
			{ return true; }
		      cur.pos = *end;
		      if NILP (string)
			{
			  cur.pos_byte = CHAR_TO_BYTE (cur.pos);
			  cur.p = BYTE_POS_ADDR (cur.pos_byte);
			}
		      else
			{
			  cur.pos_byte = string_char_to_byte (string, cur.pos);
			  cur.p = SDATA (string).offset (cur.pos_byte);
			}
		      break;
		    }
		}
	      val = XCDR (val);
	    }
	  if ! CONSP (val)
	    /* We found no composition here.  */
	    { FORWARD_CHAR! (cur, stop); }
	}

      if pos < limit		/* case (2) and (4)*/
	{ return false; }
      if ! NILP (*gstring)
	{ return true; }
      if prev.pos == head
	{ return false; }
      cur = prev;
      if need_adjustment
	{
	  if NILP (string)
	    { cur.p = BYTE_POS_ADDR (cur.pos_byte); }
	  else
	    { cur.p = SDATA (string).offset (cur.pos_byte); }
	}
      BACKWARD_CHAR! (cur, stop);
    }
}

/* Return the adjusted point provided that point is moved from LAST_PT
   to NEW_PT.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn composition_adjust_point (last_pt: ptrdiff_t,
						   new_pt: ptrdiff_t) -> ptrdiff_t
{
  let mut i: ptrdiff_t = 0;
  let mut beg: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  let mut val: Lisp_Object = lisp_h_Qnil ();

  if new_pt == BEGV! () || new_pt == ZV! ()
    { return new_pt; }

  /* At first check the static composition. */
  if get_property_and_range (new_pt, Qcomposition (), &raw mut val, &raw mut beg,
			     &raw mut end, Qnil ())
      && composition_valid_p (beg, end, val)
    {
      if beg < new_pt /* && end > new_pt   <- It's always the case.  */
	  && (last_pt <= beg || last_pt >= end)
	{ return if new_pt < last_pt { beg } else { end }; }
      return new_pt;
    }

  if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
      || inhibit_auto_composition ()
    { return new_pt; }

  /* Next check the automatic composition.  */
  if ! find_automatic_composition (new_pt, -1, -1, &raw mut beg, &raw mut end,
				   &raw mut val, Qnil ())
      || beg == new_pt
    { return new_pt; }
  i = 0;
  while i < LGSTRING_GLYPH_LEN (val)
    {
      let glyph: Lisp_Object = LGSTRING_GLYPH (val, i);

      if NILP (glyph)
	{ break; }
      if beg + LGLYPH_FROM (glyph) == new_pt
	{ return new_pt; }
      if beg + LGLYPH_TO (glyph) >= new_pt
	{ return (if new_pt < last_pt
		  { beg + LGLYPH_FROM (glyph) }
		  else { beg + LGLYPH_TO (glyph) + 1 }); }
      i += 1;
    }
  return new_pt;
}

DEFUN! ("composition-get-gstring", Fcomposition_get_gstring,
       Scomposition_get_gstring, 4, 4, 0,
       doc: "Return a glyph-string for characters between FROM and TO.\nIf the glyph string is for graphic display, FONT-OBJECT must be\na font-object to use for those characters.\nOtherwise (for terminal display), FONT-OBJECT must be a terminal ID, a\nframe, or nil for the selected frame's terminal device.\n\nIf the optional 4th argument STRING is not nil, it is a string\ncontaining the target characters between indices FROM and TO,\nwhich are treated as in `substring'.  Otherwise FROM and TO are\ncharacter positions in current buffer; they can be in either order,\nand can be integers or markers.\n\nA glyph-string is a vector containing information about how to display\na specific character sequence.  The format is:\n   [HEADER ID GLYPH ...]\n\nHEADER is a vector of this form:\n    [FONT-OBJECT CHAR ...]\nwhere\n    FONT-OBJECT is a font-object for all glyphs in the glyph-string,\n    or the terminal coding system of the specified terminal.\n    CHARs are characters to be composed by GLYPHs.\n\nID is an identification number of the glyph-string.  It may be nil if\nnot yet shaped.\n\nGLYPH is a vector whose elements have this form:\n    [ FROM-IDX TO-IDX C CODE WIDTH LBEARING RBEARING ASCENT DESCENT\n      [ [X-OFF Y-OFF WADJUST] | nil] ]\nwhere\n    FROM-IDX and TO-IDX are used internally and should not be touched.\n    C is the character of the glyph.\n    CODE is the glyph-code of C in FONT-OBJECT.\n    WIDTH thru DESCENT are the metrics (in pixels) of the glyph.\n    X-OFF and Y-OFF are offsets to the base position for the glyph.\n    WADJUST is the adjustment to the normal width of the glyph.\n\nIf GLYPH is nil, the remaining elements of the glyph-string vector\nshould be ignored.",
       (from: Lisp_Object, to: Lisp_Object, font_object: Lisp_Object,
	string: Lisp_Object),
       {
  let mut from = from;
  let mut to = to;
  let mut font_object = font_object;
  let mut string = string;
  let mut gstring: Lisp_Object = lisp_h_Qnil ();
  let mut header: Lisp_Object = lisp_h_Qnil ();
  let mut frompos: ptrdiff_t = 0;
  let mut frombyte: ptrdiff_t = 0;
  let mut topos: ptrdiff_t = 0;

  if ! FONT_OBJECT_P (font_object)
    {
      let coding: *mut coding_system;
      let terminal: *mut terminal = decode_live_terminal (font_object);

      coding = if ((*(TERMINAL_TERMINAL_CODING (terminal) as *mut coding_system)).bits
		   & CODING_REQUIRE_ENCODING_MASK) != 0
	{ TERMINAL_TERMINAL_CODING (terminal) as *mut coding_system }
	/* the mirror packs coding_system's `common_flags` into `bits` (coding.rs 218-247);
	   C 1929 reads the field, the port the same 14-bit window (docs/ports/composite.md) */
	else { &raw mut safe_terminal_coding };
      font_object = CODING_ID_NAME ((*coding).id as hash_idx_t);
    }

  if NILP (string)
    {
      if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	{ error (c"Attempt to shape unibyte text".as_ptr ()); }
      validate_region (&raw mut from, &raw mut to);
      frompos = XFIXNAT (from);
      topos = XFIXNAT (to);
      frombyte = CHAR_TO_BYTE (frompos);
    }
  else
    {
      CHECK_STRING (string);
      let chars: ptrdiff_t = SCHARS (string);
      validate_subarray (string, from, to, chars, &raw mut frompos, &raw mut topos);
      if ! STRING_MULTIBYTE (string)
	{
	  let mut i: ptrdiff_t = 0;

	  i = SBYTES (string) - 1;
	  while i >= 0
	    {
	      if !ASCII_CHAR_P (SREF (string, i as usize) as isize)
		{ error (c"Attempt to shape unibyte text".as_ptr ()); }
	      i -= 1;
	    }
	  /* STRING is a pure-ASCII string, so we can convert it (or,
	     rather, its copy) to multibyte and use that thereafter.  */
	  /* FIXME: Not clear why we need to do that: AFAICT the rest of
             the code should work on an ASCII-only unibyte string just
             as well (bug#56347).  */
	  string = make_multibyte_string (SSDATA (string), chars, chars);
	}
      frombyte = string_char_to_byte (string, frompos);
    }

  header = fill_gstring_header (frompos, frombyte,
				topos, font_object, string);
  gstring = composition_gstring_lookup_cache (header);
  if ! NILP (gstring)
    { return gstring; }

  if LGSTRING_GLYPH_LEN (gstring_work) < topos - frompos
    { gstring_work = make_nil_vector (topos - frompos + 2); }
  LGSTRING_SET_HEADER (gstring_work, header);
  LGSTRING_SET_ID (gstring_work, Qnil ());
  fill_gstring_body (gstring_work);
  return gstring_work;
});


/* Emacs Lisp APIs.  */

DEFUN! ("compose-region-internal", Fcompose_region_internal,
       Scompose_region_internal, 2, 4, 0,
       doc: "Internal use only.\n\nCompose text in the region between START and END.\nOptional 3rd and 4th arguments are COMPONENTS and MODIFICATION-FUNC\nfor the composition.  See `compose-region' for more details.",
       (start: Lisp_Object, end: Lisp_Object, components: Lisp_Object,
	modification_func: Lisp_Object),
       {
  let mut start = start;
  let mut end = end;
  validate_region (&raw mut start, &raw mut end);
  if !NILP (components)
      && !FIXNUMP (components)
      && !CONSP (components)
      && !STRINGP (components)
    { CHECK_VECTOR (components); }

  compose_text (XFIXNUM (start), XFIXNUM (end), components, modification_func, Qnil ());
  return Qnil ();
});

DEFUN! ("compose-string-internal", Fcompose_string_internal,
       Scompose_string_internal, 3, 5, 0,
       doc: "Internal use only.\n\nCompose text between indices START and END of STRING, where\nSTART and END are treated as in `substring'.  Optional 4th\nand 5th arguments are COMPONENTS and MODIFICATION-FUNC\nfor the composition.  See `compose-string' for more details.",
       (string: Lisp_Object, start: Lisp_Object, end: Lisp_Object,
	components: Lisp_Object, modification_func: Lisp_Object),
       {
  let mut from: ptrdiff_t = 0;
  let mut to: ptrdiff_t = 0;

  CHECK_STRING (string);
  validate_subarray (string, start, end, SCHARS (string), &raw mut from, &raw mut to);
  compose_text (from, to, components, modification_func, string);
  return string;
});

DEFUN! ("find-composition-internal", Ffind_composition_internal,
       Sfind_composition_internal, 4, 4, 0,
       doc: "Internal use only.\n\nReturn information about composition at or nearest to position POS.\nSee `find-composition' for more details.",
       (pos: Lisp_Object, limit: Lisp_Object, string: Lisp_Object,
	detail_p: Lisp_Object),
       {
  let mut prop: Lisp_Object = lisp_h_Qnil ();
  let mut tail: Lisp_Object = lisp_h_Qnil ();
  let mut gstring: Lisp_Object = lisp_h_Qnil ();
  let mut start: ptrdiff_t = 0;
  let mut end: ptrdiff_t = 0;
  let mut from: ptrdiff_t = 0;
  let mut to: ptrdiff_t = 0;
  let mut id: c_int = 0;

  let fixed_pos: EMACS_INT = fix_position (pos);
  if !NILP (limit)
    { to = clip_to_bounds (PTRDIFF_MIN, fix_position (limit), ZV! ()); }
  else
    { to = -1; }

  if !NILP (string)
    {
      CHECK_STRING (string);
      if ! (0 <= fixed_pos && fixed_pos <= SCHARS (string))
	{ args_out_of_range (string, pos); }
    }
  else
    {
      if ! (BEGV! () <= fixed_pos && fixed_pos <= ZV! ())
	{ args_out_of_range (Fcurrent_buffer (), pos); }
    }
  from = fixed_pos;

  if !find_composition (from, to, &raw mut start, &raw mut end, &raw mut prop, string)
    {
      if ((NILP (string)
	   && !NILP (BVAR! (current_buffer, enable_multibyte_characters_)))
	   || (!NILP (string) && STRING_MULTIBYTE (string)))
	  && ! inhibit_auto_composition ()
	  && find_automatic_composition (from, to, -1,
					 &raw mut start, &raw mut end, &raw mut gstring,
					 string)
	{ return list3 (make_fixnum (start), make_fixnum (end), gstring); }
      return Qnil ();
    }
  if ! (start <= fixed_pos && fixed_pos < end)
    {
      let mut s: ptrdiff_t = 0;
      let mut e: ptrdiff_t = 0;

      if find_automatic_composition (from, to, -1,
				     &raw mut s, &raw mut e, &raw mut gstring, string)
	  && (if e <= fixed_pos { e > end } else { s < start })
	{ return list3 (make_fixnum (s), make_fixnum (e), gstring); }
    }
  if !composition_valid_p (start, end, prop)
    { return list3 (make_fixnum (start), make_fixnum (end), Qnil ()); }
  if NILP (detail_p)
    { return list3 (make_fixnum (start), make_fixnum (end), Qt ()); }

  if composition_registered_p (prop)
    { id = COMPOSITION_ID (prop) as c_int; }
  else
    {
      let start_byte: ptrdiff_t = (if NILP (string)
				   { CHAR_TO_BYTE (start) }
				   else { string_char_to_byte (string, start) });
      id = get_composition_id (start, start_byte, end - start, prop, string) as c_int;
    }

  if id >= 0
    {
      let mut components: Lisp_Object = lisp_h_Qnil ();
      let mut relative_p: Lisp_Object = lisp_h_Qnil ();
      let mut mod_func: Lisp_Object = lisp_h_Qnil ();
      let method: composition_method = composition_method (prop);
      let width: c_int = (**composition_table.add (id as usize)).width;

      components = Fcopy_sequence (COMPOSITION_COMPONENTS (prop));
      relative_p = (if method == COMPOSITION_WITH_RULE_ALTCHARS
		    { Qnil () } else { Qt () });
      mod_func = COMPOSITION_MODIFICATION_FUNC (prop);
      tail = list4 (components, relative_p, mod_func, make_fixnum (width as EMACS_INT));
    }
  else
    { tail = Qnil (); }

  return Fcons (make_fixnum (start), Fcons (make_fixnum (end), tail));
});

unsafe extern "C" fn compare_composition_rules (r1: *const c_void, r2: *const c_void) -> c_int
{
  let vec1: Lisp_Object = *(r1 as *const Lisp_Object);
  let vec2: Lisp_Object = *(r2 as *const Lisp_Object);

  return (XFIXNAT (AREF (vec2, 1)) - XFIXNAT (AREF (vec1, 1))) as c_int;
}

DEFUN! ("composition-sort-rules", Fcomposition_sort_rules,
       Scomposition_sort_rules, 1, 1, 0,
       doc: "Sort composition RULES by their LOOKBACK parameter.\n\nIf RULES include just one rule, return RULES.\nOtherwise, return a new list of rules where all the rules are\narranged in decreasing order of the LOOKBACK parameter of the\nrules (the second element of the rule's vector).  This is required\nwhen combining composition rules from different sources, because\nof the way buffer text is examined for matching one of the rules.",
       (rules: Lisp_Object),
       {
  let mut rules = rules;
  let mut nrules: ptrdiff_t = 0;
  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  CHECK_LIST (rules);
  nrules = list_length (rules);
  if nrules > 1
    {
      let mut i: ptrdiff_t = 0;
      let mut sortvec: *mut Lisp_Object = std::ptr::null_mut ();

      SAFE_NALLOCA! (sa_avail, sortvec, 1, nrules);
      i = 0;
      while i < nrules
	{
	  let elt: Lisp_Object = XCAR (rules);
	  if VECTORP (elt) && ASIZE (elt) == 3 && FIXNATP (AREF (elt, 1))
	    { *sortvec.offset (i as isize) = elt; }
	  else
	    { error (c"Invalid composition rule in RULES argument".as_ptr ()); }
	  rules = XCDR (rules);
	  i += 1;
	}
      qsort (sortvec as *mut c_void, nrules as usize,
	     std::mem::size_of::<Lisp_Object> (), Some (compare_composition_rules));
      rules = Flist (nrules, sortvec);
      SAFE_FREE! (sa_count);
    }

  return rules;
});


/* Not strictly necessary, because all those "keys" are also
   reachable from `composition_hash_table`.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_composite ()
{
  let mut i: c_int = 0;
  while (i as ptrdiff_t) < n_compositions
    {
      mark_object ((**composition_table.add (i as usize)).key);
      i += 1;
    }
}

// The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
// (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
// port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
// defines it here (status §3.29, the casetab.rs shape).
#[unsafe(no_mangle)]
pub static mut Vcompose_chars_after_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vauto_composition_mode: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vauto_composition_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vcomposition_function_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vauto_composition_emoji_eligible_codepoints: Lisp_Object = lisp_h_Qnil ();


#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_composite ()
{
  let mut i: c_int = 0;

  DEFSYM! (Qcomposition, c"composition");

  /* Make a hash table for static composition.  */
  /* We used to make the hash table weak so that unreferenced
     compositions can be garbage-collected.  But, usually once
     created compositions are repeatedly used in an Emacs session,
     and thus it's not worth to save memory in such a way.  So, we
     make the table not weak.  */
  let args: [Lisp_Object; 4] = [QCtest (), Qequal (), QCsize (), make_fixnum (311)];
  composition_hash_table = CALLMANY! (Fmake_hash_table, args);
  staticpro (&raw const composition_hash_table);

  /* Make a hash table for glyph-string.  */
  gstring_hash_table = CALLMANY! (Fmake_hash_table, args);
  staticpro (&raw const gstring_hash_table);

  staticpro (&raw const gstring_work_headers);
  gstring_work_headers = make_nil_vector (8);
  i = 0;
  while i < 8
    {
      ASET (gstring_work_headers, i as isize, make_nil_vector (i as isize + 2));
      i += 1;
    }
  staticpro (&raw const gstring_work);
  gstring_work = make_nil_vector (10);

  /* Text property `composition' should be nonsticky by default.  */
  Vtext_property_default_nonsticky
    = Fcons (Fcons (Qcomposition (), Qt ()), Vtext_property_default_nonsticky);

  DEFVAR_LISP! (c"compose-chars-after-function", Vcompose_chars_after_function,
		"Function to adjust composition of buffer text.\n\nThis function is called with three arguments: FROM, TO, and OBJECT.\nFROM and TO specify the range of text whose composition should be\nadjusted.  OBJECT, if non-nil, is a string that contains the text.\n\nThis function is called after a text with `composition' property is\ninserted or deleted to keep `composition' property of buffer text\nvalid.\n\nThe default value is the function `compose-chars-after'.");
  Vcompose_chars_after_function = intern_c_string (c"compose-chars-after".as_ptr ());

  DEFSYM! (Qauto_composed, c"auto-composed");

  DEFVAR_LISP! (c"auto-composition-mode", Vauto_composition_mode,
		"Non-nil if Auto-Composition mode is enabled.\nUse the command `auto-composition-mode' to change this variable.\n\nIf this variable is a string, `auto-composition-mode' will be disabled in\nbuffers displayed on a terminal whose type, as reported by `tty-type',\ncompares equal to that string.");
  Vauto_composition_mode = Qt ();

  DEFVAR_LISP! (c"auto-composition-function", Vauto_composition_function,
		"Function to call to compose characters automatically.\nThis function is called from the display engine with 6 arguments:\nFUNC, FROM, TO, FONT-OBJECT, STRING, and DIRECTION.\n\nFUNC is the function to compose characters.  On text-mode display,\nFUNC is ignored and `compose-gstring-for-terminal' is used instead.\n\nIf STRING is nil, the function must compose characters in the region\nbetween FROM and TO in the current buffer.\n\nOtherwise, STRING is a string, and FROM and TO are indices into the\nstring.  In this case, the function must compose characters in the\nstring.\n\nFONT-OBJECT is the font to use, or nil if characters are to be\ncomposed on a text-mode display.\n\nDIRECTION is the bidi directionality of the text to shape.  It could\nbe L2R or R2L, or nil if unknown.");
  Vauto_composition_function = Qnil ();

  DEFVAR_LISP! (c"composition-function-table", Vcomposition_function_table,
		"Char-table of functions for automatic character composition.\nFor each character that has to be composed automatically with\npreceding and/or following characters, this char-table contains\na function to call to compose that character.\n\nThe element at index C in the table, if non-nil, is a list of\ncomposition rules of the form ([PATTERN PREV-CHARS FUNC] ...);\nthe rules must be specified in the descending order of PREV-CHARS\nvalues.\n\nPATTERN is a regular expression which C and the surrounding\ncharacters must match.\n\nPREV-CHARS is a non-negative integer (less than 4) specifying how many\ncharacters before C to check the matching with PATTERN.  If it is 0,\nPATTERN must match C and the following characters.  If it is 1,\nPATTERN must match a character before C and the following characters.\n\nIf PREV-CHARS is 0, PATTERN can be nil, which means that the\nsingle character C should be composed.\n\nFUNC is a function to return a glyph-string representing a\ncomposition of the characters that match PATTERN.  It is\ncalled with one argument GSTRING.\n\nGSTRING is a template of a glyph-string to return.  It is already\nfilled with a proper header for the characters to compose, and\nglyphs corresponding to those characters one by one.  The\nfunction must return a new glyph-string with the same header as\nGSTRING, or modify GSTRING itself and return it.\n\nSee also the documentation of `auto-composition-mode'.");
  Vcomposition_function_table = Fmake_char_table (Qnil (), Qnil ());

  DEFVAR_LISP! (c"auto-composition-emoji-eligible-codepoints",
		Vauto_composition_emoji_eligible_codepoints,
		"List of codepoints for which auto-composition will check for an emoji font.\n\nThese are codepoints which have Emoji_Presentation = No, and thus by\ndefault are not displayed as emoji.  In certain circumstances, such as\nwhen followed by U+FE0F (VS-16) the emoji font should be used for\nthem anyway.\n\nThis list is auto-generated, you should not need to modify it.");
  Vauto_composition_emoji_eligible_codepoints = Qnil ();

  defsubr (&raw mut Scompose_region_internal);
  defsubr (&raw mut Scompose_string_internal);
  defsubr (&raw mut Sfind_composition_internal);
  defsubr (&raw mut Scomposition_get_gstring);
  defsubr (&raw mut Sclear_composition_cache);
  defsubr (&raw mut Scomposition_sort_rules);
}
