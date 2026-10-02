// femacs: port of src/buffer.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// Header mirror: types, constants, macros, inline functions and the `unsafe
// extern "C"` declarations C spells as `extern` (PORTING.md 0.1).  The macros and
// inline accessors are the same statements in the same order as C's.
//
// One of the reference file's macros is deferred because its Rust form is the open
// P-22 decision (`FOR_EACH_LIVE_BUFFER`); it is recorded in
// docs/ports/excluded-macros.txt and commented in place.  docs/ports/include-buffer.md
// records every lexical deviation this mirror takes, and what the gates do and do not
// cover for it (the two adversarial reviews of 2026-10-02 are summarised there).

/* Copyright (C) 1985-1986, 1994, 1995, 1997-2002, 2004-2026 Free Software
   Foundation, Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

// src/buffer.h 23-24 include <sys/types.h> and <time.h>; the port reaches the two
// system mirrors the same way (both are WIP partial mirrors, PORTING.md P-26).
include!("../../lib/include/sys_types.in.rs");
include!("../../lib/include/time.in.rs");

// C's include guard `EMACS_BUFFER_H` has no Rust counterpart; the constant below exists
// only so the mirror's name set matches the reference header's (PORTING.md 0.1, P-24).
pub const EMACS_BUFFER_H: bool = true;

// C reaches the current buffer through a thread-local macro
// (`#define current_buffer (current_thread->m_current_buffer)`, thread.h 147)
// via lisp.h.  thread.rs is not written yet, so the mirror declares the storage
// here and the accessor arrives with it, exactly as lisp.rs already does for
// the specpdl trio (PORTING.md P-19).
unsafe extern "C" { pub static mut current_buffer: *mut buffer; }

/* Accessing the parameters of the current buffer.  */

/* These constants and macros come in pairs, one for the char position
   and one for the byte position.  */

/* Position of beginning of buffer.  */
pub const BEG: c_int = 1;
pub const BEG_BYTE: c_int = BEG;

/* Accessing the parameters of the current buffer.  */

/* These constants and macros come in pairs, one for the char position
   and one for the byte position.  */

// C's object-like macros become `macro_rules!` whose body is C's body, so that the
// 159 statements in the reference that *write* to `BEGV`/`Z`/`GPT`/`MODIFF`/… and the
// 11 that take `&MODIFF` stay expressible (`NAME! ()` is a place expression, exactly
// as C's macro is a place; a `fn` would be neither assignable nor addressable).  The
// call site is therefore `NAME! ()` where C writes `NAME` — the one spelling
// deviation this shape costs (PORTING.md P-37).  `PT`/`PT_BYTE` keep C's `+ 0`, which
// is what makes them not l-values (PORTING.md 0.1-3).

/* Position of beginning of accessible range of buffer.  */
#[macro_export]
macro_rules! BEGV { () => { (*current_buffer).begv } }
#[macro_export]
macro_rules! BEGV_BYTE { () => { (*current_buffer).begv_byte } }

/* Position of point in buffer.  The "+ 0" makes this
   not an l-value, so you can't assign to it.  Use SET_PT instead.  */
#[macro_export]
macro_rules! PT { () => { ((*current_buffer).pt + 0) } }
#[macro_export]
macro_rules! PT_BYTE { () => { ((*current_buffer).pt_byte + 0) } }

/* Position of gap in buffer.  */
#[macro_export]
macro_rules! GPT { () => { (*(*current_buffer).text).gpt } }
#[macro_export]
macro_rules! GPT_BYTE { () => { (*(*current_buffer).text).gpt_byte } }

/* Position of end of accessible range of buffer.  */
#[macro_export]
macro_rules! ZV { () => { (*current_buffer).zv } }
#[macro_export]
macro_rules! ZV_BYTE { () => { (*current_buffer).zv_byte } }

/* Position of end of buffer.  */
#[macro_export]
macro_rules! Z { () => { (*(*current_buffer).text).z } }
#[macro_export]
macro_rules! Z_BYTE { () => { (*(*current_buffer).text).z_byte } }

/* Macros for the addresses of places in the buffer.  */

/* WARNING: Use the 'char *' pointers to buffer text with care in code
   that could GC: GC can relocate buffer text, invalidating such
   pointers.  It is best to use character or byte position instead,
   delaying the access through BYTE_POS_ADDR etc. pointers to the
   latest possible moment.  If you must use the 'char *' pointers
   (e.g., for speed), be sure to adjust them after any call that could
   potentially GC.  */

/* Address of beginning of buffer.  */
#[macro_export]
macro_rules! BEG_ADDR { () => { (*(*current_buffer).text).beg } }

/* Address of beginning of accessible range of buffer.  */
#[macro_export]
macro_rules! BEGV_ADDR { () => { BYTE_POS_ADDR ((*current_buffer).begv_byte) } }

/* Address of point in buffer.  */
#[macro_export]
macro_rules! PT_ADDR { () => { BYTE_POS_ADDR ((*current_buffer).pt_byte) } }

/* Address of beginning of gap in buffer.  */
#[macro_export]
macro_rules! GPT_ADDR { () => { (*(*current_buffer).text).beg.offset ((*(*current_buffer).text).gpt_byte - BEG_BYTE as isize) } }

/* Address of end of gap in buffer.  */
#[macro_export]
macro_rules! GAP_END_ADDR { () => { (*(*current_buffer).text).beg.offset ((*(*current_buffer).text).gpt_byte + (*(*current_buffer).text).gap_size - BEG_BYTE as isize) } }

/* Address of end of accessible range of buffer.  */
#[macro_export]
macro_rules! ZV_ADDR { () => { BYTE_POS_ADDR ((*current_buffer).zv_byte) } }

/* Address of end of buffer.  */
#[macro_export]
macro_rules! Z_ADDR { () => { (*(*current_buffer).text).beg.offset ((*(*current_buffer).text).gap_size + (*(*current_buffer).text).z_byte - BEG_BYTE as isize) } }

/* Size of gap.  */
#[macro_export]
macro_rules! GAP_SIZE { () => { (*(*current_buffer).text).gap_size } }

/* Modification count.  */
#[macro_export]
macro_rules! MODIFF { () => { (*(*current_buffer).text).modiff } }

/* Character modification count.  */
#[macro_export]
macro_rules! CHARS_MODIFF { () => { (*(*current_buffer).text).chars_modiff } }

/* Overlay modification count.  */
#[macro_export]
macro_rules! OVERLAY_MODIFF { () => { (*(*current_buffer).text).overlay_modiff } }

/* Modification count as of last visit or save.  */
#[macro_export]
macro_rules! SAVE_MODIFF { () => { (*(*current_buffer).text).save_modiff } }

/* Position of gap in buffer.  */
#[macro_export]
macro_rules! BUF_GPT { ($buf:expr) => { (*(*$buf).text).gpt } }
#[macro_export]
macro_rules! BUF_GPT_BYTE { ($buf:expr) => { (*(*$buf).text).gpt_byte } }

/* Position of end of buffer.  */
#[macro_export]
macro_rules! BUF_Z { ($buf:expr) => { (*(*$buf).text).z } }
#[macro_export]
macro_rules! BUF_Z_BYTE { ($buf:expr) => { (*(*$buf).text).z_byte } }

/* Address of beginning of buffer.  */
#[macro_export]
macro_rules! BUF_BEG_ADDR { ($buf:expr) => { (*(*$buf).text).beg } }

/* Size of gap.  */
#[macro_export]
macro_rules! BUF_GAP_SIZE { ($buf:expr) => { (*(*$buf).text).gap_size } }

/* Modification count.  */
#[macro_export]
macro_rules! BUF_MODIFF { ($buf:expr) => { (*(*$buf).text).modiff } }

/* Character modification count.  */
#[macro_export]
macro_rules! BUF_CHARS_MODIFF { ($buf:expr) => { (*(*$buf).text).chars_modiff } }

/* Modification count as of last visit or save.  */
#[macro_export]
macro_rules! BUF_SAVE_MODIFF { ($buf:expr) => { (*(*$buf).text).save_modiff } }

/* Overlay modification count.  */
#[macro_export]
macro_rules! BUF_OVERLAY_MODIFF { ($buf:expr) => { (*(*$buf).text).overlay_modiff } }

/* Modification count as of last auto-save.  */
/* FIXME: should we move this into ->text->auto_save_modiff?  */
#[macro_export]
macro_rules! BUF_AUTOSAVE_MODIFF { ($buf:expr) => { (*$buf).auto_save_modified } }

/* Compaction count.  */
#[macro_export]
macro_rules! BUF_COMPACT { ($buf:expr) => { (*(*$buf).text).compact } }

/* Marker chain of buffer.  */
#[macro_export]
macro_rules! BUF_MARKERS { ($buf:expr) => { (*(*$buf).text).markers } }

#[macro_export]
macro_rules! BUF_UNCHANGED_MODIFIED { ($buf:expr) => { (*(*$buf).text).unchanged_modified } }

#[macro_export]
macro_rules! BUF_OVERLAY_UNCHANGED_MODIFIED { ($buf:expr) => { (*(*$buf).text).overlay_unchanged_modified } }
#[macro_export]
macro_rules! BUF_BEG_UNCHANGED { ($buf:expr) => { (*(*$buf).text).beg_unchanged } }
#[macro_export]
macro_rules! BUF_END_UNCHANGED { ($buf:expr) => { (*(*$buf).text).end_unchanged } }

#[macro_export]
macro_rules! UNCHANGED_MODIFIED { () => { BUF_UNCHANGED_MODIFIED! (current_buffer) } }
#[macro_export]
macro_rules! OVERLAY_UNCHANGED_MODIFIED { () => { BUF_OVERLAY_UNCHANGED_MODIFIED! (current_buffer) } }
#[macro_export]
macro_rules! BEG_UNCHANGED { () => { BUF_BEG_UNCHANGED! (current_buffer) } }
#[macro_export]
macro_rules! END_UNCHANGED { () => { BUF_END_UNCHANGED! (current_buffer) } }

/* Functions to set PT in the current buffer, or another buffer.  */

