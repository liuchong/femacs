// femacs: port of src/charset.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct/union field, file-scope object, local), same control flow, same error
// model.  C's 6 non-`static` definitions (`string_xstring_p`, `map_charset_chars`,
// `decode_char`, `encode_char`, `char_charset`, `init_charset`, `init_charset_once`,
// `syms_of_charset`) carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static` helpers
// (`load_charset_map`, `read_hex`, `load_charset_map_from_file`,
// `load_charset_map_from_vector`, `load_charset`, `map_charset_for_dump`,
// `define_charset_internal`, `check_iso_charset_parameter`, `find_charsets_in_text`,
// `maybe_unify_char`, `shrink_charset_table`, `charset_compare`) stay plain `unsafe fn`.
//
// **`charset.h`'s mirror lands completed with this unit** (`femacs/src/include/charset.rs`,
// the composite.rs/intervals.rs precedent): the three enums, both structs, the 63 macros,
// the two INLINE functions and every `extern` prototype and object.  This unit defines
// five of the header's prototypes, so those moved into a `charset_decls` module + glob
// import (PORTING.md 0.1-10, the composite.rs shape) and the header's nineteen objects
// into `charset_objs` (P-21); all nineteen are defined here, and the definitions shadow
// the glob imports.  Until this commit three units declared the names they take from this
// header at their own use sites (chartab.rs, ccl.rs and the coding.h/buffer.h chain);
// their own declarations keep shadowing the glob imports (0.1-10), so none of them
// changes.
//
// **`src/charset.c` is in `docs/baseline/config/cond-eval-verified.txt`**, so the frozen
// configuration's branch selection is the clang-verified one (P-08): the file's `#if`s are
// `#ifdef REL_ALLOC` (C 1607-1612, 1628-1630, both in `maybe_unify_char`) and the literal
// `#if 13 <= __GNUC__`/`#endif` only, and they are recorded in `docs/ports/charset.md`
// rather than transcribed.  `REL_ALLOC` is a source-header macro the frozen table does not
// know, so `check-cond-eval.py` reports `0 unknown` for the file and the two arms are dead
// by clang's own verdict.
//
// C's uninitialised locals (`vec`/`table` `UNINIT`) are zero-initialised (PORTING.md 1.5)
// and C 331-337's `#include`s are reproduced in the mirror graph: `<stdlib.h>`, `<unistd.h>`,
// `<limits.h>`, `<sys/types.h>` and `sysstdio.h`'s `<stdio.h>` all have mirrors; the four
// system entries this file needs that no mirror carries yet (`getc`, `emacs_fdopen`,
// `exit`, `qsort`, `fprintf`/`stderr`, `strerror`, `add_to_log`, `set_unwind_protect_ptr`,
// `fclose_unwind`, `nconc2`, `map_char_table_for_charset`, `file_accessible_directory_p`,
// `coding_system_charset_list`, `record_unwind_protect_nothing`, `clear_unwind_protect`)
// are declared at the use site with C's prototype, the ccl.rs shape.
//
// The six `bool_bf : 1` members of `struct charset` are read and written through the
// mirror's `charset_bits!` macro over the `CHARSET_*_P` masks (PORTING.md 1.3/P-80, the
// intervals.rs `interval_bits!` shape), so every C read and write keeps its C field name
// and `docs/ports/token-aliases.tsv` carries six occurrence-checked families.
//
// `struct charset_map_entries`'s `entry` member is an array of a C *anonymous* struct, and
// `tools/extract-abi.py` attributes that struct's members (`from`, `to`, `c`) to the outer
// container; the three zero-sized `[u8; 0]` markers below reproduce that view without
// adding storage or moving a field (the ledger records the deviation, the layout is
// pinned by `const _` assertions).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut, unused_parens, unused_labels, unused_comparisons,
         overflowing_literals)]

include!("include/config.rs");                 // C 22: `#include <config.h>`
include!("../lib/include/errno.rs");           // C 24: `#include <errno.h>`
include!("../lib/include/stdlib.rs");          // C 25: `#include <stdlib.h>`
include!("../lib/include/unistd.in.rs");       // C 26: `#include <unistd.h>`
include!("../lib/include/limits.in.rs");       // C 27: `#include <limits.h>`
include!("../lib/include/sys_types.in.rs");    // C 28: `#include <sys/types.h>`
include!("../lib/include/string.rs");          // lisp.h 30: `<string.h>` (memcpy, memset, strlen)
include!("../lib/include/stdio.rs");           // C 34: sysstdio.h 24 `#include <stdio.h>`
include!("include/lisp.rs");                   // C 29: `#include "lisp.h"`
include!("include/character.rs");              // C 30: `#include "character.h"`
include!("include/charset.rs");                // C 31: `#include "charset.h"`
include!("include/itree.rs");                  // buffer.h 27: `#include "itree.h"` (buffer.rs's members)
include!("include/systime.rs");                // coding.h's termhooks.h chain
include!("include/font.rs");                   // composite.h 28: `#include "font.h"`
include!("include/composite.rs");              // coding.h 284: `#include "composite.h"`
include!("include/buffer.rs");                 // C 33: `#include "buffer.h"`
include!("include/intervals.rs");              // buffer.h 29: `#include "intervals.h"`
include!("include/termhooks.rs");              // frame.h 25: `#include "termhooks.h"` (terminal, output_method)
include!("include/keyboard.rs");               // frame.h 26: `#include "keyboard.h"`
include!("include/frame.rs");                  // buffer.h 30: `#include "frame.h"`
include!("include/dispextern.rs");             // composite.h 29: `#include "dispextern.h"`
include!("include/window.rs");                 // buffer.h 31: `#include "window.h"`
include!("include/coding.rs");                 // C 32: `#include "coding.h"`
include!("include/pdumper.rs");                // C 35: `#include "pdumper.h"`

/* The four system entries charset.c names that no mirror carries yet, each with C's own
   prototype (PORTING.md 0.1-2/P-72, the ccl.rs shape; the mirror gaps are reported in
   docs/ports/charset.md).  */
unsafe extern "C" {
  /// C: lisp.h 4832: `extern void set_unwind_protect_ptr (specpdl_ref, void (*) (void *), void *);`
  fn set_unwind_protect_ptr (count: specpdl_ref,
			     func: Option<unsafe extern "C" fn (*mut c_void)>,
			     arg: *mut c_void);
  /// C: lisp.h 4977: `extern void fclose_unwind (void *);`
  fn fclose_unwind (arg: *mut c_void);
  /// C: lisp.h 5320: `extern FILE *emacs_fdopen (int, const char *)`
  fn emacs_fdopen (fd: c_int, mode: *const c_char) -> *mut FILE;
  /// C: <stdio.h>: `int getc (FILE *);`
  fn getc (stream: *mut FILE) -> c_int;
  /// C: lisp.h 4378: `extern void add_to_log (char const *, ...);`
  fn add_to_log (format: *const c_char, ...);
  /// C: lisp.h 4692: `extern void map_char_table_for_charset (void (*c_function) (Lisp_Object,
  /// Lisp_Object), Lisp_Object, Lisp_Object, Lisp_Object, struct charset *, unsigned, unsigned);`
  fn map_char_table_for_charset (c_function: Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object)>,
				 function: Lisp_Object, table: Lisp_Object, arg: Lisp_Object,
				 charset: *mut charset, from: c_uint, to: c_uint);
  /// C: lisp.h 4271: `extern Lisp_Object nconc2 (Lisp_Object, Lisp_Object);`
  fn nconc2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4988: `extern bool file_accessible_directory_p (Lisp_Object);`
  fn file_accessible_directory_p (filename: Lisp_Object) -> bool;
  /// C: coding.h 688: `extern Lisp_Object coding_system_charset_list (Lisp_Object);`
  fn coding_system_charset_list (coding_system: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4743-4745: `extern int openp (Lisp_Object, Lisp_Object, Lisp_Object,
  /// Lisp_Object *, Lisp_Object, bool, bool, void **);`
  fn openp (path: Lisp_Object, str_: Lisp_Object, suffixes: Lisp_Object,
	    storeptr: *mut Lisp_Object, predicate: Lisp_Object, newer: bool,
	    no_native: bool, platform: *mut *mut c_void) -> c_int;
  /// C: lisp.h 5316-5318: `# define emacs_fclose fclose` (so the reference calls `fclose`;
  /// the port keeps the C spelling the unit's text uses and links the system symbol, the
  /// pthread_sigmask.rs `#[link_name]` shape).
  #[link_name = "fclose"]
  fn emacs_fclose (stream: *mut FILE) -> c_int;
  /// C: lisp.h 4827: `extern void record_unwind_protect_nothing (void)`
  fn record_unwind_protect_nothing ();
  /// C: lisp.h 4829: `extern void clear_unwind_protect (specpdl_ref)`
  fn clear_unwind_protect (count: specpdl_ref);
  /// C: <stdlib.h>: `void exit (int);`
  fn exit (status: c_int) -> !;
  /// C: <stdio.h>: `int fprintf (FILE *stream, const char *format, ...);`
  fn fprintf (stream: *mut FILE, format: *const c_char, ...) -> c_int;
  /// C: <stdio.h>: `extern FILE *stderr;` (Darwin's libc spells the symbol `__stderrp`).
  #[cfg_attr (target_os = "macos", link_name = "__stderrp")]
  static mut stderr: *mut FILE;
  /// C: <string.h>: `char *strerror (int);`
  fn strerror (errnum: c_int) -> *mut c_char;
  /// C: <stdlib.h>: `void qsort (void *, size_t, size_t, int (*) (const void *, const void *));`
  fn qsort (base: *mut c_void, nmemb: usize, size: usize,
	    compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);
}

/* <intprops.h> has no mirror (PORTING.md P-26); this is `INT_LEFT_SHIFT_OVERFLOW`'s
   unsigned branch: `INT_LEFT_SHIFT_RANGE_OVERFLOW (a, b, 0, UINT_MAX)` is
   `(_GL_INT_MAXIMUM (a)) >> (b) < (a)` for an unsigned `a` (intprops.h 206-213), the
   canonicalize-lgpl.rs shape.  */
macro_rules! INT_LEFT_SHIFT_OVERFLOW {
  ($a:expr, $b:expr) => { (UINT_MAX >> ($b)) < ($a as c_uint) };
}

/*** GENERAL NOTES on CODED CHARACTER SETS (CHARSETS) ***

  A coded character set ("charset" hereafter) is a meaningful
  collection (i.e. language, culture, functionality, etc.) of
  characters.  Emacs handles multiple charsets at once.  In Emacs Lisp
  code, a charset is represented by a symbol.  In C code, a charset is
  represented by its ID number or by a pointer to a struct charset.

  The actual information about each charset is stored in two places.
  Lispy information is stored in the hash table Vcharset_hash_table as
  a vector (charset attributes).  The other information is stored in
  charset_table as a struct charset.

*/

/* Hash table that contains attributes of each charset.  Keys are
   charset symbols, and values are vectors of charset attributes.  */
#[unsafe(no_mangle)]
pub static mut Vcharset_hash_table: Lisp_Object = lisp_h_Qnil ();

/* The table of all charsets.  */
#[unsafe(no_mangle)]
pub static mut charset_table: charset_table = charset_table {
  start: std::ptr::null_mut (), size: 0, used: 0, attributes_table: lisp_h_Qnil (),
};

/* Special charsets corresponding to symbols.  */
#[unsafe(no_mangle)]
pub static mut charset_ascii: c_int = 0;
#[unsafe(no_mangle)]
pub static mut charset_eight_bit: c_int = 0;
static mut charset_iso_8859_1: c_int = 0;
#[unsafe(no_mangle)]
pub static mut charset_unicode: c_int = 0;
static mut charset_emacs: c_int = 0;

/* The other special charsets.  */
#[unsafe(no_mangle)]
pub static mut charset_jisx0201_roman: c_int = 0;
#[unsafe(no_mangle)]
pub static mut charset_jisx0208_1978: c_int = 0;
#[unsafe(no_mangle)]
pub static mut charset_jisx0208: c_int = 0;
#[unsafe(no_mangle)]
pub static mut charset_ksc5601: c_int = 0;

/* Charset of unibyte characters.  */
#[unsafe(no_mangle)]
pub static mut charset_unibyte: c_int = 0;

/* List of charsets ordered by the priority.  */
#[unsafe(no_mangle)]
pub static mut Vcharset_ordered_list: Lisp_Object = lisp_h_Qnil ();

/* Sub-list of Vcharset_ordered_list that contains all non-preferred
   charsets.  */
#[unsafe(no_mangle)]
pub static mut Vcharset_non_preferred_head: Lisp_Object = lisp_h_Qnil ();

/* Incremented every time we change the priority of charsets.
   Wraps around.  */
#[unsafe(no_mangle)]
pub static mut charset_ordered_list_tick: EMACS_UINT = 0;

/* List of iso-2022 charsets.  */
#[unsafe(no_mangle)]
pub static mut Viso_2022_charset_list: Lisp_Object = lisp_h_Qnil ();

/* List of emacs-mule charsets.  */
#[unsafe(no_mangle)]
pub static mut Vemacs_mule_charset_list: Lisp_Object = lisp_h_Qnil ();

#[unsafe(no_mangle)]
pub static mut emacs_mule_charset: [c_int; 256] = [0; 256];

/* Mapping table from ISO2022's charset (specified by DIMENSION,
   CHARS, and FINAL-CHAR) to Emacs's charset.  */
#[unsafe(no_mangle)]
pub static mut iso_charset_table:
  [[[c_int; ISO_MAX_FINAL as usize]; ISO_MAX_CHARS as usize]; ISO_MAX_DIMENSION as usize]
  = [[[0; ISO_MAX_FINAL as usize]; ISO_MAX_CHARS as usize]; ISO_MAX_DIMENSION as usize];

/* The DEFVAR_* variables this unit owns (status §3.29): C stores them as fields of
   `struct emacs_globals`; here the owner defines them, and the globals.rs declarations are
   shadowed.  */
#[unsafe(no_mangle)]
pub static mut Vcharset_map_path: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vcharset_list: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vcurrent_iso639_language: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut inhibit_load_charset_map: bool = false;

