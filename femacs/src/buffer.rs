// femacs: port of src/buffer.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names
// (function, macro, struct, struct field, file-scope object, local), the same control flow,
// the same error model.  The 52 `DEFUN`s use `DEFUN!` (PORTING.md 5.1); C's non-`static`
// definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2) and the C `static` helpers stay
// plain `unsafe fn`.  This file owns the buffer object itself: the `BVAR` slots, the buffer
// list, `Fget_buffer_create`, the overlays' buffer side, and the `DEFVAR_PER_BUFFER` slots
// `syms_of_buffer` registers.
//
// `src/buffer.c` *is* in `docs/baseline/config/cond-eval-verified.txt`
// (`python3 tools/check-cond-eval.py src/buffer.c` = `28 conditions, 0 unknown,
// 0 disagreements [VERIFIED]`), so per P-08 the arms the frozen configuration does not
// compile are *recorded in `docs/ports/buffer.md`, not transcribed*.  The dead arms are
// `WINDOWSNT` (C 47-49, `w32heap.h`), `HAVE_TREE_SITTER` (C 51-53, 239-245, 648-653,
// 881-886, 1120-1122, 2648-2653, 4819-4821, 4890-4892), `GNUC_PREREQ (13, 0, 0)` (C
// 59-61), `USE_MMAP_FOR_BUFFERS` (C 2329-2332 and the whole C 4259-4609 allocator),
// `REL_ALLOC` (C 2609-2614), and `ITREE_DEBUG` (C 5047-5085, 6145-6147); all five gate
// macros are `undef` in `docs/baseline/config/darwin-arm64.macros.tsv`.
//
// names C declares in headers whose mirrors do not carry them yet are declared at their use
// site below, each with C's own prototype in a `/// C:` line (PORTING.md 0.1-2/P-72 rule 3,
// the insdel.rs/terminal.rs shape); the mirror gaps and the `xwidget.h` inline no-op
// `kill_buffer_xwidgets` are reported in `docs/ports/buffer.md` for the mirror owners.
//
// The `V*` objects this file's `syms_of_buffer` owns, plus the `DEFVAR_PER_BUFFER` slots
// (C stores those in `globals.f_V*`, status 3.29), are defined here, not merely referenced.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut)]

include!("include/config.rs");                 // C 19: `#include <config.h>`
include!("../lib/include/sys_types.in.rs");    // C 21: `#include <sys/types.h>`
include!("../lib/include/errno.rs");           // C 24: `#include <errno.h>`
include!("../lib/include/stdio.rs");           // C 25: `#include <stdio.h>`
include!("../lib/include/stdlib.rs");          // C 26: `#include <stdlib.h>`
include!("../lib/include/stdint.in.rs");       // OVERLAY_COUNT_MAX's PTRDIFF_MAX/SIZE_MAX
include!("../lib/include/unistd.in.rs");       // C 27: `#include <unistd.h>`
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`
include!("include/lisp.rs");                   // C 29: `#include "lisp.h"`
include!("include/intervals.rs");              // C 30: `#include "intervals.h"`
include!("include/process.rs");                // C 31: `#include "process.h"`
include!("include/systime.rs");                // C 32: `#include "systime.h"`
include!("include/composite.rs");              // coding.h 284: `#include "composite.h"`
include!("include/coding.rs");                 // frame.h 26: `#include "coding.h"`
include!("include/dispextern.rs");             // frame.h 25: `#include "dispextern.h"`
include!("include/font.rs");                   // dispextern.h's font names
include!("include/termhooks.rs");              // frame.h 24: `#include "termhooks.h"`
include!("include/window.rs");                 // C 33: `#include "window.h"`
include!("include/commands.rs");               // C 34: `#include "commands.h"`
include!("include/character.rs");              // C 35: `#include "character.h"`
include!("include/buffer.rs");                 // C 36: `#include "buffer.h"`
// C 37's `#include "region-cache.h"` has **no mirror** (`femacs/src/include/region-cache.rs`
// does not exist; the file of that name is the *unit* for src/region-cache.c).  The names
// this file takes from that header are declared at their use site below, each with C's own
// prototype in a `/// C:` line, and the gap is reported in docs/ports/buffer.md.
include!("include/indent.rs");                 // C 38: `#include "indent.h"`
include!("include/blockinput.rs");             // C 39: `#include "blockinput.h"`
include!("include/keymap.rs");                 // C 40: `#include "keymap.h"`
include!("include/frame.rs");                  // C 41: `#include "frame.h"`
include!("include/itree.rs");                  // C 44: `#include "itree.h"`
include!("include/pdumper.rs");                // C 45: `#include "pdumper.h"`

/* C spells `enum itree_order`'s members bare (`ASCENDING` inside `ITREE_FOREACH`), and
   Rust 2024 keeps enum variants out of scope, so the port imports them (the
   itree.rs/textprop.rs shape).  */
use itree_order::*;

/* The names C gets from lisp.h whose mirrors do not carry them yet (the insdel.rs/
   terminal.rs shape), each with C's own prototype in a `/// C:` line; they are reported
   in docs/ports/buffer.md for the mirror owners.  */
unsafe extern "C" {
  /// C: lisp.h 4660: `extern struct buffer *allocate_buffer (void) ATTRIBUTE_RETURNS_NONNULL;`
  fn allocate_buffer () -> *mut buffer;
  /// C: lisp.h 4434: `extern AVOID buffer_memory_full (ptrdiff_t);`
  fn buffer_memory_full (size: ptrdiff_t);
  /// C: lisp.h 4271: `extern Lisp_Object nconc2 (Lisp_Object, Lisp_Object);`
  fn nconc2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 4326: `extern void notify_variable_watchers (Lisp_Object, Lisp_Object,
  /// Lisp_Object, Lisp_Object);`
  fn notify_variable_watchers (symbol: Lisp_Object, newval: Lisp_Object,
                               operation: Lisp_Object, r#where: Lisp_Object);
  /// C: lisp.h 5297: `extern EMACS_INT get_random (void);`
  fn get_random () -> EMACS_INT;
  // `bset_update_mode_line` (window.h 1175) is the window.rs mirror's; the use-site copy and
  // its call sites' casts are gone - the mirror spells `struct buffer *` as `*mut c_void`
  // (P-72 rule 3), so the three callers pass the pointer cast (P-26).
  /// C: frame.h 1476: `extern struct frame *decode_live_frame (Lisp_Object);` (defined in
  /// frame.c)
  fn decode_live_frame (frame: Lisp_Object) -> *mut c_void; // struct frame *
  /// C: lisp.h 5250: `extern void truncate_undo_list (struct buffer *);`
  fn truncate_undo_list (b: *mut buffer);
  /// C: lisp.h 4322: `extern void make_gap_1 (struct buffer *, ptrdiff_t);`
  fn make_gap_1 (b: *mut buffer, delta: ptrdiff_t);
  /// C: lisp.h 4826: `extern void record_unwind_protect_excursion (void);`
  fn record_unwind_protect_excursion ();
  /// C: lisp.h 4267: `extern Lisp_Object do_yes_or_no_p (Lisp_Object);`
  fn do_yes_or_no_p (prompt: Lisp_Object) -> Lisp_Object;
  /// C: lisp.h 5121: `extern void frames_discard_buffer (Lisp_Object);`
  fn frames_discard_buffer (buffer: Lisp_Object);
  /// C: lisp.h 5180: `extern void kill_buffer_processes (Lisp_Object);`
  fn kill_buffer_processes (buffer: Lisp_Object);
  /// C: lisp.h 5346: `extern void unlock_buffer (struct buffer *);`
  fn unlock_buffer (b: *mut buffer);
  /// C: lisp.h 4954: `extern void clear_charpos_cache (struct buffer *);`
  fn clear_charpos_cache (b: *mut buffer);
  // window.h 1228/1229/1234 (`replace_buffer_in_windows`,
  // `replace_buffer_in_windows_safely`, `window_discard_buffer_from_dead_windows`) are the
  // window.rs mirror's now; the use-site copies are gone (P-26).
  /// C: region-cache.h 71: `void free_region_cache (struct region_cache *);` -- that header
  /// has no mirror (the gap docs/ports/buffer.md reports).
  fn free_region_cache (cache: *mut c_void); // struct region_cache *
  /// C: lisp.h 4931: `extern void labeled_restrictions_remove_in_current_buffer (void);`
  fn labeled_restrictions_remove_in_current_buffer ();
  /// C: lisp.h 4226: `extern ptrdiff_t chars_in_text (const unsigned char *, ptrdiff_t);`
  fn chars_in_text (ptr: *const u8, nbytes: ptrdiff_t) -> ptrdiff_t;
  /// C: lisp.h 5278: `extern void insert_1_both (const char *, ptrdiff_t, ptrdiff_t, bool,
  /// bool, bool);`
  fn insert_1_both (string: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
                    inherit: bool, prepare: bool, before_markers: bool);
  /// C: lisp.h 5273: `extern void del_range_2 (ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t,
  /// bool);`
  fn del_range_2 (from: ptrdiff_t, from_byte: ptrdiff_t, to: ptrdiff_t, to_byte: ptrdiff_t,
                  prepare: bool);
  /// C: lisp.h 5202: `extern void setup_process_coding_systems (Lisp_Object);`
  fn setup_process_coding_systems (process: Lisp_Object);
  /// C: <stdlib.h> 489: `extern void qsort (void *__base, size_t __nmemb, size_t __size,
  /// __compar_fn_t __compar) __nonnull ((1, 4));` (the port's stdlib.rs mirror does not
  /// carry it; declared here, the insdel.rs shape).
  fn qsort (base: *mut c_void, nmemb: usize, size: usize,
            compar: Option<unsafe extern "C" fn (*const c_void, *const c_void) -> c_int>);
  /// C: lisp.h 4323: `extern ptrdiff_t copy_text (const unsigned char *, unsigned char *,
  /// ptrdiff_t, bool, bool);`
  fn copy_text (from: *const u8, to: *mut u8, nbytes: ptrdiff_t, from_multibyte: bool,
                to_multibyte: bool) -> ptrdiff_t;
  /// C: lisp.h 4432: `extern AVOID memory_full (size_t);`
  fn memory_full (size: usize) -> !;
  /// C: lisp.h 5144: `extern char *emacs_strerror (int) ATTRIBUTE_RETURNS_NONNULL;`
  fn emacs_strerror (error_num: c_int) -> *mut c_char;
  /// C: lisp.h 5058: `extern Lisp_Object get_minibuffer (EMACS_INT);`
  fn get_minibuffer (depth: c_int) -> Lisp_Object;
  /// C: lisp.h 4245: `extern Lisp_Object larger_vector (Lisp_Object, ptrdiff_t, ptrdiff_t);`
  fn larger_vector (vec: Lisp_Object, elt_size: ptrdiff_t, new_size: ptrdiff_t) -> Lisp_Object;
}

/// C: xwidget.h 239: `INLINE void kill_buffer_xwidgets (Lisp_Object buf) {}` -- the
/// `#else` arm of `#ifdef HAVE_XWIDGETS` (undef in the frozen configuration, macros.tsv).
/// xwidget.h has no mirror, so the port keeps C's per-unit `static inline` copy here.
/// docs/ports/buffer.md reports the gap.
#[inline(always)]
unsafe fn kill_buffer_xwidgets (_buf: Lisp_Object) {}

/* This structure holds the default values of the buffer-local variables
   defined with DEFVAR_PER_BUFFER, that have special slots in each buffer.
   The default value occupies the same slot in this structure
   as an individual buffer's value occupies in that buffer.
   Setting the default value also goes through the alist of buffers
   and stores into each buffer that does not say it has a local value.  */

#[unsafe(no_mangle)]
pub static mut buffer_defaults: buffer = unsafe { std::mem::zeroed () };

/* This structure marks which slots in a buffer have corresponding
   default values in buffer_defaults.
   Each such slot has a value in this structure.
   The value is a positive Lisp integer that must be smaller than
   MAX_PER_BUFFER_VARS.

   When a buffer has its own local value for a slot,
   the entry for that slot (found in the same slot in this structure)
   is turned on in the buffer's local_flags array.

   If a slot in this structure is -1, then even though there may
   be a DEFVAR_PER_BUFFER for the slot, there is no default value for it;
   and the corresponding slot in buffer_defaults is not used.

   If a slot in this structure corresponding to a DEFVAR_PER_BUFFER is
   zero, that is a bug.  */

#[unsafe(no_mangle)]
pub static mut buffer_local_flags: buffer = unsafe { std::mem::zeroed () };

/* This structure holds the names of symbols whose values may be
   buffer-local.  It is indexed and accessed in the same way as the above.  */

#[unsafe(no_mangle)]
pub static mut buffer_local_symbols: buffer = unsafe { std::mem::zeroed () };

/* Return the symbol of the per-buffer variable at offset OFFSET in
   the buffer structure.  */

// C: `#define PER_BUFFER_SYMBOL(OFFSET) \
//      (*(Lisp_Object *)((OFFSET) + (char *) &buffer_local_symbols))`.
// `P-37`: the macro is a place expression, exactly as C's is (C 5039 assigns to it).
#[macro_export]
macro_rules! PER_BUFFER_SYMBOL {
  ($offset:expr) => {
    (*((($offset) as *mut u8).offset (&raw const buffer_local_symbols as *const c_char as isize)
       as *mut Lisp_Object))
  };
}

/* Maximum length of an overlay vector.  */
// C: `#define OVERLAY_COUNT_MAX						\
//   ((ptrdiff_t) min (MOST_POSITIVE_FIXNUM,				\
// 		    min (PTRDIFF_MAX, SIZE_MAX) / word_size))`
#[macro_export]
macro_rules! OVERLAY_COUNT_MAX {
  () => { (MOST_POSITIVE_FIXNUM.min ((PTRDIFF_MAX.min (SIZE_MAX as isize)) / word_size)) };
}

/* Flags indicating which built-in buffer-local variables
   are permanent locals.  */
static mut buffer_permanent_local_flags: [c_char; MAX_PER_BUFFER_VARS as usize]
  = [0; MAX_PER_BUFFER_VARS as usize];

/* Number of per-buffer variables used.  */

static mut last_per_buffer_idx: c_int = 0;

// C 116-120's `static void call_overlay_mod_hooks (...)` and C 121's
// `static void reset_buffer_local_variables (struct buffer *, int);` are forward
// declarations and are dropped (PORTING.md 0.1-12: Rust items are order-independent);
// the definitions stay where C has them (call_overlay_mod_hooks at C 4241,
// reset_buffer_local_variables at C 1137).

/* Alist of all buffer names vs the buffers.  This used to be
   a Lisp-visible variable, but is no longer, to prevent lossage
   due to user rplac'ing this alist or its elements.  */
#[unsafe(no_mangle)]
pub static mut Vbuffer_alist: Lisp_Object = lisp_h_Qnil ();

static mut QSFundamental: Lisp_Object = lisp_h_Qnil ();	/* A string "Fundamental".  */

// C 127-132's six `static` forward declarations (alloc_buffer_text, free_buffer_text,
// copy_overlays, modify_overlay, buffer_lisp_local_variables, buffer_local_variables_1)
// are dropped for the same reason; the definitions stay at C 4620, 4689, 698, 3689, 1425
// and 1460.

unsafe fn CHECK_OVERLAY (x: Lisp_Object)
{
  CHECK_TYPE (OVERLAYP (x), Qoverlayp (), x);
}

/* Convert the position POS to an EMACS_INT that fits in a fixnum.
   Yield POS's value if POS is already a fixnum, POS's marker position
   if POS is a marker, and MOST_NEGATIVE_FIXNUM or
   MOST_POSITIVE_FIXNUM if POS is a negative or positive bignum.
   Signal an error if POS is not of the proper form.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fix_position (pos: Lisp_Object) -> EMACS_INT
{
  if FIXNUMP (pos)
    { return XFIXNUM (pos); }
  if MARKERP (pos)
    { return marker_position (pos); }
  CHECK_TYPE (BIGNUMP (pos), Qinteger_or_marker_p (), pos);
  return if ! NILP (Fnatnump (pos)) { MOST_POSITIVE_FIXNUM } else { MOST_NEGATIVE_FIXNUM };
}

/* These setters are used only in this file, so they can be private.
   The public setters are inline functions defined in buffer.h.  */
unsafe fn bset_abbrev_mode (b: *mut buffer, val: Lisp_Object)
{
  (*b).abbrev_mode_ = val;
}
unsafe fn bset_abbrev_table (b: *mut buffer, val: Lisp_Object)
{
  (*b).abbrev_table_ = val;
}
unsafe fn bset_auto_fill_function (b: *mut buffer, val: Lisp_Object)
{
  (*b).auto_fill_function_ = val;
}
unsafe fn bset_auto_save_file_format (b: *mut buffer, val: Lisp_Object)
{
  (*b).auto_save_file_format_ = val;
}
unsafe fn bset_auto_save_file_name (b: *mut buffer, val: Lisp_Object)
{
  (*b).auto_save_file_name_ = val;
}
unsafe fn bset_backed_up (b: *mut buffer, val: Lisp_Object)
{
  (*b).backed_up_ = val;
}
unsafe fn bset_begv_marker (b: *mut buffer, val: Lisp_Object)
{
  (*b).begv_marker_ = val;
}
unsafe fn bset_bidi_display_reordering (b: *mut buffer, val: Lisp_Object)
{
  (*b).bidi_display_reordering_ = val;
}
unsafe fn bset_bidi_paragraph_start_re (b: *mut buffer, val: Lisp_Object)
{
  (*b).bidi_paragraph_start_re_ = val;
}
unsafe fn bset_bidi_paragraph_separate_re (b: *mut buffer, val: Lisp_Object)
{
  (*b).bidi_paragraph_separate_re_ = val;
}
unsafe fn bset_buffer_file_coding_system (b: *mut buffer, val: Lisp_Object)
{
  (*b).buffer_file_coding_system_ = val;
}
unsafe fn bset_ctl_arrow (b: *mut buffer, val: Lisp_Object)
{
  (*b).ctl_arrow_ = val;
}
unsafe fn bset_cursor_in_non_selected_windows (b: *mut buffer, val: Lisp_Object)
{
  (*b).cursor_in_non_selected_windows_ = val;
}
unsafe fn bset_cursor_type (b: *mut buffer, val: Lisp_Object)
{
  (*b).cursor_type_ = val;
}
unsafe fn bset_display_table (b: *mut buffer, val: Lisp_Object)
{
  (*b).display_table_ = val;
}
unsafe fn bset_extra_line_spacing (b: *mut buffer, val: Lisp_Object)
{
  (*b).extra_line_spacing_ = val;
}
unsafe fn bset_file_format (b: *mut buffer, val: Lisp_Object)
{
  (*b).file_format_ = val;
}
unsafe fn bset_file_truename (b: *mut buffer, val: Lisp_Object)
{
  (*b).file_truename_ = val;
}
unsafe fn bset_fringe_cursor_alist (b: *mut buffer, val: Lisp_Object)
{
  (*b).fringe_cursor_alist_ = val;
}
unsafe fn bset_fringe_indicator_alist (b: *mut buffer, val: Lisp_Object)
{
  (*b).fringe_indicator_alist_ = val;
}
unsafe fn bset_fringes_outside_margins (b: *mut buffer, val: Lisp_Object)
{
  (*b).fringes_outside_margins_ = val;
}
unsafe fn bset_header_line_format (b: *mut buffer, val: Lisp_Object)
{
  (*b).header_line_format_ = val;
}
unsafe fn bset_tab_line_format (b: *mut buffer, val: Lisp_Object)
{
  (*b).tab_line_format_ = val;
}
unsafe fn bset_indicate_buffer_boundaries (b: *mut buffer, val: Lisp_Object)
{
  (*b).indicate_buffer_boundaries_ = val;
}
unsafe fn bset_indicate_empty_lines (b: *mut buffer, val: Lisp_Object)
{
  (*b).indicate_empty_lines_ = val;
}
unsafe fn bset_invisibility_spec (b: *mut buffer, val: Lisp_Object)
{
  (*b).invisibility_spec_ = val;
}
unsafe fn bset_left_fringe_width (b: *mut buffer, val: Lisp_Object)
{
  (*b).left_fringe_width_ = val;
}
unsafe fn bset_major_mode (b: *mut buffer, val: Lisp_Object)
{
  (*b).major_mode_ = val;
}
unsafe fn bset_local_minor_modes (b: *mut buffer, val: Lisp_Object)
{
  (*b).local_minor_modes_ = val;
}
unsafe fn bset_mark (b: *mut buffer, val: Lisp_Object)
{
  (*b).mark_ = val;
}
unsafe fn bset_mode_line_format (b: *mut buffer, val: Lisp_Object)
{
  (*b).mode_line_format_ = val;
}
unsafe fn bset_mode_name (b: *mut buffer, val: Lisp_Object)
{
  (*b).mode_name_ = val;
}
unsafe fn bset_name (b: *mut buffer, val: Lisp_Object)
{
  (*b).name_ = val;
}
unsafe fn bset_last_name (b: *mut buffer, val: Lisp_Object)
{
  (*b).last_name_ = val;
}
unsafe fn bset_overwrite_mode (b: *mut buffer, val: Lisp_Object)
{
  (*b).overwrite_mode_ = val;
}
unsafe fn bset_pt_marker (b: *mut buffer, val: Lisp_Object)
{
  (*b).pt_marker_ = val;
}
unsafe fn bset_right_fringe_width (b: *mut buffer, val: Lisp_Object)
{
  (*b).right_fringe_width_ = val;
}
unsafe fn bset_save_length (b: *mut buffer, val: Lisp_Object)
{
  (*b).save_length_ = val;
}
unsafe fn bset_scroll_bar_width (b: *mut buffer, val: Lisp_Object)
{
  (*b).scroll_bar_width_ = val;
}
unsafe fn bset_scroll_bar_height (b: *mut buffer, val: Lisp_Object)
{
  (*b).scroll_bar_height_ = val;
}
unsafe fn bset_scroll_down_aggressively (b: *mut buffer, val: Lisp_Object)
{
  (*b).scroll_down_aggressively_ = val;
}
unsafe fn bset_scroll_up_aggressively (b: *mut buffer, val: Lisp_Object)
{
  (*b).scroll_up_aggressively_ = val;
}
unsafe fn bset_selective_display (b: *mut buffer, val: Lisp_Object)
{
  (*b).selective_display_ = val;
}
unsafe fn bset_selective_display_ellipses (b: *mut buffer, val: Lisp_Object)
{
  (*b).selective_display_ellipses_ = val;
}
unsafe fn bset_vertical_scroll_bar_type (b: *mut buffer, val: Lisp_Object)
{
  (*b).vertical_scroll_bar_type_ = val;
}
unsafe fn bset_horizontal_scroll_bar_type (b: *mut buffer, val: Lisp_Object)
{
  (*b).horizontal_scroll_bar_type_ = val;
}
unsafe fn bset_word_wrap (b: *mut buffer, val: Lisp_Object)
{
  (*b).word_wrap_ = val;
}
unsafe fn bset_zv_marker (b: *mut buffer, val: Lisp_Object)
{
  (*b).zv_marker_ = val;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nsberror (spec: Lisp_Object)
{
  if STRINGP (spec)
    { error (c"No buffer named %s".as_ptr (), SDATA (spec)); }
  error (c"Invalid buffer argument".as_ptr ());
}

DEFUN! ("buffer-live-p", Fbuffer_live_p, Sbuffer_live_p, 1, 1, 0,
       doc: "Return t if OBJECT is a buffer which has not been killed.\nValue is nil if OBJECT is not a buffer or if it has been killed.",
       (object: Lisp_Object),
       {
  return if BUFFERP (object) && BUFFER_LIVE_P (XBUFFER (object)) { Qt () } else { Qnil () };
});

DEFUN! ("buffer-list", Fbuffer_list, Sbuffer_list, 0, 1, 0,
       doc: "Return a list of all live buffers.\nIf the optional arg FRAME is a frame, return the buffer list in the\nproper order for that frame: the buffers shown in FRAME come first,\nfollowed by the rest of the buffers.",
       (frame: Lisp_Object),
       {
  let mut general: Lisp_Object = Qnil ();
  general = Fmapcar (Qcdr (), Vbuffer_alist);

  if FRAMEP (frame)
    {
      let mut framelist: Lisp_Object = Qnil ();
      let mut prevlist: Lisp_Object = Qnil ();
      let mut tail: Lisp_Object = Qnil ();

      framelist = Fcopy_sequence (BVAR! (XFRAME (frame), buffer_list));
      prevlist = Fnreverse (Fcopy_sequence (BVAR! (XFRAME (frame), buried_buffer_list)));

      /* Remove from GENERAL any buffer that duplicates one in
         FRAMELIST or PREVLIST.  */
      tail = framelist;
      while CONSP (tail)
	{
	  general = Fdelq (XCAR (tail), general);
	  tail = XCDR (tail);
	}
      tail = prevlist;
      while CONSP (tail)
	{
	  general = Fdelq (XCAR (tail), general);
	  tail = XCDR (tail);
	}

      return CALLN! (Fnconc, framelist, general, prevlist);
    }
  else
    { return general; }
});

/* Like Fassoc, but use Fstring_equal to compare
   (which ignores text properties), and don't ever quit.  */

unsafe fn assoc_ignore_text_properties (key: Lisp_Object, list: Lisp_Object) -> Lisp_Object
{
  let mut tail: Lisp_Object = Qnil ();
  tail = list;
  while CONSP (tail)
    {
      let elt: Lisp_Object = XCAR (tail);
      if ! NILP (Fstring_equal (Fcar (elt), key))
	{ return elt; }
      tail = XCDR (tail);
    }
  return Qnil ();
}

DEFUN! ("get-buffer", Fget_buffer, Sget_buffer, 1, 1, 0,
       doc: "Return the buffer named BUFFER-OR-NAME.\nBUFFER-OR-NAME must be either a string or a buffer.  If BUFFER-OR-NAME\nis a string and there is no buffer with that name, return nil.  If\nBUFFER-OR-NAME is a buffer, return it as given.",
       (buffer_or_name: Lisp_Object),
       {
  if BUFFERP (buffer_or_name)
    { return buffer_or_name; }
  CHECK_STRING (buffer_or_name);

  return Fcdr (assoc_ignore_text_properties (buffer_or_name, Vbuffer_alist));
});

DEFUN! ("get-file-buffer", Fget_file_buffer, Sget_file_buffer, 1, 1, 0,
       doc: "Return the buffer visiting file FILENAME (a string).\nThe buffer's `buffer-file-name' must match exactly the expansion of FILENAME.\nIf there is no such live buffer, return nil.\nSee also `find-buffer-visiting'.",
       (filename: Lisp_Object),
       {
  /* C's `filename = Fexpand_file_name (...)` below rebinds the argument.  */
  let mut filename = filename;
  let mut tail: Lisp_Object = Qnil ();
  let mut buf: Lisp_Object = Qnil ();
  let mut handler: Lisp_Object = Qnil ();

  CHECK_STRING (filename);
  filename = Fexpand_file_name (filename, Qnil ());

  /* If the file name has special constructs in it,
     call the corresponding file name handler.  */
  handler = Ffind_file_name_handler (filename, Qget_file_buffer ());
  if ! NILP (handler)
    {
      let handled_buf: Lisp_Object = calln! (handler, Qget_file_buffer (), filename);
      return if BUFFERP (handled_buf) { handled_buf } else { Qnil () };
    }

  /* C: `FOR_EACH_LIVE_BUFFER (tail, buf)`, i.e. `FOR_EACH_ALIST_VALUE (Vbuffer_alist, tail,
     buf)`; the loop macro has no Rust counterpart (P-22,
     docs/ports/excluded-macros.txt), so the expansion is written here (the unlock_all_files
     shape).  C's `continue` becomes the `if`'s condition, since the loop's increment runs
     after the body on every path.  */
  tail = Vbuffer_alist;
  while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }
    {
      if STRINGP (BVAR! (XBUFFER (buf), filename_))
	&& ! NILP (Fstring_equal (BVAR! (XBUFFER (buf), filename_), filename))
	{ return buf; }
      tail = XCDR (tail);
    }
  return Qnil ();
});

DEFUN! ("get-truename-buffer", Fget_truename_buffer, Sget_truename_buffer, 1, 1, 0,
       doc: "Return the buffer with `file-truename' equal to FILENAME (a string).\nIf there is no such live buffer, return nil.\nSee also `find-buffer-visiting'.",
       (filename: Lisp_Object),
       {
  let mut tail: Lisp_Object = Qnil ();
  let mut buf: Lisp_Object = Qnil ();

  /* C 521-522.  */
  tail = Vbuffer_alist;
  while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }
    {
      if STRINGP (BVAR! (XBUFFER (buf), file_truename_))
	&& ! NILP (Fstring_equal (BVAR! (XBUFFER (buf), file_truename_), filename))
	{ return buf; }
      tail = XCDR (tail);
    }
  return Qnil ();
});

DEFUN! ("find-buffer", Ffind_buffer, Sfind_buffer, 2, 2, 0,
       doc: "Return the buffer with buffer-local VARIABLE `equal' to VALUE.\nIf there is no such live buffer, return nil.\nSee also `find-buffer-visiting'.",
       (variable: Lisp_Object, value: Lisp_Object),
       {
  let mut tail: Lisp_Object = Qnil ();
  let mut buf: Lisp_Object = Qnil ();

  /* C 549-552.  */
  tail = Vbuffer_alist;
  while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }
    {
      if ! NILP (Fequal (value, Fbuffer_local_value (variable, buf)))
	{ return buf; }
      tail = XCDR (tail);
    }
  return Qnil ();
});

/* Run buffer-list-update-hook if Vrun_hooks is non-nil and BUF does
   not have buffer hooks inhibited.  */

unsafe fn run_buffer_list_update_hook (buf: *mut buffer)
{
  eassert! (! buf.is_null ());
  if ! (NILP (Vrun_hooks) || (*buf).bits & BUFFER_INHIBIT_BUFFER_HOOKS != 0)
    { calln! (Vrun_hooks, Qbuffer_list_update_hook ()); }
}

DEFUN! ("get-buffer-create", Fget_buffer_create, Sget_buffer_create, 1, 2, 0,
       doc: "Return the buffer specified by BUFFER-OR-NAME, creating a new one if needed.\nIf BUFFER-OR-NAME is a string and a live buffer with that name exists,\nreturn that buffer.  If no such buffer exists, create a new buffer with\nthat name and return it.\n\nIf BUFFER-OR-NAME starts with a space, the new buffer does not keep undo\ninformation.  If optional argument INHIBIT-BUFFER-HOOKS is non-nil, the\nnew buffer does not run the hooks `kill-buffer-hook',\n`kill-buffer-query-functions', and `buffer-list-update-hook'.  This\navoids slowing down internal or temporary buffers that are never\npresented to users or passed on to other applications.\n\nIf BUFFER-OR-NAME is a buffer instead of a string, return it as given,\neven if it is dead.  The return value is never nil.",
       (buffer_or_name: Lisp_Object, inhibit_buffer_hooks: Lisp_Object),
       {
  let mut buffer: Lisp_Object = Qnil ();
  let mut name: Lisp_Object = Qnil ();
  let mut b: *mut buffer;

  buffer = Fget_buffer (buffer_or_name);
  if ! NILP (buffer)
    { return buffer; }

  if SCHARS (buffer_or_name) == 0
    { error (c"Empty string for buffer name is not allowed".as_ptr ()); }

  b = allocate_buffer ();

  /* An ordinary buffer uses its own struct buffer_text.  */
  (*b).text = &raw mut (*b).own_text;
  (*b).base_buffer = std::ptr::null_mut ();
  /* No one shares the text with us now.  */
  (*b).indirections = 0;
  /* No one shows us now.  */
  (*b).window_count = 0;

  memset (&raw mut (*b).local_flags as *mut c_void, 0,
	  std::mem::size_of_val (&(*b).local_flags));

  BUF_GAP_SIZE! (b) = 20;
  block_input ();
  /* We allocate extra 1-byte at the tail and keep it always '\0' for
     anchoring a search.  */
  alloc_buffer_text (b, BUF_GAP_SIZE! (b) + 1);
  unblock_input ();
  if BUF_BEG_ADDR! (b).is_null ()
    { buffer_memory_full (BUF_GAP_SIZE! (b) + 1); }

  (*b).pt = BEG as ptrdiff_t;
  (*b).begv = BEG as ptrdiff_t;
  (*b).zv = BEG as ptrdiff_t;
  (*b).pt_byte = BEG_BYTE as ptrdiff_t;
  (*b).begv_byte = BEG_BYTE as ptrdiff_t;
  (*b).zv_byte = BEG_BYTE as ptrdiff_t;

  BUF_GPT! (b) = BEG as ptrdiff_t;
  BUF_GPT_BYTE! (b) = BEG_BYTE as ptrdiff_t;

  BUF_Z! (b) = BEG as ptrdiff_t;
  BUF_Z_BYTE! (b) = BEG_BYTE as ptrdiff_t;
  BUF_MODIFF! (b) = 1;
  BUF_CHARS_MODIFF! (b) = 1;
  BUF_OVERLAY_MODIFF! (b) = 1;
  BUF_SAVE_MODIFF! (b) = 1;
  BUF_COMPACT! (b) = 1;
  set_buffer_intervals (b, std::ptr::null_mut ());
  BUF_UNCHANGED_MODIFIED! (b) = 1;
  BUF_OVERLAY_UNCHANGED_MODIFIED! (b) = 1;
  BUF_END_UNCHANGED! (b) = 0;
  BUF_BEG_UNCHANGED! (b) = 0;
  /* C: `*(BUF_GPT_ADDR (b)) = *(BUF_Z_ADDR (b)) = 0;  /* Put an anchor '\0'.  */` */
  *(BUF_GPT_ADDR (b)) = 0;
  *(BUF_Z_ADDR (b)) = 0;
  // C: `b->text->inhibit_shrinking = false;` / `b->text->redisplay = false;` -- the two
  // `bool_bf` members buffer.h packs into `bits` (token-aliases.tsv's two families).
  (*(*b).text).bits &= ! BUFFER_TEXT_INHIBIT_SHRINKING;
  (*(*b).text).bits &= ! BUFFER_TEXT_REDISPLAY;

  (*b).newline_cache = std::ptr::null_mut ();
  (*b).width_run_cache = std::ptr::null_mut ();
  (*b).bidi_paragraph_cache = std::ptr::null_mut ();
  bset_width_table (b, Qnil ());
  // C: `b->prevent_redisplay_optimizations_p = 1;` (buffer.h's 1-bit member).
  (*b).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;

  // C 648-653 is `#ifdef HAVE_TREE_SITTER` (undef): the three `SET_BUF_TS_LINECOL_*`
  // assignments of `TREESIT_EMPTY_LINECOL`.  Recorded in docs/ports/buffer.md.

  /* An ordinary buffer normally doesn't need markers
     to handle BEGV and ZV.  */
  bset_pt_marker (b, Qnil ());
  bset_begv_marker (b, Qnil ());
  bset_zv_marker (b, Qnil ());

  name = Fcopy_sequence (buffer_or_name);
  set_string_intervals (name, std::ptr::null_mut ());
  bset_name (b, name);
  bset_last_name (b, name);

  // C: `b->inhibit_buffer_hooks = !NILP (inhibit_buffer_hooks);` (buffer.h's 1-bit member).
  if ! NILP (inhibit_buffer_hooks)
    { (*b).bits |= BUFFER_INHIBIT_BUFFER_HOOKS; }
  else
    { (*b).bits &= ! BUFFER_INHIBIT_BUFFER_HOOKS; }
  bset_undo_list (b, if SREF (name, 0) != b' ' { Qnil () } else { Qt () });

  reset_buffer (b);
  reset_buffer_local_variables (b, 1);

  bset_mark (b, Fmake_marker ());
  BUF_MARKERS! (b) = std::ptr::null_mut ();

  /* Put this in the alist of all live buffers.  */
  XSETBUFFER! (buffer, b);
  Vbuffer_alist = nconc2 (Vbuffer_alist, list1 (Fcons (name, buffer)));

  run_buffer_list_update_hook (b);

  return buffer;
});

