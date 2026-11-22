// femacs: generated mirror of the frozen build's C-ABI exports - the C-ABI entry points the
// macOS Swift units import by name (`@_silgen_name`, tools/check-swift-imports.py).
//
// This is **not** a port of a reference file: C has no functions with these names.  Each one is
// the C-ABI spelling of a C construct (a `struct frame` member, a `FRAME_*` accessor macro or a
// bitfield of `bits1`) that the Swift lanes import by name, so the core has to provide it as a
// real symbol.  The per-name table - the Swift import, the Rust item that provides it, and the C
// line it stands for - is `docs/ports/abi_shim.md`; the rulings behind it are
// `docs/reviews/2026-10-04-swift-import-rulings.md`.
//
// Why a unit of its own rather than the unit that owns the record: a mirror is `include!`d into
// every consumer, so a definition there is one definition per unit (`E0428`), and a name that C
// never defines as a function is rejected by `extract-abi.py`'s "invented exported symbol" rule
// when it stands beside a C file (measured).  A unit with no C counterpart is outside every
// `--check` pairing, and `xtask` is the precedent for a product with no C source (status 3.241).
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::common::*;

// [U1] config.rs provided by common
// [U1] ../lib/include/stdio.rs imported via common
// [U1] ../lib/include/stdlib.rs imported via common
// [U1] ../lib/include/errno.rs imported via common
// [U1] ../lib/include/limits.in.rs imported via common
// [U1] ../lib/include/stdint.in.rs imported via common
// [U1] ../lib/include/string.rs imported via common
// [U1] ../lib/include/c-ctype.rs imported via common
// [U1] ../lib/include/stdcountof.rs imported via common
// [U1] include/lisp.rs imported via common
// [U1] include/character.rs imported via common
// [U1] include/intervals.rs imported via common
// [U1] include/systime.rs imported via common
// [U1] include/itree.rs imported via common
// [U1] include/composite.rs imported via common
// [U1] include/coding.rs imported via common
// [U1] include/font.rs imported via common
// [U1] include/termhooks.rs imported via common
// [U1] include/frame.rs imported via common
// [U1] include/keyboard.rs imported via common
// [U1] include/buffer.rs imported via common

/// Snapshot the selected buffer for Cocoa accessibility without reading its gap
/// or depending on the interpreter's temporarily selected current_buffer.
#[cfg(feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_accessibility_buffer_string(buf: *mut buffer) -> Lisp_Object {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  let begin = BUF_BEGV_BYTE(buf);
  let end = BUF_ZV_BYTE(buf);
  let bytes = end - begin;
  let text = if !NILP((*buf).enable_multibyte_characters_) {
    make_uninit_multibyte_string(BUF_ZV(buf) - BUF_BEGV(buf), bytes)
  } else { make_uninit_string(bytes) };
  let gap = BUF_GPT_BYTE!(buf);
  let first = if begin < gap && gap < end { gap - begin } else { bytes };
  core::ptr::copy_nonoverlapping(BUF_BYTE_ADDRESS(buf, begin), SDATA(text), first as usize);
  if first < bytes {
    core::ptr::copy_nonoverlapping(BUF_BYTE_ADDRESS(buf, begin + first),
                                 SDATA(text).offset(first), (bytes - first) as usize);
  }
  text

  }
}
// [U1] include/blockinput.rs imported via common
// [U1] include/termchar.rs imported via common
// [U1] include/dispextern.rs imported via common
// [U1] include/window.rs imported via common
// [U1] include/cm.rs imported via common
// [U1] include/pdumper.rs imported via common
// `struct ns_output` / `struct ns_display_info` - the two records the batch-6 entries below read
// and write - are `nsterm.h`'s, and their mirror sits behind the same `HAVE_NS` C gates the
// whole file does (`include/nsterm.rs` 29, which is why every entry that names them carries
// `have_ns` too).  A unit that names a mirrored type has to reach the mirror (P-72), so the
// include carries the gate of the entries that need it: in the frozen configuration the line is
// stripped before expansion and the frozen object is unchanged (measured, `## Evidence`).
#[cfg (feature = "have_ns")]
const _: () = ();

/* ---------------------------------------------------------------------------------------------
   `struct frame` readers.  The record itself is `include/frame.rs`'s (its members were measured
   there); every function below reads the member C's macro or member access reads, and the
   `/// C:` line carries C's own text with the reference's line number.
   --------------------------------------------------------------------------------------------- */

/// `frame_left_pos` is the ABI of C's `f->left_pos` (`struct frame` member, frame.h 604); the
/// Swift unit `nsmenu.swift` imports it.
/// C: int left_pos, top_pos;   (frame.h 604)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_left_pos (f: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).left_pos }
}

/// C: int left_pos, top_pos;   (frame.h 604)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_top_pos (f: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).top_pos }
}

/// C: Lisp_Object title;   (frame.h 162)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_title (f: *mut c_void) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).title }
}

/// C: Lisp_Object icon_name;   (frame.h 158)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_icon_name (f: *mut c_void) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).icon_name }
}

/// C: Lisp_Object param_alist;   (frame.h 209)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_param_alist (f: *mut c_void) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).param_alist }
}

/// C: #define FRAME_SELECTED_WINDOW(f) f->selected_window   (frame.h 1204)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_selected_window (f: *mut c_void) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).selected_window }
}

/// C: Lisp_Object menu_bar_vector;   (frame.h 234)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_menu_bar_vector (f: *mut c_void) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).menu_bar_vector }
}

/// C: Lisp_Object tool_bar_items;   (frame.h 283)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_tool_bar_items (f: *mut c_void) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).tool_bar_items }
}

/// C: int menu_bar_items_used;   (frame.h 307)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_menu_bar_items_used (f: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).menu_bar_items_used }
}

/// C: int n_tool_bar_items;   (frame.h 573)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_n_tool_bar_items (f: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).n_tool_bar_items }
}

/* ---------------------------------------------------------------------------------------------
   `struct frame` writers.  Same record, same table (`docs/ports/abi_shim.md`); C writes these
   through plain member assignment (`f->border_width = v`), which is what each body does.
   --------------------------------------------------------------------------------------------- */

/// C: int border_width;   (frame.h 613)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_border_width (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).border_width = v; }
}

/// C: int child_frame_border_width;   (frame.h 617)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_child_frame_border_width (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).child_frame_border_width = v; }
}

/// C: int internal_border_width;   (frame.h 622)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_internal_border_width (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).internal_border_width = v; }
}

/// C: `int menu_bar_lines;` (frame.h 635, `menu_bar_lines`); the reader is
/// `FRAME_MENU_BAR_LINES` (frame.h 1086)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_menu_bar_lines (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).menu_bar_lines = v; }
}

/// C: `int tab_bar_lines;` (frame.h 558, `tab_bar_lines`); the reader is
/// `FRAME_TAB_BAR_LINES` (frame.h 1090)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_tab_bar_lines (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).tab_bar_lines = v; }
}

/// C: `int tab_bar_height;` (frame.h 561, `tab_bar_height`); the reader is
/// `FRAME_TAB_BAR_HEIGHT` (frame.h 1091)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_tab_bar_height (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).tab_bar_height = v; }
}

/// C: `unsigned long background_pixel;` (frame.h 732, `background_pixel`); the reader is
/// `FRAME_BACKGROUND_PIXEL` (frame.h 1376)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_background_pixel (f: *mut c_void, v: c_ulong)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).background_pixel = v; }
}

/// C: `unsigned long foreground_pixel;` (frame.h 733, `foreground_pixel`); the reader is
/// `FRAME_FOREGROUND_PIXEL` (frame.h 1375)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_foreground_pixel (f: *mut c_void, v: c_ulong)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).foreground_pixel = v; }
}

/* ---------------------------------------------------------------------------------------------
   `frames.bits1` accessors, and the three names the rulings of status 3.242 fixed.  The bit
   shifts are `include/frame.rs`'s (`FRAME_WANTS_MODELINE_SHIFT` 11, `FRAME_EXPLICIT_NAME_SHIFT`
   15, `FRAME_OUTPUT_METHOD_SHIFT` 21 with `FRAME_OUTPUT_METHOD_MASK` 0x0F, the two
   `FRAME_TAB_BAR_*_SHIFT` 30/31); C's own text is on each `/// C:` line.
   --------------------------------------------------------------------------------------------- */

/// C: bool_bf explicit_name : 1;   (frame.h 417)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_explicit_name (f: *mut c_void) -> bool
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { ((*f.cast::<frame> ()).bits1 >> FRAME_EXPLICIT_NAME_SHIFT) & 1 != 0 }
}

/// C: bool_bf explicit_name : 1;   (frame.h 417)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_explicit_name (f: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_EXPLICIT_NAME_SHIFT)) | ((v as u32) << FRAME_EXPLICIT_NAME_SHIFT);
  }
}

/// C: `bool_bf wants_modeline : 1;` (frame.h 402, `wants_modeline`); the reader is
/// `FRAME_WANTS_MODELINE_P` (frame.h 1183)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_wants_modeline (f: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_WANTS_MODELINE_SHIFT)) | ((v as u32) << FRAME_WANTS_MODELINE_SHIFT);
  }
}

/// C: ENUM_BF (output_method) output_method : 4;   (frame.h 446; `ENUM_BF` is three bits of
/// padding plus the four-bit field, which is why the port's mask is 0x0F at shift 21)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_output_method (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(FRAME_OUTPUT_METHOD_MASK << FRAME_OUTPUT_METHOD_SHIFT))
            | (((v as u32) & FRAME_OUTPUT_METHOD_MASK) << FRAME_OUTPUT_METHOD_SHIFT);
  }
}

/// C: bool_bf tab_bar_redisplayed : 1;   (frame.h 503)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_tab_bar_redisplayed (f: *mut c_void) -> bool
{
  frame_flag (f.cast (), FRAME_TAB_BAR_REDISPLAYED_UNIT, FRAME_TAB_BAR_REDISPLAYED_SHIFT)
}

/// C: bool_bf tab_bar_resized : 1;   (frame.h 504)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_tab_bar_resized (f: *mut c_void) -> bool
{
  frame_flag (f.cast (), FRAME_TAB_BAR_RESIZED_UNIT, FRAME_TAB_BAR_RESIZED_SHIFT)
}

/// C: bool_bf tab_bar_resized : 1;   (frame.h 504)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_tab_bar_resized (f: *mut c_void, v: bool)
{
  frame_set_flag (f.cast (), FRAME_TAB_BAR_RESIZED_UNIT, FRAME_TAB_BAR_RESIZED_SHIFT, v as u32);
}

/// C: Lisp_Object tab_bar_window;   (frame.h 254, **inside `#ifdef HAVE_WINDOW_SYSTEM`**)
///
/// The member exists only on window-system targets.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_tab_bar_window (f: *mut c_void) -> Lisp_Object
{
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
  #[cfg(feature = "have_window_system")]
  { (*f.cast::<frame>()).tab_bar_window }
  #[cfg(not(feature = "have_window_system"))]
  { let _ = f; Qnil() }

  }
}

/// C: #ifdef HAVE_WINDOW_SYSTEM / #define FRAME_EXTERNAL_MENU_BAR(f) (f)->external_menu_bar /
/// C: #else / #define FRAME_EXTERNAL_MENU_BAR(f) false / #endif   (frame.h 1080-1082)
///
/// Native targets write the measured bitfield; terminal targets have no such member.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_external_menu_bar (f: *mut c_void, v: c_int)
{
  #[cfg(feature = "have_ext_menu_bar")]
  { frame_set_flag(f.cast(), FRAME_EXTERNAL_MENU_BAR_FLAG_UNIT,
                   FRAME_EXTERNAL_MENU_BAR_FLAG_SHIFT, v as u32); }
  #[cfg(not(feature = "have_ext_menu_bar"))]
  { let _ = (f, v); }
}

/// C: #ifdef HAVE_WINDOW_SYSTEM / #define FRAME_EXTERNAL_TOOL_BAR(f) (f)->external_tool_bar /
/// C: #else / #define FRAME_EXTERNAL_TOOL_BAR(f) false / #endif   (frame.h 1095-1097)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_external_tool_bar (f: *mut c_void, v: c_int)
{
  #[cfg(feature = "have_ext_tool_bar")]
  { frame_set_flag(f.cast(), FRAME_EXTERNAL_TOOL_BAR_FLAG_UNIT,
                   FRAME_EXTERNAL_TOOL_BAR_FLAG_SHIFT, v as u32); }
  #[cfg(not(feature = "have_ext_tool_bar"))]
  { let _ = (f, v); }
}

/* ---------------------------------------------------------------------------------------------
   Batch 5, group 1: the `struct frame` members the Swift lanes' `nsfns.m` port reads and writes.
   Every member here is in the frozen record (`include/frame.rs`), so the export carries no gate;
   the flag positions are that mirror's `FRAME_*_SHIFT` constants, and each `/// C:` line is C's
   own text with the reference's line number.
   --------------------------------------------------------------------------------------------- */

/// `frame_name` is the ABI of C's `f->name` (`struct frame` member, frame.h 154); the Swift unit
/// `nsfns.swift` imports it (`nsfns.swift` 252) and its `nsfns.m` port reads it in six places.
/// C: Lisp_Object name;   (frame.h 154)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_name (f: *mut c_void) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).name }
}

/// C: struct terminal *terminal;   (frame.h 648; C's writes are src/nsfns.m 1267 and 2988,
/// `f->terminal = dpyinfo->terminal;`)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_terminal (f: *mut c_void, t: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).terminal = t.cast::<terminal> (); }
}

/// C: bool_bf no_split : 1;   (frame.h 411; C's writes are src/nsfns.m 1474 `f->no_split = …`
/// and 3139 `f->no_split = true;`)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_no_split (f: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_NO_SPLIT_SHIFT)) | ((v as u32) << FRAME_NO_SPLIT_SHIFT);
  }
}

/// C: bool_bf undecorated : 1;   (frame.h 449; C's write is src/nsfns.m 1404
/// `FRAME_UNDECORATED (f) = !NILP (tem) && !EQ (tem, Qunbound);`)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_undecorated (f: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_UNDECORATED_SHIFT)) | ((v as u32) << FRAME_UNDECORATED_SHIFT);
  }
}

/// C: bool_bf can_set_window_size : 1;   (frame.h 491; C's writes are src/nsfns.m 1545 and 3149,
/// `f->can_set_window_size = true;`)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_can_set_window_size (f: *mut c_void, v: bool)
{
  frame_set_flag (f.cast (), FRAME_CAN_SET_WINDOW_SIZE_UNIT,
                  FRAME_CAN_SET_WINDOW_SIZE_SHIFT, v as u32);
}

/// C: bool_bf was_invisible : 1;   (frame.h 530; the second flag word, so this writer and the
/// `was_invisible!` macro of `frame.rs` 276-278 address the same bit; C's write is src/nsfns.m
/// 1569 `f->was_invisible = true;`)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_was_invisible (f: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits2;
    *bits = (*bits & !(1 << FRAME_WAS_INVISIBLE_SHIFT)) | ((v as u32) << FRAME_WAS_INVISIBLE_SHIFT);
  }
}

/* ---------------------------------------------------------------------------------------------
   Batch 5, group 2: the `struct window`, `struct buffer` and `struct terminal` members the same
   Swift lane reads.  Those three records belong to the core and no Swift file declares their
   layout, so the read is a C-ABI entry named after the member - the rule the lanes' own headers
   state (`nsfns.swift` 503-505, 370-375, `nsterm.swift` 818-825).
   --------------------------------------------------------------------------------------------- */

/// C: Lisp_Object contents;   (window.h 143)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_contents (w: *mut c_void) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).contents }
}

/// C: struct glyph_matrix *current_matrix;   (window.h 224)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_current_matrix (w: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).current_matrix }
}

