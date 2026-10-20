// femacs: port of src/search.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, struct field, macro, file-scope object, local), same control flow, same error
// model.  C's 19 `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  The non-`static` definitions C
// exports - `shrink_regexp_cache`, `clear_regexp_cache`, `fast_string_match_internal`,
// `fast_c_string_match_internal`, `fast_looking_at`, `find_newline`, `scan_newline`,
// `scan_newline_from_point`, `find_newline_no_quit`, `find_before_next_newline`,
// `search_buffer`, `restore_search_regs`, `update_search_regs`,
// `record_unwind_save_match_data`, `find_newline1`, `syms_of_search` and the one object
// `re_match_object` - carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static`
// helpers stay plain `unsafe fn` (the ones that can signal are `-> !`, C's `AVOID`).
//
// **Carried arms.**  `src/search.c` is *not* in `docs/baseline/config/cond-eval-verified.txt`,
// so `tools/extract-abi.py`'s `blind_conditions` keeps *both* arms of every `#if` live and
// the per-function token check demands the tokens of C's text on both sides.  Per the
// integrator's 2026-10-03 decision (docs/status.md 3.49, the `timefns.rs`/`region-cache.rs`
// precedent) this file therefore *carries* the text of the arms the frozen build does not
// compile, each under a `#[cfg (...)]` that names the C condition's non-frozen side
// (PORTING.md P-51).  The arms:
//
//   * C 84-97 `freeze_buffer_relocation`: the `#ifdef REL_ALLOC` half (`REL_ALLOC` is undef
//     in the frozen `config.h`, macros.tsv 3302) - feature `rel_alloc`.
//   * C 1091-1094, `search_command`'s literal `#if 0` - dead in *every* configuration, so
//     it is recorded in docs/ports/search.md and not transcribed (the `#if 0` rule of
//     P-08).
//   * C 2019-2028 and 2040-2058, `boyer_moore`'s two `#ifdef REL_ALLOC` blocks - feature
//     `rel_alloc`.
//   * C 3394-3412, `Fre__describe_compiled`'s three arms: `ENABLE_CHECKING` is undef
//     (macros.tsv 872), so clang compiles the first one; the port keys the first on
//     `#[cfg (not (enable_checking))]` and the second (ENABLE_CHECKING && HAVE_OPEN_MEMSTREAM,
//     the latter 1 in the frozen table, macros.tsv 1482) on `#[cfg (enable_checking)]`.
//     The third (`!HAVE_OPEN_MEMSTREAM`) is dead in the frozen configuration and is carried
//     under `#[cfg (all (enable_checking, feature = "no_have_open_memstream"))]`.
//
// None of the `feature` keys above is ever set: `xtask dead-arms` can only flip the keys
// `config.rs` carries and these are the preprocessor predicates of this file
// (docs/status.md 3.9), so the compiled build is exactly what clang compiles.
//
// **Use-site declarations** (this unit declares what it needs with C's own prototype above
// it, PORTING.md 0.1-2/P-72, the indent.rs/ccl.rs shape): `memrchr`; `substring_both`;
// `make_specified_string`; `open_memstream`/`stderr` (checking arm only).  The two header
// families that used to stand here - `regex-emacs.h`'s engine half and `region-cache.h`'s five
// entries - are their mirrors' now (`include/regex-emacs.rs`, `include/region-cache.rs`);
// the mirror gaps that remain are reported in docs/ports/search.md.
//
// `struct regexp_cache` is declared here (C declares it in the .c), which is why
// `tools/gen-layout-fields.py`'s `STRUCTS` list does not name it; its layout and
// `struct re_pattern_buffer`'s are measured with clang against the frozen target and pinned
// with the `const _` assertions below.  The three `bool_bf`/`unsigned` bit runs of
// `re_pattern_buffer` (C 95-113) are packed into one `bits: u8` with the `RE_PATTERN_*`
// masks (PORTING.md 1.3/P-80); the reads and writes of `used_syntax`, `multibyte` and
// `target_multibyte` go through them, and the fields `can_be_null`, `regs_allocated` and
// `fastmap_accurate` keep their declaration (layout) without a reader in this file.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code)]

include!("include/config.rs");                 // C 22: `#include <config.h>`
include!("../lib/include/stdio.rs");           // C 24: `#include <stdio.h>`
include!("../lib/include/stdlib.rs");          // C 25: `#include <stdlib.h>`
include!("../lib/include/string.rs");          // lisp.h 30: `<string.h>` (memchr, memcpy)
include!("../lib/include/stdint.in.rs");       // lisp.h 25: `<stdint.h>` (PTRDIFF_MIN/MAX)
include!("../lib/include/limits.in.rs");       // lisp.h 26: `<limits.h>` (UINT_MAX)
include!("include/lisp.rs");                   // C 27: `#include "lisp.h"`
include!("include/character.rs");              // C 28: `#include "character.h"`
include!("include/itree.rs");                  // buffer.h 27: `#include "itree.h"`
include!("include/buffer.rs");                 // C 29: `#include "buffer.h"`
include!("include/syntax.rs");                 // C 30: `#include "syntax.h"`
include!("include/charset.rs");                // C 31: `#include "charset.h"`
include!("include/region-cache.rs");           // C 32: `#include "region-cache.h"`
include!("include/blockinput.rs");             // C 33: `#include "blockinput.h"`
include!("include/intervals.rs");              // C 33: `#include "intervals.h"`
include!("include/pdumper.rs");                // C 34: `#include "pdumper.h"`
include!("include/composite.rs");              // C 35: `#include "composite.h"`
include!("include/regex-emacs.rs");            // C 37: `#include "regex-emacs.h"`

/* `<intprops.h>` has no mirror (PORTING.md P-26): C's `TYPE_MAXIMUM (t)` is the type's own
   largest value, the syntax.rs/dtotimespec.rs shape.  */
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }
macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }

/* The `regex-emacs.h` engine half (`struct re_pattern_buffer`, its six bit accessors, the
   five `re_*` entries and the checking-only `print_compiled_pattern`) is the mirror's now:
   `femacs/src/include/regex-emacs.rs` carries them with C's prototypes, and this unit names
   them through that header exactly as C does (PORTING.md 0.1-10; the two copies that stood
   here went away in the same commit as the mirror, or this unit would be an `E0428`).  */
/// C: region-cache.h 68: `struct region_cache` (region-cache.c defines it), 65's `struct buffer;`.
pub type region_cache = c_void;
/* `region-cache.h`'s five entries (`new_region_cache`, `free_region_cache`,
   `know_region_cache`, `region_cache_forward`, `region_cache_backward`) and C's `struct
   region_cache` - the record is reached only through a pointer, so the mirror names it with
   the ABI-equal `c_void` alias (PORTING.md 0.1-2/P-72 rule 3) - are the mirror's now:
   `include/region-cache.rs` carries them with C's prototypes.  The declarations and the alias
   that stood here went away in the same change as the mirror, or this unit would be an
   `E0428`; nothing in the bodies had to move, because the alias keeps `struct region_cache *`
   in their text.  */

unsafe extern "C" {
  /// C: <string.h>: `void *memrchr (const void *, int, size_t);` (gnulib's replacement;
  /// the filelock.rs shape).
  fn memrchr (s: *const c_void, c: c_int, n: usize) -> *mut c_void;
  /// C: lisp.h 4566-4567: `extern Lisp_Object make_specified_string (const char *chars,
  /// ptrdiff_t nchars, ptrdiff_t nbytes, bool multibyte);`
  fn make_specified_string (chars: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
			    multibyte: bool) -> Lisp_Object;

  /// C: lisp.h 4010-4012: `extern ptrdiff_t copy_text (const unsigned char *from_addr,
  /// unsigned char *to_addr, ptrdiff_t nbytes, bool from_multibyte, bool to_multibyte);`
  /// (insdel.c's definition; the mirror does not carry it - the `Fcopy_sequence` shape).
  fn copy_text (from_addr: *const u8, to_addr: *mut u8, nbytes: ptrdiff_t,
		from_multibyte: bool, to_multibyte: bool) -> ptrdiff_t;
  /// C: <stdio.h>: `FILE *open_memstream (char **, size_t *);` (the checking arm only).
  fn open_memstream (bufloc: *mut *mut c_char, sizeloc: *mut usize) -> *mut FILE;
}

/* If the regexp is non-nil, then the buffer contains the compiled form
   of that regexp, suitable for searching.  */
// C 40: REGEXP_CACHE_SIZE.
pub const REGEXP_CACHE_SIZE: usize = 20;                        // C 42

// C 46-58.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct regexp_cache
{
  pub next: *mut regexp_cache,
  pub regexp: Lisp_Object,
  pub f_whitespace_regexp: Lisp_Object,
  /* Syntax table for which the regexp applies.  We need this because
     of character classes.  If this is t, then the compiled pattern is valid
     for any syntax-table.  */
  pub syntax_table: Lisp_Object,
  pub buf: re_pattern_buffer,
  pub fastmap: [c_char; 256],                                   // 0400
  /* True means regexp was compiled to do full POSIX backtracking.  */
  pub posix: bool,
  /* True means we're inside a buffer match.  */
  pub busy: bool,
}
/// Measured with clang against the frozen target: `sizeof` 360, `_Alignof` 8, `next`@0,
/// `regexp`@8, `f_whitespace_regexp`@16, `syntax_table`@24, `buf`@32, `fastmap`@96,
/// `posix`@352, `busy`@353.
const _: () = core::assert! (std::mem::size_of::<regexp_cache> () == 360);
const _: () = core::assert! (std::mem::align_of::<regexp_cache> () == 8);
const _: () = core::assert! (std::mem::offset_of! (regexp_cache, next) == 0);
const _: () = core::assert! (std::mem::offset_of! (regexp_cache, regexp) == 8);
const _: () = core::assert! (std::mem::offset_of! (regexp_cache, f_whitespace_regexp) == 16);
const _: () = core::assert! (std::mem::offset_of! (regexp_cache, syntax_table) == 24);
const _: () = core::assert! (std::mem::offset_of! (regexp_cache, buf) == 32);
const _: () = core::assert! (std::mem::offset_of! (regexp_cache, fastmap) == 96);
const _: () = core::assert! (std::mem::offset_of! (regexp_cache, posix) == 352);
const _: () = core::assert! (std::mem::offset_of! (regexp_cache, busy) == 353);

/* The instances of that struct.  */
static mut searchbufs: [regexp_cache; REGEXP_CACHE_SIZE] =  // C 61
  [regexp_cache {
     next: std::ptr::null_mut (),
     regexp: std::ptr::null_mut (),
     f_whitespace_regexp: std::ptr::null_mut (),
     syntax_table: std::ptr::null_mut (),
     buf: re_pattern_buffer {
       buffer: std::ptr::null_mut (),
       allocated: 0,
       used: 0,
       charset_unibyte: 0,
       fastmap: std::ptr::null_mut (),
       translate: std::ptr::null_mut (),
       re_nsub: 0,
       bits: 0,
     },
     fastmap: [0; 256],
     posix: false,
     busy: false,
   }; REGEXP_CACHE_SIZE];

/* The head of the linked list; points to the most recently used buffer.  */
static mut searchbuf_head: *mut regexp_cache = std::ptr::null_mut ();   // C 64

// C 66-73: C's four forward declarations of `set_search_regs`, `save_search_regs`,
// `simple_search` and `boyer_moore` are dropped (PORTING.md 0.1-12); the definitions stay
// where C has them.

/// C 75: `Lisp_Object re_match_object;` - non-`static`, so this unit is the one that
/// exports the symbol (regex-emacs.h 50 declares it; PORTING.md 0.2).
#[unsafe(no_mangle)]
pub static mut re_match_object: Lisp_Object = std::ptr::null_mut ();    // C 75

/* The bit members of `struct re_pattern_buffer` this unit reads or writes are the mirror's
   accessors now: `femacs/src/include/regex-emacs.rs` carries `can_be_null_`/`regs_allocated_`/
   `fastmap_accurate_`/`used_syntax_`/`multibyte_`/`target_multibyte_` with the `RE_PATTERN_*`
   masks, exactly as the copy that stood here did (PORTING.md 1.3/P-80, 0.1-10).  */
/* Prevent ralloc.c from relocating the current buffer while searching it.  */
#[cfg (feature = "rel_alloc")]
unsafe extern "C" {
  /// C: lisp.h 4681: `extern void r_alloc_inhibit_buffer_relocation (int);`
  fn r_alloc_inhibit_buffer_relocation (inhibit: c_int);
}

unsafe fn matcher_overflow () -> !                           // C 76-80
{
  error (c"Stack overflow in regexp matcher".as_ptr ());
}

unsafe fn freeze_buffer_relocation ()                        // C 82-91
{
  #[cfg (feature = "rel_alloc")]
  /* Prevent ralloc.c from relocating the current buffer while
     searching it.  */
  {
    r_alloc_inhibit_buffer_relocation (1);
    record_unwind_protect_int (Some (r_alloc_inhibit_buffer_relocation), 0);
  }
}

/* Compile a regexp and signal a Lisp error if anything goes wrong.
   PATTERN is the pattern to compile.
   CP is the place to put the result.
   TRANSLATE is a translation table for ignoring case, or nil for none.
   POSIX is true if we want full backtracking (POSIX style) for this pattern.
   False means backtrack only enough to get a valid match.

   The behavior also depends on Vsearch_spaces_regexp.  */

unsafe fn compile_pattern_1 (cp: *mut regexp_cache, pattern: Lisp_Object,   // C 100-140
			     translate: Lisp_Object, posix: bool)
{
  let mut whitespace_regexp: *const c_char;
  let mut val: *mut c_char = std::ptr::null_mut ();

  eassert! (!(*cp).busy);
  (*cp).regexp = Qnil ();
  (*cp).buf.translate = translate;
  (*cp).posix = posix;
  set_multibyte_ (&raw mut (*cp).buf, STRING_MULTIBYTE (pattern));
  (*cp).buf.charset_unibyte = charset_unibyte;
  if STRINGP (Vsearch_spaces_regexp)
    { (*cp).f_whitespace_regexp = Vsearch_spaces_regexp; }
  else
    { (*cp).f_whitespace_regexp = Qnil (); }

  whitespace_regexp = if STRINGP (Vsearch_spaces_regexp)
    { SSDATA (Vsearch_spaces_regexp) } else { std::ptr::null () };

  val = re_compile_pattern (SSDATA (pattern), SBYTES (pattern), posix,
			    whitespace_regexp, &raw mut (*cp).buf) as *mut c_char;

  /* If the compiled pattern hard codes some of the contents of the
     syntax-table, it can only be reused with *this* syntax table.  */
  (*cp).syntax_table = if used_syntax_ (&raw const (*cp).buf)
    { BVAR! ((current_buffer! ()), syntax_table_) } else { Qt () };

  if ! val.is_null ()
    { xsignal1 (Qinvalid_regexp (), build_string (val)); }

  (*cp).regexp = Fcopy_sequence (pattern);
}

/* Shrink each compiled regexp buffer in the cache
   to the size actually used right now.
   This is called from garbage collection.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shrink_regexp_cache ()              // C 142-158
{
  let mut cp: *mut regexp_cache;

  cp = searchbuf_head;
  while ! cp.is_null ()
    {
      if !(*cp).busy
	{
	  (*cp).buf.allocated = (*cp).buf.used;
	  (*cp).buf.buffer = xrealloc ((*cp).buf.buffer as *mut c_void,
				       (*cp).buf.used as usize) as *mut u8;
	}
      cp = (*cp).next;
    }
}

/* Clear the regexp cache w.r.t. a particular syntax table,
   because it was changed.
   There is no danger of memory leak here because re_compile_pattern
   automagically manages the memory in each re_pattern_buffer struct,
   based on its `allocated' and `buffer' values.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clear_regexp_cache ()               // C 160-171
{
  let mut i: c_int;

  i = 0;
  while i < REGEXP_CACHE_SIZE as c_int
    {
      /* It's tempting to compare with the syntax-table we've actually changed,
	 but it's not sufficient because char-table inheritance means that
	 modifying one syntax-table can change others at the same time.  */
      if !searchbufs[i as usize].busy && !BASE_EQ (searchbufs[i as usize].syntax_table, Qt ())
	{ searchbufs[i as usize].regexp = Qnil (); }
      i += 1;
    }
}

unsafe extern "C" fn unfreeze_pattern (arg: *mut c_void)     // C 173-178
{
  let searchbuf = arg as *mut regexp_cache;
  (*searchbuf).busy = false;
}

unsafe fn freeze_pattern (searchbuf: *mut regexp_cache)      // C 180-186
{
  eassert! (!(*searchbuf).busy);
  record_unwind_protect_ptr (Some (unfreeze_pattern), searchbuf as *mut c_void);
  (*searchbuf).busy = true;
}

/* Compile a regexp if necessary, but first check to see if there's one in
   the cache.
   PATTERN is the pattern to compile.
   TRANSLATE is a translation table for ignoring case, or nil for none.
   REGP is the structure that says where to store the "register"
   values that will result from matching this pattern.
   If it is 0, we should compile the pattern not to record any
   subexpression bounds.
   POSIX is true if we want full backtracking (POSIX style) for this pattern.
   False means backtrack only enough to get a valid match.  */

