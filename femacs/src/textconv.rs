// femacs: port of src/textconv.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The C non-`static` definitions carry `#[unsafe(no_mangle)]` and `extern "C"` (PORTING.md
// 0.2); the C `static` helpers stay plain `unsafe fn`s, except the four whose address C hands to
// `record_unwind_protect`/`record_unwind_protect_ptr` (`restore_selected_window`, `complete_edit`,
// `complete_edit_check`, `decrement_inside`), which are spelled `unsafe extern "C" fn` (PORTING.md
// P-12: one definition per linkage unit, which is what C's `static` gives too).
//
// **`src/textconv.c` is a P-86 unit: the frozen configuration compiles nothing from it.**
// `textconv.o` joins the build only through `WINDOW_SYSTEM_OBJ` (`emacs/src/Makefile.in` 304/468),
// and `configure.ac` 7521-7526 appends it under `test "$window_system" = "x11" ||
// test "$REALLY_ANDROID" = "yes"` - the same `if` that `AC_DEFINE`s `HAVE_TEXT_CONVERSION` (7524).
// The frozen target has no window system at all: `build/emacs-ref/src/Makefile` 304 is
// `WINDOW_SYSTEM_OBJ=` (empty), `docs/baseline/config/darwin-arm64.macros.tsv` 1645/1678 record
// `HAVE_TEXT_CONVERSION` and `HAVE_WINDOW_SYSTEM` as `undef`, and
// `docs/baseline/frozen-build-sources.txt` lists no `textconv.o`.  So the file gate is
// `feature = "have_text_conversion"` (registered, `femacs/xtask/Cargo.toml` 332), it goes on every
// item **and on the include graph** (P-86 8 - that is what makes the frozen `build-tu` reading an
// empty object), and P-86 1 applies: this is *transcription*, and both arms of every `#if` are
// carried.
//
// The file's own conditionals beyond the gate are three, all carried with both arms:
//   * C 45's `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` around C 46-53's `#if 0`, whose
//     body is `#include <android/log.h>` and the `__android_log_print` spelling of
//     `TEXTCONV_DEBUG`.  `#if 0` is dead in *every* configuration, so the region is recorded at its
//     position rather than transcribed (the `src/androidfns.rs` 1015-1022 shape).
//     `HAVE_ANDROID` is `undef` (macros.tsv 1240) and `ANDROID_STUBIFY` is `undef` (macros.tsv 78),
//     so the arm the port would compile is `all (feature = "have_text_conversion",
//     feature = "have_android", feature = "not_android_stubify")`; the guard is named at the
//     recorded text so the dead arm stays visible.
//   * C 55's `#ifndef TEXTCONV_DEBUG`, whose live arm is the `((void) 0)` fallback (C's other arm
//     is the `#if 0` body above).  The fallback is a `macro_rules!` under
//     `not (feature = "textconv_debug")`; C's arguments are *not* evaluated, and the port's macro
//     does not evaluate them either (PORTING.md 2.1).  `textconv_debug` is a name this lane
//     introduces and cannot register itself (`## Owed` 2).
//   * C 1742-2312's `#ifdef HAVE_ANDROID`: the whole Android surface (the eleven action-queueing
//     entry points, `get_extracted_text`, `get_surrounding_text`, `get_conversion_field`).  Every
//     item under it carries `all (feature = "have_text_conversion", feature = "have_android")`.
//
// **`textconv.h` is not `include!`d.**  Its mirror (`femacs/src/include/textconv.rs`) declares
// `register_textconv_interface` at its top level (the mirror's C 161 sits *outside* its
// `textconv_decls` module), and this unit *defines* that function: a top-level declaration plus a
// definition of one name in one crate is `E0428` (measured on this checkout), so the unit carries
// the header's three records, two enums and one constant in place (P-88 1's shape, `## Deviations`
// 1) and drops the function prototypes (0.1-12: a definition needs no declaration in Rust).
// `## Owed` 1 names the mirror change - moving that one declaration into `textconv_decls` - that
// would let this unit `include!` the mirror instead.
//
// Names the mirrors this unit includes do not carry are declared at the use site with the
// reference header's line, one citation per line.  Each line below ends with its own range, so the
// reading `check-citations.py` takes pairs a name with its own line:
//   * `record_unwind_protect_excursion' - lisp.h 4826;
//   * `save_restriction_save' and `save_restriction_restore' - lisp.h 4924-4925;
//   * `insert_from_string' - lisp.h 4335-4336;
//   * `del_range_1' - lisp.h 4346;
//   * `safe_del_range' - lisp.h 4351;
//   * `find_field' - lisp.h 4920-4921;
//   * `reading_key_sequence' - keyboard.h 252.
// `## Owed` 3-5 name the mirrors that should carry them.  `TYPE_MINIMUM` is `<intprops.h>` 68-69's macro, which the frozen
// `lib/include/intprops.rs` keeps as text rather than code, so the unit writes the Rust primitive
// at the use site (the `src/androidselect.rs` 113-115 shape).
//
// `struct window`'s `ephemeral_last_point` (window.h 302, under `#ifdef HAVE_TEXT_CONVERSION`) is
// not in `include/window.rs` (`window.rs` 11 records it as the pending field), so the port reaches
// it through a named accessor macro at the one place C's layout allows - `## Deviations` 2 and
// `## Owed` 6.
//
// `docs/ports/textconv.md` carries the C line ranges, the acceptance readings, every deviation and
// the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons, overflowing_literals,
         unused_must_use, let_underscore_drop, clippy::all)]

include!("include/config.rs");                 // C 33: `#include <config.h>` (every unit includes it)

/* C 20-31: "String conversion support." - the C comment block, carried as prose:

   Many input methods require access to text surrounding the cursor.
   They may then request that the text editor remove or substitute
   that text for something else, for example when providing the
   ability to ``undo'' or ``edit'' previously composed text.  This is
   most commonly seen in input methods for CJK languages for X Windows,
   and is extensively used throughout Android by input methods for all
   kinds of scripts.

   In addition, these input methods may also need to make detailed
   edits to the content of a buffer.  That is also handled here.  */

/* C 35-39: `textconv.h`, `buffer.h`, `syntax.h`, `blockinput.h`, `keyboard.h`.  The file gate goes
   on every edge (P-86 8): the frozen build's empty object is the criterion, and an include outside
   the gate fails it at once.  The first include carries `textconv.h`'s own chain (`lisp.h` and
   `frame.h`, C 22-23 of that header -> `window.rs` and `buffer.rs` follow `frame.rs`), then the
   header's own surface (see the note above).  */
#[cfg (feature = "have_text_conversion")]
include!("include/lisp.rs");                   // textconv.h 22: `#include "lisp.h"`
#[cfg (feature = "have_text_conversion")]
include!("include/frame.rs");                  // textconv.h 23: `#include "frame.h"`
#[cfg (feature = "have_text_conversion")]
include!("include/buffer.rs");                 // C 36: `#include "buffer.h"`
#[cfg (feature = "have_text_conversion")]
include!("include/syntax.rs");                 // C 37: `#include "syntax.h"`
#[cfg (feature = "have_text_conversion")]
include!("include/blockinput.rs");             // C 38: `#include "blockinput.h"`
#[cfg (feature = "have_text_conversion")]
include!("include/keyboard.rs");               // C 39: `#include "keyboard.h"`

/* textconv.h 25-52: "The function pointers in this structure should be filled out by each GUI
   backend interested in supporting text conversion."  Measured (the mirror's own clang probe):
   sizeof 32, alignof 8, offsets 0/8/16/24.  */
#[cfg (feature = "have_text_conversion")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct textconv_interface
{
  /* Notice that the text conversion context has changed (which can
     happen if the window is deleted or switches buffers, or an
     unexpected buffer change occurs.) */
  /// C: textconv.h 36: `void (*reset) (struct frame *);`
  pub reset: Option<unsafe extern "C" fn (*mut frame)>,

  /* Notice that point or mark has moved in the specified frame's
     selected window's selected buffer.  The second argument is the
     window whose point changed, and the third argument is the
     buffer.  */
  /// C: textconv.h 42-43: `void (*point_changed) (struct frame *, struct window *, struct buffer *);`
  /// (`struct window`/`struct buffer` are pointers only here, and this header includes neither,
  /// so both are `*mut c_void` with C's prototype above - PORTING.md P-55/P-72 rule 3.)
  pub point_changed: Option<unsafe extern "C" fn (*mut frame, *mut c_void, *mut c_void)>,

  /* Notice that the preconversion region has changed without point
     being moved.  */
  /// C: textconv.h 47: `void (*compose_region_changed) (struct frame *);`
  pub compose_region_changed: Option<unsafe extern "C" fn (*mut frame)>,

  /* Notice that an asynch conversion identified by COUNTER has
     completed.  */
  /// C: textconv.h 51: `void (*notify_conversion) (unsigned long);` - `unsigned long` is `c_ulong`.
  pub notify_conversion: Option<unsafe extern "C" fn (c_ulong)>,
}

#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::size_of::<textconv_interface> () == 32);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::align_of::<textconv_interface> () == 8);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (textconv_interface, reset) == 0);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (textconv_interface, point_changed) == 8);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (textconv_interface, compose_region_changed) == 16);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (textconv_interface, notify_conversion) == 24);

/* textconv.h 56-69: `enum textconv_caret_direction` - no explicit values, so the members are
   declaration order and the port spells them as constants over C's own type name (PORTING.md 1.3).
   4 bytes each.  */
#[cfg (feature = "have_text_conversion")]
pub type textconv_caret_direction = c_int;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_FORWARD_CHAR: textconv_caret_direction = 0;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_BACKWARD_CHAR: textconv_caret_direction = 1;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_FORWARD_WORD: textconv_caret_direction = 2;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_BACKWARD_WORD: textconv_caret_direction = 3;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_CARET_UP: textconv_caret_direction = 4;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_CARET_DOWN: textconv_caret_direction = 5;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_NEXT_LINE: textconv_caret_direction = 6;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_PREVIOUS_LINE: textconv_caret_direction = 7;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_LINE_START: textconv_caret_direction = 8;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_LINE_END: textconv_caret_direction = 9;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_ABSOLUTE_POSITION: textconv_caret_direction = 10;

/* textconv.h 71-75: `enum textconv_operation`.  */
#[cfg (feature = "have_text_conversion")]
pub type textconv_operation = c_int;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_SUBSTITUTION: textconv_operation = 0;
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_RETRIEVAL: textconv_operation = 1;

/* textconv.h 77-88: "Structure describing text in a buffer corresponding to a ``struct
   textconv_callback_struct''."  Measured: sizeof 24, alignof 8, offsets 0/8/16.  */
#[cfg (feature = "have_text_conversion")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct textconv_conversion_text
{
  /* Length of the text in characters and bytes.  */
  /// C: textconv.h 83: `ptrdiff_t length, bytes;`
  pub length: ptrdiff_t,
  pub bytes: ptrdiff_t,

  /* Pointer to the text data.  This must be deallocated by the
     caller.  */
  /// C: textconv.h 87: `char *text;`
  pub text: *mut c_char,
}

/* textconv.h 90-119: "Structure describing a single query submitted by the input method."
   Measured: sizeof 48, alignof 8, offsets 0/8/12/16/24.  */
#[cfg (feature = "have_text_conversion")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct textconv_callback_struct
{
  /* Character position, relative to the current spot location, from
     where on text should be returned.  */
  /// C: textconv.h 97: `EMACS_INT position;`
  pub position: EMACS_INT,

  /* The type of scanning to perform to determine either the start or
     the end of the conversion.  */
  /// C: textconv.h 101: `enum textconv_caret_direction direction;`
  pub direction: textconv_caret_direction,

  /* The number of times for which to repeat the scanning in order to
     determine the starting position of the text to return.  */
  /// C: textconv.h 105: `unsigned short factor;`
  pub factor: c_ushort,

  /* The operation to perform upon the current buffer contents.

     If this is TEXTCONV_SUBSTITUTION, then the text that is returned
     will be deleted from the buffer itself.

     Otherwise, the text is simply returned without modifying the
     buffer contents.  */
  /// C: textconv.h 114: `enum textconv_operation operation;`
  pub operation: textconv_operation,

  /* Structure that will be filled with a description of the resulting
     text.  */
  /// C: textconv.h 118: `struct textconv_conversion_text text;` - this header's own record, by value.
  pub text: textconv_conversion_text,
}

#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::size_of::<textconv_callback_struct> () == 48);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::align_of::<textconv_callback_struct> () == 8);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (textconv_callback_struct, position) == 0);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (textconv_callback_struct, direction) == 8);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (textconv_callback_struct, factor) == 12);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (textconv_callback_struct, operation) == 16);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (textconv_callback_struct, text) == 24);

/// C: textconv.h 123: `#define TEXTCONV_SKIP_CONVERSION_REGION (1 << 0)` - the value is compared
/// as C's token pair (PORTING.md P-44's expression rule); C uses the name as a value, so it is a
/// constant rather than a `macro_rules!`.
#[cfg (feature = "have_text_conversion")]
pub const TEXTCONV_SKIP_CONVERSION_REGION: c_int = 1 << 0;

/* C 43: "Define debugging macros."  */

/* C 45-53: `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` (the arm
   `all (feature = "have_text_conversion", feature = "have_android",
   feature = "not_android_stubify")`) around C 46-52's `#if 0`, whose body is `#include
   <android/log.h>` and `#define TEXTCONV_DEBUG(fmt, ...) __android_log_print (ANDROID_LOG_VERBOSE,
   "EmacsInputConnection", "%s: " fmt, __func__, ## __VA_ARGS__)`.  `#if 0` is dead in every
   configuration (the `src/androidfns.rs` 1015-1022 shape), so the region is recorded, not
   transcribed; the other arm of the outer `#if` is empty.  */

/* C 55-57: `#ifndef TEXTCONV_DEBUG`.  C's live arm is the fallback below (the definition above is
   inside `#if 0`), and C's other arm - the macro already defined - is the dead text just recorded.
   The macro does **not** evaluate its arguments, exactly as C's `((void) 0)` does not.  */