// C 105-124: `CODE_POINT_TO_INDEX (charset, code)`.  C's `code` is `unsigned` at every
// call site, so the shift/mask arithmetic stays in `unsigned` (c_uint) as C's promotions
// make it, and the two `(int)` casts are the result conversions; the bitfield read is the
// mirror's mask (docs/ports/token-aliases.tsv's `code_linear_p` family).
macro_rules! CODE_POINT_TO_INDEX {
  ($charset:expr, $code:expr) => {
    {
      let charset: *mut charset = $charset;
      let code: c_uint = $code;
      (if (*charset).bits & CHARSET_CODE_LINEAR_P != 0
	 { code.wrapping_sub ((*charset).min_code) as c_int }
       else if *((*charset).code_space_mask).offset ((code >> 24) as isize) & 0x8 != 0
	    && *((*charset).code_space_mask).offset (((code >> 16) & 0xFF) as isize) & 0x4 != 0
	    && *((*charset).code_space_mask).offset (((code >> 8) & 0xFF) as isize) & 0x2 != 0
	    && *((*charset).code_space_mask).offset ((code & 0xFF) as isize) & 0x1 != 0
	 { (code >> 24).wrapping_sub ((*charset).code_space[12] as c_uint)
	     .wrapping_mul ((*charset).code_space[11] as c_uint)
	   .wrapping_add (((code >> 16) & 0xFF)
			  .wrapping_sub ((*charset).code_space[8] as c_uint)
			  .wrapping_mul ((*charset).code_space[7] as c_uint))
	   .wrapping_add (((code >> 8) & 0xFF)
			  .wrapping_sub ((*charset).code_space[4] as c_uint)
			  .wrapping_mul ((*charset).code_space[3] as c_uint))
	   .wrapping_add ((code & 0xFF).wrapping_sub ((*charset).code_space[0] as c_uint))
	   .wrapping_sub ((*charset).char_index_offset) as c_int }
       else { -1 })
    }
  };
}


/* Return the code-point for the character index IDX in CHARSET.
   IDX should be an unsigned int variable in a valid range (which is
   always in nonnegative int range too).  IDX contains garbage afterwards.  */

// C 127-141: `INDEX_TO_CODE_POINT (charset, idx)`.  C's comma expression assigns the
// caller's `idx` before computing, so the port's second branch is a block that assigns
// `$idx` first; the arithmetic is `unsigned` exactly as C's promotions make it.
macro_rules! INDEX_TO_CODE_POINT {
  ($charset:expr, $idx:expr) => {
    {
      let charset: *mut charset = $charset;
      (if (*charset).bits & CHARSET_CODE_LINEAR_P != 0
	 { $idx + (*charset).min_code }
       else
	 {
	   $idx += (*charset).char_index_offset;
	   ((*charset).code_space[0] as c_uint).wrapping_add ($idx % (*charset).code_space[2] as c_uint)
	   | (((*charset).code_space[4] as c_uint)
	      .wrapping_add ($idx / (*charset).code_space[3] as c_uint
			     % (*charset).code_space[6] as c_uint)) << 8
	   | (((*charset).code_space[8] as c_uint)
	      .wrapping_add ($idx / (*charset).code_space[7] as c_uint
			     % (*charset).code_space[10] as c_uint)) << 16
	   | (((*charset).code_space[12] as c_uint)
	      .wrapping_add ($idx / (*charset).code_space[11] as c_uint)) << 24
	 })
    }
  };
}

/* Structure to hold mapping tables for a charset.  Used by temacs
   invoked for dumping.  */

/// C: charset.c 144-168: the file-scope anonymous struct `temp_charset_work` is a pointer
/// to.  It has no tag in C, so the port names the type after the variable (P-67).
#[repr(C)]
pub struct temp_charset_work
{
  /* The current charset for which the following tables are setup.  */
  pub current: *mut charset,

  /* 1 iff the following table is used for encoder.  */
  pub for_encoder: c_short,

  /* When the following table is used for encoding, minimum and
     maximum character of the current charset.  */
  pub min_char: c_int,
  pub max_char: c_int,

  /* A Unicode character corresponding to the code index 0 (i.e. the
     minimum code-point) of the current charset, or -1 if the code
     index 0 is not a Unicode character.  This is checked when
     table.encoder[CHAR] is zero.  */
  pub zero_index_char: c_int,

  /// C: charset.c 156-165: the anonymous `union { int decoder[0x10000];
  /// unsigned short encoder[0x20000]; } table`.
  pub table: temp_charset_work_table,
}

/// C: charset.c 156-165: the anonymous union member `table` (PORTING.md P-67: an
/// anonymous union is named `<owner>_<member>`).
#[repr(C)]
pub union temp_charset_work_table
{
  /* Table mapping code-indices (not code-points) of the current
     charset to Unicode characters.  If decoder[CHAR] is -1, CHAR
     doesn't belong to the current charset.  */
  pub decoder: [c_int; 0x10000],
  /* Table mapping Unicode characters to code-indices of the current
     charset.  The first 0x10000 elements are for BMP (0..0xFFFF),
     and the last 0x10000 are for SMP (0x10000..0x1FFFF) or SIP
     (0x20000..0x2FFFF).  Note that there is no charset map that
     uses both SMP and SIP.  */
  pub encoder: [c_ushort; 0x20000],
}

static mut temp_charset_work: *mut temp_charset_work = std::ptr::null_mut ();

// C 176-184: `SET_TEMP_CHARSET_WORK_ENCODER (C, CODE)`.
macro_rules! SET_TEMP_CHARSET_WORK_ENCODER {
  ($c:expr, $code:expr) => { {
    if ($code) == 0
      { (*temp_charset_work).zero_index_char = ($c); }
    else if ($c) < 0x20000
      { (*temp_charset_work).table.encoder[($c) as usize] = ($code) as c_ushort; }
    else
      { (*temp_charset_work).table.encoder[($c) as usize - 0x10000] = ($code) as c_ushort; }
  } };
}

// C 186-191: `GET_TEMP_CHARSET_WORK_ENCODER (C)`.
macro_rules! GET_TEMP_CHARSET_WORK_ENCODER {
  ($c:expr) => {
    (if ($c) == (*temp_charset_work).zero_index_char
       { 0 }
     else if ($c) < 0x20000
       { if (*temp_charset_work).table.encoder[($c) as usize] != 0
	   { (*temp_charset_work).table.encoder[($c) as usize] as c_int }
	 else { -1 } }
     else if (*temp_charset_work).table.encoder[($c) as usize - 0x10000] != 0
       { (*temp_charset_work).table.encoder[($c) as usize - 0x10000] as c_int }
     else { -1 })
  };
}

// C 193-194: `SET_TEMP_CHARSET_WORK_DECODER (C, CODE)`.
macro_rules! SET_TEMP_CHARSET_WORK_DECODER {
  ($c:expr, $code:expr) => {
    { (*temp_charset_work).table.decoder[($code) as usize] = ($c) }
  };
}

// C 196-197: `GET_TEMP_CHARSET_WORK_DECODER (CODE)`.
macro_rules! GET_TEMP_CHARSET_WORK_DECODER {
  ($code:expr) => { (*temp_charset_work).table.decoder[($code) as usize] };
}


/* Set to 1 to warn that a charset map is loaded and thus a buffer
   text and a string data may be relocated.  */
#[unsafe(no_mangle)]
pub static mut charset_map_loaded: bool = false;

/// C: charset.c 207-215: `struct charset_map_entries`, whose `entry` member is an array
/// of an anonymous struct.  `charset_map_entry` names that element type; the three
/// zero-sized markers below reproduce `tools/extract-abi.py`'s flattened view of the
/// anonymous struct (the C members are attributed to the outer container and an array
/// field cannot recurse into its element type).  They add no storage and no offset: the
/// `const _` assertions pin `entry` at 0 and `next` at 0xC0000.
#[repr(C)]
pub struct charset_map_entry
{
  pub from: c_uint,
  pub to: c_uint,
  pub c: c_int,
}

#[repr(C)]
pub struct charset_map_entries
{
  pub entry: [charset_map_entry; 0x10000],
  pub next: *mut charset_map_entries,
  /// The anonymous struct's members (see the note above): zero-sized, at this offset.
  pub from: [u8; 0],
  pub to: [u8; 0],
  pub c: [u8; 0],
}

const _: () = assert! (std::mem::offset_of! (charset_map_entries, entry) == 0);
const _: () = assert! (std::mem::offset_of! (charset_map_entries, next) == 0xC0000);
const _: () = assert! (std::mem::size_of::<charset_map_entries> () == 0xC0008);


/* Load the mapping information of CHARSET from ENTRIES for
   initializing (CONTROL_FLAG == 0), decoding (CONTROL_FLAG == 1), and
   encoding (CONTROL_FLAG == 2).

   If CONTROL_FLAG is 0, setup CHARSET->min_char, CHARSET->max_char,
   and CHARSET->fast_map.

   If CONTROL_FLAG is 1, setup the following tables according to
   CHARSET->method and inhibit_load_charset_map.

   CHARSET->method       | inhibit_lcm == 0   | inhibit_lcm == 1
   ----------------------+--------------------+---------------------------
   CHARSET_METHOD_MAP    | CHARSET->decoder   | temp_charset_work->decoder
   ----------------------+--------------------+---------------------------
   CHARSET_METHOD_OFFSET | Vchar_unify_table  | temp_charset_work->decoder

   If CONTROL_FLAG is 2, setup the following tables.

   CHARSET->method       | inhibit_lcm == 0   | inhibit_lcm == 1
   ----------------------+--------------------+---------------------------
   CHARSET_METHOD_MAP    | CHARSET->encoder   | temp_charset_work->encoder
   ----------------------+--------------------+--------------------------
   CHARSET_METHOD_OFFSET | CHARSET->deunifier | temp_charset_work->encoder
*/

unsafe fn load_charset_map (charset: *mut charset, mut entries: *mut charset_map_entries,
			    n_entries: c_int, mut control_flag: c_int)
{
  let mut vec: Lisp_Object = Qnil ();          // C: `Lisp_Object vec UNINIT;` (PORTING.md 1.5)
  let mut table: Lisp_Object = Qnil ();        // C: `Lisp_Object table UNINIT;` (1.5)
  let max_code: c_uint = CHARSET_MAX_CODE! (charset);
  let ascii_compatible_p: bool = charset_bits! (ascii_compatible_p, charset);
  let mut min_char: c_int = 0;
  let mut max_char: c_int = 0;
  let mut nonascii_min_char: c_int = 0;
  let mut i: c_int = 0;
  let fast_map: *mut u8 = (*charset).fast_map.as_mut_ptr ();

  if n_entries <= 0
    { return; }

  if control_flag != 0
    {
      if ! inhibit_load_charset_map
	{
	  if control_flag == 1
	    {
	      if (*charset).method == CHARSET_METHOD_MAP
		{
		  let n: c_int = CODE_POINT_TO_INDEX! (charset, max_code) + 1;

		  vec = make_vector (n as ptrdiff_t, make_fixnum (-1));
		  set_charset_attr (charset, charset_decoder, vec);
		}
	      else
		{
		  char_table_set_range (Vchar_unify_table,
					(*charset).min_char, (*charset).max_char,
					Qnil ());
		}
	    }
	  else
	    {
	      table = Fmake_char_table (Qnil (), Qnil ());
	      set_charset_attr (charset,
				if (*charset).method == CHARSET_METHOD_MAP
				  { charset_encoder } else { charset_deunifier },
				table);
	    }
	}
      else
	{
	  if temp_charset_work.is_null ()
	    { temp_charset_work = xmalloc (std::mem::size_of::<temp_charset_work> ()) as *mut temp_charset_work; }
	  if control_flag == 1
	    {
	      memset ((*temp_charset_work).table.decoder.as_mut_ptr () as *mut c_void, -1,
		      std::mem::size_of::<c_int> () * 0x10000);
	    }
	  else
	    {
	      memset ((*temp_charset_work).table.encoder.as_mut_ptr () as *mut c_void, 0,
		      std::mem::size_of::<c_ushort> () * 0x20000);
	      (*temp_charset_work).zero_index_char = -1;
	    }
	  (*temp_charset_work).current = charset;
	  (*temp_charset_work).for_encoder = (control_flag == 2) as c_short;
	  control_flag += 2;
	}
      charset_map_loaded = true;
    }

  min_char = (*entries).entry[0].c;
  max_char = min_char;
  nonascii_min_char = MAX_CHAR;
  for i in 0..n_entries
    {
      let mut from: c_uint = 0;
      let mut to: c_uint = 0;
      let mut from_index: c_int = 0;
      let mut to_index: c_int = 0;
      let mut lim_index: c_int = 0;
      let mut from_c: c_int = 0;
      let mut to_c: c_int = 0;
      let idx: c_int = i % 0x10000;

      if i > 0 && idx == 0
	{ entries = (*entries).next; }
      from = (*entries).entry[idx as usize].from;
      to = (*entries).entry[idx as usize].to;
      from_c = (*entries).entry[idx as usize].c;
      from_index = CODE_POINT_TO_INDEX! (charset, from);
      if from == to
	{
	  to_index = from_index;
	  to_c = from_c;
	}
      else
	{
	  to_index = CODE_POINT_TO_INDEX! (charset, to);
	  to_c = from_c + (to_index - from_index);
	}
      if from_index < 0 || to_index < 0
	{ continue; }
      lim_index = to_index + 1;

      if to_c > max_char
	{ max_char = to_c; }
      else if from_c < min_char
	{ min_char = from_c; }

      if control_flag == 1
	{
	  if (*charset).method == CHARSET_METHOD_MAP
	    {
	      while from_index < lim_index
		{
		  ASET (vec, from_index as isize, make_fixnum (from_c as EMACS_INT));
		  from_index += 1;
		  from_c += 1;
		}
	    }
	  else
	    {
	      while from_index < lim_index
		{
		  CHAR_TABLE_SET (Vchar_unify_table,
				  (CHARSET_CODE_OFFSET! (charset) + from_index) as isize,
				  make_fixnum (from_c as EMACS_INT));
		  from_index += 1;
		  from_c += 1;
		}
	    }
	}
      else if control_flag == 2
	{
	  if (*charset).method == CHARSET_METHOD_MAP
	      && charset_bits! (compact_codes_p, charset)
	    {
	      while from_index < lim_index
		{
		  let mut code: c_uint = from_index as c_uint;
		  code = INDEX_TO_CODE_POINT! (charset, code);

		  if NILP (CHAR_TABLE_REF (table, from_c as isize))
		    { CHAR_TABLE_SET (table, from_c as isize, make_fixnum (code as EMACS_INT)); }
		  from_index += 1;
		  from_c += 1;
		}
	    }
	  else
	    {
	      while from_index < lim_index
		{
		  if NILP (CHAR_TABLE_REF (table, from_c as isize))
		    { CHAR_TABLE_SET (table, from_c as isize, make_fixnum (from_index as EMACS_INT)); }
		  from_index += 1;
		  from_c += 1;
		}
	    }
	}
      else if control_flag == 3
	{
	  while from_index < lim_index
	    {
	      SET_TEMP_CHARSET_WORK_DECODER! (from_c, from_index);
	      from_index += 1;
	      from_c += 1;
	    }
	}
      else if control_flag == 4
	{
	  while from_index < lim_index
	    {
	      SET_TEMP_CHARSET_WORK_ENCODER! (from_c, from_index);
	      from_index += 1;
	      from_c += 1;
	    }
	}
      else			/* control_flag == 0 */
	{
	  if ascii_compatible_p
	    {
	      if ! ASCII_CHAR_P (from_c as isize)
		{
		  if from_c < nonascii_min_char
		    { nonascii_min_char = from_c; }
		}
	      else if ! ASCII_CHAR_P (to_c as isize)
		{
		  nonascii_min_char = 0x80;
		}
	    }

	  while from_c <= to_c
	    {
	      CHARSET_FAST_MAP_SET! (from_c, fast_map);
	      from_c += 1;
	    }
	}
    }

  if control_flag == 0
    {
      (*charset).min_char = if ascii_compatible_p { nonascii_min_char } else { min_char };
      (*charset).max_char = max_char;
    }
  else if control_flag == 4
    {
      (*temp_charset_work).min_char = min_char;
      (*temp_charset_work).max_char = max_char;
    }
}


