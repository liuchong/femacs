// femacs: port of src/print.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, struct/union field, file-scope object, local), same control flow, same error model.
// C's 11 `DEFUN`s use `DEFUN!` (PORTING.md 5.1); the 47 functions, 3 macros, 6 types and 12
// file-scope objects of `docs/ports/print.md` are all here.
//
// **Carried arms (docs/status.md 3.49, PORTING.md P-51).**  `src/print.c` is *not* in
// `docs/baseline/config/cond-eval-verified.txt`: `python3 tools/check-cond-eval.py
// src/print.c` now stands at `14 conditions, 0 unknown, 0 disagreements [VERIFIED]` - it stopped
// disagreeing when the `lisp.h\tIEEE_FLOATING_POINT` row landed (status 3.70).  It is still *not*
// added to `cond-eval-verified.txt`, because a verified file's checker demands only the live arm
// while this port carries both (status 3.51), so the evaluator stays blind here on purpose.  When
// this lane wrote the text below the count was `2 disagreements [NOT VERIFIED]`, the two
// being `#if IEEE_FLOATING_POINT` (C 42, C 1200) — a *source-header* macro (`lisp.h
// 3201-3202` computes it from `<float.h>`, so the frozen table cannot see it; clang says
// true).  `tools/extract-abi.py`'s blind evaluator therefore keeps both arms of every `#if`
// live, and per the integrator's decision the port *carries* the text of the arms the frozen
// build does not compile, each under a `#[cfg (feature = "…")]` naming the C condition's
// non-frozen side.  The arms, in C order:
//
//   * C 46-48, C 264-269, C 282-285: `WINDOWSNT` (feature `windowsnt`).
//   * C 50-52, C 2054-2062, C 2066-2100, C 2104-2107: `HAVE_TREE_SITTER`
//     (feature `have_tree_sitter`).
//   * C 1612-1639, C 2009-2036: `HAVE_MODULES` (feature `have_modules`).
//   * C 1839-1858: `HAVE_XWIDGETS` with its `USE_GTK` sub-arm (features `have_xwidgets`,
//     `use_gtk`).
//   * C 2040-2050: `HAVE_NATIVE_COMP` (feature `have_native_comp`).
//
// C 42-44 (`#include <ieee754.h>`) and C 1200-1208 (the NaN arm of `float_to_string`) are
// live for this target, so they are plain text; `<ieee754.h>` has no mirror yet (it is one of
// gnulib's generated `*.in.h` templates, `emacs/lib/ieee754.in.h`), so the union is declared
// at its use site below with the frozen target's clang-measured bit layout and the mirror gap
// is reported in `docs/ports/print.md`.
//
// C's uninitialised locals (`Lisp_Object dv UNINIT;` in `printchar_to_stream`, `char *cp;` in
// `float_to_string`) are zero-initialised (PORTING.md 1.5) and C 622-625's three forward
// declarations are dropped (PORTING.md 0.1-12); both are named in the ledger, as is the
// `EXTERNALLY_VISIBLE` attribute, which has no Rust counterpart (PORTING.md 8: `#[no_mangle]
// pub` is the closest thing and it is already the reason those definitions are exported).
//
// `print_output_debug_flag` (C 93-94) is the file's one `extern` object besides
// `Vprin1_to_string_buffer` (C 793); both are exported.  The `DEFVAR_*` storage this file owns
// is defined here as well (docs/status.md 3.29, the casetab.rs shape) because C's storage is a
// field of `struct emacs_globals`, whose single definition is alloc.c's.
//
// Names no mirror carries are declared at their use site (the terminal.rs/filelock.rs shape),
// each with C's own prototype in a `/// C:` line; the ones the frozen target renames are
// linked through `#[link_name]` after reading the reference object's undefined symbols
// (`nm -u build/emacs-ref/src/print.o`, docs/status.md 3.57): `putc` and the two stdio
// streams.  Every one of those gaps is listed in `docs/ports/print.md`.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_parens, unreachable_code, unused_assignments,
         unused_mut, unused_labels, while_true, overflowing_literals, unused_comparisons,
         clashing_extern_declarations)]

include!("include/config.rs");                 // C 21: `#include <config.h>`
include!("../lib/include/stdio.rs");           // C 22: sysstdio.h 24's `#include <stdio.h>`
include!("../lib/include/string.rs");          // lisp.h 30's `<string.h>` (memcpy, memset,
                                               //         strlen, strcpy)
include!("../lib/include/c-ctype.rs");         // C 34: `#include <c-ctype.h>`
include!("../lib/include/float.in.rs");        // C 35: `#include <float.h>` (DBL_DIG)
include!("../lib/include/math.rs");            // C 40: `#include <math.h>` (isinf, isnan)
include!("../lib/include/fcntl.in.rs");        // Fredirect_debugging_output's fcntl, O_*
include!("../lib/include/unistd.in.rs");       // dup2, STDERR_FILENO's header
include!("../lib/include/stdint.in.rs");       // UINTMAX_WIDTH (print_object's buffer size)
include!("../lib/include/limits.in.rs");       // INTMAX_WIDTH (the same expression)
include!("include/lisp.rs");                   // C 25: `#include "lisp.h"`
include!("include/character.rs");              // C 26: `#include "character.h"`
include!("include/charset.rs");                // C 29: `#include "charset.h"`
include!("include/itree.rs");                  // buffer.h 28: `#include "itree.h"`
include!("include/intervals.rs");              // C 33: `#include "intervals.h"`
include!("include/coding.rs");                 // C 27: `#include "coding.h"`
include!("include/buffer.rs");                 // C 28: `#include "buffer.h"`
include!("include/systime.rs");                // frame.h 24: `#include "systime.h"`
include!("include/font.rs");                   // dispextern.h's font names (FONTP family)
include!("include/composite.rs");              // coding.h 284: `#include "composite.h"`
include!("include/dispextern.rs");             // frame.h 25: `#include "dispextern.h"`
include!("include/termhooks.rs");              // frame.h's `struct terminal`
include!("include/window.rs");                 // XWINDOW, the WINDOWP family
include!("include/frame.rs");                  // C 30: `#include "frame.h"`
include!("include/process.rs");                // C 31: `#include "process.h"`
include!("include/disptab.rs");                // C 32: `#include "disptab.h"`
include!("include/blockinput.rs");             // C 36: `#include "blockinput.h"`
include!("include/dynlib.rs");                 // C 38: `#include "dynlib.h"`

use std::ffi::CStr;                            // the `-1.0e+INF` literal in float_to_string

/* C 37's `#include "xwidget.h"` has no mirror: every name print.c takes from it
   (`XXWIDGET`, `XWIDGET`, the `struct xwidget` fields) is inside the `HAVE_XWIDGETS` arm
   carried below, and `HAVE_XWIDGETS` is undefined in the frozen configuration.  C 43's
   `<ieee754.h>` and C 47's `<sys/socket.h>` are the same class (the latter is the
   `WINDOWSNT` arm).  */

/* The names C gets from headers whose mirrors do not carry them yet (the terminal.rs shape),
   each with C's own prototype in a `/// C:` line.  */
unsafe extern "C" {
  /// C: lisp.h 4799: `extern bool FUNCTIONP (Lisp_Object);`
  fn FUNCTIONP (object: Lisp_Object) -> bool;
  /// C: lisp.h 4378: `extern void message_dolog (const char *, ptrdiff_t, int, bool);`
  fn message_dolog (m: *const c_char, len: ptrdiff_t, nlflag: c_int, multibyte: bool);
  /// C: lisp.h 4348: `extern void message3 (Lisp_Object);`
  fn message3 (m: Lisp_Object);
  /// C: keyboard.h 476: `extern void setup_echo_area_for_printing (bool);`
  fn setup_echo_area_for_printing (multibyte_p: bool);
  /// C: keyboard.h 477: `extern void reset_message_log_need_newline (void);`
  fn reset_message_log_need_newline ();
  /// C: lisp.h 4331-4332: `extern void insert_1_both (const char *, ptrdiff_t, ptrdiff_t,
  /// bool, bool, bool);`
  fn insert_1_both (string: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
                    inherit: bool, prepare: bool, before_markers: bool);
  /// C: lisp.h 4323-4324: `extern ptrdiff_t copy_text (const unsigned char *,
  /// unsigned char *, ptrdiff_t, bool, bool);`
  fn copy_text (from: *const u8, to: *mut u8, nbytes: ptrdiff_t, from_multibyte: bool,
                to_multibyte: bool) -> ptrdiff_t;
  /// C: lisp.h 4825: `extern void record_unwind_protect_void (void (*) (void));`
  fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
  /// C: lisp.h 4253: `Lisp_Object hash_table_weakness_symbol (hash_table_weakness_t weak);`
  fn hash_table_weakness_symbol (weak: hash_table_weakness_t) -> Lisp_Object;
  /// C: font.h 780: `extern Lisp_Object font_style_symbolic (Lisp_Object,
  /// enum font_property_index, bool);`
  fn font_style_symbolic (font: Lisp_Object, prop: c_int, for_face: bool) -> Lisp_Object;
  /// C: lib/ftoastr.h 54: `int dtoastr (char *buf, size_t bufsize, int flags, int width,
  /// double x);`
  fn dtoastr (buf: *mut c_char, bufsize: usize, flags: c_int, width: c_int, x: c_double)
    -> c_int;
}

/* C: <unistd.h>'s `STDERR_FILENO` now comes from the unistd.in.rs mirror (2026-10-04); this
   unit's use-site copy is gone, and the mirror is the single declaration point (PORTING.md
   0.1-3/P-26).  */

/* gnulib's `<inttypes.h>` (P-26/P-50 has no mirror): the frozen target's three format
   macros, measured with clang (`printf ("%s %s %s", PRIdMAX, PRIuMAX, PRIxMAX)` prints
   `jd ju jx`).  C concatenates them into its format strings; Rust cannot concatenate a
   const into a literal, so the port writes the concatenated literal and keeps the name
   here, and `docs/ports/macro-body-notes.tsv` records the token for each body that used it
   (the filelock.rs/timefns.rs shape).  */
const PRIdMAX: &str = "jd";
const PRIuMAX: &str = "ju";
const PRIxMAX: &str = "jx";

/// C: lisp.h 71: `#define max(a, b) ((a) > (b) ? (a) : (b))`.  lisp.rs's `max` is an
/// `isize` function; an array length must be a constant, so the one place that needs the
/// fold inside a `const` (print_object's buffer) uses this const-evaluable spelling.
const fn max_usize (a: usize, b: usize) -> usize { if a > b { a } else { b } }

/* C 43's `#include <ieee754.h>`: the mirror of the generated template
   (`lib/include/ieee754.in.rs`) is the owner now, so the `union ieee754_double` and its
   `ieee_nan` accessors come from there; the measurements that used to be written out here
   (clang's packing, sizeof 8, alignof 4 for the member and 8 for the union) are recorded in
   `docs/ports/include-ieee754.in.md`.  */
include!("../lib/include/ieee754.in.rs");
/* The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
   (lisp.h 3495-3530) and `struct emacs_globals globals` is defined once, in alloc.c, so the
   reference object exports `globals` and *no* `V*` symbol for these names.  The port gives
   each one its own exported object anyway (docs/status.md 3.29), because `include/
   globals.rs` mirrors them as *external* declarations: `keyboard.c` reads `Vprint_level`
   (and `Vprint_circle` is read here), and a unit that read another unit's variable through
   that declaration would have nothing to link against if the owner kept it private.
   `tools/check-symbols.py` knows this class — it is the port's one systematic symbol
   deviation from C, and the gate counts it instead of failing on it (status 3.60).
   syms_of_print registers them and assigns the same initial values C does.  */
#[unsafe(no_mangle)]
pub static mut Vstandard_output: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vfloat_output_format: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut print_integers_as_characters: bool = false;
#[unsafe(no_mangle)]
pub static mut Vprint_length: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vprint_level: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut print_escape_newlines: bool = false;
#[unsafe(no_mangle)]
pub static mut print_escape_control_characters: bool = false;
#[unsafe(no_mangle)]
pub static mut print_escape_nonascii: bool = false;
#[unsafe(no_mangle)]
pub static mut print_escape_multibyte: bool = false;
#[unsafe(no_mangle)]
pub static mut print_quoted: bool = false;
#[unsafe(no_mangle)]
pub static mut Vprint_gensym: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vprint_circle: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vprint_continuous_numbering: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vprint_number_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vprint_charset_text_property: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut print_symbols_bare: bool = false;
#[unsafe(no_mangle)]
pub static mut Vprint_unreadable_function: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vprint__unreadable_callback_buffer: Lisp_Object = lisp_h_Qnil ();

/* gnulib's `<intprops.h>` (P-26) is `INT_STRLEN_BOUND (t)`, type-generic in C
   (`_GL_SIGNED_TYPE_OR_EXPR`); the two instantiations this file uses are `EMACS_INT`
   (signed, 64 bits) and `intmax_t` (signed, 64 bits), so the port spells the macro with
   their two parameters (the data.rs INT_BUFSIZE_BOUND shape): `INT_BITS_STRLEN_BOUND (b)`
   is `(b * 146 + 484) / 485` and `INT_STRLEN_BOUND (t)` is
   `INT_BITS_STRLEN_BOUND (TYPE_WIDTH (t) - signed) + signed`.  */
const fn INT_STRLEN_BOUND (bits: usize, signed: usize) -> usize {
  ((bits - signed) * 146 + 484) / 485 + signed
}

/* C 57-60: the three file-scope counters and the cycle table.  */
/* Avoid actual stack overflow in print.  */
static mut print_depth: ptrdiff_t = 0;
/* Level of nesting inside outputting backquote in new style.  */
static mut new_backquote_output: ptrdiff_t = 0;
/* Detect most circularities to print finite output.  */
pub const PRINT_CIRCLE: usize = 200;
static mut being_printed: [Lisp_Object; PRINT_CIRCLE] =
  [std::ptr::null_mut (); PRINT_CIRCLE];
/* Last char printed to stdout by printchar.  */
static mut printchar_stdout_last: c_uint = 0;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct print_buffer {
  pub buffer: *mut c_char,      /* Allocated buffer.  */
  pub size: ptrdiff_t,          /* Size of allocated buffer.  */
  pub pos: ptrdiff_t,           /* Chars stored in buffer.  */
  pub pos_byte: ptrdiff_t,      /* Bytes stored in buffer.  */
}
/* Measured with clang -std=gnu23 on the frozen target: sizeof 32, align 8, buffer@0,
   size@8, pos@16, pos_byte@24.  */
const _: () = assert! (std::mem::size_of::<print_buffer> () == 32);
const _: () = assert! (std::mem::align_of::<print_buffer> () == 8);
const _: () = assert! (std::mem::offset_of! (print_buffer, buffer) == 0);
const _: () = assert! (std::mem::offset_of! (print_buffer, size) == 8);
const _: () = assert! (std::mem::offset_of! (print_buffer, pos) == 16);
const _: () = assert! (std::mem::offset_of! (print_buffer, pos_byte) == 24);

/* When printing into a buffer, first we put the text in this
   block, then insert it all at once.  */
static mut print_buffer: print_buffer = print_buffer {
  buffer: std::ptr::null_mut (),
  size: 0,
  pos: 0,
  pos_byte: 0,
};

/* Vprint_number_table is a table, that keeps objects that are going to
   be printed, to allow use of #n= and #n# to express sharing.
   For any given object, the table can give the following values:
     t    the object will be printed only once.
     -N   the object will be printed several times and will take number N.
     N    the object has been printed so we can refer to it as #N#.
   print_number_index holds the largest N already used.
   N has to be strictly larger than 0 since we need to distinguish -N.  */
static mut print_number_index: ptrdiff_t = 0;

/* GDB resets this to zero on W32 to disable OutputDebugString calls.  */
/* C 93: `extern bool print_output_debug_flag;`; C 94 adds EXTERNALLY_VISIBLE, which the
   port spells as the `#[unsafe(no_mangle)] pub` that makes the object visible at all.  */
#[unsafe(no_mangle)]
pub static mut print_output_debug_flag: bool = true;

/* Low level output routines for characters and strings.  */

/* This is used to free the print buffer; we don't simply record xfree
   since print_buffer can be reallocated during the printing.  */
unsafe extern "C" fn print_free_buffer ()
{
  xfree (print_buffer.buffer as *mut c_void);
  print_buffer.buffer = std::ptr::null_mut ();
}

/* This is used to restore the saved contents of print_buffer
   when there is a recursive call to print.  */
unsafe extern "C" fn print_unwind (saved_text: Lisp_Object)
{
  memcpy (print_buffer.buffer as *mut c_void, SDATA (saved_text) as *const c_void,
          SCHARS (saved_text) as usize);
}

/* Lisp functions to do output using a stream must start with a call to
   print_prepare, and end with calling print_finish.
   Use printchar to output one character, or call strout to output a
   block of characters.  */

/* State carried between print_prepare and print_finish.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct print_context {
  pub printcharfun: Lisp_Object,
  pub old_printcharfun: Lisp_Object,
  pub old_point: ptrdiff_t,
  pub start_point: ptrdiff_t,
  pub old_point_byte: ptrdiff_t,
  pub start_point_byte: ptrdiff_t,
  pub specpdl_count: specpdl_ref,
}
/* Measured with clang -std=gnu23 on the frozen target: sizeof 56, align 8, printcharfun@0,
   old_printcharfun@8, old_point@16, start_point@24, old_point_byte@32,
   start_point_byte@40, specpdl_count@48.  */
const _: () = assert! (std::mem::size_of::<print_context> () == 56);
const _: () = assert! (std::mem::align_of::<print_context> () == 8);
const _: () = assert! (std::mem::offset_of! (print_context, printcharfun) == 0);
const _: () = assert! (std::mem::offset_of! (print_context, old_printcharfun) == 8);
const _: () = assert! (std::mem::offset_of! (print_context, old_point) == 16);
const _: () = assert! (std::mem::offset_of! (print_context, start_point) == 24);
const _: () = assert! (std::mem::offset_of! (print_context, old_point_byte) == 32);
const _: () = assert! (std::mem::offset_of! (print_context, start_point_byte) == 40);
const _: () = assert! (std::mem::offset_of! (print_context, specpdl_count) == 48);