/// C: BVAR (XBUFFER (buf), filename)   (buffer.h 308; the member `BVAR` concatenates is
/// `Lisp_Object filename_;`, buffer.h 330)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn buffer_filename (b: *mut c_void) -> Lisp_Object
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*b.cast::<buffer> ()).filename_ }
}

/// C: int id;   (termhooks.h 516)
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn terminal_id (t: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*t.cast::<terminal> ()).id }
}

/// C: union display_info { … struct ns_display_info *ns; … };   (termhooks.h 533, the member at
/// 538 - "nsterm.h").  The port keeps the untyped pointer the union holds (P-72 rule 3), so the
/// read is a plain pointer even though the Swift declaration names `ns_display_info`.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn terminal_display_info_ns (t: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*t.cast::<terminal> ()).display_info.ns }
}

/* ---------------------------------------------------------------------------------------------
   Batch 5, group 3: the arm-only writers.  `struct ns_output` (nsterm.rs 419-489) and
   `struct ns_display_info` (nsterm.rs 317-392) are `#[cfg (feature = "have_ns")]`, and `HAVE_NS`
   is undef in the frozen configuration (`docs/baseline/config/darwin-arm64.macros.tsv` 1481), so
   each export carries the gate of the record it writes - the `frame_set_double_buffered` ruling
   (`docs/status.md` 3.255 item 3): the ratchet row stays with "only a HAVE_NS build can clear
   it".  Exporting any of them ungated would invent a host for a member the frozen record does not
   have (P-08).
   --------------------------------------------------------------------------------------------- */

/// C's prototype is lisp.h's (`extern void *xzalloc (size_t)`); the port's definition is
/// `src/alloc.rs` 775-783 (`#[unsafe(no_mangle)] pub unsafe extern "C" fn xzalloc`), which
/// is the symbol the frozen build's `_xzalloc` resolves to.
#[cfg (feature = "have_ns")]
unsafe extern "C-unwind" { fn xzalloc (size: size_t) -> *mut c_void; }

/// C: f->output_data.ns = xzalloc (sizeof *f->output_data.ns);   (src/nsfns.m 1270 and 2991)
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_alloc_ns_output (f: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*f.cast::<frame> ()).output_data.ns = xzalloc (std::mem::size_of::<ns_output> ());
  }
}

/// C: FRAME_FONTSET (f) = -1;   (src/nsfns.m 1272 and 2994; the macro is nsterm.h 1096,
/// `#define FRAME_FONTSET(f) ((f)->output_data.ns->fontset)`)
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_fontset (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let ns = (*f.cast::<frame> ()).output_data.ns as *mut ns_output;
    (*ns).fontset = v;
  }
}

/// C: FRAME_DISPLAY_INFO (f) = dpyinfo;   (src/nsfns.m 1280 and 2995; the macro is nsterm.h 1032,
/// `#define FRAME_DISPLAY_INFO(f) ((f)->output_data.ns->display_info)`)
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_display_info (f: *mut c_void, dpyinfo: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let ns = (*f.cast::<frame> ()).output_data.ns as *mut ns_output;
    (*ns).display_info = dpyinfo as *mut ns_display_info;
  }
}

/// C: #if defined (NS_IMPL_COCOA) && MAC_OS_X_VERSION_MIN_REQUIRED >= 101400 /
/// C: #define FRAME_DOUBLE_BUFFERED(f) ((f)->output_data.ns->double_buffered) / #endif
/// (nsterm.h 1051-1053; C's writes are src/nsfns.m 813 `FRAME_DOUBLE_BUFFERED (f) = NILP
/// (new_value);` and 1464 `FRAME_DOUBLE_BUFFERED (f) = NILP (tem) || EQ (tem, Qunbound);`, and
/// the Swift import is `nsfns.swift` 495-496).
///
/// The gate is the mirror's own pair, `have_ns` + `ns_impl_cocoa` (nsterm.rs 582-584 on the
/// macro, 487-488 on the member): the member lives on `ns_output`, itself `have_ns`-gated, so a
/// lone `ns_impl_cocoa` gate would name a type the mirror does not have in that configuration.
/// Status 3.255 item 3 rules the export out of the ungated set; this is that export.
#[cfg (feature = "have_ns")]
#[cfg (feature = "ns_impl_cocoa")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_double_buffered (f: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let ns = (*f.cast::<frame> ()).output_data.ns as *mut ns_output;
    (*ns).double_buffered = v;
  }
}

/* ---------------------------------------------------------------------------------------------
   Batch 6, group 1: `struct ns_output`'s member accessors - the whole `ns_output_*` /
   `ns_output_set_*` family the ratchet carried (14 reads + 28 writes).  Every entry reads or
   writes a member of `struct ns_output` (nsterm.h 935-1022), whose mirror is
   `include/nsterm.rs` 419-489; C keeps that record behind `#ifdef HAVE_NS`, and the mirror keeps
   the gate, so **all 42 carry it** - the `frame_set_double_buffered` shape of status 3.255 item
   3 - and the frozen object answers none of them (the expected reading; `## Evidence`).
   `HAVE_NS`, `NS_IMPL_COCOA` and `NS_IMPL_GNUSTEP` are all undef in the frozen configuration
   (`docs/baseline/config/darwin-arm64.macros.tsv` 1481/2996/2997), so the arm's text is recorded
   rather than compiled (P-51).

   Why these are A-class entries and not B: ruling 3.255 makes the Swift-side expansion the
   first choice when the importing file has declared the record's layout, and the family's
   importing units have no `ns_output` prefix (`nsfns.swift` 246-248's own comment says the names
   stand for members of a record the core owns); expanding them needs the record's layout, which
   is a core fact.  The bodies spell C's own accessor where C has one (`FRAME_NS_VIEW`,
   `FRAME_NS_WINDOW`, `FRAME_FOREGROUND_COLOR`, `FRAME_BACKGROUND_COLOR`, `FRAME_CURSOR_COLOR`,
   `FRAME_DISPLAY_INFO` - the mirror's `macro_rules!`, nsterm.rs 513-577) and C's own
   `f->output_data.ns->member` chain (through the mirror's `FRAME_OUTPUT_DATA!`) otherwise, so
   each entry stands for the access C's `.m` files make.
   --------------------------------------------------------------------------------------------- */

/// C: `EmacsView *view;` (nsterm.h 938; the frozen `#else` half is `void *view;` 948) and
/// C: `#define FRAME_NS_VIEW(f) ((f)->output_data.ns->view)` (nsterm.h 1045) - C reads it at
/// C: nsfns.m 145, 281.
/// Swift `nsfns.swift` 275 (`(UnsafeMutableRawPointer?) -> NSView?`); the write's entry is
/// `ns_output_set_view` below.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_view (f: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { FRAME_NS_VIEW! (f.cast::<frame> ()) }
}

/// C: `id miniimage;` (nsterm.h 939; the frozen `#else` half is `void *miniimage;` 949) - C
/// C: reads it at nsfns.m 858 and nsterm.m 9729.  No accessor macro in C, so the body spells
/// C: `f->output_data.ns->miniimage` itself.
/// Swift `nsfns.swift` 278 (`-> AnyObject?`) and `nsterm.swift` 1850 (`-> NSImage?`) - one
/// C-ABI entry, two declarations; both spellings are one pointer.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_miniimage (f: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).miniimage
  }
}

/// C: `NSColor *cursor_color;` (nsterm.h 940; the frozen `#else` half is `void *cursor_color;`
/// C: 950) and `#define FRAME_CURSOR_COLOR(f) ((f)->output_data.ns->cursor_color)` (nsterm.h
/// C: 1046) - C reads it at nsterm.m 3283.
/// Swift `nsfns.swift` 282 (`-> NSColor?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_cursor_color (f: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { FRAME_CURSOR_COLOR! (f.cast::<frame> ()) }
}

/// C: `NSColor *foreground_color;` (nsterm.h 941; the frozen `#else` half is
/// C: `void *foreground_color;` 951) and `#define FRAME_FOREGROUND_COLOR(f)
/// C: ((f)->output_data.ns->foreground_color)` (nsterm.h 1037) - C reads it at nsfont.m 1198
/// C: and nsterm.m 4878.
/// Swift `nsfns.swift` 286 (`-> NSColor?`); the write's entry is below.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_foreground_color (f: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { FRAME_FOREGROUND_COLOR! (f.cast::<frame> ()) }
}

/// C: `NSColor *background_color;` (nsterm.h 942; the frozen `#else` half is
/// C: `void *background_color;` 952) and `#define FRAME_BACKGROUND_COLOR(f)
/// C: ((f)->output_data.ns->background_color)` (nsterm.h 1038) - C reads it at nsterm.m 2362.
/// Swift `nsfns.swift` 290 (`-> NSColor?`); the write's entry is below.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_background_color (f: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { FRAME_BACKGROUND_COLOR! (f.cast::<frame> ()) }
}

/// C: `int icon_top;` (nsterm.h 990) - C writes it at nsfns.m 1036 (`= -1;`) and 1048
/// C: (`= XFIXNUM (icon_y);`), and the reads the Swift unit makes are of the same member.
/// Swift `nsfns.swift` 294 (`-> Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_icon_top (f: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).icon_top
  }
}

/// C: `int icon_left;` (nsterm.h 991) - C writes it at nsfns.m 1037 (`= -1;`) and 1049
/// C: (`= XFIXNUM (icon_x);`).
/// Swift `nsfns.swift` 297 (`-> Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_icon_left (f: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).icon_left
  }
}

/// C: `Window window_desc, parent_desc;` (nsterm.h 980) and `#define FRAME_NS_WINDOW(f)
/// C: ((f)->output_data.ns->window_desc)` (nsterm.h 1034) - C writes it at nsfns.m 1285
/// C: (`f->output_data.ns->window_desc = desc_ctr++;`) and reads it at nsfns.m 654.  The
/// C: member is C's `Window` (`typedef int Window`, nsgui.h 67) while the lane's declaration is
/// C: `UInt`; the ABI is the declaration (`docs/ports/abi_shim.md`), so the read widens.
/// Swift `nsfns.swift` 300 (`-> UInt`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_window_desc (f: *mut c_void) -> c_ulong
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { FRAME_NS_WINDOW! (f.cast::<frame> ()) as c_ulong }
}

/// C: `Window window_desc, parent_desc;` (nsterm.h 980) - C writes `parent_desc` at nsfns.m
/// C: 1288 (`= XFIXNAT (parent);`), 1293 and 3076 (`= FRAME_DISPLAY_INFO (f)->root_window;`).
/// C: The member is C's `Window` (`int`) and the lane's declaration is `UInt`, so the read
/// C: widens (the `ns_output_window_desc` note).
/// Swift `nsfns.swift` 303 (`-> UInt`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_parent_desc (f: *mut c_void) -> Window
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).parent_desc
  }
}

/// C: `char explicit_parent;` (nsterm.h 981) - C writes it at nsfns.m 1289 (`= 1;`), 1294
/// C: (`= 0;`) and tests it at 1550 (`if (! f->output_data.ns->explicit_parent)`), i.e. C uses
/// C: the byte as a truth value.  The lane's declaration is `Bool`, so the read is `!= 0`.
/// Swift `nsfns.swift` 306 (`-> Bool`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_explicit_parent (f: *mut c_void) -> bool
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).explicit_parent != 0
  }
}

/// C: `Emacs_Cursor text_cursor;` (nsterm.h 960; `Emacs_Cursor` is `void *` in the frozen
/// C: half, nsgui.rs 57) - C reads it at nsfns.m 1498
/// C: (`f->output_data.ns->current_pointer = f->output_data.ns->text_cursor;`) and writes it at
/// C: 1478.
/// Swift `nsfns.swift` 310 (`-> NSObject?`); the write's entry is `ns_output_set_text_cursor`.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_text_cursor (f: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).text_cursor
  }
}

/// C: `int in_animation;` (nsterm.h 1010) - C writes it at nsfns.m 1500 (`= NO;`) and
/// C: nsmenu.m 1109/1111 (`= 1;` / `= 0;`); the reads are the Swift side's.
/// Swift `nsfns.swift` 314 and `nsmenu.swift` 326 (`-> Int32`) - one entry, two units.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_in_animation (f: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).in_animation
  }
}

/// C: `struct ns_display_info *display_info;` (nsterm.h 1004) and `#define FRAME_DISPLAY_INFO(f)
/// C: ((f)->output_data.ns->display_info)` (nsterm.h 1032) - C reads it at nsfns.m 109, 129 and
/// C: writes it at 1280 (`FRAME_DISPLAY_INFO (f) = dpyinfo;`), whose entry landed in batch 5
/// C: (`frame_set_display_info`).  The member's own type is the mirror's (`include/nsterm.rs`),
/// C: so the read returns it rather than degrading to `*mut c_void`.
/// Swift `nsfns.swift` 318 (`-> UnsafeMutablePointer<ns_display_info>`, the unit's own Swift
/// prefix declaration); the ABI is one pointer.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_display_info (f: *mut c_void) -> *mut ns_display_info
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { FRAME_DISPLAY_INFO! (f.cast::<frame> ()) }
}

/// C: `int zooming;` (nsterm.h 1007) - C tests it at nsterm.m 1906 and 1948 (`if
/// C: (f->output_data.ns->zooming)`) and sets it at 1907/1949 (`= 0;`) and 8662 (`= 1;`), i.e.
/// C: the int is a truth value.  The lane's declaration is `Bool`, so the read is `!= 0`.
/// Swift `nsterm.swift` 2011 (`-> Bool`); the write's entry is below.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_zooming (f: *mut c_void) -> bool
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).zooming != 0
  }
}

/// C: `EmacsView *view;` (nsterm.h 938; the frozen half is `void *view;` 948) - C's write is
/// C: `FRAME_NS_VIEW (f) = self;` (nsterm.m 8587, in `[EmacsView initFrameFromEmacs:]`).
/// Swift `nsfns.swift` 276 (`_ v: NSView?`); the read's entry is `ns_output_view`.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_view (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    replace_native_object(&raw mut (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).view, v);
  }
}

/// C: `id miniimage;` (nsterm.h 939; the frozen half is `void *miniimage;` 949) - C writes it
/// C: at nsfns.m 921 (`f->output_data.ns->miniimage = image;`) and 962.
/// Swift `nsfns.swift` 280 (`_ v: AnyObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_miniimage (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    replace_native_object(&raw mut (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).miniimage, v);
  }
}

/// C: `NSColor *cursor_color;` (nsterm.h 940; the frozen half is `void *cursor_color;` 950) -
/// C: C's write is the macro's (`FRAME_CURSOR_COLOR (f) = [col retain];`, nsfns.m 358, after the
/// C: release at 357).
/// Swift `nsfns.swift` 284 (`_ v: NSColor?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_cursor_color (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    replace_native_object(&raw mut (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).cursor_color, v);
  }
}

/// C: `NSColor *foreground_color;` (nsterm.h 941; the frozen half is `void *foreground_color;`
/// C: 951) - C's write is `f->output_data.ns->foreground_color = col;` (nsfns.m 277, after the
/// C: release at 276).
/// Swift `nsfns.swift` 288 (`_ v: NSColor?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_foreground_color (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    replace_native_object(&raw mut (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).foreground_color, v);
  }
}

/// C: `NSColor *background_color;` (nsterm.h 942; the frozen half is `void *background_color;`
/// C: 952) - C's write is `f->output_data.ns->background_color = col;` (nsfns.m 310, after the
/// C: release at 309).
/// Swift `nsfns.swift` 292 (`_ v: NSColor?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_background_color (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    replace_native_object(&raw mut (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).background_color, v);
  }
}

/// C: `int icon_top;` (nsterm.h 990) - C writes it at nsfns.m 1036 (`= -1;`) and 1048
/// C: (`= XFIXNUM (icon_y);`).
/// Swift `nsfns.swift` 295 (`_ v: Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_icon_top (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).icon_top = v;
  }
}

