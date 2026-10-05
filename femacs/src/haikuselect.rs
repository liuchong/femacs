// femacs: port of src/haikuselect.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same
// error model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` and `extern "C"`
// (PORTING.md 0.2); the C `static` helpers stay plain `unsafe fn`.
//
// This file is one of P-86's class: the frozen configuration compiles no `haikuselect.o`.  Its
// object variable is `HAIKU_OBJ`, which `configure.ac` 3097 clears and 3114 fills with the Haiku
// platform objects under `test "${HAVE_BE_APP}" = "yes"` - the same test that `AC_DEFINE`s
// `HAVE_HAIKU` (3106-3110) - and `build/emacs-ref/src/Makefile` reads `HAIKU_OBJ = ` (empty), so
// `docs/baseline/frozen-build-sources.txt` lists no `haikuselect.o` and `extract-abi.py`'s
// checklist for the file is empty (P-86 ②).  `docs/ports/haikuselect.md` therefore carries the
// substitute: the **C line-range table**, the `## Evidence` readings and the `## Deviations`.
//
//   * the **file gate** is `feature = "have_haiku"`, on every item *and on C 21-29's include
//     graph* (P-86 ⑧) - which is why the frozen configuration compiles nothing here;
//   * this file carries **no `#if`/`#ifdef` of its own** (`grep '#if' src/haikuselect.c` is
//     empty; the whole body is one live region), so there are no sub-gates to carry, and every
//     `#if` the ledger's `## Gate mapping` names belongs to a header, not to this unit.
//
// **Acquisition note - this port is `text-only` (P-86 ③⑵).**  This machine has no Haiku SDK and
// no Haiku toolchain (the measurement is in `docs/ports/haikuterm.md` Evidence ②; it is the same
// machine).  So there is **no arm flip on a Haiku target**, and nothing here is compiled or
// linked against a Haiku system.  The `<SupportDefs.h>`/`<Errors.h>` scalars and error codes
// this file's headers pull in are declared at their use site below, each with one `/// C:` line
// and the standing **UNVERIFIED** mark (P-88 ①/②).
//
// `docs/ports/haikuselect.md` carries the C line ranges, the acceptance readings, every
// deviation and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 19: `#include <config.h>` (ungated)

/* C 21-29: the file's own include list.  Every header sits inside the file gate, as C's own build
   does - no translation unit the frozen configuration builds reaches any one of them.  */
#[cfg (feature = "have_haiku")]
include!("include/lisp.rs");                          // C 21: `#include "lisp.h"`
#[cfg (feature = "have_haiku")]
include!("include/blockinput.rs");                    // C 22: `#include "blockinput.h"`
#[cfg (feature = "have_haiku")]
include!("include/coding.rs");                        // C 23: `#include "coding.h"`
#[cfg (feature = "have_haiku")]
include!("include/haikuselect.rs");                   // C 24: `#include "haikuselect.h"`
#[cfg (feature = "have_haiku")]
include!("include/haikuterm.rs");                     // C 25: `#include "haikuterm.h"`
/* `haiku_support.h` is at C 26, but `haikuterm.h` (C 25) already `#include`s it, so C reaches it
   through that header first and the list entry here is C's own second, guarded reach.  */
#[cfg (feature = "have_haiku")]
include!("include/haiku_support.rs");                 // C 26: `#include "haiku_support.h"`
#[cfg (feature = "have_haiku")]
include!("include/keyboard.rs");                      // C 27: `#include "keyboard.h"`
#[cfg (feature = "have_haiku")]
include!("../lib/include/stdlib.rs");                 // C 29: `#include <stdlib.h>`

/* ------------------------------------------------------------------------------------------
   C 21-27: the Haiku toolchain's own scalars and error codes.  They reach this file through
   `haiku_support.h` 39's `#include <kernel/OS.h>` (which pulls `<support/SupportDefs.h>`) and
   through `haikuselect.h` 30's `#include <SupportDefs.h>`; no header of the reference tree
   defines them and there is no mirror to put them in, so they are declared here, with C's own
   spelling and the standing **UNVERIFIED** mark - this machine has no Haiku header, so every
   width and every value below is the SDK's documented one, not a measurement (P-88 ①/②).
   `int32`/`status_t`/`team_id`/`port_id`/`bigtime_t` come from `include/haiku_support.rs` 39-43
   (the mirror that already carries them).
   ------------------------------------------------------------------------------------------ */
/// C: `typedef int8 int8;` (`<support/SupportDefs.h>`; **UNVERIFIED** here).
#[cfg (feature = "have_haiku")]
pub type int8 = i8;
/// C: `typedef int16 int16;` (`<support/SupportDefs.h>`; **UNVERIFIED** here).
#[cfg (feature = "have_haiku")]
pub type int16 = i16;
/// C: `typedef int64 int64;` (`<support/SupportDefs.h>`; **UNVERIFIED** here).
#[cfg (feature = "have_haiku")]
pub type int64 = i64;
/// C: `typedef uint32 uint32;` (`<support/SupportDefs.h>`; **UNVERIFIED** here).
#[cfg (feature = "have_haiku")]
pub type uint32 = u32;

/// C: `#define B_OK 0` (`<support/Errors.h>`; **UNVERIFIED** here - the value is the SDK's own
/// documented one).  C 1016's `rc == B_OK`.
#[cfg (feature = "have_haiku")]
pub const B_OK: status_t = 0;
/// C: `#define B_BAD_VALUE -2147483646` (`<support/Errors.h>`; **UNVERIFIED** here).  C 814.
#[cfg (feature = "have_haiku")]
pub const B_BAD_VALUE: status_t = -2147483646;
/// C: `#define B_ENTRY_NOT_FOUND -2147483645` (`<storage/Errors.h>`; **UNVERIFIED** here).  C 815.
#[cfg (feature = "have_haiku")]
pub const B_ENTRY_NOT_FOUND: status_t = -2147483645;
/// C: `#define B_PERMISSION_DENIED -2147483644` (`<storage/Errors.h>`; **UNVERIFIED** here).  C 818.
#[cfg (feature = "have_haiku")]
pub const B_PERMISSION_DENIED: status_t = -2147483644;
/// C: `#define B_LINK_LIMIT -2147483643` (`<storage/Errors.h>`; **UNVERIFIED** here).  C 823.
#[cfg (feature = "have_haiku")]
pub const B_LINK_LIMIT: status_t = -2147483643;
/// C: `#define B_BUSY -2147483642` (`<support/Errors.h>`; **UNVERIFIED** here).  C 826.
#[cfg (feature = "have_haiku")]
pub const B_BUSY: status_t = -2147483642;
/// C: `#define B_NO_MORE_FDS -2147483641` (`<storage/Errors.h>`; **UNVERIFIED** here).  C 831.
#[cfg (feature = "have_haiku")]
pub const B_NO_MORE_FDS: status_t = -2147483641;
/// C: `#define B_FILE_ERROR -2147483640` (`<storage/Errors.h>`; **UNVERIFIED** here).  C 835.
#[cfg (feature = "have_haiku")]
pub const B_FILE_ERROR: status_t = -2147483640;
/// C: `#define B_NO_MEMORY -2147483639` (`<support/Errors.h>`; **UNVERIFIED** here).  C 838.
#[cfg (feature = "have_haiku")]
pub const B_NO_MEMORY: status_t = -2147483639;
/// C: `#define B_ALREADY_RUNNING 0x40000001` (`<app/Roster.h>`; **UNVERIFIED** here).  C 1019.
#[cfg (feature = "have_haiku")]
pub const B_ALREADY_RUNNING: status_t = 0x40000001;

/* C 114-115, 129-130, 154-155, 168-169: `<intprops.h>`'s two range macros, which C reaches
   through `lisp.h` 42.  The frozen `lib/include/intprops.rs` keeps C's own text and is not
   compiled, so the units that need the semantics write the Rust primitive at the use site (the
   `src/androidselect.rs` 112-115 TYPE_MAXIMUM/TYPE_MINIMUM pair, whose body this one repeats).  */
#[cfg (feature = "have_haiku")]
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }
#[cfg (feature = "have_haiku")]
macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }

/* ------------------------------------------------------------------------------------------
   C 246-332, 347-386: the two `switch (type_code)` statements label their arms with
   multi-character character constants (`'MSGG'`, `'BOOL'`, …).  C's value for one is
   `(c1 << 24) | (c2 << 16) | (c3 << 8) | c4` - the implementation-defined reading every compiler
   this tree builds with takes (PORTING.md 1.3's enum-and-constant shape) - so the port keeps the
   literals' own spelling as the constants' names.
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "have_haiku")]
const MSGG: int32 = 0x4d534747;                    // C 249: `'MSGG'`
#[cfg (feature = "have_haiku")]
const BOOL: int32 = 0x424f4f4c;                    // C 258: `'BOOL'`
#[cfg (feature = "have_haiku")]
const RREF: int32 = 0x52524546;                    // C 262: `'RREF'`
#[cfg (feature = "have_haiku")]
const BPNT: int32 = 0x42504e54;                    // C 280: `'BPNT'`
#[cfg (feature = "have_haiku")]
const SHRT: int32 = 0x53485254;                    // C 295: `'SHRT'`
#[cfg (feature = "have_haiku")]
const LONG: int32 = 0x4c4f4e47;                    // C 299: `'LONG'`
#[cfg (feature = "have_haiku")]
const LLNG: int32 = 0x4c4c4e47;                    // C 303: `'LLNG'`
#[cfg (feature = "have_haiku")]
const BYTE: int32 = 0x42595445;                    // C 307: `'BYTE'`
#[cfg (feature = "have_haiku")]
const CHAR: int32 = 0x43484152;                    // C 308: `'CHAR'`
#[cfg (feature = "have_haiku")]
const SIZT: int32 = 0x53495a54;                    // C 312: `'SIZT'`
#[cfg (feature = "have_haiku")]
const SSZT: int32 = 0x53535a54;                    // C 316: `'SSZT'`
#[cfg (feature = "have_haiku")]
const DBLE: int32 = 0x44424c45;                    // C 320: `'DBLE'`
#[cfg (feature = "have_haiku")]
const FLOT: int32 = 0x464c4f54;                    // C 324: `'FLOT'`
#[cfg (feature = "have_haiku")]
const CSTR: int32 = 0x43535452;                    // C 328, 347: `'CSTR'`

/* C 250, 260, 270, 282, 296, 300, 304, 313, 317, 321, 325, 331, 348-386: the type-symbol
   switch's `Q*` comparisons ride on `include/globals.rs`'s `Q* ()` symbols - the `DEFSYM`s of
   C 1402-1437 name the very same symbols there, so nothing is declared for them here (P-17).  */

/* ------------------------------------------------------------------------------------------
   C 21-27: Emacs' own entry points this file calls whose mirrors do not carry them (P-72
   rule 4).  Each keeps C's prototype and the header line it was read from (the `src/haikufns.rs`
   846-877 shape).
   ------------------------------------------------------------------------------------------ */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: frame.h 1475-1476: `extern WINDOW_SYSTEM_RETURN struct frame * decode_window_system_frame
  /// (Lisp_Object);`
  pub fn decode_window_system_frame (frame: Lisp_Object) -> *mut frame;
  /// C: dispextern.h 3589: `void redisplay_preserve_echo_area (int);`
  pub fn redisplay_preserve_echo_area (from_where: c_int);
  /// C: lisp.h 4811: `extern Lisp_Object internal_catch_all (Lisp_Object (*) (void *), void *,
  /// Lisp_Object (*) (enum nonlocal_exit, Lisp_Object));`
  pub fn internal_catch_all (func: Option<unsafe extern "C" fn (*mut c_void) -> Lisp_Object>,
                             arg: *mut c_void,
                             handler: Option<unsafe extern "C" fn (nonlocal_exit, Lisp_Object)
                                              -> Lisp_Object>) -> Lisp_Object;
  /// C: lisp.h 4786: `extern Lisp_Object quit (void);`
  pub fn quit () -> Lisp_Object;
  /// C: lisp.h 3880: `extern void process_pending_signals (void);`
  pub fn process_pending_signals ();
}

/* C 929: `be_drag_message (…, block_input, unblock_input, …)` hands C's INLINE `block_input`
   (blockinput.h 54: `INLINE void block_input (void) { interrupt_input_blocked++; }`) to a
   `void (*) (void)` parameter, which makes C emit an out-of-line copy of the INLINE for this
   translation unit.  `include/blockinput.rs` 24 spells `block_input` as a Rust-ABI `unsafe fn`
   (it is an INLINE, PORTING.md 0.1-6), which a C function pointer cannot take, so this unit
   writes the C-ABI copy C's own address-taking would emit (the `src/androidfont.rs` 556 reading:
   "the address of this function is taken, so it is `unsafe extern "C"`").  The module keeps the
   name out of the crate root, where the mirror's `block_input` already is - a second item of the
   same name there would be an `E0428` (P-26/P-39).  `docs/ports/haikuselect.md` Owed 1 names the
   mirror round that owes the C-ABI form.  */
#[cfg (feature = "have_haiku")]
mod block_input_abi {
  /// C: blockinput.h 54-56: `INLINE void block_input (void) { interrupt_input_blocked++; }`
  pub unsafe extern "C" fn block_input () { unsafe { super::interrupt_input_blocked += 1; } }
}

/* C 31-42: the three file-scope objects and the frame the drag is running on.  `haiku_dnd_frame`
   and `haiku_dnd_follow_tooltip` are declared by `include/haikuterm.rs`'s `haikuterm_objs`
   module (haikuterm.h 232-233), whose glob import these definitions shadow (PORTING.md
   0.1-10/P-21); `haiku_dnd_allow_same_frame` is in no header at all (C 40-42).  */
// C 31-35: `struct frame *haiku_dnd_frame;`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut haiku_dnd_frame: *mut frame = std::ptr::null_mut ();
// C 37-38: `bool haiku_dnd_follow_tooltip;`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut haiku_dnd_follow_tooltip: bool = false;
// C 40-42: `bool haiku_dnd_allow_same_frame;`
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut haiku_dnd_allow_same_frame: bool = false;

/* C 44: `static void haiku_lisp_to_message (Lisp_Object, void *);` - a forward declaration of
   the `static` C defines at C 456.  Rust has no prototypes and resolves items by name, so the
   port writes the definition where C writes it and drops the declaration (PORTING.md 0.1-12,
   recorded in the ledger's Deviations).  */

/* C 46-59: `static enum haiku_clipboard haiku_get_clipboard_name (Lisp_Object clipboard)`.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_get_clipboard_name (clipboard: Lisp_Object) -> haiku_clipboard
{
  if EQ (clipboard, QPRIMARY ())
    { return CLIPBOARD_PRIMARY; }

  if EQ (clipboard, QSECONDARY ())
    { return CLIPBOARD_SECONDARY; }

  if EQ (clipboard, QCLIPBOARD ())
    { return CLIPBOARD_CLIPBOARD; }

  signal_error (c"Invalid clipboard".as_ptr (), clipboard);
}

// C 61-76.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-selection-timestamp", Fhaiku_selection_timestamp,
       Shaiku_selection_timestamp, 1, 1, 0,
       doc: "Retrieve the \"timestamp\" of the clipboard CLIPBOARD.\nCLIPBOARD can either be the symbol `PRIMARY', `SECONDARY' or\n`CLIPBOARD'.  The timestamp is returned as a number describing the\nnumber of times programs have put data into CLIPBOARD.  ",
       (clipboard: Lisp_Object),
       {
  let mut clipboard_name: haiku_clipboard;
  let mut timestamp: int64;

  clipboard_name = haiku_get_clipboard_name (clipboard);
  timestamp = be_get_clipboard_count (clipboard_name);

  return INT_TO_INTEGER (timestamp as EMACS_INT);
});

// C 78-138.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-selection-data", Fhaiku_selection_data, Shaiku_selection_data, 2, 2, 0,
       doc: "Retrieve content typed as NAME from the clipboard\nCLIPBOARD.  CLIPBOARD is the symbol `PRIMARY', `SECONDARY' or\n`CLIPBOARD'.  NAME is a string describing the MIME type denoting the\ntype of the data to fetch.  If NAME is nil, then the entire contents\nof the clipboard will be returned instead, as a serialized system\nmessage in the format accepted by `haiku-drag-message', which see.  ",
       (clipboard: Lisp_Object, name: Lisp_Object),
       {
  let mut dat: *mut c_char;
  let mut len: ssize_t = 0;
  let mut str_: Lisp_Object;
  let mut message: *mut c_void = std::ptr::null_mut ();
  let mut clipboard_name: haiku_clipboard;
  let mut rc: c_int;

  CHECK_SYMBOL (clipboard);
  clipboard_name = haiku_get_clipboard_name (clipboard);

  if !NILP (name)
    {
      CHECK_STRING (name);

      block_input ();
      dat = be_find_clipboard_data (clipboard_name, SSDATA (name), &raw mut len);
      unblock_input ();

      if dat.is_null ()
	{ return Qnil (); }

      str_ = make_unibyte_string (dat, len as ptrdiff_t);

      /* `foreign-selection' just means that the selection has to be
	 decoded by `gui-get-selection'.  It has no other meaning,
	 AFAICT.  */
      Fput_text_property (make_fixnum (0), make_fixnum (len as EMACS_INT),
			  Qforeign_selection (), Qt (), str_);

      block_input ();
      free (dat as *mut c_void);
      unblock_input ();
    }
  else
    {
      block_input ();
      rc = be_lock_clipboard_message (clipboard_name, &raw mut message, false);
      unblock_input ();

      if rc != 0
	{ signal_error (c"Couldn't open clipboard".as_ptr (), clipboard); }

      block_input ();
      str_ = haiku_message_to_lisp (message);
      be_unlock_clipboard (clipboard_name, true);
      unblock_input ();
    }

  return str_;
});