unsafe fn compile_pattern (pattern: Lisp_Object, regp: *mut re_registers,   // C 195-259
			   translate: Lisp_Object, posix: bool, multibyte: bool)
  -> *mut regexp_cache
{
  let mut cp: *mut regexp_cache = std::ptr::null_mut ();
  let mut cpp: *mut *mut regexp_cache;
  let mut lru_nonbusy: *mut *mut regexp_cache;

  cpp = &raw mut searchbuf_head;
  lru_nonbusy = std::ptr::null_mut ();
  'search: loop
    {
      cp = *cpp;
      if !(*cp).busy
	{ lru_nonbusy = cpp; }
      /* Entries are initialized to nil, and may be set to nil by
	 compile_pattern_1 if the pattern isn't valid.  Don't apply
	 string accessors in those cases.  However, compile_pattern_1
	 is only applied to the cache entry we pick here to reuse.  So
	 nil should never appear before a non-nil entry.  */
      let mut compile_it: bool = NILP ((*cp).regexp);
      if ! compile_it
	{
	  if SCHARS ((*cp).regexp) == SCHARS (pattern)
	      && !(*cp).busy
	      && STRING_MULTIBYTE ((*cp).regexp) == STRING_MULTIBYTE (pattern)
	      && !NILP (Fstring_equal ((*cp).regexp, pattern))
	      && BASE_EQ ((*cp).buf.translate, translate)
	      && (*cp).posix == posix
	      && (BASE_EQ ((*cp).syntax_table, Qt ())
		  || BASE_EQ ((*cp).syntax_table,
			      BVAR! ((current_buffer! ()), syntax_table_)))
	      && !NILP (Fequal ((*cp).f_whitespace_regexp, Vsearch_spaces_regexp))
	      && (*cp).buf.charset_unibyte == charset_unibyte
	    { break 'search; }

	  /* If we're at the end of the cache, compile into the last
	     (least recently used) non-busy cell in the cache.  */
	  if (*cp).next.is_null ()
	    {
	      if lru_nonbusy.is_null ()
		{ error (c"Too much matching reentrancy".as_ptr ()); }
	      cpp = lru_nonbusy;
	      cp = *cpp;
	      compile_it = true;
	    }
	}
      if compile_it
	{
	  /* compile_it: */
	  eassert! (!(*cp).busy);
	  compile_pattern_1 (cp, pattern, translate, posix);
	  break 'search;
	}
      cpp = &raw mut (*cp).next;
    }

  /* When we get here, cp (aka *cpp) contains the compiled pattern,
     either because we found it in the cache or because we just compiled it.
     Move it to the front of the queue to mark it as most recently used.  */
  *cpp = (*cp).next;
  (*cp).next = searchbuf_head;
  searchbuf_head = cp;

  /* Advise the searching functions about the space we have allocated
     for register data.  */
  if ! regp.is_null ()
    {
      re_set_registers (&raw mut (*cp).buf, regp,
			(*(regp as *mut re_registers)).num_regs,
			(*(regp as *mut re_registers)).start,
			(*(regp as *mut re_registers)).end);
    }

  /* The compiled pattern can be used both for multibyte and unibyte
     target.  But, we have to tell which the pattern is used for. */
  set_target_multibyte_ (&raw mut (*cp).buf, multibyte);
  return cp;
}

unsafe fn looking_at_1 (string: Lisp_Object, posix: bool, modify_data: bool)   // C 261-350
  -> Lisp_Object
{
  let mut val: Lisp_Object;
  let mut p1: *mut u8;
  let mut p2: *mut u8;
  let mut s1: ptrdiff_t;
  let mut s2: ptrdiff_t;
  let mut i: ptrdiff_t;

  if running_asynch_code
    { save_search_regs (); }

  /* This is so set_image_of_range_1 in regex-emacs.c can find the EQV
     table.  */
  set_char_table_extras (BVAR! ((current_buffer! ()), case_canon_table_), 2,
			 BVAR! ((current_buffer! ()), case_eqv_table_));

  CHECK_STRING (string);

  /* Snapshot in case Lisp changes the value.  */
  let modify_match_data: bool = NILP (Vinhibit_changing_match_data) && modify_data;

  let cache_entry: *mut regexp_cache = compile_pattern (
    string,
    if modify_match_data { std::ptr::addr_of_mut! (search_regs! ()) } else { std::ptr::null_mut () },
    if !NILP (Vcase_fold_search) { BVAR! ((current_buffer! ()), case_canon_table_) } else { Qnil () },
    posix,
    !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)));

  /* Do a pending quit right away, to avoid paradoxical behavior */
  maybe_quit ();

  /* Get pointers and sizes of the two strings
     that make up the visible portion of the buffer. */

  p1 = BEGV_ADDR! ();
  s1 = GPT_BYTE! () - BEGV_BYTE! ();
  p2 = GAP_END_ADDR! ();
  s2 = ZV_BYTE! () - GPT_BYTE! ();
  if s1 < 0
    {
      p2 = p1;
      s2 = ZV_BYTE! () - BEGV_BYTE! ();
      s1 = 0;
    }
  if s2 < 0
    {
      s1 = ZV_BYTE! () - BEGV_BYTE! ();
      s2 = 0;
    }

  let count: specpdl_ref = SPECPDL_INDEX ();
  freeze_buffer_relocation ();
  freeze_pattern (cache_entry);
  re_match_object = Qnil ();
  i = re_match_2 (&raw mut (*cache_entry).buf, p1 as *const c_char, s1, p2 as *const c_char, s2,
		  PT_BYTE! () - BEGV_BYTE! (),
		  if modify_match_data { std::ptr::addr_of_mut! (search_regs! ()) } else { std::ptr::null_mut () },
		  ZV_BYTE! () - BEGV_BYTE! ());

  if i == -2
    {
      unbind_to (count, Qnil ());
      matcher_overflow ();
    }

  /* Instrument, not behaviour (P-115's shape; status 4.494): detect a *repeating* call - the same
     point with the same pattern a few hundred times in a row - and report that call's text and
     pattern once.  Repetition, not buffer size, is what identifies the loop, so the load-time noise
     (which also has small buffers) does not hide it.  */
  unsafe {
    static mut LAST_POINT: ptrdiff_t = -1;
    static mut LAST_PAT: Lisp_Object = std::ptr::null_mut ();
    static mut SAME: c_int = 0;
    /* Sampled once: the switch is read on the hottest path in the program, and `getenv` per call
       would be a real cost (P-126's "sample at startup" rule, applied to a diagnostic).  */
    static mut TRACE: c_int = -1;
    if TRACE < 0
      { TRACE = if getenv (c"FEMACS_REG_TRACE".as_ptr ()).is_null () { 0 } else { 1 }; }
    if TRACE != 0
      {
        let here: ptrdiff_t = PT! ();
        if here == LAST_POINT && string == LAST_PAT
          { SAME += 1; }
        else
          { LAST_POINT = here; LAST_PAT = string; SAME = 1; }
        if SAME == 200
          {
            fprintf (stderr, c"REGTRACE loop point=%td i=%td text=".as_ptr (), here, i as isize);
            let total: ptrdiff_t = ZV! () - BEGV! ();
            let mut k: ptrdiff_t = 0;
            while k < total && k < 40
              { fputc (FETCH_BYTE! (BEGV_BYTE! () + k) as c_int, stderr); k += 1; }
            fprintf (stderr, c" pat=".as_ptr ());
            if STRINGP (string)
              {
                let d: *const c_char = SSDATA (string);
                let n: ptrdiff_t = SBYTES (string);
                let mut j: ptrdiff_t = 0;
                while j < n && j < 160
                  { fputc (*d.offset (j) as c_int, stderr); j += 1; }
              }
            fprintf (stderr, c"\n".as_ptr ());
          }
      }
  }

  val = if i >= 0 { Qt () } else { Qnil () };
  if modify_match_data && i >= 0
    {
      i = 0;
      while i < search_regs! ().num_regs
	{
	  if *search_regs! ().start.offset (i as isize) >= 0
	    {
	      *search_regs! ().start.offset (i as isize)
		= BYTE_TO_CHAR (*search_regs! ().start.offset (i as isize) + BEGV_BYTE! ());
	      *search_regs! ().end.offset (i as isize)
		= BYTE_TO_CHAR (*search_regs! ().end.offset (i as isize) + BEGV_BYTE! ());
	    }
	  i += 1;
	}
      /* Set last_thing_searched only when match data is changed.  */
      XSETBUFFER! (last_thing_searched! (), (current_buffer! ()));
    }

  return unbind_to (count, val);
}

DEFUN! ("looking-at", Flooking_at, Slooking_at, 1, 2, 0,          // C 347-358
       doc: "Return t if text after point matches regular expression REGEXP.\nBy default, this function modifies the match data that\n`match-beginning', `match-end' and `match-data' access.  If\nINHIBIT-MODIFY is non-nil, don't modify the match data.",
       (regexp: Lisp_Object, inhibit_modify: Lisp_Object),
       {
	 return looking_at_1 (regexp, false, NILP (inhibit_modify));
       });

DEFUN! ("posix-looking-at", Fposix_looking_at, Sposix_looking_at, 1, 2, 0,  // C 357-367
       doc: "Return t if text after point matches REGEXP according to Posix rules.\nFind the longest match, in accordance with Posix regular expression rules.\n\nBy default, this function modifies the match data that\n`match-beginning', `match-end' and `match-data' access.  If\nINHIBIT-MODIFY is non-nil, don't modify the match data.",
       (regexp: Lisp_Object, inhibit_modify: Lisp_Object),
       {
	 return looking_at_1 (regexp, true, NILP (inhibit_modify));
       });

unsafe fn string_match_1 (regexp: Lisp_Object, string: Lisp_Object,   // C 369-440
			  start: Lisp_Object, posix: bool, modify_data: bool)
  -> Lisp_Object
{
  let mut val: ptrdiff_t;
  let mut pos: EMACS_INT;
  let mut pos_byte: ptrdiff_t;
  let mut i: ptrdiff_t;
  let modify_match_data: bool = NILP (Vinhibit_changing_match_data) && modify_data;

  if running_asynch_code
    { save_search_regs (); }

  CHECK_STRING (regexp);
  CHECK_STRING (string);

  if NILP (start)
    {
      pos = 0;
      pos_byte = 0;
    }
  else
    {
      let len: ptrdiff_t = SCHARS (string);

      CHECK_FIXNUM (start);
      pos = XFIXNUM (start);
      if pos < 0 && -pos <= len
	{ pos = len + pos; }
      else if 0 > pos || pos > len
	{ args_out_of_range (string, start); }
      pos_byte = string_char_to_byte (string, pos);
    }

  /* This is so set_image_of_range_1 in regex-emacs.c can find the EQV
     table.  */
  set_char_table_extras (BVAR! ((current_buffer! ()), case_canon_table_), 2,
			 BVAR! ((current_buffer! ()), case_eqv_table_));

  let count: specpdl_ref = SPECPDL_INDEX ();
  let cache_entry: *mut regexp_cache
    = compile_pattern (regexp,
		       if modify_match_data { std::ptr::addr_of_mut! (search_regs! ()) } else { std::ptr::null_mut () },
		       if !NILP (Vcase_fold_search) { BVAR! ((current_buffer! ()), case_canon_table_) } else { Qnil () },
		       posix,
		       STRING_MULTIBYTE (string));
  freeze_pattern (cache_entry);
  re_match_object = string;
  val = re_search (&raw mut (*cache_entry).buf, SSDATA (string),
		   SBYTES (string), pos_byte,
		   SBYTES (string) - pos_byte,
		   if modify_match_data { std::ptr::addr_of_mut! (search_regs! ()) } else { std::ptr::null_mut () });
  unbind_to (count, Qnil ());

  /* Set last_thing_searched only when match data is changed.  */
  if modify_match_data
    { last_thing_searched! () = Qt (); }

  if val == -2
    { matcher_overflow (); }
  if val < 0
    { return Qnil (); }

  if modify_match_data
    {
      i = 0;
      while i < search_regs! ().num_regs
	{
	  if *search_regs! ().start.offset (i as isize) >= 0
	    {
	      *search_regs! ().start.offset (i as isize)
		= string_byte_to_char (string, *search_regs! ().start.offset (i as isize));
	      *search_regs! ().end.offset (i as isize)
		= string_byte_to_char (string, *search_regs! ().end.offset (i as isize));
	    }
	  i += 1;
	}
    }

  return make_fixnum (string_byte_to_char (string, val));
}

DEFUN! ("string-match", Fstring_match, Sstring_match, 2, 4, 0,     // C 442-460
       doc: "Return index of start of first match for REGEXP in STRING, or nil.\nMatching ignores case if `case-fold-search' is non-nil.\nIf third arg START is non-nil, start search at that index in STRING.\n\nIf INHIBIT-MODIFY is non-nil, match data is not changed.\n\nIf INHIBIT-MODIFY is nil or missing, match data is changed, and\n`match-end' and `match-beginning' give indices of substrings matched\nby parenthesis constructs in the pattern.  You can use the function\n`match-string' to extract the substrings matched by the parenthesis\nconstructions in REGEXP.  For index of first char beyond the match, do\n(match-end 0).",
       (regexp: Lisp_Object, string: Lisp_Object, start: Lisp_Object,
	inhibit_modify: Lisp_Object),
       {
	 return string_match_1 (regexp, string, start, false, NILP (inhibit_modify));
       });

DEFUN! ("posix-string-match", Fposix_string_match, Sposix_string_match, 2, 4, 0,  // C 461-480
       doc: "Return index of start of first match for Posix REGEXP in STRING, or nil.\nFind the longest match, in accord with Posix regular expression rules.\nCase is ignored if `case-fold-search' is non-nil in the current buffer.\n\nIf INHIBIT-MODIFY is non-nil, match data is not changed.\n\nIf INHIBIT-MODIFY is nil or missing, match data is changed, and\n`match-end' and `match-beginning' give indices of substrings matched\nby parenthesis constructs in the pattern.  You can use the function\n`match-string' to extract the substrings matched by the parenthesis\nconstructions in REGEXP.  For index of first char beyond the match, do\n(match-end 0).",
       (regexp: Lisp_Object, string: Lisp_Object, start: Lisp_Object,
	inhibit_modify: Lisp_Object),
       {
	 return string_match_1 (regexp, string, start, true, NILP (inhibit_modify));
       });

/* Match REGEXP against STRING using translation table TABLE,
   searching all of STRING, and return the index of the match,
   or negative on failure.  This does not clobber the match data.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fast_string_match_internal (regexp: Lisp_Object,   // C 483-500
						    string: Lisp_Object,
						    table: Lisp_Object)
  -> ptrdiff_t
{
  re_match_object = string;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let cache_entry: *mut regexp_cache
    = compile_pattern (regexp, std::ptr::null_mut (), table, false, STRING_MULTIBYTE (string));
  freeze_pattern (cache_entry);
  let val: ptrdiff_t = re_search (&raw mut (*cache_entry).buf, SSDATA (string),
				  SBYTES (string), 0,
				  SBYTES (string), std::ptr::null_mut ());
  unbind_to (count, Qnil ());
  return val;
}

/* Match REGEXP against STRING, searching all of STRING and return the
   index of the match, or negative on failure.  This does not clobber
   the match data.  Table is a canonicalize table for ignoring case,
   or nil for none.

   We assume that STRING contains single-byte characters.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fast_c_string_match_internal (regexp: Lisp_Object,   // C 508-533
						      string: *const c_char,
						      len: ptrdiff_t,
						      table: Lisp_Object)
  -> ptrdiff_t
{
  /* FIXME: This is expensive and not obviously correct when it makes
     a difference. I.e., no longer "fast", and may hide bugs.
     Something should be done about this.  */
  let regexp: Lisp_Object = string_make_unibyte (regexp);
  /* Record specpdl index because freeze_pattern pushes an
     unwind-protect on the specpdl.  */
  let count: specpdl_ref = SPECPDL_INDEX ();
  let cache_entry: *mut regexp_cache
    = compile_pattern (regexp, std::ptr::null_mut (), table, false, false);
  freeze_pattern (cache_entry);
  re_match_object = Qt ();
  let val: ptrdiff_t = re_search (&raw mut (*cache_entry).buf, string, len, 0, len,
				  std::ptr::null_mut ());
  unbind_to (count, Qnil ());
  return val;
}

/* Match REGEXP against the characters after POS to LIMIT, and return
   the number of matched characters.  If STRING is non-nil, match
   against the characters in it.  In that case, POS and LIMIT are
   indices into the string.  This function doesn't modify the match
   data.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fast_looking_at (regexp: Lisp_Object, pos: ptrdiff_t,   // C 535-600
					  pos_byte: ptrdiff_t, limit: ptrdiff_t,
					  limit_byte: ptrdiff_t, string: Lisp_Object)
  -> ptrdiff_t
{
  let mut multibyte: bool;
  let mut p1: *mut u8;
  let mut p2: *mut u8;
  let mut s1: ptrdiff_t;
  let mut s2: ptrdiff_t;
  let mut len: ptrdiff_t;
  let mut pos_byte = pos_byte;
  let mut limit_byte = limit_byte;

  if STRINGP (string)
    {
      if pos_byte < 0
	{ pos_byte = string_char_to_byte (string, pos); }
      if limit_byte < 0
	{ limit_byte = string_char_to_byte (string, limit); }
      p1 = std::ptr::null_mut ();
      s1 = 0;
      p2 = SDATA (string);
      s2 = SBYTES (string);
      multibyte = STRING_MULTIBYTE (string);
    }
  else
    {
      if pos_byte < 0
	{ pos_byte = CHAR_TO_BYTE (pos); }
      if limit_byte < 0
	{ limit_byte = CHAR_TO_BYTE (limit); }
      pos_byte -= BEGV_BYTE! ();
      limit_byte -= BEGV_BYTE! ();
      p1 = BEGV_ADDR! ();
      s1 = GPT_BYTE! () - BEGV_BYTE! ();
      p2 = GAP_END_ADDR! ();
      s2 = ZV_BYTE! () - GPT_BYTE! ();
      if s1 < 0
	{
	  p2 = p1;
	  s2 = ZV_BYTE! () - BEGV_BYTE! ();
	  s1 = 0;
	}
      if s2 < 0
	{
	  s1 = ZV_BYTE! () - BEGV_BYTE! ();
	  s2 = 0;
	}
      multibyte = ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
    }

  let cache_entry: *mut regexp_cache =
    compile_pattern (regexp, std::ptr::null_mut (), Qnil (), false, multibyte);
  let count: specpdl_ref = SPECPDL_INDEX ();
  freeze_buffer_relocation ();
  freeze_pattern (cache_entry);
  re_match_object = if STRINGP (string) { string } else { Qnil () };
  len = re_match_2 (&raw mut (*cache_entry).buf, p1 as *const c_char, s1,
		    p2 as *const c_char, s2,
		    pos_byte, std::ptr::null_mut (), limit_byte);

  unbind_to (count, Qnil ());
  return len;
}

/* The newline cache: remembering which sections of text have no newlines.  */

/* If the user has requested the long scans caching, make sure it's on.
   Otherwise, make sure it's off.
   This is our cheezy way of associating an action with the change of
   state of a buffer-local variable.  */
unsafe fn newline_cache_on_off (buf: *mut buffer) -> *mut region_cache   // C 602-655
{
  let mut base_buf: *mut buffer = buf;
  let mut indirect_p: bool = false;

  if !(*buf).base_buffer.is_null ()
    {
      base_buf = (*buf).base_buffer;
      indirect_p = true;
    }

  /* Don't turn on or off the cache in the base buffer, if the value
     of cache-long-scans of the base buffer is inconsistent with that.
     This is because doing so will just make the cache pure overhead,
     since if we turn it on via indirect buffer, it will be
     immediately turned off by its base buffer.  */
  if NILP (BVAR! (buf, cache_long_scans_))
    {
      if !indirect_p
	  || NILP (BVAR! (base_buf, cache_long_scans_))
	{
	  /* It should be off.  */
	  if !(*base_buf).newline_cache.is_null ()
	    {
	      free_region_cache ((*base_buf).newline_cache);
	      (*base_buf).newline_cache = std::ptr::null_mut ();
	    }
	}
      return std::ptr::null_mut ();
    }
  else
    {
      if !indirect_p
	  || !NILP (BVAR! (base_buf, cache_long_scans_))
	{
	  /* It should be on.  */
	  if (*base_buf).newline_cache.is_null ()
	    {
	      (*base_buf).newline_cache = new_region_cache ();
	      __lsan_ignore_object ((*base_buf).newline_cache);
	    }
	}
      return (*base_buf).newline_cache;
    }
}