unsafe fn print_prepare (mut printcharfun: Lisp_Object) -> print_context
{
  let mut pc: print_context = print_context {
    printcharfun: std::ptr::null_mut (),
    old_printcharfun: printcharfun,
    old_point: -1,
    start_point: -1,
    old_point_byte: -1,
    start_point_byte: -1,
    specpdl_count: SPECPDL_INDEX (),
  };
  let multibyte: bool = ! NILP (BVAR! (current_buffer, enable_multibyte_characters_));
  record_unwind_current_buffer ();
  specbind(Qprint__unreadable_callback_buffer (), Fcurrent_buffer ());
  if NILP (printcharfun)
    { printcharfun = Qt (); }
  if BUFFERP (printcharfun)
    {
      if XBUFFER (printcharfun) != current_buffer
	{ Fset_buffer (printcharfun); }
      printcharfun = Qnil ();
    }
  if MARKERP (printcharfun)
    {
      if (*XMARKER (printcharfun)).buffer.is_null ()
	{ error (c"Marker does not point anywhere".as_ptr ()); }
      if (*XMARKER (printcharfun)).buffer != current_buffer as *mut c_void
	{ set_buffer_internal ((*XMARKER (printcharfun)).buffer as *mut buffer); }
      let marker_pos: ptrdiff_t = marker_position (printcharfun);
      if marker_pos < BEGV! () || marker_pos > ZV! ()
	{
	  signal_error (c"Marker is outside the accessible part of the buffer".as_ptr (),
			printcharfun);
	}
      pc.old_point = PT! ();
      pc.old_point_byte = PT_BYTE! ();
      SET_PT_BOTH (marker_pos, marker_byte_position (printcharfun));
      pc.start_point = PT! ();
      pc.start_point_byte = PT_BYTE! ();
      printcharfun = Qnil ();
    }
  if NILP (printcharfun)
    {
      if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	  && ! print_escape_multibyte
	{ specbind (Qprint_escape_multibyte (), Qt ()); }
      if ! NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	  && ! print_escape_nonascii
	{ specbind (Qprint_escape_nonascii (), Qt ()); }
      if ! print_buffer.buffer.is_null ()
	{
	  let string: Lisp_Object = make_string_from_bytes (print_buffer.buffer,
							    print_buffer.pos,
							    print_buffer.pos_byte);
	  record_unwind_protect (Some (print_unwind), string);
	}
      else
	{
	  let new_size: c_int = 1000;
	  print_buffer.buffer = xmalloc (new_size as usize) as *mut c_char;
	  print_buffer.size = new_size as ptrdiff_t;
	  record_unwind_protect_void (Some (print_free_buffer));
	}
      print_buffer.pos = 0;
      print_buffer.pos_byte = 0;
    }
  if EQ (printcharfun, Qt ()) && ! noninteractive
    { setup_echo_area_for_printing (multibyte); }
  pc.printcharfun = printcharfun;
  return pc;
}

unsafe fn print_finish (pc: *mut print_context)
{
  if NILP ((*pc).printcharfun)
    {
      if print_buffer.pos != print_buffer.pos_byte
	  && NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	{
	  USE_SAFE_ALLOCA! (sa_avail, sa_count);
	  let temp: *mut u8 = SAFE_ALLOCA! (sa_avail, print_buffer.pos + 1) as *mut u8;
	  copy_text (print_buffer.buffer as *mut u8, temp,
		     print_buffer.pos_byte, true, false);
	  insert_1_both (temp as *mut c_char, print_buffer.pos,
			 print_buffer.pos, false, true, false);
	  SAFE_FREE! (sa_count);
	}
      else
	{ insert_1_both (print_buffer.buffer, print_buffer.pos,
			 print_buffer.pos_byte, false, true, false); }
      signal_after_change (PT! () - print_buffer.pos, 0, print_buffer.pos);
    }
  if MARKERP ((*pc).old_printcharfun)
    { set_marker_both ((*pc).old_printcharfun, Qnil (), PT! (), PT_BYTE! ()); }
  if (*pc).old_point >= 0
    { SET_PT_BOTH ((*pc).old_point
		    + (if (*pc).old_point >= (*pc).start_point
			 { PT! () - (*pc).start_point } else { 0 }),
		    (*pc).old_point_byte
		    + (if (*pc).old_point_byte >= (*pc).start_point_byte
			 { PT_BYTE! () - (*pc).start_point_byte } else { 0 })); }
  unbind_to ((*pc).specpdl_count, Qnil ());
}

/* Print character CH to the stdio stream STREAM.  */

unsafe fn printchar_to_stream (mut ch: c_uint, stream: *mut FILE)
{
  let mut dv: Lisp_Object = Qnil ();   /* C: `Lisp_Object dv UNINIT;` (PORTING.md 1.5) */
  let mut i: ptrdiff_t = 0;
  let mut n: ptrdiff_t = 1;
  let mut coding_system: Lisp_Object = Vlocale_coding_system;
  let mut encode_p: bool = false;

  if ! NILP (Vcoding_system_for_write)
    { coding_system = Vcoding_system_for_write; }
  if ! NILP (coding_system)
    { encode_p = true; }

  let mut goto_next_char: bool = false;
  if CHAR_VALID_P (ch as intmax_t) && DISP_TABLE_P (Vstandard_display_table)
    {
      dv = DISP_CHAR_VECTOR (XCHAR_TABLE (Vstandard_display_table), ch as c_int);
      if VECTORP (dv)
	{
	  n = ASIZE (dv);
	  goto_next_char = true;
	}
    }

  'next_char: loop {
      if ! goto_next_char
	{
	  if ASCII_CHAR_P (ch as isize)
	    {
	      putc (ch as c_int, stream);
	      /* C 46-48's `#ifdef WINDOWSNT` arm: the debugger output below is carried
		 text (feature `windowsnt`).  */
	      #[cfg (feature = "windowsnt")]
	      {
		/* Send the output to a debugger (nothing happens if there
		   isn't one).  */
		if print_output_debug_flag && stream == stderr
		  { OutputDebugString ((&[ch as c_char, 0]).as_ptr ()); }
	      }
	    }
	  else
	    {
	      let mut mbstr: [u8; MAX_MULTIBYTE_LENGTH as usize] =
		[0; MAX_MULTIBYTE_LENGTH as usize];
	      let len: c_int = CHAR_STRING (ch as c_int, mbstr.as_mut_ptr ());
	      let mut encoded_ch: Lisp_Object =
		make_multibyte_string (mbstr.as_ptr () as *const c_char, 1, len as ptrdiff_t);

	      if encode_p
		{ encoded_ch = code_convert_string_norecord (encoded_ch,
							     coding_system, true); }
	      fwrite (SSDATA (encoded_ch) as *const c_void, 1, SBYTES (encoded_ch) as usize,
		      stream);
	      /* C 282-285's `#ifdef WINDOWSNT` arm (feature `windowsnt`).  */
	      #[cfg (feature = "windowsnt")]
	      {
		if print_output_debug_flag && stream == stderr
		  { OutputDebugString (SSDATA (encoded_ch)); }
	      }
	    }

	  i += 1;
	}
      goto_next_char = false;

      /* next_char: */
      while i < n
	{
	  if CHARACTERP (AREF (dv, i)) { break; }
	  i += 1;
	}
      if ! (i < n)
	{ break 'next_char; }
      ch = XFIXNAT (AREF (dv, i)) as c_uint;
  }
}

/* Print character CH using method FUN.  FUN nil means print to
   print_buffer.  FUN t means print to echo area or stdout if
   non-interactive.  If FUN is neither nil nor t, call FUN with CH as
   argument.  */

unsafe fn printchar (ch: c_uint, fun: Lisp_Object)
{
  if ! NILP (fun) && ! EQ (fun, Qt ())
    { calln! (fun, make_fixnum (ch as isize)); }
  else
    {
      let mut str: [u8; MAX_MULTIBYTE_LENGTH as usize] = [0; MAX_MULTIBYTE_LENGTH as usize];
      let len: c_int = CHAR_STRING (ch as c_int, str.as_mut_ptr ());

      maybe_quit ();

      if NILP (fun)
	{
	  let incr: ptrdiff_t = len as ptrdiff_t - (print_buffer.size - print_buffer.pos_byte);
	  if incr > 0
	    { print_buffer.buffer = xpalloc (print_buffer.buffer as *mut c_void,
					     &raw mut print_buffer.size,
					     incr, -1, 1) as *mut c_char; }
	  memcpy (print_buffer.buffer.add (print_buffer.pos_byte as usize) as *mut c_void,
		  str.as_ptr () as *const c_void, len as usize);
	  print_buffer.pos += 1;
	  print_buffer.pos_byte += len as ptrdiff_t;
	}
      else if noninteractive
	{
	  printchar_stdout_last = ch;
	  if DISP_TABLE_P (Vstandard_display_table)
	    { printchar_to_stream (ch, stdout); }
	  else
	    { fwrite (str.as_ptr () as *const c_void, 1, len as usize, stdout); }
	  noninteractive_need_newline = true;
	}
      else
	{
	  let multibyte_p: bool
	    = ! NILP (BVAR! (current_buffer, enable_multibyte_characters_));

	  setup_echo_area_for_printing (multibyte_p);
	  insert_char (ch as c_int);
	  message_dolog (str.as_ptr () as *const c_char, len as ptrdiff_t, 0, multibyte_p);
	}
    }
}

/* Output an octal escape for C.  If C is less than '\100' consult the
   following character (if any) to see whether to use three octal
   digits to avoid misinterpretation of the next character.  The next
   character after C will be taken from DATA, starting at byte
   location I, if I is less than SIZE.  Use PRINTCHARFUN to output
   each character.  */

unsafe fn octalout (c: u8, data: *mut u8, i: ptrdiff_t, size: ptrdiff_t,
		    printcharfun: Lisp_Object)
{
  let mut digits: c_int = (if c as c_int > b'\x3f' as c_int
			      || (i < size && b'0' as u8 <= *data.offset (i)
				  && *data.offset (i) <= b'7' as u8)
			    { 3 }
			    else if c as c_int > b'\x07' as c_int { 2 } else { 1 });
  printchar (b'\\' as c_uint, printcharfun);
  loop
    {
      digits -= 1;
      printchar ((b'0' as c_int + ((c as c_int >> (3 * digits)) & 7)) as c_uint,
		 printcharfun);
      if digits == 0 { break; }
    }
}

/* Output SIZE characters, SIZE_BYTE bytes from string PTR using
   method PRINTCHARFUN.  PRINTCHARFUN nil means output to
   print_buffer.  PRINTCHARFUN t means output to the echo area or to
   stdout if non-interactive.  If neither nil nor t, call Lisp
   function PRINTCHARFUN for each character printed.  MULTIBYTE
   non-zero means PTR contains multibyte characters.

   In the case where PRINTCHARFUN is nil, it is safe for PTR to point
   to data in a Lisp string.  Otherwise that is not safe.  */

unsafe fn strout (ptr: *const c_char, size: ptrdiff_t, size_byte: ptrdiff_t,
		  printcharfun: Lisp_Object)
{
  if NILP (printcharfun)
    {
      let incr: ptrdiff_t = size_byte - (print_buffer.size - print_buffer.pos_byte);
      if incr > 0
	{ print_buffer.buffer = xpalloc (print_buffer.buffer as *mut c_void,
					 &raw mut print_buffer.size,
					 incr, -1, 1) as *mut c_char; }
      memcpy (print_buffer.buffer.add (print_buffer.pos_byte as usize) as *mut c_void,
	      ptr as *const c_void, size_byte as usize);
      print_buffer.pos += size;
      print_buffer.pos_byte += size_byte;
    }
  else if noninteractive && EQ (printcharfun, Qt ())
    {
      if DISP_TABLE_P (Vstandard_display_table)
	{
	  let mut len: c_int = 0;
	  let mut i: ptrdiff_t = 0;
	  while i < size_byte
	    {
	      let ch: c_int = string_char_and_length (ptr.add (i as usize) as *const u8,
						      &raw mut len);
	      printchar_to_stream (ch as c_uint, stdout);
	      i += len as ptrdiff_t;
	    }
	}
      else
	{ fwrite (ptr as *const c_void, 1, size_byte as usize, stdout); }

      noninteractive_need_newline = true;
    }
  else if EQ (printcharfun, Qt ())
    {
      /* Output to echo area.  We're trying to avoid a little overhead
	 here, that's the reason we don't call printchar to do the
	 job.  */
      let mut i: c_int;
      let multibyte_p: bool
	= ! NILP (BVAR! (current_buffer, enable_multibyte_characters_));

      setup_echo_area_for_printing (multibyte_p);
      message_dolog (ptr, size_byte, 0, multibyte_p);

      if size == size_byte
	{
	  i = 0;
	  while i < size as c_int
	    {
	      insert_char (*ptr.add (i as usize) as c_int);
	      i += 1;
	    }
	}
      else
	{
	  let mut len: c_int = 0;
	  i = 0;
	  while i < size_byte as c_int
	    {
	      let ch: c_int = string_char_and_length (ptr.add (i as usize) as *const u8,
						      &raw mut len);
	      insert_char (ch);
	      i += len;
	    }
	}
    }
  else
    {
      /* PRINTCHARFUN is a Lisp function.  */
      let mut i: ptrdiff_t = 0;

      if size == size_byte
	{
	  while i < size_byte
	    {
	      let ch: c_int = *ptr.add (i as usize) as c_int;
	      i += 1;
	      printchar (ch as c_uint, printcharfun);
	    }
	}
      else
	{
	  while i < size_byte
	    {
	      /* Here, we must convert each multi-byte form to the
		 corresponding character code before handing it to
		 PRINTCHAR.  */
	      let mut len: c_int = 0;
	      let ch: c_int = (string_char_and_length
			       (ptr.add (i as usize) as *const u8, &raw mut len));
	      printchar (ch as c_uint, printcharfun);
	      i += len as ptrdiff_t;
	    }
	}
    }
}

/* Print the contents of a string STRING using PRINTCHARFUN.
   It isn't safe to use strout in many cases,
   because printing one char can relocate.  */

unsafe fn print_string_1 (mut string: Lisp_Object, printcharfun: Lisp_Object,
			  escape_nonascii: bool)
{
  if EQ (printcharfun, Qt ()) || NILP (printcharfun)
    {
      let chars: ptrdiff_t;

      if escape_nonascii
	{ string = string_escape_byte8 (string); }

      if STRING_MULTIBYTE (string)
	{ chars = SCHARS (string); }
      else if ! escape_nonascii
	       && (if EQ (printcharfun, Qt ())
		     { ! NILP (BVAR! (&raw mut buffer_defaults, enable_multibyte_characters_)) }
		   else
		     { ! NILP (BVAR! (current_buffer, enable_multibyte_characters_)) })
	{
	  /* If unibyte string STRING contains 8-bit codes, we must
	     convert STRING to a multibyte string containing the same
	     character codes.  */
	  let newstr: Lisp_Object;
	  let bytes: ptrdiff_t;

	  chars = SBYTES (string);
	  bytes = count_size_as_multibyte (SDATA (string), chars);
	  if chars < bytes
	    {
	      newstr = make_uninit_multibyte_string (chars, bytes);
	      str_to_multibyte (SDATA (newstr), SDATA (string), chars);
	      string = newstr;
	    }
	}
      else
	{ chars = SBYTES (string); }

      if EQ (printcharfun, Qt ())
	{
	  /* Output to echo area.  */
	  let nbytes: ptrdiff_t = SBYTES (string);

	  /* Copy the string contents so that relocation of STRING by
	     GC does not cause trouble.  */
	  USE_SAFE_ALLOCA! (sa_avail, sa_count);
	  let buffer: *mut c_char = SAFE_ALLOCA! (sa_avail, nbytes) as *mut c_char;
	  memcpy (buffer as *mut c_void, SDATA (string) as *const c_void, nbytes as usize);

	  strout (buffer, chars, nbytes, printcharfun);

	  SAFE_FREE! (sa_count);
	}
      else
	/* No need to copy, since output to print_buffer can't GC.  */
	{ strout (SSDATA (string), chars, SBYTES (string), printcharfun); }
    }
  else
    {
      /* Otherwise, string may be relocated by printing one char.
	 So re-fetch the string address for each character.  */
      let mut i: ptrdiff_t;
      let size: ptrdiff_t = SCHARS (string);
      let size_byte: ptrdiff_t = SBYTES (string);
      if size == size_byte
	{
	  i = 0;
	  while i < size
	    {
	      printchar (SREF (string, i as usize) as c_uint, printcharfun);
	      i += 1;
	    }
	}
      else
	{
	  i = 0;
	  while i < size_byte
	    {
	      /* Here, we must convert each multi-byte form to the
		 corresponding character code before handing it to PRINTCHAR.  */
	      let mut len: c_int = 0;
	      let ch: c_int = string_char_and_length (SDATA (string).add (i as usize),
						      &raw mut len);
	      printchar (ch as c_uint, printcharfun);
	      i += len as ptrdiff_t;
	    }
	}
    }
}

unsafe fn print_string (string: Lisp_Object, printcharfun: Lisp_Object)
{
  print_string_1 (string, printcharfun, print_escape_nonascii);
}