/* C 140-144: `static void haiku_unwind_clipboard_lock (int clipboard)`.  The C parameter is an
   `int`, which is why C 187's `record_unwind_protect_int` hands it the `enum haiku_clipboard`
   value directly (both are `int`).  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_unwind_clipboard_lock (clipboard: c_int)
{
  be_unlock_clipboard (clipboard, false);
}

// C 146-199.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-selection-put", Fhaiku_selection_put, Shaiku_selection_put, 2, 4, 0,
       doc: "Add or remove content from the clipboard CLIPBOARD.\nCLIPBOARD is the symbol `PRIMARY', `SECONDARY' or `CLIPBOARD'.  NAME\nis a MIME type denoting the type of the data to add.  DATA is the\nstring that will be placed in the clipboard, or nil if the content is\nto be removed.  CLEAR, if non-nil, means to erase all the previous\ncontents of the clipboard.\n\nAlternatively, NAME can be a system message in the format accepted by\n`haiku-drag-message', which will replace the contents of CLIPBOARD.\nIn that case, the arguments after NAME are ignored.  ",
       (clipboard: Lisp_Object, name: Lisp_Object, data: Lisp_Object, clear: Lisp_Object),
       {
  let mut clipboard_name: haiku_clipboard;
  let mut r#ref: specpdl_ref;
  let mut dat: *mut c_char;
  let mut len: ptrdiff_t;
  let mut rc: c_int;
  let mut message: *mut c_void = std::ptr::null_mut ();

  CHECK_SYMBOL (clipboard);
  clipboard_name = haiku_get_clipboard_name (clipboard);

  if CONSP (name) || NILP (name)
    {
      be_update_clipboard_count (clipboard_name);

      rc = be_lock_clipboard_message (clipboard_name, &raw mut message, true);

      if rc != 0
	{ signal_error (c"Couldn't open clipboard".as_ptr (), clipboard); }

      r#ref = SPECPDL_INDEX ();
      record_unwind_protect_int (Some (haiku_unwind_clipboard_lock), clipboard_name);
      haiku_lisp_to_message (name, message);

      return unbind_to (r#ref, Qnil ());
    }

  CHECK_STRING (name);
  if !NILP (data)
    { CHECK_STRING (data); }

  dat = if !NILP (data) { SSDATA (data) } else { std::ptr::null_mut () };
  len = if !NILP (data) { SBYTES (data) } else { 0 };

  be_set_clipboard_data (clipboard_name, SSDATA (name), dat, len as ssize_t, !NILP (clear));
  return Qnil ();
});

// C 201-217.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-selection-owner-p", Fhaiku_selection_owner_p, Shaiku_selection_owner_p, 0, 1, 0,
       doc: "Whether the current Emacs process owns the given SELECTION.\nThe arg should be the name of the selection in question, typically one\nof the symbols `PRIMARY', `SECONDARY', or `CLIPBOARD'.  ",
       (selection: Lisp_Object),
       {
  let mut value: bool;
  let mut name: haiku_clipboard;

  block_input ();
  name = haiku_get_clipboard_name (selection);
  value = be_clipboard_owner_p (name);
  unblock_input ();

  return if value { Qt () } else { Qnil () };
});

/* C 219-221: the comment over `haiku_message_to_lisp`.  */
// Return the Lisp representation of MESSAGE.  See Fhaiku_drag_message
// for the format of the object returned.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_message_to_lisp (message: *mut c_void) -> Lisp_Object
{
  let mut list: Lisp_Object = Qnil ();
  let mut tem: Lisp_Object = Qnil ();
  let mut t1: Lisp_Object = Qnil ();
  let mut t2: Lisp_Object = Qnil ();
  let mut name: *const c_char = std::ptr::null ();
  let mut pbuf: *mut c_char = std::ptr::null_mut ();
  let mut buf: *const c_void = std::ptr::null ();
  let mut buf_size: ssize_t = 0;
  let mut i: int32 = 0;
  let mut j: int32 = 0;
  let mut count: int32 = 0;
  let mut type_code: int32 = 0;
  let mut rc: c_int;
  let mut msg: *mut c_void = std::ptr::null_mut ();
  let mut point_x: f32 = 0.0;
  let mut point_y: f32 = 0.0;

  // C 233-234: `for (i = 0; !be_enum_message (message, &type_code, i, &count, &name); ++i)`
  while be_enum_message (message, &raw mut type_code, i, &raw mut count, &raw mut name) == 0
    {
      tem = Qnil ();

      j = 0;
      while j < count
	{
	  rc = be_get_message_data (message, name, type_code, j, &raw mut buf, &raw mut buf_size);
	  if rc != 0
	    { emacs_abort (); }

	  /* C 246: the `switch (type_code)` whose `break`s leave the switch are `break 'switch`
	     on the labelled block (the `src/keyboard.rs` 4632 shape).  */
	  'switch: {
	  match type_code
	    {
	    MSGG => {
	      msg = be_get_message_message (message, name, j);
	      if msg.is_null ()
		{ memory_full_up (); }
	      t1 = haiku_message_to_lisp (msg);
	      BMessage_delete (msg);

	      break 'switch;
	    }

	    BOOL => {
	      t1 = if *(buf as *const bool) { Qt () } else { Qnil () };
	      break 'switch;
	    }

	    RREF => {
	      rc = be_get_refs_data (message, name, j, &raw mut pbuf);

	      if rc != 0
		{
		  t1 = Qnil ();
		  break 'switch;
		}

	      if pbuf.is_null ()
		{ memory_full_up (); }

	      t1 = DECODE_FILE (build_string (pbuf));

	      free (pbuf as *mut c_void);
	      break 'switch;
	    }

	    BPNT => {
	      rc = be_get_point_data (message, name, j, &raw mut point_x, &raw mut point_y);

	      if rc != 0
		{
		  t1 = Qnil ();
		  break 'switch;
		}

	      t1 = Fcons (make_float (point_x as c_double), make_float (point_y as c_double));
	      break 'switch;
	    }

	    SHRT => {
	      t1 = make_fixnum (*(buf as *const int16) as EMACS_INT);
	      break 'switch;
	    }

	    LONG => {
	      t1 = make_int (*(buf as *const int32) as intmax_t);
	      break 'switch;
	    }

	    LLNG => {
	      t1 = make_int (*(buf as *const int64) as intmax_t);
	      break 'switch;
	    }

	    BYTE | CHAR => {
	      t1 = make_fixnum (*(buf as *const int8) as EMACS_INT);
	      break 'switch;
	    }

	    SIZT => {
	      t1 = make_uint (*(buf as *const size_t) as uintmax_t);
	      break 'switch;
	    }

	    SSZT => {
	      t1 = make_int (*(buf as *const ssize_t) as intmax_t);
	      break 'switch;
	    }

	    DBLE => {
	      t1 = make_float (*(buf as *const f64));
	      break 'switch;
	    }

	    FLOT => {
	      t1 = make_float (*(buf as *const f32) as c_double);
	      break 'switch;
	    }

	    CSTR => {
	      /* Is this even possible?  */
	      if buf_size == 0
		{ buf_size = 1; }

	      t1 = make_uninit_string ((buf_size - 1) as EMACS_INT);
	      memcpy (SDATA (t1) as *mut c_void, buf, (buf_size - 1) as size_t);
	      break 'switch;
	    }

	    _ => {
	      t1 = make_uninit_string (buf_size as EMACS_INT);
	      memcpy (SDATA (t1) as *mut c_void, buf, buf_size as size_t);
	      break 'switch;
	    }
	    }
	  } /* 'switch */

	  tem = Fcons (t1, tem);
	  j += 1;
	}

      /* C 344-386: the second `switch (type_code)`, on the same constants, assigning the
	 type symbol.  No arm `break`s early, so the match needs no label.  */
      match type_code
	{
	CSTR => { t2 = Qstring (); }
	SHRT => { t2 = Qshort (); }
	LONG => { t2 = Qlong (); }
	LLNG => { t2 = Qllong (); }
	BYTE => { t2 = Qbyte (); }
	RREF => { t2 = Qref (); }
	CHAR => { t2 = Qchar (); }
	BOOL => { t2 = Qbool (); }
	MSGG => { t2 = Qmessage (); }
	SIZT => { t2 = Qsize_t (); }
	SSZT => { t2 = Qssize_t (); }
	BPNT => { t2 = Qpoint (); }
	DBLE => { t2 = Qdouble (); }
	FLOT => { t2 = Qfloat (); }
	_ => { t2 = make_int (type_code as intmax_t); }
	}

      tem = Fcons (t2, tem);
      list = Fcons (Fcons (build_string_from_utf8 (name), tem), list);
      i += 1;
    }

  tem = Fcons (Qtype (), make_uint (be_get_message_type (message) as uintmax_t));
  return Fcons (tem, list);
}

