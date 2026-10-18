// femacs: port of src/androidterm.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// struct, struct field, macro, file-scope object, local), the same control flow and the same
// error model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2);
// the C `static` helpers stay plain `unsafe fn` except the seven whose address C takes - they
// become `unsafe extern "C" fn` items (a function-pointer member cannot be a Rust-ABI item; the
// androidmenu.rs / androidselect.rs / androidfont.rs shape).
//
// **This is the first slice of P-86's largest Android unit: C 1-829.**  `configure.ac` 2597 puts
// `androidterm.o` in `ANDROID_OBJ` under `AC_DEFINE([HAVE_ANDROID])` (2594) - both Android
// branches, like `androidmenu.o` - so `docs/baseline/frozen-build-sources.txt` lists no
// `androidterm.o` for this target, `extract-abi.py`'s checklist for the file is empty, and P-86
// ②'s substitute is what the ledger `docs/ports/androidterm.md` carries: the C line-range table,
// the `## Evidence` readings and the `## Deviations`.  Two gates, in C's order:
//
//   * the file gate (`HAVE_ANDROID`) is `feature = "have_android"`, on every item *and on C 21-36's
//     include graph* - which is why the frozen configuration compiles nothing here (P-86 ⑧);
//   * C 47's `#ifndef ANDROID_STUBIFY` ... C 6662's `#endif` (this file's file-level arm has **no**
//     `#else`, so its stub arm is *nothing*; the three stub `#if`s the file carries are all past
//     this slice) is `feature = "not_android_stubify"`, the feature the tree registered for C's
//     `HAVE_ANDROID && !defined ANDROID_STUBIFY` (`docs/baseline/feature-coverage.tsv` line 640).
//
// **Why 829.**  P-86 ④ says to stop at a whole-item boundary and P-86 ⑦ that the ledger must name
// who is next.  C 836-1892 is `handle_one_android_event`, one 1,056-line function - the next
// whole-item boundary after it is C 1892, far past 1,000 - so this slice ends at C 829's `}`, the
// end of `android_handle_ime_event`.  **The next live definition is `handle_one_android_event`
// (C 836-1892), with its forward declaration `static void android_notify_conversion (unsigned
// long);` at C 834** (the port's table records C 830-7117 as the remainder).
//
// A third C condition has no `#if` in this file and therefore no gate either: `HAVE_WINDOW_SYSTEM`
// (configure.ac 7518; the `window_system=android` branch at 2594-2596 is what puts this file in the
// build list at all, so an Android configuration always has it).  Four places in this slice's text
// need it - C 519's `Emacs_Rectangle *r` (dispextern.h 47-64 sits under `#ifdef
// HAVE_WINDOW_SYSTEM`), C 543's `note_mouse_highlight`, C 545's `remember_mouse_glyph` and C 350's
// `buffer_flipping_blocked_p` - so the ledger records that the arm reading is a
// `have_android`-plus-`not_android_stubify` reading of the mirror's *frozen* side, and names the
// `include/frame.rs`/`include/dispextern.rs` window-system round that is owed by other lanes.
//
// The three known mirror blockers this slice touches are named in the ledger, not repaired here:
// the three `bool_bf` macros `FRAME_ANDROID_DRAWABLE`/`FRAME_ANDROID_COMPLETE_P`/
// `FRAME_ANDROID_NEED_BUFFER_FLIP` and the direct `need_cursor_updates` access (the mirror folds
// those five `android_output` bits into `bits`, `include/androidterm.rs` 229-247), the forward
// call to `android_frame_rehighlight` (defined at C 2127, past the slice), and the seven
// use-site declarations in the block below.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 20: `#include <config.h>` (ungated)

/* C 21-24: the four system headers, on the file gate like the rest of the graph (P-86 ⑧).
   C 24's `<semaphore.h>` has no mirror in the tree; the only names C takes from it (`sem_t`,
   `sem_timedwait`, `sem_getvalue`, `sem_post`, `sem_init`, C 5032-6864) are all past this slice,
   so the include stands as this comment and the ledger records the gap (P-88 ①).  */
#[cfg (feature = "have_android")]
include!("../lib/include/stdio.rs");                  // C 21: `#include <stdio.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/math.rs");                   // C 22: `#include <math.h>`
#[cfg (feature = "have_android")]
include!("../lib/include/stdlib.rs");                 // C 23: `#include <stdlib.h>`

/* C 26-36: the file's own include list.  It sits outside C 47's `#ifndef`, so the whole graph is
   on the file gate alone - no translation unit the frozen configuration builds reaches any of it.  */
#[cfg (feature = "have_android")]
include!("include/lisp.rs");                          // C 26: `#include "lisp.h"`
#[cfg (feature = "have_android")]
include!("include/androidterm.rs");                   // C 27: `#include "androidterm.h"`
#[cfg (feature = "have_android")]
include!("include/keyboard.rs");                      // C 28: `#include "keyboard.h"`
#[cfg (feature = "have_android")]
include!("include/blockinput.rs");                    // C 29: `#include "blockinput.h"`
#[cfg (feature = "have_android")]
include!("include/android.rs");                       // C 30: `#include "android.h"`
#[cfg (feature = "have_android")]
include!("include/buffer.rs");                        // C 31: `#include "buffer.h"`
#[cfg (feature = "have_android")]
include!("include/window.rs");                        // C 32: `#include "window.h"`
/* `MOUSE_HL_INFO!` is `include/frame.rs` 450-461's macro, and its body names `tty_output` -
   which frame.rs cannot include (termchar.rs includes frame.rs instead), so the *including* unit
   brings termchar.rs in, exactly as that mirror's comment says.  The frozen configuration has no
   window system, so the macro's live arm is C 1037-1039's TTY one and the HAVE_ANDROID arm this
   file needs (`&FRAME_ANDROID_OUTPUT (f)->mouse_highlight`) is the recorded mirror round the
   ledger owes: until it lands, C 332's expansion reads the wrong arm and is a named blocker, not
   a port choice.  */
#[cfg (feature = "have_android")]
include!("include/termchar.rs");                      // `tty_output` (frame.rs 450's MOUSE_HL_INFO!)
#[cfg (feature = "have_android")]
include!("include/textconv.rs");                      // C 33: `#include "textconv.h"`
#[cfg (feature = "have_android")]
include!("include/coding.rs");                        // C 34: `#include "coding.h"`
#[cfg (feature = "have_android")]
include!("include/pdumper.rs");                       // C 35: `#include "pdumper.h"`
#[cfg (feature = "have_android")]
include!("include/keymap.rs");                        // C 36: `#include "keymap.h"`

/* Three headers the file reaches through `lisp.h`/`systime.h` rather than naming, plus the two
   `include/androidterm.rs` pulls in (`dispextern.rs`/`androidgui.rs`/`frame.rs`/`termhooks.rs`).
   The C side gets `memset` from lisp.h 30's `<string.h>`, `fd_set`/`FD_ZERO`/`pselect` from
   `<sys/select.h>` (lisp.h 33's own `#include <limits.h>` supplies INT_MAX, used at C 390).  */
#[cfg (feature = "have_android")]
include!("../lib/include/string.rs");                 // lisp.h 30's `#include <string.h>` (memset)
#[cfg (feature = "have_android")]
include!("../lib/include/limits.in.rs");              // lisp.h 33's `#include <limits.h>` (INT_MAX, C 390)
#[cfg (feature = "have_android")]
include!("../lib/include/sys_select.rs");             // `fd_set`/`FD_ZERO`/`pselect` (C 199-251)

/* `struct face` (C dispextern.h 1787-1937) - the use-site copy `src/xfaces.rs` 288 and a dozen
   other display units carry, for the reason the mirror's `dispextern_face_records` ruling
   records (status 3.220): this unit needs the record in scope, and a second definition in the
   same crate is `E0428`.  It had no copy, so its `face`-typed locals and casts read
   `69 x cannot find type face` on `aarch64-linux-android` (measured).  The text below is the
   mirror's, arm for arm, with the feature gate spent: `have_window_system` selects C's own
   layout, whose Android arm is measured (`gc`@168, `stipple`@176, `foreground`@184, sizeof 296 -
   docs/baseline/face-layout-android.tsv; the mirror's assertions read the same numbers).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct face
{
  pub lface: [Lisp_Object; LFACE_VECTOR_SIZE as usize],
  pub id: c_int,

  // dispextern.h 1797-1811: the `gc` (C's three arms) and `stipple` that C adds to the face
  // record under `#ifdef HAVE_WINDOW_SYSTEM`.  Read from `docs/baseline/face-layout.tsv` (P-82: frozen
  // 280/8, both window-system arms 296/8, `gc` at 168 and `stipple` at 176 in both), which is
  // also why the two window-system arms add the same sixteen bytes.
  #[cfg (all (feature = "have_window_system", feature = "have_x_windows"))]
  pub gc: *mut c_void,        /// C: `GC gc;` (1799) - X11's `struct _XGC *`; no mirror, so the
                              /// pointer stays untyped (P-72 rule 3, as in `glyph_string`)
  #[cfg (all (feature = "have_window_system", feature = "have_android"))]
  pub gc: *mut c_void,        /// C: `struct android_gc *gc;` (1801)
  #[cfg (all (feature = "have_window_system", not (feature = "have_x_windows"),
              not (feature = "have_android")))]
  pub gc: *mut Emacs_GC,      // C: `Emacs_GC *gc;` (1803)
  #[cfg (feature = "have_window_system")]
  pub stipple: ptrdiff_t,     // 1810 (measured 176)
  pub foreground: c_ulong,
  pub background: c_ulong,
  pub underline_color: c_ulong,
  pub overline_color: c_ulong,
  pub strike_through_color: c_ulong,
  pub box_color: c_ulong,
  /* P-72 rule ③: `struct font` is font.rs's type and this mirror cannot reach it (C's
     dispextern.h only knows the incomplete tag), so the member is the untyped pointer; a unit
     that includes `include/font.rs` casts it (`src/xfaces.rs` 294 spells it `*mut font`).  */
  pub font: *mut c_void,   /// C: `struct font *font;` (dispextern.h 1845)
  pub fontset: c_int,
  pub box_vertical_line_width: c_int,
  pub box_horizontal_line_width: c_int,
  pub underline_pixels_above_descent_line: c_int,
  /* C 1893-1935: `ENUM_BF (face_box_type) box : 2; ENUM_BF (face_underline_type)
     underline : 3;` and the seventeen 1-bit `bool_bf` members in one storage unit.  The port
     stores C's bit-field bytes and names every member through the masks below (PORTING.md
     1.3/P-80).  */
  pub bits: u32,
  pub hash: uintptr_t,
  pub next: *mut face,
  pub prev: *mut face,
  pub ascii_face: *mut face,
}

/* C 1797-1811's two `HAVE_WINDOW_SYSTEM` members sit between `id` and `foreground`, so every
   member from `foreground` on is sixteen bytes further along in a window-system build
   (`docs/baseline/face-layout.tsv`: `frozen` 280/8 with `foreground`@168, `x11`/`ns_cocoa`/
   `x11_ext` 296/8 with `foreground`@184 - the same two members, measured in both).  The
   assertions below add this shift rather than repeating a second table, so the one set of
   measured numbers covers every arm; status 3.505 added it after `src/fringe.rs`'s flip read
   `13 x E0080` on the frozen values.  */
#[cfg (not (feature = "have_window_system"))]
const FACE_WS_SHIFT: usize = 0;
#[cfg (feature = "have_window_system")]
const FACE_WS_SHIFT: usize = 16;

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::size_of::<face> () == 280 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::align_of::<face> () == 8);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, id) == 160);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, gc) == 168);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face, stipple) == 176);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, foreground) == 168 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, background) == 176 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, underline_color) == 184 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, overline_color) == 192 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, strike_through_color) == 200 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, box_color) == 208 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, font) == 216 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, fontset) == 224 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, bits) == 240 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, hash) == 248 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, next) == 256 + FACE_WS_SHIFT);
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = core::assert! (std::mem::offset_of! (face, ascii_face) == 272 + FACE_WS_SHIFT);

/* `frame.h` 1379/1646-1661's three accessors: the mirror keeps them in its own
   `frame_face_accessors` module behind `dispextern_face_records` (which no configuration
   produces), and this unit calls them six times, so it carries the module itself - the
   `src/haikufns.rs` 857-876 device, and the shape status 3.220's ruling prescribes for every
   unit that needs one of the gated records.  */
/* `struct face_cache` (C dispextern.h 1991-2009) - the use-site copy status 3.220 prescribes.  */
/// C 1991-2009: `struct face_cache`.  Measured: `sizeof` 40, `buckets`@0, `f`@8,
/// `faces_by_id`@16, `size`@24, `used`@32, and C's one `bool_bf menu_face_changed_p : 1` in
/// the int unit at 36 (carried as a byte, the P-80 convention - `src/window.rs` 907 spells
/// the same member `bool`, one byte wide either way).
#[cfg (feature = "have_window_system")]
#[repr(C)]
#[cfg (feature = "have_window_system")]
pub struct face_cache
{
  pub buckets: *mut *mut face,
  /* P-72 rule ③: `struct frame` is frame.rs's type and a unit that includes this mirror need
     not include that one (measured: `build-tu src/bidi.c` and `src/lread.c` answered "cannot
     find type frame" while this member was `*mut frame`), so the member is the untyped
     pointer - the same pointer, which is what C's incomplete `struct frame *` is.  */
  pub f: *mut c_void,   /// C: `struct frame *f;` (dispextern.h 1997)
  pub faces_by_id: *mut *mut face,
  pub size: ptrdiff_t,
  pub used: c_int,
  pub menu_face_changed_p: u8,
}
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::size_of::<face_cache> () == 40);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, buckets) == 0);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, f) == 8);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, faces_by_id) == 16);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, size) == 24);
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (std::mem::offset_of! (face_cache, used) == 32);

/* `include/frame.rs` packs C's bit fields into `bits1`/`bits2` (the mirror's `new_size_p` and
   `want_fullscreen` notes), so this unit reads them the way `src/dispnew.rs` 497 and
   `src/haikuterm.rs` 3058 do - the mirror's own shifts and masks, not named members.  */
#[inline(always)]
#[cfg (feature = "have_window_system")]
pub fn frame_want_fullscreen (f: *mut frame) -> fullscreen_type {
  ((unsafe { (*f).bits2 } >> FRAME_WANT_FULLSCREEN_SHIFT) & FRAME_WANT_FULLSCREEN_MASK) as fullscreen_type
}
#[inline(always)]
#[cfg (feature = "have_window_system")]
pub fn frame_new_size_p (f: *mut frame) -> bool {
  (unsafe { (*f).bits2 } >> FRAME_NEW_SIZE_P_SHIFT & 1) != 0
}

#[cfg (feature = "have_window_system")]
pub mod frame_face_accessors
{
  use super::*;

  /// C 1379: `#define FRAME_FACE_CACHE(F) (F)->face_cache`
  #[inline (always)]
  pub fn FRAME_FACE_CACHE (f: *mut frame) -> *mut face_cache
  { unsafe { (*f).face_cache as *mut face_cache } }

  /// C 1646-1650: `INLINE struct face * FACE_FROM_ID (struct frame *f, int id)`
  #[inline (always)]
  pub fn FACE_FROM_ID (f: *mut frame, id: c_int) -> *mut face
  {
    eassert! (0 <= id && id < unsafe { (*FRAME_FACE_CACHE (f)).used });
    unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) }
  }

  /// C 1656-1661: `INLINE struct face * FACE_FROM_ID_OR_NULL (struct frame *f, int id)`
  #[inline (always)]
  pub fn FACE_FROM_ID_OR_NULL (f: *mut frame, id: c_int) -> *mut face
  {
    let used: c_int = unsafe { (*FRAME_FACE_CACHE (f)).used };
    eassume! (0 <= used);
    if 0 <= id && id < used { unsafe { *(*FRAME_FACE_CACHE (f)).faces_by_id.add (id as usize) } }
    else { core::ptr::null_mut () }
  }
}

/* `struct redisplay_interface` - dispextern.h 3069-3196.  The mirror carries the record inside
   its `dispextern_unit_names` module with the `struct window *`/`struct frame *` slots degraded to
   `*mut c_void` (its own comments say so), and this unit's table (C 6664-6696) needs C's own
   parameter types: the port therefore keeps a **local copy** with C's types - the
   `src/haikuterm.rs` 4485-4554 device, whose ledger note records exactly this trade against
   `src/pgtkterm.rs` 5412-5480's fifteen `transmute`s - and the four numbers pin the members this
   file reads (`flush_display` 72, `draw_fringe_bitmap` 104, `clear_under_internal_border` 160,
   `default_font_parameter` 216, `sizeof` 224).  `struct it` has no record in this unit, so C's
   `struct it *` keeps the pointer-sized stand-in (P-72 rule 3, the haikuterm copy's choice).
   Slice 1 declared only the first two members here (the prefix array and `flush_display`);
   slice 26 grew the copy to the whole record.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
pub struct redisplay_interface
{
  pub frame_parm_handlers: *mut c_void,                  // C 3072, offset 0
  pub produce_glyphs: Option<unsafe extern "C" fn (it: *mut c_void)>,  // C 3076, offset 8
  pub write_glyphs: Option<unsafe extern "C" fn (w: *mut window, row: *mut glyph_row,
						 string: *mut glyph, area: glyph_row_area,
						 len: c_int)>,                    // C 3080-3082, offset 16
  pub insert_glyphs: Option<unsafe extern "C" fn (w: *mut window, row: *mut glyph_row,
						  start: *mut glyph, area: glyph_row_area,
						  len: c_int)>,                   // C 3083-3085, offset 24
  pub clear_end_of_line: Option<unsafe extern "C" fn (w: *mut window, row: *mut glyph_row,
						      area: glyph_row_area, x: c_int)>,  // C 3089-3090, offset 32
  pub scroll_run_hook: Option<unsafe extern "C" fn (w: *mut window, run: *mut run)>,  // C 3094, offset 40
  pub after_update_window_line_hook: Option<unsafe extern "C" fn (w: *mut window, desired_row: *mut glyph_row)>,  // C 3099-3100, offset 48
  pub update_window_begin_hook: Option<unsafe extern "C" fn (w: *mut window)>,  // C 3104, offset 56
  pub update_window_end_hook: Option<unsafe extern "C" fn (w: *mut window, cursor_on_p: bool,
							   mouse_face_overwritten_p: bool)>,  // C 3111-3112, offset 64
  pub flush_display: Option<unsafe extern "C" fn (f: *mut frame)>,  // C 3115, offset 72
  pub clear_window_mouse_face: Option<unsafe extern "C" fn (w: *mut window)>,  // C 3118, offset 80
  pub get_glyph_overhangs: Option<unsafe extern "C" fn (glyph: *mut glyph, f: *mut frame,
							left: *mut c_int, right: *mut c_int)>,  // C 3122-3123, offset 88
  pub fix_overlapping_area: Option<unsafe extern "C" fn (w: *mut window, row: *mut glyph_row,
							 area: glyph_row_area, overlaps: c_int)>,  // C 3128-3129, offset 96
  pub draw_fringe_bitmap: Option<unsafe extern "C" fn (w: *mut window, row: *mut glyph_row,
						       p: *mut draw_fringe_bitmap_params)>,  // C 3134-3135, offset 104
  pub define_fringe_bitmap: Option<unsafe extern "C" fn (which: c_int, bits: *mut c_ushort,
							 h: c_int, wd: c_int)>,  // C 3138-3139, offset 112
  pub destroy_fringe_bitmap: Option<unsafe extern "C" fn (which: c_int)>,  // C 3140, offset 120
  pub compute_glyph_string_overhangs: Option<unsafe extern "C" fn (s: *mut glyph_string)>,  // C 3144, offset 128
  pub draw_glyph_string: Option<unsafe extern "C" fn (s: *mut glyph_string)>,  // C 3147, offset 136
  pub define_frame_cursor: Option<unsafe extern "C" fn (f: *mut frame, cursor: Emacs_Cursor)>,  // C 3150, offset 144
  pub clear_frame_area: Option<unsafe extern "C" fn (f: *mut frame, x: c_int, y: c_int,
						     width: c_int, height: c_int)>,  // C 3153-3154, offset 152
  pub clear_under_internal_border: Option<unsafe extern "C" fn (f: *mut frame)>,  // C 3159, offset 160
  pub draw_window_cursor: Option<unsafe extern "C" fn (w: *mut window, glyph_row: *mut glyph_row,
						       x: c_int, y: c_int,
						       cursor_type: text_cursor_kinds,
						       cursor_width: c_int, on_p: bool,
						       active_p: bool)>,  // C 3165-3169, offset 168
  pub draw_vertical_window_border: Option<unsafe extern "C" fn (w: *mut window, x: c_int,
								y_0: c_int, y_1: c_int)>,  // C 3172-3173, offset 176
  pub draw_window_divider: Option<unsafe extern "C" fn (w: *mut window, x_0: c_int, x_1: c_int,
							y_0: c_int, y_1: c_int)>,  // C 3176-3177, offset 184
  pub shift_glyphs_for_insert: Option<unsafe extern "C" fn (f: *mut frame, x: c_int, y: c_int,
							    width: c_int, height: c_int,
							    shift_by: c_int)>,  // C 3182-3184, offset 192
  pub show_hourglass: Option<unsafe extern "C" fn (f: *mut frame)>,  // C 3187, offset 200
  pub hide_hourglass: Option<unsafe extern "C" fn (f: *mut frame)>,  // C 3190, offset 208
  pub default_font_parameter: Option<unsafe extern "C" fn (f: *mut frame, parms: Lisp_Object)>,  // C 3194, offset 216
}
/* C 3131-3195's fifteen `HAVE_WINDOW_SYSTEM` members are appended after `fix_overlapping_area`:
   `draw_fringe_bitmap`@104 … `default_font_parameter`@216, `sizeof` 224, align 8 - the
   `docs/baseline/redisplay_interface-layout.tsv` arm this record's.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  assert! (std::mem::size_of::<redisplay_interface> () == 224);
  assert! (std::mem::align_of::<redisplay_interface> () == 8);
  assert! (std::mem::offset_of! (redisplay_interface, flush_display) == 72);
  assert! (std::mem::offset_of! (redisplay_interface, draw_fringe_bitmap) == 104);
  assert! (std::mem::offset_of! (redisplay_interface, clear_under_internal_border) == 160);
  assert! (std::mem::offset_of! (redisplay_interface, default_font_parameter) == 216);
};

/* C 353/408's `flush_frame (f)`.  C's is frame.h 1914-1921's inline and `include/frame.rs`
   1193-1200 carries its `macro_rules!`, but that body's `FRAME_RIF!` hands back the mirror's
   `*mut c_void` (`include/termhooks.rs` 219 degrades `rif`), so the macro cannot type-check as
   written (measured: `E0308 expected *mut redisplay_interface, found *mut c_void` at
   `termhooks.rs` 425, two call sites).  The unit therefore writes C's own body with C's own text,
   the `minibuf.rs` 645-651 shape, and the ledger records the mirror round.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn flush_frame (f: *mut frame)
{
  let rif: *mut redisplay_interface =
    (*(*f).terminal).rif as *mut redisplay_interface;   /* C: `FRAME_RIF (f)`.  */

  if !rif.is_null () && (*rif).flush_display.is_some ()
    { ((*rif).flush_display).unwrap () (f); }
}

/* `include/textconv.rs` 160-243 puts textconv.h's non-Android declarations in
   `pub mod textconv_decls`, and that mirror's own header comment says the module carries "a glob
   import (P-17/P-21)"; the glob line is not in the frozen mirror (no unit called these ten
   functions yet), so this unit writes the use site rather than repeating the ten declarations
   (PORTING.md P-26: one declaration per name, in the mirror).  The ledger records the missing
   line for the integrator.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
use textconv_decls::*;

/* Eight declarations the sixteen headers above reach, which the frozen mirrors do not carry yet
   (P-72 rule 4: a mirror must reach what it needs through its own `include!`s).  Until each lands
   in its mirror the unit supplies it at the use site - the `alloc.rs` `mark_haiku_display` /
   `androidfont.rs` seven-declaration shape; every line names the C header it comes from, and
   `docs/ports/androidterm.md` records the gap for the integrator.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3879: `int popup_activated (void);` (androidmenu.c defines it).
  pub fn popup_activated () -> c_int;
  /// C: dispextern.h 4006: `extern bool gui_mouse_grabbed (Display_Info *);` -
  /// dispextern.h 165's `typedef struct android_display_info Display_Info;`.
  pub fn gui_mouse_grabbed (dpyinfo: *mut android_display_info) -> bool;
  /// C: dispextern.h 3719: `extern bool buffer_flipping_blocked_p (void);`
  pub fn buffer_flipping_blocked_p () -> bool;
  /// C: dispextern.h 3586: `void remember_mouse_glyph (struct frame *, int, int,
  /// NativeRectangle *);` - androidgui.h 218's `#define NativeRectangle Emacs_Rectangle`.
  pub fn remember_mouse_glyph (f: *mut frame, x: c_int, y: c_int,
                               rectangle: *mut Emacs_Rectangle);
  /// C: dispextern.h 3688: `extern void note_mouse_highlight (struct frame *, int, int);`
  pub fn note_mouse_highlight (f: *mut frame, x: c_int, y: c_int);
  /// C: coding.h 686: `extern void setup_coding_system (Lisp_Object, struct coding_system *);`
  pub fn setup_coding_system (system: Lisp_Object, coding: *mut coding_system);
  /// C: lisp.h 5095: `extern bool detect_input_pending (void);`
  pub fn detect_input_pending () -> bool;
  /// C: androidgui.h 926-927: `extern struct android_gc *android_create_gc (enum
  /// android_gc_value_mask, struct android_gc_values *);`
  pub fn android_create_gc (valuemask: android_gc_value_mask, values: *mut android_gc_values)
    -> *mut android_gc;
}

/* C 49: `#include <android/log.h>`.  The NDK's own header is not part of the reference tree, so it
   has no mirror (P-88 ①).  This slice takes no name from it - the file's only use,
   `__android_log_print` (C 1837), is inside the remainder - so the include stands as this comment.  */

/* C 61-67: C's anonymous `enum` with the three event-dispatch codes.  C names no tag and the
   members are never assigned, so they are 0/1/2 (`cm.rs`'s `CM_*` unnamed-enum shape).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_EVENT_NORMAL: c_int = 0;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_EVENT_GOTO_OUT: c_int = 1;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const ANDROID_EVENT_DROP: c_int = 2;

/* termhooks.h 373-412 says `enum event_kind kind : 16`, which the mirror's `input_event::bits`
   shares with `part : 16` (PORTING.md 1.3/P-80).  C writes `e->kind = x;`, so the port keeps C's
   field name in a per-unit macro - the `androidselect.rs` 136-143 shape, body-identical.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! input_event_bits {
  (kind, $e:expr) => { ((*$e).bits & INPUT_EVENT_KIND_MASK) as event_kind };
  (kind, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & !INPUT_EVENT_KIND_MASK) | ((($v) as u32) & INPUT_EVENT_KIND_MASK); };
  (part, $e:expr) => { ((*$e).bits >> INPUT_EVENT_PART_SHIFT) as c_int };
  (part, $e:expr, $v:expr) => {
    (*$e).bits = ((*$e).bits & INPUT_EVENT_KIND_MASK)
                 | (((($v) as u32) & INPUT_EVENT_KIND_MASK) << INPUT_EVENT_PART_SHIFT); };
}

/* frame.h 300-341's `bool_bf` flag run.  C's `f->auto_raise`, `f->auto_lower`, `f->mouse_moved`
   and `f->pointer_invisible` are struct fields, so no header mirror carries an accessor; the unit
   that owns `struct frame` (frame.rs) and each unit whose bodies touch the fields (keyboard.rs,
   term.rs) define their own per-unit macro, and this unit does the same for the four its slice
   touches (PORTING.md 1.3; docs/ports/frame.md).  The shift constants are `include/frame.rs`'s and
   are gated to the frozen (non-window-system) arm - a window-system configuration has no numbering
   there yet, which is the `include/frame.rs` round the ledger owes (docs/ports/include-frame.md).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! set_frame_flag1 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! auto_raise {
  ($f:expr) => { (*$f).bits1 >> FRAME_AUTO_RAISE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_AUTO_RAISE_SHIFT, $v) };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! auto_lower {
  ($f:expr) => { (*$f).bits1 >> FRAME_AUTO_LOWER_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_AUTO_LOWER_SHIFT, $v) };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! mouse_moved {
  ($f:expr) => { (*$f).bits1 >> FRAME_MOUSE_MOVED_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_MOUSE_MOVED_SHIFT, $v) };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! pointer_invisible {
  ($f:expr) => { (*$f).bits1 >> FRAME_POINTER_INVISIBLE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_POINTER_INVISIBLE_SHIFT, $v) };
}

/* C 41: `struct android_display_info *x_display_list;` - a non-`static` definition, so
   `#[unsafe(no_mangle)]` (PORTING.md 0.2); `include/androidterm.rs` 517 declares it in the
   glob-imported `androidterm_objs` module, which this definition shadows (PORTING.md 0.1-10).  */
#[cfg (feature = "have_android")]
#[unsafe(no_mangle)]
pub static mut x_display_list: *mut android_display_info = std::ptr::null_mut ();

/* ------------------------------------------------------------------------------------------
   C 47-829: the whole body of this slice, under C 47's `#ifndef ANDROID_STUBIFY`.  C writes no
   `#else` here - the file's three stub arms (C 6666/6712/6737/6884) are all past C 829 - so a
   configuration with `ANDROID_STUBIFY` compiles the include graph and no item of this slice.  */

/* C 54: `static bool any_help_event_p;` (tentative, i.e. zero).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut any_help_event_p: bool = false;

/* C 59: `static double wheel_event_x, wheel_event_y;` (two names, one C statement).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut wheel_event_x: f64 = 0.0;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut wheel_event_y: f64 = 0.0;

/* Find the frame whose window has the identifier WDESC.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_window_to_frame (dpyinfo: *mut android_display_info,
                                   wdesc: android_window) -> *mut frame
{                                                              // C 74-96
  let mut tail: Lisp_Object;
  let mut frame: Lisp_Object;
  let mut f: *mut frame;

  if wdesc == ANDROID_NONE as android_window
    { return std::ptr::null_mut (); }

  /* C's `continue` jumps to the `for` increment; the mirror's FOR_EACH_FRAME puts the increment
     at the body's end, so the body is a labelled block and `break 'continue_;` leaves it - the
     increment still runs (`frame.rs` 2836-2845, docs/ports/frame.md).  */
  FOR_EACH_FRAME! (tail, frame, {
    'continue_: {
      f = XFRAME (frame);

      if !FRAME_ANDROID_P (f)
	{ break 'continue_; }

      if FRAME_ANDROID_WINDOW! (f) == wdesc
        { return f; }
    }
  });

  return std::ptr::null_mut ();
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_clear_frame (f: *mut frame)             // C 98-105
{
  /* Clearing the frame will erase any cursor, so mark them all as no
     longer visible.  */
  mark_window_cursors_off (XWINDOW (FRAME_ROOT_WINDOW (f)));
  android_clear_window (FRAME_ANDROID_DRAWABLE! (f));
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_show_hourglass (f: *mut frame)          // C 107-132
{
  let mut x: *mut android_output;

  /* This isn't implemented like X because a window brings alongside
     too many unneeded resources.  */

  x = FRAME_ANDROID_OUTPUT! (f) as *mut android_output;

  /* If the hourglass window is mapped inside a popup menu, input
     could be lost if the menu is popped down and the grab is
     relinquished, but the hourglass window is still up.  Just
     avoid displaying the hourglass at all while popups are
     active.  */

  if popup_activated () != 0
    { return; }

  (*x).hourglass = true;

  /* An hourglass cursor ought to be visible whether or not the standard
     cursor is invisible.  */
  android_define_cursor (FRAME_ANDROID_WINDOW! (f),
			 (*x).hourglass_cursor);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn make_invisible_cursor (dpyinfo: *mut android_display_info) -> android_cursor
{                                                                    // C 134-138
  return android_create_font_cursor (ANDROID_XC_NULL);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_hide_hourglass (f: *mut frame)          // C 140-161
{
  let mut x: *mut android_output;
  let mut dpyinfo: *mut android_display_info;

  x = FRAME_ANDROID_OUTPUT! (f) as *mut android_output;
  dpyinfo = FRAME_DISPLAY_INFO! (f);
  (*x).hourglass = false;

  if !pointer_invisible! (f)
    { android_define_cursor (FRAME_ANDROID_WINDOW! (f), (*x).current_cursor); }
  else
    {
      if (*dpyinfo).invisible_cursor == 0
	{ (*dpyinfo).invisible_cursor = make_invisible_cursor (dpyinfo); }

      android_define_cursor (FRAME_ANDROID_WINDOW! (f),
			     (*dpyinfo).invisible_cursor);
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_flash (f: *mut frame)                              // C 163-264
{
  let mut gc: *mut android_gc;
  let mut values: android_gc_values = std::mem::zeroed ();   // C 166 (PORTING.md 1.5)
  let mut rc: c_int;
  let mut fds: fd_set = std::mem::zeroed ();                 // C 168 (PORTING.md 1.5)

  block_input ();
  values.function = ANDROID_GC_INVERT;
  gc = android_create_gc (ANDROID_GC_FUNCTION, &raw mut values);

  /* Get the height not including a menu bar widget.  */
  let mut height: c_int = FRAME_PIXEL_HEIGHT (f);
  /* Height of each line to flash.  */
  let mut flash_height: c_int = FRAME_LINE_HEIGHT (f);
  /* These will be the left and right margins of the rectangles.  */
  let mut flash_left: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);
  let mut flash_right: c_int = FRAME_PIXEL_WIDTH (f) - FRAME_INTERNAL_BORDER_WIDTH (f);
  let mut width: c_int = flash_right - flash_left;

  /* If window is tall, flash top and bottom line.  */
  if height > 3 * FRAME_LINE_HEIGHT (f)
    {
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left,
			      (FRAME_INTERNAL_BORDER_WIDTH (f)
			       + FRAME_TOP_MARGIN_HEIGHT (f)),
			      width as c_uint, flash_height as c_uint);
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left,
			      (height - flash_height
			       - FRAME_INTERNAL_BORDER_WIDTH (f)
			       - FRAME_BOTTOM_MARGIN_HEIGHT (f)),
			      width as c_uint, flash_height as c_uint);

    }
  else
    /* If it is short, flash it all.  */
    {
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left, FRAME_INTERNAL_BORDER_WIDTH (f),
			      width as c_uint, (height - 2
						* FRAME_INTERNAL_BORDER_WIDTH (f)) as c_uint);
    }

  flush_frame (f);

  let mut delay: timespec = make_timespec (0, 150 * 1000 * 1000);
  let mut wakeup: timespec = timespec_add (current_timespec (), delay);

  /* Keep waiting until past the time wakeup or any input gets
     available.  */
  while !detect_input_pending ()
    {
      let mut current: timespec = current_timespec ();
      let mut timeout: timespec;

      /* Break if result would not be positive.  */
      if timespec_cmp (wakeup, current) <= 0
	{ break; }

      /* How long `select' should wait.  */
      timeout = make_timespec (0, 10 * 1000 * 1000);

      /* Wait for some input to become available on the X
	 connection.  */
      FD_ZERO (&raw mut fds);

      /* Try to wait that long--but we might wake up sooner.  */
      rc = pselect (0, &raw mut fds, std::ptr::null_mut (), std::ptr::null_mut (),
		    &raw const timeout, std::ptr::null ());

      /* Some input is available, exit the visible bell.  */
      if rc >= 0
	{ break; }
    }

  /* If window is tall, flash top and bottom line.  */
  if height > 3 * FRAME_LINE_HEIGHT (f)
    {
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left,
			      (FRAME_INTERNAL_BORDER_WIDTH (f)
			       + FRAME_TOP_MARGIN_HEIGHT (f)),
			      width as c_uint, flash_height as c_uint);
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left,
			      (height - flash_height
			       - FRAME_INTERNAL_BORDER_WIDTH (f)
			       - FRAME_BOTTOM_MARGIN_HEIGHT (f)),
			      width as c_uint, flash_height as c_uint);
    }
  else
    /* If it is short, flash it all.  */
    {
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,
			      flash_left, FRAME_INTERNAL_BORDER_WIDTH (f),
			      width as c_uint, (height - 2
						* FRAME_INTERNAL_BORDER_WIDTH (f)) as c_uint);
    }

  android_free_gc (gc);
  flush_frame (f);

  unblock_input ();
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_ring_bell (f: *mut frame)               // C 266-277
{
  if visible_bell
    { android_flash (f); }
  else
    {
      block_input ();
      android_bell ();
      unblock_input ();
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_toggle_visible_pointer (f: *mut frame, invisible: bool)   // C 279-302
{
  let mut dpyinfo: *mut android_display_info;

  dpyinfo = FRAME_DISPLAY_INFO! (f);

  /* An hourglass cursor overrides invisibility.  */
  /* C's `goto set_invisibility` is a forward jump to the trailing statement, so the port writes
     the labelled block with a `break` (PORTING.md 2.2, the androidmenu.rs `'finish` shape).  */
  'set_invisibility: {
    if (*(FRAME_ANDROID_OUTPUT! (f) as *mut android_output)).hourglass
      { break 'set_invisibility; }

    if (*dpyinfo).invisible_cursor == 0
      { (*dpyinfo).invisible_cursor = make_invisible_cursor (dpyinfo); }

    if invisible
      { android_define_cursor (FRAME_ANDROID_WINDOW! (f),
			       (*dpyinfo).invisible_cursor); }
    else
      { android_define_cursor (FRAME_ANDROID_WINDOW! (f),
			       (*((*f).output_data.android as *mut android_output)).current_cursor); }
  }

  pointer_invisible! (f, invisible);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_toggle_invisible_pointer (f: *mut frame, invisible: bool)
{                                                                    // C 304-310
  block_input ();
  android_toggle_visible_pointer (f, invisible);
  unblock_input ();
}

/* Start an update of frame F.  This function is installed as a hook
   for update_begin, i.e. it is called when update_begin is called.
   This function is called prior to calls to gui_update_window_begin
   for each window being updated.  Currently, there is nothing to do
   here because all interesting stuff is done on a window basis.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_update_begin (f: *mut frame)            // C 318-324
{
  /* The frame is no longer complete, as it is in the midst of an
     update.  */
  /* C: `FRAME_ANDROID_COMPLETE_P (f) = false;` - C's macro *is* `((f)->output_data.android->complete)`,
     a place, and the port's folded byte has no such field: the writer site spells the mask
     (PORTING.md P-80, the `cm.rs` convention; macro-body row for C's member name).  */
  (*( (*f).output_data.android as *mut android_output )).bits &= !ANDROID_OUTPUT_COMPLETE;
}

/* End update of frame F.  This function is installed as a hook in
   update_end.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_update_end (f: *mut frame)              // C 329-334
{
  /* Mouse highlight may be displayed again.  */
  /* C 3039: `hlinfo->mouse_face_defer = false;` - the member is one of C 3019-3023's three
     `bool_bf : 1` flags the mirror folds and names through accessors (PORTING.md P-80, the
     `get_mouse_face_defer`/`set_mouse_face_defer` pair at `include/dispextern.rs` 2114/2117).  */
  set_mouse_face_defer (MOUSE_HL_INFO! (f), false);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn show_back_buffer (f: *mut frame)                           // C 336-348
{
  let mut swap_info: android_swap_info = std::mem::zeroed ();   // C 338 (PORTING.md 1.5)

  memset (&raw mut swap_info as *mut c_void, 0,
	  std::mem::size_of::<android_swap_info> ());
  swap_info.swap_window = FRAME_ANDROID_WINDOW! (f);
  swap_info.swap_action = ANDROID_COPIED;
  android_swap_buffers (&raw mut swap_info, 1);

  /* Now the back buffer no longer needs to be flipped.  */
  /* C 347: `FRAME_ANDROID_NEED_BUFFER_FLIP (f) = false;` - the same folded bit as above,
     written through the mask (PORTING.md P-80).  */
  (*( (*f).output_data.android as *mut android_output )).bits &= !ANDROID_OUTPUT_NEED_BUFFER_FLIP;
}

/* Flip back buffers on F if it has undrawn content.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_flush_dirty_back_buffer_on (f: *mut frame)         // C 350-364
{
  if FRAME_GARBAGED_P (f)
      || buffer_flipping_blocked_p ()
      /* If the frame is not already up to date, do not flush buffers
	 on input, as that will result in flicker.  */
      || !FRAME_ANDROID_COMPLETE_P! (f)
      || !FRAME_ANDROID_NEED_BUFFER_FLIP! (f)
    { return; }

  show_back_buffer (f);
}

/* Convert between the modifier bits Android uses and the modifier
   bits Emacs uses.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_android_to_emacs_modifiers (dpyinfo: *mut android_display_info,
                                              state: c_int) -> c_int   // C 369-393
{
  let mut mod_ctrl: c_int = ctrl_modifier;
  let mut mod_meta: c_int = meta_modifier;
  let mut mod_alt: c_int  = alt_modifier;
  let mut mod_super: c_int = super_modifier;
  let mut tem: Lisp_Object;

  tem = Fget (Vx_ctrl_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_ctrl = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }
  tem = Fget (Vx_alt_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_alt = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }
  tem = Fget (Vx_meta_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_meta = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }
  tem = Fget (Vx_super_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_super = (XFIXNUM (tem) & INT_MAX as EMACS_INT) as c_int; }

  return (if (state & ANDROID_CONTROL_MASK) != 0 { mod_ctrl } else { 0 })
	  | (if (state & ANDROID_SHIFT_MASK) != 0 { shift_modifier } else { 0 })
	  | (if (state & ANDROID_ALT_MASK)   != 0 { mod_meta } else { 0 })
	  | (if (state & ANDROID_SUPER_MASK) != 0 { mod_super } else { 0 })
	  | (if (state & ANDROID_META_MASK)  != 0 { mod_alt } else { 0 });
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_emacs_to_android_modifiers (dpyinfo: *mut android_display_info,
                                              state: intmax_t) -> c_int   // C 395-419
{
  let mut mod_ctrl: EMACS_INT  = ctrl_modifier as EMACS_INT;
  let mut mod_meta: EMACS_INT  = meta_modifier as EMACS_INT;
  let mut mod_alt: EMACS_INT   = alt_modifier as EMACS_INT;
  let mut mod_super: EMACS_INT = super_modifier as EMACS_INT;
  let mut tem: Lisp_Object;

  tem = Fget (Vx_ctrl_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_ctrl = XFIXNUM (tem); }
  tem = Fget (Vx_alt_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_alt = XFIXNUM (tem); }
  tem = Fget (Vx_meta_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_meta = XFIXNUM (tem); }
  tem = Fget (Vx_super_keysym, Qmodifier_value ());
  if FIXNUMP (tem) { mod_super = XFIXNUM (tem); }

  return (if (state & (mod_ctrl as intmax_t))  != 0 { ANDROID_CONTROL_MASK } else { 0 })
	  | (if (state & (shift_modifier as intmax_t)) != 0 { ANDROID_SHIFT_MASK } else { 0 })
	  | (if (state & (mod_meta as intmax_t))  != 0 { ANDROID_ALT_MASK   } else { 0 })
	  | (if (state & (mod_super as intmax_t)) != 0 { ANDROID_SUPER_MASK } else { 0 })
	  | (if (state & (mod_alt as intmax_t))   != 0 { ANDROID_META_MASK  } else { 0 });
}

/* C 421: `static void android_frame_rehighlight (struct android_display_info *);` - a forward
   declaration of a `static` function defined at C 2127, past this slice.  Rust has no forward
   declarations (PORTING.md 0.1-12: a declaration plus a definition is `E0428`, and items are
   order-independent), so the line has no counterpart here; C 458's call is written as C writes it
   and `docs/ports/androidterm.md` names it as the first thing the next slice must land.
   C: static void android_frame_rehighlight (struct android_display_info *);  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_lower_frame (f: *mut frame)                        // C 423-427
{
  android_lower_window (FRAME_ANDROID_WINDOW! (f));
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_raise_frame (f: *mut frame)                        // C 429-433
{
  android_raise_window (FRAME_ANDROID_WINDOW! (f));
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_new_focus_frame (dpyinfo: *mut android_display_info, frame: *mut frame)
{                                                                    // C 435-459
  let mut old_focus: *mut frame;

  old_focus = (*dpyinfo).focus_frame;

  if frame != (*dpyinfo).focus_frame
    {
      /* Set this before calling other routines, so that they see
	 the correct value of x_focus_frame.  */
      (*dpyinfo).focus_frame = frame;

      if !old_focus.is_null () && auto_lower! (old_focus)
	{ android_lower_frame (old_focus); }

      if !(*dpyinfo).focus_frame.is_null () && auto_raise! ((*dpyinfo).focus_frame)
	{ (*dpyinfo).pending_autoraise_frame = (*dpyinfo).focus_frame; }
      else
	{ (*dpyinfo).pending_autoraise_frame = std::ptr::null_mut (); }
    }

  android_frame_rehighlight (dpyinfo);
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_focus_changed (r#type: c_int, state: c_int,
                                 dpyinfo: *mut android_display_info,
                                 frame: *mut frame, bufp: *mut input_event)   // C 461-494
{
  if r#type == ANDROID_FOCUS_IN
    {
      if (*dpyinfo).x_focus_event_frame != frame
        {
          android_new_focus_frame (dpyinfo, frame);
          (*dpyinfo).x_focus_event_frame = frame;
          input_event_bits! (kind, bufp, FOCUS_IN_EVENT);
          XSETFRAME! ((*bufp).frame_or_window, frame);
        }

      (*((*frame).output_data.android as *mut android_output)).focus_state |= state;
    }
  else if r#type == ANDROID_FOCUS_OUT
    {
      (*((*frame).output_data.android as *mut android_output)).focus_state &= !state;

      if (*dpyinfo).x_focus_event_frame == frame
        {
          (*dpyinfo).x_focus_event_frame = std::ptr::null_mut ();
          android_new_focus_frame (dpyinfo, std::ptr::null_mut ());

          input_event_bits! (kind, bufp, FOCUS_OUT_EVENT);
          XSETFRAME! ((*bufp).frame_or_window, frame);
        }

      if pointer_invisible! (frame)
        { android_toggle_invisible_pointer (frame, false); }
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_detect_focus_change (dpyinfo: *mut android_display_info,
                                       frame: *mut frame,
                                       event: *mut android_event,
                                       bufp: *mut input_event)       // C 496-516
{
  if frame.is_null ()
    { return; }

  /* C 502's `switch (event->type)` with a shared `case`, an empty `default` and a `break` becomes
     the `match` below (PORTING.md 2.2); Rust has no `switch`/`case`/`default`/`break` here.  */
  match (*event).r#type
    {
    ANDROID_FOCUS_IN | ANDROID_FOCUS_OUT =>
      android_focus_changed ((*event).r#type, FOCUS_EXPLICIT, dpyinfo, frame, bufp),

    _ => {}
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_note_mouse_movement (frame: *mut frame,
                                       event: *mut android_motion_event) -> bool   // C 518-549
{
  let mut dpyinfo: *mut android_display_info;
  let mut r: *mut Emacs_Rectangle;

  if FRAME_ANDROID_OUTPUT! (frame).is_null ()
    { return false; }

  dpyinfo = FRAME_DISPLAY_INFO! (frame);
  (*dpyinfo).last_mouse_motion_frame = frame;
  (*dpyinfo).last_mouse_motion_x = (*event).x;
  (*dpyinfo).last_mouse_motion_y = (*event).y;
  (*dpyinfo).last_mouse_movement_time = (*event).time;

  /* Has the mouse moved off the glyph it was on at the last sighting?  */
  r = &raw mut (*dpyinfo).last_mouse_glyph;
  if frame != (*dpyinfo).last_mouse_glyph_frame
      || (*event).x < (*r).x || (*event).x >= (*r).x + (*r).width as c_int
      || (*event).y < (*r).y || (*event).y >= (*r).y + (*r).height as c_int
    {
      mouse_moved! (frame, true);
      note_mouse_highlight (frame, (*event).x, (*event).y);
      /* Remember which glyph we're now on.  */
      remember_mouse_glyph (frame, (*event).x, (*event).y, r);
      (*dpyinfo).last_mouse_glyph_frame = frame;
      return true;
    }

  return false;
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn mouse_or_wdesc_frame (dpyinfo: *mut android_display_info, wdesc: c_int) -> *mut frame
{                                                                    // C 551-573
  let mut lm_f: *mut frame = if gui_mouse_grabbed (dpyinfo)
    { (*dpyinfo).last_mouse_frame } else { std::ptr::null_mut () };

  if !lm_f.is_null () && !EQ (track_mouse, Qdropping ())
      && !EQ (track_mouse, Qdrag_source ())
    { return lm_f; }
  else
    {
      let mut w_f: *mut frame = android_window_to_frame (dpyinfo, wdesc as android_window);

      /* Do not return a tooltip frame.  */
      if w_f.is_null () || FRAME_TOOLTIP_P (w_f)
	{ return if EQ (track_mouse, Qdropping ()) { lm_f } else { std::ptr::null_mut () }; }
      else
	/* When dropping it would be probably nice to raise w_f
	   here.  */
	{ return w_f; }
    }
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_construct_mouse_click (result: *mut input_event,
                                         event: *mut android_button_event,
                                         f: *mut frame) -> Lisp_Object   // C 575-602
{
  let mut dpyinfo: *mut android_display_info;
  let mut x: c_int;
  let mut y: c_int;

  dpyinfo = FRAME_DISPLAY_INFO! (f);
  x = (*event).x;
  y = (*event).y;

  /* Make the event type NO_EVENT; we'll change that when we decide
     otherwise.  */
  input_event_bits! (kind, result, MOUSE_CLICK_EVENT);
  (*result).code = (*event).button - 1;
  (*result).timestamp = (*event).time;
  (*result).modifiers = (android_android_to_emacs_modifiers (dpyinfo,
							     (*event).state as c_int)
			  | (if (*event).r#type == ANDROID_BUTTON_RELEASE
			     { up_modifier } else { down_modifier })) as c_uint;

  XSETINT! ((*result).x, x as EMACS_INT);
  XSETINT! ((*result).y, y as EMACS_INT);
  XSETFRAME! ((*result).frame_or_window, f);
  (*result).arg = Qnil ();
  return Qnil ();
}

/* Generate a TOUCHSCREEN_UPDATE_EVENT for all pressed tools in FRAME.
   Return the event in IE.  Do not set IE->timestamp, as that is left
   to the caller.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_update_tools (f: *mut frame, ie: *mut input_event)   // C 608-631
{
  let mut touchpoint: *mut android_touch_point;

  input_event_bits! (kind, ie, TOUCHSCREEN_UPDATE_EVENT);
  XSETFRAME! ((*ie).frame_or_window, f);
  (*ie).arg = Qnil ();

  /* Build the list of active touches.  */
  touchpoint = (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).touch_points;
  while !touchpoint.is_null ()
    {
      /* C's `continue` jumps to the `for` increment; the port's labelled block puts that
	 increment after the block, so it still runs (frame.rs 2836-2845).  */
      'continue_: {
	/* Skip touch points which originated on the tool bar.  */

	if (*touchpoint).tool_bar_p
	  { break 'continue_; }

	(*ie).arg = Fcons (list3i ((*touchpoint).x as EMACS_INT,
				   (*touchpoint).y as EMACS_INT,
				   (*touchpoint).tool_id as EMACS_INT),
			   (*ie).arg);
      }

      touchpoint = (*touchpoint).next;
    }
}

/* Find and return an existing tool pressed against FRAME, identified
   by POINTER_ID.  Return NULL if no tool by that ID was found.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_find_tool (f: *mut frame, pointer_id: c_int) -> *mut android_touch_point
{                                                                    // C 636-649
  let mut touchpoint: *mut android_touch_point;

  touchpoint = (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).touch_points;
  while !touchpoint.is_null ()
    {
      if (*touchpoint).tool_id == pointer_id
	{ return touchpoint; }

      touchpoint = (*touchpoint).next;
    }

  return std::ptr::null_mut ();
}

/* Decode STRING, an array of N little endian UTF-16 characters, into
   a Lisp string.  Return Qnil if the string is too large, and the
   encoded string otherwise.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_decode_utf16 (utf16: *mut c_ushort, n: size_t) -> Lisp_Object   // C 655-675
{
  let mut coding: coding_system = std::mem::zeroed ();   // C 660 (PORTING.md 1.5)
  let mut size: ptrdiff_t = 0;                           // C 661 (PORTING.md 1.5)

  /* C 662: `if (ckd_mul (&size, n, sizeof *utf16)) return Qnil;` - the P-15
     `ckd_mul`/`overflowing_mul` pair (docs/ports/token-aliases.tsv 13).  */
  let (size_, mul_overflow) = (n as ptrdiff_t)
    .overflowing_mul (std::mem::size_of::<c_ushort> () as ptrdiff_t);
  size = size_;
  if mul_overflow
    { return Qnil (); }

  /* Set up the coding system.  Decoding a UTF-16 string (with no BOM)
     should not signal.  */

  memset (&raw mut coding as *mut c_void, 0, std::mem::size_of::<coding_system> ());

  setup_coding_system (Qutf_16le (), &raw mut coding);
  coding.source = utf16 as *const u8;
  decode_coding_object (&raw mut coding, Qnil (), 0, 0, size,
			size, Qt ());

  return coding.dst_object;
}

/* Handle a cursor update request for F from the input method.
   MODE specifies whether or not an update should be sent immediately,
   and whether or not they are needed in the future.

   If MODE & ANDROID_CURSOR_UPDATE_IMMEDIATE, report the position of
   F's old selected window's phys cursor now.

   If MODE & ANDROID_CURSOR_UPDATE_MONITOR, set
   `need_cursor_updates'.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_request_cursor_updates (f: *mut frame, mode: c_int)   // C 687-708
{
  let mut w: *mut window;

  if (mode & ANDROID_CURSOR_UPDATE_IMMEDIATE) != 0
      && WINDOWP (if WINDOW_LIVE_P ((*f).old_selected_window)
		  { (*f).old_selected_window } else { (*f).selected_window })
    {
      /* Prefer the old selected window, as its selection is what was
	 reported to the IME previously.  */

      w = XWINDOW (if WINDOW_LIVE_P ((*f).old_selected_window)
		   { (*f).old_selected_window } else { (*f).selected_window });
      android_set_preeditarea (w as *mut c_void, (*w).cursor.x, (*w).cursor.y);
    }

  /* C 707-708: `FRAME_OUTPUT_DATA (f)->need_cursor_updates
     = (mode & ANDROID_CURSOR_UPDATE_MONITOR);` - the folded bit's value write (the member is one
     bit wide, so C's int expression lands as 0/1; PORTING.md P-80 spells it as the two arms).  */
  if (mode & ANDROID_CURSOR_UPDATE_MONITOR) != 0
    { (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).bits |= ANDROID_OUTPUT_NEED_CURSOR_UPDATES; }
  else
    { (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).bits &= !ANDROID_OUTPUT_NEED_CURSOR_UPDATES; }
}

/* Handle a single input method event EVENT, delivered to the frame
   F.

   Perform the text conversion action specified inside.  */

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_handle_ime_event (event: *mut android_event, f: *mut frame)   // C 716-829
{
  let mut text: Lisp_Object = Qnil ();   // C 718: `Lisp_Object text UNINIT;` (PORTING.md 1.5)
  let mut output: *mut android_output;

  /* First, decode the text if necessary.  */

  /* C 722's `switch (event->ime.operation)`: three `case`s share one body and a `break`, and the
     `default` is empty (PORTING.md 2.2); the `match` below has no `switch`/`case`/`break`.  */
  match (*event).ime.operation
    {
    ANDROID_IME_COMMIT_TEXT
    | ANDROID_IME_SET_COMPOSING_TEXT
    | ANDROID_IME_REPLACE_TEXT =>
      {
	text = android_decode_utf16 ((*event).ime.text,
				     (*event).ime.length);
	xfree ((*event).ime.text as *mut c_void);

	/* Return should text be long enough that it overflows ptrdiff_t.
	   Such circumstances are detected within android_decode_utf16.  */

	if NILP (text)
	  { return; }
      }

    _ => {}
    }

  /* Finally, perform the appropriate conversion action.  */

  /* C 744's second `switch (event->ime.operation)`: one `case` per operation, each with a
     `break` (PORTING.md 2.2).  */
  match (*event).ime.operation
    {
    ANDROID_IME_COMMIT_TEXT =>
      {
	commit_text (f, text, (*event).ime.position,
		     (*event).ime.counter);
      }

    ANDROID_IME_DELETE_SURROUNDING_TEXT =>
      {
	delete_surrounding_text (f, (*event).ime.start,
				 (*event).ime.end,
				 (*event).ime.counter);
      }

    ANDROID_IME_FINISH_COMPOSING_TEXT =>
      {
	if (*event).ime.length == 2
	  {
	    output = FRAME_ANDROID_OUTPUT! (f) as *mut android_output;

	    /* A new input method has connected to Emacs.  Stop
	       reporting changes that the previous input method has
	       asked to monitor.  */

	    (*output).extracted_text_flags = 0;
	    (*output).extracted_text_token = 0;
	    (*output).extracted_text_hint = 0;
	    (*output).bits &= !ANDROID_OUTPUT_NEED_CURSOR_UPDATES;   /* C: `output->need_cursor_updates = false;` (P-80 mask) */
	  }

	finish_composing_text (f, (*event).ime.counter,
			       (*event).ime.length == 1);

	if (*event).ime.length == 2
	  {
	    /* Now cancel outstanding batch edits if a new input method
	       has connected.  */

	    (*f).conversion.batch_edit_flags = 0;
	    (*f).conversion.batch_edit_count = 0;
	  }
      }

    ANDROID_IME_SET_COMPOSING_TEXT =>
      {
	set_composing_text (f, text, (*event).ime.position,
			    (*event).ime.counter);
      }

    ANDROID_IME_SET_COMPOSING_REGION =>
      {
	set_composing_region (f, (*event).ime.start,
			      (*event).ime.end,
			      (*event).ime.counter);
      }

    ANDROID_IME_SET_POINT =>
      {
	textconv_set_point_and_mark (f, (*event).ime.start,
				     (*event).ime.end,
				     (*event).ime.counter);
      }

    ANDROID_IME_START_BATCH_EDIT =>
      { start_batch_edit (f, (*event).ime.counter); }

    ANDROID_IME_END_BATCH_EDIT =>
      { end_batch_edit (f, (*event).ime.counter); }

    ANDROID_IME_REQUEST_SELECTION_UPDATE =>
      { request_point_update (f, (*event).ime.counter); }

    ANDROID_IME_REQUEST_CURSOR_UPDATES =>
      { android_request_cursor_updates (f, (*event).ime.length as c_int); }

    ANDROID_IME_REPLACE_TEXT =>
      {
	replace_text (f, (*event).ime.start, (*event).ime.end,
		      text, (*event).ime.position,
		      (*event).ime.counter);
      }

    _ => {}
    }
}

/* C 831-834: C 831-832's form feed and blank, C 833's `/* Forward declaration.  */` and C 834's
   `static void android_notify_conversion (unsigned long);`.  Rust needs no forward declaration for
   a definition later in the same unit (PORTING.md 10.10), so this slice carries the comment and no
   item - which leaves the call at C 1761 an `E0425` until C 6635-6648's definition lands (its own
   address is taken by C 6657's `textconv_interface` array): the named
   blocker class the ledger already records for C 458's `android_frame_rehighlight`.  */

/* `Mouse_HLInfo`'s third flag bit (`dispextern.h` 3023's `bool_bf mouse_face_hidden : 1`, mask
   `include/dispextern.rs` 2139's `MOUSE_FACE_HIDDEN`).  The mirror carries the mask and the
   `mouse_face_defer` pair, and no accessor for this bit; C 959/967/1170/1172 read and write the
   member, so the unit names the pair - the `src/xterm.rs` 31036-31046 copy (whose own note points
   at `src/haikuterm.rs` 4884-4893 and `src/pgtkterm.rs` for the same macro).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! mouse_face_hidden {
  ($h:expr) => { (*$h).bits & MOUSE_FACE_HIDDEN != 0 };
  ($h:expr, $v:expr) => {
    (*$h).bits = if ($v) as c_int != 0 { (*$h).bits | MOUSE_FACE_HIDDEN }
		 else { (*$h).bits & ! MOUSE_FACE_HIDDEN } };
}

/* The eleven names of this slice's body that no mirror in this unit's include graph carries.  Each
   stands at its use site with C's prototype in a `/// C:` line (P-72 rule 4/P-88 ①), the
   `android_seq_filter`-style device slice 1 used for its own eight.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: frame.h 1515-1516: `extern void frame_size_history_extra (struct frame *, Lisp_Object,
  /// int, int, int, int, int, int);`
  pub fn frame_size_history_extra (f: *mut frame, parameter: Lisp_Object,
				   a: c_int, b: c_int, c: c_int, d: c_int,
				   e: c_int, g: c_int);
  /// C: frame.h 1911: `extern bool frame_ancestor_p (struct frame *af, struct frame *df);`
  pub fn frame_ancestor_p (a: *mut frame, b: *mut frame) -> bool;
  /// C: dispextern.h 3675: `extern void expose_frame (struct frame *, int, int, int, int);`
  pub fn expose_frame (f: *mut frame, x: c_int, y: c_int, w: c_int, h: c_int);
  /// C: dispextern.h 3668-3669: `extern Lisp_Object handle_tab_bar_click (struct frame *,
  /// int, int, bool, int);`
  pub fn handle_tab_bar_click (f: *mut frame, x: c_int, y: c_int, down_p: bool,
			       modifiers: c_int) -> Lisp_Object;
  /// C: dispextern.h 3670-3671: `extern void handle_tool_bar_click (struct frame *,
  /// int, int, bool, int);`
  pub fn handle_tool_bar_click (f: *mut frame, x: c_int, y: c_int, down_p: bool,
				modifiers: c_int);
  /// C: dispextern.h 3688: `extern void note_mouse_highlight (struct frame *, int, int);`
  /// (slice 1 carries this one; repeated here for this slice's C 1491/1575 calls.)
  /// C: dispextern.h 3690: `extern void cancel_mouse_face (struct frame *);`
  pub fn cancel_mouse_face (f: *mut frame);
  /// C: dispextern.h 3691: `extern bool clear_mouse_face (Mouse_HLInfo *);`
  pub fn clear_mouse_face (hlinfo: *mut Mouse_HLInfo) -> bool;
  /// C: dispextern.h 3924: `void change_frame_size (struct frame *, int, int, bool, bool, bool);`
  pub fn change_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
			    pretend: bool, delay: bool, safe: bool);
  /// C: keyboard.h 513-514: `extern void gen_help_event (Lisp_Object, Lisp_Object, Lisp_Object,
  /// Lisp_Object, ptrdiff_t);`
  pub fn gen_help_event (help: Lisp_Object, frame: Lisp_Object, window: Lisp_Object,
			 object: Lisp_Object, pos: ptrdiff_t);
  /// C: coding.h 737: `extern Lisp_Object from_unicode_buffer (const wchar_t *wstr);`
  pub fn from_unicode_buffer (wstr: *const wchar_t) -> Lisp_Object;
  /// C: `<math.h>`: `double fabs (double);`
  pub fn fabs (x: c_double) -> c_double;
}

/* dispextern.h 3614-3616's five objects (`keyboard.c` defines all five), which the mirror declares
   inside a module no glob import brings into this unit - the `src/xterm.rs` 31179-31192 device.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3614: `extern Lisp_Object help_echo_string, help_echo_window;`
  pub static mut help_echo_string: Lisp_Object;
  pub static mut help_echo_window: Lisp_Object;
  /// C: dispextern.h 3615: `extern Lisp_Object help_echo_object, previous_help_echo_string;`
  pub static mut help_echo_object: Lisp_Object;
  pub static mut previous_help_echo_string: Lisp_Object;
  /// C: dispextern.h 3616: `extern ptrdiff_t help_echo_pos;`
  pub static mut help_echo_pos: ptrdiff_t;
}

/* C 836-848's declarations and C 836-1892's `handle_one_android_event`, the file's largest
   function.  C takes no address of it, so it stays a Rust-ABI `unsafe fn` (PORTING.md 1.4); it is
   `static` in C, hence no `no_mangle`.  C's `switch (event->type)` (C 870) is a `match`, and C's
   `OTHER:` label (C 1860) - which is also where every `break` out of that switch lands, since the
   label's statement immediately follows it - is the labelled block `'other`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn handle_one_android_event (dpyinfo: *mut android_display_info,
				    event: *mut android_event, finish: *mut c_int,
				    hold_quit: *mut input_event) -> c_int
{
  /* C leaves every one of these uninitialised (PORTING.md 1.5): each is assigned before any path
     that reads it.  */
  let mut configureEvent: android_event = std::mem::zeroed ();          // C 841
  let mut f: *mut frame = std::ptr::null_mut ();                        // C 842
  let mut any: *mut frame = std::ptr::null_mut ();
  let mut mouse_frame: *mut frame = std::ptr::null_mut ();
  let mut hlinfo: *mut Mouse_HLInfo = std::ptr::null_mut ();            // C 843
  let mut inev: buffered_input_event = std::mem::zeroed ();             // C 844
  let mut modifiers: c_int = 0;                                         // C 845
  let mut count: c_int = 0;
  let mut do_help: c_int = 0;
  let mut touchpoint: *mut android_touch_point = std::ptr::null_mut (); // C 846
  let mut last: *mut *mut android_touch_point = std::ptr::null_mut ();
  let mut window: Lisp_Object = Qnil ();                                // C 847
  let mut scroll_height: c_int = 0;                                     // C 848
  let mut scroll_unit: f64 = 0.0;                                       // C 849
  let mut keysym: c_int = 0;                                            // C 850
  let mut nchars: ptrdiff_t = 0;                                        // C 851
  let mut i: ptrdiff_t = 0;
  let mut w: *mut window = std::ptr::null_mut ();                       // C 852

  /* C 853's `static struct android_compose_status compose_status;` is a *function-local* static:
     one object for the life of the process, zero-initialised, and the port keeps C's own storage
     class by writing a function-local `static mut` (PORTING.md 1.6).  */
  static mut compose_status: android_compose_status = android_compose_status { accent: 0, chars_matched: 0 };

  /* It is okay for this to not resemble handle_one_xevent so much.
     Differences in event handling code are much less nasty than
     stuble differences in the graphics code.  */                       // C 855-857

  count = 0;                                                           // C 859 (C's chained
  do_help = count;                                                     //  `do_help = count = 0;`)
  hlinfo = &raw mut (*dpyinfo).mouse_highlight;                         // C 860
  *finish = ANDROID_EVENT_NORMAL;                                       // C 861
  any = android_window_to_frame (dpyinfo, (*event).xany.window);        // C 862
  nchars = 0;                                                           // C 863

  if ! any.is_null () && (*any).wait_event_type == (*event).r#type     // C 865
    { (*any).wait_event_type = 0; }                                    // C 866
									//  (Indicates we got it.)

  EVENT_INIT (&raw mut inev.ie);                                        // C 868

  'other: {
  match (*event).r#type                                                // C 870
    {
    ANDROID_CONFIGURE_NOTIFY =>
      {
	configureEvent = *event;                                       // C 873

	f = android_window_to_frame (dpyinfo,                           // C 875
				     configureEvent.xconfigure.window);   // C 876

	if f.is_null ()                                                // C 878
	  { break 'other; }                                            // C 879 (C's `goto OTHER`)

	if FRAME_TOOLTIP_P (f)                                         // C 881
	  {
	    if FRAME_PIXEL_HEIGHT (f) != configureEvent.xconfigure.height   // C 883
		|| FRAME_PIXEL_WIDTH (f) != configureEvent.xconfigure.width     // C 884
	      { SET_FRAME_GARBAGED! (f); }                              // C 885

	    /* C: `FRAME_PIXEL_HEIGHT (f) = …;`/`FRAME_PIXEL_WIDTH (f) = …;` - the mirror's two
	       accessors are `fn`s (frame.rs 1036/1038), so C's l-value use is the field write
	       (`src/xterm.rs` 34289-34290 is the same pair of statements).  */
	    (*f).pixel_height = configureEvent.xconfigure.height;      // C 887
	    (*f).pixel_width = configureEvent.xconfigure.width;        // C 888
	  }

	let width: c_int = configureEvent.xconfigure.width;            // C 891
	let height: c_int = configureEvent.xconfigure.height;          // C 892

	if CONSP (frame_size_history)                                 // C 894
	  { frame_size_history_extra (f, build_string (c"ConfigureNotify".as_ptr ()),   // C 895
				      FRAME_PIXEL_WIDTH (f),           // C 896
				      FRAME_PIXEL_HEIGHT (f),          // C 897
				      width, height, (*f).new_width,   // C 898
				      (*f).new_height); }              // C 899

	/* Even if the number of character rows and columns has
	   not changed, the font size may have changed, so we need
	   to check the pixel dimensions as well.  */                  // C 901-903

	if width != FRAME_PIXEL_WIDTH (f)                              // C 905
	    || height != FRAME_PIXEL_HEIGHT (f)                        // C 906
	    || ({ let new_size_p: bool = frame_new_size_p (f); new_size_p }   // C 907: `f->new_size_p`
		&& (((*f).new_width >= 0 && width != (*f).new_width)   // C 908
		    || ((*f).new_height >= 0 && height != (*f).new_height)))   // C 909
	  {
	    change_frame_size (f, width, height, false, true, false);  // C 911
	    android_clear_under_internal_border (f);                   // C 912
	    SET_FRAME_GARBAGED! (f);                                    // C 913
	    cancel_mouse_face (f);                                     // C 914
	  }

	/* Now change the left and top position of this window.  */     // C 917

	{
	  let old_left: c_int = (*f).left_pos;                         // C 920
	  let old_top: c_int = (*f).top_pos;                           // C 921
	  let mut frame: Lisp_Object = Qnil ();                        // C 922

	  XSETFRAME! (frame, f);                                       // C 924

	  {
	    let mut root: android_window = 0;                          // C 927
	    let mut dummy_uint: c_uint = 0;                            // C 928

	    android_get_geometry (FRAME_ANDROID_WINDOW! (f),           // C 930
				  &raw mut root, &raw mut (*f).left_pos, &raw mut (*f).top_pos,   // C 931
				  &raw mut dummy_uint, &raw mut dummy_uint,   // C 932
				  &raw mut dummy_uint);                        // C 933
	  }

	  if ! FRAME_TOOLTIP_P (f)                                     // C 936
	      && (old_left != (*f).left_pos || old_top != (*f).top_pos)   // C 937
	    {
	      input_event_bits! (kind, &raw mut inev.ie, MOVE_FRAME_EVENT);   // C 939
	      XSETFRAME! (inev.ie.frame_or_window, f);                 // C 940
	    }

	  if ! f.is_null ()                                            // C 943
	      && (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).bits
	      & ANDROID_OUTPUT_NEED_CURSOR_UPDATES != 0              // C 943
	    {
	      w = XWINDOW ((*f).selected_window);                      // C 945
	      android_set_preeditarea (w as *mut c_void, (*w).cursor.x, (*w).cursor.y);   // C 946
	    }
	}

	break 'other;                                                  // C 950 (C's `goto OTHER`)
      }

    ANDROID_KEY_PRESS =>
      {
	/* Set f to any.  There are no ``outer windows'' on Android.  */   // C 954
	f = any;                                                       // C 955

	/* If mouse-highlight is an integer, input clears out
	   mouse highlighting.  */                                     // C 957-958
	if ! mouse_face_hidden! (hlinfo) && FIXNUMP (Vmouse_highlight)   // C 959
	    && (any.is_null ()                                         // C 960
		|| (! EQ ((*any).tool_bar_window, (*hlinfo).mouse_face_window)   // C 961
		    && ! EQ ((*any).tab_bar_window, (*hlinfo).mouse_face_window)))   // C 962
	  {
	    mouse_frame = (*hlinfo).mouse_face_mouse_frame as *mut frame;   // C 964

	    clear_mouse_face (hlinfo);                                 // C 966
	    mouse_face_hidden! (hlinfo, true);                       // C 967

	    if ! mouse_frame.is_null ()                                // C 969
	      { android_flush_dirty_back_buffer_on (mouse_frame); }    // C 970
	  }

	if f.is_null ()                                                // C 973
	  { break 'other; }                                            // C 974 (C's `goto OTHER`)

	if (*event).xkey.counter > 0                                   // C 976
	  /* This event was generated by `performEditorAction'.  Make
	     sure it is processed before any subsequent edits.  */     // C 977-978
	  { textconv_barrier (f, (*event).xkey.counter); }             // C 979

	let mut copy_buffer: [wchar_t; 512] = [0; 512];                // C 981
	let mut copy_bufptr: *mut wchar_t = copy_buffer.as_mut_ptr ();  // C 982
	let copy_bufsiz: c_int = 512;                                  // C 983

	(*event).xkey.state                                            // C 985
	  |= android_emacs_to_android_modifiers (dpyinfo,              // C 986
						 extra_keyboard_modifiers as intmax_t) as c_uint;   // C 987
	modifiers = (*event).xkey.state as c_int;                               // C 988

	/* In case Meta is ComposeCharacter, clear its status.  According
	   to Markus Ehrnsperger
	   Markus.Ehrnsperger@lehrstuhl-bross.physik.uni-muenchen.de this
	   enables ComposeCharacter to work whether or not it is combined
	   with Meta.  */                                              // C 990-994
	if modifiers & ANDROID_ALT_MASK != 0                           // C 995
	  { memset (&raw mut compose_status as *mut c_void, 0, std::mem::size_of::<android_compose_status> ()); }   // C 996

	/* Common for all keysym input events.  */                     // C 998
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 999
	inev.ie.modifiers                                              // C 1000
	  = android_android_to_emacs_modifiers (dpyinfo, modifiers) as c_uint;   // C 1001
	inev.ie.timestamp = (*event).xkey.time as u64;                 // C 1002

	keysym = (*event).xkey.keycode as c_int;                                // C 1004

	/* C 1110's `done_keysym:` label sits after C 1108's `goto done_keysym`, and every jump to it
	   starts inside the C 1006-1050 block, so the port's labelled block opens here and its tail
	   (the label's own statements, C 1115-1121) follows the block.  */
	'done_keysym: {
	  {
	    let mut status_return: android_lookup_status = 0;          // C 1007

	    nchars = android_wc_lookup_string (&raw mut (*event).xkey, copy_bufptr,   // C 1009
					       copy_bufsiz, &raw mut keysym,   // C 1010
					       &raw mut status_return,    // C 1011
					       &raw mut compose_status) as ptrdiff_t;  // C 1012

	    /* android_lookup_string can't be called twice, so there's no
	       way to recover from buffer overflow.  */                // C 1014-1015
	    if status_return == ANDROID_BUFFER_OVERFLOW                // C 1016
	      { break 'done_keysym; }                                  // C 1017 (C's `goto done_keysym`)
	    else if status_return == ANDROID_LOOKUP_NONE               // C 1018
	      {
		/* Don't skip preedit text events.  */                 // C 1020
		if (*event).xkey.keycode != u32::MAX                   // C 1021 (C's `(uint32_t) -1`)
		  { break 'done_keysym; }                              // C 1022
	      }
	    else if status_return == ANDROID_LOOKUP_CHARS              // C 1024
	      { keysym = ANDROID_NO_SYMBOL; }                          // C 1025
	    else if status_return != ANDROID_LOOKUP_KEYSYM             // C 1026
		     && status_return != ANDROID_LOOKUP_BOTH           // C 1027
	      { emacs_abort (); }                                      // C 1028

	    /* Deal with pre-edit text events.  On Android, these are
	       simply encoded as events with associated strings and a
	       keycode set to ``-1''.  */                             // C 1030-1032

	    if (*event).xkey.keycode == u32::MAX                       // C 1034 (C's `(uint32_t) -1`)
	      {
		input_event_bits! (kind, &raw mut inev.ie, PREEDIT_TEXT_EVENT);   // C 1036
		inev.ie.arg = Qnil ();                                 // C 1037

		/* If text was looked up, decode it and make it the
		   preedit text.  */                                   // C 1039-1040

		if status_return == ANDROID_LOOKUP_CHARS && nchars != 0   // C 1042
		  {
		    *copy_bufptr.offset (nchars) = 0;                   // C 1044
		    inev.ie.arg = from_unicode_buffer (copy_bufptr);    // C 1045
		  }

		break 'done_keysym;                                    // C 1048
	      }
	  }

	  /* If a compose sequence is in progress, we break here.
	     Otherwise, chars_matched is always 0.  */                 // C 1052-1053
	  if compose_status.chars_matched > 0 && nchars == 0           // C 1054
	    { break 'other; }                                          // C 1055 (C's `break` out of the switch)

	  memset (&raw mut compose_status as *mut c_void, 0, std::mem::size_of::<android_compose_status> ());   // C 1057

	  if nchars == 1 && *copy_bufptr >= 32                         // C 1059
	    {
	      /* Deal with characters.  */                             // C 1061

	      if *copy_bufptr < 128                                    // C 1063
		{ input_event_bits! (kind, &raw mut inev.ie, ASCII_KEYSTROKE_EVENT); }   // C 1064
	      else
		{ input_event_bits! (kind, &raw mut inev.ie, MULTIBYTE_CHAR_KEYSTROKE_EVENT); }   // C 1066

	      inev.ie.code = *copy_bufptr as c_uint;                   // C 1068
	    }
	  else if nchars < 2 && keysym != 0                            // C 1070
	    {
	      /* If the key is a modifier key, just return.  */        // C 1072
	      if ANDROID_IS_MODIFIER_KEY! (keysym)                     // C 1073
		{ break 'done_keysym; }                                // C 1074

	      /* Next, deal with special ``characters'' by giving the
		 keycode to keyboard.c.  */                            // C 1076-1077
	      input_event_bits! (kind, &raw mut inev.ie, NON_ASCII_KEYSTROKE_EVENT);   // C 1078
	      inev.ie.code = keysym as c_uint;                         // C 1079
	    }
	  else
	    {
	      /* Finally, deal with strings.  */                       // C 1083

	      i = 0;                                                   // C 1085
	      while i < nchars                                       // C 1085
		{
		  input_event_bits! (kind, &raw mut inev.ie,           // C 1087
				     if SINGLE_BYTE_CHAR_P (*copy_bufptr.offset (i) as intmax_t)   // C 1087
				       { ASCII_KEYSTROKE_EVENT }       // C 1088
				     else { MULTIBYTE_CHAR_KEYSTROKE_EVENT });   // C 1089
		  inev.ie.code = *copy_bufptr.offset (i) as c_uint;    // C 1090

		  /* If the character is actually '\n', then change this
		     to RET.  */                                       // C 1092-1093

		  if *copy_bufptr.offset (i) == b'\n' as wchar_t       // C 1095
		    {
		      input_event_bits! (kind, &raw mut inev.ie, NON_ASCII_KEYSTROKE_EVENT);   // C 1097
		      inev.ie.code = 66;                               // C 1098
		    }

		  kbd_buffer_store_buffered_event (&raw mut inev, hold_quit);   // C 1101
		  i += 1;                                              // C 1085 (the `for` step)
		}

	      count += nchars as c_int;                                // C 1104
	      input_event_bits! (kind, &raw mut inev.ie, NO_EVENT);    // C 1105 (Already stored above.)
	    }

	  break 'done_keysym;                                          // C 1108 (C's `goto done_keysym`)
	}

	/* Now proceed to tell the input method the current position of
	   the cursor, if required.  */                               // C 1112-1113

	if ! f.is_null ()                                              // C 1115
	    && (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).bits
	      & ANDROID_OUTPUT_NEED_CURSOR_UPDATES != 0              // C 1115
	  {
	    w = XWINDOW ((*f).selected_window);                        // C 1117
	    android_set_preeditarea (w as *mut c_void, (*w).cursor.x, (*w).cursor.y);   // C 1118
	  }

	break 'other;                                                  // C 1121
      }

    ANDROID_FOCUS_IN | ANDROID_FOCUS_OUT =>
      {
	android_detect_focus_change (dpyinfo, any, event, &raw mut inev.ie);   // C 1125
	break 'other;                                                  // C 1126
      }

    /* C's `case ANDROID_WINDOW_ACTION` (C 1128) falls through into `case ANDROID_ENTER_NOTIFY`
       (C 1158) - its block has no `break` - so the two arms share one `match` arm and both bodies
       stand in C's order.  */                                        // C 1158
    ANDROID_WINDOW_ACTION | ANDROID_ENTER_NOTIFY =>
      {
	/* This is a special event sent by android_run_in_emacs_thread
	   used to make Android run stuff.  */                        // C 1130-1131

	if (*event).xaction.window == 0 && (*event).xaction.action == 0   // C 1133
	  /* Don't run queries here, as it may run inside editor
	     commands, which can expose an inconsistent view of buffer
	     contents to the input method during command execution.

	     Instead, wait for Emacs to return to `android_select'.  */   // C 1134-1138
	  { break 'other; }                                            // C 1139

	f = any;                                                       // C 1141

	if (*event).xaction.action == 0                                // C 1143
	  {
	    /* Action 0 either means that a window has been destroyed
	       and its associated frame should be as well.  */         // C 1145-1146

	    if (*event).xaction.window != 0                            // C 1148
	      {
		if f.is_null ()                                        // C 1150
		  { break 'other; }                                    // C 1151

		input_event_bits! (kind, &raw mut inev.ie, DELETE_WINDOW_EVENT);   // C 1153
		XSETFRAME! (inev.ie.frame_or_window, f);               // C 1154
	      }
	  }

	/* (C 1158's `case ANDROID_ENTER_NOTIFY:`, which control reaches by falling through.)  */
	f = any;                                                       // C 1159

	if ! f.is_null ()                                              // C 1161
	  { android_note_mouse_movement (f, &raw mut (*event).xmotion); }   // C 1162
	break 'other;                                                  // C 1163
      }

    ANDROID_MOTION_NOTIFY =>
      {
	previous_help_echo_string = help_echo_string;                  // C 1167
	help_echo_string = Qnil ();                                    // C 1168

	if mouse_face_hidden! (hlinfo)                                 // C 1170
	  {
	    mouse_face_hidden! (hlinfo, false);                       // C 1172
	    clear_mouse_face (hlinfo);                                 // C 1173
	  }

	f = any;                                                       // C 1176

	if ! f.is_null ()                                              // C 1178
	  {
	    /* Maybe generate a SELECT_WINDOW_EVENT for
	       `mouse-autoselect-window' but don't let popup menus
	       interfere with this (Bug#1261).  */                     // C 1180-1182
	    if ! NILP (Vmouse_autoselect_window)                       // C 1183
		&& popup_activated () == 0                                // C 1184
		/* Don't switch if we're currently in the minibuffer.
		   This tries to work around problems where the
		   minibuffer gets unselected unexpectedly, and where
		   you then have to move your mouse all the way down to
		   the minibuffer to select it.  */                    // C 1185-1189
		&& ! MINI_WINDOW_P (XWINDOW (selected_window))         // C 1190
		/* With `focus-follows-mouse' non-nil create an event
		   also when the target window is on another frame.  */   // C 1191-1192
		&& (f == XFRAME (selected_frame)                       // C 1193
		    || ! NILP (focus_follows_mouse))                   // C 1194
	      {
		let window: Lisp_Object                                // C 1196
		  = window_from_coordinates (f as *mut c_void, (*event).xmotion.x,    // C 1197
					     (*event).xmotion.y, std::ptr::null_mut (),    // C 1198
					     false, false, false);     // C 1199

		/* A window will be autoselected only when it is not
		   selected now and the last mouse movement event was
		   not in it.  The remainder of the code is a bit vague
		   wrt what a "window" is.  For immediate autoselection,
		   the window is usually the entire window but for GTK
		   where the scroll bars don't count.  For delayed
		   autoselection the window is usually the window's text
		   area including the margins.  */                     // C 1201-1208
		if WINDOWP (window)                                    // C 1209
		    && ! EQ (window, last_mouse_window)                // C 1210
		    && ! EQ (window, selected_window)                  // C 1211
		  {
		    input_event_bits! (kind, &raw mut inev.ie, SELECT_WINDOW_EVENT);   // C 1213
		    inev.ie.frame_or_window = window;                  // C 1214
		  }

		/* Remember the last window where we saw the mouse.  */   // C 1217
		last_mouse_window = window;                            // C 1218
	      }

	    if ! android_note_mouse_movement (f, &raw mut (*event).xmotion)   // C 1221
	      { help_echo_string = previous_help_echo_string; }        // C 1222
	  }

	/* If the contents of the global variable help_echo_string
	   has changed, generate a HELP_EVENT.  */                     // C 1225-1226
	if ! NILP (help_echo_string)                                   // C 1227
	    || ! NILP (previous_help_echo_string)                      // C 1228
	  { do_help = 1; }                                             // C 1229

	if ! f.is_null ()                                              // C 1231
	  { android_flush_dirty_back_buffer_on (f); }                  // C 1232

	break 'other;                                                  // C 1234
      }

    ANDROID_LEAVE_NOTIFY =>
      {
	f = any;                                                       // C 1237

	if ! f.is_null ()                                              // C 1239
	  {
	    /* Now clear dpyinfo->last_mouse_motion_frame, or
	       gui_redo_mouse_highlight will end up highlighting the
	       last known position of the mouse if a tooltip frame is
	       later unmapped.  */                                     // C 1241-1244

	    if f == (*dpyinfo).last_mouse_motion_frame                 // C 1246
	      { (*dpyinfo).last_mouse_motion_frame = std::ptr::null_mut (); }   // C 1247

	    /* Something similar applies to
	       dpyinfo->last_mouse_glyph_frame.  */                    // C 1249-1250
	    if f == (*dpyinfo).last_mouse_glyph_frame                  // C 1251
	      { (*dpyinfo).last_mouse_glyph_frame = std::ptr::null_mut (); }   // C 1252

	    if f == (*hlinfo).mouse_face_mouse_frame as *mut frame          // C 1254
	      {
		/* If we move outside the frame, then we're
		   certainly no longer on any text in the frame.  */   // C 1256-1257
		clear_mouse_face (hlinfo);                             // C 1258
		(*hlinfo).mouse_face_mouse_frame = std::ptr::null_mut ();   // C 1259
		android_flush_dirty_back_buffer_on (f);                // C 1260
	      }

	    /* Generate a nil HELP_EVENT to cancel a help-echo.
	       Do it only if there's something to cancel.
	       Otherwise, the startup message is cleared when
	       the mouse leaves the frame.  */                        // C 1263-1266
	    if any_help_event_p                                        // C 1267
		/* But never if `mouse-drag-and-drop-region' is in
		   progress, since that results in the tooltip being
		   dismissed when the mouse moves on top.  */          // C 1268-1270
		&& !((EQ (track_mouse, Qdrag_source ())                 // C 1271
		      || EQ (track_mouse, Qdropping ()))               // C 1272
		     && gui_mouse_grabbed (dpyinfo))                   // C 1273
	      { do_help = -1; }                                        // C 1274
	  }

	break 'other;                                                  // C 1277
      }

    ANDROID_EXPOSE =>
      {
	f = any;                                                       // C 1281

	if ! f.is_null ()                                              // C 1283
	  {
	    if ! FRAME_VISIBLE_P (f)                                   // C 1285
	      {
		(*( (*f).output_data.android as *mut android_output )).bits |= ANDROID_OUTPUT_HAS_BEEN_VISIBLE;   // C 1287 (C's `has_been_visible = true;`)
		SET_FRAME_GARBAGED! (f);                                // C 1288
	      }

	    if ! FRAME_GARBAGED_P (f)                                  // C 1291
	      {
		expose_frame (f, (*event).xexpose.x, (*event).xexpose.y,   // C 1293
			      (*event).xexpose.width, (*event).xexpose.height);   // C 1294

		/* Do not display the back buffer if F is yet being
		   updated, as this might trigger premature bitmap
		   reconfiguration.  */                                // C 1296-1298
		if FRAME_ANDROID_COMPLETE_P! (f)                        // C 1299
		  { show_back_buffer (f); }                            // C 1300
	      }
	  }

	break 'other;                                                  // C 1304
      }

    ANDROID_BUTTON_PRESS | ANDROID_BUTTON_RELEASE =>
      {
	/* If we decide we want to generate an event to be seen
	   by the rest of Emacs, we put it here.  */                   // C 1308-1309

	f = any;                                                       // C 1311

	let mut tab_bar_arg: Lisp_Object = Qnil ();                    // C 1313
	let mut tab_bar_p: bool = false;                               // C 1314
	let mut tool_bar_p: bool = false;                              // C 1315

	(*dpyinfo).last_mouse_glyph_frame = std::ptr::null_mut ();     // C 1317

	f = mouse_or_wdesc_frame (dpyinfo, (*event).xbutton.window as c_int);   // C 1319

	if ! f.is_null ()                                              // C 1321
	    && (*event).xbutton.r#type == ANDROID_BUTTON_PRESS         // C 1321
	    && popup_activated () == 0                                // C 1322
	    /* && !x_window_to_scroll_bar (event->xbutton.display, */   // C 1323
	    /* 			      event->xbutton.window, 2) */   // C 1324
	    && FRAME_NO_ACCEPT_FOCUS (f) == 0                           // C 1325
	  {
	    /* When clicking into a child frame or when clicking
	       into a parent frame with the child frame selected and
	       `no-accept-focus' is not set, select the clicked
	       frame.  */                                              // C 1327-1330
	    let hf: *mut frame = (*dpyinfo).highlight_frame;           // C 1331

	    if ! FRAME_PARENT_FRAME (f).is_null ()                      // C 1333
		|| (! hf.is_null () && frame_ancestor_p (f, hf))        // C 1333
	      {
		android_set_input_focus (FRAME_ANDROID_WINDOW! (f),    // C 1335
					 (*event).xbutton.time);       // C 1336

		if ! FRAME_PARENT_FRAME (f).is_null ()                  // C 1338
		  { android_raise_window (FRAME_ANDROID_WINDOW! (f)); }   // C 1339
	      }
	  }

	if ! f.is_null ()                                              // C 1343
	  {
	    /* Is this in the tab-bar?  */                             // C 1345
	    if WINDOWP ((*f).tab_bar_window)                           // C 1346
		&& WINDOW_TOTAL_LINES (XWINDOW ((*f).tab_bar_window)) != 0   // C 1347
	      {
		let mut window: Lisp_Object = Qnil ();                 // C 1349
		let x: c_int = (*event).xbutton.x;                     // C 1350
		let y: c_int = (*event).xbutton.y;                     // C 1351

		window = window_from_coordinates (f as *mut c_void, x, y, std::ptr::null_mut (), true, true, true);   // C 1353
		tab_bar_p = EQ (window, (*f).tab_bar_window);          // C 1354

		if tab_bar_p                                           // C 1356
		  {
		    tab_bar_arg = handle_tab_bar_click                // C 1358
		      (f, x, y, ((*event).xbutton.r#type             // C 1359
				 == ANDROID_BUTTON_PRESS),             // C 1360
		       android_android_to_emacs_modifiers (dpyinfo,   // C 1361
							   (*event).xbutton.state as c_int));   // C 1362
		    android_flush_dirty_back_buffer_on (f);            // C 1363
		  }
	      }

	    /* Is this in the tool-bar?  */                            // C 1367
	    if WINDOWP ((*f).tool_bar_window)                          // C 1368
		&& WINDOW_TOTAL_LINES (XWINDOW ((*f).tool_bar_window)) != 0   // C 1369
	      {
		let mut window: Lisp_Object = Qnil ();                 // C 1371
		let x: c_int = (*event).xbutton.x;                     // C 1372
		let y: c_int = (*event).xbutton.y;                     // C 1373

		window = window_from_coordinates (f as *mut c_void, x, y, std::ptr::null_mut (), true, true, true);   // C 1375
		tool_bar_p = (EQ (window, (*f).tool_bar_window)        // C 1376
			      && (((*event).xbutton.r#type             // C 1377
				   != ANDROID_BUTTON_RELEASE)          // C 1378
				  || (*f).last_tool_bar_item != -1));   // C 1379

		if tool_bar_p && (*event).xbutton.button < 4           // C 1381
		  {
		    handle_tool_bar_click                              // C 1383
		      (f, x, y, ((*event).xbutton.r#type               // C 1384
				 == ANDROID_BUTTON_PRESS),             // C 1385
		       android_android_to_emacs_modifiers (dpyinfo,   // C 1386
							   (*event).xbutton.state as c_int));   // C 1387
		    android_flush_dirty_back_buffer_on (f);            // C 1388
		  }
	      }

	    if !(tab_bar_p && NILP (tab_bar_arg)) && ! tool_bar_p      // C 1392
	      {
		if popup_activated () == 0                            // C 1393
		  {
		    android_construct_mouse_click (&raw mut inev.ie, &raw mut (*event).xbutton, f);   // C 1395

		    if ! NILP (tab_bar_arg)                            // C 1397
		      { inev.ie.arg = tab_bar_arg; }                   // C 1398
		  }
	      }
	  }

	if (*event).r#type == ANDROID_BUTTON_PRESS                     // C 1402
	  {
	    (*dpyinfo).grabbed |= 1 << (*event).xbutton.button;        // C 1404
	    (*dpyinfo).last_mouse_frame = f;                           // C 1405
	    if ! f.is_null () && ! tab_bar_p                           // C 1406
	      { (*f).last_tab_bar_item = -1; }                         // C 1407
	    if ! f.is_null () && ! tool_bar_p                          // C 1408
	      { (*f).last_tool_bar_item = -1; }                        // C 1409
	  }
	else
	  { (*dpyinfo).grabbed &= ! (1 << (*event).xbutton.button); }   // C 1412

	/* Ignore any mouse motion that happened before this event;
	   any subsequent mouse-movement Emacs events should reflect
	   only motion after the ButtonPress/Release.  */              // C 1414-1416
	if ! f.is_null ()                                              // C 1417
	  { mouse_moved! (f, false); }                                 // C 1418

	break 'other;                                                  // C 1420
      }

    /* Touch events.  The events here don't parallel X so much.  */   // C 1422
    ANDROID_TOUCH_DOWN =>
      {
	if any.is_null ()                                              // C 1425
	  { break 'other; }                                            // C 1426

	/* This event is sent when a tool is put on the screen.  X and Y
	   are the location of the finger, and pointer_id identifies the
	   tool for as long as it is still held down.  First, see if the
	   touch point already exists and can be reused (this shouldn't
	   happen, but be safe.)  */                                   // C 1428-1432

	touchpoint = android_find_tool (any, (*event).touch.pointer_id as c_int);   // C 1434

	if ! touchpoint.is_null ()                                     // C 1436
	  {
	    /* Simply update the tool position and send an update.  */   // C 1438
	    (*touchpoint).x = (*event).touch.x;                        // C 1439
	    (*touchpoint).y = (*event).touch.y;                        // C 1440
	    android_update_tools (any, &raw mut inev.ie);              // C 1441
	    inev.ie.timestamp = (*event).touch.time as u64;            // C 1442

	    break 'other;                                              // C 1444
	  }

	/* Otherwise, link a new touchpoint onto the output's list of
	   pressed tools.  */                                          // C 1447-1448

	touchpoint = xmalloc (std::mem::size_of::<android_touch_point> ()) as *mut android_touch_point;   // C 1450
	(*touchpoint).tool_id = (*event).touch.pointer_id as c_int;    // C 1451
	(*touchpoint).x = (*event).touch.x;                            // C 1452
	(*touchpoint).y = (*event).touch.y;                            // C 1453
	(*touchpoint).next = (*(FRAME_OUTPUT_DATA! (any) as *mut android_output)).touch_points;   // C 1454
	(*touchpoint).tool_bar_p = false;                              // C 1455
	(*(FRAME_OUTPUT_DATA! (any) as *mut android_output)).touch_points = touchpoint;   // C 1456

	/* Figure out whether or not the tool was pressed on the tool
	   bar.  Note that the code which runs when it was is more or
	   less an abuse of the mouse highlight machinery, but it works
	   well enough in practice.  */                                // C 1458-1461

	if WINDOWP ((*any).tool_bar_window)                            // C 1463
	    && WINDOW_TOTAL_LINES (XWINDOW ((*any).tool_bar_window)) != 0   // C 1464
	  {
	    let mut window: Lisp_Object = Qnil ();                     // C 1466
	    let x: c_int = (*event).touch.x;                           // C 1467
	    let y: c_int = (*event).touch.y;                           // C 1468

	    window = window_from_coordinates (any as *mut c_void, x, y, std::ptr::null_mut (), true,      // C 1470
					      true, true);             // C 1471

	    /* If this touch has started in the tool bar, do not
	       send it to Lisp.  Instead, simulate a tool bar
	       click, releasing it once it goes away.  */              // C 1473-1475

	    if EQ (window, (*any).tool_bar_window)                     // C 1477
	      {
		/* Call note_mouse_highlight on the tool bar
		   item.  Otherwise, get_tool_bar_item will
		   return 1.

		   This is not necessary when mouse-highlight is
		   nil.  */                                            // C 1479-1484

		if ! NILP (Vmouse_highlight)                           // C 1486
		  {
		    /* Clear the pointer invisible flag to always make
		       note_mouse_highlight do its thing.  */          // C 1488-1489
		    pointer_invisible! (any, false);                   // C 1490
		    note_mouse_highlight (any, x, y);                  // C 1491

		    /* Always allow future mouse motion to
		       update the mouse highlight, no matter
		       where it is.  */                               // C 1493-1495
		    memset (&raw mut (*dpyinfo).last_mouse_glyph as *mut c_void, 0,   // C 1496
			    std::mem::size_of_val (&(*dpyinfo).last_mouse_glyph));   // C 1497
		    (*dpyinfo).last_mouse_glyph_frame = any;           // C 1498
		  }

		handle_tool_bar_click (any, x, y, true, 0);            // C 1501

		/* Flush any changes made by that to the front
		   buffer.  */                                         // C 1503-1504
		android_flush_dirty_back_buffer_on (any);              // C 1505

		/* Mark the touch point as being grabbed by the tool
		   bar.  */                                            // C 1507-1508
		(*touchpoint).tool_bar_p = true;                       // C 1509
		break 'other;                                          // C 1510
	      }
	  }

	/* Now generate the Emacs event.  */                            // C 1514
	input_event_bits! (kind, &raw mut inev.ie, TOUCHSCREEN_BEGIN_EVENT);   // C 1515
	inev.ie.timestamp = (*event).touch.time as u64;                // C 1516
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 1517
	XSETINT! (inev.ie.x, (*event).touch.x as isize);   // C 1518
	XSETINT! (inev.ie.y, (*event).touch.y as isize);   // C 1519
	XSETINT! (inev.ie.arg, (*event).touch.pointer_id as isize);   // C 1520

	break 'other;                                                  // C 1522
      }

    ANDROID_TOUCH_MOVE =>
      {
	if any.is_null ()                                              // C 1526
	  { break 'other; }                                            // C 1527

	/* Look for the tool that moved.  */                            // C 1529

	touchpoint = android_find_tool (any, (*event).touch.pointer_id as c_int);   // C 1531

	/* If it doesn't exist or has been grabbed by the tool bar, skip
	   processing this event.  */                                  // C 1533-1534

	if touchpoint.is_null () || (*touchpoint).tool_bar_p           // C 1536
	  { break 'other; }                                            // C 1537

	/* Otherwise, update the position and send the update event.  */   // C 1539

	(*touchpoint).x = (*event).touch.x;                            // C 1541
	(*touchpoint).y = (*event).touch.y;                            // C 1542
	android_update_tools (any, &raw mut inev.ie);                  // C 1543
	inev.ie.timestamp = (*event).touch.time as u64;                // C 1544

	break 'other;                                                  // C 1546
      }

    ANDROID_TOUCH_UP =>
      {
	if any.is_null ()                                              // C 1550
	  { break 'other; }                                            // C 1551

	/* Now find and unlink the tool in question.  */                // C 1553

	last = &raw mut (*(FRAME_OUTPUT_DATA! (any) as *mut android_output)).touch_points;   // C 1555
	loop
	  {
	    touchpoint = *last;                                        // C 1556
	    if touchpoint.is_null ()
	      { break; }
	    if (*touchpoint).tool_id == (*event).touch.pointer_id as c_int   // C 1558
	      {
		*last = (*touchpoint).next;                            // C 1560

		if (*touchpoint).tool_bar_p                            // C 1562
		  {
		    xfree (touchpoint as *mut c_void);                 // C 1564

		    /* Do what is necessary to release the tool bar and
		       possibly trigger a click.  */                   // C 1566-1567

		    if (*any).last_tool_bar_item != -1                 // C 1569
		      {
			handle_tool_bar_click (any, (*event).touch.x,   // C 1570
					       (*event).touch.y, false,   // C 1571
					       0);                      // C 1572
		      }

		    /* Cancel any outstanding mouse highlight.  */     // C 1574
		    note_mouse_highlight (any, -1, -1);                // C 1575
		    android_flush_dirty_back_buffer_on (any);          // C 1576

		    break 'other;                                      // C 1578
		  }

		/* The tool was unlinked.  Free it and generate the
		   appropriate Emacs event (assuming that it was not
		   grabbed by the tool bar).  */                      // C 1581-1583
		xfree (touchpoint as *mut c_void);                     // C 1584

		input_event_bits! (kind, &raw mut inev.ie, TOUCHSCREEN_END_EVENT);   // C 1586
		inev.ie.timestamp = (*event).touch.time as u64;        // C 1587

		/* Report whether the sequence has been canceled.  */   // C 1589

		if (*event).touch.flags & ANDROID_TOUCH_SEQUENCE_CANCELED != 0   // C 1591
		  { inev.ie.modifiers = 1; }                           // C 1592

		XSETFRAME! (inev.ie.frame_or_window, any);             // C 1594
		XSETINT! (inev.ie.x, (*event).touch.x as isize);   // C 1595
		XSETINT! (inev.ie.y, (*event).touch.y as isize);   // C 1596
		XSETINT! (inev.ie.arg, (*event).touch.pointer_id as isize);   // C 1597

		/* Break out of the loop.  */                          // C 1599
		break 'other;                                          // C 1600
	      }
	    else
	      { last = &raw mut (*touchpoint).next; }                  // C 1603
	  }

	/* No touch point was found.  This shouldn't happen.  */        // C 1606
	break 'other;                                                  // C 1607
      }

    /* Wheel motion.  The events here don't parallel X because
       Android doesn't have scroll valuators.  */                    // C 1609-1610

    ANDROID_WHEEL =>
      {
	if any.is_null ()                                              // C 1614
	  { break 'other; }                                            // C 1615

	if fabs ((*event).wheel.x_delta) > 0.0                         // C 1617
	    || fabs ((*event).wheel.y_delta) > 0.0                     // C 1618
	  {
	    if mwheel_coalesce_scroll_events                           // C 1620
	      {
		if signbit! ((*event).wheel.x_delta)                    // C 1622
		    != signbit! (wheel_event_x)                         // C 1623
		  { wheel_event_x = 0.0; }                             // C 1624

		if signbit! ((*event).wheel.y_delta)                    // C 1626
		    != signbit! (wheel_event_y)                         // C 1627
		  { wheel_event_y = 0.0; }                             // C 1628

		/* Tally up deltas until one of them exceeds 1.0.  */   // C 1630
		wheel_event_x += (*event).wheel.x_delta;               // C 1631
		wheel_event_y += (*event).wheel.y_delta;               // C 1632

		if fabs (wheel_event_x) < 1.0                          // C 1634
		    && fabs (wheel_event_y) < 1.0                      // C 1635
		  { break 'other; }                                    // C 1636
	      }
	    else
	      {
		/* Use the deltas in the event.  */                    // C 1640
		wheel_event_x = (*event).wheel.x_delta;                // C 1641
		wheel_event_y = (*event).wheel.y_delta;                // C 1642
	      }

	    /* Determine what kind of event to send.  */               // C 1645
	    input_event_bits! (kind, &raw mut inev.ie,                 // C 1646
			       if fabs (wheel_event_y)                 // C 1646
				  >= fabs (wheel_event_x)              // C 1647
				 { WHEEL_EVENT } else { HORIZ_WHEEL_EVENT });   // C 1648
	    inev.ie.timestamp = (*event).wheel.time as u64;            // C 1649

	    /* Set the event coordinates.  */                          // C 1651
	    XSETINT! (inev.ie.x, (*event).wheel.x as isize);   // C 1652
	    XSETINT! (inev.ie.y, (*event).wheel.y as isize);   // C 1653

	    /* Set the frame.  */                                      // C 1655
	    XSETFRAME! (inev.ie.frame_or_window, any);                 // C 1656

	    /* Figure out the scroll direction.  */                    // C 1658
	    inev.ie.modifiers = (if signbit! (if fabs (wheel_event_x)   // C 1659
					       >= fabs (wheel_event_y)   // C 1660
					      { wheel_event_x }        // C 1661
					    else { wheel_event_y })    // C 1662
				 { down_modifier as c_uint } else { up_modifier as c_uint });   // C 1663

	    /* Figure out how much to scale the deltas by.  */         // C 1665
	    window = window_from_coordinates (any as *mut c_void, (*event).wheel.x,   // C 1666
					      (*event).wheel.y, std::ptr::null_mut (),   // C 1667
					      false, false, false);    // C 1668

	    if WINDOWP (window)                                        // C 1670
	      { scroll_height = (*XWINDOW (window)).pixel_height; }    // C 1671
	    else
	      /* EVENT_X and EVENT_Y can be outside the
		 frame if F holds the input grab, so fall
		 back to the height of the frame instead.  */          // C 1673-1675
	      { scroll_height = FRAME_PIXEL_HEIGHT (any); }            // C 1676

	    scroll_unit = pow (scroll_height as f64, 2.0 / 3.0);       // C 1678

	    /* Add the keyboard modifiers.  */                         // C 1680
	    inev.ie.modifiers                                          // C 1681
	      |= android_android_to_emacs_modifiers (dpyinfo,          // C 1682
						     (*event).wheel.state as c_int) as c_uint;   // C 1683

	    /* Finally include the scroll deltas.  */                  // C 1685
	    inev.ie.arg = list3 (Qnil (),                              // C 1686
				 make_float (wheel_event_x              // C 1687
					     * scroll_unit),           // C 1688
				 make_float (wheel_event_y              // C 1689
					     * scroll_unit));          // C 1690

	    wheel_event_x = 0.0;                                       // C 1692
	    wheel_event_y = 0.0;                                       // C 1693
	  }

	break 'other;                                                  // C 1696
      }

    /* Iconification.  This is vastly simpler than on X.  */           // C 1698
    ANDROID_ICONIFIED =>
      {
	if any.is_null ()                                              // C 1701
	  { break 'other; }                                            // C 1702

	if FRAME_ICONIFIED_P (any)                                     // C 1704
	  { break 'other; }                                            // C 1705

	SET_FRAME_VISIBLE! (any, false);                                // C 1707
	SET_FRAME_ICONIFIED (any, 1);                                  // C 1708

	input_event_bits! (kind, &raw mut inev.ie, ICONIFY_EVENT);      // C 1710
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 1711
	break 'other;                                                  // C 1712
      }

    ANDROID_DEICONIFIED =>
      {
	if any.is_null ()                                              // C 1716
	  { break 'other; }                                            // C 1717

	if ! FRAME_ICONIFIED_P (any)                                   // C 1719
	  { break 'other; }                                            // C 1720

	SET_FRAME_VISIBLE! (any, true);                                 // C 1722
	SET_FRAME_ICONIFIED (any, 0);                                  // C 1723

	input_event_bits! (kind, &raw mut inev.ie, DEICONIFY_EVENT);    // C 1725
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 1726
	break 'other;                                                  // C 1727
      }

    /* Context menu handling.  */                                      // C 1729
    ANDROID_CONTEXT_MENU =>
      {
	if (*dpyinfo).menu_event_id == -1                              // C 1732
	    /* Previously displayed popup menus might generate events
	       after dismissal, which might interfere.
	       `current_menu_serial' is always set to an identifier
	       identifying the last context menu to be displayed.  */   // C 1733-1736
	    && (*event).menu.menu_event_serial as c_uint == current_menu_serial   // C 1737
	  { (*dpyinfo).menu_event_id = (*event).menu.menu_event_id; }   // C 1738

	break 'other;                                                  // C 1740
      }

    /* Input method events.  textconv.c functions are called here to
       queue events, which are then executed in a safe context
       inside keyboard.c.  */                                         // C 1742-1744
    ANDROID_INPUT_METHOD =>
      {
	if any.is_null ()                                              // C 1747
	  {
	    /* Free any text allocated for this event.  */             // C 1749
	    xfree ((*event).ime.text as *mut c_void);                  // C 1750

	    /* If edits associated with this event haven't been
	       processed yet, signal their completion to avoid delays
	       the next time a call to `android_sync_edit' is made.

	       If events for a deleted frame are interleaved with events
	       for another frame, the edit counter may be prematurely
	       incremented before edits associated with the other frames
	       are processed.  This is not a problem in practice.  */   // C 1752-1759

	    android_notify_conversion ((*event).ime.counter);          // C 1761
	  }
	else
	  { android_handle_ime_event (event, any); }                   // C 1764

	break 'other;                                                  // C 1766
      }

    ANDROID_DND_DRAG_EVENT =>
      {
	if any.is_null ()                                              // C 1770
	  { break 'other; }                                            // C 1771

	/* Generate a drag and drop event to convey its position.  */   // C 1773
	input_event_bits! (kind, &raw mut inev.ie, DRAG_N_DROP_EVENT);   // C 1774
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 1775
	inev.ie.timestamp = ANDROID_CURRENT_TIME as u64;               // C 1776
	XSETINT! (inev.ie.x, (*event).dnd.x as isize);   // C 1777
	XSETINT! (inev.ie.y, (*event).dnd.y as isize);   // C 1778
	inev.ie.arg = Fcons (inev.ie.x, inev.ie.y);                    // C 1779
	break 'other;                                                  // C 1780
      }

    ANDROID_DND_URI_EVENT | ANDROID_DND_TEXT_EVENT =>
      {
	if any.is_null ()                                              // C 1785
	  {
	    free ((*event).dnd.uri_or_string as *mut c_void);          // C 1787
	    break 'other;                                              // C 1788
	  }

	/* An item was dropped over ANY, and is a file in the form of a
	   content or file URI or a string to be inserted.  Generate an
	   event with this information.  */                            // C 1791-1793

	input_event_bits! (kind, &raw mut inev.ie, DRAG_N_DROP_EVENT);   // C 1795
	XSETFRAME! (inev.ie.frame_or_window, any);                     // C 1796
	inev.ie.timestamp = ANDROID_CURRENT_TIME as u64;               // C 1797
	XSETINT! (inev.ie.x, (*event).dnd.x as isize);   // C 1798
	XSETINT! (inev.ie.y, (*event).dnd.y as isize);   // C 1799
	inev.ie.arg = Fcons (if (*event).r#type == ANDROID_DND_TEXT_EVENT   // C 1800
			       { Qtext () } else { Quri () },          // C 1801
			     android_decode_utf16 ((*event).dnd.uri_or_string,   // C 1802
						   (*event).dnd.length));   // C 1803
	free ((*event).dnd.uri_or_string as *mut c_void);              // C 1804
	break 'other;                                                  // C 1805
      }

    ANDROID_NOTIFICATION_DELETED | ANDROID_NOTIFICATION_ACTION =>
      {
	if (*event).notification.r#type == ANDROID_NOTIFICATION_DELETED   // C 1810
	  { android_notification_deleted (&raw mut (*event).notification as *mut c_void, &raw mut inev.ie); }   // C 1811
	else
	  {
	    let mut action: Lisp_Object = Qnil ();                     // C 1814

	    action = android_decode_utf16 ((*event).notification.action,   // C 1816
					   (*event).notification.length);   // C 1817
	    android_notification_action (&raw mut (*event).notification as *mut c_void, &raw mut inev.ie,   // C 1818
					 action);                      // C 1819
	  }

	/* Free dynamically allocated data.  */                        // C 1822
	free ((*event).notification.tag as *mut c_void);               // C 1823
	free ((*event).notification.action as *mut c_void);            // C 1824
	break 'other;                                                  // C 1825
      }

    ANDROID_CONFIGURATION_CHANGED =>
      {
	if (*event).config.detail == ANDROID_PIXEL_DENSITY_CHANGED     // C 1829
	  {
	    /* Update the display configuration from the event.  */    // C 1831
	    (*dpyinfo).resx = (*event).config.u.pixel_density.dpi_x;   // C 1832
	    (*dpyinfo).resy = (*event).config.u.pixel_density.dpi_y;   // C 1833
	    (*dpyinfo).font_resolution                                // C 1834
	      = (*event).config.u.pixel_density.dpi_scaled;            // C 1835
	    /* C 1836-1841's `#if notdef` block (the `__android_log_print` trace) is dead text in
	       every configuration, so it is recorded in the ledger and not transcribed (the slice-1
	       `#if false` rule).  */
	    input_event_bits! (kind, &raw mut inev.ie, CONFIG_CHANGED_EVENT);   // C 1842
	    inev.ie.frame_or_window = XCAR ((*dpyinfo).name_list_element);   // C 1843
	    inev.ie.arg = Qfont_render ();                             // C 1844
	  }
	else if (*event).config.detail == ANDROID_UI_MODE_CHANGED      // C 1846
	  {
	    android_ui_mode = (*event).config.u.ui_mode;                 // C 1848
	    input_event_bits! (kind, &raw mut inev.ie, TOOLKIT_THEME_CHANGED_EVENT);   // C 1849
	    inev.ie.arg = if android_ui_mode == UI_MODE_NIGHT_YES      // C 1850
			    { Qdark () } else { Qlight () };           // C 1851
	  }

	break 'other;                                                  // C 1854
      }

    _ => { break 'other; }                                             // C 1856-1857
    }
  }

  /* OTHER: */
  if input_event_bits! (kind, &raw mut inev.ie) != NO_EVENT            // C 1861
    {
      kbd_buffer_store_buffered_event (&raw mut inev, hold_quit);      // C 1863
      count += 1;                                                      // C 1864
    }

  if do_help != 0                                                      // C 1867
      && !(! hold_quit.is_null () && (*hold_quit).bits != NO_EVENT as u32)    // C 1868
    {
      let mut frame: Lisp_Object;                                      // C 1870

      if ! f.is_null ()                                                // C 1872
	{ XSETFRAME! (frame, f); }                                     // C 1873
      else
	{ frame = Qnil (); }                                           // C 1875
									// C 1870 (C leaves `frame`
									//  uninitialised; every path
									//  assigns it)

      if do_help > 0                                                   // C 1877
	{
	  any_help_event_p = true;                                     // C 1879
	  gen_help_event (help_echo_string, frame, help_echo_window,   // C 1880
			  help_echo_object, help_echo_pos);            // C 1881
	}
      else
	{
	  help_echo_string = Qnil ();                                  // C 1885
	  gen_help_event (Qnil (), frame, Qnil (), Qnil (), 0);        // C 1886
	}
      count += 1;                                                      // C 1888
    }

  return count;                                                        // C 1891
}

/* C 1894-2238: the run of frame, terminal and redisplay-interface callbacks that follows the event
   dispatcher - the event reader, the buffer-flipping pair, the four colour helpers, the
   focus/highlight hooks, the visibility hooks and the iconification hook.  Four names this text
   needs are not in the frozen mirrors, so they stand at their use sites below (P-72 rule 4/P-88 ①).
   Which item is `#[unsafe(no_mangle)]` and which is an `unsafe extern "C" fn` follows C's linkage
   and the addresses `syms_of_androidterm` (C 6745-6761) and the interface arrays take.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3656: `extern void gui_update_cursor (struct frame *, bool);`
  pub fn gui_update_cursor (f: *mut frame, on_p: bool);
  /// C: dispextern.h 3798-3799: `bool parse_color_spec (const char *,
  /// unsigned short *, unsigned short *, unsigned short *);`
  pub fn parse_color_spec (spec: *const c_char, r: *mut c_ushort, g: *mut c_ushort,
			   b: *mut c_ushort) -> bool;
  /// C: dispextern.h 3839: `void gamma_correct (struct frame *, XColor *);` - `XColor` is this
  /// tree's `Emacs_Color` (dispextern.h 22-64's X-like record, `include/dispextern.rs` 25).
  pub fn gamma_correct (f: *mut frame, color: *mut Emacs_Color);
  /// C: lisp.h 5117: `extern void store_frame_param (struct frame *, Lisp_Object, Lisp_Object);`
  /// (`src/androidfns.rs` 103-104 declares the same header line for the same unit family.)
  pub fn store_frame_param (f: *mut frame, prop: Lisp_Object, val: Lisp_Object);
}

/* C 1894-1927.  `static` in C and C 6745 takes its address, so it is an `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_read_socket (terminal: *mut terminal,
					  hold_quit: *mut input_event) -> c_int
{
  let mut count: c_int = 0;                                             // C 1898
  let mut dpyinfo: *mut android_display_info;                           // C 1899

  dpyinfo = (*terminal).display_info.android as *mut android_display_info;   // C 1901

  block_input ();                                                       // C 1903
  while android_pending () != 0                                         // C 1904
    {
      let mut finish: c_int = 0;                                        // C 1906
      let mut event: android_event = std::mem::zeroed ();               // C 1907

      android_next_event (&raw mut event);                              // C 1909
      count += handle_one_android_event (dpyinfo, &raw mut event, &raw mut finish,   // C 1910
					 hold_quit);                    // C 1911

      if finish == ANDROID_EVENT_GOTO_OUT                               // C 1913
	{ break; }                                                      // C 1914
    }
  unblock_input ();                                                     // C 1916

  /* If the focus was just given to an auto-raising frame, raise it
     now.  */                                                           // C 1918-1919
  if ! (*dpyinfo).pending_autoraise_frame.is_null ()                    // C 1920
    {
      android_raise_frame ((*dpyinfo).pending_autoraise_frame);          // C 1922
      (*dpyinfo).pending_autoraise_frame = std::ptr::null_mut ();        // C 1923
    }

  return count;                                                         // C 1926
}

/* C 1929-1946.  `static`, and C 6746 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_frame_up_to_date (f: *mut frame)
{
  eassert! (FRAME_ANDROID_P (f));                                       // C 1932
  block_input ();                                                       // C 1933
  FRAME_MOUSE_UPDATE! (f);                                              // C 1934

  if ! buffer_flipping_blocked_p ()                                     // C 1936
      && FRAME_ANDROID_NEED_BUFFER_FLIP! (f)                            // C 1937
    { show_back_buffer (f); }                                          // C 1938

  /* The frame is now complete, as its contents have been drawn.  */     // C 1940
  /* C 1941's `FRAME_ANDROID_COMPLETE_P (f) = true;` is an l-value use of the folded member: the
     mirror's macro is a read (include/androidterm.rs 474-479) and its own comment says a writer
     site spells `bits |= MASK` (the `cm.rs` convention, status 3.516).  */
  (*( (*f).output_data.android as *mut android_output )).bits |= ANDROID_OUTPUT_COMPLETE;   // C 1941

  /* Shrink the scanline buffer used by the font backend.  */            // C 1943
  sfntfont_android_shrink_scanline_buffer ();                           // C 1944
  unblock_input ();                                                     // C 1945
}

/* C 1948-1957.  `static`, and C 6748 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_buffer_flipping_unblocked_hook (f: *mut frame)
{
  block_input ();                                                       // C 1951

  if FRAME_ANDROID_NEED_BUFFER_FLIP! (f)                                // C 1953
    { show_back_buffer (f); }                                          // C 1954

  unblock_input ();                                                     // C 1956
}

/* C 1959-1968.  `static`, and C 6751 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_query_frame_background_color (f: *mut frame,
							   bgcolor: *mut Emacs_Color)
{
  let mut background: c_ulong;                                          // C 1962

  background = FRAME_BACKGROUND_PIXEL (f);                              // C 1964
  (*bgcolor).pixel = background;                                        // C 1965

  android_query_colors (f, bgcolor, 1);                                 // C 1967
}

/* C 1970-2004.  Non-`static` in C and no address is taken: `#[unsafe(no_mangle)]` with the C ABI
   the mirror's declaration uses (`include/androidterm.rs` 584).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_parse_color (f: *mut frame, color_name: *const c_char,
					      color: *mut Emacs_Color) -> c_int
{
  let mut r: c_ushort = 0;                                              // C 1974
  let mut g: c_ushort = 0;
  let mut b: c_ushort = 0;
  let mut tem: Lisp_Object = Qnil ();                                   // C 1975
  let mut tem1: Lisp_Object = Qnil ();
  let mut lisp_color: c_ulong = 0;                                      // C 1976

  if parse_color_spec (color_name, &raw mut r, &raw mut g, &raw mut b)   // C 1978
    {
      (*color).red = r;                                                 // C 1980
      (*color).green = g;                                               // C 1981
      (*color).blue = b;                                                // C 1982

      return 1;                                                         // C 1984
    }

  tem = (*x_display_list).color_map;                                    // C 1987
  while CONSP (tem)                                                     // C 1988
    {
      tem1 = XCAR (tem);                                                // C 1990

      if CONSP (tem1)                                                   // C 1992
	  && dispextern_externs::xstrcasecmp (SSDATA (XCAR (tem1)), color_name) == 0        // C 1993
	{
	  lisp_color = XFIXNUM (XCDR (tem1)) as c_ulong;                 // C 1995
	  (*color).red = (RED_FROM_ULONG! (lisp_color) * 257) as c_ushort;   // C 1996
	  (*color).green = (GREEN_FROM_ULONG! (lisp_color) * 257) as c_ushort;   // C 1997
	  (*color).blue = (BLUE_FROM_ULONG! (lisp_color) * 257) as c_ushort;   // C 1998
	  return 1;                                                     // C 1999
	}
      tem = XCDR (tem);                                                 // C 1988 (the `for` step)
    }

  return 0;                                                             // C 2003
}

/* C 2006-2038.  Non-`static` in C and no address is taken.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_alloc_nearest_color (f: *mut frame,
						      color: *mut Emacs_Color) -> bool
{
  let mut ntsc: c_uint;                                                 // C 2009

  gamma_correct (f, color);                                             // C 2011

  if (*FRAME_DISPLAY_INFO! (f)).n_planes == 1                           // C 2013
    {
      /* Black and white.  I think this is the luminance formula applied
	 by the X server on generic monochrome framebuffers.  */        // C 2015-2016
      (*color).pixel = (if ((((30 * (*color).red as c_ulong)        // C 2017
			      + (59 * (*color).green as c_ulong)        // C 2018
			      + (11 * (*color).blue as c_ulong)) >> 8)  // C 2019
			     >= (((1 << 8) - 1) * 50))                  // C 2020
			 { 0xffffff } else { 0 }) as c_ulong;           // C 2021
    }
  else if (*FRAME_DISPLAY_INFO! (f)).n_planes <= 8                      // C 2023
    {
      /* 256 grays.  */                                                 // C 2025
      ntsc = min (255, ((((*color).red as f64 * 0.299              // C 2026
			   + (*color).green as f64 * 0.587            // C 2027
			   + (*color).blue as f64 * 0.114)            // C 2028
			  / 256.0) as c_uint));                       // C 2029
      (*color).pixel = RGB_TO_ULONG! (ntsc as c_ulong, ntsc as c_ulong,   // C 2030
				      ntsc as c_ulong);                    // C 2030
    }
  else
    { (*color).pixel = RGB_TO_ULONG! (((*color).red / 256) as c_ulong,   // C 2033
				      ((*color).green / 256) as c_ulong,   // C 2034
				      ((*color).blue / 256) as c_ulong); }   // C 2035

  return true;                                                          // C 2037
}

/* C 2040-2051.  Non-`static` in C and C 6752 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_query_colors (f: *mut frame, colors: *mut Emacs_Color,
					       ncolors: c_int)
{
  let mut i: c_int;                                                     // C 2043

  i = 0;                                                                // C 2045
  while i < ncolors
    {
      (*colors.offset (i as isize)).red = (RED_FROM_ULONG! ((*colors.offset (i as isize)).pixel) * 257) as c_ushort;   // C 2047
      (*colors.offset (i as isize)).green = (GREEN_FROM_ULONG! ((*colors.offset (i as isize)).pixel) * 257) as c_ushort;   // C 2048
      (*colors.offset (i as isize)).blue = (BLUE_FROM_ULONG! ((*colors.offset (i as isize)).pixel) * 257) as c_ushort;   // C 2049
      i += 1;                                                           // C 2045 (the `for` step)
    }
}

/* C 2053-2085.  `static`, and C 6753 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_mouse_position (fp: *mut *mut frame, insist: c_int,
					     bar_window: *mut Lisp_Object,
					     part: *mut scroll_bar_part, x: *mut Lisp_Object,
					     y: *mut Lisp_Object, timestamp: *mut Time)
{
  let mut tail: Lisp_Object = Qnil ();                                  // C 2059
  let mut frame: Lisp_Object = Qnil ();
  let mut dpyinfo: *mut android_display_info;                           // C 2060

  dpyinfo = FRAME_DISPLAY_INFO! (*fp);                                  // C 2062

  /* This is the best implementation possible on Android, where the
     system doesn't let Emacs obtain any information about the mouse
     pointer at all.  */                                                // C 2064-2066

  if ! (*dpyinfo).last_mouse_motion_frame.is_null ()                    // C 2068
    {
      *fp = (*dpyinfo).last_mouse_motion_frame;                         // C 2070
      *timestamp = (*dpyinfo).last_mouse_movement_time;                 // C 2071
      *x = make_fixnum ((*dpyinfo).last_mouse_motion_x as EMACS_INT);   // C 2072
      *y = make_fixnum ((*dpyinfo).last_mouse_motion_y as EMACS_INT);   // C 2073
      *bar_window = Qnil ();                                            // C 2074
      *part = scroll_bar_nowhere;                                       // C 2075

      FOR_EACH_FRAME! (tail, frame, {                                 // C 2077
	  if FRAME_ANDROID_P (XFRAME (frame))                           // C 2079
	    { mouse_moved! (XFRAME (frame), false); }                   // C 2080
	});

      mouse_moved! ((*dpyinfo).last_mouse_motion_frame, false);         // C 2083
    }
}

/* C 2087-2100.  `static`, and C 6754 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_get_focus_frame (f: *mut frame) -> Lisp_Object
{
  let mut lisp_focus: Lisp_Object = Qnil ();                            // C 2090
  let mut focus: *mut frame;                                            // C 2091

  focus = (*FRAME_DISPLAY_INFO! (f)).focus_frame;                       // C 2093

  if focus.is_null ()                                                   // C 2095
    { return Qnil (); }                                                 // C 2096

  XSETFRAME! (lisp_focus, focus);                                       // C 2098
  return lisp_focus;                                                    // C 2099
}

/* C 2102-2109.  `static`, and C 6755 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_focus_frame (f: *mut frame, noactivate: bool)
{
  /* Set the input focus to the frame's window.  The system only lets
     this work on child frames.  */                                     // C 2105-2106
  android_set_input_focus (FRAME_ANDROID_WINDOW! (f),                   // C 2107
			   ANDROID_CURRENT_TIME as u64);                 // C 2108
}

/* The two procedures below only have to update the cursor on Android,
   as there are no window borders there.  */                           // C 2111-2112

/* C 2114-2118.  `static`; only called (C 2152/2154), so a plain Rust-ABI item.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_frame_highlight (f: *mut frame)
{
  gui_update_cursor (f, true);                                          // C 2117
}

/* C 2120-2124.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_frame_unhighlight (f: *mut frame)
{
  gui_update_cursor (f, true);                                          // C 2123
}

/* C 2126-2156.  `static`; only called (C 2161), so a plain Rust-ABI item.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_frame_rehighlight (dpyinfo: *mut android_display_info)
{
  let mut old_highlight: *mut frame;                                    // C 2129

  old_highlight = (*dpyinfo).highlight_frame;                           // C 2131

  if ! (*dpyinfo).focus_frame.is_null ()                                // C 2133
    {
      (*dpyinfo).highlight_frame                                        // C 2135
	= (if FRAMEP (FRAME_FOCUS_FRAME ((*dpyinfo).focus_frame))       // C 2136
	     { XFRAME (FRAME_FOCUS_FRAME ((*dpyinfo).focus_frame)) }    // C 2137
	   else { (*dpyinfo).focus_frame });                            // C 2138
      if ! FRAME_LIVE_P ((*dpyinfo).highlight_frame)                    // C 2139
	{
	  fset_focus_frame ((*dpyinfo).focus_frame, Qnil ());           // C 2141
	  (*dpyinfo).highlight_frame = (*dpyinfo).focus_frame;          // C 2142
	}
    }
  else
    { (*dpyinfo).highlight_frame = std::ptr::null_mut (); }             // C 2146

  if (*dpyinfo).highlight_frame != old_highlight                        // C 2148
    {
      /* This is not yet required on Android.  */                       // C 2150
      if ! old_highlight.is_null ()                                     // C 2151
	{ android_frame_unhighlight (old_highlight); }                  // C 2152
      if ! (*dpyinfo).highlight_frame.is_null ()                        // C 2153
	{ android_frame_highlight ((*dpyinfo).highlight_frame); }        // C 2154
    }
}

/* C 2158-2162.  `static`, and C 6756 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_frame_rehighlight_hook (f: *mut frame)
{
  android_frame_rehighlight (FRAME_DISPLAY_INFO! (f));                  // C 2161
}

/* C 2164-2171.  `static`, and C 6757 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_frame_raise_lower (f: *mut frame, raise_flag: bool)
{
  if raise_flag                                                         // C 2167
    { android_raise_frame (f); }                                       // C 2168
  else
    { android_lower_frame (f); }                                       // C 2170
}

/* C 2173-2180.  Non-`static` in C and no address is taken.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_make_frame_visible (f: *mut frame)
{
  android_map_window (FRAME_ANDROID_WINDOW! (f));                       // C 2176

  SET_FRAME_VISIBLE! (f, true);                                         // C 2178
  SET_FRAME_ICONIFIED (f, 0);                                          // C 2179
}

/* C 2182-2193.  Non-`static` in C and no address is taken.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_make_frame_invisible (f: *mut frame)
{
  /* Don't keep the highlight on an invisible frame.  */                 // C 2185
  if (*FRAME_DISPLAY_INFO! (f)).highlight_frame == f                    // C 2186
    { (*FRAME_DISPLAY_INFO! (f)).highlight_frame = std::ptr::null_mut (); }   // C 2187

  android_unmap_window (FRAME_ANDROID_WINDOW! (f));                     // C 2189

  SET_FRAME_VISIBLE! (f, false);                                        // C 2191
  SET_FRAME_ICONIFIED (f, 0);                                          // C 2192
}

/* C 2195-2202.  `static`, and C 6759 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_make_frame_visible_invisible (f: *mut frame, visible: bool)
{
  if visible                                                            // C 2198
    { android_make_frame_visible (f); }                                // C 2199
  else
    { android_make_frame_invisible (f); }                              // C 2201
}

/* C 2204-2231.  `static`, and C 6760 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_fullscreen_hook (f: *mut frame)
{
  let mut wanted: Lisp_Object = Qnil ();                                // C 2207

  if ! FRAME_PARENT_FRAME (f).is_null ()                                // C 2209
    {
      /* Explicitly setting fullscreen is not supported on older
	 Android versions.  */                                         // C 2211-2212

      wanted = (if { let want_fullscreen = frame_want_fullscreen (f); want_fullscreen } == FULLSCREEN_BOTH
		  { Qfullscreen () } else { Qmaximized () });          // C 2215

      if android_set_fullscreen (FRAME_ANDROID_WINDOW! (f),            // C 2217
				 EQ (wanted, Qfullscreen ())) != 0     // C 2218
	{ store_frame_param (f, Qfullscreen (), Qmaximized ()); }      // C 2219
      else
	{ store_frame_param (f, Qfullscreen (), wanted); }             // C 2221
    }
  else
    {
      store_frame_param (f, Qfullscreen (), Qnil ());                  // C 2225

      /* If this is a child frame, don't keep it fullscreen
	 anymore.  */                                                 // C 2227-2228
      android_set_fullscreen (FRAME_ANDROID_WINDOW! (f), false);       // C 2229
    }
}

/* C 2233-2238.  Non-`static` in C and C 6761 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_iconify_frame (f: *mut frame)
{
  /* This really doesn't work on Android.  */                            // C 2236
  error (c"Can't notify window manager of iconification".as_ptr ());    // C 2237
}

/* C 2240-2648: the wait/size/position group and the frame-teardown group.  Four names this text
   needs are not in the frozen mirrors, so they stand at their use sites below (P-72 rule 4), and
   four frame.h accessors C *writes* need the write arm the mirror does not carry, so the unit
   carries the two-arm macros - the `src/xterm.rs` 15127-15145 shape.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_COLUMN_WIDTH {
  ($f:expr) => { FRAME_COLUMN_WIDTH ($f) };
  ($f:expr, $v:expr) => { (*$f).column_width = $v };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_LINE_HEIGHT {
  ($f:expr) => { FRAME_LINE_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).line_height = $v };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_TAB_BAR_HEIGHT {
  ($f:expr) => { FRAME_TAB_BAR_HEIGHT ($f) };
  ($f:expr, $v:expr) => { (*$f).tab_bar_height = $v };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr) => { FRAME_CONFIG_SCROLL_BAR_COLS ($f) };
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_cols = $v };
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: frame.h 1511-1512: `extern void adjust_frame_size (struct frame *, int, int, int, bool,
  /// Lisp_Object);`
  pub fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
			    inhibit: c_int, pretend: bool, parameter: Lisp_Object);
  /// C: dispextern.h 3631: `extern void get_font_ascent_descent (struct font *, int *, int *);`
  pub fn get_font_ascent_descent (font: *mut font, ascent: *mut c_int, descent: *mut c_int);
  /// C: dispextern.h 3822: `void free_frame_faces (struct frame *);`
  pub fn free_frame_faces (f: *mut frame);
  /// C: fontset.h 39: `extern int fontset_from_font (Lisp_Object);`
  pub fn fontset_from_font (font_object: Lisp_Object) -> c_int;
}

/* C 2240-2280.  `static` and only called (C 2296/2351), so a plain Rust-ABI item.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_wait_for_event (f: *mut frame, eventtype: c_int)
{
  if ! FLOATP (Vandroid_wait_for_event_timeout)                       // C 2243
    { return; }                                                       // C 2244

  let level: c_int = interrupt_input_blocked;                          // C 2246
  let mut tmo: timespec = std::mem::zeroed ();                         // C 2247
  let mut tmo_at: timespec = std::mem::zeroed ();
  let mut time_now: timespec = std::mem::zeroed ();

  (*f).wait_event_type = eventtype;                                    // C 2249

  /* Default timeout is 0.1 second.  Hopefully not noticeable.  */       // C 2251
  let timeout: f64 = XFLOAT_DATA (Vandroid_wait_for_event_timeout);     // C 2252
  let timeout_seconds: time_t = timeout as time_t;                     // C 2253
  tmo = make_timespec (timeout_seconds,                                // C 2254
		       ((timeout - timeout_seconds as f64)             // C 2255
			* 1000.0 * 1000.0 * 1000.0) as c_long);        // C 2256
  tmo_at = timespec_add (current_timespec (), tmo);                    // C 2257

  while (*f).wait_event_type != 0                                      // C 2259
    {
      pending_signals = true;                                          // C 2261
      totally_unblock_input ();                                        // C 2262
      /* android_read_socket is called after unblock.  */               // C 2263
      block_input ();                                                  // C 2264
      interrupt_input_blocked = level;                                 // C 2265

      if (*f).wait_event_type == 0                                     // C 2267
	{ break; }                                                     // C 2268

      time_now = current_timespec ();                                  // C 2270
      if timespec_cmp (tmo_at, time_now) < 0                           // C 2271
	{ break; }                                                     // C 2272

      tmo = timespec_sub (tmo_at, time_now);                           // C 2274
      if android_select (0, std::ptr::null_mut (), std::ptr::null_mut (),   // C 2275
			 std::ptr::null_mut (), &raw mut tmo) == 0     // C 2275
	{ break; }                                                     // C 2276 (Timeout)
    }

  (*f).wait_event_type = 0;                                            // C 2279
}

/* C 2282-2315.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_window_size_1 (f: *mut frame, change_gravity: bool,
				     width: c_int, height: c_int)
{
  if change_gravity                                                    // C 2286
    { (*f).win_gravity = NorthWestGravity; }                           // C 2287

  android_resize_window (FRAME_ANDROID_WINDOW! (f), width as c_uint,   // C 2289
			 height as c_uint);                            // C 2290

  SET_FRAME_GARBAGED! (f);                                             // C 2292

  if FRAME_VISIBLE_P (f)                                               // C 2294
    {
      android_wait_for_event (f, ANDROID_CONFIGURE_NOTIFY);            // C 2296

      if CONSP (frame_size_history)                                    // C 2298
	{ frame_size_history_extra (f, build_string (c"set_window_size_1 visible".as_ptr ()),   // C 2299
				    FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f),   // C 2300
				    width, height, (*f).new_width, (*f).new_height); }   // C 2301
    }
  else
    {
      if CONSP (frame_size_history)                                    // C 2305
	{ frame_size_history_extra (f, build_string (c"set_window_size_1 invisible".as_ptr ()),   // C 2306
				    FRAME_PIXEL_WIDTH (f), FRAME_PIXEL_HEIGHT (f),   // C 2308
				    width, height, (*f).new_width, (*f).new_height); }   // C 2309

      adjust_frame_size (f, FRAME_PIXEL_TO_TEXT_WIDTH (f, width),      // C 2311
			 FRAME_PIXEL_TO_TEXT_HEIGHT (f, height),       // C 2312
			 5, false, Qx_set_window_size_1 ());           // C 2313
    }
}

/* C 2317-2338.  Non-`static` in C and C 6762 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_window_size (f: *mut frame, change_gravity: bool,
						  width: c_int, height: c_int)
{
  block_input ();                                                       // C 2321

  android_set_window_size_1 (f, change_gravity, width, height);          // C 2323
  android_clear_under_internal_border (f);                              // C 2324

  /* If cursor was outside the new size, mark it as off.  */             // C 2326
  mark_window_cursors_off (XWINDOW ((*f).root_window));                 // C 2327

  /* Clear out any recollection of where the mouse highlighting was,
     since it might be in a place that's outside the new frame size.
     Actually checking whether it is outside is a pain in the neck,
     so don't try--just let the highlighting be done afresh with new size.  */   // C 2329-2332
  cancel_mouse_face (f);                                                // C 2333

  unblock_input ();                                                     // C 2335

  do_pending_window_change (false);                                     // C 2337
}

/* C 2340-2358.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_window_size_and_position_1 (f: *mut frame, width: c_int, height: c_int)
{
  let x: c_int = (*f).left_pos;                                         // C 2343
  let y: c_int = (*f).top_pos;                                          // C 2344

  android_move_resize_window (FRAME_ANDROID_WINDOW! (f), x, y,   // C 2346
				    width as c_uint, height as c_uint);        // C 2346

  SET_FRAME_GARBAGED! (f);                                             // C 2348

  if FRAME_VISIBLE_P (f)                                               // C 2350
    { android_wait_for_event (f, ANDROID_CONFIGURE_NOTIFY); }           // C 2351
  else
    /* Call adjust_frame_size right.  It might be tempting to clear out
       f->new_width and f->new_height here.  */                        // C 2353-2354
    { adjust_frame_size (f, FRAME_PIXEL_TO_TEXT_WIDTH (f, width),      // C 2355
			 FRAME_PIXEL_TO_TEXT_HEIGHT (f, height),       // C 2356
			 5, false, Qx_set_window_size_1 ()); }         // C 2357
}

/* C 2360-2380.  Non-`static` in C and C 6764 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_set_window_size_and_position (f: *mut frame, width: c_int,
							       height: c_int)
{
  block_input ();                                                       // C 2363

  android_set_window_size_and_position_1 (f, width, height);             // C 2365
  android_clear_under_internal_border (f);                              // C 2366

  /* If cursor was outside the new size, mark it as off.  */             // C 2368
  mark_window_cursors_off (XWINDOW (FRAME_ROOT_WINDOW (f)));            // C 2369

  /* Clear out any recollection of where the mouse highlighting was,
     since it might be in a place that's outside the new frame size.
     Actually checking whether it is outside is a pain in the neck,
     so don't try--just let the highlighting be done afresh with new size.  */   // C 2371-2374
  cancel_mouse_face (f);                                                // C 2375

  unblock_input ();                                                     // C 2377

  do_pending_window_change (false);                                     // C 2379
}

/* Calculate the absolute position in frame F
   from its current recorded position values and gravity.  */         // C 2382-2383

/* C 2385-2455.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_calc_absolute_position (f: *mut frame)
{
  let flags: c_int = (*f).size_hint_flags;                              // C 2388
  let p: *mut frame = FRAME_PARENT_FRAME (f);                           // C 2389

  /* We have nothing to do if the current position
     is already for the top-left corner.  */                            // C 2391-2392
  if !((flags & XNegative != 0) || (flags & YNegative != 0))            // C 2393
    { return; }                                                         // C 2394

  /* Treat negative positions as relative to the leftmost bottommost
     position that fits on the screen.  */                              // C 2396-2397
  if flags & XNegative != 0                                             // C 2398
    {
      let mut width: c_int = FRAME_PIXEL_WIDTH (f);                     // C 2400

      /* A frame that has been visible at least once should have outer
	 edges.  */                                                     // C 2402-2403
      if (*( (*f).output_data.android as *mut android_output )).bits    // C 2404
	  & ANDROID_OUTPUT_HAS_BEEN_VISIBLE != 0                        // C 2404 (C's `has_been_visible`)
	  && p.is_null ()
	{
	  let mut frame: Lisp_Object = Qnil ();                         // C 2406
	  let mut edges: Lisp_Object = Qnil ();                         // C 2407

	  XSETFRAME! (frame, f);                                        // C 2409
	  edges = Fandroid_frame_edges (frame, Qouter_edges ());         // C 2410
	  if ! NILP (edges)                                             // C 2411
	    { width = (XFIXNUM (Fnth (make_fixnum (2), edges))          // C 2412
		       - XFIXNUM (Fnth (make_fixnum (0), edges))) as c_int; }   // C 2413
	}

      if ! p.is_null ()                                                // C 2416
	{ (*f).left_pos = (FRAME_PIXEL_WIDTH (p) - width - 2 * (*f).border_width   // C 2417
			   + (*f).left_pos); }                          // C 2418
      else
	/* Not that this is of much significance, for Android programs
	   cannot position their windows at absolute positions in the
	   screen.  */                                                  // C 2420-2422
	{ (*f).left_pos = (android_get_screen_width () - width + (*f).left_pos); }   // C 2423
    }

  if flags & YNegative != 0                                             // C 2427
    {
      let mut height: c_int = FRAME_PIXEL_HEIGHT (f);                   // C 2429

      if (*( (*f).output_data.android as *mut android_output )).bits    // C 2431
	  & ANDROID_OUTPUT_HAS_BEEN_VISIBLE != 0                        // C 2431
	  && p.is_null ()
	{
	  let mut frame: Lisp_Object = Qnil ();                         // C 2433
	  let mut edges: Lisp_Object = Qnil ();                         // C 2434

	  XSETFRAME! (frame, f);                                        // C 2436
	  if NILP (edges)                                               // C 2437
	    { edges = Fandroid_frame_edges (frame, Qouter_edges ()); }  // C 2438
	  if ! NILP (edges)                                             // C 2439
	    { height = (XFIXNUM (Fnth (make_fixnum (3), edges))         // C 2440
			- XFIXNUM (Fnth (make_fixnum (1), edges))) as c_int; }   // C 2441
	}

      if ! p.is_null ()                                                // C 2444
	{ (*f).top_pos = (FRAME_PIXEL_HEIGHT (p) - height - 2 * (*f).border_width   // C 2445
			  + (*f).top_pos); }                           // C 2446
      else
	{ (*f).top_pos = (android_get_screen_height () - height + (*f).top_pos); }   // C 2448
  }

  /* The left_pos and top_pos
     are now relative to the top and left screen edges,
     so the flags should correspond.  */                               // C 2451-2453
  (*f).size_hint_flags &= !(XNegative | YNegative);                     // C 2454
}

/* C 2457-2476.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_offset (f: *mut frame, xoff: c_int, yoff: c_int, change_gravity: c_int)
{
  if change_gravity > 0                                                // C 2461
    {
      (*f).top_pos = yoff;                                            // C 2463
      (*f).left_pos = xoff;                                           // C 2464
      (*f).size_hint_flags &= !(XNegative | YNegative);                 // C 2465
      if xoff < 0                                                     // C 2466
	{ (*f).size_hint_flags |= XNegative; }                        // C 2467
      if yoff < 0                                                     // C 2468
	{ (*f).size_hint_flags |= YNegative; }                        // C 2469
      (*f).win_gravity = NorthWestGravity;                            // C 2470
    }

  android_calc_absolute_position (f);                                  // C 2473
  android_move_window (FRAME_ANDROID_WINDOW! (f), (*f).left_pos,        // C 2474
		       (*f).top_pos);                                  // C 2475
}

/* C 2478-2482.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_alpha (f: *mut frame)
{
  /* Not supported on Android.  */                                       // C 2481
}

/* C 2484-2528.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_new_font (f: *mut frame, font_object: Lisp_Object, mut fontset: c_int)
    -> Lisp_Object
{
  let font: *mut font = XFONT_OBJECT (font_object);                     // C 2487
  let mut unit: c_int = 0;                                             // C 2488
  let mut font_ascent: c_int = 0;
  let mut font_descent: c_int = 0;

  if fontset < 0                                                       // C 2490
    { fontset = fontset_from_font (font_object); }                     // C 2491
  FRAME_FONTSET! (f) = fontset;                                        // C 2492
  if FRAME_FONT! (f) == font                                           // C 2493
    /* This font is already set in frame F.  There's nothing more to
       do.  */                                                         // C 2494-2495
    { return font_object; }                                            // C 2496

  FRAME_FONT! (f) = font;                                              // C 2498
  FRAME_BASELINE_OFFSET! (f) = (*font).baseline_offset;                 // C 2499
  FRAME_COLUMN_WIDTH! (f, (*font).average_width);                       // C 2500
  get_font_ascent_descent (font, &raw mut font_ascent, &raw mut font_descent);   // C 2501
  FRAME_LINE_HEIGHT! (f, font_ascent + font_descent);                   // C 2502

  /* We could use a more elaborate calculation here.  */                // C 2504
  FRAME_TAB_BAR_HEIGHT! (f, FRAME_TAB_BAR_LINES (f) * FRAME_LINE_HEIGHT (f));   // C 2505

  /* Compute character columns occupied by scrollbar.

     Don't do things differently for non-toolkit scrollbars
     (Bug#17163).  */                                                  // C 2507-2510
  unit = FRAME_COLUMN_WIDTH (f);                                       // C 2511
  if FRAME_CONFIG_SCROLL_BAR_WIDTH (f) > 0                             // C 2512
    { FRAME_CONFIG_SCROLL_BAR_COLS! (f, (FRAME_CONFIG_SCROLL_BAR_WIDTH (f) + unit - 1) / unit); }   // C 2513-2514
  else
    { FRAME_CONFIG_SCROLL_BAR_COLS! (f, (14 + unit - 1) / unit); }      // C 2516

  /* Don't change the size of a tip frame; there's no point in doing it
     because it's done in Fx_show_tip, and it leads to problems because
     the tip frame has no widget.  */                                  // C 2519-2521
  if FRAME_ANDROID_WINDOW! (f) != 0 && ! FRAME_TOOLTIP_P (f)            // C 2522
    { adjust_frame_size (f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH (f),   // C 2523
			 FRAME_LINES (f) * FRAME_LINE_HEIGHT (f), 3,   // C 2524
			 false, Qfont ()); }                           // C 2525

  return font_object;                                                  // C 2527
}

/* C 2530-2534.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_bitmap_icon (f: *mut frame, file: Lisp_Object) -> bool
{
  return false;                                                        // C 2533
}

/* C 2536-2540.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_free_pixmap_hook (f: *mut frame, pixmap: Emacs_Pixmap)
{
  android_free_pixmap (pixmap);                                        // C 2539
}

/* C 2542-2634.  Non-`static` in C and no address is taken in this file.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_free_frame_resources (f: *mut frame)
{
  let mut dpyinfo: *mut android_display_info;                           // C 2545
  let mut hlinfo: *mut Mouse_HLInfo;                                    // C 2546
  let mut last: *mut android_touch_point;                               // C 2547
  let mut next: *mut android_touch_point;

  dpyinfo = FRAME_DISPLAY_INFO! (f);                                    // C 2549
  hlinfo = &raw mut (*dpyinfo).mouse_highlight;                         // C 2550

  block_input ();                                                       // C 2552
  free_frame_faces (f);                                                 // C 2553

  /* FRAME_ANDROID_WINDOW can be 0 if frame creation failed.  */         // C 2555
  if FRAME_ANDROID_WINDOW! (f) != 0                                     // C 2556
    { android_destroy_window (FRAME_ANDROID_WINDOW! (f)); }             // C 2557

  android_free_gcs (f);                                                 // C 2559

  /* Free cursors.  */                                                  // C 2561
  if (*( (*f).output_data.android as *mut android_output )).text_cursor != 0   // C 2562
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).text_cursor); }   // C 2563
  if (*( (*f).output_data.android as *mut android_output )).nontext_cursor != 0   // C 2564
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).nontext_cursor); }   // C 2565
  if (*( (*f).output_data.android as *mut android_output )).modeline_cursor != 0   // C 2566
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).modeline_cursor); }   // C 2567
  if (*( (*f).output_data.android as *mut android_output )).hand_cursor != 0   // C 2568
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).hand_cursor); }   // C 2569
  if (*( (*f).output_data.android as *mut android_output )).hourglass_cursor != 0   // C 2570
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).hourglass_cursor); }   // C 2571
  if (*( (*f).output_data.android as *mut android_output )).horizontal_drag_cursor != 0   // C 2572
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).horizontal_drag_cursor); }   // C 2573
  if (*( (*f).output_data.android as *mut android_output )).vertical_drag_cursor != 0   // C 2574
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).vertical_drag_cursor); }   // C 2575
  if (*( (*f).output_data.android as *mut android_output )).left_edge_cursor != 0   // C 2576
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).left_edge_cursor); }   // C 2577
  if (*( (*f).output_data.android as *mut android_output )).top_left_corner_cursor != 0   // C 2578
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).top_left_corner_cursor); }   // C 2579
  if (*( (*f).output_data.android as *mut android_output )).top_edge_cursor != 0   // C 2580
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).top_edge_cursor); }   // C 2581
  if (*( (*f).output_data.android as *mut android_output )).top_right_corner_cursor != 0   // C 2582
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).top_right_corner_cursor); }   // C 2583
  if (*( (*f).output_data.android as *mut android_output )).right_edge_cursor != 0   // C 2584
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).right_edge_cursor); }   // C 2585
  if (*( (*f).output_data.android as *mut android_output )).bottom_right_corner_cursor != 0   // C 2586
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).bottom_right_corner_cursor); }   // C 2587
  if (*( (*f).output_data.android as *mut android_output )).bottom_edge_cursor != 0   // C 2588
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).bottom_edge_cursor); }   // C 2589
  if (*( (*f).output_data.android as *mut android_output )).bottom_left_corner_cursor != 0   // C 2590
    { android_free_cursor ((*( (*f).output_data.android as *mut android_output )).bottom_left_corner_cursor); }   // C 2591

  /* Free extra GCs allocated by android_setup_relief_colors.  */        // C 2593
  if ! (*( (*f).output_data.android as *mut android_output )).white_relief.gc.is_null ()   // C 2594
    {
      android_free_gc ((*( (*f).output_data.android as *mut android_output )).white_relief.gc);   // C 2596
      (*( (*f).output_data.android as *mut android_output )).white_relief.gc = std::ptr::null_mut ();   // C 2597
    }
  if ! (*( (*f).output_data.android as *mut android_output )).black_relief.gc.is_null ()   // C 2599
    {
      android_free_gc ((*( (*f).output_data.android as *mut android_output )).black_relief.gc);   // C 2601
      (*( (*f).output_data.android as *mut android_output )).black_relief.gc = std::ptr::null_mut ();   // C 2602
    }

  if f == (*dpyinfo).focus_frame                                      // C 2605
    { (*dpyinfo).focus_frame = std::ptr::null_mut (); }                 // C 2606
  if f == (*dpyinfo).x_focus_event_frame                              // C 2607
    { (*dpyinfo).x_focus_event_frame = std::ptr::null_mut (); }         // C 2608
  if f == (*dpyinfo).highlight_frame                                  // C 2609
    { (*dpyinfo).highlight_frame = std::ptr::null_mut (); }             // C 2610
  if f == (*hlinfo).mouse_face_mouse_frame as *mut frame                // C 2611
    { reset_mouse_highlight (hlinfo); }                                 // C 2612

  /* These two need to be freed now that they are used to compute the
     mouse position, I think.  */                                      // C 2614-2615
  if f == (*dpyinfo).last_mouse_motion_frame                          // C 2616
    { (*dpyinfo).last_mouse_motion_frame = std::ptr::null_mut (); }     // C 2617
  if f == (*dpyinfo).last_mouse_frame                                 // C 2618
    { (*dpyinfo).last_mouse_frame = std::ptr::null_mut (); }            // C 2619

  /* Free all tool presses currently active on this frame.  */           // C 2621
  next = (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).touch_points;   // C 2622
  while ! next.is_null ()                                               // C 2623
    {
      last = next;                                                      // C 2625
      next = (*next).next;                                              // C 2626
      xfree (last as *mut c_void);                                      // C 2627
    }

  /* Clear this in case unblock_input reads events.  */                  // C 2630
  (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).touch_points = std::ptr::null_mut ();   // C 2631

  unblock_input ();                                                     // C 2633
}

/* C 2636-2642.  `static`, and C 6779 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_delete_frame (f: *mut frame)
{
  android_free_frame_resources (f);                                     // C 2639
  xfree ((*f).output_data.android);                                     // C 2640
  (*f).output_data.android = std::ptr::null_mut ();                      // C 2641
}

/* C 2644-2648.  `static`, and C 6780 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_delete_terminal (terminal: *mut terminal)
{
  error (c"Cannot terminate connection to Android display server".as_ptr ());   // C 2647
}

/* C 2652-3067: the redisplay-interface group C 2652 calls "RIF functions" - the scroll, the
   window-line hook, the flush, the three clipping helpers, the fringe bitmap, and the glyph-string
   GC/clipping setters.  Three names this text needs are not in the frozen mirrors, so they stand at
   their use sites below (P-72 rule 4), and `window.h` 339-351's `pseudo_window_p` needs the
   per-unit accessor pair the four sibling window-system units carry (the `src/androidfns.rs`
   2953-2962 shape, which has the write arm only; this unit reads the member too).  */
/* The five records and the two `draw_fringe_bitmap_params` accessors this slice's text names live
   in `include/dispextern.rs` 2800-3684's `pub mod dispextern_unit_names` (behind
   `dispextern_face_records`), which no glob import brings into this unit - the `src/abi_shim.rs`
   2047 note's device.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* `draw_fringe_bitmap_params` (C 2159-2200) and its two `bool_bf` accessors - the mirror
   keeps them behind `dispextern_face_records`, so this unit carries its own copy (status
   3.220's ruling; the import below named a module that no configuration produces).  */
#[cfg (feature = "have_window_system")]
pub const DRAW_FRINGE_BITMAP_CURSOR_P: u8 = 1 << 0;
#[cfg (feature = "have_window_system")]
pub const DRAW_FRINGE_BITMAP_OVERLAY_P: u8 = 1 << 1;
#[inline(always)]
#[cfg (feature = "have_window_system")]
pub fn get_cursor_p (p: *const draw_fringe_bitmap_params) -> bool
{ unsafe { (*p).flags & DRAW_FRINGE_BITMAP_CURSOR_P != 0 } }
#[inline(always)]
#[cfg (feature = "have_window_system")]
pub fn set_cursor_p (p: *mut draw_fringe_bitmap_params, value: bool)
{
  unsafe { if value { (*p).flags |= DRAW_FRINGE_BITMAP_CURSOR_P }
           else { (*p).flags &= !DRAW_FRINGE_BITMAP_CURSOR_P } }
}
#[inline(always)]
#[cfg (feature = "have_window_system")]
pub fn get_overlay_p (p: *const draw_fringe_bitmap_params) -> bool
{ unsafe { (*p).flags & DRAW_FRINGE_BITMAP_OVERLAY_P != 0 } }
#[inline(always)]
#[cfg (feature = "have_window_system")]
pub fn set_overlay_p (p: *mut draw_fringe_bitmap_params, value: bool)
{
  unsafe { if value { (*p).flags |= DRAW_FRINGE_BITMAP_OVERLAY_P }
           else { (*p).flags &= !DRAW_FRINGE_BITMAP_OVERLAY_P } }
}

#[repr (C)]
#[derive (Clone, Copy)]
#[cfg (feature = "have_window_system")]
pub struct draw_fringe_bitmap_params
{
  pub which: c_int,                  // offset 0
  pub bits: *mut c_ushort,                  // offset 8
  pub wd: c_int,                  // offset 16
  pub h: c_int,                  // offset 20
  pub dh: c_int,                  // offset 24
  pub x: c_int,                  // offset 28
  pub y: c_int,                  // offset 32
  pub bx: c_int,                  // offset 36
  pub nx: c_int,                  // offset 40
  pub by: c_int,                  // offset 44
  pub ny: c_int,                  // offset 48
  /// C 2215-2240's bitfield run, stored as `flags` (PORTING.md 1.3): offset 52.
  pub flags: u8,
  pub face: *mut face,                  // offset 56
}
#[cfg (feature = "have_window_system")]
const _: () = core::assert! (core::mem::size_of::<draw_fringe_bitmap_params> () == 64);


#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! window_bits {
  (pseudo_window_p, $w:expr) => { window_bit ($w, WINDOW_PSEUDO_WINDOW_P_SHIFT) };
  (pseudo_window_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_PSEUDO_WINDOW_P_SHIFT) }; };
  /* C 4881's `w->phys_cursor_on_p = true;` - the mirror numbers the run
     (`include/window.rs` 191) and carries no reader, so the unit names the two arms
     (the `src/haikuterm.rs` 3377-3400 and `src/pgtkterm.rs` 2875-2882 shape).  */
  (phys_cursor_on_p, $w:expr) => { window_bit ($w, WINDOW_PHYS_CURSOR_ON_P_SHIFT) };
  (phys_cursor_on_p, $w:expr, $value:expr) =>
    { (*($w)).bits = if $value { (*($w)).bits | (1u32 << WINDOW_PHYS_CURSOR_ON_P_SHIFT) }
        else { (*($w)).bits & !(1u32 << WINDOW_PHYS_CURSOR_ON_P_SHIFT) }; };
}

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3574-3575: `void window_box (struct window *, enum glyph_row_area,
  /// int *, int *, int *, int *);`
  pub fn window_box (w: *mut window, area: glyph_row_area, x: *mut c_int, y: *mut c_int,
		     width: *mut c_int, height: *mut c_int);
  /// C: dispextern.h 3657: `extern void gui_clear_cursor (struct window *);` (xdisp.c)
  pub fn gui_clear_cursor (w: *mut window);
  /// C: dispextern.h 3810: `void prepare_face_for_display (struct frame *, struct face *);`
  pub fn prepare_face_for_display (f: *mut frame, face: *mut face);
  /// C: dispextern.h 3661-3662: `extern int get_glyph_string_clip_rects (struct glyph_string *,
  /// NativeRectangle *, int);` - `NativeRectangle` is this unit's `android_rectangle`
  /// (androidgui.h 218; the mirror's own note, `include/dispextern.rs` 1733-1741).
  pub fn get_glyph_string_clip_rects (s: *mut glyph_string, nr: *mut android_rectangle,
				      max: c_int) -> c_int;
}

/* C 2654-2711.  `static`, and C 6672 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_scroll_run (w: *mut window, run: *mut run)
{
  let f: *mut frame = XFRAME ((*w).frame);                              // C 2657
  let mut x: c_int = 0;                                                 // C 2658
  let mut y: c_int = 0;
  let mut width: c_int = 0;
  let mut height: c_int = 0;
  let mut from_y: c_int = 0;
  let mut to_y: c_int = 0;
  let mut bottom_y: c_int = 0;

  /* Get frame-relative bounding box of the text display area of W,
     without mode lines.  Include in this box the left and right
     fringe of W.  */                                                   // C 2660-2662
  window_box (w, ANY_AREA, &raw mut x, &raw mut y, &raw mut width, &raw mut height);   // C 2663

  from_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*run).current_y);              // C 2665
  to_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*run).desired_y);                // C 2666
  bottom_y = y + height;                                                // C 2667

  if to_y < from_y                                                     // C 2669
    {
      /* Scrolling up.  Make sure we don't copy part of the mode
	 line at the bottom.  */                                        // C 2671-2672
      if from_y + (*run).height > bottom_y                             // C 2673
	{ height = bottom_y - from_y; }                                // C 2674
      else
	{ height = (*run).height; }                                    // C 2676

      /* Don't draw over the display parts above if destination is off
         the top of the current window.  */                           // C 2678-2679
      if to_y < y                                                      // C 2680
	{
	  let d: c_int = y - to_y;                                      // C 2682
	  height -= d;                                                  // C 2683
	  to_y += d;                                                    // C 2684
	  from_y += d;                                                  // C 2685
	}
    }
  else
    {
      /* Scrolling down.  Make sure we don't copy over the mode line.
	 at the bottom.  */                                            // C 2690-2691
      if to_y + (*run).height > bottom_y                               // C 2692
	{ height = bottom_y - to_y; }                                  // C 2693
      else
	{ height = (*run).height; }                                    // C 2695
    }

  block_input ();                                                       // C 2698

  /* Cursor off.  Will be switched on again in gui_update_window_end.  */   // C 2700
  gui_clear_cursor (w);                                                 // C 2701

  /* To avoid sequence point problems, make sure to only call
     FRAME_ANDROID_DRAWABLE once.  */                                  // C 2703-2704
  android_copy_area (FRAME_ANDROID_DRAWABLE! (f),                       // C 2705
		     FRAME_ANDROID_WINDOW! (f),                        // C 2706
		     (*( (*f).output_data.android as *mut android_output )).normal_gc,   // C 2707
		     x, from_y, width as c_uint, height as c_uint, x, to_y);   // C 2708

  unblock_input ();                                                     // C 2710
}

/* C 2713-2720.  `static`, and C 6673 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_after_update_window_line (w: *mut window,
						       desired_row: *mut glyph_row)
{
  eassert! (! w.is_null ());                                            // C 2716

  if ! glyph_row_bits! (mode_line_p, desired_row)                       // C 2718
      && ! window_bits! (pseudo_window_p, w)                            // C 2718
    { glyph_row_bits! (redraw_fringe_bitmaps_p, desired_row, true); }    // C 2719
}

/* C 2722-2734.  `static`, and C 6676 takes its address.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_flip_and_flush (f: *mut frame)
{
  block_input ();                                                       // C 2725

  if FRAME_ANDROID_NEED_BUFFER_FLIP! (f)                                // C 2727
    { show_back_buffer (f); }                                          // C 2728

  /* The frame is complete again as its contents were just
     flushed.  */                                                      // C 2730-2731
  (*( (*f).output_data.android as *mut android_output )).bits |= ANDROID_OUTPUT_COMPLETE;   // C 2732
  unblock_input ();                                                     // C 2733
}

/* C 2736-2749.  `static`; called from C 3102, so a plain Rust-ABI item.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_clear_rectangle (f: *mut frame, gc: *mut android_gc, x: c_int,
				   y: c_int, width: c_int, height: c_int)
{
  let mut xgcv: android_gc_values = std::mem::zeroed ();                // C 2740

  android_get_gc_values (gc, (ANDROID_GC_BACKGROUND                    // C 2742
			      | ANDROID_GC_FOREGROUND),                // C 2743
			 &raw mut xgcv);                               // C 2744
  android_set_foreground (gc, xgcv.background);                        // C 2745
  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,             // C 2746
			  x, y, width as c_uint, height as c_uint);    // C 2747
  android_set_foreground (gc, xgcv.foreground);                        // C 2748
}

/* C 2751-2755.  `static`; called from C 2894/3317/3318.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_reset_clip_rectangles (f: *mut frame, gc: *mut android_gc)
{
  android_set_clip_mask (gc, ANDROID_NONE as android_pixmap);                            // C 2754
}

/* C 2757-2777.  `static`; called from C 2789/4762/4820.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_clip_to_row (w: *mut window, row: *mut glyph_row,
			       area: glyph_row_area, gc: *mut android_gc,
			       rect_return: *mut android_rectangle)
{
  let mut clip_rect: android_rectangle = std::mem::zeroed ();           // C 2762
  let mut window_x: c_int = 0;                                         // C 2763
  let mut window_y: c_int = 0;
  let mut window_width: c_int = 0;

  window_box (w, area, &raw mut window_x, &raw mut window_y, &raw mut window_width,
	      std::ptr::null_mut ());                                   // C 2765

  clip_rect.x = window_x;                                              // C 2767
  clip_rect.y = WINDOW_TO_FRAME_PIXEL_Y! (w, max (0, (*row).y)) as c_int;   // C 2768
  clip_rect.y = max (clip_rect.y, window_y) as c_int;                   // C 2769
  clip_rect.width = window_width as c_uint;                             // C 2770
  clip_rect.height = (*row).visible_height as c_uint;                   // C 2771

  android_set_clip_rectangles (gc, 0, 0, &raw mut clip_rect, 1);        // C 2773

  if ! rect_return.is_null ()                                          // C 2775
    { *rect_return = clip_rect; }                                      // C 2776
}

/* C 2779-2895.  `static`, and C 6680 takes its address.  C's `undo_clip:` label is the port's
   labelled block (`break 'undo_clip` at C 2844), and `p->overlay_p`/`p->cursor_p` are the mirror's
   `get_overlay_p`/`get_cursor_p` accessors (the `src/xterm.rs` 8761-8784 device).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_draw_fringe_bitmap (w: *mut window, row: *mut glyph_row,
						 p: *mut draw_fringe_bitmap_params)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                       // C 2783
  let gc: *mut android_gc = (*( (*f).output_data.android as *mut android_output )).normal_gc;   // C 2784
  let face: *mut face = (*p).face;                                      // C 2785
  let mut clip_rect: android_rectangle = std::mem::zeroed ();           // C 2786

  /* Must clip because of partially visible lines.  */                   // C 2788
  android_clip_to_row (w, row, ANY_AREA, gc, &raw mut clip_rect);        // C 2789

  if (*p).bx >= 0 && ! get_overlay_p (p)                               // C 2791
    {
      /* In case the same realized face is used for fringes and for
	 something displayed in the text (e.g. face `region' on
	 mono-displays, the fill style may have been changed to
	 ANDROID_FILL_SOLID in
	 android_draw_glyph_string_background.  */                      // C 2793-2797
      if (*face).stipple != 0                                          // C 2798
	{
	  android_set_fill_style ((*face).gc as *mut android_gc, ANDROID_FILL_OPAQUE_STIPPLED);   // C 2800
	  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), (*face).gc as *mut android_gc,   // C 2801
				  (*p).bx, (*p).by, (*p).nx as c_uint, (*p).ny as c_uint);   // C 2802
	  android_set_fill_style ((*face).gc as *mut android_gc, ANDROID_FILL_SOLID);   // C 2803

	  glyph_row_bits! (stipple_p, row, true);                       // C 2805
	}
      else
	{
	  android_set_background ((*face).gc as *mut android_gc, (*face).background);   // C 2809
	  android_clear_rectangle (f, (*face).gc as *mut android_gc, (*p).bx, (*p).by, (*p).nx, (*p).ny);   // C 2810
	  android_set_foreground ((*face).gc as *mut android_gc, (*face).foreground);   // C 2811
	}
    }

  'undo_clip: {
  if (*p).which != 0                                                   // C 2815
    {
      let mut drawable: android_drawable;                               // C 2817
      let mut bits: *mut c_char;
      let mut pixmap: android_pixmap;                                   // C 2819
      let mut clipmask: android_pixmap;
      let mut gcv: android_gc_values = std::mem::zeroed ();             // C 2820
      let mut background: c_ulong = 0;                                  // C 2821
      let mut cursor_pixel: c_ulong = 0;
      let mut depth: c_int = 0;                                         // C 2822
      let mut image_rect: android_rectangle = std::mem::zeroed ();      // C 2823
      let mut dest: android_rectangle = std::mem::zeroed ();
      let mut px: c_int = 0;                                            // C 2824
      let mut py: c_int = 0;
      let mut pwidth: c_int = 0;
      let mut pheight: c_int = 0;

      drawable = FRAME_ANDROID_DRAWABLE! (f);                           // C 2826
      clipmask = ANDROID_NONE as android_pixmap;                                          // C 2827
      background = (*face).background;                                  // C 2828
      cursor_pixel = (*( (*f).output_data.android as *mut android_output )).cursor_pixel;   // C 2829
      depth = (*FRAME_DISPLAY_INFO! (f)).n_image_planes;                 // C 2830

      /* Intersect the destination rectangle with that of the row.
	 Setting a clip mask overrides the clip rectangles provided by
	 android_clip_to_row, so clipping must be performed by
	 hand.  */                                                      // C 2832-2835

      image_rect.x = (*p).x;                                            // C 2837
      image_rect.y = (*p).y;                                            // C 2838
      image_rect.width = (*p).wd as c_uint;                             // C 2839
      image_rect.height = (*p).h as c_uint;                             // C 2840

      if ! gui_intersect_rectangles (&raw const clip_rect as *const Emacs_Rectangle,
				&raw const image_rect as *const Emacs_Rectangle,
				&raw mut dest as *mut Emacs_Rectangle)   // C 2842
	/* The entire destination rectangle falls outside the row.  */    // C 2843
	{ break 'undo_clip; }                                            // C 2844

      /* Extrapolate the source rectangle from the difference between
	 the destination and image rectangles.  */                      // C 2846-2847

      px = dest.x - image_rect.x;                                       // C 2849
      py = dest.y - image_rect.y;                                       // C 2850
      pwidth = dest.width as c_int;                                     // C 2851
      pheight = dest.height as c_int;                                   // C 2852

      if (*p).wd > 8                                                    // C 2854
	{ bits = ((*p).bits.offset ((*p).dh as isize)) as *mut c_char; }   // C 2855
      else
	{ bits = ((*p).bits as *mut c_char).offset ((*p).dh as isize); }   // C 2857

      pixmap = android_create_pixmap_from_bitmap_data (bits, (*p).wd as c_uint,
						       (*p).h as c_uint,   // C 2859
						       (if get_cursor_p (p)   // C 2860
							  { if get_overlay_p (p)   // C 2861
							      { (*face).background }   // C 2862
							    else { cursor_pixel } }   // C 2863
							else { (*face).foreground }),   // C 2864
						       background, depth as c_uint);   // C 2865

      if get_overlay_p (p)                                             // C 2867
	{
	  clipmask = android_create_pixmap_from_bitmap_data (bits, (*p).wd as c_uint,
							      (*p).h as c_uint,   // C 2869
							     1, 0, 1);   // C 2870

	  gcv.clip_mask = clipmask;                                      // C 2872
	  gcv.clip_x_origin = (*p).x;                                    // C 2873
	  gcv.clip_y_origin = (*p).y;                                    // C 2874
	  android_change_gc (gc, (ANDROID_GC_CLIP_MASK                   // C 2875
				  | ANDROID_GC_CLIP_X_ORIGIN             // C 2876
				  | ANDROID_GC_CLIP_Y_ORIGIN),           // C 2877
			     &raw mut gcv);                              // C 2878
	}

      android_copy_area (pixmap, drawable, gc, px, py,                   // C 2881
			 pwidth as c_uint, pheight as c_uint, dest.x, dest.y);   // C 2882
      android_free_pixmap (pixmap);                                     // C 2883

      if get_overlay_p (p)                                             // C 2885
	{
	  gcv.clip_mask = ANDROID_NONE as android_pixmap;                                  // C 2887
	  android_change_gc (gc, ANDROID_GC_CLIP_MASK, &raw mut gcv);     // C 2888
	  android_free_pixmap (clipmask);                                // C 2889
	}
    }
  }

  /* undo_clip: */
  android_reset_clip_rectangles (f, gc);                                // C 2894
}

/* Set S->gc to a suitable GC for drawing glyph string S in cursor
   face.  */                                                            // C 2897-2898

/* C 2900-2944.  `static`; only called (C 3011).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_cursor_gc (s: *mut glyph_string)
{
  if (*s).font == FRAME_FONT! ((*s).f)                                  // C 2903
      && (*( (*s).face as *mut face )).background == FRAME_BACKGROUND_PIXEL ((*s).f)   // C 2904
      && (*( (*s).face as *mut face )).foreground == FRAME_FOREGROUND_PIXEL ((*s).f)   // C 2905
      && ((*s).cmp).is_null ()                                          // C 2906
    { (*s).gc = (*( (*(*s).f).output_data.android as *mut android_output )).cursor_gc as *mut c_void; }   // C 2907
  else
    {
      /* Cursor on non-default face: must merge.  */                    // C 2910
      let mut xgcv: android_gc_values = std::mem::zeroed ();            // C 2911
      let mut mask: android_gc_value_mask;                                           // C 2912

      xgcv.background = (*( (*(*s).f).output_data.android as *mut android_output )).cursor_pixel;   // C 2914
      xgcv.foreground = (*( (*s).face as *mut face )).background;        // C 2915

      /* If the glyph would be invisible, try a different foreground.  */   // C 2917
      if xgcv.foreground == xgcv.background                             // C 2918
	{ xgcv.foreground = (*( (*s).face as *mut face )).foreground; }   // C 2919
      if xgcv.foreground == xgcv.background                             // C 2920
	{ xgcv.foreground = (*( (*(*s).f).output_data.android as *mut android_output )).cursor_foreground_pixel; }   // C 2921
      if xgcv.foreground == xgcv.background                             // C 2922
	{ xgcv.foreground = (*( (*s).face as *mut face )).foreground; }   // C 2923

      /* Make sure the cursor is distinct from text in this face.  */    // C 2925
      if xgcv.background == (*( (*s).face as *mut face )).background     // C 2926
	  && xgcv.foreground == (*( (*s).face as *mut face )).foreground   // C 2927
	{
	  xgcv.background = (*( (*s).face as *mut face )).foreground;      // C 2929
	  xgcv.foreground = (*( (*s).face as *mut face )).background;      // C 2930
	}

      mask = ANDROID_GC_FOREGROUND | ANDROID_GC_BACKGROUND;   // C 2933

      if ! (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc.is_null ()   // C 2935
	{ android_change_gc ((*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc,   // C 2936
			     mask, &raw mut xgcv); }                     // C 2937
      else
	{ (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc              // C 2939
	    = android_create_gc (mask, &raw mut xgcv); }                 // C 2940

      (*s).gc = (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc as *mut c_void;   // C 2942
    }
}

/* Set up S->gc of glyph string S for drawing text in mouse face.  */    // C 2947

/* C 2949-2977.  `static`; only called (C 3016).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_mouse_face_gc (s: *mut glyph_string)
{
  if (*s).font == (*( (*s).face as *mut face )).font as *mut font      // C 2952
    { (*s).gc = (*( (*s).face as *mut face )).gc; }                     // C 2953
  else
    {
      /* Otherwise construct scratch_cursor_gc with values from FACE
	 except for FONT.  */                                          // C 2956-2957
      let mut xgcv: android_gc_values = std::mem::zeroed ();            // C 2958
      let mut mask: android_gc_value_mask;                                           // C 2959

      xgcv.background = (*( (*s).face as *mut face )).background;        // C 2961
      xgcv.foreground = (*( (*s).face as *mut face )).foreground;        // C 2962

      mask = ANDROID_GC_FOREGROUND | ANDROID_GC_BACKGROUND;   // C 2964

      if ! (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc.is_null ()   // C 2966
	{ android_change_gc ((*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc,   // C 2967
			     mask, &raw mut xgcv); }                     // C 2968
      else
	{ (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc              // C 2970
	    = android_create_gc (mask, &raw mut xgcv); }                 // C 2971

      (*s).gc = (*FRAME_DISPLAY_INFO! ((*s).f)).scratch_cursor_gc as *mut c_void;   // C 2973
    }

  eassert! (! (*s).gc.is_null ());                                      // C 2976
}

/* Set S->gc of glyph string S to a GC suitable for drawing a mode line.
   Faces to use in the mode line have already been computed when the
   matrix was built, so there isn't much to do, here.  */                // C 2980-2982

/* C 2984-2988.  `static`; only called (C 3006).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_mode_line_face_gc (s: *mut glyph_string)
{
  (*s).gc = (*( (*s).face as *mut face )).gc;                           // C 2987
}

/* Set S->gc of glyph string S for drawing that glyph string.  Set
   S->stippled_p to a non-zero value if the face of S has a stipple
   pattern.  */                                                         // C 2990-2992

/* C 2994-3030.  `static`; only called (C 4300/4311/4596).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_glyph_string_gc (s: *mut glyph_string)
{
  prepare_face_for_display ((*s).f, (*s).face as *mut face);            // C 2997

  if (*s).hl == DRAW_NORMAL_TEXT                                       // C 2999
    {
      (*s).gc = (*( (*s).face as *mut face )).gc;                       // C 3001
      glyph_string_bits! (stippled_p, (*s), (*( (*s).face as *mut face )).stipple != 0);   // C 3002
    }
  else if (*s).hl == DRAW_INVERSE_VIDEO                                // C 3004
    {
      android_set_mode_line_face_gc (s);                                // C 3006
      glyph_string_bits! (stippled_p, (*s), (*( (*s).face as *mut face )).stipple != 0);   // C 3007
    }
  else if (*s).hl == DRAW_CURSOR                                       // C 3009
    {
      android_set_cursor_gc (s);                                        // C 3011
      glyph_string_bits! (stippled_p, (*s), false);                     // C 3012
    }
  else if (*s).hl == DRAW_MOUSE_FACE                                   // C 3014
    {
      android_set_mouse_face_gc (s);                                    // C 3016
      glyph_string_bits! (stippled_p, (*s), (*( (*s).face as *mut face )).stipple != 0);   // C 3017
    }
  else if (*s).hl == DRAW_IMAGE_RAISED                                 // C 3019
	   || (*s).hl == DRAW_IMAGE_SUNKEN                              // C 3020
    {
      (*s).gc = (*( (*s).face as *mut face )).gc;                       // C 3022
      glyph_string_bits! (stippled_p, (*s), (*( (*s).face as *mut face )).stipple != 0);   // C 3023
    }
  else
    { emacs_abort (); }                                                // C 3026

  /* GC must have been set.  */                                         // C 3028
  eassert! (! (*s).gc.is_null ());                                      // C 3029
}

/* Set clipping for output of glyph string S.  S may be part of a mode
   line or menu if we don't have X toolkit support.  */                  // C 3033-3034

/* C 3036-3045.  `static`; only called (C 3790/4301/4321).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_glyph_string_clipping (s: *mut glyph_string)
{
  /* C 3039's `struct android_rectangle *r = s->clip;` reads the array through the degraded
     `NativeRectangle` pointer - the `src/xterm.rs` 9687-9696 device.  */
  let r: *mut android_rectangle = &raw mut (*s).clip as *mut [c_int; 2] as *mut android_rectangle;   // C 3039
  let n: c_int = get_glyph_string_clip_rects (s, r, 2);                 // C 3040

  if n > 0                                                             // C 3042
    { android_set_clip_rectangles ((*s).gc as *mut android_gc, 0, 0, r, n); }   // C 3043
  (*s).num_clips = n;                                                  // C 3044
}

/* Set SRC's clipping for output of glyph string DST.  This is called
   when we are drawing DST's left_overhang or right_overhang only in
   the area of SRC.  */                                                  // C 3048-3050

/* C 3052-3065.  `static`; only called (C 4334/4597).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_set_glyph_string_clipping_exactly (src: *mut glyph_string,
						     dst: *mut glyph_string)
{
  let mut r: android_rectangle = std::mem::zeroed ();                   // C 3056

  r.x = (*src).x;                                                       // C 3058
  r.width = (*src).width as c_uint;                                     // C 3059
  r.y = (*src).y;                                                       // C 3060
  r.height = (*src).height as c_uint;                                   // C 3061
  /* C 3062's `dst->clip[0] = r;` stores an `android_rectangle` through the array: `NativeRectangle`
     is `Emacs_Rectangle` on Android (androidgui.h 218) while the mirror keeps the fallback
     `int [2]` (`include/dispextern.rs` 1732-1741, whose note names the integrator's call), so this
     is the `src/xterm.rs` 9695-9696 device.  */
  *((&raw mut (*dst).clip) as *mut android_rectangle).add (0) = r;       // C 3062
  (*dst).num_clips = 1;                                                 // C 3063
  android_set_clip_rectangles ((*dst).gc as *mut android_gc, 0, 0, &raw mut r, 1);   // C 3064
}

/* C 3068-3357: the glyph-string drawing helpers - the overhang computation, the background fill,
   the relief/box helpers with their two local geometry helpers, and the relief colour constant.
   `composition_gstring_from_id`/`composition_gstring_width` come from the mirror's
   `composite_decls` glob import (`include/composite.rs` 411-471, reached through C 34's coding.h
   edge), so they stay bare here - the `src/xterm.rs` 9727-9731 device, and
   `tools/check-mirror-dupes.py` is what asks for that (one declaration per name, in the mirror).  */

/* C 3068-3096.  `static`; only called (C 4312).  `s->first_glyph->type` is the glyph record's
   bitfield run read through the mirror's `glyph_bits!`, and the two `font->driver->text_extents`
   calls are the unwrapped function-pointer field of the mirror's `font_driver`
   (`include/font.rs` 134 keeps `driver` untyped, P-72 rule 3; the `src/xterm.rs` 9704-9736
   device).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* C 6664-6696's table takes this function's address, so the port declares it
   `unsafe extern "C" fn` - the C-ABI rule for an address-taken item (slice 14-onward's
   `android_sync_edit` shape).  */
unsafe extern "C" fn android_compute_glyph_string_overhangs (s: *mut glyph_string)
{
  if ((*s).cmp).is_null ()                                             // C 3070
      && (glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH        // C 3071
	  || glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH)   // C 3072
    {
      let mut metrics: font_metrics = std::mem::zeroed ();              // C 3074

      if glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH          // C 3076
	{
	  let font: *mut font = (*s).font;                               // C 3078
	  ((*((*font).driver as *const font_driver)).text_extents.unwrap ())   // C 3079
	    (font, (*s).char2b, (*s).nchars, &raw mut metrics);
	}
      else
	{
	  let gstring: Lisp_Object = composition_gstring_from_id ((*s).cmp_id);   // C 3083

	  composition_gstring_width (gstring, (*s).cmp_from as ptrdiff_t,   // C 3085
				     (*s).cmp_to as ptrdiff_t,
				     &raw mut metrics as *mut c_void);
	}
      (*s).right_overhang = if metrics.rbearing > metrics.width        // C 3087
	{ (metrics.rbearing - metrics.width) as c_int } else { 0 };     // C 3088
      (*s).left_overhang = if metrics.lbearing < 0                     // C 3089
	{ (- metrics.lbearing) as c_int } else { 0 }; 
    }
  else if ! ((*s).cmp).is_null ()                                       // C 3091
    {
      (*s).right_overhang = ((*( (*s).cmp as *mut composition )).rbearing    // C 3093
	- (*( (*s).cmp as *mut composition )).pixel_width) as c_int; 
(*s).left_overhang = (- (*( (*s).cmp as *mut composition )).lbearing) as c_int; // C 3094
    }
}

/* C 3098-3103.  `static`; only called (C 3135).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_clear_glyph_string_rect (s: *mut glyph_string, x: c_int, y: c_int,
					   w: c_int, h: c_int)
{
  android_clear_rectangle ((*s).f, (*s).gc as *mut android_gc, x, y, w, h);   // C 3102
}

/* C 3105-3141.  `static`; only called (C 2999/4308).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_glyph_string_background (s: *mut glyph_string, force_p: bool)
{
  /* Nothing to do if background has already been drawn or if it
     shouldn't be drawn in the first place.  */                        // C 3108-3109
  if ! glyph_string_bits! (background_filled_p, (*s))                   // C 3110
    {
      let box_line_width: c_int = max ((*( (*s).face as *mut face )).box_horizontal_line_width, 0);   // C 3112

      if glyph_string_bits! (stippled_p, (*s))                          // C 3114
	{
	  /* Fill background with a stipple pattern.  */                // C 3116
	  android_set_fill_style ((*s).gc as *mut android_gc, ANDROID_FILL_OPAQUE_STIPPLED);   // C 3117
	  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc,   // C 3118
				  (*s).x, (*s).y + box_line_width,     // C 3119
				  (*s).background_width as c_uint,     // C 3120
				  ((*s).height - 2 * box_line_width) as c_uint); // C 3121
	  android_set_fill_style ((*s).gc as *mut android_gc, ANDROID_FILL_SOLID);   // C 3122
	  glyph_string_bits! (background_filled_p, (*s), true);            // C 3123
	}
      else if FONT_HEIGHT! ((*s).font) < (*s).height - 2 * box_line_width   // C 3125
	       /* When xdisp.c ignores FONT_HEIGHT, we cannot trust
		  font dimensions, since the actual glyphs might be
		  much smaller.  So in that case we always clear the
		  rectangle with background color.  */                 // C 3126-3129
	       || FONT_TOO_HIGH! ((*s).font)                           // C 3130
	       || glyph_string_bits! (font_not_found_p, (*s))          // C 3131
	       || glyph_string_bits! (extends_to_end_of_line_p, (*s))   // C 3132
	       || force_p                                              // C 3133
	{
	  android_clear_glyph_string_rect (s, (*s).x, (*s).y + box_line_width,   // C 3135
					   (*s).background_width,      // C 3136
					   (*s).height - 2 * box_line_width);   // C 3137
	  glyph_string_bits! (background_filled_p, (*s), true);            // C 3138
	}
    }
}

/* C 3143-3158.  `static`; only called (C 3257/3264).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_fill_triangle (f: *mut frame, gc: *mut android_gc,
				 point1: android_point, point2: android_point,
				 point3: android_point)
{
  let mut abc: [android_point; 3] = std::mem::zeroed ();                // C 3149

  abc[0] = point1;                                                      // C 3151
  abc[1] = point2;                                                      // C 3152
  abc[2] = point3;                                                      // C 3153

  android_fill_polygon (FRAME_ANDROID_DRAWABLE! (f),                    // C 3155
			gc, abc.as_mut_ptr (), 3, ANDROID_CONVEX,      // C 3156
			ANDROID_COORD_MODE_ORIGIN);                   // C 3157
}

/* C 3160-3169.  `static`; only called (C 3258-3268).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_make_point (x: c_int, y: c_int) -> android_point
{
  let mut pt: android_point = std::mem::zeroed ();                      // C 3163

  pt.x = x;                                                             // C 3165
  pt.y = y;                                                             // C 3166

  return pt;                                                            // C 3168
}

/* C 3171-3186.  `static`; only called (C 3300-3313).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_inside_rect_p (rects: *mut android_rectangle, nrects: c_int, x: c_int,
				 y: c_int) -> bool
{
  let mut i: c_int;                                                     // C 3175

  i = 0;                                                                // C 3177
  while i < nrects
    {
      if x >= (*rects.offset (i as isize)).x as c_int                   // C 3179
	  && y >= (*rects.offset (i as isize)).y as c_int               // C 3179
	  && x < ((*rects.offset (i as isize)).x + (*rects.offset (i as isize)).width as c_int)   // C 3180
	  && y < ((*rects.offset (i as isize)).y + (*rects.offset (i as isize)).height as c_int)   // C 3181
	{ return true; }                                                // C 3182
      i += 1;                                                           // C 3177 (the `for` step)
    }

  return false;                                                         // C 3185
}

/* C 3188-3199.  `static`; only called (C 3302-3314).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_clear_point (f: *mut frame, gc: *mut android_gc, x: c_int, y: c_int)
{
  let mut xgcv: android_gc_values = std::mem::zeroed ();                // C 3192

  android_get_gc_values (gc, ANDROID_GC_BACKGROUND | ANDROID_GC_FOREGROUND,   // C 3194
			 &raw mut xgcv);                               // C 3195
  android_set_foreground (gc, xgcv.background);                          // C 3196
  android_draw_point (FRAME_ANDROID_DRAWABLE! (f), gc, x, y);             // C 3197
  android_set_foreground (gc, xgcv.foreground);                          // C 3198
}

/* C 3201-3319.  `static`; only called (C 3510).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_relief_rect (f: *mut frame, left_x: c_int, top_y: c_int,
				    right_x: c_int, bottom_y: c_int, hwidth: c_int,
				    vwidth: c_int, raised_p: bool, top_p: bool,
				    bot_p: bool, left_p: bool, right_p: bool,
				    clip_rect: *mut android_rectangle)
{
  let mut gc: *mut android_gc;                                          // C 3207
  let mut white_gc: *mut android_gc;
  let mut black_gc: *mut android_gc;
  let mut normal_gc: *mut android_gc;
  let mut drawable: android_drawable;                                   // C 3208

  /* This code is more complicated than it has to be, because of two
     minor hacks to make the boxes look nicer: (i) if width > 1, draw
     the outermost line using the black relief.  (ii) Omit the four
     corner pixels.  */                                                 // C 3210-3213

  white_gc = (*( (*f).output_data.android as *mut android_output )).white_relief.gc;   // C 3215
  black_gc = (*( (*f).output_data.android as *mut android_output )).black_relief.gc;   // C 3216
  normal_gc = (*( (*f).output_data.android as *mut android_output )).normal_gc;   // C 3217

  drawable = FRAME_ANDROID_DRAWABLE! (f);                               // C 3219

  android_set_clip_rectangles (white_gc, 0, 0, clip_rect, 1);            // C 3221
  android_set_clip_rectangles (black_gc, 0, 0, clip_rect, 1);            // C 3222

  if raised_p                                                           // C 3224
    { gc = white_gc; }                                                  // C 3225
  else
    { gc = black_gc; }                                                  // C 3227

  /* Draw lines.  */                                                    // C 3229

  if top_p                                                              // C 3231
    { android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc, left_x, top_y,   // C 3232
			      (right_x - left_x + 1) as c_uint, hwidth as c_uint); } // C 3233

  if left_p                                                             // C 3235
    { android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc, left_x, top_y,   // C 3236
			      vwidth as c_uint, (bottom_y - top_y + 1) as c_uint); } // C 3237

  if raised_p                                                           // C 3239
    { gc = black_gc; }                                                  // C 3240
  else
    { gc = white_gc; }                                                  // C 3242

  if bot_p                                                              // C 3244
    { android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc, left_x,   // C 3245
			      bottom_y - hwidth + 1,                    // C 3246
			      (right_x - left_x + 1) as c_uint, hwidth as c_uint); } // C 3247

  if right_p                                                            // C 3249
    { android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc,          // C 3250
			      right_x - vwidth + 1,                     // C 3251
			      top_y, vwidth as c_uint, (bottom_y - top_y + 1) as c_uint); } // C 3252

  /* Draw corners.  */                                                  // C 3254

  if bot_p && left_p                                                    // C 3256
    { android_fill_triangle (f, if raised_p { white_gc } else { black_gc },   // C 3257
			     android_make_point (left_x, bottom_y - hwidth),   // C 3258
			     android_make_point (left_x + vwidth,       // C 3259
						 bottom_y - hwidth),   // C 3260
			     android_make_point (left_x, bottom_y)); }   // C 3261

  if top_p && right_p                                                   // C 3263
    { android_fill_triangle (f, if raised_p { white_gc } else { black_gc },   // C 3264
			     android_make_point (right_x - vwidth, top_y),   // C 3265
			     android_make_point (right_x, top_y),       // C 3266
			     android_make_point (right_x - vwidth,      // C 3267
						 top_y + hwidth)); }   // C 3268

  /* Draw outer line.  */                                               // C 3270

  if top_p && left_p && bot_p && right_p                                // C 3272
      && hwidth > 1 && vwidth > 1                                       // C 3273
    { android_draw_rectangle (FRAME_ANDROID_DRAWABLE! (f),               // C 3274
			      black_gc, left_x, top_y,                  // C 3275
			      (right_x - left_x) as c_uint, (bottom_y - top_y) as c_uint); } // C 3276
  else
    {
      if top_p && hwidth > 1                                            // C 3279
	{ android_draw_line (drawable, black_gc, left_x, top_y,          // C 3280
			     right_x + 1, top_y); }                    // C 3281

      if bot_p && hwidth > 1                                            // C 3283
	{ android_draw_line (drawable, black_gc, left_x, bottom_y,       // C 3284
			     right_x + 1, bottom_y); }                 // C 3285

      if left_p && vwidth > 1                                           // C 3287
	{ android_draw_line (drawable, black_gc, left_x, top_y,          // C 3288
			     left_x, bottom_y + 1); }                  // C 3289

      if right_p && vwidth > 1                                          // C 3291
	{ android_draw_line (drawable, black_gc, right_x, top_y,         // C 3292
			     right_x, bottom_y + 1); }                 // C 3293
    }

  /* Erase corners.  */                                                 // C 3296

  if hwidth > 1 && vwidth > 1                                           // C 3298
    {
      if left_p && top_p && android_inside_rect_p (clip_rect, 1,       // C 3300
						   left_x, top_y)      // C 3301
	{ android_clear_point (f, normal_gc, left_x, top_y); }          // C 3302

      if left_p && bot_p && android_inside_rect_p (clip_rect, 1,       // C 3304
						   left_x, bottom_y)   // C 3305
	{ android_clear_point (f, normal_gc, left_x, bottom_y); }       // C 3306

      if right_p && top_p && android_inside_rect_p (clip_rect, 1,      // C 3308
						    right_x, top_y)    // C 3309
	{ android_clear_point (f, normal_gc, right_x, top_y); }         // C 3310

      if right_p && bot_p && android_inside_rect_p (clip_rect, 1,      // C 3312
						    right_x, bottom_y)   // C 3313
	{ android_clear_point (f, normal_gc, right_x, bottom_y); }      // C 3314
    }

  android_reset_clip_rectangles (f, white_gc);                          // C 3317
  android_reset_clip_rectangles (f, black_gc);                          // C 3318
}

/* C 3321-3355.  `static`; only called (C 3514).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_box_rect (s: *mut glyph_string,
				 left_x: c_int, top_y: c_int, right_x: c_int, bottom_y: c_int,
				 hwidth: c_int, vwidth: c_int, left_p: bool, right_p: bool,
				 clip_rect: *mut android_rectangle)
{
  let mut xgcv: android_gc_values = std::mem::zeroed ();                // C 3327

  android_get_gc_values ((*s).gc as *mut android_gc, ANDROID_GC_FOREGROUND, &raw mut xgcv);   // C 3329
  android_set_foreground ((*s).gc as *mut android_gc, (*( (*s).face as *mut face )).box_color);   // C 3330
  android_set_clip_rectangles ((*s).gc as *mut android_gc, 0, 0, clip_rect, 1);   // C 3331

  /* Top.  */                                                           // C 3333
  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc, left_x,   // C 3334
			  top_y, (right_x - left_x + 1) as c_uint, hwidth as c_uint); // C 3335

  /* Left.  */                                                          // C 3337
  if left_p                                                             // C 3338
    { android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc, left_x,   // C 3339
			      top_y, vwidth as c_uint, (bottom_y - top_y + 1) as c_uint); } // C 3340

  /* Bottom.  */                                                        // C 3342
  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc, left_x,   // C 3343
			  bottom_y - hwidth + 1, (right_x - left_x + 1) as c_uint, // C 3344
			  hwidth as c_uint);                            // C 3345

  /* Right.  */                                                         // C 3347
  if right_p                                                            // C 3348
    { android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc,   // C 3349
			      right_x - vwidth + 1, top_y, vwidth as c_uint, // C 3350
			      (bottom_y - top_y + 1) as c_uint); }      // C 3351

  android_set_foreground ((*s).gc as *mut android_gc, xgcv.foreground);   // C 3353
  android_reset_clip_rectangles ((*s).f, (*s).gc as *mut android_gc);     // C 3354
}

/* C 3357: `#define HIGHLIGHT_COLOR_DARK_BOOST_LIMIT 48000` - C's `int` constant (the missing
   `missing macro`/`missing constant` pair of the inventory row).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const HIGHLIGHT_COLOR_DARK_BOOST_LIMIT: c_int = 48000;              // C 3357

/* C 3360-3580: the relief-colour group (the lighter-colour allocator, the two relief setters and
   the box/bg-rect drawing helpers).  Five names this text needs are not in the frozen mirrors - two
   declarations and three per-unit macro families (the `src/xterm.rs` 9860-9897 / `src/haikuterm.rs`
   27-245 copies).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3581: `int window_box_right (struct window *, enum glyph_row_area);`
  pub fn window_box_right (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3769-3770: `RGB_PIXEL_COLOR image_background (struct image *,
  /// struct frame *, Emacs_Pixmap);` - `RGB_PIXEL_COLOR` is dispextern.h's `unsigned long`.
  pub fn image_background (img: *mut image, f: *mut frame, ximg: Emacs_Pixmap) -> c_ulong;
  /// C: dispextern.h 3771-3772: `int image_background_transparent (struct image *,
  /// struct frame *, Emacs_Pixmap);`
  pub fn image_background_transparent (img: *mut image, f: *mut frame, mask: Emacs_Pixmap) -> c_int;
}

/* `struct face`'s bit-field run (dispextern.h 1893-1935, `include/dispextern.rs` 2259-2266 keeps it
   in one `bits` unit): C reads `face->box` and `face->use_box_color_for_shadows_p` here, and no
   mirror carries their accessors, so the unit names them - the `src/xterm.rs` 9871-9879 copy.  */
/// C: dispextern.h 1893-1913: `ENUM_BF (face_box_type) box : 2;` … `bool_bf
/// use_box_color_for_shadows_p : 1;` - the two positions this slice reads (the `src/xterm.rs`
/// 9844-9849 copies; the mirror keeps the run's `bits` unit, `include/dispextern.rs` 2259-2266).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_BOX_SHIFT: u32 = 0;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_BOX_MASK: u32 = 0x3;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_USE_BOX_COLOR_FOR_SHADOWS_P: u32 = 1 << 5;
/* The eight members slice 11's `android_draw_glyph_string` reads and this unit had not named
   yet, at the positions dispextern.h 1893-1935 gives them (the `src/xterm.rs` 9850-9869 row of
   the same record; `include/dispextern.rs`'s own copy is behind `dispextern_face_records`, so
   the mirror row is owed - P-26/P-80).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_UNDERLINE_SHIFT: u32 = 2;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_UNDERLINE_MASK: u32 = 0x7;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_OVERLINE_P: u32 = 1 << 6;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_STRIKE_THROUGH_P: u32 = 1 << 7;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_UNDERLINE_DEFAULTED_P: u32 = 1 << 10;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_OVERLINE_COLOR_DEFAULTED_P: u32 = 1 << 11;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P: u32 = 1 << 12;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_UNDERLINE_AT_DESCENT_LINE_P: u32 = 1 << 14;

#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! face_bits {
  (box, $f:expr) => { (((*$f).bits & FACE_BOX_MASK) >> FACE_BOX_SHIFT) as c_int };
  (underline, $f:expr) =>
    { (((*$f).bits & FACE_UNDERLINE_MASK) >> FACE_UNDERLINE_SHIFT) as c_int };
  (use_box_color_for_shadows_p, $f:expr) =>
    { ((*$f).bits & FACE_USE_BOX_COLOR_FOR_SHADOWS_P) != 0 };
  (overline_p, $f:expr) => { ((*$f).bits & FACE_OVERLINE_P) != 0 };
  (strike_through_p, $f:expr) => { ((*$f).bits & FACE_STRIKE_THROUGH_P) != 0 };
  (underline_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_UNDERLINE_DEFAULTED_P) != 0 };
  (overline_color_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_OVERLINE_COLOR_DEFAULTED_P) != 0 };
  (strike_through_color_defaulted_p, $f:expr) =>
    { ((*$f).bits & FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P) != 0 };
  (underline_at_descent_line_p, $f:expr) =>
    { ((*$f).bits & FACE_UNDERLINE_AT_DESCENT_LINE_P) != 0 };
  (overstrike, $f:expr) => { ((*$f).bits & FACE_OVERSTRIKE) != 0 };
}

/// C: dispextern.h 3860-3863: `#define IMAGE_BACKGROUND(img, f, ximg)` `((img)->background_valid`
/// `? (img)->background` `: image_background (img, f, ximg))`, and 3869-3872's
/// `IMAGE_BACKGROUND_TRANSPARENT` twin reading `(img)->background_transparent_valid`.  The mirror
/// keeps the four validity masks of the record's bit unit (`include/dispextern.rs` 2566-2574) and
/// not the two macros, so the unit carries them - the `src/haikuterm.rs` 227-244 shape, with the
/// two `image_background*` declarations above.
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! IMAGE_BACKGROUND {
  ($img:expr, $f:expr, $ximg:expr) =>
    {
      (if (*$img).bits & IMAGE_BACKGROUND_VALID_MASK != 0
	 { (*$img).background }
       else
	 { image_background ($img, $f, $ximg) })
    };
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! IMAGE_BACKGROUND_TRANSPARENT {
  ($img:expr, $f:expr, $mask:expr) =>
    {
      (if (*$img).bits & IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK != 0
	 { ((*$img).bits >> IMAGE_BACKGROUND_TRANSPARENT_SHIFT & 1) as c_int }
       else
	 { image_background_transparent ($img, $f, $mask) })
    };
}

/* `struct glyph`'s `u.cmp` run (dispextern.h 1425-1430, `include/dispextern.rs` 452-455 keeps
   `automatic : 1` and `id : 31` in one `bits`): C 3518/3525 read both - the `src/xterm.rs`
   9894-9897 copy.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! glyph_u_cmp_bits {
  (automatic, $g:expr) => { (((*$g).u.cmp.bits & GLYPH_U_CMP_AUTOMATIC) != 0) };
  (id, $g:expr) => { (((*$g).u.cmp.bits >> GLYPH_U_CMP_ID_SHIFT) as ptrdiff_t) };
}

/* C 3359-3426.  `static bool`; only called (C 3438/3455? no - the two `android_setup_relief_color`
   callers pass it by name; the address is not taken), so a plain Rust-ABI item.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_alloc_lighter_color (f: *mut frame, pixel: *mut c_ulong,
				       factor: f64, delta: c_int) -> bool
{
  let mut color: Emacs_Color = std::mem::zeroed ();                     // C 3363
  let mut new: Emacs_Color = std::mem::zeroed ();
  let mut bright: c_long;                                               // C 3364
  let mut success_p: bool;                                              // C 3365

  /* Get RGB color values.  */                                          // C 3367
  color.pixel = *pixel;                                                 // C 3368
  android_query_colors (f, &raw mut color, 1);                          // C 3369

  /* Change RGB values by specified FACTOR.  Avoid overflow!  */         // C 3371
  eassert! (factor >= 0.0);                                             // C 3372
  new.red = min (0xffff, (factor * color.red as f64) as c_int) as c_ushort;   // C 3373
  new.green = min (0xffff, (factor * color.green as f64) as c_int) as c_ushort;   // C 3374
  new.blue = min (0xffff, (factor * color.blue as f64) as c_int) as c_ushort;   // C 3375

  /* Calculate brightness of COLOR.  */                                 // C 3377
  bright = (2 * color.red as c_long + 3 * color.green as c_long      // C 3378
	    + color.blue as c_long) / 6;

  /* We only boost colors that are darker than
     HIGHLIGHT_COLOR_DARK_BOOST_LIMIT.  */                             // C 3380-3381
  if bright < HIGHLIGHT_COLOR_DARK_BOOST_LIMIT as c_long               // C 3382
    /* Make an additive adjustment to NEW, because it's dark enough so
       that scaling by FACTOR alone isn't enough.  */                  // C 3383-3384
    {
      /* How far below the limit this color is (0 - 1, 1 being darker).  */   // C 3386
      let dimness: f64 = 1.0 - bright as f64 / HIGHLIGHT_COLOR_DARK_BOOST_LIMIT as f64;   // C 3387
      /* The additive adjustment.  */                                   // C 3388
      let min_delta: c_int = (delta as f64 * dimness * factor / 2.0) as c_int;   // C 3389

      if factor < 1.0                                                 // C 3391
	{
	  new.red = max (0, new.red as c_int - min_delta) as c_ushort;   // C 3393
	  new.green = max (0, new.green as c_int - min_delta) as c_ushort;   // C 3394
	  new.blue = max (0, new.blue as c_int - min_delta) as c_ushort;   // C 3395
	}
      else
	{
	  new.red = min (0xffff, min_delta + new.red as c_int) as c_ushort;   // C 3399
	  new.green = min (0xffff, min_delta + new.green as c_int) as c_ushort;   // C 3400
	  new.blue = min (0xffff, min_delta + new.blue as c_int) as c_ushort;   // C 3401
	}
    }

  /* Try to allocate the color.  */                                     // C 3405
  success_p = android_alloc_nearest_color (f, &raw mut new);            // C 3406

  if success_p                                                         // C 3408
    {
      if new.pixel == *pixel                                           // C 3410
	{
	  /* If we end up with the same color as before, try adding
	     delta to the RGB values.  */                              // C 3412-3413
	  new.red = min (0xffff, delta + color.red as c_int) as c_ushort;   // C 3414
	  new.green = min (0xffff, delta + color.green as c_int) as c_ushort;   // C 3415
	  new.blue = min (0xffff, delta + color.blue as c_int) as c_ushort;   // C 3416
	  success_p = android_alloc_nearest_color (f, &raw mut new);      // C 3417
	}
      else
	{ success_p = true; }                                          // C 3420

      *pixel = new.pixel;                                              // C 3422
    }

  return success_p;                                                     // C 3425
}

/* Set up the foreground color for drawing relief lines of glyph
   string S.  RELIEF is a pointer to a struct relief containing the GC
   with which lines will be drawn.  Use a color that is FACTOR or
   DELTA lighter or darker than the relief's background which is found
   in S->f->output_data.android->relief_background.  If such a color
   cannot be allocated, use DEFAULT_PIXEL, instead.  */                 // C 3428-3433

/* C 3435-3462.  `static`; only called (C 3493/3495).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_setup_relief_color (f: *mut frame, relief: *mut relief,
				      factor: f64, delta: c_int,
				      default_pixel: c_ulong)
{
  let mut xgcv: android_gc_values = std::mem::zeroed ();                // C 3440
  let di: *mut android_output = (*f).output_data.android as *mut android_output;   // C 3441
  let mask: c_ulong = ANDROID_GC_FOREGROUND as c_ulong;                 // C 3442
  let mut pixel: c_ulong;                                               // C 3443
  let background: c_ulong = (*di).relief_background;                    // C 3444
  let dpyinfo: *mut android_display_info = FRAME_DISPLAY_INFO! (f);     // C 3445

  if ! (*relief).gc.is_null ()                                    // C 3447
      && (*relief).pixel != (-1 as c_long) as c_ulong               // C 3447
    { (*relief).pixel = (-1 as c_long) as c_ulong; }                                          // C 3448

  /* Allocate new color.  */                                            // C 3450
  xgcv.foreground = default_pixel;                                      // C 3451
  pixel = background;                                                   // C 3452

  if (*dpyinfo).n_planes != 1                                          // C 3454
      && android_alloc_lighter_color (f, &raw mut pixel, factor, delta)   // C 3455
    { xgcv.foreground = pixel; (*relief).pixel = pixel; }                // C 3456

  if (*relief).gc.is_null ()                                            // C 3458
    { (*relief).gc = android_create_gc (mask as android_gc_value_mask, &raw mut xgcv); }   // C 3459
  else
    { android_change_gc ((*relief).gc, mask as android_gc_value_mask, &raw mut xgcv); }   // C 3461
}

/* Set up colors for the relief lines around glyph string S.  */         // C 3464

/* C 3466-3498.  `static`; only called (C 3559).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_setup_relief_colors (s: *mut glyph_string)
{
  let mut di: *mut android_output;                                      // C 3469
  let mut color: c_ulong;                                               // C 3470

  di = (*(*s).f).output_data.android as *mut android_output;            // C 3472

  if face_bits! (use_box_color_for_shadows_p, (*s).face as *mut face)   // C 3474
    { color = (*( (*s).face as *mut face )).box_color; }                // C 3475
  else if glyph_bits! (type, (*(*s).first_glyph)) == IMAGE_GLYPH        // C 3476
	   && !(*( (*s).img )).pixmap.is_null ()                        // C 3477
	   && IMAGE_BACKGROUND_TRANSPARENT! ((*s).img, (*s).f, 0) == 0   // C 3478
    { color = IMAGE_BACKGROUND! ((*s).img, (*s).f, 0); }   // C 3479
  else
    {
      let mut xgcv: android_gc_values = std::mem::zeroed ();            // C 3482

      /* Get the background color of the face.  */                      // C 3484
      android_get_gc_values ((*s).gc as *mut android_gc, ANDROID_GC_BACKGROUND, &raw mut xgcv);   // C 3485
      color = xgcv.background;                                          // C 3486
    }

  if (*di).white_relief.gc.is_null ()                                   // C 3489
      || color != (*di).relief_background                               // C 3490
    {
      (*di).relief_background = color;                                  // C 3492
      android_setup_relief_color ((*s).f, &raw mut (*di).white_relief, 1.2, 0x8000,   // C 3493
				  WHITE_PIX_DEFAULT! ((*s).f));         // C 3494
      android_setup_relief_color ((*s).f, &raw mut (*di).black_relief, 0.6, 0x4000,   // C 3495
				  BLACK_PIX_DEFAULT! ((*s).f));         // C 3496
    }
}

/* C 3500-3564.  `static`; only called (C 4589).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_glyph_string_box (s: *mut glyph_string)
{
  let mut hwidth: c_int = 0;                                            // C 3503
  let mut vwidth: c_int = 0;
  let mut left_x: c_int = 0;
  let mut right_x: c_int = 0;
  let mut top_y: c_int = 0;
  let mut bottom_y: c_int = 0;
  let mut last_x: c_int = 0;
  let mut raised_p: bool = false;                                       // C 3504
  let mut left_p: bool = false;
  let mut right_p: bool = false;
  let mut last_glyph: *mut glyph;                                       // C 3505
  let mut clip_rect: android_rectangle = std::mem::zeroed ();           // C 3506

  last_x = if glyph_row_bits! (full_width_p, (*s).row)                  // C 3508
	      && ! window_bits! (pseudo_window_p, (*s).w)               // C 3508
	     { WINDOW_RIGHT_EDGE_X! ((*s).w) }                          // C 3509
	   else { window_box_right ((*s).w, (*s).area) };               // C 3510

  /* The glyph that may have a right box line.  For static
     compositions and images, the right-box flag is on the first glyph
     of the glyph string; for other types it's on the last glyph.  */   // C 3512-3514
  if ! (*s).cmp.is_null () || ! (*s).img.is_null ()                     // C 3515
    { last_glyph = (*s).first_glyph; }                                  // C 3516
  else if glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH    // C 3517
	   && glyph_u_cmp_bits! (automatic, (*s).first_glyph)        // C 3518
    {
      /* For automatic compositions, we need to look up the last glyph
	 in the composition.  */                                        // C 3520-3521
      let end: *mut glyph = (*((*s).row)).glyphs[(*s).area as usize]    // C 3522
	.offset ((*((*s).row)).used[(*s).area as usize] as isize);
      let mut g: *mut glyph = (*s).first_glyph;                         // C 3523
      last_glyph = g;                                                   // C 3524
      g = g.offset (1);                                                 // C 3524 (the `g++`)
      while g < end                                                     // C 3525
	    && glyph_u_cmp_bits! (automatic, g)                        // C 3525
	    && glyph_u_cmp_bits! (id, g) == (*s).cmp_id                // C 3525
	    && (*g).slice.cmp.to < (*s).cmp_to                        // C 3526
	{
	  last_glyph = g;                                               // C 3527 (the `last_glyph =
									//  g++`)
	  g = g.offset (1);                                             // C 3527
	}
    }
  else
    { last_glyph = (*s).first_glyph.offset ((*s).nchars as isize - 1); }   // C 3531

  vwidth = eabs! ((*( (*s).face as *mut face )).box_vertical_line_width);   // C 3533
  hwidth = eabs! ((*( (*s).face as *mut face )).box_horizontal_line_width);   // C 3534
  raised_p = face_bits! (box, (*s).face as *mut face) == FACE_RAISED_BOX;   // C 3535
  left_x = (*s).x;                                                      // C 3536
  right_x = if glyph_row_bits! (full_width_p, (*s).row)                 // C 3537
	       && glyph_string_bits! (extends_to_end_of_line_p, (*s))   // C 3537
	      { last_x - 1 }                                            // C 3538
	    else { min (last_x, (*s).x + (*s).background_width) - 1 };   // C 3539
  top_y = (*s).y;                                                       // C 3540
  bottom_y = top_y + (*s).height - 1;                                   // C 3541

  left_p = glyph_bits! (left_box_line_p, (*(*s).first_glyph))           // C 3543
	   || ((*s).hl == DRAW_MOUSE_FACE                               // C 3544
	       && ((*s).prev.is_null ()                                 // C 3545
		   || (*(*s).prev).hl != (*s).hl));                     // C 3546
  right_p = glyph_bits! (right_box_line_p, (*last_glyph))               // C 3547
	    || ((*s).hl == DRAW_MOUSE_FACE                              // C 3548
		&& ((*s).next.is_null ()                                // C 3549
		    || (*(*s).next).hl != (*s).hl));                    // C 3550

  get_glyph_string_clip_rect (s, &raw mut clip_rect as *mut c_void);                   // C 3552

  if face_bits! (box, (*s).face as *mut face) == FACE_SIMPLE_BOX   // C 3554
    { android_draw_box_rect (s, left_x, top_y, right_x, bottom_y, hwidth,   // C 3555
			     vwidth, left_p, right_p, &raw mut clip_rect); }   // C 3556
  else
    {
      android_setup_relief_colors (s);                                  // C 3559
      android_draw_relief_rect ((*s).f, left_x, top_y, right_x, bottom_y, hwidth,   // C 3560
				vwidth, raised_p, true, true, left_p, right_p,   // C 3561
				&raw mut clip_rect);                        // C 3562
    }
}

/* C 3566-3580.  `static`; only called (C 3796/4327/4350).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_glyph_string_bg_rect (s: *mut glyph_string, x: c_int, y: c_int,
					     w: c_int, h: c_int)
{
  if glyph_string_bits! (stippled_p, (*s))                              // C 3570
    {
      /* Fill background with a stipple pattern.  */                    // C 3572
      android_set_fill_style ((*s).gc as *mut android_gc, ANDROID_FILL_OPAQUE_STIPPLED);   // C 3573
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc, x,   // C 3574
			      y, w as c_uint, h as c_uint);             // C 3575
      android_set_fill_style ((*s).gc as *mut android_gc, ANDROID_FILL_SOLID);   // C 3576
    }
  else
    { android_clear_glyph_string_rect (s, x, y, w, h); }                // C 3579
}

/* C 3582-3797: the three image-drawing helpers.  `image_ascent` is the one name this text needs
   that no mirror reaches, so it stands at its use site below (P-72 rule 4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3774: `int image_ascent (struct image *, struct face *, struct glyph_slice *);`
  pub fn image_ascent (img: *mut image, face: *mut face, slice: *mut glyph_slice) -> c_int;
}

/* C 3582-3670.  `static`; only called (C 3796).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_image_relief (s: *mut glyph_string)
{
  let mut x1: c_int;                                                    // C 3585
  let mut y1: c_int;
  let mut thick: c_int;
  let mut raised_p: bool;                                               // C 3586
  let mut top_p: bool;
  let mut bot_p: bool;
  let mut left_p: bool;
  let mut right_p: bool;
  let mut extra_x: c_int;                                               // C 3587
  let mut extra_y: c_int;
  let mut r: android_rectangle = std::mem::zeroed ();                   // C 3588
  let mut x: c_int = (*s).x;                                            // C 3589
  let mut y: c_int = (*s).ybase - image_ascent ((*s).img, (*s).face as *mut face,   // C 3590
						&raw mut (*s).slice);

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */                                           // C 3592-3593
  if face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX            // C 3594
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))             // C 3595
      && glyph_slice_bits! (x, (*s).slice) as c_int == 0                                             // C 3596
    { x += max ((*( (*s).face as *mut face )).box_vertical_line_width, 0); }   // C 3597

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */                                               // C 3599-3600
  if glyph_slice_bits! (x, (*s).slice) as c_int == 0                                                 // C 3601
    { x += (*( (*s).img )).hmargin; }                                   // C 3602
  if glyph_slice_bits! (y, (*s).slice) as c_int == 0                                                 // C 3603
    { y += (*( (*s).img )).vmargin; }                                   // C 3604

  if (*s).hl == DRAW_IMAGE_SUNKEN                                       // C 3606
      || (*s).hl == DRAW_IMAGE_RAISED                                  // C 3607
    {
      if (*( (*s).face as *mut face )).id == TAB_BAR_FACE_ID           // C 3609
	{ thick = if tab_bar_button_relief < 0                         // C 3610
		    { DEFAULT_TAB_BAR_BUTTON_RELIEF } else { min (tab_bar_button_relief as c_int, 1000000) }; }   // C 3611-3612
      else
	{ thick = if tool_bar_button_relief < 0                        // C 3614
		    { DEFAULT_TOOL_BAR_BUTTON_RELIEF } else { min (tool_bar_button_relief as c_int, 1000000) }; }   // C 3615-3616
      raised_p = (*s).hl == DRAW_IMAGE_RAISED;                          // C 3617
    }
  else
    {
      thick = eabs! ((*( (*s).img )).relief);                           // C 3621
      raised_p = (*( (*s).img )).relief > 0;                            // C 3622
    }

  x1 = x + glyph_slice_bits! (width, (*s).slice) as c_int - 1;                                        // C 3625
  y1 = y + glyph_slice_bits! (height, (*s).slice) as c_int - 1;                                       // C 3626

  extra_x = 0;                                                          // C 3628
  extra_y = 0;
  if (*( (*s).face as *mut face )).id == TAB_BAR_FACE_ID                // C 3629
    {
      if CONSP (Vtab_bar_button_margin)                                 // C 3631
	  && FIXNUMP (XCAR (Vtab_bar_button_margin))                    // C 3632
	  && FIXNUMP (XCDR (Vtab_bar_button_margin))                    // C 3633
	{
	  extra_x = XFIXNUM (XCAR (Vtab_bar_button_margin)) as c_int - thick;   // C 3635
	  extra_y = XFIXNUM (XCDR (Vtab_bar_button_margin)) as c_int - thick;   // C 3636
	}
      else if FIXNUMP (Vtab_bar_button_margin)                          // C 3638
	{ extra_x = XFIXNUM (Vtab_bar_button_margin) as c_int - thick;   // C 3639
	  extra_y = extra_x; }
    }

  if (*( (*s).face as *mut face )).id == TOOL_BAR_FACE_ID               // C 3642
    {
      if CONSP (Vtool_bar_button_margin)                                // C 3644
	  && FIXNUMP (XCAR (Vtool_bar_button_margin))                   // C 3645
	  && FIXNUMP (XCDR (Vtool_bar_button_margin))                   // C 3646
	{
	  extra_x = XFIXNUM (XCAR (Vtool_bar_button_margin)) as c_int;   // C 3648
	  extra_y = XFIXNUM (XCDR (Vtool_bar_button_margin)) as c_int;   // C 3649
	}
      else if FIXNUMP (Vtool_bar_button_margin)                         // C 3651
	{ extra_x = XFIXNUM (Vtool_bar_button_margin) as c_int;          // C 3652
	  extra_y = extra_x; }
    }

  top_p = false;                                                        // C 3655
  bot_p = false;
  left_p = false;
  right_p = false;

  if glyph_slice_bits! (x, (*s).slice) as c_int == 0                                                 // C 3657
    { x -= thick + extra_x; left_p = true; }                           // C 3658
  if glyph_slice_bits! (y, (*s).slice) as c_int == 0                                                 // C 3659
    { y -= thick + extra_y; top_p = true; }                            // C 3660
  if glyph_slice_bits! (x, (*s).slice) as c_int + glyph_slice_bits! (width, (*s).slice) as c_int == (*( (*s).img )).width          // C 3661
    { x1 += thick + extra_x; right_p = true; }                         // C 3662
  if glyph_slice_bits! (y, (*s).slice) as c_int + glyph_slice_bits! (height, (*s).slice) as c_int == (*( (*s).img )).height        // C 3663
    { y1 += thick + extra_y; bot_p = true; }                           // C 3664

  android_setup_relief_colors (s);                                      // C 3666
  get_glyph_string_clip_rect (s, &raw mut r as *mut c_void);            // C 3667
  android_draw_relief_rect ((*s).f, x, y, x1, y1, thick, thick, raised_p,   // C 3668
			    top_p, bot_p, left_p, right_p, &raw mut r);   // C 3669
}

/* C 3672-3740.  `static`; only called (C 3789).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_image_foreground (s: *mut glyph_string)
{
  let mut x: c_int = (*s).x;                                            // C 3675
  let mut y: c_int = (*s).ybase - image_ascent ((*s).img, (*s).face as *mut face,   // C 3676
						&raw mut (*s).slice);

  /* If first glyph of S has a left box line, start drawing it to the
     right of that line.  */                                           // C 3678-3679
  if face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX            // C 3680
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))             // C 3681
      && glyph_slice_bits! (x, (*s).slice) as c_int == 0                                             // C 3682
    { x += max ((*( (*s).face as *mut face )).box_vertical_line_width, 0); }   // C 3683

  /* If there is a margin around the image, adjust x- and y-position
     by that margin.  */                                               // C 3685-3686
  if glyph_slice_bits! (x, (*s).slice) as c_int == 0                                                 // C 3687
    { x += (*( (*s).img )).hmargin; }                                   // C 3688
  if glyph_slice_bits! (y, (*s).slice) as c_int == 0                                                 // C 3689
    { y += (*( (*s).img )).vmargin; }                                   // C 3690

  if ! (*( (*s).img )).pixmap.is_null ()                                // C 3692
    {
      let mask: c_ulong = (ANDROID_GC_CLIP_MASK                        // C 3694
			   | ANDROID_GC_CLIP_X_ORIGIN                  // C 3695
			   | ANDROID_GC_CLIP_Y_ORIGIN                  // C 3696
			   | ANDROID_GC_FUNCTION) as c_ulong;          // C 3697
      let mut xgcv: android_gc_values = std::mem::zeroed ();            // C 3698
      let mut clip_rect: android_rectangle = std::mem::zeroed ();       // C 3699
      let mut image_rect: android_rectangle = std::mem::zeroed ();
      let mut r: android_rectangle = std::mem::zeroed ();

      xgcv.clip_mask = (*( (*s).img )).mask as android_pixmap;          // C 3701
      xgcv.clip_x_origin = x - glyph_slice_bits! (x, (*s).slice) as c_int;                            // C 3702
      xgcv.clip_y_origin = y - glyph_slice_bits! (y, (*s).slice) as c_int;                            // C 3703
      xgcv.function = ANDROID_GC_COPY;                                  // C 3704
      android_change_gc ((*s).gc as *mut android_gc, mask as android_gc_value_mask, &raw mut xgcv);   // C 3705

      get_glyph_string_clip_rect (s, &raw mut clip_rect as *mut c_void);   // C 3707
      image_rect.x = x;                                                 // C 3708
      image_rect.y = y;                                                 // C 3709
      image_rect.width = glyph_slice_bits! (width, (*s).slice) as c_int as c_uint;                    // C 3710
      image_rect.height = glyph_slice_bits! (height, (*s).slice) as c_int as c_uint;                  // C 3711

      if gui_intersect_rectangles (&raw const clip_rect as *const Emacs_Rectangle,   // C 3713
				   &raw const image_rect as *const Emacs_Rectangle,
				   &raw mut r as *mut Emacs_Rectangle)
	{ android_copy_area ((*( (*s).img )).pixmap as android_drawable,   // C 3714
			     FRAME_ANDROID_DRAWABLE! ((*s).f),          // C 3715
			     (*s).gc as *mut android_gc, glyph_slice_bits! (x, (*s).slice) as c_int + r.x - x,   // C 3716
			     glyph_slice_bits! (y, (*s).slice) as c_int + r.y - y,                    // C 3717
			     r.width, r.height, r.x, r.y); }            // C 3718

      /* When the image has a mask, we can expect that at least part
	 of a mouse highlight or a block cursor will be visible.  If
	 the image doesn't have a mask, make a block cursor visible by
	 drawing a rectangle around the image.  I believe it's looking
	 better if we do nothing here for mouse-face.  */               // C 3720-3724
      if (*s).hl == DRAW_CURSOR && (*( (*s).img )).mask.is_null ()      // C 3725
	{
	  let relief: c_int = eabs! ((*( (*s).img )).relief);            // C 3727
	  android_draw_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc,   // C 3728
				  x - relief, y - relief,               // C 3729
				  (glyph_slice_bits! (width, (*s).slice) as c_int + relief * 2 - 1) as c_uint,   // C 3730
				  (glyph_slice_bits! (height, (*s).slice) as c_int + relief * 2 - 1) as c_uint);   // C 3731
	}

      android_set_clip_mask ((*s).gc as *mut android_gc, ANDROID_NONE as android_pixmap);   // C 3734
    }
  else
    /* Draw a rectangle if image could not be loaded.  */                // C 3737
    { android_draw_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc, x, y,   // C 3738
			      (glyph_slice_bits! (width, (*s).slice) as c_int - 1) as c_uint, (glyph_slice_bits! (height, (*s).slice) as c_int - 1) as c_uint); }   // C 3739
}

/* C 3742-3797.  `static`; only called (C 4582).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_image_glyph_string (s: *mut glyph_string)
{
  let box_line_hwidth: c_int = max ((*( (*s).face as *mut face )).box_vertical_line_width, 0);   // C 3745
  let box_line_vwidth: c_int = max ((*( (*s).face as *mut face )).box_horizontal_line_width, 0);   // C 3746
  let mut height: c_int;                                                // C 3747

  height = (*s).height;                                                 // C 3749
  if glyph_slice_bits! (y, (*s).slice) as c_int == 0                                                  // C 3750
    { height -= box_line_vwidth; }                                      // C 3751
  if glyph_slice_bits! (y, (*s).slice) as c_int + glyph_slice_bits! (height, (*s).slice) as c_int >= (*( (*s).img )).height         // C 3752
    { height -= box_line_vwidth; }                                      // C 3753

  /* Fill background with face under the image.  Do it only if row is
     taller than image or if image has a clip mask to reduce
     flickering.  */                                                    // C 3755-3757
  glyph_string_bits! (stippled_p, (*s), (*( (*s).face as *mut face )).stipple != 0);   // C 3758
  if height > glyph_slice_bits! (height, (*s).slice) as c_int                                        // C 3759
      || (*( (*s).img )).hmargin != 0                                   // C 3760
      || (*( (*s).img )).vmargin != 0                                   // C 3761
      || ! (*( (*s).img )).mask.is_null ()                              // C 3762
      || (*( (*s).img )).pixmap.is_null ()                              // C 3763
      || (*s).width != (*s).background_width                            // C 3764
    {
      if glyph_string_bits! (stippled_p, (*s))                          // C 3766
	{ glyph_row_bits! (stipple_p, (*s).row, true); }                 // C 3767

      let mut x: c_int = (*s).x;                                        // C 3769
      let mut y: c_int = (*s).y;                                        // C 3770
      let mut width: c_int = (*s).background_width;                     // C 3771

      if glyph_bits! (left_box_line_p, (*(*s).first_glyph))             // C 3773
	  && glyph_slice_bits! (x, (*s).slice) as c_int == 0                                         // C 3774
	{
	  x += box_line_hwidth;                                         // C 3776
	  width -= box_line_hwidth;                                     // C 3777
	}

      if glyph_slice_bits! (y, (*s).slice) as c_int == 0                                             // C 3780
	{ y += box_line_vwidth; }                                      // C 3781

      android_draw_glyph_string_bg_rect (s, x, y, width, height);        // C 3783

      glyph_string_bits! (background_filled_p, (*s), true);              // C 3785
    }

  /* Draw the foreground.  */                                           // C 3788
  android_draw_image_foreground (s);                                    // C 3789
  android_set_glyph_string_clipping (s);                                // C 3790

  /* If we must draw a relief around the image, do it.  */                // C 3792
  if (*( (*s).img )).relief != 0                                        // C 3793
      || (*s).hl == DRAW_IMAGE_RAISED                                   // C 3794
      || (*s).hl == DRAW_IMAGE_SUNKEN                                   // C 3795
    { android_draw_image_relief (s); }                                  // C 3796
}

/* C 3799-4123: the stretch, scale, underwave and the two glyph-string foregrounds.  Three names
   this text needs are not in the frozen mirrors, so they stand at their use sites below (P-72
   rule 4), and `face->overstrike` needs one more arm of this unit's `face_bits!` (with C's own bit
   position).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3579: `int window_box_left (struct window *, enum glyph_row_area);`
  pub fn window_box_left (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3580: `int window_box_left_offset (struct window *, enum glyph_row_area);`
  pub fn window_box_left_offset (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3692: `extern bool cursor_in_mouse_face_p (struct window *w);` (xdisp.c)
  pub fn cursor_in_mouse_face_p (w: *mut window) -> bool;
}

/// C: dispextern.h 1893-1935: `bool_bf overstrike : 1;` - the bit `android_draw_glyph_string_foreground`
/// reads (the `src/xterm.rs` 9850 value; the mirror keeps the run's `bits` unit).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const FACE_OVERSTRIKE: u32 = 1 << 21;

/* C 3799-3911.  `static`; only called (C 4586).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_stretch_glyph_string (s: *mut glyph_string)
{
  eassert! (glyph_bits! (type, (*(*s).first_glyph)) == STRETCH_GLYPH);   // C 3802

  if (*s).hl == DRAW_CURSOR && ! x_stretch_cursor_p                     // C 3804
    {
      /* If `x-stretch-cursor' is nil, don't draw a block cursor as
	 wide as the stretch glyph.  */                                // C 3806-3807
      let mut width: c_int;                                             // C 3808
      let mut background_width: c_int = (*s).background_width;
      let mut x: c_int = (*s).x;                                        // C 3809

      if ! glyph_row_bits! (reversed_p, (*s).row)                       // C 3811
	{
	  let left_x: c_int = window_box_left_offset ((*s).w, TEXT_AREA);   // C 3813

	  if x < left_x                                                    // C 3815
	    {
	      background_width -= left_x - x;                             // C 3817
	      x = left_x;                                                 // C 3818
	    }
	}
      else
	{
	  /* In R2L rows, draw the cursor on the right edge of the
	     stretch glyph.  */                                         // C 3823-3824
	  let right_x: c_int = window_box_right ((*s).w, TEXT_AREA);       // C 3825

	  if x + background_width > right_x                                // C 3827
	    { background_width -= x - right_x; }                          // C 3828
	  x += background_width;                                          // C 3829
	}
      width = min (FRAME_COLUMN_WIDTH ((*s).f), background_width);      // C 3831
      if glyph_row_bits! (reversed_p, (*s).row)                         // C 3832
	{ x -= width; }                                                  // C 3833

      /* Draw cursor.  */                                               // C 3835
      android_draw_glyph_string_bg_rect (s, x, (*s).y, width, (*s).height);   // C 3836

      /* Clear rest using the GC of the original non-cursor face.  */    // C 3838
      if width < background_width                                       // C 3839
	{
	  let y: c_int = (*s).y;                                        // C 3841
	  let w: c_int = background_width - width;                      // C 3842
	  let h: c_int = (*s).height;
	  let mut r: android_rectangle = std::mem::zeroed ();            // C 3843
	  let mut gc: *mut android_gc;                                   // C 3844

	  if ! glyph_row_bits! (reversed_p, (*s).row)                    // C 3846
	    { x += width; }                                             // C 3847
	  else
	    { x = (*s).x; }                                             // C 3849
	  if glyph_row_bits! (mouse_face_p, (*s).row)                    // C 3850
	      && cursor_in_mouse_face_p ((*s).w)                        // C 3851
	    {
	      android_set_mouse_face_gc (s);                            // C 3853
	      gc = (*s).gc as *mut android_gc;                          // C 3854
	    }
	  else
	    { gc = (*( (*s).face as *mut face )).gc as *mut android_gc; }   // C 3857

	  get_glyph_string_clip_rect (s, &raw mut r as *mut c_void);     // C 3859
	  android_set_clip_rectangles (gc, 0, 0, &raw mut r, 1);          // C 3860

	  if (*( (*s).face as *mut face )).stipple != 0                   // C 3862
	    {
	      /* Fill background with a stipple pattern.  */             // C 3864
	      android_set_fill_style (gc, ANDROID_FILL_OPAQUE_STIPPLED);   // C 3865
	      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f),   // C 3866
				      gc, x, y, w as c_uint, h as c_uint);   // C 3867
	      android_set_fill_style (gc, ANDROID_FILL_SOLID);           // C 3868

	      glyph_row_bits! (stipple_p, (*s).row, true);               // C 3870
	    }
	  else
	    {
	      let mut xgcv: android_gc_values = std::mem::zeroed ();      // C 3874
	      android_get_gc_values (gc, (ANDROID_GC_FOREGROUND           // C 3875
					  | ANDROID_GC_BACKGROUND),       // C 3876
				     &raw mut xgcv);                      // C 3877
	      android_set_foreground (gc, xgcv.background);              // C 3878
	      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f),   // C 3879
				      gc, x, y, w as c_uint, h as c_uint);   // C 3880
	      android_set_foreground (gc, xgcv.foreground);              // C 3881
	    }

	  android_reset_clip_rectangles ((*s).f, gc);                     // C 3884
	}
    }
  else if ! glyph_string_bits! (background_filled_p, (*s))               // C 3887
    {
      let mut background_width: c_int = (*s).background_width;         // C 3889
      let mut x: c_int = (*s).x;                                       // C 3890
      let text_left_x: c_int = window_box_left ((*s).w, TEXT_AREA);

      /* Don't draw into left fringe or scrollbar area except for
         header line and mode line.  */                                // C 3892-3893
      if (*s).area == TEXT_AREA                                        // C 3894
	  && x < text_left_x && ! glyph_row_bits! (mode_line_p, (*s).row)   // C 3895
	{
	  background_width -= text_left_x - x;                          // C 3897
	  x = text_left_x;                                              // C 3898
	}

      if ! glyph_row_bits! (stipple_p, (*s).row)                       // C 3901
	{ glyph_row_bits! (stipple_p, (*s).row, glyph_string_bits! (stippled_p, (*s))); }   // C 3902

      if background_width > 0                                          // C 3904
	{ android_draw_glyph_string_bg_rect (s, x, (*s).y,             // C 3905
					     background_width,         // C 3906
					     (*s).height); }           // C 3907
    }

  glyph_string_bits! (background_filled_p, (*s), true);                  // C 3910
}

/* C 3913-3926.  `static`; only called (C 3933).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_get_scale_factor (scale_x: *mut c_int, scale_y: *mut c_int)
{
  /* This is 96 everywhere else, but 160 on Android.  */                 // C 3916
  let base_res: c_int = 160;                                            // C 3917

  *scale_x = 1;                                                         // C 3919
  *scale_y = 1;
  eassert! (! x_display_list.is_null ());                                // C 3920

  if (*x_display_list).resx > base_res as f64                           // C 3922
    { *scale_x = floor ((*x_display_list).resx / base_res as f64) as c_int; }   // C 3923
  if (*x_display_list).resy > base_res as f64                           // C 3924
    { *scale_y = floor ((*x_display_list).resy / base_res as f64) as c_int; }   // C 3925
}

/* C 3928-3987.  `static`; only called (C 4521).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_underwave (s: *mut glyph_string, decoration_width: c_int)
{
  /* C leaves these two uninitialised until `android_get_scale_factor` writes them; the port
     gives them a value because `&raw mut` needs an initialised place (the `haikuterm.rs` 1287-1288
     and `w32term.rs` 1007-1008 shape).  */
  let mut scale_x: c_int = 0;                                           // C 3931
  let mut scale_y: c_int = 0;

  android_get_scale_factor (&raw mut scale_x, &raw mut scale_y);         // C 3933

  let wave_height: c_int = 3 * scale_y;                                 // C 3935
  let wave_length: c_int = 2 * scale_x;

  let mut dx: c_int;                                                    // C 3937
  let mut dy: c_int;
  let mut x0: c_int;
  let mut y0: c_int;
  let mut width: c_int;
  let mut x1: c_int;
  let mut y1: c_int;
  let mut x2: c_int;
  let mut y2: c_int;
  let mut xmax: c_int;
  let mut odd: bool;                                                    // C 3938
  let mut wave_clip: android_rectangle = std::mem::zeroed ();           // C 3939
  let mut string_clip: android_rectangle = std::mem::zeroed ();
  let mut final_clip: android_rectangle = std::mem::zeroed ();

  dx = wave_length;                                                     // C 3941
  dy = wave_height - 1;                                                 // C 3942
  x0 = (*s).x;                                                          // C 3943
  y0 = (*s).ybase + wave_height / 2;                                    // C 3944
  width = decoration_width;                                             // C 3945
  xmax = x0 + width;                                                    // C 3946

  /* Find and set clipping rectangle */                                  // C 3948

  wave_clip.x = x0;                                                     // C 3950
  wave_clip.y = y0;                                                     // C 3951
  wave_clip.width = width as c_uint;                                    // C 3952
  wave_clip.height = wave_height as c_uint;                             // C 3953
  get_glyph_string_clip_rect (s, &raw mut string_clip as *mut c_void);   // C 3954

  if ! gui_intersect_rectangles (&raw const wave_clip as *const Emacs_Rectangle,   // C 3956
				 &raw const string_clip as *const Emacs_Rectangle,
				 &raw mut final_clip as *mut Emacs_Rectangle)
    { return; }                                                         // C 3957

  android_set_clip_rectangles ((*s).gc as *mut android_gc, 0, 0, &raw mut final_clip, 1);   // C 3959

  /* Draw the waves */                                                  // C 3961

  x1 = x0 - (x0 % dx);                                                  // C 3963
  x2 = x1 + dx;                                                         // C 3964
  odd = ((x1 / dx) & 1) != 0;                                           // C 3965
  y1 = y0;                                                              // C 3966
  y2 = y0;

  if odd                                                                // C 3968
    { y1 += dy; }                                                       // C 3969
  else
    { y2 += dy; }                                                       // C 3971

  if INT_MAX - dx < xmax                                                // C 3973
    { emacs_abort (); }                                                 // C 3974

  while x1 <= xmax                                                      // C 3976
    {
      android_draw_line (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc,   // C 3978
			 x1, y1, x2, y2);                               // C 3979
      x1 = x2; y1 = y2;                                                 // C 3980
      x2 += dx; y2 = y0 + odd as c_int * dy;                            // C 3981
      odd = ! odd;                                                      // C 3982
    }

  /* Restore previous clipping rectangle(s) */                          // C 3985
  android_set_clip_rectangles ((*s).gc as *mut android_gc, 0, 0,        // C 3986
			       &raw mut (*s).clip as *mut [c_int; 2] as *mut android_rectangle,   // C 3986
			       (*s).num_clips);
}

/* C 3989-4034.  `static`; only called (C 4516).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_glyph_string_foreground (s: *mut glyph_string)
{
  let mut i: c_int;                                                     // C 3992
  let mut x: c_int;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */                           // C 3994-3995
  if face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX            // C 3996
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))             // C 3997
    { x = (*s).x + max ((*( (*s).face as *mut face )).box_vertical_line_width, 0); }   // C 3998
  else
    { x = (*s).x; }                                                     // C 4000

  /* Draw characters of S as rectangles if S's font could not be
     loaded.  */                                                       // C 4002-4003
  if glyph_string_bits! (font_not_found_p, (*s))                        // C 4004
    {
      i = 0;                                                            // C 4006
      while i < (*s).nchars
	{
	  let g: *mut glyph = (*s).first_glyph.offset (i as isize);      // C 4008
	  android_draw_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f),     // C 4009
				  (*s).gc as *mut android_gc, x, (*s).y,   // C 4010
				  ((*g).pixel_width - 1) as c_uint,     // C 4011
				  ((*s).height - 1) as c_uint);         // C 4012
	  x += (*g).pixel_width as c_int;                               // C 4013
	  i += 1;                                                       // C 4006 (the `for` step)
	}
    }
  else
    {
      let font: *mut font = (*s).font;                                  // C 4018
      let mut boff: c_int = (*font).baseline_offset;                    // C 4019
      let y: c_int;                                                     // C 4020

      if (*font).vertical_centering                                      // C 4022
	{ boff = VCENTER_BASELINE_OFFSET! (font, (*s).f) - boff; }      // C 4023

      y = (*s).ybase - boff;                                            // C 4025
      if glyph_string_bits! (for_overlaps, (*s)) != 0                   // C 4026
	  || (glyph_string_bits! (background_filled_p, (*s)) && (*s).hl != DRAW_CURSOR)   // C 4027
	{ ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, 0, (*s).nchars as c_int, x, y, false); }   // C 4028
      else
	{ ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, 0, (*s).nchars as c_int, x, y, true); }   // C 4030
      if face_bits! (overstrike, (*s).face as *mut face)                // C 4031
	{ ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, 0, (*s).nchars as c_int, x + 1, y, false); }   // C 4032
    }
}

/* C 4036-4123.  `static`; only called (C 4519).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_composite_glyph_string_foreground (s: *mut glyph_string)
{
  let mut i: c_int;                                                     // C 4039
  let mut j: c_int;
  let mut x: c_int;
  let font: *mut font = (*s).font;                                      // C 4040

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */                           // C 4042-4043
  if ! (*s).face.is_null ()                                             // C 4044
      && face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX        // C 4044
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))             // C 4045
    { x = (*s).x + max ((*( (*s).face as *mut face )).box_vertical_line_width, 0); }   // C 4046
  else
    { x = (*s).x; }                                                     // C 4048

  /* S is a glyph string for a composition.  S->cmp_from is the index
     of the first character drawn for glyphs of this composition.
     S->cmp_from == 0 means we are drawing the very first character of
     this composition.  */                                              // C 4050-4053

  /* Draw a rectangle for the composition if the font for the very
     first character of the composition could not be loaded.  */       // C 4055-4056
  if glyph_string_bits! (font_not_found_p, (*s))                        // C 4057
    {
      if (*s).cmp_from == 0                                             // C 4059
	{ android_draw_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f),     // C 4060
				  (*s).gc as *mut android_gc, x, (*s).y,   // C 4061
				  ((*s).width - 1) as c_uint, ((*s).height - 1) as c_uint); }   // C 4062
    }
  else if ! glyph_u_cmp_bits! (automatic, (*s).first_glyph)             // C 4064
    {
      let y: c_int = (*s).ybase;                                        // C 4066

      i = 0;                                                            // C 4068
      j = (*s).cmp_from;
      while i < (*s).nchars
	{
	  /* TAB in a composition means display glyphs with
	     padding space on the left or right.  */                    // C 4069-4070
	  if COMPOSITION_GLYPH ((*s).cmp as *mut composition, j as ptrdiff_t) != b'\t' as isize   // C 4071
	    {
	      let xx: c_int = x + (*( (*s).cmp as *mut composition )).offsets.offset ((j * 2) as isize) as c_int;   // C 4073
	      let yy: c_int = y - (*( (*s).cmp as *mut composition )).offsets.offset ((j * 2 + 1) as isize) as c_int;   // C 4074

	      ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, j, j + 1, xx, yy, false);   // C 4076
	      if face_bits! (overstrike, (*s).face as *mut face)        // C 4077
		{ ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, j, j + 1, xx + 1, yy, false); }   // C 4078
	    }
	  i += 1;                                                       // C 4068 (the two-part `for` step)
	  j += 1;
	}
    }
  else
    {
      let gstring: Lisp_Object = composition_gstring_from_id ((*s).cmp_id);   // C 4083
      let mut glyph: Lisp_Object;                                       // C 4084
      let y: c_int = (*s).ybase;                                        // C 4085
      let mut width: c_int = 0;                                         // C 4086

      i = (*s).cmp_from;                                                // C 4088
      j = (*s).cmp_from;
      while i < (*s).cmp_to
	{
	  glyph = LGSTRING_GLYPH (gstring, i as ptrdiff_t);              // C 4090
	  if NILP (LGLYPH_ADJUSTMENT (glyph))                            // C 4091
	    { width += LGLYPH_WIDTH (glyph) as c_int; }                  // C 4092
	  else
	    {
	      let mut xoff: c_int;                                       // C 4095
	      let mut yoff: c_int;
	      let mut wadjust: c_int;

	      if j < i                                                   // C 4097
		{
		  ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, j, i, x, y, false);   // C 4099
		  if face_bits! (overstrike, (*s).face as *mut face)     // C 4100
		    { ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, j, i, x + 1, y, false); }   // C 4101
		  x += width;                                            // C 4102
		}
	      xoff = LGLYPH_XOFF (glyph) as c_int;                        // C 4104
	      yoff = LGLYPH_YOFF (glyph) as c_int;                        // C 4105
	      wadjust = LGLYPH_WADJUST (glyph) as c_int;                  // C 4106
	      ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, i, i + 1, x + xoff, y + yoff, false);   // C 4107
	      if face_bits! (overstrike, (*s).face as *mut face)          // C 4108
		{ ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, i, i + 1, x + xoff + 1, y + yoff, false); }   // C 4109-4110
	      x += wadjust;                                              // C 4111
	      j = i + 1;                                                 // C 4112
	      width = 0;                                                 // C 4113
	    }
	  i += 1;                                                         // C 4088 (the `for` step)
	}
      if j < i                                                           // C 4116
	{
	  ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, j, i, x, y, false);   // C 4118
	  if face_bits! (overstrike, (*s).face as *mut face)              // C 4119
	    { ((*((*font).driver as *const font_driver)).draw.unwrap ()) (s, j, i, x + 1, y, false); }   // C 4120
	}
    }
}

/* C 4125-4206: `android_draw_glyphless_glyph_string_foreground`.  C's `unsigned char2b[8]` is
   an `unsigned int` array (the name C gives the implicit-`int` declaration, the `src/xterm.rs`
   10220-10223 reading) and `s->char2b = char2b` its decay (`include/dispextern.rs` 1654's
   `*mut c_uint`); C 4144-4148's `#ifdef GCC_LINT` pair is carried under that feature (`GCC_LINT`
   is 1 in the frozen table, so the `= 1` arm is the live one); C 4151-4174's five
   `glyph->u.glyphless.*` reads are the `glyphless_bits!` accessors named after the member - the
   mirror keeps the run's `bits` unit (`include/dispextern.rs` 474-481) and no reader, so this
   unit names its own (Owed 22's family).  */
/// C: dispextern.h 626-632: `unsigned method : 2; bool_bf for_no_font : 1; unsigned len : 4;
/// unsigned ch : 25;` - the four positions this function reads (the `src/xterm.rs` 9899-9910 copy).
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! glyphless_bits {
  (method, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_METHOD_SHIFT) & 0x3) as c_int };
  (for_no_font, $g:expr) => { ((*$g).u.glyphless.bits & GLYPH_U_GLYPHLESS_FOR_NO_FONT) != 0 };
  (len, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_LEN_SHIFT) & 0xf) as c_int };
  (ch, $g:expr) =>
    { (((*$g).u.glyphless.bits >> GLYPH_U_GLYPHLESS_CH_SHIFT) & 0x1ffffff) as c_int };
}

/* C 4125-4206.  `static`; only called (C 4516).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_glyphless_glyph_string_foreground (s: *mut glyph_string)
{
  let mut glyph: *mut glyph = (*s).first_glyph;                          // C 4128
  let mut char2b: [c_uint; 8] = [0; 8];                                  // C 4129
  let mut x: c_int;                                                      // C 4130
  let mut i: c_int;
  let mut j: c_int;

  /* If first glyph of S has a left box line, start drawing the text
     of S to the right of that box line.  */                            // C 4132-4133
  if ! (*s).face.is_null ()                                              // C 4134
      && face_bits! (box, (*s).face as *mut face) != FACE_NO_BOX         // C 4134
      && glyph_bits! (left_box_line_p, (*(*s).first_glyph))              // C 4135
    { x = (*s).x + max ((*( (*s).face as *mut face )).box_vertical_line_width, 0); }   // C 4136
  else
    { x = (*s).x; }                                                      // C 4138

  (*s).char2b = char2b.as_mut_ptr ();                                    // C 4140

  i = 0;                                                                 // C 4142
  while i < (*s).nchars
    {
      /* C 4144-4148's `#ifdef GCC_LINT` pair: the frozen table has `GCC_LINT` 1
	 (docs/baseline/config 1162; conf_post.h 368 tests it), so the first arm is the text
	 this target compiles and the `#else` arm is carried under `not gcc_lint`.  */
      #[cfg (feature = "gcc_lint")]
      const PACIFY_GCC_BUG_81401: c_int = 1;                             // C 4145
      #[cfg (not (feature = "gcc_lint"))]
      const PACIFY_GCC_BUG_81401: c_int = 0;                             // C 4147
      let mut buf: [c_char; 7 + PACIFY_GCC_BUG_81401 as usize] =
	[0; 7 + PACIFY_GCC_BUG_81401 as usize];                          // C 4149
      let mut str: *mut c_char = std::ptr::null_mut ();                   // C 4150
      let len: c_int = glyphless_bits! (len, glyph);                   // C 4151

      if glyphless_bits! (method, glyph)
	  == dispextern_unit_names::GLYPHLESS_DISPLAY_ACRONYM             // C 4153
	{
	  if len > 0                                                        // C 4155
	      && CHAR_TABLE_P (Vglyphless_char_display)                     // C 4156
	      && (CHAR_TABLE_EXTRA_SLOTS (XCHAR_TABLE (Vglyphless_char_display))
		  >= 1)                                                     // C 4157-4158
	    {
	      let mut acronym: Lisp_Object
		= if ! glyphless_bits! (for_no_font, glyph)               // C 4160-4161
		  { CHAR_TABLE_REF (Vglyphless_char_display,
				    glyphless_bits! (ch, glyph) as isize) }   // C 4162-4163
		  else { FAM_ELEM! ((*XCHAR_TABLE (Vglyphless_char_display)).extras, 0) };   // C 4164
	      if CONSP (acronym)                                             // C 4165
		{ acronym = XCAR (acronym); }                                // C 4166
	      if STRINGP (acronym)                                           // C 4167
		{ str = SSDATA (acronym); }                                  // C 4168
	    }
	}
      else if glyphless_bits! (method, glyph)
	       == dispextern_unit_names::GLYPHLESS_DISPLAY_HEX_CODE        // C 4171
	{
	  let ch: c_uint = glyphless_bits! (ch, glyph) as c_uint;         // C 4173
	  eassume! (ch as c_int <= MAX_CHAR);                                // C 4174
	  sprintf (buf.as_mut_ptr (), c"%0*X".as_ptr (),                     // C 4175
		   if ch < 0x10000 { 4 } else { 6 }, ch);
	  str = buf.as_mut_ptr ();                                           // C 4176
	}

      if ! str.is_null ()                                                // C 4179
	{
	  let upper_len: c_int = (len + 1) / 2;                              // C 4181

	  /* It is assured that all LEN characters in STR is ASCII.  */     // C 4183
	  j = 0;                                                             // C 4184
	  while j < len
	    {
	      char2b[j as usize]
		= ((*((*(*s).font).driver as *const font_driver)).encode_char.unwrap ())
		    ((*s).font, *str.offset (j as isize) as c_int) & 0xFFFF;   // C 4185
	      j += 1;                                                        // C 4184 (the `for` step)
	    }
	  ((*((*(*s).font).driver as *const font_driver)).draw.unwrap ())
	    (s, 0, upper_len,                                                // C 4186
	     x + (*glyph).slice.glyphless.upper_xoff as c_int,               // C 4187
	     (*s).ybase + (*glyph).slice.glyphless.upper_yoff as c_int,      // C 4188
	     false);                                                         // C 4189
	  ((*((*(*s).font).driver as *const font_driver)).draw.unwrap ())
	    (s, upper_len, len,                                              // C 4190
	     x + (*glyph).slice.glyphless.lower_xoff as c_int,               // C 4191
	     (*s).ybase + (*glyph).slice.glyphless.lower_yoff as c_int,      // C 4192
	     false);                                                         // C 4193
	}
      if glyphless_bits! (method, glyph)
	  != dispextern_unit_names::GLYPHLESS_DISPLAY_THIN_SPACE            // C 4195
	{
	  android_draw_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f),         // C 4196
				  (*s).gc as *mut android_gc, x, (*s).ybase - (*glyph).ascent as c_int,   // C 4197
				  ((*glyph).pixel_width - 1) as c_uint,          // C 4198
				  ((*glyph).ascent + (*glyph).descent - 1) as c_uint);   // C 4199
	}
      x += (*glyph).pixel_width as c_int;                                 // C 4200
      glyph = glyph.offset (1);                                           // C 4142 (the `for` step)
      i += 1;
   }

  /* Defend against hypothetical bad code elsewhere that uses
     s->char2b after this function returns.  */                          // C 4203-4204
  (*s).char2b = std::ptr::null_mut ();                                    // C 4205
}

/* C 4208-4239: `android_draw_dash`.  C's `struct android_gc_values gcv` is written field by
   field and passed whole, so it is zero-initialised first (PORTING.md 1.5, the `src/xterm.rs`
   12645-12697 reading of the same function); C's `&gcv`/`&segment` are the port's `&raw mut`,
   and the three `android_*` calls are the mirror's (`include/androidgui.rs` 928-948).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_dash (f: *mut frame, s: *mut glyph_string, width: c_int,
			     mut segment: c_int, offset: c_int, thickness: c_int)
{
  let mut gcv: android_gc_values = std::mem::zeroed ();                  // C 4217 (PORTING.md 1.5)
  let mut y_center: c_int;                                               // C 4218

  /* Configure the GC, the dash pattern and a suitable offset.  */       // C 4220
  let gc: *mut android_gc = (*s).gc as *mut android_gc;                  // C 4221

  gcv.line_style = ANDROID_LINE_ON_OFF_DASH;                             // C 4223
  gcv.line_width = thickness;                                            // C 4224
  android_change_gc ((*s).gc as *mut android_gc, (ANDROID_GC_LINE_STYLE  // C 4225-4226
						  | ANDROID_GC_LINE_WIDTH),
		     &raw mut gcv);
  android_set_dashes ((*s).gc as *mut android_gc, (*s).x, &raw mut segment, 1);   // C 4227

  /* Offset the origin of the line by half the line width. */            // C 4229
  y_center = (*s).ybase + offset + thickness / 2;                        // C 4230
  android_draw_line (FRAME_ANDROID_WINDOW! (f), gc,                      // C 4231
		     (*s).x, y_center, (*s).x + width, y_center);        // C 4232

  /* Restore the initial line style.  */                                 // C 4234
  gcv.line_style = ANDROID_LINE_SOLID;                                   // C 4235
  gcv.line_width = 1;                                                    // C 4236
  android_change_gc ((*s).gc as *mut android_gc, (ANDROID_GC_LINE_STYLE  // C 4237-4238
						  | ANDROID_GC_LINE_WIDTH),
		     &raw mut gcv);
}

/* C 4241-4280: `android_fill_underline`.  C's `switch` is the port's `match` and the
   `FALLTHROUGH` (lib/attribute.h) from the DOTS arm into the DASHES one is the shared entry
   stage the two arms carry (PORTING.md 2.2; the `macro-body-notes.tsv` row the `--check` gate
   asks for is appended by this slice) - the `src/xterm.rs` 12700-12735 and `src/w32term.rs`
   4083-4120 readings of the same function.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_fill_underline (f: *mut frame, s: *mut glyph_string,
				  style: face_underline_type, position: c_int,
				  decoration_width: c_int, thickness: c_int)
{
  let mut segment: c_int;                                                // C 4250

  segment = thickness * 3;                                               // C 4252

  match style
    {
      /* FACE_UNDERLINE_DOUBLE_LINE is treated identically to SINGLE, as
	 the second line will be filled by another invocation of this
	 function.  */                                                    // C 4256-4258
      FACE_UNDERLINE_SINGLE | FACE_UNDERLINE_DOUBLE_LINE =>              // C 4259-4260
	{
	  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f),           // C 4261
				  (*s).gc as *mut android_gc, (*s).x, (*s).ybase + position,   // C 4262
				  decoration_width as c_uint, thickness as c_uint);   // C 4263
	}

      FACE_UNDERLINE_DOTS | FACE_UNDERLINE_DASHES =>                     // C 4266-4270
	{
	  /* C's `FALLTHROUGH;` (C 4268) sends the DOTS arm on into the DASHES arm's
	     body, so the one statement below is the entry stage both arms share.  */
	  if style == FACE_UNDERLINE_DOTS                                // C 4266
	    { segment = thickness; }                                     // C 4267
	  android_draw_dash (f, s, decoration_width, segment, position,  // C 4271
			     thickness);                                 // C 4272
	}

      FACE_NO_UNDERLINE | FACE_UNDERLINE_WAVE | _ =>                     // C 4275-4277
	{ emacs_abort (); }                                              // C 4278
    }
}

/* One name this text needs is not in the frozen mirrors, so it stands at its use site below
   (P-72 rule 4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3638: `extern struct font *font_for_underline_metrics (struct glyph_string
  /// *);` (xdisp.c)
  pub fn font_for_underline_metrics (s: *mut glyph_string) -> *mut font;
}

/* C 4282-4646: `android_draw_glyph_string`.  The three walks C writes as `for` loops keep their
   bodies and the port's `while` (the `src/xterm.rs` 12751-13163 and `src/haikuterm.rs` 1440-1560
   readings of the same text); `switch (s->first_glyph->type)` is the port's `match` on the
   mirror's `glyph_bits!` value with `emacs_abort` for `default:`, `s->first_glyph->u.cmp
   .automatic` is `glyph_u_cmp_bits!`, and every `bool_bf` member (`for_overlaps`,
   `background_filled_p`, the face's `underline*`/`overline*`/`strike_through*` family) goes
   through its accessor.  The underline arithmetic keeps C's `unsigned long` locals and therefore
   C's conversions and wraps; C 4568's `FRAME_ANDROID_WINDOW` (against 4576's
   `FRAME_ANDROID_DRAWABLE`) is carried exactly as C writes it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* C 6664-6696's table takes this function's address, so the port declares it
   `unsafe extern "C" fn` - the C-ABI rule for an address-taken item (slice 14-onward's
   `android_sync_edit` shape).  */
unsafe extern "C" fn android_draw_glyph_string (s: *mut glyph_string)
{
  let face: *mut face = (*s).face as *mut face;
  let mut relief_drawn_p: bool = false;                                 // C 4285

  /* If S draws into the background of its successors, draw the
     background of the successors first so that S can draw into it.
     This makes S->next use XDrawString instead of XDrawImageString.  */   // C 4287-4289
  if ! (*s).next.is_null () && (*s).right_overhang != 0                 // C 4290
      && glyph_string_bits! (for_overlaps, (*s)) == 0
    {
      let mut width: c_int;                                             // C 4292
      let mut next: *mut glyph_string;                                  // C 4293

      width = 0;                                                        // C 4295
      next = (*s).next;
      while ! next.is_null () && width < (*s).right_overhang
	{
	  if glyph_bits! (type, (*(*next).first_glyph)) != IMAGE_GLYPH     // C 4298
	    {
	      android_set_glyph_string_gc (next);                          // C 4300
	      android_set_glyph_string_clipping (next);                    // C 4301
	      if glyph_bits! (type, (*(*next).first_glyph)) == STRETCH_GLYPH   // C 4302
		{ android_draw_stretch_glyph_string (next); }              // C 4303
	      else
		{ android_draw_glyph_string_background (next, true); }     // C 4305
	      (*next).num_clips = 0;                                       // C 4306
	    }
	  width += (*next).width;                                          // C 4297 (the `for` step)
	  next = (*next).next;
	}
    }

  /* Set up S->gc, set clipping and draw S.  */                          // C 4310
  android_set_glyph_string_gc (s);                                       // C 4311

  /* Draw relief (if any) in advance for char/composition so that the
     glyph string can be drawn over it.  */                             // C 4313-4314
  if glyph_string_bits! (for_overlaps, (*s)) == 0                       // C 4315
      && face_bits! (box, face) != FACE_NO_BOX                          // C 4316
      && (glyph_bits! (type, (*(*s).first_glyph)) == CHAR_GLYPH         // C 4317
	  || glyph_bits! (type, (*(*s).first_glyph)) == COMPOSITE_GLYPH)   // C 4318

    {
      android_set_glyph_string_clipping (s);                            // C 4321
      android_draw_glyph_string_background (s, true);                   // C 4322
      android_draw_glyph_string_box (s);                                // C 4323
      android_set_glyph_string_clipping (s);                            // C 4324
      relief_drawn_p = true;                                            // C 4325
    }
  else if (*s).clip_head.is_null () /* draw_glyphs didn't specify a clip mask. */   // C 4327
	   && (*s).clip_tail.is_null ()                                 // C 4328
	   && ((! (*s).prev.is_null () && (*((*s).prev)).hl != (*s).hl   // C 4329
		&& (*s).left_overhang != 0)
	       || (! (*s).next.is_null () && (*((*s).next)).hl != (*s).hl   // C 4330
		   && (*s).right_overhang != 0))
    /* We must clip just this glyph.  left_overhang part has already
       drawn when s->prev was drawn, and right_overhang part will be
       drawn later when s->next is drawn. */                            // C 4331-4333
    { android_set_glyph_string_clipping_exactly (s, s); }               // C 4334
  else
    { android_set_glyph_string_clipping (s); }                          // C 4336

  match glyph_bits! (type, (*(*s).first_glyph))                         // C 4338
    {
      IMAGE_GLYPH =>                                                    // C 4340
	{ android_draw_image_glyph_string (s); }                        // C 4341

      XWIDGET_GLYPH =>                                                  // C 4344
	{ emacs_abort (); }                                             // C 4345

      STRETCH_GLYPH =>                                                  // C 4348
	{ android_draw_stretch_glyph_string (s); }                      // C 4349

      CHAR_GLYPH =>                                                     // C 4352
	{
	  if glyph_string_bits! (for_overlaps, (*s)) != 0                // C 4353
	    { glyph_string_bits! (background_filled_p, (*s), true); }    // C 4354
	  else
	    { android_draw_glyph_string_background (s, false); }         // C 4356
	  android_draw_glyph_string_foreground (s);                      // C 4357
	}

      COMPOSITE_GLYPH =>                                                // C 4360
	{
	  if glyph_string_bits! (for_overlaps, (*s)) != 0                // C 4361
	      || ((*s).cmp_from > 0
		  && ! glyph_u_cmp_bits! (automatic, (*s).first_glyph))    // C 4362
	    { glyph_string_bits! (background_filled_p, (*s), true); }    // C 4363
	  else
	    { android_draw_glyph_string_background (s, true); }          // C 4365
	  android_draw_composite_glyph_string_foreground (s);            // C 4366
	}

      GLYPHLESS_GLYPH =>                                                // C 4369
	{
	  if glyph_string_bits! (for_overlaps, (*s)) != 0                // C 4370
	    { glyph_string_bits! (background_filled_p, (*s), true); }    // C 4371
	  else
	    { android_draw_glyph_string_background (s, true); }          // C 4373
	  android_draw_glyphless_glyph_string_foreground (s);            // C 4374
	}

      _ =>                                                              // C 4377
	{ emacs_abort (); }                                             // C 4378
    }

  if glyph_string_bits! (for_overlaps, (*s)) == 0                       // C 4381
    {
      /* C leaves these four uninitialised until `window_box` writes them through their
	 addresses; the port gives them a value because `&raw mut` needs an initialised place
	 (the C 3931 `scale_x`/`scale_y` device above).  */
      let mut area_x: c_int = 0;                                        // C 4383
      let mut area_y: c_int = 0;
      let mut area_width: c_int = 0;
      let mut area_height: c_int = 0;
      let mut area_max_x: c_int;                                        // C 4384
      let mut decoration_width: c_int;

      /* Prevent the underline from overwriting surrounding areas
	 and the fringe.  */                                            // C 4386-4387
      window_box ((*s).w, (*s).area, &raw mut area_x, &raw mut area_y,   // C 4388
		  &raw mut area_width, &raw mut area_height);
      area_max_x = area_x + area_width - 1;                             // C 4390

      decoration_width = (*s).width;                                    // C 4392
      if ! glyph_row_bits! (mode_line_p, (*s).row)                       // C 4393
	  && ! glyph_row_bits! (tab_line_p, (*s).row)                    // C 4394
	  && area_max_x < ((*s).x + decoration_width - 1)                // C 4395
	{ decoration_width -= ((*s).x + decoration_width - 1) - area_max_x; }   // C 4396

      /* Draw relief if not yet drawn.  */                              // C 4398
      if ! relief_drawn_p && face_bits! (box, face) != FACE_NO_BOX       // C 4399
	{ android_draw_glyph_string_box (s); }                          // C 4400

      /* Draw underline.  */                                           // C 4402
      if face_bits! (underline, face) != 0                              // C 4403
	{
	  if face_bits! (underline, face) == FACE_UNDERLINE_WAVE          // C 4405
	    {
	      if face_bits! (underline_defaulted_p, face)                 // C 4407
		{ android_draw_underwave (s, decoration_width); }         // C 4408
	      else
		{
		  let mut xgcv: android_gc_values = std::mem::zeroed ();    // C 4411
		  android_get_gc_values ((*s).gc as *mut android_gc,       // C 4412
					 ANDROID_GC_FOREGROUND, &raw mut xgcv);
		  android_set_foreground ((*s).gc as *mut android_gc, (*face).underline_color);   // C 4413
		  android_draw_underwave (s, decoration_width);           // C 4414
		  android_set_foreground ((*s).gc as *mut android_gc, xgcv.foreground);   // C 4415
		}
	    }
	  else if face_bits! (underline, face) >= FACE_UNDERLINE_SINGLE    // C 4418
	    {
	      let mut thickness: c_ulong;                                  // C 4420
	      let mut position: c_ulong;

	      if ! (*s).prev.is_null ()                                    // C 4422
		  && (face_bits! (underline, (*((*s).prev)).face as *mut face)   // C 4423
		      != FACE_UNDERLINE_WAVE
		      && face_bits! (underline, (*((*s).prev)).face as *mut face)   // C 4424
			 >= FACE_UNDERLINE_SINGLE)
		  && (face_bits! (underline_at_descent_line_p,             // C 4425
				  (*((*s).prev)).face as *mut face)
		      == face_bits! (underline_at_descent_line_p, face))   // C 4426
		  && ((*( (*((*s).prev)).face as *mut face )).underline_pixels_above_descent_line   // C 4427
		      == (*face).underline_pixels_above_descent_line)      // C 4428
		{
		  /* We use the same underline style as the previous one.  */   // C 4430
		  thickness = (*(*s).prev).underline_thickness as c_ulong;   // C 4431
		  position = (*(*s).prev).underline_position as c_ulong;   // C 4432
		}
	      else
		{
		  let font: *mut font = font_for_underline_metrics (s);     // C 4436
		  let mut minimum_offset: c_ulong;                          // C 4437
		  let mut underline_at_descent_line: bool;                  // C 4438
		  let mut use_underline_position_properties: bool;          // C 4439
		  let mut val: Lisp_Object = (WINDOW_BUFFER_LOCAL_VALUE     // C 4440-4441
					      (Qunderline_minimum_offset (), (*s).w));

		  if FIXNUMP (val)                                          // C 4443
		    { minimum_offset = max (0, XFIXNUM (val)) as c_ulong; }   // C 4444
		  else
		    { minimum_offset = 1; }                                 // C 4446

		  val = (WINDOW_BUFFER_LOCAL_VALUE                          // C 4448-4449
			 (Qx_underline_at_descent_line (), (*s).w));
		  underline_at_descent_line
		    = (! (NILP (val) || BASE_EQ (val, Qunbound ()))          // C 4451
		       || face_bits! (underline_at_descent_line_p, face));   // C 4452

		  val = (WINDOW_BUFFER_LOCAL_VALUE                          // C 4454-4455
			 (Qx_use_underline_position_properties (), (*s).w));
		  use_underline_position_properties
		    = ! (NILP (val) || BASE_EQ (val, Qunbound ()));          // C 4457

		  /* Get the underline thickness.  Default is 1 pixel.  */   // C 4459
		  if ! font.is_null () && (*font).underline_thickness > 0    // C 4460
		    { thickness = (*font).underline_thickness as c_ulong; }   // C 4461
		  else
		    { thickness = 1; }                                       // C 4463
		  if underline_at_descent_line                              // C 4464
		    {
		      position = (((*s).height) as c_ulong).wrapping_sub (thickness)   // C 4465
				 .wrapping_sub (((*s).ybase - (*s).y) as c_ulong)   // C 4466
				 .wrapping_sub ((*face).underline_pixels_above_descent_line   // C 4467
						as c_ulong);
		    }
		  else
		    {
		      /* Get the underline position.  This is the
			 recommended vertical offset in pixels from
			 the baseline to the top of the underline.
			 This is a signed value according to the
			 specs, and its default is

			 ROUND ((maximum descent) / 2), with
			 ROUND(x) = floor (x + 0.5)  */                  // C 4470-4477

		      if use_underline_position_properties                  // C 4479
			  && ! font.is_null () && (*font).underline_position >= 0   // C 4480
			{ position = (*font).underline_position as c_ulong; }   // C 4481
		      else if ! font.is_null ()                             // C 4482
			{ position = (((*font).descent + 1) / 2) as c_ulong; }   // C 4483
		      else
			{ position = minimum_offset; }                      // C 4485
		    }

		  /* Ignore minimum_offset if the amount of pixels was
		     explicitly specified.  */                              // C 4488-4489
		  if (*face).underline_pixels_above_descent_line == 0       // C 4490
		    { position = max (position as isize, minimum_offset as isize) as c_ulong; }   // C 4491
		}
	      /* Check the sanity of thickness and position.  We should
		 avoid drawing underline out of the current line area.  */   // C 4493-4494
	      if (((*s).y + (*s).height) as c_ulong)
		  <= ((*s).ybase as c_ulong).wrapping_add (position)        // C 4495
		{ position = (((*s).height - 1) - ((*s).ybase - (*s).y)) as c_ulong; }   // C 4496
	      if (((*s).y + (*s).height) as c_ulong)
		  < ((*s).ybase as c_ulong).wrapping_add (position).wrapping_add (thickness)   // C 4497
		{
		  thickness = (((*s).y + (*s).height) as c_ulong)         // C 4498
			      .wrapping_sub ((*s).ybase as c_ulong + position);
		}
	      (*s).underline_thickness = thickness as c_int;              // C 4499
	      (*s).underline_position = position as c_int;                // C 4500

	      {
		let mut xgcv: android_gc_values = std::mem::zeroed ();     // C 4503

		if ! face_bits! (underline_defaulted_p, face)             // C 4505
		  {
		    android_get_gc_values ((*s).gc as *mut android_gc,     // C 4507
					   ANDROID_GC_FOREGROUND, &raw mut xgcv);
		    android_set_foreground ((*s).gc as *mut android_gc, (*face).underline_color);   // C 4508
		  }

		android_fill_underline ((*s).f, s, face_bits! (underline, face),   // C 4511
					position as c_int, decoration_width,    // C 4512
					thickness as c_int);                    // C 4513

		/* Place a second underline above the first if this was
		   requested in the face specification.  */               // C 4515-4516

		if face_bits! (underline, face) == FACE_UNDERLINE_DOUBLE_LINE   // C 4518
		  {
		    /* Compute the position of the second underline.  */    // C 4520
		    position = position.wrapping_sub (thickness).wrapping_sub (1);   // C 4521
		    android_fill_underline ((*s).f, s, face_bits! (underline, face),   // C 4522
					    position as c_int, decoration_width,   // C 4523
					    thickness as c_int);        // C 4524
		  }

		if ! face_bits! (underline_defaulted_p, face)             // C 4527
		  { android_set_foreground ((*s).gc as *mut android_gc, xgcv.foreground); }   // C 4528
	      }
	    }
	}
      /* Draw overline.  */                                            // C 4532
      if face_bits! (overline_p, face)                                 // C 4533
	{
	  let dy: c_ulong = 0;                                              // C 4535
	  let h: c_ulong = 1;

	  if face_bits! (overline_color_defaulted_p, face)                 // C 4537
	    {
	      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f),     // C 4538
				      (*s).gc as *mut android_gc, (*s).x,   // C 4539
				      ((*s).y as c_ulong + dy) as c_int,
				      decoration_width as c_uint, h as c_uint);   // C 4540
	    }
	  else
	    {
	      let mut xgcv: android_gc_values = std::mem::zeroed ();       // C 4543
	      android_get_gc_values ((*s).gc as *mut android_gc,           // C 4544
				     ANDROID_GC_FOREGROUND, &raw mut xgcv);
	      android_set_foreground ((*s).gc as *mut android_gc, (*face).overline_color);   // C 4545
	      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc,   // C 4546
				      (*s).x, ((*s).y as c_ulong + dy) as c_int,   // C 4547
				      decoration_width as c_uint, h as c_uint);
	      android_set_foreground ((*s).gc as *mut android_gc, xgcv.foreground);   // C 4548
	    }
	}

      /* Draw strike-through.  */                                      // C 4552
      if face_bits! (strike_through_p, face)                            // C 4553
	{
	  /* Y-coordinate and height of the glyph string's first
	     glyph.  We cannot use s->y and s->height because those
	     could be larger if there are taller display elements
	     (e.g., characters displayed with a larger font) in the
	     same glyph row.  */                                          // C 4555-4559
	  let glyph_y: c_int = (*s).ybase - (*((*s).first_glyph)).ascent as c_int;   // C 4560
	  let glyph_height: c_int = (*((*s).first_glyph)).ascent as c_int   // C 4561
				    + (*((*s).first_glyph)).descent as c_int;
	  /* Strike-through width and offset from the glyph string's
	     top edge.  */                                              // C 4562-4563
	  let h: c_ulong = 1;                                              // C 4564
	  let dy: c_ulong = ((glyph_height as c_ulong).wrapping_sub (h)) / 2;   // C 4565

	  if face_bits! (strike_through_color_defaulted_p, face)           // C 4567
	    {
	      android_fill_rectangle (FRAME_ANDROID_WINDOW! ((*s).f),      // C 4568
				      (*s).gc as *mut android_gc, (*s).x,   // C 4569
				      ((glyph_y as c_ulong + dy) as c_int),
				      (*s).width as c_uint, h as c_uint);   // C 4570
	    }
	  else
	    {
	      let mut xgcv: android_gc_values = std::mem::zeroed ();       // C 4573
	      android_get_gc_values ((*s).gc as *mut android_gc,           // C 4574
				     ANDROID_GC_FOREGROUND, &raw mut xgcv);
	      android_set_foreground ((*s).gc as *mut android_gc, (*face).strike_through_color);   // C 4575
	      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! ((*s).f), (*s).gc as *mut android_gc,   // C 4576
				      (*s).x, ((glyph_y as c_ulong + dy) as c_int),   // C 4577
				      decoration_width as c_uint, h as c_uint);   // C 4578
	      android_set_foreground ((*s).gc as *mut android_gc, xgcv.foreground);   // C 4579
	    }
	}

      if ! (*s).prev.is_null ()                                        // C 4583
	{
	  let mut prev: *mut glyph_string;                                 // C 4585

	  prev = (*s).prev;                                                // C 4587
	  while ! prev.is_null ()
	    {
	      if (*prev).hl != (*s).hl                                    // C 4588
		  && (*prev).x + (*prev).width + (*prev).right_overhang > (*s).x   // C 4589
		{
		  /* As prev was drawn while clipped to its own area, we
		     must draw the right_overhang part using s->hl now.  */   // C 4591-4592
		  let save: draw_glyphs_face = (*prev).hl;                 // C 4593

		  (*prev).hl = (*s).hl;                                    // C 4595
		  android_set_glyph_string_gc (prev);                      // C 4596
		  android_set_glyph_string_clipping_exactly (s, prev);     // C 4597
		  if glyph_bits! (type, (*(*prev).first_glyph)) == CHAR_GLYPH   // C 4598
		    { android_draw_glyph_string_foreground (prev); }       // C 4599
		  else
		    { android_draw_composite_glyph_string_foreground (prev); }   // C 4601
		  android_reset_clip_rectangles ((*prev).f, (*prev).gc as *mut android_gc);   // C 4602
		  (*prev).hl = save;                                       // C 4603
		  (*prev).num_clips = 0;                                   // C 4604
		}
	      prev = (*prev).prev;                                         // C 4587 (the `for` step)
	    }
	}

      if ! (*s).next.is_null ()                                        // C 4608
	{
	  let mut next: *mut glyph_string;                                 // C 4610

	  next = (*s).next;                                                // C 4612
	  while ! next.is_null ()
	    {
	      if (*next).hl != (*s).hl                                    // C 4613
		  && (*next).x - (*next).left_overhang < (*s).x + (*s).width   // C 4614
		{
		  /* As next will be drawn while clipped to its own area,
		     we must draw the left_overhang part using s->hl now.  */   // C 4616-4617
		  let save: draw_glyphs_face = (*next).hl;                 // C 4618

		  (*next).hl = (*s).hl;                                    // C 4620
		  android_set_glyph_string_gc (next);                      // C 4621
		  android_set_glyph_string_clipping_exactly (s, next);     // C 4622
		  if glyph_bits! (type, (*(*next).first_glyph)) == CHAR_GLYPH   // C 4623
		    { android_draw_glyph_string_foreground (next); }       // C 4624
		  else
		    { android_draw_composite_glyph_string_foreground (next); }   // C 4626
		  android_reset_clip_rectangles ((*next).f, (*next).gc as *mut android_gc);   // C 4627
		  (*next).hl = save;                                       // C 4628
		  (*next).num_clips = 0;                                   // C 4629
		  (*next).clip_head = (*s).next;                           // C 4630
		}
	      next = (*next).next;                                         // C 4612 (the `for` step)
	    }
	}
    }

  /* Reset clipping.  */                                               // C 4635
  android_reset_clip_rectangles ((*s).f, (*s).gc as *mut android_gc);   // C 4636
  (*s).num_clips = 0;                                                   // C 4637

  /* Set the stippled flag that tells redisplay whether or not a
     stipple was actually draw.  */                                    // C 4639-4640

  if glyph_bits! (type, (*(*s).first_glyph)) != STRETCH_GLYPH           // C 4642
      && glyph_bits! (type, (*(*s).first_glyph)) != IMAGE_GLYPH         // C 4643
      && ! glyph_row_bits! (stipple_p, (*s).row)                        // C 4644
    { glyph_row_bits! (stipple_p, (*s).row, glyph_string_bits! (stippled_p, (*s))); }   // C 4645
}

/* One name this text needs is not in the frozen mirrors' glob, so it stands at its use site
   below (P-72 rule 4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3813: `int lookup_basic_face (struct window *, struct frame *, int);`
  /// (xfaces.c)
  pub fn lookup_basic_face (w: *mut window, f: *mut frame, face_id: c_int) -> c_int;
}

/* C 4648-4657: `android_define_frame_cursor`.  `static`; the redisplay interface's
   `define_frame_cursor` member points at it (C 5016).  `f->pointer_invisible` is the frame's own
   folded `bool_bf` (this unit's `pointer_invisible!`), and `Emacs_Cursor` is the cursor handle the
   mirror's `android_output.current_cursor` field carries (`include/androidterm.rs` 283).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* C 6664-6696's table takes this function's address, so the port declares it
   `unsafe extern "C" fn` - the C-ABI rule for an address-taken item (slice 14-onward's
   `android_sync_edit` shape).  */
unsafe extern "C" fn android_define_frame_cursor (f: *mut frame, cursor: Emacs_Cursor)
{
  let x: *mut android_output = FRAME_ANDROID_OUTPUT! (f) as *mut android_output;

  if ! pointer_invisible! (f)                                          // C 4651
      && ! (*x).hourglass                                              // C 4652
      && (*x).current_cursor != cursor                                 // C 4653
    { android_define_cursor (FRAME_ANDROID_WINDOW! (f), cursor); }     // C 4654

  (*x).current_cursor = cursor;                                        // C 4656
}

/* C 4659-4665: `android_clear_frame_area`.  `static`; the redisplay interface's
   `clear_frame_area` member points at it (C 5017).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* C 6664-6696's table takes this function's address, so the port declares it
   `unsafe extern "C" fn` - the C-ABI rule for an address-taken item (slice 14-onward's
   `android_sync_edit` shape).  */
unsafe extern "C" fn android_clear_frame_area (f: *mut frame, x: c_int, y: c_int,
				    width: c_int, height: c_int)
{
  android_clear_area (FRAME_ANDROID_DRAWABLE! (f),                     // C 4663
		      x, y, width as c_uint, height as c_uint);        // C 4664
}

/* C 4667-4718: `android_clear_under_internal_border`.  Non-`static` in C and no address is
   taken: `#[unsafe(no_mangle)]` with the C ABI (PORTING.md 0.2).  C 4677-4685's nested `?:` is
   the port's nested `if`/`else` (the `src/haikuterm.rs` 6877-6892 reading of the same text), and
   `Vface_remapping_alist` is the globals mirror's and `FACE_FROM_ID_OR_NULL` the frame mirror's own
   module path.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
unsafe extern "C" fn android_clear_under_internal_border (f: *mut frame)
{
  if FRAME_INTERNAL_BORDER_WIDTH (f) > 0                               // C 4670
    {
      let border: c_int = FRAME_INTERNAL_BORDER_WIDTH (f);             // C 4672
      let width: c_int = FRAME_PIXEL_WIDTH (f);                        // C 4673
      let height: c_int = FRAME_PIXEL_HEIGHT (f);                      // C 4674
      let margin: c_int = FRAME_TOP_MARGIN_HEIGHT (f);                 // C 4675
      let bottom_margin: c_int = FRAME_BOTTOM_MARGIN_HEIGHT (f);       // C 4676
      let face_id: c_int = if ! FRAME_PARENT_FRAME (f).is_null ()      // C 4677
	{ if ! NILP (Vface_remapping_alist)                            // C 4678
	    { lookup_basic_face (std::ptr::null_mut (), f,             // C 4679
				 CHILD_FRAME_BORDER_FACE_ID) }         // C 4680
	  else
	    { CHILD_FRAME_BORDER_FACE_ID } }                           // C 4681
	else
	{ if ! NILP (Vface_remapping_alist)                            // C 4682
	    { lookup_basic_face (std::ptr::null_mut (), f,             // C 4683
				 INTERNAL_BORDER_FACE_ID) }            // C 4684
	  else
	    { INTERNAL_BORDER_FACE_ID } };                             // C 4685
      let face: *mut face = frame_face_accessors::FACE_FROM_ID_OR_NULL (f, face_id);   // C 4686

      if ! face.is_null ()                                             // C 4688
	{
	  let color: c_ulong = (*face).background;                      // C 4690
	  let gc: *mut android_gc
	    = (*( (*f).output_data.android as *mut android_output )).normal_gc;   // C 4691

	  android_set_foreground (gc, color);                           // C 4693
	  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc, 0, margin,   // C 4694
				  width as c_uint, border as c_uint);   // C 4695
	  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc, 0, 0,   // C 4696
				  border as c_uint, height as c_uint);   // C 4697
	  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc, width - border,   // C 4698
				  0, border as c_uint, height as c_uint);   // C 4699
	  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc, 0,   // C 4700
				  height - bottom_margin - border,      // C 4701
				  width as c_uint, border as c_uint);   // C 4702
	  android_set_foreground (gc, FRAME_FOREGROUND_PIXEL (f));      // C 4703
	}
      else
	{
	  android_clear_area (FRAME_ANDROID_DRAWABLE! (f), 0, 0,        // C 4707
			      border as c_uint, height as c_uint);      // C 4708
	  android_clear_area (FRAME_ANDROID_DRAWABLE! (f), 0,           // C 4709
			      margin, width as c_uint, border as c_uint);   // C 4710
	  android_clear_area (FRAME_ANDROID_DRAWABLE! (f), width - border,   // C 4711
			      0, border as c_uint, height as c_uint);   // C 4712
	  android_clear_area (FRAME_ANDROID_DRAWABLE! (f), 0,          // C 4713
			      height - bottom_margin - border,         // C 4714
			      width as c_uint, border as c_uint);      // C 4715
	}
    }
}

/* Four names this text needs are not in the frozen mirrors, so they stand at their use sites
   below (P-72 rule 4).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: window.h 1186: `struct glyph *get_phys_cursor_glyph (struct window *w);` (xdisp.c)
  pub fn get_phys_cursor_glyph (w: *mut window) -> *mut glyph;
  /// C: dispextern.h 3652-3653: `extern void get_phys_cursor_geometry (struct window *,
  /// struct glyph_row *, struct glyph *, int *, int *, int *);` (xdisp.c)
  pub fn get_phys_cursor_geometry (w: *mut window, row: *mut glyph_row, glyph: *mut glyph,
				   x: *mut c_int, y: *mut c_int, h: *mut c_int);
  /// C: dispextern.h 3649-3651: `extern void draw_phys_cursor_glyph (struct window *,
  /// struct glyph_row *, enum draw_glyphs_face);` (xdisp.c)
  pub fn draw_phys_cursor_glyph (w: *mut window, row: *mut glyph_row, draw: draw_glyphs_face);
  /// C: dispextern.h 3703: `void draw_fringe_bitmap (struct window *, struct glyph_row *,
  /// int);` (fringe.c)
  pub fn draw_fringe_bitmap (w: *mut window, row: *mut glyph_row, left_p: c_int);
}

/* C 4720-4765: `android_draw_hollow_cursor`.  `static`; the redisplay interface's
   `draw_hollow_cursor` member points at it (C 5015).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_hollow_cursor (w: *mut window, row: *mut glyph_row)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                       // C 4723
  let dpyinfo: *mut android_display_info = FRAME_DISPLAY_INFO! (f);      // C 4724
  /* C 4725 leaves `x`, `y` and `h` to `get_phys_cursor_geometry`'s writes through their
     addresses, so the port gives them a value (the eighth slice's `scale_x` device).  */
  let mut x: c_int = 0;                                                 // C 4725
  let mut y: c_int = 0;
  let mut wd: c_int;
  let mut h: c_int = 0;
  let mut xgcv: android_gc_values = std::mem::zeroed ();                 // C 4726
  let mut cursor_glyph: *mut glyph;                                     // C 4727
  let gc: *mut android_gc;                                              // C 4728

  /* Get the glyph the cursor is on.  If we can't tell because
     the current matrix is invalid or such, give up.  */               // C 4730-4731
  cursor_glyph = get_phys_cursor_glyph (w);                             // C 4732
  if cursor_glyph.is_null ()                                            // C 4733
    { return; }                                                         // C 4734

  /* Compute frame-relative coordinates for phys cursor.  */            // C 4736
  get_phys_cursor_geometry (w, row, cursor_glyph, &raw mut x, &raw mut y, &raw mut h);   // C 4737
  wd = (*w).phys_cursor_width - 1;                                      // C 4738

  /* The foreground of cursor_gc is typically the same as the normal
     background color, which can cause the cursor box to be invisible.  */   // C 4740-4741
  xgcv.foreground = (*((*f).output_data.android as *mut android_output)).cursor_pixel;   // C 4742
  if ! (*dpyinfo).scratch_cursor_gc.is_null ()                          // C 4743
    {
      android_change_gc ((*dpyinfo).scratch_cursor_gc,                  // C 4744
			 ANDROID_GC_FOREGROUND, &raw mut xgcv);         // C 4745
    }
  else
    {
      (*dpyinfo).scratch_cursor_gc
	= android_create_gc (ANDROID_GC_FOREGROUND, &raw mut xgcv);     // C 4747-4748
    }
  gc = (*dpyinfo).scratch_cursor_gc;                                    // C 4749

  /* When on R2L character, show cursor at the right edge of the
     glyph, unless the cursor box is as wide as the glyph or wider
     (the latter happens when x-stretch-cursor is non-nil).  */         // C 4751-4753
  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0           // C 4754
      && (*cursor_glyph).pixel_width as c_int > wd                     // C 4755
    {
      x += (*cursor_glyph).pixel_width as c_int - wd;                   // C 4757
      if wd > 0                                                         // C 4758
	{ wd -= 1; }                                                    // C 4759
    }
  /* Set clipping, draw the rectangle, and reset clipping again.  */    // C 4761
  android_clip_to_row (w, row, TEXT_AREA, gc, std::ptr::null_mut ());   // C 4762
  android_draw_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc, x, y,        // C 4763
			  wd as c_uint, (h - 1) as c_uint);
  android_reset_clip_rectangles (f, gc);                                // C 4764
}

/* C 4767-4867: `android_draw_bar_cursor`.  `static`; only called (C 4904/4908).  C 4798's
   `unsigned long mask` is the port's `c_ulong` with C's own narrowing at `android_change_gc`
   and `android_create_gc`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_draw_bar_cursor (w: *mut window, row: *mut glyph_row, mut width: c_int,
				   kind: text_cursor_kinds)
{
  let f: *mut frame = XFRAME ((*w).frame);                              // C 4771
  let mut cursor_glyph: *mut glyph;                                     // C 4772
  let mut cursor_start_y: c_int;                                        // C 4773

  /* If cursor is out of bounds, don't draw garbage.  This can happen
     in mini-buffer windows when switching between echo area glyphs
     and mini-buffer.  */                                               // C 4775-4777
  cursor_glyph = get_phys_cursor_glyph (w);                             // C 4778
  if cursor_glyph.is_null ()                                            // C 4779
    { return; }                                                         // C 4780

  /* Experimental avoidance of cursor on xwidget.  */                    // C 4782
  if glyph_bits! (type, (*cursor_glyph)) == XWIDGET_GLYPH               // C 4783
    { return; }                                                         // C 4784

  /* If on an image, draw like a normal cursor.  That's usually better
     visible than drawing a bar, esp. if the image is large so that
     the bar might not be in the window.  */                            // C 4786-4788
  if glyph_bits! (type, (*cursor_glyph)) == IMAGE_GLYPH                 // C 4789
    {
      let mut r: *mut glyph_row;                                        // C 4791
      r = MATRIX_ROW! ((*w).current_matrix as *mut glyph_matrix,     // C 4792
			(*w).phys_cursor.vpos);
      draw_phys_cursor_glyph (w, r, DRAW_CURSOR);                       // C 4793
    }
  else
    {
      let mut gc: *mut android_gc
	= (*FRAME_DISPLAY_INFO! (f)).scratch_cursor_gc;                 // C 4797
      let mask: c_ulong
	= (ANDROID_GC_FOREGROUND | ANDROID_GC_BACKGROUND) as c_ulong;   // C 4798
      let face: *mut face
	= frame_face_accessors::FACE_FROM_ID (f, glyph_bits! (face_id, (*cursor_glyph)));   // C 4799
      let mut xgcv: android_gc_values = std::mem::zeroed ();            // C 4800

      /* If the glyph's background equals the color we normally draw
	 the bars cursor in, the bar cursor in its normal color is
	 invisible.  Use the glyph's foreground color instead in this
	 case, on the assumption that the glyph's colors are chosen so
	 that the glyph is legible.  */                                 // C 4802-4806
      if (*face).background
	 == (*((*f).output_data.android as *mut android_output)).cursor_pixel   // C 4807
	{ xgcv.background = (*face).foreground;                          // C 4808
	  xgcv.foreground = (*face).foreground; }
      else
	{ xgcv.background = (*((*f).output_data.android as *mut android_output)).cursor_pixel;   // C 4810
	  xgcv.foreground = (*((*f).output_data.android as *mut android_output)).cursor_pixel; }

      if ! gc.is_null ()                                                // C 4812
	{ android_change_gc (gc, mask as android_gc_value_mask, &raw mut xgcv); }   // C 4813
      else
	{
	  gc = android_create_gc (mask as android_gc_value_mask, &raw mut xgcv);   // C 4816
	  (*FRAME_DISPLAY_INFO! (f)).scratch_cursor_gc = gc;            // C 4817
	}

      android_clip_to_row (w, row, TEXT_AREA, gc, std::ptr::null_mut ());   // C 4820

      if kind == BAR_CURSOR                                             // C 4822
	{
	  let mut x: c_int = WINDOW_TEXT_TO_FRAME_PIXEL_X! (w, (*w).phys_cursor.x);   // C 4824

	  if width < 0                                                      // C 4826
	    { width = FRAME_CURSOR_WIDTH (f); }                            // C 4827
	  width = min ((*cursor_glyph).pixel_width as c_int, width);        // C 4828

	  (*w).phys_cursor_width = width;                                  // C 4830

	  /* If the character under cursor is R2L, draw the bar cursor
	     on the right of its glyph, rather than on the left.  */     // C 4832-4833
	  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0      // C 4834
	    { x += (*cursor_glyph).pixel_width as c_int - width; }         // C 4835

	  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc, x,     // C 4837
				  WINDOW_TO_FRAME_PIXEL_Y! (w, (*w).phys_cursor.y),   // C 4838
				  width as c_uint, (*row).height as c_uint);   // C 4839
	}
      else /* HBAR_CURSOR */                                            // C 4841
	{
	  let mut dummy_x: c_int = 0;                                     // C 4843
	  let mut dummy_y: c_int = 0;
	  let mut dummy_h: c_int = 0;
	  let mut x: c_int = WINDOW_TEXT_TO_FRAME_PIXEL_X! (w, (*w).phys_cursor.x);   // C 4844

	  if width < 0                                                      // C 4846
	    { width = (*row).height; }                                     // C 4847

	  width = min ((*row).height, width);                              // C 4849

	  get_phys_cursor_geometry (w, row, cursor_glyph, &raw mut dummy_x,   // C 4851
				    &raw mut dummy_y, &raw mut dummy_h);   // C 4852

	  cursor_start_y = WINDOW_TO_FRAME_PIXEL_Y! (w, (*w).phys_cursor.y   // C 4854
						     + (*row).height - width);   // C 4855

	  if (glyph_bits! (resolved_level, (*cursor_glyph)) & 1) != 0      // C 4857
	      && (*cursor_glyph).pixel_width as c_int > (*w).phys_cursor_width - 1   // C 4858
	    { x += (*cursor_glyph).pixel_width as c_int - (*w).phys_cursor_width + 1; }   // C 4859
	  android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f), gc, x,     // C 4860
				  cursor_start_y,                        // C 4861
				  ((*w).phys_cursor_width - 1) as c_uint, width as c_uint);   // C 4862
	}

      android_reset_clip_rectangles (f, gc);                            // C 4865
    }
}

/* C 4869-4927: `android_draw_window_cursor`.  `static`; the redisplay interface's
   `draw_window_cursor` member points at it (C 5018).  C's `switch` is the port's `match`;
   `w->phys_cursor_on_p` is the arm this slice adds to the unit's `window_bits!`, and
   `FRAME_OUTPUT_DATA (f)->need_cursor_updates` is the folded bit the unit reads with its mask
   (P-80, the C 707-708 device).  `active_p` has no reader in C's body.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* C 6664-6696's table takes this function's address, so the port declares it
   `unsafe extern "C" fn` - the C-ABI rule for an address-taken item (slice 14-onward's
   `android_sync_edit` shape).  */
unsafe extern "C" fn android_draw_window_cursor (w: *mut window, glyph_row: *mut glyph_row,
				      x: c_int, y: c_int, cursor_type: text_cursor_kinds,
				      cursor_width: c_int, on_p: bool, active_p: bool)
{
  let f: *mut frame;                                                    // C 4874

  let _ = active_p;
  f = WINDOW_XFRAME! (w);                                               // C 4876

  if on_p                                                               // C 4878
    {
      (*w).phys_cursor_type = cursor_type;                              // C 4880
      window_bits! (phys_cursor_on_p, w, true);                         // C 4881

      if glyph_row_bits! (exact_window_width_line_p, glyph_row)          // C 4883
	  && (if glyph_row_bits! (reversed_p, glyph_row)                // C 4884
	      { (*w).phys_cursor.hpos < 0 }                             // C 4885
	      else
	      { (*w).phys_cursor.hpos >= (*glyph_row).used[TEXT_AREA as usize] as c_int })   // C 4886
	{
	  glyph_row_bits! (cursor_in_fringe_p, glyph_row, true);          // C 4888
	  draw_fringe_bitmap (w, glyph_row,                                // C 4889
			      glyph_row_bits! (reversed_p, glyph_row) as c_int);
	}
      else
	{
	  match cursor_type                                              // C 4893
	    {
	    HOLLOW_BOX_CURSOR =>                                         // C 4895
	      { android_draw_hollow_cursor (w, glyph_row); }             // C 4896

	    FILLED_BOX_CURSOR =>                                         // C 4899
	      { draw_phys_cursor_glyph (w, glyph_row, DRAW_CURSOR); }    // C 4900

	    BAR_CURSOR =>                                                // C 4903
	      { android_draw_bar_cursor (w, glyph_row, cursor_width, BAR_CURSOR); }   // C 4904

	    HBAR_CURSOR =>                                               // C 4907
	      { android_draw_bar_cursor (w, glyph_row, cursor_width, HBAR_CURSOR); }   // C 4908

	    NO_CURSOR =>                                                 // C 4911
	      { (*w).phys_cursor_width = 0; }                            // C 4912

	    _ =>                                                         // C 4915
	      { emacs_abort (); }                                        // C 4916
	    }
	}

      /* Now proceed to tell the input method the current position of
	 the cursor, if required.  */                                  // C 4920-4921

      if (*(FRAME_OUTPUT_DATA! (f) as *mut android_output)).bits
	  & ANDROID_OUTPUT_NEED_CURSOR_UPDATES != 0                      // C 4923
	  && w == XWINDOW ((*f).selected_window)                         // C 4924
	{ android_set_preeditarea (w as *mut c_void, x, y); }            // C 4925
    }
}

/* C 4929-4943: `android_draw_vertical_window_border`.  `static`; the redisplay interface's
   `draw_vertical_window_border` member points at it (C 5019).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* C 6664-6696's table takes this function's address, so the port declares it
   `unsafe extern "C" fn` - the C-ABI rule for an address-taken item (slice 14-onward's
   `android_sync_edit` shape).  */
unsafe extern "C" fn android_draw_vertical_window_border (w: *mut window, x: c_int, y0: c_int, y1: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                       // C 4932
  let face: *mut face;                                                  // C 4933

  face = frame_face_accessors::FACE_FROM_ID_OR_NULL (f, VERTICAL_BORDER_FACE_ID);   // C 4935
  if ! face.is_null ()                                                  // C 4936
    {
      android_set_foreground ((*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4937
			      (*face).foreground);                      // C 4938
    }

  android_draw_line (FRAME_ANDROID_DRAWABLE! (f),                       // C 4940
		     (*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4941
		     x, y0, x, y1);                                     // C 4942
}

/* C 4945-5010: `android_draw_window_divider`.  `static`; the redisplay interface's
   `draw_window_divider` member points at it (C 5020).  C's three `?:` are the port's
   `if`/`else` expressions.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
/* C 6664-6696's table takes this function's address, so the port declares it
   `unsafe extern "C" fn` - the C-ABI rule for an address-taken item (slice 14-onward's
   `android_sync_edit` shape).  */
unsafe extern "C" fn android_draw_window_divider (w: *mut window, x0: c_int, x1: c_int,
				       y0: c_int, y1: c_int)
{
  let f: *mut frame = XFRAME (WINDOW_FRAME! (w));                       // C 4948
  let face: *mut face
    = frame_face_accessors::FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_FACE_ID);   // C 4949
  let face_first: *mut face
    = frame_face_accessors::FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_FIRST_PIXEL_FACE_ID);   // C 4950-4951
  let face_last: *mut face
    = frame_face_accessors::FACE_FROM_ID_OR_NULL (f, WINDOW_DIVIDER_LAST_PIXEL_FACE_ID);   // C 4952-4953
  let color: c_ulong = if ! face.is_null ()                             // C 4954
    { (*face).foreground } else { FRAME_FOREGROUND_PIXEL (f) };
  let color_first: c_ulong = if ! face_first.is_null ()                 // C 4955-4956
    { (*face_first).foreground } else { FRAME_FOREGROUND_PIXEL (f) };
  let color_last: c_ulong = if ! face_last.is_null ()                   // C 4958-4959
    { (*face_last).foreground } else { FRAME_FOREGROUND_PIXEL (f) };

  if (y1 - y0 > x1 - x0) && (x1 - x0 >= 3)                              // C 4962
    /* A vertical divider, at least three pixels wide: Draw first and
       last pixels differently.  */                                    // C 4963-4964
    {
      android_set_foreground ((*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4966
			      color_first);                             // C 4967
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f),              // C 4968
			      (*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4969
			      x0, y0, 1 as c_uint, (y1 - y0) as c_uint);   // C 4970
      android_set_foreground ((*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4971
			      color);                                   // C 4972
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f),              // C 4973
			      (*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4974
			      x0 + 1, y0, (x1 - x0 - 2) as c_uint, (y1 - y0) as c_uint);   // C 4975
      android_set_foreground ((*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4976
			      color_last);                              // C 4977
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f),              // C 4978
			      (*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4979
			      x1 - 1, y0, 1 as c_uint, (y1 - y0) as c_uint);   // C 4980
    }
  else if (x1 - x0 > y1 - y0) && (y1 - y0 >= 3)                         // C 4982
    /* A horizontal divider, at least three pixels high: Draw first
       and last pixels differently.  */                                // C 4983-4984
    {
      android_set_foreground ((*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4986
			      color_first);                             // C 4987
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f),              // C 4988
			      (*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4989
			      x0, y0, (x1 - x0) as c_uint, 1 as c_uint);   // C 4990
      android_set_foreground ((*((*f).output_data.android as *mut android_output)).normal_gc, color);   // C 4991
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f),              // C 4992
			      (*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4993
			      x0, y0 + 1, (x1 - x0) as c_uint, (y1 - y0 - 2) as c_uint);   // C 4994
      android_set_foreground ((*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4995
			      color_last);                              // C 4996
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f),              // C 4997
			      (*((*f).output_data.android as *mut android_output)).normal_gc,   // C 4998
			      x0, y1 - 1, (x1 - x0) as c_uint, 1 as c_uint);   // C 4999
    }
  else
    {
      /* In any other case do not draw the first and last pixels
	 differently.  */                                              // C 5003-5004
      android_set_foreground ((*((*f).output_data.android as *mut android_output)).normal_gc, color);   // C 5005
      android_fill_rectangle (FRAME_ANDROID_DRAWABLE! (f),              // C 5006
			      (*((*f).output_data.android as *mut android_output)).normal_gc,   // C 5007
			      x0, y0, (x1 - x0) as c_uint, (y1 - y0) as c_uint);   // C 5008
    }
}

/* C 5012-5020: the diagnostic-pragma pair around the input-method group.  C 5014-5016's
   `#ifdef __clang__` arm and C 5018-5020's `#else` arm are text only - `#pragma clang
   diagnostic push` / `#pragma clang diagnostic ignored "-Wmissing-prototypes"` and the `#pragma
   GCC` twins - and the two matching pops are at the file's tail (C 7110-7116); Rust has no
   counterpart for either (the `docs/ports/excluded-macros.txt` class), so the text is carried
   here as C's own and the ledger records the pair.  */

/* Input method related functions.  Some of these are called from Java
   within the UI thread.  */                                          // C 5022-5023

/* A counter used to decide when an editing request completes.  */     // C 5025
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut edit_counter: c_ulong = 0;                                 // C 5026

/* The last counter known to have completed.  */                       // C 5028
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut last_edit_counter: c_ulong = 0;                            // C 5029

/* Semaphore posted every time the counter increases.  */               // C 5031
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut edit_sem: sem_t = sem_t { __opaque: [0; 16] };              // C 5032

/* Two declarations this group needs and no mirror carries: `<semaphore.h>` (C 24) is the NDK's
   own header (PORTING.md P-49), so the type and the one function this slice calls stand here -
   the `src/android.rs` 175-216 copy of bionic's `sem_t` (16 bytes, align 4) - and the two names
   C 5048/5065 use are the GCC atomics builtin, which PORTING.md §6's rule ports as the same
   object and the same memory order through `core::sync::atomic`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C, align(4))]
pub struct sem_t { pub __opaque: [c_uchar; 16] }                 // bionic, sizeof 16 align 4
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /* int sem_timedwait (sem_t *, const struct timespec *);  */
  pub fn sem_timedwait (sem: *mut sem_t, abs_timeout: *const timespec) -> c_int;
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
pub const __ATOMIC_SEQ_CST: core::sync::atomic::Ordering = core::sync::atomic::Ordering::SeqCst;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn __atomic_load_n (p: *const c_ulong, order: core::sync::atomic::Ordering) -> c_ulong
{
  (*(p as *const core::sync::atomic::AtomicU64)).load (order)         // C 5048-5049 / C 5065-5066
}

/* Try to synchronize with the UI thread, waiting a certain amount of
   time for outstanding editing requests to complete.

   Every time one of the text retrieval functions is called and an
   editing request is made, Emacs gives the main thread approximately
   100 ms to process it, in order to mostly keep the input method in
   sync with the buffer contents.  */                                 // C 5034-5040

/* C 5042-5072.  `static`; only called (C 5645/5661/5726).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_sync_edit ()
{
  let mut start: timespec;                                              // C 5045
  let mut end: timespec;
  let mut rem: timespec;
  let mut counter: c_ulong;                                             // C 5046

  counter = __atomic_load_n (&raw const last_edit_counter,             // C 5048
			     __ATOMIC_SEQ_CST);                        // C 5049

  if counter == edit_counter                                            // C 5051
    { return; }                                                         // C 5052

  start = current_timespec ();                                          // C 5054
  end = timespec_add (start, make_timespec (0, 100000000));             // C 5055

  while true                                                            // C 5057
    {
      rem = timespec_sub (end, current_timespec ());                    // C 5059

      /* Timeout.  */                                                   // C 5061
      if timespec_sign (rem) < 0                                        // C 5062
	{ break; }                                                      // C 5063

      if __atomic_load_n (&raw const last_edit_counter,                 // C 5065
			  __ATOMIC_SEQ_CST)                             // C 5066
	  == edit_counter                                               // C 5067
	{ break; }                                                      // C 5068

      sem_timedwait (&raw mut edit_sem, &raw const end);                 // C 5070
    }
}

/* Return a copy of the specified Java string and its length in
   *LENGTH.  Use the JNI environment ENV.  Value is NULL if copying
   the string fails.  */                                               // C 5074-5076

/* C 5078-5105.  `static`; called from the JNI group below.  C's `(*env)->Method (env, …)` is the
   port's `((*(*env)).Method.unwrap ()) (env, …)` - `JNIEnv` is the NDK mirror's
   `*const JNINativeInterface` (`include/jni.rs` 118) and the table's entries the
   `src/android.rs` 1683 device.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_copy_java_string (env: *mut JNIEnv, string: jstring, length: *mut size_t)
  -> *mut c_ushort
{
  let mut size: jsize;                                                  // C 5081
  let mut i: jsize;
  let java: *const jchar;                                               // C 5082
  let mut buffer: *mut c_ushort;                                        // C 5083

  size = ((*(*env)).GetStringLength.unwrap ()) (env, string);           // C 5085
  buffer = malloc (size as usize * std::mem::size_of::<c_ushort> ()) as *mut c_ushort;   // C 5086

  if buffer.is_null ()                                                  // C 5088
    { return std::ptr::null_mut (); }                                   // C 5089

  java = ((*(*env)).GetStringChars.unwrap ()) (env, string,             // C 5091
					       std::ptr::null_mut ());

  if java.is_null ()                                                    // C 5093
    {
      free (buffer as *mut c_void);                                     // C 5095
      return std::ptr::null_mut ();                                     // C 5096
    }

  i = 0;                                                                // C 5099
  while i < size
    {
      *buffer.offset (i as isize) = *java.offset (i as isize);          // C 5100
      i += 1;                                                           // C 5099 (the `for` step)
    }

  *length = size as size_t;                                             // C 5102
  ((*(*env)).ReleaseStringChars.unwrap ()) (env, string, java);         // C 5103
  return buffer;                                                        // C 5104
}

/* C 5107-5293: the seven uniform input-method entry points Java calls on the UI thread -
   `beginBatchEdit`, `endBatchEdit`, `commitCompletion`, `commitText`, `deleteSurroundingText`,
   `finishComposingText` and `replaceText`.  Every one of them is C's `JNIEXPORT void JNICALL
   NATIVE_NAME (name) (...)`; the paste is written at each definition site as this project's own
   class name, `Java_org_femacs_EmacsNative_<name>` (the `src/android.rs` 1655-1663 device,
   PORTING.md 0.1-19's name half / status 3.503), and C's `JNI_STACK_ALIGNMENT_PROLOGUE;` is the
   mirror's `JNI_STACK_ALIGNMENT_PROLOGUE! ()` (`include/android.rs` 211).  C's `union android_event
   event;` is `std::mem::zeroed ()` (PORTING.md 1.5, the C 1907 site in this unit), `++event_serial`
   and `++edit_counter` are the two-statement form (C's `event_serial`/`edit_counter` are the
   file-scope objects of the previous slice and `android.h` 254), and `event.ime.window`'s
   `android_window` is `jlong`'s width.  */

/* C 5107-5126: `NATIVE_NAME (beginBatchEdit)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_beginBatchEdit")]
pub unsafe extern "C" fn beginBatchEdit (env: *mut JNIEnv, object: jobject, window: jlong)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5110

  let mut event: android_event = std::mem::zeroed ();                   // C 5112 (PORTING.md 1.5)

  event.ime.r#type = ANDROID_INPUT_METHOD;                                // C 5114
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5115 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5116
  event.ime.operation = ANDROID_IME_START_BATCH_EDIT;                   // C 5117
  event.ime.start = 0;                                                  // C 5118
  event.ime.end = 0;                                                    // C 5119
  event.ime.length = 0;                                                 // C 5120
  event.ime.position = 0;                                               // C 5121
  event.ime.text = std::ptr::null_mut ();                               // C 5122
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5123 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5125
}

/* C 5128-5147: `NATIVE_NAME (endBatchEdit)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_endBatchEdit")]
pub unsafe extern "C" fn endBatchEdit (env: *mut JNIEnv, object: jobject, window: jlong)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5131

  let mut event: android_event = std::mem::zeroed ();                   // C 5133 (PORTING.md 1.5)

  event.ime.r#type = ANDROID_INPUT_METHOD;                                // C 5135
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5136 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5137
  event.ime.operation = ANDROID_IME_END_BATCH_EDIT;                     // C 5138
  event.ime.start = 0;                                                  // C 5139
  event.ime.end = 0;                                                    // C 5140
  event.ime.length = 0;                                                 // C 5141
  event.ime.position = 0;                                               // C 5142
  event.ime.text = std::ptr::null_mut ();                               // C 5143
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5144 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5146
}

/* C 5149-5181: `NATIVE_NAME (commitCompletion)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_commitCompletion")]
pub unsafe extern "C" fn commitCompletion (env: *mut JNIEnv, object: jobject, window: jlong,
					   completion_text: jstring, position: jint)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5153

  let mut event: android_event = std::mem::zeroed ();                   // C 5155 (PORTING.md 1.5)
  let mut text: *mut c_ushort;                                          // C 5156
  let mut length: size_t = 0;                                           // C 5157

  /* First, obtain a copy of the Java string.  */                        // C 5159
  text = android_copy_java_string (env, completion_text, &raw mut length);   // C 5160

  if text.is_null ()                                                    // C 5162
    { return; }                                                         // C 5163

  /* Next, populate the event.  Events will always eventually be
     delivered on Android, so handle_one_android_event can be relied
     on to free text.  */                                               // C 5165-5167

  event.ime.r#type = ANDROID_INPUT_METHOD;                                // C 5169
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5170 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5171
  event.ime.operation = ANDROID_IME_COMMIT_TEXT;                        // C 5172
  event.ime.start = 0;                                                  // C 5173
  event.ime.end = 0;                                                    // C 5174
  event.ime.length = min (length, PTRDIFF_MAX as size_t);               // C 5175
  event.ime.position = position as ptrdiff_t;                           // C 5176
  event.ime.text = text;                                                // C 5177
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5178 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5180
}

/* C 5183-5215: `NATIVE_NAME (commitText)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_commitText")]
pub unsafe extern "C" fn commitText (env: *mut JNIEnv, object: jobject, window: jlong,
				     commit_text: jstring, position: jint)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5187

  let mut event: android_event = std::mem::zeroed ();                   // C 5189 (PORTING.md 1.5)
  let mut text: *mut c_ushort;                                          // C 5190
  let mut length: size_t = 0;                                           // C 5191

  /* First, obtain a copy of the Java string.  */                        // C 5193
  text = android_copy_java_string (env, commit_text, &raw mut length);   // C 5194

  if text.is_null ()                                                    // C 5196
    { return; }                                                         // C 5197

  /* Next, populate the event.  Events will always eventually be
     delivered on Android, so handle_one_android_event can be relied
     on to free text.  */                                               // C 5199-5201

  event.ime.r#type = ANDROID_INPUT_METHOD;                                // C 5203
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5204 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5205
  event.ime.operation = ANDROID_IME_COMMIT_TEXT;                        // C 5206
  event.ime.start = 0;                                                  // C 5207
  event.ime.end = 0;                                                    // C 5208
  event.ime.length = min (length, PTRDIFF_MAX as size_t);               // C 5209
  event.ime.position = position as ptrdiff_t;                           // C 5210
  event.ime.text = text;                                                // C 5211
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5212 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5214
}

/* C 5217-5238: `NATIVE_NAME (deleteSurroundingText)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_deleteSurroundingText")]
pub unsafe extern "C" fn deleteSurroundingText (env: *mut JNIEnv, object: jobject,
						window: jlong, left_length: jint,
						right_length: jint)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5222

  let mut event: android_event = std::mem::zeroed ();                   // C 5224 (PORTING.md 1.5)

  event.ime.r#type = ANDROID_INPUT_METHOD;                                // C 5226
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5227 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5228
  event.ime.operation = ANDROID_IME_DELETE_SURROUNDING_TEXT;            // C 5229
  event.ime.start = left_length as ptrdiff_t;                           // C 5230
  event.ime.end = right_length as ptrdiff_t;                            // C 5231
  event.ime.length = 0;                                                 // C 5232
  event.ime.position = 0;                                               // C 5233
  event.ime.text = std::ptr::null_mut ();                               // C 5234
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5235 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5237
}

/* C 5240-5260: `NATIVE_NAME (finishComposingText)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_finishComposingText")]
pub unsafe extern "C" fn finishComposingText (env: *mut JNIEnv, object: jobject, window: jlong)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5244

  let mut event: android_event = std::mem::zeroed ();                   // C 5246 (PORTING.md 1.5)

  event.ime.r#type = ANDROID_INPUT_METHOD;                                // C 5248
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5249 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5250
  event.ime.operation = ANDROID_IME_FINISH_COMPOSING_TEXT;              // C 5251
  event.ime.start = 0;                                                  // C 5252
  event.ime.end = 0;                                                    // C 5253
  event.ime.length = 0;                                                 // C 5254
  event.ime.position = 0;                                               // C 5255
  event.ime.text = std::ptr::null_mut ();                               // C 5256
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5257 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5259
}

/* C 5262-5293: `NATIVE_NAME (replaceText)`.  C reuses the parameter name `text` for the copied
   buffer (`jobject text` at C 5264, `unsigned short *text` at C 5270), so the port shadows the
   parameter with the `let text: *mut c_ushort` below - one name, C's two roles.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_replaceText")]
pub unsafe extern "C" fn replaceText (env: *mut JNIEnv, object: jobject, window: jlong,
				      start: jint, end: jint, text: jobject,
				      new_cursor_position: c_int, attribute: jobject)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5267

  let mut event: android_event = std::mem::zeroed ();                   // C 5269 (PORTING.md 1.5)
  let mut length: size_t = 0;                                           // C 5270 (the `&raw mut` place)

  /* First, obtain a copy of the Java string.  */                        // C 5272
  let text: *mut c_ushort = android_copy_java_string (env, text, &raw mut length);   // C 5273

  if text.is_null ()                                                    // C 5275
    { return; }                                                         // C 5276

  /* Next, populate the event with the information in this function's
     arguments.  */                                                     // C 5278-5279

  event.ime.r#type = ANDROID_INPUT_METHOD;                                // C 5281
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5282 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5283
  event.ime.operation = ANDROID_IME_REPLACE_TEXT;                       // C 5284
  event.ime.start = start as ptrdiff_t + 1;                             // C 5285
  event.ime.end = end as ptrdiff_t + 1;                                 // C 5286
  event.ime.length = length;                                            // C 5287
  event.ime.position = new_cursor_position as ptrdiff_t;                // C 5288
  event.ime.text = text;                                                // C 5289
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5290 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5292
}

/* C 5295-5307: `struct android_conversion_query_context`, the record the text-conversion
   query callbacks carry.  Its layout follows from the mirror's measured
   `textconv_callback_struct` (`include/textconv.rs` 144-150: 48 bytes, align 8) with
   `#[repr(C)]`; the four numbers below are derived from that measurement and are recorded as
   derived in this unit's ledger (the C probe route for a unit-local record is
   `tools/gen-layout-fields.py`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_conversion_query_context
{
  /* The conversion request.  */
  pub query: textconv_callback_struct,                                 // C 5300

  /* The window the request is being made on.  */
  pub window: android_window,                                          // C 5303

  /* Whether or not the request was successful.  */
  pub success: bool,                                                   // C 5306
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::offset_of! (android_conversion_query_context, query) == 0);
  core::assert! (std::mem::offset_of! (android_conversion_query_context, window) == 48);
  core::assert! (std::mem::offset_of! (android_conversion_query_context, success) == 56);
  core::assert! (std::mem::size_of::<android_conversion_query_context> () == 64);
};

/* Obtain the text from the frame whose window is that specified in
   DATA using the text conversion query specified there.

   Set ((struct android_conversion_query_context *) DATA)->success on
   success.  */                                                       // C 5309-5313

/* C 5315-5335.  `static` and the *address is taken* (C 5449/5493 pass it to
   `android_run_in_emacs_thread`), so `unsafe extern "C" fn` - the `android_show_hourglass` shape
   of this unit.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_perform_conversion_query (data: *mut c_void)
{
  let mut context: *mut android_conversion_query_context;               // C 5318
  let mut f: *mut frame;                                                // C 5319

  context = data as *mut android_conversion_query_context;              // C 5321

  /* Find the frame associated with the window.  */                      // C 5323
  f = android_window_to_frame (std::ptr::null_mut (), (*context).window);   // C 5324

  if f.is_null ()                                                       // C 5326
    { return; }                                                         // C 5327

  textconv_query (f, &raw mut (*context).query, 0);                     // C 5329

  /* context->query.text will have been set even if textconv_query
     returns 1.  */                                                     // C 5331-5332

  (*context).success = true;                                            // C 5334
}

/* Convert a string in BUFFER, containing N characters in Emacs's
   internal multibyte encoding, to a Java string utilizing the
   specified JNI environment ENV.

   If N is equal to BYTES, then BUFFER holds unibyte or plain-ASCII
   characters.  Otherwise, BUFFER holds multibyte characters.

   Make sure N and BYTES are absolutely correct, or you are asking for
   trouble.

   Value is a jstring upon success, NULL otherwise.  Any exceptions
   generated are not cleared.  */                                      // C 5337-5348

/* C 5350-5426.  `static`; only called (C 5464/5508/6025/6081).  C's two
   `ckd_mul (&size, n, sizeof *utf16)` sites are the P-15 `ckd_mul`/`overflowing_mul` pair
   (docs/ports/token-aliases.tsv 13, the C 662 site of this unit), and C's two `while (n--)`
   loops keep their body with the decrement as the step (C's `n` is not read inside the body).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_text_to_string (env: *mut JNIEnv, mut buffer: *mut c_char,
				  mut n: ptrdiff_t, bytes: ptrdiff_t) -> jstring
{
  let mut utf16: *mut jchar;                                            // C 5354
  let mut size: size_t = 0;                                             // C 5355
  let mut index: size_t;
  let mut string: jstring;                                             // C 5356
  let mut encoded: c_int;                                               // C 5357

  if n == bytes                                                         // C 5359
    {
      /* This buffer holds no multibyte characters.  */                 // C 5361

      /* C 5363: `if (ckd_mul (&size, n, sizeof *utf16)) return NULL;` - the P-15
	 `ckd_mul`/`overflowing_mul` pair (docs/ports/token-aliases.tsv 13).  */
      let (size_, mul_overflow) = (n as size_t)
	.overflowing_mul (std::mem::size_of::<jchar> ());
      size = size_;
      if mul_overflow
	{ return std::ptr::null_mut (); }                              // C 5364

      utf16 = malloc (size) as *mut jchar;                              // C 5366
      index = 0;                                                        // C 5367

      if utf16.is_null ()                                              // C 5369
	{ return std::ptr::null_mut (); }                              // C 5370

      while n != 0                                                      // C 5372 (the `n--` test)
	{
	  *utf16.offset (index as isize) = *buffer.offset (index as isize) as jchar;   // C 5374
	  index += 1;                                                   // C 5375
	  n -= 1;                                                       // C 5372 (the `n--` step)
	}

      string = ((*(*env)).NewString.unwrap ()) (env, utf16, bytes as jsize);   // C 5378
      free (utf16 as *mut c_void);                                      // C 5379

      return string;                                                    // C 5381
    }

  /* Allocate enough to hold N characters.  */                          // C 5384

  /* C 5386: the same `ckd_mul` pair as C 5363.  */
  let (size_, mul_overflow) = (n as size_t)
    .overflowing_mul (std::mem::size_of::<jchar> ());
  size = size_;
  if mul_overflow
    { return std::ptr::null_mut (); }                                  // C 5387

  utf16 = malloc (size) as *mut jchar;                                 // C 5389
  index = 0;                                                           // C 5390

  if utf16.is_null ()                                                  // C 5392
    { return std::ptr::null_mut (); }                                  // C 5393

  while n != 0                                                         // C 5395 (the `n--` test)
    {
      eassert! (CHAR_HEAD_P (*buffer as c_int));                        // C 5397
      encoded = STRING_CHAR (buffer as *mut u8 as *const u8);           // C 5398

      /* Now establish how to save ENCODED into the string.
	 Emacs operates on multibyte characters, not UTF-16 characters
	 with surrogate pairs as Android does.

	 However, character positions in Java are represented as
	 character (rather than codepoint) indices into UTF-16
	 strings, meaning that text positions reported to Android can
	 become decoupled from their actual values if the text
	 returned incorporates characters that must be encoded as
	 surrogate pairs.

	 The hack used by Emacs is to simply replace each multibyte
	 character that doesn't fit in a jchar with the Unicode
	 replacement character.  */                                  // C 5400-5413

      if encoded >= 65536                                               // C 5415
	{ encoded = 0xfffd; }                                          // C 5416

      *utf16.offset (index as isize) = encoded as jchar;                // C 5418
      index += 1;                                                       // C 5418 (the `index++`)
      buffer = buffer.offset (BYTES_BY_CHAR_HEAD (*buffer as c_int) as isize);   // C 5419
      n -= 1;                                                           // C 5395 (the `n--` step)
    }

  /* Create the string.  */                                            // C 5422
  string = ((*(*env)).NewString.unwrap ()) (env, utf16, index as jsize);   // C 5423
  free (utf16 as *mut c_void);                                         // C 5424
  return string;                                                       // C 5425
}

/* C 5428-5571: four more input-method entry points - the two text retrievals
   (`getTextAfterCursor`, `getTextBeforeCursor`) and the two composing setters
   (`setComposingText`, `setComposingRegion`).  The retrievals are the first callers of this
   unit's `android_conversion_query_context` (slice 16) and of `android_run_in_emacs_thread`
   (C 5449's address-taken callback), and C 5482's `TYPE_MINIMUM (EMACS_INT)` is
   `lib/intprops.h`'s macro, which no mirror carries: the tree's precedent is a one-line
   unit-local macro (the `src/androidselect.rs` 115 and `src/dbusbind.rs` 782 copies).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! TYPE_MINIMUM { ($t:ty) => { <$t>::MIN }; }

/* C 5428-5470: `NATIVE_NAME (getTextAfterCursor)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_getTextAfterCursor")]
pub unsafe extern "C" fn getTextAfterCursor (env: *mut JNIEnv, object: jobject, window: jlong,
					     length: jint, flags: jint) -> jstring
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5432

  let mut context: android_conversion_query_context = std::mem::zeroed ();   // C 5434
  let mut string: jstring;                                              // C 5435

  /* First, set up the conversion query.  */                            // C 5437
  context.query.position = EMACS_INT_MAX;                               // C 5438
  context.query.direction = TEXTCONV_FORWARD_CHAR;                      // C 5439
  context.query.factor = min (length, 65535) as c_ushort;               // C 5440
  context.query.operation = TEXTCONV_RETRIEVAL;                         // C 5441

  /* Next, set the rest of the context.  */                             // C 5443
  context.window = window as android_window;                            // C 5444
  context.success = false;                                              // C 5445

  /* Now try to perform the query.  */                                  // C 5447
  android_sync_edit ();                                                 // C 5448
  if android_run_in_emacs_thread (Some (android_perform_conversion_query),   // C 5449
				  &raw mut context as *mut c_void) != 0   // C 5450
    { return std::ptr::null_mut (); }                                   // C 5451

  if ! context.success                                                 // C 5453
    { return std::ptr::null_mut (); }                                   // C 5454

  /* context->query.text now contains the text in Emacs's internal
     UTF-8 based encoding.

     Convert it to Java's UTF-16 encoding, which is the same as
     UTF-16, except that NULL bytes are encoded as surrogate pairs.

     This assumes that `free' can free data allocated with xmalloc.  */   // C 5456-5462

  string = android_text_to_string (env, context.query.text.text,        // C 5464
				   context.query.text.length,           // C 5465
				   context.query.text.bytes);           // C 5466
  free (context.query.text.text as *mut c_void);                        // C 5467

  return string;                                                        // C 5469
}

/* C 5472-5514: `NATIVE_NAME (getTextBeforeCursor)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_getTextBeforeCursor")]
pub unsafe extern "C" fn getTextBeforeCursor (env: *mut JNIEnv, object: jobject, window: jlong,
					      length: jint, flags: jint) -> jstring
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5476

  let mut context: android_conversion_query_context = std::mem::zeroed ();   // C 5478
  let mut string: jstring;                                              // C 5479

  /* First, set up the conversion query.  */                            // C 5481
  context.query.position = TYPE_MINIMUM! (EMACS_INT);                   // C 5482
  context.query.direction = TEXTCONV_BACKWARD_CHAR;                     // C 5483
  context.query.factor = min (length, 65535) as c_ushort;               // C 5484
  context.query.operation = TEXTCONV_RETRIEVAL;                         // C 5485

  /* Next, set the rest of the context.  */                             // C 5487
  context.window = window as android_window;                            // C 5488
  context.success = false;                                              // C 5489

  /* Now try to perform the query.  */                                  // C 5491
  android_sync_edit ();                                                 // C 5492
  if android_run_in_emacs_thread (Some (android_perform_conversion_query),   // C 5493
				  &raw mut context as *mut c_void) != 0   // C 5494
    { return std::ptr::null_mut (); }                                   // C 5495

  if ! context.success                                                 // C 5497
    { return std::ptr::null_mut (); }                                   // C 5498

  /* context->query.text now contains the text in Emacs's internal
     UTF-8 based encoding.

     Convert it to Java's UTF-16 encoding, which is the same as
     UTF-16, except that NULL bytes are encoded as surrogate pairs.

     This assumes that `free' can free data allocated with xmalloc.  */   // C 5500-5506

  string = android_text_to_string (env, context.query.text.text,        // C 5508
				   context.query.text.length,           // C 5509
				   context.query.text.bytes);           // C 5510
  free (context.query.text.text as *mut c_void);                        // C 5511

  return string;                                                        // C 5513
}

/* C 5516-5549: `NATIVE_NAME (setComposingText)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_setComposingText")]
pub unsafe extern "C" fn setComposingText (env: *mut JNIEnv, object: jobject, window: jlong,
					   composing_text: jstring, new_cursor_position: jint)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5521

  let mut event: android_event = std::mem::zeroed ();                   // C 5523 (PORTING.md 1.5)
  let mut text: *mut c_ushort;                                          // C 5524
  let mut length: size_t = 0;                                           // C 5525 (the `&raw mut` place)

  /* First, obtain a copy of the Java string.  */                        // C 5527
  text = android_copy_java_string (env, composing_text, &raw mut length);   // C 5528

  if text.is_null ()                                                    // C 5530
    { return; }                                                         // C 5531

  /* Next, populate the event.  Events will always eventually be
     delivered on Android, so handle_one_android_event can be relied
     on to free text.  */                                               // C 5533-5535

  event.ime.r#type = ANDROID_INPUT_METHOD;                              // C 5537
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5538 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5539
  event.ime.operation = ANDROID_IME_SET_COMPOSING_TEXT;                 // C 5540
  event.ime.start = 0;                                                  // C 5541
  event.ime.end = 0;                                                    // C 5542
  event.ime.length = min (length, PTRDIFF_MAX as size_t);               // C 5543
  event.ime.position = new_cursor_position as ptrdiff_t;                // C 5544
  event.ime.text = text;                                                // C 5545
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5546 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5548
}

/* C 5551-5571: `NATIVE_NAME (setComposingRegion)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_setComposingRegion")]
pub unsafe extern "C" fn setComposingRegion (env: *mut JNIEnv, object: jobject, window: jlong,
					     start: jint, end: jint)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5555

  let mut event: android_event = std::mem::zeroed ();                   // C 5557 (PORTING.md 1.5)

  event.ime.r#type = ANDROID_INPUT_METHOD;                              // C 5559
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5560 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5561
  event.ime.operation = ANDROID_IME_SET_COMPOSING_REGION;               // C 5562
  event.ime.start = start as ptrdiff_t + 1;                             // C 5563
  event.ime.end = end as ptrdiff_t + 1;                                 // C 5564
  event.ime.length = 0;                                                 // C 5565
  event.ime.position = 0;                                               // C 5566
  event.ime.text = std::ptr::null_mut ();                               // C 5567
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5568 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5570
}

/* One field this text needs and the mirror still owes: C window.h 296-302's
   `ptrdiff_t ephemeral_last_point;` (`#ifdef HAVE_TEXT_CONVERSION`).  `include/window.rs` carries
   `last_point` (its 134) and records `ephemeral_last_point` as an owed field (its 11), so the port
   names it at its one use site as the `ptrdiff_t` that follows `last_point` in C's record - the
   `src/textconv.rs` 335-347 device, whose Evidence 2 measured the two adjacent members at offsets
   352 and 360 for that unit's window record.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! ephemeral_last_point {
  ($w:expr) => { *(&raw mut (*$w).last_point).add (1) };
}

/* C 5573-5739: five more input-method/selection entry points - `setSelection`, the
   `getSelection` group (`struct android_get_selection_context`, its worker
   `android_get_selection` and the JNI wrapper) and `performEditorAction`.  C 5630's
   `w->ephemeral_last_point`/`w->last_mark`, C 5643-5645's `BVAR (b, mark_active)`/`BVAR (b, mark)`
   and C 5645's `marker_position` are the mirrors' (the buffer fields are spelled with the
   trailing underscore the mirror uses: `mark_active_`/`mark_`), C 5634's `window_point` lives in
   `include/window.rs`'s `window_fn_decls` module, and C 5600-5608's record gets the same derived
   asserts as slice 16's (the `android_window` member, two `ptrdiff_t`s - Owed 30's class).  */

/* C 5573-5596: `NATIVE_NAME (setSelection)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_setSelection")]
pub unsafe extern "C" fn setSelection (env: *mut JNIEnv, object: jobject, window: jlong,
				       start: jint, end: jint)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5577

  let mut event: android_event = std::mem::zeroed ();                   // C 5579 (PORTING.md 1.5)

  /* While IMEs want access to the entire selection, Emacs only
     supports setting the point.  */                                   // C 5581-5582

  event.ime.r#type = ANDROID_INPUT_METHOD;                              // C 5584
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5585 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5586
  event.ime.operation = ANDROID_IME_SET_POINT;                          // C 5587
  event.ime.start = start as ptrdiff_t + 1;                             // C 5588
  event.ime.end = end as ptrdiff_t + 1;                                 // C 5589
  event.ime.length = 0;                                                 // C 5590
  event.ime.position = start as ptrdiff_t;                              // C 5591
  event.ime.text = std::ptr::null_mut ();                               // C 5592
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5593 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5595
}

/* C 5598-5608: `struct android_get_selection_context`, the record the selection worker
   carries.  The three numbers are derived from the mirror's `android_window` (`isize`) and two
   `ptrdiff_t` members with `#[repr(C)]` - Owed 30's class.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_get_selection_context
{
  /* The window in question.  */
  pub window: android_window,                                          // C 5603

  /* The position of the window's point when it was last
     redisplayed, and its last mark if active.  */
  pub point: ptrdiff_t,                                                // C 5607
  pub mark: ptrdiff_t,
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::offset_of! (android_get_selection_context, window) == 0);
  core::assert! (std::mem::offset_of! (android_get_selection_context, point) == 8);
  core::assert! (std::mem::offset_of! (android_get_selection_context, mark) == 16);
  core::assert! (std::mem::size_of::<android_get_selection_context> () == 24);
};

/* Function run on the main thread by `getSelection'.
   Place the character position of point in PT.  */                     // C 5610-5611

/* C 5613-5649.  `static` and the *address is taken* (C 5663), so `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_get_selection (data: *mut c_void)
{
  let mut context: *mut android_get_selection_context;                  // C 5616
  let mut f: *mut frame;                                                // C 5617
  let mut w: *mut window;                                               // C 5618
  let mut b: *mut buffer;                                               // C 5619

  context = data as *mut android_get_selection_context;                 // C 5621

  /* Look up the associated frame and its selected window.  */           // C 5623
  f = android_window_to_frame (std::ptr::null_mut (), (*context).window);   // C 5624

  if f.is_null ()                                                       // C 5626
    { (*context).point = -1; }                                          // C 5627
  else
    {
      w = XWINDOW ((*f).selected_window);                               // C 5630

      /* Return W's point as it is now.  Then, set
	 W->ephemeral_last_point to match the current point.  */        // C 5632-5633
      (*context).point = window_fn_decls::window_point (w);             // C 5634
      ephemeral_last_point! (w) = (*context).point;                     // C 5635

      /* Default context->mark to w->last_point too.  */                // C 5637
      (*context).mark = (*context).point;                               // C 5638

      /* If the mark is active, then set it properly.  Also, adjust
	 w->last_mark to match.  */                                     // C 5640-5641
      b = XBUFFER ((*w).contents);                                      // C 5642
      if ! NILP (BVAR! (b, mark_active_))                               // C 5643
	{
	  (*context).mark = marker_position (BVAR! (b, mark_));           // C 5645
	  (*w).last_mark = (*context).mark;                               // C 5646
	}
    }
}

/* C 5651-5687: `NATIVE_NAME (getSelection)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_getSelection")]
pub unsafe extern "C" fn getSelection (env: *mut JNIEnv, object: jobject, window: jlong)
  -> jintArray
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5654

  let mut context: android_get_selection_context = std::mem::zeroed ();   // C 5656
  let mut array: jintArray;                                             // C 5657
  let mut contents: [jint; 2] = [0; 2];                                 // C 5658

  context.window = window as android_window;                            // C 5660

  android_sync_edit ();                                                 // C 5662
  if android_run_in_emacs_thread (Some (android_get_selection),         // C 5663
				  &raw mut context as *mut c_void) != 0   // C 5664
    { return std::ptr::null_mut (); }                                   // C 5665

  if context.point == -1                                                // C 5667
    { return std::ptr::null_mut (); }                                   // C 5668

  /* Wraparound actually makes more sense than truncation; at least
     editing will sort of work.  Convert the positions to start from
     index 0, as that is what Android expects.  */                      // C 5670-5672
  contents[0] = (min (context.point, context.mark) as c_uint).wrapping_sub (1) as jint;   // C 5673-5674
  contents[1] = (max (context.point, context.mark) as c_uint).wrapping_sub (1) as jint;   // C 5675-5676

  /* Now create the array.  */                                          // C 5678
  array = ((*(*env)).NewIntArray.unwrap ()) (env, 2);                   // C 5679

  if array.is_null ()                                                   // C 5681
    { return std::ptr::null_mut (); }                                   // C 5682

  /* Set its contents.  */                                              // C 5684
  ((*(*env)).SetIntArrayRegion.unwrap ()) (env, array, 0, 2,            // C 5685
					   contents.as_ptr ());
  return array;                                                         // C 5686
}

/* C 5689-5739: `NATIVE_NAME (performEditorAction)`.  C's unused `int action` parameter is kept
   (C 5691 never reads it).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_performEditorAction")]
pub unsafe extern "C" fn performEditorAction (env: *mut JNIEnv, object: jobject, window: jlong,
					      action: c_int)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5693

  let mut event: android_event = std::mem::zeroed ();                   // C 5695 (PORTING.md 1.5)

  /* It's a good idea to call `android_sync_edit' before sending the
     key event.  Otherwise, if RET causes the current window to be
     changed, any text previously committed might end up in the newly
     selected window.  */                                               // C 5697-5700

  android_sync_edit ();                                                 // C 5702

  /* Undocumented behavior: performEditorAction is apparently expected
     to finish composing any text.  */                                  // C 5704-5705

  event.ime.r#type = ANDROID_INPUT_METHOD;                              // C 5707
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 5708 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 5709
  event.ime.operation = ANDROID_IME_FINISH_COMPOSING_TEXT;              // C 5710
  event.ime.start = 0;                                                  // C 5711
  event.ime.end = 0;                                                    // C 5712

  /* This value of `length' means that the input method should receive
     an update containing the new conversion region.  */                // C 5714-5715

  event.ime.length = 1;                                                 // C 5717
  event.ime.position = 0;                                               // C 5718
  event.ime.text = std::ptr::null_mut ();                               // C 5719
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 5720 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5722

  /* Finally, send the return key press.  `counter' is set; this means
     that a text conversion barrier will be generated once the event
     is read, which will cause subsequent edits to wait until the
     edits associated with this key press complete.  */                 // C 5724-5727

  event.xkey.r#type = ANDROID_KEY_PRESS;                                // C 5729
  event_serial += 1;
  event.xkey.serial = event_serial as c_ulong;                          // C 5730 (the `++event_serial`)
  event.xkey.window = window as android_window;                         // C 5731
  event.xkey.time = 0;                                                  // C 5732
  event.xkey.state = 0;                                                 // C 5733
  event.xkey.keycode = 66;                                              // C 5734
  event.xkey.unicode_char = 0;                                          // C 5735
  edit_counter += 1;
  event.xkey.counter = edit_counter;                                    // C 5736 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5738
}

/* C 5741-5893: `performContextMenuAction`, the extracted-text request context and its worker
   (`android_get_extracted_text`), plus the two JNI class records and their file-scope objects
   (`request_class`, `text_class` - the last two `missing file-scope object` rows of this file's
   checklist).  C 5741's `switch (action)` keeps its six labels; C 5770's `default: return;` is
   the `_` arm, moved last because Rust rejects arms after `_` (recorded in the ledger), and the
   three records' layout numbers are derived (Owed 30's class) - the probe checks them.  */

/* C 5741-5796: `NATIVE_NAME (performContextMenuAction)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_performContextMenuAction")]
pub unsafe extern "C" fn performContextMenuAction (env: *mut JNIEnv, object: jobject,
						   window: jlong, action: c_int)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5745

  let mut event: android_event = std::mem::zeroed ();                   // C 5747 (PORTING.md 1.5)
  let mut key: c_int;                                                   // C 5748

  /* Note that ACTION is determined in EmacsInputConnection, and as
     such they are not actual resource IDs.  */                        // C 5750-5751

  match action                                                          // C 5753
    {
      /* The subsequent three keycodes are addressed by
	 android_get_keysym_name rather than in keyboard.c.  */         // C 5755-5756

      0 => /* android.R.id.selectAll */                                 // C 5758
	{ key = 65536 + 1; }                                            // C 5759

      1 => /* android.R.id.startSelectingText */                        // C 5762
	{ key = 65536 + 2; }                                            // C 5763

      2 => /* android.R.id.stopSelectingText */                         // C 5766
	{ key = 65536 + 3; }                                            // C 5767

      3 => /* android.R.id.cut */                                       // C 5773
	{ key = 277; }                                                  // C 5774

      4 => /* android.R.id.copy */                                      // C 5777
	{ key = 278; }                                                  // C 5778

      5 => /* android.R.id.paste */                                     // C 5781
	{ key = 279; }                                                  // C 5782

      _ =>                                                              // C 5770 (C's `default:`)
	{ return; }                                                     // C 5771
    }

  event.xkey.r#type = ANDROID_KEY_PRESS;                                // C 5786
  event_serial += 1;
  event.xkey.serial = event_serial as c_ulong;                          // C 5787 (the `++event_serial`)
  event.xkey.window = window as android_window;                         // C 5788
  event.xkey.time = 0;                                                  // C 5789
  event.xkey.state = 0;                                                 // C 5790
  event.xkey.keycode = key as c_uint;                                   // C 5791
  event.xkey.unicode_char = 0;                                          // C 5792
  edit_counter += 1;
  event.xkey.counter = edit_counter;                                    // C 5793 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 5795
}

/* Text extraction.  */                                                 // C 5800

/* C 5802-5827: `struct android_get_extracted_text_context`, the extracted-text request.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_get_extracted_text_context
{
  /* The parameters of the request.  */
  pub hint_max_chars: c_int,                                            // C 5805

  /* Token for the request.  */
  pub token: c_int,                                                     // C 5808

  /* Flags associated with the request.  */
  pub flags: c_int,                                                     // C 5811

  /* The returned text, or NULL.  */
  pub text: *mut c_char,                                                // C 5814

  /* The size of that text in characters and bytes.  */
  pub length: ptrdiff_t,                                                // C 5817
  pub bytes: ptrdiff_t,

  /* Offsets into that text.  */
  pub start: ptrdiff_t,                                                 // C 5820
  pub start_offset: ptrdiff_t,
  pub end_offset: ptrdiff_t,

  /* The window.  */
  pub window: android_window,                                           // C 5823

  /* Whether or not the mark is active.  */
  pub mark_active: bool,                                                // C 5826
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, hint_max_chars) == 0);
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, token) == 4);
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, flags) == 8);
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, text) == 16);
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, length) == 24);
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, bytes) == 32);
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, start) == 40);
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, start_offset) == 48);
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, end_offset) == 56);
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, window) == 64);
  core::assert! (std::mem::offset_of! (android_get_extracted_text_context, mark_active) == 72);
  core::assert! (std::mem::size_of::<android_get_extracted_text_context> () == 80);
};

/* Return the extracted text in the extracted text context specified
   by DATA.  Save its flags and token into its frame's state.  */      // C 5829-5830

/* C 5832-5860.  `static` and the *address is taken* (C 6017), so `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_get_extracted_text (data: *mut c_void)
{
  let mut request: *mut android_get_extracted_text_context;             // C 5835
  let mut f: *mut frame;                                                // C 5836

  request = data as *mut android_get_extracted_text_context;            // C 5838

  /* Find the frame associated with the window.  */                      // C 5840
  f = android_window_to_frame (std::ptr::null_mut (), (*request).window);   // C 5841

  if f.is_null ()                                                       // C 5843
    { return; }                                                         // C 5844

  /* Now get the extracted text.  */                                    // C 5846
  (*request).text
    = get_extracted_text (f, min ((*request).hint_max_chars, 600) as ptrdiff_t,   // C 5848
			  &raw mut (*request).start, &raw mut (*request).start_offset,   // C 5849
			  &raw mut (*request).end_offset, &raw mut (*request).length,   // C 5850
			  &raw mut (*request).bytes, &raw mut (*request).mark_active);   // C 5851

  /* See if request->flags & GET_EXTRACTED_TEXT_MONITOR.  If so, then
     the input method has asked to monitor changes to the extracted
     text until the next IM context reset.  */                          // C 5853-5855

  (*(FRAME_ANDROID_OUTPUT! (f) as *mut android_output)).extracted_text_flags = (*request).flags;   // C 5857
  (*(FRAME_ANDROID_OUTPUT! (f) as *mut android_output)).extracted_text_token = (*request).token;   // C 5858
  (*(FRAME_ANDROID_OUTPUT! (f) as *mut android_output)).extracted_text_hint = (*request).hint_max_chars;   // C 5859
}

/* Structure describing the `ExtractedTextRequest' class.
   Valid only on the UI thread.  */                                   // C 5862-5863

/* C 5865-5870: `struct android_extracted_text_request_class`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_extracted_text_request_class
{
  pub initialized: bool,                                                // C 5867
  pub hint_max_chars: jfieldID,                                         // C 5868
  pub token: jfieldID,                                                  // C 5869
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::offset_of! (android_extracted_text_request_class, initialized) == 0);
  core::assert! (std::mem::offset_of! (android_extracted_text_request_class, hint_max_chars) == 8);
  core::assert! (std::mem::offset_of! (android_extracted_text_request_class, token) == 16);
  core::assert! (std::mem::size_of::<android_extracted_text_request_class> () == 24);
};

/* Structure describing the `ExtractedText' class.
   Valid only on the UI thread.  */                                   // C 5872-5873

/* C 5875-5886: `struct android_extracted_text_class`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_extracted_text_class
{
  pub class: jclass,                                                    // C 5877
  pub constructor: jmethodID,                                           // C 5878
  pub flags: jfieldID,                                                  // C 5879
  pub partial_start_offset: jfieldID,                                   // C 5880
  pub partial_end_offset: jfieldID,                                     // C 5881
  pub selection_start: jfieldID,                                        // C 5882
  pub selection_end: jfieldID,                                          // C 5883
  pub start_offset: jfieldID,                                           // C 5884
  pub text: jfieldID,                                                   // C 5885
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::offset_of! (android_extracted_text_class, class) == 0);
  core::assert! (std::mem::offset_of! (android_extracted_text_class, constructor) == 8);
  core::assert! (std::mem::offset_of! (android_extracted_text_class, flags) == 16);
  core::assert! (std::mem::offset_of! (android_extracted_text_class, partial_start_offset) == 24);
  core::assert! (std::mem::offset_of! (android_extracted_text_class, partial_end_offset) == 32);
  core::assert! (std::mem::offset_of! (android_extracted_text_class, selection_start) == 40);
  core::assert! (std::mem::offset_of! (android_extracted_text_class, selection_end) == 48);
  core::assert! (std::mem::offset_of! (android_extracted_text_class, start_offset) == 56);
  core::assert! (std::mem::offset_of! (android_extracted_text_class, text) == 64);
  core::assert! (std::mem::size_of::<android_extracted_text_class> () == 72);
};

/* Fields and methods associated with the `ExtractedTextRequest'
   class.  */                                                          // C 5888-5889
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut request_class: android_extracted_text_request_class =        // C 5890
  android_extracted_text_request_class
  {
    initialized: false,
    hint_max_chars: std::ptr::null_mut (),
    token: std::ptr::null_mut (),
  };

/* Fields and methods associated with the `ExtractedText' class.  */    // C 5892
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut text_class: android_extracted_text_class =                  // C 5893
  android_extracted_text_class
  {
    class: std::ptr::null_mut (),
    constructor: std::ptr::null_mut (),
    flags: std::ptr::null_mut (),
    partial_start_offset: std::ptr::null_mut (),
    partial_end_offset: std::ptr::null_mut (),
    selection_start: std::ptr::null_mut (),
    selection_end: std::ptr::null_mut (),
    start_offset: std::ptr::null_mut (),
    text: std::ptr::null_mut (),
  };

/* C 5895-6054: the extracted-text builder and the `getExtractedText` entry point.  C's
   `TYPE_MAXIMUM (jint)` (C 5934/5936/5941 and 6044/6046/6051) is `lib/intprops.h`'s macro and no
   mirror carries it, so the unit names it beside slice 17's `TYPE_MINIMUM!` (the
   `src/androidmenu.rs` 335 / `src/androidselect.rs` 113 copies); the two Java string literals C
   writes as adjacent pieces (C 5963-5964 and 5981-5982) are one literal each here, since C's
   concatenation happens before the compiler sees a string.  */

/* <intprops.h> 68-69: `# define TYPE_MAXIMUM(t) ...` (the `TYPE_MINIMUM` twin of slice 17).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! TYPE_MAXIMUM { ($t:ty) => { <$t>::MAX }; }

/* Return an ExtractedText object corresponding to the extracted text
   TEXT.  START is a character position describing the offset of the
   first character in TEXT.  START_OFFSET is the offset of the lesser
   of point or mark relative to START, and END_OFFSET is that of the
   greater of point or mark relative to START.  MARK_ACTIVE specifies
   whether or not the mark is currently active.

   Assume that request_class and text_class have already been
   initialized.

   Value is NULL if an error occurs; the exception is not cleared,
   else a local reference to the ExtractedText object.  */            // C 5895-5906

/* C 5908-5944.  `static`; called from C 6514.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_build_extracted_text (text: jstring, start: ptrdiff_t,
					start_offset: ptrdiff_t,
					end_offset: ptrdiff_t, mark_active: bool) -> jobject
{
  let env: *mut JNIEnv;                                                 // C 5913
  let mut object: jobject;                                              // C 5914

  env = android_java_env;                                               // C 5916

  /* Return NULL if the class has not yet been obtained.  */             // C 5918
  if text_class.class.is_null ()                                        // C 5919
    { return std::ptr::null_mut (); }                                   // C 5920

  /* Create an ExtractedText object containing this information.  */     // C 5922
  object = ((*(*env)).NewObject.unwrap ()) (env, text_class.class,      // C 5923
					    text_class.constructor);    // C 5924
  if object.is_null ()                                                  // C 5925
    { return std::ptr::null_mut (); }                                   // C 5926

  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.flags,     // C 5928
				     /* ExtractedText.FLAG_SELECTING */
				     if mark_active { 2 } else { 0 });  // C 5930

  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.partial_start_offset, -1);   // C 5931
  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.partial_end_offset, -1);   // C 5932
  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.selection_start,   // C 5933
				     min (start_offset, TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint);   // C 5934
  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.selection_end,   // C 5935
				     min (end_offset, TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint);   // C 5936

  /* Subtract 1 from start: point indices in Emacs start from 1, but
     Android expects 0.  */                                            // C 5938-5939
  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.start_offset,   // C 5940
				     min (start - 1, TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint);   // C 5941
  ((*(*env)).SetObjectField.unwrap ()) (env, object, text_class.text, text);   // C 5942
  return object;                                                        // C 5943
}

/* C 5946-6054: `NATIVE_NAME (getExtractedText)`.  C's `jobject ignored_object` parameter is
   kept - C 5947 never reads it.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_getExtractedText")]
pub unsafe extern "C" fn getExtractedText (env: *mut JNIEnv, ignored_object: jobject,
					   window: jlong, request: jobject,
					   flags: jint) -> jobject
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 5951

  let mut context: android_get_extracted_text_context = std::mem::zeroed ();   // C 5953
  let mut string: jstring;                                              // C 5954
  let mut class: jclass;                                                // C 5955
  let mut object: jobject;                                              // C 5956

  /* Initialize both classes if necessary.  */                           // C 5958

  if ! request_class.initialized                                         // C 5960
    {
      /* C 5963-5964's adjacent literals are one literal here: Rust has no compile-time
	 concatenation, and C's happens before the compiler sees a string.  */
      class
	= ((*(*env)).FindClass.unwrap ()) (env, c"android/view/inputmethod/ExtractedTextRequest".as_ptr ()) as jclass;   // C 5963-5964
      eassert! (! class.is_null ());                                      // C 5965

      request_class.hint_max_chars
	= ((*(*env)).GetFieldID.unwrap ()) (env, class, c"hintMaxChars".as_ptr (),   // C 5968
					    c"I".as_ptr ());
      eassert! (! request_class.hint_max_chars.is_null ());               // C 5969

      request_class.token
	= ((*(*env)).GetFieldID.unwrap ()) (env, class, c"token".as_ptr (),   // C 5972
					    c"I".as_ptr ());
      eassert! (! request_class.token.is_null ());                        // C 5973

      request_class.initialized = true;                                   // C 5975
    }

  if text_class.class.is_null ()                                        // C 5978
    {
      text_class.class
	= ((*(*env)).FindClass.unwrap ()) (env, c"android/view/inputmethod/ExtractedText".as_ptr ()) as jclass;   // C 5981-5982
      eassert! (! text_class.class.is_null ());                           // C 5983

      text_class.class
	= ((*(*env)).NewGlobalRef.unwrap ()) (env, text_class.class);     // C 5987
      class = text_class.class;                                           // C 5986
      eassert! (! text_class.class.is_null ());                           // C 5988

      text_class.flags
	= ((*(*env)).GetFieldID.unwrap ()) (env, class, c"flags".as_ptr (),   // C 5991
					    c"I".as_ptr ());
      text_class.partial_start_offset
	= ((*(*env)).GetFieldID.unwrap ()) (env, class, c"partialStartOffset".as_ptr (),   // C 5993
					    c"I".as_ptr ());
      text_class.partial_end_offset
	= ((*(*env)).GetFieldID.unwrap ()) (env, class, c"partialEndOffset".as_ptr (),   // C 5995
					    c"I".as_ptr ());
      text_class.selection_start
	= ((*(*env)).GetFieldID.unwrap ()) (env, class, c"selectionStart".as_ptr (),   // C 5997
					    c"I".as_ptr ());
      text_class.selection_end
	= ((*(*env)).GetFieldID.unwrap ()) (env, class, c"selectionEnd".as_ptr (),   // C 5999
					    c"I".as_ptr ());
      text_class.start_offset
	= ((*(*env)).GetFieldID.unwrap ()) (env, class, c"startOffset".as_ptr (),   // C 6001
					    c"I".as_ptr ());
      text_class.text
	= ((*(*env)).GetFieldID.unwrap ()) (env, class, c"text".as_ptr (),   // C 6003
					    c"Ljava/lang/CharSequence;".as_ptr ());
      text_class.constructor
	= ((*(*env)).GetMethodID.unwrap ()) (env, class, c"<init>".as_ptr (),   // C 6005
					     c"()V".as_ptr ());
    }

  context.hint_max_chars
    = ((*(*env)).GetIntField.unwrap ()) (env, request, request_class.hint_max_chars);   // C 6009
  context.token
    = ((*(*env)).GetIntField.unwrap ()) (env, request, request_class.token);   // C 6011
  context.flags = flags;                                                // C 6012
  context.text = std::ptr::null_mut ();                                 // C 6013
  context.window = window as android_window;                            // C 6014

  android_sync_edit ();                                                 // C 6016
  if android_run_in_emacs_thread (Some (android_get_extracted_text),    // C 6017
				  &raw mut context as *mut c_void) != 0   // C 6018
    { return std::ptr::null_mut (); }                                   // C 6019

  if context.text.is_null ()                                            // C 6021
    { return std::ptr::null_mut (); }                                   // C 6022

  /* Encode the returned text.  */                                      // C 6024
  string = android_text_to_string (env, context.text, context.length,   // C 6025
				   context.bytes);                      // C 6026
  free (context.text as *mut c_void);                                   // C 6027

  if string.is_null ()                                                  // C 6029
    { return std::ptr::null_mut (); }                                   // C 6030

  /* Create an ExtractedText object containing this information.  */     // C 6032
  object = ((*(*env)).NewObject.unwrap ()) (env, text_class.class,      // C 6033
					    text_class.constructor);    // C 6034
  if object.is_null ()                                                  // C 6035
    { return std::ptr::null_mut (); }                                   // C 6036

  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.flags,     // C 6038
				     /* ExtractedText.FLAG_SELECTING */
				     if context.mark_active { 2 } else { 0 });   // C 6040

  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.partial_start_offset, -1);   // C 6041
  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.partial_end_offset, -1);   // C 6042
  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.selection_start,   // C 6043
				     min (context.start_offset, TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint);   // C 6044
  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.selection_end,   // C 6045
				     min (context.end_offset, TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint);   // C 6046

  /* Subtract 1 from start: point indices in Emacs start from 1, but
     Android expects 0.  */                                            // C 6048-6049
  ((*(*env)).SetIntField.unwrap ()) (env, object, text_class.start_offset,   // C 6050
				     min (context.start - 1, TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint);   // C 6051
  ((*(*env)).SetObjectField.unwrap ()) (env, object, text_class.text, string);   // C 6052
  return object;                                                        // C 6053
}

/* C 6056-6163: four more input-method entry points - `getSelectedText`, the two request
   entries (`requestSelectionUpdate`, `requestCursorUpdates`) and `clearInputFlags`.  The first
   reuses slice 19's extracted-text context and slice 16's converter, and the three event
   entries are the uniform build-then-`android_write_event` shape of slices 15/17 (with C 6124's
   `mode` narrowing to the record's `size_t` length and C 6130's plain `0` counter).  */

/* C 6058-6086: `NATIVE_NAME (getSelectedText)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_getSelectedText")]
pub unsafe extern "C" fn getSelectedText (env: *mut JNIEnv, object: jobject, window: jlong)
  -> jstring
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 6062

  let mut context: android_get_extracted_text_context = std::mem::zeroed ();   // C 6064
  let mut string: jstring;                                              // C 6065

  context.hint_max_chars = -1;                                          // C 6067
  context.token = 0;                                                    // C 6068
  context.text = std::ptr::null_mut ();                                 // C 6069
  context.window = window as android_window;                            // C 6070

  android_sync_edit ();                                                 // C 6072
  if android_run_in_emacs_thread (Some (android_get_extracted_text),    // C 6073
				  &raw mut context as *mut c_void) != 0   // C 6074
    { return std::ptr::null_mut (); }                                   // C 6075

  if context.text.is_null ()                                            // C 6077
    { return std::ptr::null_mut (); }                                   // C 6078

  /* Encode the returned text.  */                                      // C 6080
  string = android_text_to_string (env, context.text, context.length,   // C 6081
				   context.bytes);                      // C 6082
  free (context.text as *mut c_void);                                   // C 6083

  return string;                                                        // C 6085
}

/* C 6088-6108: `NATIVE_NAME (requestSelectionUpdate)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_requestSelectionUpdate")]
pub unsafe extern "C" fn requestSelectionUpdate (env: *mut JNIEnv, object: jobject, window: jlong)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 6092

  let mut event: android_event = std::mem::zeroed ();                   // C 6094 (PORTING.md 1.5)

  event.ime.r#type = ANDROID_INPUT_METHOD;                              // C 6096
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 6097 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 6098
  event.ime.operation = ANDROID_IME_REQUEST_SELECTION_UPDATE;           // C 6099
  event.ime.start = 0;                                                  // C 6100
  event.ime.end = 0;                                                    // C 6101
  event.ime.length = 0;                                                 // C 6102
  event.ime.position = 0;                                               // C 6103
  event.ime.text = std::ptr::null_mut ();                               // C 6104
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 6105 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 6107
}

/* C 6110-6133: `NATIVE_NAME (requestCursorUpdates)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_requestCursorUpdates")]
pub unsafe extern "C" fn requestCursorUpdates (env: *mut JNIEnv, object: jobject, window: jlong,
					       mode: jint)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 6114

  let mut event: android_event = std::mem::zeroed ();                   // C 6116 (PORTING.md 1.5)

  event.ime.r#type = ANDROID_INPUT_METHOD;                              // C 6118
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 6119 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 6120
  event.ime.operation = ANDROID_IME_REQUEST_CURSOR_UPDATES;             // C 6121
  event.ime.start = 0;                                                  // C 6122
  event.ime.end = 0;                                                    // C 6123
  event.ime.length = mode as size_t;                                    // C 6124
  event.ime.position = 0;                                               // C 6125
  event.ime.text = std::ptr::null_mut ();                               // C 6126

  /* Since this does not affect the state of the buffer text, there is
     no need to apply synchronization to this event.  */                // C 6128-6129
  event.ime.counter = 0;                                                // C 6130

  android_write_event (&raw mut event);                                 // C 6132
}

/* Notice that a new input method connection has been initialized and
   clear cursor update requests, extracted text requests, and the
   composing region.  */                                               // C 6135-6137

/* C 6139-6163: `NATIVE_NAME (clearInputFlags)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_clearInputFlags")]
pub unsafe extern "C" fn clearInputFlags (env: *mut JNIEnv, object: jobject, window: jlong)
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 6143

  let mut event: android_event = std::mem::zeroed ();                   // C 6145 (PORTING.md 1.5)

  event.ime.r#type = ANDROID_INPUT_METHOD;                              // C 6147
  event_serial += 1;
  event.ime.serial = event_serial as c_ulong;                           // C 6148 (the `++event_serial`)
  event.ime.window = window as android_window;                          // C 6149
  event.ime.operation = ANDROID_IME_FINISH_COMPOSING_TEXT;              // C 6150
  event.ime.start = 0;                                                  // C 6151
  event.ime.end = 0;                                                    // C 6152

  /* This value of `length' means that updates to the cursor position
     and extracted text should not be reported anymore.  */             // C 6154-6155

  event.ime.length = 2;                                                 // C 6157
  event.ime.position = 0;                                               // C 6158
  event.ime.text = std::ptr::null_mut ();                               // C 6159
  edit_counter += 1;
  event.ime.counter = edit_counter;                                     // C 6160 (the `++edit_counter`)

  android_write_event (&raw mut event);                                 // C 6162
}

/* C 6165-6356: the surrounding-text group - its context record, the worker,
   `android_get_surrounding_text_internal` (the `SurroundingText` class cache) and the JNI wrapper.
   Two devices are new here: C 6277's `#if __ANDROID_API__ < 31` is the derived feature
   `android_api_lt_31` (the `android_api_lt_21`/`android_api_le_30` family of
   `docs/baseline/feature-coverage.tsv`), and C 6266-6267's two function-scope `static`s are
   function-local `static mut`s (the `src/android.rs` 1201 / `src/androidselect.rs` 561 shape).  */

/* Context for a call to `getSurroundingText'.  */                      // C 6167

/* C 6169-6192: `struct android_get_surrounding_text_context`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct android_get_surrounding_text_context
{
  /* Number of characters before the region to return.  */
  pub before_length: c_int,                                             // C 6172

  /* Number of characters after the region to return.  */
  pub after_length: c_int,                                              // C 6175

  /* The returned text, or NULL.  */
  pub text: *mut c_char,                                                // C 6178

  /* The size of that text in characters and bytes.  */
  pub length: ptrdiff_t,                                                // C 6181
  pub bytes: ptrdiff_t,

  /* Offsets into that text.  */
  pub offset: ptrdiff_t,                                                // C 6184
  pub start: ptrdiff_t,
  pub end: ptrdiff_t,

  /* The start and end indices of the conversion region.
     -1 if it does not exist.  */
  pub conversion_start: ptrdiff_t,                                      // C 6188
  pub conversion_end: ptrdiff_t,

  /* The window.  */
  pub window: android_window,                                           // C 6191
}
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
const _: () = {
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, before_length) == 0);
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, after_length) == 4);
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, text) == 8);
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, length) == 16);
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, bytes) == 24);
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, offset) == 32);
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, start) == 40);
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, end) == 48);
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, conversion_start) == 56);
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, conversion_end) == 64);
  core::assert! (std::mem::offset_of! (android_get_surrounding_text_context, window) == 72);
  core::assert! (std::mem::size_of::<android_get_surrounding_text_context> () == 80);
};

/* Return the surrounding text in the surrounding text context
   specified by DATA.  */                                              // C 6194-6195

/* C 6197-6241.  `static` and the *address is taken* (C 6308), so `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_get_surrounding_text (data: *mut c_void)
{
  let mut request: *mut android_get_surrounding_text_context;           // C 6200
  let mut f: *mut frame;                                                // C 6201
  let mut temp: ptrdiff_t;                                              // C 6202

  request = data as *mut android_get_surrounding_text_context;          // C 6204

  /* Find the frame associated with the window.  */                      // C 6206
  f = android_window_to_frame (std::ptr::null_mut (), (*request).window);   // C 6207

  if f.is_null ()                                                       // C 6209
    { return; }                                                         // C 6210

  /* Now get the surrounding text.  */                                  // C 6212
  (*request).text
    = get_surrounding_text (f, (*request).before_length as ptrdiff_t,   // C 6214
			    (*request).after_length as ptrdiff_t, &raw mut (*request).length,   // C 6215
			    &raw mut (*request).bytes, &raw mut (*request).offset,   // C 6216
			    &raw mut (*request).start, &raw mut (*request).end);   // C 6217

  /* Sort request->start and request->end for compatibility with some
     bad input methods.  */                                             // C 6219-6220

  if (*request).end < (*request).start                                  // C 6222
    {
      temp = (*request).start;                                          // C 6224
      (*request).start = (*request).end;                                // C 6225
      (*request).end = temp;                                            // C 6226
    }

  /* Retrieve the conversion region.  */                                // C 6229

  (*request).conversion_start = -1;                                     // C 6231
  (*request).conversion_end = -1;                                       // C 6232

  if MARKERP ((*f).conversion.compose_region_start)                     // C 6234
    {
      (*request).conversion_start
	= marker_position ((*f).conversion.compose_region_start) - 1;   // C 6237
      (*request).conversion_end
	= marker_position ((*f).conversion.compose_region_end) - 1;     // C 6239
    }
}

/* Return a local reference to a `SurroundingText' object describing
   WINDOW's surrounding text.  ENV should be a valid JNI environment
   for the current thread.

   BEFORE_LENGTH and AFTER_LENGTH specify the number of characters
   around point and mark to return.

   Return the conversion region (or -1) in *CONVERSION_START and
   *CONVERSION_END if non-NULL.

   Value is the object upon success, else NULL.  */                    // C 6243-6253

/* C 6255-6345.  `static`; called from C 6354 and the snapshot entry.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_get_surrounding_text_internal (env: *mut JNIEnv, window: jlong,
						 before_length: jint,
						 after_length: jint,
						 conversion_start: *mut ptrdiff_t,
						 conversion_end: *mut ptrdiff_t)
  -> jobject
{
  let mut context: android_get_surrounding_text_context = std::mem::zeroed ();   // C 6262
  let mut string: jstring;                                              // C 6263
  let mut object: jobject;                                              // C 6264

  static mut class: jclass = std::ptr::null_mut ();                     // C 6266 (tentative, i.e. zero)
  static mut constructor: jmethodID = std::ptr::null_mut ();            // C 6267

  /* Initialize CLASS if it has not yet been initialized.  */            // C 6269

  if class.is_null ()                                                   // C 6271
    {
      class
	= ((*(*env)).FindClass.unwrap ()) (env, c"android/view/inputmethod/SurroundingText".as_ptr ()) as jclass;   // C 6273-6275

      /* C 6277-6288's `#if __ANDROID_API__ < 31` pair: the frozen macro table leaves
	 `__ANDROID_API__` undefined, so the reference's own preprocessor takes the first arm
	 (an undefined identifier is 0 in `#if`), and the mirror's measured target is API 24 -
	 the port carries the first arm under the derived feature `android_api_lt_31` and the
	 second under its negation (P-51).  */
      #[cfg (feature = "android_api_lt_31")]
      {
	/* If CLASS cannot be found, the version of Android currently
	   running is too old.  */                                    // C 6278-6279

	if class.is_null ()                                            // C 6281
	  {
	    ((*(*env)).ExceptionClear.unwrap ()) (env);                 // C 6283
	    return std::ptr::null_mut ();                              // C 6284
	  }
      }
      #[cfg (not (feature = "android_api_lt_31"))]
      { eassert! (! class.is_null ()); }                               // C 6287

      class = ((*(*env)).NewGlobalRef.unwrap ()) (env, class) as jclass;   // C 6290
      if class.is_null ()                                              // C 6291
	/* Clear class to prevent a local reference from remaining in
	   `class'.  */
	{
	  class = std::ptr::null_mut ();                                   // C 6294
	  return class;
	}

      /* Now look for its constructor.  */                             // C 6296
      constructor = ((*(*env)).GetMethodID.unwrap ()) (env, class, c"<init>".as_ptr (),   // C 6297
						      c"(Ljava/lang/CharSequence;III)V".as_ptr ());   // C 6298
      eassert! (! constructor.is_null ());                              // C 6299
    }

  context.before_length = before_length;                                // C 6302
  context.after_length = after_length;                                  // C 6303
  context.window = window as android_window;                            // C 6304
  context.text = std::ptr::null_mut ();                                 // C 6305

  android_sync_edit ();                                                 // C 6307
  if android_run_in_emacs_thread (Some (android_get_surrounding_text),   // C 6308
				  &raw mut context as *mut c_void) != 0   // C 6309
    { return std::ptr::null_mut (); }                                   // C 6310

  if context.text.is_null ()                                            // C 6312
    { return std::ptr::null_mut (); }                                   // C 6313

  /* Encode the returned text.  */                                      // C 6315
  string = android_text_to_string (env, context.text, context.length,   // C 6316
				   context.bytes);                      // C 6317
  free (context.text as *mut c_void);                                   // C 6318

  if string.is_null ()                                                  // C 6320
    { return std::ptr::null_mut (); }                                   // C 6321

  /* Create an SurroundingText object containing this information.  */    // C 6323
  object = ((*(*env)).NewObject.unwrap ()) (env, class, constructor, string,   // C 6324
					    min (context.start,          // C 6325
						 TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint,   // C 6326
					    min (context.end,            // C 6327
						 TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint,   // C 6328
					    /* Adjust point offsets to fit into
					       Android's 0-based indexing. */
					    min (context.offset - 1,     // C 6331
						 TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint);   // C 6332
  if object.is_null ()                                                  // C 6333
    { return std::ptr::null_mut (); }                                   // C 6334

  /* Now return the conversion region if that was requested.  */          // C 6336

  if ! conversion_start.is_null ()                                      // C 6338
    {
      *conversion_start = context.conversion_start;                     // C 6340
      /* C 6341 assigns `context.conversion_start` to `*conversion_end` as well (C's own text,
	 copied verbatim here - the port does not correct it).  */
      *conversion_end = context.conversion_start;
    }

  return object;                                                        // C 6344
}

/* C 6347-6356: `NATIVE_NAME (getSurroundingText)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_getSurroundingText")]
pub unsafe extern "C" fn getSurroundingText (env: *mut JNIEnv, object: jobject,
					     window: jlong, before_length: jint,
					     after_length: jint, flags: jint) -> jobject
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 6352

  return android_get_surrounding_text_internal (env, window, before_length,   // C 6354
						after_length, std::ptr::null_mut (),   // C 6355
						std::ptr::null_mut ());
}

/* C 6358-6544: `takeSnapshot` (the second `TextSnapshot` class cache), the two
   diagnostic-`#pragma` pops that close slice 14's pair, and the tiny frame test
   `android_event_is_for_frame`.  C 6385's `#if __ANDROID_API__ < 33` is the second API-level
   derived feature of this unit (`android_api_lt_33`, slice 22's `android_api_lt_31` family), and
   C 6366-6367's two function-scope `static`s take the same function-local `static mut` shape.  */

/* C 6358-6417: `NATIVE_NAME (takeSnapshot)`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(export_name = "Java_org_femacs_EmacsNative_takeSnapshot")]
pub unsafe extern "C" fn takeSnapshot (env: *mut JNIEnv, object: jobject, window: jlong)
  -> jobject
{
  JNI_STACK_ALIGNMENT_PROLOGUE! ();                                     // C 6361

  let mut text: jobject;                                                // C 6363
  /* C 6364 leaves `start` and `end` to the callee's writes through their addresses (the
     `scale_x` device of slices 8/12).  */
  let mut start: ptrdiff_t = 0;                                         // C 6364
  let mut end: ptrdiff_t = 0;

  static mut class: jclass = std::ptr::null_mut ();                     // C 6366 (tentative, i.e. zero)
  static mut constructor: jmethodID = std::ptr::null_mut ();            // C 6367

  /* First, obtain the surrounding text and conversion region.  */        // C 6369
  text = android_get_surrounding_text_internal (env, window, 600, 600,  // C 6370
						&raw mut start, &raw mut end);   // C 6371

  /* If that fails, return NULL.  */                                    // C 6373

  if text.is_null ()                                                    // C 6375
    { return std::ptr::null_mut (); }                                   // C 6376

  /* Next, initialize the TextSnapshot class.  */                        // C 6378

  if class.is_null ()                                                   // C 6380
    {
      class
	= ((*(*env)).FindClass.unwrap ()) (env, c"android/view/inputmethod/TextSnapshot".as_ptr ()) as jclass;   // C 6382-6384
      /* C 6385-6396's `#if __ANDROID_API__ < 33` pair: the same reading as slice 22's
	 `android_api_lt_31` (the frozen table leaves `__ANDROID_API__` undefined, and the
	 mirror's measured target is API 24).  */
      #[cfg (feature = "android_api_lt_33")]
      {
	/* If CLASS cannot be found, the version of Android currently
	   running is too old.  */                                    // C 6386-6387

	if class.is_null ()                                            // C 6389
	  {
	    ((*(*env)).ExceptionClear.unwrap ()) (env);                 // C 6391
	    return std::ptr::null_mut ();                              // C 6392
	  }
      }
      #[cfg (not (feature = "android_api_lt_33"))]
      { eassert! (! class.is_null ()); }                               // C 6395

      class = ((*(*env)).NewGlobalRef.unwrap ()) (env, class) as jclass;   // C 6398
      if class.is_null ()                                              // C 6399
	/* Clear class to prevent a local reference from remaining in
	   `class'.  */
	{
	  class = std::ptr::null_mut ();                                   // C 6402
	  return class;
	}

      constructor = ((*(*env)).GetMethodID.unwrap ()) (env, class, c"<init>".as_ptr (),   // C 6404
						      c"(Landroid/view/inputmethod/SurroundingText;III)V".as_ptr ());   // C 6405-6406
      eassert! (! constructor.is_null ());                              // C 6407
    }

  /* Try to create a TextSnapshot object.  */                            // C 6410
  eassert! (start <= end);                                              // C 6411
  /* C 6412 assigns C's `jobject object` parameter (never read before this line), so the port
     shadows the parameter with the local of the same name.  */
  let object: jobject = ((*(*env)).NewObject.unwrap ()) (env, class, constructor, text,   // C 6412
					    min (start, TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint,   // C 6413
					    min (end, TYPE_MAXIMUM! (jint) as ptrdiff_t) as jint,   // C 6414
					    0 as jint);                     // C 6415
  return object;                                                        // C 6416
}

/* C 6419-6423: the two `#pragma` pops that close the pair slice 14 carried.  C 6419-6420's
   `#ifdef __clang__` arm is `#pragma clang diagnostic pop` and C 6422's `#else` arm the `#pragma
   GCC` twin; Rust has no counterpart for either, so the text is C's own in this comment (the
   `docs/ports/excluded-macros.txt` class, the C 5014-5020 push's other half).  */

/* Tell the input method where the composing region and selection of
   F's selected window is located.  W should be F's selected window;
   if it is NULL, then F->selected_window is used in its place.  */    // C 6427-6429

/* Return whether or not EVENT is an input method event destined for
   the frame (struct frame *) ARG.  */                                 // C 6533-6534

/* C 6536-6544.  `static` and the *address is taken* (the event-dispatch table), so
   `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_event_is_for_frame (event: *mut android_event, arg: *mut c_void)
  -> bool
{
  let mut f: *mut frame;                                                // C 6539

  f = arg as *mut frame;                                                // C 6541
  return (*event).r#type == ANDROID_INPUT_METHOD                        // C 6542
    && (*event).ime.window == FRAME_ANDROID_WINDOW! (f);                // C 6543
}

/* C 6425-6531: `android_update_selection`, the 107-line function the table has carried as a
   `recorded, not ported` row since slice 23 (it sits between that slice's blocks).  One name the
   mirror describes but cannot carry is named here: C 340-342's `ANDROID_DELETE_LOCAL_REF`
   (`include/android.rs` 197-199 records that it reads a `<jni.h>` vtable entry with no Rust text),
   so the unit spells it as the `src/android.rs` 1427 macro.  */

/* C: android.h 340-342: `#define ANDROID_DELETE_LOCAL_REF(ref) ((*android_java_env)->DeleteLocalRef
   (android_java_env, ref))` - the `src/android.rs` 1427 copy.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! ANDROID_DELETE_LOCAL_REF {
  ($r:expr) => { ((*(*android_java_env)).DeleteLocalRef.unwrap ()) (android_java_env, $r) };
}

/* Tell the input method where the composing region and selection of
   F's selected window is located.  W should be F's selected window;
   if it is NULL, then F->selected_window is used in its place.  */    // C 6427-6429

/* C 6431-6531.  `static`; called from slice 1's `android_change_...` paths (C 2015/2038/2085/2114)
   and the frame's redisplay callbacks.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_update_selection (f: *mut frame, mut w: *mut window)
{
  /* C 6434-6442's locals that the callees write through their addresses start at 0 (the
     `field_start`/`field_end` pair of C 6445 is the same device).  */
  let mut start: ptrdiff_t = 0;                                         // C 6434
  let mut end: ptrdiff_t = 0;
  let mut point: ptrdiff_t;
  let mut mark: ptrdiff_t;
  let mut start_offset: ptrdiff_t = 0;                                  // C 6434-6435
  let mut end_offset: ptrdiff_t = 0;
  let mut length: ptrdiff_t = 0;
  let mut bytes: ptrdiff_t = 0;
  let mut b: *mut buffer;                                               // C 6436
  let mut hint: c_int;                                                  // C 6437
  let mut token: c_int;
  let mut text: *mut c_char;                                            // C 6438
  let mut extracted: jobject;                                           // C 6439
  let mut string: jstring;                                              // C 6440
  let mut mark_active: bool = false;                                    // C 6441
  let mut field_start: ptrdiff_t = 0;                                   // C 6442
  let mut field_end: ptrdiff_t = 0;

  /* Offset these values by the start offset of the field.  */           // C 6444
  get_conversion_field (f, &raw mut field_start, &raw mut field_end);    // C 6445

  if MARKERP ((*f).conversion.compose_region_start)                     // C 6447
    {
      eassert! (MARKERP ((*f).conversion.compose_region_end));           // C 6449

      /* Indexing in android starts from 0 instead of 1.  */            // C 6451
      start = marker_position ((*f).conversion.compose_region_start);   // C 6452
      end = marker_position ((*f).conversion.compose_region_end);       // C 6453

      /* Offset and detect underflow.  */                              // C 6455
      start = max (start, field_start) - field_start;                   // C 6456
      end = min (end, field_end) - field_start;                         // C 6457
      if end < 0 || start < 0                                            // C 6458
	{
	  start = -1;                                                      // C 6459
	  end = -1;
	}
    }
  else
    { start = -1; end = -1; }                                           // C 6462

  /* Now constrain START and END to the maximum size of a Java
     integer.  */                                                      // C 6464-6465
  start = min (start, TYPE_MAXIMUM! (jint) as ptrdiff_t);               // C 6466
  end = min (end, TYPE_MAXIMUM! (jint) as ptrdiff_t);                   // C 6467

  if w.is_null ()                                                       // C 6469
    { w = XWINDOW ((*f).selected_window); }                            // C 6470

  /* Figure out where the point and mark are.  If the mark is not
     active, then point is set to equal mark.  */                      // C 6472-6473
  b = XBUFFER ((*w).contents);                                          // C 6474
  point = min (min (max (ephemeral_last_point! (w),                     // C 6475
			 field_start),
		    field_end) - field_start,
	       TYPE_MAXIMUM! (jint) as ptrdiff_t);                      // C 6478
  mark = if ! NILP (BVAR! (b, mark_active_))                            // C 6479
	    && (*w).last_mark != -1                                     // C 6480
    { min (min (max ((*w).last_mark, field_start),                      // C 6481
	       field_end) - field_start,
	   TYPE_MAXIMUM! (jint) as ptrdiff_t) }                         // C 6483
    else { point };                                                     // C 6484

  /* Send the update.  Android doesn't employ a concept of "point" and
     "mark"; instead, it only has a selection, where the start of the
     selection is less than or equal to the end, and the region is
     "active" when those two values differ.  The indices will have been
     converted from 1-based Emacs indices to 0-based Android ones.  */   // C 6486-6490
  android_update_ic (FRAME_ANDROID_WINDOW! (f), min (point, mark),      // C 6491
		     max (point, mark), start, end);                    // C 6492

  /* Update the extracted text as well, if the input method has asked
     for updates.  1 is InputConnection.GET_EXTRACTED_TEXT_MONITOR.  */   // C 6494-6495

  if (*(FRAME_ANDROID_OUTPUT! (f) as *mut android_output)).extracted_text_flags & 1 != 0   // C 6497
    {
      hint = (*(FRAME_ANDROID_OUTPUT! (f) as *mut android_output)).extracted_text_hint;   // C 6499
      token = (*(FRAME_ANDROID_OUTPUT! (f) as *mut android_output)).extracted_text_token;   // C 6500
      text = get_extracted_text (f, min (hint, 600) as ptrdiff_t, &raw mut start,   // C 6501
				 &raw mut start_offset, &raw mut end_offset,   // C 6502
				 &raw mut length, &raw mut bytes, &raw mut mark_active);   // C 6503

      if ! text.is_null ()                                              // C 6505
	{
	  /* Make a string out of the extracted text.  */               // C 6507
	  string = android_text_to_string (android_java_env,            // C 6508
					   text, length, bytes);        // C 6509
	  xfree (text as *mut c_void);                                  // C 6510
	  android_exception_check ();                                   // C 6511

	  /* Make extracted text out of that string.  */                // C 6513
	  extracted = android_build_extracted_text (string, start,       // C 6514
						    start_offset,       // C 6515
						    end_offset,         // C 6516
						    mark_active);       // C 6517
	  android_exception_check_1 (string);                           // C 6518
	  ANDROID_DELETE_LOCAL_REF! (string);                            // C 6519

	  if ! extracted.is_null ()                                     // C 6521
	    {
	      /* extracted is now an associated ExtractedText object.
		 Perform the update.  */                              // C 6523-6524
	      android_update_extracted_text (FRAME_ANDROID_WINDOW! (f),   // C 6525
					     extracted as *mut c_void, token);   // C 6526
	      ANDROID_DELETE_LOCAL_REF! (extracted);                     // C 6527
	    }
	}
    }
}

/* C 6546-6658: the text-conversion group's four callbacks and the interface record they fill -
   `android_reset_conversion`, `android_set_point`, `android_compose_region_changed`,
   **`android_notify_conversion`** (C 6635-6648: the file's last forward reference, and the reason
   the arm probe has carried three errors since slice 1) and `text_conversion_interface`
   (C 6652-6658).  This slice also completes the two helper sets Owed 28 named: the atomic store
   beside slice 14's load, and `sem_getvalue`/`sem_post` beside `sem_timedwait`.  */

/* C 6636's `__atomic_store_n (&last_edit_counter, counter, __ATOMIC_SEQ_CST)` - the store half of
   slice 14's helper (PORTING.md §6: the same object, the same memory order).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn __atomic_store_n (p: *mut c_ulong, v: c_ulong, order: core::sync::atomic::Ordering)
{
  (*(p as *mut core::sync::atomic::AtomicU64)).store (v, order)          // C 6641-6642
}

/* The two `<semaphore.h>` entries C 6644/6647 calls (the `sem_timedwait` declaration of slice 14
   is above).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /* int sem_getvalue (sem_t *, int *);  */
  pub fn sem_getvalue (sem: *mut sem_t, sval: *mut c_int) -> c_int;
  /* int sem_post (sem_t *);  */
  pub fn sem_post (sem: *mut sem_t) -> c_int;
}

/* Notice that the input method connection to F should be reset as a
   result of a change to its contents.  */                           // C 6546-6547

/* C 6549-6612.  `static` and the *address is taken* (C 6654), so `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_reset_conversion (f: *mut frame)
{
  let mut mode: android_ic_mode;                                        // C 6552
  let mut w: *mut window;                                               // C 6553
  let mut buffer: *mut buffer;                                          // C 6554
  let mut style: Lisp_Object;                                           // C 6555
  let mut event: android_event = std::mem::zeroed ();                   // C 6556 (PORTING.md 1.5)

  /* Reset the input method.

     Select an appropriate ``input mode'' based on whether or not the
     minibuffer window is selected, which in turn affects if ``RET''
     inserts a newline or sends an editor action Emacs transforms into
     a key event (refer to `performEditorAction'.)  */                  // C 6558-6563

  w = XWINDOW ((*f).selected_window);                                   // C 6565
  buffer = XBUFFER (WINDOW_BUFFER (w));                                 // C 6566

  style = if EQ (find_symbol_value (Qoverriding_text_conversion_style ()),   // C 6568
		 Qlambda ())                                            // C 6569
    { BVAR! (buffer, text_conversion_style_) }                          // C 6570
    else { find_symbol_value (Qoverriding_text_conversion_style ()) };   // C 6571

  if NILP (style) || conversion_disabled_p ()                           // C 6573
    { mode = ANDROID_IC_MODE_NULL; }                                    // C 6574
  else if EQ (style, Qpassword ())                                      // C 6575
    { mode = ANDROID_IC_MODE_PASSWORD; }                                // C 6576
  else if EQ (style, Qaction ()) || EQ ((*f).selected_window,           // C 6577
						(*f).minibuffer_window)   // C 6578
    { mode = ANDROID_IC_MODE_ACTION; }                                  // C 6579
  else
    { mode = ANDROID_IC_MODE_TEXT; }                                    // C 6581

  /* Remove any existing input method events that apply to FRAME from
     the event queue.

     There's a small window between this and the call to
     android_reset_ic between which more events can be generated.  */   // C 6583-6587

  while android_check_if_event (&raw mut event,                          // C 6589
				Some (android_event_is_for_frame),      // C 6589
				f as *mut c_void)                       // C 6589
    {
      /* C 6591-6601's `switch`: three labels share C's `xfree` body and `default:` is the
	 empty arm (the `macro-body-notes.tsv` row this slice appends).  */
      match event.ime.operation
	{
	  ANDROID_IME_COMMIT_TEXT
	    | ANDROID_IME_FINISH_COMPOSING_TEXT                             // C 6593-6595
	    | ANDROID_IME_SET_COMPOSING_TEXT =>
	    { xfree (event.ime.text as *mut c_void); }                      // C 6596

	  _ => {}                                                           // C 6599-6600
	}
    }

  android_reset_ic (FRAME_ANDROID_WINDOW! (f), mode);                   // C 6604

  /* Clear extracted text flags.  Since the IM has been reinitialized,
     it should no longer be displaying extracted text.  */              // C 6606-6607
  (*(FRAME_ANDROID_OUTPUT! (f) as *mut android_output)).extracted_text_flags = 0;   // C 6608

  /* Move its selection to the specified position.  */                   // C 6610
  android_update_selection (f, std::ptr::null_mut ());                  // C 6611
}

/* Notice that point has moved in the F's selected window's selected
   buffer.  W is the window, and BUFFER is that buffer.  */            // C 6614-6615

/* C 6617-6622.  `static` and the *address is taken* (C 6655), so `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_set_point (f: *mut frame, w: *mut window, buffer: *mut buffer)
{
  android_update_selection (f, w);                                      // C 6621
}

/* Notice that the composition region on F's old selected window has
   changed.  */                                                       // C 6624-6625

/* C 6627-6631.  `static` and the *address is taken* (C 6656).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_compose_region_changed (f: *mut frame)
{
  android_update_selection (f, XWINDOW ((*f).old_selected_window));     // C 6630
}

/* Notice that the text conversion has completed.  */                   // C 6633

/* C 6635-6648.  `static` and the *address is taken* (C 6657) - and the file's last forward
   reference, which the arm probe's third error named from slice 1 until this round.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_notify_conversion (counter: c_ulong)
{
  let mut sval: c_int = 0;                                              // C 6638 (the `&raw mut` place)

  if last_edit_counter < counter                                        // C 6640
    { __atomic_store_n (&raw mut last_edit_counter, counter,            // C 6641
			__ATOMIC_SEQ_CST); }                            // C 6642

  sem_getvalue (&raw mut edit_sem, &raw mut sval);                      // C 6644

  if sval <= 0                                                          // C 6646
    { sem_post (&raw mut edit_sem); }                                   // C 6647
}

/* Android text conversion interface.  */                                // C 6650

/* C 6652-6658: the record itself.  `point_changed`'s mirror field is typed
   `(*mut frame, *mut c_void, *mut c_void)` (the mirror's own `C 42-43` note), so assigning C's
   `android_set_point (struct frame *, struct window *, struct buffer *)` needs the cast the
   `src/pgtkterm.rs` 5450 redisplay-interface row uses.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut text_conversion_interface: textconv_interface =                  // C 6652
  textconv_interface
  {
    reset: Some (android_reset_conversion),                             // C 6654
    point_changed: Some (unsafe { core::mem::transmute::<*const (),
			     unsafe extern "C" fn (*mut frame, *mut c_void, *mut c_void)> (
			       android_set_point as *const ()) }),      // C 6655
    compose_region_changed: Some (android_compose_region_changed),      // C 6656
    notify_conversion: Some (android_notify_conversion),                // C 6657
  };

/* C 6660-6718: the end of C 47's `#ifndef ANDROID_STUBIFY` arm, the redisplay-interface table, and
   the two small terminal-side entry points (`frame_set_mouse_pixel_position`, `get_keysym_name`).
   The table's type is the mirror's full record
   (`dispextern_unit_names::redisplay_interface`, dispextern.h 3069-3196, its 26 fields in C's
   order), and its first entry needs one use-site declaration: C's `android_frame_parm_handlers[]`
   is defined in androidfns.c 3142 and declared in androidterm.h 398.  */

/* `frame_parm_handler` and the seven `gui_*` entries C 6667-6679's table names and **no mirror
   carries**: dispextern.h 3063's typedef is the element type of C's
   `android_frame_parm_handlers[]` (the `src/haikufns.rs` 218-225 alias), and dispextern.h
   3636-3648/3689 declare the seven `xdisp.c` definitions (the `src/haikuterm.rs` 4555-4583
   block, same citations).  */
/// C: dispextern.h 3063: `typedef void (*frame_parm_handler) (struct frame *, Lisp_Object,
/// Lisp_Object);`
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
type frame_parm_handler =
  Option<unsafe extern "C" fn (f: *mut frame, val: Lisp_Object, oldval: Lisp_Object)>;
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  /// C: dispextern.h 3636-3638: `extern void gui_get_glyph_overhangs (struct glyph *,
  /// struct frame *, int *, int *);` - C 6678's entry.
  pub fn gui_get_glyph_overhangs (glyph: *mut glyph, f: *mut frame, left: *mut c_int,
				  right: *mut c_int);
  /// C: dispextern.h 3639: `extern void gui_produce_glyphs (struct it *);` - C 6668's entry.
  pub fn gui_produce_glyphs (it: *mut c_void);
  /// C: dispextern.h 3641-3642: `extern void gui_write_glyphs (struct window *,
  /// struct glyph_row *, struct glyph *, enum glyph_row_area, int);` - C 6669's entry.
  pub fn gui_write_glyphs (w: *mut window, row: *mut glyph_row, string: *mut glyph,
			   area: glyph_row_area, len: c_int);
  /// C: dispextern.h 3643-3644: the `gui_insert_glyphs` pair - C 6670's entry.
  pub fn gui_insert_glyphs (w: *mut window, row: *mut glyph_row, start: *mut glyph,
			    area: glyph_row_area, len: c_int);
  /// C: dispextern.h 3645-3646: `extern void gui_clear_end_of_line (struct window *,
  /// struct glyph_row *, enum glyph_row_area, int);` - C 6671's entry.
  pub fn gui_clear_end_of_line (w: *mut window, row: *mut glyph_row, area: glyph_row_area,
				x: c_int);
  /// C: dispextern.h 3647-3648: `extern void gui_fix_overlapping_area (struct window *,
  /// struct glyph_row *, enum glyph_row_area, int);` - C 6679's entry.
  pub fn gui_fix_overlapping_area (w: *mut window, row: *mut glyph_row, area: glyph_row_area,
				   overlaps: c_int);
  /// C: dispextern.h 3689: `extern void gui_clear_window_mouse_face (struct window *);` -
  /// C 6677's entry.
  pub fn gui_clear_window_mouse_face (w: *mut window);
}

/* C 47's real arm ends here.  */
/* C 6662: `#endif /* !ANDROID_STUBIFY */` - the closing half of the include-region's gate; every
   item below this line is at file scope, and the file's two `#ifdef ANDROID_STUBIFY` arms are
   C 6782-6784 and C 6884-6885 (both carried as text).  */

/* C: androidterm.h 398: `extern frame_parm_handler android_frame_parm_handlers[];` - the table C
   *defines* in androidfns.c 3142-3195 (51 entries, measured: 51 `,`-terminated entries between the
   braces), the `src/pgtkterm.rs` 5396-5409 device.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub static mut android_frame_parm_handlers: [frame_parm_handler; 51];
}

/* C 6664-6696.  A `static` record at file scope; C's positional initializer is the port's named
   fields in the mirror's order (offsets 0-200), and C's `#ifndef ANDROID_STUBIFY` arm is the
   whole list (the `#else` arm is the empty initializer C 6695's `#endif` leaves).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
static mut android_redisplay_interface: redisplay_interface =
  redisplay_interface
  {
    frame_parm_handlers: &raw mut android_frame_parm_handlers as *mut c_void,   // C 6667
    produce_glyphs: Some (gui_produce_glyphs),                          // C 6668
    write_glyphs: Some (gui_write_glyphs),                              // C 6669
    insert_glyphs: Some (gui_insert_glyphs),                            // C 6670
    clear_end_of_line: Some (gui_clear_end_of_line),                    // C 6671
    scroll_run_hook: Some (android_scroll_run),                         // C 6672
    after_update_window_line_hook: Some (android_after_update_window_line),   // C 6673
    update_window_begin_hook: None,                                     // C 6674 (`NULL`)
    update_window_end_hook: None,                                       // C 6675 (`NULL`)
    flush_display: Some (android_flip_and_flush),                       // C 6676
    clear_window_mouse_face: Some (gui_clear_window_mouse_face),        // C 6677
    get_glyph_overhangs: Some (gui_get_glyph_overhangs),                // C 6678
    fix_overlapping_area: Some (gui_fix_overlapping_area),              // C 6679
    draw_fringe_bitmap: Some (android_draw_fringe_bitmap),              // C 6680
    define_fringe_bitmap: None,                                         // C 6681 (`NULL`)
    destroy_fringe_bitmap: None,                                        // C 6682 (`NULL`)
    compute_glyph_string_overhangs: Some (android_compute_glyph_string_overhangs),   // C 6683
    draw_glyph_string: Some (android_draw_glyph_string),                // C 6684
    define_frame_cursor: Some (android_define_frame_cursor),            // C 6685
    clear_frame_area: Some (android_clear_frame_area),                  // C 6686
    clear_under_internal_border: Some (android_clear_under_internal_border),   // C 6687
    draw_window_cursor: Some (android_draw_window_cursor),              // C 6688
    draw_vertical_window_border: Some (android_draw_vertical_window_border),   // C 6689
    draw_window_divider: Some (android_draw_window_divider),            // C 6690
    shift_glyphs_for_insert: None,                                      // C 6691 (`NULL`)
    show_hourglass: Some (android_show_hourglass),                      // C 6692
    hide_hourglass: Some (android_hide_hourglass),                      // C 6693
    default_font_parameter: Some (android_default_font_parameter),       // C 6694
  };

/* C 6700-6705: `frame_set_mouse_pixel_position`.  Non-`static` (termhooks.h declares it) and no
   address is taken: `#[unsafe(no_mangle)]` with the C ABI; C's body is the comment alone, which is
   why the port's body is the same comment and nothing else.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn frame_set_mouse_pixel_position (f: *mut frame, pix_x: c_int, pix_y: c_int)
{
  /* This cannot be implemented on Android, and as such is left
     blank.  */                                                        // C 6703-6704
}

/* C 6707-6718: `get_keysym_name`.  Non-`static` and no address is taken.  C's function-scope
   `static char buffer[64]` is a function-local `static mut` (the `src/android.rs` 1201 shape), and
   C 6712-6716's `#ifndef ANDROID_STUBIFY` pair carries both arms.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_keysym_name (keysym: c_int) -> *mut c_char
{
  static mut buffer: [c_char; 64] = [0; 64];                            // C 6710

  #[cfg (not (feature = "android_stubify"))]
  {
    android_get_keysym_name (keysym, &raw mut buffer as *mut c_char, 64);   // C 6713
  }
  #[cfg (feature = "android_stubify")]
  { emacs_abort (); }                                                   // C 6715

  return &raw mut buffer as *mut c_char;                                // C 6717
}

/* C 6720-6787: `android_create_terminal`, the ~28 `terminal` hook assignments C 6739-6780 makes.
   The `terminal` record is the mirror's and types every `struct frame *`/`struct window *` slot
   `*mut c_void`, so each assignment goes through the tree's `hook!` macro - the
   `src/pgtkterm.rs` 7032 / `src/haikuterm.rs` 6238 / `src/msdos.rs` 2586 device - whose target
   type is the field's own.  */

/* C: termhooks.h 964-965: `extern struct terminal *create_terminal (enum output_method,
   struct redisplay_interface *);` - `src/terminal.rs` 366 defines it; no mirror carries the
   prototype (the `src/haikuterm.rs` 6244-6246 note).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub fn create_terminal (output_method: output_method, rif: *mut c_void) -> *mut terminal;
}

/* The tree's hook-slot cast (the `src/pgtkterm.rs` 7032-7036 copy).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! hook {
  ($target:ty, $f:expr) => { unsafe {
    let p: *const c_void = $f as *const c_void;
    std::mem::transmute::<*const c_void, Option<$target>> (p) } };
}

/* Create a struct terminal, initialize it with the Android specific
   functions and make DISPLAY->TERMINAL point to it.  */              // C 6722-6723

/* C 6725-6787.  `static`; only called (C 6800) - the address is not taken.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe fn android_create_terminal (dpyinfo: *mut android_display_info) -> *mut terminal
{
  let terminal: *mut terminal;                                          // C 6728

  terminal = create_terminal (output_android,                           // C 6730
			      &raw mut android_redisplay_interface as *mut c_void);   // C 6731
  (*terminal).display_info.android = dpyinfo as *mut c_void;            // C 6732
  (*dpyinfo).terminal = terminal;                                       // C 6733

  /* kboard is initialized in android_term_init.  */                     // C 6735

  #[cfg (not (feature = "android_stubify"))]
  {
    (*terminal).clear_frame_hook = hook! (unsafe extern "C" fn (*mut c_void), android_clear_frame);   // C 6739
    (*terminal).ring_bell_hook = hook! (unsafe extern "C" fn (*mut c_void), android_ring_bell);   // C 6740
    (*terminal).toggle_invisible_pointer_hook                           // C 6741-6742
      = hook! (unsafe extern "C" fn (*mut c_void, bool), android_toggle_invisible_pointer);
    (*terminal).update_begin_hook = hook! (unsafe extern "C" fn (*mut c_void), android_update_begin);   // C 6743
    (*terminal).update_end_hook = hook! (unsafe extern "C" fn (*mut c_void), android_update_end);   // C 6744
    (*terminal).read_socket_hook                                        // C 6745
      = hook! (unsafe extern "C" fn (*mut terminal, *mut input_event) -> c_int, android_read_socket);
    (*terminal).frame_up_to_date_hook                                   // C 6746
      = hook! (unsafe extern "C" fn (*mut c_void), android_frame_up_to_date);
    (*terminal).buffer_flipping_unblocked_hook                          // C 6747-6748
      = hook! (unsafe extern "C" fn (*mut c_void), android_buffer_flipping_unblocked_hook);
    (*terminal).defined_color_hook = hook! (unsafe extern "C" fn (*mut c_void, *const c_char,   // C 6749
								  *mut Emacs_Color, bool, bool) -> bool,
					    android_defined_color);
    (*terminal).query_frame_background_color                            // C 6750-6751
      = hook! (unsafe extern "C" fn (*mut c_void, *mut Emacs_Color), android_query_frame_background_color);
    (*terminal).query_colors                                            // C 6752
      = hook! (unsafe extern "C" fn (*mut c_void, *mut Emacs_Color, c_int), android_query_colors);
    (*terminal).mouse_position_hook = hook! (unsafe extern "C" fn (*mut *mut c_void, c_int,   // C 6753
								   *mut Lisp_Object, *mut scroll_bar_part,
								   *mut Lisp_Object, *mut Lisp_Object,
								   *mut Time), android_mouse_position);
    (*terminal).get_focus_frame = hook! (unsafe extern "C" fn (*mut c_void) -> Lisp_Object,   // C 6754
					 android_get_focus_frame);
    (*terminal).focus_frame_hook = hook! (unsafe extern "C" fn (*mut c_void, bool), android_focus_frame);   // C 6755
    (*terminal).frame_rehighlight_hook                                  // C 6756
      = hook! (unsafe extern "C" fn (*mut c_void), android_frame_rehighlight_hook);
    (*terminal).frame_raise_lower_hook                                  // C 6757
      = hook! (unsafe extern "C" fn (*mut c_void, bool), android_frame_raise_lower);
    (*terminal).frame_visible_invisible_hook                            // C 6758-6759
      = hook! (unsafe extern "C" fn (*mut c_void, bool), android_make_frame_visible_invisible);
    (*terminal).fullscreen_hook = hook! (unsafe extern "C" fn (*mut c_void), android_fullscreen_hook);   // C 6760
    (*terminal).iconify_frame_hook = hook! (unsafe extern "C" fn (*mut c_void), android_iconify_frame);   // C 6761
    (*terminal).set_window_size_hook                                    // C 6762
      = hook! (unsafe extern "C" fn (*mut c_void, bool, c_int, c_int), android_set_window_size);
    (*terminal).set_window_size_and_position_hook                       // C 6763-6764
      = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int), android_set_window_size_and_position);
    (*terminal).set_frame_offset_hook                                   // C 6765
      = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int), android_set_offset);
    (*terminal).set_frame_alpha_hook = hook! (unsafe extern "C" fn (*mut c_void), android_set_alpha);   // C 6766
    (*terminal).set_new_font_hook                                       // C 6767
      = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object, c_int) -> Lisp_Object, android_new_font);
    (*terminal).set_bitmap_icon_hook                                    // C 6768
      = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object) -> bool, android_bitmap_icon);
    (*terminal).implicit_set_name_hook                                  // C 6769
      = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object, Lisp_Object), android_implicitly_set_name);
    (*terminal).menu_show_hook = hook! (unsafe extern "C" fn (*mut c_void, c_int, c_int, c_int,   // C 6770
							      Lisp_Object, *mut *const c_char) -> Lisp_Object,
					android_menu_show);
    (*terminal).popup_dialog_hook                                       // C 6771
      = hook! (unsafe extern "C" fn (*mut c_void, Lisp_Object, Lisp_Object) -> Lisp_Object,
	       android_popup_dialog);
    (*terminal).change_tab_bar_height_hook                              // C 6772
      = hook! (unsafe extern "C" fn (*mut c_void, c_int), android_change_tab_bar_height);
    (*terminal).change_tool_bar_height_hook                             // C 6773
      = hook! (unsafe extern "C" fn (*mut c_void, c_int), android_change_tool_bar_height);
    (*terminal).set_scroll_bar_default_width_hook                       // C 6774-6775
      = hook! (unsafe extern "C" fn (*mut c_void), android_set_scroll_bar_default_width);
    (*terminal).set_scroll_bar_default_height_hook                      // C 6776-6777
      = hook! (unsafe extern "C" fn (*mut c_void), android_set_scroll_bar_default_height);
    (*terminal).free_pixmap = hook! (unsafe extern "C" fn (*mut c_void, *mut c_void), android_free_pixmap_hook);   // C 6778
    (*terminal).delete_frame_hook = hook! (unsafe extern "C" fn (*mut c_void), android_delete_frame);   // C 6779
    (*terminal).delete_terminal_hook                                    // C 6780
      = hook! (unsafe extern "C" fn (*mut terminal), android_delete_terminal);
  }
  #[cfg (feature = "android_stubify")]
  { emacs_abort (); }                                                   // C 6783

  return terminal;                                                      // C 6786
}

/* C 6789-6874: `android_term_init`, the third-from-last definition of the file.  It needs two
   use-site declarations (`allocate_kboard`, keyboard.h 477, and `sem_init`, the third
   `<semaphore.h>` entry this file calls), and C 6843's `ckd_add` is the P-15
   `ckd_add`/`overflowing_add` pair (`docs/ports/token-aliases.tsv` 13, the C 662 site of this
   unit and `src/androidmenu.rs` 555-558).  C 6838 declares `title` as a `static char const` array
   and C 6841 an `at` twin; the port keeps the same bytes, because `x_id_name`'s text is what X11
   sees.  */

/* C: keyboard.h 38: `#define KVAR (kboard, field) ((kboard)->field ## _)` - a token-pasting
   macro the mirror records as having no Rust text (`include/keyboard.rs` 10), so the unit names
   the one field it reads - the `src/window.rs` 6106 / `src/haikufns.rs` 1299 device - and the
   field's own spelling (`Vinput_decode_map_`).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
macro_rules! KVAR {
  ($kb:expr, Vinput_decode_map) => { (*( $kb as *mut kboard )).Vinput_decode_map_ };
}

/* C: keyboard.h 477: `extern KBOARD *allocate_kboard (Lisp_Object);` - C 6801.  The mirror does
   not carry the prototype (the `src/haikuterm.rs` 6244-6250 note).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub fn allocate_kboard (type_: Lisp_Object) -> *mut kboard;
  /// C: lisp.h 5371: `extern AVOID fatal (const char *msgid, ...)
  /// ATTRIBUTE_FORMAT_PRINTF (1, 2);` - C 6818's call; the `...` parameter *is* the list
  /// (PORTING.md P-04, the `src/haikuterm.rs` 6261 spelling).
  pub fn fatal (msgid: *const c_char, ...) -> !;
  /* int sem_init (sem_t *, int, unsigned int);  -- the third `<semaphore.h>` entry this file
     calls (C 6864).  */
  pub fn sem_init (sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
}

/* Initialize the Android terminal interface.  The display connection
   has already been set up by the system at this point.  */            // C 6789-6790

/* C 6792-6874.  Non-`static` (androidterm.h declares it) and no address is taken:
   `#[unsafe(no_mangle)]` with the C ABI.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_term_init ()
{
  let mut terminal: *mut terminal;                                      // C 6795
  let mut dpyinfo: *mut android_display_info;                           // C 6796
  let mut color_file: Lisp_Object;                                      // C 6797
  let mut color_map: Lisp_Object;

  dpyinfo = xzalloc (std::mem::size_of::<android_display_info> ()) as *mut android_display_info;   // C 6799
  terminal = android_create_terminal (dpyinfo);                         // C 6800
  (*terminal).kboard = allocate_kboard (Qandroid ()) as *mut c_void;     // C 6801
  (*( (*terminal).kboard as *mut kboard )).reference_count += 1;        // C 6802
  (*dpyinfo).n_planes = 24;                                             // C 6803
  (*dpyinfo).n_image_planes = 24;                                       // C 6804

  /* This function should only be called once at startup.  */            // C 6806
  eassert! (x_display_list.is_null ());                                 // C 6807
  x_display_list = dpyinfo;                                             // C 6808

  (*dpyinfo).name_list_element
    = Fcons (build_string (c"android".as_ptr ()), Qnil ());             // C 6810-6811

  color_file = Fexpand_file_name (build_string (c"rgb.txt".as_ptr ()),   // C 6813
				  Vdata_directory);                     // C 6814
  color_map = Fx_load_color_file (color_file);                          // C 6815

  if NILP (color_map)                                                   // C 6817
    { fatal (c"Could not read %s.\n".as_ptr (), SDATA (color_file)); }   // C 6818

  (*dpyinfo).color_map = color_map;                                     // C 6820

  #[cfg (not (feature = "android_stubify"))]
  {
    (*dpyinfo).resx = android_pixel_density_x;                          // C 6823
    (*dpyinfo).resy = android_pixel_density_y;                          // C 6824
    (*dpyinfo).font_resolution = android_scaled_pixel_density;          // C 6825
    Vtoolkit_theme = if android_ui_mode == UI_MODE_NIGHT_YES            // C 6826
      { Qdark () } else { Qlight () };                                  // C 6827
  }

  /* https://lists.gnu.org/r/emacs-devel/2015-11/msg00194.html  */       // C 6830
  (*dpyinfo).smallest_font_height = 1;                                  // C 6831
  (*dpyinfo).smallest_char_width = 1;                                   // C 6832

  (*terminal).name = xstrdup (c"android".as_ptr ());                     // C 6834

  {
    let system_name: Lisp_Object = Fsystem_name ();                     // C 6837
    static title: [u8; 10] = *b"GNU Emacs\0";                        // C 6838
    if STRINGP (system_name)                                            // C 6839
      {
	static at: [u8; 5] = *b" at \0";                              // C 6841
	let mut nbytes: ptrdiff_t = (std::mem::size_of_val (&title)
				     + std::mem::size_of_val (&at)) as ptrdiff_t;   // C 6842
	/* C 6843: `if (ckd_add (&nbytes, nbytes, SBYTES (system_name))) memory_full_up ();` -
	   the P-15 `ckd_add`/`overflowing_add` pair (docs/ports/token-aliases.tsv 13).  */
	let (nbytes_, add_overflow) = nbytes.overflowing_add (SBYTES (system_name));
	nbytes = nbytes_;
	if add_overflow
	  { memory_full_up (); }                                       // C 6844
	(*dpyinfo).x_id_name = xmalloc (nbytes as usize) as *mut c_char;   // C 6845
	sprintf ((*dpyinfo).x_id_name, c"%s%s%s".as_ptr (),               // C 6846
		 &raw const title as *const c_char, &raw const at as *const c_char,   // C 6846
		 SDATA (system_name));                                  // C 6847
      }
    else
      {
	(*dpyinfo).x_id_name = xmalloc (std::mem::size_of_val (&title)) as *mut c_char;   // C 6851
	strcpy ((*dpyinfo).x_id_name, &raw const title as *const c_char);   // C 6852
      }
  }

  /* The display "connection" is now set up, and it must never go
     away.  */                                                          // C 6856-6857
  (*terminal).reference_count = 30000;                                  // C 6858

  /* Set the baud rate to the same value it gets set to on X.  */        // C 6860
  baud_rate = 19200;                                                    // C 6861

  #[cfg (not (feature = "android_stubify"))]
  {
    sem_init (&raw mut edit_sem, false as c_int, 0);                    // C 6864
    register_textconv_interface (&raw mut text_conversion_interface);    // C 6865
  }

  /* Binding certain key events in the terminal's `input-decode-map',
     which being keyboard-local is not accessible from any point in
     android-win.el.  */                                               // C 6868-6870
  Fdefine_key (KVAR! ((*terminal).kboard, Vinput_decode_map),           // C 6871
	       make_vector (1, Qselect ()), make_vector (1, Qreturn ()),   // C 6872
	       Qnil ());                                                // C 6873
}

/* C 6876-7117: the file's last three definitions - `android_set_build_fingerprint`,
   `syms_of_androidterm` and `mark_androidterm`.  The first is C's `goto fail` shape (the port's
   labelled block, `break 'fail` at each jump - PORTING.md 2.2), and `syms_of_androidterm` is the
   tree's defvar device (`DEFSYM!`/`DEFVAR_LISP!`/`DEFVAR_INT!`/`DEFVAR_BOOL!`, the
   `include/lisp.rs` macros `src/androidfns.rs` 4579-4600 uses), with C's docstrings transcribed as
   one string each (`\n` for C's line ends, the `src/alloc.rs` 2349 spelling).  */

/* Set Vandroid_build_fingerprint to a reasonable value, and also
   Vandroid_build_manufacturer.  */                                    // C 6878-6879

/* C 6881-6989.  `static` and the *address is taken* (C 7083), so `unsafe extern "C" fn`.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" fn android_set_build_fingerprint ()
{
  #[cfg (feature = "android_stubify")]
  {
    Vandroid_build_fingerprint = Qnil ();                               // C 6885
  }
  #[cfg (not (feature = "android_stubify"))]
  {
    let mut class: jclass;                                              // C 6887
    let mut field: jfieldID;                                            // C 6888
    let mut string: jobject;                                            // C 6889
    let mut data: *const c_char;                                        // C 6890

    'fail: {                                                            // C 6982 (C's `fail:` label)
      /* Set class to NULL so freeing an uninitialized local ref can be
	 avoided.  */                                                   // C 6892-6893
      class = std::ptr::null_mut ();                                    // C 6894

      /* Likewise for string.  */                                      // C 6896
      string = std::ptr::null_mut ();                                   // C 6897

      if ! android_init_gui                                             // C 6899
	{ break 'fail; }                                                // C 6900 (C's `goto fail`)
      else
	{
	  /* Obtain Build.FINGERPRINT.  Clear exceptions after each query;
	     JNI can't find Build.FINGERPRINT on some systems.  */      // C 6903-6904

	  class = ((*(*android_java_env)).FindClass.unwrap ()) (android_java_env,   // C 6906
							       c"android/os/Build".as_ptr ());   // C 6907
	  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 6908

	  if class.is_null ()                                           // C 6910
	    { break 'fail; }                                            // C 6911

	  field = ((*(*android_java_env)).GetStaticFieldID.unwrap ()) (android_java_env,   // C 6913
								       class,   // C 6914
								       c"FINGERPRINT".as_ptr (),   // C 6915
								       c"Ljava/lang/String;".as_ptr ());   // C 6916
	  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 6917

	  if field.is_null ()                                           // C 6919
	    { break 'fail; }                                            // C 6920

	  string
	    = ((*(*android_java_env)).GetStaticObjectField.unwrap ()) (android_java_env,   // C 6922-6923
								       class, field);   // C 6924
	  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 6925

	  if string.is_null ()                                          // C 6927
	    { break 'fail; }                                            // C 6928

	  data = ((*(*android_java_env)).GetStringUTFChars.unwrap ()) (android_java_env,   // C 6930
								       string, std::ptr::null_mut ());   // C 6931
	  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 6932

	  if data.is_null ()                                            // C 6934
	    { break 'fail; }                                            // C 6935

	  Vandroid_build_fingerprint = build_string_from_utf8 (data);   // C 6937
	  ((*(*android_java_env)).ReleaseStringUTFChars.unwrap ()) (android_java_env,   // C 6938
								    string, data);   // C 6939

	  /* Now retrieve Build.MANUFACTURER.  */                       // C 6941

	  ANDROID_DELETE_LOCAL_REF! (string);                           // C 6943
	  string = std::ptr::null_mut ();                               // C 6944

	  field = ((*(*android_java_env)).GetStaticFieldID.unwrap ()) (android_java_env,   // C 6946
								       class,   // C 6947
								       c"MANUFACTURER".as_ptr (),   // C 6948
								       c"Ljava/lang/String;".as_ptr ());   // C 6949
	  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 6950

	  if field.is_null ()                                           // C 6952
	    { break 'fail; }                                            // C 6953

	  string
	    = ((*(*android_java_env)).GetStaticObjectField.unwrap ()) (android_java_env,   // C 6955-6956
								       class, field);   // C 6957
	  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 6958

	  if string.is_null ()                                          // C 6960
	    { break 'fail; }                                            // C 6961

	  data = ((*(*android_java_env)).GetStringUTFChars.unwrap ()) (android_java_env,   // C 6963
								       string, std::ptr::null_mut ());   // C 6964
	  ((*(*android_java_env)).ExceptionClear.unwrap ()) (android_java_env);   // C 6965

	  if data.is_null ()                                            // C 6967
	    { break 'fail; }                                            // C 6968

	  Vandroid_build_manufacturer = build_string_from_utf8 (data);   // C 6970
	  ((*(*android_java_env)).ReleaseStringUTFChars.unwrap ()) (android_java_env,   // C 6971
								    string, data);   // C 6972
	}

      if ! string.is_null ()                                            // C 6975
	{ ANDROID_DELETE_LOCAL_REF! (string); }                         // C 6976

      ANDROID_DELETE_LOCAL_REF! (class);                                // C 6978

      return;                                                           // C 6980
    }

    /* C 6982's `fail:` arm runs the two cleanup statements and clears both variables.  */
    if ! class.is_null ()                                               // C 6983
      { ANDROID_DELETE_LOCAL_REF! (class); }                            // C 6984

    Vandroid_build_fingerprint = Qnil ();                               // C 6986
    Vandroid_build_manufacturer = Qnil ();                              // C 6987
  }
}

/* C's ten `globals.f_<name>` fields this unit's `syms_of_androidterm` registers (its
   `DEFVAR_INT`/`DEFVAR_LISP` calls at C 7008, 7036, 7042, 7046, 7057, 7061, 7065, 7069, 7073 and
   7078), defined here as the port's storage - C's `struct emacs_globals` is zero-initialised and
   the `DEFVAR_*` macros only take each field's address, so the initialisers are `lisp_h_Qnil ()`
   / `0` (docs/status.md 3.29, the `src/cmds.rs` 489-492 row).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_quit_keycode: EMACS_INT = 0;   // C 7008
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut Vandroid_build_fingerprint: Lisp_Object = lisp_h_Qnil ();   // C 7036
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut Vandroid_build_manufacturer: Lisp_Object = lisp_h_Qnil ();   // C 7042
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut android_display_planes: EMACS_INT = 0;   // C 7046
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut Vx_ctrl_keysym: Lisp_Object = lisp_h_Qnil ();   // C 7057
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut Vx_alt_keysym: Lisp_Object = lisp_h_Qnil ();   // C 7061
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut Vx_hyper_keysym: Lisp_Object = lisp_h_Qnil ();   // C 7065
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut Vx_meta_keysym: Lisp_Object = lisp_h_Qnil ();   // C 7069
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut Vx_super_keysym: Lisp_Object = lisp_h_Qnil ();   // C 7073
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub static mut Vx_toolkit_scroll_bars: Lisp_Object = lisp_h_Qnil ();   // C 7078

/* C 6991-7110.  Non-`static` (androidterm.h declares it) and no address is taken:
   `#[unsafe(no_mangle)]` with the C ABI.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_androidterm ()
{
/* C 6997's `DEFVAR_LISP` expands to the global's definition plus its registration; the port's
   `DEFVAR_LISP!` only registers, and `globals.rs` only declares, so the definition is here with
   its C symbol (status 4.721).  */
#[unsafe(no_mangle)]
pub static mut Vandroid_wait_for_event_timeout: Lisp_Object = lisp_h_Qnil ();

  Fprovide (Qandroid (), Qnil ());                                      // C 6994

  DEFVAR_LISP! (c"android-wait-for-event-timeout",                      // C 6996-6997
		Vandroid_wait_for_event_timeout,                        // C 6997
    doc: "How long to wait for Android events.\n\nEmacs will wait up to this many seconds to receive events after\nmaking changes which affect the state of the graphical interface.\nUnder some situations this can take an indefinite amount of time,\nso it is important to limit the wait.\n\nIf set to a non-float value, there will be no wait at all.  ");   // C 6998-7005
  Vandroid_wait_for_event_timeout = make_float (0.1);                   // C 7006

  DEFVAR_INT! (c"android-quit-keycode", android_quit_keycode,           // C 7008
    "Keycode that signals quit when typed twice in rapid succession.\n\nThis is the key code of a key whose repeated activation should prompt\nEmacs to quit, enabling quitting on systems where a keyboard capable of\ntyping C-g is unavailable, when set to a key that does exist on the\ndevice.  Its value must be a keycode defined by the operating system,\nand defaults to 25 (KEYCODE_VOLUME_DOWN), though one of the following\nvalues might be desired on those devices where this default is also\nunavailable, or if another key must otherwise serve this function\ninstead:\n\n  - 4  (KEYCODE_BACK)\n  - 24 (KEYCODE_VOLUME_UP)  ");                                // C 7009-7021
  android_quit_keycode = 25;                                            // C 7022

  DEFVAR_BOOL! (c"x-use-underline-position-properties",                 // C 7024-7025
		x_use_underline_position_properties,                    // C 7025
     doc: "SKIP: real doc in xterm.c.  ");                              // C 7026
  x_use_underline_position_properties = true;                           // C 7027
  DEFSYM! (Qx_use_underline_position_properties,                        // C 7028
	   "x-use-underline-position-properties");                      // C 7029

  DEFVAR_BOOL! (c"x-underline-at-descent-line",                         // C 7031-7032
		x_underline_at_descent_line,                            // C 7032
     doc: "SKIP: real doc in xterm.c.  ");                              // C 7033
  x_underline_at_descent_line = false;                                  // C 7034

  DEFVAR_LISP! (c"android-build-fingerprint", Vandroid_build_fingerprint,   // C 7036
    doc: "String identifying the device's OS version.\nThis is a string that uniquely identifies the version of Android\nEmacs is running on.  ");   // C 7037-7039
  Vandroid_build_fingerprint = Qnil ();                                 // C 7040

  DEFVAR_LISP! (c"android-build-manufacturer", Vandroid_build_manufacturer,   // C 7042
    doc: "Name of the developer of the running version of Android.  ");   // C 7043
  Vandroid_build_manufacturer = Qnil ();                                // C 7044

  DEFVAR_INT! (c"android-display-planes", android_display_planes,        // C 7046
    "Depth and visual class of the display.\nThis variable controls the visual class and depth of the display, which\ncannot be detected on Android.  The default value of 24, and values from\nthere to 8 represent a TrueColor display providing 24 planes, values\nbetween 8 and 1 StaticGray displays providing that many planes, and 1 or\nlower monochrome displays with a single plane.  Modifications to this\nvariable must be completed before the window system is initialized, in,\nfor instance, `early-init.el', or they will be of no effect.  ");   // C 7047-7054
  android_display_planes = 24;                                          // C 7055

  DEFVAR_LISP! (c"x-ctrl-keysym", Vx_ctrl_keysym,                       // C 7057-7058
    doc: "SKIP: real doc in xterm.c.  ");                              // C 7058
  Vx_ctrl_keysym = Qnil ();                                             // C 7059

  DEFVAR_LISP! (c"x-alt-keysym", Vx_alt_keysym,                         // C 7061-7062
    doc: "SKIP: real doc in xterm.c.  ");                              // C 7062
  Vx_alt_keysym = Qnil ();                                              // C 7063

  DEFVAR_LISP! (c"x-hyper-keysym", Vx_hyper_keysym,                     // C 7065-7066
    doc: "SKIP: real doc in xterm.c.  ");                              // C 7066
  Vx_hyper_keysym = Qnil ();                                            // C 7067

  DEFVAR_LISP! (c"x-meta-keysym", Vx_meta_keysym,                       // C 7069-7070
    doc: "SKIP: real doc in xterm.c.  ");                              // C 7070
  Vx_meta_keysym = Qnil ();                                             // C 7071

  DEFVAR_LISP! (c"x-super-keysym", Vx_super_keysym,                     // C 7073-7074
    doc: "SKIP: real doc in xterm.c.  ");                              // C 7074
  Vx_super_keysym = Qnil ();                                            // C 7075

  /* Only defined so loadup.el loads scroll-bar.el.  */                  // C 7077
  DEFVAR_LISP! (c"x-toolkit-scroll-bars", Vx_toolkit_scroll_bars,       // C 7078-7079
    doc: "SKIP: real doc in xterm.c.  ");                              // C 7079
  Vx_toolkit_scroll_bars = Qnil ();                                     // C 7080

  /* Avoid dumping Vandroid_build_fingerprint.  */                       // C 7082
  pdumper_do_now_and_after_load (Some (android_set_build_fingerprint));   // C 7083

  DEFSYM! (Qx_underline_at_descent_line, "x-underline-at-descent-line");   // C 7085

  /* Symbols defined for DND events.  */                                // C 7087
  DEFSYM! (Quri, "uri");                                                // C 7088
  DEFSYM! (Qtext, "text");                                              // C 7089

  /* Symbols defined for modifier value reassignment.  */                // C 7091
  DEFSYM! (Qmodifier_value, "modifier-value");                          // C 7092
  DEFSYM! (Qctrl, "ctrl");                                              // C 7093
  Fput (Qctrl (), Qmodifier_value (), make_fixnum (ctrl_modifier as isize));     // C 7094
  DEFSYM! (Qalt, "alt");                                                // C 7095
  Fput (Qalt (), Qmodifier_value (), make_fixnum (alt_modifier as isize));       // C 7096
  DEFSYM! (Qmeta, "meta");                                              // C 7097
  Fput (Qmeta (), Qmodifier_value (), make_fixnum (meta_modifier as isize));     // C 7098
  DEFSYM! (Qsuper, "super");                                            // C 7099
  Fput (Qsuper (), Qmodifier_value (), make_fixnum (super_modifier as isize));   // C 7100

  /* Key symbols.  */                                                   // C 7102
  DEFSYM! (Qselect, "select");                                          // C 7103
  DEFSYM! (Qreturn, "return");                                          // C 7104

  /* Display configuration updates.  */                                 // C 7106
  DEFSYM! (Qfont_render, "font-render");                                // C 7107
  DEFSYM! (Qdynamic_setting, "dynamic-setting");                        // C 7108
  Fprovide (Qdynamic_setting (), Qnil ());                              // C 7109
}

/* C: lisp.h 4436: `extern void mark_object (Lisp_Object);` - C 7116's call; the mirror carries
   only the `mark_objects` twin (lisp.rs 3969), so the unit names the one-argument form at its use
   site (the `src/haikuterm.rs` 6519-6524 device).  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
unsafe extern "C" {
  pub fn mark_object (obj: Lisp_Object);
}

/* C 7112-7117: `mark_androidterm`, the file's last definition.  Non-`static` and no address is
   taken: `#[unsafe(no_mangle)]` with the C ABI.  */
#[cfg (all (feature = "have_android", feature = "not_android_stubify"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mark_androidterm ()
{
  if ! x_display_list.is_null ()                                        // C 7115
    { mark_object ((*x_display_list).color_map); }                      // C 7116
}
