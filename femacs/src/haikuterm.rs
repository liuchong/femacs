// femacs: port of src/haikuterm.c @ fd7afced403.
//
// One-to-one port (PORTING.md): the same statements in the same order, the same names (function,
// macro, struct, struct field, file-scope object, local), the same control flow and the same error
// model.  The non-`static` C definitions carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the
// thirteen C `static` items whose address this file takes later (the redisplay interface at
// C 3161 and the twelve terminal hooks at C 4419-4456) are `unsafe extern "C" fn` items (a
// function-pointer member cannot be a Rust-ABI item - the `androidfont.rs`/`androidmenu.rs`
// shape); every other `static` helper stays a plain `unsafe fn`.
//
// This file is one of P-86's class: the frozen configuration compiles no `haikuterm.o`.  Its
// object variable is `HAIKU_OBJ`, which `configure.ac` 3097 clears and 3114 fills with
// `haikuterm.o` under `test "${HAVE_BE_APP}" = "yes"` - the same test that `AC_DEFINE`s
// `HAVE_HAIKU` (3106-3110) - and `build/emacs-ref/src/Makefile` 378 reads `HAIKU_OBJ = ` (empty),
// so `docs/baseline/frozen-build-sources.txt` lists no `haikuterm.o` for this target.  The
// `extract-abi.py` checklist for the file is therefore empty (P-86 ②), and `docs/ports/haikuterm.md`
// carries the substitute: the **C line-range table**, the `## Evidence` readings and the
// `## Deviations`.  This batch is P-86 ④'s first slice of the file: **C 1-1044**, which is the
// whole-item boundary nearest C 1000 (C 1044 is the `}` of `haiku_draw_text_decoration`; the
// previous item's `}` is C 885, and the ledger records the choice).
//
//   * the **file gate** is `feature = "have_haiku"`, on every item *and on C 21-38's include
//     graph* - which is why the frozen configuration compiles nothing here.  `HAVE_HAIKU` is the
//     macro `configure.ac` 3107 defines for the Haiku app kit, i.e. exactly the condition that
//     puts `haikuterm.o` in `HAIKU_OBJ`;
//   * C 40-42's `#ifdef USE_BE_CAIRO` / `#include <cairo.h>` is `be_cairo` (the name
//     `femacs/xtask/Cargo.toml` registers for `USE_BE_CAIRO`).  `<cairo.h>` is a system header of
//     the Haiku toolchain, not part of the reference tree, so it has no mirror (P-88) and no name
//     it declares is used by this slice: the gate is carried as the record below and the ledger's
//     table row (the `lwlib/include/lwlib-utils.rs` 61-62 reading of the same dead region; the
//     line numbers moved when 3.508 put that mirror's prototypes in its `_decls` module, P-39);
//   * the file's other conditions are C 1330's `#ifdef GCC_LINT`, C 4148's `#if 0` and C 4772's /
//     C 4895's `#ifdef USE_BE_CAIRO`, all of them **outside this slice** (C 1045 onward).  Every
//     one of the five is decidable: `tools/check-cond-eval.py src/haikuterm.c` reads the file as
//     `5 conditions, 0 unknown, 0 disagreements [VERIFIED]`, and all five take their false arm on
//     this target.
//
// **Acquisition note - this batch is `text-only` (P-86 ③⑵).**  The machine has **no Haiku SDK and
// no Haiku toolchain**.  Measured: `which -a x86_64-unknown-haiku-gcc i686-unknown-haiku-gcc
// haiku-gcc x86_64-pc-haiku-gcc` is empty; `rustup target list | grep -i haiku` is empty (rustc's
// own `--print target-list` names `i686-unknown-haiku`/`x86_64-unknown-haiku`, but no `std` for
// either is installed, and the C++ half of the backend needs Haiku's `libbe`/`libroot` headers,
// which `find / -maxdepth 6 -name 'be_interface.h' -o -name 'Application.h'` does not find).  So
// there is **no arm flip on a Haiku target** and nothing here is compiled or linked against a
// Haiku system.  What this batch carries instead: the frozen `build-tu` reading, P-86 ②'s
// machine-checked coverage table, the `#if` verdict above, a per-item C citation, and an
// arm-reach pre-flight that compiles this unit's own text against the real mirrors plus the
// unit's own use-site declarations (ledger `## Evidence` ②) - which is a `/tmp` harness reading
// and **not** a Haiku-target compile.  Nothing here runs.
//
// `docs/ports/haikuterm.md` carries the C line ranges, the acceptance readings, every deviation
// (the use-site declarations this slice needs, the `struct face` copy, the `--check` baseline rows
// and the line-count tolerance) and the items the integrator owes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unused_parens, unused_mut,
         unreachable_code, unused_labels, while_true, unused_comparisons,
         overflowing_literals, unused_must_use, clippy::all)]

include!("include/config.rs");                        // C 19: `#include <config.h>` (ungated)

/* C 21-35: the file's own include list.  Every header sits inside the file gate, as C's own build
   does - no translation unit the frozen configuration builds reaches any one of them.  */
#[cfg (feature = "have_haiku")]
include!("include/dispextern.rs");                     // C 21: `#include "dispextern.h"`
#[cfg (feature = "have_haiku")]
include!("include/frame.rs");                         // C 22: `#include "frame.h"`
#[cfg (feature = "have_haiku")]
include!("include/lisp.rs");                          // C 23: `#include "lisp.h"`
/* dispextern.h 171: `# include "fontset.h"` - C reaches `fontset_from_font` (C 301/327) through
   C 21's `dispextern.h`, and `include/dispextern.rs` does not carry that include yet, so the
   unit names it here (P-72 rule 4); it stands after `lisp.h` because the mirror invokes lisp.h
   macros in its own bodies (P-57), a mirror gap the ledger records.  */
#[cfg (feature = "have_haiku")]
include!("include/fontset.rs");                       // dispextern.h 171: `#include "fontset.h"`
#[cfg (feature = "have_haiku")]
include!("include/haikugui.rs");                      // C 24: `#include "haikugui.h"`
#[cfg (feature = "have_haiku")]
include!("include/keyboard.rs");                      // C 25: `#include "keyboard.h"`
#[cfg (feature = "have_haiku")]
include!("include/haikuterm.rs");                     // C 26: `#include "haikuterm.h"`
#[cfg (feature = "have_haiku")]
include!("include/blockinput.rs");                    // C 27: `#include "blockinput.h"`
#[cfg (feature = "have_haiku")]
include!("include/termchar.rs");                      // C 28: `#include "termchar.h"`
#[cfg (feature = "have_haiku")]
include!("include/termhooks.rs");                     // C 29: `#include "termhooks.h"`
#[cfg (feature = "have_haiku")]
include!("include/menu.rs");                          // C 30: `#include "menu.h"`
#[cfg (feature = "have_haiku")]
include!("include/buffer.rs");                        // C 31: `#include "buffer.h"`
#[cfg (feature = "have_haiku")]
include!("include/haiku_support.rs");                 // C 32: `#include "haiku_support.h"`
#[cfg (feature = "have_haiku")]
include!("include/thread.rs");                        // C 33: `#include "thread.h"`
#[cfg (feature = "have_haiku")]
include!("include/window.rs");                        // C 34: `#include "window.h"`
#[cfg (feature = "have_haiku")]
include!("include/haikuselect.rs");                   // C 35: `#include "haikuselect.h"`
#[cfg (feature = "have_haiku")]
include!("../lib/include/math.rs");                   // C 37: `#include <math.h>`
#[cfg (feature = "have_haiku")]
include!("../lib/include/stdlib.rs");                 // C 38: `#include <stdlib.h>`

/* C 40-42: `#ifdef USE_BE_CAIRO` / `#include <cairo.h>` / `#endif`.  Dead on this target (the
   frozen table leaves `USE_BE_CAIRO` undefined) and dead text even on a Haiku build unless cairo
   is configured; `<cairo.h>` is a system header with no mirror, and no name it declares is used
   by C 1-1044 (the two `USE_BE_CAIRO` regions that use it are C 4772-4793 and C 4895-4897, both
   outside this slice).  The gate is carried here as the record and in the ledger's table row.  */
// C 41: `#include <cairo.h>`

/* Minimum and maximum values used for Haiku scroll bars.  */
// C 45: `#define BE_SB_MAX 12000000`
#[cfg (feature = "have_haiku")]
pub const BE_SB_MAX: c_int = 12000000;

/* The single Haiku display (if any).  */
// C 48: `struct haiku_display_info *x_display_list;` - the definition of the object
// `include/haikuterm.rs`'s `haikuterm_objs` declares (PORTING.md 0.1-10/P-21).
#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub static mut x_display_list: *mut haiku_display_info = std::ptr::null_mut ();

/* This is used to determine when to evict the font lookup cache,
   which we do every 50 updates.  */