unsafe extern "C" {
  // Parameter names come from the definitions (C's prototypes have none):
  // intervals.c 1766/1776/1849/1751 and buffer.c 4647.
  pub fn set_point (charpos: ptrdiff_t);
  pub fn temp_set_point (buffer: *mut buffer, charpos: ptrdiff_t);
  pub fn set_point_both (charpos: ptrdiff_t, bytepos: ptrdiff_t);
  pub fn temp_set_point_both (buffer: *mut buffer, charpos: ptrdiff_t, bytepos: ptrdiff_t);
  pub fn set_point_from_marker (marker: Lisp_Object);
  pub fn enlarge_buffer_text (b: *mut buffer, delta: ptrdiff_t);
}

#[inline(always)]
pub fn SET_PT (position: ptrdiff_t)
{
  unsafe { set_point (position); }
}
#[inline(always)]
pub fn TEMP_SET_PT (position: ptrdiff_t)
{
  unsafe { temp_set_point (current_buffer, position); }
}
#[inline(always)]
pub fn SET_PT_BOTH (position: ptrdiff_t, byte: ptrdiff_t)
{
  unsafe { set_point_both (position, byte); }
}
#[inline(always)]
pub fn TEMP_SET_PT_BOTH (position: ptrdiff_t, byte: ptrdiff_t)
{
  unsafe { temp_set_point_both (current_buffer, position, byte); }
}
#[inline(always)]
pub fn BUF_TEMP_SET_PT (buffer: *mut buffer, position: ptrdiff_t)
{
  unsafe { temp_set_point (buffer, position); }
}

/* Maximum number of bytes in a buffer.
   A buffer cannot contain more bytes than a 1-origin fixnum can represent,
   nor can it be so large that C pointer arithmetic stops working.
   The ptrdiff_t cast ensures that this is signed, not unsigned.  */
// C: `(ptrdiff_t) min (MOST_POSITIVE_FIXNUM - 1, min (SIZE_MAX, PTRDIFF_MAX))`.
// C's `min` runs each comparison in the common type of its operands, so the two
// unsigned bounds and the signed one are all compared as unsigned here; the
// result is MOST_POSITIVE_FIXNUM - 1 because it is the smallest of the three.
pub const BUF_BYTES_MAX: isize = {
  let fixnum_bound = (MOST_POSITIVE_FIXNUM - 1) as u64;
  let inner = if (SIZE_MAX as u64) < (PTRDIFF_MAX as u64) { SIZE_MAX as u64 } else { PTRDIFF_MAX as u64 };
  (if fixnum_bound < inner { fixnum_bound } else { inner }) as isize
};

/* Maximum gap size after compact_buffer, in bytes.  Also
   used in make_gap_larger to get some extra reserved space.  */

pub const GAP_BYTES_DFL: c_int = 2000;

/* Minimum gap size after compact_buffer, in bytes.  Also
   used in make_gap_smaller to avoid too small gap size.  */

pub const GAP_BYTES_MIN: c_int = 20;

/* For those very rare cases where you may have a "random" pointer into
   the middle of a multibyte char, this moves to the next boundary.  */
unsafe extern "C" {
  pub fn advance_to_char_boundary (byte_pos: ptrdiff_t) -> ptrdiff_t;
}

/* Return the byte at byte position N.
   Do not check that the position is in range.  */

#[macro_export]
macro_rules! FETCH_BYTE { ($n:expr) => { (*BYTE_POS_ADDR ($n)) } }

/* Define the actual buffer data structures.  */

/* This data structure stores the cache of a position and its line and
   column number.  The column number is counted in bytes.  The line
   number and column number don't respect narrowing.  */
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct ts_linecol {
    /// The byte position.
    pub bytepos: isize,
    /// The line number of this position.
    pub line: isize,
    /// The column number (in bytes) of this position (0-based).  Basically
    /// the byte offset from BOL (or BOB).
    pub col: isize,
}

/* This data structure describes the actual text contents of a buffer.
   It is shared between indirect buffers and their base buffer.  */
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct buffer_text {
    /// Actual address of buffer contents.  If REL_ALLOC is defined,
    /// this address might change when blocks are relocated which can
    /// e.g. happen when malloc is called.  So, don't pass a pointer
    /// into a buffer's text to functions that malloc.
    pub beg: *mut u8,
    /// Char pos of gap in buffer.
    pub gpt: isize,
    /// Char pos of end of buffer.
    pub z: isize,
    /// Byte pos of gap in buffer.
    pub gpt_byte: isize,
    /// Byte pos of end of buffer.
    pub z_byte: isize,
    /// Size of buffer's gap.
    pub gap_size: isize,
    /// This counts buffer-modification events
    /// for this buffer.  It is increased
    /// logarithmically to the extent of the
    /// modification for each such event,
    /// and never otherwise changed.
    pub modiff: modiff_count,
    /// This is modified with character change
    /// events for this buffer.  It is set to
    /// modiff for each such event, and never
    /// otherwise changed.
    pub chars_modiff: modiff_count,
    /// Previous value of modiff, as of last
    /// time buffer visited or saved a file.
    pub save_modiff: modiff_count,
    /// Counts modifications to overlays.
    pub overlay_modiff: modiff_count,
    /// Set to modiff each time when compact_buffer
    /// is called for this buffer.
    pub compact: modiff_count,
    /// Minimum value of GPT - BEG since last redisplay that finished.
    pub beg_unchanged: isize,
    /// Minimum value of Z - GPT since last redisplay that finished.
    pub end_unchanged: isize,
    /// MODIFF as of last redisplay that finished; if it matches MODIFF,
    /// beg_unchanged and end_unchanged contain no useful information.
    pub unchanged_modified: modiff_count,
    /// BUF_OVERLAY_MODIFF of current buffer, as of last redisplay that
    /// finished; if it matches BUF_OVERLAY_MODIFF, beg_unchanged and
    /// end_unchanged contain no useful information.
    pub overlay_unchanged_modified: modiff_count,
    /// Properties of this buffer's text.
    pub intervals: INTERVAL,
    /// The markers that refer to this buffer.
    /// This is actually a single marker ---
    /// successive elements in its marker `chain'
    /// are the other markers referring to this buffer.
    /// This is a singly linked unordered list, which means that it's
    /// very cheap to add a marker to the list and it's also very cheap
    /// to move a marker within a buffer.
    pub markers: *mut Lisp_Marker,
    /// Usually false.  Temporarily true in decode_coding_gap to
    /// prevent Fgarbage_collect from shrinking the gap and losing
    /// not-yet-decoded bytes.
    /// bool_bf inhibit_shrinking : 1;
    /// True if it needs to be redisplayed.
    /// bool_bf redisplay : 1;
    /// clang's record layout puts the two bits at 136:0 and 136:1 of this byte.
    pub bits: u8,
}

/// `buffer_text`'s two `bool_bf` members, in buffer.h's declaration order (clang:
/// 136:0-136:1, the byte named `bits`).  C reads them through the bitfields; Rust reads
/// them by mask (PORTING.md 1.3), and `offsetof` cannot name a bitfield, so the numbers
/// are recorded here and checked by the C-side probe the ledger describes.
pub const BUFFER_TEXT_INHIBIT_SHRINKING: u8 = 1 << 0;
pub const BUFFER_TEXT_REDISPLAY: u8 = 1 << 1;

/* Most code should use this macro to access Lisp fields in struct buffer.  */

// C: `#define BVAR(buf, field) ((buf)->field ## _)`.  `macro_rules!` cannot paste
// tokens onto an identifier (`concat_idents!` is not stable), so the call site
// spells the member the way Rust spells it — with the `_` C's own macro appends —
// and the macro only dereferences.  PORTING.md P-37.
#[macro_export]
macro_rules! BVAR {
    ($buf:expr, $field:ident) => { (*$buf).$field };
}

/* Max number of builtin per-buffer variables.  */

pub const MAX_PER_BUFFER_VARS: c_int = 50;

/* Special values for struct buffer.modtime.  */

pub const NONEXISTENT_MODTIME_NSECS: c_int = -1;
pub const UNKNOWN_MODTIME_NSECS: c_int = -2;