unsafe fn add_buffer_overlay (b: *mut buffer, ov: *mut Lisp_Overlay,
                              begin: ptrdiff_t, end: ptrdiff_t)
{
  eassert! ((*ov).buffer.is_null ());
  if (*b).overlays.is_null ()
    { (*b).overlays = itree_create (); }
  // lisp.h's `struct Lisp_Overlay.buffer` is a `struct buffer *`; the mirror spells it
  // `*mut c_void` because lisp.h does not include buffer.h (P-72 rule 3).
  (*ov).buffer = b as *mut c_void;
  itree_insert ((*b).overlays, (*ov).interval as *mut itree_node, begin, end);
}

/* Copy overlays of buffer FROM to buffer TO.  */

unsafe fn copy_overlays (from: *mut buffer, to: *mut buffer)
{
  eassert! (! to.is_null () && (*to).overlays.is_null ());
  let mut node: *mut itree_node;

  /* itree.h 166-176's `ITREE_FOREACH (node, from->overlays, PTRDIFF_MIN, PTRDIFF_MAX,
     ASCENDING)` is expanded here (P-33, status 3.7), the textprop.rs shape.  */
  if (*from).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_,
				(*from).overlays, PTRDIFF_MIN, PTRDIFF_MAX, ITREE_ASCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  let ov: Lisp_Object = (*node).data;
	  let copy: Lisp_Object = build_overlay (
	    (*node).bits & ITREE_FRONT_ADVANCE != 0,
	    (*node).bits & ITREE_REAR_ADVANCE != 0,
	    Fcopy_sequence (OVERLAY_PLIST (ov)));
	  add_buffer_overlay (to, XOVERLAY (copy), (*node).begin, (*node).end);
	}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn valid_per_buffer_idx (idx: c_int) -> bool
{
  return 0 <= idx && idx < last_per_buffer_idx;
}

/* Clone per-buffer values of buffer FROM.

   Buffer TO gets the same per-buffer values as FROM, with the
   following exceptions: (1) TO's name is left untouched, (2) markers
   are copied and made to refer to TO, and (3) overlay lists are
   copied.  */

unsafe fn clone_per_buffer_values (from: *mut buffer, to: *mut buffer)
{
  /* C declares this `int offset`, but the mirror's FOR_EACH_PER_BUFFER_OBJECT_AT does
     `offset += word_size` where the port's `word_size` is a `ptrdiff_t` const, so the
     port uses C's ptrdiff_t and casts at the two per_buffer_* calls (PORTING.md P-37).  */
  let mut offset: ptrdiff_t = 0;

  FOR_EACH_PER_BUFFER_OBJECT_AT! (offset, {
      let mut obj: Lisp_Object = Qnil ();

      /* Don't touch the `name' which should be unique for every buffer.  */
      if offset == PER_BUFFER_VAR_OFFSET! (name_)
	{ continue; }

      obj = per_buffer_value (from, offset as c_int);
      if MARKERP (obj) && (*XMARKER (obj)).buffer as *mut buffer == from
	{
	  let m: *mut Lisp_Marker = XMARKER (obj);

	  obj = build_marker (to as *mut c_void, (*m).charpos, (*m).bytepos);
	  // C: `XMARKER (obj)->insertion_type = m->insertion_type;` (lisp.h's 1-bit member,
	  // the `insertion_type` family of token-aliases.tsv).
	  if (*m).bits & MARKER_INSERTION_TYPE != 0
	    { (*XMARKER (obj)).bits |= MARKER_INSERTION_TYPE; }
	  else
	    { (*XMARKER (obj)).bits &= ! MARKER_INSERTION_TYPE; }
	}

      set_per_buffer_value (to, offset as c_int, obj);
    });

  // C: `memcpy (to->local_flags, from->local_flags, sizeof to->local_flags);`
  std::ptr::copy_nonoverlapping (&raw const (*from).local_flags as *const c_char,
				 &raw mut (*to).local_flags as *mut c_char,
				 std::mem::size_of_val (&(*to).local_flags));

  copy_overlays (from, to);

  /* Get (a copy of) the alist of Lisp-level local variables of FROM
     and install that in TO.  */
  bset_local_var_alist (to, buffer_lisp_local_variables (from, true));
}

/* If buffer B has markers to record PT, BEGV and ZV when it is not
   current, update these markers.  */

unsafe fn record_buffer_markers (b: *mut buffer)
{
  if ! NILP (BVAR! (b, pt_marker_))
    {
      let mut buffer: Lisp_Object = Qnil ();

      eassert! (! NILP (BVAR! (b, begv_marker_)));
      eassert! (! NILP (BVAR! (b, zv_marker_)));

      XSETBUFFER! (buffer, b);
      set_marker_both (BVAR! (b, pt_marker_), buffer, (*b).pt, (*b).pt_byte);
      set_marker_both (BVAR! (b, begv_marker_), buffer, (*b).begv, (*b).begv_byte);
      set_marker_both (BVAR! (b, zv_marker_), buffer, (*b).zv, (*b).zv_byte);
    }
}

/* If buffer B has markers to record PT, BEGV and ZV when it is not
   current, fetch these values into B->begv etc.  */

unsafe fn fetch_buffer_markers (b: *mut buffer)
{
  if ! NILP (BVAR! (b, pt_marker_))
    {
      let mut m: Lisp_Object = Qnil ();

      eassert! (! NILP (BVAR! (b, begv_marker_)));
      eassert! (! NILP (BVAR! (b, zv_marker_)));

      m = BVAR! (b, pt_marker_);
      SET_BUF_PT_BOTH (b, marker_position (m), marker_byte_position (m));

      m = BVAR! (b, begv_marker_);
      SET_BUF_BEGV_BOTH (b, marker_position (m), marker_byte_position (m));

      m = BVAR! (b, zv_marker_);
      SET_BUF_ZV_BOTH (b, marker_position (m), marker_byte_position (m));
    }
}

DEFUN! ("make-indirect-buffer", Fmake_indirect_buffer, Smake_indirect_buffer,
       2, 4,
       "bMake indirect buffer (to buffer): \nBName of indirect buffer: ",
       doc: "Create and return an indirect buffer for buffer BASE-BUFFER, named NAME.\nBASE-BUFFER should be a live buffer, or the name of an existing buffer.\n\nNAME should be a string which is not the name of an existing buffer.\n\nInteractively, prompt for BASE-BUFFER (offering the current buffer as\nthe default), and for NAME (offering as default the name of a recently\nused buffer).\n\nOptional argument CLONE non-nil means preserve BASE-BUFFER's state,\nsuch as major and minor modes, in the indirect buffer.\nCLONE nil means the indirect buffer's state is reset to default values.\n\nIf optional argument INHIBIT-BUFFER-HOOKS is non-nil, the new buffer\ndoes not run the hooks `kill-buffer-hook',\n`kill-buffer-query-functions', and `buffer-list-update-hook'.\n\nInteractively, CLONE and INHIBIT-BUFFER-HOOKS are nil.",
       (base_buffer: Lisp_Object, name: Lisp_Object, clone: Lisp_Object,
        inhibit_buffer_hooks: Lisp_Object),
       {
  let mut buf: Lisp_Object = Qnil ();
  let mut tem: Lisp_Object = Qnil ();
  let mut b: *mut buffer;

  /* C's `base_buffer`, `name` and `buf` are the arguments themselves; the port shadows the
     two that are reassigned (PORTING.md: `DEFUN!` parameters are immutable).  */
  let mut base_buffer = base_buffer;
  let mut name = name;

  CHECK_STRING (name);
  buf = Fget_buffer (name);
  if ! NILP (buf)
    { error (c"Buffer name `%s' is in use".as_ptr (), SDATA (name)); }

  tem = base_buffer;
  base_buffer = Fget_buffer (base_buffer);
  if NILP (base_buffer)
    { error (c"No such buffer: `%s'".as_ptr (), SDATA (tem)); }
  if ! BUFFER_LIVE_P (XBUFFER (base_buffer))
    { error (c"Base buffer has been killed".as_ptr ()); }

  if SCHARS (name) == 0
    { error (c"Empty string for buffer name is not allowed".as_ptr ()); }

  b = allocate_buffer ();

  /* No double indirection - if base buffer is indirect,
     new buffer becomes an indirect to base's base.  */
  (*b).base_buffer = if ! (*XBUFFER (base_buffer)).base_buffer.is_null ()
    { (*XBUFFER (base_buffer)).base_buffer }
    else { XBUFFER (base_buffer) };

  /* Use the base buffer's text object.  */
  (*b).text = (*(*b).base_buffer).text;
  /* We have no own text.  */
  (*b).indirections = -1;
  /* Notify base buffer that we share the text now.  */
  (*(*b).base_buffer).indirections += 1;
  /* Always -1 for an indirect buffer.  */
  (*b).window_count = -1;

  memset (&raw mut (*b).local_flags as *mut c_void, 0,
	  std::mem::size_of_val (&(*b).local_flags));

  (*b).pt = (*(*b).base_buffer).pt;
  (*b).begv = (*(*b).base_buffer).begv;
  (*b).zv = (*(*b).base_buffer).zv;
  (*b).pt_byte = (*(*b).base_buffer).pt_byte;
  (*b).begv_byte = (*(*b).base_buffer).begv_byte;
  (*b).zv_byte = (*(*b).base_buffer).zv_byte;
  // C: `b->inhibit_buffer_hooks = !NILP (inhibit_buffer_hooks);` (buffer.h's 1-bit member).
  if ! NILP (inhibit_buffer_hooks)
    { (*b).bits |= BUFFER_INHIBIT_BUFFER_HOOKS; }
  else
    { (*b).bits &= ! BUFFER_INHIBIT_BUFFER_HOOKS; }

  (*b).newline_cache = std::ptr::null_mut ();
  (*b).width_run_cache = std::ptr::null_mut ();
  (*b).bidi_paragraph_cache = std::ptr::null_mut ();
  bset_width_table (b, Qnil ());

  // C 881-886 is `#ifdef HAVE_TREE_SITTER` (undef).  Recorded in docs/ports/buffer.md.

  name = Fcopy_sequence (name);
  set_string_intervals (name, std::ptr::null_mut ());
  bset_name (b, name);
  bset_last_name (b, name);

  /* An indirect buffer shares undo list of its base (Bug#18180).  */
  bset_undo_list (b, BVAR! ((*b).base_buffer, undo_list_));

  reset_buffer (b);
  reset_buffer_local_variables (b, 1);

  /* Put this in the alist of all live buffers.  */
  XSETBUFFER! (buf, b);
  Vbuffer_alist = nconc2 (Vbuffer_alist, list1 (Fcons (name, buf)));

  bset_mark (b, Fmake_marker ());

  /* The multibyte status belongs to the base buffer.  */
  bset_enable_multibyte_characters
    (b, BVAR! ((*b).base_buffer, enable_multibyte_characters_));

  /* Make sure the base buffer has markers for its narrowing.  */
  if NILP (BVAR! ((*b).base_buffer, pt_marker_))
    {
      eassert! (NILP (BVAR! ((*b).base_buffer, begv_marker_)));
      eassert! (NILP (BVAR! ((*b).base_buffer, zv_marker_)));

      bset_pt_marker ((*b).base_buffer,
		      build_marker ((*b).base_buffer as *mut c_void,
				    (*(*b).base_buffer).pt, (*(*b).base_buffer).pt_byte));

      bset_begv_marker ((*b).base_buffer,
			build_marker ((*b).base_buffer as *mut c_void,
				      (*(*b).base_buffer).begv,
				      (*(*b).base_buffer).begv_byte));

      bset_zv_marker ((*b).base_buffer,
		      build_marker ((*b).base_buffer as *mut c_void,
				    (*(*b).base_buffer).zv, (*(*b).base_buffer).zv_byte));

      // C: `XMARKER (BVAR (b->base_buffer, zv_marker))->insertion_type = 1;`
      (*XMARKER (BVAR! ((*b).base_buffer, zv_marker_))).bits |= MARKER_INSERTION_TYPE;
    }

  if NILP (clone)
    {
      /* Give the indirect buffer markers for its narrowing.  */
      bset_pt_marker (b, build_marker (b as *mut c_void, (*b).pt, (*b).pt_byte));
      bset_begv_marker (b, build_marker (b as *mut c_void, (*b).begv, (*b).begv_byte));
      bset_zv_marker (b, build_marker (b as *mut c_void, (*b).zv, (*b).zv_byte));
      (*XMARKER (BVAR! (b, zv_marker_))).bits |= MARKER_INSERTION_TYPE;
    }
  else
    {
      let count: specpdl_ref = SPECPDL_INDEX ();

      clone_per_buffer_values ((*b).base_buffer, b);
      bset_filename (b, Qnil ());
      bset_file_truename (b, Qnil ());
      bset_display_count (b, make_fixnum (0));
      bset_backed_up (b, Qnil ());
      bset_local_minor_modes (b, Qnil ());
      bset_auto_save_file_name (b, Qnil ());
      record_unwind_current_buffer ();
      set_buffer_internal_1 (b);
      Fset (Qbuffer_save_without_query (), Qnil ());
      Fset (Qbuffer_file_number (), Qnil ());
      if ! NILP (Flocal_variable_p (Qbuffer_stale_function (), base_buffer))
	{ Fkill_local_variable (Qbuffer_stale_function ()); }
      /* Cloned buffers need extra setup, to do things such as deep
	 variable copies for list variables that might be mangled due
	 to destructive operations in the indirect buffer. */
      run_hook (Qclone_indirect_buffer_hook ());
      unbind_to (count, Qnil ());
    }

  run_buffer_list_update_hook (b);

  return buf;
});

unsafe fn remove_buffer_overlay (b: *mut buffer, ov: *mut Lisp_Overlay)
{
  eassert! (! (*b).overlays.is_null ());
  eassert! ((*ov).buffer as *mut buffer == b);
  itree_remove ((*((*ov).buffer as *mut buffer)).overlays,
		(*ov).interval as *mut itree_node);
  (*ov).buffer = std::ptr::null_mut ();
}

/* Mark OV as no longer associated with its buffer.  */

unsafe fn drop_overlay (ov: *mut Lisp_Overlay)
{
  if (*ov).buffer.is_null ()
    { return; }

  if ! NILP ((*ov).plist)
    { modify_overlay ((*ov).buffer as *mut buffer, overlay_start (ov), overlay_end (ov)); }
  remove_buffer_overlay ((*ov).buffer as *mut buffer, ov);
}

/* Delete all overlays of B and reset its overlay lists.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn delete_all_overlays (b: *mut buffer)
{
  let mut node: *mut itree_node;

  if (*b).overlays.is_null ()
    { return; }

  /* The general rule is that the tree cannot be modified from within
     ITREE_FOREACH, but here we bend this rule a little because we know
     that the POST_ORDER iterator will not need to look at `node` again.  */
  /* itree.h 166-176's `ITREE_FOREACH (node, b->overlays, PTRDIFF_MIN, PTRDIFF_MAX,
     POST_ORDER)` is expanded here (P-33, status 3.7).  */
  if (*b).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_,
				(*b).overlays, PTRDIFF_MIN, PTRDIFF_MAX, ITREE_POST_ORDER);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  modify_overlay (b, (*node).begin, (*node).end);
	  (*XOVERLAY ((*node).data)).buffer = std::ptr::null_mut ();
	  (*node).parent = std::ptr::null_mut ();
	  (*node).left = std::ptr::null_mut ();
	  (*node).right = std::ptr::null_mut ();
	}
    }
  itree_clear ((*b).overlays);
}

unsafe fn free_buffer_overlays (b: *mut buffer)
{
  /* Actually this does not free any overlay, but the tree only.  --ap */
  if ! (*b).overlays.is_null ()
    {
      itree_destroy ((*b).overlays);
      (*b).overlays = std::ptr::null_mut ();
    }
}

/* Adjust the position of overlays in the current buffer according to
   MULTIBYTE.

   Assume that positions currently correspond to byte positions, if
   MULTIBYTE is true and to character positions if not.
*/

unsafe fn set_overlays_multibyte (multibyte: bool)
{
  if (*(current_buffer! ())).overlays.is_null () || Z! () == Z_BYTE! ()
    { return; }

  let mut nodes: *mut *mut itree_node = std::ptr::null_mut ();
  let tree: *mut itree_tree = (*(current_buffer! ())).overlays;
  let size: intmax_t = itree_size (tree);

  /* We can't use `itree_node_set_region` at the same time
     as we iterate over the itree, so we need an auxiliary storage
     to keep the list of nodes.  */
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  SAFE_NALLOCA! (sa_avail, nodes, 1, size);
  {
    let mut node: *mut itree_node;
    let mut cursor: *mut *mut itree_node = nodes;
    /* itree.h 166-176's `ITREE_FOREACH (node, tree, PTRDIFF_MIN, PTRDIFF_MAX,
       ASCENDING)`.  */
    if tree.is_null ()
      { }
    else
      {
	let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
	let itree_iter_: *mut itree_iterator
	  = itree_iterator_start (&raw mut itree_local_iter_,
				  tree, PTRDIFF_MIN, PTRDIFF_MAX, ITREE_ASCENDING);
	while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	  {
	    *cursor = node;
	    cursor = cursor.add (1);
	  }
      }
  }

  let mut i: intmax_t = 0;
  while i < size
    {
      let node: *mut itree_node = *nodes;

      if multibyte
	{
	  let mut begin: ptrdiff_t = itree_node_begin (tree, node);
	  let mut end: ptrdiff_t = itree_node_end (tree, node);

	  /* This models the behavior of markers.  (The behavior of
	     text-intervals differs slightly.) */
	  while begin < Z_BYTE! () && ! CHAR_HEAD_P (FETCH_BYTE! (begin) as c_int)
	    { begin += 1; }
	  while end < Z_BYTE! () && ! CHAR_HEAD_P (FETCH_BYTE! (end) as c_int)
	    { end += 1; }
	  itree_node_set_region (tree, node, BYTE_TO_CHAR (begin),
				    BYTE_TO_CHAR (end));
	}
      else
	{
	  itree_node_set_region (tree, node, CHAR_TO_BYTE ((*node).begin),
				    CHAR_TO_BYTE ((*node).end));
	}
      i += 1;
      nodes = nodes.add (1);
    }
  SAFE_FREE! (sa_count);
}

/* Reinitialize everything about a buffer except its name and contents
   and local variables.
   If called on an already-initialized buffer, the list of overlays
   should be deleted before calling this function, otherwise we end up
   with overlays that claim to belong to the buffer but the buffer
   claims it doesn't belong to it.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset_buffer (b: *mut buffer)
{
  bset_filename (b, Qnil ());
  bset_file_truename (b, Qnil ());
  bset_directory (b, if ! (current_buffer! ()).is_null () { BVAR! ((current_buffer! ()), directory_) } else { Qnil () });
  (*b).modtime = make_timespec (0, UNKNOWN_MODTIME_NSECS as c_long);
  (*b).modtime_size = -1;
  XSETFASTINT! (BVAR! (b, save_length_), 0);
  (*b).last_window_start = 1;
  /* It is more conservative to start out "changed" than "unchanged".  */
  (*b).bits &= ! BUFFER_CLIP_CHANGED;
  (*b).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;
  (*b).bits &= ! BUFFER_LONG_LINE_OPTIMIZATIONS_P;
  bset_backed_up (b, Qnil ());
  bset_local_minor_modes (b, Qnil ());
  BUF_AUTOSAVE_MODIFF! (b) = 0;
  (*b).auto_save_failure_time = 0;
  bset_auto_save_file_name (b, Qnil ());
  bset_read_only (b, Qnil ());
  (*b).overlays = std::ptr::null_mut ();
  bset_mark_active (b, Qnil ());
  bset_point_before_scroll (b, Qnil ());
  bset_file_format (b, Qnil ());
  bset_auto_save_file_format (b, Qt ());
  bset_last_selected_window (b, Qnil ());
  bset_display_count (b, make_fixnum (0));
  bset_display_time (b, Qnil ());
  bset_enable_multibyte_characters
    (b, BVAR! (&raw mut buffer_defaults, enable_multibyte_characters_));
  bset_cursor_type (b, BVAR! (&raw mut buffer_defaults, cursor_type_));
  bset_extra_line_spacing (b, BVAR! (&raw mut buffer_defaults, extra_line_spacing_));
  // C 1120-1122 is `#ifdef HAVE_TREE_SITTER` (undef): `bset_ts_parser_list (b, Qnil);`
  // (the setter itself is dead too, C 239-245).

  (*b).display_error_modiff = 0;
}

/* Reset buffer B's local variables info.
   Don't use this on a buffer that has already been in use;
   it does not treat permanent locals consistently.
   Instead, use Fkill_all_local_variables.

   If PERMANENT_TOO, reset permanent buffer-local variables.
   If not, preserve those.  PERMANENT_TOO = 2 means ignore
   the permanent-local property of non-builtin variables.  */

unsafe fn reset_buffer_local_variables (b: *mut buffer, permanent_too: c_int)
{
  let mut offset: ptrdiff_t = 0;
  let mut i: c_int = 0;

  /* Reset the major mode to Fundamental, together with all the
     things that depend on the major mode.
     default-major-mode is handled at a higher level.
     We ignore it here.  */
  bset_major_mode (b, Qfundamental_mode ());
  bset_keymap (b, Qnil ());
  bset_mode_name (b, QSFundamental);

  /* If the standard case table has been altered and invalidated,
     fix up its insides first.  */
  if ! (CHAR_TABLE_P (FAM_ELEM! ((*XCHAR_TABLE (Vascii_downcase_table)).extras, 0))
	&& CHAR_TABLE_P (FAM_ELEM! ((*XCHAR_TABLE (Vascii_downcase_table)).extras, 1))
	&& CHAR_TABLE_P (FAM_ELEM! ((*XCHAR_TABLE (Vascii_downcase_table)).extras, 2)))
    { Fset_standard_case_table (Vascii_downcase_table); }

  bset_downcase_table (b, Vascii_downcase_table);
  bset_upcase_table (b, FAM_ELEM! ((*XCHAR_TABLE (Vascii_downcase_table)).extras, 0));
  bset_case_canon_table (b, FAM_ELEM! ((*XCHAR_TABLE (Vascii_downcase_table)).extras, 1));
  bset_case_eqv_table (b, FAM_ELEM! ((*XCHAR_TABLE (Vascii_downcase_table)).extras, 2));
  bset_invisibility_spec (b, Qt ());

  /* Reset all (or most) per-buffer variables to their defaults.  */
  if permanent_too == 1
    { bset_local_var_alist (b, Qnil ()); }
  else
    {
      let mut tmp: Lisp_Object = Qnil ();
      let mut last: Lisp_Object = Qnil ();
      let mut buffer: Lisp_Object = Qnil ();
      XSETBUFFER! (buffer, b);

      tmp = BVAR! (b, local_var_alist_);
      while CONSP (tmp)
	{
	  let local_var: Lisp_Object = XCAR (XCAR (tmp));
	  let prop: Lisp_Object = Fget (local_var, Qpermanent_local ());
	  let sym: Lisp_Object = local_var;

	  /* Watchers are run *before* modifying the var.  */
	  // C: `XSYMBOL (local_var)->u.s.trapped_write == SYMBOL_TRAPPED_WRITE` (the
	  // `trapped_write` family of token-aliases.tsv).
	  if ((*XSYMBOL (local_var)).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK)
	    >> SYMBOL_TRAPPED_WRITE_SHIFT == SYMBOL_TRAPPED_WRITE
	    { notify_variable_watchers (local_var, Qnil (),
					Qmakunbound (), Fcurrent_buffer ()); }

	  eassert! (((*XSYMBOL (sym)).u.s.bits & SYMBOL_REDIRECT_MASK)
		    >> SYMBOL_REDIRECT_SHIFT == SYMBOL_LOCALIZED);
	  /* Need not do anything if some other buffer's binding is
	     now cached.  */
	  let blv_ = SYMBOL_BLV (XSYMBOL (sym)) as *mut Lisp_Buffer_Local_Value;
	  if BASE_EQ ((*blv_).r#where,
		      buffer)
	    {
	      /* Symbol is set up for this buffer's old local value:
	         swap it out!  */
	      swap_in_global_binding (XSYMBOL (sym));
	    }

	  if ! NILP (prop) && permanent_too == 0
	    {
	      /* If permanent-local, keep it.  */
	      last = tmp;
	      if EQ (prop, Qpermanent_local_hook ())
		{
		  /* This is a partially permanent hook variable.
		     Preserve only the elements that want to be preserved.  */
		  let mut list: Lisp_Object = Qnil ();
		  let mut newlist: Lisp_Object = Qnil ();
		  list = XCDR (XCAR (tmp));
		  if ! CONSP (list)
		    { newlist = list; }
		  else
		    {
		      newlist = Qnil ();
		      while CONSP (list)
			{
			  let elt: Lisp_Object = XCAR (list);
			  /* Preserve element ELT if it's t, or if it is a
			     function with a `permanent-local-hook'
			     property. */
			  if EQ (elt, Qt ())
			    || (SYMBOLP (elt)
				&& ! NILP (Fget (elt, Qpermanent_local_hook ())))
			    { newlist = Fcons (elt, newlist); }
			  list = XCDR (list);
			}
		    }
		  newlist = Fnreverse (newlist);
		  if ((*XSYMBOL (local_var)).u.s.bits & SYMBOL_TRAPPED_WRITE_MASK)
		    >> SYMBOL_TRAPPED_WRITE_SHIFT == SYMBOL_TRAPPED_WRITE
		    { notify_variable_watchers (local_var, newlist,
						Qmakunbound (), Fcurrent_buffer ()); }
		  XSETCDR (XCAR (tmp), newlist);
		  tmp = XCDR (tmp);
		  continue; /* Don't do variable write trapping twice.  */
		}
	    }
	  /* Delete this local variable.  */
	  else if NILP (last)
	    { bset_local_var_alist (b, XCDR (tmp)); }
	  else
	    { XSETCDR (last, XCDR (tmp)); }
	  tmp = XCDR (tmp);
	}
    }

  while i < last_per_buffer_idx
    {
      if permanent_too != 0 || buffer_permanent_local_flags[i as usize] == 0
	{ SET_PER_BUFFER_VALUE_P (b, i, false); }
      i += 1;
    }

  /* For each slot that has a default value, copy that into the slot.  */
  FOR_EACH_PER_BUFFER_OBJECT_AT! (offset, {
      let idx: c_int = PER_BUFFER_IDX (offset);
      if idx > 0
	&& (permanent_too != 0
	    || buffer_permanent_local_flags[idx as usize] == 0)
	{ set_per_buffer_value (b, offset as c_int, per_buffer_default (offset as c_int)); }
    });
}

/* We split this away from generate-new-buffer, because rename-buffer
   and set-visited-file-name ought to be able to use this to really
   rename the buffer properly.  */

/* gnulib's INT_BUFSIZE_BOUND (intprops.h 108-114) is `INT_STRLEN_BOUND (t) + 1`, i.e.
   `(TYPE_WIDTH (t) - 1) * 146 / 485 + 1 + 1`; the <intprops.h> mirror does not exist yet
   (P-26), so the two instantiations this file uses are spelled here, as data.rs and
   json.rs spell theirs.  */
const INT_BUFSIZE_BOUND: usize = ((PTRDIFF_WIDTH - 1) * 146 + 484) / 485 + 1 + 1;
const INT_BUFSIZE_BOUND_int: usize = ((INT_WIDTH as usize - 1) * 146 + 484) / 485 + 1 + 1;
/* C: `enum { bug_57211 = true };  /* https://bugs.gnu.org/57211 */`.  */
const bug_57211: bool = true;

DEFUN! ("generate-new-buffer-name", Fgenerate_new_buffer_name,
       Sgenerate_new_buffer_name, 1, 2, 0,
       doc: "Return a string that is the name of no existing buffer based on NAME.\nIf there is no live buffer named NAME, then return NAME.\nOtherwise modify name by appending `<NUMBER>', incrementing NUMBER\n(starting at 2) until an unused name is found, and then return that name.\nOptional second argument IGNORE specifies a name that is okay to use (if\nit is in the sequence to be tried) even if a buffer with that name exists.\n\nIf NAME begins with a space (i.e., a buffer that is not normally\nvisible to users), then if buffer NAME already exists a random number\nis first appended to NAME, to speed up finding a non-existent buffer.",
       (name: Lisp_Object, ignore: Lisp_Object),
       {
  let mut genbase: Lisp_Object = Qnil ();

  CHECK_STRING (name);

  if (! NILP (ignore) && ! NILP (Fstring_equal (name, ignore)))
      || NILP (Fget_buffer (name))
    { return name; }

  if SREF (name, 0) != b' ' /* See bug#1229.  */
    { genbase = name; }
  else
    {
      /* C: `enum { bug_57211 = true };` (https://bugs.gnu.org/57211) and
	 `char number[bug_57211 ? INT_BUFSIZE_BOUND (int) + 1 : sizeof "-999999"];`  */
      const bug_57211: bool = true;
      const bug_57211_size: usize = if bug_57211 { INT_BUFSIZE_BOUND_int + 1 } else { 8 };
      let mut number: [c_char; bug_57211_size] = [0; bug_57211_size];
      let r: EMACS_INT = get_random ();
      eassume! (0 <= r);
      let i: c_int = (r % 1000000) as c_int;
      AUTO_STRING_WITH_LEN! (lnumber, number.as_mut_ptr (),
			     sprintf (number.as_mut_ptr (), c"-%d".as_ptr (), i)
			       as ptrdiff_t);
      genbase = concat2 (name, lnumber);
      if NILP (Fget_buffer (genbase))
	{ return genbase; }
    }

  let mut count: ptrdiff_t = 2;
  loop
    {
      let mut number: [c_char; INT_BUFSIZE_BOUND + 3] = [0; INT_BUFSIZE_BOUND + 3];
      AUTO_STRING_WITH_LEN! (lnumber, number.as_mut_ptr (),
			     sprintf (number.as_mut_ptr (), c"<%td>".as_ptr (),
				      count as ptrdiff_t) as ptrdiff_t);
      let gentemp: Lisp_Object = concat2 (genbase, lnumber);
      if ! NILP (Fstring_equal (gentemp, ignore))
	  || NILP (Fget_buffer (gentemp))
	{ return gentemp; }
      count += 1;
    }
});

DEFUN! ("buffer-name", Fbuffer_name, Sbuffer_name, 0, 1, 0,
       doc: "Return the name of BUFFER, as a string.\nBUFFER defaults to the current buffer.\nReturn nil if BUFFER has been killed.",
       (buffer: Lisp_Object),
       {
  return BVAR! (decode_buffer (buffer), name_);
});

DEFUN! ("buffer-last-name", Fbuffer_last_name, Sbuffer_last_name, 0, 1, 0,
       doc: "Return last name of BUFFER, as a string.\nBUFFER defaults to the current buffer.\n\nThis is the name BUFFER had before the last time it was renamed or\nimmediately before it was killed.",
       (buffer: Lisp_Object),
       {
  return BVAR! (decode_buffer (buffer), last_name_);
});