/// C: `int icon_left;` (nsterm.h 991) - C writes it at nsfns.m 1037 (`= -1;`) and 1049
/// C: (`= XFIXNUM (icon_x);`).
/// Swift `nsfns.swift` 298 (`_ v: Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_icon_left (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).icon_left = v;
  }
}

/// C: `Window window_desc, parent_desc;` (nsterm.h 980) - C's write is
/// C: `f->output_data.ns->window_desc = desc_ctr++;` (nsfns.m 1285), where `desc_ctr` is the
/// C: file's `static int`.  The lane's declaration passes `UInt` (`UInt(desc_ctr)` at
/// C: nsfns.swift 2269), so the parameter is `c_ulong` and the store narrows to C's `int`.
/// Swift `nsfns.swift` 301 (`_ v: UInt`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_window_desc (f: *mut c_void, v: c_ulong)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).window_desc = v as c_int;
  }
}

/// C: `Window window_desc, parent_desc;` (nsterm.h 980) - C writes `parent_desc` at nsfns.m
/// C: 1288 (`= XFIXNAT (parent);`), 1293 and 3076 (`= FRAME_DISPLAY_INFO (f)->root_window;`),
/// C: the lane's declaration passes `UInt`, so the parameter is `c_ulong` and the store narrows
/// C: to C's `int` (the `ns_output_set_window_desc` note).
/// Swift `nsfns.swift` 304 (`_ v: UInt`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_parent_desc (f: *mut c_void, v: Window)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).parent_desc = v as c_int;
  }
}

/// C: `char explicit_parent;` (nsterm.h 981) - C writes 1 and 0 (`= 1;` nsfns.m 1289, `= 0;`
/// C: 1294) and tests the byte as a truth value (1550).  The lane's declaration is `Bool`, so
/// C: the store writes C's two byte values back (`v as c_char`).
/// Swift `nsfns.swift` 308 (`_ v: Bool`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_explicit_parent (f: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).explicit_parent = v as c_char;
  }
}

/// C: `Emacs_Cursor current_pointer;` (nsterm.h 977) plus `#define FRAME_POINTER_TYPE(f)
/// C: ((f)->output_data.ns->current_pointer)` (nsterm.h 1047) - C writes it at nsfns.m 1498
/// C: (`= f->output_data.ns->text_cursor;`) and nsterm.m 2634 (`FRAME_POINTER_TYPE (f) =
/// C: cursor;`).
/// Swift `nsfns.swift` 312 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_current_pointer (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    replace_native_object(&raw mut (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).current_pointer, v);
  }
}

/// C: `Emacs_Cursor text_cursor;` (nsterm.h 960; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1478, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->text_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 651 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_text_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    replace_native_object(&raw mut (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).text_cursor, v);
  }
}

/// C: `Emacs_Cursor nontext_cursor;` (nsterm.h 961; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1479, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->nontext_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 653 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_nontext_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).nontext_cursor = v;
  }
}

/// C: `Emacs_Cursor modeline_cursor;` (nsterm.h 962; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1480, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->modeline_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 655 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_modeline_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).modeline_cursor = v;
  }
}

/// C: `Emacs_Cursor hand_cursor;` (nsterm.h 963; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1481, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->hand_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 657 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_hand_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).hand_cursor = v;
  }
}

/// C: `Emacs_Cursor hourglass_cursor;` (nsterm.h 964; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1482, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->hourglass_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 659 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_hourglass_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).hourglass_cursor = v;
  }
}

/// C: `Emacs_Cursor horizontal_drag_cursor;` (nsterm.h 965; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1483, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->horizontal_drag_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 661 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_horizontal_drag_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).horizontal_drag_cursor = v;
  }
}

/// C: `Emacs_Cursor vertical_drag_cursor;` (nsterm.h 966; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1484, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->vertical_drag_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 663 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_vertical_drag_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).vertical_drag_cursor = v;
  }
}

/// C: `Emacs_Cursor left_edge_cursor;` (nsterm.h 967; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1485, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->left_edge_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 665 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_left_edge_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).left_edge_cursor = v;
  }
}

/// C: `Emacs_Cursor top_left_corner_cursor;` (nsterm.h 968; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1486, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->top_left_corner_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 667 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_top_left_corner_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).top_left_corner_cursor = v;
  }
}

/// C: `Emacs_Cursor top_edge_cursor;` (nsterm.h 969; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1487, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->top_edge_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 669 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_top_edge_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).top_edge_cursor = v;
  }
}

/// C: `Emacs_Cursor top_right_corner_cursor;` (nsterm.h 970; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1488, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->top_right_corner_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 671 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_top_right_corner_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).top_right_corner_cursor = v;
  }
}

/// C: `Emacs_Cursor right_edge_cursor;` (nsterm.h 971; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1489, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->right_edge_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 673 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_right_edge_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).right_edge_cursor = v;
  }
}

/// C: `Emacs_Cursor bottom_right_corner_cursor;` (nsterm.h 972; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1490, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->bottom_right_corner_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 675 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_bottom_right_corner_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).bottom_right_corner_cursor = v;
  }
}

/// C: `Emacs_Cursor bottom_edge_cursor;` (nsterm.h 973; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1491, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->bottom_edge_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 677 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_bottom_edge_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).bottom_edge_cursor = v;
  }
}

/// C: `Emacs_Cursor bottom_left_corner_cursor;` (nsterm.h 974; `Emacs_Cursor` is `void *` in the frozen half,
/// C: nsgui.rs 57) - C writes it at nsfns.m 1492, in the 15-cursor block at 1478-1492
/// C: (`[EmacsView initFrameFromEmacs:]`: `f->output_data.ns->bottom_left_corner_cursor = [NSCursor …];`).
/// Swift `nsfns.swift` 679 (`_ v: NSObject?`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_bottom_left_corner_cursor (f: *mut c_void, v: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).bottom_left_corner_cursor = v;
  }
}

/// C: `int in_animation;` (nsterm.h 1010) - C writes it at nsfns.m 1500 (`= NO;`) and nsmenu.m
/// C: 1109/1111 (`= 1;` / `= 0;`).
/// Swift `nsfns.swift` 316 and `nsmenu.swift` 328 (`_ v: Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_in_animation (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).in_animation = v;
  }
}

/// C: `int zooming;` (nsterm.h 1007) - C resets it at nsterm.m 1907 and 1949 (`= 0;`) and sets
/// C: it at 8662 (`emacsframe->output_data.ns->zooming = 1;`).  The lane's declaration is
/// C: `Bool`, so the store writes C's two int values back (`v as c_int`).
/// Swift `nsterm.swift` 2012 (`_ v: Bool`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_set_zooming (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    (*(FRAME_OUTPUT_DATA! (f.cast::<frame> ()) as *mut ns_output)).zooming = v as c_int;
  }
}

/* ---------------------------------------------------------------------------------------------
   Batch 6, group 2: `struct glyph_row`'s two members the nsterm lane reads directly.  The record
   is the core's (`include/dispextern.rs` 773-..., ungated text: `struct glyph_row` is not behind
   a platform switch in C either), so each member access is a C-ABI entry - the rule status
   3.257 item 5 (3) states for this pair ("每次访问就是一条 C-ABI 条目").  Unlike group 1 these
   two carry no gate: the record is live in the frozen configuration.
   --------------------------------------------------------------------------------------------- */

/// C: `int x, y;` (dispextern.h 950, `y`; inside `struct glyph_row` 918) - C reads the y member as
/// `row->y` (nsterm.m 818, in `static NSRect ns_row_rect (struct window *w, struct glyph_row
/// *row, enum glyph_row_area area)` 807-824).
/// Swift `nsterm.swift` 834 (`(UnsafeMutableRawPointer?) -> Int32`), used at `nsterm.swift` 980.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_row_y (row: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*row.cast::<glyph_row> ()).y }
}

/// C: `int visible_height;` (dispextern.h 969, `visible_height`; inside `struct glyph_row` 918) - C reads it
/// as `row->visible_height`; the same `ns_row_rect` (nsterm.m 808, its read at 821).
/// Swift `nsterm.swift` 835-836 (`(UnsafeMutableRawPointer?) -> Int32`), used at
/// `nsterm.swift` 983.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_row_visible_height (row: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*row.cast::<glyph_row> ()).visible_height }
}

/* ---------------------------------------------------------------------------------------------
   Batch 6, group 3: `thread.h`'s `waiting_for_input` flag, written through the mirror's own
   macro.  `struct thread_state` (thread.h 66-...) is the core's record and is reachable here
   through `include/lisp.rs` 1442's `include!("thread.rs")`, so this one carries no gate.
   --------------------------------------------------------------------------------------------- */

/// C: `#define waiting_for_input (current_thread->m_waiting_for_input)` (thread.h 185, over
/// `bool m_waiting_for_input;` 184) - C's write is `waiting_for_input = 0;` (nsmenu.m 202, right
/// after the read into `owfi` at 201).  The body is the mirror's own accessor
/// (`include/thread.rs`'s `macro_rules! waiting_for_input`, thread.h 185), i.e. the same
/// expression the core's readers use, so writer and reader cannot drift apart.
/// Swift `nsmenu.swift` 407 (`func waiting_for_input_set(_ v: Bool)`); the read's entry
/// (`waiting_for_input`, `nsmenu.swift` 406) is still owed and is *not* part of this batch.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn set_waiting_for_input_flag (v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { waiting_for_input! () = v; }
}

/* ---------------------------------------------------------------------------------------------
   Batch 6, group 4: `struct face`'s `background`, behind the mirror's own gate.  C has `struct
   face` unconditionally (dispextern.h 1787-1937), but the port's copy sits behind
   `#[cfg (feature = "have_ns")]` (`include/dispextern.rs` 1941-1943) because the
   units that keep typed copies of it (`xfaces.rs`, `term.rs`, `alloc.rs`) are not yet closed
   under the flip (`include/frame.rs` 698-718), so the frozen configuration has no `face` name
   for this body to reach and the export can only exist in that arm - the
   `frame_set_double_buffered` shape again (`## Decisions`).
   --------------------------------------------------------------------------------------------- */

/// C: `unsigned long background;` (dispextern.h 1819, `background`; inside `struct face` 1787) - C's
/// write is `face->background = [[col colorWithAlphaComponent: alpha] unsignedLong];`
/// (nsfns.m 328-329, whose right-hand side reads `NS_FACE_BACKGROUND (face)` at 327).
/// Swift `nsfns.swift` 475-476 (`func face_set_background(_ face: UnsafeMutableRawPointer?,
/// _ v: UInt)`), used at `nsfns.swift` 1404.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn face_set_background (face: *mut c_void, v: c_ulong)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*face.cast::<face> ()).background = v; }
}

/* ---------------------------------------------------------------------------------------------
   Batch 7, group 1: the two entries of the nsterm lane's tenth slice (status 3.259) that the
   frozen mirror can host.  Both names are new to this file and free at the crate root, so they
   are ordinary exports; the rest of that slice's list is held back for the reasons `## Decisions`
   gives (missing hosts) or is not the core's (the `Q*` family, `setAppearance`).
   --------------------------------------------------------------------------------------------- */

/// C: `#define FRAME_NO_ACCEPT_FOCUS(f) ((f)->no_accept_focus)` (frame.h 1249, `FRAME_NO_ACCEPT_FOCUS`) - the `bool_bf`
/// member at frame.h 411, written at src/nsterm.m 2074 (`FRAME_NO_ACCEPT_FOCUS (f) = !NILP
/// (new_value);`).  The write uses the mirror's own shift constant
/// (`FRAME_NO_ACCEPT_FOCUS_SHIFT` 26, `include/frame.rs` 170) - the same bit `src/frame.rs`
/// 244-247's reader/writer macro uses - so writer and reader cannot drift apart (the batch-5
/// `frame_set_no_split` shape).
/// Swift `nsterm.swift` 2162-2163 (`_ v: Bool`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_no_accept_focus (f: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let bits = &mut (*f.cast::<frame> ()).bits1;
    *bits = (*bits & !(1 << FRAME_NO_ACCEPT_FOCUS_SHIFT)) | ((v as u32) << FRAME_NO_ACCEPT_FOCUS_SHIFT);
  }
}

/// C: `int menu_bar_items_used;` (frame.h 307) - C's writes are src/nsmenu.m 303
/// (`f->menu_bar_items_used = menu_items_used;`) and 359 (`= 0;`).  The reader landed in batch 1
/// (`frame_menu_bar_items_used`, above), and the member is the mirror's own (`include/frame.rs`
/// 69), so this closes the ruling's class-1 pair for the name.
/// Swift `nsmenu.swift` 316-317 (`(UnsafeMutableRawPointer?, Int32)`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_menu_bar_items_used (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).menu_bar_items_used = v; }
}

/// C: `double alpha[2];` (frame.h 707-711, "alpha[0]: alpha transparency of the active frame,
/// alpha[1]: alpha transparency of inactive frames") - read at src/nsterm.m 2378/2380
/// (`alpha = f->alpha[0];` / `alpha = f->alpha[1];`) with C's own subscript, which is the
/// entry's second argument (`nsterm.swift` 2235-2236).
/// Swift `nsterm.swift` 2235-2236 (`(UnsafeMutableRawPointer?, Int32) -> Double`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_alpha (f: *mut c_void, index: c_int) -> c_double
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).alpha[index as usize] }
}

/* ---------------------------------------------------------------------------------------------
   Batch 7, group 2: the entries whose *names* the mirrors already spell in Rust.  C's
   enumerators (`enum z_group`, frame.h 61-67) and header accessors (`FLOATP` lisp.h 448,
   `XFLOAT_DATA` lisp.h 3186) have no symbol in C either, and the port already carries each value
   under its own name: `include/frame.rs` 752-756's `pub const z_group_none` ... and
   `include/lisp.rs` 916/1695's `pub fn FLOATP`/`XFLOAT_DATA`.  A crate-root definition of the
   same name beside them is `E0428` (measured, `## Evidence`), so the C-ABI spelling lives in a
   module of its own - the `include/thread.rs` 281-288 `thread_objs` and `include/frame.rs` 720's
   `frame_face_accessors` shape - and each body calls the mirror's own item through `super::`, so
   the value has one source of truth.  `use super::*` is what puts the crate root's types
   (`Lisp_Object`, `c_int`) in scope here; the `super::` prefixes are deliberate, because a glob
   import is shadowed by the module's own item of the same name.
   --------------------------------------------------------------------------------------------- */

#[allow(unused_imports)]
pub mod c_abi_of_mirror_named_items
{
  use super::*;

  /// C: `enum z_group { z_group_none, z_group_above, z_group_below, z_group_above_suspended, };`
  /// (frame.h 61-67, inside `#ifdef HAVE_WINDOW_SYSTEM`; the port carries the four members
  /// because C's live `#else` arm of `FRAME_Z_GROUP` names one - `include/frame.rs` 748-756).
  /// The Swift import is `nsterm.swift` 2169 (`func z_group_none() -> Int32`), the lane's
  /// function-shaped spelling of a C enumerator (status 3.255 lists enumerators as A class).
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn z_group_none () -> c_int { super::z_group_none as c_int }

  /// C: `z_group_above,` (frame.h 64), value 1 (`include/frame.rs` 754).
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn z_group_above () -> c_int { super::z_group_above as c_int }

  /// C: `z_group_below,` (frame.h 65), value 2 (`include/frame.rs` 755).
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn z_group_below () -> c_int { super::z_group_below as c_int }

  /// C: `z_group_above_suspended,` (frame.h 66), value 3 (`include/frame.rs` 756).
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn z_group_above_suspended () -> c_int
  { super::z_group_above_suspended as c_int }