#[cfg (not (feature = "textconv_debug"))]
macro_rules! TEXTCONV_DEBUG { ($($arg:tt)*) => { () } }

/* C 61-64: "The window system's text conversion interface.  NULL when the
   window system has not set up text conversion."  */
#[cfg (feature = "have_text_conversion")]
static mut text_interface: *mut textconv_interface = std::ptr::null_mut ();

/* C 66-68: "How many times text conversion has been disabled."  */
#[cfg (feature = "have_text_conversion")]
static mut suppress_conversion_count: c_int = 0;

/* C 70-77: "Flags used to determine what must be sent after a batch edit ends."  Explicit values,
   so the members are the values C wrote, over C's own type name.  */
#[cfg (feature = "have_text_conversion")]
pub type textconv_batch_edit_flags = c_int;
#[cfg (feature = "have_text_conversion")]
pub const PENDING_POINT_CHANGE: textconv_batch_edit_flags = 1;
#[cfg (feature = "have_text_conversion")]
pub const PENDING_COMPOSE_CHANGE: textconv_batch_edit_flags = 2;

/* The lisp.h and keyboard.h entry points this unit's C text calls and no mirror in its include
   graph carries.  One citation per line, at C's first use (C 197, C 216, C 430, C 671, C 686,
   C 1151, C 2547); `## Owed` 3-5 name the mirrors that should carry them instead.  */
#[cfg (feature = "have_text_conversion")]
unsafe extern "C" {
  /// C: lisp.h 4826: `extern void record_unwind_protect_excursion (void);` (eval.c).
  fn record_unwind_protect_excursion ();
  /// C: lisp.h 4924-4925: `extern Lisp_Object save_restriction_save (void);` /
  /// `extern void save_restriction_restore (Lisp_Object);` (editfns.c).
  fn save_restriction_save () -> Lisp_Object;
  fn save_restriction_restore (buffer: Lisp_Object);
  /// C: lisp.h 4335-4336: `extern void insert_from_string (Lisp_Object, ptrdiff_t, ptrdiff_t,
  /// ptrdiff_t, ptrdiff_t, bool);` (insdel.c).
  fn insert_from_string (string: Lisp_Object, from: ptrdiff_t, from_byte: ptrdiff_t,
			 chars: ptrdiff_t, bytes: ptrdiff_t, inherit: bool);
  /// C: lisp.h 4346: `extern Lisp_Object del_range_1 (ptrdiff_t, ptrdiff_t, bool, bool);` (insdel.c).
  fn del_range_1 (from: ptrdiff_t, to: ptrdiff_t, prepare: bool,
		  retain_markers: bool) -> Lisp_Object;
  /// C: lisp.h 4351: `extern int safe_del_range (ptrdiff_t, ptrdiff_t);` (insdel.c).
  fn safe_del_range (from: ptrdiff_t, to: ptrdiff_t) -> c_int;
  /// C: lisp.h 4920-4921: `extern void find_field (Lisp_Object, Lisp_Object, Lisp_Object,
  /// ptrdiff_t *, Lisp_Object, ptrdiff_t *);` (editfns.c).
  fn find_field (pos: Lisp_Object, merge_at_boundary: Lisp_Object, beg_limit: Lisp_Object,
		 beg: *mut ptrdiff_t, end_limit: Lisp_Object, end: *mut ptrdiff_t);
  /// C: keyboard.h 252: `extern bool reading_key_sequence;` (keyboard.c).
  static mut reading_key_sequence: bool;
}

/* C 81-83: "Copy the portion of the current buffer's text described by BEG, BEG_BYTE, END, END_BYTE
   to the char * buffer BUFFER, which should be at least END_BYTE - BEG_BYTEs long."  */

/* C window.h 296-302, under `#ifdef HAVE_TEXT_CONVERSION`: `ptrdiff_t last_point;` and
   `ptrdiff_t ephemeral_last_point;`.  `include/window.rs` carries `last_point` (its 134) and
   records `ephemeral_last_point` as the field it still owes (its 11), so the port names the
   field's address at the one place: the two are adjacent `ptrdiff_t`s in the C record, and the
   measurement on this machine (Evidence 2) is offset 352 for `last_point` and 360 for
   `ephemeral_last_point` over `sizeof (ptrdiff_t) == 8`.  `## Owed` 6 wants the field in the
   mirror, after which this macro disappears.  */
#[cfg (feature = "have_text_conversion")]
macro_rules! ephemeral_last_point {
  ($w:expr) => { *(&raw mut (*$w).last_point).add (1) };
}

/* <intprops.h> 68-69: `# define TYPE_MINIMUM(t) ((t) ~ TYPE_MAXIMUM (t))`, reached through lisp.h
   42.  The frozen `lib/include/intprops.rs` keeps C's text and is not compiled, so the unit writes
   the Rust primitive at the use site (the `src/androidselect.rs` 113-115 shape).  */
#[cfg (feature = "have_text_conversion")]
macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }

#[cfg (feature = "have_text_conversion")]
unsafe fn copy_buffer_text (beg: ptrdiff_t, beg_byte: ptrdiff_t,
			    end: ptrdiff_t, end_byte: ptrdiff_t,
			    buffer: *mut c_char)
{
  let mut beg0: ptrdiff_t;
  let mut end0: ptrdiff_t;
  let mut beg1: ptrdiff_t;
  let mut end1: ptrdiff_t;
  let mut size: ptrdiff_t;

  if beg_byte < GPT_BYTE! () && GPT_BYTE! () < end_byte                    // C 92
    {
      /* Two regions, before and after the gap.  */
      beg0 = beg_byte;                                                     // C 95
      end0 = GPT_BYTE! ();
      beg1 = GPT_BYTE! () + GAP_SIZE! () - BEG_BYTE as ptrdiff_t;
      end1 = end_byte + GAP_SIZE! () - BEG_BYTE as ptrdiff_t;
    }
  else
    {
      /* The only region.  */
      beg0 = beg_byte;                                                     // C 102
      end0 = end_byte;
      beg1 = -1;
      end1 = -1;
    }

  size = end0 - beg0;                                                      // C 109
  memcpy (buffer as *mut c_void, BYTE_POS_ADDR (beg0) as *const c_void, size as usize);
  if beg1 != -1                                                            // C 111
    { memcpy (buffer.offset (size) as *mut c_void,
	      BEG_ADDR! ().offset (beg1) as *const c_void, (end1 - beg1) as usize); }
}

/* C 117: "Conversion query."  */

/* C 119-120: "Return the position of the active mark, or -1 if there is no mark
   or it is not active."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn get_mark () -> ptrdiff_t
{
  if !NILP (BVAR! (current_buffer, mark_active_))                         // C 123
    && !(*XMARKER (BVAR! (current_buffer, mark_))).buffer.is_null ()
    { return marker_position (BVAR! (current_buffer, mark_)); }           // C 126

  return -1;                                                              // C 130
}

/* C 133-135: "Like Fselect_window.  However, if WINDOW is a minibuffer window
   but not the active minibuffer window, select its frame's selected
   window instead."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn select_window (mut window: Lisp_Object, norecord: Lisp_Object)
{
  let mut w: *mut window;

  w = XWINDOW (window);                                                   // C 142

  /* Work around GCC bug 114893
     <https://gcc.gnu.org/bugzilla/show_bug.cgi?id=114893>.  */
  eassume! (!w.is_null ());                                                // C 148

  if MINI_WINDOW_P (w)                                                    // C 150
    && WINDOW_LIVE_P (window)
    && !EQ (window, Factive_minibuffer_window ())
    { window = (*WINDOW_XFRAME! (w)).selected_window; }

  Fselect_window (window, norecord);                                      // C 153
}

/* C 156: "Restore the selected window WINDOW."  Its address goes to `record_unwind_protect`
   (C 199 and C 696/862/1016/...), so it is `unsafe extern "C" fn` (PORTING.md P-12).  */
#[cfg (feature = "have_text_conversion")]
unsafe extern "C" fn restore_selected_window (window: Lisp_Object)
{
  /* FIXME: not sure what to do if WINDOW has been deleted.  */
  select_window (window, Qt ());                                          // C 165
}

/* C 165-184: "Perform the text conversion operation specified in QUERY and return
   the results.

   Find the text between QUERY->position from point on frame F's
   selected window and QUERY->factor times QUERY->direction from that
   position.  Return it in QUERY->text.

   If QUERY->position is TYPE_MINIMUM (EMACS_INT) or EMACS_INT_MAX,
   start at the window's last point or mark, whichever is greater or
   smaller.

   Then, either delete that text from the buffer if QUERY->operation
   is TEXTCONV_SUBSTITUTION, or return 0.

   If FLAGS & TEXTCONV_SKIP_CONVERSION_REGION, then first move point
   past the conversion region in the specified direction if it is
   inside.

   Value is 0 if QUERY->operation was not TEXTCONV_SUBSTITUTION
   or if deleting the text was successful, and 1 otherwise.  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn textconv_query (f: *mut frame, query: *mut textconv_callback_struct,
					 flags: c_int) -> c_int
{
  let mut count: specpdl_ref;
  let mut pos: ptrdiff_t;
  let mut pos_byte: ptrdiff_t;
  let mut end: ptrdiff_t;
  let mut end_byte: ptrdiff_t;
  let mut start: ptrdiff_t;
  let mut temp: ptrdiff_t;
  let mut temp1: ptrdiff_t;
  let mut mark: ptrdiff_t;
  let mut buffer: *mut c_char;
  let mut w: *mut window;

  /* Save the excursion, as there will be extensive changes to the
     selected window.  */
  count = SPECPDL_INDEX ();                                               // C 196
  record_unwind_protect_excursion ();
  record_unwind_protect (Some (restore_selected_window), selected_window);

  /* Inhibit quitting.  */
  specbind (Qinhibit_quit (), Qt ());                                     // C 204

  /* Temporarily switch to F's selected window at the time of the last
     redisplay.  */
  select_window (if WINDOW_LIVE_P ((*f).old_selected_window)              // C 208
		   { (*f).old_selected_window } else { (*f).selected_window }, Qt ());
  w = XWINDOW (selected_window);

  /* Narrow to the field, if any.  */
  if !NILP ((*f).conversion.field)                                        // C 215
    {
      record_unwind_protect (Some (save_restriction_restore),
			     save_restriction_save ());
      Fnarrow_to_region (XCAR ((*f).conversion.field),
			 XCAR (XCDR ((*f).conversion.field)));
    }

  /* Now find the appropriate text bounds for QUERY.  First, move
     point QUERY->position steps forward or backwards.  */

  pos = PT! ();                                                           // C 224

  /* If QUERY->position is EMACS_INT_MAX, use the last mark or the
     ephemeral last point, whichever is greater.

     The opposite applies for EMACS_INT_MIN.  */

  mark = get_mark ();                                                     // C 231

  /* C 233/243's two `goto escape1`s become the labelled block below (PORTING.md 2.2); C's
     `escape1:` tail is the one copy after it.  */
  'escape1: {
    if (*query).position == EMACS_INT_MAX
      {
	pos = if mark == -1
		{ ephemeral_last_point! (w) }
	      else
		{ max (ephemeral_last_point! (w), mark) };
	break 'escape1;
      }
    else if (*query).position == TYPE_MINIMUM! (EMACS_INT)
      {
	pos = if mark == -1
		{ ephemeral_last_point! (w) }
	      else
		{ min (ephemeral_last_point! (w), mark) };
	break 'escape1;
      }

    /* Next, if POS lies within the conversion region and the caller
       asked for it to be moved away, move it away from the conversion
       region.  */

    if flags & TEXTCONV_SKIP_CONVERSION_REGION != 0                        // C 252
      && MARKERP ((*f).conversion.compose_region_start)
      {
	start = marker_position ((*f).conversion.compose_region_start);    // C 256
	end = marker_position ((*f).conversion.compose_region_end);

	if pos >= start && pos < end                                       // C 258
	  {
	    match (*query).direction
	      {
		TEXTCONV_FORWARD_CHAR | TEXTCONV_FORWARD_WORD
		| TEXTCONV_CARET_DOWN | TEXTCONV_NEXT_LINE | TEXTCONV_LINE_START =>
		  { pos = end; }                                           // C 266
		_ =>
		  { pos = max (BEGV! (), start - 1); }                     // C 273
	      }
	  }
      }

    /* If pos is outside the accessible part of the buffer or if it
       overflows, move back to point or to the extremes of the
       accessible region.  */

    {
      /* C 281: `if (ckd_add (&pos, pos, query->position)) pos = PT;` - the sum is stored either
	 way, then the overflow flag decides (the `src/cmds.rs` 469 shape).  */
      let (ckd_pos, ckd_overflow) = pos.overflowing_add ((*query).position);
      pos = ckd_pos;
      if ckd_overflow
	{ pos = PT! (); }
    }
  }
  /* escape1: */

  if pos < BEGV! ()                                                        // C 286
    { pos = BEGV! (); }

  if pos > ZV! ()                                                          // C 289
    { pos = ZV! (); }

  /* Move to pos.  */
  set_point (pos);                                                         // C 293
  pos = PT! ();
  pos_byte = PT_BYTE! ();

  /* Now scan forward or backwards according to what is in QUERY.  */

  match (*query).direction                                                 // C 299
    {
      TEXTCONV_FORWARD_CHAR =>
	{
	  /* Move forward by query->factor characters.  */
	  /* C 303: `if (ckd_add (&end, pos, query->factor) || end > ZV) end = ZV;`  */
	  let (ckd_end, ckd_overflow) = pos.overflowing_add ((*query).factor as ptrdiff_t);
	  end = ckd_end;
	  if ckd_overflow || end > ZV! ()
	    { end = ZV! (); }

	  end_byte = CHAR_TO_BYTE (end);                                   // C 306
	}

      TEXTCONV_BACKWARD_CHAR =>
	{
	  /* Move backward by query->factor characters.  */
	  /* C 310: `if (ckd_sub (&end, pos, query->factor) || end < BEGV) end = BEGV;`  */
	  let (ckd_end, ckd_overflow) = pos.overflowing_sub ((*query).factor as ptrdiff_t);
	  end = ckd_end;
	  if ckd_overflow || end < BEGV! ()
	    { end = BEGV! (); }

	  end_byte = CHAR_TO_BYTE (end);                                   // C 314
	}

      TEXTCONV_FORWARD_WORD =>
	{
	  /* Move forward by query->factor words.  */
	  end = scan_words (pos, (*query).factor as EMACS_INT);            // C 319

	  if end == 0                                                      // C 321
	    {
	      end = ZV! ();
	      end_byte = ZV_BYTE! ();
	    }
	  else
	    { end_byte = CHAR_TO_BYTE (end); }                             // C 327
	}

      TEXTCONV_BACKWARD_WORD =>
	{
	  /* Move backwards by query->factor words.  */
	  end = scan_words (pos, 0 - ((*query).factor as EMACS_INT));      // C 333

	  if end == 0                                                      // C 335
	    {
	      end = BEGV! ();
	      end_byte = BEGV_BYTE! ();
	    }
	  else
	    { end_byte = CHAR_TO_BYTE (end); }                             // C 341
	}

      TEXTCONV_CARET_UP =>
	{
	  /* Move upwards one visual line, keeping the column intact.  */
	  Fvertical_motion (Fcons (Fcurrent_column (), make_fixnum (-1)),  // C 346-347
			    Qnil (), Qnil ());
	  end = PT! ();
	  end_byte = PT_BYTE! ();
	}

      TEXTCONV_CARET_DOWN =>
	{
	  /* Move downwards one visual line, keeping the column
	     intact.  */
	  Fvertical_motion (Fcons (Fcurrent_column (), make_fixnum (1)),   // C 354-355
			    Qnil (), Qnil ());
	  end = PT! ();
	  end_byte = PT_BYTE! ();
	}

      TEXTCONV_NEXT_LINE =>
	{
	  /* Move one line forward.  */
	  scan_newline (pos, pos_byte, ZV! (), ZV_BYTE! (),                  // C 363-364
			(*query).factor as ptrdiff_t, false);
	  end = PT! ();
	  end_byte = PT_BYTE! ();
	}

      TEXTCONV_PREVIOUS_LINE =>
	{
	  /* Move one line backwards.  */
	  scan_newline (pos, pos_byte, BEGV! (), BEGV_BYTE! (),              // C 371-372
			0 - ((*query).factor as ptrdiff_t), false);
	  end = PT! ();
	  end_byte = PT_BYTE! ();
	}

      TEXTCONV_LINE_START =>
	{
	  /* Move to the beginning of the line.  */
	  Fbeginning_of_line (Qnil ());                                     // C 380
	  end = PT! ();
	  end_byte = PT_BYTE! ();
	}

      TEXTCONV_LINE_END =>
	{
	  /* Move to the end of the line.  */
	  Fend_of_line (Qnil ());                                           // C 387
	  end = PT! ();
	  end_byte = PT_BYTE! ();
	}

      TEXTCONV_ABSOLUTE_POSITION =>
	{
	  /* How to implement this is unclear.  */
	  SET_PT ((*query).factor as ptrdiff_t);                            // C 394
	  end = PT! ();
	  end_byte = PT_BYTE! ();
	}

      _ =>
	{
	  unbind_to (count, Qnil ());                                       // C 400
	  return 1;
	}
    }

  /* Sort end and pos.  */

  if end < pos                                                             // C 406
    {
      eassert! (end_byte < pos_byte);
      temp = pos_byte;
      temp1 = pos;
      pos_byte = end_byte;
      pos = end;
      end = temp1;
      end_byte = temp;
    }

  /* Return the string first.  */
  buffer = xmalloc ((end_byte - pos_byte) as usize) as *mut c_char;        // C 418
  copy_buffer_text (pos, pos_byte, end, end_byte, buffer);
  (*query).text.text = buffer;
  (*query).text.length = end - pos;
  (*query).text.bytes = end_byte - pos_byte;

  /* Next, perform any operation specified.  */

  match (*query).operation                                                 // C 426
    {
      TEXTCONV_SUBSTITUTION =>
	{
	  if safe_del_range (pos, end) != 0
	    {
	      /* Undo any changes to the excursion.  */
	      unbind_to (count, Qnil ());
	      return 1;
	    }
	}

      _ => { }
    }

  /* Undo any changes to the excursion.  */
  unbind_to (count, Qnil ());                                              // C 443
  return 0;
}