/* Read a hexadecimal number (preceded by "0x") from the file FP while
   paying attention to comment character '#'.  LOOKAHEAD is the
   lookahead byte if it is nonnegative.  Store into *TERMINATOR the
   input byte after the number, or EOF if an end-of-file or input
   error occurred.  Set *OVERFLOW if the number overflows.  */

unsafe fn read_hex (fp: *mut FILE, lookahead: c_int, terminator: *mut c_int,
		    overflow: *mut bool) -> c_uint
{
  let mut c: c_int = if lookahead < 0 { getc (fp) } else { lookahead };

  while true
    {
      if c == b'#' as c_int
	{
	  loop
	    {
	      c = getc (fp);
	      if ! (0 <= c && c != b'\n' as c_int)
		{ break; }
	    }
	}
      else if c == b'0' as c_int
	{
	  c = getc (fp);
	  if c < 0 || c == b'x' as c_int
	    { break; }
	}
      if c < 0
	{ break; }
      c = getc (fp);
    }

  let mut n: c_uint = 0;
  let mut v: bool = false;

  if 0 <= c
    {
      while true
	{
	  c = getc (fp);
	  let digit: c_int = char_hexdigit (c);
	  if digit < 0
	    { break; }
	  v |= INT_LEFT_SHIFT_OVERFLOW! (n, 4);
	  n = (n << 4) + digit as c_uint;
	}
    }

  *terminator = c;
  *overflow |= v;
  return n;
}


/* Return a mapping vector for CHARSET loaded from MAPFILE.
   Each line of MAPFILE has this form
	0xAAAA 0xCCCC
   where 0xAAAA is a code-point and 0xCCCC is the corresponding
   character code, or this form
	0xAAAA-0xBBBB 0xCCCC
   where 0xAAAA and 0xBBBB are code-points specifying a range, and
   0xCCCC is the first character code of the range.

   The returned vector has this form:
	[ CODE1 CHAR1 CODE2 CHAR2 .... ]
   where CODE1 is a code-point or a cons of code-points specifying a
   range.

   Note that this function uses `openp' to open MAPFILE but ignores
   `file-name-handler-alist' to avoid running any Lisp code.  */

unsafe fn load_charset_map_from_file (charset: *mut charset, mapfile: Lisp_Object,
				      control_flag: c_int)
{
  let min_code: c_uint = CHARSET_MIN_CODE! (charset);
  let max_code: c_uint = CHARSET_MAX_CODE! (charset);
  let mut fd: c_int = 0;
  let mut fp: *mut FILE = std::ptr::null_mut ();
  let mut head: *mut charset_map_entries = std::ptr::null_mut ();
  let mut entries: *mut charset_map_entries = std::ptr::null_mut ();
  let mut n_entries: c_int = 0;
  AUTO_STRING! (map, c".map".as_ptr ());
  AUTO_STRING! (txt, c".txt".as_ptr ());
  AUTO_LIST2! (suffixes, map, txt);
  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_nothing ();
  specbind (Qfile_name_handler_alist (), Qnil ());
  fd = openp (Vcharset_map_path, mapfile, suffixes, std::ptr::null_mut (), Qnil (),
	      false, false, std::ptr::null_mut ());
  fp = if fd < 0 { std::ptr::null_mut () } else { emacs_fdopen (fd, c"r".as_ptr ()) };
  if fp.is_null ()
    {
      let open_errno: c_int = *errno ();
      emacs_close (fd);
      report_file_errno (c"Loading charset map".as_ptr (), mapfile, open_errno);
    }
  set_unwind_protect_ptr (count, Some (fclose_unwind), fp as *mut c_void);
  unbind_to (specpdl_ref_add (count, 1), Qnil ());

  /* charset_map_entries is large, so don't SAFE_ALLOCA.  */
  entries = xzalloc (std::mem::size_of::<charset_map_entries> ()) as *mut charset_map_entries;
  head = entries;
  record_unwind_protect_ptr (Some (xfree), entries as *mut c_void);

  n_entries = 0;
  let mut ch: c_int = -1;
  while true
    {
      let mut overflow: bool = false;
      let from: c_uint = read_hex (fp, ch, &mut ch, &mut overflow);
      let mut to: c_uint = 0;
      if ch < 0
	{ break; }
      if ch == b'-' as c_int
	{
	  to = read_hex (fp, -1, &mut ch, &mut overflow);
	  if ch < 0
	    { break; }
	}
      else
	{
	  to = from;
	  ch = -1;
	}
      let c: c_uint = read_hex (fp, ch, &mut ch, &mut overflow);
      if ch < 0
	{ break; }

      if overflow
	{ continue; }
      if from < min_code || to > max_code || from > to || c > MAX_CHAR as c_uint
	{ continue; }

      if n_entries == 0x10000
	{
	  let new_entries = xzalloc (std::mem::size_of::<charset_map_entries> ()) as *mut charset_map_entries;
	  (*entries).next = new_entries;
	  entries = new_entries;
	  record_unwind_protect_ptr (Some (xfree), entries as *mut c_void);
	  n_entries = 0;
	}
      let idx: c_int = n_entries;
      (*entries).entry[idx as usize].from = from;
      (*entries).entry[idx as usize].to = to;
      (*entries).entry[idx as usize].c = c as c_int;
      n_entries += 1;
    }
  emacs_fclose (fp);
  clear_unwind_protect (count);

  load_charset_map (charset, head, n_entries, control_flag);
  unbind_to (count, Qnil ());
}

unsafe fn load_charset_map_from_vector (charset: *mut charset, vec: Lisp_Object, control_flag: c_int)
{
  let min_code: c_uint = CHARSET_MIN_CODE! (charset);
  let max_code: c_uint = CHARSET_MAX_CODE! (charset);
  let mut head: *mut charset_map_entries = std::ptr::null_mut ();
  let mut entries: *mut charset_map_entries = std::ptr::null_mut ();
  let mut n_entries: c_int = 0;
  let len: c_int = ASIZE (vec) as c_int;
  let count: specpdl_ref = SPECPDL_INDEX ();

  if len % 2 == 1
    {
      add_to_log (c"Failure in loading charset map: %V".as_ptr (), vec);
      return;
    }

  /* charset_map_entries is large, so don't SAFE_ALLOCA.  */
  entries = xzalloc (std::mem::size_of::<charset_map_entries> ()) as *mut charset_map_entries;
  head = entries;
  record_unwind_protect_ptr (Some (xfree), entries as *mut c_void);

  n_entries = 0;
  for i in (0..len).step_by (2)
    {
      let mut val: Lisp_Object = Qnil ();
      let mut val2: Lisp_Object = Qnil ();
      let mut from: c_uint = 0;
      let mut to: c_uint = 0;
      let mut c: EMACS_INT = 0;
      let idx: c_int;

      val = AREF (vec, i as isize);
      if CONSP (val)
	{
	  val2 = XCDR (val);
	  val = XCAR (val);
	  from = XFIXNAT (val) as c_uint;
	  to = XFIXNAT (val2) as c_uint;
	}
      else
	{ from = XFIXNAT (val) as c_uint; to = from; }
      val = AREF (vec, (i + 1) as isize);
      CHECK_FIXNAT (val);
      c = XFIXNAT (val);

      if from < min_code || to > max_code || from > to || c > MAX_CHAR as EMACS_INT
	{ continue; }

      if n_entries > 0 && (n_entries % 0x10000) == 0
	{
	  let new_entries = xzalloc (std::mem::size_of::<charset_map_entries> ()) as *mut charset_map_entries;
	  (*entries).next = new_entries;
	  entries = new_entries;
	  record_unwind_protect_ptr (Some (xfree), entries as *mut c_void);
	}
      idx = n_entries % 0x10000;
      (*entries).entry[idx as usize].from = from;
      (*entries).entry[idx as usize].to = to;
      (*entries).entry[idx as usize].c = c as c_int;
      n_entries += 1;
    }

  load_charset_map (charset, head, n_entries, control_flag);
  unbind_to (count, Qnil ());
}


/* Load a mapping table for CHARSET.  CONTROL-FLAG tells what kind of
   map it is (see the comment of load_charset_map for the detail).  */

unsafe fn load_charset (charset: *mut charset, control_flag: c_int)
{
  let mut map: Lisp_Object = Qnil ();

  if inhibit_load_charset_map
      && ! temp_charset_work.is_null ()
      && charset == (*temp_charset_work).current
      && ((control_flag == 2) as c_short) == (*temp_charset_work).for_encoder
    { return; }

  if CHARSET_METHOD! (charset) == CHARSET_METHOD_MAP
    { map = CHARSET_MAP! (charset); }
  else
    {
      if ! CHARSET_UNIFIED_P! (charset)
	{ emacs_abort (); }
      map = CHARSET_UNIFY_MAP! (charset);
    }
  if STRINGP (map)
    { load_charset_map_from_file (charset, map, control_flag); }
  else
    { load_charset_map_from_vector (charset, map, control_flag); }
}


DEFUN! ("charsetp", Fcharsetp, Scharsetp, 1, 1, 0,
       doc: "Return non-nil if and only if OBJECT is a charset.",
       (object: Lisp_Object),
       {
	 return if CHARSETP! (object) { Qt () } else { Qnil () };
       });