/* This is the structure that the buffer Lisp object points to.  */
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct buffer {
    pub header: vectorlike_header,
    /// The name of this buffer.
    pub name_: Lisp_Object,
    /// The last name of this buffer before it was renamed or killed.
    pub last_name_: Lisp_Object,
    /// The name of the file visited in this buffer, or nil.
    pub filename_: Lisp_Object,
    /// Directory for expanding relative file names.
    pub directory_: Lisp_Object,
    /// True if this buffer has been backed up (if you write to the visited
    /// file and it hasn't been backed up, then a backup will be made).
    pub backed_up_: Lisp_Object,
    /// Length of file when last read or saved.
    /// -1 means auto saving turned off because buffer shrank a lot.
    /// -2 means don't turn off auto saving if buffer shrinks.
    /// (That value is used with buffer-swap-text.)
    /// This is not in the  struct buffer_text
    /// because it's not used in indirect buffers at all.
    pub save_length_: Lisp_Object,
    /// File name used for auto-saving this buffer.
    /// This is not in the  struct buffer_text
    /// because it's not used in indirect buffers at all.
    pub auto_save_file_name_: Lisp_Object,
    /// Non-nil if buffer read-only.
    pub read_only_: Lisp_Object,
    /// "The mark".  This is a marker which may
    /// point into this buffer or may point nowhere.
    pub mark_: Lisp_Object,
    /// Alist of elements (SYMBOL . VALUE-IN-THIS-BUFFER) for all
    /// per-buffer variables of this buffer.  For locally unbound
    /// symbols, just the symbol appears as the element.
    pub local_var_alist_: Lisp_Object,
    /// Symbol naming major mode (e.g., lisp-mode).
    pub major_mode_: Lisp_Object,
    /// Symbol listing all currently enabled minor modes.
    pub local_minor_modes_: Lisp_Object,
    /// Pretty name of major mode (e.g., "Lisp").
    pub mode_name_: Lisp_Object,
    /// Mode line element that controls format of mode line.
    pub mode_line_format_: Lisp_Object,
    /// Analogous to mode_line_format for the line displayed at the top
    /// of windows.  Nil means don't display that line.
    pub header_line_format_: Lisp_Object,
    /// Analogous to mode_line_format for the line displayed at the top
    /// of windows.  Nil means don't display that line.
    pub tab_line_format_: Lisp_Object,
    /// Keys that are bound local to this buffer.
    pub keymap_: Lisp_Object,
    /// This buffer's local abbrev table.
    pub abbrev_table_: Lisp_Object,
    /// This buffer's syntax table.
    pub syntax_table_: Lisp_Object,
    /// This buffer's category table.
    pub category_table_: Lisp_Object,
    /// Values of several buffer-local variables.
    /// tab-width is buffer-local so that redisplay can find it
    /// in buffers that are not current.
    pub tab_width_: Lisp_Object,
    pub fill_column_: Lisp_Object,
    pub left_margin_: Lisp_Object,
    /// Function to call when insert space past fill column.
    pub auto_fill_function_: Lisp_Object,
    /// Case table for case-conversion in this buffer.
    /// This char-table maps each char into its lower-case version.
    pub downcase_table_: Lisp_Object,
    /// Char-table mapping each char to its upper-case version.
    pub upcase_table_: Lisp_Object,
    /// Char-table for conversion for case-folding search.
    pub case_canon_table_: Lisp_Object,
    /// Char-table of equivalences for case-folding search.
    pub case_eqv_table_: Lisp_Object,
    /// Non-nil means do not display continuation lines.
    pub truncate_lines_: Lisp_Object,
    /// Non-nil means to use word wrapping when displaying continuation lines.
    pub word_wrap_: Lisp_Object,
    /// Non-nil means display ctl chars with uparrow.
    pub ctl_arrow_: Lisp_Object,
    /// Non-nil means reorder bidirectional text for display in the
    /// visual order.
    pub bidi_display_reordering_: Lisp_Object,
    /// If non-nil, specifies which direction of text to force in all the
    /// paragraphs of the buffer.  Nil means determine paragraph
    /// direction dynamically for each paragraph.
    pub bidi_paragraph_direction_: Lisp_Object,
    /// If non-nil, a regular expression for bidi paragraph separator.
    pub bidi_paragraph_separate_re_: Lisp_Object,
    /// If non-nil, a regular expression for bidi paragraph start.
    pub bidi_paragraph_start_re_: Lisp_Object,
    /// Non-nil means do selective display;
    /// see doc string in syms_of_buffer (buffer.c) for details.
    pub selective_display_: Lisp_Object,
    /// Non-nil means show ... at end of line followed by invisible lines.
    pub selective_display_ellipses_: Lisp_Object,
    /// t if "self-insertion" should overwrite; `binary' if it should also
    /// overwrite newlines and tabs - for editing executables and the like.
    pub overwrite_mode_: Lisp_Object,
    /// Non-nil means abbrev mode is on.  Expand abbrevs automatically.
    pub abbrev_mode_: Lisp_Object,
    /// Display table to use for text in this buffer.
    pub display_table_: Lisp_Object,
    /// t means the mark and region are currently active.
    pub mark_active_: Lisp_Object,
    /// Non-nil means the buffer contents are regarded as multi-byte
    /// form of characters, not a binary code.
    pub enable_multibyte_characters_: Lisp_Object,
    /// Coding system to be used for encoding the buffer contents on
    /// saving.
    pub buffer_file_coding_system_: Lisp_Object,
    /// List of symbols naming the file format used for visited file.
    pub file_format_: Lisp_Object,
    /// List of symbols naming the file format used for auto-save file.
    pub auto_save_file_format_: Lisp_Object,
    /// True if the newline position cache, width run cache and BIDI paragraph
    /// cache are enabled.  See search.c, indent.c and bidi.c for details.
    pub cache_long_scans_: Lisp_Object,
    /// If the width run cache is enabled, this table contains the
    /// character widths width_run_cache (see above) assumes.  When we
    /// do a thorough redisplay, we compare this against the buffer's
    /// current display table to see whether the display table has
    /// affected the widths of any characters.  If it has, we
    /// invalidate the width run cache, and re-initialize width_table.
    pub width_table_: Lisp_Object,
    /// In an indirect buffer, or a buffer that is the base of an
    /// indirect buffer, this holds a marker that records
    /// PT for this buffer when the buffer is not current.
    pub pt_marker_: Lisp_Object,
    /// In an indirect buffer, or a buffer that is the base of an
    /// indirect buffer, this holds a marker that records
    /// BEGV for this buffer when the buffer is not current.
    pub begv_marker_: Lisp_Object,
    /// In an indirect buffer, or a buffer that is the base of an
    /// indirect buffer, this holds a marker that records
    /// ZV for this buffer when the buffer is not current.
    pub zv_marker_: Lisp_Object,
    /// This holds the point value before the last scroll operation.
    /// Explicitly setting point sets this to nil.
    pub point_before_scroll_: Lisp_Object,
    /// Truename of the visited file, or nil.
    pub file_truename_: Lisp_Object,
    /// Invisibility spec of this buffer.
    /// t => any non-nil `invisible' property means invisible.
    /// A list => `invisible' property means invisible
    /// if it is memq in that list.
    pub invisibility_spec_: Lisp_Object,
    /// This is the last window that was selected with this buffer in it,
    /// or nil if that window no longer displays this buffer.
    pub last_selected_window_: Lisp_Object,
    /// Incremented each time the buffer is displayed in a window.
    pub display_count_: Lisp_Object,
    /// Widths of left and right marginal areas for windows displaying
    /// this buffer.
    pub left_margin_cols_: Lisp_Object,
    pub right_margin_cols_: Lisp_Object,
    /// Widths of left and right fringe areas for windows displaying
    /// this buffer.
    pub left_fringe_width_: Lisp_Object,
    pub right_fringe_width_: Lisp_Object,
    /// Non-nil means fringes are drawn outside display margins;
    /// othersize draw them between margin areas and text.
    pub fringes_outside_margins_: Lisp_Object,
    /// Width, height and types of scroll bar areas for windows displaying
    /// this buffer.
    pub scroll_bar_width_: Lisp_Object,
    pub scroll_bar_height_: Lisp_Object,
    pub vertical_scroll_bar_type_: Lisp_Object,
    pub horizontal_scroll_bar_type_: Lisp_Object,
    /// Non-nil means indicate lines not displaying text (in a style
    /// like vi).
    pub indicate_empty_lines_: Lisp_Object,
    /// Non-nil means indicate buffer boundaries and scrolling.
    pub indicate_buffer_boundaries_: Lisp_Object,
    /// Logical to physical fringe bitmap mappings.
    pub fringe_indicator_alist_: Lisp_Object,
    /// Logical to physical cursor bitmap mappings.
    pub fringe_cursor_alist_: Lisp_Object,
    /// Time stamp updated each time this buffer is displayed in a window.
    pub display_time_: Lisp_Object,
    /// If scrolling the display because point is below the bottom of a
    /// window showing this buffer, try to choose a window start so
    /// that point ends up this number of lines from the top of the
    /// window.  Nil means that scrolling method isn't used.
    pub scroll_up_aggressively_: Lisp_Object,
    /// If scrolling the display because point is above the top of a
    /// window showing this buffer, try to choose a window start so
    /// that point ends up this number of lines from the bottom of the
    /// window.  Nil means that scrolling method isn't used.
    pub scroll_down_aggressively_: Lisp_Object,
    /// Desired cursor type in this buffer.  See the doc string of
    /// per-buffer variable `cursor-type'.
    pub cursor_type_: Lisp_Object,
    /// An integer > 0 means put that number of pixels below text lines
    /// in the display of this buffer.
    /// A float ~ 1.0 means add extra number of pixels below text lines
    /// relative to the line height.
    /// A cons means put car spacing above and cdr spacing below the line.
    pub extra_line_spacing_: Lisp_Object,
    /// A list of tree-sitter parsers for this buffer.
    #[cfg(feature = "tree_sitter")]
    pub ts_parser_list_: Lisp_Object,
    /// What type of text conversion the input method should apply to
    /// this buffer.
    pub text_conversion_style_: Lisp_Object,
    /// Cursor type to display in non-selected windows.
    /// t means to use hollow box cursor.
    /// See `cursor-type' for other values.
    pub cursor_in_non_selected_windows_: Lisp_Object,
    /// No more Lisp_Object beyond cursor_in_non_selected_windows_.
    /// Except undo_list, which is handled specially in Fgarbage_collect.
    /// This structure holds the coordinates of the buffer contents
    /// in ordinary buffers.  In indirect buffers, this is not used.
    pub own_text: buffer_text,
    /// This points to the `struct buffer_text' that used for this buffer.
    /// In an ordinary buffer, this is the own_text field above.
    /// In an indirect buffer, this is the own_text field of another buffer.
    pub text: *mut buffer_text,
    /// Char position of point in buffer.
    pub pt: isize,
    /// Byte position of point in buffer.
    pub pt_byte: isize,
    /// Char position of beginning of accessible range.
    pub begv: isize,
    /// Byte position of beginning of accessible range.
    pub begv_byte: isize,
    /// Char position of end of accessible range.
    pub zv: isize,
    /// Byte position of end of accessible range.
    pub zv_byte: isize,
    /// In an indirect buffer, this points to the base buffer.
    /// In an ordinary buffer, it is 0.
    pub base_buffer: *mut buffer,
    /// In an indirect buffer, this is -1.  In an ordinary buffer,
    /// it's the number of indirect buffers that share our text;
    /// zero means that we're the only owner of this text.
    pub indirections: c_int,
    /// Number of windows showing this buffer.  Always -1 for
    /// an indirect buffer since it counts as its base buffer.
    pub window_count: c_int,
    /// A non-zero value in slot IDX means that per-buffer variable
    /// with index IDX has a local value in this buffer.  The index IDX
    /// for a buffer-local variable is stored in that variable's slot
    /// in buffer_local_flags as a Lisp integer.  If the index is -1,
    /// this means the variable is always local in all buffers.
    pub local_flags: [c_char; MAX_PER_BUFFER_VARS as usize],
    /// Set to the modtime of the visited file when read or written.
    /// modtime.tv_nsec == NONEXISTENT_MODTIME_NSECS means
    /// visited file was nonexistent.  modtime.tv_nsec ==
    /// UNKNOWN_MODTIME_NSECS means visited file modtime unknown;
    /// in no case complain about any mismatch on next save attempt.
    pub modtime: timespec,
    /// Size of the file when modtime was set.  This is used to detect when
    /// the file size changed while we were reading the file, but the
    /// modtime is still the same (since it's truncated to clock resolution)
    /// so we're actually not up-to-date.  Meaningful only if it is
    /// nonnegative and the modtime is actually set.  Useful only on
    /// platforms with coarse clock resolution.
    pub modtime_size: off_t,
    /// The value of text->modiff at the last auto-save.
    pub auto_save_modified: modiff_count,
    /// The value of text->modiff at the last display error.
    /// Redisplay of this buffer is inhibited until it changes again.
    pub display_error_modiff: modiff_count,
    /// The time at which we detected a failure to auto-save,
    /// Or 0 if we didn't have a failure.
    pub auto_save_failure_time: time_t,
    /// Position in buffer at which display started
    /// the last time this buffer was displayed.
    pub last_window_start: isize,
    /// If the long line scan cache is enabled (i.e. the buffer-local
    /// variable cache-long-scans is non-nil), newline_cache points to
    /// the newline cache, and width_run_cache points to the width run
    /// cache.
    /// The newline cache records which stretches of the buffer are
    /// known *not* to contain newlines, so that they can be skipped
    /// quickly when we search for newlines.
    /// The width run cache records which stretches of the buffer are
    /// known to contain characters whose widths are all the same.  If
    /// the width run cache maps a character to a value > 0, that value is
    /// the character's width; if it maps a character to zero, we don't
    /// know what its width is.  This allows compute_motion to process
    /// such regions very quickly, using algebra instead of inspecting
    /// each character.   See also width_table, below.
    /// The latter cache is used to speedup bidi_find_paragraph_start.
    pub newline_cache: *mut c_void,        // struct region_cache *
    pub width_run_cache: *mut c_void,        // struct region_cache *
    pub bidi_paragraph_cache: *mut c_void,        // struct region_cache *
    /// Non-zero means disable redisplay optimizations when rebuilding the glyph
    /// matrices (but not when redrawing).
    /// bool_bf prevent_redisplay_optimizations_p : 1;
    /// Non-zero whenever the narrowing is changed in this buffer.
    /// bool_bf clip_changed : 1;
    /// Non-zero for internal or temporary buffers that don't need to
    /// run hooks kill-buffer-hook, kill-buffer-query-functions, and
    /// buffer-list-update-hook.  This is used in coding.c to avoid
    /// slowing down en/decoding when a lot of these hooks are
    /// defined, as well as by with-temp-buffer, for example.
    /// bool_bf inhibit_buffer_hooks : 1;
    /// Non-zero when the buffer contains long lines and specific
    /// display optimizations must be used.
    /// bool_bf long_line_optimizations_p : 1;
    /// clang's record layout puts the four bits at 960:0-960:3 of this byte.
    pub bits: u8,

    /// The interval tree containing this buffer's overlays.
    pub overlays: *mut itree_tree,
    /// Right now only tree-sitter makes use of this, so I don't want
    /// non-tree-sitter build to pay for it.  If something else can make
    /// use of this, we can remove the gate.
    /// Cache of line and column number of a position.  Tree-sitter uses
    /// this cache to calculate line and column of the beginning and end of
    /// buffer edits.  Stores three caches for BEGV, point, ZV,
    /// respectively.  All three are refreshed in buffer edit functions, so
    /// they're always up-to-date (in the sense that the bytepos and
    /// line/column number are in sync, not in the sense that the bytepos
    /// is at the actual position of point/BEGV/ZV, indeed, most of the
    /// time the bytepos is only near the actual position).  All caches are
    /// initialized to empty, meaning no linecol tracking for this
    /// buffer.
    #[cfg(feature = "tree_sitter")]
    pub ts_linecol_begv: ts_linecol,
    #[cfg(feature = "tree_sitter")]
    pub ts_linecol_point: ts_linecol,
    #[cfg(feature = "tree_sitter")]
    pub ts_linecol_zv: ts_linecol,
    /// Changes in the buffer are recorded here for undo, and t means
    /// don't record anything.  This information belongs to the base
    /// buffer of an indirect buffer.  But we can't store it in the
    /// struct buffer_text because local variables have to be right in
    /// the struct buffer. So we copy it around in set_buffer_internal.
    pub undo_list_: Lisp_Object,
}
/// `struct buffer`'s four `bool_bf` members, in buffer.h's declaration order (clang:
/// 960:0-960:3, the byte named `bits`).  C reads them through the bitfields; Rust reads
/// them by mask (PORTING.md 1.3), and `offsetof` cannot name a bitfield, so the numbers
/// are recorded here and checked by the C-side probe the ledger describes.
pub const BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P: u8 = 1 << 0;
pub const BUFFER_CLIP_CHANGED: u8 = 1 << 1;
pub const BUFFER_INHIBIT_BUFFER_HOOKS: u8 = 1 << 2;
pub const BUFFER_LONG_LINE_OPTIMIZATIONS_P: u8 = 1 << 3;

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct sortvec {
    pub overlay: Lisp_Object,
    pub beg: isize,
    pub end: isize,
    pub priority: EMACS_INT,
    /// Secondary priority.
    pub spriority: EMACS_INT,
}