/* C 445-446: "Update the overlay displaying the conversion area on frame F after
   a change to the conversion region."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn sync_overlay (f: *mut frame)
{
  if MARKERP ((*f).conversion.compose_region_start)                        // C 451
    && !NILP (Vtext_conversion_face)
    {
      if NILP ((*f).conversion.compose_region_overlay)                      // C 455
	{
	  (*f).conversion.compose_region_overlay                            // C 457
	    = Fmake_overlay ((*f).conversion.compose_region_start,
			     (*f).conversion.compose_region_end, Qnil (),
			     Qt (), Qnil ());
	  Foverlay_put ((*f).conversion.compose_region_overlay,
			Qface (), Vtext_conversion_face);
	}

      Fmove_overlay ((*f).conversion.compose_region_overlay,                // C 464
		     (*f).conversion.compose_region_start,
		     (*f).conversion.compose_region_end, Qnil ());
    }
  else if !NILP ((*f).conversion.compose_region_overlay)
    {
      Fdelete_overlay ((*f).conversion.compose_region_overlay);
      (*f).conversion.compose_region_overlay = Qnil ();
    }
}

/* C 475-479: "Record a change to the current buffer as a result of an
   asynchronous text conversion operation.

   Consult the doc string of `text-conversion-edits' for the meaning
   of BEG, END, and EPHEMERAL."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn record_buffer_change (beg: ptrdiff_t, end: ptrdiff_t,
				ephemeral: Lisp_Object)
{
  let mut buffer: Lisp_Object;
  let mut beg_marker: Lisp_Object;
  let mut end_marker: Lisp_Object;

  XSETBUFFER! (buffer, current_buffer);                                    // C 487

  /* Make markers for both BEG and END.  */
  beg_marker = build_marker (current_buffer as *mut c_void, beg,            // C 492-493
			     CHAR_TO_BYTE (beg));

  /* If BEG and END are identical, make sure to keep the markers
     eq.  */

  if beg == end                                                            // C 496
    { end_marker = beg_marker; }
  else
    {
      end_marker = build_marker (current_buffer as *mut c_void, end,        // C 502-503
				 CHAR_TO_BYTE (end));

      /* Otherwise, make sure the marker extends past inserted
	 text.  */
      Fset_marker_insertion_type (end_marker, Qt ());
    }

  Vtext_conversion_edits                                                    // C 508
    = Fcons (list4 (buffer, beg_marker, end_marker,
		    ephemeral),
	     Vtext_conversion_edits);
}

/* C 514-515: "Reset text conversion state of frame F, and resume text conversion.
   Delete any overlays or markers inside."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_frame_conversion (f: *mut frame)
{
  reset_frame_state (f);
  if !text_interface.is_null () && FRAME_WINDOW_P (f) && FRAME_VISIBLE_P (f)
    { ((*text_interface).reset.unwrap ()) (f); }
}

/* C 525-526: "Reset text conversion state of frame F.  Delete any overlays or
   markers inside."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_frame_state (f: *mut frame)
{
  let mut last: *mut text_conversion_action;
  let mut next: *mut text_conversion_action;

  /* Make the composition region markers point elsewhere.  */

  if !NILP ((*f).conversion.compose_region_start)                          // C 535
    {
      Fset_marker ((*f).conversion.compose_region_start, Qnil (), Qnil ());
      Fset_marker ((*f).conversion.compose_region_end, Qnil (), Qnil ());
      (*f).conversion.compose_region_start = Qnil ();
      (*f).conversion.compose_region_end = Qnil ();
    }

  /* Delete the composition region overlay.  */

  if !NILP ((*f).conversion.compose_region_overlay)                        // C 547
    { Fdelete_overlay ((*f).conversion.compose_region_overlay); }

  /* Delete each text conversion action queued up.  */

  next = (*f).conversion.actions;                                          // C 551
  while !next.is_null ()
    {
      last = next;
      next = (*next).next;

      /* Say that the conversion is finished.  */
      if !text_interface.is_null () && (*text_interface).notify_conversion.is_some ()
	{ ((*text_interface).notify_conversion.unwrap ()) ((*last).counter); }   // C 561

      xfree (last as *mut c_void);                                          // C 563
    }
  (*f).conversion.actions = std::ptr::null_mut ();                          // C 565

  /* Clear batch edit state.  */
  (*f).conversion.batch_edit_count = 0;                                     // C 568
  (*f).conversion.batch_edit_flags = 0;

  /* Clear active field.  */
  if !NILP ((*f).conversion.field)                                          // C 571
    {
      Fset_marker (XCAR ((*f).conversion.field), Qnil (), Qnil ());
      Fset_marker (XCAR (XCDR ((*f).conversion.field)), Qnil (),
		   Qnil ());
    }
  (*f).conversion.field = Qnil ();                                          // C 578
}

/* C 580-581: "Return whether or not there are pending edits from an input method
   on any frame."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn detect_conversion_events () -> bool
{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;

  FOR_EACH_FRAME! (tail, frame, {
    /* See if there's a pending edit on this frame.  */
    if !(*XFRAME (frame)).conversion.actions.is_null ()                     // C 592
      && ((*(*XFRAME (frame)).conversion.actions).operation != TEXTCONV_BARRIER
	  || kbd_fetch_ptr == kbd_store_ptr)
      { return true; }
  });

  return false;                                                             // C 598
}

/* C 605-609: "Commit the given text in the composing region.  If there is no
   composing region, then insert the text after frame F's selected
   window's last point instead, unless the mark is active.  Finally,
   remove the composing region.

   If the mark is active, delete the text between mark and point.

   Then, move point to POSITION relative to TEXT.  If POSITION is
   greater than zero, it is relative to the character at the end of
   TEXT; otherwise, it is relative to the start of TEXT."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn really_commit_text (f: *mut frame, position: EMACS_INT,
			      text: Lisp_Object)
{
  let mut count: specpdl_ref;
  let mut wanted: ptrdiff_t;
  let mut start: ptrdiff_t;
  let mut end: ptrdiff_t;
  let mut mark: ptrdiff_t;
  let mut w: *mut window;

  /* If F's old selected window is no longer alive, fail.  */

  if !WINDOW_LIVE_P ((*f).old_selected_window)                            // C 624
    { return; }

  count = SPECPDL_INDEX ();                                                // C 626
  record_unwind_protect (Some (restore_selected_window),
			 selected_window);

  /* Temporarily switch to F's selected window at the time of the last
     redisplay.  */
  select_window ((*f).old_selected_window, Qt ());                         // C 633

  /* Now detect whether or not there is a composing or active region.
     If there is, then replace it with TEXT.  Don't do that
     otherwise.  */

  mark = get_mark ();                                                      // C 639
  TEXTCONV_DEBUG! ("the mark is: %td", mark);                              // C 640
  if MARKERP ((*f).conversion.compose_region_start) || mark != -1
    {
      /* Replace its contents.  Set START and END to the start and end
	 of the composing region if it exists.  */

      if MARKERP ((*f).conversion.compose_region_start)                    // C 647
	{
	  start = marker_position ((*f).conversion.compose_region_start);
	  end = marker_position ((*f).conversion.compose_region_end);
	}
      else
	{
	  /* Otherwise, set it to the start and end of the region.  */
	  start = min (mark, PT! ());                                      // C 654
	  end = max (mark, PT! ());
	}

      TEXTCONV_DEBUG! ("replacing text in composing region: %td, %td",     // C 658
		       start, end);

      /* If it transpires that the start of the compose region is not
	 point, move point there.  */

      if start != PT! ()                                                   // C 665
	{ set_point (start); }

      /* Now delete whatever needs to go.  */

      del_range_1 (start, end, true, false);                               // C 671
      record_buffer_change (start, start, Qt ());

      /* Don't record changes if TEXT is empty.  */

      if SCHARS (text) != 0                                                // C 677
	{
	  /* Insert the new text.  Make sure to inherit text
	     properties from the surroundings: if this doesn't happen,
	     CC Mode fontification can get thrown off and become very
	     slow.  */

	  insert_from_string (text, 0, 0, SCHARS (text),                    // C 686-687
			      SBYTES (text), true);
	  record_buffer_change (start, PT! (), text);
	}

      /* Move to the position specified in POSITION.  */

      if position <= 0                                                     // C 693
	{
	  /* If POSITION is less than zero, it is relative to the
	     start of the text that was inserted.  */
	  wanted = start;

	  /* C 698: `if (ckd_add (&wanted, wanted, position) || wanted < BEGV) wanted = BEGV;`  */
	  let (ckd_wanted, ckd_overflow) = wanted.overflowing_add (position);
	  wanted = ckd_wanted;
	  if ckd_overflow || wanted < BEGV! ()
	    { wanted = BEGV! (); }

	  if wanted > ZV! ()                                               // C 702
	    { wanted = ZV! (); }

	  set_point (wanted);                                              // C 705
	}
      else
	{
	  /* Otherwise, it is relative to the last character in
	     TEXT.  */
	  wanted = PT! ();                                                 // C 712

	  /* C 714: `if (ckd_add (&wanted, wanted, position - 1) || wanted > ZV) wanted = ZV;`  */
	  let (ckd_wanted, ckd_overflow) = wanted.overflowing_add (position - 1);
	  wanted = ckd_wanted;
	  if ckd_overflow || wanted > ZV! ()
	    { wanted = ZV! (); }

	  if wanted < BEGV! ()                                             // C 717
	    { wanted = BEGV! (); }

	  set_point (wanted);                                              // C 720
	}

      /* Make the composition region markers point elsewhere.  */

      if !NILP ((*f).conversion.compose_region_start)                      // C 725
	{
	  Fset_marker ((*f).conversion.compose_region_start, Qnil (), Qnil ());
	  Fset_marker ((*f).conversion.compose_region_end, Qnil (), Qnil ());
	  (*f).conversion.compose_region_start = Qnil ();
	  (*f).conversion.compose_region_end = Qnil ();
	}

      /* Delete the composition region overlay.  */

      if !NILP ((*f).conversion.compose_region_overlay)                    // C 737
	{ Fdelete_overlay ((*f).conversion.compose_region_overlay); }
    }
  else
    {
      /* Otherwise, move the text and point to an appropriate
	 location.  */
      wanted = PT! ();                                                     // C 745

      /* Don't record changes if TEXT is empty.  */

      if SCHARS (text) != 0                                                // C 749
	{
	  /* Insert the new text.  Make sure to inherit text
	     properties from the surroundings: if this doesn't happen,
	     CC Mode fontification can get thrown off and become very
	     slow.  */

	  insert_from_string (text, 0, 0, SCHARS (text),                   // C 758-759
			      SBYTES (text), true);

	  record_buffer_change (wanted, PT! (), text);
	}

      if position <= 0                                                     // C 765
	{
	  /* C 767: `if (ckd_add (&wanted, wanted, position) || wanted < BEGV) wanted = BEGV;`  */
	  let (ckd_wanted, ckd_overflow) = wanted.overflowing_add (position);
	  wanted = ckd_wanted;
	  if ckd_overflow || wanted < BEGV! ()
	    { wanted = BEGV! (); }

	  if wanted > ZV! ()                                               // C 770
	    { wanted = ZV! (); }

	  set_point (wanted);
	}
      else
	{
	  wanted = PT! ();                                                 // C 777

	  /* C 779: `if (ckd_add (&wanted, wanted, position - 1) || wanted > ZV) wanted = ZV;`  */
	  let (ckd_wanted, ckd_overflow) = wanted.overflowing_add (position - 1);
	  wanted = ckd_wanted;
	  if ckd_overflow || wanted > ZV! ()
	    { wanted = ZV! (); }

	  if wanted < BEGV! ()                                             // C 782
	    { wanted = BEGV! (); }

	  set_point (wanted);
	}
    }

  /* This should deactivate the mark.  */
  call0 (Qdeactivate_mark ());                                             // C 790

  /* Print some debugging information.  */
  TEXTCONV_DEBUG! ("text inserted: %s, point now: %td",                    // C 793
		   SSDATA (text), PT! ());

  /* Update the ephemeral last point.  */
  w = XWINDOW (selected_window);                                           // C 797
  ephemeral_last_point! (w) = PT! ();
  unbind_to (count, Qnil ());
}