  /// C: `#define FLOATP(x) lisp_h_FLOATP (x)` (src/lisp.h 448, over the INLINE at 445-447) - the
  /// header-only predicate the lane imports as a function (`nsterm.swift` 2228).  The body is the
  /// mirror's own `FLOATP` (`include/lisp.rs` 916, which is `lisp_h_FLOATP`), so the tag test has
  /// one spelling in this tree.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn FLOATP (x: Lisp_Object) -> bool { super::FLOATP (x) }

  /// C: `INLINE double XFLOAT_DATA (Lisp_Object f)` (src/lisp.h 3186-3189,
  /// `return XFLOAT (f)->u.data;`) - the lane's import is `nsterm.swift` 2229.  The body is the
  /// mirror's own `XFLOAT_DATA` (`include/lisp.rs` 1695).
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn XFLOAT_DATA (f: Lisp_Object) -> f64 { super::XFLOAT_DATA (f) }

  /* -------------------------------------------------------------------------------------------
     Batch 8: the head of the `Lisp_Object` accessor family - the two universal symbols
     (`Qnil`, `Qt`), the four `Q*` of the nsterm lane's slice (the names status 3.264 item 3
     ruled on), and the predicate/list accessors the same Swift import blocks already group as
     "header-only lisp.h constructs" (`nsfns.swift` 178-196, `nsterm.swift` 1137-1144).  Every
     one is A class by the 3.255 test: the Swift lane declares `Lisp_Object` as an opaque
     pointer (`typealias Lisp_Object = UnsafeMutableRawPointer?`, `nsfns.swift` 29 /
     `macfont.swift` 38), so no expansion of `XLI`/`TAGGEDP`/`XCONS`/`XSTRING` is expressible
     there - the representation is a core-internal fact.  The port already carries each name
     (`include/globals.rs`'s `pub fn Q*`, `include/lisp.rs`'s `pub fn NILP` ...), and a
     crate-root definition of the same name beside them is `E0428` (status 3.264 item 2), so
     the C-ABI spelling stands here and each body calls the mirror's own item through
     `super::`.
     ------------------------------------------------------------------------------------------- */

  /// C: `#define Qnil builtin_lisp_symbol (0)` (build/emacs-ref/src/globals.h 7363, `Qnil`; `iQnil` 0
  /// at 1242) - the port's symbol is `include/globals.rs` 10's `pub fn Qnil`, i.e. P-17's call
  /// shape.  The Swift import is `nsfns.swift` 323 (`func Qnil() -> Lisp_Object`), the head of
  /// the seven units that spell it this way (`nsterm.swift` 157, `nsmenu.swift` 367,
  /// `nsselect.swift` 190, `nsimage.swift` 131, `nsfont.swift` 532, `macfont.swift` 868).
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn Qnil () -> Lisp_Object { super::Qnil () }

  /// C: `# define Qt builtin_lisp_symbol (1)` (build/emacs-ref/src/globals.h 7365, inside
  /// `#if DEFINE_NON_NIL_Q_SYMBOL_MACROS` - defined in the frozen configuration; `iQt` 1 at
  /// 1244) - the port's `include/globals.rs` 13.  Swift: `nsfns.swift` 324, `nsmenu.swift` 368,
  /// `nsselect.swift` 191.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn Qt () -> Lisp_Object { super::Qt () }

  /// C: `# define Qabove builtin_lisp_symbol (218)` (build/emacs-ref/src/globals.h 7582;
  /// `iQabove` 218 at 1678, `DEFINE_LISP_SYMBOL (Qabove)` 1679) - C's `DEFSYM (Qabove,
  /// "above")` is src/window.c 9251, and the port's symbol is `include/globals.rs` 991.  Swift:
  /// `nsterm.swift` 2191, whose own comment reads `_Qdark` and friends as `S` in
  /// `build/emacs-ref/src/` (the data symbol 3.264 item 3 rules on).
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn Qabove () -> Lisp_Object { super::Qabove () }

  /// C: `#define iQabove_suspended 220` (build/emacs-ref/src/globals.h 1682) - C's
  /// `DEFSYM (Qabove_suspended, "above-suspended")` is src/frame.c 7368; the port's symbol is
  /// `include/globals.rs` 997.  Swift: `nsterm.swift` 2192.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn Qabove_suspended () -> Lisp_Object { super::Qabove_suspended () }

  /// C: `#define iQbelow 278` (build/emacs-ref/src/globals.h 1798) - C's `DEFSYM (Qbelow,
  /// "below")` is src/frame.c 7367; the port's symbol is `include/globals.rs` 1249.  Swift:
  /// `nsterm.swift` 2193.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn Qbelow () -> Lisp_Object { super::Qbelow () }

  /// C: `# define Qdark builtin_lisp_symbol (447)` (build/emacs-ref/src/globals.h 7811;
  /// `iQdark` 447 at 2136) - C's `DEFSYM (Qdark, "dark")` is src/nsfns.m 4207 (and src/frame.c
  /// 7905); the port's symbol is `include/globals.rs` 1906.  Swift: `nsterm.swift` 2194.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn Qdark () -> Lisp_Object { super::Qdark () }

  /// C: `# define NILP(x) lisp_h_NILP (x)` (src/lisp.h 450, over `#define lisp_h_NILP(x)
  /// BASE_EQ (x, Qnil)` 397 and the `INLINE bool (NILP) (Lisp_Object)` 1447-1452) - the
  /// header-only predicate, so the core exports the same name.  The body is the mirror's own
  /// `NILP` (`include/lisp.rs` 843).  Swift: `nsfns.swift` 180, plus `nsterm.swift` 818,
  /// `nsmenu.swift` 252, `nsselect.swift` 152, `nsfont.swift` 476, `macfont.swift` 741.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn NILP (x: Lisp_Object) -> bool { super::NILP (x) }

  /// C: `# define EQ(x, y)` -> `INLINE bool EQ (Lisp_Object, Lisp_Object)` (src/lisp.h
  /// 1314-1325, over `BASE_EQ` 390-460) - header-only.  The body is the mirror's own `EQ`
  /// (`include/lisp.rs` 819, which is C's body line for line, `symbols_with_pos_enabled`
  /// included).  Swift: `nsfns.swift` 181, plus `nsterm.swift` 137, `nsmenu.swift` 251,
  /// `nsselect.swift` 151, `nsfont.swift` 478, `macfont.swift` 649, `nsimage.swift` 117 - the
  /// most widely imported name of this batch.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn EQ (x: Lisp_Object, y: Lisp_Object) -> bool { super::EQ (x, y) }

  /// C: `# define CONSP(x) lisp_h_CONSP (x)` (src/lisp.h 446, over `#define lisp_h_CONSP(x)
  /// TAGGEDP (x, Lisp_Cons)` 392 and the `INLINE bool (CONSP)` 1455-1458) - the tag test is the
  /// core's.  Body: the mirror's `CONSP` (`include/lisp.rs` 851).  Swift: `nsfns.swift` 183,
  /// `nsterm.swift` 1142 (the `FOR_EACH_FRAME` list walker), `nsselect.swift` 153,
  /// `nsfont.swift` 480, `macfont.swift` 651.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn CONSP (x: Lisp_Object) -> bool { super::CONSP (x) }

  /// C: `INLINE bool SYMBOLP (Lisp_Object)` (src/lisp.h 1074-1079: `BARE_SYMBOL_P (x) ||
  /// (symbols_with_pos_enabled && SYMBOL_WITH_POS_P (x))`) - header-only.  Body: the mirror's
  /// `SYMBOLP` (`include/lisp.rs` 639).  Swift: `nsfns.swift` 185, `nsterm.swift` 135,
  /// `nsselect.swift` 156, `nsfont.swift` 484, `macfont.swift` 653.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn SYMBOLP (x: Lisp_Object) -> bool { super::SYMBOLP (x) }

  /// C: `INLINE bool STRINGP (Lisp_Object)` (src/lisp.h 1581-1584: `TAGGEDP (x, Lisp_String)`)
  /// - header-only.  Body: the mirror's `STRINGP` (`include/lisp.rs` 919).  Swift:
  /// `nsfns.swift` 184, `nsterm.swift` 2212 (`ns_lisp_to_color`), `nsmenu.swift` 253,
  /// `nsselect.swift` 155, `nsimage.swift` 118.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn STRINGP (x: Lisp_Object) -> bool { super::STRINGP (x) }

  /// C: `# define FIXNUMP(x) lisp_h_FIXNUMP (x)` (src/lisp.h 449, over `#define
  /// lisp_h_FIXNUMP(x) ((XTYPE (x) & (Lisp_Int0 | ~Lisp_Int1)) == Lisp_Int0)` 395 and the
  /// `INLINE bool (FIXNUMP)` 1263-1266) - header-only.  Body: the mirror's `FIXNUMP`
  /// (`include/lisp.rs` 721).  Swift: `nsfns.swift` 190, `nsterm.swift` 2230,
  /// `nsselect.swift` 154, `nsimage.swift` 119, `nsfont.swift` 482, `macfont.swift` 655.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn FIXNUMP (x: Lisp_Object) -> bool { super::FIXNUMP (x) }

  /// C: `# define XCAR(c) lisp_h_XCAR (c)` (src/lisp.h 455, over `#define lisp_h_XCAR(c)
  /// XCONS (c)->u.s.car` 405 and the `INLINE Lisp_Object (XCAR)` 1492-1497) - the read is
  /// `struct Lisp_Cons`'s layout, a core-internal fact.  Body: the mirror's `XCAR`
  /// (`include/lisp.rs` 881).  Swift: `nsfns.swift` 195, `nsterm.swift` 1143, `nsselect.swift`
  /// 158, `nsfont.swift` 486, `macfont.swift` 784.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn XCAR (c: Lisp_Object) -> Lisp_Object { super::XCAR (c) }

  /// C: `# define XCDR(c) lisp_h_XCDR (c)` (src/lisp.h 456, over `#define lisp_h_XCDR(c)
  /// XCONS (c)->u.s.u.cdr` 406 and the `INLINE Lisp_Object (XCDR)` 1499-1503).  Body: the
  /// mirror's `XCDR` (`include/lisp.rs` 884).  Swift: `nsfns.swift` 196, `nsterm.swift` 1144,
  /// `nsselect.swift` 159, `nsimage.swift` 121, `nsfont.swift` 488, `macfont.swift` 785.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn XCDR (c: Lisp_Object) -> Lisp_Object { super::XCDR (c) }

  /// C: `INLINE void set_hash_value_slot (struct Lisp_Hash_Table *h, ptrdiff_t idx, Lisp_Object
  /// val)` (src/lisp.h 3982-3985, `h->key_and_value[2 * idx + 1] = val`) - header-only, and the
  /// read is `struct Lisp_Hash_Table`'s layout, a core-internal fact.  The port already carries
  /// the name (`include/lisp.rs` 2505), so the C-ABI spelling stands here and the body calls it;
  /// the mirror's `idx` is `hash_idx_t` (`include/lisp.rs` 2262, C's `ptrdiff_t` folded to i32),
  /// while the lane declares the argument `Int` (`macfont.swift` 834), so the conversion is
  /// explicit and the declaration's width wins (the `## Decisions` rule).
  /// Swift `macfont.swift` 834-835 (`(UnsafeMutableRawPointer?, Int, Lisp_Object)`), used at
  /// `macfont.swift` 1703.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn set_hash_value_slot (h: *mut c_void, idx: c_long, val: Lisp_Object)
  {
    super::set_hash_value_slot (h.cast::<Lisp_Hash_Table> (), idx as hash_idx_t, val)
  }

  /* -------------------------------------------------------------------------------------------
     Batch 11, group 2: `enum glyph_type`'s two enumerators this slice compares glyph types
     against (src/dispextern.h 431-452, `CHAR_GLYPH` 434 = 0 and `COMPOSITE_GLYPH` 437 = 1; C's
     reads are nsterm.m 3054/3055/3059).  Both names are already Rust items in the mirror
     (`pub const CHAR_GLYPH: glyph_type = 0`, `include/dispextern.rs` 240), so the C-ABI
     spelling stands here and each body reads `super::`, the value having one source of truth
     (status 3.264 item 2's shape; a crate-root definition of the same name is `E0428`).
     ------------------------------------------------------------------------------------------- */

  /// C: `CHAR_GLYPH` (src/dispextern.h 434, `CHAR_GLYPH`; first member of `enum glyph_type` 431-452) -
  /// the port's enumerator is `include/dispextern.rs` 240.  The Swift import is
  /// `nsterm.swift` 3578 (`func CHAR_GLYPH() -> Int32`), the lane's function-shaped spelling of
  /// a C enumerator; its use is `s->first_glyph->type == CHAR_GLYPH` (nsterm.m 3054, 3059).
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn CHAR_GLYPH () -> c_int { super::CHAR_GLYPH as c_int }

  /// C: `COMPOSITE_GLYPH,` (src/dispextern.h 437), value 1 (`include/dispextern.rs` 241); C's
  /// use is `s->first_glyph->type == COMPOSITE_GLYPH` (nsterm.m 3055).  Swift `nsterm.swift`
  /// 3579.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn COMPOSITE_GLYPH () -> c_int { super::COMPOSITE_GLYPH as c_int }

  /* -------------------------------------------------------------------------------------------
     Batch 12, group 1: the two names of this batch the mirrors already spell in Rust.  Both are
     in `nsterm`'s fifteenth slice (`nsterm.swift` 3747 and 3796), and both are A class: the
     macro `WINDOW_FRAME (w)` is a `struct window` member read, and `max_used_fringe_bitmap` is
     a data symbol the Swift seam imports as a `var` while no unit defines the storage yet
     (`src/fringe.rs` is the empty unit status 3.31 records).
     ------------------------------------------------------------------------------------------- */

  /// C: `#define WINDOW_FRAME(w) ((w)->frame)` (src/frame.h 929) - the macro's own name, which
  /// the lane imports as a function (`nsterm.swift` 3747) although round 13's `window_frame`
  /// entry already carries the bare member read: the two spellings coexist by PORTING.md 0.2's
  /// names-verbatim rule, the same way `NS_FACE_BACKGROUND` and `face_background` do.  C's read
  /// is `XFRAME (WINDOW_FRAME (w))` (src/nsterm.m 3228, 3435, 3458).  The mirror already carries
  /// the name as a function (`include/frame.rs` 939, whose note records that the *macro*
  /// spelling stays in `window.rs`), so the C-ABI spelling stands here and the body calls it.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn WINDOW_FRAME (w: *mut c_void) -> Lisp_Object
  { super::WINDOW_FRAME (w.cast::<window> ()) }

  /// C: `int max_used_fringe_bitmap = MAX_STANDARD_FRINGE_BITMAPS;` (src/fringe.c 497; C's own
  /// declaration is `extern int max_used_fringe_bitmap;`, src/dispextern.h 3710) - a *data*
  /// symbol, imported by the Swift lane as a `var` (`nsterm.swift` 3796) and read at
  /// src/nsterm.m 3261 (`p->which < max_used_fringe_bitmap`).  The mirror already declares the
  /// name (`include/dispextern.rs` 2510, in the `unsafe extern "C"` block that carries C 3617-
  /// 3710's three globals), so the C-ABI spelling stands here - and this entry is the
  /// definition the seam needs, the mirror's block being a declaration only.
  /// The initial value is C's own: `#define MAX_STANDARD_FRINGE_BITMAPS countof
  /// (standard_bitmaps)` (src/fringe.c 491) over the twenty-five-entry table at fringe.c
  /// 460-489 (counted there: `NO_FRINGE_BITMAP`'s `{ NULL, 0, 0, 0, 0, 0 }` plus twenty-four
  /// bitmaps).  C's `standard_bitmaps` is that unported unit's own table, so the count is the
  /// one thing this entry cannot reach by name; the number is C's text and the row records it.
  // The live bitmap count is exported by fringe.rs.