/* Search for COUNT newlines between START/START_BYTE and END/END_BYTE.

   If COUNT is positive, search forwards; END must be >= START.
   If COUNT is negative, search backwards for the -COUNTth instance;
      END must be <= START.
   If COUNT is zero, do anything you please; run rogue, for all I care.

   If END is zero, use BEGV or ZV instead, as appropriate for the
   direction indicated by COUNT.  If START_BYTE is -1 it is unknown,
   and similarly for END_BYTE.

   If we find COUNT instances, set *COUNTED to COUNT, and return the
   position past the COUNTth match.  Note that for reverse motion
   this is not the same as the usual convention for Emacs motion commands.

   If we don't find COUNT instances before reaching END, set *COUNTED
   to the number of newlines left found (negated if COUNT is negative),
   and return END.

   If BYTEPOS is not NULL, set *BYTEPOS to the byte position corresponding
   to the returned character position.

   If ALLOW_QUIT, check for quitting.  That's good to do
   except when inside redisplay.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_newline (start: ptrdiff_t, start_byte: ptrdiff_t,   // C 675-966
				       end: ptrdiff_t, end_byte: ptrdiff_t,
				       count: ptrdiff_t, counted: *mut ptrdiff_t,
				       bytepos: *mut ptrdiff_t, allow_quit: bool)
  -> ptrdiff_t
{
  let newline_cache: *mut region_cache;
  let cache_buffer: *mut buffer;
  let mut start = start;
  let mut start_byte = start_byte;
  let mut end = end;
  let mut end_byte = end_byte;
  let mut count = count;

  if end == 0
    {
      if count > 0
	{ end = ZV! (); end_byte = ZV_BYTE! (); }
      else
	{ end = BEGV! (); end_byte = BEGV_BYTE! (); }
    }
  if end_byte == -1
    { end_byte = CHAR_TO_BYTE (end); }

  newline_cache = newline_cache_on_off ((current_buffer! ()));
  if !(*(current_buffer! ())).base_buffer.is_null ()
    { cache_buffer = (*(current_buffer! ())).base_buffer; }
  else
    { cache_buffer = (current_buffer! ()); }

  if ! counted.is_null ()
    { *counted = count; }

  if count > 0
    {
      while start != end
	{
	  /* Our innermost scanning loop is very simple; it doesn't know
	     about gaps, buffer ends, or the newline cache.  ceiling is
	     the position of the last character before the next such
	     obstacle --- the last character the dumb search loop should
	     examine.  */
	  let mut tem: ptrdiff_t;
	  let mut ceiling_byte: ptrdiff_t = end_byte - 1;

	  /* If we're using the newline cache, consult it to see whether
	     we can avoid some scanning.  */
	  if ! newline_cache.is_null ()
	    {
	      let mut next_change: ptrdiff_t = 0;
	      let mut result: c_int = 1;

	      while start < end && result != 0
		{
		  let mut lim1: ptrdiff_t;

		  result = region_cache_forward (cache_buffer, newline_cache,
						 start, &raw mut next_change);
		  if result != 0
		    {
		      /* When the cache revalidation is deferred,
			 next-change might point beyond ZV, which will
			 cause assertion violation in CHAR_TO_BYTE below.
			 Limit next_change to ZV to avoid that.  */
		      if next_change > ZV! ()
			{ next_change = ZV! (); }
		      start = next_change;
		      next_change = end;
		      lim1 = next_change;
		    }
		  else
		    { lim1 = min (next_change, end); }

		  /* The cache returned zero for this region; see if
		     this is because the region is known and includes
		     only newlines.  While at that, count any newlines
		     we bump into, and exit if we found enough off them.  */
		  start_byte = CHAR_TO_BYTE (start);
		  while start < lim1
			&& FETCH_BYTE! (start_byte) == b'\n'
		    {
		      start_byte += 1;
		      start += 1;
		      count -= 1;
		      if count == 0
			{
			  if ! bytepos.is_null ()
			    { *bytepos = start_byte; }
			  return start;
			}
		    }
		  /* If we found a non-newline character before hitting
		     position where the cache will again return non-zero
		     (i.e. no newlines beyond that position), it means
		     this region is not yet known to the cache, and we
		     must resort to the "dumb loop" method.  */
		  if start < next_change && result == 0
		    { break; }
		  result = 1;
		}
	      if start >= end
		{
		  start = end;
		  start_byte = end_byte;
		  break;
		}

	      /* START should never be after END.  */
	      if start_byte > ceiling_byte
		{ start_byte = ceiling_byte; }

	      /* Now the text after start is an unknown region, and
		 next_change is the position of the next known region. */
	      ceiling_byte = min (CHAR_TO_BYTE (next_change) - 1, ceiling_byte);
	    }
	  else if start_byte == -1
	    { start_byte = CHAR_TO_BYTE (start); }

	  /* The dumb loop can only scan text stored in contiguous
	     bytes. BUFFER_CEILING_OF returns the last character
	     position that is contiguous, so the ceiling is the
	     position after that.  */
	  tem = BUFFER_CEILING_OF (start_byte);
	  ceiling_byte = min (tem, ceiling_byte);

	  {
	    /* The termination address of the dumb loop.  */
	    let mut lim_addr: *mut u8 = BYTE_POS_ADDR (ceiling_byte).offset (1);
	    let lim_byte: ptrdiff_t = ceiling_byte + 1;

	    /* Nonpositive offsets (relative to LIM_ADDR and LIM_BYTE)
	       of the base, the cursor, and the next line.  */
	    let base: ptrdiff_t = start_byte - lim_byte;
	    let mut cursor: ptrdiff_t;
	    let mut next: ptrdiff_t;

	    cursor = base;
	    while cursor < 0
	      {
		/* The dumb loop.  */
		let nl: *mut u8 = memchr (lim_addr.offset (cursor as isize) as *const c_void,
					  b'\n' as c_int, (- cursor) as usize) as *mut u8;
		next = if ! nl.is_null () { nl.offset_from (lim_addr) } else { 0 };

		/* If we're using the newline cache, cache the fact that
		   the region we just traversed is free of newlines. */
		if ! newline_cache.is_null () && cursor != next
		  {
		    know_region_cache (cache_buffer, newline_cache,
				       BYTE_TO_CHAR (lim_byte + cursor),
				       BYTE_TO_CHAR (lim_byte + next));
		    /* know_region_cache can relocate buffer text.  */
		    lim_addr = BYTE_POS_ADDR (ceiling_byte).offset (1);
		  }

		if nl.is_null ()
		  { break; }
		next += 1;

		count -= 1;
		if count == 0
		  {
		    if ! bytepos.is_null ()
		      { *bytepos = lim_byte + next; }
		    return BYTE_TO_CHAR (lim_byte + next);
		  }
		if allow_quit
		  { maybe_quit (); }

		cursor = next;
	      }

	    start_byte = lim_byte;
	    start = BYTE_TO_CHAR (start_byte);
	  }
	}
    }
  else
    {
      while start > end
	{
	  /* The last character to check before the next obstacle.  */
	  let mut tem: ptrdiff_t;
	  let mut ceiling_byte: ptrdiff_t = end_byte;

	  /* Consult the newline cache, if appropriate.  */
	  if ! newline_cache.is_null ()
	    {
	      let mut next_change: ptrdiff_t = 0;
	      let mut result: c_int = 1;

	      while start > end && result != 0
		{
		  let mut lim1: ptrdiff_t;

		  result = region_cache_backward (cache_buffer, newline_cache,
						  start, &raw mut next_change);
		  if result != 0
		    {
		      start = next_change;
		      next_change = end;
		      lim1 = next_change;
		    }
		  else
		    { lim1 = max (next_change, end); }
		  start_byte = CHAR_TO_BYTE (start);
		  while start > lim1
			&& FETCH_BYTE! (start_byte - 1) == b'\n'
		    {
		      count += 1;
		      if count == 0
			{
			  if ! bytepos.is_null ()
			    { *bytepos = start_byte; }
			  return start;
			}
		      start_byte -= 1;
		      start -= 1;
		    }
		  if start > next_change && result == 0
		    { break; }
		  result = 1;
		}
	      if start <= end
		{
		  start = end;
		  start_byte = end_byte;
		  break;
		}

	      /* Start should never be at or before end.  */
	      if start_byte <= ceiling_byte
		{ start_byte = ceiling_byte + 1; }

	      /* Now the text before start is an unknown region, and
		 next_change is the position of the next known region. */
	      ceiling_byte = max (CHAR_TO_BYTE (next_change), ceiling_byte);
	    }
	  else if start_byte == -1
	    { start_byte = CHAR_TO_BYTE (start); }

	  /* Stop scanning before the gap.  */
	  tem = BUFFER_FLOOR_OF (start_byte - 1);
	  ceiling_byte = max (tem, ceiling_byte);

	  {
	    /* The termination address of the dumb loop.  */
	    let mut ceiling_addr: *mut u8 = BYTE_POS_ADDR (ceiling_byte);

	    /* Offsets (relative to CEILING_ADDR and CEILING_BYTE) of
	       the base, the cursor, and the previous line.  These
	       offsets are at least -1.  */
	    let base: ptrdiff_t = start_byte - ceiling_byte;
	    let mut cursor: ptrdiff_t;
	    let mut prev: ptrdiff_t;

	    cursor = base;
	    while 0 < cursor
	      {
		let nl: *mut u8 = memrchr (ceiling_addr as *const c_void, b'\n' as c_int,
					   cursor as usize) as *mut u8;
		prev = if ! nl.is_null () { nl.offset_from (ceiling_addr) } else { -1 };

		/* If we're looking for newlines, cache the fact that
		   this line's region is free of them. */
		if ! newline_cache.is_null () && cursor != prev + 1
		  {
		    know_region_cache (cache_buffer, newline_cache,
				       BYTE_TO_CHAR (ceiling_byte + prev + 1),
				       BYTE_TO_CHAR (ceiling_byte + cursor));
		    /* know_region_cache can relocate buffer text.  */
		    ceiling_addr = BYTE_POS_ADDR (ceiling_byte);
		  }

		if nl.is_null ()
		  { break; }

		count += 1;
		if count >= 0
		  {
		    if ! bytepos.is_null ()
		      { *bytepos = ceiling_byte + prev + 1; }
		    return BYTE_TO_CHAR (ceiling_byte + prev + 1);
		  }
		if allow_quit
		  { maybe_quit (); }

		cursor = prev;
	      }

	    start_byte = ceiling_byte;
	    start = BYTE_TO_CHAR (start_byte);
	  }
	}
    }

  if ! counted.is_null ()
    { *counted -= count; }
  if ! bytepos.is_null ()
    {
      *bytepos = if start_byte == -1 { CHAR_TO_BYTE (start) } else { start_byte };
      eassert! (*bytepos == CHAR_TO_BYTE (start));
    }
  return start;
}

/* Search for COUNT instances of a line boundary.
   Start at START.  If COUNT is negative, search backwards.

   We report the resulting position by calling TEMP_SET_PT_BOTH.

   If we find COUNT instances. we position after (always after,
   even if scanning backwards) the COUNTth match.

   If we don't find COUNT instances before reaching the end of the
   buffer (or the beginning, if scanning backwards), we position at
   the limit we bumped up against.

   If ALLOW_QUIT, check for quitting.  That's good to do
   except in special cases.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scan_newline (start: ptrdiff_t, start_byte: ptrdiff_t,   // C 968-984
				       limit: ptrdiff_t, limit_byte: ptrdiff_t,
				       count: ptrdiff_t, allow_quit: bool)
{
  let mut charpos: ptrdiff_t;
  let mut bytepos: ptrdiff_t = 0;
  let mut counted: ptrdiff_t = 0;

  charpos = find_newline (start, start_byte, limit, limit_byte,
			  count, &raw mut counted, &raw mut bytepos, allow_quit);
  if counted != count
    { TEMP_SET_PT_BOTH (limit, limit_byte); }
  else
    { TEMP_SET_PT_BOTH (charpos, bytepos); }
}

/* Like above, but always scan from point and report the
   resulting position in *CHARPOS and *BYTEPOS.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn scan_newline_from_point (count: ptrdiff_t,   // C 986-1001
						  charpos: *mut ptrdiff_t,
						  bytepos: *mut ptrdiff_t)
  -> ptrdiff_t
{
  let mut counted: ptrdiff_t = 0;

  if count <= 0
    { *charpos = find_newline (PT! (), PT_BYTE! (), BEGV! (), BEGV_BYTE! (), count - 1,
			       &raw mut counted, bytepos, true); }
  else
    { *charpos = find_newline (PT! (), PT_BYTE! (), ZV! (), ZV_BYTE! (), count,
			       &raw mut counted, bytepos, true); }
  return counted;
}

/* Like find_newline, but doesn't allow QUITting and doesn't return
   COUNTED.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_newline_no_quit (from: ptrdiff_t, frombyte: ptrdiff_t,   // C 1003-1012
					       cnt: ptrdiff_t, bytepos: *mut ptrdiff_t)
  -> ptrdiff_t
{
  return find_newline (from, frombyte, 0, -1, cnt, std::ptr::null_mut (), bytepos, false);
}

/* Like find_newline, but returns position before the newline, not
   after, and only search up to TO.
   This isn't just find_newline_no_quit (...)-1, because you might hit TO.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_before_next_newline (from: ptrdiff_t, to: ptrdiff_t,   // C 1014-1031
						   cnt: ptrdiff_t, bytepos: *mut ptrdiff_t)
  -> ptrdiff_t
{
  let mut counted: ptrdiff_t = 0;
  let mut pos: ptrdiff_t = find_newline (from, -1, to, -1, cnt, &raw mut counted, bytepos, true);

  if counted == cnt
    {
      if ! bytepos.is_null ()
	{ dec_both (&raw mut pos, &raw mut *bytepos); }
      else
	{ pos -= 1; }
    }
  return pos;
}

/* Subroutines of Lisp buffer search functions. */

unsafe fn search_command (string: Lisp_Object, bound: Lisp_Object, noerror: Lisp_Object,   // C 1033-1107
			  count: Lisp_Object, direction: c_int, RE: bool, posix: bool)
  -> Lisp_Object
{
  let mut np: EMACS_INT;
  let mut lim: EMACS_INT;
  let mut lim_byte: ptrdiff_t;
  let mut n: EMACS_INT = direction as EMACS_INT;

  if !NILP (count)
    {
      CHECK_FIXNUM (count);
      n *= XFIXNUM (count);
    }

  CHECK_STRING (string);
  if NILP (bound)
    {
      if n > 0
	{ lim = ZV! (); lim_byte = ZV_BYTE! (); }
      else
	{ lim = BEGV! (); lim_byte = BEGV_BYTE! (); }
    }
  else
    {
      lim = fix_position (bound);
      if if n > 0 { lim < PT! () } else { lim > PT! () }
	{ error (c"Invalid search bound (wrong side of point)".as_ptr ()); }
      if lim > ZV! ()
	{ lim = ZV! (); lim_byte = ZV_BYTE! (); }
      else if lim < BEGV! ()
	{ lim = BEGV! (); lim_byte = BEGV_BYTE! (); }
      else
	{ lim_byte = CHAR_TO_BYTE (lim); }
    }

  /* This is so set_image_of_range_1 in regex-emacs.c can find the EQV
     table.  */
  set_char_table_extras (BVAR! ((current_buffer! ()), case_canon_table_), 2,
			 BVAR! ((current_buffer! ()), case_eqv_table_));

  np = search_buffer (string, PT! (), PT_BYTE! (), lim, lim_byte, n, RE,
		      if !NILP (Vcase_fold_search) { BVAR! ((current_buffer! ()), case_canon_table_) } else { Qnil () },
		      if !NILP (Vcase_fold_search) { BVAR! ((current_buffer! ()), case_eqv_table_) } else { Qnil () },
		      posix);
  if np <= 0
    {
      if NILP (noerror)
	{ xsignal1 (Qsearch_failed (), string); }

      if !EQ (noerror, Qt ())
	{
	  eassert! (BEGV! () <= lim && lim <= ZV! ());
	  SET_PT_BOTH (lim, lim_byte);
	  return Qnil ();
	  /* C 1091-1094: `#if 0` - `np = lim;` - is dead text in every configuration
	     (P-08); docs/ports/search.md records it.  */
	}
      else
	{ return Qnil (); }
    }

  eassert! (BEGV! () <= np && np <= ZV! ());
  SET_PT (np);

  return make_fixnum (np);
}

/* Return true if REGEXP it matches just one constant string.  */

unsafe fn trivial_regexp_p (regexp: Lisp_Object) -> bool       // C 1109-1135
{
  let mut len: ptrdiff_t = SBYTES (regexp);
  let mut s: *mut u8 = SDATA (regexp);
  while { len -= 1; len >= 0 }
    {
      let mut c: u8 = *s;
      s = s.offset (1);
      match c
	{
	  b'.' | b'*' | b'+' | b'?' | b'[' | b'^' | b'$' => { return false; }
	  b'\\' =>
	    {
	      len -= 1;
	      if len < 0
		{ return false; }
	      c = *s;
	      s = s.offset (1);
	      match c
		{
		  b'|' | b'(' | b')' | b'`' | b'\'' | b'b'
		    | b'B' | b'<' | b'>' | b'w' | b'W' | b's'
		    | b'S' | b'=' | b'{' | b'}' | b'_'
		    | b'c' | b'C'	/* for categoryspec and notcategoryspec */
		    | b'1' | b'2' | b'3' | b'4' | b'5'
		    | b'6' | b'7' | b'8' | b'9' => { return false; }
		  _ => {}
		}
	    }
	  _ => {}
	}
    }
  return true;
}

macro_rules! TRANSLATE {                                       // C 1137-1149
  ($out:ident, $trt:expr, $d:expr) => {
    if ! NILP ($trt)
      {
	let temp: Lisp_Object;
	temp = Faref ($trt, make_fixnum ($d as EMACS_INT));
	if FIXNUMP (temp)
	  { $out = XFIXNUM (temp) as c_int; }
	else
	  { $out = $d; }
      }
    else
      { $out = $d; }
  };
}

/* Only used in search_buffer, to record the end position of the match
   when searching regexps and SEARCH_REGS should not be changed
   (i.e. Vinhibit_changing_match_data is non-nil).  */