/* C 801-803: "Remove the composition region on the frame F, while leaving its
   contents intact.  If UPDATE, also notify the input method of the
   change."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn really_finish_composing_text (f: *mut frame, update: bool)
{
  if !NILP ((*f).conversion.compose_region_start)                          // C 807
    {
      Fset_marker ((*f).conversion.compose_region_start, Qnil (), Qnil ());
      Fset_marker ((*f).conversion.compose_region_end, Qnil (), Qnil ());
      (*f).conversion.compose_region_start = Qnil ();
      (*f).conversion.compose_region_end = Qnil ();

      if update && !text_interface.is_null ()                              // C 813
	  && (*text_interface).compose_region_changed.is_some ()
	{ ((*text_interface).compose_region_changed.unwrap ()) (f); }
    }

  /* Delete the composition region overlay.  */

  if !NILP ((*f).conversion.compose_region_overlay)                        // C 821
    { Fdelete_overlay ((*f).conversion.compose_region_overlay); }

  TEXTCONV_DEBUG! ("conversion region removed");                           // C 824
}

/* C 826-831: "Set the composing text on frame F to TEXT.  Then, move point to an
   appropriate position relative to POSITION, and call
   `compose_region_changed' in the text conversion interface should
   point not have been changed relative to F's old selected window's
   last point."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn really_set_composing_text (f: *mut frame, position: ptrdiff_t,
				     text: Lisp_Object)
{
  let mut count: specpdl_ref;
  let mut start: ptrdiff_t;
  let mut wanted: ptrdiff_t;
  let mut end: ptrdiff_t;
  let mut w: *mut window;

  /* If F's old selected window is no longer live, fail.  */

  if !WINDOW_LIVE_P ((*f).old_selected_window)                             // C 835
    { return; }

  count = SPECPDL_INDEX ();                                                // C 838
  record_unwind_protect (Some (restore_selected_window),
			 selected_window);

  /* Temporarily switch to F's selected window at the time of the last
     redisplay.  */
  w = XWINDOW ((*f).old_selected_window);
  select_window ((*f).old_selected_window, Qt ());

  /* Now set up the composition region if necessary.  */

  if !MARKERP ((*f).conversion.compose_region_start)                       // C 852
    {
      /* Set START and END.  */
      start = PT! ();                                                      // C 855
      wanted = get_mark ();
      end = wanted;

      /* If END is -1, set it to start.  */

      if end == -1                                                         // C 859
	{ end = start; }
      else
	{
	  /* Now sort start and end.  */
	  start = min (start, end);
	  end  = max (PT! (), wanted);
	}

      /* If END is not the same as start, delete the text in
	 between.  */

      if end != start                                                      // C 871
	{
	  del_range_1 (start, end, true, false);
	  set_point (start);
	  record_buffer_change (start, start, Qt ());
	}

      /* Now set the markers which denote the composition region.  */
      (*f).conversion.compose_region_start                                 // C 879
	= build_marker (current_buffer as *mut c_void, PT! (), PT_BYTE! ());
      (*f).conversion.compose_region_end
	= build_marker (current_buffer as *mut c_void, PT! (), PT_BYTE! ());

      Fset_marker_insertion_type ((*f).conversion.compose_region_end,       // C 882
				  Qt ());
    }
  else
    {
      /* Delete the text between the start of the composing region and
	 its end.  */
      start = marker_position ((*f).conversion.compose_region_start);      // C 890
      end = marker_position ((*f).conversion.compose_region_end);
      del_range_1 (start, end, true, false);
      set_point (start);

      if start != end                                                      // C 895
	{ record_buffer_change (start, start, Qt ()); }
    }

  /* Insert the new text.  Make sure to inherit text properties from
     the surroundings: if this doesn't happen, CC Mode fontification
     can get thrown off and become very slow.  */

  insert_from_string (text, 0, 0, SCHARS (text),                           // C 903-904
		      SBYTES (text), true);

  if start != PT! ()                                                       // C 906
    { record_buffer_change (start, PT! (), Qt ()); }

  /* Now move point to an appropriate location.  */
  if position <= 0                                                         // C 909
    {
      wanted = start;

      /* C 912: `if (ckd_sub (&wanted, wanted, position) || wanted < BEGV) wanted = BEGV;`  */
      let (ckd_wanted, ckd_overflow) = wanted.overflowing_sub (position);
      wanted = ckd_wanted;
      if ckd_overflow || wanted < BEGV! ()
	{ wanted = BEGV! (); }

      if wanted > ZV! ()                                                   // C 915
	{ wanted = ZV! (); }
    }
  else
    {
      end = marker_position ((*f).conversion.compose_region_end);          // C 920
      wanted = end;

      /* end should be PT after the edit.  */
      eassert! (end == PT! ());                                            // C 924

      /* C 926: `if (ckd_add (&wanted, wanted, position - 1) || wanted > ZV) wanted = ZV;`  */
      let (ckd_wanted, ckd_overflow) = wanted.overflowing_add (position - 1);
      wanted = ckd_wanted;
      if ckd_overflow || wanted > ZV! ()
	{ wanted = ZV! (); }

      if wanted < BEGV! ()                                                 // C 929
	{ wanted = BEGV! (); }
    }

  set_point (wanted);                                                      // C 932

  /* This should deactivate the mark.  */
  call0 (Qdeactivate_mark ());                                             // C 935

  /* Move the composition overlay.  */
  sync_overlay (f);                                                        // C 938

  /* If TEXT is empty, remove the composing region.  This goes against
     the documentation, but is ultimately what programs expect.  */

  if SCHARS (text) == 0                                                    // C 943
    { really_finish_composing_text (f, false); }

  /* If PT hasn't changed, the conversion region definitely has.
     Otherwise, redisplay will update the input method instead.  */

  if PT! () == ephemeral_last_point! (w)                                   // C 950
    && !text_interface.is_null ()
    && (*text_interface).compose_region_changed.is_some ()
    {
      if (*f).conversion.batch_edit_count > 0
	{ (*f).conversion.batch_edit_flags |= PENDING_COMPOSE_CHANGE; }
      else
	{ ((*text_interface).compose_region_changed.unwrap ()) (f); }
    }

  /* Update the ephemeral last point.  */
  w = XWINDOW (selected_window);                                           // C 962
  ephemeral_last_point! (w) = PT! ();

  if SCHARS (text) != 0                                                    // C 965
    { TEXTCONV_DEBUG! ("conversion region set to: %td %td",
		       marker_position ((*f).conversion.compose_region_start),
		       marker_position ((*f).conversion.compose_region_end)); }
  else
    { TEXTCONV_DEBUG! ("conversion region removed; PT is now: %td", PT! ()); }

  unbind_to (count, Qnil ());                                              // C 971
}

/* C 969-970: "Set the composing region of frame F to START by END.  Make it if
   it is not already set."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn really_set_composing_region (f: *mut frame, start: ptrdiff_t,
				       end: ptrdiff_t)
{
  let mut count: specpdl_ref;
  let mut w: *mut window;

  /* If F's old selected window is no longer live, fail.  */

  if !WINDOW_LIVE_P ((*f).old_selected_window)                             // C 980
    { return; }

  /* If MAX (0, start) == end, then this should behave the same as
     really_finish_composing_text.  */

  if max (0, start) == max (0, end)                                        // C 988
    {
      really_finish_composing_text (f, false);
      return;
    }

  count = SPECPDL_INDEX ();                                                // C 994
  record_unwind_protect (Some (restore_selected_window),
			 selected_window);

  /* Temporarily switch to F's selected window at the time of the last
     redisplay.  */
  select_window ((*f).old_selected_window, Qt ());                         // C 1001

  /* Now set up the composition region if necessary.  */

  if !MARKERP ((*f).conversion.compose_region_start)                       // C 1005
    {
      (*f).conversion.compose_region_start = Fmake_marker ();
      (*f).conversion.compose_region_end = Fmake_marker ();
      Fset_marker_insertion_type ((*f).conversion.compose_region_end,
				  Qt ());
    }

  Fset_marker ((*f).conversion.compose_region_start,                       // C 1015
	       make_fixnum (start), Qnil ());
  Fset_marker ((*f).conversion.compose_region_end,
	       make_fixnum (end), Qnil ());
  sync_overlay (f);                                                        // C 1019

  TEXTCONV_DEBUG! ("composing region set to: %td, %td; point is: %td",     // C 1021
		   start, end, PT! ());

  /* Update the ephemeral last point.  */
  w = XWINDOW (selected_window);                                           // C 1025
  ephemeral_last_point! (w) = PT! ();

  unbind_to (count, Qnil ());                                              // C 1028
}

/* C 1022-1025: "Delete LEFT and RIGHT chars around point or the active mark,
   whichever is larger, in frame F's selected window, avoiding the
   composing region if necessary."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn really_delete_surrounding_text (f: *mut frame, left: ptrdiff_t,
					  right: ptrdiff_t)
{
  let mut count: specpdl_ref;
  let mut start: ptrdiff_t;
  let mut end: ptrdiff_t;
  let mut a: ptrdiff_t;
  let mut b: ptrdiff_t;
  let mut a1: ptrdiff_t;
  let mut b1: ptrdiff_t;
  let mut lstart: ptrdiff_t;
  let mut rstart: ptrdiff_t;
  let mut w: *mut window;
  let mut text: Lisp_Object = Qnil ();         // C 1040 (uninitialized in C; PORTING.md 1.13)

  /* If F's old selected window is no longer live, fail.  */

  if !WINDOW_LIVE_P ((*f).old_selected_window)                             // C 1044
    { return; }

  count = SPECPDL_INDEX ();                                                // C 1047
  record_unwind_protect (Some (restore_selected_window),
			 selected_window);

  /* Temporarily switch to F's selected window at the time of the last
     redisplay.  */
  select_window ((*f).old_selected_window, Qt ());                         // C 1054

  /* Narrow to the field, if any.  */
  if !NILP ((*f).conversion.field)                                         // C 1057
    {
      record_unwind_protect (Some (save_restriction_restore),
			     save_restriction_save ());
      Fnarrow_to_region (XCAR ((*f).conversion.field),
			 XCAR (XCDR ((*f).conversion.field)));
    }

  /* Figure out where to start deleting from.  */

  a = get_mark ();                                                         // C 1066

  if a != -1 && a != PT! ()                                                // C 1068
    { lstart = max (a, PT! ()); rstart = lstart; }
  else
    { lstart = PT! (); rstart = lstart; }

  /* Avoid the composing text.  This behavior is identical to how
     Android's BaseInputConnection actually implements avoiding the
     composing span.  */

  if MARKERP ((*f).conversion.compose_region_start)                        // C 1077
    {
      a = marker_position ((*f).conversion.compose_region_start);
      b = marker_position ((*f).conversion.compose_region_end);

      a1 = min (a, b);
      b1 = max (a, b);

      lstart = min (lstart, min (PT! (), a1));                             // C 1085
      rstart = max (rstart, max (PT! (), b1));
    }

  if lstart == rstart                                                      // C 1089
    {
      start = max (BEGV! (), lstart - left);
      end = min (ZV! (), rstart + right);

      text = del_range_1 (start, end, true, true);
      record_buffer_change (start, start, text);
    }
  else
    {
      /* Don't record a deletion if the text which was deleted lies
	 after point.  */

      start = rstart;                                                      // C 1101
      end = min (ZV! (), rstart + right);
      text = del_range_1 (start, end, true, true);
      record_buffer_change (start, start, Qnil ());

      /* Now delete what must be deleted on the left.  */

      start = max (BEGV! (), lstart - left);                               // C 1109
      end = lstart;
      text = del_range_1 (start, end, true, true);
      record_buffer_change (start, start, text);
    }

  TEXTCONV_DEBUG! ("deleted surrounding text: %td, %td; PT is now %td",    // C 1115
		   left, right, PT! ());

  /* if the mark is now equal to start, deactivate it.  */

  if get_mark () == PT! ()                                                 // C 1120
    { call0 (Qdeactivate_mark ()); }

  /* Update the ephemeral last point.  */
  w = XWINDOW (selected_window);                                           // C 1124
  ephemeral_last_point! (w) = PT! ();

  unbind_to (count, Qnil ());                                              // C 1127
}