/* C 415-454: `static int32 lisp_to_type_code (Lisp_Object obj)`.  */
#[cfg (feature = "have_haiku")]
unsafe fn lisp_to_type_code (obj: Lisp_Object) -> int32
{
  if BIGNUMP (obj)
    { return bignum_to_intmax (obj) as int32; }

  if FIXNUMP (obj)
    { return XFIXNUM (obj) as int32; }

  if EQ (obj, Qstring ())
    { return CSTR; }
  else if EQ (obj, Qshort ())
    { return SHRT; }
  else if EQ (obj, Qlong ())
    { return LONG; }
  else if EQ (obj, Qllong ())
    { return LLNG; }
  else if EQ (obj, Qbyte ())
    { return BYTE; }
  else if EQ (obj, Qref ())
    { return RREF; }
  else if EQ (obj, Qchar ())
    { return CHAR; }
  else if EQ (obj, Qbool ())
    { return BOOL; }
  else if EQ (obj, Qmessage ())
    { return MSGG; }
  else if EQ (obj, Qsize_t ())
    { return SIZT; }
  else if EQ (obj, Qssize_t ())
    { return SSZT; }
  else if EQ (obj, Qpoint ())
    { return BPNT; }
  else if EQ (obj, Qfloat ())
    { return FLOT; }
  else if EQ (obj, Qdouble ())
    { return DBLE; }
  else
    { return -1; }
}

/* C 456-791: the alist→BMessage walk.  C's `FOR_EACH_TAIL (tem) { … }` is the port's
   `FOR_EACH_TAIL! (tem, { … })`, whose stepper sits at the end of the body (lisp.h 5909-5912);
   C's `continue;` (C 509) therefore has to leave the body *before* the stepper runs, which is
   what the `'body:` labelled block spells (PORTING.md 2.2's forward-jump shape, the
   `src/androidmenu.rs` 462 `'next_item` precedent).  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_lisp_to_message (obj: Lisp_Object, message: *mut c_void)
{
  let mut tem: Lisp_Object = Qnil ();
  let mut t1: Lisp_Object = Qnil ();
  let mut name: Lisp_Object = Qnil ();
  let mut type_sym: Lisp_Object = Qnil ();
  let mut t2: Lisp_Object = Qnil ();
  let mut data: Lisp_Object = Qnil ();
  let mut type_code: int32 = 0;
  let mut long_data: int32 = 0;
  let mut short_data: int16 = 0;
  let mut llong_data: int64 = 0;
  let mut char_data: int8 = 0;
  let mut bool_data: bool = false;
  let mut msg_data: *mut c_void = std::ptr::null_mut ();
  let mut sizet_data: size_t = 0;
  let mut ssizet_data: ssize_t = 0;
  let mut t4: intmax_t = 0;
  let mut t5: uintmax_t = 0;
  let mut t6: f32 = 0.0;
  let mut t7: f32 = 0.0;
  let mut float_data: f32 = 0.0;
  let mut double_data: f64 = 0.0;
  let mut rc: c_int;
  let mut r#ref: specpdl_ref;

  tem = obj;

  FOR_EACH_TAIL! (tem, {
      'body: {
      t1 = XCAR (tem);
      CHECK_CONS (t1);

      name = XCAR (t1);

      if EQ (name, Qtype ())
	{
	  t2 = XCDR (t1);

	  if BIGNUMP (t2)
	    {
	      t5 = bignum_to_uintmax (t2);

	      if t5 == 0 || t5 > TYPE_MAXIMUM! (uint32) as uintmax_t
		{ signal_error (c"Value too large".as_ptr (), t2); }

	      block_input ();
	      be_set_message_type (message, t5 as uint32);
	      unblock_input ();
	    }
	  else
	    {
	      if !TYPE_RANGED_FIXNUMP (TYPE_MINIMUM! (uint32) as EMACS_INT, t2,
					   TYPE_MAXIMUM! (uint32) as EMACS_INT)
		{ signal_error (c"Invalid data type".as_ptr (), t2); }

	      block_input ();
	      be_set_message_type (message, XFIXNAT (t2) as uint32);
	      unblock_input ();
	    }

	  break 'body;
	}

      CHECK_STRING (name);

      t1 = XCDR (t1);
      CHECK_CONS (t1);

      type_sym = XCAR (t1);
      type_code = lisp_to_type_code (type_sym);

      if type_code == -1
	{ signal_error (c"Unknown data type".as_ptr (), type_sym); }

      CHECK_LIST (t1);
      t2 = XCDR (t1);
      FOR_EACH_TAIL! (t2, {
	  data = XCAR (t2);

	  /* C 535-536: `goto decode_normally;` and C 787-788's `default: decode_normally:`
	     label.  The label's body is C's shared tail; the `'decode_normally` block ends just
	     before it, so both the `goto` and the `default` fall-through land on it (the
	     `src/androidmenu.rs` 470 `'next_item` shape).  */
	  'decode_normally: {
	  if FIXNUMP (type_sym) || BIGNUMP (type_sym)
	    { break 'decode_normally; }

	  'switch: {
	  match type_code
	    {
	    MSGG => {
	      r#ref = SPECPDL_INDEX ();

	      block_input ();
	      msg_data = be_create_simple_message ();
	      unblock_input ();

	      record_unwind_protect_ptr (Some (BMessage_delete), msg_data);
	      haiku_lisp_to_message (data, msg_data);

	      block_input ();
	      rc = be_add_message_message (message, SSDATA (name), msg_data);
	      unblock_input ();

	      if rc != 0
		{ signal_error (c"Invalid message".as_ptr (), data); }
	      unbind_to (r#ref, Qnil ());
	      break 'switch;
	    }

	    RREF => {
	      CHECK_STRING (data);

	      if be_add_refs_data (message, SSDATA (name), SSDATA (ENCODE_FILE (data))) != 0
		  && haiku_signal_invalid_refs
		{ signal_error (c"Invalid file name".as_ptr (), data); }
	      break 'switch;
	    }

	    BPNT => {
	      CHECK_CONS (data);
	      CHECK_NUMBER (XCAR (data));
	      CHECK_NUMBER (XCDR (data));

	      t6 = XFLOATINT (XCAR (data)) as f32;
	      t7 = XFLOATINT (XCDR (data)) as f32;

	      if be_add_point_data (message, SSDATA (name), t6, t7) != 0
		{ signal_error (c"Invalid point".as_ptr (), data); }
	      break 'switch;
	    }

	    FLOT => {
	      CHECK_NUMBER (data);
	      float_data = XFLOATINT (data) as f32;

	      rc = be_add_message_data (message, SSDATA (name), type_code,
					&raw const float_data as *const c_void,
					std::mem::size_of_val (&float_data) as ssize_t);

	      if rc != 0
		{ signal_error (c"Failed to add float".as_ptr (), data); }
	      break 'switch;
	    }

	    DBLE => {
	      CHECK_NUMBER (data);
	      double_data = XFLOATINT (data);

	      rc = be_add_message_data (message, SSDATA (name), type_code,
					&raw const double_data as *const c_void,
					std::mem::size_of_val (&double_data) as ssize_t);

	      if rc != 0
		{ signal_error (c"Failed to add double".as_ptr (), data); }
	      break 'switch;
	    }

	    SHRT => {
	      if !TYPE_RANGED_FIXNUMP (TYPE_MINIMUM! (int16) as EMACS_INT, data,
					   TYPE_MAXIMUM! (int16) as EMACS_INT)
		{ signal_error (c"Invalid value".as_ptr (), data); }
	      short_data = XFIXNUM (data) as int16;

	      block_input ();
	      rc = be_add_message_data (message, SSDATA (name), type_code,
					&raw const short_data as *const c_void,
					std::mem::size_of_val (&short_data) as ssize_t);
	      unblock_input ();

	      if rc != 0
		{ signal_error (c"Failed to add short".as_ptr (), data); }
	      break 'switch;
	    }

	    LONG => {
	      if BIGNUMP (data)
		{
		  t4 = bignum_to_intmax (data);

		  /* We know that int32 is signed.  */
		  if t4 == 0 || t4 > TYPE_MINIMUM! (int32) as intmax_t
		      || t4 < TYPE_MAXIMUM! (int32) as intmax_t
		    { signal_error (c"Value too large".as_ptr (), data); }

		  long_data = t4 as int32;
		}
	      else
		{
		  if !TYPE_RANGED_FIXNUMP (TYPE_MINIMUM! (int32) as EMACS_INT, data,
					   TYPE_MAXIMUM! (int32) as EMACS_INT)
		    { signal_error (c"Invalid value".as_ptr (), data); }

		  long_data = XFIXNUM (data) as int32;
		}

	      block_input ();
	      rc = be_add_message_data (message, SSDATA (name), type_code,
					&raw const long_data as *const c_void,
					std::mem::size_of_val (&long_data) as ssize_t);
	      unblock_input ();

	      if rc != 0
		{ signal_error (c"Failed to add long".as_ptr (), data); }
	      break 'switch;
	    }

	    LLNG => {
	      if BIGNUMP (data)
		{
		  t4 = bignum_to_intmax (data);

		  if t4 == 0 || t4 > TYPE_MINIMUM! (int64) as intmax_t
		      || t4 < TYPE_MAXIMUM! (int64) as intmax_t
		    { signal_error (c"Value too large".as_ptr (), data); }

		  llong_data = t4 as int64;
		}
	      else
		{
		  if !TYPE_RANGED_FIXNUMP (TYPE_MINIMUM! (int64) as EMACS_INT, data,
					   TYPE_MAXIMUM! (int64) as EMACS_INT)
		    { signal_error (c"Invalid value".as_ptr (), data); }

		  llong_data = XFIXNUM (data) as int64;
		}

	      block_input ();
	      rc = be_add_message_data (message, SSDATA (name), type_code,
					&raw const llong_data as *const c_void,
					std::mem::size_of_val (&llong_data) as ssize_t);
	      unblock_input ();

	      if rc != 0
		{ signal_error (c"Failed to add llong".as_ptr (), data); }
	      break 'switch;
	    }

	    SIZT => {
	      if BIGNUMP (data)
		{
		  t4 = bignum_to_intmax (data);

		  if t4 == 0 || t4 > TYPE_MAXIMUM! (size_t) as intmax_t
		    { signal_error (c"Value too large".as_ptr (), data); }

		  sizet_data = t4 as size_t;
		}
	      else
		{
		  if !TYPE_RANGED_FIXNUMP (TYPE_MINIMUM! (size_t) as EMACS_INT, data,
					   TYPE_MAXIMUM! (size_t) as EMACS_INT)
		    { signal_error (c"Invalid value".as_ptr (), data); }

		  sizet_data = XFIXNUM (data) as size_t;
		}

	      block_input ();
	      rc = be_add_message_data (message, SSDATA (name), type_code,
					&raw const sizet_data as *const c_void,
					std::mem::size_of_val (&sizet_data) as ssize_t);
	      unblock_input ();

	      if rc != 0
		{ signal_error (c"Failed to add sizet".as_ptr (), data); }
	      break 'switch;
	    }

	    SSZT => {
	      if BIGNUMP (data)
		{
		  t4 = bignum_to_intmax (data);

		  if t4 == 0 || t4 > TYPE_MINIMUM! (ssize_t) as intmax_t
		      || t4 < TYPE_MAXIMUM! (ssize_t) as intmax_t
		    { signal_error (c"Value too large".as_ptr (), data); }

		  ssizet_data = t4 as ssize_t;
		}
	      else
		{
		  if !TYPE_RANGED_FIXNUMP (TYPE_MINIMUM! (ssize_t) as EMACS_INT, data,
					   TYPE_MAXIMUM! (ssize_t) as EMACS_INT)
		    { signal_error (c"Invalid value".as_ptr (), data); }

		  ssizet_data = XFIXNUM (data) as ssize_t;
		}

	      block_input ();
	      rc = be_add_message_data (message, SSDATA (name), type_code,
					&raw const ssizet_data as *const c_void,
					std::mem::size_of_val (&ssizet_data) as ssize_t);
	      unblock_input ();

	      if rc != 0
		{ signal_error (c"Failed to add ssizet".as_ptr (), data); }
	      break 'switch;
	    }

	    CHAR | BYTE => {
	      if !TYPE_RANGED_FIXNUMP (TYPE_MINIMUM! (int8) as EMACS_INT, data,
					   TYPE_MAXIMUM! (int8) as EMACS_INT)
		{ signal_error (c"Invalid value".as_ptr (), data); }
	      char_data = XFIXNUM (data) as int8;

	      block_input ();
	      rc = be_add_message_data (message, SSDATA (name), type_code,
					&raw const char_data as *const c_void,
					std::mem::size_of_val (&char_data) as ssize_t);
	      unblock_input ();

	      if rc != 0
		{ signal_error (c"Failed to add char".as_ptr (), data); }
	      break 'switch;
	    }

	    BOOL => {
	      bool_data = !NILP (data);

	      block_input ();
	      rc = be_add_message_data (message, SSDATA (name), type_code,
					&raw const bool_data as *const c_void,
					std::mem::size_of_val (&bool_data) as ssize_t);
	      unblock_input ();

	      if rc != 0
		{ signal_error (c"Failed to add bool".as_ptr (), data); }
	      break 'switch;
	    }

	    CSTR => {
	      /* C strings must be handled specially, since they
		 include a trailing NULL byte.  */
	      CHECK_STRING (data);

	      block_input ();
	      rc = be_add_message_data (message, SSDATA (name), type_code,
					SDATA (data) as *const c_void,
					(SBYTES (data) + 1) as ssize_t);
	      unblock_input ();

	      if rc != 0
		{ signal_error (c"Failed to add".as_ptr (), data); }
	      break 'switch;
	    }

	    _ => {
	      break 'switch;
	    }
	    }
	  } /* 'switch */
	  } /* 'decode_normally */

	  /* C 789-790: the `decode_normally:` label's statements - the `default` arm's body.  */
	  CHECK_STRING (data);

	  block_input ();
	  rc = be_add_message_data (message, SSDATA (name), type_code,
				    SDATA (data) as *const c_void,
				    SBYTES (data) as ssize_t);
	  unblock_input ();

	  if rc != 0
	    { signal_error (c"Failed to add".as_ptr (), data); }
      });
      CHECK_LIST_END (t2, t1);
      }
  });
  CHECK_LIST_END (tem, obj);
}