  /* -------------------------------------------------------------------------------------------
     Batch 13, group 1: the two frame macros and the ten enumerators this slice compares against.
     All twelve are Rust items the mirrors already spell (`pub fn FRAME_CURSOR_WIDTH` /
     `FRAME_FOREGROUND_PIXEL`, `include/frame.rs` 691/696, and the `pub const`s of
     `include/dispextern.rs` 88-93 and 1731-1734), so the C-ABI spelling stands in this module
     (status 3.264 item 2: a crate-root definition of the same name is `E0428`).
     ------------------------------------------------------------------------------------------- */

  /// C: `#define FRAME_CURSOR_WIDTH(f) ((f)->cursor_width)` (src/frame.h 1372) - C's read is the
  /// cursor comparison of `ns_draw_window_cursor` (src/nsterm.m 3294-3426); the mirror's item is
  /// `include/frame.rs` 691.  Swift `nsterm.swift` 4029 (`(UnsafeMutableRawPointer?) -> Int32`).
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn FRAME_CURSOR_WIDTH (f: *mut c_void) -> c_int
  { super::FRAME_CURSOR_WIDTH (f.cast::<frame> ()) }

  /// C: `#define FRAME_FOREGROUND_PIXEL(f) ((f)->foreground_pixel)` (src/frame.h 1375) - the
  /// mirror's item is `include/frame.rs` 696 (`c_ulong`, which is the lane's `-> UInt`).
  /// Swift `nsterm.swift` 4030.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn FRAME_FOREGROUND_PIXEL (f: *mut c_void) -> c_ulong
  { super::FRAME_FOREGROUND_PIXEL (f.cast::<frame> ()) }

  /// C: `DEFAULT_CURSOR,` (src/dispextern.h 216-225's `enum text_cursor_kinds`, the file's own
  /// count: `DEFAULT_CURSOR = -2`) - the port's enumerator is `include/dispextern.rs` 88.
  /// Swift `nsterm.swift` 4039 (`func DEFAULT_CURSOR() -> Int32`).
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn DEFAULT_CURSOR () -> c_int { super::DEFAULT_CURSOR as c_int }

  /// C: `NO_CURSOR,` (src/dispextern.h 216-225) = -1; the port's `include/dispextern.rs` 89.
  /// Swift `nsterm.swift` 4040.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn NO_CURSOR () -> c_int { super::NO_CURSOR as c_int }

  /// C: `FILLED_BOX_CURSOR,` (src/dispextern.h 216-225) = 0; `include/dispextern.rs` 90.
  /// Swift `nsterm.swift` 4041.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn FILLED_BOX_CURSOR () -> c_int { super::FILLED_BOX_CURSOR as c_int }

  /// C: `HOLLOW_BOX_CURSOR,` (src/dispextern.h 216-225) = 1; `include/dispextern.rs` 91.
  /// Swift `nsterm.swift` 4042.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn HOLLOW_BOX_CURSOR () -> c_int { super::HOLLOW_BOX_CURSOR as c_int }

  /// C: `BAR_CURSOR,` (src/dispextern.h 216-225) = 2; `include/dispextern.rs` 92.
  /// Swift `nsterm.swift` 4043.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn BAR_CURSOR () -> c_int { super::BAR_CURSOR as c_int }

  /// C: `HBAR_CURSOR,` (src/dispextern.h 216-225) = 3; `include/dispextern.rs` 93.
  /// Swift `nsterm.swift` 4044.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn HBAR_CURSOR () -> c_int { super::HBAR_CURSOR as c_int }

  /// C: `VERTICAL_BORDER_FACE_ID,` (src/dispextern.h 1973, in the face-id enum 1962-1976) - C's
  /// reads are src/nsterm.m 3441/3459-3463; the port's enumerator is `include/dispextern.rs`
  /// 1731 (value 12).  Swift `nsterm.swift` 4045.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn VERTICAL_BORDER_FACE_ID () -> c_int
  { super::VERTICAL_BORDER_FACE_ID as c_int }

  /// C: `WINDOW_DIVIDER_FACE_ID,` (src/dispextern.h 1974), value 13
  /// (`include/dispextern.rs` 1732).  Swift `nsterm.swift` 4046.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn WINDOW_DIVIDER_FACE_ID () -> c_int
  { super::WINDOW_DIVIDER_FACE_ID as c_int }

  /// C: `WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID,` (src/dispextern.h 1975), value 14
  /// (`include/dispextern.rs` 1733).  Swift `nsterm.swift` 4047.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID () -> c_int
  { super::WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID as c_int }

  /// C: `WINDOW_DIVIDER_LAST_PIXEL_FACE_ID,` (src/dispextern.h 1976), value 15
  /// (`include/dispextern.rs` 1734).  Swift `nsterm.swift` 4049.
  #[unsafe(no_mangle)]
  pub unsafe extern "C-unwind" fn WINDOW_DIVIDER_LAST_PIXEL_FACE_ID () -> c_int
  { super::WINDOW_DIVIDER_LAST_PIXEL_FACE_ID as c_int }
}

/* ---------------------------------------------------------------------------------------------
   Batch 8, the one entry of the family whose name is free at the crate root: `XSETFRAME` is a
   C *macro* (frame.h 926) and the mirror spells it as a `macro_rules!` (`include/frame.rs`
   202-204), not as a Rust item - macros live in their own namespace, so a crate-root function
   of the same name is not `E0428`, and the `c_abi_of_mirror_named_items` shape is not needed.
   (The batch-6 `set_waiting_for_input_flag` is the same situation with a differently spelled
   macro; the `mirror-named` criterion of status 3.264 item 2 is about *items*.)
   --------------------------------------------------------------------------------------------- */

/// C: `#define XSETFRAME(a, b) XSETPSEUDOVECTOR (a, b, PVEC_FRAME)` (src/frame.h 926) - the
/// frame-writing macro the Swift lanes import with the caller's slot first
/// (`UnsafeMutablePointer<Lisp_Object>`): `nsfns.swift` 711, `nsmenu.swift` 272,
/// `nsselect.swift` 181, `nsterm.swift` 1594 (paired with `FRAMEP` at 1593).  The body is the
/// mirror's own macro, so the tag-and-pointer write has one spelling in this tree: it reaches
/// `XSETPSEUDOVECTOR!` (`include/lisp.rs` 3090-3093) and `XSETVECTOR!`'s `make_lisp_ptr`
/// (3073), i.e. the same write the core's `FOR_EACH_FRAME` users make.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn XSETFRAME (a: *mut Lisp_Object, b: *mut c_void)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { XSETFRAME! (*a, b); }
}

/* ---------------------------------------------------------------------------------------------
   Batch 10, group 1: `struct frame`'s non-bitfield members the two NS Swift units write or read
   (the integrator's list, minus the bitfield entries - status 3.272/3.273, their bit numbers are
   not written out yet - minus `frame_set_fontset`, which batch 5 already landed behind its
   `have_ns` gate, and minus `set_frame_cursor_types`, which is not a member accessor at all).
   Every member here is in the frozen arm: frame.h's `struct frame` has no gate over 596-702
   (the file's only `#if` in that region is 668-672, over `wait_event_type`), and the mirror
   carries each field ungated in that run (`include/frame.rs` 185-230; the only gated member
   beside them is `wait_event_type`, cfg at 215 and the field at 216).
   --------------------------------------------------------------------------------------------- */

/// C: `int total_cols, total_lines;` (frame.h 596) - C's write is `FRAME_TOTAL_COLS (tip_f) =
/// w->total_cols;` (src/nsfns.m 3462; the macro is `#define FRAME_TOTAL_COLS(f)
/// ((f)->total_cols)`, frame.h 1068).
/// Swift `nsfns.swift` 868-869 (`(UnsafeMutableRawPointer?, Int32)`), used at `nsfns.swift` 4361.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_total_cols (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).total_cols = v; }
}

/// C: `int pixel_width, pixel_height;` (frame.h 598) - C's writes are `FRAME_PIXEL_WIDTH (f) =
/// new_native_width;` (src/frame.c 1098; the macro is frame.h 1066).
/// Swift `nsfns.swift` 871-872 (`(…, Int32)`), used at `nsfns.swift` 4407.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_pixel_width (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).pixel_width = v; }
}

/// C: `int pixel_width, pixel_height;` (frame.h 598) - C's write is `FRAME_PIXEL_HEIGHT (f) =
/// new_native_height;` (src/frame.c 1099; the macro is frame.h 1067).
/// Swift `nsfns.swift` 874-875 (`(…, Int32)`), used at `nsfns.swift` 4408.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_pixel_height (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).pixel_height = v; }
}

/// C: `int border_width;` (frame.h 613, "Border width of the frame window as known by the (X)
/// window system") - C has no `FRAME_BORDER_WIDTH` macro, and C's read is `int border = fullscreen
/// ? 0 : f->border_width;` (src/nsfns.m 3560; C's writes are src/nsterm.m 9080/9112/9854).
/// Swift `nsfns.swift` 908 (`(UnsafeMutableRawPointer?) -> Int32`), used at
/// `nsfns.swift` 4446.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_border_width (f: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).border_width }
}

/// C: `int size_hint_flags;` (frame.h 610, "The geometry flags for this window") - C's reads are
/// `if (f->size_hint_flags & XNegative)` / `& YNegative` (src/nsterm.m 1827/1834).
/// Swift `nsterm.swift` 1884-1885 (`-> Int32`), used at `nsterm.swift` 1981/1989.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_size_hint_flags (f: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).size_hint_flags }
}

/// C: `int size_hint_flags;` (frame.h 610) - C's read-modify-writes are
/// `f->size_hint_flags &= ~(XNegative|YNegative);` (src/nsterm.m 1871, 1955).
/// Swift `nsterm.swift` 1886-1887 (`(…, Int32)`), used at `nsterm.swift` 2017/2136.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_size_hint_flags (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).size_hint_flags = v; }
}

/// C: `int config_scroll_bar_width;` (frame.h 693) - C's write is
/// `FRAME_CONFIG_SCROLL_BAR_WIDTH (f) = NS_SCROLL_BAR_WIDTH_DEFAULT;` (src/nsfns.m 2530; the macro
/// is frame.h 1286).
/// Swift `nsfns.swift` 1155-1156 (`(…, Int32)`), used at `nsfns.swift` 3441.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_config_scroll_bar_width (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).config_scroll_bar_width = v; }
}

/// C: `int config_scroll_bar_cols;` (frame.h 694) - C's write is
/// `FRAME_CONFIG_SCROLL_BAR_COLS (f) = (FRAME_CONFIG_SCROLL_BAR_WIDTH (f) + wid - 1) / wid;`
/// (src/nsfns.m 2531; the macro is frame.h 1296).
/// Swift `nsfns.swift` 1159-1160 (`(…, Int32)`), used at `nsfns.swift` 3442.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_config_scroll_bar_cols (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).config_scroll_bar_cols = v; }
}

/// C: `int config_scroll_bar_height;` (frame.h 701) - C's write is
/// `FRAME_CONFIG_SCROLL_BAR_HEIGHT (f) = NS_SCROLL_BAR_WIDTH_DEFAULT;` (src/nsfns.m 2539; the
/// macro is frame.h 1291).
/// Swift `nsfns.swift` 1163-1164 (`(…, Int32)`), used at `nsfns.swift` 3449.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_config_scroll_bar_height (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).config_scroll_bar_height = v; }
}

/// C: `int config_scroll_bar_lines;` (frame.h 702) - C's write is
/// `FRAME_CONFIG_SCROLL_BAR_LINES (f) = (FRAME_CONFIG_SCROLL_BAR_HEIGHT (f) + height - 1) /
/// height;` (src/nsfns.m 2540; the macro is frame.h 1301).
/// Swift `nsfns.swift` 1167-1168 (`(…, Int32)`), used at `nsfns.swift` 3450.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_config_scroll_bar_lines (f: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*f.cast::<frame> ()).config_scroll_bar_lines = v; }
}

/* ---------------------------------------------------------------------------------------------
   Batch 10, group 2: `struct window`'s non-bitfield members, all of them written or read by
   `ns_show_tip`'s window-sizing block (src/nsfns.m 3442-3501) and by the two cursor helpers
   (src/nsfns.m 4024-4025, `phys_cursor`).  The record in window.h has no gate over the members used here
   (the file's only `#if` in 60-340 is 290-303, `HAVE_TEXT_CONVERSION`), and the mirror carries
   them ungated (`include/window.rs` 75-115; the `have_window_system` gate starts at 120, on
   `phys_cursor_type`).  The writers are plain field writes - no mirror item is reached, the
   `frame_set_*` shape of batches 1/5.
   --------------------------------------------------------------------------------------------- */

/// C: `int left_col;` (window.h 248, "The upper left corner coordinates of this window, relative
/// to upper left corner of frame = 0, 0") - C's write is `w->left_col = 0;` (src/nsfns.m 3442).
/// Swift `nsfns.swift` 917-918 (`(UnsafeMutableRawPointer?, Int32)`), used at `nsfns.swift` 4344.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_left_col (w: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).left_col = v; }
}

/// C: `int top_line;` (window.h 249) - C's write is `w->top_line = 0;` (src/nsfns.m 3443).
/// Swift `nsfns.swift` 919-920, used at `nsfns.swift` 4345.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_top_line (w: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).top_line = v; }
}

/// C: `int pixel_left;` (window.h 243) - C's write is `w->pixel_left = 0;` (src/nsfns.m 3444).
/// Swift `nsfns.swift` 921-922, used at `nsfns.swift` 4346.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_pixel_left (w: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).pixel_left = v; }
}

/// C: `int pixel_top;` (window.h 244) - C's write is `w->pixel_top = 0;` (src/nsfns.m 3445).
/// Swift `nsfns.swift` 923-924, used at `nsfns.swift` 4347.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_pixel_top (w: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).pixel_top = v; }
}

/// C: `int total_cols;` (window.h 264) - C's writes are `w->total_cols = XFIXNAT (XCAR
/// (Vx_max_tooltip_size));` (src/nsfns.m 3451) and `w->total_cols = 80;` (3456).
/// Swift `nsfns.swift` 926-927, used at `nsfns.swift` 4352.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_total_cols (w: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).total_cols = v; }
}

/// C: `int total_lines;` (window.h 265) - C's writes are `w->total_lines = XFIXNAT (XCDR
/// (Vx_max_tooltip_size));` (src/nsfns.m 3452) and `w->total_lines = 40;` (3457).
/// Swift `nsfns.swift` 929-930, used at `nsfns.swift` 4353.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_total_lines (w: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).total_lines = v; }
}

/// C: `int pixel_width;` (window.h 252) - C's write is `w->pixel_width = w->total_cols *
/// FRAME_COLUMN_WIDTH (tip_f);` (src/nsfns.m 3460).
/// Swift `nsfns.swift` 932-933, used at `nsfns.swift` 4359.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_pixel_width (w: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).pixel_width = v; }
}

/// C: `int pixel_height;` (window.h 253) - C's write is `w->pixel_height = w->total_lines *
/// FRAME_LINE_HEIGHT (tip_f);` (src/nsfns.m 3461).
/// Swift `nsfns.swift` 935-936, used at `nsfns.swift` 4360.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_pixel_height (w: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).pixel_height = v; }
}

/// C: `int total_cols;` (window.h 264) - C's reads are `w->pixel_width = w->total_cols * …`
/// (src/nsfns.m 3460) and `FRAME_TOTAL_COLS (tip_f) = w->total_cols;` (3462).
/// Swift `nsfns.swift` 925 (`(UnsafeMutableRawPointer?) -> Int32`), used at `nsfns.swift`
/// 4359/4361.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_total_cols (w: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).total_cols }
}