// C 52: `static int up_to_date_count;` (C's tentative definition, i.e. zero)
#[cfg (feature = "have_haiku")]
static mut up_to_date_count: c_int = 0;

/* List of defined fringe bitmaps.  */
// C 55: `static void **fringe_bmps;` (tentative)
#[cfg (feature = "have_haiku")]
static mut fringe_bmps: *mut *mut c_void = std::ptr::null_mut ();

/* The amount of fringe bitmaps in that list.  */
// C 58: `static int max_fringe_bmp;` (tentative)
#[cfg (feature = "have_haiku")]
static mut max_fringe_bmp: c_int = 0;

/* Alist of resources to their values.  */
// C 61: `static Lisp_Object rdb;` (tentative, i.e. the null `Lisp_Object`)
#[cfg (feature = "have_haiku")]
static mut rdb: Lisp_Object = std::ptr::null_mut ();

/* Non-zero means that a HELP_EVENT has been generated since Emacs
   start.  */
// C 65: `static bool any_help_event_p;` (tentative)
#[cfg (feature = "have_haiku")]
static mut any_help_event_p: bool = false;

/* The declarations the seventeen headers above reach, which the frozen mirrors do not carry yet
   (P-72 rule 4: a mirror must reach what it needs through its own `include!`s).  Until each lands
   in its mirror the unit supplies it at the use site - the `androidfont.rs`/`androidmenu.rs`
   shape; every entry names the C header it comes from, and `docs/ports/haikuterm.md` records each
   gap for the integrator.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" {
  /// C: lisp.h 4274: `extern Lisp_Object assoc_no_quit (Lisp_Object, Lisp_Object);` (fns.c).
  fn assoc_no_quit (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
  /// C: frame.h 1511-1512: `extern void adjust_frame_size (struct frame *, int, int, int, bool,
  /// Lisp_Object);` (frame.c).  `window.rs` 7466-7469 and `dispnew.rs` 300-303 carry the same
  /// use-site copy of this prototype.
  fn adjust_frame_size (f: *mut frame, new_width: c_int, new_height: c_int,
                        inhibit: c_int, pretend: bool, parameter: Lisp_Object);
  /// C: dispextern.h 3581: `int window_box_right (struct window *, enum glyph_row_area);`
  fn window_box_right (w: *mut window, area: glyph_row_area) -> c_int;
  /// C: dispextern.h 3631: `extern void get_font_ascent_descent (struct font *, int *, int *);`
  fn get_font_ascent_descent (font: *mut font, ascent: *mut c_int, descent: *mut c_int);
  /// C: dispextern.h 3638: `extern struct font *font_for_underline_metrics (struct glyph_string
  /// *);` (xfaces.c)
  fn font_for_underline_metrics (s: *mut glyph_string) -> *mut font;
  /// C: dispextern.h 3656: `extern void gui_update_cursor (struct frame *, bool);`
  fn gui_update_cursor (f: *mut frame, on_p: bool);
  /// C: dispextern.h 3661-3662: `extern int get_glyph_string_clip_rects (struct glyph_string *,
  /// NativeRectangle *, int);` - `NativeRectangle` is `haiku_rect` for this configuration
  /// (haikugui.h 74), and C's one use site casts to it, so the pointer is untyped here.
  fn get_glyph_string_clip_rects (s: *mut glyph_string, nr: *mut c_void, n: c_int) -> c_int;
  /// C: dispextern.h 3719: `extern bool buffer_flipping_blocked_p (void);`
  fn buffer_flipping_blocked_p () -> bool;
  /// C: dispextern.h 3769-3770: `RGB_PIXEL_COLOR image_background (struct image *, struct frame
  /// *, Emacs_Pix_Context img);` (image.c).  `Emacs_Pix_Context` is `Emacs_Pixmap` for this
  /// configuration (dispextern.h 153-160/196), i.e. `haikugui.h` 41's `haiku`.
  fn image_background (image: *mut image, f: *mut frame, img: Emacs_Pixmap) -> RGB_PIXEL_COLOR;
  /// C: dispextern.h 3771-3772: `int image_background_transparent (struct image *, struct frame
  /// *, Emacs_Pix_Context mask);` (image.c)
  fn image_background_transparent (image: *mut image, f: *mut frame, mask: Emacs_Pixmap) -> c_int;
  /// C: dispextern.h 3845: `void gamma_correct (struct frame *, Emacs_Color *);` (the
  /// `#ifdef HAVE_HAIKU` arm of dispextern.h 3841-3850)
  fn gamma_correct (f: *mut frame, color: *mut Emacs_Color);
  /// C: dispextern.h 4006: `extern bool gui_mouse_grabbed (Display_Info *);` - `Display_Info` is
  /// dispextern.h 159's `typedef struct haiku_display_info Display_Info` for this configuration.
  fn gui_mouse_grabbed (dpyinfo: *mut haiku_display_info) -> bool;
  /// C: math.h: `long int lrint (double);` - the system header, not part of the reference tree
  /// (`xfaces.rs` 509 carries the same use-site declaration).
  fn lrint (x: c_double) -> c_long;
  /// C: math.h: `double fmin (double, double);`
  fn fmin (x: c_double, y: c_double) -> c_double;
  /// C: math.h: `double fmax (double, double);`
  fn fmax (x: c_double, y: c_double) -> c_double;
}

/* C: dispextern.h 3760-3763: the `#define RGB_PIXEL_COLOR unsigned long` arm of
   `#if defined HAVE_X_WINDOWS || defined USE_CAIRO || defined HAVE_NS || defined HAVE_HAIKU ||
   defined HAVE_ANDROID`.  The frozen mirror has no such macro (it is a window-system name), so
   the unit names it at the use site; the two `image_background*` prototypes above are spelled
   with it, as C spells them.  */
#[cfg (feature = "have_haiku")]
pub type RGB_PIXEL_COLOR = c_ulong;

/* C's `IMAGE_BACKGROUND` and `IMAGE_BACKGROUND_TRANSPARENT` (dispextern.h 3860-3872, inside the
   `#ifdef HAVE_WINDOW_SYSTEM` block C 3874 closes) are macros the mirror does not carry yet, so
   they are defined here at the use site (P-26/P-57: the macro belongs in the mirror and moves
   there with it) - `haiku_calculate_relief_colors` (C 630-637) is their only user in this file.  */
#[cfg (feature = "have_haiku")]
macro_rules! IMAGE_BACKGROUND {
  ($img:expr, $f:expr, $ximg:expr) =>
    {
      (if (*$img).bits & IMAGE_BACKGROUND_VALID_MASK != 0
	 { (*$img).background }
       else
	 { image_background ($img, $f, $ximg) })
    };
}
#[cfg (feature = "have_haiku")]
macro_rules! IMAGE_BACKGROUND_TRANSPARENT {
  ($img:expr, $f:expr, $mask:expr) =>
    {
      (if (*$img).bits & IMAGE_BACKGROUND_TRANSPARENT_VALID_MASK != 0
	 { ((*$img).bits >> IMAGE_BACKGROUND_TRANSPARENT_SHIFT & 1) as c_int }
       else
	 { image_background_transparent ($img, $f, $mask) })
    };
}

/* `struct face` (dispextern.h 1787-1937) is this slice's other use-site record (P-72 rule 3/P-80):
   `include/dispextern.rs`'s copy is behind `dispextern_face_records` and so is not available to a
   unit compiled on its own, and `haiku_draw_text_decoration` reads fifteen of its members.  The
   layout is the one `xfaces.rs` 279-330 and `font.rs` 174-225 assert with the frozen clang
   (`sizeof` 280, align 8, `underline_color`@184, `foreground`@168, `background`@176,
   `overline_color`@192, `strike_through_color`@200, `box_color`@208, `underline_pixels_above_
   descent_line`@236, the 21 bit-field members in the 4-byte unit at 240); the bit members are the
   masked accessors of PORTING.md 1.3/P-80, named after C's own members.  */
#[cfg (feature = "have_haiku")]
#[repr(C)]
pub struct face
{
  pub lface: [Lisp_Object; LFACE_VECTOR_SIZE as usize],
  pub id: c_int,
  pub foreground: c_ulong,
  pub background: c_ulong,
  pub underline_color: c_ulong,
  pub overline_color: c_ulong,
  pub strike_through_color: c_ulong,
  pub box_color: c_ulong,
  pub font: *mut font,
  pub fontset: c_int,
  pub box_vertical_line_width: c_int,
  pub box_horizontal_line_width: c_int,
  pub underline_pixels_above_descent_line: c_int,
  pub bits: u32,
  pub hash: uintptr_t,
  pub next: *mut face,
  pub prev: *mut face,
  pub ascii_face: *mut face,
}