/* C 1110-1112: "Save the confines of the field surrounding point in w into F's text
   conversion state.  If NOTIFY_COMPOSE, notify the input method of
   changes to the composition region if they arise in this process."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn locate_and_save_position_in_field (f: *mut frame, w: *mut window,
					     mut notify_compose: bool)
{
  let mut pos: Lisp_Object;
  let mut window: Lisp_Object;
  let mut c1: Lisp_Object = Qnil ();           // C 1136 (uninitialized in C; PORTING.md 1.13)
  let mut c2: Lisp_Object = Qnil ();           // C 1136
  let mut count: specpdl_ref;
  let mut beg: ptrdiff_t = 0;      // C 1136 (uninitialized in C; PORTING.md 1.13)
  let mut end: ptrdiff_t = 0;      // C 1136
  let mut cstart: ptrdiff_t;
  let mut cend: ptrdiff_t;
  let mut newstart: ptrdiff_t;
  let mut newend: ptrdiff_t;

  /* Set the current buffer to W's.  */
  count = SPECPDL_INDEX ();                                                // C 1142

  /* The current buffer must be saved, not merely the selected
     window.  */
  record_unwind_protect_excursion ();                                      // C 1147
  record_unwind_protect (Some (restore_selected_window), selected_window);
  XSETWINDOW! (window, w);                                                 // C 1149
  select_window (window, Qt ());

  /* Search for a field around the current editing position; this should
     also serve to confine text conversion to the visible region.  */
  XSETFASTINT! (pos, min (max (ephemeral_last_point! (w), BEGV! ()), ZV! ()));   // C 1151
  find_field (pos, Qnil (), Qnil (), &raw mut beg, Qnil (), &raw mut end);  // C 1152

  /* If beg is 1 and end is ZV, disable the active field entirely.  */
  'exit: {
    if beg == 1 && end == ZV! ()                                           // C 1155
      {
	(*f).conversion.field = Qnil ();
	break 'exit;
      }

    /* Don't cons if a pair already exists.  */
    if !NILP ((*f).conversion.field)                                       // C 1162
      {
	c1 = (*f).conversion.field;
	c2 = XCDR (c1);
	Fset_marker (XCAR (c1), make_fixed_natnum (beg), Qnil ());
	Fset_marker (XCAR (c2), make_fixed_natnum (end), Qnil ());
	XSETCDR (c2, window);
      }
    else
      {
	c1 = build_marker (current_buffer as *mut c_void, beg, CHAR_TO_BYTE (beg));
	c2 = build_marker (current_buffer as *mut c_void, end, CHAR_TO_BYTE (end));
	Fset_marker_insertion_type (c2, Qt ());
	(*f).conversion.field = Fcons (c1, Fcons (c2, window));
      }

    /* If the composition region is active and oversteps the active field,
       restrict it to the same.  */

    if !NILP ((*f).conversion.compose_region_start)                        // C 1181
      {
	cstart = marker_position ((*f).conversion.compose_region_start);
	cend   = marker_position ((*f).conversion.compose_region_end);

	if cend < beg || cstart > end                                      // C 1187
	  {
	    /* Remove the composition region in whole.  */
	    /* Make the composition region markers point elsewhere.  */

	    if !NILP ((*f).conversion.compose_region_start)                // C 1193
	      {
		Fset_marker ((*f).conversion.compose_region_start, Qnil (), Qnil ());
		Fset_marker ((*f).conversion.compose_region_end, Qnil (), Qnil ());
		(*f).conversion.compose_region_start = Qnil ();
		(*f).conversion.compose_region_end = Qnil ();
	      }

	    /* Delete the composition region overlay.  */

	    if !NILP ((*f).conversion.compose_region_overlay)              // C 1204
	      { Fdelete_overlay ((*f).conversion.compose_region_overlay); }

	    TEXTCONV_DEBUG! ("removing composing region outside active field");  // C 1207
	  }
	else
	  {
	    newstart = max (beg, min (cstart, end));                       // C 1211
	    newend   = max (beg, min (cend, end));

	    if newstart != cstart || newend != cend                        // C 1214
	      {
		TEXTCONV_DEBUG! ("confined composing region to %td, %td",   // C 1216
				 newstart, newend);
		Fset_marker ((*f).conversion.compose_region_start,
			     make_fixed_natnum (newstart), Qnil ());
		Fset_marker ((*f).conversion.compose_region_end,
			     make_fixed_natnum (newend), Qnil ());
	      }
	    else
	      { notify_compose = false; }                                  // C 1225
	  }
      }
    else
      { notify_compose = false; }                                          // C 1229

    if notify_compose                                                      // C 1231
      && (*text_interface).compose_region_changed.is_some ()
      {
	if (*f).conversion.batch_edit_count > 0
	  { (*f).conversion.batch_edit_flags |= PENDING_COMPOSE_CHANGE; }
	else
	  { ((*text_interface).compose_region_changed.unwrap ()) (f); }
      }
  }
  /* exit: */

  unbind_to (count, Qnil ());                                              // C 1244
}

/* C 1238-1240: "Update the interface with frame F's new point and mark.  If a batch
   edit is in progress, schedule the update for when it finishes
   instead."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn really_request_point_update (f: *mut frame)
{
  let mut w: *mut window;

  /* If F's old selected window is no longer live, fail.  */

  if !WINDOW_LIVE_P ((*f).old_selected_window)                             // C 1249
    { return; }

  if (*f).conversion.batch_edit_count > 0                                  // C 1252
    { (*f).conversion.batch_edit_flags |= PENDING_POINT_CHANGE; }
  else if !text_interface.is_null () && (*text_interface).point_changed.is_some ()
    {
      w = XWINDOW ((*f).old_selected_window);
      locate_and_save_position_in_field (f, w, false);
      ((*text_interface).point_changed.unwrap ()) (f, w as *mut c_void,
						   current_buffer as *mut c_void);
    }
}

/* C 1265-1271: "Set point in frame F's selected window to POSITION.  If MARK is not
   at POSITION, activate the mark and set MARK to that as well.

   If point was not changed, signal an update through the text input
   interface, which is necessary for the IME to acknowledge that the
   change has completed."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn really_set_point_and_mark (f: *mut frame, point: ptrdiff_t,
				     mark: ptrdiff_t)
{
  let mut count: specpdl_ref;
  let mut w: *mut window;

  /* If F's old selected window is no longer live, fail.  */

  if !WINDOW_LIVE_P ((*f).old_selected_window)                             // C 1278
    { return; }

  count = SPECPDL_INDEX ();                                                // C 1281
  record_unwind_protect (Some (restore_selected_window),
			 selected_window);

  /* Temporarily switch to F's selected window at the time of the last
     redisplay.  */
  select_window ((*f).old_selected_window, Qt ());                         // C 1288

  if point == PT! ()                                                       // C 1290
    {
      if (*f).conversion.batch_edit_count > 0
	{ (*f).conversion.batch_edit_flags |= PENDING_POINT_CHANGE; }
      else if !text_interface.is_null () && (*text_interface).point_changed.is_some ()
	{
	  w = XWINDOW ((*f).old_selected_window);
	  locate_and_save_position_in_field (f, w, false);
	  ((*text_interface).point_changed.unwrap ()) (f, w as *mut c_void,
								 current_buffer as *mut c_void);
	}
    }
  else
    /* Set the point.  */
    { Fgoto_char (make_fixnum (point)); }                                  // C 1302

  if mark == point                                                        // C 1304
    && !NILP (BVAR! (current_buffer, mark_active_))
    { call0 (Qdeactivate_mark ()); }
  else
    { calln! (Qpush_mark (), make_fixnum (mark)); }                        // C 1307

  /* Update the ephemeral last point.  */
  w = XWINDOW (selected_window);                                           // C 1310
  ephemeral_last_point! (w) = PT! ();

  TEXTCONV_DEBUG! ("set point and mark: %td %td",                          // C 1313
		   PT! (), get_mark ());

  unbind_to (count, Qnil ());                                              // C 1316
}

/* C 1318-1321: "Remove the composing region.  Replace the text between START and
   END in F's selected window with TEXT, then set point to POSITION
   relative to it.  If the mark is active, deactivate it."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn really_replace_text (f: *mut frame, mut start: ptrdiff_t, mut end: ptrdiff_t,
			       text: Lisp_Object, position: ptrdiff_t)
{
  let mut count: specpdl_ref;
  let mut new_start: ptrdiff_t;
  let mut new_end: ptrdiff_t;
  let mut wanted: ptrdiff_t;
  let mut w: *mut window;

  /* If F's old selected window is no longer alive, fail.  */

  if !WINDOW_LIVE_P ((*f).old_selected_window)                             // C 1331
    { return; }

  count = SPECPDL_INDEX ();                                                // C 1334
  record_unwind_protect (Some (restore_selected_window),
			 selected_window);

  /* Make the composition region markers point elsewhere.  */

  if !NILP ((*f).conversion.compose_region_start)                          // C 1340
    {
      Fset_marker ((*f).conversion.compose_region_start, Qnil (), Qnil ());
      Fset_marker ((*f).conversion.compose_region_end, Qnil (), Qnil ());
      (*f).conversion.compose_region_start = Qnil ();
      (*f).conversion.compose_region_end = Qnil ();

      /* Notify the IME of an update to the composition region,
	 inasmuch as the point might not change if START and END are
	 identical and TEXT is empty, among other circumstances.  */

      if !text_interface.is_null ()                                        // C 1351
	  && (*text_interface).compose_region_changed.is_some ()
	{ ((*text_interface).compose_region_changed.unwrap ()) (f); }
    }

  /* Delete the composition region overlay.  */

  if !NILP ((*f).conversion.compose_region_overlay)                        // C 1359
    { Fdelete_overlay ((*f).conversion.compose_region_overlay); }

  /* Temporarily switch to F's selected window at the time of the last
     redisplay.  */
  select_window ((*f).old_selected_window, Qt ());                         // C 1364

  /* Sort START and END by magnitude.  */
  new_start = min (start, end);                                            // C 1367
  new_end   = max (start, end);

  /* Now constrain both to the accessible region.  */

  if new_start < BEGV! ()                                                  // C 1372
    { new_start = BEGV! (); }
  else if new_start > ZV! ()
    { new_start = ZV! (); }

  if new_end < BEGV! ()                                                    // C 1376
    { new_end = BEGV! (); }
  else if new_end > ZV! ()
    { new_end = ZV! (); }

  start = new_start;                                                       // C 1381
  end   = new_end;

  /* This should deactivate the mark.  */
  call0 (Qdeactivate_mark ());                                             // C 1385

  /* Go to start.  */
  set_point (start);                                                       // C 1388

  /* Now delete the text in between, and save PT before TEXT is
     inserted.  */
  del_range_1 (start, end, true, false);                                   // C 1392
  record_buffer_change (start, start, Qt ());
  wanted = PT! ();

  /* So long as TEXT isn't empty, insert it now.  */

  if SCHARS (text) != 0                                                    // C 1398
    {
      /* Insert the new text.  Make sure to inherit text properties
	 from the surroundings: if this doesn't happen, CC Mode
	 fontification might grow confused and become very slow.  */

      insert_from_string (text, 0, 0, SCHARS (text),                       // C 1405-1406
			  SBYTES (text), true);
      record_buffer_change (start, PT! (), text);
    }

  /* Now, move point to the position designated by POSITION.  */

  if position <= 0                                                         // C 1413
    {
      /* C 1415: `if (ckd_add (&wanted, wanted, position) || wanted < BEGV) wanted = BEGV;`  */
      let (ckd_wanted, ckd_overflow) = wanted.overflowing_add (position);
      wanted = ckd_wanted;
      if ckd_overflow || wanted < BEGV! ()
	{ wanted = BEGV! (); }

      if wanted > ZV! ()                                                   // C 1418
	{ wanted = ZV! (); }

      set_point (wanted);
    }
  else
    {
      wanted = PT! ();                                                     // C 1425

      /* C 1427: `if (ckd_add (&wanted, wanted, position - 1) || wanted > ZV) wanted = ZV;`  */
      let (ckd_wanted, ckd_overflow) = wanted.overflowing_add (position - 1);
      wanted = ckd_wanted;
      if ckd_overflow || wanted > ZV! ()
	{ wanted = ZV! (); }

      if wanted < BEGV! ()                                                 // C 1430
	{ wanted = BEGV! (); }

      set_point (wanted);
    }

  /* Print some debugging information.  */
  TEXTCONV_DEBUG! ("text inserted: %s, point now: %td",                    // C 1437
		   SSDATA (text), PT! ());

  /* Update the ephemeral last point.  */
  w = XWINDOW (selected_window);                                           // C 1441
  ephemeral_last_point! (w) = PT! ();
  unbind_to (count, Qnil ());                                              // C 1443
}

/* C 1445: "Complete the edit specified by the counter value inside *TOKEN."  Its address goes to
   `record_unwind_protect_ptr` (C 1553), so it is `unsafe extern "C" fn` (PORTING.md P-12).  */
#[cfg (feature = "have_text_conversion")]
unsafe extern "C" fn complete_edit (token: *mut c_void)
{
  if !text_interface.is_null () && (*text_interface).notify_conversion.is_some ()   // C 1447
    { ((*text_interface).notify_conversion.unwrap ()) (*(token as *mut c_ulong)); }
}