unsafe fn map_charset_for_dump (c_function: Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object)>,
				function: Lisp_Object, arg: Lisp_Object,
				from: c_uint, to: c_uint)
{
  let from_idx: c_int = CODE_POINT_TO_INDEX! ((*temp_charset_work).current, from);
  let to_idx: c_int = CODE_POINT_TO_INDEX! ((*temp_charset_work).current, to);
  let range: Lisp_Object = Fcons (Qnil (), Qnil ());
  let mut c: c_int = 0;
  let mut stop: c_int = 0;

  c = (*temp_charset_work).min_char;
  stop = if (*temp_charset_work).max_char < 0x20000
	   { (*temp_charset_work).max_char } else { 0xFFFF };

  while true
    {
      let idx: c_int = GET_TEMP_CHARSET_WORK_ENCODER! (c);

      if idx >= from_idx && idx <= to_idx
	{
	  if NILP (XCAR (range))
	    { XSETCAR (range, make_fixnum (c as EMACS_INT)); }
	}
      else if ! NILP (XCAR (range))
	{
	  XSETCDR (range, make_fixnum ((c - 1) as EMACS_INT));
	  if let Some (c_function) = c_function
	    { c_function (arg, range); }
	  else
	    { calln! (function, range, arg); }
	  XSETCAR (range, Qnil ());
	}
      if c == stop
	{
	  if c == (*temp_charset_work).max_char
	    {
	      if ! NILP (XCAR (range))
		{
		  XSETCDR (range, make_fixnum (c as EMACS_INT));
		  if let Some (c_function) = c_function
		    { c_function (arg, range); }
		  else
		    { calln! (function, range, arg); }
		}
	      break;
	    }
	  c = 0x1FFFF;
	  stop = (*temp_charset_work).max_char;
	}
      c += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_charset_chars (c_function: Option<unsafe extern "C" fn (Lisp_Object, Lisp_Object)>,
			   function: Lisp_Object, arg: Lisp_Object, mut charset: *mut charset,
			   mut from: c_uint, mut to: c_uint)
{
  let mut range: Lisp_Object = Qnil ();
  let partial: bool = from > CHARSET_MIN_CODE! (charset)
		       || to < CHARSET_MAX_CODE! (charset);

  if CHARSET_METHOD! (charset) == CHARSET_METHOD_OFFSET
    {
      let from_idx: c_int = CODE_POINT_TO_INDEX! (charset, from);
      let to_idx: c_int = CODE_POINT_TO_INDEX! (charset, to);
      let from_c: c_int = from_idx + CHARSET_CODE_OFFSET! (charset);
      let to_c: c_int = to_idx + CHARSET_CODE_OFFSET! (charset);

      if CHARSET_UNIFIED_P! (charset)
	{
	  if ! CHAR_TABLE_P (CHARSET_DEUNIFIER! (charset))
	    { load_charset (charset, 2); }
	  if CHAR_TABLE_P (CHARSET_DEUNIFIER! (charset))
	    { map_char_table_for_charset (c_function, function,
					  CHARSET_DEUNIFIER! (charset), arg,
					  if partial { charset } else { std::ptr::null_mut () },
					  from, to); }
	  else
	    { map_charset_for_dump (c_function, function, arg, from, to); }
	}

      range = Fcons (make_fixnum (from_c as EMACS_INT), make_fixnum (to_c as EMACS_INT));
      if NILP (function)
	{ (c_function.unwrap ()) (arg, range); }
      else
	{ calln! (function, range, arg); }
    }
  else if CHARSET_METHOD! (charset) == CHARSET_METHOD_MAP
    {
      if ! CHAR_TABLE_P (CHARSET_ENCODER! (charset))
	{ load_charset (charset, 2); }
      if CHAR_TABLE_P (CHARSET_ENCODER! (charset))
	{ map_char_table_for_charset (c_function, function,
				      CHARSET_ENCODER! (charset), arg,
				      if partial { charset } else { std::ptr::null_mut () },
				      from, to); }
      else
	{ map_charset_for_dump (c_function, function, arg, from, to); }
    }
  else if CHARSET_METHOD! (charset) == CHARSET_METHOD_SUBSET
    {
      let mut subset_info: Lisp_Object = Qnil ();
      let mut offset: c_int = 0;

      subset_info = CHARSET_SUBSET! (charset);
      charset = CHARSET_FROM_ID! (XFIXNAT (AREF (subset_info, 0)) as isize);
      offset = XFIXNUM (AREF (subset_info, 3)) as c_int;
      from = from.wrapping_sub (offset as c_uint);
      if from < XFIXNAT (AREF (subset_info, 1)) as c_uint
	{ from = XFIXNAT (AREF (subset_info, 1)) as c_uint; }
      to = to.wrapping_sub (offset as c_uint);
      if to > XFIXNAT (AREF (subset_info, 2)) as c_uint
	{ to = XFIXNAT (AREF (subset_info, 2)) as c_uint; }
      map_charset_chars (c_function, function, arg, charset, from, to);
    }
  else				/* i.e. CHARSET_METHOD_SUPERSET */
    {
      let mut parents: Lisp_Object = Qnil ();

      parents = CHARSET_SUPERSET! (charset);
      while CONSP (parents)
	{
	  let mut offset: c_int = 0;
	  let mut this_from: c_uint = 0;
	  let mut this_to: c_uint = 0;

	  charset = CHARSET_FROM_ID! (XFIXNAT (XCAR (XCAR (parents))) as isize);
	  offset = XFIXNUM (XCDR (XCAR (parents))) as c_int;
	  this_from = if from > offset as c_uint { from - offset as c_uint } else { 0 };
	  this_to = if to > offset as c_uint { to - offset as c_uint } else { 0 };
	  if this_from < CHARSET_MIN_CODE! (charset)
	    { this_from = CHARSET_MIN_CODE! (charset); }
	  if this_to > CHARSET_MAX_CODE! (charset)
	    { this_to = CHARSET_MAX_CODE! (charset); }
	  map_charset_chars (c_function, function, arg, charset,
			     this_from, this_to);
	  parents = XCDR (parents);
	}
    }
}

DEFUN! ("map-charset-chars", Fmap_charset_chars, Smap_charset_chars, 2, 5, 0,
       doc: "Call FUNCTION for all characters in CHARSET.\nOptional 3rd argument ARG is an additional argument to be passed\nto FUNCTION, see below.\nOptional 4th and 5th arguments FROM-CODE and TO-CODE specify the\nrange of code points (in CHARSET) of target characters on which to\nmap the FUNCTION.  Note that these are not character codes, but code\npoints of CHARSET; for the difference see `decode-char' and\n`list-charset-chars'.  If FROM-CODE is nil or omitted, it stands for\nthe first code point of CHARSET; if TO-CODE is nil or omitted, it\nstands for the last code point of CHARSET.\n\nFUNCTION will be called with two arguments: RANGE and ARG.\nRANGE is a cons (FROM .  TO), where FROM and TO specify a range of\ncharacters that belong to CHARSET on which FUNCTION should do its\njob.  FROM and TO are Emacs character codes, unlike FROM-CODE and\nTO-CODE, which are CHARSET code points.",
       (function: Lisp_Object, charset: Lisp_Object, arg: Lisp_Object,
	from_code: Lisp_Object, to_code: Lisp_Object),
       {
	 let mut cs: *mut charset = std::ptr::null_mut ();
	 let mut from: c_uint = 0;
	 let mut to: c_uint = 0;

	 CHECK_CHARSET_GET_CHARSET! (charset, cs);
	 if NILP (from_code)
	   { from = CHARSET_MIN_CODE! (cs); }
	 else
	   {
	     CHECK_FIXNAT (from_code);
	     from = XFIXNUM (from_code) as c_uint;
	     if from < CHARSET_MIN_CODE! (cs)
	       { from = CHARSET_MIN_CODE! (cs); }
	   }
	 if NILP (to_code)
	   { to = CHARSET_MAX_CODE! (cs); }
	 else
	   {
	     CHECK_FIXNAT (to_code);
	     to = XFIXNUM (to_code) as c_uint;
	     if to > CHARSET_MAX_CODE! (cs)
	       { to = CHARSET_MAX_CODE! (cs); }
	   }
	 map_charset_chars (None, function, arg, cs, from, to);
	 return Qnil ();
       });


/* Define a charset according to the arguments.  The Nth argument is
   the Nth attribute of the charset (the last attribute `charset-id'
   is not included).  See the docstring of `define-charset' for the
   details.  */

DEFUN! ("define-charset-internal", Fdefine_charset_internal,
       Sdefine_charset_internal, charset_arg_max, MANY, 0,
       doc: "For internal use only.\nusage: (define-charset-internal ...)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
	 /* Charset attr vector.  */
	 let mut attrs: Lisp_Object = Qnil ();
	 let mut val: Lisp_Object = Qnil ();
	 let hash_table: *mut Lisp_Hash_Table = XHASH_TABLE (Vcharset_hash_table);
	 let mut i: c_int = 0;
	 let mut j: c_int = 0;
	 let mut charset: charset = charset {
	   id: 0, dimension: 0, code_space: [0; 15],
	   code_space_mask: std::ptr::null_mut (), bits: 0,
	   iso_final: 0, iso_revision: 0, emacs_mule_id: 0, method: 0,
	   min_code: 0, max_code: 0, char_index_offset: 0,
	   min_char: 0, max_char: 0, invalid_code: 0, fast_map: [0; 190],
	   code_offset: 0,
	 };
	 let mut id: c_int = 0;
	 let mut dimension: c_int = 0;
	 let mut new_definition_p: bool = false;
	 let mut nchars: c_int = 0;

	 if nargs != charset_arg_max as ptrdiff_t
	   { Fsignal (Qwrong_number_of_arguments (),
		      Fcons (Qdefine_charset_internal (),
			     make_fixnum (nargs))); }

	 attrs = make_nil_vector (charset_attr_max as isize);

	 CHECK_SYMBOL (*args.offset (charset_arg_name as isize));
	 ASET (attrs, charset_name as isize, *args.offset (charset_arg_name as isize));

	 val = *args.offset (charset_arg_code_space as isize);
	 i = 0;
	 dimension = 0;
	 nchars = 1;
	 loop
	   {
	     let min_byte_obj: Lisp_Object = Faref (val, make_fixnum ((i * 2) as EMACS_INT));
	     let max_byte_obj: Lisp_Object = Faref (val, make_fixnum ((i * 2 + 1) as EMACS_INT));
	     let min_byte: c_int = check_integer_range (min_byte_obj, 0, 255) as c_int;
	     let max_byte: c_int = check_integer_range (max_byte_obj, min_byte as intmax_t, 255) as c_int;
	     charset.code_space[(i * 4) as usize] = min_byte;
	     charset.code_space[(i * 4 + 1) as usize] = max_byte;
	     charset.code_space[(i * 4 + 2) as usize] = max_byte - min_byte + 1;
	     if max_byte > 0
	       { dimension = i + 1; }
	     if i == 3
	       { break; }
	     nchars *= charset.code_space[(i * 4 + 2) as usize];
	     charset.code_space[(i * 4 + 3) as usize] = nchars;
	     i += 1;
	   }

	 val = *args.offset (charset_arg_dimension as isize);
	 charset.dimension
	   = if ! NILP (val) { check_integer_range (val, 1, 4) as c_int } else { dimension };

	 charset_bits! (code_linear_p, &mut charset,
			charset.dimension == 1
			|| (charset.code_space[2] == 256
			    && (charset.dimension == 2
				|| (charset.code_space[6] == 256
				    && (charset.dimension == 3
					|| charset.code_space[10] == 256)))));

	 if ! charset_bits! (code_linear_p, &mut charset)
	   {
	     charset.code_space_mask = xzalloc (256) as *mut u8;
	     for i in 0..4
	       {
		 j = charset.code_space[(i * 4) as usize];
		 while j <= charset.code_space[(i * 4 + 1) as usize]
		   {
		     *charset.code_space_mask.offset (j as isize) |= (1 << i) as u8;
		     j += 1;
		   }
	       }
	   }

	 charset_bits! (iso_chars_96, &mut charset, charset.code_space[2] == 96);

	 charset.min_code = (charset.code_space[0] as c_uint
			     | ((charset.code_space[4] as c_uint) << 8)
			     | ((charset.code_space[8] as c_uint) << 16)
			     | ((charset.code_space[12] as c_uint) << 24));
	 charset.max_code = (charset.code_space[1] as c_uint
			     | ((charset.code_space[5] as c_uint) << 8)
			     | ((charset.code_space[9] as c_uint) << 16)
			     | ((charset.code_space[13] as c_uint) << 24));
	 charset.char_index_offset = 0;

	 val = *args.offset (charset_arg_min_code as isize);
	 if ! NILP (val)
	   {
	     let code: c_uint = cons_to_unsigned (val, UINT_MAX as u64) as c_uint;

	     if code < charset.min_code
		 || code > charset.max_code
	       { args_out_of_range_3 (INT_TO_INTEGER (charset.min_code as EMACS_INT),
				      INT_TO_INTEGER (charset.max_code as EMACS_INT), val); }
	     charset.char_index_offset = CODE_POINT_TO_INDEX! (&mut charset, code) as c_uint;
	     charset.min_code = code;
	   }

	 val = *args.offset (charset_arg_max_code as isize);
	 if ! NILP (val)
	   {
	     let code: c_uint = cons_to_unsigned (val, UINT_MAX as u64) as c_uint;

	     if code < charset.min_code
		 || code > charset.max_code
	       { args_out_of_range_3 (INT_TO_INTEGER (charset.min_code as EMACS_INT),
				      INT_TO_INTEGER (charset.max_code as EMACS_INT), val); }
	     charset.max_code = code;
	   }

	 charset_bits! (compact_codes_p, &mut charset, charset.max_code < 0x10000);

	 val = *args.offset (charset_arg_invalid_code as isize);
	 if NILP (val)
	   {
	     if charset.min_code > 0
	       { charset.invalid_code = 0; }
	     else
	       {
		 if charset.max_code < UINT_MAX
		   { charset.invalid_code = charset.max_code + 1; }
		 else
		   { error (c"Attribute :invalid-code must be specified".as_ptr ()); }
	       }
	   }
	 else
	   { charset.invalid_code = cons_to_unsigned (val, UINT_MAX as u64) as c_uint; }

	 val = *args.offset (charset_arg_iso_final as isize);
	 if NILP (val)
	   { charset.iso_final = -1; }
	 else
	   {
	     CHECK_FIXNUM (val);
	     if XFIXNUM (val) < b'0' as EMACS_INT || XFIXNUM (val) > 127
	       { error (c"Invalid iso-final-char: %td".as_ptr (), XFIXNUM (val)); }
	     charset.iso_final = XFIXNUM (val) as c_int;
	   }

	 val = *args.offset (charset_arg_iso_revision as isize);
	 charset.iso_revision = if ! NILP (val) { check_integer_range (val, -1, 63) as c_int } else { -1 };

	 val = *args.offset (charset_arg_emacs_mule_id as isize);
	 if NILP (val)
	   { charset.emacs_mule_id = -1; }
	 else
	   {
	     CHECK_FIXNAT (val);
	     if (XFIXNUM (val) > 0 && XFIXNUM (val) <= 128) || XFIXNUM (val) >= 256
	       { error (c"Invalid emacs-mule-id: %td".as_ptr (), XFIXNUM (val)); }
	     charset.emacs_mule_id = XFIXNUM (val) as c_int;
	   }

	 charset_bits! (ascii_compatible_p, &mut charset,
			! NILP (*args.offset (charset_arg_ascii_compatible_p as isize)));

	 charset_bits! (supplementary_p, &mut charset,
			! NILP (*args.offset (charset_arg_supplementary_p as isize)));

	 charset_bits! (unified_p, &mut charset, 0);

	 memset (charset.fast_map.as_mut_ptr () as *mut c_void, 0,
		 std::mem::size_of::<[u8; 190]> ());

	 if ! NILP (*args.offset (charset_arg_code_offset as isize))
	   {
	     val = *args.offset (charset_arg_code_offset as isize);
	     CHECK_CHARACTER (val);

	     charset.method = CHARSET_METHOD_OFFSET;
	     charset.code_offset = XFIXNUM (val) as c_int;

	     i = CODE_POINT_TO_INDEX! (&mut charset, charset.max_code);
	     if MAX_CHAR - charset.code_offset < i
	       { error (c"Unsupported max char: %d + %ud > MAX_CHAR (%d)".as_ptr (),
			i, charset.max_code, MAX_CHAR); }
	     charset.max_char = i + charset.code_offset;
	     i = CODE_POINT_TO_INDEX! (&mut charset, charset.min_code);
	     charset.min_char = i + charset.code_offset;

	     i = (charset.min_char >> 7) << 7;
	     while i < 0x10000 && i <= charset.max_char
	       { CHARSET_FAST_MAP_SET! (i, charset.fast_map.as_mut_ptr ()); i += 128; }
	     i = (i >> 12) << 12;
	     while i <= charset.max_char
	       { CHARSET_FAST_MAP_SET! (i, charset.fast_map.as_mut_ptr ()); i += 0x1000; }
	     if charset.code_offset == 0 && charset.max_char >= 0x80
	       { charset_bits! (ascii_compatible_p, &mut charset, 1); }
	   }
	 else if ! NILP (*args.offset (charset_arg_map as isize))
	   {
	     val = *args.offset (charset_arg_map as isize);
	     ASET (attrs, charset_map as isize, val);
	     charset.method = CHARSET_METHOD_MAP;
	   }
	 else if ! NILP (*args.offset (charset_arg_subset as isize))
	   {
	     let mut parent: Lisp_Object = Qnil ();
	     let mut parent_min_code: Lisp_Object = Qnil ();
	     let mut parent_max_code: Lisp_Object = Qnil ();
	     let mut parent_code_offset: Lisp_Object = Qnil ();
	     let mut parent_charset: *mut charset = std::ptr::null_mut ();

	     val = *args.offset (charset_arg_subset as isize);
	     parent = Fcar (val);
	     CHECK_CHARSET_GET_CHARSET! (parent, parent_charset);
	     parent_min_code = Fnth (make_fixnum (1), val);
	     CHECK_FIXNAT (parent_min_code);
	     parent_max_code = Fnth (make_fixnum (2), val);
	     CHECK_FIXNAT (parent_max_code);
	     parent_code_offset = Fnth (make_fixnum (3), val);
	     CHECK_FIXNUM (parent_code_offset);
	     ASET (attrs, charset_subset as isize,
		   CALLN! (Fvector, make_fixnum ((*parent_charset).id as EMACS_INT),
			   parent_min_code, parent_max_code, parent_code_offset));

	     charset.method = CHARSET_METHOD_SUBSET;
	     /* Here, we just copy the parent's fast_map.  It's not accurate,
		but at least it works for quickly detecting which character
		DOESN'T belong to this charset.  */
	     memcpy (charset.fast_map.as_mut_ptr () as *mut c_void,
		     (*parent_charset).fast_map.as_mut_ptr () as *const c_void,
		     std::mem::size_of::<[u8; 190]> ());

	     /* We also copy these for parents.  */
	     charset.min_char = (*parent_charset).min_char;
	     charset.max_char = (*parent_charset).max_char;
	   }
	 else if ! NILP (*args.offset (charset_arg_superset as isize))
	   {
	     val = *args.offset (charset_arg_superset as isize);
	     charset.method = CHARSET_METHOD_SUPERSET;
	     val = Fcopy_sequence (val);
	     ASET (attrs, charset_superset as isize, val);

	     charset.min_char = MAX_CHAR;
	     charset.max_char = 0;
	     while ! NILP (val)
	       {
		 let mut elt: Lisp_Object = Qnil ();
		 let mut car_part: Lisp_Object = Qnil ();
		 let mut cdr_part: Lisp_Object = Qnil ();
		 let mut this_id: c_int = 0;
		 let mut offset: c_int = 0;
		 let mut this_charset: *mut charset = std::ptr::null_mut ();

		 elt = Fcar (val);
		 if CONSP (elt)
		   {
		     car_part = XCAR (elt);
		     cdr_part = XCDR (elt);
		     CHECK_CHARSET_GET_ID! (car_part, this_id);
		     offset = check_integer_range (cdr_part, INT_MIN as intmax_t, INT_MAX as intmax_t) as c_int;
		   }
		 else
		   {
		     CHECK_CHARSET_GET_ID! (elt, this_id);
		     offset = 0;
		   }
		 XSETCAR (val, Fcons (make_fixnum (this_id as EMACS_INT),
				      make_fixnum (offset as EMACS_INT)));

		 this_charset = CHARSET_FROM_ID! (this_id as isize);
		 if charset.min_char > (*this_charset).min_char
		   { charset.min_char = (*this_charset).min_char; }
		 if charset.max_char < (*this_charset).max_char
		   { charset.max_char = (*this_charset).max_char; }
		 for i in 0..190
		   { charset.fast_map[i as usize] |= (*this_charset).fast_map[i as usize]; }
		 val = Fcdr (val);
	       }
	   }
	 else
	   { error (c"None of :code-offset, :map, :parents are specified".as_ptr ()); }

	 val = *args.offset (charset_arg_unify_map as isize);
	 if ! NILP (val) && ! STRINGP (val)
	   { CHECK_VECTOR (val); }
	 ASET (attrs, charset_unify_map as isize, val);

	 CHECK_LIST (*args.offset (charset_arg_plist as isize));
	 ASET (attrs, charset_plist as isize, *args.offset (charset_arg_plist as isize));

	 let mut hash_code: hash_hash_t = 0;
	 let hash_index: ptrdiff_t
	   = hash_find_get_hash (hash_table, *args.offset (charset_arg_name as isize), &mut hash_code);
	 if hash_index >= 0
	   {
	     new_definition_p = false;
	     id = XFIXNAT (CHARSET_SYMBOL_ID! (*args.offset (charset_arg_name as isize))) as c_int;
	     set_hash_value_slot (hash_table, hash_index as hash_idx_t, attrs);
	   }
	 else
	   {
	     hash_put (hash_table, *args.offset (charset_arg_name as isize), attrs, hash_code);
	     if charset_table.used == charset_table.size
	       {
		 /* Ensure that charset IDs fit into 'int' as well as into the
		    restriction imposed by fixnums.  Although the 'int' restriction
		    could be removed, too much other code would need altering; for
		    example, the IDs are stuffed into struct
		    coding_system.charbuf[i] entries, which are 'int'.  */
		 let old_size: c_int = charset_table.size;
		 let mut new_size: ptrdiff_t = old_size as ptrdiff_t;
		 let new_table: *mut charset
		   = xpalloc (std::ptr::null_mut (), &mut new_size, 1,
			      min (INT_MAX as isize, MOST_POSITIVE_FIXNUM),
			      std::mem::size_of::<charset> () as ptrdiff_t) as *mut charset;
		 memcpy (new_table as *mut c_void, charset_table.start as *const c_void,
			 (old_size as usize) * std::mem::size_of::<charset> ());
		 xfree (charset_table.start as *mut c_void);
		 charset_table.start = new_table;
		 charset_table.size = new_size as c_int;
		 let new_attr_table: Lisp_Object = make_vector (new_size, Qnil ());
		 for i in 0..old_size
		   { ASET (new_attr_table, i as isize,
			   AREF (charset_table.attributes_table, i as isize)); }
		 charset_table.attributes_table = new_attr_table;
	       }
	     id = charset_table.used;
	     charset_table.used += 1;
	     new_definition_p = true;
	   }

	 ASET (attrs, charset_id as isize, make_fixnum (id as EMACS_INT));
	 charset.id = id;
	 *charset_table.start.offset (id as isize) = charset;
	 ASET (charset_table.attributes_table, id as isize, attrs);
	 eassert! (ASIZE (charset_table.attributes_table) == charset_table.size as isize);

	 if charset.method == CHARSET_METHOD_MAP
	   {
	     load_charset (&mut charset, 0);
	     *charset_table.start.offset (id as isize) = charset;
	   }

	 if charset.iso_final >= 0
	   {
	     ISO_CHARSET_TABLE! (charset.dimension, charset_bits! (iso_chars_96, &mut charset),
				 charset.iso_final) = id;
	     if new_definition_p
	       { Viso_2022_charset_list = nconc2 (Viso_2022_charset_list, list1i (id as EMACS_INT)); }
	     if ISO_CHARSET_TABLE! (1, 0, b'J' as c_int) == id
	       { charset_jisx0201_roman = id; }
	     else if ISO_CHARSET_TABLE! (2, 0, b'@' as c_int) == id
	       { charset_jisx0208_1978 = id; }
	     else if ISO_CHARSET_TABLE! (2, 0, b'B' as c_int) == id
	       { charset_jisx0208 = id; }
	     else if ISO_CHARSET_TABLE! (2, 0, b'C' as c_int) == id
	       { charset_ksc5601 = id; }
	   }

	 if charset.emacs_mule_id >= 0
	   {
	     emacs_mule_charset[charset.emacs_mule_id as usize] = id;
	     if charset.emacs_mule_id < 0xA0
	       { emacs_mule_bytes[charset.emacs_mule_id as usize] = (charset.dimension + 1) as c_char; }
	     else
	       { emacs_mule_bytes[charset.emacs_mule_id as usize] = (charset.dimension + 2) as c_char; }
	     if new_definition_p
	       { Vemacs_mule_charset_list = nconc2 (Vemacs_mule_charset_list,
						    list1i (id as EMACS_INT)); }
	   }

	 if new_definition_p
	   {
	     Vcharset_list = Fcons (*args.offset (charset_arg_name as isize), Vcharset_list);
	     if charset_bits! (supplementary_p, &mut charset)
	       { Vcharset_ordered_list = nconc2 (Vcharset_ordered_list, list1i (id as EMACS_INT)); }
	     else
	       {
		 let mut tail: Lisp_Object = Qnil ();

		 tail = Vcharset_ordered_list;
		 while CONSP (tail)
		   {
		     let cs: *mut charset = CHARSET_FROM_ID! (XFIXNUM (XCAR (tail)));

		     if charset_bits! (supplementary_p, cs)
		       { break; }
		     tail = XCDR (tail);
		   }
		 if EQ (tail, Vcharset_ordered_list)
		   { Vcharset_ordered_list = Fcons (make_fixnum (id as EMACS_INT),
						    Vcharset_ordered_list); }
		 else if NILP (tail)
		   { Vcharset_ordered_list = nconc2 (Vcharset_ordered_list,
						     list1i (id as EMACS_INT)); }
		 else
		   {
		     val = Fcons (XCAR (tail), XCDR (tail));
		     XSETCDR (tail, val);
		     XSETCAR (tail, make_fixnum (id as EMACS_INT));
		   }
	       }
	     charset_ordered_list_tick += 1;
	   }

	 return Qnil ();
       });


/* Same as Fdefine_charset_internal but arguments are more convenient
   to call from C (typically in syms_of_charset).  This can define a
   charset of `offset' method only.  Return the ID of the new
   charset.  */

unsafe fn define_charset_internal (name: Lisp_Object,
				   dimension: c_int,
				   code_space_chars: *const c_char,
				   min_code: c_uint, max_code: c_uint,
				   iso_final: c_int, iso_revision: c_int,
				   emacs_mule_id: c_int,
				   ascii_compatible: bool, supplementary: bool,
				   code_offset: c_int) -> c_int
{
  let code_space: *const u8 = code_space_chars as *const u8;
  let mut args: [Lisp_Object; charset_arg_max as usize]
    = [Qnil (); charset_arg_max as usize];
  let mut val: Lisp_Object = Qnil ();
  let mut i: c_int = 0;

  args[charset_arg_name as usize] = name;
  args[charset_arg_dimension as usize] = make_fixnum (dimension as EMACS_INT);
  val = make_uninit_vector (8);
  for i in 0..8
    { ASET (val, i as isize, make_fixnum (*code_space.offset (i as isize) as EMACS_INT)); }
  args[charset_arg_code_space as usize] = val;
  args[charset_arg_min_code as usize] = make_fixnum (min_code as EMACS_INT);
  args[charset_arg_max_code as usize] = make_fixnum (max_code as EMACS_INT);
  args[charset_arg_iso_final as usize]
    = if iso_final < 0 { Qnil () } else { make_fixnum (iso_final as EMACS_INT) };
  args[charset_arg_iso_revision as usize] = make_fixnum (iso_revision as EMACS_INT);
  args[charset_arg_emacs_mule_id as usize]
    = if emacs_mule_id < 0 { Qnil () } else { make_fixnum (emacs_mule_id as EMACS_INT) };
  args[charset_arg_ascii_compatible_p as usize] = if ascii_compatible { Qt () } else { Qnil () };
  args[charset_arg_supplementary_p as usize] = if supplementary { Qt () } else { Qnil () };
  args[charset_arg_invalid_code as usize] = Qnil ();
  args[charset_arg_code_offset as usize] = make_fixnum (code_offset as EMACS_INT);
  args[charset_arg_map as usize] = Qnil ();
  args[charset_arg_subset as usize] = Qnil ();
  args[charset_arg_superset as usize] = Qnil ();
  args[charset_arg_unify_map as usize] = Qnil ();

  args[charset_arg_plist as usize] =
     list! (QCname (),
	    args[charset_arg_name as usize],
	    intern_c_string (c":dimension".as_ptr ()),
	    args[charset_arg_dimension as usize],
	    intern_c_string (c":code-space".as_ptr ()),
	    args[charset_arg_code_space as usize],
	    intern_c_string (c":iso-final-char".as_ptr ()),
	    args[charset_arg_iso_final as usize],
	    intern_c_string (c":emacs-mule-id".as_ptr ()),
	    args[charset_arg_emacs_mule_id as usize],
	    QCascii_compatible_p (),
	    args[charset_arg_ascii_compatible_p as usize],
	    intern_c_string (c":code-offset".as_ptr ()),
	    args[charset_arg_code_offset as usize]);
  Fdefine_charset_internal (charset_arg_max as ptrdiff_t, args.as_mut_ptr ());

  return XFIXNUM (CHARSET_SYMBOL_ID! (name)) as c_int;
}


DEFUN! ("define-charset-alias", Fdefine_charset_alias,
       Sdefine_charset_alias, 2, 2, 0,
       doc: "Define ALIAS as an alias for charset CHARSET.",
       (alias: Lisp_Object, charset: Lisp_Object),
       {
	 let mut attr: Lisp_Object = Qnil ();

	 CHECK_CHARSET_GET_ATTR! (charset, attr);
	 Fputhash (alias, attr, Vcharset_hash_table);
	 Vcharset_list = Fcons (alias, Vcharset_list);
	 return Qnil ();
       });


DEFUN! ("charset-plist", Fcharset_plist, Scharset_plist, 1, 1, 0,
       doc: "Return the property list of CHARSET.",
       (charset: Lisp_Object),
       {
	 let mut attrs: Lisp_Object = Qnil ();

	 CHECK_CHARSET_GET_ATTR! (charset, attrs);
	 return CHARSET_ATTR_PLIST! (attrs);
       });


DEFUN! ("set-charset-plist", Fset_charset_plist, Sset_charset_plist, 2, 2, 0,
       doc: "Set CHARSET's property list to PLIST.",
       (charset: Lisp_Object, plist: Lisp_Object),
       {
	 let mut attrs: Lisp_Object = Qnil ();

	 CHECK_CHARSET_GET_ATTR! (charset, attrs);
	 ASET (attrs, charset_plist as isize, plist);
	 return plist;
       });


DEFUN! ("unify-charset", Funify_charset, Sunify_charset, 1, 3, 0,
       doc: "Unify characters of CHARSET with Unicode.\nThis means reading the relevant file and installing the table defined\nby CHARSET's `:unify-map' property.\n\nOptional second arg UNIFY-MAP is a file name string or a vector.  It has\nthe same meaning as the `:unify-map' attribute in the function\n`define-charset' (which see).\n\nOptional third argument DEUNIFY, if non-nil, means to de-unify CHARSET.  ",
       (charset: Lisp_Object, unify_map: Lisp_Object, deunify: Lisp_Object),
       {
	 let mut unify_map: Lisp_Object = unify_map;
	 let mut id: c_int = 0;
	 let mut cs: *mut charset = std::ptr::null_mut ();

	 CHECK_CHARSET_GET_ID! (charset, id);
	 cs = CHARSET_FROM_ID! (id as isize);
	 if if NILP (deunify)
	      { charset_bits! (unified_p, cs) && ! NILP (CHARSET_DEUNIFIER! (cs)) }
	    else
	      { ! charset_bits! (unified_p, cs) }
	   { return Qnil (); }

	 charset_bits! (unified_p, cs, 0);
	 if NILP (deunify)
	   {
	     if CHARSET_METHOD! (cs) != CHARSET_METHOD_OFFSET
		 || CHARSET_CODE_OFFSET! (cs) < 0x110000
	       { error (c"Can't unify charset: %s".as_ptr (), SDATA (SYMBOL_NAME (charset))); }
	     if NILP (unify_map)
	       { unify_map = CHARSET_UNIFY_MAP! (cs); }
	     else
	       {
		 if ! STRINGP (unify_map) && ! VECTORP (unify_map)
		   { signal_error (c"Bad unify-map".as_ptr (), unify_map); }
		 set_charset_attr (cs, charset_unify_map, unify_map);
	       }
	     if NILP (Vchar_unify_table)
	       { Vchar_unify_table = Fmake_char_table (Qnil (), Qnil ()); }
	     char_table_set_range (Vchar_unify_table,
				   (*cs).min_char, (*cs).max_char, charset);
	     charset_bits! (unified_p, cs, 1);
	   }
	 else if CHAR_TABLE_P (Vchar_unify_table)
	   {
	     let min_code: c_uint = CHARSET_MIN_CODE! (cs);
	     let max_code: c_uint = CHARSET_MAX_CODE! (cs);
	     let min_char: c_int = DECODE_CHAR! (cs, min_code);
	     let max_char: c_int = DECODE_CHAR! (cs, max_code);

	     char_table_set_range (Vchar_unify_table, min_char, max_char, Qnil ());
	   }

	 return Qnil ();
       });

/* Check that DIMENSION, CHARS, and FINAL_CHAR specify a valid ISO charset.
   Return true if it's a 96-character set, false if 94.  */

unsafe fn check_iso_charset_parameter (dimension: Lisp_Object, chars: Lisp_Object,
					final_char: Lisp_Object) -> bool
{
  CHECK_FIXNUM (dimension);
  CHECK_FIXNUM (chars);
  CHECK_CHARACTER (final_char);

  if ! (1 <= XFIXNUM (dimension) && XFIXNUM (dimension) <= 3)
    { error (c"Invalid DIMENSION %td, it should be 1, 2, or 3".as_ptr (),
	     XFIXNUM (dimension)); }

  let chars_flag: bool = XFIXNUM (chars) == 96;
  if ! (chars_flag || XFIXNUM (chars) == 94)
    { error (c"Invalid CHARS %td, it should be 94 or 96".as_ptr (), XFIXNUM (chars)); }

  let final_ch: c_int = XFIXNAT (final_char) as c_int;
  if ! (b'0' as c_int <= final_ch && final_ch <= b'~' as c_int)
    { error (c"Invalid FINAL-CHAR `%c', it should be `0'..`~'".as_ptr (), final_ch); }

  return chars_flag;
}

DEFUN! ("get-unused-iso-final-char", Fget_unused_iso_final_char,
       Sget_unused_iso_final_char, 2, 2, 0,
       doc: "\nReturn an unused ISO final char for a charset of DIMENSION and CHARS.\nDIMENSION is the number of bytes to represent a character: 1 or 2.\nCHARS is the number of characters in a dimension: 94 or 96.\n\nThis final char is for private use, thus the range is `0' (48) .. `?' (63).\nIf there's no unused final char for the specified kind of charset,\nreturn nil.  ",
       (dimension: Lisp_Object, chars: Lisp_Object),
       {
	 let chars_flag: bool = check_iso_charset_parameter (dimension, chars,
							    make_fixnum (b'0' as EMACS_INT));
	 let mut final_char: c_int = b'0' as c_int;
	 while final_char <= b'?' as c_int
	   {
	     if ISO_CHARSET_TABLE! (XFIXNUM (dimension), chars_flag as c_int, final_char) < 0
	       { return make_fixnum (final_char as EMACS_INT); }
	     final_char += 1;
	   }
	 return Qnil ();
       });


DEFUN! ("declare-equiv-charset", Fdeclare_equiv_charset, Sdeclare_equiv_charset,
       4, 4, 0,
       doc: "Declare an equivalent charset for ISO-2022 decoding.\n\nOn decoding by an ISO-2022 base coding system, when a charset\nspecified by DIMENSION, CHARS, and FINAL-CHAR is designated, behave as\nif CHARSET is designated instead.  ",
       (dimension: Lisp_Object, chars: Lisp_Object, final_char: Lisp_Object,
	charset: Lisp_Object),
       {
	 let mut id: c_int = 0;

	 CHECK_CHARSET_GET_ID! (charset, id);
	 let chars_flag: bool = check_iso_charset_parameter (dimension, chars, final_char);
	 ISO_CHARSET_TABLE! (XFIXNUM (dimension), chars_flag as c_int, XFIXNAT (final_char) as c_int) = id;
	 return Qnil ();
       });


/* Return information about charsets in the text at PTR of NBYTES
   bytes, which are NCHARS characters.  The value is:

	0: Each character is represented by one byte.  This is always
	   true for a unibyte string.  For a multibyte string, true if
	   it contains only ASCII characters.

	1: No charsets other than ascii, control-1, and latin-1 are
	   found.

	2: Otherwise.
*/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn string_xstring_p (string: Lisp_Object) -> c_int
{
  let p: *const u8 = SDATA (string) as *const u8;
  let endp: *const u8 = p.offset (SBYTES (string));

  if SCHARS (string) == SBYTES (string)
    { return 0; }

  let mut p: *const u8 = p;
  while p < endp
    {
      let c: c_int = string_char_advance (&mut p);

      if c >= 0x100
	{ return 2; }
    }
  return 1;
}


/* Find charsets in the string at PTR of NCHARS and NBYTES.

   CHARSETS is a vector.  If Nth element is non-nil, it means the
   charset whose id is N is already found.

   It may lookup a translation table TABLE if supplied.  */

unsafe fn find_charsets_in_text (ptr: *const u8, nchars: ptrdiff_t,
				 nbytes: ptrdiff_t, charsets: Lisp_Object,
				 table: Lisp_Object, multibyte: bool)
{
  let pend: *const u8 = ptr.offset (nbytes);

  if nchars == nbytes
    {
      if multibyte
	{ ASET (charsets, charset_ascii as isize, Qt ()); }
      else
	{
	  let mut ptr: *const u8 = ptr;
	  while ptr < pend
	    {
	      let mut c: c_int = *ptr as c_int;
	      ptr = ptr.offset (1);

	      if ! NILP (table)
		{ c = translate_char (table, c); }
	      if ASCII_CHAR_P (c as isize)
		{ ASET (charsets, charset_ascii as isize, Qt ()); }
	      else
		{ ASET (charsets, charset_eight_bit as isize, Qt ()); }
	    }
	}
    }
  else
    {
      let mut ptr: *const u8 = ptr;
      while ptr < pend
	{
	  let mut c: c_int = string_char_advance (&mut ptr);
	  let mut charset: *mut charset = std::ptr::null_mut ();

	  if ! NILP (table)
	    { c = translate_char (table, c); }
	  charset = CHAR_CHARSET! (c);
	  ASET (charsets, CHARSET_ID! (charset) as isize, Qt ());
	}
    }
}

DEFUN! ("find-charset-region", Ffind_charset_region, Sfind_charset_region,
       2, 3, 0,
       doc: "Return a list of charsets in the region between BEG and END.\nBEG and END are buffer positions.\nOptional arg TABLE if non-nil is a translation table to look up.\n\nIf the current buffer is unibyte, the returned list may contain\nonly `ascii' and `eight-bit'.  ",
       (beg: Lisp_Object, end: Lisp_Object, table: Lisp_Object),
       {
	 let mut beg: Lisp_Object = beg;
	 let mut end: Lisp_Object = end;
	 let mut charsets: Lisp_Object = Qnil ();
	 let mut from: ptrdiff_t = 0;
	 let mut from_byte: ptrdiff_t = 0;
	 let mut to: ptrdiff_t = 0;
	 let mut stop: ptrdiff_t = 0;
	 let mut stop_byte: ptrdiff_t = 0;
	 let mut i: c_int = 0;
	 let mut val: Lisp_Object = Qnil ();
	 let multibyte: bool = ! NILP (BVAR! (current_buffer, enable_multibyte_characters_));

	 validate_region (&mut beg, &mut end);
	 from = XFIXNAT (beg);
	 stop = XFIXNAT (end);
	 to = stop;

	 if from < GPT! () && GPT! () < to
	   {
	     stop = GPT! ();
	     stop_byte = GPT_BYTE! ();
	   }
	 else
	   { stop_byte = CHAR_TO_BYTE (stop); }

	 from_byte = CHAR_TO_BYTE (from);

	 charsets = make_nil_vector (charset_table.used as isize);
	 loop
	   {
	     find_charsets_in_text (BYTE_POS_ADDR (from_byte) as *const u8, stop - from,
				    stop_byte - from_byte, charsets, table,
				    multibyte);
	     if stop < to
	       {
		 from = stop;
		 from_byte = stop_byte;
		 stop = to;
		 stop_byte = CHAR_TO_BYTE (stop);
	       }
	     else
	       { break; }
	   }

	 val = Qnil ();
	 i = charset_table.used - 1;
	 while i >= 0
	   {
	     if ! NILP (AREF (charsets, i as isize))
	       { val = Fcons (CHARSET_NAME! (charset_table.start.offset (i as isize)), val); }
	     i -= 1;
	   }
	 return val;
       });

DEFUN! ("find-charset-string", Ffind_charset_string, Sfind_charset_string,
       1, 2, 0,
       doc: "Return a list of charsets in STR.\nOptional arg TABLE if non-nil is a translation table to look up.\n\nIf STR is unibyte, the returned list may contain\nonly `ascii' and `eight-bit'.",
       (str_: Lisp_Object, table: Lisp_Object),
       {
	 CHECK_STRING (str_);

	 let charsets: Lisp_Object = make_nil_vector (charset_table.used as isize);
	 find_charsets_in_text (SDATA (str_) as *const u8, SCHARS (str_), SBYTES (str_),
				charsets, table,
				STRING_MULTIBYTE (str_));
	 let mut val: Lisp_Object = Qnil ();
	 let mut i: c_int = charset_table.used - 1;
	 while i >= 0
	   {
	     if ! NILP (AREF (charsets, i as isize))
	       { val = Fcons (CHARSET_NAME! (charset_table.start.offset (i as isize)), val); }
	     i -= 1;
	   }
	 return val;
       });



/* Return a unified character code for C (>= 0x110000).  VAL is a
   value of Vchar_unify_table for C; i.e. it is nil, an integer, or a
   charset symbol.  */
unsafe fn maybe_unify_char (c: c_int, val: Lisp_Object) -> c_int
{
  let mut c: c_int = c;
  let mut val: Lisp_Object = val;
  let mut charset: *mut charset = std::ptr::null_mut ();

  if FIXNUMP (val)
    { return XFIXNAT (val) as c_int; }
  if NILP (val)
    { return c; }

  CHECK_CHARSET_GET_CHARSET! (val, charset);
  load_charset (charset, 1);
  if ! inhibit_load_charset_map
    {
      val = CHAR_TABLE_REF (Vchar_unify_table, c as isize);
      if ! NILP (val)
	{ c = XFIXNAT (val) as c_int; }
    }
  else
    {
      let code_index: c_int = c - CHARSET_CODE_OFFSET! (charset) as c_int;
      let unified: c_int = GET_TEMP_CHARSET_WORK_DECODER! (code_index);

      if unified > 0
	{ c = unified; }
    }
  return c;
}


/* Return a character corresponding to the code-point CODE of
   CHARSET.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode_char (charset_: *mut charset, code: c_uint) -> c_int
{
  let mut charset: *mut charset = charset_;
  let mut c: c_int = 0;
  let mut char_index: c_int = 0;
  let method: charset_method = CHARSET_METHOD! (charset);

  if code < CHARSET_MIN_CODE! (charset) || code > CHARSET_MAX_CODE! (charset)
    { return -1; }

  if method == CHARSET_METHOD_SUBSET
    {
      let mut subset_info: Lisp_Object = Qnil ();

      subset_info = CHARSET_SUBSET! (charset);
      charset = CHARSET_FROM_ID! (XFIXNAT (AREF (subset_info, 0)) as isize);
      let code: c_uint = code.wrapping_sub (XFIXNUM (AREF (subset_info, 3)) as c_uint);
      if code < XFIXNAT (AREF (subset_info, 1)) as c_uint
	  || code > XFIXNAT (AREF (subset_info, 2)) as c_uint
	{ c = -1; }
      else
	{ c = DECODE_CHAR! (charset, code); }
    }
  else if method == CHARSET_METHOD_SUPERSET
    {
      let mut parents: Lisp_Object = Qnil ();

      parents = CHARSET_SUPERSET! (charset);
      c = -1;
      while CONSP (parents)
	{
	  let id: c_int = XFIXNUM (XCAR (XCAR (parents))) as c_int;
	  let code_offset: c_int = XFIXNUM (XCDR (XCAR (parents))) as c_int;
	  let this_code: c_uint = code.wrapping_sub (code_offset as c_uint);

	  charset = CHARSET_FROM_ID! (id as isize);
	  c = DECODE_CHAR! (charset, this_code);
	  if c >= 0
	    { break; }
	  parents = XCDR (parents);
	}
    }
  else
    {
      char_index = CODE_POINT_TO_INDEX! (charset, code);
      if char_index < 0
	{ return -1; }

      if method == CHARSET_METHOD_MAP
	{
	  let mut decoder: Lisp_Object = Qnil ();

	  decoder = CHARSET_DECODER! (charset);
	  if ! VECTORP (decoder)
	    {
	      load_charset (charset, 1);
	      decoder = CHARSET_DECODER! (charset);
	    }
	  if VECTORP (decoder)
	    { c = XFIXNUM (AREF (decoder, char_index as isize)) as c_int; }
	  else
	    { c = GET_TEMP_CHARSET_WORK_DECODER! (char_index); }
	}
      else			/* method == CHARSET_METHOD_OFFSET */
	{
	  c = char_index + CHARSET_CODE_OFFSET! (charset);
	  if CHARSET_UNIFIED_P! (charset)
	      && MAX_UNICODE_CHAR < c && c <= MAX_5_BYTE_CHAR
	    {
	      /* Unify C with a Unicode character if possible.  */
	      let val: Lisp_Object = CHAR_TABLE_REF (Vchar_unify_table, c as isize);
	      c = maybe_unify_char (c, val);
	    }
	}
    }

  return c;
}