#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::size_of::<face> () == 280);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::align_of::<face> () == 8);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, id) == 160);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, foreground) == 168);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, background) == 176);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, underline_color) == 184);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, overline_color) == 192);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, strike_through_color) == 200);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, box_color) == 208);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, font) == 216);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, fontset) == 224);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, bits) == 240);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, hash) == 248);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, next) == 256);
#[cfg (feature = "have_haiku")]
const _: () = core::assert! (std::mem::offset_of! (face, ascii_face) == 272);

/* The `ENUM_BF (face_box_type) box : 2` / `ENUM_BF (face_underline_type) underline : 3` members
   and the seventeen `bool_bf` members C declares at dispextern.h 1893-1935, in C's declaration
   order (the `font.rs` 228-250 copy, whose masks the `--check` macro-body comparison compares).  */
#[cfg (feature = "have_haiku")]
pub const FACE_BOX_SHIFT: u32 = 0;
#[cfg (feature = "have_haiku")]
pub const FACE_BOX_MASK: u32 = 0x3;
#[cfg (feature = "have_haiku")]
pub const FACE_UNDERLINE_SHIFT: u32 = 2;
#[cfg (feature = "have_haiku")]
pub const FACE_UNDERLINE_MASK: u32 = 0x7;
#[cfg (feature = "have_haiku")]
pub const FACE_USE_BOX_COLOR_FOR_SHADOWS_P: u32 = 1 << 5;
#[cfg (feature = "have_haiku")]
pub const FACE_OVERLINE_P: u32 = 1 << 6;
#[cfg (feature = "have_haiku")]
pub const FACE_STRIKE_THROUGH_P: u32 = 1 << 7;
#[cfg (feature = "have_haiku")]
pub const FACE_FOREGROUND_DEFAULTED_P: u32 = 1 << 8;
#[cfg (feature = "have_haiku")]
pub const FACE_BACKGROUND_DEFAULTED_P: u32 = 1 << 9;
#[cfg (feature = "have_haiku")]
pub const FACE_UNDERLINE_DEFAULTED_P: u32 = 1 << 10;
#[cfg (feature = "have_haiku")]
pub const FACE_OVERLINE_COLOR_DEFAULTED_P: u32 = 1 << 11;
#[cfg (feature = "have_haiku")]
pub const FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P: u32 = 1 << 12;
#[cfg (feature = "have_haiku")]
pub const FACE_BOX_COLOR_DEFAULTED_P: u32 = 1 << 13;
#[cfg (feature = "have_haiku")]
pub const FACE_UNDERLINE_AT_DESCENT_LINE_P: u32 = 1 << 14;
#[cfg (feature = "have_haiku")]
pub const FACE_TTY_BOLD_P: u32 = 1 << 15;
#[cfg (feature = "have_haiku")]
pub const FACE_TTY_ITALIC_P: u32 = 1 << 16;
#[cfg (feature = "have_haiku")]
pub const FACE_TTY_REVERSE_P: u32 = 1 << 17;
#[cfg (feature = "have_haiku")]
pub const FACE_TTY_STRIKE_THROUGH_P: u32 = 1 << 18;
#[cfg (feature = "have_haiku")]
pub const FACE_TTY_OVERLINE_P: u32 = 1 << 19;
#[cfg (feature = "have_haiku")]
pub const FACE_COLORS_COPIED_BITWISE_P: u32 = 1 << 20;
#[cfg (feature = "have_haiku")]
pub const FACE_OVERSTRIKE: u32 = 1 << 21;

/* C's member names are the macro's first argument, so they survive the `--check` body comparison
   (the `xfaces.rs` 377-420 / `haiku_output_bits!` shape).  The members this slice reads are the
   eight `bool_bf`/`ENUM_BF` ones C 901-1043 names plus `underline` (C 910/912/998).  */
#[cfg (feature = "have_haiku")]
macro_rules! face_bits {
  (underline, $f:expr) => { unsafe { ((*$f).bits & FACE_UNDERLINE_MASK) >> FACE_UNDERLINE_SHIFT } };
  (underline, $f:expr, $v:expr) => {{
    unsafe { (*$f).bits = ((*$f).bits & ! FACE_UNDERLINE_MASK)
                         | ((($v) as u32) << FACE_UNDERLINE_SHIFT); } }};
  (use_box_color_for_shadows_p, $f:expr) =>
    { face_bit_p ($f, FACE_USE_BOX_COLOR_FOR_SHADOWS_P) };
  (use_box_color_for_shadows_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_USE_BOX_COLOR_FOR_SHADOWS_P, $v) };
  (overline_p, $f:expr) => { face_bit_p ($f, FACE_OVERLINE_P) };
  (overline_p, $f:expr, $v:expr) => { face_bit_set ($f, FACE_OVERLINE_P, $v) };
  (strike_through_p, $f:expr) => { face_bit_p ($f, FACE_STRIKE_THROUGH_P) };
  (strike_through_p, $f:expr, $v:expr) => { face_bit_set ($f, FACE_STRIKE_THROUGH_P, $v) };
  (underline_defaulted_p, $f:expr) => { face_bit_p ($f, FACE_UNDERLINE_DEFAULTED_P) };
  (underline_defaulted_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_UNDERLINE_DEFAULTED_P, $v) };
  (overline_color_defaulted_p, $f:expr) => { face_bit_p ($f, FACE_OVERLINE_COLOR_DEFAULTED_P) };
  (overline_color_defaulted_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_OVERLINE_COLOR_DEFAULTED_P, $v) };
  (strike_through_color_defaulted_p, $f:expr) =>
    { face_bit_p ($f, FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P) };
  (strike_through_color_defaulted_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_STRIKE_THROUGH_COLOR_DEFAULTED_P, $v) };
  (underline_at_descent_line_p, $f:expr) => { face_bit_p ($f, FACE_UNDERLINE_AT_DESCENT_LINE_P) };
  (underline_at_descent_line_p, $f:expr, $v:expr) =>
    { face_bit_set ($f, FACE_UNDERLINE_AT_DESCENT_LINE_P, $v) };
}

#[cfg (feature = "have_haiku")]
#[inline(always)]
pub unsafe fn face_bit_p (f: *mut face, mask: u32) -> bool { ((*f).bits & mask) != 0 }

#[cfg (feature = "have_haiku")]
#[inline(always)]
pub unsafe fn face_bit_set (f: *mut face, mask: u32, v: bool)
{ if v { (*f).bits |= mask; } else { (*f).bits &= ! mask; } }

/* `s->face`'s pointee: `include/dispextern.rs`'s `struct glyph_string` types the member as
   `*mut c_void` (`/// C: struct face *face;`, dispextern.h 1414) because the face record is gated
   there, so every read of a member goes through this cast - the use-site conversion P-72 rule 3
   asks for (`androidfont.rs` 881 passes the same pointer on untyped).  */
#[cfg (feature = "have_haiku")]
macro_rules! GS_FACE {
  ($s:expr) => { ((*$s).face as *mut face) };
}