static mut search_regs_1: re_registers =                      // C 1155
  re_registers { num_regs: 0, start: std::ptr::null_mut (), end: std::ptr::null_mut () };

unsafe fn search_buffer_re (string: Lisp_Object, pos: ptrdiff_t, pos_byte: ptrdiff_t,   // C 1157-1296
			    lim: ptrdiff_t, lim_byte: ptrdiff_t, n: EMACS_INT,
			    trt: Lisp_Object, inverse_trt: Lisp_Object, posix: bool)
  -> EMACS_INT
{
  let mut p1: *mut u8;
  let mut p2: *mut u8;
  let mut s1: ptrdiff_t;
  let mut s2: ptrdiff_t;
  let mut pos = pos;
  let mut pos_byte = pos_byte;
  let mut n = n;

  /* Snapshot in case Lisp changes the value.  */
  let preserve_match_data: bool = NILP (Vinhibit_changing_match_data);

  let cache_entry: *mut regexp_cache =
    compile_pattern (string,
		     if preserve_match_data { std::ptr::addr_of_mut! (search_regs! ()) } else { &raw mut search_regs_1 },
		     trt, posix,
		     !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)));
  let bufp: *mut re_pattern_buffer = &raw mut (*cache_entry).buf;

  maybe_quit ();		/* Do a pending quit right away,
				   to avoid paradoxical behavior */
  /* Get pointers and sizes of the two strings
     that make up the visible portion of the buffer. */

  p1 = BEGV_ADDR! ();
  s1 = GPT_BYTE! () - BEGV_BYTE! ();
  p2 = GAP_END_ADDR! ();
  s2 = ZV_BYTE! () - GPT_BYTE! ();
  if s1 < 0
    {
      p2 = p1;
      s2 = ZV_BYTE! () - BEGV_BYTE! ();
      s1 = 0;
    }
  if s2 < 0
    {
      s1 = ZV_BYTE! () - BEGV_BYTE! ();
      s2 = 0;
    }

  let count: specpdl_ref = SPECPDL_INDEX ();
  freeze_buffer_relocation ();
  freeze_pattern (cache_entry);

  while n < 0
    {
      let mut val: ptrdiff_t;

      re_match_object = Qnil ();
      val = re_search_2 (bufp, p1 as *const c_char, s1, p2 as *const c_char, s2,
			 pos_byte - BEGV_BYTE! (), lim_byte - pos_byte,
			 if preserve_match_data { std::ptr::addr_of_mut! (search_regs! ()) } else { &raw mut search_regs_1 },
			 /* Don't allow match past current point */
			 pos_byte - BEGV_BYTE! ());
      if val == -2
	{
	  unbind_to (count, Qnil ());
	  matcher_overflow ();
	}
      if val >= 0
	{
	  if preserve_match_data
	    {
	      pos_byte = *search_regs! ().start.offset (0) + BEGV_BYTE! ();
	      let mut i: ptrdiff_t = 0;
	      while i < search_regs! ().num_regs
		{
		  if *search_regs! ().start.offset (i as isize) >= 0
		    {
		      *search_regs! ().start.offset (i as isize)
			= BYTE_TO_CHAR (*search_regs! ().start.offset (i as isize) + BEGV_BYTE! ());
		      *search_regs! ().end.offset (i as isize)
			= BYTE_TO_CHAR (*search_regs! ().end.offset (i as isize) + BEGV_BYTE! ());
		    }
		  i += 1;
		}
	      XSETBUFFER! (last_thing_searched! (), (current_buffer! ()));
	      /* Set pos to the new position. */
	      pos = *search_regs! ().start.offset (0);
	    }
	  else
	    {
	      pos_byte = *search_regs_1.start.offset (0) + BEGV_BYTE! ();
	      /* Set pos to the new position.  */
	      pos = BYTE_TO_CHAR (*search_regs_1.start.offset (0) + BEGV_BYTE! ());
	    }
	}
      else
	{
	  unbind_to (count, Qnil ());
	  return n;
	}
      n += 1;
      maybe_quit ();
    }
  while n > 0
    {
      let mut val: ptrdiff_t;

      re_match_object = Qnil ();
      val = re_search_2 (bufp, p1 as *const c_char, s1, p2 as *const c_char, s2,
			 pos_byte - BEGV_BYTE! (), lim_byte - pos_byte,
			 if preserve_match_data { std::ptr::addr_of_mut! (search_regs! ()) } else { &raw mut search_regs_1 },
			 lim_byte - BEGV_BYTE! ());
      if val == -2
	{
	  unbind_to (count, Qnil ());
	  matcher_overflow ();
	}
      if val >= 0
	{
	  if preserve_match_data
	    {
	      pos_byte = *search_regs! ().end.offset (0) + BEGV_BYTE! ();
	      let mut i: ptrdiff_t = 0;
	      while i < search_regs! ().num_regs
		{
		  if *search_regs! ().start.offset (i as isize) >= 0
		    {
		      *search_regs! ().start.offset (i as isize)
			= BYTE_TO_CHAR (*search_regs! ().start.offset (i as isize) + BEGV_BYTE! ());
		      *search_regs! ().end.offset (i as isize)
			= BYTE_TO_CHAR (*search_regs! ().end.offset (i as isize) + BEGV_BYTE! ());
		    }
		  i += 1;
		}
	      XSETBUFFER! (last_thing_searched! (), (current_buffer! ()));
	      pos = *search_regs! ().end.offset (0);
	    }
	  else
	    {
	      pos_byte = *search_regs_1.end.offset (0) + BEGV_BYTE! ();
	      pos = BYTE_TO_CHAR (*search_regs_1.end.offset (0) + BEGV_BYTE! ());
	    }
	}
      else
	{
	  unbind_to (count, Qnil ());
	  return 0 - n;
	}
      n -= 1;
      maybe_quit ();
    }
  unbind_to (count, Qnil ());
  return pos;
}

unsafe fn search_buffer_non_re (string: Lisp_Object, pos: ptrdiff_t,   // C 1298-1472
				pos_byte: ptrdiff_t, lim: ptrdiff_t, lim_byte: ptrdiff_t,
				n: EMACS_INT, RE: bool, trt: Lisp_Object,
				inverse_trt: Lisp_Object, posix: bool)
  -> EMACS_INT
{
  let mut raw_pattern: *mut u8;
  let mut pat: *mut u8;
  let mut raw_pattern_size: ptrdiff_t;
  let mut raw_pattern_size_byte: ptrdiff_t;
  let mut patbuf: *mut u8 = std::ptr::null_mut ();
  let multibyte: bool = !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
  let mut base_pat: *mut u8;
  /* Set to positive if we find a non-ASCII char that need
     translation.  Otherwise set to zero later.  */
  let mut char_base: c_int = -1;
  let mut boyer_moore_ok: bool = true;

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  /* MULTIBYTE says whether the text to be searched is multibyte.
     We must convert PATTERN to match that, or we will not really
     find things right.  */

  if multibyte == STRING_MULTIBYTE (string)
    {
      raw_pattern = SDATA (string);
      raw_pattern_size = SCHARS (string);
      raw_pattern_size_byte = SBYTES (string);
    }
  else if multibyte
    {
      raw_pattern_size = SCHARS (string);
      raw_pattern_size_byte
	= count_size_as_multibyte (SDATA (string),
				   raw_pattern_size);
      raw_pattern = SAFE_ALLOCA! (sa_avail, raw_pattern_size_byte + 1) as *mut u8;
      copy_text (SDATA (string), raw_pattern,
		 SCHARS (string), false, true);
    }
  else
    {
      /* Converting multibyte to single-byte.  */
      raw_pattern_size = SCHARS (string);
      raw_pattern_size_byte = SCHARS (string);
      raw_pattern = SAFE_ALLOCA! (sa_avail, raw_pattern_size + 1) as *mut u8;
      copy_text (SDATA (string), raw_pattern,
		 SBYTES (string), true, false);
    }

  /* Copy and optionally translate the pattern.  */
  let mut len: ptrdiff_t = raw_pattern_size;
  let mut len_byte: ptrdiff_t = raw_pattern_size_byte;
  SAFE_NALLOCA! (sa_avail, patbuf, MAX_MULTIBYTE_LENGTH, len);
  pat = patbuf;
  base_pat = raw_pattern;
  if multibyte
    {
      /* Fill patbuf by translated characters in STRING while
	 checking if we can use boyer-moore search.  If TRT is
	 non-nil, we can use boyer-moore search only if TRT can be
	 represented by the byte array of 256 elements.  For that,
	 all non-ASCII case-equivalents of all case-sensitive
	 characters in STRING must belong to the same character
	 group (two characters belong to the same group iff their
	 multibyte forms are the same except for the last byte;
	 i.e. every 64 characters form a group; U+0000..U+003F,
	 U+0040..U+007F, U+0080..U+00BF, ...).  */

      while { len -= 1; len >= 0 }
	{
	  let mut str_base: [u8; 5] = [0; MAX_MULTIBYTE_LENGTH as usize];
	  let mut str: *mut u8;
	  let mut translated: c_int = 0;
	  let mut inverse: c_int = 0;
	  let mut charlen: c_int;

	  /* If we got here and the RE flag is set, it's because we're
	     dealing with a regexp known to be trivial, so the backslash
	     just quotes the next character.  */
	  if RE && *base_pat == b'\\'
	    {
	      len -= 1;
	      raw_pattern_size -= 1;
	      len_byte -= 1;
	      base_pat = base_pat.offset (1);
	    }

	  let mut in_charlen: c_int = 0;
	  let c: c_int = string_char_and_length (base_pat, &raw mut in_charlen);

	  if NILP (trt)
	    {
	      str = base_pat;
	      charlen = in_charlen;
	    }
	  else
	    {
	      /* Translate the character.  */
	      TRANSLATE! (translated, trt, c);
	      charlen = CHAR_STRING (translated, str_base.as_mut_ptr ());
	      str = str_base.as_mut_ptr ();

	      /* Check if C has any other case-equivalents.  */
	      TRANSLATE! (inverse, inverse_trt, c);
	      /* If so, check if we can use boyer-moore.  */
	      if c != inverse && boyer_moore_ok
		{
		  /* Check if all equivalents belong to the same
		     group of characters.  Note that the check of C
		     itself is done by the last iteration.  */
		  let mut this_char_base: c_int = -1;

		  while boyer_moore_ok
		    {
		      if ASCII_CHAR_P (inverse as isize)
			{
			  if this_char_base > 0
			    { boyer_moore_ok = false; }
			  else
			    { this_char_base = 0; }
			}
		      else if CHAR_BYTE8_P (inverse)
			/* Boyer-moore search can't handle a
			   translation of an eight-bit
			   character.  */
			{ boyer_moore_ok = false; }
		      else if this_char_base < 0
			{
			  this_char_base = inverse & ! 0x3F;
			  if char_base < 0
			    { char_base = this_char_base; }
			  else if this_char_base != char_base
			    { boyer_moore_ok = false; }
			}
		      else if (inverse & ! 0x3F) != this_char_base
			{ boyer_moore_ok = false; }
		      if c == inverse
			{ break; }
		      TRANSLATE! (inverse, inverse_trt, inverse);
		    }
		}
	    }

	  /* Store this character into the translated pattern.  */
	  memcpy (pat as *mut c_void, str as *const c_void, charlen as usize);
	  pat = pat.offset (charlen as isize);
	  base_pat = base_pat.offset (in_charlen as isize);
	  len_byte -= in_charlen as isize;
	}

      /* If char_base is still negative we didn't find any translated
	 non-ASCII characters.  */
      if char_base < 0
	{ char_base = 0; }
    }
  else
    {
      /* Unibyte buffer.  */
      char_base = 0;
      while { len -= 1; len >= 0 }
	{
	  let mut c: c_int;
	  let mut translated: c_int = 0;
	  let mut inverse: c_int = 0;

	  /* If we got here and the RE flag is set, it's because we're
	     dealing with a regexp known to be trivial, so the backslash
	     just quotes the next character.  */
	  if RE && *base_pat == b'\\'
	    {
	      len -= 1;
	      raw_pattern_size -= 1;
	      base_pat = base_pat.offset (1);
	    }
	  c = *base_pat as c_int;
	  base_pat = base_pat.offset (1);
	  TRANSLATE! (translated, trt, c);
	  *pat = translated as u8;
	  pat = pat.offset (1);
	  /* Check that none of C's equivalents violates the
	     assumptions of boyer_moore.  */
	  TRANSLATE! (inverse, inverse_trt, c);
	  loop
	    {
	      if inverse >= 0o200
		{
		  boyer_moore_ok = false;
		  break;
		}
	      if c == inverse
		{ break; }
	      TRANSLATE! (inverse, inverse_trt, inverse);
	    }
	}
    }

  len_byte = pat.offset_from (patbuf);
  pat = patbuf;
  base_pat = patbuf;

  let result: EMACS_INT
    = if boyer_moore_ok
      { boyer_moore (n, pat, len_byte, trt, inverse_trt, pos_byte, lim_byte, char_base) }
    else
      { simple_search (n, pat, raw_pattern_size, len_byte, trt, pos, pos_byte, lim, lim_byte) };
  SAFE_FREE! (sa_count, sa_mark);
  return result;
}

/* Search for the Nth occurrence of STRING in the current buffer,
   from buffer position POS/POS_BYTE until LIM/LIM_BYTE.

   If RE, look for matches against the regular expression STRING instead;
   if POSIX, enable POSIX style backtracking within that regular
   expression.

   If N is positive, search forward; in this case, LIM must be greater
   than POS.

   If N is negative, search backward; LIM must be less than POS.

   Return -X if there are X remaining occurrences or matches,
   or else the position at the beginning (if N is negative) or the end
   (if N is positive) of the Nth occurrence or match against STRING.

   Use TRT and INVERSE_TRT as character translation tables.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn search_buffer (string: Lisp_Object, pos: ptrdiff_t,   // C 1518-1554
					pos_byte: ptrdiff_t, lim: ptrdiff_t,
					lim_byte: ptrdiff_t, n: EMACS_INT, RE: bool,
					trt: Lisp_Object, inverse_trt: Lisp_Object,
					posix: bool)
  -> EMACS_INT
{
  let mut pos = pos;

  if running_asynch_code
    { save_search_regs (); }

  /* Searching 0 times means don't move.  */
  /* Null string is found at starting position.  */
  if n == 0 || SCHARS (string) == 0
    {
      set_search_regs (pos_byte, 0);
      return pos;
    }

  if RE && !(trivial_regexp_p (string) && NILP (Vsearch_spaces_regexp))
    { pos = search_buffer_re (string, pos, pos_byte, lim, lim_byte, n, trt, inverse_trt, posix); }
  else
    { pos = search_buffer_non_re (string, pos, pos_byte, lim, lim_byte, n, RE, trt, inverse_trt, posix); }

  return pos;
}

/* Do a simple string search N times for the string PAT,
   whose length is LEN/LEN_BYTE,
   from buffer position POS/POS_BYTE until LIM/LIM_BYTE.
   TRT is the translation table.

   Return the character position where the match is found.
   Otherwise, if M matches remained to be found, return -M.

   This kind of search works regardless of what is in PAT and
   regardless of what is in TRT.  It is used in cases where
   boyer_moore cannot work.  */