/* Variable used temporarily by the macro ENCODE_CHAR.  */
#[unsafe(no_mangle)]
pub static mut charset_work: Lisp_Object = lisp_h_Qnil ();

/* Return a code-point of C in CHARSET.  If C doesn't belong to
   CHARSET, return CHARSET_INVALID_CODE (CHARSET).  If STRICT is true,
   use CHARSET's strict_max_char instead of max_char.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn encode_char (charset_: *mut charset, c: c_int) -> c_uint
{
  let mut charset: *mut charset = charset_;
  let mut c: c_int = c;
  let mut code: c_uint = 0;
  let method: charset_method = CHARSET_METHOD! (charset);

  if CHARSET_UNIFIED_P! (charset)
    {
      let mut deunifier: Lisp_Object = Qnil ();
      let mut code_index: c_int = -1;

      deunifier = CHARSET_DEUNIFIER! (charset);
      if ! CHAR_TABLE_P (deunifier)
	{
	  load_charset (charset, 2);
	  deunifier = CHARSET_DEUNIFIER! (charset);
	}
      if CHAR_TABLE_P (deunifier)
	{
	  let deunified: Lisp_Object = CHAR_TABLE_REF (deunifier, c as isize);

	  if FIXNUMP (deunified)
	    { code_index = XFIXNUM (deunified) as c_int; }
	}
      else
	{
	  code_index = GET_TEMP_CHARSET_WORK_ENCODER! (c);
	}
      if code_index >= 0
	{ c = (CHARSET_CODE_OFFSET! (charset) as c_int) + code_index; }
    }

  if method == CHARSET_METHOD_SUBSET
    {
      let mut subset_info: Lisp_Object = Qnil ();
      let mut this_charset: *mut charset = std::ptr::null_mut ();

      subset_info = CHARSET_SUBSET! (charset);
      this_charset = CHARSET_FROM_ID! (XFIXNAT (AREF (subset_info, 0)) as isize);
      code = ENCODE_CHAR! (this_charset, c);
      if code == CHARSET_INVALID_CODE! (this_charset)
	  || code < XFIXNAT (AREF (subset_info, 1)) as c_uint
	  || code > XFIXNAT (AREF (subset_info, 2)) as c_uint
	{ return CHARSET_INVALID_CODE! (charset); }
      code = code.wrapping_add (XFIXNUM (AREF (subset_info, 3)) as c_uint);
      return code;
    }

  if method == CHARSET_METHOD_SUPERSET
    {
      let mut parents: Lisp_Object = Qnil ();

      parents = CHARSET_SUPERSET! (charset);
      while CONSP (parents)
	{
	  let id: c_int = XFIXNUM (XCAR (XCAR (parents))) as c_int;
	  let code_offset: c_int = XFIXNUM (XCDR (XCAR (parents))) as c_int;
	  let this_charset: *mut charset = CHARSET_FROM_ID! (id as isize);

	  code = ENCODE_CHAR! (this_charset, c);
	  if code != CHARSET_INVALID_CODE! (this_charset)
	    { return code.wrapping_add (code_offset as c_uint); }
	  parents = XCDR (parents);
	}
      return CHARSET_INVALID_CODE! (charset);
    }

  if ! CHARSET_FAST_MAP_REF! (c, (*charset).fast_map.as_ptr ())
      || c < CHARSET_MIN_CHAR! (charset) || c > CHARSET_MAX_CHAR! (charset)
    { return CHARSET_INVALID_CODE! (charset); }

  if method == CHARSET_METHOD_MAP
    {
      let mut encoder: Lisp_Object = Qnil ();
      let mut val: Lisp_Object = Qnil ();

      encoder = CHARSET_ENCODER! (charset);
      if ! CHAR_TABLE_P (CHARSET_ENCODER! (charset))
	{
	  load_charset (charset, 2);
	  encoder = CHARSET_ENCODER! (charset);
	}
      if CHAR_TABLE_P (encoder)
	{
	  val = CHAR_TABLE_REF (encoder, c as isize);
	  if NILP (val)
	    { return CHARSET_INVALID_CODE! (charset); }
	  code = XFIXNUM (val) as c_uint;
	  if ! CHARSET_COMPACT_CODES_P! (charset)
	    { code = INDEX_TO_CODE_POINT! (charset, code); }
	}
      else
	{
	  code = GET_TEMP_CHARSET_WORK_ENCODER! (c) as c_uint;
	  code = INDEX_TO_CODE_POINT! (charset, code);
	}
    }
  else				/* method == CHARSET_METHOD_OFFSET */
    {
      let mut code_index: c_uint = c.wrapping_sub (CHARSET_CODE_OFFSET! (charset) as c_int) as c_uint;

      code = INDEX_TO_CODE_POINT! (charset, code_index);
    }

  return code;
}