/// C: `int total_lines;` (window.h 265) - C's read is `w->pixel_height = w->total_lines *
/// FRAME_LINE_HEIGHT (tip_f);` (src/nsfns.m 3461).
/// Swift `nsfns.swift` 928, used at `nsfns.swift` 4360.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_total_lines (w: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).total_lines }
}

/// C: `int pixel_width;` (window.h 252) - the read counterpart of the write at src/nsfns.m 3460
/// (`w->pixel_width = w->total_cols * FRAME_COLUMN_WIDTH (tip_f);`); C reads the *height* of the
/// pair at 3501, the width's own read sites sitting outside these two units.
/// Swift `nsfns.swift` 931 (`-> Int32`); the lane has declared it but has no call site yet
/// (measured: `grep -n "window_pixel_width"` over the Swift tree finds the declaration only).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_pixel_width (w: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).pixel_width }
}

/// C: `int pixel_height;` (window.h 253) - C's read is `make_fixnum (w->pixel_height)`
/// (src/nsfns.m 3501).
/// Swift `nsfns.swift` 934, used at `nsfns.swift` 4389.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_pixel_height (w: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).pixel_height }
}

/// C: `struct cursor_pos phys_cursor;` (window.h 332) over `struct cursor_pos { int x, y; … }`
/// (window.h 91-97, `x` at 94) - C's read is `int x = text_area_x + window->phys_cursor.x;`
/// (src/nsfns.m 4024, `phys_cursor`).  The lane declares the result 64-bit (`Int` on Darwin) while the
/// member is `int`, so the declaration wins and the widening is explicit - `c_long` is Darwin's
/// 64-bit `long`, the same width as Swift's `Int` (the `ns_output_window_desc` `as c_ulong`
/// precedent of batch 6).
/// Swift `nsfns.swift` 770 (`(UnsafeMutableRawPointer?) -> Int`), used at `nsfns.swift` 4837.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_phys_cursor_x (w: *mut c_void) -> c_long
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).phys_cursor.x as c_long }
}

/// C: `struct cursor_pos phys_cursor;` (window.h 332), `y` at window.h 95 - C's read is
/// `int y = text_area_y + window->phys_cursor.y;` (src/nsfns.m 4025).
/// Swift `nsfns.swift` 771 (`-> Int`), used at `nsfns.swift` 4838.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_phys_cursor_y (w: *mut c_void) -> c_long
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).phys_cursor.y as c_long }
}

/// C: `struct glyph_matrix *desired_matrix;` (window.h 225) - C's reads are
/// `clear_glyph_matrix (w->desired_matrix);` (src/nsfns.m 3476) and
/// `row = w->desired_matrix->rows;` (3484).  The mirror degrades the pointer (`*mut c_void`,
/// `include/window.rs` 75), which is what the lane's `UnsafeMutableRawPointer?` asks for.
/// Swift `nsfns.swift` 937-938, used at `nsfns.swift` 4376.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_desired_matrix (w: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).desired_matrix }
}

/* ---------------------------------------------------------------------------------------------
   Batch 11, group 1: `struct glyph_string`'s ten members - the slice nsterm's fourteenth round
   reads and the two overhangs it writes (src/dispextern.h 1384-1480, C's uses nsterm.m
   3053-3077).  The record is **gated**: C opens `#ifdef HAVE_WINDOW_SYSTEM` at 1358 and closes
   it at 1509 (the `gc` member's three platform arms sit inside it, 1464-1475), and the mirror
   carries `pub struct glyph_string` under the same feature (`include/dispextern.rs` 1369, whose
   own note records that the record is not layout-asserted at all: this target has no
   window-system configuration to measure it with).  So these ten exports carry
   `#[cfg (feature = "have_window_system")]` - the batch-5 `have_ns` shape and status 3.255's
   ruling: an ungated export would name a record the frozen arm does not have (P-08), and the
   work is done even though no configuration this tree builds turns the arm on, so the ratchet
   rows stay as the `rust-side-gated` record (`## Evidence` measures both halves: the frozen
   object answers none of the ten, the `have_window_system` probe answers all ten).
   Only the `_set_` half of the two overhangs is declared: the lane's own note says their reads
   belong to a later slice, and an entry the Swift unit declares but never calls is dropped from
   the object (that lane's measurement, `docs/ports/nsterm.md`'s `## ABI`).
   --------------------------------------------------------------------------------------------- */

/// C: `int left_overhang;` (src/dispextern.h 1384, "Number of pixels this string overwrites in
/// front of its x-origin") - C's writes are `s->left_overhang = metrics.lbearing < 0 ?
/// - metrics.lbearing : 0;` (src/nsterm.m 3071) and `s->left_overhang = - s->cmp->lbearing;`
/// (3077).
/// Swift `nsterm.swift` 3556-3557 (`(UnsafeMutableRawPointer?, Int32)`), used at
/// `nsterm.swift` 4549/4553.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_string_set_left_overhang (s: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*s.cast::<glyph_string> ()).left_overhang = v; }
}

/// C: `int right_overhang;` (src/dispextern.h 1389) - C's writes are
/// `s->right_overhang = (metrics.rbearing > metrics.width ? metrics.rbearing - metrics.width
/// : 0);` (src/nsterm.m 3070-3071) and `s->right_overhang = s->cmp->rbearing -
/// s->cmp->pixel_width;` (3076).
/// Swift `nsterm.swift` 3558-3559, used at `nsterm.swift` 4548/4552.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_string_set_right_overhang (s: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*s.cast::<glyph_string> ()).right_overhang = v; }
}

/// C: `unsigned *char2b;` (src/dispextern.h 1407) - C's read is the `text_extents` call's
/// argument `s->char2b` (src/nsterm.m 3062).  The member is `unsigned *` while the lane
/// declares `UnsafePointer<UInt32>?` (a *const* view), so the declaration wins and the body
/// casts the mirror's `*mut c_uint` (`include/dispextern.rs` 1407) to `*const c_uint`.
/// Swift `nsterm.swift` 3560-3561.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_string_char2b (s: *mut c_void) -> *const c_uint
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*s.cast::<glyph_string> ()).char2b }
}

/// C: `int nchars;` (src/dispextern.h 1408, "Characters to be drawn, and number of characters.
/// Note that NCHARS can be zero if this is a composition glyph string") - C's read is
/// `text_extents (font, s->char2b, s->nchars, &metrics)` (src/nsterm.m 3062).
/// Swift `nsterm.swift` 3562.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_string_nchars (s: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*s.cast::<glyph_string> ()).nchars }
}

/// C: `struct font *font;` (src/dispextern.h 1417) - C's read is `struct font *font = s->font;`
/// (src/nsterm.m 3062).  The mirror types the member (`*mut font`, `include/dispextern.rs`
/// 1417); the lane declares `UnsafeMutableRawPointer?`, so the body degrades it with a cast -
/// the reverse of the batch-6 `ns_output_display_info` decision, the declaration winning.
/// Swift `nsterm.swift` 3563-3564.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_string_font (s: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*s.cast::<glyph_string> ()).font as *mut c_void }
}

/// C: `struct composition *cmp;` (src/dispextern.h 1421, "Non-null means this string describes
/// (part of) a static composition") - C's reads are `if (s->cmp == NULL …)` (src/nsterm.m 3053)
/// and `else if (s->cmp)` (3076).
/// Swift `nsterm.swift` 3565-3566.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_string_cmp (s: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*s.cast::<glyph_string> ()).cmp as *mut c_void }
}

/// C: `ptrdiff_t cmp_id;` (src/dispextern.h 1424) - C's read is
/// `composition_gstring_from_id (s->cmp_id)` (src/nsterm.m 3068), a call
/// `src/composite.rs` already provides.  The member is `ptrdiff_t`
/// (`include/lisp.rs` 50, `isize`), which is the width the lane's `-> Int` asks for.
/// Swift `nsterm.swift` 3567-3568.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_string_cmp_id (s: *mut c_void) -> ptrdiff_t
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*s.cast::<glyph_string> ()).cmp_id }
}

/// C: `int cmp_from, cmp_to;` (src/dispextern.h 1427, "Start and end glyph indices in a
/// glyph-string") - C's read is `composition_gstring_width (gstring, s->cmp_from, s->cmp_to,
/// &metrics)` (src/nsterm.m 3069).
/// Swift `nsterm.swift` 3569-3570.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_string_cmp_from (s: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*s.cast::<glyph_string> ()).cmp_from }
}

/// C: `int cmp_from, cmp_to;` (src/dispextern.h 1428) - the same call, `src/nsterm.m` 3069.
/// Swift `nsterm.swift` 3571.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_string_cmp_to (s: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*s.cast::<glyph_string> ()).cmp_to }
}

/// C: `struct glyph *first_glyph;` (src/dispextern.h 1480) - C's reads are
/// `s->first_glyph->type == CHAR_GLYPH` (src/nsterm.m 3054, 3059) and
/// `== COMPOSITE_GLYPH` (3055).  The mirror types it (`*mut glyph`,
/// `include/dispextern.rs` 1477); the lane declares `UnsafeMutableRawPointer?`.
/// Swift `nsterm.swift` 3572-3573.
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_string_first_glyph (s: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*s.cast::<glyph_string> ()).first_glyph as *mut c_void }
}

/* ---------------------------------------------------------------------------------------------
   Batch 11, group 3: `struct glyph`'s `type` bitfield and the three `struct glyph_row` flags
   this slice reads and writes.  Both records are ungated in C (dispextern.h 472-... and
   918-...) and in the mirror (`include/dispextern.rs` 295 and 771-773; the gated member beside
   `glyph_row`'s run is `clip`, cfg at 823).  Rust has no bitfields, so both mirrors carry the
   allocation units and name every member (`glyph_bits!`, `include/dispextern.rs` 511;
   `glyph_row_bits!`, 886): each body goes through the mirror's own accessor, so the bit
   numbering has one source of truth (the batch-7 `FRAME_NO_ACCEPT_FOCUS_SHIFT` shape).
   --------------------------------------------------------------------------------------------- */

/// C: `unsigned type : 3;` (src/dispextern.h 514, "Which kind of glyph this is - character,
/// image etc.  Value should be an enumerator of type enum glyph_type") - C's reads are
/// `s->first_glyph->type == CHAR_GLYPH` (src/nsterm.m 3054, 3059) and `== COMPOSITE_GLYPH`
/// (3055).  The body is the mirror's own bit accessor (`glyph_bits! (type, …)`,
/// `include/dispextern.rs` 511-514, over `GLYPH_TYPE_SHIFT` 0 and width 3 at 421-422).
/// Swift `nsterm.swift` 3577 (`(UnsafeMutableRawPointer?) -> Int32`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_type (g: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { glyph_bits! (type, *g.cast::<glyph> ()) }
}

/// C: `mode_line_p` (src/dispextern.h 1086, `mode_line_p`; one of `glyph_row`'s 29 `bool_bf : 1`
/// members) - C's read is `!desired_row->mode_line_p` (src/nsterm.m 2977).  The body is the
/// mirror's own accessor (`glyph_row_bits! (mode_line_p, …)`, `include/dispextern.rs` 947-948,
/// i.e. `bools [1]` bit 4).
/// Swift `nsterm.swift` 3539-3540 (`(UnsafeMutableRawPointer?) -> Bool`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_row_mode_line_p (row: *mut c_void) -> bool
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { glyph_row_bits! (mode_line_p, row.cast::<glyph_row> ()) }
}

/// C: `bool_bf full_width_p : 1;` (src/dispextern.h 1083) - C's read is
/// `desired_row->full_width_p` (src/nsterm.m 2983).  Body: `glyph_row_bits! (full_width_p, …)`
/// (`include/dispextern.rs` 945-946, `bools [1]` bit 3).
/// Swift `nsterm.swift` 3541-3542.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_row_full_width_p (row: *mut c_void) -> bool
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { glyph_row_bits! (full_width_p, row.cast::<glyph_row> ()) }
}

/// C: `bool_bf redraw_fringe_bitmaps_p : 1;` (src/dispextern.h 1043) - C's **write** is
/// `desired_row->redraw_fringe_bitmaps_p = 1;` (src/nsterm.m 2978).  Body:
/// `glyph_row_bits! (redraw_fringe_bitmaps_p, …, v)` (`include/dispextern.rs` 922-924,
/// `bools [0]` bit 1) - the mirror's setter evaluates the value once, C's own shape.
/// Swift `nsterm.swift` 3543-3544 (`(UnsafeMutableRawPointer?, Bool)`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_row_set_redraw_fringe_bitmaps_p (row: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { glyph_row_bits! (redraw_fringe_bitmaps_p, row.cast::<glyph_row> (), v) }
}

/// C: `bool_bf pseudo_window_p : 1;` (src/window.h 438, "Flag indicating that this window is
/// not a real one") - C's read is `!w->pseudo_window_p` (src/nsterm.m 2977); the member is bit
/// 11 of `struct window`'s nineteen-flag unit at offset 500 (`docs/baseline/window-bits.tsv`
/// line 18, the same unit and bit in all three arms).  The body is the mirror's own packed-flag
/// reader plus that bit (`window_bit`, `include/window.rs` 392, over the mirror's
/// `WINDOW_PSEUDO_WINDOW_P_SHIFT` 165) - the batch-7 `FRAME_NO_ACCEPT_FOCUS_SHIFT` shape.  The
/// writer-side macro `window_bits!` is *not* reachable from this unit: it lives in the unit
/// `src/window.rs` 88-93, not in the mirror, which is why the body names the reader and the
/// shift directly (measured: the macro is `cannot find macro` here, `## Evidence`).
/// Swift `nsterm.swift` 3532-3533 (`(UnsafeMutableRawPointer?) -> Bool`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_pseudo_window_p (w: *mut c_void) -> bool
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { window_bit (w.cast::<window> (), WINDOW_PSEUDO_WINDOW_P_SHIFT) }
}

/* ---------------------------------------------------------------------------------------------
   Batch 11, group 4: `struct composition`'s three `short` metrics (composite.h 148-155) and the
   two hops of C's `font->driver->text_extents (…)` call (font.h 400 and 672-674).  All five
   members are ungated in C (the `#ifdef HAVE_WINDOW_SYSTEM` at font.h 676 comes *after*
   `text_extents`, and `driver` follows the one that closes at 397) and in the mirror
   (`include/composite.rs` 50, `include/font.rs` 134 and 511).
   --------------------------------------------------------------------------------------------- */

/// C: `short pixel_width, ascent, descent;` (src/composite.h 153) - C's read is
/// `s->cmp->rbearing - s->cmp->pixel_width` (src/nsterm.m 3076).  The member is a `short` while
/// the lane declares `-> Int32`, so the declaration wins and the body widens with `as c_int`
/// (the batch-6 `ns_output_window_desc` rule).
/// Swift `nsterm.swift` 3582-3583 (`(UnsafeMutableRawPointer?) -> Int32`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn composition_pixel_width (cmp: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*cmp.cast::<composition> ()).pixel_width as c_int }
}

/// C: `short lbearing, rbearing;` (src/composite.h 155) - C's read is
/// `s->left_overhang = - s->cmp->lbearing;` (src/nsterm.m 3077, and the `metrics.lbearing < 0`
/// test at 3071 is the other branch).
/// Swift `nsterm.swift` 3584-3585.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn composition_lbearing (cmp: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*cmp.cast::<composition> ()).lbearing as c_int }
}

/// C: `short lbearing, rbearing;` (src/composite.h 155) - C's read is
/// `s->right_overhang = s->cmp->rbearing - s->cmp->pixel_width;` (src/nsterm.m 3076).
/// Swift `nsterm.swift` 3586-3587.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn composition_rbearing (cmp: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*cmp.cast::<composition> ()).rbearing as c_int }
}