#[inline(always)]
pub fn BUFFERP (a: Lisp_Object) -> bool
{
  return PSEUDOVECTORP (a, pvec_type::PVEC_BUFFER);
}

#[inline(always)]
pub fn CHECK_BUFFER (x: Lisp_Object)
{
  CHECK_TYPE (BUFFERP (x), Qbufferp (), x);
}

#[inline(always)]
pub fn XBUFFER (a: Lisp_Object) -> *mut buffer
{
  eassert! (BUFFERP (a));
  return XUNTAG! (a, Lisp_Type::Lisp_Vectorlike, buffer);
}

/* Most code should use these functions to set Lisp fields in struct
   buffer.  (Some setters that are private to a single .c file are
   defined as static in those files.)  */
#[inline(always)]
pub fn bset_bidi_paragraph_direction (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).bidi_paragraph_direction_ = val; }
}
#[inline(always)]
pub fn bset_cache_long_scans (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).cache_long_scans_ = val; }
}
#[inline(always)]
pub fn bset_case_canon_table (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).case_canon_table_ = val; }
}
#[inline(always)]
pub fn bset_case_eqv_table (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).case_eqv_table_ = val; }
}
#[inline(always)]
pub fn bset_directory (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).directory_ = val; }
}
#[inline(always)]
pub fn bset_display_count (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).display_count_ = val; }
}
#[inline(always)]
pub fn bset_left_margin_cols (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).left_margin_cols_ = val; }
}
#[inline(always)]
pub fn bset_right_margin_cols (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).right_margin_cols_ = val; }
}
#[inline(always)]
pub fn bset_display_time (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).display_time_ = val; }
}
#[inline(always)]
pub fn bset_downcase_table (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).downcase_table_ = val; }
}
#[inline(always)]
pub fn bset_enable_multibyte_characters (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).enable_multibyte_characters_ = val; }
}
#[inline(always)]
pub fn bset_filename (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).filename_ = val; }
}
#[inline(always)]
pub fn bset_keymap (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).keymap_ = val; }
}
#[inline(always)]
pub fn bset_last_selected_window (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).last_selected_window_ = val; }
}
#[inline(always)]
pub fn bset_local_var_alist (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).local_var_alist_ = val; }
}
#[inline(always)]
pub fn bset_mark_active (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).mark_active_ = val; }
}
#[inline(always)]
pub fn bset_point_before_scroll (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).point_before_scroll_ = val; }
}
#[inline(always)]
pub fn bset_read_only (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).read_only_ = val; }
}
#[inline(always)]
pub fn bset_truncate_lines (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).truncate_lines_ = val; }
}
#[inline(always)]
pub fn bset_undo_list (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).undo_list_ = val; }
}
#[inline(always)]
pub fn bset_upcase_table (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).upcase_table_ = val; }
}
#[inline(always)]
pub fn bset_width_table (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).width_table_ = val; }
}

#[inline(always)]
pub fn bset_text_conversion_style (b: *mut buffer, val: Lisp_Object)
{
  unsafe { (*b).text_conversion_style_ = val; }
}

/* BUFFER_CEILING_OF (resp. BUFFER_FLOOR_OF), when applied to n, return
   the max (resp. min) p such that

   BYTE_POS_ADDR (p) - BYTE_POS_ADDR (n) == p - n       */

#[inline(always)]
pub fn BUFFER_CEILING_OF (bytepos: ptrdiff_t) -> ptrdiff_t
{
  return (unsafe {
    if bytepos < GPT_BYTE! () && GPT! () < ZV! () { GPT_BYTE! () } else { ZV_BYTE! () }
  }) - 1;
}

#[inline(always)]
pub fn BUFFER_FLOOR_OF (bytepos: ptrdiff_t) -> ptrdiff_t
{
  return unsafe {
    if BEGV! () <= GPT! () && GPT_BYTE! () <= bytepos { GPT_BYTE! () } else { BEGV_BYTE! () }
  };
}

/* The BUF_BEGV[_BYTE], BUF_ZV[_BYTE], and BUF_PT[_BYTE] functions cannot
   be used for assignment; use SET_BUF_* functions below for that.  */

/* Position of beginning of accessible range of buffer.  */
#[inline(always)]
pub fn BUF_BEGV (buf: *mut buffer) -> ptrdiff_t
{
  return if unsafe { buf == current_buffer } { unsafe { BEGV! () } }
    else if NILP (unsafe { BVAR! (buf, begv_marker_) }) { unsafe { (*buf).begv } }
    else { unsafe { marker_position (BVAR! (buf, begv_marker_)) } };
}

