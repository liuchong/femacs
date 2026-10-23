// femacs: port of src/textconv.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1): records, enums, one constant and declarations.  `textconv.h` is
// the string-conversion interface the GUI backends register against (`src/xterm.c` 32386 and
// `src/androidterm.c` 6652 both build a `struct textconv_interface` and call
// `register_textconv_interface`), and its live text is what C compiles on this target: the file's
// only conditional is C 129's `#ifdef HAVE_ANDROID`, and `HAVE_ANDROID` is `undef` in
// `docs/baseline/config/darwin-arm64.macros.tsv`.  `src/textconv.c` defines every declaration in
// the non-Android block, so those sit in `pub mod textconv_decls` with a glob import (P-17/P-21:
// a same-named definition in the unit shadows the glob import, PORTING.md 0.1-10).
//
// C 20/163's guard is `#ifndef _TEXTCONV_H_` with **no** `#define _TEXTCONV_H_` anywhere in the
// file (measured: `rg -n '_TEXTCONV_H_' emacs/src/textconv.h` finds only the `#ifndef` and its
// `#endif` comment, and `tools/extract-abi.py` therefore inventories no such macro), so C re-reads
// the whole body on every include; there is no macro for the port to mirror (PORTING.md 0.1-4).
//
// `struct textconv_interface`'s four function pointers are `Option<unsafe extern "C" fn …>`, the
// `include/termhooks.rs` 222-310 reading of a C callback field (P-27: a C function pointer is
// nullable).
//
// Degraded names (PORTING.md P-55/P-72 rule 3): `struct window`, `struct buffer` and
// `struct frame` are named only by pointer here, and of the three the header includes only
// `frame.h` (C 23), so `struct frame *` is named - `include/frame.rs` owns that record - while the
// two the header does not include keep the untyped pointer with C's prototype in a `/// C:` line.

include!("lisp.rs");                 // C 22: `#include "lisp.h"`
include!("frame.rs");                // C 23: `#include "frame.h"` (`struct frame`)

/* C 25-52: "The function pointers in this structure should be filled out by each GUI backend
   interested in supporting text conversion."  Measured with the frozen clang
   (`clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim -I build/emacs-ref/src
   -I emacs/src -I emacs/lib` on a TU that includes `textconv.h`): sizeof 32, alignof 8, offsets
   0/8/16/24.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct textconv_interface
{
  /* Notice that the text conversion context has changed (which can
     happen if the window is deleted or switches buffers, or an
     unexpected buffer change occurs.) */
  /// C 36: `void (*reset) (struct frame *);`
  pub reset: Option<unsafe extern "C" fn (*mut frame)>,

  /* Notice that point or mark has moved in the specified frame's
     selected window's selected buffer.  The second argument is the
     window whose point changed, and the third argument is the
     buffer.  */
  /// C 42-43: `void (*point_changed) (struct frame *, struct window *, struct buffer *);`
  pub point_changed: Option<unsafe extern "C" fn (*mut frame, *mut c_void, *mut c_void)>,

  /* Notice that the preconversion region has changed without point
     being moved.  */
  /// C 47: `void (*compose_region_changed) (struct frame *);`
  pub compose_region_changed: Option<unsafe extern "C" fn (*mut frame)>,

  /* Notice that an asynch conversion identified by COUNTER has
     completed.  */
  /// C 51: `void (*notify_conversion) (unsigned long);` - `unsigned long` is `c_ulong`.
  pub notify_conversion: Option<unsafe extern "C" fn (c_ulong)>,
}

const _: () = core::assert! (std::mem::size_of::<textconv_interface> () == 32);
const _: () = core::assert! (std::mem::align_of::<textconv_interface> () == 8);
const _: () = core::assert! (std::mem::offset_of! (textconv_interface, reset) == 0);
const _: () = core::assert! (std::mem::offset_of! (textconv_interface, point_changed) == 8);
const _: () = core::assert! (std::mem::offset_of! (textconv_interface, compose_region_changed) == 16);
const _: () = core::assert! (std::mem::offset_of! (textconv_interface, notify_conversion) == 24);

/* C 56-69: `enum textconv_caret_direction` - no explicit values, so the members are declaration
   order and the port spells them as constants over C's own type name (PORTING.md 1.3, the
   `include/dispextern.rs` 66-75 `text_cursor_kinds` shape).  C's enum is 4 bytes.  */
pub type textconv_caret_direction = c_int;
pub const TEXTCONV_FORWARD_CHAR: textconv_caret_direction = 0;
pub const TEXTCONV_BACKWARD_CHAR: textconv_caret_direction = 1;
pub const TEXTCONV_FORWARD_WORD: textconv_caret_direction = 2;
pub const TEXTCONV_BACKWARD_WORD: textconv_caret_direction = 3;
pub const TEXTCONV_CARET_UP: textconv_caret_direction = 4;
pub const TEXTCONV_CARET_DOWN: textconv_caret_direction = 5;
pub const TEXTCONV_NEXT_LINE: textconv_caret_direction = 6;
pub const TEXTCONV_PREVIOUS_LINE: textconv_caret_direction = 7;
pub const TEXTCONV_LINE_START: textconv_caret_direction = 8;
pub const TEXTCONV_LINE_END: textconv_caret_direction = 9;
pub const TEXTCONV_ABSOLUTE_POSITION: textconv_caret_direction = 10;