unsafe fn simple_search (n: EMACS_INT, pat: *mut u8, len: ptrdiff_t,   // C 1556-1760
			 len_byte: ptrdiff_t, trt: Lisp_Object, pos: ptrdiff_t,
			 pos_byte: ptrdiff_t, lim: ptrdiff_t, lim_byte: ptrdiff_t)
  -> EMACS_INT
{
  let multibyte: bool = ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
  let forward: bool = n > 0;
  let mut n = n;
  let mut pos = pos;
  let mut pos_byte = pos_byte;
  /* Number of buffer bytes matched.  Note that this may be different
     from len_byte in a multibyte buffer.  */
  let mut match_byte: ptrdiff_t = PTRDIFF_MIN;

  'stop: {
    if lim > pos && multibyte
      {
	while n > 0
	  {
	    loop
	      {
		/* Try matching at position POS.  */
		let mut this_pos_byte: ptrdiff_t = pos_byte;
		let mut this_len: ptrdiff_t = len;
		let mut p: *mut u8 = pat;
		if pos + len > lim || pos_byte + len_byte > lim_byte
		  { break 'stop; }

		while this_len > 0
		  {
		    let mut charlen: c_int = 0;
		    let pat_ch: c_int = string_char_and_length (p, &raw mut charlen);
		    let mut buf_charlen: c_int = 0;
		    let mut buf_ch: c_int
		      = string_char_and_length (BYTE_POS_ADDR (this_pos_byte),
						&raw mut buf_charlen);
		    TRANSLATE! (buf_ch, trt, buf_ch);

		    if buf_ch != pat_ch
		      { break; }

		    this_len -= 1;
		    p = p.offset (charlen as isize);

		    this_pos_byte += buf_charlen as isize;
		  }

		if this_len == 0
		  {
		    match_byte = this_pos_byte - pos_byte;
		    pos += len;
		    pos_byte += match_byte;
		    break;
		  }

		inc_both (&raw mut pos, &raw mut pos_byte);
	      }

	    n -= 1;
	  }
      }
    else if lim > pos
      {
	while n > 0
	  {
	    loop
	      {
		/* Try matching at position POS.  */
		let mut this_pos: ptrdiff_t = pos;
		let mut this_len: ptrdiff_t = len;
		let mut p: *mut u8 = pat;

		if pos + len > lim
		  { break 'stop; }

		while this_len > 0
		  {
		    let pat_ch: c_int = *p as c_int;
		    p = p.offset (1);
		    let mut buf_ch: c_int = FETCH_BYTE! (this_pos) as c_int;
		    TRANSLATE! (buf_ch, trt, buf_ch);

		    if buf_ch != pat_ch
		      { break; }

		    this_len -= 1;
		    this_pos += 1;
		  }

		if this_len == 0
		  {
		    match_byte = len;
		    pos += len;
		    break;
		  }

		pos += 1;
	      }

	    n -= 1;
	  }
      }
    /* Backwards search.  */
    else if lim < pos && multibyte
      {
	while n < 0
	  {
	    loop
	      {
		/* Try matching at position POS.  */
		let mut this_pos: ptrdiff_t = pos;
		let mut this_pos_byte: ptrdiff_t = pos_byte;
		let mut this_len: ptrdiff_t = len;
		let mut p: *const u8 = pat.offset (len_byte);

		if this_pos - len < lim || (pos_byte - len_byte) < lim_byte
		  { break 'stop; }

		while this_len > 0
		  {
		    let mut pat_ch: c_int;
		    let mut buf_ch: c_int;

		    dec_both (&raw mut this_pos, &raw mut this_pos_byte);
		    p = p.offset (-(raw_prev_char_len (p) as isize));
		    pat_ch = STRING_CHAR (p);
		    buf_ch = STRING_CHAR (BYTE_POS_ADDR (this_pos_byte) as *const u8);
		    TRANSLATE! (buf_ch, trt, buf_ch);

		    if buf_ch != pat_ch
		      { break; }

		    this_len -= 1;
		  }

		if this_len == 0
		  {
		    match_byte = pos_byte - this_pos_byte;
		    pos = this_pos;
		    pos_byte = this_pos_byte;
		    break;
		  }

		dec_both (&raw mut pos, &raw mut pos_byte);
	      }

	    n += 1;
	  }
      }
    else if lim < pos
      {
	while n < 0
	  {
	    loop
	      {
		/* Try matching at position POS.  */
		let mut this_pos: ptrdiff_t = pos - len;
		let mut this_len: ptrdiff_t = len;
		let mut p: *mut u8 = pat;

		if this_pos < lim
		  { break 'stop; }

		while this_len > 0
		  {
		    let pat_ch: c_int = *p as c_int;
		    p = p.offset (1);
		    let mut buf_ch: c_int = FETCH_BYTE! (this_pos) as c_int;
		    TRANSLATE! (buf_ch, trt, buf_ch);

		    if buf_ch != pat_ch
		      { break; }
		    this_len -= 1;
		    this_pos += 1;
		  }

		if this_len == 0
		  {
		    match_byte = len;
		    pos -= len;
		    break;
		  }

		pos -= 1;
	      }

	    n += 1;
	  }
      }
  }

  if n == 0
    {
      eassert! (match_byte != PTRDIFF_MIN);
      if forward
	{ set_search_regs (if multibyte { pos_byte } else { pos } - match_byte, match_byte); }
      else
	{ set_search_regs (if multibyte { pos_byte } else { pos }, match_byte); }

      return pos;
    }
  else if n > 0
    { return -n; }
  else
    { return n; }
}

/* Do Boyer-Moore search N times for the string BASE_PAT,
   whose length is LEN_BYTE,
   from buffer position POS_BYTE until LIM_BYTE.
   DIRECTION says which direction we search in.
   TRT and INVERSE_TRT are translation tables.
   Characters in PAT are already translated by TRT.

   This kind of search works if all the characters in BASE_PAT that
   have nontrivial translation are the same aside from the last byte.
   This makes it possible to translate just the last byte of a
   character, and do so after just a simple test of the context.
   CHAR_BASE is nonzero if there is such a non-ASCII character.

   If that criterion is not satisfied, do not call this function.  */

unsafe fn boyer_moore (n: EMACS_INT, base_pat: *mut u8, len_byte: ptrdiff_t,   // C 1762-2176
		       trt: Lisp_Object, inverse_trt: Lisp_Object, pos_byte: ptrdiff_t,
		       lim_byte: ptrdiff_t, char_base: c_int)
  -> EMACS_INT
{
  let direction: c_int = if n > 0 { 1 } else { -1 };
  let mut n = n;
  let mut pos_byte = pos_byte;
  let mut base_pat = base_pat;
  let dirlen: ptrdiff_t;
  let mut limit: ptrdiff_t;
  let mut stride_for_teases: c_int = 0;
  let mut BM_tab: [c_int; 256] = [0; 256];                     // 0400
  let mut cursor: *mut u8;
  let mut p_limit: *mut u8;
  let mut i: ptrdiff_t;
  let mut j: c_int;
  let mut pat: *mut u8;
  let pat_end: *mut u8;
  let multibyte: bool = ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));

  let mut simple_translate: [u8; 256] = [0; 256];              // 0400
  /* These are set to the preceding bytes of a byte to be translated
     if char_base is nonzero.  As the maximum byte length of a
     multibyte character is 5, we have to check at most four previous
     bytes.  */
  let mut translate_prev_byte1: c_int = 0;
  let mut translate_prev_byte2: c_int = 0;
  let mut translate_prev_byte3: c_int = 0;

  /* The general approach is that we are going to maintain that we know
     the first (closest to the present position, in whatever direction
     we're searching) character that could possibly be the last
     (furthest from present position) character of a valid match.  We
     advance the state of our knowledge by looking at that character
     and seeing whether it indeed matches the last character of the
     pattern.  If it does, we take a closer look.  If it does not, we
     move our pointer (to putative last characters) as far as is
     logically possible.  This amount of movement, which I call a
     stride, will be the length of the pattern if the actual character
     appears nowhere in the pattern, otherwise it will be the distance
     from the last occurrence of that character to the end of the
     pattern.  If the amount is zero we have a possible match.  */

  /* Here we make a "mickey mouse" BM table.  The stride of the search
     is determined only by the last character of the putative match.
     If that character does not match, we will stride the proper
     distance to propose a match that superimposes it on the last
     instance of a character that matches it (per trt), or misses
     it entirely if there is none. */

  dirlen = len_byte * direction as isize;

  /* Record position after the end of the pattern.  */
  pat_end = base_pat.offset (len_byte);
  /* BASE_PAT points to a character that we start scanning from.
     It is the first character in a forward search,
     the last character in a backward search.  */
  if direction < 0
    { base_pat = pat_end.offset (-1); }

  /* A character that does not appear in the pattern induces a
     stride equal to the pattern length.  */
  i = 0;
  while i < 0o400
    {
      BM_tab[i as usize] = dirlen as c_int;
      i += 1;
    }

  /* We use this for translation, instead of TRT itself.
     We fill this in to handle the characters that actually
     occur in the pattern.  Others don't matter anyway!  */
  i = 0;
  while i < 0o400
    {
      simple_translate[i as usize] = i as u8;
      i += 1;
    }

  if char_base != 0
    {
      /* Setup translate_prev_byte1/2/3/4 from CHAR_BASE.  Only a
	 byte following them are the target of translation.  */
      eassume! (0x80 <= char_base && char_base <= MAX_CHAR);
      let mut str: [u8; 5] = [0; MAX_MULTIBYTE_LENGTH as usize];
      let cblen: c_int = CHAR_STRING (char_base, str.as_mut_ptr ());

      translate_prev_byte1 = str[(cblen - 2) as usize] as c_int;
      if cblen > 2
	{
	  translate_prev_byte2 = str[(cblen - 3) as usize] as c_int;
	  if cblen > 3
	    { translate_prev_byte3 = str[(cblen - 4) as usize] as c_int; }
	}
    }

  i = 0;
  while i != dirlen
    {
      let mut ptr: *mut u8 = base_pat.offset (i as isize);
      i += direction as isize;
      if ! NILP (trt)
	{
	  /* If the byte currently looking at is the last of a
	     character to check case-equivalents, set CH to that
	     character.  An ASCII character and a non-ASCII character
	     matching with CHAR_BASE are to be checked.  */
	  let mut ch: c_int = -1;

	  if ASCII_CHAR_P (*ptr as isize) || ! multibyte
	    { ch = *ptr as c_int; }
	  else if char_base != 0
		   && (pat_end.offset_from (ptr) == 1 || CHAR_HEAD_P (*ptr.offset (1) as c_int))
	    {
	      let mut charstart: *mut u8 = ptr.offset (-1);

	      while !(CHAR_HEAD_P (*charstart as c_int))
		{ charstart = charstart.offset (-1); }
	      ch = STRING_CHAR (charstart);
	      if char_base != (ch & ! 0x3F)
		{ ch = -1; }
	    }

	  if ch >= 0o200 && multibyte
	    { j = (ch & 0x3F) | 0o200; }
	  else
	    { j = *ptr as c_int; }

	  if i == dirlen
	    { stride_for_teases = BM_tab[j as usize]; }

	  BM_tab[j as usize] = (dirlen - i) as c_int;
	  /* A translation table is accompanied by its inverse -- see
	     comment following downcase_table for details.  */
	  if ch >= 0
	    {
	      let starting_ch: c_int = ch;
	      let starting_j: c_int = j;

	      loop
		{
		  TRANSLATE! (ch, inverse_trt, ch);
		  if ch >= 0o200 && multibyte
		    { j = (ch & 0x3F) | 0o200; }
		  else
		    { j = ch; }

		  /* For all the characters that map into CH,
		     set up simple_translate to map the last byte
		     into STARTING_J.  */
		  simple_translate[j as usize] = starting_j as u8;
		  if ch == starting_ch
		    { break; }
		  BM_tab[j as usize] = (dirlen - i) as c_int;
		}
	    }
	}
      else
	{
	  j = *ptr as c_int;

	  if i == dirlen
	    { stride_for_teases = BM_tab[j as usize]; }
	  BM_tab[j as usize] = (dirlen - i) as c_int;
	}
      /* stride_for_teases tells how much to stride if we get a
	 match on the far character but are subsequently
	 disappointed, by recording what the stride would have been
	 for that character if the last character had been
	 different.  */
    }
  pos_byte += dirlen - (if direction > 0 { direction as isize } else { 0 });
  /* loop invariant - POS_BYTE points at where last char (first
     char if reverse) of pattern would align in a possible match.  */
  while n != 0
    {
      let mut tail_end: ptrdiff_t;
      let mut tail_end_ptr: *mut u8;

      /* It's been reported that some (broken) compiler thinks that
	 Boolean expressions in an arithmetic context are unsigned.
	 Using an explicit ?1:0 prevents this.  */
      if ((lim_byte - pos_byte - (if direction > 0 { 1 } else { 0 })) * direction as isize)
	  < 0
	{ return n * (0 - direction) as EMACS_INT; }
      /* First we do the part we can by pointers (maybe nothing) */
      maybe_quit ();
      pat = base_pat;
      limit = pos_byte - dirlen + direction as isize;
      if direction > 0
	{
	  limit = BUFFER_CEILING_OF (limit);
	  /* LIMIT is now the last (not beyond-last!) value POS_BYTE
	     can take on without hitting edge of buffer or the gap.  */
	  limit = min (limit, pos_byte + 20000);
	  limit = min (limit, lim_byte - 1);
	}
      else
	{
	  limit = BUFFER_FLOOR_OF (limit);
	  /* LIMIT is now the last (not beyond-last!) value POS_BYTE
	     can take on without hitting edge of buffer or the gap.  */
	  limit = max (limit, pos_byte - 20000);
	  limit = max (limit, lim_byte);
	}
      tail_end = BUFFER_CEILING_OF (pos_byte) + 1;
      tail_end_ptr = BYTE_POS_ADDR (tail_end);

      if (limit - pos_byte) * direction as isize > 20
	{
	  let mut p2: *mut u8;

	  p_limit = BYTE_POS_ADDR (limit);
	  cursor = BYTE_POS_ADDR (pos_byte);
	  p2 = cursor;
	  /* In this loop, pos + cursor - p2 is the surrogate for pos.  */
	  loop			/* use one cursor setting as long as i can */
	    {
	      let mut hit: bool = false;
	      if direction > 0	/* worth duplicating */
		{
		  while cursor <= p_limit
		    {
		      if BM_tab[*cursor as usize] == 0
			{ hit = true; break; }
		      cursor = cursor.offset (BM_tab[*cursor as usize] as isize);
		    }
		}
	      else
		{
		  while cursor >= p_limit
		    {
		      if BM_tab[*cursor as usize] == 0
			{ hit = true; break; }
		      cursor = cursor.offset (BM_tab[*cursor as usize] as isize);
		    }
		}
	      /* If you are here, cursor is beyond the end of the
		 searched region.  You fail to match within the
		 permitted region and would otherwise try a character
		 beyond that region.  */
	      if ! hit
		{ break; }

	      /* hit: */
	      i = dirlen - direction as isize;
	      if ! NILP (trt)
		{
		  loop
		    {
		      i -= direction as isize;
		      if i + direction as isize == 0
			{ break; }
		      let ch: c_int;
		      cursor = cursor.offset (-(direction as isize));
		      /* Translate only the last byte of a character.  */
		      if ! multibyte
			  || ((cursor == tail_end_ptr
			       || CHAR_HEAD_P (*cursor.offset (1) as c_int))
			      && (CHAR_HEAD_P (*cursor as c_int)
				  /* Check if this is the last byte of
				     a translatable character.  */
				  || (translate_prev_byte1 == *cursor.offset (-1) as c_int
				      && (CHAR_HEAD_P (translate_prev_byte1)
					  || (translate_prev_byte2 == *cursor.offset (-2) as c_int
					      && (CHAR_HEAD_P (translate_prev_byte2)
						  || (translate_prev_byte3 == *cursor.offset (-3) as c_int)))))))
			{ ch = simple_translate[*cursor as usize] as c_int; }
		      else
			{ ch = *cursor as c_int; }
		      if *pat.offset (i as isize) as c_int != ch
			{ break; }
		    }
		}
	      else
		{
		  loop
		    {
		      i -= direction as isize;
		      if i + direction as isize == 0
			{ break; }
		      cursor = cursor.offset (-(direction as isize));
		      if *pat.offset (i as isize) != *cursor
			{ break; }
		    }
		}
	      cursor = cursor.offset (dirlen - i - direction as isize);	/* fix cursor */
	      if i + direction as isize == 0
		{
		  let mut position: ptrdiff_t;
		  let mut start: ptrdiff_t;
		  let mut end: ptrdiff_t;
		  #[cfg (feature = "rel_alloc")]
		  let cursor_off: ptrdiff_t;

		  cursor = cursor.offset (-(direction as isize));

		  position = pos_byte + cursor.offset_from (p2)
		    + (if direction > 0 { 1 - len_byte } else { 0 });
		  #[cfg (feature = "rel_alloc")]
		  /* set_search_regs might call malloc, which could
		     cause ralloc.c relocate buffer text.  We need to
		     update pointers into buffer text due to that.  */
		  { cursor_off = cursor.offset_from (p2); }
		  set_search_regs (position, len_byte);
		  #[cfg (feature = "rel_alloc")]
		  {
		    p_limit = BYTE_POS_ADDR (limit);
		    p2 = BYTE_POS_ADDR (pos_byte);
		    cursor = p2.offset (cursor_off);
		  }

		  if NILP (Vinhibit_changing_match_data)
		    {
		      start = *search_regs! ().start.offset (0);
		      end = *search_regs! ().end.offset (0);
		    }
		  else
		    /* If Vinhibit_changing_match_data is non-nil,
		       search_regs will not be changed.  So let's
		       compute start and end here.  */
		    {
		      start = BYTE_TO_CHAR (position);
		      end = BYTE_TO_CHAR (position + len_byte);
		    }

		  n -= direction as isize;
		  if n != 0
		    { cursor = cursor.offset (dirlen as isize); } /* to resume search */
		  else
		    { return if direction > 0 { end } else { start }; }
		}
	      else
		{ cursor = cursor.offset (stride_for_teases as isize); } /* <sigh> we lose -  */
	    }
	  pos_byte += cursor.offset_from (p2);
	}
      else
	/* Now we'll pick up a clump that has to be done the hard
	   way because it covers a discontinuity.  */
	{
	  limit = if direction > 0
	  { BUFFER_CEILING_OF (pos_byte - dirlen + 1) }
	  else
	  { BUFFER_FLOOR_OF (pos_byte - dirlen - 1) };
	  limit = if direction > 0
	  { min (limit + len_byte, lim_byte - 1) }
	  else
	  { max (limit - len_byte, lim_byte) };
	  /* LIMIT is now the last value POS_BYTE can have
	     and still be valid for a possible match.  */
	  loop
	    {
	      let mut hit2: bool = false;
	      /* This loop can be coded for space rather than
		 speed because it will usually run only once.
		 (the reach is at most len + 21, and typically
		 does not exceed len).  */
	      while (limit - pos_byte) * direction as isize >= 0
		{
		  let ch: c_int = FETCH_BYTE! (pos_byte) as c_int;
		  if BM_tab[ch as usize] == 0
		    { hit2 = true; break; }
		  pos_byte += BM_tab[ch as usize] as isize;
		}
	      if ! hit2
		{ break; }	/* ran off the end */

	      /* hit2: */
	      /* Found what might be a match.  */
	      i = dirlen - direction as isize;
	      loop
		{
		  let ch: c_int;
		  let mut ptr: *mut u8;
		  i -= direction as isize;
		  if i + direction as isize == 0
		    { break; }
		  pos_byte -= direction as isize;
		  ptr = BYTE_POS_ADDR (pos_byte);
		  /* Translate only the last byte of a character.  */
		  if ! multibyte
		      || ((ptr == tail_end_ptr
			   || CHAR_HEAD_P (*ptr.offset (1) as c_int))
			  && (CHAR_HEAD_P (*ptr as c_int)
			      /* Check if this is the last byte of a
				 translatable character.  */
			      || (translate_prev_byte1 == *ptr.offset (-1) as c_int
				  && (CHAR_HEAD_P (translate_prev_byte1)
				      || (translate_prev_byte2 == *ptr.offset (-2) as c_int
					  && (CHAR_HEAD_P (translate_prev_byte2)
					      || translate_prev_byte3 == *ptr.offset (-3) as c_int))))))
		    { ch = simple_translate[*ptr as usize] as c_int; }
		  else
		    { ch = *ptr as c_int; }
		  if *pat.offset (i as isize) as c_int != ch
		    { break; }
		}
	      /* Above loop has moved POS_BYTE part or all the way
		 back to the first pos (last pos if reverse).
		 Set it once again at the last (first if reverse) char.  */
	      pos_byte += dirlen - i - direction as isize;
	      if i + direction as isize == 0
		{
		  let mut position: ptrdiff_t;
		  let mut start: ptrdiff_t;
		  let mut end: ptrdiff_t;
		  pos_byte -= direction as isize;

		  position = pos_byte + (if direction > 0 { 1 - len_byte } else { 0 });
		  set_search_regs (position, len_byte);

		  if NILP (Vinhibit_changing_match_data)
		    {
		      start = *search_regs! ().start.offset (0);
		      end = *search_regs! ().end.offset (0);
		    }
		  else
		    /* If Vinhibit_changing_match_data is non-nil,
		       search_regs will not be changed.  So let's
		       compute start and end here.  */
		    {
		      start = BYTE_TO_CHAR (position);
		      end = BYTE_TO_CHAR (position + len_byte);
		    }

		  n -= direction as isize;
		  if n != 0
		    { pos_byte += dirlen; } /* to resume search */
		  else
		    { return if direction > 0 { end } else { start }; }
		}
	      else
		{ pos_byte += stride_for_teases as isize; }
	    }
	}
      /* We have done one clump.  Can we continue? */
      if ((lim_byte - pos_byte) * direction as isize) < 0
	{ return (0 - n) * direction as EMACS_INT; }
    }
  return BYTE_TO_CHAR (pos_byte);
}

/* Record beginning BEG_BYTE and end BEG_BYTE + NBYTES
   for the overall match just found in the current buffer.
   Also clear out the match data for registers 1 and up.  */

unsafe fn set_search_regs (beg_byte: ptrdiff_t, nbytes: ptrdiff_t)   // C 2178-2204
{
  let mut i: ptrdiff_t;

  if !NILP (Vinhibit_changing_match_data)
    { return; }

  /* Make sure we have registers in which to store
     the match position.  */
  if search_regs! ().num_regs == 0
    {
      search_regs! ().start = xmalloc (2 * std::mem::size_of::<ptrdiff_t> ()) as *mut ptrdiff_t;
      search_regs! ().end = xmalloc (2 * std::mem::size_of::<ptrdiff_t> ()) as *mut ptrdiff_t;
      search_regs! ().num_regs = 2;
    }

  /* Clear out the other registers.  */
  i = 1;
  while i < search_regs! ().num_regs
    {
      *search_regs! ().start.offset (i as isize) = -1;
      *search_regs! ().end.offset (i as isize) = -1;
      i += 1;
    }

  *search_regs! ().start.offset (0) = BYTE_TO_CHAR (beg_byte);
  *search_regs! ().end.offset (0) = BYTE_TO_CHAR (beg_byte + nbytes);
  XSETBUFFER! (last_thing_searched! (), (current_buffer! ()));
}

DEFUN! ("search-backward", Fsearch_backward, Ssearch_backward, 1, 4,      // C 2206-2216
       "MSearch backward: ",
       doc: "Search backward from point for STRING.\nThis function is almost identical to `search-forward', except that\nby default it searches backward instead of forward, and the sign of\nCOUNT also indicates exactly the opposite searching direction.\nSee `search-forward' for details.",
       (string: Lisp_Object, bound: Lisp_Object, noerror: Lisp_Object, count: Lisp_Object),
       {
	 return search_command (string, bound, noerror, count, -1, false, false);
       });

DEFUN! ("search-forward", Fsearch_forward, Ssearch_forward, 1, 4,         // C 2218-2246
       "MSearch: ",
       doc: "Search forward from point for STRING.\nSet point to the end of the occurrence found, and return point.\nThe optional second argument BOUND is a buffer position that bounds\n  the search.  The match found must not end after that position.  A\n  value of nil means search to the end of the accessible portion of\n  the buffer.\nThe optional third argument NOERROR indicates how errors are handled\n  when the search fails: if it is nil or omitted, emit an error; if\n  it is t, simply return nil and do nothing; if it is neither nil nor\n  t, move to the limit of search and return nil.\nThe optional fourth argument COUNT is a number that indicates the\n  search direction and the number of occurrences to search for.  If it\n  is positive, search forward for COUNT successive occurrences; if it\n  is negative, search backward, instead of forward, for -COUNT\n  occurrences.  A value of nil means the same as 1.\nWith COUNT positive/negative, the match found is the COUNTth/-COUNTth\n  one in the buffer located entirely after/before the origin of the\n  search.\n\nSearch case-sensitivity is determined by the value of the variable\n`case-fold-search', which see.\n\nSee also the functions `match-beginning', `match-end', `match-string',\nand `replace-match'.",
       (string: Lisp_Object, bound: Lisp_Object, noerror: Lisp_Object, count: Lisp_Object),
       {
	 return search_command (string, bound, noerror, count, 1, false, false);
       });

DEFUN! ("re-search-backward", Fre_search_backward, Sre_search_backward, 1, 4,   // C 2248-2262
       "sRE search backward: ",
       doc: "Search backward from point for regular expression REGEXP.\nThis function is almost identical to `re-search-forward', except that\nby default it searches backward instead of forward, and the sign of\nCOUNT also indicates exactly the opposite searching direction.\nSee `re-search-forward' for details.\n\nNote that searching backwards may give a shorter match than expected,\nbecause REGEXP is still matched in the forward direction.  See Info\nanchor `(elisp) re-search-backward' for details.",
       (regexp: Lisp_Object, bound: Lisp_Object, noerror: Lisp_Object, count: Lisp_Object),
       {
	 return search_command (regexp, bound, noerror, count, -1, true, false);
       });

DEFUN! ("re-search-forward", Fre_search_forward, Sre_search_forward, 1, 4,   // C 2264-2293
       "sRE search: ",
       doc: "Search forward from point for regular expression REGEXP.\nSet point to the end of the occurrence found, and return point.\nThe optional second argument BOUND is a buffer position that bounds\n  the search.  The match found must not end after that position.  A\n  value of nil means search to the end of the accessible portion of\n  the buffer.\nThe optional third argument NOERROR indicates how errors are handled\n  when the search fails: if it is nil or omitted, emit an error; if\n  it is t, simply return nil and do nothing; if it is neither nil nor\n  t, move to the limit of search and return nil.\nThe optional fourth argument COUNT is a number that indicates the\n  search direction and the number of occurrences to search for.  If it\n  is positive, search forward for COUNT successive occurrences; if it\n  is negative, search backward, instead of forward, for -COUNT\n  occurrences.  A value of nil means the same as 1.\nWith COUNT positive/negative, the match found is the COUNTth/-COUNTth\n  one in the buffer located entirely after/before the origin of the\n  search.\n\nSearch case-sensitivity is determined by the value of the variable\n`case-fold-search', which see.\n\nSee also the functions `match-beginning', `match-end', `match-string',\nand `replace-match'.",
       (regexp: Lisp_Object, bound: Lisp_Object, noerror: Lisp_Object, count: Lisp_Object),
       {
	 return search_command (regexp, bound, noerror, count, 1, true, false);
       });

DEFUN! ("posix-search-backward", Fposix_search_backward, Sposix_search_backward, 1, 4,  // C 2295-2310
       "sPosix search backward: ",
       doc: "Search backward from point for match for REGEXP according to Posix rules.\nFind the longest match in accord with Posix regular expression rules.\nThis function is almost identical to `posix-search-forward', except that\nby default it searches backward instead of forward, and the sign of\nCOUNT also indicates exactly the opposite searching direction.\nSee `posix-search-forward' for details.\n\nNote that searching backwards may give a shorter match than expected,\nbecause REGEXP is still matched in the forward direction.  See Info\nanchor `(elisp) re-search-backward' for details.",
       (regexp: Lisp_Object, bound: Lisp_Object, noerror: Lisp_Object, count: Lisp_Object),
       {
	 return search_command (regexp, bound, noerror, count, -1, true, true);
       });

DEFUN! ("posix-search-forward", Fposix_search_forward, Sposix_search_forward, 1, 4,  // C 2312-2342
       "sPosix search: ",
       doc: "Search forward from point for REGEXP according to Posix rules.\nFind the longest match in accord with Posix regular expression rules.\nSet point to the end of the occurrence found, and return point.\nThe optional second argument BOUND is a buffer position that bounds\n  the search.  The match found must not end after that position.  A\n  value of nil means search to the end of the accessible portion of\n  the buffer.\nThe optional third argument NOERROR indicates how errors are handled\n  when the search fails: if it is nil or omitted, emit an error; if\n  it is t, simply return nil and do nothing; if it is neither nil nor\n  t, move to the limit of search and return nil.\nThe optional fourth argument COUNT is a number that indicates the\n  search direction and the number of occurrences to search for.  If it\n  is positive, search forward for COUNT successive occurrences; if it\n  is negative, search backward, instead of forward, for -COUNT\n  occurrences.  A value of nil means the same as 1.\nWith COUNT positive/negative, the match found is the COUNTth/-COUNTth\n  one in the buffer located entirely after/before the origin of the\n  search.\n\nSearch case-sensitivity is determined by the value of the variable\n`case-fold-search', which see.\n\nSee also the functions `match-beginning', `match-end', `match-string',\nand `replace-match'.",
       (regexp: Lisp_Object, bound: Lisp_Object, noerror: Lisp_Object, count: Lisp_Object),
       {
	 return search_command (regexp, bound, noerror, count, 1, true, true);
       });

/* C 2344-2767: `enum { nochange, all_caps, cap_initial }` is a function-local enum in C; the
   port keeps its three members as an enum so the names C's body uses stay in the text.  */
#[derive (Clone, Copy, PartialEq)]
enum case_action_kind { nochange, all_caps, cap_initial }
#[allow(unused_imports)]
use case_action_kind::*;

DEFUN! ("replace-match", Freplace_match, Sreplace_match, 1, 5, 0,        // C 2344-2767
       doc: "Replace text matched by last search with NEWTEXT.\nLeave point at the end of the replacement text.\n\nIf optional second arg FIXEDCASE is non-nil, do not alter the case of\nthe replacement text.  Otherwise, maybe capitalize the whole text, or\nmaybe just word initials, based on the replaced text.  If the replaced\ntext has only capital letters and has at least one multiletter word,\nconvert NEWTEXT to all caps.  Otherwise if all words are capitalized\nin the replaced text, capitalize each word in NEWTEXT.  Note that\nwhat exactly is a word is determined by the syntax tables in effect\nin the current buffer, and the variable `case-symbols-as-words'.\n\nIf optional third arg LITERAL is non-nil, insert NEWTEXT literally.\nOtherwise treat `\\' as special:\n  `\\&' in NEWTEXT means substitute original matched text.\n  `\\N' means substitute what matched the Nth `\\(...\\)'.\n       If Nth parens didn't match, substitute nothing.\n  `\\\\' means insert one `\\'.\n  `\\?' is treated literally\n       (for compatibility with `query-replace-regexp').\n  Any other character following `\\' signals an error.\nCase conversion does not apply to these substitutions.\n\nIf optional fourth argument STRING is non-nil, it should be a string\nto act on; this should be the string on which the previous match was\ndone via `string-match'.  In this case, `replace-match' creates and\nreturns a new string, made by copying STRING and replacing the part of\nSTRING that was matched (the original STRING itself is not altered).\n\nThe optional fifth argument SUBEXP specifies a subexpression;\nit says to replace just that subexpression with NEWTEXT,\nrather than replacing the entire matched text.\nThis is, in a vague sense, the inverse of using `\\N' in NEWTEXT;\n`\\N' copies subexp N into NEWTEXT, but using N as SUBEXP puts\nNEWTEXT in place of subexp N.\nThis is useful only after a regular expression search or match,\nsince only regular expressions have distinguished subexpressions.",
       (newtext: Lisp_Object, fixedcase: Lisp_Object, literal: Lisp_Object,
	string: Lisp_Object, subexp: Lisp_Object),
       {
	 let mut newtext: Lisp_Object = newtext;
	 let mut case_action: case_action_kind;
	 let mut pos: ptrdiff_t;
	 let mut pos_byte: ptrdiff_t;
	 let mut some_multiletter_word: bool;
	 let mut some_lowercase: bool;
	 let mut some_uppercase: bool;
	 let mut some_nonuppercase_initial: bool;
	 let mut c: c_int;
	 let mut prevc: c_int;
	 let mut sub: ptrdiff_t;
	 let opoint: EMACS_INT;
	 let mut newpoint: ptrdiff_t;
	 let mut literal: Lisp_Object = literal;

	 CHECK_STRING (newtext);

	 if ! NILP (string)
	   { CHECK_STRING (string); }

	 /* Most replacement texts don't contain any backslash directives in
	    the replacements.  Check whether that's the case, which will
	    enable us to take the fast path later.  */
	 if NILP (literal)
	     && memchr (SSDATA (newtext) as *const c_void, b'\\' as c_int,
			SBYTES (newtext) as usize).is_null ()
	   { literal = Qt (); }

	 case_action = nochange;	/* We tried an initialization */
					/* but some C compilers blew it */

	 let mut num_regs: ptrdiff_t = search_regs! ().num_regs;
	 if num_regs <= 0
	   { error (c"`replace-match' called before any match found".as_ptr ()); }

	 sub = if !NILP (subexp) { check_integer_range (subexp, 0, (num_regs - 1) as intmax_t) as ptrdiff_t } else { 0 };
	 let sub_start: ptrdiff_t = *search_regs! ().start.offset (sub as isize);
	 let sub_end: ptrdiff_t = *search_regs! ().end.offset (sub as isize);
	 eassert! (sub_start <= sub_end);

	 /* Check whether the text to replace is present in the buffer/string.  */
	 if !(if NILP (string)
	     { BEGV! () <= sub_start && sub_end <= ZV! () } else { 0 <= sub_start && sub_end <= SCHARS (string) })
	   {
	     if sub_start < 0
	       { xsignal2 (Qerror (),
			   build_string (c"replace-match subexpression does not exist".as_ptr ()),
			   subexp); }
	     args_out_of_range (make_fixnum (sub_start), make_fixnum (sub_end));
	   }

	 if NILP (fixedcase)
	   {
	     /* Decide how to casify by examining the matched text. */
	     let last: ptrdiff_t;

	     pos = sub_start;
	     last = sub_end;

	     if NILP (string)
	       { pos_byte = CHAR_TO_BYTE (pos); }
	     else
	       { pos_byte = string_char_to_byte (string, pos); }

	     prevc = b'\n' as c_int;
	     case_action = all_caps;

	     /* some_multiletter_word is set nonzero if any original word
		is more than one letter long. */
	     some_multiletter_word = false;
	     some_lowercase = false;
	     some_nonuppercase_initial = false;
	     some_uppercase = false;

	     while pos < last
	       {
		 if NILP (string)
		   {
		     c = FETCH_CHAR_AS_MULTIBYTE (pos_byte);
		     inc_both (&raw mut pos, &raw mut pos_byte);
		   }
		 else
		   { c = fetch_string_char_as_multibyte_advance (string, &raw mut pos, &raw mut pos_byte); }

		 if lowercasep (c)
		   {
		     /* Cannot be all caps if any original char is lower case */

		     some_lowercase = true;
		     if SYNTAX (prevc) != Sword
			 && !(case_symbols_as_words && SYNTAX (prevc) == Ssymbol)
		       { some_nonuppercase_initial = true; }
		     else
		       { some_multiletter_word = true; }
		   }
		 else if uppercasep (c)
		   {
		     some_uppercase = true;
		     if SYNTAX (prevc) != Sword
			 && !(case_symbols_as_words && SYNTAX (prevc) == Ssymbol)
		       {}
		     else
		       { some_multiletter_word = true; }
		   }
		 else
		   {
		     /* If the initial is a caseless word constituent,
			treat that like a lowercase initial.  */
		     if SYNTAX (prevc) != Sword
			 && !(case_symbols_as_words && SYNTAX (prevc) == Ssymbol)
		       { some_nonuppercase_initial = true; }
		   }

		 prevc = c;
	       }

	     /* Convert to all caps if the old text is all caps
		and has at least one multiletter word.  */
	     if ! some_lowercase && some_multiletter_word
	       { case_action = all_caps; }
	     /* Capitalize each word, if the old text has all capitalized words.  */
	     else if !some_nonuppercase_initial && some_multiletter_word
	       { case_action = cap_initial; }
	     else if !some_nonuppercase_initial && some_uppercase
	       /* Should x -> yz, operating on X, give Yz or YZ?
		  We'll assume the latter.  */
	       { case_action = all_caps; }
	     else
	       { case_action = nochange; }
	   }

	 /* Do replacement in a string.  */
	 if !NILP (string)
	   {
	     let before: Lisp_Object;
	     let mut after: Lisp_Object;

	     before = Fsubstring (string, make_fixnum (0), make_fixnum (sub_start));
	     after = Fsubstring (string, make_fixnum (sub_end), Qnil ());

	     /* Substitute parts of the match into NEWTEXT
		if desired.  */
	     if NILP (literal)
	       {
		 let mut lastpos: ptrdiff_t = 0;
		 let mut lastpos_byte: ptrdiff_t = 0;
		 /* We build up the substituted string in ACCUM.  */
		 let mut accum: Lisp_Object;
		 let mut middle: Lisp_Object;
		 let length: ptrdiff_t = SBYTES (newtext);

		 accum = Qnil ();

		 pos_byte = 0;
		 pos = 0;
		 while pos_byte < length
		   {
		     let mut substart: ptrdiff_t = -1;
		     let mut subend: ptrdiff_t = 0;
		     let mut delbackslash: bool = false;

		     c = fetch_string_char_advance (newtext, &raw mut pos, &raw mut pos_byte);

		     if c == b'\\' as c_int
		       {
			 c = fetch_string_char_advance (newtext, &raw mut pos, &raw mut pos_byte);

			 if c == b'&' as c_int
			   {
			     substart = sub_start;
			     subend = sub_end;
			   }
			 else if c >= b'1' as c_int && c <= b'9' as c_int
			   {
			     if (c - b'0' as c_int) < num_regs as c_int
				 && *search_regs! ().start.offset ((c - b'0' as c_int) as isize) >= 0
			       {
				 substart = *search_regs! ().start.offset ((c - b'0' as c_int) as isize);
				 subend = *search_regs! ().end.offset ((c - b'0' as c_int) as isize);
			       }
			     else
			       {
				 /* If that subexp did not match,
				    replace \\N with nothing.  */
				 substart = 0;
				 subend = 0;
			       }
			   }
			 else if c == b'\\' as c_int
			   { delbackslash = true; }
			 else if c != b'?' as c_int
			   { error (c"Invalid use of `\\' in replacement text".as_ptr ()); }
		       }
		     if substart >= 0
		       {
			 if pos - 2 != lastpos
			   { middle = substring_both (newtext, lastpos,
						      lastpos_byte,
						      pos - 2, pos_byte - 2); }
			 else
			   { middle = Qnil (); }
			 accum = concat3 (accum, middle,
					  Fsubstring (string,
						      make_fixnum (substart),
						      make_fixnum (subend)));
			 lastpos = pos;
			 lastpos_byte = pos_byte;
		       }
		     else if delbackslash
		       {
			 middle = substring_both (newtext, lastpos,
						  lastpos_byte,
						  pos - 1, pos_byte - 1);

			 accum = concat2 (accum, middle);
			 lastpos = pos;
			 lastpos_byte = pos_byte;
		       }
		   }

		 if pos != lastpos
		   { middle = substring_both (newtext, lastpos,
					      lastpos_byte,
					      pos, pos_byte); }
		 else
		   { middle = Qnil (); }

		 newtext = concat2 (accum, middle);
	       }

	     /* Do case substitution in NEWTEXT if desired.  */
	     if case_action == all_caps
	       { newtext = Fupcase (newtext); }
	     else if case_action == cap_initial
	       { newtext = Fupcase_initials (newtext); }

	     return concat3 (before, newtext, after);
	   }

	 /* Record point.  A nonpositive OPOINT is actually an offset from ZV.  */
	 opoint = if PT! () <= sub_start { PT! () } else { max (PT! (), sub_end) - ZV! () };

	 /* If we want non-literal replacement,
	    perform substitution on the replacement string.  */
	 if NILP (literal)
	   {
	     let length: ptrdiff_t = SBYTES (newtext);
	     let mut substed: *mut u8;
	     let mut substed_alloc_size: ptrdiff_t;
	     let mut substed_len: ptrdiff_t;
	     let buf_multibyte: bool = !NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
	     let str_multibyte: bool = STRING_MULTIBYTE (newtext);
	     let mut really_changed: bool = false;

	     substed_alloc_size = if length <= (STRING_BYTES_BOUND - 100) / 2
	     { length * 2 + 100 } else { STRING_BYTES_BOUND };
	     substed = xmalloc (substed_alloc_size as usize) as *mut u8;
	     substed_len = 0;

	     /* Go thru NEWTEXT, producing the actual text to insert in
		SUBSTED while adjusting multibyteness to that of the current
		buffer.  */

	     pos_byte = 0;
	     pos = 0;
	     while pos_byte < length
	       {
		 let mut str: [u8; 5] = [0; MAX_MULTIBYTE_LENGTH as usize];
		 let mut add_stuff: *const u8 = std::ptr::null ();
		 let mut add_len: ptrdiff_t = 0;
		 let mut idx: ptrdiff_t = -1;
		 let mut begbyte: ptrdiff_t = 0;

		 if str_multibyte
		   {
		     c = fetch_string_char_advance_no_check (newtext, &raw mut pos, &raw mut pos_byte);
		     if !buf_multibyte
		       { c = CHAR_TO_BYTE8 (c); }
		   }
		 else
		   {
		     /* Note that we don't have to increment POS.  */
		     c = SREF (newtext, pos_byte as usize) as c_int;
		     pos_byte += 1;
		     if buf_multibyte
		       { c = make_char_multibyte (c); }
		   }

		 /* Either set ADD_STUFF and ADD_LEN to the text to put in SUBSTED,
		    or set IDX to a match index, which means put that part
		    of the buffer text into SUBSTED.  */

		 if c == b'\\' as c_int
		   {
		     really_changed = true;

		     if str_multibyte
		       {
			 c = fetch_string_char_advance_no_check (newtext, &raw mut pos, &raw mut pos_byte);
			 if !buf_multibyte && !ASCII_CHAR_P (c as isize)
			   { c = CHAR_TO_BYTE8 (c); }
		       }
		     else
		       {
			 c = SREF (newtext, pos_byte as usize) as c_int;
			 pos_byte += 1;
			 if buf_multibyte
			   { c = make_char_multibyte (c); }
		       }

		     if c == b'&' as c_int
		       { idx = sub; }
		     else if b'1' as c_int <= c && c <= b'9' as c_int && (c - b'0' as c_int) < num_regs as c_int
		       {
			 if *search_regs! ().start.offset ((c - b'0' as c_int) as isize) >= 1
			   { idx = (c - b'0' as c_int) as ptrdiff_t; }
		       }
		     else if c == b'\\' as c_int
		       {
			 add_len = 1;
			 add_stuff = c"\\".as_ptr () as *const u8;
		       }
		     else
		       {
			 xfree (substed as *mut c_void);
			 error (c"Invalid use of `\\' in replacement text".as_ptr ());
		       }
		   }
		 else
		   {
		     add_len = CHAR_STRING (c, str.as_mut_ptr ()) as ptrdiff_t;
		     add_stuff = str.as_ptr ();
		   }

		 /* If we want to copy part of a previous match,
		    set up ADD_STUFF and ADD_LEN to point to it.  */
		 if idx >= 0
		   {
		     begbyte = CHAR_TO_BYTE (*search_regs! ().start.offset (idx as isize));
		     add_len = CHAR_TO_BYTE (*search_regs! ().end.offset (idx as isize)) - begbyte;
		     if *search_regs! ().start.offset (idx as isize) < GPT! ()
			 && GPT! () < *search_regs! ().end.offset (idx as isize)
		       { move_gap_both (*search_regs! ().start.offset (idx as isize), begbyte); }
		   }

		 /* Now the stuff we want to add to SUBSTED
		    is invariably ADD_LEN bytes starting at ADD_STUFF.  */

		 /* Make sure SUBSTED is big enough.  */
		 if substed_alloc_size - substed_len < add_len
		   {
		     substed = xpalloc (substed as *mut c_void, &raw mut substed_alloc_size,
					add_len - (substed_alloc_size - substed_len),
					STRING_BYTES_BOUND, 1) as *mut u8;
		   }

		 /* We compute this after the call to xpalloc, because that
		    could cause buffer text be relocated when ralloc.c is used.  */
		 if idx >= 0
		   { add_stuff = BYTE_POS_ADDR (begbyte) as *const u8; }

		 /* Now add to the end of SUBSTED.  */
		 if ! add_stuff.is_null ()
		   {
		     memcpy (substed.offset (substed_len) as *mut c_void,
			     add_stuff as *const c_void, add_len as usize);
		     substed_len += add_len;
		   }
	       }

	     if really_changed
	       { newtext = make_specified_string (substed as *const c_char, -1,
						  substed_len, buf_multibyte); }
	     xfree (substed as *mut c_void);
	   }

	 newpoint = sub_start + SCHARS (newtext);

	 /* Replace the old text with the new in the cleanest possible way.  */
	 replace_range (sub_start, sub_end, newtext, true, false, true);

	 if case_action == all_caps
	   { Fupcase_region (make_fixnum (*search_regs! ().start.offset (sub as isize)),
			     make_fixnum (newpoint),
			     Qnil ()); }
	 else if case_action == cap_initial
	   { Fupcase_initials_region (make_fixnum (*search_regs! ().start.offset (sub as isize)),
				      make_fixnum (newpoint), Qnil ()); }

	 /* Put point back where it was in the text, if possible.  */
	 TEMP_SET_PT (clip_to_bounds (BEGV! (), opoint + (if opoint <= 0 { ZV! () } else { 0 }), ZV! ()));
	 /* Now move point "officially" to the end of the inserted replacement.  */
	 move_if_not_intangible (newpoint);

	 update_compositions (sub_start, newpoint, CHECK_BORDER);

	 return Qnil ();
       });

unsafe fn match_limit (num: Lisp_Object, beginningp: bool) -> Lisp_Object   // C 2769-2784
{
  let mut n: EMACS_INT;

  CHECK_FIXNUM (num);
  n = XFIXNUM (num);
  if n < 0
    { args_out_of_range (num, make_fixnum (0)); }
  if search_regs! ().num_regs <= 0
    { error (c"No match data, because no search succeeded".as_ptr ()); }
  if n >= search_regs! ().num_regs
      || *search_regs! ().start.offset (n as isize) < 0
    { return Qnil (); }
  return make_fixnum (if beginningp { *search_regs! ().start.offset (n as isize) }
		      else { *search_regs! ().end.offset (n as isize) });
}

DEFUN! ("match-beginning", Fmatch_beginning, Smatch_beginning, 1, 1, 0,    // C 2786-2799
       doc: "Return position of start of text matched by last search.\nSUBEXP, a number, specifies the parenthesized subexpression in the last\n  regexp for which to return the start position.\nValue is nil if SUBEXPth subexpression didn't match, or there were fewer\n  than SUBEXP subexpressions.\nSUBEXP zero means the entire text matched by the whole regexp or whole\n  string.\n\nReturn value is undefined if the last search failed.",
       (subexp: Lisp_Object),
       {
	 return match_limit (subexp, true);
       });

DEFUN! ("match-end", Fmatch_end, Smatch_end, 1, 1, 0,          // C 2801-2814
       doc: "Return position of end of text matched by last search.\nSUBEXP, a number, specifies the parenthesized subexpression in the last\n  regexp for which to return the end position.\nValue is nil if SUBEXPth subexpression didn't match, or there were fewer\n  than SUBEXP subexpressions.\nSUBEXP zero means the entire text matched by the whole regexp or whole\n  string.\n\nReturn value is undefined if the last search failed.",
       (subexp: Lisp_Object),
       {
	 return match_limit (subexp, false);
       });

DEFUN! ("match-data", Fmatch_data, Smatch_data, 0, 3, 0,       // C 2816-2947
       doc: "Return a list of positions that record text matched by the last search.\nElement 2N of the returned list is the position of the beginning of the\nmatch of the Nth subexpression; it corresponds to `(match-beginning N)';\nelement 2N + 1 is the position of the end of the match of the Nth\nsubexpression; it corresponds to `(match-end N)'.  See `match-beginning'\nand `match-end'.\nIf the last search was on a buffer, all the elements are by default\nmarkers or nil (nil when the Nth pair didn't match); they are integers\nor nil if the search was on a string.  But if the optional argument\nINTEGERS is non-nil, the elements that represent buffer positions are\nalways integers, not markers, and (if the search was on a buffer) the\nbuffer itself is appended to the list as one additional element.\n\nUse `set-match-data' to reinstate the match data from the elements of\nthis list.\n\nNote that non-matching optional groups at the end of the regexp are\nelided instead of being represented with two `nil's each.  For instance:\n\n  (progn\n    (string-match \"^\\(a\\)?\\(b\\)\\(c\\)?$\" \"b\")\n    (match-data))\n  => (0 1 nil nil 0 1)\n\nIf REUSE is a list, store the value in REUSE by destructively modifying it.\nIf REUSE is long enough to hold all the values, its length remains the\nsame, and any unused elements are set to nil.  If REUSE is not long\nenough, it is extended.  Note that if REUSE is long enough and INTEGERS\nis non-nil, no consing is done to make the return value; this minimizes GC.\n\nIf optional third argument RESEAT is non-nil, any previous markers on the\nREUSE list will be modified to point to nowhere.\n\nReturn value is undefined if the last search failed.",
       (integers: Lisp_Object, reuse: Lisp_Object, reseat: Lisp_Object),
       {
	 let mut tail: Lisp_Object;
	 let mut prev: Lisp_Object;
	 let mut data: *mut Lisp_Object = std::ptr::null_mut ();
	 let mut i: ptrdiff_t;
	 let mut len: ptrdiff_t;
	 let mut reuse: Lisp_Object = reuse;

	 if !NILP (reseat)
	   {
	     tail = reuse;
	     while CONSP (tail)
	       {
		 if MARKERP (XCAR (tail))
		   {
		     unchain_marker (XMARKER (XCAR (tail)) as *mut c_void);
		     XSETCAR (tail, Qnil ());
		   }
		 tail = XCDR (tail);
	       }
	   }

	 if NILP (last_thing_searched! ())
	   { return Qnil (); }

	 prev = Qnil ();

	 USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
	 SAFE_NALLOCA! (sa_avail, data, 1, 2 * search_regs! ().num_regs + 1);

	 len = 0;
	 i = 0;
	 while i < search_regs! ().num_regs
	   {
	     let start: ptrdiff_t = *search_regs! ().start.offset (i as isize);
	     if start >= 0
	       {
		 if BASE_EQ (last_thing_searched! (), Qt ())
		     || ! NILP (integers)
		   {
		     XSETFASTINT! (*data.offset ((2 * i) as isize), start);
		     XSETFASTINT! (*data.offset ((2 * i + 1) as isize), *search_regs! ().end.offset (i as isize));
		   }
		 else if BUFFERP (last_thing_searched! ())
		   {
		     *data.offset ((2 * i) as isize) = Fmake_marker ();
		     Fset_marker (*data.offset ((2 * i) as isize),
				  make_fixnum (start),
				  last_thing_searched! ());
		     *data.offset ((2 * i + 1) as isize) = Fmake_marker ();
		     Fset_marker (*data.offset ((2 * i + 1) as isize),
				  make_fixnum (*search_regs! ().end.offset (i as isize)),
				  last_thing_searched! ());
		   }
		 else
		   /* last_thing_searched must always be Qt, a buffer, or Qnil.  */
		   { emacs_abort (); }

		 len = 2 * i + 2;
	       }
	     else
	       {
		 *data.offset ((2 * i) as isize) = Qnil ();
		 *data.offset ((2 * i + 1) as isize) = Qnil ();
	       }
	     i += 1;
	   }

	 if BUFFERP (last_thing_searched! ()) && !NILP (integers)
	   {
	     *data.offset (len as isize) = last_thing_searched! ();
	     len += 1;
	   }

	 /* If REUSE is not usable, cons up the values and return them.  */
	 if ! CONSP (reuse)
	   { reuse = Flist (len, data); }
	 else
	   {
	     /* If REUSE is a list, store as many value elements as will fit
		into the elements of REUSE.  */
	     i = 0;
	     tail = reuse;
	     while CONSP (tail)
	       {
		 if i < len
		   { XSETCAR (tail, *data.offset (i as isize)); }
		 else
		   { XSETCAR (tail, Qnil ()); }
		 prev = tail;
		 i += 1;
		 tail = XCDR (tail);
	       }

	     /* If we couldn't fit all value elements into REUSE,
		cons up the rest of them and add them to the end of REUSE.  */
	     if i < len
	       { XSETCDR (prev, Flist (len - i, data.offset (i as isize))); }
	   }

	 SAFE_FREE! (sa_count, sa_mark);
	 return reuse;
       });

/* We used to have an internal use variant of `reseat' described as:

      If RESEAT is `evaporate', put the markers back on the free list
      immediately.  No other references to the markers must exist in this
      case, so it is used only internally on the unwind stack and
      save-match-data from Lisp.

   But it was ill-conceived: those supposedly-internal markers get exposed via
   the undo-list, so freeing them here is unsafe.  */

DEFUN! ("set-match-data", Fset_match_data, Sset_match_data, 1, 2, 0,   // C 2949-3060
       doc: "Set internal data on last search match from elements of LIST.\nLIST should have been created by calling `match-data' previously.\n\nIf optional arg RESEAT is non-nil, make markers on LIST point nowhere.",
       (list: Lisp_Object, reseat: Lisp_Object),
       {
	 let mut i: ptrdiff_t;
	 let mut marker: Lisp_Object;
	 let mut list: Lisp_Object = list;

	 if running_asynch_code
	   { save_search_regs (); }

	 CHECK_LIST (list);

	 /* Unless we find a marker with a buffer or an explicit buffer
	    in LIST, assume that this match data came from a string.  */
	 last_thing_searched! () = Qt ();

	 /* Allocate registers if they don't already exist.  */
	 {
	   let length: ptrdiff_t = list_length (list) / 2;

	   if length > search_regs! ().num_regs
	     {
	       let mut num_regs: ptrdiff_t = search_regs! ().num_regs;
	       search_regs! ().start =
		 xpalloc (search_regs! ().start as *mut c_void, &raw mut num_regs,
			  length - num_regs,
			  min (PTRDIFF_MAX, UINT_MAX as isize),
			  std::mem::size_of::<ptrdiff_t> () as ptrdiff_t) as *mut ptrdiff_t;
	       search_regs! ().end =
		 xrealloc (search_regs! ().end as *mut c_void,
			   num_regs as usize * std::mem::size_of::<ptrdiff_t> ()) as *mut ptrdiff_t;

	       i = search_regs! ().num_regs;
	       while i < num_regs
		 {
		   *search_regs! ().start.offset (i as isize) = -1;
		   i += 1;
		 }

	       search_regs! ().num_regs = num_regs;
	     }

	   i = 0;
	   while CONSP (list)
	     {
	       marker = XCAR (list);
	       if BUFFERP (marker)
		 {
		   last_thing_searched! () = marker;
		   break;
		 }
	       if i >= length
		 { break; }
	       if NILP (marker)
		 {
		   *search_regs! ().start.offset (i as isize) = -1;
		   list = XCDR (list);
		 }
	       else
		 {
		   let mut from: Lisp_Object;
		   let mut m: Lisp_Object;

		   m = marker;
		   if MARKERP (marker)
		     {
		       if (*XMARKER (marker)).buffer.is_null ()
			 { XSETFASTINT! (marker, 0); }
		       else
			 { XSETBUFFER! (last_thing_searched! (), (*XMARKER (marker)).buffer); }
		     }

		   CHECK_FIXNUM_COERCE_MARKER! (marker);
		   from = marker;

		   if !NILP (reseat) && MARKERP (m)
		     {
		       unchain_marker (XMARKER (m) as *mut c_void);
		       XSETCAR (list, Qnil ());
		     }

		   list = XCDR (list);
		   if !CONSP (list)
		     { break; }

		   marker = XCAR (list);
		   m = marker;

		   if MARKERP (marker) && (*XMARKER (marker)).buffer.is_null ()
		     { XSETFASTINT! (marker, 0); }

		   CHECK_FIXNUM_COERCE_MARKER! (marker);
		   if PTRDIFF_MIN <= XFIXNUM (from) && XFIXNUM (from) <= PTRDIFF_MAX
		       && PTRDIFF_MIN <= XFIXNUM (marker)
		       && XFIXNUM (marker) <= PTRDIFF_MAX
		     {
		       *search_regs! ().start.offset (i as isize) = XFIXNUM (from);
		       *search_regs! ().end.offset (i as isize) = XFIXNUM (marker);
		     }
		   else
		     {
		       *search_regs! ().start.offset (i as isize) = -1;
		     }

		   if !NILP (reseat) && MARKERP (m)
		     {
		       unchain_marker (XMARKER (m) as *mut c_void);
		       XSETCAR (list, Qnil ());
		     }
		 }
	       list = XCDR (list);
	       i += 1;
	     }

	   while i < search_regs! ().num_regs
	     {
	       *search_regs! ().start.offset (i as isize) = -1;
	       i += 1;
	     }
	 }

	 return Qnil ();
       });

DEFUN! ("match-data--translate", Fmatch_data__translate, Smatch_data__translate,   // C 3062-3079
       1, 1, 0,
       doc: "Add N to all positions in the match data.  Internal.",
       (n: Lisp_Object),
       {
	 CHECK_FIXNUM (n);
	 let delta: EMACS_INT = XFIXNUM (n);
	 if !NILP (last_thing_searched! ())
	   {
	     let mut i: ptrdiff_t = 0;
	     while i < search_regs! ().num_regs
	       {
		 if *search_regs! ().start.offset (i as isize) >= 0
		   {
		     *search_regs! ().start.offset (i as isize)
		       = max (0, *search_regs! ().start.offset (i as isize) + delta);
		     *search_regs! ().end.offset (i as isize)
		       = max (0, *search_regs! ().end.offset (i as isize) + delta);
		   }
		 i += 1;
	       }
	   }
	 return Qnil ();
       });

/* Called from Flooking_at, Fstring_match, search_buffer, Fstore_match_data
   if asynchronous code (filter or sentinel) is running. */
unsafe fn save_search_regs ()                                 // C 3082-3095
{
  if saved_search_regs! ().num_regs == 0
    {
      saved_search_regs! () = search_regs! ();
      saved_last_thing_searched! () = last_thing_searched! ();
      last_thing_searched! () = Qnil ();
      search_regs! ().num_regs = 0;
      search_regs! ().start = std::ptr::null_mut ();
      search_regs! ().end = std::ptr::null_mut ();
    }
}

/* Called upon exit from filters and sentinels. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_search_regs ()               // C 3097-3113
{
  if saved_search_regs! ().num_regs != 0
    {
      if search_regs! ().num_regs > 0
	{
	  xfree (search_regs! ().start as *mut c_void);
	  xfree (search_regs! ().end as *mut c_void);
	}
      search_regs! () = saved_search_regs! ();
      last_thing_searched! () = saved_last_thing_searched! ();
      saved_last_thing_searched! () = Qnil ();
      saved_search_regs! ().num_regs = 0;
    }
}

/* Called from replace-match via replace_range.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_search_regs (oldstart: ptrdiff_t, oldend: ptrdiff_t,   // C 3115-3147
					     newend: ptrdiff_t)
{
  /* Adjust search data for this change.  */
  let change: ptrdiff_t = newend - oldend;
  let mut i: ptrdiff_t;

  /* When replacing subgroup 3 in a match for regexp '\(\)\(\(\)\)\(\)'
     start[i] should ideally stay unchanged for all but i=4 and end[i]
     should move for all but i=1.
     We don't have enough info here to distinguish those different subgroups
     (except for subgroup 0), so instead we lean towards leaving the start[i]s
     unchanged and towards moving the end[i]s.  */

  i = 0;
  while i < search_regs! ().num_regs
    {
      if *search_regs! ().start.offset (i as isize) <= oldstart
	/* If the subgroup that 'replace-match' is modifying encloses the
	   subgroup 'i', then its 'start' position should stay unchanged.
	   That's always true for subgroup 0.
	   For other subgroups it depends on details we don't have, so
	   we optimistically assume that it also holds for them.  */
	{}
      else if *search_regs! ().start.offset (i as isize) >= oldend
	{ *search_regs! ().start.offset (i as isize) += change; }
      else
	{ *search_regs! ().start.offset (i as isize) = oldstart; }
      if *search_regs! ().end.offset (i as isize) >= oldend
	{ *search_regs! ().end.offset (i as isize) += change; }
      else if *search_regs! ().end.offset (i as isize) > oldstart
	{ *search_regs! ().end.offset (i as isize) = oldstart; }
      i += 1;
    }
}