#[inline(always)]
pub fn BUF_BEGV_BYTE (buf: *mut buffer) -> ptrdiff_t
{
  return if unsafe { buf == current_buffer } { unsafe { BEGV_BYTE! () } }
    else if NILP (unsafe { BVAR! (buf, begv_marker_) }) { unsafe { (*buf).begv_byte } }
    else { unsafe { marker_byte_position (BVAR! (buf, begv_marker_)) } };
}

/* Position of point in buffer.  */
#[inline(always)]
pub fn BUF_PT (buf: *mut buffer) -> ptrdiff_t
{
  return if unsafe { buf == current_buffer } { unsafe { PT! () } }
    else if NILP (unsafe { BVAR! (buf, pt_marker_) }) { unsafe { (*buf).pt } }
    else { unsafe { marker_position (BVAR! (buf, pt_marker_)) } };
}

#[inline(always)]
pub fn BUF_PT_BYTE (buf: *mut buffer) -> ptrdiff_t
{
  return if unsafe { buf == current_buffer } { unsafe { PT_BYTE! () } }
    else if NILP (unsafe { BVAR! (buf, pt_marker_) }) { unsafe { (*buf).pt_byte } }
    else { unsafe { marker_byte_position (BVAR! (buf, pt_marker_)) } };
}

/* Position of end of accessible range of buffer.  */
#[inline(always)]
pub fn BUF_ZV (buf: *mut buffer) -> ptrdiff_t
{
  return if unsafe { buf == current_buffer } { unsafe { ZV! () } }
    else if NILP (unsafe { BVAR! (buf, zv_marker_) }) { unsafe { (*buf).zv } }
    else { unsafe { marker_position (BVAR! (buf, zv_marker_)) } };
}

#[inline(always)]
pub fn BUF_ZV_BYTE (buf: *mut buffer) -> ptrdiff_t
{
  return if unsafe { buf == current_buffer } { unsafe { ZV_BYTE! () } }
    else if NILP (unsafe { BVAR! (buf, zv_marker_) }) { unsafe { (*buf).zv_byte } }
    else { unsafe { marker_byte_position (BVAR! (buf, zv_marker_)) } };
}

/* Similar functions to operate on a specified buffer.  */

/* Position of beginning of buffer.  */
#[inline(always)]
pub fn BUF_BEG (buf: *mut buffer) -> ptrdiff_t
{
  return BEG as ptrdiff_t;
}

#[inline(always)]
pub fn BUF_BEG_BYTE (buf: *mut buffer) -> ptrdiff_t
{
  return BEG_BYTE as ptrdiff_t;
}

/* Address of beginning of gap of buffer.  */
#[inline(always)]
pub fn BUF_GPT_ADDR (buf: *mut buffer) -> *mut u8
{
  return unsafe { (*(*buf).text).beg.offset ((*(*buf).text).gpt_byte - BEG_BYTE as isize) };
}

/* Address of end of buffer.  */
#[inline(always)]
pub fn BUF_Z_ADDR (buf: *mut buffer) -> *mut u8
{
  return unsafe {
    (*(*buf).text).beg.offset ((*(*buf).text).gap_size + (*(*buf).text).z_byte
                               - BEG_BYTE as isize)
  };
}

/* Address of end of gap in buffer.  */
#[inline(always)]
pub fn BUF_GAP_END_ADDR (buf: *mut buffer) -> *mut u8
{
  return unsafe {
    (*(*buf).text).beg.offset ((*(*buf).text).gpt_byte + (*(*buf).text).gap_size
                               - BEG_BYTE as isize)
  };
}

/* Compute how many characters at the top and bottom of BUF are
   unchanged when the range START..END is modified.  This computation
   must be done each time BUF is modified.  */

#[inline(always)]
pub fn BUF_COMPUTE_UNCHANGED (buf: *mut buffer, start: ptrdiff_t, end: ptrdiff_t)
{
  if unsafe { BUF_UNCHANGED_MODIFIED! (buf) == BUF_MODIFF! (buf)
              && BUF_OVERLAY_UNCHANGED_MODIFIED! (buf) == BUF_OVERLAY_MODIFF! (buf) }
    {
      unsafe {
        (*(*buf).text).beg_unchanged = start - BUF_BEG (buf);
        (*(*buf).text).end_unchanged = BUF_Z! (buf) - (end);
      }
    }
  else
    {
      if unsafe { BUF_Z! (buf) - end < BUF_END_UNCHANGED! (buf) }
        {
          unsafe { (*(*buf).text).end_unchanged = BUF_Z! (buf) - end; }
        }
      if unsafe { start - BUF_BEG (buf) < BUF_BEG_UNCHANGED! (buf) }
        {
          unsafe { (*(*buf).text).beg_unchanged = start - BUF_BEG (buf); }
        }
    }
}

/* Functions for setting the BEGV, ZV or PT of a given buffer.

   The ..._BOTH functions take both a charpos and a bytepos,
   which must correspond to each other.

   The functions without ..._BOTH take just a charpos,
   and compute the bytepos from it.  */

#[inline(always)]
pub fn SET_BUF_BEGV (buf: *mut buffer, charpos: ptrdiff_t)
{
  unsafe {
    (*buf).begv_byte = buf_charpos_to_bytepos (buf as *mut c_void, charpos);
    (*buf).begv = charpos;
  }
}

#[inline(always)]
pub fn SET_BUF_ZV (buf: *mut buffer, charpos: ptrdiff_t)
{
  unsafe {
    (*buf).zv_byte = buf_charpos_to_bytepos (buf as *mut c_void, charpos);
    (*buf).zv = charpos;
  }
}

#[inline(always)]
pub fn SET_BUF_BEGV_BOTH (buf: *mut buffer, charpos: ptrdiff_t, byte: ptrdiff_t)
{
  unsafe {
    (*buf).begv = charpos;
    (*buf).begv_byte = byte;
  }
}

#[inline(always)]
pub fn SET_BUF_ZV_BOTH (buf: *mut buffer, charpos: ptrdiff_t, byte: ptrdiff_t)
{
  unsafe {
    (*buf).zv = charpos;
    (*buf).zv_byte = byte;
  }
}

#[inline(always)]
pub fn SET_BUF_PT_BOTH (buf: *mut buffer, charpos: ptrdiff_t, byte: ptrdiff_t)
{
  unsafe {
    (*buf).pt = charpos;
    (*buf).pt_byte = byte;
  }
}

/* Functions to access a character or byte in the current buffer,
   or convert between a byte position and an address.
   These functions do not check that the position is in range.  */

/* See the important WARNING above about using the 'char *' pointers
   returned by these functions.  */

/* Return the address of byte position N in current buffer.  */

#[inline(always)]
pub fn BYTE_POS_ADDR (n: ptrdiff_t) -> *mut u8
{
  return unsafe {
    BEG_ADDR! ().offset ((if n < GPT_BYTE! () { 0 } else { GAP_SIZE! () }) + n - BEG_BYTE as isize)
  };
}

/* Return the address of char position N.  */

#[inline(always)]
pub fn CHAR_POS_ADDR (n: ptrdiff_t) -> *mut u8
{
  return unsafe {
    BEG_ADDR! ().offset ((if n < GPT! () { 0 } else { GAP_SIZE! () })
                        + buf_charpos_to_bytepos (current_buffer as *mut c_void, n) - BEG_BYTE as isize)
  };
}

/* Convert a character position to a byte position.  */

#[inline(always)]
pub fn CHAR_TO_BYTE (charpos: ptrdiff_t) -> ptrdiff_t
{
  return unsafe { buf_charpos_to_bytepos (current_buffer as *mut c_void, charpos) };
}

/* Convert a byte position to a character position.  */

#[inline(always)]
pub fn BYTE_TO_CHAR (bytepos: ptrdiff_t) -> ptrdiff_t
{
  return unsafe { buf_bytepos_to_charpos (current_buffer as *mut c_void, bytepos) };
}

/* Convert PTR, the address of a byte in the buffer, into a byte position.  */

#[inline(always)]
pub fn PTR_BYTE_POS (ptr: *const u8) -> ptrdiff_t
{
  let byte = unsafe { ptr.offset_from ((*(*current_buffer).text).beg) };
  return byte - unsafe {
    if byte <= GPT_BYTE! () - BEG_BYTE as isize { 0 } else { GAP_SIZE! () }
  } + BEG_BYTE as isize;
}

/* Number of Lisp_Objects at the beginning of struct buffer.
   If you add, remove, or reorder Lisp_Objects within buffer
   structure, make sure that this is still correct.  */

pub const BUFFER_LISP_SIZE: isize = PSEUDOVECSIZE! (buffer, cursor_in_non_selected_windows_);

/* Allocated size of the struct buffer part beyond leading
   Lisp_Objects, in word_size units.  */

pub const BUFFER_REST_SIZE: isize =
  VECSIZE (std::mem::size_of::<buffer> ()) - BUFFER_LISP_SIZE;

/* Initialize the pseudovector header of buffer object.  BUFFER_LISP_SIZE
   is required for GC, but BUFFER_REST_SIZE is set up just to be consistent
   with other pseudovectors.  */

#[inline(always)]
pub fn BUFFER_PVEC_INIT (b: *mut buffer)
{
  XSETPVECTYPESIZE! (b, pvec_type::PVEC_BUFFER, BUFFER_LISP_SIZE, BUFFER_REST_SIZE);
}

/* Convenient check whether buffer B is live.  */

#[inline(always)]
pub fn BUFFER_LIVE_P (b: *mut buffer) -> bool
{
  return !NILP (unsafe { BVAR! (b, name_) });
}

/* Convenient check whether buffer B is hidden (i.e. its name
   starts with a space).  Caller must ensure that B is live.  */

#[inline(always)]
pub fn BUFFER_HIDDEN_P (b: *mut buffer) -> bool
{
  return SREF (unsafe { BVAR! (b, name_) }, 0) == b' ';
}

/* Verify indirection counters.  */