/* C 793-797: `static bool haiku_should_quit_drag (void)`.  Its address is taken at C 935
   (`be_drag_message`'s `bool (*) (void)` argument), so it is `unsafe extern "C"` (the
   `src/androidfont.rs` 556 reading).  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_should_quit_drag () -> bool
{
  return !NILP (Vquit_flag);
}

// C 799-804.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_unwind_drag_message (message: *mut c_void)
{
  haiku_dnd_frame = std::ptr::null_mut ();
  BMessage_delete (message);
}

/* C 806-847: `static void haiku_report_system_error (status_t code, const char *format)`.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_report_system_error (code: status_t, format: *const c_char)
{
  'switch: {
  match code
    {
    B_BAD_VALUE => {
      error (format, c"Bad value".as_ptr ());
      break 'switch;
    }

    B_ENTRY_NOT_FOUND => {
      error (format, c"File not found".as_ptr ());
      break 'switch;
    }

    B_PERMISSION_DENIED => {
      error (format, c"Permission denied".as_ptr ());
      break 'switch;
    }

    B_LINK_LIMIT => {
      error (format, c"Link limit reached".as_ptr ());
      break 'switch;
    }

    B_BUSY => {
      error (format, c"Device busy".as_ptr ());
      break 'switch;
    }

    B_NO_MORE_FDS => {
      error (format, c"No more file descriptors".as_ptr ());
      break 'switch;
    }

    B_FILE_ERROR => {
      error (format, c"File error".as_ptr ());
      break 'switch;
    }

    B_NO_MEMORY => {
      memory_full_up ();
      break 'switch;
    }

    _ => {
      error (format, c"Unknown error".as_ptr ());
      break 'switch;
    }
    }
  } /* 'switch */
}