DEFUN! ("buffer-file-name", Fbuffer_file_name, Sbuffer_file_name, 0, 1, 0,
       doc: "Return absolute name of file BUFFER is visiting, or nil if none.\nNo argument or nil as argument means use the current buffer.",
       (buffer: Lisp_Object),
       {
  return BVAR! (decode_buffer (buffer), filename_);
});

DEFUN! ("buffer-base-buffer", Fbuffer_base_buffer, Sbuffer_base_buffer,
       0, 1, 0,
       doc: "Return the base buffer of indirect buffer BUFFER.\nIf BUFFER is not indirect, return nil.\nBUFFER defaults to the current buffer.",
       (buffer: Lisp_Object),
       {
  let base: *mut buffer = (*decode_buffer (buffer)).base_buffer;
  let mut buffer = buffer;   /* C's `XSETBUFFER (buffer, base)` rebinds the argument.  */
  return if ! base.is_null () { XSETBUFFER! (buffer, base); buffer } else { Qnil () };
});

DEFUN! ("buffer-local-value", Fbuffer_local_value,
       Sbuffer_local_value, 2, 2, 0,
       doc: "Return the value of VARIABLE in BUFFER.\nIf VARIABLE does not have a buffer-local binding in BUFFER, the value\nis the default binding of the variable.",
       (variable: Lisp_Object, buffer: Lisp_Object),
       {
  let result: Lisp_Object = buffer_local_value (variable, buffer);

  if BASE_EQ (result, Qunbound ())
    { xsignal1 (Qvoid_variable (), variable); }

  return result;
});

/* Like Fbuffer_local_value, but return Qunbound if the variable is
   locally unbound.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn buffer_local_value (variable: Lisp_Object, buffer: Lisp_Object)
  -> Lisp_Object
{
  let mut variable = variable;   /* C's `XSETSYMBOL (variable, sym)` rebinds it.  */
  let mut buf: *mut buffer;
  let mut result: Lisp_Object = Qnil ();
  let mut sym: *mut Lisp_Symbol;

  CHECK_SYMBOL (variable);
  CHECK_BUFFER (buffer);
  buf = XBUFFER (buffer);
  sym = XSYMBOL (variable);

  'start: loop
    {
      // C: `switch (sym->u.s.redirect)`
      match ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
	{
	  SYMBOL_VARALIAS => { sym = SYMBOL_ALIAS (sym); continue 'start; }
	  SYMBOL_PLAINVAL => { result = SYMBOL_VAL (sym); }
	  SYMBOL_LOCALIZED =>
	    { /* Look in local_var_alist.  */
	      let blv: *mut Lisp_Buffer_Local_Value
		= SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value;
	      XSETSYMBOL! (variable, sym); /* Update in case of aliasing.  */
	      result = assq_no_quit (variable, BVAR! (buf, local_var_alist_));
	      if ! NILP (result)
		{
		  if ! (*blv).fwd.is_null ()
		    { /* What binding is loaded right now?  */
		      let current_alist_element: Lisp_Object = (*blv).valcell;

		      /* The value of the currently loaded binding is not
			 stored in it, but rather in the realvalue slot.
			 Store that value into the binding it belongs to
			 in case that is the one we are about to use.  */

		      XSETCDR (current_alist_element,
			       do_symval_forwarding ((*blv).fwd));
		    }
		  /* Now get the (perhaps updated) value out of the binding.  */
		  result = XCDR (result);
		}
	      else
		{ result = Fdefault_value (variable); }
	    }
	  SYMBOL_FORWARDED =>
	    {
	      let fwd: lispfwd = SYMBOL_FWD (sym) as lispfwd;
	      if BUFFER_OBJFWDP (fwd)
		{ result = per_buffer_value (buf, XBUFFER_OFFSET (fwd) as c_int); }
	      else
		{ result = Fdefault_value (variable); }
	    }
	  _ => { emacs_abort (); }
	}
      break 'start;
    }

  return result;
}

/* Return an alist of the Lisp-level buffer-local bindings of
   buffer BUF.  That is, don't include the variables maintained
   in special slots in the buffer object.
   If not CLONE, replace elements of the form (VAR . unbound)
   by VAR.  */

unsafe fn buffer_lisp_local_variables (buf: *mut buffer, clone: bool) -> Lisp_Object
{
  let mut result: Lisp_Object = Qnil ();
  let mut tail: Lisp_Object = Qnil ();
  tail = BVAR! (buf, local_var_alist_);
  while CONSP (tail)
    {
      let mut val: Lisp_Object = Qnil ();
      let mut elt: Lisp_Object = Qnil ();

      elt = XCAR (tail);

      /* Reference each variable in the alist in buf.
	 If inquiring about the current buffer, this gets the current values,
	 so store them into the alist so the alist is up to date.
	 If inquiring about some other buffer, this swaps out any values
	 for that buffer, making the alist up to date automatically.  */
      val = find_symbol_value (XCAR (elt));
      /* Use the current buffer value only if buf is the current buffer.  */
      if buf != (current_buffer! ())
	{ val = XCDR (elt); }

      result = Fcons (if ! clone && BASE_EQ (val, Qunbound ())
		      { XCAR (elt) }
		      else { Fcons (XCAR (elt), val) },
		      result);
      tail = XCDR (tail);
    }

  return result;
}

/* If the variable at position index OFFSET in buffer BUF has a
   buffer-local value, return (name . value).  If SYM is non-nil,
   it replaces name.  */

unsafe fn buffer_local_variables_1 (buf: *mut buffer, offset: c_int, sym: Lisp_Object)
  -> Lisp_Object
{
  let mut sym = sym;   /* C reassigns `sym` below.  */
  let idx: c_int = PER_BUFFER_IDX (offset as ptrdiff_t);
  if (idx == -1 || PER_BUFFER_VALUE_P (buf, idx))
      && SYMBOLP (PER_BUFFER_SYMBOL! (offset))
    {
      sym = if NILP (sym) { PER_BUFFER_SYMBOL! (offset) } else { sym };
      let val: Lisp_Object = per_buffer_value (buf, offset);
      return if BASE_EQ (val, Qunbound ()) { sym } else { Fcons (sym, val) };
    }
  return Qnil ();
}

DEFUN! ("buffer-local-variables", Fbuffer_local_variables,
       Sbuffer_local_variables, 0, 1, 0,
       doc: "Return an alist of variables that are buffer-local in BUFFER.\nMost elements look like (SYMBOL . VALUE), describing one variable.\nFor a symbol that is locally unbound, just the symbol appears in the value.\nNote that storing new VALUEs in these elements doesn't change the variables.\nNo argument or nil as argument means use current buffer as BUFFER.",
       (buffer: Lisp_Object),
       {
  let buf: *mut buffer = decode_buffer (buffer);
  let mut result: Lisp_Object = buffer_lisp_local_variables (buf, false);
  let mut tem: Lisp_Object;

  /* Add on all the variables stored in special slots.  */
  {
    let mut offset: ptrdiff_t = 0;

    FOR_EACH_PER_BUFFER_OBJECT_AT! (offset, {
	tem = buffer_local_variables_1 (buf, offset as c_int, Qnil ());
	if ! NILP (tem)
	  { result = Fcons (tem, result); }
      });
  }

  tem = buffer_local_variables_1 (buf, PER_BUFFER_VAR_OFFSET! (undo_list_) as c_int,
				  Qbuffer_undo_list ());
  if ! NILP (tem)
    { result = Fcons (tem, result); }

  return result;
});

DEFUN! ("buffer-modified-p", Fbuffer_modified_p, Sbuffer_modified_p,
       0, 1, 0,
       doc: "Return non-nil if BUFFER was modified since its file was last read or saved.\nNo argument or nil as argument means use current buffer as BUFFER.\n\nIf BUFFER was autosaved since it was last modified, this function\nreturns the symbol `autosaved'.",
       (buffer: Lisp_Object),
       {
  let buf: *mut buffer = decode_buffer (buffer);
  if BUF_SAVE_MODIFF! (buf) < BUF_MODIFF! (buf)
    {
      if BUF_AUTOSAVE_MODIFF! (buf) == BUF_MODIFF! (buf)
	{ return Qautosaved (); }
      else
	{ return Qt (); }
    }
  else
    { return Qnil (); }
});

DEFUN! ("force-mode-line-update", Fforce_mode_line_update,
       Sforce_mode_line_update, 0, 1, 0,
       doc: "Force redisplay of the current buffer's mode line and header line.\nWith optional non-nil ALL, force redisplay of all mode lines, tab lines and\nheader lines.  This function also forces recomputation of the\nmenu bar menus and the frame title.",
       (all: Lisp_Object),
       {
  if ! NILP (all)
    {
      update_mode_lines = 10;
      /* FIXME: This can't be right.  */
      // C: `current_buffer->prevent_redisplay_optimizations_p = true;`
      (*(current_buffer! ())).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;
    }
  else if buffer_window_count ((current_buffer! ())) != 0
    {
      bset_update_mode_line ((current_buffer! ()) as *mut c_void);
      (*(current_buffer! ())).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;
    }
  return all;
});

DEFUN! ("set-buffer-modified-p", Fset_buffer_modified_p, Sset_buffer_modified_p,
       1, 1, 0,
       doc: "Mark current buffer as modified or unmodified according to FLAG.\nA non-nil FLAG means mark the buffer modified.\nIn addition, this function unconditionally forces redisplay of the\nmode lines of the windows that display the current buffer, and also\nlocks or unlocks the file visited by the buffer, depending on whether\nthe function's argument is non-nil, but only if both `buffer-file-name'\nand `buffer-file-truename' are non-nil.",
       (flag: Lisp_Object),
       {
  Frestore_buffer_modified_p (flag);

  /* Set update_mode_lines only if buffer is displayed in some window.
     Packages like jit-lock or lazy-lock preserve a buffer's modified
     state by recording/restoring the state around blocks of code.
     Setting update_mode_lines makes redisplay consider all windows
     (on all frames).  Stealth fontification of buffers not displayed
     would incur additional redisplay costs if we'd set
     update_modes_lines unconditionally.

     Ideally, I think there should be another mechanism for fontifying
     buffers without "modifying" buffers, or redisplay should be
     smarter about updating the `*' in mode lines.  --gerd  */
  return Fforce_mode_line_update (Qnil ());
});

DEFUN! ("restore-buffer-modified-p", Frestore_buffer_modified_p,
       Srestore_buffer_modified_p, 1, 1, 0,
       doc: "Like `set-buffer-modified-p', but doesn't redisplay buffer's mode line.\nA nil FLAG means to mark the buffer as unmodified.  A non-nil FLAG\nmeans mark the buffer as modified.  A special value of `autosaved'\nwill mark the buffer as modified and also as autosaved since it was\nlast modified.\n\nThis function also locks or unlocks the file visited by the buffer,\nif both `buffer-file-truename' and `buffer-file-name' are non-nil.\n\nIt is not ensured that mode lines will be updated to show the modified\nstate of the current buffer.  Use with care.",
       (flag: Lisp_Object),
       {
  // C 1560-1569 is the `restore-buffer-modified-p` body's opening; the `if
  // (!inhibit_modification_hooks)` block below is C 1565-1576.

  /* If buffer becoming modified, lock the file.
     If buffer becoming unmodified, unlock the file.  */

  let b: *mut buffer = if ! (*(current_buffer! ())).base_buffer.is_null ()
    { (*(current_buffer! ())).base_buffer }
    else { (current_buffer! ()) };

  if ! inhibit_modification_hooks
    {
      let fn_: Lisp_Object = BVAR! (b, file_truename_);
      /* Test buffer-file-name so that binding it to nil is effective.  */
      if ! NILP (fn_) && ! NILP (BVAR! (b, filename_))
	{
	  let already: bool = SAVE_MODIFF! () < MODIFF! ();
	  if ! already && ! NILP (flag)
	    { Flock_file (fn_); }
	  else if already && NILP (flag)
	    { Funlock_file (fn_); }
	}
    }
  /* Here we have a problem.  SAVE_MODIFF is used here to encode
     buffer-modified-p (as SAVE_MODIFF<MODIFF) as well as
     recent-auto-save-p (as SAVE_MODIFF<auto_save_modified).  So if we
     modify SAVE_MODIFF to affect one, we may affect the other
     as well.
     E.g. if FLAG is nil we need to set SAVE_MODIFF to MODIFF, but
     if SAVE_MODIFF<auto_save_modified that means we risk changing
     recent-auto-save-p from t to nil.
     Vice versa, if FLAG is non-nil and SAVE_MODIFF>=auto_save_modified
     we risk changing recent-auto-save-p from nil to t.  */
  if NILP (flag)
    /* This unavoidably sets recent-auto-save-p to nil.  */
    { SAVE_MODIFF! () = MODIFF! (); }
  else
    {
      /* If SAVE_MODIFF == auto_save_modified == MODIFF, we can either
	 decrease SAVE_MODIFF and auto_save_modified or increase
	 MODIFF.  */
      if SAVE_MODIFF! () >= MODIFF! ()
	{
	  /* C: `SAVE_MODIFF = modiff_incr (&MODIFF, 1);`.  The mirror's `modiff_incr`
	     returns `()` where C's returns the *old* value (`return a0;`), so the old value
	     is read here before the call (the mirror gap is reported in
	     docs/ports/buffer.md); the call's side effect is unchanged.  */
	  let a0: modiff_count = MODIFF! ();
	  modiff_incr (&raw mut MODIFF! (), 1);
	  SAVE_MODIFF! () = a0;
	}
      if EQ (flag, Qautosaved ())
	{ BUF_AUTOSAVE_MODIFF! (b) = MODIFF! (); }
    }
  return flag;
});

DEFUN! ("buffer-modified-tick", Fbuffer_modified_tick, Sbuffer_modified_tick,
       0, 1, 0,
       doc: "Return BUFFER's tick counter, incremented for each change in text.\nEach buffer has a tick counter which is incremented each time the\ntext in that buffer is changed.  No argument or nil as argument means\nuse current buffer as BUFFER.",
       (buffer: Lisp_Object),
       {
  return modiff_to_integer (BUF_MODIFF! (decode_buffer (buffer)));
});

DEFUN! ("internal--set-buffer-modified-tick",
       Finternal__set_buffer_modified_tick, Sinternal__set_buffer_modified_tick,
       1, 2, 0,
       doc: "Set BUFFER's tick counter to TICK.\nNo argument or nil as argument means use current buffer as BUFFER.",
       (tick: Lisp_Object, buffer: Lisp_Object),
       {
  CHECK_FIXNUM (tick);
  BUF_MODIFF! (decode_buffer (buffer)) = XFIXNUM (tick) as modiff_count;
  return Qnil ();
});

DEFUN! ("buffer-chars-modified-tick", Fbuffer_chars_modified_tick,
       Sbuffer_chars_modified_tick, 0, 1, 0,
       doc: "Return BUFFER's character-change tick counter.\nEach buffer has a character-change tick counter, which is set to the\nvalue of the buffer's tick counter (see `buffer-modified-tick'), each\ntime text in that buffer is inserted or deleted.  By comparing the\nvalues returned by two individual calls of `buffer-chars-modified-tick',\nyou can tell whether a character change occurred in that buffer in\nbetween these calls.  No argument or nil as argument means use current\nbuffer as BUFFER.",
       (buffer: Lisp_Object),
       {
  return modiff_to_integer (BUF_CHARS_MODIFF! (decode_buffer (buffer)));
});

DEFUN! ("rename-buffer", Frename_buffer, Srename_buffer, 1, 2,
       "(list (read-string \"Rename buffer (to new name): \" \
	      nil 'buffer-name-history (buffer-name (current-buffer))) \
	      current-prefix-arg)",
       doc: "Change current buffer's name to NEWNAME (a string).\nIf second arg UNIQUE is nil or omitted, it is an error if a\nbuffer named NEWNAME already exists.\nIf UNIQUE is non-nil, come up with a new name using\n`generate-new-buffer-name'.\nInteractively, you can set UNIQUE with a prefix argument.\nWe return the name we actually gave the buffer.\nThis does not change the name of the visited file (if any).",
       (newname: Lisp_Object, unique: Lisp_Object),
       {
  let mut tem: Lisp_Object = Qnil ();
  let mut buf: Lisp_Object = Qnil ();
  let oldname: Lisp_Object = BVAR! ((current_buffer! ()), name_);
  let requestedname: Lisp_Object = newname;
  let mut newname = newname;   /* C's `newname = Fgenerate_new_buffer_name` rebinds it.  */

  CHECK_STRING (newname);

  if SCHARS (newname) == 0
    { error (c"Empty string is invalid as a buffer name".as_ptr ()); }

  tem = Fget_buffer (newname);
  if ! NILP (tem)
    {
      /* Don't short-circuit if UNIQUE is t.  That is a useful way to
	 rename the buffer automatically so you can create another
	 with the original name.  It makes UNIQUE equivalent to
	 (rename-buffer (generate-new-buffer-name NEWNAME)).  */
      if NILP (unique) && XBUFFER (tem) == (current_buffer! ())
	{ return BVAR! ((current_buffer! ()), name_); }
      if ! NILP (unique)
	{ newname = Fgenerate_new_buffer_name (newname, oldname); }
      else
	{ error (c"Buffer name `%s' is in use".as_ptr (), SDATA (newname)); }
    }

  bset_last_name ((current_buffer! ()), oldname);
  bset_name ((current_buffer! ()), newname);

  /* Catch redisplay's attention.  Unless we do this, the mode lines for
     any windows displaying current_buffer will stay unchanged.  */
  bset_update_mode_line ((current_buffer! ()) as *mut c_void);

  XSETBUFFER! (buf, (current_buffer! ()));
  Fsetcar (Frassq (buf, Vbuffer_alist), newname);
  if NILP (BVAR! ((current_buffer! ()), filename_))
      && ! NILP (BVAR! ((current_buffer! ()), auto_save_file_name_))
    { call0 (Qrename_auto_save_file ()); }

  run_buffer_list_update_hook ((current_buffer! ()));

  calln! (Quniquify__rename_buffer_advice (), requestedname, unique);

  /* Refetch since that last call may have done GC.  */
  return BVAR! ((current_buffer! ()), name_);
});

/* True if B can be used as 'other-than-BUFFER' buffer.  */

unsafe fn candidate_buffer (b: Lisp_Object, buffer: Lisp_Object) -> bool
{
  return BUFFERP (b) && ! BASE_EQ (b, buffer)
    && BUFFER_LIVE_P (XBUFFER (b))
    && ! BUFFER_HIDDEN_P (XBUFFER (b));
}

DEFUN! ("other-buffer", Fother_buffer, Sother_buffer, 0, 3, 0,
       doc: "Return most recently selected buffer other than BUFFER.\nBuffers not visible in windows are preferred to visible buffers, unless\noptional second argument VISIBLE-OK is non-nil.  Ignore the argument\nBUFFER unless it denotes a live buffer.  If the optional third argument\nFRAME specifies a live frame, then use that frame's buffer list instead\nof the selected frame's buffer list.  Do not return a hidden buffer --\na buffer whose name starts with a space.\n\nThe buffer is found by scanning the selected or specified frame's buffer\nlist first, followed by the list of all buffers.  If no other buffer\nexists, return the buffer `*scratch*' (creating it if necessary).",
       (buffer: Lisp_Object, visible_ok: Lisp_Object, frame: Lisp_Object),
       {
  let f: *mut frame = decode_live_frame (frame) as *mut frame;
  let mut tail: Lisp_Object = (*f).buffer_list;
  let pred: Lisp_Object = (*f).buffer_predicate;
  let mut buf: Lisp_Object = Qnil ();
  let mut notsogood: Lisp_Object = Qnil ();

  /* Consider buffers that have been seen in the frame first.  */
  while CONSP (tail)
    {
      buf = XCAR (tail);
      if candidate_buffer (buf, buffer)
	  /* If the frame has a buffer_predicate, disregard buffers that
	     don't fit the predicate.  */
	  && (NILP (pred) || ! NILP (calln! (pred, buf)))
	{
	  if ! NILP (visible_ok)
	      || NILP (Fget_buffer_window (buf, Qvisible ()))
	    { return buf; }
	  else if NILP (notsogood)
	    { notsogood = buf; }
	}
      tail = XCDR (tail);
    }

  /* Consider alist of all buffers next.  */
  /* C: `FOR_EACH_LIVE_BUFFER (tail, buf)` (P-22: the expansion, as above).  */
  tail = Vbuffer_alist;
  while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }
    {
      if candidate_buffer (buf, buffer)
	  /* If the frame has a buffer_predicate, disregard buffers that
	     don't fit the predicate.  */
	  && (NILP (pred) || ! NILP (calln! (pred, buf)))
	{
	  if ! NILP (visible_ok)
	      || NILP (Fget_buffer_window (buf, Qvisible ()))
	    { return buf; }
	  else if NILP (notsogood)
	    { notsogood = buf; }
	}
      tail = XCDR (tail);
    }

  if ! NILP (notsogood)
    { return notsogood; }
  else
    { return safe_calln! (Qget_scratch_buffer_create ()); }
});

/* The following function is a safe variant of Fother_buffer: It doesn't
   pay attention to any frame-local buffer lists, doesn't care about
   visibility of buffers, and doesn't evaluate any frame predicates.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn other_buffer_safely (buffer: Lisp_Object) -> Lisp_Object
{
  let mut tail: Lisp_Object = Qnil ();
  let mut buf: Lisp_Object = Qnil ();

  /* C: `FOR_EACH_LIVE_BUFFER (tail, buf)` followed by a body without braces; the port
     writes the macro's expansion (P-22, docs/ports/excluded-macros.txt).  */
  tail = Vbuffer_alist;
  while CONSP (tail) && { buf = XCDR (XCAR (tail)); true }
    {
      if candidate_buffer (buf, buffer)
	{ return buf; }
      tail = XCDR (tail);
    }

  /* This function must return a valid buffer, since it is frequently
     our last line of defense in the face of the expected buffers
     becoming dead under our feet.  safe_call below could return nil
     if recreating *scratch* in Lisp, which does some fancy stuff,
     signals an error in some weird use case.  */
  buf = safe_calln! (Qget_scratch_buffer_create ());
  if NILP (buf)
    {
      AUTO_STRING! (scratch, c"*scratch*".as_ptr ());
      buf = Fget_buffer_create (scratch, Qnil ());
      Fset_buffer_major_mode (buf);
    }
  return buf;
}

DEFUN! ("buffer-enable-undo", Fbuffer_enable_undo, Sbuffer_enable_undo,
       0, 1, "",
       doc: "Start keeping undo information for buffer BUFFER.\nNo argument or nil as argument means do this for the current buffer.",
       (buffer: Lisp_Object),
       {
  let mut real_buffer: Lisp_Object = Qnil ();

  if NILP (buffer)
    { XSETBUFFER! (real_buffer, (current_buffer! ())); }
  else
    {
      real_buffer = Fget_buffer (buffer);
      if NILP (real_buffer)
	{ nsberror (buffer); }
    }

  if EQ (BVAR! (XBUFFER (real_buffer), undo_list_), Qt ())
    { bset_undo_list (XBUFFER (real_buffer), Qnil ()); }

  return Qnil ();
});

/* Truncate undo list and shrink the gap of BUFFER.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compact_buffer (buffer: *mut buffer)
{
  BUFFER_CHECK_INDIRECTION (buffer);

  /* Skip dead buffers, indirect buffers and buffers
     which aren't changed since last compaction.  */
  if BUFFER_LIVE_P (buffer)
      && (*buffer).base_buffer.is_null ()
      && BUF_COMPACT! (buffer) != BUF_MODIFF! (buffer)
    {
      /* If a buffer's undo list is Qt, that means that undo is
	 turned off in that buffer.  Calling truncate_undo_list on
	 Qt tends to return NULL, which effectively turns undo back on.
	 So don't call truncate_undo_list if undo_list is Qt.  */
      if ! EQ (BVAR! (buffer, undo_list_), Qt ())
	{ truncate_undo_list (buffer); }

      /* Shrink buffer gaps.  */
      if (*(*buffer).text).bits & BUFFER_TEXT_INHIBIT_SHRINKING == 0
	{
	  /* If a buffer's gap size is more than 10% of the buffer
	     size, or larger than GAP_BYTES_DFL bytes, then shrink it
	     accordingly.  Keep a minimum size of GAP_BYTES_MIN bytes.  */
	  let size: ptrdiff_t = clip_to_bounds (GAP_BYTES_MIN as ptrdiff_t,
						BUF_Z_BYTE! (buffer) / 10,
						GAP_BYTES_DFL as ptrdiff_t);
	  if BUF_GAP_SIZE! (buffer) > size
	    { make_gap_1 (buffer, -(BUF_GAP_SIZE! (buffer) - size)); }
	}
      BUF_COMPACT! (buffer) = BUF_MODIFF! (buffer);
    }
}

DEFUN! ("kill-buffer", Fkill_buffer, Skill_buffer, 0, 1, "bKill buffer: ",
       doc: "Kill the buffer specified by BUFFER-OR-NAME.\nThe argument may be a buffer or the name of an existing buffer.\nArgument nil or omitted means kill the current buffer.  Return t if the\nbuffer is actually killed, nil otherwise.\n\nThe functions in `kill-buffer-query-functions' are called with the\nbuffer to be killed as the current buffer.  If any of them returns nil,\nthe buffer is not killed.  The hook `kill-buffer-hook' is run before the\nbuffer is actually killed.  The buffer being killed will be current\nwhile the hook is running.  Functions called by any of these hooks are\nsupposed to not change the current buffer.  Neither hook is run for\ninternal or temporary buffers created by `get-buffer-create' or\n`generate-new-buffer' with argument INHIBIT-BUFFER-HOOKS non-nil.\n\nAny processes that have this buffer as the `process-buffer' are killed\nwith SIGHUP.  This function calls `replace-buffer-in-windows' for\ncleaning up all windows currently displaying the buffer to be killed.",
       (buffer_or_name: Lisp_Object),
       {
  let mut buffer: Lisp_Object = Qnil ();
  let mut b: *mut buffer;
  let mut tem: Lisp_Object = Qnil ();
  let mut m: *mut Lisp_Marker;

  if NILP (buffer_or_name)
    { buffer = Fcurrent_buffer (); }
  else
    { buffer = Fget_buffer (buffer_or_name); }
  if NILP (buffer)
    { nsberror (buffer_or_name); }

  b = XBUFFER (buffer);

  /* Avoid trouble for buffer already dead.  */
  if ! BUFFER_LIVE_P (b)
    { return Qnil (); }

  if thread_check_current_buffer (b as *mut c_void)
    { return Qnil (); }

  /* Run hooks with the buffer to be killed as the current buffer.  */
  {
    let count: specpdl_ref = SPECPDL_INDEX ();
    let mut modified: bool;

    record_unwind_protect_excursion ();
    set_buffer_internal (b);

    /* First run the query functions; if any query is answered no,
       don't kill the buffer.  */
    if (*b).bits & BUFFER_INHIBIT_BUFFER_HOOKS == 0
      {
	tem = CALLN! (Frun_hook_with_args_until_failure,
		      Qkill_buffer_query_functions ());
	if NILP (tem)
	  { return unbind_to (count, Qnil ()); }
      }

    /* Is this a modified buffer that's visiting a file? */
    modified = ! NILP (BVAR! (b, filename_))
      && BUF_MODIFF! (b) > BUF_SAVE_MODIFF! (b);

    /* Query if the buffer is still modified.  */
    if INTERACTIVE! () && modified
      {
	/* Ask whether to kill the buffer, and exit if the user says
	   "no".  */
	if NILP (calln! (Qkill_buffer__possibly_save (), buffer))
	  { return unbind_to (count, Qnil ()); }
	/* Recheck modified.  */
	modified = BUF_MODIFF! (b) > BUF_SAVE_MODIFF! (b);
      }

    /* Delete the autosave file, if requested. */
    if modified
	&& kill_buffer_delete_auto_save_files
	&& delete_auto_save_files
	&& ! NILP (Frecent_auto_save_p ())
	&& STRINGP (BVAR! (b, auto_save_file_name_))
	&& ! NILP (Ffile_exists_p (BVAR! (b, auto_save_file_name_)))
	/* If `auto-save-visited-mode' is on, then we're auto-saving
	   to the visited file -- don't delete it.. */
	&& NILP (Fstring_equal (BVAR! (b, auto_save_file_name_),
				BVAR! (b, filename_)))
      {
	tem = do_yes_or_no_p (build_string (c"Delete auto-save file? ".as_ptr ()));
	if ! NILP (tem)
	  { call0 (Qdelete_auto_save_file_if_necessary ()); }
      }

    /* If the hooks have killed the buffer, exit now.  */
    if ! BUFFER_LIVE_P (b)
      { return unbind_to (count, Qt ()); }

    /* Then run the hooks.  */
    if (*b).bits & BUFFER_INHIBIT_BUFFER_HOOKS == 0
      { run_hook (Qkill_buffer_hook ()); }
    unbind_to (count, Qnil ());
  }

  /* If the hooks have killed the buffer, exit now.  */
  if ! BUFFER_LIVE_P (b)
    { return Qt (); }

  /* We have no more questions to ask.  Verify that it is valid
     to kill the buffer.  This must be done after the questions
     since anything can happen within do_yes_or_no_p.  */

  /* Don't kill the minibuffer now current.  */
  if BASE_EQ (buffer, (*XWINDOW (minibuf_window)).contents)
    { return Qnil (); }

  /* When we kill an ordinary buffer which shares its buffer text
     with indirect buffer(s), we must kill indirect buffer(s) too.
     We do it at this stage so nothing terrible happens if they
     ask questions or their hooks get errors.  */
  if (*b).base_buffer.is_null () && (*b).indirections > 0
    {
      let mut tail: Lisp_Object = Qnil ();
      let mut other: Lisp_Object = Qnil ();

      /* C: `FOR_EACH_LIVE_BUFFER (tail, other)` (P-22: the expansion).  */
      tail = Vbuffer_alist;
      while CONSP (tail) && { other = XCDR (XCAR (tail)); true }
	{
	  let obuf: *mut buffer = XBUFFER (other);
	  if (*obuf).base_buffer == b
	    {
	      Fkill_buffer (other);
	      if BUFFER_LIVE_P (obuf)
		{ error (c"Unable to kill buffer whose indirect buffer `%s' cannot be killed".as_ptr (),
			 SDATA (BVAR! (obuf, name_))); }
	    }
	  tail = XCDR (tail);
	}

      /* Exit if we now have killed the base buffer (Bug#11665).  */
      if ! BUFFER_LIVE_P (b)
	{ return Qt (); }
    }

  /* Run replace_buffer_in_windows before making another buffer current
     since set-window-buffer-start-and-point will refuse to make another
     buffer current if the selected window does not show the current
     buffer (bug#10114).  */
  replace_buffer_in_windows (buffer);

  /* For dead windows that have not been collected yet, remove this
     buffer from those windows' lists of previously and next shown
     buffers and remove any 'quit-restore' or 'quit-restore-prev'
     parameters mentioning the buffer.  */
  if XFIXNUM (BVAR! (b, display_count_)) > 0
    { window_discard_buffer_from_dead_windows (buffer); }

  /* Exit if replacing the buffer in windows has killed our buffer.  */
  if ! BUFFER_LIVE_P (b)
    { return Qt (); }

  /* Make this buffer not be current.  Exit if it is the sole visible
     buffer.  */
  if b == (current_buffer! ())
    {
      tem = Fother_buffer (buffer, Qnil (), Qnil ());
      Fset_buffer (tem);
      if b == (current_buffer! ())
	{ return Qnil (); }
    }

  /* Check all threads again, in case a hook changed something.  */
  if thread_check_current_buffer (b as *mut c_void)
    { return Qnil (); }

  /* Clean up references to the buffer in threads.  */
  thread_all_before_buffer_killed (buffer);

  /* If the buffer now current is shown in the minibuffer and our buffer
     is the sole other buffer give up.  */
  XSETBUFFER! (tem, (current_buffer! ()));
  if BASE_EQ (tem, (*XWINDOW (minibuf_window)).contents)
      && BASE_EQ (buffer, Fother_buffer (buffer, Qnil (), Qnil ()))
    { return Qnil (); }

  /* Now there is no question: we can kill the buffer.  */

  /* Unlock this buffer's file, if it is locked.  */
  unlock_buffer (b);

  kill_buffer_processes (buffer);
  kill_buffer_xwidgets (buffer);

  /* Killing buffer processes may run sentinels which may have killed
     our buffer.  */
  if ! BUFFER_LIVE_P (b)
    { return Qt (); }

  /* These may run Lisp code and into infinite loops (if someone
     insisted on circular lists) so allow quitting here.  */
  frames_discard_buffer (buffer);

  clear_charpos_cache (b);

  tem = Vinhibit_quit;
  Vinhibit_quit = Qt ();
  /* Once the buffer is removed from Vbuffer_alist, its undo_list field is
     not traced by the GC in the same way.  So set it to nil early.  */
  bset_undo_list (b, Qnil ());
  /* Remove the buffer from the list of all buffers.  */
  Vbuffer_alist = Fdelq (Frassq (buffer, Vbuffer_alist), Vbuffer_alist);
  /* If replace_buffer_in_windows didn't do its job fix that now.  */
  replace_buffer_in_windows_safely (buffer);
  Vinhibit_quit = tem;

  if ! (*b).base_buffer.is_null ()
    {
      let mut i: INTERVAL;
      /* Unchain all markers that belong to this indirect buffer.
	 Don't unchain the markers that belong to the base buffer
	 or its other indirect buffers.  */
      let mut mp: *mut *mut Lisp_Marker = &raw mut BUF_MARKERS! (b);
      loop
	{
	  m = *mp;
	  if m.is_null () { break; }
	  if (*m).buffer as *mut buffer == b
	    {
	      (*m).buffer = std::ptr::null_mut ();
	      *mp = (*m).next;
	    }
	  else
	    { mp = &raw mut (*m).next; }
	}
      /* Intervals should be owned by the base buffer (Bug#16502).  */
      i = buffer_intervals (b);
      if ! i.is_null ()
	{
	  let mut owner: Lisp_Object = Qnil ();
	  XSETBUFFER! (owner, (*b).base_buffer);
	  set_interval_object (i, owner);
	}
    }
  else
    {
      /* Unchain all markers of this buffer and its indirect buffers.
	 and leave them pointing nowhere.  */
      m = BUF_MARKERS! (b);
      while ! m.is_null ()
	{
	  let next: *mut Lisp_Marker = (*m).next;
	  (*m).buffer = std::ptr::null_mut ();
	  (*m).next = std::ptr::null_mut ();
	  m = next;
	}
      BUF_MARKERS! (b) = std::ptr::null_mut ();
      set_buffer_intervals (b, std::ptr::null_mut ());

      /* Perhaps we should explicitly free the interval tree here...  */
    }
  delete_all_overlays (b);
  free_buffer_overlays (b);

  /* Reset the local variables, so that this buffer's local values
     won't be protected from GC.  They would be protected
     if they happened to remain cached in their symbols.
     This gets rid of them for certain.  */
  reset_buffer_local_variables (b, 1);

  bset_last_name (b, BVAR! (b, name_));
  bset_name (b, Qnil ());

  block_input ();
  if ! (*b).base_buffer.is_null ()
    {
      /* Notify our base buffer that we don't share the text anymore.  */
      eassert! ((*b).indirections == -1);
      (*(*b).base_buffer).indirections -= 1;
      eassert! ((*(*b).base_buffer).indirections >= 0);
      /* Make sure that we wasn't confused.  */
      eassert! ((*b).window_count == -1);
    }
  else
    {
      /* Make sure that no one shows us.  */
      eassert! ((*b).window_count == 0);
      /* No one shares our buffer text, can free it.  */
      free_buffer_text (b);
    }

  if ! (*b).newline_cache.is_null ()
    {
      free_region_cache ((*b).newline_cache);
      (*b).newline_cache = std::ptr::null_mut ();
    }
  if ! (*b).width_run_cache.is_null ()
    {
      free_region_cache ((*b).width_run_cache);
      (*b).width_run_cache = std::ptr::null_mut ();
    }
  if ! (*b).bidi_paragraph_cache.is_null ()
    {
      free_region_cache ((*b).bidi_paragraph_cache);
      (*b).bidi_paragraph_cache = std::ptr::null_mut ();
    }
  bset_width_table (b, Qnil ());
  unblock_input ();

  run_buffer_list_update_hook (b);

  return Qt ();
});