#[inline(always)]
pub fn BUFFER_CHECK_INDIRECTION (b: *mut buffer)
{
  if BUFFER_LIVE_P (b)
    {
      if unsafe { !(*b).base_buffer.is_null () }
        {
          eassert! (unsafe { (*b).indirections == -1 });
          eassert! (unsafe { (*(*b).base_buffer).indirections > 0 });
        }
      else
        {
          eassert! (unsafe { (*b).indirections >= 0 });
        }
    }
}

// C gates these six on HAVE_TREE_SITTER (buffer.h 1155-1210), which the frozen
// configuration does not define; the three struct members are gated the same way.
// The `cfg` follows the port's convention for configuration-dependent code, and this
// assertion keeps it tied to the frozen configuration: enabling the feature without
// regenerating the configuration (which is what would add the members' layout keys)
// fails to compile instead of silently growing unasserted fields (P-08, P-34).
const _: () = assert! (!cfg! (feature = "tree_sitter") || HAVE_TREE_SITTER);
#[cfg(feature = "tree_sitter")]
#[inline(always)]
pub fn BUF_TS_LINECOL_BEGV (buf: *mut buffer) -> ts_linecol
{
  return unsafe { (*buf).ts_linecol_begv };
}
#[cfg(feature = "tree_sitter")]
#[inline(always)]
pub fn BUF_TS_LINECOL_POINT (buf: *mut buffer) -> ts_linecol
{
  return unsafe { (*buf).ts_linecol_point };
}

#[cfg(feature = "tree_sitter")]
#[inline(always)]
pub fn BUF_TS_LINECOL_ZV (buf: *mut buffer) -> ts_linecol
{
  return unsafe { (*buf).ts_linecol_zv };
}

#[cfg(feature = "tree_sitter")]
#[inline(always)]
pub fn SET_BUF_TS_LINECOL_BEGV (buf: *mut buffer, linecol: ts_linecol)
{
  unsafe { (*buf).ts_linecol_begv = linecol; }
}

#[cfg(feature = "tree_sitter")]
#[inline(always)]
pub fn SET_BUF_TS_LINECOL_POINT (buf: *mut buffer, linecol: ts_linecol)
{
  unsafe { (*buf).ts_linecol_point = linecol; }
}

#[cfg(feature = "tree_sitter")]
#[inline(always)]
pub fn SET_BUF_TS_LINECOL_ZV (buf: *mut buffer, linecol: ts_linecol)
{
  unsafe { (*buf).ts_linecol_zv = linecol; }
}

/* This structure holds the default values of the buffer-local variables
   that have special slots in each buffer.
   The default value occupies the same slot in this structure
   as an individual buffer's value occupies in that buffer.
   Setting the default value also goes through the alist of buffers
   and stores into each buffer that does not say it has a local value.  */

unsafe extern "C" { pub static mut buffer_defaults: buffer; }

/* This structure marks which slots in a buffer have corresponding
   default values in buffer_defaults.
   Each such slot has a nonzero value in this structure.
   The value has only one nonzero bit.

   When a buffer has its own local value for a slot,
   the entry for that slot (found in the same slot in this structure)
   is turned on in the buffer's local_flags array.

   If a slot in this structure is zero, then even though there may
   be a Lisp-level local variable for the slot, it has no default value,
   and the corresponding slot in buffer_defaults is not used.  */

unsafe extern "C" { pub static mut buffer_local_flags: buffer; }

/* For each buffer slot, this points to the Lisp symbol name
   for that slot in the current buffer.  It is 0 for slots
   that don't have such names.  */

unsafe extern "C" { pub static mut buffer_local_symbols: buffer; }

/* verify_interval_modification saves insertion hooks here
   to be run later by report_interval_modification.  */
unsafe extern "C" { pub static mut interval_insert_behind_hooks: Lisp_Object; }
unsafe extern "C" { pub static mut interval_insert_in_front_hooks: Lisp_Object; }

unsafe extern "C" {
  pub fn fix_position (pos: Lisp_Object) -> EMACS_INT;
}

// C is an assignment expression, `((x) = make_fixnum (fix_position (x)))`; a Rust
// macro cannot be an l-value, so this is a statement macro (C never uses the value).
#[macro_export]
macro_rules! CHECK_FIXNUM_COERCE_MARKER {
    ($x:ident) => { $x = make_fixnum (unsafe { fix_position ($x) }); };
}

unsafe extern "C" {
  // Parameter names come from the definitions in buffer.c (C's prototypes have
  // none): 992, 1089, 1857, 3161, 3170, 3334, 3469, 2492, 2327, 2343, 2399,
  // 1361, 2206, 2439, 2447, and alloc.c 3768.
  pub fn delete_all_overlays (b: *mut buffer);
  pub fn reset_buffer (b: *mut buffer);
  pub fn compact_buffer (buffer: *mut buffer);
  pub fn overlays_at (pos: ptrdiff_t, extend: bool, vec_ptr: *mut *mut Lisp_Object,
                      len_ptr: *mut ptrdiff_t, next_ptr: *mut ptrdiff_t) -> ptrdiff_t;
  pub fn next_overlay_change (pos: ptrdiff_t) -> ptrdiff_t;
  // `struct window` is incomplete in this header, so the port keeps C's pointee
  // opaque and spells the type in the comment (PORTING.md 0.1-2).
  pub fn sort_overlays (overlay_vec: *mut Lisp_Object, noverlays: ptrdiff_t,
                        w: *mut c_void) -> ptrdiff_t; // struct window *
  pub fn recenter_overlay_lists (buf: *mut buffer, pos: ptrdiff_t);
  pub fn overlay_strings (pos: ptrdiff_t, w: *mut c_void, // struct window *
                          pstr: *mut *mut u8) -> ptrdiff_t;
  pub fn validate_region (b: *mut Lisp_Object, e: *mut Lisp_Object);
  pub fn set_buffer_internal_1 (b: *mut buffer);
  pub fn set_buffer_internal_2 (b: *mut buffer);
  pub fn set_buffer_temp (b: *mut buffer);
  pub fn buffer_local_value (variable: Lisp_Object, buffer: Lisp_Object) -> Lisp_Object;
  pub fn record_buffer (buffer: Lisp_Object);
  pub fn fix_overlays_before (bp: *mut buffer, prev: ptrdiff_t, pos: ptrdiff_t);
  pub fn mmap_set_vars (on: bool);
  pub fn restore_buffer (buffer_or_name: Lisp_Object);
  pub fn set_buffer_if_live (buffer: Lisp_Object);
  pub fn build_overlay (front_advance: bool, rear_advance: bool,
                        plist: Lisp_Object) -> Lisp_Object;
}

/* Return B as a struct buffer pointer, defaulting to the current buffer.  */

#[inline(always)]
pub fn decode_buffer (b: Lisp_Object) -> *mut buffer
{
  return if NILP (b) { unsafe { current_buffer } }
    else { CHECK_BUFFER (b); XBUFFER (b) };
}

/* Set the current buffer to B.

   We previously set windows_or_buffers_changed here to invalidate
   global unchanged information in beg_unchanged and end_unchanged.
   This is no longer necessary because we now compute unchanged
   information on a buffer-basis.  Every action affecting other
   windows than the selected one requires a select_window at some
   time, and that increments windows_or_buffers_changed.  */

#[inline(always)]
pub fn set_buffer_internal (b: *mut buffer)
{
  if unsafe { current_buffer } != b
    {
      unsafe { set_buffer_internal_1 (b); }
    }
}

/* Arrange to go back to the original buffer after the next
   call to unbind_to if the original buffer is still alive.  */

#[inline(always)]
pub fn record_unwind_current_buffer ()
{
  unsafe { record_unwind_protect (Some (set_buffer_if_live), Fcurrent_buffer ()); }
}

/* Get overlays at POSN into array OVERLAYS with NOVERLAYS elements.
   If NEXTP is non-NULL, return next overlay there.
   See overlay_at arg CHANGE_REQ for meaning of CHRQ arg.
   This macro might evaluate its args multiple times,
   and it treat some args as lvalues.  */

// C's `SAFE_NALLOCA` reads the enclosing function's `sa_avail`; Rust macro hygiene
// requires that name as an argument (the port's SAFE_NALLOCA already takes it), so
// the call site passes its `sa_avail` as the first argument.  PORTING.md P-37.
#[macro_export]
macro_rules! GET_OVERLAYS_AT {
  ($sa_avail:ident, $posn:expr, $overlays:ident, $noverlays:ident, $next:expr) => {{
    let mut maxlen: ptrdiff_t = 40;
    SAFE_NALLOCA! ($sa_avail, $overlays, 1, maxlen);
    $noverlays = unsafe {
      overlays_at ($posn, false, &raw mut $overlays, &raw mut maxlen, $next)
    };
    if $noverlays > maxlen
      {
        maxlen = $noverlays;
        SAFE_NALLOCA! ($sa_avail, $overlays, 1, maxlen);
        $noverlays = unsafe {
          overlays_at ($posn, false, &raw mut $overlays, &raw mut maxlen, $next)
        };
      }
  }};
}

unsafe extern "C" { pub static mut Vbuffer_alist: Lisp_Object; }

/* FOR_EACH_LIVE_BUFFER (LIST_VAR, BUF_VAR) followed by a statement is
   a `for' loop which iterates over the buffers from Vbuffer_alist.  */
// C: `#define FOR_EACH_LIVE_BUFFER(list_var, buf_var) FOR_EACH_ALIST_VALUE
// (Vbuffer_alist, list_var, buf_var)`.  Both are loop macros whose Rust form is
// the open P-22 decision, so the mirror carries no counterpart yet; recorded in
// docs/ports/excluded-macros.txt.

/* Get text properties of B.  */

#[inline(always)]
pub fn buffer_intervals (b: *mut buffer) -> INTERVAL
{
  eassert! (unsafe { !(*b).text.is_null () });
  return unsafe { (*(*b).text).intervals };
}

/* Set text properties of B to I.  */

#[inline(always)]
pub fn set_buffer_intervals (b: *mut buffer, i: INTERVAL)
{
  eassert! (unsafe { !(*b).text.is_null () });
  unsafe { (*(*b).text).intervals = i; }
}