unsafe extern "C" fn unwind_set_match_data (list: Lisp_Object)   // C 3149-3154
{
  /* It is NOT ALWAYS safe to free (evaporate) the markers immediately.  */
  Fset_match_data (list, Qt ());
}

/* Called to unwind protect the match data.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn record_unwind_save_match_data ()      // C 3157-3162
{
  record_unwind_protect (Some (unwind_set_match_data),
			 Fmatch_data (Qnil (), Qnil (), Qnil ()));
}

/* Quote a string to deactivate reg-expr chars */

DEFUN! ("regexp-quote", Fregexp_quote, Sregexp_quote, 1, 1, 0,   // C 3165-3205
       doc: "Return a regexp string which matches exactly STRING and nothing else.",
       (string: Lisp_Object),
       {
	 let mut in_: *mut c_char;
	 let mut out: *mut c_char;
	 let end: *mut c_char;
	 let mut temp: *mut c_char = std::ptr::null_mut ();
	 let mut backslashes_added: ptrdiff_t = 0;

	 CHECK_STRING (string);

	 USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
	 SAFE_NALLOCA! (sa_avail, temp, 2, SBYTES (string));

	 /* Now copy the data into the new string, inserting escapes. */

	 in_ = SSDATA (string);
	 end = in_.offset (SBYTES (string));
	 out = temp;

	 while in_ != end
	   {
	     if *in_ as c_int == b'[' as c_int
		 || *in_ as c_int == b'*' as c_int || *in_ as c_int == b'.' as c_int
		 || *in_ as c_int == b'\\' as c_int
		 || *in_ as c_int == b'?' as c_int || *in_ as c_int == b'+' as c_int
		 || *in_ as c_int == b'^' as c_int || *in_ as c_int == b'$' as c_int
	       {
		 *out = b'\\' as c_char;
		 out = out.offset (1);
		 backslashes_added += 1;
	       }
	     *out = *in_;
	     out = out.offset (1);
	     in_ = in_.offset (1);
	   }

	 let result: Lisp_Object
	   = if backslashes_added > 0
	   { make_specified_string (temp,
				    SCHARS (string) + backslashes_added,
				    out.offset_from (temp),
				    STRING_MULTIBYTE (string)) }
	   else
	   { string };
	 SAFE_FREE! (sa_count, sa_mark);
	 return result;
       });