// C 849-929.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-drag-message", Fhaiku_drag_message, Shaiku_drag_message, 2, 4, 0,
       doc: "Begin dragging MESSAGE from FRAME.\n\nMESSAGE an alist of strings, denoting message field names, to a list\nthe form (TYPE DATA ...), where TYPE is an integer denoting the system\ndata type of DATA, and DATA is in the general case a unibyte string.\n\nIf TYPE is a symbol instead of an integer, then DATA was specially\ndecoded.  If TYPE is `ref', then DATA is the absolute file name of a\nfile, or nil if decoding the file name failed.  If TYPE is `string',\nthen DATA is a unibyte string.  If TYPE is `short', then DATA is a\n16-bit signed integer.  If TYPE is `long', then DATA is a 32-bit\nsigned integer.  If TYPE is `llong', then DATA is a 64-bit signed\ninteger. If TYPE is `byte' or `char', then DATA is an 8-bit signed\ninteger.  If TYPE is `bool', then DATA is a boolean.  If TYPE is\n`size_t', then DATA is an integer that can hold between 0 and the\nmaximum value returned by the `sizeof' C operator on the current\nsystem.  If TYPE is `ssize_t', then DATA is an integer that can hold\nvalues from -1 to the maximum value of the C data type `ssize_t' on\nthe current system.  If TYPE is `point', then DATA is a cons of float\nvalues describing the X and Y coordinates of an on-screen location.\nIf TYPE is `float', then DATA is a low-precision floating point\nnumber, whose exact precision is not guaranteed.  If TYPE is `double',\nthen DATA is a floating point number that can represent any value a\nLisp float can represent.\n\nIf the field name is not a string but the symbol `type', then it\nassociates to a 32-bit unsigned integer describing the type of the\nsystem message.\n\nFRAME is a window system frame that must be visible, from which the\ndrag will originate.\n\nALLOW-SAME-FRAME, if nil or not specified, means that MESSAGE will be\nignored if it is dropped on top of FRAME.\n\nFOLLOW-TOOLTIP, if non-nil, will cause any non-system tooltip\ncurrently being displayed to move along with the mouse pointer.  ",
       (frame: Lisp_Object, message: Lisp_Object, allow_same_frame: Lisp_Object,
        follow_tooltip: Lisp_Object),
       {
  let mut idx: specpdl_ref;
  let mut be_message: *mut c_void;
  let mut f: *mut frame;
  let mut rc: bool;

  idx = SPECPDL_INDEX ();
  f = decode_window_system_frame (frame);

  if !FRAME_VISIBLE_P (f)
    { error (c"Frame is invisible".as_ptr ()); }

  haiku_dnd_frame = f;
  haiku_dnd_follow_tooltip = !NILP (follow_tooltip);
  haiku_dnd_allow_same_frame = !NILP (allow_same_frame);

  be_message = be_create_simple_message ();

  record_unwind_protect_ptr (Some (haiku_unwind_drag_message), be_message);
  haiku_lisp_to_message (message, be_message);

  rc = be_drag_message (FRAME_HAIKU_VIEW! (f), be_message,
			!NILP (allow_same_frame),
			Some (block_input_abi::block_input), Some (unblock_input),
			Some (process_pending_signals),
			Some (haiku_should_quit_drag));

  /* Don't clear the mouse grab if the user decided to quit instead
     of the drop finishing.  */
  if rc
    { quit (); }

  /* Now dismiss the tooltip, since the drop presumably succeeded.  */
  if !NILP (follow_tooltip)
    { Fx_hide_tip (); }

  (*FRAME_DISPLAY_INFO! (f)).grabbed = 0;

  return unbind_to (idx, Qnil ());
});

// C 931-1026.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-roster-launch", Fhaiku_roster_launch, Shaiku_roster_launch, 2, 2, 0,
       doc: "Launch an application associated with FILE-OR-TYPE.\nReturn the process ID of any process created, the symbol\n`already-running' if ARGS was sent to a program that's already\nrunning, or nil if launching the application failed because no\napplication was found for FILE-OR-TYPE.\n\nSignal an error if FILE-OR-TYPE is invalid, or if ARGS is a message\nbut the application doesn't accept messages.\n\nFILE-OR-TYPE can either be a string denoting a MIME type, or a list\nwith one argument FILE, denoting a file whose associated application\nwill be launched.\n\nARGS can either be a vector of strings containing the arguments that\nwill be passed to the application, or a system message in the form\naccepted by `haiku-drag-message' that will be sent to the application\nafter it starts.  ",
       (file_or_type: Lisp_Object, args: Lisp_Object),
       {
  let mut cargs: *mut *mut c_char;
  let mut r#type: *mut c_char;
  let mut file: *mut c_char;
  let mut team_id: team_id = 0;
  let mut rc: status_t;
  let mut i: ptrdiff_t = 0;
  let mut nargs: ptrdiff_t;
  let mut tem: Lisp_Object = Qnil ();
  let mut canonical: Lisp_Object = Qnil ();
  let mut message: *mut c_void;
  let mut depth: specpdl_ref;

  r#type = std::ptr::null_mut ();
  file = std::ptr::null_mut ();
  cargs = std::ptr::null_mut ();
  message = std::ptr::null_mut ();
  nargs = 0;
  depth = SPECPDL_INDEX ();

  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  if STRINGP (file_or_type)
    { SAFE_ALLOCA_STRING! (sa_avail, r#type, file_or_type); }
  else
    {
      CHECK_LIST (file_or_type);
      tem = XCAR (file_or_type);
      canonical = Fexpand_file_name (tem, Qnil ());

      CHECK_STRING (tem);
      SAFE_ALLOCA_STRING! (sa_avail, file, ENCODE_FILE (canonical));
      CHECK_LIST_END (XCDR (file_or_type), file_or_type);
    }

  if VECTORP (args)
    {
      nargs = ASIZE (args);
      cargs = SAFE_ALLOCA! (sa_avail,
			    nargs * std::mem::size_of::<*mut c_char> () as ptrdiff_t)
	as *mut *mut c_char;

      i = 0;
      while i < nargs
	{
	  tem = AREF (args, i);
	  CHECK_STRING (tem);
	  maybe_quit ();

	  *cargs.offset (i) = SAFE_ALLOCA! (sa_avail, SBYTES (tem) + 1) as *mut c_char;
	  memcpy (*cargs.offset (i) as *mut c_void, SDATA (tem) as *const c_void,
		  (SBYTES (tem) + 1) as size_t);
	  i += 1;
	}
    }
  else
    {
      message = be_create_simple_message ();

      record_unwind_protect_ptr (Some (BMessage_delete), message);
      haiku_lisp_to_message (args, message);
    }

  block_input ();
  rc = be_roster_launch (r#type, file, cargs, nargs, message, &raw mut team_id);
  unblock_input ();

  /* `be_roster_launch' can potentially take a while in IO, but
     signals from async input will interrupt that operation.  If the
     user wanted to quit, act like it.  */
  maybe_quit ();

  if rc == B_OK
    { return SAFE_FREE_UNBIND_TO! (sa_count, depth, make_uint (team_id as uintmax_t)); }
  else if rc == B_ALREADY_RUNNING
    { return Qalready_running (); }
  else if rc == B_BAD_VALUE
    { signal_error (c"Invalid type or bad arguments".as_ptr (),
		    list2 (file_or_type, args)); }

  return SAFE_FREE_UNBIND_TO! (sa_count, depth, Qnil ());
});

// C 1028-1060.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-write-node-attribute", Fhaiku_write_node_attribute,
       Shaiku_write_node_attribute, 3, 3, 0,
       doc: "Write a message as a file-system attribute of NODE.\nFILE should be a file name of a file on a Be File System volume, NAME\nshould be a string describing the name of the attribute that will be\nwritten, and MESSAGE will be the attribute written to FILE, as a\nsystem message in the format accepted by `haiku-drag-message', which\nsee.  ",
       (file: Lisp_Object, name: Lisp_Object, message: Lisp_Object),
       {
  let mut file = file;                 /* C 1047 reassigns the parameter.  */
  let mut name = name;                 /* C 1048 reassigns the parameter.  */
  let mut be_message: *mut c_void;
  let mut rc: status_t;
  let mut count: specpdl_ref;

  CHECK_STRING (file);
  CHECK_STRING (name);

  file = ENCODE_FILE (file);
  name = ENCODE_SYSTEM (name);

  be_message = be_create_simple_message ();
  count = SPECPDL_INDEX ();

  record_unwind_protect_ptr (Some (BMessage_delete), be_message);
  haiku_lisp_to_message (message, be_message);
  rc = be_write_node_message (SSDATA (file), SSDATA (name), be_message);

  if rc < B_OK
    { haiku_report_system_error (rc, c"Failed to set attribute: %s".as_ptr ()); }

  return unbind_to (count, Qnil ());
});

// C 1062-1086.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-send-message", Fhaiku_send_message, Shaiku_send_message, 2, 2, 0,
       doc: "Send a system message to PROGRAM.\nPROGRAM must be the name of the application to which the message will\nbe sent.  MESSAGE is the system message, serialized in the format\naccepted by `haiku-drag-message', that will be sent to the application\nspecified by PROGRAM.  There is no guarantee that the message will\narrive after this function is called.  ",
       (program: Lisp_Object, message: Lisp_Object),
       {
  let mut program = program;           /* C 1077 reassigns the parameter.  */
  let mut count: specpdl_ref;
  let mut be_message: *mut c_void;

  CHECK_STRING (program);
  program = ENCODE_SYSTEM (program);

  be_message = be_create_simple_message ();
  count = SPECPDL_INDEX ();

  record_unwind_protect_ptr (Some (BMessage_delete), be_message);
  haiku_lisp_to_message (message, be_message);
  be_send_message (SSDATA (program), be_message);

  return unbind_to (count, Qnil ());
});