/* Non-zero if current buffer has overlays.  */

#[inline(always)]
pub fn buffer_has_overlays () -> bool
{
  return !itree_empty_p (unsafe { (*current_buffer).overlays });
}

/* Functions for accessing a character or byte,
   or converting between byte positions and addresses,
   in a specified buffer.  */

/* Return character code of multi-byte form at byte position POS.  If POS
   doesn't point the head of valid multi-byte form, only the byte at
   POS is returned.  No range checking.  */

#[inline(always)]
pub fn FETCH_MULTIBYTE_CHAR (pos: ptrdiff_t) -> c_int
{
  let p = BYTE_POS_ADDR (pos);
  return STRING_CHAR (p as *const u8);
}

/* Return character code of multi-byte form at byte position POS in BUF.
   If POS doesn't point the head of valid multi-byte form, only the byte at
   POS is returned.  No range checking.  */

#[inline(always)]
pub fn BUF_FETCH_MULTIBYTE_CHAR (buf: *mut buffer, pos: ptrdiff_t) -> c_int
{
  let p = unsafe {
    BUF_BEG_ADDR! (buf).offset ((if pos >= BUF_GPT_BYTE! (buf) { BUF_GAP_SIZE! (buf) } else { 0 })
                               + pos - BEG_BYTE as isize)
  };
  return STRING_CHAR (p as *const u8);
}

/* Return character at byte position POS.
   If the current buffer is unibyte and the character is not ASCII,
   make the returning character multibyte.  */

#[inline(always)]
pub fn FETCH_CHAR_AS_MULTIBYTE (pos: ptrdiff_t) -> c_int
{
  return if !NILP (unsafe { BVAR! (current_buffer, enable_multibyte_characters_) })
    { FETCH_MULTIBYTE_CHAR (pos) }
    else { UNIBYTE_TO_CHAR (unsafe { FETCH_BYTE! (pos) } as c_int) };
}

/* Return character at byte position POS.
   See the caveat WARNING for FETCH_MULTIBYTE_CHAR above.  */

#[inline(always)]
pub fn FETCH_CHAR (pos: ptrdiff_t) -> c_int
{
  return if !NILP (unsafe { BVAR! (current_buffer, enable_multibyte_characters_) })
    { FETCH_MULTIBYTE_CHAR (pos) }
    else { (unsafe { FETCH_BYTE! (pos) }) as c_int };
}

/* Return the address of character at byte position POS in buffer BUF.
   Note that both arguments can be computed more than once.  */

#[inline(always)]
pub fn BUF_BYTE_ADDRESS (buf: *mut buffer, pos: ptrdiff_t) -> *mut u8
{
  return unsafe {
    (*(*buf).text).beg.offset (pos - BEG_BYTE as isize
                               + (if pos < (*(*buf).text).gpt_byte { 0 }
                                  else { (*(*buf).text).gap_size }))
  };
}

/* Return the address of character at char position POS in buffer BUF.
   Note that both arguments can be computed more than once.  */

#[inline(always)]
pub fn BUF_CHAR_ADDRESS (buf: *mut buffer, pos: ptrdiff_t) -> *mut u8
{
  return unsafe {
    (*(*buf).text).beg.offset (buf_charpos_to_bytepos (buf as *mut c_void, pos) - BEG_BYTE as isize
                               + (if pos < (*(*buf).text).gpt { 0 }
                                  else { (*(*buf).text).gap_size }))
  };
}

/* Convert PTR, the address of a char in buffer BUF, into a byte
   position.  */

#[inline(always)]
pub fn BUF_PTR_BYTE_POS (buf: *mut buffer, ptr: *mut u8) -> ptrdiff_t
{
  let byte = unsafe { ptr.offset_from ((*(*buf).text).beg) };
  return byte - unsafe {
    if byte <= BUF_GPT_BYTE! (buf) - BEG_BYTE as isize { 0 } else { BUF_GAP_SIZE! (buf) }
  } + BEG_BYTE as isize;
}

/* Return the byte at byte position N in buffer BUF.   */

#[inline(always)]
pub fn BUF_FETCH_BYTE (buf: *mut buffer, n: ptrdiff_t) -> u8
{
  return unsafe { *BUF_BYTE_ADDRESS (buf, n) };
}

/* Return character at byte position POS in buffer BUF.  If BUF is
   unibyte and the character is not ASCII, make the returning
   character multibyte.  */

#[inline(always)]
pub fn BUF_FETCH_CHAR_AS_MULTIBYTE (buf: *mut buffer, pos: ptrdiff_t) -> c_int
{
  return if !NILP (unsafe { BVAR! (buf, enable_multibyte_characters_) })
    { BUF_FETCH_MULTIBYTE_CHAR (buf, pos) }
    else { UNIBYTE_TO_CHAR (BUF_FETCH_BYTE (buf, pos) as c_int) };
}

/* Return number of windows showing B or a buffer that has B as its base
   buffer.  If B is an indirect buffer, this returns buffer_window_count
   of its base buffer.  */

#[inline(always)]
pub fn buffer_window_count (b: *mut buffer) -> c_int
{
  let mut b = b;
  if unsafe { !(*b).base_buffer.is_null () }
    {
      b = unsafe { (*b).base_buffer };
    }
  eassert! (unsafe { (*b).window_count >= 0 });
  return unsafe { (*b).window_count };
}

/* Overlays */

#[inline(always)]
pub fn overlay_start (ov: *mut Lisp_Overlay) -> ptrdiff_t
{
  if unsafe { (*ov).buffer.is_null () }
    {
      return -1;
    }
  return unsafe {
    let buf = (*ov).buffer as *mut buffer;
    let node = (*ov).interval as *mut itree_node;
    itree_node_begin ((*buf).overlays, node)
  };
}

#[inline(always)]
pub fn overlay_end (ov: *mut Lisp_Overlay) -> ptrdiff_t
{
  if unsafe { (*ov).buffer.is_null () }
    {
      return -1;
    }
  return unsafe {
    let buf = (*ov).buffer as *mut buffer;
    let node = (*ov).interval as *mut itree_node;
    itree_node_end ((*buf).overlays, node)
  };
}

/* Return the start of OV in its buffer, or -1 if OV is not associated
   with any buffer.  */

#[inline(always)]
pub fn OVERLAY_START (ov: Lisp_Object) -> ptrdiff_t
{
  return overlay_start (XOVERLAY (ov));
}

/* Return the end of OV in its buffer, or -1. */

#[inline(always)]
pub fn OVERLAY_END (ov: Lisp_Object) -> ptrdiff_t
{
  return overlay_end (XOVERLAY (ov));
}

/* Return the plist of overlay OV.  */

#[inline(always)]
pub fn OVERLAY_PLIST (ov: Lisp_Object) -> Lisp_Object
{
  return unsafe { (*XOVERLAY (ov)).plist };
}

/* Return the buffer of overlay OV. */

#[inline(always)]
pub fn OVERLAY_BUFFER (ov: Lisp_Object) -> *mut buffer
{
  // Lisp_Overlay.buffer is declared `*mut c_void` because `struct buffer` is
  // incomplete in lisp.h (PORTING.md 0.1-2); C's return type is `struct buffer *`.
  return unsafe { (*XOVERLAY (ov)).buffer as *mut buffer };
}

/* Return true, if OV's rear-advance is set. */

#[inline(always)]
pub fn OVERLAY_REAR_ADVANCE_P (ov: Lisp_Object) -> bool
{
  // itree.h's `rear_advance` is bit 1 of the node's packed bool_bf byte (itree.rs).
  return unsafe {
    let node = (*XOVERLAY (ov)).interval as *mut itree_node;
    ((*node).bits & ITREE_REAR_ADVANCE) != 0
  };
}

/* Return true, if OV's front-advance is set. */

#[inline(always)]
pub fn OVERLAY_FRONT_ADVANCE_P (ov: Lisp_Object) -> bool
{
  return unsafe {
    let node = (*XOVERLAY (ov)).interval as *mut itree_node;
    ((*node).bits & ITREE_FRONT_ADVANCE) != 0
  };
}

/***********************************************************************
			Buffer-local Variables
 ***********************************************************************/

/* Return the offset in bytes of member VAR of struct buffer
   from the start of a buffer structure.  */

// C: `#define PER_BUFFER_VAR_OFFSET(VAR) offsetof (struct buffer, VAR ## _)`; the
// call site spells the member with the `_` C's macro appends (PORTING.md P-37).
#[macro_export]
macro_rules! PER_BUFFER_VAR_OFFSET {
    ($var:ident) => { std::mem::offset_of! (buffer, $var) as ptrdiff_t };
}

/* Used to iterate over normal Lisp_Object fields of struct buffer (all
   Lisp_Objects except undo_list).  If you add, remove, or reorder
   Lisp_Objects in a struct buffer, make sure that this is still correct.  */
// C: `#define FOR_EACH_PER_BUFFER_OBJECT_AT(offset) for (offset =
// PER_BUFFER_VAR_OFFSET (name); offset <= PER_BUFFER_VAR_OFFSET
// (cursor_in_non_selected_windows); offset += word_size)`.  C's body follows the macro
// at the call site; Rust macro hygiene cannot leave a block to be appended, so the body
// is the last argument (the shape `DEFUN!` already uses) and the `for` becomes a
// `while`.  The iteration variable stays a macro argument, as in C (PORTING.md P-37).
#[macro_export]
macro_rules! FOR_EACH_PER_BUFFER_OBJECT_AT {
    ($offset:ident, $body:block) => {
        $offset = PER_BUFFER_VAR_OFFSET! (name_);
        while $offset <= PER_BUFFER_VAR_OFFSET! (cursor_in_non_selected_windows_) {
            $body
            $offset += word_size;
        }
    };
}

/* Return the index of buffer-local variable VAR.  Each per-buffer
   variable has an index > 0 associated with it, except when it always
   has buffer-local values, in which case the index is -1.  If this is
   0, this is a bug and means that the slot of VAR in
   buffer_local_flags wasn't initialized.  */

#[macro_export]
macro_rules! PER_BUFFER_VAR_IDX {
    ($var:ident) => { PER_BUFFER_IDX (PER_BUFFER_VAR_OFFSET! ($var)) };
}