/* C 1455-1456: "Context for complete_edit_check."  Measured: sizeof 16, alignof 8, offsets 0/8.  */
#[cfg (feature = "have_text_conversion")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct complete_edit_check_context
{
  /* The window.  */
  /// C 1460: `struct window *w;`
  pub w: *mut window,

  /* Whether or not editing was successful.  */
  /// C 1463: `bool check;`
  pub check: bool,
}

#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::size_of::<complete_edit_check_context> () == 16);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::align_of::<complete_edit_check_context> () == 8);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (complete_edit_check_context, w) == 0);
#[cfg (feature = "have_text_conversion")]
const _: () = core::assert! (std::mem::offset_of! (complete_edit_check_context, check) == 8);

/* C 1470-1472: "Convert PTR to CONTEXT.  If CONTEXT->check is false, then update
   CONTEXT->w's ephemeral last point and give it to the input method,
   the assumption being that an editing operation signaled."  */
#[cfg (feature = "have_text_conversion")]
unsafe extern "C" fn complete_edit_check (ptr: *mut c_void)
{
  let mut context: *mut complete_edit_check_context;
  let mut f: *mut frame;

  context = ptr as *mut complete_edit_check_context;                       // C 1480

  if !(*context).check                                                  // C 1482
    {
      /* Figure out the new position of point.  */
      ephemeral_last_point! ((*context).w) = window_point ((*context).w);  // C 1485-1486

      /* See if the frame is still alive.  */

      f = WINDOW_XFRAME! ((*context).w);                                   // C 1491

      if !FRAME_LIVE_P (f)                                                 // C 1493
	{ return; }

      if !text_interface.is_null () && (*text_interface).point_changed.is_some ()   // C 1495
	{
	  if (*f).conversion.batch_edit_count > 0
	    { (*f).conversion.batch_edit_flags |= PENDING_POINT_CHANGE; }
	  else
	    {
	      locate_and_save_position_in_field (f, (*context).w, false);
	      ((*text_interface).point_changed.unwrap ()) (f, (*context).w as *mut c_void,
							   std::ptr::null_mut ());
	    }
	}
    }
}

/* C 1504-1507: "Process and free the text conversion ACTION.  F must be the frame
   on which ACTION will be performed.

   Value is the window which was used, or NULL."  */
#[cfg (feature = "have_text_conversion")]
unsafe fn handle_pending_conversion_events_1 (f: *mut frame,
					      action: *mut text_conversion_action)
    -> *mut window
{
  let mut data: Lisp_Object;
  let mut operation: text_conversion_operation;
  let mut buffer: *mut buffer = std::ptr::null_mut ();   // C 1519 (`UNINIT`; PORTING.md 1.13)
  let mut w: *mut window;
  let mut count: specpdl_ref;
  let mut token: c_ulong;
  let mut context: complete_edit_check_context =
    complete_edit_check_context { w: std::ptr::null_mut (), check: false };  // C 1524 (1.13)

  /* Next, process this action and free it.  */

  data = (*action).data;                                                   // C 1528
  operation = (*action).operation;
  token = (*action).counter;
  xfree (action as *mut c_void);

  /* Text conversion events can still arrive immediately after
     `conversion_disabled_p' becomes true.  In that case, process all
     events, but don't perform any associated actions.  */

  if conversion_disabled_p ()                                              // C 1540
    { return std::ptr::null_mut (); }

  /* check is a flag used by complete_edit_check to determine whether
     or not the editing operation completed successfully.  */
  context.check = false;                                                   // C 1547

  /* Make sure completion is signaled.  */
  count = SPECPDL_INDEX ();                                                // C 1550
  record_unwind_protect_ptr (Some (complete_edit), &raw mut token as *mut c_void);
  w = std::ptr::null_mut ();                                               // C 1552

  if WINDOW_LIVE_P ((*f).old_selected_window)                              // C 1554
    {
      w = XWINDOW ((*f).old_selected_window);
      buffer = XBUFFER (WINDOW_BUFFER (w));
      context.w = w;

      /* Notify the input method of any editing failures.  */
      record_unwind_protect_ptr (Some (complete_edit_check),
				 &raw mut context as *mut c_void);
    }

  match operation                                                          // C 1567
    {
      TEXTCONV_START_BATCH_EDIT =>
	{
	  (*f).conversion.batch_edit_count += 1;                           // C 1570
	}

      TEXTCONV_END_BATCH_EDIT =>
	{
	  /* C 1576's `break` is the two labelled blocks below (PORTING.md 2.2: a `break` out of
	     a C `switch` arm has no Rust counterpart, and the statements keep C's order).  */
	  'end_batch_edit: {
	    if (*f).conversion.batch_edit_count > 0                         // C 1573
	      { (*f).conversion.batch_edit_count -= 1; }

	    if !WINDOW_LIVE_P ((*f).old_selected_window)                    // C 1576
	      { break 'end_batch_edit; }

	    if (*f).conversion.batch_edit_flags & PENDING_POINT_CHANGE != 0  // C 1578
	      {
		locate_and_save_position_in_field (f, w, false);
		((*text_interface).point_changed.unwrap ()) (f, w as *mut c_void,
							     buffer as *mut c_void);
	      }

	    if (*f).conversion.batch_edit_flags & PENDING_COMPOSE_CHANGE != 0   // C 1585
	      { ((*text_interface).compose_region_changed.unwrap ()) (f); }

	    (*f).conversion.batch_edit_flags = 0;                           // C 1588
	  }
	}

      TEXTCONV_COMMIT_TEXT =>
	{
	  really_commit_text (f, XFIXNUM (XCAR (data)), XCDR (data));       // C 1592
	}

      TEXTCONV_FINISH_COMPOSING_TEXT =>
	{
	  really_finish_composing_text (f, !NILP (data));                   // C 1596
	}

      TEXTCONV_SET_COMPOSING_TEXT =>
	{
	  really_set_composing_text (f, XFIXNUM (XCAR (data)),              // C 1600-1601
				     XCDR (data));
	}

      TEXTCONV_SET_COMPOSING_REGION =>
	{
	  really_set_composing_region (f, XFIXNUM (XCAR (data)),            // C 1605-1606
				       XFIXNUM (XCDR (data)));
	}

      TEXTCONV_SET_POINT_AND_MARK =>
	{
	  really_set_point_and_mark (f, XFIXNUM (XCAR (data)),              // C 1610-1611
				     XFIXNUM (XCDR (data)));
	}

      TEXTCONV_DELETE_SURROUNDING_TEXT =>
	{
	  really_delete_surrounding_text (f, XFIXNUM (XCAR (data)),         // C 1615-1616
					  XFIXNUM (XCDR (data)));
	}

      TEXTCONV_REQUEST_POINT_UPDATE =>
	{
	  really_request_point_update (f);                                  // C 1620
	}

      TEXTCONV_BARRIER =>
	{
	  if kbd_fetch_ptr != kbd_store_ptr                                 // C 1623
	    { emacs_abort (); }

	  /* Once a barrier is hit, synchronize F's selected window's
	     `ephemeral_last_point' with its current point.  The reason
	     for this is because otherwise a previous keyboard event may
	     have taken place without redisplay happening in between.  */

	  if !w.is_null ()                                                  // C 1632
	    { ephemeral_last_point! (w) = window_point (w); }
	}

      TEXTCONV_REPLACE_TEXT =>
	{
	  really_replace_text (f, XFIXNUM (XCAR (data)),                    // C 1636-1639
			       XFIXNUM (XCAR (XCDR (data))),
			       XCAR (XCDR (XCDR (data))),
			       XFIXNUM (XCAR (XCDR (XCDR (XCDR (data))))));
	}

      _ => { }
    }

  /* Signal success.  */
  context.check = true;                                                    // C 1642
  unbind_to (count, Qnil ());                                              // C 1643

  return w;
}

/* C 1646: "Decrement the variable pointed to by *PTR."  Its address goes to
   `record_unwind_protect_ptr` (C 1685), so it is `unsafe extern "C" fn` (PORTING.md P-12).  */
#[cfg (feature = "have_text_conversion")]
unsafe extern "C" fn decrement_inside (ptr: *mut c_void)
{
  let mut i: *mut c_int;

  i = ptr as *mut c_int;                                                   // C 1652
  *i -= 1;
}

/* C 1655: `static int inside;` - a function-local static in C (C 1667), i.e. one object with
   internal linkage; the port spells it at file scope, which is the same object and lifetime.  */
#[cfg (feature = "have_text_conversion")]
static mut inside: c_int = 0;

/* C 1658-1659: "Process any outstanding text conversion events.
   This may run Lisp or signal."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_pending_conversion_events ()
{
  let mut f: *mut frame;
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut action: *mut text_conversion_action;
  let mut next: *mut text_conversion_action;
  let mut handled: bool;
  let mut count: specpdl_ref;
  let mut last_point: ptrdiff_t;
  let mut w: *mut window;

  handled = false;                                                         // C 1671

  /* Reset Vtext_conversion_edits.  Do not do this if called
     reentrantly.  */

  if inside == 0                                                           // C 1676
    { Vtext_conversion_edits = Qnil (); }

  inside += 1;                                                             // C 1678

  count = SPECPDL_INDEX ();                                                // C 1680
  record_unwind_protect_ptr (Some (decrement_inside), &raw mut inside as *mut c_void);

  FOR_EACH_FRAME! (tail, frame, {                                          // C 1683
    f = XFRAME (frame);
    last_point = -1;
    w = std::ptr::null_mut ();

    /* Test if F has any outstanding conversion events.  Then
       process them in bottom to up order.  */
    loop
      {
	/* Update the input method if handled &&
	   w->ephemeral_last_point != last_point.  */
	if !w.is_null () && last_point != ephemeral_last_point! (w)        // C 1696
	  {
	    if handled
		&& last_point != -1
		&& !text_interface.is_null ()
		&& (*text_interface).point_changed.is_some ()
	      {
		if (*f).conversion.batch_edit_count > 0
		  { (*f).conversion.batch_edit_flags |= PENDING_POINT_CHANGE; }
		else
		  {
		    locate_and_save_position_in_field (f, w, false);
		    ((*text_interface).point_changed.unwrap ()) (f,
								 std::ptr::null_mut (),
								 std::ptr::null_mut ());
		  }
	      }

	    last_point = ephemeral_last_point! (w);                        // C 1712
	  }

	/* Reload action.  This needs to be reentrant as buffer
	   modification functions can call `read-char'.  */
	action = (*f).conversion.actions;                                  // C 1719

	/* If there are no more actions, break.  */

	if action.is_null ()                                               // C 1723
	  { break; }

	/* If action is a barrier event and the keyboard buffer is
	   not yet empty, break out of the loop.  */

	if (*action).operation == TEXTCONV_BARRIER                         // C 1729
	    && kbd_store_ptr != kbd_fetch_ptr
	  { break; }

	/* Unlink this action.  */
	next = (*action).next;                                             // C 1733
	(*f).conversion.actions = next;

	/* Handle and free the action.  */
	w = handle_pending_conversion_events_1 (f, action);                // C 1737
	handled = true;
      }
  });

  unbind_to (count, Qnil ());                                              // C 1741
}

/* C 1742: `#ifdef HAVE_ANDROID`.  `HAVE_ANDROID` is `undef` (macros.tsv 1240), so the arm the port
   compiles is `all (feature = "have_text_conversion", feature = "have_android")`: the file gate
   holds inside as well (the file itself is compiled only where `HAVE_TEXT_CONVERSION` is), and each
   item below carries both.  C 2312's `#endif /* HAVE_ANDROID */` closes it.  */

/* C 1745-1747: "Return the confines of the field to which editing operations on frame
   F should be constrained in *BEG and *END.  Should no field be active,
   set *END to PTRDIFF_MAX."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_conversion_field (f: *mut frame, beg: *mut ptrdiff_t,
					       end: *mut ptrdiff_t)
{
  let mut c1: Lisp_Object;
  let mut c2: Lisp_Object;
  let mut w: *mut window;

  if !NILP ((*f).conversion.field)                                         // C 1754
    {
      c1 = (*f).conversion.field;
      c2 = XCDR (c1);

      if !EQ (XCDR (c2), (*f).old_selected_window)                         // C 1759
	{
	  /* Update this outdated field location.  */
	  w = XWINDOW ((*f).old_selected_window);
	  locate_and_save_position_in_field (f, w, true);
	  get_conversion_field (f, beg, end);
	  return;
	}

      *beg = marker_position (XCAR (c1));                                  // C 1767
      *end = marker_position (XCAR (c2));
      return;
    }

  *beg = 1;                                                                // C 1773
  *end = PTRDIFF_MAX;
}

/* C 1777-1782: "Start a ``batch edit'' in frame F.  During a batch edit,
   point_changed will not be called until the batch edit ends.

   Process the actual operation in the event loop in keyboard.c; then,
   call `notify_conversion' in the text conversion interface with
   COUNTER."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_batch_edit (f: *mut frame, counter: c_ulong)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_START_BATCH_EDIT;                         // C 1787
  (*action).data = Qnil ();
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  /* C 1793: `for (last = &f->conversion.actions; *last; last = &(*last)->next) ;;`  */
  last = &raw mut (*f).conversion.actions;
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                    // C 1797
}