/* Move association for BUFFER to the front of buffer (a)lists.  Since
   we do this each time BUFFER is selected visibly, the more recently
   selected buffers are always closer to the front of those lists.  This
   means that other_buffer is more likely to choose a relevant buffer.

   Note that this moves BUFFER to the front of the buffer lists of the
   selected frame even if BUFFER is not shown there.  If BUFFER is not
   shown in the selected frame, consider the present behavior a feature.
   `select-window' gets this right since it shows BUFFER in the selected
   window when calling us.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn record_buffer (buffer: Lisp_Object)
{
  let mut aelt: Lisp_Object = Qnil ();
  let mut aelt_cons: Lisp_Object = Qnil ();
  let mut tem: Lisp_Object = Qnil ();
  let f: *mut frame = XFRAME (selected_frame);

  CHECK_BUFFER (buffer);

  /* Update Vbuffer_alist (we know that it has an entry for BUFFER).
     Don't allow quitting since this might leave the buffer list in an
     inconsistent state.  */
  tem = Vinhibit_quit;
  Vinhibit_quit = Qt ();
  aelt = Frassq (buffer, Vbuffer_alist);
  aelt_cons = Fmemq (aelt, Vbuffer_alist);
  Vbuffer_alist = Fdelq (aelt, Vbuffer_alist);
  XSETCDR (aelt_cons, Vbuffer_alist);
  Vbuffer_alist = aelt_cons;
  Vinhibit_quit = tem;

  /* Update buffer list of selected frame.  */
  fset_buffer_list (f, Fcons (buffer, Fdelq (buffer, (*f).buffer_list)));
  fset_buried_buffer_list (f, Fdelq (buffer, (*f).buried_buffer_list));

  run_buffer_list_update_hook (XBUFFER (buffer));
}

/* Move BUFFER to the end of the buffer (a)lists.  Do nothing if the
   buffer is killed.  For the selected frame's buffer list this moves
   BUFFER to its end even if it was never shown in that frame.  If
   this happens we have a feature, hence `bury-buffer-internal' should be
   called only when BUFFER was shown in the selected frame.  */

DEFUN! ("bury-buffer-internal", Fbury_buffer_internal, Sbury_buffer_internal,
       1, 1, 0,
       doc: "Move BUFFER to the end of the buffer list.",
       (buffer: Lisp_Object),
       {
  let mut aelt: Lisp_Object = Qnil ();
  let mut aelt_cons: Lisp_Object = Qnil ();
  let mut tem: Lisp_Object = Qnil ();
  let f: *mut frame = XFRAME (selected_frame);

  CHECK_BUFFER (buffer);

  /* Update Vbuffer_alist (we know that it has an entry for BUFFER).
     Don't allow quitting since this might leave the buffer list in an
     inconsistent state.  */
  tem = Vinhibit_quit;
  Vinhibit_quit = Qt ();
  aelt = Frassq (buffer, Vbuffer_alist);
  aelt_cons = Fmemq (aelt, Vbuffer_alist);
  Vbuffer_alist = Fdelq (aelt, Vbuffer_alist);
  XSETCDR (aelt_cons, Qnil ());
  Vbuffer_alist = nconc2 (Vbuffer_alist, aelt_cons);
  Vinhibit_quit = tem;

  /* Update buffer lists of selected frame.  */
  fset_buffer_list (f, Fdelq (buffer, (*f).buffer_list));
  fset_buried_buffer_list
    (f, Fcons (buffer, Fdelq (buffer, (*f).buried_buffer_list)));

  run_buffer_list_update_hook (XBUFFER (buffer));

  return Qnil ();
});

DEFUN! ("set-buffer-major-mode", Fset_buffer_major_mode, Sset_buffer_major_mode, 1, 1, 0,
       doc: "Set an appropriate major mode for BUFFER.\nFor the *scratch* buffer, use `initial-major-mode', otherwise choose a mode\naccording to the default value of `major-mode'.\nUse this function before selecting the buffer, since it may need to inspect\nthe current buffer's major mode.",
       (buffer: Lisp_Object),
       {
  let mut function: Lisp_Object = Qnil ();

  CHECK_BUFFER (buffer);

  if ! BUFFER_LIVE_P (XBUFFER (buffer))
    { error (c"Attempt to set major mode for a dead buffer".as_ptr ()); }

  if strcmp (SSDATA (BVAR! (XBUFFER (buffer), name_)) as *const c_char,
	     c"*scratch*".as_ptr ()) == 0
    { function = find_symbol_value (Qinitial_major_mode ()); }
  else
    {
      function = BVAR! (&raw mut buffer_defaults, major_mode_);
      if NILP (function)
	  && NILP (Fget (BVAR! ((current_buffer! ()), major_mode_), Qmode_class ()))
	{ function = BVAR! ((current_buffer! ()), major_mode_); }
    }

  if NILP (function) /* If function is `fundamental-mode', allow it to run
                        so that `run-mode-hooks' and thus
                        `hack-local-variables' get run. */
    { return Qnil (); }

  let count: specpdl_ref = SPECPDL_INDEX ();

  /* To select a nonfundamental mode,
     select the buffer temporarily and then call the mode function.  */

  record_unwind_current_buffer ();

  Fset_buffer (buffer);
  call0 (function);

  return unbind_to (count, Qnil ());
});

DEFUN! ("current-buffer", Fcurrent_buffer, Scurrent_buffer, 0, 0, 0,
       doc: "Return the current buffer as a Lisp object.",
       (),
       {
  let mut buf: Lisp_Object = Qnil ();
  XSETBUFFER! (buf, (current_buffer! ()));
  return buf;
});

/* Set the current buffer to B, and do not set windows_or_buffers_changed.
   This is used by redisplay.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_buffer_internal_1 (b: *mut buffer)
{
  // C 2331-2334 is `#ifdef USE_MMAP_FOR_BUFFERS` (undef): `if (b->text->beg == NULL)
  // enlarge_buffer_text (b, 0);`.

  if (current_buffer! ()) == b
    { return; }

  set_buffer_internal_2 (b);
}

/* Like set_buffer_internal_1, but doesn't check whether B is already
   the current buffer.  Called upon switch of the current thread, see
   post_acquire_global_lock.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_buffer_internal_2 (b: *mut buffer)
{
  let mut old_buf: *mut buffer;
  let mut tail: Lisp_Object = Qnil ();
  let mut b = b;

  BUFFER_CHECK_INDIRECTION (b);

  old_buf = (current_buffer! ());
  set_current_buffer! (b);
  last_known_column_point = -1;   /* Invalidate indentation cache.  */

  if ! old_buf.is_null ()
    {
      /* Put the undo list back in the base buffer, so that it appears
	 that an indirect buffer shares the undo list of its base.  */
      if ! (*old_buf).base_buffer.is_null ()
	{ bset_undo_list ((*old_buf).base_buffer, BVAR! (old_buf, undo_list_)); }

      /* If the old current buffer has markers to record PT, BEGV and ZV
	 when it is not current, update them now.  */
      record_buffer_markers (old_buf);
    }

  /* Get the undo list from the base buffer, so that it appears
     that an indirect buffer shares the undo list of its base.  */
  if ! (*b).base_buffer.is_null ()
    { bset_undo_list (b, BVAR! ((*b).base_buffer, undo_list_)); }

  /* If the new current buffer has markers to record PT, BEGV and ZV
     when it is not current, fetch them now.  */
  fetch_buffer_markers (b);

  /* Look down buffer's list of local Lisp variables
     to find and update any that forward into C variables.  */

  loop
    {
      tail = BVAR! (b, local_var_alist_);
      while CONSP (tail)
	{
	  let var: Lisp_Object = XCAR (XCAR (tail));
	  let sym: *mut Lisp_Symbol = XSYMBOL (var);
	  if ((*sym).u.s.bits & SYMBOL_REDIRECT_MASK) >> SYMBOL_REDIRECT_SHIFT
	      == SYMBOL_LOCALIZED /* Just to be sure.  */
	      && ! (*(SYMBOL_BLV (sym) as *mut Lisp_Buffer_Local_Value)).fwd.is_null ()
	    /* Just reference the variable
	       to cause it to become set for this buffer.  */
	    { Fsymbol_value (var); }
	  tail = XCDR (tail);
	}
      /* Do the same with any others that were local to the previous buffer */
      if ! (b != old_buf) { break; }
      b = old_buf;
      if ! (true) { break; }
    }
}

/* Switch to buffer B temporarily for redisplay purposes.
   This avoids certain things that don't need to be done within redisplay.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_buffer_temp (b: *mut buffer)
{
  let mut old_buf: *mut buffer;

  if (current_buffer! ()) == b
    { return; }

  old_buf = (current_buffer! ());
  set_current_buffer! (b);

  /* If the old current buffer has markers to record PT, BEGV and ZV
     when it is not current, update them now.  */
  record_buffer_markers (old_buf);

  /* If the new current buffer has markers to record PT, BEGV and ZV
     when it is not current, fetch them now.  */
  fetch_buffer_markers (b);
}

DEFUN! ("set-buffer", Fset_buffer, Sset_buffer, 1, 1, 0,
       doc: "Make buffer BUFFER-OR-NAME current for editing operations.\nBUFFER-OR-NAME may be a buffer or the name of an existing buffer.\nSee also `with-current-buffer' when you want to make a buffer current\ntemporarily.  This function does not display the buffer, so its effect\nends when the current command terminates.  Use `switch-to-buffer' or\n`pop-to-buffer' to switch buffers permanently.\nThe return value is the buffer made current.",
       (buffer_or_name: Lisp_Object),
       {
  let mut buffer: Lisp_Object = Qnil ();
  buffer = Fget_buffer (buffer_or_name);
  if NILP (buffer)
    { nsberror (buffer_or_name); }
  if ! BUFFER_LIVE_P (XBUFFER (buffer))
    { error (c"Selecting deleted buffer".as_ptr ()); }
  set_buffer_internal (XBUFFER (buffer));
  return buffer;
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore_buffer (buffer_or_name: Lisp_Object)
{
  Fset_buffer (buffer_or_name);
}

/* Set the current buffer to BUFFER provided if it is alive.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_buffer_if_live (buffer: Lisp_Object)
{
  if BUFFER_LIVE_P (XBUFFER (buffer))
    { set_buffer_internal (XBUFFER (buffer)); }
}

DEFUN! ("barf-if-buffer-read-only", Fbarf_if_buffer_read_only,
				   Sbarf_if_buffer_read_only, 0, 1, 0,
       doc: "Signal a `buffer-read-only' error if the current buffer is read-only.\nIf the text under POSITION (which defaults to point) has the\n`inhibit-read-only' text property set, the error will not be raised.",
       (position: Lisp_Object),
       {
  let mut position = position;   /* C's `XSETFASTINT (position, PT)` rebinds it.  */
  if NILP (position)
    { XSETFASTINT! (position, PT! ()); }
  else
    { CHECK_FIXNUM (position); }

  if ! NILP (BVAR! ((current_buffer! ()), read_only_))
      && NILP (Vinhibit_read_only)
      && NILP (Fget_text_property (position, Qinhibit_read_only (), Qnil ()))
    { xsignal1 (Qbuffer_read_only (), Fcurrent_buffer ()); }
  return Qnil ();
});

DEFUN! ("erase-buffer", Ferase_buffer, Serase_buffer, 0, 0, "*",
       doc: "Delete the entire contents of the current buffer.\nAny narrowing restriction in effect (see `narrow-to-region') is removed,\nso the buffer is truly empty after this.",
       (),
       {
  labeled_restrictions_remove_in_current_buffer ();
  Fwiden ();

  del_range (BEG as ptrdiff_t, Z! ());

  (*(current_buffer! ())).last_window_start = 1;
  /* Prevent warnings, or suspension of auto saving, that would happen
     if future size is less than past size.  Use of erase-buffer
     implies that the future text is not really related to the past text.  */
  XSETFASTINT! (BVAR! ((current_buffer! ()), save_length_), 0);
  return Qnil ();
});

#[unsafe(no_mangle)]
pub unsafe extern "C" fn validate_region (b: *mut Lisp_Object, e: *mut Lisp_Object)
{
  let mut beg: EMACS_INT = fix_position (*b);
  let mut end: EMACS_INT = fix_position (*e);

  if end < beg
    {
      let tem: EMACS_INT = beg;
      beg = end;
      end = tem;
    }

  if ! (BEGV! () <= beg && end <= ZV! ())
    { args_out_of_range_3 (Fcurrent_buffer (), *b, *e); }

  *b = make_fixnum (beg);
  *e = make_fixnum (end);
}

/* Advance BYTE_POS up to a character boundary
   and return the adjusted position.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn advance_to_char_boundary (byte_pos: ptrdiff_t) -> ptrdiff_t
{
  let mut c: c_int;
  let mut byte_pos = byte_pos;

  if byte_pos == BEG as ptrdiff_t
    /* Beginning of buffer is always a character boundary.  */
    { return BEG as ptrdiff_t; }

  c = FETCH_BYTE! (byte_pos) as c_int;
  if ! CHAR_HEAD_P (c)
    {
      /* We should advance BYTE_POS only when C is a constituent of a
         multibyte sequence.  */
      let orig_byte_pos: ptrdiff_t = byte_pos;

      loop
	{
	  byte_pos -= 1;
	  c = FETCH_BYTE! (byte_pos) as c_int;
	  if CHAR_HEAD_P (c) || byte_pos <= BEG as ptrdiff_t { break; }
	}
      byte_pos += next_char_len (byte_pos) as ptrdiff_t;
      if byte_pos < orig_byte_pos
	{ byte_pos = orig_byte_pos; }
      /* If C is a constituent of a multibyte sequence, BYTE_POS was
         surely advance to the correct character boundary.  If C is
         not, BYTE_POS was unchanged.  */
    }

  return byte_pos;
}

unsafe fn swap_buffer_overlays (buffer: *mut buffer, other: *mut buffer)
{
  let mut node: *mut itree_node;

  /* itree.h 166-176's `ITREE_FOREACH (node, buffer->overlays, PTRDIFF_MIN, PTRDIFF_MAX,
     ASCENDING)`, then the same over `other` (P-33, status 3.7).  */
  if (*buffer).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*buffer).overlays,
				PTRDIFF_MIN, PTRDIFF_MAX, ITREE_ASCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{ (*XOVERLAY ((*node).data)).buffer = other as *mut c_void; }
    }

  if (*other).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*other).overlays,
				PTRDIFF_MIN, PTRDIFF_MAX, ITREE_ASCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{ (*XOVERLAY ((*node).data)).buffer = buffer as *mut c_void; }
    }

  /* Swap the interval trees. */
  let tmp: *mut itree_tree = (*buffer).overlays;
  (*buffer).overlays = (*other).overlays;
  (*other).overlays = tmp;
}

DEFUN! ("buffer-swap-text", Fbuffer_swap_text, Sbuffer_swap_text,
       1, 1, 0,
       doc: "Swap the text between current buffer and BUFFER.\nUsing this function from `save-excursion' might produce surprising\nresults, see Info node `(elisp)Swapping Text'.",
       (buffer: Lisp_Object),
       {
  let mut other_buffer: *mut buffer;
  CHECK_BUFFER (buffer);
  other_buffer = XBUFFER (buffer);

  if ! BUFFER_LIVE_P (other_buffer)
    { error (c"Cannot swap a dead buffer's text".as_ptr ()); }

  /* Actually, it probably works just fine.
   * if (other_buffer == current_buffer)
   *   error ("Cannot swap a buffer's text with itself"); */

  /* Actually, this may be workable as well, tho probably only if they're
     *both* indirect.  */
  if ! (*other_buffer).base_buffer.is_null ()
      || ! (*(current_buffer! ())).base_buffer.is_null ()
    { error (c"Cannot swap indirect buffers's text".as_ptr ()); }

  { /* This is probably harder to make work.  */
    let mut tail: Lisp_Object = Qnil ();
    let mut other: Lisp_Object = Qnil ();
    /* C: `FOR_EACH_LIVE_BUFFER (tail, other) if (...) error (...);` -- the loop macro
       (P-22) with C's brace-less body, written out.  */
    tail = Vbuffer_alist;
    while CONSP (tail) && { other = XCDR (XCAR (tail)); true }
      {
	if (*XBUFFER (other)).base_buffer == other_buffer
	    || (*XBUFFER (other)).base_buffer == (current_buffer! ())
	  { error (c"One of the buffers to swap has indirect buffers".as_ptr ()); }
	tail = XCDR (tail);
      }
  }

  /* C's `#define swapfield(field, type)` and `#define swapfield_(field, type)` paste
     `tmp##field`/`bset_##field`, which `macro_rules!` cannot do; the port's two macros take
     the same three statements with the pasted names passed in (PORTING.md 0.1-12/P-37).
     The C bodies' type names (`struct buffer_text`, `ptrdiff_t`, `struct region_cache *`,
     `bool_bf`) are inferred in Rust; docs/ports/buffer.md records the deviation.  */
  macro_rules! swapfield {
    ($tmp:ident, $other_buffer:expr, $current_buffer:expr, $field:ident, $type:ty) => {{
      let $tmp: $type = (*$other_buffer).$field;
      (*$other_buffer).$field = (*$current_buffer).$field;
      (*$current_buffer).$field = $tmp;
    }};
  }
  macro_rules! swapfield_ {
    ($tmp:ident, $other_buffer:expr, $current_buffer:expr, $bset:ident, $field:ident,
     $type:ty) => {{
      let $tmp: $type = BVAR! ($other_buffer, $field);
      $bset ($other_buffer, BVAR! ($current_buffer, $field));
      $bset ($current_buffer, $tmp);
    }};
  }

  swapfield! (tmp_own_text, other_buffer, (current_buffer! ()), own_text, buffer_text);
  eassert! ((*(current_buffer! ())).text == &raw mut (*(current_buffer! ())).own_text);
  eassert! ((*other_buffer).text == &raw mut (*other_buffer).own_text);
  // C 2609-2614 is `#ifdef REL_ALLOC` (undef): the two `r_alloc_reset_variable` calls.

  swapfield! (tmp_pt, other_buffer, (current_buffer! ()), pt, ptrdiff_t);
  swapfield! (tmp_pt_byte, other_buffer, (current_buffer! ()), pt_byte, ptrdiff_t);
  swapfield! (tmp_begv, other_buffer, (current_buffer! ()), begv, ptrdiff_t);
  swapfield! (tmp_begv_byte, other_buffer, (current_buffer! ()), begv_byte, ptrdiff_t);
  swapfield! (tmp_zv, other_buffer, (current_buffer! ()), zv, ptrdiff_t);
  swapfield! (tmp_zv_byte, other_buffer, (current_buffer! ()), zv_byte, ptrdiff_t);
  eassert! ((*(current_buffer! ())).base_buffer.is_null ());
  eassert! ((*other_buffer).base_buffer.is_null ());
  swapfield! (tmp_indirections, other_buffer, (current_buffer! ()), indirections, c_int);
  (*(current_buffer! ())).bits |= BUFFER_CLIP_CHANGED;
  (*other_buffer).bits |= BUFFER_CLIP_CHANGED;
  swapfield! (tmp_newline_cache, other_buffer, (current_buffer! ()), newline_cache, *mut c_void);
  swapfield! (tmp_width_run_cache, other_buffer, (current_buffer! ()), width_run_cache, *mut c_void);
  swapfield! (tmp_bidi_paragraph_cache, other_buffer, (current_buffer! ()), bidi_paragraph_cache, *mut c_void);
  (*(current_buffer! ())).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;
  (*other_buffer).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;
  // C: `swapfield (long_line_optimizations_p, bool_bf);` -- the field is buffer.h's 1-bit
  // `long_line_optimizations_p` inside `bits`.
  if ((*other_buffer).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P)
    != ((*(current_buffer! ())).bits & BUFFER_LONG_LINE_OPTIMIZATIONS_P)
    {
      (*(current_buffer! ())).bits ^= BUFFER_LONG_LINE_OPTIMIZATIONS_P;
      (*other_buffer).bits ^= BUFFER_LONG_LINE_OPTIMIZATIONS_P;
    }
  swapfield_! (tmp_undo_list, other_buffer, (current_buffer! ()), bset_undo_list, undo_list_, Lisp_Object);
  swapfield_! (tmp_mark, other_buffer, (current_buffer! ()), bset_mark, mark_, Lisp_Object);
  swapfield_! (tmp_mark_active, other_buffer, (current_buffer! ()), bset_mark_active, mark_active_, Lisp_Object); /* Belongs with the `mark'.  */
  swapfield_! (tmp_enable_multibyte_characters, other_buffer, (current_buffer! ()), bset_enable_multibyte_characters, enable_multibyte_characters_, Lisp_Object);
  swapfield_! (tmp_bidi_display_reordering, other_buffer, (current_buffer! ()), bset_bidi_display_reordering, bidi_display_reordering_, Lisp_Object);
  swapfield_! (tmp_bidi_paragraph_direction, other_buffer, (current_buffer! ()), bset_bidi_paragraph_direction, bidi_paragraph_direction_, Lisp_Object);
  swapfield_! (tmp_bidi_paragraph_separate_re, other_buffer, (current_buffer! ()), bset_bidi_paragraph_separate_re, bidi_paragraph_separate_re_, Lisp_Object);
  swapfield_! (tmp_bidi_paragraph_start_re, other_buffer, (current_buffer! ()), bset_bidi_paragraph_start_re, bidi_paragraph_start_re_, Lisp_Object);
  /* FIXME: Not sure what we should do with these *_marker fields.
     Hopefully they're just nil anyway.  */
  swapfield_! (tmp_pt_marker, other_buffer, (current_buffer! ()), bset_pt_marker, pt_marker_, Lisp_Object);
  swapfield_! (tmp_begv_marker, other_buffer, (current_buffer! ()), bset_begv_marker, begv_marker_, Lisp_Object);
  swapfield_! (tmp_zv_marker, other_buffer, (current_buffer! ()), bset_zv_marker, zv_marker_, Lisp_Object);
  bset_point_before_scroll ((current_buffer! ()), Qnil ());
  bset_point_before_scroll (other_buffer, Qnil ());

  // C 2648-2653 is `#ifdef HAVE_TREE_SITTER` (undef): `ts_parser_list` and the three
  // `ts_linecol_*` swaps.

  modiff_incr (&raw mut (*(*(current_buffer! ())).text).modiff, 1);
  modiff_incr (&raw mut (*(*other_buffer).text).modiff, 1);
  modiff_incr (&raw mut (*(*(current_buffer! ())).text).chars_modiff, 1);
  modiff_incr (&raw mut (*(*other_buffer).text).chars_modiff, 1);
  modiff_incr (&raw mut (*(*(current_buffer! ())).text).overlay_modiff, 1);
  modiff_incr (&raw mut (*(*other_buffer).text).overlay_modiff, 1);
  (*(*(current_buffer! ())).text).beg_unchanged = (*(*(current_buffer! ())).text).gpt;
  (*(*(current_buffer! ())).text).end_unchanged = (*(*(current_buffer! ())).text).gpt;
  (*(*other_buffer).text).beg_unchanged = (*(*other_buffer).text).gpt;
  (*(*other_buffer).text).end_unchanged = (*(*other_buffer).text).gpt;
  swap_buffer_overlays ((current_buffer! ()), other_buffer);
  {
    let mut m: *mut Lisp_Marker;
    m = BUF_MARKERS! ((current_buffer! ()));
    while ! m.is_null ()
      {
	if (*m).buffer as *mut buffer == other_buffer
	  { (*m).buffer = (current_buffer! ()) as *mut c_void; }
	else
	  /* Since there's no indirect buffer in sight, markers on
	     BUF_MARKERS(buf) should either be for `buf' or dead.  */
	  { eassert! ((*m).buffer.is_null ()); }
	m = (*m).next;
      }
    m = BUF_MARKERS! (other_buffer);
    while ! m.is_null ()
      {
	if (*m).buffer as *mut buffer == (current_buffer! ())
	  { (*m).buffer = other_buffer as *mut c_void; }
	else
	  /* Since there's no indirect buffer in sight, markers on
	     BUF_MARKERS(buf) should either be for `buf' or dead.  */
	  { eassert! ((*m).buffer.is_null ()); }
	m = (*m).next;
      }
  }
  { /* Some of the C code expects that both window markers of a
       live window points to that window's buffer.  So since we
       just swapped the markers between the two buffers, we need
       to undo the effect of this swap for window markers.  */
    let mut w: Lisp_Object = selected_window;
    let mut ws: Lisp_Object = Qnil ();
    let mut buf1: Lisp_Object = Qnil ();
    let mut buf2: Lisp_Object = Qnil ();
    XSETBUFFER! (buf1, (current_buffer! ()));
    XSETBUFFER! (buf2, other_buffer);

    while NILP (Fmemq (w, ws))
      {
	ws = Fcons (w, ws);
	if MARKERP ((*XWINDOW (w)).pointm)
	    && (BASE_EQ ((*XWINDOW (w)).contents, buf1)
		|| BASE_EQ ((*XWINDOW (w)).contents, buf2))
	  { Fset_marker ((*XWINDOW (w)).pointm,
			 make_fixnum (BUF_BEGV (XBUFFER ((*XWINDOW (w)).contents))),
			 (*XWINDOW (w)).contents); }
	/* Blindly copied from pointm part.  */
	if MARKERP ((*XWINDOW (w)).old_pointm)
	    && (BASE_EQ ((*XWINDOW (w)).contents, buf1)
		|| BASE_EQ ((*XWINDOW (w)).contents, buf2))
	  { Fset_marker ((*XWINDOW (w)).old_pointm,
			 make_fixnum (BUF_BEGV (XBUFFER ((*XWINDOW (w)).contents))),
			 (*XWINDOW (w)).contents); }
	if MARKERP ((*XWINDOW (w)).start)
	    && (BASE_EQ ((*XWINDOW (w)).contents, buf1)
		|| BASE_EQ ((*XWINDOW (w)).contents, buf2))
	  { Fset_marker ((*XWINDOW (w)).start,
			 make_fixnum ((*XBUFFER ((*XWINDOW (w)).contents)).last_window_start),
			 (*XWINDOW (w)).contents); }
	w = Fnext_window (w, Qt (), Qt ());
      }
  }

  if ! (*(*(current_buffer! ())).text).intervals.is_null ()
    {
      let iv: *mut interval = (*(*(current_buffer! ())).text).intervals as *mut interval;
      eassert! (BASE_EQ ((*iv).up.obj, buffer));
      XSETBUFFER! ((*iv).up.obj, (current_buffer! ()));
    }
  if ! (*(*other_buffer).text).intervals.is_null ()
    {
      let iv: *mut interval = (*(*other_buffer).text).intervals as *mut interval;
      eassert! (BASE_EQ ((*iv).up.obj, Fcurrent_buffer ()));
      XSETBUFFER! ((*iv).up.obj, other_buffer);
    }

  return Qnil ();
});