/* C 71-75: `enum textconv_operation`.  */
pub type textconv_operation = c_int;
pub const TEXTCONV_SUBSTITUTION: textconv_operation = 0;
pub const TEXTCONV_RETRIEVAL: textconv_operation = 1;

/* C 77-88: "Structure describing text in a buffer corresponding to a ``struct
   textconv_callback_struct''."  Measured: sizeof 24, alignof 8, offsets 0/8/16.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct textconv_conversion_text
{
  /* Length of the text in characters and bytes.  */
  /// C 83: `ptrdiff_t length, bytes;`
  pub length: ptrdiff_t,
  pub bytes: ptrdiff_t,

  /* Pointer to the text data.  This must be deallocated by the
     caller.  */
  /// C 87: `char *text;`
  pub text: *mut c_char,
}

const _: () = core::assert! (std::mem::size_of::<textconv_conversion_text> () == 24);
const _: () = core::assert! (std::mem::align_of::<textconv_conversion_text> () == 8);
const _: () = core::assert! (std::mem::offset_of! (textconv_conversion_text, length) == 0);
const _: () = core::assert! (std::mem::offset_of! (textconv_conversion_text, bytes) == 8);
const _: () = core::assert! (std::mem::offset_of! (textconv_conversion_text, text) == 16);

/* C 90-119: "Structure describing a single query submitted by the input method."  Measured:
   sizeof 48, alignof 8, offsets 0/8/12/16/24.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct textconv_callback_struct
{
  /* Character position, relative to the current spot location, from
     where on text should be returned.  */
  /// C 97: `EMACS_INT position;`
  pub position: EMACS_INT,

  /* The type of scanning to perform to determine either the start or
     the end of the conversion.  */
  /// C 101: `enum textconv_caret_direction direction;`
  pub direction: textconv_caret_direction,

  /* The number of times for which to repeat the scanning in order to
     determine the starting position of the text to return.  */
  /// C 105: `unsigned short factor;`
  pub factor: c_ushort,

  /* The operation to perform upon the current buffer contents. … (C 107-113) */
  /// C 114: `enum textconv_operation operation;`
  pub operation: textconv_operation,

  /* Structure that will be filled with a description of the resulting
     text.  */
  /// C 118: `struct textconv_conversion_text text;` - this header's own record, by value.
  pub text: textconv_conversion_text,
}

const _: () = core::assert! (std::mem::size_of::<textconv_callback_struct> () == 48);
const _: () = core::assert! (std::mem::align_of::<textconv_callback_struct> () == 8);
const _: () = core::assert! (std::mem::offset_of! (textconv_callback_struct, position) == 0);
const _: () = core::assert! (std::mem::offset_of! (textconv_callback_struct, direction) == 8);
const _: () = core::assert! (std::mem::offset_of! (textconv_callback_struct, factor) == 12);
const _: () = core::assert! (std::mem::offset_of! (textconv_callback_struct, operation) == 16);
const _: () = core::assert! (std::mem::offset_of! (textconv_callback_struct, text) == 24);

/// C 123: `#define TEXTCONV_SKIP_CONVERSION_REGION (1 << 0)` - the value is compared as C's
/// token pair (PORTING.md P-44's expression rule); the name is a constant, not a macro, because C
/// uses it as one (`include/w32font.rs` 36-38's `NTMFLAGS_OPENTYPE` reading).
pub const TEXTCONV_SKIP_CONVERSION_REGION: c_int = 1 << 0;

/* C 125-159: the entry points `src/textconv.c` defines.  The Android half (C 129-157) is dead on
   this target and carries `#[cfg (feature = "have_android")]` - P-51's arm for the non-frozen
   side; every parameter there is a name this mirror can spell.  */
pub mod textconv_decls {
  use super::*;