DEFUN! ("decode-char", Fdecode_char, Sdecode_char, 2, 2, 0,
       doc: "Decode the pair of CHARSET and CODE-POINT into a character.\nReturn nil if CODE-POINT is not valid in CHARSET.\n\nCODE-POINT may be a cons (HIGHER-16-BIT-VALUE . LOWER-16-BIT-VALUE),\nalthough this usage is obsolescent.  ",
       (charset: Lisp_Object, code_point: Lisp_Object),
       {
	 let mut c: c_int = 0;
	 let mut id: c_int = 0;
	 let mut code: c_uint = 0;
	 let mut charsetp: *mut charset = std::ptr::null_mut ();

	 CHECK_CHARSET_GET_ID! (charset, id);
	 code = cons_to_unsigned (code_point, UINT_MAX as u64) as c_uint;
	 charsetp = CHARSET_FROM_ID! (id as isize);
	 c = DECODE_CHAR! (charsetp, code);
	 return if c >= 0 { make_fixnum (c as EMACS_INT) } else { Qnil () };
       });


DEFUN! ("encode-char", Fencode_char, Sencode_char, 2, 2, 0,
       doc: "Encode the character CH into a code-point of CHARSET.\nReturn the encoded code-point as an integer,\nor nil if CHARSET doesn't support CH.  ",
       (ch: Lisp_Object, charset: Lisp_Object),
       {
	 let mut c: c_int = 0;
	 let mut id: c_int = 0;
	 let mut code: c_uint = 0;
	 let mut charsetp: *mut charset = std::ptr::null_mut ();

	 CHECK_CHARSET_GET_ID! (charset, id);
	 CHECK_CHARACTER (ch);
	 c = XFIXNAT (ch) as c_int;
	 charsetp = CHARSET_FROM_ID! (id as isize);
	 code = ENCODE_CHAR! (charsetp, c);
	 if code == CHARSET_INVALID_CODE! (charsetp)
	   { return Qnil (); }
	 /* There are much fewer codepoints in the world than we have positive
	    fixnums, so it could be argued that we never really need a bignum,
	    e.g. Unicode codepoints only need 21bit, and China's GB-10830
	    can fit in 22bit.  Yet we encode GB-10830's chars in a sparse way
	    (we just take the 4byte sequences as a 32bit int), so some
	    GB-10830 chars (such as 0x81308130 in etc/charsets/gb108304.map) end
	    up represented as bignums if EMACS_INT is 32 bits.  */
	 return INT_TO_INTEGER (code as EMACS_INT);
       });