unsafe extern "C" {
  pub fn valid_per_buffer_idx (idx: c_int) -> bool;
}

/* Value is true if the variable with index IDX has a local value
   in buffer B.  */

#[inline(always)]
pub fn PER_BUFFER_VALUE_P (b: *mut buffer, idx: c_int) -> bool
{
  eassert! (unsafe { valid_per_buffer_idx (idx) });
  return unsafe { (*b).local_flags[idx as usize] != 0 };
}

/* Set whether per-buffer variable with index IDX has a buffer-local
   value in buffer B.  VAL zero means it hasn't.  */

#[inline(always)]
pub fn SET_PER_BUFFER_VALUE_P (b: *mut buffer, idx: c_int, val: bool)
{
  eassert! (unsafe { valid_per_buffer_idx (idx) });
  unsafe { (*b).local_flags[idx as usize] = val as c_char; }
}

/* Return the index value of the per-buffer variable at offset OFFSET
   in the buffer structure.

   If the slot OFFSET has a corresponding default value in
   buffer_defaults, the index value is positive and has only one
   nonzero bit.  When a buffer has its own local value for a slot, the
   bit for that slot (found in the same slot in this structure) is
   turned on in the buffer's local_flags array.

   If the index value is -1, even though there may be a
   DEFVAR_PER_BUFFER for the slot, there is no default value for it;
   and the corresponding slot in buffer_defaults is not used.

   If the index value is -2, then there is no DEFVAR_PER_BUFFER for
   the slot, but there is a default value which is copied into each
   new buffer.

   If a slot in this structure corresponding to a DEFVAR_PER_BUFFER is
   zero, that is a bug.  */

#[inline(always)]
pub fn PER_BUFFER_IDX (offset: ptrdiff_t) -> c_int
{
  return unsafe {
    XFIXNUM (*((&raw const buffer_local_flags as *const u8).offset (offset)
               as *const Lisp_Object)) as c_int
  };
}

/* Functions to get and set default value of the per-buffer
   variable at offset OFFSET in the buffer structure.  */

#[inline(always)]
pub fn per_buffer_default (offset: c_int) -> Lisp_Object
{
  return unsafe {
    *((&raw const buffer_defaults as *const u8).offset (offset as isize) as *const Lisp_Object)
  };
}

#[inline(always)]
pub fn set_per_buffer_default (offset: c_int, value: Lisp_Object)
{
  unsafe {
    *((&raw const buffer_defaults as *const u8).offset (offset as isize)
      as *mut Lisp_Object) = value;
  }
}

/* Functions to get and set buffer-local value of the per-buffer
   variable at offset OFFSET in the buffer structure.  */

#[inline(always)]
pub fn per_buffer_value (b: *mut buffer, offset: c_int) -> Lisp_Object
{
  return unsafe {
    *((b as *const u8).offset (offset as isize) as *const Lisp_Object)
  };
}

#[inline(always)]
pub fn set_per_buffer_value (b: *mut buffer, offset: c_int, value: Lisp_Object)
{
  unsafe { *((b as *mut u8).offset (offset as isize) as *mut Lisp_Object) = value; }
}

/* Downcase a character C, or make no change if that cannot be done.  */
#[inline(always)]
pub fn downcase (c: c_int) -> c_int
{
  let downcase_table = unsafe { BVAR! (current_buffer, downcase_table_) };
  let down = CHAR_TABLE_REF (downcase_table, c as isize);
  return if FIXNATP (down) { XFIXNAT (down) as c_int } else { c };
}

/* Upcase a character C, or make no change if that cannot be done. */
#[inline(always)]
pub fn upcase (c: c_int) -> c_int
{
  let upcase_table = unsafe { BVAR! (current_buffer, upcase_table_) };
  let up = CHAR_TABLE_REF (upcase_table, c as isize);
  return if FIXNATP (up) { XFIXNAT (up) as c_int } else { c };
}

/* True if C is upper case.  */
#[inline(always)]
pub fn uppercasep (c: c_int) -> bool
{
  return downcase (c) != c;
}

/* True if C is lower case.  */
#[inline(always)]
pub fn lowercasep (c: c_int) -> bool
{
  return !uppercasep (c) && upcase (c) != c;
}

/* Return a non-outlandish value for the tab width.  */

#[inline(always)]
pub fn sanitize_tab_width (width: Lisp_Object) -> c_int
{
  return if FIXNUMP (width) && 0 < XFIXNUM (width) && XFIXNUM (width) <= 1000
    { XFIXNUM (width) as c_int } else { 8 };
}

#[inline(always)]
pub fn SANE_TAB_WIDTH (buf: *mut buffer) -> c_int
{
  return sanitize_tab_width (unsafe { BVAR! (buf, tab_width_) });
}

/* Return a non-outlandish value for a character width.  */

#[inline(always)]
pub fn sanitize_char_width (width: EMACS_INT) -> c_int
{
  return if 0 <= width && width <= 1000 { width as c_int } else { 1000 };
}

/* Return the width of character C.  The width is measured by how many
   columns C will occupy on the screen when displayed in the current
   buffer.  The name CHARACTER_WIDTH avoids a collision with <limits.h>
   CHAR_WIDTH.  */

#[inline(always)]
pub fn CHARACTER_WIDTH (c: c_int) -> c_int
{
  return if 0x20 <= c && c < 0x7f { 1 }
    else if 0x7f < c
      {
        sanitize_char_width (XFIXNUM (CHAR_TABLE_REF (unsafe { Vchar_width_table },
                                                       c as isize)))
      }
    else if c == '\t' as c_int { SANE_TAB_WIDTH (unsafe { current_buffer }) }
    else if c == '\n' as c_int { 0 }
    else if !NILP (unsafe { BVAR! (current_buffer, ctl_arrow_) }) { 2 }
    else { 4 };
}

/* Like fetch_string_char_advance, but fetch character from the current
   buffer.  */

#[inline(always)]
pub fn fetch_char_advance (charidx: *mut ptrdiff_t, byteidx: *mut ptrdiff_t) -> c_int
{
  let output: c_int;
  let mut c = unsafe { *charidx };
  let mut b = unsafe { *byteidx };
  c += 1;
  let chp = BYTE_POS_ADDR (b);
  if !NILP (unsafe { BVAR! (current_buffer, enable_multibyte_characters_) })
    {
      let mut chlen: c_int = 0;
      output = string_char_and_length (chp as *const u8, &raw mut chlen);
      b += chlen as isize;
    }
  else
    {
      output = unsafe { *chp } as c_int;
      b += 1;
    }
  unsafe {
    *charidx = c;
    *byteidx = b;
  }
  return output;
}

/* Like fetch_char_advance, but assumes the current buffer is multibyte.  */

#[inline(always)]
pub fn fetch_char_advance_no_check (charidx: *mut ptrdiff_t, byteidx: *mut ptrdiff_t) -> c_int
{
  let output: c_int;
  let mut c = unsafe { *charidx };
  let mut b = unsafe { *byteidx };
  c += 1;
  let chp = BYTE_POS_ADDR (b);
  let mut chlen: c_int = 0;
  output = string_char_and_length (chp as *const u8, &raw mut chlen);
  b += chlen as isize;
  unsafe {
    *charidx = c;
    *byteidx = b;
  }
  return output;
}

/* Return the number of bytes in the multibyte character in BUF
   that starts at position POS_BYTE.  This relies on the fact that
   *GPT_ADDR and *Z_ADDR are always accessible and the values are
   '\0'.  No range checking of POS_BYTE.  */

#[inline(always)]
pub fn buf_next_char_len (buf: *mut buffer, pos_byte: ptrdiff_t) -> c_int
{
  let chp = BUF_BYTE_ADDRESS (buf, pos_byte);
  return BYTES_BY_CHAR_HEAD (unsafe { *chp } as c_int);
}

#[inline(always)]
pub fn next_char_len (pos_byte: ptrdiff_t) -> c_int
{
  return buf_next_char_len (unsafe { current_buffer }, pos_byte);
}

/* Return the number of bytes in the multibyte character in BUF just
   before POS_BYTE.  No range checking of POS_BYTE.  */

#[inline(always)]
pub fn buf_prev_char_len (buf: *mut buffer, pos_byte: ptrdiff_t) -> c_int
{
  let chp = unsafe {
    BUF_BEG_ADDR! (buf).offset (pos_byte - BEG_BYTE as isize
                               + (if pos_byte <= BUF_GPT_BYTE! (buf) { 0 }
                                  else { BUF_GAP_SIZE! (buf) }))
  };
  return raw_prev_char_len (chp as *const u8);
}

#[inline(always)]
pub fn prev_char_len (pos_byte: ptrdiff_t) -> c_int
{
  return buf_prev_char_len (unsafe { current_buffer }, pos_byte);
}

/* Increment both *CHARPOS and *BYTEPOS, each in the appropriate way.  */

#[inline(always)]
pub fn inc_both (charpos: *mut ptrdiff_t, bytepos: *mut ptrdiff_t)
{
  unsafe {
    (*charpos) += 1;
    (*bytepos) += (if !NILP (BVAR! (current_buffer, enable_multibyte_characters_))
      { next_char_len (*bytepos) } else { 1 }) as ptrdiff_t;
  }
}

/* Decrement both *CHARPOS and *BYTEPOS, each in the appropriate way.  */

#[inline(always)]
pub fn dec_both (charpos: *mut ptrdiff_t, bytepos: *mut ptrdiff_t)
{
  unsafe {
    (*charpos) -= 1;
    (*bytepos) -= (if !NILP (BVAR! (current_buffer, enable_multibyte_characters_))
      { prev_char_len (*bytepos) } else { 1 }) as ptrdiff_t;
  }
}

// INLINE_HEADER_END has no Rust counterpart (PORTING.md 0.1).

unsafe extern "C" {
  pub fn compare_overlays (v1: *const c_void, v2: *const c_void) -> c_int;
  pub fn make_sortvec_item (item: *mut sortvec, overlay: Lisp_Object);
}