DEFUN! ("write-char", Fwrite_char, Swrite_char, 1, 2, 0,
       doc: "Output character CHARACTER to stream PRINTCHARFUN.\nPRINTCHARFUN defaults to the value of `standard-output' (which see).",
       (character: Lisp_Object, printcharfun: Lisp_Object),
       {
  let mut printcharfun = printcharfun;   /* C 560 rebinds the parameter.  */
  if NILP (printcharfun)
    { printcharfun = Vstandard_output; }
  CHECK_FIXNUM (character);
  let mut pc: print_context = print_prepare (printcharfun);
  printchar (XFIXNUM (character) as c_uint, pc.printcharfun);
  print_finish (&raw mut pc);
  return character;
});

/* Print the contents of a unibyte C string STRING using PRINTCHARFUN.
   The caller should arrange to put this inside print_prepare and print_finish.
   Do not use this on the contents of a Lisp string.  */

unsafe fn print_c_string (string: *const c_char, printcharfun: Lisp_Object)
{
  let len: ptrdiff_t = strlen (string) as ptrdiff_t;
  strout (string, len, len, printcharfun);
}

/* Print unibyte C string at DATA on a specified stream PRINTCHARFUN.
   Do not use this on the contents of a Lisp string.  */

unsafe fn write_string (data: *const c_char, printcharfun: Lisp_Object)
{
  let mut pc: print_context = print_prepare (printcharfun);
  print_c_string (data, pc.printcharfun);
  print_finish (&raw mut pc);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn temp_output_buffer_setup (bufname: *const c_char)
{
  let count: specpdl_ref = SPECPDL_INDEX ();
  let old: *mut buffer = current_buffer;
  let buf: Lisp_Object;

  record_unwind_current_buffer ();

  Fset_buffer (Fget_buffer_create (build_string (bufname), Qnil ()));

  Fkill_all_local_variables (Qnil ());
  delete_all_overlays (current_buffer);
  bset_directory (current_buffer, BVAR! (old, directory_));
  bset_read_only (current_buffer, Qnil ());
  bset_filename (current_buffer, Qnil ());
  bset_undo_list (current_buffer, Qt ());
  eassert! ((*current_buffer).overlays.is_null ());
  bset_enable_multibyte_characters
    (current_buffer, BVAR! (&raw mut buffer_defaults, enable_multibyte_characters_));
  specbind (Qinhibit_read_only (), Qt ());
  specbind (Qinhibit_modification_hooks (), Qt ());
  Ferase_buffer ();
  XSETBUFFER! (buf, current_buffer);

  run_hook (Qtemp_buffer_setup_hook ());

  unbind_to (count, Qnil ());

  specbind (Qstandard_output (), buf);
}

/* C 622-625's three forward declarations (`print`, `print_preprocess`,
   `print_object`) are dropped (PORTING.md 0.1-12); the definitions keep C's order.  */

DEFUN! ("terpri", Fterpri, Sterpri, 0, 2, 0,
       doc: "Output a newline to stream PRINTCHARFUN.\nIf ENSURE is non-nil only output a newline if not already at the\nbeginning of a line.  Value is non-nil if a newline is printed.\nIf PRINTCHARFUN is omitted or nil, the value of `standard-output' is used.",
       (printcharfun: Lisp_Object, ensure: Lisp_Object),
       {
  let val: Lisp_Object;

  let mut printcharfun = printcharfun;   /* C 634 rebinds the parameter.  */
  if NILP (printcharfun)
    { printcharfun = Vstandard_output; }
  let mut pc: print_context = print_prepare (printcharfun);

  if NILP (ensure)
    { val = Qt (); }
  /* Difficult to check if at line beginning so abort.  */
  else if FUNCTIONP (pc.printcharfun)
    { signal_error (c"Unsupported function argument".as_ptr (), pc.printcharfun); }
  else if noninteractive && ! NILP (pc.printcharfun)
    { val = if printchar_stdout_last == 10 { Qnil () } else { Qt () }; }
  else
    { val = if NILP (Fbolp ()) { Qt () } else { Qnil () }; }

  if ! NILP (val)
    { printchar (b'\n' as c_uint, pc.printcharfun); }
  print_finish (&raw mut pc);
  return val;
});

static mut Vprint_variable_mapping: Lisp_Object = std::ptr::null_mut ();

unsafe fn print_bind_all_defaults ()
{
  let mut vars: Lisp_Object = Vprint_variable_mapping;
  while ! NILP (vars)
    {
      let elem: Lisp_Object = XCDR (XCAR (vars));
      specbind (XCAR (elem), XCAR (XCDR (elem)));
      vars = XCDR (vars);
    }
}

unsafe fn print_create_variable_mapping ()
{
  let total: [Lisp_Object; 17] = [
    list3 (intern (c"length".as_ptr ()), intern (c"print-length".as_ptr ()), Qnil ()),
    list3 (intern (c"level".as_ptr ()), intern (c"print-level".as_ptr ()), Qnil ()),
    list3 (intern (c"circle".as_ptr ()), intern (c"print-circle".as_ptr ()), Qnil ()),
    list3 (intern (c"quoted".as_ptr ()), intern (c"print-quoted".as_ptr ()), Qt ()),
    list3 (intern (c"escape-newlines".as_ptr ()), intern (c"print-escape-newlines".as_ptr ()),
	   Qnil ()),
    list3 (intern (c"escape-control-characters".as_ptr ()),
	   intern (c"print-escape-control-characters".as_ptr ()), Qnil ()),
    list3 (intern (c"escape-nonascii".as_ptr ()), intern (c"print-escape-nonascii".as_ptr ()),
	   Qnil ()),
    list3 (intern (c"escape-multibyte".as_ptr ()),
	   intern (c"print-escape-multibyte".as_ptr ()), Qnil ()),
    list3 (intern (c"charset-text-property".as_ptr ()),
	   intern (c"print-charset-text-property".as_ptr ()), Qnil ()),
    list3 (intern (c"unreadable-function".as_ptr ()),
	   intern (c"print-unreadable-function".as_ptr ()), Qnil ()),
    list3 (intern (c"unreadeable-function".as_ptr ()),
	   intern (c"print-unreadable-function".as_ptr ()), Qnil ()),
    list3 (intern (c"gensym".as_ptr ()), intern (c"print-gensym".as_ptr ()), Qnil ()),
    list3 (intern (c"continuous-numbering".as_ptr ()),
	   intern (c"print-continuous-numbering".as_ptr ()), Qnil ()),
    list3 (intern (c"number-table".as_ptr ()), intern (c"print-number-table".as_ptr ()),
	   Qnil ()),
    list3 (intern (c"float-format".as_ptr ()), intern (c"float-output-format".as_ptr ()),
	   Qnil ()),
    list3 (intern (c"integers-as-characters".as_ptr ()),
	   intern (c"print-integers-as-characters".as_ptr ()), Qnil ()),
    list3 (intern (c"symbols-bare".as_ptr ()), intern (c"print-symbols-bare".as_ptr ()),
	   Qnil ()),
  ];

  Vprint_variable_mapping = CALLMANY! (Flist, total);
}

unsafe fn print_bind_overrides (mut overrides: Lisp_Object)
{
  if NILP (Vprint_variable_mapping)
    { print_create_variable_mapping (); }

  if EQ (overrides, Qt ())
    { print_bind_all_defaults (); }
  else if ! CONSP (overrides)
    { wrong_type_argument (Qconsp (), overrides); }
  else
    {
      while ! NILP (overrides)
	{
	  let setting: Lisp_Object = XCAR (overrides);
	  if EQ (setting, Qt ())
	    { print_bind_all_defaults (); }
	  else if ! CONSP (setting)
	    { wrong_type_argument (Qconsp (), setting); }
	  else
	    {
	      let key: Lisp_Object = XCAR (setting);
	      let value: Lisp_Object = XCDR (setting);
	      let map: Lisp_Object = Fassq (key, Vprint_variable_mapping);
	      if NILP (map)
		{ wrong_type_argument (Qsymbolp (), map); }
	      specbind (XCAR (XCDR (map)), value);
	    }

	  if ! NILP (XCDR (overrides)) && ! CONSP (XCDR (overrides))
	    { wrong_type_argument (Qconsp (), overrides); }
	  overrides = XCDR (overrides);
	}
    }
}

DEFUN! ("prin1", Fprin1, Sprin1, 1, 3, 0,
       doc: "Output the printed representation of OBJECT, any Lisp object.\nQuoting characters are printed when needed to make output that `read'\ncan handle, whenever this is possible.  For complex objects, the behavior\nis controlled by `print-level' and `print-length', which see.\n\nOBJECT is any of the Lisp data types: a number, a string, a symbol,\na list, a buffer, a window, a frame, etc.\n\nA printed representation of an object is text which describes that object.\n\nOptional argument PRINTCHARFUN is the output stream, which can be one\nof these:\n\n   - a buffer, in which case output is inserted into that buffer at point;\n   - a marker, in which case output is inserted at marker's position;\n   - a function, in which case that function is called once for each\n     character of OBJECT's printed representation;\n   - a symbol, in which case that symbol's function definition is called; or\n   - t, in which case the output is displayed in the echo area.\n\nIf PRINTCHARFUN is omitted, the value of `standard-output' (which see)\nis used instead.\n\nOptional argument OVERRIDES should be a list of settings for print-related\nvariables.  An element in this list can be the symbol t, which means \"reset\nall the values to their defaults\".  Otherwise, an element should be a pair,\nwhere the `car' or the pair is the setting symbol, and the `cdr' is the\nvalue of the setting to use for this `prin1' call.\n\nFor instance:\n\n  (prin1 object nil \\='((length . 100) (circle . t))).\n\nSee Info node `(elisp)Output Overrides' for a list of possible values.\n\nAs a special case, OVERRIDES can also simply be the symbol t, which\nmeans \"use default values for all the print-related settings\".",
       (object: Lisp_Object, printcharfun: Lisp_Object, overrides: Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();

  let mut printcharfun = printcharfun;   /* C 781 rebinds the parameter.  */
  if NILP (printcharfun)
    { printcharfun = Vstandard_output; }
  if ! NILP (overrides)
    { print_bind_overrides (overrides); }

  let mut pc: print_context = print_prepare (printcharfun);
  print (object, pc.printcharfun, true);
  print_finish (&raw mut pc);

  return unbind_to (count, object);
});

/* A buffer which is used to hold output being built by prin1-to-string.  */
#[unsafe(no_mangle)]
pub static mut Vprin1_to_string_buffer: Lisp_Object = lisp_h_Qnil ();

DEFUN! ("prin1-to-string", Fprin1_to_string, Sprin1_to_string, 1, 3, 0,
       doc: "Return a string containing the printed representation of OBJECT.\nOBJECT can be any Lisp object.  This function outputs quoting characters\nwhen necessary to make output that `read' can handle, whenever possible,\nunless the optional second argument NOESCAPE is non-nil.  For complex objects,\nthe behavior is controlled by `print-level' and `print-length', which see.\n\nOBJECT is any of the Lisp data types: a number, a string, a symbol,\na list, a buffer, a window, a frame, etc.\n\nSee `prin1' for the meaning of OVERRIDES.\n\nA printed representation of an object is text which describes that object.",
       (object: Lisp_Object, noescape: Lisp_Object, overrides: Lisp_Object),
       {
  let count: specpdl_ref = SPECPDL_INDEX ();

  specbind (Qinhibit_modification_hooks (), Qt ());

  if ! NILP (overrides)
    { print_bind_overrides (overrides); }

  /* Save and restore this: we are altering a buffer
     but we don't want to deactivate the mark just for that.
     No need for specbind, since errors deactivate the mark.  */
  let save_deactivate_mark: Lisp_Object = Vdeactivate_mark;

  let mut pc: print_context = print_prepare (Vprin1_to_string_buffer);
  let mut object = object;               /* C 831 reassigns the parameter.  */
  print (object, pc.printcharfun, NILP (noescape));
  /* Make Vprin1_to_string_buffer be the default buffer after print_finish */
  print_finish (&raw mut pc);

  let previous: *mut buffer = current_buffer;
  set_buffer_internal (XBUFFER (Vprin1_to_string_buffer));
  object = Fbuffer_string ();
  if SBYTES (object) == SCHARS (object)
    { STRING_SET_UNIBYTE (&raw mut object); }

  /* Note that this won't make prepare_to_modify_buffer call
     ask-user-about-supersession-threat because this buffer
     does not visit a file.  */
  Ferase_buffer ();
  set_buffer_internal (previous);

  Vdeactivate_mark = save_deactivate_mark;

  return unbind_to (count, object);
});

DEFUN! ("princ", Fprinc, Sprinc, 1, 2, 0,
       doc: "Output the printed representation of OBJECT, any Lisp object.\nNo quoting characters are used; no delimiters are printed around\nthe contents of strings.\n\nOBJECT is any of the Lisp data types: a number, a string, a symbol,\na list, a buffer, a window, a frame, etc.\n\nA printed representation of an object is text which describes that object.\n\nOptional argument PRINTCHARFUN is the output stream, which can be one\nof these:\n\n   - a buffer, in which case output is inserted into that buffer at point;\n   - a marker, in which case output is inserted at marker's position;\n   - a function, in which case that function is called once for each\n     character of OBJECT's printed representation;\n   - a symbol, in which case that symbol's function definition is called; or\n   - t, in which case the output is displayed in the echo area.\n\nIf PRINTCHARFUN is omitted, the value of `standard-output' (which see)\nis used instead.",
       (object: Lisp_Object, printcharfun: Lisp_Object),
       {
  let mut printcharfun = printcharfun;   /* C 857 rebinds the parameter.  */
  if NILP (printcharfun)
    { printcharfun = Vstandard_output; }
  let mut pc: print_context = print_prepare (printcharfun);
  if STRINGP (object)
      && string_intervals (object).is_null ()
      && NILP (Vprint_continuous_numbering)
    /* fast path for plain strings */
    { print_string (object, pc.printcharfun); }
  else
    { print (object, pc.printcharfun, false); }
  print_finish (&raw mut pc);
  return object;
});

DEFUN! ("print", Fprint, Sprint, 1, 2, 0,
       doc: "Output the printed representation of OBJECT, with newlines around it.\nQuoting characters are printed when needed to make output that `read'\ncan handle, whenever this is possible.  For complex objects, the behavior\nis controlled by `print-level' and `print-length', which see.\n\nOBJECT is any of the Lisp data types: a number, a string, a symbol,\na list, a buffer, a window, a frame, etc.\n\nA printed representation of an object is text which describes that object.\n\nOptional argument PRINTCHARFUN is the output stream, which can be one\nof these:\n\n   - a buffer, in which case output is inserted into that buffer at point;\n   - a marker, in which case output is inserted at marker's position;\n   - a function, in which case that function is called once for each\n     character of OBJECT's printed representation;\n   - a symbol, in which case that symbol's function definition is called; or\n   - t, in which case the output is displayed in the echo area.\n\nIf PRINTCHARFUN is omitted, the value of `standard-output' (which see)\nis used instead.",
       (object: Lisp_Object, printcharfun: Lisp_Object),
       {
  let mut printcharfun = printcharfun;   /* C 897 rebinds the parameter.  */
  if NILP (printcharfun)
    { printcharfun = Vstandard_output; }
  let mut pc: print_context = print_prepare (printcharfun);
  printchar (b'\n' as c_uint, pc.printcharfun);
  print (object, pc.printcharfun, true);
  printchar (b'\n' as c_uint, pc.printcharfun);
  print_finish (&raw mut pc);
  return object;
});

DEFUN! ("flush-standard-output", Fflush_standard_output, Sflush_standard_output,
       0, 0, 0,
       doc: "Flush standard-output.\nThis can be useful after using `princ' and the like in scripts.",
       (),
       {
  fflush (stdout);
  return Qnil ();
});

DEFUN! ("external-debugging-output", Fexternal_debugging_output, Sexternal_debugging_output, 1, 1, 0,
       doc: "Write CHARACTER to stderr.\nYou can call `print' while debugging emacs, and pass it this function\nto make it write to the debugging output.",
       (character: Lisp_Object),
       {
  CHECK_FIXNUM (character);
  printchar_to_stream (XFIXNUM (character) as c_uint, stderr);
  return character;
});

/* This function is never called.  Its purpose is to prevent
   print_output_debug_flag from being optimized away.  */

/* C 943: `extern void debug_output_compilation_hack (bool) EXTERNALLY_VISIBLE;` */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn debug_output_compilation_hack (x: bool)
{
  print_output_debug_flag = x;
}

DEFUN! ("redirect-debugging-output", Fredirect_debugging_output, Sredirect_debugging_output,
       1, 2,
       "FDebug output file: \nP",
       doc: "Redirect debugging output (stderr stream) to file FILE.\nIf FILE is nil, reset target to the initial stderr stream.\nOptional arg APPEND non-nil (interactively, with prefix arg) means\nappend to existing target file.",
       (file: Lisp_Object, append: Lisp_Object),
       {
  /* If equal to STDERR_FILENO, stderr has not been duplicated and is OK as-is.
     Otherwise, this is a close-on-exec duplicate of the original stderr. */
  static mut stderr_dup: c_int = STDERR_FILENO;
  let mut fd: c_int = stderr_dup;

  let mut file = file;                   /* C 958 rebinds the parameter.  */
  if ! NILP (file)
    {
      file = Fexpand_file_name (file, Qnil ());

      if stderr_dup == STDERR_FILENO
	{
	  let n: c_int = fcntl (STDERR_FILENO, F_DUPFD_CLOEXEC, STDERR_FILENO + 1);
	  if n < 0
	    { report_file_error (c"dup".as_ptr (), file); }
	  stderr_dup = n;
	}

      fd = emacs_open (SSDATA (ENCODE_FILE (file)),
		       (O_WRONLY | O_CREAT
			| (if ! NILP (append) { O_APPEND } else { O_TRUNC })),
		       0o666);
      if fd < 0
	{ report_file_error (c"Cannot open debugging output stream".as_ptr (), file); }
    }

  fflush (stderr);
  if dup2 (fd, STDERR_FILENO) < 0
    { report_file_error (c"dup2".as_ptr (), file); }
  if fd != stderr_dup
    { emacs_close (fd); }
  return Qnil ();
});

/* This is the interface for debugging printing.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn debug_print (arg: Lisp_Object)
{
  Fprin1 (arg, Qexternal_debugging_output (), Qnil ());
  fputs (c"\r\n".as_ptr (), stderr);
}

/* C 1002: `void safe_debug_print (Lisp_Object) EXTERNALLY_VISIBLE;` */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn safe_debug_print (arg: Lisp_Object)
{
  let valid: c_int = valid_lisp_object_p (arg);

  if valid > 0
    { debug_print (arg); }
  else
    {
      let n: EMACS_UINT = XLI (arg) as EMACS_UINT;
      fprintf (stderr, c"#<%s_LISP_OBJECT 0x%08jx>\r\n".as_ptr (),
	       if valid == 0 { c"INVALID".as_ptr () } else { c"SOME".as_ptr () },
	       n);
    }
}

/* This function formats the given object and returns the result as a
   string. Use this in contexts where you can inspect strings, but
   where stderr output won't work --- e.g. while replaying rr
   recordings.  */
/* C 1023: `const char * debug_format (const char *, Lisp_Object) EXTERNALLY_VISIBLE;` */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn debug_format (fmt: *const c_char, arg: Lisp_Object) -> *const c_char
{
  return SSDATA (CALLN! (Fformat, build_string (fmt), arg));
}

/* Erase the Vprin1_to_string_buffer, potentially switching to it.  */
unsafe extern "C" fn erase_prin1_to_string_buffer ()
{
  set_buffer_internal (XBUFFER (Vprin1_to_string_buffer));
  Ferase_buffer ();
}

DEFUN! ("error-message-string", Ferror_message_string, Serror_message_string,
       1, 1, 0,
       doc: "Convert an error value (ERROR-SYMBOL . DATA) to an error message.\nSee Info anchor `(elisp)Definition of signal' for some details on how this\nerror message is constructed.",
       (obj: Lisp_Object),
       {
  /* If OBJ is (error STRING), just return STRING.
     That is not only faster, it also avoids the need to allocate
     space here when the error is due to memory full.  */
  if CONSP (obj) && EQ (XCAR (obj), Qerror ())
      && CONSP (XCDR (obj))
      && STRINGP (XCAR (XCDR (obj)))
      && NILP (XCDR (XCDR (obj)))
    { return XCAR (XCDR (obj)); }

  /* print_error_message can throw after producing some output, in which
     case we need to ensure the buffer is cleared again (bug#78842).  */
  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_current_buffer ();
  record_unwind_protect_void (Some (erase_prin1_to_string_buffer));
  print_error_message (obj, Vprin1_to_string_buffer, std::ptr::null (), Qnil ());

  set_buffer_internal (XBUFFER (Vprin1_to_string_buffer));
  return unbind_to (count, Fbuffer_string ());
});

/* Print an error message for the error DATA onto Lisp output stream
   STREAM (suitable for the print functions).
   CONTEXT is a C string describing the context of the error.
   CALLER is the Lisp function inside which the error was signaled.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn print_error_message (mut data: Lisp_Object, stream: Lisp_Object,
					      context: *const c_char, caller: Lisp_Object)
{
  let mut errname: Lisp_Object = Qnil ();
  let mut errmsg: Lisp_Object = Qnil ();
  let mut file_error: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object = Qnil ();

  if ! context.is_null ()
    { write_string (context, stream); }

  /* If we know from where the error was signaled, show it in
   *Messages*.  */
  if ! NILP (caller) && SYMBOLP (caller)
    {
      let cname: Lisp_Object = SYMBOL_NAME (caller);
      let cnamelen: ptrdiff_t = SBYTES (cname);
      USE_SAFE_ALLOCA! (sa_avail, sa_count);
      let name: *mut c_char = SAFE_ALLOCA! (sa_avail, cnamelen) as *mut c_char;
      memcpy (name as *mut c_void, SDATA (cname) as *const c_void, cnamelen as usize);
      message_dolog (name, cnamelen, 0, STRING_MULTIBYTE (cname));
      message_dolog (c": ".as_ptr (), 2, 0, false);
      SAFE_FREE! (sa_count);
    }

  errname = Fcar (data);

  if EQ (errname, Qerror ())
    {
      data = Fcdr (data);
      if ! CONSP (data)
	{ data = Qnil (); }
      errmsg = Fcar (data);
      file_error = Qnil ();
    }
  else
    {
      let error_conditions: Lisp_Object = Fget (errname, Qerror_conditions ());
      errmsg = Fget (errname, Qerror_message ());
      /* During loadup 'substitute-command-keys' might not be available.  */
      if ! NILP (Ffboundp (Qsubstitute_command_keys ()))
	{
	  /* `substitute-command-keys' may bug out, which would lead
	     to infinite recursion when we're called from
	     skip_debugger, so ignore errors.  */
	  let subs: Lisp_Object = safe_calln! (Qsubstitute_command_keys (), errmsg);
	  if ! NILP (subs)
	    { errmsg = subs; }
	}

      file_error = Fmemq (Qfile_error (), error_conditions);
    }

  /* Print an error message including the data items.  */

  tail = Fcdr_safe (data);

  /* For a user-error displayed in the echo area, use message3 rather
     than Fprinc in order to preserve fontification.
     In particular, there might be hints to the user about key sequences
     they could type to do what they seemed to want.  */
  if EQ (errname, Quser_error ()) && EQ (stream, Qt ())
      /* These should always be true for a user-error, but check, lest
	 we throw any information away.  */
      && ! NILP (XCAR (tail)) && NILP (XCDR (tail))
    {
      /* Prevent message3 from outputting a newline after "user-error:".  */
      reset_message_log_need_newline ();
      message3 (XCAR (tail));
      return;
    }

  /* For file-error, make error message by concatenating
     all the data items.  They are all strings.  */
  if ! NILP (file_error) && CONSP (tail)
    { errmsg = XCAR (tail); tail = XCDR (tail); }

  {
    let mut sep: *const c_char = c": ".as_ptr ();

    if ! STRINGP (errmsg)
      { write_string (c"peculiar error".as_ptr (), stream); }
    else if SCHARS (errmsg) != 0
      { Fprinc (errmsg, stream); }
    else
      { sep = std::ptr::null (); }

    FOR_EACH_TAIL! (tail, {
	if ! sep.is_null ()
	  { write_string (sep, stream); }
	sep = c", ".as_ptr ();
	let obj: Lisp_Object = XCAR (tail);
	if ! NILP (file_error)
	    || EQ (errname, Qend_of_file ()) || EQ (errname, Quser_error ())
	  { Fprinc (obj, stream); }
	else
	  { Fprin1 (obj, stream, Qnil ()); }
      });
  }
}