/// C: `struct font_driver const *driver;` (src/font.h 400, the first member after the
/// `#endif /* HAVE_WINDOW_SYSTEM */` at 397) - C's read is the first hop of
/// `font->driver->text_extents (font, s->char2b, s->nchars, &metrics);` (src/nsterm.m 3062; the
/// local `struct font *font = s->font;` is `glyph_string_font` above).  The mirror keeps the
/// pointee untyped (`*const c_void`, `include/font.rs` 134, P-55/P-72 rule 3: `struct
/// font_driver` had not landed in this mirror when that field was written), so the body casts
/// to the lane's `UnsafeMutableRawPointer?`.
/// Swift `nsterm.swift` 3596 (`(UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer?`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn font_driver (font: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*font.cast::<font> ()).driver as *mut c_void }
}

/// C: `void (*text_extents) (struct font *font, const unsigned *code, int nglyphs, struct
/// font_metrics *metrics);` (src/font.h 672-674) - the second hop of src/nsterm.m 3062's call,
/// at offset 104 of `struct font_driver` (measured: `include/font.rs` 564's
/// `offset_of! (font_driver, text_extents)`, the 208-byte layout the probe recorded).  The
/// return type is the mirror's own slot type (`include/font.rs` 511-512,
/// `Option<unsafe extern "C" fn (*mut font, *const c_uint, c_int, *mut font_metrics)>`), which
/// is ABI-identical to the lane's `font_text_extents_fn` (`nsterm.swift` 3595) - four pointers
/// and an `Int32` in, `void` out; naming the mirror's type keeps one spelling of the slot.
/// Swift `nsterm.swift` 3597-3598 (`(UnsafeMutableRawPointer?) -> font_text_extents_fn?`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn font_driver_text_extents (driver: *mut c_void)
  -> Option<unsafe extern "C-unwind" fn (*mut font, *const c_uint, c_int, *mut font_metrics)>
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*driver.cast::<font_driver> ()).text_extents }
}

/* ---------------------------------------------------------------------------------------------
   Batch 12, group 2: the fifteen members of `struct face` and `struct draw_fringe_bitmap_params`
   this slice reads (src/nsterm.m 3229-3281), and the one fringe name the frozen configuration
   owes.  The two records are ungated in C (dispextern.h 1787-1937 and 2223-2233) but the mirror
   carries them behind `#[cfg (feature = "have_ns")]` - status 3.220's arm for
   the records the ported units keep typed copies of (`include/dispextern.rs` 1983 on
   `struct face`, 2784 on `struct draw_fringe_bitmap_params`, whose note also records that its
   layout was measured with an `offsetof` probe) - so these fifteen carry that gate, the batch-6
   `face_set_background` shape: the work is done, and only a `dispextern_face_records` build can
   clear those rows.  The two records sit at different places in the mirror and the bodies say
   so: `struct face` is a crate-root item (1985), while the five records of C 2159-3200 live
   inside the mirror's `pub mod dispextern_unit_names` (2542-3684, declared because every unit
   that includes the mirror declares its own copy of each), so the thirteen
   `draw_fringe_bitmap_params` bodies name the type by that path - measured: the unqualified
   name is `E0425` in the flip arm (`## Evidence`).
   --------------------------------------------------------------------------------------------- */

/// C: `unsigned long foreground;` (src/dispextern.h 1816, "Pixel value of foreground color for X
/// frames.  Color index for tty frames") - C's reads are the bare member (`face->foreground`,
/// src/nsterm.m 3279, 3445, 3464), which is why this entry and the macro entry
/// `NS_FACE_FOREGROUND` coexist: each spelling gets its own name (the lane's note, and
/// PORTING.md 0.2).  The mirror types the member (`c_ulong`, `include/dispextern.rs` 2004).
/// Swift `nsterm.swift` 3751 (`(UnsafeMutableRawPointer?) -> UInt`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn face_foreground (face: *mut c_void) -> c_ulong
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*face.cast::<face> ()).foreground }
}

/// C: `unsigned long background;` (src/dispextern.h 1819) - C's reads are the bare member
/// (`face->background`; the bare member at src/nsterm.m 3254 (`background`) and 3281), beside the
/// macro `NS_FACE_BACKGROUND` (nsterm.m 2811), whose write is nsfns.m 328-329 (batch 6's `face_set_background`); the mirror's field is
/// `include/dispextern.rs` 2005.
/// Swift `nsterm.swift` 3752 (`-> UInt`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn face_background (face: *mut c_void) -> c_ulong
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*face.cast::<face> ()).background }
}

/// C: `struct face *face;` (src/dispextern.h 2232, the last member of `struct
/// draw_fringe_bitmap_params` 2223-2233) - C's read is `struct face *face = p->face;`
/// (src/nsterm.m 3229).  The mirror types it (`*mut face`, `include/dispextern.rs` 2802); the
/// lane declares `UnsafeMutableRawPointer?`, so the body degrades it with a cast.
/// Swift `nsterm.swift` 3757-3758.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_face (p: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).face as *mut c_void }
}

/// C: `int which;  /* enum fringe_bitmap_type */` (src/dispextern.h 2225) - C's reads are
/// `p->which` (src/nsterm.m 3235, 3258, 3261, 3263).
/// Swift `nsterm.swift` 3759-3760 (`(UnsafeMutableRawPointer?) -> Int32`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_which (p: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).which }
}

/// C: `int wd, h, dh;` (src/dispextern.h 2227) - C's reads are `p->wd, p->h, p->dh`
/// (src/nsterm.m 3235) and `p->wd, p->h` (3238).
/// Swift `nsterm.swift` 3761-3762.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_wd (p: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).wd }
}

/// C: `int wd, h, dh;` (src/dispextern.h 2227), read at src/nsterm.m 3235/3238.
/// Swift `nsterm.swift` 3763-3764.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_h (p: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).h }
}

/// C: `int wd, h, dh;` (src/dispextern.h 2227), read at src/nsterm.m 3235.
/// Swift `nsterm.swift` 3765-3766.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_dh (p: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).dh }
}

/// C: `int x, y;` (src/dispextern.h 2228) - C's read is `NSMakeRect (p->x, p->y, p->wd, p->h)`
/// (src/nsterm.m 3238).
/// Swift `nsterm.swift` 3767-3768.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_x (p: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).x }
}

/// C: `int x, y;` (src/dispextern.h 2228), read at src/nsterm.m 3238.
/// Swift `nsterm.swift` 3769-3770.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_y (p: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).y }
}

/// C: `int bx, nx, by, ny;` (src/dispextern.h 2229) - C's reads are `p->bx`, `p->by`, `p->nx`,
/// `p->ny` (src/nsterm.m 3240-3241).
/// Swift `nsterm.swift` 3771-3772.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_bx (p: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).bx }
}

/// C: `int bx, nx, by, ny;` (src/dispextern.h 2229), read at src/nsterm.m 3240-3241.
/// Swift `nsterm.swift` 3773-3774.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_by (p: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).by }
}

/// C: `int bx, nx, by, ny;` (src/dispextern.h 2229), read at src/nsterm.m 3240-3241.
/// Swift `nsterm.swift` 3775-3776.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_nx (p: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).nx }
}

/// C: `int bx, nx, by, ny;` (src/dispextern.h 2229), read at src/nsterm.m 3240-3241.
/// Swift `nsterm.swift` 3777-3778.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_ny (p: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).ny }
}

/// C: `bool_bf cursor_p : 1;` (src/dispextern.h 2230) - C's reads are `p->cursor_p`
/// (src/nsterm.m 3235, 3278).  Rust has no bitfields: the mirror stores C's two-bit run as
/// `flags` (`include/dispextern.rs` 2802, a `u8` at offset 52) and names no accessor for it, so
/// the bit is this batch's own measurement - `spikes/gated-layout-probe.py 'struct
/// draw_fringe_bitmap_params' dispextern.h c --bits` answers `52:0-0 bool_bf cursor_p` and
/// `52:1-1 bool_bf overlay_p` in the frozen arm (`## Evidence`).
/// Swift `nsterm.swift` 3779-3780 (`(UnsafeMutableRawPointer?) -> Bool`).
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_cursor_p (p: *mut c_void) -> bool
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).flags & (1 << 0) != 0 }
}

/// C: `bool_bf overlay_p : 1;` (src/dispextern.h 2231) - C's reads are `p->overlay_p`
/// (src/nsterm.m 3235, 3250, 3280); the measured bit is byte 52 bit 1 (the entry above).
/// Swift `nsterm.swift` 3781-3782.
#[cfg (feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn draw_fringe_bitmap_params_overlay_p (p: *mut c_void) -> bool
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*p.cast::<draw_fringe_bitmap_params> ()).flags & (1 << 1) != 0 }
}

/* ---------------------------------------------------------------------------------------------
   Batch 12, group 3: the one name of the batch whose owner the frozen configuration does not
   compile at all - a gated debt, not a missing name (status 3.255 item 3's class, the shape the
   batch-5 `have_ns` exports and batch 11's ten took).
   --------------------------------------------------------------------------------------------- */

/* ---------------------------------------------------------------------------------------------
   **Moved back to the unit that owns the name (P-88 (5), status 3.669).**  This shim used to
   define two fringe exports whose unit did not exist yet: `gui_define_fringe_bitmap` (C's
   `void gui_define_fringe_bitmap (struct frame *f, int n)`, src/fringe.c 1852-1863) and
   `draw_fringe_bitmap` (C's `void draw_fringe_bitmap (struct window *w, struct glyph_row
   *row, int left_p)`, src/fringe.c 838's function).  `src/fringe.rs` now carries
   both - the first landed with slice 2 (C 1365-1880), the second with slice 1 - each
   `#[unsafe(no_mangle)] pub unsafe extern "C-unwind" fn` under the same `have_window_system` gate,
   so a second copy here is a duplicate definition in the configuration that compiles both
   (`E0428`, and two `_gui_define_fringe_bitmap` symbols at link time).  The Swift seam's two
   imports (`nsterm.swift` 3794 and 4115) are satisfied by the unit's own exports; the rows in
   `docs/ports/abi_shim.md` and `docs/ports/fringe.md` (Owed 4) record the move.
   --------------------------------------------------------------------------------------------- */

/* ---------------------------------------------------------------------------------------------
   Batch 13, group 2: `struct window`'s cursor members, `struct glyph_row`'s and `struct glyph`'s
   members this slice reads - and, of those, **which ones are gated**: window.h's
   `#ifdef HAVE_WINDOW_SYSTEM` block is 341-352 and holds exactly `phys_cursor_type` (344) and
   `phys_cursor_width` (347), so three of the five names the handoff grouped together carry
   `have_window_system`; the other two do **not**, and the C lines say why: `phys_cursor_on_p` is
   a `bool_bf … : 1` of the window's flag word (window.h 422, outside the block), and
   `phys_cursor` itself is `struct cursor_pos` at window.h 332, *before* the `#ifdef` opens.  The
   mirror agrees on both (`include/window.rs` 115 carries `phys_cursor` ungated; the flag's bit is
   the same unit and shift in all four arms - `docs/baseline/window-bits.tsv` line 14 - and
   `WINDOW_PHYS_CURSOR_ON_P_SHIFT` is `include/window.rs` 161), so those two are exported
   ungated - gating them would hide names C has in the frozen configuration, the error mode this
   file's own gate audit warns about (`## Gate audit`).
   --------------------------------------------------------------------------------------------- */

/// C: `#define WINDOW_XFRAME(W) XFRAME (WINDOW_FRAME (W))` (src/window.h 620) - the macro C 3305
/// and 3435/3458 read; the mirror keeps it as `macro_rules! WINDOW_XFRAME`
/// (`include/window.rs` 420, over `XFRAME` 346 and `WINDOW_FRAME!` 419).  The lane declares the
/// result untyped (`UnsafeMutableRawPointer?`), so the body casts.
/// Swift `nsterm.swift` 3993.
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn WINDOW_XFRAME (w: *mut c_void) -> *mut c_void
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { WINDOW_XFRAME! (w.cast::<window> ()) as *mut c_void }
}

/// C: `enum text_cursor_kinds phys_cursor_type;` (src/window.h 344, inside the
/// `#ifdef HAVE_WINDOW_SYSTEM` block 341-352) - C's write is `w->phys_cursor_type =
/// HOLLOW_BOX_CURSOR;` and friends in `ns_draw_window_cursor` (src/nsterm.m 3319).  The mirror's
/// field carries the same feature (`include/window.rs` 121).
/// Swift `nsterm.swift` 4001 (`(UnsafeMutableRawPointer?, Int32)`).
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_phys_cursor_type (w: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).phys_cursor_type = v; }
}

/// C: `bool_bf phys_cursor_on_p : 1;` (src/window.h 422, **outside** the window-system block) -
/// C's write is `w->phys_cursor_on_p = on_p;` (src/nsterm.m 3319, and the same member is saved
/// and restored around `draw_glyphs`).  The mirror's flag word is ungated and its bit is
/// `WINDOW_PHYS_CURSOR_ON_P_SHIFT` (`include/window.rs` 161), the same unit and shift in every
/// arm; the body writes the bit with the mirror's shift, the batch-7 `frame_set_no_accept_focus`
/// shape (the `window_bits!` setter lives in the unit `src/window.rs`, which this file cannot
/// reach).
/// Swift `nsterm.swift` 4003 (`(UnsafeMutableRawPointer?, Bool)`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_phys_cursor_on_p (w: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe {
    let bits = &mut (*w.cast::<window> ()).bits;
    *bits = (*bits & ! (1u32 << WINDOW_PHYS_CURSOR_ON_P_SHIFT))
            | ((v as u32) << WINDOW_PHYS_CURSOR_ON_P_SHIFT);
  }
}

/// C: `int phys_cursor_width;` (src/window.h 347, inside the window-system block 341-352) - C's
/// reads are src/nsterm.m 3351/3358/3373 and its writes 3324/3352.  The mirror's field carries
/// the gate (`include/window.rs` 123).
/// Swift `nsterm.swift` 4005 (`(UnsafeMutableRawPointer?) -> Int32`).
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_phys_cursor_width (w: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).phys_cursor_width }
}

/// C: `int phys_cursor_width;` (src/window.h 347) - C's writes are src/nsterm.m 3324 (`w->
/// phys_cursor_width = 0;`) and 3352.  Same gate as the reader above.
/// Swift `nsterm.swift` 4007 (`(…, Int32)`).
#[cfg (feature = "have_window_system")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_set_phys_cursor_width (w: *mut c_void, v: c_int)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).phys_cursor_width = v; }
}

/// C: `struct cursor_pos phys_cursor;` (src/window.h 332) over `struct cursor_pos { int x, y;
/// … int hpos, vpos; }` (window.h 91-99, `hpos` at 97) - C's read is
/// `int hpos = w->phys_cursor.hpos;` (src/nsterm.m 3333), the nested read the lane names after
/// the `window_phys_cursor_x`/`_y` entries `nsfns.swift` already imports.  `phys_cursor` sits
/// **before** window.h's `#ifdef` (341), so the member is ungated in every arm and the mirror
/// carries it ungated (`include/window.rs` 115).
/// Swift `nsterm.swift` 4009 (`(UnsafeMutableRawPointer?) -> Int32`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_phys_cursor_hpos (w: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*w.cast::<window> ()).phys_cursor.hpos }
}

/* ---------------------------------------------------------------------------------------------
   Batch 13, group 3: `struct glyph_row`'s and `struct glyph`'s members.  Both records are
   ungated (dispextern.h 918-… and 472-…; the mirror's `include/dispextern.rs` 771/295), and the
   two bitfields go through the mirror's own accessors (`glyph_row_bits!` 886, `glyph_bits!`
   511), so no gate is needed - the batch-11 shape.
   --------------------------------------------------------------------------------------------- */