DEFUN! ("make-char", Fmake_char, Smake_char, 1, 5, 0,
       doc: "\nReturn a character of CHARSET whose position codes are CODEn.\n\nCODE1 through CODE4 are optional, but if you don't supply sufficient\nposition codes, it is assumed that the minimum code in each dimension\nis specified.  ",
       (charset: Lisp_Object, code1: Lisp_Object, code2: Lisp_Object, code3: Lisp_Object,
	code4: Lisp_Object),
       {
	 let mut id: c_int = 0;
	 let mut dimension: c_int = 0;
	 let mut charsetp: *mut charset = std::ptr::null_mut ();
	 let mut code: c_uint = 0;
	 let mut c: c_int = 0;

	 CHECK_CHARSET_GET_ID! (charset, id);
	 charsetp = CHARSET_FROM_ID! (id as isize);

	 dimension = CHARSET_DIMENSION! (charsetp);
	 if NILP (code1)
	   { code = if CHARSET_ASCII_COMPATIBLE_P! (charsetp) { 0 } else { CHARSET_MIN_CODE! (charsetp) }; }
	 else
	   {
	     CHECK_FIXNAT (code1);
	     if XFIXNAT (code1) >= 0x100
	       { args_out_of_range (make_fixnum (0xFF), code1); }
	     code = XFIXNAT (code1) as c_uint;

	     if dimension > 1
	       {
		 code <<= 8;
		 if NILP (code2)
		   { code |= (*charsetp).code_space[((dimension - 2) * 4) as usize] as c_uint; }
		 else
		   {
		     CHECK_FIXNAT (code2);
		     if XFIXNAT (code2) >= 0x100
		       { args_out_of_range (make_fixnum (0xFF), code2); }
		     code |= XFIXNAT (code2) as c_uint;
		   }

		 if dimension > 2
		   {
		     code <<= 8;
		     if NILP (code3)
		       { code |= (*charsetp).code_space[((dimension - 3) * 4) as usize] as c_uint; }
		     else
		       {
			 CHECK_FIXNAT (code3);
			 if XFIXNAT (code3) >= 0x100
			   { args_out_of_range (make_fixnum (0xFF), code3); }
			 code |= XFIXNAT (code3) as c_uint;
		       }

		     if dimension > 3
		       {
			 code <<= 8;
			 if NILP (code4)
			   { code |= (*charsetp).code_space[0] as c_uint; }
			 else
			   {
			     CHECK_FIXNAT (code4);
			     if XFIXNAT (code4) >= 0x100
			       { args_out_of_range (make_fixnum (0xFF), code4); }
			     code |= XFIXNAT (code4) as c_uint;
			   }
		       }
		   }
	       }
	   }

	 if CHARSET_ISO_FINAL! (charsetp) >= 0
	   { code &= 0x7F7F7F7F; }
	 c = DECODE_CHAR! (charsetp, code);
	 if c < 0
	   { error (c"Invalid code(s)".as_ptr ()); }
	 return make_fixnum (c as EMACS_INT);
       });


/* Return the first charset in CHARSET_LIST that contains C.
   CHARSET_LIST is a list of charset IDs.  If it is nil, use
   Vcharset_ordered_list.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn char_charset (c: c_int, charset_list_: Lisp_Object,
				code_return: *mut c_uint) -> *mut charset
{
  let mut charset_list: Lisp_Object = charset_list_;
  let mut maybe_null: bool = false;

  if NILP (charset_list)
    { charset_list = Vcharset_ordered_list; }
  else
    { maybe_null = true; }

  while CONSP (charset_list)
    {
      let charset: *mut charset = CHARSET_FROM_ID! (XFIXNUM (XCAR (charset_list)));
      let code: c_uint = ENCODE_CHAR! (charset, c);

      if code != CHARSET_INVALID_CODE! (charset)
	{
	  if ! code_return.is_null ()
	    { *code_return = code; }
	  return charset;
	}
      charset_list = XCDR (charset_list);
      if ! maybe_null
	  && c <= MAX_UNICODE_CHAR
	  && EQ (charset_list, Vcharset_non_preferred_head)
	{ return CHARSET_FROM_ID! (charset_unicode); }
    }
  return if maybe_null { std::ptr::null_mut () }
	 else if c <= MAX_5_BYTE_CHAR { CHARSET_FROM_ID! (charset_emacs) }
	 else { CHARSET_FROM_ID! (charset_eight_bit) };
}


DEFUN! ("split-char", Fsplit_char, Ssplit_char, 1, 1, 0,
       doc: "Return list of charset and one to four position-codes of CH.\nThe charset is decided by the current priority order of charsets.\nA position-code is a byte value of each dimension of the code-point of\nCH in the charset.  ",
       (ch: Lisp_Object),
       {
	 let mut charset: *mut charset = std::ptr::null_mut ();
	 let mut c: c_int = 0;
	 let mut dimension: c_int = 0;
	 let mut code: c_uint = 0;
	 let mut val: Lisp_Object = Qnil ();

	 CHECK_CHARACTER (ch);
	 c = XFIXNAT (ch) as c_int;
	 charset = CHAR_CHARSET! (c);
	 if charset.is_null ()
	   { emacs_abort (); }
	 code = ENCODE_CHAR! (charset, c);
	 if code == CHARSET_INVALID_CODE! (charset)
	   { emacs_abort (); }
	 dimension = CHARSET_DIMENSION! (charset);
	 val = Qnil ();
	 while dimension > 0
	   {
	     val = Fcons (make_fixnum ((code & 0xFF) as EMACS_INT), val);
	     code >>= 8;
	     dimension -= 1;
	   }
	 return Fcons (CHARSET_NAME! (charset), val);
       });


DEFUN! ("char-charset", Fchar_charset, Schar_charset, 1, 2, 0,
       doc: "Return the charset of highest priority that contains CH.\nASCII characters are an exception: for them, this function always\nreturns `ascii'.\nIf optional 2nd arg RESTRICTION is non-nil, it is a list of charsets\nfrom which to find the charset.  It may also be a coding system.  In\nthat case, find the charset in those supported by that coding system.  ",
       (ch: Lisp_Object, restriction: Lisp_Object),
       {
	 let mut restriction: Lisp_Object = restriction;
	 let mut charset: *mut charset = std::ptr::null_mut ();

	 CHECK_CHARACTER (ch);
	 if NILP (restriction)
	   { charset = CHAR_CHARSET! (XFIXNUM (ch) as c_int); }
	 else
	   {
	     if CONSP (restriction)
	       {
		 let c: c_int = XFIXNAT (ch) as c_int;

		 while CONSP (restriction)
		   {
		     let mut rcharset: *mut charset = std::ptr::null_mut ();

		     CHECK_CHARSET_GET_CHARSET! (XCAR (restriction), rcharset);
		     if ENCODE_CHAR! (rcharset, c) != CHARSET_INVALID_CODE! (rcharset)
		       { return XCAR (restriction); }
		     restriction = XCDR (restriction);
		   }
		 return Qnil ();
	       }
	     restriction = coding_system_charset_list (restriction);
	     charset = char_charset (XFIXNUM (ch) as c_int, restriction, std::ptr::null_mut ());
	     if charset.is_null ()
	       { return Qnil (); }
	   }
	 return CHARSET_NAME! (charset);
       });


DEFUN! ("charset-after", Fcharset_after, Scharset_after, 0, 1, 0,
       doc: "\nReturn charset of a character in the current buffer at position POS.\nIf POS is nil, it defaults to the current point.\nIf POS is out of range, the value is nil.  ",
       (pos: Lisp_Object),
       {
	 let mut ch: Lisp_Object = Qnil ();
	 let mut charset: *mut charset = std::ptr::null_mut ();

	 ch = Fchar_after (pos);
	 if ! FIXNUMP (ch)
	   { return ch; }
	 charset = CHAR_CHARSET! (XFIXNUM (ch) as c_int);
	 return CHARSET_NAME! (charset);
       });

DEFUN! ("iso-charset", Fiso_charset, Siso_charset, 3, 3, 0,
       doc: "\nReturn charset of ISO's specification DIMENSION, CHARS, and FINAL-CHAR.\n\nISO 2022's designation sequence (escape sequence) distinguishes charsets\nby their DIMENSION, CHARS, and FINAL-CHAR,\nwhereas Emacs distinguishes them by charset symbol.\nSee the documentation of the function `charset-info' for the meanings of\nDIMENSION, CHARS, and FINAL-CHAR.  ",
       (dimension: Lisp_Object, chars: Lisp_Object, final_char: Lisp_Object),
       {
	 let chars_flag: bool = check_iso_charset_parameter (dimension, chars, final_char);
	 let id: c_int = ISO_CHARSET_TABLE! (XFIXNUM (dimension), chars_flag as c_int,
					     XFIXNAT (final_char) as c_int);
	 return if id >= 0 { CHARSET_NAME! (CHARSET_FROM_ID! (id as isize)) } else { Qnil () };
       });

/* Shrink charset_table to charset_table.used.  */
unsafe fn shrink_charset_table ()
{
  eassert! (charset_table.size >= charset_table.used);
  eassert! (ASIZE (charset_table.attributes_table) == charset_table.size as isize);

  if charset_table.size > charset_table.used
    {
      eassert! (! pdumper_object_p (charset_table.start as *const c_void));
      charset_table.start
	= xnrealloc (charset_table.start as *mut c_void,
		     charset_table.used as ptrdiff_t,
		     std::mem::size_of::<charset> () as ptrdiff_t) as *mut charset;

      charset_table.attributes_table
	= Fvector (charset_table.used as ptrdiff_t,
		   xvector_contents (charset_table.attributes_table) as *mut Lisp_Object);

      charset_table.size = charset_table.used;
    }
  eassert! (charset_table.size == charset_table.used);
  eassert! (ASIZE (charset_table.attributes_table) == charset_table.size as isize);
}

DEFUN! ("clear-charset-maps", Fclear_charset_maps, Sclear_charset_maps,
       0, 0, 0,
       doc: "\nInternal use only.\nClear temporary charset mapping tables.\nIt should be called only from temacs invoked for dumping.  ",
       (),
       {
	 if ! temp_charset_work.is_null ()
	   {
	     xfree (temp_charset_work as *mut c_void);
	     temp_charset_work = std::ptr::null_mut ();
	   }

	 if CHAR_TABLE_P (Vchar_unify_table)
	   { Foptimize_char_table (Vchar_unify_table, Qnil ()); }

	 shrink_charset_table ();

	 return Qnil ();
       });