DEFUN! ("set-buffer-multibyte", Fset_buffer_multibyte, Sset_buffer_multibyte,
       1, 1, 0,
       doc: "Set the multibyte flag of the current buffer to FLAG.\nIf FLAG is t, this makes the buffer a multibyte buffer.\nIf FLAG is nil, this makes the buffer a single-byte buffer.\nIn these cases, the buffer contents remain unchanged as a sequence of\nbytes but the contents viewed as characters do change.\nIf FLAG is `to', this makes the buffer a multibyte buffer by changing\nall eight-bit bytes to eight-bit characters.\nIf the multibyte flag was really changed, undo information of the\ncurrent buffer is cleared.",
       (flag: Lisp_Object),
       {
  let mut tail: *mut Lisp_Marker;
  let mut markers: *mut Lisp_Marker;
  let mut btail: Lisp_Object = Qnil ();
  let mut other: Lisp_Object = Qnil ();
  let mut begv: ptrdiff_t;
  let mut zv: ptrdiff_t;
  let narrowed: bool = BEG as ptrdiff_t != BEGV! () || Z! () != ZV! ();
  let modified_p: bool = ! NILP (Fbuffer_modified_p (Qnil ()));
  let old_undo: Lisp_Object = BVAR! ((current_buffer! ()), undo_list_);

  if ! (*(current_buffer! ())).base_buffer.is_null ()
    { error (c"Cannot do `set-buffer-multibyte' on an indirect buffer".as_ptr ()); }

  /* Do nothing if nothing actually changes.  */
  if NILP (flag) == NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { return flag; }

  /* Don't record these buffer changes.  We will put a special undo entry
     instead.  */
  bset_undo_list ((current_buffer! ()), Qt ());

  /* If the cached position is for this buffer, clear it out.  */
  clear_charpos_cache ((current_buffer! ()));

  if NILP (flag)
    { begv = BEGV_BYTE! (); zv = ZV_BYTE! (); }
  else
    { begv = BEGV! (); zv = ZV! (); }

  if narrowed
    { error (c"Changing multibyteness in a narrowed buffer".as_ptr ()); }

  invalidate_buffer_caches ((current_buffer! ()) as *mut c_void, BEGV! (), ZV! ());

  if NILP (flag)
    {
      let mut pos: ptrdiff_t;
      let mut stop: ptrdiff_t;
      let mut p: *mut u8;

      /* Do this first, so it can use CHAR_TO_BYTE
	 to calculate the old correspondences.  */
      set_intervals_multibyte (false);
      set_overlays_multibyte (false);

      bset_enable_multibyte_characters ((current_buffer! ()), Qnil ());

      Z! () = Z_BYTE! ();
      BEGV! () = BEGV_BYTE! ();
      ZV! () = ZV_BYTE! ();
      GPT! () = GPT_BYTE! ();
      TEMP_SET_PT_BOTH (PT_BYTE! (), PT_BYTE! ());

      tail = BUF_MARKERS! ((current_buffer! ()));
      while ! tail.is_null ()
	{
	  (*tail).charpos = (*tail).bytepos;
	  tail = (*tail).next;
	}

      /* Convert multibyte form of 8-bit characters to unibyte.  */
      pos = BEG as ptrdiff_t;
      stop = GPT! ();
      p = BEG_ADDR! ();
      loop
	{
	  if pos == stop
	    {
	      if pos == Z! ()
		{ break; }
	      p = GAP_END_ADDR! ();
	      stop = Z! ();
	    }
	  if ASCII_CHAR_P (*p as isize)
	    { p = p.add (1); pos += 1; }
	  else if CHAR_BYTE8_HEAD_P (*p as c_int)
	    {
	      let mut bytes: c_int = 0;
	      let c: c_int = string_char_and_length (p, &mut bytes);
	      /* Delete all bytes for this 8-bit character but the
		 last one, and change the last one to the character
		 code.  */
	      bytes -= 1;
	      del_range_2 (pos, pos, pos + bytes as ptrdiff_t, pos + bytes as ptrdiff_t, false);
	      p = GAP_END_ADDR! ();
	      *p = c as u8;
	      p = p.add (1);
	      pos += 1;
	      if begv > pos
		{ begv -= bytes as ptrdiff_t; }
	      if zv > pos
		{ zv -= bytes as ptrdiff_t; }
	      stop = Z! ();
	    }
	  else
	    {
	      let bytes: c_int = BYTES_BY_CHAR_HEAD (*p as c_int);
	      p = p.add (bytes as usize);
	      pos += bytes as ptrdiff_t;
	    }
	}
    }
  else
    {
      let mut pt: ptrdiff_t = PT! ();
      let mut pos: ptrdiff_t;
      let mut stop: ptrdiff_t;
      let mut p: *mut u8;
      let mut pend: *mut u8;

      /* Be sure not to have a multibyte sequence striding over the GAP.
	 Ex: We change this: "...abc\302 _GAP_ \241def..."
	     to: "...abc _GAP_ \302\241def..."  */

      if EQ (flag, Qt ())
	  && GPT_BYTE! () > 1 && GPT_BYTE! () < Z_BYTE! ()
	  && ! CHAR_HEAD_P (*GAP_END_ADDR! () as c_int)
	{
	  let mut q: *mut u8 = GPT_ADDR! ().offset (-1);

	  while ! CHAR_HEAD_P (*q as c_int) && q > BEG_ADDR! ()
	    { q = q.offset (-1); }
	  if LEADING_CODE_P (*q as c_int)
	    {
	      let new_gpt: ptrdiff_t = GPT_BYTE! ()
		- GPT_ADDR! ().offset_from (q) as ptrdiff_t;

	      move_gap_both (new_gpt, new_gpt);
	    }
	}

      /* Make the buffer contents valid as multibyte by converting
	 8-bit characters to multibyte form.  */
      pos = BEG as ptrdiff_t;
      stop = GPT! ();
      p = BEG_ADDR! ();
      pend = GPT_ADDR! ();
      loop
	{
	  let mut bytes: c_int = 0;   /* PORTING.md 1.5: C assigns it in the condition */

	  if pos == stop
	    {
	      if pos == Z! ()
		{ break; }
	      p = GAP_END_ADDR! ();
	      pend = Z_ADDR! ();
	      stop = Z! ();
	    }

	  if ASCII_CHAR_P (*p as isize)
	    { p = p.add (1); pos += 1; }
	  else if EQ (flag, Qt ())
		   && { bytes = multibyte_length (p, pend, true, false); bytes > 0 }
	    { p = p.add (bytes as usize); pos += bytes as ptrdiff_t; }
	  else
	    {
	      let mut tmp: [u8; MAX_MULTIBYTE_LENGTH as usize]
		= [0; MAX_MULTIBYTE_LENGTH as usize];
	      let mut c: c_int;

	      c = BYTE8_TO_CHAR (*p as c_int);
	      bytes = CHAR_STRING (c, tmp.as_mut_ptr ());
	      *p = tmp[0];
	      TEMP_SET_PT_BOTH (pos + 1, pos + 1);
	      bytes -= 1;
	      insert_1_both (tmp.as_mut_ptr ().add (1) as *mut c_char,
			     bytes as ptrdiff_t, bytes as ptrdiff_t, true, false, false);
	      /* Now the gap is after the just inserted data.  */
	      pos = GPT! ();
	      p = GAP_END_ADDR! ();
	      if pos <= begv
		{ begv += bytes as ptrdiff_t; }
	      if pos <= zv
		{ zv += bytes as ptrdiff_t; }
	      if pos <= pt
		{ pt += bytes as ptrdiff_t; }
	      pend = Z_ADDR! ();
	      stop = Z! ();
	    }
	}

      if pt != PT! ()
	{ TEMP_SET_PT (pt); }

      /* Do this first, so that chars_in_text asks the right question.
	 set_intervals_multibyte needs it too.  */
      bset_enable_multibyte_characters ((current_buffer! ()), Qt ());

      GPT_BYTE! () = advance_to_char_boundary (GPT_BYTE! ());
      GPT! () = chars_in_text (BEG_ADDR! (), GPT_BYTE! () - BEG_BYTE as ptrdiff_t) + BEG as ptrdiff_t;

      Z! () = chars_in_text (GAP_END_ADDR! (), Z_BYTE! () - GPT_BYTE! ()) + GPT! ();

      BEGV_BYTE! () = advance_to_char_boundary (BEGV_BYTE! ());
      if BEGV_BYTE! () > GPT_BYTE! ()
	{ BEGV! () = chars_in_text (GAP_END_ADDR! (), BEGV_BYTE! () - GPT_BYTE! ()) + GPT! (); }
      else
	{ BEGV! () = chars_in_text (BEG_ADDR! (), BEGV_BYTE! () - BEG_BYTE as ptrdiff_t) + BEG as ptrdiff_t; }

      ZV_BYTE! () = advance_to_char_boundary (ZV_BYTE! ());
      if ZV_BYTE! () > GPT_BYTE! ()
	{ ZV! () = chars_in_text (GAP_END_ADDR! (), ZV_BYTE! () - GPT_BYTE! ()) + GPT! (); }
      else
	{ ZV! () = chars_in_text (BEG_ADDR! (), ZV_BYTE! () - BEG_BYTE as ptrdiff_t) + BEG as ptrdiff_t; }

      {
	let byte: ptrdiff_t = advance_to_char_boundary (PT_BYTE! ());
	let position: ptrdiff_t;

	if byte > GPT_BYTE! ()
	  { position = chars_in_text (GAP_END_ADDR! (), byte - GPT_BYTE! ()) + GPT! (); }
	else
	  { position = chars_in_text (BEG_ADDR! (), byte - BEG_BYTE as ptrdiff_t) + BEG as ptrdiff_t; }
	TEMP_SET_PT_BOTH (position, byte);
      }

      tail = BUF_MARKERS! ((current_buffer! ()));
      markers = tail;

      /* This prevents BYTE_TO_CHAR (that is, buf_bytepos_to_charpos) from
	 getting confused by the markers that have not yet been updated.
	 It is also a signal that it should never create a marker.  */
      BUF_MARKERS! ((current_buffer! ())) = std::ptr::null_mut ();

      while ! tail.is_null ()
	{
	  (*tail).bytepos = advance_to_char_boundary ((*tail).bytepos);
	  (*tail).charpos = BYTE_TO_CHAR ((*tail).bytepos);
	  tail = (*tail).next;
	}

      /* Make sure no markers were put on the chain
	 while the chain value was incorrect.  */
      if ! BUF_MARKERS! ((current_buffer! ())).is_null ()
	{ emacs_abort (); }

      BUF_MARKERS! ((current_buffer! ())) = markers;

      /* Do this last, so it can calculate the new correspondences
	 between chars and bytes.  */
      /* FIXME: Is it worth the trouble, really?  Couldn't we just throw
         away all the text-properties instead of trying to guess how
         to adjust them?  AFAICT the result is not reliable anyway.  */
      set_intervals_multibyte (true);
      set_overlays_multibyte (true);
    }

  if ! EQ (old_undo, Qt ())
    {
      /* Represent all the above changes by a special undo entry.  */
      bset_undo_list ((current_buffer! ()),
		      Fcons (list3 (Qapply (),
				    Qset_buffer_multibyte (),
				    if NILP (flag) { Qt () } else { Qnil () }),
			     old_undo));
    }

  (*(current_buffer! ())).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;

  /* If buffer is shown in a window, let redisplay consider other windows.  */
  if buffer_window_count ((current_buffer! ())) != 0
    { windows_or_buffers_changed = 10; }

  /* Copy this buffer's new multibyte status
     into all of its indirect buffers.  */
  /* C: `FOR_EACH_LIVE_BUFFER (btail, other)` (P-22: the expansion).  */
  btail = Vbuffer_alist;
  while CONSP (btail) && { other = XCDR (XCAR (btail)); true }
    {
      let o: *mut buffer = XBUFFER (other);
      if (*o).base_buffer == (current_buffer! ()) && BUFFER_LIVE_P (o)
	{
	  BVAR! (o, enable_multibyte_characters_)
	    = BVAR! ((current_buffer! ()), enable_multibyte_characters_);
	  (*o).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P;
	}
      btail = XCDR (btail);
    }

  /* Restore the modifiedness of the buffer.  */
  if ! modified_p && ! NILP (Fbuffer_modified_p (Qnil ()))
    { Fset_buffer_modified_p (Qnil ()); }

  /* Update coding systems of this buffer's process (if any).  */
  {
    let process: Lisp_Object;

    process = Fget_buffer_process (Fcurrent_buffer ());
    if PROCESSP (process)
      { setup_process_coding_systems (process); }
  }

  return flag;
});

DEFUN! ("kill-all-local-variables", Fkill_all_local_variables,
       Skill_all_local_variables, 0, 1, 0,
       doc: "Switch to Fundamental mode by killing current buffer's local variables.\nMost local variable bindings are eliminated so that the default values\nbecome effective once more.  Also, the syntax table is set from\n`standard-syntax-table', the local keymap is set to nil,\nand the abbrev table from `fundamental-mode-abbrev-table'.\nThis function also forces redisplay of the mode line.\n\nEvery function to select a new major mode starts by\ncalling this function.\n\nAs a special exception, local variables whose names have a non-nil\n`permanent-local' property are not eliminated by this function.  If\nthe optional KILL-PERMANENT argument is non-nil, clear out these local\nvariables, too.\n\nIf KILL-PERMANENT is the symbol `permanent-local', kill local variables\nand ignore variable watchers.  If KILL-PERMANENT is the symbol `reset',\nignore variable watchers and reset the buffer as if newly created.  Use\nthese with caution.  For example, `reset' sets buffer variables such as\n`default-directory' to nil and may result in unexpected behavior.\n\nThe first thing this function does is run\nthe normal hook `change-major-mode-hook'.",
       (kill_permanent: Lisp_Object),
       {
  run_hook (Qchange_major_mode_hook ());

  let mut permanent_too: c_int = 0;
  if ! NILP (kill_permanent)
    {
      permanent_too = 2;
      if EQ (kill_permanent, Qpermanent_local ())
	{ permanent_too = 1; }
      else if EQ (kill_permanent, Qreset ())
	{
	  permanent_too = 1;
	  reset_buffer ((current_buffer! ()));
	}
    }

  /* Actually eliminate all local bindings of this buffer.  */
  reset_buffer_local_variables ((current_buffer! ()), permanent_too);

  /* Force mode-line redisplay.  Useful here because all major mode
     commands call this function.  */
  bset_update_mode_line ((current_buffer! ()) as *mut c_void);

  return Qnil ();
});

/* Find all the overlays in the current buffer that overlap the range
   [BEG, END).

   If EMPTY is true, include empty overlays in that range and also
   those at END, provided END denotes the position at the end of the
   buffer.

   If TRAILING is true, include overlays that begin at ZV.
   (FIXME: This argument is nowadays effectively ignored, since the code
   no longer pays any attention to ZV.  We basically rely on the caller
   to ensure that TRAILING is only true when END > ZV.  This happens, e.g.,
   when we are called from overlays_at with BEG = ZV and END = ZV + 1.)

   Return the number found, and store them in a vector in *VEC_PTR.
   Store in *LEN_PTR the size allocated for the vector.
   Store in *NEXT_PTR the next position after POS where an overlay starts,
     or Z if there are no more overlays after POS.
   NEXT_PTR may be 0, meaning don't store that info.

   *VEC_PTR and *LEN_PTR should contain a valid vector and size
   when this function is called.

   If EXTEND, make the vector bigger if necessary.  If not, never
   extend the vector, and store only as many overlays as will fit.
   But still return the total number of overlays.
*/

unsafe fn overlays_in (beg: ptrdiff_t, end: ptrdiff_t, extend: bool,
		       vec_ptr: *mut *mut Lisp_Object, len_ptr: *mut ptrdiff_t,
		       empty: bool, trailing: bool,
		       next_ptr: *mut ptrdiff_t) -> ptrdiff_t
{
  let mut idx: ptrdiff_t = 0;
  let mut len: ptrdiff_t = *len_ptr;
  let mut next: ptrdiff_t = Z! ();
  let mut vec: *mut Lisp_Object = *vec_ptr;
  let mut node: *mut itree_node;

  /* Extend the search range if overlays beginning at Z are wanted.  */
  let mut search_end: ptrdiff_t = Z! ();
  if end >= Z! () && (empty || trailing)
    { search_end += 1; }

  /* itree.h 166-176's `ITREE_FOREACH (node, current_buffer->overlays, beg, search_end,
     ASCENDING)` (P-33, status 3.7).  */
  if (*(current_buffer! ())).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*(current_buffer! ())).overlays,
				beg, search_end, ITREE_ASCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  if (*node).begin > end
	    {
	      next = next.min ((*node).begin);
	      break;
	    }
	  else if (*node).begin == end
	    {
	      next = (*node).begin;
	      if (! empty || end < Z! ()) && beg < end
		{ break; }
	      if empty && (*node).begin != (*node).end
		{ continue; }
	    }

	  if ! empty && (*node).begin == (*node).end
	    { continue; }

	  if extend && idx == len
	    {
	      vec = xpalloc (vec as *mut c_void, len_ptr, 1, OVERLAY_COUNT_MAX! (),
			     std::mem::size_of::<Lisp_Object> () as isize) as *mut Lisp_Object;
	      *vec_ptr = vec;
	      len = *len_ptr;
	    }
	  if idx < len
	    { *vec.add (idx as usize) = (*node).data; }
	  /* Keep counting overlays even if we can't return them all.  */
	  idx += 1;
	}
    }
  if ! next_ptr.is_null ()
    { *next_ptr = if next != 0 { next } else { Z! () }; }

  return idx;
}

/* Find all non-empty overlays in the current buffer that contain
   position POS.

   See overlays_in for the meaning of the arguments.
  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn overlays_at (pos: ptrdiff_t, extend: bool,
				      vec_ptr: *mut *mut Lisp_Object, len_ptr: *mut ptrdiff_t,
				      next_ptr: *mut ptrdiff_t) -> ptrdiff_t
{
  return overlays_in (pos, pos + 1, extend, vec_ptr, len_ptr,
		      false, true, next_ptr);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn next_overlay_change (pos: ptrdiff_t) -> ptrdiff_t
{
  let mut next: ptrdiff_t = ZV! ();
  let mut node: *mut itree_node;

  /* C: `ITREE_FOREACH (node, current_buffer->overlays, pos, next, ASCENDING)`.  */
  if (*(current_buffer! ())).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*(current_buffer! ())).overlays,
				pos, next, ITREE_ASCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  if (*node).begin > pos
	    {
	      /* If we reach this branch, node->begin must be the least upper bound
		 of pos, because the search is limited to [pos,next) . */
	      eassert! ((*node).begin < next);
	      next = (*node).begin;
	      break;
	    }
	  else if (*node).begin < (*node).end && (*node).end < next
	    {
	      next = (*node).end;
	      itree_iterator_narrow (itree_iter_, pos, next);
	    }
	}
    }

  return next;
}

unsafe fn previous_overlay_change (pos: ptrdiff_t) -> ptrdiff_t
{
  let mut node: *mut itree_node;
  let mut prev: ptrdiff_t = BEGV! ();

  /* C: `ITREE_FOREACH (node, current_buffer->overlays, prev, pos, DESCENDING)`.  */
  if (*(current_buffer! ())).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*(current_buffer! ())).overlays,
				prev, pos, ITREE_DESCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  if (*node).end < pos
	    { prev = (*node).end; }
	  else
	    { prev = prev.max ((*node).begin); }
	  itree_iterator_narrow (itree_iter_, prev, pos);
	}
    }

  return prev;
}

/* Return true if there exists an overlay with a non-nil
   `mouse-face' property overlapping OVERLAY.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mouse_face_overlay_overlaps (overlay: Lisp_Object) -> bool
{
  let start: ptrdiff_t = OVERLAY_START (overlay);
  let end: ptrdiff_t = OVERLAY_END (overlay);
  let mut tem: Lisp_Object;
  let mut node: *mut itree_node;

  /* C: `ITREE_FOREACH (node, current_buffer->overlays, start, min (end, ZV) + 1,
     ASCENDING)`.  */
  if (*(current_buffer! ())).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*(current_buffer! ())).overlays,
				start, end.min (ZV! ()) + 1, ITREE_ASCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  if (*node).begin < end && (*node).end > start
	      && (*node).begin < (*node).end
	      && ! BASE_EQ ((*node).data, overlay)
	      && { tem = Foverlay_get (overlay, Qmouse_face ()); ! NILP (tem) }
	    { return true; }
	}
    }
  return false;
}

/* Return the value of the 'display-line-numbers-disable' property at
   EOB, if there's an overlay at ZV with a non-nil value of that property.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn disable_line_numbers_overlay_at_eob () -> bool
{
  let mut tem: Lisp_Object = Qnil ();
  let mut node: *mut itree_node;

  /* C: `ITREE_FOREACH (node, current_buffer->overlays, ZV, ZV, ASCENDING)`.  */
  if (*(current_buffer! ())).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*(current_buffer! ())).overlays,
				ZV! (), ZV! (), ITREE_ASCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  if { tem = Foverlay_get ((*node).data, Qdisplay_line_numbers_disable ());
	       ! NILP (tem) }
	    { return true; }
	}
    }
  return false;
}

/* Fast function to just test if we're at an overlay boundary.

   Returns true if some overlay starts or ends (or both) at POS,
*/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn overlay_touches_p (pos: ptrdiff_t) -> bool
{
  let mut node: *mut itree_node;

  /* We need to find overlays ending in pos, as well as empty ones at
     pos. */
  /* C: `ITREE_FOREACH (node, current_buffer->overlays, pos - 1, pos + 1, DESCENDING)`
     with C's brace-less body.  */
  if (*(current_buffer! ())).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*(current_buffer! ())).overlays,
				pos - 1, pos + 1, ITREE_DESCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  if (*node).begin == pos || (*node).end == pos
	    { return true; }
	}
    }
  return false;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compare_overlays (v1: *const c_void, v2: *const c_void) -> c_int
{
  let s1: *const sortvec = v1 as *const sortvec;
  let s2: *const sortvec = v2 as *const sortvec;
  /* Return 1 if s1 should take precedence, -1 if v2 should take precedence,
     and 0 if they're equal.  */
  if (*s1).priority != (*s2).priority
    { return if (*s1).priority < (*s2).priority { -1 } else { 1 }; }
  /* If the priority is equal, give precedence to the one not covered by the
     other.  If neither covers the other, obey spriority.  */
  else if (*s1).beg < (*s2).beg
    { return if (*s1).end < (*s2).end && (*s1).spriority > (*s2).spriority { 1 } else { -1 }; }
  else if (*s1).beg > (*s2).beg
    { return if (*s1).end > (*s2).end && (*s1).spriority < (*s2).spriority { -1 } else { 1 }; }
  else if (*s1).end != (*s2).end
    { return if (*s2).end < (*s1).end { -1 } else { 1 }; }
  else if (*s1).spriority != (*s2).spriority
    { return if (*s1).spriority < (*s2).spriority { -1 } else { 1 }; }
  else if BASE_EQ ((*s1).overlay, (*s2).overlay)
    { return 0; }
  else
    /* Avoid the non-determinism of qsort by choosing an arbitrary ordering
       between "equal" overlays.  The result can still change between
       invocations of Emacs, but it won't change in the middle of
       `find_field' (bug#6830).  */
    { return if XLI ((*s1).overlay) < XLI ((*s2).overlay) { -1 } else { 1 }; }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_sortvec_item (item: *mut sortvec, overlay: Lisp_Object)
{
  let mut tem: Lisp_Object;
  /* This overlay is good and counts: put it into sortvec.  */
  (*item).overlay = overlay;
  (*item).beg = OVERLAY_START (overlay);
  (*item).end = OVERLAY_END (overlay);
  tem = Foverlay_get (overlay, Qpriority ());
  if NILP (tem)
    {
      (*item).priority = 0;
      (*item).spriority = 0;
    }
  else if FIXNUMP (tem)
    {
      (*item).priority = XFIXNUM (tem);
      (*item).spriority = 0;
    }
  else if CONSP (tem)
    {
      let car: Lisp_Object = XCAR (tem);
      let cdr: Lisp_Object = XCDR (tem);
      (*item).priority  = if FIXNUMP (car) { XFIXNUM (car) } else { 0 };
      (*item).spriority = if FIXNUMP (cdr) { XFIXNUM (cdr) } else { 0 };
    }
}
/* Sort an array of overlays by priority.  The array is modified in place.
   The return value is the new size; this may be smaller than the original
   size if some of the overlays were invalid or were window-specific.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sort_overlays (overlay_vec: *mut Lisp_Object, noverlays: ptrdiff_t,
					w: *mut c_void) -> ptrdiff_t
{
  let mut i: ptrdiff_t;
  let mut j: ptrdiff_t;
  USE_SAFE_ALLOCA! (sa_avail, sa_count);
  let mut sortvec: *mut sortvec = std::ptr::null_mut ();
  let mut noverlays = noverlays;   /* C's `noverlays = j;` below rebinds it.  */

  SAFE_NALLOCA! (sa_avail, sortvec, 1, noverlays);

  /* Put the valid and relevant overlays into sortvec.  */

  i = 0;
  j = 0;
  while i < noverlays
    {
      let overlay: Lisp_Object;

      overlay = *overlay_vec.add (i as usize);
      if OVERLAYP (overlay)
	  && OVERLAY_START (overlay) > 0
	  && OVERLAY_END (overlay) > 0
	{
	  /* If we're interested in a specific window, then ignore
	     overlays that are limited to some other window.  */
	  if ! w.is_null () && ! overlay_matches_window (w as *const window, overlay)
	    { i += 1; continue; }
	  make_sortvec_item (sortvec.add (j as usize), overlay);
	  j += 1;
	}
      i += 1;
    }
  noverlays = j;

  /* Sort the overlays into the proper order: increasing priority.  */

  if noverlays > 1
    { qsort (sortvec as *mut c_void, noverlays as usize,
	     std::mem::size_of::<sortvec> (), Some (compare_overlays)); }

  i = 0;
  while i < noverlays
    {
      *overlay_vec.add (i as usize) = (*sortvec.add (i as usize)).overlay;
      i += 1;
    }

  SAFE_FREE! (sa_count);
  return noverlays;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sortstr
{
  string: Lisp_Object,
  string2: Lisp_Object,
  size: ptrdiff_t,
  priority: EMACS_INT,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sortstrlist
{
  buf: *mut sortstr,	/* An array that expands as needed; never freed.  */
  size: ptrdiff_t,	/* Allocated length of that array.  */
  used: ptrdiff_t,	/* How much of the array is currently in use.  */
  bytes: ptrdiff_t,	/* Total length of the strings in buf.  */
}

/* Buffers for storing information about the overlays touching a given
   position.  These could be automatic variables in overlay_strings, but
   it's more efficient to hold onto the memory instead of repeatedly
   allocating and freeing it.  */
static mut overlay_heads: sortstrlist = sortstrlist {
  buf: std::ptr::null_mut (), size: 0, used: 0, bytes: 0 };
static mut overlay_tails: sortstrlist = sortstrlist {
  buf: std::ptr::null_mut (), size: 0, used: 0, bytes: 0 };
static mut overlay_str_buf: *mut u8 = std::ptr::null_mut ();

/* Allocated length of overlay_str_buf.  */
static mut overlay_str_len: ptrdiff_t = 0;

/* A comparison function suitable for passing to qsort.  */
unsafe extern "C" fn cmp_for_strings (as1: *const c_void, as2: *const c_void) -> c_int
{
  let s1: *const sortstr = as1 as *const sortstr;
  let s2: *const sortstr = as2 as *const sortstr;
  if (*s1).size != (*s2).size
    { return if (*s2).size < (*s1).size { -1 } else { 1 }; }
  if (*s1).priority != (*s2).priority
    { return if (*s1).priority < (*s2).priority { -1 } else { 1 }; }
  return 0;
}

unsafe fn record_overlay_string (ssl: *mut sortstrlist, str_: Lisp_Object,
				 str2: Lisp_Object, pri: Lisp_Object, size: ptrdiff_t)
{
  let mut nbytes: ptrdiff_t;

  if (*ssl).used == (*ssl).size
    { (*ssl).buf = xpalloc ((*ssl).buf as *mut c_void, &raw mut (*ssl).size, 5, -1,
			    std::mem::size_of::<sortstr> () as isize) as *mut sortstr; }
  (*(*ssl).buf.add ((*ssl).used as usize)).string = str_;
  (*(*ssl).buf.add ((*ssl).used as usize)).string2 = str2;
  (*(*ssl).buf.add ((*ssl).used as usize)).size = size;
  (*(*ssl).buf.add ((*ssl).used as usize)).priority
    = if FIXNUMP (pri) { XFIXNUM (pri) } else { 0 };
  (*ssl).used += 1;

  if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
    { nbytes = SCHARS (str_); }
  else if ! STRING_MULTIBYTE (str_)
    { nbytes = count_size_as_multibyte (SDATA (str_), SBYTES (str_)); }
  else
    { nbytes = SBYTES (str_); }

  /* C: `if (ckd_add (&nbytes, nbytes, ssl->bytes)) memory_full_up ();` (the
     `ckd_add`/`overflowing_add` family of token-aliases.tsv, P-15).  */
  let (nbytes_, add_overflow) = nbytes.overflowing_add ((*ssl).bytes);
  nbytes = nbytes_;
  if add_overflow
    { memory_full_up (); }
  (*ssl).bytes = nbytes;

  if STRINGP (str2)
    {
      if NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	{ nbytes = SCHARS (str2); }
      else if ! STRING_MULTIBYTE (str2)
	{ nbytes = count_size_as_multibyte (SDATA (str2), SBYTES (str2)); }
      else
	{ nbytes = SBYTES (str2); }

      let (nbytes_, add_overflow) = nbytes.overflowing_add ((*ssl).bytes);
      nbytes = nbytes_;
      if add_overflow
	{ memory_full_up (); }
      (*ssl).bytes = nbytes;
    }
}

/* Concatenate the strings associated with overlays that begin or end
   at POS, ignoring overlays that are specific to windows other than W.
   The strings are concatenated in the appropriate order: shorter
   overlays nest inside longer ones, and higher priority inside lower.
   Normally all of the after-strings come first, but zero-sized
   overlays have their after-strings ride along with the
   before-strings because it would look strange to print them
   inside-out.

   Returns the concatenated string's length, and return the pointer to
   that string via PSTR, if that variable is non-NULL.  The storage of
   the concatenated strings may be overwritten by subsequent calls.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn overlay_strings (pos: ptrdiff_t, w: *mut c_void,
					  pstr: *mut *mut u8) -> ptrdiff_t
{
  let multibyte: bool = ! NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_));
  let mut node: *mut itree_node;

  overlay_heads.used = 0;
  overlay_heads.bytes = 0;
  overlay_tails.used = 0;
  overlay_tails.bytes = 0;

  /* C: `ITREE_FOREACH (node, current_buffer->overlays, pos - 1, pos + 1, DESCENDING)`.  */
  if (*(current_buffer! ())).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*(current_buffer! ())).overlays,
				pos - 1, pos + 1, ITREE_DESCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  let overlay: Lisp_Object = (*node).data;
	  eassert! (OVERLAYP (overlay));

	  let startpos: ptrdiff_t = (*node).begin;
	  let endpos: ptrdiff_t = (*node).end;

	  if endpos != pos && startpos != pos
	    { continue; }
	  let window: Lisp_Object = Foverlay_get (overlay, Qwindow ());
	  if WINDOWP (window) && XWINDOW (window) as *mut c_void != w
	    { continue; }
	  let mut str_: Lisp_Object;
	  /* FIXME: Are we really sure that `record_overlay_string` can
	     never cause a non-local exit?  */
	  if startpos == pos
	      && { str_ = Foverlay_get (overlay, Qbefore_string ()); STRINGP (str_) }
	    { record_overlay_string (&raw mut overlay_heads, str_,
				     if startpos == endpos
				       { Foverlay_get (overlay, Qafter_string ()) }
				       else { Qnil () },
				     Foverlay_get (overlay, Qpriority ()),
				     endpos - startpos); }
	  else if endpos == pos
	      && { str_ = Foverlay_get (overlay, Qafter_string ()); STRINGP (str_) }
	    { record_overlay_string (&raw mut overlay_tails, str_, Qnil (),
				     Foverlay_get (overlay, Qpriority ()),
				     endpos - startpos); }
	}
    }

  if overlay_tails.used > 1
    { qsort (overlay_tails.buf as *mut c_void, overlay_tails.used as usize,
	     std::mem::size_of::<sortstr> (), Some (cmp_for_strings)); }
  if overlay_heads.used > 1
    { qsort (overlay_heads.buf as *mut c_void, overlay_heads.used as usize,
	     std::mem::size_of::<sortstr> (), Some (cmp_for_strings)); }
  if overlay_heads.bytes != 0 || overlay_tails.bytes != 0
    {
      let mut tem: Lisp_Object;
      let mut i: ptrdiff_t;
      let mut p: *mut u8;
      let mut total: ptrdiff_t;

      /* C: `if (ckd_add (&total, overlay_heads.bytes, overlay_tails.bytes)) memory_full_up ();` */
      let (total_, add_overflow) = overlay_heads.bytes.overflowing_add (overlay_tails.bytes);
      total = total_;
      if add_overflow
	{ memory_full_up (); }
      if total > overlay_str_len
	{ overlay_str_buf = xpalloc (overlay_str_buf as *mut c_void,
				     &raw mut overlay_str_len,
				     total - overlay_str_len, -1, 1) as *mut u8; }

      p = overlay_str_buf;
      i = overlay_tails.used;
      loop
	{
	  i -= 1;
	  if ! (i >= 0) { break; }
	  let nbytes: ptrdiff_t;
	  tem = (*overlay_tails.buf.add (i as usize)).string;
	  nbytes = copy_text (SDATA (tem), p, SBYTES (tem),
			      STRING_MULTIBYTE (tem), multibyte);
	  p = p.add (nbytes as usize);
	}
      i = 0;
      while i < overlay_heads.used
	{
	  let mut nbytes: ptrdiff_t;
	  tem = (*overlay_heads.buf.add (i as usize)).string;
	  nbytes = copy_text (SDATA (tem), p, SBYTES (tem),
			      STRING_MULTIBYTE (tem), multibyte);
	  p = p.add (nbytes as usize);
	  tem = (*overlay_heads.buf.add (i as usize)).string2;
	  if STRINGP (tem)
	    {
	      nbytes = copy_text (SDATA (tem), p, SBYTES (tem),
				  STRING_MULTIBYTE (tem), multibyte);
	      p = p.add (nbytes as usize);
	    }
	  i += 1;
	}
      if p != overlay_str_buf.add (total as usize)
	{ emacs_abort (); }
      if ! pstr.is_null ()
	{ *pstr = overlay_str_buf; }
      return total;
    }
  return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adjust_overlays_for_insert (pos: ptrdiff_t, length: ptrdiff_t,
						    before_markers: bool)
{
  if (*(current_buffer! ())).indirections == 0
    { itree_insert_gap ((*(current_buffer! ())).overlays, pos, length, before_markers); }
  else
    {
      let base: *mut buffer = if ! (*(current_buffer! ())).base_buffer.is_null ()
	{ (*(current_buffer! ())).base_buffer }
	else
	{ (current_buffer! ()) };
      let mut tail: Lisp_Object = Qnil ();
      let mut other: Lisp_Object = Qnil ();
      itree_insert_gap ((*base).overlays, pos, length, before_markers);
      /* C: `FOR_EACH_LIVE_BUFFER (tail, other) if (...)` -- the expansion (P-22).  */
      tail = Vbuffer_alist;
      while CONSP (tail) && { other = XCDR (XCAR (tail)); true }
	{
	  if (*XBUFFER (other)).base_buffer == base
	    { itree_insert_gap ((*XBUFFER (other)).overlays, pos, length, before_markers); }
	  tail = XCDR (tail);
	}
    }
}