/// C: `bool_bf exact_window_width_line_p : 1;` (src/dispextern.h 1115) - C's read is
/// src/nsterm.m 3332.  Body: `glyph_row_bits! (exact_window_width_line_p, …)`
/// (`include/dispextern.rs` 1010).
/// Swift `nsterm.swift` 4015 (`(UnsafeMutableRawPointer?) -> Bool`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_row_exact_window_width_line_p (row: *mut c_void) -> bool
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { glyph_row_bits! (exact_window_width_line_p, row.cast::<glyph_row> ()) }
}

/// C: `short used[1 + LAST_AREA];` (src/dispextern.h 938) - C's read is
/// `row->used[TEXT_AREA]` (src/nsterm.m 3333), whose subscript is the entry's second argument
/// (the `frame_alpha` index shape).  The mirror's field is `used: [c_short; 4]`
/// (`include/dispextern.rs` 822), sized `1 + LAST_AREA` exactly as C's declaration is.
/// Swift `nsterm.swift` 4017 (`(UnsafeMutableRawPointer?, Int32) -> Int16`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_row_used (row: *mut c_void, area: c_int) -> c_short
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*row.cast::<glyph_row> ()).used [area as usize] }
}

/// C: `bool_bf cursor_in_fringe_p : 1;` (src/dispextern.h 1118) - C's write is
/// `row->cursor_in_fringe_p = 0;` (src/nsterm.m 3335).  Body:
/// `glyph_row_bits! (cursor_in_fringe_p, …, v)` (`include/dispextern.rs` 1013).
/// Swift `nsterm.swift` 4019 (`(…, Bool)`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_row_set_cursor_in_fringe_p (row: *mut c_void, v: bool)
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { glyph_row_bits! (cursor_in_fringe_p, row.cast::<glyph_row> (), v) }
}

/// C: `int ascent, height;` (src/dispextern.h 959) - C's reads are src/nsterm.m 3364-3365.
/// Swift `nsterm.swift` 4021 (`-> Int32`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_row_height (row: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*row.cast::<glyph_row> ()).height }
}

/// C: `unsigned resolved_level : 7;` (src/dispextern.h 553, `resolved_level`; one of `struct glyph`'s bitfields) -
/// C's read is src/nsterm.m 3357.  Body: `glyph_bits! (resolved_level, …)`
/// (`include/dispextern.rs` 554-555, over `GLYPH_RESOLVED_LEVEL_SHIFT` 10 / width 7 at 473-474).
/// Swift `nsterm.swift` 4024 (`(UnsafeMutableRawPointer?) -> Int32`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_resolved_level (g: *mut c_void) -> c_int
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { glyph_bits! (resolved_level, *g.cast::<glyph> ()) }
}

/// C: `short pixel_width;` (src/dispextern.h 503) - C's read is src/nsterm.m 3358.  The member
/// is a `short` and the lane declares `-> Int16`, so the mirror's `c_short` is returned as is.
/// Swift `nsterm.swift` 4025 (`(UnsafeMutableRawPointer?) -> Int16`).
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn glyph_pixel_width (g: *mut c_void) -> c_short
{
  /* SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock. */ unsafe { (*g.cast::<glyph> ()).pixel_width }
}

/* ---------------------------------------------------------------------------------------------
   Batch 13, group 4: the three cursor/fringe functions no built configuration has a symbol for
   (all measured).  Two are `#ifdef HAVE_WINDOW_SYSTEM` blocks of src/xdisp.c and carry that
   gate; the third belongs to `fringe.c`, the file `src/fringe.rs` records as compiled by
   no configuration (status 3.31).  Their rows are the "gated debt" class (3.255 item 3), and the
   bodies say what is carried and what is not.
   --------------------------------------------------------------------------------------------- */

/* C: `void window_text_bottom_y (…)` is a **ported** function (defined `src/xdisp.rs`
   2852, `T _window_text_bottom_y` among the built objects), but its declaration lives in the unit
   `src/window.rs` 952 - not in any mirror - so this file declares the symbol it calls, the
   `xzalloc` shape above; the declaration carries `have_window_system` because only the body
   below uses it.  (Plain comments, not `///`: rustdoc warns that it generates no documentation
   for an `extern` block - measured in the flip arm.)  */
#[cfg (feature = "have_window_system")]
unsafe extern "C-unwind" { fn window_text_bottom_y (w: *mut window) -> c_int; }

// Cursor geometry is exported by the actual xdisp translation unit.


// draw_phys_cursor_glyph is owned by the complete xdisp implementation.

// Cocoa-only header reification. The build driver supplies a target-local generated
// module whose wrappers call the existing mirrored implementation, never a stub.
#[cfg(feature = "have_ns")]
include!(env!("FEMACS_GUI_EXPORTS"));
#[cfg(feature = "have_ns")]
use crate::fontset::{face, frame_face_accessors};
#[cfg(feature = "have_ns")]
use crate::fringe::{draw_fringe_bitmap_params, redisplay_interface};
#[cfg(feature = "have_ns")]
use crate::xdisp::{bidi_it, it};

#[cfg(feature = "have_ns")]
const _: () = ();
#[cfg(feature = "have_ns")]
unsafe extern "C-unwind" {
  fn estimate_mode_line_height(f: *mut frame, face: c_int) -> c_int;
  fn window_box_left(w: *mut window, area: c_int) -> c_int;
  fn font_style_to_value(prop: c_int, val: Lisp_Object, noerror: bool) -> c_int;
}

// Measured against the same out-of-tree Cocoa config and fixed reference headers.
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::size_of::<frame>() == 608);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::size_of::<font>() == 216);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::size_of::<face>() == 296);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::size_of::<font_driver>() == 232);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::size_of::<input_event>() == 64);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::size_of::<ns_display_info>() == 280);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::size_of::<ns_output>() == 280);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::size_of::<Emacs_GC>() == 16);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::size_of::<NSRect>() == 32);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::offset_of!(font, driver) == 208);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::offset_of!(face, gc) == 168);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::offset_of!(face, foreground) == 184);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::offset_of!(ns_display_info, root_window) == 92);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::offset_of!(ns_display_info, mouse_highlight) == 120);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::offset_of!(input_event, code) == 4);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::offset_of!(input_event, x) == 16);
#[cfg(feature = "have_ns")]
const _: () = assert!(core::mem::offset_of!(input_event, device) == 56);

#[cfg(feature = "have_ns")]
include!(env!("FEMACS_GUI_RUNTIME"));

// Cocoa views use the measured shared records, never copied layouts.
#[cfg(feature = "have_ns")]
const _: () = core::assert!(core::mem::size_of::<glyph_string>() == 272);
#[cfg(feature = "have_ns")]
const _: () = core::assert!(core::mem::offset_of!(glyph_string, clip) == 176);
#[cfg(feature = "have_ns")]
const _: () = core::assert!(core::mem::offset_of!(glyph_string, num_clips) == 240);
#[cfg(feature = "have_ns")]
const _: () = core::assert!(core::mem::offset_of!(glyph_string, next) == 256);
#[cfg(feature = "have_ns")]
mod cocoa_record_views {
use super::*;
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_set_ns_focus_frame_from_display(p: *mut ns_display_info, f: *mut frame) {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p).ns_focus_frame = f; 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_set_highlight_frame_from_display(p: *mut ns_display_info, f: *mut frame) {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p).highlight_frame = f; 
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_text_cols(f: *mut c_void) -> c_int {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*f.cast::<frame>()).text_cols 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_text_lines(f: *mut c_void) -> c_int {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*f.cast::<frame>()).text_lines 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_left_pos(f: *mut c_void, v: c_int) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*f.cast::<frame>()).left_pos = v; 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_top_pos(f: *mut c_void, v: c_int) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*f.cast::<frame>()).top_pos = v; 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_garbaged_p(f: *mut c_void) -> bool { frame_flag(f.cast::<frame>(), FRAME_GARBAGED_UNIT, FRAME_GARBAGED_SHIFT) }
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_mouse_moved(f: *mut c_void) -> bool { frame_flag(f.cast::<frame>(), FRAME_MOUSE_MOVED_UNIT, FRAME_MOUSE_MOVED_SHIFT) }
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_no_accept_focus(f: *mut c_void) -> bool { frame_flag(f.cast::<frame>(), FRAME_NO_ACCEPT_FOCUS_UNIT, FRAME_NO_ACCEPT_FOCUS_SHIFT) }
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_mouse_moved(f: *mut c_void, v: bool) -> () { frame_set_flag(f.cast::<frame>(), FRAME_MOUSE_MOVED_UNIT, FRAME_MOUSE_MOVED_SHIFT, v as u32); }
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_want_fullscreen(f: *mut c_void) -> c_int { frame_field(f.cast::<frame>(), FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT, FRAME_WANT_FULLSCREEN_MASK) as c_int }
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_set_want_fullscreen(f: *mut c_void, v: c_int) -> () { frame_set_field(f.cast::<frame>(), FRAME_WANT_FULLSCREEN_UNIT, FRAME_WANT_FULLSCREEN_SHIFT, FRAME_WANT_FULLSCREEN_MASK, v as u32); }
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn frame_output_data_ns(f: *mut c_void) -> *mut c_void {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*f.cast::<frame>()).output_data.ns 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn buffer_enable_multibyte_characters(p: *mut c_void) -> Lisp_Object {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<buffer>()).enable_multibyte_characters_ 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn buffer_mark(p: *mut c_void) -> Lisp_Object {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<buffer>()).mark_ 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn buffer_mark_active(p: *mut c_void) -> Lisp_Object {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<buffer>()).mark_active_ 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn current_buffer() -> *mut c_void {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { current_buffer!().cast() 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn waiting_for_input() -> bool {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { waiting_for_input!() 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn terminal_type(p: *mut c_void) -> c_int {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<terminal>()).r#type 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn window_frame(p: *mut c_void) -> Lisp_Object {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<window>()).frame 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn image_pixmap(p: *mut c_void) -> *mut c_void {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<image>()).pixmap 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn image_load_failed_p(p: *mut c_void) -> bool {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<image>()).load_failed_p 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn input_event_set_part(p: *mut input_event, v: c_int) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p).bits = ((*p).bits & INPUT_EVENT_KIND_MASK) | (((v as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mouse_highlight_mouse_face_window(p: *mut c_void) -> Lisp_Object {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<Mouse_HLInfo>()).mouse_face_window 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mouse_highlight_mouse_face_mouse_frame(p: *mut c_void) -> *mut c_void {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<Mouse_HLInfo>()).mouse_face_mouse_frame 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mouse_highlight_mouse_face_hidden(p: *mut c_void) -> bool {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<Mouse_HLInfo>()).bits & MOUSE_FACE_HIDDEN != 0 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mouse_highlight_set_mouse_face_mouse_frame(p: *mut c_void, v: *mut c_void) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<Mouse_HLInfo>()).mouse_face_mouse_frame = v; 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mouse_highlight_set_mouse_face_hidden(p: *mut c_void, v: c_int) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { if v != 0 { (*p.cast::<Mouse_HLInfo>()).bits |= MOUSE_FACE_HIDDEN; } else { (*p.cast::<Mouse_HLInfo>()).bits &= !MOUSE_FACE_HIDDEN; } 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn mouse_highlight_set_mouse_face_defer(p: *mut c_void, v: c_int) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { if v != 0 { (*p.cast::<Mouse_HLInfo>()).bits |= MOUSE_FACE_DEFER; } else { (*p.cast::<Mouse_HLInfo>()).bits &= !MOUSE_FACE_DEFER; } 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_next(p: *mut c_void) -> *mut c_void {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<ns_display_info>()).next as *mut c_void 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_grabbed(p: *mut c_void) -> c_int {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<ns_display_info>()).grabbed 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_highlight_frame(f: *mut c_void) -> *mut c_void {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*FRAME_DISPLAY_INFO!(f.cast::<frame>())).highlight_frame.cast() 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_ns_focus_frame(f: *mut c_void) -> *mut c_void {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*FRAME_DISPLAY_INFO!(f.cast::<frame>())).ns_focus_frame.cast() 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_ns_focus_frame_from_display(p: *mut c_void) -> *mut c_void {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<ns_display_info>()).ns_focus_frame.cast() 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_set_highlight_frame(f: *mut c_void, v: *mut c_void) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*FRAME_DISPLAY_INFO!(f.cast::<frame>())).highlight_frame = v.cast::<frame>(); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_set_ns_focus_frame(f: *mut c_void, v: *mut c_void) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*FRAME_DISPLAY_INFO!(f.cast::<frame>())).ns_focus_frame = v.cast::<frame>(); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_set_last_mouse_motion_frame(f: *mut c_void, v: *mut c_void) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*FRAME_DISPLAY_INFO!(f.cast::<frame>())).last_mouse_motion_frame = v.cast::<frame>(); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_set_grabbed(p: *mut c_void, v: c_int) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<ns_display_info>()).grabbed = v; 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_set_last_mouse_motion_x(p: *mut c_void, v: c_int) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<ns_display_info>()).last_mouse_motion_x = v; 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_set_last_mouse_motion_y(p: *mut c_void, v: c_int) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<ns_display_info>()).last_mouse_motion_y = v; 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_set_last_mouse_frame(p: *mut c_void, v: *mut c_void) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<ns_display_info>()).last_mouse_frame = v.cast::<frame>(); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_set_last_mouse_movement_time(p: *mut c_void, v: c_ulong) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*p.cast::<ns_display_info>()).last_mouse_movement_time = v; 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_display_info_last_mouse_glyph(f: *mut c_void) -> *mut NSRect {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { core::ptr::addr_of_mut!((*FRAME_DISPLAY_INFO!(f.cast::<frame>())).last_mouse_glyph) 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_double_buffered(f: *mut c_void) -> bool {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*(*f.cast::<frame>()).output_data.ns.cast::<ns_output>()).double_buffered 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_output_current_pointer(f: *mut c_void) -> *mut c_void {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { (*(*f.cast::<frame>()).output_data.ns.cast::<ns_output>()).current_pointer 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn femacs_get_ns_antialias_text() -> Lisp_Object {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { cocoa_runtime_exports::ns_antialias_text }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn femacs_get_ns_use_thin_smoothing() -> Lisp_Object {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { cocoa_runtime_exports::ns_use_thin_smoothing }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn __darwin_fd_isset(fd: c_int, set: *const fd_set) -> c_int {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { FD_ISSET(fd, set) as c_int 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn __darwin_fd_set(fd: c_int, set: *mut fd_set) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { FD_SET(fd, set); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn __darwin_fd_zero(set: *mut fd_set) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { FD_ZERO(set); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn XSETINT(p: *mut Lisp_Object, n: EMACS_INT) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { XSETINT!(*p, n); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn XSETWINDOW(p: *mut Lisp_Object, w: *mut window) -> () {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe { XSETWINDOW!(*p, w); 
  }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn LGLYPH_SET_CODE(glyph: Lisp_Object, code: EMACS_INT) -> () { LGLYPH_SET_CODE!(glyph, code); }
}

// Core fields hold one native reference; Swift getter wrappers borrow it.
#[cfg(feature = "have_ns")]
unsafe extern "C" {
    fn objc_retain(value: *mut c_void) -> *mut c_void;
    fn objc_release(value: *mut c_void);
}
#[cfg(feature = "have_ns")]
unsafe fn replace_native_object(slot: *mut *mut c_void, value: *mut c_void) {
  // SAFETY: display/frame/glyph/window or Cocoa/TTY bridge boundary; caller supplies live frame/window/glyph/image pointers under the owning VM/UI thread lock.
  unsafe {
    let retained = objc_retain(value);
    let old = *slot;
    *slot = retained;
    objc_release(old);

  }
}

#[cfg(feature = "have_ns")]
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ns_frame_parm_handlers_address() -> *mut c_void {
    core::ptr::addr_of_mut!(cocoa_runtime_exports::ns_frame_parm_handlers).cast()
}