/* Like find_newline, but doesn't use the cache, and only searches forward.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_newline1 (start: ptrdiff_t, start_byte: ptrdiff_t,   // C 3207-3290
					end: ptrdiff_t, end_byte: ptrdiff_t,
					count: ptrdiff_t, counted: *mut ptrdiff_t,
					bytepos: *mut ptrdiff_t, allow_quit: bool)
  -> ptrdiff_t
{
  let mut start = start;
  let mut start_byte = start_byte;
  let mut end = end;
  let mut end_byte = end_byte;
  let mut count = count;

  if count > 0
    {
      if end == 0
	{ end = ZV! (); end_byte = ZV_BYTE! (); }
    }
  else
    {
      if end == 0
	{ end = BEGV! (); end_byte = BEGV_BYTE! (); }
    }
  if end_byte == -1
    { end_byte = CHAR_TO_BYTE (end); }

  if ! counted.is_null ()
    { *counted = count; }

  if count > 0
    {
      while start != end
	{
	  /* Our innermost scanning loop is very simple; it doesn't know
	     about gaps, buffer ends, or the newline cache.  ceiling is
	     the position of the last character before the next such
	     obstacle --- the last character the dumb search loop should
	     examine.  */
	  let mut tem: ptrdiff_t;
	  let mut ceiling_byte: ptrdiff_t = end_byte - 1;

	  if start_byte == -1
	    { start_byte = CHAR_TO_BYTE (start); }

	  /* The dumb loop can only scan text stored in contiguous
	     bytes. BUFFER_CEILING_OF returns the last character
	     position that is contiguous, so the ceiling is the
	     position after that.  */
	  tem = BUFFER_CEILING_OF (start_byte);
	  ceiling_byte = min (tem, ceiling_byte);

	  {
	    /* The termination address of the dumb loop.  */
	    let lim_addr: *mut u8 = BYTE_POS_ADDR (ceiling_byte).offset (1);
	    let lim_byte: ptrdiff_t = ceiling_byte + 1;

	    /* Nonpositive offsets (relative to LIM_ADDR and LIM_BYTE)
	       of the base, the cursor, and the next line.  */
	    let base: ptrdiff_t = start_byte - lim_byte;
	    let mut cursor: ptrdiff_t;
	    let mut next: ptrdiff_t;

	    cursor = base;
	    while cursor < 0
	      {
		/* The dumb loop.  */
		let nl: *mut u8 = memchr (lim_addr.offset (cursor as isize) as *const c_void,
					  b'\n' as c_int, (- cursor) as usize) as *mut u8;
		next = if ! nl.is_null () { nl.offset_from (lim_addr) } else { 0 };

		if nl.is_null ()
		  { break; }
		next += 1;

		count -= 1;
		if count == 0
		  {
		    if ! bytepos.is_null ()
		      { *bytepos = lim_byte + next; }
		    return BYTE_TO_CHAR (lim_byte + next);
		  }
		if allow_quit
		  { maybe_quit (); }

		cursor = next;
	      }

	    start_byte = lim_byte;
	    start = BYTE_TO_CHAR (start_byte);
	  }
	}
    }

  if ! counted.is_null ()
    { *counted -= count; }
  if ! bytepos.is_null ()
    {
      *bytepos = if start_byte == -1 { CHAR_TO_BYTE (start) } else { start_byte };
      eassert! (*bytepos == CHAR_TO_BYTE (start));
    }
  return start;
}