#[cfg (feature = "have_haiku")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_keysym_name (keysym: c_int) -> *mut c_char          // C 67-73
{
  static mut value: [c_char; 16] = [0; 16];
  sprintf (std::ptr::addr_of_mut! (value) as *mut c_char, c"%d".as_ptr (), keysym);
  return std::ptr::addr_of_mut! (value) as *mut c_char;
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_window_to_frame (window: *mut c_void) -> *mut frame            // C 75-94
{
  let mut tail: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut f: *mut frame;

  FOR_EACH_FRAME! (tail, tem, {
    f = XFRAME (tem);
    if ! FRAME_HAIKU_P (f)
      { continue; }

    eassert! (FRAME_DISPLAY_INFO! (f) == x_display_list);

    if FRAME_HAIKU_WINDOW! (f) == window
      { return f; }
  });

  return 0 as *mut frame;
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_coords_from_parent (f: *mut frame, x: *mut c_int, y: *mut c_int)  // C 96-103
{
  let p: *mut frame = FRAME_PARENT_FRAME (f);

  *x -= (*(FRAME_OUTPUT_DATA! (p) as *mut haiku_output)).frame_x;
  *y -= (*(FRAME_OUTPUT_DATA! (p) as *mut haiku_output)).frame_y;
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_toolkit_position (f: *mut frame, x: c_int, y: c_int,
				  menu_bar_p: *mut bool, tool_bar_p: *mut bool)  // C 105-112
{
  if ! (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).menubar.is_null ()
    {
      *menu_bar_p = (x >= 0 && x < FRAME_PIXEL_WIDTH (f)
		     && y >= 0 && y < FRAME_MENU_BAR_HEIGHT (f));
    }
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_delete_terminal (terminal: *mut terminal)           // C 114-118
{
  error (c"The Haiku terminal cannot be deleted".as_ptr ());
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_get_string_resource (ignored: *mut c_void, name: *const c_char,
						class: *const c_char) -> *const c_char  // C 120-138
{
  let native: *const c_char;

  if name.is_null ()
    { return std::ptr::null (); }

  let lval: Lisp_Object = assoc_no_quit (build_string (name), rdb);

  if ! NILP (lval)
    { return SSDATA (XCDR (lval)) as *const c_char; }

  /* C's `if ((native = be_find_setting (name)))` is the assignment-then-test pair Rust
     spells in two statements (an assignment is `()`, so it cannot be a condition).  */
  native = be_find_setting (name);
  if ! native.is_null ()
    { return native; }

  return std::ptr::null ();
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_update_size_hints (f: *mut frame)                    // C 140-153
{
  /* C's `f->tooltip` is a bit in the flag row, so the mirror's reader is its name.  */
  if FRAME_TOOLTIP_P (f)
    { return; }

  block_input ();
  BWindow_set_size_alignment (FRAME_HAIKU_WINDOW! (f),
			      (if frame_resize_pixelwise
				 { 1 } else { FRAME_COLUMN_WIDTH (f) }),
			      (if frame_resize_pixelwise
				 { 1 } else { FRAME_LINE_HEIGHT (f) }));
  unblock_input ();
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_clip_to_string (s: *mut glyph_string)                          // C 155-196
{
  let mut r: [haiku_rect; 2] = unsafe { core::mem::zeroed () };   // C's uninitialised local (1.5)
  let n: c_int = get_glyph_string_clip_rects (s, &raw mut r as *mut c_void, 2);

  if n != 0
    {
      /* If n[FOO].width is 0, it means to not draw at all, so set the
	 clipping to some impossible value.  */
      if r[0].width <= 0
	{
	  BView_ClipToRect (FRAME_HAIKU_DRAWABLE! ((*s).f),
			    FRAME_PIXEL_WIDTH ((*s).f),
			    FRAME_PIXEL_HEIGHT ((*s).f),
			    10, 10);
	}
      else
	{
	  BView_ClipToRect (FRAME_HAIKU_DRAWABLE! ((*s).f), r[0].x,
			    r[0].y, r[0].width, r[0].height);
	  BView_invalidate_region (FRAME_HAIKU_DRAWABLE! ((*s).f), r[0].x,
				   r[0].y, r[0].width, r[0].height);
	}
    }

  if n > 1
    {
      /* If n[FOO].width is 0, it means to not draw at all, so set the
	 clipping to some impossible value.  */
      if r[1].width <= 0
	{
	  BView_ClipToRect (FRAME_HAIKU_DRAWABLE! ((*s).f),
			    FRAME_PIXEL_WIDTH ((*s).f),
			    FRAME_PIXEL_HEIGHT ((*s).f),
			    10, 10);
	}
      else
	{
	  BView_ClipToRect (FRAME_HAIKU_DRAWABLE! ((*s).f), r[1].x, r[1].y,
			    r[1].width, r[1].height);
	  BView_invalidate_region (FRAME_HAIKU_DRAWABLE! ((*s).f), r[1].x,
				   r[1].y, r[1].width, r[1].height);
	}
    }
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_clip_to_string_exactly (s: *mut glyph_string, dst: *mut glyph_string)  // C 198-205
{
  BView_ClipToRect (FRAME_HAIKU_DRAWABLE! ((*s).f), (*s).x, (*s).y,
		    (*s).width, (*s).height);
  BView_invalidate_region (FRAME_HAIKU_DRAWABLE! ((*s).f), (*s).x,
			   (*s).y, (*s).width, (*s).height);
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_flip_buffers (f: *mut frame)                                   // C 207-219
{
  let view: *mut c_void = (*(FRAME_OUTPUT_DATA! (f) as *mut haiku_output)).view;
  block_input ();

  BView_draw_lock (view, false, 0, 0, 0, 0);
  FRAME_DIRTY_P! (f, 0);
  EmacsView_flip_and_blit (view);
  BView_draw_unlock (view);

  unblock_input ();
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_frame_up_to_date (f: *mut frame)                     // C 221-239
{
  block_input ();
  FRAME_MOUSE_UPDATE! (f);
  if FRAME_DIRTY_P! (f) && ! buffer_flipping_blocked_p ()
    { haiku_flip_buffers (f); }

  up_to_date_count += 1;
  if up_to_date_count == 50
    {
      be_evict_font_cache ();
      up_to_date_count = 0;
    }

  /* Mark the frame as complete.  */
  FRAME_COMPLETE_P! (f, true);
  unblock_input ();
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_buffer_flipping_unblocked_hook (f: *mut frame)      // C 241-246
{
  if FRAME_DIRTY_P! (f)
    { haiku_flip_buffers (f); }
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_clear_frame_area (f: *mut frame, x: c_int, y: c_int,
					     width: c_int, height: c_int)      // C 248-262
{
  let vw: *mut c_void = FRAME_HAIKU_DRAWABLE! (f);
  block_input ();
  BView_draw_lock (vw, true, x, y, width, height);
  BView_StartClip (vw);
  BView_ClipToRect (vw, x, y, width, height);
  BView_SetHighColor (vw, FRAME_BACKGROUND_PIXEL (f) as uint32_t);
  BView_FillRectangle (vw, x, y, width, height);
  BView_EndClip (vw);
  BView_draw_unlock (vw);
  unblock_input ();
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_clear_frame (f: *mut frame)                          // C 264-285
{
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! (f);

  mark_window_cursors_off (XWINDOW (FRAME_ROOT_WINDOW (f)));

  FRAME_COMPLETE_P! (f, false);

  block_input ();
  BView_draw_lock (view, true, 0, 0, FRAME_PIXEL_WIDTH (f),
		   FRAME_PIXEL_HEIGHT (f));
  BView_StartClip (view);
  BView_ClipToRect (view, 0, 0, FRAME_PIXEL_WIDTH (f),
		    FRAME_PIXEL_HEIGHT (f));
  BView_SetHighColor (view, FRAME_BACKGROUND_PIXEL (f) as uint32_t);
  BView_FillRectangle (view, 0, 0, FRAME_PIXEL_WIDTH (f),
		       FRAME_PIXEL_HEIGHT (f));
  BView_EndClip (view);
  BView_draw_unlock (view);
  unblock_input ();
}

/* C's `FRAME_OUTPUT_DATA (f)->field` reads and `FRAME_* (f) = value` writes of C 303-321 (and
   the `FRAME_OUTPUT_DATA (f)->frame_x`/`menubar`/`view` reads of C 101-109/210).  `frame.rs`'s
   `output_data.haiku` member is the untyped P-72 ③ stand-in, so every read casts once - the
   `abi_shim.rs` 532 shape.  The seven write targets are read-only in the mirrors (macros in
   `include/haikuterm.rs` 406-440, functions in `include/frame.rs` 501-715) and a Rust macro is
   not an assignment target, so the unit carries C's two-argument write arm under the same name
   (the `xfaces.rs` 438-447 `FRAME_FACE_CACHE` precedent).  `FRAME_FONTSET`/`FRAME_FONT`/
   `FRAME_BASELINE_OFFSET` shadow the mirror's one-armed macros and keep the read arm
   (`androidmenu.rs`'s `ANDROID_DELETE_LOCAL_REF` shape); the other four are macros *beside* the
   mirror's read functions, so reads keep the function call, exactly as C's one macro serves both.
   `frame.h 300-341`'s two `bool_bf` members this slice reads (`auto_raise`, `auto_lower`) get the
   per-unit member macros `androidterm.rs` 170-195 uses for the same fields.  */
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_FONTSET {
  ($f:expr) => { (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).fontset };
  ($f:expr, $v:expr) => { (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).fontset = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_FONT {
  ($f:expr) => { (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).font };
  ($f:expr, $v:expr) => { (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).font = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_BASELINE_OFFSET {
  ($f:expr) => { (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).baseline_offset };
  ($f:expr, $v:expr) => {
    (*(FRAME_OUTPUT_DATA! ($f) as *mut haiku_output)).baseline_offset = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_COLUMN_WIDTH { ($f:expr, $v:expr) => { (*$f).column_width = $v }; }
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_LINE_HEIGHT { ($f:expr, $v:expr) => { (*$f).line_height = $v }; }
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_TAB_BAR_HEIGHT { ($f:expr, $v:expr) => { (*$f).tab_bar_height = $v }; }
#[cfg (feature = "have_haiku")]
macro_rules! FRAME_CONFIG_SCROLL_BAR_COLS {
  ($f:expr, $v:expr) => { (*$f).config_scroll_bar_cols = $v };
}
#[cfg (feature = "have_haiku")]
macro_rules! set_frame_flag1 {
  ($f:expr, $shift:expr, $v:expr) => {
    (*$f).bits1 = ((*$f).bits1 & ! (1u32 << $shift)) | (($v as u32) << $shift)
  };
}
#[cfg (feature = "have_haiku")]
macro_rules! auto_raise {
  ($f:expr) => { (*$f).bits1 >> FRAME_AUTO_RAISE_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_AUTO_RAISE_SHIFT, $v) };
}
#[cfg (feature = "have_haiku")]
macro_rules! auto_lower {
  ($f:expr) => { (*$f).bits1 >> FRAME_AUTO_LOWER_SHIFT & 1 != 0 };
  ($f:expr, $v:expr) => { set_frame_flag1! ($f, FRAME_AUTO_LOWER_SHIFT, $v) };
}

/* Give frame F the font FONT-OBJECT as its default font.  The return
   value is FONT-OBJECT.  FONTSET is an ID of the fontset for the
   frame.  If it is negative, generate a new fontset from
   FONT-OBJECT.  */

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_new_font (f: *mut frame, font_object: Lisp_Object,
				     fontset: c_int) -> Lisp_Object        // C 287-329
{
  let font: *mut font;
  let mut ascent: c_int = 0;
  let mut descent: c_int = 0;
  let mut unit: c_int = 0;
  /* C's `fontset` parameter is assigned at C 301, so the port rebinds it.  */
  let mut fontset = fontset;

  font = XFONT_OBJECT (font_object);

  if fontset < 0
    { fontset = fontset_from_font (font_object); }

  FRAME_FONTSET! (f, fontset);

  if FRAME_FONT! (f) == font
    { return font_object; }

  FRAME_FONT! (f, font);
  FRAME_BASELINE_OFFSET! (f, (*font).baseline_offset);
  FRAME_COLUMN_WIDTH! (f, (*font).average_width);

  get_font_ascent_descent (font, &raw mut ascent, &raw mut descent);
  FRAME_LINE_HEIGHT! (f, ascent + descent);
  FRAME_TAB_BAR_HEIGHT! (f, FRAME_TAB_BAR_LINES (f) * FRAME_LINE_HEIGHT (f));

  unit = FRAME_COLUMN_WIDTH (f);
  if FRAME_CONFIG_SCROLL_BAR_WIDTH (f) > 0
    {
      FRAME_CONFIG_SCROLL_BAR_COLS! (f, (FRAME_CONFIG_SCROLL_BAR_WIDTH (f) + unit - 1) / unit);
    }
  else
    { FRAME_CONFIG_SCROLL_BAR_COLS! (f, (14 + unit - 1) / unit); }

  if ! FRAME_HAIKU_WINDOW! (f).is_null () && ! FRAME_TOOLTIP_P (f)
    {
      adjust_frame_size (f, FRAME_COLS (f) * FRAME_COLUMN_WIDTH (f),
			 FRAME_LINES (f) * FRAME_LINE_HEIGHT (f),
			 3, false, Qfont ());
    }

  return font_object;
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_valid_modifier_p (sym: Lisp_Object) -> c_int                 // C 331-336
{
  /* C returns the `||` expression, whose `_Bool` result converts to `int`.  */
  return (EQ (sym, Qcommand ()) || EQ (sym, Qshift ())
	  || EQ (sym, Qcontrol ()) || EQ (sym, Qoption ())) as c_int;
}

/* C 338: `#define MODIFIER_OR(obj, def) (haiku_valid_modifier_p (obj) ? obj : def)`.  The macro
   keeps C's double evaluation of `obj`; C 365's `#undef` has no Rust counterpart, and nothing
   after C 365 uses it (recorded in the ledger).  */
#[cfg (feature = "have_haiku")]
macro_rules! MODIFIER_OR {
  ($obj:expr, $def:expr) =>
    { (if haiku_valid_modifier_p ($obj) != 0 { $obj } else { $def }) };
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_add_modifier (modifier: c_int, toput: c_int, qtem: Lisp_Object,
			      modifiers: *mut c_int)                          // C 340-348
{
  if (modifier & HAIKU_MODIFIER_ALT != 0 && EQ (qtem, Qcommand ()))
      || (modifier & HAIKU_MODIFIER_SHIFT != 0 && EQ (qtem, Qshift ()))
      || (modifier & HAIKU_MODIFIER_CTRL != 0 && EQ (qtem, Qcontrol ()))
      || (modifier & HAIKU_MODIFIER_SUPER != 0 && EQ (qtem, Qoption ()))
    { *modifiers |= toput; }
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_modifiers_to_emacs (haiku_key: c_int) -> c_int                // C 350-363
{
  let mut modifiers: c_int = 0;
  haiku_add_modifier (haiku_key, shift_modifier,
		      MODIFIER_OR! (Vhaiku_shift_keysym, Qshift ()), &raw mut modifiers);
  haiku_add_modifier (haiku_key, super_modifier,
		      MODIFIER_OR! (Vhaiku_super_keysym, Qoption ()), &raw mut modifiers);
  haiku_add_modifier (haiku_key, meta_modifier,
		      MODIFIER_OR! (Vhaiku_meta_keysym, Qcommand ()), &raw mut modifiers);
  haiku_add_modifier (haiku_key, ctrl_modifier,
		      MODIFIER_OR! (Vhaiku_control_keysym, Qcontrol ()), &raw mut modifiers);
  return modifiers;
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_rehighlight ()                                               // C 367-397
{
  eassert! (x_display_list != std::ptr::null_mut () && (*x_display_list).next.is_null ());

  block_input ();

  let old_hl: *mut frame = (*x_display_list).highlight_frame;

  if ! (*x_display_list).focused_frame.is_null ()
    {
      (*x_display_list).highlight_frame
	= (if FRAMEP (FRAME_FOCUS_FRAME ((*x_display_list).focused_frame))
	     { XFRAME (FRAME_FOCUS_FRAME ((*x_display_list).focused_frame)) }
	   else
	     { (*x_display_list).focused_frame });
      if ! FRAME_LIVE_P ((*x_display_list).highlight_frame)
	{
	  fset_focus_frame ((*x_display_list).focused_frame, Qnil ());
	  (*x_display_list).highlight_frame = (*x_display_list).focused_frame;
	}
    }
  else
    { (*x_display_list).highlight_frame = 0 as *mut frame; }

  if ! old_hl.is_null ()
    { gui_update_cursor (old_hl, true); }

  if ! (*x_display_list).highlight_frame.is_null ()
    { gui_update_cursor ((*x_display_list).highlight_frame, true); }
  unblock_input ();
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_frame_raise_lower (f: *mut frame, raise_p: bool)   // C 399-416
{
  if raise_p
    {
      block_input ();
      BWindow_activate (FRAME_HAIKU_WINDOW! (f));
      BWindow_sync (FRAME_HAIKU_WINDOW! (f));
      unblock_input ();
    }
  else
    {
      block_input ();
      BWindow_send_behind (FRAME_HAIKU_WINDOW! (f), std::ptr::null_mut ());
      BWindow_sync (FRAME_HAIKU_WINDOW! (f));
      unblock_input ();
    }
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_mouse_or_wdesc_frame (window: *mut c_void,
				      accept_tooltip: bool) -> *mut frame  // C 418-440
{
  let lm_f: *mut frame
    = (if gui_mouse_grabbed (x_display_list)
	 { (*x_display_list).last_mouse_frame }
       else
	 { std::ptr::null_mut () });

  if ! lm_f.is_null () && ! EQ (track_mouse, Qdropping ())
      && ! EQ (track_mouse, Qdrag_source ())
    { return lm_f; }
  else
    {
      let w_f: *mut frame = haiku_window_to_frame (window);

      /* Do not return a tooltip frame.  */
      if w_f.is_null () || (FRAME_TOOLTIP_P (w_f) && ! accept_tooltip)
	{ return if EQ (track_mouse, Qdropping ()) { lm_f } else { std::ptr::null_mut () }; }
      else
	/* When dropping it would be probably nice to raise w_f
	   here.  */
	{ return w_f; }
    }
}

/* Set the thumb size and position of scroll bar BAR.  We are
   currently displaying PORTION out of a whole WHOLE, and our position
   POSITION.  */

#[cfg (feature = "have_haiku")]
unsafe fn haiku_set_scroll_bar_thumb (bar: *mut scroll_bar, portion: c_int,
				      position: c_int, whole: c_int)        // C 442-495
{
  let scroll_bar: *mut c_void = (*bar).scroll_bar;
  let mut top: c_double = 0.0;
  let mut shown: c_double = 0.0;
  let mut size: c_double = 0.0;
  let mut value: c_double = 0.0;
  let mut portion = portion;
  let mut whole = whole;

  if scroll_bar_adjust_thumb_portion_p
    {
      /* We use an estimate of 30 chars per line rather than the real
         `portion' value.  This has the disadvantage that the thumb
         size is not very representative, but it makes our life a lot
         easier.  Otherwise, we have to constantly adjust the thumb
         size, which we can't always do quickly enough: while
         dragging, the size of the thumb might prevent the user from
         dragging the thumb all the way to the end.  */
      portion = WINDOW_TOTAL_LINES (XWINDOW ((*bar).window)) * 30;
      /* When the thumb is at the bottom, position == whole.  So we
         need to increase `whole' to make space for the thumb.  */
      whole += portion;
    }
  else
    { (*bar).page_size = 0; }

  if whole <= 0
    {
      /* C's single comma expression `top = 0, shown = 1;` keeps both assignments in
	 C's order (PORTING.md 2.1).  */
      top = 0.0;
      shown = 1.0;
    }
  else
    {
      top = position as c_double / whole as c_double;
      shown = portion as c_double / whole as c_double;
    }

  /* Slider size.  Must be in the range [1 .. MAX - MIN] where MAX
     is the scroll bar's maximum and MIN is the scroll bar's minimum
     value.  */
  size = clip_to_bounds (1, (shown * BE_SB_MAX as f64) as isize, BE_SB_MAX as isize) as c_double;

  /* Position.  Must be in the range [MIN .. MAX - SLIDER_SIZE].  */
  value = top * BE_SB_MAX as f64;
  value = f64::min (value, BE_SB_MAX as f64 - size);

  if (*bar).dragging == 0 && scroll_bar_adjust_thumb_portion_p
    { (*bar).page_size = size as c_int; }

  BView_scroll_bar_update (scroll_bar, lrint (size) as c_int,
			   BE_SB_MAX, ceil (value) as c_int,
			   (if scroll_bar_adjust_thumb_portion_p
			      { (*bar).dragging }
			    else if (*bar).dragging != 0
			      { -1 }
			    else
			      { 0 }),
			   ! scroll_bar_adjust_thumb_portion_p);
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_set_horizontal_scroll_bar_thumb (bar: *mut scroll_bar, portion: c_int,
						 position: c_int, whole: c_int)  // C 497-515
{
  let scroll_bar: *mut c_void = (*bar).scroll_bar;
  let mut size: c_double = 0.0;
  let mut value: c_double = 0.0;
  let mut shown: c_double = 0.0;
  let mut top: c_double = 0.0;

  shown = portion as c_double / whole as c_double;
  top = position as c_double / whole as c_double;

  size = shown * BE_SB_MAX as f64;
  value = top * BE_SB_MAX as f64;

  if (*bar).dragging == 0
    { (*bar).page_size = size as c_int; }

  BView_scroll_bar_update (scroll_bar, lrint (size) as c_int, BE_SB_MAX,
			   ceil (value) as c_int, if (*bar).dragging != 0 { -1 } else { 0 }, true);
}

/* C 517-540.  C's `for` runs its update expression on the `continue` path too; this loop has
   none, so the port's `tem = XSCROLL_BAR (tem)->next` stands at the foot (the `w32fns.rs` 1524
   reading of the same shape).  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_scroll_bar_from_widget (scroll_bar: *mut c_void,
					window: *mut c_void) -> *mut scroll_bar
{
  let mut tem: Lisp_Object;
  let frame: *mut frame = haiku_window_to_frame (window);

  if frame.is_null ()
    { return std::ptr::null_mut (); }

  if scroll_bar.is_null ()
    { return std::ptr::null_mut (); }

  if ! NILP (FRAME_SCROLL_BARS (frame))
    {
      tem = FRAME_SCROLL_BARS (frame);
      while ! NILP (tem)
	{
	  if (*XSCROLL_BAR! (tem)).scroll_bar == scroll_bar
	    { return XSCROLL_BAR! (tem); }
	  tem = (*XSCROLL_BAR! (tem)).next;
	}
    }

  return std::ptr::null_mut ();
}

/* Unfortunately, NOACTIVATE is not implementable on Haiku.  */
#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_focus_frame (frame: *mut frame, noactivate: bool)  // C 542-548
{
  if (*x_display_list).focused_frame != frame
    { haiku_frame_raise_lower (frame, true); }
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_new_focus_frame (frame: *mut frame)                          // C 550-570
{
  eassert! (x_display_list != std::ptr::null_mut () && (*x_display_list).next.is_null ());

  block_input ();
  if frame != (*x_display_list).focused_frame
    {
      if ! (*x_display_list).focused_frame.is_null ()
	  && auto_lower! ((*x_display_list).focused_frame)
	{ haiku_frame_raise_lower ((*x_display_list).focused_frame, false); }

      (*x_display_list).focused_frame = frame;

      if ! frame.is_null () && auto_raise! (frame)
	  && popup_activated_p == 0
	{ haiku_frame_raise_lower (frame, true); }
    }
  unblock_input ();

  haiku_rehighlight ();
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_implicitly_set_name (f: *mut frame, arg: Lisp_Object,
						oldval: Lisp_Object)          // C 572-576
{
  haiku_set_name (f, arg, false);
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_query_frame_background_color (f: *mut frame,
							 bgcolor: *mut Emacs_Color)  // C 578-582
{
  haiku_query_color (FRAME_BACKGROUND_PIXEL (f) as uint32_t, bgcolor);
}

#[cfg (feature = "have_haiku")]
unsafe extern "C" fn haiku_defined_color (f: *mut frame, name: *const c_char,
					  color: *mut Emacs_Color, alloc: bool,
					  make_index: bool) -> bool       // C 584-596
{
  let rc: c_int;

  /* C's `! haiku_get_color (…)` is a `_Bool` converted to `int`.  */
  rc = (! haiku_get_color (name, color)) as c_int;

  if rc != 0 && (*f).gamma != 0.0 && alloc
    { gamma_correct (f, color); }

  /* C returns the `int` as a `_Bool`.  */
  return rc != 0;
}

/* Adapted from xterm `x_draw_box_rect'.  */
#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_box_rect (s: *mut glyph_string, left_x: c_int, top_y: c_int,
			       right_x: c_int, bottom_y: c_int, hwidth: c_int,
			       vwidth: c_int, left_p: bool, right_p: bool,
			       clip_rect: *mut haiku_rect)                     // C 598-620
{
  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! ((*s).f);
  let face: *mut face = GS_FACE! (s);

  BView_SetHighColor (view, (*face).box_color as uint32_t);
  if ! clip_rect.is_null ()
    {
      BView_ClipToRect (view, (*clip_rect).x, (*clip_rect).y, (*clip_rect).width,
			(*clip_rect).height);
    }
  BView_FillRectangle (view, left_x, top_y, right_x - left_x + 1, hwidth);
  if left_p
    { BView_FillRectangle (view, left_x, top_y, vwidth, bottom_y - top_y + 1); }

  BView_FillRectangle (view, left_x, bottom_y - hwidth + 1,
		       right_x - left_x + 1, hwidth);
  if right_p
    {
      BView_FillRectangle (view, right_x - vwidth + 1,
			   top_y, vwidth, bottom_y - top_y + 1);
    }
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_calculate_relief_colors (s: *mut glyph_string, rgbout_w: *mut uint32_t,
					 rgbout_b: *mut uint32_t)       // C 622-657
{
  let mut h: c_double = 0.0;
  let mut cs: c_double = 0.0;
  let mut l: c_double = 0.0;
  let mut rgbin: uint32_t = 0;
  let mut di: *mut haiku_output;

  if face_bits! (use_box_color_for_shadows_p, ((*s).face as *mut face))
    { rgbin = (*(*s).face as *mut face).box_color as uint32_t; }
  else if glyph_bits! (type, *(*s).first_glyph) == IMAGE_GLYPH
	   && ! (*(*s).img).pixmap.is_null ()
	   && IMAGE_BACKGROUND_TRANSPARENT! ((*s).img, (*s).f, 0 as Emacs_Pixmap) == 0
    { rgbin = IMAGE_BACKGROUND! ((*s).img, (*s).f, 0 as Emacs_Pixmap) as uint32_t; }
  else
    { rgbin = (*(*s).face as *mut face).background as uint32_t; }

  di = FRAME_OUTPUT_DATA! ((*s).f) as *mut haiku_output;

  if (*s).hl == DRAW_CURSOR
    { rgbin = FRAME_CURSOR_COLOR! ((*s).f).pixel as uint32_t; }

  if (*di).relief_background != rgbin as c_long
    {
      (*di).relief_background = (rgbin & 0xffffffff) as c_long;

      rgb_color_hsl (rgbin, &raw mut h, &raw mut cs, &raw mut l);
      hsl_color_rgb (h, cs, fmin (1.0, fmax (0.2, l) * 0.6),
		     &raw mut (*di).black_relief_pixel);
      hsl_color_rgb (h, cs, fmin (1.0, fmax (0.2, l) * 1.2),
		     &raw mut (*di).white_relief_pixel);
    }

  *rgbout_w = (*di).white_relief_pixel;
  *rgbout_b = (*di).black_relief_pixel;
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_relief_rect (s: *mut glyph_string, left_x: c_int, top_y: c_int,
				  right_x: c_int, bottom_y: c_int, hwidth: c_int,
				  vwidth: c_int, raised_p: bool, top_p: bool,
				  bot_p: bool, left_p: bool, right_p: bool,
				  clip_rect: *mut haiku_rect)              // C 659-746
{
  let mut color_white: uint32_t = 0;
  let mut color_black: uint32_t = 0;
  let view: *mut c_void;

  view = FRAME_HAIKU_DRAWABLE! ((*s).f);
  haiku_calculate_relief_colors (s, &raw mut color_white, &raw mut color_black);

  BView_SetHighColor (view, if raised_p { color_white } else { color_black });

  if ! clip_rect.is_null ()
    {
      BView_StartClip (view);
      haiku_clip_to_string (s);
      BView_ClipToRect (view, (*clip_rect).x, (*clip_rect).y,
			(*clip_rect).width, (*clip_rect).height);
    }

  if top_p
    {
      BView_FillRectangle (view, left_x, top_y,
			   right_x - left_x + 1, hwidth);
    }

  if left_p
    {
      BView_FillRectangle (view, left_x, top_y,
			   vwidth, bottom_y - top_y + 1);
    }

  BView_SetHighColor (view, if ! raised_p { color_white } else { color_black });

  if bot_p
    {
      BView_FillRectangle (view, left_x, bottom_y - hwidth + 1,
			   right_x - left_x + 1, hwidth);
    }
  if right_p
    {
      BView_FillRectangle (view, right_x - vwidth + 1, top_y,
			   vwidth, bottom_y - top_y + 1);
    }

  /* Draw the triangle for the bottom-left corner.  */
  if bot_p && left_p
    {
      BView_SetHighColor (view, if raised_p { color_white } else { color_black });
      BView_FillTriangle (view, left_x, bottom_y - hwidth, left_x + vwidth,
			  bottom_y - hwidth, left_x, bottom_y);
    }

  /* Now draw the triangle for the top-right corner.  */
  if top_p && right_p
    {
      BView_SetHighColor (view, if raised_p { color_white } else { color_black });
      BView_FillTriangle (view, right_x - vwidth, top_y,
			  right_x, top_y,
			  right_x - vwidth, top_y + hwidth);
    }

  /* If (h/v)width is > 1, we draw the outer-most line on each side in the
     black relief color.  */

  BView_SetHighColor (view, color_black);

  if hwidth > 1 && top_p
    { BView_StrokeLine (view, left_x, top_y, right_x, top_y); }
  if hwidth > 1 && bot_p
    { BView_StrokeLine (view, left_x, bottom_y, right_x, bottom_y); }
  if vwidth > 1 && left_p
    { BView_StrokeLine (view, left_x, top_y, left_x, bottom_y); }
  if vwidth > 1 && right_p
    { BView_StrokeLine (view, right_x, top_y, right_x, bottom_y); }

  BView_SetHighColor (view, FRAME_BACKGROUND_PIXEL ((*s).f) as uint32_t);

  /* Omit corner pixels.  */
  if hwidth > 1 && vwidth > 1
    {
      if left_p && top_p
	{ BView_FillRectangle (view, left_x, top_y, 1, 1); }
      if left_p && bot_p
	{ BView_FillRectangle (view, left_x, bottom_y, 1, 1); }
      if right_p && top_p
	{ BView_FillRectangle (view, right_x, top_y, 1, 1); }
      if right_p && bot_p
	{ BView_FillRectangle (view, right_x, bottom_y, 1, 1); }
    }

  if ! clip_rect.is_null ()
    { BView_EndClip (view); }
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_get_scale_factor (scale_x: *mut c_int, scale_y: *mut c_int)  // C 748-757
{
  let dpyinfo: *mut haiku_display_info = x_display_list;

  /* C's `double` fields meet the `int` literal `96`, which C promotes; the port writes the
     promoted literal (`dispnew.rs` 7665's `duration > 0.0` shape).  */
  if (*dpyinfo).resx > 96.0
    { *scale_x = floor ((*dpyinfo).resx / 96.0) as c_int; }
  if (*dpyinfo).resy > 96.0
    { *scale_y = floor ((*dpyinfo).resy / 96.0) as c_int; }
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_underwave (s: *mut glyph_string, width: c_int, x: c_int)  // C 759-805
{
  let mut wave_height: c_int = 0;
  let mut wave_length: c_int = 0;
  let mut y: c_int = 0;
  let mut dx: c_int = 0;
  let mut dy: c_int = 0;
  let mut odd: c_int = 0;
  let mut xmax: c_int = 0;
  let mut scale_x: c_int = 0;
  let mut scale_y: c_int = 0;
  let mut ax: f32 = 0.0;
  let mut ay: f32 = 0.0;
  let mut bx: f32 = 0.0;
  let mut by: f32 = 0.0;
  let view: *mut c_void;

  scale_x = 1;
  scale_y = 1;
  haiku_get_scale_factor (&raw mut scale_x, &raw mut scale_y);
  wave_height = 3 * scale_y;
  wave_length = 2 * scale_x;

  dx = wave_length;
  dy = wave_height - 1;
  y = (*s).ybase - wave_height + 3;
  xmax = x + width;
  view = FRAME_HAIKU_DRAWABLE! ((*s).f);

  BView_StartClip (view);
  haiku_clip_to_string (s);
  BView_ClipToRect (view, x, y, width, wave_height);

  ax = (x - (x % dx)) as f32 + 0.5;
  bx = ax + dx as f32;
  odd = ((ax / dx as f32) as c_int) % 2;
  /* C's `ay = by = y + 0.5;` is a chained assignment; the port writes the two assignments
     (the `androidmenu.rs` chained-assignment reading).  */
  ay = y as f32 + 0.5;
  by = y as f32 + 0.5;

  if odd != 0
    { ay += dy as f32; }
  else
    { by += dy as f32; }

  BView_SetPenSize (view, scale_y);

  while ax <= xmax as f32
    {
      BView_StrokeLine (view, ax as c_int, ay as c_int, bx as c_int, by as c_int);
      /* C's `ax = bx, ay = by;` and its two-expression `for` increment, in C's order.  */
      ax = bx;
      ay = by;
      bx += dx as f32;
      by = y as f32 + 0.5 + (odd * dy) as f32;
      odd = (! (odd != 0)) as c_int;
    }

  BView_SetPenSize (view, 1);
  BView_EndClip (view);
}

/* Draw a dashed underline of thickness THICKNESS and width WIDTH onto F
   at a vertical offset of OFFSET from the position of the glyph string
   S, with each segment SEGMENT pixels in length.  */

#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_dash (f: *mut frame, s: *mut glyph_string, width: c_int,
			   segment: c_int, offset: c_int,
			   thickness: c_int)                              // C 807-844
{
  let mut y_center: c_int = 0;
  let mut which: c_int = 0;
  let mut length: c_int = 0;
  let mut x: c_int = 0;
  let mut doffset: c_int = 0;
  let view: *mut c_void;

  /* Configure the thickness of the view's strokes.  */
  view = FRAME_HAIKU_VIEW! (f);
  BView_SetPenSize (view, thickness);

  /* Offset the origin of the line by half the line width. */
  y_center = (*s).ybase + offset + thickness / 2;

  /* Remove redundant portions of OFFSET.  */
  doffset = (*s).x % (segment * 2);

  /* Set which to the phase of the first dash that ought to be drawn and
     length to its length.  */
  which = (doffset < segment) as c_int;
  length = segment - ((*s).x % segment);

  /* Begin drawing this dash.  C's `for` increment has two expressions; the port's `while`
     keeps both at the foot of the body, in C's order.  */
  x = (*s).x;
  while x < (*s).x + width
    {
      if which != 0
	{
	  BView_StrokeLine (view, x, y_center,
			    std::cmp::min (x + length - 1,
					   (*s).x + width - 1),
			    y_center);
	}

      which = (! (which != 0)) as c_int;
      x += length;
      length = segment;
    }
}

/* Draw an underline of STYLE onto F at an offset of POSITION from the
   baseline of the glyph string S, S->WIDTH in length, and THICKNESS in
   height.  */

#[cfg (feature = "have_haiku")]
unsafe fn haiku_fill_underline (f: *mut frame, s: *mut glyph_string,
				style: face_underline_type, position: c_int,
				thickness: c_int)                        // C 846-885
{
  let mut segment: c_int;
  let view: *mut c_void;

  segment = thickness * 3;
  view = FRAME_HAIKU_VIEW! (f);

  /* C's `switch` becomes a `match` and the `FALLTHROUGH` (lib/attribute.h) from the DOTS arm
     into the DASHES one is the entry stage the two arms share (PORTING.md 2.2; the
     `macro-body-notes.tsv` row this needs is owed by the integrator).  */
  match style
    {
      /* FACE_UNDERLINE_DOUBLE_LINE is treated identically to SINGLE, as
	 the second line will be filled by another invocation of this
	 function.  */
      FACE_UNDERLINE_SINGLE | FACE_UNDERLINE_DOUBLE_LINE =>
	{
	  BView_FillRectangle (view, (*s).x, (*s).ybase + position,
			       (*s).width, thickness);
	}

      FACE_UNDERLINE_DOTS | FACE_UNDERLINE_DASHES =>
	{
	  if style == FACE_UNDERLINE_DOTS
	    { segment = thickness; }
	  haiku_draw_dash (f, s, (*s).width, segment, position, thickness);
	}

      FACE_NO_UNDERLINE | FACE_UNDERLINE_WAVE | _ =>
	{ emacs_abort (); }
    }
}

#[cfg (feature = "have_haiku")]
unsafe fn haiku_draw_text_decoration (s: *mut glyph_string, face: *mut face,
				      width: c_int, x: c_int)          // C 887-1044
{
  let mut cursor_color: c_ulong = 0;

  if glyph_string_bits! (for_overlaps, (*s)) != 0
    { return; }

  if (*s).hl == DRAW_CURSOR
    { haiku_merge_cursor_foreground (s as *mut c_void, &raw mut cursor_color,
				     std::ptr::null_mut ()); }

  let view: *mut c_void = FRAME_HAIKU_DRAWABLE! ((*s).f);

  if face_bits! (underline, face) as face_underline_type != FACE_NO_UNDERLINE
    {
      if (*s).hl == DRAW_CURSOR
	{ BView_SetHighColor (view, cursor_color as uint32_t); }
      else if ! face_bits! (underline_defaulted_p, face)
	{ BView_SetHighColor (view, (*face).underline_color as uint32_t); }
      else
	{ BView_SetHighColor (view, (*face).foreground as uint32_t); }

      if face_bits! (underline, face) as face_underline_type == FACE_UNDERLINE_WAVE
	{ haiku_draw_underwave (s, width, x); }
      else if face_bits! (underline, face) as face_underline_type >= FACE_UNDERLINE_SINGLE
	{
	  let mut thickness: c_ulong = 0;
	  let mut position: c_ulong = 0;

	  if ! (*s).prev.is_null ()
	      && (face_bits! (underline, GS_FACE! ((*s).prev)) as face_underline_type
		  != FACE_UNDERLINE_WAVE
		  && face_bits! (underline, GS_FACE! ((*s).prev)) as face_underline_type
		     >= FACE_UNDERLINE_SINGLE)
	      && (face_bits! (underline_at_descent_line_p, GS_FACE! ((*s).prev))
		  == face_bits! (underline_at_descent_line_p, GS_FACE! (s)))
	      && ((*GS_FACE! ((*s).prev)).underline_pixels_above_descent_line
		  == (*GS_FACE! (s)).underline_pixels_above_descent_line)
	    {
	      /* We use the same underline style as the previous one.  */
	      thickness = (*(*s).prev).underline_thickness as c_ulong;
	      position = (*(*s).prev).underline_position as c_ulong;
	    }
	  else
	    {
	      let font: *mut font = font_for_underline_metrics (s);
	      let mut minimum_offset: c_ulong = 0;
	      let mut underline_at_descent_line: bool = false;
	      let mut use_underline_position_properties: bool = false;
	      /* `include/window.rs`'s `WINDOW_BUFFER_LOCAL_VALUE` takes C's own order `(V, W)`
	      since 3.408 fixed the mirror (window.h 633), so the use sites below spell the value
	      first and the window second.  */
	      let mut val: Lisp_Object = WINDOW_BUFFER_LOCAL_VALUE (Qunderline_minimum_offset (), (*s).w);

	      /* C's `max` over an `int` and the `EMACS_INT` fixnum, converted to the
		 `unsigned long` target.  */
	      if FIXNUMP (val)
		{ minimum_offset = std::cmp::max (0, XFIXNUM (val)) as c_ulong; }
	      else
		{ minimum_offset = 1; }

	      val = WINDOW_BUFFER_LOCAL_VALUE (Qx_underline_at_descent_line (), (*s).w);
	      underline_at_descent_line
		= (! (NILP (val) || BASE_EQ (val, Qunbound ()))
		   || face_bits! (underline_at_descent_line_p, GS_FACE! (s)));

	      val = WINDOW_BUFFER_LOCAL_VALUE (Qx_use_underline_position_properties (), (*s).w);
	      use_underline_position_properties
		= ! (NILP (val) || BASE_EQ (val, Qunbound ()));

	      /* Get the underline thickness.  Default is 1 pixel.  */
	      if ! font.is_null () && (*font).underline_thickness > 0
		{ thickness = (*font).underline_thickness as c_ulong; }
	      else
		{ thickness = 1; }
	      if underline_at_descent_line
		{
		  /* C mixes the `int` members with the `unsigned long` locals here, so it
		     converts and wraps; the port writes the same conversions and the same
		     wrap (PORTING.md 1.1).  */
		  position = (((*s).height) as c_ulong).wrapping_sub (thickness)
			     .wrapping_sub (((*s).ybase - (*s).y) as c_ulong)
			     .wrapping_sub ((*GS_FACE! (s)).underline_pixels_above_descent_line
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
		     ROUND(x) = floor (x + 0.5)  */

		  if use_underline_position_properties
		      && ! font.is_null () && (*font).underline_position >= 0
		    { position = (*font).underline_position as c_ulong; }
		  else if ! font.is_null ()
		    { position = (((*font).descent + 1) / 2) as c_ulong; }
		  else
		    { position = minimum_offset; }
		}
	      position = std::cmp::max (position, minimum_offset);
	    }
	  /* Check the sanity of thickness and position.  We should
	     avoid drawing underline out of the current line area.  */
	  if (((*s).y + (*s).height) as c_ulong)
	      <= ((*s).ybase as c_ulong).wrapping_add (position)
	    { position = (((*s).height - 1) - ((*s).ybase - (*s).y)) as c_ulong; }
	  if (((*s).y + (*s).height) as c_ulong)
	      < ((*s).ybase as c_ulong).wrapping_add (position).wrapping_add (thickness)
	    {
	      thickness = (((*s).y + (*s).height) as c_ulong)
			  .wrapping_sub ((*s).ybase as c_ulong + position);
	    }
	  (*s).underline_thickness = thickness as c_int;
	  (*s).underline_position = position as c_int;

	  haiku_fill_underline ((*s).f, s,
				face_bits! (underline, GS_FACE! (s)) as face_underline_type,
				position as c_int, thickness as c_int);

	  /* Place a second underline above the first if this was
	     requested in the face specification.  */

	  if face_bits! (underline, GS_FACE! (s)) as face_underline_type
	     == FACE_UNDERLINE_DOUBLE_LINE
	    {
	      /* Compute the position of the second underline.  */
	      position = position.wrapping_sub (thickness).wrapping_sub (1);
	      haiku_fill_underline ((*s).f, s,
				    face_bits! (underline, GS_FACE! (s)) as face_underline_type,
				    position as c_int, thickness as c_int);
	    }
	}
    }

  if face_bits! (overline_p, face)
    {
      let mut dy: c_ulong = 0;
      let h: c_ulong = 1;
      if (*s).hl == DRAW_CURSOR
	{ BView_SetHighColor (view, cursor_color as uint32_t); }
      else if ! face_bits! (overline_color_defaulted_p, face)
	{ BView_SetHighColor (view, (*face).overline_color as uint32_t); }
      else
	{ BView_SetHighColor (view, (*face).foreground as uint32_t); }

      BView_FillRectangle (view, (*s).x, ((*s).y as c_ulong + dy) as c_int,
			   (*s).width, h as c_int);
    }

  if face_bits! (strike_through_p, face)
    {
      /* Y-coordinate and height of the glyph string's first
	 glyph.  We cannot use s->y and s->height because those
	 could be larger if there are taller display elements
	 (e.g., characters displayed with a larger font) in the
	 same glyph row.  */
      let glyph_y: c_int = (*s).ybase - (*(*s).first_glyph).ascent as c_int;
      let glyph_height: c_int = (*(*s).first_glyph).ascent as c_int
				+ (*(*s).first_glyph).descent as c_int;
      /* Strike-through width and offset from the glyph string's
	 top edge.  */
      let h: c_ulong = 1;
      let dy: c_ulong = (glyph_height as c_ulong).wrapping_sub (h) / 2;

      if (*s).hl == DRAW_CURSOR
	{ BView_SetHighColor (view, cursor_color as uint32_t); }
      else if ! face_bits! (strike_through_color_defaulted_p, face)
	{ BView_SetHighColor (view, (*face).strike_through_color as uint32_t); }
      else
	{ BView_SetHighColor (view, (*face).foreground as uint32_t); }

      BView_FillRectangle (view, (*s).x, (glyph_y as c_ulong + dy) as c_int,
			   (*s).width, h as c_int);
    }
}