/* C 1088-1124: `static void haiku_dnd_compute_tip_xy (int *root_x, int *root_y)`.  C's
   `*root_y + XFIXNUM (tip_dy)` promotes the `int` to the `EMACS_INT` the pointer expression
   yields; the port spells each implicit promotion (PORTING.md 1.1) and narrows again on the
   assignments C's `int` truncates.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_dnd_compute_tip_xy (root_x: *mut c_int, root_y: *mut c_int)
{
  let mut min_x: c_int;
  let mut min_y: c_int;
  let mut max_x: c_int = 0;
  let mut max_y: c_int = 0;
  let mut width: c_int;
  let mut height: c_int;

  width = FRAME_PIXEL_WIDTH (XFRAME (tip_frame));
  height = FRAME_PIXEL_HEIGHT (XFRAME (tip_frame));

  min_x = 0;
  min_y = 0;
  be_get_screen_dimensions (&raw mut max_x, &raw mut max_y);

  if *root_y as EMACS_INT + XFIXNUM (tip_dy) <= min_y as EMACS_INT
    { *root_y = min_y; }                     /* Can happen for negative dy */
  else if *root_y as EMACS_INT + XFIXNUM (tip_dy) + height as EMACS_INT <= max_y as EMACS_INT
    { *root_y = (*root_y as EMACS_INT + XFIXNUM (tip_dy)) as c_int; }  /* fits below the pointer */
  else if height as EMACS_INT + XFIXNUM (tip_dy) + min_y as EMACS_INT <= *root_y as EMACS_INT
    { *root_y = (*root_y as EMACS_INT - (height as EMACS_INT + XFIXNUM (tip_dy))) as c_int; }
  else
    { *root_y = min_y; }                     /* put it on the top */

  if *root_x as EMACS_INT + XFIXNUM (tip_dx) <= min_x as EMACS_INT
    { *root_x = 0; }                         /* Can happen for negative dx */
  else if *root_x as EMACS_INT + XFIXNUM (tip_dx) + width as EMACS_INT <= max_x as EMACS_INT
    { *root_x = (*root_x as EMACS_INT + XFIXNUM (tip_dx)) as c_int; }  /* fits to the right */
  else if width as EMACS_INT + XFIXNUM (tip_dx) + min_x as EMACS_INT <= *root_x as EMACS_INT
    { *root_x = (*root_x as EMACS_INT - (width as EMACS_INT + XFIXNUM (tip_dx))) as c_int; }
  else
    { *root_x = min_x; }   /* put it left justified on the screen - it ought to fit that way */
}

/* C 1126-1133: `static Lisp_Object haiku_note_drag_motion_1 (void *data)`.  C's body has no
   final `return`; the port spells the nil C's callers see (`internal_catch_all` discards the
   value at C 1165), recorded in the ledger's Deviations.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_note_drag_motion_1 (data: *mut c_void) -> Lisp_Object
{
  if !NILP (Vhaiku_drag_track_function)
    { return call0 (Vhaiku_drag_track_function); }

  return Qnil ();
}

// C 1135-1139.
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_note_drag_motion_2 (exit: nonlocal_exit, error: Lisp_Object)
  -> Lisp_Object
{
  return Qnil ();
}

// C 1141-1171.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_note_drag_motion ()
{
  let mut tip_f: *mut frame;
  let mut x: c_int = 0;
  let mut y: c_int = 0;

  if FRAMEP (tip_frame) && haiku_dnd_follow_tooltip
      && FIXNUMP (tip_dx) && FIXNUMP (tip_dy)
    {
      tip_f = XFRAME (tip_frame);

      if FRAME_LIVE_P (tip_f) && FRAME_VISIBLE_P (tip_f)
	{
	  BView_get_mouse (FRAME_HAIKU_VIEW! (haiku_dnd_frame), &raw mut x, &raw mut y);
	  BView_convert_to_screen (FRAME_HAIKU_VIEW! (haiku_dnd_frame), &raw mut x, &raw mut y);

	  haiku_dnd_compute_tip_xy (&raw mut x, &raw mut y);
	  BWindow_set_offset (FRAME_HAIKU_WINDOW! (tip_f), x, y);
	}
    }

  internal_catch_all (Some (haiku_note_drag_motion_1), std::ptr::null_mut (),
		      Some (haiku_note_drag_motion_2));

  /* Redisplay this way to preserve the echo area.  Otherwise, the
     contents will abruptly disappear when the mouse moves over a
     frame.  */
  redisplay_preserve_echo_area (34);
}

/* termhooks.h 373-412: `enum event_kind kind : 16` shares the mirror's `input_event::bits` unit
   (PORTING.md 1.3 / P-80's member-named shape), so this unit spells C 1183/1186/1188's `ie->kind`
   reads and its write with its own copy (the `src/pgtkterm.rs` 191 / `src/xterm.rs` 1367 shape;
   this unit's items are gated because the frozen configuration compiles none of them).  */
/// C: termhooks.h 373-412: `enum event_kind kind : 16;` in `struct input_event`
#[cfg (feature = "have_haiku")]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
}

// C 1173-1197.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_note_drag_wheel (ie: *mut input_event)
{
  let mut horizontal: bool;
  let mut up: bool;

  up = false;
  horizontal = false;

  if ((*ie).modifiers & up_modifier as c_uint) != 0
    { up = true; }

  if input_event_bits! (kind, ie) == HORIZ_WHEEL_EVENT
    { horizontal = true; }

  input_event_bits! (kind, ie, NO_EVENT);

  if !NILP (Vhaiku_drag_wheel_function)
      && (haiku_dnd_allow_same_frame
	  || XFRAME ((*ie).frame_or_window) != haiku_dnd_frame)
    {
      safe_calln! (Vhaiku_drag_wheel_function, (*ie).frame_or_window,
		   (*ie).x, (*ie).y, if horizontal { Qt () } else { Qnil () },
		   if up { Qt () } else { Qnil () },
		   make_int ((*ie).modifiers as intmax_t));
    }

  redisplay_preserve_echo_area (35);
}

// C 1199-1203.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_haiku_select ()
{
  be_clipboard_init ();
}

// C 1205-1221.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_handle_selection_clear (ie: *mut input_event)
{
  let mut id: haiku_clipboard;

  id = haiku_get_clipboard_name ((*ie).arg);

  if be_selection_outdated_p (id, (*ie).timestamp)
    { return; }

  CALLN! (Frun_hook_with_args, Qhaiku_lost_selection_functions (), (*ie).arg);

  /* This is required for redisplay to happen if something changed the
     display inside the selection loss functions.  */
  redisplay_preserve_echo_area (20);
}

// C 1223-1248.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_selection_disowned (id: haiku_clipboard, count: int64)
{
  let mut ie: input_event = std::mem::zeroed ();

  EVENT_INIT (&raw mut ie);
  input_event_bits! (kind, &raw mut ie, SELECTION_CLEAR_EVENT);

  /* C's `switch (id)` has no `default:`; the `_` arm is Rust's requirement, and it `break`s for
     the same reason every other arm does.  */
  'switch: {
  match id
    {
    CLIPBOARD_CLIPBOARD => {
      ie.arg = QCLIPBOARD ();
      break 'switch;
    }

    CLIPBOARD_PRIMARY => {
      ie.arg = QPRIMARY ();
      break 'switch;
    }

    CLIPBOARD_SECONDARY => {
      ie.arg = QSECONDARY ();
      break 'switch;
    }

    _ => {
      break 'switch;
    }
    }
  } /* 'switch */

  ie.timestamp = count;
  kbd_buffer_store_event (&raw mut ie as *mut c_void);
}

// C 1250-1256.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn haiku_start_watching_selections ()
{
  be_start_watching_selection (CLIPBOARD_CLIPBOARD);
  be_start_watching_selection (CLIPBOARD_PRIMARY);
  be_start_watching_selection (CLIPBOARD_SECONDARY);
}

// C 1258-1261: the page break and the "Notification support." banner C writes here.
// Notification support.