DEFUN! ("newline-cache-check", Fnewline_cache_check, Snewline_cache_check,   // C 3292-3373
       0, 1, 0,
       doc: "Check the newline cache of BUFFER against buffer contents.\n\nBUFFER defaults to the current buffer.\n\nValue is an array of 2 sub-arrays of buffer positions for newlines,\nthe first based on the cache, the second based on actually scanning\nthe buffer.  If the buffer doesn't have a cache, the value is nil.",
       (buffer: Lisp_Object),
       {
	 let mut buf: *mut buffer;
	 let mut old: *mut buffer = std::ptr::null_mut ();
	 let mut nl_count_cache: ptrdiff_t = 0;
	 let mut nl_count_buf: ptrdiff_t = 0;
	 let mut cache_newlines: Lisp_Object;
	 let mut buf_newlines: Lisp_Object;
	 let val: Lisp_Object;
	 let mut from: ptrdiff_t;
	 let mut found: ptrdiff_t;
	 let mut i: ptrdiff_t;

	 if NILP (buffer)
	   { buf = (current_buffer! ()); }
	 else
	   {
	     CHECK_BUFFER (buffer);
	     buf = XBUFFER (buffer);
	     old = (current_buffer! ());
	   }
	 if !(*buf).base_buffer.is_null ()
	   { buf = (*buf).base_buffer; }

	 /* If the buffer doesn't have a newline cache, return nil.  */
	 if NILP (BVAR! (buf, cache_long_scans_))
	     || (*buf).newline_cache.is_null ()
	   { return Qnil (); }

	 /* find_newline can only work on the current buffer.  */
	 if ! old.is_null ()
	   { set_buffer_internal_1 (buf); }

	 /* How many newlines are there according to the cache?  */
	 find_newline (BEGV! (), BEGV_BYTE! (), ZV! (), ZV_BYTE! (),
		       TYPE_MAXIMUM! (ptrdiff_t), &raw mut nl_count_cache, std::ptr::null_mut (), true);

	 /* Create vector and populate it.  */
	 cache_newlines = make_vector (nl_count_cache, make_fixnum (-1));

	 if nl_count_cache != 0
	   {
	     from = BEGV! ();
	     found = from;
	     i = 0;
	     while from < ZV! ()
	       {
		 let from_byte: ptrdiff_t = CHAR_TO_BYTE (from);
		 let mut counted: ptrdiff_t = 0;

		 found = find_newline (from, from_byte, 0, -1, 1, &raw mut counted,
				       std::ptr::null_mut (), true);
		 if counted == 0 || i >= nl_count_cache
		   { break; }
		 ASET (cache_newlines, i, make_fixnum (found - 1));
		 from = found;
		 i += 1;
	       }
	   }

	 /* Now do the same, but without using the cache.  */
	 find_newline1 (BEGV! (), BEGV_BYTE! (), ZV! (), ZV_BYTE! (),
			TYPE_MAXIMUM! (ptrdiff_t), &raw mut nl_count_buf, std::ptr::null_mut (), true);
	 buf_newlines = make_vector (nl_count_buf, make_fixnum (-1));
	 if nl_count_buf != 0
	   {
	     from = BEGV! ();
	     found = from;
	     i = 0;
	     while from < ZV! ()
	       {
		 let from_byte: ptrdiff_t = CHAR_TO_BYTE (from);
		 let mut counted: ptrdiff_t = 0;

		 found = find_newline1 (from, from_byte, 0, -1, 1, &raw mut counted,
					std::ptr::null_mut (), true);
		 if counted == 0 || i >= nl_count_buf
		   { break; }
		 ASET (buf_newlines, i, make_fixnum (found - 1));
		 from = found;
		 i += 1;
	       }
	   }

	 /* Construct the value and return it.  */
	 val = CALLN! (Fvector, cache_newlines, buf_newlines);

	 if ! old.is_null ()
	   { set_buffer_internal_1 (old); }
	 return val;
       });

DEFUN! ("re--describe-compiled", Fre__describe_compiled, Sre__describe_compiled,   // C 3375-3417
       1, 2, 0,
       doc: "Return a string describing the compiled form of REGEXP.\nIf RAW is non-nil, just return the actual bytecode.",
       (regexp: Lisp_Object, raw: Lisp_Object),
       {
	 CHECK_STRING (regexp);
	 let cache_entry: *mut regexp_cache
	   = compile_pattern (regexp, std::ptr::null_mut (),
			      if !NILP (Vcase_fold_search) { BVAR! ((current_buffer! ()), case_canon_table_) } else { Qnil () },
			      false,
			      !NILP (BVAR! ((current_buffer! ()),
					    enable_multibyte_characters_)));
	 if !NILP (raw)
	   { return make_unibyte_string ((*cache_entry).buf.buffer as *const c_char,
					 (*cache_entry).buf.used); }
	 else
	   {			    /* FIXME: Why ENABLE_CHECKING?  */
	     #[cfg (not (enable_checking))]
	     {
	       error (c"Not available: rebuild with --enable-checking".as_ptr ());
	     }
	     #[cfg (enable_checking)]
	     {
	       let mut buffer: *mut c_char = std::ptr::null_mut ();
	       let mut size: usize = 0;
	       let f: *mut FILE = open_memstream (&raw mut buffer, &raw mut size);
	       if f.is_null ()
		 { report_file_error (c"open_memstream failed".as_ptr (), regexp); }
	       print_compiled_pattern (f, &raw mut (*cache_entry).buf);
	       fclose (f);
	       if buffer.is_null ()
		 { return Qnil (); }
	       let description: Lisp_Object = make_unibyte_string (buffer, size as ptrdiff_t);
	       free (buffer as *mut c_void);
	       return description;
	     }
	     #[cfg (all (enable_checking, feature = "no_have_open_memstream"))]
	     {
	       print_compiled_pattern (stderr, &raw mut (*cache_entry).buf);
	       return build_string (c"Description was sent to standard error".as_ptr ());
	     }
	   }
       });

/* The two `DEFVAR_LISP` variables this C file owns: C's storage is `globals.f_<vname>`
   (lisp.h 3495-3530) and `struct emacs_globals globals` is defined once, in alloc.c; the
   port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
   defines it here (docs/status.md 3.29, the callint.rs/syntax.rs shape).  */
#[unsafe(no_mangle)]
pub static mut Vsearch_spaces_regexp: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vinhibit_changing_match_data: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_search ()                 // C 3420-3498
{
  let mut i: c_int = 0;
  while i < REGEXP_CACHE_SIZE as c_int
    {
      staticpro (&raw const searchbufs[i as usize].regexp);
      staticpro (&raw const searchbufs[i as usize].f_whitespace_regexp);
      staticpro (&raw const searchbufs[i as usize].syntax_table);
      i += 1;
    }

  /* Error condition used for failing searches.  */
  DEFSYM! (Qsearch_failed, c"search-failed");

  /* Error condition used for failing searches started by user, i.e.,
     where failure should not invoke the debugger.  */
  DEFSYM! (Quser_search_failed, c"user-search-failed");

  /* Error condition signaled when regexp compile_pattern fails.  */
  DEFSYM! (Qinvalid_regexp, c"invalid-regexp");

  Fput (Qsearch_failed (), Qerror_conditions (),
	list! (Qsearch_failed (), Qerror ()));
  Fput (Qsearch_failed (), Qerror_message (),
	build_string (c"Search failed".as_ptr ()));

  Fput (Quser_search_failed (), Qerror_conditions (),
	list! (Quser_search_failed (), Quser_error (), Qsearch_failed (), Qerror ()));
  Fput (Quser_search_failed (), Qerror_message (),
	build_string (c"Search failed".as_ptr ()));

  Fput (Qinvalid_regexp (), Qerror_conditions (),
	list! (Qinvalid_regexp (), Qerror ()));
  Fput (Qinvalid_regexp (), Qerror_message (),
	build_string (c"Invalid regexp".as_ptr ()));

  re_match_object = Qnil ();
  staticpro (&raw const re_match_object);

  DEFVAR_LISP! (c"search-spaces-regexp", Vsearch_spaces_regexp,
      "Regexp to substitute for bunches of spaces in regexp search.\nSome commands use this for user-specified regexps.\nSpaces that occur inside character classes or repetition operators\nor other such regexp constructs are not replaced with this.\nA value of nil (which is the normal value) means treat spaces\nliterally.  Note that a value with capturing groups can change the\nnumbering of existing capture groups in unexpected ways.  ");
  Vsearch_spaces_regexp = Qnil ();

  DEFSYM! (Qinhibit_changing_match_data, c"inhibit-changing-match-data");
  DEFVAR_LISP! (c"inhibit-changing-match-data", Vinhibit_changing_match_data,
      "Internal use only.\nIf non-nil, the primitive searching and matching functions\nsuch as `looking-at', `string-match', `re-search-forward', etc.,\ndo not set the match data.  The proper way to use this variable\nis to bind it with `let' around a small expression.  ");
  Vinhibit_changing_match_data = Qnil ();

  defsubr (&raw mut Slooking_at);
  defsubr (&raw mut Sposix_looking_at);
  defsubr (&raw mut Sstring_match);
  defsubr (&raw mut Sposix_string_match);
  defsubr (&raw mut Ssearch_forward);
  defsubr (&raw mut Ssearch_backward);
  defsubr (&raw mut Sre_search_forward);
  defsubr (&raw mut Sre_search_backward);
  defsubr (&raw mut Sposix_search_forward);
  defsubr (&raw mut Sposix_search_backward);
  defsubr (&raw mut Sreplace_match);
  defsubr (&raw mut Smatch_beginning);
  defsubr (&raw mut Smatch_end);
  defsubr (&raw mut Smatch_data);
  defsubr (&raw mut Sset_match_data);
  defsubr (&raw mut Smatch_data__translate);
  defsubr (&raw mut Sregexp_quote);
  defsubr (&raw mut Snewline_cache_check);
  defsubr (&raw mut Sre__describe_compiled);

  pdumper_do_now_and_after_load (Some (syms_of_search_for_pdumper));
}

unsafe extern "C" fn syms_of_search_for_pdumper ()         // C 3500-3515
{
  let mut i: c_int = 0;
  while i < REGEXP_CACHE_SIZE as c_int
    {
      searchbufs[i as usize].buf.allocated = 100;
      searchbufs[i as usize].buf.buffer = xmalloc (100) as *mut u8;
      searchbufs[i as usize].buf.fastmap = searchbufs[i as usize].fastmap.as_mut_ptr ();
      searchbufs[i as usize].regexp = Qnil ();
      searchbufs[i as usize].f_whitespace_regexp = Qnil ();
      searchbufs[i as usize].busy = false;
      searchbufs[i as usize].syntax_table = Qnil ();
      searchbufs[i as usize].next
	= if i == REGEXP_CACHE_SIZE as c_int - 1
	{ std::ptr::null_mut () } else { &raw mut searchbufs[(i + 1) as usize] };
      i += 1;
    }
  searchbuf_head = &raw mut searchbufs[0];
}