/*
 * The buffer should be at least as large as the max string size of the
 * largest float, printed in the biggest notation.  This is undoubtedly
 * 20d float_output_format, with the negative of the C-constant "HUGE"
 * from <math.h>.
 *
 * On the vax the worst case is -1e38 in 20d format which takes 61 bytes.
 *
 * I assume that IEEE-754 format numbers can take 329 bytes for the worst
 * case of -1e307 in 20d float_output_format. What is one to do (short of
 * re-writing _doprnt to be more sane)?
 * 			-wsr
 * Given the above, the buffer must be least FLOAT_TO_STRING_BUFSIZE bytes.
 */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn float_to_string (buf: *mut c_char, data: c_double) -> c_int
{
  let mut cp: *mut c_char = std::ptr::null_mut ();  /* C: `char *cp;` (PORTING.md 1.5) */
  let mut width: c_int = 0;
  let mut len: c_int = 0;
  let mut lost: bool = false;

  if isinf! (data)
    {
      /* C 1190: `static char const minus_infinity_string[] = "-1.0e+INF";`  */
      let minus_infinity_string: &CStr = c"-1.0e+INF";
      let positive: bool = 0.0 < data;
      strcpy (buf, minus_infinity_string.as_ptr ().add (positive as usize));
      return minus_infinity_string.to_bytes_with_nul ().len () as c_int - 1
	- positive as c_int;
    }
  /* C 1200-1208's `#if IEEE_FLOATING_POINT` arm is live for this target (clang: true).  */
  if isnan! (data)
    {
      let u: ieee754_double = ieee754_double { d: data };
      let hi: uintmax_t = ieee754_double_bits! (ieee_nan.mantissa0, u) as uintmax_t;
      return sprintf (buf, c"-%ju.0e+NaN".as_ptr ()
		      .add ((! ieee754_double_bits! (ieee_nan.negative, u)) as usize),
		      (hi << 31 << 1) + (ieee754_double_bits! (ieee_nan.mantissa1, u) as uintmax_t));
    }

  if NILP (Vfloat_output_format)
      || ! STRINGP (Vfloat_output_format)
    { lost = true; }
  else			/* oink oink */
    {
      /* Check that the spec we have is fully valid.
	 This means not only valid for printf,
	 but meant for floats, and reasonable.  */
      cp = SSDATA (Vfloat_output_format);

      'check: {
      if *cp != b'%' as c_char
	{ lost = true; break 'check; }
      if *cp.offset (1) != b'.' as c_char
	{ lost = true; break 'check; }

      cp = cp.offset (2);

      /* Check the width specification.  */
      width = -1;
      if b'0' as c_char <= *cp && *cp <= b'9' as c_char
	{
	  width = 0;
	  loop
	    {
	      width = (width * 10) + (*cp as c_int - b'0' as c_int);
	      cp = cp.offset (1);
	      if DBL_DIG < width
		{ lost = true; break 'check; }
	      if ! (*cp >= b'0' as c_char && *cp <= b'9' as c_char)
		{ break; }
	    }

	  /* A precision of zero is valid only for %f.  */
	  if width == 0 && *cp != b'f' as c_char
	    { lost = true; break 'check; }
	}

      if *cp != b'e' as c_char && *cp != b'f' as c_char && *cp != b'g' as c_char
	{ lost = true; break 'check; }

      if *cp.offset (1) != 0
	{ lost = true; break 'check; }

      len = sprintf (buf, SSDATA (Vfloat_output_format), data);
      }
    }

  if lost
    {
      /* Generate the fewest number of digits that represent the
	 floating point value without losing information.  */
      len = dtoastr (buf, FLOAT_TO_STRING_BUFSIZE - 2, 0, 0, data);
      /* The decimal point must be printed, or the byte compiler can
	 get confused (Bug#8033). */
      width = 1;
    }

  /* Make sure there is a decimal point with digit after, or an
     exponent, so that the value is readable as a float.  But don't do
     this with "%.0f"; it's valid for that not to produce a decimal
     point.  Note that width can be 0 only for %.0f.  */
  if width != 0
    {
      cp = buf;
      while *cp != 0
	{
	  if (*cp < b'0' as c_char || *cp > b'9' as c_char) && *cp != b'-' as c_char
	    { break; }
	  cp = cp.offset (1);
	}

      if *cp == b'.' as c_char && *cp.offset (1) == 0
	{
	  *cp.offset (1) = b'0' as c_char;
	  *cp.offset (2) = 0;
	  len += 1;
	}
      else if *cp == 0
	{
	  *cp = b'.' as c_char;
	  cp = cp.offset (1);
	  *cp = b'0' as c_char;
	  cp = cp.offset (1);
	  *cp = 0;
	  cp = cp.offset (1);
	  len += 2;
	}
    }

  return len;
}

unsafe fn print (obj: Lisp_Object, printcharfun: Lisp_Object, escapeflag: bool)
{
  new_backquote_output = 0;

  /* Reset print_number_index and Vprint_number_table only when
     the variable Vprint_continuous_numbering is nil.  Otherwise,
     the values of these variables will be kept between several
     print functions.  */
  if NILP (Vprint_continuous_numbering)
      || NILP (Vprint_number_table)
    {
      print_number_index = 0;
      Vprint_number_table = Qnil ();
    }

  /* Construct Vprint_number_table for print-circle.  */
  if ! NILP (Vprint_circle)
    {
      /* Construct Vprint_number_table.
	 This increments print_number_index for the objects added.  */
      print_preprocess (obj);

      if HASH_TABLE_P (Vprint_number_table)
	{ /* Remove unnecessary objects, which appear only once in OBJ;
	     that is, whose status is Qt.  */
	  let h: *mut Lisp_Hash_Table = XHASH_TABLE (Vprint_number_table);
	  DOHASH! (h, k, v, {
	      if EQ (v, Qt ())
		{ Fremhash (k, Vprint_number_table); }
	    });
	}
    }

  print_depth = 0;
  print_object (obj, printcharfun, escapeflag);
}

unsafe fn print_circle_candidate_p (obj: Lisp_Object) -> bool
{
  if CONSP (obj)
    { return true; }
  else if STRINGP (obj)
    { return SCHARS (obj) > 0; }
  else if SYMBOLP (obj)
    { return ! NILP (Vprint_gensym) && ! SYMBOL_INTERNED_P (obj); }
  else if VECTORLIKEP (obj)
    {
      if VECTORP (obj)
	{ return ASIZE (obj) > 0; }
      else
	{ return (CLOSUREP (obj)
		  || CHAR_TABLE_P (obj) || SUB_CHAR_TABLE_P (obj)
		  || HASH_TABLE_P (obj) || FONTP (obj)
		  || RECORDP (obj)); }
    }
  else
    { return false; }
}

/* The print preprocess stack, used to traverse data structures.  */

#[repr(C)]
#[derive(Clone, Copy)]
pub struct print_pp_entry {
  pub n: ptrdiff_t,		/* number of values, or 0 if a single value */
  pub u: print_pp_entry_u,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union print_pp_entry_u {
  pub value: Lisp_Object,	/* when n = 0 */
  pub values: *mut Lisp_Object,	/* when n > 0 */
}
/* Measured with clang -std=gnu23 on the frozen target: sizeof 16, align 8, n@0, u@8.  */
const _: () = assert! (std::mem::size_of::<print_pp_entry> () == 16);
const _: () = assert! (std::mem::align_of::<print_pp_entry> () == 8);
const _: () = assert! (std::mem::offset_of! (print_pp_entry, n) == 0);
const _: () = assert! (std::mem::offset_of! (print_pp_entry, u) == 8);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct print_pp_stack {
  pub stack: *mut print_pp_entry,	 /* base of stack */
  pub size: ptrdiff_t,			 /* allocated size in entries */
  pub sp: ptrdiff_t,			 /* current number of entries */
}
/* Measured with clang -std=gnu23 on the frozen target: sizeof 24, align 8, stack@0,
   size@8, sp@16.  */
const _: () = assert! (std::mem::size_of::<print_pp_stack> () == 24);
const _: () = assert! (std::mem::align_of::<print_pp_stack> () == 8);
const _: () = assert! (std::mem::offset_of! (print_pp_stack, stack) == 0);
const _: () = assert! (std::mem::offset_of! (print_pp_stack, size) == 8);
const _: () = assert! (std::mem::offset_of! (print_pp_stack, sp) == 16);

static mut ppstack: print_pp_stack = print_pp_stack {
  stack: std::ptr::null_mut (),
  size: 0,
  sp: 0,
};

unsafe fn grow_pp_stack ()
{
  let ps: *mut print_pp_stack = &raw mut ppstack;
  eassert! ((*ps).sp == (*ps).size);
  (*ps).stack = xpalloc ((*ps).stack as *mut c_void, &raw mut (*ps).size, 1, -1,
			 std::mem::size_of::<print_pp_entry> () as ptrdiff_t)
    as *mut print_pp_entry;
  eassert! ((*ps).sp < (*ps).size);
}

unsafe fn pp_stack_push_value (value: Lisp_Object)
{
  if ppstack.sp >= ppstack.size
    { grow_pp_stack (); }
  *ppstack.stack.offset (ppstack.sp) = print_pp_entry {
    n: 0, u: print_pp_entry_u { value: value } };
  ppstack.sp += 1;
}

unsafe fn pp_stack_push_values (values: *mut Lisp_Object, n: ptrdiff_t)
{
  eassume! (n >= 0);
  if n == 0
    { return; }
  if ppstack.sp >= ppstack.size
    { grow_pp_stack (); }
  *ppstack.stack.offset (ppstack.sp) = print_pp_entry {
    n: n, u: print_pp_entry_u { values: values } };
  ppstack.sp += 1;
}

unsafe fn pp_stack_empty_p () -> bool
{
  return ppstack.sp <= 0;
}

unsafe fn pp_stack_pop () -> Lisp_Object
{
  eassume! (! pp_stack_empty_p ());
  let e: *mut print_pp_entry = ppstack.stack.offset (ppstack.sp - 1);
  if (*e).n == 0		/* single value */
    {
      ppstack.sp -= 1;
      return (*e).u.value;
    }
  /* Array of values: pop them left to right, which seems to be slightly
     faster than right to left.  */
  (*e).n -= 1;
  if (*e).n == 0
    { ppstack.sp -= 1; }		/* last value consumed */
  (*e).u.values = (*e).u.values.offset (1);
  return *(*e).u.values.offset (-1);
}

/* Construct Vprint_number_table for the print-circle feature
   according to the structure of OBJ.  OBJ itself and all its elements
   will be added to Vprint_number_table recursively if it is a list,
   vector, compiled function, char-table, string (its text properties
   will be traced), or a symbol that has no obarray (this is for the
   print-gensym feature).  The status fields of Vprint_number_table
   mean whether each object appears more than once in OBJ: Qnil at the
   first time, and Qt after that.  */
unsafe fn print_preprocess (mut obj: Lisp_Object)
{
  eassert! (! NILP (Vprint_circle));
  /* The ppstack may contain HASH_UNUSED_ENTRY_KEY which is an invalid
     Lisp value.  Make sure that our filter stops us from traversing it.  */
  eassert! (! print_circle_candidate_p (HASH_UNUSED_ENTRY_KEY ()));
  let base_sp: ptrdiff_t = ppstack.sp;

  loop
    {
      if print_circle_candidate_p (obj)
	{
	  if ! HASH_TABLE_P (Vprint_number_table)
	    { Vprint_number_table = CALLN! (Fmake_hash_table, QCtest (), Qeq ()); }

	  let num: Lisp_Object = Fgethash (obj, Vprint_number_table, Qnil ());
	  if ! NILP (num)
	      /* If Vprint_continuous_numbering is non-nil and OBJ is a gensym,
		 always print the gensym with a number.  This is a special for
		 the lisp function byte-compile-output-docform.  */
	      || (! NILP (Vprint_continuous_numbering)
		  && SYMBOLP (obj)
		  && ! SYMBOL_INTERNED_P (obj))
	    { /* OBJ appears more than once.  Let's remember that.  */
	      if SYMBOLP (num) /* In practice, nil or t.  */
		{
		  print_number_index += 1;
		  /* Negative number indicates it hasn't been printed yet.  */
		  Fputhash (obj, make_fixnum (- print_number_index),
			    Vprint_number_table);
		}
	    }
	  else
	    {
	      /* OBJ is not yet recorded.  Let's add to the table.  */
	      Fputhash (obj, Qt (), Vprint_number_table);

	      match XTYPE (obj)
		{
		  Lisp_Type::Lisp_String => {
		    /* A string may have text properties,
		       which can be circular. */
		    traverse_intervals_noorder (string_intervals (obj),
						Some (print_preprocess_string),
						std::ptr::null_mut ());
		  }
		  Lisp_Type::Lisp_Cons => {
		    if ! NILP (XCDR (obj))
		      { pp_stack_push_value (XCDR (obj)); }
		    obj = XCAR (obj);
		    continue;
		  }
		  Lisp_Type::Lisp_Vectorlike => {
		    let vec: *mut Lisp_Vector = XVECTOR (obj);
		    let mut size: ptrdiff_t = ASIZE (obj);
		    if (size & PSEUDOVECTOR_FLAG) != 0
		      { size &= PSEUDOVECTOR_SIZE_MASK; }
		    let start: ptrdiff_t = if SUB_CHAR_TABLE_P (obj)
		      { SUB_CHAR_TABLE_OFFSET } else { 0 };
		    pp_stack_push_values ((*vec).contents.as_mut_ptr ().offset (start),
					  size - start);
		    if HASH_TABLE_P (obj)
		      {
			let h: *mut Lisp_Hash_Table = XHASH_TABLE (obj);
			/* The values pushed here may include
			   HASH_UNUSED_ENTRY_KEY; see top of this function.  */
			pp_stack_push_values ((*h).key_and_value,
					      2 * (*h).table_size as ptrdiff_t);
		      }
		  }
		  _ => { }
		}
	    }
	}

      if ppstack.sp <= base_sp
	{ break; }
      obj = pp_stack_pop ();
    }
}

DEFUN! ("print--preprocess", Fprint_preprocess, Sprint_preprocess, 1, 1, 0,
       doc: "Extract sharing info from OBJECT needed to print it.\nFills `print-number-table' if `print-circle' is non-nil.  Does nothing\nif `print-circle' is nil.",
       (object: Lisp_Object),
       {
  if ! NILP (Vprint_circle)
    {
      print_number_index = 0;
      print_preprocess (object);
    }
  return Qnil ();
});

unsafe extern "C" fn print_preprocess_string (interval: INTERVAL, arg: *mut c_void)
{
  let _ = arg;
  let interval: *mut interval = interval as *mut interval;   /* P-72 rule 4 */
  print_preprocess ((*interval).plist);
}

/* C 1536-1537: */
pub const PRINT_STRING_NON_CHARSET_FOUND: c_int = 1;
pub const PRINT_STRING_UNSAFE_CHARSET_FOUND: c_int = 2;

/* Bitwise or of the above macros.  */
static mut print_check_string_result: c_int = 0;

unsafe extern "C" fn print_check_string_charset_prop (interval: INTERVAL, pstring: *mut c_void)
{
  let interval: *mut interval = interval as *mut interval;   /* P-72 rule 4 */
  let mut val: Lisp_Object;

  if NILP ((*interval).plist)
      || (print_check_string_result == (PRINT_STRING_NON_CHARSET_FOUND
					| PRINT_STRING_UNSAFE_CHARSET_FOUND))
    { return; }
  val = (*interval).plist;
  while CONSP (val) && ! EQ (XCAR (val), Qcharset ())
    { val = XCDR (XCDR (val)); }
  if ! CONSP (val)
    {
      print_check_string_result |= PRINT_STRING_NON_CHARSET_FOUND;
      return;
    }
  if (print_check_string_result & PRINT_STRING_NON_CHARSET_FOUND) == 0
    {
      if ! EQ (val, (*interval).plist)
	  || CONSP (XCDR (XCDR (val)))
	{ print_check_string_result |= PRINT_STRING_NON_CHARSET_FOUND; }
    }
  if (print_check_string_result & PRINT_STRING_UNSAFE_CHARSET_FOUND) == 0
    {
      let mut charpos: ptrdiff_t = (*interval).position;
      let string: Lisp_Object = *(pstring as *mut Lisp_Object);
      let mut bytepos: ptrdiff_t = string_char_to_byte (string, charpos);
      let charset: Lisp_Object = XCAR (XCDR (val));

      let mut i: ptrdiff_t = 0;
      while i < LENGTH! (interval)
	{
	  let c: c_int = fetch_string_char_advance (string, &raw mut charpos, &raw mut bytepos);
	  if ! ASCII_CHAR_P (c as isize)
	      && ! EQ (CHARSET_NAME! (CHAR_CHARSET! (c)), charset)
	    {
	      print_check_string_result |= PRINT_STRING_UNSAFE_CHARSET_FOUND;
	      break;
	    }
	  i += 1;
	}
    }
}

/* The value is (charset . nil).  */
static mut print_prune_charset_plist: Lisp_Object = std::ptr::null_mut ();

unsafe fn print_prune_string_charset (mut string: Lisp_Object) -> Lisp_Object
{
  print_check_string_result = 0;
  traverse_intervals (string_intervals (string), 0,
		      Some (print_check_string_charset_prop), &raw mut string as *mut c_void);
  if NILP (Vprint_charset_text_property)
      || (print_check_string_result & PRINT_STRING_UNSAFE_CHARSET_FOUND) == 0
    {
      string = Fcopy_sequence (string);
      if (print_check_string_result & PRINT_STRING_NON_CHARSET_FOUND) != 0
	{
	  if NILP (print_prune_charset_plist)
	    { print_prune_charset_plist = list1 (Qcharset ()); }
	  Fremove_text_properties (make_fixnum (0),
				   make_fixnum (SCHARS (string)),
				   print_prune_charset_plist, string);
	}
      else
	{ Fset_text_properties (make_fixnum (0), make_fixnum (SCHARS (string)),
				Qnil (), string); }
    }
  return string;
}

/* C 1612-1639's `#ifdef HAVE_MODULES` arm (feature `have_modules`): the two helpers the
   `PVEC_MODULE_FUNCTION` case below calls.  */
#[cfg (feature = "have_modules")]
/* Return a data pointer equal to FUNCPTR.  */
unsafe fn data_from_funcptr (funcptr: Option<unsafe extern "C" fn ()>) -> *const c_void
{
  /* The module code, and the POSIX API for dynamic linking, already
     assume that function and data pointers are represented
     interchangeably, so it's OK to assume that here too.  */
  return funcptr.map_or (std::ptr::null (), |f| f as *const c_void);
}

/* Print the value of the pointer PTR.  */
#[cfg (feature = "have_modules")]
unsafe fn print_pointer (printcharfun: Lisp_Object, buf: *mut c_char, prefix: *const c_char,
			 ptr: *const c_void)
{
  let ui: uintptr_t = ptr as uintptr_t;

  /* In theory this assignment could lose info on pre-C99 hosts, but
     in practice it doesn't.  */
  let up: uintmax_t = ui;

  let len: c_int = sprintf (buf, c"%s 0x%jx".as_ptr (), prefix, up);
  strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
}

unsafe fn print_bignum (obj: Lisp_Object, printcharfun: Lisp_Object)
{
  let size: ptrdiff_t = bignum_bufsize (obj, 10);
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let str: *mut c_char = SAFE_ALLOCA! (sa_avail, size) as *mut c_char;
  let len: ptrdiff_t = bignum_to_c_string (str, size, obj, 10);
  strout (str, len, len, printcharfun);
  SAFE_FREE! (sa_count);
}

unsafe fn print_bool_vector (obj: Lisp_Object, printcharfun: Lisp_Object)
{
  let size: EMACS_INT = bool_vector_size (obj);
  let mut size_in_bytes: ptrdiff_t = bool_vector_bytes (size) as ptrdiff_t;
  let real_size_in_bytes: ptrdiff_t = size_in_bytes;
  let data: *mut u8 = bool_vector_uchar_data (obj);

  let mut buf: [c_char; 3 + INT_STRLEN_BOUND (EMACS_INT_WIDTH, 1)] =
    [0; 3 + INT_STRLEN_BOUND (EMACS_INT_WIDTH, 1)];
  let len: c_int = sprintf (buf.as_mut_ptr (), c"#&%ld\"".as_ptr (), size);
  strout (buf.as_ptr (), len as ptrdiff_t, len as ptrdiff_t, printcharfun);

  /* Don't print more bytes than the specified maximum.
     Negative values of print-length are invalid.  Treat them
     like a print-length of nil.  */
  if FIXNATP (Vprint_length)
      && XFIXNAT (Vprint_length) < size_in_bytes as EMACS_INT
    { size_in_bytes = XFIXNAT (Vprint_length) as ptrdiff_t; }

  let mut i: ptrdiff_t = 0;
  while i < size_in_bytes
    {
      maybe_quit ();
      let c: u8 = *data.offset (i);
      if c == b'\n' as u8 && print_escape_newlines
	{ print_c_string (c"\\n".as_ptr (), printcharfun); }
      else if c == b'\x0c' as u8 && print_escape_newlines
	{ print_c_string (c"\\f".as_ptr (), printcharfun); }
      else if c as c_int > 0o177
	       || (print_escape_control_characters && c_iscntrl (c as c_int))
	{
	  /* Use octal escapes to avoid encoding issues.  */
	  octalout (c, data, i + 1, size_in_bytes, printcharfun);
	}
      else
	{
	  if c == b'\x22' as u8 || c == b'\\' as u8
	    { printchar (b'\\' as c_uint, printcharfun); }
	  printchar (c as c_uint, printcharfun);
	}
      i += 1;
    }

  if size_in_bytes < real_size_in_bytes
    { print_c_string (c" ...".as_ptr (), printcharfun); }
  printchar (b'\x22' as c_uint, printcharfun);
}

/* Print a pseudovector that has no readable syntax.  */
unsafe fn print_vectorlike_unreadable (obj: Lisp_Object, printcharfun: Lisp_Object,
				       escapeflag: bool, buf: *mut c_char)
{
  /* First check whether this is handled by `print-unreadable-function'.  */
  if ! NILP (Vprint_unreadable_function)
      && FUNCTIONP (Vprint_unreadable_function)
    {
      let count: specpdl_ref = SPECPDL_INDEX ();
      /* Bind `print-unreadable-function' to nil to avoid accidental
	 infinite recursion in the function called.  */
      let func: Lisp_Object = Vprint_unreadable_function;
      specbind (Qprint_unreadable_function (), Qnil ());

      /* If we're being called from `prin1-to-string' or the like,
	 we're now in the secret " prin1" buffer.  This can lead to
	 problems if, for instance, the callback function switches a
	 window to this buffer -- this will make Emacs segfault.  */
      if ! NILP (Vprint__unreadable_callback_buffer)
	  && ! NILP (Fbuffer_live_p (Vprint__unreadable_callback_buffer))
	{
	  record_unwind_current_buffer ();
	  set_buffer_internal (XBUFFER (Vprint__unreadable_callback_buffer));
	}
      let result: Lisp_Object = calln! (func, obj, if escapeflag { Qt () } else { Qnil () });
      unbind_to (count, Qnil ());

      if ! NILP (result)
	{
	  if STRINGP (result)
	    { print_string (result, printcharfun); }
	  /* It's handled, so stop processing here.  */
	  return;
	}
    }

  /* Not handled; print unreadable object.  */
  match PSEUDOVECTOR_TYPE (XVECTOR (obj))
    {
    pvec_type::PVEC_MARKER => {
      print_c_string (c"#<marker ".as_ptr (), printcharfun);
      /* Do you think this is necessary?  */
      if MARKER_INSERTION_TYPE_P (XMARKER (obj))
	{ print_c_string (c"(moves after insertion) ".as_ptr (), printcharfun); }
      if (*XMARKER (obj)).buffer.is_null ()
	{ print_c_string (c"in no buffer".as_ptr (), printcharfun); }
      else
	{
	  let len: c_int = sprintf (buf, c"at %td in ".as_ptr (), marker_position (obj));
	  strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
	  print_string (BVAR! ((*XMARKER (obj)).buffer as *mut buffer, name_), printcharfun);
	}
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_SYMBOL_WITH_POS => {
      let sp: *mut Lisp_Symbol_With_Pos = XSYMBOL_WITH_POS (obj);
      if print_symbols_bare
	{ print_object ((*sp).sym, printcharfun, escapeflag); }
      else
	{
	  print_c_string (c"#<symbol ".as_ptr (), printcharfun);
	  if BARE_SYMBOL_P ((*sp).sym)
	    { print_object ((*sp).sym, printcharfun, escapeflag); }
	  else
	    { print_c_string (c"NOT A SYMBOL!!".as_ptr (), printcharfun); }
	  if FIXNUMP ((*sp).pos)
	    {
	      print_c_string (c" at ".as_ptr (), printcharfun);
	      print_object ((*sp).pos, printcharfun, escapeflag);
	    }
	  else
	    { print_c_string (c" NOT A POSITION!!".as_ptr (), printcharfun); }
	  printchar (b'>' as c_uint, printcharfun);
	}
      return;
    }

    pvec_type::PVEC_OVERLAY => {
      print_c_string (c"#<overlay ".as_ptr (), printcharfun);
      if OVERLAY_BUFFER (obj).is_null ()
	{ print_c_string (c"in no buffer".as_ptr (), printcharfun); }
      else
	{
	  let len: c_int = sprintf (buf, c"from %td to %td in ".as_ptr (),
				    OVERLAY_START (obj),
				    OVERLAY_END   (obj));
	  strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
	  print_string (BVAR! (OVERLAY_BUFFER (obj), name_),
			printcharfun);
	}
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_USER_PTR => {
      print_c_string (c"#<user-ptr ".as_ptr (), printcharfun);
      let i: c_int = sprintf (buf, c"ptr=%p finalizer=%p".as_ptr (),
			      (*XUSER_PTR (obj)).p,
			      (*XUSER_PTR (obj)).finalizer
				.map_or (std::ptr::null_mut (), |f| f as *mut c_void));
      strout (buf, i as ptrdiff_t, i as ptrdiff_t, printcharfun);
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_FINALIZER => {
      print_c_string (c"#<finalizer".as_ptr (), printcharfun);
      if NILP ((*XFINALIZER (obj)).function)
	{ print_c_string (c" used".as_ptr (), printcharfun); }
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_MISC_PTR => {
      /* This shouldn't happen in normal usage, but let's
	 print it anyway for the benefit of the debugger.  */
      let i: c_int = sprintf (buf, c"#<ptr %p>".as_ptr (), xmint_pointer (obj));
      strout (buf, i as ptrdiff_t, i as ptrdiff_t, printcharfun);
      return;
    }

    pvec_type::PVEC_PROCESS => {
      if escapeflag
	{
	  print_c_string (c"#<process ".as_ptr (), printcharfun);
	  print_string ((*XPROCESS (obj)).name, printcharfun);
	  printchar (b'>' as c_uint, printcharfun);
	}
      else
	{ print_string ((*XPROCESS (obj)).name, printcharfun); }
      return;
    }

    pvec_type::PVEC_SUBR => {
      print_c_string (c"#<subr ".as_ptr (), printcharfun);
      print_c_string ((*XSUBR (obj)).symbol_name, printcharfun);
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_XWIDGET => {
      /* C 1839-1858's `#ifdef HAVE_XWIDGETS` arm, with its `USE_GTK` sub-arm (features
	 `have_xwidgets`, `use_gtk`).  */
      #[cfg (feature = "have_xwidgets")]
      {
	if NILP ((*XXWIDGET (obj)).buffer)
	  { print_c_string (c"#<killed xwidget>".as_ptr (), printcharfun); }
	else
	  {
	    #[cfg (feature = "use_gtk")]
	    {
	      let len: c_int = sprintf (buf, c"#<xwidget %u %p>".as_ptr (),
					(*XXWIDGET (obj)).xwidget_id,
					(*XXWIDGET (obj)).widget_osr);
	      strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
	    }
	    #[cfg (not (feature = "use_gtk"))]
	    {
	      let len: c_int = sprintf (buf, c"#<xwidget %u %p>".as_ptr (),
					(*XXWIDGET (obj)).xwidget_id,
					(*XXWIDGET (obj)).xwWidget);
	      strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
	    }
	  }
	return;
      }
    }

    pvec_type::PVEC_XWIDGET_VIEW => {
      print_c_string (c"#<xwidget view".as_ptr (), printcharfun);
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_WINDOW => {
      let len: c_int = sprintf (buf, c"#<window %ld".as_ptr (),
				(*XWINDOW (obj)).sequence_number);
      strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
      if BUFFERP ((*XWINDOW (obj)).contents)
	{
	  print_c_string (c" on ".as_ptr (), printcharfun);
	  print_string (BVAR! (XBUFFER ((*XWINDOW (obj)).contents), name_),
			printcharfun);
	}
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_TERMINAL => {
      let t: *mut terminal = XTERMINAL (obj);
      let len: c_int = sprintf (buf, c"#<terminal %d".as_ptr (), (*t).id);
      strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
      if ! (*t).name.is_null ()
	{
	  print_c_string (c" on ".as_ptr (), printcharfun);
	  print_c_string ((*t).name, printcharfun);
	}
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_BUFFER => {
      if ! BUFFER_LIVE_P (XBUFFER (obj))
	{ print_c_string (c"#<killed buffer>".as_ptr (), printcharfun); }
      else if escapeflag
	{
	  print_c_string (c"#<buffer ".as_ptr (), printcharfun);
	  print_string (BVAR! (XBUFFER (obj), name_), printcharfun);
	  printchar (b'>' as c_uint, printcharfun);
	}
      else
	{ print_string (BVAR! (XBUFFER (obj), name_), printcharfun); }
      return;
    }

    pvec_type::PVEC_WINDOW_CONFIGURATION => {
      print_c_string (c"#<window-configuration>".as_ptr (), printcharfun);
      return;
    }

    pvec_type::PVEC_FRAME => {
      let ptr: *mut c_void = XFRAME (obj) as *mut c_void;
      let mut frame_name: Lisp_Object = (*XFRAME (obj)).name;

      print_c_string (if FRAME_LIVE_P (XFRAME (obj))
			{ c"#<frame ".as_ptr () }
		      else
			{ c"#<dead frame ".as_ptr () },
			printcharfun);
      if ! STRINGP (frame_name)
	{
	  /* A frame could be too young and have no name yet;
	     don't crash.  */
	  if SYMBOLP (frame_name)
	    { frame_name = Fsymbol_name (frame_name); }
	  else	/* can't happen: name should be either nil or string */
	    { frame_name = build_string (c"*INVALID*FRAME*NAME*".as_ptr ()); }
	}
      print_string (frame_name, printcharfun);
      let len: c_int = sprintf (buf, c" %p>".as_ptr (), ptr);
      strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
      return;
    }

    pvec_type::PVEC_FONT => {
      if ! FONT_OBJECT_P (obj)
	{
	  if FONT_SPEC_P (obj)
	    { print_c_string (c"#<font-spec".as_ptr (), printcharfun); }
	  else
	    { print_c_string (c"#<font-entity".as_ptr (), printcharfun); }
	  let mut i: c_int = 0;
	  while i < FONT_SPEC_MAX as c_int
	    {
	      /* FONT_EXTRA_INDEX can contain private information in
		 font entities which isn't safe to print.  */
	      if i != FONT_EXTRA_INDEX as c_int || ! FONT_ENTITY_P (obj)
		{
		  printchar (b' ' as c_uint, printcharfun);
		  if i < FONT_WEIGHT_INDEX as c_int || i > FONT_WIDTH_INDEX as c_int
		    { print_object (AREF (obj, i as isize), printcharfun, escapeflag); }
		  else
		    { print_object (font_style_symbolic (obj, i, false),
				    printcharfun, escapeflag); }
		}
	      i += 1;
	    }
	}
      else
	{
	  print_c_string (c"#<font-object ".as_ptr (), printcharfun);
	  print_object (AREF (obj, FONT_NAME_INDEX as isize), printcharfun,
			escapeflag);
	}
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_THREAD => {
      print_c_string (c"#<thread ".as_ptr (), printcharfun);
      if STRINGP ((*XTHREAD (obj)).name)
	{ print_string ((*XTHREAD (obj)).name, printcharfun); }
      else
	{
	  let p: *mut c_void = XTHREAD (obj) as *mut c_void;
	  let len: c_int = sprintf (buf, c"%p".as_ptr (), p);
	  strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
	}
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_MUTEX => {
      print_c_string (c"#<mutex ".as_ptr (), printcharfun);
      if STRINGP ((*XMUTEX (obj)).name)
	{ print_string ((*XMUTEX (obj)).name, printcharfun); }
      else
	{
	  let p: *mut c_void = XMUTEX (obj) as *mut c_void;
	  let len: c_int = sprintf (buf, c"%p".as_ptr (), p);
	  strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
	}
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_CONDVAR => {
      print_c_string (c"#<condvar ".as_ptr (), printcharfun);
      if STRINGP ((*XCONDVAR (obj)).name)
	{ print_string ((*XCONDVAR (obj)).name, printcharfun); }
      else
	{
	  let p: *mut c_void = XCONDVAR (obj) as *mut c_void;
	  let len: c_int = sprintf (buf, c"%p".as_ptr (), p);
	  strout (buf, len as ptrdiff_t, len as ptrdiff_t, printcharfun);
	}
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_MODULE_FUNCTION => {
      /* C 2009-2036's `#ifdef HAVE_MODULES` arm (feature `have_modules`).  */
      #[cfg (feature = "have_modules")]
      {
	print_c_string (c"#<module function ".as_ptr (), printcharfun);
        let function: *const Lisp_Module_Function = XMODULE_FUNCTION (obj);
        let ptr: module_funcptr = module_function_address (function);
	let mut file: *const c_char = std::ptr::null ();
	let mut symbol: *const c_char = std::ptr::null ();
	dynlib_addr (ptr as *mut c_void, &raw mut file, &raw mut symbol);

	if symbol.is_null ()
          { print_pointer (printcharfun, buf, c"at".as_ptr (),
			   data_from_funcptr (ptr)); }
        else
	  { print_c_string (symbol, printcharfun); }

        let data: *mut c_void = module_function_data (function);
        if ! data.is_null ()
          { print_pointer (printcharfun, buf, c" with data".as_ptr (), data); }

        if ! file.is_null ()
	  {
	    print_c_string (c" from ".as_ptr (), printcharfun);
	    print_c_string (file, printcharfun);
	  }

	printchar (b'>' as c_uint, printcharfun);
	return;
      }
    }

    pvec_type::PVEC_NATIVE_COMP_UNIT => {
      /* C 2040-2050's `#ifdef HAVE_NATIVE_COMP` arm (feature `have_native_comp`).  */
      #[cfg (feature = "have_native_comp")]
      {
	let cu: *mut Lisp_Native_Comp_Unit = XNATIVE_COMP_UNIT (obj);
	print_c_string (c"#<native compilation unit: ".as_ptr (), printcharfun);
	print_object ((*cu).file, printcharfun, escapeflag);
	printchar (b' ' as c_uint, printcharfun);
	print_object ((*cu).optimize_qualities, printcharfun, escapeflag);
	printchar (b'>' as c_uint, printcharfun);
	return;
      }
    }

    pvec_type::PVEC_TS_PARSER => {
      /* C 2054-2062's `#ifdef HAVE_TREE_SITTER` arm (feature `have_tree_sitter`).  */
      #[cfg (feature = "have_tree_sitter")]
      {
	print_c_string (c"#<treesit-parser for ".as_ptr (), printcharfun);
	let language: Lisp_Object = (*XTS_PARSER (obj)).language_symbol;
	/* No need to print the buffer because it's not that useful: we
	   usually know which buffer a parser belongs to.  */
	print_string (Fsymbol_name (language), printcharfun);
	printchar (b'>' as c_uint, printcharfun);
	return;
      }
    }

    pvec_type::PVEC_TS_NODE => {
      /* C 2066-2100's `#ifdef HAVE_TREE_SITTER` arm (feature `have_tree_sitter`).  */
      #[cfg (feature = "have_tree_sitter")]
      {
	/* Prints #<treesit-node (identifier) in 12-15> or
           #<treesit-node "keyword" in 28-31>. */
	print_c_string (c"#<treesit-node".as_ptr (), printcharfun);
	if ! treesit_node_uptodate_p (obj)
	  {
	    print_c_string (c"-outdated>".as_ptr (), printcharfun);
	    return;
	  }
	if ! treesit_node_buffer_live_p (obj)
	  {
	    print_c_string (c"-in-killed-buffer>".as_ptr (), printcharfun);
	    return;
	  }
	printchar (b' ' as c_uint, printcharfun);
	/* Now the node must be up-to-date, and calling functions like
	   Ftreesit_node_start will not signal.  */
	let named: bool = treesit_named_node_p ((*XTS_NODE (obj)).node);
	/* We used to use () as delimiters for named nodes, but that
	   confuses pretty-printing a tad bit.  There might be more
	   little breakages here and there if we print parenthesizes
	   inside an object, so I guess better not do it.
	   (bug#60696)  */
	let delim1: *const c_char = if named { c"".as_ptr () } else { c"\"".as_ptr () };
	let delim2: *const c_char = if named { c"".as_ptr () } else { c"\"".as_ptr () };
	print_c_string (delim1, printcharfun);
	print_string (Ftreesit_node_type (obj), printcharfun);
	print_c_string (delim2, printcharfun);
	print_c_string (c" in ".as_ptr (), printcharfun);
	print_object (Ftreesit_node_start (obj), printcharfun, escapeflag);
	printchar (b'-' as c_uint, printcharfun);
	print_object (Ftreesit_node_end (obj), printcharfun, escapeflag);
	printchar (b'>' as c_uint, printcharfun);
	return;
      }
    }

    pvec_type::PVEC_TS_COMPILED_QUERY => {
      /* C 2104-2107's `#ifdef HAVE_TREE_SITTER` arm (feature `have_tree_sitter`).  */
      #[cfg (feature = "have_tree_sitter")]
      {
	print_c_string (c"#<treesit-compiled-query>".as_ptr (), printcharfun);
	return;
      }
    }

    pvec_type::PVEC_SQLITE => {
      print_c_string (c"#<sqlite ".as_ptr (), printcharfun);
      let mut i: c_int = sprintf (buf, c"db=%p".as_ptr (), (*XSQLITE (obj)).db);
      strout (buf, i as ptrdiff_t, i as ptrdiff_t, printcharfun);
      if (*XSQLITE (obj)).is_statement
	{
	  i = sprintf (buf, c" stmt=%p".as_ptr (), (*XSQLITE (obj)).stmt);
	  strout (buf, i as ptrdiff_t, i as ptrdiff_t, printcharfun);
	}
      print_c_string (c" name=".as_ptr (), printcharfun);
      print_c_string ((*XSQLITE (obj)).name, printcharfun);
      printchar (b'>' as c_uint, printcharfun);
      return;
    }

    pvec_type::PVEC_OBARRAY => {
      let o: *mut Lisp_Obarray = XOBARRAY (obj);
      /* FIXME: Would it make sense to print the actual symbols (up to
	 a limit)?  */
      let i: c_int = sprintf (buf, c"#<obarray n=%u>".as_ptr (), (*o).count);
      strout (buf, i as ptrdiff_t, i as ptrdiff_t, printcharfun);
      return;
    }

    /* Types handled earlier.  */
    pvec_type::PVEC_NORMAL_VECTOR
    | pvec_type::PVEC_RECORD
    | pvec_type::PVEC_CLOSURE
    | pvec_type::PVEC_CHAR_TABLE
    | pvec_type::PVEC_SUB_CHAR_TABLE
    | pvec_type::PVEC_HASH_TABLE
    | pvec_type::PVEC_BIGNUM
    | pvec_type::PVEC_BOOL_VECTOR
    /* Impossible cases.  */
    | pvec_type::PVEC_FREE
    | pvec_type::PVEC_OTHER => { }
    }
  emacs_abort ();
}

unsafe fn named_escape (i: c_int) -> c_char
{
  match i
    {
    0x08 => { return b'b' as c_char; }
    0x09 => { return b't' as c_char; }
    0x0a => { return b'n' as c_char; }
    0x0c => { return b'f' as c_char; }
    0x0d => { return b'r' as c_char; }
    0x20 => { return b's' as c_char; }
      /* \a, \v, \e and \d are excluded from printing as escapes since
         they are somewhat rare as characters and more likely to be
         plain integers. */
    _ => { }
    }
  return 0;
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum print_entry_type
  {
    PE_list = 0,			/* print rest of list */
    PE_rbrac = 1,			/* print ")" */
    PE_vector = 2,			/* print rest of vector */
    PE_hash = 3,			/* print rest of hash data */
  }
/* Measured with clang -std=gnu23 on the frozen target: sizeof 4, align 4.  */
const _: () = assert! (std::mem::size_of::<print_entry_type> () == 4);
const _: () = assert! (std::mem::align_of::<print_entry_type> () == 4);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct print_stack_entry_list {
  pub last: Lisp_Object,		/* cons whose car was just printed  */
  pub maxlen: intmax_t,		/* max number of elements left to print */
  /* State for Brent cycle detection.  See
     Brent RP. BIT. 1980;20(2):176-184. doi:10.1007/BF01933190
     https://maths-people.anu.edu.au/~brent/pd/rpb051i.pdf */
  pub tortoise: Lisp_Object,     /* slow pointer */
  pub n: ptrdiff_t,		/* tortoise step countdown */
  pub m: ptrdiff_t,		/* tortoise step period */
  pub tortoise_idx: intmax_t,	/* index of tortoise */
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct print_stack_entry_dotted_cdr {
  pub obj: Lisp_Object,		/* object to print after " . " */
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct print_stack_entry_vector {
  pub obj: Lisp_Object,		/* vector object */
  pub size: ptrdiff_t,		/* length of vector */
  pub idx: ptrdiff_t,		/* index of next element */
  pub end: *const c_char,		/* string to print at end */
  pub truncated: bool,		/* whether to print "..." before end */
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct print_stack_entry_hash {
  pub obj: Lisp_Object,		/* hash-table object */
  pub nobjs: ptrdiff_t,		/* number of keys and values to print */
  pub idx: ptrdiff_t,		/* index of key-value pair */
  pub printed: ptrdiff_t,	/* number of keys and values printed */
  pub truncated: bool,		/* whether to print "..." before end */
}

/// C: the anonymous union member `u` of `struct print_stack_entry` (PORTING.md 1.3 names
/// nested anonymous members `<container>_<member>`).
#[repr(C)]
#[derive(Clone, Copy)]
pub union print_stack_entry_u {
  pub list: print_stack_entry_list,
  pub dotted_cdr: print_stack_entry_dotted_cdr,
  pub vector: print_stack_entry_vector,
  pub hash: print_stack_entry_hash,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct print_stack_entry {
  pub r#type: print_entry_type,
  pub u: print_stack_entry_u,
}
/* Measured with clang -std=gnu23 on the frozen target: sizeof 56, align 8, type@0, u@8,
   list.last@8, list.maxlen@16, list.tortoise@24, list.n@32, list.m@40,
   list.tortoise_idx@48, dotted_cdr.obj@8, vector.obj@8, vector.size@16, vector.idx@24,
   vector.end@32, vector.truncated@40, hash.obj@8, hash.nobjs@16, hash.idx@24,
   hash.printed@32, hash.truncated@40.  */
const _: () = assert! (std::mem::size_of::<print_stack_entry> () == 56);
const _: () = assert! (std::mem::align_of::<print_stack_entry> () == 8);
const _: () = assert! (std::mem::offset_of! (print_stack_entry, r#type) == 0);
const _: () = assert! (std::mem::offset_of! (print_stack_entry, u) == 8);
const _: () = assert! (std::mem::size_of::<print_stack_entry_list> () == 48);
const _: () = assert! (std::mem::offset_of! (print_stack_entry_list, last) == 0);
const _: () = assert! (std::mem::offset_of! (print_stack_entry_list, maxlen) == 8);
const _: () = assert! (std::mem::offset_of! (print_stack_entry_list, tortoise) == 16);
const _: () = assert! (std::mem::offset_of! (print_stack_entry_list, n) == 24);
const _: () = assert! (std::mem::offset_of! (print_stack_entry_list, m) == 32);
const _: () = assert! (std::mem::offset_of! (print_stack_entry_list, tortoise_idx) == 40);
const _: () = assert! (std::mem::size_of::<print_stack_entry_vector> () == 40);
const _: () = assert! (std::mem::offset_of! (print_stack_entry_vector, end) == 24);
const _: () = assert! (std::mem::offset_of! (print_stack_entry_vector, truncated) == 32);
const _: () = assert! (std::mem::size_of::<print_stack_entry_hash> () == 40);
const _: () = assert! (std::mem::offset_of! (print_stack_entry_hash, printed) == 24);
const _: () = assert! (std::mem::offset_of! (print_stack_entry_hash, truncated) == 32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct print_stack {
  pub stack: *mut print_stack_entry,  /* base of stack */
  pub size: ptrdiff_t,		    /* allocated size in entries */
  pub sp: ptrdiff_t,		    /* current number of entries */
}
/* Measured with clang -std=gnu23 on the frozen target: sizeof 24, align 8, stack@0,
   size@8, sp@16.  */
const _: () = assert! (std::mem::size_of::<print_stack> () == 24);
const _: () = assert! (std::mem::align_of::<print_stack> () == 8);
const _: () = assert! (std::mem::offset_of! (print_stack, stack) == 0);
const _: () = assert! (std::mem::offset_of! (print_stack, size) == 8);
const _: () = assert! (std::mem::offset_of! (print_stack, sp) == 16);

static mut prstack: print_stack = print_stack {
  stack: std::ptr::null_mut (),
  size: 0,
  sp: 0,
};

unsafe fn grow_print_stack ()
{
  let ps: *mut print_stack = &raw mut prstack;
  eassert! ((*ps).sp == (*ps).size);
  (*ps).stack = xpalloc ((*ps).stack as *mut c_void, &raw mut (*ps).size, 1, -1,
			 std::mem::size_of::<print_stack_entry> () as ptrdiff_t)
    as *mut print_stack_entry;
  eassert! ((*ps).sp < (*ps).size);
}

unsafe fn print_stack_push (e: print_stack_entry)
{
  if prstack.sp >= prstack.size
    { grow_print_stack (); }
  *prstack.stack.offset (prstack.sp) = e;
  prstack.sp += 1;
}

unsafe fn print_stack_push_vector (lbrac: *const c_char, rbrac: *const c_char,
				   obj: Lisp_Object, start: ptrdiff_t, size: ptrdiff_t,
				   printcharfun: Lisp_Object)
{
  print_c_string (lbrac, printcharfun);

  let print_size: ptrdiff_t = if FIXNATP (Vprint_length)
				&& XFIXNAT (Vprint_length) < size as EMACS_INT
    { XFIXNAT (Vprint_length) as ptrdiff_t } else { size };
  print_stack_push (print_stack_entry {
      r#type: print_entry_type::PE_vector,
      u: print_stack_entry_u { vector: print_stack_entry_vector {
	obj: obj,
	size: print_size,
	idx: start,
	end: rbrac,
	truncated: print_size < size,
      } },
    });
}

/* C's `h->weakness` is the `weakness : 3` bitfield of `struct Lisp_Hash_Table`
   (lisp.h 2620-2630), which the mirror packs into `bits` (lisp.rs 2263, 2289).  Rust has no
   bitfields (PORTING.md 1.3/P-80), so the read is the member-name accessor macro that keeps
   C's member name in the port's text.  */
macro_rules! hash_table_bits {
  (weakness, $h:expr) => { (((*$h).bits & HASH_WEAKNESS_MASK) as hash_table_weakness_t) };
}

unsafe fn print_object (mut obj: Lisp_Object, printcharfun: Lisp_Object, escapeflag: bool)
{
  let base_depth: ptrdiff_t = print_depth;
  let base_sp: ptrdiff_t = prstack.sp;
  /* C 2275-2281: `char buf[max (sizeof "from..to..in " + 2 * INT_STRLEN_BOUND (EMACS_INT),
     max (sizeof " . #" + INT_STRLEN_BOUND (intmax_t), max (sizeof " with data 0x"
     + (UINTMAX_WIDTH + 4 - 1) / 4, 40)))]`.  An array length must be const-evaluable and
     lisp.rs's `max` is an `isize` function, so the fold is `max_usize`; the C `max` token
     is the one `docs/ports/macro-body-notes.tsv` records for this body (the
     Fnumber_to_string row's reason).  `sizeof "…"` is the literal's byte count plus its
     null: 14, 5 and 14.  */
  const BUF_SIZE: usize = max_usize (
    14 + 2 * INT_STRLEN_BOUND (EMACS_INT_WIDTH, 1),
    max_usize (5 + INT_STRLEN_BOUND (INTMAX_WIDTH as usize, 1),
	       max_usize (14 + (UINTMAX_WIDTH as usize + 4 - 1) / 4, 40)));
  let mut buf: [c_char; BUF_SIZE] = [0; BUF_SIZE];
  (*current_thread).stack_top = NEAR_STACK_TOP! (buf.as_mut_ptr () as *const c_void);

  'print_obj: loop {
  maybe_quit ();

  let mut to_next_obj: bool = false;

  /* Detect circularities and truncate them.  */
  if NILP (Vprint_circle)
    {
      /* Simple but incomplete way.  */
      if print_depth >= PRINT_CIRCLE as ptrdiff_t
	{ error (c"Apparently circular structure being printed".as_ptr ()); }

      let mut i: c_int = 0;
      while i < print_depth as c_int
	{
	  if BASE_EQ (obj, being_printed[i as usize])
	    {
	      let len: c_int = sprintf (buf.as_mut_ptr (), c"#%d".as_ptr (), i);
	      strout (buf.as_ptr (), len as ptrdiff_t, len as ptrdiff_t, printcharfun);
	      to_next_obj = true;
	      break;
	    }
	  i += 1;
	}
      if ! to_next_obj
	{ being_printed[print_depth as usize] = obj; }
    }
  else if print_circle_candidate_p (obj)
    {
      /* With the print-circle feature.  */
      let num: Lisp_Object = Fgethash (obj, Vprint_number_table, Qnil ());
      if FIXNUMP (num)
	{
	  let n: EMACS_INT = XFIXNUM (num);
	  if n < 0
	    { /* Add a prefix #n= if OBJ has not yet been printed;
		 that is, its status field is nil.  */
	      let len: c_int = sprintf (buf.as_mut_ptr (), c"#%ld=".as_ptr (), -n);
	      strout (buf.as_ptr (), len as ptrdiff_t, len as ptrdiff_t, printcharfun);
	      /* OBJ is going to be printed.  Remember that fact.  */
	      Fputhash (obj, make_fixnum (- n), Vprint_number_table);
	    }
	  else
	    {
	      /* Just print #n# if OBJ has already been printed.  */
	      let len: c_int = sprintf (buf.as_mut_ptr (), c"#%ld#".as_ptr (), n);
	      strout (buf.as_ptr (), len as ptrdiff_t, len as ptrdiff_t, printcharfun);
	      to_next_obj = true;
	    }
	}
      else if STRINGP (num)
	{
	  print_string_1 (num, printcharfun, false);
	  to_next_obj = true;
	}
    }

  if ! to_next_obj {
  print_depth += 1;

  match XTYPE (obj)
    {
    Lisp_Type::Lisp_Int0 | Lisp_Type::Lisp_Int1 => {
        let mut i: EMACS_INT = XFIXNUM (obj);
        let mut escaped_name: c_char = named_escape (i as c_int);

	if print_integers_as_characters && i >= 0 && i <= MAX_UNICODE_CHAR as EMACS_INT
            && (escaped_name != 0 || graphic_base_p (i as c_int))
	  {
	    printchar (b'?' as c_uint, printcharfun);
            if escaped_name != 0
              {
                printchar (b'\\' as c_uint, printcharfun);
                i = escaped_name as EMACS_INT;
              }
            else if escapeflag
                     && (i == b';' as EMACS_INT || i == b'\x22' as EMACS_INT
                         || i == b'\'' as EMACS_INT || i == b'\\' as EMACS_INT
                         || i == b'(' as EMACS_INT || i == b')' as EMACS_INT
                         || i == b'{' as EMACS_INT || i == b'}' as EMACS_INT
                         || i == b'[' as EMACS_INT || i == b']' as EMACS_INT)
	      { printchar (b'\\' as c_uint, printcharfun); }
	    printchar (i as c_uint, printcharfun);
	  }
	else
	  {
	    let end: *mut c_char = buf.as_mut_ptr ().add (BUF_SIZE);
	    let start: *mut c_char = fixnum_to_string (i, buf.as_mut_ptr (), end);
	    let len: ptrdiff_t = end.offset_from (start);
	    strout (start, len, len, printcharfun);
	  }
      }

    Lisp_Type::Lisp_Float => {
	let mut pigbuf: [c_char; FLOAT_TO_STRING_BUFSIZE] = [0; FLOAT_TO_STRING_BUFSIZE];
	let len: c_int = float_to_string (pigbuf.as_mut_ptr (), XFLOAT_DATA (obj));
	strout (pigbuf.as_ptr (), len as ptrdiff_t, len as ptrdiff_t, printcharfun);
      }

    Lisp_Type::Lisp_String => {
      if ! escapeflag
	{ print_string (obj, printcharfun); }
      else
	{
	  let mut i: ptrdiff_t;
	  let mut i_byte: ptrdiff_t;
	  let size_byte: ptrdiff_t;
	  /* True means we must ensure that the next character we output
	     cannot be taken as part of a hex character escape.	 */
	  let mut need_nonhex: bool = false;
	  let multibyte: bool = STRING_MULTIBYTE (obj);

	  if ! EQ (Vprint_charset_text_property, Qt ())
	    { obj = print_prune_string_charset (obj); }

	  if ! string_intervals (obj).is_null ()
	    { print_c_string (c"#(".as_ptr (), printcharfun); }

	  printchar (b'\x22' as c_uint, printcharfun);
	  size_byte = SBYTES (obj);

	  i = 0;
	  i_byte = 0;
	  while i_byte < size_byte
	    {
	      /* Here, we must convert each multi-byte form to the
		 corresponding character code before handing it to
		 printchar.  */
	      let mut c: c_int = fetch_string_char_advance (obj, &raw mut i, &raw mut i_byte);

	      maybe_quit ();

	      if if multibyte
		   { CHAR_BYTE8_P (c) && { c = CHAR_TO_BYTE8 (c); true } }
		 else
		   { SINGLE_BYTE_CHAR_P (c as intmax_t) && ! ASCII_CHAR_P (c as isize)
		       && print_escape_nonascii }
		{
		  /* When printing a raw 8-bit byte in a multibyte buffer, or
		     (when requested) a non-ASCII character in a unibyte buffer,
		     print single-byte non-ASCII string chars
		     using octal escapes.  */
		  octalout (c as u8, SDATA (obj), i_byte, size_byte, printcharfun);
		  need_nonhex = false;
		}
	      else if multibyte
		       && ! ASCII_CHAR_P (c as isize) && print_escape_multibyte
		{
		  /* When requested, print multibyte chars using
		     hex escapes.  */
		  let mut outbuf: [c_char; 3 + INT_STRLEN_BOUND (std::mem::size_of::<c_int> () * 8, 1)] =
		    [0; 3 + INT_STRLEN_BOUND (std::mem::size_of::<c_int> () * 8, 1)];
		  let len: c_int = sprintf (outbuf.as_mut_ptr (), c"\\x%04x".as_ptr (),
					    (c + 0) as c_uint);
		  strout (outbuf.as_ptr (), len as ptrdiff_t, len as ptrdiff_t,
			  printcharfun);
		  need_nonhex = true;
		}
	      else
		{
		  /* If we just had a hex escape, and this character
		     could be taken as part of it,
		     output `\ ' to prevent that.  */
		  if c_isxdigit (c)
		    {
		      if need_nonhex
			{ print_c_string (c"\\ ".as_ptr (), printcharfun); }
		      printchar (c as c_uint, printcharfun);
		    }
		  else if if c == b'\n' as c_int && print_escape_newlines
			    { c = b'n' as c_int; true }
			    else if c == b'\x0c' as c_int && print_escape_newlines
			    { c = b'f' as c_int; true }
			    else
			    { c == b'\x22' as c_int || c == b'\\' as c_int }
		    {
		      printchar (b'\\' as c_uint, printcharfun);
		      printchar (c as c_uint, printcharfun);
		    }
		  else if print_escape_control_characters && c_iscntrl (c)
		    { octalout (c as u8, SDATA (obj), i_byte, size_byte, printcharfun); }
		  else if ! multibyte
			   && SINGLE_BYTE_CHAR_P (c as intmax_t)
			   && ! ASCII_CHAR_P (c as isize)
		    { printchar (BYTE8_TO_CHAR (c) as c_uint, printcharfun); }
		  else
		    { printchar (c as c_uint, printcharfun); }
		  need_nonhex = false;
		}
	    }
	  printchar (b'\x22' as c_uint, printcharfun);

	  if ! string_intervals (obj).is_null ()
	    {
	      let mut pcf: Lisp_Object = printcharfun;
	      traverse_intervals (string_intervals (obj),
				  0, Some (print_interval), &raw mut pcf as *mut c_void);
	      printchar (b')' as c_uint, printcharfun);
	    }
	}
      }

    Lisp_Type::Lisp_Symbol => {
	let name: Lisp_Object = SYMBOL_NAME (obj);
	let size_byte: ptrdiff_t = SBYTES (name);

	let p: *mut c_char = SSDATA (name);
	let signedp: bool = *p == b'-' as c_char || *p == b'+' as c_char;
	let mut len: ptrdiff_t = 0;
	let mut confusing: bool =
	  /* Set CONFUSING if NAME looks like a number, calling
	     string_to_number for non-obvious cases.  */
	  ((c_isdigit (*p.offset (signedp as isize) as c_int)
	    || *p.offset (signedp as isize) == b'.' as c_char)
	   && ! NILP (string_to_number (p, 10, &raw mut len))
	   && len == size_byte)
	  /* We don't escape "?" unless it's the first character in the
	     symbol name.  */
	  || *p == b'?' as c_char
	  /* We don't escape "." unless it's the first character in the
	     symbol name; even then, we don't escape it if it's followed
	     by [a-zA-Z]. */
	  || (*p == b'.' as c_char
	      && ! (size_byte > 1 && c_isalpha (*p.offset (1) as c_int)));

	if ! NILP (Vprint_gensym)
	    && ! SYMBOL_INTERNED_IN_INITIAL_OBARRAY_P (obj)
	  { print_c_string (c"#:".as_ptr (), printcharfun); }
	else if size_byte == 0
	  { print_c_string (c"##".as_ptr (), printcharfun); }
	else
	  {
	    let mut i: ptrdiff_t = 0;
	    let mut i_byte: ptrdiff_t = 0;
	    while i_byte < size_byte
	      {
		/* Here, we must convert each multi-byte form to the
		   corresponding character code before handing it to PRINTCHAR.  */
		let c: c_int = fetch_string_char_as_multibyte_advance (name, &raw mut i,
								      &raw mut i_byte);
		maybe_quit ();

		if escapeflag
		  {
		    if c == b'\x22' as c_int || c == b'\\' as c_int || c == b'\'' as c_int
			|| c == b';' as c_int || c == b'#' as c_int || c == b'(' as c_int
			|| c == b')' as c_int
			|| c == b',' as c_int || c == b'`' as c_int
			|| c == b'[' as c_int || c == b']' as c_int || c <= 0o40
			|| c == NO_BREAK_SPACE
			|| confusing
		      {
			printchar (b'\\' as c_uint, printcharfun);
			confusing = false;
		      }
		  }
		printchar (c as c_uint, printcharfun);
	      }
	  }
      }

    Lisp_Type::Lisp_Cons => {
      /* If deeper than spec'd depth, print placeholder.  */
      if FIXNUMP (Vprint_level)
	  && print_depth > XFIXNUM (Vprint_level)
	{ print_c_string (c"...".as_ptr (), printcharfun); }
      else if print_quoted && CONSP (XCDR (obj)) && NILP (XCDR (XCDR (obj)))
	       && EQ (XCAR (obj), Qquote ())
	{
	  printchar (b'\'' as c_uint, printcharfun);
	  obj = XCAR (XCDR (obj));
	  print_depth -= 1;	/* tail recursion */
	  continue 'print_obj;
	}
      else if print_quoted && CONSP (XCDR (obj)) && NILP (XCDR (XCDR (obj)))
	       && EQ (XCAR (obj), Qfunction ())
	{
	  print_c_string (c"#'".as_ptr (), printcharfun);
	  obj = XCAR (XCDR (obj));
	  print_depth -= 1;	/* tail recursion */
	  continue 'print_obj;
	}
      /* FIXME: Do we really need the new_backquote_output gating of
	 special syntax for comma and comma-at?  There is basically no
	 benefit from it at all, and it would be nice to get rid of
	 the recursion here without additional complexity.  */
      else if print_quoted && CONSP (XCDR (obj)) && NILP (XCDR (XCDR (obj)))
	       && EQ (XCAR (obj), Qbackquote ())
	{
	  printchar (b'`' as c_uint, printcharfun);
	  new_backquote_output += 1;
	  print_object (XCAR (XCDR (obj)), printcharfun, escapeflag);
	  new_backquote_output -= 1;
	}
      else if print_quoted && CONSP (XCDR (obj)) && NILP (XCDR (XCDR (obj)))
	       && (EQ (XCAR (obj), Qcomma ())
		   || EQ (XCAR (obj), Qcomma_at ()))
	       && new_backquote_output != 0
	{
	  print_object (XCAR (obj), printcharfun, false);
	  new_backquote_output -= 1;
	  print_object (XCAR (XCDR (obj)), printcharfun, escapeflag);
	  new_backquote_output += 1;
	}
      else
	{
	  printchar (b'(' as c_uint, printcharfun);
	  /* Negative values of print-length are invalid in CL.
	     Treat them like nil, as CMUCL does.  */
	  let print_length: intmax_t = if FIXNATP (Vprint_length)
	    { XFIXNAT (Vprint_length) as intmax_t } else { INTMAX_MAX };
	  if print_length == 0
	    { print_c_string (c"...)".as_ptr (), printcharfun); }
	  else
	    {
	      print_stack_push (print_stack_entry {
		  r#type: print_entry_type::PE_list,
		  u: print_stack_entry_u { list: print_stack_entry_list {
		    last: obj,
		    maxlen: print_length,
		    tortoise: obj,
		    n: 2,
		    m: 2,
		    tortoise_idx: 0,
		  } },
		});
	      /* print the car */
	      obj = XCAR (obj);
	      continue 'print_obj;
	    }
	}
      }

    Lisp_Type::Lisp_Vectorlike => {
      /* First do all the vectorlike types that have a readable syntax.  */
      match PSEUDOVECTOR_TYPE (XVECTOR (obj))
	{
	pvec_type::PVEC_NORMAL_VECTOR => {
	  print_stack_push_vector (c"[".as_ptr (), c"]".as_ptr (), obj, 0, ASIZE (obj),
				   printcharfun);
	  to_next_obj = true;
	}
	pvec_type::PVEC_RECORD => {
	  print_stack_push_vector (c"#s(".as_ptr (), c")".as_ptr (), obj, 0, PVSIZE (obj),
				   printcharfun);
	  to_next_obj = true;
	}
	pvec_type::PVEC_CLOSURE => {
	  print_stack_push_vector (c"#[".as_ptr (), c"]".as_ptr (), obj, 0, PVSIZE (obj),
				   printcharfun);
	  to_next_obj = true;
	}
	pvec_type::PVEC_CHAR_TABLE => {
	  print_stack_push_vector (c"#^[".as_ptr (), c"]".as_ptr (), obj, 0, PVSIZE (obj),
				   printcharfun);
	  to_next_obj = true;
	}
	pvec_type::PVEC_SUB_CHAR_TABLE => {
	    print_c_string (c"#^^[".as_ptr (), printcharfun);
	    let n: c_int = sprintf (buf.as_mut_ptr (), c"%d %d".as_ptr (),
				    (*XSUB_CHAR_TABLE (obj)).depth,
				    (*XSUB_CHAR_TABLE (obj)).min_char);
	    strout (buf.as_ptr (), n as ptrdiff_t, n as ptrdiff_t, printcharfun);
	    print_stack_push_vector (c"".as_ptr (), c"]".as_ptr (), obj,
				     SUB_CHAR_TABLE_OFFSET, PVSIZE (obj),
				     printcharfun);
	    to_next_obj = true;
	}
	pvec_type::PVEC_HASH_TABLE => {
	    let h: *mut Lisp_Hash_Table = XHASH_TABLE (obj);
	    /* Implement a readable output, e.g.:
	       #s(hash-table test equal data (k1 v1 k2 v2)) */
	    print_c_string (c"#s(hash-table".as_ptr (), printcharfun);

	    if ! BASE_EQ ((*(*h).test).name, Qeql ())
	      {
		print_c_string (c" test ".as_ptr (), printcharfun);
		print_object ((*(*h).test).name, printcharfun, escapeflag);
	      }

	    if hash_table_bits! (weakness, h) != Weak_None
	      {
		print_c_string (c" weakness ".as_ptr (), printcharfun);
		print_object (hash_table_weakness_symbol (hash_table_bits! (weakness, h)),
			      printcharfun, escapeflag);
	      }

	    let mut size: ptrdiff_t = (*h).count as ptrdiff_t;
	    if size > 0
	      {
		print_c_string (c" data (".as_ptr (), printcharfun);

		/* Don't print more elements than the specified maximum.  */
		if FIXNATP (Vprint_length)
		    && XFIXNAT (Vprint_length) < size as EMACS_INT
		  { size = XFIXNAT (Vprint_length) as ptrdiff_t; }

		print_stack_push (print_stack_entry {
		    r#type: print_entry_type::PE_hash,
		    u: print_stack_entry_u { hash: print_stack_entry_hash {
		      obj: obj,
		      nobjs: size * 2,
		      idx: 0,
		      printed: 0,
		      truncated: size < (*h).count as ptrdiff_t,
		    } },
		  });
	      }
	    else
	      {
		/* Empty table: we can omit the data entirely.  */
		printchar (b')' as c_uint, printcharfun);
		print_depth -= 1;   /* Done with this.  */
	      }
	    to_next_obj = true;
	}
	pvec_type::PVEC_BIGNUM => {
	  print_bignum (obj, printcharfun);
	}
	pvec_type::PVEC_BOOL_VECTOR => {
	  print_bool_vector (obj, printcharfun);
	}
	_ => {
	  print_vectorlike_unreadable (obj, printcharfun, escapeflag, buf.as_mut_ptr ());
	}
	}
      }

    _ => { emacs_abort (); }
    }
  if ! to_next_obj
    { print_depth -= 1; }
  }

 'next_obj: loop {
  if prstack.sp > base_sp
    {
      /* Handle a continuation on the print stack.  */
      let e: *mut print_stack_entry = prstack.stack.offset (prstack.sp - 1);
      match (*e).r#type
	{
	print_entry_type::PE_list => {
	    /* after "(" ELEM (* " " ELEM) */
	    let next: Lisp_Object = XCDR ((*e).u.list.last);
	    if NILP (next)
	      {
		/* end of list: print ")" */
		printchar (b')' as c_uint, printcharfun);
		prstack.sp -= 1;
		print_depth -= 1;
		continue 'next_obj;
	      }
	    else if CONSP (next)
	      {
		if ! NILP (Vprint_circle)
		  {
		    /* With the print-circle feature.  */
		    let num: Lisp_Object = Fgethash (next, Vprint_number_table,
						     Qnil ());
		    if ! (NILP (num) || EQ (num, Qt ()))
		      {
			print_c_string (c" . ".as_ptr (), printcharfun);
			obj = next;
			(*e).r#type = print_entry_type::PE_rbrac;
			continue 'print_obj;
		      }
		  }

		/* list continues: print " " ELEM ... */

		printchar (b' ' as c_uint, printcharfun);

		(*e).u.list.maxlen -= 1;
		if (*e).u.list.maxlen <= 0
		  {
		    print_c_string (c"...)".as_ptr (), printcharfun);
		    prstack.sp -= 1;
		    print_depth -= 1;
		    continue 'next_obj;
		  }

		(*e).u.list.last = next;
		(*e).u.list.n -= 1;
		if (*e).u.list.n == 0
		  {
		    /* Double tortoise update period and teleport it.  */
		    (*e).u.list.tortoise_idx += (*e).u.list.m as intmax_t;
		    (*e).u.list.m <<= 1;
		    (*e).u.list.n = (*e).u.list.m;
		    (*e).u.list.tortoise = next;
		  }
		else if BASE_EQ (next, (*e).u.list.tortoise)
		  {
		    /* FIXME: This #N tail index is somewhat ambiguous;
		       see bug#55395.  */
		    let len: c_int = sprintf (buf.as_mut_ptr (), c". #%jd)".as_ptr (),
					      (*e).u.list.tortoise_idx);
		    strout (buf.as_ptr (), len as ptrdiff_t, len as ptrdiff_t, printcharfun);
		    prstack.sp -= 1;
		    print_depth -= 1;
		    continue 'next_obj;
		  }
		obj = XCAR (next);
	      }
	    else
	      {
		/* non-nil ending: print " . " ELEM ")" */
		print_c_string (c" . ".as_ptr (), printcharfun);
		obj = next;
		(*e).r#type = print_entry_type::PE_rbrac;
	      }
	}

	print_entry_type::PE_rbrac => {
	  printchar (b')' as c_uint, printcharfun);
	  prstack.sp -= 1;
	  print_depth -= 1;
	  continue 'next_obj;
	}

	print_entry_type::PE_vector => {
	  if (*e).u.vector.idx >= (*e).u.vector.size
	    {
	      if (*e).u.vector.truncated
		{
		  if (*e).u.vector.idx > 0
		    { printchar (b' ' as c_uint, printcharfun); }
		  print_c_string (c"...".as_ptr (), printcharfun);
		}
	      print_c_string ((*e).u.vector.end, printcharfun);
	      prstack.sp -= 1;
	      print_depth -= 1;
	      continue 'next_obj;
	    }
	  if (*e).u.vector.idx > 0
	    { printchar (b' ' as c_uint, printcharfun); }
	  obj = AREF ((*e).u.vector.obj, (*e).u.vector.idx);
	  (*e).u.vector.idx += 1;
	}

	print_entry_type::PE_hash => {
	  if (*e).u.hash.printed >= (*e).u.hash.nobjs
	    {
	      if (*e).u.hash.truncated
		{
		  if (*e).u.hash.printed != 0
		    { printchar (b' ' as c_uint, printcharfun); }
		  print_c_string (c"...".as_ptr (), printcharfun);
		}
	      print_c_string (c"))".as_ptr (), printcharfun);
	      prstack.sp -= 1;
	      print_depth -= 1;
	      continue 'next_obj;
	    }

	  if (*e).u.hash.printed != 0
	    { printchar (b' ' as c_uint, printcharfun); }

	  let h: *mut Lisp_Hash_Table = XHASH_TABLE ((*e).u.hash.obj);
	  if ((*e).u.hash.printed & 1) == 0
	    {
	      let mut key: Lisp_Object;
	      let mut idx: ptrdiff_t = (*e).u.hash.idx;
	      loop
		{
		  key = HASH_KEY (h, idx as hash_idx_t);
		  if ! hash_unused_entry_key_p (key)
		    { break; }
		  idx += 1;
		}
	      (*e).u.hash.idx = idx;
	      obj = key;
	    }
	  else
	    {
	      obj = HASH_VALUE (h, (*e).u.hash.idx as hash_idx_t);
	      (*e).u.hash.idx += 1;
	    }
	  (*e).u.hash.printed += 1;
	}
	}
      continue 'print_obj;
    }
  break;
  }
  }
  eassert! (print_depth == base_depth);
}

/* Print a description of INTERVAL using PRINTCHARFUN.
   This is part of printing a string that has text properties.  */

unsafe extern "C" fn print_interval (interval: INTERVAL, pprintcharfun: *mut c_void)
{
  let interval: *mut interval = interval as *mut interval;   /* P-72 rule 4 */
  if NILP ((*interval).plist)
    { return; }
  let printcharfun: Lisp_Object = *(pprintcharfun as *mut Lisp_Object);
  printchar (b' ' as c_uint, printcharfun);
  print_object (make_fixnum ((*interval).position), printcharfun, true);
  printchar (b' ' as c_uint, printcharfun);
  print_object (make_fixnum ((*interval).position + LENGTH! (interval)),
		printcharfun, true);
  printchar (b' ' as c_uint, printcharfun);
  print_object ((*interval).plist, printcharfun, true);
}

/* Initialize debug_print stuff early to have it working from the very
   beginning.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_print_once ()
{
  /* The subroutine object for external-debugging-output is kept here
     for the convenience of the debugger.  */
  DEFSYM! (Qexternal_debugging_output, "external-debugging-output");

  defsubr (&raw mut Sexternal_debugging_output);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_print ()
{
  DEFSYM! (Qtemp_buffer_setup_hook, "temp-buffer-setup-hook");

  DEFVAR_LISP! (c"standard-output", Vstandard_output,
	       "Output stream `print' uses by default for outputting a character.\nThis may be any function of one argument.\nIt may also be a buffer (output is inserted before point)\nor a marker (output is inserted and the marker is advanced)\nor the symbol t (output appears in the echo area).");
  Vstandard_output = Qt ();
  DEFSYM! (Qstandard_output, "standard-output");

  DEFVAR_LISP! (c"float-output-format", Vfloat_output_format,
	       "The format descriptor string used to print floats.\nThis is a %-spec like those accepted by `printf' in C,\nbut with some restrictions.  It must start with the two characters `%.'.\nAfter that comes an integer precision specification,\nand then a letter which controls the format.\nThe letters allowed are `e', `f' and `g'.\nUse `e' for exponential notation \"DIG.DIGITSeEXPT\"\nUse `f' for decimal point notation \"DIGITS.DIGITS\".\nUse `g' to choose the shorter of those two formats for the number at hand.\nThe precision in any of these cases is the number of digits following\nthe decimal point.  With `f', a precision of 0 means to omit the\ndecimal point.  0 is not allowed with `e' or `g'.\n\nA value of nil means to use the shortest notation\nthat represents the number without losing information.");
  Vfloat_output_format = Qnil ();
  DEFSYM! (Qfloat_output_format, "float-output-format");

  DEFVAR_BOOL! (c"print-integers-as-characters", print_integers_as_characters,
	       "Non-nil means integers are printed using characters syntax.\nOnly independent graphic characters, and control characters with named\nescape sequences such as newline, are printed this way.  Other\nintegers, including those corresponding to raw bytes, are printed\nas numbers the usual way.");
  print_integers_as_characters = false;

  DEFVAR_LISP! (c"print-length", Vprint_length,
	       "Maximum length of list to print before abbreviating.\nA value of nil means no limit.  See also `eval-expression-print-length'.");
  Vprint_length = Qnil ();

  DEFVAR_LISP! (c"print-level", Vprint_level,
	       "Maximum depth of list nesting to print before abbreviating.\nA value of nil means no limit.  See also `eval-expression-print-level'.");
  Vprint_level = Qnil ();

  DEFVAR_BOOL! (c"print-escape-newlines", print_escape_newlines,
	       "Non-nil means print newlines in strings as `\\n'.\nAlso print formfeeds as `\\f'.");
  print_escape_newlines = false;

  DEFVAR_BOOL! (c"print-escape-control-characters", print_escape_control_characters,
	       "Non-nil means print control characters in strings as `\\OOO'.\n(OOO is the octal representation of the character code.)");
  print_escape_control_characters = false;

  DEFVAR_BOOL! (c"print-escape-nonascii", print_escape_nonascii,
	       "Non-nil means print unibyte non-ASCII chars in strings as \\OOO.\n(OOO is the octal representation of the character code.)\nOnly single-byte characters are affected, and only in `prin1'.\nWhen the output goes in a multibyte buffer, this feature is\nenabled regardless of the value of the variable.");
  print_escape_nonascii = false;

  DEFVAR_BOOL! (c"print-escape-multibyte", print_escape_multibyte,
	       "Non-nil means print multibyte characters in strings as \\xXXXX.\n(XXXX is the hex representation of the character code.)\nThis affects only `prin1'.");
  print_escape_multibyte = false;

  DEFVAR_BOOL! (c"print-quoted", print_quoted,
	       "Non-nil means print quoted forms with reader syntax.\nI.e., (quote foo) prints as \\='foo, (function foo) as #\\='foo.");
  print_quoted = true;

  DEFVAR_LISP! (c"print-gensym", Vprint_gensym,
	       "Non-nil means print uninterned symbols so they will read as uninterned.\nI.e., the value of (make-symbol \"foobar\") prints as #:foobar.\nWhen the uninterned symbol appears multiple times within the printed\nexpression, and `print-circle' is non-nil, in addition use the #N#\nand #N= constructs as needed, so that multiple references to the same\nsymbol are shared once again when the text is read back.");
  Vprint_gensym = Qnil ();

  DEFVAR_LISP! (c"print-circle", Vprint_circle,
	       "Non-nil means print recursive structures using #N= and #N# syntax.\nIf nil, printing proceeds recursively and may lead to\n`max-lisp-eval-depth' being exceeded or an error may occur:\n\"Apparently circular structure being printed.\"  Also see\n`print-length' and `print-level'.\nIf non-nil, shared substructures anywhere in the structure are printed\nwith `#N=' before the first occurrence (in the order of the print\nrepresentation) and `#N#' in place of each subsequent occurrence,\nwhere N is a positive decimal integer.");
  Vprint_circle = Qnil ();

  DEFVAR_LISP! (c"print-continuous-numbering", Vprint_continuous_numbering,
	       "Non-nil means number continuously across print calls.\nThis affects the numbers printed for #N= labels and #M# references.\nSee also `print-circle', `print-gensym', and `print-number-table'.\nThis variable should not be set with `setq'; bind it with a `let' instead.");
  Vprint_continuous_numbering = Qnil ();

  DEFVAR_LISP! (c"print-number-table", Vprint_number_table,
	       "A vector used internally to produce `#N=' labels and `#N#' references.\nThe Lisp printer uses this vector to detect Lisp objects referenced more\nthan once.  If an entry contains a number, then the corresponding key is\nreferenced more than once: a positive sign indicates that it's already been\nprinted, and the absolute value indicates the number to use when printing.\nIf an entry contains a string, that string is printed instead.\n\nWhen you bind `print-continuous-numbering' to t, you should probably\nalso bind `print-number-table' to nil.  This ensures that the value of\n`print-number-table' can be garbage-collected once the printing is\ndone.  If all elements of `print-number-table' are nil, it means that\nthe printing done so far has not found any shared structure or objects\nthat need to be recorded in the table.");
  Vprint_number_table = Qnil ();

  DEFVAR_LISP! (c"print-charset-text-property", Vprint_charset_text_property,
	       "A flag to control printing of `charset' text property on printing a string.\nThe value should be nil, t, or `default'.\n\nIf the value is nil, don't print the text property `charset'.\n\nIf the value is t, always print the text property `charset'.\n\nIf the value is `default', print the text property `charset' only when\nthe value is different from what is guessed in the current charset\npriorities.  Values other than nil or t are also treated as\n`default'.");
  Vprint_charset_text_property = Qdefault ();

  DEFVAR_BOOL! (c"print-symbols-bare", print_symbols_bare,
               "A flag to control printing of symbols with position.\nIf the value is nil, print these objects complete with position.\nOtherwise print just the bare symbol.");
  print_symbols_bare = false;
  DEFSYM! (Qprint_symbols_bare, "print-symbols-bare");

  /* prin1_to_string_buffer initialized in init_buffer_once in buffer.c */
  staticpro (&raw mut Vprin1_to_string_buffer);

  defsubr (&raw mut Sprin1);
  defsubr (&raw mut Sprin1_to_string);
  defsubr (&raw mut Serror_message_string);
  defsubr (&raw mut Sprinc);
  defsubr (&raw mut Sprint);
  defsubr (&raw mut Sterpri);
  defsubr (&raw mut Swrite_char);
  defsubr (&raw mut Sredirect_debugging_output);
  defsubr (&raw mut Sprint_preprocess);

  DEFSYM! (Qprint_escape_multibyte, "print-escape-multibyte");
  DEFSYM! (Qprint_escape_nonascii, "print-escape-nonascii");

  print_prune_charset_plist = Qnil ();
  staticpro (&raw mut print_prune_charset_plist);

  DEFVAR_LISP! (c"print-unreadable-function", Vprint_unreadable_function,
	       "If non-nil, a function to call when printing unreadable objects.\nBy default, Emacs printing functions (like `prin1') print unreadable\nobjects as \"#<...>\", where \"...\" describes the object (for\ninstance, \"#<marker in no buffer>\").\n\nIf non-nil, it should be a function that will be called with two\narguments: the object to be printed, and the NOESCAPE flag (see\n`prin1-to-string').  If this function returns nil, the object will be\nprinted as usual.  If it returns a string, that string will then be\nprinted.  If the function returns anything else, the object will not\nbe printed.");
  Vprint_unreadable_function = Qnil ();
  DEFSYM! (Qprint_unreadable_function, "print-unreadable-function");

  DEFVAR_LISP! (c"print--unreadable-callback-buffer",
	       Vprint__unreadable_callback_buffer,
	       "Dynamically bound to indicate current buffer.");
  Vprint__unreadable_callback_buffer = Qnil ();
  DEFSYM! (Qprint__unreadable_callback_buffer,
	  "print--unreadable-callback-buffer");
  /* Don't export this variable to Elisp.  */
  Funintern (Qprint__unreadable_callback_buffer (), Qnil ());

  defsubr (&raw mut Sflush_standard_output);

  /* Initialized in print_create_variable_mapping.  */
  staticpro (&raw mut Vprint_variable_mapping);
}