unsafe fn adjust_overlays_for_delete_in_buffer (buf: *mut buffer,
						pos: ptrdiff_t, length: ptrdiff_t)
{
  let mut hit_list: Lisp_Object = Qnil ();
  let mut node: *mut itree_node;

  /* Ideally, the evaporate check would be done directly within
     `itree_delete_gap`, but that code isn't supposed to know about overlays,
     only about `itree_node`s, so it would break an abstraction boundary.  */
  itree_delete_gap ((*buf).overlays, pos, length);

  /* Delete any zero-sized overlays at position POS, if the `evaporate'
     property is set.  */

  /* C: `ITREE_FOREACH (node, buf->overlays, pos, pos, ASCENDING)`.  */
  if (*buf).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*buf).overlays,
				pos, pos, ITREE_ASCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{
	  if (*node).end == pos && (*node).begin == pos
	      && ! NILP (Foverlay_get ((*node).data, Qevaporate ()))
	    { hit_list = Fcons ((*node).data, hit_list); }
	}
    }

  while CONSP (hit_list)
    {
      Fdelete_overlay (XCAR (hit_list));
      hit_list = XCDR (hit_list);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adjust_overlays_for_delete (pos: ptrdiff_t, length: ptrdiff_t)
{
  if (*(current_buffer! ())).indirections == 0
    { adjust_overlays_for_delete_in_buffer ((current_buffer! ()), pos, length); }
  else
    {
      let base: *mut buffer = if ! (*(current_buffer! ())).base_buffer.is_null ()
	{ (*(current_buffer! ())).base_buffer }
	else
	{ (current_buffer! ()) };
      let mut tail: Lisp_Object = Qnil ();
      let mut other: Lisp_Object = Qnil ();
      adjust_overlays_for_delete_in_buffer (base, pos, length);
      /* C: `FOR_EACH_LIVE_BUFFER (tail, other) if (...)` -- the expansion (P-22).  */
      tail = Vbuffer_alist;
      while CONSP (tail) && { other = XCDR (XCAR (tail)); true }
	{
	  if (*XBUFFER (other)).base_buffer == base
	    { adjust_overlays_for_delete_in_buffer (XBUFFER (other), pos, length); }
	  tail = XCDR (tail);
	}
    }
}

DEFUN! ("overlayp", Foverlayp, Soverlayp, 1, 1, 0,
       doc: "Return t if OBJECT is an overlay.",
       (object: Lisp_Object),
       {
  return if OVERLAYP (object) { Qt () } else { Qnil () };
});

DEFUN! ("make-overlay", Fmake_overlay, Smake_overlay, 2, 5, 0,
       doc: "Create a new overlay with range BEG to END in BUFFER and return it.\nIf omitted, BUFFER defaults to the current buffer.\nBEG and END may be integers or markers.\nThe fourth arg FRONT-ADVANCE, if non-nil, makes the marker\nfor the front of the overlay advance when text is inserted there\n(which means the text *is not* included in the overlay).\nThe fifth arg REAR-ADVANCE, if non-nil, makes the marker\nfor the rear of the overlay advance when text is inserted there\n(which means the text *is* included in the overlay).",
       (beg: Lisp_Object, end: Lisp_Object, buffer: Lisp_Object,
        front_advance: Lisp_Object, rear_advance: Lisp_Object),
       {
  let mut ov: Lisp_Object;
  let mut b: *mut buffer;
  let mut beg = beg;      /* C swaps `beg`/`end` below.  */
  let mut end = end;
  let mut buffer = buffer;   /* C's `XSETBUFFER (buffer, current_buffer)` rebinds it.  */

  if NILP (buffer)
    { XSETBUFFER! (buffer, (current_buffer! ())); }
  else
    { CHECK_BUFFER (buffer); }

  b = XBUFFER (buffer);
  if ! BUFFER_LIVE_P (b)
    { error (c"Attempt to create overlay in a dead buffer".as_ptr ()); }

  if MARKERP (beg) && ! BASE_EQ (Fmarker_buffer (beg), buffer)
    { signal_error (c"Marker points into wrong buffer".as_ptr (), beg); }
  if MARKERP (end) && ! BASE_EQ (Fmarker_buffer (end), buffer)
    { signal_error (c"Marker points into wrong buffer".as_ptr (), end); }

  CHECK_FIXNUM_COERCE_MARKER! (beg);
  CHECK_FIXNUM_COERCE_MARKER! (end);

  if XFIXNUM (beg) > XFIXNUM (end)
    {
      let temp: Lisp_Object;
      temp = beg; beg = end; end = temp;
    }

  let obeg: ptrdiff_t = clip_to_bounds (BUF_BEG (b), XFIXNUM (beg), BUF_Z! (b));
  let oend: ptrdiff_t = clip_to_bounds (obeg, XFIXNUM (end), BUF_Z! (b));
  ov = build_overlay (! NILP (front_advance),
		      ! NILP (rear_advance), Qnil ());
  add_buffer_overlay (b, XOVERLAY (ov), obeg, oend);

  /* We don't need to redisplay the region covered by the overlay, because
     the overlay has no properties at the moment.  */

  return ov;
});

/* Mark a section of BUF as needing redisplay because of overlays changes.  */

unsafe fn modify_overlay (buf: *mut buffer, mut start: ptrdiff_t, mut end: ptrdiff_t)
{
  if start > end
    {
      let temp: ptrdiff_t = start;
      start = end;
      end = temp;
    }

  BUF_COMPUTE_UNCHANGED (buf, start, end);

  bset_redisplay (buf as *mut c_void);

  modiff_incr (&raw mut BUF_OVERLAY_MODIFF! (buf), 1);
}

DEFUN! ("move-overlay", Fmove_overlay, Smove_overlay, 3, 4, 0,
       doc: "Set the endpoints of OVERLAY to BEG and END in BUFFER.\nIf BUFFER is omitted, leave OVERLAY in the same buffer it inhabits now.\nIf BUFFER is omitted, and OVERLAY is in no buffer, put it in the current\nbuffer.",
       (overlay: Lisp_Object, beg: Lisp_Object, end: Lisp_Object, buffer: Lisp_Object),
       {
  let mut buffer = buffer;   /* C rebinds `buffer` to `Foverlay_buffer (overlay)`.  */
  let mut b: *mut buffer;
  let mut ob: *mut buffer;
  let mut obuffer: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut o_beg: ptrdiff_t = 0;   /* C: `ptrdiff_t o_beg UNINIT, o_end UNINIT;` */
  let mut o_end: ptrdiff_t = 0;
  let mut beg = beg;   /* C swaps `beg`/`end` below.  */
  let mut end = end;

  CHECK_OVERLAY (overlay);
  if NILP (buffer)
    { buffer = Foverlay_buffer (overlay); }
  if NILP (buffer)
    { XSETBUFFER! (buffer, (current_buffer! ())); }
  CHECK_BUFFER (buffer);

  if NILP (Fbuffer_live_p (buffer))
    { error (c"Attempt to move overlay to a dead buffer".as_ptr ()); }

  if MARKERP (beg) && ! BASE_EQ (Fmarker_buffer (beg), buffer)
    { signal_error (c"Marker points into wrong buffer".as_ptr (), beg); }
  if MARKERP (end) && ! BASE_EQ (Fmarker_buffer (end), buffer)
    { signal_error (c"Marker points into wrong buffer".as_ptr (), end); }

  CHECK_FIXNUM_COERCE_MARKER! (beg);
  CHECK_FIXNUM_COERCE_MARKER! (end);

  if XFIXNUM (beg) > XFIXNUM (end)
    {
      let temp: Lisp_Object;
      temp = beg; beg = end; end = temp;
    }

  specbind (Qinhibit_quit (), Qt ()); /* FIXME: Why?  */

  obuffer = Foverlay_buffer (overlay);
  b = XBUFFER (buffer);

  let n_beg: ptrdiff_t = clip_to_bounds (BUF_BEG (b), XFIXNUM (beg), BUF_Z! (b));
  let n_end: ptrdiff_t = clip_to_bounds (n_beg, XFIXNUM (end), BUF_Z! (b));

  ob = std::ptr::null_mut ();
  if ! NILP (obuffer)
    {
      ob = XBUFFER (obuffer);

      o_beg = OVERLAY_START (overlay);
      o_end = OVERLAY_END (overlay);
    }

  if ! BASE_EQ (buffer, obuffer)
    {
      if ! NILP (obuffer)
	{ remove_buffer_overlay (XBUFFER (obuffer), XOVERLAY (overlay)); }
      add_buffer_overlay (XBUFFER (buffer), XOVERLAY (overlay), n_beg, n_end);
    }
  else
    { itree_node_set_region ((*b).overlays,
			     (*XOVERLAY (overlay)).interval as *mut itree_node,
			     n_beg, n_end); }

  /* If the overlay has changed buffers, do a thorough redisplay.  */
  if NILP (OVERLAY_PLIST (overlay))
    { } /* No props so the overlay doesn't affect the display.  */
  else if ! BASE_EQ (buffer, obuffer)
    {
      /* Redisplay where the overlay was.  */
      if ! ob.is_null ()
	{ modify_overlay (ob, o_beg, o_end); }

      /* Redisplay where the overlay is going to be.  */
      modify_overlay (b, n_beg, n_end);
    }
  else
    /* Redisplay the area the overlay has just left, or just enclosed.  */
    {
      if o_beg == n_beg
	{ modify_overlay (b, o_end, n_end); }
      else if o_end == n_end
	{ modify_overlay (b, o_beg, n_beg); }
      else
	{ modify_overlay (b, o_beg.min (n_beg), o_end.max (n_end)); }
    }

  /* Delete the overlay if it is empty after clipping and has the
     evaporate property.  */
  if n_beg == n_end && ! NILP (Foverlay_get (overlay, Qevaporate ()))
    { /* We used to call `Fdelete_overlay' here, but it causes problems:
         - At this stage, `overlay' is not included in its buffer's lists
           of overlays (the data-structure is in an inconsistent state),
           contrary to `Fdelete_overlay's assumptions.
         - Most of the work done by Fdelete_overlay has already been done
           here for other reasons.  */
      drop_overlay (XOVERLAY (overlay));
      return unbind_to (count, overlay);
    }

  return unbind_to (count, overlay);
});

DEFUN! ("delete-overlay", Fdelete_overlay, Sdelete_overlay, 1, 1, 0,
       doc: "Delete the overlay OVERLAY from its buffer.",
       (overlay: Lisp_Object),
       {
  let mut b: *mut buffer;
  let count: specpdl_ref = SPECPDL_INDEX ();

  CHECK_OVERLAY (overlay);

  b = OVERLAY_BUFFER (overlay);
  if b.is_null ()
    { return Qnil (); }

  specbind (Qinhibit_quit (), Qt ());

  drop_overlay (XOVERLAY (overlay));

  /* When deleting an overlay with before or after strings, turn off
     display optimizations for the affected buffer, on the basis that
     these strings may contain newlines.  This is easier to do than to
     check for that situation during redisplay.  */
  if windows_or_buffers_changed == 0
      && (! NILP (Foverlay_get (overlay, Qbefore_string ()))
	  || ! NILP (Foverlay_get (overlay, Qafter_string ())))
    { (*b).bits |= BUFFER_PREVENT_REDISPLAY_OPTIMIZATIONS_P; }

  return unbind_to (count, Qnil ());
});

DEFUN! ("delete-all-overlays", Fdelete_all_overlays, Sdelete_all_overlays, 0, 1, 0,
       doc: "Delete all overlays of BUFFER.\nBUFFER omitted or nil means delete all overlays of the current\nbuffer.",
       (buffer: Lisp_Object),
       {
  delete_all_overlays (decode_buffer (buffer));
  return Qnil ();
});

/* Overlay dissection functions.  */

DEFUN! ("overlay-start", Foverlay_start, Soverlay_start, 1, 1, 0,
       doc: "Return the position at which OVERLAY starts.",
       (overlay: Lisp_Object),
       {
  CHECK_OVERLAY (overlay);
  if OVERLAY_BUFFER (overlay).is_null ()
    { return Qnil (); }

  return make_fixnum (OVERLAY_START (overlay));
});

DEFUN! ("overlay-end", Foverlay_end, Soverlay_end, 1, 1, 0,
       doc: "Return the position at which OVERLAY ends.",
       (overlay: Lisp_Object),
       {
  CHECK_OVERLAY (overlay);
  if OVERLAY_BUFFER (overlay).is_null ()
    { return Qnil (); }

  return make_fixnum (OVERLAY_END (overlay));
});

DEFUN! ("overlay-buffer", Foverlay_buffer, Soverlay_buffer, 1, 1, 0,
       doc: "Return the buffer OVERLAY belongs to.\nReturn nil if OVERLAY has been deleted.",
       (overlay: Lisp_Object),
       {
  let mut buffer: Lisp_Object;

  CHECK_OVERLAY (overlay);

  if OVERLAY_BUFFER (overlay).is_null ()
    { return Qnil (); }

  XSETBUFFER! (buffer, OVERLAY_BUFFER (overlay) as *mut buffer);

  return buffer;
});

DEFUN! ("overlay-properties", Foverlay_properties, Soverlay_properties, 1, 1, 0,
       doc: "Return a list of the properties on OVERLAY.\nThis is a copy of OVERLAY's plist; modifying its conses has no effect on\nOVERLAY.",
       (overlay: Lisp_Object),
       {
  CHECK_OVERLAY (overlay);

  return Fcopy_sequence (OVERLAY_PLIST (overlay));
});

DEFUN! ("overlays-at", Foverlays_at, Soverlays_at, 1, 2, 0,
       doc: "Return a list of the overlays that contain the character at POS.\nIf SORTED is non-nil, then sort them by decreasing priority.\n\nZero-length (a.k.a. \"empty\") overlays that start and stop at POS are not\nincluded in the return value.  Use `overlays-in' if empty overlays are of\ninterest.\n\nThis function can return overlays outside of the current narrowing of\nthe buffer if POS is outside of the narrowing.",
       (pos: Lisp_Object, sorted: Lisp_Object),
       {
  let mut pos = pos;   /* C's `CHECK_FIXNUM_COERCE_MARKER (pos)` rebinds it.  */
  let mut len: ptrdiff_t;
  let mut noverlays: ptrdiff_t;
  let mut overlay_vec: *mut Lisp_Object;
  let mut result: Lisp_Object;

  CHECK_FIXNUM_COERCE_MARKER! (pos);

  if ! buffer_has_overlays ()
    { return Qnil (); }

  len = 10;
  /* We can't use alloca here because overlays_at can call xrealloc.  */
  overlay_vec = xmalloc ((len * std::mem::size_of::<Lisp_Object> () as ptrdiff_t) as usize) as *mut Lisp_Object;

  /* Put all the overlays we want in a vector in overlay_vec.
     Store the length in len.  */
  noverlays = overlays_at (XFIXNUM (pos), true, &raw mut overlay_vec, &raw mut len,
			   std::ptr::null_mut ());

  if ! NILP (sorted)
    { noverlays = sort_overlays (overlay_vec, noverlays,
				 if WINDOWP (sorted) { XWINDOW (sorted) as *mut c_void }
				 else { std::ptr::null_mut () }); }

  /* Make a list of them all.  */
  result = Flist (noverlays, overlay_vec);

  /* The doc string says the list should be in decreasing order of
     priority, so we reverse the list, because sort_overlays sorts in
     the increasing order of priority.  */
  if ! NILP (sorted)
    { result = Fnreverse (result); }

  xfree (overlay_vec as *mut c_void);
  return result;
});

DEFUN! ("overlays-in", Foverlays_in, Soverlays_in, 2, 2, 0,
       doc: "Return a list of the overlays that overlap the region BEG ... END.\nOverlap means that at least one character between BEG and END is contained\nwithin the overlay.  Note that this excludes the character at (i.e., after)\nEND.\n\nEmpty overlays do not contain any characters, so they are included in the\nresult if they are located at BEG, between BEG and END, or at END provided\nEND denotes the position at the end of the buffer.  (If BEG and END are the\nsame buffer position, an empty overlay at BEG will also be at END, but it\nstill qualifies because it's at BEG.)\n\nThe resulting list of overlays is in an arbitrary unpredictable order.\n\nThis function can return overlays outside of the current narrowing of\nthe buffer if BEG and/or END are outside of the narrowing.",
       (beg: Lisp_Object, end: Lisp_Object),
       {
  let mut beg = beg;   /* C's `CHECK_FIXNUM_COERCE_MARKER` rebinds both.  */
  let mut end = end;
  let mut len: ptrdiff_t;
  let mut noverlays: ptrdiff_t;
  let mut overlay_vec: *mut Lisp_Object;
  let mut result: Lisp_Object;

  CHECK_FIXNUM_COERCE_MARKER! (beg);
  CHECK_FIXNUM_COERCE_MARKER! (end);

  if ! buffer_has_overlays ()
    { return Qnil (); }

  len = 10;
  overlay_vec = xmalloc ((len * std::mem::size_of::<Lisp_Object> () as ptrdiff_t) as usize) as *mut Lisp_Object;

  /* Put all the overlays we want in a vector in overlay_vec.
     Store the length in len.  */
  noverlays = overlays_in (XFIXNUM (beg), XFIXNUM (end), true,
			   &raw mut overlay_vec, &raw mut len,
			   true, false, std::ptr::null_mut ());

  /* Make a list of them all.  */
  result = Flist (noverlays, overlay_vec);

  xfree (overlay_vec as *mut c_void);
  return result;
});

DEFUN! ("next-overlay-change", Fnext_overlay_change, Snext_overlay_change,
       1, 1, 0,
       doc: "Return the next position after POS where an overlay starts or ends.\nIf there are no overlay boundaries from POS to (point-max),\nthe value is (point-max).",
       (pos: Lisp_Object),
       {
  let mut pos = pos;
  CHECK_FIXNUM_COERCE_MARKER! (pos);

  if ! buffer_has_overlays ()
    { return make_fixnum (ZV! ()); }

  return make_fixnum (next_overlay_change (XFIXNUM (pos)));
});

DEFUN! ("previous-overlay-change", Fprevious_overlay_change,
       Sprevious_overlay_change, 1, 1, 0,
       doc: "Return the previous position before POS where an overlay starts or ends.\nIf there are no overlay boundaries from (point-min) to POS,\nthe value is (point-min).",
       (pos: Lisp_Object),
       {
  let mut pos = pos;
  CHECK_FIXNUM_COERCE_MARKER! (pos);

  if ! buffer_has_overlays ()
    { return make_fixnum (BEGV! ()); }

  return make_fixnum (previous_overlay_change (XFIXNUM (pos)));
});

/* These functions are for debugging overlays.  */

DEFUN! ("overlay-lists", Foverlay_lists, Soverlay_lists, 0, 0, 0,
       doc: "Return a list giving all the overlays of the current buffer.\n\nFor backward compatibility, the value is actually a list that\nholds another list; the overlays are in the inner list.\nThe list you get is a copy, so that changing it has no effect.\nHowever, the overlays you get are the real objects that the buffer uses.",
       (),
       {
  let mut overlays: Lisp_Object = Qnil ();
  let mut node: *mut itree_node;

  /* C: `ITREE_FOREACH (node, current_buffer->overlays, BEG, Z, DESCENDING) overlays =
     Fcons (node->data, overlays);` -- the expansion (P-33).  */
  if (*(current_buffer! ())).overlays.is_null ()
    { }
  else
    {
      let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
      let itree_iter_: *mut itree_iterator
	= itree_iterator_start (&raw mut itree_local_iter_, (*(current_buffer! ())).overlays,
				BEG as ptrdiff_t, Z! (), ITREE_DESCENDING);
      while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	{ overlays = Fcons ((*node).data, overlays); }
    }

  return Fcons (overlays, Qnil ());
});

DEFUN! ("overlay-recenter", Foverlay_recenter, Soverlay_recenter, 1, 1, 0,
       doc: "Recenter the overlays of the current buffer around position POS.\nThat makes overlay lookup faster for positions near POS (but perhaps slower\nfor positions far away from POS).\n\nSince Emacs 29.1, this function is a no-op, because the implementation\nof overlays changed and their lookup is now fast regardless of their\nposition in the buffer.  In particular, this function no longer affects\nthe value returned by `overlay-lists'.",
       (pos: Lisp_Object),
       {
  let mut pos = pos;
  CHECK_FIXNUM_COERCE_MARKER! (pos);
  /* Noop */
  return Qnil ();
});

DEFUN! ("overlay-get", Foverlay_get, Soverlay_get, 2, 2, 0,
       doc: "Get the property of overlay OVERLAY with property name PROP.",
       (overlay: Lisp_Object, prop: Lisp_Object),
       {
  CHECK_OVERLAY (overlay);
  return lookup_char_property ((*XOVERLAY (overlay)).plist, prop, false);
});

DEFUN! ("overlay-put", Foverlay_put, Soverlay_put, 3, 3, 0,
       doc: "Set one property of overlay OVERLAY: give property PROP value VALUE.\nVALUE will be returned.",
       (overlay: Lisp_Object, prop: Lisp_Object, value: Lisp_Object),
       {
  let mut tail: Lisp_Object;
  let mut b: *mut buffer;
  let mut changed: bool = false;

  CHECK_OVERLAY (overlay);

  b = OVERLAY_BUFFER (overlay) as *mut buffer;

  'found: loop
    {
      tail = (*XOVERLAY (overlay)).plist;
      while CONSP (tail) && CONSP (XCDR (tail))
	{
	  if EQ (XCAR (tail), prop)
	    {
	      changed = ! EQ (XCAR (XCDR (tail)), value);
	      XSETCAR (XCDR (tail), value);
	      break 'found;
	    }
	  tail = XCDR (XCDR (tail));
	}
      /* It wasn't in the list, so add it to the front.  */
      changed = ! NILP (value);
      set_overlay_plist
	(overlay, Fcons (prop, Fcons (value, (*XOVERLAY (overlay)).plist)));
      break 'found;
    }
  if ! b.is_null ()
    {
      if changed
	{ modify_overlay (b, OVERLAY_START (overlay), OVERLAY_END (overlay)); }
      if EQ (prop, Qevaporate ()) && ! NILP (value)
	  && OVERLAY_START (overlay) == OVERLAY_END (overlay)
	{ Fdelete_overlay (overlay); }
    }

  return value;
});

/* Subroutine of report_overlay_modification.  */

/* Lisp vector holding overlay hook functions to call.
   Vector elements come in pairs.
   Each even-index element is a list of hook functions.
   The following odd-index element is the overlay they came from.

   Before the buffer change, we fill in this vector
   as we call overlay hook functions.
   After the buffer change, we get the functions to call from this vector.
   This way we always call the same functions before and after the change.  */
static mut last_overlay_modification_hooks: Lisp_Object = lisp_h_Qnil ();

/* Number of elements actually used in last_overlay_modification_hooks.  */
static mut last_overlay_modification_hooks_used: ptrdiff_t = 0;

/* Add one functionlist/overlay pair
   to the end of last_overlay_modification_hooks.  */

unsafe fn add_overlay_mod_hooklist (functionlist: Lisp_Object, overlay: Lisp_Object)
{
  let oldsize: ptrdiff_t = ASIZE (last_overlay_modification_hooks) as ptrdiff_t;

  if oldsize - 1 <= last_overlay_modification_hooks_used
    { last_overlay_modification_hooks
	= larger_vector (last_overlay_modification_hooks, 2, -1); }
  ASET (last_overlay_modification_hooks, last_overlay_modification_hooks_used,
	functionlist);
  last_overlay_modification_hooks_used += 1;
  ASET (last_overlay_modification_hooks, last_overlay_modification_hooks_used,
	overlay);
  last_overlay_modification_hooks_used += 1;
}

/* Run the modification-hooks of overlays that include
   any part of the text in START to END.
   If this change is an insertion, also
   run the insert-before-hooks of overlay starting at END,
   and the insert-after-hooks of overlay ending at START.

   This is called both before and after the modification.
   AFTER is true when we call after the modification.

   ARG1, ARG2, ARG3 are arguments to pass to the hook functions.
   When AFTER is nonzero, they are the start position,
   the position after the inserted new text,
   and the length of deleted or replaced old text.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn report_overlay_modification (start: Lisp_Object, end: Lisp_Object,
						     after: bool, arg1: Lisp_Object,
						     arg2: Lisp_Object, arg3: Lisp_Object)
{
  /* True if this change is an insertion.  */
  let insertion: bool = if after { XFIXNAT (arg3) == 0 } else { BASE_EQ (start, end) };

  /* We used to run the functions as soon as we found them and only register
     them in last_overlay_modification_hooks for the purpose of the `after'
     case.  But running elisp code as we traverse the list of overlays is
     painful because the list can be modified by the elisp code so we had to
     copy at several places.  We now simply do a read-only traversal that
     only collects the functions to run and we run them afterwards.  It's
     simpler, especially since all the code was already there.  -stef  */

  if ! after
    {
      let mut node: *mut itree_node;
      let begin_arg: EMACS_INT = XFIXNUM (start);
      let end_arg: EMACS_INT = XFIXNUM (end);
      /* We are being called before a change.
	 Scan the overlays to find the functions to call.  */
      last_overlay_modification_hooks_used = 0;

      if (*(current_buffer! ())).overlays.is_null ()
	{ return; }
      /* C: `ITREE_FOREACH (node, current_buffer->overlays, begin_arg - (insertion ? 1 : 0),
	 end_arg + (insertion ? 1 : 0), ASCENDING)`.  */
      {
	let mut itree_local_iter_: itree_iterator = std::mem::zeroed ();
	let itree_iter_: *mut itree_iterator
	  = itree_iterator_start (&raw mut itree_local_iter_, (*(current_buffer! ())).overlays,
				  begin_arg - if insertion { 1 } else { 0 },
				  end_arg + if insertion { 1 } else { 0 },
				  ITREE_ASCENDING);
	while { node = itree_iterator_next (itree_iter_); !node.is_null () }
	  {
	    let overlay: Lisp_Object = (*node).data;
	    let obegin: ptrdiff_t = OVERLAY_START (overlay);
	    let oend: ptrdiff_t = OVERLAY_END (overlay);

	    if insertion && (begin_arg == obegin as EMACS_INT
			     || end_arg == obegin as EMACS_INT)
	      {
		let prop: Lisp_Object = Foverlay_get (overlay, Qinsert_in_front_hooks ());
		if ! NILP (prop)
		  { add_overlay_mod_hooklist (prop, overlay); }
	      }
	    if insertion && (begin_arg == oend as EMACS_INT
			     || end_arg == oend as EMACS_INT)
	      {
		let prop: Lisp_Object = Foverlay_get (overlay, Qinsert_behind_hooks ());
		if ! NILP (prop)
		  { add_overlay_mod_hooklist (prop, overlay); }
	      }
	    /* Test for intersecting intervals.  This does the right thing
	       for both insertion and deletion.  */
	    if end_arg > obegin as EMACS_INT && begin_arg < oend as EMACS_INT
	      {
		let prop: Lisp_Object = Foverlay_get (overlay, Qmodification_hooks ());
		if ! NILP (prop)
		  { add_overlay_mod_hooklist (prop, overlay); }
	      }
	  }
      }
    }
  {
    /* Call the functions recorded in last_overlay_modification_hooks.
       First copy the vector contents, in case some of these hooks
       do subsequent modification of the buffer.  */
    let size: ptrdiff_t = last_overlay_modification_hooks_used;
    let mut copy: *mut Lisp_Object;
    let mut i: ptrdiff_t;

    USE_SAFE_ALLOCA! (sa_avail, sa_count);
    SAFE_ALLOCA_LISP! (sa_avail, copy, size);
    memcpy (copy as *mut c_void,
	    (*XVECTOR (last_overlay_modification_hooks)).contents.as_ptr () as *const c_void,
	    (size * word_size) as usize);

    i = 0;
    while i < size
      {
	let prop_i: Lisp_Object;
	let overlay_i: Lisp_Object;
	prop_i = *copy.add (i as usize);
	i += 1;
	overlay_i = *copy.add (i as usize);
	i += 1;
	/* It is possible that the recorded overlay has been deleted
	   (which makes its markers' buffers be nil), or that (due to
	   some bug) it belongs to a different buffer.  Only run this
	   hook if the overlay belongs to the current buffer.  */
	if (*XOVERLAY (overlay_i)).buffer as *mut buffer == (current_buffer! ())
	  { call_overlay_mod_hooks (prop_i, overlay_i, after, arg1, arg2, arg3); }
      }

    SAFE_FREE! (sa_count);
  }
}

unsafe fn call_overlay_mod_hooks (list: Lisp_Object, overlay: Lisp_Object, after: bool,
				  arg1: Lisp_Object, arg2: Lisp_Object, arg3: Lisp_Object)
{
  let mut list = list;
  while CONSP (list)
    {
      if NILP (arg3)
	{ calln! (XCAR (list), overlay, if after { Qt () } else { Qnil () }, arg1, arg2); }
      else
	{ calln! (XCAR (list), overlay, if after { Qt () } else { Qnil () }, arg1, arg2, arg3); }
      list = XCDR (list);
    }
}

/***********************************************************************
			 Allocation with mmap
 ***********************************************************************/

// C 4260-4609 is `#if defined USE_MMAP_FOR_BUFFERS && !defined WINDOWSNT` (undef in the
// frozen configuration, macros.tsv): `struct mmap_region` and the whole
// mmap_init/mmap_alloc/mmap_free/mmap_realloc/mmap_set_vars/mmap_reset_vars allocator.
// Recorded in docs/ports/buffer.md, not transcribed (P-08).

/***********************************************************************
			    Buffer-text Allocation
 ***********************************************************************/

/* Allocate NBYTES bytes for buffer B's text buffer.  */

unsafe fn alloc_buffer_text (b: *mut buffer, nbytes: ptrdiff_t)
{
  let mut p: *mut c_void;

  block_input ();
  // C 4624-4630's `#if defined USE_MMAP_FOR_BUFFERS` / `#elif defined REL_ALLOC` are both
  // undef, so the port keeps the `#else` arm (`p = xmalloc (nbytes);`).

  p = xmalloc (nbytes as usize);

  if p.is_null ()
    {
      unblock_input ();
      memory_full (nbytes as usize);
    }

  (*(*b).text).beg = p as *mut u8;
  unblock_input ();
}

/* Enlarge buffer B's text buffer by DELTA bytes.  DELTA < 0 means
   shrink it.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn enlarge_buffer_text (b: *mut buffer, delta: ptrdiff_t)
{
  block_input ();
  let mut p: *mut c_void;
  let mut old_beg: *mut u8 = (*(*b).text).beg;
  let old_nbytes: ptrdiff_t
    = BUF_Z_BYTE! (b) - BUF_BEG_BYTE (b) + BUF_GAP_SIZE! (b) + 1;
  let new_nbytes: ptrdiff_t = old_nbytes + delta;

  if pdumper_object_p (old_beg as *mut c_void)
    { (*(*b).text).beg = std::ptr::null_mut (); }
  else
    { old_beg = std::ptr::null_mut (); }

  // C 4650-4656's `#if defined USE_MMAP_FOR_BUFFERS` / `#elif defined REL_ALLOC` are both
  // undef, so the port keeps the `#else` arm.
  p = xrealloc ((*(*b).text).beg as *mut c_void, new_nbytes as usize);
  __lsan_ignore_object (p);

  if p.is_null ()
    {
      if ! old_beg.is_null ()
	{ (*(*b).text).beg = old_beg; }
      unblock_input ();
      memory_full (new_nbytes as usize);
    }

  if ! old_beg.is_null ()
    { memcpy (p, old_beg as *const c_void, old_nbytes.min (new_nbytes) as usize); }

  BUF_BEG_ADDR! (b) = p as *mut u8;
  unblock_input ();
}

/* Free buffer B's text buffer.  */

unsafe fn free_buffer_text (b: *mut buffer)
{
  block_input ();

  if ! pdumper_object_p ((*(*b).text).beg as *mut c_void)
    {
      // C 4695-4701's `#if defined USE_MMAP_FOR_BUFFERS` / `#elif defined REL_ALLOC` are
      // both undef, so the port keeps the `#else` arm.
      xfree ((*(*b).text).beg as *mut c_void);
    }

  BUF_BEG_ADDR! (b) = std::ptr::null_mut ();
  unblock_input ();
}