/* C 1801-1806: "End a ``batch edit''.  It is ok to call this function even if a
   batch edit has not yet started, in which case it does nothing.

   COUNTER means the same as in `start_batch_edit'."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn end_batch_edit (f: *mut frame, counter: c_ulong)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_END_BATCH_EDIT;                           // C 1811
  (*action).data = Qnil ();
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  last = &raw mut (*f).conversion.actions;                                 // C 1816
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                    // C 1820
}

/* C 1822-1830: "Insert the specified STRING into frame F's selected-window's
   buffer's composition region, and set point to POSITION relative to
   STRING.

   If there is no composition region, use the active region instead.
   If that doesn't exist either, insert STRING after point.

   COUNTER means the same as in `start_batch_edit'."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn commit_text (f: *mut frame, string: Lisp_Object,
				      position: ptrdiff_t, counter: c_ulong)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_COMMIT_TEXT;                              // C 1836
  (*action).data = Fcons (make_fixnum (position), string);
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  last = &raw mut (*f).conversion.actions;                                 // C 1841
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                    // C 1845
}

/* C 1847-1856: "Remove the composition region and its overlay from frame F's
   selected-window's current buffer.  Leave the text being composed
   intact.

   If UPDATE, call `compose_region_changed' after the region is
   removed.

   COUNTER means the same as in `start_batch_edit'."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn finish_composing_text (f: *mut frame, counter: c_ulong,
						bool_update: bool)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_FINISH_COMPOSING_TEXT;                    // C 1862
  (*action).data = if bool_update { Qt () } else { Qnil () };
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  last = &raw mut (*f).conversion.actions;                                 // C 1867
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                    // C 1871
}

/* C 1873-1888: "Insert the given STRING and make it the currently active
   composition.

   If there is currently no composing or active region, then the new
   value of point is used as the composing region.

   Then, the composing or active region is replaced with the text in
   the specified string.

   Finally, move point to new_point, which is relative to either the
   start or the end of OBJECT depending on whether or not it is less
   than zero.

   COUNTER means the same as in `start_batch_edit'."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_composing_text (f: *mut frame, object: Lisp_Object,
					     new_point: ptrdiff_t, counter: c_ulong)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_SET_COMPOSING_TEXT;                       // C 1894
  (*action).data = Fcons (make_fixnum (new_point),
			  object);
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  last = &raw mut (*f).conversion.actions;                                 // C 1900
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                    // C 1904
}

/* C 1906-1912: "Make the region between START and END the currently active
   ``composing region'' on frame F.  Which of START and END is the
   larger value is not significant.

   The ``composing region'' is a region of text in the buffer that is
   about to undergo editing by the input method."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_composing_region (f: *mut frame, mut start: ptrdiff_t,
					       mut end: ptrdiff_t, counter: c_ulong)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;
  let mut field_start: ptrdiff_t = 0;   // C 1917 (set by get_conversion_field; PORTING.md 1.13)
  let mut field_end: ptrdiff_t = 0;     // C 1917
  let mut temp: ptrdiff_t;

  if start > end                                                            // C 1919
    {
      temp  = end;
      end   = start;
      start = temp;
    }

  get_conversion_field (f, &raw mut field_start, &raw mut field_end);       // C 1925
  start = min (start + field_start - 1, MOST_POSITIVE_FIXNUM);              // C 1926
  end = max (start, min (end + field_start - 1, field_end));                // C 1927

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_SET_COMPOSING_REGION;                      // C 1930
  (*action).data = Fcons (make_fixnum (start), make_fixnum (end));
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  last = &raw mut (*f).conversion.actions;                                  // C 1935
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                     // C 1939
}

/* C 1941-1944: "Move point in frame F's selected-window's buffer to POINT and maybe
   push MARK.

   COUNTER means the same as in `start_batch_edit'."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn textconv_set_point_and_mark (f: *mut frame, mut point: ptrdiff_t,
						      mut mark: ptrdiff_t, counter: c_ulong)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;
  let mut field_start: ptrdiff_t = 0;   // C 1948 (PORTING.md 1.13)
  let mut field_end: ptrdiff_t = 0;     // C 1948

  get_conversion_field (f, &raw mut field_start, &raw mut field_end);       // C 1951
  point = min (max (point + field_start - 1, field_start),                  // C 1952
	       field_end);
  mark = min (max (mark + field_start - 1, field_start),                    // C 1954
	      field_end);

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_SET_POINT_AND_MARK;                        // C 1958
  (*action).data = Fcons (make_fixnum (point),
			  make_fixnum (mark));
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  last = &raw mut (*f).conversion.actions;                                  // C 1964
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                     // C 1968
}

/* C 1970-1971: "Delete LEFT and RIGHT characters around point in frame F's old
   selected window."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_surrounding_text (f: *mut frame, left: ptrdiff_t,
						  right: ptrdiff_t, counter: c_ulong)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_DELETE_SURROUNDING_TEXT;                   // C 1978
  (*action).data = Fcons (make_fixnum (left),
			  make_fixnum (right));
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  last = &raw mut (*f).conversion.actions;                                  // C 1983
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                     // C 1987
}

/* C 1989-1991: "Request an immediate call to TEXT_INTERFACE->point_changed with the
   new details of frame F's region unless a batch edit is in
   progress."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn request_point_update (f: *mut frame, counter: c_ulong)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_REQUEST_POINT_UPDATE;                     // C 1997
  (*action).data = Qnil ();
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  last = &raw mut (*f).conversion.actions;                                 // C 2002
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                    // C 2006
}

/* C 2008-2014: "Request that text conversion on frame F pause until the keyboard
   buffer becomes empty.

   Use this function to ensure that edits associated with a keyboard
   event complete before the text conversion edits after the barrier
   take place."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn textconv_barrier (f: *mut frame, counter: c_ulong)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_BARRIER;                                  // C 2020
  (*action).data = Qnil ();
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  last = &raw mut (*f).conversion.actions;                                 // C 2025
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                    // C 2029
}

/* C 2031-2038: "Remove the composing region.  Replace the text between START and END
   (whose order, as in `set_composing_region', is not significant)
   within F's selected window with TEXT; deactivate the mark if it is
   active.  Subsequently, set point to POSITION relative to TEXT, as
   `commit_text' would."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn replace_text (f: *mut frame, mut start: ptrdiff_t, mut end: ptrdiff_t,
				       text: Lisp_Object, position: ptrdiff_t,
				       counter: c_ulong)
{
  let mut action: *mut text_conversion_action;
  let mut last: *mut *mut text_conversion_action;
  let mut field_start: ptrdiff_t = 0;   // C 2046 (PORTING.md 1.13)
  let mut field_end: ptrdiff_t = 0;     // C 2046
  let mut temp: ptrdiff_t;

  if start > end                                                           // C 2049
    {
      temp  = end;
      end   = start;
      start = temp;
    }

  get_conversion_field (f, &raw mut field_start, &raw mut field_end);      // C 2055
  start = min (start + field_start - 1, MOST_POSITIVE_FIXNUM);             // C 2056
  end = max (start, min (end + field_start - 1, field_end));               // C 2057

  action = xmalloc (core::mem::size_of::<text_conversion_action> ()) as *mut text_conversion_action;
  (*action).operation = TEXTCONV_REPLACE_TEXT;                             // C 2060
  (*action).data = list4 (make_fixnum (start), make_fixnum (end),
			  text, make_fixnum (position));
  (*action).next = std::ptr::null_mut ();
  (*action).counter = counter;
  last = &raw mut (*f).conversion.actions;                                 // C 2066
  while !(*last).is_null ()
    { last = &raw mut (**last).next; }
  *last = action;
  input_pending = true;                                                    // C 2070
}

/* C 2072-2087: "Return N characters of text around point in frame F's old selected
   window.

   If N is -1, return the text between point and mark instead, given
   that the mark is active.

   Set *START_RETURN to the position of the first character returned,
   *START_OFFSET to the offset of the lesser of mark and point within
   that text, *END_OFFSET to the greater of mark and point within that
   text, and *LENGTH to the actual number of characters returned,
   *BYTES to the actual number of bytes returned, and *MARK_ACTIVE to
   whether or not the mark is active.

   Value is NULL upon failure, and a malloced string upon success."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_extracted_text (f: *mut frame, mut n: ptrdiff_t,
					     start_return: *mut ptrdiff_t,
					     start_offset: *mut ptrdiff_t,
					     end_offset: *mut ptrdiff_t,
					     length: *mut ptrdiff_t,
					     bytes: *mut ptrdiff_t,
					     mark_active: *mut bool) -> *mut c_char
{
  let mut count: specpdl_ref;
  let mut start: ptrdiff_t;
  let mut end: ptrdiff_t;
  let mut start_byte: ptrdiff_t = 0;   // C 2095 (assigned before use; PORTING.md 1.13)
  let mut end_byte: ptrdiff_t = 0;     // C 2095
  let mut mark: ptrdiff_t;
  let mut buffer: *mut c_char;
  let mut field_start: ptrdiff_t = 0;   // C 2095 (PORTING.md 1.13)
  let mut field_end: ptrdiff_t = 0;     // C 2095

  if !WINDOW_LIVE_P ((*f).old_selected_window)                             // C 2099
    { return std::ptr::null_mut (); }

  /* Save the excursion, as there will be extensive changes to the
     selected window.  */
  count = SPECPDL_INDEX ();                                                // C 2104
  record_unwind_protect_excursion ();
  record_unwind_protect (Some (restore_selected_window), selected_window);

  /* Inhibit quitting.  */
  specbind (Qinhibit_quit (), Qt ());                                      // C 2110

  /* Temporarily switch to F's selected window at the time of the last
     redisplay.  */
  select_window ((*f).old_selected_window, Qt ());                         // C 2114
  buffer = std::ptr::null_mut ();                                          // C 2115

  /* Figure out the bounds of the text to return.  */
  'finish: {
    if n != -1                                                             // C 2118
      {
	/* Make sure n is at least 4, leaving two characters around
	   PT.  */
	n = max (4, n);                                                    // C 2122

	start = PT! () - n / 2;                                            // C 2124
	end = PT! () + n - n / 2;
      }
    else
      {
	if !NILP (BVAR! (current_buffer, mark_active_))                    // C 2129
	    && !(*XMARKER (BVAR! (current_buffer, mark_))).buffer.is_null ()
	  {
	    start = marker_position (BVAR! (current_buffer, mark_));       // C 2132
	    end = PT! ();

	    /* Sort start and end.  start_byte is used to hold a
	       temporary value.  */

	    if start > end                                                 // C 2140
	      {
		start_byte = end;
		end = start;
		start = start_byte;
	      }
	  }
	else
	  { break 'finish; }
      }


    /* Narrow to the field, if any.  */
    if !NILP ((*f).conversion.field)                                       // C 2151
      {
	record_unwind_protect (Some (save_restriction_restore),
			       save_restriction_save ());
	Fnarrow_to_region (XCAR ((*f).conversion.field),
			   XCAR (XCDR ((*f).conversion.field)));
      }

    start = max (start, BEGV! ());                                         // C 2158
    end = min (end, ZV! ());

    /* Detect overflow.  */

    if !(start <= PT! () && PT! () <= end)                                 // C 2163
      { break 'finish; }

    /* Convert the character positions to byte positions.  */
    start_byte = CHAR_TO_BYTE (start);                                     // C 2167
    end_byte = CHAR_TO_BYTE (end);

    /* Extract the text from the buffer.  */
    buffer = xmalloc ((end_byte - start_byte) as usize) as *mut c_char;    // C 2171
    copy_buffer_text (start, start_byte, end, end_byte, buffer);

    /* Get the mark.  If it's not active, use PT.  */

    mark = get_mark ();                                                    // C 2176
    *mark_active = true;

    if mark == -1                                                          // C 2179
      {
	mark = PT! ();
	*mark_active = false;
      }

    /* Return the offsets.  */
    get_conversion_field (f, &raw mut field_start, &raw mut field_end);    // C 2187
    *start_return = max (1, start - field_start + 1);                      // C 2188
    *start_offset = min (mark - start, PT! () - start);
    *end_offset = max (mark - start, PT! () - start);
    *length = end - start;
    *bytes = end_byte - start_byte;

    TEXTCONV_DEBUG! ("get_extracted_text: PT, mark, start: %td, %td, %td",  // C 2195
		     PT! (), mark, start);
  }
  /* finish: */

  unbind_to (count, Qnil ());                                              // C 2199
  return buffer;
}

/* C 2201-2210: "Return the text between the positions pt - LEFT and pt + RIGHT,
   where pt is the position of point in frame F's selected window.  If
   the mark is active, return the range of text relative to the bounds
   of the region instead.

   Set *LENGTH to the number of characters returned, *BYTES to the
   number of bytes returned, *OFFSET to the character position of the
   returned text, and *START_RETURN and *END_RETURN to the mark and
   point relative to that position."  */