  unsafe extern "C" {
    /// C 125-126: `extern int textconv_query (struct frame *, struct textconv_callback_struct *,
    /// int);`
    pub fn textconv_query (f: *mut frame, query: *mut textconv_callback_struct,
                           flags: c_int) -> c_int;
    /// C 127: `extern bool detect_conversion_events (void);`
    pub fn detect_conversion_events () -> bool;
    /// C 128: `extern void handle_pending_conversion_events (void);`
    pub fn handle_pending_conversion_events ();

    #[cfg (feature = "have_android")]
    /// C 130: `extern void start_batch_edit (struct frame *, unsigned long);`
    pub fn start_batch_edit (f: *mut frame, counter: c_ulong);
    #[cfg (feature = "have_android")]
    /// C 131: `extern void end_batch_edit (struct frame *, unsigned long);`
    pub fn end_batch_edit (f: *mut frame, counter: c_ulong);
    #[cfg (feature = "have_android")]
    /// C 132-133: `extern void commit_text (struct frame *, Lisp_Object, ptrdiff_t,
    /// unsigned long);`
    pub fn commit_text (f: *mut frame, text: Lisp_Object, len: ptrdiff_t, counter: c_ulong);
    #[cfg (feature = "have_android")]
    /// C 134-135: `extern void finish_composing_text (struct frame *, unsigned long, bool);`
    pub fn finish_composing_text (f: *mut frame, counter: c_ulong, cancel: bool);
    #[cfg (feature = "have_android")]
    /// C 136-137: `extern void set_composing_text (struct frame *, Lisp_Object, ptrdiff_t,
    /// unsigned long);`
    pub fn set_composing_text (f: *mut frame, text: Lisp_Object, len: ptrdiff_t,
                               counter: c_ulong);
    #[cfg (feature = "have_android")]
    /// C 138-139: `extern void set_composing_region (struct frame *, ptrdiff_t, ptrdiff_t,
    /// unsigned long);`
    pub fn set_composing_region (f: *mut frame, start: ptrdiff_t, end: ptrdiff_t,
                                 counter: c_ulong);
    #[cfg (feature = "have_android")]
    /// C 140-141: `extern void textconv_set_point_and_mark (struct frame *, ptrdiff_t,
    /// ptrdiff_t, unsigned long);`
    pub fn textconv_set_point_and_mark (f: *mut frame, point: ptrdiff_t, mark: ptrdiff_t,
                                        counter: c_ulong);
    #[cfg (feature = "have_android")]
    /// C 142-143: `extern void delete_surrounding_text (struct frame *, ptrdiff_t,
    /// ptrdiff_t, unsigned long);`
    pub fn delete_surrounding_text (f: *mut frame, before: ptrdiff_t, after: ptrdiff_t,
                                    counter: c_ulong);
    #[cfg (feature = "have_android")]
    /// C 144: `extern void request_point_update (struct frame *, unsigned long);`
    pub fn request_point_update (f: *mut frame, counter: c_ulong);
    #[cfg (feature = "have_android")]
    /// C 145: `extern void textconv_barrier (struct frame *, unsigned long);`
    pub fn textconv_barrier (f: *mut frame, counter: c_ulong);
    #[cfg (feature = "have_android")]
    /// C 146-147: `extern void replace_text (struct frame *, ptrdiff_t, ptrdiff_t,
    /// Lisp_Object, ptrdiff_t, unsigned long);`
    pub fn replace_text (f: *mut frame, start: ptrdiff_t, end: ptrdiff_t, text: Lisp_Object,
                         len: ptrdiff_t, counter: c_ulong);
    #[cfg (feature = "have_android")]
    /// C 149-151: `extern char *get_extracted_text (struct frame *, ptrdiff_t, ptrdiff_t *,
    /// ptrdiff_t *, ptrdiff_t *, ptrdiff_t *, ptrdiff_t *, bool *);`
    pub fn get_extracted_text (f: *mut frame, pos: ptrdiff_t, start_return: *mut ptrdiff_t,
                               end_return: *mut ptrdiff_t, start_offset_return: *mut ptrdiff_t,
                               end_offset_return: *mut ptrdiff_t,
                               length_return: *mut ptrdiff_t, active: *mut bool)
                               -> *mut c_char;
    #[cfg (feature = "have_android")]
    /// C 152-155: `extern char *get_surrounding_text (struct frame *, ptrdiff_t, ptrdiff_t,
    /// ptrdiff_t *, ptrdiff_t *, ptrdiff_t *, ptrdiff_t *, ptrdiff_t *);`
    pub fn get_surrounding_text (f: *mut frame, before: ptrdiff_t, after: ptrdiff_t,
                                 start_return: *mut ptrdiff_t, end_return: *mut ptrdiff_t,
                                 length_return: *mut ptrdiff_t,
                                 start_offset_return: *mut ptrdiff_t,
                                 end_offset_return: *mut ptrdiff_t) -> *mut c_char;
    #[cfg (feature = "have_android")]
    /// C 156: `extern void get_conversion_field (struct frame *, ptrdiff_t *, ptrdiff_t *);`
    pub fn get_conversion_field (f: *mut frame, field_return: *mut ptrdiff_t,
                                 start_return: *mut ptrdiff_t);

    /// C 158: `extern bool conversion_disabled_p (void);`
    pub fn conversion_disabled_p () -> bool;
    /// C 159: `extern void check_postponed_buffers (void);`
    pub fn check_postponed_buffers ();
  }
}

/// C 161: `extern void register_textconv_interface (struct textconv_interface *);` - declared
/// beside the mod's block, because a GUI backend's unit (not `src/textconv.c`) is one of its
/// callers and the port keeps one declaration per name (PORTING.md P-26).
/// C: extern void register_textconv_interface (struct textconv_interface *);
unsafe extern "C" {
  pub fn register_textconv_interface (interface: *mut textconv_interface);
}