/***********************************************************************
			    Initialization
 ***********************************************************************/
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_buffer_once ()
{
  /* TODO: clean up the buffer-local machinery.  Right now,
     we have:

     buffer_defaults: default values of buffer-locals
     buffer_local_flags: metadata
     buffer_permanent_local_flags: metadata
     buffer_local_symbols: metadata

     There must be a simpler way to store the metadata.
  */

  let mut idx: c_int;

  /* Items flagged permanent get an explicit permanent-local property
     added in bindings.el, for clarity.  */
  PDUMPER_REMEMBER_SCALAR! (buffer_permanent_local_flags);
  memset (buffer_permanent_local_flags.as_mut_ptr () as *mut c_void, 0,
	  std::mem::size_of_val (&buffer_permanent_local_flags));

  /* 0 means not a lisp var, -1 means always local, else mask.  */
  memset (&raw mut buffer_local_flags as *mut c_void, 0,
	  std::mem::size_of_val (&buffer_local_flags));
  bset_filename (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_directory (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_backed_up (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_save_length (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_auto_save_file_name (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_read_only (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_major_mode (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_local_minor_modes (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_mode_name (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_undo_list (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_mark_active (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_point_before_scroll (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_file_truename (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_invisibility_spec (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_file_format (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_auto_save_file_format (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_display_count (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_display_time (&raw mut buffer_local_flags, make_fixnum (-1));
  bset_enable_multibyte_characters (&raw mut buffer_local_flags, make_fixnum (-1));

  /* These used to be stuck at 0 by default, but now that the all-zero value
     means Qnil, we have to initialize them explicitly.  */
  bset_name (&raw mut buffer_local_flags, make_fixnum (0));
  bset_last_name (&raw mut buffer_local_flags, make_fixnum (0));
  bset_mark (&raw mut buffer_local_flags, make_fixnum (0));
  bset_local_var_alist (&raw mut buffer_local_flags, make_fixnum (0));
  bset_keymap (&raw mut buffer_local_flags, make_fixnum (0));
  bset_downcase_table (&raw mut buffer_local_flags, make_fixnum (0));
  bset_upcase_table (&raw mut buffer_local_flags, make_fixnum (0));
  bset_case_canon_table (&raw mut buffer_local_flags, make_fixnum (0));
  bset_case_eqv_table (&raw mut buffer_local_flags, make_fixnum (0));
  bset_width_table (&raw mut buffer_local_flags, make_fixnum (0));
  bset_pt_marker (&raw mut buffer_local_flags, make_fixnum (0));
  bset_begv_marker (&raw mut buffer_local_flags, make_fixnum (0));
  bset_zv_marker (&raw mut buffer_local_flags, make_fixnum (0));
  bset_last_selected_window (&raw mut buffer_local_flags, make_fixnum (0));

  idx = 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, mode_line_format_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, abbrev_mode_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, overwrite_mode_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, auto_fill_function_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, selective_display_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, selective_display_ellipses_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, tab_width_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, truncate_lines_), idx as EMACS_INT);
  /* Make this one a permanent local.  */
  buffer_permanent_local_flags[idx as usize] = 1 as c_char; idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, word_wrap_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, ctl_arrow_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, fill_column_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, left_margin_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, abbrev_table_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, display_table_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, syntax_table_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, cache_long_scans_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, category_table_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, bidi_display_reordering_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, bidi_paragraph_direction_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, bidi_paragraph_separate_re_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, bidi_paragraph_start_re_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, buffer_file_coding_system_), idx as EMACS_INT);
  /* Make this one a permanent local.  */
  buffer_permanent_local_flags[idx as usize] = 1 as c_char; idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, left_margin_cols_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, right_margin_cols_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, left_fringe_width_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, right_fringe_width_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, fringes_outside_margins_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, scroll_bar_width_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, scroll_bar_height_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, vertical_scroll_bar_type_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, horizontal_scroll_bar_type_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, indicate_empty_lines_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, indicate_buffer_boundaries_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, fringe_indicator_alist_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, fringe_cursor_alist_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, scroll_up_aggressively_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, scroll_down_aggressively_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, header_line_format_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, tab_line_format_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, cursor_type_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, extra_line_spacing_), idx as EMACS_INT); idx += 1;
  // C 4819-4821 is `#ifdef HAVE_TREE_SITTER` (undef): `XSETFASTINT (BVAR
  // (&buffer_local_flags, ts_parser_list), idx); ++idx;`
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, text_conversion_style_), idx as EMACS_INT); idx += 1;
  XSETFASTINT! (BVAR! (&raw mut buffer_local_flags, cursor_in_non_selected_windows_), idx as EMACS_INT); idx += 1;

  /* buffer_local_flags contains no pointers, so it's safe to treat it
     as a blob for pdumper.  */
  PDUMPER_REMEMBER_SCALAR! (buffer_local_flags);

  /* Need more room? */
  if idx >= MAX_PER_BUFFER_VARS
    { emacs_abort (); }
  last_per_buffer_idx = idx;
  PDUMPER_REMEMBER_SCALAR! (last_per_buffer_idx);

  /* Make sure all markable slots in buffer_defaults
     are initialized reasonably, so mark_buffer won't choke.  */
  reset_buffer (&raw mut buffer_defaults);
  eassert! (NILP (BVAR! (&raw mut buffer_defaults, name_)));
  reset_buffer_local_variables (&raw mut buffer_defaults, 1);
  eassert! (NILP (BVAR! (&raw mut buffer_local_symbols, name_)));
  reset_buffer (&raw mut buffer_local_symbols);
  reset_buffer_local_variables (&raw mut buffer_local_symbols, 1);
  /* Prevent GC from getting confused.  */
  buffer_defaults.text = &raw mut buffer_defaults.own_text;
  buffer_local_symbols.text = &raw mut buffer_local_symbols.own_text;
  /* No one will share the text with these buffers, but let's play it safe.  */
  buffer_defaults.indirections = 0;
  buffer_local_symbols.indirections = 0;
  /* Likewise no one will display them.  */
  buffer_defaults.window_count = 0;
  buffer_local_symbols.window_count = 0;
  set_buffer_intervals (&raw mut buffer_defaults, std::ptr::null_mut ());
  set_buffer_intervals (&raw mut buffer_local_symbols, std::ptr::null_mut ());
  /* This is not strictly necessary, but let's make them initialized.  */
  bset_name (&raw mut buffer_defaults, build_string (c" *buffer-defaults*".as_ptr ()));
  bset_name (&raw mut buffer_local_symbols, build_string (c" *buffer-local-symbols*".as_ptr ()));
  BUFFER_PVEC_INIT (&raw mut buffer_defaults);
  BUFFER_PVEC_INIT (&raw mut buffer_local_symbols);

  /* Set up the default values of various buffer slots.  */
  /* Must do these before making the first buffer! */

  /* real setup is done in bindings.el */
  bset_mode_line_format (&raw mut buffer_defaults, build_string (c"%-".as_ptr ()));
  bset_header_line_format (&raw mut buffer_defaults, Qnil ());
  bset_tab_line_format (&raw mut buffer_defaults, Qnil ());
  bset_abbrev_mode (&raw mut buffer_defaults, Qnil ());
  bset_overwrite_mode (&raw mut buffer_defaults, Qnil ());
  bset_auto_fill_function (&raw mut buffer_defaults, Qnil ());
  bset_selective_display (&raw mut buffer_defaults, Qnil ());
  bset_selective_display_ellipses (&raw mut buffer_defaults, Qt ());
  bset_abbrev_table (&raw mut buffer_defaults, Qnil ());
  bset_display_table (&raw mut buffer_defaults, Qnil ());
  bset_undo_list (&raw mut buffer_defaults, Qnil ());
  bset_mark_active (&raw mut buffer_defaults, Qnil ());
  bset_file_format (&raw mut buffer_defaults, Qnil ());
  bset_auto_save_file_format (&raw mut buffer_defaults, Qt ());
  buffer_defaults.overlays = std::ptr::null_mut ();

  XSETFASTINT! (BVAR! (&raw mut buffer_defaults, tab_width_), 8);
  bset_truncate_lines (&raw mut buffer_defaults, Qnil ());
  bset_word_wrap (&raw mut buffer_defaults, Qnil ());
  bset_ctl_arrow (&raw mut buffer_defaults, Qt ());
  bset_bidi_display_reordering (&raw mut buffer_defaults, Qt ());
  bset_bidi_paragraph_direction (&raw mut buffer_defaults, Qnil ());
  bset_bidi_paragraph_start_re (&raw mut buffer_defaults, Qnil ());
  bset_bidi_paragraph_separate_re (&raw mut buffer_defaults, Qnil ());
  bset_cursor_type (&raw mut buffer_defaults, Qt ());
  bset_extra_line_spacing (&raw mut buffer_defaults, Qnil ());
  // C 4890-4892 is `#ifdef HAVE_TREE_SITTER` (undef): `bset_ts_parser_list (...)`.
  bset_text_conversion_style (&raw mut buffer_defaults, Qnil ());
  bset_cursor_in_non_selected_windows (&raw mut buffer_defaults, Qt ());

  bset_enable_multibyte_characters (&raw mut buffer_defaults, Qt ());
  bset_buffer_file_coding_system (&raw mut buffer_defaults, Qnil ());
  XSETFASTINT! (BVAR! (&raw mut buffer_defaults, fill_column_), 70);
  XSETFASTINT! (BVAR! (&raw mut buffer_defaults, left_margin_), 0);
  bset_cache_long_scans (&raw mut buffer_defaults, Qt ());
  bset_file_truename (&raw mut buffer_defaults, Qnil ());
  XSETFASTINT! (BVAR! (&raw mut buffer_defaults, display_count_), 0);
  XSETFASTINT! (BVAR! (&raw mut buffer_defaults, left_margin_cols_), 0);
  XSETFASTINT! (BVAR! (&raw mut buffer_defaults, right_margin_cols_), 0);
  bset_left_fringe_width (&raw mut buffer_defaults, Qnil ());
  bset_right_fringe_width (&raw mut buffer_defaults, Qnil ());
  bset_fringes_outside_margins (&raw mut buffer_defaults, Qnil ());
  bset_scroll_bar_width (&raw mut buffer_defaults, Qnil ());
  bset_scroll_bar_height (&raw mut buffer_defaults, Qnil ());
  bset_vertical_scroll_bar_type (&raw mut buffer_defaults, Qt ());
  bset_horizontal_scroll_bar_type (&raw mut buffer_defaults, Qt ());
  bset_indicate_empty_lines (&raw mut buffer_defaults, Qnil ());
  bset_indicate_buffer_boundaries (&raw mut buffer_defaults, Qnil ());
  bset_fringe_indicator_alist (&raw mut buffer_defaults, Qnil ());
  bset_fringe_cursor_alist (&raw mut buffer_defaults, Qnil ());
  bset_scroll_up_aggressively (&raw mut buffer_defaults, Qnil ());
  bset_scroll_down_aggressively (&raw mut buffer_defaults, Qnil ());
  bset_display_time (&raw mut buffer_defaults, Qnil ());

  /* Assign the local-flags to the slots that have default values.
     The local flag is a bit that is used in the buffer
     to say that it has its own local value for the slot.
     The local flag bits are in the local_var_flags slot of the buffer.  */

  /* Nothing can work if this isn't true.  */
  { static_assert! (std::mem::size_of::<EMACS_INT> () == word_size as usize); }

  Vbuffer_alist = Qnil ();
  set_current_buffer! (std::ptr::null_mut ());                                            /* C 4929 */
  pdumper_remember_lv_ptr_raw ((&raw mut (*current_thread).m_current_buffer) as *mut c_void,
			      Lisp_Type::Lisp_Vectorlike);

  QSFundamental = build_string (c"Fundamental".as_ptr ());

  DEFSYM! (Qfundamental_mode, c"fundamental-mode");
  bset_major_mode (&raw mut buffer_defaults, Qfundamental_mode ());

  DEFSYM! (Qmode_class, c"mode-class");
  DEFSYM! (Qprotected_field, c"protected-field");

  DEFSYM! (Qpermanent_local, c"permanent-local");
  DEFSYM! (Qkill_buffer_hook, c"kill-buffer-hook");
  Fput (Qkill_buffer_hook (), Qpermanent_local (), Qt ());

  /* Super-magic invisible buffer.  */
  Vprin1_to_string_buffer =
    Fget_buffer_create (build_string (c" prin1".as_ptr ()), Qt ());
  Vbuffer_alist = Qnil ();

  Fset_buffer (Fget_buffer_create (build_string (c"*scratch*".as_ptr ()), Qnil ()));

  inhibit_modification_hooks = false;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_buffer ()
{
  let mut temp: Lisp_Object;

  AUTO_STRING! (scratch, c"*scratch*".as_ptr ());
  Fset_buffer (Fget_buffer_create (scratch, Qnil ()));
  if NILP (BVAR! (&raw mut buffer_defaults, enable_multibyte_characters_))
    { Fset_buffer_multibyte (Qnil ()); }

  let pwd: *const c_char = emacs_wd;

  if pwd.is_null ()
    {
      fprintf (stderr, c"Error getting directory: %s\n".as_ptr (),
	       emacs_strerror (*errno ()));
      bset_directory ((current_buffer! ()), Qnil ());
    }
  else
    {
      /* Maybe this should really use some standard subroutine
         whose definition is filename syntax dependent.  */
      let len: ptrdiff_t = strlen (pwd) as ptrdiff_t;
      let add_slash: bool = ! IS_DIRECTORY_SEP (*pwd.add (len as usize - 1) as c_int);

      /* At this moment, we still don't know how to decode the directory
         name.  So, we keep the bytes in unibyte form so that file I/O
         routines correctly get the original bytes.  */
      let dirname: Lisp_Object = make_unibyte_string (pwd,
						      len + add_slash as ptrdiff_t);
      if add_slash
	{ SSET (dirname, len as usize, DIRECTORY_SEP as u8); }
      bset_directory ((current_buffer! ()), dirname);

      /* Add /: to the front of the name
         if it would otherwise be treated as magic.  */
      temp = Ffind_file_name_handler (BVAR! ((current_buffer! ()), directory_), Qt ());
      if ! NILP (temp)
          /* If the default dir is just /, TEMP is non-nil
             because of the ange-ftp completion handler.
             However, it is not necessary to turn / into /:/.
             So avoid doing that.  */
          && strcmp (c"/".as_ptr (), SSDATA (BVAR! ((current_buffer! ()), directory_)) as *const c_char) != 0
        {
          AUTO_STRING! (slash_colon, c"/:".as_ptr ());
          bset_directory ((current_buffer! ()),
                          concat2 (slash_colon,
                                   BVAR! ((current_buffer! ()), directory_)));
        }
    }

  temp = get_minibuffer (0);
  bset_directory (XBUFFER (temp), BVAR! ((current_buffer! ()), directory_));
}

/// C: sys/fileio.h 69: `#define DIRECTORY_SEP '/'` and 87:
/// `#define IS_DIRECTORY_SEP(c) ((c) == DIRECTORY_SEP)`.  That header has no mirror; the two
/// names are declared here (the insdel.rs shape) and reported in docs/ports/buffer.md.
const DIRECTORY_SEP: c_char = b'/' as c_char;
#[inline(always)]
fn IS_DIRECTORY_SEP (c: c_int) -> bool { c == DIRECTORY_SEP as c_int }

/* Similar to defvar_lisp but define a variable whose value is the
   Lisp_Object stored in the current buffer.  LNAME is the Lisp-level
   variable name.  VNAME is the name of the buffer slot.  PREDICATE
   is nil for a general Lisp variable.  If PREDICATE is non-nil, then
   only Lisp values that satisfies the PREDICATE are allowed (except
   that nil is allowed too).  DOC is a dummy where you write the doc
   string as a comment.  */

// C: `#define DEFVAR_PER_BUFFER(lname, vname, predicate_, doc)` pastes `vname##_` and
// `FWDPRED_##predicate_`, which `macro_rules!` cannot do (PORTING.md P-37): the port's macro
// takes the pasted `PER_BUFFER_VAR_OFFSET!` value and the `FWDPRED_` constant as arguments, so
// each call site spells them with the names C pastes (`name_`, `FWDPRED_Qnil`, …) and the
// body keeps C's statements.
macro_rules! DEFVAR_PER_BUFFER {
  ($lname:expr, $vname:ident, $predicate:ident, doc: $doc:literal) => {{
    static mut BO_FWD: Lisp_Fwd = Lisp_Fwd {
      r#type: Lisp_Fwd_Buffer_Obj,
      u: Lisp_Fwd_u { buf: Lisp_Fwd_buf { offset: 0, predicate: 0, _pad: 0 } },
    };
    static_assert! (std::mem::offset_of! (buffer, $vname)
                    < (1 << 8 * std::mem::size_of::<u16> ()));
    unsafe {
      BO_FWD.u.buf.offset = std::mem::offset_of! (buffer, $vname) as u16;
      BO_FWD.u.buf.predicate = FWDPRED! ($predicate) as u8;
      defvar_per_buffer (&raw const BO_FWD, $lname.as_ptr () as *const c_char);
    }
  }};
}

// C: `FWDPRED_##predicate_` (lisp.h 3099's `enum Lisp_Fwd_Predicate`, mirrored as the
// `FWDPRED_*` constants in lisp.rs).  `macro_rules!` cannot paste the argument into the
// constant's name, so the call site passes C's own predicate symbol and this table resolves
// it; the `DEFVAR_PER_BUFFER` body then mentions `FWDPRED` exactly as C's does.
macro_rules! FWDPRED {
  (Qnil) => { FWDPRED_Qnil };
  (Qintegerp) => { FWDPRED_Qintegerp };
  (Qsymbolp) => { FWDPRED_Qsymbolp };
  (Qstringp) => { FWDPRED_Qstringp };
  (Qfraction) => { FWDPRED_Qfraction };
  (Qoverwrite_mode) => { FWDPRED_Qoverwrite_mode };
  (Qvertical_scroll_bar) => { FWDPRED_Qvertical_scroll_bar };
}

unsafe fn defvar_per_buffer (fwd: *const Lisp_Fwd, namestring: *const c_char)
{
  eassert! ((*fwd).r#type == Lisp_Fwd_Buffer_Obj);
  let sym: *mut Lisp_Symbol = XSYMBOL (intern (namestring));

  (*sym).u.s.bits |= SYMBOL_DECLARED_SPECIAL_BIT;
  (*sym).u.s.bits = ((*sym).u.s.bits & ! SYMBOL_REDIRECT_MASK)
    | (SYMBOL_FORWARDED << SYMBOL_REDIRECT_SHIFT);
  SET_SYMBOL_FWD (sym, fwd as *mut c_void);
  XSETSYMBOL! (PER_BUFFER_SYMBOL! (XBUFFER_OFFSET (fwd as *mut Lisp_Fwd)), sym);

  if PER_BUFFER_IDX (XBUFFER_OFFSET (fwd as *mut Lisp_Fwd) as ptrdiff_t) == 0
    /* Did a DEFVAR_PER_BUFFER without initializing the corresponding
       slot of buffer_local_flags.  */
    { emacs_abort (); }
}

// C 5047-5089 and C 6145-6163 are `#ifdef ITREE_DEBUG` (undefined in the frozen
// configuration): `make_lispy_itree_node`, `overlay_tree` and the `overlay-tree` subr.
// Recorded in docs/ports/buffer.md, not transcribed (P-08).

/* The storage this unit owns (docs/status.md 3.29): C's `DEFVAR_LISP`/`DEFVAR_INT`/
   `DEFVAR_BOOL` name a field of the single `globals` object (`lisp.h` 3495-3530), whose
   declaration the port mirrors in `include/globals.rs`'s `variables` module; the unit whose
   `syms_of_buffer` registers the variable defines the object here, and its address must be
   stable for `staticpro`/the forwarding records.  C zero-initialises the `globals` object, so
   the port's initialisers are `lisp_h_Qnil ()`/`false`/`0`; the values `syms_of_buffer` sets
   below are C's own.  */
#[unsafe(no_mangle)]
pub static mut Vbefore_change_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vafter_change_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vfirst_change_hook: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vtransient_mark_mode: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vinhibit_read_only: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vkill_buffer_query_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vchange_major_mode_hook: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vbuffer_list_update_hook: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vcase_fold_search: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vclone_indirect_buffer_hook: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vlong_line_threshold: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut kill_buffer_delete_auto_save_files: bool = false;
#[unsafe(no_mangle)]
pub static mut delete_auto_save_files: bool = false;
#[unsafe(no_mangle)]
pub static mut long_line_optimizations_region_size: EMACS_INT = 0;
#[unsafe(no_mangle)]
pub static mut long_line_optimizations_bol_search_limit: EMACS_INT = 0;
#[unsafe(no_mangle)]
pub static mut large_hscroll_threshold: EMACS_INT = 0;

/* Initialize the buffer routines.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_buffer ()
{
  staticpro (&raw const last_overlay_modification_hooks);
  last_overlay_modification_hooks = make_nil_vector (10);
  staticpro (&raw const QSFundamental);
  staticpro (&raw const Vbuffer_alist);
  DEFSYM! (Qchoice, c"choice");
  DEFSYM! (Qleft, c"left");
  DEFSYM! (Qright, c"right");
  DEFSYM! (Qrange, c"range");
  DEFSYM! (Qpermanent_local_hook, c"permanent-local-hook");
  DEFSYM! (Qoverlayp, c"overlayp");
  DEFSYM! (Qevaporate, c"evaporate");
  DEFSYM! (Qmodification_hooks, c"modification-hooks");
  DEFSYM! (Qinsert_in_front_hooks, c"insert-in-front-hooks");
  DEFSYM! (Qinsert_behind_hooks, c"insert-behind-hooks");
  DEFSYM! (Qget_file_buffer, c"get-file-buffer");
  DEFSYM! (Qpriority, c"priority");
  DEFSYM! (Qbefore_string, c"before-string");
  DEFSYM! (Qafter_string, c"after-string");
  DEFSYM! (Qfirst_change_hook, c"first-change-hook");
  DEFSYM! (Qbefore_change_functions, c"before-change-functions");
  DEFSYM! (Qafter_change_functions, c"after-change-functions");
  DEFSYM! (Qkill_buffer_query_functions, c"kill-buffer-query-functions");
  DEFSYM! (Qget_scratch_buffer_create, c"get-scratch-buffer-create");
  DEFSYM! (Qvertical_scroll_bar, c"vertical-scroll-bar");
  Fput (Qvertical_scroll_bar (), Qchoice (), list4 (Qnil (), Qt (), Qleft (), Qright ()));
  DEFSYM! (Qhorizontal_scroll_bar, c"horizontal-scroll-bar");
  DEFSYM! (Qfraction, c"fraction");
  Fput (Qfraction (), Qrange (), Fcons (make_float (0.0), make_float (1.0)));
  DEFSYM! (Qoverwrite_mode, c"overwrite-mode");
  Fput (Qoverwrite_mode (), Qchoice (), list3 (Qnil (), intern (c"overwrite-mode-textual".as_ptr ()), Qoverwrite_mode_binary ()));
  Fput (Qprotected_field (), Qerror_conditions (), list! (Qprotected_field (), Qerror ()));
  Fput (Qprotected_field (), Qerror_message (), build_string (c"Attempt to modify a protected field".as_ptr ()));
  DEFSYM! (Qclone_indirect_buffer_hook, c"clone-indirect-buffer-hook");
  DEFVAR_PER_BUFFER! (c"tab-line-format", tab_line_format_, Qnil,
		     doc: "Analogous to `mode-line-format', but controls the tab line.\nThe tab line appears, optionally, at the top of a window;\nthe mode line appears at the bottom.");
  DEFVAR_PER_BUFFER! (c"header-line-format", header_line_format_, Qnil,
		     doc: "Analogous to `mode-line-format', but controls the header line.\nThe header line appears, optionally, at the top of a window; the mode\nline appears at the bottom.\n\nAlso see `header-line-indent-mode' if `display-line-numbers-mode' is\nturned on and header-line text should be aligned with buffer text.");
  DEFVAR_PER_BUFFER! (c"mode-line-format", mode_line_format_, Qnil,
		     doc: "Template for displaying mode line for a window's buffer.\n\nThe value may be nil, a string, a symbol or a list.\n\nA value of nil means don't display a mode line.\n\nFor any symbol other than t or nil, the symbol's value is processed as\n a mode line construct.  As a special exception, if that value is a\n string, the string is processed verbatim, without handling any\n %-constructs (see below).  Also, unless the symbol has a non-nil\n `risky-local-variable' property, all properties in any strings, as\n well as all :eval and :propertize forms in the value, are ignored.\n\nWhen the value is processed, the window's buffer is temporarily the\ncurrent buffer.\n\nA list whose car is a string or list is processed by processing each\n of the list elements recursively, as separate mode line constructs,\n and concatenating the results.\n\nA list of the form `(:eval FORM)' is processed by evaluating FORM and\n using the result as a mode line construct.  Be careful--FORM should\n not load any files, because that can cause an infinite recursion.\n\nA list of the form `(:propertize ELT PROPS...)' is processed by\n processing ELT as the mode line construct, and adding the text\n properties PROPS to the result.\n\nA list whose car is a symbol is processed by examining the symbol's\n value, and, if that value is non-nil, processing the cadr of the list\n recursively; and if that value is nil, processing the caddr of the\n list recursively.\n\nA list whose car is an integer is processed by processing the cadr of\n the list, and padding (if the number is positive) or truncating (if\n negative) to the width specified by that number.\n\nA string is printed verbatim in the mode line except for %-constructs:\n  %b -- print buffer name.\n  %c -- print the current column number (this makes editing slower).\n        Columns are numbered starting from the left margin, and the\n        leftmost column is displayed as zero.\n        To make the column number update correctly in all cases,\n        `column-number-mode' must be non-nil.\n  %C -- Like %c, but the leftmost column is displayed as one.\n  %e -- print error message about full memory.\n  %f -- print visited file name.\n  %F -- print frame name.\n  %i -- print the size of the buffer.\n  %I -- like %i, but use k, M, G, etc., to abbreviate.\n  %l -- print the current line number.\n  %n -- print Narrow if appropriate.\n  %o -- print percent of window travel through buffer, or Top, Bot or All.\n  %p -- print percent of buffer above top of window, or Top, Bot or All.\n  %P -- print percent of buffer above bottom of window, perhaps plus Top,\n        or print Bottom or All.\n  %q -- print percent of buffer above both the top and the bottom of the\n        window, separated by `-', or `All'.\n  %s -- print process status.\n  %z -- print mnemonics of keyboard, terminal, and buffer coding systems.\n  %Z -- like %z, but including the end-of-line format.\n  %& -- print * if the buffer is modified, otherwise hyphen.\n  %+ -- print *, % or hyphen (modified, read-only, neither).\n  %* -- print %, * or hyphen (read-only, modified, neither).\n        For a modified read-only buffer, %+ prints * and %* prints %.\n  %@ -- print @ if default-directory is on a remote machine, else hyphen.\n  %[ -- print one [ for each recursive editing level.\n  %] -- print one ] for each recursive editing level.\n  %- -- print enough dashes to fill the mode line.\n  %% -- print %.\nDecimal digits after the % specify field width to which to pad.");
  DEFVAR_PER_BUFFER! (c"major-mode", major_mode_, Qsymbolp,
		     doc: "Symbol for current buffer's major mode.\nThe default value (normally `fundamental-mode') affects new buffers.\nA value of nil means to use the current buffer's major mode, provided\nit is not marked as \"special\".");
  DEFVAR_PER_BUFFER! (c"local-minor-modes", local_minor_modes_, Qnil,
		     doc: "Minor modes currently active in the current buffer.\nThis is a list of symbols, or nil if there are no minor modes active.");
  DEFVAR_PER_BUFFER! (c"mode-name", mode_name_, Qnil,
		     doc: "Pretty name of current buffer's major mode.\nUsually a string, but can use any of the constructs for `mode-line-format',\nwhich see.\nFormat with `format-mode-line' to produce a string value.");
  DEFVAR_PER_BUFFER! (c"local-abbrev-table", abbrev_table_, Qnil,
		     doc: "Local (mode-specific) abbrev table of current buffer.");
  DEFVAR_PER_BUFFER! (c"abbrev-mode", abbrev_mode_, Qnil,
		     doc: "Non-nil if Abbrev mode is enabled.\nUse the command `abbrev-mode' to change the value of this variable in\nthe current buffer.  Customize this variable to non-nil to enable Abbrev\nmode by default in all buffers.");
  DEFVAR_PER_BUFFER! (c"fill-column", fill_column_, Qintegerp,
		     doc: "Column beyond which automatic line-wrapping should happen.\nIt is used by filling commands, such as `fill-region' and `fill-paragraph',\nand by `auto-fill-mode', which see.\nSee also `current-fill-column'.\nInteractively, you can set the buffer local value using \\[set-fill-column].");
  DEFVAR_PER_BUFFER! (c"left-margin", left_margin_, Qintegerp,
		     doc: "Column for the default `indent-line-function' to indent to.\nLinefeed indents to this column in Fundamental mode.");
  DEFVAR_PER_BUFFER! (c"tab-width", tab_width_, Qintegerp,
		     doc: "Distance between tab stops (for display of tab characters), in columns.\nThis controls the width of a TAB character on display.\nThe value should be a positive integer.\nNote that this variable doesn't necessarily affect the size of the\nindentation step.  However, if the major mode's indentation facility\ninserts one or more TAB characters, this variable will affect the\nindentation step as well, even if `indent-tabs-mode' is non-nil.");
  DEFVAR_PER_BUFFER! (c"ctl-arrow", ctl_arrow_, Qnil,
		     doc: "Non-nil means display control chars with uparrow `^'.\nA value of nil means use backslash `\\' and octal digits.\nThis variable does not apply to characters whose display is specified in\nthe current display table (if there is one; see `standard-display-table').");
  DEFVAR_PER_BUFFER! (c"enable-multibyte-characters", enable_multibyte_characters_, Qnil,
		     doc: "Non-nil means the buffer contents are regarded as multi-byte characters.\nOtherwise they are regarded as unibyte.  This affects the display,\nfile I/O and the behavior of various editing commands.\n\nThis variable is buffer-local but you cannot set it directly;\nuse the function `set-buffer-multibyte' to change a buffer's representation.\nTo prevent any attempts to set it or make it buffer-local, Emacs will\nsignal an error in those cases.\nSee also Info node `(elisp)Text Representations'.");
  make_symbol_constant (intern_c_string (c"enable-multibyte-characters".as_ptr ()));
  DEFVAR_PER_BUFFER! (c"buffer-file-coding-system", buffer_file_coding_system_, Qnil,
		     doc: "Coding system to be used for encoding the buffer contents on saving.\nThis variable applies to saving the buffer, and also to `write-region'\nand other functions that use `write-region'.\nIt does not apply to sending output to subprocesses, however.\n\nIf this is nil, the buffer is saved without any code conversion\nunless some coding system is specified in `file-coding-system-alist'\nfor the buffer file.\n\nIf the text to be saved cannot be encoded as specified by this variable,\nan alternative encoding is selected by `select-safe-coding-system', which see.\n\nThe variable `coding-system-for-write', if non-nil, overrides this variable.\n\nThis variable is never applied to a way of decoding a file while reading it.");
  DEFVAR_PER_BUFFER! (c"bidi-display-reordering", bidi_display_reordering_, Qnil,
		     doc: "Non-nil means reorder bidirectional text for display in the visual order.\nSetting this to nil is intended for use in debugging the display code.\nDon't set to nil in normal sessions, as that is not supported.\nSee also `bidi-paragraph-direction'; setting that non-nil might\nspeed up redisplay.");
  DEFVAR_PER_BUFFER! (c"bidi-paragraph-start-re", bidi_paragraph_start_re_, Qnil,
		     doc: "If non-nil, a regexp matching a line that starts OR separates paragraphs.\n\nThe value of nil means to use empty lines as lines that start and\nseparate paragraphs.\n\nWhen Emacs displays bidirectional text, it by default computes\nthe base paragraph direction separately for each paragraph.\nSetting this variable changes the places where paragraph base\ndirection is recomputed.\n\nThe regexp is always matched after a newline, so it is best to\nanchor it by beginning it with a \"^\".\n\nIf you change the value of this variable, be sure to change\nthe value of `bidi-paragraph-separate-re' accordingly.  For\nexample, to have a single newline behave as a paragraph separator,\nset both these variables to \"^\".\n\nSee also `bidi-paragraph-direction'.");
  DEFVAR_PER_BUFFER! (c"bidi-paragraph-separate-re", bidi_paragraph_separate_re_, Qnil,
		     doc: "If non-nil, a regexp matching a line that separates paragraphs.\n\nThe value of nil means to use empty lines as paragraph separators.\n\nWhen Emacs displays bidirectional text, it by default computes\nthe base paragraph direction separately for each paragraph.\nSetting this variable changes the places where paragraph base\ndirection is recomputed.\n\nThe regexp is always matched after a newline, so it is best to\nanchor it by beginning it with a \"^\".\n\nIf you change the value of this variable, be sure to change\nthe value of `bidi-paragraph-start-re' accordingly.  For\nexample, to have a single newline behave as a paragraph separator,\nset both these variables to \"^\".\n\nSee also `bidi-paragraph-direction'.");
  DEFVAR_PER_BUFFER! (c"bidi-paragraph-direction", bidi_paragraph_direction_, Qnil,
		     doc: "If non-nil, forces directionality of text paragraphs in the buffer.\n\nIf this is nil (the default), the direction of each paragraph is\ndetermined by the first strong directional character of its text.\nThe values of `right-to-left' and `left-to-right' override that.\nAny other value is treated as nil.\n\nThis variable has no effect unless the buffer's value of\n`bidi-display-reordering' is non-nil.");
  DEFVAR_PER_BUFFER! (c"truncate-lines", truncate_lines_, Qnil,
		     doc: "Non-nil means do not display continuation lines.\nInstead, give each line of text just one screen line.\n\nNote that this is overridden by the variable\n`truncate-partial-width-windows' if that variable is non-nil\nand this buffer is not full-frame width.\n\nMinibuffers set this variable to nil.\n\nDon't set this to a non-nil value when `visual-line-mode' is\nturned on, as it could produce confusing results.");
  DEFVAR_PER_BUFFER! (c"word-wrap", word_wrap_, Qnil,
		     doc: "Non-nil means to use word-wrapping for continuation lines.\nWhen word-wrapping is on, continuation lines are wrapped at the space\nor tab character nearest to the right window edge.\nIf nil, continuation lines are wrapped at the right screen edge.\n\nThis variable has no effect if long lines are truncated (see\n`truncate-lines' and `truncate-partial-width-windows').  If you use\nword-wrapping, you might want to reduce the value of\n`truncate-partial-width-windows', since wrapping can make text readable\nin narrower windows.\n\nInstead of setting this variable directly, most users should use\nVisual Line mode.  Visual Line mode, when enabled, sets `word-wrap'\nto t, and additionally redefines simple editing commands to act on\nvisual lines rather than logical lines.  See the documentation of\n`visual-line-mode'.");
  DEFVAR_PER_BUFFER! (c"default-directory", directory_, Qstringp,
		     doc: "Name of default directory of current buffer.\nIt should be an absolute directory name; on GNU and Unix systems,\nthese names start with \"/\" or \"~\" and end with \"/\".\nTo interactively change the default directory, use the command `cd'.");
  DEFVAR_PER_BUFFER! (c"auto-fill-function", auto_fill_function_, Qnil,
		     doc: "Function called (if non-nil) to perform auto-fill.\nIt is called after self-inserting any character specified in\nthe `auto-fill-chars' table.\nNOTE: This variable is not a hook;\nits value may not be a list of functions.");
  DEFVAR_PER_BUFFER! (c"buffer-file-name", filename_, Qstringp,
		     doc: "Name of file visited in current buffer, or nil if not visiting a file.\nThis should be an absolute file name.");
  DEFVAR_PER_BUFFER! (c"buffer-file-truename", file_truename_, Qstringp,
		     doc: "Abbreviated truename of file visited in current buffer, or nil if none.\nThe truename of a file is calculated by `file-truename'\nand then abbreviated with `abbreviate-file-name'.");
  DEFVAR_PER_BUFFER! (c"buffer-auto-save-file-name", auto_save_file_name_, Qstringp,
		     doc: "Name of file for auto-saving current buffer.\nIf it is nil, that means don't auto-save this buffer.");
  DEFVAR_PER_BUFFER! (c"buffer-read-only", read_only_, Qnil,
		     doc: "Non-nil if this buffer is read-only.");
  DEFVAR_PER_BUFFER! (c"buffer-backed-up", backed_up_, Qnil,
		     doc: "Non-nil if this buffer's file has been backed up.\nBacking up is done before the first time the file is saved.");
  DEFVAR_PER_BUFFER! (c"buffer-saved-size", save_length_, Qintegerp,
		     doc: "Length of current buffer when last read in, saved or auto-saved.\n0 initially.\n-1 means auto-saving turned off until next real save.\n\nIf you set this to -2, that means don't turn off auto-saving in this buffer\nif its text size shrinks.   If you use `buffer-swap-text' on a buffer,\nyou probably should set this to -2 in that buffer.");
  DEFVAR_PER_BUFFER! (c"selective-display", selective_display_, Qnil,
		     doc: "Non-nil enables selective display.\n\nAn integer N as value means display only lines\nthat start with less than N columns of space.\n\nA value of t means that the character ^M makes itself and\nall the rest of the line invisible; also, when saving the buffer\nin a file, save the ^M as a newline.  This usage is obsolete; use\noverlays or text properties instead.");
  DEFVAR_PER_BUFFER! (c"selective-display-ellipses", selective_display_ellipses_, Qnil,
		     doc: "Non-nil means display ... on previous line when a line is invisible.");
  DEFVAR_PER_BUFFER! (c"overwrite-mode", overwrite_mode_, Qoverwrite_mode,
		     doc: "Non-nil if self-insertion should replace existing text.\nThe value should be one of `overwrite-mode-textual',\n`overwrite-mode-binary', or nil.\nIf it is `overwrite-mode-textual', self-insertion still\ninserts at the end of a line, and inserts when point is before a tab,\nuntil the tab is filled in.\nIf `overwrite-mode-binary', self-insertion replaces newlines and tabs too.");
  DEFVAR_PER_BUFFER! (c"buffer-display-table", display_table_, Qnil,
		     doc: "Display table that controls display of the contents of current buffer.\n\nIf this variable is nil, the value of `standard-display-table' is used.\nEach window can have its own, overriding display table, see\n`set-window-display-table' and `window-display-table'.\n\nThe display table is a char-table created with `make-display-table'.\nA char-table is an array indexed by character codes.  Normal array\nprimitives `aref' and `aset' can be used to access elements of a char-table.\n\nEach of the char-table elements control how to display the corresponding\ntext character: the element at index C in the table says how to display\nthe character whose code is C.  Each element should be a vector of\ncharacters or nil.  The value nil means display the character in the\ndefault fashion; otherwise, the characters from the vector are delivered\nto the screen instead of the original character.\n\nFor example, (aset buffer-display-table ?X [?Y]) tells Emacs\nto display a capital Y instead of each X character.\n\nIn addition, a char-table has six extra slots to control the display of:\n\n  the end of a truncated screen line (extra-slot 0, a single character);\n  the end of a continued line (extra-slot 1, a single character);\n  the escape character used to display character codes in octal\n    (extra-slot 2, a single character);\n  the character used as an arrow for control characters (extra-slot 3,\n    a single character);\n  the decoration indicating the presence of invisible lines (extra-slot 4,\n    a vector of characters);\n  the character used to draw the border between side-by-side windows\n    (extra-slot 5, a single character).\n\nSee also the functions `display-table-slot' and `set-display-table-slot'.");
  DEFVAR_PER_BUFFER! (c"left-margin-width", left_margin_cols_, Qintegerp,
		     doc: "Width in columns of left marginal area for display of a buffer.\nA value of nil means no marginal area.\n\nSetting this variable does not take effect until a new buffer is displayed\nin a window.  To make the change take effect, call `set-window-buffer'.");
  DEFVAR_PER_BUFFER! (c"right-margin-width", right_margin_cols_, Qintegerp,
		     doc: "Width in columns of right marginal area for display of a buffer.\nA value of nil means no marginal area.\n\nSetting this variable does not take effect until a new buffer is displayed\nin a window.  To make the change take effect, call `set-window-buffer'.");
  DEFVAR_PER_BUFFER! (c"left-fringe-width", left_fringe_width_, Qintegerp,
		     doc: "Width of this buffer's left fringe (in pixels).\nA value of 0 means no left fringe is shown in this buffer's window.\nA value of nil means to use the left fringe width from the window's frame.\n\nSetting this variable does not take effect until a new buffer is displayed\nin a window.  To make the change take effect, call `set-window-buffer'.");
  DEFVAR_PER_BUFFER! (c"right-fringe-width", right_fringe_width_, Qintegerp,
		     doc: "Width of this buffer's right fringe (in pixels).\nA value of 0 means no right fringe is shown in this buffer's window.\nA value of nil means to use the right fringe width from the window's frame.\n\nSetting this variable does not take effect until a new buffer is displayed\nin a window.  To make the change take effect, call `set-window-buffer'.");
  DEFVAR_PER_BUFFER! (c"fringes-outside-margins", fringes_outside_margins_, Qnil,
		     doc: "Non-nil means to display fringes outside display margins.\nA value of nil means to display fringes between margins and buffer text.\n\nSetting this variable does not take effect until a new buffer is displayed\nin a window.  To make the change take effect, call `set-window-buffer'.");
  DEFVAR_PER_BUFFER! (c"scroll-bar-width", scroll_bar_width_, Qintegerp,
		     doc: "Width of this buffer's vertical scroll bars in pixels.\nA value of nil means to use the scroll bar width from the window's frame.");
  DEFVAR_PER_BUFFER! (c"scroll-bar-height", scroll_bar_height_, Qintegerp,
		     doc: "Height of this buffer's horizontal scroll bars in pixels.\nA value of nil means to use the scroll bar height from the window's frame.");
  DEFVAR_PER_BUFFER! (c"vertical-scroll-bar", vertical_scroll_bar_type_, Qvertical_scroll_bar,
		     doc: "Position of this buffer's vertical scroll bar.\nThe value takes effect whenever you tell a window to display this buffer;\nfor instance, with `set-window-buffer' or when `display-buffer' displays it.\n\nA value of `left' or `right' means put the vertical scroll bar at that side\nof the window; a value of nil means don't show any vertical scroll bars.\nA value of t (the default) means do whatever the window's frame specifies.");
  DEFVAR_PER_BUFFER! (c"horizontal-scroll-bar", horizontal_scroll_bar_type_, Qnil,
		     doc: "Position of this buffer's horizontal scroll bar.\nThe value takes effect whenever you tell a window to display this buffer;\nfor instance, with `set-window-buffer' or when `display-buffer' displays it.\n\nA value of `bottom' means put the horizontal scroll bar at the bottom of\nthe window; a value of nil means don't show any horizontal scroll bars.\nA value of t (the default) means do whatever the window's frame\nspecifies.");
  DEFVAR_PER_BUFFER! (c"indicate-empty-lines", indicate_empty_lines_, Qnil,
		     doc: "Visually indicate unused (\"empty\") screen lines after the buffer end.\nIf non-nil, a bitmap is displayed in the left fringe of a window\non graphical displays for each screen line that doesn't correspond\nto any buffer text.");
  DEFVAR_PER_BUFFER! (c"indicate-buffer-boundaries", indicate_buffer_boundaries_, Qnil,
		     doc: "Visually indicate buffer boundaries and scrolling.\nIf non-nil, the first and last line of the buffer are marked in the fringe\nof a window on graphical displays with angle bitmaps, or if the window can be\nscrolled, the top and bottom line of the window are marked with up and down\narrow bitmaps.\n\nIf value is a symbol `left' or `right', both angle and arrow bitmaps\nare displayed in the left or right fringe, resp.  Any other value\nthat doesn't look like an alist means display the angle bitmaps in\nthe left fringe but no arrows.\n\nYou can exercise more precise control by using an alist as the\nvalue.  Each alist element (INDICATOR . POSITION) specifies\nwhere to show one of the indicators.  INDICATOR is one of `top',\n`bottom', `up', `down', or t, which specifies the default position,\nand POSITION is one of `left', `right', or nil, meaning do not show\nthis indicator.\n\nFor example, ((top . left) (t . right)) places the top angle bitmap in\nleft fringe, the bottom angle bitmap in right fringe, and both arrow\nbitmaps in right fringe.  To show just the angle bitmaps in the left\nfringe, but no arrow bitmaps, use ((top .  left) (bottom . left)).");
  DEFVAR_PER_BUFFER! (c"fringe-indicator-alist", fringe_indicator_alist_, Qnil,
		     doc: "Mapping from logical to physical fringe indicator bitmaps.\nThe value is an alist where each element (INDICATOR . BITMAPS)\nspecifies the fringe bitmaps used to display a specific logical\nfringe indicator.\n\nINDICATOR specifies the logical indicator type which is one of the\nfollowing symbols: `truncation' , `continuation', `overlay-arrow',\n`top', `bottom', `top-bottom', `up', `down', empty-line', or `unknown'.\n\nBITMAPS is a list of symbols (LEFT RIGHT [LEFT1 RIGHT1]) which specifies\nthe actual bitmap shown in the left or right fringe for the logical\nindicator.  LEFT and RIGHT are the bitmaps shown in the left and/or\nright fringe for the specific indicator.  The LEFT1 or RIGHT1 bitmaps\nare used only for the `bottom' and `top-bottom' indicators when the\nlast (only) line has no final newline.  BITMAPS may also be a single\nsymbol which is used in both left and right fringes.");
  DEFVAR_PER_BUFFER! (c"fringe-cursor-alist", fringe_cursor_alist_, Qnil,
		     doc: "Mapping from logical to physical fringe cursor bitmaps.\nThe value is an alist where each element (CURSOR . BITMAP)\nspecifies the fringe bitmaps used to display a specific logical\ncursor type in the fringe.\n\nCURSOR specifies the logical cursor type which is one of the following\nsymbols: `box' , `hollow', `bar', `hbar', or `hollow-small'.  The last\none is used to show a hollow cursor on narrow lines display lines\nwhere the normal hollow cursor will not fit.\n\nBITMAP is the corresponding fringe bitmap shown for the logical\ncursor type.");
  DEFVAR_PER_BUFFER! (c"scroll-up-aggressively", scroll_up_aggressively_, Qfraction,
		     doc: "How far to scroll windows upward.\nIf you move point off the bottom, the window scrolls automatically.\nThis variable controls how far it scrolls.  The value nil, the default,\nmeans scroll to center point.  A fraction means scroll to put point\nthat fraction of the window's height from the bottom of the window.\nWhen the value is 0.0, point goes at the bottom line, which in the\nsimple case that you moved off with C-f means scrolling just one line.\n1.0 means point goes at the top, so that in that simple case, the\nwindow scrolls by a full window height.  Meaningful values are\nbetween 0.0 and 1.0, inclusive.");
  DEFVAR_PER_BUFFER! (c"scroll-down-aggressively", scroll_down_aggressively_, Qfraction,
		     doc: "How far to scroll windows downward.\nIf you move point off the top, the window scrolls automatically.\nThis variable controls how far it scrolls.  The value nil, the default,\nmeans scroll to center point.  A fraction means scroll to put point\nthat fraction of the window's height from the top of the window.\nWhen the value is 0.0, point goes at the top line, which in the\nsimple case that you moved off with C-b means scrolling just one line.\n1.0 means point goes at the bottom, so that in that simple case, the\nwindow scrolls by a full window height.  Meaningful values are\nbetween 0.0 and 1.0, inclusive.");
  DEFVAR_LISP! (c"before-change-functions", Vbefore_change_functions,
	      "List of functions to call before each text change.\nTwo arguments are passed to each function: the positions of\nthe beginning and end of the range of old text to be changed.\n(For an insertion, the beginning and end are at the same place.)\nNo information is given about the length of the text after the change.\n\nBuffer changes made while executing the `before-change-functions'\ndon't call any before-change or after-change functions.\nThat's because `inhibit-modification-hooks' is temporarily set non-nil.\n\nIf an unhandled error happens in running these functions,\nthe variable's value remains nil.  That prevents the error\nfrom happening repeatedly and making Emacs nonfunctional.");
  Vbefore_change_functions = Qnil ();
  DEFVAR_LISP! (c"after-change-functions", Vafter_change_functions,
	      "List of functions to call after each text change.\nThree arguments are passed to each function: the positions of\nthe beginning and end of the range of changed text,\nand the length in chars of the pre-change text replaced by that range.\n(For an insertion, the pre-change length is zero;\nfor a deletion, that length is the number of chars deleted,\nand the post-change beginning and end are at the same place.)\n\nBuffer changes made while executing the `after-change-functions'\ndon't call any before-change or after-change functions.\nThat's because `inhibit-modification-hooks' is temporarily set non-nil.\n\nIf an unhandled error happens in running these functions,\nthe variable's value remains nil.  That prevents the error\nfrom happening repeatedly and making Emacs nonfunctional.");
  Vafter_change_functions = Qnil ();
  DEFVAR_LISP! (c"first-change-hook", Vfirst_change_hook,
	      "A list of functions to call before changing a buffer which is unmodified.\nThe functions are run using the `run-hooks' function.");
  Vfirst_change_hook = Qnil ();
  DEFVAR_PER_BUFFER! (c"buffer-undo-list", undo_list_, Qnil,
		     doc: "List of undo entries in current buffer.\nRecent changes come first; older changes follow newer.\n\nAn entry (BEG . END) represents an insertion which begins at\nposition BEG and ends at position END.\n\nAn entry (TEXT . POSITION) represents the deletion of the string TEXT\nfrom (abs POSITION).  If POSITION is positive, point was at the front\nof the text being deleted; if negative, point was at the end.\n\nAn entry (t . TIMESTAMP), where TIMESTAMP is in the style of\n`current-time', indicates that the buffer was previously unmodified;\nTIMESTAMP is the visited file's modification time, as of that time.\nIf the modification time of the most recent save is different, this\nentry is obsolete.\n\nAn entry (t . 0) means the buffer was previously unmodified but\nits time stamp was unknown because it was not associated with a file.\nAn entry (t . -1) is similar, except that it means the buffer's visited\nfile did not exist.\n\nAn entry (nil PROPERTY VALUE BEG . END) indicates that a text property\nwas modified between BEG and END.  PROPERTY is the property name,\nand VALUE is the old value.\n\nAn entry (apply FUN-NAME . ARGS) means undo the change with\n(apply FUN-NAME ARGS).\n\nAn entry (apply DELTA BEG END FUN-NAME . ARGS) supports selective undo\nin the active region.  BEG and END is the range affected by this entry\nand DELTA is the number of characters added or deleted in that range by\nthis change.\n\nAn entry (apply DELTA (BEG . END) FUN-NAME . ARGS) is similar to the\nprevious kind.  The difference is that Emacs invokes FUN-NAME with\n(apply FUN-NAME BEG2 END2 ARGS), where BEG2 and END2 are the adjusted\nvalues of BEG and END.  The function can use those arguments to compute\npositions relative to the adjusted region.\n\nAn entry (MARKER . DISTANCE) indicates that the marker MARKER\nwas adjusted in position by the offset DISTANCE (an integer).\n\nAn entry of the form POSITION indicates that point was at the buffer\nlocation given by the integer.  Undoing an entry of this form places\npoint at POSITION.\n\nEntries with value nil mark undo boundaries.  The undo command treats\nthe changes between two undo boundaries as a single step to be undone.\n\nIf the value of the variable is t, undo information is not recorded.");
  DEFVAR_PER_BUFFER! (c"mark-active", mark_active_, Qnil,
		     doc: "Non-nil means the mark and region are currently active in this buffer.");
  DEFVAR_PER_BUFFER! (c"cache-long-scans", cache_long_scans_, Qnil,
		     doc: "Non-nil means that Emacs should use caches in attempt to speedup buffer scans.\n\nThere is no reason to set this to nil except for debugging purposes.\n\nNormally, the line-motion functions work by scanning the buffer for\nnewlines.  Columnar operations (like `move-to-column' and\n`compute-motion') also work by scanning the buffer, summing character\nwidths as they go.  This works well for ordinary text, but if the\nbuffer's lines are very long (say, more than 500 characters), these\nmotion functions will take longer to execute.  Emacs may also take\nlonger to update the display.\n\nIf `cache-long-scans' is non-nil, these motion functions cache the\nresults of their scans, and consult the cache to avoid rescanning\nregions of the buffer until the text is modified.  The caches are most\nbeneficial when they prevent the most searching---that is, when the\nbuffer contains long lines and large regions of characters with the\nsame, fixed screen width.\n\nWhen `cache-long-scans' is non-nil, processing short lines will\nbecome slightly slower (because of the overhead of consulting the\ncache), and the caches will use memory roughly proportional to the\nnumber of newlines and characters whose screen width varies.\n\nBidirectional editing also requires buffer scans to find paragraph\nseparators.  If you have large paragraphs or no paragraph separators\nat all, these scans may be slow.  If `cache-long-scans' is non-nil,\nresults of these scans are cached.  This doesn't help too much if\nparagraphs are of the reasonable (few thousands of characters) size.\n\nThe caches require no explicit maintenance; their accuracy is\nmaintained internally by the Emacs primitives.  Enabling or disabling\nthe cache should not affect the behavior of any of the motion\nfunctions; it should only affect their performance.");
  DEFVAR_PER_BUFFER! (c"point-before-scroll", point_before_scroll_, Qnil,
		     doc: "Value of point before the last series of scroll operations, or nil.");
  DEFVAR_PER_BUFFER! (c"buffer-file-format", file_format_, Qnil,
		     doc: "List of formats to use when saving this buffer.\nFormats are defined by `format-alist'.  This variable is\nset when a file is visited.");
  DEFVAR_PER_BUFFER! (c"buffer-auto-save-file-format", auto_save_file_format_, Qnil,
		     doc: "Format in which to write auto-save files.\nShould be a list of symbols naming formats that are defined in `format-alist'.\nIf it is t, which is the default, auto-save files are written in the\nsame format as a regular save would use.");
  DEFVAR_PER_BUFFER! (c"buffer-invisibility-spec", invisibility_spec_, Qnil,
		     doc: "Invisibility spec of this buffer.\nThe default is t, which means that text is invisible if it has a non-nil\n`invisible' property.\nThis variable can also be a list.  The list can have two kinds of elements:\n`ATOM' and `(ATOM . ELLIPSIS)'.  A text character is invisible if its\n`invisible' property is `ATOM', or has an `invisible' property that is a list\nthat contains `ATOM'.\nIf the `(ATOM . ELLIPSIS)' form is used, and `ELLIPSIS' is non-nil, an\nellipsis will be displayed after the invisible characters.\nSetting this variable is very fast, much faster than scanning all the text in\nthe buffer looking for properties to change.");
  DEFVAR_PER_BUFFER! (c"buffer-display-count", display_count_, Qintegerp,
		     doc: "A number incremented each time this buffer is displayed in a window.\nThe function `set-window-buffer' increments it.");
  DEFVAR_PER_BUFFER! (c"buffer-display-time", display_time_, Qnil,
		     doc: "Time stamp updated each time this buffer is displayed in a window.\nThe function `set-window-buffer' updates this variable\nto the value obtained by calling `current-time'.\nIf the buffer has never been shown in a window, the value is nil.");
  DEFVAR_LISP! (c"transient-mark-mode", Vtransient_mark_mode,
	      "Non-nil if Transient Mark mode is enabled.\nSee the command `transient-mark-mode' for a description of this minor mode.\n\nNon-nil also enables highlighting of the region whenever the mark is active.\nThe region is highlighted with the `region' face.\nThe variable `highlight-nonselected-windows' controls whether to highlight\nall windows or just the selected window.\n\nLisp programs may give this variable certain special values:\n\n- The symbol `lambda' enables Transient Mark mode temporarily.\n  The mode is disabled again after any subsequent action that would\n  normally deactivate the mark (e.g. buffer modification).\n\n- The pair (only . OLDVAL) enables Transient Mark mode\n  temporarily.  After any subsequent point motion command that is\n  not shift-translated, or any other action that would normally\n  deactivate the mark (e.g. buffer modification), the value of\n  `transient-mark-mode' is set to OLDVAL.");
  Vtransient_mark_mode = Qnil ();
  DEFVAR_LISP! (c"inhibit-read-only", Vinhibit_read_only,
	      "Non-nil means disregard read-only status of buffers or characters.\nA non-nil value that is a list means disregard `buffer-read-only' status,\nand disregard a `read-only' text property if the property value is a\nmember of the list.  Any other non-nil value means disregard `buffer-read-only'\nand all `read-only' text properties.");
  Vinhibit_read_only = Qnil ();
  DEFVAR_PER_BUFFER! (c"cursor-type", cursor_type_, Qnil,
		     doc: "Cursor to use when this buffer is in the selected window.\nValues are interpreted as follows:\n\n  t               use the cursor specified for the frame\n  nil             don't display a cursor\n  box             display a filled box cursor\n  (box . SIZE)    display a filled box cursor, but make it\n                  hollow if cursor is under masked image larger than\n                  SIZE pixels in either dimension.\n  hollow          display a hollow box cursor\n  bar             display a vertical bar cursor with default width\n  (bar . WIDTH)   display a vertical bar cursor with width WIDTH\n  hbar            display a horizontal bar cursor with default height\n  (hbar . HEIGHT) display a horizontal bar cursor with height HEIGHT\n  ANYTHING ELSE   display a hollow box cursor\n\nWIDTH and HEIGHT can't exceed the frame's canonical character size.\n\nWhen the buffer is displayed in a non-selected window, the\ncursor's appearance is instead controlled by the variable\n`cursor-in-non-selected-windows'.");
  DEFVAR_PER_BUFFER! (c"line-spacing", extra_line_spacing_, Qnil,
		     doc: "Additional space to put between lines when displaying a buffer.\nThe space is measured in pixels, and put below lines on graphic displays,\nsee `display-graphic-p'.\nIf value is a floating point number, it specifies the spacing relative\nto the default frame line height.\nIf value is a cons cell containing a pair of floats or integers,\nit is interpreted as space above and below the line, respectively.\nA value of nil means add no extra space.");
  DEFVAR_PER_BUFFER! (c"cursor-in-non-selected-windows", cursor_in_non_selected_windows_, Qnil,
		     doc: "Non-nil means show a cursor in non-selected windows.\nIf nil, only shows a cursor in the selected window.\nIf t, displays a cursor related to the usual cursor type\n(a solid box becomes hollow, a bar becomes a narrower bar).\nYou can also specify the cursor type as in the `cursor-type' variable.\nUse Custom to set this variable and update the display.");
  DEFVAR_PER_BUFFER! (c"text-conversion-style", text_conversion_style_, Qnil,
		     doc: "How the on screen keyboard's input method should insert in this buffer.\n\nIf nil, the input method will be disabled and an ordinary keyboard\nwill be displayed in its place.\n\nIf the value is the symbol `action', the input method will insert text\ndirectly, but will send `return' key events instead of inserting new\nline characters.\n\nIf the value is the symbol `password', an input method capable of ASCII\ninput will be enabled, and will not save the entered text where it will\nbe retrieved for text suggestions or other features not suitable for\nhandling sensitive information, in addition to reporting `return' as\nwhen `action'.\n\nAny other value means that the input method will insert text directly.\n\nIf you need to make non-buffer local changes to this variable, use\n`overriding-text-conversion-style', which see.\n\nThis variable does not take immediate effect when set; rather, it\ntakes effect upon the next redisplay after the selected window or\nits buffer changes.");
  DEFVAR_LISP! (c"kill-buffer-query-functions", Vkill_buffer_query_functions,
	      "List of functions called with no args to query before killing a buffer.\nThe buffer being killed will be current while the functions are running.\nSee `kill-buffer'.\n\nIf any of them returns nil, the buffer is not killed.  Functions run by\nthis hook are supposed to not change the current buffer.\n\nThis hook is not run for internal or temporary buffers created by\n`get-buffer-create' or `generate-new-buffer' with argument\nINHIBIT-BUFFER-HOOKS non-nil.");
  Vkill_buffer_query_functions = Qnil ();
  DEFVAR_LISP! (c"change-major-mode-hook", Vchange_major_mode_hook,
	      "Normal hook run before changing the major mode of a buffer.\nThe function `kill-all-local-variables' runs this before doing anything else.");
  Vchange_major_mode_hook = Qnil ();
  DEFSYM! (Qchange_major_mode_hook, c"change-major-mode-hook");
  DEFVAR_LISP! (c"buffer-list-update-hook", Vbuffer_list_update_hook,
	      "Hook run when the buffer list changes.\nFunctions (implicitly) running this hook are `get-buffer-create',\n`make-indirect-buffer', `rename-buffer', `kill-buffer', `bury-buffer'\nand `select-window'.  This hook is not run for internal or temporary\nbuffers created by `get-buffer-create' or `generate-new-buffer' with\nargument INHIBIT-BUFFER-HOOKS non-nil.\n\nFunctions run by this hook should avoid calling `select-window' with a\nnil NORECORD argument since it may lead to infinite recursion.");
  Vbuffer_list_update_hook = Qnil ();
  DEFSYM! (Qbuffer_list_update_hook, c"buffer-list-update-hook");
  DEFVAR_BOOL! (c"kill-buffer-delete-auto-save-files", kill_buffer_delete_auto_save_files,
	      "If non-nil, offer to delete any autosave file when killing a buffer.\n\nIf `delete-auto-save-files' is nil, any autosave deletion is inhibited.");
  kill_buffer_delete_auto_save_files = false;
  DEFVAR_BOOL! (c"delete-auto-save-files", delete_auto_save_files,
	      "Non-nil means delete auto-save file when a buffer is saved.\nThis is the default.  If nil, auto-save file deletion is inhibited.");
  delete_auto_save_files = true;
  DEFVAR_LISP! (c"case-fold-search", Vcase_fold_search,
	      "Non-nil if searches and matches should ignore case.");
  Vcase_fold_search = Qt ();
  DEFSYM! (Qcase_fold_search, c"case-fold-search");
  Fmake_variable_buffer_local (Qcase_fold_search ());
  DEFVAR_LISP! (c"clone-indirect-buffer-hook", Vclone_indirect_buffer_hook,
	      "Normal hook to run in the new buffer at the end of `make-indirect-buffer'.\n\nSince `clone-indirect-buffer' calls `make-indirect-buffer', this hook\nwill run for `clone-indirect-buffer' calls as well.");
  Vclone_indirect_buffer_hook = Qnil ();
  DEFVAR_LISP! (c"long-line-threshold", Vlong_line_threshold,
	      "Line length above which to use redisplay shortcuts.\n\nThe value should be a positive integer or nil.\nIf the value is an integer, shortcuts in the display code intended\nto speed up redisplay for long lines will automatically be enabled\nin buffers which contain one or more lines whose length is above\nthis threshold.\nIf nil, these display shortcuts will always remain disabled.\n\nThere is no reason to change that value except for debugging purposes.");
  XSETFASTINT! (Vlong_line_threshold, 50000);
  DEFVAR_INT! (c"long-line-optimizations-region-size", long_line_optimizations_region_size,
	      "Region size for narrowing in buffers with long lines.\n\nThis variable has effect only in buffers in which\n`long-line-optimizations-p' is non-nil.  For performance reasons, in\nsuch buffers, the `fontification-functions', `pre-command-hook' and\n`post-command-hook' hooks are executed on a narrowed buffer around\npoint, as if they were called in a `with-restriction' form with a label.\nThis variable specifies the size of the narrowed region around point.\n\nTo disable that narrowing, set this variable to 0.\n\nSee also `long-line-optimizations-bol-search-limit'.\n\nThere is no reason to change that value except for debugging purposes.");
  long_line_optimizations_region_size = 500000;
  DEFVAR_INT! (c"long-line-optimizations-bol-search-limit", long_line_optimizations_bol_search_limit,
	      "Limit for beginning of line search in buffers with long lines.\n\nThis variable has effect only in buffers in which\n`long-line-optimizations-p' is non-nil.  For performance reasons, in\nsuch buffers, the `fontification-functions', `pre-command-hook' and\n`post-command-hook' hooks are executed on a narrowed buffer around\npoint, as if they were called in a `with-restriction' form with a label.\nThe variable `long-line-optimizations-region-size' specifies the\nsize of the narrowed region around point.  This variable, which should\nbe a small integer, specifies the number of characters by which that\nregion can be extended backwards to make it start at the beginning of\na line.\n\nThere is no reason to change that value except for debugging purposes.");
  long_line_optimizations_bol_search_limit = 128;
  DEFVAR_INT! (c"large-hscroll-threshold", large_hscroll_threshold,
	      "Horizontal scroll of truncated lines above which to use redisplay shortcuts.\n\nThe value should be a positive integer.\n\nShortcuts in the display code intended to speed up redisplay for long\nand truncated lines will automatically be enabled when a line's\nhorizontal scroll amount is or about to become larger than the value\nof this variable.\n\nThis variable has effect only in buffers which contain one or more\nlines whose length is above `long-line-threshold', which see.\nTo disable redisplay shortcuts for long truncated line, set this\nvariable to `most-positive-fixnum'.\n\nThere is no reason to change that value except for debugging purposes.");
  large_hscroll_threshold = 10000;
  defsubr (&raw mut Sbuffer_live_p);
  defsubr (&raw mut Sbuffer_list);
  defsubr (&raw mut Sget_buffer);
  defsubr (&raw mut Sget_file_buffer);
  defsubr (&raw mut Sget_truename_buffer);
  defsubr (&raw mut Sfind_buffer);
  defsubr (&raw mut Sget_buffer_create);
  defsubr (&raw mut Smake_indirect_buffer);
  defsubr (&raw mut Sgenerate_new_buffer_name);
  defsubr (&raw mut Sbuffer_name);
  defsubr (&raw mut Sbuffer_last_name);
  defsubr (&raw mut Sbuffer_file_name);
  defsubr (&raw mut Sbuffer_base_buffer);
  defsubr (&raw mut Sbuffer_local_value);
  defsubr (&raw mut Sbuffer_local_variables);
  defsubr (&raw mut Sbuffer_modified_p);
  defsubr (&raw mut Sforce_mode_line_update);
  defsubr (&raw mut Sset_buffer_modified_p);
  defsubr (&raw mut Sbuffer_modified_tick);
  defsubr (&raw mut Sinternal__set_buffer_modified_tick);
  defsubr (&raw mut Sbuffer_chars_modified_tick);
  defsubr (&raw mut Srename_buffer);
  defsubr (&raw mut Sother_buffer);
  defsubr (&raw mut Sbuffer_enable_undo);
  defsubr (&raw mut Skill_buffer);
  defsubr (&raw mut Sbury_buffer_internal);
  defsubr (&raw mut Sset_buffer_major_mode);
  defsubr (&raw mut Scurrent_buffer);
  defsubr (&raw mut Sset_buffer);
  defsubr (&raw mut Sbarf_if_buffer_read_only);
  defsubr (&raw mut Serase_buffer);
  defsubr (&raw mut Sbuffer_swap_text);
  defsubr (&raw mut Sset_buffer_multibyte);
  defsubr (&raw mut Skill_all_local_variables);
  defsubr (&raw mut Soverlayp);
  defsubr (&raw mut Smake_overlay);
  defsubr (&raw mut Sdelete_overlay);
  defsubr (&raw mut Sdelete_all_overlays);
  defsubr (&raw mut Smove_overlay);
  defsubr (&raw mut Soverlay_start);
  defsubr (&raw mut Soverlay_end);
  defsubr (&raw mut Soverlay_buffer);
  defsubr (&raw mut Soverlay_properties);
  defsubr (&raw mut Soverlays_at);
  defsubr (&raw mut Soverlays_in);
  defsubr (&raw mut Snext_overlay_change);
  defsubr (&raw mut Sprevious_overlay_change);
  defsubr (&raw mut Soverlay_recenter);
  defsubr (&raw mut Soverlay_lists);
  defsubr (&raw mut Soverlay_get);
  defsubr (&raw mut Soverlay_put);
  defsubr (&raw mut Srestore_buffer_modified_p);
  DEFSYM! (Qautosaved, c"autosaved");
  // C 6145-6147 is `#ifdef ITREE_DEBUG` (undefined in the frozen
  // configuration): `defsubr (&Soverlay_tree);`.  Recorded in
  // docs/ports/buffer.md.
  DEFSYM! (Qkill_buffer__possibly_save, c"kill-buffer--possibly-save");
  DEFSYM! (Qbuffer_stale_function, c"buffer-stale-function");
  Fput (intern_c_string (c"erase-buffer".as_ptr ()), Qdisabled (), Qt ());
  DEFSYM! (Qbuffer_save_without_query, c"buffer-save-without-query");
  DEFSYM! (Qbuffer_file_number, c"buffer-file-number");
  DEFSYM! (Qbuffer_undo_list, c"buffer-undo-list");
  DEFSYM! (Qrename_auto_save_file, c"rename-auto-save-file");
  DEFSYM! (Quniquify__rename_buffer_advice, c"uniquify--rename-buffer-advice");
  DEFSYM! (Qdelete_auto_save_file_if_necessary, c"delete-auto-save-file-if-necessary");
  DEFSYM! (Qinitial_major_mode, c"initial-major-mode");
  DEFSYM! (Qset_buffer_multibyte, c"set-buffer-multibyte");
}