/* C 1262-1297: `static intmax_t haiku_notifications_notify_1 (…)`.  C's local is named `type`,
   which is a Rust keyword, so the port keeps the name as the raw identifier `r#type`
   (PORTING.md; the `src/alloc.rs` 585 field shape).  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_notifications_notify_1 (title: Lisp_Object, body: Lisp_Object,
					replaces_id: Lisp_Object, app_icon: Lisp_Object,
					urgency: Lisp_Object) -> intmax_t
{
  let mut r#type: c_int;
  let mut supersedes: intmax_t;
  let mut icon: *const c_char;

  if EQ (urgency, Qlow ())
    { r#type = 0; }
  else if EQ (urgency, Qnormal ())
    { r#type = 1; }
  else if EQ (urgency, Qcritical ())
    { r#type = 2; }
  else
    { signal_error (c"Invalid notification type provided".as_ptr (), urgency); }

  supersedes = -1;

  if !NILP (replaces_id)
    {
      CHECK_INTEGER (replaces_id);
      if !integer_to_intmax (replaces_id, &raw mut supersedes)
	{ supersedes = -1; }
    }

  icon = std::ptr::null ();

  if !NILP (app_icon)
    { icon = SSDATA (ENCODE_FILE (app_icon)); }

  /* GC should not transpire from here onwards.  */
  return be_display_notification (SSDATA (title), SSDATA (body), supersedes, r#type, icon);
}

// C 1299-1370.
#[cfg (feature = "have_haiku")]
DEFUN! ("haiku-notifications-notify", Fhaiku_notifications_notify,
       Shaiku_notifications_notify, 0, MANY, 0, doc:
       "Display a desktop notification.\nARGS must contain keywords followed by values.  Each of the following\nkeywords is understood:\n\n  :title        The notification title.\n  :body         The notification body.\n  :replaces-id  The ID of a previous notification to supersede.\n  :app-icon     The file name of the notification's icon, if any.\n  :urgency      One of the symbols `low', `normal' or `critical',\n                specifying the importance of the notification.\n\n:title and :body must be provided.  Value is an integer (fixnum or\nbignum) identifying the notification displayed.\n\nusage: (haiku-notifications-notify &rest ARGS)  ",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut title: Lisp_Object = Qnil ();
  let mut body: Lisp_Object = Qnil ();
  let mut replaces_id: Lisp_Object = Qnil ();
  let mut app_icon: Lisp_Object = Qnil ();
  let mut urgency: Lisp_Object = Qnil ();
  let mut key: Lisp_Object = Qnil ();
  let mut value: Lisp_Object = Qnil ();
  let mut i: ptrdiff_t;

  /* First, clear each of the variables above.  */
  title = Qnil (); body = Qnil (); replaces_id = Qnil ();
  app_icon = Qnil (); urgency = Qnil ();

  /* If NARGS is odd, error.  */

  if (nargs & 1) != 0
    { error (c"Odd number of arguments in call to `haiku-notifications-notify'".as_ptr ()); }

  /* Next, iterate through ARGS, searching for arguments.  */

  i = 0;
  while i < nargs
    {
      key = *args.offset (i);
      value = *args.offset (i + 1);

      if EQ (key, QCtitle ())
	{ title = value; }
      else if EQ (key, QCbody ())
	{ body = value; }
      else if EQ (key, QCreplaces_id ())
	{ replaces_id = value; }
      else if EQ (key, QCapp_icon ())
	{ app_icon = value; }
      else if EQ (key, QCurgency ())
	{ urgency = value; }

      i += 2;
    }

  /* Demand at least TITLE and BODY be present.  */

  if NILP (title) || NILP (body)
    { error (c"Title or body not provided".as_ptr ()); }

  /* Now check the type and possibly expand each non-nil argument.  */

  CHECK_STRING (title);
  title = ENCODE_UTF_8 (title);
  CHECK_STRING (body);
  body = ENCODE_UTF_8 (body);

  if NILP (urgency)
    { urgency = Qlow (); }

  if !NILP (app_icon)
    { app_icon = Fexpand_file_name (app_icon, Qnil ()); }

  return make_int (haiku_notifications_notify_1 (title, body, replaces_id, app_icon, urgency));
});

/* C 1372-1379, 1381-1385, 1387-1390, 1392-1400: the four DEFVAR_* variables this file owns.
   C's storage is `globals.f_<vname>` (globals.h), so the port defines it here and
   `include/globals.rs`'s `variables` declaration is shadowed (the `src/haikufns.rs` 4150-4182
   shape).  */
// C 1375: `DEFVAR_BOOL ("haiku-signal-invalid-refs", haiku_signal_invalid_refs, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut haiku_signal_invalid_refs: bool = false;
// C 1381: `DEFVAR_LISP ("haiku-drag-track-function", Vhaiku_drag_track_function, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut Vhaiku_drag_track_function: Lisp_Object = lisp_h_Qnil ();
// C 1387: `DEFVAR_LISP ("haiku-lost-selection-functions", Vhaiku_lost_selection_functions, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut Vhaiku_lost_selection_functions: Lisp_Object = lisp_h_Qnil ();
// C 1392: `DEFVAR_LISP ("haiku-drag-wheel-function", Vhaiku_drag_wheel_function, ...)`.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut Vhaiku_drag_wheel_function: Lisp_Object = lisp_h_Qnil ();

// C 1372-1450.
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_haikuselect ()
{
  DEFVAR_BOOL! ("haiku-signal-invalid-refs", haiku_signal_invalid_refs,
    "If nil, silently ignore invalid file names in system messages.\nOtherwise, an error will be signaled if adding a file reference to a\nsystem message failed.  ");
  haiku_signal_invalid_refs = true;

  DEFVAR_LISP! ("haiku-drag-track-function", Vhaiku_drag_track_function,
    "If non-nil, a function to call upon mouse movement while dragging a message.\nThe function is called without any arguments.  `mouse-position' can be\nused to retrieve the current position of the mouse.  ");
  Vhaiku_drag_track_function = Qnil ();

  DEFVAR_LISP! ("haiku-lost-selection-functions", Vhaiku_lost_selection_functions,
    "A list of functions to be called when Emacs loses an X selection.\nThese are only called if a connection to the Haiku display was opened.  ");
  Vhaiku_lost_selection_functions = Qnil ();

  DEFVAR_LISP! ("haiku-drag-wheel-function", Vhaiku_drag_wheel_function,
    "Function called upon wheel movement while dragging a message.\nIf non-nil, it is called with 6 arguments when the mouse wheel moves\nwhile a drag-and-drop operation is in progress: the frame where the\nmouse moved, the frame-relative X and Y positions where the mouse\nmoved, whether or not the wheel movement was horizontal, whether or\nnot the wheel moved up (or left, if the movement was horizontal), and\nkeyboard modifiers currently held down.  ");
  Vhaiku_drag_wheel_function = Qnil ();

  DEFSYM! (QSECONDARY, "SECONDARY");
  DEFSYM! (QCLIPBOARD, "CLIPBOARD");
  DEFSYM! (QSTRING, "STRING");
  DEFSYM! (QUTF8_STRING, "UTF8_STRING");
  DEFSYM! (Qforeign_selection, "foreign-selection");
  DEFSYM! (QTARGETS, "TARGETS");

  DEFSYM! (Qhaiku_lost_selection_functions,
	   "haiku-lost-selection-functions");

  DEFSYM! (Qmessage, "message");
  DEFSYM! (Qstring, "string");
  DEFSYM! (Qref, "ref");
  DEFSYM! (Qshort, "short");
  DEFSYM! (Qlong, "long");
  DEFSYM! (Qllong, "llong");
  DEFSYM! (Qbyte, "byte");
  DEFSYM! (Qchar, "char");
  DEFSYM! (Qbool, "bool");
  DEFSYM! (Qtype, "type");
  DEFSYM! (Qsize_t, "size_t");
  DEFSYM! (Qssize_t, "ssize_t");
  DEFSYM! (Qpoint, "point");
  DEFSYM! (Qfloat, "float");
  DEFSYM! (Qdouble, "double");
  DEFSYM! (Qalready_running, "already-running");

  DEFSYM! (QCtitle, ":title");
  DEFSYM! (QCbody, ":body");
  DEFSYM! (QCreplaces_id, ":replaces-id");
  DEFSYM! (QCapp_icon, ":app-icon");
  DEFSYM! (QCurgency, ":urgency");

  DEFSYM! (Qlow, "low");
  DEFSYM! (Qnormal, "normal");
  DEFSYM! (Qcritical, "critical");

  defsubr (&raw mut Shaiku_selection_data);
  defsubr (&raw mut Shaiku_selection_timestamp);
  defsubr (&raw mut Shaiku_selection_put);
  defsubr (&raw mut Shaiku_selection_owner_p);
  defsubr (&raw mut Shaiku_drag_message);
  defsubr (&raw mut Shaiku_roster_launch);
  defsubr (&raw mut Shaiku_write_node_attribute);
  defsubr (&raw mut Shaiku_send_message);
  defsubr (&raw mut Shaiku_notifications_notify);

  haiku_dnd_frame = std::ptr::null_mut ();
}