#[cfg (all (feature = "have_text_conversion", feature = "have_android"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_surrounding_text (f: *mut frame, left: ptrdiff_t,
					       right: ptrdiff_t, length: *mut ptrdiff_t,
					       bytes: *mut ptrdiff_t, offset: *mut ptrdiff_t,
					       start_return: *mut ptrdiff_t,
					       end_return: *mut ptrdiff_t) -> *mut c_char
{
  let mut count: specpdl_ref;
  let mut start: ptrdiff_t;
  let mut end: ptrdiff_t;
  let mut start_byte: ptrdiff_t = 0;   // C 2218 (assigned before use; PORTING.md 1.13)
  let mut end_byte: ptrdiff_t = 0;     // C 2218
  let mut mark: ptrdiff_t;
  let mut temp: ptrdiff_t;
  let mut field_start: ptrdiff_t = 0;   // C 2218 (PORTING.md 1.13)
  let mut field_end: ptrdiff_t = 0;     // C 2218
  let mut buffer: *mut c_char;

  if !WINDOW_LIVE_P ((*f).old_selected_window)                             // C 2223
    { return std::ptr::null_mut (); }

  /* Save the excursion, as there will be extensive changes to the
     selected window.  */
  count = SPECPDL_INDEX ();                                                // C 2228
  record_unwind_protect_excursion ();
  record_unwind_protect (Some (restore_selected_window), selected_window);

  /* Inhibit quitting.  */
  specbind (Qinhibit_quit (), Qt ());                                      // C 2234

  /* Temporarily switch to F's selected window at the time of the last
     redisplay.  */
  select_window ((*f).old_selected_window, Qt ());                         // C 2238
  buffer = std::ptr::null_mut ();                                          // C 2239

  /* Figure out the bounds of the text to return.  */

  'finish: {
    /* First, obtain start and end.  */
    end = get_mark ();                                                     // C 2245
    start = PT! ();                                                        // C 2246

    /* If the mark is not active, make it start and end.  */

    if end == -1                                                           // C 2249
      { end = start; }

    /* Now sort start and end.  */

    if end < start                                                         // C 2253
      {
	temp = start;
	start = end;
	end = temp;
      }

    /* And subtract left and right.  */

    /* C 2262-2263: `if (ckd_sub (&start, start, left) || ckd_add (&end, end, right))
	 goto finish;` - both sums are stored either way (the `src/cmds.rs` 469 shape).  */
    {
      let (ckd_start, ckd_sub_overflow) = start.overflowing_sub (left);
      start = ckd_start;
      let (ckd_end, ckd_add_overflow) = end.overflowing_add (right);
      end = ckd_end;
      if ckd_sub_overflow || ckd_add_overflow
	{ break 'finish; }
    }

    /* Narrow to the field, if any.  */
    if !NILP ((*f).conversion.field)                                       // C 2267
      {
	record_unwind_protect (Some (save_restriction_restore),
			       save_restriction_save ());
	Fnarrow_to_region (XCAR ((*f).conversion.field),
			   XCAR (XCDR ((*f).conversion.field)));
      }

    start = max (start, BEGV! ());                                         // C 2275
    end = min (end, ZV! ());

    /* Detect overflow.  */

    if !(start <= PT! () && PT! () <= end)                                 // C 2280
      { break 'finish; }

    /* Convert the character positions to byte positions.  */
    start_byte = CHAR_TO_BYTE (start);                                     // C 2284
    end_byte = CHAR_TO_BYTE (end);

    /* Extract the text from the buffer.  */
    buffer = xmalloc ((end_byte - start_byte) as usize) as *mut c_char;    // C 2288
    copy_buffer_text (start, start_byte, end, end_byte, buffer);

    /* Get the mark.  If it's not active, use PT.  */

    mark = get_mark ();                                                    // C 2293

    if mark == -1                                                          // C 2295
      { mark = PT! (); }

    /* Return the offsets.  Unlike `get_extracted_text', this need not
       sort mark and point.  */

    get_conversion_field (f, &raw mut field_start, &raw mut field_end);    // C 2301
    *offset = max (1, start - field_start + 1);                            // C 2302
    *start_return = mark - start;
    *end_return = PT! () - start;
    *length = end - start;
    *bytes = end_byte - start_byte;
  }
  /* finish: */

  unbind_to (count, Qnil ());                                              // C 2309
  return buffer;
}
/* C 2312: `#endif /* HAVE_ANDROID */`  */

/* C 2314-2316: "Return whether or not text conversion is temporarily disabled.
   `reset' should always call this to determine whether or not to
   disable the input method."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn conversion_disabled_p () -> bool
{
  return suppress_conversion_count > 0;                                    // C 2320
}

/* C 2324-2326: "Window system interface.  These are called from the rest of
   Emacs."  */

/* C 2328-2329: "Notice that frame F's selected window has been set from redisplay.
   Reset F's input method state."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn report_selected_window_change (f: *mut frame)
{
  let mut w: *mut window;

  reset_frame_state (f);                                                   // C 2337

  if text_interface.is_null ()                                             // C 2339
    { return; }

  /* When called from window.c, F's selected window has already been
     redisplayed, but w->last_point has not yet been updated.  Update
     it here to avoid race conditions when the IM asks for the initial
     selection position immediately after.  */

  if WINDOWP ((*f).selected_window)                                        // C 2347
    {
      w = XWINDOW ((*f).selected_window);
      ephemeral_last_point! (w) = window_point (w);
    }

  ((*text_interface).reset.unwrap ()) (f);                                 // C 2353
}

/* C 2356-2363: "Notice that point in frame F's selected window's current buffer has
   changed.

   F is the frame whose selected window was changed, WINDOW is the
   window in question, and BUFFER is that window's buffer.

   Tell the text conversion interface about the change; it will likely
   pass the information on to the system input method."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn report_point_change (f: *mut frame, window: *mut window,
					      buffer: *mut buffer)
{
  if text_interface.is_null ()                                             // C 2367
    || (*text_interface).point_changed.is_none ()
    { return; }

  if (*f).conversion.batch_edit_count > 0                                  // C 2372
    { (*f).conversion.batch_edit_flags |= PENDING_POINT_CHANGE; }
  else
    {
      locate_and_save_position_in_field (f, window, false);
      ((*text_interface).point_changed.unwrap ()) (f, window as *mut c_void,
						  buffer as *mut c_void);
    }
}

/* C 2381-2382: "Temporarily disable text conversion.  Must be paired with a
   corresponding call to resume_text_conversion."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn disable_text_conversion ()
{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut f: *mut frame;

  suppress_conversion_count += 1;                                          // C 2390

  if text_interface.is_null () || suppress_conversion_count > 1            // C 2392
    { return; }

  /* Loop through and reset the input method on each window system
     frame.  It should call conversion_disabled_p and then DTRT.  */

  FOR_EACH_FRAME! (tail, frame, {                                          // C 2398
    f = XFRAME (frame);
    reset_frame_state (f);

    if FRAME_WINDOW_P (f) && FRAME_VISIBLE_P (f)
      { ((*text_interface).reset.unwrap ()) (f); }
  });
}

/* C 2408: "Undo the effect of the last call to `disable_text_conversion'."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn resume_text_conversion ()
{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut f: *mut frame;

  suppress_conversion_count -= 1;                                          // C 2416
  eassert! (suppress_conversion_count >= 0);                               // C 2417

  if text_interface.is_null () || suppress_conversion_count != 0           // C 2419
    { return; }

  /* Loop through and reset the input method on each window system
     frame.  It should call conversion_disabled_p and then DTRT.  */

  FOR_EACH_FRAME! (tail, frame, {                                          // C 2425
    f = XFRAME (frame);
    reset_frame_state (f);

    if FRAME_WINDOW_P (f) && FRAME_VISIBLE_P (f)
      { ((*text_interface).reset.unwrap ()) (f); }
  });
}

/* C 2435: "Register INTERFACE as the text conversion interface."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn register_textconv_interface (interface: *mut textconv_interface)
{
  text_interface = interface;                                              // C 2439
}

/* C 2445-2446: "List of buffers whose text conversion state will be reset after a
   key sequence is read."  */
#[cfg (feature = "have_text_conversion")]
static mut postponed_buffers: Lisp_Object = lisp_h_Qnil ();

/* C 2449-2451: "Reset the text conversion style of each frame whose selected buffer
   is contained inside `postponed_buffers'.  Set `postponed_buffers'
   to nil."  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_postponed_buffers ()
{
  let mut buffer: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut b: *mut buffer;
  let mut f: *mut frame;

  buffer = postponed_buffers;                                              // C 2460
  postponed_buffers = Qnil ();

  if (*text_interface).reset.is_none ()                                    // C 2463
    { return; }

  FOR_EACH_TAIL! (buffer, {
    /* C's `continue` jumps to the `for` increment; the port's labelled block puts that
       increment after the block, so it still runs (`src/androidterm.rs` 279-283).  */
    'continue_: {
      b = XBUFFER (XCAR (buffer));

      /* Continue if this is a dead buffer.  */

      if !BUFFER_LIVE_P (b)                                                // C 2472
	{ break 'continue_; }

      /* If no windows are displaying B anymore, continue.  */

      if buffer_window_count (b) == 0                                      // C 2477
	{ break 'continue_; }

      /* Look for frames which have B selected.  */

      FOR_EACH_FRAME! (tail, frame, {                                      // C 2482
	f = XFRAME (frame);

	if WINDOW_LIVE_P ((*f).old_selected_window)
	    && FRAME_WINDOW_P (f)
	    /* N.B. that the same frame can't be reset twice as long
	       as the list of buffers remains unique.  */
	    && EQ ((*XWINDOW ((*f).old_selected_window)).contents,
		   XCAR (buffer))
	  {
	    block_input ();                                                // C 2494
	    reset_frame_state (f);
	    ((*text_interface).reset.unwrap ()) (f);
	    unblock_input ();
	  }
      });
    }
  });
}

/* C 2502: "Lisp interface."  */

/* C 2504-2529.  The docstring is C's text, byte for byte (the `docstring` gate compares it with
   the reference).  */
#[cfg (feature = "have_text_conversion")]
DEFUN! ("set-text-conversion-style", Fset_text_conversion_style,
	Sset_text_conversion_style, 1, 2, 0,
	doc: "Set the current buffer's text conversion style to VALUE.\n\nAfter setting `text-conversion-style', force input methods\nediting in a selected window displaying this buffer on any frame\nto stop themselves.\n\nThis can lead to a significant amount of time being taken by the input\nmethod resetting itself, so you should not use this function lightly;\ninstead, set `text-conversion-style' before your buffer is displayed,\nand let redisplay manage the input method appropriately.\n\nIf a key sequence is currently being read (either through the command\nloop or by a call to `read-key-sequence') and AFTER-KEY-SEQUENCE is\nnon-nil, don't perform changes to the input method until the key\nsequence is read.  This is useful within a function bound to\n`input-decode-map' or `local-function-key-map', as it prevents the\ninput method from being redundantly enabled according to VALUE if the\nreplacement key sequence returned starts a new key sequence and makes\n`read-key-sequence' disable text conversion again.",
	(value: Lisp_Object, after_key_sequence: Lisp_Object),
	{
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut f: *mut frame;
  let mut buffer: Lisp_Object;

  bset_text_conversion_style (current_buffer, value);                      // C 2531

  if text_interface.is_null ()                                             // C 2533
    { return Qnil (); }

  /* If there are any selected windows displaying this buffer, reset
     text conversion on their associated frames.  */

  if buffer_window_count (current_buffer) != 0                             // C 2539
    {
      buffer = Fcurrent_buffer ();

      /* Postpone changes to the actual text conversion state if
	 AFTER_KEY_SEQUENCE is non-nil and a key sequence is being
	 read.  */

      if reading_key_sequence && !NILP (after_key_sequence)                // C 2547
	{
	  if NILP (Fmemq (buffer, postponed_buffers))                      // C 2550
	    /* `check_postponed_buffers' will hopefully be called soon
	       enough to avoid postponed_buffers growing
	       indefinitely.  */
	    { postponed_buffers = Fcons (buffer, postponed_buffers); }
	  return Qnil ();
	}

      FOR_EACH_FRAME! (tail, frame, {                                      // C 2557
	f = XFRAME (frame);

	if WINDOW_LIVE_P ((*f).old_selected_window)
	    && FRAME_WINDOW_P (f)
	    && (EQ ((*XWINDOW ((*f).old_selected_window)).contents,
		    buffer)
		/* Always reset the text conversion style of the
		   selected frame.  */
		|| f == SELECTED_FRAME ())
	  {
	    block_input ();                                                // C 2570
	    reset_frame_state (f);
	    ((*text_interface).reset.unwrap ()) (f);
	    unblock_input ();
	  }
      });
    }

  return Qnil ();                                                          // C 2577
});

/* C 2581: `void syms_of_textconv (void)`.  The three `DEFVAR_LISP` variables this C file owns:
   C's storage is `globals.f_<vname>` (lisp.h 3495-3530) and `struct emacs_globals globals` is
   defined once, in alloc.c; the port's `include/globals.rs` mirrors the names as declarations, so
   the unit that owns one defines it here (status 3.29, the `src/syntax.rs` shape).  */
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub static mut Vtext_conversion_edits: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub static mut Voverriding_text_conversion_style: Lisp_Object = lisp_h_Qnil ();
#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub static mut Vtext_conversion_face: Lisp_Object = lisp_h_Qnil ();

#[cfg (feature = "have_text_conversion")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_textconv ()
{
  DEFSYM! (Qaction, "action");                                             // C 2585
  DEFSYM! (Qpassword, "password");
  DEFSYM! (Qtext_conversion, "text-conversion");
  DEFSYM! (Qpush_mark, "push-mark");
  DEFSYM! (Qunderline, "underline");
  DEFSYM! (Qoverriding_text_conversion_style,
	   "overriding-text-conversion-style");

  DEFVAR_LISP! (c"text-conversion-edits", Vtext_conversion_edits,
    "List of buffers last edited as a result of text conversion.\n\nThis list can be used while handling a `text-conversion' event to\ndetermine which changes have taken place.\n\nEach element of the list describes a single edit in a buffer, and\nis of the form:\n\n    (BUFFER BEG END EPHEMERAL)\n\nIf an insertion or an edit to the buffer text is described, then BEG\nand END are markers which denote the bounds of the text that was\nchanged or inserted.  If a deletion is described, then BEG and END are\nthe same object.\n\nIf EPHEMERAL is t, then the input method is preparing to make further\nedits to the text, so any actions that would otherwise be taken, such\nas indenting or automatically filling text, should not take place.\n\nOtherwise, it is either a string containing text that was inserted,\ntext deleted before point, or nil if text was deleted after point.\n\nThe list contents are arranged in the reverse of the order of editing,\ni.e. latest edit first, so you must iterate through the list in\nreverse.");
  Vtext_conversion_edits = Qnil ();                                        // C 2622

  DEFVAR_LISP! (c"overriding-text-conversion-style",
	       Voverriding_text_conversion_style,
    "Non-buffer local version of `text-conversion-style'.\n\nIf this variable is the symbol `lambda', it means to consult the\nbuffer-local value of `text-conversion-style' to determine whether or\nnot to activate the input method.  Otherwise, the value is used in\npreference to any buffer-local value of `text-conversion-style'.");
  Voverriding_text_conversion_style = Qlambda ();                          // C 2636

  DEFVAR_LISP! (c"text-conversion-face", Vtext_conversion_face,
    "Face in which to display temporary edits by an input method.\nThe value nil means to display no indication of a temporary edit.");
  Vtext_conversion_face = Qunderline ();                                   // C 2638

  defsubr (&raw mut Sset_text_conversion_style);                           // C 2636

  postponed_buffers = Qnil ();                                             // C 2638
  staticpro (&raw const postponed_buffers);                                // C 2639
}