DEFUN! ("charset-priority-list", Fcharset_priority_list,
       Scharset_priority_list, 0, 1, 0,
       doc: "Return the list of charsets ordered by priority.\nHIGHESTP non-nil means just return the highest priority one.  ",
       (highestp: Lisp_Object),
       {
	 let mut val: Lisp_Object = Qnil ();
	 let mut list: Lisp_Object = Vcharset_ordered_list;

	 if ! NILP (highestp)
	   { return CHARSET_NAME! (CHARSET_FROM_ID! (XFIXNUM (Fcar (list)) as isize)); }

	 while ! NILP (list)
	   {
	     val = Fcons (CHARSET_NAME! (CHARSET_FROM_ID! (XFIXNUM (XCAR (list)) as isize)), val);
	     list = XCDR (list);
	   }
	 return Fnreverse (val);
       });

DEFUN! ("set-charset-priority", Fset_charset_priority, Sset_charset_priority,
       1, MANY, 0,
       doc: "Assign higher priority to the charsets given as arguments.\nusage: (set-charset-priority &rest charsets)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
	 let mut new_head: Lisp_Object = Qnil ();
	 let mut old_list: Lisp_Object = Qnil ();
	 let mut list_2022: Lisp_Object = Qnil ();
	 let mut list_emacs_mule: Lisp_Object = Qnil ();
	 let mut i: ptrdiff_t = 0;
	 let mut id: c_int = 0;

	 old_list = Fcopy_sequence (Vcharset_ordered_list);
	 new_head = Qnil ();
	 i = 0;
	 while i < nargs
	   {
	     CHECK_CHARSET_GET_ID! (*args.offset (i), id);
	     if ! NILP (Fmemq (make_fixnum (id as EMACS_INT), old_list))
	       {
		 old_list = Fdelq (make_fixnum (id as EMACS_INT), old_list);
		 new_head = Fcons (make_fixnum (id as EMACS_INT), new_head);
	       }
	     i += 1;
	   }
	 Vcharset_non_preferred_head = old_list;
	 Vcharset_ordered_list = nconc2 (Fnreverse (new_head), old_list);

	 charset_ordered_list_tick += 1;

	 charset_unibyte = -1;
	 old_list = Vcharset_ordered_list;
	 list_2022 = Qnil ();
	 list_emacs_mule = Qnil ();
	 while CONSP (old_list)
	   {
	     if ! NILP (Fmemq (XCAR (old_list), Viso_2022_charset_list))
	       { list_2022 = Fcons (XCAR (old_list), list_2022); }
	     if ! NILP (Fmemq (XCAR (old_list), Vemacs_mule_charset_list))
	       { list_emacs_mule = Fcons (XCAR (old_list), list_emacs_mule); }
	     if charset_unibyte < 0
	       {
		 let charset: *mut charset = CHARSET_FROM_ID! (XFIXNUM (XCAR (old_list)));

		 if CHARSET_DIMENSION! (charset) == 1
		     && CHARSET_ASCII_COMPATIBLE_P! (charset)
		     && CHARSET_MAX_CHAR! (charset) >= 0x80
		   { charset_unibyte = CHARSET_ID! (charset); }
	       }
	     old_list = XCDR (old_list);
	   }
	 Viso_2022_charset_list = Fnreverse (list_2022);
	 Vemacs_mule_charset_list = Fnreverse (list_emacs_mule);
	 if charset_unibyte < 0
	   { charset_unibyte = charset_iso_8859_1; }

	 return Qnil ();
       });

DEFUN! ("charset-id-internal", Fcharset_id_internal, Scharset_id_internal,
       0, 1, 0,
       doc: "Internal use only.\nReturn charset identification number of CHARSET.  ",
       (charset: Lisp_Object),
       {
	 let mut id: c_int = 0;

	 CHECK_CHARSET_GET_ID! (charset, id);
	 return make_fixnum (id as EMACS_INT);
       });

#[repr(C)]
pub struct charset_sort_data
{
  pub charset: Lisp_Object,
  pub id: c_int,
  pub priority: ptrdiff_t,
}

unsafe extern "C" fn charset_compare (d1: *const c_void, d2: *const c_void) -> c_int
{
  let data1: *const charset_sort_data = d1 as *const charset_sort_data;
  let data2: *const charset_sort_data = d2 as *const charset_sort_data;
  if (*data1).priority != (*data2).priority
    { return if (*data1).priority < (*data2).priority { -1 } else { 1 }; }
  return 0;
}

DEFUN! ("sort-charsets", Fsort_charsets, Ssort_charsets, 1, 1, 0,
       doc: "Sort charset list CHARSETS by a priority of each charset.\nReturn the sorted list.  CHARSETS is modified by side effects.\nSee also `charset-priority-list' and `set-charset-priority'.  ",
       (charsets: Lisp_Object),
       {
	 let n: ptrdiff_t = list_length (charsets);
	 let mut i: ptrdiff_t = 0;
	 let mut j: ptrdiff_t = 0;
	 let mut done: c_int = 0;
	 let mut tail: Lisp_Object = Qnil ();
	 let mut elt: Lisp_Object = Qnil ();
	 let mut attrs: Lisp_Object = Qnil ();
	 let mut id: c_int = 0;
	 let mut min_id: c_int = INT_MAX;
	 let mut max_id: c_int = INT_MIN;
	 USE_SAFE_ALLOCA! (sa_avail, sa_count);

	 if n == 0
	   { return Qnil (); }
	 let mut sort_data: *mut charset_sort_data = std::ptr::null_mut ();
	 SAFE_NALLOCA! (sa_avail, sort_data, 1, n);
	 tail = charsets;
	 i = 0;
	 while CONSP (tail)
	   {
	     elt = XCAR (tail);
	     CHECK_CHARSET_GET_ATTR! (elt, attrs);
	     (*sort_data.offset (i)).charset = elt;
	     id = XFIXNUM (CHARSET_ATTR_ID! (attrs)) as c_int;
	     (*sort_data.offset (i)).id = id;
	     if id < min_id
	       { min_id = id; }
	     if id > max_id
	       { max_id = id; }
	     tail = XCDR (tail);
	     i += 1;
	   }
	 done = 0;
	 tail = Vcharset_ordered_list;
	 i = 0;
	 while done < n as c_int && CONSP (tail)
	   {
	     elt = XCAR (tail);
	     id = XFIXNAT (elt) as c_int;
	     if id >= min_id && id <= max_id
	       {
		 j = 0;
		 while j < n
		   {
		     if (*sort_data.offset (j)).id == id
		       {
			 (*sort_data.offset (j)).priority = i;
			 done += 1;
		       }
		     j += 1;
		   }
	       }
	     tail = XCDR (tail);
	     i += 1;
	   }
	 qsort (sort_data as *mut c_void, n as usize,
		std::mem::size_of::<charset_sort_data> (), Some (charset_compare));
	 i = 0;
	 tail = charsets;
	 while CONSP (tail)
	   {
	     XSETCAR (tail, (*sort_data.offset (i)).charset);
	     tail = XCDR (tail);
	     i += 1;
	   }
	 SAFE_FREE! (sa_count);
	 return charsets;
       });



#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_charset ()
{
  let mut tempdir: Lisp_Object = Qnil ();
  tempdir = Fexpand_file_name (build_string (c"charsets".as_ptr ()), Vdata_directory);
  if ! file_accessible_directory_p (tempdir)
    {
      /* This used to be non-fatal (dir_warning), but it should not
	 happen, and if it does sooner or later it will cause some
	 obscure problem (eg bug#6401), so better exit.  */
      fprintf (stderr,
	       c"Error: %s: %s\nEmacs will not function correctly without the character map files.\n%sPlease check your installation!\n".as_ptr (),
	       SDATA (tempdir), strerror (*errno ()),
	       if ! egetenv (c"EMACSDATA".as_ptr ()).is_null ()
		 { c"The EMACSDATA environment variable is set.  Maybe it has the wrong value?\n".as_ptr () }
	       else { c"".as_ptr () });
      exit (1);
    }

  Vcharset_map_path = list1 (tempdir);
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_charset_once ()
{
  let mut i: c_int = 0;
  let mut j: c_int = 0;
  let mut k: c_int = 0;

  for i in 0..ISO_MAX_DIMENSION
    {
      for j in 0..ISO_MAX_CHARS
	{
	  for k in 0..ISO_MAX_FINAL
	    {
	      iso_charset_table[i as usize][j as usize][k as usize] = -1;
	    }
	}
    }

  PDUMPER_REMEMBER_SCALAR! (iso_charset_table);

  for i in 0..256
    { emacs_mule_charset[i as usize] = -1; }

  PDUMPER_REMEMBER_SCALAR! (emacs_mule_charset);

  charset_jisx0201_roman = -1;
  PDUMPER_REMEMBER_SCALAR! (charset_jisx0201_roman);

  charset_jisx0208_1978 = -1;
  PDUMPER_REMEMBER_SCALAR! (charset_jisx0208_1978);

  charset_jisx0208 = -1;
  PDUMPER_REMEMBER_SCALAR! (charset_jisx0208);

  charset_ksc5601 = -1;
  PDUMPER_REMEMBER_SCALAR! (charset_ksc5601);
}

/* Initial value for charset_table_size.  As of October 2025, the
   charset table uses 179 entries.  charset_table grows if needed and
   we only dump the used entries; so getting the initial value right
   is not overly important.  */
pub const CHARSET_TABLE_INIT_SIZE: c_int = 190;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_charset ()
{
  DEFSYM! (Qcharsetp, c"charsetp");
  DEFSYM! (Qdefine_charset_internal, c"define-charset-internal");

  /* Special charset symbols.  */
  DEFSYM! (Qascii, c"ascii");
  DEFSYM! (Qunicode, c"unicode");
  DEFSYM! (Qemacs, c"emacs");
  DEFSYM! (Qeight_bit, c"eight-bit");
  DEFSYM! (Qiso_8859_1, c"iso-8859-1");

  staticpro (&raw const Vcharset_ordered_list);
  Vcharset_ordered_list = Qnil ();

  staticpro (&raw const Viso_2022_charset_list);
  Viso_2022_charset_list = Qnil ();

  staticpro (&raw const Vemacs_mule_charset_list);
  Vemacs_mule_charset_list = Qnil ();

  staticpro (&raw const Vcharset_hash_table);
  Vcharset_hash_table = CALLN! (Fmake_hash_table, QCtest (), Qeq ());

  charset_table.size = CHARSET_TABLE_INIT_SIZE;
  PDUMPER_REMEMBER_SCALAR! (charset_table.size);
  charset_table.start
    = xmalloc ((charset_table.size as usize) * std::mem::size_of::<charset> ()) as *mut charset;
  charset_table.used = 0;
  PDUMPER_REMEMBER_SCALAR! (charset_table.used);

  charset_table.attributes_table
    = make_vector (charset_table.size as ptrdiff_t, Qnil ());

  staticpro (&raw const charset_table.attributes_table);
  defsubr (&raw mut Scharsetp);
  defsubr (&raw mut Smap_charset_chars);
  defsubr (&raw mut Sdefine_charset_internal);
  defsubr (&raw mut Sdefine_charset_alias);
  defsubr (&raw mut Scharset_plist);
  defsubr (&raw mut Sset_charset_plist);
  defsubr (&raw mut Sunify_charset);
  defsubr (&raw mut Sget_unused_iso_final_char);
  defsubr (&raw mut Sdeclare_equiv_charset);
  defsubr (&raw mut Sfind_charset_region);
  defsubr (&raw mut Sfind_charset_string);
  defsubr (&raw mut Sdecode_char);
  defsubr (&raw mut Sencode_char);
  defsubr (&raw mut Ssplit_char);
  defsubr (&raw mut Smake_char);
  defsubr (&raw mut Schar_charset);
  defsubr (&raw mut Scharset_after);
  defsubr (&raw mut Siso_charset);
  defsubr (&raw mut Sclear_charset_maps);
  defsubr (&raw mut Scharset_priority_list);
  defsubr (&raw mut Sset_charset_priority);
  defsubr (&raw mut Scharset_id_internal);
  defsubr (&raw mut Ssort_charsets);

  DEFVAR_LISP! (c"charset-map-path", Vcharset_map_path,
	       "List of directories to search for charset map files.  ");
  Vcharset_map_path = Qnil ();

  DEFVAR_BOOL! (c"inhibit-load-charset-map", inhibit_load_charset_map,
	       "Inhibit loading of charset maps.  Used when dumping Emacs.  ");
  inhibit_load_charset_map = false;

  DEFVAR_LISP! (c"charset-list", Vcharset_list,
	       "List of all charsets ever defined.  ");
  Vcharset_list = Qnil ();

  DEFVAR_LISP! (c"current-iso639-language", Vcurrent_iso639_language,
	       "ISO639 language mnemonic symbol for the current language environment.\nIf the current language environment is for multiple languages (e.g. \"Latin-1\"),\nthe value may be a list of mnemonics.  ");
  Vcurrent_iso639_language = Qnil ();

  charset_ascii
    = define_charset_internal (Qascii (), 1, b"\x00\x7F\0\0\0\0\0".as_ptr () as *const c_char,
			       0, 127, b'B' as c_int, -1, 0, true, false, 0);
  PDUMPER_REMEMBER_SCALAR! (charset_ascii);

  charset_iso_8859_1
    = define_charset_internal (Qiso_8859_1 (), 1, b"\x00\xFF\0\0\0\0\0".as_ptr () as *const c_char,
			       0, 255, -1, -1, -1, true, false, 0);
  PDUMPER_REMEMBER_SCALAR! (charset_iso_8859_1);

  charset_unicode
    = define_charset_internal (Qunicode (), 3, b"\x00\xFF\x00\xFF\x00\x10\0".as_ptr () as *const c_char,
			       0, MAX_UNICODE_CHAR as c_uint, -1, 0, -1, true, false, 0);
  PDUMPER_REMEMBER_SCALAR! (charset_unicode);

  charset_emacs
    = define_charset_internal (Qemacs (), 3, b"\x00\xFF\x00\xFF\x00\x3F\0".as_ptr () as *const c_char,
			       0, MAX_5_BYTE_CHAR as c_uint, -1, 0, -1, true, true, 0);
  PDUMPER_REMEMBER_SCALAR! (charset_emacs);

  charset_eight_bit
    = define_charset_internal (Qeight_bit (), 1, b"\x80\xFF\0\0\0\0\0".as_ptr () as *const c_char,
			       128, 255, -1, 0, -1, false, true,
			       MAX_5_BYTE_CHAR + 1);
  PDUMPER_REMEMBER_SCALAR! (charset_eight_bit);

  charset_unibyte = charset_iso_8859_1;
  PDUMPER_REMEMBER_SCALAR! (charset_unibyte);
}
