// femacs: port of src/buffer.h @ fd7afced403 (GNU Emacs 32.0.50).
//
// Header mirror: types, constants, macros, inline functions and the `unsafe
// extern "C"` declarations C spells as `extern` (PORTING.md 0.1).  The macros and
// inline accessors are the same statements in the same order as C's.
//
// WIP: this revision carries the types, the constants and the globals; the 53 code
// macros and the inline accessors are still to be written (docs/ports/include-buffer.md).

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

// The include guard has no Rust counterpart (PORTING.md 0.1).
pub const EMACS_BUFFER_H: bool = true;

/* Accessing the parameters of the current buffer.  */

/* These constants and macros come in pairs, one for the char position
   and one for the byte position.  */

/* Position of beginning of buffer.  */
pub const BEG: c_int = 1;
pub const BEG_BYTE: c_int = BEG;

/* Maximum gap size after compact_buffer, in bytes.  Also
   used in make_gap_larger to get some extra reserved space.  */
pub const GAP_BYTES_DFL: c_int = 2000;

/* Minimum gap size after compact_buffer, in bytes.  Also
   used in make_gap_smaller to avoid too small gap size.  */
pub const GAP_BYTES_MIN: c_int = 20;

/* Max number of builtin per-buffer variables.  */
pub const MAX_PER_BUFFER_VARS: c_int = 50;

/* Special values for struct buffer.modtime.  */
pub const NONEXISTENT_MODTIME_NSECS: c_int = -1;
pub const UNKNOWN_MODTIME_NSECS: c_int = -2;

/* Define the actual buffer data structures.  */

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
    /// bool_bf inhibit_shrinking : 1;
    /// bool_bf redisplay : 1;
    pub bits: u8,
}

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
    /// bool_bf prevent_redisplay_optimizations_p : 1;
    /// bool_bf clip_changed : 1;
    /// bool_bf inhibit_buffer_hooks : 1;
    /// bool_bf long_line_optimizations_p : 1;
    pub bits: u8,
    /// Non-zero means disable redisplay optimizations when rebuilding the glyph
    /// matrices (but not when redrawing).
    /// Non-zero whenever the narrowing is changed in this buffer.
    /// Non-zero for internal or temporary buffers that don't need to
    /// run hooks kill-buffer-hook, kill-buffer-query-functions, and
    /// buffer-list-update-hook.  This is used in coding.c to avoid
    /// slowing down en/decoding when a lot of these hooks are
    /// defined, as well as by with-temp-buffer, for example.
    /// Non-zero when the buffer contains long lines and specific
    /// display optimizations must be used.
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

unsafe extern "C" { pub static mut buffer_defaults: buffer; }
unsafe extern "C" { pub static mut buffer_local_flags: buffer; }
unsafe extern "C" { pub static mut buffer_local_symbols: buffer; }
unsafe extern "C" { pub static mut interval_insert_behind_hooks: Lisp_Object; }
unsafe extern "C" { pub static mut interval_insert_in_front_hooks: Lisp_Object; }
unsafe extern "C" { pub static mut Vbuffer_alist: Lisp_Object; }
// --- hand work still required ---
// bitfield: 0
// unmapped: 0
// array: 0
// unparsed: 0
